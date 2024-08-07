use std::env;
use std::path::PathBuf;

fn main() {
    eprintln!("{}", env::var("OUT_DIR").unwrap());

    cgo::Build::new()
        .package("../../sharedcgo")
        .build_mode(cgo::BuildMode::CArchive)
        .build("xxdk");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings = bindgen::Builder::default()
        .header(out_path.join("libxxdk.h").to_string_lossy())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_arg("-I../../sharedcgo")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");

    println!("cargo:rustc-link-search=native=./target");
    println!("cargo:rustc-link-lib=static=xxdk");
    println!("cargo:rustc-flags=-lresolv");
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-flags=-l framework=CoreFoundation -l framework=Security");
    }
}
