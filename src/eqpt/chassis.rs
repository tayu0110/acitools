use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{AciObject, EndpointScheme};

use super::lcslot;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    boot_source: String,
    child_action: String,
    cimc_version: String,
    config_role: String,
    descr: String,
    hybrid_mode: String,
    id: String,
    mfg_tm: String,
    mod_ts: String,
    model: String,
    mon_pol_dn: String,
    oper_st: String,
    oper_st_qual: String,
    rev: String,
    rn: String,
    role: String,
    ser: String,
    status: String,
    v_id: String,
    vendor: String,
    virtual_mode: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    EqptBSlot {},
    EqptFCSlot {},
    EqptFlashConfig {},
    EqptFtSlot {},
    EqptIndLed {},
    EqptLCSlot(lcslot::EqptLCSlot),
    EqptLocLed {},
    EqptNSlot {},
    EqptPsuSlot {},
    EqptSpromBP {},
    EqptStorage {},
    EqptSupCSlot {},
    EqptSysCSlot {},
    EqptUsbConfig {},
}

#[derive(Debug, Clone, Copy)]
pub enum Endpoint {}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        unimplemented!()
    }
}

pub type EqptCh = AciObject<__internal::EqptCh>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;

    #[derive(Debug, Clone, Copy)]
    pub struct EqptCh;

    impl AciObjectScheme for EqptCh {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "eqptCh";
    }
}
