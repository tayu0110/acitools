use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    access_vlan: String,
    allowed_vlans: String,
    backplane_mac: String,
    bundle_bup_id: String,
    bundle_index: String,
    cfg_access_vlan: String,
    cfg_native_vlan: String,
    child_action: String,
    curr_err_index: String,
    diags: String,
    encap: String,
    err_dis_timer_running: String,
    err_vlan_status_ht: String,
    err_vlans: String,
    hw_bd_id: String,
    hw_resource_id: String,
    intf_t: String,
    iod: String,
    last_errors: String,
    last_link_st_chg: String,
    media: String,
    mod_ts: String,
    mon_pol_dn: String,
    native_vlan: String,
    #[serde(rename = "numOfSI")]
    num_of_si: String,
    oper_bitset: String,
    oper_dce_mode: String,
    oper_duplex: String,
    #[serde(rename = "operEEERxWkTime")]
    oper_eee_rx_wk_time: String,
    #[serde(rename = "operEEEState")]
    oper_eee_state: String,
    #[serde(rename = "operEEETxWkTime")]
    oper_eee_tx_wk_time: String,
    oper_err_dis_qual: String,
    oper_fec_mode: String,
    oper_flow_ctrl: String,
    oper_mdix: String,
    oper_mode: String,
    oper_mode_detail: String,
    oper_phy_en_st: String,
    oper_router_mac: String,
    oper_speed: String,
    oper_st: String,
    oper_st_qual: String,
    oper_st_qual_code: String,
    oper_vlans: String,
    os_sum: String,
    port_cfg_wait_flags: String,
    primary_vlan: String,
    reset_ctr: String,
    rn: String,
    si_list: String,
    status: String,
    tx_t: String,
    usage: String,
    user_cfgd_flags: String,
    vdc_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    EthpmFcot {},
    EthpmFcotX2 {},
    EthpmPortCap {},
}
