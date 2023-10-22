use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::path;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pfx: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rd: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rt_flags: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    ver: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    BgpPath(path::BgpPath),
}

#[derive(Debug, Clone)]
pub enum BgpRouteEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoAf {
        pod: String,
        node: String,
        dom: String,
        af: String,
        rt: String,
    },
    MoSysAf {
        dom: String,
        af: String,
        rt: String,
    },
}

impl EndpointScheme for BgpRouteEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpRoute.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoAf {
                pod,
                node,
                dom,
                af,
                rt,
            } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/af-{af}/rt-[{rt}].json"
            )),
            Self::MoSysAf { dom, af, rt } => {
                Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/af-{af}/rt-[{rt}].json"))
            }
        }
    }
}

pub type BgpRoute = AciObject<__internal::BgpRoute>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpRoute;
    impl AciObjectScheme for BgpRoute {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpRouteEndpoint;
        const CLASS_NAME: &'static str = "bgpRoute";
    }
}
