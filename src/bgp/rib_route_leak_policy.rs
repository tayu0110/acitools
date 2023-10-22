use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    always: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lc_own: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rt_map: String,
    status: ConfigStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {}

#[derive(Debug, Clone)]
pub enum BgpRibLeakPEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoAf {
        pod: String,
        node: String,
        dom: String,
        af: String,
    },
    MoSysAf {
        dom: String,
        af: String,
    },
}

impl EndpointScheme for BgpRibLeakPEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpRibLeakP.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoAf { pod, node, dom, af } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/af-{af}/ribleak.json"
            )),
            Self::MoSysAf { dom, af } => {
                Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/af-{af}/ribleak.json"))
            }
        }
    }
}

pub type BgpRibLeakP = AciObject<__internal::BgpRibLeakP>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpRibLeakP;
    impl AciObjectScheme for BgpRibLeakP {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpRibLeakPEndpoint;
        const CLASS_NAME: &'static str = "bgpRibLeakP";
    }
}
