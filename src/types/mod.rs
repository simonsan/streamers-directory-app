use std::collections::HashMap;

use serde::{
    Deserialize,
    Serialize,
};

use crate::types::{
    categories::ContentCategory,
    content_creator::ContentCreatorInfo,
    stream_platforms::ContentPlatformInfo,
};

pub mod categories;
pub mod content_creator;
pub mod elo;
pub mod game_platforms;
pub mod games;
pub mod info_platforms;
pub mod languages;
pub mod stream_platforms;

type ContentUrl = String;
type ImagePath = String;
pub type GameId = String;

#[derive(typed_builder::TypedBuilder, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Api {
    content_platform_infos: Vec<ContentPlatformInfo>,
    content_categories: Vec<ContentCategory>,
    content_creators: Vec<ContentCreatorInfo>,
}

/// Api error info for Unprocessable Entity error
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}
