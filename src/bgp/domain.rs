use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::{
    bridge_domain_evpn_instance, db, domain_address_family, domain_evpn_instance, graceful_restart,
    infra_l3out, path_control, peer,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    bgp_cfg_failed_bmp: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    bgp_cfg_failed_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    bgp_cfg_state: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    cluster_id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ctrl: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    encap: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    evpn_rd: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    first_best_path_delay: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    first_peer_up_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    flags: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    hold_intvl: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    internal_domain_id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ka_intvl: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lc_own: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    max_as_limit: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mode: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mon_pol_dn: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "mplsVPNLabelIndex")]
    mpls_vpn_label_index: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    msite_cluster_id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    num_est_peers: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    num_peers: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    oper_rtr_id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    oper_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rd: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rtr_id: String,
    status: ConfigStatus,
    #[serde(rename = "type", skip_serializing_if = "String::is_empty")]
    r#type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    BgpBdEvi(bridge_domain_evpn_instance::BgpBdEvi),
    BgpDb(db::BgpDb),
    BgpDomAf(domain_address_family::BgpDomAf),
    BgpDomEvi(domain_evpn_instance::BgpDomEvi),
    BgpGr(graceful_restart::BgpGr),
    BgpInfraL3Out(infra_l3out::BgpInfraL3Out),
    BgpPathCtrl(path_control::BgpPathCtrl),
    BgpPeer(peer::BgpPeer),
    FaultCounts {},
    FaultInst {},
    HealthInst {},
}

#[derive(Debug, Clone)]
pub enum BgpDomEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoInst {
        pod: String,
        node: String,
        dom: String,
    },
    MoSysInst {
        dom: String,
    },
}

impl EndpointScheme for BgpDomEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpDom.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoInst { pod, node, dom } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}.json"
            )),
            Self::MoSysInst { dom } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}.json")),
        }
    }
}

pub type BgpDom = AciObject<__internal::BgpDom>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpDom;
    impl AciObjectScheme for BgpDom {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpDomEndpoint;
        const CLASS_NAME: &'static str = "bgpDom";
    }
}
