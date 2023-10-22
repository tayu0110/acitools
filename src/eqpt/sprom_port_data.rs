use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    cksum: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    len: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    num_pts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    prt_ty: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    sig: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    ver: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {}

#[derive(Debug, Clone)]
pub enum EqptSpPdEndpoint {
    ClassAll,
    MoUni,
    MoScSplc {
        pod: String,
        node: String,
        scslot: String,
    },
    MoSysScSplc {
        scslot: String,
    },
    MoFcSplc {
        pod: String,
        node: String,
        fcslot: String,
    },
    MoSysFcSplc {
        fcslot: String,
    },
    MoLcSplc {
        pod: String,
        node: String,
        lcslot: String,
    },
    MoSysLcSplc {
        lcslot: String,
    },
    MoExtChSpsup {
        pod: String,
        node: String,
        extch: String,
    },
    MoSysExtChSpsup {
        extch: String,
    },
    MoSpsup {
        pod: String,
        node: String,
        supslot: String,
    },
    MoSysSpsup {
        supslot: String,
    },
    MoScSplcblk {
        pod: String,
        node: String,
        scslot: String,
    },
    MoSysScSplcblk {
        scslot: String,
    },
    MoFcSplcblk {
        pod: String,
        node: String,
        fcslot: String,
    },
    MoSysFcSplcblk {
        fcslot: String,
    },
    MoLcSplcblk {
        pod: String,
        node: String,
        lcslot: String,
    },
    MoSysLcSplcblk {
        lcslot: String,
    },
    MoExtChSpsupblk {
        pod: String,
        node: String,
        extch: String,
    },
    MoSysExtChSpsupblk {
        extch: String,
    },
    MoSpsupblk {
        pod: String,
        node: String,
        supslot: String,
    },
    MoSysSpsupblk {
        supslot: String,
    },
}

impl EndpointScheme for EqptSpPdEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptSpPd.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoScSplc {
                pod,
                node,
                scslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/scslot-{scslot}/sc/splc/sppd.json")),
            Self::MoSysScSplc {
                scslot,
            } => Cow::Owned(format!("mo/sys/ch/scslot-{scslot}/sc/splc/sppd.json")),
            Self::MoFcSplc {
                pod,
                node,
                fcslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/fcslot-{fcslot}/fc/splc/sppd.json")),
            Self::MoSysFcSplc {
                fcslot,
            } => Cow::Owned(format!("mo/sys/ch/fcslot-{fcslot}/fc/splc/sppd.json")),
            Self::MoLcSplc {
                pod,
                node,
                lcslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/lcslot-{lcslot}/lc/splc/sppd.json")),
            Self::MoSysLcSplc {
                lcslot,
            } => Cow::Owned(format!("mo/sys/ch/lcslot-{lcslot}/lc/splc/sppd.json")),
            Self::MoExtChSpsup {
                pod,
                node,
                extch,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/spsup/sppd.json")),
            Self::MoSysExtChSpsup {
                extch,
            } => Cow::Owned(format!("mo/sys/extch-{extch}/spsup/sppd.json")),
            Self::MoSpsup {
                pod,
                node,
                supslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/supslot-{supslot}/sup/spsup/sppd.json")),
            Self::MoSysSpsup {
                supslot,
            } => Cow::Owned(format!("mo/sys/ch/supslot-{supslot}/sup/spsup/sppd.json")),
            Self::MoScSplcblk {
                pod,
                node,
                scslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/scslot-{scslot}/sc/splc/splcblk/sppd.json")),
            Self::MoSysScSplcblk {
                scslot,
            } => Cow::Owned(format!("mo/sys/ch/scslot-{scslot}/sc/splc/splcblk/sppd.json")),
            Self::MoFcSplcblk {
                pod,
                node,
                fcslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/fcslot-{fcslot}/fc/splc/splcblk/sppd.json")),
            Self::MoSysFcSplcblk {
                fcslot,
            } => Cow::Owned(format!("mo/sys/ch/fcslot-{fcslot}/fc/splc/splcblk/sppd.json")),
            Self::MoLcSplcblk {
                pod,
                node,
                lcslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/lcslot-{lcslot}/lc/splc/splcblk/sppd.json")),
            Self::MoSysLcSplcblk {
                lcslot,
            } => Cow::Owned(format!("mo/sys/ch/lcslot-{lcslot}/lc/splc/splcblk/sppd.json")),
            Self::MoExtChSpsupblk {
                pod,
                node,
                extch,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/spsup/spsupblk/sppd.json")),
            Self::MoSysExtChSpsupblk {
                extch,
            } => Cow::Owned(format!("mo/sys/extch-{extch}/spsup/spsupblk/sppd.json")),
            Self::MoSpsupblk {
                pod,
                node,
                supslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/supslot-{supslot}/sup/spsup/spsupblk/sppd.json")),
            Self::MoSysSpsupblk {
                supslot,
            } => Cow::Owned(format!("mo/sys/ch/supslot-{supslot}/sup/spsup/spsupblk/sppd.json")),
        }
    }
}

pub type EqptSpPd = AciObject<__internal::EqptSpPd>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptSpPd;
    impl AciObjectScheme for EqptSpPd {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptSpPdEndpoint;
        const CLASS_NAME: &'static str = "eqptSpPd";
    }
}
