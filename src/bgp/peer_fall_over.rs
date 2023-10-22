use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lc_own: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rt_map: String,
    status: ConfigStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {}

#[derive(Debug, Clone)]
pub enum BgpPeerFallOverEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoPeer {
        pod: String,
        node: String,
        dom: String,
        peer: String,
    },
    MoSysPeer {
        dom: String,
        peer: String,
    },
}

impl EndpointScheme for BgpPeerFallOverEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpPeerFallOver.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoPeer {
                pod,
                node,
                dom,
                peer,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/peer-[{peer}]/peerfallover.json")),
            Self::MoSysPeer {
                dom,
                peer,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/peer-[{peer}]/peerfallover.json")),
        }
    }
}

pub type BgpPeerFallOver = AciObject<__internal::BgpPeerFallOver>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpPeerFallOver;
    impl AciObjectScheme for BgpPeerFallOver {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpPeerFallOverEndpoint;
        const CLASS_NAME: &'static str = "bgpPeerFallOver";
    }
}
