use serde::{Deserialize, Serialize};
use crate::{AciObject, ConfigStatus, EndpointScheme};
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
    gwip: String,
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
pub enum ChildItem {
}

#[derive(Debug, Clone)]
pub enum BgpEVpnPfxRouteEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoBdevi {
        pod: String,
        node: String,
        dom: String,
        bdevi: String,
        evpnpfxrt: String,
        eti: String,
        ip: String,
    },
    MoSysBdevi {
        dom: String,
        bdevi: String,
        evpnpfxrt: String,
        eti: String,
        ip: String,
    },
}

impl EndpointScheme for BgpEVpnPfxRouteEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpEVpnPfxRoute.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoBdevi {
                pod,
                node,
                dom,
                bdevi,
                evpnpfxrt,
                eti,
                ip,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/bdevi-[{bdevi}]/evpnpfxrt-{evpnpfxrt}-eti-{eti}-ip-[{ip}].json")),
            Self::MoSysBdevi {
                dom,
                bdevi,
                evpnpfxrt,
                eti,
                ip,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/bdevi-[{bdevi}]/evpnpfxrt-{evpnpfxrt}-eti-{eti}-ip-[{ip}].json")),
        }
    }
}

pub type BgpEVpnPfxRoute = AciObject<__internal::BgpEVpnPfxRoute>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpEVpnPfxRoute;
    impl AciObjectScheme for BgpEVpnPfxRoute {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpEVpnPfxRouteEndpoint;
        const CLASS_NAME: &'static str = "bgpEVpnPfxRoute";
    }
}

