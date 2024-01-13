use chrono::{DateTime, Utc};
use hypertext::{html_elements, maud, GlobalAttributes, Renderable};

use crate::{
    api::{
        error::{ApiError, ApiResult},
        legacy_article::{fetch_legacy_article, parse_legacy_article},
    },
    render::legacy_article_byline::render_byline,
    routes::HtmxAttributes,
};

pub fn render_legacy_article(
    client: &ureq::Agent,
    path: &str,
) -> Result<ApiResult<String>, rouille::Response> {
    let response = match fetch_legacy_article(client, path) {
        Ok(response) => response,
        Err(err) => {
            return Ok(Err(ApiError::from(err)));
        }
    };

    let news = match response.status() {
        200..=299 => parse_legacy_article(response),
        300..=399 => {
            let target = response.header("location").unwrap();
            return Err(rouille::Response {
                status_code: response.status(),
                headers: vec![("Location".into(), target.to_owned().into())],
                data: rouille::ResponseBody::empty(),
                upgrade: None,
            });
        }
        code => {
            return Ok(Err(ApiError::External(
                code,
                response
                    .into_string()
                    .unwrap_or_else(|_| "Unknown error".to_string()),
            )));
        }
    };

    let news = match news {
        Ok(news) => news,
        Err(err) => {
            return Ok(Err(err));
        }
    };

    let article = &news.props.initial_state.article.stream[0];

    let published_time = article
        .date
        .published
        .parse::<DateTime<Utc>>()
        .map(|time| time.format("%Y-%m-%d %H:%M").to_string());

    let doc = crate::document!(
        &article.headline,
        maud! {
            h1 { (&article.headline) }
            p class="byline" {
                @if let Ok(time) = &published_time {
                    (time) " - "
                }
                (render_byline(&article.authors))
            }
            @for content in article.body_items.iter() {
                @match content.r#type.as_str() {
                    "paragraph" => {
                        p {
                            (&content.content)
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
        maud! {
            meta property="og:title" content=(&article.headline);
            meta property="og:type" content="article";
            meta property="og:description" content=(&article.description);
            meta property="og:url" content=(path);
        }
    );

    Ok(Ok(doc.render().into_inner()))
}
