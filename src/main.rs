mod api;
mod client;
mod render;
mod routes;
mod settings;

use std::collections::HashMap;

use api::{error::ApiError, redirect::load_redirect};
use client::Client;
use routes::{
    about::render_about,
    article::render_article,
    internet_news::render_legacy_article,
    markets::render_market,
    proxy::image_proxy,
    search::{render_search, render_section, render_topic},
    settings::handle_settings,
};

const CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/main.css"));

macro_rules! document {
    ($title:expr, $content:expr, $( $head:expr )? ) => {
        maud::html! {
            (maud::DOCTYPE)
            html lang="en" {
                head {
                    title { ($title) }
                    link rel="stylesheet" href="/main.css?v=1";
                    meta name="viewport" content="width=device-width, initial-scale=1";
                    $( ($head) )?
                }
                body {
                    main { ($content) }
                    footer { div {
                        a href="/" { "Home" }
                        " - "
                        // a href="/search" { "Search" }
                        // " - "
                        a href="/settings" { "Settings" }
                        " - "
                        a href="/about" { "About" } } }
                }
            }
        }
    };
}
pub(crate) use document;
use settings::Settings;

pub struct Section {
    id: String,
    name: String,
    children: Vec<SectionChild>,
}

// Shallow copy of the section, so we don't have to clone the whole hierarchy multiple times
pub struct SectionChild {
    id: String,
    name: String,
}

fn main() {
    let mut pargs = pico_args::Arguments::from_env();
    let list_address: String = pargs
        .value_from_str("--address")
        .unwrap_or_else(|_| "127.0.0.1:13369".into());

    let client: ureq::Agent = {
        let certs = rustls_native_certs::load_native_certs().expect("Could not load certs!");

        let mut root_store = rustls::RootCertStore::empty();
        for cert in certs {
            root_store
                .add(&rustls::Certificate(cert.0))
                .expect("Could not add cert!");
        }
        let tls_config = std::sync::Arc::new(
            rustls::ClientConfig::builder()
                .with_safe_defaults()
                .with_root_certificates(root_store)
                .with_no_client_auth(),
        );

        let mut client_builder = ureq::AgentBuilder::new();

        client_builder = client_builder.tls_config(tls_config).redirects(0);

        #[cfg(debug_assertions)]
        {
            println!("Installing middleware");

            struct LoggerMiddleware;

            impl ureq::Middleware for LoggerMiddleware {
                fn handle(
                    &self,
                    request: ureq::Request,
                    next: ureq::MiddlewareNext,
                ) -> Result<ureq::Response, ureq::Error> {
                    println!("{}: {}", request.method(), request.url());
                    let response = next.handle(request);
                    println!(
                        " -> {:?}",
                        response
                            .as_ref()
                            .ok()
                            .map(|r| (r.status(), r.status_text()))
                            .unwrap_or((500, "Failed"))
                    );
                    response
                }
            }

            client_builder = client_builder.middleware(LoggerMiddleware)
        }

        client_builder.build()
    };

    let mut headers = vec![];

    if let Ok(cookie) = pargs.value_from_str("--cookie") {
        println!("Cookie: {cookie}");
        headers.push(("Cookie".to_string(), cookie));
    };

    let client = Client::new(client, headers);

    println!("Fetching site hierarchy");
    let mut sections_by_id: HashMap<String, Section> = std::collections::HashMap::new();
    if let Ok(section) = api::section::fetch_site_hierarchy_by_name(&client) {
        let mut queue = vec![section];
        while let Some(section) = queue.pop() {
            let children = section
                .children
                .as_deref()
                .unwrap_or_default()
                .iter()
                .map(|s| SectionChild {
                    id: s.id.clone(),
                    name: s.name.clone(),
                })
                .collect();
            for child in section.children.unwrap_or_default() {
                queue.push(child);
            }
            let node = Section {
                id: section.id.clone(),
                name: section.name.clone(),
                children,
            };
            sections_by_id.insert(section.id, node);
        }
        println!("Fetched site hierarchy");
        println!("Sections: {}", sections_by_id.len());
    } else {
        eprintln!("Failed to fetch site hierarchy");
    };

    println!("Listening on http://{}", list_address);
    rouille::start_server(list_address, move |request| {
        let path = request.url();
        let settings = Settings::from_request(request);

        let response = match path.as_str() {
            "/" | "/home" | "/world/" => {
                let offset = request
                    .get_param("offset")
                    .map_or(0, |s| s.parse::<u32>().unwrap_or(0));
                let section = sections_by_id
                    .get("/world/")
                    .unwrap_or_else(|| panic!("Section 'world' not found"));

                render_section(&client, section, offset, 8)
            }
            "/about" => render_about(),
            "/settings" => return handle_settings(request, &settings),
            "/search" | "/search/" => render_search(&client, request),
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
                if let Some(section) = sections_by_id.get(path.as_str()) {
                    let offset = request
                        .get_param("offset")
                        .map_or(0, |s| s.parse::<u32>().unwrap_or(0));
                    render_section(&client, section, offset, 8)
                } else if path.starts_with("/authors/") {
                    let offset = request
                        .get_param("offset")
                        .map_or(0, |s| s.parse::<u32>().unwrap_or(0));
                    render_topic(&client, &path, offset, 20)
                } else if let Some(path) = path.strip_prefix("/topic/") {
                    let full_path = format!("https://www.reuters.com/topic/{path}");
                    let redirect = load_redirect(&client, &full_path);
                    match redirect {
                        Ok((status, location)) => {
                            return rouille::Response {
                                status_code: status,
                                headers: vec![
                                    ("Location".into(), strip_prefix(&location).to_owned().into()),
                                    ("Cache-Control".into(), "public, max-age=31536000".into()),
                                ],
                                data: rouille::ResponseBody::empty(),
                                upgrade: None,
                            };
                        }
                        Err(err) => Err(err),
                    }
                } else if path.starts_with("/article/") {
                    match render_legacy_article(&client, &path) {
                        Ok(result) => result,
                        Err(response) => return response,
                    }
                } else if let Some(path) = path.strip_prefix("/companies/") {
                    render_market(&client, path)
                } else if let Some(path) = path.strip_prefix("/markets/companies/") {
                    render_market(&client, path)
                } else if let Some(path) = request.raw_url().strip_prefix("/proxy/") {
                    return image_proxy(&client, request, path);
                } else {
                    render_article(&client, &path, &settings)
                }
            }
        };

        match response {
            Ok(body) => rouille::Response::html(body),
            Err(err) => render_api_error(&err, &path, &settings),
        }
    });
}

fn render_api_error(err: &ApiError, path: &str, settings: &Settings) -> rouille::Response {
    if settings.fast_redirect {
        if let ApiError::Redirect(code, location) = err {
            return rouille::Response {
                status_code: *code,
                headers: vec![
                    ("Location".into(), strip_prefix(location).to_owned().into()),
                    ("Cache-Control".into(), "public, max-age=31536000".into()),
                ],
                data: rouille::ResponseBody::empty(),
                upgrade: None,
            };
        }
    }

    let (status, title) = match err {
        ApiError::Empty | ApiError::External(404, _) => {
            (404, "404 - Content not found".to_string())
        }
        ApiError::Redirect(_, _) => (200, "Redirect found".to_string()),
        ApiError::External(code, _) => (*code, format!("{code} - External error")),
        ApiError::Internal(message) => (500, format!("500 - Internal server error {message}")),
    };

    let (head, details) = match err {
        ApiError::Empty => (maud::html!(), maud::html!()),
        ApiError::Redirect(_, location) => {
            let location = strip_prefix(location);
            (
                maud::html! {
                    meta http-equiv="refresh" content=(format!("{}; url={}", settings.redirect_timer, location));
                    link rel="canonical" href=(location);
                },
                maud::html! {
                    p { "Redirecting to " (location) " in " (settings.redirect_timer) " seconds. Or click " a href=(location) { "here" } " to follow the link directly." }
                },
            )
        }
        ApiError::External(_, message) => (
            maud::html!(),
            maud::html! {
                details {
                    summary { "Server response" }
                    p { (message) }
                }
            },
        ),
        ApiError::Internal(_) => (maud::html!(), maud::html!()),
    };

    let doc = document!(
        &title,
        maud::html! {
            h1 { (&title) }
            p { "You tried to access \"" (path) "\"" }
            (details)
            p { a href="/" { "Go home" } }
            p { a href=(path) { "Try again" } }
        },
        head
    );

    rouille::Response::html(doc.into_string()).with_status_code(status)
}

pub fn strip_prefix(path: &str) -> &str {
    if let Some(path) = path.strip_prefix("https://www.reuters.com") {
        path
    } else if let Some(path) = path.strip_prefix("http://www.reuters.com") {
        path
    } else {
        path
    }
}
