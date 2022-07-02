use std::collections::HashMap;

use crate::api::{
    common::ApiResponse,
    error::{ApiError, ApiResult},
};
use serde::Deserialize;

pub(crate) fn fetch<T>(client: &ureq::Agent, url: &str, query: &str) -> ApiResult<T>
where
    T: for<'a> Deserialize<'a>,
{
    fn is_success(status: u16) -> bool {
        (200..300).contains(&status)
    }

    let response = client.get(url).query("query", query).call()?;

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
            response.message.unwrap_or_else(|| "Unknown error".to_string()),
        ))
    } else {
        Ok(response.result.unwrap())
    }
}

#[derive(Deserialize)]
pub struct ApiResponseMultiple<T> {
    #[serde(rename = "statusCode")]
    pub status_code: HashMap<String, u16>,
    pub message: HashMap<String, String>,
    pub result: Option<T>,
}

pub(crate) fn fetch_multiple<T>(client: &ureq::Agent, url: &str, query: &str) -> ApiResult<T>
where
    T: for<'a> Deserialize<'a>,
{
    fn is_success(status: u16) -> bool {
        (200..300).contains(&status)
    }

    let response = client.get(url).query("query", query).call()?;

    if !is_success(response.status()) {
        return Err(ApiError::External(
            response.status(),
            response.into_string().unwrap(),
        ));
    };

    let response = response.into_json::<ApiResponseMultiple<T>>()?;

    let first_error: Vec<(&String, &u16)> = response
        .status_code
        .iter()
        .filter(|(_, &status)| !is_success(status))
        .collect();
    if !first_error.is_empty() || response.result.is_none() {
        let which = first_error[0];
        Err(ApiError::External(
            *which.1,
            format!("Failed to fetch: {}", which.0),
        ))
    } else {
        Ok(response.result.unwrap())
    }
}
