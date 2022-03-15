use super::{common::Articles, error::ApiError, fetch::fetch};

const API_URL: &str = "https://www.reuters.com/pf/api/v3/content/fetch/articles-by-topic-v1";

pub fn fetch_articles_by_topic(path: &str, offset: u32, size: u32) -> Result<Articles, ApiError> {
    let query = format!(
        r#"{{"offset":{},"size":{},"topic_url":"{}","website":"reuters"}}"#,
        offset, size, path
    );

    fetch(API_URL, &query)
}
