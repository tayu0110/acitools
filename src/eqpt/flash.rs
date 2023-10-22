use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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
    deltape: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    descr: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    gbb: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lba: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lifetime: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    major_alarm: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mfg_tm: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    minor_alarm: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    model: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mon_pol_dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    oper_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pe_cycles: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    read_err: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rev: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ser: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    tbw: String,
    #[serde(rename = "type", skip_serializing_if = "String::is_empty")]
    r#type: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    vendor: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    warning: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    wlc: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    EqptSsdStatsSOrder {},
    EqptSsdStatsWOrder {},
    FaultCounts {},
    FaultInst {},
    HealthInst {},
}

#[derive(Debug, Clone)]
pub enum EqptFlashEndpoint {
    ClassAll,
    MoUni,
    MoExtchc {
        pod: String,
        node: String,
        extch: String,
        extchslot: String,
    },
    MoSysExtchc {
        extch: String,
        extchslot: String,
    },
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
    MoSup {
        pod: String,
        node: String,
        supslot: String,
    },
    MoSysSup {
        supslot: String,
    },
    MoNic {
        pod: String,
        node: String,
        nslot: String,
        nic: String,
    },
    MoSysNic {
        nslot: String,
        nic: String,
    },
    MoBoard {
        pod: String,
        node: String,
    },
    MoSysBoard {},
}

impl EndpointScheme for EqptFlashEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptFlash.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoExtchc {
                pod,
                node,
                extch,
                extchslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/extchslot-{extchslot}/extchc/flash.json")),
            Self::MoSysExtchc {
                extch,
                extchslot,
            } => Cow::Owned(format!("mo/sys/extch-{extch}/extchslot-{extchslot}/extchc/flash.json")),
            Self::MoSc {
                pod,
                node,
                scslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/scslot-{scslot}/sc/flash.json")),
            Self::MoSysSc {
                scslot,
            } => Cow::Owned(format!("mo/sys/ch/scslot-{scslot}/sc/flash.json")),
            Self::MoFc {
                pod,
                node,
                fcslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/fcslot-{fcslot}/fc/flash.json")),
            Self::MoSysFc {
                fcslot,
            } => Cow::Owned(format!("mo/sys/ch/fcslot-{fcslot}/fc/flash.json")),
            Self::MoLc {
                pod,
                node,
                lcslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/lcslot-{lcslot}/lc/flash.json")),
            Self::MoSysLc {
                lcslot,
            } => Cow::Owned(format!("mo/sys/ch/lcslot-{lcslot}/lc/flash.json")),
            Self::MoSup {
                pod,
                node,
                supslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/supslot-{supslot}/sup/flash.json")),
            Self::MoSysSup {
                supslot,
            } => Cow::Owned(format!("mo/sys/ch/supslot-{supslot}/sup/flash.json")),
            Self::MoNic {
                pod,
                node,
                nslot,
                nic,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/nslot-{nslot}/nic-{nic}/flash.json")),
            Self::MoSysNic {
                nslot,
                nic,
            } => Cow::Owned(format!("mo/sys/ch/nslot-{nslot}/nic-{nic}/flash.json")),
            Self::MoBoard {
                pod,
                node,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/bslot/board/flash.json")),
            Self::MoSysBoard {
            } => Cow::Owned(format!("mo/sys/ch/bslot/board/flash.json")),
        }
    }
}

pub type EqptFlash = AciObject<__internal::EqptFlash>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptFlash;
    impl AciObjectScheme for EqptFlash {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptFlashEndpoint;
        const CLASS_NAME: &'static str = "eqptFlash";
    }
}
