use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::route_policy;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    bd_vnid: String,
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
    epg_vnid: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    l_2_stretch: String,
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
    #[serde(rename = "type", skip_serializing_if = "String::is_empty")]
    r#type: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    vrf_vnid: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    BgpRtP(route_policy::BgpRtP),
}

#[derive(Debug, Clone)]
pub enum BgpVniEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoEncapgroupevi {
        pod: String,
        node: String,
        encapgroupevi: String,
        vni: String,
        vrf: String,
        bd: String,
        epg: String,
    },
    MoSysEncapgroupevi {
        encapgroupevi: String,
        vni: String,
        vrf: String,
        bd: String,
        epg: String,
    },
}

impl EndpointScheme for BgpVniEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpVni.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoEncapgroupevi {
                pod,
                node,
                encapgroupevi,
                vni,
                vrf,
                bd,
                epg,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/encapgroupevi-{encapgroupevi}/vni-{vni}-vrf-[{vrf}]-bd-[{bd}]-epg-[{epg}].json")),
            Self::MoSysEncapgroupevi {
                encapgroupevi,
                vni,
                vrf,
                bd,
                epg,
            } => Cow::Owned(format!("mo/sys/bgp/inst/encapgroupevi-{encapgroupevi}/vni-{vni}-vrf-[{vrf}]-bd-[{bd}]-epg-[{epg}].json")),
        }
    }
}

pub type BgpVni = AciObject<__internal::BgpVni>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpVni;
    impl AciObjectScheme for BgpVni {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpVniEndpoint;
        const CLASS_NAME: &'static str = "bgpVni";
    }
}
