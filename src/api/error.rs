use std::io;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use maud::Markup;

use crate::document;

#[derive(Debug)]
pub enum ApiError {
    External(u16, String, String),
    Reqwest(reqwest::Error),
    Internal(String),
    Empty,
}

pub type ApiResult<T> = Result<T, ApiError>;

impl From<serde_json::Error> for ApiError {
    fn from(e: serde_json::Error) -> Self {
        Self::Internal(format!("Failed to deserialize API response: {e}"))
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> Self {
        Self::Reqwest(error)
    }
}

fn render_reqwest_error(error: reqwest::Error) -> (StatusCode, Markup) {
    let code = error.status().unwrap_or(StatusCode::CONFLICT);
    let message = error.to_string();
    let path = error.url().map(|url| url.to_string()).unwrap_or(String::new());
    
    render_error(code, &message, &path)
}

fn render_error(code: StatusCode, message: &str, path: &str) -> (StatusCode, Markup) {
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

    (code, doc)
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Reqwest(error) => render_reqwest_error(error).into_response(),
            ApiError::External(code, path, message) => render_error(StatusCode::from_u16(code).unwrap(), &message, &path).into_response(),
            ApiError::Internal(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
            }
            ApiError::Empty => StatusCode::NOT_FOUND.into_response(),
        }
    }
}

impl From<io::Error> for ApiError {
    fn from(err: io::Error) -> Self {
        Self::Internal(format!("IO error: {}", err))
    }
}
