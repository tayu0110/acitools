use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    available: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    blocks: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    cap_utilized: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    device: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    fail_reason: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    file_system: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    firmware_version: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lc_own: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    media_wearout: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    model: String,
    #[allow(dead_code)]
    #[serde(skip_serializing, default)]
    mon_pol_dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mount: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name_alias: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    oper_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    serial: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    used: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    FaultCounts {},
    FaultInst {},
    HealthInst {},
}

#[derive(Debug, Clone)]
pub enum EqptStorageEndpoint {
    ClassAll,
    MoUni,
    MoCh {
        pod: String,
        node: String,
        p: String,
        f: String,
    },
    MoSysCh {
        p: String,
        f: String,
    },
}

impl EndpointScheme for EqptStorageEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptStorage.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoCh { pod, node, p, f } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/p-[{p}]-f-[{f}].json"
            )),
            Self::MoSysCh { p, f } => Cow::Owned(format!("mo/sys/ch/p-[{p}]-f-[{f}].json")),
        }
    }
}

pub type EqptStorage = AciObject<__internal::EqptStorage>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptStorage;
    impl AciObjectScheme for EqptStorage {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptStorageEndpoint;
        const CLASS_NAME: &'static str = "eqptStorage";
    }
}
