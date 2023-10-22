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
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    len: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ser_num: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    sig: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    ver: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {}

#[derive(Debug, Clone)]
pub enum EqptSpromFanSnEndpoint {
    ClassAll,
    MoUni,
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

impl EndpointScheme for EqptSpromFanSnEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptSpromFanSN.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoExtChSpfan {
                pod,
                node,
                extch,
                ftslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/ftslot-{ftslot}/ft/spfan/spfansn.json")),
            Self::MoSysExtChSpfan {
                extch,
                ftslot,
            } => Cow::Owned(format!("mo/sys/extch-{extch}/ftslot-{ftslot}/ft/spfan/spfansn.json")),
            Self::MoSpfan {
                pod,
                node,
                ftslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/ftslot-{ftslot}/ft/spfan/spfansn.json")),
            Self::MoSysSpfan {
                ftslot,
            } => Cow::Owned(format!("mo/sys/ch/ftslot-{ftslot}/ft/spfan/spfansn.json")),
        }
    }
}

pub type EqptSpromFanSn = AciObject<__internal::EqptSpromFanSn>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptSpromFanSn;
    impl AciObjectScheme for EqptSpromFanSn {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptSpromFanSnEndpoint;
        const CLASS_NAME: &'static str = "eqptSpromFanSN";
    }
}
