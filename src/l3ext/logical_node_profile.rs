use serde::{Deserialize, Serialize};

use crate::{AciObject, EndpointScheme};

use super::logical_if_profile;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    annotation: String,
    child_action: String,
    config_issues: String,
    descr: String,
    ext_mngd_by: String,
    lc_own: String,
    mod_ts: String,
    mon_pol_dn: String,
    name: String,
    name_alias: String,
    owner_key: String,
    owner_tag: String,
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
    L3extRsNodeL3OutAtt {},
    L3extLIfP(logical_if_profile::L3extLIfP),
    BgpPeerP {},
    BgpProtP {},
}

#[derive(Debug, Clone, Copy)]
pub enum Endpoint {}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> std::borrow::Cow<'_, str> {
        unimplemented!()
    }
}

pub type L3extLNodeP = AciObject<__internal::L3extLNodeP>;

mod __internal {
    use crate::AciObjectScheme;

    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub struct L3extLNodeP;

    impl AciObjectScheme for L3extLNodeP {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "l3extLNodeP";
    }
}
