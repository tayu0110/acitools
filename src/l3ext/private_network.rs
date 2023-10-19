use serde::{Deserialize, Serialize};

use crate::{AciObject, EndpointScheme};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    t_context_dn: String,
    t_dn: String,
    t_rn: String,
    t_type: String,
    tn_fv_ctx_name: String,
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

pub type L3extRsEctx = AciObject<__internal::L3extRsEctx>;

mod __internal {
    use crate::AciObjectScheme;

    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub struct L3extRsEctx;

    impl AciObjectScheme for L3extRsEctx {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "l3extRsEctx";
    }
}
