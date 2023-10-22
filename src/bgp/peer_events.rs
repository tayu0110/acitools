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
    last_err_data_rsvd: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    last_err_data_sent: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    last_err_len_rsvd: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    last_err_len_sent: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    last_err_val_rsvd: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    last_err_val_sent: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    maj_err_rst_rsvd: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    maj_err_rst_sent: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    min_err_rst_rsvd: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    min_err_rst_sent: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    mon_pol_dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rst_rsvd_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rst_sent_ts: String,
    status: ConfigStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {}

#[derive(Debug, Clone)]
pub enum BgpPeerEventsEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoEnt {
        pod: String,
        node: String,
        dom: String,
        peer: String,
        ent: String,
    },
    MoSysEnt {
        dom: String,
        peer: String,
        ent: String,
    },
}

impl EndpointScheme for BgpPeerEventsEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpPeerEvents.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoEnt {
                pod,
                node,
                dom,
                peer,
                ent,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/peer-[{peer}]/ent-[{ent}]/ev.json")),
            Self::MoSysEnt {
                dom,
                peer,
                ent,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/peer-[{peer}]/ent-[{ent}]/ev.json")),
        }
    }
}

pub type BgpPeerEvents = AciObject<__internal::BgpPeerEvents>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpPeerEvents;
    impl AciObjectScheme for BgpPeerEvents {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpPeerEventsEndpoint;
        const CLASS_NAME: &'static str = "bgpPeerEvents";
    }
}
