use serde::Deserialize;

#[derive(Deserialize)]
pub struct Articles {
    pub pagination: Pagination,
    pub articles: Option<Box<[Article]>>,
    pub topics: Option<Box<[Topic]>>,
    pub section: Option<SectionDescription>,
}

#[derive(Deserialize)]
pub struct Pagination {
    pub size: u32,
    pub total_size: Option<u32>,
    pub orderby: String,
}

#[derive(Deserialize)]
pub struct Article {
    pub title: String,
    pub canonical_url: String,
    pub description: String,
    pub content_elements: Option<Box<[serde_json::Value]>>,
    pub authors: Option<Box<[Topic]>>,
    pub published_time: String,
}

#[derive(Deserialize)]
pub struct Section {
    pub path: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct ApiResponse<T> {
    #[serde(rename = "statusCode")]
    pub status_code: u16,
    pub message: Option<String>,
    pub result: Option<T>,
}

#[derive(Deserialize)]
pub struct SectionDescription {
    pub name: String,
}

#[derive(Deserialize)]
pub struct Topic {
    pub name: String,
    pub topic_url: Option<String>,
    pub byline: String,
}
