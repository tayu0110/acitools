use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{AciObject, EndpointScheme};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    child_action: String,
    descr: String,
    domain_id: String,
    lc_own: String,
    mod_ts: String,
    name: String,
    name_alias: String,
    rn: String,
    status: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    BgpDomainIdCons {},
}

#[derive(Debug, Clone, Copy)]
pub enum Endpoint {}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> Cow<'_, str> {
        unimplemented!()
    }
}

pub type BgpDomainIdAllocator = AciObject<__internal::BgpDomainIdAllocator>;

mod __internal {
    use crate::AciObjectScheme;

    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub struct BgpDomainIdAllocator;

    impl AciObjectScheme for BgpDomainIdAllocator {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "bgpDomainIdAllocator";
    }
}
