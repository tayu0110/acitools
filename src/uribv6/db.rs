use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::route;

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
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    status: ConfigStatus,
    #[serde(rename = "type")]
    r#type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    Uribv6Route(route::Uribv6Route),
}

#[derive(Debug, Clone)]
pub enum Endpoint {
    ClassAll,
    MoUni,
    MoDom {
        pod: String,
        node: String,
        dom: String,
        db: String,
    },
    MoSys {
        dom: String,
        db: String,
    },
}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/uribv6Db.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoDom { pod, node, dom, db } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/uribv6/dom-{dom}/db-{db}.json"
            )),
            Self::MoSys { dom, db } => Cow::Owned(format!("mo/sys/uribv6/dom-{dom}/db-{db}.json")),
        }
    }
}

pub type Uribv6Db = AciObject<__internal::Uribv6Db>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct Uribv6Db;
    impl AciObjectScheme for Uribv6Db {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "uribv6Db";
    }
}
