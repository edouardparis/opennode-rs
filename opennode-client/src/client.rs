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
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::error::Error;

pub struct Client {
    client: reqwest::Client,
    host: String,
    api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct Data<T> {
    pub data: T,
}

impl Client {
    /// Creates a new client pointed to `https://api.opennode.co`
    pub fn new(apikey: impl Into<String>) -> Client {
        Client::from_url("https://api.opennode.co", apikey)
    }

    /// Creates a new client posted to a custom host.
    pub fn from_url(host: impl Into<String>, apikey: impl Into<String>) -> Client {
        Client {
            client: reqwest::Client::new(),
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

    pub async fn get<S, T>(&self, path: &str, params: Option<S>) -> Result<T, Error>
    where
        S: Into<String>,
        T: DeserializeOwned,
    {
        let p = params.map_or("".to_string(), |par| (par.into()));
        let url = self.host.to_owned() + path + &p;

        let res = self.client.get(&url)
            .header("Authorization", self.api_key.clone())
            .send()
            .await
            .map_err(|e| Error::Http(e))?;

        if res.status().is_success() {
            let d: Data<T> = res.json().await.map_err(|e| Error::Http(e))?;
            return Ok(d.data);
        }

        let e: opennode::error::RequestError = res.json().await.map_err(|e|{Error::Http(e)})?;
        Err(Error::Opennode(e))
    }


    pub async fn post<P, T>(&self, path: &str, payload: Option<P>) -> Result<T, Error>
    where
        P: Serialize,
        T: DeserializeOwned,
    {

        let mut body: Vec<u8> = Vec::new();
        let mut content_type = "".to_string();
        if let Some(p) = payload {
            body = serde_json::to_vec(&p).unwrap();
            content_type = "application/json".to_string();
        }

        let res = self.client.post(path)
            .header("Content-Type", content_type)
            .header("Authorization", self.api_key.clone())
            .body(body)
            .send()
            .await
            .map_err(|e| Error::Http(e))?;

        if res.status().is_success() {
            let d: Data<T> = res.json().await.map_err(|e| Error::Http(e))?;
            return Ok(d.data);
        }

        let e: opennode::error::RequestError = res.json().await.map_err(|e|{Error::Http(e)})?;
        Err(Error::Opennode(e))
    }
}
