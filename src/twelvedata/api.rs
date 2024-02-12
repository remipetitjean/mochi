use serde::Deserialize;
use thiserror::Error;

const API_URL: &str = "https://api.twelvedata.com";
const API_KEY: &str = "16ebf3860688468b9cdab89899669b30";

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("url not found")]
    NotFound,

    #[error("resource is forbidden")]
    Forbidden,

    #[error("unhandled error: {text:}")]
    Unhandled { text: String },

    #[error("reqwest error {code:?}: {text:}")]
    Reqwest { code: u16, text: String },
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct ErrorModel {
    code: u16,
    message: String,
}

pub async fn json_from_endpoint<T>(endpoint: &str) -> Result<T, ApiError>
where
    T: for<'a> Deserialize<'a>
{
    // add api key
    let url = format!("{}/{}&apikey={}", API_URL, endpoint, API_KEY);

    // request data
    let response = reqwest::get(&url).await.unwrap();
    let code = response.status();
    let text = response.text().await.unwrap();
    println!("GET {} -> {}", url, code);

    if !code.is_success() {
        return Err(ApiError::Reqwest {
            code: code.into(),
            text: text.to_string(),
        });
    }

    // try deserializing into T
    let data: Result<T, serde_json::error::Error> = serde_json::from_str(&text);
    match data {
        Ok(res) => return Ok(res),
        Err(_) => (),
    };

    // try deserializing into ApiError
    let err: Result<ErrorModel, serde_json::error::Error> = serde_json::from_str(&text);
    match err {
        Ok(_) => return Err(ApiError::Forbidden),
        Err(err) => {
            return Err(ApiError::Unhandled {
                text: err.to_string(),
            })
        }
    };
}
