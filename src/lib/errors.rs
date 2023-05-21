#[derive(Debug)]
pub enum AppErrorInternal {
    IoError(std::io::Error),
    AddrParseError(std::net::AddrParseError),
    HyperError(hyper::Error),
}

impl From<std::io::Error> for AppErrorInternal {
    fn from(inner: std::io::Error) -> Self {
        AppErrorInternal::IoError(inner)
    }
}

impl From<std::net::AddrParseError> for AppErrorInternal {
    fn from(inner: std::net::AddrParseError) -> Self {
        AppErrorInternal::AddrParseError(inner)
    }
}

impl From<hyper::Error> for AppErrorInternal {
    fn from(inner: hyper::Error) -> Self {
        AppErrorInternal::HyperError(inner)
    }
}


use axum::http::{StatusCode};
use axum::response::{IntoResponse, Response};
use hyper::Body;

// Assume `AppErrorExternal` is defined somewhere.
#[derive(Debug)]
pub struct AppErrorExternal {
    // Fields go here.
}

impl AppErrorExternal {
    pub fn into_http_response(self) -> Response {
        let error_message = format!("An error occurred: {:?}", self);

        let body = match self {
            _ => "something went wrong"
        };

        // its often easiest to implement `IntoResponse` by calling other implementations
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

