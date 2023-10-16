use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    annotation: String,
    area_cost: String,
    area_ctrl: String,
    area_id: String,
    area_type: String,
    child_action: String,
    descr: String,
    ext_mngd_by: String,
    lc_own: String,
    mod_ts: String,
    mon_pol_dn: String,
    multipod_internal: String,
    name: String,
    name_alias: String,
    rn: String,
    status: String,
    uid: String,
    userdom: String,
}
