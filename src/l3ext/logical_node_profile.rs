use serde::{Deserialize, Serialize};

use super::logical_if_profile;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    annotation: String,
    child_action: String,
    config_issues: String,
    descr: String,
    ext_mngd_by: String,
    lc_own: String,
    mod_ts: String,
    mon_pol_dn: String,
    name: String,
    name_alias: String,
    owner_key: String,
    owner_tag: String,
    rn: String,
    status: String,
    tag: String,
    target_dscp: String,
    uid: String,
    userdom: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    L3extRsNodeL3OutAtt {},
    L3extLIfP {
        attributes: logical_if_profile::Attributes,
        #[serde(default)]
        children: Vec<logical_if_profile::ChildItem>,
    },
}
