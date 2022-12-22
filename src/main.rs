#[macro_use]
extern crate rocket;

use rocket::http::{Cookie, Status};
use rocket::response::{content, status};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::http::CookieJar;

use requests::*;

use crate::auth::AuthData;
use crate::history::MatchHistoryEntry;

mod r#match;
mod auth;
mod requests;
mod tls;
mod common;
mod inventory;
mod history;
mod player;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        // .mount("/", routes![index])
        .mount("/auth", routes![login])
        .mount("/history", routes![get_match_history])
        .launch().await;
}

// #[tokio::main]
// #[get("/")]
// async fn index() -> Json<Vec<history::MatchHistoryEntry>> {
//     let client = Client::new().unwrap();
//     let auth_data = auth::authenticate(&client, "", "").await.unwrap();
//     // println!("{:?}", &auth_data);
//     //
//
//     let history = history::get_match_history(&client, &auth_data, &auth_data.user_id).await.unwrap();
//
//     // let wallet = inventory::get_wallet(&client, &auth_data).await.unwrap();
//     //
//     // println!("{:?}", wallet);
//     //
//     let shop = inventory::get_shop(&client, &auth_data).await.unwrap();
//     // println!("{:?}", shop);
//
//     Json(history)
// }

#[derive(Deserialize, Debug, Serialize)]
struct AuthRequest<'a> {
    username: &'a str,
    password: &'a str,
}

#[post("/login", data = "<auth_request>")]
async fn login(cookie_jar: &CookieJar<'_>, auth_request: Json<AuthRequest<'_>>) -> Json<AuthData> {
    let client = Client::new().unwrap();
    let auth_data = auth::authenticate(&client, auth_request.username, auth_request.password).await.unwrap();

    cookie_jar.add(Cookie::new("auth_data", serde_json::to_string(&auth_data).unwrap()));

    Json(auth_data)
}

#[derive(Serialize, Deserialize)]
struct MatchHistoryRequest<'a> {
    player_id: &'a str,
}

#[get("/", data = "<req_data>")]
async fn get_match_history(cookie_jar: &CookieJar<'_>, req_data: Json<MatchHistoryRequest<'_>>) -> Json<Vec<MatchHistoryEntry>> {
    let client = Client::new().unwrap();

    println!("{:?}", req_data.player_id);
    let auth_data = serde_json::from_str::<AuthData>(cookie_jar.get("auth_data").unwrap().value()).unwrap();

    let history = history::get_match_history(&client, &auth_data, req_data.player_id).await.unwrap();

    Json(history)
}

#[macro_export] macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
        let mut map = ::std::collections::HashMap::new();
        $( map.insert($key, $val); )*
        map
    }}
}
