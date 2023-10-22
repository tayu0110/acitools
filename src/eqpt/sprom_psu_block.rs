use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    c_110_v: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    c_220_v: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    cksum: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    fbits: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    len: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ps_c_md_1: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ps_c_md_2: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ps_c_md_3: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ps_c_md_4: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ps_c_mx_md_1: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ps_c_mx_md_2: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ps_c_mx_md_3: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ps_c_mx_md_4: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    sig: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    sm_oid: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    ver: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {}

#[derive(Debug, Clone)]
pub enum EqptSpromPsuBlkEndpoint {
    ClassAll,
    MoUni,
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
}

impl EndpointScheme for EqptSpromPsuBlkEndpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/eqptSpromPsuBlk.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoExtChSppsu {
                pod,
                node,
                extch,
                psuslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/extch-{extch}/psuslot-{psuslot}/psu/sppsu/sppsublk.json")),
            Self::MoSysExtChSppsu {
                extch,
                psuslot,
            } => Cow::Owned(format!("mo/sys/extch-{extch}/psuslot-{psuslot}/psu/sppsu/sppsublk.json")),
            Self::MoSppsu {
                pod,
                node,
                psuslot,
            } => Cow::Owned(format!("mo/topology/pod-{pod}/node-{node}/sys/ch/psuslot-{psuslot}/psu/sppsu/sppsublk.json")),
            Self::MoSysSppsu {
                psuslot,
            } => Cow::Owned(format!("mo/sys/ch/psuslot-{psuslot}/psu/sppsu/sppsublk.json")),
        }
    }
}

pub type EqptSpromPsuBlk = AciObject<__internal::EqptSpromPsuBlk>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct EqptSpromPsuBlk;
    impl AciObjectScheme for EqptSpromPsuBlk {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = EqptSpromPsuBlkEndpoint;
        const CLASS_NAME: &'static str = "eqptSpromPsuBlk";
    }
}
