use serde::{
    Deserialize,
    Serialize,
};

use crate::types::games::GameShortCode;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum EloRange {
    Greater2k,
    Greater1k4,
    Greater800,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Elo(u64);

type HighestElo = Elo;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlatformElo((GameShortCode, HighestElo));
