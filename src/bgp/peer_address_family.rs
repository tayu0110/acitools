use super::{
    default_route_leak_policy, maximum_prefix_policy, route_control_policy, route_export_policy,
};
use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    allowed_self_as_cnt: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ctrl: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ctrl_ext: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lc_own: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    soo: String,
    status: ConfigStatus,
    #[serde(rename = "type", skip_serializing_if = "String::is_empty")]
    r#type: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    weight: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    BgpDefRtLeakP(default_route_leak_policy::BgpDefRtLeakP),
    BgpMaxPfxP(maximum_prefix_policy::BgpMaxPfxP),
    BgpRtCtrlP(route_control_policy::BgpRtCtrlP),
    BgpRtExpP(route_export_policy::BgpRtExpP),
}

#[derive(Debug, Clone)]
pub enum BgpPeerAfEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoPeer {
        pod: String,
        node: String,
        dom: String,
        peer: String,
        af: String,
    },
    MoSysPeer {
        dom: String,
        peer: String,
        af: String,
    },
}

impl EndpointScheme for BgpPeerAfEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpPeerAf.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoPeer {
                pod,
                node,
                dom,
                peer,
                af,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/peer-[{peer}]/af-{af}.json")),
            Self::MoSysPeer {
                dom,
                peer,
                af,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/peer-[{peer}]/af-{af}.json")),
        }
    }
}

pub type BgpPeerAf = AciObject<__internal::BgpPeerAf>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpPeerAf;
    impl AciObjectScheme for BgpPeerAf {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpPeerAfEndpoint;
        const CLASS_NAME: &'static str = "bgpPeerAf";
    }
}
