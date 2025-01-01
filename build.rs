use std::env;
use std::path::PathBuf;

use qpm_cli::models::package::{PackageConfigExtensions, SharedPackageConfigExtensions};
use qpm_cli::package::models::dependency::SharedPackageConfig;
use qpm_cli::package::models::package::PackageConfig;
use qpm_cli::repository;
use qpm_cli::resolver::dependency;

// without lib|.so prefix/suffix
const SCOTLAND2_LIB_NAME: &str = "sl2";

const SCOTLAND2_PACKAGE_NAME: &str = "scotland2";
const SCOTLAND2_HEADER: &str = "modloader.h";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=qpm.json");

    let manifest_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let lib_path = out_path.join("extern").join("libs");
    let headers_path = out_path.join("extern").join("includes");

    eprintln!("Downloading sl2 from qpm");
    let package = PackageConfig::read(manifest_path)?;
    let mut repo = repository::useful_default_new(false)?;
    let (shared_package, resolved_deps) =
        SharedPackageConfig::resolve_from_package(package, &repo)?;

    dependency::restore(&out_path, &shared_package, &resolved_deps, &mut repo)?;

    // change if qpm.shared.json modified
    println!(
        "cargo:rerun-if-changed={}",
        out_path.join("qpm.shared.json").display()
    );

    eprintln!("Generating bindings for scotland2");
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.

    let header_path = headers_path
        .join(SCOTLAND2_PACKAGE_NAME)
        .join("shared")
        .join(SCOTLAND2_HEADER);

    // if std::env::var("RUST_BINDGEN").is_ok() {
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        // .header(header_path.to_str().unwrap())
        .generate_cstr(true)
        .header(header_path.to_str().unwrap())
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    // }

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", lib_path.display());

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib={SCOTLAND2_LIB_NAME}");

    Ok(())
}
