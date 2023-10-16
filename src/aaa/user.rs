use crate::aaa::domain::AaaUserDomain;
use crate::aaa::role::PrivType;
use crate::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AaaUser {
    aaa_user: AaaUserInner,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct AaaUserInner {
    attributes: AttributeAaaUser,
    children: Vec<AaaUserDomain>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct AttributeAaaUser {
    account_status: String,
    clear_pwd_history: String,
    expiration: String,
    expires: String,
    name: String,
    pwd_life_time: String,
    pwd_update_required: String,
    pwd: String,
}

impl AaaUser {
    pub fn new(name: impl Into<String>, pwd: impl Into<String>) -> Self {
        Self {
            aaa_user: AaaUserInner {
                attributes: AttributeAaaUser {
                    account_status: "active".to_string(),
                    clear_pwd_history: "no".to_string(),
                    expiration: "never".to_string(),
                    expires: "no".to_string(),
                    name: name.into(),
                    pwd_life_time: "no-password-expire".to_string(),
                    pwd_update_required: "no".to_string(),
                    pwd: pwd.into(),
                },
                children: vec![],
            },
        }
    }

    pub fn no_expiration(mut self) -> Self {
        self.aaa_user.attributes.expiration = "never".to_string();
        self.aaa_user.attributes.expires = "no".to_string();
        self
    }

    pub fn no_pwd_expiration(mut self) -> Self {
        self.aaa_user.attributes.pwd_life_time = "no-password-expire".to_string();
        self.aaa_user.attributes.pwd_update_required = "no".to_string();
        self
    }

    pub fn add_role(mut self, domain: &str, role: &str, priv_type: PrivType) -> Self {
        if let Some(pos) = self
            .aaa_user
            .children
            .iter()
            .position(|d| &d.domain() == domain)
        {
            self.aaa_user.children[pos].append(role, priv_type);
        } else {
            let mut domain = AaaUserDomain::new(domain);
            domain.append(role, priv_type);
            self.aaa_user.children.push(domain);
        }
        self
    }

    pub async fn create(
        &self,
        client: &Client,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let json = serde_json::to_value(self)?;
        eprintln!("json: {:#?}", json);
        client.post("mo/uni/userext.json", &json).await
    }
}
