use chrono::{DateTime, Utc};
use maud::{html, PreEscaped};

use crate::{
    api::{error::ApiResult, internet_news::fetch_internet_news},
    render::internet_news_byline::render_byline,
};

pub fn render_internet_news(client: &ureq::Agent, path: &str) -> ApiResult<String> {
    let news = fetch_internet_news(client, path)?;

    let article = &news.props.initial_state.article.stream[0];

    let published_time = article
        .date
        .published
        .parse::<DateTime<Utc>>()
        .map(|time| time.format("%Y-%m-%d %H:%M").to_string());

    let doc = crate::document!(
        &article.headline,
        html! {
            h1 { (&article.headline) }
            p class="byline" {
                @let byline = render_byline(&article.authors);
                @if let Ok(time) = published_time {
                    (time) " - "
                }
                (PreEscaped(byline))
            }
            @for content in &article.body_items {
                @match content.r#type.as_str() {
                    "paragraph" => {
                        p {
                            (content.content)
                        }
                    }
                    t => {
                        p {
                            "Unknown type: " (t)
                        }
                    }
                }
            }
        },
        html! {
            meta property="og:title" content=(&article.headline);
            meta property="og:type" content="article";
            meta property="og:description" content=(&article.description);
            meta property="og:url" content=(path);
        }
    );

    Ok(doc.into_string())
}
