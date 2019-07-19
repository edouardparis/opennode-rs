use futures::future::Future;
use crate::error::Error;

use awc;
use serde;
use serde_json;

use actix_http::body::Body;

pub struct Client {
    client: awc::Client,
    host: String,
    version: String,
    api_key: String,
}

impl Client {
    pub fn new(apikey: impl Into<String>) -> Client {
        Client{
            client: awc::Client::default(),
            host: "https://api.opennode.co".to_string(),
            version: "v1".to_string(),
            api_key: apikey.into(),
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
        where T: serde::de::DeserializeOwned, S: Into<String>, U: Into<String> {
            let p = params.map_or("".to_string(), |par|(par.into()));
            let req = self.client
                .get(self.host.to_owned()+&self.version+&path.into()+&p)
                .header("Authorization", self.api_key.clone());

            self.send(req, Body::Empty)
        }

    fn send<T>(&self, req: awc::ClientRequest, body: Body) -> impl Future<Item=T, Error=Error>
        where T: serde::de::DeserializeOwned {
            req.send_body(body).map_err(|e| {Error::Http(e)})
                .and_then(|mut resp| {
                    resp.body().map(move |body_out| {
                        (resp, body_out)
                    }).map_err(|e| {Error::Payload(awc::error::JsonPayloadError::Payload(e))})
                }).and_then(|(_, body)| {
                    serde_json::from_slice(&body).map_err(|e| {Error::Payload(awc::error::JsonPayloadError::Deserialize(e))})
                })
        }
}
