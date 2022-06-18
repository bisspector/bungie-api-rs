use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    #[serde(rename = "Response")]
    pub response: T,
    #[serde(rename = "ErrorCode")]
    pub error_code: i32,
    #[serde(rename = "ThrottleSeconds")]
    pub throttle_seconds: i32,
    #[serde(rename = "ErrorStatus")]
    pub error_status: String,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "MessageData")]
    pub message_data: HashMap<String, String>,
    #[serde(rename = "DetailedErrorTrace")]
    pub detailed_error_trace: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DestinyManifest {
    pub version: String,
    #[serde(rename = "mobileAssetContentPath")]
    pub mobile_asset_content_path: String,
    #[serde(rename = "mobileGearAssetDataBases")]
    pub mobile_gear_asset_data_bases: Vec<GearAssetDataBaseDefinition>,
    #[serde(rename = "mobileWorldContentPaths")]
    pub mobile_world_content_paths: HashMap<String, String>,
    #[serde(rename = "jsonWorldContentPaths")]
    pub json_world_content_paths: HashMap<String, String>,
    #[serde(rename = "jsonWorldComponentContentPaths")]
    pub json_world_component_content_paths: HashMap<String, HashMap<String, String>>,
    #[serde(rename = "mobileClanBannerDatabasePath")]
    pub mobile_clan_banner_database_path: String,
    #[serde(rename = "mobileGearCDN")]
    pub mobile_gear_cdn: HashMap<String, String>,
    #[serde(rename = "iconImagePyramidInfo")]
    pub icon_image_pyramid_info: Vec<ImagePyramidEntry>,
}

#[derive(Debug, Deserialize)]
pub struct GearAssetDataBaseDefinition {
    pub version: i32,
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct ImagePyramidEntry {
    pub name: String,
    pub factor: f64,
}
