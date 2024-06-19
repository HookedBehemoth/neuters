mod api;
mod render;
mod routes;

use api::error::ApiError;
use routes::{
    about::render_about,
    article::render_article,
    internet_news::render_legacy_article,
    markets::render_market,
    search::{render_search, render_section, render_topic},
};

const CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/main.css"));

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
                    print!("{}: {}", request.method(), request.url());
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

    println!("Listening on http://{}", list_address);
    rouille::start_server(list_address, move |request| {
        let path = request.url();
        let response = match path.as_str() {
            "/" | "/home" => render_section(&client, "/world/", 0, 8),
            "/about" => render_about(),
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
                if path.starts_with("/authors/") {
                    let offset = request
                        .get_param("offset")
                        .map_or(0, |s| s.parse::<u32>().unwrap_or(0));
                    render_topic(&client, &path, offset, 20)
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
}

fn render_error(code: u16, message: &str, path: &str) -> rouille::Response {
    let title = format!("{} - {}", code, message);

    let doc = document!(
        &title,
        maud::html! {
            h1 { (&title) }
            p { "You tried to access \"" (path) "\"" }
            p { a href="/" { "Go home" } }
            p { a href=(path) { "Try again" } }
        },
    );

    rouille::Response::html(doc.into_string()).with_status_code(code)
}

fn render_api_error(err: &ApiError, path: &str) -> rouille::Response {
    match &err {
        ApiError::External(code, message) => render_error(*code, message, path),
        ApiError::Internal(message) => render_error(500, message, path),
        ApiError::Empty => rouille::Response::empty_404(),
    }
}
