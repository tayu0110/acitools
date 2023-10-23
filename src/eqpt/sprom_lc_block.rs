use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::{sprom_port_data, sprom_sensor_data};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    amb_t: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    cksum: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    cool_rq: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    crd_idx: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    eobc_n: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    epld_n: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    epld_v: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    fbits: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    hw_c_bits: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    len: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mac_b: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mac_l: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    max_c_pwr: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    sig: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    sram_sz: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    ver: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    EqptSpPd(sprom_port_data::EqptSpPd),
    EqptSpSd(sprom_sensor_data::EqptSpSd),
}

#[derive(Debug, Clone)]
pub enum EqptSpromLcBlkEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoScSplc {
        pod: String,
        node: String,
        scslot: String,
    },
    MoSysScSplc {
        scslot: String,
    },
    MoFcSplc {
        pod: String,
        node: String,
        fcslot: String,
    },
    MoSysFcSplc {
        fcslot: String,
    },
    MoLcSplc {
        pod: String,
        node: String,
        lcslot: String,
    },
    MoSysLcSplc {
        lcslot: String,
    },
}

impl EndpointScheme for EqptSpromLcBlkEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptSpromLcBlk.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoScSplc { pod, node, scslot } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/scslot-{scslot}/sc/splc/splcblk.json"
            )),
            Self::MoSysScSplc { scslot } => {
                Cow::Owned(format!("mo/sys/ch/scslot-{scslot}/sc/splc/splcblk.json"))
            }
            Self::MoFcSplc { pod, node, fcslot } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/fcslot-{fcslot}/fc/splc/splcblk.json"
            )),
            Self::MoSysFcSplc { fcslot } => {
                Cow::Owned(format!("mo/sys/ch/fcslot-{fcslot}/fc/splc/splcblk.json"))
            }
            Self::MoLcSplc { pod, node, lcslot } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/lcslot-{lcslot}/lc/splc/splcblk.json"
            )),
            Self::MoSysLcSplc { lcslot } => {
                Cow::Owned(format!("mo/sys/ch/lcslot-{lcslot}/lc/splc/splcblk.json"))
            }
        }
    }
}

pub type EqptSpromLcBlk = AciObject<__internal::EqptSpromLcBlk>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptSpromLcBlk;
    impl AciObjectScheme for EqptSpromLcBlk {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptSpromLcBlkEndpoint;
        const CLASS_NAME: &'static str = "eqptSpromLcBlk";
    }
}
