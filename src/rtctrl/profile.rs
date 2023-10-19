use serde::{Deserialize, Serialize};

use crate::{AciObject, EndpointScheme};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    annotation: String,
    auto_continue: String,
    child_action: String,
    descr: String,
    direction: String,
    ext_mngd_by: String,
    lc_own: String,
    level: String,
    mod_ts: String,
    mon_pol_dn: String,
    name: String,
    name_alias: String,
    owner_key: String,
    owner_tag: String,
    rn: String,
    status: String,
    #[serde(rename = "type")]
    r#type: String,
    uid: String,
    userdom: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    RtctrlCtxP {},
    RtctrlRtBDSubnetToProfile {},
    RtctrlRtBDToProfile {},
    RtctrlRtInstPToProfile {},
    RtctrlRtSubnetToProfile {},
}

#[derive(Debug, Clone, Copy)]
pub enum Endpoint {}

impl EndpointScheme for Endpoint {
    fn endpoint(&self) -> std::borrow::Cow<'_, str> {
        unimplemented!()
    }
}

pub type RtctrlProfile = AciObject<__internal::RtctrlProfile>;

mod __internal {
    use crate::AciObjectScheme;

    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub struct RtctrlProfile;

    impl AciObjectScheme for RtctrlProfile {
        type Attributes = Attributes;
        type ChildItem = ChildItem;
        type Endpoint = Endpoint;
        const CLASS_NAME: &'static str = "rtctrlProfile";
    }
}
