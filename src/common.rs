use std::collections::HashMap;

use anyhow::Result;
use serde_json::Value;

pub async fn get_client_headers() -> Result<HashMap<String, String>> {
    const CLIENT_PLATFORM: &str = "ew0KCSJwbGF0Zm9ybVR5cGUiOiAiUEMiLA0KCSJwbGF0Zm9ybU9TIjogIldpbmRvd3MiLA0KCSJwbGF0Zm9ybU9TVmVyc2lvbiI6ICIxMC4wLjE5MDQyLjEuMjU2LjY0Yml0IiwNCgkicGxhdGZvcm1DaGlwc2V0IjogIlVua25vd24iDQp9";

    let version = reqwest::get("https://valorant-api.com/v1/version")
        .await?
        .json::<Value>()
        .await?
        .get("data")
        .expect("No data in version response")
        .get("riotClientVersion")
        .expect("No riotClientVersion in version response")
        .to_string();

    let mut headers = HashMap::new();

    headers.insert("X-Riot-ClientVersion".parse().unwrap(), version);

    headers.insert(
        "X-Riot-ClientPlatform".parse().unwrap(),
        CLIENT_PLATFORM.parse().unwrap(),
    );

    Ok(headers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_client_headers() {
        let headers = get_client_headers().await.unwrap();

        assert!(headers.contains_key("X-Riot-ClientVersion"));
        assert!(headers.contains_key("X-Riot-ClientPlatform"));
    }
}
