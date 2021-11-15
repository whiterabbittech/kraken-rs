use serde::ser::Serialize;
use reqwest::{Url, Method, Request};
use reqwest::header::{HeaderValue, CONTENT_TYPE};

const FORM_URLENCODED: &'static str = "application/x-www-form-urlencoded; charset=utf-8";

pub struct RequestBuilder<F, R> where
    F: Serialize + ?Sized,
    R: T: Serialize + ?Sized,
{
    pub method: Method,
    // Must be provided.
    pub url: Url,
    // Not always required.
    pub form_param: Option<F>,
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

impl <F, R> RequestBuilder<F, R> where
    F: Serialize + ?Sized,
    R: T: Serialize + ?Sized {
    
        pub fn execute(&self, client: &reqwest::Client) -> Result<Request, reqwest::Error> {
            // Build the requerst.
            let mut req = self
                client
                    .request(self.method, self.url,)
                    .form(&self.form_params)
                    .header(CONTENT_TYPE, FORM_URLENCODED)
                    .build()?;
            // If the request required authentication,
            // update the headers with the API key and the signature.
            if let PrivacyLevel::Private{api_key, private_key, nonce} = self.private_level {
                req.headers_mut().insert("API-Key", api_key);
                let signature = get_kraken_signature(nonce, &req);
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
