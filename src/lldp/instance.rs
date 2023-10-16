use serde::{Deserialize, Serialize};

use super::interface;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    admin_st: String,
    child_action: String,
    ctrl: String,
    hold_time: String,
    infra_vlan: String,
    init_delay_time: String,
    lc_own: String,
    #[serde(rename = "md5CACert")]
    md5_ca_cert: String,
    mod_ts: String,
    mon_pol_dn: String,
    name: String,
    oper_err: String,
    opt_tlv_sel: String,
    rn: String,
    status: String,
    sys_desc: String,
    tx_freq: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    LldpIf {
        attributes: interface::Attributes,
        #[serde(default)]
        children: Vec<interface::ChildItem>,
    },
    LldpInstIfSendTask {},
    LldptlvpolComplex {},
    LldpInstSendTask {},
    LldptlvpolText {},
    LldpRsLldpInstPolCons {},
    LldptlvpolIp {},
    LldptlvpolUByte {},
    LldptlvpolUInt16 {},
    LldptlvpolUInt32 {},
}
