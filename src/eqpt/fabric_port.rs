use crate::{AciObject, EndpointScheme};

use super::port;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    child_action: String,
    descr: String,
    flags: String,
    global_port: String,
    id: String,
    is_lem: String,
    mod_ts: String,
    mon_pol_dn: String,
    rn: String,
    speed: String,
    status: String,
    #[serde(rename = "type")]
    r#type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    EqptLPort(port::EqptLPort),
    EqptRsIoPPhysConf {},
    DbgRemotePort {},
}

#[derive(Debug, Clone, Copy)]
pub enum Endpoint {}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> std::borrow::Cow<'_, str> {
        unimplemented!()
    }
}

pub type EqptFabP = AciObject<__internal::EqptFabP>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;

    #[derive(Debug, Clone, Copy)]
    pub struct EqptFabP;

    impl AciObjectScheme for EqptFabP {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "eqptFabP";
    }
}
