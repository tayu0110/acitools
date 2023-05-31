use crate::{error::InvalidSubnetError, BuilderTrait, Client, ResponseFormat};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::IpAddr};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FvSubnet {
    fv_subnet: Inner,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Inner {
    attributes: Attributes,
    #[serde(flatten)]
    children: HashMap<String, String>,
}

impl FvSubnet {
    pub fn builder(
        subnet: IpAddr,
        mask_len: u8,
        bd_name: &str,
        tenant_name: &str,
    ) -> Result<SubnetBuilder, Box<dyn std::error::Error>> {
        Ok(SubnetBuilder::new(subnet, mask_len, bd_name, tenant_name)?)
    }

    pub fn get(client: &mut Client) -> Result<GetSubnetRequestBuilder, Box<dyn std::error::Error>> {
        Ok(GetSubnetRequestBuilder::new(
            client.get("node/class/fvSubnet.json")?,
        ))
    }
}

pub struct GetSubnetRequestBuilder<'a> {
    builder: crate::client::GetRequestBuilder<'a>,
}

impl<'a> GetSubnetRequestBuilder<'a> {
    fn new(builder: crate::client::GetRequestBuilder<'a>) -> Self {
        Self { builder }
    }

    pub async fn send(self) -> Result<Box<[FvSubnet]>, Box<dyn std::error::Error>> {
        let res = self.builder.send().await?;
        let res = serde_json::from_value::<ResponseFormat<FvSubnet>>(res)?.extract();
        Ok(res.into_boxed_slice())
    }
}

impl<'a> BuilderTrait<'a> for GetSubnetRequestBuilder<'a> {
    fn renew(builder: crate::GetRequestBuilder<'a>) -> Self {
        Self::new(builder)
    }
    fn builder(self) -> crate::GetRequestBuilder<'a> {
        self.builder
    }
}

pub struct SubnetBuilder {
    tenant: String,
    bd: String,
    data: Attributes,
}

impl SubnetBuilder {
    fn new(
        gateway_address: IpAddr,
        mask_len: u8,
        bd_name: &str,
        tenant_name: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        match gateway_address {
            IpAddr::V4(address) => {
                if mask_len > 32 {
                    return Err(Box::new(InvalidSubnetError(format!(
                        "Invalid IPv4 Subnet Mask '{}'",
                        mask_len
                    ))));
                }
                let octets = address.octets().iter().fold(0, |s, &v| (s << 8) | v as u32);
                let mask = !0u32 >> mask_len;

                if octets & mask == 0 {
                    return Err(Box::new(InvalidSubnetError(format!(
                        "{}/{} is a network address. This can be used for Gateway Address.",
                        address, mask_len
                    ))));
                } else if octets & mask == mask {
                    return Err(Box::new(InvalidSubnetError(format!(
                        "{}/{} is a broadcast address. This can be used for Gateway Address.",
                        address, mask_len
                    ))));
                }
            }
            IpAddr::V6(address) => {
                if mask_len > 128 {
                    return Err(Box::new(InvalidSubnetError(format!(
                        "Invalid IPv6 Subnet Mask '{}'",
                        mask_len
                    ))));
                }
                let octets = address
                    .octets()
                    .iter()
                    .fold(0u128, |s, &v| (s << 8) | v as u128);
                let mask = !0u128 >> mask_len;

                if octets & mask == 0 {
                    return Err(Box::new(InvalidSubnetError(format!(
                        "{}/{} is a network address. This can be used for Gateway Address.",
                        address, mask_len
                    ))));
                } else if octets & mask == mask {
                    return Err(Box::new(InvalidSubnetError(format!(
                        "{}/{} is a broadcast address. This can be used for Gateway Address.",
                        address, mask_len
                    ))));
                }
            }
        };
        Ok(Self {
            tenant: tenant_name.to_string(),
            bd: bd_name.to_string(),
            data: Attributes {
                annotation: String::new(),
                child_action: String::new(),
                ctrl: String::new(),
                descr: String::new(),
                dn: format!(
                    "uni/tn-{}/BD-{}/subnet-[{}/{}]",
                    tenant_name, bd_name, gateway_address, mask_len
                ),
                ip: format!("{}/{}", gateway_address, mask_len),
                name: String::new(),
                name_alias: String::new(),
                preferred: "no".to_string(),
                scope: String::new(),
                status: String::new(),
                userdom: String::new(),
                virtual_: "no".to_string(),
                payload: None,
            },
        })
    }

    fn set_flag(flag: bool) -> String {
        if flag { "yes" } else { "no" }.to_string()
    }

    pub fn set_annotation(mut self, value: impl ToString) -> Self {
        self.data.annotation = value.to_string();
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

    pub fn make_primary(mut self, flag: bool) -> Self {
        self.data.preferred = Self::set_flag(flag);
        self
    }

    pub fn treat_as_virtual_ip(mut self, flag: bool) -> Self {
        self.data.virtual_ = Self::set_flag(flag);
        self
    }

    async fn post(
        &mut self,
        client: &mut Client,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "totalCount": "1",
            "imdata": [{
                "fvSubnet": {
                    "attributes": self.data,
                }
            }]
        });
        Ok(client
            .post(
                &format!("mo/uni/tn-{}/BD-{}.json", self.tenant, self.bd),
                &json,
            )
            .await?)
    }

    pub async fn create(
        &mut self,
        client: &mut Client,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        self.data.status = "created".to_string();
        Ok(self.post(client).await?)
    }

    pub async fn update(
        &mut self,
        client: &mut Client,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        self.data.status = "modified".to_string();
        Ok(self.post(client).await?)
    }

    pub async fn delete(
        &mut self,
        client: &mut Client,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        self.data.status = "deleted".to_string();
        Ok(self.post(client).await?)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Attributes {
    annotation: String,
    child_action: String,
    ctrl: String,
    descr: String,
    dn: String,
    ip: String,
    name: String,
    name_alias: String,
    preferred: String,
    scope: String,
    status: String,
    userdom: String,
    #[serde(rename = "virtual")]
    virtual_: String,
    #[serde(flatten)]
    payload: Option<HashMap<String, String>>,
    // config_issues: String,
    // debug_message: String,
    // ext_mngd_by: String,
    // #[serde(rename = "ipDPLearning")]
    // ip_dp_learning: String,
    // lc_own: String,
    // mod_ts: String,
    // mon_pol_dn: String,
    // uid: String,
}
