use serde::{
    Deserialize,
    Serialize,
};

use crate::types::{
    categories::Category,
    des_streamers::{
        GamePlatform,
        InfoPlatform,
    },
    elo::PlatformElo,
    languages::LanguageShortCode,
    platforms::ContentPlatform,
    ContentUrl,
    ImagePath,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContentCreatorPlatformInfo {
    content_platform: ContentPlatform,
    creator_url: ContentUrl,
    content_languages: Vec<LanguageShortCode>,
    content_categories: Vec<Category>,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContentCreatorInfo {
    pub id: usize,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ImagePath>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub following: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_platforms: Option<Vec<ContentCreatorPlatformInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info_platforms: Option<Vec<InfoPlatform>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gaming_profiles: Option<Vec<GamePlatform>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_elos: Option<Vec<PlatformElo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aoc_ref_id: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContentCreatorInfoWrapper {
    pub profile: ContentCreatorInfo,
}
