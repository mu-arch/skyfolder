use std::path::{Path, PathBuf};
use std::sync::Arc;
use crate::lib::{fs_interaction, parse_cli_args, request_handler};
use crate::lib::errors::AppErrorInternal;
use axum::routing::get;
use axum::{Extension, Server};

mod lib {
    pub(crate) mod errors;
    pub(crate) mod fs_interaction;
    pub(crate) mod parse_cli_args;
    pub(crate) mod request_handler;
}

#[tokio::main]
async fn main() {

    if let Err(e) = init().await {
        eprintln!("The program has crashed: {:?}", e)
    }

}

pub struct AppState {
    pub(crate) root_path: PathBuf
}

async fn init() -> Result<(), AppErrorInternal> {

    let app_state = {
        let args = parse_cli_args::parse_args()?;
        Arc::new(AppState {
            root_path: args.path
        })
    };

    println!("Serving directory: {}", app_state.root_path.display());

    let app = axum::Router::new()
        .route("/", get(request_handler::handle_root_path))
        .route("/*path", get(request_handler::handle_path))
        .layer(Extension(app_state));

    Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
