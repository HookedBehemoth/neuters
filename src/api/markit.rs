use super::{
    common::{Article, Pagination},
    error::ApiResult,
    fetch::fetch,
};

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockSearchResult {
    pub pagination: Pagination,
    pub market_info: MarketInfo,
    pub articles: Box<[Article]>,
}

#[derive(serde::Deserialize)]
pub struct MarketInfo {
    pub website: String,
    pub about: String,
}

pub fn fetch_by_stock_symbol(client: &ureq::Agent, symbol: &str) -> ApiResult<StockSearchResult> {
    const API_URL: &str =
        "https://www.reuters.com/pf/api/v3/content/fetch/articles-by-stock-symbol-v1";

    let query = format!(r#"{{"website":"reuters","symbol":"{symbol}","arc-site":"reuters"}}"#);

    fetch(client, API_URL, &query)
}
