use crate::{
    api::{
        common::ApiResponse,
        error::{ApiError, ApiResult},
    },
    client::Client,
};
use serde::Deserialize;

pub(crate) fn fetch<T>(client: &Client, url: &str, query: &str) -> ApiResult<T>
where
    T: for<'a> Deserialize<'a>,
{
    fn is_success(status: u16) -> bool {
        (200..300).contains(&status)
    }

    let response = get(client, url).query("query", query).call()?;

    if (300..400).contains(&response.status()) {
        let target = response.header("Location").unwrap_or("/");

        return Err(ApiError::Redirect(response.status(), target.to_string()));
    }
    if !is_success(response.status()) {
        return Err(ApiError::External(
            response.status(),
            response.into_string().unwrap(),
        ));
    };

    let response = response.into_json::<ApiResponse<T>>()?;

    if !is_success(response.status_code) || response.result.is_none() {
        Err(ApiError::External(
            response.status_code,
            response
                .message
                .unwrap_or_else(|| "Unknown error".to_string()),
        ))
    } else {
        Ok(response.result.unwrap())
    }
}

pub(crate) fn get(client: &Client, url: &str) -> ureq::Request {
    client.get(url)
        .set("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36")
        .set("Accept", "application/json, text/plain, */*")
        .set("Accept-Language", "en-GB,en;q=0.9")
        .set("Referer", "https://www.reuters.com/")
        .set("Origin", "https://www.reuters.com")
}
