extern "C" {
    fn ixheaacd_read_bits_buf(
        it_bit_buff: *mut ia_bit_buf_struct,
        no_of_bits: WORD,
    ) -> WORD32;
    fn ixheaacd_aac_showbits_32(
        ptr_read_next: *mut UWORD8,
        cnt_bits: WORD32,
        increment: *mut WORD32,
    ) -> UWORD32;
    fn ixheaacd_aac_read_byte_corr(
        ptr_read_next: *mut *mut UWORD8,
        ptr_bit_pos: *mut WORD32,
        readword: *mut WORD32,
        p_bit_buf_end: *mut UWORD8,
    ) -> VOID;
    fn ixheaacd_aac_read_byte_corr1(
        ptr_read_next: *mut *mut UWORD8,
        ptr_bit_pos: *mut WORD32,
        readword: *mut WORD32,
        p_bit_buf_end: *mut UWORD8,
    ) -> VOID;
    fn ixheaacd_huffman_decode(
        it_bit_buff: WORD32,
        huff_index: *mut WORD16,
        len: *mut WORD16,
        input_table: *const UWORD16,
        idx_table: *const UWORD32,
    ) -> VOID;
    fn ixheaacd_getscalefactorbandoffsets(
        ptr_ics_info: *mut ia_ics_info_struct,
        ptr_aac_tables: *mut ia_aac_dec_tables_struct,
    ) -> *mut WORD16;
}
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
pub const AOT_ER_AAC_LC: AUDIO_OBJECT_TYPE = 17;
pub const AOT_ER_AAC_LD: AUDIO_OBJECT_TYPE = 23;
pub const AOT_ER_AAC_ELD: AUDIO_OBJECT_TYPE = 39;
pub type AUDIO_OBJECT_TYPE = core::ffi::c_uint;
pub const AOT_USAC: AUDIO_OBJECT_TYPE = 42;
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
pub const MAX_16: WORD16 = 0x7fff as core::ffi::c_int as WORD16;
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
unsafe extern "C" fn ixheaac_add16_sat(mut op1: WORD16, mut op2: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    let mut sum: WORD32 = 0;
    sum = op1 as WORD32 + op2 as WORD32;
    var_out = ixheaac_sat16(sum);
    return var_out;
}
pub const MAX_SCALE_FACTOR_BANDS_SHORT: core::ffi::c_int = 16 as core::ffi::c_int;
pub const MAX_SCALE_FACTOR_BANDS_LONG: core::ffi::c_int = 52 as core::ffi::c_int;
pub const ZERO_HCB: core::ffi::c_int = 0 as core::ffi::c_int;
pub const NOISE_OFFSET: core::ffi::c_int = 90 as core::ffi::c_int;
pub const ESC_HCB: core::ffi::c_int = 11 as core::ffi::c_int;
pub const NOISE_HCB: core::ffi::c_int = 13 as core::ffi::c_int;
pub const INTENSITY_HCB2: core::ffi::c_int = 14 as core::ffi::c_int;
pub const INTENSITY_HCB: core::ffi::c_int = 15 as core::ffi::c_int;
pub const EIGHT_SHORT_SEQUENCE: core::ffi::c_int = 2 as core::ffi::c_int;
pub const LINES_PER_UNIT: core::ffi::c_int = 4 as core::ffi::c_int;
pub const MAX_SFB_HCR: core::ffi::c_int = 1024 as core::ffi::c_int
    / 8 as core::ffi::c_int / LINES_PER_UNIT * 8 as core::ffi::c_int;
pub const IA_XHEAAC_DEC_API_NONFATAL_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_EXCEEDS_SFB_TRANSMITTED: core::ffi::c_int = 0x1808
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_INVALID_CODE_BOOK: core::ffi::c_int = 0x180e
    as core::ffi::c_int;
pub const AAC_DEC_OK: core::ffi::c_int = IA_XHEAAC_DEC_API_NONFATAL_NO_ERROR;
pub const LONG_BLOCK_SECT_LEN: core::ffi::c_int = 5 as core::ffi::c_int;
pub const SHORT_BLOCK_SECT_LEN: core::ffi::c_int = 3 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_read_section_data(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
    mut aac_spect_data_resil_flag: WORD32,
    mut aac_sect_data_resil_flag: WORD32,
    mut ptr_aac_tables: *mut ia_aac_dec_tables_struct,
) -> IA_ERRORCODE {
    let mut sfb: WORD = 0;
    let mut sect_cb: WORD = 0;
    let mut sect_len: WORD = 0;
    let mut sect_len_incr: WORD = 0;
    let mut sect_esc_val: WORD = 0;
    let mut ptr_ics_info: *mut ia_ics_info_struct = &mut (*ptr_aac_dec_channel_info)
        .str_ics_info;
    let mut max_sfb: WORD = (*ptr_ics_info).max_sfb as WORD;
    let mut num_win_group: WORD = 0;
    let mut ptr_code_book: *mut WORD8 = (*ptr_aac_dec_channel_info).ptr_code_book;
    let mut ptr_code_book_temp: *mut WORD8 = ptr_code_book;
    let mut sect_bitlen: WORD32 = LONG_BLOCK_SECT_LEN;
    let mut num_lines_sec_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut top: core::ffi::c_int = 0;
    let mut ptr_num_sect_lines: *mut core::ffi::c_short = ((*ptr_aac_dec_channel_info)
        .num_line_in_sec4_hcr_arr)
        .as_mut_ptr() as *mut core::ffi::c_short;
    let mut ptr_hcr_code_book: *mut UWORD8 = ((*ptr_aac_dec_channel_info).cb4_hcr_arr)
        .as_mut_ptr();
    let mut band_offsets: *const core::ffi::c_short = ixheaacd_getscalefactorbandoffsets(
        &mut (*ptr_aac_dec_channel_info).str_ics_info,
        ptr_aac_tables,
    );
    (*ptr_aac_dec_channel_info).number_sect = 0 as core::ffi::c_int as WORD32;
    if (*ptr_aac_dec_channel_info).str_ics_info.window_sequence as core::ffi::c_int
        == EIGHT_SHORT_SEQUENCE
    {
        sect_bitlen = SHORT_BLOCK_SECT_LEN as WORD32;
    }
    sect_esc_val = (((1 as core::ffi::c_int) << sect_bitlen) - 1 as core::ffi::c_int)
        as WORD;
    num_win_group = 0 as core::ffi::c_int as WORD;
    while num_win_group < (*ptr_ics_info).num_window_groups as core::ffi::c_int {
        sfb = 0 as core::ffi::c_int as WORD;
        while sfb < max_sfb {
            sect_len = 0 as core::ffi::c_int as WORD;
            if aac_sect_data_resil_flag != 0 {
                sect_cb = ixheaacd_read_bits_buf(it_bit_buff, 5 as WORD) as WORD;
            } else {
                sect_cb = ixheaacd_read_bits_buf(it_bit_buff, 4 as WORD) as WORD;
            }
            if aac_sect_data_resil_flag == 0 as core::ffi::c_int
                || (sect_cb < 11 as core::ffi::c_int
                    || sect_cb > 11 as core::ffi::c_int
                        && sect_cb < 16 as core::ffi::c_int)
            {
                sect_len_incr = ixheaacd_read_bits_buf(it_bit_buff, sect_bitlen as WORD)
                    as WORD;
                while sect_len_incr == sect_esc_val {
                    sect_len = sect_len + sect_esc_val;
                    sect_len_incr = ixheaacd_read_bits_buf(
                        it_bit_buff,
                        sect_bitlen as WORD,
                    ) as WORD;
                }
            } else {
                sect_len_incr = 1 as core::ffi::c_int as WORD;
            }
            sect_len = sect_len + sect_len_incr;
            if aac_spect_data_resil_flag != 0 {
                top = (sfb + sect_len) as core::ffi::c_int;
                if num_lines_sec_idx >= MAX_SFB_HCR || top >= MAX_SCALE_FACTOR_BANDS_LONG
                {
                    return -(1 as IA_ERRORCODE);
                }
                *ptr_num_sect_lines.offset(num_lines_sec_idx as isize) = (*band_offsets
                    .offset(top as isize) as core::ffi::c_int
                    - *band_offsets.offset(sfb as isize) as core::ffi::c_int)
                    as core::ffi::c_short;
                num_lines_sec_idx += 1;
                if sect_cb == ESC_HCB + 1 as core::ffi::c_int {
                    return IA_XHEAAC_DEC_EXE_NONFATAL_INVALID_CODE_BOOK
                } else {
                    let fresh6 = ptr_hcr_code_book;
                    ptr_hcr_code_book = ptr_hcr_code_book.offset(1);
                    *fresh6 = sect_cb as UWORD8;
                }
                (*ptr_aac_dec_channel_info).number_sect += 1;
            }
            sfb = sfb + sect_len;
            if sfb > max_sfb {
                return IA_XHEAAC_DEC_EXE_NONFATAL_EXCEEDS_SFB_TRANSMITTED as WORD16
                    as IA_ERRORCODE;
            }
            if sect_cb == ESC_HCB + 1 as core::ffi::c_int {
                return IA_XHEAAC_DEC_EXE_NONFATAL_INVALID_CODE_BOOK as WORD16
                    as IA_ERRORCODE;
            }
            loop {
                let fresh7 = sect_len;
                sect_len = sect_len - 1;
                if !(fresh7 != 0) {
                    break;
                }
                let fresh8 = ptr_code_book_temp;
                ptr_code_book_temp = ptr_code_book_temp.offset(1);
                *fresh8 = sect_cb as WORD8;
            }
        }
        ptr_code_book = ptr_code_book.offset(MAX_SCALE_FACTOR_BANDS_SHORT as isize);
        ptr_code_book_temp = ptr_code_book;
        num_win_group += 1;
    }
    return AAC_DEC_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_read_scale_factor_data(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
    mut ptr_aac_tables: *mut ia_aac_dec_tables_struct,
    mut object_type: WORD32,
) -> VOID {
    let mut sfb: WORD = 0;
    let mut position: WORD16 = 0 as WORD16;
    let mut num_win_group: WORD = 0;
    let mut factor: WORD16 = (*ptr_aac_dec_channel_info).global_gain;
    let mut ptr_code_book: *mut WORD8 = 0 as *mut WORD8;
    let mut ptr_code_book_short: *mut WORD8 = 0 as *mut WORD8;
    let mut ptr_scale_fact: *mut WORD16 = 0 as *mut WORD16;
    let mut ptr_scale_fact_short: *mut WORD16 = 0 as *mut WORD16;
    let mut norm_value: WORD16 = 0;
    let mut index: WORD16 = 0;
    let mut length: WORD16 = 0;
    let mut hcod_sf: *const UWORD16 = ((*(*ptr_aac_tables).pstr_huffmann_tables)
        .huffman_code_book_scl)
        .as_mut_ptr();
    let mut table_idx: *const UWORD32 = ((*(*ptr_aac_tables).pstr_huffmann_tables)
        .huffman_code_book_scl_index)
        .as_mut_ptr();
    let mut start_bit_pos: WORD = (*it_bit_buff).bit_pos as WORD;
    let mut start_read_pos: *mut UWORD8 = (*it_bit_buff).ptr_read_next;
    let mut ptr_read_next: *mut UWORD8 = (*it_bit_buff).ptr_read_next;
    let mut bit_pos: WORD32 = 7 as WORD32 - (*it_bit_buff).bit_pos;
    let mut read_word: WORD32 = 0;
    let mut increment: WORD32 = 0;
    read_word = ixheaacd_aac_showbits_32(
        ptr_read_next,
        (*it_bit_buff).cnt_bits,
        &mut increment,
    ) as WORD32;
    ptr_read_next = ptr_read_next.offset(increment as isize);
    ptr_code_book = (*ptr_aac_dec_channel_info).ptr_code_book;
    ptr_scale_fact = (*ptr_aac_dec_channel_info).ptr_scale_factor;
    num_win_group = 0 as core::ffi::c_int as WORD;
    while num_win_group
        < (*ptr_aac_dec_channel_info).str_ics_info.num_window_groups as core::ffi::c_int
    {
        ptr_code_book_short = &mut *ptr_code_book
            .offset(
                (num_win_group as core::ffi::c_int * MAX_SCALE_FACTOR_BANDS_SHORT)
                    as isize,
            ) as *mut WORD8;
        ptr_scale_fact_short = &mut *ptr_scale_fact
            .offset(
                (num_win_group as core::ffi::c_int * MAX_SCALE_FACTOR_BANDS_SHORT)
                    as isize,
            ) as *mut WORD16;
        sfb = ((*ptr_aac_dec_channel_info).str_ics_info.max_sfb as core::ffi::c_int
            - 1 as core::ffi::c_int) as WORD;
        while sfb >= 0 as core::ffi::c_int {
            let fresh0 = ptr_code_book_short;
            ptr_code_book_short = ptr_code_book_short.offset(1);
            let mut sfb_cb: WORD32 = *fresh0 as WORD32;
            if sfb_cb == ZERO_HCB {
                let fresh1 = ptr_scale_fact_short;
                ptr_scale_fact_short = ptr_scale_fact_short.offset(1);
                *fresh1 = 0 as WORD16;
            } else {
                let mut pns_present: WORD32 = 0 as WORD32;
                let mut pns_band: WORD = 0;
                let mut ptr_pns_info: *mut ia_pns_info_struct = &mut (*ptr_aac_dec_channel_info)
                    .str_pns_info;
                if sfb_cb == NOISE_HCB
                    && (*ptr_pns_info).pns_active as core::ffi::c_int
                        != 1 as core::ffi::c_int
                {
                    pns_present = 1 as core::ffi::c_int as WORD32;
                }
                if pns_present == 0 {
                    let mut read_word1: UWORD32 = 0;
                    read_word1 = (read_word << bit_pos) as UWORD32;
                    ixheaacd_huffman_decode(
                        read_word1 as WORD32,
                        &mut index,
                        &mut length,
                        hcod_sf,
                        table_idx,
                    );
                    bit_pos += length as core::ffi::c_int;
                    ixheaacd_aac_read_byte_corr(
                        &mut ptr_read_next,
                        &mut bit_pos,
                        &mut read_word,
                        (*it_bit_buff).ptr_bit_buf_end,
                    );
                    ixheaacd_aac_read_byte_corr1(
                        &mut ptr_read_next,
                        &mut bit_pos,
                        &mut read_word,
                        (*it_bit_buff).ptr_bit_buf_end,
                    );
                    norm_value = (index as core::ffi::c_int - 60 as core::ffi::c_int)
                        as WORD16;
                } else {
                    let mut noise_start_value: WORD32 = 0;
                    let mut temp: UWORD32 = 0;
                    temp = (read_word << bit_pos) as UWORD32;
                    temp = temp >> 32 as core::ffi::c_int - 9 as core::ffi::c_int;
                    noise_start_value = temp as WORD32;
                    bit_pos += 9 as core::ffi::c_int;
                    ixheaacd_aac_read_byte_corr1(
                        &mut ptr_read_next,
                        &mut bit_pos,
                        &mut read_word,
                        (*it_bit_buff).ptr_bit_buf_end,
                    );
                    norm_value = (noise_start_value as core::ffi::c_int
                        - 256 as core::ffi::c_int) as WORD16;
                    (*ptr_pns_info).pns_active = 1 as UWORD16;
                    (*ptr_pns_info).noise_energy = ((*ptr_aac_dec_channel_info)
                        .global_gain as core::ffi::c_int - NOISE_OFFSET) as WORD16;
                }
                if object_type != AOT_ER_AAC_ELD as core::ffi::c_int
                    && object_type != AOT_ER_AAC_LD as core::ffi::c_int
                    && object_type != AOT_ER_AAC_LC as core::ffi::c_int
                {
                    if sfb_cb > NOISE_HCB {
                        position = (position as core::ffi::c_int
                            + norm_value as core::ffi::c_int) as WORD16;
                        let fresh2 = ptr_scale_fact_short;
                        ptr_scale_fact_short = ptr_scale_fact_short.offset(1);
                        *fresh2 = -(position as core::ffi::c_int) as WORD16;
                    } else if sfb_cb < NOISE_HCB {
                        factor = (factor as core::ffi::c_int
                            + norm_value as core::ffi::c_int) as WORD16;
                        let fresh3 = ptr_scale_fact_short;
                        ptr_scale_fact_short = ptr_scale_fact_short.offset(1);
                        *fresh3 = factor;
                    } else {
                        (*ptr_pns_info).noise_energy = ixheaac_add16_sat(
                            (*ptr_pns_info).noise_energy,
                            norm_value,
                        );
                        pns_band = (((num_win_group as core::ffi::c_int)
                            << 4 as core::ffi::c_int)
                            + (*ptr_aac_dec_channel_info).str_ics_info.max_sfb
                                as core::ffi::c_int - sfb as core::ffi::c_int
                            - 1 as core::ffi::c_int) as WORD;
                        *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                            .offset(pns_band as isize) = (*ptr_pns_info).noise_energy;
                        (*ptr_pns_info).pns_used[pns_band as usize] = 1 as UWORD8;
                        ptr_scale_fact_short = ptr_scale_fact_short.offset(1);
                    }
                } else if sfb_cb == INTENSITY_HCB || sfb_cb == INTENSITY_HCB2 {
                    position = (position as core::ffi::c_int
                        + norm_value as core::ffi::c_int) as WORD16;
                    let fresh4 = ptr_scale_fact_short;
                    ptr_scale_fact_short = ptr_scale_fact_short.offset(1);
                    *fresh4 = -(position as core::ffi::c_int) as WORD16;
                } else if sfb_cb == NOISE_HCB {
                    (*ptr_pns_info).noise_energy = ixheaac_add16_sat(
                        (*ptr_pns_info).noise_energy,
                        norm_value,
                    );
                    pns_band = (((num_win_group as core::ffi::c_int)
                        << 4 as core::ffi::c_int)
                        + (*ptr_aac_dec_channel_info).str_ics_info.max_sfb
                            as core::ffi::c_int - sfb as core::ffi::c_int
                        - 1 as core::ffi::c_int) as WORD;
                    *((*ptr_aac_dec_channel_info).ptr_scale_factor)
                        .offset(pns_band as isize) = (*ptr_pns_info).noise_energy;
                    (*ptr_pns_info).pns_used[pns_band as usize] = 1 as UWORD8;
                    ptr_scale_fact_short = ptr_scale_fact_short.offset(1);
                } else {
                    factor = (factor as core::ffi::c_int
                        + norm_value as core::ffi::c_int) as WORD16;
                    let fresh5 = ptr_scale_fact_short;
                    ptr_scale_fact_short = ptr_scale_fact_short.offset(1);
                    *fresh5 = factor;
                }
            }
            sfb -= 1;
        }
        num_win_group += 1;
    }
    (*it_bit_buff).ptr_read_next = ptr_read_next.offset(-(increment as isize));
    (*it_bit_buff).bit_pos = 7 as WORD32 - bit_pos;
    let mut bits_consumed: WORD = 0;
    bits_consumed = (((((*it_bit_buff).ptr_read_next).offset_from(start_read_pos)
        as core::ffi::c_long) << 3 as core::ffi::c_int)
        + (start_bit_pos as WORD32 - (*it_bit_buff).bit_pos) as core::ffi::c_long)
        as WORD;
    (*it_bit_buff).cnt_bits -= bits_consumed as core::ffi::c_int;
}
