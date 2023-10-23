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
pub enum ChildItem {}

#[derive(Debug, Clone)]
pub enum Uribv6NexthopEndpoint {
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
    },
    MoSysOwner {
        dom: String,
        db: String,
        rt: String,
        owner: String,
        nh: String,
    },
    MoRt {
        pod: String,
        node: String,
        dom: String,
        db: String,
        rt: String,
        nh: String,
    },
    MoSysRt {
        dom: String,
        db: String,
        rt: String,
        nh: String,
    },
}

impl EndpointScheme for Uribv6NexthopEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/uribv6Nexthop.json"),
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
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/uribv6/dom-{dom}/db-{db}/rt-[{rt}]/owner-[{owner}]/nh-[{nh}]-[{addr}]-[{if}]-[{vrf}].json")),
            Self::MoSysOwner {
                dom,
                db,
                rt,
                owner,
                nh,
            } => Cow::Owned(format!("mo/sys/uribv6/dom-{dom}/db-{db}/rt-[{rt}]/owner-[{owner}]/nh-[{nh}]-[{addr}]-[{if}]-[{vrf}].json")),
            Self::MoRt {
                pod,
                node,
                dom,
                db,
                rt,
                nh,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/uribv6/dom-{dom}/db-{db}/rt-[{rt}]/nh-[{nh}]-[{addr}]-[{if}]-[{vrf}].json")),
            Self::MoSysRt {
                dom,
                db,
                rt,
                nh,
            } => Cow::Owned(format!("mo/sys/uribv6/dom-{dom}/db-{db}/rt-[{rt}]/nh-[{nh}]-[{addr}]-[{if}]-[{vrf}].json")),
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
        type Endpoint = Uribv6NexthopEndpoint;
        const CLASS_NAME: &'static str = "uribv6Nexthop";
    }
}
