#[derive(Debug)]
pub enum AppError {
    IoError(std::io::Error),
    AddrParseError(std::net::AddrParseError),
    HyperError(hyper::Error),
}

impl From<std::io::Error> for AppError {
    fn from(inner: std::io::Error) -> Self {
        AppError::IoError(inner)
    }
}

impl From<std::net::AddrParseError> for AppError {
    fn from(inner: std::net::AddrParseError) -> Self {
        AppError::AddrParseError(inner)
    }
}

impl From<hyper::Error> for AppError {
    fn from(inner: hyper::Error) -> Self {
        AppError::HyperError(inner)
    }
}