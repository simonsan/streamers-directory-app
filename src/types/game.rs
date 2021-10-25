use serde::{
    Deserialize,
    Serialize,
};


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum GameShortCode {
    Aoe1,
    Aoe2,
    Aoe3,
    Aoe4,
    Aom,
    AoeO,
    Other(String),
}
