use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    admin_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    cimc_version: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    color: String,
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
    oper_st: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rev: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ser: String,
    status: ConfigStatus,
    #[serde(rename = "type")]
    r#type: String,
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
    MoCh {
        pod: String,
        node: String,
        locled: String,
    },
    MoSysCh {
        locled: String,
    },
    MoExtChFt {
        pod: String,
        node: String,
        extch: String,
        ftslot: String,
        locled: String,
    },
    MoSysExtChFt {
        extch: String,
        ftslot: String,
        locled: String,
    },
    MoFt {
        pod: String,
        node: String,
        ftslot: String,
        locled: String,
    },
    MoSysFt {
        ftslot: String,
        locled: String,
    },
    MoSc {
        pod: String,
        node: String,
        scslot: String,
        locled: String,
    },
    MoSysSc {
        scslot: String,
        locled: String,
    },
    MoFc {
        pod: String,
        node: String,
        fcslot: String,
        locled: String,
    },
    MoSysFc {
        fcslot: String,
        locled: String,
    },
    MoLc {
        pod: String,
        node: String,
        lcslot: String,
        locled: String,
    },
    MoSysLc {
        lcslot: String,
        locled: String,
    },
    MoSup {
        pod: String,
        node: String,
        supslot: String,
        locled: String,
    },
    MoSysSup {
        supslot: String,
        locled: String,
    },
    MoLeafport {
        pod: String,
        node: String,
        lcslot: String,
        leafport: String,
        locled: String,
    },
    MoSysLeafport {
        lcslot: String,
        leafport: String,
        locled: String,
    },
    MoFabport {
        pod: String,
        node: String,
        lcslot: String,
        fabport: String,
        locled: String,
    },
    MoSysFabport {
        lcslot: String,
        fabport: String,
        locled: String,
    },
    MoMgmt {
        pod: String,
        node: String,
        supslot: String,
        mgmt: String,
        locled: String,
    },
    MoSysMgmt {
        supslot: String,
        mgmt: String,
        locled: String,
    },
}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptLocLed.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoCh {
                pod,
                node,
                locled,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/locled-{locled}.json")),
            Self::MoSysCh {
                locled,
            } => Cow::Owned(format!("mo/sys/ch/locled-{locled}.json")),
            Self::MoExtChFt {
                pod,
                node,
                extch,
                ftslot,
                locled,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/ftslot-{ftslot}/ft/locled-{locled}.json")),
            Self::MoSysExtChFt {
                extch,
                ftslot,
                locled,
            } => Cow::Owned(format!("mo/sys/extch-{extch}/ftslot-{ftslot}/ft/locled-{locled}.json")),
            Self::MoFt {
                pod,
                node,
                ftslot,
                locled,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/ftslot-{ftslot}/ft/locled-{locled}.json")),
            Self::MoSysFt {
                ftslot,
                locled,
            } => Cow::Owned(format!("mo/sys/ch/ftslot-{ftslot}/ft/locled-{locled}.json")),
            Self::MoSc {
                pod,
                node,
                scslot,
                locled,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/scslot-{scslot}/sc/locled-{locled}.json")),
            Self::MoSysSc {
                scslot,
                locled,
            } => Cow::Owned(format!("mo/sys/ch/scslot-{scslot}/sc/locled-{locled}.json")),
            Self::MoFc {
                pod,
                node,
                fcslot,
                locled,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/fcslot-{fcslot}/fc/locled-{locled}.json")),
            Self::MoSysFc {
                fcslot,
                locled,
            } => Cow::Owned(format!("mo/sys/ch/fcslot-{fcslot}/fc/locled-{locled}.json")),
            Self::MoLc {
                pod,
                node,
                lcslot,
                locled,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/lcslot-{lcslot}/lc/locled-{locled}.json")),
            Self::MoSysLc {
                lcslot,
                locled,
            } => Cow::Owned(format!("mo/sys/ch/lcslot-{lcslot}/lc/locled-{locled}.json")),
            Self::MoSup {
                pod,
                node,
                supslot,
                locled,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/supslot-{supslot}/sup/locled-{locled}.json")),
            Self::MoSysSup {
                supslot,
                locled,
            } => Cow::Owned(format!("mo/sys/ch/supslot-{supslot}/sup/locled-{locled}.json")),
            Self::MoLeafport {
                pod,
                node,
                lcslot,
                leafport,
                locled,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/lcslot-{lcslot}/lc/leafport-{leafport}/locled-{locled}.json")),
            Self::MoSysLeafport {
                lcslot,
                leafport,
                locled,
            } => Cow::Owned(format!("mo/sys/ch/lcslot-{lcslot}/lc/leafport-{leafport}/locled-{locled}.json")),
            Self::MoFabport {
                pod,
                node,
                lcslot,
                fabport,
                locled,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/lcslot-{lcslot}/lc/fabport-{fabport}/locled-{locled}.json")),
            Self::MoSysFabport {
                lcslot,
                fabport,
                locled,
            } => Cow::Owned(format!("mo/sys/ch/lcslot-{lcslot}/lc/fabport-{fabport}/locled-{locled}.json")),
            Self::MoMgmt {
                pod,
                node,
                supslot,
                mgmt,
                locled,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/supslot-{supslot}/sup/mgmt-{mgmt}/locled-{locled}.json")),
            Self::MoSysMgmt {
                supslot,
                mgmt,
                locled,
            } => Cow::Owned(format!("mo/sys/ch/supslot-{supslot}/sup/mgmt-{mgmt}/locled-{locled}.json")),
        }
    }
}

pub type EqptLocLed = AciObject<__internal::EqptLocLed>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptLocLed;
    impl AciObjectScheme for EqptLocLed {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "eqptLocLed";
    }
}
