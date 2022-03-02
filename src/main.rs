mod api;

use api::{
    common::Articles, error::ApiError, section::fetch_articles_by_section,
    topic::fetch_articles_by_topic,
};
use cached::proc_macro::{cached, once};
use chrono::{DateTime, Utc};
use typed_html::{dom::DOMTree, html, text, unsafe_text};

use crate::api::{article::fetch_article, byline};

const CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/main.css"));

macro_rules! document {
    ($title:expr, $content:expr, $( $head:expr ),*) => {
        html!(
            <html lang="en">
                <head>
                    <title>{ text!($title) }</title>
                    <style>{ text!(CSS) }</style>
                    $(
                        { $head }
                    )*
                </head>
                <body>
                    { $content }
                    <footer>
                        <div>
                            <a href="/">"Home"</a>
                            " - "
                            <a href="/about">"About"</a>
                        </div>
                    </footer>
                </body>
            </html>
        )
    };
}

fn main() {
    rouille::start_server("0.0.0.0:13369", move |request| {
        let response = render_page(request.url());

        match response.body {
            Body::Html(body) => rouille::Response::html(body),
            Body::Data(content_type, data) => rouille::Response::from_data(content_type, data),
        }
        .with_status_code(response.code)
        .with_public_cache(response.cache_time)
    });
}

// Wrappers that implement Clone so we can use them in the cache
#[derive(Clone)]
struct Response {
    code: u16,
    body: Body,
    cache_time: u64,
}

#[derive(Clone)]
enum Body {
    Html(String),
    Data(&'static str, Vec<u8>),
}

fn render_page(path: String) -> Response {
    match path.as_str() {
        "/favicon.ico" => Response {
            code: 404,
            body: Body::Data("image/x-icon", vec![]),
            cache_time: 0,
        },
        "/" | "/home" => render_section("/home".to_string(), 8),
        "/about" => render_about(),
        _ => {
            if path.starts_with("/authors/") {
                render_topic(path, 0, 20)
            } else if path.starts_with("/article/") {
                render_error(400, "Please disable forwards to this page.", &path)
            } else {
                render_article(path)
            }
        }
    }
}

#[cached(time = 86400)]
fn render_article(path: String) -> Response {
    let article = match fetch_article(&path) {
        Ok(article) => article,
        Err(err) => {
            return render_api_error(&err, &path);
        }
    };

    let published_time = article
        .published_time
        .parse::<DateTime<Utc>>()
        .unwrap_or_else(|_| Utc::now());

    let doc: DOMTree<String> = document!(
        &article.title,
        html!(
            <main>
                <h1>{ text!(&article.title) }</h1>
                <p class="byline">
                {
                    let time = published_time.format("%Y-%m-%d %H:%M").to_string();
                    let byline = byline::render_byline(&article.authors);
                    unsafe_text!("{} - {}", time, byline)
                }
                </p>
                {
                    article.content_elements.unwrap().iter().map(|content| {
                        match content["type"].as_str() {
                            Some("paragraph") => {
                                let content = content["content"].as_str().unwrap_or_default();
                                html!(
                                    <div>
                                        <p>{ unsafe_text!(content) }</p>
                                    </div>
                                )
                            },
                            Some("image") => {
                                let image = content["url"].as_str().unwrap_or_default();
                                html!(
                                    <div>
                                        <img src=image />
                                    </div>
                                )
                            },
                            Some("table") => {
                                let empty_row = vec![];
                                let rows = content["rows"].as_array().unwrap_or(&empty_row);
                                let mut iter = rows.iter();

                                html!(
                                    <div>
                                        <table>
                                            <thead>
                                            {
                                                let empty_value = serde_json::Value::Null;
                                                let empty_row = vec![];
                                                let row = iter.next().unwrap_or(&empty_value).as_array().unwrap_or(&empty_row);
                                                html!(
                                                    <tr>
                                                    {
                                                        row.iter().map(|cell| {
                                                            let cell = cell.as_str().unwrap_or_default();
                                                            html!(<td> { text!(cell) } </td>)
                                                        })
                                                    }
                                                    </tr>
                                                )
                                            }
                                            </thead>
                                            <tbody>
                                            {
                                                iter.map(|row| {
                                                    let empty_cell = vec![];
                                                    let cells = row.as_array().unwrap_or(&empty_cell);
                                                    html!(<tr>
                                                    {
                                                        cells.iter().map(|cell| {
                                                            let cell = cell.as_str().unwrap_or_default();
                                                            html!(<td> { text!(cell) } </td>)
                                                        })
                                                    }
                                                    </tr>)
                                                })
                                            }
                                            </tbody>
                                        </table>
                                    </div>
                                )
                            }
                            Some(unk) => {
                                html!(<div>{ text!("Unknown content type: {}", unk) }</div>)
                            },
                            _ => html!(<div>"Failed to parse!"</div>),
                        }
                    })
                }
            </main>
        ),
        html!(<meta property="og:title" content=&article.title />),
        html!(<meta property="og:type" content="article" />),
        html!(<meta property="og:description" content=&article.description />),
        html!(<meta property="og:url" content=path />)
    );

    let doc_string = doc.to_string();

    Response {
        code: 200,
        body: Body::Html(doc_string),
        cache_time: 24 * 60 * 60,
    }
}

#[cached(time = 3600)]
fn render_topic(path: String, offset: u32, size: u32) -> Response {
    render_articles(&path, fetch_articles_by_topic(&path, offset, size))
}

#[cached(time = 3600)]
fn render_section(path: String, size: u32) -> Response {
    render_articles(&path, fetch_articles_by_section(&path, size))
}

fn render_articles(path: &str, response: Result<Articles, ApiError>) -> Response {
    let articles = match response {
        Ok(articles) => articles,
        Err(err) => {
            return render_api_error(&err, path);
        }
    };

    // let title = articles
    //     .topics
    //     .first()
    //     .map(|topic| topic.name.as_str())
    //     .unwrap_or_default();
    let title = "Placeholder";

    let doc: DOMTree<String> = document!(
        title,
        html!(
            <main>
                <h1>{ text!(title) }</h1>
                <ul>
                {
                    articles.articles.iter().map(|article| {
                        html!(
                            <li>
                                <a href=&article.canonical_url>{ text!(&article.title) }</a>
                            </li>
                        )
                    })
                }
                </ul>
            </main>
        ),
    );

    Response {
        code: 200,
        body: Body::Html(doc.to_string()),
        cache_time: 60 * 60,
    }
}

#[once]
fn render_about() -> Response {
    let doc: DOMTree<String> = document!(
        "About",
        html!(
            <main>
                <h1>"About"</h1>
                <p>
                    "This is an alternative frontent to " <a href="https://www.reuters.com/">"Reuters"</a> ". "
                    "It is intented to be lightweight and fast and was heavily inspired by " <a href="https://nitter.net/">"Nitter"</a> "."
                </p>
                <ul>
                    <li>"No JavaScript or ads"</li>
                    <li>"No tracking"</li>
                    <li>"No cookies"</li>
                    <li>"Lightweight (usually <10KiB vs 50MiB from Reuters)"</li>
                    <li>"Dynamic Theming (respects system theme)"</li>
                </ul>
                <p>
                    "This is a work in progress. Please report any bugs or suggestions at " <a href="https://github.com/HookedBehemoth/supreme-waffle">"GitHub"</a> "."
                </p>
                <h2>"Contact"</h2>
                <p>
                    "If you have any questions, feel free to contact me at " <a href="mailto:admin@boxcat.site">"admin@boxcat.site"</a>"."
                </p>
                <h2>"Credits"</h2>
                <ul>
                    <li><a href="https://github.com/bodil/typed-html">"typed-html"</a>", a fast and intuitive inline html macro"</li>
                    <li><a href="https://github.com/jaemk/cached">"cached"</a>", a macro for caching responses"</li>
                </ul>
                <h2>"License"</h2>
                <p>
                    "This project is licensed under the " <a href="https://www.gnu.org/licenses/licenses.html#AGPL">"GNU Affero General Public License"</a>"."
                </p>
            </main>
        ),
    );

    Response {
        code: 200,
        body: Body::Html(doc.to_string()),
        cache_time: 24 * 60 * 60,
    }
}

fn render_error(code: u16, message: &str, path: &str) -> Response {
    let title = format!("{} 🞄 {}", code, message);

    let doc: DOMTree<String> = document!(
        &title,
        html!(
            <main>
                <h1>{ text!(&title) }</h1>
                <p>{ text!("You tried to access \"{}\"", path) }</p>
                <p><a href="/">"Go home"</a></p>
                <p><a href=path>"Try again"</a></p>
            </main>
        ),
    );

    Response {
        code,
        body: Body::Html(doc.to_string()),
        cache_time: 0,
    }
}

fn render_api_error(err: &ApiError, path: &str) -> Response {
    match &err {
        ApiError::External(code, message) => render_error(*code, message, path),
        ApiError::InternalServerError(message) => render_error(500, message, path),
    }
}
