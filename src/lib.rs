#![recursion_limit = "256"]

mod aaa;
mod bgp;
mod client;
mod eqpt;
mod error;
mod ethpm;
mod fabric;
mod fault;
mod fv;
mod l1;
mod l3ext;
mod lldp;
mod ospf;
mod rtctrl;
mod top;

use std::borrow::Cow;
use std::marker::PhantomData;

pub use aaa::login::AaaLogin;
pub use aaa::role::PrivType;
pub use aaa::user::*;
pub use client::*;
pub use error::*;
pub use fabric::pod::{FabricPod, FabricPodEndpoint};
pub use fault::FaultInst;
pub use fv::ap::{FvAp, QoSClass};
pub use fv::bd::{FvBD, L2UnknownUnicast, L3UnknownMulticast, MultiDestinationFlooding};
pub use fv::ctx::FvCtx;
pub use fv::subnet::FvSubnet;
pub use fv::tenant::*;
pub use l3ext::out::{L3extOut, L3extOutEndpoint};
use serde::de::DeserializeOwned;
pub use top::system::TopSystem;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Response {
    #[serde(rename = "totalCount")]
    total_count: String,
    imdata: Vec<serde_json::Value>,
}

impl Response {
    pub(crate) async fn extract(mut self) -> Result<Vec<serde_json::Value>, Error> {
        if self.imdata.len() != 1 {
            return Ok(self.imdata);
        }

        if let Some(mut error) = self.imdata[0].as_object_mut().unwrap().remove("error") {
            let attributes = error.as_object_mut().unwrap().remove("attributes").unwrap();
            return Err(serde_json::from_value::<Error>(attributes).unwrap());
        }

        Ok(self.imdata)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AciObject<T: AciObjectScheme> {
    attributes: T::Attributes,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    children: Vec<T::ChildItem>,
}

impl<T: AciObjectScheme> AciObject<T> {
    pub fn new(attr: T::Attributes) -> Self {
        Self {
            attributes: attr,
            children: vec![],
        }
    }

    pub fn attributes(&self) -> &T::Attributes {
        &self.attributes
    }

    pub fn attributes_mut(&mut self) -> &mut T::Attributes {
        &mut self.attributes
    }

    pub fn children(&self) -> &[T::ChildItem] {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut [T::ChildItem] {
        &mut self.children
    }

    pub fn push_child(mut self, child: T::ChildItem) -> Self {
        self.children.push(child);
        self
    }

    pub fn extend_child(mut self, child: Vec<T::ChildItem>) -> Self {
        self.children.extend(child);
        self
    }

    pub fn get(endpoint: T::Endpoint) -> GetRequester<T> {
        GetRequester::new(endpoint)
    }

    pub async fn post(
        self,
        endpoint: T::Endpoint,
        client: &Client,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "totalCount": "1",
            "imdata": [{
                T::CLASS_NAME: {
                    "attributes": self.attributes,
                    "children": self.children,
                }
            }]
        });
        Ok(client.post(endpoint.endpoint().as_ref(), &json).await?)
    }
}

pub struct GetRequester<T: AciObjectScheme> {
    endpoint: T::Endpoint,
    queries: Vec<(&'static str, String)>,
    _phantom: PhantomData<fn() -> T>,
}

impl<T: AciObjectScheme> GetRequester<T> {
    fn new(endpoint: T::Endpoint) -> Self {
        Self {
            endpoint,
            queries: vec![],
            _phantom: PhantomData,
        }
    }

    pub fn query_target(mut self, target: QueryTarget) -> Self {
        self.queries.push(("query-target", target.to_string()));
        self
    }

    pub fn target_subtree_class(mut self, class_name: ClassName) -> Self {
        self.queries
            .push(("target-subtree-class", class_name.to_string()));
        self
    }

    pub fn query_target_filter(mut self, filter: Filter) -> Self {
        self.queries
            .push(("query-target-filter", filter.to_string()));
        self
    }

    pub fn rsp_subtree(mut self, rsp_subtree: RspSubTree) -> Self {
        self.queries.push(("rsp-subtree", rsp_subtree.to_string()));
        self
    }

    pub fn rsp_subtree_class(mut self, class_name: ClassName) -> Self {
        self.queries
            .push(("rsp-subtree-class", class_name.to_string()));
        self
    }

    pub fn rsp_subtree_filter(mut self, filter: Filter) -> Self {
        self.queries
            .push(("rsp-subtree-filter", filter.to_string()));
        self
    }

    pub fn rsp_prop_include(mut self, rsp_prop_include: RspPropInclude) -> Self {
        self.queries
            .push(("rsp-prop-include", rsp_prop_include.to_string()));
        self
    }

    async fn convert(
        res: Vec<serde_json::Value>,
    ) -> Result<Box<[AciObject<T>]>, Box<dyn std::error::Error>> {
        Ok(res
            .into_iter()
            .map(|mut value| {
                serde_json::from_value::<AciObject<T>>(
                    value
                        .as_object_mut()
                        .unwrap()
                        .remove(T::CLASS_NAME)
                        .unwrap(),
                )
            })
            .collect::<Result<Vec<_>, serde_json::Error>>()?
            .into_boxed_slice())
    }

    pub async fn send(
        self,
        client: &Client,
    ) -> Result<Box<[AciObject<T>]>, Box<dyn std::error::Error>> {
        let endpoint = client
            .endpoint
            .clone()
            .join(self.endpoint.endpoint().as_ref())?;
        let res = client.get_unchecked(endpoint.clone(), &self.queries).await;
        if let Ok(res) = res {
            return Self::convert(res).await;
        }
        client.refresh().await?;
        let res = client.get_unchecked(endpoint, &self.queries).await?;
        Self::convert(res).await
    }
}

pub trait EndpointScheme {
    fn endpoint(&self) -> Cow<'_, str>;
}

pub trait AciObjectScheme {
    type Attributes: DeserializeOwned + Serialize;
    type ChildItem: DeserializeOwned + Serialize;
    type Endpoint: EndpointScheme;
    const CLASS_NAME: &'static str;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_format_test() {
        let ser = serde_json::to_value(Response {
            total_count: "1".to_owned(),
            imdata: vec![],
        });

        assert!(ser.is_ok());

        let ser = ser.unwrap().to_string();
        assert_eq!(ser, "{\"imdata\":[],\"totalCount\":\"1\"}")
    }
}
