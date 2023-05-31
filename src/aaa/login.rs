use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct AaaLogin {
    total_count: String,
    imdata: Vec<ImdataElement>,
}

impl AaaLogin {
    fn total_count(&self) -> u32 {
        self.total_count.parse().unwrap()
    }
    pub fn timeout(&self) -> u32 {
        assert!(self.total_count() > 0);
        self.imdata[0]
            .aaa_login
            .attributes
            .refresh_timeout_seconds
            .parse()
            .unwrap()
    }

    pub fn token(&self) -> String {
        assert!(self.total_count() > 0);
        self.imdata[0].aaa_login.attributes.token.clone()
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
struct ImdataElement {
    aaa_login: ImdataInner,
}

#[derive(Debug, Deserialize, Clone)]
struct ImdataInner {
    attributes: Attributes,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
struct Attributes {
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
            "totalCount": "1",
            "imdata": [{
                "aaaLogin": {
                    "attributes": {
                        "token": "abcdefg",
                        "refreshTimeoutSeconds": "30"
                    }
                }
            }]
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
