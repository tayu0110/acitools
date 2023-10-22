use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::attached_nexthop;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    active_rnh_epoch: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    addr: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    flags: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    igp_metric: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    igp_pref: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    igp_rt_type: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    last_resolv: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    next_adv: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pending_rnh_epoch: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    resolved_rt: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rt_ref_count: String,
    status: ConfigStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    BgpAttNextHop(attached_nexthop::BgpAttNextHop),
}

#[derive(Debug, Clone)]
pub enum BgpNextHopEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoAf {
        pod: String,
        node: String,
        dom: String,
        af: String,
        nh: String,
    },
    MoSysAf {
        dom: String,
        af: String,
        nh: String,
    },
}

impl EndpointScheme for BgpNextHopEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpNextHop.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoAf {
                pod,
                node,
                dom,
                af,
                nh,
            } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/af-{af}/nh-[{nh}].json"
            )),
            Self::MoSysAf { dom, af, nh } => {
                Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/af-{af}/nh-[{nh}].json"))
            }
        }
    }
}

pub type BgpNextHop = AciObject<__internal::BgpNextHop>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpNextHop;
    impl AciObjectScheme for BgpNextHop {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpNextHopEndpoint;
        const CLASS_NAME: &'static str = "bgpNextHop";
    }
}
