use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::fantray;

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
    EqptFt(fantray::EqptFt),
    EqptRtOosSlot {},
    FaultCounts {},
    FaultInst {},
    HealthInst {},
}

#[derive(Debug, Clone)]
pub enum EqptFtSlotEndpoint {
    ClassAll,
    MoUni,
    MoExtch {
        pod: String,
        node: String,
        extch: String,
        ftslot: String,
    },
    MoSysExtch {
        extch: String,
        ftslot: String,
    },
    MoCh {
        pod: String,
        node: String,
        ftslot: String,
    },
    MoSysCh {
        ftslot: String,
    },
}

impl EndpointScheme for EqptFtSlotEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptFtSlot.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoExtch {
                pod,
                node,
                extch,
                ftslot,
            } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/ftslot-{ftslot}.json"
            )),
            Self::MoSysExtch { extch, ftslot } => {
                Cow::Owned(format!("mo/sys/extch-{extch}/ftslot-{ftslot}.json"))
            }
            Self::MoCh { pod, node, ftslot } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/ftslot-{ftslot}.json"
            )),
            Self::MoSysCh { ftslot } => Cow::Owned(format!("mo/sys/ch/ftslot-{ftslot}.json")),
        }
    }
}

pub type EqptFtSlot = AciObject<__internal::EqptFtSlot>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptFtSlot;
    impl AciObjectScheme for EqptFtSlot {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptFtSlotEndpoint;
        const CLASS_NAME: &'static str = "eqptFtSlot";
    }
}
