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
        indled: String,
    },
    MoSysCh {
        indled: String,
    },
    MoExtChFt {
        pod: String,
        node: String,
        extch: String,
        ftslot: String,
        indled: String,
    },
    MoSysExtChFt {
        extch: String,
        ftslot: String,
        indled: String,
    },
    MoFt {
        pod: String,
        node: String,
        ftslot: String,
        indled: String,
    },
    MoSysFt {
        ftslot: String,
        indled: String,
    },
    MoSc {
        pod: String,
        node: String,
        scslot: String,
        indled: String,
    },
    MoSysSc {
        scslot: String,
        indled: String,
    },
    MoFc {
        pod: String,
        node: String,
        fcslot: String,
        indled: String,
    },
    MoSysFc {
        fcslot: String,
        indled: String,
    },
    MoLc {
        pod: String,
        node: String,
        lcslot: String,
        indled: String,
    },
    MoSysLc {
        lcslot: String,
        indled: String,
    },
    MoSup {
        pod: String,
        node: String,
        supslot: String,
        indled: String,
    },
    MoSysSup {
        supslot: String,
        indled: String,
    },
    MoLeafport {
        pod: String,
        node: String,
        lcslot: String,
        leafport: String,
        indled: String,
    },
    MoSysLeafport {
        lcslot: String,
        leafport: String,
        indled: String,
    },
    MoFabport {
        pod: String,
        node: String,
        lcslot: String,
        fabport: String,
        indled: String,
    },
    MoSysFabport {
        lcslot: String,
        fabport: String,
        indled: String,
    },
    MoMgmt {
        pod: String,
        node: String,
        supslot: String,
        mgmt: String,
        indled: String,
    },
    MoSysMgmt {
        supslot: String,
        mgmt: String,
        indled: String,
    },
    MoExtChPsu {
        pod: String,
        node: String,
        extch: String,
        psuslot: String,
        indled: String,
    },
    MoSysExtChPsu {
        extch: String,
        psuslot: String,
        indled: String,
    },
    MoPsu {
        pod: String,
        node: String,
        psuslot: String,
        indled: String,
    },
    MoSysPsu {
        psuslot: String,
        indled: String,
    },
}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptIndLed.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoCh {
                pod,
                node,
                indled,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/indled-{indled}.json")),
            Self::MoSysCh {
                indled,
            } => Cow::Owned(format!("mo/sys/ch/indled-{indled}.json")),
            Self::MoExtChFt {
                pod,
                node,
                extch,
                ftslot,
                indled,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/ftslot-{ftslot}/ft/indled-{indled}.json")),
            Self::MoSysExtChFt {
                extch,
                ftslot,
                indled,
            } => Cow::Owned(format!("mo/sys/extch-{extch}/ftslot-{ftslot}/ft/indled-{indled}.json")),
            Self::MoFt {
                pod,
                node,
                ftslot,
                indled,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/ftslot-{ftslot}/ft/indled-{indled}.json")),
            Self::MoSysFt {
                ftslot,
                indled,
            } => Cow::Owned(format!("mo/sys/ch/ftslot-{ftslot}/ft/indled-{indled}.json")),
            Self::MoSc {
                pod,
                node,
                scslot,
                indled,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/scslot-{scslot}/sc/indled-{indled}.json")),
            Self::MoSysSc {
                scslot,
                indled,
            } => Cow::Owned(format!("mo/sys/ch/scslot-{scslot}/sc/indled-{indled}.json")),
            Self::MoFc {
                pod,
                node,
                fcslot,
                indled,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/fcslot-{fcslot}/fc/indled-{indled}.json")),
            Self::MoSysFc {
                fcslot,
                indled,
            } => Cow::Owned(format!("mo/sys/ch/fcslot-{fcslot}/fc/indled-{indled}.json")),
            Self::MoLc {
                pod,
                node,
                lcslot,
                indled,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/lcslot-{lcslot}/lc/indled-{indled}.json")),
            Self::MoSysLc {
                lcslot,
                indled,
            } => Cow::Owned(format!("mo/sys/ch/lcslot-{lcslot}/lc/indled-{indled}.json")),
            Self::MoSup {
                pod,
                node,
                supslot,
                indled,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/supslot-{supslot}/sup/indled-{indled}.json")),
            Self::MoSysSup {
                supslot,
                indled,
            } => Cow::Owned(format!("mo/sys/ch/supslot-{supslot}/sup/indled-{indled}.json")),
            Self::MoLeafport {
                pod,
                node,
                lcslot,
                leafport,
                indled,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/lcslot-{lcslot}/lc/leafport-{leafport}/indled-{indled}.json")),
            Self::MoSysLeafport {
                lcslot,
                leafport,
                indled,
            } => Cow::Owned(format!("mo/sys/ch/lcslot-{lcslot}/lc/leafport-{leafport}/indled-{indled}.json")),
            Self::MoFabport {
                pod,
                node,
                lcslot,
                fabport,
                indled,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/lcslot-{lcslot}/lc/fabport-{fabport}/indled-{indled}.json")),
            Self::MoSysFabport {
                lcslot,
                fabport,
                indled,
            } => Cow::Owned(format!("mo/sys/ch/lcslot-{lcslot}/lc/fabport-{fabport}/indled-{indled}.json")),
            Self::MoMgmt {
                pod,
                node,
                supslot,
                mgmt,
                indled,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/supslot-{supslot}/sup/mgmt-{mgmt}/indled-{indled}.json")),
            Self::MoSysMgmt {
                supslot,
                mgmt,
                indled,
            } => Cow::Owned(format!("mo/sys/ch/supslot-{supslot}/sup/mgmt-{mgmt}/indled-{indled}.json")),
            Self::MoExtChPsu {
                pod,
                node,
                extch,
                psuslot,
                indled,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/psuslot-{psuslot}/psu/indled-{indled}.json")),
            Self::MoSysExtChPsu {
                extch,
                psuslot,
                indled,
            } => Cow::Owned(format!("mo/sys/extch-{extch}/psuslot-{psuslot}/psu/indled-{indled}.json")),
            Self::MoPsu {
                pod,
                node,
                psuslot,
                indled,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/psuslot-{psuslot}/psu/indled-{indled}.json")),
            Self::MoSysPsu {
                psuslot,
                indled,
            } => Cow::Owned(format!("mo/sys/ch/psuslot-{psuslot}/psu/indled-{indled}.json")),
        }
    }
}

pub type EqptIndLed = AciObject<__internal::EqptIndLed>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptIndLed;
    impl AciObjectScheme for EqptIndLed {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "eqptIndLed";
    }
}
