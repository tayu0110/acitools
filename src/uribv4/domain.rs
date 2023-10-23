use super::db;
use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[allow(dead_code)]
    #[serde(skip_serializing, default)]
    mon_pol_dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    oper_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    status: ConfigStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    FaultCounts {},
    HealthInst {},
    Uribv4Db(db::Uribv4Db),
}

#[derive(Debug, Clone)]
pub enum Endpoint {
    ClassAll,
    MoUni,
    MoUribv4 {
        pod: String,
        node: String,
        dom: String,
    },
    MoSys {
        dom: String,
    },
}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/uribv4Dom.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoUribv4 { pod, node, dom } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/uribv4/dom-{dom}.json"
            )),
            Self::MoSys { dom } => Cow::Owned(format!("mo/sys/uribv4/dom-{dom}.json")),
        }
    }
}

pub type Uribv4Dom = AciObject<__internal::Uribv4Dom>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct Uribv4Dom;
    impl AciObjectScheme for Uribv4Dom {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "uribv4Dom";
    }
}
