use serde::{Deserialize, Serialize};

use super::{adjacency, controller_adjacency};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    admin_rx_st: String,
    admin_st: String,
    admin_tx_st: String,
    child_action: String,
    descr: String,
    id: String,
    lc_own: String,
    mac: String,
    mod_ts: String,
    mon_pol_dn: String,
    name: String,
    oper_rx_st: String,
    oper_tx_st: String,
    port_desc: String,
    port_mode: String,
    port_vlan: String,
    rn: String,
    status: String,
    sys_desc: String,
    wiring_issues: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    L2RsEthIf {},
    LldpAdjEp {
        attributes: adjacency::Attributes,
        #[serde(default)]
        children: Vec<adjacency::ChildItem>,
    },
    LldpCtrlrAdjEp {
        attributes: controller_adjacency::Attributes,
        #[serde(default)]
        children: Vec<controller_adjacency::ChildItem>,
    },
    LldptlvpolUByte {},
    LldptlvpolUInt16 {},
}
