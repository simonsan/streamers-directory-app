use serde::{
    Deserialize,
    Serialize,
};


type BaseUrl = String;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Platform {
    Twitch(PlatformInfo),
    Youtube(PlatformInfo),
    Facebook(PlatformInfo),
    Douyu(PlatformInfo),
    Discord(PlatformInfo),
}

type LogoPath = String;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PlatformInfo {
    logo: LogoPath,
    url: BaseUrl,
}
