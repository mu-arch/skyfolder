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

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {

    //cool rainbow effect on launch text
    let colors = [31, 33, 32, 36, 34, 35, 37]; // ANSI color codes
    let mut out = String::new();

    for (i, letter) in "JollyFolder".chars().enumerate() {
        let color = colors[i % colors.len()];
        out.push_str(&format!("\x1b[{}m{}\x1b[0m", color, letter));
    }

    println!("Starting up {out} {VERSION}");

    //start the actual application
    if let Err(e) = init().await {
        eprintln!("The program has crashed: {:?}", e)
    }

}

pub struct AppState {
    pub(crate) root_path: PathBuf,
    pub(crate) port: u16
}

async fn init() -> Result<(), AppErrorInternal> {

    let app_state = {
        let args = parse_cli_args::parse_args()?;
        Arc::new(AppState {
            root_path: args.path,
            port: 30080
        })
    };

    println!("Serving path: {}", app_state.root_path.display());
    println!("Bound to [::]:{}, 0.0.0.0:{}", app_state.port, app_state.port);
    let ifaces = get_if_addrs::get_if_addrs().unwrap();

    println!("Available at (non-exhaustive list):");

    for iface in ifaces {
        match iface.addr {
            get_if_addrs::IfAddr::V4(addr) => {
                println!("    http://{}:{}", addr.ip, app_state.port);
            }
            get_if_addrs::IfAddr::V6(addr) => {
                println!("    http://[{}]:{}", addr.ip, app_state.port);
            }
        }
    }

    let app = axum::Router::new()
        .route("/", get(request_handler::handle_root_path))
        .route("/*path", get(request_handler::handle_path))
        .layer(Extension(app_state));

    Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
