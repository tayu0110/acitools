use crate::client::Client;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub enum FaultInst {
    FaultInst { attributes: Attributes },
}

impl FaultInst {
    pub async fn get(client: &Client) -> Result<Box<[Self]>, Box<dyn std::error::Error>> {
        let res = client.get("class/faultInst.json")?.send().await?;
        Ok(res
            .into_iter()
            .map(|res| serde_json::from_value(res))
            .collect::<Result<Vec<FaultInst>, serde_json::Error>>()?
            .into_boxed_slice())
    }

    fn attributes(&self) -> &Attributes {
        let FaultInst::FaultInst { attributes } = self;
        attributes
    }

    pub fn is_acked(&self) -> bool {
        &self.attributes().ack != "no"
    }

    pub async fn ack(&self, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_acked() {
            return Ok(());
        }

        let req = json!({
            "totalCount": "1",
            "imdata": [{
                "faultInst": {
                    "attributes": {
                        "ack": "yes",
                        "alert": self.attributes().alert,
                        "code": self.attributes().code,
                        "delegated": self.attributes().delegated,
                        "dn": self.attributes().dn,
                        "status": "modified",
                        "title": self.attributes().title,
                        "type": self.attributes().fault_type
                    }
                }
            }]
        });
        client
            .post(&format!("mo/{}.json", self.attributes().dn), &req)
            .await?;
        Ok(())
    }

    pub async fn noack(&self, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
        if !self.is_acked() {
            return Ok(());
        }

        let req = json!({
            "totalCount": "1",
            "imdata": [{
                "faultInst": {
                    "attributes": {
                        "ack": "no",
                        "alert": self.attributes().alert,
                        "code": self.attributes().code,
                        "delegated": self.attributes().delegated,
                        "dn": self.attributes().dn,
                        "status": "modified",
                        "title": self.attributes().title,
                        "type": self.attributes().fault_type
                    }
                }
            }]
        });
        client
            .post(&format!("mo/{}.json", self.attributes().dn), &req)
            .await?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Attributes {
    ack: String,
    alert: String,
    code: String,
    delegated: String,
    dn: String,
    title: String,
    #[serde(rename(deserialize = "type"))]
    fault_type: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{from_value, json};

    #[test]
    fn fault_deserialize_test() {
        let fault = json!({
            "faultInst": {
                "attributes": {
                    "ack": "no",
                    "alert": "no",
                    "code": "F1238",
                    "delegated": "no",
                    "dn": "abcdef",
                    "title": "test fault",
                    "type": "type"
                }
            }
        });

        let res = from_value::<FaultInst>(fault);
        if let Err(e) = res.as_ref() {
            eprintln!("{}", e);
        }
        assert!(res.is_ok());
    }
}
