use crate::kraken::{endpoint, AssetPair, RECENT_SPREADS, OPEN_ORDERS, SYSTEM_TIME, SYSTEM_STATUS, ASSET_INFO, TICKER, ACCOUNT_BALANCE, TRADE_BALANCE};
use crate::kraken::payload::{self, AssetInfoInput, AssetInfoResponse};
use crate::kraken::signature::get_kraken_signature;
use crate::kraken::request_builder::{PrivacyLevel, RequestBuilder};
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

    fn nonce(&self) -> String {
        let utc: DateTime<Utc> = Utc::now();
        let ms = utc.timestamp_millis();
        ms.to_string()
    }

    pub async fn server_time(&self) -> Result<payload::ServerTimeResponse, reqwest::Error> {
        let client = &self.http;
        let req = RequestBuilder::<()>{
            method: Method::GET,
            url: endpoint(SYSTEM_TIME),
            form_params: None,
            privacy_level: PrivacyLevel::Public,
        };
        let resp = req.execute(client).await?;
        Ok(resp)
    }

    pub async fn system_status(&self) -> Result<payload::SystemStatusResponse, reqwest::Error> {
        let client = &self.http;
        let req = RequestBuilder::<()> {
            method: Method::GET,
            url: endpoint(SYSTEM_STATUS),
            form_params: None,
            privacy_level: PrivacyLevel::Public,
        };
        let resp = req.execute(client).await?;
        Ok(resp)
    }

    pub async fn account_balance(&self) -> Result<payload::AccountBalanceResponse, reqwest::Error> {
        let nonce = self.nonce();
        let client = &self.http;
        let req = RequestBuilder {
            method: Method::POST,
            url: endpoint(ACCOUNT_BALANCE),
            form_params: Some(payload::AccountBalanceInput{
                nonce: nonce.clone(),
            }),
            privacy_level: PrivacyLevel::Private{
                nonce,
                api_key: self.api_key.clone(),
                private_key: self.private_key.clone(),
            },
        };
        let resp = req.execute(client).await?;
        Ok(resp)
    }

    pub async fn asset_info(&self, asset: Option<String>, asset_class: Option<String>) -> Result<AssetInfoResponse, reqwest::Error> {
        let client = &self.http;
        let req = RequestBuilder {
            method: Method::GET,
            url: endpoint(ASSET_INFO),
            form_params: Some(AssetInfoInput{asset, asset_class}),
            privacy_level: PrivacyLevel::Public,
        };
        let resp: AssetInfoResponse = req.execute(client).await?;
        Ok(resp)
    }

///////////////////////////////////////////////////////////////////////////
// Everything under this line does not strongly type their responses. /////
///////////////////////////////////////////////////////////////////////////

    pub async fn recent_spreads(&self, pair: String, since: Option<u64>) -> Result<payload::RecentSpreadsResponse, reqwest::Error> {
        let method = Method::GET;
        let url = endpoint(RECENT_SPREADS);
        let query_param = payload::RecentSpreadsInput{pair, since};
        // Next, we have to attach the API Key header.
        let req = self
            .http
            .request(method, url)
            .query(&query_param)
            .build()?;
        let resp = self.http.execute(req)
            .await?
            .json::<payload::RecentSpreadsResponse>()
            .await?;
        Ok(resp)
    }

    pub async fn open_orders(&self, trades: Option<bool>, user_ref: Option<u32>) -> Result<String, reqwest::Error> {
        let nonce = self.nonce();
        let method = Method::POST;
        let api_key = &self.api_key;
        let content_type = "application/x-www-form-urlencoded; charset=utf-8";
        let url = endpoint(OPEN_ORDERS);
        let private_key = self.private_key.clone();
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
        let signature = get_kraken_signature(nonce, private_key, &req);
        // We also need to attach the API-Sign header.
        let api_sign = HeaderValue::from_str(&signature).unwrap();
        req.headers_mut().insert("API-Sign", api_sign);
        let resp = self.http.execute(req).await?.text().await?;
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
        let private_key = self.private_key.clone();
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
        let signature = get_kraken_signature(nonce, private_key, &req);
        // We also need to attach the API-Sign header.
        let api_sign = HeaderValue::from_str(&signature).unwrap();
        req.headers_mut().insert("API-Sign", api_sign);
        let resp = self.http.execute(req).await?.text().await?;
        Ok(resp)
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
