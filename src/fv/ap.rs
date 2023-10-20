use crate::{AciObject, ConfigStatus, Configurable, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::aepg;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    annotation: String,
    child_action: String,
    descr: String,
    dn: String,
    name: String,
    name_alias: String,
    owner_key: String,
    owner_tag: String,
    prio: QoSClass,
    status: ConfigStatus,
    userdom: String,
    #[serde(flatten)]
    payload: Option<HashMap<String, String>>,
    // extMngdBy: String,
    // lcOwn: String,
    // modTs: String,
    // monPolDn: String,
    // uid: String,
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
#[serde(rename_all = "lowercase")]
pub enum QoSClass {
    Unspecified,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    AaaRbacAnnotation {},
    FaultCounts {},
    FaultDelegate {},
    FaultInst {},
    FvAEPg(aepg::FvAEPg),
    FvESg {},
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
    FvRsApMonPol {},
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
    TagAliasDelInst {},
    TagAliasInst {},
    TagAnnotation {},
    TagExtMngdInst {},
    TagInst {},
    TagTag {},
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
    VnsSvcPol {},
}

pub enum FvApEndpoint {
    ClassAll,
    ClassTenant { tenant: String },
    MoUni,
    MoAp { tenant: String, ap: String },
}

impl EndpointScheme for FvApEndpoint {
    fn endpoint(&self) -> std::borrow::Cow<'_, str> {
        match self {
            Self::ClassAll => std::borrow::Cow::Borrowed("node/class/fvAp.json"),
            Self::ClassTenant { tenant } => {
                std::borrow::Cow::Owned(format!("node/class/tn-{tenant}/fvAp.json"))
            }
            Self::MoUni => std::borrow::Cow::Borrowed("mo/uni.json"),
            Self::MoAp { tenant, ap } => {
                std::borrow::Cow::Owned(format!("mo/uni/tn-{tenant}/ap-{ap}.json"))
            }
        }
    }
}

pub type FvAp = AciObject<__internal::FvAp>;

impl FvAp {
    pub fn new(name: &str, tenant: &str) -> Self {
        Self {
            attributes: Attributes {
                annotation: String::new(),
                child_action: String::new(),
                descr: String::new(),
                dn: format!("uni/tn-{}/ap-{}", tenant, name),
                name: name.to_string(),
                name_alias: String::new(),
                owner_key: String::new(),
                owner_tag: String::new(),
                prio: QoSClass::Unspecified,
                status: ConfigStatus::None,
                userdom: String::new(),
                payload: None,
            },
            children: vec![],
        }
    }

    pub fn set_descr(&mut self, descr: impl ToString) {
        self.attributes.descr = descr.to_string();
    }

    pub fn set_qos_class(&mut self, class: QoSClass) {
        self.attributes.prio = class;
    }
}

mod __internal {
    use super::*;
    use crate::AciObjectScheme;

    #[derive(Debug, Clone, Copy)]
    pub struct FvAp;

    impl AciObjectScheme for FvAp {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = FvApEndpoint;
        const CLASS_NAME: &'static str = "fvAp";
    }
}
