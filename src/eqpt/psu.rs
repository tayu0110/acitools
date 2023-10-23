use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::{fan, ind_led, sprom_psu};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    alm_reg: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    cap: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    cimc_version: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    descr: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    drawn_curr: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    fan_op_st: String,
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
    #[allow(dead_code)]
    #[serde(skip_serializing, default)]
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
    tc: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    v_id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    v_src: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    vendor: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    volt: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    EqptFan(fan::EqptFan),
    EqptIndLed(ind_led::EqptIndLed),
    EqptPsPower15Min {},
    EqptPsPower1D {},
    EqptPsPower1H {},
    EqptPsPower1Mo {},
    EqptPsPower1Qtr {},
    EqptPsPower1W {},
    EqptPsPower1Year {},
    EqptPsPower5Min {},
    EqptPsPowerHist15Min {},
    EqptPsPowerHist1D {},
    EqptPsPowerHist1H {},
    EqptPsPowerHist1Mo {},
    EqptPsPowerHist1Qtr {},
    EqptPsPowerHist1W {},
    EqptPsPowerHist1Year {},
    EqptPsPowerHist5Min {},
    EqptSpromPsu(sprom_psu::EqptSpromPsu),
    FaultCounts {},
    FaultInst {},
    HealthInst {},
}

#[derive(Debug, Clone)]
pub enum EqptPsuEndpoint {
    ClassAll,
    MoUni,
    MoExtChPsuslot {
        pod: String,
        node: String,
        extch: String,
        psuslot: String,
    },
    MoSysExtChPsuslot {
        extch: String,
        psuslot: String,
    },
    MoPsuslot {
        pod: String,
        node: String,
        psuslot: String,
    },
    MoSysPsuslot {
        psuslot: String,
    },
}

impl EndpointScheme for EqptPsuEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptPsu.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoExtChPsuslot {
                pod,
                node,
                extch,
                psuslot,
            } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/psuslot-{psuslot}/psu.json"
            )),
            Self::MoSysExtChPsuslot { extch, psuslot } => {
                Cow::Owned(format!("mo/sys/extch-{extch}/psuslot-{psuslot}/psu.json"))
            }
            Self::MoPsuslot { pod, node, psuslot } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/psuslot-{psuslot}/psu.json"
            )),
            Self::MoSysPsuslot { psuslot } => {
                Cow::Owned(format!("mo/sys/ch/psuslot-{psuslot}/psu.json"))
            }
        }
    }
}

pub type EqptPsu = AciObject<__internal::EqptPsu>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptPsu;
    impl AciObjectScheme for EqptPsu {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptPsuEndpoint;
        const CLASS_NAME: &'static str = "eqptPsu";
    }
}
