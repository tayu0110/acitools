use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    community: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    status: ConfigStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {}

#[derive(Debug, Clone)]
pub enum BgpRegCommEndpoint {
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
        rcomm: String,
    },
    MoSysVpnrtPath {
        dom: String,
        af: String,
        vpnrt: String,
        rd: String,
        path: String,
        id: String,
        nh: String,
        rcomm: String,
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
        rcomm: String,
    },
    MoSysPath {
        dom: String,
        af: String,
        rt: String,
        path: String,
        id: String,
        nh: String,
        rcomm: String,
    },
}

impl EndpointScheme for BgpRegCommEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpRegComm.json"),
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
                rcomm,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/af-{af}/vpnrt-[{vpnrt}]-{rd}/path-[{path}]-id-{id}-nh-[{nh}]/rcomm-{rcomm}.json")),
            Self::MoSysVpnrtPath {
                dom,
                af,
                vpnrt,
                rd,
                path,
                id,
                nh,
                rcomm,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/af-{af}/vpnrt-[{vpnrt}]-{rd}/path-[{path}]-id-{id}-nh-[{nh}]/rcomm-{rcomm}.json")),
            Self::MoPath {
                pod,
                node,
                dom,
                af,
                rt,
                path,
                id,
                nh,
                rcomm,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/af-{af}/rt-[{rt}]/path-[{path}]-id-{id}-nh-[{nh}]/rcomm-{rcomm}.json")),
            Self::MoSysPath {
                dom,
                af,
                rt,
                path,
                id,
                nh,
                rcomm,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/af-{af}/rt-[{rt}]/path-[{path}]-id-{id}-nh-[{nh}]/rcomm-{rcomm}.json")),
        }
    }
}

pub type BgpRegComm = AciObject<__internal::BgpRegComm>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpRegComm;
    impl AciObjectScheme for BgpRegComm {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpRegCommEndpoint;
        const CLASS_NAME: &'static str = "bgpRegComm";
    }
}
