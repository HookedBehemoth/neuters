use crate::client::Client;

use super::{common::Article, error::ApiResult, fetch::fetch};

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockSearchResult {
    pub articles: Box<[Article]>,
}

pub fn fetch_by_stock_symbol(client: &Client, symbol: &str) -> ApiResult<StockSearchResult> {
    const API_URL: &str =
        "https://www.reuters.com/pf/api/v3/content/fetch/articles-by-stock-symbol-v1";

    let query = format!(r#"{{"website":"reuters","symbol":"{symbol}","arc-site":"reuters"}}"#);

    fetch(client, API_URL, &query)
}
