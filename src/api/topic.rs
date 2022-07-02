use super::{common::Articles, error::ApiResult, fetch::fetch};

const API_URL: &str = "https://www.reuters.com/pf/api/v3/content/fetch/articles-by-topic-v1";

pub fn fetch_articles_by_topic(
    client: &ureq::Agent,
    path: &str,
    offset: u32,
    size: u32,
) -> ApiResult<Articles> {
    let query =
        format!(r#"{{"offset":{offset},"size":{size},"topic_url":"{path}","website":"reuters"}}"#);

    fetch(client, API_URL, &query)
}
