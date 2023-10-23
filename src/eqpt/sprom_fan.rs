use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::{sprom_common_block, sprom_fan_block, sprom_fan_sn};

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
    EqptSpromFanBlk(sprom_fan_block::EqptSpromFanBlk),
    EqptSpromFanSN(sprom_fan_sn::EqptSpromFanSn),
    FaultCounts {},
    FaultInst {},
    HealthInst {},
}

#[derive(Debug, Clone)]
pub enum EqptSpromFanEndpoint {
    ClassAll,
    MoUni,
    MoExtChFt {
        pod: String,
        node: String,
        extch: String,
        ftslot: String,
    },
    MoSysExtChFt {
        extch: String,
        ftslot: String,
    },
    MoFt {
        pod: String,
        node: String,
        ftslot: String,
    },
    MoSysFt {
        ftslot: String,
    },
}

impl EndpointScheme for EqptSpromFanEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptSpromFan.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoExtChFt {
                pod,
                node,
                extch,
                ftslot,
            } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/ftslot-{ftslot}/ft/spfan.json"
            )),
            Self::MoSysExtChFt { extch, ftslot } => Cow::Owned(format!(
                "mo/sys/extch-{extch}/ftslot-{ftslot}/ft/spfan.json"
            )),
            Self::MoFt { pod, node, ftslot } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/ftslot-{ftslot}/ft/spfan.json"
            )),
            Self::MoSysFt { ftslot } => {
                Cow::Owned(format!("mo/sys/ch/ftslot-{ftslot}/ft/spfan.json"))
            }
        }
    }
}

pub type EqptSpromFan = AciObject<__internal::EqptSpromFan>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptSpromFan;
    impl AciObjectScheme for EqptSpromFan {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptSpromFanEndpoint;
        const CLASS_NAME: &'static str = "eqptSpromFan";
    }
}
