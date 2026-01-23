cmake_minimum_required(VERSION 3.10)
project(libxaac C CXX)
enable_language(ASM)

option(RC_FALLBACK "Reference build based on the original source code for verification during migration to Rust." OFF)

option(BUILD64 "Build for 64 bit" OFF)
set(XAAC_ROOT "${CMAKE_CURRENT_SOURCE_DIR}")
set(XAAC_CONFIG_DIR "${CMAKE_CURRENT_BINARY_DIR}")
find_package(Threads REQUIRED)

set(CMAKE_STATIC_LIBRARY_PREFIX "")

include("${XAAC_ROOT}/cmake/utils.cmake")

# Determine Rust build profile based on CMake build type
if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(RUST_PROFILE "debug")
    set(CARGO_CMD cargo build -F integration --target-dir "${CMAKE_CURRENT_BINARY_DIR}")
else()
    set(RUST_PROFILE "release")
    set(CARGO_CMD cargo build -F integration --target-dir "${CMAKE_CURRENT_BINARY_DIR}" --release)
endif()

if(RC_FALLBACK)
    message(STATUS "Reference build based on the original source code")
    add_definitions(-DRC_FALLBACK)
endif()
libxaac_add_compile_options()
libxaac_add_definitions()

if(NOT COMPILER_HAS_SANITIZER)
  libxaac_set_link_libraries()
endif()

include("${XAAC_ROOT}/common/common.cmake")
include("${XAAC_ROOT}/decoder/libxaacdec.cmake")
include("${XAAC_ROOT}/test/decoder/xaacdec.cmake")
include("${XAAC_ROOT}/fuzzer/xaac_dec_fuzzer.cmake")

include("${XAAC_ROOT}/encoder/libxaacenc.cmake")
include("${XAAC_ROOT}/test/encoder/xaacenc.cmake")
include("${XAAC_ROOT}/fuzzer/xaac_enc_fuzzer.cmake")

