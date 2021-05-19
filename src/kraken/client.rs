use crate::kraken::{endpoint, AssetPair, Endpoints};
use chrono::prelude::*;
use reqwest::{Method, Request, Url};
use std::time::Duration;
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

    pub async fn assets(&self) -> Result<String, reqwest::Error> {
        let method = Method::GET;
        let raw_url = endpoint(Endpoints::Assets);
        let url = Url::parse(&raw_url).unwrap();
        let resp = self.http.request(method, url).send().await?.text().await?;
        Ok(resp)
    }

    pub async fn ticker(&self, asset: AssetPair) -> Result<String, reqwest::Error> {
        // Clone the current HTTP client.
        let method = Method::GET;
        let raw_url = endpoint(Endpoints::Ticker);
        let url = Url::parse(&raw_url).unwrap();
        let query_param = &[("pair", &asset.to_string())];
        println!("Param: {:?}", query_param);
        println!("{}", asset.to_string());

        let req = self.http.request(method, url).query(query_param).build()?;
        println!("WAIT DID I MAKE IT THIS FAR?");
        println!("{:?}", req);
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
