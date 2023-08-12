use std::env;
use std::fs;
use std::path::Path;
use rand::Rng;
use rand::distributions::Alphanumeric;
use std::fmt::Write;
use minify_js::{Session, TopLevelMode, minify};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("constants.rs");

    let build_id = generate_random_string(15);

    let raster_sprites_filename = format!("/r{}.webp", build_id);
    let vector_sprites_filename = format!("/v{}.svg", build_id);

    let mut out = String::new();

    let css_file = fs::read_to_string("assets/styles.css").unwrap();
    let css_file = css_file.replace("/r.webp", &raster_sprites_filename);
    let css_file = css_file.replace("/v.svg", &vector_sprites_filename);

    write!(out, "pub const RASTER_SPRITES_FILENAME: &str = \"{raster_sprites_filename}\";").unwrap();
    write!(out, "pub const VECTOR_SPRITES_FILENAME: &str = \"{vector_sprites_filename}\";").unwrap();
    write!(out, "pub const JS_FILENAME: &str = \"/js{}.js\";", build_id).unwrap();
    write!(out, "pub const CSS_FILENAME: &str = \"/css{}.css\";", build_id).unwrap();

    //write in the css file with our replaced filenames
    write!(out, "pub const STYLES: &[u8] = b\"{css_file}\";").unwrap();

    //minify js
    let js_file = fs::read_to_string("assets/scripts.js").unwrap();
    let session = Session::new();
    let mut minified = Vec::new();
    minify(&session, TopLevelMode::Global, js_file.as_ref(), &mut minified).unwrap();
    let js_file = String::from_utf8(Vec::from(minified.as_slice())).unwrap();

    write!(out, "pub const SCRIPTS: &[u8] = b\"{js_file}\";").unwrap();


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