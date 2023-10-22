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
    esi: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    eti: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mac: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pfx: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rd: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rt_flags: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    ver: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {}

#[derive(Debug, Clone)]
pub enum BgpEVpnMacIpRouteEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoBdevi {
        pod: String,
        node: String,
        dom: String,
        bdevi: String,
        evpnmaciprt: String,
        eti: String,
        mac: String,
        ip: String,
    },
    MoSysBdevi {
        dom: String,
        bdevi: String,
        evpnmaciprt: String,
        eti: String,
        mac: String,
        ip: String,
    },
}

impl EndpointScheme for BgpEVpnMacIpRouteEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpEVpnMacIpRoute.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoBdevi {
                pod,
                node,
                dom,
                bdevi,
                evpnmaciprt,
                eti,
                mac,
                ip,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/bdevi-[{bdevi}]/evpnmaciprt-{evpnmaciprt}-eti-{eti}-mac-{mac}-ip-[{ip}].json")),
            Self::MoSysBdevi {
                dom,
                bdevi,
                evpnmaciprt,
                eti,
                mac,
                ip,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/bdevi-[{bdevi}]/evpnmaciprt-{evpnmaciprt}-eti-{eti}-mac-{mac}-ip-[{ip}].json")),
        }
    }
}

pub type BgpEVpnMacIpRoute = AciObject<__internal::BgpEVpnMacIpRoute>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpEVpnMacIpRoute;
    impl AciObjectScheme for BgpEVpnMacIpRoute {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpEVpnMacIpRouteEndpoint;
        const CLASS_NAME: &'static str = "bgpEVpnMacIpRoute";
    }
}
