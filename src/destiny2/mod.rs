mod models;
use models::*;

use async_trait::async_trait;

use crate::{BungieClient, Error};

#[async_trait]
pub trait Destiny2 {
    async fn get_destiny_manifest(&self) -> Result<Response<DestinyManifest>, Error>;
}

#[async_trait]
impl Destiny2 for BungieClient {
    async fn get_destiny_manifest(&self) -> Result<Response<DestinyManifest>, Error> {
        self.send_request("Destiny2/Manifest/").await
    }
}
