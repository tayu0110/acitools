use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::{asic, cpu, dimm, fpga, ind_led, loc_led};

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
    #[serde(rename = "type")]
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
    EqptCPU(cpu::EqptCpu),
    EqptConsP {},
    EqptCpuP {},
    EqptDimm(dimm::EqptDimm),
    EqptEjec {},
    EqptEobcP {},
    EqptEpcP {},
    EqptFlash {},
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
    EqptMgmtP {},
    EqptObfl {},
    EqptRsMonPolModulePolCons {},
    EqptRtSupCOdDiag {},
    EqptSensor {},
    EqptSpromSup {},
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

#[derive(Debug, Clone)]
pub enum Endpoint {
    ClassAll,
    MoUni,
    MoSupslot {
        pod: String,
        node: String,
        supslot: String,
    },
    MoSysSupslot {
        supslot: String,
    },
}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptSupC.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoSupslot { pod, node, supslot } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/supslot-{supslot}/sup.json"
            )),
            Self::MoSysSupslot { supslot } => {
                Cow::Owned(format!("mo/sys/ch/supslot-{supslot}/sup.json"))
            }
        }
    }
}

pub type EqptSupC = AciObject<__internal::EqptSupC>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptSupC;
    impl AciObjectScheme for EqptSupC {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "eqptSupC";
    }
}
