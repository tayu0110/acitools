use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::{as_segment, extended_community, regular_community};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    aggr: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    aggr_as: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    as_path: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ext_comm: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    flags: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    imported_rd: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    local_pref: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    metric: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    nh: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    nh_metric: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    oper_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    origin: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    peer: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    reg_comm: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    status: ConfigStatus,
    #[serde(rename = "type", skip_serializing_if = "String::is_empty")]
    r#type: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    unknown_attr_data: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    unknown_attr_len: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    weight: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    BgpAsSeg(as_segment::BgpAsSeg),
    BgpExtComm(extended_community::BgpExtComm),
    BgpRegComm(regular_community::BgpRegComm),
}

#[derive(Debug, Clone)]
pub enum BgpPathEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoVpnrt {
        pod: String,
        node: String,
        dom: String,
        af: String,
        vpnrt: String,
        rd: String,
        path: String,
        id: String,
        nh: String,
    },
    MoSysVpnrt {
        dom: String,
        af: String,
        vpnrt: String,
        rd: String,
        path: String,
        id: String,
        nh: String,
    },
    MoRt {
        pod: String,
        node: String,
        dom: String,
        af: String,
        rt: String,
        path: String,
        id: String,
        nh: String,
    },
    MoSysRt {
        dom: String,
        af: String,
        rt: String,
        path: String,
        id: String,
        nh: String,
    },
}

impl EndpointScheme for BgpPathEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpPath.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoVpnrt {
                pod,
                node,
                dom,
                af,
                vpnrt,
                rd,
                path,
                id,
                nh,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/af-{af}/vpnrt-[{vpnrt}]-{rd}/path-[{path}]-id-{id}-nh-[{nh}].json")),
            Self::MoSysVpnrt {
                dom,
                af,
                vpnrt,
                rd,
                path,
                id,
                nh,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/af-{af}/vpnrt-[{vpnrt}]-{rd}/path-[{path}]-id-{id}-nh-[{nh}].json")),
            Self::MoRt {
                pod,
                node,
                dom,
                af,
                rt,
                path,
                id,
                nh,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/af-{af}/rt-[{rt}]/path-[{path}]-id-{id}-nh-[{nh}].json")),
            Self::MoSysRt {
                dom,
                af,
                rt,
                path,
                id,
                nh,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/af-{af}/rt-[{rt}]/path-[{path}]-id-{id}-nh-[{nh}].json")),
        }
    }
}

pub type BgpPath = AciObject<__internal::BgpPath>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpPath;
    impl AciObjectScheme for BgpPath {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpPathEndpoint;
        const CLASS_NAME: &'static str = "bgpPath";
    }
}
