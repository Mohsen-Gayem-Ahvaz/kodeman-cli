use reqwest::blocking::{Client, Response};
use serde::{Deserialize, Serialize};
use crate::core::AppResult;

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