use crate::{AciObject, EndpointScheme};

use super::lc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    card_oper_st: String,
    child_action: String,
    descr: String,
    id: String,
    loc: String,
    mod_ts: String,
    mon_pol_dn: String,
    oper_st: String,
    phys_id: String,
    rn: String,
    status: String,
    #[serde(rename = "type")]
    r#type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    EqptLC(lc::EqptLC),
}

#[derive(Debug, Clone, Copy)]
pub enum Endpoint {}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> std::borrow::Cow<'_, str> {
        unimplemented!()
    }
}

pub type EqptLCSlot = AciObject<__internal::EqptLCSlot>;

mod __internal {
    use crate::AciObjectScheme;

    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub struct EqptLCSlot;

    impl AciObjectScheme for EqptLCSlot {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "eqptLCSlot";
    }
}
