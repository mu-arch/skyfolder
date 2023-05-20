use crate::lib::{fs_interaction, parse_cli_args};
use crate::lib::errors::AppError;
use askama::Template;
use axum::extract::Path;
use axum::routing::get;
use axum::Server;

mod lib {
    pub(crate) mod errors;
    pub(crate) mod fs_interaction;
    pub(crate) mod parse_cli_args;
}

#[tokio::main]
async fn main() {

    if let Err(e) = init().await {
        eprintln!("The program has crashed: {:?}", e)
    }

}

async fn init() -> Result<(), AppError> {
    let args = parse_cli_args::parse_args()?;

    println!("The current directory is {}", args.path.display());

    let result = fs_interaction::list_dir_contents(&args.path).await;
    dbg!(result);

    let app = axum::Router::new()
        .route("/:name/:age", get(index));

    Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[derive(Template)]
#[template(path = "index.html")]
struct DirectoryListing {
    name: String,
    age: u8,
}

pub async fn list_dir(Path((name, age)): Path<(String, u8)>) -> DirectoryListing {
    DirectoryListing { name, age }
}