use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::{sprom_common_block, sprom_psu_block};

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
    #[serde(skip_serializing_if = "String::is_empty")]
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
    EqptSpromPsuBlk(sprom_psu_block::EqptSpromPsuBlk),
    FaultCounts {},
    FaultInst {},
    HealthInst {},
}

#[derive(Debug, Clone)]
pub enum EqptSpromPsuEndpoint {
    ClassAll,
    MoUni,
    MoExtChPsu {
        pod: String,
        node: String,
        extch: String,
        psuslot: String,
    },
    MoSysExtChPsu {
        extch: String,
        psuslot: String,
    },
    MoPsu {
        pod: String,
        node: String,
        psuslot: String,
    },
    MoSysPsu {
        psuslot: String,
    },
}

impl EndpointScheme for EqptSpromPsuEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptSpromPsu.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoExtChPsu {
                pod,
                node,
                extch,
                psuslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/psuslot-{psuslot}/psu/sppsu.json")),
            Self::MoSysExtChPsu {
                extch,
                psuslot,
            } => Cow::Owned(format!("mo/sys/extch-{extch}/psuslot-{psuslot}/psu/sppsu.json")),
            Self::MoPsu {
                pod,
                node,
                psuslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/psuslot-{psuslot}/psu/sppsu.json")),
            Self::MoSysPsu {
                psuslot,
            } => Cow::Owned(format!("mo/sys/ch/psuslot-{psuslot}/psu/sppsu.json")),
        }
    }
}

pub type EqptSpromPsu = AciObject<__internal::EqptSpromPsu>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptSpromPsu;
    impl AciObjectScheme for EqptSpromPsu {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptSpromPsuEndpoint;
        const CLASS_NAME: &'static str = "eqptSpromPsu";
    }
}
