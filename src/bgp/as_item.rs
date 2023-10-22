use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    asn: String,
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
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {}

#[derive(Debug, Clone)]
pub enum BgpAsItemEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoVpnrtSeg {
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
        asn: String,
    },
    MoSysVpnrtSeg {
        dom: String,
        af: String,
        vpnrt: String,
        rd: String,
        path: String,
        id: String,
        nh: String,
        seg: String,
        asn: String,
    },
    MoSeg {
        pod: String,
        node: String,
        dom: String,
        af: String,
        rt: String,
        path: String,
        id: String,
        nh: String,
        seg: String,
        asn: String,
    },
    MoSysSeg {
        dom: String,
        af: String,
        rt: String,
        path: String,
        id: String,
        nh: String,
        seg: String,
        asn: String,
    },
}

impl EndpointScheme for BgpAsItemEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpAsItem.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoVpnrtSeg {
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
                asn,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/af-{af}/vpnrt-[{vpnrt}]-{rd}/path-[{path}]-id-{id}-nh-[{nh}]/seg-{seg}/asn-{asn}.json")),
            Self::MoSysVpnrtSeg {
                dom,
                af,
                vpnrt,
                rd,
                path,
                id,
                nh,
                seg,
                asn,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/af-{af}/vpnrt-[{vpnrt}]-{rd}/path-[{path}]-id-{id}-nh-[{nh}]/seg-{seg}/asn-{asn}.json")),
            Self::MoSeg {
                pod,
                node,
                dom,
                af,
                rt,
                path,
                id,
                nh,
                seg,
                asn,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/af-{af}/rt-[{rt}]/path-[{path}]-id-{id}-nh-[{nh}]/seg-{seg}/asn-{asn}.json")),
            Self::MoSysSeg {
                dom,
                af,
                rt,
                path,
                id,
                nh,
                seg,
                asn,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/af-{af}/rt-[{rt}]/path-[{path}]-id-{id}-nh-[{nh}]/seg-{seg}/asn-{asn}.json")),
        }
    }
}

pub type BgpAsItem = AciObject<__internal::BgpAsItem>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpAsItem;
    impl AciObjectScheme for BgpAsItem {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpAsItemEndpoint;
        const CLASS_NAME: &'static str = "bgpAsItem";
    }
}
