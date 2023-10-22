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
pub enum EqptEpcPEndpoint {
    ClassAll,
    MoUni,
    MoSc {
        pod: String,
        node: String,
        scslot: String,
        epc: String,
    },
    MoSysSc {
        scslot: String,
        epc: String,
    },
    MoFc {
        pod: String,
        node: String,
        fcslot: String,
        epc: String,
    },
    MoSysFc {
        fcslot: String,
        epc: String,
    },
    MoSup {
        pod: String,
        node: String,
        supslot: String,
        epc: String,
    },
    MoSysSup {
        supslot: String,
        epc: String,
    },
}

impl EndpointScheme for EqptEpcPEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptEpcP.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoSc {
                pod,
                node,
                scslot,
                epc,
            } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/scslot-{scslot}/sc/epc-{epc}.json"
            )),
            Self::MoSysSc { scslot, epc } => {
                Cow::Owned(format!("mo/sys/ch/scslot-{scslot}/sc/epc-{epc}.json"))
            }
            Self::MoFc {
                pod,
                node,
                fcslot,
                epc,
            } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/fcslot-{fcslot}/fc/epc-{epc}.json"
            )),
            Self::MoSysFc { fcslot, epc } => {
                Cow::Owned(format!("mo/sys/ch/fcslot-{fcslot}/fc/epc-{epc}.json"))
            }
            Self::MoSup {
                pod,
                node,
                supslot,
                epc,
            } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/supslot-{supslot}/sup/epc-{epc}.json"
            )),
            Self::MoSysSup { supslot, epc } => {
                Cow::Owned(format!("mo/sys/ch/supslot-{supslot}/sup/epc-{epc}.json"))
            }
        }
    }
}

pub type EqptEpcP = AciObject<__internal::EqptEpcP>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptEpcP;
    impl AciObjectScheme for EqptEpcP {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptEpcPEndpoint;
        const CLASS_NAME: &'static str = "eqptEpcP";
    }
}
