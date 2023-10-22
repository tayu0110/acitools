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
    eti: String,
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
pub enum BgpEVpnImetRouteEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoBdevi {
        pod: String,
        node: String,
        dom: String,
        bdevi: String,
        evpnimetrt: String,
        eti: String,
        ip: String,
    },
    MoSysBdevi {
        dom: String,
        bdevi: String,
        evpnimetrt: String,
        eti: String,
        ip: String,
    },
}

impl EndpointScheme for BgpEVpnImetRouteEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpEVpnImetRoute.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoBdevi {
                pod,
                node,
                dom,
                bdevi,
                evpnimetrt,
                eti,
                ip,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/bdevi-[{bdevi}]/evpnimetrt-{evpnimetrt}-eti-{eti}-ip-[{ip}].json")),
            Self::MoSysBdevi {
                dom,
                bdevi,
                evpnimetrt,
                eti,
                ip,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/bdevi-[{bdevi}]/evpnimetrt-{evpnimetrt}-eti-{eti}-ip-[{ip}].json")),
        }
    }
}

pub type BgpEVpnImetRoute = AciObject<__internal::BgpEVpnImetRoute>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpEVpnImetRoute;
    impl AciObjectScheme for BgpEVpnImetRoute {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpEVpnImetRouteEndpoint;
        const CLASS_NAME: &'static str = "bgpEVpnImetRoute";
    }
}
