use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    cksum: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    cool_coe: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    fbits: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    hw_c_bits: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    len: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mac_b: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mac_l: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    max_c_pwr: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    oem_eprise: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "oemMIB")]
    oem_mib: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    sig: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "stackMIB")]
    stack_mib: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    ver: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {}

#[derive(Debug, Clone)]
pub enum EqptSpromBpBlkEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoExtChSpbp {
        pod: String,
        node: String,
        extch: String,
    },
    MoSysExtChSpbp {
        extch: String,
    },
    MoSpbp {
        pod: String,
        node: String,
    },
    MoSysSpbp,
}

impl EndpointScheme for EqptSpromBpBlkEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptSpromBPBlk.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoExtChSpbp { pod, node, extch } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/spbp/spbpblk.json"
            )),
            Self::MoSysExtChSpbp { extch } => {
                Cow::Owned(format!("mo/sys/extch-{extch}/spbp/spbpblk.json"))
            }
            Self::MoSpbp { pod, node } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/spbp/spbpblk.json"
            )),
            Self::MoSysSpbp => Cow::Borrowed("mo/sys/ch/spbp/spbpblk.json"),
        }
    }
}

pub type EqptSpromBpBlk = AciObject<__internal::EqptSpromBpBlk>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptSpromBpBlk;
    impl AciObjectScheme for EqptSpromBpBlk {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptSpromBpBlkEndpoint;
        const CLASS_NAME: &'static str = "eqptSpromBPBlk";
    }
}
