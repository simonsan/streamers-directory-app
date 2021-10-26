use std::collections::HashMap;

use serde::{
    Deserialize,
    Serialize,
};

pub mod categories;
pub mod content_creator;
pub mod des_streamers;
pub mod elo;
pub mod games;
pub mod languages;
pub mod platforms;

type ContentUrl = String;
type ImagePath = String;

/// Api error info for Unprocessable Entity error
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}
