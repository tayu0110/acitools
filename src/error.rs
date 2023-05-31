use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Error {
    code: u32,
    text: String,
}

impl Error {
    pub fn is_error(json: &serde_json::Value) -> bool {
        let imdata = json
            .as_object()
            .and_then(|obj| obj.get("imdata"))
            .and_then(|imdata| imdata.as_array());
        if imdata.is_none() {
            return true;
        }
        let imdata = imdata.unwrap().get(0);
        if imdata.is_none() {
            return false;
        }
        imdata
            .unwrap()
            .as_object()
            .and_then(|obj| Some(obj.contains_key("error")))
            .unwrap_or(true)
    }

    pub fn try_new(json: &serde_json::Value) -> Result<Self, Box<dyn std::error::Error>> {
        if let Some(error) = json
            .as_object()
            .and_then(|obj| obj.get("imdata"))
            .and_then(|imdata| imdata.as_array())
            .and_then(|imdata| imdata.get(0))
            .and_then(|imdata| imdata.as_object())
            .and_then(|obj| obj.get("error"))
            .and_then(|err| err.as_object())
            .and_then(|err| err.get("attributes"))
        {
            Ok(Self {
                code: error["code"].as_str().unwrap().parse().unwrap(),
                text: error["text"].as_str().unwrap().to_string(),
            })
        } else {
            Err(Box::new(InvalidFromatError))
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}, code({})", self.text, self.code)
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone, Copy)]
pub struct InvalidFromatError;

impl Display for InvalidFromatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: Invalid Error Format.")
    }
}

impl std::error::Error for InvalidFromatError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_fmt_test() {
        let err = serde_json::json!({
            "imdata": [{
                "error": {
                    "attributes": {
                        "code": "102",
                        "text": "configured object ((Dn0)) not found Dn0=uni/tn-test-tenan, "
                    }
                }
            }]
        });

        assert!(Error::is_error(&err));

        let err = Error::try_new(&err);
        assert!(err.is_ok())
    }
}

#[derive(Debug, Clone)]
pub struct InvalidSubnetError(pub(crate) String);

impl Display for InvalidSubnetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: Invalid Subnet Error. (message: {})", self.0)
    }
}

impl std::error::Error for InvalidSubnetError {}
