#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]

mod ixheaacd_metadata_read;
mod ixheaacd_error;
mod ixheaacd_fileifc;

/// Retrurns C-compatible stderr handle
#[inline(always)]
fn get_stderr_fd() -> *mut FILE {
    #[cfg(not(target_env = "msvc"))]
    return stderr;

    #[cfg(target_env = "msvc")]
    {
        use std::os::windows::io::AsRawHandle;
        let handle = std::io::stderr().as_raw_handle();
        return handle as *mut FILE;
    }
}

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memmove(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn strcpy(
        __dest: *mut core::ffi::c_char,
        __src: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_char;
    fn strcat(
        __dest: *mut core::ffi::c_char,
        __src: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_char;
    fn strcmp(
        __s1: *const core::ffi::c_char,
        __s2: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn strncmp(
        __s1: *const core::ffi::c_char,
        __s2: *const core::ffi::c_char,
        __n: size_t,
    ) -> core::ffi::c_int;
    fn strlen(__s: *const core::ffi::c_char) -> size_t;
    fn strtod(
        __nptr: *const core::ffi::c_char,
        __endptr: *mut *mut core::ffi::c_char,
    ) -> core::ffi::c_double;
    fn strtol(
        __nptr: *const core::ffi::c_char,
        __endptr: *mut *mut core::ffi::c_char,
        __base: core::ffi::c_int,
    ) -> core::ffi::c_long;
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn exit(__status: core::ffi::c_int) -> !;
    #[cfg(not(target_env = "msvc"))]
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> core::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> core::ffi::c_int;
    fn fopen(
        __filename: *const core::ffi::c_char,
        __modes: *const core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn printf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn putc(__c: core::ffi::c_int, __stream: *mut FILE) -> core::ffi::c_int;
    fn fwrite(
        __ptr: *const core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> core::ffi::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: core::ffi::c_long,
        __whence: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn ferror(__stream: *mut FILE) -> core::ffi::c_int;
    fn FileWrapper_Open(fileName: *mut core::ffi::c_char) -> FileWrapperPtr;
    fn FileWrapper_Read(
        transport: FileWrapperPtr,
        buffer: *mut core::ffi::c_uchar,
        bufSize: core::ffi::c_int,
        len: *mut core::ffi::c_uint,
    ) -> core::ffi::c_int;
    fn FileWrapper_Close(transport: FileWrapperPtr) -> core::ffi::c_uint;
    fn ixheaacd_error_handler(
        p_mod_err_info: *mut ia_error_info_struct,
        pb_context: *mut WORD8,
        code: IA_ERRORCODE,
    ) -> IA_ERRORCODE;
    fn ixheaacd_get_lib_id_strings(pv_output: pVOID) -> VOID;
    fn ixheaacd_read_metadata_info(
        fp: *mut FILE,
        meta_info_0: *mut metadata_info,
    ) -> core::ffi::c_int;
    fn get_metadata_dec_info_init(meta_info_0: metadata_info) -> core::ffi::c_int;
    fn get_metadata_dec_exec(
        meta_info_0: metadata_info,
        frame: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn get_start_offset_in_samples(meta_info_0: metadata_info) -> core::ffi::c_int;
    fn get_play_time_in_samples(meta_info_0: metadata_info) -> core::ffi::c_int;
    fn ixheaacd_dec_api(
        p_ia_module_obj: pVOID,
        i_cmd: WORD32,
        i_idx: WORD32,
        pv_value: pVOID,
    ) -> IA_ERRORCODE;
    fn ia_drc_dec_api(
        p_ia_module_obj: pVOID,
        i_cmd: WORD32,
        i_idx: WORD32,
        pv_value: pVOID,
    ) -> IA_ERRORCODE;
    fn ixheaacd_error_handler_init() -> VOID;
    fn ia_testbench_error_handler_init() -> VOID;
    fn metadata_mp4_stsz_size_free(meta_info_0: *mut metadata_info) -> VOID;
    static mut ixheaacd_ia_testbench_error_info: ia_error_info_struct;
    static mut ixheaacd_error_info: ia_error_info_struct;
}
pub type size_t = usize;
pub type __off_t = core::ffi::c_long;
pub type __off64_t = core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: core::ffi::c_int,
    pub _IO_read_ptr: *mut core::ffi::c_char,
    pub _IO_read_end: *mut core::ffi::c_char,
    pub _IO_read_base: *mut core::ffi::c_char,
    pub _IO_write_base: *mut core::ffi::c_char,
    pub _IO_write_ptr: *mut core::ffi::c_char,
    pub _IO_write_end: *mut core::ffi::c_char,
    pub _IO_buf_base: *mut core::ffi::c_char,
    pub _IO_buf_end: *mut core::ffi::c_char,
    pub _IO_save_base: *mut core::ffi::c_char,
    pub _IO_backup_base: *mut core::ffi::c_char,
    pub _IO_save_end: *mut core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: core::ffi::c_int,
    pub _flags2: core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: core::ffi::c_ushort,
    pub _vtable_offset: core::ffi::c_schar,
    pub _shortbuf: [core::ffi::c_char; 1],
    pub _lock: *mut core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: core::ffi::c_int,
    pub _unused2: [core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FileWrapper {
    pub isMp4File: core::ffi::c_uint,
    pub inputFile: *mut FILE,
}
pub type FileWrapperPtr = *mut FileWrapper;
pub type pCHAR8 = *mut core::ffi::c_char;
pub type WORD8 = core::ffi::c_schar;
pub type pWORD8 = *mut core::ffi::c_schar;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD32 = core::ffi::c_int;
pub type pWORD32 = *mut core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type pUWORD32 = *mut core::ffi::c_uint;
pub type FLOAT32 = core::ffi::c_float;
pub type VOID = ();
pub type pVOID = *mut core::ffi::c_void;
pub type LOOPIDX = core::ffi::c_int;
pub type WORD = core::ffi::c_int;
pub type IA_ERRORCODE = WORD32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_error_info_struct {
    pub pb_module_name: pWORD8,
    pub ppb_class_names: [pWORD8; 16],
    pub ppppb_error_msg_pointers: [[*mut *mut WORD8; 16]; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_lib_info_struct {
    pub p_lib_name: *mut WORD8,
    pub p_version_num: *mut WORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct metadata_info {
    pub ia_mp4_stsz_size: *mut UWORD32,
    pub ia_mp4_stsz_entries: UWORD32,
    pub fill_once: UWORD32,
    pub movie_time_scale: UWORD32,
    pub media_time_scale: UWORD32,
    pub dec_info_init: UWORD32,
    pub g_track_count: UWORD32,
    pub useEditlist: [UWORD32; 50],
    pub startOffsetInSamples: [UWORD32; 50],
    pub playTimeInSamples: [UWORD32; 50],
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const NULL_0: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const core::ffi::c_char) -> core::ffi::c_int {
    return strtol(__nptr, NULL as *mut *mut core::ffi::c_char, 10 as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const core::ffi::c_char) -> core::ffi::c_double {
    return strtod(__nptr, NULL as *mut *mut core::ffi::c_char);
}
pub const IA_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_FATAL_ERROR: core::ffi::c_uint = 0x80000000 as core::ffi::c_uint;
pub const SEEK_SET: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_API_CMD_GET_LIB_ID_STRINGS: core::ffi::c_int = 0x1 as core::ffi::c_int;
pub const IA_API_CMD_GET_API_SIZE: core::ffi::c_int = 0x2 as core::ffi::c_int;
pub const IA_API_CMD_INIT: core::ffi::c_int = 0x3 as core::ffi::c_int;
pub const IA_API_CMD_SET_CONFIG_PARAM: core::ffi::c_int = 0x4 as core::ffi::c_int;
pub const IA_API_CMD_GET_CONFIG_PARAM: core::ffi::c_int = 0x5 as core::ffi::c_int;
pub const IA_API_CMD_GET_MEMTABS_SIZE: core::ffi::c_int = 0x6 as core::ffi::c_int;
pub const IA_API_CMD_SET_MEMTABS_PTR: core::ffi::c_int = 0x7 as core::ffi::c_int;
pub const IA_API_CMD_GET_N_MEMTABS: core::ffi::c_int = 0x8 as core::ffi::c_int;
pub const IA_API_CMD_EXECUTE: core::ffi::c_int = 0x9 as core::ffi::c_int;
pub const IA_API_CMD_GET_CURIDX_INPUT_BUF: core::ffi::c_int = 0xb as core::ffi::c_int;
pub const IA_API_CMD_SET_INPUT_BYTES: core::ffi::c_int = 0xc as core::ffi::c_int;
pub const IA_API_CMD_GET_OUTPUT_BYTES: core::ffi::c_int = 0xd as core::ffi::c_int;
pub const IA_API_CMD_INPUT_OVER: core::ffi::c_int = 0xe as core::ffi::c_int;
pub const IA_API_CMD_GET_MEM_INFO_SIZE: core::ffi::c_int = 0x11 as core::ffi::c_int;
pub const IA_API_CMD_GET_MEM_INFO_ALIGNMENT: core::ffi::c_int = 0x12 as core::ffi::c_int;
pub const IA_API_CMD_GET_MEM_INFO_TYPE: core::ffi::c_int = 0x13 as core::ffi::c_int;
pub const IA_API_CMD_SET_MEM_PTR: core::ffi::c_int = 0x16 as core::ffi::c_int;
pub const IA_CMD_TYPE_LIB_NAME: core::ffi::c_int = 0x100 as core::ffi::c_int;
pub const IA_CMD_TYPE_LIB_VERSION: core::ffi::c_int = 0x200 as core::ffi::c_int;
pub const IA_CMD_TYPE_INIT_API_PRE_CONFIG_PARAMS: core::ffi::c_int = 0x100
    as core::ffi::c_int;
pub const IA_CMD_TYPE_INIT_API_POST_CONFIG_PARAMS: core::ffi::c_int = 0x200
    as core::ffi::c_int;
pub const IA_CMD_TYPE_INIT_PROCESS: core::ffi::c_int = 0x300 as core::ffi::c_int;
pub const IA_CMD_TYPE_INIT_DONE_QUERY: core::ffi::c_int = 0x400 as core::ffi::c_int;
pub const IA_CMD_TYPE_DO_EXECUTE: core::ffi::c_int = 0x100 as core::ffi::c_int;
pub const IA_CMD_TYPE_DONE_QUERY: core::ffi::c_int = 0x200 as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_PCM_WDSZ: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_SAMP_FREQ: core::ffi::c_int = 0x1
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_NUM_CHANNELS: core::ffi::c_int = 0x2
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_CHANNEL_MASK: core::ffi::c_int = 0x3
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_CHANNEL_MODE: core::ffi::c_int = 0x4
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_SBR_MODE: core::ffi::c_int = 0x5
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_DRC_EFFECT_TYPE: core::ffi::c_int = 0x6
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_DRC_TARGET_LOUDNESS: core::ffi::c_int = 0x7
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_DRC_LOUD_NORM: core::ffi::c_int = 0x8
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_DOWNMIX: core::ffi::c_int = 0x9 as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_TOSTEREO: core::ffi::c_int = 0xa
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_DSAMPLE: core::ffi::c_int = 0xb as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_MP4FLAG: core::ffi::c_int = 0xc as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_MAX_CHANNEL: core::ffi::c_int = 0xd
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_COUP_CHANNEL: core::ffi::c_int = 0xe
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_DOWNMIX_STEREO: core::ffi::c_int = 0xf
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_DISABLE_SYNC: core::ffi::c_int = 0x10 as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_AUTO_SBR_UPSAMPLE: core::ffi::c_int = 0x11
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_DRC_CUT: core::ffi::c_int = 0x12
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_DRC_BOOST: core::ffi::c_int = 0x13
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_DRC_TARGET_LEVEL: core::ffi::c_int = 0x14
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_DRC_HEAVY_COMP: core::ffi::c_int = 0x15
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_FRAMESIZE: core::ffi::c_int = 0x16
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_LD_TESTING: core::ffi::c_int = 0x17
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_HQ_ESBR: core::ffi::c_int = 0x18
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_PS_ENABLE: core::ffi::c_int = 0x19
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_AOT: core::ffi::c_int = 0x1a as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_PEAK_LIMITER: core::ffi::c_int = 0x1b
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_FRAMELENGTH_FLAG: core::ffi::c_int = 0x1c
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_ERROR_CONCEALMENT: core::ffi::c_int = 0x1d
    as core::ffi::c_int;
pub const IA_ENHAACPLUS_DEC_CONFIG_EXT_ELE_PTR: core::ffi::c_int = 0x1e
    as core::ffi::c_int;
pub const IA_ENHAACPLUS_DEC_CONFIG_EXT_ELE_BUF_SIZES: core::ffi::c_int = 0x1f
    as core::ffi::c_int;
pub const IA_ENHAACPLUS_DEC_CONFIG_NUM_ELE: core::ffi::c_int = 0x20 as core::ffi::c_int;
pub const IA_ENHAACPLUS_DEC_CONFIG_NUM_CONFIG_EXT: core::ffi::c_int = 0x21
    as core::ffi::c_int;
pub const IA_ENHAACPLUS_DEC_CONFIG_GAIN_PAYLOAD_LEN: core::ffi::c_int = 0x22
    as core::ffi::c_int;
pub const IA_ENHAACPLUS_DEC_CONFIG_GAIN_PAYLOAD_BUF: core::ffi::c_int = 0x23
    as core::ffi::c_int;
pub const IA_ENHAACPLUS_DEC_CONFIG_GET_NUM_PRE_ROLL_FRAMES: core::ffi::c_int = 0x24
    as core::ffi::c_int;
pub const IA_ENHAACPLUS_DEC_DRC_IS_CONFIG_CHANGED: core::ffi::c_int = 0x25
    as core::ffi::c_int;
pub const IA_ENHAACPLUS_DEC_DRC_APPLY_CROSSFADE: core::ffi::c_int = 0x26
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_ESBR: core::ffi::c_int = 0x28 as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_DRC_LOUDNESS_LEVELING: core::ffi::c_int = 0x29
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_DRC_MODE_CUT: core::ffi::c_int = 0x2a
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_DRC_MODE_BOOST: core::ffi::c_int = 0x2b
    as core::ffi::c_int;
pub const IA_MEMTYPE_INPUT: core::ffi::c_int = 0x2 as core::ffi::c_int;
pub const IA_MEMTYPE_OUTPUT: core::ffi::c_int = 0x3 as core::ffi::c_int;
pub const IA_DRC_DEC_CONFIG_PARAM_PCM_WDSZ: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_DRC_DEC_CONFIG_PARAM_SAMP_FREQ: core::ffi::c_int = 0x1 as core::ffi::c_int;
pub const IA_DRC_DEC_CONFIG_PARAM_NUM_CHANNELS: core::ffi::c_int = 0x2
    as core::ffi::c_int;
pub const IA_DRC_DEC_CONFIG_PARAM_BITS_FORMAT: core::ffi::c_int = 0x7
    as core::ffi::c_int;
pub const IA_DRC_DEC_CONFIG_PARAM_INT_PRESENT: core::ffi::c_int = 0x8
    as core::ffi::c_int;
pub const IA_DRC_DEC_CONFIG_PARAM_FRAME_SIZE: core::ffi::c_int = 0xe as core::ffi::c_int;
pub const IA_DRC_DEC_CONFIG_GAIN_STREAM_FLAG: core::ffi::c_int = 0x10
    as core::ffi::c_int;
pub const IA_DRC_DEC_CONFIG_DRC_EFFECT_TYPE: core::ffi::c_int = 0x11 as core::ffi::c_int;
pub const IA_DRC_DEC_CONFIG_DRC_TARGET_LOUDNESS: core::ffi::c_int = 0x12
    as core::ffi::c_int;
pub const IA_DRC_DEC_CONFIG_DRC_LOUD_NORM: core::ffi::c_int = 0x13 as core::ffi::c_int;
pub const IA_DRC_DEC_CONFIG_DRC_BOOST: core::ffi::c_int = 0x15 as core::ffi::c_int;
pub const IA_DRC_DEC_CONFIG_DRC_CUT: core::ffi::c_int = 0x16 as core::ffi::c_int;
pub const IA_DRC_DEC_CONFIG_PARAM_APPLY_CROSSFADE: core::ffi::c_int = 0x17
    as core::ffi::c_int;
pub const IA_DRC_DEC_CONFIG_PARAM_CONFIG_CHANGED: core::ffi::c_int = 0x18
    as core::ffi::c_int;
pub const IA_DRC_DEC_CONFIG_DRC_LOUDNESS_LEVELING: core::ffi::c_int = 0x19
    as core::ffi::c_int;
pub const IA_API_CMD_SET_INPUT_BYTES_BS: core::ffi::c_int = 0x26 as core::ffi::c_int;
pub const IA_API_CMD_SET_INPUT_BYTES_IC_BS: core::ffi::c_int = 0x27 as core::ffi::c_int;
pub const IA_API_CMD_SET_INPUT_BYTES_IL_BS: core::ffi::c_int = 0x29 as core::ffi::c_int;
pub const IA_CMD_TYPE_INIT_CPY_BSF_BUFF: core::ffi::c_int = 0x201 as core::ffi::c_int;
pub const IA_CMD_TYPE_INIT_CPY_IC_BSF_BUFF: core::ffi::c_int = 0x202 as core::ffi::c_int;
pub const IA_CMD_TYPE_INIT_CPY_IL_BSF_BUFF: core::ffi::c_int = 0x203 as core::ffi::c_int;
pub const IA_CMD_TYPE_INIT_CPY_IN_BSF_BUFF: core::ffi::c_int = 0x205 as core::ffi::c_int;
pub const IA_CMD_TYPE_INIT_SET_BUFF_PTR: core::ffi::c_int = 0x20b as core::ffi::c_int;
pub const IA_SCREEN_WIDTH: core::ffi::c_int = 80 as core::ffi::c_int;
pub const IA_TESTBENCH_MFMAN_FATAL_MEM_ALLOC_FAILED: core::ffi::c_uint = 0xffff8000
    as core::ffi::c_uint;
pub const IA_TESTBENCH_MFMAN_FATAL_FILE_OPEN_FAILED: core::ffi::c_uint = 0xffff8001
    as core::ffi::c_uint;
#[no_mangle]
pub static mut g_pv_arr_alloc_memory: [pVOID; 100] = [0 as *const core::ffi::c_void
    as *mut core::ffi::c_void; 100];
#[no_mangle]
pub static mut g_w_malloc_count: WORD = 0;
#[no_mangle]
pub static mut g_pf_out: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut g_pf_inp: FileWrapperPtr = 0 as *const FileWrapper as *mut FileWrapper;
#[no_mangle]
pub static mut mpeg_d_drc_on: WORD32 = 0 as WORD32;
#[no_mangle]
pub static mut meta_info: metadata_info = metadata_info {
    ia_mp4_stsz_size: 0 as *mut UWORD32,
    ia_mp4_stsz_entries: 0,
    fill_once: 0,
    movie_time_scale: 0,
    media_time_scale: 0,
    dec_info_init: 0,
    g_track_count: 0,
    useEditlist: [0; 50],
    startOffsetInSamples: [0; 50],
    playTimeInSamples: [0; 50],
};
#[no_mangle]
pub static mut ixheaacd_i_bytes_to_read: WORD32 = 0;
#[no_mangle]
pub static mut prev_i_bytes_to_read: WORD32 = 0;
#[no_mangle]
pub static mut flush_frame: WORD32 = 0 as WORD32;
#[no_mangle]
pub static mut g_pf_meta: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut raw_testing: WORD32 = 0 as WORD32;
#[no_mangle]
pub static mut eld_testing: WORD32 = 0 as WORD32;
#[no_mangle]
pub static mut ec_enable: WORD32 = 0 as WORD32;
#[no_mangle]
pub static mut esbr_testing: WORD32 = 1 as WORD32;
unsafe extern "C" fn ixheaacd_write16_bits_lh(mut fp: *mut FILE, mut i: WORD32) -> VOID {
    putc(i as core::ffi::c_int & 0xff as core::ffi::c_int, fp);
    putc(i as core::ffi::c_int >> 8 as core::ffi::c_int & 0xff as core::ffi::c_int, fp);
}
unsafe extern "C" fn ixheaacd_write32_bits_lh(mut fp: *mut FILE, mut i: WORD32) -> VOID {
    ixheaacd_write16_bits_lh(
        fp,
        (i as core::ffi::c_long & 0xffff as core::ffi::c_long) as WORD32,
    );
    ixheaacd_write16_bits_lh(
        fp,
        ((i >> 16 as core::ffi::c_int) as core::ffi::c_long
            & 0xffff as core::ffi::c_long) as WORD32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn write_wav_header(
    mut fp: *mut FILE,
    mut pcmbytes: WORD32,
    mut freq: WORD32,
    mut channels: WORD32,
    mut bits: WORD32,
    mut i_channel_mask: WORD32,
) -> WORD32 {
    if channels > 2 as core::ffi::c_int {
        let mut bytes: WORD32 = (bits + 7 as WORD32) / 8 as WORD32;
        fwrite(
            b"RIFF\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            1 as size_t,
            4 as size_t,
            fp,
        );
        ixheaacd_write32_bits_lh(fp, pcmbytes + 44 as WORD32 - 8 as WORD32);
        fwrite(
            b"WAVEfmt \0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            2 as size_t,
            4 as size_t,
            fp,
        );
        if channels > 2 as core::ffi::c_int {
            ixheaacd_write16_bits_lh(fp, 0x28 as WORD32);
            ixheaacd_write16_bits_lh(fp, 0 as WORD32);
            ixheaacd_write16_bits_lh(fp, 0xfffe as WORD32);
        } else {
            ixheaacd_write32_bits_lh(
                fp,
                2 as WORD32 + 2 as WORD32 + 4 as WORD32 + 4 as WORD32 + 2 as WORD32
                    + 2 as WORD32,
            );
            ixheaacd_write16_bits_lh(fp, 1 as WORD32);
        }
        ixheaacd_write16_bits_lh(fp, channels);
        ixheaacd_write32_bits_lh(fp, freq);
        ixheaacd_write32_bits_lh(fp, freq * channels * bytes);
        ixheaacd_write16_bits_lh(fp, channels * bytes);
        ixheaacd_write16_bits_lh(fp, bits);
        if channels > 2 as core::ffi::c_int {
            ixheaacd_write16_bits_lh(fp, 0x16 as WORD32);
            ixheaacd_write16_bits_lh(fp, 0x10 as WORD32);
            ixheaacd_write32_bits_lh(fp, i_channel_mask);
            ixheaacd_write32_bits_lh(fp, 0x1 as WORD32);
            ixheaacd_write32_bits_lh(fp, 0x100000 as WORD32);
            ixheaacd_write16_bits_lh(fp, 0x80 as WORD32);
            ixheaacd_write16_bits_lh(fp, 0xaa00 as WORD32);
            ixheaacd_write16_bits_lh(fp, 0x3800 as WORD32);
            ixheaacd_write16_bits_lh(fp, 0x719b as WORD32);
        }
        fwrite(
            b"data\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            1 as size_t,
            4 as size_t,
            fp,
        );
        ixheaacd_write32_bits_lh(fp, pcmbytes);
        return if ferror(fp) != 0 { -(1 as WORD32) } else { 0 as WORD32 };
    } else {
        let mut bytes_0: WORD32 = (bits + 7 as WORD32) / 8 as WORD32;
        fwrite(
            b"RIFF\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            1 as size_t,
            4 as size_t,
            fp,
        );
        ixheaacd_write32_bits_lh(fp, pcmbytes + 44 as WORD32 - 8 as WORD32);
        fwrite(
            b"WAVEfmt \0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            2 as size_t,
            4 as size_t,
            fp,
        );
        ixheaacd_write32_bits_lh(
            fp,
            2 as WORD32 + 2 as WORD32 + 4 as WORD32 + 4 as WORD32 + 2 as WORD32
                + 2 as WORD32,
        );
        ixheaacd_write16_bits_lh(fp, 1 as WORD32);
        ixheaacd_write16_bits_lh(fp, channels);
        ixheaacd_write32_bits_lh(fp, freq);
        ixheaacd_write32_bits_lh(fp, freq * channels * bytes_0);
        ixheaacd_write16_bits_lh(fp, channels * bytes_0);
        ixheaacd_write16_bits_lh(fp, bits);
        fwrite(
            b"data\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            1 as size_t,
            4 as size_t,
            fp,
        );
        ixheaacd_write32_bits_lh(fp, pcmbytes);
        return if ferror(fp) != 0 { -(1 as WORD32) } else { 0 as WORD32 };
    };
}
#[no_mangle]
pub unsafe extern "C" fn ia_display_id_message(
    mut lib_name: *mut WORD8,
    mut lib_version: *mut WORD8,
) -> VOID {
    let mut str: [[WORD8; 80]; 4] = [
        ::core::mem::transmute::<
            [u8; 80],
            [WORD8; 80],
        >(
            *b"ITTIAM SYSTEMS PVT LTD, BANGALORE\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        ::core::mem::transmute::<
            [u8; 80],
            [WORD8; 80],
        >(
            *b"http:\\\\www.ittiam.com\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        ::core::mem::transmute::<
            [u8; 80],
            [WORD8; 80],
        >(
            *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        ::core::mem::transmute::<
            [u8; 80],
            [WORD8; 80],
        >(
            *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
    ];
    let mut spaces: [WORD8; 41] = [0; 41];
    let mut i: WORD32 = 0;
    let mut spclen: WORD32 = 0;
    strcpy(
        (str[2 as core::ffi::c_int as usize]).as_mut_ptr() as *mut core::ffi::c_char,
        lib_name as pCHAR8 as *const core::ffi::c_char,
    );
    strcat(
        (str[2 as core::ffi::c_int as usize]).as_mut_ptr() as *mut core::ffi::c_char,
        lib_version as pCHAR8 as *const core::ffi::c_char,
    );
    strcat(
        (str[2 as core::ffi::c_int as usize]).as_mut_ptr() as *mut core::ffi::c_char,
        b"\n\0" as *const u8 as *const core::ffi::c_char,
    );
    strcat(
        (str[(4 as core::ffi::c_int - 1 as core::ffi::c_int) as usize]).as_mut_ptr()
            as *mut core::ffi::c_char,
        b"\n\0" as *const u8 as *const core::ffi::c_char,
    );
    i = 0 as core::ffi::c_int as WORD32;
    while i < IA_SCREEN_WIDTH / 2 as core::ffi::c_int + 1 as core::ffi::c_int {
        spaces[i as usize] = ' ' as i32 as WORD8;
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < 4 as core::ffi::c_int {
        spclen = (IA_SCREEN_WIDTH / 2 as core::ffi::c_int
            - strlen(
                (str[i as usize]).as_mut_ptr() as pCHAR8 as *const core::ffi::c_char,
            ) as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
        spaces[spclen as usize] = '\0' as i32 as WORD8;
        printf(
            b"%s\0" as *const u8 as *const core::ffi::c_char,
            spaces.as_mut_ptr() as pCHAR8,
        );
        spaces[spclen as usize] = ' ' as i32 as WORD8;
        printf(
            b"%s\0" as *const u8 as *const core::ffi::c_char,
            (str[i as usize]).as_mut_ptr() as pCHAR8,
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_set_config_param(
    mut argc: WORD32,
    mut argv: *mut pWORD8,
    mut p_ia_process_api_obj: pVOID,
) -> IA_ERRORCODE {
    let mut i: LOOPIDX = 0;
    let mut err_code: IA_ERRORCODE = IA_NO_ERROR;
    let mut p_ia_process_api: Option<
        unsafe extern "C" fn(pVOID, WORD32, WORD32, pVOID) -> IA_ERRORCODE,
    > = Some(
        ixheaacd_dec_api
            as unsafe extern "C" fn(pVOID, WORD32, WORD32, pVOID) -> IA_ERRORCODE,
    );
    let p_proc_err_info: *mut ia_error_info_struct = &raw mut ixheaacd_error_info;
    i = 0 as core::ffi::c_int as LOOPIDX;
    while i < argc {
        if strncmp(
            *argv.offset(i as isize) as pCHAR8 as *const core::ffi::c_char,
            b"-mp4:\0" as *const u8 as *const core::ffi::c_char,
            5 as size_t,
        ) == 0
        {
            let mut pb_arg_val: pCHAR8 = (*argv.offset(i as isize))
                .offset(5 as core::ffi::c_int as isize) as pCHAR8;
            let mut ui_mp4_flag: UWORD32 = atoi(pb_arg_val as *const core::ffi::c_char)
                as UWORD32;
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                p_ia_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_XHEAAC_DEC_CONFIG_PARAM_MP4FLAG,
                &mut ui_mp4_flag as *mut UWORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code;
                }
            }
        }
        if strncmp(
            *argv.offset(i as isize) as pCHAR8 as *const core::ffi::c_char,
            b"-pcmsz:\0" as *const u8 as *const core::ffi::c_char,
            7 as size_t,
        ) == 0
        {
            let mut pb_arg_val_0: pCHAR8 = (*argv.offset(i as isize))
                .offset(7 as core::ffi::c_int as isize) as pCHAR8;
            let mut ui_pcm_wd_sz: UWORD32 = atoi(
                pb_arg_val_0 as *const core::ffi::c_char,
            ) as UWORD32;
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                p_ia_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_XHEAAC_DEC_CONFIG_PARAM_PCM_WDSZ,
                &mut ui_pcm_wd_sz as *mut UWORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code;
                }
            }
        }
        if strncmp(
            *argv.offset(i as isize) as pCHAR8 as *const core::ffi::c_char,
            b"-dmix:\0" as *const u8 as *const core::ffi::c_char,
            6 as size_t,
        ) == 0
        {
            let mut pb_arg_val_1: pCHAR8 = (*argv.offset(i as isize))
                .offset(6 as core::ffi::c_int as isize) as pCHAR8;
            let mut ui_down_mix: UWORD32 = atoi(pb_arg_val_1 as *const core::ffi::c_char)
                as UWORD32;
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                p_ia_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_XHEAAC_DEC_CONFIG_PARAM_DOWNMIX,
                &mut ui_down_mix as *mut UWORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code;
                }
            }
        }
        if strncmp(
            *argv.offset(i as isize) as pCHAR8 as *const core::ffi::c_char,
            b"-esbr_hq:\0" as *const u8 as *const core::ffi::c_char,
            9 as size_t,
        ) == 0
        {
            let mut pb_arg_val_2: pCHAR8 = (*argv.offset(i as isize))
                .offset(9 as core::ffi::c_int as isize) as pCHAR8;
            let mut ui_esbr_hq: UWORD32 = atoi(pb_arg_val_2 as *const core::ffi::c_char)
                as UWORD32;
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                p_ia_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_XHEAAC_DEC_CONFIG_PARAM_HQ_ESBR,
                &mut ui_esbr_hq as *mut UWORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code;
                }
            }
        }
        if strncmp(
            *argv.offset(i as isize) as pCHAR8 as *const core::ffi::c_char,
            b"-esbr_ps:\0" as *const u8 as *const core::ffi::c_char,
            9 as size_t,
        ) == 0
        {
            let mut pb_arg_val_3: pCHAR8 = (*argv.offset(i as isize))
                .offset(9 as core::ffi::c_int as isize) as pCHAR8;
            let mut ui_esbr_ps: UWORD32 = atoi(pb_arg_val_3 as *const core::ffi::c_char)
                as UWORD32;
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                p_ia_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_XHEAAC_DEC_CONFIG_PARAM_PS_ENABLE,
                &mut ui_esbr_ps as *mut UWORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code;
                }
            }
        }
        if strncmp(
            *argv.offset(i as isize) as pCHAR8 as *const core::ffi::c_char,
            b"-esbr:\0" as *const u8 as *const core::ffi::c_char,
            6 as size_t,
        ) == 0
        {
            let mut pb_arg_val_4: pCHAR8 = (*argv.offset(i as isize))
                .offset(6 as core::ffi::c_int as isize) as pCHAR8;
            let mut ui_esbr: UWORD32 = atoi(pb_arg_val_4 as *const core::ffi::c_char)
                as UWORD32;
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                p_ia_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_XHEAAC_DEC_CONFIG_PARAM_ESBR,
                &mut ui_esbr as *mut UWORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code;
                }
            }
            esbr_testing = ui_esbr as WORD32;
        }
        if strncmp(
            *argv.offset(i as isize) as pCHAR8 as *const core::ffi::c_char,
            b"-tostereo:\0" as *const u8 as *const core::ffi::c_char,
            10 as size_t,
        ) == 0
        {
            let mut pb_arg_val_5: pCHAR8 = (*argv.offset(i as isize))
                .offset(10 as core::ffi::c_int as isize) as pCHAR8;
            let mut ui_to_stereo: UWORD32 = atoi(
                pb_arg_val_5 as *const core::ffi::c_char,
            ) as UWORD32;
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                p_ia_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_XHEAAC_DEC_CONFIG_PARAM_TOSTEREO,
                &mut ui_to_stereo as *mut UWORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code;
                }
            }
        }
        if strncmp(
            *argv.offset(i as isize) as pCHAR8 as *const core::ffi::c_char,
            b"-dsample:\0" as *const u8 as *const core::ffi::c_char,
            9 as size_t,
        ) == 0
        {
            let mut pb_arg_val_6: pCHAR8 = (*argv.offset(i as isize))
                .offset(9 as core::ffi::c_int as isize) as pCHAR8;
            let mut ui_dsample: UWORD32 = atoi(pb_arg_val_6 as *const core::ffi::c_char)
                as UWORD32;
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                p_ia_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_XHEAAC_DEC_CONFIG_PARAM_DSAMPLE,
                &mut ui_dsample as *mut UWORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code;
                }
            }
        }
        if strncmp(
            *argv.offset(i as isize) as pCHAR8 as *const core::ffi::c_char,
            b"-drc_cut_fac:\0" as *const u8 as *const core::ffi::c_char,
            13 as size_t,
        ) == 0
        {
            let mut pb_arg_val_7: pCHAR8 = (*argv.offset(i as isize))
                .offset(13 as core::ffi::c_int as isize) as pCHAR8;
            let mut drc_cut: FLOAT32 = atof(pb_arg_val_7 as *const core::ffi::c_char)
                as FLOAT32;
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                p_ia_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_XHEAAC_DEC_CONFIG_PARAM_DRC_CUT,
                &mut drc_cut as *mut FLOAT32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code;
                }
            }
        }
        if strncmp(
            *argv.offset(i as isize) as pCHAR8 as *const core::ffi::c_char,
            b"-drc_boost_fac:\0" as *const u8 as *const core::ffi::c_char,
            15 as size_t,
        ) == 0
        {
            let mut pb_arg_val_8: pCHAR8 = (*argv.offset(i as isize))
                .offset(15 as core::ffi::c_int as isize) as pCHAR8;
            let mut drc_boost: FLOAT32 = atof(pb_arg_val_8 as *const core::ffi::c_char)
                as FLOAT32;
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                p_ia_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_XHEAAC_DEC_CONFIG_PARAM_DRC_BOOST,
                &mut drc_boost as *mut FLOAT32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code;
                }
            }
        }
        if strncmp(
            *argv.offset(i as isize) as pCHAR8 as *const core::ffi::c_char,
            b"-drc_target_level:\0" as *const u8 as *const core::ffi::c_char,
            18 as size_t,
        ) == 0
        {
            let mut pb_arg_val_9: pCHAR8 = (*argv.offset(i as isize))
                .offset(18 as core::ffi::c_int as isize) as pCHAR8;
            let mut ui_drc_target: UWORD32 = atoi(
                pb_arg_val_9 as *const core::ffi::c_char,
            ) as UWORD32;
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                p_ia_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_XHEAAC_DEC_CONFIG_PARAM_DRC_TARGET_LEVEL,
                &mut ui_drc_target as *mut UWORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code;
                }
            }
        }
        if strncmp(
            *argv.offset(i as isize) as pCHAR8 as *const core::ffi::c_char,
            b"-drc_heavy_comp:\0" as *const u8 as *const core::ffi::c_char,
            16 as size_t,
        ) == 0
        {
            let mut pb_arg_val_10: pCHAR8 = (*argv.offset(i as isize))
                .offset(16 as core::ffi::c_int as isize) as pCHAR8;
            let mut ui_drc_heavy_comp: UWORD32 = atoi(
                pb_arg_val_10 as *const core::ffi::c_char,
            ) as UWORD32;
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                p_ia_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_XHEAAC_DEC_CONFIG_PARAM_DRC_HEAVY_COMP,
                &mut ui_drc_heavy_comp as *mut UWORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code;
                }
            }
        }
        if strncmp(
            *argv.offset(i as isize) as pCHAR8 as *const core::ffi::c_char,
            b"-effect:\0" as *const u8 as *const core::ffi::c_char,
            8 as size_t,
        ) == 0
        {
            let mut pb_arg_val_11: pCHAR8 = (*argv.offset(i as isize))
                .offset(8 as core::ffi::c_int as isize) as pCHAR8;
            let mut ui_effect: WORD32 = atoi(pb_arg_val_11 as *const core::ffi::c_char)
                as WORD32;
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                p_ia_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_XHEAAC_DEC_CONFIG_PARAM_DRC_EFFECT_TYPE,
                &mut ui_effect as *mut WORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code;
                }
            }
            mpeg_d_drc_on = 1 as core::ffi::c_int as WORD32;
        }
        if strncmp(
            *argv.offset(i as isize) as pCHAR8 as *const core::ffi::c_char,
            b"-target_loudness:\0" as *const u8 as *const core::ffi::c_char,
            17 as size_t,
        ) == 0
        {
            let mut pb_arg_val_12: pCHAR8 = (*argv.offset(i as isize))
                .offset(17 as core::ffi::c_int as isize) as pCHAR8;
            let mut ui_target_loudness: WORD32 = atoi(
                pb_arg_val_12 as *const core::ffi::c_char,
            ) as WORD32;
            if ui_target_loudness > 0 as core::ffi::c_int
                || ui_target_loudness < -(63 as core::ffi::c_int)
            {
                ui_target_loudness = 0 as core::ffi::c_int as WORD32;
            }
            ui_target_loudness = -(ui_target_loudness << 2 as core::ffi::c_int);
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                p_ia_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_XHEAAC_DEC_CONFIG_PARAM_DRC_TARGET_LOUDNESS,
                &mut ui_target_loudness as *mut WORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code;
                }
            }
            mpeg_d_drc_on = 1 as core::ffi::c_int as WORD32;
        }
        if strncmp(
            *argv.offset(i as isize) as pCHAR8 as *const core::ffi::c_char,
            b"-nosync:\0" as *const u8 as *const core::ffi::c_char,
            8 as size_t,
        ) == 0
        {
            let mut pb_arg_val_13: pCHAR8 = (*argv.offset(i as isize))
                .offset(8 as core::ffi::c_int as isize) as pCHAR8;
            let mut ui_disable_sync: UWORD32 = atoi(
                pb_arg_val_13 as *const core::ffi::c_char,
            ) as UWORD32;
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                p_ia_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_XHEAAC_DEC_CONFIG_DISABLE_SYNC,
                &mut ui_disable_sync as *mut UWORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code;
                }
            }
        }
        if strncmp(
            *argv.offset(i as isize) as pCHAR8 as *const core::ffi::c_char,
            b"-sbrup:\0" as *const u8 as *const core::ffi::c_char,
            7 as size_t,
        ) == 0
        {
            let mut pb_arg_val_14: pCHAR8 = (*argv.offset(i as isize))
                .offset(7 as core::ffi::c_int as isize) as pCHAR8;
            let mut ui_auto_sbr_upsample: UWORD32 = atoi(
                pb_arg_val_14 as *const core::ffi::c_char,
            ) as UWORD32;
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                p_ia_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_XHEAAC_DEC_CONFIG_PARAM_AUTO_SBR_UPSAMPLE,
                &mut ui_auto_sbr_upsample as *mut UWORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code;
                }
            }
        }
        if strncmp(
            *argv.offset(i as isize) as pCHAR8 as *const core::ffi::c_char,
            b"-flflag:\0" as *const u8 as *const core::ffi::c_char,
            8 as size_t,
        ) == 0
        {
            let mut pb_arg_val_15: pCHAR8 = (*argv.offset(i as isize))
                .offset(8 as core::ffi::c_int as isize) as pCHAR8;
            let mut ui_fl_flag: UWORD32 = atoi(pb_arg_val_15 as *const core::ffi::c_char)
                as UWORD32;
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                p_ia_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_XHEAAC_DEC_CONFIG_PARAM_FRAMELENGTH_FLAG,
                &mut ui_fl_flag as *mut UWORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code;
                }
            }
        }
        if strncmp(
            *argv.offset(i as isize) as pCHAR8 as *const core::ffi::c_char,
            b"-fs:\0" as *const u8 as *const core::ffi::c_char,
            4 as size_t,
        ) == 0
        {
            let mut pb_arg_val_16: pCHAR8 = (*argv.offset(i as isize))
                .offset(4 as core::ffi::c_int as isize) as pCHAR8;
            let mut ui_samp_freq: UWORD32 = atoi(
                pb_arg_val_16 as *const core::ffi::c_char,
            ) as UWORD32;
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                p_ia_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_XHEAAC_DEC_CONFIG_PARAM_SAMP_FREQ,
                &mut ui_samp_freq as *mut UWORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code;
                }
            }
        }
        if strncmp(
            *argv.offset(i as isize) as pCHAR8 as *const core::ffi::c_char,
            b"-maxchannel:\0" as *const u8 as *const core::ffi::c_char,
            12 as size_t,
        ) == 0
        {
            let mut pb_arg_val_17: pCHAR8 = (*argv.offset(i as isize))
                .offset(12 as core::ffi::c_int as isize) as pCHAR8;
            let mut ui_max_channel: UWORD32 = atoi(
                pb_arg_val_17 as *const core::ffi::c_char,
            ) as UWORD32;
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                p_ia_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_XHEAAC_DEC_CONFIG_PARAM_MAX_CHANNEL,
                &mut ui_max_channel as *mut UWORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code;
                }
            }
        }
        if strncmp(
            *argv.offset(i as isize) as pCHAR8 as *const core::ffi::c_char,
            b"-coupchannel:\0" as *const u8 as *const core::ffi::c_char,
            13 as size_t,
        ) == 0
        {
            let mut pb_arg_val_18: pCHAR8 = (*argv.offset(i as isize))
                .offset(13 as core::ffi::c_int as isize) as pCHAR8;
            let mut ui_coupling_channel: UWORD32 = atoi(
                pb_arg_val_18 as *const core::ffi::c_char,
            ) as UWORD32;
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                p_ia_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_XHEAAC_DEC_CONFIG_PARAM_COUP_CHANNEL,
                &mut ui_coupling_channel as *mut UWORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code;
                }
            }
        }
        if strncmp(
            *argv.offset(i as isize) as pCHAR8 as *const core::ffi::c_char,
            b"-downmix:\0" as *const u8 as *const core::ffi::c_char,
            9 as size_t,
        ) == 0
        {
            let mut pb_arg_val_19: pCHAR8 = (*argv.offset(i as isize))
                .offset(9 as core::ffi::c_int as isize) as pCHAR8;
            let mut ui_downmix: UWORD32 = atoi(pb_arg_val_19 as *const core::ffi::c_char)
                as UWORD32;
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                p_ia_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_XHEAAC_DEC_CONFIG_PARAM_DOWNMIX_STEREO,
                &mut ui_downmix as *mut UWORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code;
                }
            }
        }
        if strncmp(
            *argv.offset(i as isize) as pCHAR8 as *const core::ffi::c_char,
            b"-fs480:\0" as *const u8 as *const core::ffi::c_char,
            7 as size_t,
        ) == 0
        {
            let mut pb_arg_val_20: pCHAR8 = (*argv.offset(i as isize))
                .offset(7 as core::ffi::c_int as isize) as pCHAR8;
            let mut ui_fs480: UWORD32 = atoi(pb_arg_val_20 as *const core::ffi::c_char)
                as UWORD32;
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                p_ia_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_XHEAAC_DEC_CONFIG_PARAM_FRAMESIZE,
                &mut ui_fs480 as *mut UWORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code;
                }
            }
        }
        if strncmp(
            *argv.offset(i as isize) as pCHAR8 as *const core::ffi::c_char,
            b"-ld_testing:\0" as *const u8 as *const core::ffi::c_char,
            12 as size_t,
        ) == 0
        {
            let mut pb_arg_val_21: pCHAR8 = (*argv.offset(i as isize))
                .offset(12 as core::ffi::c_int as isize) as pCHAR8;
            let mut ld_testing: UWORD32 = atoi(pb_arg_val_21 as *const core::ffi::c_char)
                as UWORD32;
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                p_ia_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_XHEAAC_DEC_CONFIG_PARAM_LD_TESTING,
                &mut ld_testing as *mut UWORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code;
                }
            }
        }
        if strncmp(
            *argv.offset(i as isize) as pCHAR8 as *const core::ffi::c_char,
            b"-peak_limiter_off:\0" as *const u8 as *const core::ffi::c_char,
            18 as size_t,
        ) == 0
        {
            let mut pb_arg_val_22: pCHAR8 = (*argv.offset(i as isize))
                .offset(18 as core::ffi::c_int as isize) as pCHAR8;
            let mut peak_limiter_flag: UWORD32 = atoi(
                pb_arg_val_22 as *const core::ffi::c_char,
            ) as UWORD32;
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                p_ia_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_XHEAAC_DEC_CONFIG_PARAM_PEAK_LIMITER,
                &mut peak_limiter_flag as *mut UWORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code;
                }
            }
        }
        if strncmp(
            *argv.offset(i as isize) as pCHAR8 as *const core::ffi::c_char,
            b"-err_conceal:\0" as *const u8 as *const core::ffi::c_char,
            13 as size_t,
        ) == 0
        {
            let mut pb_arg_val_23: pCHAR8 = (*argv.offset(i as isize))
                .offset(13 as core::ffi::c_int as isize) as pCHAR8;
            let mut ui_err_conceal: UWORD32 = atoi(
                pb_arg_val_23 as *const core::ffi::c_char,
            ) as UWORD32;
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                p_ia_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_XHEAAC_DEC_CONFIG_ERROR_CONCEALMENT,
                &mut ui_err_conceal as *mut UWORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code;
                }
            }
            ec_enable = ui_err_conceal as WORD32;
        }
        if strncmp(
            *argv.offset(i as isize) as pCHAR8 as *const core::ffi::c_char,
            b"-loudness_leveling:\0" as *const u8 as *const core::ffi::c_char,
            19 as size_t,
        ) == 0
        {
            let mut pb_arg_val_24: pCHAR8 = (*argv.offset(i as isize))
                .offset(19 as core::ffi::c_int as isize) as pCHAR8;
            let mut loudness_leveling_flag: UWORD32 = atoi(
                pb_arg_val_24 as *const core::ffi::c_char,
            ) as UWORD32;
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                p_ia_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_XHEAAC_DEC_CONFIG_PARAM_DRC_LOUDNESS_LEVELING,
                &mut loudness_leveling_flag as *mut UWORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code;
                }
            }
            mpeg_d_drc_on = 1 as core::ffi::c_int as WORD32;
        }
        i += 1;
    }
    return IA_NO_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_get_config_param(
    mut p_ia_process_api_obj: pVOID,
    mut pi_samp_freq: pWORD32,
    mut pi_num_chan: pWORD32,
    mut pi_pcm_wd_sz: pWORD32,
    mut pi_channel_mask: pWORD32,
    mut pi_sbr_mode: pWORD32,
    mut pi_aot: pWORD32,
) -> IA_ERRORCODE {
    let mut err_code: IA_ERRORCODE = IA_NO_ERROR;
    let mut p_ia_process_api: Option<
        unsafe extern "C" fn(pVOID, WORD32, WORD32, pVOID) -> IA_ERRORCODE,
    > = Some(
        ixheaacd_dec_api
            as unsafe extern "C" fn(pVOID, WORD32, WORD32, pVOID) -> IA_ERRORCODE,
    );
    let mut p_proc_err_info: *mut ia_error_info_struct = &raw mut ixheaacd_error_info;
    err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        p_ia_process_api_obj,
        IA_API_CMD_GET_CONFIG_PARAM,
        IA_XHEAAC_DEC_CONFIG_PARAM_SAMP_FREQ,
        pi_samp_freq as pVOID,
    );
    if err_code != IA_NO_ERROR {
        ixheaacd_error_handler(
            p_proc_err_info,
            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
            err_code,
        );
        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
            return err_code;
        }
    }
    err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        p_ia_process_api_obj,
        IA_API_CMD_GET_CONFIG_PARAM,
        IA_XHEAAC_DEC_CONFIG_PARAM_NUM_CHANNELS,
        pi_num_chan as pVOID,
    );
    if err_code != IA_NO_ERROR {
        ixheaacd_error_handler(
            p_proc_err_info,
            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
            err_code,
        );
        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
            return err_code;
        }
    }
    err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        p_ia_process_api_obj,
        IA_API_CMD_GET_CONFIG_PARAM,
        IA_XHEAAC_DEC_CONFIG_PARAM_PCM_WDSZ,
        pi_pcm_wd_sz as pVOID,
    );
    if err_code != IA_NO_ERROR {
        ixheaacd_error_handler(
            p_proc_err_info,
            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
            err_code,
        );
        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
            return err_code;
        }
    }
    err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        p_ia_process_api_obj,
        IA_API_CMD_GET_CONFIG_PARAM,
        IA_XHEAAC_DEC_CONFIG_PARAM_CHANNEL_MASK,
        pi_channel_mask as pVOID,
    );
    if err_code != IA_NO_ERROR {
        ixheaacd_error_handler(
            p_proc_err_info,
            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
            err_code,
        );
        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
            return err_code;
        }
    }
    let mut ui_channel_mode: UWORD32 = 0;
    err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        p_ia_process_api_obj,
        IA_API_CMD_GET_CONFIG_PARAM,
        IA_XHEAAC_DEC_CONFIG_PARAM_CHANNEL_MODE,
        &mut ui_channel_mode as *mut UWORD32 as pVOID,
    );
    if err_code != IA_NO_ERROR {
        ixheaacd_error_handler(
            p_proc_err_info,
            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
            err_code,
        );
        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
            return err_code;
        }
    }
    if ui_channel_mode == 0 as UWORD32 {
        printf(b"Channel Mode: MONO_OR_PS\n\0" as *const u8 as *const core::ffi::c_char);
    } else if ui_channel_mode == 1 as UWORD32 {
        printf(b"Channel Mode: STEREO\n\0" as *const u8 as *const core::ffi::c_char);
    } else if ui_channel_mode == 2 as UWORD32 {
        printf(b"Channel Mode: DUAL-MONO\n\0" as *const u8 as *const core::ffi::c_char);
    } else {
        printf(
            b"Channel Mode: NONE_OF_THESE or MULTICHANNEL\n\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    let mut ui_sbr_mode: UWORD32 = 0;
    err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        p_ia_process_api_obj,
        IA_API_CMD_GET_CONFIG_PARAM,
        IA_XHEAAC_DEC_CONFIG_PARAM_SBR_MODE,
        &mut ui_sbr_mode as *mut UWORD32 as pVOID,
    );
    if err_code != IA_NO_ERROR {
        ixheaacd_error_handler(
            p_proc_err_info,
            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
            err_code,
        );
        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
            return err_code;
        }
    }
    if ui_sbr_mode == 0 as UWORD32 {
        printf(b"SBR Mode: NOT_PRESENT\n\0" as *const u8 as *const core::ffi::c_char);
    } else if ui_sbr_mode == 1 as UWORD32 {
        printf(
            b"SBR Mode: UPSAMPLING FACTOR 2 or 8/3\n\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else if ui_sbr_mode == 2 as UWORD32 {
        printf(b"SBR Mode: ILLEGAL\n\0" as *const u8 as *const core::ffi::c_char);
    } else if ui_sbr_mode == 3 as UWORD32 {
        printf(
            b"ESBR Mode: UPSAMPLING FACTOR 4\n\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {
        printf(b"ui_sbr_mode not vaild\n\0" as *const u8 as *const core::ffi::c_char);
    }
    *pi_sbr_mode = ui_sbr_mode as core::ffi::c_int;
    let mut ui_aot: UWORD32 = 0;
    err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        p_ia_process_api_obj,
        IA_API_CMD_GET_CONFIG_PARAM,
        IA_XHEAAC_DEC_CONFIG_PARAM_AOT,
        &mut ui_aot as *mut UWORD32 as pVOID,
    );
    if err_code != IA_NO_ERROR {
        ixheaacd_error_handler(
            p_proc_err_info,
            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
            err_code,
        );
        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
            return err_code;
        }
    }
    *pi_aot = ui_aot as core::ffi::c_int;
    return IA_NO_ERROR;
}
#[no_mangle]
pub static mut counter_bl: core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_main_process(
    mut argc: WORD32,
    mut argv: *mut pWORD8,
) -> core::ffi::c_int {
    let mut i: LOOPIDX = 0;
    let mut frame_counter: WORD = 0 as WORD;
    let mut pb_process_name: [WORD8; 80] = ::core::mem::transmute::<
        [u8; 80],
        [WORD8; 80],
    >(
        *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut pb_lib_version: [WORD8; 80] = ::core::mem::transmute::<
        [u8; 80],
        [WORD8; 80],
    >(
        *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut err_code: IA_ERRORCODE = IA_NO_ERROR;
    let mut err_code_reinit: IA_ERRORCODE = IA_NO_ERROR;
    let mut pv_ia_process_api_obj: pVOID = 0 as *mut core::ffi::c_void;
    let mut pv_ia_drc_process_api_obj: pVOID = 0 as *mut core::ffi::c_void;
    let mut pui_api_size: UWORD32 = 0;
    let mut drc_ip_buf: [UWORD8; 131072] = [0; 131072];
    let mut drc_op_buf: [UWORD8; 131072] = [0; 131072];
    let mut n_mems: UWORD32 = 0;
    let mut ui_rem: UWORD32 = 0;
    let mut ui_proc_mem_tabs_size: UWORD32 = 0;
    let mut pui_ap_isize: UWORD32 = 0;
    let mut ui_init_done: UWORD32 = 0;
    let mut ui_exec_done: UWORD32 = 0;
    let mut pb_inp_buf: pWORD8 = 0 as pWORD8;
    let mut pb_out_buf: pWORD8 = 0 as pWORD8;
    let mut num_preroll: WORD32 = 0 as WORD32;
    let mut ui_inp_size: UWORD32 = 0 as UWORD32;
    let mut i_bytes_consumed: WORD32 = 0;
    let mut i_bytes_read: WORD32 = 0;
    let mut i_buff_size: WORD32 = 0;
    let mut prev_sampling_rate: WORD32 = 0 as WORD32;
    let mut skip_samples: WORD32 = 0 as WORD32;
    let mut total_samples: WORD32 = 0 as WORD32;
    let mut write_flag: WORD32 = 1 as WORD32;
    let mut bytes_to_write: WORD32 = 0 as WORD32;
    let mut ixheaacd_drc_offset: WORD32 = 0 as WORD32;
    let mut current_samples: WORD32 = 0 as WORD32;
    let mut samples_written: WORD32 = 0 as WORD32;
    let mut init_iteration: WORD32 = 1 as WORD32;
    let mut fatal_error_chk: WORD32 = 0;
    let mut i_out_bytes: WORD32 = 0;
    let mut i_total_bytes: WORD32 = 0 as WORD32;
    let mut i_samp_freq: WORD32 = 0;
    let mut i_num_chan: WORD32 = 0;
    let mut i_pcm_wd_sz: WORD32 = 0;
    let mut i_channel_mask: WORD32 = 0;
    let mut i_sbr_mode: WORD32 = 0;
    let mut i_effect_type: WORD32 = 0 as WORD32;
    let mut i_target_loudness: WORD32 = 0 as WORD32;
    let mut i_loud_norm: WORD32 = 0 as WORD32;
    let mut drc_flag: WORD32 = 0 as WORD32;
    let mut mpegd_drc_present: WORD32 = 0 as WORD32;
    let mut uo_num_chan: WORD32 = 0;
    let mut ui_boost: UWORD32 = 0 as UWORD32;
    let mut ui_cut: UWORD32 = 0 as UWORD32;
    let mut ui_drc_mode_cut: UWORD8 = 0 as UWORD8;
    let mut ui_drc_mode_boost: UWORD8 = 0 as UWORD8;
    let mut i_loudness_leveling_flag: WORD32 = 1 as WORD32;
    let mut p_ia_process_api: Option<
        unsafe extern "C" fn(pVOID, WORD32, WORD32, pVOID) -> IA_ERRORCODE,
    > = None;
    let mut p_set_config_param: Option<
        unsafe extern "C" fn(WORD32, *mut pWORD8, pVOID) -> IA_ERRORCODE,
    > = None;
    let mut p_get_config_param: Option<
        unsafe extern "C" fn(
            pVOID,
            pWORD32,
            pWORD32,
            pWORD32,
            pWORD32,
            pWORD32,
            pWORD32,
        ) -> IA_ERRORCODE,
    > = None;
    let mut ui_aot: WORD32 = 0 as WORD32;
    let mut p_error_init: Option<unsafe extern "C" fn() -> VOID> = None;
    let mut p_proc_err_info: *mut ia_error_info_struct = 0 as *mut ia_error_info_struct;
    p_ia_process_api = Some(
        ixheaacd_dec_api
            as unsafe extern "C" fn(pVOID, WORD32, WORD32, pVOID) -> IA_ERRORCODE,
    ) as Option<unsafe extern "C" fn(pVOID, WORD32, WORD32, pVOID) -> IA_ERRORCODE>;
    p_set_config_param = Some(
        ixheaacd_set_config_param
            as unsafe extern "C" fn(WORD32, *mut pWORD8, pVOID) -> IA_ERRORCODE,
    ) as Option<unsafe extern "C" fn(WORD32, *mut pWORD8, pVOID) -> IA_ERRORCODE>;
    p_get_config_param = Some(
        ixheaacd_get_config_param
            as unsafe extern "C" fn(
                pVOID,
                pWORD32,
                pWORD32,
                pWORD32,
                pWORD32,
                pWORD32,
                pWORD32,
            ) -> IA_ERRORCODE,
    )
        as Option<
            unsafe extern "C" fn(
                pVOID,
                pWORD32,
                pWORD32,
                pWORD32,
                pWORD32,
                pWORD32,
                pWORD32,
            ) -> IA_ERRORCODE,
        >;
    p_error_init = Some(
        ::core::mem::transmute::<
            unsafe extern "C" fn() -> VOID,
            unsafe extern "C" fn() -> VOID,
        >(ixheaacd_error_handler_init),
    ) as Option<unsafe extern "C" fn() -> VOID>;
    p_proc_err_info = &raw mut ixheaacd_error_info;
    ::core::mem::transmute::<
        _,
        fn() -> VOID,
    >(
        (Some(p_error_init.expect("non-null function pointer")))
            .expect("non-null function pointer"),
    )();
    err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        NULL_0,
        IA_API_CMD_GET_LIB_ID_STRINGS,
        IA_CMD_TYPE_LIB_NAME,
        pb_process_name.as_mut_ptr() as pVOID,
    );
    if err_code != IA_NO_ERROR {
        ixheaacd_error_handler(
            p_proc_err_info,
            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
            err_code,
        );
        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
            return err_code as core::ffi::c_int;
        }
    }
    err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        NULL_0,
        IA_API_CMD_GET_LIB_ID_STRINGS,
        IA_CMD_TYPE_LIB_VERSION,
        pb_lib_version.as_mut_ptr() as pVOID,
    );
    if err_code != IA_NO_ERROR {
        ixheaacd_error_handler(
            p_proc_err_info,
            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
            err_code,
        );
        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
            return err_code as core::ffi::c_int;
        }
    }
    ia_display_id_message(pb_process_name.as_mut_ptr(), pb_lib_version.as_mut_ptr());
    err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        NULL_0,
        IA_API_CMD_GET_API_SIZE,
        0 as WORD32,
        &mut pui_ap_isize as *mut UWORD32 as pVOID,
    );
    if err_code != IA_NO_ERROR {
        ixheaacd_error_handler(
            p_proc_err_info,
            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
            err_code,
        );
        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
            return err_code as core::ffi::c_int;
        }
    }
    g_pv_arr_alloc_memory[g_w_malloc_count as usize] = malloc(
        pui_ap_isize.wrapping_add(4 as UWORD32) as size_t,
    ) as pVOID;
    if (g_pv_arr_alloc_memory[g_w_malloc_count as usize]).is_null() {
        err_code = IA_TESTBENCH_MFMAN_FATAL_MEM_ALLOC_FAILED as IA_ERRORCODE;
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                &raw mut ixheaacd_ia_testbench_error_info,
                b"API struct alloc\0" as *const u8 as *const core::ffi::c_char
                    as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
    }
    ui_rem = (g_pv_arr_alloc_memory[g_w_malloc_count as usize] as size_t & 3 as size_t)
        as UWORD32;
    pv_ia_process_api_obj = (g_pv_arr_alloc_memory[g_w_malloc_count as usize]
        as *mut WORD8)
        .offset(4 as core::ffi::c_int as isize)
        .offset(-(ui_rem as isize)) as pVOID;
    g_w_malloc_count += 1;
    err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        pv_ia_process_api_obj,
        IA_API_CMD_INIT,
        IA_CMD_TYPE_INIT_API_PRE_CONFIG_PARAMS,
        NULL_0,
    );
    if err_code != IA_NO_ERROR {
        ixheaacd_error_handler(
            p_proc_err_info,
            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
            err_code,
        );
        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
            return err_code as core::ffi::c_int;
        }
    }
    err_code = ia_drc_dec_api(
        NULL_0,
        IA_API_CMD_GET_API_SIZE,
        0 as WORD32,
        &mut pui_api_size as *mut UWORD32 as pVOID,
    );
    if err_code != IA_NO_ERROR {
        ixheaacd_error_handler(
            p_proc_err_info,
            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
            err_code,
        );
        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
            return err_code as core::ffi::c_int;
        }
    }
    g_pv_arr_alloc_memory[g_w_malloc_count as usize] = malloc(
        pui_api_size.wrapping_add(4 as UWORD32) as size_t,
    ) as pVOID;
    if (g_pv_arr_alloc_memory[g_w_malloc_count as usize]).is_null() {
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                &raw mut ixheaacd_ia_testbench_error_info,
                b"API struct alloc\0" as *const u8 as *const core::ffi::c_char
                    as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
    }
    ui_rem = (g_pv_arr_alloc_memory[g_w_malloc_count as usize] as size_t & 3 as size_t)
        as UWORD32;
    pv_ia_drc_process_api_obj = (g_pv_arr_alloc_memory[g_w_malloc_count as usize]
        as *mut WORD8)
        .offset(4 as core::ffi::c_int as isize)
        .offset(-(ui_rem as isize)) as pVOID;
    g_w_malloc_count += 1;
    err_code = ia_drc_dec_api(
        pv_ia_drc_process_api_obj,
        IA_API_CMD_INIT,
        IA_CMD_TYPE_INIT_API_PRE_CONFIG_PARAMS,
        NULL_0,
    );
    if err_code != IA_NO_ERROR {
        ixheaacd_error_handler(
            p_proc_err_info,
            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
            err_code,
        );
        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
            return err_code as core::ffi::c_int;
        }
    }
    err_code = (Some(p_set_config_param.expect("non-null function pointer")))
        .expect("non-null function pointer")(argc, argv, pv_ia_process_api_obj);
    if err_code != IA_NO_ERROR {
        ixheaacd_error_handler(
            p_proc_err_info,
            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
            err_code,
        );
        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
            return err_code as core::ffi::c_int;
        }
    }
    err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        pv_ia_process_api_obj,
        IA_API_CMD_GET_MEMTABS_SIZE,
        0 as WORD32,
        &mut ui_proc_mem_tabs_size as *mut UWORD32 as pVOID,
    );
    if err_code != IA_NO_ERROR {
        ixheaacd_error_handler(
            p_proc_err_info,
            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
            err_code,
        );
        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
            return err_code as core::ffi::c_int;
        }
    }
    g_pv_arr_alloc_memory[g_w_malloc_count as usize] = malloc(
        ui_proc_mem_tabs_size.wrapping_add(4 as UWORD32) as size_t,
    ) as pVOID;
    if (g_pv_arr_alloc_memory[g_w_malloc_count as usize]).is_null() {
        err_code = IA_TESTBENCH_MFMAN_FATAL_MEM_ALLOC_FAILED as IA_ERRORCODE;
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                &raw mut ixheaacd_ia_testbench_error_info,
                b"Mem tables alloc\0" as *const u8 as *const core::ffi::c_char
                    as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
    }
    ui_rem = (g_pv_arr_alloc_memory[g_w_malloc_count as usize] as size_t & 3 as size_t)
        as UWORD32;
    err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        pv_ia_process_api_obj,
        IA_API_CMD_SET_MEMTABS_PTR,
        0 as WORD32,
        (g_pv_arr_alloc_memory[g_w_malloc_count as usize] as *mut WORD8)
            .offset(4 as core::ffi::c_int as isize)
            .offset(-(ui_rem as isize)) as pVOID,
    );
    if err_code != IA_NO_ERROR {
        ixheaacd_error_handler(
            p_proc_err_info,
            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
            err_code,
        );
        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
            return err_code as core::ffi::c_int;
        }
    }
    g_w_malloc_count += 1;
    err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        pv_ia_process_api_obj,
        IA_API_CMD_INIT,
        IA_CMD_TYPE_INIT_API_POST_CONFIG_PARAMS,
        NULL_0,
    );
    if err_code != IA_NO_ERROR {
        ixheaacd_error_handler(
            p_proc_err_info,
            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
            err_code,
        );
        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
            return err_code as core::ffi::c_int;
        }
    }
    err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        pv_ia_process_api_obj,
        IA_API_CMD_GET_N_MEMTABS,
        0 as WORD32,
        &mut n_mems as *mut UWORD32 as pVOID,
    );
    if err_code != IA_NO_ERROR {
        ixheaacd_error_handler(
            p_proc_err_info,
            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
            err_code,
        );
        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
            return err_code as core::ffi::c_int;
        }
    }
    i = 0 as core::ffi::c_int as LOOPIDX;
    while i < n_mems as WORD32 {
        let mut ui_size: core::ffi::c_int = 0;
        let mut ui_alignment: core::ffi::c_int = 0;
        let mut ui_type: core::ffi::c_int = 0;
        let mut pv_alloc_ptr: pVOID = 0 as *mut core::ffi::c_void;
        err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            pv_ia_process_api_obj,
            IA_API_CMD_GET_MEM_INFO_SIZE,
            i as WORD32,
            &mut ui_size as *mut core::ffi::c_int as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            pv_ia_process_api_obj,
            IA_API_CMD_GET_MEM_INFO_ALIGNMENT,
            i as WORD32,
            &mut ui_alignment as *mut core::ffi::c_int as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            pv_ia_process_api_obj,
            IA_API_CMD_GET_MEM_INFO_TYPE,
            i as WORD32,
            &mut ui_type as *mut core::ffi::c_int as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        err_code = (Some(p_get_config_param.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            pv_ia_process_api_obj,
            &mut i_samp_freq,
            &mut i_num_chan,
            &mut i_pcm_wd_sz,
            &mut i_channel_mask,
            &mut i_sbr_mode,
            &mut ui_aot,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        g_pv_arr_alloc_memory[g_w_malloc_count as usize] = malloc(
            (ui_size + ui_alignment) as size_t,
        ) as pVOID;
        if (g_pv_arr_alloc_memory[g_w_malloc_count as usize]).is_null() {
            err_code = IA_TESTBENCH_MFMAN_FATAL_MEM_ALLOC_FAILED as IA_ERRORCODE;
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    &raw mut ixheaacd_ia_testbench_error_info,
                    b"Mem tables alloc\0" as *const u8 as *const core::ffi::c_char
                        as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code as core::ffi::c_int;
                }
            }
        }
        ui_rem = (g_pv_arr_alloc_memory[g_w_malloc_count as usize] as size_t)
            .wrapping_rem(ui_alignment as size_t) as UWORD32;
        pv_alloc_ptr = (g_pv_arr_alloc_memory[g_w_malloc_count as usize] as *mut WORD8)
            .offset(ui_alignment as isize)
            .offset(-(ui_rem as isize)) as pVOID;
        g_w_malloc_count += 1;
        err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(pv_ia_process_api_obj, IA_API_CMD_SET_MEM_PTR, i as WORD32, pv_alloc_ptr);
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        if ui_type == IA_MEMTYPE_INPUT {
            pb_inp_buf = pv_alloc_ptr as pWORD8;
            ui_inp_size = ui_size as UWORD32;
        }
        if ui_type == IA_MEMTYPE_OUTPUT {
            pb_out_buf = pv_alloc_ptr as pWORD8;
        }
        i += 1;
    }
    i_bytes_consumed = ui_inp_size as WORD32;
    i_buff_size = ui_inp_size as WORD32;
    loop {
        i_bytes_read = 0 as core::ffi::c_int as WORD32;
        if ui_inp_size.wrapping_sub((i_buff_size - i_bytes_consumed) as UWORD32)
            > 0 as UWORD32
        {
            i = 0 as core::ffi::c_int as LOOPIDX;
            while i < i_buff_size - i_bytes_consumed {
                *pb_inp_buf.offset(i as isize) = *pb_inp_buf
                    .offset((i as WORD32 + i_bytes_consumed) as isize);
                i += 1;
            }
            FileWrapper_Read(
                g_pf_inp,
                pb_inp_buf
                    .offset(i_buff_size as isize)
                    .offset(-(i_bytes_consumed as isize)) as *mut core::ffi::c_uchar,
                ui_inp_size.wrapping_sub((i_buff_size - i_bytes_consumed) as UWORD32)
                    as core::ffi::c_int,
                &mut i_bytes_read as *mut WORD32 as *mut core::ffi::c_uint,
            );
            i_buff_size = i_buff_size - (i_bytes_consumed - i_bytes_read);
            if i_buff_size <= 0 as core::ffi::c_int
                || err_code_reinit == 0x1804 as core::ffi::c_int
                    && i_bytes_read == 0 as core::ffi::c_int
            {
                i_buff_size = 0 as core::ffi::c_int as WORD32;
                err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(pv_ia_process_api_obj, IA_API_CMD_INPUT_OVER, 0 as WORD32, NULL_0);
                if err_code != IA_NO_ERROR {
                    ixheaacd_error_handler(
                        p_proc_err_info,
                        b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                        err_code,
                    );
                    if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                        return err_code as core::ffi::c_int;
                    }
                }
            }
        }
        if i_buff_size <= 0 as core::ffi::c_int
            || err_code_reinit == 0x1804 as core::ffi::c_int
                && i_bytes_read == 0 as core::ffi::c_int
        {
            i_buff_size = 0 as core::ffi::c_int as WORD32;
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(pv_ia_process_api_obj, IA_API_CMD_INPUT_OVER, 0 as WORD32, NULL_0);
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code as core::ffi::c_int;
                }
            }
            err_code = (Some(p_get_config_param.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                pv_ia_process_api_obj,
                &mut i_samp_freq,
                &mut i_num_chan,
                &mut i_pcm_wd_sz,
                &mut i_channel_mask,
                &mut i_sbr_mode,
                &mut ui_aot,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code as core::ffi::c_int;
                }
            }
            if i_samp_freq == 0 as core::ffi::c_int {
                i_samp_freq = prev_sampling_rate;
            }
            if fseek(g_pf_out, 0 as core::ffi::c_long, SEEK_SET) == 0 {
                write_wav_header(
                    g_pf_out,
                    i_total_bytes,
                    i_samp_freq,
                    i_num_chan,
                    i_pcm_wd_sz,
                    i_channel_mask,
                );
            }
            return 1 as core::ffi::c_int;
        }
        if init_iteration == 1 as core::ffi::c_int {
            if raw_testing != 0 {
                ixheaacd_i_bytes_to_read = get_metadata_dec_info_init(meta_info)
                    as WORD32;
            } else {
                ixheaacd_i_bytes_to_read = i_buff_size;
            }
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                pv_ia_process_api_obj,
                IA_API_CMD_SET_INPUT_BYTES,
                0 as WORD32,
                &raw mut ixheaacd_i_bytes_to_read as *mut WORD32 as pVOID,
            );
            init_iteration += 1;
        } else if raw_testing != 0 {
            ixheaacd_i_bytes_to_read = get_metadata_dec_exec(
                meta_info,
                frame_counter as core::ffi::c_int,
            ) as WORD32;
            if ixheaacd_i_bytes_to_read > ui_inp_size as WORD32 {
                return IA_FATAL_ERROR as core::ffi::c_int;
            }
            if ixheaacd_i_bytes_to_read <= 0 as core::ffi::c_int {
                err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(pv_ia_process_api_obj, IA_API_CMD_INPUT_OVER, 0 as WORD32, NULL_0);
                if err_code != IA_NO_ERROR {
                    ixheaacd_error_handler(
                        p_proc_err_info,
                        b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                        err_code,
                    );
                    if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                        return err_code as core::ffi::c_int;
                    }
                }
                return IA_NO_ERROR;
            }
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                pv_ia_process_api_obj,
                IA_API_CMD_SET_INPUT_BYTES,
                0 as WORD32,
                &raw mut ixheaacd_i_bytes_to_read as *mut WORD32 as pVOID,
            );
            init_iteration += 1;
        } else {
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                pv_ia_process_api_obj,
                IA_API_CMD_SET_INPUT_BYTES,
                0 as WORD32,
                &mut i_buff_size as *mut WORD32 as pVOID,
            );
        }
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(pv_ia_process_api_obj, IA_API_CMD_INIT, IA_CMD_TYPE_INIT_PROCESS, NULL_0);
        err_code_reinit = err_code;
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            pv_ia_process_api_obj,
            IA_API_CMD_INIT,
            IA_CMD_TYPE_INIT_DONE_QUERY,
            &mut ui_init_done as *mut UWORD32 as pVOID,
        );
        if init_iteration > 2 as core::ffi::c_int && ui_init_done == 0 as UWORD32 {
            frame_counter += 1;
        }
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            pv_ia_process_api_obj,
            IA_API_CMD_GET_CURIDX_INPUT_BUF,
            0 as WORD32,
            &mut i_bytes_consumed as *mut WORD32 as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        if !(ui_init_done == 0) {
            break;
        }
    }
    if ec_enable == 1 as core::ffi::c_int {
        mpeg_d_drc_on = 0 as core::ffi::c_int as WORD32;
    }
    if mpeg_d_drc_on == 1 as core::ffi::c_int {
        err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            pv_ia_process_api_obj,
            IA_API_CMD_GET_CONFIG_PARAM,
            IA_XHEAAC_DEC_CONFIG_PARAM_SBR_MODE,
            &mut i_sbr_mode as *mut WORD32 as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        if i_sbr_mode != 0 as core::ffi::c_int {
            let mut frame_length: WORD32 = 0;
            if i_sbr_mode == 1 as core::ffi::c_int {
                frame_length = 2048 as core::ffi::c_int as WORD32;
            } else if i_sbr_mode == 3 as core::ffi::c_int {
                frame_length = 4096 as core::ffi::c_int as WORD32;
            } else {
                frame_length = 1024 as core::ffi::c_int as WORD32;
            }
            err_code = ia_drc_dec_api(
                pv_ia_drc_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_DRC_DEC_CONFIG_PARAM_FRAME_SIZE,
                &mut frame_length as *mut WORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code as core::ffi::c_int;
                }
            }
        }
        err_code = (Some(p_get_config_param.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            pv_ia_process_api_obj,
            &mut i_samp_freq,
            &mut i_num_chan,
            &mut i_pcm_wd_sz,
            &mut i_channel_mask,
            &mut i_sbr_mode,
            &mut ui_aot,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        err_code = ia_drc_dec_api(
            pv_ia_drc_process_api_obj,
            IA_API_CMD_SET_CONFIG_PARAM,
            IA_DRC_DEC_CONFIG_PARAM_SAMP_FREQ,
            &mut i_samp_freq as *mut WORD32 as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        err_code = ia_drc_dec_api(
            pv_ia_drc_process_api_obj,
            IA_API_CMD_SET_CONFIG_PARAM,
            IA_DRC_DEC_CONFIG_PARAM_NUM_CHANNELS,
            &mut i_num_chan as *mut WORD32 as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        err_code = ia_drc_dec_api(
            pv_ia_drc_process_api_obj,
            IA_API_CMD_SET_CONFIG_PARAM,
            IA_DRC_DEC_CONFIG_PARAM_PCM_WDSZ,
            &mut i_pcm_wd_sz as *mut WORD32 as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            pv_ia_process_api_obj,
            IA_API_CMD_GET_CONFIG_PARAM,
            IA_XHEAAC_DEC_CONFIG_PARAM_DRC_EFFECT_TYPE,
            &mut i_effect_type as *mut WORD32 as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        err_code = ia_drc_dec_api(
            pv_ia_drc_process_api_obj,
            IA_API_CMD_SET_CONFIG_PARAM,
            IA_DRC_DEC_CONFIG_DRC_EFFECT_TYPE,
            &mut i_effect_type as *mut WORD32 as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            pv_ia_process_api_obj,
            IA_API_CMD_GET_CONFIG_PARAM,
            IA_XHEAAC_DEC_CONFIG_PARAM_DRC_TARGET_LOUDNESS,
            &mut i_target_loudness as *mut WORD32 as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        err_code = ia_drc_dec_api(
            pv_ia_drc_process_api_obj,
            IA_API_CMD_SET_CONFIG_PARAM,
            IA_DRC_DEC_CONFIG_DRC_TARGET_LOUDNESS,
            &mut i_target_loudness as *mut WORD32 as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            pv_ia_process_api_obj,
            IA_API_CMD_GET_CONFIG_PARAM,
            IA_XHEAAC_DEC_CONFIG_PARAM_DRC_LOUDNESS_LEVELING,
            &mut i_loudness_leveling_flag as *mut WORD32 as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        err_code = ia_drc_dec_api(
            pv_ia_drc_process_api_obj,
            IA_API_CMD_SET_CONFIG_PARAM,
            IA_DRC_DEC_CONFIG_DRC_LOUDNESS_LEVELING,
            &mut i_loudness_leveling_flag as *mut WORD32 as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            pv_ia_process_api_obj,
            IA_API_CMD_GET_CONFIG_PARAM,
            IA_XHEAAC_DEC_CONFIG_PARAM_DRC_MODE_CUT,
            &mut ui_drc_mode_cut as *mut UWORD8 as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        if ui_drc_mode_cut != 0 {
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                pv_ia_process_api_obj,
                IA_API_CMD_GET_CONFIG_PARAM,
                IA_XHEAAC_DEC_CONFIG_PARAM_DRC_CUT,
                &mut ui_cut as *mut UWORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code as core::ffi::c_int;
                }
            }
            err_code = ia_drc_dec_api(
                pv_ia_drc_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_DRC_DEC_CONFIG_DRC_CUT,
                &mut ui_cut as *mut UWORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code as core::ffi::c_int;
                }
            }
        }
        err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            pv_ia_process_api_obj,
            IA_API_CMD_GET_CONFIG_PARAM,
            IA_XHEAAC_DEC_CONFIG_PARAM_DRC_MODE_BOOST,
            &mut ui_drc_mode_boost as *mut UWORD8 as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        if ui_drc_mode_boost != 0 {
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                pv_ia_process_api_obj,
                IA_API_CMD_GET_CONFIG_PARAM,
                IA_XHEAAC_DEC_CONFIG_PARAM_DRC_BOOST,
                &mut ui_boost as *mut UWORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code as core::ffi::c_int;
                }
            }
            err_code = ia_drc_dec_api(
                pv_ia_drc_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_DRC_DEC_CONFIG_DRC_BOOST,
                &mut ui_boost as *mut UWORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code as core::ffi::c_int;
                }
            }
        }
        err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            pv_ia_process_api_obj,
            IA_API_CMD_GET_CONFIG_PARAM,
            IA_XHEAAC_DEC_CONFIG_PARAM_DRC_LOUD_NORM,
            &mut i_loud_norm as *mut WORD32 as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        err_code = ia_drc_dec_api(
            pv_ia_drc_process_api_obj,
            IA_API_CMD_SET_CONFIG_PARAM,
            IA_DRC_DEC_CONFIG_DRC_LOUD_NORM,
            &mut i_loud_norm as *mut WORD32 as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        err_code = ia_drc_dec_api(
            pv_ia_drc_process_api_obj,
            IA_API_CMD_GET_MEMTABS_SIZE,
            0 as WORD32,
            &mut ui_proc_mem_tabs_size as *mut UWORD32 as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        g_pv_arr_alloc_memory[g_w_malloc_count as usize] = malloc(
            ui_proc_mem_tabs_size as size_t,
        ) as pVOID;
        if (g_pv_arr_alloc_memory[g_w_malloc_count as usize]).is_null() {
            err_code = IA_TESTBENCH_MFMAN_FATAL_MEM_ALLOC_FAILED as IA_ERRORCODE;
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    &raw mut ixheaacd_ia_testbench_error_info,
                    b"Mem tables alloc\0" as *const u8 as *const core::ffi::c_char
                        as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code as core::ffi::c_int;
                }
            }
        }
        err_code = ia_drc_dec_api(
            pv_ia_drc_process_api_obj,
            IA_API_CMD_SET_MEMTABS_PTR,
            0 as WORD32,
            g_pv_arr_alloc_memory[g_w_malloc_count as usize] as *mut WORD8 as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        g_w_malloc_count += 1;
        err_code = ia_drc_dec_api(
            pv_ia_drc_process_api_obj,
            IA_API_CMD_INIT,
            IA_CMD_TYPE_INIT_API_POST_CONFIG_PARAMS,
            NULL_0,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        err_code = ia_drc_dec_api(
            pv_ia_drc_process_api_obj,
            IA_API_CMD_GET_N_MEMTABS,
            0 as WORD32,
            &mut n_mems as *mut UWORD32 as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        i = 0 as core::ffi::c_int as LOOPIDX;
        while i < n_mems as core::ffi::c_int - 2 as core::ffi::c_int {
            let mut ui_size_0: WORD32 = 0;
            let mut ui_alignment_0: WORD32 = 0;
            let mut ui_type_0: WORD32 = 0;
            let mut pv_alloc_ptr_0: pVOID = 0 as *mut core::ffi::c_void;
            err_code = ia_drc_dec_api(
                pv_ia_drc_process_api_obj,
                IA_API_CMD_GET_MEM_INFO_SIZE,
                i as WORD32,
                &mut ui_size_0 as *mut WORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code as core::ffi::c_int;
                }
            }
            err_code = ia_drc_dec_api(
                pv_ia_drc_process_api_obj,
                IA_API_CMD_GET_MEM_INFO_ALIGNMENT,
                i as WORD32,
                &mut ui_alignment_0 as *mut WORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code as core::ffi::c_int;
                }
            }
            err_code = ia_drc_dec_api(
                pv_ia_drc_process_api_obj,
                IA_API_CMD_GET_MEM_INFO_TYPE,
                i as WORD32,
                &mut ui_type_0 as *mut WORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code as core::ffi::c_int;
                }
            }
            g_pv_arr_alloc_memory[g_w_malloc_count as usize] = malloc(
                (ui_size_0 + ui_alignment_0) as size_t,
            ) as pVOID;
            if (g_pv_arr_alloc_memory[g_w_malloc_count as usize]).is_null() {
                if err_code != IA_NO_ERROR {
                    ixheaacd_error_handler(
                        &raw mut ixheaacd_ia_testbench_error_info,
                        b"Mem tables alloc\0" as *const u8 as *const core::ffi::c_char
                            as *mut WORD8,
                        err_code,
                    );
                    if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                        return err_code as core::ffi::c_int;
                    }
                }
            }
            ui_rem = (g_pv_arr_alloc_memory[g_w_malloc_count as usize] as size_t)
                .wrapping_rem(ui_alignment_0 as size_t) as UWORD32;
            pv_alloc_ptr_0 = (g_pv_arr_alloc_memory[g_w_malloc_count as usize]
                as *mut WORD8)
                .offset(ui_alignment_0 as isize)
                .offset(-(ui_rem as isize)) as pVOID;
            g_w_malloc_count += 1;
            err_code = ia_drc_dec_api(
                pv_ia_drc_process_api_obj,
                IA_API_CMD_SET_MEM_PTR,
                i as WORD32,
                pv_alloc_ptr_0,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code as core::ffi::c_int;
                }
            }
            i += 1;
        }
        err_code = ia_drc_dec_api(
            pv_ia_drc_process_api_obj,
            IA_API_CMD_SET_MEM_PTR,
            2 as WORD32,
            drc_ip_buf.as_mut_ptr() as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        err_code = ia_drc_dec_api(
            pv_ia_drc_process_api_obj,
            IA_API_CMD_SET_MEM_PTR,
            3 as WORD32,
            drc_op_buf.as_mut_ptr() as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        let mut p_array: [[*mut core::ffi::c_void; 16]; 2] = [[0
            as *mut core::ffi::c_void; 16]; 2];
        let mut ii: WORD32 = 0;
        let mut buf_sizes: [[WORD32; 16]; 2] = [[0; 16]; 2];
        let mut num_elements: WORD32 = 0;
        let mut num_config_ext: WORD32 = 0;
        let mut bit_str_fmt: WORD32 = 1 as WORD32;
        memset(
            buf_sizes.as_mut_ptr() as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (32 as size_t).wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
        );
        err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            pv_ia_process_api_obj,
            IA_API_CMD_GET_CONFIG_PARAM,
            IA_ENHAACPLUS_DEC_CONFIG_EXT_ELE_BUF_SIZES,
            &mut *(*buf_sizes.as_mut_ptr().offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut WORD32 as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            pv_ia_process_api_obj,
            IA_API_CMD_GET_CONFIG_PARAM,
            IA_ENHAACPLUS_DEC_CONFIG_EXT_ELE_PTR,
            &mut p_array as *mut [[*mut core::ffi::c_void; 16]; 2] as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        err_code = ia_drc_dec_api(
            pv_ia_drc_process_api_obj,
            IA_API_CMD_INIT,
            IA_CMD_TYPE_INIT_SET_BUFF_PTR,
            0 as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            pv_ia_process_api_obj,
            IA_API_CMD_GET_CONFIG_PARAM,
            IA_ENHAACPLUS_DEC_CONFIG_NUM_ELE,
            &mut num_elements as *mut WORD32 as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            pv_ia_process_api_obj,
            IA_API_CMD_GET_CONFIG_PARAM,
            IA_ENHAACPLUS_DEC_CONFIG_NUM_CONFIG_EXT,
            &mut num_config_ext as *mut WORD32 as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        ii = 0 as core::ffi::c_int as WORD32;
        while ii < num_config_ext {
            if buf_sizes[0 as core::ffi::c_int as usize][ii as usize]
                > 0 as core::ffi::c_int
            {
                memcpy(
                    drc_ip_buf.as_mut_ptr() as *mut core::ffi::c_void,
                    p_array[0 as core::ffi::c_int as usize][ii as usize],
                    buf_sizes[0 as core::ffi::c_int as usize][ii as usize] as size_t,
                );
                err_code = ia_drc_dec_api(
                    pv_ia_drc_process_api_obj,
                    IA_API_CMD_SET_CONFIG_PARAM,
                    IA_DRC_DEC_CONFIG_PARAM_BITS_FORMAT,
                    &mut bit_str_fmt as *mut WORD32 as pVOID,
                );
                err_code = ia_drc_dec_api(
                    pv_ia_drc_process_api_obj,
                    IA_API_CMD_SET_INPUT_BYTES_IL_BS,
                    0 as WORD32,
                    &mut *(*buf_sizes
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(ii as isize) as *mut WORD32 as pVOID,
                );
                if err_code != IA_NO_ERROR {
                    ixheaacd_error_handler(
                        p_proc_err_info,
                        b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                        err_code,
                    );
                    if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                        return err_code as core::ffi::c_int;
                    }
                }
                err_code = ia_drc_dec_api(
                    pv_ia_drc_process_api_obj,
                    IA_API_CMD_INIT,
                    IA_CMD_TYPE_INIT_CPY_IL_BSF_BUFF,
                    NULL_0,
                );
                if err_code != IA_NO_ERROR {
                    ixheaacd_error_handler(
                        p_proc_err_info,
                        b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                        err_code,
                    );
                    if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                        return err_code as core::ffi::c_int;
                    }
                }
                drc_flag = 1 as core::ffi::c_int as WORD32;
            }
            ii += 1;
        }
        ii = 0 as core::ffi::c_int as WORD32;
        while ii < num_elements {
            if buf_sizes[1 as core::ffi::c_int as usize][ii as usize]
                > 0 as core::ffi::c_int
            {
                memcpy(
                    drc_ip_buf.as_mut_ptr() as *mut core::ffi::c_void,
                    p_array[1 as core::ffi::c_int as usize][ii as usize],
                    buf_sizes[1 as core::ffi::c_int as usize][ii as usize] as size_t,
                );
                err_code = ia_drc_dec_api(
                    pv_ia_drc_process_api_obj,
                    IA_API_CMD_SET_CONFIG_PARAM,
                    IA_DRC_DEC_CONFIG_PARAM_BITS_FORMAT,
                    &mut bit_str_fmt as *mut WORD32 as pVOID,
                );
                err_code = ia_drc_dec_api(
                    pv_ia_drc_process_api_obj,
                    IA_API_CMD_SET_INPUT_BYTES_IC_BS,
                    0 as WORD32,
                    &mut *(*buf_sizes
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(ii as isize) as *mut WORD32 as pVOID,
                );
                if err_code != IA_NO_ERROR {
                    ixheaacd_error_handler(
                        p_proc_err_info,
                        b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                        err_code,
                    );
                    if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                        return err_code as core::ffi::c_int;
                    }
                }
                err_code = ia_drc_dec_api(
                    pv_ia_drc_process_api_obj,
                    IA_API_CMD_INIT,
                    IA_CMD_TYPE_INIT_CPY_IC_BSF_BUFF,
                    NULL_0,
                );
                if err_code != IA_NO_ERROR {
                    ixheaacd_error_handler(
                        p_proc_err_info,
                        b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                        err_code,
                    );
                    if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                        return err_code as core::ffi::c_int;
                    }
                }
                drc_flag = 1 as core::ffi::c_int as WORD32;
            }
            ii += 1;
        }
        if drc_flag == 1 as core::ffi::c_int {
            mpegd_drc_present = 1 as core::ffi::c_int as WORD32;
        } else {
            mpegd_drc_present = 0 as core::ffi::c_int as WORD32;
        }
        if mpegd_drc_present == 1 as core::ffi::c_int {
            let mut interface_is_present: WORD32 = 1 as WORD32;
            err_code = ia_drc_dec_api(
                pv_ia_drc_process_api_obj,
                IA_API_CMD_SET_CONFIG_PARAM,
                IA_DRC_DEC_CONFIG_PARAM_INT_PRESENT,
                &mut interface_is_present as *mut WORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code as core::ffi::c_int;
                }
            }
            err_code = ia_drc_dec_api(
                pv_ia_drc_process_api_obj,
                IA_API_CMD_INIT,
                IA_CMD_TYPE_INIT_CPY_IN_BSF_BUFF,
                NULL_0,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code as core::ffi::c_int;
                }
            }
            err_code = ia_drc_dec_api(
                pv_ia_drc_process_api_obj,
                IA_API_CMD_INIT,
                IA_CMD_TYPE_INIT_PROCESS,
                NULL_0,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code as core::ffi::c_int;
                }
            }
            err_code = ia_drc_dec_api(
                pv_ia_drc_process_api_obj,
                IA_API_CMD_GET_CONFIG_PARAM,
                IA_DRC_DEC_CONFIG_PARAM_NUM_CHANNELS,
                &mut uo_num_chan as *mut WORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code as core::ffi::c_int;
                }
            }
        }
    }
    err_code = (Some(p_get_config_param.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        pv_ia_process_api_obj,
        &mut i_samp_freq,
        &mut i_num_chan,
        &mut i_pcm_wd_sz,
        &mut i_channel_mask,
        &mut i_sbr_mode,
        &mut ui_aot,
    );
    if err_code != IA_NO_ERROR {
        ixheaacd_error_handler(
            p_proc_err_info,
            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
            err_code,
        );
        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
            return err_code as core::ffi::c_int;
        }
    }
    if ui_aot == 42 as core::ffi::c_int {
        esbr_testing = 1 as core::ffi::c_int as WORD32;
    }
    if raw_testing != 0 {
        skip_samples = get_start_offset_in_samples(meta_info) as WORD32;
        if ui_aot >= 23 as core::ffi::c_int && esbr_testing != 0 {
            skip_samples = (skip_samples as core::ffi::c_int - 2048 as core::ffi::c_int)
                as WORD32;
            if skip_samples < 0 as core::ffi::c_int {
                skip_samples = 0 as core::ffi::c_int as WORD32;
            }
        }
        if eld_testing == 0 as core::ffi::c_int {
            total_samples = get_play_time_in_samples(meta_info) as WORD32;
        }
    }
    write_wav_header(
        g_pf_out,
        0 as WORD32,
        i_samp_freq,
        i_num_chan,
        i_pcm_wd_sz,
        i_channel_mask,
    );
    prev_sampling_rate = i_samp_freq;
    loop {
        if ui_inp_size as WORD32 - (i_buff_size - i_bytes_consumed)
            > 0 as core::ffi::c_int
        {
            if i_sbr_mode != 0 && ui_aot < 23 as core::ffi::c_int && esbr_testing != 0 {
                if meta_info.ia_mp4_stsz_entries != frame_counter as UWORD32 {
                    i = 0 as core::ffi::c_int as LOOPIDX;
                    while i < i_buff_size - i_bytes_consumed {
                        *pb_inp_buf.offset(i as isize) = *pb_inp_buf
                            .offset((i as WORD32 + i_bytes_consumed) as isize);
                        i += 1;
                    }
                    FileWrapper_Read(
                        g_pf_inp,
                        pb_inp_buf
                            .offset(i_buff_size as isize)
                            .offset(-(i_bytes_consumed as isize))
                            as *mut core::ffi::c_uchar,
                        ui_inp_size as core::ffi::c_int
                            - (i_buff_size - i_bytes_consumed),
                        &mut i_bytes_read as *mut WORD32 as *mut core::ffi::c_uint,
                    );
                    i_buff_size = i_buff_size - (i_bytes_consumed - i_bytes_read);
                    if i_buff_size <= 0 as core::ffi::c_int
                        || err_code_reinit == 0x1804 as core::ffi::c_int
                            && i_bytes_read == 0 as core::ffi::c_int
                    {
                        i_buff_size = 0 as core::ffi::c_int as WORD32;
                        raw_testing = 0 as core::ffi::c_int as WORD32;
                        err_code = (Some(
                            p_ia_process_api.expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            pv_ia_process_api_obj,
                            IA_API_CMD_INPUT_OVER,
                            0 as WORD32,
                            NULL_0,
                        );
                        if err_code != IA_NO_ERROR {
                            ixheaacd_error_handler(
                                p_proc_err_info,
                                b"\0" as *const u8 as *const core::ffi::c_char
                                    as *mut WORD8,
                                err_code,
                            );
                            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                                return err_code as core::ffi::c_int;
                            }
                        }
                    }
                }
            } else {
                i = 0 as core::ffi::c_int as LOOPIDX;
                while i < i_buff_size - i_bytes_consumed {
                    *pb_inp_buf.offset(i as isize) = *pb_inp_buf
                        .offset((i as WORD32 + i_bytes_consumed) as isize);
                    i += 1;
                }
                FileWrapper_Read(
                    g_pf_inp,
                    pb_inp_buf
                        .offset(i_buff_size as isize)
                        .offset(-(i_bytes_consumed as isize)) as *mut core::ffi::c_uchar,
                    ui_inp_size as core::ffi::c_int - (i_buff_size - i_bytes_consumed),
                    &mut i_bytes_read as *mut WORD32 as *mut core::ffi::c_uint,
                );
                i_buff_size = i_buff_size - (i_bytes_consumed - i_bytes_read);
                if i_buff_size <= 0 as core::ffi::c_int
                    || err_code_reinit == 0x1804 as core::ffi::c_int
                        && i_bytes_read == 0 as core::ffi::c_int
                {
                    i_buff_size = 0 as core::ffi::c_int as WORD32;
                    raw_testing = 0 as core::ffi::c_int as WORD32;
                    err_code = (Some(
                        p_ia_process_api.expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        pv_ia_process_api_obj,
                        IA_API_CMD_INPUT_OVER,
                        0 as WORD32,
                        NULL_0,
                    );
                    if err_code != IA_NO_ERROR {
                        ixheaacd_error_handler(
                            p_proc_err_info,
                            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                            err_code,
                        );
                        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                            return err_code as core::ffi::c_int;
                        }
                    }
                }
            }
        }
        if i_sbr_mode != 0 && ui_aot < 23 as core::ffi::c_int && esbr_testing != 0 {
            if meta_info.ia_mp4_stsz_entries != frame_counter as UWORD32 {
                if raw_testing != 0 {
                    ixheaacd_i_bytes_to_read = get_metadata_dec_exec(
                        meta_info,
                        frame_counter as core::ffi::c_int,
                    ) as WORD32;
                    if ixheaacd_i_bytes_to_read > ui_inp_size as WORD32 {
                        return IA_FATAL_ERROR as core::ffi::c_int;
                    }
                    if ixheaacd_i_bytes_to_read <= 0 as core::ffi::c_int
                        && ec_enable == 0 as core::ffi::c_int
                    {
                        err_code = (Some(
                            p_ia_process_api.expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            pv_ia_process_api_obj,
                            IA_API_CMD_INPUT_OVER,
                            0 as WORD32,
                            NULL_0,
                        );
                        if err_code != IA_NO_ERROR {
                            ixheaacd_error_handler(
                                p_proc_err_info,
                                b"\0" as *const u8 as *const core::ffi::c_char
                                    as *mut WORD8,
                                err_code,
                            );
                            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                                return err_code as core::ffi::c_int;
                            }
                        }
                        return IA_NO_ERROR;
                    }
                    err_code = (Some(
                        p_ia_process_api.expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        pv_ia_process_api_obj,
                        IA_API_CMD_SET_INPUT_BYTES,
                        0 as WORD32,
                        &raw mut ixheaacd_i_bytes_to_read as *mut WORD32 as pVOID,
                    );
                } else {
                    err_code = (Some(
                        p_ia_process_api.expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        pv_ia_process_api_obj,
                        IA_API_CMD_SET_INPUT_BYTES,
                        0 as WORD32,
                        &mut i_buff_size as *mut WORD32 as pVOID,
                    );
                }
            }
        } else if raw_testing != 0 {
            ixheaacd_i_bytes_to_read = get_metadata_dec_exec(
                meta_info,
                frame_counter as core::ffi::c_int,
            ) as WORD32;
            if ixheaacd_i_bytes_to_read > ui_inp_size as WORD32 {
                return IA_FATAL_ERROR as core::ffi::c_int;
            }
            if ixheaacd_i_bytes_to_read <= 0 as core::ffi::c_int
                && ec_enable == 0 as core::ffi::c_int
            {
                err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(pv_ia_process_api_obj, IA_API_CMD_INPUT_OVER, 0 as WORD32, NULL_0);
                if err_code != IA_NO_ERROR {
                    ixheaacd_error_handler(
                        p_proc_err_info,
                        b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                        err_code,
                    );
                    if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                        return err_code as core::ffi::c_int;
                    }
                }
                return IA_NO_ERROR;
            }
            if ec_enable == 1 as core::ffi::c_int {
                if ixheaacd_i_bytes_to_read != 0 as core::ffi::c_int {
                    err_code = (Some(
                        p_ia_process_api.expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        pv_ia_process_api_obj,
                        IA_API_CMD_SET_INPUT_BYTES,
                        0 as WORD32,
                        &raw mut ixheaacd_i_bytes_to_read as *mut WORD32 as pVOID,
                    );
                } else if i_buff_size != 0 as core::ffi::c_int {
                    err_code = (Some(
                        p_ia_process_api.expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        pv_ia_process_api_obj,
                        IA_API_CMD_SET_INPUT_BYTES,
                        0 as WORD32,
                        &mut i_buff_size as *mut WORD32 as pVOID,
                    );
                }
            } else {
                err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    pv_ia_process_api_obj,
                    IA_API_CMD_SET_INPUT_BYTES,
                    0 as WORD32,
                    &raw mut ixheaacd_i_bytes_to_read as *mut WORD32 as pVOID,
                );
            }
        } else {
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                pv_ia_process_api_obj,
                IA_API_CMD_SET_INPUT_BYTES,
                0 as WORD32,
                &mut i_buff_size as *mut WORD32 as pVOID,
            );
            if i_buff_size <= 0 as core::ffi::c_int {
                err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(pv_ia_process_api_obj, IA_API_CMD_INPUT_OVER, 0 as WORD32, NULL_0);
                if err_code != IA_NO_ERROR {
                    ixheaacd_error_handler(
                        p_proc_err_info,
                        b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                        err_code,
                    );
                    if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                        return err_code as core::ffi::c_int;
                    }
                }
            }
        }
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        counter_bl = frame_counter as core::ffi::c_int;
        err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(pv_ia_process_api_obj, IA_API_CMD_EXECUTE, IA_CMD_TYPE_DO_EXECUTE, NULL_0);
        err_code_reinit = err_code;
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        fatal_error_chk = (err_code as core::ffi::c_uint & IA_FATAL_ERROR) as WORD32;
        err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            pv_ia_process_api_obj,
            IA_API_CMD_EXECUTE,
            IA_CMD_TYPE_DONE_QUERY,
            &mut ui_exec_done as *mut UWORD32 as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            pv_ia_process_api_obj,
            IA_API_CMD_GET_CONFIG_PARAM,
            IA_ENHAACPLUS_DEC_CONFIG_GET_NUM_PRE_ROLL_FRAMES,
            &mut num_preroll as *mut WORD32 as pVOID,
        );
        if err_code != IA_NO_ERROR {
            ixheaacd_error_handler(
                p_proc_err_info,
                b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                err_code,
            );
            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                return err_code as core::ffi::c_int;
            }
        }
        let mut preroll_frame_offset: WORD32 = 0 as WORD32;
        loop {
            if mpeg_d_drc_on == 1 as core::ffi::c_int {
                if ui_exec_done != 1 as UWORD32 {
                    let mut p_array_0: *mut core::ffi::c_void = 0
                        as *mut core::ffi::c_void;
                    let mut buf_size: WORD32 = 0 as WORD32;
                    let mut bit_str_fmt_0: WORD32 = 1 as WORD32;
                    let mut gain_stream_flag: WORD32 = 1 as WORD32;
                    err_code = (Some(
                        p_ia_process_api.expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        pv_ia_process_api_obj,
                        IA_API_CMD_GET_CONFIG_PARAM,
                        IA_ENHAACPLUS_DEC_CONFIG_GAIN_PAYLOAD_LEN,
                        &mut buf_size as *mut WORD32 as pVOID,
                    );
                    if err_code != IA_NO_ERROR {
                        ixheaacd_error_handler(
                            p_proc_err_info,
                            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                            err_code,
                        );
                        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                            return err_code as core::ffi::c_int;
                        }
                    }
                    err_code = (Some(
                        p_ia_process_api.expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        pv_ia_process_api_obj,
                        IA_API_CMD_GET_CONFIG_PARAM,
                        IA_ENHAACPLUS_DEC_CONFIG_GAIN_PAYLOAD_BUF,
                        &mut p_array_0 as *mut *mut core::ffi::c_void as pVOID,
                    );
                    if err_code != IA_NO_ERROR {
                        ixheaacd_error_handler(
                            p_proc_err_info,
                            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                            err_code,
                        );
                        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                            return err_code as core::ffi::c_int;
                        }
                    }
                    if buf_size > 0 as core::ffi::c_int {
                        err_code = ia_drc_dec_api(
                            pv_ia_drc_process_api_obj,
                            IA_API_CMD_SET_CONFIG_PARAM,
                            IA_DRC_DEC_CONFIG_PARAM_BITS_FORMAT,
                            &mut bit_str_fmt_0 as *mut WORD32 as pVOID,
                        );
                        memcpy(
                            drc_ip_buf.as_mut_ptr() as *mut core::ffi::c_void,
                            p_array_0,
                            buf_size as size_t,
                        );
                        err_code = ia_drc_dec_api(
                            pv_ia_drc_process_api_obj,
                            IA_API_CMD_SET_INPUT_BYTES_BS,
                            0 as WORD32,
                            &mut buf_size as *mut WORD32 as pVOID,
                        );
                        err_code = ia_drc_dec_api(
                            pv_ia_drc_process_api_obj,
                            IA_API_CMD_SET_CONFIG_PARAM,
                            IA_DRC_DEC_CONFIG_GAIN_STREAM_FLAG,
                            &mut gain_stream_flag as *mut WORD32 as pVOID,
                        );
                        if err_code != IA_NO_ERROR {
                            ixheaacd_error_handler(
                                p_proc_err_info,
                                b"\0" as *const u8 as *const core::ffi::c_char
                                    as *mut WORD8,
                                err_code,
                            );
                            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                                return err_code as core::ffi::c_int;
                            }
                        }
                        err_code = ia_drc_dec_api(
                            pv_ia_drc_process_api_obj,
                            IA_API_CMD_INIT,
                            IA_CMD_TYPE_INIT_CPY_BSF_BUFF,
                            NULL_0,
                        );
                        if err_code != IA_NO_ERROR {
                            ixheaacd_error_handler(
                                p_proc_err_info,
                                b"\0" as *const u8 as *const core::ffi::c_char
                                    as *mut WORD8,
                                err_code,
                            );
                            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                                return err_code as core::ffi::c_int;
                            }
                        }
                        mpegd_drc_present = 1 as core::ffi::c_int as WORD32;
                    }
                }
            }
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                pv_ia_process_api_obj,
                IA_API_CMD_GET_CURIDX_INPUT_BUF,
                0 as WORD32,
                &mut i_bytes_consumed as *mut WORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code as core::ffi::c_int;
                }
            }
            err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                pv_ia_process_api_obj,
                IA_API_CMD_GET_OUTPUT_BYTES,
                0 as WORD32,
                &mut i_out_bytes as *mut WORD32 as pVOID,
            );
            if err_code != IA_NO_ERROR {
                ixheaacd_error_handler(
                    p_proc_err_info,
                    b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                    err_code,
                );
                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                    return err_code as core::ffi::c_int;
                }
            }
            if err_code_reinit != 0 as core::ffi::c_int {
                memset(
                    pb_out_buf as *mut core::ffi::c_void,
                    0 as core::ffi::c_int,
                    i_out_bytes as size_t,
                );
            }
            if i_sbr_mode != 0 && ui_aot < 23 as core::ffi::c_int && esbr_testing != 0 {
                if frame_counter > 0 as core::ffi::c_int {
                    i_total_bytes += i_out_bytes;
                }
            } else {
                i_total_bytes += i_out_bytes;
            }
            if mpegd_drc_present == 1 as core::ffi::c_int {
                let mut is_config_changed: WORD32 = 0 as WORD32;
                let mut apply_crossfade: WORD32 = 0 as WORD32;
                err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    pv_ia_process_api_obj,
                    IA_API_CMD_GET_CONFIG_PARAM,
                    IA_XHEAAC_DEC_CONFIG_PARAM_SBR_MODE,
                    &mut i_sbr_mode as *mut WORD32 as pVOID,
                );
                if err_code != IA_NO_ERROR {
                    ixheaacd_error_handler(
                        p_proc_err_info,
                        b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                        err_code,
                    );
                    if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                        return err_code as core::ffi::c_int;
                    }
                }
                if i_sbr_mode != 0 as core::ffi::c_int {
                    let mut frame_length_0: WORD32 = 0;
                    if i_sbr_mode == 1 as core::ffi::c_int {
                        frame_length_0 = 2048 as core::ffi::c_int as WORD32;
                    } else if i_sbr_mode == 3 as core::ffi::c_int {
                        frame_length_0 = 4096 as core::ffi::c_int as WORD32;
                    } else {
                        frame_length_0 = 1024 as core::ffi::c_int as WORD32;
                    }
                    err_code = ia_drc_dec_api(
                        pv_ia_drc_process_api_obj,
                        IA_API_CMD_SET_CONFIG_PARAM,
                        IA_DRC_DEC_CONFIG_PARAM_FRAME_SIZE,
                        &mut frame_length_0 as *mut WORD32 as pVOID,
                    );
                    if err_code != IA_NO_ERROR {
                        ixheaacd_error_handler(
                            p_proc_err_info,
                            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                            err_code,
                        );
                        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                            return err_code as core::ffi::c_int;
                        }
                    }
                }
                err_code = ia_drc_dec_api(
                    pv_ia_drc_process_api_obj,
                    IA_API_CMD_SET_INPUT_BYTES,
                    0 as WORD32,
                    &mut i_out_bytes as *mut WORD32 as pVOID,
                );
                if err_code != IA_NO_ERROR {
                    ixheaacd_error_handler(
                        p_proc_err_info,
                        b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                        err_code,
                    );
                    if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                        return err_code as core::ffi::c_int;
                    }
                }
                err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    pv_ia_process_api_obj,
                    IA_API_CMD_GET_CONFIG_PARAM,
                    IA_ENHAACPLUS_DEC_DRC_IS_CONFIG_CHANGED,
                    &mut is_config_changed as *mut WORD32 as pVOID,
                );
                if err_code != IA_NO_ERROR {
                    ixheaacd_error_handler(
                        p_proc_err_info,
                        b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                        err_code,
                    );
                    if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                        return err_code as core::ffi::c_int;
                    }
                }
                err_code = ia_drc_dec_api(
                    pv_ia_drc_process_api_obj,
                    IA_API_CMD_SET_CONFIG_PARAM,
                    IA_DRC_DEC_CONFIG_PARAM_CONFIG_CHANGED,
                    &mut is_config_changed as *mut WORD32 as pVOID,
                );
                if err_code != IA_NO_ERROR {
                    ixheaacd_error_handler(
                        p_proc_err_info,
                        b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                        err_code,
                    );
                    if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                        return err_code as core::ffi::c_int;
                    }
                }
                err_code = (Some(p_ia_process_api.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    pv_ia_process_api_obj,
                    IA_API_CMD_GET_CONFIG_PARAM,
                    IA_ENHAACPLUS_DEC_DRC_APPLY_CROSSFADE,
                    &mut apply_crossfade as *mut WORD32 as pVOID,
                );
                if err_code != IA_NO_ERROR {
                    ixheaacd_error_handler(
                        p_proc_err_info,
                        b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                        err_code,
                    );
                    if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                        return err_code as core::ffi::c_int;
                    }
                }
                err_code = ia_drc_dec_api(
                    pv_ia_drc_process_api_obj,
                    IA_API_CMD_SET_CONFIG_PARAM,
                    IA_DRC_DEC_CONFIG_PARAM_APPLY_CROSSFADE,
                    &mut apply_crossfade as *mut WORD32 as pVOID,
                );
                if err_code != IA_NO_ERROR {
                    ixheaacd_error_handler(
                        p_proc_err_info,
                        b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                        err_code,
                    );
                    if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                        return err_code as core::ffi::c_int;
                    }
                }
                if is_config_changed == 1 as core::ffi::c_int {
                    let mut p_array_1: [[*mut core::ffi::c_void; 16]; 2] = [[0
                        as *mut core::ffi::c_void; 16]; 2];
                    let mut ii_0: WORD32 = 0;
                    let mut num_elements_0: WORD32 = 0;
                    let mut num_config_ext_0: WORD32 = 0;
                    let mut buf_sizes_0: [[WORD32; 16]; 2] = [[0; 16]; 2];
                    let mut bit_str_fmt_1: WORD32 = 1 as WORD32;
                    memset(
                        buf_sizes_0.as_mut_ptr() as *mut core::ffi::c_void,
                        0 as core::ffi::c_int,
                        (32 as size_t)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
                    );
                    err_code = (Some(
                        p_ia_process_api.expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        pv_ia_process_api_obj,
                        IA_API_CMD_GET_CONFIG_PARAM,
                        IA_ENHAACPLUS_DEC_CONFIG_EXT_ELE_BUF_SIZES,
                        &mut *(*buf_sizes_0
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize) as *mut WORD32
                            as pVOID,
                    );
                    if err_code != IA_NO_ERROR {
                        ixheaacd_error_handler(
                            p_proc_err_info,
                            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                            err_code,
                        );
                        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                            return err_code as core::ffi::c_int;
                        }
                    }
                    err_code = (Some(
                        p_ia_process_api.expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        pv_ia_process_api_obj,
                        IA_API_CMD_GET_CONFIG_PARAM,
                        IA_ENHAACPLUS_DEC_CONFIG_EXT_ELE_PTR,
                        &mut p_array_1 as *mut [[*mut core::ffi::c_void; 16]; 2] as pVOID,
                    );
                    if err_code != IA_NO_ERROR {
                        ixheaacd_error_handler(
                            p_proc_err_info,
                            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                            err_code,
                        );
                        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                            return err_code as core::ffi::c_int;
                        }
                    }
                    err_code = ia_drc_dec_api(
                        pv_ia_drc_process_api_obj,
                        IA_API_CMD_INIT,
                        IA_CMD_TYPE_INIT_SET_BUFF_PTR,
                        0 as pVOID,
                    );
                    if err_code != IA_NO_ERROR {
                        ixheaacd_error_handler(
                            p_proc_err_info,
                            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                            err_code,
                        );
                        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                            return err_code as core::ffi::c_int;
                        }
                    }
                    err_code = (Some(
                        p_ia_process_api.expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        pv_ia_process_api_obj,
                        IA_API_CMD_GET_CONFIG_PARAM,
                        IA_ENHAACPLUS_DEC_CONFIG_NUM_ELE,
                        &mut num_elements_0 as *mut WORD32 as pVOID,
                    );
                    if err_code != IA_NO_ERROR {
                        ixheaacd_error_handler(
                            p_proc_err_info,
                            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                            err_code,
                        );
                        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                            return err_code as core::ffi::c_int;
                        }
                    }
                    err_code = (Some(
                        p_ia_process_api.expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        pv_ia_process_api_obj,
                        IA_API_CMD_GET_CONFIG_PARAM,
                        IA_ENHAACPLUS_DEC_CONFIG_NUM_CONFIG_EXT,
                        &mut num_config_ext_0 as *mut WORD32 as pVOID,
                    );
                    if err_code != IA_NO_ERROR {
                        ixheaacd_error_handler(
                            p_proc_err_info,
                            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                            err_code,
                        );
                        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                            return err_code as core::ffi::c_int;
                        }
                    }
                    ii_0 = 0 as core::ffi::c_int as WORD32;
                    while ii_0 < num_config_ext_0 {
                        if buf_sizes_0[0 as core::ffi::c_int as usize][ii_0 as usize]
                            > 0 as core::ffi::c_int
                        {
                            memcpy(
                                drc_ip_buf.as_mut_ptr() as *mut core::ffi::c_void,
                                p_array_1[0 as core::ffi::c_int as usize][ii_0 as usize],
                                buf_sizes_0[0 as core::ffi::c_int as usize][ii_0 as usize]
                                    as size_t,
                            );
                            err_code = ia_drc_dec_api(
                                pv_ia_drc_process_api_obj,
                                IA_API_CMD_SET_CONFIG_PARAM,
                                IA_DRC_DEC_CONFIG_PARAM_BITS_FORMAT,
                                &mut bit_str_fmt_1 as *mut WORD32 as pVOID,
                            );
                            err_code = ia_drc_dec_api(
                                pv_ia_drc_process_api_obj,
                                IA_API_CMD_SET_INPUT_BYTES_IL_BS,
                                0 as WORD32,
                                &mut *(*buf_sizes_0
                                    .as_mut_ptr()
                                    .offset(0 as core::ffi::c_int as isize))
                                    .as_mut_ptr()
                                    .offset(ii_0 as isize) as *mut WORD32 as pVOID,
                            );
                            if err_code != IA_NO_ERROR {
                                ixheaacd_error_handler(
                                    p_proc_err_info,
                                    b"\0" as *const u8 as *const core::ffi::c_char
                                        as *mut WORD8,
                                    err_code,
                                );
                                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                                    return err_code as core::ffi::c_int;
                                }
                            }
                            err_code = ia_drc_dec_api(
                                pv_ia_drc_process_api_obj,
                                IA_API_CMD_INIT,
                                IA_CMD_TYPE_INIT_CPY_IL_BSF_BUFF,
                                NULL_0,
                            );
                            if err_code != IA_NO_ERROR {
                                ixheaacd_error_handler(
                                    p_proc_err_info,
                                    b"\0" as *const u8 as *const core::ffi::c_char
                                        as *mut WORD8,
                                    err_code,
                                );
                                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                                    return err_code as core::ffi::c_int;
                                }
                            }
                            drc_flag = 1 as core::ffi::c_int as WORD32;
                        }
                        ii_0 += 1;
                    }
                    ii_0 = 0 as core::ffi::c_int as WORD32;
                    while ii_0 < num_elements_0 {
                        if buf_sizes_0[1 as core::ffi::c_int as usize][ii_0 as usize]
                            > 0 as core::ffi::c_int
                        {
                            memcpy(
                                drc_ip_buf.as_mut_ptr() as *mut core::ffi::c_void,
                                p_array_1[1 as core::ffi::c_int as usize][ii_0 as usize],
                                buf_sizes_0[1 as core::ffi::c_int as usize][ii_0 as usize]
                                    as size_t,
                            );
                            err_code = ia_drc_dec_api(
                                pv_ia_drc_process_api_obj,
                                IA_API_CMD_SET_CONFIG_PARAM,
                                IA_DRC_DEC_CONFIG_PARAM_BITS_FORMAT,
                                &mut bit_str_fmt_1 as *mut WORD32 as pVOID,
                            );
                            err_code = ia_drc_dec_api(
                                pv_ia_drc_process_api_obj,
                                IA_API_CMD_SET_INPUT_BYTES_IC_BS,
                                0 as WORD32,
                                &mut *(*buf_sizes_0
                                    .as_mut_ptr()
                                    .offset(1 as core::ffi::c_int as isize))
                                    .as_mut_ptr()
                                    .offset(ii_0 as isize) as *mut WORD32 as pVOID,
                            );
                            if err_code != IA_NO_ERROR {
                                ixheaacd_error_handler(
                                    p_proc_err_info,
                                    b"\0" as *const u8 as *const core::ffi::c_char
                                        as *mut WORD8,
                                    err_code,
                                );
                                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                                    return err_code as core::ffi::c_int;
                                }
                            }
                            err_code = ia_drc_dec_api(
                                pv_ia_drc_process_api_obj,
                                IA_API_CMD_INIT,
                                IA_CMD_TYPE_INIT_CPY_IC_BSF_BUFF,
                                NULL_0,
                            );
                            if err_code != IA_NO_ERROR {
                                ixheaacd_error_handler(
                                    p_proc_err_info,
                                    b"\0" as *const u8 as *const core::ffi::c_char
                                        as *mut WORD8,
                                    err_code,
                                );
                                if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                                    return err_code as core::ffi::c_int;
                                }
                            }
                            drc_flag = 1 as core::ffi::c_int as WORD32;
                        }
                        ii_0 += 1;
                    }
                }
                memcpy(
                    drc_ip_buf.as_mut_ptr() as *mut core::ffi::c_void,
                    pb_out_buf.offset(preroll_frame_offset as isize)
                        as *const core::ffi::c_void,
                    i_out_bytes as size_t,
                );
                preroll_frame_offset += i_out_bytes;
                err_code = ia_drc_dec_api(
                    pv_ia_drc_process_api_obj,
                    IA_API_CMD_EXECUTE,
                    IA_CMD_TYPE_DO_EXECUTE,
                    NULL_0,
                );
                if err_code != IA_NO_ERROR {
                    ixheaacd_error_handler(
                        p_proc_err_info,
                        b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
                        err_code,
                    );
                    if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                        return err_code as core::ffi::c_int;
                    }
                }
                memcpy(
                    pb_out_buf as *mut core::ffi::c_void,
                    drc_op_buf.as_mut_ptr() as *const core::ffi::c_void,
                    i_out_bytes as size_t,
                );
            } else {
                memmove(
                    pb_out_buf as *mut core::ffi::c_void,
                    pb_out_buf.offset(preroll_frame_offset as isize)
                        as *const core::ffi::c_void,
                    i_out_bytes as size_t,
                );
                preroll_frame_offset += i_out_bytes;
            }
            num_preroll -= 1;
            if !(num_preroll > 0 as core::ffi::c_int) {
                break;
            }
        }
        if total_samples != 0 as core::ffi::c_int {
            if raw_testing != 0 {
                if i_total_bytes
                    <= skip_samples * i_num_chan * (i_pcm_wd_sz >> 3 as core::ffi::c_int)
                {
                    err_code = (Some(
                        p_get_config_param.expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        pv_ia_process_api_obj,
                        &mut i_samp_freq,
                        &mut i_num_chan,
                        &mut i_pcm_wd_sz,
                        &mut i_channel_mask,
                        &mut i_sbr_mode,
                        &mut ui_aot,
                    );
                    write_flag = 0 as core::ffi::c_int as WORD32;
                } else {
                    write_flag = 1 as core::ffi::c_int as WORD32;
                    bytes_to_write = i_total_bytes
                        - skip_samples * i_num_chan
                            * (i_pcm_wd_sz >> 3 as core::ffi::c_int);
                    if bytes_to_write < i_out_bytes {
                        ixheaacd_drc_offset = i_out_bytes - bytes_to_write;
                        i_out_bytes = bytes_to_write;
                        current_samples = bytes_to_write
                            / (i_num_chan * (i_pcm_wd_sz >> 3 as core::ffi::c_int));
                    } else {
                        ixheaacd_drc_offset = 0 as core::ffi::c_int as WORD32;
                        current_samples = i_out_bytes
                            / (i_num_chan * (i_pcm_wd_sz >> 3 as core::ffi::c_int));
                    }
                }
            }
            if raw_testing != 0 {
                samples_written += current_samples;
                if samples_written > total_samples {
                    i_out_bytes = (total_samples - (samples_written - current_samples))
                        * (i_num_chan * (i_pcm_wd_sz >> 3 as core::ffi::c_int));
                    if i_out_bytes < 0 as core::ffi::c_int {
                        i_out_bytes = 0 as core::ffi::c_int as WORD32;
                    }
                }
            }
        }
        if write_flag != 0 {
            if i_sbr_mode != 0 && ui_aot < 23 as core::ffi::c_int && esbr_testing != 0 {
                if frame_counter != 0 as core::ffi::c_int {
                    fwrite(
                        pb_out_buf.offset(ixheaacd_drc_offset as isize)
                            as *const core::ffi::c_void,
                        ::core::mem::size_of::<WORD8>() as size_t,
                        i_out_bytes as size_t,
                        g_pf_out,
                    );
                    fflush(g_pf_out);
                }
            } else {
                fwrite(
                    pb_out_buf.offset(ixheaacd_drc_offset as isize)
                        as *const core::ffi::c_void,
                    ::core::mem::size_of::<WORD8>() as size_t,
                    i_out_bytes as size_t,
                    g_pf_out,
                );
                fflush(g_pf_out);
            }
        }
        frame_counter += 1;
        if !(ui_exec_done == 0 && fatal_error_chk == 0) {
            break;
        }
    }
    fprintf(
        get_stderr_fd(),
        b"TOTAL FRAMES : [%5d] \n\0" as *const u8 as *const core::ffi::c_char,
        frame_counter,
    );
    err_code = (Some(p_get_config_param.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        pv_ia_process_api_obj,
        &mut i_samp_freq,
        &mut i_num_chan,
        &mut i_pcm_wd_sz,
        &mut i_channel_mask,
        &mut i_sbr_mode,
        &mut ui_aot,
    );
    if err_code != IA_NO_ERROR {
        ixheaacd_error_handler(
            p_proc_err_info,
            b"\0" as *const u8 as *const core::ffi::c_char as *mut WORD8,
            err_code,
        );
        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
            return err_code as core::ffi::c_int;
        }
    }
    if fseek(g_pf_out, 0 as core::ffi::c_long, SEEK_SET) == 0 {
        write_wav_header(
            g_pf_out,
            i_total_bytes,
            i_samp_freq,
            i_num_chan,
            i_pcm_wd_sz,
            i_channel_mask,
        );
    }
    return IA_NO_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn print_usage() {
    let mut str_lib_info: ia_lib_info_struct = {
        let mut init = ia_lib_info_struct {
            p_lib_name: 0 as *mut WORD8,
            p_version_num: 0 as *mut WORD8,
        };
        init
    };
    ixheaacd_get_lib_id_strings(&mut str_lib_info as *mut ia_lib_info_struct as pVOID);
    ia_display_id_message(
        str_lib_info.p_lib_name as *mut WORD8,
        str_lib_info.p_version_num as *mut WORD8,
    );
    printf(b"\n Usage \n\0" as *const u8 as *const core::ffi::c_char);
    printf(
        b"\n <executable> -ifile:<input_file> -imeta:<meta_data_file> -ofile:<output_file> [options]\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    printf(b"\n[options] can be,\0" as *const u8 as *const core::ffi::c_char);
    printf(b"\n[-mp4:<mp4_flag>]\0" as *const u8 as *const core::ffi::c_char);
    printf(b"\n[-pcmsz:<pcmwordsize>]\0" as *const u8 as *const core::ffi::c_char);
    printf(b"\n[-dmix:<down_mix>]\0" as *const u8 as *const core::ffi::c_char);
    printf(b"\n[-esbr_hq:<esbr_hq_flag>]\0" as *const u8 as *const core::ffi::c_char);
    printf(b"\n[-esbr_ps:<esbr_ps_flag>]\0" as *const u8 as *const core::ffi::c_char);
    printf(
        b"\n[-tostereo:<interleave_to_stereo>]\0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(b"\n[-dsample:<down_sample_sbr>]\0" as *const u8 as *const core::ffi::c_char);
    printf(
        b"\n[-drc_cut_fac:<drc_cut_factor>]\0" as *const u8 as *const core::ffi::c_char,
    );
    printf(
        b"\n[-drc_boost_fac:<drc_boost_factor>]\0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n[-drc_target_level:<drc_target_level>]\0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n[-drc_heavy_comp:<drc_heavy_compression>]\0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(b"\n[-effect:<effect_type>]\0" as *const u8 as *const core::ffi::c_char);
    printf(
        b"\n[-target_loudness:<target_loudness>]\0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(b"\n[-nosync:<disable_sync>]\0" as *const u8 as *const core::ffi::c_char);
    printf(b"\n[-sbrup:<auto_sbr_upsample>]\0" as *const u8 as *const core::ffi::c_char);
    printf(b"\n[-flflag:<framelength_flag>}\0" as *const u8 as *const core::ffi::c_char);
    printf(b"\n[-fs:<RAW_sample_rate>]\0" as *const u8 as *const core::ffi::c_char);
    printf(
        b"\n[-maxchannel:<maximum_num_channels>]\0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n[-coupchannel:<coupling_channel>]\0" as *const u8 as *const core::ffi::c_char,
    );
    printf(b"\n[-downmix:<down_mix_stereo>]\0" as *const u8 as *const core::ffi::c_char);
    printf(b"\n[-fs480:<ld_frame_size>]\0" as *const u8 as *const core::ffi::c_char);
    printf(
        b"\n[-ld_testing:<ld_testing_flag>]\0" as *const u8 as *const core::ffi::c_char,
    );
    printf(
        b"\n[-peak_limiter_off:<peak_limiter_off_flag>]\0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n[-err_conceal:<error_concealment_flag>]\0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(b"\n[-esbr:<esbr_flag>]\0" as *const u8 as *const core::ffi::c_char);
    printf(
        b"\n[-loudness_leveling:<loudness_leveling_flag>]\0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n\nwhere, \n  <input_file> is the input AAC-LC/HE-AACv1/HE-AACv2//AAC-LD/AAC-ELD/AAC-ELDv2/USAC file name\0"
            as *const u8 as *const core::ffi::c_char,
    );
    printf(
        b"\n  <meta_data_file> is a text file which contains metadata.\0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n   To be given when -mp4:1 is enabled\0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n  <output_file> is the output file name\0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n  <mp4_flag> is a flag that should be set to 1 when passing \0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n  raw stream along with meta data text file \0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n  <pcmwordsize> is the bits per sample info. 16/24\0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n  <down_mix> is to enable/disable always mono output. Default 1\0"
            as *const u8 as *const core::ffi::c_char,
    );
    printf(
        b"\n  <esbr_hq_flag> is to enable/disable high quality eSBR. Default 0\0"
            as *const u8 as *const core::ffi::c_char,
    );
    printf(
        b"\n  <esbr_ps_flag> is to indicate eSBR with PS. Default 0\0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n  <interleave_to_stereo> is to enable/disable always \0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n    interleaved to stereo output. Default 1 \0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n  <down_sample_sbr> is to enable/disable down-sampled SBR \0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n    output. Default auto identification from header\0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n  <drc_cut_factor> is to set DRC cut factor value. Default value is 1\0"
            as *const u8 as *const core::ffi::c_char,
    );
    printf(
        b"\n  <drc_boost_factor> is to set DRC boost factor. Default value is 1\0"
            as *const u8 as *const core::ffi::c_char,
    );
    printf(
        b"\n  <drc_target_level> is to set DRC target reference level.\0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(b"\n    Default value is 108\0" as *const u8 as *const core::ffi::c_char);
    printf(
        b"\n  <drc_heavy_compression> is to enable / disable DRC heavy compression.\0"
            as *const u8 as *const core::ffi::c_char,
    );
    printf(b"\n    Default value is 0\0" as *const u8 as *const core::ffi::c_char);
    printf(
        b"\n  <effect_type> is set DRC effect type. Default value is 0\0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n  <target_loudness> is to set target loudness level.\0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(b"\n    Default value is -24\0" as *const u8 as *const core::ffi::c_char);
    printf(
        b"\n  <disable_sync> is to disable the ADTS/ADIF sync search i.e\0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n    when enabled the decoder expects the header to \0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n    be at the start of input buffer. Default 0\0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n  <auto_sbr_upsample> is to enable(1) or disable(0) auto SBR upsample \0"
            as *const u8 as *const core::ffi::c_char,
    );
    printf(
        b"\n    in case of stream changing from SBR present to SBR not present. Default 1\0"
            as *const u8 as *const core::ffi::c_char,
    );
    printf(
        b"\n  <framelength_flag> is flag for decoding framelength of 1024 or 960.\0"
            as *const u8 as *const core::ffi::c_char,
    );
    printf(
        b"\n    1 to decode 960 frame length, 0 to decode 1024 frame length\0"
            as *const u8 as *const core::ffi::c_char,
    );
    printf(
        b"\n    Frame length value in the GA header will override this option.\0"
            as *const u8 as *const core::ffi::c_char,
    );
    printf(b"\n    Default 0  \0" as *const u8 as *const core::ffi::c_char);
    printf(
        b"\n  <RAW_sample_rate> is to indicate the core AAC sample rate for\0"
            as *const u8 as *const core::ffi::c_char,
    );
    printf(
        b"\n    a RAW stream. If this is specified no other file format\0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n    headers are searched for.\0" as *const u8 as *const core::ffi::c_char,
    );
    printf(
        b"\n  <maximum_num_channels> is the number of maxiumum \0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n    channels the input may have. Default is 6 \0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n    for multichannel libraries and 2 for stereo libraries\0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n  <coupling_channel> is element instance tag of \0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n    independent coupling channel to be mixed. Default is 0\0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n  <down_mix_stereo> is flag for Downmix. Give 1 to\0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n    get stereo (downmix) output. Default is 0\0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n  <ld_frame_size> is to indicate ld frame size.\0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(
        b"\n   0 is for 512 frame length, 1 is for 480 frame length.\0" as *const u8
            as *const core::ffi::c_char,
    );
    printf(b"\n    Default value is 512 (0)\0" as *const u8 as *const core::ffi::c_char);
    printf(
        b"\n  <ld_testing_flag> is to enable / disable ld decoder testing.\0"
            as *const u8 as *const core::ffi::c_char,
    );
    printf(b"\n    Default value is 0\0" as *const u8 as *const core::ffi::c_char);
    printf(
        b"\n  <peak_limiter_off_flag> is to enable / disable peak limiter.\0"
            as *const u8 as *const core::ffi::c_char,
    );
    printf(b"\n    Default value is 0\0" as *const u8 as *const core::ffi::c_char);
    printf(
        b"\n  <error_concealment_flag> is to enable / disable error concealment.\0"
            as *const u8 as *const core::ffi::c_char,
    );
    printf(b"\n    Default value is 0\0" as *const u8 as *const core::ffi::c_char);
    printf(
        b"\n  <esbr_flag> is to enable / disable eSBR. Default value is 1\n\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    printf(
        b"\n  <loudness_leveling_flag> is to enable / disable loudness leveling.\0"
            as *const u8 as *const core::ffi::c_char,
    );
    printf(b"\n    Default value is 1\0" as *const u8 as *const core::ffi::c_char);
}
unsafe fn main_0(
    mut argc: WORD32,
    mut argv: *mut *mut core::ffi::c_char,
) -> core::ffi::c_int {
    let mut i: WORD32 = 0;
    let mut err_code: WORD32 = IA_NO_ERROR;
    ia_testbench_error_handler_init();
    g_pf_inp = 0 as FileWrapperPtr;
    g_pf_meta = 0 as *mut FILE;
    g_pf_out = 0 as *mut FILE;
    i = 1 as core::ffi::c_int as WORD32;
    while i < argc {
        printf(
            b"%s \0" as *const u8 as *const core::ffi::c_char,
            *argv.offset(i as isize),
        );
        if strncmp(
            *argv.offset(i as isize) as *const core::ffi::c_char,
            b"-ifile:\0" as *const u8 as *const core::ffi::c_char,
            7 as size_t,
        ) == 0
        {
            let mut pb_arg_val: pWORD8 = (*argv.offset(i as isize) as pWORD8)
                .offset(7 as core::ffi::c_int as isize);
            g_pf_inp = FileWrapper_Open(pb_arg_val as *mut core::ffi::c_char);
            if g_pf_inp.is_null() {
                err_code = IA_TESTBENCH_MFMAN_FATAL_FILE_OPEN_FAILED as WORD32;
                ixheaacd_error_handler(
                    &raw mut ixheaacd_ia_testbench_error_info,
                    b"Input File\0" as *const u8 as *const core::ffi::c_char
                        as *mut WORD8,
                    err_code as IA_ERRORCODE,
                );
                exit(1 as core::ffi::c_int);
            }
            raw_testing = 0 as core::ffi::c_int as WORD32;
        }
        if strncmp(
            *argv.offset(i as isize) as *const core::ffi::c_char,
            b"-imeta:\0" as *const u8 as *const core::ffi::c_char,
            7 as size_t,
        ) == 0
        {
            let mut pb_arg_val_0: pWORD8 = (*argv.offset(i as isize) as pWORD8)
                .offset(7 as core::ffi::c_int as isize);
            g_pf_meta = fopen(
                pb_arg_val_0 as *const core::ffi::c_char,
                b"r\0" as *const u8 as *const core::ffi::c_char,
            ) as *mut FILE;
            if g_pf_meta.is_null() {
                err_code = IA_TESTBENCH_MFMAN_FATAL_FILE_OPEN_FAILED as WORD32;
                ixheaacd_error_handler(
                    &raw mut ixheaacd_ia_testbench_error_info,
                    b"Metadata File\0" as *const u8 as *const core::ffi::c_char
                        as *mut WORD8,
                    err_code as IA_ERRORCODE,
                );
                exit(1 as core::ffi::c_int);
            }
            err_code = ixheaacd_read_metadata_info(g_pf_meta, &raw mut meta_info) as WORD32;
            if err_code == -(1 as core::ffi::c_int) {
                exit(1 as core::ffi::c_int);
            }
            raw_testing = 1 as core::ffi::c_int as WORD32;
        }
        if strncmp(
            *argv.offset(i as isize) as *const core::ffi::c_char,
            b"-ofile:\0" as *const u8 as *const core::ffi::c_char,
            7 as size_t,
        ) == 0
        {
            let mut pb_arg_val_1: pWORD8 = (*argv.offset(i as isize) as pWORD8)
                .offset(7 as core::ffi::c_int as isize);
            g_pf_out = fopen(
                pb_arg_val_1 as *const core::ffi::c_char,
                b"wb\0" as *const u8 as *const core::ffi::c_char,
            ) as *mut FILE;
            if g_pf_out.is_null() {
                err_code = IA_TESTBENCH_MFMAN_FATAL_FILE_OPEN_FAILED as WORD32;
                ixheaacd_error_handler(
                    &raw mut ixheaacd_ia_testbench_error_info,
                    b"Output File\0" as *const u8 as *const core::ffi::c_char
                        as *mut WORD8,
                    err_code as IA_ERRORCODE,
                );
                exit(1 as core::ffi::c_int);
            }
        }
        i += 1;
    }
    if g_pf_inp.is_null() || g_pf_out.is_null() {
        print_usage();
        err_code = IA_TESTBENCH_MFMAN_FATAL_FILE_OPEN_FAILED as WORD32;
        ixheaacd_error_handler(
            &raw mut ixheaacd_ia_testbench_error_info,
            b"Input or Output File\0" as *const u8 as *const core::ffi::c_char
                as *mut WORD8,
            err_code as IA_ERRORCODE,
        );
        exit(1 as core::ffi::c_int);
    }
    g_w_malloc_count = 0 as core::ffi::c_int as WORD;
    printf(b"\n\0" as *const u8 as *const core::ffi::c_char);
    i = 0 as core::ffi::c_int as WORD32;
    while i < argc {
        if strcmp(
            *argv.offset(i as isize) as *const core::ffi::c_char,
            b"-mp4:1\0" as *const u8 as *const core::ffi::c_char,
        ) == 0
        {
            if g_pf_meta.is_null() {
                print_usage();
                err_code = IA_TESTBENCH_MFMAN_FATAL_FILE_OPEN_FAILED as WORD32;
                ixheaacd_error_handler(
                    &raw mut ixheaacd_ia_testbench_error_info,
                    b"Metadata File\0" as *const u8 as *const core::ffi::c_char
                        as *mut WORD8,
                    err_code as IA_ERRORCODE,
                );
                exit(1 as core::ffi::c_int);
            }
        }
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < argc {
        if strcmp(
            *argv.offset(i as isize) as *const core::ffi::c_char,
            b"-eld_testing:1\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
        {
            eld_testing = 0 as core::ffi::c_int as WORD32;
            i += 1;
        } else {
            eld_testing = 1 as core::ffi::c_int as WORD32;
            break;
        }
    }
    ixheaacd_main_process(
        argc - 1 as WORD32,
        &mut *argv.offset(1 as core::ffi::c_int as isize) as *mut *mut core::ffi::c_char
            as *mut pWORD8,
    );
    i = 0 as core::ffi::c_int as WORD32;
    while i < g_w_malloc_count {
        if !(g_pv_arr_alloc_memory[i as usize]).is_null() {
            free(g_pv_arr_alloc_memory[i as usize]);
        }
        i += 1;
    }
    if !g_pf_out.is_null() {
        fclose(g_pf_out);
    }
    if !g_pf_meta.is_null() {
        fclose(g_pf_meta);
        metadata_mp4_stsz_size_free(&raw mut meta_info);
    }
    FileWrapper_Close(g_pf_inp);
    mpeg_d_drc_on = 0 as core::ffi::c_int as WORD32;
    return IA_NO_ERROR;
}
pub fn main() {
    let mut args: Vec<*mut core::ffi::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as WORD32,
                args.as_mut_ptr() as *mut *mut core::ffi::c_char,
            ) as i32,
        )
    }
}
