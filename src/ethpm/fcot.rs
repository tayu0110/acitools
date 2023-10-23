use serde::{Deserialize, Serialize};

use crate::{AciObject, AciObjectScheme, EndpointScheme};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    actual_type: String,
    base_resvd1: String,
    base_resvd2: String,
    base_resvd3: String,
    base_resvd4: String,
    #[serde(rename = "brIn100MHz")]
    br_in100mhz: String,
    br_max_margin: String,
    br_min_margin: String,
    ccex: String,
    ccid: String,
    child_action: String,
    connect_type: String,
    date_code: String,
    description: String,
    diag_mon_type: String,
    dist_in100m_for9u: String,
    dist_in10m_for50u: String,
    dist_in10m_for60u: String,
    dist_in1m_for_cu: String,
    dist_in_km_for9u: String,
    encoding: String,
    enh_option: String,
    ext_option: String,
    f_c_tx_type: String,
    fcot_num: String,
    fcot_status: String,
    fcot_type: String,
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
    max_speed: String,
    min_speed: String,
    mod_ts: String,
    #[allow(dead_code)]
    #[serde(skip_serializing, default)]
    mon_pol_dn: String,
    part_number: String,
    rn: String,
    sff8472_compl: String,
    state: String,
    status: String,
    #[serde(rename = "type")]
    r#type: String,
    type_name: String,
    vendor_data: String,
    vendor_id: String,
    vendor_name: String,
    vendor_pn: String,
    vendor_rev: String,
    vendor_sn: String,
    version_id: String,
    xcvr_code: String,
    xcvr_ext_id: String,
    xcvr_id: String,
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
        unimplemented!();
    }
}

pub type EthpmFcot = AciObject<__internal::EthpmFcot>;

mod __internal {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub struct EthpmFcot;

    impl AciObjectScheme for EthpmFcot {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "ethpmFcot";
    }
}
