use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::{
    additional_paths, administrative_distance, dampening_control, default_route_leak_policy,
    host_route_leak_policy, interprotocol_route_leak_policy, nexthop, prefix_route_leak_policy,
    rib_route_leak_policy, route, route_export_policy, route_summarization, route_target_policy,
    vpn_control_policy, vpn_route,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    alloc_lbl_all: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    alloc_lbl_rt_map: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    best_path_cmplt_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    best_path_sig_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    crit_nh_timeout: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lc_own: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    max_ecmp: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    max_ecmp_ibgp: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    max_local_ecmp: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    non_crit_nh_timeout: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    tbl_ver: String,
    #[serde(rename = "type", skip_serializing_if = "String::is_empty")]
    r#type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    BgpAddlPath(additional_paths::BgpAddlPath),
    BgpAdminDist(administrative_distance::BgpAdminDist),
    BgpDampeningCtrl(dampening_control::BgpDampeningCtrl),
    BgpDefRtLeakP(default_route_leak_policy::BgpDefRtLeakP),
    BgpHostLeakP(host_route_leak_policy::BgpHostLeakP),
    BgpInterLeakP(interprotocol_route_leak_policy::BgpInterLeakP),
    BgpNextHop(nexthop::BgpNextHop),
    BgpPfxLeakP(prefix_route_leak_policy::BgpPfxLeakP),
    BgpRibLeakP(rib_route_leak_policy::BgpRibLeakP),
    BgpRoute(route::BgpRoute),
    BgpRtExpP(route_export_policy::BgpRtExpP),
    BgpRtSum(route_summarization::BgpRtSum),
    BgpRttP(route_target_policy::BgpRttP),
    BgpVpnCtrlP(vpn_control_policy::BgpVpnCtrlP),
    BgpVpnRoute(vpn_route::BgpVpnRoute),
}

#[derive(Debug, Clone)]
pub enum BgpDomAfEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoDom {
        pod: String,
        node: String,
        dom: String,
        af: String,
    },
    MoSysDom {
        dom: String,
        af: String,
    },
}

impl EndpointScheme for BgpDomAfEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpDomAf.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoDom { pod, node, dom, af } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/af-{af}.json"
            )),
            Self::MoSysDom { dom, af } => {
                Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/af-{af}.json"))
            }
        }
    }
}

pub type BgpDomAf = AciObject<__internal::BgpDomAf>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpDomAf;
    impl AciObjectScheme for BgpDomAf {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpDomAfEndpoint;
        const CLASS_NAME: &'static str = "bgpDomAf";
    }
}
