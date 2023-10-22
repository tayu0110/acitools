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
    mfg_tm: String,
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
pub enum Endpoint {
    ClassAll,
    MoUni,
    MoExtChCpu {
        pod: String,
        node: String,
        extch: String,
        extchslot: String,
        cpu: String,
        core: String,
    },
    MoSysExtChCpu {
        extch: String,
        extchslot: String,
        cpu: String,
        core: String,
    },
    MoScCpu {
        pod: String,
        node: String,
        scslot: String,
        cpu: String,
        core: String,
    },
    MoSysScCpu {
        scslot: String,
        cpu: String,
        core: String,
    },
    MoFcCpu {
        pod: String,
        node: String,
        fcslot: String,
        cpu: String,
        core: String,
    },
    MoSysFcCpu {
        fcslot: String,
        cpu: String,
        core: String,
    },
    MoLcCpu {
        pod: String,
        node: String,
        lcslot: String,
        cpu: String,
        core: String,
    },
    MoSysLcCpu {
        lcslot: String,
        cpu: String,
        core: String,
    },
    MoSupCpu {
        pod: String,
        node: String,
        supslot: String,
        cpu: String,
        core: String,
    },
    MoSysSupCpu {
        supslot: String,
        cpu: String,
        core: String,
    },
    MoNicCpu {
        pod: String,
        node: String,
        nslot: String,
        nic: String,
        cpu: String,
        core: String,
    },
    MoSysNicCpu {
        nslot: String,
        nic: String,
        cpu: String,
        core: String,
    },
    MoNodeCpu {
        pod: String,
        node: String,
        cpu: String,
        core: String,
    },
    MoSysNodeCpu {
        cpu: String,
        core: String,
    },
}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptCore.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoExtChCpu {
                pod,
                node,
                extch,
                extchslot,
                cpu,
                core,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/extchslot-{extchslot}/extchc/cpu-{cpu}/core-{core}.json")),
            Self::MoSysExtChCpu {
                extch,
                extchslot,
                cpu,
                core,
            } => Cow::Owned(format!("mo/sys/extch-{extch}/extchslot-{extchslot}/extchc/cpu-{cpu}/core-{core}.json")),
            Self::MoScCpu {
                pod,
                node,
                scslot,
                cpu,
                core,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/scslot-{scslot}/sc/cpu-{cpu}/core-{core}.json")),
            Self::MoSysScCpu {
                scslot,
                cpu,
                core,
            } => Cow::Owned(format!("mo/sys/ch/scslot-{scslot}/sc/cpu-{cpu}/core-{core}.json")),
            Self::MoFcCpu {
                pod,
                node,
                fcslot,
                cpu,
                core,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/fcslot-{fcslot}/fc/cpu-{cpu}/core-{core}.json")),
            Self::MoSysFcCpu {
                fcslot,
                cpu,
                core,
            } => Cow::Owned(format!("mo/sys/ch/fcslot-{fcslot}/fc/cpu-{cpu}/core-{core}.json")),
            Self::MoLcCpu {
                pod,
                node,
                lcslot,
                cpu,
                core,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/lcslot-{lcslot}/lc/cpu-{cpu}/core-{core}.json")),
            Self::MoSysLcCpu {
                lcslot,
                cpu,
                core,
            } => Cow::Owned(format!("mo/sys/ch/lcslot-{lcslot}/lc/cpu-{cpu}/core-{core}.json")),
            Self::MoSupCpu {
                pod,
                node,
                supslot,
                cpu,
                core,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/supslot-{supslot}/sup/cpu-{cpu}/core-{core}.json")),
            Self::MoSysSupCpu {
                supslot,
                cpu,
                core,
            } => Cow::Owned(format!("mo/sys/ch/supslot-{supslot}/sup/cpu-{cpu}/core-{core}.json")),
            Self::MoNicCpu {
                pod,
                node,
                nslot,
                nic,
                cpu,
                core,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/nslot-{nslot}/nic-{nic}/cpu-{cpu}/core-{core}.json")),
            Self::MoSysNicCpu {
                nslot,
                nic,
                cpu,
                core,
            } => Cow::Owned(format!("mo/sys/ch/nslot-{nslot}/nic-{nic}/cpu-{cpu}/core-{core}.json")),
            Self::MoNodeCpu {
                pod,
                node,
                cpu,
                core,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/bslot/board/cpu-{cpu}/core-{core}.json")),
            Self::MoSysNodeCpu {
                cpu,
                core,
            } => Cow::Owned(format!("mo/sys/ch/bslot/board/cpu-{cpu}/core-{core}.json")),
        }
    }
}

pub type EqptCore = AciObject<__internal::EqptCore>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptCore;
    impl AciObjectScheme for EqptCore {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "eqptCore";
    }
}
