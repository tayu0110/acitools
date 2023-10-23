use super::{asic, cpu, dimm, flash, fpga, onboard_failure_log, sensor};
use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    cimc_version: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    descr: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    hw_ver: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    id: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    lc_own: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    mac_b: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    mac_l: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    mfg_tm: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    model: String,
    #[allow(dead_code)]
    #[serde(skip_serializing, default)]
    mon_pol_dn: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    num_p: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    oper_st: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pwr_st: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    rd_st: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    rev: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    ser: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    sw_c_id: String,
    #[serde(rename = "type", skip_serializing_if = "String::is_empty", default)]
    r#type: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    up_ts: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    v_id: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    vendor: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    EqptAsic(asic::EqptAsic),
    #[serde(rename = "eqptCPU")]
    EqptCpu(cpu::EqptCpu),
    EqptDimm(dimm::EqptDimm),
    #[serde(rename = "eqptExtAP")]
    EqptExtAp(serde_json::Value),
    EqptFlash(flash::EqptFlash),
    EqptFpga(fpga::EqptFpga),
    EqptObfl(onboard_failure_log::EqptObfl),
    EqptRsMonPolModulePolCons(serde_json::Value),
    EqptSensor(sensor::EqptSensor),
    FaultCounts(serde_json::Value),
    FaultInst(serde_json::Value),
    HealthInst(serde_json::Value),
    TagAliasDelInst(serde_json::Value),
    TagAliasInst(serde_json::Value),
    TagAnnotation(serde_json::Value),
    TagExtMngdInst(serde_json::Value),
    TagInst(serde_json::Value),
    TagTag(serde_json::Value),
}

#[derive(Debug, Clone)]
pub enum EqptNicEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoNslot {
        pod: String,
        node: String,
        nslot: String,
        nic: String,
    },
    MoSysNslot {
        nslot: String,
        nic: String,
    },
}

impl EndpointScheme for EqptNicEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptNic.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoNslot {
                pod,
                node,
                nslot,
                nic,
            } => Cow::Owned(format!(
                "mo/topology/pod-{pod}/node-{node}/sys/ch/nslot-{nslot}/nic-{nic}.json"
            )),
            Self::MoSysNslot { nslot, nic } => {
                Cow::Owned(format!("mo/sys/ch/nslot-{nslot}/nic-{nic}.json"))
            }
        }
    }
}

pub type EqptNic = AciObject<__internal::EqptNic>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptNic;
    impl AciObjectScheme for EqptNic {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptNicEndpoint;
        const CLASS_NAME: &'static str = "eqptNic";
    }
}
