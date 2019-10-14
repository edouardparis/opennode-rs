use serde::{Deserialize, Serialize};

/// RequestError is the error format returned by Opennode API.
/// Example: `{\"success\":false,\"message\":\"Failed to authenticate token\"}"`
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestError {
    pub success: bool,
    pub message: String,
}
