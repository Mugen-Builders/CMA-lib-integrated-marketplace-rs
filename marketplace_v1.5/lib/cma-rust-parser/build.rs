use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let include_dir = manifest_dir.join("lib/cpp-build/include").canonicalize().unwrap();

    let bindings = bindgen::Builder::default()
        .header(manifest_dir.join("wrapper.h").to_str().unwrap())
        .clang_arg(format!("-I{}", include_dir.display()))
        .allowlist_function("cma_.*")
        .allowlist_type("cma_.*")
        .allowlist_type("cmt_.*")
        .allowlist_var("CMT_.*")
        .allowlist_var("CMA_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("bindgen failed");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Failed to write bindings");

    // Only link the C++ library when not using native mocks
    if !cfg!(feature = "native") {
        let lib_dir = manifest_dir.join("lib/cpp-build/lib").canonicalize().unwrap();
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
        println!("cargo:rustc-link-lib=static=cma");
    }

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=lib/cpp-build/include/");
}
