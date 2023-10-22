use super::{local_asn, peer_address_family, peer_entry, peer_fall_over, peer_intervals};
use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    active_pfx_peers: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    addr: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    admin_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    asn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    bgp_cfg_failed_bmp: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    bgp_cfg_failed_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    bgp_cfg_state: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ctrl: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    cur_pfx_peers: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    external_domain_id: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "infraL3outName")]
    infra_l_3_out_name: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "l3outInTag")]
    l_3_out_in_tag: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lc_own: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    max_cur_peers: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    max_pfx_peers: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    mon_pol_dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    passwd_set: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    password: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    peer_role: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    private_a_sctrl: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    src_if: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    total_pfx_peers: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ttl: String,
    #[serde(rename = "type", skip_serializing_if = "String::is_empty")]
    r#type: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    type_ext: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    BgpLocalAsn(local_asn::BgpLocalAsn),
    BgpPeerAf(peer_address_family::BgpPeerAf),
    #[serde(rename = "bgpPeerBytes15min")]
    BgpPeerBytes15Min {},
    #[serde(rename = "bgpPeerBytes1d")]
    BgpPeerBytes1D {},
    #[serde(rename = "bgpPeerBytes1h")]
    BgpPeerBytes1H {},
    #[serde(rename = "bgpPeerBytes1mo")]
    BgpPeerBytes1Mo {},
    #[serde(rename = "bgpPeerBytes1qtr")]
    BgpPeerBytes1Qtr {},
    #[serde(rename = "bgpPeerBytes1w")]
    BgpPeerBytes1W {},
    #[serde(rename = "bgpPeerBytes1year")]
    BgpPeerBytes1Year {},
    #[serde(rename = "bgpPeerBytes5min")]
    BgpPeerBytes5Min {},
    #[serde(rename = "bgpPeerBytesHist15min")]
    BgpPeerBytesHist15Min {},
    #[serde(rename = "bgpPeerBytesHist1d")]
    BgpPeerBytesHist1D {},
    #[serde(rename = "bgpPeerBytesHist1h")]
    BgpPeerBytesHist1H {},
    #[serde(rename = "bgpPeerBytesHist1mo")]
    BgpPeerBytesHist1Mo {},
    #[serde(rename = "bgpPeerBytesHist1qtr")]
    BgpPeerBytesHist1Qtr {},
    #[serde(rename = "bgpPeerBytesHist1w")]
    BgpPeerBytesHist1W {},
    #[serde(rename = "bgpPeerBytesHist1year")]
    BgpPeerBytesHist1Year {},
    #[serde(rename = "bgpPeerBytesHist5min")]
    BgpPeerBytesHist5Min {},
    BgpPeerEntry(peer_entry::BgpPeerEntry),
    BgpPeerFallOver(peer_fall_over::BgpPeerFallOver),
    BgpPeerIntervals(peer_intervals::BgpPeerIntervals),
    #[serde(rename = "bgpPeerKeepAlive15min")]
    BgpPeerKeepAlive15Min {},
    #[serde(rename = "bgpPeerKeepAlive1d")]
    BgpPeerKeepAlive1D {},
    #[serde(rename = "bgpPeerKeepAlive1h")]
    BgpPeerKeepAlive1H {},
    #[serde(rename = "bgpPeerKeepAlive1mo")]
    BgpPeerKeepAlive1Mo {},
    #[serde(rename = "bgpPeerKeepAlive1qtr")]
    BgpPeerKeepAlive1Qtr {},
    #[serde(rename = "bgpPeerKeepAlive1w")]
    BgpPeerKeepAlive1W {},
    #[serde(rename = "bgpPeerKeepAlive1year")]
    BgpPeerKeepAlive1Year {},
    #[serde(rename = "bgpPeerKeepAlive5min")]
    BgpPeerKeepAlive5Min {},
    #[serde(rename = "bgpPeerKeepAliveHist15min")]
    BgpPeerKeepAliveHist15Min {},
    #[serde(rename = "bgpPeerKeepAliveHist1d")]
    BgpPeerKeepAliveHist1D {},
    #[serde(rename = "bgpPeerKeepAliveHist1h")]
    BgpPeerKeepAliveHist1H {},
    #[serde(rename = "bgpPeerKeepAliveHist1mo")]
    BgpPeerKeepAliveHist1Mo {},
    #[serde(rename = "bgpPeerKeepAliveHist1qtr")]
    BgpPeerKeepAliveHist1Qtr {},
    #[serde(rename = "bgpPeerKeepAliveHist1w")]
    BgpPeerKeepAliveHist1W {},
    #[serde(rename = "bgpPeerKeepAliveHist1year")]
    BgpPeerKeepAliveHist1Year {},
    #[serde(rename = "bgpPeerKeepAliveHist5min")]
    BgpPeerKeepAliveHist5Min {},
    #[serde(rename = "bgpPeerMsg15min")]
    BgpPeerMsg15Min {},
    #[serde(rename = "bgpPeerMsg1d")]
    BgpPeerMsg1D {},
    #[serde(rename = "bgpPeerMsg1h")]
    BgpPeerMsg1H {},
    #[serde(rename = "bgpPeerMsg1mo")]
    BgpPeerMsg1Mo {},
    #[serde(rename = "bgpPeerMsg1qtr")]
    BgpPeerMsg1Qtr {},
    #[serde(rename = "bgpPeerMsg1w")]
    BgpPeerMsg1W {},
    #[serde(rename = "bgpPeerMsg1year")]
    BgpPeerMsg1Year {},
    #[serde(rename = "bgpPeerMsg5min")]
    BgpPeerMsg5Min {},
    #[serde(rename = "bgpPeerMsgHist15min")]
    BgpPeerMsgHist15Min {},
    #[serde(rename = "bgpPeerMsgHist1d")]
    BgpPeerMsgHist1D {},
    #[serde(rename = "bgpPeerMsgHist1h")]
    BgpPeerMsgHist1H {},
    #[serde(rename = "bgpPeerMsgHist1mo")]
    BgpPeerMsgHist1Mo {},
    #[serde(rename = "bgpPeerMsgHist1qtr")]
    BgpPeerMsgHist1Qtr {},
    #[serde(rename = "bgpPeerMsgHist1w")]
    BgpPeerMsgHist1W {},
    #[serde(rename = "bgpPeerMsgHist1year")]
    BgpPeerMsgHist1Year {},
    #[serde(rename = "bgpPeerMsgHist5min")]
    BgpPeerMsgHist5Min {},
    #[serde(rename = "bgpPeerOpen15min")]
    BgpPeerOpen15Min {},
    #[serde(rename = "bgpPeerOpen1d")]
    BgpPeerOpen1D {},
    #[serde(rename = "bgpPeerOpen1h")]
    BgpPeerOpen1H {},
    #[serde(rename = "bgpPeerOpen1mo")]
    BgpPeerOpen1Mo {},
    #[serde(rename = "bgpPeerOpen1qtr")]
    BgpPeerOpen1Qtr {},
    #[serde(rename = "bgpPeerOpen1w")]
    BgpPeerOpen1W {},
    #[serde(rename = "bgpPeerOpen1year")]
    BgpPeerOpen1Year {},
    #[serde(rename = "bgpPeerOpen5min")]
    BgpPeerOpen5Min {},
    #[serde(rename = "bgpPeerOpenHist15min")]
    BgpPeerOpenHist15Min {},
    #[serde(rename = "bgpPeerOpenHist1d")]
    BgpPeerOpenHist1D {},
    #[serde(rename = "bgpPeerOpenHist1h")]
    BgpPeerOpenHist1H {},
    #[serde(rename = "bgpPeerOpenHist1mo")]
    BgpPeerOpenHist1Mo {},
    #[serde(rename = "bgpPeerOpenHist1qtr")]
    BgpPeerOpenHist1Qtr {},
    #[serde(rename = "bgpPeerOpenHist1w")]
    BgpPeerOpenHist1W {},
    #[serde(rename = "bgpPeerOpenHist1year")]
    BgpPeerOpenHist1Year {},
    #[serde(rename = "bgpPeerOpenHist5min")]
    BgpPeerOpenHist5Min {},
    #[serde(rename = "bgpPeerRoute15min")]
    BgpPeerRoute15Min {},
    #[serde(rename = "bgpPeerRoute1d")]
    BgpPeerRoute1D {},
    #[serde(rename = "bgpPeerRoute1h")]
    BgpPeerRoute1H {},
    #[serde(rename = "bgpPeerRoute1mo")]
    BgpPeerRoute1Mo {},
    #[serde(rename = "bgpPeerRoute1qtr")]
    BgpPeerRoute1Qtr {},
    #[serde(rename = "bgpPeerRoute1w")]
    BgpPeerRoute1W {},
    #[serde(rename = "bgpPeerRoute1year")]
    BgpPeerRoute1Year {},
    #[serde(rename = "bgpPeerRoute5min")]
    BgpPeerRoute5Min {},
    #[serde(rename = "bgpPeerRouteHist15min")]
    BgpPeerRouteHist15Min {},
    #[serde(rename = "bgpPeerRouteHist1d")]
    BgpPeerRouteHist1D {},
    #[serde(rename = "bgpPeerRouteHist1h")]
    BgpPeerRouteHist1H {},
    #[serde(rename = "bgpPeerRouteHist1mo")]
    BgpPeerRouteHist1Mo {},
    #[serde(rename = "bgpPeerRouteHist1qtr")]
    BgpPeerRouteHist1Qtr {},
    #[serde(rename = "bgpPeerRouteHist1w")]
    BgpPeerRouteHist1W {},
    #[serde(rename = "bgpPeerRouteHist1year")]
    BgpPeerRouteHist1Year {},
    #[serde(rename = "bgpPeerRouteHist5min")]
    BgpPeerRouteHist5Min {},
    FaultCounts {},
    FaultInst {},
    HealthInst {},
}

#[derive(Debug, Clone)]
pub enum BgpPeerEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoDom {
        pod: String,
        node: String,
        dom: String,
        peer: String,
    },
    MoSysDom {
        dom: String,
        peer: String,
    },
}

impl EndpointScheme for BgpPeerEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpPeer.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoDom {
                pod,
                node,
                dom,
                peer,
            } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/peer-[{peer}].json"
            )),
            Self::MoSysDom { dom, peer } => {
                Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/peer-[{peer}].json"))
            }
        }
    }
}

pub type BgpPeer = AciObject<__internal::BgpPeer>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpPeer;
    impl AciObjectScheme for BgpPeer {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpPeerEndpoint;
        const CLASS_NAME: &'static str = "bgpPeer";
    }
}
