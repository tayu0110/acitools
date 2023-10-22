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
pub enum ChildItem {}

#[derive(Debug, Clone)]
pub enum Endpoint {
    ClassAll,
    MoUni,
    MoExtchc {
        pod: String,
        node: String,
        extch: String,
        extchslot: String,
        dimm: String,
    },
    MoSysExtchc {
        extch: String,
        extchslot: String,
        dimm: String,
    },
    MoSc {
        pod: String,
        node: String,
        scslot: String,
        dimm: String,
    },
    MoSysSc {
        scslot: String,
        dimm: String,
    },
    MoFc {
        pod: String,
        node: String,
        fcslot: String,
        dimm: String,
    },
    MoSysFc {
        fcslot: String,
        dimm: String,
    },
    MoLc {
        pod: String,
        node: String,
        lcslot: String,
        dimm: String,
    },
    MoSysLc {
        lcslot: String,
        dimm: String,
    },
    MoSup {
        pod: String,
        node: String,
        supslot: String,
        dimm: String,
    },
    MoSysSup {
        supslot: String,
        dimm: String,
    },
    MoNic {
        pod: String,
        node: String,
        nslot: String,
        nic: String,
        dimm: String,
    },
    MoSysNic {
        nslot: String,
        nic: String,
        dimm: String,
    },
    MoBoard {
        pod: String,
        node: String,
        dimm: String,
    },
    MoSysBoard {
        dimm: String,
    },
}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptDimm.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoExtchc {
                pod,
                node,
                extch,
                extchslot,
                dimm,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/extchslot-{extchslot}/extchc/dimm-{dimm}.json")),
            Self::MoSysExtchc {
                extch,
                extchslot,
                dimm,
            } => Cow::Owned(format!("mo/sys/extch-{extch}/extchslot-{extchslot}/extchc/dimm-{dimm}.json")),
            Self::MoSc {
                pod,
                node,
                scslot,
                dimm,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/scslot-{scslot}/sc/dimm-{dimm}.json")),
            Self::MoSysSc {
                scslot,
                dimm,
            } => Cow::Owned(format!("mo/sys/ch/scslot-{scslot}/sc/dimm-{dimm}.json")),
            Self::MoFc {
                pod,
                node,
                fcslot,
                dimm,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/fcslot-{fcslot}/fc/dimm-{dimm}.json")),
            Self::MoSysFc {
                fcslot,
                dimm,
            } => Cow::Owned(format!("mo/sys/ch/fcslot-{fcslot}/fc/dimm-{dimm}.json")),
            Self::MoLc {
                pod,
                node,
                lcslot,
                dimm,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/lcslot-{lcslot}/lc/dimm-{dimm}.json")),
            Self::MoSysLc {
                lcslot,
                dimm,
            } => Cow::Owned(format!("mo/sys/ch/lcslot-{lcslot}/lc/dimm-{dimm}.json")),
            Self::MoSup {
                pod,
                node,
                supslot,
                dimm,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/supslot-{supslot}/sup/dimm-{dimm}.json")),
            Self::MoSysSup {
                supslot,
                dimm,
            } => Cow::Owned(format!("mo/sys/ch/supslot-{supslot}/sup/dimm-{dimm}.json")),
            Self::MoNic {
                pod,
                node,
                nslot,
                nic,
                dimm,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/nslot-{nslot}/nic-{nic}/dimm-{dimm}.json")),
            Self::MoSysNic {
                nslot,
                nic,
                dimm,
            } => Cow::Owned(format!("mo/sys/ch/nslot-{nslot}/nic-{nic}/dimm-{dimm}.json")),
            Self::MoBoard {
                pod,
                node,
                dimm,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/bslot/board/dimm-{dimm}.json")),
            Self::MoSysBoard {
                dimm,
            } => Cow::Owned(format!("mo/sys/ch/bslot/board/dimm-{dimm}.json")),
        }
    }
}

pub type EqptDimm = AciObject<__internal::EqptDimm>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptDimm;
    impl AciObjectScheme for EqptDimm {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "eqptDimm";
    }
}
