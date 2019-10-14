use awc::error::{JsonPayloadError, SendRequestError};

use opennode::error::RequestError;

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
