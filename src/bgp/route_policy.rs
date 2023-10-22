use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::{route_control_map_policy, route_target_entry};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    descr: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    instantiation_t: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lc_own: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name_alias: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    status: ConfigStatus,
    #[serde(rename = "type", skip_serializing_if = "String::is_empty")]
    r#type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    BgpRtCtrlMapP(route_control_map_policy::BgpRtCtrlMapP),
    BgpRttEntry(route_target_entry::BgpRttEntry),
    FaultDelegate {},
}

#[derive(Debug, Clone)]
pub enum BgpRtPEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoVni {
        pod: String,
        node: String,
        encapgroupevi: String,
        vni: String,
        vrf: String,
        bd: String,
        epg: String,
        rtp: String,
    },
    MoSysVni {
        encapgroupevi: String,
        vni: String,
        vrf: String,
        bd: String,
        epg: String,
        rtp: String,
    },
    MoCtrl {
        pod: String,
        node: String,
        dom: String,
        af: String,
        ctrl: String,
        rtp: String,
    },
    MoSysCtrl {
        dom: String,
        af: String,
        ctrl: String,
        rtp: String,
    },
    MoEncapgroupevi {
        pod: String,
        node: String,
        encapgroupevi: String,
        rtp: String,
    },
    MoSysEncapgroupevi {
        encapgroupevi: String,
        rtp: String,
    },
}

impl EndpointScheme for BgpRtPEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpRtP.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoVni {
                pod,
                node,
                encapgroupevi,
                vni,
                vrf,
                bd,
                epg,
                rtp,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/encapgroupevi-{encapgroupevi}/vni-{vni}-vrf-[{vrf}]-bd-[{bd}]-epg-[{epg}]/rtp-{rtp}.json")),
            Self::MoSysVni {
                encapgroupevi,
                vni,
                vrf,
                bd,
                epg,
                rtp,
            } => Cow::Owned(format!("mo/sys/bgp/inst/encapgroupevi-{encapgroupevi}/vni-{vni}-vrf-[{vrf}]-bd-[{bd}]-epg-[{epg}]/rtp-{rtp}.json")),
            Self::MoCtrl {
                pod,
                node,
                dom,
                af,
                ctrl,
                rtp,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/af-{af}/ctrl-{ctrl}/rtp-{rtp}.json")),
            Self::MoSysCtrl {
                dom,
                af,
                ctrl,
                rtp,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/af-{af}/ctrl-{ctrl}/rtp-{rtp}.json")),
            Self::MoEncapgroupevi {
                pod,
                node,
                encapgroupevi,
                rtp,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/encapgroupevi-{encapgroupevi}/rtp-{rtp}.json")),
            Self::MoSysEncapgroupevi {
                encapgroupevi,
                rtp,
            } => Cow::Owned(format!("mo/sys/bgp/inst/encapgroupevi-{encapgroupevi}/rtp-{rtp}.json")),
        }
    }
}

pub type BgpRtP = AciObject<__internal::BgpRtP>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpRtP;
    impl AciObjectScheme for BgpRtP {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpRtPEndpoint;
        const CLASS_NAME: &'static str = "bgpRtP";
    }
}
