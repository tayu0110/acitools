use crate::{BuilderTrait, Client};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FvBD {
    #[serde(rename = "fvBD")]
    FvBD {
        attributes: Attributes,
        #[serde(flatten)]
        children: HashMap<String, String>,
    },
}

impl FvBD {
    pub fn builder(name: &str, tenant_name: &str) -> BDBuilder {
        BDBuilder::new(name, tenant_name)
    }

    fn attributes(&self) -> &Attributes {
        let FvBD::FvBD { attributes, .. } = self;
        attributes
    }

    pub fn get(client: &mut Client) -> Result<GetBDRequestBuilder, Box<dyn std::error::Error>> {
        Ok(GetBDRequestBuilder::new(
            client.get("node/class/fvBD.json")?,
        ))
    }

    pub fn annotation(&self) -> &str {
        &self.attributes().annotation
    }

    pub fn child_action(&self) -> &str {
        &self.attributes().child_action
    }

    pub fn descr(&self) -> &str {
        &self.attributes().descr
    }

    pub fn name(&self) -> &str {
        &self.attributes().name
    }

    pub fn name_alias(&self) -> &str {
        &self.attributes().name_alias
    }

    pub fn owner_key(&self) -> &str {
        &self.attributes().owner_key
    }

    pub fn owner_tag(&self) -> &str {
        &self.attributes().owner_tag
    }
}

pub struct GetBDRequestBuilder<'a> {
    builder: crate::client::GetRequestBuilder<'a>,
}

impl<'a> GetBDRequestBuilder<'a> {
    fn new(builder: crate::client::GetRequestBuilder<'a>) -> Self {
        Self { builder }
    }

    pub async fn send(self) -> Result<Box<[FvBD]>, Box<dyn std::error::Error>> {
        let res = self.builder.send().await?;
        Ok(res
            .into_iter()
            .map(|res| serde_json::from_value(res))
            .collect::<Result<Vec<FvBD>, serde_json::Error>>()?
            .into_boxed_slice())
    }
}

impl<'a> BuilderTrait<'a> for GetBDRequestBuilder<'a> {
    fn renew(builder: crate::GetRequestBuilder<'a>) -> Self {
        Self::new(builder)
    }
    fn builder(self) -> crate::GetRequestBuilder<'a> {
        self.builder
    }
}

pub struct BDBuilder {
    parent: String,
    data: Attributes,
}

impl BDBuilder {
    fn new(name: &str, tenant_name: &str) -> Self {
        Self {
            parent: tenant_name.to_string(),
            data: Attributes {
                optimize_wan_bandwidth: "no".to_string(),
                annotation: String::new(),
                arp_flood: "no".to_string(),
                child_action: String::new(),
                descr: String::new(),
                dn: format!("uni/tn-{}/BD-{}", tenant_name, name),
                ep_move_detect_mode: String::new(),
                ll_addr: String::new(),
                multi_dst_pkt_act: MultiDestinationFlooding::FloodInBD,
                name: name.to_string(),
                name_alias: String::new(),
                owner_key: String::new(),
                owner_tag: String::new(),
                status: String::new(),
                unk_mac_ucast_act: L2UnknownUnicast::HardwareProxy,
                unk_mcast_act: L3UnknownMulticast::Flood,
                userdom: String::new(),
                v6unk_mcast_act: L3UnknownMulticast::Flood,
                vmac: String::new(),
                payload: None,
            },
        }
    }

    fn set_flag(flag: bool) -> String {
        if flag {
            "yes".to_string()
        } else {
            "no".to_string()
        }
    }

    pub fn set_optimeze_wan_bandwidth(mut self, flag: bool) -> Self {
        self.data.optimize_wan_bandwidth = Self::set_flag(flag);
        self
    }

    pub fn set_annotation(mut self, value: impl ToString) -> Self {
        self.data.annotation = value.to_string();
        self
    }

    pub fn set_arp_flood(mut self, flag: bool) -> Self {
        self.data.arp_flood = Self::set_flag(flag);
        self
    }

    pub fn set_descr(mut self, value: impl ToString) -> Self {
        self.data.descr = value.to_string();
        self
    }

    pub fn set_name_alias(mut self, value: impl ToString) -> Self {
        self.data.name_alias = value.to_string();
        self
    }

    pub fn set_l2_unknown_unicast(mut self, value: L2UnknownUnicast) -> Self {
        self.data.unk_mac_ucast_act = value;
        self
    }

    pub fn set_l3_unknown_multicast(mut self, value: L3UnknownMulticast) -> Self {
        self.data.unk_mcast_act = value;
        self
    }

    pub fn set_ipv6_l3_unknown_multicast(mut self, value: L3UnknownMulticast) -> Self {
        self.data.v6unk_mcast_act = value;
        self
    }

    pub fn set_vmac(mut self, value: impl ToString) -> Self {
        self.data.vmac = value.to_string();
        self
    }

    async fn post(
        &mut self,
        client: &Client,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "totalCount": "1",
            "imdata": [{
                "fvTenant": {
                    "attributes": {
                        "name": self.parent,
                        "status": "modified",
                    },
                    "children": [{
                        "fvBD": {
                            "attributes": self.data
                        }
                    }]
                }
            }]
        });
        Ok(client.post("mo/uni.json", &json).await?)
    }

    pub async fn create(
        &mut self,
        client: &Client,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        self.data.status = "created".to_string();
        Ok(self.post(client).await?)
    }

    pub async fn update(
        &mut self,
        client: &Client,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        self.data.status = "modified".to_string();
        Ok(self.post(client).await?)
    }

    pub async fn delete(
        &mut self,
        client: &Client,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        self.data.status = "deleted".to_string();
        Ok(self.post(client).await?)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum L2UnknownUnicast {
    #[serde(rename = "flood")]
    Flood,
    #[serde(rename = "proxy")]
    HardwareProxy,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum L3UnknownMulticast {
    #[serde(rename = "flood")]
    Flood,
    #[serde(rename = "opt-flood")]
    OptimizeFlood,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MultiDestinationFlooding {
    #[serde(rename = "bd-flood")]
    FloodInBD,
    #[serde(rename = "drop")]
    Drop,
    #[serde(rename = "encap-flood")]
    FloodInEncapsulation,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(rename = "OptimizeWanBandwidth")]
    optimize_wan_bandwidth: String,
    annotation: String,
    arp_flood: String,
    child_action: String,
    descr: String,
    dn: String,
    ep_move_detect_mode: String,
    ll_addr: String,
    multi_dst_pkt_act: MultiDestinationFlooding,
    name: String,
    name_alias: String,
    owner_key: String,
    owner_tag: String,
    status: String,
    unk_mac_ucast_act: L2UnknownUnicast,
    unk_mcast_act: L3UnknownMulticast,
    userdom: String,
    v6unk_mcast_act: L3UnknownMulticast,
    vmac: String,
    #[serde(flatten)]
    payload: Option<HashMap<String, String>>,
    // bcastP: String,
    // epClear: String,
    // enable_rogue_except_mac: String,
    // hostBasedRouting: String,
    // intersiteBumTrafficAllow: String,
    // intersiteL2Stretch: String,
    // ipLearning: String,
    // ipv6McastAllow: String,
    // lcOwn: String,
    // limit_ip_learn_to_subnets: String,
    // mcastARPDrop: String,
    // mcastAllow: String,
    // mod_ts: String,
    // mtu: String,
    // pc_tag: String,
    // scope: String,
    // seg: String,
    // type: String,
    // uid: String,
    // unicastRoute: String,
    // configIssues: String,
    // extMngdBy: String,
    // mon_pol_dn: String,
    // mac: String,
}
