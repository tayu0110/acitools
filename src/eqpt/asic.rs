use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::forwarding_instance;

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
    #[serde(rename = "type")]
    r#type: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    vendor: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    EqptFwdInst(forwarding_instance::EqptFwdInst),
    FaultCounts {},
    FaultInst {},
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
        asic: String,
    },
    MoSysExtchc {
        extch: String,
        extchslot: String,
        asic: String,
    },
    MoSc {
        pod: String,
        node: String,
        scslot: String,
        asic: String,
    },
    MoSysSc {
        scslot: String,
        asic: String,
    },
    MoFc {
        pod: String,
        node: String,
        fcslot: String,
        asic: String,
    },
    MoSysFc {
        fcslot: String,
        asic: String,
    },
    MoLc {
        pod: String,
        node: String,
        lcslot: String,
        asic: String,
    },
    MoSysLc {
        lcslot: String,
        asic: String,
    },
    MoSup {
        pod: String,
        node: String,
        supslot: String,
        asic: String,
    },
    MoSysSup {
        supslot: String,
        asic: String,
    },
    MoNic {
        pod: String,
        node: String,
        nslot: String,
        nic: String,
        asic: String,
    },
    MoSysNic {
        nslot: String,
        nic: String,
        asic: String,
    },
    MoBoard {
        pod: String,
        node: String,
        asic: String,
    },
    MoSysBoard {
        asic: String,
    },
}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptAsic.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoExtchc {
                pod,
                node,
                extch,
                extchslot,
                asic,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/extchslot-{extchslot}/extchc/asic-{asic}.json")),
            Self::MoSysExtchc {
                extch,
                extchslot,
                asic,
            } => Cow::Owned(format!("mo/sys/extch-{extch}/extchslot-{extchslot}/extchc/asic-{asic}.json")),
            Self::MoSc {
                pod,
                node,
                scslot,
                asic,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/scslot-{scslot}/sc/asic-{asic}.json")),
            Self::MoSysSc {
                scslot,
                asic,
            } => Cow::Owned(format!("mo/sys/ch/scslot-{scslot}/sc/asic-{asic}.json")),
            Self::MoFc {
                pod,
                node,
                fcslot,
                asic,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/fcslot-{fcslot}/fc/asic-{asic}.json")),
            Self::MoSysFc {
                fcslot,
                asic,
            } => Cow::Owned(format!("mo/sys/ch/fcslot-{fcslot}/fc/asic-{asic}.json")),
            Self::MoLc {
                pod,
                node,
                lcslot,
                asic,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/lcslot-{lcslot}/lc/asic-{asic}.json")),
            Self::MoSysLc {
                lcslot,
                asic,
            } => Cow::Owned(format!("mo/sys/ch/lcslot-{lcslot}/lc/asic-{asic}.json")),
            Self::MoSup {
                pod,
                node,
                supslot,
                asic,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/supslot-{supslot}/sup/asic-{asic}.json")),
            Self::MoSysSup {
                supslot,
                asic,
            } => Cow::Owned(format!("mo/sys/ch/supslot-{supslot}/sup/asic-{asic}.json")),
            Self::MoNic {
                pod,
                node,
                nslot,
                nic,
                asic,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/nslot-{nslot}/nic-{nic}/asic-{asic}.json")),
            Self::MoSysNic {
                nslot,
                nic,
                asic,
            } => Cow::Owned(format!("mo/sys/ch/nslot-{nslot}/nic-{nic}/asic-{asic}.json")),
            Self::MoBoard {
                pod,
                node,
                asic,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/bslot/board/asic-{asic}.json")),
            Self::MoSysBoard {
                asic,
            } => Cow::Owned(format!("mo/sys/ch/bslot/board/asic-{asic}.json")),
        }
    }
}

pub type EqptAsic = AciObject<__internal::EqptAsic>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptAsic;
    impl AciObjectScheme for EqptAsic {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "eqptAsic";
    }
}
