use serde::ser::{Serialize};
use serde::de::Deserialize;
use reqwest::{Url, Method};
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use reqwest::{Request, RequestBuilder as ReqwestBuilder};
use crate::kraken::signature::get_kraken_signature;

const FORM_URLENCODED: &'static str = "application/x-www-form-urlencoded; charset=utf-8";

#[derive(Clone)]
pub struct RequestBuilder<F> where
    F: Serialize,
{
    pub method: Method,
    // Must be provided.
    pub url: Url,
    // Not always required.
    pub params: Option<F>,
    pub param_encoding: ParamEncoding,
    pub privacy_level: PrivacyLevel,
}

#[derive(Clone, Copy)]
pub enum ParamEncoding {
    FormEncoded,
    QueryEncoded,
}

#[derive(Clone)]
pub enum PrivacyLevel {
    Public,
    Private{
        api_key: String,
        private_key: String,
        nonce: String,
    }
}

impl <F> RequestBuilder<F> where
    F: Serialize,
     {

         pub fn build_basic_request(&self, client: &reqwest::Client) -> ReqwestBuilder{
            client
                .request(self.method.clone(), self.url.clone())
                .header(CONTENT_TYPE, FORM_URLENCODED)
         }

         pub fn attach_data(&self, intermediate: ReqwestBuilder) -> ReqwestBuilder {
            let has_data = self.params.is_some();
            match (has_data, self.param_encoding) {
                (true, ParamEncoding::QueryEncoded) => intermediate.query(&self.params),
                (true, ParamEncoding::FormEncoded) => intermediate.form(&self.params),
                (false, _) => intermediate,
            }
         }

         pub fn attach_auth_headers(&self, mut req: Request) -> Request {
            if let PrivacyLevel::Private{api_key, private_key, nonce} = &self.privacy_level {
                let nonce = nonce.to_string();
                let private_key = private_key.to_string();
                let key = HeaderValue::from_str(&api_key).unwrap();
                req.headers_mut().insert("API-Key", key);
                let signature = get_kraken_signature(nonce, private_key, &req);
                let api_sign = HeaderValue::from_str(&signature).unwrap();
                req.headers_mut().insert("API-Sign", api_sign);
            }
            req
         }

         pub async fn build_and_run(&self, client: &reqwest::Client) -> Result<reqwest::Response, reqwest::Error> {
             let mut intermediate = self.build_basic_request(client);
            intermediate = self.attach_data(intermediate);
            let mut req = intermediate.build()?;
            // If the request required authentication,
            // update the headers with the API key and the signature.
            req = self.attach_auth_headers(req);
            let resp = client
                .execute(req)
                .await?;
            Ok(resp)
         }

        pub async fn execute<R>(self, client: &reqwest::Client) -> Result<R, reqwest::Error> where
    R: for<'de> Deserialize<'de>
        {
            let response = self.build_and_run(client)
            .await?
            .json::<R>()
            .await?;
        Ok(response)
    }

    pub async fn debug(self, client: &reqwest::Client) -> Result<String, reqwest::Error> 
        {
            let response = self.build_and_run(client)
            .await?
            .text()
            .await?;
        Ok(response)
    }
}
