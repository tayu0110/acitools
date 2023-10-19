use crate::{eqpt, l1, lldp, BuilderTrait, Client};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TopSystem {
    TopSystem {
        attributes: Attributes,
        #[serde(default)]
        children: Vec<ChildItem>,
    },
}

impl TopSystem {
    fn attributes(&self) -> &Attributes {
        let TopSystem::TopSystem { attributes, .. } = self;
        attributes
    }

    pub fn get(
        client: &Client,
        pod: u32,
    ) -> Result<GetSystemRequestBuilder, Box<dyn std::error::Error>> {
        Ok(GetSystemRequestBuilder::new(
            client
                .get(format!("node/class/topology/pod-{pod}/topSystem.json").as_str())?
                .rsp_subtree(crate::RspSubTree::Full),
        ))
    }

    pub fn child_action(&self) -> &str {
        &self.attributes().child_action
    }

    pub fn name(&self) -> &str {
        &self.attributes().name
    }

    pub fn name_alias(&self) -> &str {
        &self.attributes().name_alias
    }
}

pub struct GetSystemRequestBuilder<'a> {
    builder: crate::client::GetRequestBuilder<'a>,
}

impl<'a> GetSystemRequestBuilder<'a> {
    fn new(builder: crate::client::GetRequestBuilder<'a>) -> Self {
        Self { builder }
    }

    pub async fn send(self) -> Result<Box<[TopSystem]>, Box<dyn std::error::Error>> {
        let res = self.builder.send().await?;
        Ok(res
            .into_iter()
            .map(|res| serde_json::from_value(res))
            .collect::<Result<Vec<TopSystem>, serde_json::Error>>()?
            .into_boxed_slice())
    }
}

impl<'a> BuilderTrait<'a> for GetSystemRequestBuilder<'a> {
    fn renew(builder: crate::GetRequestBuilder<'a>) -> Self {
        Self::new(builder)
    }
    fn builder(self) -> crate::GetRequestBuilder<'a> {
        self.builder
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    address: String,
    bootstrap_state: String,
    child_action: String,
    cluster_time_diff: String,
    config_issues: String,
    #[serde(rename = "controlPlaneMTU")]
    control_plane_mtu: String,
    current_time: String,
    dn: String,
    enforce_subnet_check: String,
    etep_addr: String,
    fabric_domain: String,
    fabric_id: String,
    #[serde(rename = "fabricMAC")]
    fabric_mac: String,
    id: String,
    inb_mgmt_addr: String,
    inb_mgmt_addr6: String,
    inb_mgmt_addr6_mask: String,
    inb_mgmt_addr_mask: String,
    inb_mgmt_gateway: String,
    inb_mgmt_gateway6: String,
    last_reboot_time: String,
    last_reset_reason: String,
    lc_own: String,
    mod_ts: String,
    mode: String,
    mon_pol_dn: String,
    name: String,
    name_alias: String,
    node_type: String,
    oob_mgmt_addr: String,
    oob_mgmt_addr6: String,
    oob_mgmt_addr6_mask: String,
    oob_mgmt_addr_mask: String,
    oob_mgmt_gateway: String,
    oob_mgmt_gateway6: String,
    pod_id: String,
    remote_network_id: String,
    remote_node: String,
    rl_oper_pod_id: String,
    rl_routable_mode: String,
    rldirect_mode: String,
    role: String,
    serial: String,
    server_type: String,
    site_id: String,
    state: String,
    status: String,
    system_up_time: String,
    tep_pool: String,
    unicast_xr_ep_learn_disable: String,
    version: String,
    virtual_mode: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    AaaCtrlrFipsState {},
    AaaFipsState {},
    AcEntity {},
    AclcapProv {},
    ActionLCont {},
    ActrlEntity {},
    ActrlcapProv {},
    AnalyticsEntity {},
    ArpEntity {},
    BdEnforceExpCont {},
    BfdEntity {},
    BgpEntity {},
    CapCat {},
    CdpEntity {},
    CertEntity {},
    CnwAggrIf {},
    CnwPhysIf {},
    CommSshInst {},
    CoopEntity {},
    CoppEntity {},
    DatetimeClkPol {},
    DhcpEntity {},
    DnsEntity {},
    Dot1xEntity {},
    DppEntity {},
    EigrpEntity {},
    EthpmEntity {},
    EqptCh(eqpt::chassis::EqptCh),
    EqptcapacityEntity {},
    EqptdiagEntity {},
    EqptEntity {},
    EqptFeatureEx {},
    EqptReloadSwitch {},
    FabrgmEntity {},
    FabricDecommission {},
    FabricLeafNodeRole {},
    FabricSpineNodeRole {},
    FabricSystemInfo {},
    FcoeEntity {},
    FcpmEntity {},
    FirmwareCtrlrFwStatusCont {},
    FirmwareCatFwStatusCont {},
    FirmwareFwStatusCont {},
    FvEpNs {},
    FvIfConnOper {},
    FvImplicitStaleEpCont {},
    FvStaleTunEpCont {},
    HsrpEntity {},
    LicenseEntity {},
    LldpEntity(lldp::entity::LldpEntity),
    L1PhysIf(l1::physif::L1PhysIf),
    L1capProv {},
    L2BrIf {},
    L2capProv {},
    L3Inst {},
    L3capProv {},
    L3Ctx {},
    LacpEntity {},
    IgmpEntity {},
    Icmpv4Entity {},
    IgmpsnoopEntity {},
    Icmpv6Entity {},
    IsisEntity {},
    IpagectrlEntity {},
    Ipv4Entity {},
    Ipv6Entity {},
    MacsecEntity {},
    McpEntity {},
    MgmtMgmtIf {},
    MldsnoopEntity {},
    MockMockRoot {},
    NdEntity {},
    NpvEntity {},
    NwConnGrp {},
    OpflexOeHupTrigger {},
    OspfEntity {},
    Ospfv3Entity {},
    PatchingEntity {},
    PcAggrIf {},
    PcEntity {},
    PcFcEntity {},
    PoeEntity {},
    PconsBootStrapTracking {},
    PkiFabricSelfCACertsModified {},
    Pim6Entity {},
    PimEntity {},
    PtpEntity {},
    QosmEntity {},
    RadiusEntity {},
    RpmEntity {},
    SnmpEntity {},
    SpanAcct {},
    SpanEntity {},
    SpanRetryCont {},
    StatsprefCont {},
    StpEntity {},
    SysdebugCoreFileRepository {},
    SysdebugTechSupFileRepository {},
    SysdebugEp {},
    TelemetryInst {},
    TelemetryCapabilityCont {},
    TopoctrlEntity {},
    TopRsSysErrDisRecoverPolCons {},
    TopRsSysFastLinkFailoverInstPolCons {},
    TopRsSysFcFabricPolCons {},
    TopRsSysFcInstPolCons {},
    TopRsSysFlashConfigPolCons {},
    TopRsSysFwdScaleProfPolCons {},
    TopRsSysIaclProfilePolCons {},
    TopRsSysLldpInstPolCons {},
    TopRsSysL2NodePolAuthCons {},
    TopRsSysMcpInstPolCons {},
    TopRsSysMstInstPolCons {},
    TopRsSysNetflowNodePolCons {},
    TopRsSysPoeInstPolCons {},
    TopRsSysPsuInstPolCons {},
    TopRsSysUsbConfigProfilePolCons {},
    TopRsSysSynceInstPolCons {},
    TopRsSystemRack {},
    TunnelIf {},
    TwampEntity {},
    VpcEntity {},
}
