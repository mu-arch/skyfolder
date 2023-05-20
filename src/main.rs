use std::path::Path;
use std::env;
use crate::lib::{fs_interaction, parse_cli_args};
use clap::{Arg};

mod lib {
    pub(crate) mod errors;
    pub(crate) mod fs_interaction;
    pub(crate) mod parse_cli_args;
}

#[tokio::main]
async fn main() {

    let args = parse_cli_args::parse_args()?;

    println!("The current directory is {}", args.path.display());

    let result = fs_interaction::list_dir_contents(&args.path).await;
    dbg!(result);
}
