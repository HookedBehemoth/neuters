use crate::api::{common::Article, error::ApiError, fetch::fetch};

const API_URL: &str = "https://www.reuters.com/pf/api/v3/content/fetch/article-by-id-or-url-v1";

pub fn fetch_article(path: &str) -> Result<Article, ApiError> {
    let query = format!(r#"{{"website_url":"{}","website":"reuters"}}"#, path);

    fetch(API_URL, &query)
}
