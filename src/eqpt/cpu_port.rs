use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::port;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    descr: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mon_pol_dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    status: ConfigStatus,
    #[serde(rename = "type")]
    r#type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    EqptEgrBytes15Min {},
    EqptEgrBytes1D {},
    EqptEgrBytes1H {},
    EqptEgrBytes1Mo {},
    EqptEgrBytes1Qtr {},
    EqptEgrBytes1W {},
    EqptEgrBytes1Year {},
    EqptEgrBytes5Min {},
    EqptEgrBytesHist15Min {},
    EqptEgrBytesHist1D {},
    EqptEgrBytesHist1H {},
    EqptEgrBytesHist1Mo {},
    EqptEgrBytesHist1Qtr {},
    EqptEgrBytesHist1W {},
    EqptEgrBytesHist1Year {},
    EqptEgrBytesHist5Min {},
    EqptEgrPkts15Min {},
    EqptEgrPkts1D {},
    EqptEgrPkts1H {},
    EqptEgrPkts1Mo {},
    EqptEgrPkts1Qtr {},
    EqptEgrPkts1W {},
    EqptEgrPkts1Year {},
    EqptEgrPkts5Min {},
    EqptEgrPktsHist15Min {},
    EqptEgrPktsHist1D {},
    EqptEgrPktsHist1H {},
    EqptEgrPktsHist1Mo {},
    EqptEgrPktsHist1Qtr {},
    EqptEgrPktsHist1W {},
    EqptEgrPktsHist1Year {},
    EqptEgrPktsHist5Min {},
    EqptEgrTotal15Min {},
    EqptEgrTotal1D {},
    EqptEgrTotal1H {},
    EqptEgrTotal1Mo {},
    EqptEgrTotal1Qtr {},
    EqptEgrTotal1W {},
    EqptEgrTotal1Year {},
    EqptEgrTotal5Min {},
    EqptEgrTotalHist15Min {},
    EqptEgrTotalHist1D {},
    EqptEgrTotalHist1H {},
    EqptEgrTotalHist1Mo {},
    EqptEgrTotalHist1Qtr {},
    EqptEgrTotalHist1W {},
    EqptEgrTotalHist1Year {},
    EqptEgrTotalHist5Min {},
    EqptIngrBytes15Min {},
    EqptIngrBytes1D {},
    EqptIngrBytes1H {},
    EqptIngrBytes1Mo {},
    EqptIngrBytes1Qtr {},
    EqptIngrBytes1W {},
    EqptIngrBytes1Year {},
    EqptIngrBytes5Min {},
    EqptIngrBytesHist15Min {},
    EqptIngrBytesHist1D {},
    EqptIngrBytesHist1H {},
    EqptIngrBytesHist1Mo {},
    EqptIngrBytesHist1Qtr {},
    EqptIngrBytesHist1W {},
    EqptIngrBytesHist1Year {},
    EqptIngrBytesHist5Min {},
    EqptIngrPkts15Min {},
    EqptIngrPkts1D {},
    EqptIngrPkts1H {},
    EqptIngrPkts1Mo {},
    EqptIngrPkts1Qtr {},
    EqptIngrPkts1W {},
    EqptIngrPkts1Year {},
    EqptIngrPkts5Min {},
    EqptIngrPktsHist15Min {},
    EqptIngrPktsHist1D {},
    EqptIngrPktsHist1H {},
    EqptIngrPktsHist1Mo {},
    EqptIngrPktsHist1Qtr {},
    EqptIngrPktsHist1W {},
    EqptIngrPktsHist1Year {},
    EqptIngrPktsHist5Min {},
    EqptIngrTotal15Min {},
    EqptIngrTotal1D {},
    EqptIngrTotal1H {},
    EqptIngrTotal1Mo {},
    EqptIngrTotal1Qtr {},
    EqptIngrTotal1W {},
    EqptIngrTotal1Year {},
    EqptIngrTotal5Min {},
    EqptIngrTotalHist15Min {},
    EqptIngrTotalHist1D {},
    EqptIngrTotalHist1H {},
    EqptIngrTotalHist1Mo {},
    EqptIngrTotalHist1Qtr {},
    EqptIngrTotalHist1W {},
    EqptIngrTotalHist1Year {},
    EqptIngrTotalHist5Min {},
    EqptIngrUnkBytes15Min {},
    EqptIngrUnkBytes1D {},
    EqptIngrUnkBytes1H {},
    EqptIngrUnkBytes1Mo {},
    EqptIngrUnkBytes1Qtr {},
    EqptIngrUnkBytes1W {},
    EqptIngrUnkBytes1Year {},
    EqptIngrUnkBytes5Min {},
    EqptIngrUnkBytesHist15Min {},
    EqptIngrUnkBytesHist1D {},
    EqptIngrUnkBytesHist1H {},
    EqptIngrUnkBytesHist1Mo {},
    EqptIngrUnkBytesHist1Qtr {},
    EqptIngrUnkBytesHist1W {},
    EqptIngrUnkBytesHist1Year {},
    EqptIngrUnkBytesHist5Min {},
    EqptIngrUnkPkts15Min {},
    EqptIngrUnkPkts1D {},
    EqptIngrUnkPkts1H {},
    EqptIngrUnkPkts1Mo {},
    EqptIngrUnkPkts1Qtr {},
    EqptIngrUnkPkts1W {},
    EqptIngrUnkPkts1Year {},
    EqptIngrUnkPkts5Min {},
    EqptIngrUnkPktsHist15Min {},
    EqptIngrUnkPktsHist1D {},
    EqptIngrUnkPktsHist1H {},
    EqptIngrUnkPktsHist1Mo {},
    EqptIngrUnkPktsHist1Qtr {},
    EqptIngrUnkPktsHist1W {},
    EqptIngrUnkPktsHist1Year {},
    EqptIngrUnkPktsHist5Min {},
    EqptLPort(port::EqptLPort),
    FaultCounts {},
    FaultInst {},
    HealthInst {},
}

#[derive(Debug, Clone)]
pub enum EqptCpuPEndpoint {
    ClassAll,
    MoUni,
    MoFc {
        pod: String,
        node: String,
        fcslot: String,
        cpuport: String,
    },
    MoSysFc {
        fcslot: String,
        cpuport: String,
    },
    MoSup {
        pod: String,
        node: String,
        supslot: String,
        cpuport: String,
    },
    MoSysSup {
        supslot: String,
        cpuport: String,
    },
}

impl EndpointScheme for EqptCpuPEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptCpuP.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoFc {
                pod,
                node,
                fcslot,
                cpuport,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/fcslot-{fcslot}/fc/cpuport-{cpuport}.json")),
            Self::MoSysFc {
                fcslot,
                cpuport,
            } => Cow::Owned(format!("mo/sys/ch/fcslot-{fcslot}/fc/cpuport-{cpuport}.json")),
            Self::MoSup {
                pod,
                node,
                supslot,
                cpuport,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/supslot-{supslot}/sup/cpuport-{cpuport}.json")),
            Self::MoSysSup {
                supslot,
                cpuport,
            } => Cow::Owned(format!("mo/sys/ch/supslot-{supslot}/sup/cpuport-{cpuport}.json")),
        }
    }
}

pub type EqptCpuP = AciObject<__internal::EqptCpuP>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptCpuP;
    impl AciObjectScheme for EqptCpuP {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptCpuPEndpoint;
        const CLASS_NAME: &'static str = "eqptCpuP";
    }
}
