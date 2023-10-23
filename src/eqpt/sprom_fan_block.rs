use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    amb_temp: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    cksum: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    cool_cap: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    fbits: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    hp_cool_con: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    hp_pwr_con: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    hw_c_bits: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    len: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
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
pub enum EqptSpromFanBlkEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoExtChSpfan {
        pod: String,
        node: String,
        extch: String,
        ftslot: String,
    },
    MoSysExtChSpfan {
        extch: String,
        ftslot: String,
    },
    MoSpfan {
        pod: String,
        node: String,
        ftslot: String,
    },
    MoSysSpfan {
        ftslot: String,
    },
}

impl EndpointScheme for EqptSpromFanBlkEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptSpromFanBlk.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoExtChSpfan {
                pod,
                node,
                extch,
                ftslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/ftslot-{ftslot}/ft/spfan/spfanblk.json")),
            Self::MoSysExtChSpfan {
                extch,
                ftslot,
            } => Cow::Owned(format!("mo/sys/extch-{extch}/ftslot-{ftslot}/ft/spfan/spfanblk.json")),
            Self::MoSpfan {
                pod,
                node,
                ftslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/ftslot-{ftslot}/ft/spfan/spfanblk.json")),
            Self::MoSysSpfan {
                ftslot,
            } => Cow::Owned(format!("mo/sys/ch/ftslot-{ftslot}/ft/spfan/spfanblk.json")),
        }
    }
}

pub type EqptSpromFanBlk = AciObject<__internal::EqptSpromFanBlk>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptSpromFanBlk;
    impl AciObjectScheme for EqptSpromFanBlk {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptSpromFanBlkEndpoint;
        const CLASS_NAME: &'static str = "eqptSpromFanBlk";
    }
}
