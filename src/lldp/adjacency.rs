use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    capability: String,
    chassis_id_t: String,
    chassis_id_v: String,
    child_action: String,
    en_cap: String,
    id: String,
    mgmt_id: String,
    mgmt_ip: String,
    mgmt_port_mac: String,
    mod_ts: String,
    mon_pol_dn: String,
    name: String,
    port_desc: String,
    port_id_t: String,
    port_id_v: String,
    port_vlan: String,
    rn: String,
    st_qual: String,
    status: String,
    sys_desc: String,
    sys_name: String,
    ttl: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildItem {
    LldptlvComplex {},
    LldptlvIp {},
    LldptlvText {},
    LldptlvUByte {},
    LldptlvUInt16 {},
    LldptlvUInt32 {},
}
