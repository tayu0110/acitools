use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, Clone, Deserialize)]
pub struct Error {
    code: String,
    text: String,
}

impl Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}, (code: {})", self.text, self.code)
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone, Copy)]
pub struct InvalidFormatError;

impl Display for InvalidFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: Invalid Error Format.")
    }
}

impl std::error::Error for InvalidFormatError {}

#[cfg(test)]
mod tests {}

#[derive(Debug, Clone)]
pub struct InvalidSubnetError(pub(crate) String);

impl Display for InvalidSubnetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: Invalid Subnet Error. (message: {})", self.0)
    }
}

impl std::error::Error for InvalidSubnetError {}
