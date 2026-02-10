list(APPEND XAACENC_SRCS
     "${XAAC_ROOT}/test/encoder/impd_drc_user_config.c"
     "${XAAC_ROOT}/test/encoder/ixheaace_error.c"
     "${XAAC_ROOT}/test/encoder/ixheaace_testbench.c")

set(LIBXAACENC_INCLUDES ${XAAC_ROOT}/encoder
                        ${XAAC_ROOT}/test/encoder
                        ${XAAC_ROOT}/encoder/drc_src
                        ${XAAC_ROOT}/common)

include_directories(${LIBXAACENC_INCLUDES})

libxaac_add_executable(xaacenc libxaacenc SOURCES ${XAACENC_SRCS} INCLUDES 
                       ${LIBXAACENC_INCLUDES})

target_compile_definitions(xaacenc PRIVATE
    _X86_ LOUDNESS_LEVELING_SUPPORT
    $<$<OR:$<C_COMPILER_ID:MSVC>,$<STREQUAL:${CMAKE_C_SIMULATE_ID},MSVC>>:_CRT_SECURE_NO_WARNINGS>)

target_compile_options(xaacenc PRIVATE
    $<$<C_COMPILER_FRONTEND_VARIANT:MSVC>:/Wall>
    $<$<NOT:$<C_COMPILER_FRONTEND_VARIANT:MSVC>>: -O3;-Wall;-Wsequence-point;-Wunused-function>)
