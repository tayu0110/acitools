use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    addr: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    if_id: String,
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
pub enum BgpAttNextHopEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoNh {
        pod: String,
        node: String,
        dom: String,
        af: String,
        nh: String,
        attnh: String,
    },
    MoSysNh {
        dom: String,
        af: String,
        nh: String,
        attnh: String,
    },
}

impl EndpointScheme for BgpAttNextHopEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpAttNextHop.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoNh {
                pod,
                node,
                dom,
                af,
                nh,
                attnh,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/af-{af}/nh-[{nh}]/attnh-[{attnh}].json")),
            Self::MoSysNh {
                dom,
                af,
                nh,
                attnh,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/af-{af}/nh-[{nh}]/attnh-[{attnh}].json")),
        }
    }
}

pub type BgpAttNextHop = AciObject<__internal::BgpAttNextHop>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpAttNextHop;
    impl AciObjectScheme for BgpAttNextHop {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpAttNextHopEndpoint;
        const CLASS_NAME: &'static str = "bgpAttNextHop";
    }
}
