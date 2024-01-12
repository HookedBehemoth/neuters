use crate::{
    api::{article::fetch_article_by_url, error::ApiResult},
    render::byline,
};
use chrono::{DateTime, Utc};
use hypertext::{html_elements, maud, GlobalAttributes, Rendered, Raw, Attribute};

trait HtmxAttributes: GlobalAttributes {
    #[allow(non_upper_case_globals)]
    const property: Attribute = Attribute;
}

impl<T: GlobalAttributes> HtmxAttributes for T {}

pub fn render_article(client: &ureq::Agent, path: &str) -> ApiResult<String> {
    let article = fetch_article_by_url(client, path)?;

    let published_time = article
        .published_time
        .parse::<DateTime<Utc>>()
        .map(|time| time.format("%Y-%m-%d %H:%M").to_string());

    let doc = crate::document!(
        article.title.as_str(),
        maud!(
            h1 { (article.title.as_str()) }
            p class="byline" {
                @if let Some(authors) = &article.authors {
                    @let byline = byline::render_byline(authors);
                    @if let Ok(time) = &published_time {
                        (time.as_str()) " - "
                    }
                    (Raw(byline))
                }
            }
            @if let Some(articles) = &article.content_elements {
                (Raw(render_items(articles)))
            }
        ),
        maud! {
            meta property="og:title" content=(article.title.as_str());
            meta property="og:type" content="article";
            meta property="og:description" content=(article.description.as_str());
            meta property="og:url" content=(path);
        }
    );

    Ok(doc.render().0)
}

fn render_items(items: &[serde_json::Value]) -> Rendered<String> {
    maud! {
        @for content in items {
            @match content["type"].as_str() {
                Some("header") => {
                    @if let Some(header) = content["content"].as_str() {
                        @match content["level"].as_u64().unwrap_or(0) {
                            0 => h1 { (header) },
                            1 => h2 { (header) },
                            _ => h3 { (header) },
                        }
                    }
                }
                Some("paragraph") => {
                    @if let Some(content) = content["content"].as_str() {
                        p { (Raw(&content)) }
                    }
                }
                Some("image") => {
                    @if let Some(image) = content["url"].as_str() {
                        @let alt = content["alt"].as_str();
                        @let (width, height) = (content["width"].as_u64(), content["height"].as_u64());
                        img src=(image) alt=[alt] width=[width] height=[height];
                    }
                }
                Some("graphic") => {
                    @match content["graphic_type"].as_str() {
                        Some("image") => {
                            @if let (Some(image), Some(description)) = (content["url"].as_str(), content["description"].as_str()) {
                                figure {
                                    img src=(image) alt=(description);
                                    figcaption { (description) }
                                }
                            }
                        }
                        Some(unknown) => { p { "Unknown graphic type: " (unknown) } }
                        None => { p { "Missing graphic type" } }
                    }
                }
                Some("table") => {
                    @let rows = match content["rows"].as_array() { Some(rows) => rows, None => continue };
                    table {
                        thead {
                            @let row = match rows[0].as_array() { Some(row) => row, None => continue };
                            tr {
                                @for cell in row.iter() {
                                    th { (cell.as_str().unwrap_or_default()) }
                                }
                            }
                        }
                        tbody {
                            @for row in rows[1..].iter() {
                                tr {
                                    @let cells = match row.as_array() { Some(cells) => cells, None => continue };
                                    @for cell in cells {
                                        td { (Raw(cell.as_str().unwrap_or_default())) }
                                    }
                                }
                            }
                        }
                    }
                }
                Some("list") => {
                    @if let Some(items) = content["items"].as_array() {
                        (Raw(render_items(items)))
                    }
                }
                Some("social_media") => {
                    @if let Some(markup) = content["html"].as_str() {
                        @let embed = if let Some(index) = markup.find("\n<script") {
                            &markup[..index]
                        } else {
                            markup
                        };
                       (Raw(embed))
                    }
                }
                Some(unknown) => { p { "Unknown type: " (unknown) } }
                None => { p { "Failed to parse content element" } }
            }
        }
    }.render()
}
