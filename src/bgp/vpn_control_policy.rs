use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::{prefix_route_leak_control_policy, prefix_route_leak_policy, route_policy};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
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
    status: ConfigStatus,
    #[serde(rename = "type", skip_serializing_if = "String::is_empty")]
    r#type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    BgpPfxLeakCtrlP(prefix_route_leak_control_policy::BgpPfxLeakCtrlP),
    BgpPfxLeakP(prefix_route_leak_policy::BgpPfxLeakP),
    BgpRtP(route_policy::BgpRtP),
}

#[derive(Debug, Clone)]
pub enum BgpVpnCtrlPEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoAf {
        pod: String,
        node: String,
        dom: String,
        af: String,
        ctrl: String,
    },
    MoSysAf {
        dom: String,
        af: String,
        ctrl: String,
    },
}

impl EndpointScheme for BgpVpnCtrlPEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpVpnCtrlP.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoAf {
                pod,
                node,
                dom,
                af,
                ctrl,
            } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/af-{af}/ctrl-{ctrl}.json"
            )),
            Self::MoSysAf { dom, af, ctrl } => Cow::Owned(format!(
                "mo/sys/bgp/inst/dom-{dom}/af-{af}/ctrl-{ctrl}.json"
            )),
        }
    }
}

pub type BgpVpnCtrlP = AciObject<__internal::BgpVpnCtrlP>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpVpnCtrlP;
    impl AciObjectScheme for BgpVpnCtrlP {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpVpnCtrlPEndpoint;
        const CLASS_NAME: &'static str = "bgpVpnCtrlP";
    }
}
