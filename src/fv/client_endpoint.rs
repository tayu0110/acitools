use crate::{AciObject, ConfigStatus, Configurable, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    annotation: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    base_epg_dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    bd_dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    cont_name: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    encap: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    esg_useg_dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ext_mngd_by: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    fabric_path_dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    hosting_server: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    idepdn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ip: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lc_c: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lc_own: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mac: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mcast_addr: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mon_pol_dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name_alias: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    reporting_controller_name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    uid: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    userdom: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    uuid: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    vmm_src: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    vrf_dn: String,
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
    DhcpEp {},
    FaultCounts {},
    FvIp {},
    FvPathEp {},
    FvPrimaryEncap {},
    FvRsCEpToPathEp {},
    FvRsHyper {},
    FvRsNic {},
    FvRsToEpMacTag {},
    FvRsToNic {},
    FvRsToVm {},
    FvRsVm {},
    FvRtDestToVPort {},
    FvRtFromEp {},
    FvRtFromEpForEpToEpg {},
    FvRtSrcToVPort {},
    FvRtToEp {},
    FvRtToEpForEpToEp {},
    FvRtToEpForEpgToEp {},
    FvRtToEpMacTag {},
    FvRtTrEpDst {},
    FvRtTrEpSrc {},
    HealthInst {},
    TagAnnotation {},
    TagTag {},
    TagTagDef {},
}

#[derive(Debug, Clone)]
pub enum Endpoint {
    ClassAll,
    MoUni,
    MoEsg {
        tenant: String,
        ap: String,
        esg: String,
        ep: String,
    },
    MoTnlepg {
        tenant: String,
        tnlepg: String,
        ep: String,
    },
    MoLdev {
        pri_key: String,
        ctx_dn: String,
        bd_dn: String,
        ep: String,
    },
    MoCtx {
        tenant: String,
        ctx: String,
        ep: String,
    },
    MoEpg {
        tenant: String,
        ap: String,
        epg: String,
        ep: String,
    },
    MoInstP {
        tenant: String,
        l2out: String,
        inst_prof: String,
        ep: String,
    },
}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/fvCEp.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoEsg {
                tenant,
                ap,
                esg,
                ep,
            } => Cow::Owned(format!(
                "mo/uni/tn-{tenant}/ap-{ap}/esg-{esg}/cep-{ep}.json"
            )),
            Self::MoTnlepg { tenant, tnlepg, ep } => {
                Cow::Owned(format!("mo/uni/tn-{tenant}/Tnlepg-{tnlepg}/cep-{ep}.json"))
            }
            Self::MoLdev {
                pri_key,
                ctx_dn,
                bd_dn,
                ep,
            } => Cow::Owned(format!(
                "mo/uni/ldev-[{pri_key}]-ctx-[{ctx_dn}]-bd-[{bd_dn}]/cep-{ep}.json"
            )),
            Self::MoCtx { tenant, ctx, ep } => {
                Cow::Owned(format!("mo/uni/tn-{tenant}/ctx-{ctx}/cep-{ep}.json"))
            }
            Self::MoEpg {
                tenant,
                ap,
                epg,
                ep,
            } => Cow::Owned(format!(
                "mo/uni/tn-{tenant}/ap-{ap}/epg-{epg}/cep-{ep}.json"
            )),
            Self::MoInstP {
                tenant,
                l2out,
                inst_prof,
                ep,
            } => Cow::Owned(format!(
                "mo/uni/tn-{tenant}/l2out-{l2out}/instP-{inst_prof}/cep-{ep}.json"
            )),
        }
    }
}

pub type FvCEp = AciObject<__internal::FvCEp>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct FvCEp;
    impl AciObjectScheme for FvCEp {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "fvCEp";
    }
}
