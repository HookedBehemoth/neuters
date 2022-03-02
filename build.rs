use glob::glob;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let is_debug = env::var_os("PROFILE") == Some("debug".into());

    let src: &str = "sass/main.scss";
    let dst: &str = "main.css";
    let out_dir: PathBuf = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    for entry in glob("sass/**/*.scss").expect("Failed to glob") {
        println!("cargo:rerun-if-changed={}", entry.unwrap().display());
    }

    /* Compress css in release mode */
    let options = grass::Options::default().style(if is_debug {
        grass::OutputStyle::Expanded
    } else {
        grass::OutputStyle::Compressed
    });

    let css = grass::from_path(src, &options).unwrap();
    let dst = out_dir.join(dst);
    fs::write(dst, &css).unwrap();
}
