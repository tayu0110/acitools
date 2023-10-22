use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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
    #[serde(rename = "type")]
    r#type: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    vendor: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    FaultCounts {},
    FirmwareCompRunning {},
    HealthInst {},
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
        fpga: String,
    },
    MoSysExtchc {
        extch: String,
        extchslot: String,
        fpga: String,
    },
    MoSc {
        pod: String,
        node: String,
        scslot: String,
        fpga: String,
    },
    MoSysSc {
        scslot: String,
        fpga: String,
    },
    MoFc {
        pod: String,
        node: String,
        fcslot: String,
        fpga: String,
    },
    MoSysFc {
        fcslot: String,
        fpga: String,
    },
    MoLc {
        pod: String,
        node: String,
        lcslot: String,
        fpga: String,
    },
    MoSysLc {
        lcslot: String,
        fpga: String,
    },
    MoSup {
        pod: String,
        node: String,
        supslot: String,
        fpga: String,
    },
    MoSysSup {
        supslot: String,
        fpga: String,
    },
    MoNic {
        pod: String,
        node: String,
        nslot: String,
        nic: String,
        fpga: String,
    },
    MoSysNic {
        nslot: String,
        nic: String,
        fpga: String,
    },
    MoBoard {
        pod: String,
        node: String,
        fpga: String,
    },
    MoSysBoard {
        fpga: String,
    },
}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptFpga.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoExtchc {
                pod,
                node,
                extch,
                extchslot,
                fpga,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/extchslot-{extchslot}/extchc/fpga-{fpga}.json")),
            Self::MoSysExtchc {
                extch,
                extchslot,
                fpga,
            } => Cow::Owned(format!("mo/sys/extch-{extch}/extchslot-{extchslot}/extchc/fpga-{fpga}.json")),
            Self::MoSc {
                pod,
                node,
                scslot,
                fpga,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/scslot-{scslot}/sc/fpga-{fpga}.json")),
            Self::MoSysSc {
                scslot,
                fpga,
            } => Cow::Owned(format!("mo/sys/ch/scslot-{scslot}/sc/fpga-{fpga}.json")),
            Self::MoFc {
                pod,
                node,
                fcslot,
                fpga,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/fcslot-{fcslot}/fc/fpga-{fpga}.json")),
            Self::MoSysFc {
                fcslot,
                fpga,
            } => Cow::Owned(format!("mo/sys/ch/fcslot-{fcslot}/fc/fpga-{fpga}.json")),
            Self::MoLc {
                pod,
                node,
                lcslot,
                fpga,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/lcslot-{lcslot}/lc/fpga-{fpga}.json")),
            Self::MoSysLc {
                lcslot,
                fpga,
            } => Cow::Owned(format!("mo/sys/ch/lcslot-{lcslot}/lc/fpga-{fpga}.json")),
            Self::MoSup {
                pod,
                node,
                supslot,
                fpga,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/supslot-{supslot}/sup/fpga-{fpga}.json")),
            Self::MoSysSup {
                supslot,
                fpga,
            } => Cow::Owned(format!("mo/sys/ch/supslot-{supslot}/sup/fpga-{fpga}.json")),
            Self::MoNic {
                pod,
                node,
                nslot,
                nic,
                fpga,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/nslot-{nslot}/nic-{nic}/fpga-{fpga}.json")),
            Self::MoSysNic {
                nslot,
                nic,
                fpga,
            } => Cow::Owned(format!("mo/sys/ch/nslot-{nslot}/nic-{nic}/fpga-{fpga}.json")),
            Self::MoBoard {
                pod,
                node,
                fpga,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/bslot/board/fpga-{fpga}.json")),
            Self::MoSysBoard {
                fpga,
            } => Cow::Owned(format!("mo/sys/ch/bslot/board/fpga-{fpga}.json")),
        }
    }
}

pub type EqptFpga = AciObject<__internal::EqptFpga>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptFpga;
    impl AciObjectScheme for EqptFpga {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "eqptFpga";
    }
}
