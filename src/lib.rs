#![recursion_limit = "256"]

mod aaa;
mod client;
mod error;
mod fault;
mod fv;

pub use aaa::login::*;
pub use aaa::role::PrivType;
pub use aaa::user::*;
pub use client::*;
pub use error::*;
pub use fault::*;
pub use fv::ap::*;
pub use fv::bd::*;
pub use fv::ctx::*;
pub use fv::subnet::*;
pub use fv::tenant::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub(crate) enum ResponseFormat<T: Clone> {
    #[serde(rename_all = "camelCase")]
    Response { total_count: String, imdata: Vec<T> },
}

impl<T: Clone> ResponseFormat<T> {
    pub fn extract(self) -> Vec<T> {
        match self {
            ResponseFormat::Response {
                imdata,
                total_count: _,
            } => imdata,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_format_test() {
        let ser = serde_json::to_value(ResponseFormat::<()>::Response {
            total_count: "1".to_string(),
            imdata: vec![],
        });

        assert!(ser.is_ok());

        let ser = ser.unwrap().to_string();
        assert_eq!(ser, "{\"imdata\":[],\"totalCount\":\"1\"}")
    }
}
