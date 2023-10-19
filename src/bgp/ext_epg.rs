use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{AciObject, EndpointScheme};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    annotation: String,
    child_action: String,
    descr: String,
    ext_mngd_by: String,
    lc_own: String,
    mod_ts: String,
    mon_pol_dn: String,
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
    fn endpoint(&self) -> Cow<'_, str> {
        unimplemented!()
    }
}

pub type BgpExtP = AciObject<__internal::BgpExtP>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;

    #[derive(Debug, Clone, Copy)]
    pub struct BgpExtP;

    impl AciObjectScheme for BgpExtP {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "bgpExtP";
    }
}
