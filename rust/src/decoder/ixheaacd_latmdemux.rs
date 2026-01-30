extern "C" {
    pub type ia_sbr_dec_inst_struct;
    pub type ia_mps_dec_ducker_interface;
    fn ixheaacd_read_bits_buf(
        it_bit_buff: *mut ia_bit_buf_struct,
        no_of_bits: WORD,
    ) -> WORD32;
    fn ixheaacd_read_bidirection(
        it_bit_buff: *mut ia_bit_buf_struct,
        ixheaacd_drc_offset: WORD32,
    ) -> VOID;
    fn ixheaacd_ga_hdr_dec(
        pstate_aac_dec: *mut ia_aac_dec_state_struct,
        header_len: WORD32,
        bytes_consumed: *mut WORD32,
        pstr_samp_rate_info: *mut ia_sampling_rate_info_struct,
        it_bit_buff: *mut ia_bit_buf_struct,
    ) -> WORD32;
}
pub type size_t = usize;
pub type WORD8 = core::ffi::c_schar;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD16 = core::ffi::c_short;
pub type UWORD16 = core::ffi::c_ushort;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type WORD64 = core::ffi::c_longlong;
pub type FLOAT32 = core::ffi::c_float;
pub type FLOAT64 = core::ffi::c_double;
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
pub struct ixheaac_drc_data_struct {
    pub prog_ref_level: WORD32,
    pub n_mdct_bands: [WORD16; 16],
    pub drc_fac: [WORD16; 16],
    pub drc_fac_dvb: [WORD16; 16],
    pub drc_exp: WORD8,
    pub short_block: UWORD8,
    pub drc_interp_scheme: UWORD8,
    pub n_drc_bands: UWORD8,
    pub new_prog_ref_level: UWORD8,
    pub new_drc_fac: UWORD8,
    pub prev_interp_scheme: UWORD8,
    pub drc_factors_sbr: [[WORD32; 64]; 64],
    pub drc_factors_sbr_lat: [[WORD32; 64]; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ixheaac_drc_bs_data_struct {
    pub b_channel_on: [UWORD8; 8],
    pub prog_ref_level_present: UWORD8,
    pub prog_ref_level: UWORD8,
    pub drc_num_bands: UWORD8,
    pub drc_band_top: [UWORD8; 16],
    pub dyn_rng_dlbl: [WORD8; 16],
    pub dyn_rng_dlbl_dvb: [WORD8; 16],
    pub max_dyn_rng_dlbl: WORD8,
    pub drc_interpolation_scheme: UWORD8,
    pub drc_data_type: WORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_dec_struct {
    pub str_drc_bs_data: [ixheaac_drc_bs_data_struct; 10],
    pub str_drc_channel_data: [ixheaac_drc_data_struct; 10],
    pub drc_ref_level: WORD16,
    pub drc_def_level: WORD16,
    pub drc_channel_next_index: [UWORD8; 10],
    pub sbr_allowed: UWORD8,
    pub sbr_found: UWORD8,
    pub drc_element_found: UWORD8,
    pub max_audio_channels: UWORD8,
    pub length_history: UWORD8,
    pub num_drc_elements: UWORD8,
    pub state: WORD32,
    pub target_ref_level: WORD32,
    pub prog_ref_level: WORD32,
    pub cut_factor: WORD32,
    pub boost_factor: WORD32,
    pub drc_dig_norm: FLAG,
    pub drc_on: FLAG,
    pub dvb_anc_data_present: FLAG,
    pub dvb_anc_data_pos: WORD32,
    pub pres_mode: WORD32,
    pub heavy_mode: WORD32,
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
pub struct ia_ec_sfb_str {
    pub num_sfb_long: WORD32,
    pub num_sfb_short: WORD32,
    pub ptr_sfb_long: *mut WORD16,
    pub ptr_sfb_short: *mut WORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_ec_scratch_str {
    pub prev_sfb_nrg: [WORD32; 51],
    pub pres_sfb_nrg: [WORD32; 51],
    pub spec_coeff: [WORD32; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_ec_state_str {
    pub spectral_coeff: [WORD32; 1024],
    pub q_spec_coeff: [WORD16; 128],
    pub prev_frame_ok: [WORD32; 2],
    pub win_shape: UWORD8,
    pub win_shape_prev: UWORD8,
    pub win_seq: WORD32,
    pub td_frame_prev: WORD32,
    pub fac_data_present: WORD32,
    pub prev_win_group_len: UWORD8,
    pub conceal_state: WORD32,
    pub prev_core_mode: WORD32,
    pub fade_idx: WORD32,
    pub lsf4: [FLOAT32; 16],
    pub str_ec_sfb: ia_ec_sfb_str,
    pub pstr_ec_scratch: *mut ia_ec_scratch_str,
    pub str_ec_scratch: ia_ec_scratch_str,
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
pub struct ia_aac_err_config_struct {
    pub aac_sect_data_resil_flag: WORD32,
    pub aac_sf_data_resil_flag: WORD32,
    pub aac_spect_data_resil_flag: WORD32,
    pub ep_config: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_eld_specific_config_struct {
    pub ld_sbr_flag_present: WORD32,
    pub ld_sbr_samp_rate: WORD32,
    pub ld_sbr_crc_flag: WORD32,
    pub ldSbrHeaderPresent: WORD32,
    pub aac_sect_data_resil_flag: WORD32,
    pub aac_sf_data_resil_flag: WORD32,
    pub aac_spect_data_resil_flag: WORD32,
    pub ep_config: WORD32,
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
pub struct ia_aac_dec_ola_data {
    pub win_shape: WORD16,
    pub win_seq: WORD16,
    pub ptr_overlap_buf: *mut WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_channel_info {
    pub ptr_long_window: [*const WORD16; 2],
    pub ptr_short_window: [*const WORD16; 2],
    pub overlap_add_data: ia_aac_dec_ola_data,
    pub ltp_buf: *mut WORD16,
    pub ltp_lag_1: UWORD16,
    pub ltp_lag_2: UWORD16,
    pub str_ec_state: ia_ec_state_str,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_element_stream_struct {
    pub sbr_ele_id: WORD32,
    pub extension_type: WORD32,
    pub size_payload: WORD32,
    pub ptr_sbr_data: *mut WORD8,
    pub ptr_prev_sbr_data: *mut WORD8,
    pub prev_size_payload: WORD32,
    pub frame_error_flag: [WORD32; 2],
    pub use_frame_slot: WORD32,
    pub prev_sbr_ele_id: WORD32,
    pub prev_extension_type: WORD32,
    pub size_payload_old: WORD32,
    pub sbr_prev_data: [WORD8; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_sbr_bitstream_struct {
    pub no_elements: WORD16,
    pub str_sbr_ele: [ia_sbr_element_stream_struct; 1],
}
pub type ia_handle_sbr_dec_inst_struct = *mut ia_sbr_dec_inst_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ixheaacd_misc_tables {
    pub trig_data: [WORD16; 513],
    pub sine_table8_16: [WORD16; 8],
    pub log_dual_is_table: [WORD16; 65],
    pub down_mix_martix: [[[WORD32; 8]; 2]; 4],
    pub cc_gain_scale: [WORD32; 4],
    pub inv_table: [WORD16; 256],
    pub sqrt_table: [WORD16; 257],
    pub dummy: WORD32,
    pub start_band: [[WORD32; 16]; 10],
    pub stop_band: [[WORD32; 16]; 10],
    pub stop_freq_table_fs40k_2: [WORD32; 14],
    pub stop_freq_table_fs40k_4: [WORD32; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_scale_fact_struct {
    pub lb_scale: WORD16,
    pub st_lb_scale: WORD16,
    pub ov_lb_scale: WORD16,
    pub hb_scale: WORD16,
    pub ov_hb_scale: WORD16,
    pub st_syn_scale: WORD16,
    pub ps_scale: WORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_freq_band_data_struct {
    pub num_sf_bands: [WORD16; 2],
    pub num_nf_bands: WORD16,
    pub num_mf_bands: WORD16,
    pub sub_band_start: WORD16,
    pub sub_band_end: WORD16,
    pub freq_band_tbl_lim: [WORD16; 13],
    pub num_lf_bands: WORD16,
    pub num_if_bands: WORD16,
    pub freq_band_table: [*mut WORD16; 2],
    pub freq_band_tbl_lo: [WORD16; 29],
    pub freq_band_tbl_hi: [WORD16; 57],
    pub freq_band_tbl_noise: [WORD16; 6],
    pub f_master_tbl: [WORD16; 57],
    pub qmf_sb_prev: WORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_header_data_struct {
    pub sync_state: WORD32,
    pub err_flag: FLAG,
    pub err_flag_prev: FLAG,
    pub num_time_slots: WORD16,
    pub time_step: WORD16,
    pub core_frame_size: WORD16,
    pub out_sampling_freq: WORD32,
    pub channel_mode: WORD32,
    pub amp_res: WORD16,
    pub start_freq: WORD16,
    pub stop_freq: WORD16,
    pub xover_band: WORD16,
    pub freq_scale: WORD16,
    pub alter_scale: WORD16,
    pub noise_bands: WORD16,
    pub limiter_bands: WORD16,
    pub limiter_gains: WORD16,
    pub interpol_freq: WORD16,
    pub smoothing_mode: WORD16,
    pub pstr_freq_band_data: *mut ia_freq_band_data_struct,
    pub header_extra_1: WORD16,
    pub header_extra_2: WORD16,
    pub pre_proc_flag: WORD16,
    pub status: WORD32,
    pub sbr_ratio_idx: WORD32,
    pub upsamp_fac: WORD32,
    pub is_usf_4: WORD32,
    pub output_framesize: WORD32,
    pub usac_independency_flag: WORD32,
    pub pvc_flag: FLAG,
    pub hbe_flag: FLAG,
    pub esbr_start_up: WORD32,
    pub esbr_start_up_pvc: WORD32,
    pub usac_flag: WORD32,
    pub pvc_mode: UWORD8,
    pub enh_sbr: FLAG,
    pub esbr_hq: FLAG,
    pub enh_sbr_ps: FLAG,
    pub eld_sbr: FLAG,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_frame_info_struct {
    pub frame_class: WORD16,
    pub num_env: WORD16,
    pub transient_env: WORD16,
    pub num_noise_env: WORD16,
    pub border_vec: [WORD16; 9],
    pub freq_res: [WORD16; 8],
    pub noise_border_vec: [WORD16; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_env_calc_tables_struct {
    pub sbr_lim_gains_m: [WORD16; 8],
    pub sbr_lim_bands_per_octave_q13: [WORD16; 4],
    pub sbr_smooth_filter: [WORD16; 4],
    pub sbr_inv_int_table: [WORD16; 49],
    pub sbr_rand_ph: [WORD32; 568],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_qmf_dec_tables_struct {
    pub w_32: [WORD16; 60],
    pub w_16: [WORD16; 24],
    pub dig_rev_table2_32: [WORD32; 4],
    pub dig_rev_table4_16: [WORD32; 2],
    pub sbr_sin_cos_twiddle_l64: [WORD16; 64],
    pub sbr_alt_sin_twiddle_l64: [WORD16; 32],
    pub sbr_cos_sin_twiddle_ds_l32: [WORD16; 64],
    pub sbr_sin_cos_twiddle_l32: [WORD16; 32],
    pub sbr_alt_sin_twiddle_l32: [WORD16; 16],
    pub sbr_t_cos_sin_l32: [WORD16; 64],
    pub post_fft_tbl: [WORD16; 18],
    pub dct23_tw: [WORD16; 66],
    pub qmf_c: [WORD16; 1280],
    pub dig_rev_table2_128: [UWORD8; 4],
    pub w1024: [WORD32; 1536],
    pub esbr_qmf_c: [WORD32; 1280],
    pub esbr_qmf_c_24: [WORD32; 480],
    pub esbr_w_32: [WORD32; 60],
    pub esbr_w_16: [WORD32; 24],
    pub esbr_sin_cos_twiddle_l64: [WORD32; 64],
    pub esbr_alt_sin_twiddle_l64: [WORD32; 32],
    pub esbr_sin_cos_twiddle_l32: [WORD32; 32],
    pub esbr_alt_sin_twiddle_l32: [WORD32; 16],
    pub esbr_t_cos_sin_l32: [WORD32; 64],
    pub esbr_sin_cos_twiddle_l24: [WORD32; 24],
    pub esbr_alt_sin_twiddle_l24: [WORD32; 12],
    pub esbr_t_cos_sin_l24: [WORD32; 48],
    pub esbr_sin_cos_twiddle_l16: [WORD32; 16],
    pub esbr_alt_sin_twiddle_l16: [WORD32; 8],
    pub esbr_t_cos_sin_l16: [WORD32; 32],
    pub ixheaacd_sbr_t_cos_sin_l32_eld: [WORD16; 64],
    pub qmf_c_eld: [WORD16; 640],
    pub qmf_c_eld2: [WORD16; 640],
    pub qmf_c_eld3: [WORD16; 640],
    pub qmf_c_ldsbr_mps: [WORD32; 640],
    pub ixheaacd_sbr_synth_cos_sin_l32: [WORD16; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_env_extr_tables_struct {
    pub sbr_frame_info1_2_4_16: [ia_frame_info_struct; 7],
    pub str_sbr_default_header: ia_sbr_header_data_struct,
    pub ixheaacd_t_huffman_env_bal_1_5db_inp_table: [WORD16; 50],
    pub ixheaacd_f_huffman_env_bal_1_5db_inp_table: [WORD16; 50],
    pub ixheaacd_t_huffman_env_bal_3_0db_inp_table: [WORD16; 26],
    pub ixheaacd_f_huffman_env_bal_3_0db_inp_table: [WORD16; 26],
    pub ixheaacd_t_huffman_noise_3_0db_inp_table: [WORD16; 64],
    pub ixheaacd_t_huffman_noise_bal_3_0db_inp_table: [WORD16; 26],
    pub ixheaacd_t_huffman_env_1_5db_inp_table: [WORD16; 122],
    pub ixheaacd_f_huffman_env_1_5db_inp_table: [WORD16; 122],
    pub ixheaacd_t_huffman_env_3_0db_inp_table: [WORD16; 64],
    pub ixheaacd_f_huffman_env_3_0db_inp_table: [WORD16; 64],
    pub ixheaacd_t_huffman_env_bal_1_5db_idx_table: [WORD32; 20],
    pub ixheaacd_f_huffman_env_bal_1_5db_idx_table: [WORD32; 23],
    pub ixheaacd_t_huffman_env_bal_3_0db_idx_table: [WORD32; 16],
    pub ixheaacd_f_huffman_env_bal_3_0db_idx_table: [WORD32; 17],
    pub ixheaacd_t_huffman_noise_3_0db_idx_table: [WORD32; 17],
    pub ixheaacd_t_huffman_noise_bal_3_0db_idx_table: [WORD32; 11],
    pub ixheaacd_t_huffman_env_1_5db_idx_table: [WORD32; 27],
    pub ixheaacd_f_huffman_env_1_5db_idx_table: [WORD32; 28],
    pub ixheaacd_t_huffman_env_3_0db_idx_table: [WORD32; 26],
    pub ixheaacd_f_huffman_env_3_0db_idx_table: [WORD32; 25],
    pub start_min: [WORD8; 12],
    pub offset_idx: [WORD8; 12],
    pub ixheaacd_drc_offset: [[WORD8; 16]; 7],
    pub stop_min: [WORD8; 12],
    pub stop_off: [[WORD8; 14]; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_ps_tables_struct {
    pub decay_scale_factor: [WORD16; 72],
    pub hyb_resol: [WORD16; 3],
    pub rev_link_decay_ser: [WORD16; 3],
    pub rev_link_delay_ser: [WORD16; 3],
    pub borders_group: [WORD16; 23],
    pub group_shift: [WORD16; 6],
    pub group_to_bin: [WORD16; 22],
    pub hybrid_to_bin: [WORD16; 10],
    pub delay_to_bin: [WORD16; 32],
    pub frac_delay_phase_fac_qmf_re_im: [WORD16; 48],
    pub frac_delay_phase_fac_qmf_sub_re_im: [WORD16; 32],
    pub frac_delay_phase_fac_qmf_ser_re_im: [[WORD16; 64]; 3],
    pub frac_delay_phase_fac_qmf_sub_ser_re_im: [[WORD16; 32]; 3],
    pub scale_factors: [WORD16; 15],
    pub scale_factors_fine: [WORD16; 31],
    pub alpha_values: [WORD16; 8],
    pub p2_6: [WORD16; 6],
    pub p8_13: [WORD16; 13],
    pub huff_iid_dt: [WORD16; 28],
    pub huff_iid_df: [WORD16; 28],
    pub huff_icc_dt: [WORD16; 14],
    pub huff_icc_df: [WORD16; 14],
    pub huff_iid_dt_fine: [WORD16; 60],
    pub huff_iid_df_fine: [WORD16; 60],
    pub dummy: WORD32,
    pub qmf_fract_delay_phase_factor_im: [FLOAT32; 64],
    pub qmf_fract_delay_phase_factor_re: [FLOAT32; 64],
    pub frac_delay_phase_fac_qmf_sub_im_20: [FLOAT32; 12],
    pub frac_delay_phase_fac_qmf_sub_re_20: [FLOAT32; 12],
    pub frac_delay_phase_fac_qmf_sub_im_34: [FLOAT32; 32],
    pub frac_delay_phase_fac_qmf_sub_re_34: [FLOAT32; 32],
    pub qmf_ser_fract_delay_phase_factor_im: [[FLOAT32; 3]; 64],
    pub qmf_ser_fract_delay_phase_factor_re: [[FLOAT32; 3]; 64],
    pub frac_delay_phase_fac_ser_qmf_sub_im_20: [[FLOAT32; 3]; 12],
    pub frac_delay_phase_fac_ser_qmf_sub_re_20: [[FLOAT32; 3]; 12],
    pub frac_delay_phase_fac_ser_qmf_sub_im_34: [[FLOAT32; 3]; 32],
    pub frac_delay_phase_fac_ser_qmf_sub_re_34: [[FLOAT32; 3]; 32],
    pub scale_factors_flt: [FLOAT32; 15],
    pub scale_factors_fine_flt: [FLOAT32; 31],
    pub alphas: [FLOAT32; 8],
    pub all_pass_link_decay_ser: [FLOAT32; 3],
    pub p8_13_20: [FLOAT32; 13],
    pub p2_13_20: [FLOAT32; 13],
    pub p12_13_34: [FLOAT32; 13],
    pub p8_13_34: [FLOAT32; 13],
    pub p4_13_34: [FLOAT32; 13],
    pub cos_mod_2channel: [[FLOAT32; 13]; 2],
    pub cos_sin_mod_4channel: [[FLOAT32; 26]; 4],
    pub cos_sin_mod_8channel: [[FLOAT32; 26]; 8],
    pub cos_sin_mod_12channel: [[FLOAT32; 26]; 12],
    pub qmf_delay_idx_tbl: [WORD32; 64],
    pub group_borders_20_tbl: [WORD32; 23],
    pub group_borders_34_tbl: [WORD32; 51],
    pub bin_group_map_20: [WORD32; 22],
    pub bin_group_map_34: [WORD32; 50],
    pub quantized_iids: [WORD32; 7],
    pub quantized_iids_fine: [WORD32; 15],
    pub quantized_rhos: [FLOAT32; 8],
    pub ipd_bins_tbl: [WORD32; 3],
    pub band_res_hyb20: [WORD16; 3],
    pub band_res_hyb34: [WORD16; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_tables_struct {
    pub env_calc_tables_ptr: *mut ia_env_calc_tables_struct,
    pub qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
    pub env_extr_tables_ptr: *mut ia_env_extr_tables_struct,
    pub ps_tables_ptr: *mut ia_ps_tables_struct,
    pub sbr_rand_ph: *mut WORD32,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ixheaacd_latm_specs {
    pub aot: AUDIO_OBJECT_TYPE,
    pub sampling_freq_index: UWORD32,
    pub sampling_freq: UWORD32,
    pub channel_config: WORD32,
    pub samples_per_frame: UWORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ixheaacd_latm_layer_info {
    pub frame_len_type: UWORD32,
    pub buffer_fullness: UWORD32,
    pub frame_len_bits: UWORD32,
    pub asc: ixheaacd_latm_specs,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ixheaacd_latm_struct {
    pub use_same_stream_mux: UWORD32,
    pub audio_mux_version: UWORD32,
    pub all_streams_same_time_framing: UWORD32,
    pub num_sub_frames: UWORD32,
    pub num_program: UWORD32,
    pub num_layer: UWORD32,
    pub use_same_config: UWORD32,
    pub other_data_present: UWORD32,
    pub other_data_length: UWORD32,
    pub crc_check_present: UWORD32,
    pub crc_check_sum: UWORD32,
    pub frame_length: UWORD32,
    pub layer_info: [[ixheaacd_latm_layer_info; 8]; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_decoder_struct {
    pub frame_status: FLAG,
    pub byte_align_bits: WORD32,
    pub pstr_sbr_bitstream: *mut ia_aac_dec_sbr_bitstream_struct,
    pub pstr_aac_dec_ch_info: [*mut ia_aac_dec_channel_info_struct; 2],
    pub ptr_aac_dec_static_channel_info: [*mut ia_aac_dec_channel_info; 2],
    pub pstr_aac_dec_overlap_info: [*mut ia_aac_dec_overlap_info; 2],
    pub pstr_pns_rand_vec_data: *mut ia_pns_rand_vec_struct,
    pub p_ind_channel_info: *mut core::ffi::c_void,
    pub block_number: WORD32,
    pub sampling_rate_index: WORD16,
    pub sampling_rate: WORD32,
    pub samples_per_frame: WORD32,
    pub channels: WORD16,
    pub num_swb_window: [WORD8; 2],
    pub pstr_aac_tables: *mut ia_aac_dec_tables_struct,
    pub pstr_common_tables: *mut ixheaacd_misc_tables,
    pub is_first: WORD32,
    pub conceal_count: WORD32,
    pub sbr_num_elements: WORD16,
}
pub type UINT32 = UWORD32;
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
pub struct ia_parametric_drc_lim_struct {
    pub parametric_lim_threshold_present: WORD32,
    pub parametric_lim_threshold: FLOAT32,
    pub parametric_lim_attack: WORD32,
    pub parametric_lim_release_present: WORD32,
    pub parametric_lim_release: WORD32,
    pub drc_characteristic: WORD32,
    pub disable_paramteric_drc: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_parametric_drc_type_feed_forward_struct {
    pub level_estim_k_weighting_type: WORD32,
    pub level_estim_integration_time_present: WORD32,
    pub level_estim_integration_time: WORD32,
    pub drc_curve_definition_type: WORD32,
    pub drc_characteristic: WORD32,
    pub node_count: WORD32,
    pub node_level: [WORD32; 9],
    pub node_gain: [WORD32; 9],
    pub drc_gain_smooth_parameters_present: WORD32,
    pub gain_smooth_attack_time_slow: WORD32,
    pub gain_smooth_release_time_slow: WORD32,
    pub gain_smooth_time_fast_present: WORD32,
    pub gain_smooth_attack_time_fast: WORD32,
    pub gain_smooth_release_time_fast: WORD32,
    pub gain_smooth_threshold_present: WORD32,
    pub gain_smooth_attack_threshold: WORD32,
    pub gain_smooth_rel_threshold: WORD32,
    pub gain_smooth_hold_off_count_present: WORD32,
    pub gain_smooth_hold_off: WORD32,
    pub disable_paramteric_drc: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_parametric_drc_instructions_struct {
    pub parametric_drc_id: WORD32,
    pub parametric_drc_look_ahead_flag: WORD32,
    pub parametric_drc_look_ahead: WORD32,
    pub parametric_drc_preset_id_present: WORD32,
    pub parametric_drc_preset_id: WORD32,
    pub parametric_drc_type: WORD32,
    pub len_bit_size: WORD32,
    pub str_parametric_drc_type_feed_forward: ia_parametric_drc_type_feed_forward_struct,
    pub parametric_drc_lim: ia_parametric_drc_lim_struct,
    pub drc_characteristic: WORD32,
    pub disable_paramteric_drc: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_parametric_drc_gain_set_params_struct {
    pub parametric_drc_id: WORD32,
    pub side_chain_config_type: WORD32,
    pub downmix_id: WORD32,
    pub level_estim_channel_weight_format: WORD32,
    pub level_estim_ch_weight: [FLOAT32; 128],
    pub drc_input_loudness_present: WORD32,
    pub drc_input_loudness: FLOAT32,
    pub ch_count_from_dwnmix_id: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_coeff_parametric_drc_struct {
    pub drc_location: WORD32,
    pub parametric_drc_frame_size_format: WORD32,
    pub parametric_drc_frame_size: WORD32,
    pub parametric_drc_delay_max_present: WORD32,
    pub parametric_drc_delay_max: WORD32,
    pub reset_parametric_drc: WORD32,
    pub parametric_drc_gain_set_count: WORD32,
    pub str_parametric_drc_gain_set_params: [ia_parametric_drc_gain_set_params_struct; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_loud_eq_instructions_struct {
    pub loud_eq_set_id: WORD32,
    pub drc_location: WORD32,
    pub dwnmix_id_count: WORD32,
    pub downmix_id: [WORD32; 8],
    pub drc_set_id_count: WORD32,
    pub drc_set_id: [WORD32; 16],
    pub eq_set_id_count: WORD32,
    pub eq_set_id: [WORD32; 8],
    pub loudness_after_drc: WORD32,
    pub loudness_after_eq: WORD32,
    pub loud_eq_gain_sequence_count: WORD32,
    pub gain_seq_idx: [WORD32; 4],
    pub drc_characteristic_format_is_cicp: [WORD32; 4],
    pub drc_characteristic: [WORD32; 4],
    pub drc_characteristic_left_index: [WORD32; 4],
    pub drc_characteristic_right_index: [WORD32; 4],
    pub frequency_range_index: [WORD32; 4],
    pub loud_eq_scaling: [FLOAT32; 4],
    pub loud_eq_offset: [FLOAT32; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_filt_ele_struct {
    pub filt_ele_idx: WORD32,
    pub filt_ele_gain_flag: WORD32,
    pub filt_ele_gain: FLOAT32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_filt_block_struct {
    pub filter_element_count: WORD32,
    pub str_filter_element: [ia_filt_ele_struct; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_unique_td_filt_element {
    pub eq_filter_format: WORD32,
    pub bs_real_zero_radius_one_count: WORD32,
    pub real_zero_count: WORD32,
    pub generic_zero_count: WORD32,
    pub real_pole_count: WORD32,
    pub cmplx_pole_count: WORD32,
    pub zero_sign: [WORD32; 14],
    pub real_zero_radius: [FLOAT32; 64],
    pub generic_zero_radius: [FLOAT32; 64],
    pub generic_zero_angle: [FLOAT32; 64],
    pub real_pole_radius: [FLOAT32; 16],
    pub complex_pole_radius: [FLOAT32; 16],
    pub complex_pole_angle: [FLOAT32; 16],
    pub fir_filt_order: WORD32,
    pub fir_symmetry: WORD32,
    pub fir_coeff: [FLOAT32; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_eq_subband_gain_spline_struct {
    pub num_eq_nodes: WORD32,
    pub eq_slope: [FLOAT32; 33],
    pub eq_freq_delta: [WORD32; 33],
    pub eq_gain_initial: FLOAT32,
    pub eq_gain_delta: [FLOAT32; 33],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_eq_subband_gain_vector {
    pub eq_subband_gain: [FLOAT32; 135],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_eq_coeff_struct {
    pub eq_delay_max_present: WORD32,
    pub eq_delay_max: WORD32,
    pub unique_filter_block_count: WORD32,
    pub str_filter_block: [ia_filt_block_struct; 16],
    pub unique_td_filter_element_count: WORD32,
    pub unique_td_filt_ele: [ia_unique_td_filt_element; 16],
    pub unique_eq_subband_gains_count: WORD32,
    pub eq_subband_gain_representation: WORD32,
    pub eq_subband_gain_format: WORD32,
    pub eq_subband_gain_count: WORD32,
    pub str_eq_subband_gain_spline: [ia_eq_subband_gain_spline_struct; 16],
    pub str_eq_subband_gain_vector: [ia_eq_subband_gain_vector; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_filter_block_refs_struct {
    pub filter_block_count: WORD32,
    pub filter_block_index: [WORD32; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_td_filter_cascade_struct {
    pub eq_cascade_gain_present: [WORD32; 4],
    pub eq_cascade_gain: [FLOAT32; 4],
    pub str_filter_block_refs: [ia_filter_block_refs_struct; 4],
    pub eq_phase_alignment_present: WORD32,
    pub eq_phase_alignment: [[WORD32; 4]; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_eq_instructions_struct {
    pub eq_set_id: WORD32,
    pub eq_set_complexity_level: WORD32,
    pub dwnmix_id_count: WORD32,
    pub downmix_id: [WORD32; 8],
    pub eq_apply_to_downmix: WORD32,
    pub drc_set_id_count: WORD32,
    pub drc_set_id: [WORD32; 16],
    pub eq_set_purpose: WORD32,
    pub depends_on_eq_set_present: WORD32,
    pub depends_on_eq_set: WORD32,
    pub no_independent_eq_use: WORD32,
    pub eq_channel_count: WORD32,
    pub eq_ch_group_count: WORD32,
    pub eq_ch_group_of_channel: [WORD32; 128],
    pub td_filter_cascade_present: WORD32,
    pub str_td_filter_cascade: ia_td_filter_cascade_struct,
    pub subband_gains_present: WORD32,
    pub subband_gains_index: [WORD32; 4],
    pub eq_transition_duration_present: WORD32,
    pub eq_transition_duration: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_config_ext {
    pub drc_config_ext_type: [WORD32; 2],
    pub ext_bit_size: [WORD32; 1],
    pub parametric_drc_present: WORD32,
    pub str_drc_coeff_param_drc: ia_drc_coeff_parametric_drc_struct,
    pub parametric_drc_instructions_count: WORD32,
    pub str_parametric_drc_instructions: [ia_parametric_drc_instructions_struct; 8],
    pub drc_extension_v1_present: WORD32,
    pub loud_eq_instructions_flag: WORD32,
    pub loud_eq_instructions_count: WORD32,
    pub loud_eq_instructions: [ia_loud_eq_instructions_struct; 8],
    pub eq_flag: WORD32,
    pub str_eq_coeff: ia_eq_coeff_struct,
    pub eq_instructions_count: WORD32,
    pub str_eq_instructions: [ia_eq_instructions_struct; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_coefficients_basic_struct {
    pub drc_location: WORD32,
    pub drc_characteristic: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_instructions_basic_struct {
    pub drc_set_id: WORD32,
    pub drc_location: WORD32,
    pub dwnmix_id_count: WORD32,
    pub downmix_id: [WORD32; 8],
    pub drc_set_effect: WORD32,
    pub limiter_peak_target_present: WORD32,
    pub limiter_peak_target: FLOAT32,
    pub drc_set_target_loudness_present: WORD32,
    pub drc_set_target_loudness_value_upper: WORD32,
    pub drc_set_target_loudness_value_lower_present: WORD32,
    pub drc_set_target_loudness_value_lower: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_gain_params_struct {
    pub gain_seq_idx: WORD32,
    pub drc_characteristic: WORD32,
    pub drc_characteristic_present: WORD32,
    pub drc_characteristic_format_is_cicp: WORD32,
    pub drc_characteristic_left_index: WORD32,
    pub drc_characteristic_right_index: WORD32,
    pub crossover_freq_idx: WORD32,
    pub start_subband_index: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_delta_time_code_table_entry_struct {
    pub size: WORD32,
    pub code: WORD32,
    pub value: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_tables_struct {
    pub delta_time_code_table: [ia_delta_time_code_table_entry_struct; 526],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_gain_set_params_struct {
    pub gain_coding_profile: WORD32,
    pub gain_interpolation_type: WORD32,
    pub full_frame: WORD32,
    pub time_alignment: WORD32,
    pub time_delt_min_flag: WORD32,
    pub time_delt_min_val: WORD32,
    pub band_count: WORD32,
    pub drc_band_type: WORD32,
    pub gain_params: [ia_gain_params_struct; 8],
    pub num_gain_max_values: WORD32,
    pub str_tables: ia_tables_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_split_drc_characteristic_struct {
    pub characteristic_format: WORD32,
    pub in_out_ratio: FLOAT32,
    pub gain: FLOAT32,
    pub exp: FLOAT32,
    pub flip_sign: WORD32,
    pub characteristic_node_count: WORD32,
    pub node_level: [FLOAT32; 5],
    pub node_gain: [FLOAT32; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_shape_filter_params_struct {
    pub corner_freq_index: WORD32,
    pub filter_strength_index: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_shape_filter_block_params_struct {
    pub lf_cut_filter_present: WORD32,
    pub str_lf_cut_params: ia_shape_filter_params_struct,
    pub lf_boost_filter_present: WORD32,
    pub str_lf_boost_params: ia_shape_filter_params_struct,
    pub hf_cut_filter_present: WORD32,
    pub str_hf_cut_params: ia_shape_filter_params_struct,
    pub hf_boost_filter_present: WORD32,
    pub str_hf_boost_params: ia_shape_filter_params_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_uni_drc_coeffs_struct {
    pub version: WORD32,
    pub drc_location: WORD32,
    pub drc_frame_size_present: WORD32,
    pub drc_frame_size: WORD32,
    pub gain_set_count: WORD32,
    pub gain_set_params: [ia_gain_set_params_struct; 24],
    pub drc_characteristic_left_present: WORD32,
    pub characteristic_left_count: WORD32,
    pub str_split_characteristic_left: [ia_split_drc_characteristic_struct; 8],
    pub drc_characteristic_right_present: WORD32,
    pub characteristic_right_count: WORD32,
    pub str_split_characteristic_right: [ia_split_drc_characteristic_struct; 8],
    pub shape_filters_present: WORD32,
    pub shape_num_filter: WORD32,
    pub str_shape_filter_block_params: [ia_shape_filter_block_params_struct; 9],
    pub gain_sequence_count: WORD32,
    pub gain_set_params_index_for_gain_sequence: [WORD32; 24],
    pub gain_set_count_plus: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_gain_modifiers_struct {
    pub target_characteristic_left_present: [WORD32; 8],
    pub target_characteristic_left_index: [WORD32; 8],
    pub target_characteristic_right_present: [WORD32; 8],
    pub target_characteristic_right_index: [WORD32; 8],
    pub shape_filter_flag: WORD32,
    pub shape_filter_idx: WORD32,
    pub gain_scaling_flag: [WORD32; 8],
    pub attn_scaling: [FLOAT32; 8],
    pub ampl_scaling: [FLOAT32; 8],
    pub gain_offset_flag: [WORD32; 8],
    pub gain_offset: [FLOAT32; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_ducking_modifiers_struct {
    pub ducking_scaling_flag: WORD32,
    pub ducking_scaling: FLOAT32,
    pub ducking_scaling_quantized: FLOAT32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_instructions_struct {
    pub drc_set_id: WORD32,
    pub drc_set_complexity_level: WORD32,
    pub requires_eq: WORD32,
    pub drc_apply_to_dwnmix: WORD32,
    pub drc_location: WORD32,
    pub dwnmix_id_count: WORD32,
    pub downmix_id: [WORD32; 8],
    pub depends_on_drc_set_present: WORD32,
    pub depends_on_drc_set: WORD32,
    pub no_independent_use: WORD32,
    pub drc_set_effect: WORD32,
    pub gain_set_index: [WORD32; 128],
    pub str_gain_modifiers_of_ch_group: [ia_gain_modifiers_struct; 24],
    pub str_ducking_modifiers_for_channel: [ia_ducking_modifiers_struct; 128],
    pub limiter_peak_target_present: WORD32,
    pub limiter_peak_target: FLOAT32,
    pub drc_set_target_loudness_present: WORD32,
    pub drc_set_target_loudness_value_upper: WORD32,
    pub drc_set_target_loudness_value_lower_present: WORD32,
    pub drc_set_target_loudness_value_lower: WORD32,
    pub audio_num_chan: WORD32,
    pub num_drc_ch_groups: WORD32,
    pub gain_set_index_for_channel_group: [WORD32; 24],
    pub band_count_of_ch_group: [WORD32; 24],
    pub gain_interpolation_type_for_channel_group: [WORD32; 24],
    pub time_delta_min_for_channel_group: [WORD32; 24],
    pub time_alignment_for_channel_group: [WORD32; 24],
    pub str_ducking_modifiers_for_channel_group: [ia_ducking_modifiers_struct; 24],
    pub channel_group_of_ch: [WORD32; 128],
    pub num_chan_per_ch_group: [WORD32; 24],
    pub gain_element_count: WORD32,
    pub multiband_audio_sig_count: WORD32,
    pub ch_group_parametric_drc_flag: [WORD32; 24],
    pub gain_set_idx_of_ch_group_parametric_drc: [WORD32; 24],
    pub parametric_drc_look_ahead_samples: [WORD32; 24],
    pub parametric_drc_look_ahead_samples_max: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_channel_layout_struct {
    pub base_channel_count: WORD32,
    pub layout_signaling_present: WORD32,
    pub defined_layout: WORD32,
    pub speaker_position: [WORD32; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_downmix_instructions_struct {
    pub downmix_id: WORD32,
    pub target_channel_count: WORD32,
    pub target_layout: WORD32,
    pub downmix_coefficients_present: WORD32,
    pub downmix_coefficient: [FLOAT32; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_config {
    pub sample_rate_present: WORD32,
    pub sampling_rate: WORD32,
    pub dwnmix_instructions_count: WORD32,
    pub drc_coefficients_drc_count: WORD32,
    pub drc_instructions_uni_drc_count: WORD32,
    pub drc_instructions_count_plus: WORD32,
    pub drc_description_basic_present: WORD32,
    pub drc_coefficients_basic_count: WORD32,
    pub drc_instructions_basic_count: WORD32,
    pub drc_config_ext_present: WORD32,
    pub apply_drc: WORD32,
    pub str_drc_config_ext: ia_drc_config_ext,
    pub str_drc_coefficients_basic: [ia_drc_coefficients_basic_struct; 8],
    pub str_drc_instructions_basic: [ia_drc_instructions_basic_struct; 36],
    pub str_p_loc_drc_coefficients_uni_drc: [ia_uni_drc_coeffs_struct; 8],
    pub str_drc_instruction_str: [ia_drc_instructions_struct; 36],
    pub channel_layout: ia_channel_layout_struct,
    pub dwnmix_instructions: [ia_downmix_instructions_struct; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_qmf_filter_bank_struct {
    pub no_channels: WORD32,
    pub analy_win_coeff: *const WORD16,
    pub p_filter: *const WORD16,
    pub cos_twiddle: *const WORD16,
    pub sin_twiddle: *const WORD16,
    pub alt_sin_twiddle: *const WORD16,
    pub t_cos: *const WORD16,
    pub t_sin: *const WORD16,
    pub anal_filter_states: *mut WORD16,
    pub filter_states: *mut WORD16,
    pub num_time_slots: WORD16,
    pub lsb: WORD16,
    pub usb: WORD16,
    pub qmf_filter_state_size: WORD16,
    pub core_samples_buffer: *mut WORD16,
    pub ana_offset: WORD16,
    pub filter_pos: *mut WORD16,
    pub dummy_0: *mut WORD16,
    pub ixheaacd_drc_offset: WORD16,
    pub filter_pos_syn: *mut WORD16,
    pub dummy_1: *mut WORD16,
    pub analy_win_coeff_32: *mut WORD32,
    pub p_filter_32: *const WORD32,
    pub esbr_cos_twiddle: *const WORD32,
    pub esbr_alt_sin_twiddle: *const WORD32,
    pub esbr_t_cos: *const WORD32,
    pub anal_filter_states_32: *mut WORD32,
    pub state_new_samples_pos_low_32: *mut WORD32,
    pub filter_states_32: *mut WORD32,
    pub filter_pos_32: *mut WORD32,
    pub filter_pos_syn_32: *mut WORD32,
    pub fp1_anal: *mut WORD16,
    pub fp2_anal: *mut WORD16,
    pub filter_2: *mut WORD16,
    pub fp1_syn: *mut WORD16,
    pub fp2_syn: *mut WORD16,
    pub sixty4: WORD16,
    pub core_samples_buffer_32: *mut WORD32,
    pub fp1_anal_32: *mut WORD32,
    pub fp2_anal_32: *mut WORD32,
    pub filter_2_32: *mut WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_qmf_ana_filter_bank {
    pub p_filter_ana: *const WORD32,
    pub ref_co_eff_ptr_l: *const WORD32,
    pub ref_co_eff_ptr_r: *const WORD32,
    pub offset_l: WORD32,
    pub offset_r: WORD32,
    pub qmf_states_buffer: *mut WORD32,
    pub flag: WORD16,
    pub offset: WORD16,
    pub qmf_states_curr_pos: *mut WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_qmf_syn_filter_bank {
    pub p_filter_syn: *const WORD32,
    pub sbr_qmf_states_synthesis: *mut WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_qmf_tables_struct {
    pub ia_mps_enc_qmf_64_640: [WORD32; 650],
    pub sbr_alt_sin_twiddle: [WORD16; 33],
    pub sbr_cos_twiddle: [WORD16; 32],
    pub sbr_sin_twiddle: [WORD16; 32],
    pub fft_c: [WORD16; 4],
    pub ia_qmf_anl_addt_cos: [WORD16; 32],
    pub ia_qmf_anl_addt_sin: [WORD16; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_common_tables_struct {
    pub sqrt_tab: [WORD32; 513],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_hybrid_tables_struct {
    pub p8_13: [WORD16; 19],
    pub p2_6: [WORD16; 6],
    pub sine_array: [WORD32; 2048],
    pub cosine_array: [WORD32; 2048],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_index_tables_struct {
    pub row_2_channel_stp: [[WORD32; 8]; 7],
    pub row_2_channel_ges: [[WORD32; 8]; 7],
    pub row_2_residual: [[WORD32; 8]; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_m1_m2_tables_struct {
    pub idx_table: ia_mps_dec_index_tables_struct,
    pub hybrid_2_param_28: [WORD32; 71],
    pub r1_matrix_l: [WORD32; 31],
    pub ten_cld_by_10: [WORD32; 31],
    pub w00_cld2_15: [WORD32; 31],
    pub table_kappa: [WORD32; 8],
    pub dec_pow: [WORD32; 31],
    pub cld_tab_1: [WORD32; 31],
    pub cld_tab_2: [WORD32; 31],
    pub cld_tab_3: [WORD32; 31],
    pub reciprocal: [WORD32; 576],
    pub c_l_table: [WORD32; 31],
    pub cos_table: [[WORD32; 31]; 16],
    pub sin_table: [[WORD32; 31]; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_rev_tables_struct {
    pub rev_delay: [[WORD32; 10]; 4],
    pub rev_split_freq_0: [WORD32; 4],
    pub rev_split_freq_1: [WORD32; 4],
    pub rev_split_freq_2: [WORD32; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_decorr_tables_struct {
    pub rev_table: ia_mps_dec_rev_tables_struct,
    pub lattice_coeff_0: [[WORD32; 20]; 10],
    pub lattice_coeff_1: [[WORD32; 15]; 10],
    pub lattice_coeff_2: [[WORD32; 6]; 10],
    pub lattice_coeff_3: [[WORD32; 3]; 10],
    pub den_coef_0: [[WORD32; 21]; 10],
    pub den_coef_1: [[WORD32; 16]; 10],
    pub den_coef_2: [[WORD32; 7]; 10],
    pub den_coef_3: [[WORD32; 4]; 10],
    pub lattice_delta_phi: [[WORD32; 20]; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_tp_process_tables_struct {
    pub bp: [WORD32; 25],
    pub bpxgf: [WORD32; 25],
    pub bp2xgf2: [WORD32; 25],
    pub ia_mps_dec_qmf_64_640: [WORD32; 325],
    pub time_out_idx_5xxx: [WORD32; 6],
    pub time_out_idx_7xxx: [WORD32; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_wf_ptr_table_struct {
    pub wf: [*const WORD32; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_mdct2qmf_table_struct {
    pub twi_post_cos: [WORD32; 64],
    pub twi_post_sin: [WORD32; 64],
    pub hybrid_2_qmf: [WORD32; 71],
    pub local_sin_4: [WORD32; 4],
    pub local_sin_15: [WORD32; 16],
    pub local_sin_16: [WORD32; 16],
    pub local_sin_18: [WORD32; 18],
    pub local_sin_24: [WORD32; 24],
    pub local_sin_30: [WORD32; 30],
    pub local_sin_32: [WORD32; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_tonality_tables_struct {
    pub gmax_fix: [WORD16; 72],
    pub dwin_fix: [WORD32; 72],
    pub nstart_fix: [[WORD32; 72]; 5],
    pub dfrac_fix: [[WORD32; 56]; 5],
    pub part4: [WORD32; 4],
    pub part5: [WORD32; 5],
    pub part7: [WORD32; 7],
    pub part10: [WORD32; 10],
    pub part14: [WORD32; 14],
    pub part20: [WORD32; 20],
    pub part28: [WORD32; 28],
    pub part40: [WORD32; 40],
    pub w_real: [WORD32; 16],
    pub w_imag: [WORD32; 16],
    pub bitrev: [WORD32; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_kernels_table_struct {
    pub kernels_4_to_71: [WORD32; 71],
    pub kernels_5_to_71: [WORD32; 71],
    pub kernels_7_to_71: [WORD32; 71],
    pub kernels_10_to_71: [WORD32; 71],
    pub kernels_14_to_71: [WORD32; 71],
    pub kernels_20_to_71: [WORD32; 71],
    pub kernels_28_to_71: [WORD32; 71],
    pub bb_env_kernels: [WORD32; 71],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_mapping_table_struct {
    pub mapping_4_to_28: [WORD32; 28],
    pub mapping_5_to_28: [WORD32; 28],
    pub mapping_7_to_28: [WORD32; 28],
    pub mapping_10_to_28: [WORD32; 28],
    pub mapping_14_to_28: [WORD32; 28],
    pub mapping_20_to_28: [WORD32; 28],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_tree_properties_struct {
    pub num_input_channels: WORD32,
    pub num_output_channels: WORD32,
    pub num_ott_boxes: WORD32,
    pub num_ttt_boxes: WORD32,
    pub ott_mode_lfe: [WORD32; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_bitdec_tables_struct {
    pub kernel_table: ia_mps_dec_kernels_table_struct,
    pub map_table: ia_mps_dec_mapping_table_struct,
    pub tree_property_table: [ia_mps_dec_tree_properties_struct; 7],
    pub sampling_freq_table: [WORD32; 15],
    pub freq_res_table: [WORD32; 8],
    pub temp_shape_chan_table: [[WORD32; 7]; 2],
    pub surround_gain_table: [WORD32; 5],
    pub lfe_gain_table: [WORD32; 5],
    pub clip_gain_table: [WORD32; 8],
    pub pb_stride_table: [WORD32; 4],
    pub smg_time_table: [WORD32; 4],
    pub dequant_cld: [WORD32; 31],
    pub dequant_cld_coarse: [WORD32; 15],
    pub dequant_cpc: [WORD32; 52],
    pub dequant_cpc_coarse: [WORD32; 26],
    pub dequant_icc: [WORD32; 8],
    pub factor_cld_tab_1: [WORD32; 31],
    pub hrtf_power: [WORD32; 64],
    pub envshape_data: [[WORD32; 5]; 2],
    pub pcm_chnksz_level_3: [WORD32; 5],
    pub pcm_chnksz_level_4: WORD32,
    pub pcm_chnksz_level_7: [WORD32; 6],
    pub pcm_chnksz_level_8: WORD32,
    pub pcm_chnksz_level_11: [WORD32; 2],
    pub pcm_chnksz_level_13: [WORD32; 4],
    pub pcm_chnksz_level_15: WORD32,
    pub pcm_chnksz_level_19: [WORD32; 4],
    pub pcm_chnksz_level_25: [WORD32; 3],
    pub pcm_chnksz_level_26: WORD32,
    pub pcm_chnksz_level_31: WORD32,
    pub pcm_chnksz_level_51: [WORD32; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_mesh_tables_struct {
    pub blind_cld_mesh: [[WORD32; 21]; 31],
    pub blind_icc_mesh: [[WORD32; 21]; 31],
    pub blind_cpc_1_mesh: [[WORD32; 21]; 31],
    pub blind_cpc_2_mesh: [[WORD32; 21]; 31],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_blind_tables_struct {
    pub mesh_table: ia_mps_dec_mesh_tables_struct,
    pub exp_1: [WORD32; 13],
    pub exp_2: [WORD32; 13],
    pub exp_4: [WORD32; 13],
    pub exp_8: [WORD32; 13],
    pub exp_16: [WORD32; 13],
    pub exp_32: [WORD32; 13],
    pub exp_64: [WORD32; 13],
    pub exp_128: [WORD32; 13],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_mdct2qmf_cos_table_struct {
    pub cos_table_long: [*mut WORD16; 64],
    pub cos_table_short: [*mut WORD16; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_mdct2qmf_tables_struct {
    pub cos_table_long_32_00: [WORD16; 32],
    pub cos_table_long_32_01: [WORD16; 32],
    pub cos_table_long_32_02: [WORD16; 32],
    pub cos_table_long_32_03: [WORD16; 32],
    pub cos_table_long_32_04: [WORD16; 32],
    pub cos_table_long_32_05: [WORD16; 32],
    pub cos_table_long_32_06: [WORD16; 32],
    pub cos_table_long_32_07: [WORD16; 32],
    pub cos_table_long_32_08: [WORD16; 32],
    pub cos_table_long_32_09: [WORD16; 32],
    pub cos_table_long_32_10: [WORD16; 32],
    pub cos_table_long_32_11: [WORD16; 32],
    pub cos_table_long_32_12: [WORD16; 32],
    pub cos_table_long_32_13: [WORD16; 32],
    pub cos_table_long_32_14: [WORD16; 32],
    pub cos_table_long_32_15: [WORD16; 32],
    pub cos_table_long_32_16: [WORD16; 32],
    pub cos_table_long_32_17: [WORD16; 32],
    pub cos_table_long_32_18: [WORD16; 32],
    pub cos_table_long_32_19: [WORD16; 32],
    pub cos_table_long_32_20: [WORD16; 32],
    pub cos_table_long_32_21: [WORD16; 32],
    pub cos_table_long_32_22: [WORD16; 32],
    pub cos_table_long_32_23: [WORD16; 32],
    pub cos_table_long_32_24: [WORD16; 32],
    pub cos_table_long_32_25: [WORD16; 32],
    pub cos_table_long_32_26: [WORD16; 32],
    pub cos_table_long_32_27: [WORD16; 32],
    pub cos_table_long_32_28: [WORD16; 32],
    pub cos_table_long_32_29: [WORD16; 32],
    pub cos_table_long_32_30: [WORD16; 32],
    pub cos_table_long_32_31: [WORD16; 32],
    pub cos_table_long_30_00: [WORD16; 30],
    pub cos_table_long_30_01: [WORD16; 30],
    pub cos_table_long_30_02: [WORD16; 30],
    pub cos_table_long_30_03: [WORD16; 30],
    pub cos_table_long_30_04: [WORD16; 30],
    pub cos_table_long_30_05: [WORD16; 30],
    pub cos_table_long_30_06: [WORD16; 30],
    pub cos_table_long_30_07: [WORD16; 30],
    pub cos_table_long_30_08: [WORD16; 30],
    pub cos_table_long_30_09: [WORD16; 30],
    pub cos_table_long_30_10: [WORD16; 30],
    pub cos_table_long_30_11: [WORD16; 30],
    pub cos_table_long_30_12: [WORD16; 30],
    pub cos_table_long_30_13: [WORD16; 30],
    pub cos_table_long_30_14: [WORD16; 30],
    pub cos_table_long_30_15: [WORD16; 30],
    pub cos_table_long_30_16: [WORD16; 30],
    pub cos_table_long_30_17: [WORD16; 30],
    pub cos_table_long_30_18: [WORD16; 30],
    pub cos_table_long_30_19: [WORD16; 30],
    pub cos_table_long_30_20: [WORD16; 30],
    pub cos_table_long_30_21: [WORD16; 30],
    pub cos_table_long_30_22: [WORD16; 30],
    pub cos_table_long_30_23: [WORD16; 30],
    pub cos_table_long_30_24: [WORD16; 30],
    pub cos_table_long_30_25: [WORD16; 30],
    pub cos_table_long_30_26: [WORD16; 30],
    pub cos_table_long_30_27: [WORD16; 30],
    pub cos_table_long_30_28: [WORD16; 30],
    pub cos_table_long_30_29: [WORD16; 30],
    pub cos_table_long_24_00: [WORD16; 24],
    pub cos_table_long_24_01: [WORD16; 24],
    pub cos_table_long_24_02: [WORD16; 24],
    pub cos_table_long_24_03: [WORD16; 24],
    pub cos_table_long_24_04: [WORD16; 24],
    pub cos_table_long_24_05: [WORD16; 24],
    pub cos_table_long_24_06: [WORD16; 24],
    pub cos_table_long_24_07: [WORD16; 24],
    pub cos_table_long_24_08: [WORD16; 24],
    pub cos_table_long_24_09: [WORD16; 24],
    pub cos_table_long_24_10: [WORD16; 24],
    pub cos_table_long_24_11: [WORD16; 24],
    pub cos_table_long_24_12: [WORD16; 24],
    pub cos_table_long_24_13: [WORD16; 24],
    pub cos_table_long_24_14: [WORD16; 24],
    pub cos_table_long_24_15: [WORD16; 24],
    pub cos_table_long_24_16: [WORD16; 24],
    pub cos_table_long_24_17: [WORD16; 24],
    pub cos_table_long_24_18: [WORD16; 24],
    pub cos_table_long_24_19: [WORD16; 24],
    pub cos_table_long_24_20: [WORD16; 24],
    pub cos_table_long_24_21: [WORD16; 24],
    pub cos_table_long_24_22: [WORD16; 24],
    pub cos_table_long_24_23: [WORD16; 24],
    pub cos_table_long_18_00: [WORD16; 18],
    pub cos_table_long_18_01: [WORD16; 18],
    pub cos_table_long_18_02: [WORD16; 18],
    pub cos_table_long_18_03: [WORD16; 18],
    pub cos_table_long_18_04: [WORD16; 18],
    pub cos_table_long_18_05: [WORD16; 18],
    pub cos_table_long_18_06: [WORD16; 18],
    pub cos_table_long_18_07: [WORD16; 18],
    pub cos_table_long_18_08: [WORD16; 18],
    pub cos_table_long_18_09: [WORD16; 18],
    pub cos_table_long_18_10: [WORD16; 18],
    pub cos_table_long_18_11: [WORD16; 18],
    pub cos_table_long_18_12: [WORD16; 18],
    pub cos_table_long_18_13: [WORD16; 18],
    pub cos_table_long_18_14: [WORD16; 18],
    pub cos_table_long_18_15: [WORD16; 18],
    pub cos_table_long_18_16: [WORD16; 18],
    pub cos_table_long_18_17: [WORD16; 18],
    pub cos_table_long_16_00: [WORD16; 16],
    pub cos_table_long_16_01: [WORD16; 16],
    pub cos_table_long_16_02: [WORD16; 16],
    pub cos_table_long_16_03: [WORD16; 16],
    pub cos_table_long_16_04: [WORD16; 16],
    pub cos_table_long_16_05: [WORD16; 16],
    pub cos_table_long_16_06: [WORD16; 16],
    pub cos_table_long_16_07: [WORD16; 16],
    pub cos_table_long_16_08: [WORD16; 16],
    pub cos_table_long_16_09: [WORD16; 16],
    pub cos_table_long_16_10: [WORD16; 16],
    pub cos_table_long_16_11: [WORD16; 16],
    pub cos_table_long_16_12: [WORD16; 16],
    pub cos_table_long_16_13: [WORD16; 16],
    pub cos_table_long_16_14: [WORD16; 16],
    pub cos_table_long_16_15: [WORD16; 16],
    pub cos_table_long_15_00: [WORD16; 15],
    pub cos_table_long_15_01: [WORD16; 15],
    pub cos_table_long_15_02: [WORD16; 15],
    pub cos_table_long_15_03: [WORD16; 15],
    pub cos_table_long_15_04: [WORD16; 15],
    pub cos_table_long_15_05: [WORD16; 15],
    pub cos_table_long_15_06: [WORD16; 15],
    pub cos_table_long_15_07: [WORD16; 15],
    pub cos_table_long_15_08: [WORD16; 15],
    pub cos_table_long_15_09: [WORD16; 15],
    pub cos_table_long_15_10: [WORD16; 15],
    pub cos_table_long_15_11: [WORD16; 15],
    pub cos_table_long_15_12: [WORD16; 15],
    pub cos_table_long_15_13: [WORD16; 15],
    pub cos_table_long_15_14: [WORD16; 15],
    pub cos_table_short_4_00: [WORD16; 4],
    pub cos_table_short_4_01: [WORD16; 4],
    pub cos_table_short_4_02: [WORD16; 4],
    pub cos_table_short_4_03: [WORD16; 4],
    pub cos_table_short_3_00: [WORD16; 3],
    pub cos_table_short_3_01: [WORD16; 3],
    pub cos_table_short_3_02: [WORD16; 3],
    pub cos_table_short_2_00: [WORD16; 2],
    pub cos_table_short_2_01: [WORD16; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_res_block_tables_struct {
    pub pow_table_q17: [WORD32; 129],
    pub scale_table: [WORD32; 4],
    pub scale_table_960: [WORD32; 4],
    pub tns_max_bands_tbl: [[WORD8; 2]; 12],
    pub tns_coeff3_16: [WORD16; 8],
    pub tns_coeff4_16: [WORD16; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_res_huffmann_tables_struct {
    pub sfb_96_1024: [WORD8; 43],
    pub sfb_96_128: [WORD8; 14],
    pub sfb_64_1024: [WORD8; 49],
    pub sfb_48_1024: [WORD8; 51],
    pub sfb_48_128: [WORD8; 16],
    pub sfb_32_1024: [WORD8; 53],
    pub sfb_24_1024: [WORD8; 49],
    pub sfb_24_128: [WORD8; 17],
    pub sfb_16_1024: [WORD8; 45],
    pub sfb_16_128: [WORD8; 17],
    pub sfb_8_1024: [WORD8; 42],
    pub sfb_8_128: [WORD8; 17],
    pub sfb_96_960: [WORD8; 41],
    pub sfb_96_120: [WORD8; 13],
    pub sfb_64_960: [WORD8; 47],
    pub sfb_48_960: [WORD8; 50],
    pub sfb_48_120: [WORD8; 15],
    pub sfb_24_960: [WORD8; 47],
    pub sfb_24_120: [WORD8; 16],
    pub sfb_16_960: [WORD8; 43],
    pub sfb_16_120: [WORD8; 16],
    pub sfb_8_960: [WORD8; 41],
    pub sfb_8_120: [WORD8; 16],
    pub huffman_code_book_1: [UWORD16; 108],
    pub huffman_code_book_2: [UWORD16; 110],
    pub huffman_code_book_3: [UWORD16; 136],
    pub huffman_code_book_4: [UWORD16; 116],
    pub huffman_code_book_5: [UWORD16; 126],
    pub huffman_code_book_6: [UWORD16; 120],
    pub huffman_code_book_7: [UWORD16; 112],
    pub huffman_code_book_8: [UWORD16; 92],
    pub huffman_code_book_9: [UWORD16; 236],
    pub huffman_code_book_10: [UWORD16; 218],
    pub huffman_codebook_11: [UWORD16; 344],
    pub huffman_code_book_scl: [UWORD16; 273],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_residual_aac_tables_struct {
    pub res_block_tables_ptr: *mut ia_mps_dec_res_block_tables_struct,
    pub res_huffmann_tables_ptr: *mut ia_mps_dec_res_huffmann_tables_struct,
    pub scale_factor_bands_long: [*mut WORD8; 24],
    pub scale_factor_bands_short: [*mut WORD8; 24],
    pub sfb_index_long: *mut WORD16,
    pub sfb_index_short: *mut WORD16,
    pub sfb_index_long_width: *mut WORD8,
    pub sfb_index_short_width: *mut WORD8,
    pub code_book: [*mut UWORD16; 13],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_residual_dynamic_data_struct {
    pub a_scale_factor: [WORD16; 128],
    pub a_code_book: [WORD8; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_residual_ics_info_struct {
    pub window_sequence: WORD16,
    pub max_sf_bands: WORD16,
    pub total_sf_bands: WORD16,
    pub sampling_rate_index: WORD16,
    pub window_groups: WORD16,
    pub window_group_length: [WORD8; 8],
    pub frame_length: WORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_residual_filter_struct {
    pub start_band: WORD16,
    pub stop_band: WORD16,
    pub direction: WORD8,
    pub resolution: WORD8,
    pub order: WORD8,
    pub coeff: [WORD8; 31],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_residual_tns_data {
    pub tns_data_present: FLAG,
    pub number_of_filters: [WORD8; 8],
    pub filter: [[ia_mps_dec_residual_filter_struct; 3]; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_residual_pulse_data_struct {
    pub pulse_data_present: FLAG,
    pub number_pulse: WORD16,
    pub pulse_start_band: WORD16,
    pub pulse_offset: [WORD8; 4],
    pub pulse_amp: [WORD8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_residual_pns_data_struct {
    pub pns_used: [UWORD8; 128],
    pub current_energy: WORD16,
    pub pns_active: UWORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_residual_channel_info_struct {
    pub p_scale_factor: *mut WORD16,
    pub p_code_book: *mut WORD8,
    pub p_spectral_coefficient: *mut WORD32,
    pub ics_info: ia_mps_dec_residual_ics_info_struct,
    pub tns_data: ia_mps_dec_residual_tns_data,
    pub pulse_data: ia_mps_dec_residual_pulse_data_struct,
    pub pns_data: ia_mps_dec_residual_pns_data_struct,
    pub common_window: WORD16,
    pub global_gain: WORD16,
    pub p_tns_scratch: *mut WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_residual_sfband_info_struct {
    pub sfb_long_idx: [WORD16; 52],
    pub sfb_short_idx: [WORD16; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_cmplx_flt_struct {
    pub re: FLOAT32,
    pub im: FLOAT32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_decor_filt_struct {
    pub state_len: WORD32,
    pub num_len: WORD32,
    pub den_len: WORD32,
    pub state: [ia_cmplx_flt_struct; 21],
    pub num: *const FLOAT32,
    pub den: *const FLOAT32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ixheaacd_mps_decor_energy_adjust_filt_struct {
    pub num_bins: WORD32,
    pub smooth_in_energy: [FLOAT32; 28],
    pub smooth_out_energy: [FLOAT32; 28],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_decor_struct {
    pub num_bins: WORD32,
    pub filter: [ia_mps_decor_filt_struct; 71],
    pub decor_nrg_smooth: ixheaacd_mps_decor_energy_adjust_filt_struct,
    pub delay_sample_count: [WORD32; 71],
    pub decor_delay_buffer: [[ia_cmplx_flt_struct; 86]; 71],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_hybrid_filt_struct {
    pub hf_buffer: [[ia_cmplx_flt_struct; 78]; 128],
    pub lf_buffer: [[ia_cmplx_flt_struct; 84]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_data_struct {
    pub bs_xxx_data_mode: [WORD32; 9],
    pub quant_coarse_xxx_flag: [WORD32; 9],
    pub bs_freq_res_stride_xxx: [WORD32; 9],
    pub bs_quant_coarse_xxx: [WORD8; 9],
    pub bs_quant_coarse_xxx_prev: WORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_bs_frame {
    pub independency_flag: WORD8,
    pub cld_idx: [[WORD32; 28]; 9],
    pub icc_idx: [[WORD32; 28]; 9],
    pub cld_idx_pre: [WORD32; 28],
    pub icc_idx_pre: [WORD32; 28],
    pub cmp_cld_idx: [[WORD32; 28]; 9],
    pub cmp_icc_idx: [[WORD32; 28]; 9],
    pub cmp_cld_idx_prev: [WORD32; 28],
    pub cmp_icc_idx_prev: [WORD32; 28],
    pub cld_data: ia_mps_data_struct,
    pub icc_data: ia_mps_data_struct,
    pub ipd_data: ia_mps_data_struct,
    pub ipd_idx_data: [[WORD32; 28]; 9],
    pub ipd_idx_data_prev: [WORD32; 28],
    pub ipd_idx: [[WORD32; 28]; 9],
    pub ipd_idx_prev: [WORD32; 28],
    pub bs_smooth_mode: [WORD32; 9],
    pub bs_smooth_time: [WORD32; 9],
    pub bs_freq_res_stride_smg: [WORD32; 9],
    pub bs_smg_data: [[WORD32; 28]; 9],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_smoothing_struct {
    pub prev_smg_time: WORD32,
    pub inv_prev_smg_time: FLOAT32,
    pub prev_smg_data: [WORD32; 28],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_env_reshape_struct {
    pub pb_energy_prev: [[FLOAT32; 28]; 3],
    pub avg_energy_prev: [FLOAT32; 3],
    pub frame_energy_prev: [FLOAT32; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_stp_struct {
    pub nrg_dir: FLOAT32,
    pub nrg_diff: [FLOAT32; 2],
    pub nrg_dir_prev: FLOAT32,
    pub nrg_diff_prev: [FLOAT32; 2],
    pub tp_scale_last: [FLOAT32; 2],
    pub init_flag: WORD32,
    pub update_old_ener: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_opd_smooth_struct {
    pub smooth_l_phase: [WORD32; 28],
    pub smooth_r_phase: [WORD32; 28],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_state_struct {
    pub in_ch_count: WORD32,
    pub out_ch_count: WORD32,
    pub input_gain: FLOAT32,
    pub dir_sig_count: WORD32,
    pub decor_sig_count: WORD32,
    pub time_slots: WORD32,
    pub pre_mix_req: WORD32,
    pub temp_shape_enable_ch_stp: [WORD32; 2],
    pub temp_shape_enable_ch_ges: [WORD32; 2],
    pub env_shape_data: [[FLOAT32; 72]; 2],
    pub parse_nxt_frame: WORD8,
    pub band_count: [WORD32; 2],
    pub synth_count: WORD32,
    pub qmf_band_count: WORD32,
    pub hyb_band_count: [WORD32; 2],
    pub hyb_band_count_max: WORD32,
    pub hyb_band_to_processing_band_table: *const WORD32,
    pub res_ch_count: WORD32,
    pub res_bands: WORD32,
    pub max_res_bands: WORD32,
    pub bs_param_bands: WORD32,
    pub ext_frame_flag: WORD32,
    pub num_parameter_sets: WORD32,
    pub num_parameter_sets_prev: WORD32,
    pub param_slots: [WORD32; 9],
    pub param_slot_diff: [WORD32; 9],
    pub inv_param_slot_diff: [FLOAT32; 9],
    pub inv_param_slot_diff_Q30: [WORD32; 9],
    pub frame_length: WORD32,
    pub residual_coding: WORD32,
    pub bs_residual_present: WORD32,
    pub bs_residual_bands: WORD32,
    pub config: *mut ia_usac_dec_mps_config_struct,
    pub ldmps_config: ia_usac_dec_mps_config_struct,
    pub bs_frame: ia_mps_bs_frame,
    pub smoothing_time: [WORD32; 9],
    pub inv_smoothing_time: [FLOAT32; 9],
    pub smoothing_data: [[WORD32; 28]; 9],
    pub bs_tsd_enable: WORD32,
    pub bs_tsd_sep_data: [WORD32; 72],
    pub bs_tsd_tr_phase_data: [WORD32; 72],
    pub tsd_num_tr_slots: WORD32,
    pub tsd_codeword_len: WORD32,
    pub cld_data: [[FLOAT32; 28]; 9],
    pub icc_data: [[FLOAT32; 28]; 9],
    pub ipd_data: [[FLOAT32; 28]; 9],
    pub bs_phase_mode: WORD32,
    pub opd_smoothing_mode: WORD32,
    pub num_bands_ipd: WORD32,
    pub phase_l: [[FLOAT32; 28]; 9],
    pub phase_r: [[FLOAT32; 28]; 9],
    pub phase_l_prev: [FLOAT32; 28],
    pub phase_r_prev: [FLOAT32; 28],
    pub m1_param_re: [[[[FLOAT32; 2]; 2]; 28]; 9],
    pub m1_param_im: [[[[FLOAT32; 2]; 2]; 28]; 9],
    pub m2_decor_re: [[[[FLOAT32; 2]; 2]; 28]; 9],
    pub m2_decor_im: [[[[FLOAT32; 2]; 2]; 28]; 9],
    pub m2_resid_re: [[[[FLOAT32; 2]; 2]; 28]; 9],
    pub m2_resid_im: [[[[FLOAT32; 2]; 2]; 28]; 9],
    pub m1_param_re_prev: [[[FLOAT32; 2]; 2]; 28],
    pub m1_param_im_prev: [[[FLOAT32; 2]; 2]; 28],
    pub m2_decor_re_prev: [[[FLOAT32; 2]; 2]; 28],
    pub m2_decor_im_prev: [[[FLOAT32; 2]; 2]; 28],
    pub m2_resid_re_prev: [[[FLOAT32; 2]; 2]; 28],
    pub m2_resid_im_prev: [[[FLOAT32; 2]; 2]; 28],
    pub qmf_in: [[[ia_cmplx_flt_struct; 72]; 64]; 2],
    pub hyb_in: [[[ia_cmplx_flt_struct; 72]; 71]; 2],
    pub hyb_res: [[ia_cmplx_flt_struct; 72]; 71],
    pub v: [[[ia_cmplx_flt_struct; 71]; 72]; 2],
    pub w_diff: [[[ia_cmplx_flt_struct; 71]; 72]; 2],
    pub w_dir: [[[ia_cmplx_flt_struct; 71]; 72]; 2],
    pub hyb_dir_out: [[[ia_cmplx_flt_struct; 71]; 72]; 2],
    pub hyb_diff_out: [[[ia_cmplx_flt_struct; 71]; 72]; 2],
    pub qmf_out_dir: [[[ia_cmplx_flt_struct; 128]; 72]; 2],
    pub scratch: [[ia_cmplx_flt_struct; 71]; 72],
    pub output_buffer: *mut [FLOAT32; 4096],
    pub hyb_filt_state: [ia_mps_hybrid_filt_struct; 2],
    pub qmf_filt_state: [[FLOAT32; 1280]; 2],
    pub mps_decor: ia_mps_decor_struct,
    pub smoothing_filt_state: ia_mps_smoothing_struct,
    pub guided_env_shaping: ia_mps_env_reshape_struct,
    pub bs_high_rate_mode: WORD32,
    pub tmp_buf: [FLOAT32; 10752],
    pub r_out_re_in_m1: [[[[FLOAT32; 2]; 2]; 28]; 72],
    pub r_out_im_in_m1: [[[[FLOAT32; 2]; 2]; 28]; 72],
    pub r_out_re_scratch_m1: [[[[WORD32; 2]; 2]; 28]; 72],
    pub r_out_im_scratch_m1: [[[[WORD32; 2]; 2]; 28]; 72],
    pub r_out_re_in_m2: [[[[FLOAT32; 2]; 2]; 28]; 72],
    pub r_out_im_in_m2: [[[[FLOAT32; 2]; 2]; 28]; 72],
    pub r_out_diff_re_in_m2: [[[[FLOAT32; 2]; 2]; 28]; 72],
    pub r_out_diff_im_in_m2: [[[[FLOAT32; 2]; 2]; 28]; 72],
    pub r_out_re_fix_in_m2: [[[[WORD32; 2]; 2]; 28]; 72],
    pub r_out_im_fix_in_m2: [[[[WORD32; 2]; 2]; 28]; 72],
    pub r_diff_out_re_fix_in_m2: [[[[WORD32; 2]; 2]; 28]; 72],
    pub r_diff_out_im_fix_in_m2: [[[[WORD32; 2]; 2]; 28]; 72],
    pub r_out_ph_re_in_m2: [[[FLOAT32; 2]; 28]; 72],
    pub r_out_ph_im_in_m2: [[[FLOAT32; 2]; 28]; 72],
    pub subband_var: ia_mps_stp_struct,
    pub opd_smooth: ia_mps_opd_smooth_struct,
    pub resolution: WORD32,
    pub p_sbr_dec: [*mut core::ffi::c_void; 2],
    pub p_sbr_frame: [*mut core::ffi::c_void; 2],
    pub p_sbr_header: [*mut core::ffi::c_void; 2],
    pub object_type: WORD32,
    pub mps_init_done: WORD32,
    pub str_mps_qmf_bank: ia_sbr_qmf_filter_bank_struct,
    pub qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
    pub sbr_tables_ptr: *mut ia_sbr_tables_struct,
    pub str_sbr_scale_fact: *mut ia_sbr_scale_fact_struct,
    pub ec_flag: WORD8,
    pub frame_ok: WORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_decorr_filter_instance_struct {
    pub state_length: WORD32,
    pub num_length: WORD32,
    pub den_length: WORD32,
    pub complex: WORD32,
    pub state_real: *mut WORD32,
    pub state_imag: *mut WORD32,
    pub numerator_real: *mut WORD32,
    pub numerator_imag: *mut WORD32,
    pub denominator_real: *mut WORD32,
    pub denominator_imag: *mut WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub decorr_seed: WORD32,
    pub numbins: WORD32,
    pub filter: [*mut ia_mps_dec_decorr_filter_instance_struct; 71],
    pub ducker: *mut ia_mps_dec_ducker_interface,
    pub no_sample_delay: [WORD32; 71],
    pub delay_buffer_real: *mut *mut WORD32,
    pub delay_buffer_imag: *mut *mut WORD32,
}
pub type ia_mps_dec_decorr_dec_handle = *mut C2RustUnnamed;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_thyb_filter_state_struct {
    pub buffer_lf_real: [[WORD32; 84]; 3],
    pub buffer_lf_imag: [[WORD32; 84]; 3],
    pub qmf_lf_real: [[WORD32; 84]; 3],
    pub qmf_lf_imag: [[WORD32; 84]; 3],
    pub buffer_hf_real: [[WORD32; 78]; 64],
    pub buffer_hf_imag: [[WORD32; 78]; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_spatial_bs_config_struct {
    pub ui_pcm_wdsz: UWORD32,
    pub ui_samp_freq: UWORD32,
    pub ui_in_channels: UWORD32,
    pub ui_out_channels: UWORD32,
    pub ui_channel_mask: WORD32,
    pub frame_ok: WORD32,
    pub ui_bs_is_buried: UWORD32,
    pub ui_dec_type: WORD32,
    pub ui_upmix_type: WORD32,
    pub ui_binaural_quality: WORD32,
    pub ui_hrtf_model: WORD32,
    pub ui_qmf_bands: UWORD32,
    pub bs_frame_length: WORD32,
    pub bs_sampling_freq_index: WORD32,
    pub bs_sampling_frequency: WORD32,
    pub bs_freq_res: WORD32,
    pub bs_tree_config: WORD32,
    pub bs_quant_mode: WORD32,
    pub bs_one_icc: WORD32,
    pub bs_arbitrary_downmix: WORD32,
    pub bs_residual_coding: WORD32,
    pub bs_smooth_config: WORD32,
    pub bs_fixed_gain_sur: WORD32,
    pub bs_fixed_gain_lfe: WORD32,
    pub bs_fixed_gain_dmx: WORD32,
    pub bs_matrix_mode: WORD32,
    pub bs_temp_shape_config: WORD32,
    pub bs_decorr_config: WORD32,
    pub bs_3d_audio_mode: WORD32,
    pub bs_3d_audio_hrtf_set: WORD32,
    pub bs_hrtf_freq_res: WORD32,
    pub hrtf_num_band: WORD32,
    pub bs_hrtf_num_chan: WORD32,
    pub bs_hrtf_asymmetric: WORD32,
    pub bs_hrtf_level_left: [[WORD32; 28]; 8],
    pub bs_hrtf_level_right: [[WORD32; 28]; 8],
    pub bs_hrtf_phase: [WORD32; 8],
    pub bs_hrtf_phase_lr: [[WORD32; 28]; 8],
    pub bs_ott_bands: [WORD32; 5],
    pub bs_ttt_dual_mode: [WORD32; 1],
    pub bs_ttt_mode_low: [WORD32; 1],
    pub bs_ttt_mode_high: [WORD32; 1],
    pub bs_ttt_bands_low: [WORD32; 1],
    pub bs_sac_ext_type: [WORD32; 8],
    pub sac_ext_cnt: WORD32,
    pub bs_residual_present: [WORD32; 10],
    pub bs_residual_sampling_freq_index: WORD32,
    pub bs_residual_frames_per_spatial_frame: WORD32,
    pub bs_residual_bands: [WORD32; 10],
    pub bs_arbitrary_downmix_residual_sampling_freq_index: WORD32,
    pub bs_arbitrary_downmix_residual_frames_per_spatial_frame: WORD32,
    pub bs_arbitrary_downmix_residual_bands: WORD32,
    pub bs_env_quant_mode: WORD32,
    pub arbitrary_tree: WORD32,
    pub num_out_chan_at: WORD32,
    pub num_ott_boxes_at: WORD32,
    pub bs_output_channel_pos_at: [WORD32; 8],
    pub bs_ott_box_present_at: [[WORD32; 7]; 8],
    pub bs_ott_default_cld_at: [WORD32; 56],
    pub bs_ott_mode_lfe_at: [WORD32; 56],
    pub bs_ott_bands_at: [WORD32; 56],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_lossless_data_struct {
    pub bs_xxx_data_mode: [[WORD32; 8]; 33],
    pub bs_quant_coarse_xxx: [[WORD32; 8]; 33],
    pub bs_freq_res_stride_xxx: [[WORD32; 8]; 33],
    pub bs_quant_coarse_xxx_prev: [WORD32; 33],
    pub no_cmp_quant_coarse_xxx: [[WORD32; 8]; 33],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RESIDUAL_FRAME_DATA {
    pub bs_icc_diff_present: [[WORD32; 8]; 10],
    pub bs_icc_diff: [[[WORD32; 28]; 8]; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_spatial_bs_frame_struct {
    pub bs_independency_flag: WORD32,
    pub ott_cld_idx: [[[WORD32; 28]; 8]; 33],
    pub ott_icc_idx: [[[WORD32; 28]; 8]; 5],
    pub ttt_cpc_1_idx: [[[WORD32; 28]; 8]; 1],
    pub ttt_cpc_2_idx: [[[WORD32; 28]; 8]; 1],
    pub ttt_cld_1_idx: [[[WORD32; 28]; 8]; 1],
    pub ttt_cld_2_idx: [[[WORD32; 28]; 8]; 1],
    pub ttt_icc_idx: [[[WORD32; 28]; 8]; 1],
    pub ott_cld_idx_prev: [[WORD32; 28]; 33],
    pub ott_icc_idx_prev: [[WORD32; 28]; 5],
    pub ttt_cpc_1_idx_prev: [[WORD32; 28]; 1],
    pub ttt_cpc_2_idx_prev: [[WORD32; 28]; 1],
    pub ttt_cld_1_idx_prev: [[WORD32; 28]; 1],
    pub ttt_cld_2_idx_prev: [[WORD32; 28]; 1],
    pub ttt_icc_idx_prev: [[WORD32; 28]; 1],
    pub cmp_ott_cld_idx: [[[WORD32; 28]; 8]; 33],
    pub cmp_ott_icc_idx: [[[WORD32; 28]; 8]; 5],
    pub ott_icc_diff_idx: [[[WORD32; 28]; 8]; 5],
    pub cmp_ttt_cpc_1_idx: [[[WORD32; 28]; 8]; 1],
    pub cmp_ttt_cpc_2_idx: [[[WORD32; 28]; 8]; 1],
    pub cmp_ttt_cld_1_idx: [[[WORD32; 28]; 8]; 1],
    pub cmp_ttt_cld_2_idx: [[[WORD32; 28]; 8]; 1],
    pub cmp_ttt_icc_idx: [[[WORD32; 28]; 8]; 1],
    pub cmp_ott_cld_idx_prev: [[WORD32; 28]; 33],
    pub cmp_ott_icc_idx_prev: [[WORD32; 28]; 5],
    pub cmp_ttt_cpc_1_idx_prev: [[WORD32; 28]; 1],
    pub cmp_ttt_cpc_2_idx_prev: [[WORD32; 28]; 1],
    pub cmp_ttt_cld_1_idx_prev: [[WORD32; 28]; 1],
    pub cmp_ttt_cld_2_idx_prev: [[WORD32; 28]; 1],
    pub cmp_ttt_icc_idx_prev: [[WORD32; 28]; 1],
    pub cld_lossless_data: ia_mps_dec_lossless_data_struct,
    pub icc_lossless_data: ia_mps_dec_lossless_data_struct,
    pub cpc_lossless_data: ia_mps_dec_lossless_data_struct,
    pub bs_smooth_control: WORD32,
    pub bs_smooth_mode: [WORD32; 8],
    pub bs_smooth_time: [WORD32; 8],
    pub bs_freq_res_stride_smg: [WORD32; 8],
    pub bs_smg_data: [[WORD32; 28]; 8],
    pub res_data: RESIDUAL_FRAME_DATA,
    pub arbdmx_gain_idx: [[[WORD32; 28]; 8]; 6],
    pub arbdmx_gain_idx_prev: [[WORD32; 28]; 6],
    pub cmp_arbdmx_gain_idx: [[[WORD32; 28]; 8]; 6],
    pub cmp_arbdmx_gain_idx_prev: [[WORD32; 28]; 6],
    pub bs_arbitrary_downmix_residual_abs: [WORD32; 6],
    pub bs_arbitrary_downmix_residual_alpha_update_set: [WORD32; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_tonality_state_struct {
    pub spec_prev_real: [WORD32; 512],
    pub spec_prev_imag: [WORD32; 512],
    pub p_cross_real: [WORD32; 512],
    pub p_cross_imag: [WORD32; 512],
    pub p_sum: [WORD32; 512],
    pub p_sum_prev: [WORD32; 512],
    pub buf_real: [[WORD32; 6]; 64],
    pub buf_imag: [[WORD32; 6]; 64],
    pub win_buf_real: [[WORD32; 16]; 64],
    pub win_buf_imag: [[WORD32; 16]; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_smoothing_state_struct {
    pub prev_smg_time: WORD32,
    pub prev_smg_data: [WORD32; 28],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_reshape_bb_env_state_struct {
    pub part_nrg_prev: [[WORD32; 28]; 22],
    pub norm_nrg_prev: [WORD32; 22],
    pub frame_nrg_prev: [WORD32; 22],
    pub q_part_nrg_prev: [[WORD16; 28]; 22],
    pub q_norm_nrg_prev: [WORD16; 22],
    pub q_frame_nrg_prev: [WORD16; 22],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_ttt_config_struct {
    pub use_ttt_decorr: WORD32,
    pub mode: WORD32,
    pub start_band: WORD32,
    pub stop_band: WORD32,
    pub bitstream_start_band: WORD32,
    pub bitstream_stop_band: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_blind_decoder_struct {
    pub excitation: [[WORD32; 28]; 3],
    pub filter_coeff: WORD32,
    pub q_excitation: [[WORD16; 28]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_subband_tp_params_struct {
    pub run_dry_ener: [WORD32; 6],
    pub run_wet_ener: [WORD32; 8],
    pub old_dry_ener: [WORD32; 6],
    pub old_wet_ener: [WORD32; 8],
    pub q_run_dry_ener: [WORD16; 6],
    pub q_run_wet_ener: [WORD16; 8],
    pub q_old_dry_ener: [WORD16; 6],
    pub q_old_wet_ener: [WORD16; 8],
    pub update_old_ener: WORD32,
    pub prev_tp_scale: [WORD32; 8],
    pub q_prev_tp_scale: [WORD16; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_persistent_mem {
    pub prev_gain_at: *mut WORD32,
    pub arbdmx_alpha_prev: *mut WORD32,
    pub m1_param_real_prev: *mut WORD32,
    pub m1_param_imag_prev: *mut WORD32,
    pub m2_decor_real_prev: *mut WORD32,
    pub m2_decor_imag_prev: *mut WORD32,
    pub m2_resid_real_prev: *mut WORD32,
    pub m2_resid_imag_prev: *mut WORD32,
    pub qmf_input_delay_real: *mut WORD32,
    pub qmf_input_delay_imag: *mut WORD32,
    pub ana_qmf_states_buffer: *mut WORD32,
    pub syn_qmf_states_buffer: *mut WORD32,
    pub decorr_ptr: *mut core::ffi::c_void,
    pub hyb_filter_state: *mut ia_mps_dec_thyb_filter_state_struct,
    pub ton_state: *mut ia_mps_dec_tonality_state_struct,
    pub smooth_state: *mut ia_mps_dec_smoothing_state_struct,
    pub reshape_bb_env_state: *mut ia_mps_dec_reshape_bb_env_state_struct,
    pub sub_band_params: *mut ia_mps_dec_subband_tp_params_struct,
    pub blind_decoder: *mut ia_mps_dec_blind_decoder_struct,
    pub p_bs_frame: *mut ia_mps_dec_spatial_bs_frame_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_mps_tables_struct {
    pub qmf_table_ptr: *mut ia_mps_dec_qmf_tables_struct,
    pub common_table_ptr: *mut ia_mps_dec_common_tables_struct,
    pub hybrid_table_ptr: *mut ia_mps_dec_hybrid_tables_struct,
    pub m1_m2_table_ptr: *mut ia_mps_dec_m1_m2_tables_struct,
    pub decor_table_ptr: *mut ia_mps_dec_decorr_tables_struct,
    pub tp_process_table_ptr: *mut ia_mps_dec_tp_process_tables_struct,
    pub mdct2qmf_table_ptr: *mut ia_mps_dec_mdct2qmf_table_struct,
    pub tonality_table_ptr: *mut ia_mps_dec_tonality_tables_struct,
    pub bitdec_table_ptr: *mut ia_mps_dec_bitdec_tables_struct,
    pub blind_table_ptr: *mut ia_mps_dec_blind_tables_struct,
    pub mdct2qmfcos_table_ptr: *mut ia_mps_dec_mdct2qmf_tables_struct,
    pub mdct2qmfcos_tab_ptr: *mut ia_mps_dec_mdct2qmf_cos_table_struct,
    pub aac_tab: *mut core::ffi::c_void,
    pub wf_tab_ptr: *mut ia_mps_dec_wf_ptr_table_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_synthesis_interface {
    pub syn_filter_bank: Option<
        unsafe extern "C" fn(
            *mut ia_mps_dec_qmf_syn_filter_bank,
            *mut WORD32,
            *mut WORD32,
            *mut WORD32,
            WORD32,
            WORD32,
            WORD32,
            *mut ia_mps_dec_qmf_tables_struct,
        ) -> VOID,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_m1_param_struct {
    pub m1_param_real: [[[[WORD32; 28]; 8]; 6]; 8],
    pub m1_param_imag: [[[[WORD32; 28]; 8]; 6]; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_reuse_array_struct {
    pub qmf_residual_real: *mut WORD32,
    pub qmf_residual_imag: *mut WORD32,
    pub qmf_residual_real_pre: *mut WORD32,
    pub qmf_residual_real_post: *mut WORD32,
    pub qmf_residual_imag_pre: *mut WORD32,
    pub qmf_residual_imag_post: *mut WORD32,
    pub res_mdct: *mut WORD32,
    pub time_out: *mut WORD32,
    pub x_real: *mut WORD32,
    pub x_imag: *mut WORD32,
    pub hyb_output_real_dry: *mut WORD32,
    pub hyb_output_imag_dry: *mut WORD32,
    pub env_dmx_0: *mut WORD32,
    pub env_dmx_1: *mut WORD32,
    pub m_qmf_real: *mut WORD32,
    pub m_qmf_imag: *mut WORD32,
    pub w_dry_real: *mut WORD32,
    pub w_dry_imag: *mut WORD32,
    pub buf_real: *mut WORD32,
    pub buf_imag: *mut WORD32,
    pub buffer_real: *mut WORD32,
    pub buffer_imag: *mut WORD32,
    pub m1_param: *mut ia_mps_dec_m1_param_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_m2_param_struct {
    pub m2_decor_real: [[[WORD32; 28]; 8]; 15],
    pub m2_decor_imag: [[[WORD32; 28]; 8]; 15],
    pub m2_resid_real: [[[WORD32; 28]; 8]; 19],
    pub m2_resid_imag: [[[WORD32; 28]; 8]; 19],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_auxilary_struct {
    pub m2_param: *mut ia_mps_dec_m2_param_struct,
    pub temp_shape_enable_channel_stp: [WORD32; 8],
    pub temp_shape_enable_channel_ges: [WORD32; 8],
    pub env_shape_data: [[WORD32; 72]; 8],
    pub num_ott_bands: [WORD32; 5],
    pub ttt_config: [[ia_mps_dec_ttt_config_struct; 1]; 2],
    pub param_slot: [WORD32; 8],
    pub smg_time: [WORD32; 8],
    pub smg_data: [[WORD32; 28]; 8],
    pub ott_cld: [[[WORD32; 28]; 8]; 33],
    pub ott_icc: [[[WORD32; 28]; 8]; 5],
    pub ttt_cpc_1: [[[WORD32; 28]; 8]; 1],
    pub ttt_cpc_2: [[[WORD32; 28]; 8]; 1],
    pub ttt_cld_1: [[[WORD32; 28]; 8]; 1],
    pub ttt_cld_2: [[[WORD32; 28]; 8]; 1],
    pub ttt_icc: [[[WORD32; 28]; 8]; 1],
    pub arbdmx_gain: [[[WORD32; 28]; 8]; 6],
    pub arbdmx_residual_abs: [WORD32; 6],
    pub arbdmx_alpha_upd_set: [WORD32; 6],
    pub arbdmx_alpha: [WORD32; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_heaac_mps_state_struct {
    pub sac_time_align_flag: WORD32,
    pub sac_time_align: WORD32,
    pub sampling_freq: WORD32,
    pub tree_config: WORD32,
    pub num_input_channels: WORD32,
    pub num_output_channels: WORD32,
    pub num_ott_boxes: WORD32,
    pub num_ttt_boxes: WORD32,
    pub num_output_channels_at: WORD32,
    pub quant_mode: WORD32,
    pub one_icc: WORD32,
    pub arbitrary_downmix: WORD32,
    pub residual_coding: WORD32,
    pub smooth_config: WORD32,
    pub temp_shape_config: WORD32,
    pub decorr_config: WORD32,
    pub mtx_inversion: WORD32,
    pub _3d_stereo_inversion: WORD32,
    pub env_quant_mode: WORD32,
    pub clip_protect_gain: WORD32,
    pub surround_gain: WORD32,
    pub lfe_gain: WORD32,
    pub cpc_default: WORD32,
    pub icc_default: WORD32,
    pub arbdmx_gain_default: WORD32,
    pub num_direct_signals: WORD32,
    pub num_residual_signals: WORD32,
    pub num_decor_signals: WORD32,
    pub num_v_channels: WORD32,
    pub num_w_channels: WORD32,
    pub w_start_residual_idx: WORD32,
    pub num_x_channels: WORD32,
    pub time_slots: WORD32,
    pub cur_time_slot: WORD32,
    pub frame_length: WORD32,
    pub dec_type: WORD32,
    pub up_mix_type: WORD32,
    pub binaural_quality: WORD32,
    pub hrtf_model: WORD32,
    pub tp_hyb_band_border: WORD32,
    pub parse_next_bitstream_frame: WORD32,
    pub qmf_bands: WORD32,
    pub hybrid_bands: WORD32,
    pub residual_frames_per_spatial_frame: WORD32,
    pub upd_qmf: WORD32,
    pub arbdmx_residual_bands: WORD32,
    pub arbdmx_frames_per_spatial_frame: WORD32,
    pub arbdmx_upd_qmf: WORD32,
    pub bitstream_parameter_bands: WORD32,
    pub num_parameter_bands: WORD32,
    pub extend_frame: WORD32,
    pub num_parameter_sets: WORD32,
    pub num_parameter_sets_prev: WORD32,
    pub smooth_control: WORD32,
    pub i_bytes_consumed_mps: WORD32,
    pub bytes_remaining: WORD32,
    pub ui_mps_in_bytes: WORD32,
    pub is_sbr_present: WORD32,
    pub bits_per_sample: WORD32,
    pub qmf_input_delay_index: WORD32,
    pub m1_param_imag_present: WORD32,
    pub m2_param_imag_present: WORD32,
    pub m1_param_present: [[WORD32; 6]; 8],
    pub m2_param_present: [[WORD32; 8]; 8],
    pub index: [WORD32; 10],
    pub ott_cld_default: [WORD32; 5],
    pub ttt_cld_1_default: [WORD32; 1],
    pub ttt_cld_2_default: [WORD32; 1],
    pub kernels: [size_t; 71],
    pub res_bands: [WORD32; 10],
    pub ott_mode_lfe: [WORD32; 5],
    pub bitstream_ott_bands: [WORD32; 5],
    pub scaling_enable: WORD32,
    pub is_buried_flag: WORD32,
    pub sfband_info_tab: ia_mps_dec_residual_sfband_info_struct,
    pub pcm_out_buf: *mut WORD16,
    pub res_block_type: [[WORD32; 4]; 10],
    pub bs_config: ia_mps_spatial_bs_config_struct,
    pub ap_decor: [ia_mps_dec_decorr_dec_handle; 5],
    pub qmf_bank: [ia_mps_dec_qmf_ana_filter_bank; 6],
    pub syn_qmf_bank: ia_mps_dec_qmf_syn_filter_bank,
    pub mps_bit_buf: ia_bit_buf_struct,
    pub ptr_mps_bit_buff: *mut ia_bit_buf_struct,
    pub bs_frame: *mut ia_mps_dec_spatial_bs_frame_struct,
    pub array_struct: *mut ia_mps_dec_reuse_array_struct,
    pub aux_struct: *mut ia_mps_dec_auxilary_struct,
    pub mps_scratch_mem_v: *mut core::ffi::c_void,
    pub mps_persistent_mem: ia_mps_persistent_mem,
    pub mps_persistent_mem_v: *mut core::ffi::c_void,
    pub syn: *mut ia_mps_dec_synthesis_interface,
    pub p_aac_decoder_channel_info: [*mut ia_mps_dec_residual_channel_info_struct; 2],
    pub p_aac_decoder_dynamic_data_init: [*mut ia_mps_dec_residual_dynamic_data_struct; 2],
    pub tot_sf_bands_ls: [WORD8; 2],
    pub ia_mps_dec_mps_table: ia_mps_dec_mps_tables_struct,
    pub aac_table: ia_mps_dec_residual_aac_tables_struct,
    pub ia_mps_dec_mdct2qmfcos_table: ia_mps_dec_mdct2qmf_cos_table_struct,
    pub wf_tab: ia_mps_dec_wf_ptr_table_struct,
    pub is_first: WORD32,
    pub mps_decode: WORD32,
    pub temp_buf: [UWORD8; 1024],
    pub heaac_mps_present: WORD32,
    pub mps_with_sbr: WORD32,
    pub mps_init_done: WORD32,
    pub ec_flag: WORD32,
    pub frame_ok: WORD32,
    pub first_frame: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_peak_limiter_struct {
    pub attack_time: FLOAT32,
    pub release_time: FLOAT32,
    pub attack_constant: FLOAT32,
    pub release_constant: FLOAT32,
    pub limit_threshold: FLOAT32,
    pub num_channels: UWORD32,
    pub sample_rate: UWORD32,
    pub attack_time_samples: UWORD32,
    pub limiter_on: UWORD32,
    pub gain_modified: FLOAT32,
    pub pre_smoothed_gain: FLOAT64,
    pub delayed_input: *mut FLOAT32,
    pub delayed_input_index: UWORD32,
    pub max_buf: *mut FLOAT32,
    pub min_gain: FLOAT32,
    pub buffer: [FLOAT32; 16384],
    pub max_idx: WORD32,
    pub cir_buf_pnt: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_program_config_struct {
    pub element_instance_tag: WORD8,
    pub object_type: WORD32,
    pub samp_freq_index: WORD32,
    pub num_front_channel_elements: WORD32,
    pub num_side_channel_elements: WORD32,
    pub num_back_channel_elements: WORD32,
    pub num_lfe_channel_elements: WORD32,
    pub num_assoc_data_elements: WORD32,
    pub num_valid_cc_elements: WORD32,
    pub front_element_is_cpe: [WORD8; 16],
    pub front_element_tag_select: [WORD8; 16],
    pub side_element_is_cpe: [WORD8; 16],
    pub side_element_tag_select: [WORD8; 16],
    pub back_element_is_cpe: [WORD8; 16],
    pub back_element_tag_select: [WORD8; 16],
    pub lfe_element_tag_select: [WORD8; 16],
    pub channels: WORD32,
    pub alignment_bits: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_enhaacplus_dec_ind_cc {
    pub cc_target_is_cpe: [WORD8; 10],
    pub cc_target_tag_select: [WORD8; 10],
    pub cc_l: [WORD8; 10],
    pub cc_r: [WORD8; 10],
    pub cc_gain: [WORD32; 20],
    pub elements_coupled: [WORD8; 10],
    pub num_coupled_elements: WORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_config_struct {
    pub ui_pcm_wdsz: UWORD32,
    pub ui_samp_freq: UWORD32,
    pub ui_n_channels: UWORD32,
    pub i_channel_mask: WORD32,
    pub ui_channel_mode: UWORD32,
    pub ui_sbr_mode: UWORD32,
    pub ui_effect_type: WORD32,
    pub ui_target_loudness: WORD32,
    pub ui_loud_norm_flag: WORD32,
    pub flag_downmix: UWORD32,
    pub flag_08khz_out: UWORD32,
    pub flag_16khz_out: UWORD32,
    pub flag_to_stereo: UWORD32,
    pub down_sample_flag: UWORD32,
    pub header_dec_done: UWORD32,
    pub ui_mp4_flag: UWORD32,
    pub ui_disable_sync: UWORD32,
    pub ui_auto_sbr_upsample: UWORD32,
    pub frame_status: WORD32,
    pub ui_max_channels: UWORD32,
    pub ui_pce_found_in_hdr: UWORD32,
    pub ui_n_memtabs: UWORD32,
    pub ui_drc_boost: WORD32,
    pub ui_drc_cut: WORD32,
    pub ui_drc_target_level: WORD32,
    pub ui_drc_set: WORD32,
    pub ui_drc_heavy_comp: WORD32,
    pub str_prog_config: ia_program_config_struct,
    pub element_type: [WORD32; 11],
    pub slot_element: [WORD32; 11],
    pub element_instance_order: [WORD; 10],
    pub ui_coupling_channel: WORD,
    pub downmix: WORD,
    pub loas_present: WORD32,
    pub framesize_480: WORD,
    pub ld_decoder: WORD,
    pub eld_sbr_present: WORD,
    pub ui_out_channels: UWORD32,
    pub ui_channel_mask: WORD32,
    pub ui_dec_type: WORD32,
    pub ui_qmf_bands: UWORD32,
    pub ui_flush_cmd: WORD32,
    pub drc_config_struct: ia_drc_config,
    pub output_level: WORD32,
    pub i_loud_ref_level: WORD32,
    pub dup_stereo_flag: UWORD8,
    pub peak_limiter_off: WORD32,
    pub mps_present: WORD32,
    pub ui_frame_size: UWORD32,
    pub ui_enh_sbr: WORD32,
    pub ui_hq_esbr: WORD32,
    pub ui_enh_sbr_ps: WORD32,
    pub ui_usac_flag: WORD32,
    pub ui_err_conceal: WORD32,
    pub first_frame: FLAG,
    pub ui_loudness_leveling_flag: WORD32,
    pub ui_drc_mode_cut: UWORD8,
    pub ui_drc_mode_boost: UWORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_state_struct {
    pub p_config: *mut ia_aac_dec_config_struct,
    pub audio_object_type: AUDIO_OBJECT_TYPE,
    pub ui_in_bytes: UWORD32,
    pub ui_out_bytes: UWORD32,
    pub ui_exec_done: UWORD32,
    pub b_n_raw_data_blk: WORD16,
    pub s_adts_hdr_present: WORD16,
    pub s_adif_hdr_present: WORD16,
    pub num_channel_last: WORD16,
    pub sampling_rate: UWORD32,
    pub extension_samp_rate: UWORD32,
    pub bit_rate: UWORD32,
    pub ui_init_done: UWORD32,
    pub ui_input_over: UWORD32,
    pub header_dec_done: UWORD32,
    pub frame_counter: WORD32,
    pub pstr_aac_dec_info: [*mut ia_aac_decoder_struct; 10],
    pub ch_config: UWORD32,
    pub str_bit_buf: ia_bit_buf_struct,
    pub pstr_bit_buf: *mut ia_bit_buf_struct,
    pub pstr_stream_sbr: *mut [ia_aac_dec_sbr_bitstream_struct; 2],
    pub str_sbr_dec_info: [ia_handle_sbr_dec_inst_struct; 10],
    pub sbr_present_flag: WORD32,
    pub ps_present: WORD32,
    pub ptr_bit_stream: *mut ia_bit_buf_struct,
    pub aac_scratch_mem_v: *mut core::ffi::c_void,
    pub aac_persistent_mem_v: *mut core::ffi::c_void,
    pub sbr_persistent_mem_v: *mut core::ffi::c_void,
    pub ptr_overlap_buf: *mut WORD32,
    pub num_of_out_samples: WORD16,
    pub last_frame_ok: WORD32,
    pub i_bytes_consumed: WORD32,
    pub coup_ch_output: *mut WORD32,
    pub ind_cc_info: ia_enhaacplus_dec_ind_cc,
    pub protection_absent: WORD8,
    pub crc_check: WORD32,
    pub ui_flush_cmd: WORD32,
    pub frame_len_flag: WORD32,
    pub depends_on_core_coder: WORD32,
    pub extension_flag: WORD32,
    pub bs_format: WORD32,
    pub bit_count: WORD32,
    pub sync_status: WORD32,
    pub extension_flag_3: WORD32,
    pub latm_struct_element: ixheaacd_latm_struct,
    pub latm_initialized: WORD32,
    pub str_drc_dec_info: ia_drc_dec_struct,
    pub drc_dummy: ia_drc_dec_struct,
    pub pstr_drc_dec: *mut ia_drc_dec_struct,
    pub pstr_hdrc_data: [*mut ixheaac_drc_data_struct; 30],
    pub prev_channel_mode: WORD32,
    pub drc_cut_fac: WORD32,
    pub drc_boost_fac: WORD32,
    pub first_drc_frame: WORD32,
    pub str_err_config: ia_aac_err_config_struct,
    pub frame_size: WORD32,
    pub frame_length: WORD32,
    pub dwnsmp_signal: WORD32,
    pub eld_specific_config: ia_eld_specific_config_struct,
    pub usac_flag: FLAG,
    pub num_of_output_ch: WORD32,
    pub ia_audio_specific_config: *mut core::ffi::c_void,
    pub mps_dec_handle: ia_mps_dec_state_struct,
    pub heaac_mps_handle: ia_heaac_mps_state_struct,
    pub mps_buffer: [UWORD8; 1024],
    pub huffman_code_book_scl: *mut UWORD16,
    pub huffman_code_book_scl_index: *mut UWORD32,
    pub pstr_aac_tables: *mut ia_aac_dec_tables_struct,
    pub pstr_dec_data: *mut core::ffi::c_void,
    pub sbr_persistent_mem_u: *mut core::ffi::c_void,
    pub sbr_scratch_mem_u: *mut core::ffi::c_void,
    pub header_ptr: *mut UWORD8,
    pub header_length: WORD32,
    pub str_sbr_config: ia_sbr_header_data_struct,
    pub xaac_jmp_buf: jmp_buf,
    pub decode_create_done: WORD32,
    pub ldmps_present: WORD32,
    pub fatal_err_present: WORD32,
    pub pers_mem_ptr: *mut WORD8,
    pub preroll_config_present: UWORD8,
    pub preroll_config_prev: [UWORD8; 285],
    pub qshift_cnt: UWORD8,
    pub qshift_adj: [WORD8; 16],
    pub delay_in_samples: UWORD32,
    pub peak_lim_init: UWORD8,
    pub peak_limiter: ia_peak_limiter_struct,
    pub sbr_present: UWORD8,
    pub slot_pos: UWORD8,
    pub mps_header: WORD32,
    pub ui_mps_out_bytes: WORD32,
    pub drc_config_changed: WORD32,
    pub apply_crossfade: WORD32,
    pub ec_enable: WORD32,
    pub first_frame: WORD32,
}
pub const MAX_32: WORD32 = 0x7fffffff as core::ffi::c_long as WORD32;
pub const MIN_32: WORD32 = 0x80000000 as core::ffi::c_long as WORD32;
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
pub const IA_XHEAAC_DEC_API_NONFATAL_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_XHEAAC_DEC_INIT_FATAL_DEC_INIT_FAIL: core::ffi::c_uint = 0xffff9000
    as core::ffi::c_uint;
pub const IA_XHEAAC_DEC_EXE_FATAL_INVALID_LOAS_HEADER: core::ffi::c_uint = 0xffff9802
    as core::ffi::c_uint;
pub const IA_FATAL_ERROR: core::ffi::c_uint = 0x80000000 as core::ffi::c_uint;
pub const LATM_MAX_PROG: core::ffi::c_int = 8 as core::ffi::c_int;
pub const AAC_DEC_OK: core::ffi::c_int = IA_XHEAAC_DEC_API_NONFATAL_NO_ERROR;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_latm_au_chunk_length_info(
    mut it_bit_buff: *mut ia_bit_buf_struct,
) -> WORD32 {
    let mut reading_done: UWORD8 = 0;
    let mut len: WORD32 = 0 as WORD32;
    loop {
        let mut tmp: UWORD32 = ixheaacd_read_bits_buf(
            it_bit_buff as *mut ia_bit_buf_struct,
            8 as WORD,
        ) as UWORD32;
        reading_done = (tmp < 255 as UWORD32) as core::ffi::c_int as UWORD8;
        len = (len as UWORD32).wrapping_add(tmp) as WORD32 as WORD32;
        if !(reading_done as core::ffi::c_int == 0 as core::ffi::c_int) {
            break;
        }
    }
    len <<= 3 as core::ffi::c_int;
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_latm_payload_length_info(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut latm_element: *mut ixheaacd_latm_struct,
) -> WORD32 {
    let mut error_code: WORD32 = AAC_DEC_OK;
    let mut prog: UWORD32 = 0;
    let mut lay: UWORD32 = 0;
    if (*latm_element).all_streams_same_time_framing == 1 as UWORD32 {
        prog = 0 as UWORD32;
        while prog < (*latm_element).num_program {
            lay = 0 as UWORD32;
            while lay < (*latm_element).num_layer {
                let mut layer_info: *mut ixheaacd_latm_layer_info = &mut *(*((*latm_element)
                    .layer_info)
                    .as_mut_ptr()
                    .offset(prog as isize))
                    .as_mut_ptr()
                    .offset(lay as isize) as *mut ixheaacd_latm_layer_info;
                match (*layer_info).frame_len_type {
                    0 => {
                        (*layer_info).frame_len_bits = ixheaacd_latm_au_chunk_length_info(
                            it_bit_buff,
                        ) as UWORD32;
                        if ((*layer_info).frame_len_bits).wrapping_rem(8 as UWORD32)
                            != 0 as UWORD32
                        {
                            error_code = IA_XHEAAC_DEC_EXE_FATAL_INVALID_LOAS_HEADER
                                as WORD32;
                            return error_code;
                        }
                        (*latm_element).frame_length = (*layer_info).frame_len_bits
                            >> 3 as core::ffi::c_int;
                        (*latm_element).frame_length = ((*latm_element).frame_length)
                            .wrapping_add(
                                ((*it_bit_buff).size - (*it_bit_buff).cnt_bits
                                    >> 3 as core::ffi::c_int) as UWORD32,
                            );
                    }
                    _ => {
                        error_code = IA_XHEAAC_DEC_EXE_FATAL_INVALID_LOAS_HEADER
                            as WORD32;
                        return error_code;
                    }
                }
                lay = lay.wrapping_add(1);
            }
            prog = prog.wrapping_add(1);
        }
    } else {
        error_code = IA_XHEAAC_DEC_EXE_FATAL_INVALID_LOAS_HEADER as WORD32;
        return error_code;
    }
    return error_code;
}
unsafe extern "C" fn ixheaacd_latm_get_value(
    mut it_bit_buff: *mut ia_bit_buf_struct,
) -> UWORD32 {
    let mut bytes_read: UWORD32 = 0;
    bytes_read = (ixheaacd_read_bits_buf(it_bit_buff, 2 as WORD) as core::ffi::c_int
        + 1 as core::ffi::c_int) as UWORD32;
    if bytes_read <= 3 as UWORD32 {
        return ixheaacd_read_bits_buf(
            it_bit_buff,
            (8 as UWORD32).wrapping_mul(bytes_read) as WORD,
        ) as UWORD32
    } else {
        return ixheaac_add32_sat(
            ixheaac_shl32_sat(
                ixheaacd_read_bits_buf(it_bit_buff, 24 as WORD),
                8 as WORD,
            ),
            ixheaacd_read_bits_buf(it_bit_buff, 8 as WORD),
        ) as UWORD32
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_latm_stream_mux_config(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut latm_element: *mut ixheaacd_latm_struct,
    mut aac_state_struct: *mut ia_aac_dec_state_struct,
    mut sample_rate_info: *mut ia_sampling_rate_info_struct,
) -> IA_ERRORCODE {
    let mut prog: UWORD32 = 0;
    let mut lay: UWORD32 = 0;
    let mut bytes_consumed: WORD32 = 0;
    let mut audio_mux_version_a: WORD32 = 0;
    let mut error_code: IA_ERRORCODE = AAC_DEC_OK;
    let mut layer_info: *mut ixheaacd_latm_layer_info = 0
        as *mut ixheaacd_latm_layer_info;
    (*latm_element).audio_mux_version = ixheaacd_read_bits_buf(
        it_bit_buff as *mut ia_bit_buf_struct,
        1 as WORD,
    ) as UWORD32;
    if (*latm_element).audio_mux_version == 1 as UWORD32 {
        audio_mux_version_a = ixheaacd_read_bits_buf(
            it_bit_buff as *mut ia_bit_buf_struct,
            1 as WORD,
        );
    } else {
        audio_mux_version_a = 0 as core::ffi::c_int as WORD32;
    }
    if audio_mux_version_a == 0 as core::ffi::c_int {
        if (*latm_element).audio_mux_version == 1 as UWORD32 {
            ixheaacd_latm_get_value(it_bit_buff as *mut ia_bit_buf_struct);
        }
        (*latm_element).all_streams_same_time_framing = ixheaacd_read_bits_buf(
            it_bit_buff as *mut ia_bit_buf_struct,
            1 as WORD,
        ) as UWORD32;
        (*latm_element).num_sub_frames = (ixheaacd_read_bits_buf(
            it_bit_buff as *mut ia_bit_buf_struct,
            6 as WORD,
        ) as core::ffi::c_int + 1 as core::ffi::c_int) as UWORD32;
        if (*latm_element).num_sub_frames != 1 as UWORD32 {
            return IA_XHEAAC_DEC_EXE_FATAL_INVALID_LOAS_HEADER as IA_ERRORCODE;
        }
        (*latm_element).num_program = (ixheaacd_read_bits_buf(
            it_bit_buff as *mut ia_bit_buf_struct,
            4 as WORD,
        ) as core::ffi::c_int + 1 as core::ffi::c_int) as UWORD32;
        if (*latm_element).num_program > LATM_MAX_PROG as UWORD32 {
            return IA_FATAL_ERROR as IA_ERRORCODE;
        }
        prog = 0 as UWORD32;
        while prog < (*latm_element).num_program {
            (*latm_element).num_layer = (ixheaacd_read_bits_buf(
                it_bit_buff as *mut ia_bit_buf_struct,
                3 as WORD,
            ) as core::ffi::c_int + 1 as core::ffi::c_int) as UWORD32;
            lay = 0 as UWORD32;
            while lay < (*latm_element).num_layer {
                layer_info = &mut *(*((*latm_element).layer_info)
                    .as_mut_ptr()
                    .offset(prog as isize))
                    .as_mut_ptr()
                    .offset(lay as isize) as *mut ixheaacd_latm_layer_info;
                (*layer_info).frame_len_bits = 0 as UWORD32;
                if prog == 0 as UWORD32 && lay == 0 as UWORD32 {
                    let mut asc_len: WORD32 = 0;
                    let mut pos: WORD32 = 0;
                    (*latm_element).use_same_config = 0 as UWORD32;
                    asc_len = (if (*latm_element).audio_mux_version == 1 as UWORD32 {
                        ixheaacd_latm_get_value(it_bit_buff as *mut ia_bit_buf_struct)
                    } else {
                        0 as UWORD32
                    }) as WORD32;
                    pos = (*it_bit_buff).size - (*it_bit_buff).cnt_bits;
                    if asc_len
                        > (*it_bit_buff).size as core::ffi::c_int
                            - 106 as core::ffi::c_int
                        || asc_len > 2592 as core::ffi::c_int
                        || asc_len < 0 as core::ffi::c_int
                    {
                        return IA_XHEAAC_DEC_INIT_FATAL_DEC_INIT_FAIL as IA_ERRORCODE;
                    }
                    error_code = ixheaacd_ga_hdr_dec(
                        aac_state_struct,
                        (*it_bit_buff).cnt_bits,
                        &mut bytes_consumed,
                        sample_rate_info,
                        it_bit_buff,
                    ) as IA_ERRORCODE;
                    if error_code != 0 {
                        return error_code;
                    }
                    if asc_len != 0 {
                        asc_len -= (*it_bit_buff).size - (*it_bit_buff).cnt_bits - pos;
                        ixheaacd_read_bidirection(
                            it_bit_buff as *mut ia_bit_buf_struct,
                            asc_len,
                        );
                    }
                    (*layer_info).asc.aot = (*aac_state_struct).audio_object_type;
                    (*layer_info).asc.channel_config = (*aac_state_struct).ch_config
                        as WORD32;
                    (*layer_info).asc.samples_per_frame = (*aac_state_struct)
                        .frame_len_flag as UWORD32;
                    (*layer_info).asc.sampling_freq = (*aac_state_struct).sampling_rate;
                    (*layer_info).asc.samples_per_frame = (*aac_state_struct)
                        .frame_length as UWORD32;
                } else {
                    (*latm_element).use_same_config = ixheaacd_read_bits_buf(
                        it_bit_buff as *mut ia_bit_buf_struct,
                        1 as WORD,
                    ) as UWORD32;
                    if (*latm_element).use_same_config != 0 && lay > 0 as UWORD32 {
                        (*layer_info).asc = (*latm_element)
                            .layer_info[prog
                                as usize][lay.wrapping_sub(1 as UWORD32) as usize]
                            .asc;
                    } else {
                        error_code = ixheaacd_ga_hdr_dec(
                            aac_state_struct,
                            (*it_bit_buff).cnt_bits,
                            &mut bytes_consumed,
                            sample_rate_info,
                            it_bit_buff,
                        ) as IA_ERRORCODE;
                        if error_code != 0 {
                            return error_code;
                        }
                    }
                }
                (*layer_info).frame_len_type = ixheaacd_read_bits_buf(
                    it_bit_buff as *mut ia_bit_buf_struct,
                    3 as WORD,
                ) as UWORD32;
                's_234: {
                    match (*layer_info).frame_len_type {
                        0 => {
                            (*layer_info).buffer_fullness = ixheaacd_read_bits_buf(
                                it_bit_buff as *mut ia_bit_buf_struct,
                                8 as WORD,
                            ) as UWORD32;
                            if (*latm_element).all_streams_same_time_framing == 0 {
                                lay > 0 as UWORD32
                                    && (*layer_info).asc.aot as core::ffi::c_uint
                                        == AOT_AAC_SCAL as core::ffi::c_int as core::ffi::c_uint;
                            }
                            break 's_234;
                        }
                        1 => {
                            (*latm_element).frame_length = ixheaacd_read_bits_buf(
                                it_bit_buff as *mut ia_bit_buf_struct,
                                9 as WORD,
                            ) as UWORD32;
                        }
                        _ => {}
                    }
                    return IA_XHEAAC_DEC_EXE_FATAL_INVALID_LOAS_HEADER as IA_ERRORCODE;
                }
                lay = lay.wrapping_add(1);
            }
            prog = prog.wrapping_add(1);
        }
        (*latm_element).other_data_present = ixheaacd_read_bits_buf(
            it_bit_buff as *mut ia_bit_buf_struct,
            1 as WORD,
        ) as UWORD32;
        if (*latm_element).other_data_present != 0 {
            if (*latm_element).audio_mux_version == 1 as UWORD32 {
                (*latm_element).other_data_length = ixheaacd_latm_get_value(
                    it_bit_buff as *mut ia_bit_buf_struct,
                );
            } else {
                let mut other_data_len: UWORD32 = 0;
                (*latm_element).other_data_length = 0 as UWORD32;
                loop {
                    other_data_len = ixheaacd_read_bits_buf(
                        it_bit_buff as *mut ia_bit_buf_struct,
                        1 as WORD,
                    ) as UWORD32;
                    (*latm_element).other_data_length <<= 8 as core::ffi::c_int;
                    (*latm_element).other_data_length = ((*latm_element)
                        .other_data_length)
                        .wrapping_add(
                            ixheaacd_read_bits_buf(
                                it_bit_buff as *mut ia_bit_buf_struct,
                                8 as WORD,
                            ) as UWORD32,
                        );
                    if (*latm_element).other_data_length
                        > (*it_bit_buff).cnt_bits as UWORD32
                    {
                        return IA_FATAL_ERROR as IA_ERRORCODE;
                    }
                    if !(other_data_len != 0) {
                        break;
                    }
                }
            }
        }
        (*latm_element).crc_check_present = ixheaacd_read_bits_buf(
            it_bit_buff as *mut ia_bit_buf_struct,
            1 as WORD,
        ) as UWORD32;
        if (*latm_element).crc_check_present != 0 {
            (*latm_element).crc_check_sum = ixheaacd_read_bits_buf(
                it_bit_buff as *mut ia_bit_buf_struct,
                8 as WORD,
            ) as UWORD32;
        }
    } else {
        error_code = IA_XHEAAC_DEC_EXE_FATAL_INVALID_LOAS_HEADER as IA_ERRORCODE;
    }
    return error_code;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_latm_audio_mux_element(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut latm_element: *mut ixheaacd_latm_struct,
    mut aac_state_struct: *mut ia_aac_dec_state_struct,
    mut sample_rate_info: *mut ia_sampling_rate_info_struct,
) -> WORD32 {
    let mut i: UWORD32 = 0;
    let mut error_code: IA_ERRORCODE = AAC_DEC_OK;
    ixheaacd_read_bits_buf(it_bit_buff as *mut ia_bit_buf_struct, 13 as WORD);
    (*latm_element).use_same_stream_mux = ixheaacd_read_bits_buf(
        it_bit_buff as *mut ia_bit_buf_struct,
        1 as WORD,
    ) as UWORD32;
    if (*latm_element).use_same_stream_mux == 0 {
        error_code = ixheaacd_latm_stream_mux_config(
            it_bit_buff,
            latm_element,
            aac_state_struct,
            sample_rate_info,
        );
        if error_code != 0 {
            return error_code as WORD32;
        }
    }
    i = 0 as UWORD32;
    while i < (*latm_element).num_sub_frames {
        error_code = ixheaacd_latm_payload_length_info(it_bit_buff, latm_element)
            as IA_ERRORCODE;
        if error_code != 0 {
            if error_code != 0 as core::ffi::c_int {
                return error_code as WORD32;
            }
        }
        i = i.wrapping_add(1);
    }
    return error_code as WORD32;
}
