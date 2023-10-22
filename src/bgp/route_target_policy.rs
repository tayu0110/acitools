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
pub enum BgpRttPEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoCktepevi {
        pod: String,
        node: String,
        dom: String,
        bdevi: String,
        cktepevi: String,
        rttp: String,
    },
    MoSysCktepevi {
        dom: String,
        bdevi: String,
        cktepevi: String,
        rttp: String,
    },
    MoBdevi {
        pod: String,
        node: String,
        dom: String,
        bdevi: String,
        rttp: String,
    },
    MoSysBdevi {
        dom: String,
        bdevi: String,
        rttp: String,
    },
    MoEvi {
        pod: String,
        node: String,
        dom: String,
        rttp: String,
    },
    MoSysEvi {
        dom: String,
        rttp: String,
    },
    MoEncapgroupevi {
        pod: String,
        node: String,
        encapgroupevi: String,
        rttp: String,
    },
    MoSysEncapgroupevi {
        encapgroupevi: String,
        rttp: String,
    },
    MoAf {
        pod: String,
        node: String,
        dom: String,
        af: String,
        rttp: String,
    },
    MoSysAf {
        dom: String,
        af: String,
        rttp: String,
    },
}

impl EndpointScheme for BgpRttPEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpRttP.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoCktepevi {
                pod,
                node,
                dom,
                bdevi,
                cktepevi,
                rttp,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/bdevi-[{bdevi}]/cktepevi-[{cktepevi}]/rttp-{rttp}.json")),
            Self::MoSysCktepevi {
                dom,
                bdevi,
                cktepevi,
                rttp,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/bdevi-[{bdevi}]/cktepevi-[{cktepevi}]/rttp-{rttp}.json")),
            Self::MoBdevi {
                pod,
                node,
                dom,
                bdevi,
                rttp,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/bdevi-[{bdevi}]/rttp-{rttp}.json")),
            Self::MoSysBdevi {
                dom,
                bdevi,
                rttp,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/bdevi-[{bdevi}]/rttp-{rttp}.json")),
            Self::MoEvi {
                pod,
                node,
                dom,
                rttp,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/evi/rttp-{rttp}.json")),
            Self::MoSysEvi {
                dom,
                rttp,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/evi/rttp-{rttp}.json")),
            Self::MoEncapgroupevi {
                pod,
                node,
                encapgroupevi,
                rttp,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/encapgroupevi-{encapgroupevi}/rttp-{rttp}.json")),
            Self::MoSysEncapgroupevi {
                encapgroupevi,
                rttp,
            } => Cow::Owned(format!("mo/sys/bgp/inst/encapgroupevi-{encapgroupevi}/rttp-{rttp}.json")),
            Self::MoAf {
                pod,
                node,
                dom,
                af,
                rttp,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/af-{af}/rttp-{rttp}.json")),
            Self::MoSysAf {
                dom,
                af,
                rttp,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/af-{af}/rttp-{rttp}.json")),
        }
    }
}

pub type BgpRttP = AciObject<__internal::BgpRttP>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpRttP;
    impl AciObjectScheme for BgpRttP {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpRttPEndpoint;
        const CLASS_NAME: &'static str = "bgpRttP";
    }
}
