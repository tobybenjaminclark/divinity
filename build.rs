use std::fs;
use std::path::Path;

fn main() {

    let src = "program.div";
    let dest = format!("{}/{}", std::env::var("OUT_DIR").unwrap(), src);

    // Copy the file to the output directory
    fs::copy(src, &dest).expect("Failed to copy file");


    lalrpop::process_root().unwrap();
}
