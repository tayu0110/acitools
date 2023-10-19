use serde::{Deserialize, Serialize};

use crate::{AciObject, EndpointScheme};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    child_action: String,
    lc_own: String,
    mod_ts: String,
    rn: String,
    status: String,
    t_cl: String,
    t_dn: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {}

#[derive(Debug, Clone, Copy)]
pub enum Endpoint {}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> std::borrow::Cow<'_, str> {
        unimplemented!()
    }
}

pub type L3extRtBDToOut = AciObject<__internal::L3extRtBDToOut>;

mod __internal {
    use super::*;
    use crate::AciObjectScheme;

    #[derive(Debug, Clone, Copy)]
    pub struct L3extRtBDToOut;

    impl AciObjectScheme for L3extRtBDToOut {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "l3extRtBDToOut";
    }
}
