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
    #[serde(skip_serializing_if = "String::is_empty")]
    clei: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    count: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    eng_bits: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    hw_rev_maj: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    hw_rev_min: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    len: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    major: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mfg_bits: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mfg_dev: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    minor: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    oem: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    p_rev: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pd_num: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    prt_num: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pwr_con: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ram_fl: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ser_num: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    sig: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    size: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    vdr_id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ver: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {}

#[derive(Debug, Clone)]
pub enum EqptSpCmnBlkEndpoint {
    ClassAll,
    MoUni,
    MoNodeScSplc {
        pod: String,
        node: String,
        scslot: String,
    },
    MoSysScSplc {
        scslot: String,
    },
    MoNodeFcSplc {
        pod: String,
        node: String,
        fcslot: String,
    },
    MoSysFcSplc {
        fcslot: String,
    },
    MoNodeLcSplc {
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
    MoExtChSpbp {
        pod: String,
        node: String,
        extch: String,
    },
    MoSysExtChSpbp {
        extch: String,
    },
    MoSpbp {
        pod: String,
        node: String,
    },
    MoSysSpbp,
    MoExtChSppsu {
        pod: String,
        node: String,
        extch: String,
        psuslot: String,
    },
    MoSysExtChSppsu {
        extch: String,
        psuslot: String,
    },
    MoSppsu {
        pod: String,
        node: String,
        psuslot: String,
    },
    MoSysSppsu {
        psuslot: String,
    },
    MoExtChSpfan {
        pod: String,
        node: String,
        extch: String,
        ftslot: String,
    },
    MoSysExtChSpfan {
        extch: String,
        ftslot: String,
    },
    MoSpfan {
        pod: String,
        node: String,
        ftslot: String,
    },
    MoSysSpfan {
        ftslot: String,
    },
}

impl EndpointScheme for EqptSpCmnBlkEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptSpCmnBlk.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoNodeScSplc {
                pod,
                node,
                scslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/scslot-{scslot}/sc/splc/spcmn.json")),
            Self::MoSysScSplc {
                scslot,
            } => Cow::Owned(format!("mo/sys/ch/scslot-{scslot}/sc/splc/spcmn.json")),
            Self::MoNodeFcSplc {
                pod,
                node,
                fcslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/fcslot-{fcslot}/fc/splc/spcmn.json")),
            Self::MoSysFcSplc {
                fcslot,
            } => Cow::Owned(format!("mo/sys/ch/fcslot-{fcslot}/fc/splc/spcmn.json")),
            Self::MoNodeLcSplc {
                pod,
                node,
                lcslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/lcslot-{lcslot}/lc/splc/spcmn.json")),
            Self::MoSysLcSplc {
                lcslot,
            } => Cow::Owned(format!("mo/sys/ch/lcslot-{lcslot}/lc/splc/spcmn.json")),
            Self::MoExtChSpsup {
                pod,
                node,
                extch,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/spsup/spcmn.json")),
            Self::MoSysExtChSpsup {
                extch,
            } => Cow::Owned(format!("mo/sys/extch-{extch}/spsup/spcmn.json")),
            Self::MoSpsup {
                pod,
                node,
                supslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/supslot-{supslot}/sup/spsup/spcmn.json")),
            Self::MoSysSpsup {
                supslot,
            } => Cow::Owned(format!("mo/sys/ch/supslot-{supslot}/sup/spsup/spcmn.json")),
            Self::MoExtChSpbp {
                pod,
                node,
                extch,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/spbp/spcmn.json")),
            Self::MoSysExtChSpbp {
                extch,
            } => Cow::Owned(format!("mo/sys/extch-{extch}/spbp/spcmn.json")),
            Self::MoSpbp {
                pod,
                node,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/spbp/spcmn.json")),
            Self::MoSysSpbp => Cow::Owned(format!("mo/sys/ch/spbp/spcmn.json")),
            Self::MoExtChSppsu {
                pod,
                node,
                extch,
                psuslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/psuslot-{psuslot}/psu/sppsu/spcmn.json")),
            Self::MoSysExtChSppsu {
                extch,
                psuslot,
            } => Cow::Owned(format!("mo/sys/extch-{extch}/psuslot-{psuslot}/psu/sppsu/spcmn.json")),
            Self::MoSppsu {
                pod,
                node,
                psuslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/psuslot-{psuslot}/psu/sppsu/spcmn.json")),
            Self::MoSysSppsu {
                psuslot,
            } => Cow::Owned(format!("mo/sys/ch/psuslot-{psuslot}/psu/sppsu/spcmn.json")),
            Self::MoExtChSpfan {
                pod,
                node,
                extch,
                ftslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/ftslot-{ftslot}/ft/spfan/spcmn.json")),
            Self::MoSysExtChSpfan {
                extch,
                ftslot,
            } => Cow::Owned(format!("mo/sys/extch-{extch}/ftslot-{ftslot}/ft/spfan/spcmn.json")),
            Self::MoSpfan {
                pod,
                node,
                ftslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/ftslot-{ftslot}/ft/spfan/spcmn.json")),
            Self::MoSysSpfan {
                ftslot,
            } => Cow::Owned(format!("mo/sys/ch/ftslot-{ftslot}/ft/spfan/spcmn.json")),
        }
    }
}

pub type EqptSpCmnBlk = AciObject<__internal::EqptSpCmnBlk>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptSpCmnBlk;
    impl AciObjectScheme for EqptSpCmnBlk {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptSpCmnBlkEndpoint;
        const CLASS_NAME: &'static str = "eqptSpCmnBlk";
    }
}
