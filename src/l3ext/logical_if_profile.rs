use serde::{Deserialize, Serialize};

use crate::{AciObject, EndpointScheme};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    addr: String,
    annotation: String,
    child_action: String,
    descr: String,
    encap: String,
    ext_mngd_by: String,
    lc_own: String,
    mod_ts: String,
    mon_pol_dn: String,
    name: String,
    name_alias: String,
    owner_key: String,
    owner_tag: String,
    prio: String,
    rn: String,
    status: String,
    tag: String,
    target_dscp: String,
    uid: String,
    userdom: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    BfdIfP {},
    EigrpIfP {},
    L3extRsArpIfPol {},
    L3extRsEgressQosDppPol {},
    L3extRsIngressQosDppPol {},
    L3extRsLIfPCustQosPol {},
    L3extRsNdIfPol {},
    L3extRsPathL3OutAtt {},
    OspfIfP {},
}

#[derive(Debug, Clone, Copy)]
pub enum Endpoint {}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> std::borrow::Cow<'_, str> {
        unimplemented!()
    }
}

pub type L3extLIfP = AciObject<__internal::L3extLIfP>;

mod __internal {
    use crate::AciObjectScheme;

    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub struct L3extLIfP;

    impl AciObjectScheme for L3extLIfP {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "l3extLIfP";
    }
}
