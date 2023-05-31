use crate::client::Client;
use serde::Deserialize;
use serde_json::{from_value, json};
use std::ops::Index;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct FaultInst {
    total_count: String,
    imdata: Vec<Imdata>,
}

impl FaultInst {
    pub async fn get(client: &mut Client) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(from_value::<FaultInst>(
            client.get("class/faultInst.json")?.send().await?,
        )?)
    }

    fn total_count(&self) -> usize {
        self.total_count.parse().unwrap()
    }

    pub fn len(&self) -> usize {
        let total_count = self.total_count();
        assert_eq!(total_count, self.imdata.len());
        self.imdata.len()
    }
}

impl Index<usize> for FaultInst {
    type Output = Fault;
    fn index(&self, index: usize) -> &Self::Output {
        &self.imdata[index].fault_inst
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
struct Imdata {
    fault_inst: Fault,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Fault {
    attributes: Attributes,
}

impl Fault {
    pub fn is_acked(&self) -> bool {
        &self.attributes.ack != "no"
    }

    pub async fn ack(&self, client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_acked() {
            return Ok(());
        }

        let req = json!({
            "totalCount": "1",
            "imdata": [{
                "faultInst": {
                    "attributes": {
                        "ack": "yes",
                        "alert": self.attributes.alert,
                        "code": self.attributes.code,
                        "delegated": self.attributes.delegated,
                        "dn": self.attributes.dn,
                        "status": "modified",
                        "title": self.attributes.title,
                        "type": self.attributes.fault_type
                    }
                }
            }]
        });
        client
            .post(&format!("mo/{}.json", self.attributes.dn), &req)
            .await?;
        Ok(())
    }

    pub async fn noack(&self, client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
        if !self.is_acked() {
            return Ok(());
        }

        let req = json!({
            "totalCount": "1",
            "imdata": [{
                "faultInst": {
                    "attributes": {
                        "ack": "no",
                        "alert": self.attributes.alert,
                        "code": self.attributes.code,
                        "delegated": self.attributes.delegated,
                        "dn": self.attributes.dn,
                        "status": "modified",
                        "title": self.attributes.title,
                        "type": self.attributes.fault_type
                    }
                }
            }]
        });
        client
            .post(&format!("mo/{}.json", self.attributes.dn), &req)
            .await?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
struct Attributes {
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
            "totalCount": "1",
            "imdata": [{
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
            }]
        });

        let res = from_value::<FaultInst>(fault);
        if let Err(e) = res.as_ref() {
            eprintln!("{}", e);
        }
        assert!(res.is_ok());
    }
}
