use super::db;
use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
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
    Uribv6Db(db::Uribv6Db),
}

#[derive(Debug, Clone)]
pub enum Endpoint {
    ClassAll,
    MoUni,
    MoUribv6 {
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
            Self::ClassAll => Cow::Borrowed("node/class/uribv6Dom.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoUribv6 { pod, node, dom } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/uribv6/dom-{dom}.json"
            )),
            Self::MoSys { dom } => Cow::Owned(format!("mo/sys/uribv6/dom-{dom}.json")),
        }
    }
}

pub type Uribv6Dom = AciObject<__internal::Uribv6Dom>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct Uribv6Dom;
    impl AciObjectScheme for Uribv6Dom {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "uribv6Dom";
    }
}
