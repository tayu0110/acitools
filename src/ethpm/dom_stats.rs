use super::{
    dom_current_stats, dom_rx_power_stats, dom_temp_stats, dom_tx_power_stats, dom_volt_stats,
};
use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    alerts: String,
    child_action: String,
    #[serde(default)]
    dn: String,
    mod_ts: String,
    #[allow(dead_code)]
    #[serde(skip_serializing, default)]
    mon_pol_dn: String,
    rn: String,
    rx_los: String,
    status: ConfigStatus,
    tx_fault_count: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    EthpmDOMCurrentStats(dom_current_stats::EthpmDOMCurrentStats),
    EthpmDOMRxPwrStats(dom_rx_power_stats::EthpmDOMRxPwrStats),
    EthpmDOMTempStats(dom_temp_stats::EthpmDOMTempStats),
    EthpmDOMTxPwrStats(dom_tx_power_stats::EthpmDOMTxPwrStats),
    EthpmDOMVoltStats(dom_volt_stats::EthpmDOMVoltStats),
    FaultCounts {},
    HealthInst {},
}

#[derive(Debug, Clone)]
pub enum Endpoint {}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> std::borrow::Cow<'_, str> {
        unimplemented!()
    }
}

pub type EthpmDOMStats = AciObject<__internal::EthpmDOMStats>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;

    #[derive(Debug, Clone, Copy)]
    pub struct EthpmDOMStats;

    impl AciObjectScheme for EthpmDOMStats {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "ethpmDOMStats";
    }
}
