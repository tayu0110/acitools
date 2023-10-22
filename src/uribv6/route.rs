use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::{nexthop, route_owner};

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
    prefix: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    prefix_length: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    status: ConfigStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    Uribv6Nexthop(nexthop::Uribv6Nexthop),
    Uribv6RouteOwner(route_owner::Uribv6RouteOwner),
}

#[derive(Debug, Clone)]
pub enum Endpoint {
    ClassAll,
    MoUni,
    MoDb {
        pod: String,
        node: String,
        dom: String,
        db: String,
        rt: String,
    },
    MoSys {
        dom: String,
        db: String,
        rt: String,
    },
}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/uribv6Route.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoDb {
                pod,
                node,
                dom,
                db,
                rt,
            } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/uribv6/dom-{dom}/db-{db}/rt-[{rt}].json"
            )),
            Self::MoSys { dom, db, rt } => {
                Cow::Owned(format!("mo/sys/uribv6/dom-{dom}/db-{db}/rt-[{rt}].json"))
            }
        }
    }
}

pub type Uribv6Route = AciObject<__internal::Uribv6Route>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct Uribv6Route;
    impl AciObjectScheme for Uribv6Route {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "uribv6Route";
    }
}
