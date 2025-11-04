use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    // Configure and run bindgen
    let bindings = bindgen::Builder::default()
        .header("bindgen-wrapper.h")
        .blocklist_item("IPPORT_RESERVED")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the Cargo output directory
    // let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    // let out_path = PathBuf::from(out_path.iter().nth(1).unwrap());
    let out_path = PathBuf::from("src/");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
