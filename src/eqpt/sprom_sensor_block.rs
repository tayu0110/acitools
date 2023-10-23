use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::sprom_sensor_data;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    cksum: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    len: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    sig: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    ver: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    EqptSpSd(sprom_sensor_data::EqptSpSd),
}

#[derive(Debug, Clone)]
pub enum EqptSpSensorBlkEndpoint {
    ClassAll,
    MoUni,
    MoScSplc {
        pod: String,
        node: String,
        scslot: String,
    },
    MoSysScSplc {
        scslot: String,
    },
    MoFcSplc {
        pod: String,
        node: String,
        fcslot: String,
    },
    MoSysFcSplc {
        fcslot: String,
    },
    MoLcSplc {
        pod: String,
        node: String,
        lcslot: String,
    },
    MoSysLcSplc {
        lcslot: String,
    },
    MoExtChSpsup {
        pod: String,
        node: String,
        extch: String,
    },
    MoSysExtChSpsup {
        extch: String,
    },
    MoSpsup {
        pod: String,
        node: String,
        supslot: String,
    },
    MoSysSpsup {
        supslot: String,
    },
}

impl EndpointScheme for EqptSpSensorBlkEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptSpSensorBlk.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoScSplc {
                pod,
                node,
                scslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/scslot-{scslot}/sc/splc/spsensorblk.json")),
            Self::MoSysScSplc {
                scslot,
            } => Cow::Owned(format!("mo/sys/ch/scslot-{scslot}/sc/splc/spsensorblk.json")),
            Self::MoFcSplc {
                pod,
                node,
                fcslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/fcslot-{fcslot}/fc/splc/spsensorblk.json")),
            Self::MoSysFcSplc {
                fcslot,
            } => Cow::Owned(format!("mo/sys/ch/fcslot-{fcslot}/fc/splc/spsensorblk.json")),
            Self::MoLcSplc {
                pod,
                node,
                lcslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/lcslot-{lcslot}/lc/splc/spsensorblk.json")),
            Self::MoSysLcSplc {
                lcslot,
            } => Cow::Owned(format!("mo/sys/ch/lcslot-{lcslot}/lc/splc/spsensorblk.json")),
            Self::MoExtChSpsup {
                pod,
                node,
                extch,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/spsup/spsensorblk.json")),
            Self::MoSysExtChSpsup {
                extch,
            } => Cow::Owned(format!("mo/sys/extch-{extch}/spsup/spsensorblk.json")),
            Self::MoSpsup {
                pod,
                node,
                supslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/supslot-{supslot}/sup/spsup/spsensorblk.json")),
            Self::MoSysSpsup {
                supslot,
            } => Cow::Owned(format!("mo/sys/ch/supslot-{supslot}/sup/spsup/spsensorblk.json")),
        }
    }
}

pub type EqptSpSensorBlk = AciObject<__internal::EqptSpSensorBlk>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptSpSensorBlk;
    impl AciObjectScheme for EqptSpSensorBlk {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptSpSensorBlkEndpoint;
        const CLASS_NAME: &'static str = "eqptSpSensorBlk";
    }
}
