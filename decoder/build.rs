use std::path::PathBuf;

fn main() {
    if cfg!(feature = "fallback") {
        println!("cargo::rerun-if-env-changed=CMAKE_TOOLCHAIN_FILE");

        let mut cmake_cfg = cmake::Config::new("..");

        cmake_cfg.build_target("libxaacdec");
        cmake_cfg.define("RC_FALLBACK", "ON");

        if let Some(toolchain_file) = option_env!("CMAKE_TOOLCHAIN_FILE") {
            cmake_cfg.define("CMAKE_TOOLCHAIN_FILE", toolchain_file);
        }

        if cfg!(target_os = "linux")
            && std::process::Command::new("ninja")
                .arg("--version")
                .spawn()
                .is_ok()
        {
            cmake_cfg.generator("Ninja");
        }

        let out_path = cmake_cfg.build();
        println!(
            "cargo::rustc-link-search=native={}/build",
            out_path.display()
        );
        println!("cargo::rustc-link-lib=static=xaacdec-ref");
    }
    //println!("cargo:rerun-if-changed=ixheaacd_vec_baisc_ops.h");
    let compiler_options = vec!["-DLOUDNESS_LEVELING_SUPPORT", "-I../common", "-I../drc_src"];

    let ixheaacd_headers = vec!["ixheaacd_vec_baisc_ops.h", "ixheaacd_main.h"];

    let bindings = bindgen::Builder::default()
        .headers(ixheaacd_headers)
        .clang_args(compiler_options)
        .raw_line("#![allow(non_camel_case_types, non_snake_case, unused, non_upper_case_globals)]")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("gen_ixheaacd_ref.rs"))
        .expect("Couldn't write bindings!");
}
