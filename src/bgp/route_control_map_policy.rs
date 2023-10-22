use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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
    lc_own: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name_alias: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rt_map: String,
    status: ConfigStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    FaultDelegate {},
}

#[derive(Debug, Clone)]
pub enum BgpRtCtrlMapPEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoEncapGroupeviRtp {
        pod: String,
        node: String,
        encapgroupevi: String,
        vni: String,
        vrf: String,
        bd: String,
        epg: String,
        rtp: String,
    },
    MoSysEncapGroupeviVniRtp {
        encapgroupevi: String,
        vni: String,
        vrf: String,
        bd: String,
        epg: String,
        rtp: String,
    },
    MoRtp {
        pod: String,
        node: String,
        dom: String,
        af: String,
        ctrl: String,
        rtp: String,
    },
    MoSysRtp {
        dom: String,
        af: String,
        ctrl: String,
        rtp: String,
    },
    MoNodeEncapGroupeviRtp {
        pod: String,
        node: String,
        encapgroupevi: String,
        rtp: String,
    },
    MoSysEncapGroupeviRtp {
        encapgroupevi: String,
        rtp: String,
    },
    MoNodeCktepeviRttp {
        pod: String,
        node: String,
        dom: String,
        bdevi: String,
        cktepevi: String,
        rttp: String,
    },
    MoSysCktepeviRttp {
        dom: String,
        bdevi: String,
        cktepevi: String,
        rttp: String,
    },
    MoNodeBdeviRttp {
        pod: String,
        node: String,
        dom: String,
        bdevi: String,
        rttp: String,
    },
    MoSysBdeviRttp {
        dom: String,
        bdevi: String,
        rttp: String,
    },
    MoRttp {
        pod: String,
        node: String,
        dom: String,
        rttp: String,
    },
    MoSysRttp {
        dom: String,
        rttp: String,
    },
    MoEncapGroupeviRttp {
        pod: String,
        node: String,
        encapgroupevi: String,
        rttp: String,
    },
    MoSysEncapGroupeviRttp {
        encapgroupevi: String,
        rttp: String,
    },
    MoNodeAfRttp {
        pod: String,
        node: String,
        dom: String,
        af: String,
        rttp: String,
    },
    MoSysAfRttp {
        dom: String,
        af: String,
        rttp: String,
    },
}

impl EndpointScheme for BgpRtCtrlMapPEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpRtCtrlMapP.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoEncapGroupeviRtp {
                pod,
                node,
                encapgroupevi,
                vni,
                vrf,
                bd,
                epg,
                rtp,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/encapgroupevi-{encapgroupevi}/vni-{vni}-vrf-[{vrf}]-bd-[{bd}]-epg-[{epg}]/rtp-{rtp}/rtctrlmap.json")),
            Self::MoSysEncapGroupeviVniRtp {
                encapgroupevi,
                vni,
                vrf,
                bd,
                epg,
                rtp,
            } => Cow::Owned(format!("mo/sys/bgp/inst/encapgroupevi-{encapgroupevi}/vni-{vni}-vrf-[{vrf}]-bd-[{bd}]-epg-[{epg}]/rtp-{rtp}/rtctrlmap.json")),
            Self::MoRtp {
                pod,
                node,
                dom,
                af,
                ctrl,
                rtp,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/af-{af}/ctrl-{ctrl}/rtp-{rtp}/rtctrlmap.json")),
            Self::MoSysRtp {
                dom,
                af,
                ctrl,
                rtp,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/af-{af}/ctrl-{ctrl}/rtp-{rtp}/rtctrlmap.json")),
            Self::MoNodeEncapGroupeviRtp {
                pod,
                node,
                encapgroupevi,
                rtp,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/encapgroupevi-{encapgroupevi}/rtp-{rtp}/rtctrlmap.json")),
            Self::MoSysEncapGroupeviRtp {
                encapgroupevi,
                rtp,
            } => Cow::Owned(format!("mo/sys/bgp/inst/encapgroupevi-{encapgroupevi}/rtp-{rtp}/rtctrlmap.json")),
            Self::MoNodeCktepeviRttp {
                pod,
                node,
                dom,
                bdevi,
                cktepevi,
                rttp,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/bdevi-[{bdevi}]/cktepevi-[{cktepevi}]/rttp-{rttp}/rtctrlmap.json")),
            Self::MoSysCktepeviRttp {
                dom,
                bdevi,
                cktepevi,
                rttp,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/bdevi-[{bdevi}]/cktepevi-[{cktepevi}]/rttp-{rttp}/rtctrlmap.json")),
            Self::MoNodeBdeviRttp {
                pod,
                node,
                dom,
                bdevi,
                rttp,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/bdevi-[{bdevi}]/rttp-{rttp}/rtctrlmap.json")),
            Self::MoSysBdeviRttp {
                dom,
                bdevi,
                rttp,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/bdevi-[{bdevi}]/rttp-{rttp}/rtctrlmap.json")),
            Self::MoRttp {
                pod,
                node,
                dom,
                rttp,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/evi/rttp-{rttp}/rtctrlmap.json")),
            Self::MoSysRttp {
                dom,
                rttp,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/evi/rttp-{rttp}/rtctrlmap.json")),
            Self::MoEncapGroupeviRttp {
                pod,
                node,
                encapgroupevi,
                rttp,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/encapgroupevi-{encapgroupevi}/rttp-{rttp}/rtctrlmap.json")),
            Self::MoSysEncapGroupeviRttp {
                encapgroupevi,
                rttp,
            } => Cow::Owned(format!("mo/sys/bgp/inst/encapgroupevi-{encapgroupevi}/rttp-{rttp}/rtctrlmap.json")),
            Self::MoNodeAfRttp {
                pod,
                node,
                dom,
                af,
                rttp,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/af-{af}/rttp-{rttp}/rtctrlmap.json")),
            Self::MoSysAfRttp {
                dom,
                af,
                rttp,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/af-{af}/rttp-{rttp}/rtctrlmap.json")),
        }
    }
}

pub type BgpRtCtrlMapP = AciObject<__internal::BgpRtCtrlMapP>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpRtCtrlMapP;
    impl AciObjectScheme for BgpRtCtrlMapP {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpRtCtrlMapPEndpoint;
        const CLASS_NAME: &'static str = "bgpRtCtrlMapP";
    }
}
