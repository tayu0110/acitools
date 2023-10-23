use super::fc;
use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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
    #[serde(rename = "type", skip_serializing_if = "String::is_empty")]
    r#type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    #[serde(rename = "eqptFC")]
    EqptFc(fc::EqptFc),
    EqptRtOosSlot {},
    FaultCounts {},
    FaultInst {},
    HealthInst {},
    SyshistCardRstRec {},
}

#[derive(Debug, Clone)]
pub enum EqptFcSlotEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoCh {
        pod: String,
        node: String,
        fcslot: String,
    },
    MoSysCh {
        fcslot: String,
    },
}

impl EndpointScheme for EqptFcSlotEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptFCSlot.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoCh { pod, node, fcslot } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/fcslot-{fcslot}.json"
            )),
            Self::MoSysCh { fcslot } => Cow::Owned(format!("mo/sys/ch/fcslot-{fcslot}.json")),
        }
    }
}

pub type EqptFcSlot = AciObject<__internal::EqptFcSlot>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptFcSlot;
    impl AciObjectScheme for EqptFcSlot {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptFcSlotEndpoint;
        const CLASS_NAME: &'static str = "eqptFCSlot";
    }
}
