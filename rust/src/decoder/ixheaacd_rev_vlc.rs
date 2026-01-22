extern "C" {
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn ixheaacd_read_bits_buf(
        it_bit_buff: *mut ia_bit_buf_struct,
        no_of_bits: WORD,
    ) -> WORD32;
    fn ixheaacd_aac_read_bit_rev(it_bit_buff: *mut ia_bit_buf_struct) -> WORD32;
    fn ixheaacd_aac_read_bit(it_bit_buff: *mut ia_bit_buf_struct) -> WORD32;
}
pub type size_t = usize;
pub type WORD8 = core::ffi::c_schar;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD16 = core::ffi::c_short;
pub type UWORD16 = core::ffi::c_ushort;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type VOID = ();
pub type FLAG = core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_pulse_info_struct {
    pub pulse_data_present: FLAG,
    pub number_pulse: WORD16,
    pub pulse_start_band: WORD16,
    pub pulse_offset: [WORD8; 4],
    pub pulse_amp: [WORD8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_pns_correlation_info_struct {
    pub correlated: [UWORD8; 16],
    pub random_vector: [WORD32; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_pns_rand_vec_struct {
    pub current_seed: WORD32,
    pub pns_frame_number: WORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_pns_info_struct {
    pub pns_used: [UWORD8; 128],
    pub noise_energy: WORD16,
    pub pns_active: UWORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ltp_info {
    pub last_band: UWORD8,
    pub data_present: UWORD8,
    pub lag: UWORD16,
    pub lag_update: UWORD8,
    pub coef: UWORD8,
    pub long_used: [UWORD8; 51],
    pub short_used: [UWORD8; 8],
    pub short_lag_present: [UWORD8; 8],
    pub short_lag: [UWORD8; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_ics_info_struct {
    pub window_shape: WORD16,
    pub window_sequence: WORD16,
    pub max_sfb: WORD16,
    pub num_swb_window: WORD16,
    pub sampling_rate_index: WORD16,
    pub num_window_groups: WORD16,
    pub window_group_length: [WORD8; 8],
    pub frame_length: WORD16,
    pub frame_size: WORD32,
    pub predictor_data_present: WORD16,
    pub ltp: ltp_info,
    pub ltp2: ltp_info,
    pub qshift_adj: WORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_stereo_info_struct {
    pub ms_used: [[UWORD8; 64]; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_filter_info_struct {
    pub start_band: WORD16,
    pub stop_band: WORD16,
    pub direction: WORD8,
    pub resolution: WORD8,
    pub order: WORD8,
    pub coef: [WORD8; 31],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_tns_info_aac_struct {
    pub tns_data_present: FLAG,
    pub n_filt: [WORD8; 8],
    pub str_filter: [[ia_filter_info_struct; 3]; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_overlap_info {
    pub ptr_long_window: [*const WORD16; 2],
    pub ptr_short_window: [*const WORD16; 2],
    pub window_shape: WORD16,
    pub window_sequence: WORD16,
    pub ptr_overlap_buf: *mut WORD32,
    pub rvlc_prev_sf: [WORD16; 128],
    pub rvlc_prev_cb: [WORD16; 128],
    pub rvlc_prev_blk_type: WORD8,
    pub rvlc_prev_sf_ok: WORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_rvlc_info_struct {
    pub sf_concealment: WORD32,
    pub rev_global_gain: WORD32,
    pub rvlc_sf_len: WORD16,
    pub dpcm_noise_nrg: WORD32,
    pub sf_esc_present: WORD32,
    pub rvlc_esc_len: WORD16,
    pub dpcm_noise_last_pos: WORD32,
    pub dpcm_is_last_pos: WORD32,
    pub rvlc_sf_fwd_len: WORD16,
    pub rvlc_sf_bwd_len: WORD16,
    pub ptr_rvl_bit_cnt: *mut WORD16,
    pub ptr_rvl_bit_str_idx: *mut UWORD16,
    pub num_wind_grps: WORD16,
    pub max_sfb_transmitted: WORD16,
    pub first_noise_group: UWORD8,
    pub first_noise_band: UWORD8,
    pub direction: UWORD8,
    pub rvl_fwd_bit_str_idx: UWORD16,
    pub rvl_bwd_bit_str_idx: UWORD16,
    pub esc_bit_str_idx: UWORD16,
    pub ptr_huff_tree_rvl_cw: *const UWORD32,
    pub ptr_huff_tree_rvl_esc: *const UWORD32,
    pub num_fwd_esc_words_decoded: UWORD8,
    pub num_bwd_esc_words_decoded: UWORD8,
    pub num_esc_words_decoded: UWORD8,
    pub noise_used: WORD8,
    pub intensity_used: WORD8,
    pub sf_used: WORD8,
    pub firt_scale_fac: WORD16,
    pub last_scale_fac: WORD16,
    pub first_nrg: WORD16,
    pub last_nrg: WORD16,
    pub is_first: WORD16,
    pub is_last: WORD16,
    pub rvlc_err_log: UWORD32,
    pub conceal_min: WORD16,
    pub conceal_max: WORD16,
    pub conceal_min_esc: WORD16,
    pub conceal_max_esc: WORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_huff_code_reorder_io_struct {
    pub err_log: UWORD32,
    pub ptr_quant_spec_coeff_base: *mut WORD32,
    pub quant_spec_coeff_idx: WORD32,
    pub reordered_spec_data_len: WORD16,
    pub num_sect: WORD16,
    pub ptr_num_line_in_sect: *mut WORD16,
    pub bit_str_idx: UWORD16,
    pub longest_cw_len: WORD8,
    pub ptr_cb: *mut UWORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_huff_code_reorder_cb_pairs_struct {
    pub ptr_min_cb_pair_tbl: *const UWORD8,
    pub ptr_max_cb_pair_tbl: *const UWORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_huff_code_reorder_tbl_struct {
    pub ptr_lav_tbl: *const UWORD16,
    pub ptr_max_cw_len_tbl: *const UWORD8,
    pub ptr_cb_dimension_tbl: *const UWORD8,
    pub ptr_cb_dim_shift_tbl: *const UWORD8,
    pub ptr_cb_sign_tbl: *const UWORD8,
    pub ptr_cb_priority: *const UWORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_huff_code_reord_seg_info_struct {
    pub num_segment: WORD32,
    pub segment_offset: UWORD32,
    pub arr_temp_values: [WORD32; 1024],
    pub arr_seg_start_l: [UWORD16; 512],
    pub arr_seg_start_r: [UWORD16; 512],
    pub p_remaining_bits_in_seg: [WORD8; 512],
    pub code: [WORD32; 512],
    pub code_extra: [WORD32; 512],
    pub p_num_bits: [WORD8; 512],
    pub read_direction: UWORD8,
    pub is_decoded: [WORD32; 512],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_huff_code_reord_sect_info_struct {
    pub num_code_word: UWORD32,
    pub current_codeword: UWORD32,
    pub num_sorted_section: UWORD32,
    pub ptr_num_cw_in_sect: [UWORD16; 256],
    pub ptr_num_sorted_cw_in_sect: [UWORD16; 256],
    pub ptr_num_ext_sorted_cw_in_sect: [UWORD16; 270],
    pub num_ext_sorted_cw_in_sect_idx: WORD32,
    pub ptr_num_ext_sorted_sect_in_sets: [UWORD16; 14],
    pub num_ext_sorted_sect_in_sets_idx: WORD32,
    pub ptr_reorder_offset: [UWORD16; 256],
    pub ptr_sorted_cb: [UWORD8; 256],
    pub ptr_ext_sorted_cw: [UWORD8; 270],
    pub ext_sorted_cw_idx: WORD32,
    pub ptr_ext_sorted_sect_max_cb_len: [UWORD8; 270],
    pub ext_sorted_sect_max_cb_len_idx: WORD32,
    pub ptr_cb_switch: [UWORD8; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_hcr_non_pcw_sideinfo_struct {
    pub ptr_result_base: *mut WORD32,
    pub res_ptr_idx: [UWORD16; 256],
    pub cw_offset: UWORD32,
    pub ptr_cb: [UWORD8; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_hcr_info_struct {
    pub str_dec_io: ia_huff_code_reorder_io_struct,
    pub codebook_pairs: ia_huff_code_reorder_cb_pairs_struct,
    pub table_info: ia_huff_code_reorder_tbl_struct,
    pub str_segment_info: ia_huff_code_reord_seg_info_struct,
    pub sect_info: ia_huff_code_reord_sect_info_struct,
    pub str_non_pcw_side_info: ia_hcr_non_pcw_sideinfo_struct,
    pub global_hcr_type: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_channel_info_struct {
    pub ptr_scale_factor: *mut WORD16,
    pub ptr_code_book: *mut WORD8,
    pub ptr_spec_coeff: *mut WORD32,
    pub pstr_stereo_info: *mut ia_stereo_info_struct,
    pub pstr_pns_corr_info: *mut ia_pns_correlation_info_struct,
    pub pstr_pns_rand_vec_data: *mut ia_pns_rand_vec_struct,
    pub str_ics_info: ia_ics_info_struct,
    pub str_tns_info: ia_tns_info_aac_struct,
    pub str_pulse_info: ia_pulse_info_struct,
    pub str_pns_info: ia_pns_info_struct,
    pub common_window: WORD16,
    pub element_instance_tag: WORD16,
    pub global_gain: WORD16,
    pub scratch_buf_ptr: *mut WORD32,
    pub pulse_scratch: *mut WORD32,
    pub ptr_rvlc_info: ia_rvlc_info_struct,
    pub str_hcr_info: ia_hcr_info_struct,
    pub reorder_spect_data_len: WORD16,
    pub longest_cw_len: WORD8,
    pub rvlc_scf_esc_arr: [WORD16; 128],
    pub rvlc_scf_fwd_arr: [WORD16; 128],
    pub rvlc_scf_bwd_arr: [WORD16; 128],
    pub rvlc_intensity_used: WORD8,
    pub num_line_in_sec4_hcr_arr: [WORD16; 256],
    pub cb4_hcr_arr: [UWORD8; 256],
    pub number_sect: WORD32,
    pub granule_len: WORD32,
    pub rvlc_curr_sf_flag: WORD16,
    pub ltp_buf: *mut WORD16,
    pub ltp_lag: UWORD16,
}
pub type C2RustUnnamed = core::ffi::c_uint;
pub const END_HDR: C2RustUnnamed = 12;
pub const CRC_LEVEL_FIN: C2RustUnnamed = 11;
pub const ID_IIND_ICS: C2RustUnnamed = 10;
pub const ID_NULL: C2RustUnnamed = 9;
pub const ID_HDR: C2RustUnnamed = 8;
pub const ID_END: C2RustUnnamed = 7;
pub const ID_FIL: C2RustUnnamed = 6;
pub const ID_PCE: C2RustUnnamed = 5;
pub const ID_DSE: C2RustUnnamed = 4;
pub const ID_LFE: C2RustUnnamed = 3;
pub const ID_CCE: C2RustUnnamed = 2;
pub const ID_CPE: C2RustUnnamed = 1;
pub const ID_SCE: C2RustUnnamed = 0;
#[inline]
unsafe extern "C" fn ixheaac_min32(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut min_val: WORD32 = 0;
    min_val = if a < b { a } else { b };
    return min_val;
}
#[inline]
unsafe extern "C" fn ixheaac_max32(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut max_val: WORD32 = 0;
    max_val = if a > b { a } else { b };
    return max_val;
}
pub const IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES: core::ffi::c_int = 0x1804
    as core::ffi::c_int;
pub const ZERO_HCB: core::ffi::c_int = 0;
pub const NOISE_HCB: core::ffi::c_int = 13;
pub const INTENSITY_HCB2: core::ffi::c_int = 14 as core::ffi::c_int;
pub const INTENSITY_HCB: core::ffi::c_int = 15 as core::ffi::c_int;
pub const EIGHT_SHORT_SEQUENCE: core::ffi::c_int = 2 as core::ffi::c_int;
pub const IA_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const RVLC_ERROR_ALL_ESCAPE_WORDS_INVALID: core::ffi::c_uint = 0x80000000
    as core::ffi::c_uint;
pub const RVLC_ERROR_FORBIDDEN_CW_DETECTED_FWD: core::ffi::c_int = 0x8000000
    as core::ffi::c_int;
pub const RVLC_ERROR_FORBIDDEN_CW_DETECTED_BWD: core::ffi::c_int = 0x4000000
    as core::ffi::c_int;
pub const FWD: core::ffi::c_int = 0 as core::ffi::c_int;
pub const MAX_LEN_RVLC_CODE_WORD: core::ffi::c_int = 9 as core::ffi::c_int;
pub const MAX_LEN_RVLC_ESCAPE_WORD: core::ffi::c_int = 20 as core::ffi::c_int;
pub const CONCEAL_MAX_INIT: core::ffi::c_int = 1311 as core::ffi::c_int;
pub const CONCEAL_MIN_INIT: core::ffi::c_int = -(1311 as core::ffi::c_int);
pub const RVLC_MAX_SFB: core::ffi::c_int = 8 as core::ffi::c_int
    * 16 as core::ffi::c_int;
pub const MASK_LEFT: core::ffi::c_int = 0xfff000 as core::ffi::c_int;
pub const MASK_RIGHT: core::ffi::c_int = 0xfff as core::ffi::c_int;
pub const CLR_BIT_10: core::ffi::c_int = 0x3ff as core::ffi::c_int;
pub const LEFT_OFFSET: core::ffi::c_int = 12 as core::ffi::c_int;
unsafe extern "C" fn ixheaacd_rvlc_decode(
    mut cw: WORD16,
    mut len: WORD32,
    mut found: *mut WORD32,
) -> WORD32 {
    let mut indx: WORD16 = 0 as WORD16;
    *found = 0 as core::ffi::c_int as WORD32;
    match len {
        1 => {
            if cw as core::ffi::c_int == 0 as core::ffi::c_int {
                indx = 0 as WORD16;
            } else {
                return 3 as WORD32
            }
        }
        3 => {
            match cw as core::ffi::c_int {
                5 => {
                    indx = -(1 as core::ffi::c_int) as WORD16;
                }
                7 => {
                    indx = 1 as WORD16;
                }
                _ => return 4 as WORD32,
            }
        }
        4 => {
            if cw as core::ffi::c_int == 9 as core::ffi::c_int {
                indx = -(2 as core::ffi::c_int) as WORD16;
            } else {
                return 5 as WORD32
            }
        }
        5 => {
            match cw as core::ffi::c_int {
                17 => {
                    indx = -(3 as core::ffi::c_int) as WORD16;
                }
                27 => {
                    indx = 2 as WORD16;
                }
                _ => return 6 as WORD32,
            }
        }
        6 => {
            match cw as core::ffi::c_int {
                33 => {
                    indx = -(4 as core::ffi::c_int) as WORD16;
                }
                51 => {
                    indx = 3 as WORD16;
                }
                _ => return 7 as WORD32,
            }
        }
        7 => {
            match cw as core::ffi::c_int {
                65 => {
                    indx = -(7 as core::ffi::c_int) as WORD16;
                }
                107 => {
                    indx = 4 as WORD16;
                }
                99 => {
                    indx = 7 as WORD16;
                }
                _ => return 8 as WORD32,
            }
        }
        8 => {
            match cw as core::ffi::c_int {
                129 => {
                    indx = -(5 as core::ffi::c_int) as WORD16;
                }
                195 => {
                    indx = 5 as WORD16;
                }
                _ => return 9 as WORD32,
            }
        }
        9 => {
            match cw as core::ffi::c_int {
                257 => {
                    indx = -(6 as core::ffi::c_int) as WORD16;
                }
                427 => {
                    indx = 6 as WORD16;
                }
                _ => return -(1 as WORD32),
            }
        }
        _ => return -(1 as WORD32),
    }
    *found = 1 as core::ffi::c_int as WORD32;
    return indx as WORD32;
}
unsafe extern "C" fn ixheaacd_rvlc_decode_esc(
    mut cw: WORD32,
    mut len: WORD32,
    mut found: *mut WORD32,
) -> WORD32 {
    let mut indx: WORD16 = 0 as WORD16;
    *found = 0 as core::ffi::c_int as WORD32;
    match len {
        2 => {
            match cw {
                2 => {
                    indx = 0 as WORD16;
                }
                0 => {
                    indx = 1 as WORD16;
                }
                _ => return 3 as WORD32,
            }
        }
        3 => {
            match cw {
                6 => {
                    indx = 2 as WORD16;
                }
                2 => {
                    indx = 3 as WORD16;
                }
                _ => return 4 as WORD32,
            }
        }
        4 => {
            if cw == 14 as core::ffi::c_int {
                indx = 4 as WORD16;
            } else {
                return 5 as WORD32
            }
        }
        5 => {
            match cw {
                31 => {
                    indx = 5 as WORD16;
                }
                15 => {
                    indx = 6 as WORD16;
                }
                13 => {
                    indx = 7 as WORD16;
                }
                _ => return 6 as WORD32,
            }
        }
        6 => {
            match cw {
                61 => {
                    indx = 8 as WORD16;
                }
                29 => {
                    indx = 9 as WORD16;
                }
                25 => {
                    indx = 10 as WORD16;
                }
                24 => {
                    indx = 11 as WORD16;
                }
                _ => return 7 as WORD32,
            }
        }
        7 => {
            match cw {
                120 => {
                    indx = 12 as WORD16;
                }
                56 => {
                    indx = 13 as WORD16;
                }
                _ => return 8 as WORD32,
            }
        }
        8 => {
            match cw {
                242 => {
                    indx = 14 as WORD16;
                }
                114 => {
                    indx = 15 as WORD16;
                }
                _ => return 9 as WORD32,
            }
        }
        9 => {
            match cw {
                486 => {
                    indx = 16 as WORD16;
                }
                230 => {
                    indx = 17 as WORD16;
                }
                _ => return 10 as WORD32,
            }
        }
        10 => {
            match cw {
                974 => {
                    indx = 18 as WORD16;
                }
                463 => {
                    indx = 19 as WORD16;
                }
                _ => return 11 as WORD32,
            }
        }
        11 => {
            match cw {
                1950 => {
                    indx = 20 as WORD16;
                }
                1951 => {
                    indx = 21 as WORD16;
                }
                925 => {
                    indx = 22 as WORD16;
                }
                _ => return 12 as WORD32,
            }
        }
        12 => {
            if cw == 1848 as core::ffi::c_int {
                indx = 23 as WORD16;
            } else {
                return 13 as WORD32
            }
        }
        13 => {
            if cw == 3698 as core::ffi::c_int {
                indx = 25 as WORD16;
            } else {
                return 14 as WORD32
            }
        }
        14 => {
            if cw == 7399 as core::ffi::c_int {
                indx = 24 as WORD16;
            } else {
                return 15 as WORD32
            }
        }
        15 => {
            if cw == 14797 as core::ffi::c_int {
                indx = 26 as WORD16;
            } else {
                return 19 as WORD32
            }
        }
        19 => {
            if cw >= 236736 as core::ffi::c_int && cw <= 236740 as core::ffi::c_int {
                indx = (53 as WORD32 - (236740 as WORD32 - cw)) as WORD16;
            } else {
                return 20 as WORD32
            }
        }
        20 => {
            if cw >= 473482 as core::ffi::c_int && cw <= 473503 as core::ffi::c_int {
                indx = (48 as WORD32 - (473503 as WORD32 - cw)) as WORD16;
            } else {
                return -(1 as WORD32)
            }
        }
        _ => return -(1 as WORD32),
    }
    *found = 1 as core::ffi::c_int as WORD32;
    return indx as WORD32;
}
unsafe extern "C" fn ixheaacd_rvlc_check_intensity_cb(
    mut ptr_rvlc: *mut ia_rvlc_info_struct,
    mut ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
) -> VOID {
    let mut group: WORD32 = 0;
    let mut band: WORD32 = 0;
    let mut bnds: WORD32 = 0;
    (*ptr_rvlc).intensity_used = 0 as WORD8;
    group = 0 as core::ffi::c_int as WORD32;
    while group < (*ptr_rvlc).num_wind_grps as core::ffi::c_int {
        band = 0 as core::ffi::c_int as WORD32;
        while band < (*ptr_rvlc).max_sfb_transmitted as core::ffi::c_int {
            bnds = 16 as WORD32 * group + band;
            if *((*ptr_aac_dec_channel_info).ptr_code_book).offset(bnds as isize)
                as core::ffi::c_int == INTENSITY_HCB
                || *((*ptr_aac_dec_channel_info).ptr_code_book).offset(bnds as isize)
                    as core::ffi::c_int == INTENSITY_HCB2
            {
                (*ptr_rvlc).intensity_used = 1 as WORD8;
                break;
            } else {
                band += 1;
            }
        }
        group += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_carry_bit_branch_val(
    mut carry_bit: UWORD8,
    mut tree_node: UWORD32,
    mut branch_val: *mut UWORD32,
    mut branch_node: *mut UWORD32,
) -> VOID {
    if carry_bit as core::ffi::c_int == 0 as core::ffi::c_int {
        *branch_node = (tree_node & MASK_LEFT as UWORD32) >> LEFT_OFFSET;
    } else {
        *branch_node = tree_node & MASK_RIGHT as UWORD32;
    }
    *branch_val = *branch_node & CLR_BIT_10 as UWORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_rvlc_read_bits(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut ptr_position: *mut UWORD16,
    mut read_direction: UWORD8,
) -> UWORD8 {
    let mut bit: UWORD32 = 0;
    let mut read_bit_offset: WORD32 = *ptr_position as WORD32
        - ((*it_bit_buff).size - (*it_bit_buff).cnt_bits);
    if read_bit_offset != 0 {
        (*it_bit_buff).cnt_bits -= read_bit_offset;
    }
    (*it_bit_buff).ptr_read_next = ((*it_bit_buff).ptr_bit_buf_base)
        .offset(
            ((*it_bit_buff).size - (*it_bit_buff).cnt_bits >> 3 as core::ffi::c_int)
                as isize,
        );
    (*it_bit_buff).bit_pos = ((*it_bit_buff).size as core::ffi::c_int
        - (*it_bit_buff).cnt_bits as core::ffi::c_int & 7 as core::ffi::c_int) as WORD32;
    if read_direction as core::ffi::c_int == 0 as core::ffi::c_int {
        bit = ixheaacd_aac_read_bit_rev(it_bit_buff) as UWORD32;
        *ptr_position = (*ptr_position as core::ffi::c_int + 1 as core::ffi::c_int)
            as UWORD16;
    } else {
        bit = ixheaacd_aac_read_bit(it_bit_buff) as UWORD32;
        *ptr_position = (*ptr_position as core::ffi::c_int - 1 as core::ffi::c_int)
            as UWORD16;
    }
    return bit as UWORD8;
}
unsafe extern "C" fn ixheaacd_rvlc_decode_escape_word(
    mut ptr_rvlc: *mut ia_rvlc_info_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
) -> WORD8 {
    let mut i: WORD32 = 0;
    let mut carry_bit: UWORD8 = 0;
    let mut ptr_bitstream_index_esc: *mut UWORD16 = 0 as *mut UWORD16;
    let mut len: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut codeword: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut found: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut indx: core::ffi::c_int = 0;
    ptr_bitstream_index_esc = &mut (*ptr_rvlc).esc_bit_str_idx;
    i = (MAX_LEN_RVLC_ESCAPE_WORD - 1 as core::ffi::c_int) as WORD32;
    while i >= 0 as core::ffi::c_int {
        carry_bit = ixheaacd_rvlc_read_bits(
            it_bit_buff,
            ptr_bitstream_index_esc,
            FWD as UWORD8,
        );
        len += 1;
        codeword = codeword << 1 as core::ffi::c_int | carry_bit as core::ffi::c_int;
        indx = ixheaacd_rvlc_decode_esc(codeword as WORD32, len as WORD32, &mut found)
            as core::ffi::c_int;
        if found != 0 {
            (*ptr_rvlc).rvlc_esc_len = ((*ptr_rvlc).rvlc_esc_len as core::ffi::c_int
                - (MAX_LEN_RVLC_ESCAPE_WORD - i) as core::ffi::c_int) as WORD16;
            return indx as WORD8;
        }
        i -= 1;
    }
    (*ptr_rvlc).rvlc_err_log |= RVLC_ERROR_ALL_ESCAPE_WORDS_INVALID;
    return -(1 as core::ffi::c_int) as WORD8;
}
unsafe extern "C" fn ixheaacd_rvlc_decode_escape(
    mut ptr_rvlc: *mut ia_rvlc_info_struct,
    mut ptr_escape: *mut WORD16,
    mut it_bit_buff: *mut ia_bit_buf_struct,
) -> VOID {
    let mut esc_word: WORD8 = 0;
    let mut esc_cnt: WORD8 = 0 as WORD8;
    let mut ptr_esc_bit_cnt_sum: *mut WORD16 = 0 as *mut WORD16;
    ptr_esc_bit_cnt_sum = &mut (*ptr_rvlc).rvlc_esc_len;
    while *ptr_esc_bit_cnt_sum as core::ffi::c_int > 0 as core::ffi::c_int {
        esc_word = ixheaacd_rvlc_decode_escape_word(ptr_rvlc, it_bit_buff);
        if esc_word as core::ffi::c_int >= 0 as core::ffi::c_int {
            *ptr_escape.offset(esc_cnt as isize) = esc_word as WORD16;
            esc_cnt += 1;
        } else {
            (*ptr_rvlc).rvlc_err_log |= RVLC_ERROR_ALL_ESCAPE_WORDS_INVALID;
            (*ptr_rvlc).num_esc_words_decoded = esc_cnt as UWORD8;
            return;
        }
    }
    (*ptr_rvlc).num_esc_words_decoded = esc_cnt as UWORD8;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_decode_rvlc_code_word(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut ptr_rvlc: *mut ia_rvlc_info_struct,
) -> WORD8 {
    let mut i: WORD32 = 0;
    let mut carry_bit: UWORD8 = 0;
    let mut direction: UWORD8 = (*ptr_rvlc).direction;
    let mut ptr_bit_str_idx_rvl: *mut UWORD16 = (*ptr_rvlc).ptr_rvl_bit_str_idx;
    let mut len: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut codeword: core::ffi::c_short = 0 as core::ffi::c_short;
    let mut found: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut indx: core::ffi::c_int = 0;
    i = (MAX_LEN_RVLC_CODE_WORD - 1 as core::ffi::c_int) as WORD32;
    while i >= 0 as core::ffi::c_int {
        carry_bit = ixheaacd_rvlc_read_bits(it_bit_buff, ptr_bit_str_idx_rvl, direction);
        len += 1;
        codeword = ((codeword as core::ffi::c_int) << 1 as core::ffi::c_int
            | carry_bit as core::ffi::c_int) as core::ffi::c_short;
        indx = ixheaacd_rvlc_decode(codeword as WORD16, len as WORD32, &mut found)
            as core::ffi::c_int;
        if found != 0 {
            indx = indx + 7 as core::ffi::c_int;
            *(*ptr_rvlc).ptr_rvl_bit_cnt = (*(*ptr_rvlc).ptr_rvl_bit_cnt
                as core::ffi::c_int - (MAX_LEN_RVLC_CODE_WORD - i) as core::ffi::c_int)
                as WORD16;
            return indx as WORD8;
        }
        i -= 1;
    }
    return -(1 as core::ffi::c_int) as WORD8;
}
unsafe extern "C" fn ixheaacd_rvlc_decode_forward(
    mut ptr_rvlc: *mut ia_rvlc_info_struct,
    mut ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
) -> VOID {
    let mut band: WORD32 = 0 as WORD32;
    let mut group: WORD32 = 0 as WORD32;
    let mut bnds: WORD32 = 0 as WORD32;
    let mut dpcm: WORD16 = 0;
    let mut temp_buf: ia_bit_buf_struct = {
        let mut init = ia_bit_buf_struct {
            ptr_bit_buf_base: 0 as *mut UWORD8,
            ptr_bit_buf_end: 0 as *mut UWORD8,
            ptr_read_next: 0 as *mut UWORD8,
            bit_pos: 0,
            cnt_bits: 0,
            size: 0,
            adts_header_present: 0,
            crc_check: 0,
            protection_absent: 0,
            no_raw_data_blocks: 0,
            str_adts_crc_info: ia_adts_crc_info_struct {
                crc_active: 0,
                no_reg: 0,
                file_value: 0,
                crc_lookup: [0; 256],
                str_crc_reg_data: [ia_crc_reg_data_struct {
                    active: 0,
                    buf_size: 0,
                    max_bits: 0,
                    bit_cnt: 0,
                    bit_buf_cnt: 0,
                    str_bit_buf: ia_crc_bit_buf_struct {
                        ptr_bit_buf_base: 0 as *mut UWORD8,
                        ptr_bit_buf_end: 0 as *mut UWORD8,
                        ptr_read_next: 0 as *mut UWORD8,
                        bit_pos: 0,
                        cnt_bits: 0,
                        size: 0,
                    },
                }; 7],
            },
            pstr_adts_crc_info: 0 as *mut ia_adts_crc_info_struct,
            initial_cnt_bits: 0,
            audio_mux_align: 0,
            bit_count: 0,
            valid_bits: 0,
            byte: 0,
            byte_ptr: 0 as *mut UWORD8,
            ptr_start: 0 as *mut UWORD8,
            write_bit_count: 0,
            max_size: 0,
            xaac_jmp_buf: 0 as *mut jmp_buf,
        };
        init
    };
    let mut factor: WORD16 = (*ptr_aac_dec_channel_info).global_gain;
    let mut position: WORD16 = 0 as WORD16;
    let mut noise_energy: WORD16 = ((*ptr_aac_dec_channel_info).global_gain
        as core::ffi::c_int - 90 as core::ffi::c_int - 256 as core::ffi::c_int)
        as WORD16;
    let mut ptr_scf_fwd: *mut WORD16 = ((*ptr_aac_dec_channel_info).rvlc_scf_fwd_arr)
        .as_mut_ptr();
    let mut ptr_scf_esc: *mut WORD16 = ((*ptr_aac_dec_channel_info).rvlc_scf_esc_arr)
        .as_mut_ptr();
    let mut ptr_esc_fwd_cnt: *mut UWORD8 = &mut (*ptr_rvlc).num_fwd_esc_words_decoded;
    (*ptr_rvlc).ptr_rvl_bit_cnt = &mut (*ptr_rvlc).rvlc_sf_fwd_len;
    (*ptr_rvlc).ptr_rvl_bit_str_idx = &mut (*ptr_rvlc).rvl_fwd_bit_str_idx;
    *ptr_esc_fwd_cnt = 0 as UWORD8;
    (*ptr_rvlc).direction = 0 as UWORD8;
    (*ptr_rvlc).noise_used = 0 as WORD8;
    (*ptr_rvlc).sf_used = 0 as WORD8;
    (*ptr_rvlc).last_scale_fac = 0 as WORD16;
    (*ptr_rvlc).last_nrg = 0 as WORD16;
    (*ptr_rvlc).is_last = 0 as WORD16;
    ixheaacd_rvlc_check_intensity_cb(ptr_rvlc, ptr_aac_dec_channel_info);
    group = 0 as core::ffi::c_int as WORD32;
    while group < (*ptr_rvlc).num_wind_grps as core::ffi::c_int {
        band = 0 as core::ffi::c_int as WORD32;
        while band < (*ptr_rvlc).max_sfb_transmitted as core::ffi::c_int {
            bnds = 16 as WORD32 * group + band;
            match *((*ptr_aac_dec_channel_info).ptr_code_book).offset(bnds as isize)
                as core::ffi::c_int
            {
                ZERO_HCB => {
                    *ptr_scf_fwd.offset(bnds as isize) = 0 as WORD16;
                }
                INTENSITY_HCB2 | INTENSITY_HCB => {
                    dpcm = ixheaacd_decode_rvlc_code_word(it_bit_buff, ptr_rvlc)
                        as WORD16;
                    if (dpcm as core::ffi::c_int) < 0 as core::ffi::c_int {
                        (*ptr_rvlc).conceal_max = bnds as WORD16;
                        return;
                    }
                    dpcm = (dpcm as core::ffi::c_int - 7 as core::ffi::c_int) as WORD16;
                    if dpcm as core::ffi::c_int == -(7 as core::ffi::c_int)
                        || dpcm as core::ffi::c_int == 7 as core::ffi::c_int
                    {
                        if (*ptr_rvlc).rvlc_esc_len != 0 {
                            (*ptr_rvlc).conceal_max = bnds as WORD16;
                            return;
                        } else {
                            if dpcm as core::ffi::c_int == -(7 as core::ffi::c_int) {
                                let fresh0 = ptr_scf_esc;
                                ptr_scf_esc = ptr_scf_esc.offset(1);
                                dpcm = (dpcm as core::ffi::c_int
                                    - *fresh0 as core::ffi::c_int) as WORD16;
                            } else {
                                let fresh1 = ptr_scf_esc;
                                ptr_scf_esc = ptr_scf_esc.offset(1);
                                dpcm = (dpcm as core::ffi::c_int
                                    + *fresh1 as core::ffi::c_int) as WORD16;
                            }
                            *ptr_esc_fwd_cnt = (*ptr_esc_fwd_cnt).wrapping_add(1);
                            if (*ptr_rvlc).conceal_max_esc as core::ffi::c_int
                                == 1311 as core::ffi::c_int
                            {
                                (*ptr_rvlc).conceal_max_esc = bnds as WORD16;
                            }
                        }
                    }
                    position = (position as core::ffi::c_int + dpcm as core::ffi::c_int)
                        as WORD16;
                    *ptr_scf_fwd.offset(bnds as isize) = position;
                    (*ptr_rvlc).is_last = position;
                }
                NOISE_HCB => {
                    if (*ptr_rvlc).noise_used as core::ffi::c_int
                        == 0 as core::ffi::c_int
                    {
                        (*ptr_rvlc).noise_used = 1 as WORD8;
                        (*ptr_rvlc).first_noise_band = bnds as UWORD8;
                        noise_energy = (noise_energy as core::ffi::c_int
                            + (*ptr_rvlc).dpcm_noise_nrg as core::ffi::c_int) as WORD16;
                        *ptr_scf_fwd.offset(bnds as isize) = noise_energy;
                        (*ptr_rvlc).last_nrg = noise_energy;
                    } else {
                        dpcm = ixheaacd_decode_rvlc_code_word(it_bit_buff, ptr_rvlc)
                            as WORD16;
                        if (dpcm as core::ffi::c_int) < 0 as core::ffi::c_int {
                            (*ptr_rvlc).conceal_max = bnds as WORD16;
                            return;
                        }
                        dpcm = (dpcm as core::ffi::c_int - 7 as core::ffi::c_int)
                            as WORD16;
                        if dpcm as core::ffi::c_int == -(7 as core::ffi::c_int)
                            || dpcm as core::ffi::c_int == 7 as core::ffi::c_int
                        {
                            if (*ptr_rvlc).rvlc_esc_len != 0 {
                                (*ptr_rvlc).conceal_max = bnds as WORD16;
                                return;
                            } else {
                                if dpcm as core::ffi::c_int == -(7 as core::ffi::c_int) {
                                    let fresh2 = ptr_scf_esc;
                                    ptr_scf_esc = ptr_scf_esc.offset(1);
                                    dpcm = (dpcm as core::ffi::c_int
                                        - *fresh2 as core::ffi::c_int) as WORD16;
                                } else {
                                    let fresh3 = ptr_scf_esc;
                                    ptr_scf_esc = ptr_scf_esc.offset(1);
                                    dpcm = (dpcm as core::ffi::c_int
                                        + *fresh3 as core::ffi::c_int) as WORD16;
                                }
                                *ptr_esc_fwd_cnt = (*ptr_esc_fwd_cnt).wrapping_add(1);
                                if (*ptr_rvlc).conceal_max_esc as core::ffi::c_int
                                    == 1311 as core::ffi::c_int
                                {
                                    (*ptr_rvlc).conceal_max_esc = bnds as WORD16;
                                }
                            }
                        }
                        noise_energy = (noise_energy as core::ffi::c_int
                            + dpcm as core::ffi::c_int) as WORD16;
                        *ptr_scf_fwd.offset(bnds as isize) = noise_energy;
                        (*ptr_rvlc).last_nrg = noise_energy;
                    }
                    (*ptr_aac_dec_channel_info).str_pns_info.pns_used[bnds as usize] = 1
                        as UWORD8;
                }
                _ => {
                    (*ptr_rvlc).sf_used = 1 as WORD8;
                    memcpy(
                        &mut temp_buf as *mut ia_bit_buf_struct
                            as *mut core::ffi::c_void,
                        it_bit_buff as *const core::ffi::c_void,
                        ::core::mem::size_of::<ia_bit_buf_struct>() as size_t,
                    );
                    dpcm = ixheaacd_decode_rvlc_code_word(it_bit_buff, ptr_rvlc)
                        as WORD16;
                    if (dpcm as core::ffi::c_int) < 0 as core::ffi::c_int {
                        (*ptr_rvlc).conceal_max = bnds as WORD16;
                        return;
                    }
                    dpcm = (dpcm as core::ffi::c_int - 7 as core::ffi::c_int) as WORD16;
                    if dpcm as core::ffi::c_int == -(7 as core::ffi::c_int)
                        || dpcm as core::ffi::c_int == 7 as core::ffi::c_int
                    {
                        if (*ptr_rvlc).rvlc_esc_len != 0 {
                            (*ptr_rvlc).conceal_max = bnds as WORD16;
                            return;
                        } else {
                            if dpcm as core::ffi::c_int == -(7 as core::ffi::c_int) {
                                let fresh4 = ptr_scf_esc;
                                ptr_scf_esc = ptr_scf_esc.offset(1);
                                dpcm = (dpcm as core::ffi::c_int
                                    - *fresh4 as core::ffi::c_int) as WORD16;
                            } else {
                                let fresh5 = ptr_scf_esc;
                                ptr_scf_esc = ptr_scf_esc.offset(1);
                                dpcm = (dpcm as core::ffi::c_int
                                    + *fresh5 as core::ffi::c_int) as WORD16;
                            }
                            *ptr_esc_fwd_cnt = (*ptr_esc_fwd_cnt).wrapping_add(1);
                            if (*ptr_rvlc).conceal_max_esc as core::ffi::c_int
                                == 1311 as core::ffi::c_int
                            {
                                (*ptr_rvlc).conceal_max_esc = bnds as WORD16;
                            }
                        }
                    }
                    factor = (factor as core::ffi::c_int + dpcm as core::ffi::c_int)
                        as WORD16;
                    *ptr_scf_fwd.offset(bnds as isize) = factor;
                    (*ptr_rvlc).last_scale_fac = factor;
                }
            }
            band += 1;
        }
        group += 1;
    }
    if (*ptr_rvlc).intensity_used != 0 {
        dpcm = ixheaacd_decode_rvlc_code_word(it_bit_buff, ptr_rvlc) as WORD16;
        if (dpcm as core::ffi::c_int) < 0 as core::ffi::c_int {
            (*ptr_rvlc).conceal_max = bnds as WORD16;
            return;
        }
        dpcm = (dpcm as core::ffi::c_int - 7 as core::ffi::c_int) as WORD16;
        if dpcm as core::ffi::c_int == -(7 as core::ffi::c_int)
            || dpcm as core::ffi::c_int == 7 as core::ffi::c_int
        {
            if (*ptr_rvlc).rvlc_esc_len != 0 {
                (*ptr_rvlc).conceal_max = bnds as WORD16;
                return;
            } else {
                if dpcm as core::ffi::c_int == -(7 as core::ffi::c_int) {
                    let fresh6 = ptr_scf_esc;
                    ptr_scf_esc = ptr_scf_esc.offset(1);
                    dpcm = (dpcm as core::ffi::c_int - *fresh6 as core::ffi::c_int)
                        as WORD16;
                } else {
                    let fresh7 = ptr_scf_esc;
                    ptr_scf_esc = ptr_scf_esc.offset(1);
                    dpcm = (dpcm as core::ffi::c_int + *fresh7 as core::ffi::c_int)
                        as WORD16;
                }
                *ptr_esc_fwd_cnt = (*ptr_esc_fwd_cnt).wrapping_add(1);
                if (*ptr_rvlc).conceal_max_esc as core::ffi::c_int
                    == 1311 as core::ffi::c_int
                {
                    (*ptr_rvlc).conceal_max_esc = bnds as WORD16;
                }
            }
        }
        (*ptr_rvlc).dpcm_is_last_pos = dpcm as WORD32;
    }
}
unsafe extern "C" fn ixheaacd_rvlc_decode_backward(
    mut ptr_rvlc: *mut ia_rvlc_info_struct,
    mut ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
) -> VOID {
    let mut band: WORD16 = 0;
    let mut group: WORD16 = 0;
    let mut dpcm: WORD16 = 0;
    let mut bnds: WORD16 = ((*ptr_rvlc).max_sfb_transmitted as core::ffi::c_int
        - 1 as core::ffi::c_int) as WORD16;
    let mut factor: WORD16 = (*ptr_rvlc).rev_global_gain as WORD16;
    let mut position: WORD16 = (*ptr_rvlc).dpcm_is_last_pos as WORD16;
    let mut noise_energy: WORD16 = ((*ptr_rvlc).rev_global_gain as core::ffi::c_int
        + (*ptr_rvlc).dpcm_noise_last_pos as core::ffi::c_int - 90 as core::ffi::c_int
        - 256 as core::ffi::c_int) as WORD16;
    let mut ptr_scf_bwd: *mut WORD16 = ((*ptr_aac_dec_channel_info).rvlc_scf_bwd_arr)
        .as_mut_ptr();
    let mut ptr_scf_esc: *mut WORD16 = ((*ptr_aac_dec_channel_info).rvlc_scf_esc_arr)
        .as_mut_ptr();
    let mut ptr_esc_cnt: *mut UWORD8 = &mut (*ptr_rvlc).num_esc_words_decoded;
    let mut ptr_esc_bwd_cnt: *mut UWORD8 = &mut (*ptr_rvlc).num_bwd_esc_words_decoded;
    (*ptr_rvlc).ptr_rvl_bit_cnt = &mut (*ptr_rvlc).rvlc_sf_bwd_len;
    (*ptr_rvlc).ptr_rvl_bit_str_idx = &mut (*ptr_rvlc).rvl_bwd_bit_str_idx;
    *ptr_esc_bwd_cnt = 0 as UWORD8;
    (*ptr_rvlc).direction = 1 as UWORD8;
    ptr_scf_esc = ptr_scf_esc
        .offset((*ptr_esc_cnt as core::ffi::c_int - 1 as core::ffi::c_int) as isize);
    (*ptr_rvlc).firt_scale_fac = 0 as WORD16;
    (*ptr_rvlc).first_nrg = 0 as WORD16;
    (*ptr_rvlc).is_first = 0 as WORD16;
    if (*ptr_rvlc).intensity_used != 0 {
        dpcm = ixheaacd_decode_rvlc_code_word(it_bit_buff, ptr_rvlc) as WORD16;
        if (dpcm as core::ffi::c_int) < 0 as core::ffi::c_int {
            (*ptr_rvlc).dpcm_is_last_pos = 0 as core::ffi::c_int as WORD32;
            (*ptr_rvlc).conceal_min = bnds;
            return;
        }
        dpcm = (dpcm as core::ffi::c_int - 7 as core::ffi::c_int) as WORD16;
        if dpcm as core::ffi::c_int == -(7 as core::ffi::c_int)
            || dpcm as core::ffi::c_int == 7 as core::ffi::c_int
        {
            if (*ptr_rvlc).rvlc_esc_len != 0 {
                (*ptr_rvlc).conceal_min = bnds;
                return;
            } else {
                if dpcm as core::ffi::c_int == -(7 as core::ffi::c_int) {
                    let fresh8 = ptr_scf_esc;
                    ptr_scf_esc = ptr_scf_esc.offset(-1);
                    dpcm = (dpcm as core::ffi::c_int - *fresh8 as core::ffi::c_int)
                        as WORD16;
                } else {
                    let fresh9 = ptr_scf_esc;
                    ptr_scf_esc = ptr_scf_esc.offset(-1);
                    dpcm = (dpcm as core::ffi::c_int + *fresh9 as core::ffi::c_int)
                        as WORD16;
                }
                *ptr_esc_bwd_cnt = (*ptr_esc_bwd_cnt).wrapping_add(1);
                if (*ptr_rvlc).conceal_min_esc as core::ffi::c_int
                    == -(1311 as core::ffi::c_int)
                {
                    (*ptr_rvlc).conceal_min_esc = bnds;
                }
            }
        }
        (*ptr_rvlc).dpcm_is_last_pos = dpcm as WORD32;
    }
    group = ((*ptr_rvlc).num_wind_grps as core::ffi::c_int - 1 as core::ffi::c_int)
        as WORD16;
    while group as core::ffi::c_int >= 0 as core::ffi::c_int {
        band = ((*ptr_rvlc).max_sfb_transmitted as core::ffi::c_int
            - 1 as core::ffi::c_int) as WORD16;
        while band as core::ffi::c_int >= 0 as core::ffi::c_int {
            bnds = (16 as core::ffi::c_int * group as core::ffi::c_int
                + band as core::ffi::c_int) as WORD16;
            match *((*ptr_aac_dec_channel_info).ptr_code_book).offset(bnds as isize)
                as core::ffi::c_int
            {
                ZERO_HCB => {
                    *ptr_scf_bwd.offset(bnds as isize) = 0 as WORD16;
                }
                INTENSITY_HCB2 | INTENSITY_HCB => {
                    dpcm = ixheaacd_decode_rvlc_code_word(it_bit_buff, ptr_rvlc)
                        as WORD16;
                    if (dpcm as core::ffi::c_int) < 0 as core::ffi::c_int {
                        *ptr_scf_bwd.offset(bnds as isize) = position;
                        return;
                    }
                    dpcm = (dpcm as core::ffi::c_int - 7 as core::ffi::c_int) as WORD16;
                    if dpcm as core::ffi::c_int == -(7 as core::ffi::c_int)
                        || dpcm as core::ffi::c_int == 7 as core::ffi::c_int
                    {
                        if (*ptr_rvlc).rvlc_esc_len != 0 {
                            *ptr_scf_bwd.offset(bnds as isize) = position;
                            return;
                        } else {
                            if dpcm as core::ffi::c_int == -(7 as core::ffi::c_int) {
                                let fresh10 = ptr_scf_esc;
                                ptr_scf_esc = ptr_scf_esc.offset(-1);
                                dpcm = (dpcm as core::ffi::c_int
                                    - *fresh10 as core::ffi::c_int) as WORD16;
                            } else {
                                let fresh11 = ptr_scf_esc;
                                ptr_scf_esc = ptr_scf_esc.offset(-1);
                                dpcm = (dpcm as core::ffi::c_int
                                    + *fresh11 as core::ffi::c_int) as WORD16;
                            }
                            *ptr_esc_bwd_cnt = (*ptr_esc_bwd_cnt).wrapping_add(1);
                            (*ptr_rvlc).conceal_min_esc as core::ffi::c_int
                                == -(1311 as core::ffi::c_int);
                        }
                    }
                    *ptr_scf_bwd.offset(bnds as isize) = position;
                    position = (position as core::ffi::c_int - dpcm as core::ffi::c_int)
                        as WORD16;
                    (*ptr_rvlc).is_first = position;
                }
                NOISE_HCB => {
                    if bnds as core::ffi::c_int
                        == (*ptr_rvlc).first_noise_band as core::ffi::c_int
                    {
                        *ptr_scf_bwd.offset(bnds as isize) = ((*ptr_rvlc).dpcm_noise_nrg
                            as core::ffi::c_int
                            + (*ptr_aac_dec_channel_info).global_gain as core::ffi::c_int
                            - 90 as core::ffi::c_int - 256 as core::ffi::c_int)
                            as WORD16;
                        (*ptr_rvlc).first_nrg = *ptr_scf_bwd.offset(bnds as isize);
                    } else {
                        dpcm = ixheaacd_decode_rvlc_code_word(it_bit_buff, ptr_rvlc)
                            as WORD16;
                        if (dpcm as core::ffi::c_int) < 0 as core::ffi::c_int {
                            *ptr_scf_bwd.offset(bnds as isize) = noise_energy;
                            return;
                        }
                        dpcm = (dpcm as core::ffi::c_int - 7 as core::ffi::c_int)
                            as WORD16;
                        if dpcm as core::ffi::c_int == -(7 as core::ffi::c_int)
                            || dpcm as core::ffi::c_int == 7 as core::ffi::c_int
                        {
                            if (*ptr_rvlc).rvlc_esc_len != 0 {
                                *ptr_scf_bwd.offset(bnds as isize) = noise_energy;
                                return;
                            } else {
                                if dpcm as core::ffi::c_int == -(7 as core::ffi::c_int) {
                                    let fresh12 = ptr_scf_esc;
                                    ptr_scf_esc = ptr_scf_esc.offset(-1);
                                    dpcm = (dpcm as core::ffi::c_int
                                        - *fresh12 as core::ffi::c_int) as WORD16;
                                } else {
                                    let fresh13 = ptr_scf_esc;
                                    ptr_scf_esc = ptr_scf_esc.offset(-1);
                                    dpcm = (dpcm as core::ffi::c_int
                                        + *fresh13 as core::ffi::c_int) as WORD16;
                                }
                                *ptr_esc_bwd_cnt = (*ptr_esc_bwd_cnt).wrapping_add(1);
                                (*ptr_rvlc).conceal_min_esc as core::ffi::c_int
                                    == -(1311 as core::ffi::c_int);
                            }
                        }
                        *ptr_scf_bwd.offset(bnds as isize) = noise_energy;
                        noise_energy = (noise_energy as core::ffi::c_int
                            - dpcm as core::ffi::c_int) as WORD16;
                        (*ptr_rvlc).first_nrg = noise_energy;
                    }
                }
                _ => {
                    dpcm = ixheaacd_decode_rvlc_code_word(it_bit_buff, ptr_rvlc)
                        as WORD16;
                    if (dpcm as core::ffi::c_int) < 0 as core::ffi::c_int {
                        *ptr_scf_bwd.offset(bnds as isize) = factor;
                        return;
                    }
                    dpcm = (dpcm as core::ffi::c_int - 7 as core::ffi::c_int) as WORD16;
                    if dpcm as core::ffi::c_int == -(7 as core::ffi::c_int)
                        || dpcm as core::ffi::c_int == 7 as core::ffi::c_int
                    {
                        if (*ptr_rvlc).rvlc_esc_len != 0 {
                            *ptr_scf_bwd.offset(bnds as isize) = factor;
                            return;
                        } else {
                            if dpcm as core::ffi::c_int == -(7 as core::ffi::c_int) {
                                let fresh14 = ptr_scf_esc;
                                ptr_scf_esc = ptr_scf_esc.offset(-1);
                                dpcm = (dpcm as core::ffi::c_int
                                    - *fresh14 as core::ffi::c_int) as WORD16;
                            } else {
                                let fresh15 = ptr_scf_esc;
                                ptr_scf_esc = ptr_scf_esc.offset(-1);
                                dpcm = (dpcm as core::ffi::c_int
                                    + *fresh15 as core::ffi::c_int) as WORD16;
                            }
                            *ptr_esc_bwd_cnt = (*ptr_esc_bwd_cnt).wrapping_add(1);
                            (*ptr_rvlc).conceal_min_esc as core::ffi::c_int
                                == -(1311 as core::ffi::c_int);
                        }
                    }
                    *ptr_scf_bwd.offset(bnds as isize) = factor;
                    factor = (factor as core::ffi::c_int - dpcm as core::ffi::c_int)
                        as WORD16;
                    (*ptr_rvlc).firt_scale_fac = factor;
                }
            }
            band -= 1;
        }
        group -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_rvlc_read(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
) -> VOID {
    let mut ptr_rvlc: *mut ia_rvlc_info_struct = &mut (*ptr_aac_dec_channel_info)
        .ptr_rvlc_info;
    let mut group: WORD32 = 0;
    let mut band: WORD32 = 0;
    (*ptr_rvlc).num_wind_grps = (*ptr_aac_dec_channel_info)
        .str_ics_info
        .num_window_groups;
    (*ptr_rvlc).max_sfb_transmitted = (*ptr_aac_dec_channel_info).str_ics_info.max_sfb;
    (*ptr_rvlc).noise_used = 0 as WORD8;
    (*ptr_rvlc).dpcm_noise_nrg = 0 as core::ffi::c_int as WORD32;
    (*ptr_rvlc).dpcm_noise_last_pos = 0 as core::ffi::c_int as WORD32;
    (*ptr_rvlc).rvlc_esc_len = -(1 as core::ffi::c_int) as WORD16;
    (*ptr_rvlc).dpcm_is_last_pos = 0 as core::ffi::c_int as WORD32;
    (*ptr_rvlc).sf_concealment = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
    (*ptr_rvlc).rev_global_gain = ixheaacd_read_bits_buf(it_bit_buff, 8 as WORD);
    if (*ptr_aac_dec_channel_info).str_ics_info.window_sequence as core::ffi::c_int
        == EIGHT_SHORT_SEQUENCE
    {
        (*ptr_rvlc).rvlc_sf_len = ixheaacd_read_bits_buf(it_bit_buff, 11 as WORD)
            as WORD16;
    } else {
        (*ptr_rvlc).rvlc_sf_len = ixheaacd_read_bits_buf(it_bit_buff, 9 as WORD)
            as WORD16;
    }
    group = 0 as core::ffi::c_int as WORD32;
    while group < (*ptr_rvlc).num_wind_grps as core::ffi::c_int {
        band = 0 as core::ffi::c_int as WORD32;
        while band < (*ptr_rvlc).max_sfb_transmitted as core::ffi::c_int {
            if *((*ptr_aac_dec_channel_info).ptr_code_book)
                .offset((16 as WORD32 * group + band) as isize) as core::ffi::c_int
                == NOISE_HCB
            {
                (*ptr_rvlc).noise_used = 1 as WORD8;
                break;
            } else {
                band += 1;
            }
        }
        group += 1;
    }
    if (*ptr_rvlc).noise_used != 0 {
        (*ptr_rvlc).dpcm_noise_nrg = ixheaacd_read_bits_buf(it_bit_buff, 9 as WORD);
    }
    (*ptr_rvlc).sf_esc_present = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
    if (*ptr_rvlc).sf_esc_present != 0 {
        (*ptr_rvlc).rvlc_esc_len = ixheaacd_read_bits_buf(it_bit_buff, 8 as WORD)
            as WORD16;
    }
    if (*ptr_rvlc).noise_used != 0 {
        (*ptr_rvlc).dpcm_noise_last_pos = ixheaacd_read_bits_buf(it_bit_buff, 9 as WORD);
        (*ptr_rvlc).rvlc_sf_len = ((*ptr_rvlc).rvlc_sf_len as core::ffi::c_int
            - 9 as core::ffi::c_int) as WORD16;
    }
    (*ptr_rvlc).rvlc_sf_fwd_len = (*ptr_rvlc).rvlc_sf_len;
    (*ptr_rvlc).rvlc_sf_bwd_len = (*ptr_rvlc).rvlc_sf_len;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hcr_read(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
    mut ele_type: WORD32,
) -> VOID {
    let mut len_reordered_spec_data: WORD16 = 0;
    let mut len_longest_code_word: WORD8 = 0;
    (*ptr_aac_dec_channel_info).reorder_spect_data_len = 0 as WORD16;
    (*ptr_aac_dec_channel_info).longest_cw_len = 0 as WORD8;
    len_reordered_spec_data = ixheaacd_read_bits_buf(it_bit_buff, 14 as WORD) as WORD16;
    if ele_type == ID_CPE as core::ffi::c_int {
        if len_reordered_spec_data as core::ffi::c_int >= 0 as core::ffi::c_int
            && len_reordered_spec_data as core::ffi::c_int <= 12288 as core::ffi::c_int
        {
            (*ptr_aac_dec_channel_info).reorder_spect_data_len = len_reordered_spec_data;
        } else if len_reordered_spec_data as core::ffi::c_int > 12288 as core::ffi::c_int
        {
            (*ptr_aac_dec_channel_info).reorder_spect_data_len = 12288 as WORD16;
        }
    } else if ele_type == ID_SCE as core::ffi::c_int
        || ele_type == ID_LFE as core::ffi::c_int
        || ele_type == ID_CCE as core::ffi::c_int
    {
        if len_reordered_spec_data as core::ffi::c_int >= 0 as core::ffi::c_int
            && len_reordered_spec_data as core::ffi::c_int <= 6144 as core::ffi::c_int
        {
            (*ptr_aac_dec_channel_info).reorder_spect_data_len = len_reordered_spec_data;
        } else if len_reordered_spec_data as core::ffi::c_int > 6144 as core::ffi::c_int
        {
            (*ptr_aac_dec_channel_info).reorder_spect_data_len = 6144 as WORD16;
        }
    }
    len_longest_code_word = ixheaacd_read_bits_buf(it_bit_buff, 6 as WORD) as WORD8;
    if len_longest_code_word as core::ffi::c_int >= 0 as core::ffi::c_int
        && len_longest_code_word as core::ffi::c_int <= 49 as core::ffi::c_int
    {
        (*ptr_aac_dec_channel_info).longest_cw_len = len_longest_code_word;
    } else if len_longest_code_word as core::ffi::c_int > 49 as core::ffi::c_int {
        (*ptr_aac_dec_channel_info).longest_cw_len = 49 as WORD8;
    }
}
unsafe extern "C" fn ixheaacd_rvlc_init(
    mut ptr_rvlc: *mut ia_rvlc_info_struct,
    mut ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
) -> IA_ERRORCODE {
    let mut ptr_scf_esc: *mut WORD16 = ((*ptr_aac_dec_channel_info).rvlc_scf_esc_arr)
        .as_mut_ptr();
    let mut ptr_scf_fwd: *mut WORD16 = ((*ptr_aac_dec_channel_info).rvlc_scf_fwd_arr)
        .as_mut_ptr();
    let mut ptr_scf_bwd: *mut WORD16 = ((*ptr_aac_dec_channel_info).rvlc_scf_bwd_arr)
        .as_mut_ptr();
    let mut ptr_scale_factor: *mut WORD16 = (*ptr_aac_dec_channel_info).ptr_scale_factor;
    let mut bnds: WORD32 = 0;
    (*ptr_aac_dec_channel_info).rvlc_intensity_used = 0 as WORD8;
    (*ptr_rvlc).num_esc_words_decoded = 0 as UWORD8;
    (*ptr_rvlc).num_fwd_esc_words_decoded = 0 as UWORD8;
    (*ptr_rvlc).num_bwd_esc_words_decoded = 0 as UWORD8;
    (*ptr_rvlc).intensity_used = 0 as WORD8;
    (*ptr_rvlc).rvlc_err_log = 0 as UWORD32;
    (*ptr_rvlc).conceal_max = CONCEAL_MAX_INIT as WORD16;
    (*ptr_rvlc).conceal_min = CONCEAL_MIN_INIT as WORD16;
    (*ptr_rvlc).conceal_max_esc = CONCEAL_MAX_INIT as WORD16;
    (*ptr_rvlc).conceal_min_esc = CONCEAL_MIN_INIT as WORD16;
    bnds = 0 as core::ffi::c_int as WORD32;
    while bnds < RVLC_MAX_SFB {
        *ptr_scf_fwd.offset(bnds as isize) = 0 as WORD16;
        *ptr_scf_bwd.offset(bnds as isize) = 0 as WORD16;
        *ptr_scf_esc.offset(bnds as isize) = 0 as WORD16;
        *ptr_scale_factor.offset(bnds as isize) = 0 as WORD16;
        bnds += 1;
    }
    (*ptr_rvlc).rvl_fwd_bit_str_idx = ((*it_bit_buff).size - (*it_bit_buff).cnt_bits)
        as UWORD16;
    (*ptr_rvlc).rvl_bwd_bit_str_idx = ((*it_bit_buff).size as core::ffi::c_int
        - (*it_bit_buff).cnt_bits as core::ffi::c_int
        + (*ptr_rvlc).rvlc_sf_len as core::ffi::c_int - 1 as core::ffi::c_int)
        as UWORD16;
    (*it_bit_buff).cnt_bits -= (*ptr_rvlc).rvlc_sf_len as core::ffi::c_int;
    (*it_bit_buff).ptr_read_next = ((*it_bit_buff).ptr_bit_buf_base)
        .offset(
            ((*it_bit_buff).size - (*it_bit_buff).cnt_bits >> 3 as core::ffi::c_int)
                as isize,
        );
    (*it_bit_buff).bit_pos = ((*it_bit_buff).size as core::ffi::c_int
        - (*it_bit_buff).cnt_bits as core::ffi::c_int & 7 as core::ffi::c_int) as WORD32;
    if (*ptr_rvlc).sf_esc_present != 0 as core::ffi::c_int {
        (*ptr_rvlc).esc_bit_str_idx = ((*it_bit_buff).size - (*it_bit_buff).cnt_bits)
            as UWORD16;
        (*it_bit_buff).cnt_bits -= (*ptr_rvlc).rvlc_esc_len as core::ffi::c_int;
        (*it_bit_buff).ptr_read_next = ((*it_bit_buff).ptr_bit_buf_base)
            .offset(
                ((*it_bit_buff).size - (*it_bit_buff).cnt_bits >> 3 as core::ffi::c_int)
                    as isize,
            );
        (*it_bit_buff).bit_pos = ((*it_bit_buff).size as core::ffi::c_int
            - (*it_bit_buff).cnt_bits as core::ffi::c_int & 7 as core::ffi::c_int)
            as WORD32;
    }
    if (*it_bit_buff).cnt_bits < 0 as core::ffi::c_int {
        return IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES
    } else {
        return IA_NO_ERROR
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_bi_dir_est_scf_prev_frame_reference(
    mut ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
    mut ptr_aac_dec_static_channel_info: *mut ia_aac_dec_overlap_info,
) -> VOID {
    let mut ptr_rvlc: *mut ia_rvlc_info_struct = &mut (*ptr_aac_dec_channel_info)
        .ptr_rvlc_info;
    let mut band: WORD32 = 0;
    let mut bnds: WORD32 = 0;
    let mut start_band: WORD32 = 0;
    let mut end_band: WORD32 = 0;
    let mut group: WORD32 = 0;
    let mut conceal_min: WORD32 = 0;
    let mut conceal_max: WORD32 = 0;
    let mut conceal_group_min: WORD32 = 0;
    let mut conceal_group_max: WORD32 = 0;
    let mut max_scf_bands: WORD32 = 0;
    let mut common_min: WORD32 = 0;
    if (*ptr_aac_dec_channel_info).str_ics_info.window_sequence as core::ffi::c_int
        == EIGHT_SHORT_SEQUENCE
    {
        max_scf_bands = 16 as core::ffi::c_int as WORD32;
    } else {
        max_scf_bands = 64 as core::ffi::c_int as WORD32;
    }
    if (*ptr_rvlc).conceal_min as core::ffi::c_int == CONCEAL_MIN_INIT {
        (*ptr_rvlc).conceal_min = 0 as WORD16;
    }
    if (*ptr_rvlc).conceal_max as core::ffi::c_int == CONCEAL_MAX_INIT {
        (*ptr_rvlc).conceal_max = (((*ptr_rvlc).num_wind_grps as core::ffi::c_int
            - 1 as core::ffi::c_int) * 16 as core::ffi::c_int
            + (*ptr_rvlc).max_sfb_transmitted as core::ffi::c_int
            - 1 as core::ffi::c_int) as WORD16;
    }
    conceal_min = (*ptr_rvlc).conceal_min as WORD32 % max_scf_bands;
    conceal_group_min = (*ptr_rvlc).conceal_min as WORD32 / max_scf_bands;
    conceal_max = (*ptr_rvlc).conceal_max as WORD32 % max_scf_bands;
    conceal_group_max = (*ptr_rvlc).conceal_max as WORD32 / max_scf_bands;
    (*ptr_aac_dec_channel_info).rvlc_scf_fwd_arr[(*ptr_rvlc).conceal_max as usize] = (*ptr_aac_dec_channel_info)
        .rvlc_scf_bwd_arr[(*ptr_rvlc).conceal_max as usize];
    (*ptr_aac_dec_channel_info).rvlc_scf_bwd_arr[(*ptr_rvlc).conceal_min as usize] = (*ptr_aac_dec_channel_info)
        .rvlc_scf_fwd_arr[(*ptr_rvlc).conceal_min as usize];
    start_band = conceal_min;
    if conceal_group_min == conceal_group_max {
        end_band = conceal_max;
    } else {
        end_band = ((*ptr_rvlc).max_sfb_transmitted as core::ffi::c_int
            - 1 as core::ffi::c_int) as WORD32;
    }
    group = conceal_group_min;
    while group <= conceal_group_max {
        band = start_band;
        while band <= end_band {
            bnds = 16 as WORD32 * group + band;
            match *((*ptr_aac_dec_channel_info).ptr_code_book).offset(bnds as isize)
                as core::ffi::c_int
            {
                ZERO_HCB => {
                    *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                        .offset(bnds as isize) = 0 as WORD16;
                }
                INTENSITY_HCB | INTENSITY_HCB2 => {
                    if (*ptr_aac_dec_static_channel_info).rvlc_prev_cb[bnds as usize]
                        as core::ffi::c_int == INTENSITY_HCB
                        || (*ptr_aac_dec_static_channel_info).rvlc_prev_cb[bnds as usize]
                            as core::ffi::c_int == INTENSITY_HCB2
                    {
                        common_min = ixheaac_min32(
                            (*ptr_aac_dec_channel_info).rvlc_scf_fwd_arr[bnds as usize]
                                as WORD32,
                            (*ptr_aac_dec_channel_info).rvlc_scf_bwd_arr[bnds as usize]
                                as WORD32,
                        );
                        *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                            .offset(bnds as isize) = ixheaac_min32(
                            common_min,
                            (*ptr_aac_dec_static_channel_info)
                                .rvlc_prev_sf[bnds as usize] as WORD32,
                        ) as WORD16;
                    } else {
                        *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                            .offset(bnds as isize) = ixheaac_min32(
                            (*ptr_aac_dec_channel_info).rvlc_scf_fwd_arr[bnds as usize]
                                as WORD32,
                            (*ptr_aac_dec_channel_info).rvlc_scf_bwd_arr[bnds as usize]
                                as WORD32,
                        ) as WORD16;
                    }
                }
                NOISE_HCB => {
                    if (*ptr_aac_dec_static_channel_info).rvlc_prev_cb[bnds as usize]
                        as core::ffi::c_int == NOISE_HCB
                    {
                        common_min = ixheaac_min32(
                            (*ptr_aac_dec_channel_info).rvlc_scf_fwd_arr[bnds as usize]
                                as WORD32,
                            (*ptr_aac_dec_channel_info).rvlc_scf_bwd_arr[bnds as usize]
                                as WORD32,
                        );
                        *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                            .offset(bnds as isize) = ixheaac_min32(
                            common_min,
                            (*ptr_aac_dec_static_channel_info)
                                .rvlc_prev_sf[bnds as usize] as WORD32,
                        ) as WORD16;
                    } else {
                        *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                            .offset(bnds as isize) = ixheaac_min32(
                            (*ptr_aac_dec_channel_info).rvlc_scf_fwd_arr[bnds as usize]
                                as WORD32,
                            (*ptr_aac_dec_channel_info).rvlc_scf_bwd_arr[bnds as usize]
                                as WORD32,
                        ) as WORD16;
                    }
                }
                _ => {
                    if (*ptr_aac_dec_static_channel_info).rvlc_prev_cb[bnds as usize]
                        as core::ffi::c_int != ZERO_HCB
                        && (*ptr_aac_dec_static_channel_info).rvlc_prev_cb[bnds as usize]
                            as core::ffi::c_int != NOISE_HCB
                        && (*ptr_aac_dec_static_channel_info).rvlc_prev_cb[bnds as usize]
                            as core::ffi::c_int != INTENSITY_HCB
                        && (*ptr_aac_dec_static_channel_info).rvlc_prev_cb[bnds as usize]
                            as core::ffi::c_int != INTENSITY_HCB2
                    {
                        common_min = ixheaac_min32(
                            (*ptr_aac_dec_channel_info).rvlc_scf_fwd_arr[bnds as usize]
                                as WORD32,
                            (*ptr_aac_dec_channel_info).rvlc_scf_bwd_arr[bnds as usize]
                                as WORD32,
                        );
                        *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                            .offset(bnds as isize) = ixheaac_min32(
                            common_min,
                            (*ptr_aac_dec_static_channel_info)
                                .rvlc_prev_sf[bnds as usize] as WORD32,
                        ) as WORD16;
                    } else {
                        *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                            .offset(bnds as isize) = ixheaac_min32(
                            (*ptr_aac_dec_channel_info).rvlc_scf_fwd_arr[bnds as usize]
                                as WORD32,
                            (*ptr_aac_dec_channel_info).rvlc_scf_bwd_arr[bnds as usize]
                                as WORD32,
                        ) as WORD16;
                    }
                }
            }
            band += 1;
        }
        start_band = 0 as core::ffi::c_int as WORD32;
        if group as core::ffi::c_int + 1 as core::ffi::c_int == conceal_group_max {
            end_band = conceal_max;
        }
        group += 1;
    }
    if conceal_group_min == 0 as core::ffi::c_int {
        end_band = conceal_min;
    } else {
        end_band = (*ptr_rvlc).max_sfb_transmitted as WORD32;
    }
    group = 0 as core::ffi::c_int as WORD32;
    while group <= conceal_group_min {
        band = 0 as core::ffi::c_int as WORD32;
        while band < end_band {
            bnds = 16 as WORD32 * group + band;
            *((*ptr_aac_dec_channel_info).ptr_scale_factor).offset(bnds as isize) = (*ptr_aac_dec_channel_info)
                .rvlc_scf_fwd_arr[bnds as usize];
            band += 1;
        }
        if group as core::ffi::c_int + 1 as core::ffi::c_int == conceal_group_min {
            end_band = conceal_min;
        }
        group += 1;
    }
    start_band = (conceal_max as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
    group = conceal_group_max;
    while group < (*ptr_rvlc).num_wind_grps as core::ffi::c_int {
        band = start_band;
        while band < (*ptr_rvlc).max_sfb_transmitted as core::ffi::c_int {
            bnds = 16 as WORD32 * group + band;
            *((*ptr_aac_dec_channel_info).ptr_scale_factor).offset(bnds as isize) = (*ptr_aac_dec_channel_info)
                .rvlc_scf_bwd_arr[bnds as usize];
            band += 1;
        }
        start_band = 0 as core::ffi::c_int as WORD32;
        group += 1;
    }
}
unsafe extern "C" fn ixheaacd_calc_ref_val_fwd(
    mut ptr_rvlc: *mut ia_rvlc_info_struct,
    mut ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
    mut ref_fwd: *mut WORD32,
    mut ref_nrg_fwd: *mut WORD32,
    mut ref_scf_fwd: *mut WORD32,
) -> VOID {
    let mut band: WORD32 = 0;
    let mut bnds: WORD32 = 0;
    let mut group: WORD32 = 0;
    let mut start_band: WORD32 = 0;
    let mut id_is: WORD32 = 0;
    let mut id_nrg: WORD32 = 0;
    let mut id_scf: WORD32 = 0;
    let mut conceal_min: WORD32 = 0;
    let mut conceal_group_min: WORD32 = 0;
    let mut max_scf_bands: WORD32 = 0;
    if (*ptr_aac_dec_channel_info).str_ics_info.window_sequence as core::ffi::c_int
        == EIGHT_SHORT_SEQUENCE
    {
        max_scf_bands = 16 as core::ffi::c_int as WORD32;
    } else {
        max_scf_bands = 64 as core::ffi::c_int as WORD32;
    }
    conceal_min = (*ptr_rvlc).conceal_min as WORD32 % max_scf_bands;
    conceal_group_min = (*ptr_rvlc).conceal_min as WORD32 / max_scf_bands;
    id_scf = 1 as core::ffi::c_int as WORD32;
    id_nrg = id_scf;
    id_is = id_nrg;
    *ref_nrg_fwd = ((*ptr_aac_dec_channel_info).global_gain as core::ffi::c_int
        - 90 as core::ffi::c_int - 256 as core::ffi::c_int) as WORD32;
    *ref_scf_fwd = (*ptr_aac_dec_channel_info).global_gain as WORD32;
    start_band = (conceal_min as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    group = conceal_group_min;
    while group >= 0 as core::ffi::c_int {
        band = start_band;
        while band >= 0 as core::ffi::c_int {
            bnds = 16 as WORD32 * group + band;
            match *((*ptr_aac_dec_channel_info).ptr_code_book).offset(bnds as isize)
                as core::ffi::c_int
            {
                ZERO_HCB => {}
                INTENSITY_HCB | INTENSITY_HCB2 => {
                    if id_is != 0 {
                        *ref_fwd = (*ptr_aac_dec_channel_info)
                            .rvlc_scf_fwd_arr[bnds as usize] as WORD32;
                        id_is = 0 as core::ffi::c_int as WORD32;
                    }
                }
                NOISE_HCB => {
                    if id_nrg != 0 {
                        *ref_nrg_fwd = (*ptr_aac_dec_channel_info)
                            .rvlc_scf_fwd_arr[bnds as usize] as WORD32;
                        id_nrg = 0 as core::ffi::c_int as WORD32;
                    }
                }
                _ => {
                    if id_scf != 0 {
                        *ref_scf_fwd = (*ptr_aac_dec_channel_info)
                            .rvlc_scf_fwd_arr[bnds as usize] as WORD32;
                        id_scf = 0 as core::ffi::c_int as WORD32;
                    }
                }
            }
            band -= 1;
        }
        start_band = ((*ptr_rvlc).max_sfb_transmitted as core::ffi::c_int
            - 1 as core::ffi::c_int) as WORD32;
        group -= 1;
    }
}
unsafe extern "C" fn ixheaacd_calc_ref_val_bwd(
    mut ptr_rvlc: *mut ia_rvlc_info_struct,
    mut ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
    mut ref_bwd: *mut WORD32,
    mut ref_nrg_bwd: *mut WORD32,
    mut ref_scf_bwd: *mut WORD32,
) -> VOID {
    let mut band: WORD32 = 0;
    let mut bnds: WORD32 = 0;
    let mut group: WORD32 = 0;
    let mut start_band: WORD32 = 0;
    let mut id_is: WORD32 = 0;
    let mut id_nrg: WORD32 = 0;
    let mut id_scf: WORD32 = 0;
    let mut conceal_max: WORD32 = 0;
    let mut conceal_group_max: WORD32 = 0;
    let mut max_scf_bands: WORD32 = 0;
    if (*ptr_aac_dec_channel_info).str_ics_info.window_sequence as core::ffi::c_int
        == EIGHT_SHORT_SEQUENCE
    {
        max_scf_bands = 16 as core::ffi::c_int as WORD32;
    } else {
        max_scf_bands = 64 as core::ffi::c_int as WORD32;
    }
    conceal_max = (*ptr_rvlc).conceal_max as WORD32 % max_scf_bands;
    conceal_group_max = (*ptr_rvlc).conceal_max as WORD32 / max_scf_bands;
    id_scf = 1 as core::ffi::c_int as WORD32;
    id_nrg = id_scf;
    id_is = id_nrg;
    *ref_bwd = (*ptr_rvlc).dpcm_is_last_pos;
    *ref_nrg_bwd = (*ptr_rvlc).rev_global_gain + (*ptr_rvlc).dpcm_noise_last_pos
        - 90 as WORD32 - 256 as WORD32 + (*ptr_rvlc).dpcm_noise_nrg;
    *ref_scf_bwd = (*ptr_rvlc).rev_global_gain;
    start_band = (conceal_max as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
    group = conceal_group_max;
    while group < (*ptr_rvlc).num_wind_grps as core::ffi::c_int {
        band = start_band;
        while band < (*ptr_rvlc).max_sfb_transmitted as core::ffi::c_int {
            bnds = 16 as WORD32 * group + band;
            match *((*ptr_aac_dec_channel_info).ptr_code_book).offset(bnds as isize)
                as core::ffi::c_int
            {
                ZERO_HCB => {}
                INTENSITY_HCB | INTENSITY_HCB2 => {
                    if id_is != 0 {
                        *ref_bwd = (*ptr_aac_dec_channel_info)
                            .rvlc_scf_bwd_arr[bnds as usize] as WORD32;
                        id_is = 0 as core::ffi::c_int as WORD32;
                    }
                }
                NOISE_HCB => {
                    if id_nrg != 0 {
                        *ref_nrg_bwd = (*ptr_aac_dec_channel_info)
                            .rvlc_scf_bwd_arr[bnds as usize] as WORD32;
                        id_nrg = 0 as core::ffi::c_int as WORD32;
                    }
                }
                _ => {
                    if id_scf != 0 {
                        *ref_scf_bwd = (*ptr_aac_dec_channel_info)
                            .rvlc_scf_bwd_arr[bnds as usize] as WORD32;
                        id_scf = 0 as core::ffi::c_int as WORD32;
                    }
                }
            }
            band += 1;
        }
        start_band = 0 as core::ffi::c_int as WORD32;
        group += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_bi_dir_est_lower_scf_cur_frame(
    mut ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
) -> VOID {
    let mut ptr_rvlc: *mut ia_rvlc_info_struct = &mut (*ptr_aac_dec_channel_info)
        .ptr_rvlc_info;
    let mut band: WORD32 = 0;
    let mut bnds: WORD32 = 0;
    let mut start_band: WORD32 = 0;
    let mut end_band: WORD32 = 0;
    let mut group: WORD32 = 0;
    let mut conceal_min: WORD32 = 0;
    let mut conceal_max: WORD32 = 0;
    let mut conceal_group_min: WORD32 = 0;
    let mut conceal_group_max: WORD32 = 0;
    let mut max_scf_bands: WORD32 = 0;
    if (*ptr_aac_dec_channel_info).str_ics_info.window_sequence as core::ffi::c_int
        == EIGHT_SHORT_SEQUENCE
    {
        max_scf_bands = 16 as core::ffi::c_int as WORD32;
    } else {
        max_scf_bands = 64 as core::ffi::c_int as WORD32;
    }
    if (*ptr_rvlc).conceal_min as core::ffi::c_int == CONCEAL_MIN_INIT {
        (*ptr_rvlc).conceal_min = 0 as WORD16;
    }
    if (*ptr_rvlc).conceal_max as core::ffi::c_int == CONCEAL_MAX_INIT {
        (*ptr_rvlc).conceal_max = (((*ptr_rvlc).num_wind_grps as core::ffi::c_int
            - 1 as core::ffi::c_int) * 16 as core::ffi::c_int
            + (*ptr_rvlc).max_sfb_transmitted as core::ffi::c_int
            - 1 as core::ffi::c_int) as WORD16;
    }
    conceal_min = (*ptr_rvlc).conceal_min as WORD32 % max_scf_bands;
    conceal_group_min = (*ptr_rvlc).conceal_min as WORD32 / max_scf_bands;
    conceal_max = (*ptr_rvlc).conceal_max as WORD32 % max_scf_bands;
    conceal_group_max = (*ptr_rvlc).conceal_max as WORD32 / max_scf_bands;
    if (*ptr_rvlc).conceal_min as core::ffi::c_int
        == (*ptr_rvlc).conceal_max as core::ffi::c_int
    {
        let mut ref_fwd: WORD32 = 0 as WORD32;
        let mut ref_nrg_fwd: WORD32 = 0 as WORD32;
        let mut ref_scf_fwd: WORD32 = 0 as WORD32;
        let mut ref_bwd: WORD32 = 0 as WORD32;
        let mut ref_nrg_bwd: WORD32 = 0 as WORD32;
        let mut ref_scf_bwd: WORD32 = 0 as WORD32;
        bnds = (*ptr_rvlc).conceal_min as WORD32;
        ixheaacd_calc_ref_val_fwd(
            ptr_rvlc,
            ptr_aac_dec_channel_info,
            &mut ref_fwd,
            &mut ref_nrg_fwd,
            &mut ref_scf_fwd,
        );
        ixheaacd_calc_ref_val_bwd(
            ptr_rvlc,
            ptr_aac_dec_channel_info,
            &mut ref_bwd,
            &mut ref_nrg_bwd,
            &mut ref_scf_bwd,
        );
        match *((*ptr_aac_dec_channel_info).ptr_code_book).offset(bnds as isize)
            as core::ffi::c_int
        {
            ZERO_HCB => {}
            INTENSITY_HCB | INTENSITY_HCB2 => {
                if ref_fwd < ref_bwd {
                    *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                        .offset(bnds as isize) = ref_fwd as WORD16;
                } else {
                    *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                        .offset(bnds as isize) = ref_bwd as WORD16;
                }
            }
            NOISE_HCB => {
                if ref_nrg_fwd < ref_nrg_bwd {
                    *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                        .offset(bnds as isize) = ref_nrg_fwd as WORD16;
                } else {
                    *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                        .offset(bnds as isize) = ref_nrg_bwd as WORD16;
                }
            }
            _ => {
                if ref_scf_fwd < ref_scf_bwd {
                    *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                        .offset(bnds as isize) = ref_scf_fwd as WORD16;
                } else {
                    *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                        .offset(bnds as isize) = ref_scf_bwd as WORD16;
                }
            }
        }
    } else {
        (*ptr_aac_dec_channel_info).rvlc_scf_fwd_arr[(*ptr_rvlc).conceal_max as usize] = (*ptr_aac_dec_channel_info)
            .rvlc_scf_bwd_arr[(*ptr_rvlc).conceal_max as usize];
        (*ptr_aac_dec_channel_info).rvlc_scf_bwd_arr[(*ptr_rvlc).conceal_min as usize] = (*ptr_aac_dec_channel_info)
            .rvlc_scf_fwd_arr[(*ptr_rvlc).conceal_min as usize];
        start_band = conceal_min;
        if conceal_group_min == conceal_group_max {
            end_band = conceal_max;
        } else {
            end_band = ((*ptr_rvlc).max_sfb_transmitted as core::ffi::c_int
                - 1 as core::ffi::c_int) as WORD32;
        }
        group = conceal_group_min;
        while group <= conceal_group_max {
            band = start_band;
            while band <= end_band {
                bnds = 16 as WORD32 * group + band;
                if ((*ptr_aac_dec_channel_info).rvlc_scf_fwd_arr[bnds as usize]
                    as core::ffi::c_int)
                    < (*ptr_aac_dec_channel_info).rvlc_scf_bwd_arr[bnds as usize]
                        as core::ffi::c_int
                {
                    *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                        .offset(bnds as isize) = (*ptr_aac_dec_channel_info)
                        .rvlc_scf_fwd_arr[bnds as usize];
                } else {
                    *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                        .offset(bnds as isize) = (*ptr_aac_dec_channel_info)
                        .rvlc_scf_bwd_arr[bnds as usize];
                }
                band += 1;
            }
            start_band = 0 as core::ffi::c_int as WORD32;
            if group as core::ffi::c_int + 1 as core::ffi::c_int == conceal_group_max {
                end_band = conceal_max;
            }
            group += 1;
        }
    }
    if conceal_group_min == 0 as core::ffi::c_int {
        end_band = conceal_min;
    } else {
        end_band = (*ptr_rvlc).max_sfb_transmitted as WORD32;
    }
    group = 0 as core::ffi::c_int as WORD32;
    while group <= conceal_group_min {
        band = 0 as core::ffi::c_int as WORD32;
        while band < end_band {
            bnds = 16 as WORD32 * group + band;
            *((*ptr_aac_dec_channel_info).ptr_scale_factor).offset(bnds as isize) = (*ptr_aac_dec_channel_info)
                .rvlc_scf_fwd_arr[bnds as usize];
            band += 1;
        }
        if group as core::ffi::c_int + 1 as core::ffi::c_int == conceal_group_min {
            end_band = conceal_min;
        }
        group += 1;
    }
    start_band = (conceal_max as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
    group = conceal_group_max;
    while group < (*ptr_rvlc).num_wind_grps as core::ffi::c_int {
        band = start_band;
        while band < (*ptr_rvlc).max_sfb_transmitted as core::ffi::c_int {
            bnds = 16 as WORD32 * group + band;
            *((*ptr_aac_dec_channel_info).ptr_scale_factor).offset(bnds as isize) = (*ptr_aac_dec_channel_info)
                .rvlc_scf_bwd_arr[bnds as usize];
            band += 1;
        }
        start_band = 0 as core::ffi::c_int as WORD32;
        group += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_statistical_estimation(
    mut ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
) -> VOID {
    let mut ptr_rvlc: *mut ia_rvlc_info_struct = &mut (*ptr_aac_dec_channel_info)
        .ptr_rvlc_info;
    let mut band: WORD32 = 0;
    let mut bnds: WORD32 = 0;
    let mut group: WORD32 = 0;
    let mut sum_fwd: WORD32 = 0;
    let mut sum_bwd: WORD32 = 0;
    let mut sum_nrg_fwd: WORD32 = 0;
    let mut sum_nrg_bwd: WORD32 = 0;
    let mut sum_scf_fwd: WORD32 = 0;
    let mut sum_scf_bwd: WORD32 = 0;
    let mut use_fwd: WORD32 = 0;
    let mut use_nrg_fwd: WORD32 = 0;
    let mut use_scf_fwd: WORD32 = 0;
    sum_scf_bwd = 0 as core::ffi::c_int as WORD32;
    sum_scf_fwd = sum_scf_bwd;
    sum_nrg_bwd = sum_scf_fwd;
    sum_nrg_fwd = sum_nrg_bwd;
    sum_bwd = sum_nrg_fwd;
    sum_fwd = sum_bwd;
    use_scf_fwd = 0 as core::ffi::c_int as WORD32;
    use_nrg_fwd = use_scf_fwd;
    use_fwd = use_nrg_fwd;
    group = 0 as core::ffi::c_int as WORD32;
    while group < (*ptr_rvlc).num_wind_grps as core::ffi::c_int {
        band = 0 as core::ffi::c_int as WORD32;
        while band < (*ptr_rvlc).max_sfb_transmitted as core::ffi::c_int {
            bnds = 16 as WORD32 * group + band;
            match *((*ptr_aac_dec_channel_info).ptr_code_book).offset(bnds as isize)
                as core::ffi::c_int
            {
                ZERO_HCB => {}
                INTENSITY_HCB | INTENSITY_HCB2 => {
                    sum_fwd
                        += (*ptr_aac_dec_channel_info).rvlc_scf_fwd_arr[bnds as usize]
                            as core::ffi::c_int;
                    sum_bwd
                        += (*ptr_aac_dec_channel_info).rvlc_scf_bwd_arr[bnds as usize]
                            as core::ffi::c_int;
                }
                NOISE_HCB => {
                    sum_nrg_fwd
                        += (*ptr_aac_dec_channel_info).rvlc_scf_fwd_arr[bnds as usize]
                            as core::ffi::c_int;
                    sum_nrg_bwd
                        += (*ptr_aac_dec_channel_info).rvlc_scf_bwd_arr[bnds as usize]
                            as core::ffi::c_int;
                }
                _ => {
                    sum_scf_fwd
                        += (*ptr_aac_dec_channel_info).rvlc_scf_fwd_arr[bnds as usize]
                            as core::ffi::c_int;
                    sum_scf_bwd
                        += (*ptr_aac_dec_channel_info).rvlc_scf_bwd_arr[bnds as usize]
                            as core::ffi::c_int;
                }
            }
            band += 1;
        }
        group += 1;
    }
    if sum_fwd < sum_bwd {
        use_fwd = 1 as core::ffi::c_int as WORD32;
    }
    if sum_nrg_fwd < sum_nrg_bwd {
        use_nrg_fwd = 1 as core::ffi::c_int as WORD32;
    }
    if sum_scf_fwd < sum_scf_bwd {
        use_scf_fwd = 1 as core::ffi::c_int as WORD32;
    }
    group = 0 as core::ffi::c_int as WORD32;
    while group < (*ptr_rvlc).num_wind_grps as core::ffi::c_int {
        band = 0 as core::ffi::c_int as WORD32;
        while band < (*ptr_rvlc).max_sfb_transmitted as core::ffi::c_int {
            bnds = 16 as WORD32 * group + band;
            match *((*ptr_aac_dec_channel_info).ptr_code_book).offset(bnds as isize)
                as core::ffi::c_int
            {
                ZERO_HCB => {}
                INTENSITY_HCB | INTENSITY_HCB2 => {
                    if use_fwd != 0 {
                        *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                            .offset(bnds as isize) = (*ptr_aac_dec_channel_info)
                            .rvlc_scf_fwd_arr[bnds as usize];
                    } else {
                        *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                            .offset(bnds as isize) = (*ptr_aac_dec_channel_info)
                            .rvlc_scf_bwd_arr[bnds as usize];
                    }
                }
                NOISE_HCB => {
                    if use_nrg_fwd != 0 {
                        *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                            .offset(bnds as isize) = (*ptr_aac_dec_channel_info)
                            .rvlc_scf_fwd_arr[bnds as usize];
                    } else {
                        *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                            .offset(bnds as isize) = (*ptr_aac_dec_channel_info)
                            .rvlc_scf_bwd_arr[bnds as usize];
                    }
                }
                _ => {
                    if use_scf_fwd != 0 {
                        *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                            .offset(bnds as isize) = (*ptr_aac_dec_channel_info)
                            .rvlc_scf_fwd_arr[bnds as usize];
                    } else {
                        *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                            .offset(bnds as isize) = (*ptr_aac_dec_channel_info)
                            .rvlc_scf_bwd_arr[bnds as usize];
                    }
                }
            }
            band += 1;
        }
        group += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_predictive_interpolation(
    mut ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
    mut ptr_aac_dec_static_channel_info: *mut ia_aac_dec_overlap_info,
) -> VOID {
    let mut ptr_rvlc: *mut ia_rvlc_info_struct = &mut (*ptr_aac_dec_channel_info)
        .ptr_rvlc_info;
    let mut band: WORD32 = 0;
    let mut bnds: WORD32 = 0;
    let mut group: WORD32 = 0;
    let mut common_min: WORD32 = 0;
    group = 0 as core::ffi::c_int as WORD32;
    while group < (*ptr_rvlc).num_wind_grps as core::ffi::c_int {
        band = 0 as core::ffi::c_int as WORD32;
        while band < (*ptr_rvlc).max_sfb_transmitted as core::ffi::c_int {
            bnds = 16 as WORD32 * group + band;
            match *((*ptr_aac_dec_channel_info).ptr_code_book).offset(bnds as isize)
                as core::ffi::c_int
            {
                ZERO_HCB => {
                    *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                        .offset(bnds as isize) = 0 as WORD16;
                }
                INTENSITY_HCB | INTENSITY_HCB2 => {
                    if (*ptr_aac_dec_static_channel_info).rvlc_prev_cb[bnds as usize]
                        as core::ffi::c_int == INTENSITY_HCB
                        || (*ptr_aac_dec_static_channel_info).rvlc_prev_cb[bnds as usize]
                            as core::ffi::c_int == INTENSITY_HCB2
                    {
                        common_min = ixheaac_min32(
                            (*ptr_aac_dec_channel_info).rvlc_scf_fwd_arr[bnds as usize]
                                as WORD32,
                            (*ptr_aac_dec_channel_info).rvlc_scf_bwd_arr[bnds as usize]
                                as WORD32,
                        );
                        *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                            .offset(bnds as isize) = ixheaac_min32(
                            common_min,
                            (*ptr_aac_dec_static_channel_info)
                                .rvlc_prev_sf[bnds as usize] as WORD32,
                        ) as WORD16;
                    } else {
                        *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                            .offset(bnds as isize) = -(110 as core::ffi::c_int)
                            as WORD16;
                    }
                }
                NOISE_HCB => {
                    if (*ptr_aac_dec_static_channel_info).rvlc_prev_cb[bnds as usize]
                        as core::ffi::c_int == NOISE_HCB
                    {
                        common_min = ixheaac_min32(
                            (*ptr_aac_dec_channel_info).rvlc_scf_fwd_arr[bnds as usize]
                                as WORD32,
                            (*ptr_aac_dec_channel_info).rvlc_scf_bwd_arr[bnds as usize]
                                as WORD32,
                        );
                        *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                            .offset(bnds as isize) = ixheaac_min32(
                            common_min,
                            (*ptr_aac_dec_static_channel_info)
                                .rvlc_prev_sf[bnds as usize] as WORD32,
                        ) as WORD16;
                    } else {
                        *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                            .offset(bnds as isize) = -(110 as core::ffi::c_int)
                            as WORD16;
                    }
                }
                _ => {
                    if (*ptr_aac_dec_static_channel_info).rvlc_prev_cb[bnds as usize]
                        as core::ffi::c_int != ZERO_HCB
                        && (*ptr_aac_dec_static_channel_info).rvlc_prev_cb[bnds as usize]
                            as core::ffi::c_int != NOISE_HCB
                        && (*ptr_aac_dec_static_channel_info).rvlc_prev_cb[bnds as usize]
                            as core::ffi::c_int != INTENSITY_HCB
                        && (*ptr_aac_dec_static_channel_info).rvlc_prev_cb[bnds as usize]
                            as core::ffi::c_int != INTENSITY_HCB2
                    {
                        common_min = ixheaac_min32(
                            (*ptr_aac_dec_channel_info).rvlc_scf_fwd_arr[bnds as usize]
                                as WORD32,
                            (*ptr_aac_dec_channel_info).rvlc_scf_bwd_arr[bnds as usize]
                                as WORD32,
                        );
                        *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                            .offset(bnds as isize) = ixheaac_min32(
                            common_min,
                            (*ptr_aac_dec_static_channel_info)
                                .rvlc_prev_sf[bnds as usize] as WORD32,
                        ) as WORD16;
                    } else {
                        *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                            .offset(bnds as isize) = 0 as WORD16;
                    }
                }
            }
            band += 1;
        }
        group += 1;
    }
}
unsafe extern "C" fn ixheaacd_rvlc_final_error_detection(
    mut ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
    mut ptr_aac_dec_static_channel_info: *mut ia_aac_dec_overlap_info,
) -> VOID {
    let mut ptr_rvlc: *mut ia_rvlc_info_struct = &mut (*ptr_aac_dec_channel_info)
        .ptr_rvlc_info;
    let mut err_status_complete: UWORD8 = 0 as UWORD8;
    let mut err_status_length_fwd: UWORD8 = 0 as UWORD8;
    let mut err_status_length_bwd: UWORD8 = 0 as UWORD8;
    let mut err_status_length_escape: UWORD8 = 0 as UWORD8;
    let mut err_status_first_scf: UWORD8 = 0 as UWORD8;
    let mut err_status_last_scf: UWORD8 = 0 as UWORD8;
    let mut err_status_first_nrg: UWORD8 = 0 as UWORD8;
    let mut err_status_last_nrg: UWORD8 = 0 as UWORD8;
    let mut err_status_first_is: UWORD8 = 0 as UWORD8;
    let mut err_status_last_is: UWORD8 = 0 as UWORD8;
    let mut err_status_forbidden_cw_fwd: UWORD8 = 0 as UWORD8;
    let mut err_status_forbidden_cw_bwd: UWORD8 = 0 as UWORD8;
    let mut err_status_num_escapes_fwd: UWORD8 = 0 as UWORD8;
    let mut err_status_num_escapes_bwd: UWORD8 = 0 as UWORD8;
    let mut conceal_status: UWORD8 = 1 as UWORD8;
    let mut current_block_type: UWORD8 = 0;
    (*ptr_aac_dec_channel_info).rvlc_curr_sf_flag = 1 as WORD16;
    if (*ptr_rvlc).rvlc_err_log & RVLC_ERROR_FORBIDDEN_CW_DETECTED_FWD as UWORD32 != 0 {
        err_status_forbidden_cw_fwd = 1 as UWORD8;
    }
    if (*ptr_rvlc).rvlc_err_log & RVLC_ERROR_FORBIDDEN_CW_DETECTED_BWD as UWORD32 != 0 {
        err_status_forbidden_cw_bwd = 1 as UWORD8;
    }
    if (*ptr_rvlc).rvlc_sf_fwd_len != 0 {
        err_status_length_fwd = 1 as UWORD8;
    }
    if (*ptr_rvlc).rvlc_sf_bwd_len != 0 {
        err_status_length_bwd = 1 as UWORD8;
    }
    if (*ptr_rvlc).sf_esc_present != 0 {
        if (*ptr_rvlc).rvlc_esc_len != 0 {
            err_status_length_escape = 1 as UWORD8;
        }
    }
    if (*ptr_rvlc).sf_used != 0 {
        if (*ptr_rvlc).firt_scale_fac as core::ffi::c_int
            != (*ptr_aac_dec_channel_info).global_gain as core::ffi::c_int
        {
            err_status_first_scf = 1 as UWORD8;
        }
        if (*ptr_rvlc).last_scale_fac as core::ffi::c_int != (*ptr_rvlc).rev_global_gain
        {
            err_status_last_scf = 1 as UWORD8;
        }
    }
    if (*ptr_rvlc).noise_used != 0 {
        if (*ptr_rvlc).first_nrg as core::ffi::c_int
            != (*ptr_aac_dec_channel_info).global_gain as core::ffi::c_int
                + (*ptr_rvlc).dpcm_noise_nrg as core::ffi::c_int - 90 as core::ffi::c_int
                - 256 as core::ffi::c_int
        {
            err_status_first_nrg = 1 as UWORD8;
        }
        if (*ptr_rvlc).last_nrg as core::ffi::c_int
            != (*ptr_rvlc).rev_global_gain as core::ffi::c_int
                + (*ptr_rvlc).dpcm_noise_last_pos as core::ffi::c_int
                - 90 as core::ffi::c_int - 256 as core::ffi::c_int
        {
            err_status_last_nrg = 1 as UWORD8;
        }
    }
    if (*ptr_rvlc).intensity_used != 0 {
        if (*ptr_rvlc).is_first as core::ffi::c_int != 0 as core::ffi::c_int {
            err_status_first_is = 1 as UWORD8;
        }
        if (*ptr_rvlc).is_last as core::ffi::c_int != (*ptr_rvlc).dpcm_is_last_pos {
            err_status_last_is = 1 as UWORD8;
        }
    }
    if (*ptr_rvlc).num_fwd_esc_words_decoded as core::ffi::c_int
        != (*ptr_rvlc).num_esc_words_decoded as core::ffi::c_int
        && (*ptr_rvlc).conceal_max as core::ffi::c_int == CONCEAL_MAX_INIT
    {
        err_status_num_escapes_fwd = 1 as UWORD8;
    }
    if (*ptr_rvlc).num_bwd_esc_words_decoded as core::ffi::c_int
        != (*ptr_rvlc).num_esc_words_decoded as core::ffi::c_int
        && (*ptr_rvlc).conceal_min as core::ffi::c_int == CONCEAL_MIN_INIT
    {
        err_status_num_escapes_bwd = 1 as UWORD8;
    }
    if err_status_length_escape as core::ffi::c_int != 0
        || (*ptr_rvlc).conceal_max as core::ffi::c_int == CONCEAL_MAX_INIT
            && (*ptr_rvlc).num_fwd_esc_words_decoded as core::ffi::c_int
                != (*ptr_rvlc).num_esc_words_decoded as core::ffi::c_int
            && (err_status_last_scf as core::ffi::c_int != 0
                || err_status_last_nrg as core::ffi::c_int != 0
                || err_status_last_is as core::ffi::c_int != 0)
            && ((*ptr_rvlc).conceal_min as core::ffi::c_int == CONCEAL_MIN_INIT
                && (*ptr_rvlc).num_bwd_esc_words_decoded as core::ffi::c_int
                    != (*ptr_rvlc).num_esc_words_decoded as core::ffi::c_int
                && (err_status_first_scf as core::ffi::c_int != 0
                    || err_status_first_nrg as core::ffi::c_int != 0
                    || err_status_first_is as core::ffi::c_int != 0))
        || (*ptr_rvlc).conceal_max as core::ffi::c_int == CONCEAL_MAX_INIT
            && ((*ptr_rvlc).rev_global_gain as core::ffi::c_int
                - (*ptr_rvlc).last_scale_fac as core::ffi::c_int)
                < -(15 as core::ffi::c_int)
        || (*ptr_rvlc).conceal_min as core::ffi::c_int == CONCEAL_MIN_INIT
            && ((*ptr_aac_dec_channel_info).global_gain as core::ffi::c_int
                - (*ptr_rvlc).firt_scale_fac as core::ffi::c_int)
                < -(15 as core::ffi::c_int)
    {
        if (*ptr_rvlc).conceal_max as core::ffi::c_int == CONCEAL_MAX_INIT
            || (*ptr_rvlc).conceal_min as core::ffi::c_int == CONCEAL_MIN_INIT
        {
            (*ptr_rvlc).conceal_max = 0 as WORD16;
            (*ptr_rvlc).conceal_min = ixheaac_max32(
                0 as WORD32,
                ((*ptr_rvlc).num_wind_grps as WORD32 - 1 as WORD32) * 16 as WORD32
                    + (*ptr_rvlc).max_sfb_transmitted as WORD32 - 1 as WORD32,
            ) as WORD16;
        } else {
            (*ptr_rvlc).conceal_max = ixheaac_min32(
                (*ptr_rvlc).conceal_max as WORD32,
                (*ptr_rvlc).conceal_max_esc as WORD32,
            ) as WORD16;
            (*ptr_rvlc).conceal_min = ixheaac_max32(
                (*ptr_rvlc).conceal_min as WORD32,
                (*ptr_rvlc).conceal_min_esc as WORD32,
            ) as WORD16;
        }
    }
    err_status_complete = (err_status_last_scf as core::ffi::c_int != 0
        || err_status_first_scf as core::ffi::c_int != 0
        || err_status_last_nrg as core::ffi::c_int != 0
        || err_status_first_nrg as core::ffi::c_int != 0
        || err_status_last_is as core::ffi::c_int != 0
        || err_status_first_is as core::ffi::c_int != 0
        || err_status_forbidden_cw_fwd as core::ffi::c_int != 0
        || err_status_forbidden_cw_bwd as core::ffi::c_int != 0
        || err_status_length_fwd as core::ffi::c_int != 0
        || err_status_length_bwd as core::ffi::c_int != 0
        || err_status_length_escape as core::ffi::c_int != 0
        || err_status_num_escapes_fwd as core::ffi::c_int != 0
        || err_status_num_escapes_bwd as core::ffi::c_int != 0) as core::ffi::c_int
        as UWORD8;
    current_block_type = (if (*ptr_aac_dec_channel_info).str_ics_info.window_sequence
        as core::ffi::c_int == EIGHT_SHORT_SEQUENCE
    {
        0 as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    }) as UWORD8;
    if err_status_complete == 0 {
        let mut band: WORD32 = 0;
        let mut group: WORD32 = 0;
        let mut bnds: WORD32 = 0;
        let mut last_sfb_idx: WORD32 = 0;
        last_sfb_idx = (if (*ptr_rvlc).num_wind_grps as core::ffi::c_int
            > 1 as core::ffi::c_int
        {
            16 as core::ffi::c_int
        } else {
            64 as core::ffi::c_int
        }) as WORD32;
        group = 0 as core::ffi::c_int as WORD32;
        while group < (*ptr_rvlc).num_wind_grps as core::ffi::c_int {
            band = 0 as core::ffi::c_int as WORD32;
            while band < (*ptr_rvlc).max_sfb_transmitted as core::ffi::c_int {
                bnds = 16 as WORD32 * group + band;
                (*ptr_aac_dec_static_channel_info).rvlc_prev_sf[bnds as usize] = (*ptr_aac_dec_channel_info)
                    .rvlc_scf_fwd_arr[bnds as usize];
                *((*ptr_aac_dec_channel_info).ptr_scale_factor).offset(bnds as isize) = (*ptr_aac_dec_static_channel_info)
                    .rvlc_prev_sf[bnds as usize];
                band += 1;
            }
            group += 1;
        }
        group = 0 as core::ffi::c_int as WORD32;
        while group < (*ptr_rvlc).num_wind_grps as core::ffi::c_int {
            band = 0 as core::ffi::c_int as WORD32;
            while band < (*ptr_rvlc).max_sfb_transmitted as core::ffi::c_int {
                bnds = 16 as WORD32 * group + band;
                (*ptr_aac_dec_static_channel_info).rvlc_prev_cb[bnds as usize] = *((*ptr_aac_dec_channel_info)
                    .ptr_code_book)
                    .offset(bnds as isize) as WORD16;
                band += 1;
            }
            while band < last_sfb_idx {
                bnds = 16 as WORD32 * group + band;
                (*ptr_aac_dec_static_channel_info).rvlc_prev_cb[bnds as usize] = ZERO_HCB
                    as WORD16;
                band += 1;
            }
            group += 1;
        }
    } else {
        let mut band_0: WORD32 = 0;
        let mut group_0: WORD32 = 0;
        if ((*ptr_rvlc).conceal_min as core::ffi::c_int != CONCEAL_MIN_INIT
            || (*ptr_rvlc).conceal_max as core::ffi::c_int != CONCEAL_MAX_INIT)
            && (*ptr_rvlc).conceal_min as core::ffi::c_int
                <= (*ptr_rvlc).conceal_max as core::ffi::c_int
            && (*ptr_aac_dec_static_channel_info).rvlc_prev_blk_type as core::ffi::c_int
                == current_block_type as core::ffi::c_int
            && (*ptr_aac_dec_static_channel_info).rvlc_prev_sf_ok as core::ffi::c_int
                != 0 && (*ptr_rvlc).sf_concealment != 0
            && conceal_status as core::ffi::c_int != 0
        {
            ixheaacd_bi_dir_est_scf_prev_frame_reference(
                ptr_aac_dec_channel_info,
                ptr_aac_dec_static_channel_info,
            );
            conceal_status = 0 as UWORD8;
        }
        if (*ptr_rvlc).conceal_min as core::ffi::c_int
            <= (*ptr_rvlc).conceal_max as core::ffi::c_int
            && ((*ptr_rvlc).conceal_min as core::ffi::c_int != CONCEAL_MIN_INIT
                || (*ptr_rvlc).conceal_max as core::ffi::c_int != CONCEAL_MAX_INIT)
            && !((*ptr_aac_dec_static_channel_info).rvlc_prev_sf_ok as core::ffi::c_int
                != 0 && (*ptr_rvlc).sf_concealment != 0
                && (*ptr_aac_dec_static_channel_info).rvlc_prev_blk_type
                    as core::ffi::c_int == current_block_type as core::ffi::c_int)
            && conceal_status as core::ffi::c_int != 0
        {
            ixheaacd_bi_dir_est_lower_scf_cur_frame(ptr_aac_dec_channel_info);
            conceal_status = 0 as UWORD8;
        }
        if (*ptr_rvlc).conceal_min as core::ffi::c_int
            <= (*ptr_rvlc).conceal_max as core::ffi::c_int
            && (err_status_last_scf as core::ffi::c_int != 0
                && err_status_first_scf as core::ffi::c_int != 0
                || err_status_last_nrg as core::ffi::c_int != 0
                    && err_status_first_nrg as core::ffi::c_int != 0
                || err_status_last_is as core::ffi::c_int != 0
                    && err_status_first_is as core::ffi::c_int != 0)
            && !(err_status_forbidden_cw_fwd as core::ffi::c_int != 0
                || err_status_forbidden_cw_bwd as core::ffi::c_int != 0
                || err_status_length_escape as core::ffi::c_int != 0)
            && conceal_status as core::ffi::c_int != 0
        {
            ixheaacd_statistical_estimation(ptr_aac_dec_channel_info);
            conceal_status = 0 as UWORD8;
        }
        if (*ptr_rvlc).conceal_min as core::ffi::c_int
            <= (*ptr_rvlc).conceal_max as core::ffi::c_int
            && (*ptr_aac_dec_static_channel_info).rvlc_prev_sf_ok as core::ffi::c_int
                != 0 && (*ptr_rvlc).sf_concealment != 0
            && (*ptr_aac_dec_static_channel_info).rvlc_prev_blk_type as core::ffi::c_int
                == current_block_type as core::ffi::c_int
            && conceal_status as core::ffi::c_int != 0
        {
            ixheaacd_predictive_interpolation(
                ptr_aac_dec_channel_info,
                ptr_aac_dec_static_channel_info,
            );
            conceal_status = 0 as UWORD8;
        }
        if conceal_status != 0 {
            group_0 = 0 as core::ffi::c_int as WORD32;
            while group_0 < (*ptr_rvlc).num_wind_grps as core::ffi::c_int {
                band_0 = 0 as core::ffi::c_int as WORD32;
                while band_0 < (*ptr_rvlc).max_sfb_transmitted as core::ffi::c_int {
                    *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                        .offset((16 as WORD32 * group_0 + band_0) as isize) = 0
                        as WORD16;
                    band_0 += 1;
                }
                group_0 += 1;
            }
            (*ptr_aac_dec_channel_info).rvlc_curr_sf_flag = 0 as WORD16;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_rvlc_dec(
    mut ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
    mut ptr_aac_dec_static_channel_info: *mut ia_aac_dec_overlap_info,
    mut it_bit_buff: *mut ia_bit_buf_struct,
) -> IA_ERRORCODE {
    let mut ptr_rvlc: *mut ia_rvlc_info_struct = &mut (*ptr_aac_dec_channel_info)
        .ptr_rvlc_info;
    let mut saved_it_bit_buff: ia_bit_buf_struct = ia_bit_buf_struct {
        ptr_bit_buf_base: 0 as *mut UWORD8,
        ptr_bit_buf_end: 0 as *mut UWORD8,
        ptr_read_next: 0 as *mut UWORD8,
        bit_pos: 0,
        cnt_bits: 0,
        size: 0,
        adts_header_present: 0,
        crc_check: 0,
        protection_absent: 0,
        no_raw_data_blocks: 0,
        str_adts_crc_info: ia_adts_crc_info_struct {
            crc_active: 0,
            no_reg: 0,
            file_value: 0,
            crc_lookup: [0; 256],
            str_crc_reg_data: [ia_crc_reg_data_struct {
                active: 0,
                buf_size: 0,
                max_bits: 0,
                bit_cnt: 0,
                bit_buf_cnt: 0,
                str_bit_buf: ia_crc_bit_buf_struct {
                    ptr_bit_buf_base: 0 as *mut UWORD8,
                    ptr_bit_buf_end: 0 as *mut UWORD8,
                    ptr_read_next: 0 as *mut UWORD8,
                    bit_pos: 0,
                    cnt_bits: 0,
                    size: 0,
                },
            }; 7],
        },
        pstr_adts_crc_info: 0 as *mut ia_adts_crc_info_struct,
        initial_cnt_bits: 0,
        audio_mux_align: 0,
        bit_count: 0,
        valid_bits: 0,
        byte: 0,
        byte_ptr: 0 as *mut UWORD8,
        ptr_start: 0 as *mut UWORD8,
        write_bit_count: 0,
        max_size: 0,
        xaac_jmp_buf: 0 as *mut jmp_buf,
    };
    let mut error_code: IA_ERRORCODE = 0 as IA_ERRORCODE;
    error_code = ixheaacd_rvlc_init(ptr_rvlc, ptr_aac_dec_channel_info, it_bit_buff);
    if error_code != 0 {
        return error_code;
    }
    saved_it_bit_buff = *it_bit_buff;
    if (*ptr_rvlc).sf_esc_present != 0 {
        ixheaacd_rvlc_decode_escape(
            ptr_rvlc,
            ((*ptr_aac_dec_channel_info).rvlc_scf_esc_arr).as_mut_ptr(),
            it_bit_buff,
        );
    }
    ixheaacd_rvlc_decode_forward(ptr_rvlc, ptr_aac_dec_channel_info, it_bit_buff);
    ixheaacd_rvlc_decode_backward(ptr_rvlc, ptr_aac_dec_channel_info, it_bit_buff);
    ixheaacd_rvlc_final_error_detection(
        ptr_aac_dec_channel_info,
        ptr_aac_dec_static_channel_info,
    );
    (*ptr_aac_dec_channel_info).rvlc_intensity_used = (*ptr_rvlc).intensity_used;
    (*ptr_aac_dec_channel_info).str_pns_info.pns_active = (*ptr_rvlc).noise_used
        as UWORD16;
    *it_bit_buff = saved_it_bit_buff;
    return error_code;
}
