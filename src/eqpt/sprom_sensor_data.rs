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
    maj_th: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mfg_tm: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    min_th: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    model: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rev: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ser: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    vendor: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {}

#[derive(Debug, Clone)]
pub enum EqptSpSdEndpoint {
    ClassAll,
    MoUni,
    Raw(String),
    MoScSplc {
        pod: String,
        node: String,
        scslot: String,
        spsd: String,
    },
    MoSysScSplc {
        scslot: String,
        spsd: String,
    },
    MoFcSplc {
        pod: String,
        node: String,
        fcslot: String,
        spsd: String,
    },
    MoSysFcSplc {
        fcslot: String,
        spsd: String,
    },
    MoLcSplc {
        pod: String,
        node: String,
        lcslot: String,
        spsd: String,
    },
    MoSysLcSplc {
        lcslot: String,
        spsd: String,
    },
    MoExtChSpsup {
        pod: String,
        node: String,
        extch: String,
        spsd: String,
    },
    MoSysExtChSpsup {
        extch: String,
        spsd: String,
    },
    MoSpsup {
        pod: String,
        node: String,
        supslot: String,
        spsd: String,
    },
    MoSysSpsup {
        supslot: String,
        spsd: String,
    },
    MoScSpsensorblk {
        pod: String,
        node: String,
        scslot: String,
        spsd: String,
    },
    MoSysScSpsensorblk {
        scslot: String,
        spsd: String,
    },
    MoFcSpsensorblk {
        pod: String,
        node: String,
        fcslot: String,
        spsd: String,
    },
    MoSysFcSpsensorblk {
        fcslot: String,
        spsd: String,
    },
    MoLcSpsensorblk {
        pod: String,
        node: String,
        lcslot: String,
        spsd: String,
    },
    MoSysLcSpsensorblk {
        lcslot: String,
        spsd: String,
    },
    MoExtChSpsensorblk {
        pod: String,
        node: String,
        extch: String,
        spsd: String,
    },
    MoSysExtChSpsensorblk {
        extch: String,
        spsd: String,
    },
    MoSpsensorblk {
        pod: String,
        node: String,
        supslot: String,
        spsd: String,
    },
    MoSysSpsensorblk {
        supslot: String,
        spsd: String,
    },
    MoScSplcblk {
        pod: String,
        node: String,
        scslot: String,
        spsd: String,
    },
    MoSysScSplcblk {
        scslot: String,
        spsd: String,
    },
    MoFcSplcblk {
        pod: String,
        node: String,
        fcslot: String,
        spsd: String,
    },
    MoSysFcSplcblk {
        fcslot: String,
        spsd: String,
    },
    MoLcSplcblk {
        pod: String,
        node: String,
        lcslot: String,
        spsd: String,
    },
    MoSysLcSplcblk {
        lcslot: String,
        spsd: String,
    },
    MoExtChSpsupblk {
        pod: String,
        node: String,
        extch: String,
        spsd: String,
    },
    MoSysExtChSpsupblk {
        extch: String,
        spsd: String,
    },
    MoSpsupblk {
        pod: String,
        node: String,
        supslot: String,
        spsd: String,
    },
    MoSysSpsupblk {
        supslot: String,
        spsd: String,
    },
}

impl EndpointScheme for EqptSpSdEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptSpSd.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::Raw(endpoint) => Cow::Owned(format!("{endpoint}")),
            Self::MoScSplc {
                pod,
                node,
                scslot,
                spsd,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/scslot-{scslot}/sc/splc/spsd-{spsd}.json")),
            Self::MoSysScSplc {
                scslot,
                spsd,
            } => Cow::Owned(format!("mo/sys/ch/scslot-{scslot}/sc/splc/spsd-{spsd}.json")),
            Self::MoFcSplc {
                pod,
                node,
                fcslot,
                spsd,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/fcslot-{fcslot}/fc/splc/spsd-{spsd}.json")),
            Self::MoSysFcSplc {
                fcslot,
                spsd,
            } => Cow::Owned(format!("mo/sys/ch/fcslot-{fcslot}/fc/splc/spsd-{spsd}.json")),
            Self::MoLcSplc {
                pod,
                node,
                lcslot,
                spsd,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/lcslot-{lcslot}/lc/splc/spsd-{spsd}.json")),
            Self::MoSysLcSplc {
                lcslot,
                spsd,
            } => Cow::Owned(format!("mo/sys/ch/lcslot-{lcslot}/lc/splc/spsd-{spsd}.json")),
            Self::MoExtChSpsup {
                pod,
                node,
                extch,
                spsd,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/spsup/spsd-{spsd}.json")),
            Self::MoSysExtChSpsup {
                extch,
                spsd,
            } => Cow::Owned(format!("mo/sys/extch-{extch}/spsup/spsd-{spsd}.json")),
            Self::MoSpsup {
                pod,
                node,
                supslot,
                spsd,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/supslot-{supslot}/sup/spsup/spsd-{spsd}.json")),
            Self::MoSysSpsup {
                supslot,
                spsd,
            } => Cow::Owned(format!("mo/sys/ch/supslot-{supslot}/sup/spsup/spsd-{spsd}.json")),
            Self::MoScSpsensorblk {
                pod,
                node,
                scslot,
                spsd,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/scslot-{scslot}/sc/splc/spsensorblk/spsd-{spsd}.json")),
            Self::MoSysScSpsensorblk {
                scslot,
                spsd,
            } => Cow::Owned(format!("mo/sys/ch/scslot-{scslot}/sc/splc/spsensorblk/spsd-{spsd}.json")),
            Self::MoFcSpsensorblk {
                pod,
                node,
                fcslot,
                spsd,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/fcslot-{fcslot}/fc/splc/spsensorblk/spsd-{spsd}.json")),
            Self::MoSysFcSpsensorblk {
                fcslot,
                spsd,
            } => Cow::Owned(format!("mo/sys/ch/fcslot-{fcslot}/fc/splc/spsensorblk/spsd-{spsd}.json")),
            Self::MoLcSpsensorblk {
                pod,
                node,
                lcslot,
                spsd,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/lcslot-{lcslot}/lc/splc/spsensorblk/spsd-{spsd}.json")),
            Self::MoSysLcSpsensorblk {
                lcslot,
                spsd,
            } => Cow::Owned(format!("mo/sys/ch/lcslot-{lcslot}/lc/splc/spsensorblk/spsd-{spsd}.json")),
            Self::MoExtChSpsensorblk {
                pod,
                node,
                extch,
                spsd,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/spsup/spsensorblk/spsd-{spsd}.json")),
            Self::MoSysExtChSpsensorblk {
                extch,
                spsd,
            } => Cow::Owned(format!("mo/sys/extch-{extch}/spsup/spsensorblk/spsd-{spsd}.json")),
            Self::MoSpsensorblk {
                pod,
                node,
                supslot,
                spsd,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/supslot-{supslot}/sup/spsup/spsensorblk/spsd-{spsd}.json")),
            Self::MoSysSpsensorblk {
                supslot,
                spsd,
            } => Cow::Owned(format!("mo/sys/ch/supslot-{supslot}/sup/spsup/spsensorblk/spsd-{spsd}.json")),
            Self::MoScSplcblk {
                pod,
                node,
                scslot,
                spsd,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/scslot-{scslot}/sc/splc/splcblk/spsd-{spsd}.json")),
            Self::MoSysScSplcblk {
                scslot,
                spsd,
            } => Cow::Owned(format!("mo/sys/ch/scslot-{scslot}/sc/splc/splcblk/spsd-{spsd}.json")),
            Self::MoFcSplcblk {
                pod,
                node,
                fcslot,
                spsd,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/fcslot-{fcslot}/fc/splc/splcblk/spsd-{spsd}.json")),
            Self::MoSysFcSplcblk {
                fcslot,
                spsd,
            } => Cow::Owned(format!("mo/sys/ch/fcslot-{fcslot}/fc/splc/splcblk/spsd-{spsd}.json")),
            Self::MoLcSplcblk {
                pod,
                node,
                lcslot,
                spsd,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/lcslot-{lcslot}/lc/splc/splcblk/spsd-{spsd}.json")),
            Self::MoSysLcSplcblk {
                lcslot,
                spsd,
            } => Cow::Owned(format!("mo/sys/ch/lcslot-{lcslot}/lc/splc/splcblk/spsd-{spsd}.json")),
            Self::MoExtChSpsupblk {
                pod,
                node,
                extch,
                spsd,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/spsup/spsupblk/spsd-{spsd}.json")),
            Self::MoSysExtChSpsupblk {
                extch,
                spsd,
            } => Cow::Owned(format!("mo/sys/extch-{extch}/spsup/spsupblk/spsd-{spsd}.json")),
            Self::MoSpsupblk {
                pod,
                node,
                supslot,
                spsd,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/supslot-{supslot}/sup/spsup/spsupblk/spsd-{spsd}.json")),
            Self::MoSysSpsupblk {
                supslot,
                spsd,
            } => Cow::Owned(format!("mo/sys/ch/supslot-{supslot}/sup/spsup/spsupblk/spsd-{spsd}.json")),
        }
    }
}

pub type EqptSpSd = AciObject<__internal::EqptSpSd>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptSpSd;
    impl AciObjectScheme for EqptSpSd {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptSpSdEndpoint;
        const CLASS_NAME: &'static str = "eqptSpSd";
    }
}
