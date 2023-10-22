use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::system_controller;

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
    #[serde(skip_serializing_if = "String::is_empty")]
    mon_pol_dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    oper_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    phys_id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    status: ConfigStatus,
    #[serde(rename = "type", skip_serializing_if = "String::is_empty")]
    r#type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    EqptRtOosSlot {},
    EqptSysC(system_controller::EqptSysC),
    FaultCounts {},
    FaultInst {},
    HealthInst {},
    SyshistCardRstRec {},
}

#[derive(Debug, Clone)]
pub enum EqptSysCSlotEndpoint {
    ClassAll,
    MoUni,
    MoCh {
        pod: String,
        node: String,
        scslot: String,
    },
    MoSysCh {
        scslot: String,
    },
}

impl EndpointScheme for EqptSysCSlotEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptSysCSlot.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoCh { pod, node, scslot } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/scslot-{scslot}.json"
            )),
            Self::MoSysCh { scslot } => Cow::Owned(format!("mo/sys/ch/scslot-{scslot}.json")),
        }
    }
}

pub type EqptSysCSlot = AciObject<__internal::EqptSysCSlot>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptSysCSlot;
    impl AciObjectScheme for EqptSysCSlot {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptSysCSlotEndpoint;
        const CLASS_NAME: &'static str = "eqptSysCSlot";
    }
}
