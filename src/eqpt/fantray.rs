use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::{ejector, fan, ind_led, loc_led, sprom_fan};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    cimc_version: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    descr: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    fan_name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    fanlet_fail_string: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    hw_ver: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mfg_tm: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    model: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mon_pol_dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    oper_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rev: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ser: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    vendor: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    EqptEjec(ejector::EqptEjec),
    EqptFan(fan::EqptFan),
    EqptFruPower15Min {},
    EqptFruPower1D {},
    EqptFruPower1H {},
    EqptFruPower1Mo {},
    EqptFruPower1Qtr {},
    EqptFruPower1W {},
    EqptFruPower1Year {},
    EqptFruPower5Min {},
    EqptFruPowerHist15Min {},
    EqptFruPowerHist1D {},
    EqptFruPowerHist1H {},
    EqptFruPowerHist1Mo {},
    EqptFruPowerHist1Qtr {},
    EqptFruPowerHist1W {},
    EqptFruPowerHist1Year {},
    EqptFruPowerHist5Min {},
    EqptIndLed(ind_led::EqptIndLed),
    EqptLocLed(loc_led::EqptLocLed),
    EqptSpromFan(sprom_fan::EqptSpromFan),
    FaultCounts {},
    FaultInst {},
    HealthInst {},
}

#[derive(Debug, Clone)]
pub enum EqptFtEndpoint {
    ClassAll,
    MoUni,
    MoExtChFtslot {
        pod: String,
        node: String,
        extch: String,
        ftslot: String,
    },
    MoSysExtChFtslot {
        extch: String,
        ftslot: String,
    },
    MoFtslot {
        pod: String,
        node: String,
        ftslot: String,
    },
    MoSysFtslot {
        ftslot: String,
    },
}

impl EndpointScheme for EqptFtEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptFt.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoExtChFtslot {
                pod,
                node,
                extch,
                ftslot,
            } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/ftslot-{ftslot}/ft.json"
            )),
            Self::MoSysExtChFtslot { extch, ftslot } => {
                Cow::Owned(format!("mo/sys/extch-{extch}/ftslot-{ftslot}/ft.json"))
            }
            Self::MoFtslot { pod, node, ftslot } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/ftslot-{ftslot}/ft.json"
            )),
            Self::MoSysFtslot { ftslot } => {
                Cow::Owned(format!("mo/sys/ch/ftslot-{ftslot}/ft.json"))
            }
        }
    }
}

pub type EqptFt = AciObject<__internal::EqptFt>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptFt;
    impl AciObjectScheme for EqptFt {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptFtEndpoint;
        const CLASS_NAME: &'static str = "eqptFt";
    }
}
