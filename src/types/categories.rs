use serde::{
    Deserialize,
    Serialize,
};

use super::elo::EloRange;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Category {
    CastingRanked(EloRange),
    CastingTournaments,
    OrganizingTournaments,
    CommunityGames,
    PovContent(EloRange),
    LearningResources,
}
