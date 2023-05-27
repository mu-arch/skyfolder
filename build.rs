use std::env;
use std::fs;
use std::path::Path;
use rand::Rng;
use rand::distributions::Alphanumeric;
use std::fmt::Write;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("constants.rs");

    let sprites_filename = format!("/sprites{}.webp", generate_random_string(25));

    let mut out = String::new();

    let css_file = fs::read_to_string("assets/styles.css").unwrap();
    let css_file = css_file.replace("/spritesheet.webp", &sprites_filename);

    write!(out, "pub const SPRITES_FILENAME: &str = \"{sprites_filename}\";").unwrap();
    write!(out, "pub const JS_FILENAME: &str = \"/js{}.js\";", generate_random_string(25)).unwrap();
    write!(out, "pub const CSS_FILENAME: &str = \"/css{}.css\";", generate_random_string(25)).unwrap();
    //write in the css file with our changes
    write!(out, "pub const STYLES: &[u8] = b\"{css_file}\";").unwrap();

    fs::write(
        &dest_path,
        out,
    )
        .unwrap();
}

fn generate_random_string(length: u16) -> String {
    let rand_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length.try_into().unwrap())
        .map(char::from)
        .collect();
    rand_string
}