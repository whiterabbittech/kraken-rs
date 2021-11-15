use serde::ser::{Serialize};
use serde::de::Deserialize;
use reqwest::{Url, Method, Request};
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use crate::kraken::signature::get_kraken_signature;

const FORM_URLENCODED: &'static str = "application/x-www-form-urlencoded; charset=utf-8";

pub struct RequestBuilder<F> where
    F: Serialize,
{
    pub method: Method,
    // Must be provided.
    pub url: Url,
    // Not always required.
    pub form_params: Option<F>,
    pub privacy_level: PrivacyLevel,
}

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
    
        pub async fn execute<R>(self, client: &reqwest::Client) -> Result<R, reqwest::Error> where
    R: for<'de> Deserialize<'de>
        {

            // Build the requerst.
            let mut req = client
                .request(self.method, self.url,)
                .form(&self.form_params)
                .header(CONTENT_TYPE, FORM_URLENCODED)
                .build()?;
            // If the request required authentication,
            // update the headers with the API key and the signature.
            if let PrivacyLevel::Private{api_key, private_key, nonce} = self.privacy_level {
                let key = HeaderValue::from_str(&api_key).unwrap();
                req.headers_mut().insert("API-Key", key);
                let signature = get_kraken_signature(nonce, private_key, &req);
                let api_sign = HeaderValue::from_str(&signature).unwrap();
                req.headers_mut().insert("API-Sign", api_sign);
            }
            let resp = client
                .execute(req)
                .await?
                .json::<R>()
                .await?;
            Ok(resp)
        }
}
