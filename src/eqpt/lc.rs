use super::{fabric_port, leaf_port};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    child_action: String,
    cimc_version: String,
    descr: String,
    fw_ver: String,
    hw_ver: String,
    id: String,
    is_lem: String,
    mac_b: String,
    mac_e: String,
    mac_l: String,
    mfg_tm: String,
    mod_ts: String,
    model: String,
    mon_pol_dn: String,
    num_p: String,
    oper_st: String,
    part_number: String,
    pwr_st: String,
    rd_st: String,
    rev: String,
    rn: String,
    ser: String,
    status: String,
    sw_c_id: String,
    sw_ver: String,
    #[serde(rename = "type")]
    r#type: String,
    up_ts: String,
    v_id: String,
    vdr_id: String,
    vendor: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    EqptCPU {},
    EqptDimm {},
    EqptEobcP {},
    EqptFabP {
        attributes: fabric_port::Attributes,
        #[serde(default)]
        children: Vec<fabric_port::ChildItem>,
    },
    EqptFpga {},
    EqptIndLed {},
    EqptLeafP {
        attributes: leaf_port::Attributes,
        #[serde(default)]
        children: Vec<leaf_port::ChildItem>,
    },
    EqptLocLed {},
    EqptObfl {},
}