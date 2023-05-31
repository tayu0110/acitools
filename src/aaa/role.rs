use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AaaUserRole {
    aaa_user_role: AaaUserRoleInner,
}

impl AaaUserRole {
    pub fn new(role: &str, priv_type: PrivType) -> Self {
        Self {
            aaa_user_role: AaaUserRoleInner {
                attributes: AttributeAaaUserRole::new(role, priv_type),
            },
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct AaaUserRoleInner {
    attributes: AttributeAaaUserRole,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttributeAaaUserRole {
    name: String,
    priv_type: String,
}

impl AttributeAaaUserRole {
    pub fn new(name: &str, priv_type: PrivType) -> Self {
        Self {
            name: name.to_string(),
            priv_type: priv_type.to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PrivType {
    ReadPriv,
    WritePriv,
}

impl ToString for PrivType {
    fn to_string(&self) -> String {
        match self {
            Self::ReadPriv => "readPriv".to_string(),
            _ => "writePriv".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ParsePrivTypeError;
impl std::fmt::Display for ParsePrivTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for ParsePrivTypeError {}

impl FromStr for PrivType {
    type Err = ParsePrivTypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "r" | "R" | "read" | "Read" | "readPriv" | "ReadPriv" => Ok(Self::ReadPriv),
            "w" | "W" | "write" | "Write" | "writePriv" | "WritePriv" => Ok(Self::WritePriv),
            _ => Err(ParsePrivTypeError),
        }
    }
}
