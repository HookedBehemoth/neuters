use reqwest::Client;

use super::{common::Articles, error::ApiResult, fetch::fetch};

const API_URL: &str =
    "https://www.reuters.com/pf/api/v3/content/fetch/articles-by-section-alias-or-id-v1";

pub async fn fetch_articles_by_section(
    client: &Client,
    path: &str,
    offset: u32,
    size: u32,
) -> ApiResult<Articles> {
    let query =
        format!(r#"{{"offset":{offset},"size":{size},"section_id":"{path}","website":"reuters"}}"#);

    fetch(client, API_URL, &query).await
}
