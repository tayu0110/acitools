use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::{
    backplane_sprom_block, backplane_sprom_license, backplane_sprom_sn, backplane_sprom_wwn,
    sprom_common_block,
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
    EqptBpSpLic(backplane_sprom_license::EqptBpSpLic),
    EqptBpSpSsn(backplane_sprom_sn::EqptBpSpSsn),
    EqptBpSpWwn(backplane_sprom_wwn::EqptBpSpWwn),
    EqptSpCmnBlk(sprom_common_block::EqptSpCmnBlk),
    EqptSpromBPBlk(backplane_sprom_block::EqptSpromBpBlk),
    FaultCounts {},
    FaultInst {},
    HealthInst {},
}

#[derive(Debug, Clone)]
pub enum EqptSpromBpEndpoint {
    ClassAll,
    MoUni,
    MoExtch {
        pod: String,
        node: String,
        extch: String,
    },
    MoSysExtch {
        extch: String,
    },
    MoCh {
        pod: String,
        node: String,
    },
    MoSysCh,
}

impl EndpointScheme for EqptSpromBpEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptSpromBP.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoExtch { pod, node, extch } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/spbp.json"
            )),
            Self::MoSysExtch { extch } => Cow::Owned(format!("mo/sys/extch-{extch}/spbp.json")),
            Self::MoCh { pod, node } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/spbp.json"
            )),
            Self::MoSysCh => Cow::Borrowed("mo/sys/ch/spbp.json"),
        }
    }
}

pub type EqptSpromBp = AciObject<__internal::EqptSpromBp>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptSpromBp;
    impl AciObjectScheme for EqptSpromBp {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptSpromBpEndpoint;
        const CLASS_NAME: &'static str = "eqptSpromBP";
    }
}
