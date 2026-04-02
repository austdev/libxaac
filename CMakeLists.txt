cmake_minimum_required(VERSION 3.26)
project(libxaac C CXX)
enable_language(ASM)

option(LEGACY_BUILD "Reference build based on the original source code for verification during migration to Rust.")
set(LEGACY_BUILD_SUFFIX "-legacy") # for the output files

# Unify output layout for all generators
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR})
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR})
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR})
foreach(cfg IN ITEMS Debug Release RelWithDebInfo MinSizeRel)
  string(TOUPPER ${cfg} CFGU)
  set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_${CFGU} ${CMAKE_CURRENT_BINARY_DIR})
  set(CMAKE_LIBRARY_OUTPUT_DIRECTORY_${CFGU} ${CMAKE_CURRENT_BINARY_DIR})
  set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY_${CFGU} ${CMAKE_CURRENT_BINARY_DIR})
endforeach()

option(BUILD64 "Build for 64 bit" OFF)
set(XAAC_ROOT "${CMAKE_CURRENT_SOURCE_DIR}")
set(XAAC_CONFIG_DIR "${CMAKE_CURRENT_BINARY_DIR}")
find_package(Threads REQUIRED)

include("${XAAC_ROOT}/cmake/utils.cmake")

# Determine Rust build profile based on CMake build type
set(RUST_PROFILE $<IF:$<CONFIG:Release>,release,debug>)
set(CARGO_CMD cargo build -F integration --target-dir "${CMAKE_CURRENT_BINARY_DIR}" $<$<CONFIG:Release>:--release>)

if(LEGACY_BUILD)
    message(STATUS "Reference build based on the original source code")
    add_definitions(-DLEGACY_BUILD)
endif()
libxaac_add_compile_options()
libxaac_add_definitions()

if(NOT COMPILER_HAS_SANITIZER)
  libxaac_set_link_libraries()
endif()

option(BUILD_FUZZER "Build fuzzer for encoder and decoder" OFF)

include("${XAAC_ROOT}/common/common.cmake")
include("${XAAC_ROOT}/decoder/libxaacdec.cmake")
include("${XAAC_ROOT}/test/decoder/xaacdec.cmake")
if (BUILD_FUZZER)
    include("${XAAC_ROOT}/fuzzer/xaac_dec_fuzzer.cmake")
endif()

include("${XAAC_ROOT}/encoder/libxaacenc.cmake")
include("${XAAC_ROOT}/test/encoder/xaacenc.cmake")
if (BUILD_FUZZER)
    include("${XAAC_ROOT}/fuzzer/xaac_enc_fuzzer.cmake")
endif()
