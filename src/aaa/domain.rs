use crate::aaa::role::{AaaUserRole, PrivType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AaaUserDomain {
    aaa_user_domain: AaaUserDomainInner,
}

impl AaaUserDomain {
    pub fn new(domain: &str) -> Self {
        Self {
            aaa_user_domain: AaaUserDomainInner {
                attributes: AttributeAaaUserDomain::new(domain),
                children: vec![],
            },
        }
    }

    pub fn append(&mut self, role: &str, priv_type: PrivType) {
        self.aaa_user_domain
            .children
            .push(AaaUserRole::new(role, priv_type));
    }

    pub fn domain(&self) -> String {
        self.aaa_user_domain.attributes.name.clone()
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct AaaUserDomainInner {
    attributes: AttributeAaaUserDomain,
    children: Vec<AaaUserRole>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
struct AttributeAaaUserDomain {
    name: String,
}

impl AttributeAaaUserDomain {
    fn new(domain: &str) -> Self {
        Self {
            name: domain.to_string(),
        }
    }
}
