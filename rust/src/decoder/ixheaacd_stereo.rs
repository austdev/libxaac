pub type WORD8 = core::ffi::c_schar;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD16 = core::ffi::c_short;
pub type UWORD16 = core::ffi::c_ushort;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type WORD64 = core::ffi::c_longlong;
pub type VOID = ();
pub type FLAG = core::ffi::c_int;
pub type WORD = core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sampling_rate_info_struct {
    pub sampling_frequency: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_block_tables_struct {
    pub ixheaacd_pow_table_Q13: [WORD32; 129],
    pub scale_table: [WORD32; 4],
    pub tns_max_bands_tbl: [[WORD8; 2]; 12],
    pub tns_coeff3_16: [WORD16; 8],
    pub tns_coeff4_16: [WORD16; 16],
    pub scale_mant_tab: [WORD32; 4],
    pub tns_coeff3: [WORD32; 8],
    pub tns_coeff4: [WORD32; 16],
    pub tns_coeff3_32: [WORD32; 8],
    pub tns_coeff4_32: [WORD32; 16],
    pub tns_max_bands_tbl_usac: [[WORD32; 2]; 16],
    pub tns_max_bands_tbl_ld: [WORD8; 12],
    pub tns_max_bands_tbl_480: [WORD8; 12],
    pub scale_table_960: [WORD32; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_huffman_tables_struct {
    pub ixheaacd_sfb_96_1024: [WORD8; 43],
    pub ixheaacd_sfb_96_128: [WORD8; 14],
    pub ixheaacd_sfb_64_1024: [WORD8; 49],
    pub ixheaacd_sfb_48_1024: [WORD8; 51],
    pub ixheaacd_sfb_48_128: [WORD8; 16],
    pub ixheaacd_sfb_32_1024: [WORD8; 53],
    pub ixheaacd_sfb_24_1024: [WORD8; 49],
    pub ixheaacd_sfb_24_128: [WORD8; 17],
    pub ixheaacd_sfb_16_1024: [WORD8; 45],
    pub ixheaacd_sfb_16_128: [WORD8; 17],
    pub ixheaacd_sfb_8_1024: [WORD8; 42],
    pub ixheaacd_sfb_8_128: [WORD8; 17],
    pub str_sample_rate_info: [ia_sampling_rate_info_struct; 13],
    pub idx_table_hf11: [UWORD32; 21],
    pub idx_table_hf10: [UWORD32; 20],
    pub idx_table_hf9: [UWORD32; 23],
    pub idx_table_hf8: [UWORD32; 17],
    pub idx_table_hf7: [UWORD32; 18],
    pub idx_table_hf6: [UWORD32; 17],
    pub idx_table_hf5: [UWORD32; 19],
    pub idx_table_hf4: [UWORD32; 19],
    pub idx_table_hf3: [UWORD32; 27],
    pub idx_table_hf2: [UWORD32; 16],
    pub idx_table_hf1: [UWORD32; 12],
    pub input_table_cb11: [UWORD16; 290],
    pub input_table_cb10: [UWORD16; 170],
    pub input_table_cb9: [UWORD16; 170],
    pub input_table_cb8: [UWORD16; 65],
    pub input_table_cb7: [UWORD16; 65],
    pub input_table_cb6: [UWORD16; 82],
    pub input_table_cb5: [UWORD16; 82],
    pub input_table_cb4: [UWORD16; 82],
    pub input_table_cb3: [UWORD16; 82],
    pub input_table_cb2: [UWORD16; 82],
    pub input_table_cb1: [UWORD16; 82],
    pub huffman_code_book_scl: [UWORD16; 122],
    pub huffman_code_book_scl_index: [UWORD32; 33],
    pub ixheaacd_sfb_48_512: [WORD8; 37],
    pub ixheaacd_sfb_32_512: [WORD8; 38],
    pub ixheaacd_sfb_24_512: [WORD8; 32],
    pub ixheaacd_sfb_48_480: [WORD8; 36],
    pub ixheaacd_sfb_32_480: [WORD8; 38],
    pub ixheaacd_sfb_24_480: [WORD8; 31],
    pub ixheaacd_sfb_96_960: [WORD8; 41],
    pub ixheaacd_sfb_96_120: [WORD8; 13],
    pub ixheaacd_sfb_64_960: [WORD8; 47],
    pub ixheaacd_sfb_48_960: [WORD8; 50],
    pub ixheaacd_sfb_48_120: [WORD8; 15],
    pub ixheaacd_sfb_24_960: [WORD8; 47],
    pub ixheaacd_sfb_24_120: [WORD8; 16],
    pub ixheaacd_sfb_16_960: [WORD8; 43],
    pub ixheaacd_sfb_16_120: [WORD8; 16],
    pub ixheaacd_sfb_8_960: [WORD8; 41],
    pub ixheaacd_sfb_8_120: [WORD8; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_imdct_tables_struct {
    pub cosine_array_2048_256: [WORD16; 514],
    pub dig_rev_table8_long: [WORD8; 64],
    pub dig_rev_table8_short: [WORD8; 8],
    pub fft_twiddle: [WORD32; 448],
    pub only_long_window_sine: [WORD16; 1024],
    pub only_long_window_kbd: [WORD16; 1024],
    pub only_short_window_sine: [WORD16; 128],
    pub only_short_window_kbd: [WORD16; 128],
    pub cosine_array_2048_256p: [WORD16; 514],
    pub w1024: [WORD32; 768],
    pub bit_rev_1024: [UWORD8; 256],
    pub bit_rev_512: [UWORD8; 64],
    pub bit_rev_128: [UWORD8; 16],
    pub bit_rev_32: [UWORD8; 4],
    pub w_256: [WORD32; 504],
    pub low_overlap_win: [WORD32; 512],
    pub window_sine_512: [WORD32; 512],
    pub cosine_array_1024: [WORD32; 512],
    pub low_overlap_win_480: [WORD32; 480],
    pub window_sine_480: [WORD32; 480],
    pub re_arr_tab_16: [UWORD8; 240],
    pub re_arr_tab_sml_240: [UWORD8; 240],
    pub cosine_array_960: [WORD32; 480],
    pub w_16: [WORD32; 24],
    pub window_sine_480_eld: [WORD16; 1920],
    pub window_sine_512_eld: [WORD16; 2048],
    pub only_long_window_sine_960: [WORD16; 960],
    pub only_long_window_kbd_960: [WORD16; 960],
    pub only_short_window_sine_120: [WORD16; 120],
    pub only_short_window_kbd_120: [WORD16; 120],
    pub re_arr_tab_32: [WORD16; 480],
    pub re_arr_tab_sml: [WORD16; 16],
    pub re_arr_tab_4: [WORD16; 60],
    pub re_arr_tab_15_4: [WORD16; 60],
    pub re_arr_tab_120: [WORD16; 60],
    pub re_arr_tab_5: [WORD16; 16],
    pub re_arr_tab_3: [WORD16; 16],
    pub re_arr_tab_sml_480: [WORD16; 480],
    pub cosine_array_1920: [WORD32; 960],
    pub w_512: [WORD16; 1020],
    pub w_32: [WORD16; 60],
    pub cosine_array_240: [WORD16; 120],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_sfb_info {
    pub sfb_index: *mut WORD16,
    pub sfb_width: *mut WORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_tables_struct {
    pub pstr_block_tables: *mut ia_aac_dec_block_tables_struct,
    pub pstr_huffmann_tables: *mut ia_aac_dec_huffman_tables_struct,
    pub pstr_imdct_tables: *mut ia_aac_dec_imdct_tables_struct,
    pub str_aac_sfb_info: [ia_aac_sfb_info; 4],
    pub scale_factor_bands_long: [*mut WORD8; 24],
    pub scale_factor_bands_short: [*mut WORD8; 24],
    pub sfb_long_table: [WORD16; 52],
    pub sfb_short_table: [WORD16; 16],
    pub code_book: [*mut UWORD16; 13],
    pub index_table: [*mut UWORD32; 13],
    pub scale_fac_bands_512: [*mut WORD8; 16],
    pub scale_fac_bands_480: [*mut WORD8; 16],
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
pub type AUDIO_OBJECT_TYPE = core::ffi::c_uint;
pub const AOT_USAC: AUDIO_OBJECT_TYPE = 42;
pub const AOT_ER_AAC_ELD: AUDIO_OBJECT_TYPE = 39;
pub const AOT_ESC: AUDIO_OBJECT_TYPE = 31;
pub const AOT_RSVD_31: AUDIO_OBJECT_TYPE = 31;
pub const AOT_RSVD_30: AUDIO_OBJECT_TYPE = 30;
pub const AOT_PS: AUDIO_OBJECT_TYPE = 29;
pub const AOT_RSVD_28: AUDIO_OBJECT_TYPE = 28;
pub const AOT_ER_PARA: AUDIO_OBJECT_TYPE = 27;
pub const AOT_ER_HILN: AUDIO_OBJECT_TYPE = 26;
pub const AOT_ER_HVXC: AUDIO_OBJECT_TYPE = 25;
pub const AOT_ER_CELP: AUDIO_OBJECT_TYPE = 24;
pub const AOT_ER_AAC_LD: AUDIO_OBJECT_TYPE = 23;
pub const AOT_ER_BSAC: AUDIO_OBJECT_TYPE = 22;
pub const AOT_ER_TWIN_VQ: AUDIO_OBJECT_TYPE = 21;
pub const AOT_ER_AAC_SCAL: AUDIO_OBJECT_TYPE = 20;
pub const AOT_ER_AAC_LTP: AUDIO_OBJECT_TYPE = 19;
pub const AOT_RSVD_18: AUDIO_OBJECT_TYPE = 18;
pub const AOT_ER_AAC_LC: AUDIO_OBJECT_TYPE = 17;
pub const AOT_ALG_SYNTH_AUD_FX: AUDIO_OBJECT_TYPE = 16;
pub const AOT_GEN_MIDI: AUDIO_OBJECT_TYPE = 15;
pub const AOT_WAV_TAB_SYNTH: AUDIO_OBJECT_TYPE = 14;
pub const AOT_MAIN_SYNTH: AUDIO_OBJECT_TYPE = 13;
pub const AOT_TTSI: AUDIO_OBJECT_TYPE = 12;
pub const AOT_RSVD_11: AUDIO_OBJECT_TYPE = 11;
pub const AOT_RSVD_10: AUDIO_OBJECT_TYPE = 10;
pub const AOT_HVXC: AUDIO_OBJECT_TYPE = 9;
pub const AOT_CELP: AUDIO_OBJECT_TYPE = 8;
pub const AOT_TWIN_VQ: AUDIO_OBJECT_TYPE = 7;
pub const AOT_AAC_SCAL: AUDIO_OBJECT_TYPE = 6;
pub const AOT_SBR: AUDIO_OBJECT_TYPE = 5;
pub const AOT_AAC_LTP: AUDIO_OBJECT_TYPE = 4;
pub const AOT_AAC_SSR: AUDIO_OBJECT_TYPE = 3;
pub const AOT_AAC_LC: AUDIO_OBJECT_TYPE = 2;
pub const AOT_AAC_MAIN: AUDIO_OBJECT_TYPE = 1;
pub const AOT_NULL_OBJECT: AUDIO_OBJECT_TYPE = 0;
pub const MAX_32: WORD32 = 0x7fffffff as core::ffi::c_long as WORD32;
pub const MIN_32: WORD32 = 0x80000000 as core::ffi::c_long as WORD32;
#[inline]
unsafe extern "C" fn ixheaac_shl32(mut a: WORD32, mut b: WORD) -> WORD32 {
    let mut out_val: WORD32 = 0;
    b = ((b << 24 as core::ffi::c_int) as UWORD32 >> 24 as core::ffi::c_int) as WORD;
    if b > 31 as core::ffi::c_int {
        out_val = 0 as core::ffi::c_int as WORD32;
    } else {
        out_val = a << b;
    }
    return out_val;
}
#[inline]
unsafe extern "C" fn ixheaac_shr32(mut a: WORD32, mut b: WORD) -> WORD32 {
    let mut out_val: WORD32 = 0;
    b = ((b << 24 as core::ffi::c_int) as UWORD32 >> 24 as core::ffi::c_int) as WORD;
    if b >= 31 as core::ffi::c_int {
        if a < 0 as core::ffi::c_int {
            out_val = -(1 as core::ffi::c_int) as WORD32;
        } else {
            out_val = 0 as core::ffi::c_int as WORD32;
        }
    } else {
        out_val = a >> b;
    }
    return out_val;
}
#[inline]
unsafe extern "C" fn ixheaac_shl32_sat(mut a: WORD32, mut b: WORD) -> WORD32 {
    let mut out_val: WORD32 = 0;
    if a > MAX_32 >> b {
        out_val = MAX_32;
    } else if a < MIN_32 >> b {
        out_val = MIN_32;
    } else {
        out_val = a << b;
    }
    return out_val;
}
#[inline]
unsafe extern "C" fn ixheaac_add32_sat(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut sum: WORD64 = 0;
    sum = a as WORD64 + b as WORD64;
    if sum >= MAX_32 as WORD64 {
        return MAX_32;
    }
    if sum <= MIN_32 as WORD64 {
        return MIN_32;
    }
    return sum as WORD32;
}
#[inline]
unsafe extern "C" fn ixheaac_sub32_sat(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut diff: WORD64 = 0;
    diff = a as WORD64 - b as WORD64;
    if diff >= MAX_32 as WORD64 {
        return MAX_32;
    }
    if diff <= MIN_32 as WORD64 {
        return MIN_32;
    }
    return diff as WORD32;
}
#[inline]
unsafe extern "C" fn ixheaac_norm32(mut a: WORD32) -> WORD {
    let mut norm_val: WORD = 0;
    if a == 0 as core::ffi::c_int {
        norm_val = 31 as core::ffi::c_int as WORD;
    } else if a == 0xffffffff as core::ffi::c_long as WORD32 {
        norm_val = 31 as core::ffi::c_int as WORD;
    } else {
        if a < 0 as core::ffi::c_int {
            a = !a;
        }
        norm_val = 0 as core::ffi::c_int as WORD;
        while a < 0x40000000 as core::ffi::c_long as WORD32 {
            a <<= 1 as core::ffi::c_int;
            norm_val += 1;
        }
    }
    return norm_val;
}
#[inline]
unsafe extern "C" fn ixheaac_negate32(mut a: WORD32) -> WORD32 {
    let mut neg_val: WORD32 = 0;
    neg_val = -a;
    return neg_val;
}
pub const INTENSITY_HCB2: core::ffi::c_int = 14 as core::ffi::c_int;
pub const INTENSITY_HCB: core::ffi::c_int = 15 as core::ffi::c_int;
pub const JOINT_STEREO_MAX_BANDS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const LEFT: core::ffi::c_int = 0 as core::ffi::c_int;
pub const RIGHT: core::ffi::c_int = 1 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_ms_stereo_process(
    mut ptr_aac_dec_channel_info: *mut *mut ia_aac_dec_channel_info_struct,
    mut ptr_aac_tables: *mut ia_aac_dec_tables_struct,
) -> VOID {
    let mut win_grp: WORD32 = 0;
    let mut grp_len: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut l_spec: *mut WORD32 = (**ptr_aac_dec_channel_info.offset(LEFT as isize))
        .ptr_spec_coeff;
    let mut r_spec: *mut WORD32 = (**ptr_aac_dec_channel_info.offset(RIGHT as isize))
        .ptr_spec_coeff;
    let mut maximum_bins_short: WORD16 = ((**ptr_aac_dec_channel_info
        .offset(LEFT as isize))
        .str_ics_info
        .frame_length as core::ffi::c_int >> 3 as core::ffi::c_int) as WORD16;
    let mut ptr_group_len: *mut WORD8 = ((**ptr_aac_dec_channel_info
        .offset(LEFT as isize))
        .str_ics_info
        .window_group_length)
        .as_mut_ptr();
    let mut ptr_sfb_width: *const WORD8 = (*ptr_aac_tables)
        .str_aac_sfb_info[(**ptr_aac_dec_channel_info.offset(RIGHT as isize))
            .str_ics_info
            .window_sequence as usize]
        .sfb_width;
    let mut ptr_ms_used: *mut UWORD8 = &mut *(*((*(**ptr_aac_dec_channel_info
        .offset(LEFT as isize))
        .pstr_stereo_info)
        .ms_used)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize))
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut UWORD8;
    win_grp = 0 as core::ffi::c_int as WORD32;
    while win_grp
        < (**ptr_aac_dec_channel_info.offset(LEFT as isize))
            .str_ics_info
            .num_window_groups as core::ffi::c_int
    {
        grp_len = 0 as core::ffi::c_int as WORD32;
        while grp_len < *ptr_group_len.offset(win_grp as isize) as core::ffi::c_int {
            let mut sfb: WORD32 = 0;
            let mut ixheaacd_drc_offset: WORD32 = 0 as WORD32;
            sfb = 0 as core::ffi::c_int as WORD32;
            while sfb
                < (**ptr_aac_dec_channel_info.offset(LEFT as isize)).str_ics_info.max_sfb
                    as core::ffi::c_int
            {
                ixheaacd_drc_offset
                    += *ptr_sfb_width.offset(sfb as isize) as core::ffi::c_int;
                let fresh0 = ptr_ms_used;
                ptr_ms_used = ptr_ms_used.offset(1);
                if *fresh0 != 0 {
                    k = 0 as core::ffi::c_int as WORD32;
                    while k < *ptr_sfb_width.offset(sfb as isize) as core::ffi::c_int {
                        let mut left_coef: WORD32 = *l_spec;
                        let mut right_coef: WORD32 = *r_spec;
                        let mut left_coef2: WORD32 = *l_spec
                            .offset(1 as core::ffi::c_int as isize);
                        let mut right_coef2: WORD32 = *r_spec
                            .offset(1 as core::ffi::c_int as isize);
                        let fresh1 = l_spec;
                        l_spec = l_spec.offset(1);
                        *fresh1 = ixheaac_add32_sat(left_coef, right_coef);
                        let fresh2 = r_spec;
                        r_spec = r_spec.offset(1);
                        *fresh2 = ixheaac_sub32_sat(left_coef, right_coef);
                        let fresh3 = l_spec;
                        l_spec = l_spec.offset(1);
                        *fresh3 = ixheaac_add32_sat(left_coef2, right_coef2);
                        let fresh4 = r_spec;
                        r_spec = r_spec.offset(1);
                        *fresh4 = ixheaac_sub32_sat(left_coef2, right_coef2);
                        k = (k as core::ffi::c_int + 2 as core::ffi::c_int) as WORD32;
                    }
                } else {
                    l_spec = l_spec
                        .offset(
                            *ptr_sfb_width.offset(sfb as isize) as core::ffi::c_int
                                as isize,
                        );
                    r_spec = r_spec
                        .offset(
                            *ptr_sfb_width.offset(sfb as isize) as core::ffi::c_int
                                as isize,
                        );
                }
                sfb += 1;
            }
            ptr_ms_used = ptr_ms_used
                .offset(
                    -((**ptr_aac_dec_channel_info.offset(LEFT as isize))
                        .str_ics_info
                        .max_sfb as core::ffi::c_int as isize),
                );
            if maximum_bins_short as core::ffi::c_int == 120 as core::ffi::c_int {
                l_spec = l_spec
                    .offset(maximum_bins_short as core::ffi::c_int as isize)
                    .offset(-(ixheaacd_drc_offset as isize));
                r_spec = r_spec
                    .offset(maximum_bins_short as core::ffi::c_int as isize)
                    .offset(-(ixheaacd_drc_offset as isize));
            } else {
                l_spec = l_spec
                    .offset(128 as core::ffi::c_int as isize)
                    .offset(-(ixheaacd_drc_offset as isize));
                r_spec = r_spec
                    .offset(128 as core::ffi::c_int as isize)
                    .offset(-(ixheaacd_drc_offset as isize));
            }
            grp_len += 1;
        }
        ptr_ms_used = ptr_ms_used.offset(JOINT_STEREO_MAX_BANDS as isize);
        win_grp += 1;
    }
}
#[inline]
unsafe extern "C" fn ixheaacd_mult32x16in32l(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 16 as core::ffi::c_int) as WORD32;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_intensity_stereo_process(
    mut ptr_aac_dec_channel_info: *mut *mut ia_aac_dec_channel_info_struct,
    mut ptr_aac_tables: *mut ia_aac_dec_tables_struct,
    mut object_type: WORD32,
    mut aac_sf_data_resil_flag: WORD32,
    mut framelength: WORD16,
) -> VOID {
    let mut ptr_ms_used: *mut UWORD8 = &mut *(*((*(**ptr_aac_dec_channel_info
        .offset(LEFT as isize))
        .pstr_stereo_info)
        .ms_used)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize))
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut UWORD8;
    let mut ptr_code_book: *mut WORD8 = &mut *((**ptr_aac_dec_channel_info
        .offset(RIGHT as isize))
        .ptr_code_book)
        .offset(0 as core::ffi::c_int as isize) as *mut WORD8;
    let mut ptr_scale_factor: *mut WORD16 = &mut *((**ptr_aac_dec_channel_info
        .offset(RIGHT as isize))
        .ptr_scale_factor)
        .offset(0 as core::ffi::c_int as isize) as *mut WORD16;
    let mut r_spec: *mut WORD32 = &mut *((**ptr_aac_dec_channel_info
        .offset(RIGHT as isize))
        .ptr_spec_coeff)
        .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
    let mut l_spec: *mut WORD32 = &mut *((**ptr_aac_dec_channel_info
        .offset(LEFT as isize))
        .ptr_spec_coeff)
        .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
    let mut maximum_bins_short: WORD16 = ((**ptr_aac_dec_channel_info
        .offset(LEFT as isize))
        .str_ics_info
        .frame_length as core::ffi::c_int >> 3 as core::ffi::c_int) as WORD16;
    let mut ptr_group_len: *mut WORD8 = ((**ptr_aac_dec_channel_info
        .offset(RIGHT as isize))
        .str_ics_info
        .window_group_length)
        .as_mut_ptr();
    let mut ptr_sfb_width: *const WORD8 = (*ptr_aac_tables)
        .str_aac_sfb_info[(**ptr_aac_dec_channel_info.offset(RIGHT as isize))
            .str_ics_info
            .window_sequence as usize]
        .sfb_width;
    let mut ptr_scale_table: *mut WORD32 = 0 as *mut WORD32;
    if 960 as core::ffi::c_int == framelength as core::ffi::c_int {
        ptr_scale_table = ((*(*ptr_aac_tables).pstr_block_tables).scale_table_960)
            .as_mut_ptr();
    } else {
        ptr_scale_table = ((*(*ptr_aac_tables).pstr_block_tables).scale_table)
            .as_mut_ptr();
    }
    let mut win_grp: WORD32 = 0;
    let mut grp_len: WORD32 = 0;
    let mut k: WORD32 = 0;
    win_grp = 0 as core::ffi::c_int as WORD32;
    while win_grp
        < (**ptr_aac_dec_channel_info.offset(RIGHT as isize))
            .str_ics_info
            .num_window_groups as core::ffi::c_int
    {
        grp_len = 0 as core::ffi::c_int as WORD32;
        while grp_len < *ptr_group_len.offset(win_grp as isize) as core::ffi::c_int {
            let mut sfb: WORD32 = 0;
            let mut ixheaacd_drc_offset: WORD32 = 0 as WORD32;
            sfb = 0 as core::ffi::c_int as WORD32;
            while sfb
                < (**ptr_aac_dec_channel_info.offset(RIGHT as isize))
                    .str_ics_info
                    .max_sfb as core::ffi::c_int
            {
                let mut code_book: WORD8 = *ptr_code_book.offset(sfb as isize);
                ixheaacd_drc_offset
                    += *ptr_sfb_width.offset(sfb as isize) as core::ffi::c_int;
                if code_book as core::ffi::c_int >= INTENSITY_HCB2
                    && (object_type != AOT_ER_AAC_ELD as core::ffi::c_int
                        && object_type != AOT_ER_AAC_LD as core::ffi::c_int)
                    || (code_book as core::ffi::c_int == INTENSITY_HCB2
                        || code_book as core::ffi::c_int == INTENSITY_HCB)
                        && (object_type == AOT_ER_AAC_ELD as core::ffi::c_int
                            || object_type == AOT_ER_AAC_LD as core::ffi::c_int)
                {
                    let mut sfb_factor: WORD32 = 0;
                    let mut scale: WORD32 = 0;
                    let mut scf_exp: WORD32 = 0;
                    sfb_factor = *ptr_scale_factor.offset(sfb as isize) as WORD32;
                    if aac_sf_data_resil_flag != 0 {
                        sfb_factor = -sfb_factor;
                    }
                    scf_exp = sfb_factor >> 2 as core::ffi::c_int;
                    scale = *ptr_scale_table
                        .offset(
                            (sfb_factor as core::ffi::c_int & 3 as core::ffi::c_int)
                                as isize,
                        );
                    if *ptr_ms_used.offset(sfb as isize) as core::ffi::c_int
                        ^ code_book as core::ffi::c_int & 0x1 as core::ffi::c_int == 0
                    {
                        scale = ixheaac_negate32(scale);
                    }
                    scf_exp = -(scf_exp as core::ffi::c_int + 2 as core::ffi::c_int)
                        as WORD32;
                    k = 0 as core::ffi::c_int as WORD32;
                    while k < *ptr_sfb_width.offset(sfb as isize) as core::ffi::c_int {
                        let mut temp: WORD32 = 0;
                        let mut shift_val: WORD32 = 0;
                        let fresh5 = l_spec;
                        l_spec = l_spec.offset(1);
                        temp = *fresh5;
                        shift_val = ixheaac_norm32(temp) as WORD32;
                        temp = ixheaac_shl32(temp, shift_val as WORD);
                        temp = ixheaacd_mult32x16in32l(temp, scale);
                        shift_val = shift_val + scf_exp;
                        if shift_val < 0 as core::ffi::c_int {
                            if shift_val < -(31 as core::ffi::c_int) {
                                shift_val = -(31 as core::ffi::c_int) as WORD32;
                            }
                            temp = ixheaac_shl32_sat(temp, -(shift_val as WORD));
                        } else {
                            if shift_val > 31 as core::ffi::c_int {
                                shift_val = 31 as core::ffi::c_int as WORD32;
                            }
                            temp = ixheaac_shr32(temp, shift_val as WORD);
                        }
                        let fresh6 = r_spec;
                        r_spec = r_spec.offset(1);
                        *fresh6 = temp;
                        k += 1;
                    }
                } else {
                    l_spec = l_spec
                        .offset(
                            *ptr_sfb_width.offset(sfb as isize) as core::ffi::c_int
                                as isize,
                        );
                    r_spec = r_spec
                        .offset(
                            *ptr_sfb_width.offset(sfb as isize) as core::ffi::c_int
                                as isize,
                        );
                }
                sfb += 1;
            }
            if maximum_bins_short as core::ffi::c_int == 120 as core::ffi::c_int {
                l_spec = l_spec
                    .offset(
                        (maximum_bins_short as WORD32 - ixheaacd_drc_offset) as isize,
                    );
                r_spec = r_spec
                    .offset(
                        (maximum_bins_short as WORD32 - ixheaacd_drc_offset) as isize,
                    );
            } else {
                l_spec = l_spec.offset((128 as WORD32 - ixheaacd_drc_offset) as isize);
                r_spec = r_spec.offset((128 as WORD32 - ixheaacd_drc_offset) as isize);
            }
            grp_len += 1;
        }
        ptr_ms_used = ptr_ms_used.offset(64 as core::ffi::c_int as isize);
        ptr_code_book = ptr_code_book.offset(16 as core::ffi::c_int as isize);
        ptr_scale_factor = ptr_scale_factor.offset(16 as core::ffi::c_int as isize);
        win_grp += 1;
    }
}
