use crate::{AciObject, ConfigStatus, EndpointScheme};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "String::is_empty")]
    child_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    dn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    lc_own: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mod_ts: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    rn: String,
    status: ConfigStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    t_cl: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    t_dn: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {}

#[derive(Debug, Clone)]
pub enum Endpoint {
    ClassAll,
    MoUni,
    MoCtx {
        tn: String,
        ctx: String,
        rtctx: String,
    },
}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        match self {
            Self::ClassAll => Cow::Borrowed("node/class/fvRtCtx.json"),
            Self::MoUni => Cow::Borrowed("mo/uni.json"),
            Self::MoCtx { tn, ctx, rtctx } => {
                Cow::Owned(format!("mo/uni/tn-{tn}/ctx-{ctx}/rtctx-[{rtctx}].json"))
            }
        }
    }
}

pub type FvRtCtx = AciObject<__internal::FvRtCtx>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;
    #[derive(Debug, Clone, Copy)]
    pub struct FvRtCtx;
    impl AciObjectScheme for FvRtCtx {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "fvRtCtx";
    }
}
