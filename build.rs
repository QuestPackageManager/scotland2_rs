use std::env;
use std::path::PathBuf;

// without lib|.so prefix/suffix
const SCOTLAND2_LIB_NAME: &str = "sl2.debug";

const SCOTLAND2_PACKAGE_NAME: &str = "scotland2";
const SCOTLAND2_HEADER: &str = "modloader.h";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=qpm.json");

    // invoke qpm to download the package
    // and its dependencies

    let qpm_path = PathBuf::from(env::var("QPM_PATH").unwrap_or_else(|_| "qpm".into()));

    // call restore
    eprintln!("Downloading sl2 from qpm");
    let manifest_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let mut cmd = std::process::Command::new(qpm_path);
    cmd.current_dir(&manifest_path)
        .arg("restore")
        // .arg("--quiet")
        .status()
        .map_err(|e| format!("Failed to run qpm: {}", e))?;

    let libs_path = manifest_path.join("extern").join("libs");
    let headers_path = manifest_path.join("extern").join("includes");
    let bin_path = libs_path.join(SCOTLAND2_LIB_NAME);

    // assert!(bin_path.with_extension("so").exists(), "lib_path does not exist: {}", bin_path.display());

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
        .clang_arg("-includestdint.h")
        .clang_arg("-includestdbool.h")
        .header(header_path.to_str().unwrap())
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    // }

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", libs_path.display());

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib={SCOTLAND2_LIB_NAME}");

    Ok(())
}
