use serde::{Deserialize, Serialize};

use crate::{AciObject, AciObjectScheme, EndpointScheme};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    autoneg: String,
    channel: String,
    child_action: String,
    cos_rewrite: String,
    cts_capable: String,
    duplex: String,
    eee_cap_val: String,
    eee_flap_flags: String,
    eee_wake_times10g: String,
    fcot_capable: String,
    id: String,
    link_debounce: String,
    link_debounce_time: String,
    mdix: String,
    mod_ts: String,
    model: String,
    port_cap: String,
    port_group: String,
    port_grp_mbrs: String,
    proto_support: String,
    qos_rx_prio: String,
    qos_rx_queue: String,
    qos_rx_thold: String,
    qos_tx_prio: String,
    qos_tx_queue: String,
    qos_tx_thold: String,
    rate_mode: String,
    rn: String,
    rx_flow_control: String,
    span: String,
    speed: String,
    speed32: String,
    status: String,
    suppression: String,
    tos_rewrite: String,
    trunk_encap: String,
    tx_flow_control: String,
    #[serde(rename = "type")]
    r#type: String,
    udld: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {}

#[derive(Debug, Clone, Copy)]
pub enum Endpoint {}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> std::borrow::Cow<'_, str> {
        unimplemented!()
    }
}

pub type EthpmPortCap = AciObject<__internal::EthpmPortCap>;

mod __internal {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub struct EthpmPortCap;

    impl AciObjectScheme for EthpmPortCap {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "ethpmPortCap";
    }
}
