
use std::path::PathBuf;

fn main() {
    if cfg!(feature = "fallback") {
        println!("cargo:rustc-link-search=native=build");
        println!("cargo:rustc-link-lib=static=libxaacdec-ref");
    }
    //println!("cargo:rerun-if-changed=ixheaacd_vec_baisc_ops.h");
    let  compiler_options = vec!(
        "-DLOUDNESS_LEVELING_SUPPORT",
        "-I../common",
        "-I../drc_src",
    );

    let  ixheaacd_headers = vec!(
        "ixheaacd_vec_baisc_ops.h",
        "ixheaacd_main.h",
    );

    let bindings = bindgen::Builder::default()
        .headers(ixheaacd_headers)
        .clang_args(compiler_options)
        .raw_line("#![allow(non_camel_case_types, non_snake_case, unused)]")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("gen_ixheaacd_ref.rs"))
        .expect("Couldn't write bindings!");

}