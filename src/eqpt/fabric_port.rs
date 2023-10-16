use super::port;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    child_action: String,
    descr: String,
    flags: String,
    global_port: String,
    id: String,
    is_lem: String,
    mod_ts: String,
    mon_pol_dn: String,
    rn: String,
    speed: String,
    status: String,
    #[serde(rename = "type")]
    r#type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    EqptLPort {
        attributes: port::Attributes,
        #[serde(default)]
        children: Vec<port::ChildItem>,
    },
    EqptRsIoPPhysConf {},
    DbgRemotePort {},
}
