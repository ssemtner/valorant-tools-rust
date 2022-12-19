use std::collections::HashMap;
use std::io::Read;

mod tls;

mod errors {
    error_chain::error_chain! {}
}

fn main() {
    let tls_config = tls::create_tls_config().expect("Failed to create TLS config");
    let client = reqwest::blocking::ClientBuilder::new()
        .use_preconfigured_tls(tls_config)
        .build().expect("a");
    // let mut res = client.get("https://randomuser.me/api").send()?;
    // let mut body = String::new();
    // res.read_to_string(&mut body)?;
    let mut body = HashMap::new();
    body.insert("client_id", "play-valorant-web-prod");
    body.insert("nonce", "1");
    body.insert("redirect_uri", "https://playvalorant.com/opt_in");
    body.insert("response_type", "token id_token");
    body.insert("scope", "account openid");
    let mut res = client
        .post("https://auth.riotgames.com/api/v1/authorization")
        .json(&body)
        .header("Content-Type", "application/json")
        .header("Referer", "https://auth.riotgames.com/")
        .header(
            "User-Agent",
            "RiotClient/60.0.10.4802528.4749685 rso-auth (Windows; 10;;Professional, x64)",
        )
        .header("Cookie", "")
        .send().expect("no send");
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);
}
