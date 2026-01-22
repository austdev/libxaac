extern "C" {
    fn printf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
}
pub type WORD8 = core::ffi::c_schar;
pub type pWORD8 = *mut core::ffi::c_schar;
pub type WORD32 = core::ffi::c_int;
pub type VOID = ();
pub type WORD = core::ffi::c_int;
pub type UWORD = core::ffi::c_uint;
pub type IA_ERRORCODE = WORD32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_error_info_struct {
    pub pb_module_name: pWORD8,
    pub ppb_class_names: [pWORD8; 16],
    pub ppppb_error_msg_pointers: [[*mut *mut WORD8; 16]; 2],
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const IA_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_FATAL_ERROR: core::ffi::c_uint = 0x80000000 as core::ffi::c_uint;
pub const IA_MAX_ERROR_SUB_CODE: core::ffi::c_int = 28 as core::ffi::c_int;
#[no_mangle]
pub static mut ixheaacd_ppb_api_non_fatal: [pWORD8; 28] = [
    b"No Error\0" as *const u8 as *const core::ffi::c_char as pWORD8,
    b"API Command not supported\0" as *const u8 as *const core::ffi::c_char as pWORD8,
    b"API Command type not supported\0" as *const u8 as *const core::ffi::c_char
        as pWORD8,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
];
#[no_mangle]
pub static mut ixheaacd_ppb_api_fatal: [pWORD8; 28] = [
    b"Invalid Memory Table Index\0" as *const u8 as *const core::ffi::c_char as pWORD8,
    b"Invalid Library ID String Index\0" as *const u8 as *const core::ffi::c_char
        as pWORD8,
    b"NULL Pointer: Memory Allocation Error\0" as *const u8 as *const core::ffi::c_char
        as pWORD8,
    b"Invalid Config Param\0" as *const u8 as *const core::ffi::c_char as pWORD8,
    b"Invalid Execute type\0" as *const u8 as *const core::ffi::c_char as pWORD8,
    b"Invalid Command\0" as *const u8 as *const core::ffi::c_char as pWORD8,
    b"Memory Allocation Error: Alignment requirement not met\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
];
#[no_mangle]
pub static mut ixheaacd_ppb_config_non_fatal: [pWORD8; 28] = [
    b"Invalid Output PCM WORD Size. Setting to default, 16 \0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Invalid Down-mix flag option. Setting to default, 0 \0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Invalid eSBR PS flag option. Setting to default, 0 \0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Invalid downmix to stereo flag option. Setting to default, 0 \0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Invalid interleave to stereo flag option. Setting to default, 1 \0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Invalid downsample flag option. Setting to default, 0 \0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Invalid peak limiter flag option. Setting to default, 1 \0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Invalid MP4 flag option. Setting to default, 0 \0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Invalid maximum number of channels. limiting to between 2 and 8\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Invalid instance for coupling channel. Setting to default 1\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Invalid error concealment flag option. Setting to default 0\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Invalid Disable Sync flag option. Setting to default, 0 \0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Invalid Auto SBR upsample option. Setting to default, 1 \0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Invalid DRC heavy compression flag option. Setting to default 0\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Invalid DRC cut value\0" as *const u8 as *const core::ffi::c_char as pWORD8,
    b"Invalid DRC boost value\0" as *const u8 as *const core::ffi::c_char as pWORD8,
    b"Invalid DRC target\0" as *const u8 as *const core::ffi::c_char as pWORD8,
    b"Invalid Frame size\0" as *const u8 as *const core::ffi::c_char as pWORD8,
    b"Invalid LD testing flag option. Setting to default 0\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Invalid effect type\0" as *const u8 as *const core::ffi::c_char as pWORD8,
    b"Invalid target loudness value\0" as *const u8 as *const core::ffi::c_char
        as pWORD8,
    b"Invalid HQ eSBR flag option. Setting to default 0\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Invalid frame length flag option. Setting to default 0\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Invalid eSBR flag option. Setting to default 1\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Invalid Loudness leveling flag option. Setting to default 1\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
];
#[no_mangle]
pub static mut ixheaacd_ppb_config_fatal: [pWORD8; 28] = [
    b"Invalid Sample rate specified for RAW decoding\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
];
#[no_mangle]
pub static mut ixheaacd_ppb_init_non_fatal: [pWORD8; 28] = [
    b"Header not found at the beginning of input data continuing syncing\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Invalid number of QMF bands\0" as *const u8 as *const core::ffi::c_char as pWORD8,
    b"Decoder initialization failed\0" as *const u8 as *const core::ffi::c_char
        as pWORD8,
    b"Input bytes insufficient for decoding\0" as *const u8 as *const core::ffi::c_char
        as pWORD8,
    b"Error in AAC decoding\0" as *const u8 as *const core::ffi::c_char as pWORD8,
    b"DRC instruction count exceeded\0" as *const u8 as *const core::ffi::c_char
        as pWORD8,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
];
#[no_mangle]
pub static mut ixheaacd_ppb_init_fatal: [pWORD8; 28] = [
    b"AAC Decoder initialization failed\0" as *const u8 as *const core::ffi::c_char
        as pWORD8,
    b"End of input reached during initialization\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"No. of channels in stream greater than max channels defined\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Audio object type is not supported\0" as *const u8 as *const core::ffi::c_char
        as pWORD8,
    b"Decoder initialization failed\0" as *const u8 as *const core::ffi::c_char
        as pWORD8,
    b"Channel coupling not supported\0" as *const u8 as *const core::ffi::c_char
        as pWORD8,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
];
#[no_mangle]
pub static mut ixheaacd_ppb_exe_non_fatal: [pWORD8; 28] = [
    b"ADTS syncronization is lost. Re-syncing\0" as *const u8 as *const core::ffi::c_char
        as pWORD8,
    b"Though SBR was present in previous frame, not present in currentframe (SBR turned off)\0"
        as *const u8 as *const core::ffi::c_char as pWORD8,
    b"SBR was not present in previous frame, but it is present incurrent frame (SBR turned on)\0"
        as *const u8 as *const core::ffi::c_char as pWORD8,
    b"ADTS Header CRC failed.Re-syncing\0" as *const u8 as *const core::ffi::c_char
        as pWORD8,
    b"Input bytes insufficient for decoding\0" as *const u8 as *const core::ffi::c_char
        as pWORD8,
    b"Element instance tag mismatch, because of new channel mode\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"max huffman decoded value exceeded\0" as *const u8 as *const core::ffi::c_char
        as pWORD8,
    b"Error in AAC decoding\0" as *const u8 as *const core::ffi::c_char as pWORD8,
    b"Scale factor exceeds the transmitted boundary\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Gain control not supported\0" as *const u8 as *const core::ffi::c_char as pWORD8,
    b"Filter Order of TNS data is greater than maximum order\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"LTP data found, not supported\0" as *const u8 as *const core::ffi::c_char
        as pWORD8,
    b"The base sampling frequency has changed in ADTS header\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Pulse Data exceeds the permitted boundary\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Invalid code ixheaacd_book number in ia_huffman_data_type decoding\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Channel index not within allowed range\0" as *const u8 as *const core::ffi::c_char
        as pWORD8,
    b"Smoothing mode not within allowed range\0" as *const u8 as *const core::ffi::c_char
        as pWORD8,
    b"Smoothing time not within allowed range\0" as *const u8 as *const core::ffi::c_char
        as pWORD8,
    b"Extension type in the bitstream not within allowed range\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"QMF update type in the bitstream not within allowed range\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Window type in the bitstream not within allowed range\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Evaluated sine parameter not within allowed range\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
];
#[no_mangle]
pub static mut ixheaacd_ppb_exe_fatal: [pWORD8; 28] = [
    b"Channel coupling not supported\0" as *const u8 as *const core::ffi::c_char
        as pWORD8,
    b"TNS data range is errorneous\0" as *const u8 as *const core::ffi::c_char as pWORD8,
    b"Invalid LOAS header\0" as *const u8 as *const core::ffi::c_char as pWORD8,
    b"Invalid DRC data\0" as *const u8 as *const core::ffi::c_char as pWORD8,
    b"MPS reshaping input not valid\0" as *const u8 as *const core::ffi::c_char
        as pWORD8,
    b"Tree config present in bit stream not valid\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Number of timeslots not valid\0" as *const u8 as *const core::ffi::c_char
        as pWORD8,
    b"MPS dequantization parameter not valid\0" as *const u8 as *const core::ffi::c_char
        as pWORD8,
    b"MPS quantization mode not valid\0" as *const u8 as *const core::ffi::c_char
        as pWORD8,
    b"MPS input channels not valid\0" as *const u8 as *const core::ffi::c_char as pWORD8,
    b"Bitstream data in arbitrary downmix spatial frame not valid\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Window sequence value not valid\0" as *const u8 as *const core::ffi::c_char
        as pWORD8,
    b"Temporal shape config in the bitstream not valid\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"3D audio HRTF set present in the bitstream not valid\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"TTT mode read from the bitstream not valid\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Number of OTT boxes in the bitstream not valid\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Number of parameter sets present in the bitstream not valid\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Mapping of index data failed during decoding\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    b"Number of parameter bands present in the bitstream not valid\0" as *const u8
        as *const core::ffi::c_char as pWORD8,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
];
#[no_mangle]
pub static mut ixheaacd_error_info: ia_error_info_struct = {
    let mut init = ia_error_info_struct {
        pb_module_name: b"Ittiam xheaac_dec\0" as *const u8 as *const core::ffi::c_char
            as pWORD8,
        ppb_class_names: [
            b"API\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"Configuration\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"Initialization\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"Execution\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"xHeaac\0" as *const u8 as *const core::ffi::c_char as pWORD8,
        ],
        ppppb_error_msg_pointers: [
            [
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
            ],
            [
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
            ],
        ],
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_error_handler_init() -> VOID {
    ixheaacd_error_info
        .ppppb_error_msg_pointers[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int as usize] = ixheaacd_ppb_api_non_fatal
        .as_mut_ptr() as *mut *mut WORD8;
    ixheaacd_error_info
        .ppppb_error_msg_pointers[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int as usize] = ixheaacd_ppb_api_fatal.as_mut_ptr()
        as *mut *mut WORD8;
    ixheaacd_error_info
        .ppppb_error_msg_pointers[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int as usize] = ixheaacd_ppb_config_non_fatal
        .as_mut_ptr() as *mut *mut WORD8;
    ixheaacd_error_info
        .ppppb_error_msg_pointers[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int as usize] = ixheaacd_ppb_config_fatal
        .as_mut_ptr() as *mut *mut WORD8;
    ixheaacd_error_info
        .ppppb_error_msg_pointers[0 as core::ffi::c_int
        as usize][2 as core::ffi::c_int as usize] = ixheaacd_ppb_init_non_fatal
        .as_mut_ptr() as *mut *mut WORD8;
    ixheaacd_error_info
        .ppppb_error_msg_pointers[1 as core::ffi::c_int
        as usize][2 as core::ffi::c_int as usize] = ixheaacd_ppb_init_fatal.as_mut_ptr()
        as *mut *mut WORD8;
    ixheaacd_error_info
        .ppppb_error_msg_pointers[0 as core::ffi::c_int
        as usize][3 as core::ffi::c_int as usize] = ixheaacd_ppb_exe_non_fatal
        .as_mut_ptr() as *mut *mut WORD8;
    ixheaacd_error_info
        .ppppb_error_msg_pointers[1 as core::ffi::c_int
        as usize][3 as core::ffi::c_int as usize] = ixheaacd_ppb_exe_fatal.as_mut_ptr()
        as *mut *mut WORD8;
}
#[no_mangle]
pub static mut ixheaacd_ppb_ia_testbench_mem_file_man_fatal: [pWORD8; 28] = [
    b"Memory Allocation Error\0" as *const u8 as *const core::ffi::c_char as pWORD8,
    b"File Open Failed\0" as *const u8 as *const core::ffi::c_char as pWORD8,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
    0 as *const core::ffi::c_schar as *mut core::ffi::c_schar,
];
#[no_mangle]
pub static mut ixheaacd_ia_testbench_error_info: ia_error_info_struct = {
    let mut init = ia_error_info_struct {
        pb_module_name: b"ia_testbench\0" as *const u8 as *const core::ffi::c_char
            as pWORD8,
        ppb_class_names: [
            b"Memory & File Manager\0" as *const u8 as *const core::ffi::c_char
                as pWORD8,
            b"\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"\0" as *const u8 as *const core::ffi::c_char as pWORD8,
            b"\0" as *const u8 as *const core::ffi::c_char as pWORD8,
        ],
        ppppb_error_msg_pointers: [
            [
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
            ],
            [
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
                0 as *const *mut WORD8 as *mut *mut WORD8,
            ],
        ],
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn ia_testbench_error_handler_init() -> VOID {
    ixheaacd_ia_testbench_error_info
        .ppppb_error_msg_pointers[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int as usize] = ixheaacd_ppb_ia_testbench_mem_file_man_fatal
        .as_mut_ptr() as *mut *mut WORD8;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_error_handler(
    mut p_mod_err_info: *mut ia_error_info_struct,
    mut pb_context: *mut WORD8,
    mut code: IA_ERRORCODE,
) -> IA_ERRORCODE {
    if code == IA_NO_ERROR {
        return IA_NO_ERROR;
    }
    let mut is_fatal: WORD = 0;
    let mut err_class: WORD = 0;
    let mut err_sub_code: WORD = 0;
    if code as core::ffi::c_uint == IA_FATAL_ERROR {
        is_fatal = 1 as core::ffi::c_int as WORD;
    } else {
        is_fatal = ((code as UWORD & 0x8000 as UWORD) >> 15 as core::ffi::c_int) as WORD;
    }
    err_class = ((code as UWORD & 0x7800 as UWORD) >> 11 as core::ffi::c_int) as WORD;
    err_sub_code = (code as UWORD & 0x7ff as UWORD) as WORD;
    if is_fatal == 0 {
        printf(b"non \0" as *const u8 as *const core::ffi::c_char);
    }
    printf(b"fatal error: \0" as *const u8 as *const core::ffi::c_char);
    if !((*p_mod_err_info).pb_module_name).is_null() {
        printf(
            b"%s: \0" as *const u8 as *const core::ffi::c_char,
            (*p_mod_err_info).pb_module_name,
        );
    }
    if !((*p_mod_err_info).ppb_class_names[err_class as usize]).is_null() {
        printf(
            b"%s: \0" as *const u8 as *const core::ffi::c_char,
            (*p_mod_err_info).ppb_class_names[err_class as usize],
        );
    }
    if !pb_context.is_null() {
        printf(b"%s: \0" as *const u8 as *const core::ffi::c_char, pb_context);
    }
    if err_sub_code >= IA_MAX_ERROR_SUB_CODE
        || (*((*p_mod_err_info)
            .ppppb_error_msg_pointers[is_fatal as usize][err_class as usize])
            .offset(err_sub_code as isize))
            .is_null()
    {
        printf(b"error unlisted\0" as *const u8 as *const core::ffi::c_char);
    } else {
        printf(
            b"%s\n\0" as *const u8 as *const core::ffi::c_char,
            *((*p_mod_err_info)
                .ppppb_error_msg_pointers[is_fatal as usize][err_class as usize])
                .offset(err_sub_code as isize),
        );
    }
    return IA_NO_ERROR;
}
