use crate::kraken::{endpoint, AssetPair, OPEN_ORDERS, SYSTEM_TIME, SYSTEM_STATUS, ASSETS, TICKER, ACCOUNT_BALANCE, TRADE_BALANCE, signature::SignatureInput};
use crate::kraken::payload;
use chrono::prelude::*;
use std::time::Duration;
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use reqwest::{Method, Request, Url};
use tower::service_fn;
use tower::{Service, ServiceExt};

pub struct Client {
    http: reqwest::Client,
    api_key: String,
    private_key: String,
}

impl Client {
    pub fn new(api_key: String, private_key: String) -> Self {
        // First, create a new reqwest client.
        let client = reqwest::Client::new();
        // Set the API Key and the Private Key.
        Self {
            http: client,
            api_key,
            private_key,
        }
    }

    pub async fn account_balance(&self) -> Result<String, reqwest::Error> {
        let nonce = self.nonce();
        let method = Method::POST;
        let api_key = &self.api_key;
        let content_type = "application/x-www-form-urlencoded; charset=utf-8";
        let url = endpoint(ACCOUNT_BALANCE);
        let form_param = payload::AccountBalanceInput{
            nonce: nonce.clone(),
        };
        // Next, we have to attach the API Key header.
        let mut req = self
            .http
            .request(method, url)
            .form(&form_param)
            .header("API-Key", api_key)
            .header(CONTENT_TYPE, content_type)
            .build()?;
        let signature = self.get_kraken_signature(nonce, &req);
        // We also need to attach the API-Sign header.
        let api_sign = HeaderValue::from_str(&signature).unwrap();
        req.headers_mut().insert("API-Sign", api_sign);
        let resp = self.http.execute(req).await?.text().await?;
        Ok(resp)
    }

    pub async fn open_orders(&self, trades: Option<bool>, user_ref: Option<u32>) -> Result<String, reqwest::Error> {
        let nonce = self.nonce();
        let method = Method::POST;
        let api_key = &self.api_key;
        let content_type = "application/x-www-form-urlencoded; charset=utf-8";
        let url = endpoint(OPEN_ORDERS);
        let form_param = payload::OpenOrdersInput{
            nonce: nonce.clone(),
            trades,
            user_ref,
        };
        // Next, we have to attach the API Key header.
        let mut req = self
            .http
            .request(method, url)
            .form(&form_param)
            .header("API-Key", api_key)
            .header(CONTENT_TYPE, content_type)
            .build()?;
        let signature = self.get_kraken_signature(nonce, &req);
        // We also need to attach the API-Sign header.
        let api_sign = HeaderValue::from_str(&signature).unwrap();
        req.headers_mut().insert("API-Sign", api_sign);
        let resp = self.http.execute(req).await?.text().await?;
        Ok(resp)
    }

    pub fn get_kraken_signature(&self, nonce: String, req: &Request) -> String {
        let path = req.url().path();
        let req_body = req.body().unwrap().as_bytes().unwrap().to_vec();
        let body_str = String::from_utf8(req_body).unwrap();
        // Here, we need to calculat the API-Sign
        let signature = SignatureInput{
            private_key: self.private_key.clone(),
            nonce,
            encoded_payload: body_str,
            uri_path: path.to_owned(),
        };
        signature.sign()
    }

    pub async fn server_time(&self) -> Result<payload::ServerTimeResponse, reqwest::Error> {
        let method = Method::GET;
        let url = endpoint(SYSTEM_TIME);
        let resp = self.http.request(method, url)
            .send()
            .await?
            .json::<payload::ServerTimeResponse>()
            .await?;
        Ok(resp)
    }

    pub async fn system_status(&self) -> Result<payload::SystemStatusResponse, reqwest::Error> {
        let method = Method::GET;
        let url = endpoint(SYSTEM_STATUS);
        let resp = self.http.request(method, url)
            .send()
            .await?
            .json::<payload::SystemStatusResponse>()
            .await?;
        Ok(resp)
    }

    pub async fn assets(&self) -> Result<String, reqwest::Error> {
        let method = Method::GET;
        let url = endpoint(ASSETS);
        let resp = self.http.request(method, url).send().await?.text().await?;
        Ok(resp)
    }

    pub async fn ticker(&self, asset: AssetPair) -> Result<String, reqwest::Error> {
        // Clone the current HTTP client.
        let method = Method::GET;
        let url = endpoint(TICKER);
        let query_param = &[("pair", &asset.to_string())];

        let req = self.http.request(method, url).query(query_param).build()?;
        let resp = self.http.execute(req).await?.text().await?;
        Ok(resp)
    }

    pub async fn trade_balance(&self, asset: Option<String>) ->  Result<String, reqwest::Error> {
        let nonce = self.nonce();
        let method = Method::POST;
        let api_key = &self.api_key;
        let content_type = "application/x-www-form-urlencoded; charset=utf-8";
        let url = endpoint(TRADE_BALANCE);
        let form_param = payload::TradeBalanceInput{
            nonce: nonce.clone(),
            asset,
        };
        // Next, we have to attach the API Key header.
        let mut req = self
            .http
            .request(method, url)
            .form(&form_param)
            .header("API-Key", api_key)
            .header(CONTENT_TYPE, content_type)
            .build()?;
        let signature = self.get_kraken_signature(nonce, &req);
        // We also need to attach the API-Sign header.
        let api_sign = HeaderValue::from_str(&signature).unwrap();
        req.headers_mut().insert("API-Sign", api_sign);
        let resp = self.http.execute(req).await?.text().await?;
        Ok(resp)
    }

    fn nonce(&self) -> String {
        let utc: DateTime<Utc> = Utc::now();
        let ms = utc.timestamp_millis();
        ms.to_string()
    }

    pub async fn make_request(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Clone the current HTTP client.
        let client = self.http.clone();
        // Build the Tower service.
        let mut svc = tower::ServiceBuilder::new()
            .rate_limit(100, Duration::new(10, 0)) // 100 requests every 10 seconds
            .service(service_fn(move |req| client.execute(req)));
        // Make the request which you're about to feed to
        // the Tower service.
        let req = Request::new(Method::GET, Url::parse("https://httpbin.org/ip")?);
        // Send the request and await the response.
        let resp = svc.ready().await?.call(req).await?.text().await?;
        println!("{:#?}", resp);
        Ok(())
    }
}
