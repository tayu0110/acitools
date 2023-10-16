use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    aggregate: String,
    annotation: String,
    child_action: String,
    descr: String,
    ext_mngd_by: String,
    ip: String,
    lc_own: String,
    mod_ts: String,
    mon_pol_dn: String,
    name: String,
    name_alias: String,
    rn: String,
    scope: String,
    status: String,
    uid: String,
    userdom: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    L3extRsSubnetToRtSumm {},
    L3extRsSubnetToProfile {},
}
