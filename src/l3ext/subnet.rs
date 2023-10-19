use serde::{Deserialize, Serialize};

use crate::{AciObject, EndpointScheme};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    aggregate: String,
    annotation: String,
    child_action: String,
    descr: String,
    ext_mngd_by: String,
    ip: String,
    lc_own: String,
    mod_ts: String,
    mon_pol_dn: String,
    name: String,
    name_alias: String,
    rn: String,
    scope: String,
    status: String,
    uid: String,
    userdom: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    L3extRsSubnetToRtSumm {},
    L3extRsSubnetToProfile {},
}

#[derive(Debug, Clone, Copy)]
pub enum Endpoint {}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> std::borrow::Cow<'_, str> {
        unimplemented!()
    }
}

pub type L3extSubnet = AciObject<__internal::L3extSubnet>;

mod __internal {
    use crate::AciObjectScheme;

    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub struct L3extSubnet;

    impl AciObjectScheme for L3extSubnet {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "l3extSubnet";
    }
}
