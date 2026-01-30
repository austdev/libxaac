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

if(WIN32)
    target_compile_definitions(xaacenc PRIVATE
       WIN32  _CRT_SECURE_NO_WARNINGS)
endif()

target_compile_definitions(xaacenc PRIVATE
    _X86_ LOUDNESS_LEVELING_SUPPORT) 

target_compile_options(xaacenc PRIVATE
    $<$<C_COMPILER_ID:MSVC>:/Wall>
    $<$<NOT:$<C_COMPILER_ID:MSVC>>: -O3;-Wall;-Wsequence-point;-Wunused-function>)
