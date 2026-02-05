use std::env;

fn main() {
    // Configuration type:
    let config = env::var("PROFILE").unwrap();
    let config = if config == "release" { "Release" } else { "Debug" };

    println!("cargo:rustc-link-search=native=../build/{}", config);
    println!("cargo:rustc-link-lib=dylib=xaacdec-ref");

    // Make the binary find the .so at runtime without LD_LIBRARY_PATH:
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");

    // Make the binary find the .dylib at runtime without DYLD_LIBRARY_PATH:
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path");

    // Link definitions of `printf` and other legacy functions that are not in the msvcrt
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        println!("cargo:rustc-link-lib=legacy_stdio_definitions");
    }

}
