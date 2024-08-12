use crate::api::{
    common::ApiResponse,
    error::{ApiError, ApiResult},
};
use log::debug;
use reqwest::Client;
use serde::Deserialize;

pub(crate) async fn fetch<T>(client: &Client, url: &str, query: &str) -> ApiResult<T>
where
    T: for<'a> Deserialize<'a>,
{
    debug!("Fetching content from {url} with query {query}");

    let response = client
        .get(url)
        .query(&[("query", &query)])
        .send()
        .await?
        .error_for_status()?;

    let response: ApiResponse<T> = response.json().await?;

    if !(200..300).contains(&response.status_code) || response.result.is_none() {
        debug!("Got non-OK result {:?} with message {:?}", response.status_code, response.message);
        Err(ApiError::External(
            response.status_code,
            format!("{url} query: {query}"),
            response
                .message
                .unwrap_or_else(|| "Unknown error".to_string()),
        ))
    } else {
        Ok(response.result.unwrap())
    }
}
