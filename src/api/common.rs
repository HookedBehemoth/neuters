use serde::Deserialize;

#[derive(Deserialize)]
pub struct Articles {
    pub pagination: Pagination,
    pub articles: Option<Box<[Article]>>,
    pub topics: Option<Box<[Topic]>>,
}

#[derive(Deserialize)]
pub struct Pagination {
    pub total_size: Option<u32>,
}

#[derive(Deserialize)]
pub struct Article {
    pub title: String,
    pub subtype: Option<String>,
    pub canonical_url: String,
    pub description: String,
    pub content_elements: Option<Box<[serde_json::Value]>>,
    pub authors: Option<Box<[Topic]>>,
    pub thumbnail: Option<Image>,
    pub published_time: String,
}

#[derive(Deserialize)]
pub struct Image {
    pub caption: Option<String>,
    pub width: u16,
    pub height: u16,
    pub resizer_url: String,
}

#[derive(Deserialize)]
pub struct ApiResponse<T> {
    #[serde(rename = "statusCode")]
    pub status_code: u16,
    pub message: Option<String>,
    pub result: Option<T>,
}

#[derive(Deserialize)]
pub struct Topic {
    pub name: String,
    pub topic_url: Option<String>,
    pub byline: String,
}

#[derive(Clone, Deserialize)]
pub struct Section {
    pub name: String,
    pub id: String,
    pub children: Option<Vec<Section>>,
}
