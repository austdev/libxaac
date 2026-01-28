list(APPEND XAACDEC_SRCS "${XAAC_ROOT}/test/decoder/ixheaacd_error.c"
     "${XAAC_ROOT}/test/decoder/ixheaacd_fileifc.c" "${XAAC_ROOT}/test/decoder/ixheaacd_main.c"
     "${XAAC_ROOT}/test/decoder/ixheaacd_metadata_read.c")

set(LIBXAACDEC_INCLUDES ${XAAC_ROOT}/decoder ${XAAC_ROOT}/test/decoder/
                        ${XAAC_ROOT}/decoder/drc_src)

include_directories(${LIBXAACDEC_INCLUDES})

libxaac_add_executable(xaacdec libxaacdec SOURCES ${XAACDEC_SRCS} INCLUDES
                       ${LIBXAACDEC_INCLUDES})

if(NOT RC_FALLBACK)
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

if(WIN32)
    target_compile_definitions(xaacdec PRIVATE
       WIN32  _CRT_SECURE_NO_WARNINGS)
endif()

target_compile_definitions(xaacdec PRIVATE
    DRC_ENABLE MULTICHANNEL_ENABLE ECLIPSE LOUDNESS_LEVELING_SUPPORT
    WIN32) # TODO: fix WIN32 diverging from _WIN32

target_compile_options(xaacdec PRIVATE
    $<$<C_COMPILER_ID:MSVC>:/UARM_PROFILE_HW;/UARM_PROFILE_BOARD>
    $<$<NOT:$<C_COMPILER_ID:MSVC>>:-UARM_PROFILE_HW;-UARM_PROFILE_BOARD>)
