use clap::{arg, command, value_parser, ArgAction, Command};
use std::env;
use std::path::PathBuf;
use crate::lib::errors::AppError;

pub struct Args {
    pub path: PathBuf,
}

pub fn parse_args() -> Result<Args, AppError> {

    let matches = command!()
        .arg(
            arg!(
                -p --path <PATH> "Sets a custom path to serve from"
            )
                // We don't have syntax yet for optional options, so manually calling `required`
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();


    let path = match matches.get_one::<PathBuf>("path") {
        None => env::current_dir().expect("Couldn't determine the current directory. The program will exit."),
        Some(v) => v.clone()
    };
    
    let args = Args {
        path
    };

    Ok(args)
}