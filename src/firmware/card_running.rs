use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    bios_upg_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    bios_ver: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    descr: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    expected_ver: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    interim_ver: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    internal_label: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mode: String,
    #[allow(dead_code)]
    #[serde(skip_serializing, default)]
    mon_pol_dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    oper_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    sr_fw_flash_rec_ver: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    sr_fw_flash_ver: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    sr_fw_image_ver: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    sr_fw_running_src: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    sr_fw_running_ver: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    ts: String,
    #[serde(rename = "type", skip_serializing_if = "String::is_empty")]
    r#type: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    version: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    FaultCounts {},
    FaultInst {},
    HealthInst {},
}

#[derive(Debug, Clone)]
pub enum FirmwareCardRunningEndpoint {
    ClassAll,
    MoUni,
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
}

impl EndpointScheme for FirmwareCardRunningEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/firmwareCardRunning.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoSc { pod, node, scslot } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/scslot-{scslot}/sc/running.json"
            )),
            Self::MoSysSc { scslot } => {
                Cow::Owned(format!("mo/sys/ch/scslot-{scslot}/sc/running.json"))
            }
            Self::MoFc { pod, node, fcslot } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/fcslot-{fcslot}/fc/running.json"
            )),
            Self::MoSysFc { fcslot } => {
                Cow::Owned(format!("mo/sys/ch/fcslot-{fcslot}/fc/running.json"))
            }
            Self::MoLc { pod, node, lcslot } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/lcslot-{lcslot}/lc/running.json"
            )),
            Self::MoSysLc { lcslot } => {
                Cow::Owned(format!("mo/sys/ch/lcslot-{lcslot}/lc/running.json"))
            }
            Self::MoSup { pod, node, supslot } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/supslot-{supslot}/sup/running.json"
            )),
            Self::MoSysSup { supslot } => {
                Cow::Owned(format!("mo/sys/ch/supslot-{supslot}/sup/running.json"))
            }
        }
    }
}

pub type FirmwareCardRunning = AciObject<__internal::FirmwareCardRunning>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct FirmwareCardRunning;
    impl AciObjectScheme for FirmwareCardRunning {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = FirmwareCardRunningEndpoint;
        const CLASS_NAME: &'static str = "firmwareCardRunning";
    }
}
