use reqwest::Client;

use crate::api::{common::Article, error::ApiResult, fetch::fetch};

const API_URL: &str = "https://www.reuters.com/pf/api/v3/content/fetch/article-by-id-or-url-v1";

pub async fn fetch_article_by_url(client: &Client, path: &str) -> ApiResult<Article> {
    let query = format!(r#"{{"website_url":"{path}","website":"reuters"}}"#);

    fetch(client, API_URL, &query).await
}
