use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::nexthop;

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
    owner: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    status: ConfigStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    FaultCounts {},
    HealthInst {},
    Uribv4Nexthop(nexthop::Uribv4Nexthop),
    Uribv4RsRouteOwnerToNexthopAtt {},
}

#[derive(Debug, Clone)]
pub enum Endpoint {
    ClassAll,
    MoUni,
    MoRt {
        pod: String,
        node: String,
        dom: String,
        db: String,
        rt: String,
        owner: String,
    },
    MoSysRt {
        dom: String,
        db: String,
        rt: String,
        owner: String,
    },
}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/uribv4RouteOwner.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoRt {
                pod,
                node,
                dom,
                db,
                rt,
                owner,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/uribv4/dom-{dom}/db-{db}/rt-[{rt}]/owner-[{owner}].json")),
            Self::MoSysRt {
                dom,
                db,
                rt,
                owner,
            } => Cow::Owned(format!("mo/sys/uribv4/dom-{dom}/db-{db}/rt-[{rt}]/owner-[{owner}].json")),
        }
    }
}

pub type Uribv4RouteOwner = AciObject<__internal::Uribv4RouteOwner>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct Uribv4RouteOwner;
    impl AciObjectScheme for Uribv4RouteOwner {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "uribv4RouteOwner";
    }
}
