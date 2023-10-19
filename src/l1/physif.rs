use serde::{Deserialize, Serialize};

use crate::{ethpm, AciObject, EndpointScheme};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    admin_st: String,
    auto_neg: String,
    break_t: String,
    brkout_map: String,
    bw: String,
    child_action: String,
    delay: String,
    descr: String,
    dfe_delay_ms: String,
    dot1q_ether_type: String,
    emi_retrain: String,
    enable_poap: String,
    ethpm_cfg_failed_bmp: String,
    ethpm_cfg_failed_ts: String,
    ethpm_cfg_state: String,
    fcot_channel_number: String,
    fec_mode: String,
    id: String,
    inh_bw: String,
    is_reflective_relay_cfg_supported: String,
    layer: String,
    lc_own: String,
    link_debounce: String,
    link_flap_error_max: String,
    link_flap_error_seconds: String,
    link_log: String,
    mdix: String,
    medium: String,
    mod_ts: String,
    mode: String,
    mon_pol_dn: String,
    mtu: String,
    name: String,
    path_s_descr: String,
    port_phy_media_type: String,
    port_t: String,
    prio_flow_ctrl: String,
    reflective_relay_en: String,
    rn: String,
    router_mac: String,
    snmp_trap_st: String,
    span_mode: String,
    speed: String,
    status: String,
    switching_st: String,
    trunk_log: String,
    usage: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    EthpmPhysIf(ethpm::physif::EthpmPhysIf),
    FvDomDef {},
    L1LinkLevelFlowCtrlP {},
    L1LoadP {},
    L1PrioFlowCtrlP {},
    L1RsCdpIfPolCons {},
    L1RsCoppIfPolCons {},
    L1RsDwdmFabIfPolCons {},
    L1RsDwdmIfPolCons {},
    L1EeeP {},
    L1RsAttEntityPCons {},
    L1RsFIfPolCons {},
    L1RsFLinkFlapPolCons {},
    L1RsFcIfPolCons {},
    L1RsHIfPolCons {},
    L1RsLacpIfPolCons {},
    L1RsLinkFlapPolCons {},
    L1RsLldpIfPolCons {},
    L1RsL2PortSecurityCons {},
    L1RsL2IfPolCons {},
    L1RsL3IfPolCons {},
    L1RsMacsecPolCons {},
    L1RsMcpIfPolCons {},
    L1RsMonPolIfPolCons {},
    L1RsPoeIfPolCons {},
    L1RsQosEgressDppIfPolCons {},
    L1RsQosIngressDppIfPolCons {},
    L1RsQosLlfcIfPolCons {},
    L1RsQosPfcIfPolCons {},
    L1RsQosSdIfPolCons {},
    L1RsStormctrlIfPolCons {},
    L1RsStpIfPolCons {},
    L1RsSynceEthIfPolCons {},
    L1RtBrConf {},
    L1RtEncPhysRtdConf {},
    L1RtEthIf {},
    L1RtMbrIfs {},
    L1RtIoPPhysConf {},
    L1RtPhysRtdConf {},
    L1RtToObservedEthIf {},
    L1StormCtrlP {},
    NwRtPathToIf {},
}

#[derive(Debug, Clone, Copy)]
pub enum Endpoint {}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> std::borrow::Cow<'_, str> {
        unimplemented!()
    }
}

pub type L1PhysIf = AciObject<__internal::L1PhysIf>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;

    #[derive(Debug, Clone, Copy)]
    pub struct L1PhysIf;

    impl AciObjectScheme for L1PhysIf {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "l1PhysIf";
    }
}
