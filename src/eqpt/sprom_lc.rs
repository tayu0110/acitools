use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::{
    sprom_common_block, sprom_lc_block, sprom_port_data, sprom_sensor_block, sprom_sensor_data,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    acc: String,
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
    err_rsn: String,
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
    num_blk: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    oper_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rev: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ser: String,
    status: ConfigStatus,
    #[serde(rename = "type", skip_serializing_if = "String::is_empty")]
    r#type: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    vendor: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    EqptSpCmnBlk(sprom_common_block::EqptSpCmnBlk),
    EqptSpPd(sprom_port_data::EqptSpPd),
    EqptSpSd(sprom_sensor_data::EqptSpSd),
    EqptSpSensorBlk(sprom_sensor_block::EqptSpSensorBlk),
    EqptSpromLcBlk(sprom_lc_block::EqptSpromLcBlk),
    FaultCounts {},
    FaultInst {},
    HealthInst {},
}

#[derive(Debug, Clone)]
pub enum EqptSpromLcEndpoint {
    ClassAll,
    MoUni,
    MoSc {
        pod: String,
        node: String,
        scslot: String,
    },
    MoSysSc {
        scslot: String,
    },
    MoFc {
        pod: String,
        node: String,
        fcslot: String,
    },
    MoSysFc {
        fcslot: String,
    },
    MoLc {
        pod: String,
        node: String,
        lcslot: String,
    },
    MoSysLc {
        lcslot: String,
    },
}

impl EndpointScheme for EqptSpromLcEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptSpromLc.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoSc { pod, node, scslot } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/scslot-{scslot}/sc/splc.json"
            )),
            Self::MoSysSc { scslot } => {
                Cow::Owned(format!("mo/sys/ch/scslot-{scslot}/sc/splc.json"))
            }
            Self::MoFc { pod, node, fcslot } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/fcslot-{fcslot}/fc/splc.json"
            )),
            Self::MoSysFc { fcslot } => {
                Cow::Owned(format!("mo/sys/ch/fcslot-{fcslot}/fc/splc.json"))
            }
            Self::MoLc { pod, node, lcslot } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/lcslot-{lcslot}/lc/splc.json"
            )),
            Self::MoSysLc { lcslot } => {
                Cow::Owned(format!("mo/sys/ch/lcslot-{lcslot}/lc/splc.json"))
            }
        }
    }
}

pub type EqptSpromLc = AciObject<__internal::EqptSpromLc>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptSpromLc;
    impl AciObjectScheme for EqptSpromLc {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptSpromLcEndpoint;
        const CLASS_NAME: &'static str = "eqptSpromLc";
    }
}
