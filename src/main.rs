use crate::lib::{fs_interaction, parse_cli_args, request_handler};
use crate::lib::errors::AppErrorInternal;
use axum::routing::get;
use axum::Server;

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

async fn init() -> Result<(), AppErrorInternal> {
    let args = parse_cli_args::parse_args()?;

    println!("The current directory is {}", args.path.display());

    let result = fs_interaction::list_dir_contents(&args.path).await;
    dbg!(result);

    let app = axum::Router::new()
        .route("/*path", get(debug_handler::new(request_handler::handle_path)));

    Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
