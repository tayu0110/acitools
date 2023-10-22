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
    oper_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rev: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ser: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    vendor: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {}

#[derive(Debug, Clone)]
pub enum EqptEjecEndpoint {
    ClassAll,
    MoUni,
    MoExtChFt {
        pod: String,
        node: String,
        extch: String,
        ftslot: String,
        ej: String,
    },
    MoSysExtChFt {
        extch: String,
        ftslot: String,
        ej: String,
    },
    MoFt {
        pod: String,
        node: String,
        ftslot: String,
        ej: String,
    },
    MoSysFt {
        ftslot: String,
        ej: String,
    },
    MoFc {
        pod: String,
        node: String,
        fcslot: String,
        ej: String,
    },
    MoSysFc {
        fcslot: String,
        ej: String,
    },
    MoLc {
        pod: String,
        node: String,
        lcslot: String,
        ej: String,
    },
    MoSysLc {
        lcslot: String,
        ej: String,
    },
    MoSup {
        pod: String,
        node: String,
        supslot: String,
        ej: String,
    },
    MoSysSup {
        supslot: String,
        ej: String,
    },
}

impl EndpointScheme for EqptEjecEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptEjec.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoExtChFt {
                pod,
                node,
                extch,
                ftslot,
                ej,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/ftslot-{ftslot}/ft/ej-{ej}.json")),
            Self::MoSysExtChFt {
                extch,
                ftslot,
                ej,
            } => Cow::Owned(format!("mo/sys/extch-{extch}/ftslot-{ftslot}/ft/ej-{ej}.json")),
            Self::MoFt {
                pod,
                node,
                ftslot,
                ej,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/ftslot-{ftslot}/ft/ej-{ej}.json")),
            Self::MoSysFt {
                ftslot,
                ej,
            } => Cow::Owned(format!("mo/sys/ch/ftslot-{ftslot}/ft/ej-{ej}.json")),
            Self::MoFc {
                pod,
                node,
                fcslot,
                ej,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/fcslot-{fcslot}/fc/ej-{ej}.json")),
            Self::MoSysFc {
                fcslot,
                ej,
            } => Cow::Owned(format!("mo/sys/ch/fcslot-{fcslot}/fc/ej-{ej}.json")),
            Self::MoLc {
                pod,
                node,
                lcslot,
                ej,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/lcslot-{lcslot}/lc/ej-{ej}.json")),
            Self::MoSysLc {
                lcslot,
                ej,
            } => Cow::Owned(format!("mo/sys/ch/lcslot-{lcslot}/lc/ej-{ej}.json")),
            Self::MoSup {
                pod,
                node,
                supslot,
                ej,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/supslot-{supslot}/sup/ej-{ej}.json")),
            Self::MoSysSup {
                supslot,
                ej,
            } => Cow::Owned(format!("mo/sys/ch/supslot-{supslot}/sup/ej-{ej}.json")),
        }
    }
}

pub type EqptEjec = AciObject<__internal::EqptEjec>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptEjec;
    impl AciObjectScheme for EqptEjec {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptEjecEndpoint;
        const CLASS_NAME: &'static str = "eqptEjec";
    }
}
