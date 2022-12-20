use serde_json::Value;
use std::borrow::Cow;
use crate::errors::*;
use crate::requests::*;
use url::Url;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
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
    req.set_body(body);
    let res = req.send().await?;

    let headers = res.headers();
    // find set-cookie headers
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

#[derive(Debug, Default)]
pub struct LoginData {
    pub access_token: String,
    expires_in: usize,
    id_token: String,
}

pub async fn login(client: &Client, asid: &String, username: &str, password: &str) -> Result<LoginData> {
    const URL: &str = "https://auth.riotgames.com/api/v1/authorization";

    let body = hashmap![
        "type" => "auth",
        "username" => username,
        "password" => password
    ];

    let mut req = Request::new(client, URL.to_string(), Method::PUT);
    req.set_body(body);
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

                let mut login_data = LoginData::default();

                if let Some(access_token) = pairs.filter(|(key, _)| key == "access_token").next() {
                    login_data.access_token = access_token.1.to_string();
                }

                if let Some(expires_in) = pairs.filter(|(key, _)| key == "expires_in").next() {
                    login_data.expires_in = expires_in.1.parse::<usize>().unwrap();
                }

                if let Some(id_token) = pairs.filter(|(key, _)| key == "id_token").next() {
                    login_data.id_token = id_token.1.to_string();
                }

                return Ok(login_data);
            }
        }
    }

    Err("Failed to login".into())
}

pub async fn entitlements(client: &Client, asid: &String, access_token: &String) -> Result<String> {
    const URL: &str = "https://entitlements.auth.riotgames.com/api/token/v1";

    let mut req = Request::new(client, URL.to_string(), Method::POST);

    req.add_header("Authorization", format!("Bearer {}", access_token));
    req.add_header("Cookie", format!("asid={}", asid));

    let res = req.send().await?;

    let json: Value = serde_json::from_str(res.text().await?.as_str())?;

    match json.get("entitlements_token") {
        Some(entitlements_token) => Ok(entitlements_token.to_string().replace("\"", "")),
        None => Err("Failed to get entitlements token".into())
    }
}