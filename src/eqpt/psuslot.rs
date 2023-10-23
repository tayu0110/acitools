use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::psu;

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
    EqptPsu(psu::EqptPsu),
    EqptRtOosSlot {},
    FaultCounts {},
    FaultInst {},
    HealthInst {},
}

#[derive(Debug, Clone)]
pub enum EqptPsuSlotEndpoint {
    ClassAll,
    MoUni,
    MoExtCh {
        pod: String,
        node: String,
        extch: String,
        psuslot: String,
    },
    MoSysExtCh {
        extch: String,
        psuslot: String,
    },
    MoCh {
        pod: String,
        node: String,
        psuslot: String,
    },
    MoSysCh {
        psuslot: String,
    },
}

impl EndpointScheme for EqptPsuSlotEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptPsuSlot.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoExtCh {
                pod,
                node,
                extch,
                psuslot,
            } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/psuslot-{psuslot}.json"
            )),
            Self::MoSysExtCh { extch, psuslot } => {
                Cow::Owned(format!("mo/sys/extch-{extch}/psuslot-{psuslot}.json"))
            }
            Self::MoCh { pod, node, psuslot } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/psuslot-{psuslot}.json"
            )),
            Self::MoSysCh { psuslot } => Cow::Owned(format!("mo/sys/ch/psuslot-{psuslot}.json")),
        }
    }
}

pub type EqptPsuSlot = AciObject<__internal::EqptPsuSlot>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptPsuSlot;
    impl AciObjectScheme for EqptPsuSlot {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptPsuSlotEndpoint;
        const CLASS_NAME: &'static str = "eqptPsuSlot";
    }
}
