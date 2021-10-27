//! Serve converted JSON of aoc-ref-data
//! from data/streamers.json directory

use yew::{
    callback::Callback,
    services::fetch::FetchTask,
};

use super::Requests;
use crate::{
    error::Error,
    types::Api as ApiJson,
};

/// Serve streamers from converted json
#[derive(Default, Debug)]
pub struct Players {
    requests: Requests,
}

impl Players {
    pub fn new() -> Self {
        Self { requests: Requests::new() }
    }

    /// Get all players
    pub fn all(&mut self,
               callback: Callback<Result<Vec<ApiJson>, Error>>)
               -> FetchTask {
        self.requests
            .get::<Vec<ApiJson>>(format!("/main/data/streamers.json"), callback)
    }
}
