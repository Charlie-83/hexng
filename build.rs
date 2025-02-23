use std::fs;
use std::path::Path;

fn main() {
    let source = "data/data.json";
    let destination = Path::new(&std::env::var("OUT_DIR").unwrap()).join("../../../data.json");

    fs::copy(source, destination).expect("Failed to copy data.json");
}

