use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    accepted_paths: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    first_eor_rcvd_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    flags: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    last_eor_rcvd_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mem_acc_paths: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mon_pol_dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    peer_tbl_ver: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pfx_flushed: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pfx_saved: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pfx_sent: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    tbl_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    tbl_ver: String,
    #[serde(rename = "type", skip_serializing_if = "String::is_empty")]
    r#type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    FaultCounts {},
    FaultInst {},
    HealthInst {},
}

#[derive(Debug, Clone)]
pub enum BgpPeerAfEntryEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoEnt {
        pod: String,
        node: String,
        dom: String,
        peer: String,
        ent: String,
        af: String,
    },
    MoSysEnt {
        dom: String,
        peer: String,
        ent: String,
        af: String,
    },
}

impl EndpointScheme for BgpPeerAfEntryEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpPeerAfEntry.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoEnt {
                pod,
                node,
                dom,
                peer,
                ent,
                af,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/peer-[{peer}]/ent-[{ent}]/af-{af}.json")),
            Self::MoSysEnt {
                dom,
                peer,
                ent,
                af,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/peer-[{peer}]/ent-[{ent}]/af-{af}.json")),
        }
    }
}

pub type BgpPeerAfEntry = AciObject<__internal::BgpPeerAfEntry>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpPeerAfEntry;
    impl AciObjectScheme for BgpPeerAfEntry {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpPeerAfEntryEndpoint;
        const CLASS_NAME: &'static str = "bgpPeerAfEntry";
    }
}
