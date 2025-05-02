use crate::client::Client;

use super::{error::{ApiError, ApiResult}, fetch::get};


pub(crate) fn load_redirect(client: &Client, url: &str) -> ApiResult<(u16, String)> {
    let response = get(client, url).call()?;
    if !(300..400).contains(&response.status()) {
        Err(ApiError::External(
            response.status(),
            response.into_string().unwrap(),
        ))
    } else {
        let target = response.header("Location").unwrap_or("/");

        Ok((response.status(), target.to_string()))
    }
}
