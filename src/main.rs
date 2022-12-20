use auth::handshake;
use reqwest;
use std::collections::HashMap;
use requests::*;

mod auth;
mod requests;
mod tls;

mod errors {
    error_chain::error_chain! {
        foreign_links {
            Io(std::io::Error);
            HttpRequest(reqwest::Error);
            SerdeJson(serde_json::Error);
            Url(url::ParseError);
        }
    }
}

use errors::*;
use crate::auth::{entitlements, login};

#[tokio::main]
async fn main() -> Result<()> {
    // test::test();
    let client = Client::new()?;
    let asid = handshake(&client).await?;
    let ld = login(&client, &asid, "u", "p").await?;
    println!("{:?}", &ld);
    let ent = entitlements(&client, &asid, &ld.access_token).await?;
    println!("{}", &ent);

    // let req = Request::new(&client, "https://pd.na.a.pvp.net/mmr/v1/players/{puuid}".to_string(), Method::GET);


    // let client = reqwest::ClientBuilder::new()
    //     .use_preconfigured_tls(tls_config)
    //     .build()?;
    // // let mut res = client.get("https://randomuser.me/api").send()?;
    // // let mut body = String::new();
    // // res.read_to_string(&mut body)?;
    // let mut body = HashMap::new();
    // body.insert("client_id", "play-valorant-web-prod");
    // body.insert("nonce", "1");
    // body.insert("redirect_uri", "https://playvalorant.com/opt_in");
    // body.insert("response_type", "token id_token");
    // body.insert("scope", "account openid");
    //
    // let req = client
    //     .post("https://auth.riotgames.com/api/v1/authorization")
    //     .json(&body)
    //     .header("Content-Type", "application/json")
    //     .header(
    //         "User-Agent",
    //         "RiotClient/60.0.10.4802528.4749685 rso-auth (Windows; 10;;Professional, x64)",
    //     )
    //     .header("Cookie", "");
    //
    // let res = req.send().await?;
    //
    // // let mut body = String::new();
    // // res.read_to_string(&mut body)?;
    //
    // println!("Status: {}", res.status());
    // println!("Headers:\n{:#?}", res.headers());
    // println!("Body:\n{}", res.text().await?);

    Ok(())
}
