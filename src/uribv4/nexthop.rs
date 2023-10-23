use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty", rename = "Nhmetric", default)]
    nhmetric: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "Nhtag", default)]
    nhtag: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    active: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    addr: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    create_ts: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(rename = "if", skip_serializing_if = "String::is_empty", default)]
    r#if: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    metric: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    mod_ts: String,
    #[allow(dead_code)]
    #[serde(skip_serializing, default)]
    mon_pol_dn: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    mpls_label: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    owner: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pref: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    route_type: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    rw_vnid: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    tag: String,
    #[serde(rename = "type", skip_serializing_if = "String::is_empty", default)]
    r#type: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    vrf: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    FaultCounts(serde_json::Value),
    FaultInst(serde_json::Value),
    HealthInst(serde_json::Value),
    #[serde(rename = "uribv4EgrBytes15min")]
    Uribv4EgrBytes15Min(serde_json::Value),
    #[serde(rename = "uribv4EgrBytes1d")]
    Uribv4EgrBytes1D(serde_json::Value),
    #[serde(rename = "uribv4EgrBytes1h")]
    Uribv4EgrBytes1H(serde_json::Value),
    #[serde(rename = "uribv4EgrBytes1mo")]
    Uribv4EgrBytes1Mo(serde_json::Value),
    #[serde(rename = "uribv4EgrBytes1qtr")]
    Uribv4EgrBytes1Qtr(serde_json::Value),
    #[serde(rename = "uribv4EgrBytes1w")]
    Uribv4EgrBytes1W(serde_json::Value),
    #[serde(rename = "uribv4EgrBytes1year")]
    Uribv4EgrBytes1Year(serde_json::Value),
    #[serde(rename = "uribv4EgrBytes5min")]
    Uribv4EgrBytes5Min(serde_json::Value),
    #[serde(rename = "uribv4EgrBytesHist15min")]
    Uribv4EgrBytesHist15Min(serde_json::Value),
    #[serde(rename = "uribv4EgrBytesHist1d")]
    Uribv4EgrBytesHist1D(serde_json::Value),
    #[serde(rename = "uribv4EgrBytesHist1h")]
    Uribv4EgrBytesHist1H(serde_json::Value),
    #[serde(rename = "uribv4EgrBytesHist1mo")]
    Uribv4EgrBytesHist1Mo(serde_json::Value),
    #[serde(rename = "uribv4EgrBytesHist1qtr")]
    Uribv4EgrBytesHist1Qtr(serde_json::Value),
    #[serde(rename = "uribv4EgrBytesHist1w")]
    Uribv4EgrBytesHist1W(serde_json::Value),
    #[serde(rename = "uribv4EgrBytesHist1year")]
    Uribv4EgrBytesHist1Year(serde_json::Value),
    #[serde(rename = "uribv4EgrBytesHist5min")]
    Uribv4EgrBytesHist5Min(serde_json::Value),
    #[serde(rename = "uribv4EgrPkts15min")]
    Uribv4EgrPkts15Min(serde_json::Value),
    #[serde(rename = "uribv4EgrPkts1d")]
    Uribv4EgrPkts1D(serde_json::Value),
    #[serde(rename = "uribv4EgrPkts1h")]
    Uribv4EgrPkts1H(serde_json::Value),
    #[serde(rename = "uribv4EgrPkts1mo")]
    Uribv4EgrPkts1Mo(serde_json::Value),
    #[serde(rename = "uribv4EgrPkts1qtr")]
    Uribv4EgrPkts1Qtr(serde_json::Value),
    #[serde(rename = "uribv4EgrPkts1w")]
    Uribv4EgrPkts1W(serde_json::Value),
    #[serde(rename = "uribv4EgrPkts1year")]
    Uribv4EgrPkts1Year(serde_json::Value),
    #[serde(rename = "uribv4EgrPkts5min")]
    Uribv4EgrPkts5Min(serde_json::Value),
    #[serde(rename = "uribv4EgrPktsHist15min")]
    Uribv4EgrPktsHist15Min(serde_json::Value),
    #[serde(rename = "uribv4EgrPktsHist1d")]
    Uribv4EgrPktsHist1D(serde_json::Value),
    #[serde(rename = "uribv4EgrPktsHist1h")]
    Uribv4EgrPktsHist1H(serde_json::Value),
    #[serde(rename = "uribv4EgrPktsHist1mo")]
    Uribv4EgrPktsHist1Mo(serde_json::Value),
    #[serde(rename = "uribv4EgrPktsHist1qtr")]
    Uribv4EgrPktsHist1Qtr(serde_json::Value),
    #[serde(rename = "uribv4EgrPktsHist1w")]
    Uribv4EgrPktsHist1W(serde_json::Value),
    #[serde(rename = "uribv4EgrPktsHist1year")]
    Uribv4EgrPktsHist1Year(serde_json::Value),
    #[serde(rename = "uribv4EgrPktsHist5min")]
    Uribv4EgrPktsHist5Min(serde_json::Value),
    Uribv4MplsNexthop(serde_json::Value),
    Uribv4RtRouteOwnerToNexthopAtt(serde_json::Value),
}

#[derive(Debug, Clone)]
pub enum Endpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoOwner {
        pod: String,
        node: String,
        dom: String,
        db: String,
        rt: String,
        owner: String,
        nh: String,
        addr: String,
        interface: String,
        vrf: String,
    },
    MoSysOwner {
        dom: String,
        db: String,
        rt: String,
        owner: String,
        nh: String,
        addr: String,
        interface: String,
        vrf: String,
    },
    MoRt {
        pod: String,
        node: String,
        dom: String,
        db: String,
        rt: String,
        nh: String,
        addr: String,
        interface: String,
        vrf: String,
    },
    MoSysRt {
        dom: String,
        db: String,
        rt: String,
        nh: String,
        addr: String,
        interface: String,
        vrf: String,
    },
}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/uribv4Nexthop.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoOwner {
                pod,
                node,
                dom,
                db,
                rt,
                owner,
                nh,
                addr,
                interface,
                vrf,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/uribv4/dom-{dom}/db-{db}/rt-[{rt}]/owner-[{owner}]/nh-[{nh}]-[{addr}]-[{interface}]-[{vrf}].json")),
            Self::MoSysOwner {
                dom,
                db,
                rt,
                owner,
                nh,
                addr,
                interface,
                vrf,
            } => Cow::Owned(format!("mo/sys/uribv4/dom-{dom}/db-{db}/rt-[{rt}]/owner-[{owner}]/nh-[{nh}]-[{addr}]-[{interface}]-[{vrf}].json")),
            Self::MoRt {
                pod,
                node,
                dom,
                db,
                rt,
                nh,
                addr,
                interface,
                vrf,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/uribv4/dom-{dom}/db-{db}/rt-[{rt}]/nh-[{nh}]-[{addr}]-[{interface}]-[{vrf}].json")),
            Self::MoSysRt {
                dom,
                db,
                rt,
                nh,
                addr,
                interface,
                vrf,
            } => Cow::Owned(format!("mo/sys/uribv4/dom-{dom}/db-{db}/rt-[{rt}]/nh-[{nh}]-[{addr}]-[{interface}]-[{vrf}].json")),
        }
    }
}

pub type Uribv4Nexthop = AciObject<__internal::Uribv4Nexthop>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct Uribv4Nexthop;
    impl AciObjectScheme for Uribv4Nexthop {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "uribv4Nexthop";
    }
}
