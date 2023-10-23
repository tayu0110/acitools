use serde::{Deserialize, Serialize};

use crate::{AciObject, EndpointScheme};

use super::{adjacency, controller_adjacency};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    admin_rx_st: String,
    admin_st: String,
    admin_tx_st: String,
    child_action: String,
    descr: String,
    id: String,
    lc_own: String,
    mac: String,
    mod_ts: String,
    #[allow(dead_code)]
    #[serde(skip_serializing, default)]
    mon_pol_dn: String,
    name: String,
    oper_rx_st: String,
    oper_tx_st: String,
    port_desc: String,
    port_mode: String,
    port_vlan: String,
    rn: String,
    status: String,
    sys_desc: String,
    wiring_issues: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    DcbxIfCtx {},
    FaultCounts {},
    FaultInst {},
    HealthInst {},
    L2RsEthIf {},
    L2RsMgmtIf {},
    LldpAdjEp(adjacency::LldpAdjEp),
    LldpCtrlrAdjEp(controller_adjacency::LldpCtrlrAdjEp),
    LldpIfSendTask {},
    LldpIfStats {},
    LldpInvalidAciAdjEp {},
    LldptlvpolComplex {},
    LldptlvpolIp {},
    LldptlvpolMac {},
    LldptlvpolText {},
    LldptlvpolUByte {},
    LldptlvpolUInt16 {},
    LldptlvpolUInt32 {},
    LldptlvpolUInt64 {},
    NwRtPathToIf {},
}

#[derive(Debug, Clone, Copy)]
pub enum Endpoint {}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> std::borrow::Cow<'_, str> {
        unimplemented!()
    }
}

pub type LldpIf = AciObject<__internal::LldpIf>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;

    #[derive(Debug, Clone, Copy)]
    pub struct LldpIf;

    impl AciObjectScheme for LldpIf {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "lldpIf";
    }
}
