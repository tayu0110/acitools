use crate::{BuilderTrait, Client, ResponseFormat};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FvCtx {
    fv_ctx: Inner,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Inner {
    attributes: Attributes,
    #[serde(flatten)]
    children: HashMap<String, String>,
}

impl FvCtx {
    pub fn builder(name: &str, tenant_name: &str) -> CtxBuilder {
        CtxBuilder::new(name, tenant_name)
    }

    pub fn get(client: &mut Client) -> Result<GetCtxRequestBuilder, Box<dyn std::error::Error>> {
        Ok(GetCtxRequestBuilder::new(
            client.get("node/class/fvCtx.json")?,
        ))
    }

    pub fn annotation(&self) -> &str {
        &self.fv_ctx.attributes.annotation
    }

    pub fn child_action(&self) -> &str {
        &self.fv_ctx.attributes.child_action
    }

    pub fn descr(&self) -> &str {
        &self.fv_ctx.attributes.descr
    }

    pub fn name(&self) -> &str {
        &self.fv_ctx.attributes.name
    }

    pub fn name_alias(&self) -> &str {
        &self.fv_ctx.attributes.name_alias
    }

    pub fn owner_key(&self) -> &str {
        &self.fv_ctx.attributes.owner_key
    }

    pub fn owner_tag(&self) -> &str {
        &self.fv_ctx.attributes.owner_tag
    }
}

pub struct GetCtxRequestBuilder<'a> {
    builder: crate::client::GetRequestBuilder<'a>,
}

impl<'a> GetCtxRequestBuilder<'a> {
    fn new(builder: crate::client::GetRequestBuilder<'a>) -> Self {
        Self { builder }
    }

    pub async fn send(self) -> Result<Box<[FvCtx]>, Box<dyn std::error::Error>> {
        let res = self.builder.send().await?;
        let res = serde_json::from_value::<ResponseFormat<FvCtx>>(res)?.extract();
        Ok(res.into_boxed_slice())
    }
}

impl<'a> BuilderTrait<'a> for GetCtxRequestBuilder<'a> {
    fn renew(builder: crate::GetRequestBuilder<'a>) -> Self {
        Self::new(builder)
    }
    fn builder(self) -> crate::GetRequestBuilder<'a> {
        self.builder
    }
}

pub struct CtxBuilder {
    parent: String,
    data: Attributes,
}

impl CtxBuilder {
    pub fn new(name: &str, tenant_name: &str) -> Self {
        Self {
            parent: tenant_name.to_string(),
            data: Attributes {
                annotation: String::new(),
                child_action: String::new(),
                descr: String::new(),
                dn: format!("uni/tn-{}/ctx-{}", tenant_name, name),
                name: name.to_string(),
                name_alias: String::new(),
                owner_key: String::new(),
                owner_tag: String::new(),
                status: String::new(),
                userdom: String::new(),
                payload: None,
            },
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
        client: &mut Client,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "totalCount": "1",
            "imdata": [{
                "fvTenant": {
                    "attributes": {
                        "name": self.parent,
                        "status": "modified",
                    },
                    "children": [{
                        "fvCtx": {
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
        client: &mut Client,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        self.data.status = "created".to_string();
        Ok(self.post(client).await?)
    }

    pub async fn update(
        &mut self,
        client: &mut Client,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        self.data.status = "modified".to_string();
        Ok(self.post(client).await?)
    }

    pub async fn delete(
        &mut self,
        client: &mut Client,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        self.data.status = "deleted".to_string();
        Ok(self.post(client).await?)
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct Attributes {
    annotation: String,
    child_action: String,
    descr: String,
    dn: String,
    name: String,
    name_alias: String,
    owner_key: String,
    owner_tag: String,
    status: String,
    userdom: String,
    #[serde(flatten)]
    payload: Option<HashMap<String, String>>,
    // bd_enforced_enable: String,
    // ext_mngd_by: String,
    // ip_data_plane_learning: String,
    // knw_mcast_act: String,
    // lc_own: String,
    // mod_ts: String,
    // mon_pol_dn: String,
    // pc_enf_dir: String,
    // pc_enf_dir_updated: String,
    // pc_enf_pref: String,
    // pc_tag: String,
    // scope: String,
    // seg: String,
    // uid: String,
    // vrf_id: String,
    // vrf_index: String,
}
