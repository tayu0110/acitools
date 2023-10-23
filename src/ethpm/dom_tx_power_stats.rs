use serde::{Deserialize, Serialize};

use crate::{AciObject, ConfigStatus, EndpointScheme};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    alert: String,
    child_action: String,
    dn: String,
    hi_alarm: String,
    hi_alarm2: String,
    hi_alarm3: String,
    hi_alarm4: String,
    hi_alarm5: String,
    hi_alarm6: String,
    hi_alarm7: String,
    hi_alarm8: String,
    hi_warn: String,
    hi_warn2: String,
    hi_warn3: String,
    hi_warn4: String,
    hi_warn5: String,
    hi_warn6: String,
    hi_warn7: String,
    hi_warn8: String,
    lanes: String,
    lo_alarm: String,
    lo_alarm2: String,
    lo_alarm3: String,
    lo_alarm4: String,
    lo_alarm5: String,
    lo_alarm6: String,
    lo_alarm7: String,
    lo_alarm8: String,
    lo_warn: String,
    lo_warn2: String,
    lo_warn3: String,
    lo_warn4: String,
    lo_warn5: String,
    lo_warn6: String,
    lo_warn7: String,
    lo_warn8: String,
    mod_ts: String,
    #[allow(dead_code)]
    #[serde(skip_serializing, default)]
    mon_pol_dn: String,
    rn: String,
    status: ConfigStatus,
    value: String,
    value2: String,
    value3: String,
    value4: String,
    value5: String,
    value6: String,
    value7: String,
    value8: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    FaultCounts {},
    FaultInst {},
    HealthInst {},
}

#[derive(Debug, Clone, Copy)]
pub enum Endpoint {}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> std::borrow::Cow<'_, str> {
        unimplemented!()
    }
}

pub type EthpmDOMTxPwrStats = AciObject<__internal::EthpmDOMTxPwrStats>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;

    #[derive(Debug, Clone, Copy)]
    pub struct EthpmDOMTxPwrStats;

    impl AciObjectScheme for EthpmDOMTxPwrStats {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "ethpmDOMTxPwrStats";
    }
}
