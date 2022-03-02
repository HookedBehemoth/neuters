use crate::api::{common::Article, error::ApiError, fetch::fetch};
use serde::Serialize;

pub fn fetch_article(path: &str) -> Result<Article, ApiError> {
    #[derive(Serialize)]
    struct ArticleQuery<'a> {
        website_url: &'a str,
        website: &'static str,
    }
    let query = ArticleQuery {
        website_url: path,
        website: "reuters",
    };

    let query_json = serde_json::to_string(&query).unwrap();

    const API_URL: &str = "https://www.reuters.com/pf/api/v3/content/fetch/article-by-id-or-url-v1";
    fetch(API_URL, &query_json)
}
