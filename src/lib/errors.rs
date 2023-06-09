#[derive(Debug)]
pub enum AppErrorInternal {
    AddrParseError(std::net::AddrParseError),
    HyperError(hyper::Error),
    //AcmeError(acme_micro::Error),
    Custom(String),
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

/*
impl From<acme_micro::Error> for AppErrorInternal {
    fn from(inner: acme_micro::Error) -> Self {
        AppErrorInternal::AcmeError(inner)
    }
}

 */


use axum::http::{StatusCode};
use axum::response::{IntoResponse, Response};
use hyper::Body;

#[derive(Debug)]
pub enum AppErrorExternal {
    IoError(std::io::Error),
    AskamaError(askama::Error),
    AxumError(axum::http::Error),
    FileNotFound,
    PathTraversal
}

impl IntoResponse for AppErrorExternal {
    fn into_response(self) -> Response {
        dbg!(&self);

        let (status, error_message) = match self {
            AppErrorExternal::IoError(ref e) if e.kind() == std::io::ErrorKind::NotFound =>
                (StatusCode::NOT_FOUND, "File not found"),
            AppErrorExternal::IoError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal IO Error"),
            AppErrorExternal::FileNotFound => (StatusCode::NOT_FOUND, "File not found"),
            AppErrorExternal::PathTraversal => (StatusCode::FORBIDDEN, "nice try"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR")
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

impl From<axum::http::Error> for AppErrorExternal {
    fn from(inner: axum::http::Error) -> Self {
        AppErrorExternal::AxumError(inner)
    }
}

