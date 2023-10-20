use serde::{Deserialize, Serialize};

use crate::{AciObject, AciObjectScheme, EndpointScheme};

use super::instance;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    admin_st: String,
    child_action: String,
    lc_own: String,
    mod_ts: String,
    mon_pol_dn: String,
    name: String,
    oper_err: String,
    oper_st: String,
    rn: String,
    status: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    FaultCounts {},
    HealthInst {},
    LldpInst(instance::LldpInst),
}

#[derive(Debug, Clone, Copy)]
pub enum Endpoint {}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> std::borrow::Cow<'_, str> {
        unimplemented!()
    }
}

pub type LldpEntity = AciObject<__internal::LldpEntity>;

mod __internal {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub struct LldpEntity;

    impl AciObjectScheme for LldpEntity {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "lldpEntity";
    }
}
