# set(FETCHCONTENT_BASE_DIR "${CMAKE_SOURCE_DIR}/.tools/CMakeFetchContent/" CACHE PATH "")
# include(FetchContent)
# FetchContent_Declare(
#     Corrosion
#     GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
#     GIT_TAG v0.6 # Optionally specify a commit hash, version tag or branch here
# )
# set(Rust_TOOLCHAIN "nightly-2025-12-15") # must precede the line below
# FetchContent_MakeAvailable(Corrosion)

# #message(FATAL_ERROR "${CMAKE_CURRENT_LIST_DIR}")
# corrosion_import_crate(
#     MANIFEST_PATH   ${CMAKE_CURRENT_LIST_DIR}/../../rust/Cargo.toml
# #    CRATE_TYPES     bin
#     IMPORTED_CRATES some_crates    # logging (optional)
# )
# message(STATUS "Rust crates imported: ${some_crates}")

list(APPEND XAACDEC_SRCS "${XAAC_ROOT}/test/decoder/ixheaacd_error.c"
     "${XAAC_ROOT}/test/decoder/ixheaacd_fileifc.c" "${XAAC_ROOT}/test/decoder/ixheaacd_main.c"
     "${XAAC_ROOT}/test/decoder/ixheaacd_metadata_read.c")

set(LIBXAACDEC_INCLUDES ${XAAC_ROOT}/decoder ${XAAC_ROOT}/test/decoder/
                        ${XAAC_ROOT}/decoder/drc_src)

include_directories(${LIBXAACDEC_INCLUDES})

libxaac_add_executable(xaacdec libxaacdec SOURCES ${XAACDEC_SRCS} INCLUDES
                       ${LIBXAACDEC_INCLUDES})

if(NOT LEGACY_BUILD)
    target_link_libraries(xaacdec librustdec)
    if(WIN32)
        target_link_libraries(xaacdec kernel32 ws2_32 ntdll userenv dbghelp)
    elseif(APPLE)
        target_link_libraries(xaacdec pthread)
        set_target_properties(xaacdec PROPERTIES
            LINK_FLAGS
            "-framework Security -framework CoreFoundation")
    elseif(UNIX)
        target_link_libraries(xaacdec pthread dl m)
    endif()
endif()

target_compile_definitions(xaacdec PRIVATE
    DRC_ENABLE MULTICHANNEL_ENABLE ECLIPSE LOUDNESS_LEVELING_SUPPORT
    WIN32 # TODO: fix WIN32 diverging from _WIN32
    $<$<PLATFORM_ID:Windows>:_CRT_SECURE_NO_WARNINGS>)

target_compile_options(xaacdec PRIVATE
    $<$<C_COMPILER_FRONTEND_VARIANT:MSVC>:/UARM_PROFILE_HW;/UARM_PROFILE_BOARD>
    $<$<NOT:$<C_COMPILER_FRONTEND_VARIANT:MSVC>>:-UARM_PROFILE_HW;-UARM_PROFILE_BOARD>)
