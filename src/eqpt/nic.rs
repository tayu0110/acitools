use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::{asic, cpu, dimm, flash, fpga, onboard_failure_log, sensor};

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
    lc_own: String,
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
    #[serde(skip_serializing_if = "String::is_empty")]
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
    EqptExtAp {},
    EqptFlash(flash::EqptFlash),
    EqptFpga(fpga::EqptFpga),
    EqptObfl(onboard_failure_log::EqptObfl),
    EqptRsMonPolModulePolCons {},
    EqptSensor(sensor::EqptSensor),
    FaultCounts {},
    FaultInst {},
    HealthInst {},
    TagAliasDelInst {},
    TagAliasInst {},
    TagAnnotation {},
    TagExtMngdInst {},
    TagInst {},
    TagTag {},
}

#[derive(Debug, Clone)]
pub enum EqptNicEndpoint {
    ClassAll,
    MoUni,
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
