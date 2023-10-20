use serde::{Deserialize, Serialize};

use crate::{AciObject, ConfigStatus, EndpointScheme};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    alert: String,
    child_action: String,
    dn: String,
    hi_alarm: String,
    hi_warn: String,
    lanes: String,
    lo_alarm: String,
    lo_warn: String,
    mod_ts: String,
    rn: String,
    status: ConfigStatus,
    value: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum ChildItem {}

#[derive(Debug, Clone, Copy)]
pub enum Endpoint {}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> std::borrow::Cow<'_, str> {
        unimplemented!()
    }
}

pub type EthpmDOMVoltStats = AciObject<__internal::EthpmDOMVoltStats>;

mod __internal {
    use crate::AciObjectScheme;

    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub struct EthpmDOMVoltStats;

    impl AciObjectScheme for EthpmDOMVoltStats {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "ethpmDOMVoltStats";
    }
}
