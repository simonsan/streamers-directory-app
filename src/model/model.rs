enum EloRange {
    Greater2k,
    Greater1k4,
    Greater800,
}

enum Category {
    CastingRanked(EloRange),
    CastingTournaments,
    OrganizingTournaments,
    CommunityGames,
    PovContent(EloRange),
    LearningResources,
}

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

enum Platform {
    Twitch(StreamerUrl),
    Youtube(StreamerUrl),
    Facebook(StreamerUrl),
    Douyu(StreamerUrl),
    Discord(StreamerUrl),
}

struct Elo(u64);

struct ContentCreator {
    id: usize,
    name: String,
    platforms: Vec<(
        Platform,
        Vec<Category>,
        Vec<LanguageShortCode>,
        Vec<GameShortCode>,
    )>,
    elo: Option<Elo>,
    aoc_ref_name: Option<String>,
}
