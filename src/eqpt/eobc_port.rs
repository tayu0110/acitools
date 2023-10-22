use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::port;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    descr: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    status: ConfigStatus,
    #[serde(rename = "type", skip_serializing_if = "String::is_empty")]
    r#type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    EqptLPort(port::EqptLPort),
}

#[derive(Debug, Clone)]
pub enum EqptEobcPEndpoint {
    ClassAll,
    MoUni,
    MoSc {
        pod: String,
        node: String,
        scslot: String,
        eobc: String,
    },
    MoSysSc {
        scslot: String,
        eobc: String,
    },
    MoFc {
        pod: String,
        node: String,
        fcslot: String,
        eobc: String,
    },
    MoSysFc {
        fcslot: String,
        eobc: String,
    },
    MoLc {
        pod: String,
        node: String,
        lcslot: String,
        eobc: String,
    },
    MoSysLc {
        lcslot: String,
        eobc: String,
    },
    MoSup {
        pod: String,
        node: String,
        supslot: String,
        eobc: String,
    },
    MoSysSup {
        supslot: String,
        eobc: String,
    },
}

impl EndpointScheme for EqptEobcPEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptEobcP.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoSc {
                pod,
                node,
                scslot,
                eobc,
            } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/scslot-{scslot}/sc/eobc-{eobc}.json"
            )),
            Self::MoSysSc { scslot, eobc } => {
                Cow::Owned(format!("mo/sys/ch/scslot-{scslot}/sc/eobc-{eobc}.json"))
            }
            Self::MoFc {
                pod,
                node,
                fcslot,
                eobc,
            } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/fcslot-{fcslot}/fc/eobc-{eobc}.json"
            )),
            Self::MoSysFc { fcslot, eobc } => {
                Cow::Owned(format!("mo/sys/ch/fcslot-{fcslot}/fc/eobc-{eobc}.json"))
            }
            Self::MoLc {
                pod,
                node,
                lcslot,
                eobc,
            } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/lcslot-{lcslot}/lc/eobc-{eobc}.json"
            )),
            Self::MoSysLc { lcslot, eobc } => {
                Cow::Owned(format!("mo/sys/ch/lcslot-{lcslot}/lc/eobc-{eobc}.json"))
            }
            Self::MoSup {
                pod,
                node,
                supslot,
                eobc,
            } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/supslot-{supslot}/sup/eobc-{eobc}.json"
            )),
            Self::MoSysSup { supslot, eobc } => {
                Cow::Owned(format!("mo/sys/ch/supslot-{supslot}/sup/eobc-{eobc}.json"))
            }
        }
    }
}

pub type EqptEobcP = AciObject<__internal::EqptEobcP>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptEobcP;
    impl AciObjectScheme for EqptEobcP {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptEobcPEndpoint;
        const CLASS_NAME: &'static str = "eqptEobcP";
    }
}
