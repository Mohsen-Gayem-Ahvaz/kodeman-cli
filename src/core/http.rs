use crate::core::{AppResult, BASE_ST};
use crate::user::utils;
use std::collections::BTreeMap;
use base64::engine as b64engine;
use base64::Engine;
use reqwest::blocking::{Client, Response};
use rsa::{Pkcs1v15Sign, RsaPrivateKey};
use rsa::pkcs8::DecodePrivateKey;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};


// Models & Forms
#[derive(Debug, Deserialize, Serialize)]
struct KeyPairCreds {
    username: String,
    dev: String,
    sign: String,
}


pub fn post_request<F: Serialize>(form: Option<&F>, url: &str)
-> AppResult<Response> {

    let http_client = Client::new();

    if form.is_some() {
        Ok(http_client.post(url).form(&form).send()?)
    } else {
        Ok(http_client.post(url).send()?)
    }

}


pub fn get_request<F: Serialize>(form: Option<&F>, url: &str)
-> AppResult<Response> {

    let http_client = Client::new();

    if form.is_some() {
        Ok(http_client.get(url).form(&form).send()?)
    } else {
        Ok(http_client.get(url).send()?)
    }

}


pub fn post_request_with_auth<F: Serialize>(f: Option<F>, url: &str)
-> AppResult<Response> {

    let base_st = BASE_ST.lock().unwrap().clone().unwrap();
    let user_st = utils::get_default_user().map_err(|e| {
        e.operation("finding user")
            .cause("working with pool-service, requires login")
            .hint("run 'user login' first")
    })?;

    let _payload = BTreeMap::from([
        ("username", user_st.username.as_str()),
        ("email", user_st.email.as_str()),
        ("client_id", base_st.client_id.as_str()),
        ("dev", &user_st.dev),
    ]);
    let payload_json_string = serde_json::to_string(&_payload)?;
    let payload_json_vec: &[u8] = payload_json_string.as_bytes();

    let pri_key = RsaPrivateKey::from_pkcs8_pem(&user_st.prikey).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(payload_json_vec);
    let hashed = hasher.finalize();
    let padding = Pkcs1v15Sign::new::<Sha256>();
    let sign = pri_key.sign(padding, &hashed)?;
    let sign_b64 = b64engine::general_purpose::STANDARD.encode(sign);

    let creds = KeyPairCreds {
        username: user_st.username.to_owned(),
        dev: user_st.dev.to_owned(),
        sign: sign_b64,
    };

    let http_client = Client::new();

    if let Some(f) = f {
        Ok(
            http_client
                .post(url)
                .json(&f)
                .header(
                    "Authorization", serde_json::to_string(&creds).unwrap()
                ).send()?
        )
    } else {
        Ok(
            http_client
                .post(url)
                .header(
                    "Authorization", serde_json::to_string(&creds).unwrap()
                ).send()?
        )
    }

}

