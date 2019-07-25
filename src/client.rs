/// About Authentication:
///
/// The OpenNode API uses API Keys to authenticate requests.
/// You can view and manage your API Keys in the Integrations dashboard.
/// Development mode keys are generated on the Development environment,
/// while Production mode keys are generated on the Production environment.
///
/// Authentication to the API is performed with a key.
/// Provide your API key on on the Authorization header.
///
/// All API requests must be made over HTTPS.
/// Calls made over plain HTTP will fail.
/// API Requests without authentication will also fail.
///
/// | Permission                        | Invoice | Read | Withdrawals |
/// |-----------------------------------|---------|------|-------------|
/// | Create charge & fetch charge info |   ✅    |  ✅  |    ✅       |
/// | Fetch charges & withdrawals info  |   ❌    |  ✅  |    ✅       |
/// | Initiate withdrawals              |   ❌    |  ❌  |    ✅       |


use actix_http::body::Body;
use awc;
use futures::future::Future;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde;
use serde_json;

use crate::error;


pub struct Client {
    client: awc::Client,
    host: String,
    api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    pub data: T
}

impl Client {
    /// Creates a new client pointed to `https://api.opennode.co`
    pub fn new(apikey: impl Into<String>) -> Client {
        Client::from_url("https://api.opennode.co", apikey)
    }

    /// Creates a new client posted to a custom host.
    pub fn from_url(host: impl Into<String>, apikey: impl Into<String>) -> Client {
        Client {
            client: awc::Client::new(),
            api_key: apikey.into(),
            host: host.into(),
        }
    }

    pub fn with_api_key(self, apikey: impl Into<String>) -> Client {
        let mut clt = self;
        clt.api_key = apikey.into();
        clt
    }

    pub fn with_host(self, host: impl Into<String>) -> Client {
        let mut clt = self;
        clt.host = host.into();
        clt
    }

    pub fn get<T, S, U>(&self, path: S, params: Option<U>) -> impl Future<Item=T, Error=error::Error>
        where T: DeserializeOwned, S: Into<String>, U: Into<String> {
            let p = params.map_or("".to_string(), |par|(par.into()));
            let req = self.client
                .get(self.host.to_owned()+&path.into()+&p)
                .header("Authorization", self.api_key.clone());

            Client::send(req, Body::Empty)
        }

    pub fn post<T, S, P>(&self, path: S, payload: Option<P>) -> impl Future<Item=T, Error=error::Error>
        where T: DeserializeOwned, S: Into<String>, P: Serialize {
            let req = self.client
                .post(self.host.to_owned()+&path.into())
                .content_type("application/json")
                .header("Authorization", self.api_key.clone());

            match payload {
                None=> Client::send(req, Body::Empty),
                Some(p) => {
                    let body = serde_json::to_vec(&p).unwrap();
                    Client::send(req, Body::Bytes(body.into()))
                }
            }
        }


    fn send<T>(req: awc::ClientRequest, body: Body) -> impl Future<Item=T, Error=error::Error>
        where T: serde::de::DeserializeOwned {
            req.send_body(body).map_err(|e| {error::Error::Http(e)})
                .and_then(|mut resp| {
                    resp.body().map(move |body_out| {
                        (resp, body_out)
                    }).map_err(|e| {error::Error::Payload(awc::error::JsonPayloadError::Payload(e))})
                }).and_then(|(res, body)| {
                    if !res.status().is_success() {
                        let err: error::RequestError = serde_json::from_slice(&body)
                            .map_err(|e| {error::Error::Payload(awc::error::JsonPayloadError::Deserialize(e))})?;
                        return Err(error::Error::Opennode(err))
                    }
                    serde_json::from_slice(&body)
                        .map_err(|e| {error::Error::Payload(awc::error::JsonPayloadError::Deserialize(e))})
                        .map(|res: Response<T>| res.data)
                })
        }
}
