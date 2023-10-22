use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::route_target_policy;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    encap: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lc_own: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rd: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    status: ConfigStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    BgpRttP(route_target_policy::BgpRttP),
}

#[derive(Debug, Clone)]
pub enum BgpDomEviEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoDom {
        pod: String,
        node: String,
        dom: String,
    },
    MoSysDom {
        dom: String,
    },
}

impl EndpointScheme for BgpDomEviEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpDomEvi.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoDom { pod, node, dom } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/evi.json"
            )),
            Self::MoSysDom { dom } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/evi.json")),
        }
    }
}

pub type BgpDomEvi = AciObject<__internal::BgpDomEvi>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpDomEvi;
    impl AciObjectScheme for BgpDomEvi {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpDomEviEndpoint;
        const CLASS_NAME: &'static str = "bgpDomEvi";
    }
}
