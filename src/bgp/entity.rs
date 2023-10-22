use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::instance;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    admin_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lc_own: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mon_pol_dn: String,
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
    BgpInst(instance::BgpInst),
    FaultCounts {},
    HealthInst {},
}

#[derive(Debug, Clone)]
pub enum BgpEntityEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoSys { pod: String, node: String },
    MoSysSys,
}

impl EndpointScheme for BgpEntityEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpEntity.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoSys { pod, node } => {
                Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp.json"))
            }
            Self::MoSysSys => Cow::Borrowed("mo/sys/bgp.json"),
        }
    }
}

pub type BgpEntity = AciObject<__internal::BgpEntity>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpEntity;
    impl AciObjectScheme for BgpEntity {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpEntityEndpoint;
        const CLASS_NAME: &'static str = "bgpEntity";
    }
}
