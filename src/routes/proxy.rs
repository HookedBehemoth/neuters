use rouille::{Request, Response};

use crate::client::Client;

const SITE_PREFIX: &str = "https://www.reuters.com/";

/* Note: passing these to the client should be avoided */
const FORBIDDEN_CLIENT_HEADERS: &[&str] = &["connection", "cookies", "set-cookie"];
const FORBIDDEN_SERVER_HEADERS: &[&str] =
    &["connection", "cookie", "user-agent", "host", "referer"];

pub fn strip_prefix(path: &str) -> Option<&str> {
    path.strip_prefix(SITE_PREFIX)
}

pub fn image_proxy(
    client: &Client,
    request: &Request,
    path: &str,
) -> Response {
    let url = format!("{SITE_PREFIX}{}", path);
    let mut req = client.get(&url);

    for header in request
        .headers()
        .filter(|(h, _)| !FORBIDDEN_SERVER_HEADERS.contains(&h.to_lowercase().as_str()))
    {
        req = req.set(header.0, header.1);
    }

    let Ok(res) = req.call() else {
        return Response::text("Error fetching image")
            .with_status_code(500)
    };
    let status = res.status();

    let headers = res
        .headers_names()
        .iter()
        .filter(|h| !FORBIDDEN_CLIENT_HEADERS.contains(&h.as_str()))
        .map(|s| (s.clone().into(), res.header(s).unwrap().to_owned().into()))
        .collect();

    let reader = match res.header("Content-Length").map(|s| s.parse::<usize>()) {
        Some(Ok(len)) => rouille::ResponseBody::from_reader_and_size(res.into_reader(), len),
        _ => rouille::ResponseBody::from_reader(res.into_reader()),
    };

    rouille::Response {
        status_code: status,
        headers,
        data: reader,
        upgrade: None,
    }
}