use super::{client_endpoint, static_path, subnet};
use crate::{AciObject, ConfigStatus, Configurable, EndpointScheme};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    annotation: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing)]
    config_issues: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    config_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    descr: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    exception_tag: String,
    #[serde(skip_serializing)]
    ext_mngd_by: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    flood_on_encap: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    fwd_ctrl: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    has_mcast_source: String,
    #[serde(skip_serializing)]
    is_attr_based_epg: String,
    #[serde(skip_serializing)]
    is_shared_srv_msite_e_pg: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lc_own: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    match_t: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing)]
    mon_pol_dn: String,
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name_alias: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pc_enf_pref: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pc_tag: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pref_gr_memb: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    prio: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    scope: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    shutdown: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    trigger_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    tx_id: String,
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
    DhcpConsLbl {},
    FaultCounts {},
    FaultDelegate {},
    FaultInst {},
    FvAEPgBkProp {},
    FvCEp(client_endpoint::FvCEp),
    FvCrtrn {},
    FvCtrctCtxDefCont {},
    FvFltCounter15min {},
    FvFltCounter1d {},
    FvFltCounter1h {},
    FvFltCounter1mo {},
    FvFltCounter1qtr {},
    FvFltCounter1w {},
    FvFltCounter1year {},
    FvFltCounter5min {},
    FvFltCounterHist15min {},
    FvFltCounterHist1d {},
    FvFltCounterHist1h {},
    FvFltCounterHist1mo {},
    FvFltCounterHist1qtr {},
    FvFltCounterHist1w {},
    FvFltCounterHist1year {},
    FvFltCounterHist5min {},
    FvOrchsInfo {},
    FvQinqFabEncap {},
    FvRInfoHolder {},
    FvRsAEPgMonPol {},
    FvRsBd {},
    FvRsCons {},
    FvRsConsIf {},
    FvRsCustQosPol {},
    FvRsDomAtt {},
    FvRsDppPol {},
    FvRsFcPathAtt {},
    FvRsGraphDef {},
    FvRsIntraEpg {},
    FvRsNodeAtt {},
    FvRsPathAtt(static_path::FvRsPathAtt),
    FvRsProtBy {},
    FvRsProv {},
    FvRsProvDef {},
    FvRsQosRequirement {},
    FvRsSecInherited {},
    FvRsTrustCtrl {},
    FvRtARemoteHostToEpg {},
    FvRtChassisEpg {},
    FvRtDestEpg {},
    FvRtDevEpg {},
    FvRtDevMgrEpg {},
    FvRtEpg {},
    FvRtExporterToEPg {},
    FvRtExtdevMgrMgmtEPg {},
    FvRtFromAbsEpg {},
    FvRtFromEpg {},
    FvRtFuncToEpg {},
    FvRtInstPToNatMappingEPg {},
    FvRtLIfCtxToInstP {},
    FvRtMatchEPg {},
    FvRtMgmtEPg {},
    FvRtNtpProvToEpg {},
    FvRtPoeEpg {},
    FvRtProfileToEpg {},
    FvRtProv {},
    FvRtRtdEpPToNatMappingEPg {},
    FvRtSecInherited {},
    FvRtSecProvToEpg {},
    FvRtSrcToEpg {},
    FvRtSvcMgmtEpg {},
    FvRtSvrEpg {},
    FvRtSvrToMgmtEPg {},
    FvRtTermToEPg {},
    FvRtToAbsEpg {},
    FvRtToAbsEpgForEpgToEpg {},
    FvRtToEpg {},
    FvRtToEpgForEpgToEpg {},
    FvRtVConnToEpgEp {},
    FvRtVConnToEpgSubnet {},
    FvRtVsrcToEpg {},
    FvSharedService {},
    FvSiteAssociated {},
    FvStCEp {},
    FvSubnet(subnet::FvSubnet),
    FvUpdateContract {},
    FvVip {},
    FvVipUpdate {},
    HealthInst {},
    HealthNodeInst {},
    L2EgrBytesAg15min {},
    L2EgrBytesAg1d {},
    L2EgrBytesAg1h {},
    L2EgrBytesAg1mo {},
    L2EgrBytesAg1qtr {},
    L2EgrBytesAg1w {},
    L2EgrBytesAg1year {},
    L2EgrBytesAgHist15min {},
    L2EgrBytesAgHist1d {},
    L2EgrBytesAgHist1h {},
    L2EgrBytesAgHist1mo {},
    L2EgrBytesAgHist1qtr {},
    L2EgrBytesAgHist1w {},
    L2EgrBytesAgHist1year {},
    L2EgrBytesPart15min {},
    L2EgrBytesPart1d {},
    L2EgrBytesPart1h {},
    L2EgrBytesPart1mo {},
    L2EgrBytesPart1qtr {},
    L2EgrBytesPart1w {},
    L2EgrBytesPart1year {},
    L2EgrBytesPart5min {},
    L2EgrBytesPartHist15min {},
    L2EgrBytesPartHist1d {},
    L2EgrBytesPartHist1h {},
    L2EgrBytesPartHist1mo {},
    L2EgrBytesPartHist1qtr {},
    L2EgrBytesPartHist1w {},
    L2EgrBytesPartHist1year {},
    L2EgrBytesPartHist5min {},
    L2EgrPktsAg15min {},
    L2EgrPktsAg1d {},
    L2EgrPktsAg1h {},
    L2EgrPktsAg1mo {},
    L2EgrPktsAg1qtr {},
    L2EgrPktsAg1w {},
    L2EgrPktsAg1year {},
    L2EgrPktsAgHist15min {},
    L2EgrPktsAgHist1d {},
    L2EgrPktsAgHist1h {},
    L2EgrPktsAgHist1mo {},
    L2EgrPktsAgHist1qtr {},
    L2EgrPktsAgHist1w {},
    L2EgrPktsAgHist1year {},
    L2EgrPktsPart15min {},
    L2EgrPktsPart1d {},
    L2EgrPktsPart1h {},
    L2EgrPktsPart1mo {},
    L2EgrPktsPart1qtr {},
    L2EgrPktsPart1w {},
    L2EgrPktsPart1year {},
    L2EgrPktsPart5min {},
    L2EgrPktsPartHist15min {},
    L2EgrPktsPartHist1d {},
    L2EgrPktsPartHist1h {},
    L2EgrPktsPartHist1mo {},
    L2EgrPktsPartHist1qtr {},
    L2EgrPktsPartHist1w {},
    L2EgrPktsPartHist1year {},
    L2EgrPktsPartHist5min {},
    L2IngrBytesAg15min {},
    L2IngrBytesAg1d {},
    L2IngrBytesAg1h {},
    L2IngrBytesAg1mo {},
    L2IngrBytesAg1qtr {},
    L2IngrBytesAg1w {},
    L2IngrBytesAg1year {},
    L2IngrBytesAgHist15min {},
    L2IngrBytesAgHist1d {},
    L2IngrBytesAgHist1h {},
    L2IngrBytesAgHist1mo {},
    L2IngrBytesAgHist1qtr {},
    L2IngrBytesAgHist1w {},
    L2IngrBytesAgHist1year {},
    L2IngrBytesPart15min {},
    L2IngrBytesPart1d {},
    L2IngrBytesPart1h {},
    L2IngrBytesPart1mo {},
    L2IngrBytesPart1qtr {},
    L2IngrBytesPart1w {},
    L2IngrBytesPart1year {},
    L2IngrBytesPart5min {},
    L2IngrBytesPartHist15min {},
    L2IngrBytesPartHist1d {},
    L2IngrBytesPartHist1h {},
    L2IngrBytesPartHist1mo {},
    L2IngrBytesPartHist1qtr {},
    L2IngrBytesPartHist1w {},
    L2IngrBytesPartHist1year {},
    L2IngrBytesPartHist5min {},
    L2IngrPktsAg15min {},
    L2IngrPktsAg1d {},
    L2IngrPktsAg1h {},
    L2IngrPktsAg1mo {},
    L2IngrPktsAg1qtr {},
    L2IngrPktsAg1w {},
    L2IngrPktsAg1year {},
    L2IngrPktsAgHist15min {},
    L2IngrPktsAgHist1d {},
    L2IngrPktsAgHist1h {},
    L2IngrPktsAgHist1mo {},
    L2IngrPktsAgHist1qtr {},
    L2IngrPktsAgHist1w {},
    L2IngrPktsAgHist1year {},
    L2IngrPktsPart15min {},
    L2IngrPktsPart1d {},
    L2IngrPktsPart1h {},
    L2IngrPktsPart1mo {},
    L2IngrPktsPart1qtr {},
    L2IngrPktsPart1w {},
    L2IngrPktsPart1year {},
    L2IngrPktsPart5min {},
    L2IngrPktsPartHist15min {},
    L2IngrPktsPartHist1d {},
    L2IngrPktsPartHist1h {},
    L2IngrPktsPartHist1mo {},
    L2IngrPktsPartHist1qtr {},
    L2IngrPktsPartHist1w {},
    L2IngrPktsPartHist1year {},
    L2IngrPktsPartHist5min {},
    L3MplsEgrBytesAg15min {},
    L3MplsEgrBytesAg1d {},
    L3MplsEgrBytesAg1h {},
    L3MplsEgrBytesAg1mo {},
    L3MplsEgrBytesAg1qtr {},
    L3MplsEgrBytesAg1w {},
    L3MplsEgrBytesAg1year {},
    L3MplsEgrBytesAgHist15min {},
    L3MplsEgrBytesAgHist1d {},
    L3MplsEgrBytesAgHist1h {},
    L3MplsEgrBytesAgHist1mo {},
    L3MplsEgrBytesAgHist1qtr {},
    L3MplsEgrBytesAgHist1w {},
    L3MplsEgrBytesAgHist1year {},
    L3MplsEgrBytesPart15min {},
    L3MplsEgrBytesPart1d {},
    L3MplsEgrBytesPart1h {},
    L3MplsEgrBytesPart1mo {},
    L3MplsEgrBytesPart1qtr {},
    L3MplsEgrBytesPart1w {},
    L3MplsEgrBytesPart1year {},
    L3MplsEgrBytesPart5min {},
    L3MplsEgrBytesPartHist15min {},
    L3MplsEgrBytesPartHist1d {},
    L3MplsEgrBytesPartHist1h {},
    L3MplsEgrBytesPartHist1mo {},
    L3MplsEgrBytesPartHist1qtr {},
    L3MplsEgrBytesPartHist1w {},
    L3MplsEgrBytesPartHist1year {},
    L3MplsEgrBytesPartHist5min {},
    L3MplsEgrPktsAg15min {},
    L3MplsEgrPktsAg1d {},
    L3MplsEgrPktsAg1h {},
    L3MplsEgrPktsAg1mo {},
    L3MplsEgrPktsAg1qtr {},
    L3MplsEgrPktsAg1w {},
    L3MplsEgrPktsAg1year {},
    L3MplsEgrPktsAgHist15min {},
    L3MplsEgrPktsAgHist1d {},
    L3MplsEgrPktsAgHist1h {},
    L3MplsEgrPktsAgHist1mo {},
    L3MplsEgrPktsAgHist1qtr {},
    L3MplsEgrPktsAgHist1w {},
    L3MplsEgrPktsAgHist1year {},
    L3MplsEgrPktsPart15min {},
    L3MplsEgrPktsPart1d {},
    L3MplsEgrPktsPart1h {},
    L3MplsEgrPktsPart1mo {},
    L3MplsEgrPktsPart1qtr {},
    L3MplsEgrPktsPart1w {},
    L3MplsEgrPktsPart1year {},
    L3MplsEgrPktsPart5min {},
    L3MplsEgrPktsPartHist15min {},
    L3MplsEgrPktsPartHist1d {},
    L3MplsEgrPktsPartHist1h {},
    L3MplsEgrPktsPartHist1mo {},
    L3MplsEgrPktsPartHist1qtr {},
    L3MplsEgrPktsPartHist1w {},
    L3MplsEgrPktsPartHist1year {},
    L3MplsEgrPktsPartHist5min {},
    L3MplsIngrBytesAg15min {},
    L3MplsIngrBytesAg1d {},
    L3MplsIngrBytesAg1h {},
    L3MplsIngrBytesAg1mo {},
    L3MplsIngrBytesAg1qtr {},
    L3MplsIngrBytesAg1w {},
    L3MplsIngrBytesAg1year {},
    L3MplsIngrBytesAgHist15min {},
    L3MplsIngrBytesAgHist1d {},
    L3MplsIngrBytesAgHist1h {},
    L3MplsIngrBytesAgHist1mo {},
    L3MplsIngrBytesAgHist1qtr {},
    L3MplsIngrBytesAgHist1w {},
    L3MplsIngrBytesAgHist1year {},
    L3MplsIngrBytesPart15min {},
    L3MplsIngrBytesPart1d {},
    L3MplsIngrBytesPart1h {},
    L3MplsIngrBytesPart1mo {},
    L3MplsIngrBytesPart1qtr {},
    L3MplsIngrBytesPart1w {},
    L3MplsIngrBytesPart1year {},
    L3MplsIngrBytesPart5min {},
    L3MplsIngrBytesPartHist15min {},
    L3MplsIngrBytesPartHist1d {},
    L3MplsIngrBytesPartHist1h {},
    L3MplsIngrBytesPartHist1mo {},
    L3MplsIngrBytesPartHist1qtr {},
    L3MplsIngrBytesPartHist1w {},
    L3MplsIngrBytesPartHist1year {},
    L3MplsIngrBytesPartHist5min {},
    L3MplsIngrPktsAg15min {},
    L3MplsIngrPktsAg1d {},
    L3MplsIngrPktsAg1h {},
    L3MplsIngrPktsAg1mo {},
    L3MplsIngrPktsAg1qtr {},
    L3MplsIngrPktsAg1w {},
    L3MplsIngrPktsAg1year {},
    L3MplsIngrPktsAgHist15min {},
    L3MplsIngrPktsAgHist1d {},
    L3MplsIngrPktsAgHist1h {},
    L3MplsIngrPktsAgHist1mo {},
    L3MplsIngrPktsAgHist1qtr {},
    L3MplsIngrPktsAgHist1w {},
    L3MplsIngrPktsAgHist1year {},
    L3MplsIngrPktsPart15min {},
    L3MplsIngrPktsPart1d {},
    L3MplsIngrPktsPart1h {},
    L3MplsIngrPktsPart1mo {},
    L3MplsIngrPktsPart1qtr {},
    L3MplsIngrPktsPart1w {},
    L3MplsIngrPktsPart1year {},
    L3MplsIngrPktsPart5min {},
    L3MplsIngrPktsPartHist15min {},
    L3MplsIngrPktsPartHist1d {},
    L3MplsIngrPktsPartHist1h {},
    L3MplsIngrPktsPartHist1mo {},
    L3MplsIngrPktsPartHist1qtr {},
    L3MplsIngrPktsPartHist1w {},
    L3MplsIngrPktsPartHist1year {},
    L3MplsIngrPktsPartHist5min {},
    MdpClassId {},
    MdpLocalEp {},
    OrchsLDevVipCfg {},
    TagAliasDelInst {},
    TagAliasInst {},
    TagAnnotation {},
    TagExtMngdInst {},
    TagInst {},
    TagTag {},
    TelemetryMatchedSelector {},
    VnsAbsCfgRel {},
    VnsAbsFolder {},
    VnsAbsParam {},
    VnsAddrInst {},
    VnsCFolder {},
    VnsCParam {},
    VnsCRel {},
    VnsCfgRelInst {},
    VnsCtrlrEp {},
    VnsFolderInst {},
    VnsGFolder {},
    VnsGParam {},
    VnsGRel {},
    VnsLBIPReq {},
    VnsLBRNatReq {},
    VnsLBVSvcGrpSvrReq {},
    VnsLBVSvrReq {},
    VnsNATDynPATReq {},
    VnsNATPATReq {},
    VnsParamInst {},
    VnsSvcPol {},
    VsvcConsLbl {},
    VzConsCtrctLbl {},
    VzConsLbl {},
    VzConsSubjLbl {},
    VzProvCtrctLbl {},
    VzProvLbl {},
    VzProvSubjLbl {},
}

#[derive(Debug, Clone)]
pub enum FvAEPgEndpoint {
    ClassAll,
    ClassTenant {
        tenant: String,
    },
    ClassAp {
        tenant: String,
        ap: String,
    },
    MoUni,
    MoAp {
        tenant: String,
        ap: String,
        epg: String,
    },
}

impl EndpointScheme for FvAEPgEndpoint {
    fn endpoint(&self) -> std::borrow::Cow<'_, str> {
        match self {
            Self::ClassAll => std::borrow::Cow::Borrowed("node/class/fvAEPg.json"),
            Self::ClassTenant { tenant } => {
                std::borrow::Cow::Owned(format!("node/class/uni/tn-{tenant}/fvAEPg.json"))
            }
            Self::ClassAp { tenant, ap } => {
                std::borrow::Cow::Owned(format!("node/class/uni/tn-{tenant}/ap-{ap}/fvAEPg.json"))
            }
            Self::MoUni => std::borrow::Cow::Borrowed("mo/uni.json"),
            Self::MoAp { tenant, ap, epg } => {
                std::borrow::Cow::Owned(format!("mo/uni/tn-{tenant}/ap-{ap}/epg-{epg}.json"))
            }
        }
    }
}

pub type FvAEPg = AciObject<__internal::FvAEPg>;

impl FvAEPg {
    pub fn new(name: &str, tenant: &str, ap: &str) -> Self {
        let mut attr = Attributes::default();
        attr.dn = format!("uni/tn-{tenant}/ap-{ap}/epg-{name}");
        attr.name = name.to_owned();

        Self {
            attributes: attr,
            children: vec![],
        }
    }

    pub fn set_descr(&mut self, descr: impl ToString) {
        self.attributes.descr = descr.to_string();
    }
}

mod __internal {
    use super::*;
    use crate::AciObjectScheme;

    #[derive(Debug, Clone, Copy)]
    pub struct FvAEPg;

    impl AciObjectScheme for FvAEPg {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = FvAEPgEndpoint;
        const CLASS_NAME: &'static str = "fvAEPg";
    }
}
