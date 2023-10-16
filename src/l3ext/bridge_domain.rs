use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    child_action: String,
    lc_own: String,
    mod_ts: String,
    rn: String,
    status: String,
    t_cl: String,
    t_dn: String,
}
