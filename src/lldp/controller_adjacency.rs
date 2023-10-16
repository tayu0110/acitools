use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    apic_mode: String,
    auth_cookie: String,
    child_action: String,
    id: String,
    infra_vlan: String,
    ip: String,
    lc_own: String,
    mac: String,
    mod_ts: String,
    mon_pol_dn: String,
    port_role: String,
    rn: String,
    status: String,
    verified: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    LldpRsCtrlrAdjEpToStAdjEp {},
}
