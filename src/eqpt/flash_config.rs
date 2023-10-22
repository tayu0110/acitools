use crate::{AciObject, ConfigStatus, Configurable, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    annotation: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    deltape: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ext_mngd_by: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    gbb: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lc_own: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pe_cycles: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    read_err: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    uid: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    userdom: String,
}

impl Configurable for Attributes {
    fn set_status(&mut self, status: ConfigStatus) {
        self.status = status;
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    AaaRbacAnnotation {},
    TagAnnotation {},
    TagTag {},
}

#[derive(Debug, Clone)]
pub enum EqptFlashConfigEndpoint {
    ClassAll,
    MoUni,
    MoCh { pod: String, node: String },
    MoSysCh,
}

impl EndpointScheme for EqptFlashConfigEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptFlashConfig.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoCh { pod, node } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/flashconfig.json"
            )),
            Self::MoSysCh => Cow::Owned(format!("mo/sys/ch/flashconfig.json")),
        }
    }
}

pub type EqptFlashConfig = AciObject<__internal::EqptFlashConfig>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptFlashConfig;
    impl AciObjectScheme for EqptFlashConfig {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptFlashConfigEndpoint;
        const CLASS_NAME: &'static str = "eqptFlashConfig";
    }
}
