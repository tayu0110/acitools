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
    #[allow(dead_code)]
    #[serde(skip_serializing, default)]
    mon_pol_dn: String,
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
    FaultCounts {},
    HealthInst {},
    Uribv4Route(route::Uribv4Route),
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
            Self::ClassAll => Cow::Borrowed("node/class/uribv4Db.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoDom { pod, node, dom, db } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/uribv4/dom-{dom}/db-{db}.json"
            )),
            Self::MoSys { dom, db } => Cow::Owned(format!("mo/sys/uribv4/dom-{dom}/db-{db}.json")),
        }
    }
}

pub type Uribv4Db = AciObject<__internal::Uribv4Db>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct Uribv4Db;
    impl AciObjectScheme for Uribv4Db {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "uribv4Db";
    }
}
