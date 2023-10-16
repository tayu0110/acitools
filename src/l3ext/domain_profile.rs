use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    annotation: String,
    child_action: String,
    ext_mngd_by: String,
    force_resolve: String,
    lc_own: String,
    mod_ts: String,
    mon_pol_dn: String,
    r_type: String,
    rn: String,
    state: String,
    state_qual: String,
    status: String,
    t_cl: String,
    t_dn: String,
    t_type: String,
    uid: String,
    userdom: String,
}
