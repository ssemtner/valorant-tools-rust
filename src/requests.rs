use std::collections::HashMap;

use reqwest::header;

use anyhow::Result;
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

    #[allow(dead_code)]
    pub fn add_body<TKey: ToString, TValue: ToString>(&mut self, key: TKey, value: TValue) {
        self.body.insert(key.to_string(), value.to_string());
    }

    pub fn append_headers<TKey: ToString, TValue: ToString>(&mut self, headers: HashMap<TKey, TValue>) {
        for (key, value) in headers {
            self.headers.insert(key.to_string(), value.to_string());
        }
    }

    pub fn append_body<TKey: ToString, TValue: ToString>(&mut self, headers: HashMap<TKey, TValue>) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_client_headers() {
        let client = Client::new().unwrap();

        let request = Request::new(&client, "https://valorant-api.com/v1/version".to_string(), Method::GET);

        let response = request.send().await.unwrap();

        assert!(response.status().is_success());
    }

    #[tokio::test]
    async fn test_create_client() {
        let client = Client::new();

        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_append_headers() {
        let client = Client::new().unwrap();

        let mut request = Request::new(&client, "https://google.com/".to_string(), Method::GET);

        let mut headers = HashMap::new();
        headers.insert("Test", "abc");
        headers.insert("y", "x");

        request.append_headers(headers);

        assert_eq!(request.headers.get("Test").unwrap(), "abc");
        assert_eq!(request.headers.len(), 2);
    }
}