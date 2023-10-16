use serde::{Deserialize, Serialize};

use super::instance;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    admin_st: String,
    child_action: String,
    lc_own: String,
    mod_ts: String,
    mon_pol_dn: String,
    name: String,
    oper_err: String,
    oper_st: String,
    rn: String,
    status: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    LldpInst {
        attributes: instance::Attributes,
        #[serde(default)]
        children: Vec<instance::ChildItem>,
    },
}
