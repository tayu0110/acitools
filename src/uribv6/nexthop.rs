use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    nhmetric: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    nhtag: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    active: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    addr: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    create_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "if")]
    r#if: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    metric: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mpls_label: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    owner: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pref: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    route_type: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rw_vnid: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    tag: String,
    #[serde(rename = "type")]
    r#type: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    vrf: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {}

#[derive(Debug, Clone)]
pub enum Endpoint {
    ClassAll,
    MoUni,
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
            Self::ClassAll => Cow::Borrowed("node/class/uribv6Nexthop.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
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
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/uribv6/dom-{dom}/db-{db}/rt-[{rt}]/owner-[{owner}]/nh-[{nh}]-[{addr}]-[{interface}]-[{vrf}].json")),
            Self::MoSysOwner {
                dom,
                db,
                rt,
                owner,
                nh,
                addr,
                interface,
                vrf,
            } => Cow::Owned(format!("mo/sys/uribv6/dom-{dom}/db-{db}/rt-[{rt}]/owner-[{owner}]/nh-[{nh}]-[{addr}]-[{interface}]-[{vrf}].json")),
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
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/uribv6/dom-{dom}/db-{db}/rt-[{rt}]/nh-[{nh}]-[{addr}]-[{interface}]-[{vrf}].json")),
            Self::MoSysRt {
                dom,
                db,
                rt,
                nh,
                addr,
                interface,
                vrf,
            } => Cow::Owned(format!("mo/sys/uribv6/dom-{dom}/db-{db}/rt-[{rt}]/nh-[{nh}]-[{addr}]-[{interface}]-[{vrf}].json")),
        }
    }
}

pub type Uribv6Nexthop = AciObject<__internal::Uribv6Nexthop>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct Uribv6Nexthop;
    impl AciObjectScheme for Uribv6Nexthop {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "uribv6Nexthop";
    }
}
