use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

const CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/main.css"));

pub async fn css() -> Response {
    (
        [
            ("Content-Type", "text/css"),
            ("Cache-Control", "public, max-age=31536000"),
        ],
        CSS,
    )
        .into_response()
}

pub async fn empty_not_found() -> (StatusCode, ()) {
    (StatusCode::NOT_FOUND, ())
}
