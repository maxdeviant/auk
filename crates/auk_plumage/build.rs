use std::fs;
use std::path::Path;

use walkdir::WalkDir;

const SASS_INPUT_DIR: &str = "sass";

fn main() {
    println!("cargo:rerun-if-changed={SASS_INPUT_DIR}");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let sass_out_dir = Path::new(&out_dir).join("sass");
    fs::create_dir_all(&sass_out_dir).unwrap();

    for entry in WalkDir::new(SASS_INPUT_DIR) {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let relative_path = path.strip_prefix(SASS_INPUT_DIR).unwrap();
            let dest_path = sass_out_dir.join(relative_path);
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }

            fs::copy(path, dest_path).unwrap();
        }
    }
}
