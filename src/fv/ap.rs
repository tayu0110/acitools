use crate::{BuilderTrait, Client};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FvAp {
    FvAp { attributes: Attributes },
}

impl FvAp {
    pub fn builder(name: &str, tenant_name: &str) -> ApBuilder {
        ApBuilder::new(name, tenant_name)
    }

    fn attributes(&self) -> &Attributes {
        let FvAp::FvAp { attributes } = self;
        attributes
    }

    pub fn get(client: &Client) -> Result<GetApRequestBuilder, Box<dyn std::error::Error>> {
        Ok(GetApRequestBuilder::new(
            client.get("node/class/fvAp.json")?,
        ))
    }

    pub fn annotation(&self) -> &str {
        &self.attributes().annotation
    }

    pub fn child_action(&self) -> &str {
        &self.attributes().child_action
    }

    pub fn descr(&self) -> &str {
        &self.attributes().descr
    }

    pub fn name(&self) -> &str {
        &self.attributes().name
    }

    pub fn name_alias(&self) -> &str {
        &self.attributes().name_alias
    }

    pub fn owner_key(&self) -> &str {
        &self.attributes().owner_key
    }

    pub fn owner_tag(&self) -> &str {
        &self.attributes().owner_tag
    }
}

pub struct GetApRequestBuilder<'a> {
    builder: crate::client::GetRequestBuilder<'a>,
}

impl<'a> GetApRequestBuilder<'a> {
    fn new(builder: crate::client::GetRequestBuilder<'a>) -> Self {
        Self { builder }
    }

    pub async fn send(self) -> Result<Box<[FvAp]>, Box<dyn std::error::Error>> {
        let res = self.builder.send().await?;
        Ok(res
            .into_iter()
            .map(|res| serde_json::from_value(res))
            .collect::<Result<Vec<FvAp>, serde_json::Error>>()?
            .into_boxed_slice())
    }
}

impl<'a> BuilderTrait<'a> for GetApRequestBuilder<'a> {
    fn renew(builder: crate::GetRequestBuilder<'a>) -> Self {
        Self::new(builder)
    }
    fn builder(self) -> crate::GetRequestBuilder<'a> {
        self.builder
    }
}

pub struct ApBuilder {
    parent: String,
    data: Attributes,
}

impl ApBuilder {
    fn new(name: &str, tenant_name: &str) -> Self {
        Self {
            parent: tenant_name.to_string(),
            data: Attributes {
                annotation: String::new(),
                child_action: String::new(),
                descr: String::new(),
                dn: format!("uni/tn-{}/ap-{}", tenant_name, name),
                name: name.to_string(),
                name_alias: String::new(),
                owner_key: String::new(),
                owner_tag: String::new(),
                prio: QoSClass::Unspecified,
                status: String::new(),
                userdom: String::new(),
                payload: None,
            },
        }
    }

    pub fn set_annotation(mut self, value: impl ToString) -> Self {
        self.data.annotation = value.to_string();
        self
    }

    pub fn set_descr(mut self, value: impl ToString) -> Self {
        self.data.descr = value.to_string();
        self
    }

    pub fn set_name_alias(mut self, value: impl ToString) -> Self {
        self.data.name_alias = value.to_string();
        self
    }

    pub fn set_qos_class(mut self, class: QoSClass) -> Self {
        self.data.prio = class;
        self
    }

    async fn post(
        &mut self,
        client: &Client,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "totalCount": "1",
            "imdata": [{
                "fvTenant": {
                    "attributes": {
                        "name": self.parent,
                        "status": "modified",
                    },
                    "children": [{
                        "fvAp": {
                            "attributes": self.data
                        }
                    }]
                }
            }]
        });
        Ok(client.post("mo/uni.json", &json).await?)
    }

    pub async fn create(
        &mut self,
        client: &Client,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        self.data.status = "created".to_string();
        Ok(self.post(client).await?)
    }

    pub async fn update(
        &mut self,
        client: &Client,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        self.data.status = "modified".to_string();
        Ok(self.post(client).await?)
    }

    pub async fn delete(
        &mut self,
        client: &Client,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        self.data.status = "deleted".to_string();
        Ok(self.post(client).await?)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    annotation: String,
    child_action: String,
    descr: String,
    dn: String,
    name: String,
    name_alias: String,
    owner_key: String,
    owner_tag: String,
    prio: QoSClass,
    status: String,
    userdom: String,
    #[serde(flatten)]
    payload: Option<HashMap<String, String>>,
    // extMngdBy: String,
    // lcOwn: String,
    // modTs: String,
    // monPolDn: String,
    // uid: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum QoSClass {
    Unspecified,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
}
