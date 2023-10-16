use serde::{Deserialize, Serialize};

use super::lcslot;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    boot_source: String,
    child_action: String,
    cimc_version: String,
    config_role: String,
    descr: String,
    hybrid_mode: String,
    id: String,
    mfg_tm: String,
    mod_ts: String,
    model: String,
    mon_pol_dn: String,
    oper_st: String,
    oper_st_qual: String,
    rev: String,
    rn: String,
    role: String,
    ser: String,
    status: String,
    v_id: String,
    vendor: String,
    virtual_mode: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    EqptBSlot {},
    EqptFCSlot {},
    EqptFlashConfig {},
    EqptFtSlot {},
    EqptLCSlot {
        attributes: lcslot::Attributes,
        #[serde(default)]
        children: Vec<lcslot::ChildItem>,
    },
    EqptLocLed {},
    EqptNSlot {},
    EqptPsuSlot {},
    EqptStorage {},
    EqptSupCSlot {},
    EqptSysCSlot {},
    EqptUsbConfig {},
}
