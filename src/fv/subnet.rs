use crate::{AciObject, ConfigStatus, Configurable, EndpointScheme};
use ipnetwork::{IpNetwork, IpNetworkError};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::IpAddr};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    annotation: String,
    child_action: String,
    ctrl: String,
    descr: String,
    #[serde(default)]
    dn: String,
    ip: IpNetwork,
    #[serde(rename = "ipDPLearning")]
    ip_dp_learning: String,
    name: String,
    name_alias: String,
    preferred: String,
    scope: String,
    status: ConfigStatus,
    userdom: String,
    #[serde(rename = "virtual")]
    virtual_: String,
    #[serde(flatten)]
    payload: Option<HashMap<String, String>>,
    // config_issues: String,
    // debug_message: String,
    // ext_mngd_by: String,
    // lc_own: String,
    // mod_ts: String,
    // mon_pol_dn: String,
    // uid: String,
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
    FaultDelegate {},
    FaultInst {},
    FvAssocESgTagSel {},
    FvCepNetCfgPol {},
    FvDisableDPLearning {},
    FvEpAnycast {},
    FvEpNlb {},
    FvEpReachability {},
    FvRsBDSubnetToOut {},
    FvRsBDSubnetToProfile {},
    FvRsNdPfxPol {},
    HealthInst {},
    TagAliasDelInst {},
    TagAliasInst {},
    TagAnnotation {},
    TagExtMngdInst {},
    TagInst {},
    TagTag {},
}

pub enum FvSubnetEndpoint {
    ClassAll,
    ClassTenant {
        tenant: String,
    },
    ClassBD {
        tenant: String,
        bd: String,
    },
    ClassEPG {
        tenant: String,
        ap: String,
        epg: String,
    },
    MoUni,
    MoTenant {
        tenant: String,
        subnet: IpAddr,
        masklen: u8,
    },
    MoBD {
        tenant: String,
        bd: String,
        subnet: IpAddr,
        masklen: u8,
    },
    MoEPG {
        tenant: String,
        ap: String,
        epg: String,
        subnet: IpAddr,
        masklen: u8,
    },
}

impl EndpointScheme for FvSubnetEndpoint {
    fn endpoint(&self) -> std::borrow::Cow<'_, str> {
        match self {
            Self::ClassAll => std::borrow::Cow::Borrowed("node/class/fvSubnet.json"),
            Self::ClassTenant { tenant } => {
                std::borrow::Cow::Owned(format!("node/class/uni/tn-{tenant}/fvSubnet.json"))
            }
            Self::ClassBD { tenant, bd } => {
                std::borrow::Cow::Owned(format!("node/class/uni/tn-{tenant}/bd-{bd}/fvSubnet.json"))
            }
            Self::ClassEPG { tenant, ap, epg } => std::borrow::Cow::Owned(format!(
                "node/class/uni/tn-{tenant}/ap-{ap}/epg-{epg}/fvSubnet.json"
            )),
            Self::MoUni => std::borrow::Cow::Borrowed("mo/uni.json"),
            Self::MoTenant {
                tenant,
                subnet,
                masklen,
            } => std::borrow::Cow::Owned(format!(
                "mo/uni/tn-{tenant}/subnet-[{subnet}/{masklen}].json"
            )),
            Self::MoBD {
                tenant,
                bd,
                subnet,
                masklen,
            } => std::borrow::Cow::Owned(format!(
                "mo/uni/tn-{tenant}/bd-{bd}/subnet-[{subnet}/{masklen}].json"
            )),
            Self::MoEPG {
                tenant,
                ap,
                epg,
                subnet,
                masklen,
            } => std::borrow::Cow::Owned(format!(
                "mo/uni/tn-{tenant}/ap-{ap}/epg-{epg}/subnet-[{subnet}/{masklen}].json"
            )),
        }
    }
}

pub type FvSubnet = AciObject<__internal::FvSubnet>;

impl FvSubnet {
    pub fn new_in_bd(
        subnet: IpAddr,
        masklen: u8,
        tenant: &str,
        bd: &str,
    ) -> Result<Self, IpNetworkError> {
        let ip = IpNetwork::new(subnet, masklen)?;
        Ok(Self {
            attributes: Attributes {
                annotation: String::new(),
                child_action: String::new(),
                ctrl: String::new(),
                descr: String::new(),
                dn: format!("uni/tn-{tenant}/BD-{bd}/subnet-[{ip}]",),
                ip,
                ip_dp_learning: "enabled".to_owned(),
                name: String::new(),
                name_alias: String::new(),
                preferred: "no".to_string(),
                scope: String::new(),
                status: ConfigStatus::None,
                userdom: String::new(),
                virtual_: "no".to_string(),
                payload: None,
            },
            children: vec![],
        })
    }

    pub fn new_in_epg(
        subnet: IpAddr,
        masklen: u8,
        tenant: &str,
        ap: &str,
        epg: &str,
    ) -> Result<Self, IpNetworkError> {
        let ip = IpNetwork::new(subnet, masklen)?;
        Ok(Self {
            attributes: Attributes {
                annotation: String::new(),
                child_action: String::new(),
                ctrl: String::new(),
                descr: String::new(),
                dn: format!("uni/tn-{tenant}/ap-{ap}/epg-{epg}/subnet-[{ip}]",),
                ip,
                ip_dp_learning: "enabled".to_owned(),
                name: String::new(),
                name_alias: String::new(),
                preferred: "no".to_string(),
                scope: String::new(),
                status: ConfigStatus::None,
                userdom: String::new(),
                virtual_: "no".to_string(),
                payload: None,
            },
            children: vec![],
        })
    }

    pub fn set_descr(&mut self, descr: impl ToString) {
        self.attributes.descr = descr.to_string();
    }
}

mod __internal {
    use super::*;
    use crate::AciObjectScheme;

    #[derive(Debug, Clone, Copy)]
    pub struct FvSubnet;

    impl AciObjectScheme for FvSubnet {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = FvSubnetEndpoint;
        const CLASS_NAME: &'static str = "fvSubnet";
    }
}
