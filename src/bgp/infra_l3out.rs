use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    external_domain_id: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "infraL3outName")]
    infra_l_3_out_name: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "l3outInTag")]
    l_3_out_in_tag: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lc_own: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    status: ConfigStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {}

#[derive(Debug, Clone)]
pub enum BgpInfraL3OutEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoDom {
        pod: String,
        node: String,
        dom: String,
        infra_l_3_out: String,
    },
    MoSysDom {
        dom: String,
        infra_l_3_out: String,
    },
}

impl EndpointScheme for BgpInfraL3OutEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpInfraL3out.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoDom {
                pod,
                node,
                dom,
                infra_l_3_out,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/infraL3out-{infra_l_3_out}.json")),
            Self::MoSysDom {
                dom,
                infra_l_3_out,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/infraL3out-{infra_l_3_out}.json")),
        }
    }
}

pub type BgpInfraL3Out = AciObject<__internal::BgpInfraL3Out>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpInfraL3Out;
    impl AciObjectScheme for BgpInfraL3Out {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpInfraL3OutEndpoint;
        const CLASS_NAME: &'static str = "bgpInfraL3out";
    }
}
