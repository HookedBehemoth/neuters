use serde::Serialize;

use super::{common::Articles, error::ApiError, fetch::fetch};

pub fn fetch_articles_by_topic(path: &str, offset: u32, size: u32) -> Result<Articles, ApiError> {
    #[derive(Serialize)]
    struct TopicQuery<'a> {
        offset: u32,
        size: u32,
        topic_url: &'a str,
        website: &'static str,
    }
    let query = TopicQuery {
        offset,
        size,
        topic_url: path,
        website: "reuters",
    };

    let query_json = serde_json::to_string(&query).unwrap();

    const API_URL: &str = "https://www.reuters.com/pf/api/v3/content/fetch/articles-by-topic-v1";
    fetch(API_URL, &query_json)
}
