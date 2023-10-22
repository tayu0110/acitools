use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::{graceful_restart_state, peer_address_family_entry, peer_entry_stats, peer_events};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    addr: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    adv_cap: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    conn_attempts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    conn_drop: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    conn_est: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    conn_if: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    fd: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    flags: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    hold_intvl: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ka_intvl: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    last_flap_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    local_ip: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    local_port: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mon_pol_dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    oper_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    passwd_set: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    peer_idx: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    prev_oper_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rcv_cap: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    remote_port: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rtr_id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    shut_st_qual: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    st_reason: String,
    status: ConfigStatus,
    #[serde(rename = "type", skip_serializing_if = "String::is_empty")]
    r#type: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    update_elapsed_ts: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    BgpGrSt(graceful_restart_state::BgpGrSt),
    BgpPeerAfEntry(peer_address_family_entry::BgpPeerAfEntry),
    BgpPeerEntryStats(peer_entry_stats::BgpPeerEntryStats),
    BgpPeerEvents(peer_events::BgpPeerEvents),
    FaultCounts {},
    FaultInst {},
    HealthInst {},
}

#[derive(Debug, Clone)]
pub enum BgpPeerEntryEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoPeer {
        pod: String,
        node: String,
        dom: String,
        peer: String,
        ent: String,
    },
    MoSysPeer {
        dom: String,
        peer: String,
        ent: String,
    },
}

impl EndpointScheme for BgpPeerEntryEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpPeerEntry.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoPeer {
                pod,
                node,
                dom,
                peer,
                ent,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/peer-[{peer}]/ent-[{ent}].json")),
            Self::MoSysPeer {
                dom,
                peer,
                ent,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/peer-[{peer}]/ent-[{ent}].json")),
        }
    }
}

pub type BgpPeerEntry = AciObject<__internal::BgpPeerEntry>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpPeerEntry;
    impl AciObjectScheme for BgpPeerEntry {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpPeerEntryEndpoint;
        const CLASS_NAME: &'static str = "bgpPeerEntry";
    }
}
