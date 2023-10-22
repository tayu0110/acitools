use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    af: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    descr: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    flags: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lc_own: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name_alias: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rt_map: String,
    status: ConfigStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    FaultDelegate {},
}

#[derive(Debug, Clone)]
pub enum BgpRtExpPEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoPeerAf {
        pod: String,
        node: String,
        dom: String,
        peer: String,
        af: String,
        rtexp: String,
    },
    MoSysPeerAf {
        dom: String,
        peer: String,
        af: String,
        rtexp: String,
    },
    MoAf {
        pod: String,
        node: String,
        dom: String,
        af: String,
        rtexp: String,
    },
    MoSysAf {
        dom: String,
        af: String,
        rtexp: String,
    },
}

impl EndpointScheme for BgpRtExpPEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpRtExpP.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoPeerAf {
                pod,
                node,
                dom,
                peer,
                af,
                rtexp,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/peer-[{peer}]/af-{af}/rtexp-{rtexp}.json")),
            Self::MoSysPeerAf {
                dom,
                peer,
                af,
                rtexp,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/peer-[{peer}]/af-{af}/rtexp-{rtexp}.json")),
            Self::MoAf {
                pod,
                node,
                dom,
                af,
                rtexp,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/af-{af}/rtexp-{rtexp}.json")),
            Self::MoSysAf {
                dom,
                af,
                rtexp,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/af-{af}/rtexp-{rtexp}.json")),
        }
    }
}

pub type BgpRtExpP = AciObject<__internal::BgpRtExpP>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpRtExpP;
    impl AciObjectScheme for BgpRtExpP {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpRtExpPEndpoint;
        const CLASS_NAME: &'static str = "bgpRtExpP";
    }
}
