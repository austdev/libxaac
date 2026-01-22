extern "C" {
    fn ixheaacd_sqrt(op: WORD32) -> WORD32;
    fn ixheaacd_tns_ma_filter_fixed_ld(
        spectrum: *mut WORD32,
        size: WORD32,
        inc: WORD32,
        lpc: *mut WORD32,
        order: WORD32,
        shift_value: WORD16,
    );
    fn ixheaacd_tns_decode_coefficients(
        filter: *const ia_filter_info_struct,
        a: *mut WORD32,
        ptr_aac_tables: *mut ia_aac_dec_tables_struct,
    ) -> VOID;
    fn ixheaacd_tns_parcor_to_lpc(
        parcor: *mut WORD32,
        lpc: *mut WORD32,
        scale: *mut WORD16,
        order: WORD32,
    ) -> VOID;
    static mut ixheaacd_tns_ar_filter_fixed: Option<
        unsafe extern "C" fn(
            *mut WORD32,
            WORD32,
            WORD32,
            *mut WORD32,
            WORD32,
            WORD32,
            WORD,
        ) -> VOID,
    >;
    static mut ixheaacd_tns_ar_filter: Option<
        unsafe extern "C" fn(
            *mut WORD32,
            WORD32,
            WORD32,
            *mut WORD16,
            WORD32,
            WORD32,
            WORD,
            *mut WORD32,
        ) -> VOID,
    >;
    static mut ixheaacd_tns_parcor_lpc_convert: Option<
        unsafe extern "C" fn(*mut WORD16, *mut WORD16, *mut WORD16, WORD) -> VOID,
    >;
    static mut ixheaacd_calc_max_spectral_line: Option<
        unsafe extern "C" fn(*mut WORD32, WORD32) -> WORD32,
    >;
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
pub type LOOPIDX = core::ffi::c_int;
pub type WORD = core::ffi::c_int;
pub type LOOPINDEX = LOOPIDX;
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
unsafe extern "C" fn ixheaac_min32(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut min_val: WORD32 = 0;
    min_val = if a < b { a } else { b };
    return min_val;
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
unsafe extern "C" fn ixheaac_mult32_shl(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 32 as core::ffi::c_int) as WORD32;
    return result << 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn ixheaac_mult32_shl_sat(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    if a == 0x80000000 as core::ffi::c_uint as WORD32
        && b == 0x80000000 as core::ffi::c_uint as WORD32
    {
        result = 0x7fffffff as core::ffi::c_int as WORD32;
    } else {
        result = ixheaac_mult32_shl(a, b);
    }
    return result;
}
#[inline]
unsafe extern "C" fn ixheaac_div32_pos_normb(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut quotient: WORD32 = 0;
    let mut mantissa_nr: UWORD32 = a as UWORD32;
    let mut mantissa_dr: UWORD32 = b as UWORD32;
    let mut i: LOOPINDEX = 0;
    if a == b {
        quotient = MAX_32;
    } else {
        quotient = 0 as core::ffi::c_int as WORD32;
        i = 0 as core::ffi::c_int as LOOPINDEX;
        while i < 32 as core::ffi::c_int {
            quotient = quotient << 1 as core::ffi::c_int;
            if mantissa_nr >= mantissa_dr {
                mantissa_nr = mantissa_nr.wrapping_sub(mantissa_dr);
                quotient += 1 as core::ffi::c_int;
            }
            mantissa_nr = mantissa_nr << 1 as core::ffi::c_int;
            i += 1;
        }
    }
    return quotient;
}
#[inline]
unsafe extern "C" fn ixheaac_shr32_dir_sat_limit(mut a: WORD32, mut b: WORD) -> WORD32 {
    let mut out_val: WORD32 = 0;
    if b < 0 as core::ffi::c_int {
        out_val = ixheaac_shl32_sat(a, -b);
    } else {
        b = ixheaac_min32(b as WORD32, 31 as WORD32) as WORD;
        out_val = ixheaac_shr32(a, b);
    }
    return out_val;
}
pub const MAX_BINS_LONG: core::ffi::c_int = 1024 as core::ffi::c_int;
pub const PNS_BAND_FLAGS_MASK: WORD16 = (((1 as core::ffi::c_int as WORD16
    as core::ffi::c_int) << PNS_BAND_FLAGS_SHIFT) - 1 as core::ffi::c_int) as WORD16;
pub const PNS_BAND_FLAGS_SHIFT: core::ffi::c_int = 3 as core::ffi::c_int;
pub const PNS_SCALE_MANT_TAB_SIZE: core::ffi::c_int = 4 as core::ffi::c_int;
pub const PNS_SCALE_MANT_TAB_MASK: core::ffi::c_int = PNS_SCALE_MANT_TAB_SIZE
    - 1 as core::ffi::c_int;
pub const LEFT: core::ffi::c_int = 0 as core::ffi::c_int;
#[inline]
unsafe extern "C" fn ixheaacd_is_correlation(
    mut ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
    mut pns_band: WORD16,
) -> WORD16 {
    let mut ptr_corr_info: *mut ia_pns_correlation_info_struct = (*ptr_aac_dec_channel_info)
        .pstr_pns_corr_info;
    return ((*ptr_corr_info)
        .correlated[(pns_band as core::ffi::c_int >> PNS_BAND_FLAGS_SHIFT) as usize]
        as core::ffi::c_int
        >> (pns_band as core::ffi::c_int & PNS_BAND_FLAGS_MASK as core::ffi::c_int)
        & 1 as core::ffi::c_int) as WORD16;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_gen_rand_vec(
    mut scale: WORD32,
    mut shift: WORD,
    mut ptr_spec_coef: *mut WORD32,
    mut sfb_width: WORD32,
    mut seed: *mut WORD32,
) -> VOID {
    let mut nrg_scale: WORD = 0;
    let mut nrg: WORD32 = 0 as WORD32;
    let mut spec: *mut WORD32 = ptr_spec_coef;
    let mut sfb: WORD32 = 0;
    sfb = 0 as core::ffi::c_int as WORD32;
    while sfb <= sfb_width {
        *seed = (1664525 as core::ffi::c_int as WORD64 * *seed as WORD64
            + 1013904223 as core::ffi::c_int as WORD64) as WORD32;
        *spec = *seed >> 3 as core::ffi::c_int;
        nrg = ixheaac_add32_sat(nrg, ixheaac_mult32_shl_sat(*spec, *spec));
        spec = spec.offset(1);
        sfb += 1;
    }
    nrg_scale = ixheaac_norm32(nrg);
    if nrg_scale > 0 as core::ffi::c_int {
        nrg_scale &= !(1 as core::ffi::c_int);
        nrg = ixheaac_shl32_sat(nrg, nrg_scale);
        shift = shift - (nrg_scale >> 1 as core::ffi::c_int);
    }
    nrg = ixheaacd_sqrt(nrg);
    scale = ixheaac_div32_pos_normb(scale, nrg);
    spec = ptr_spec_coef;
    if shift < -(31 as core::ffi::c_int) {
        shift = -(31 as core::ffi::c_int) as WORD;
    }
    sfb = 0 as core::ffi::c_int as WORD32;
    while sfb <= sfb_width {
        *spec = ixheaac_shr32_dir_sat_limit(ixheaac_mult32_shl_sat(*spec, scale), shift);
        spec = spec.offset(1);
        sfb += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_pns_process(
    mut ptr_aac_dec_channel_info: *mut *mut ia_aac_dec_channel_info_struct,
    mut channel: WORD32,
    mut ptr_aac_tables: *mut ia_aac_dec_tables_struct,
) -> VOID {
    let mut ptr_pns_info: *mut ia_pns_info_struct = &mut (**ptr_aac_dec_channel_info
        .offset(channel as isize))
        .str_pns_info;
    let mut ptr_ics_info: *mut ia_ics_info_struct = &mut (**ptr_aac_dec_channel_info
        .offset(channel as isize))
        .str_ics_info;
    let mut maximum_bins_short: WORD16 = ((*ptr_ics_info).frame_length
        as core::ffi::c_int >> 3 as core::ffi::c_int) as WORD16;
    let mut ptr_scale_mant_tab: *mut WORD32 = ((*(*ptr_aac_tables).pstr_block_tables)
        .scale_mant_tab)
        .as_mut_ptr();
    if (*ptr_pns_info).pns_active != 0 {
        let mut swb_offset: *const WORD16 = (*ptr_aac_tables)
            .str_aac_sfb_info[(*ptr_ics_info).window_sequence as usize]
            .sfb_index;
        let mut num_win_group: WORD = 0;
        let mut grp_len: WORD = 0;
        let mut sfb: WORD = 0;
        let mut spec: *mut WORD32 = &mut *((**ptr_aac_dec_channel_info
            .offset(channel as isize))
            .ptr_spec_coeff)
            .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
        num_win_group = 0 as core::ffi::c_int as WORD;
        while num_win_group < (*ptr_ics_info).num_window_groups as core::ffi::c_int {
            grp_len = (*ptr_ics_info).window_group_length[num_win_group as usize]
                as WORD;
            grp_len = 0 as core::ffi::c_int as WORD;
            while grp_len
                < (*ptr_ics_info).window_group_length[num_win_group as usize]
                    as core::ffi::c_int
            {
                sfb = 0 as core::ffi::c_int as WORD;
                while sfb < (*ptr_ics_info).max_sfb as core::ffi::c_int {
                    let mut pns_band: WORD16 = ((num_win_group << 4 as core::ffi::c_int)
                        + sfb) as WORD16;
                    if (**ptr_aac_dec_channel_info.offset(channel as isize))
                        .str_pns_info
                        .pns_used[pns_band as usize] != 0
                    {
                        let mut scale_mant: WORD32 = 0;
                        let mut scale_exp: WORD32 = 0;
                        let mut sfb_width: WORD32 = *swb_offset
                            .offset(
                                (sfb as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                            ) as WORD32 - *swb_offset.offset(sfb as isize) as WORD32
                            - 1 as WORD32;
                        let mut ptr_spec: *mut WORD32 = &mut *spec
                            .offset(*swb_offset.offset(sfb as isize) as isize)
                            as *mut WORD32;
                        scale_mant = *ptr_scale_mant_tab
                            .offset(
                                (*((**ptr_aac_dec_channel_info.offset(channel as isize))
                                    .ptr_scale_factor)
                                    .offset(pns_band as isize) as core::ffi::c_int
                                    & PNS_SCALE_MANT_TAB_MASK) as isize,
                            );
                        scale_exp = (31 as core::ffi::c_int
                            - (*((**ptr_aac_dec_channel_info.offset(channel as isize))
                                .ptr_scale_factor)
                                .offset(pns_band as isize) as core::ffi::c_int
                                >> 2 as core::ffi::c_int) + -(4 as core::ffi::c_int))
                            as WORD32;
                        if ixheaacd_is_correlation(
                            *ptr_aac_dec_channel_info.offset(LEFT as isize),
                            pns_band,
                        ) != 0
                        {
                            if channel == 0 as core::ffi::c_int {
                                (*(**ptr_aac_dec_channel_info.offset(LEFT as isize))
                                    .pstr_pns_corr_info)
                                    .random_vector[pns_band as usize] = (*(**ptr_aac_dec_channel_info
                                    .offset(LEFT as isize))
                                    .pstr_pns_rand_vec_data)
                                    .current_seed;
                                ixheaacd_gen_rand_vec(
                                    scale_mant,
                                    scale_exp as WORD,
                                    ptr_spec,
                                    sfb_width,
                                    &mut (*(**ptr_aac_dec_channel_info.offset(LEFT as isize))
                                        .pstr_pns_rand_vec_data)
                                        .current_seed,
                                );
                            } else {
                                ixheaacd_gen_rand_vec(
                                    scale_mant,
                                    scale_exp as WORD,
                                    ptr_spec,
                                    sfb_width,
                                    &mut *((*(**ptr_aac_dec_channel_info.offset(LEFT as isize))
                                        .pstr_pns_corr_info)
                                        .random_vector)
                                        .as_mut_ptr()
                                        .offset(pns_band as isize),
                                );
                            }
                        } else {
                            ixheaacd_gen_rand_vec(
                                scale_mant,
                                scale_exp as WORD,
                                ptr_spec,
                                sfb_width,
                                &mut (*(**ptr_aac_dec_channel_info.offset(LEFT as isize))
                                    .pstr_pns_rand_vec_data)
                                    .current_seed,
                            );
                        }
                    }
                    sfb += 1;
                }
                if maximum_bins_short as core::ffi::c_int == 120 as core::ffi::c_int {
                    spec = spec.offset(maximum_bins_short as core::ffi::c_int as isize);
                } else {
                    spec = spec.offset(128 as core::ffi::c_int as isize);
                }
                grp_len += 1;
            }
            num_win_group += 1;
        }
    }
    if channel == 0 as core::ffi::c_int {
        let ref mut fresh0 = (*(**ptr_aac_dec_channel_info
            .offset(0 as core::ffi::c_int as isize))
            .pstr_pns_rand_vec_data)
            .pns_frame_number;
        *fresh0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_tns_decode_coef(
    mut filter: *const ia_filter_info_struct,
    mut parcor_coef: *mut WORD16,
    mut ptr_aac_tables: *mut ia_aac_dec_tables_struct,
) -> VOID {
    let mut order: WORD = 0;
    let mut resolution: WORD = 0;
    let mut ptr_par_coef: *mut WORD16 = parcor_coef;
    let mut tns_coeff_ptr: *mut WORD16 = 0 as *mut WORD16;
    let mut ixheaacd_drc_offset: WORD8 = 4 as WORD8;
    let mut ptr_coef: *mut WORD8 = ((*filter).coef).as_ptr() as *mut WORD8;
    resolution = (*filter).resolution as WORD;
    tns_coeff_ptr = ((*(*ptr_aac_tables).pstr_block_tables).tns_coeff3_16).as_mut_ptr();
    if resolution != 0 {
        tns_coeff_ptr = ((*(*ptr_aac_tables).pstr_block_tables).tns_coeff4_16)
            .as_mut_ptr();
        ixheaacd_drc_offset = ((ixheaacd_drc_offset as core::ffi::c_int)
            << 1 as core::ffi::c_int) as WORD8;
    }
    order = 0 as core::ffi::c_int as WORD;
    while order < (*filter).order as core::ffi::c_int {
        let fresh1 = ptr_coef;
        ptr_coef = ptr_coef.offset(1);
        let mut temp: WORD8 = *fresh1;
        let fresh2 = ptr_par_coef;
        ptr_par_coef = ptr_par_coef.offset(1);
        *fresh2 = *tns_coeff_ptr
            .offset(
                (temp as core::ffi::c_int + ixheaacd_drc_offset as core::ffi::c_int)
                    as isize,
            );
        order += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_tns_decode_coef_ld(
    mut filter: *const ia_filter_info_struct,
    mut parcor_coef: *mut WORD32,
    mut ptr_aac_tables: *mut ia_aac_dec_tables_struct,
) -> VOID {
    let mut order: WORD = 0;
    let mut resolution: WORD = 0;
    let mut ptr_par_coef: *mut WORD32 = parcor_coef;
    let mut tns_coeff_ptr: *mut WORD32 = 0 as *mut WORD32;
    let mut offset: WORD8 = 4 as WORD8;
    let mut ptr_coef: *mut WORD8 = ((*filter).coef).as_ptr() as *mut WORD8;
    resolution = (*filter).resolution as WORD;
    tns_coeff_ptr = ((*(*ptr_aac_tables).pstr_block_tables).tns_coeff3).as_mut_ptr();
    if resolution != 0 {
        tns_coeff_ptr = ((*(*ptr_aac_tables).pstr_block_tables).tns_coeff4).as_mut_ptr();
        offset = ((offset as core::ffi::c_int) << 1 as core::ffi::c_int) as WORD8;
    }
    order = 0 as core::ffi::c_int as WORD;
    while order < (*filter).order as core::ffi::c_int {
        let fresh3 = ptr_coef;
        ptr_coef = ptr_coef.offset(1);
        let mut temp: WORD8 = *fresh3;
        let fresh4 = ptr_par_coef;
        ptr_par_coef = ptr_par_coef.offset(1);
        *fresh4 = *tns_coeff_ptr
            .offset((temp as core::ffi::c_int + offset as core::ffi::c_int) as isize);
        order += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_aac_tns_process(
    mut ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
    mut num_ch: WORD32,
    mut ptr_aac_tables: *mut ia_aac_dec_tables_struct,
    mut object_type: WORD32,
    mut ar_flag: WORD32,
    mut predicted_spectrum: *mut WORD32,
) -> VOID {
    let mut i: WORD = 0;
    let mut scale_lpc: WORD16 = 0;
    let mut ptr_tns_info: *mut ia_tns_info_aac_struct = &mut (*ptr_aac_dec_channel_info)
        .str_tns_info;
    let mut spec: *mut WORD32 = (*ptr_aac_dec_channel_info).ptr_spec_coeff;
    let mut scratch_buf: *mut WORD32 = (*ptr_aac_dec_channel_info).scratch_buf_ptr;
    let mut win: WORD = 0;
    let mut filt: WORD = 0;
    let mut start: WORD = 0;
    let mut stop: WORD = 0;
    let mut size: WORD = 0;
    let mut scale_spec: WORD = 0;
    let mut ptr_ics_info: *mut ia_ics_info_struct = &mut (*ptr_aac_dec_channel_info)
        .str_ics_info;
    let mut num_window: WORD = 0;
    let mut tns_max_bands: WORD = 0;
    let mut win_seq: WORD = 0;
    let mut maximum_bins_short: WORD16 = ((*ptr_ics_info).frame_length
        as core::ffi::c_int >> 3 as core::ffi::c_int) as WORD16;
    let mut position: WORD = 0;
    let mut parcor_coef: [WORD32; 32] = [0; 32];
    let mut parcor_coef_16: [WORD16; 32] = [0; 32];
    let mut lpc_coef: [WORD32; 32] = [0; 32];
    let mut lpc_coef_16: [WORD16; 32] = [0; 32];
    let mut ptr_sfb_table: *const WORD16 = 0 as *const WORD16;
    let mut max_bin_long: WORD16 = (*ptr_ics_info).frame_length;
    win_seq = (if (*ptr_ics_info).window_sequence as core::ffi::c_int
        == 0 as core::ffi::c_int
    {
        0 as core::ffi::c_int
    } else {
        ((*ptr_ics_info).window_sequence as core::ffi::c_int % 2 as core::ffi::c_int
            == 0 as core::ffi::c_int) as core::ffi::c_int
    }) as WORD;
    if ar_flag != 0 {
        spec = (*ptr_aac_dec_channel_info).ptr_spec_coeff;
    } else {
        spec = predicted_spectrum;
    }
    if object_type == AOT_ER_AAC_ELD as core::ffi::c_int
        || object_type == AOT_ER_AAC_LD as core::ffi::c_int
        || object_type == AOT_AAC_LTP as core::ffi::c_int
    {
        if 512 as core::ffi::c_int == (*ptr_ics_info).frame_length as core::ffi::c_int {
            tns_max_bands = (*(*ptr_aac_tables).pstr_block_tables)
                .tns_max_bands_tbl_ld[(*ptr_ics_info).sampling_rate_index as usize]
                as WORD;
            win_seq = 1 as core::ffi::c_int as WORD;
            num_window = win_seq;
        } else if 480 as core::ffi::c_int
            == (*ptr_ics_info).frame_length as core::ffi::c_int
        {
            tns_max_bands = (*(*ptr_aac_tables).pstr_block_tables)
                .tns_max_bands_tbl_480[(*ptr_ics_info).sampling_rate_index as usize]
                as WORD;
            win_seq = 1 as core::ffi::c_int as WORD;
            num_window = win_seq;
        } else {
            tns_max_bands = (*(*ptr_aac_tables).pstr_block_tables)
                .tns_max_bands_tbl[(*ptr_ics_info).sampling_rate_index
                as usize][win_seq as usize] as WORD;
            num_window = (if win_seq != 0 {
                8 as core::ffi::c_int
            } else {
                1 as core::ffi::c_int
            }) as WORD;
        }
    } else {
        tns_max_bands = (*(*ptr_aac_tables).pstr_block_tables)
            .tns_max_bands_tbl[(*ptr_ics_info).sampling_rate_index
            as usize][win_seq as usize] as WORD;
        num_window = (if win_seq != 0 {
            8 as core::ffi::c_int
        } else {
            1 as core::ffi::c_int
        }) as WORD;
    }
    ptr_sfb_table = (*ptr_aac_tables)
        .str_aac_sfb_info[(*ptr_ics_info).window_sequence as usize]
        .sfb_index;
    win = 0 as core::ffi::c_int as WORD;
    while win < num_window {
        let mut n_filt: WORD = (*ptr_tns_info).n_filt[win as usize] as WORD;
        let mut current_block_124: u64;
        filt = 0 as core::ffi::c_int as WORD;
        while filt < n_filt {
            let mut filter: *mut ia_filter_info_struct = &mut *(*((*ptr_tns_info)
                .str_filter)
                .as_mut_ptr()
                .offset(win as isize))
                .as_mut_ptr()
                .offset(filt as isize) as *mut ia_filter_info_struct;
            if !((*filter).order as core::ffi::c_int <= 0 as core::ffi::c_int) {
                if object_type == AOT_ER_AAC_LD as core::ffi::c_int
                    || object_type == AOT_AAC_LTP as core::ffi::c_int
                    || num_ch > 2 as core::ffi::c_int
                {
                    ixheaacd_tns_decode_coefficients(
                        filter,
                        parcor_coef.as_mut_ptr(),
                        ptr_aac_tables,
                    );
                } else {
                    ixheaacd_tns_decode_coef(
                        filter,
                        parcor_coef_16.as_mut_ptr(),
                        ptr_aac_tables,
                    );
                }
                start = ixheaac_min32(
                    ixheaac_min32(
                        (*filter).start_band as WORD32,
                        tns_max_bands as WORD32,
                    ),
                    (*ptr_ics_info).max_sfb as WORD32,
                ) as WORD;
                start = *ptr_sfb_table.offset(start as isize) as WORD;
                stop = ixheaac_min32(
                    ixheaac_min32(
                        (*filter).stop_band as WORD32,
                        tns_max_bands as WORD32,
                    ),
                    (*ptr_ics_info).max_sfb as WORD32,
                ) as WORD;
                stop = *ptr_sfb_table.offset(stop as isize) as WORD;
                size = stop - start;
                if !(size <= 0 as core::ffi::c_int) {
                    if object_type == AOT_ER_AAC_LD as core::ffi::c_int
                        || object_type == AOT_AAC_LTP as core::ffi::c_int
                        || num_ch > 2 as core::ffi::c_int
                    {
                        ixheaacd_tns_parcor_to_lpc(
                            parcor_coef.as_mut_ptr(),
                            lpc_coef.as_mut_ptr(),
                            &mut scale_lpc,
                            (*filter).order as WORD32,
                        );
                    } else {
                        (Some(
                            ixheaacd_tns_parcor_lpc_convert
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            parcor_coef_16.as_mut_ptr(),
                            lpc_coef_16.as_mut_ptr(),
                            &mut scale_lpc,
                            (*filter).order as WORD,
                        );
                    }
                    let mut ptr_tmp: *mut WORD32 = 0 as *mut WORD32;
                    if maximum_bins_short as core::ffi::c_int == 120 as core::ffi::c_int
                    {
                        ptr_tmp = spec
                            .offset(
                                (win as core::ffi::c_int
                                    * maximum_bins_short as core::ffi::c_int) as isize,
                            )
                            .offset(start as isize);
                    } else {
                        ptr_tmp = spec
                            .offset((win << 7 as core::ffi::c_int) as isize)
                            .offset(start as isize);
                    }
                    scale_spec = (Some(
                        ixheaacd_calc_max_spectral_line
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")(ptr_tmp, size as WORD32)
                        as WORD;
                    if (*filter).direction as core::ffi::c_int
                        == -(1 as core::ffi::c_int)
                    {
                        position = (stop as core::ffi::c_int - 1 as core::ffi::c_int)
                            as WORD;
                        if maximum_bins_short as core::ffi::c_int
                            == 120 as core::ffi::c_int
                        {
                            if win * maximum_bins_short as WORD + position
                                < (*filter).order as core::ffi::c_int
                            {
                                current_block_124 = 14818589718467733107;
                            } else {
                                current_block_124 = 6560072651652764009;
                            }
                        } else if (win << 7 as core::ffi::c_int) + position
                            < (*filter).order as core::ffi::c_int
                        {
                            current_block_124 = 14818589718467733107;
                        } else {
                            current_block_124 = 6560072651652764009;
                        }
                    } else {
                        position = start;
                        if maximum_bins_short as core::ffi::c_int
                            == 120 as core::ffi::c_int
                        {
                            if win as core::ffi::c_int
                                * maximum_bins_short as core::ffi::c_int
                                + position as core::ffi::c_int
                                + (*filter).order as core::ffi::c_int
                                > max_bin_long as core::ffi::c_int
                            {
                                current_block_124 = 14818589718467733107;
                            } else {
                                current_block_124 = 6560072651652764009;
                            }
                        } else if ((win as core::ffi::c_int) << 7 as core::ffi::c_int)
                            + position as core::ffi::c_int
                            + (*filter).order as core::ffi::c_int > MAX_BINS_LONG
                        {
                            current_block_124 = 14818589718467733107;
                        } else {
                            current_block_124 = 6560072651652764009;
                        }
                    }
                    match current_block_124 {
                        14818589718467733107 => {}
                        _ => {
                            if num_ch <= 2 as core::ffi::c_int
                                && (object_type != AOT_ER_AAC_LD as core::ffi::c_int
                                    && object_type != AOT_AAC_LTP as core::ffi::c_int)
                            {
                                scale_spec = (scale_spec as core::ffi::c_int
                                    - 4 as core::ffi::c_int - scale_lpc as core::ffi::c_int)
                                    as WORD;
                            } else if scale_spec > 17 as core::ffi::c_int {
                                scale_spec = (scale_spec as core::ffi::c_int
                                    - 6 as core::ffi::c_int - scale_lpc as core::ffi::c_int)
                                    as WORD;
                            } else if scale_spec > 11 as core::ffi::c_int {
                                scale_spec = (scale_spec as core::ffi::c_int
                                    - 5 as core::ffi::c_int - scale_lpc as core::ffi::c_int)
                                    as WORD;
                            } else {
                                scale_spec = (scale_spec as core::ffi::c_int
                                    - 4 as core::ffi::c_int - scale_lpc as core::ffi::c_int)
                                    as WORD;
                            }
                            if scale_spec > 0 as core::ffi::c_int {
                                scale_spec = ixheaac_min32(
                                    scale_spec as WORD32,
                                    31 as WORD32,
                                ) as WORD;
                                if object_type == AOT_ER_AAC_LD as core::ffi::c_int
                                    || object_type == AOT_AAC_LTP as core::ffi::c_int
                                    || num_ch > 2 as core::ffi::c_int
                                {
                                    if ar_flag != 0 {
                                        if maximum_bins_short as core::ffi::c_int
                                            == 120 as core::ffi::c_int
                                        {
                                            (Some(
                                                ixheaacd_tns_ar_filter_fixed
                                                    .expect("non-null function pointer"),
                                            ))
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                &mut *spec
                                                    .offset(
                                                        (win * maximum_bins_short as WORD + position) as isize,
                                                    ),
                                                size as WORD32,
                                                (*filter).direction as WORD32,
                                                lpc_coef.as_mut_ptr(),
                                                (*filter).order as WORD32,
                                                scale_lpc as WORD32,
                                                scale_spec,
                                            );
                                        } else {
                                            (Some(
                                                ixheaacd_tns_ar_filter_fixed
                                                    .expect("non-null function pointer"),
                                            ))
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                &mut *spec
                                                    .offset(
                                                        ((win << 7 as core::ffi::c_int) + position) as isize,
                                                    ),
                                                size as WORD32,
                                                (*filter).direction as WORD32,
                                                lpc_coef.as_mut_ptr(),
                                                (*filter).order as WORD32,
                                                scale_lpc as WORD32,
                                                scale_spec,
                                            );
                                        }
                                    } else if maximum_bins_short as core::ffi::c_int
                                        == 120 as core::ffi::c_int
                                    {
                                        ixheaacd_tns_ma_filter_fixed_ld(
                                            &mut *spec
                                                .offset(
                                                    (win * maximum_bins_short as WORD + position) as isize,
                                                ),
                                            size as WORD32,
                                            (*filter).direction as WORD32,
                                            lpc_coef.as_mut_ptr(),
                                            (*filter).order as WORD32,
                                            scale_lpc,
                                        );
                                    } else {
                                        ixheaacd_tns_ma_filter_fixed_ld(
                                            &mut *spec
                                                .offset(
                                                    ((win << 7 as core::ffi::c_int) + position) as isize,
                                                ),
                                            size as WORD32,
                                            (*filter).direction as WORD32,
                                            lpc_coef.as_mut_ptr(),
                                            (*filter).order as WORD32,
                                            scale_lpc,
                                        );
                                    }
                                } else {
                                    if object_type == AOT_ER_AAC_ELD as core::ffi::c_int {
                                        scale_spec = (scale_spec as core::ffi::c_int
                                            - 1 as core::ffi::c_int) as WORD;
                                    }
                                    if maximum_bins_short as core::ffi::c_int
                                        == 120 as core::ffi::c_int
                                    {
                                        (Some(
                                            ixheaacd_tns_ar_filter.expect("non-null function pointer"),
                                        ))
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            &mut *spec
                                                .offset(
                                                    (win * maximum_bins_short as WORD + position) as isize,
                                                ),
                                            size as WORD32,
                                            (*filter).direction as WORD32,
                                            lpc_coef_16.as_mut_ptr(),
                                            (*filter).order as WORD32,
                                            scale_lpc as WORD32,
                                            scale_spec,
                                            scratch_buf,
                                        );
                                    } else {
                                        (Some(
                                            ixheaacd_tns_ar_filter.expect("non-null function pointer"),
                                        ))
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            &mut *spec
                                                .offset(
                                                    ((win << 7 as core::ffi::c_int) + position) as isize,
                                                ),
                                            size as WORD32,
                                            (*filter).direction as WORD32,
                                            lpc_coef_16.as_mut_ptr(),
                                            (*filter).order as WORD32,
                                            scale_lpc as WORD32,
                                            scale_spec,
                                            scratch_buf,
                                        );
                                    }
                                }
                            } else {
                                let mut ptr_tmp_0: *mut WORD32 = 0 as *mut WORD32;
                                if maximum_bins_short as core::ffi::c_int
                                    == 120 as core::ffi::c_int
                                {
                                    ptr_tmp_0 = spec
                                        .offset(
                                            (win as core::ffi::c_int
                                                * maximum_bins_short as core::ffi::c_int) as isize,
                                        )
                                        .offset(start as isize);
                                } else {
                                    ptr_tmp_0 = spec
                                        .offset((win >> 7 as core::ffi::c_int) as isize)
                                        .offset(start as isize);
                                }
                                scale_spec = -scale_spec;
                                scale_spec = ixheaac_min32(
                                    scale_spec as WORD32,
                                    31 as WORD32,
                                ) as WORD;
                                i = size;
                                while i != 0 as core::ffi::c_int {
                                    *ptr_tmp_0 = *ptr_tmp_0 >> scale_spec;
                                    ptr_tmp_0 = ptr_tmp_0.offset(1);
                                    i -= 1;
                                }
                                if object_type == AOT_ER_AAC_LD as core::ffi::c_int
                                    || object_type == AOT_AAC_LTP as core::ffi::c_int
                                    || num_ch > 2 as core::ffi::c_int
                                {
                                    if ar_flag != 0 {
                                        if maximum_bins_short as core::ffi::c_int
                                            == 120 as core::ffi::c_int
                                        {
                                            (Some(
                                                ixheaacd_tns_ar_filter_fixed
                                                    .expect("non-null function pointer"),
                                            ))
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                &mut *spec
                                                    .offset(
                                                        (win * maximum_bins_short as WORD + position) as isize,
                                                    ),
                                                size as WORD32,
                                                (*filter).direction as WORD32,
                                                lpc_coef.as_mut_ptr(),
                                                (*filter).order as WORD32,
                                                scale_lpc as WORD32,
                                                0 as WORD,
                                            );
                                        } else {
                                            (Some(
                                                ixheaacd_tns_ar_filter_fixed
                                                    .expect("non-null function pointer"),
                                            ))
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                &mut *spec
                                                    .offset(
                                                        ((win << 7 as core::ffi::c_int) + position) as isize,
                                                    ),
                                                size as WORD32,
                                                (*filter).direction as WORD32,
                                                lpc_coef.as_mut_ptr(),
                                                (*filter).order as WORD32,
                                                scale_lpc as WORD32,
                                                0 as WORD,
                                            );
                                        }
                                    } else if maximum_bins_short as core::ffi::c_int
                                        == 120 as core::ffi::c_int
                                    {
                                        ixheaacd_tns_ma_filter_fixed_ld(
                                            &mut *spec
                                                .offset(
                                                    (win * maximum_bins_short as WORD + position) as isize,
                                                ),
                                            size as WORD32,
                                            (*filter).direction as WORD32,
                                            lpc_coef.as_mut_ptr(),
                                            (*filter).order as WORD32,
                                            scale_lpc,
                                        );
                                    } else {
                                        ixheaacd_tns_ma_filter_fixed_ld(
                                            &mut *spec
                                                .offset(
                                                    ((win << 7 as core::ffi::c_int) + position) as isize,
                                                ),
                                            size as WORD32,
                                            (*filter).direction as WORD32,
                                            lpc_coef.as_mut_ptr(),
                                            (*filter).order as WORD32,
                                            scale_lpc,
                                        );
                                    }
                                } else {
                                    if object_type == AOT_ER_AAC_ELD as core::ffi::c_int {
                                        scale_lpc = (scale_lpc as core::ffi::c_int
                                            - 1 as core::ffi::c_int) as WORD16;
                                    }
                                    if maximum_bins_short as core::ffi::c_int
                                        == 120 as core::ffi::c_int
                                    {
                                        (Some(
                                            ixheaacd_tns_ar_filter.expect("non-null function pointer"),
                                        ))
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            &mut *spec
                                                .offset(
                                                    (win * maximum_bins_short as WORD + position) as isize,
                                                ),
                                            size as WORD32,
                                            (*filter).direction as WORD32,
                                            lpc_coef_16.as_mut_ptr(),
                                            (*filter).order as WORD32,
                                            scale_lpc as WORD32,
                                            0 as WORD,
                                            scratch_buf,
                                        );
                                    } else {
                                        (Some(
                                            ixheaacd_tns_ar_filter.expect("non-null function pointer"),
                                        ))
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            &mut *spec
                                                .offset(
                                                    ((win << 7 as core::ffi::c_int) + position) as isize,
                                                ),
                                            size as WORD32,
                                            (*filter).direction as WORD32,
                                            lpc_coef_16.as_mut_ptr(),
                                            (*filter).order as WORD32,
                                            scale_lpc as WORD32,
                                            0 as WORD,
                                            scratch_buf,
                                        );
                                    }
                                }
                                if maximum_bins_short as core::ffi::c_int
                                    == 120 as core::ffi::c_int
                                {
                                    ptr_tmp_0 = spec
                                        .offset(
                                            (win as core::ffi::c_int
                                                * maximum_bins_short as core::ffi::c_int) as isize,
                                        )
                                        .offset(start as isize);
                                } else {
                                    ptr_tmp_0 = spec
                                        .offset((win << 7 as core::ffi::c_int) as isize)
                                        .offset(start as isize);
                                }
                                i = size;
                                while i != 0 as core::ffi::c_int {
                                    *ptr_tmp_0 = *ptr_tmp_0 << scale_spec;
                                    ptr_tmp_0 = ptr_tmp_0.offset(1);
                                    i -= 1;
                                }
                            }
                        }
                    }
                }
            }
            filt += 1;
        }
        win += 1;
    }
}
