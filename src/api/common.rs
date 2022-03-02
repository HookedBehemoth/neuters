use serde::Deserialize;

#[derive(Deserialize)]
pub struct Articles {
    pub pagination: Pagination,
    // pub topics: Option<Vec<Topic>>,
    pub articles: Vec<Article>,
}

#[derive(Deserialize)]
pub struct Pagination {
    pub size: u32,
    pub total_size: u32,
    pub orderby: String,
}

#[derive(Deserialize)]
pub struct Article {
    pub title: String,
    pub canonical_url: String,
    pub description: String,
    pub content_elements: Option<Vec<serde_json::Value>>,
    pub authors: Vec<Topic>,
    pub published_time: String,
    pub thumbnail: Option<Thumbnail>,
}

#[derive(Deserialize)]
pub struct Section {
    pub path: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct Thumbnail {
    pub url: String,
}

#[derive(Deserialize)]
pub struct ApiResponse<T> {
    #[serde(rename = "statusCode")]
    pub status_code: u16,
    pub message: String,
    pub result: Option<T>,
}

#[derive(Deserialize)]
pub struct Topic {
    pub name: String,
    pub topic_url: String,
    pub byline: String,
}
