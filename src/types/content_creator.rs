use serde::{
    Deserialize,
    Serialize,
};

use super::{
    category::Category,
    elo::Elo,
    game::GameShortCode,
    language::LanguageShortCode,
    platform::Platform,
};

type PreviewImagePath = String;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContentCreatorInfo {
    name: String,
    image: PreviewImagePath,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct ContentCreatorInfoOld {
    id: usize,
    name: String,
    platforms: Vec<(Platform,
                    Vec<Category>,
                    Vec<LanguageShortCode>,
                    Vec<GameShortCode>)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elo: Option<Elo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aoc_ref_name: Option<String>,
}
