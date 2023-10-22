use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::{route_policy, route_target_policy, vni};

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
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    flags: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lc_own: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rd: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    status: ConfigStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    BgpRtP(route_policy::BgpRtP),
    BgpRttP(route_target_policy::BgpRttP),
    BgpVni(vni::BgpVni),
}

#[derive(Debug, Clone)]
pub enum BgpEncapGroupEviEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoInst {
        pod: String,
        node: String,
        encapgroupevi: String,
    },
    MoSysInst {
        encapgroupevi: String,
    },
}

impl EndpointScheme for BgpEncapGroupEviEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpEncapGroupEvi.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoInst {
                pod,
                node,
                encapgroupevi,
            } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/encapgroupevi-{encapgroupevi}.json"
            )),
            Self::MoSysInst { encapgroupevi } => Cow::Owned(format!(
                "mo/sys/bgp/inst/encapgroupevi-{encapgroupevi}.json"
            )),
        }
    }
}

pub type BgpEncapGroupEvi = AciObject<__internal::BgpEncapGroupEvi>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpEncapGroupEvi;
    impl AciObjectScheme for BgpEncapGroupEvi {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpEncapGroupEviEndpoint;
        const CLASS_NAME: &'static str = "bgpEncapGroupEvi";
    }
}
