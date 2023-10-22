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
    hold_intvl: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ka_intvl: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lc_own: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    status: ConfigStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {}

#[derive(Debug, Clone)]
pub enum BgpPeerIntervalsEndpoint {
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

impl EndpointScheme for BgpPeerIntervalsEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpPeerIntervals.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoPeer {
                pod,
                node,
                dom,
                peer,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/peer-[{peer}]/peerintervals.json")),
            Self::MoSysPeer {
                dom,
                peer,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/peer-[{peer}]/peerintervals.json")),
        }
    }
}

pub type BgpPeerIntervals = AciObject<__internal::BgpPeerIntervals>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpPeerIntervals;
    impl AciObjectScheme for BgpPeerIntervals {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpPeerIntervalsEndpoint;
        const CLASS_NAME: &'static str = "bgpPeerIntervals";
    }
}
