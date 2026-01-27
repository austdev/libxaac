use std::env;

fn main() {
    // Configuration type:
    let config = env::var("PROFILE").unwrap();
    let config = if config == "release" { "Release" } else { "Debug" };

    println!("cargo:rustc-link-search=native=../build/{}", config);
    println!("cargo:rustc-link-lib=dylib=libxaacdec-ref");

    // Make the binary find the .so at runtime without LD_LIBRARY_PATH:
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");

}
