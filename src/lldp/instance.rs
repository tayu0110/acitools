use serde::{Deserialize, Serialize};

use super::interface;
use crate::{AciObject, EndpointScheme};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    admin_st: String,
    child_action: String,
    ctrl: String,
    hold_time: String,
    infra_vlan: String,
    init_delay_time: String,
    lc_own: String,
    // #[serde(rename = "md5CACert")]
    // md5_ca_cert: String,
    mod_ts: String,
    mon_pol_dn: String,
    name: String,
    oper_err: String,
    opt_tlv_sel: String,
    rn: String,
    status: String,
    sys_desc: String,
    tx_freq: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    FaultCounts {},
    HealthInst {},
    LldpIf(interface::LldpIf),
    LldpInstIfSendTask {},
    LldpInstSendTask {},
    LldpInstStats {},
    LldpMgmtAddr {},
    LldpRsLldpInstPolCons {},
    LldptlvpolComplex {},
    LldptlvpolIp {},
    LldptlvpolMac {},
    LldptlvpolText {},
    LldptlvpolUByte {},
    LldptlvpolUInt16 {},
    LldptlvpolUInt32 {},
    LldptlvpolUInt64 {},
}

#[derive(Debug, Clone, Copy)]
pub enum Endpoint {}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> std::borrow::Cow<'_, str> {
        unimplemented!()
    }
}

pub type LldpInst = AciObject<__internal::LldpInst>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;

    #[derive(Debug, Clone, Copy)]
    pub struct LldpInst;

    impl AciObjectScheme for LldpInst {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "lldpInst";
    }
}
