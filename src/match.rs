use std::str::FromStr;
use crate::auth::AuthData;
use crate::requests::*;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type PlayerId = String;
pub type MapId = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Match {
    pub bots: Vec<Value>,
    pub coaches: Vec<Value>,
    pub kills: Vec<Kill>,

    #[serde(rename(deserialize = "matchInfo"))]
    pub match_info: MatchInfo,

    pub players: Vec<Player>,

    #[serde(rename(deserialize = "roundResults"))]
    pub round_results: Vec<RoundResult>,

    pub teams: Vec<Team>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Kill {
    pub assistants: Vec<PlayerId>,
    pub killer: PlayerId,
    pub victim: PlayerId,
    // player locations
    pub round: u32,

    #[serde(rename(deserialize = "roundTime"))]
    pub round_time: u32,

    #[serde(rename(deserialize = "gameTime"))]
    pub game_time: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchInfo {
    #[serde(rename(deserialize = "gameLengthMillis"))]
    pub game_length: u64,
    #[serde(rename(deserialize = "queueID"))]
    pub queue_id: String,
    #[serde(rename(deserialize = "mapId"))]
    pub map_id: MapId,
    #[serde(rename(deserialize = "isRanked"))]
    pub is_ranked: bool,
}

// missing things: behavioural factors, player card, player title, level border
#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    #[serde(rename(deserialize = "accountLevel"))]
    pub level: u32,
    #[serde(rename(deserialize = "subject"))]
    pub id: PlayerId,

    #[serde(rename(deserialize = "gameName"))]
    pub ign: String,

    #[serde(rename(deserialize = "tagLine"))]
    pub tagline: String,

    #[serde(rename(deserialize = "competitiveTier"))]
    pub rank: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundResult {
    #[serde(rename(deserialize = "winningTeam"))]
    pub winning_team: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    #[serde(rename(deserialize = "teamId"))]
    side: String,
    #[serde(rename(deserialize = "roundsWon"))]
    rounds_won: u32,
    won: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TeamColor {
    Red,
    Blue
}

impl Match {
    pub async fn from_id(client: &Client, auth_data: &AuthData, game_id: &str) -> Result<Self> {
        let url = format!(
            "https://pd.{}.a.pvp.net/match-details/v1/matches/{}",
            "na", game_id
        );

        let mut req = Request::new(client, url, Method::GET);
        req.append_headers(auth_data.get_auth_headers());

        let res = req.send().await?;

        let data = res.json::<Match>().await?;

        Ok(data)
    }
}
