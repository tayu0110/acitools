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
    encapt: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    role: String,
    status: ConfigStatus,
    #[serde(rename = "type", skip_serializing_if = "String::is_empty")]
    r#type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {}

#[derive(Debug, Clone)]
pub enum BgpDtEpEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoDb {
        pod: String,
        node: String,
        dom: String,
        db: String,
        dtep: String,
    },
    MoSysDb {
        dom: String,
        db: String,
        dtep: String,
    },
}

impl EndpointScheme for BgpDtEpEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpDTEp.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoDb {
                pod,
                node,
                dom,
                db,
                dtep,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/db-{db}/dtep-[{dtep}].json")),
            Self::MoSysDb {
                dom,
                db,
                dtep,
            } => Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/db-{db}/dtep-[{dtep}].json")),
        }
    }
}

pub type BgpDtEp = AciObject<__internal::BgpDtEp>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpDtEp;
    impl AciObjectScheme for BgpDtEp {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpDtEpEndpoint;
        const CLASS_NAME: &'static str = "bgpDTEp";
    }
}
