use std::{sync::Mutex, time::SystemTime};

use crate::{
    api::{
        error::ApiResult,
        markit::{fetch_graph, fetch_ids, fetch_market_token, quote, related_articles, ModToken},
    },
    render::graph::render_graph_svg,
};

pub fn render_market(client: &ureq::Agent, path: &str, markit_token: &Mutex<ModToken>) -> ApiResult<String> {
    let company = if let Some(end) = path.find('/') {
        &path[..end]
    } else {
        path
    };

    let mut token = markit_token.lock().unwrap();
    if SystemTime::now().duration_since(token.start).unwrap().as_secs() > token.expires_in {
        *token = fetch_market_token(client)?;
    }

    let ids = fetch_ids(client, &token, &[company])?;
    let graph = fetch_graph(client, &token, &ids)?;
    let quote = &quote(client, &[company])?.market_data[0];
    let articles = related_articles(client, company)?;

    let document = crate::document! {
        (graph.Elements[0].CompanyName),
        maud::html! {
            h1 { (quote.name) } (company)
            p {
                "Last Trade: " (quote.last) " " (quote.currency) " "
                @let sign = if quote.percent_change.is_sign_positive() {
                    ('+', "color:green")
                } else {
                    ('-', "color:red")
                };
                span style=(sign.1) { (sign.0)(format!("{:.2}", quote.percent_change)) }
            }
            p { "Day Range: " (quote.day_low) " - " (quote.day_high) }
            p { "52 Week Range: " (quote.fiftytwo_wk_low) " - " (quote.fiftytwo_wk_high) }
            (maud::PreEscaped(render_graph_svg(&graph)))

            ul {
                @for article in articles.articles {
                    li { a href=(&article.canonical_url) { (&article.title) } }
                }
            }
        },

    };
    Ok(document.into_string())
}
