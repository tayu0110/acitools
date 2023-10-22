use crate::{AciObject, ConfigStatus, Configurable, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{client_endpoint, related_context};

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    annotation: String,
    child_action: String,
    descr: String,
    #[serde(default)]
    dn: String,
    name: String,
    name_alias: String,
    owner_key: String,
    owner_tag: String,
    status: ConfigStatus,
    userdom: String,
    #[serde(flatten)]
    payload: Option<HashMap<String, String>>,
    // bd_enforced_enable: String,
    // ext_mngd_by: String,
    // ip_data_plane_learning: String,
    // knw_mcast_act: String,
    // lc_own: String,
    // mod_ts: String,
    // mon_pol_dn: String,
    // pc_enf_dir: String,
    // pc_enf_dir_updated: String,
    // pc_enf_pref: String,
    // pc_tag: String,
    // scope: String,
    // seg: String,
    // uid: String,
    // vrf_id: String,
    // vrf_index: String,
}

impl Attributes {
    pub fn name(&self) -> &str {
        &self.name
    }
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
    BgpDomainIdAllocator {},
    BgpRtTargetP {},
    CloudAppGwStatsAg15min {},
    CloudAppGwStatsAg1d {},
    CloudAppGwStatsAg1h {},
    CloudAppGwStatsAg1mo {},
    CloudAppGwStatsAg1qtr {},
    CloudAppGwStatsAg1w {},
    CloudAppGwStatsAg1year {},
    CloudAppGwStatsAg5min {},
    CloudAppGwStatsAgHist15min {},
    CloudAppGwStatsAgHist1d {},
    CloudAppGwStatsAgHist1h {},
    CloudAppGwStatsAgHist1mo {},
    CloudAppGwStatsAgHist1qtr {},
    CloudAppGwStatsAgHist1w {},
    CloudAppGwStatsAgHist1year {},
    CloudAppGwStatsAgHist5min {},
    CloudAzNLBStatsAg15min {},
    CloudAzNLBStatsAg1d {},
    CloudAzNLBStatsAg1h {},
    CloudAzNLBStatsAg1mo {},
    CloudAzNLBStatsAg1qtr {},
    CloudAzNLBStatsAg1w {},
    CloudAzNLBStatsAg1year {},
    CloudAzNLBStatsAg5min {},
    CloudAzNLBStatsAgHist15min {},
    CloudAzNLBStatsAgHist1d {},
    CloudAzNLBStatsAgHist1h {},
    CloudAzNLBStatsAgHist1mo {},
    CloudAzNLBStatsAgHist1qtr {},
    CloudAzNLBStatsAgHist1w {},
    CloudAzNLBStatsAgHist1year {},
    CloudAzNLBStatsAgHist5min {},
    CloudBdiId {},
    CloudHostRouterEgressBytesAg15min {},
    CloudHostRouterEgressBytesAg1d {},
    CloudHostRouterEgressBytesAg1h {},
    CloudHostRouterEgressBytesAg1mo {},
    CloudHostRouterEgressBytesAg1qtr {},
    CloudHostRouterEgressBytesAg1w {},
    CloudHostRouterEgressBytesAg1year {},
    CloudHostRouterEgressBytesAg5min {},
    CloudHostRouterEgressBytesAgHist15min {},
    CloudHostRouterEgressBytesAgHist1d {},
    CloudHostRouterEgressBytesAgHist1h {},
    CloudHostRouterEgressBytesAgHist1mo {},
    CloudHostRouterEgressBytesAgHist1qtr {},
    CloudHostRouterEgressBytesAgHist1w {},
    CloudHostRouterEgressBytesAgHist1year {},
    CloudHostRouterEgressBytesAgHist5min {},
    CloudHostRouterEgressPktsAg15min {},
    CloudHostRouterEgressPktsAg1d {},
    CloudHostRouterEgressPktsAg1h {},
    CloudHostRouterEgressPktsAg1mo {},
    CloudHostRouterEgressPktsAg1qtr {},
    CloudHostRouterEgressPktsAg1w {},
    CloudHostRouterEgressPktsAg1year {},
    CloudHostRouterEgressPktsAg5min {},
    CloudHostRouterEgressPktsAgHist15min {},
    CloudHostRouterEgressPktsAgHist1d {},
    CloudHostRouterEgressPktsAgHist1h {},
    CloudHostRouterEgressPktsAgHist1mo {},
    CloudHostRouterEgressPktsAgHist1qtr {},
    CloudHostRouterEgressPktsAgHist1w {},
    CloudHostRouterEgressPktsAgHist1year {},
    CloudHostRouterEgressPktsAgHist5min {},
    CloudHostRouterIngressBytesAg15min {},
    CloudHostRouterIngressBytesAg1d {},
    CloudHostRouterIngressBytesAg1h {},
    CloudHostRouterIngressBytesAg1mo {},
    CloudHostRouterIngressBytesAg1qtr {},
    CloudHostRouterIngressBytesAg1w {},
    CloudHostRouterIngressBytesAg1year {},
    CloudHostRouterIngressBytesAg5min {},
    CloudHostRouterIngressBytesAgHist15min {},
    CloudHostRouterIngressBytesAgHist1d {},
    CloudHostRouterIngressBytesAgHist1h {},
    CloudHostRouterIngressBytesAgHist1mo {},
    CloudHostRouterIngressBytesAgHist1qtr {},
    CloudHostRouterIngressBytesAgHist1w {},
    CloudHostRouterIngressBytesAgHist1year {},
    CloudHostRouterIngressBytesAgHist5min {},
    CloudHostRouterIngressPktsAg15min {},
    CloudHostRouterIngressPktsAg1d {},
    CloudHostRouterIngressPktsAg1h {},
    CloudHostRouterIngressPktsAg1mo {},
    CloudHostRouterIngressPktsAg1qtr {},
    CloudHostRouterIngressPktsAg1w {},
    CloudHostRouterIngressPktsAg1year {},
    CloudHostRouterIngressPktsAg5min {},
    CloudHostRouterIngressPktsAgHist15min {},
    CloudHostRouterIngressPktsAgHist1d {},
    CloudHostRouterIngressPktsAgHist1h {},
    CloudHostRouterIngressPktsAgHist1mo {},
    CloudHostRouterIngressPktsAgHist1qtr {},
    CloudHostRouterIngressPktsAgHist1w {},
    CloudHostRouterIngressPktsAgHist1year {},
    CloudHostRouterIngressPktsAgHist5min {},
    CloudHostRouterRgInfoHolder {},
    CloudHostRouterUserRgInfoHolder {},
    CloudLBStatsAg15min {},
    CloudLBStatsAg1d {},
    CloudLBStatsAg1h {},
    CloudLBStatsAg1mo {},
    CloudLBStatsAg1qtr {},
    CloudLBStatsAg1w {},
    CloudLBStatsAg1year {},
    CloudLBStatsAg5min {},
    CloudLBStatsAgHist15min {},
    CloudLBStatsAgHist1d {},
    CloudLBStatsAgHist1h {},
    CloudLBStatsAgHist1mo {},
    CloudLBStatsAgHist1qtr {},
    CloudLBStatsAgHist1w {},
    CloudLBStatsAgHist1year {},
    CloudLBStatsAgHist5min {},
    CloudNLBStatsAg15min {},
    CloudNLBStatsAg1d {},
    CloudNLBStatsAg1h {},
    CloudNLBStatsAg1mo {},
    CloudNLBStatsAg1qtr {},
    CloudNLBStatsAg1w {},
    CloudNLBStatsAg1year {},
    CloudNLBStatsAg5min {},
    CloudNLBStatsAgHist15min {},
    CloudNLBStatsAgHist1d {},
    CloudNLBStatsAgHist1h {},
    CloudNLBStatsAgHist1mo {},
    CloudNLBStatsAgHist1qtr {},
    CloudNLBStatsAgHist1w {},
    CloudNLBStatsAgHist1year {},
    CloudNLBStatsAgHist5min {},
    DnsLbl {},
    FaultCounts {},
    FaultDelegate {},
    FaultInst {},
    FvCEp(client_endpoint::FvCEp),
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
    FvRouteInfoHolder {},
    FvRsBgpCtxPol {},
    FvRsCtxMcastTo {},
    FvRsCtxMonPol {},
    FvRsCtxToBgpCtxAfPol {},
    FvRsCtxToCloudVrfRouteLeakPol {},
    FvRsCtxToEigrpCtxAfPol {},
    FvRsCtxToEpRet {},
    FvRsCtxToExtRouteTagPol {},
    FvRsCtxToOspfCtxPol {},
    FvRsCtxToRtctrlProfile {},
    FvRsCtxToSDWanVpn {},
    FvRsOspfCtxPol {},
    FvRsVrfValidationPol {},
    FvRtAcExtPolToContext {},
    FvRtCloudEPgCtx {},
    FvRtContext {},
    FvRtCopyCtx {},
    FvRtCtx(related_context::FvRtCtx),
    FvRtEctx {},
    FvRtEpTagCtx {},
    FvRtExporterToCtx {},
    FvRtExtCtxProfileToCtx {},
    FvRtExtNetworkPToCtx {},
    FvRtInstPCtx {},
    FvRtLDevToCtx {},
    FvRtOoBCtx {},
    FvRtScope {},
    FvRtServerToCtx {},
    FvRtSrcToCtx {},
    FvRtSrvCtx {},
    FvRtSubnetToCtx {},
    FvRtSvcEPgCtx {},
    FvRtToCtx {},
    FvSharedService {},
    FvSiteAssociated {},
    HealthInst {},
    HealthNodeInst {},
    IgmpCtxP {},
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
    L3EgrBytesAg15min {},
    L3EgrBytesAg1d {},
    L3EgrBytesAg1h {},
    L3EgrBytesAg1mo {},
    L3EgrBytesAg1qtr {},
    L3EgrBytesAg1w {},
    L3EgrBytesAg1year {},
    L3EgrBytesAgHist15min {},
    L3EgrBytesAgHist1d {},
    L3EgrBytesAgHist1h {},
    L3EgrBytesAgHist1mo {},
    L3EgrBytesAgHist1qtr {},
    L3EgrBytesAgHist1w {},
    L3EgrBytesAgHist1year {},
    L3EgrPktsAg15min {},
    L3EgrPktsAg1d {},
    L3EgrPktsAg1h {},
    L3EgrPktsAg1mo {},
    L3EgrPktsAg1qtr {},
    L3EgrPktsAg1w {},
    L3EgrPktsAg1year {},
    L3EgrPktsAgHist15min {},
    L3EgrPktsAgHist1d {},
    L3EgrPktsAgHist1h {},
    L3EgrPktsAgHist1mo {},
    L3EgrPktsAgHist1qtr {},
    L3EgrPktsAgHist1w {},
    L3EgrPktsAgHist1year {},
    L3IngrBytesAg15min {},
    L3IngrBytesAg1d {},
    L3IngrBytesAg1h {},
    L3IngrBytesAg1mo {},
    L3IngrBytesAg1qtr {},
    L3IngrBytesAg1w {},
    L3IngrBytesAg1year {},
    L3IngrBytesAgHist15min {},
    L3IngrBytesAgHist1d {},
    L3IngrBytesAgHist1h {},
    L3IngrBytesAgHist1mo {},
    L3IngrBytesAgHist1qtr {},
    L3IngrBytesAgHist1w {},
    L3IngrBytesAgHist1year {},
    L3IngrPktsAg15min {},
    L3IngrPktsAg1d {},
    L3IngrPktsAg1h {},
    L3IngrPktsAg1mo {},
    L3IngrPktsAg1qtr {},
    L3IngrPktsAg1w {},
    L3IngrPktsAg1year {},
    L3IngrPktsAgHist15min {},
    L3IngrPktsAgHist1d {},
    L3IngrPktsAgHist1h {},
    L3IngrPktsAgHist1mo {},
    L3IngrPktsAgHist1qtr {},
    L3IngrPktsAgHist1w {},
    L3IngrPktsAgHist1year {},
    L3extCtxExtEncapAllocator {},
    L3extGlobalCtxName {},
    L3extInfraOutExp {},
    L3extMplsVpnLabelAllocator {},
    LeakRoutes {},
    OrchsConfig {},
    PimCtxP {},
    PimIPV6CtxP {},
    SnmpCtxP {},
    TagAliasDelInst {},
    TagAliasInst {},
    TagAnnotation {},
    TagExtMngdInst {},
    TagInst {},
    TagTag {},
    TelemetrySubnetFltGrp {},
    Uribv4EgrBytesAg15min {},
    Uribv4EgrBytesAg1d {},
    Uribv4EgrBytesAg1h {},
    Uribv4EgrBytesAg1mo {},
    Uribv4EgrBytesAg1qtr {},
    Uribv4EgrBytesAg1w {},
    Uribv4EgrBytesAg1year {},
    Uribv4EgrBytesAgHist15min {},
    Uribv4EgrBytesAgHist1d {},
    Uribv4EgrBytesAgHist1h {},
    Uribv4EgrBytesAgHist1mo {},
    Uribv4EgrBytesAgHist1qtr {},
    Uribv4EgrBytesAgHist1w {},
    Uribv4EgrBytesAgHist1year {},
    Uribv4EgrPktsAg15min {},
    Uribv4EgrPktsAg1d {},
    Uribv4EgrPktsAg1h {},
    Uribv4EgrPktsAg1mo {},
    Uribv4EgrPktsAg1qtr {},
    Uribv4EgrPktsAg1w {},
    Uribv4EgrPktsAg1year {},
    Uribv4EgrPktsAgHist15min {},
    Uribv4EgrPktsAgHist1d {},
    Uribv4EgrPktsAgHist1h {},
    Uribv4EgrPktsAgHist1mo {},
    Uribv4EgrPktsAgHist1qtr {},
    Uribv4EgrPktsAgHist1w {},
    Uribv4EgrPktsAgHist1year {},
    VnsAbsCfgRel {},
    VnsAbsFolder {},
    VnsAbsParam {},
    VnsCFolder {},
    VnsCParam {},
    VnsCRel {},
    VnsCfgRelInst {},
    VnsFolderInst {},
    VnsGFolder {},
    VnsGParam {},
    VnsGRel {},
    VnsParamInst {},
    VzAny {},
}

#[derive(Debug, Clone)]
pub enum FvCtxEndpoint {
    ClassAll,
    MoUni,
    ClassTenant { tenant: String },
    MoCtx { tenant: String, ctx: String },
}

impl EndpointScheme for FvCtxEndpoint {
    fn endpoint(&self) -> std::borrow::Cow<'_, str> {
        match self {
            Self::ClassAll => std::borrow::Cow::Borrowed("node/class/fvCtx.json"),
            Self::MoUni => std::borrow::Cow::Borrowed("mo/uni.json"),
            Self::ClassTenant { tenant } => {
                std::borrow::Cow::Owned(format!("node/class/uni/tn-{tenant}/fvCtx.json"))
            }
            Self::MoCtx { tenant, ctx } => {
                std::borrow::Cow::Owned(format!("mo/uni/tn-{tenant}/ctx-{ctx}.json"))
            }
        }
    }
}

pub type FvCtx = AciObject<__internal::FvCtx>;

impl FvCtx {
    pub fn new(name: &str, tenant: &str) -> Self {
        Self {
            attributes: Attributes {
                annotation: String::new(),
                child_action: String::new(),
                descr: String::new(),
                dn: format!("uni/tn-{}/ctx-{}", tenant, name),
                name: name.to_string(),
                name_alias: String::new(),
                owner_key: String::new(),
                owner_tag: String::new(),
                status: ConfigStatus::None,
                userdom: String::new(),
                payload: None,
            },
            children: vec![],
        }
    }

    pub fn set_descr(&mut self, descr: &str) {
        self.attributes.descr = descr.to_owned();
    }
}

mod __internal {
    use super::*;
    use crate::AciObjectScheme;

    #[derive(Debug, Clone, Copy)]
    pub struct FvCtx;

    impl AciObjectScheme for FvCtx {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = FvCtxEndpoint;
        const CLASS_NAME: &'static str = "fvCtx";
    }
}
