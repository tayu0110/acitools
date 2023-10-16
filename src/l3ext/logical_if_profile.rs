use serde::{Deserialize, Serialize};

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
    OspfIfP {},
    L3extRsPathL3OutAtt {},
    L3extRsNdIfPol {},
    L3extRsLIfPCustQosPol {},
    L3extRsIngressQosDppPol {},
    L3extRsEgressQosDppPol {},
    L3extRsArpIfPol {},
    BfdIfP {},
}
