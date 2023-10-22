use super::route_target_policy;
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
pub enum BgpCktEpEviEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoBdevi {
        pod: String,
        node: String,
        dom: String,
        bdevi: String,
        cktepevi: String,
    },
    MoSysBdevi {
        dom: String,
        bdevi: String,
        cktepevi: String,
    },
}

impl EndpointScheme for BgpCktEpEviEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpCktEpEvi.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoBdevi {
                pod,
                node,
                dom,
                bdevi,
                cktepevi,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/bdevi-[{bdevi}]/cktepevi-[{cktepevi}].json")),
            Self::MoSysBdevi {
                dom,
                bdevi,
                cktepevi,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/bdevi-[{bdevi}]/cktepevi-[{cktepevi}].json")),
        }
    }
}

pub type BgpCktEpEvi = AciObject<__internal::BgpCktEpEvi>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpCktEpEvi;
    impl AciObjectScheme for BgpCktEpEvi {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpCktEpEviEndpoint;
        const CLASS_NAME: &'static str = "bgpCktEpEvi";
    }
}
