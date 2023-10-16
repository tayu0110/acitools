use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FabricPod {
    FabricPod { attributes: Attributes },
}

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
