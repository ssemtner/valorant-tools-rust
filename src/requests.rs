use std::collections::HashMap;

use reqwest::header;
use reqwest::header::HeaderMap;

use crate::errors::*;
use crate::tls;

pub enum Method {
    GET,
    POST,
    PUT,
}

pub struct Request<'a> {
    client: &'a Client,
    url: String,
    method: Method,
    body: HashMap<String, String>,
    headers: HashMap<String, String>,
}

impl Request<'_> {
    pub fn new(client: &Client, url: String, method: Method) -> Request {
        Request {
            client,
            url,
            method,
            body: HashMap::new(),
            headers: HashMap::new(),
        }
    }

    pub fn add_header<TKey: ToString, TValue: ToString>(&mut self, key: TKey, value: TValue) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn add_body<TKey: ToString, TValue: ToString>(&mut self, key: TKey, value: TValue) {
        self.body.insert(key.to_string(), value.to_string());
    }

    pub fn set_header(&mut self, headers: HashMap<&str, &str>) {
        for (key, value) in headers {
            self.headers.insert(key.to_string(), value.to_string());
        }
    }

    pub fn set_body(&mut self, headers: HashMap<&str, &str>) {
        for (key, value) in headers {
            self.body.insert(key.to_string(), value.to_string());
        }
    }

    pub async fn send(&self) -> Result<reqwest::Response> {
        let mut req = match self.method {
            Method::GET => self.client.get(&self.url),
            Method::POST => self.client.post(&self.url),
            Method::PUT => self.client.put(&self.url),
        };

        for (key, value) in self.headers.iter() {
            req = req.header(key, value);
        }

        let res = match self.method {
            Method::GET => req.send().await?,
            Method::POST => req.json(&self.body).send().await?,
            Method::PUT => req.json(&self.body).send().await?,
        };

        Ok(res)
    }
}

#[derive(Debug)]
pub struct Client {
    reqwest_client: reqwest::Client,
}

impl Client {
    pub fn new() -> Result<Client> {
        let mut default_headers = header::HeaderMap::new();

        default_headers.insert(
            "User-Agent",
            header::HeaderValue::from_static(
                "RiotClient/60.0.10.4802528.4749685 rso-auth (Windows; 10;;Professional, x64)",
            ),
        );

        default_headers.insert("Cookie", header::HeaderValue::from_static(""));

        default_headers.insert(
            "Content-Type",
            header::HeaderValue::from_static("application/json"),
        );

        let tls_config = tls::create_tls_config()?;

        let reqwest_client = reqwest::ClientBuilder::new()
            .use_preconfigured_tls(tls_config)
            .default_headers(default_headers)
            .build()?;

        Ok(Client {
            reqwest_client,
        })
    }

    pub fn get(&self, url: &str) -> reqwest::RequestBuilder {
        self.reqwest_client.get(url)
    }

    pub fn post(&self, url: &str) -> reqwest::RequestBuilder {
        self.reqwest_client.post(url)
    }

    pub fn put(&self, url: &str) -> reqwest::RequestBuilder {
        self.reqwest_client.put(url)
    }
}
