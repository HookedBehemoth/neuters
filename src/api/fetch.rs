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
            response
                .message
                .unwrap_or_else(|| "Unknown error".to_string()),
        ))
    } else {
        Ok(response.result.unwrap())
    }
}
