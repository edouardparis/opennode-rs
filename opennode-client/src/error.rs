use opennode::error::RequestError;

#[derive(Debug)]
pub enum Error {
    /// An error reported by Opennode in the response body.
    Opennode(RequestError),
    /// A networking error communicating with the Opennode server.
    Http(reqwest::Error),
}
