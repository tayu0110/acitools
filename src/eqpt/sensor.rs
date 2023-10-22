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
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    major_thresh: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mfg_tm: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    minor_thresh: String,
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
    status: ConfigStatus,
    #[serde(rename = "type", skip_serializing_if = "String::is_empty")]
    r#type: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    value: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    vendor: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    EqptTemp15Min {},
    EqptTemp1D {},
    EqptTemp1H {},
    EqptTemp1Mo {},
    EqptTemp1Qtr {},
    EqptTemp1W {},
    EqptTemp1Year {},
    EqptTemp5Min {},
    EqptTempHist15Min {},
    EqptTempHist1D {},
    EqptTempHist1H {},
    EqptTempHist1Mo {},
    EqptTempHist1Qtr {},
    EqptTempHist1W {},
    EqptTempHist1Year {},
    EqptTempHist5Min {},
    FaultCounts {},
    FaultInst {},
    HealthInst {},
}

#[derive(Debug, Clone)]
pub enum EqptSensorEndpoint {
    ClassAll,
    MoUni,
    MoExtchc {
        pod: String,
        node: String,
        extch: String,
        extchslot: String,
        sensor: String,
    },
    MoSysExtchc {
        extch: String,
        extchslot: String,
        sensor: String,
    },
    MoSc {
        pod: String,
        node: String,
        scslot: String,
        sensor: String,
    },
    MoSysSc {
        scslot: String,
        sensor: String,
    },
    MoFc {
        pod: String,
        node: String,
        fcslot: String,
        sensor: String,
    },
    MoSysFc {
        fcslot: String,
        sensor: String,
    },
    MoLc {
        pod: String,
        node: String,
        lcslot: String,
        sensor: String,
    },
    MoSysLc {
        lcslot: String,
        sensor: String,
    },
    MoSup {
        pod: String,
        node: String,
        supslot: String,
        sensor: String,
    },
    MoSysSup {
        supslot: String,
        sensor: String,
    },
    MoNic {
        pod: String,
        node: String,
        nslot: String,
        nic: String,
        sensor: String,
    },
    MoSysNic {
        nslot: String,
        nic: String,
        sensor: String,
    },
    MoBoard {
        pod: String,
        node: String,
        sensor: String,
    },
    MoSysBoard {
        sensor: String,
    },
}

impl EndpointScheme for EqptSensorEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptSensor.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoExtchc {
                pod,
                node,
                extch,
                extchslot,
                sensor,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/extchslot-{extchslot}/extchc/sensor-{sensor}.json")),
            Self::MoSysExtchc {
                extch,
                extchslot,
                sensor,
            } => Cow::Owned(format!("mo/sys/extch-{extch}/extchslot-{extchslot}/extchc/sensor-{sensor}.json")),
            Self::MoSc {
                pod,
                node,
                scslot,
                sensor,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/scslot-{scslot}/sc/sensor-{sensor}.json")),
            Self::MoSysSc {
                scslot,
                sensor,
            } => Cow::Owned(format!("mo/sys/ch/scslot-{scslot}/sc/sensor-{sensor}.json")),
            Self::MoFc {
                pod,
                node,
                fcslot,
                sensor,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/fcslot-{fcslot}/fc/sensor-{sensor}.json")),
            Self::MoSysFc {
                fcslot,
                sensor,
            } => Cow::Owned(format!("mo/sys/ch/fcslot-{fcslot}/fc/sensor-{sensor}.json")),
            Self::MoLc {
                pod,
                node,
                lcslot,
                sensor,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/lcslot-{lcslot}/lc/sensor-{sensor}.json")),
            Self::MoSysLc {
                lcslot,
                sensor,
            } => Cow::Owned(format!("mo/sys/ch/lcslot-{lcslot}/lc/sensor-{sensor}.json")),
            Self::MoSup {
                pod,
                node,
                supslot,
                sensor,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/supslot-{supslot}/sup/sensor-{sensor}.json")),
            Self::MoSysSup {
                supslot,
                sensor,
            } => Cow::Owned(format!("mo/sys/ch/supslot-{supslot}/sup/sensor-{sensor}.json")),
            Self::MoNic {
                pod,
                node,
                nslot,
                nic,
                sensor,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/nslot-{nslot}/nic-{nic}/sensor-{sensor}.json")),
            Self::MoSysNic {
                nslot,
                nic,
                sensor,
            } => Cow::Owned(format!("mo/sys/ch/nslot-{nslot}/nic-{nic}/sensor-{sensor}.json")),
            Self::MoBoard {
                pod,
                node,
                sensor,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/bslot/board/sensor-{sensor}.json")),
            Self::MoSysBoard {
                sensor,
            } => Cow::Owned(format!("mo/sys/ch/bslot/board/sensor-{sensor}.json")),
        }
    }
}

pub type EqptSensor = AciObject<__internal::EqptSensor>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptSensor;
    impl AciObjectScheme for EqptSensor {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptSensorEndpoint;
        const CLASS_NAME: &'static str = "eqptSensor";
    }
}
