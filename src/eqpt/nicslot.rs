use super::nic;
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
    lc_own: String,
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
    EqptNic(nic::EqptNic),
    EqptRtOosSlot {},
    FaultCounts {},
    FaultInst {},
    HealthInst {},
}

#[derive(Debug, Clone)]
pub enum EqptNSlotEndpoint {
    ClassAll,
    MoUni,
    MoCh {
        pod: String,
        node: String,
        nslot: String,
    },
    MoSysCh {
        nslot: String,
    },
}

impl EndpointScheme for EqptNSlotEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptNSlot.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoCh { pod, node, nslot } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/nslot-{nslot}.json"
            )),
            Self::MoSysCh { nslot } => Cow::Owned(format!("mo/sys/ch/nslot-{nslot}.json")),
        }
    }
}

pub type EqptNSlot = AciObject<__internal::EqptNSlot>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptNSlot;
    impl AciObjectScheme for EqptNSlot {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptNSlotEndpoint;
        const CLASS_NAME: &'static str = "eqptNSlot";
    }
}
