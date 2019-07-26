use awc::error::{JsonPayloadError, SendRequestError};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Error {
    /// An error reported by Opennode in the response body.
    Opennode(RequestError),
    /// A networking error communicating with the Opennode server.
    Http(SendRequestError),
    /// A set of errors that can occur during parsing payloads.
    Payload(JsonPayloadError),
    /// Indicates an operation not supported (yet?) by this library.
    Unsupported(&'static str),
}

/// RequestError is the error format returned by Opennode API.
/// Example: `{\"success\":false,\"message\":\"Failed to authenticate token\"}"`
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestError {
    pub success: bool,
    pub message: String,
}
