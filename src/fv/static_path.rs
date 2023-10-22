use crate::{AciObject, ConfigStatus, Configurable, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    annotation: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    descr: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    encap: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ext_mngd_by: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    force_resolve: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    instr_imedcy: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lc_c: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lc_own: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mode: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mon_pol_dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    primary_encap: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    r_type: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    state: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    state_qual: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    t_cl: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    t_dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    t_type: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    uid: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    userdom: String,
}

impl Configurable for Attributes {
    fn set_status(&mut self, status: ConfigStatus) {
        self.status = status;
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    AaaRbacAnnotation {},
    FaultCounts {},
    FvNlbStaticGroup {},
    HealthInst {},
    IgmpSnoopAccessGroup {},
    IgmpSnoopStaticGroup {},
    L2PortSecurityPol {},
    MldSnoopAccessGroup {},
    MldSnoopStaticGroup {},
    PtpEpgCfg {},
    TagAliasDelInst {},
    TagAliasInst {},
    TagAnnotation {},
    TagExtMngdInst {},
    TagInst {},
    TagTag {},
}

#[derive(Debug, Clone)]
pub enum Endpoint {
    ClassAll,
    MoUni,
    MoEpg {
        tenant: String,
        ap: String,
        epg: String,
        path: String,
    },
}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/fvRsPathAtt.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoEpg {
                tenant,
                ap,
                epg,
                path,
            } => Cow::Owned(format!(
                "mo/uni/tn-{tenant}/ap-{ap}/epg-{epg}/rspathAtt-[{path}].json"
            )),
        }
    }
}

pub type FvRsPathAtt = AciObject<__internal::FvRsPathAtt>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct FvRsPathAtt;
    impl AciObjectScheme for FvRsPathAtt {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "fvRsPathAtt";
    }
}
