use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    annotation: String,
    auto_continue: String,
    child_action: String,
    descr: String,
    direction: String,
    ext_mngd_by: String,
    lc_own: String,
    level: String,
    mod_ts: String,
    mon_pol_dn: String,
    name: String,
    name_alias: String,
    owner_key: String,
    owner_tag: String,
    rn: String,
    status: String,
    #[serde(rename = "type")]
    r#type: String,
    uid: String,
    userdom: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    RtctrlRtSubnetToProfile {},
}
