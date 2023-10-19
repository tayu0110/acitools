use serde::{Deserialize, Serialize};

use crate::{AciObject, EndpointScheme};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    child_action: String,
    dn: String,
    id: String,
    lc_own: String,
    mod_ts: String,
    mon_pol_dn: String,
    pod_type: String,
    status: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {}

#[derive(Debug, Clone, Copy)]
pub enum FabricPodEndpoint {
    ClassAll,
    Mo { id: u32 },
}

impl EndpointScheme for FabricPodEndpoint {
    fn endpoint(&self) -> std::borrow::Cow<'_, str> {
        match self {
            Self::ClassAll => std::borrow::Cow::Borrowed("node/class/fabricPod.json"),
            Self::Mo { id } => std::borrow::Cow::Owned(format!("node/mo/topology/pod-{id}.json")),
        }
    }
}

pub type FabricPod = AciObject<__internal::FabricPod>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;

    #[derive(Debug, Clone, Copy)]
    pub struct FabricPod;

    impl AciObjectScheme for FabricPod {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = FabricPodEndpoint;
        const CLASS_NAME: &'static str = "fabricPod";
    }
}
