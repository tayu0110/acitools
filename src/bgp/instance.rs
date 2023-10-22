use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::{domain, encap_group_evpn_instance};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    activate_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    admin_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    as_path_db_sz: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    asn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    attrib_db_sz: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    create_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ctrl: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    domain_id_base: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    fabric_so_o: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lc_own: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    m_site_so_o: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mem_alert: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mon_pol_dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    num_as_path: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    num_rt_attrib: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    oper_err: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pod_so_o: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    snmp_trap_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    spine_role: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    suppr_rt: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    syslog_lvl: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ver: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    wait_done_ts: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    BgpDom(domain::BgpDom),
    BgpEncapGroupEvi(encap_group_evpn_instance::BgpEncapGroupEvi),
    FaultCounts {},
    FaultInst {},
    HealthInst {},
}

#[derive(Debug, Clone)]
pub enum BgpInstEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoBgp { pod: String, node: String },
    MoSysBgp,
}

impl EndpointScheme for BgpInstEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpInst.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoBgp { pod, node } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/bgp/inst.json"
            )),
            Self::MoSysBgp => Cow::Borrowed("mo/sys/bgp/inst.json"),
        }
    }
}

pub type BgpInst = AciObject<__internal::BgpInst>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpInst;
    impl AciObjectScheme for BgpInst {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpInstEndpoint;
        const CLASS_NAME: &'static str = "bgpInst";
    }
}
