use crate::{firmware, AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::{
    asic, cpu, dimm, eobc_port, epc_port, flash, fpga, ind_led, loc_led, onboard_failure_log,
    sensor, sprom_lc,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    cimc_version: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    descr: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    hw_ver: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mac_b: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mac_l: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mfg_tm: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    model: String,
    #[allow(dead_code)]
    #[serde(skip_serializing, default)]
    mon_pol_dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    num_p: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    oper_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pwr_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rd_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rev: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ser: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    sw_c_id: String,
    #[serde(rename = "type", skip_serializing_if = "String::is_empty")]
    r#type: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    up_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    v_id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    vendor: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    EqptAsic(asic::EqptAsic),
    EqptCpu(cpu::EqptCpu),
    EqptDimm(dimm::EqptDimm),
    EqptEobcP(eobc_port::EqptEobcP),
    EqptEpcP(epc_port::EqptEpcP),
    EqptFlash(flash::EqptFlash),
    EqptFpga(fpga::EqptFpga),
    EqptFruPower15Min {},
    EqptFruPower1D {},
    EqptFruPower1H {},
    EqptFruPower1Mo {},
    EqptFruPower1Qtr {},
    EqptFruPower1W {},
    EqptFruPower1Year {},
    EqptFruPower5Min {},
    EqptFruPowerHist15Min {},
    EqptFruPowerHist1D {},
    EqptFruPowerHist1H {},
    EqptFruPowerHist1Mo {},
    EqptFruPowerHist1Qtr {},
    EqptFruPowerHist1W {},
    EqptFruPowerHist1Year {},
    EqptFruPowerHist5Min {},
    EqptIndLed(ind_led::EqptIndLed),
    EqptLocLed(loc_led::EqptLocLed),
    EqptObfl(onboard_failure_log::EqptObfl),
    EqptRsMonPolModulePolCons {},
    EqptRtSysCOdDiag {},
    EqptSensor(sensor::EqptSensor),
    EqptSpromLc(sprom_lc::EqptSpromLc),
    FaultCounts {},
    FaultInst {},
    FirmwareCardRunning(firmware::card_running::FirmwareCardRunning),
    HealthInst {},
    TagAliasDelInst {},
    TagAliasInst {},
    TagAnnotation {},
    TagExtMngdInst {},
    TagInst {},
    TagTag {},
}

#[derive(Debug, Clone)]
pub enum EqptSysCEndpoint {
    ClassAll,
    MoUni,
    MoScslot {
        pod: String,
        node: String,
        scslot: String,
    },
    MoSysScslot {
        scslot: String,
    },
}

impl EndpointScheme for EqptSysCEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptSysC.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoScslot { pod, node, scslot } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/scslot-{scslot}/sc.json"
            )),
            Self::MoSysScslot { scslot } => {
                Cow::Owned(format!("mo/sys/ch/scslot-{scslot}/sc.json"))
            }
        }
    }
}

pub type EqptSysC = AciObject<__internal::EqptSysC>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptSysC;
    impl AciObjectScheme for EqptSysC {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptSysCEndpoint;
        const CLASS_NAME: &'static str = "eqptSysC";
    }
}
