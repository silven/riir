extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/Hask.hs");

    println!("cargo:rustc-link-search=native=dist");
    println!("cargo:rustc-link-lib=dylib=haskell2");

    let compilation = Command::new("cabal")
        .arg("build")
        .status()
        .unwrap();

    assert!(compilation.success());

    let bindings = bindgen::Builder::default()
        .clang_args(&["-I", "/usr/lib/ghc/include"])
        .header("dist/build/Hask_stub.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    bindings.write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!"); 
}