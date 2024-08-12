use axum::extract::{Path, State};
use maud::Markup;
use reqwest::Client;

use crate::api::{error::ApiResult, markit::fetch_by_stock_symbol};

pub async fn render_market(client: State<Client>, Path(company): Path<String>) -> ApiResult<Markup> {
    let articles = fetch_by_stock_symbol(&client, &company).await?;

    let document = crate::document! {
        company,
        maud::html! {
            company
            ul {
                @for article in articles.articles.iter() {
                    li { a href=(&article.canonical_url) { (&article.title) } }
                }
            }
        },

    };
    Ok(document)
}
