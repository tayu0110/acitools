use super::{bridge_domain, domain_profile, ext_epg, logical_node_profile, private_network};
use crate::{bgp, ospf, rtctrl, AciObject, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    annotation: String,
    child_action: String,
    descr: String,
    dn: String,
    enforce_rtctrl: EnforceRouteControl,
    mpls_enabled: String,
    name: String,
    name_alias: String,
    owner_key: String,
    owner_tag: String,
    status: String,
    target_dscp: String,
    userdom: String,
    #[serde(flatten)]
    payload: Option<HashMap<String, String>>,
    // ext_mngd_by: String,
    // lc_own: String,
    // mod_ts: String,
    // mon_pol_dn: String,
    // uid: String,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EnforceRouteControl {
    #[default]
    Export,
    Import,
    #[serde(rename = "export,import")]
    ExportImport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    BgpDomainIdAllocator(bgp::domain_id::BgpDomainIdAllocator),
    BgpExtP(bgp::ext_epg::BgpExtP),
    EigrpExtP {},
    L3extCtxUpdater {},
    L3extExtEncapAllocator {},
    L3extInstP(ext_epg::L3extInstP),
    L3extLNodeP(logical_node_profile::L3extLNodeP),
    L3extRtBDToOut(bridge_domain::L3extRtBDToOut),
    L3extRsEctx(private_network::L3extRsEctx),
    L3extRsL3DomAtt(domain_profile::L3extRsL3DomAtt),
    L3extRtOutDefContToOut {},
    L3extRsOutToBDPublicSubnetHolder {},
    L3extRtSrcToL3extOut {},
    OspfExtP(ospf::ext_epg::OspfExtP),
    RtctrlProfile(rtctrl::profile::RtctrlProfile),
}

#[derive(Debug, Clone, Copy)]
pub enum L3extOutEndpoint {
    ClassAll,
}

impl EndpointScheme for L3extOutEndpoint {
    fn endpoint(&self) -> std::borrow::Cow<'_, str> {
        match self {
            Self::ClassAll => std::borrow::Cow::Borrowed("node/class/l3extOut.json"),
        }
    }
}

pub type L3extOut = AciObject<__internal::L3extOut>;

mod __internal {
    use crate::AciObjectScheme;

    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub struct L3extOut;

    impl AciObjectScheme for L3extOut {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = L3extOutEndpoint;
        const CLASS_NAME: &'static str = "l3extOut";
    }
}
