use ring::digest::{Context, Digest, SHA256};
use ring::{hmac};
use data_encoding::BASE64;

pub struct SignatureInput {
    pub private_key: String,
    pub nonce: String,
    pub encoded_payload: String,
    pub uri_path: String,
}

impl SignatureInput {
    // Kraken's doc provide a formula for the API signature at the following URL:
    // https://docs.kraken.com/rest/#section/Authentication/Headers-and-Signature
    // This yields the following algorithm:
    // • Concat the nonce and POST data.
    // • Take the SHA of the concatenated value.
    // • Base64 decode the private key: call this the "secret".
    // • Build an HMAC (SHA512) using the secret.
    // • Concat the URI path with the SHA
    // • HMAC that concated value.
    // • Base64 encode the HMAC signature.
    pub fn sign(self) -> String {
        // • Collect the SHA.
        let digest = Self::take_sha(self.nonce, self.encoded_payload);
        // • Create the signing key.
        let key = Self::build_hmac_key(self.private_key);
        // • Sign the payload.
        let signature = Self::generate_hmac(key, digest.as_ref(), self.uri_path);
        signature
    }

    fn take_sha(nonce: String, encoded_payload: String) -> Digest {
         // • Create a new Context for taking the SHA.
        let mut context = Context::new(&SHA256);
        // • Concat the nonce and POST data.
        let concat = nonce + &encoded_payload;
        // • Take the SHA of the concatenated value.
        context.update(concat.as_bytes());
        context.finish()
    }

    fn build_hmac_key(private_key: String) -> hmac::Key {
        // let mut key_value = [0u8; 48];
        // let rng = rand::SystemRandom::new();
        // • Base64 decode the private key.
        let secret_str = BASE64.decode(private_key.as_bytes()).unwrap();
        // rng.fill(&mut secret_str)?;
        // let secret = FixedBytes::new(secret_str);
        // • Build an HMAC (SHA512) using the secret key.
        hmac::Key::new(hmac::HMAC_SHA512, &secret_str)
    }

    fn generate_hmac(key: hmac::Key, digest: &[u8], uri_path: String) -> String {
        // • Concat the URI path with the SHA
        let uri_bytes = uri_path.as_bytes();
        let hmac_input = &[uri_bytes, digest].concat();
        // • HMAC that concated value.
        let tag = hmac::sign(&key, &hmac_input);
        let tag_bytes = tag.as_ref();
        BASE64.encode(tag_bytes)
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;
    use super::SignatureInput;

    #[test]
    fn test_signature() {
        // Using the provided example from the Kraken API docs
        // https://docs.kraken.com/rest/#section/Authentication/API-Sign
        // we demonstrate that our signature is calculated correctly.
        let sig = SignatureInput {
            private_key: "kQH5HW/8p1uGOVjbgWA7FunAmGO8lsSUXNsu3eow76sz84Q18fWxnyRzBHCd3pd5nE9qa99HAZtuZuj6F1huXg==".to_owned(),
            nonce: "1616492376594".to_owned(),
            encoded_payload: "nonce=1616492376594&ordertype=limit&pair=XBTUSD&price=37500&type=buy&volume=1.25".to_owned(),
            uri_path: "/0/private/AddOrder".to_owned(),
        };
        let expected = "4/dpxb3iT4tp/ZCVEwSnEsLxx0bqyhLpdfOpc6fn7OR8+UClSV5n9E6aSS8MPtnRfp32bAb0nmbRn6H8ndwLUQ==".to_owned();
        let observed = sig.sign();
        assert_eq!(expected, observed);
    }
}
