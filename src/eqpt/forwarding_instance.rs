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
    MoExtChAsic {
        pod: String,
        node: String,
        extch: String,
        extchslot: String,
        asic: String,
        fwdinst: String,
    },
    MoExtChSysAsic {
        extch: String,
        extchslot: String,
        asic: String,
        fwdinst: String,
    },
    MoScAsic {
        pod: String,
        node: String,
        scslot: String,
        asic: String,
        fwdinst: String,
    },
    MoScSysAsic {
        scslot: String,
        asic: String,
        fwdinst: String,
    },
    MoFcAsic {
        pod: String,
        node: String,
        fcslot: String,
        asic: String,
        fwdinst: String,
    },
    MoSysFcAsic {
        fcslot: String,
        asic: String,
        fwdinst: String,
    },
    MoLcAsic {
        pod: String,
        node: String,
        lcslot: String,
        asic: String,
        fwdinst: String,
    },
    MoSysLcAsic {
        lcslot: String,
        asic: String,
        fwdinst: String,
    },
    MoSupAsic {
        pod: String,
        node: String,
        supslot: String,
        asic: String,
        fwdinst: String,
    },
    MoSysSupAsic {
        supslot: String,
        asic: String,
        fwdinst: String,
    },
    MoNicAsic {
        pod: String,
        node: String,
        nslot: String,
        nic: String,
        asic: String,
        fwdinst: String,
    },
    MoSysNicAsic {
        nslot: String,
        nic: String,
        asic: String,
        fwdinst: String,
    },
    MoNodeAsic {
        pod: String,
        node: String,
        asic: String,
        fwdinst: String,
    },
    MoSysNodeAsic {
        asic: String,
        fwdinst: String,
    },
}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptFwdInst.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoExtChAsic {
                pod,
                node,
                extch,
                extchslot,
                asic,
                fwdinst,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/extchslot-{extchslot}/extchc/asic-{asic}/fwdinst-{fwdinst}.json")),
            Self::MoExtChSysAsic {
                extch,
                extchslot,
                asic,
                fwdinst,
            } => Cow::Owned(format!("mo/sys/extch-{extch}/extchslot-{extchslot}/extchc/asic-{asic}/fwdinst-{fwdinst}.json")),
            Self::MoScAsic {
                pod,
                node,
                scslot,
                asic,
                fwdinst,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/scslot-{scslot}/sc/asic-{asic}/fwdinst-{fwdinst}.json")),
            Self::MoScSysAsic {
                scslot,
                asic,
                fwdinst,
            } => Cow::Owned(format!("mo/sys/ch/scslot-{scslot}/sc/asic-{asic}/fwdinst-{fwdinst}.json")),
            Self::MoFcAsic {
                pod,
                node,
                fcslot,
                asic,
                fwdinst,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/fcslot-{fcslot}/fc/asic-{asic}/fwdinst-{fwdinst}.json")),
            Self::MoSysFcAsic {
                fcslot,
                asic,
                fwdinst,
            } => Cow::Owned(format!("mo/sys/ch/fcslot-{fcslot}/fc/asic-{asic}/fwdinst-{fwdinst}.json")),
            Self::MoLcAsic {
                pod,
                node,
                lcslot,
                asic,
                fwdinst,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/lcslot-{lcslot}/lc/asic-{asic}/fwdinst-{fwdinst}.json")),
            Self::MoSysLcAsic {
                lcslot,
                asic,
                fwdinst,
            } => Cow::Owned(format!("mo/sys/ch/lcslot-{lcslot}/lc/asic-{asic}/fwdinst-{fwdinst}.json")),
            Self::MoSupAsic {
                pod,
                node,
                supslot,
                asic,
                fwdinst,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/supslot-{supslot}/sup/asic-{asic}/fwdinst-{fwdinst}.json")),
            Self::MoSysSupAsic {
                supslot,
                asic,
                fwdinst,
            } => Cow::Owned(format!("mo/sys/ch/supslot-{supslot}/sup/asic-{asic}/fwdinst-{fwdinst}.json")),
            Self::MoNicAsic {
                pod,
                node,
                nslot,
                nic,
                asic,
                fwdinst,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/nslot-{nslot}/nic-{nic}/asic-{asic}/fwdinst-{fwdinst}.json")),
            Self::MoSysNicAsic {
                nslot,
                nic,
                asic,
                fwdinst,
            } => Cow::Owned(format!("mo/sys/ch/nslot-{nslot}/nic-{nic}/asic-{asic}/fwdinst-{fwdinst}.json")),
            Self::MoNodeAsic {
                pod,
                node,
                asic,
                fwdinst,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/bslot/board/asic-{asic}/fwdinst-{fwdinst}.json")),
            Self::MoSysNodeAsic {
                asic,
                fwdinst,
            } => Cow::Owned(format!("mo/sys/ch/bslot/board/asic-{asic}/fwdinst-{fwdinst}.json")),
        }
    }
}

pub type EqptFwdInst = AciObject<__internal::EqptFwdInst>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptFwdInst;
    impl AciObjectScheme for EqptFwdInst {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "eqptFwdInst";
    }
}
