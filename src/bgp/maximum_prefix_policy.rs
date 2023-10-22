use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    descr: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lc_own: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    max_pfx: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name_alias: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    restart_time: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    thresh: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    FaultDelegate {},
}

#[derive(Debug, Clone)]
pub enum BgpMaxPfxPEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoAf {
        pod: String,
        node: String,
        dom: String,
        peer: String,
        af: String,
    },
    MoSysAf {
        dom: String,
        peer: String,
        af: String,
    },
}

impl EndpointScheme for BgpMaxPfxPEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpMaxPfxP.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoAf {
                pod,
                node,
                dom,
                peer,
                af,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/peer-[{peer}]/af-{af}/maxpfxp.json")),
            Self::MoSysAf {
                dom,
                peer,
                af,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/peer-[{peer}]/af-{af}/maxpfxp.json")),
        }
    }
}

pub type BgpMaxPfxP = AciObject<__internal::BgpMaxPfxP>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpMaxPfxP;
    impl AciObjectScheme for BgpMaxPfxP {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpMaxPfxPEndpoint;
        const CLASS_NAME: &'static str = "bgpMaxPfxP";
    }
}
