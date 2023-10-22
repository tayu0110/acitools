use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    cimc_version: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    descr: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    dir: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    max_speed: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mfg_tm: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    model: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mon_pol_dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    oper_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rev: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ser: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    speed: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    v_id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    vendor: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    EqptFanStats15Min {},
    EqptFanStats1D {},
    EqptFanStats1H {},
    EqptFanStats1Mo {},
    EqptFanStats1Qtr {},
    EqptFanStats1W {},
    EqptFanStats1Year {},
    EqptFanStats5Min {},
    EqptFanStatsHist15Min {},
    EqptFanStatsHist1D {},
    EqptFanStatsHist1H {},
    EqptFanStatsHist1Mo {},
    EqptFanStatsHist1Qtr {},
    EqptFanStatsHist1W {},
    EqptFanStatsHist1Year {},
    EqptFanStatsHist5Min {},
    FaultCounts {},
    FaultInst {},
    HealthInst {},
}

#[derive(Debug, Clone)]
pub enum EqptFanEndpoint {
    ClassAll,
    MoUni,
    MoExtChFt {
        pod: String,
        node: String,
        extch: String,
        ftslot: String,
        fan: String,
    },
    MoSysExtChFt {
        extch: String,
        ftslot: String,
        fan: String,
    },
    MoFt {
        pod: String,
        node: String,
        ftslot: String,
        fan: String,
    },
    MoSysFt {
        ftslot: String,
        fan: String,
    },
    MoExtChPsu {
        pod: String,
        node: String,
        extch: String,
        psuslot: String,
        fan: String,
    },
    MoSysExtChPsu {
        extch: String,
        psuslot: String,
        fan: String,
    },
    MoPsu {
        pod: String,
        node: String,
        psuslot: String,
        fan: String,
    },
    MoSysPsu {
        psuslot: String,
        fan: String,
    },
}

impl EndpointScheme for EqptFanEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptFan.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoExtChFt {
                pod,
                node,
                extch,
                ftslot,
                fan,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/ftslot-{ftslot}/ft/fan-{fan}.json")),
            Self::MoSysExtChFt {
                extch,
                ftslot,
                fan,
            } => Cow::Owned(format!("mo/sys/extch-{extch}/ftslot-{ftslot}/ft/fan-{fan}.json")),
            Self::MoFt {
                pod,
                node,
                ftslot,
                fan,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/ftslot-{ftslot}/ft/fan-{fan}.json")),
            Self::MoSysFt {
                ftslot,
                fan,
            } => Cow::Owned(format!("mo/sys/ch/ftslot-{ftslot}/ft/fan-{fan}.json")),
            Self::MoExtChPsu {
                pod,
                node,
                extch,
                psuslot,
                fan,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/psuslot-{psuslot}/psu/fan-{fan}.json")),
            Self::MoSysExtChPsu {
                extch,
                psuslot,
                fan,
            } => Cow::Owned(format!("mo/sys/extch-{extch}/psuslot-{psuslot}/psu/fan-{fan}.json")),
            Self::MoPsu {
                pod,
                node,
                psuslot,
                fan,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/psuslot-{psuslot}/psu/fan-{fan}.json")),
            Self::MoSysPsu {
                psuslot,
                fan,
            } => Cow::Owned(format!("mo/sys/ch/psuslot-{psuslot}/psu/fan-{fan}.json")),
        }
    }
}

pub type EqptFan = AciObject<__internal::EqptFan>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptFan;
    impl AciObjectScheme for EqptFan {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptFanEndpoint;
        const CLASS_NAME: &'static str = "eqptFan";
    }
}
