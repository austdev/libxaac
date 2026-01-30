extern "C" {
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn longjmp(__env: *mut __jmp_buf_tag, __val: core::ffi::c_int) -> !;
    fn ixheaacd_read_bits_buf(
        it_bit_buff: *mut ia_bit_buf_struct,
        no_of_bits: WORD,
    ) -> WORD32;
}
pub type size_t = usize;
pub type WORD8 = core::ffi::c_schar;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD16 = core::ffi::c_short;
pub type UWORD16 = core::ffi::c_ushort;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type VOID = ();
pub type WORD = core::ffi::c_int;
pub type IA_ERRORCODE = WORD32;
pub type __jmp_buf = [core::ffi::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [core::ffi::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: core::ffi::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_crc_bit_buf_struct {
    pub ptr_bit_buf_base: *mut UWORD8,
    pub ptr_bit_buf_end: *mut UWORD8,
    pub ptr_read_next: *mut UWORD8,
    pub bit_pos: WORD16,
    pub cnt_bits: WORD32,
    pub size: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_crc_reg_data_struct {
    pub active: UWORD8,
    pub buf_size: WORD32,
    pub max_bits: WORD32,
    pub bit_cnt: UWORD32,
    pub bit_buf_cnt: WORD32,
    pub str_bit_buf: ia_crc_bit_buf_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_adts_crc_info_struct {
    pub crc_active: UWORD8,
    pub no_reg: UWORD16,
    pub file_value: UWORD16,
    pub crc_lookup: [UWORD16; 256],
    pub str_crc_reg_data: [ia_crc_reg_data_struct; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_bit_buf_struct {
    pub ptr_bit_buf_base: *mut UWORD8,
    pub ptr_bit_buf_end: *mut UWORD8,
    pub ptr_read_next: *mut UWORD8,
    pub bit_pos: WORD32,
    pub cnt_bits: WORD32,
    pub size: WORD32,
    pub adts_header_present: WORD32,
    pub crc_check: WORD32,
    pub protection_absent: WORD8,
    pub no_raw_data_blocks: WORD8,
    pub str_adts_crc_info: ia_adts_crc_info_struct,
    pub pstr_adts_crc_info: *mut ia_adts_crc_info_struct,
    pub initial_cnt_bits: WORD32,
    pub audio_mux_align: WORD32,
    pub bit_count: WORD32,
    pub valid_bits: WORD32,
    pub byte: UWORD8,
    pub byte_ptr: *mut UWORD8,
    pub ptr_start: *mut UWORD8,
    pub write_bit_count: WORD32,
    pub max_size: WORD32,
    pub xaac_jmp_buf: *mut jmp_buf,
}
pub type UINT32 = UWORD32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_usac_dec_sbr_config_struct {
    pub harmonic_sbr: UINT32,
    pub bs_inter_tes: UINT32,
    pub bs_pvc: UINT32,
    pub dflt_start_freq: WORD16,
    pub dflt_stop_freq: WORD16,
    pub dflt_header_extra1: WORD16,
    pub dflt_header_extra2: WORD16,
    pub dflt_freq_scale: WORD16,
    pub dflt_alter_scale: WORD16,
    pub dflt_noise_bands: WORD16,
    pub dflt_limiter_bands: WORD16,
    pub dflt_limiter_gains: WORD16,
    pub dflt_interpol_freq: WORD16,
    pub dflt_smoothing_mode: WORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_usac_dec_mps_config_struct {
    pub bs_freq_res: UINT32,
    pub bs_fixed_gain_dmx: UINT32,
    pub bs_temp_shape_config: UINT32,
    pub bs_decorr_config: UINT32,
    pub bs_high_rate_mode: UINT32,
    pub bs_phase_coding: UINT32,
    pub bs_ott_bands_phase_present: UINT32,
    pub bs_ott_bands_phase: UINT32,
    pub bs_residual_bands: UINT32,
    pub bs_pseudo_lr: UINT32,
    pub bs_env_quant_mode: UINT32,
    pub ldmps_present_flag: UINT32,
    pub bs_sampling_freq_index: UINT32,
    pub bs_fampling_frequency: UINT32,
    pub bs_frame_length: UINT32,
    pub bs_tree_config: UINT32,
    pub bs_quant_mode: UINT32,
    pub bs_one_icc: UINT32,
    pub bs_arbitrary_downmix: UINT32,
    pub bs_residual_coding: UINT32,
    pub bs_fixed_gain_sur: UINT32,
    pub bs_fixed_gain_LFE: UINT32,
    pub bs_matrix_mode: UINT32,
    pub bs_3D_audio_mode: UINT32,
    pub bs_3D_audio_HRTF_set: UINT32,
    pub bs_HRTF_freq_res: UINT32,
    pub HRTF_num_band: UINT32,
    pub HRTF_num_phase: UINT32,
    pub bs_HRTF_num_chan: UINT32,
    pub bs_HRTF_asymmetric: UINT32,
    pub bs_HRTF_level_left: [[UINT32; 28]; 7],
    pub bs_HRTF_level_right: [[UINT32; 28]; 7],
    pub bs_HRTF_phase: [UINT32; 7],
    pub bs_HRTF_phase_LR: [[UINT32; 28]; 7],
    pub bs_HRTF_icc: [UINT32; 7],
    pub bs_HRTF_icc_LR: [[UINT32; 28]; 7],
    pub bs_ott_bands: [UINT32; 5],
    pub bs_ttt_dual_mode: [UINT32; 1],
    pub bs_ttt_mode_low: [UINT32; 1],
    pub bs_ttt_mode_high: [UINT32; 1],
    pub bs_ttt_bands_low: [UINT32; 1],
    pub bs_ttt_bands_high: [UINT32; 1],
    pub bs_sac_ext_type: [UINT32; 8],
    pub sac_ext_cnt: UINT32,
    pub bs_residual_present: [UINT32; 3],
    pub bs_residual_sampling_freq_index: UINT32,
    pub bs_residual_frames_per_spatial_frame: UINT32,
    pub bs_residual_bands_ld_mps: [UINT32; 3],
    pub bs_arbitrary_downmix_residual_sampling_freq_index: UINT32,
    pub bs_arbitrary_downmix_residual_frames_per_spatial_frame: UINT32,
    pub bs_arbitrary_downmix_residual_bands: WORD32,
    pub num_out_chan_AT: UINT32,
    pub num_ott_boxes_AT: UINT32,
    pub bs_output_channel_pos_AT: [UINT32; 28],
    pub bs_ott_box_present_AT: [[UINT32; 7]; 7],
    pub bs_ott_default_cld_AT: [UINT32; 49],
    pub bs_ott_mode_lfe_AT: [UINT32; 49],
    pub bs_ott_bands_AT: [UINT32; 49],
    pub num_ott_boxes: WORD32,
    pub num_ttt_boxes: WORD32,
    pub num_input_channels: WORD32,
    pub num_output_channels: WORD32,
    pub ott_mode_lfe: [WORD32; 5],
    pub no_ldsbr_present: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_usac_dec_element_config_struct {
    pub tw_mdct: UINT32,
    pub noise_filling: UINT32,
    pub stereo_config_index: UINT32,
    pub usac_ext_eleme_def_len: UINT32,
    pub usac_ext_elem_pld_frag: UINT32,
    pub str_usac_sbr_config: ia_usac_dec_sbr_config_struct,
    pub str_usac_mps212_config: ia_usac_dec_mps_config_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_usac_decoder_config_struct {
    pub num_elements: UWORD32,
    pub num_config_extensions: UWORD32,
    pub usac_element_type: [UWORD32; 16],
    pub str_usac_element_config: [ia_usac_dec_element_config_struct; 16],
    pub usac_cfg_ext_info_present: [WORD32; 16],
    pub usac_ext_ele_payload_present: [WORD32; 16],
    pub usac_cfg_ext_info_len: [WORD32; 16],
    pub usac_ext_ele_payload_len: [WORD32; 16],
    pub usac_ext_gain_payload_len: [WORD32; 5],
    pub usac_cfg_ext_info_buf: [[UWORD8; 768]; 16],
    pub usac_ext_ele_payload_buf: [[UWORD8; 768]; 16],
    pub usac_ext_gain_payload_buf: [UWORD8; 2304],
    pub preroll_bytes: [UWORD32; 5],
    pub preroll_counter: WORD32,
    pub preroll_flag: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_usac_config_struct {
    pub usac_sampling_frequency_index: UINT32,
    pub usac_sampling_frequency: UINT32,
    pub core_sbr_framelength_index: UINT32,
    pub channel_configuration_index: UINT32,
    pub num_out_channels: UINT32,
    pub output_channel_pos: [UINT32; 255],
    pub str_usac_dec_config: ia_usac_decoder_config_struct,
}
pub const USAC_MAX_SAMPLE_RATE: core::ffi::c_int = 96000 as core::ffi::c_int;
pub const USAC_MAX_ELEMENTS: core::ffi::c_int = 16 as core::ffi::c_int;
pub const USAC_MAX_CONFIG_EXTENSIONS: core::ffi::c_int = 16 as core::ffi::c_int;
pub const ID_USAC_SCE: UWORD32 = 0 as UWORD32;
pub const ID_USAC_CPE: UWORD32 = 1 as UWORD32;
pub const ID_USAC_LFE: UWORD32 = 2 as UWORD32;
pub const ID_USAC_EXT: UWORD32 = 3 as UWORD32;
pub const ID_USAC_INVALID: core::ffi::c_int = 0xff as core::ffi::c_int;
pub const USAC_SBR_RATIO_NO_SBR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const USAC_SBR_RATIO_INDEX_2_1: core::ffi::c_int = 1 as core::ffi::c_int;
pub const USAC_SBR_RATIO_INDEX_8_3: core::ffi::c_int = 2 as core::ffi::c_int;
pub const USAC_SBR_RATIO_INDEX_4_1: core::ffi::c_int = 3 as core::ffi::c_int;
pub const USAC_OUT_FRAMELENGTH_768: core::ffi::c_int = 768 as core::ffi::c_int;
pub const USAC_OUT_FRAMELENGTH_1024: core::ffi::c_int = 1024 as core::ffi::c_int;
pub const USAC_OUT_FRAMELENGTH_2048: core::ffi::c_int = 2048 as core::ffi::c_int;
pub const USAC_OUT_FRAMELENGTH_4096: core::ffi::c_int = 4096 as core::ffi::c_int;
pub const MAX_CORE_SBR_FRAME_LEN_IDX: core::ffi::c_int = 4 as core::ffi::c_int;
pub const ID_EXT_ELE_FILL: UWORD32 = 0 as UWORD32;
pub const ID_EXT_ELE_AUDIOPREROLL: UWORD32 = 3 as UWORD32;
pub const ID_EXT_ELE_UNI_DRC: UWORD32 = 4 as UWORD32;
pub const ID_CONFIG_EXT_FILL: UWORD32 = 0 as UWORD32;
pub const ID_CONFIG_EXT_LOUDNESS_INFO: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_PARAMETER_BANDS: core::ffi::c_int = 28 as core::ffi::c_int;
pub const BS_OUTPUT_CHANNEL_POS_NA: core::ffi::c_int = -(1 as core::ffi::c_int);
pub const BS_OUTPUT_CHANNEL_POS_L: core::ffi::c_int = 0 as core::ffi::c_int;
pub const BS_OUTPUT_CHANNEL_POS_R: core::ffi::c_int = 1 as core::ffi::c_int;
pub const BS_OUTPUT_CHANNEL_POS_C: core::ffi::c_int = 2 as core::ffi::c_int;
pub const BS_MAX_NUM_OUT_CHANNELS: core::ffi::c_int = 255 as core::ffi::c_int;
pub const MAX_DECOR_CONFIG_IDX: core::ffi::c_int = 2 as core::ffi::c_int;
pub const IA_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_FATAL_ERROR: core::ffi::c_uint = 0x80000000 as core::ffi::c_uint;
pub const IA_XHEAAC_DEC_INIT_FATAL_DEC_INIT_FAIL: core::ffi::c_uint = 0xffff9000
    as core::ffi::c_uint;
pub const IA_XHEAAC_DEC_INIT_FATAL_STREAM_CHAN_GT_MAX: core::ffi::c_uint = 0xffff9002
    as core::ffi::c_uint;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES: core::ffi::c_int = 0x1804
    as core::ffi::c_int;
static mut sampling_rate_tbl: [WORD32; 31] = [
    96000 as core::ffi::c_int,
    88200 as core::ffi::c_int,
    64000 as core::ffi::c_int,
    48000 as core::ffi::c_int,
    44100 as core::ffi::c_int,
    32000 as core::ffi::c_int,
    24000 as core::ffi::c_int,
    22050 as core::ffi::c_int,
    16000 as core::ffi::c_int,
    12000 as core::ffi::c_int,
    11025 as core::ffi::c_int,
    8000 as core::ffi::c_int,
    7350 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    57600 as core::ffi::c_int,
    51200 as core::ffi::c_int,
    40000 as core::ffi::c_int,
    38400 as core::ffi::c_int,
    34150 as core::ffi::c_int,
    28800 as core::ffi::c_int,
    25600 as core::ffi::c_int,
    20000 as core::ffi::c_int,
    19200 as core::ffi::c_int,
    17075 as core::ffi::c_int,
    14400 as core::ffi::c_int,
    12800 as core::ffi::c_int,
    9600 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
];
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_sbr_ratio(
    mut core_sbr_framelength_idx: UWORD32,
) -> UWORD32 {
    let mut sbr_ratio_index: UWORD32 = 0xff as UWORD32;
    match core_sbr_framelength_idx {
        0 | 1 => {
            sbr_ratio_index = USAC_SBR_RATIO_NO_SBR as UWORD32;
        }
        2 => {
            sbr_ratio_index = USAC_SBR_RATIO_INDEX_8_3 as UWORD32;
        }
        3 => {
            sbr_ratio_index = USAC_SBR_RATIO_INDEX_2_1 as UWORD32;
        }
        4 => {
            sbr_ratio_index = USAC_SBR_RATIO_INDEX_4_1 as UWORD32;
        }
        _ => {}
    }
    return sbr_ratio_index;
}
unsafe extern "C" fn ixheaacd_get_sample_freq_indx(mut sampling_freq: WORD32) -> WORD32 {
    let mut index: WORD32 = 0;
    let tbl_size: WORD32 = (::core::mem::size_of::<[WORD32; 31]>() as usize)
        .wrapping_div(::core::mem::size_of::<WORD32>() as usize)
        .wrapping_sub(1 as usize) as WORD32;
    index = 0 as core::ffi::c_int as WORD32;
    while index < tbl_size {
        if sampling_rate_tbl[index as usize] == sampling_freq {
            break;
        }
        index += 1;
    }
    return index;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_sbr_params(
    mut core_sbr_framelength_idx: UWORD32,
    mut output_framelength: *mut WORD32,
    mut block_size: *mut WORD32,
    mut output_samples: *mut WORD32,
    mut sample_rate_layer: *mut WORD32,
    mut sample_freq_indx: *mut UWORD32,
) -> UWORD32 {
    let mut sbr_ratio_index: UWORD32 = 0xff as UWORD32;
    *output_framelength = -(1 as core::ffi::c_int) as WORD32;
    match core_sbr_framelength_idx {
        0 => {
            sbr_ratio_index = USAC_SBR_RATIO_NO_SBR as UWORD32;
            *output_framelength = USAC_OUT_FRAMELENGTH_768 as WORD32;
            *block_size = 768 as core::ffi::c_int as WORD32;
            *output_samples = *block_size;
        }
        1 => {
            sbr_ratio_index = USAC_SBR_RATIO_NO_SBR as UWORD32;
            *output_framelength = USAC_OUT_FRAMELENGTH_1024 as WORD32;
            *block_size = 1024 as core::ffi::c_int as WORD32;
            *output_samples = *block_size;
        }
        2 => {
            sbr_ratio_index = USAC_SBR_RATIO_INDEX_8_3 as UWORD32;
            *output_framelength = USAC_OUT_FRAMELENGTH_2048 as WORD32;
            *block_size = 768 as core::ffi::c_int as WORD32;
            *output_samples = (*block_size * 8 as core::ffi::c_int
                / 3 as core::ffi::c_int) as WORD32;
            *sample_rate_layer = (*sample_rate_layer * 3 as core::ffi::c_int
                >> 3 as core::ffi::c_int) as WORD32;
        }
        3 => {
            sbr_ratio_index = USAC_SBR_RATIO_INDEX_2_1 as UWORD32;
            *output_framelength = USAC_OUT_FRAMELENGTH_2048 as WORD32;
            *block_size = 1024 as core::ffi::c_int as WORD32;
            *output_samples = (*block_size * 2 as core::ffi::c_int) as WORD32;
            *sample_rate_layer = *sample_rate_layer >> 1 as core::ffi::c_int;
        }
        4 => {
            sbr_ratio_index = USAC_SBR_RATIO_INDEX_4_1 as UWORD32;
            *output_framelength = USAC_OUT_FRAMELENGTH_4096 as WORD32;
            *block_size = 1024 as core::ffi::c_int as WORD32;
            *output_samples = (*block_size * 4 as core::ffi::c_int) as WORD32;
            *sample_rate_layer = *sample_rate_layer >> 2 as core::ffi::c_int;
        }
        _ => {}
    }
    *sample_freq_indx = ixheaacd_get_sample_freq_indx(*sample_rate_layer) as UWORD32;
    return sbr_ratio_index;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_read_escape_value(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut ext_ele_value: *mut UWORD32,
    mut no_bits1: UWORD32,
    mut no_bits2: UWORD32,
    mut no_bits3: UWORD32,
) -> VOID {
    let mut value: UWORD32 = 0 as UWORD32;
    let mut val_add: UWORD32 = 0 as UWORD32;
    let mut max_val1: UWORD32 = (((1 as core::ffi::c_int) << no_bits1)
        - 1 as core::ffi::c_int) as UWORD32;
    let mut max_val2: UWORD32 = (((1 as core::ffi::c_int) << no_bits2)
        - 1 as core::ffi::c_int) as UWORD32;
    value = ixheaacd_read_bits_buf(it_bit_buff, no_bits1 as WORD) as UWORD32;
    if value == max_val1 {
        val_add = ixheaacd_read_bits_buf(it_bit_buff, no_bits2 as WORD) as UWORD32;
        value = value.wrapping_add(val_add);
        if val_add == max_val2 {
            val_add = ixheaacd_read_bits_buf(it_bit_buff, no_bits3 as WORD) as UWORD32;
            value = value.wrapping_add(val_add);
        }
    }
    *ext_ele_value = value;
}
unsafe extern "C" fn ixheaacd_get_usac_chan_conf(
    mut pstr_usac_config: *mut ia_usac_config_struct,
    mut ch_config_index: UWORD32,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut ec_flag: WORD32,
) -> IA_ERRORCODE {
    match ch_config_index {
        1 => {
            (*pstr_usac_config).num_out_channels = 1 as UINT32;
            (*pstr_usac_config).output_channel_pos[0 as core::ffi::c_int as usize] = BS_OUTPUT_CHANNEL_POS_C
                as UINT32;
        }
        2 => {
            (*pstr_usac_config).num_out_channels = 2 as UINT32;
            (*pstr_usac_config).output_channel_pos[0 as core::ffi::c_int as usize] = BS_OUTPUT_CHANNEL_POS_L
                as UINT32;
            (*pstr_usac_config).output_channel_pos[1 as core::ffi::c_int as usize] = BS_OUTPUT_CHANNEL_POS_R
                as UINT32;
        }
        8 => {
            (*pstr_usac_config).num_out_channels = 2 as UINT32;
            (*pstr_usac_config).output_channel_pos[0 as core::ffi::c_int as usize] = BS_OUTPUT_CHANNEL_POS_NA
                as UINT32;
            (*pstr_usac_config).output_channel_pos[1 as core::ffi::c_int as usize] = BS_OUTPUT_CHANNEL_POS_NA
                as UINT32;
        }
        _ => {
            if ec_flag != 0 {
                longjmp(
                    (*(*it_bit_buff).xaac_jmp_buf).as_mut_ptr(),
                    IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES,
                );
            } else {
                return IA_FATAL_ERROR as IA_ERRORCODE
            }
        }
    }
    return IA_NO_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_sbr_config(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut pstr_usac_sbr_config: *mut ia_usac_dec_sbr_config_struct,
) -> VOID {
    (*pstr_usac_sbr_config).harmonic_sbr = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD)
        as UINT32;
    (*pstr_usac_sbr_config).bs_inter_tes = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD)
        as UINT32;
    (*pstr_usac_sbr_config).bs_pvc = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD)
        as UINT32;
    (*pstr_usac_sbr_config).dflt_start_freq = ixheaacd_read_bits_buf(
        it_bit_buff,
        4 as WORD,
    ) as WORD16;
    (*pstr_usac_sbr_config).dflt_stop_freq = ixheaacd_read_bits_buf(
        it_bit_buff,
        4 as WORD,
    ) as WORD16;
    (*pstr_usac_sbr_config).dflt_header_extra1 = ixheaacd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    ) as WORD16;
    (*pstr_usac_sbr_config).dflt_header_extra2 = ixheaacd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    ) as WORD16;
    if (*pstr_usac_sbr_config).dflt_header_extra1 != 0 {
        (*pstr_usac_sbr_config).dflt_freq_scale = ixheaacd_read_bits_buf(
            it_bit_buff,
            2 as WORD,
        ) as WORD16;
        (*pstr_usac_sbr_config).dflt_alter_scale = ixheaacd_read_bits_buf(
            it_bit_buff,
            1 as WORD,
        ) as WORD16;
        (*pstr_usac_sbr_config).dflt_noise_bands = ixheaacd_read_bits_buf(
            it_bit_buff,
            2 as WORD,
        ) as WORD16;
    }
    if (*pstr_usac_sbr_config).dflt_header_extra2 != 0 {
        (*pstr_usac_sbr_config).dflt_limiter_bands = ixheaacd_read_bits_buf(
            it_bit_buff,
            2 as WORD,
        ) as WORD16;
        (*pstr_usac_sbr_config).dflt_limiter_gains = ixheaacd_read_bits_buf(
            it_bit_buff,
            2 as WORD,
        ) as WORD16;
        (*pstr_usac_sbr_config).dflt_interpol_freq = ixheaacd_read_bits_buf(
            it_bit_buff,
            1 as WORD,
        ) as WORD16;
        (*pstr_usac_sbr_config).dflt_smoothing_mode = ixheaacd_read_bits_buf(
            it_bit_buff,
            1 as WORD,
        ) as WORD16;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_ext_element_config(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut pstr_usac_element_config: *mut ia_usac_dec_element_config_struct,
    mut ptr_usac_ext_ele_payload: *mut UWORD8,
    mut ptr_usac_ext_ele_payload_len: *mut WORD32,
    mut preroll_flag: *mut WORD32,
) -> WORD32 {
    let mut usac_ext_element_type: UWORD32 = 0;
    let mut usac_ext_element_config_length: UWORD32 = 0;
    let mut flag: UWORD32 = 0;
    let mut i: UWORD32 = 0;
    ixheaacd_read_escape_value(
        it_bit_buff,
        &mut usac_ext_element_type,
        4 as UWORD32,
        8 as UWORD32,
        16 as UWORD32,
    );
    ixheaacd_read_escape_value(
        it_bit_buff,
        &mut usac_ext_element_config_length,
        4 as UWORD32,
        8 as UWORD32,
        16 as UWORD32,
    );
    if usac_ext_element_config_length >= 768 as UWORD32 {
        return -(1 as WORD32);
    }
    flag = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD) as UWORD32;
    *ptr_usac_ext_ele_payload_len = 0 as core::ffi::c_int as WORD32;
    if flag != 0 {
        ixheaacd_read_escape_value(
            it_bit_buff,
            &mut (*pstr_usac_element_config).usac_ext_eleme_def_len as *mut UINT32
                as *mut UWORD32,
            8 as UWORD32,
            16 as UWORD32,
            0 as UWORD32,
        );
        (*pstr_usac_element_config).usac_ext_eleme_def_len = ((*pstr_usac_element_config)
            .usac_ext_eleme_def_len)
            .wrapping_add(1 as UINT32);
    } else {
        (*pstr_usac_element_config).usac_ext_eleme_def_len = 0 as UINT32;
    }
    (*pstr_usac_element_config).usac_ext_elem_pld_frag = ixheaacd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    ) as UINT32;
    match usac_ext_element_type {
        0 => {}
        3 => {
            *preroll_flag = 1 as core::ffi::c_int as WORD32;
        }
        4 => {
            i = 0 as UWORD32;
            while i < usac_ext_element_config_length {
                *ptr_usac_ext_ele_payload.offset(i as isize) = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    8 as WORD,
                ) as UWORD8;
                i = i.wrapping_add(1);
            }
            *ptr_usac_ext_ele_payload_len = usac_ext_element_config_length as WORD32;
        }
        _ => {
            if ((*it_bit_buff).cnt_bits >> 3 as core::ffi::c_int)
                < usac_ext_element_config_length as WORD32
            {
                return -(1 as WORD32);
            }
            (*it_bit_buff).ptr_read_next = ((*it_bit_buff).ptr_read_next)
                .offset(usac_ext_element_config_length as isize);
            (*it_bit_buff).cnt_bits = ((*it_bit_buff).cnt_bits as UWORD32)
                .wrapping_sub(usac_ext_element_config_length << 3 as core::ffi::c_int)
                as WORD32 as WORD32;
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps212_config(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut pstr_usac_mps212_config: *mut ia_usac_dec_mps_config_struct,
    mut stereo_config_index: WORD32,
) -> IA_ERRORCODE {
    (*pstr_usac_mps212_config).bs_freq_res = ixheaacd_read_bits_buf(
        it_bit_buff,
        3 as WORD,
    ) as UINT32;
    (*pstr_usac_mps212_config).bs_fixed_gain_dmx = ixheaacd_read_bits_buf(
        it_bit_buff,
        3 as WORD,
    ) as UINT32;
    (*pstr_usac_mps212_config).bs_temp_shape_config = ixheaacd_read_bits_buf(
        it_bit_buff,
        2 as WORD,
    ) as UINT32;
    (*pstr_usac_mps212_config).bs_decorr_config = ixheaacd_read_bits_buf(
        it_bit_buff,
        2 as WORD,
    ) as UINT32;
    if (*pstr_usac_mps212_config).bs_decorr_config > MAX_DECOR_CONFIG_IDX as UINT32 {
        return IA_FATAL_ERROR as IA_ERRORCODE;
    }
    (*pstr_usac_mps212_config).bs_high_rate_mode = ixheaacd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    ) as UINT32;
    (*pstr_usac_mps212_config).bs_phase_coding = ixheaacd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    ) as UINT32;
    (*pstr_usac_mps212_config).bs_ott_bands_phase_present = ixheaacd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    ) as UINT32;
    if (*pstr_usac_mps212_config).bs_ott_bands_phase_present != 0 {
        (*pstr_usac_mps212_config).bs_ott_bands_phase = ixheaacd_read_bits_buf(
            it_bit_buff,
            5 as WORD,
        ) as UINT32;
        if (*pstr_usac_mps212_config).bs_ott_bands_phase > MAX_PARAMETER_BANDS as UINT32
        {
            return IA_FATAL_ERROR as IA_ERRORCODE;
        }
    }
    if stereo_config_index > 1 as core::ffi::c_int {
        (*pstr_usac_mps212_config).bs_residual_bands = ixheaacd_read_bits_buf(
            it_bit_buff,
            5 as WORD,
        ) as UINT32;
        if (*pstr_usac_mps212_config).bs_residual_bands > MAX_PARAMETER_BANDS as UINT32 {
            return IA_FATAL_ERROR as IA_ERRORCODE;
        }
        (*pstr_usac_mps212_config).bs_ott_bands_phase = if (*pstr_usac_mps212_config)
            .bs_ott_bands_phase > (*pstr_usac_mps212_config).bs_residual_bands
        {
            (*pstr_usac_mps212_config).bs_ott_bands_phase
        } else {
            (*pstr_usac_mps212_config).bs_residual_bands
        };
        (*pstr_usac_mps212_config).bs_pseudo_lr = ixheaacd_read_bits_buf(
            it_bit_buff,
            1 as WORD,
        ) as UINT32;
    }
    if (*pstr_usac_mps212_config).bs_temp_shape_config == 2 as UINT32 {
        (*pstr_usac_mps212_config).bs_env_quant_mode = ixheaacd_read_bits_buf(
            it_bit_buff,
            1 as WORD,
        ) as UINT32;
    }
    return IA_NO_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_cpe_config(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut pstr_usac_element_config: *mut ia_usac_dec_element_config_struct,
    mut sbr_ratio_index: WORD32,
) -> IA_ERRORCODE {
    (*pstr_usac_element_config).tw_mdct = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD)
        as UINT32;
    (*pstr_usac_element_config).noise_filling = ixheaacd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    ) as UINT32;
    if sbr_ratio_index > 0 as core::ffi::c_int {
        ixheaacd_sbr_config(
            it_bit_buff,
            &mut (*pstr_usac_element_config).str_usac_sbr_config,
        );
        (*pstr_usac_element_config).stereo_config_index = ixheaacd_read_bits_buf(
            it_bit_buff,
            2 as WORD,
        ) as UINT32;
    } else {
        (*pstr_usac_element_config).stereo_config_index = 0 as UINT32;
    }
    if (*pstr_usac_element_config).stereo_config_index > 0 as UINT32 {
        return ixheaacd_mps212_config(
            it_bit_buff,
            &mut (*pstr_usac_element_config).str_usac_mps212_config,
            (*pstr_usac_element_config).stereo_config_index as WORD32,
        );
    }
    return IA_NO_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_decoder_config(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut pstr_usac_decoder_config: *mut ia_usac_decoder_config_struct,
    mut sbr_ratio_index: WORD32,
    mut chan: *mut UINT32,
    mut ec_flag: WORD32,
) -> WORD32 {
    let mut elem_idx: UWORD32 = 0 as UWORD32;
    let mut err: UWORD32 = 0 as UWORD32;
    let mut num_channels: WORD32 = 0 as WORD32;
    ixheaacd_read_escape_value(
        it_bit_buff,
        &mut (*pstr_usac_decoder_config).num_elements,
        4 as UWORD32,
        8 as UWORD32,
        16 as UWORD32,
    );
    (*pstr_usac_decoder_config).num_elements = ((*pstr_usac_decoder_config).num_elements)
        .wrapping_add(1 as UWORD32);
    (*pstr_usac_decoder_config).preroll_flag = 0 as core::ffi::c_int as WORD32;
    if (*pstr_usac_decoder_config).num_elements > USAC_MAX_ELEMENTS as UWORD32 {
        if ec_flag != 0 {
            (*pstr_usac_decoder_config).num_elements = USAC_MAX_ELEMENTS as UWORD32;
            longjmp(
                (*(*it_bit_buff).xaac_jmp_buf).as_mut_ptr(),
                IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES,
            );
        } else {
            return IA_FATAL_ERROR as WORD32
        }
    }
    elem_idx = 0 as UWORD32;
    while elem_idx < (*pstr_usac_decoder_config).num_elements {
        let mut pstr_usac_element_config: *mut ia_usac_dec_element_config_struct = &mut *((*pstr_usac_decoder_config)
            .str_usac_element_config)
            .as_mut_ptr()
            .offset(elem_idx as isize) as *mut ia_usac_dec_element_config_struct;
        (*pstr_usac_decoder_config).usac_element_type[elem_idx as usize] = ixheaacd_read_bits_buf(
            it_bit_buff,
            2 as WORD,
        ) as UWORD32;
        match (*pstr_usac_decoder_config).usac_element_type[elem_idx as usize] {
            0 => {
                num_channels += 1;
                (*pstr_usac_element_config).tw_mdct = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    1 as WORD,
                ) as UINT32;
                (*pstr_usac_element_config).noise_filling = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    1 as WORD,
                ) as UINT32;
                (*pstr_usac_element_config).stereo_config_index = 0 as UINT32;
                if sbr_ratio_index > 0 as core::ffi::c_int {
                    ixheaacd_sbr_config(
                        it_bit_buff,
                        &mut (*pstr_usac_element_config).str_usac_sbr_config,
                    );
                }
            }
            1 => {
                num_channels += 2 as core::ffi::c_int;
                if ixheaacd_cpe_config(
                    it_bit_buff,
                    pstr_usac_element_config,
                    sbr_ratio_index,
                ) != IA_NO_ERROR
                {
                    if ec_flag != 0 {
                        longjmp(
                            (*(*it_bit_buff).xaac_jmp_buf).as_mut_ptr(),
                            IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES,
                        );
                    } else {
                        return IA_FATAL_ERROR as WORD32
                    }
                }
                if (*pstr_usac_element_config).stereo_config_index > 1 as UINT32
                    && *chan < 2 as UINT32
                {
                    if ec_flag != 0 {
                        longjmp(
                            (*(*it_bit_buff).xaac_jmp_buf).as_mut_ptr(),
                            IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES,
                        );
                    } else {
                        return IA_FATAL_ERROR as WORD32
                    }
                }
            }
            2 => {
                num_channels += 1;
                (*pstr_usac_element_config).tw_mdct = 0 as UINT32;
                (*pstr_usac_element_config).noise_filling = 0 as UINT32;
                (*pstr_usac_element_config).stereo_config_index = 0 as UINT32;
            }
            3 => {
                err = ixheaacd_ext_element_config(
                    it_bit_buff,
                    pstr_usac_element_config,
                    &mut *(*((*pstr_usac_decoder_config).usac_ext_ele_payload_buf)
                        .as_mut_ptr()
                        .offset(elem_idx as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize),
                    &mut *((*pstr_usac_decoder_config).usac_ext_ele_payload_len)
                        .as_mut_ptr()
                        .offset(elem_idx as isize),
                    &mut (*pstr_usac_decoder_config).preroll_flag,
                ) as UWORD32;
                if (*pstr_usac_decoder_config)
                    .usac_ext_ele_payload_len[elem_idx as usize] > 0 as core::ffi::c_int
                {
                    (*pstr_usac_decoder_config)
                        .usac_ext_ele_payload_present[elem_idx as usize] = 1
                        as core::ffi::c_int as WORD32;
                } else {
                    (*pstr_usac_decoder_config)
                        .usac_ext_ele_payload_present[elem_idx as usize] = 0
                        as core::ffi::c_int as WORD32;
                }
                if err != 0 as UWORD32 {
                    if ec_flag != 0 {
                        longjmp(
                            (*(*it_bit_buff).xaac_jmp_buf).as_mut_ptr(),
                            IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES,
                        );
                    } else {
                        return IA_FATAL_ERROR as WORD32
                    }
                }
            }
            _ => {
                if ec_flag != 0 {
                    longjmp(
                        (*(*it_bit_buff).xaac_jmp_buf).as_mut_ptr(),
                        IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES,
                    );
                } else {
                    return IA_FATAL_ERROR as WORD32
                }
            }
        }
        if num_channels > 2 as core::ffi::c_int {
            if ec_flag != 0 {
                longjmp(
                    (*(*it_bit_buff).xaac_jmp_buf).as_mut_ptr(),
                    IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES,
                );
            } else {
                return IA_FATAL_ERROR as WORD32
            }
        }
        elem_idx = elem_idx.wrapping_add(1);
    }
    return err as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_config_extension(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut pstr_usac_decoder_config: *mut ia_usac_decoder_config_struct,
    mut ec_flag: WORD32,
) -> WORD32 {
    let mut i: UWORD32 = 0;
    let mut j: UWORD32 = 0;
    let mut num_config_extensions: UWORD32 = 0;
    let mut usac_config_ext_type: UWORD32 = 0;
    let mut usac_config_ext_len: UWORD32 = 0;
    ixheaacd_read_escape_value(
        it_bit_buff,
        &mut num_config_extensions,
        2 as UWORD32,
        4 as UWORD32,
        8 as UWORD32,
    );
    num_config_extensions = num_config_extensions.wrapping_add(1 as UWORD32);
    if (USAC_MAX_CONFIG_EXTENSIONS as UWORD32) < num_config_extensions {
        if ec_flag != 0 {
            longjmp(
                (*(*it_bit_buff).xaac_jmp_buf).as_mut_ptr(),
                IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES,
            );
        } else {
            return IA_FATAL_ERROR as WORD32
        }
    }
    (*pstr_usac_decoder_config).num_config_extensions = num_config_extensions;
    memset(
        ((*pstr_usac_decoder_config).usac_cfg_ext_info_len).as_mut_ptr()
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (USAC_MAX_CONFIG_EXTENSIONS as size_t)
            .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
    );
    memset(
        ((*pstr_usac_decoder_config).usac_cfg_ext_info_present).as_mut_ptr()
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (USAC_MAX_CONFIG_EXTENSIONS as size_t)
            .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
    );
    j = 0 as UWORD32;
    while j < num_config_extensions {
        let mut fill_byte_val: UWORD32 = 0xa5 as UWORD32;
        ixheaacd_read_escape_value(
            it_bit_buff,
            &mut usac_config_ext_type,
            4 as UWORD32,
            8 as UWORD32,
            16 as UWORD32,
        );
        ixheaacd_read_escape_value(
            it_bit_buff,
            &mut usac_config_ext_len,
            4 as UWORD32,
            8 as UWORD32,
            16 as UWORD32,
        );
        if usac_config_ext_len > 768 as UWORD32 {
            if ec_flag != 0 {
                longjmp(
                    (*(*it_bit_buff).xaac_jmp_buf).as_mut_ptr(),
                    IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES,
                );
            } else {
                return IA_FATAL_ERROR as WORD32
            }
        }
        match usac_config_ext_type {
            0 => {
                i = 0 as UWORD32;
                while i < usac_config_ext_len {
                    fill_byte_val = ixheaacd_read_bits_buf(it_bit_buff, 8 as WORD)
                        as UWORD32;
                    if fill_byte_val != 0xa5 as UWORD32 {
                        if ec_flag != 0 {
                            longjmp(
                                (*(*it_bit_buff).xaac_jmp_buf).as_mut_ptr(),
                                IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES,
                            );
                        } else {
                            return IA_FATAL_ERROR as WORD32
                        }
                    }
                    i = i.wrapping_add(1);
                }
            }
            _ => {
                if usac_config_ext_len as WORD32
                    > (*it_bit_buff).cnt_bits >> 3 as core::ffi::c_int
                {
                    if ec_flag != 0 {
                        longjmp(
                            (*(*it_bit_buff).xaac_jmp_buf).as_mut_ptr(),
                            IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES,
                        );
                    } else {
                        return IA_FATAL_ERROR as WORD32
                    }
                }
                if ID_CONFIG_EXT_LOUDNESS_INFO as UWORD32 == usac_config_ext_type {
                    i = 0 as UWORD32;
                    while i < usac_config_ext_len {
                        let mut byte_val: UWORD8 = ixheaacd_read_bits_buf(
                            it_bit_buff,
                            8 as WORD,
                        ) as UWORD8;
                        (*pstr_usac_decoder_config)
                            .usac_cfg_ext_info_buf[j as usize][i as usize] = byte_val;
                        i = i.wrapping_add(1);
                    }
                    (*pstr_usac_decoder_config).usac_cfg_ext_info_len[j as usize] = usac_config_ext_len
                        as WORD32;
                    (*pstr_usac_decoder_config).usac_cfg_ext_info_present[j as usize] = 1
                        as core::ffi::c_int as WORD32;
                } else {
                    i = 0 as UWORD32;
                    while i < usac_config_ext_len {
                        ixheaacd_read_bits_buf(it_bit_buff, 8 as WORD);
                        i = i.wrapping_add(1);
                    }
                }
            }
        }
        j = j.wrapping_add(1);
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_config(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut pstr_usac_conf: *mut ia_usac_config_struct,
    mut chan: *mut UINT32,
    mut ec_flag: WORD32,
) -> WORD32 {
    let mut tmp: WORD32 = 0;
    let mut err: WORD32 = 0;
    err = 0 as core::ffi::c_int as WORD32;
    (*pstr_usac_conf).usac_sampling_frequency_index = ixheaacd_read_bits_buf(
        it_bit_buff,
        5 as WORD,
    ) as UINT32;
    if (*pstr_usac_conf).usac_sampling_frequency_index == 0x1f as UINT32 {
        (*pstr_usac_conf).usac_sampling_frequency = ixheaacd_read_bits_buf(
            it_bit_buff,
            24 as WORD,
        ) as UINT32;
        if (*pstr_usac_conf).usac_sampling_frequency > USAC_MAX_SAMPLE_RATE as UINT32 {
            if ec_flag != 0 {
                longjmp(
                    (*(*it_bit_buff).xaac_jmp_buf).as_mut_ptr(),
                    IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES,
                );
            } else {
                return IA_FATAL_ERROR as WORD32
            }
        }
    } else {
        (*pstr_usac_conf).usac_sampling_frequency = sampling_rate_tbl[(*pstr_usac_conf)
            .usac_sampling_frequency_index as usize] as UINT32;
    }
    if (*pstr_usac_conf).usac_sampling_frequency == 0 as UINT32 {
        if ec_flag != 0 {
            longjmp(
                (*(*it_bit_buff).xaac_jmp_buf).as_mut_ptr(),
                IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES,
            );
        } else {
            return IA_FATAL_ERROR as WORD32
        }
    }
    (*pstr_usac_conf).core_sbr_framelength_index = ixheaacd_read_bits_buf(
        it_bit_buff,
        3 as WORD,
    ) as UINT32;
    if (*pstr_usac_conf).core_sbr_framelength_index
        > MAX_CORE_SBR_FRAME_LEN_IDX as UINT32
    {
        if ec_flag != 0 {
            longjmp(
                (*(*it_bit_buff).xaac_jmp_buf).as_mut_ptr(),
                IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES,
            );
        } else {
            return IA_FATAL_ERROR as WORD32
        }
    }
    (*pstr_usac_conf).channel_configuration_index = ixheaacd_read_bits_buf(
        it_bit_buff,
        5 as WORD,
    ) as UINT32;
    if (*pstr_usac_conf).channel_configuration_index >= 3 as UINT32
        && (*pstr_usac_conf).channel_configuration_index != 8 as UINT32
    {
        if ec_flag != 0 {
            longjmp(
                (*(*it_bit_buff).xaac_jmp_buf).as_mut_ptr(),
                IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES,
            );
        } else {
            return IA_FATAL_ERROR as WORD32
        }
    }
    if (*pstr_usac_conf).channel_configuration_index == 0 as UINT32 {
        let mut i: UWORD32 = 0;
        ixheaacd_read_escape_value(
            it_bit_buff,
            &mut (*pstr_usac_conf).num_out_channels as *mut UINT32 as *mut UWORD32,
            5 as UWORD32,
            8 as UWORD32,
            16 as UWORD32,
        );
        if (BS_MAX_NUM_OUT_CHANNELS as UINT32) < (*pstr_usac_conf).num_out_channels {
            return IA_XHEAAC_DEC_INIT_FATAL_STREAM_CHAN_GT_MAX as WORD32;
        }
        if (*pstr_usac_conf).num_out_channels < 1 as UINT32 {
            return IA_XHEAAC_DEC_INIT_FATAL_DEC_INIT_FAIL as WORD32;
        }
        i = 0 as UWORD32;
        while i < (*pstr_usac_conf).num_out_channels {
            (*pstr_usac_conf).output_channel_pos[i as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                5 as WORD,
            ) as UINT32;
            i = i.wrapping_add(1);
        }
        if ec_flag != 0 {
            longjmp(
                (*(*it_bit_buff).xaac_jmp_buf).as_mut_ptr(),
                IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES,
            );
        }
    } else {
        err = ixheaacd_get_usac_chan_conf(
            pstr_usac_conf,
            (*pstr_usac_conf).channel_configuration_index as UWORD32,
            it_bit_buff,
            ec_flag,
        ) as WORD32;
        if err != 0 as core::ffi::c_int {
            return err;
        }
    }
    err = ixheaacd_decoder_config(
        it_bit_buff,
        &mut (*pstr_usac_conf).str_usac_dec_config,
        ixheaacd_sbr_ratio((*pstr_usac_conf).core_sbr_framelength_index as UWORD32)
            as WORD32,
        chan,
        ec_flag,
    );
    if err != 0 as core::ffi::c_int {
        return err;
    }
    tmp = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
    if tmp != 0 {
        err = ixheaacd_config_extension(
            it_bit_buff,
            &mut (*pstr_usac_conf).str_usac_dec_config,
            ec_flag,
        );
        if err != 0 as core::ffi::c_int {
            return -(1 as WORD32);
        }
    }
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_conf_default(
    mut pstr_usac_conf: *mut ia_usac_config_struct,
) -> VOID {
    let mut i: WORD32 = 0;
    (*pstr_usac_conf).num_out_channels = 0 as UINT32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < BS_MAX_NUM_OUT_CHANNELS {
        (*pstr_usac_conf).output_channel_pos[i as usize] = BS_OUTPUT_CHANNEL_POS_NA
            as UINT32;
        i += 1;
    }
    (*pstr_usac_conf).str_usac_dec_config.num_elements = 0 as UWORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < USAC_MAX_ELEMENTS {
        (*pstr_usac_conf).str_usac_dec_config.usac_element_type[i as usize] = ID_USAC_INVALID
            as UWORD32;
        i += 1;
    }
}
