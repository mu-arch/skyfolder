use std::sync::Arc;
use axum::{Extension, extract::Path};
use crate::lib::errors::{AppErrorExternal};
use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use hyper::Body;
use tokio::fs::File;
use axum::debug_handler;
use crate::AppState;
use crate::lib::fs_interaction::{DirEntry, list_dir_contents};

pub enum ResponseWrapper {
    File(String),
    Html(axum::response::Html<String>),
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

//this is a special case for the root path "/" to simplify axum code
pub async fn handle_root_path(app_state: Extension<Arc<AppState>>) -> Result<ResponseWrapper, AppErrorExternal> {

    Ok(ResponseWrapper::Html(Html::from(build_dir_page(&app_state.root_path).await?)))
}

pub async fn handle_path(Path(path): Path<String>, app_state: Extension<Arc<AppState>>) -> Result<ResponseWrapper, AppErrorExternal> {
    // Split the path into segments
    let path_segments: Vec<_> = path.as_str().split('/').collect();

    // Check if the last segment is a file name (contains a dot)
    let last_segment = match path_segments.last() {
        None => "",
        Some(v) => v
    };

    //create a Path type
    let path = std::path::Path::new(&path);

    if let Some(_) = last_segment.rfind('.') {
            Ok(ResponseWrapper::Html(
                axum::response::Html(format!("You requested file: {}", last_segment))
            ))
    } else {
        Ok(ResponseWrapper::Html(
            Html::from(build_dir_page(path).await?)
        ))
    }
}

pub async fn build_dir_page(path: &std::path::Path) -> Result<String, AppErrorExternal> {

    let entries = list_dir_contents(path).await?;

    build_template(&entries).await
}


#[derive(Template)]
#[template(path = "directory.html")]
pub struct DirectoryTemplate<'a> {
    entries: &'a Vec<DirEntry>,
}
pub async fn build_template(entries: &Vec<DirEntry>) -> Result<String, AppErrorExternal> {

    let template = DirectoryTemplate { entries };

    Ok(template.render()?)
}