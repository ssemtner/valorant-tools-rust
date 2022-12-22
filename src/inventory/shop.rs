use serde::Serialize;
use serde_json::Value;

use crate::auth::AuthData;
use anyhow::{anyhow, Result};
use crate::requests::*;

#[derive(Debug, Serialize)]
pub struct Offer {
    pub weapon_id: String,
}

pub async fn get_shop(client: &Client, auth_data: &AuthData) -> Result<Vec<Offer>> {
    let url = format!("https://pd.{}.a.pvp.net/store/v2/storefront/{}", "na", auth_data.user_id);

    let mut req = Request::new(client, url, Method::GET);
    req.append_headers(auth_data.get_auth_headers());

    let res = req.send().await?;

    let json = res.json::<Value>().await?;

    // println!("{:?}", json);

    let offers = json.get("SkinsPanelLayout").unwrap().get("SingleItemOffers").unwrap();

    return match offers.as_array() {
        Some(offers) => {
            Ok(offers
                .iter()
                .map(|offer| Offer {
                    weapon_id: offer.as_str().unwrap().to_string(),
                })
                .collect())
        },
        None => Err(anyhow!("Failed to parse shop")),
    }
}