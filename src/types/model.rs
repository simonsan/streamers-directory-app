// use chrono::prelude::*;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
enum EloRange {
    Greater2k,
    Greater1k4,
    Greater800,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
enum Category {
    CastingRanked(EloRange),
    CastingTournaments,
    OrganizingTournaments,
    CommunityGames,
    PovContent(EloRange),
    LearningResources,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
enum LanguageShortCode {
    En,
    De,
    Fr,
    Esp,
    It,
    Por,
    Rus,
    Other(String),
}
#[derive(Serialize, Deserialize, Clone, Debug)]
enum GameShortCode {
    Aoe1,
    Aoe2,
    Aoe3,
    Aoe4,
    Aom,
    AoeO,
    Other(String),
}

type StreamerUrl = String;
#[derive(Serialize, Deserialize, Clone, Debug)]
enum Platform {
    Twitch(StreamerUrl),
    Youtube(StreamerUrl),
    Facebook(StreamerUrl),
    Douyu(StreamerUrl),
    Discord(StreamerUrl),
}
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Elo(u64);

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ContentCreatorInfo {
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
