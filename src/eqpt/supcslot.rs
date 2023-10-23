use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::supc;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    card_oper_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    descr: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    loc: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[allow(dead_code)]
    #[serde(skip_serializing, default)]
    mon_pol_dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    oper_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    phys_id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    status: ConfigStatus,
    #[serde(rename = "type")]
    r#type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    EqptRtOosSlot {},
    EqptSupC(supc::EqptSupC),
    FaultCounts {},
    FaultInst {},
    HealthInst {},
    SyshistCardRstRec {},
    SyshistRemCardRstRec {},
}

#[derive(Debug, Clone)]
pub enum Endpoint {
    ClassAll,
    MoUni,
    MoCh {
        pod: String,
        node: String,
        supslot: String,
    },
    MoSys {
        supslot: String,
    },
}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptSupCSlot.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoCh { pod, node, supslot } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/supslot-{supslot}.json"
            )),
            Self::MoSys { supslot } => Cow::Owned(format!("mo/sys/ch/supslot-{supslot}.json")),
        }
    }
}

pub type EqptSupCSlot = AciObject<__internal::EqptSupCSlot>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptSupCSlot;
    impl AciObjectScheme for EqptSupCSlot {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "eqptSupCSlot";
    }
}
