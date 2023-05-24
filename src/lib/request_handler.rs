use std::sync::Arc;
use axum::{Extension, extract::Path};
use crate::lib::errors::{AppErrorExternal};
use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use hyper::Body;
use hyper::{StatusCode};
use tokio::fs::File;
use axum::debug_handler;
use crate::AppState;
use crate::lib::fs_interaction::{DirEntry, list_dir_contents};
use std::ffi::OsStr;
use bytes::Bytes;

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

    Ok(ResponseWrapper::Html(Html::from(build_dir_page(&app_state.title_name, &app_state.root_path, std::path::Path::new("")).await?)))
}


pub async fn handle_path(Path(path): Path<String>, app_state: Extension<Arc<AppState>>) -> Result<ResponseWrapper, AppErrorExternal> {

    let root_path = std::path::Path::new(&app_state.root_path);
    let canonical_root_path = root_path.canonicalize()?;

    // Construct the full path
    let full_path = root_path.join(&path);

    // Canonicalize the full path, resolving any ".." or "." segments
    let canonical_full_path = full_path.canonicalize()?;

    // If the canonical full path does not start with the canonical root path,
    // then this is a path traversal attempt, and we should not serve the file.
    if !canonical_full_path.starts_with(&canonical_root_path) {
        return Err(AppErrorExternal::PathTraversal);
    }

    // Here, you can be sure that canonical_full_path is within root_path.
    if path.as_str().chars().last() == Some('/') {
        Ok(ResponseWrapper::Html(
            Html::from(build_dir_page(&app_state.title_name, &app_state.root_path, &canonical_full_path).await?)
        ))
    } else {
        Ok(ResponseWrapper::Html(
            axum::response::Html(format!("You requested file: {}", path))
        ))
    }
}

pub async fn build_dir_page(title_name: &Option<String>, root_path: &std::path::Path, relative_path: &std::path::Path) -> Result<String, AppErrorExternal> {

    let path = root_path.join(relative_path);

    let entries = list_dir_contents(&path).await?;

    build_template(title_name, &entries, relative_path).await
}


#[derive(Template)]
#[template(path = "directory.html")]
pub struct DirectoryTemplate<'a> {
    title_name: String,
    relative_path: &'a str,
    entries: &'a Vec<DirEntry>,
}
pub async fn build_template(title_name: &Option<String>, entries: &Vec<DirEntry>, relative_path: &std::path::Path) -> Result<String, AppErrorExternal> {

    let title_name = title_name.as_deref().unwrap_or_else(|| "SkyFolder");
    let folder_name = relative_path.file_name().unwrap_or(OsStr::new("Home")).to_string_lossy();
    let title_name = format!("{folder_name} - {title_name}");

    let relative_path = relative_path.to_str().unwrap_or_else(|| "");

    let template = DirectoryTemplate {
        title_name,
        relative_path,
        entries
    };

    Ok(template.render()?)
}

impl DirEntry {
    pub fn formatted_last_modified(&self) -> String {
        match &self.last_modified {
            Some(date) => date.to_string(),
            None => "-".to_string(),
        }
    }
    pub fn formatted_size(&self) -> String {
        match self.size {
            Some(size) => size.to_string(),
            None => "-".to_string(),
        }
    }
    pub fn icon_picker(&self) -> String {
        let position_text = if self.is_dir {
            "-128px 0px".to_owned()
        } else {
            match &self.name.rfind('.').map(|i| &self.name[i + 1..]) {
                Some("rs") => "0px -128px".to_owned(),
                _ => "-256px 0px".to_owned()
            }
        };

        format!("style=\"background-position:{position_text}\"")
    }
}

/*
Use ServeFile. If you want to have a handler that does something to determine the file path (please make sure to guard against path traversal attacks¹) or do some work before returning the file, create ServeFile within the handler and use oneshot² to hand the request over to it.
 */


// emdedding this data in the binary allows it to work without external files
static SPRITESHEET: Bytes = Bytes::from_static(include_bytes!("../../assets/spritesheet.webp"));
static STYLES: Bytes = Bytes::from_static(include_bytes!("../../assets/styles.css"));
static SCRIPTS: Bytes = Bytes::from_static(include_bytes!("../../assets/scripts.js"));
static FAVICON: Bytes = Bytes::from_static(include_bytes!("../../assets/favicon.ico"));

//serving these files with Axum rather than dynamically templated in with Askama results in better performance and memory usage
pub async fn serve_spritesheet() -> Result<impl IntoResponse, AppErrorExternal> {
    Ok(
        Response::builder()
            .status(StatusCode::OK)
            .header(hyper::header::CONTENT_TYPE, "image/webp")
            .body(Body::from(&*SPRITESHEET))?
    )
}

pub async fn serve_favicon() -> Result<impl IntoResponse, AppErrorExternal> {
    Ok(
        Response::builder()
            .status(StatusCode::OK)
            .header(hyper::header::CONTENT_TYPE, "image/x-icon")
            .body(Body::from(&*FAVICON))?
    )
}


pub async fn serve_css() -> Result<impl IntoResponse, AppErrorExternal> {
    Ok(
        Response::builder()
            .status(StatusCode::OK)
            .header(hyper::header::CONTENT_TYPE, "text/css")
            .body(Body::from(&*STYLES))?
    )
}


pub async fn serve_js() -> Result<impl IntoResponse, AppErrorExternal> {
    Ok(
        Response::builder()
            .status(StatusCode::OK)
            .header(hyper::header::CONTENT_TYPE, "text/javascript")
            .body(Body::from(&*SCRIPTS))?
    )
}

//Indicates the last time this directory was directly modified, such as when files or subdirectories were added, removed, or renamed within it. Changes to files or modifications within subdirectories do not affect this timestamp.