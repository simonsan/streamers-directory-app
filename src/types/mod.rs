use std::collections::HashMap;

use serde::{
    Deserialize,
    Serialize,
};

pub mod category;
pub mod content_creator;
pub mod des_streamers;
pub mod elo;
pub mod game;
pub mod language;
pub mod platform;


/// Api error info for Unprocessable Entity error
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}
