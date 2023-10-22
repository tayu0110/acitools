use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    descr: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lc_own: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name_alias: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    nh: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    proxy_addr: String,
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
pub enum BgpPfxLeakCtrlPEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoCtrl {
        pod: String,
        node: String,
        dom: String,
        af: String,
        ctrl: String,
    },
    MoSysCtrl {
        dom: String,
        af: String,
        ctrl: String,
    },
}

impl EndpointScheme for BgpPfxLeakCtrlPEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpPfxLeakCtrlP.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoCtrl {
                pod,
                node,
                dom,
                af,
                ctrl,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/af-{af}/ctrl-{ctrl}/pfxleakctrl.json")),
            Self::MoSysCtrl {
                dom,
                af,
                ctrl,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/af-{af}/ctrl-{ctrl}/pfxleakctrl.json")),
        }
    }
}

pub type BgpPfxLeakCtrlP = AciObject<__internal::BgpPfxLeakCtrlP>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpPfxLeakCtrlP;
    impl AciObjectScheme for BgpPfxLeakCtrlP {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpPfxLeakCtrlPEndpoint;
        const CLASS_NAME: &'static str = "bgpPfxLeakCtrlP";
    }
}
