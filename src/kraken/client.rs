use crate::kraken::env::KrakenCredentials;
use crate::kraken::payload::{
    self, AssetInfoInput, AssetInfoResponse, AssetPairsInfo, AssetPairsInput, AssetPairsResponse,
    RawRecentSpreadsResponse, RawTickerResponse, RecentSpreadsInput, RecentSpreadsResponse,
    SerializableAssetPairsInput, TickerInput, TickerResponse,
};
use crate::kraken::ratelimiter::LeakyBucket;
use crate::kraken::request_builder::{ParamEncoding, PrivacyLevel, RequestBuilder};
use crate::kraken::signature::get_kraken_signature;
use crate::kraken::AccountTier;
use crate::kraken::{
    endpoint, AssetPair, ACCOUNT_BALANCE, ASSET_INFO, ASSET_PAIRS, OPEN_ORDERS, RECENT_SPREADS,
    SYSTEM_STATUS, SYSTEM_TIME, TICKER, TRADE_BALANCE,
};
use chrono::prelude::*;
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use reqwest::Method;
use std::error::Error;

pub struct Client {
    http: reqwest::Client,
    api_key: String,
    private_key: String,
    rate_limiter: LeakyBucket,
}

impl Client {
    pub fn new(creds: KrakenCredentials, tier: AccountTier) -> Self {
        // First, create a new reqwest client.
        let client = reqwest::Client::new();
        // Set the API Key and the Private Key.
        Self {
            http: client,
            api_key: creds.api_key().to_string(),
            private_key: creds.private_key().to_string(),
            rate_limiter: LeakyBucket::new(tier),
        }
    }

    async fn use_rate_limit(&self, count: usize) {
        self.rate_limiter.use_rate_limit(count).await
    }

    fn nonce(&self) -> String {
        let utc: DateTime<Utc> = Utc::now();
        let ms = utc.timestamp_millis();
        ms.to_string()
    }

    pub async fn server_time(&self) -> Result<payload::ServerTimeResponse, reqwest::Error> {
        self.use_rate_limit(1).await;
        let client = &self.http;
        let req = RequestBuilder::<()> {
            method: Method::GET,
            url: endpoint(SYSTEM_TIME),
            params: None,
            param_encoding: ParamEncoding::FormEncoded,
            privacy_level: PrivacyLevel::Public,
        };
        let resp = req.execute(client).await?;
        Ok(resp)
    }

    pub async fn system_status(&self) -> Result<payload::SystemStatusResponse, reqwest::Error> {
        self.use_rate_limit(1).await;
        let client = &self.http;
        let req = RequestBuilder::<()> {
            method: Method::GET,
            url: endpoint(SYSTEM_STATUS),
            params: None,
            param_encoding: ParamEncoding::QueryEncoded,
            privacy_level: PrivacyLevel::Public,
        };
        let resp = req.execute(client).await?;
        Ok(resp)
    }

    pub async fn account_balance(&self) -> Result<payload::AccountBalanceResponse, reqwest::Error> {
        self.use_rate_limit(1).await;
        let nonce = self.nonce();
        let client = &self.http;
        let req = RequestBuilder {
            method: Method::POST,
            url: endpoint(ACCOUNT_BALANCE),
            param_encoding: ParamEncoding::FormEncoded,
            params: Some(payload::AccountBalanceInput {
                nonce: nonce.clone(),
            }),
            privacy_level: PrivacyLevel::Private {
                nonce,
                api_key: self.api_key.clone(),
                private_key: self.private_key.clone(),
            },
        };
        let resp = req.execute(client).await?;
        Ok(resp)
    }

    pub async fn asset_info(
        &self,
        asset: Option<String>,
        asset_class: Option<String>,
    ) -> Result<AssetInfoResponse, reqwest::Error> {
        self.use_rate_limit(1).await;
        let client = &self.http;
        let req = RequestBuilder {
            method: Method::GET,
            url: endpoint(ASSET_INFO),
            param_encoding: ParamEncoding::FormEncoded,
            params: Some(AssetInfoInput { asset, asset_class }),
            privacy_level: PrivacyLevel::Public,
        };
        let resp: AssetInfoResponse = req.execute(client).await?;
        Ok(resp)
    }

    pub async fn recent_spreads(
        &self,
        pair: String,
        since: Option<u64>,
    ) -> Result<RecentSpreadsResponse, reqwest::Error> {
        self.use_rate_limit(1).await;
        let client = &self.http;
        let req = RequestBuilder {
            method: Method::GET,
            url: endpoint(RECENT_SPREADS),
            param_encoding: ParamEncoding::QueryEncoded,
            params: Some(RecentSpreadsInput { pair, since }),
            privacy_level: PrivacyLevel::Public,
        };
        let resp: RawRecentSpreadsResponse = req.execute(client).await?;
        Ok(RecentSpreadsResponse::from(resp))
    }

    pub async fn asset_pairs(
        &self,
        pairs: Vec<String>,
        info: Option<AssetPairsInfo>,
    ) -> Result<AssetPairsResponse, reqwest::Error> {
        self.use_rate_limit(1).await;
        let client = &self.http;
        let user_input = AssetPairsInput { pairs, info };
        let serializable_input = SerializableAssetPairsInput::from(user_input);
        let req = RequestBuilder {
            method: Method::GET,
            url: endpoint(ASSET_PAIRS),
            param_encoding: ParamEncoding::QueryEncoded,
            params: Some(serializable_input),
            privacy_level: PrivacyLevel::Public,
        };
        let resp = req.execute(client).await?;
        Ok(resp)
    }

    pub async fn ticker(&self, asset_pair: AssetPair) -> Result<TickerResponse, Box<dyn Error>> {
        self.use_rate_limit(1).await;
        let pair = asset_pair.to_string();
        let client = &self.http;
        let req = RequestBuilder {
            method: Method::GET,
            url: endpoint(TICKER),
            param_encoding: ParamEncoding::QueryEncoded,
            params: Some(TickerInput { pair }),
            privacy_level: PrivacyLevel::Public,
        };
        let resp: RawTickerResponse = req.execute(client).await?;
        let ticker = TickerResponse::try_from(resp)?;
        Ok(ticker)
    }

    ///////////////////////////////////////////////////////////////////////////
    // Everything under this line does not strongly type their responses. /////
    ///////////////////////////////////////////////////////////////////////////

    pub async fn open_orders(
        &self,
        trades: Option<bool>,
        user_ref: Option<u32>,
    ) -> Result<String, reqwest::Error> {
        self.use_rate_limit(1).await;
        let nonce = self.nonce();
        let method = Method::POST;
        let api_key = &self.api_key;
        let content_type = "application/x-www-form-urlencoded; charset=utf-8";
        let url = endpoint(OPEN_ORDERS);
        let private_key = self.private_key.clone();
        let form_param = payload::OpenOrdersInput {
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

    pub async fn trade_balance(&self, asset: Option<String>) -> Result<String, reqwest::Error> {
        self.use_rate_limit(1).await;
        let nonce = self.nonce();
        let method = Method::POST;
        let api_key = &self.api_key;
        let content_type = "application/x-www-form-urlencoded; charset=utf-8";
        let url = endpoint(TRADE_BALANCE);
        let private_key = self.private_key.clone();
        let form_param = payload::TradeBalanceInput {
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

    pub async fn debug_recent_spreads(
        &self,
        pair: String,
        since: Option<u64>,
    ) -> Result<String, reqwest::Error> {
        self.use_rate_limit(1).await;
        let client = &self.http;
        let req = RequestBuilder {
            method: Method::GET,
            url: endpoint(RECENT_SPREADS),
            param_encoding: ParamEncoding::QueryEncoded,
            params: Some(RecentSpreadsInput { pair, since }),
            privacy_level: PrivacyLevel::Public,
        };
        let resp = req.debug(client).await?;
        Ok(resp)
    }
}
