use serde::{Deserialize, Serialize};

use crate::{AciObject, EndpointScheme};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    annotation: String,
    area_cost: String,
    area_ctrl: String,
    area_id: String,
    area_type: String,
    child_action: String,
    descr: String,
    ext_mngd_by: String,
    lc_own: String,
    mod_ts: String,
    mon_pol_dn: String,
    multipod_internal: String,
    name: String,
    name_alias: String,
    rn: String,
    status: String,
    uid: String,
    userdom: String,
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

pub type OspfExtP = AciObject<__internal::OspfExtP>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;

    #[derive(Debug, Clone, Copy)]
    pub struct OspfExtP;

    impl AciObjectScheme for OspfExtP {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "ospfExtP";
    }
}
