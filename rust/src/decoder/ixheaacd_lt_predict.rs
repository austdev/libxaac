extern "C" {
    fn ixheaacd_aac_tns_process(
        ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
        num_ch: WORD32,
        ptr_aac_tables: *mut ia_aac_dec_tables_struct,
        object_type: WORD,
        ar_flag: WORD32,
        predicted_spectrum: *mut WORD32,
    ) -> VOID;
    fn ixheaacd_calc_max_spectral_line_dec(ptr_tmp: *mut WORD32, size: WORD32) -> WORD32;
    fn ixheaacd_inverse_transform(
        spec_data: *mut WORD32,
        scratch: *mut WORD32,
        ptr_imdct_tables: *mut ia_aac_dec_imdct_tables_struct,
        expo: WORD32,
        npoints: WORD32,
    ) -> WORD32;
    fn ixheaacd_post_twiddle_dec(
        out_ptr: *mut WORD32,
        spec_data: *mut WORD32,
        ptr_imdct_tables: *mut ia_aac_dec_imdct_tables_struct,
        npoints: WORD32,
    ) -> VOID;
    fn ixheaacd_inverse_transform_512(
        data: *mut WORD32,
        temp: *mut WORD32,
        imdct_scale: *mut WORD32,
        cos_sin_ptr: *mut WORD32,
        imdct_tables_ptr: *mut ia_aac_dec_imdct_tables_struct,
        object_type: WORD32,
    ) -> VOID;
    fn ixheaacd_mdct_480_ld(
        inp: *mut WORD32,
        scratch: *mut WORD32,
        mdct_scale: *mut WORD32,
        mdct_flag: WORD32,
        imdct_tables_ptr: *mut ia_aac_dec_imdct_tables_struct,
        object_type: WORD32,
    ) -> VOID;
}
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
pub const AOT_ER_AAC_LD: AUDIO_OBJECT_TYPE = 23;
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
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const MAX_32: WORD32 = 0x7fffffff as core::ffi::c_long as WORD32;
pub const MIN_32: WORD32 = 0x80000000 as core::ffi::c_long as WORD32;
pub const MAX_16: WORD16 = 0x7fff as core::ffi::c_int as WORD16;
pub const ONLY_LONG_SEQUENCE: core::ffi::c_int = 0 as core::ffi::c_int;
pub const LONG_START_SEQUENCE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const EIGHT_SHORT_SEQUENCE: core::ffi::c_int = 2 as core::ffi::c_int;
pub const LONG_STOP_SEQUENCE: core::ffi::c_int = 3 as core::ffi::c_int;
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
unsafe extern "C" fn ixheaac_shl32_dir(mut a: WORD32, mut b: WORD) -> WORD32 {
    let mut out_val: WORD32 = 0;
    if b < 0 as core::ffi::c_int {
        out_val = ixheaac_shr32(a, -b);
    } else {
        out_val = ixheaac_shl32(a, b);
    }
    return out_val;
}
#[inline]
unsafe extern "C" fn ixheaac_mult16x16in32(mut a: WORD16, mut b: WORD16) -> WORD32 {
    let mut product: WORD32 = 0;
    product = a as WORD32 * b as WORD32;
    return product;
}
#[inline]
unsafe extern "C" fn ixheaac_mult16x16in32_shl(mut a: WORD16, mut b: WORD16) -> WORD32 {
    let mut product: WORD32 = 0;
    product = ixheaac_shl32(ixheaac_mult16x16in32(a, b), 1 as WORD);
    return product;
}
#[inline]
unsafe extern "C" fn ixheaac_add32(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut sum: WORD32 = 0;
    sum = a + b;
    return sum;
}
#[inline]
unsafe extern "C" fn ixheaac_sub32(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut diff: WORD32 = 0;
    diff = a - b;
    return diff;
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
unsafe extern "C" fn ixheaac_sat16(mut op1: WORD32) -> WORD16 {
    let mut var_out: WORD16 = 0;
    if op1 as core::ffi::c_long > 0x7fff as core::ffi::c_long {
        var_out = MAX_16;
    } else if op1 < 0xffff8000 as core::ffi::c_long as WORD32 {
        var_out = -(32768 as core::ffi::c_int) as WORD16;
    } else {
        var_out = op1 as WORD16;
    }
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_shl16(mut op1: WORD16, mut shift: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = ((op1 as core::ffi::c_int) << shift as core::ffi::c_int) as WORD16;
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_round16(mut op1: WORD32) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = (ixheaac_add32_sat(op1, 0x8000 as WORD32) >> 16 as core::ffi::c_int)
        as WORD16;
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_mult32x16in32_shl(mut a: WORD32, mut b: WORD16) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 16 as core::ffi::c_int) as WORD32;
    return result << 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn ixheaac_mult32x16in32_shl_sat(
    mut a: WORD32,
    mut b: WORD16,
) -> WORD32 {
    let mut result: WORD32 = 0;
    if a == 0x80000000 as core::ffi::c_uint as WORD32
        && b as core::ffi::c_int
            == 0x8000 as core::ffi::c_int as WORD16 as core::ffi::c_int
    {
        result = 0x7fffffff as core::ffi::c_int;
    } else {
        result = ixheaac_mult32x16in32_shl(a, b);
    }
    return result;
}
#[inline]
unsafe extern "C" fn ixheaac_mult32_shl(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 32 as core::ffi::c_int) as WORD32;
    return result << 1 as core::ffi::c_int;
}
static mut ixheaacd_codebook_Q30: [WORD32; 8] = [
    612922971 as core::ffi::c_int,
    747985734 as core::ffi::c_int,
    872956397 as core::ffi::c_int,
    978505219 as core::ffi::c_int,
    1057528322 as core::ffi::c_int,
    1146642451 as core::ffi::c_int,
    1282693056 as core::ffi::c_int,
    1470524861 as core::ffi::c_int,
];
pub const SHIFT_VAL: core::ffi::c_int = 8 as core::ffi::c_int;
pub const SHIFT_VAL1: core::ffi::c_int = 15 as core::ffi::c_int - SHIFT_VAL;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_lt_prediction(
    mut ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
    mut ltp: *mut ltp_info,
    mut spec: *mut WORD32,
    mut aac_tables_ptr: *mut ia_aac_dec_tables_struct,
    mut win_shape_prev: UWORD16,
    mut sr_index: UWORD32,
    mut object_type: UWORD32,
    mut frame_len: UWORD32,
    mut in_data: *mut WORD32,
    mut out_data: *mut WORD32,
) -> VOID {
    let mut ptr_ics_info: *mut ia_ics_info_struct = &mut (*ptr_aac_dec_channel_info)
        .str_ics_info;
    let mut lt_pred_stat: *mut WORD16 = (*ptr_aac_dec_channel_info).ltp_buf;
    let mut win_shape: UWORD16 = (*ptr_aac_dec_channel_info).str_ics_info.window_shape
        as UWORD16;
    let mut sfb: WORD16 = 0;
    let mut bin: WORD16 = 0;
    let mut i: WORD16 = 0;
    let mut num_samples: WORD16 = 0;
    let mut swb_offset: *const WORD8 = (*aac_tables_ptr)
        .scale_factor_bands_long[sr_index as usize];
    let mut ptr_spec: *mut WORD32 = &mut *spec.offset(0 as core::ffi::c_int as isize)
        as *mut WORD32;
    let mut ptr_x_est: *mut WORD32 = &mut *out_data
        .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
    if 512 as core::ffi::c_int == (*ptr_ics_info).frame_length as core::ffi::c_int {
        swb_offset = (*aac_tables_ptr).scale_fac_bands_512[sr_index as usize];
    } else if 480 as core::ffi::c_int == (*ptr_ics_info).frame_length as core::ffi::c_int
    {
        swb_offset = (*aac_tables_ptr).scale_fac_bands_480[sr_index as usize];
    }
    if (*ptr_ics_info).window_sequence as core::ffi::c_int != EIGHT_SHORT_SEQUENCE {
        if (*ltp).data_present != 0 {
            num_samples = (frame_len << 1 as core::ffi::c_int) as WORD16;
            i = 0 as WORD16;
            while (i as core::ffi::c_int) < num_samples as core::ffi::c_int {
                *in_data.offset(i as isize) = ixheaac_shr32(
                    ixheaac_mult32x16in32_shl_sat(
                        ixheaacd_codebook_Q30[(*ltp).coef as usize],
                        *lt_pred_stat
                            .offset(
                                (num_samples as core::ffi::c_int + i as core::ffi::c_int
                                    - (*ltp).lag as core::ffi::c_int) as isize,
                            ),
                    ),
                    SHIFT_VAL,
                );
                i += 1;
            }
            ixheaacd_filter_bank_ltp(
                aac_tables_ptr,
                (*ptr_ics_info).window_sequence,
                win_shape as WORD16,
                win_shape_prev as WORD16,
                in_data,
                out_data,
                object_type,
                frame_len,
            );
            if (*ptr_aac_dec_channel_info).str_tns_info.tns_data_present
                == 1 as core::ffi::c_int
            {
                ixheaacd_aac_tns_process(
                    ptr_aac_dec_channel_info,
                    1 as WORD32,
                    aac_tables_ptr,
                    object_type as WORD,
                    0 as WORD32,
                    out_data,
                );
            }
            sfb = 0 as WORD16;
            while (sfb as core::ffi::c_int) < (*ltp).last_band as core::ffi::c_int {
                let mut sfb_width: WORD8 = *swb_offset.offset(sfb as isize);
                if (*ltp).long_used[sfb as usize] != 0 {
                    bin = (sfb_width as core::ffi::c_int - 1 as core::ffi::c_int)
                        as WORD16;
                    while bin as core::ffi::c_int >= 0 as core::ffi::c_int {
                        let mut temp: WORD32 = *ptr_spec;
                        let fresh8 = ptr_x_est;
                        ptr_x_est = ptr_x_est.offset(1);
                        temp = ixheaac_add32_sat(
                            temp,
                            ixheaac_shr32(*fresh8, SHIFT_VAL1),
                        );
                        let fresh9 = ptr_spec;
                        ptr_spec = ptr_spec.offset(1);
                        *fresh9 = temp;
                        bin -= 1;
                    }
                } else {
                    ptr_spec = ptr_spec.offset(sfb_width as core::ffi::c_int as isize);
                    ptr_x_est = ptr_x_est.offset(sfb_width as core::ffi::c_int as isize);
                }
                sfb += 1;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_filter_bank_ltp(
    mut aac_tables_ptr: *mut ia_aac_dec_tables_struct,
    mut window_sequence: WORD16,
    mut window_shape: WORD16,
    mut window_shape_prev: WORD16,
    mut in_data: *mut WORD32,
    mut out_mdct: *mut WORD32,
    mut object_type: UWORD32,
    mut frame_len: UWORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut window_long: *const WORD16 = 0 as *const WORD16;
    let mut window_long_prev: *const WORD16 = 0 as *const WORD16;
    let mut window_short: *const WORD16 = 0 as *const WORD16;
    let mut window_short_prev: *const WORD16 = 0 as *const WORD16;
    let mut nlong: UWORD16 = frame_len as UWORD16;
    let mut nlong2: UWORD16 = (frame_len << 1 as core::ffi::c_int) as UWORD16;
    let mut nshort: UWORD16 = frame_len.wrapping_div(8 as UWORD32) as UWORD16;
    let mut nflat_ls: UWORD16 = ((nlong as core::ffi::c_int - nshort as core::ffi::c_int)
        / 2 as core::ffi::c_int) as UWORD16;
    let mut imdct_scale: WORD32 = 0 as WORD32;
    let mut expo: WORD32 = 0 as WORD32;
    if object_type == AOT_ER_AAC_LD as core::ffi::c_int as UWORD32 {
        if window_shape == 0 {
            if 512 as UWORD32 == frame_len {
                window_long = ((*(*aac_tables_ptr).pstr_imdct_tables).window_sine_512)
                    .as_mut_ptr() as *mut WORD16;
            } else {
                window_long = ((*(*aac_tables_ptr).pstr_imdct_tables).window_sine_480)
                    .as_mut_ptr() as *mut WORD16;
            }
        } else if 512 as UWORD32 == frame_len {
            window_long = ((*(*aac_tables_ptr).pstr_imdct_tables).low_overlap_win)
                .as_mut_ptr() as *mut WORD16;
        } else {
            window_long = ((*(*aac_tables_ptr).pstr_imdct_tables).low_overlap_win_480)
                .as_mut_ptr() as *mut WORD16;
        }
        if window_shape_prev == 0 {
            if 512 as UWORD32 == frame_len {
                window_long_prev = ((*(*aac_tables_ptr).pstr_imdct_tables)
                    .window_sine_512)
                    .as_mut_ptr() as *mut WORD16;
            } else {
                window_long_prev = ((*(*aac_tables_ptr).pstr_imdct_tables)
                    .window_sine_480)
                    .as_mut_ptr() as *mut WORD16;
            }
        } else if 512 as UWORD32 == frame_len {
            window_long_prev = ((*(*aac_tables_ptr).pstr_imdct_tables).low_overlap_win)
                .as_mut_ptr() as *mut WORD16;
        } else {
            window_long_prev = ((*(*aac_tables_ptr).pstr_imdct_tables)
                .low_overlap_win_480)
                .as_mut_ptr() as *mut WORD16;
        }
        if window_shape == 0 {
            window_short = ((*(*aac_tables_ptr).pstr_imdct_tables)
                .only_short_window_sine)
                .as_mut_ptr();
        } else {
            window_short = ((*(*aac_tables_ptr).pstr_imdct_tables).only_short_window_kbd)
                .as_mut_ptr();
        }
        if window_shape_prev == 0 {
            window_short_prev = ((*(*aac_tables_ptr).pstr_imdct_tables)
                .only_short_window_sine)
                .as_mut_ptr();
        } else {
            window_short_prev = ((*(*aac_tables_ptr).pstr_imdct_tables)
                .only_short_window_kbd)
                .as_mut_ptr();
        }
    } else {
        if window_shape == 0 {
            window_long = ((*(*aac_tables_ptr).pstr_imdct_tables).only_long_window_sine)
                .as_mut_ptr();
        } else {
            window_long = ((*(*aac_tables_ptr).pstr_imdct_tables).only_long_window_kbd)
                .as_mut_ptr();
        }
        if window_shape_prev == 0 {
            window_long_prev = ((*(*aac_tables_ptr).pstr_imdct_tables)
                .only_long_window_sine)
                .as_mut_ptr();
        } else {
            window_long_prev = ((*(*aac_tables_ptr).pstr_imdct_tables)
                .only_long_window_kbd)
                .as_mut_ptr();
        }
        if window_shape == 0 {
            window_short = ((*(*aac_tables_ptr).pstr_imdct_tables)
                .only_short_window_sine)
                .as_mut_ptr();
        } else {
            window_short = ((*(*aac_tables_ptr).pstr_imdct_tables).only_short_window_kbd)
                .as_mut_ptr();
        }
        if window_shape_prev == 0 {
            window_short_prev = ((*(*aac_tables_ptr).pstr_imdct_tables)
                .only_short_window_sine)
                .as_mut_ptr();
        } else {
            window_short_prev = ((*(*aac_tables_ptr).pstr_imdct_tables)
                .only_short_window_kbd)
                .as_mut_ptr();
        }
    }
    match window_sequence as core::ffi::c_int {
        ONLY_LONG_SEQUENCE => {
            if 512 as core::ffi::c_int != nlong as core::ffi::c_int
                && 480 as core::ffi::c_int != nlong as core::ffi::c_int
            {
                i = 0 as core::ffi::c_int as WORD32;
                while i < nlong as core::ffi::c_int >> 1 as core::ffi::c_int {
                    *in_data.offset(i as isize) = ixheaac_mult32x16in32_shl(
                        *in_data.offset(i as isize),
                        *window_long_prev.offset((2 as WORD32 * i) as isize),
                    );
                    *in_data
                        .offset(
                            (i as core::ffi::c_int + nlong as core::ffi::c_int) as isize,
                        ) = ixheaac_mult32x16in32_shl(
                        *in_data
                            .offset(
                                (i as core::ffi::c_int + nlong as core::ffi::c_int) as isize,
                            ),
                        *window_long
                            .offset(
                                (2 as core::ffi::c_int * i as core::ffi::c_int
                                    + 1 as core::ffi::c_int) as isize,
                            ),
                    );
                    i += 1;
                }
                i = 0 as core::ffi::c_int as WORD32;
                while i < nlong as core::ffi::c_int >> 1 as core::ffi::c_int {
                    *in_data
                        .offset(
                            (i as core::ffi::c_int
                                + (nlong as core::ffi::c_int >> 1 as core::ffi::c_int))
                                as isize,
                        ) = ixheaac_mult32x16in32_shl(
                        *in_data
                            .offset(
                                (i as core::ffi::c_int
                                    + (nlong as core::ffi::c_int >> 1 as core::ffi::c_int))
                                    as isize,
                            ),
                        *window_long_prev
                            .offset(
                                (nlong as WORD32 - 1 as WORD32 - 2 as WORD32 * i) as isize,
                            ),
                    );
                    *in_data
                        .offset(
                            (i as core::ffi::c_int + nlong as core::ffi::c_int
                                + (nlong as core::ffi::c_int >> 1 as core::ffi::c_int))
                                as isize,
                        ) = ixheaac_mult32x16in32_shl(
                        *in_data
                            .offset(
                                (i as core::ffi::c_int + nlong as core::ffi::c_int
                                    + (nlong as core::ffi::c_int >> 1 as core::ffi::c_int))
                                    as isize,
                            ),
                        *window_long
                            .offset(
                                (nlong as core::ffi::c_int - 1 as core::ffi::c_int
                                    - 2 as core::ffi::c_int * i as core::ffi::c_int
                                    - 1 as core::ffi::c_int) as isize,
                            ),
                    );
                    i += 1;
                }
            } else {
                let mut win1: *mut WORD32 = 0 as *mut WORD32;
                let mut win2: *mut WORD32 = 0 as *mut WORD32;
                let mut ptr_in1: *mut WORD32 = 0 as *mut WORD32;
                let mut ptr_in2: *mut WORD32 = 0 as *mut WORD32;
                win1 = window_long_prev as *mut WORD32;
                win2 = window_long as *mut WORD32;
                ptr_in1 = &mut *in_data.offset(0 as core::ffi::c_int as isize)
                    as *mut WORD32;
                ptr_in2 = &mut *in_data.offset(nlong as isize) as *mut WORD32;
                i = (nlong as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
                while i >= 0 as core::ffi::c_int {
                    let fresh5 = win1;
                    win1 = win1.offset(1);
                    let mut temp1: WORD32 = ixheaac_mult32_shl(*ptr_in1, *fresh5);
                    let mut temp2: WORD32 = ixheaac_mult32_shl(
                        *ptr_in2,
                        *win2.offset(i as isize),
                    );
                    let fresh6 = ptr_in1;
                    ptr_in1 = ptr_in1.offset(1);
                    *fresh6 = temp1;
                    let fresh7 = ptr_in2;
                    ptr_in2 = ptr_in2.offset(1);
                    *fresh7 = temp2;
                    i -= 1;
                }
            }
            i = 0 as core::ffi::c_int as WORD32;
            while i < nlong as core::ffi::c_int / 2 as core::ffi::c_int {
                *out_mdct.offset((nlong as WORD32 / 2 as WORD32 + i) as isize) = ixheaac_sub32(
                    *in_data.offset(i as isize),
                    *in_data.offset((nlong as WORD32 - 1 as WORD32 - i) as isize),
                );
                *out_mdct.offset(i as isize) = -ixheaac_add32(
                    *in_data
                        .offset(
                            (nlong as core::ffi::c_int + i as core::ffi::c_int
                                + nlong as core::ffi::c_int / 2 as core::ffi::c_int)
                                as isize,
                        ),
                    *in_data
                        .offset(
                            (nlong2 as WORD32 - nlong as WORD32 / 2 as WORD32
                                - 1 as WORD32 - i) as isize,
                        ),
                );
                i += 1;
            }
            if 512 as core::ffi::c_int == nlong as core::ffi::c_int
                || 480 as core::ffi::c_int == nlong as core::ffi::c_int
            {
                if 512 as core::ffi::c_int == nlong as core::ffi::c_int {
                    ixheaacd_inverse_transform_512(
                        out_mdct as *mut WORD32,
                        in_data as *mut WORD32,
                        &mut imdct_scale,
                        ((*(*aac_tables_ptr).pstr_imdct_tables).cosine_array_1024)
                            .as_mut_ptr(),
                        (*aac_tables_ptr).pstr_imdct_tables,
                        object_type as WORD32,
                    );
                } else {
                    ixheaacd_mdct_480_ld(
                        out_mdct,
                        in_data,
                        &mut imdct_scale,
                        1 as WORD32,
                        (*aac_tables_ptr).pstr_imdct_tables,
                        object_type as WORD32,
                    );
                }
                imdct_scale += 1 as core::ffi::c_int;
                if imdct_scale > 0 as core::ffi::c_int {
                    let mut ptr_out_mdct: *mut WORD32 = &mut *out_mdct
                        .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
                    i = (nlong as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
                    while i >= 0 as core::ffi::c_int {
                        *ptr_out_mdct = ixheaac_shl32(
                            *ptr_out_mdct,
                            imdct_scale as WORD,
                        );
                        ptr_out_mdct = ptr_out_mdct.offset(1);
                        *ptr_out_mdct = ixheaac_shl32(
                            *ptr_out_mdct,
                            imdct_scale as WORD,
                        );
                        ptr_out_mdct = ptr_out_mdct.offset(1);
                        *ptr_out_mdct = ixheaac_shl32(
                            *ptr_out_mdct,
                            imdct_scale as WORD,
                        );
                        ptr_out_mdct = ptr_out_mdct.offset(1);
                        *ptr_out_mdct = ixheaac_shl32(
                            *ptr_out_mdct,
                            imdct_scale as WORD,
                        );
                        ptr_out_mdct = ptr_out_mdct.offset(1);
                        i -= 4 as core::ffi::c_int;
                    }
                } else if imdct_scale < 0 as core::ffi::c_int {
                    let mut ptr_out_mdct_0: *mut WORD32 = &mut *out_mdct
                        .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
                    imdct_scale = -imdct_scale;
                    i = (nlong as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
                    while i >= 0 as core::ffi::c_int {
                        *ptr_out_mdct_0 = ixheaac_shr32(
                            *ptr_out_mdct_0,
                            imdct_scale as WORD,
                        );
                        ptr_out_mdct_0 = ptr_out_mdct_0.offset(1);
                        *ptr_out_mdct_0 = ixheaac_shr32(
                            *ptr_out_mdct_0,
                            imdct_scale as WORD,
                        );
                        ptr_out_mdct_0 = ptr_out_mdct_0.offset(1);
                        *ptr_out_mdct_0 = ixheaac_shr32(
                            *ptr_out_mdct_0,
                            imdct_scale as WORD,
                        );
                        ptr_out_mdct_0 = ptr_out_mdct_0.offset(1);
                        *ptr_out_mdct_0 = ixheaac_shr32(
                            *ptr_out_mdct_0,
                            imdct_scale as WORD,
                        );
                        ptr_out_mdct_0 = ptr_out_mdct_0.offset(1);
                        i -= 4 as core::ffi::c_int;
                    }
                }
            } else if 1024 as core::ffi::c_int == nlong as core::ffi::c_int {
                expo = (ixheaacd_calc_max_spectral_line_dec(out_mdct, 1024 as WORD32)
                    as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
                expo = 8 as WORD32 - expo;
                imdct_scale = ixheaacd_inverse_transform(
                    out_mdct as *mut WORD32,
                    in_data as *mut WORD32,
                    (*aac_tables_ptr).pstr_imdct_tables,
                    expo,
                    1024 as WORD32,
                );
                ixheaacd_post_twiddle_dec(
                    in_data as *mut WORD32,
                    out_mdct as *mut WORD32,
                    (*aac_tables_ptr).pstr_imdct_tables,
                    1024 as WORD32,
                );
                imdct_scale += 1 as core::ffi::c_int;
                i = 0 as core::ffi::c_int as WORD32;
                while i < nlong as core::ffi::c_int {
                    *out_mdct.offset(i as isize) = ixheaac_shl32_dir(
                        *in_data.offset(i as isize),
                        imdct_scale as WORD,
                    );
                    i += 1;
                }
            }
        }
        LONG_START_SEQUENCE => {
            i = 0 as core::ffi::c_int as WORD32;
            while i < nlong as core::ffi::c_int >> 1 as core::ffi::c_int {
                *in_data.offset(i as isize) = ixheaac_mult32x16in32_shl(
                    *in_data.offset(i as isize),
                    *window_long_prev.offset((2 as WORD32 * i) as isize),
                );
                i += 1;
            }
            i = 0 as core::ffi::c_int as WORD32;
            while i < nlong as core::ffi::c_int >> 1 as core::ffi::c_int {
                *in_data
                    .offset(
                        (i as core::ffi::c_int
                            + (nlong as core::ffi::c_int >> 1 as core::ffi::c_int))
                            as isize,
                    ) = ixheaac_mult32x16in32_shl(
                    *in_data
                        .offset(
                            (i as core::ffi::c_int
                                + (nlong as core::ffi::c_int >> 1 as core::ffi::c_int))
                                as isize,
                        ),
                    *window_long_prev
                        .offset(
                            (nlong as core::ffi::c_int - 1 as core::ffi::c_int
                                - 2 as core::ffi::c_int * i as core::ffi::c_int
                                - 1 as core::ffi::c_int) as isize,
                        ),
                );
                i += 1;
            }
            i = 0 as core::ffi::c_int as WORD32;
            while i < nshort as core::ffi::c_int >> 1 as core::ffi::c_int {
                *in_data
                    .offset(
                        (i as core::ffi::c_int + nlong as core::ffi::c_int
                            + nflat_ls as core::ffi::c_int
                            + (nshort as core::ffi::c_int >> 1 as core::ffi::c_int))
                            as isize,
                    ) = ixheaac_mult32x16in32_shl(
                    *in_data
                        .offset(
                            (i as core::ffi::c_int + nlong as core::ffi::c_int
                                + nflat_ls as core::ffi::c_int
                                + (nshort as core::ffi::c_int >> 1 as core::ffi::c_int))
                                as isize,
                        ),
                    *window_short
                        .offset(
                            (nshort as core::ffi::c_int - 1 as core::ffi::c_int
                                - 2 as core::ffi::c_int * i as core::ffi::c_int
                                - 1 as core::ffi::c_int) as isize,
                        ),
                );
                i += 1;
            }
            i = 0 as core::ffi::c_int as WORD32;
            while i < nflat_ls as core::ffi::c_int {
                *in_data
                    .offset(
                        (i as core::ffi::c_int + nlong as core::ffi::c_int
                            + nflat_ls as core::ffi::c_int + nshort as core::ffi::c_int)
                            as isize,
                    ) = 0 as core::ffi::c_int as WORD32;
                i += 1;
            }
            i = 0 as core::ffi::c_int as WORD32;
            while i < nlong as core::ffi::c_int / 2 as core::ffi::c_int {
                *out_mdct.offset((nlong as WORD32 / 2 as WORD32 + i) as isize) = ixheaac_sub32(
                    *in_data.offset(i as isize),
                    *in_data.offset((nlong as WORD32 - 1 as WORD32 - i) as isize),
                );
                *out_mdct
                    .offset(
                        (nlong as WORD32 / 2 as WORD32 - 1 as WORD32 - i) as isize,
                    ) = -ixheaac_add32(
                    *in_data.offset((nlong as WORD32 + i) as isize),
                    *in_data.offset((nlong2 as WORD32 - 1 as WORD32 - i) as isize),
                );
                i += 1;
            }
            expo = (ixheaacd_calc_max_spectral_line_dec(out_mdct, 1024 as WORD32)
                as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
            expo = 8 as WORD32 - expo;
            imdct_scale = ixheaacd_inverse_transform(
                out_mdct as *mut WORD32,
                in_data as *mut WORD32,
                (*aac_tables_ptr).pstr_imdct_tables,
                expo,
                1024 as WORD32,
            );
            ixheaacd_post_twiddle_dec(
                in_data as *mut WORD32,
                out_mdct as *mut WORD32,
                (*aac_tables_ptr).pstr_imdct_tables,
                1024 as WORD32,
            );
            imdct_scale += 1 as core::ffi::c_int;
            i = 0 as core::ffi::c_int as WORD32;
            while i < nlong as core::ffi::c_int {
                *out_mdct.offset(i as isize) = ixheaac_shl32_dir(
                    *in_data.offset(i as isize),
                    imdct_scale as WORD,
                );
                i += 1;
            }
        }
        LONG_STOP_SEQUENCE => {
            i = 0 as core::ffi::c_int as WORD32;
            while i < nflat_ls as core::ffi::c_int {
                *in_data.offset(i as isize) = 0 as core::ffi::c_int as WORD32;
                i += 1;
            }
            i = 0 as core::ffi::c_int as WORD32;
            while i < nshort as core::ffi::c_int >> 1 as core::ffi::c_int {
                *in_data
                    .offset(
                        (i as core::ffi::c_int + nflat_ls as core::ffi::c_int) as isize,
                    ) = ixheaac_mult32x16in32_shl(
                    *in_data
                        .offset(
                            (i as core::ffi::c_int + nflat_ls as core::ffi::c_int)
                                as isize,
                        ),
                    *window_short_prev.offset((2 as WORD32 * i) as isize),
                );
                i += 1;
            }
            i = 0 as core::ffi::c_int as WORD32;
            while i < nshort as core::ffi::c_int >> 1 as core::ffi::c_int {
                *in_data
                    .offset(
                        (i as core::ffi::c_int + nflat_ls as core::ffi::c_int
                            + (nshort as core::ffi::c_int >> 1 as core::ffi::c_int))
                            as isize,
                    ) = ixheaac_mult32x16in32_shl(
                    *in_data
                        .offset(
                            (i as core::ffi::c_int + nflat_ls as core::ffi::c_int
                                + (nshort as core::ffi::c_int >> 1 as core::ffi::c_int))
                                as isize,
                        ),
                    *window_short_prev.offset((127 as WORD32 - 2 as WORD32 * i) as isize),
                );
                i += 1;
            }
            i = 0 as core::ffi::c_int as WORD32;
            while i < nlong as core::ffi::c_int >> 1 as core::ffi::c_int {
                *in_data
                    .offset(
                        (i as core::ffi::c_int + nlong as core::ffi::c_int) as isize,
                    ) = ixheaac_mult32x16in32_shl(
                    *in_data
                        .offset(
                            (i as core::ffi::c_int + nlong as core::ffi::c_int) as isize,
                        ),
                    *window_long
                        .offset(
                            (2 as core::ffi::c_int * i as core::ffi::c_int
                                + 1 as core::ffi::c_int) as isize,
                        ),
                );
                i += 1;
            }
            i = 0 as core::ffi::c_int as WORD32;
            while i < nlong as core::ffi::c_int >> 1 as core::ffi::c_int {
                *in_data
                    .offset(
                        (i as core::ffi::c_int + nlong as core::ffi::c_int
                            + (nlong as core::ffi::c_int >> 1 as core::ffi::c_int))
                            as isize,
                    ) = ixheaac_mult32x16in32_shl(
                    *in_data
                        .offset(
                            (i as core::ffi::c_int + nlong as core::ffi::c_int
                                + (nlong as core::ffi::c_int >> 1 as core::ffi::c_int))
                                as isize,
                        ),
                    *window_long
                        .offset(
                            (nlong as core::ffi::c_int - 1 as core::ffi::c_int
                                - 2 as core::ffi::c_int * i as core::ffi::c_int
                                - 1 as core::ffi::c_int) as isize,
                        ),
                );
                i += 1;
            }
            i = 0 as core::ffi::c_int as WORD32;
            while i < nlong as core::ffi::c_int / 2 as core::ffi::c_int {
                *out_mdct.offset((nlong as WORD32 / 2 as WORD32 + i) as isize) = ixheaac_sub32(
                    *in_data.offset(i as isize),
                    *in_data.offset((nlong as WORD32 - 1 as WORD32 - i) as isize),
                );
                *out_mdct
                    .offset(
                        (nlong as WORD32 / 2 as WORD32 - 1 as WORD32 - i) as isize,
                    ) = -ixheaac_add32(
                    *in_data.offset((nlong as WORD32 + i) as isize),
                    *in_data.offset((nlong2 as WORD32 - 1 as WORD32 - i) as isize),
                );
                i += 1;
            }
            expo = (ixheaacd_calc_max_spectral_line_dec(out_mdct, 1024 as WORD32)
                as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
            expo = 8 as WORD32 - expo;
            imdct_scale = ixheaacd_inverse_transform(
                out_mdct as *mut WORD32,
                in_data as *mut WORD32,
                (*aac_tables_ptr).pstr_imdct_tables,
                expo,
                1024 as WORD32,
            );
            ixheaacd_post_twiddle_dec(
                in_data as *mut WORD32,
                out_mdct as *mut WORD32,
                (*aac_tables_ptr).pstr_imdct_tables,
                1024 as WORD32,
            );
            imdct_scale += 1 as core::ffi::c_int;
            i = 0 as core::ffi::c_int as WORD32;
            while i < nlong as core::ffi::c_int {
                *out_mdct.offset(i as isize) = ixheaac_shl32_dir(
                    *in_data.offset(i as isize),
                    imdct_scale as WORD,
                );
                i += 1;
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_lt_update_state(
    mut lt_pred_stat: *mut WORD16,
    mut time_t: *mut core::ffi::c_void,
    mut overlap: *mut WORD32,
    mut frame_len: WORD32,
    mut object_type: WORD32,
    mut stride: WORD32,
    mut window_sequence: WORD16,
    mut p_window_next: *mut WORD16,
    mut slot_element: WORD,
) -> VOID {
    let mut i: WORD32 = 0;
    if object_type == AOT_ER_AAC_LD as core::ffi::c_int {
        let mut ptr_ltp_state0: *mut WORD16 = &mut *lt_pred_stat
            .offset(0 as core::ffi::c_int as isize) as *mut WORD16;
        let mut ptr_ltp_state_fl: *mut WORD16 = &mut *lt_pred_stat
            .offset((frame_len as core::ffi::c_int + 0 as core::ffi::c_int) as isize)
            as *mut WORD16;
        let mut ptr_ltp_state_2fl: *mut WORD16 = &mut *lt_pred_stat
            .offset(
                (frame_len as core::ffi::c_int * 2 as core::ffi::c_int
                    + 0 as core::ffi::c_int) as isize,
            ) as *mut WORD16;
        let mut time: *mut WORD16 = (time_t as *mut WORD16)
            .offset(-(slot_element as isize));
        let mut ptr_time_in: *mut WORD16 = &mut *time
            .offset((0 as WORD32 * stride) as isize) as *mut WORD16;
        i = 0 as core::ffi::c_int as WORD32;
        while i < frame_len {
            let fresh0 = ptr_ltp_state0;
            ptr_ltp_state0 = ptr_ltp_state0.offset(1);
            *fresh0 = *ptr_ltp_state_fl;
            let fresh1 = ptr_ltp_state_fl;
            ptr_ltp_state_fl = ptr_ltp_state_fl.offset(1);
            *fresh1 = *ptr_ltp_state_2fl;
            let fresh2 = ptr_ltp_state_2fl;
            ptr_ltp_state_2fl = ptr_ltp_state_2fl.offset(1);
            *fresh2 = *ptr_time_in;
            ptr_time_in = ptr_time_in.offset(stride as isize);
            i += 1;
        }
    } else {
        let mut ptr_ltp_state0_0: *mut WORD16 = &mut *lt_pred_stat
            .offset(0 as core::ffi::c_int as isize) as *mut WORD16;
        let mut ptr_ltp_state_fl_0: *mut WORD16 = &mut *lt_pred_stat
            .offset((frame_len as core::ffi::c_int + 0 as core::ffi::c_int) as isize)
            as *mut WORD16;
        let mut time_0: *mut WORD32 = time_t as *mut WORD32;
        let mut ptr_time_in_0: *mut WORD32 = &mut *time_0
            .offset((0 as WORD32 * stride) as isize) as *mut WORD32;
        time_0 = time_t as *mut WORD32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < frame_len {
            let fresh3 = ptr_ltp_state0_0;
            ptr_ltp_state0_0 = ptr_ltp_state0_0.offset(1);
            *fresh3 = *ptr_ltp_state_fl_0;
            let fresh4 = ptr_ltp_state_fl_0;
            ptr_ltp_state_fl_0 = ptr_ltp_state_fl_0.offset(1);
            *fresh4 = ixheaac_round16(ixheaac_shl32_sat(*ptr_time_in_0, 2 as WORD));
            ptr_time_in_0 = ptr_time_in_0.offset(stride as isize);
            i += 1;
        }
    }
    if window_sequence as core::ffi::c_int == ONLY_LONG_SEQUENCE
        || window_sequence as core::ffi::c_int == LONG_STOP_SEQUENCE
    {
        if 512 as core::ffi::c_int == frame_len {
            let mut window: *mut WORD32 = p_window_next as *mut WORD32;
            i = 0 as core::ffi::c_int as WORD32;
            while i < 256 as core::ffi::c_int {
                *lt_pred_stat.offset((frame_len * 3 as WORD32 + i) as isize) = ixheaac_round16(
                    ixheaac_mult16x16in32_shl(
                        ixheaac_shl16(
                            -(ixheaac_sat16(
                                *overlap.offset((255 as WORD32 - i) as isize),
                            ) as core::ffi::c_int) as WORD16,
                            1 as WORD16,
                        ),
                        ixheaac_shr32(
                            *window.offset((511 as WORD32 - i) as isize),
                            15 as WORD,
                        ) as WORD16,
                    ),
                );
                *lt_pred_stat
                    .offset((frame_len * 3 as WORD32 + 256 as WORD32 + i) as isize) = ixheaac_round16(
                    ixheaac_mult16x16in32_shl(
                        ixheaac_shl16(
                            -(ixheaac_sat16(*overlap.offset(i as isize))
                                as core::ffi::c_int) as WORD16,
                            1 as WORD16,
                        ),
                        ixheaac_shr32(
                            *window.offset((255 as WORD32 - i) as isize),
                            15 as WORD,
                        ) as WORD16,
                    ),
                );
                i += 1;
            }
        } else if 480 as core::ffi::c_int == frame_len {
            let mut window_0: *mut WORD32 = p_window_next as *mut WORD32;
            i = 0 as core::ffi::c_int as WORD32;
            while i < 240 as core::ffi::c_int {
                *lt_pred_stat.offset((frame_len * 3 as WORD32 + i) as isize) = ixheaac_round16(
                    ixheaac_mult16x16in32_shl(
                        ixheaac_shl16(
                            -(ixheaac_sat16(
                                *overlap.offset((239 as WORD32 - i) as isize),
                            ) as core::ffi::c_int) as WORD16,
                            1 as WORD16,
                        ),
                        ixheaac_shr32(
                            *window_0.offset((479 as WORD32 - i) as isize),
                            15 as WORD,
                        ) as WORD16,
                    ),
                );
                *lt_pred_stat
                    .offset((frame_len * 3 as WORD32 + 240 as WORD32 + i) as isize) = ixheaac_round16(
                    ixheaac_mult16x16in32_shl(
                        ixheaac_shl16(
                            -(ixheaac_sat16(*overlap.offset(i as isize))
                                as core::ffi::c_int) as WORD16,
                            1 as WORD16,
                        ),
                        ixheaac_shr32(
                            *window_0.offset((239 as WORD32 - i) as isize),
                            15 as WORD,
                        ) as WORD16,
                    ),
                );
                i += 1;
            }
        } else {
            i = 0 as core::ffi::c_int as WORD32;
            while i < 512 as core::ffi::c_int {
                *lt_pred_stat.offset((frame_len * 2 as WORD32 + i) as isize) = ixheaac_round16(
                    ixheaac_shl32_sat(
                        ixheaac_mult16x16in32_shl(
                            -(ixheaac_sat16(
                                *overlap.offset((511 as WORD32 - i) as isize),
                            ) as core::ffi::c_int) as WORD16,
                            *p_window_next
                                .offset(
                                    (2 as core::ffi::c_int * i as core::ffi::c_int
                                        + 1 as core::ffi::c_int) as isize,
                                ),
                        ),
                        1 as WORD,
                    ),
                );
                *lt_pred_stat
                    .offset((frame_len * 2 as WORD32 + 512 as WORD32 + i) as isize) = ixheaac_round16(
                    ixheaac_shl32_sat(
                        ixheaac_mult16x16in32_shl(
                            -(ixheaac_sat16(*overlap.offset(i as isize))
                                as core::ffi::c_int) as WORD16,
                            *p_window_next
                                .offset(
                                    (1023 as core::ffi::c_int
                                        - 2 as core::ffi::c_int * i as core::ffi::c_int
                                        - 1 as core::ffi::c_int) as isize,
                                ),
                        ),
                        1 as WORD,
                    ),
                );
                i += 1;
            }
        }
    } else if window_sequence as core::ffi::c_int == LONG_START_SEQUENCE {
        i = 0 as core::ffi::c_int as WORD32;
        while i < 448 as core::ffi::c_int {
            *lt_pred_stat.offset((frame_len * 2 as WORD32 + i) as isize) = ixheaac_shl16(
                -(ixheaac_sat16(*overlap.offset((511 as WORD32 - i) as isize))
                    as core::ffi::c_int) as WORD16,
                1 as WORD16,
            );
            i += 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < 64 as core::ffi::c_int {
            *lt_pred_stat
                .offset((frame_len * 2 as WORD32 + 448 as WORD32 + i) as isize) = ixheaac_round16(
                ixheaac_shl32_sat(
                    ixheaac_mult16x16in32_shl(
                        -(ixheaac_sat16(
                            *overlap.offset((511 as WORD32 - 448 as WORD32 - i) as isize),
                        ) as core::ffi::c_int) as WORD16,
                        *p_window_next
                            .offset(
                                (2 as core::ffi::c_int * i as core::ffi::c_int
                                    + 1 as core::ffi::c_int) as isize,
                            ),
                    ),
                    1 as WORD,
                ),
            );
            i += 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < 64 as core::ffi::c_int {
            *lt_pred_stat
                .offset((frame_len * 2 as WORD32 + 512 as WORD32 + i) as isize) = ixheaac_round16(
                ixheaac_shl32_sat(
                    ixheaac_mult16x16in32_shl(
                        -(ixheaac_sat16(*overlap.offset(i as isize)) as core::ffi::c_int)
                            as WORD16,
                        *p_window_next
                            .offset(
                                (127 as core::ffi::c_int
                                    - 2 as core::ffi::c_int * i as core::ffi::c_int
                                    - 1 as core::ffi::c_int) as isize,
                            ),
                    ),
                    1 as WORD,
                ),
            );
            i += 1;
        }
        i = 576 as core::ffi::c_int as WORD32;
        while i < 1024 as core::ffi::c_int {
            *lt_pred_stat.offset((frame_len * 2 as WORD32 + i) as isize) = 0 as WORD16;
            i += 1;
        }
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i < 448 as core::ffi::c_int {
            *lt_pred_stat.offset((frame_len * 2 as WORD32 + i) as isize) = ixheaac_shl16(
                ixheaac_sat16(*overlap.offset(i as isize)),
                1 as WORD16,
            );
            i += 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < 64 as core::ffi::c_int {
            *lt_pred_stat
                .offset((frame_len * 2 as WORD32 + 448 as WORD32 + i) as isize) = ixheaac_round16(
                ixheaac_shl32_sat(
                    ixheaac_mult16x16in32_shl(
                        -(ixheaac_sat16(*overlap.offset((511 as WORD32 - i) as isize))
                            as core::ffi::c_int) as WORD16,
                        *p_window_next
                            .offset(
                                (2 as core::ffi::c_int * i as core::ffi::c_int
                                    + 1 as core::ffi::c_int) as isize,
                            ),
                    ),
                    1 as WORD,
                ),
            );
            i += 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < 64 as core::ffi::c_int {
            *lt_pred_stat
                .offset((frame_len * 2 as WORD32 + 512 as WORD32 + i) as isize) = ixheaac_round16(
                ixheaac_shl32_sat(
                    ixheaac_mult16x16in32_shl(
                        -(ixheaac_sat16(*overlap.offset((448 as WORD32 + i) as isize))
                            as core::ffi::c_int) as WORD16,
                        *p_window_next
                            .offset(
                                (127 as core::ffi::c_int
                                    - 2 as core::ffi::c_int * i as core::ffi::c_int
                                    - 1 as core::ffi::c_int) as isize,
                            ),
                    ),
                    1 as WORD,
                ),
            );
            i += 1;
        }
        i = 576 as core::ffi::c_int as WORD32;
        while i < 1024 as core::ffi::c_int {
            *lt_pred_stat.offset((frame_len * 2 as WORD32 + i) as isize) = 0 as WORD16;
            i += 1;
        }
    };
}
