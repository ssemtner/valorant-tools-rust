use anyhow::Result;
use serde::Serialize;

use crate::auth::AuthData;
use crate::requests::*;

#[derive(Debug, Serialize)]
pub struct Wallet {
    pub valorant_points: u64,
    pub radianite_points: u64,
}

impl From<response::Wallet> for Wallet {
    fn from(wallet: response::Wallet) -> Self {
        Self {
            valorant_points: wallet.balances.valorant_points,
            radianite_points: wallet.balances.radianite_points,
        }
    }
}

pub async fn get_wallet(client: &Client, auth_data: &AuthData) -> Result<Wallet> {
    let url = format!(
        "https://pd.{}.a.pvp.net/store/v1/wallet/{}",
        "na", auth_data.user_id
    );

    let mut req = Request::new(client, url, Method::GET);
    req.append_headers(auth_data.get_auth_headers());

    let res = req.send().await?;

    let data: response::Wallet = res.json().await?;

    Ok(data.into())
}


mod response {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct Wallet {
        pub balances: Balances,
    }

    #[derive(Debug, Deserialize)]
    pub struct Balances {
        #[serde(rename = "85ad13f7-3d1b-5128-9eb2-7cd8ee0b5741")]
        pub valorant_points: u64,

        #[serde(rename = "e59aa87c-4cbf-517a-5983-6e81511be9b7")]
        pub radianite_points: u64,
    }
}
