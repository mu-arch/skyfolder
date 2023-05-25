#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

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
    pub(crate) mod helper;
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {

    println!("Starting up \x1B[95mSkyFolder\x1B[0m {VERSION}\n");
    println!("✅ If you need a feature or find a bug let me know in the Github issues tab.");
    println!("⭐ If you like the program please star it on Github as it helps me.");
    println!("💰 If you get value from the program please consider sponsoring me on Github. Ty!\n");

    //start the actual application
    if let Err(e) = init().await {
        eprintln!("The program has crashed: {:?}", e)
    }

}

pub struct AppState {
    pub(crate) root_path: PathBuf,
    pub(crate) port: u16,
    pub(crate) title_name: Option<String> // the name the user gives to their server i.e. what is shown in page title tag
}

async fn init() -> Result<(), AppErrorInternal> {

    let app_state = {
        let args = parse_cli_args::parse_args()?;
        Arc::new(AppState {
            root_path: args.path,
            port: 30080,
            title_name: None
        })
    };

    println!("Serving path: {}", app_state.root_path.display());
    println!("Bound to [::]:{}, 0.0.0.0:{}", app_state.port, app_state.port);
    let ifaces = get_if_addrs::get_if_addrs().unwrap();

    println!("Available at (non-exhaustive list):");

    for iface in ifaces {
        match iface.addr {
            get_if_addrs::IfAddr::V4(addr) => {
                println!("    http://{}:{}", addr.ip, &app_state.port);
            }
            get_if_addrs::IfAddr::V6(addr) => {
                println!("    http://[{}]:{}", addr.ip, &app_state.port);
            }
        }
    }

    let app = axum::Router::new()
        .route("/", get(request_handler::handle_root_path))
        .route("/*path", get(request_handler::handle_path))
        .route("/about_skyfolder", get(request_handler::build_about))
        .layer(Extension(app_state.clone()))
        .route("/spritesheet.webp", get(request_handler::serve_spritesheet))
        .route("/favicon.ico", get(request_handler::serve_favicon))
        .route("/styles.css", get(request_handler::serve_css))
        .route("/scripts.js", get(request_handler::serve_js));

    Server::bind(&format!("0.0.0.0:{}", app_state.port).parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
