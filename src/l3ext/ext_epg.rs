use serde::{Deserialize, Serialize};

use crate::{AciObject, EndpointScheme};

use super::subnet;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    annotation: String,
    child_action: String,
    config_issues: String,
    config_st: String,
    descr: String,
    exception_tag: String,
    ext_mngd_by: String,
    flood_on_encap: String,
    is_shared_srv_msite_e_pg: String,
    lc_own: String,
    match_t: String,
    mcast: String,
    mod_ts: String,
    mon_pol_dn: String,
    name: String,
    name_alias: String,
    pc_enf_pref: String,
    pc_tag: String,
    pc_tag_alloc_src: String,
    pref_gr_memb: String,
    prio: String,
    rn: String,
    scope: String,
    status: String,
    target_dscp: String,
    trigger_st: String,
    tx_id: String,
    uid: String,
    userdom: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    L3extConfigOutDef {},
    L3extSubnet(subnet::L3extSubnet),
    FvRsCustQosPol {},
    FvRsCons {},
    FvRsGraphDef {},
    FvRsProv {},
    FvUpdateContract {},
    L3extRsInstPToProfile {},
    FvRtLIfCtxToInstP {},
    FvRtTermToEPg {},
}

#[derive(Debug, Clone, Copy)]
pub enum Endpoint {}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> std::borrow::Cow<'_, str> {
        unimplemented!()
    }
}

pub type L3extInstP = AciObject<__internal::L3extInstP>;

mod __internal {
    use crate::AciObjectScheme;

    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub struct L3extInstP;

    impl AciObjectScheme for L3extInstP {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "l3extInstP";
    }
}
