use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::discovered_tep;

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
    rn: String,
    status: ConfigStatus,
    #[serde(rename = "type", skip_serializing_if = "String::is_empty")]
    r#type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    BgpDTEp(discovered_tep::BgpDtEp),
}

#[derive(Debug, Clone)]
pub enum BgpDbEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoDom {
        pod: String,
        node: String,
        dom: String,
        db: String,
    },
    MoSysDom {
        dom: String,
        db: String,
    },
}

impl EndpointScheme for BgpDbEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpDb.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoDom { pod, node, dom, db } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/db-{db}.json"
            )),
            Self::MoSysDom { dom, db } => {
                Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/db-{db}.json"))
            }
        }
    }
}

pub type BgpDb = AciObject<__internal::BgpDb>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpDb;
    impl AciObjectScheme for BgpDb {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpDbEndpoint;
        const CLASS_NAME: &'static str = "bgpDb";
    }
}
