use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    t_context_dn: String,
    t_dn: String,
    t_rn: String,
    t_type: String,
    tn_fv_ctx_name: String,
    uid: String,
    userdom: String,
}
