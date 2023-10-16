#![recursion_limit = "256"]

mod aaa;
mod bgp;
mod client;
mod eqpt;
mod error;
mod ethpm;
mod fabric;
mod fault;
mod fv;
mod l1;
mod l3ext;
mod lldp;
mod ospf;
mod rtctrl;
mod top;

pub use aaa::login::AaaLogin;
pub use aaa::role::PrivType;
pub use aaa::user::*;
pub use client::*;
pub use error::*;
pub use fault::FaultInst;
pub use fv::ap::{FvAp, QoSClass};
pub use fv::bd::{FvBD, L2UnknownUnicast, L3UnknownMulticast, MultiDestinationFlooding};
pub use fv::ctx::FvCtx;
pub use fv::subnet::FvSubnet;
pub use fv::tenant::*;
pub use l3ext::out::L3extOut;
pub use top::system::TopSystem;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Response {
    #[serde(rename = "totalCount")]
    total_count: String,
    imdata: Vec<serde_json::Value>,
}

impl Response {
    pub(crate) async fn extract(mut self) -> Result<Vec<serde_json::Value>, Error> {
        if self.imdata.len() != 1 {
            return Ok(self.imdata);
        }

        if let Some(mut error) = self.imdata[0].as_object_mut().unwrap().remove("error") {
            let attributes = error.as_object_mut().unwrap().remove("attributes").unwrap();
            return Err(serde_json::from_value::<Error>(attributes).unwrap());
        }

        Ok(self.imdata)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_format_test() {
        let ser = serde_json::to_value(Response {
            total_count: "1".to_owned(),
            imdata: vec![],
        });

        assert!(ser.is_ok());

        let ser = ser.unwrap().to_string();
        assert_eq!(ser, "{\"imdata\":[],\"totalCount\":\"1\"}")
    }
}
