use serde::{Deserialize, Serialize};

use crate::{AciObject, AciObjectScheme, EndpointScheme};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    actual_type: String,
    aps_stress_env: String,
    bit_encoding: String,
    bit_rate_mbps: String,
    checksum: String,
    child_action: String,
    #[serde(rename = "ciscoPN")]
    cisco_pn: String,
    cisco_pid: String,
    cisco_rev: String,
    #[serde(rename = "ciscoSN")]
    cisco_sn: String,
    cisco_vid: String,
    connect_type: String,
    date_code: String,
    description: String,
    diag_opt_mon_cap: String,
    ext_vendor_specific: String,
    f_c_tx_type: String,
    fcot_num: String,
    fcot_status: String,
    fcot_type: String,
    fibre_type: String,
    five_v_stress_env: String,
    flags: String,
    gig_eth_c_c: String,
    #[serde(rename = "guiCiscoEID")]
    gui_cisco_eid: String,
    #[serde(rename = "guiCiscoPID")]
    gui_cisco_pid: String,
    #[serde(rename = "guiCiscoPN")]
    gui_cisco_pn: String,
    gui_name: String,
    #[serde(rename = "guiPN")]
    gui_pn: String,
    gui_rev: String,
    #[serde(rename = "guiSN")]
    gui_sn: String,
    is_fcot_present: String,
    lot_code: String,
    low_pwr_startup_cap: String,
    max_speed: String,
    min_speed: String,
    mod_ts: String,
    mon_pol_dn: String,
    normal_aps_volt: String,
    package_oui: String,
    part_number: String,
    protocol_type: String,
    range: String,
    reserved: String,
    rn: String,
    sonet_sdh_code: String,
    state: String,
    status: String,
    three_three_v_stress_env: String,
    #[serde(rename = "type")]
    r#type: String,
    type_name: String,
    vendor_name: String,
    vendor_oui: String,
    vendor_pn: String,
    vendor_rev: String,
    vendor_ser_no: String,
    version_id: String,
    wave_len_ch0: String,
    wave_len_ch1: String,
    wave_len_ch2: String,
    wave_len_ch3: String,
    xcvr_type: String,
    xg_eth_code: String,
    xg_fc_code: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    FaultCounts {},
    FaultInst {},
    HealthInst {},
}

#[derive(Debug, Clone, Copy)]
pub enum Endpoint {}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> std::borrow::Cow<'_, str> {
        unimplemented!()
    }
}

pub type EthpmFcotX2 = AciObject<__internal::EthpmFcotX2>;

mod __internal {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub struct EthpmFcotX2;

    impl AciObjectScheme for EthpmFcotX2 {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "ethpmFcotX2";
    }
}
