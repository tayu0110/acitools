use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    byte_in_recv_q: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    byte_in_send_q: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    byte_rcvd: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    byte_sent: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    cap_rcvd: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    cap_sent: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    connect_retry_ts: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ka_rcvd: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ka_sent: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ka_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    mon_pol_dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    msg_rcvd: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    msg_sent: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    notif_rcvd: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    notif_sent: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    open_rcvd: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    open_sent: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    route_refresh_rcvd: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    route_refresh_sent: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    update_rcvd: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    update_sent: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    #[serde(rename = "bgpBgpPeerBytes15min")]
    BgpBgpPeerBytes15Min {},
    #[serde(rename = "bgpBgpPeerBytes1d")]
    BgpBgpPeerBytes1D {},
    #[serde(rename = "bgpBgpPeerBytes1h")]
    BgpBgpPeerBytes1H {},
    #[serde(rename = "bgpBgpPeerBytes1mo")]
    BgpBgpPeerBytes1Mo {},
    #[serde(rename = "bgpBgpPeerBytes1qtr")]
    BgpBgpPeerBytes1Qtr {},
    #[serde(rename = "bgpBgpPeerBytes1w")]
    BgpBgpPeerBytes1W {},
    #[serde(rename = "bgpBgpPeerBytes1year")]
    BgpBgpPeerBytes1Year {},
    #[serde(rename = "bgpBgpPeerBytes5min")]
    BgpBgpPeerBytes5Min {},
    #[serde(rename = "bgpBgpPeerBytesHist15min")]
    BgpBgpPeerBytesHist15Min {},
    #[serde(rename = "bgpBgpPeerBytesHist1d")]
    BgpBgpPeerBytesHist1D {},
    #[serde(rename = "bgpBgpPeerBytesHist1h")]
    BgpBgpPeerBytesHist1H {},
    #[serde(rename = "bgpBgpPeerBytesHist1mo")]
    BgpBgpPeerBytesHist1Mo {},
    #[serde(rename = "bgpBgpPeerBytesHist1qtr")]
    BgpBgpPeerBytesHist1Qtr {},
    #[serde(rename = "bgpBgpPeerBytesHist1w")]
    BgpBgpPeerBytesHist1W {},
    #[serde(rename = "bgpBgpPeerBytesHist1year")]
    BgpBgpPeerBytesHist1Year {},
    #[serde(rename = "bgpBgpPeerBytesHist5min")]
    BgpBgpPeerBytesHist5Min {},
    #[serde(rename = "bgpBgpPeerKeepAlive15min")]
    BgpBgpPeerKeepAlive15Min {},
    #[serde(rename = "bgpBgpPeerKeepAlive1d")]
    BgpBgpPeerKeepAlive1D {},
    #[serde(rename = "bgpBgpPeerKeepAlive1h")]
    BgpBgpPeerKeepAlive1H {},
    #[serde(rename = "bgpBgpPeerKeepAlive1mo")]
    BgpBgpPeerKeepAlive1Mo {},
    #[serde(rename = "bgpBgpPeerKeepAlive1qtr")]
    BgpBgpPeerKeepAlive1Qtr {},
    #[serde(rename = "bgpBgpPeerKeepAlive1w")]
    BgpBgpPeerKeepAlive1W {},
    #[serde(rename = "bgpBgpPeerKeepAlive1year")]
    BgpBgpPeerKeepAlive1Year {},
    #[serde(rename = "bgpBgpPeerKeepAlive5min")]
    BgpBgpPeerKeepAlive5Min {},
    #[serde(rename = "bgpBgpPeerKeepAliveHist15min")]
    BgpBgpPeerKeepAliveHist15Min {},
    #[serde(rename = "bgpBgpPeerKeepAliveHist1d")]
    BgpBgpPeerKeepAliveHist1D {},
    #[serde(rename = "bgpBgpPeerKeepAliveHist1h")]
    BgpBgpPeerKeepAliveHist1H {},
    #[serde(rename = "bgpBgpPeerKeepAliveHist1mo")]
    BgpBgpPeerKeepAliveHist1Mo {},
    #[serde(rename = "bgpBgpPeerKeepAliveHist1qtr")]
    BgpBgpPeerKeepAliveHist1Qtr {},
    #[serde(rename = "bgpBgpPeerKeepAliveHist1w")]
    BgpBgpPeerKeepAliveHist1W {},
    #[serde(rename = "bgpBgpPeerKeepAliveHist1year")]
    BgpBgpPeerKeepAliveHist1Year {},
    #[serde(rename = "bgpBgpPeerKeepAliveHist5min")]
    BgpBgpPeerKeepAliveHist5Min {},
    #[serde(rename = "bgpBgpPeerMsg15min")]
    BgpBgpPeerMsg15Min {},
    #[serde(rename = "bgpBgpPeerMsg1d")]
    BgpBgpPeerMsg1D {},
    #[serde(rename = "bgpBgpPeerMsg1h")]
    BgpBgpPeerMsg1H {},
    #[serde(rename = "bgpBgpPeerMsg1mo")]
    BgpBgpPeerMsg1Mo {},
    #[serde(rename = "bgpBgpPeerMsg1qtr")]
    BgpBgpPeerMsg1Qtr {},
    #[serde(rename = "bgpBgpPeerMsg1w")]
    BgpBgpPeerMsg1W {},
    #[serde(rename = "bgpBgpPeerMsg1year")]
    BgpBgpPeerMsg1Year {},
    #[serde(rename = "bgpBgpPeerMsg5min")]
    BgpBgpPeerMsg5Min {},
    #[serde(rename = "bgpBgpPeerMsgHist15min")]
    BgpBgpPeerMsgHist15Min {},
    #[serde(rename = "bgpBgpPeerMsgHist1d")]
    BgpBgpPeerMsgHist1D {},
    #[serde(rename = "bgpBgpPeerMsgHist1h")]
    BgpBgpPeerMsgHist1H {},
    #[serde(rename = "bgpBgpPeerMsgHist1mo")]
    BgpBgpPeerMsgHist1Mo {},
    #[serde(rename = "bgpBgpPeerMsgHist1qtr")]
    BgpBgpPeerMsgHist1Qtr {},
    #[serde(rename = "bgpBgpPeerMsgHist1w")]
    BgpBgpPeerMsgHist1W {},
    #[serde(rename = "bgpBgpPeerMsgHist1year")]
    BgpBgpPeerMsgHist1Year {},
    #[serde(rename = "bgpBgpPeerMsgHist5min")]
    BgpBgpPeerMsgHist5Min {},
    #[serde(rename = "bgpBgpPeerOpen15min")]
    BgpBgpPeerOpen15Min {},
    #[serde(rename = "bgpBgpPeerOpen1d")]
    BgpBgpPeerOpen1D {},
    #[serde(rename = "bgpBgpPeerOpen1h")]
    BgpBgpPeerOpen1H {},
    #[serde(rename = "bgpBgpPeerOpen1mo")]
    BgpBgpPeerOpen1Mo {},
    #[serde(rename = "bgpBgpPeerOpen1qtr")]
    BgpBgpPeerOpen1Qtr {},
    #[serde(rename = "bgpBgpPeerOpen1w")]
    BgpBgpPeerOpen1W {},
    #[serde(rename = "bgpBgpPeerOpen1year")]
    BgpBgpPeerOpen1Year {},
    #[serde(rename = "bgpBgpPeerOpen5min")]
    BgpBgpPeerOpen5Min {},
    #[serde(rename = "bgpBgpPeerOpenHist15min")]
    BgpBgpPeerOpenHist15Min {},
    #[serde(rename = "bgpBgpPeerOpenHist1d")]
    BgpBgpPeerOpenHist1D {},
    #[serde(rename = "bgpBgpPeerOpenHist1h")]
    BgpBgpPeerOpenHist1H {},
    #[serde(rename = "bgpBgpPeerOpenHist1mo")]
    BgpBgpPeerOpenHist1Mo {},
    #[serde(rename = "bgpBgpPeerOpenHist1qtr")]
    BgpBgpPeerOpenHist1Qtr {},
    #[serde(rename = "bgpBgpPeerOpenHist1w")]
    BgpBgpPeerOpenHist1W {},
    #[serde(rename = "bgpBgpPeerOpenHist1year")]
    BgpBgpPeerOpenHist1Year {},
    #[serde(rename = "bgpBgpPeerOpenHist5min")]
    BgpBgpPeerOpenHist5Min {},
    #[serde(rename = "bgpBgpPeerRoute15min")]
    BgpBgpPeerRoute15Min {},
    #[serde(rename = "bgpBgpPeerRoute1d")]
    BgpBgpPeerRoute1D {},
    #[serde(rename = "bgpBgpPeerRoute1h")]
    BgpBgpPeerRoute1H {},
    #[serde(rename = "bgpBgpPeerRoute1mo")]
    BgpBgpPeerRoute1Mo {},
    #[serde(rename = "bgpBgpPeerRoute1qtr")]
    BgpBgpPeerRoute1Qtr {},
    #[serde(rename = "bgpBgpPeerRoute1w")]
    BgpBgpPeerRoute1W {},
    #[serde(rename = "bgpBgpPeerRoute1year")]
    BgpBgpPeerRoute1Year {},
    #[serde(rename = "bgpBgpPeerRoute5min")]
    BgpBgpPeerRoute5Min {},
    #[serde(rename = "bgpBgpPeerRouteHist15min")]
    BgpBgpPeerRouteHist15Min {},
    #[serde(rename = "bgpBgpPeerRouteHist1d")]
    BgpBgpPeerRouteHist1D {},
    #[serde(rename = "bgpBgpPeerRouteHist1h")]
    BgpBgpPeerRouteHist1H {},
    #[serde(rename = "bgpBgpPeerRouteHist1mo")]
    BgpBgpPeerRouteHist1Mo {},
    #[serde(rename = "bgpBgpPeerRouteHist1qtr")]
    BgpBgpPeerRouteHist1Qtr {},
    #[serde(rename = "bgpBgpPeerRouteHist1w")]
    BgpBgpPeerRouteHist1W {},
    #[serde(rename = "bgpBgpPeerRouteHist1year")]
    BgpBgpPeerRouteHist1Year {},
    #[serde(rename = "bgpBgpPeerRouteHist5min")]
    BgpBgpPeerRouteHist5Min {},
    #[serde(rename = "bgpBgpRtPrefixCount15min")]
    BgpBgpRtPrefixCount15Min {},
    #[serde(rename = "bgpBgpRtPrefixCount1d")]
    BgpBgpRtPrefixCount1D {},
    #[serde(rename = "bgpBgpRtPrefixCount1h")]
    BgpBgpRtPrefixCount1H {},
    #[serde(rename = "bgpBgpRtPrefixCount1mo")]
    BgpBgpRtPrefixCount1Mo {},
    #[serde(rename = "bgpBgpRtPrefixCount1qtr")]
    BgpBgpRtPrefixCount1Qtr {},
    #[serde(rename = "bgpBgpRtPrefixCount1w")]
    BgpBgpRtPrefixCount1W {},
    #[serde(rename = "bgpBgpRtPrefixCount1year")]
    BgpBgpRtPrefixCount1Year {},
    #[serde(rename = "bgpBgpRtPrefixCount5min")]
    BgpBgpRtPrefixCount5Min {},
    #[serde(rename = "bgpBgpRtPrefixCountHist15min")]
    BgpBgpRtPrefixCountHist15Min {},
    #[serde(rename = "bgpBgpRtPrefixCountHist1d")]
    BgpBgpRtPrefixCountHist1D {},
    #[serde(rename = "bgpBgpRtPrefixCountHist1h")]
    BgpBgpRtPrefixCountHist1H {},
    #[serde(rename = "bgpBgpRtPrefixCountHist1mo")]
    BgpBgpRtPrefixCountHist1Mo {},
    #[serde(rename = "bgpBgpRtPrefixCountHist1qtr")]
    BgpBgpRtPrefixCountHist1Qtr {},
    #[serde(rename = "bgpBgpRtPrefixCountHist1w")]
    BgpBgpRtPrefixCountHist1W {},
    #[serde(rename = "bgpBgpRtPrefixCountHist1year")]
    BgpBgpRtPrefixCountHist1Year {},
    #[serde(rename = "bgpBgpRtPrefixCountHist5min")]
    BgpBgpRtPrefixCountHist5Min {},
    FaultCounts {},
    FaultInst {},
    HealthInst {},
}

#[derive(Debug, Clone)]
pub enum BgpPeerEntryStatsEndpoint {
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

impl EndpointScheme for BgpPeerEntryStatsEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpPeerEntryStats.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoEnt {
                pod,
                node,
                dom,
                peer,
                ent,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/peer-[{peer}]/ent-[{ent}]/peerstats.json")),
            Self::MoSysEnt {
                dom,
                peer,
                ent,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/peer-[{peer}]/ent-[{ent}]/peerstats.json")),
        }
    }
}

pub type BgpPeerEntryStats = AciObject<__internal::BgpPeerEntryStats>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpPeerEntryStats;
    impl AciObjectScheme for BgpPeerEntryStats {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpPeerEntryStatsEndpoint;
        const CLASS_NAME: &'static str = "bgpPeerEntryStats";
    }
}
