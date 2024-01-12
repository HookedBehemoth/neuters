use crate::api::{error::ApiResult, markit::fetch_by_stock_symbol};
use hypertext::{html_elements, maud, GlobalAttributes};

pub fn render_market(client: &ureq::Agent, path: &str) -> ApiResult<String> {
    let company = if let Some(end) = path.find('/') {
        &path[..end]
    } else {
        path
    };

    let articles = fetch_by_stock_symbol(client, company)?;

    let document = crate::document(
        company,
        maud! {
            (company)
            ul {
                @for article in articles.articles.iter() {
                    li { a href=(article.canonical_url.as_str()) { (article.title.as_str()) } }
                }
            }
        }.render().as_str(),
        None
    );
    Ok(document.into_string())
}
