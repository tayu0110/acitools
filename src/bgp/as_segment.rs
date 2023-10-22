use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::as_item;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    order: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    status: ConfigStatus,
    #[serde(rename = "type", skip_serializing_if = "String::is_empty")]
    r#type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    BgpAsItem(as_item::BgpAsItem),
}

#[derive(Debug, Clone)]
pub enum BgpAsSegEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoVpnrtPath {
        pod: String,
        node: String,
        dom: String,
        af: String,
        vpnrt: String,
        rd: String,
        path: String,
        id: String,
        nh: String,
        seg: String,
    },
    MoSysVpnrtPath {
        dom: String,
        af: String,
        vpnrt: String,
        rd: String,
        path: String,
        id: String,
        nh: String,
        seg: String,
    },
    MoPath {
        pod: String,
        node: String,
        dom: String,
        af: String,
        rt: String,
        path: String,
        id: String,
        nh: String,
        seg: String,
    },
    MoSysPath {
        dom: String,
        af: String,
        rt: String,
        path: String,
        id: String,
        nh: String,
        seg: String,
    },
}

impl EndpointScheme for BgpAsSegEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpAsSeg.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoVpnrtPath {
                pod,
                node,
                dom,
                af,
                vpnrt,
                rd,
                path,
                id,
                nh,
                seg,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/af-{af}/vpnrt-[{vpnrt}]-{rd}/path-[{path}]-id-{id}-nh-[{nh}]/seg-{seg}.json")),
            Self::MoSysVpnrtPath {
                dom,
                af,
                vpnrt,
                rd,
                path,
                id,
                nh,
                seg,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/af-{af}/vpnrt-[{vpnrt}]-{rd}/path-[{path}]-id-{id}-nh-[{nh}]/seg-{seg}.json")),
            Self::MoPath {
                pod,
                node,
                dom,
                af,
                rt,
                path,
                id,
                nh,
                seg,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/af-{af}/rt-[{rt}]/path-[{path}]-id-{id}-nh-[{nh}]/seg-{seg}.json")),
            Self::MoSysPath {
                dom,
                af,
                rt,
                path,
                id,
                nh,
                seg,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/af-{af}/rt-[{rt}]/path-[{path}]-id-{id}-nh-[{nh}]/seg-{seg}.json")),
        }
    }
}

pub type BgpAsSeg = AciObject<__internal::BgpAsSeg>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpAsSeg;
    impl AciObjectScheme for BgpAsSeg {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpAsSegEndpoint;
        const CLASS_NAME: &'static str = "bgpAsSeg";
    }
}
