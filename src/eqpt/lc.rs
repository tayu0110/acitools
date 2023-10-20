use crate::{AciObject, EndpointScheme};

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
    // is_lem: String,
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
    EqptAsic {},
    EqptBrkoutP {},
    EqptCPU {},
    EqptDimm {},
    EqptEjec {},
    EqptEobcP {},
    EqptExtChFP {},
    EqptExtChHP {},
    EqptFabP(fabric_port::EqptFabP),
    EqptFcP {},
    EqptFlash {},
    EqptFpga {},
    EqptFruPower15min {},
    EqptFruPower1d {},
    EqptFruPower1h {},
    EqptFruPower1mo {},
    EqptFruPower1qtr {},
    EqptFruPower1w {},
    EqptFruPower1year {},
    EqptFruPower5min {},
    EqptFruPowerHist15min {},
    EqptFruPowerHist1d {},
    EqptFruPowerHist1h {},
    EqptFruPowerHist1mo {},
    EqptFruPowerHist1qtr {},
    EqptFruPowerHist1w {},
    EqptFruPowerHist1year {},
    EqptFruPowerHist5min {},
    EqptIndLed {},
    EqptLeafP(leaf_port::EqptLeafP),
    EqptLocLed {},
    EqptObfl {},
    EqptRsMonPolModulePolCons {},
    EqptRtLcOdDiag {},
    EqptSensor {},
    EqptSpromLc {},
    FaultCounts {},
    FaultInst {},
    FirmwareCardRunning {},
    HealthInst {},
    TagAliasDelInst {},
    TagAliasInst {},
    TagAnnotation {},
    TagExtMngdInst {},
    TagInst {},
    TagTag {},
}

#[derive(Debug, Clone, Copy)]
pub enum Endpoint {}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> std::borrow::Cow<'_, str> {
        unimplemented!()
    }
}

pub type EqptLC = AciObject<__internal::EqptLC>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;

    #[derive(Debug, Clone, Copy)]
    pub struct EqptLC;

    impl AciObjectScheme for EqptLC {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "eqptLC";
    }
}
