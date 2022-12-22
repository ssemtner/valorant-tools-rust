use crate::requests::*;
use crate::auth::AuthData;
use anyhow::Result;

pub struct Player {}

impl Player {
    pub async fn from_player_id(client: &Client, auth_data: &AuthData, player_id: &str) -> Result<Player> {
        let url = format!(
            "https://pd.{}.a.pvp.net/name-service/v2/players?subject={}",
            "na", player_id
        );

        let mut req = Request::new(client, url, Method::GET);
        req.append_headers(auth_data.get_auth_headers());

        let res = req.send().await?;

        let data = res.json().await?;

        println!("{:?}", data);

        Ok(Player {})
    }
}