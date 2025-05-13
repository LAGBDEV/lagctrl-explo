use super::Protocol;
use crate::domain::{Command, Response};
use serde_json;

#[derive(Debug)]
pub struct JsonError(pub serde_json::Error);

impl std::fmt::Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for JsonError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

#[derive(Debug)]
pub struct JsonProtocol;

impl Protocol for JsonProtocol {
    type Error = JsonError;

    fn decode(&self, line: &str) -> Result<Command, Self::Error> {
        serde_json::from_str(line).map_err(JsonError)
    }

    fn encode(&self, resp: &Response) -> String {
        serde_json::to_string(resp).unwrap()
    }
}
