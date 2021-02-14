/// This HTTP client wrapper exists so that if hyper's http client becomes
/// unsupported, putting in a different client only required modifying this one
/// module.
///
use super::api::{v3, AccessToken, RefreshToken};
use http::StatusCode;
use serde::de::DeserializeOwned;
use std::result::Result;

use super::error::ApiError;

pub async fn get<T>(url: &str) -> Result<T, ApiError>
where
    T: DeserializeOwned,
{
    let response = reqwest::get(url).await?;
    match response.status() {
        StatusCode::UNAUTHORIZED => Err(ApiError::InvalidAccessToken),
        _ => Ok(response.json().await?),
    }
}

pub async fn post<T>(url: &str, form : reqwest::multipart::Form) -> Result<T, ApiError>
where
    T: DeserializeOwned,
{
    let response = reqwest::Client::builder().build()?.post(url).multipart(form).send().await?;
    println!("uploaded status={}", response.status());
    match response.status() {
        StatusCode::UNAUTHORIZED => Err(ApiError::InvalidAccessToken),
        StatusCode::BAD_REQUEST => Err(ApiError::BadRequest(response.text().await?)),
        _ => Ok(response.json().await?),
    }
}

pub async fn refresh_tokens(refresh_token: &RefreshToken) -> Result<AccessToken, ApiError> {
    let url = v3(None, "oauth/token".to_string());
    let params = [
        ("client_id", refresh_token.client_id.clone()),
        ("client_secret", refresh_token.client_secret.clone()),
        ("grant_type", "refresh_token".to_string()),
        ("refresh_token", refresh_token.refresh_token.clone()),
    ];
    let client = reqwest::Client::new();

    let response = client.post(&url[..]).form(&params).send().await?;

    match response.status() {
        StatusCode::UNAUTHORIZED => Err(ApiError::InvalidAccessToken),
        StatusCode::BAD_REQUEST => Err(ApiError::BadRequest(response.text().await?)),
        _ => Ok(response.json().await?),
    }
}

#[cfg(test)]
mod tests {
    use crate::athletes::Athlete;
    use crate::error::ApiError;
    #[tokio::test]
    async fn request_wrapper_can_fetch() {
        match super::get::<Athlete>("http://www.example.com").await {
            Err(ApiError::Http(_)) => assert!(true),
            _ => assert!(false, "Expected Json deserialization to fail."),
        }
    }
}
