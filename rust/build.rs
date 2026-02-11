fn main() {
    if cfg!(feature = "legacy-build") {
        println!("cargo::rerun-if-env-changed=CMAKE_TOOLCHAIN_FILE");

        let mut cmake_cfg = cmake::Config::new("..");

        cmake_cfg.build_target("libxaacdec");
        cmake_cfg.define("LEGACY_BUILD", "ON");

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
        println!("cargo::rustc-link-lib=static=xaacdec-legacy");
    } else {
        todo!("Linking to Rust lib not yet implemented");
    }

    // // Make the binary find the .so at runtime without LD_LIBRARY_PATH:
    // #[cfg(target_os = "linux")]
    // println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");

    // // Make the binary find the .dylib at runtime without DYLD_LIBRARY_PATH:
    // #[cfg(target_os = "macos")]
    // println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path");

    // Link definitions of `printf` and other legacy functions that are not in the msvcrt
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        println!("cargo::rustc-link-lib=legacy_stdio_definitions");
    }
}
