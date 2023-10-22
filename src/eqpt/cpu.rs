use super::core;
use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    arch: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    cimc_version: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    cores: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    cores_en: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    descr: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mfg_tm: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    model: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rev: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ser: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    sock: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    speed: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    thrds: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    vendor: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    EqptCore(core::EqptCore),
}

#[derive(Debug, Clone)]
pub enum Endpoint {
    ClassAll,
    MoUni,
    MoExtchc {
        pod: String,
        node: String,
        extch: String,
        extchslot: String,
        cpu: String,
    },
    MoSysExtchc {
        extch: String,
        extchslot: String,
        cpu: String,
    },
    MoSc {
        pod: String,
        node: String,
        scslot: String,
        cpu: String,
    },
    MoSysSc {
        scslot: String,
        cpu: String,
    },
    MoFc {
        pod: String,
        node: String,
        fcslot: String,
        cpu: String,
    },
    MoSysFc {
        fcslot: String,
        cpu: String,
    },
    MoLc {
        pod: String,
        node: String,
        lcslot: String,
        cpu: String,
    },
    MoSysLc {
        lcslot: String,
        cpu: String,
    },
    MoSup {
        pod: String,
        node: String,
        supslot: String,
        cpu: String,
    },
    MoSysSup {
        supslot: String,
        cpu: String,
    },
    MoNic {
        pod: String,
        node: String,
        nslot: String,
        nic: String,
        cpu: String,
    },
    MoSysNic {
        nslot: String,
        nic: String,
        cpu: String,
    },
    MoBoard {
        pod: String,
        node: String,
        cpu: String,
    },
    MoSysBoard {
        cpu: String,
    },
}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptCPU.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoExtchc {
                pod,
                node,
                extch,
                extchslot,
                cpu,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/extchslot-{extchslot}/extchc/cpu-{cpu}.json")),
            Self::MoSysExtchc {
                extch,
                extchslot,
                cpu,
            } => Cow::Owned(format!("mo/sys/extch-{extch}/extchslot-{extchslot}/extchc/cpu-{cpu}.json")),
            Self::MoSc {
                pod,
                node,
                scslot,
                cpu,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/scslot-{scslot}/sc/cpu-{cpu}.json")),
            Self::MoSysSc {
                scslot,
                cpu,
            } => Cow::Owned(format!("mo/sys/ch/scslot-{scslot}/sc/cpu-{cpu}.json")),
            Self::MoFc {
                pod,
                node,
                fcslot,
                cpu,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/fcslot-{fcslot}/fc/cpu-{cpu}.json")),
            Self::MoSysFc {
                fcslot,
                cpu,
            } => Cow::Owned(format!("mo/sys/ch/fcslot-{fcslot}/fc/cpu-{cpu}.json")),
            Self::MoLc {
                pod,
                node,
                lcslot,
                cpu,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/lcslot-{lcslot}/lc/cpu-{cpu}.json")),
            Self::MoSysLc {
                lcslot,
                cpu,
            } => Cow::Owned(format!("mo/sys/ch/lcslot-{lcslot}/lc/cpu-{cpu}.json")),
            Self::MoSup {
                pod,
                node,
                supslot,
                cpu,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/supslot-{supslot}/sup/cpu-{cpu}.json")),
            Self::MoSysSup {
                supslot,
                cpu,
            } => Cow::Owned(format!("mo/sys/ch/supslot-{supslot}/sup/cpu-{cpu}.json")),
            Self::MoNic {
                pod,
                node,
                nslot,
                nic,
                cpu,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/nslot-{nslot}/nic-{nic}/cpu-{cpu}.json")),
            Self::MoSysNic {
                nslot,
                nic,
                cpu,
            } => Cow::Owned(format!("mo/sys/ch/nslot-{nslot}/nic-{nic}/cpu-{cpu}.json")),
            Self::MoBoard {
                pod,
                node,
                cpu,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/bslot/board/cpu-{cpu}.json")),
            Self::MoSysBoard {
                cpu,
            } => Cow::Owned(format!("mo/sys/ch/bslot/board/cpu-{cpu}.json")),
        }
    }
}

pub type EqptCpu = AciObject<__internal::EqptCpu>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptCpu;
    impl AciObjectScheme for EqptCpu {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "eqptCPU";
    }
}
