use serde::{Deserialize, Serialize};

use crate::{AciObject, EndpointScheme};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    annotation: String,
    child_action: String,
    ext_mngd_by: String,
    force_resolve: String,
    lc_own: String,
    mod_ts: String,
    mon_pol_dn: String,
    r_type: String,
    rn: String,
    state: String,
    state_qual: String,
    status: String,
    t_cl: String,
    t_dn: String,
    t_type: String,
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

pub type L3extRsL3DomAtt = AciObject<__internal::L3extRsL3DomAtt>;

mod __internal {
    use crate::AciObjectScheme;

    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub struct L3extRsL3DomAtt;

    impl AciObjectScheme for L3extRsL3DomAtt {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "l3extRsL3DomAtt";
    }
}
