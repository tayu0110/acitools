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
    e_dist: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    i_dist: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lc_own: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    local_dist: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    status: ConfigStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {}

#[derive(Debug, Clone)]
pub enum BgpAdminDistEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoAf {
        pod: String,
        node: String,
        dom: String,
        af: String,
    },
    MoSysAf {
        dom: String,
        af: String,
    },
}

impl EndpointScheme for BgpAdminDistEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpAdminDist.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoAf { pod, node, dom, af } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/af-{af}/adminDist.json"
            )),
            Self::MoSysAf { dom, af } => {
                Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/af-{af}/adminDist.json"))
            }
        }
    }
}

pub type BgpAdminDist = AciObject<__internal::BgpAdminDist>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpAdminDist;
    impl AciObjectScheme for BgpAdminDist {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpAdminDistEndpoint;
        const CLASS_NAME: &'static str = "bgpAdminDist";
    }
}
