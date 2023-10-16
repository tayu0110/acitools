use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub enum AaaLogin {
    AaaLogin { attributes: Attributes },
}

impl AaaLogin {
    fn attributes(&self) -> &Attributes {
        let AaaLogin::AaaLogin { attributes } = self;
        attributes
    }

    pub fn timeout(&self) -> u32 {
        self.attributes().refresh_timeout_seconds.parse().unwrap()
    }

    pub fn token(&self) -> String {
        self.attributes().token.clone()
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Attributes {
    refresh_timeout_seconds: String,
    token: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn login_deserialize_test() {
        let login = json!({
            "aaaLogin": {
                "attributes": {
                    "token": "abcdefg",
                    "refreshTimeoutSeconds": "30"
                }
            }
        })
        .to_string();

        let de = serde_json::from_str::<AaaLogin>(&login);
        if let Err(e) = de.as_ref() {
            eprintln!("{}", e);
        }
        assert!(de.is_ok());
        let de = de.unwrap();
        eprintln!("{:?}", de);
    }
}
