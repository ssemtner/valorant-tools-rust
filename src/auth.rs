use std::collections::HashMap;

use anyhow::{Error, Result};
use rocket::http::Status;
use rocket::request::FromRequest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use url::Url;

use rocket::request::Outcome;

use crate::{AuthRequest, hashmap};
use crate::requests::*;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Parse error")]
    ParseError,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AuthData {
    pub access_token: String,
    pub expires_in: usize,
    pub id_token: String,
    pub entitlements_token: String,
    pub user_id: String,
    pub ign: String,
    pub tagline: String,
}

// impl<'a> FromRequest<'a> for AuthData {
//     type Error = ();
//
//     fn from_request(req: &'a rocket::Request) -> Outcome<Self, Self::Error> {
//         let token = req.headers().get_one("auth_data");
//         match token {
//             Some(token) => {
//                 // check validity
//                 Outcome::Success(serde_json::from_str::<AuthData>(token).unwrap())
//             },
//             // token does not exist
//             None => Outcome::Failure((Status::Unauthorized, ()))
//         }
//     }
// }

impl AuthData {
    pub fn get_auth_headers<'a>(&self) -> HashMap<&str, String> {
        let mut headers = HashMap::new();

        headers.insert("Authorization", format!("Bearer {}", self.access_token));

        headers.insert(
            "X-Riot-Entitlements-JWT",
            format!("{}", self.entitlements_token),
        );

        headers
    }
}

pub async fn authenticate(client: &Client, username: &str, password: &str) -> Result<AuthData> {
    let asid = handshake(&client).await?;
    let (access_token, expires_in, id_token) = login(&client, &asid, username, password).await?;
    let entitlements_token = get_entitlements(&client, &asid, &access_token).await?;
    let (user_id, ign, tagline) = get_user_info(&client, &access_token).await?;

    Ok(AuthData {
        access_token,
        expires_in,
        id_token,
        entitlements_token,
        user_id,
        ign,
        tagline,
    })
}

pub async fn handshake(client: &Client) -> Result<String> {
    const URL: &str = "https://auth.riotgames.com/api/v1/authorization";
    let body = hashmap![
        "client_id" => "play-valorant-web-prod",
        "nonce" => "1",
        "redirect_uri" => "https://playvalorant.com/opt_in",
        "response_type" => "token id_token",
        "scope" => "account openid"
    ];
    let mut req = Request::new(client, URL.to_string(), Method::POST);
    req.append_body(body);
    let res = req.send().await?;

    let headers = res.headers();
    let asid = headers
        .get_all("set-cookie")
        .iter()
        .filter(|h| h.to_str().unwrap().contains("asid"))
        .next()
        .expect("No asid cookie found")
        .to_str()
        .expect("Failed to convert asid cookie to str")
        .split('=')
        .nth(1)
        .expect("Could not parse asid cookie")
        .split(';')
        .next()
        .expect("Could not parse asid cookie");

    Ok(asid.to_string())
}

pub async fn login(
    client: &Client,
    asid: &String,
    username: &str,
    password: &str,
) -> Result<(String, usize, String)> {
    const URL: &str = "https://auth.riotgames.com/api/v1/authorization";

    let body = hashmap![
        "type" => "auth",
        "username" => username,
        "password" => password
    ];

    let mut req = Request::new(client, URL.to_string(), Method::PUT);
    req.append_body(body);
    req.add_header("Cookie", format!("asid={}", asid));

    let res = req.send().await?;

    let json: Value = serde_json::from_str(res.text().await?.as_str()).unwrap();

    if let Some(response) = json.get("response") {
        if let Some(parameters) = response.get("parameters") {
            if let Some(uri) = parameters.get("uri") {
                // Format so it can be parsed
                let uri = uri.to_string().replace("#", "?").replace("\"", "");

                let parsed = Url::parse(uri.as_str())?;

                let pairs = parsed.query_pairs();

                let access_token: String;
                let expires_in: usize;
                let id_token: String;

                match pairs.filter(|(k, _)| k == "access_token").next() {
                    Some((_, v)) => access_token = v.to_string(),
                    None => return Err(AuthError::ParseError.into()),
                }

                match pairs.filter(|(k, _)| k == "expires_in").next() {
                    Some((_, v)) => expires_in = v.parse().unwrap(),
                    None => return Err(AuthError::ParseError.into()),
                }

                match pairs.filter(|(k, _)| k == "id_token").next() {
                    Some((_, v)) => id_token = v.to_string(),
                    None => return Err(AuthError::ParseError.into()),
                }

                return Ok((access_token, expires_in, id_token));
            }
        }
    }

    Err(AuthError::InvalidCredentials.into())
}

pub async fn get_entitlements(
    client: &Client,
    asid: &String,
    access_token: &String,
) -> Result<String> {
    const URL: &str = "https://entitlements.auth.riotgames.com/api/token/v1";

    let mut req = Request::new(client, URL.to_string(), Method::POST);

    req.add_header("Authorization", format!("Bearer {}", access_token));
    req.add_header("Cookie", format!("asid={}", asid));

    let res = req.send().await?;

    let json: Value = serde_json::from_str(res.text().await?.as_str())?;

    match json.get("entitlements_token") {
        Some(entitlements_token) => Ok(entitlements_token.to_string().replace("\"", "")),
        None => Err(AuthError::ParseError.into()),
    }
}

async fn get_user_info(client: &Client, access_token: &String) -> Result<(String, String, String)> {
    const URL: &str = "https://auth.riotgames.com/userinfo";

    let mut req = Request::new(client, URL.to_string(), Method::GET);

    req.add_header("Authorization", format!("Bearer {}", access_token));

    let res = req.send().await?;

    let user_data: response::UserData = res.json().await?;

    Ok((
        user_data.user_id,
        user_data.account.game_name,
        user_data.account.tagline,
    ))
}

mod response {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct UserData {
        #[serde(rename = "sub")]
        pub user_id: String,

        #[serde(rename = "acct")]
        pub account: Account,
    }

    #[derive(Deserialize, Debug)]
    pub struct Account {
        #[serde(rename = "game_name")]
        pub game_name: String,

        #[serde(rename = "tag_line")]
        pub tagline: String,
    }
}

#[cfg(test)]
mod tests {}
