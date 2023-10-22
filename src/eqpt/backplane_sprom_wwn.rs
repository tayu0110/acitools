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
    sig: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    u_bits: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ver: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {}

#[derive(Debug, Clone)]
pub enum EqptBpSpWwnEndpoint {
    ClassAll,
    MoUni,
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

impl EndpointScheme for EqptBpSpWwnEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptBpSpWWN.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoExtChSpbp { pod, node, extch } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/spbp/bpspwwn.json"
            )),
            Self::MoSysExtChSpbp { extch } => {
                Cow::Owned(format!("mo/sys/extch-{extch}/spbp/bpspwwn.json"))
            }
            Self::MoSpbp { pod, node } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/spbp/bpspwwn.json"
            )),
            Self::MoSysSpbp => Cow::Borrowed("mo/sys/ch/spbp/bpspwwn.json"),
        }
    }
}

pub type EqptBpSpWwn = AciObject<__internal::EqptBpSpWwn>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptBpSpWwn;
    impl AciObjectScheme for EqptBpSpWwn {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptBpSpWwnEndpoint;
        const CLASS_NAME: &'static str = "eqptBpSpWWN";
    }
}
