use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    asn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    descr: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    inst: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lc_own: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name_alias: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    proto: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rt_map: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    scope: String,
    status: ConfigStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    FaultDelegate {},
}

#[derive(Debug, Clone)]
pub enum BgpInterLeakPEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoAf {
        pod: String,
        node: String,
        dom: String,
        af: String,
        proto: String,
        inst: String,
    },
    MoSysAf {
        dom: String,
        af: String,
        proto: String,
        inst: String,
    },
}

impl EndpointScheme for BgpInterLeakPEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpInterLeakP.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoAf {
                pod,
                node,
                dom,
                af,
                proto,
                inst,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/af-{af}/interleak-{proto}-interleak-{inst}.json")),
            Self::MoSysAf {
                dom,
                af,
                proto,
                inst,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/af-{af}/interleak-{proto}-interleak-{inst}.json")),
        }
    }
}

pub type BgpInterLeakP = AciObject<__internal::BgpInterLeakP>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpInterLeakP;
    impl AciObjectScheme for BgpInterLeakP {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpInterLeakPEndpoint;
        const CLASS_NAME: &'static str = "bgpInterLeakP";
    }
}
