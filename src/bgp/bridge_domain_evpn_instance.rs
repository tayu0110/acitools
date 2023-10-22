use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::{
    circuit_endpoint_evpn_instance, evpn_imet_route, evpn_ip_prefix_route, evpn_mac_ip_route,
    route_target_policy,
};

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
    BgpCktEpEvi(circuit_endpoint_evpn_instance::BgpCktEpEvi),
    BgpEVpnImetRoute(evpn_imet_route::BgpEVpnImetRoute),
    BgpEVpnMacIpRoute(evpn_mac_ip_route::BgpEVpnMacIpRoute),
    BgpEVpnPfxRoute(evpn_ip_prefix_route::BgpEVpnPfxRoute),
    BgpRttP(route_target_policy::BgpRttP),
}

#[derive(Debug, Clone)]
pub enum BgpBdEviEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoDom {
        pod: String,
        node: String,
        dom: String,
        bdevi: String,
    },
    MoSysDom {
        dom: String,
        bdevi: String,
    },
}

impl EndpointScheme for BgpBdEviEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpBDEvi.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoDom {
                pod,
                node,
                dom,
                bdevi,
            } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/bdevi-[{bdevi}].json"
            )),
            Self::MoSysDom { dom, bdevi } => {
                Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/bdevi-[{bdevi}].json"))
            }
        }
    }
}

pub type BgpBdEvi = AciObject<__internal::BgpBdEvi>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpBdEvi;
    impl AciObjectScheme for BgpBdEvi {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpBdEviEndpoint;
        const CLASS_NAME: &'static str = "bgpBDEvi";
    }
}
