use serde::{Deserialize, Serialize};

use crate::{AciObject, EndpointScheme};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    apic_mode: String,
    auth_cookie: String,
    child_action: String,
    id: String,
    infra_vlan: String,
    ip: String,
    lc_own: String,
    mac: String,
    mod_ts: String,
    mon_pol_dn: String,
    port_role: String,
    rn: String,
    status: String,
    verified: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    FaultCounts {},
    FaultInst {},
    HealthInst {},
    LldpRsCtrlrAdjEpToStAdjEp {},
}

#[derive(Debug, Clone, Copy)]
pub enum Endpoint {}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> std::borrow::Cow<'_, str> {
        unimplemented!()
    }
}

pub type LldpCtrlrAdjEp = AciObject<__internal::LldpCtrlrAdjEp>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;

    #[derive(Debug, Clone, Copy)]
    pub struct LldpCtrlrAdjEp;

    impl AciObjectScheme for LldpCtrlrAdjEp {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "lldpCtrlrAdjEp";
    }
}
