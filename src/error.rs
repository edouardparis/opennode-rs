use awc;

#[derive(Debug)]
pub enum Error {
    // A networking error communicating with the Opennode server.
    Http(awc::error::SendRequestError),
    // A set of errors that can occur during parsing payloads.
    Payload(awc::error::JsonPayloadError),
    // An error reading the response body.
    Io(std::io::Error),
}
