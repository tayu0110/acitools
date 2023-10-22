use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ctrl: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lc_own: String,
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
pub enum BgpPathCtrlEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoDom {
        pod: String,
        node: String,
        dom: String,
    },
    MoSysDom {
        dom: String,
    },
}

impl EndpointScheme for BgpPathCtrlEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/bgpPathCtrl.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoDom { pod, node, dom } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/bgp/inst/dom-{dom}/pathctrl.json"
            )),
            Self::MoSysDom { dom } => {
                Cow::Owned(format!("mo/sys/bgp/inst/dom-{dom}/pathctrl.json"))
            }
        }
    }
}

pub type BgpPathCtrl = AciObject<__internal::BgpPathCtrl>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct BgpPathCtrl;
    impl AciObjectScheme for BgpPathCtrl {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = BgpPathCtrlEndpoint;
        const CLASS_NAME: &'static str = "bgpPathCtrl";
    }
}
