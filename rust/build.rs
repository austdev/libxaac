fn main() {
    let mut cmake_cfg =  cmake::Config::new("..");
    cmake_cfg.build_target("libxaacdec");
    if cfg!(target_os = "linux") && std::process::Command::new("ninja").arg("--version").spawn().is_ok() {
        cmake_cfg.generator("Ninja");
    }
    let out_path = cmake_cfg.build();
    println!("cargo:warning={}", out_path.display());
    println!("cargo:rustc-link-search=native={}/build", out_path.display());
    println!("cargo:rustc-link-lib=static=xaacdec-ref");

    // Link definitions of `printf` and other legacy functions that are not in the msvcrt
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        println!("cargo:rustc-link-lib=legacy_stdio_definitions");
    }
}
