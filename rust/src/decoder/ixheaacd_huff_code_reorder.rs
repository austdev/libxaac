extern "C" {
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
    fn ixheaacd_aac_read_bit_rev(it_bit_buff: *mut ia_bit_buf_struct) -> WORD32;
    fn ixheaacd_aac_read_bit(it_bit_buff: *mut ia_bit_buf_struct) -> WORD32;
    fn ixheaacd_write_bit(
        it_bit_buff: *mut ia_bit_buf_struct,
        value: WORD32,
        no_of_bits: WORD32,
    ) -> VOID;
    fn ixheaacd_read_bit(data: *mut ia_bit_buf_struct, no_of_bits: WORD32) -> WORD32;
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
    fn ixheaacd_huff_sfb_table(
        it_bit_buff: WORD32,
        huff_index: *mut WORD16,
        len: *mut WORD32,
        code_book_tbl: *const UWORD16,
        idx_table: *const UWORD32,
    ) -> VOID;
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
pub type ia_pcw_type_struct = core::ffi::c_uint;
pub const PCW_ESC_SIGN: ia_pcw_type_struct = 2;
pub const PCW_SIGN: ia_pcw_type_struct = 1;
pub const PCW: ia_pcw_type_struct = 0;
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
unsafe extern "C" fn ixheaac_abs32(mut a: WORD32) -> WORD32 {
    let mut abs_val: WORD32 = 0;
    abs_val = a;
    if a < 0 as core::ffi::c_int {
        abs_val = -a;
    }
    return abs_val;
}
#[inline]
unsafe extern "C" fn ixheaac_extu(
    mut a: UWORD32,
    mut shift_left: WORD32,
    mut shift_right: WORD32,
) -> UWORD32 {
    let mut x: UWORD32 = 0;
    x = a << shift_left;
    x = x >> shift_right;
    return x;
}
pub const ZERO_HCB: core::ffi::c_int = 0 as core::ffi::c_int;
pub const ESC_HCB: core::ffi::c_int = 11 as core::ffi::c_int;
pub const NOISE_HCB: core::ffi::c_int = 13 as core::ffi::c_int;
pub const INTENSITY_HCB2: core::ffi::c_int = 14 as core::ffi::c_int;
pub const INTENSITY_HCB: core::ffi::c_int = 15 as core::ffi::c_int;
pub const EIGHT_SHORT_SEQUENCE: core::ffi::c_int = 2 as core::ffi::c_int;
pub const LINES_PER_UNIT: core::ffi::c_int = 4 as core::ffi::c_int;
pub const MAX_SFB_HCR: core::ffi::c_int = 1024 as core::ffi::c_int
    / 8 as core::ffi::c_int / LINES_PER_UNIT * 8 as core::ffi::c_int;
pub const NUMBER_OF_UNIT_GROUPS: core::ffi::c_int = LINES_PER_UNIT
    * 8 as core::ffi::c_int;
pub const LINES_PER_UNIT_GROUP: core::ffi::c_int = 1024 as core::ffi::c_int
    / NUMBER_OF_UNIT_GROUPS;
pub const FROM_LEFT_TO_RIGHT: core::ffi::c_int = 0 as core::ffi::c_int;
pub const FROM_RIGHT_TO_LEFT: core::ffi::c_int = 1 as core::ffi::c_int;
pub const MAX_HCR_SETS: core::ffi::c_int = 14 as core::ffi::c_int;
pub const MAX_CB_CHECK: core::ffi::c_int = 32 as core::ffi::c_int;
pub const ERROR_POS: core::ffi::c_int = 0x1 as core::ffi::c_int;
pub const HCR_FATAL_PCW_ERROR_MASK: core::ffi::c_int = 0x100e01fc as core::ffi::c_int;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[no_mangle]
pub static mut ixheaacd_min_huff_cb_pair_tbl: [UWORD8; 23] = [
    0 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    3 as core::ffi::c_int as UWORD8,
    5 as core::ffi::c_int as UWORD8,
    7 as core::ffi::c_int as UWORD8,
    9 as core::ffi::c_int as UWORD8,
    16 as core::ffi::c_int as UWORD8,
    17 as core::ffi::c_int as UWORD8,
    18 as core::ffi::c_int as UWORD8,
    19 as core::ffi::c_int as UWORD8,
    20 as core::ffi::c_int as UWORD8,
    21 as core::ffi::c_int as UWORD8,
    22 as core::ffi::c_int as UWORD8,
    23 as core::ffi::c_int as UWORD8,
    24 as core::ffi::c_int as UWORD8,
    25 as core::ffi::c_int as UWORD8,
    26 as core::ffi::c_int as UWORD8,
    27 as core::ffi::c_int as UWORD8,
    28 as core::ffi::c_int as UWORD8,
    29 as core::ffi::c_int as UWORD8,
    30 as core::ffi::c_int as UWORD8,
    31 as core::ffi::c_int as UWORD8,
    11 as core::ffi::c_int as UWORD8,
];
#[no_mangle]
pub static mut ixheaacd_max_huff_cb_pair_table: [UWORD8; 23] = [
    0 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    4 as core::ffi::c_int as UWORD8,
    6 as core::ffi::c_int as UWORD8,
    8 as core::ffi::c_int as UWORD8,
    10 as core::ffi::c_int as UWORD8,
    16 as core::ffi::c_int as UWORD8,
    17 as core::ffi::c_int as UWORD8,
    18 as core::ffi::c_int as UWORD8,
    19 as core::ffi::c_int as UWORD8,
    20 as core::ffi::c_int as UWORD8,
    21 as core::ffi::c_int as UWORD8,
    22 as core::ffi::c_int as UWORD8,
    23 as core::ffi::c_int as UWORD8,
    24 as core::ffi::c_int as UWORD8,
    25 as core::ffi::c_int as UWORD8,
    26 as core::ffi::c_int as UWORD8,
    27 as core::ffi::c_int as UWORD8,
    28 as core::ffi::c_int as UWORD8,
    29 as core::ffi::c_int as UWORD8,
    30 as core::ffi::c_int as UWORD8,
    31 as core::ffi::c_int as UWORD8,
    11 as core::ffi::c_int as UWORD8,
];
#[no_mangle]
pub static mut ixheaacd_max_huff_cw_len_table: [UWORD8; 32] = [
    0 as core::ffi::c_int as UWORD8,
    11 as core::ffi::c_int as UWORD8,
    9 as core::ffi::c_int as UWORD8,
    20 as core::ffi::c_int as UWORD8,
    16 as core::ffi::c_int as UWORD8,
    13 as core::ffi::c_int as UWORD8,
    11 as core::ffi::c_int as UWORD8,
    14 as core::ffi::c_int as UWORD8,
    12 as core::ffi::c_int as UWORD8,
    17 as core::ffi::c_int as UWORD8,
    14 as core::ffi::c_int as UWORD8,
    49 as core::ffi::c_int as UWORD8,
    0 as core::ffi::c_int as UWORD8,
    0 as core::ffi::c_int as UWORD8,
    0 as core::ffi::c_int as UWORD8,
    0 as core::ffi::c_int as UWORD8,
    14 as core::ffi::c_int as UWORD8,
    17 as core::ffi::c_int as UWORD8,
    21 as core::ffi::c_int as UWORD8,
    21 as core::ffi::c_int as UWORD8,
    25 as core::ffi::c_int as UWORD8,
    25 as core::ffi::c_int as UWORD8,
    29 as core::ffi::c_int as UWORD8,
    29 as core::ffi::c_int as UWORD8,
    29 as core::ffi::c_int as UWORD8,
    29 as core::ffi::c_int as UWORD8,
    33 as core::ffi::c_int as UWORD8,
    33 as core::ffi::c_int as UWORD8,
    33 as core::ffi::c_int as UWORD8,
    37 as core::ffi::c_int as UWORD8,
    37 as core::ffi::c_int as UWORD8,
    41 as core::ffi::c_int as UWORD8,
];
#[no_mangle]
pub static mut ixheaacd_huff_cb_dim_table: [UWORD8; 32] = [
    2 as core::ffi::c_int as UWORD8,
    4 as core::ffi::c_int as UWORD8,
    4 as core::ffi::c_int as UWORD8,
    4 as core::ffi::c_int as UWORD8,
    4 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
];
#[no_mangle]
pub static mut ixheaacd_huff_cb_dim_shift_table: [UWORD8; 32] = [
    1 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    0 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
];
#[no_mangle]
pub static mut ixheaacd_huff_cb_sign_table: [UWORD8; 32] = [
    0 as core::ffi::c_int as UWORD8,
    0 as core::ffi::c_int as UWORD8,
    0 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    0 as core::ffi::c_int as UWORD8,
    0 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    0 as core::ffi::c_int as UWORD8,
    0 as core::ffi::c_int as UWORD8,
    0 as core::ffi::c_int as UWORD8,
    0 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
];
#[no_mangle]
pub static mut ixheaacd_huff_cb_priority_table: [UWORD8; 32] = [
    0 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    1 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    2 as core::ffi::c_int as UWORD8,
    3 as core::ffi::c_int as UWORD8,
    3 as core::ffi::c_int as UWORD8,
    4 as core::ffi::c_int as UWORD8,
    4 as core::ffi::c_int as UWORD8,
    5 as core::ffi::c_int as UWORD8,
    5 as core::ffi::c_int as UWORD8,
    22 as core::ffi::c_int as UWORD8,
    0 as core::ffi::c_int as UWORD8,
    0 as core::ffi::c_int as UWORD8,
    0 as core::ffi::c_int as UWORD8,
    0 as core::ffi::c_int as UWORD8,
    6 as core::ffi::c_int as UWORD8,
    7 as core::ffi::c_int as UWORD8,
    8 as core::ffi::c_int as UWORD8,
    9 as core::ffi::c_int as UWORD8,
    10 as core::ffi::c_int as UWORD8,
    11 as core::ffi::c_int as UWORD8,
    12 as core::ffi::c_int as UWORD8,
    13 as core::ffi::c_int as UWORD8,
    14 as core::ffi::c_int as UWORD8,
    15 as core::ffi::c_int as UWORD8,
    16 as core::ffi::c_int as UWORD8,
    17 as core::ffi::c_int as UWORD8,
    18 as core::ffi::c_int as UWORD8,
    19 as core::ffi::c_int as UWORD8,
    20 as core::ffi::c_int as UWORD8,
    21 as core::ffi::c_int as UWORD8,
];
#[no_mangle]
pub static mut ixheaacd_huff_reord_lav_table: [UWORD16; 32] = [
    0 as core::ffi::c_int as UWORD16,
    1 as core::ffi::c_int as UWORD16,
    1 as core::ffi::c_int as UWORD16,
    2 as core::ffi::c_int as UWORD16,
    2 as core::ffi::c_int as UWORD16,
    4 as core::ffi::c_int as UWORD16,
    4 as core::ffi::c_int as UWORD16,
    7 as core::ffi::c_int as UWORD16,
    7 as core::ffi::c_int as UWORD16,
    12 as core::ffi::c_int as UWORD16,
    12 as core::ffi::c_int as UWORD16,
    8191 as core::ffi::c_int as UWORD16,
    0 as core::ffi::c_int as UWORD16,
    0 as core::ffi::c_int as UWORD16,
    0 as core::ffi::c_int as UWORD16,
    0 as core::ffi::c_int as UWORD16,
    15 as core::ffi::c_int as UWORD16,
    31 as core::ffi::c_int as UWORD16,
    47 as core::ffi::c_int as UWORD16,
    63 as core::ffi::c_int as UWORD16,
    95 as core::ffi::c_int as UWORD16,
    127 as core::ffi::c_int as UWORD16,
    159 as core::ffi::c_int as UWORD16,
    191 as core::ffi::c_int as UWORD16,
    223 as core::ffi::c_int as UWORD16,
    255 as core::ffi::c_int as UWORD16,
    319 as core::ffi::c_int as UWORD16,
    383 as core::ffi::c_int as UWORD16,
    511 as core::ffi::c_int as UWORD16,
    767 as core::ffi::c_int as UWORD16,
    1023 as core::ffi::c_int as UWORD16,
    2047 as core::ffi::c_int as UWORD16,
];
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_huff_code_reorder_tbl_init(
    mut ptr_hcr_info: *mut ia_hcr_info_struct,
) -> VOID {
    (*ptr_hcr_info).codebook_pairs.ptr_min_cb_pair_tbl = ixheaacd_min_huff_cb_pair_tbl
        .as_ptr();
    (*ptr_hcr_info).codebook_pairs.ptr_max_cb_pair_tbl = ixheaacd_max_huff_cb_pair_table
        .as_ptr();
    (*ptr_hcr_info).table_info.ptr_max_cw_len_tbl = ixheaacd_max_huff_cw_len_table
        .as_ptr();
    (*ptr_hcr_info).table_info.ptr_cb_dimension_tbl = ixheaacd_huff_cb_dim_table
        .as_ptr();
    (*ptr_hcr_info).table_info.ptr_cb_dim_shift_tbl = ixheaacd_huff_cb_dim_shift_table
        .as_ptr();
    (*ptr_hcr_info).table_info.ptr_cb_sign_tbl = ixheaacd_huff_cb_sign_table.as_ptr();
    (*ptr_hcr_info).table_info.ptr_cb_priority = ixheaacd_huff_cb_priority_table
        .as_ptr();
    (*ptr_hcr_info).table_info.ptr_lav_tbl = ixheaacd_huff_reord_lav_table.as_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_huff_mute_erroneous_lines(
    mut ptr_hcr_info: *mut ia_hcr_info_struct,
) -> VOID {
    let mut c: WORD32 = 0;
    let mut ptr_long: *mut WORD32 = (*ptr_hcr_info).str_dec_io.ptr_quant_spec_coeff_base;
    c = 0 as core::ffi::c_int as WORD32;
    while c < 1024 as core::ffi::c_int {
        if *ptr_long.offset(c as isize) == 8192 as core::ffi::c_int {
            *ptr_long.offset(c as isize) = 0 as core::ffi::c_int as WORD32;
        }
        c += 1;
    }
}
unsafe extern "C" fn ixheaacd_err_detect_pcw_segment(
    mut remaining_bits_in_segment: WORD8,
    mut ptr_hcr_info: *mut ia_hcr_info_struct,
    mut kind: ia_pcw_type_struct,
    mut qsc_base_of_cw: *mut WORD32,
    mut dimension: UWORD8,
) -> UWORD8 {
    let mut i: WORD8 = 0;
    if (remaining_bits_in_segment as core::ffi::c_int) < 0 as core::ffi::c_int {
        match kind as core::ffi::c_uint {
            0 => {
                (*ptr_hcr_info).str_dec_io.err_log
                    |= (ERROR_POS << 31 as core::ffi::c_int) as UWORD32;
            }
            1 => {
                (*ptr_hcr_info).str_dec_io.err_log
                    |= (ERROR_POS << 30 as core::ffi::c_int) as UWORD32;
            }
            2 => {
                (*ptr_hcr_info).str_dec_io.err_log
                    |= (ERROR_POS << 29 as core::ffi::c_int) as UWORD32;
            }
            _ => {}
        }
        i = dimension as WORD8;
        while i as core::ffi::c_int != 0 as core::ffi::c_int {
            let fresh0 = qsc_base_of_cw;
            qsc_base_of_cw = qsc_base_of_cw.offset(1);
            *fresh0 = 8192 as core::ffi::c_int;
            i -= 1;
        }
        return 1 as UWORD8;
    }
    return 0 as UWORD8;
}
unsafe extern "C" fn ixheaacd_nonpcw_sideinfo_init(
    mut ptr_hcr_info: *mut ia_hcr_info_struct,
) -> VOID {
    let mut i: UWORD16 = 0;
    let mut k: UWORD16 = 0;
    let mut cb_dim: UWORD8 = 0;
    let mut ptr_cb: *mut UWORD8 = ((*ptr_hcr_info).str_non_pcw_side_info.ptr_cb)
        .as_mut_ptr();
    let mut res_ptr_idx: *mut UWORD16 = ((*ptr_hcr_info)
        .str_non_pcw_side_info
        .res_ptr_idx)
        .as_mut_ptr();
    let mut ptr_num_ext_sorted_cw_in_sect: *mut UWORD16 = ((*ptr_hcr_info)
        .sect_info
        .ptr_num_ext_sorted_cw_in_sect)
        .as_mut_ptr();
    let mut num_ext_sorted_cw_in_sect_idx: WORD32 = (*ptr_hcr_info)
        .sect_info
        .num_ext_sorted_cw_in_sect_idx;
    let mut ptr_ext_sorted_cw: *mut UWORD8 = ((*ptr_hcr_info)
        .sect_info
        .ptr_ext_sorted_cw)
        .as_mut_ptr();
    let mut ext_sorted_cw_idx: WORD32 = (*ptr_hcr_info).sect_info.ext_sorted_cw_idx;
    let mut ptr_num_ext_sorted_sect_in_sets: *mut UWORD16 = ((*ptr_hcr_info)
        .sect_info
        .ptr_num_ext_sorted_sect_in_sets)
        .as_mut_ptr();
    let mut num_ext_sorted_sect_in_sets_idx: WORD32 = (*ptr_hcr_info)
        .sect_info
        .num_ext_sorted_sect_in_sets_idx;
    let mut quant_spec_coeff_idx: WORD32 = (*ptr_hcr_info)
        .str_dec_io
        .quant_spec_coeff_idx;
    let mut ptr_cb_dimension_tbl: *const UWORD8 = (*ptr_hcr_info)
        .table_info
        .ptr_cb_dimension_tbl;
    let mut loop_idx: WORD32 = 0 as WORD32;
    i = *ptr_num_ext_sorted_sect_in_sets
        .offset(num_ext_sorted_sect_in_sets_idx as isize);
    while i as core::ffi::c_int != 0 as core::ffi::c_int {
        cb_dim = *ptr_cb_dimension_tbl
            .offset(*ptr_ext_sorted_cw.offset(ext_sorted_cw_idx as isize) as isize);
        k = *ptr_num_ext_sorted_cw_in_sect
            .offset(num_ext_sorted_cw_in_sect_idx as isize);
        while k as core::ffi::c_int != 0 as core::ffi::c_int {
            loop_idx += 1;
            if loop_idx > 256 as core::ffi::c_int {
                return;
            }
            let fresh1 = ptr_cb;
            ptr_cb = ptr_cb.offset(1);
            *fresh1 = *ptr_ext_sorted_cw.offset(ext_sorted_cw_idx as isize);
            let fresh2 = res_ptr_idx;
            res_ptr_idx = res_ptr_idx.offset(1);
            *fresh2 = quant_spec_coeff_idx as UWORD16;
            quant_spec_coeff_idx += cb_dim as core::ffi::c_int;
            if quant_spec_coeff_idx >= 1024 as core::ffi::c_int {
                return;
            }
            k = k.wrapping_sub(1);
        }
        num_ext_sorted_cw_in_sect_idx += 1;
        ext_sorted_cw_idx += 1;
        if num_ext_sorted_cw_in_sect_idx >= MAX_SFB_HCR + MAX_HCR_SETS
            || ext_sorted_cw_idx >= MAX_SFB_HCR + MAX_HCR_SETS
        {
            return;
        }
        i = i.wrapping_sub(1);
    }
    num_ext_sorted_sect_in_sets_idx += 1;
    if num_ext_sorted_cw_in_sect_idx >= MAX_SFB_HCR + MAX_HCR_SETS {
        return;
    }
    (*ptr_hcr_info).sect_info.num_ext_sorted_cw_in_sect_idx = num_ext_sorted_cw_in_sect_idx;
    (*ptr_hcr_info).sect_info.ext_sorted_cw_idx = ext_sorted_cw_idx;
    (*ptr_hcr_info).sect_info.num_ext_sorted_sect_in_sets_idx = num_ext_sorted_sect_in_sets_idx;
    (*ptr_hcr_info).sect_info.num_ext_sorted_cw_in_sect_idx = num_ext_sorted_cw_in_sect_idx;
    (*ptr_hcr_info).str_dec_io.quant_spec_coeff_idx = quant_spec_coeff_idx;
}
unsafe extern "C" fn ixheaacd_calc_num_ext_sorted_sect_sets(
    mut num_segment: UWORD32,
    mut ptr_num_ext_sorted_cw_in_sect: *mut UWORD16,
    mut num_ext_sorted_cw_in_sect_idx: WORD32,
    mut ptr_num_ext_sorted_sect_in_sets: *mut UWORD16,
    mut num_ext_sorted_sect_in_sets_idx: WORD32,
) -> VOID {
    let mut counter: UWORD16 = 0 as UWORD16;
    let mut cw_sum: UWORD32 = 0 as UWORD32;
    let mut ptr_num_ext_sort_cw_in_sect: *mut UWORD16 = ptr_num_ext_sorted_cw_in_sect;
    let mut ptr_num_ext_sort_sect_in_sets: *mut UWORD16 = ptr_num_ext_sorted_sect_in_sets;
    while *ptr_num_ext_sort_cw_in_sect.offset(num_ext_sorted_cw_in_sect_idx as isize)
        as core::ffi::c_int != 0 as core::ffi::c_int
    {
        cw_sum = cw_sum
            .wrapping_add(
                *ptr_num_ext_sort_cw_in_sect
                    .offset(num_ext_sorted_cw_in_sect_idx as isize) as UWORD32,
            );
        num_ext_sorted_cw_in_sect_idx += 1;
        if num_ext_sorted_cw_in_sect_idx >= MAX_SFB_HCR + MAX_HCR_SETS {
            return;
        }
        if cw_sum > num_segment {
            return;
        }
        counter = counter.wrapping_add(1);
        if counter as core::ffi::c_int > 256 as core::ffi::c_int {
            return;
        }
        if cw_sum == num_segment {
            *ptr_num_ext_sort_sect_in_sets
                .offset(num_ext_sorted_sect_in_sets_idx as isize) = counter;
            num_ext_sorted_sect_in_sets_idx += 1;
            if num_ext_sorted_sect_in_sets_idx >= MAX_HCR_SETS {
                return;
            }
            counter = 0 as UWORD16;
            cw_sum = 0 as UWORD32;
        }
    }
    *ptr_num_ext_sort_sect_in_sets.offset(num_ext_sorted_sect_in_sets_idx as isize) = counter;
}
unsafe extern "C" fn ixheaacd_validate_hcr_sideinfo(
    mut cb: WORD8,
    mut num_line: WORD32,
    mut error_word: *mut UWORD32,
) -> VOID {
    if (cb as core::ffi::c_int) < ZERO_HCB || cb as core::ffi::c_int >= MAX_CB_CHECK
        || cb as core::ffi::c_int == ESC_HCB + 1 as core::ffi::c_int
    {
        *error_word |= (ERROR_POS << 4 as core::ffi::c_int) as UWORD32;
    }
    if num_line < 0 as core::ffi::c_int || num_line > 1024 as core::ffi::c_int {
        *error_word |= (ERROR_POS << 5 as core::ffi::c_int) as UWORD32;
    }
}
unsafe extern "C" fn ixheaacd_validate_hcr_lengths(
    mut longest_cw_len: WORD8,
    mut reordered_spec_data_len: WORD16,
    mut error_word: *mut UWORD32,
) -> VOID {
    if (reordered_spec_data_len as core::ffi::c_int) < longest_cw_len as core::ffi::c_int
    {
        *error_word |= (ERROR_POS << 8 as core::ffi::c_int) as UWORD32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_huff_code_reorder_init(
    mut ptr_hcr_info: *mut ia_hcr_info_struct,
    mut ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
    mut ptr_aac_tables: *mut ia_aac_dec_tables_struct,
    mut itt_bit_buff: *mut ia_bit_buf_struct,
) -> UWORD32 {
    let mut ptr_ics_info: *mut ia_ics_info_struct = &mut (*ptr_aac_dec_channel_info)
        .str_ics_info;
    let mut ptr_num_sect_lines: *mut WORD16 = 0 as *mut WORD16;
    let mut ptr_cb: *mut UWORD8 = 0 as *mut UWORD8;
    let mut num_sect: WORD16 = 0;
    let mut cb: WORD8 = 0;
    let mut num_line: WORD32 = 0;
    let mut i: WORD32 = 0;
    (*ptr_hcr_info).str_dec_io.reordered_spec_data_len = (*ptr_aac_dec_channel_info)
        .reorder_spect_data_len;
    (*ptr_hcr_info).str_dec_io.longest_cw_len = (*ptr_aac_dec_channel_info)
        .longest_cw_len;
    (*ptr_hcr_info).str_dec_io.ptr_quant_spec_coeff_base = (*ptr_aac_dec_channel_info)
        .ptr_spec_coeff;
    (*ptr_hcr_info).str_dec_io.quant_spec_coeff_idx = 0 as core::ffi::c_int as WORD32;
    (*ptr_hcr_info).str_dec_io.ptr_cb = ((*ptr_aac_dec_channel_info).cb4_hcr_arr)
        .as_mut_ptr();
    (*ptr_hcr_info).str_dec_io.ptr_num_line_in_sect = ((*ptr_aac_dec_channel_info)
        .num_line_in_sec4_hcr_arr)
        .as_mut_ptr();
    (*ptr_hcr_info).str_dec_io.num_sect = (*ptr_aac_dec_channel_info).number_sect
        as WORD16;
    (*ptr_hcr_info).str_dec_io.err_log = 0 as UWORD32;
    (*ptr_hcr_info).str_non_pcw_side_info.ptr_result_base = (*ptr_aac_dec_channel_info)
        .ptr_spec_coeff;
    (*ptr_hcr_info).str_dec_io.bit_str_idx = ((*itt_bit_buff).size
        - (*itt_bit_buff).cnt_bits) as UWORD16;
    (*itt_bit_buff).byte_ptr = (*ptr_aac_dec_channel_info).scratch_buf_ptr
        as *mut UWORD8;
    (*itt_bit_buff).ptr_start = (*ptr_aac_dec_channel_info).scratch_buf_ptr
        as *mut UWORD8;
    if (*ptr_aac_dec_channel_info).str_ics_info.window_sequence as core::ffi::c_int
        == EIGHT_SHORT_SEQUENCE
    {
        let mut band: WORD16 = 0;
        let mut max_band: WORD16 = 0;
        let mut group: WORD8 = 0;
        let mut win_group_len: WORD8 = 0;
        let mut window: WORD8 = 0;
        let mut num_unit_in_band: WORD8 = 0;
        let mut cnt_unit_in_band: WORD8 = 0;
        let mut grp_win: WORD8 = 0;
        let mut cb_prev: WORD8 = 0;
        let mut ptr_code_book: *mut WORD8 = 0 as *mut WORD8;
        let mut band_offsets: *const WORD16 = 0 as *const WORD16;
        let mut num_groups: WORD16 = 0;
        ptr_code_book = (*ptr_aac_dec_channel_info).ptr_code_book;
        ptr_num_sect_lines = (*ptr_hcr_info).str_dec_io.ptr_num_line_in_sect;
        ptr_cb = (*ptr_hcr_info).str_dec_io.ptr_cb;
        band_offsets = ixheaacd_getscalefactorbandoffsets(ptr_ics_info, ptr_aac_tables);
        num_groups = (*ptr_ics_info).num_window_groups;
        num_line = 0 as core::ffi::c_int as WORD32;
        num_sect = 0 as WORD16;
        cb = *ptr_code_book.offset(0 as core::ffi::c_int as isize);
        cb_prev = *ptr_code_book.offset(0 as core::ffi::c_int as isize);
        let fresh3 = ptr_cb;
        ptr_cb = ptr_cb.offset(1);
        *fresh3 = cb_prev as UWORD8;
        max_band = (*ptr_ics_info).max_sfb;
        band = 0 as WORD16;
        while (band as core::ffi::c_int) < max_band as core::ffi::c_int {
            num_unit_in_band = (*band_offsets
                .offset((band as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                as core::ffi::c_int
                - *band_offsets.offset(band as isize) as core::ffi::c_int
                >> 2 as core::ffi::c_int) as WORD8;
            cnt_unit_in_band = num_unit_in_band;
            while cnt_unit_in_band as core::ffi::c_int != 0 as core::ffi::c_int {
                window = 0 as WORD8;
                group = 0 as WORD8;
                while (group as core::ffi::c_int) < num_groups as core::ffi::c_int {
                    win_group_len = (*ptr_ics_info).window_group_length[group as usize];
                    grp_win = win_group_len;
                    while grp_win as core::ffi::c_int != 0 as core::ffi::c_int {
                        cb = *ptr_code_book
                            .offset(
                                (group as core::ffi::c_int * 16 as core::ffi::c_int
                                    + band as core::ffi::c_int) as isize,
                            );
                        if cb as core::ffi::c_int != cb_prev as core::ffi::c_int {
                            ixheaacd_validate_hcr_sideinfo(
                                cb,
                                num_line,
                                &mut (*ptr_hcr_info).str_dec_io.err_log,
                            );
                            if (*ptr_hcr_info).str_dec_io.err_log != 0 as UWORD32 {
                                return (*ptr_hcr_info).str_dec_io.err_log;
                            }
                            let fresh4 = ptr_cb;
                            ptr_cb = ptr_cb.offset(1);
                            *fresh4 = cb as UWORD8;
                            let fresh5 = ptr_num_sect_lines;
                            ptr_num_sect_lines = ptr_num_sect_lines.offset(1);
                            *fresh5 = num_line as WORD16;
                            num_sect += 1;
                            cb_prev = cb;
                            num_line = LINES_PER_UNIT as WORD32;
                        } else {
                            num_line += LINES_PER_UNIT;
                        }
                        grp_win -= 1;
                        window += 1;
                    }
                    group += 1;
                }
                cnt_unit_in_band -= 1;
            }
            band += 1;
        }
        num_sect += 1;
        ixheaacd_validate_hcr_sideinfo(
            cb,
            num_line,
            &mut (*ptr_hcr_info).str_dec_io.err_log,
        );
        if num_sect as core::ffi::c_int <= 0 as core::ffi::c_int
            || num_sect as core::ffi::c_int
                > 1024 as core::ffi::c_int / 2 as core::ffi::c_int
        {
            (*ptr_hcr_info).str_dec_io.err_log
                |= (ERROR_POS << 7 as core::ffi::c_int) as UWORD32;
        }
        ixheaacd_validate_hcr_lengths(
            (*ptr_hcr_info).str_dec_io.longest_cw_len,
            (*ptr_hcr_info).str_dec_io.reordered_spec_data_len,
            &mut (*ptr_hcr_info).str_dec_io.err_log,
        );
        if (*ptr_hcr_info).str_dec_io.err_log != 0 as UWORD32 {
            return (*ptr_hcr_info).str_dec_io.err_log;
        }
        *ptr_cb = cb as UWORD8;
        *ptr_num_sect_lines = num_line as WORD16;
        (*ptr_hcr_info).str_dec_io.num_sect = num_sect;
    } else {
        ixheaacd_validate_hcr_lengths(
            (*ptr_hcr_info).str_dec_io.longest_cw_len,
            (*ptr_hcr_info).str_dec_io.reordered_spec_data_len,
            &mut (*ptr_hcr_info).str_dec_io.err_log,
        );
        num_sect = (*ptr_hcr_info).str_dec_io.num_sect;
        ptr_num_sect_lines = (*ptr_hcr_info).str_dec_io.ptr_num_line_in_sect;
        ptr_cb = (*ptr_hcr_info).str_dec_io.ptr_cb;
        if num_sect as core::ffi::c_int <= 0 as core::ffi::c_int
            || num_sect as core::ffi::c_int > 64 as core::ffi::c_int
        {
            (*ptr_hcr_info).str_dec_io.err_log
                |= (ERROR_POS << 6 as core::ffi::c_int) as UWORD32;
            num_sect = 0 as WORD16;
        }
        i = num_sect as WORD32;
        while i != 0 as core::ffi::c_int {
            let fresh6 = ptr_cb;
            ptr_cb = ptr_cb.offset(1);
            cb = *fresh6 as WORD8;
            if (cb as core::ffi::c_int) < ZERO_HCB
                || cb as core::ffi::c_int >= MAX_CB_CHECK
                || cb as core::ffi::c_int == ESC_HCB + 1 as core::ffi::c_int
            {
                (*ptr_hcr_info).str_dec_io.err_log
                    |= (ERROR_POS << 2 as core::ffi::c_int) as UWORD32;
            }
            let fresh7 = ptr_num_sect_lines;
            ptr_num_sect_lines = ptr_num_sect_lines.offset(1);
            num_line = *fresh7 as WORD32;
            if num_line <= 0 as core::ffi::c_int || num_line > 1024 as core::ffi::c_int {
                (*ptr_hcr_info).str_dec_io.err_log
                    |= (ERROR_POS << 3 as core::ffi::c_int) as UWORD32;
            }
            i -= 1;
        }
        if (*ptr_hcr_info).str_dec_io.err_log != 0 as UWORD32 {
            return (*ptr_hcr_info).str_dec_io.err_log;
        }
    }
    ptr_cb = (*ptr_hcr_info).str_dec_io.ptr_cb;
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_sect as core::ffi::c_int {
        if *ptr_cb as core::ffi::c_int == NOISE_HCB
            || *ptr_cb as core::ffi::c_int == INTENSITY_HCB2
            || *ptr_cb as core::ffi::c_int == INTENSITY_HCB
        {
            *ptr_cb = 0 as UWORD8;
        }
        ptr_cb = ptr_cb.offset(1);
        i += 1;
    }
    return (*ptr_hcr_info).str_dec_io.err_log;
}
unsafe extern "C" fn ixheaacd_huff_calc_num_cwd(
    mut ptr_hcr_info: *mut ia_hcr_info_struct,
) -> VOID {
    let mut sect_idx: WORD32 = 0;
    let mut num_code_word: UWORD32 = 0;
    let mut num_sect: UWORD32 = (*ptr_hcr_info).str_dec_io.num_sect as UWORD32;
    let mut ptr_cb: *mut UWORD8 = (*ptr_hcr_info).str_dec_io.ptr_cb;
    let mut ptr_num_line_in_sect: *mut WORD16 = (*ptr_hcr_info)
        .str_dec_io
        .ptr_num_line_in_sect;
    let mut ptr_cb_dim_shift_tbl: *const UWORD8 = (*ptr_hcr_info)
        .table_info
        .ptr_cb_dim_shift_tbl;
    let mut ptr_num_cw_in_sect: *mut UWORD16 = ((*ptr_hcr_info)
        .sect_info
        .ptr_num_cw_in_sect)
        .as_mut_ptr();
    num_code_word = 0 as UWORD32;
    sect_idx = num_sect as WORD32;
    while sect_idx != 0 as core::ffi::c_int {
        let fresh8 = ptr_num_line_in_sect;
        ptr_num_line_in_sect = ptr_num_line_in_sect.offset(1);
        *ptr_num_cw_in_sect = (*fresh8 as core::ffi::c_int
            >> *ptr_cb_dim_shift_tbl.offset(*ptr_cb as isize) as core::ffi::c_int)
            as UWORD16;
        if *ptr_cb as core::ffi::c_int != 0 as core::ffi::c_int {
            num_code_word = num_code_word.wrapping_add(*ptr_num_cw_in_sect as UWORD32);
        }
        ptr_num_cw_in_sect = ptr_num_cw_in_sect.offset(1);
        ptr_cb = ptr_cb.offset(1);
        sect_idx -= 1;
    }
    (*ptr_hcr_info).sect_info.num_code_word = num_code_word;
}
unsafe extern "C" fn ixheaacd_huff_sort_sect_cb_cwd(
    mut ptr_hcr_info: *mut ia_hcr_info_struct,
) -> VOID {
    let mut i: UWORD32 = 0;
    let mut j: UWORD32 = 0;
    let mut k: UWORD32 = 0;
    let mut temp: UWORD8 = 0;
    let mut counter: UWORD32 = 0;
    let mut start_offset: UWORD32 = 0;
    let mut num_zero_sect: UWORD32 = 0;
    let mut ptr_dest: *mut UWORD8 = 0 as *mut UWORD8;
    let mut num_sect_dec: UWORD32 = 0;
    let mut num_sect: UWORD32 = (*ptr_hcr_info).str_dec_io.num_sect as UWORD32;
    let mut ptr_cb: *mut UWORD8 = (*ptr_hcr_info).str_dec_io.ptr_cb;
    let mut ptr_sorted_cb: *mut UWORD8 = ((*ptr_hcr_info).sect_info.ptr_sorted_cb)
        .as_mut_ptr();
    let mut ptr_num_cw_in_sect: *mut UWORD16 = ((*ptr_hcr_info)
        .sect_info
        .ptr_num_cw_in_sect)
        .as_mut_ptr();
    let mut ptr_num_sorted_cw_in_sect: *mut UWORD16 = ((*ptr_hcr_info)
        .sect_info
        .ptr_num_sorted_cw_in_sect)
        .as_mut_ptr();
    let mut ptr_cb_switch: *mut UWORD8 = ((*ptr_hcr_info).sect_info.ptr_cb_switch)
        .as_mut_ptr();
    let mut ptr_reorder_offset: *mut UWORD16 = ((*ptr_hcr_info)
        .sect_info
        .ptr_reorder_offset)
        .as_mut_ptr();
    let mut ptr_cb_priority: *const UWORD8 = (*ptr_hcr_info).table_info.ptr_cb_priority;
    let mut ptr_min_cb_pair_tbl: *const UWORD8 = (*ptr_hcr_info)
        .codebook_pairs
        .ptr_min_cb_pair_tbl;
    let mut ptr_max_cb_pair_tbl: *const UWORD8 = (*ptr_hcr_info)
        .codebook_pairs
        .ptr_max_cb_pair_tbl;
    let mut ptr_cb_dim_shift_tbl: *const UWORD8 = (*ptr_hcr_info)
        .table_info
        .ptr_cb_dim_shift_tbl;
    let mut search_start_idx: UWORD32 = 0 as UWORD32;
    ptr_dest = ptr_sorted_cb;
    num_zero_sect = 0 as UWORD32;
    i = num_sect;
    while i != 0 as UWORD32 {
        if *ptr_cb_priority.offset(*ptr_cb as isize) as core::ffi::c_int
            == 0 as core::ffi::c_int
        {
            num_zero_sect = num_zero_sect.wrapping_add(1 as UWORD32);
        }
        let fresh9 = ptr_cb;
        ptr_cb = ptr_cb.offset(1);
        let fresh10 = ptr_dest;
        ptr_dest = ptr_dest.offset(1);
        *fresh10 = *ptr_cb_priority.offset(*fresh9 as isize);
        i = i.wrapping_sub(1);
    }
    (*ptr_hcr_info).sect_info.num_sorted_section = num_sect.wrapping_sub(num_zero_sect);
    ptr_cb = (*ptr_hcr_info).str_dec_io.ptr_cb;
    num_sect_dec = num_sect.wrapping_sub(1 as UWORD32);
    if num_sect_dec > 0 as UWORD32 {
        counter = num_sect_dec;
        j = num_sect_dec;
        while j != 0 as UWORD32 {
            i = 0 as UWORD32;
            while i < counter {
                if *ptr_sorted_cb.offset(i.wrapping_add(1 as UWORD32) as isize)
                    as core::ffi::c_int
                    > *ptr_sorted_cb.offset(i as isize) as core::ffi::c_int
                {
                    temp = *ptr_sorted_cb.offset(i as isize);
                    *ptr_sorted_cb.offset(i as isize) = *ptr_sorted_cb
                        .offset(i.wrapping_add(1 as UWORD32) as isize);
                    *ptr_sorted_cb.offset(i.wrapping_add(1 as UWORD32) as isize) = temp;
                }
                i = i.wrapping_add(1);
            }
            counter = counter.wrapping_sub(1 as UWORD32);
            j = j.wrapping_sub(1);
        }
    }
    i = num_sect;
    while i != 0 as UWORD32 {
        let fresh11 = ptr_cb_switch;
        ptr_cb_switch = ptr_cb_switch.offset(1);
        *fresh11 = 0 as UWORD8;
        i = i.wrapping_sub(1);
    }
    ptr_cb_switch = ((*ptr_hcr_info).sect_info.ptr_cb_switch).as_mut_ptr();
    j = 0 as UWORD32;
    while j < num_sect {
        i = search_start_idx;
        while i < num_sect {
            if *ptr_cb_switch.offset(i as isize) as core::ffi::c_int
                == 0 as core::ffi::c_int
                && (*ptr_min_cb_pair_tbl
                    .offset(*ptr_sorted_cb.offset(j as isize) as isize)
                    as core::ffi::c_int == *ptr_cb.offset(i as isize) as core::ffi::c_int
                    || *ptr_max_cb_pair_tbl
                        .offset(*ptr_sorted_cb.offset(j as isize) as isize)
                        as core::ffi::c_int
                        == *ptr_cb.offset(i as isize) as core::ffi::c_int)
            {
                *ptr_cb_switch.offset(i as isize) = 1 as UWORD8;
                *ptr_sorted_cb.offset(j as isize) = *ptr_cb.offset(i as isize);
                *ptr_num_sorted_cw_in_sect.offset(j as isize) = *ptr_num_cw_in_sect
                    .offset(i as isize);
                start_offset = 0 as UWORD32;
                k = 0 as UWORD32;
                while k < i {
                    start_offset = start_offset
                        .wrapping_add(
                            ((*ptr_num_cw_in_sect.offset(k as isize) as core::ffi::c_int)
                                << *ptr_cb_dim_shift_tbl
                                    .offset(*ptr_cb.offset(k as isize) as isize)
                                    as core::ffi::c_int) as UWORD32,
                        );
                    k = k.wrapping_add(1);
                }
                *ptr_reorder_offset.offset(j as isize) = start_offset as UWORD16;
                if i == search_start_idx {
                    let mut k_0: UWORD32 = i;
                    loop {
                        let fresh12 = k_0;
                        k_0 = k_0.wrapping_add(1);
                        if !(*ptr_cb_switch.offset(fresh12 as isize) as core::ffi::c_int
                            == 1 as core::ffi::c_int)
                        {
                            break;
                        }
                        search_start_idx = search_start_idx.wrapping_add(1);
                    }
                }
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn ixheaacd_huff_ext_sect_info(
    mut ptr_hcr_info: *mut ia_hcr_info_struct,
) -> VOID {
    let mut srt_sec_cnt: UWORD32 = 0 as UWORD32;
    let mut x_srt_sc_cnt: UWORD32 = 0 as UWORD32;
    let mut remain_num_cw_sort_sec: UWORD32 = 0;
    let mut in_segment_remain_num_cw: UWORD32 = 0;
    let mut num_sorted_section: UWORD32 = (*ptr_hcr_info).sect_info.num_sorted_section;
    let mut ptr_sorted_cb: *mut UWORD8 = ((*ptr_hcr_info).sect_info.ptr_sorted_cb)
        .as_mut_ptr();
    let mut ptr_num_sorted_cw_in_sect: *mut UWORD16 = ((*ptr_hcr_info)
        .sect_info
        .ptr_num_sorted_cw_in_sect)
        .as_mut_ptr();
    let mut ptr_extended_sorted_code_book: *mut UWORD8 = ((*ptr_hcr_info)
        .sect_info
        .ptr_ext_sorted_cw)
        .as_mut_ptr();
    let mut ptr_num_ext_sort_cw_sect: *mut UWORD16 = ((*ptr_hcr_info)
        .sect_info
        .ptr_num_ext_sorted_cw_in_sect)
        .as_mut_ptr();
    let mut num_segment: UWORD32 = (*ptr_hcr_info).str_segment_info.num_segment
        as UWORD32;
    let mut ptr_ext_sorted_sect_max_cb_len: *mut UWORD8 = ((*ptr_hcr_info)
        .sect_info
        .ptr_ext_sorted_sect_max_cb_len)
        .as_mut_ptr();
    let mut longest_cw_len: WORD8 = (*ptr_hcr_info).str_dec_io.longest_cw_len;
    let mut ptr_max_cw_len_tbl: *const UWORD8 = (*ptr_hcr_info)
        .table_info
        .ptr_max_cw_len_tbl;
    remain_num_cw_sort_sec = *ptr_num_sorted_cw_in_sect.offset(srt_sec_cnt as isize)
        as UWORD32;
    in_segment_remain_num_cw = num_segment;
    while srt_sec_cnt < num_sorted_section {
        if in_segment_remain_num_cw < remain_num_cw_sort_sec {
            *ptr_num_ext_sort_cw_sect.offset(x_srt_sc_cnt as isize) = in_segment_remain_num_cw
                as UWORD16;
            *ptr_extended_sorted_code_book.offset(x_srt_sc_cnt as isize) = *ptr_sorted_cb
                .offset(srt_sec_cnt as isize);
            remain_num_cw_sort_sec = remain_num_cw_sort_sec
                .wrapping_sub(in_segment_remain_num_cw);
            in_segment_remain_num_cw = num_segment;
        } else if in_segment_remain_num_cw == remain_num_cw_sort_sec {
            *ptr_num_ext_sort_cw_sect.offset(x_srt_sc_cnt as isize) = in_segment_remain_num_cw
                as UWORD16;
            *ptr_extended_sorted_code_book.offset(x_srt_sc_cnt as isize) = *ptr_sorted_cb
                .offset(srt_sec_cnt as isize);
            srt_sec_cnt = srt_sec_cnt.wrapping_add(1);
            remain_num_cw_sort_sec = *ptr_num_sorted_cw_in_sect
                .offset(srt_sec_cnt as isize) as UWORD32;
            in_segment_remain_num_cw = num_segment;
        } else {
            *ptr_num_ext_sort_cw_sect.offset(x_srt_sc_cnt as isize) = remain_num_cw_sort_sec
                as UWORD16;
            *ptr_extended_sorted_code_book.offset(x_srt_sc_cnt as isize) = *ptr_sorted_cb
                .offset(srt_sec_cnt as isize);
            in_segment_remain_num_cw = in_segment_remain_num_cw
                .wrapping_sub(remain_num_cw_sort_sec);
            srt_sec_cnt = srt_sec_cnt.wrapping_add(1);
            remain_num_cw_sort_sec = *ptr_num_sorted_cw_in_sect
                .offset(srt_sec_cnt as isize) as UWORD32;
        }
        *ptr_ext_sorted_sect_max_cb_len.offset(x_srt_sc_cnt as isize) = (if (*ptr_max_cw_len_tbl
            .offset(
                *ptr_extended_sorted_code_book.offset(x_srt_sc_cnt as isize) as isize,
            ) as core::ffi::c_int) < longest_cw_len as core::ffi::c_int
        {
            *ptr_max_cw_len_tbl
                .offset(
                    *ptr_extended_sorted_code_book.offset(x_srt_sc_cnt as isize) as isize,
                ) as core::ffi::c_int
        } else {
            longest_cw_len as core::ffi::c_int
        }) as UWORD8;
        x_srt_sc_cnt = x_srt_sc_cnt.wrapping_add(1 as UWORD32);
        if x_srt_sc_cnt >= (MAX_SFB_HCR + MAX_HCR_SETS) as UWORD32 {
            (*ptr_hcr_info).str_dec_io.err_log
                |= (ERROR_POS << 28 as core::ffi::c_int) as UWORD32;
            return;
        }
    }
    *ptr_num_ext_sort_cw_sect.offset(x_srt_sc_cnt as isize) = 0 as UWORD16;
}
unsafe extern "C" fn ixheaacd_hcr_prepare_segmentation_grid(
    mut ptr_hcr_info: *mut ia_hcr_info_struct,
) -> UWORD32 {
    let mut i: UWORD16 = 0;
    let mut j: UWORD16 = 0;
    let mut num_segment: UWORD16 = 0 as UWORD16;
    let mut segment_start: UWORD16 = 0 as UWORD16;
    let mut segment_width: UWORD8 = 0;
    let mut last_segment_width: UWORD8 = 0;
    let mut sorted_code_book: UWORD8 = 0;
    let mut end_flag: UWORD8 = 0 as UWORD8;
    let mut intermediate_result: UWORD16 = 0;
    let mut longest_cw_len: WORD8 = (*ptr_hcr_info).str_dec_io.longest_cw_len;
    let mut reordered_spec_data_len: WORD16 = (*ptr_hcr_info)
        .str_dec_io
        .reordered_spec_data_len;
    let mut num_sorted_section: UWORD32 = (*ptr_hcr_info).sect_info.num_sorted_section;
    let mut ptr_sorted_cb: *mut UWORD8 = ((*ptr_hcr_info).sect_info.ptr_sorted_cb)
        .as_mut_ptr();
    let mut ptr_num_sorted_cw_in_sect: *mut UWORD16 = ((*ptr_hcr_info)
        .sect_info
        .ptr_num_sorted_cw_in_sect)
        .as_mut_ptr();
    let mut arr_seg_start_l: *mut UWORD16 = ((*ptr_hcr_info)
        .str_segment_info
        .arr_seg_start_l)
        .as_mut_ptr();
    let mut arr_seg_start_r: *mut UWORD16 = ((*ptr_hcr_info)
        .str_segment_info
        .arr_seg_start_r)
        .as_mut_ptr();
    let mut p_remaining_bits_in_seg: *mut WORD8 = ((*ptr_hcr_info)
        .str_segment_info
        .p_remaining_bits_in_seg)
        .as_mut_ptr();
    let mut bit_str_idx: UWORD16 = (*ptr_hcr_info).str_dec_io.bit_str_idx;
    let mut ptr_max_cw_len_tbl: *const UWORD8 = (*ptr_hcr_info)
        .table_info
        .ptr_max_cw_len_tbl;
    i = num_sorted_section as UWORD16;
    while i as core::ffi::c_int != 0 as core::ffi::c_int {
        let fresh13 = ptr_sorted_cb;
        ptr_sorted_cb = ptr_sorted_cb.offset(1);
        sorted_code_book = *fresh13;
        segment_width = (if (*ptr_max_cw_len_tbl.offset(sorted_code_book as isize)
            as core::ffi::c_int) < longest_cw_len as core::ffi::c_int
        {
            *ptr_max_cw_len_tbl.offset(sorted_code_book as isize) as core::ffi::c_int
        } else {
            longest_cw_len as core::ffi::c_int
        }) as UWORD8;
        j = *ptr_num_sorted_cw_in_sect;
        while j as core::ffi::c_int != 0 as core::ffi::c_int {
            intermediate_result = (bit_str_idx as core::ffi::c_int
                + segment_start as core::ffi::c_int) as UWORD16;
            if segment_start as core::ffi::c_int + segment_width as core::ffi::c_int
                <= reordered_spec_data_len as core::ffi::c_int
            {
                let fresh14 = arr_seg_start_l;
                arr_seg_start_l = arr_seg_start_l.offset(1);
                *fresh14 = intermediate_result;
                let fresh15 = arr_seg_start_r;
                arr_seg_start_r = arr_seg_start_r.offset(1);
                *fresh15 = (intermediate_result as core::ffi::c_int
                    + segment_width as core::ffi::c_int - 1 as core::ffi::c_int)
                    as UWORD16;
                let fresh16 = p_remaining_bits_in_seg;
                p_remaining_bits_in_seg = p_remaining_bits_in_seg.offset(1);
                *fresh16 = segment_width as WORD8;
                segment_start = (segment_start as core::ffi::c_int
                    + segment_width as core::ffi::c_int) as UWORD16;
                num_segment = (num_segment as core::ffi::c_int + 1 as core::ffi::c_int)
                    as UWORD16;
                j = j.wrapping_sub(1);
            } else {
                arr_seg_start_l = arr_seg_start_l.offset(-1);
                arr_seg_start_r = arr_seg_start_r.offset(-1);
                p_remaining_bits_in_seg = p_remaining_bits_in_seg.offset(-1);
                segment_start = (*arr_seg_start_l as core::ffi::c_int
                    - bit_str_idx as core::ffi::c_int) as UWORD16;
                last_segment_width = (reordered_spec_data_len as core::ffi::c_int
                    - segment_start as core::ffi::c_int) as UWORD8;
                *p_remaining_bits_in_seg = last_segment_width as WORD8;
                *arr_seg_start_r = (bit_str_idx as core::ffi::c_int
                    + segment_start as core::ffi::c_int
                    + last_segment_width as core::ffi::c_int - 1 as core::ffi::c_int)
                    as UWORD16;
                end_flag = 1 as UWORD8;
                break;
            }
        }
        ptr_num_sorted_cw_in_sect = ptr_num_sorted_cw_in_sect.offset(1);
        if end_flag as core::ffi::c_int != 0 as core::ffi::c_int {
            break;
        }
        i = i.wrapping_sub(1);
    }
    if num_segment as core::ffi::c_int == 0 as core::ffi::c_int {
        (*ptr_hcr_info).str_dec_io.err_log
            |= (ERROR_POS << 9 as core::ffi::c_int) as UWORD32;
    }
    (*ptr_hcr_info).str_segment_info.num_segment = num_segment as WORD32;
    return (*ptr_hcr_info).str_dec_io.err_log;
}
#[inline]
unsafe extern "C" fn ixheaacd_huff_dec_pair_hcr_pcw(
    mut ptr_hcr_info: *mut ia_hcr_info_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut no_bands: WORD,
    mut code_book_tbl: *const UWORD16,
    mut read_word: *mut WORD32,
    mut tbl_sign: WORD32,
    mut idx_table: *const UWORD32,
    mut arr_seg_start_l: *mut UWORD16,
    mut read_bits: *mut WORD32,
    mut huff_mode: WORD32,
    mut p_remaining_bits_in_seg: *mut WORD8,
    mut ptr_num_decoded_bits: *mut WORD32,
) -> *mut UWORD16 {
    let mut spec_index: WORD32 = (*ptr_hcr_info).str_dec_io.quant_spec_coeff_idx;
    let mut spec_coef: *mut WORD32 = ((*ptr_hcr_info)
        .str_dec_io
        .ptr_quant_spec_coeff_base)
        .offset(spec_index as isize);
    let mut index: WORD16 = 0;
    let mut length: WORD16 = 0;
    let mut y: WORD32 = 0;
    let mut z: WORD32 = 0;
    let mut ptr_read_next: *mut UWORD8 = (*it_bit_buff).ptr_read_next;
    let mut bit_pos: *mut WORD32 = &mut (*it_bit_buff).bit_pos;
    loop {
        let mut read_word1: UWORD32 = 0;
        let mut read_bit_offset: WORD32 = *arr_seg_start_l as WORD32
            - ((*it_bit_buff).size - *read_bits);
        if read_bit_offset != 0 {
            *read_bits -= read_bit_offset;
            *bit_pos += read_bit_offset;
            ixheaacd_aac_read_byte_corr(
                &mut ptr_read_next,
                bit_pos,
                read_word,
                (*it_bit_buff).ptr_bit_buf_end,
            );
        }
        *bit_pos = (if 0 as core::ffi::c_int > *bit_pos {
            0 as core::ffi::c_int
        } else {
            *bit_pos
        }) as WORD32;
        read_word1 = (*read_word << *bit_pos) as UWORD32;
        ixheaacd_huffman_decode(
            read_word1 as WORD32,
            &mut index,
            &mut length,
            code_book_tbl,
            idx_table,
        );
        *bit_pos += length as core::ffi::c_int;
        *ptr_num_decoded_bits += length as core::ffi::c_int;
        *p_remaining_bits_in_seg = (*p_remaining_bits_in_seg as core::ffi::c_int
            - length as core::ffi::c_int) as WORD8;
        *arr_seg_start_l = (*arr_seg_start_l as core::ffi::c_int
            + length as core::ffi::c_int) as UWORD16;
        *read_bits -= length as core::ffi::c_int;
        ixheaacd_aac_read_byte_corr(
            &mut ptr_read_next,
            bit_pos,
            read_word,
            (*it_bit_buff).ptr_bit_buf_end,
        );
        if tbl_sign != 0 {
            let mut temp_word: WORD32 = 0;
            temp_word = *read_word << *bit_pos;
            y = index as WORD32 / huff_mode;
            z = index as WORD32 - huff_mode * y;
            if y != 0 {
                if temp_word as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0
                {
                    y = -y;
                }
                temp_word = temp_word << 1 as core::ffi::c_int;
                *bit_pos += 1 as core::ffi::c_int;
                *p_remaining_bits_in_seg = (*p_remaining_bits_in_seg as core::ffi::c_int
                    - 1 as core::ffi::c_int) as WORD8;
                *ptr_num_decoded_bits += 1 as core::ffi::c_int;
                *arr_seg_start_l = (*arr_seg_start_l as core::ffi::c_int
                    + 1 as core::ffi::c_int) as UWORD16;
                *read_bits -= 1 as core::ffi::c_int;
            }
            let fresh17 = spec_coef;
            spec_coef = spec_coef.offset(1);
            *fresh17 = y;
            spec_index += 1;
            if z != 0 {
                if temp_word as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0
                {
                    z = -z;
                }
                temp_word <<= 1 as core::ffi::c_int;
                *bit_pos += 1 as core::ffi::c_int;
                *p_remaining_bits_in_seg = (*p_remaining_bits_in_seg as core::ffi::c_int
                    - 1 as core::ffi::c_int) as WORD8;
                *ptr_num_decoded_bits += 1 as core::ffi::c_int;
                *arr_seg_start_l = (*arr_seg_start_l as core::ffi::c_int
                    + 1 as core::ffi::c_int) as UWORD16;
                *read_bits -= 1 as core::ffi::c_int;
            }
            let fresh18 = spec_coef;
            spec_coef = spec_coef.offset(1);
            *fresh18 = z;
            spec_index += 1;
        } else {
            y = (index as core::ffi::c_int / huff_mode as core::ffi::c_int
                - 4 as core::ffi::c_int) as WORD32;
            z = (index as core::ffi::c_int
                - (y as core::ffi::c_int + 4 as core::ffi::c_int)
                    * huff_mode as core::ffi::c_int - 4 as core::ffi::c_int) as WORD32;
            let fresh19 = spec_coef;
            spec_coef = spec_coef.offset(1);
            *fresh19 = y;
            let fresh20 = spec_coef;
            spec_coef = spec_coef.offset(1);
            *fresh20 = z;
            spec_index += 2 as core::ffi::c_int;
        }
        ixheaacd_aac_read_byte_corr(
            &mut ptr_read_next,
            bit_pos,
            read_word,
            (*it_bit_buff).ptr_bit_buf_end,
        );
        no_bands -= 1;
        arr_seg_start_l = arr_seg_start_l.offset(1);
        p_remaining_bits_in_seg = p_remaining_bits_in_seg.offset(1);
        if !(no_bands != 0 as core::ffi::c_int) {
            break;
        }
    }
    (*it_bit_buff).ptr_read_next = ptr_read_next;
    (*ptr_hcr_info).str_dec_io.quant_spec_coeff_idx = spec_index;
    return arr_seg_start_l;
}
#[inline]
unsafe extern "C" fn ixheaacd_huff_dec_pair_hcr_non_pcw(
    mut itt_bit_buff: *mut ia_bit_buf_struct,
    mut spec_coef: *mut WORD32,
    mut code_book_tbl: *const UWORD16,
    mut tbl_sign: WORD32,
    mut idx_table: *const UWORD32,
    mut huff_mode: WORD32,
) -> WORD16 {
    let mut index: WORD16 = 0;
    let mut length: WORD16 = 0;
    let mut y: WORD32 = 0;
    let mut z: WORD32 = 0;
    let mut read_word1: WORD32 = 0;
    let mut read_word: WORD32 = 0;
    read_word = ixheaacd_aac_showbits_32(
        (*itt_bit_buff).byte_ptr,
        (*itt_bit_buff).bit_count,
        0 as *mut WORD32,
    ) as WORD32;
    ixheaacd_huffman_decode(
        read_word,
        &mut index,
        &mut length,
        code_book_tbl,
        idx_table,
    );
    read_word1 = read_word << length as core::ffi::c_int;
    if tbl_sign != 0 {
        let mut temp_word: WORD32 = 0;
        temp_word = read_word1;
        y = index as WORD32 / huff_mode;
        z = index as WORD32 - huff_mode * y;
        if y != 0 {
            if temp_word as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0 {
                y = -y;
            }
            temp_word = temp_word << 1 as core::ffi::c_int;
            length += 1;
        }
        let fresh21 = spec_coef;
        spec_coef = spec_coef.offset(1);
        *fresh21 = y;
        if z != 0 {
            if temp_word as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0 {
                z = -z;
            }
            temp_word <<= 1 as core::ffi::c_int;
            length += 1;
        }
        let fresh22 = spec_coef;
        spec_coef = spec_coef.offset(1);
        *fresh22 = z;
    } else {
        y = (index as core::ffi::c_int / huff_mode as core::ffi::c_int
            - 4 as core::ffi::c_int) as WORD32;
        z = (index as core::ffi::c_int
            - (y as core::ffi::c_int + 4 as core::ffi::c_int)
                * huff_mode as core::ffi::c_int - 4 as core::ffi::c_int) as WORD32;
        let fresh23 = spec_coef;
        spec_coef = spec_coef.offset(1);
        *fresh23 = y;
        let fresh24 = spec_coef;
        spec_coef = spec_coef.offset(1);
        *fresh24 = z;
    }
    return length;
}
#[inline]
unsafe extern "C" fn ixheaacd_huff_dec_quad_hcr_pcw(
    mut ptr_hcr_info: *mut ia_hcr_info_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut no_bands: WORD,
    mut code_book_tbl: *const UWORD16,
    mut tbl_sign: WORD32,
    mut idx_table: *const UWORD32,
    mut read_word: *mut WORD32,
    mut read_bits: *mut WORD32,
    mut arr_seg_start_l: *mut UWORD16,
    mut p_remaining_bits_in_seg: *mut WORD8,
    mut ptr_num_decoded_bits: *mut WORD32,
) -> *mut UWORD16 {
    let mut index: WORD16 = 0;
    let mut length: WORD16 = 0;
    let mut ptr_read_next: *mut UWORD8 = (*it_bit_buff).ptr_read_next;
    let mut spec_index: WORD32 = (*ptr_hcr_info).str_dec_io.quant_spec_coeff_idx;
    let mut spec_coef: *mut WORD32 = ((*ptr_hcr_info)
        .str_dec_io
        .ptr_quant_spec_coeff_base)
        .offset(spec_index as isize);
    let mut bit_pos: *mut WORD32 = &mut (*it_bit_buff).bit_pos;
    loop {
        let mut read_word1: UWORD32 = 0;
        let mut read_bit_offset: WORD32 = *arr_seg_start_l as WORD32
            - ((*it_bit_buff).size - *read_bits);
        if read_bit_offset != 0 {
            *read_bits -= read_bit_offset;
            *bit_pos += read_bit_offset;
            ixheaacd_aac_read_byte_corr(
                &mut ptr_read_next,
                bit_pos,
                read_word,
                (*it_bit_buff).ptr_bit_buf_end,
            );
        }
        *bit_pos = (if 0 as core::ffi::c_int > *bit_pos {
            0 as core::ffi::c_int
        } else {
            *bit_pos
        }) as WORD32;
        read_word1 = (*read_word << *bit_pos) as UWORD32;
        ixheaacd_huffman_decode(
            read_word1 as WORD32,
            &mut index,
            &mut length,
            code_book_tbl,
            idx_table,
        );
        *bit_pos += length as core::ffi::c_int;
        *p_remaining_bits_in_seg = (*p_remaining_bits_in_seg as core::ffi::c_int
            - length as core::ffi::c_int) as WORD8;
        *read_bits -= length as core::ffi::c_int;
        *ptr_num_decoded_bits += length as core::ffi::c_int;
        *arr_seg_start_l = (*arr_seg_start_l as core::ffi::c_int
            + length as core::ffi::c_int) as UWORD16;
        ixheaacd_aac_read_byte_corr(
            &mut ptr_read_next,
            bit_pos,
            read_word,
            (*it_bit_buff).ptr_bit_buf_end,
        );
        if tbl_sign != 0 {
            let mut temp_word: WORD32 = 0;
            let mut w: WORD32 = 0;
            let mut x: WORD32 = 0;
            let mut y: WORD32 = 0;
            let mut z: WORD32 = 0;
            temp_word = *read_word << *bit_pos;
            w = (index as core::ffi::c_int / 27 as core::ffi::c_int) as WORD32;
            index = (index as core::ffi::c_int
                - w as core::ffi::c_int * 27 as core::ffi::c_int) as WORD16;
            x = (index as core::ffi::c_int / 9 as core::ffi::c_int) as WORD32;
            index = (index as core::ffi::c_int
                - x as core::ffi::c_int * 9 as core::ffi::c_int) as WORD16;
            y = (index as core::ffi::c_int / 3 as core::ffi::c_int) as WORD32;
            z = (index as core::ffi::c_int
                - y as core::ffi::c_int * 3 as core::ffi::c_int) as WORD32;
            if w != 0 {
                if temp_word as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0
                {
                    w = -w;
                }
                temp_word <<= 1 as core::ffi::c_int;
                *bit_pos += 1 as core::ffi::c_int;
                *p_remaining_bits_in_seg = (*p_remaining_bits_in_seg as core::ffi::c_int
                    - 1 as core::ffi::c_int) as WORD8;
                *read_bits -= 1 as core::ffi::c_int;
                *ptr_num_decoded_bits += 1 as core::ffi::c_int;
                *arr_seg_start_l = (*arr_seg_start_l as core::ffi::c_int
                    + 1 as core::ffi::c_int) as UWORD16;
            }
            let fresh25 = spec_coef;
            spec_coef = spec_coef.offset(1);
            *fresh25 = w;
            spec_index += 1;
            if x != 0 {
                if temp_word as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0
                {
                    x = -x;
                }
                temp_word <<= 1 as core::ffi::c_int;
                *bit_pos += 1 as core::ffi::c_int;
                *p_remaining_bits_in_seg = (*p_remaining_bits_in_seg as core::ffi::c_int
                    - 1 as core::ffi::c_int) as WORD8;
                *read_bits -= 1 as core::ffi::c_int;
                *ptr_num_decoded_bits += 1 as core::ffi::c_int;
                *arr_seg_start_l = (*arr_seg_start_l as core::ffi::c_int
                    + 1 as core::ffi::c_int) as UWORD16;
            }
            let fresh26 = spec_coef;
            spec_coef = spec_coef.offset(1);
            *fresh26 = x;
            spec_index += 1;
            if y != 0 {
                if temp_word as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0
                {
                    y = -y;
                }
                temp_word <<= 1 as core::ffi::c_int;
                *bit_pos += 1 as core::ffi::c_int;
                *p_remaining_bits_in_seg = (*p_remaining_bits_in_seg as core::ffi::c_int
                    - 1 as core::ffi::c_int) as WORD8;
                *read_bits -= 1 as core::ffi::c_int;
                *ptr_num_decoded_bits += 1 as core::ffi::c_int;
                *arr_seg_start_l = (*arr_seg_start_l as core::ffi::c_int
                    + 1 as core::ffi::c_int) as UWORD16;
            }
            let fresh27 = spec_coef;
            spec_coef = spec_coef.offset(1);
            *fresh27 = y;
            spec_index += 1;
            if z != 0 {
                if temp_word as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0
                {
                    z = -z;
                }
                temp_word <<= 1 as core::ffi::c_int;
                *bit_pos += 1 as core::ffi::c_int;
                *p_remaining_bits_in_seg = (*p_remaining_bits_in_seg as core::ffi::c_int
                    - 1 as core::ffi::c_int) as WORD8;
                *read_bits -= 1 as core::ffi::c_int;
                *ptr_num_decoded_bits += 1 as core::ffi::c_int;
                *arr_seg_start_l = (*arr_seg_start_l as core::ffi::c_int
                    + 1 as core::ffi::c_int) as UWORD16;
            }
            let fresh28 = spec_coef;
            spec_coef = spec_coef.offset(1);
            *fresh28 = z;
            spec_index += 1;
        } else {
            let mut w_0: WORD32 = 0;
            let mut x_0: WORD32 = 0;
            let mut y_0: WORD32 = 0;
            let mut z_0: WORD32 = 0;
            w_0 = (index as core::ffi::c_int / 27 as core::ffi::c_int
                - 1 as core::ffi::c_int) as WORD32;
            index = (index as core::ffi::c_int
                - (w_0 as core::ffi::c_int + 1 as core::ffi::c_int)
                    * 27 as core::ffi::c_int) as WORD16;
            x_0 = (index as core::ffi::c_int / 9 as core::ffi::c_int
                - 1 as core::ffi::c_int) as WORD32;
            index = (index as core::ffi::c_int
                - (x_0 as core::ffi::c_int + 1 as core::ffi::c_int)
                    * 9 as core::ffi::c_int) as WORD16;
            y_0 = (index as core::ffi::c_int / 3 as core::ffi::c_int
                - 1 as core::ffi::c_int) as WORD32;
            z_0 = (index as core::ffi::c_int
                - (y_0 as core::ffi::c_int + 1 as core::ffi::c_int)
                    * 3 as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
            let fresh29 = spec_coef;
            spec_coef = spec_coef.offset(1);
            *fresh29 = w_0;
            let fresh30 = spec_coef;
            spec_coef = spec_coef.offset(1);
            *fresh30 = x_0;
            let fresh31 = spec_coef;
            spec_coef = spec_coef.offset(1);
            *fresh31 = y_0;
            let fresh32 = spec_coef;
            spec_coef = spec_coef.offset(1);
            *fresh32 = z_0;
            spec_index += 4 as core::ffi::c_int;
        }
        ixheaacd_aac_read_byte_corr(
            &mut ptr_read_next,
            bit_pos,
            read_word,
            (*it_bit_buff).ptr_bit_buf_end,
        );
        arr_seg_start_l = arr_seg_start_l.offset(1);
        p_remaining_bits_in_seg = p_remaining_bits_in_seg.offset(1);
        no_bands -= 1;
        if !(no_bands != 0 as core::ffi::c_int) {
            break;
        }
    }
    (*it_bit_buff).ptr_read_next = ptr_read_next;
    (*ptr_hcr_info).str_dec_io.quant_spec_coeff_idx = spec_index;
    return arr_seg_start_l;
}
unsafe extern "C" fn ixheaacd_huff_dec_word_hcr_pcw(
    mut ptr_hcr_info: *mut ia_hcr_info_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut no_bands: WORD,
    mut code_book_tbl: *const UWORD16,
    mut read_word: *mut WORD32,
    mut idx_table: *const UWORD32,
    mut arr_seg_start_l: *mut UWORD16,
    mut read_bits: *mut WORD32,
    mut p_remaining_bits_in_seg: *mut WORD8,
    mut ptr_num_decoded_bits: *mut WORD32,
) -> *mut UWORD16 {
    let mut sp1: WORD32 = 0;
    let mut sp2: WORD32 = 0;
    let mut flush_cw: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut value: WORD32 = 0;
    let mut norm_val: WORD32 = 0;
    let mut off: WORD32 = 0;
    let mut out1: WORD32 = 0;
    let mut out2: WORD32 = 0;
    let mut index: WORD16 = 0;
    let mut length: WORD32 = 0;
    let mut ptr_read_next: *mut UWORD8 = (*it_bit_buff).ptr_read_next;
    let mut spec_index: WORD32 = (*ptr_hcr_info).str_dec_io.quant_spec_coeff_idx;
    let mut spec_coef: *mut WORD32 = ((*ptr_hcr_info)
        .str_dec_io
        .ptr_quant_spec_coeff_base)
        .offset(spec_index as isize);
    let mut bit_pos: *mut WORD32 = &mut (*it_bit_buff).bit_pos;
    loop {
        let mut read_word1: UWORD32 = 0;
        let mut read_bit_offset: WORD32 = *arr_seg_start_l as WORD32
            - ((*it_bit_buff).size - *read_bits);
        if read_bit_offset != 0 {
            *read_bits -= read_bit_offset;
            *bit_pos += read_bit_offset;
            ixheaacd_aac_read_byte_corr1(
                &mut ptr_read_next,
                bit_pos,
                read_word,
                (*it_bit_buff).ptr_bit_buf_end,
            );
        }
        *bit_pos = (if 0 as core::ffi::c_int > *bit_pos {
            0 as core::ffi::c_int
        } else {
            *bit_pos
        }) as WORD32;
        read_word1 = (*read_word << *bit_pos) as UWORD32;
        ixheaacd_huff_sfb_table(
            read_word1 as WORD32,
            &mut index,
            &mut length,
            code_book_tbl,
            idx_table,
        );
        *bit_pos += length;
        *read_bits -= length;
        *arr_seg_start_l = (*arr_seg_start_l as core::ffi::c_int
            + length as core::ffi::c_int) as UWORD16;
        *p_remaining_bits_in_seg = (*p_remaining_bits_in_seg as core::ffi::c_int
            - length as core::ffi::c_int) as WORD8;
        *ptr_num_decoded_bits += length;
        ixheaacd_aac_read_byte_corr(
            &mut ptr_read_next,
            bit_pos,
            read_word,
            (*it_bit_buff).ptr_bit_buf_end,
        );
        out1 = (index as core::ffi::c_int / 17 as core::ffi::c_int) as WORD32;
        out2 = (index as core::ffi::c_int
            - out1 as core::ffi::c_int * 17 as core::ffi::c_int) as WORD32;
        flush_cw = *read_word << *bit_pos;
        sp1 = out1;
        sp2 = out2;
        if out1 != 0 {
            if flush_cw as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0 {
                out1 = -out1;
            }
            *bit_pos += 1 as core::ffi::c_int;
            *read_bits -= 1 as core::ffi::c_int;
            *p_remaining_bits_in_seg = (*p_remaining_bits_in_seg as core::ffi::c_int
                - 1 as core::ffi::c_int) as WORD8;
            *ptr_num_decoded_bits += 1 as core::ffi::c_int;
            *arr_seg_start_l = (*arr_seg_start_l as core::ffi::c_int
                + 1 as core::ffi::c_int) as UWORD16;
            flush_cw = flush_cw << 1 as core::ffi::c_int;
        }
        if out2 != 0 {
            *bit_pos += 1 as core::ffi::c_int;
            *read_bits -= 1 as core::ffi::c_int;
            *p_remaining_bits_in_seg = (*p_remaining_bits_in_seg as core::ffi::c_int
                - 1 as core::ffi::c_int) as WORD8;
            *ptr_num_decoded_bits += 1 as core::ffi::c_int;
            *arr_seg_start_l = (*arr_seg_start_l as core::ffi::c_int
                + 1 as core::ffi::c_int) as UWORD16;
            if flush_cw as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0 {
                out2 = -out2;
            }
        }
        ixheaacd_aac_read_byte_corr(
            &mut ptr_read_next,
            bit_pos,
            read_word,
            (*it_bit_buff).ptr_bit_buf_end,
        );
        if sp1 == 16 as core::ffi::c_int {
            i = 4 as core::ffi::c_int as WORD32;
            value = ixheaac_extu(*read_word as UWORD32, *bit_pos, 23 as WORD32)
                as WORD32;
            value = (value as core::ffi::c_uint | 0xfffffe00 as core::ffi::c_uint)
                as WORD32;
            norm_val = ixheaac_norm32(value) as WORD32;
            i += norm_val as core::ffi::c_int - 22 as core::ffi::c_int;
            *bit_pos += norm_val as core::ffi::c_int - 21 as core::ffi::c_int;
            *p_remaining_bits_in_seg = (*p_remaining_bits_in_seg as core::ffi::c_int
                - (norm_val as core::ffi::c_int - 21 as core::ffi::c_int)) as WORD8;
            *ptr_num_decoded_bits
                += norm_val as core::ffi::c_int - 21 as core::ffi::c_int;
            *read_bits -= norm_val as core::ffi::c_int - 21 as core::ffi::c_int;
            *arr_seg_start_l = (*arr_seg_start_l as core::ffi::c_int
                + (norm_val as core::ffi::c_int - 21 as core::ffi::c_int)) as UWORD16;
            ixheaacd_aac_read_byte_corr(
                &mut ptr_read_next,
                bit_pos,
                read_word,
                (*it_bit_buff).ptr_bit_buf_end,
            );
            off = ixheaac_extu(*read_word as UWORD32, *bit_pos, 32 as WORD32 - i)
                as WORD32;
            *bit_pos += i;
            *p_remaining_bits_in_seg = (*p_remaining_bits_in_seg as core::ffi::c_int
                - i as core::ffi::c_int) as WORD8;
            *ptr_num_decoded_bits += i;
            *read_bits -= i;
            *arr_seg_start_l = (*arr_seg_start_l as core::ffi::c_int
                + i as core::ffi::c_int) as UWORD16;
            ixheaacd_aac_read_byte_corr(
                &mut ptr_read_next,
                bit_pos,
                read_word,
                (*it_bit_buff).ptr_bit_buf_end,
            );
            ixheaacd_aac_read_byte_corr(
                &mut ptr_read_next,
                bit_pos,
                read_word,
                (*it_bit_buff).ptr_bit_buf_end,
            );
            i = off + ((1 as core::ffi::c_int) << i);
            if out1 < 0 as core::ffi::c_int {
                let fresh33 = spec_coef;
                spec_coef = spec_coef.offset(1);
                *fresh33 = -i;
            } else {
                let fresh34 = spec_coef;
                spec_coef = spec_coef.offset(1);
                *fresh34 = i;
            }
            spec_index += 1;
        } else {
            let fresh35 = spec_coef;
            spec_coef = spec_coef.offset(1);
            *fresh35 = out1;
            spec_index += 1;
        }
        if sp2 == 16 as core::ffi::c_int {
            i = 4 as core::ffi::c_int as WORD32;
            value = ixheaac_extu(*read_word as UWORD32, *bit_pos, 23 as WORD32)
                as WORD32;
            value = (value as core::ffi::c_uint | 0xfffffe00 as core::ffi::c_uint)
                as WORD32;
            norm_val = ixheaac_norm32(value) as WORD32;
            i += norm_val as core::ffi::c_int - 22 as core::ffi::c_int;
            *bit_pos += norm_val as core::ffi::c_int - 21 as core::ffi::c_int;
            *read_bits -= norm_val as core::ffi::c_int - 21 as core::ffi::c_int;
            *p_remaining_bits_in_seg = (*p_remaining_bits_in_seg as core::ffi::c_int
                - (norm_val as core::ffi::c_int - 21 as core::ffi::c_int)) as WORD8;
            *ptr_num_decoded_bits
                += norm_val as core::ffi::c_int - 21 as core::ffi::c_int;
            *arr_seg_start_l = (*arr_seg_start_l as core::ffi::c_int
                + (norm_val as core::ffi::c_int - 21 as core::ffi::c_int)) as UWORD16;
            ixheaacd_aac_read_byte_corr(
                &mut ptr_read_next,
                bit_pos,
                read_word,
                (*it_bit_buff).ptr_bit_buf_end,
            );
            off = ixheaac_extu(*read_word as UWORD32, *bit_pos, 32 as WORD32 - i)
                as WORD32;
            *bit_pos += i;
            *p_remaining_bits_in_seg = (*p_remaining_bits_in_seg as core::ffi::c_int
                - i as core::ffi::c_int) as WORD8;
            *ptr_num_decoded_bits += i;
            *read_bits -= i;
            *arr_seg_start_l = (*arr_seg_start_l as core::ffi::c_int
                + i as core::ffi::c_int) as UWORD16;
            ixheaacd_aac_read_byte_corr(
                &mut ptr_read_next,
                bit_pos,
                read_word,
                (*it_bit_buff).ptr_bit_buf_end,
            );
            ixheaacd_aac_read_byte_corr(
                &mut ptr_read_next,
                bit_pos,
                read_word,
                (*it_bit_buff).ptr_bit_buf_end,
            );
            i = off + ((1 as core::ffi::c_int) << i);
            if out2 < 0 as core::ffi::c_int {
                let fresh36 = spec_coef;
                spec_coef = spec_coef.offset(1);
                *fresh36 = -i;
            } else {
                let fresh37 = spec_coef;
                spec_coef = spec_coef.offset(1);
                *fresh37 = i;
            }
            spec_index += 1;
        } else {
            let fresh38 = spec_coef;
            spec_coef = spec_coef.offset(1);
            *fresh38 = out2;
            spec_index += 1;
        }
        arr_seg_start_l = arr_seg_start_l.offset(1);
        p_remaining_bits_in_seg = p_remaining_bits_in_seg.offset(1);
        no_bands -= 1;
        if !(no_bands != 0 as core::ffi::c_int) {
            break;
        }
    }
    (*it_bit_buff).ptr_read_next = ptr_read_next;
    (*ptr_hcr_info).str_dec_io.quant_spec_coeff_idx = spec_index;
    return arr_seg_start_l;
}
unsafe extern "C" fn ixheaacd_decode_pcw(
    mut itt_bit_buff: *mut ia_bit_buf_struct,
    mut ptr_hcr_info: *mut ia_hcr_info_struct,
    mut ptr_aac_tables: *mut ia_aac_dec_tables_struct,
) -> VOID {
    let mut ext_sort_sec: UWORD16 = 0;
    let mut cur_ext_sort_cw_sec: UWORD16 = 0;
    let mut codebook: UWORD8 = 0;
    let mut dimension: UWORD8 = 0;
    let mut increment: WORD32 = 0;
    let mut num_ext_sorted_cw_in_sect_idx: WORD32 = (*ptr_hcr_info)
        .sect_info
        .num_ext_sorted_cw_in_sect_idx;
    let mut ptr_ext_sorted_cw: *mut UWORD8 = ((*ptr_hcr_info)
        .sect_info
        .ptr_ext_sorted_cw)
        .as_mut_ptr();
    let mut ext_sorted_cw_idx: WORD32 = (*ptr_hcr_info).sect_info.ext_sorted_cw_idx;
    let mut ptr_num_ext_sorted_sect_in_sets: *mut UWORD16 = ((*ptr_hcr_info)
        .sect_info
        .ptr_num_ext_sorted_sect_in_sets)
        .as_mut_ptr();
    let mut num_ext_sorted_sect_in_sets_idx: WORD32 = (*ptr_hcr_info)
        .sect_info
        .num_ext_sorted_sect_in_sets_idx;
    let mut ptr_quant_spec_coeff: *mut WORD32 = (*ptr_hcr_info)
        .str_dec_io
        .ptr_quant_spec_coeff_base;
    let mut arr_seg_start_l: *mut UWORD16 = ((*ptr_hcr_info)
        .str_segment_info
        .arr_seg_start_l)
        .as_mut_ptr();
    let mut p_remaining_bits_in_seg: *mut WORD8 = ((*ptr_hcr_info)
        .str_segment_info
        .p_remaining_bits_in_seg)
        .as_mut_ptr();
    let mut ptr_ext_sorted_sect_max_cb_len: *mut UWORD8 = ((*ptr_hcr_info)
        .sect_info
        .ptr_ext_sorted_sect_max_cb_len)
        .as_mut_ptr();
    let mut ext_sorted_sect_max_cb_len_idx: WORD32 = (*ptr_hcr_info)
        .sect_info
        .ext_sorted_sect_max_cb_len_idx;
    let mut max_allowed_cw_len: UWORD8 = 0;
    let mut num_decoded_bits: WORD32 = 0;
    let mut ptr_cb_dimension_tbl: *const UWORD8 = (*ptr_hcr_info)
        .table_info
        .ptr_cb_dimension_tbl;
    let mut read_word: WORD32 = ixheaacd_aac_showbits_32(
        (*itt_bit_buff).ptr_read_next,
        (*itt_bit_buff).cnt_bits,
        &mut increment,
    ) as WORD32;
    let mut read_bits: WORD32 = (*itt_bit_buff).cnt_bits;
    (*itt_bit_buff).ptr_read_next = ((*itt_bit_buff).ptr_read_next)
        .offset(increment as isize);
    ext_sort_sec = *ptr_num_ext_sorted_sect_in_sets
        .offset(num_ext_sorted_sect_in_sets_idx as isize);
    while ext_sort_sec as core::ffi::c_int != 0 as core::ffi::c_int {
        codebook = *ptr_ext_sorted_cw.offset(ext_sorted_cw_idx as isize);
        if codebook as core::ffi::c_int <= 0 as core::ffi::c_int {
            return;
        }
        ext_sorted_cw_idx += 1;
        if ext_sorted_cw_idx >= MAX_SFB_HCR + MAX_HCR_SETS {
            return;
        }
        dimension = *ptr_cb_dimension_tbl.offset(codebook as isize);
        max_allowed_cw_len = *ptr_ext_sorted_sect_max_cb_len
            .offset(ext_sorted_sect_max_cb_len_idx as isize);
        ext_sorted_sect_max_cb_len_idx += 1;
        if ext_sorted_sect_max_cb_len_idx >= MAX_SFB_HCR + MAX_HCR_SETS {
            return;
        }
        if codebook as core::ffi::c_int <= 4 as core::ffi::c_int {
            let mut tbl_sign: WORD32 = 0 as WORD32;
            let mut cb_table: *const UWORD16 = (*ptr_aac_tables)
                .code_book[codebook as usize];
            let mut idx_table: *const UWORD32 = (*ptr_aac_tables)
                .index_table[codebook as usize];
            if codebook as core::ffi::c_int > 2 as core::ffi::c_int {
                tbl_sign = 1 as core::ffi::c_int as WORD32;
            }
            num_decoded_bits = 0 as core::ffi::c_int as WORD32;
            cur_ext_sort_cw_sec = (*ptr_hcr_info)
                .sect_info
                .ptr_num_ext_sorted_cw_in_sect[num_ext_sorted_cw_in_sect_idx as usize];
            arr_seg_start_l = ixheaacd_huff_dec_quad_hcr_pcw(
                ptr_hcr_info,
                itt_bit_buff,
                cur_ext_sort_cw_sec as WORD,
                cb_table,
                tbl_sign,
                idx_table,
                &mut read_word,
                &mut read_bits,
                arr_seg_start_l,
                p_remaining_bits_in_seg,
                &mut num_decoded_bits,
            );
            p_remaining_bits_in_seg = p_remaining_bits_in_seg
                .offset(cur_ext_sort_cw_sec as core::ffi::c_int as isize);
            if (cur_ext_sort_cw_sec as core::ffi::c_int
                * max_allowed_cw_len as core::ffi::c_int) < num_decoded_bits
            {
                (*ptr_hcr_info).str_dec_io.err_log
                    |= (ERROR_POS << 19 as core::ffi::c_int) as UWORD32;
            }
            if 1 as core::ffi::c_int
                == ixheaacd_err_detect_pcw_segment(
                    *p_remaining_bits_in_seg,
                    ptr_hcr_info,
                    PCW,
                    ptr_quant_spec_coeff
                        .offset((*ptr_hcr_info).str_dec_io.quant_spec_coeff_idx as isize)
                        .offset(-(dimension as core::ffi::c_int as isize)),
                    dimension,
                ) as core::ffi::c_int
            {
                return;
            }
        } else if (codebook as core::ffi::c_int) < 11 as core::ffi::c_int {
            let mut tbl_sign_0: WORD32 = 0 as WORD32;
            let mut huff_mode: WORD32 = 9 as WORD32;
            let mut cb_table_0: *const UWORD16 = (*ptr_aac_tables)
                .code_book[codebook as usize];
            let mut idx_table_0: *const UWORD32 = (*ptr_aac_tables)
                .index_table[codebook as usize];
            num_decoded_bits = 0 as core::ffi::c_int as WORD32;
            if codebook as core::ffi::c_int > 6 as core::ffi::c_int {
                if codebook as core::ffi::c_int > 8 as core::ffi::c_int {
                    huff_mode = 13 as core::ffi::c_int as WORD32;
                } else {
                    huff_mode = 8 as core::ffi::c_int as WORD32;
                }
                tbl_sign_0 = 1 as core::ffi::c_int as WORD32;
            }
            cur_ext_sort_cw_sec = (*ptr_hcr_info)
                .sect_info
                .ptr_num_ext_sorted_cw_in_sect[num_ext_sorted_cw_in_sect_idx as usize];
            arr_seg_start_l = ixheaacd_huff_dec_pair_hcr_pcw(
                ptr_hcr_info,
                itt_bit_buff,
                cur_ext_sort_cw_sec as WORD,
                cb_table_0,
                &mut read_word,
                tbl_sign_0,
                idx_table_0,
                arr_seg_start_l,
                &mut read_bits,
                huff_mode,
                p_remaining_bits_in_seg,
                &mut num_decoded_bits,
            );
            p_remaining_bits_in_seg = p_remaining_bits_in_seg
                .offset(cur_ext_sort_cw_sec as core::ffi::c_int as isize);
            if (cur_ext_sort_cw_sec as core::ffi::c_int
                * max_allowed_cw_len as core::ffi::c_int) < num_decoded_bits
            {
                (*ptr_hcr_info).str_dec_io.err_log
                    |= (ERROR_POS << 18 as core::ffi::c_int) as UWORD32;
            }
            if 1 as core::ffi::c_int
                == ixheaacd_err_detect_pcw_segment(
                    *p_remaining_bits_in_seg,
                    ptr_hcr_info,
                    PCW_SIGN,
                    ptr_quant_spec_coeff
                        .offset((*ptr_hcr_info).str_dec_io.quant_spec_coeff_idx as isize)
                        .offset(-(dimension as core::ffi::c_int as isize)),
                    dimension,
                ) as core::ffi::c_int
            {
                return;
            }
        } else if codebook as core::ffi::c_int >= 11 as core::ffi::c_int {
            let mut idx_table_1: *const UWORD32 = ((*(*ptr_aac_tables)
                .pstr_huffmann_tables)
                .idx_table_hf11)
                .as_mut_ptr();
            let mut cb_table_1: *const UWORD16 = ((*(*ptr_aac_tables)
                .pstr_huffmann_tables)
                .input_table_cb11)
                .as_mut_ptr();
            num_decoded_bits = 0 as core::ffi::c_int as WORD32;
            cur_ext_sort_cw_sec = (*ptr_hcr_info)
                .sect_info
                .ptr_num_ext_sorted_cw_in_sect[num_ext_sorted_cw_in_sect_idx as usize];
            arr_seg_start_l = ixheaacd_huff_dec_word_hcr_pcw(
                ptr_hcr_info,
                itt_bit_buff,
                cur_ext_sort_cw_sec as WORD,
                cb_table_1,
                &mut read_word,
                idx_table_1,
                arr_seg_start_l,
                &mut read_bits,
                p_remaining_bits_in_seg,
                &mut num_decoded_bits,
            );
            p_remaining_bits_in_seg = p_remaining_bits_in_seg
                .offset(cur_ext_sort_cw_sec as core::ffi::c_int as isize);
            if (cur_ext_sort_cw_sec as core::ffi::c_int
                * max_allowed_cw_len as core::ffi::c_int) < num_decoded_bits
            {
                (*ptr_hcr_info).str_dec_io.err_log
                    |= (ERROR_POS << 17 as core::ffi::c_int) as UWORD32;
            }
            if 1 as core::ffi::c_int
                == ixheaacd_err_detect_pcw_segment(
                    *p_remaining_bits_in_seg,
                    ptr_hcr_info,
                    PCW_ESC_SIGN,
                    ptr_quant_spec_coeff
                        .offset((*ptr_hcr_info).str_dec_io.quant_spec_coeff_idx as isize)
                        .offset(-(2 as core::ffi::c_int as isize)),
                    2 as UWORD8,
                ) as core::ffi::c_int
            {
                return;
            }
        }
        num_ext_sorted_cw_in_sect_idx += 1;
        if num_ext_sorted_cw_in_sect_idx >= MAX_SFB_HCR + MAX_HCR_SETS {
            return;
        }
        ext_sort_sec = ext_sort_sec.wrapping_sub(1);
    }
    num_ext_sorted_sect_in_sets_idx += 1;
    if num_ext_sorted_sect_in_sets_idx >= MAX_HCR_SETS {
        return;
    }
    (*itt_bit_buff).cnt_bits = read_bits;
    (*ptr_hcr_info).sect_info.num_ext_sorted_cw_in_sect_idx = num_ext_sorted_cw_in_sect_idx;
    (*ptr_hcr_info).sect_info.ext_sorted_cw_idx = ext_sorted_cw_idx;
    (*ptr_hcr_info).sect_info.num_ext_sorted_sect_in_sets_idx = num_ext_sorted_sect_in_sets_idx;
    (*ptr_hcr_info).sect_info.ext_sorted_sect_max_cb_len_idx = ext_sorted_sect_max_cb_len_idx;
}
unsafe extern "C" fn ixheaacd_init_segment_bit_field(
    mut num_segment: WORD32,
    mut p_remaining_bits_in_seg: *mut WORD8,
) -> UWORD32 {
    let mut i: WORD16 = 0;
    let mut num_valid_segment: WORD16 = 0 as WORD16;
    i = 0 as WORD16;
    while (i as core::ffi::c_int) < num_segment {
        if *p_remaining_bits_in_seg.offset(i as isize) as core::ffi::c_int
            != 0 as core::ffi::c_int
        {
            num_valid_segment = (num_valid_segment as core::ffi::c_int
                + 1 as core::ffi::c_int) as WORD16;
        }
        i += 1;
    }
    return num_valid_segment as UWORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_toggle_read_dir(mut read_direction: UWORD8) -> UWORD8 {
    if read_direction as core::ffi::c_int == FROM_LEFT_TO_RIGHT {
        return FROM_RIGHT_TO_LEFT as UWORD8
    } else {
        return FROM_LEFT_TO_RIGHT as UWORD8
    };
}
#[inline]
unsafe extern "C" fn ixheaacd_huff_dec_quad_hcr_non_pcw(
    mut itt_bit_buff: *mut ia_bit_buf_struct,
    mut spec_coef: *mut WORD32,
    mut code_book_tbl: *const UWORD16,
    mut tbl_sign: WORD32,
    mut idx_table: *const UWORD32,
) -> UWORD16 {
    let mut index: WORD16 = 0;
    let mut length: WORD16 = 0;
    let mut cw_len: WORD16 = 0;
    let mut read_word: WORD32 = 0;
    read_word = ixheaacd_aac_showbits_32(
        (*itt_bit_buff).byte_ptr,
        (*itt_bit_buff).bit_count,
        0 as *mut WORD32,
    ) as WORD32;
    ixheaacd_huffman_decode(
        read_word,
        &mut index,
        &mut length,
        code_book_tbl,
        idx_table,
    );
    cw_len = length;
    if tbl_sign != 0 {
        let mut temp_word: WORD32 = 0;
        let mut w: WORD32 = 0;
        let mut x: WORD32 = 0;
        let mut y: WORD32 = 0;
        let mut z: WORD32 = 0;
        temp_word = read_word << length as core::ffi::c_int;
        w = (index as core::ffi::c_int / 27 as core::ffi::c_int) as WORD32;
        index = (index as core::ffi::c_int
            - w as core::ffi::c_int * 27 as core::ffi::c_int) as WORD16;
        x = (index as core::ffi::c_int / 9 as core::ffi::c_int) as WORD32;
        index = (index as core::ffi::c_int
            - x as core::ffi::c_int * 9 as core::ffi::c_int) as WORD16;
        y = (index as core::ffi::c_int / 3 as core::ffi::c_int) as WORD32;
        z = (index as core::ffi::c_int - y as core::ffi::c_int * 3 as core::ffi::c_int)
            as WORD32;
        if w != 0 {
            if temp_word as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0 {
                w = -w;
            }
            temp_word <<= 1 as core::ffi::c_int;
            cw_len += 1;
        }
        let fresh39 = spec_coef;
        spec_coef = spec_coef.offset(1);
        *fresh39 = w;
        if x != 0 {
            if temp_word as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0 {
                x = -x;
            }
            temp_word <<= 1 as core::ffi::c_int;
            cw_len += 1;
        }
        let fresh40 = spec_coef;
        spec_coef = spec_coef.offset(1);
        *fresh40 = x;
        if y != 0 {
            if temp_word as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0 {
                y = -y;
            }
            temp_word <<= 1 as core::ffi::c_int;
            cw_len += 1;
        }
        let fresh41 = spec_coef;
        spec_coef = spec_coef.offset(1);
        *fresh41 = y;
        if z != 0 {
            if temp_word as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0 {
                z = -z;
            }
            temp_word <<= 1 as core::ffi::c_int;
            cw_len += 1;
        }
        let fresh42 = spec_coef;
        spec_coef = spec_coef.offset(1);
        *fresh42 = z;
    } else {
        let mut w_0: WORD32 = 0;
        let mut x_0: WORD32 = 0;
        let mut y_0: WORD32 = 0;
        let mut z_0: WORD32 = 0;
        w_0 = (index as core::ffi::c_int / 27 as core::ffi::c_int
            - 1 as core::ffi::c_int) as WORD32;
        index = (index as core::ffi::c_int
            - (w_0 as core::ffi::c_int + 1 as core::ffi::c_int) * 27 as core::ffi::c_int)
            as WORD16;
        x_0 = (index as core::ffi::c_int / 9 as core::ffi::c_int - 1 as core::ffi::c_int)
            as WORD32;
        index = (index as core::ffi::c_int
            - (x_0 as core::ffi::c_int + 1 as core::ffi::c_int) * 9 as core::ffi::c_int)
            as WORD16;
        y_0 = (index as core::ffi::c_int / 3 as core::ffi::c_int - 1 as core::ffi::c_int)
            as WORD32;
        z_0 = (index as core::ffi::c_int
            - (y_0 as core::ffi::c_int + 1 as core::ffi::c_int) * 3 as core::ffi::c_int
            - 1 as core::ffi::c_int) as WORD32;
        let fresh43 = spec_coef;
        spec_coef = spec_coef.offset(1);
        *fresh43 = w_0;
        let fresh44 = spec_coef;
        spec_coef = spec_coef.offset(1);
        *fresh44 = x_0;
        let fresh45 = spec_coef;
        spec_coef = spec_coef.offset(1);
        *fresh45 = y_0;
        let fresh46 = spec_coef;
        spec_coef = spec_coef.offset(1);
        *fresh46 = z_0;
    }
    return cw_len as UWORD16;
}
#[inline]
unsafe extern "C" fn ixheaacd_huff_dec_word_hcr_non_pcw(
    mut itt_bit_buff: *mut ia_bit_buf_struct,
    mut spec_coef: *mut WORD32,
    mut code_book_tbl: *const UWORD16,
    mut idx_table: *const UWORD32,
) -> UWORD16 {
    let mut sp1: WORD32 = 0;
    let mut sp2: WORD32 = 0;
    let mut flush_cw: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut value: WORD32 = 0;
    let mut norm_val: WORD32 = 0;
    let mut off: WORD32 = 0;
    let mut out1: WORD32 = 0;
    let mut out2: WORD32 = 0;
    let mut cw_len: UWORD16 = 0;
    let mut index: WORD16 = 0;
    let mut length: WORD32 = 0;
    let mut read_word: WORD32 = 0;
    let mut increment: WORD32 = 0;
    let mut ptr_read_next: *mut UWORD8 = 0 as *mut UWORD8;
    read_word = ixheaacd_aac_showbits_32(
        (*itt_bit_buff).byte_ptr,
        (*itt_bit_buff).bit_count,
        &mut increment,
    ) as WORD32;
    ptr_read_next = (*itt_bit_buff).byte_ptr;
    ptr_read_next = ptr_read_next.offset(increment as isize);
    ixheaacd_huff_sfb_table(
        read_word,
        &mut index,
        &mut length,
        code_book_tbl,
        idx_table,
    );
    cw_len = length as UWORD16;
    ixheaacd_aac_read_byte_corr1(
        &mut ptr_read_next,
        &mut length,
        &mut read_word,
        0 as *mut UWORD8,
    );
    out1 = (index as core::ffi::c_int / 17 as core::ffi::c_int) as WORD32;
    out2 = (index as core::ffi::c_int
        - out1 as core::ffi::c_int * 17 as core::ffi::c_int) as WORD32;
    flush_cw = read_word << length;
    sp1 = out1;
    sp2 = out2;
    if out1 != 0 {
        if flush_cw as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0 {
            out1 = -out1;
        }
        flush_cw = flush_cw << 1 as core::ffi::c_int;
        length += 1;
        cw_len = cw_len.wrapping_add(1);
    }
    if out2 != 0 {
        if flush_cw as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0 {
            out2 = -out2;
        }
        length += 1;
        cw_len = cw_len.wrapping_add(1);
    }
    ixheaacd_aac_read_byte_corr1(
        &mut ptr_read_next,
        &mut length,
        &mut read_word,
        0 as *mut UWORD8,
    );
    if sp1 == 16 as core::ffi::c_int {
        i = 4 as core::ffi::c_int as WORD32;
        value = ixheaac_extu(read_word as UWORD32, length, 23 as WORD32) as WORD32;
        value = (value as core::ffi::c_uint | 0xfffffe00 as core::ffi::c_uint) as WORD32;
        norm_val = ixheaac_norm32(value) as WORD32;
        i += norm_val as core::ffi::c_int - 22 as core::ffi::c_int;
        length += norm_val as core::ffi::c_int - 21 as core::ffi::c_int;
        cw_len = (cw_len as core::ffi::c_int
            + (norm_val as core::ffi::c_int - 21 as core::ffi::c_int)) as UWORD16;
        ixheaacd_aac_read_byte_corr1(
            &mut ptr_read_next,
            &mut length,
            &mut read_word,
            0 as *mut UWORD8,
        );
        off = ixheaac_extu(read_word as UWORD32, length, 32 as WORD32 - i) as WORD32;
        length += i;
        cw_len = (cw_len as core::ffi::c_int + i as core::ffi::c_int) as UWORD16;
        ixheaacd_aac_read_byte_corr1(
            &mut ptr_read_next,
            &mut length,
            &mut read_word,
            0 as *mut UWORD8,
        );
        i = off + ((1 as core::ffi::c_int) << i);
        if out1 < 0 as core::ffi::c_int {
            let fresh47 = spec_coef;
            spec_coef = spec_coef.offset(1);
            *fresh47 = -i;
        } else {
            let fresh48 = spec_coef;
            spec_coef = spec_coef.offset(1);
            *fresh48 = i;
        }
    } else {
        let fresh49 = spec_coef;
        spec_coef = spec_coef.offset(1);
        *fresh49 = out1;
    }
    if sp2 == 16 as core::ffi::c_int {
        i = 4 as core::ffi::c_int as WORD32;
        value = ixheaac_extu(read_word as UWORD32, length, 23 as WORD32) as WORD32;
        value = (value as core::ffi::c_uint | 0xfffffe00 as core::ffi::c_uint) as WORD32;
        norm_val = ixheaac_norm32(value) as WORD32;
        i += norm_val as core::ffi::c_int - 22 as core::ffi::c_int;
        length += norm_val as core::ffi::c_int - 21 as core::ffi::c_int;
        cw_len = (cw_len as core::ffi::c_int
            + (norm_val as core::ffi::c_int - 21 as core::ffi::c_int)) as UWORD16;
        ixheaacd_aac_read_byte_corr1(
            &mut ptr_read_next,
            &mut length,
            &mut read_word,
            0 as *mut UWORD8,
        );
        off = ixheaac_extu(read_word as UWORD32, length, 32 as WORD32 - i) as WORD32;
        length += i;
        cw_len = (cw_len as core::ffi::c_int + i as core::ffi::c_int) as UWORD16;
        ixheaacd_aac_read_byte_corr1(
            &mut ptr_read_next,
            &mut length,
            &mut read_word,
            0 as *mut UWORD8,
        );
        i = off + ((1 as core::ffi::c_int) << i);
        if out2 < 0 as core::ffi::c_int {
            let fresh50 = spec_coef;
            spec_coef = spec_coef.offset(1);
            *fresh50 = -i;
        } else {
            let fresh51 = spec_coef;
            spec_coef = spec_coef.offset(1);
            *fresh51 = i;
        }
    } else {
        let fresh52 = spec_coef;
        spec_coef = spec_coef.offset(1);
        *fresh52 = out2;
    }
    return cw_len;
}
unsafe extern "C" fn ixheaacd_decode_hcr_non_pcw(
    mut itt_bit_buff: *mut ia_bit_buf_struct,
    mut ptr_hcr_info: *mut ia_hcr_info_struct,
    mut ptr_aac_tables: *mut ia_aac_dec_tables_struct,
    mut cw_offset: *mut WORD32,
    mut trial: WORD32,
    mut start: WORD32,
) -> VOID {
    let mut codeword_len: WORD16 = 0 as WORD16;
    let mut seg_bits_left: WORD8 = 0;
    let mut tot_bits_to_save: UWORD8 = 0;
    let mut code_bits_to_save: UWORD8 = 0;
    let mut extra_code_bits: UWORD8 = 0;
    let mut segment_offset: WORD32 = 0 as WORD32;
    let mut p_remaining_bits_in_seg: *mut WORD8 = ((*ptr_hcr_info)
        .str_segment_info
        .p_remaining_bits_in_seg)
        .as_mut_ptr();
    let mut num_segment: WORD32 = (*ptr_hcr_info).str_segment_info.num_segment;
    segment_offset = start;
    while segment_offset < trial {
        if *p_remaining_bits_in_seg.offset(segment_offset as isize) as core::ffi::c_int
            != 0 && (*ptr_hcr_info).str_segment_info.is_decoded[*cw_offset as usize] == 0
        {
            let mut i_qsc: UWORD32 = 0;
            let mut current_seg_bits: WORD8 = *p_remaining_bits_in_seg
                .offset(segment_offset as isize);
            (*itt_bit_buff).byte_ptr = (*itt_bit_buff).ptr_start;
            (*itt_bit_buff).valid_bits = 0 as core::ffi::c_int as WORD32;
            (*itt_bit_buff).byte = 0 as UWORD8;
            (*itt_bit_buff).bit_count = 0 as core::ffi::c_int as WORD32;
            (*itt_bit_buff).write_bit_count = 0 as core::ffi::c_int as WORD32;
            if (*ptr_hcr_info).str_segment_info.p_num_bits[*cw_offset as usize] != 0 {
                extra_code_bits = (if (*ptr_hcr_info)
                    .str_segment_info
                    .p_num_bits[*cw_offset as usize] as core::ffi::c_int
                    - 32 as core::ffi::c_int > 0 as core::ffi::c_int
                {
                    (*ptr_hcr_info).str_segment_info.p_num_bits[*cw_offset as usize]
                        as core::ffi::c_int - 32 as core::ffi::c_int
                } else {
                    0 as core::ffi::c_int
                }) as UWORD8;
                code_bits_to_save = (if ((*ptr_hcr_info)
                    .str_segment_info
                    .p_num_bits[*cw_offset as usize] as core::ffi::c_int)
                    < 32 as core::ffi::c_int
                {
                    (*ptr_hcr_info).str_segment_info.p_num_bits[*cw_offset as usize]
                        as core::ffi::c_int
                } else {
                    32 as core::ffi::c_int
                }) as UWORD8;
                ixheaacd_write_bit(
                    itt_bit_buff,
                    (*ptr_hcr_info).str_segment_info.code_extra[*cw_offset as usize],
                    extra_code_bits as WORD32,
                );
                ixheaacd_write_bit(
                    itt_bit_buff,
                    (*ptr_hcr_info).str_segment_info.code[*cw_offset as usize],
                    code_bits_to_save as WORD32,
                );
            }
            let mut bit: UWORD32 = 0;
            let mut read_bit_offset: WORD32 = 0;
            if (*ptr_hcr_info).str_segment_info.read_direction as core::ffi::c_int
                == FROM_LEFT_TO_RIGHT
            {
                read_bit_offset = (*ptr_hcr_info)
                    .str_segment_info
                    .arr_seg_start_l[segment_offset as usize] as WORD32
                    - ((*itt_bit_buff).size - (*itt_bit_buff).cnt_bits);
                if read_bit_offset != 0 {
                    (*itt_bit_buff).cnt_bits += -read_bit_offset;
                }
                (*itt_bit_buff).ptr_read_next = ((*itt_bit_buff).ptr_bit_buf_base)
                    .offset(
                        ((*itt_bit_buff).size - (*itt_bit_buff).cnt_bits
                            >> 3 as core::ffi::c_int) as isize,
                    );
                (*itt_bit_buff).bit_pos = ((*itt_bit_buff).size as core::ffi::c_int
                    - (*itt_bit_buff).cnt_bits as core::ffi::c_int
                    & 7 as core::ffi::c_int) as WORD32;
                while *p_remaining_bits_in_seg.offset(segment_offset as isize)
                    as core::ffi::c_int > 0 as core::ffi::c_int
                {
                    bit = ixheaacd_aac_read_bit_rev(itt_bit_buff) as UWORD32;
                    (*ptr_hcr_info)
                        .str_segment_info
                        .arr_seg_start_l[segment_offset as usize] = ((*ptr_hcr_info)
                        .str_segment_info
                        .arr_seg_start_l[segment_offset as usize] as core::ffi::c_int
                        + 1 as core::ffi::c_int) as UWORD16;
                    ixheaacd_write_bit(itt_bit_buff, bit as WORD32, 1 as WORD32);
                    let ref mut fresh53 = *p_remaining_bits_in_seg
                        .offset(segment_offset as isize);
                    *fresh53 = (*fresh53 as core::ffi::c_int - 1 as core::ffi::c_int)
                        as WORD8;
                }
            } else {
                read_bit_offset = (*ptr_hcr_info)
                    .str_segment_info
                    .arr_seg_start_r[segment_offset as usize] as WORD32
                    - ((*itt_bit_buff).size - (*itt_bit_buff).cnt_bits);
                if read_bit_offset != 0 {
                    (*itt_bit_buff).cnt_bits += -read_bit_offset;
                }
                (*itt_bit_buff).ptr_read_next = ((*itt_bit_buff).ptr_bit_buf_base)
                    .offset(
                        ((*itt_bit_buff).size - (*itt_bit_buff).cnt_bits
                            >> 3 as core::ffi::c_int) as isize,
                    );
                (*itt_bit_buff).bit_pos = ((*itt_bit_buff).size as core::ffi::c_int
                    - (*itt_bit_buff).cnt_bits as core::ffi::c_int
                    & 7 as core::ffi::c_int) as WORD32;
                while *p_remaining_bits_in_seg.offset(segment_offset as isize)
                    as core::ffi::c_int > 0 as core::ffi::c_int
                {
                    bit = ixheaacd_aac_read_bit(itt_bit_buff) as UWORD32;
                    (*ptr_hcr_info)
                        .str_segment_info
                        .arr_seg_start_r[segment_offset as usize] = ((*ptr_hcr_info)
                        .str_segment_info
                        .arr_seg_start_r[segment_offset as usize] as core::ffi::c_int
                        - 1 as core::ffi::c_int) as UWORD16;
                    ixheaacd_write_bit(itt_bit_buff, bit as WORD32, 1 as WORD32);
                    let ref mut fresh54 = *p_remaining_bits_in_seg
                        .offset(segment_offset as isize);
                    *fresh54 = (*fresh54 as core::ffi::c_int - 1 as core::ffi::c_int)
                        as WORD8;
                }
            }
            ixheaacd_write_bit(
                itt_bit_buff,
                0 as WORD32,
                32 as WORD32 - (*itt_bit_buff).bit_count % 32 as WORD32,
            );
            (*itt_bit_buff).valid_bits = 8 as core::ffi::c_int as WORD32;
            (*itt_bit_buff).byte_ptr = (*itt_bit_buff).ptr_start;
            (*itt_bit_buff).byte = *(*itt_bit_buff).ptr_start;
            if current_seg_bits != 0 {
                i_qsc = (*ptr_hcr_info)
                    .str_non_pcw_side_info
                    .res_ptr_idx[(*cw_offset % num_segment) as usize] as UWORD32;
                if (*ptr_hcr_info)
                    .str_non_pcw_side_info
                    .ptr_cb[(*cw_offset % num_segment) as usize] as core::ffi::c_int
                    <= 4 as core::ffi::c_int
                {
                    let mut tbl_sign: WORD32 = 0 as WORD32;
                    let mut cb_table: *const UWORD16 = (*ptr_aac_tables)
                        .code_book[(*ptr_hcr_info)
                        .str_non_pcw_side_info
                        .ptr_cb[(*cw_offset % num_segment) as usize] as usize];
                    let mut idx_table: *const UWORD32 = (*ptr_aac_tables)
                        .index_table[(*ptr_hcr_info)
                        .str_non_pcw_side_info
                        .ptr_cb[(*cw_offset % num_segment) as usize] as usize];
                    if (*ptr_hcr_info)
                        .str_non_pcw_side_info
                        .ptr_cb[(*cw_offset % num_segment) as usize] as core::ffi::c_int
                        > 2 as core::ffi::c_int
                    {
                        tbl_sign = 1 as core::ffi::c_int as WORD32;
                    }
                    codeword_len = ixheaacd_huff_dec_quad_hcr_non_pcw(
                        itt_bit_buff,
                        &mut *((*ptr_hcr_info).str_non_pcw_side_info.ptr_result_base)
                            .offset(i_qsc as isize),
                        cb_table,
                        tbl_sign,
                        idx_table,
                    ) as WORD16;
                    seg_bits_left = (current_seg_bits as core::ffi::c_int
                        - codeword_len as core::ffi::c_int
                        + (*ptr_hcr_info)
                            .str_segment_info
                            .p_num_bits[*cw_offset as usize] as core::ffi::c_int)
                        as WORD8;
                } else if ((*ptr_hcr_info)
                    .str_non_pcw_side_info
                    .ptr_cb[(*cw_offset % num_segment) as usize] as core::ffi::c_int)
                    < 11 as core::ffi::c_int
                {
                    let mut tbl_sign_0: WORD32 = 0 as WORD32;
                    let mut huff_mode: WORD32 = 9 as WORD32;
                    let mut cb_table_0: *const UWORD16 = (*ptr_aac_tables)
                        .code_book[(*ptr_hcr_info)
                        .str_non_pcw_side_info
                        .ptr_cb[(*cw_offset % num_segment) as usize] as usize];
                    let mut idx_table_0: *const UWORD32 = (*ptr_aac_tables)
                        .index_table[(*ptr_hcr_info)
                        .str_non_pcw_side_info
                        .ptr_cb[(*cw_offset % num_segment) as usize] as usize];
                    if (*ptr_hcr_info)
                        .str_non_pcw_side_info
                        .ptr_cb[(*cw_offset % num_segment) as usize] as core::ffi::c_int
                        > 6 as core::ffi::c_int
                    {
                        if (*ptr_hcr_info)
                            .str_non_pcw_side_info
                            .ptr_cb[(*cw_offset % num_segment) as usize]
                            as core::ffi::c_int > 8 as core::ffi::c_int
                        {
                            huff_mode = 13 as core::ffi::c_int as WORD32;
                        } else {
                            huff_mode = 8 as core::ffi::c_int as WORD32;
                        }
                        tbl_sign_0 = 1 as core::ffi::c_int as WORD32;
                    }
                    codeword_len = ixheaacd_huff_dec_pair_hcr_non_pcw(
                        itt_bit_buff,
                        &mut *((*ptr_hcr_info).str_non_pcw_side_info.ptr_result_base)
                            .offset(i_qsc as isize),
                        cb_table_0,
                        tbl_sign_0,
                        idx_table_0,
                        huff_mode,
                    );
                    seg_bits_left = (current_seg_bits as core::ffi::c_int
                        - codeword_len as core::ffi::c_int
                        + (*ptr_hcr_info)
                            .str_segment_info
                            .p_num_bits[*cw_offset as usize] as core::ffi::c_int)
                        as WORD8;
                }
                if (*ptr_hcr_info)
                    .str_non_pcw_side_info
                    .ptr_cb[(*cw_offset % num_segment) as usize] as core::ffi::c_int
                    >= 11 as core::ffi::c_int
                {
                    let mut idx_table_1: *const UWORD32 = ((*(*ptr_aac_tables)
                        .pstr_huffmann_tables)
                        .idx_table_hf11)
                        .as_mut_ptr();
                    let mut cb_table_1: *const UWORD16 = ((*(*ptr_aac_tables)
                        .pstr_huffmann_tables)
                        .input_table_cb11)
                        .as_mut_ptr();
                    codeword_len = ixheaacd_huff_dec_word_hcr_non_pcw(
                        itt_bit_buff,
                        &mut *((*ptr_hcr_info).str_non_pcw_side_info.ptr_result_base)
                            .offset(i_qsc as isize),
                        cb_table_1,
                        idx_table_1,
                    ) as WORD16;
                    seg_bits_left = (current_seg_bits as core::ffi::c_int
                        - codeword_len as core::ffi::c_int
                        + (*ptr_hcr_info)
                            .str_segment_info
                            .p_num_bits[*cw_offset as usize] as core::ffi::c_int)
                        as WORD8;
                }
                if (seg_bits_left as core::ffi::c_int) < 0 as core::ffi::c_int {
                    tot_bits_to_save = (current_seg_bits as core::ffi::c_int
                        + (*ptr_hcr_info)
                            .str_segment_info
                            .p_num_bits[*cw_offset as usize] as core::ffi::c_int)
                        as UWORD8;
                    extra_code_bits = (if tot_bits_to_save as core::ffi::c_int
                        - 32 as core::ffi::c_int > 0 as core::ffi::c_int
                    {
                        tot_bits_to_save as core::ffi::c_int - 32 as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    }) as UWORD8;
                    code_bits_to_save = (if (tot_bits_to_save as core::ffi::c_int)
                        < 32 as core::ffi::c_int
                    {
                        tot_bits_to_save as core::ffi::c_int
                    } else {
                        32 as core::ffi::c_int
                    }) as UWORD8;
                    (*ptr_hcr_info).str_segment_info.code_extra[*cw_offset as usize] = ixheaacd_read_bit(
                        itt_bit_buff,
                        extra_code_bits as WORD32,
                    );
                    (*ptr_hcr_info).str_segment_info.code[*cw_offset as usize] = ixheaacd_read_bit(
                        itt_bit_buff,
                        code_bits_to_save as WORD32,
                    );
                    (*ptr_hcr_info).str_segment_info.p_num_bits[*cw_offset as usize] = tot_bits_to_save
                        as WORD8;
                    *p_remaining_bits_in_seg.offset(segment_offset as isize) = 0
                        as WORD8;
                    if (*p_remaining_bits_in_seg.offset(segment_offset as isize)
                        as core::ffi::c_int) < 0 as core::ffi::c_int
                    {
                        *p_remaining_bits_in_seg.offset(segment_offset as isize) = 0
                            as WORD8;
                    }
                } else {
                    *p_remaining_bits_in_seg.offset(segment_offset as isize) = (current_seg_bits
                        as core::ffi::c_int
                        - (codeword_len as core::ffi::c_int
                            - (*ptr_hcr_info)
                                .str_segment_info
                                .p_num_bits[*cw_offset as usize] as core::ffi::c_int))
                        as WORD8;
                    (*ptr_hcr_info).str_segment_info.p_num_bits[*cw_offset as usize] = 0
                        as WORD8;
                    (*ptr_hcr_info).str_segment_info.is_decoded[*cw_offset as usize] = 1
                        as core::ffi::c_int as WORD32;
                    if (*p_remaining_bits_in_seg.offset(segment_offset as isize)
                        as core::ffi::c_int) < 0 as core::ffi::c_int
                    {
                        *p_remaining_bits_in_seg.offset(segment_offset as isize) = 0
                            as WORD8;
                    }
                }
                if *p_remaining_bits_in_seg.offset(segment_offset as isize)
                    as core::ffi::c_int > 0 as core::ffi::c_int
                {
                    if (*ptr_hcr_info).str_segment_info.read_direction
                        as core::ffi::c_int == FROM_LEFT_TO_RIGHT
                    {
                        (*ptr_hcr_info)
                            .str_segment_info
                            .arr_seg_start_l[segment_offset as usize] = ((*ptr_hcr_info)
                            .str_segment_info
                            .arr_seg_start_l[segment_offset as usize] as core::ffi::c_int
                            - *p_remaining_bits_in_seg.offset(segment_offset as isize)
                                as core::ffi::c_int) as UWORD16;
                    } else {
                        (*ptr_hcr_info)
                            .str_segment_info
                            .arr_seg_start_r[segment_offset as usize] = ((*ptr_hcr_info)
                            .str_segment_info
                            .arr_seg_start_r[segment_offset as usize] as core::ffi::c_int
                            + *p_remaining_bits_in_seg.offset(segment_offset as isize)
                                as core::ffi::c_int) as UWORD16;
                    }
                }
            }
        }
        segment_offset += 1;
        *cw_offset += 1 as core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_decode_non_pcw(
    mut itt_bit_buff: *mut ia_bit_buf_struct,
    mut ptr_hcr_info: *mut ia_hcr_info_struct,
    mut ptr_aac_tables: *mut ia_aac_dec_tables_struct,
) -> VOID {
    let mut num_valid_segment: UWORD32 = 0;
    let mut cw_offset: WORD32 = 0;
    let mut trial: WORD32 = 0;
    let mut num_segment: WORD32 = 0;
    let mut num_code_word: WORD32 = 0;
    let mut num_set: UWORD8 = 0;
    let mut current_set: UWORD8 = 0;
    let mut code_word_set: WORD32 = 0;
    let mut loop1: WORD32 = 0;
    let mut loop2: WORD32 = 0;
    num_segment = (*ptr_hcr_info).str_segment_info.num_segment;
    num_valid_segment = ixheaacd_init_segment_bit_field(
        num_segment,
        ((*ptr_hcr_info).str_segment_info.p_remaining_bits_in_seg).as_mut_ptr(),
    );
    if num_valid_segment != 0 as UWORD32 {
        num_code_word = (*ptr_hcr_info).sect_info.num_code_word as WORD32;
        num_set = ((num_code_word as core::ffi::c_int - 1 as core::ffi::c_int)
            / num_segment as core::ffi::c_int + 1 as core::ffi::c_int) as UWORD8;
        (*ptr_hcr_info).str_segment_info.read_direction = FROM_RIGHT_TO_LEFT as UWORD8;
        current_set = 1 as UWORD8;
        while (current_set as core::ffi::c_int) < num_set as core::ffi::c_int {
            num_code_word -= num_segment;
            if num_code_word < num_segment {
                code_word_set = num_code_word;
            } else {
                code_word_set = num_segment;
            }
            ixheaacd_nonpcw_sideinfo_init(ptr_hcr_info);
            cw_offset = (num_segment as core::ffi::c_int
                * current_set as core::ffi::c_int) as WORD32;
            ixheaacd_decode_hcr_non_pcw(
                itt_bit_buff,
                ptr_hcr_info,
                ptr_aac_tables,
                &mut cw_offset,
                code_word_set,
                0 as WORD32,
            );
            trial = 1 as core::ffi::c_int as WORD32;
            while trial < num_segment {
                cw_offset = (num_segment as core::ffi::c_int
                    * current_set as core::ffi::c_int) as WORD32;
                loop1 = if num_segment < trial + code_word_set {
                    num_segment
                } else {
                    trial + code_word_set
                };
                loop2 = (if 0 as core::ffi::c_int > trial + code_word_set - num_segment {
                    0 as core::ffi::c_int
                } else {
                    trial as core::ffi::c_int + code_word_set as core::ffi::c_int
                        - num_segment as core::ffi::c_int
                }) as WORD32;
                ixheaacd_decode_hcr_non_pcw(
                    itt_bit_buff,
                    ptr_hcr_info,
                    ptr_aac_tables,
                    &mut cw_offset,
                    loop1,
                    trial,
                );
                ixheaacd_decode_hcr_non_pcw(
                    itt_bit_buff,
                    ptr_hcr_info,
                    ptr_aac_tables,
                    &mut cw_offset,
                    loop2,
                    0 as WORD32,
                );
                trial += 1;
            }
            (*ptr_hcr_info).str_segment_info.read_direction = ixheaacd_toggle_read_dir(
                (*ptr_hcr_info).str_segment_info.read_direction,
            );
            current_set = current_set.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn ixheaacd_hcr_reorder_quantized_spec_coeff(
    mut ptr_hcr_info: *mut ia_hcr_info_struct,
    mut ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
) -> VOID {
    let mut qsc: WORD32 = 0;
    let mut abs_qsc: UWORD32 = 0;
    let mut i: UWORD32 = 0;
    let mut j: UWORD32 = 0;
    let mut num_spec_val_sect: UWORD16 = 0;
    let mut ptr_teva: *mut WORD32 = 0 as *mut WORD32;
    let mut lav_err_cnt: UWORD16 = 0 as UWORD16;
    let mut num_sect: UWORD32 = (*ptr_hcr_info).str_dec_io.num_sect as UWORD32;
    let mut ptr_quant_spec_coeff_base: *mut WORD32 = (*ptr_hcr_info)
        .str_dec_io
        .ptr_quant_spec_coeff_base;
    let mut ptr_quant_spec_coeff: *mut WORD32 = (*ptr_hcr_info)
        .str_dec_io
        .ptr_quant_spec_coeff_base;
    let mut ptr_cb_dim_shift_tbl: *const UWORD8 = (*ptr_hcr_info)
        .table_info
        .ptr_cb_dim_shift_tbl;
    let mut ptr_lav_tbl: *const UWORD16 = (*ptr_hcr_info).table_info.ptr_lav_tbl;
    let mut ptr_sorted_cb: *mut UWORD8 = ((*ptr_hcr_info).sect_info.ptr_sorted_cb)
        .as_mut_ptr();
    let mut ptr_num_sorted_cw_in_sect: *mut UWORD16 = ((*ptr_hcr_info)
        .sect_info
        .ptr_num_sorted_cw_in_sect)
        .as_mut_ptr();
    let mut ptr_reorder_offset: *mut UWORD16 = ((*ptr_hcr_info)
        .sect_info
        .ptr_reorder_offset)
        .as_mut_ptr();
    let mut arr_temp_values: *mut WORD32 = ((*ptr_hcr_info)
        .str_segment_info
        .arr_temp_values)
        .as_mut_ptr();
    let mut ptr_bak: *mut WORD32 = ((*ptr_hcr_info).str_segment_info.arr_temp_values)
        .as_mut_ptr();
    i = num_sect;
    while i != 0 as UWORD32 {
        let fresh55 = ptr_num_sorted_cw_in_sect;
        ptr_num_sorted_cw_in_sect = ptr_num_sorted_cw_in_sect.offset(1);
        num_spec_val_sect = ((*fresh55 as core::ffi::c_int)
            << *ptr_cb_dim_shift_tbl.offset(*ptr_sorted_cb as isize) as core::ffi::c_int)
            as UWORD16;
        let fresh56 = ptr_reorder_offset;
        ptr_reorder_offset = ptr_reorder_offset.offset(1);
        ptr_teva = &mut *arr_temp_values.offset(*fresh56 as isize) as *mut WORD32;
        j = num_spec_val_sect as UWORD32;
        while j != 0 as UWORD32 {
            let fresh57 = ptr_quant_spec_coeff;
            ptr_quant_spec_coeff = ptr_quant_spec_coeff.offset(1);
            qsc = *fresh57;
            abs_qsc = ixheaac_abs32(qsc) as UWORD32;
            if abs_qsc <= *ptr_lav_tbl.offset(*ptr_sorted_cb as isize) as UWORD32 {
                let fresh58 = ptr_teva;
                ptr_teva = ptr_teva.offset(1);
                *fresh58 = qsc;
            } else if abs_qsc == 8192 as UWORD32 {
                let fresh59 = ptr_teva;
                ptr_teva = ptr_teva.offset(1);
                *fresh59 = qsc;
            } else {
                let fresh60 = ptr_teva;
                ptr_teva = ptr_teva.offset(1);
                *fresh60 = 8192 as core::ffi::c_int;
                lav_err_cnt = (lav_err_cnt as core::ffi::c_int + 1 as core::ffi::c_int)
                    as UWORD16;
            }
            j = j.wrapping_sub(1);
        }
        ptr_sorted_cb = ptr_sorted_cb.offset(1);
        i = i.wrapping_sub(1);
    }
    if (*ptr_aac_dec_channel_info).str_ics_info.window_sequence as core::ffi::c_int
        == EIGHT_SHORT_SEQUENCE
    {
        let mut ptr_out: *mut WORD32 = 0 as *mut WORD32;
        let mut window: WORD8 = 0;
        ptr_bak = ((*ptr_hcr_info).str_segment_info.arr_temp_values).as_mut_ptr();
        window = 0 as WORD8;
        while (window as core::ffi::c_int) < 8 as core::ffi::c_int {
            ptr_out = ptr_quant_spec_coeff_base
                .offset(
                    (window as WORD32 * (*ptr_aac_dec_channel_info).granule_len) as isize,
                );
            i = 0 as UWORD32;
            while i < (1024 as core::ffi::c_int / NUMBER_OF_UNIT_GROUPS) as UWORD32 {
                ptr_teva = ptr_bak
                    .offset(
                        ((window as core::ffi::c_int) << 2 as core::ffi::c_int) as isize,
                    )
                    .offset(i.wrapping_mul(32 as UWORD32) as isize);
                j = 4 as UWORD32;
                while j != 0 as UWORD32 {
                    let fresh61 = ptr_teva;
                    ptr_teva = ptr_teva.offset(1);
                    let fresh62 = ptr_out;
                    ptr_out = ptr_out.offset(1);
                    *fresh62 = *fresh61;
                    j = j.wrapping_sub(1);
                }
                i = i.wrapping_add(1);
            }
            window += 1;
        }
    } else {
        ptr_quant_spec_coeff = ptr_quant_spec_coeff_base;
        i = 1024 as UWORD32;
        while i != 0 as UWORD32 {
            let fresh63 = ptr_bak;
            ptr_bak = ptr_bak.offset(1);
            let fresh64 = ptr_quant_spec_coeff;
            ptr_quant_spec_coeff = ptr_quant_spec_coeff.offset(1);
            *fresh64 = *fresh63;
            i = i.wrapping_sub(1);
        }
    }
    if lav_err_cnt as core::ffi::c_int != 0 as core::ffi::c_int {
        (*ptr_hcr_info).str_dec_io.err_log
            |= (ERROR_POS << 1 as core::ffi::c_int) as UWORD32;
    }
}
unsafe extern "C" fn ixheaacd_err_detect_segmentation_final(
    mut ptr_hcr_info: *mut ia_hcr_info_struct,
) -> VOID {
    let mut segmentation_err_flag: UWORD8 = 0 as UWORD8;
    let mut i: UWORD16 = 0;
    let mut p_remaining_bits_in_seg: *mut WORD8 = ((*ptr_hcr_info)
        .str_segment_info
        .p_remaining_bits_in_seg)
        .as_mut_ptr();
    let mut num_segment: UWORD32 = (*ptr_hcr_info).str_segment_info.num_segment
        as UWORD32;
    i = num_segment as UWORD16;
    while i as core::ffi::c_int != 0 as core::ffi::c_int {
        let fresh65 = p_remaining_bits_in_seg;
        p_remaining_bits_in_seg = p_remaining_bits_in_seg.offset(1);
        if *fresh65 as core::ffi::c_int != 0 as core::ffi::c_int {
            segmentation_err_flag = 1 as UWORD8;
        }
        i = i.wrapping_sub(1);
    }
    if segmentation_err_flag as core::ffi::c_int == 1 as core::ffi::c_int {
        (*ptr_hcr_info).str_dec_io.err_log |= ERROR_POS as UWORD32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hcr_decoder(
    mut ptr_hcr_info: *mut ia_hcr_info_struct,
    mut ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
    mut ptr_aac_tables: *mut ia_aac_dec_tables_struct,
    mut itt_bit_buff: *mut ia_bit_buf_struct,
) -> UWORD32 {
    let mut ptr_tmp1: WORD32 = 0;
    let mut ptr_tmp2: WORD32 = 0;
    let mut ptr_tmp3: WORD32 = 0;
    let mut ptr_tmp4: WORD32 = 0;
    let mut ptr_tmp5: WORD32 = 0;
    let mut bit_cnt_offset: WORD32 = 0;
    let mut save_bit_cnt: UWORD32 = (*itt_bit_buff).cnt_bits as UWORD32;
    ixheaacd_huff_calc_num_cwd(ptr_hcr_info);
    ixheaacd_huff_sort_sect_cb_cwd(ptr_hcr_info);
    if ixheaacd_hcr_prepare_segmentation_grid(ptr_hcr_info) != 0 as UWORD32 {
        return (*ptr_hcr_info).str_dec_io.err_log;
    }
    ixheaacd_huff_ext_sect_info(ptr_hcr_info);
    if (*ptr_hcr_info).str_dec_io.err_log & HCR_FATAL_PCW_ERROR_MASK as UWORD32
        != 0 as UWORD32
    {
        return (*ptr_hcr_info).str_dec_io.err_log;
    }
    ixheaacd_calc_num_ext_sorted_sect_sets(
        (*ptr_hcr_info).str_segment_info.num_segment as UWORD32,
        ((*ptr_hcr_info).sect_info.ptr_num_ext_sorted_cw_in_sect).as_mut_ptr(),
        (*ptr_hcr_info).sect_info.num_ext_sorted_cw_in_sect_idx,
        ((*ptr_hcr_info).sect_info.ptr_num_ext_sorted_sect_in_sets).as_mut_ptr(),
        (*ptr_hcr_info).sect_info.num_ext_sorted_sect_in_sets_idx,
    );
    ptr_tmp1 = (*ptr_hcr_info).sect_info.num_ext_sorted_cw_in_sect_idx;
    ptr_tmp2 = (*ptr_hcr_info).sect_info.ext_sorted_cw_idx;
    ptr_tmp3 = (*ptr_hcr_info).sect_info.num_ext_sorted_sect_in_sets_idx;
    ptr_tmp4 = (*ptr_hcr_info).str_dec_io.quant_spec_coeff_idx;
    ptr_tmp5 = (*ptr_hcr_info).sect_info.ext_sorted_sect_max_cb_len_idx;
    ixheaacd_decode_pcw(itt_bit_buff, ptr_hcr_info, ptr_aac_tables);
    if (*ptr_hcr_info).str_dec_io.err_log & HCR_FATAL_PCW_ERROR_MASK as UWORD32
        == 0 as UWORD32
    {
        ixheaacd_decode_non_pcw(itt_bit_buff, ptr_hcr_info, ptr_aac_tables);
    }
    ixheaacd_err_detect_segmentation_final(ptr_hcr_info);
    (*ptr_hcr_info).sect_info.num_ext_sorted_cw_in_sect_idx = ptr_tmp1;
    (*ptr_hcr_info).sect_info.ext_sorted_cw_idx = ptr_tmp2;
    (*ptr_hcr_info).sect_info.num_ext_sorted_sect_in_sets_idx = ptr_tmp3;
    (*ptr_hcr_info).str_dec_io.quant_spec_coeff_idx = ptr_tmp4;
    (*ptr_hcr_info).sect_info.ext_sorted_sect_max_cb_len_idx = ptr_tmp5;
    ixheaacd_hcr_reorder_quantized_spec_coeff(ptr_hcr_info, ptr_aac_dec_channel_info);
    bit_cnt_offset = (*itt_bit_buff).cnt_bits - save_bit_cnt as WORD32;
    if bit_cnt_offset != 0 {
        (*itt_bit_buff).cnt_bits += -bit_cnt_offset;
        (*itt_bit_buff).ptr_read_next = ((*itt_bit_buff).ptr_bit_buf_base)
            .offset(
                ((*itt_bit_buff).size - (*itt_bit_buff).cnt_bits
                    >> 3 as core::ffi::c_int) as isize,
            );
        (*itt_bit_buff).bit_pos = ((*itt_bit_buff).size as core::ffi::c_int
            - (*itt_bit_buff).cnt_bits as core::ffi::c_int & 7 as core::ffi::c_int)
            as WORD32;
    }
    return (*ptr_hcr_info).str_dec_io.err_log;
}
