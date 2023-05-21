use axum::{extract::Path};
use crate::lib::errors::{AppErrorExternal};
use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use hyper::Body;
use tokio::fs::File;
use axum_debug::debug_handler;

pub enum ResponseWrapper {
    File(String),
    Html(String),
}

impl IntoResponse for ResponseWrapper {

    fn into_response(self) -> Response {
        match self {
            ResponseWrapper::File(content) => {
                content.into_response()
            },
            ResponseWrapper::Html(content) => {
                content.into_response()
            },
        }
    }
}

#[debug_handler]
pub async fn handle_path(Path(path): Path<String>) -> Result<ResponseWrapper, AppErrorExternal> {
    // Split the path into segments
    let path_segments: Vec<_> = path.as_str().split('/').collect();

    // Check if the last segment is a file name (contains a dot)
    let last_segment = match path_segments.last() {
        None => "",
        Some(v) => v
    };

    if let Some(_) = last_segment.rfind('.') {
            Ok(ResponseWrapper::Html(
                format!("You requested file: {}", last_segment)
            ))
    } else {
        Ok(ResponseWrapper::Html(
            format!("You requested directory: {}", path.as_str())
        ))
    }
}


#[derive(Template)]
#[template(path = "directory_listing.html")]
struct DirectoryListing {
    name: String,
    age: u8,
}

pub async fn list_dir(Path((name, age)): Path<(String, u8)>) -> DirectoryListing {
    DirectoryListing { name, age }
}