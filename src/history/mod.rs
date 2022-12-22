use anyhow::Result;
use rocket::serde::Serialize;

use crate::r#match::Match;
use crate::auth::AuthData;
use crate::common;
use crate::requests::{Client, Method, Request};

#[derive(Debug, Serialize)]
pub enum MatchType {
    Competitive,
    Unrated,
    Deathmatch,
    Other(String),
}

#[derive(Debug, Serialize)]
pub struct MatchHistoryEntry {
    pub match_id: String,
    pub match_type: MatchType,
    pub start_time: u64,
    pub game_data: Option<Match>,
}

impl From<&response::MatchHistoryEntry> for MatchHistoryEntry {
    fn from(match_: &response::MatchHistoryEntry) -> Self {
        let match_type = match match_.match_type.as_str() {
            "competitive" => MatchType::Competitive,
            "unrated" => MatchType::Unrated,
            "deathmatch" => MatchType::Deathmatch,
            _ => MatchType::Other(match_.match_type.clone()),
        };

        Self {
            match_id: match_.match_id.clone(),
            match_type,
            start_time: match_.start_time.clone(),
            game_data: None,
        }
    }
}

pub async fn get_match_history(client: &Client, auth_data: &AuthData, player_id: &str) -> Result<Vec<MatchHistoryEntry>> {
    let url = format!(
        "https://pd.{}.a.pvp.net/match-history/v1/history/{}?queue=custom",
        "na", player_id /*auth_data.user_id*/
    );

    let mut req = Request::new(client, url, Method::GET);
    req.append_headers(auth_data.get_auth_headers());
    req.append_headers(common::get_client_headers().await?);

    let res = req.send().await?;

    let history: response::History = res.json().await?;
    let mut history: Vec<MatchHistoryEntry> = history.matches.iter().map(|m| m.into()).collect();

    for match_ in history.iter_mut() {
        match_.game_data = Some(Match::from_id(client, auth_data, &match_.match_id).await?);
    }

    Ok(history)
}

mod response {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct History {
        #[serde(rename = "History")]
        pub matches: Vec<MatchHistoryEntry>,
    }

    #[derive(Debug, Deserialize)]
    pub struct MatchHistoryEntry {
        #[serde(rename = "MatchID")]
        pub match_id: String,

        #[serde(rename = "QueueID")]
        pub match_type: String,

        #[serde(rename = "GameStartTime")]
        pub start_time: u64,
    }
}
