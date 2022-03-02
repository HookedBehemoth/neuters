use crate::api::{common::ApiResponse, error::ApiError};
use reqwest::blocking::Client;
use serde::Deserialize;

pub(crate) fn fetch<T>(url: &str, query: &str) -> Result<T, ApiError>
where
    T: for<'a> Deserialize<'a>,
{
    let query = [("query", query), ("d", "75"), ("_website", "reuters")];

    let client = Client::new();

    let response = match client.get(url).query(&query).send() {
        Ok(response) => {
            if response.status() != 200 {
                return Err(ApiError::External(
                    response.status().as_u16(),
                    response.text().unwrap(),
                ));
            }
            response
        }
        Err(err) => {
            return Err(ApiError::InternalServerError(err.to_string()));
        }
    };

    match response.json::<ApiResponse<T>>() {
        Ok(response) => {
            if response.status_code != 200 || response.result.is_none() {
                Err(ApiError::External(response.status_code, response.message))
            } else {
                Ok(response.result.unwrap())
            }
        }
        Err(err) => Err(ApiError::InternalServerError(err.to_string())),
    }
}
