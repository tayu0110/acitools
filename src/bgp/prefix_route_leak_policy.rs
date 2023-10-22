use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ctrl: String,
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
    pfx: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rt_map: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    scope: String,
    status: ConfigStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    FaultDelegate {},
    IpCons {},
}

#[derive(Debug, Clone)]
pub enum BgpPfxLeakPEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoCtrl {
        pod: String,
        node: String,
        dom: String,
        af: String,
        ctrl: String,
        pfxleak: String,
    },
    MoSysCtrl {
        dom: String,
        af: String,
        ctrl: String,
        pfxleak: String,
    },
    MoAf {
        pod: String,
        node: String,
        dom: String,
        af: String,
        pfxleak: String,
    },
    MoSysAf {
        dom: String,
        af: String,
        pfxleak: String,
    },
}

impl EndpointScheme for BgpPfxLeakPEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpPfxLeakP.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoCtrl {
                pod,
                node,
                dom,
                af,
                ctrl,
                pfxleak,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/af-{af}/ctrl-{ctrl}/pfxleak-[{pfxleak}].json")),
            Self::MoSysCtrl {
                dom,
                af,
                ctrl,
                pfxleak,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/af-{af}/ctrl-{ctrl}/pfxleak-[{pfxleak}].json")),
            Self::MoAf {
                pod,
                node,
                dom,
                af,
                pfxleak,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/af-{af}/pfxleak-[{pfxleak}].json")),
            Self::MoSysAf {
                dom,
                af,
                pfxleak,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/af-{af}/pfxleak-[{pfxleak}].json")),
        }
    }
}

pub type BgpPfxLeakP = AciObject<__internal::BgpPfxLeakP>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpPfxLeakP;
    impl AciObjectScheme for BgpPfxLeakP {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpPfxLeakPEndpoint;
        const CLASS_NAME: &'static str = "bgpPfxLeakP";
    }
}
