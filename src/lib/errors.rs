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

#[derive(Debug)]
pub enum AppErrorExternal {
    TestError
}

impl IntoResponse for AppErrorExternal {
    fn into_response(self) -> Response {

        let (status, error_message) = match self {
            AppErrorExternal::TestError => (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong")
        };

        (status, error_message).into_response()
    }
}

