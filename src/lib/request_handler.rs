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
use std::time::{SystemTime, UNIX_EPOCH};
use bytes::Bytes;
use crate::lib::fs_interaction;
use crate::VERSION;

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
    let relative_path = std::path::Path::new(&path);
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
            Html::from(build_dir_page(&app_state.title_name, &app_state.root_path, &relative_path).await?)
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
    title: String,
    relative_path: &'a str,
    entries: &'a [DirEntry],
    current_location_name: &'a str,
    path_parts: Vec<PathPart<'a>>,
}

pub struct PathPart<'a> {
    pub name: &'a str,
    pub url: &'a str,
}

pub async fn build_template(title_name: &Option<String>, entries: &[DirEntry], relative_path: &std::path::Path) -> Result<String, AppErrorExternal> {
    let title = title_name.as_deref().unwrap_or_else(|| "SkyFolder");
    let folder_name = relative_path.file_name().and_then(OsStr::to_str).unwrap_or_else(|| "Home");
    let title = format!("{folder_name} - {title}");
    let current_location_name = folder_name;

    let relative_path_str = relative_path.to_str().unwrap_or_else(|| "");

    let mut path_parts: Vec<PathPart> = Vec::new();

    if relative_path_str.len() > 1 {
        let mut last_index: usize = 0;
        for (index, character) in relative_path_str[1..].char_indices() {
            if character == '/' {
                let name = &relative_path_str[last_index..index+1];
                let url = &relative_path_str[..index+2];
                path_parts.push(PathPart { name, url });
                last_index = index + 2;
            }
        }
    }

    let template = DirectoryTemplate {
        title,
        relative_path: relative_path_str,
        entries,
        current_location_name,
        path_parts,
    };

    Ok(template.render()?)
}




impl<'a> DirectoryTemplate<'a> {
    fn js(&self) -> &'static str {
        crate::JS_FILENAME
    }
    fn css(&self) -> &'static str {
        crate::CSS_FILENAME
    }
}

impl DirEntry {
    pub fn display_modified_raw(&self) -> String {
        match &self.last_modified {
            Some(date) => date.to_string(),
            None => "?".to_string(),
        }
    }
    pub fn display_size_raw(&self) -> String {
        match self.size {
            Some(size) => size.to_string(),
            None => "?".to_string(),
        }
    }
    pub fn icon_picker(&self) -> &str {
        // The `position_text` variable stores the position of the icon in the sprite sheet
        // The coordinates are assigned based on whether the item is a directory or a file with a specific extension
        let position_text = if self.is_dir {
            // If the item is a directory, its icon is located at "-128px 0px" in the sprite sheet
            "-128px 0px"
        } else {
            // If the item is not a directory, it's a file. We need to look at the file's extension
            let name = &self.name.to_lowercase();

            // We find the last occurrence of the '.' character in the name,
            // and slice the string from one character after this position to the end, to get the file extension
            match &name.rfind('.').map(|i| &name[i + 1..]) {
                // Depending on the extension, we match the appropriate icon from the sprite sheet
                Some("rs") => "0px -128px",
                Some("iso") => "-384px 0px",
                Some("json") | Some("js") => "-512px 0px",
                Some("py") => "-640px 0px",
                Some("zip") | Some("gz") | Some("rar") | Some("7z") | Some("tar") | Some("bz2") | Some("xz") => "-768px 0px",
                Some("pdf") => "-896px 0px",
                Some("jpg") | Some("jpeg") => "-512px -128px",
                Some("svg")  => "-384px -128px",
                Some("png")  => "-640px -128px",
                Some("gif")  => "-896px -128px",
                Some("ds_store")  => "-768px -128px",
                // If we can't match the extension to any of the above, we use a default icon located at "-256px 0px"
                _ => "-256px 0px"
            }
        };

        // Return the coordinates of the appropriate icon
        position_text
    }


    pub fn format_file_size(&self) -> String {
        let units = vec!["b", "kb", "mb", "gb", "tb"];
        let kib = 1024.0;

        let bytes = match self.size {
            Some(b) => b as f64,
            None => return "-".to_string(),
        };

        if bytes == 0.0 || bytes.is_nan() {
            return "-".to_string();
        }

        let base = f64::log(bytes, kib).floor() as usize;
        if base >= units.len() {
            return "Too Large".to_string();
        }

        let adjusted_bytes = bytes / f64::powi(kib, base as i32);
        let unit = &units[base];

        if f64::round(adjusted_bytes.fract() * 10.0) == 0.0 {
            return format!("{} <span>{}</span>", adjusted_bytes.floor(), unit);
        } else {
            let formatted_adjusted_bytes = format!("{:.1}", adjusted_bytes);
            return format!("{} <span>{}</span>", formatted_adjusted_bytes.trim_end_matches(".0"), unit);
        }
    }





    pub fn format_time_ago(&self) -> String {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let time_difference = match self.last_modified {
            Some(time) => current_time - time,
            None => return "Unknown".to_string(),
        };

        if time_difference < 60 {
            return "Just now".to_string();
        }

        let minutes = time_difference / 60;

        if minutes < 60 {
            return format!("{} {} ago", minutes, if minutes == 1 { "minute" } else { "minutes" });
        }

        let hours = minutes / 60;

        if hours < 24 {
            return format!("{} {} ago", hours, if hours == 1 { "hour" } else { "hours" });
        }

        let days = hours / 24;

        format!("{} {} ago", days, if days == 1 { "day" } else { "days" })
    }

}

trait FormatPath {
    fn format_path(&self) -> Vec<&str>;
}

impl FormatPath for &str {
    fn format_path(&self) -> Vec<&str> {
        let mut path_parts: Vec<&str> = self.split('/').collect();
        if self.ends_with('/') {
            path_parts.pop();
        }
        path_parts
    }
}





// embedding this data in the binary allows it to work without external files
static SPRITESHEET: Bytes = Bytes::from_static(include_bytes!("../../assets/spritesheet.webp"));

static SCRIPTS: Bytes = Bytes::from_static(include_bytes!("../../assets/scripts.js"));
static FAVICON: Bytes = Bytes::from_static(include_bytes!("../../assets/favicon.ico"));

//serving these files with Axum rather than dynamically templated in with Askama results in better performance and memory usage
pub async fn serve_spritesheet() -> Result<impl IntoResponse, AppErrorExternal> {
    Ok(
        Response::builder()
            .status(StatusCode::OK)
            .header(hyper::header::CONTENT_TYPE, "image/webp")
            .header("Cache-Control", "public, max-age=7884000")
            .body(Body::from(&*SPRITESHEET))?
    )
}

pub async fn serve_favicon() -> Result<impl IntoResponse, AppErrorExternal> {
    Ok(
        Response::builder()
            .status(StatusCode::OK)
            .header(hyper::header::CONTENT_TYPE, "image/x-icon")
            .header("Cache-Control", "public, max-age=7884000")
            .body(Body::from(&*FAVICON))?
    )
}


pub async fn serve_css() -> Result<impl IntoResponse, AppErrorExternal> {
    Ok(
        Response::builder()
            .status(StatusCode::OK)
            .header(hyper::header::CONTENT_TYPE, "text/css")
            .header("Cache-Control", "public, max-age=7884000")
            .body(Body::from(crate::STYLES))?
    )
}


pub async fn serve_js() -> Result<impl IntoResponse, AppErrorExternal> {
    Ok(
        Response::builder()
            .status(StatusCode::OK)
            .header(hyper::header::CONTENT_TYPE, "text/javascript")
            .header("Cache-Control", "public, max-age=7884000")
            .body(Body::from(&*SCRIPTS))?
    )
}

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate<'a> {
    version: &'a str
}
pub async fn build_about() -> Result<Html<String>, AppErrorExternal> {

    let template = AboutTemplate {
        version: VERSION
    };

    Ok(Html::from(template.render()?))
}