// src/protocol/mod.rs

use crate::domain::{Command, Response};
use std::fmt::Debug;

/// A codec between raw text frames and your shared `Command`/`Response`.
pub trait Protocol {
    /// Wire-format or parse error
    type Error: Debug;

    /// Decode a raw line into a domain `Command`
    fn decode(&self, line: &str) -> Result<Command, Self::Error>;

    /// Encode a domain `Response` into a raw line
    fn encode(&self, resp: &Response) -> String;
}

pub mod json;
