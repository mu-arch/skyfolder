#[derive(Debug)]
pub enum AppErrorInternal {
    AddrParseError(std::net::AddrParseError),
    HyperError(hyper::Error),
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
     IoError(std::io::Error),
    AskamaError(askama::Error)
}

impl IntoResponse for AppErrorExternal {
    fn into_response(self) -> Response {

        let (status, error_message) = match self {
            AppErrorExternal::IoError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal IO Error"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong")
        };

        (status, error_message).into_response()
    }
}

impl From<std::io::Error> for AppErrorExternal {
    fn from(inner: std::io::Error) -> Self {
        AppErrorExternal::IoError(inner)
    }
}

impl From<askama::Error> for AppErrorExternal {
    fn from(inner: askama::Error) -> Self {
        AppErrorExternal::AskamaError(inner)
    }
}
