use serde::{
    Deserialize,
    Serialize,
};

use crate::types::{
    language::LanguageShortCode as Language,
    profiles::ProfileInfo,
    ImagePath,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StreamerCard {
    pub profile: ProfileInfo,
    pub media_content: MediaContent,
    pub streamer_language: Vec<Language>,
    pub content_language: Vec<Language>,
    pub viewer_numbers: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MediaContent {
    pub stream_preview: Option<ImagePath>,
    pub game_logo: Option<ImagePath>,
}
