use serde::{Deserialize, Serialize};

use super::lc;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    card_oper_st: String,
    child_action: String,
    descr: String,
    id: String,
    loc: String,
    mod_ts: String,
    mon_pol_dn: String,
    oper_st: String,
    phys_id: String,
    rn: String,
    status: String,
    #[serde(rename = "type")]
    r#type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    EqptLC {
        attributes: lc::Attributes,
        #[serde(default)]
        children: Vec<lc::ChildItem>,
    },
}
