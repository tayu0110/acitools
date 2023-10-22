use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::domain;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    admin_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    oper_err: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    oper_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    status: ConfigStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    Uribv6Dom(domain::Uribv6Dom),
}

#[derive(Debug, Clone)]
pub enum Endpoint {
    ClassAll,
    MoUni,
    MoNode { pod: String, node: String },
    MoSys,
}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/uribv6Entity.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoNode { pod, node } => {
                Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/uribv6.json"))
            }
            Self::MoSys => Cow::Owned(format!("mo/sys/uribv6.json")),
        }
    }
}

pub type Uribv6Entity = AciObject<__internal::Uribv6Entity>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct Uribv6Entity;
    impl AciObjectScheme for Uribv6Entity {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "uribv6Entity";
    }
}
