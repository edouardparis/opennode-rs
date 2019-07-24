use futures::future::Future;
use crate::error::Error;
use serde::Serialize;
use serde::de::DeserializeOwned;

use awc;
use serde;
use serde_json;
use serde::{Deserialize};

use actix_http::body::Body;

pub struct Client {
    client: awc::Client,
    host: String,
    version: String,
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
            version: "v1".to_string(),
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

    pub fn get<T, S, U>(&self, path: S, params: Option<U>) -> impl Future<Item=T, Error=Error>
        where T: DeserializeOwned, S: Into<String>, U: Into<String> {
            let p = params.map_or("".to_string(), |par|(par.into()));
            let req = self.client
                .get(self.host.to_owned()+"/"+&self.version+&path.into()+&p)
                .header("Authorization", self.api_key.clone());

            self.send(req, Body::Empty)
        }

    pub fn post<T, S, P>(&self, path: S, payload: Option<P>) -> impl Future<Item=T, Error=Error>
        where T: DeserializeOwned, S: Into<String>, P: Serialize {
            let req = self.client
                .post(self.host.to_owned()+"/"+&self.version+&path.into())
                .content_type("application/json")
                .header("Authorization", self.api_key.clone());

            match payload {
                None=> self.send(req, Body::Empty),
                Some(p) => {
                    let body = serde_json::to_vec(&p).unwrap();
                    self.send(req, Body::Bytes(body.into()))
                }
            }
        }

    fn send<T>(&self, req: awc::ClientRequest, body: Body) -> impl Future<Item=T, Error=Error>
        where T: serde::de::DeserializeOwned {
            req.send_body(body).map_err(|e| {Error::Http(e)})
                .and_then(|mut resp| {
                    resp.body().map(move |body_out| {
                        (resp, body_out)
                    }).map_err(|e| {Error::Payload(awc::error::JsonPayloadError::Payload(e))})
                }).and_then(|(_, body)| {
                    serde_json::from_slice(&body)
                        .map_err(|e| {Error::Payload(awc::error::JsonPayloadError::Deserialize(e))})
                        .map(|res: Response<T>| res.data)
                })
        }
}
