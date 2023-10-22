use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lc_own: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rtt: String,
    status: ConfigStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {}

#[derive(Debug, Clone)]
pub enum BgpRttEntryEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoEncapGroupeviVniRtp {
        pod: String,
        node: String,
        encapgroupevi: String,
        vni: String,
        vrf: String,
        bd: String,
        epg: String,
        rtp: String,
        ent: String,
    },
    MoSysEncapGroupeviVniRtp {
        encapgroupevi: String,
        vni: String,
        vrf: String,
        bd: String,
        epg: String,
        rtp: String,
        ent: String,
    },
    MoRtp {
        pod: String,
        node: String,
        dom: String,
        af: String,
        ctrl: String,
        rtp: String,
        ent: String,
    },
    MoSysRtp {
        dom: String,
        af: String,
        ctrl: String,
        rtp: String,
        ent: String,
    },
    MoEncapGroupeviRtp {
        pod: String,
        node: String,
        encapgroupevi: String,
        rtp: String,
        ent: String,
    },
    MoSysEncapGroupeviRtp {
        encapgroupevi: String,
        rtp: String,
        ent: String,
    },
    MoCktepeviRttp {
        pod: String,
        node: String,
        dom: String,
        bdevi: String,
        cktepevi: String,
        rttp: String,
        ent: String,
    },
    MoSysCktepeviRttp {
        dom: String,
        bdevi: String,
        cktepevi: String,
        rttp: String,
        ent: String,
    },
    MoBdeviRttp {
        pod: String,
        node: String,
        dom: String,
        bdevi: String,
        rttp: String,
        ent: String,
    },
    MoSysBdeviRttp {
        dom: String,
        bdevi: String,
        rttp: String,
        ent: String,
    },
    MoRttp {
        pod: String,
        node: String,
        dom: String,
        rttp: String,
        ent: String,
    },
    MoSysRttp {
        dom: String,
        rttp: String,
        ent: String,
    },
    MoEncapGroupeviRttp {
        pod: String,
        node: String,
        encapgroupevi: String,
        rttp: String,
        ent: String,
    },
    MoSysEncapGroupeviRttp {
        encapgroupevi: String,
        rttp: String,
        ent: String,
    },
    MoNodeAfRttp {
        pod: String,
        node: String,
        dom: String,
        af: String,
        rttp: String,
        ent: String,
    },
    MoSysNodeAfRttp {
        dom: String,
        af: String,
        rttp: String,
        ent: String,
    },
}

impl EndpointScheme for BgpRttEntryEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpRttEntry.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoEncapGroupeviVniRtp {
                pod,
                node,
                encapgroupevi,
                vni,
                vrf,
                bd,
                epg,
                rtp,
                ent,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/encapgroupevi-{encapgroupevi}/vni-{vni}-vrf-[{vrf}]-bd-[{bd}]-epg-[{epg}]/rtp-{rtp}/ent-{ent}.json")),
            Self::MoSysEncapGroupeviVniRtp {
                encapgroupevi,
                vni,
                vrf,
                bd,
                epg,
                rtp,
                ent,
            } => Cow::Owned(format!("mo/sys/bgp/inst/encapgroupevi-{encapgroupevi}/vni-{vni}-vrf-[{vrf}]-bd-[{bd}]-epg-[{epg}]/rtp-{rtp}/ent-{ent}.json")),
            Self::MoRtp {
                pod,
                node,
                dom,
                af,
                ctrl,
                rtp,
                ent,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/af-{af}/ctrl-{ctrl}/rtp-{rtp}/ent-{ent}.json")),
            Self::MoSysRtp {
                dom,
                af,
                ctrl,
                rtp,
                ent,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/af-{af}/ctrl-{ctrl}/rtp-{rtp}/ent-{ent}.json")),
            Self::MoEncapGroupeviRtp {
                pod,
                node,
                encapgroupevi,
                rtp,
                ent,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/encapgroupevi-{encapgroupevi}/rtp-{rtp}/ent-{ent}.json")),
            Self::MoSysEncapGroupeviRtp {
                encapgroupevi,
                rtp,
                ent,
            } => Cow::Owned(format!("mo/sys/bgp/inst/encapgroupevi-{encapgroupevi}/rtp-{rtp}/ent-{ent}.json")),
            Self::MoCktepeviRttp {
                pod,
                node,
                dom,
                bdevi,
                cktepevi,
                rttp,
                ent,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/bdevi-[{bdevi}]/cktepevi-[{cktepevi}]/rttp-{rttp}/ent-{ent}.json")),
            Self::MoSysCktepeviRttp {
                dom,
                bdevi,
                cktepevi,
                rttp,
                ent,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/bdevi-[{bdevi}]/cktepevi-[{cktepevi}]/rttp-{rttp}/ent-{ent}.json")),
            Self::MoBdeviRttp {
                pod,
                node,
                dom,
                bdevi,
                rttp,
                ent,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/bdevi-[{bdevi}]/rttp-{rttp}/ent-{ent}.json")),
            Self::MoSysBdeviRttp {
                dom,
                bdevi,
                rttp,
                ent,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/bdevi-[{bdevi}]/rttp-{rttp}/ent-{ent}.json")),
            Self::MoRttp {
                pod,
                node,
                dom,
                rttp,
                ent,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/evi/rttp-{rttp}/ent-{ent}.json")),
            Self::MoSysRttp {
                dom,
                rttp,
                ent,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/evi/rttp-{rttp}/ent-{ent}.json")),
            Self::MoEncapGroupeviRttp {
                pod,
                node,
                encapgroupevi,
                rttp,
                ent,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/encapgroupevi-{encapgroupevi}/rttp-{rttp}/ent-{ent}.json")),
            Self::MoSysEncapGroupeviRttp {
                encapgroupevi,
                rttp,
                ent,
            } => Cow::Owned(format!("mo/sys/bgp/inst/encapgroupevi-{encapgroupevi}/rttp-{rttp}/ent-{ent}.json")),
            Self::MoNodeAfRttp {
                pod,
                node,
                dom,
                af,
                rttp,
                ent,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/af-{af}/rttp-{rttp}/ent-{ent}.json")),
            Self::MoSysNodeAfRttp {
                dom,
                af,
                rttp,
                ent,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/af-{af}/rttp-{rttp}/ent-{ent}.json")),
        }
    }
}

pub type BgpRttEntry = AciObject<__internal::BgpRttEntry>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpRttEntry;
    impl AciObjectScheme for BgpRttEntry {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpRttEntryEndpoint;
        const CLASS_NAME: &'static str = "bgpRttEntry";
    }
}
