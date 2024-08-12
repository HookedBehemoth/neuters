mod api;
mod render;
mod routes;

use api::error::ApiError;
use axum::{routing::get, Router};
use reqwest::redirect::Policy;
use routes::{
    about::about,
    article::render_article,
    // internet_news::render_legacy_article,
    markets::render_market,
    article_list::{search, section, topic, home, author},
    statics::css,
};

macro_rules! document {
    ($title:expr, $content:expr, $( $head:expr )? ) => {
        maud::html! {
            (maud::DOCTYPE)
            html lang="en" {
                head {
                    title { ($title) }
                    link rel="stylesheet" href="/main.css?v=0";
                    meta name="viewport" content="width=device-width, initial-scale=1";
                    $( ($head) )?
                }
                body {
                    main { ($content) }
                    footer { div {
                        a href="/" { "Home" }
                        " - "
                        a href="/search" { "Search" }
                        " - "
                        a href="/about" { "About" } } }
                }
            }
        }
    };
}
pub(crate) use document;

use crate::routes::statics::empty_not_found;

#[tokio::main]
async fn main() {
    env_logger::init();
    // tracing_subscriber::fmt::init();

    let mut pargs = pico_args::Arguments::from_env();
    let list_address: String = pargs
        .value_from_str("--address")
        .unwrap_or_else(|_| "127.0.0.1:13369".into());

    let client = reqwest::ClientBuilder::new()
        .redirect(Policy::none())
        .tls_built_in_root_certs(true)
        .build()
        .expect("HTTP-Client");

    println!("Listening on http://{}", list_address);
    
    let routes = Router::new()
        .route("/", get(home))
        .route("/home", get(home))
        .route("/search", get(search))
        .route("/authors/:author/", get(author))
        // .route("/article/:name", get(render_legacy_article))
        .route("/about", get(about))
        .route("/main.css", get(css))
        .route("/favicon.ico", get(empty_not_found))
        .route("/markets/companies/:id", get(render_market))
        .fallback(get(render_article))
        .with_state(client);

    let listener = tokio::net::TcpListener::bind(list_address).await.unwrap();
    axum::serve(listener, routes).await.unwrap();
    /*
    rouille::start_server(list_address, move |request| {
        let path = request.url();
        let response = match path.as_str() {
            "/" | "/home" => section(&client, "/home", 0, 8),
            "/about" => get(about),
            "/search" | "/search/" => search(&client, request),
            "/main.css" => {
                return rouille::Response {
                    status_code: 200,
                    headers: vec![
                        ("Content-Type".into(), "text/css".into()),
                        ("Cache-Control".into(), "public, max-age=31536000".into()),
                    ],
                    data: rouille::ResponseBody::from_string(CSS),
                    upgrade: None,
                };
            }
            "/favicon.ico" => Err(ApiError::Empty),
            _ => {
                if path.starts_with("/authors/") {
                    let offset = request
                        .get_param("offset")
                        .map_or(0, |s| s.parse::<u32>().unwrap_or(0));
                    topic(&client, &path, offset, 20)
                } else if path.starts_with("/article/") {
                    match render_legacy_article(&client, &path) {
                        Ok(result) => result,
                        Err(response) => return response,
                    }
                } else if let Some(path) = path.strip_prefix("/companies/") {
                    render_market(&client, path)
                } else if let Some(path) = path.strip_prefix("/markets/companies/") {
                    render_market(&client, path)
                } else {
                    render_article(&client, &path)
                }
            }
        };

        match response {
            Ok(body) => rouille::Response::html(body),
            Err(err) => render_api_error(&err, &path),
        }
    });
    */
}
