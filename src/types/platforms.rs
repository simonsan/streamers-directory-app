use serde::{
    Deserialize,
    Serialize,
};

use crate::types::ImagePath;

type PlatformBaseUrl = String;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ContentPlatform {
    Twitch(ContentPlatformInfo),
    Youtube(ContentPlatformInfo),
    FacebookGaming(ContentPlatformInfo),
    Douyu(ContentPlatformInfo),
    Discord(ContentPlatformInfo),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContentPlatformInfo {
    pub logo: ImagePath,
    pub base_url: PlatformBaseUrl,
}
