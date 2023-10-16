use crate::{BuilderTrait, Client, FvCtx};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FvTenant {
    FvTenant { attributes: Attributes },
}

impl FvTenant {
    pub fn builder(name: &str) -> TenantBuilder {
        TenantBuilder::new(name)
    }

    fn attributes(&self) -> &Attributes {
        let FvTenant::FvTenant { attributes } = self;
        attributes
    }

    fn attributes_mut(&mut self) -> &mut Attributes {
        let FvTenant::FvTenant { attributes } = self;
        attributes
    }

    pub fn get(client: &Client) -> Result<GetTenantRequestBuilder, Box<dyn std::error::Error>> {
        Ok(GetTenantRequestBuilder::new(
            client.get("node/class/fvTenant.json")?,
        ))
    }

    pub async fn get_vrfs(
        &self,
        client: &Client,
    ) -> Result<Vec<FvCtx>, Box<dyn std::error::Error>> {
        let val = client
            .get(&format!("mo/{}.json", self.attributes().dn))?
            .query_target(crate::QueryTarget::CHILDREN)
            .target_subtree_class(crate::ClassName::FvCtx)
            .send()
            .await?;
        Ok(val
            .into_iter()
            .map(|res| serde_json::from_value(res))
            .collect::<Result<Vec<FvCtx>, serde_json::Error>>()?)
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

pub struct GetTenantRequestBuilder<'a> {
    builder: crate::client::GetRequestBuilder<'a>,
}

impl<'a> GetTenantRequestBuilder<'a> {
    fn new(builder: crate::client::GetRequestBuilder<'a>) -> Self {
        Self { builder }
    }

    pub async fn send(self) -> Result<Box<[FvTenant]>, Box<dyn std::error::Error>> {
        let res = self.builder.send().await?;
        Ok(res
            .into_iter()
            .map(|res| serde_json::from_value(res))
            .collect::<Result<Vec<FvTenant>, serde_json::Error>>()?
            .into_boxed_slice())
    }
}

impl<'a> BuilderTrait<'a> for GetTenantRequestBuilder<'a> {
    fn renew(builder: crate::GetRequestBuilder<'a>) -> Self {
        Self::new(builder)
    }
    fn builder(self) -> crate::GetRequestBuilder<'a> {
        self.builder
    }
}

pub struct TenantBuilder {
    data: Attributes,
}

impl TenantBuilder {
    fn new(name: &str) -> Self {
        Self {
            data: Attributes {
                annotation: String::new(),
                child_action: String::new(),
                descr: String::new(),
                dn: format!("uni/tn-{}", name),
                name: name.to_string(),
                name_alias: String::new(),
                owner_key: String::new(),
                owner_tag: String::new(),
                status: String::new(),
                payload: None,
            },
        }
    }

    pub fn from_tenant(mut tenant: FvTenant) -> Self {
        if let Some(payload) = tenant.attributes_mut().payload.as_mut() {
            payload.remove("extMngdBy");
            payload.remove("lcOwn");
            payload.remove("modTs");
            payload.remove("monPolDn");
            payload.remove("uid");
        }
        Self {
            data: tenant.attributes().clone(),
        }
    }

    pub fn set_annotation(mut self, annotation: impl ToString) -> Self {
        self.data.annotation = annotation.to_string();
        self
    }

    pub fn set_descr(mut self, descr: impl ToString) -> Self {
        self.data.descr = descr.to_string();
        self
    }

    pub fn set_name(mut self, name: impl ToString) -> Self {
        self.data.name = name.to_string();
        self
    }

    pub fn set_name_alias(mut self, name_alias: impl ToString) -> Self {
        self.data.name_alias = name_alias.to_string();
        self
    }

    pub fn set_status(mut self, status: impl ToString) -> Self {
        self.data.status = status.to_string();
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
                    "attributes": self.data
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
        client: &mut Client,
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

#[derive(Debug, Deserialize, Clone, Serialize)]
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
    status: String,
    #[serde(flatten)]
    payload: Option<HashMap<String, String>>,
    // ext_mngd_by: String,
    // lc_own: String,
    // mod_ts: String,
    // mon_pol_dn: String,
    // uid: String,
}
