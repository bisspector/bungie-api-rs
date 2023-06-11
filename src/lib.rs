pub mod destiny2;

use core::fmt;
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::ReqwestError(err)
    }
}

pub struct BungieClient {
    api_key: String,
}

impl BungieClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    async fn send_request<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let url = format!("https://www.bungie.net/Platform/{}", path);

        let client = reqwest::Client::builder().build()?;

        let res = client
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .send()
            .await?;

        Ok(res.json::<T>().await?)
    }

    pub async fn send_request_string(&self, path: &str) -> Result<String, Error> {
        let url = format!("https://www.bungie.net/Platform/{}", path);

        let client = reqwest::Client::builder().build()?;

        let res = client
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .send()
            .await?;

        Ok(res.text().await?)
    }
}
