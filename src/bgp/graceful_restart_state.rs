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
    gr_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    oper_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    restart_intvl: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    status: ConfigStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {}

#[derive(Debug, Clone)]
pub enum BgpGrStEndpoint {
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

impl EndpointScheme for BgpGrStEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpGrSt.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoEnt {
                pod,
                node,
                dom,
                peer,
                ent,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/peer-[{peer}]/ent-[{ent}]/gr.json")),
            Self::MoSysEnt {
                dom,
                peer,
                ent,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/peer-[{peer}]/ent-[{ent}]/gr.json")),
        }
    }
}

pub type BgpGrSt = AciObject<__internal::BgpGrSt>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpGrSt;
    impl AciObjectScheme for BgpGrSt {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpGrStEndpoint;
        const CLASS_NAME: &'static str = "bgpGrSt";
    }
}
