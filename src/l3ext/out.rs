use super::{bridge_domain, domain_profile, ext_epg, logical_node_profile, private_network};
use crate::{bgp, ospf, rtctrl};
use crate::{BuilderTrait, Client};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum L3extOut {
    L3extOut {
        attributes: Attributes,
        #[serde(default)]
        children: Vec<ChildItem>,
    },
}

impl L3extOut {
    pub fn builder(tenant: &str, name: &str) -> L3extOutBuilder {
        L3extOutBuilder::new(tenant, name)
    }

    pub fn attributes(&self) -> &Attributes {
        let L3extOut::L3extOut { attributes, .. } = self;
        attributes
    }

    pub fn attributes_mut(&mut self) -> &mut Attributes {
        let L3extOut::L3extOut { attributes, .. } = self;
        attributes
    }

    pub fn get(client: &Client) -> Result<GetL3extOutRequestBuilder, Box<dyn std::error::Error>> {
        Ok(GetL3extOutRequestBuilder::new(
            client
                .get("class/l3extOut.json")?
                .rsp_subtree(crate::RspSubTree::FULL),
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

pub struct GetL3extOutRequestBuilder<'a> {
    builder: crate::client::GetRequestBuilder<'a>,
}

impl<'a> GetL3extOutRequestBuilder<'a> {
    fn new(builder: crate::client::GetRequestBuilder<'a>) -> Self {
        Self { builder }
    }

    pub async fn send(self) -> Result<Box<[L3extOut]>, Box<dyn std::error::Error>> {
        let res = self.builder.send().await?;
        Ok(res
            .into_iter()
            .map(|res| serde_json::from_value(res))
            .collect::<Result<Vec<L3extOut>, serde_json::Error>>()?
            .into_boxed_slice())
    }
}

impl<'a> BuilderTrait<'a> for GetL3extOutRequestBuilder<'a> {
    fn renew(builder: crate::GetRequestBuilder<'a>) -> Self {
        Self::new(builder)
    }
    fn builder(self) -> crate::GetRequestBuilder<'a> {
        self.builder
    }
}

pub struct L3extOutBuilder {
    data: Attributes,
    children: Vec<ChildItem>,
}

impl L3extOutBuilder {
    fn new(tenant: &str, name: &str) -> Self {
        Self {
            data: Attributes {
                annotation: String::new(),
                child_action: String::new(),
                descr: String::new(),
                dn: format!("uni/tn-{}/out-{}", tenant, name),
                enforce_rtctrl: EnforceRouteControl::default(),
                mpls_enabled: String::new(),
                name: name.to_string(),
                name_alias: String::new(),
                owner_key: String::new(),
                owner_tag: String::new(),
                status: String::new(),
                target_dscp: String::new(),
                userdom: String::new(),
                payload: None,
            },
            children: vec![],
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

    pub fn enforce_route_control(mut self, val: EnforceRouteControl) -> Self {
        self.data.enforce_rtctrl = val;
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

    async fn post(
        &mut self,
        client: &Client,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "totalCount": "1",
            "imdata": [{
                "l3extOut": {
                    "attributes": self.data,
                    "children": self.children
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
    enforce_rtctrl: EnforceRouteControl,
    mpls_enabled: String,
    name: String,
    name_alias: String,
    owner_key: String,
    owner_tag: String,
    status: String,
    target_dscp: String,
    userdom: String,
    #[serde(flatten)]
    payload: Option<HashMap<String, String>>,
    // ext_mngd_by: String,
    // lc_own: String,
    // mod_ts: String,
    // mon_pol_dn: String,
    // uid: String,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EnforceRouteControl {
    #[default]
    Export,
    Import,
    #[serde(rename = "export,import")]
    ExportImport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    L3extRsOutToBDPublicSubnetHolder {},
    L3extRtBDToOut {
        attributes: bridge_domain::Attributes,
    },
    L3extRsEctx {
        attributes: private_network::Attributes,
    },
    OspfExtP {
        attributes: ospf::ext_epg::Attributes,
    },
    L3extRsL3DomAtt {
        attributes: domain_profile::Attributes,
    },
    L3extLNodeP {
        attributes: logical_node_profile::Attributes,
        #[serde(default)]
        children: Vec<logical_node_profile::ChildItem>,
    },
    L3extInstP {
        attributes: ext_epg::Attributes,
        #[serde(default)]
        children: Vec<ext_epg::ChildItem>,
    },
    L3extExtEncapAllocator {},
    L3extCtxUpdater {},
    RtctrlProfile {
        attributes: rtctrl::profile::Attributes,
        #[serde(default)]
        children: Vec<rtctrl::profile::ChildItem>,
    },
    BgpDomainIdAllocator {
        attributes: bgp::domain_id::Attributes,
        #[serde(default)]
        children: Vec<bgp::domain_id::ChildItem>,
    },
    BgpExtP {
        attributes: bgp::ext_epg::Attributes,
    },
    L3extRtOutDefContToOut {},
    L3extRtSrcToL3extOut {},
}
