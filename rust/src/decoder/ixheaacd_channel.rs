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
    fn ixheaacd_inverse_quantize(
        x_invquant: *mut WORD32,
        no_band: WORD,
        ixheaacd_pow_table_Q13: *mut WORD32,
        scratch_in: *mut WORD8,
    ) -> VOID;
    fn ixheaacd_right_shift_block(
        p_spectrum: *mut WORD32,
        length: WORD,
        shift_val: WORD,
    ) -> VOID;
    fn ixheaacd_decode_huffman(
        it_bit_buff: *mut ia_bit_buf_struct,
        cb_no: WORD32,
        spec_coef: *mut WORD32,
        sfb_offset: *mut WORD16,
        start: WORD,
        sfb: WORD,
        group_len: WORD,
        ptr_aac_tables: *mut ia_aac_dec_tables_struct,
        maximum_bins_short: WORD32,
    ) -> WORD;
    fn ixheaacd_huffman_dec_word2(
        it_bit_buff: *mut ia_bit_buf_struct,
        cb_no: WORD32,
        width: WORD32,
        ptr_aac_tables: *mut ia_aac_dec_tables_struct,
        x_invquant: *mut WORD32,
        scratch_ptr: *mut WORD8,
    ) -> WORD;
    fn ixheaacd_read_scale_factor_data(
        it_bit_buff: *mut ia_bit_buf_struct,
        ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
        ptr_aac_tables: *mut ia_aac_dec_tables_struct,
        object_type: WORD32,
    ) -> VOID;
    fn ixheaacd_read_section_data(
        it_bit_buff: *mut ia_bit_buf_struct,
        ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
        aac_spect_data_resil_flag: WORD32,
        aac_sect_data_resil_flag: WORD32,
        ptr_aac_tables: *mut ia_aac_dec_tables_struct,
    ) -> IA_ERRORCODE;
    fn ixheaacd_pns_process(
        ptr_aac_dec_channel_info: *mut *mut ia_aac_dec_channel_info_struct,
        channel: WORD32,
        ptr_aac_tables: *mut ia_aac_dec_tables_struct,
    ) -> VOID;
    fn ixheaacd_ms_stereo_process(
        ptr_aac_dec_channel_info: *mut *mut ia_aac_dec_channel_info_struct,
        ptr_aac_tables: *mut ia_aac_dec_tables_struct,
    ) -> VOID;
    fn ixheaacd_intensity_stereo_process(
        ptr_aac_dec_channel_info: *mut *mut ia_aac_dec_channel_info_struct,
        ptr_aac_tables: *mut ia_aac_dec_tables_struct,
        object_type: WORD32,
        aac_sf_data_resil_flag: WORD32,
        framelength: WORD16,
    ) -> VOID;
    fn ixheaacd_aac_tns_process(
        ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
        num_ch: WORD32,
        ptr_aac_tables: *mut ia_aac_dec_tables_struct,
        object_type: WORD,
        ar_flag: WORD32,
        predicted_spectrum: *mut WORD32,
    ) -> VOID;
    fn ixheaacd_adts_crc_start_reg(
        ptr_adts_crc_info: *mut ia_adts_crc_info_struct,
        it_bit_buff_src: *mut ia_bit_buf_struct,
        no_bits: WORD32,
    ) -> WORD32;
    fn ixheaacd_adts_crc_end_reg(
        ptr_adts_crc_info: *mut ia_adts_crc_info_struct,
        it_bit_buff_src: *mut ia_bit_buf_struct,
        reg: WORD32,
    ) -> VOID;
    fn ixheaacd_rvlc_read(
        itt_bit_buff: *mut ia_bit_buf_struct,
        ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
    );
    fn ixheaacd_rvlc_dec(
        ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
        ptr_aac_dec_static_channel_info: *mut ia_aac_dec_overlap_info,
        itt_bit_buff: *mut ia_bit_buf_struct,
    ) -> IA_ERRORCODE;
    fn ixheaacd_hcr_read(
        itt_bit_buff: *mut ia_bit_buf_struct,
        ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
        ele_type: WORD32,
    );
    fn ixheaacd_huff_code_reorder_init(
        ptr_hcr_info: *mut ia_hcr_info_struct,
        ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
        ptr_aac_tables: *mut ia_aac_dec_tables_struct,
        itt_bit_buff: *mut ia_bit_buf_struct,
    ) -> UWORD32;
    fn ixheaacd_hcr_decoder(
        ptr_hcr_info: *mut ia_hcr_info_struct,
        ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
        ptr_aac_tables: *mut ia_aac_dec_tables_struct,
        itt_bit_buff: *mut ia_bit_buf_struct,
    ) -> UWORD32;
    fn ixheaacd_huff_mute_erroneous_lines(ptr_hcr_info: *mut ia_hcr_info_struct) -> VOID;
    fn ixheaacd_lt_prediction(
        ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
        ltp: *mut ltp_info,
        spec: *mut WORD32,
        aac_tables_ptr: *mut ia_aac_dec_tables_struct,
        win_shape_prev: UWORD16,
        sr_index: UWORD32,
        object_type: UWORD32,
        frame_len: UWORD32,
        in_data: *mut WORD32,
        out_data: *mut WORD32,
    );
    static mut ixheaacd_scale_factor_process: Option<
        unsafe extern "C" fn(
            *mut WORD32,
            *mut WORD16,
            WORD,
            *mut WORD8,
            *mut WORD32,
            WORD32,
            WORD32,
            WORD32,
        ) -> VOID,
    >;
}
pub type size_t = usize;
pub type WORD8 = core::ffi::c_schar;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD16 = core::ffi::c_short;
pub type UWORD16 = core::ffi::c_ushort;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type FLOAT32 = core::ffi::c_float;
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
pub const AOT_ER_AAC_ELD: AUDIO_OBJECT_TYPE = 39;
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
pub const AOT_ER_AAC_LD: AUDIO_OBJECT_TYPE = 23;
pub const AOT_AAC_LTP: AUDIO_OBJECT_TYPE = 4;
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
pub const AOT_ER_AAC_LC: AUDIO_OBJECT_TYPE = 17;
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
pub const AOT_AAC_SSR: AUDIO_OBJECT_TYPE = 3;
pub const AOT_AAC_LC: AUDIO_OBJECT_TYPE = 2;
pub const AOT_AAC_MAIN: AUDIO_OBJECT_TYPE = 1;
pub const AOT_NULL_OBJECT: AUDIO_OBJECT_TYPE = 0;
pub const CRC_ADTS_RAW_IIND_ICS: core::ffi::c_int = 128 as core::ffi::c_int;
pub const IA_XHEAAC_DEC_API_NONFATAL_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES: core::ffi::c_int = 0x1804
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_EXCEEDS_MAX_HUFFDEC_VAL: core::ffi::c_int = 0x1806
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_DECODE_FRAME_ERROR: core::ffi::c_int = 0x1807
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_EXCEEDS_SFB_TRANSMITTED: core::ffi::c_int = 0x1808
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_GAIN_CONTROL_DATA_PRESENT: core::ffi::c_int = 0x1809
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_TNS_ORDER_ERROR: core::ffi::c_int = 0x180a
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_PREDICTION_DATA_PRESENT: core::ffi::c_int = 0x180b
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_PULSEDATA_ERROR: core::ffi::c_int = 0x180d
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_FATAL_TNS_RANGE_ERROR: core::ffi::c_uint = 0xffff9801
    as core::ffi::c_uint;
pub const MAX_ORDER_LONG: core::ffi::c_int = 12 as core::ffi::c_int;
pub const MAX_BINS_SHORT: core::ffi::c_int = 128 as core::ffi::c_int;
pub const MAX_SCALE_FACTOR_BANDS_SHORT: core::ffi::c_int = 16 as core::ffi::c_int;
pub const ZERO_HCB: core::ffi::c_int = 0 as core::ffi::c_int;
pub const NOISE_HCB: core::ffi::c_int = 13 as core::ffi::c_int;
pub const INTENSITY_HCB2: core::ffi::c_int = 14 as core::ffi::c_int;
pub const INTENSITY_HCB: core::ffi::c_int = 15 as core::ffi::c_int;
pub const PNS_BAND_FLAGS_MASK: WORD16 = (((1 as core::ffi::c_int as WORD16
    as core::ffi::c_int) << PNS_BAND_FLAGS_SHIFT) - 1 as core::ffi::c_int) as WORD16;
pub const PNS_BAND_FLAGS_SHIFT: core::ffi::c_int = 3 as core::ffi::c_int;
pub const MAX_LTP_SFB: core::ffi::c_int = 40 as core::ffi::c_int;
pub const MAX_LTP_SFB_SR_FIVE_PLUS_480: core::ffi::c_int = 31 as core::ffi::c_int;
pub const MAX_LTP_SFB_SR_FIVE_480: core::ffi::c_int = 38 as core::ffi::c_int;
pub const MAX_LTP_SFB_SR_FIVE_LESS_480: core::ffi::c_int = 36 as core::ffi::c_int;
pub const MAX_LTP_SFB_SR_FIVE_PLUS_512: core::ffi::c_int = 32 as core::ffi::c_int;
pub const MAX_LTP_SFB_SR_FIVE_512: core::ffi::c_int = 38 as core::ffi::c_int;
pub const MAX_LTP_SFB_SR_FIVE_LESS_512: core::ffi::c_int = 37 as core::ffi::c_int;
pub const EIGHT_SHORT_SEQUENCE: core::ffi::c_int = 2 as core::ffi::c_int;
pub const JOINT_STEREO_MAX_GROUPS: core::ffi::c_int = 8 as core::ffi::c_int;
pub const JOINT_STEREO_MAX_BANDS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const LEFT: core::ffi::c_int = 0 as core::ffi::c_int;
pub const RIGHT: core::ffi::c_int = 1 as core::ffi::c_int;
pub const ER_OBJECT_START: core::ffi::c_int = 17 as core::ffi::c_int;
pub const AAC_DEC_OK: core::ffi::c_int = IA_XHEAAC_DEC_API_NONFATAL_NO_ERROR;
pub const MAX_WINDOWS: core::ffi::c_int = 8 as core::ffi::c_int;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const IA_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_aac_showbits_32(
    mut ptr_read_next: *mut UWORD8,
    mut cnt_bits: WORD32,
    mut increment: *mut WORD32,
) -> UWORD32 {
    let mut v: *mut UWORD8 = ptr_read_next;
    let mut b: UWORD32 = 0 as UWORD32;
    let mut i: WORD32 = 0;
    let mut bumped: WORD32 = 0 as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < 4 as core::ffi::c_int {
        b = b << 8 as core::ffi::c_int;
        if cnt_bits > 0 as core::ffi::c_int {
            b = b | *v as UWORD32;
            v = v.offset(1);
            bumped += 1;
        }
        cnt_bits -= 8 as core::ffi::c_int;
        i += 1;
    }
    if !increment.is_null() {
        *increment = bumped;
    }
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_getscalefactorbandoffsets(
    mut ptr_ics_info: *mut ia_ics_info_struct,
    mut ptr_aac_tables: *mut ia_aac_dec_tables_struct,
) -> *mut WORD16 {
    if (*ptr_ics_info).window_sequence as core::ffi::c_int != EIGHT_SHORT_SEQUENCE {
        return ((*ptr_aac_tables).sfb_long_table).as_mut_ptr()
    } else {
        return ((*ptr_aac_tables).sfb_short_table).as_mut_ptr()
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_getscalefactorbandwidth(
    mut ptr_ics_info: *mut ia_ics_info_struct,
    mut ptr_aac_tables: *mut ia_aac_dec_tables_struct,
) -> *mut WORD8 {
    if (*ptr_ics_info).frame_length as core::ffi::c_int == 512 as core::ffi::c_int {
        return &mut *(*((*ptr_aac_tables).scale_fac_bands_512)
            .as_mut_ptr()
            .offset((*ptr_ics_info).sampling_rate_index as isize))
            .offset(0 as core::ffi::c_int as isize) as *mut WORD8
    } else if (*ptr_ics_info).frame_length as core::ffi::c_int
        == 1024 as core::ffi::c_int
    {
        return &mut *(*((*ptr_aac_tables).scale_factor_bands_long)
            .as_mut_ptr()
            .offset((*ptr_ics_info).sampling_rate_index as isize))
            .offset(0 as core::ffi::c_int as isize) as *mut WORD8
    } else {
        return &mut *(*((*ptr_aac_tables).scale_fac_bands_480)
            .as_mut_ptr()
            .offset((*ptr_ics_info).sampling_rate_index as isize))
            .offset(0 as core::ffi::c_int as isize) as *mut WORD8
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_cblock_scale_spect_data(
    mut ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
    mut ptr_aac_tables: *mut ia_aac_dec_tables_struct,
    mut total_channels: WORD32,
    mut object_type: WORD32,
    mut aac_sf_data_resil_flag: WORD32,
) {
    let mut ptr_spect_coeff: *mut WORD32 = (*ptr_aac_dec_channel_info).ptr_spec_coeff;
    let mut ptr_sfb_width: *mut WORD8 = ixheaacd_getscalefactorbandwidth(
        &mut (*ptr_aac_dec_channel_info).str_ics_info,
        ptr_aac_tables,
    );
    let mut ptr_scale_fac: *mut WORD16 = (*ptr_aac_dec_channel_info).ptr_scale_factor;
    let mut tot_bands: WORD = (*ptr_aac_dec_channel_info).str_ics_info.max_sfb as WORD;
    let mut num_win_grp: WORD = 0;
    let mut group_len: WORD = 0;
    let mut ptr_scale_table: *mut WORD32 = ((*(*ptr_aac_tables).pstr_block_tables)
        .scale_table)
        .as_mut_ptr();
    let mut ptr_ics_info: *mut ia_ics_info_struct = &mut (*ptr_aac_dec_channel_info)
        .str_ics_info;
    if object_type == AOT_ER_AAC_LC as core::ffi::c_int {
        ptr_sfb_width = (*ptr_aac_tables)
            .str_aac_sfb_info[(*ptr_ics_info).window_sequence as usize]
            .sfb_width;
    }
    num_win_grp = 0 as core::ffi::c_int as WORD;
    while num_win_grp < (*ptr_ics_info).num_window_groups as core::ffi::c_int {
        group_len = 0 as core::ffi::c_int as WORD;
        while group_len
            < (*ptr_ics_info).window_group_length[num_win_grp as usize]
                as core::ffi::c_int
        {
            (Some(ixheaacd_scale_factor_process.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                &mut *ptr_spect_coeff.offset(0 as core::ffi::c_int as isize),
                &mut *ptr_scale_fac.offset(0 as core::ffi::c_int as isize),
                tot_bands,
                ptr_sfb_width,
                ptr_scale_table,
                total_channels,
                object_type,
                aac_sf_data_resil_flag,
            );
            ptr_spect_coeff = ptr_spect_coeff.offset(MAX_BINS_SHORT as isize);
            group_len += 1;
        }
        ptr_scale_fac = ptr_scale_fac.offset(MAX_SCALE_FACTOR_BANDS_SHORT as isize);
        num_win_grp += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_read_pulse_data(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut ptr_pulse_info: *mut ia_pulse_info_struct,
    mut ptr_aac_tables: *mut ia_aac_dec_tables_struct,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut total_offset: WORD32 = 0;
    let mut error_code: WORD32 = 0 as WORD32;
    let mut value: WORD32 = ixheaacd_read_bits_buf(it_bit_buff, 8 as WORD);
    (*ptr_pulse_info).number_pulse = (value >> 6 as core::ffi::c_int) as WORD16;
    (*ptr_pulse_info).pulse_start_band = (value as core::ffi::c_int
        & 0x3f as core::ffi::c_int) as WORD16;
    if (*ptr_pulse_info).pulse_start_band as core::ffi::c_int >= 52 as core::ffi::c_int {
        return IA_XHEAAC_DEC_EXE_NONFATAL_PULSEDATA_ERROR;
    }
    total_offset = *((*ptr_aac_tables)
        .str_aac_sfb_info[0 as core::ffi::c_int as usize]
        .sfb_index)
        .offset((*ptr_pulse_info).pulse_start_band as isize) as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*ptr_pulse_info).number_pulse as core::ffi::c_int + 1 as core::ffi::c_int
    {
        let mut value_0: WORD32 = ixheaacd_read_bits_buf(it_bit_buff, 9 as WORD);
        (*ptr_pulse_info).pulse_offset[i as usize] = (value_0 >> 4 as core::ffi::c_int)
            as WORD8;
        (*ptr_pulse_info).pulse_amp[i as usize] = (value_0 as core::ffi::c_int
            & 0xf as core::ffi::c_int) as WORD8;
        total_offset += (*ptr_pulse_info).pulse_offset[i as usize] as core::ffi::c_int;
        if total_offset >= 1024 as core::ffi::c_int {
            error_code = IA_XHEAAC_DEC_EXE_NONFATAL_PULSEDATA_ERROR;
        }
        i += 1;
    }
    return error_code;
}
unsafe extern "C" fn ixheaacd_read_block_data(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
    mut ptr_aac_tables: *mut ia_aac_dec_tables_struct,
    mut total_channels: WORD32,
    mut frame_size: WORD32,
    mut object_type: WORD32,
    mut aac_spect_data_resil_flag: WORD32,
    mut aac_sect_data_resil_flag: WORD32,
    mut aac_sf_data_resil_flag: WORD32,
    mut ele_type: WORD32,
    mut ptr_aac_dec_static_channel_info: *mut ia_aac_dec_overlap_info,
) -> IA_ERRORCODE {
    let mut gain_control_data_present: FLAG = 0;
    let mut error_code: WORD16 = AAC_DEC_OK as WORD16;
    if (*ptr_aac_dec_channel_info).str_ics_info.window_sequence as core::ffi::c_int
        == EIGHT_SHORT_SEQUENCE
    {
        memset(
            (*ptr_aac_dec_channel_info).ptr_scale_factor as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (MAX_WINDOWS * MAX_SCALE_FACTOR_BANDS_SHORT * 3 as core::ffi::c_int)
                as size_t,
        );
    }
    error_code = ixheaacd_read_section_data(
        it_bit_buff,
        ptr_aac_dec_channel_info,
        aac_spect_data_resil_flag,
        aac_sect_data_resil_flag,
        ptr_aac_tables,
    ) as WORD16;
    if error_code != 0 {
        return error_code as IA_ERRORCODE;
    }
    if aac_sf_data_resil_flag != 0
        && (object_type == AOT_ER_AAC_ELD as core::ffi::c_int
            || object_type == AOT_ER_AAC_LD as core::ffi::c_int)
    {
        ixheaacd_rvlc_read(it_bit_buff, ptr_aac_dec_channel_info);
    } else {
        ixheaacd_read_scale_factor_data(
            it_bit_buff,
            ptr_aac_dec_channel_info,
            ptr_aac_tables,
            object_type,
        );
    }
    error_code = 0 as WORD16;
    if object_type != AOT_ER_AAC_ELD as core::ffi::c_int {
        (*ptr_aac_dec_channel_info).str_pulse_info.pulse_data_present = ixheaacd_read_bits_buf(
            it_bit_buff,
            1 as WORD,
        ) as FLAG;
        if (*ptr_aac_dec_channel_info).str_pulse_info.pulse_data_present != 0 {
            error_code = ixheaacd_read_pulse_data(
                it_bit_buff,
                &mut (*ptr_aac_dec_channel_info).str_pulse_info,
                ptr_aac_tables,
            ) as WORD16;
        }
        if error_code != 0 {
            return error_code as IA_ERRORCODE;
        }
    }
    (*ptr_aac_dec_channel_info).str_tns_info.tns_data_present = ixheaacd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    );
    if object_type < ER_OBJECT_START {
        error_code = 0 as WORD16;
        if (*ptr_aac_dec_channel_info).str_tns_info.tns_data_present != 0 {
            error_code = ixheaacd_read_tns_data(it_bit_buff, ptr_aac_dec_channel_info);
        }
        if error_code != 0 {
            return error_code as IA_ERRORCODE;
        }
    }
    if object_type != AOT_ER_AAC_ELD as core::ffi::c_int {
        gain_control_data_present = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD)
            as FLAG;
        if gain_control_data_present != 0 {
            return IA_XHEAAC_DEC_EXE_NONFATAL_GAIN_CONTROL_DATA_PRESENT as WORD16
                as IA_ERRORCODE;
        }
    }
    if object_type == AOT_ER_AAC_ELD as core::ffi::c_int {
        if (*ptr_aac_dec_channel_info).str_tns_info.tns_data_present != 0 {
            error_code = ixheaacd_read_tns_data(it_bit_buff, ptr_aac_dec_channel_info);
        }
        if error_code != 0 {
            return error_code as IA_ERRORCODE;
        }
    }
    if aac_spect_data_resil_flag != 0
        && (object_type == AOT_ER_AAC_ELD as core::ffi::c_int
            || object_type == AOT_ER_AAC_LD as core::ffi::c_int
            || object_type == AOT_ER_AAC_LC as core::ffi::c_int)
    {
        ixheaacd_hcr_read(it_bit_buff, ptr_aac_dec_channel_info, ele_type);
    }
    if aac_sf_data_resil_flag != 0
        && (object_type == AOT_ER_AAC_ELD as core::ffi::c_int
            || object_type == AOT_ER_AAC_LD as core::ffi::c_int)
    {
        error_code = ixheaacd_rvlc_dec(
            ptr_aac_dec_channel_info,
            ptr_aac_dec_static_channel_info,
            it_bit_buff,
        ) as WORD16;
        if error_code != 0 {
            return error_code as IA_ERRORCODE;
        }
        (*it_bit_buff).bit_pos = 7 as WORD32 - (*it_bit_buff).bit_pos;
    }
    if object_type == AOT_ER_AAC_LD as core::ffi::c_int
        || object_type == AOT_ER_AAC_LC as core::ffi::c_int
    {
        if (*ptr_aac_dec_channel_info).str_tns_info.tns_data_present != 0 {
            error_code = ixheaacd_read_tns_data(it_bit_buff, ptr_aac_dec_channel_info);
        }
        if error_code != 0 {
            return error_code as IA_ERRORCODE;
        }
    }
    (*it_bit_buff).bit_pos = 7 as WORD32 - (*it_bit_buff).bit_pos;
    error_code = ixheaacd_read_spectral_data(
        it_bit_buff,
        ptr_aac_dec_channel_info,
        ptr_aac_tables,
        total_channels,
        frame_size,
        object_type,
        aac_spect_data_resil_flag,
        aac_sf_data_resil_flag,
    ) as WORD16;
    (*it_bit_buff).bit_pos = 7 as WORD32 - (*it_bit_buff).bit_pos;
    return error_code as IA_ERRORCODE;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_ltp_decode(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut ptr_ics_info: *mut ia_ics_info_struct,
    mut object_type: WORD32,
    mut frame_size: WORD32,
    mut ch: WORD32,
) -> IA_ERRORCODE {
    let mut retval: IA_ERRORCODE = AAC_DEC_OK;
    if (*ptr_ics_info).predictor_data_present != 0 {
        if ch == 0 as core::ffi::c_int {
            ixheaacd_init_ltp_object(&mut (*ptr_ics_info).ltp);
            (*ptr_ics_info).ltp.data_present = ixheaacd_read_bits_buf(
                it_bit_buff,
                1 as WORD,
            ) as UWORD8;
            if (*ptr_ics_info).ltp.data_present != 0 {
                retval = ixheaacd_ltp_data(
                    object_type,
                    ptr_ics_info,
                    &mut (*ptr_ics_info).ltp,
                    it_bit_buff,
                    frame_size,
                ) as IA_ERRORCODE;
                if retval > 0 as core::ffi::c_int {
                    return retval;
                }
            }
        } else {
            ixheaacd_init_ltp_object(&mut (*ptr_ics_info).ltp2);
            (*ptr_ics_info).ltp2.data_present = ixheaacd_read_bits_buf(
                it_bit_buff,
                1 as WORD,
            ) as UWORD8;
            if (*ptr_ics_info).ltp2.data_present != 0 {
                retval = ixheaacd_ltp_data(
                    object_type,
                    ptr_ics_info,
                    &mut (*ptr_ics_info).ltp2,
                    it_bit_buff,
                    frame_size,
                ) as IA_ERRORCODE;
                if retval > 0 as core::ffi::c_int {
                    return retval;
                }
            }
        }
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_ics_read(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut ptr_ics_info: *mut ia_ics_info_struct,
    mut num_swb_window: *mut WORD8,
    mut object_type: WORD32,
    mut common_window: WORD32,
    mut frame_size: WORD32,
) -> WORD16 {
    let mut i: WORD = 0;
    let mut mask: WORD = 0;
    let mut value: WORD = 0 as WORD;
    if object_type == AOT_ER_AAC_ELD as core::ffi::c_int {
        (*ptr_ics_info).window_sequence = 0 as WORD16;
        (*ptr_ics_info).window_shape = 1 as WORD16;
    } else {
        if object_type != AOT_ER_AAC_LD as core::ffi::c_int {
            if frame_size == 960 as core::ffi::c_int {
                (*ptr_ics_info).frame_length = 960 as WORD16;
            } else {
                (*ptr_ics_info).frame_length = 1024 as WORD16;
            }
        }
        value = ixheaacd_read_bits_buf(it_bit_buff, 4 as WORD) as WORD;
        (*ptr_ics_info).window_sequence = ((value as core::ffi::c_int
            & 0x6 as core::ffi::c_int) >> 1 as core::ffi::c_int) as WORD16;
        (*ptr_ics_info).window_shape = (value as core::ffi::c_int
            & 0x1 as core::ffi::c_int) as WORD16;
    }
    if (*ptr_ics_info).window_sequence as core::ffi::c_int != EIGHT_SHORT_SEQUENCE {
        (*ptr_ics_info).num_swb_window = *num_swb_window
            .offset(0 as core::ffi::c_int as isize) as WORD16;
        (*ptr_ics_info).num_window_groups = 1 as WORD16;
        (*ptr_ics_info).window_group_length[0 as core::ffi::c_int as usize] = 1 as WORD8;
        if object_type == AOT_ER_AAC_ELD as core::ffi::c_int {
            (*ptr_ics_info).max_sfb = ixheaacd_read_bits_buf(it_bit_buff, 6 as WORD)
                as WORD16;
            if (*ptr_ics_info).max_sfb as core::ffi::c_int == 0 as core::ffi::c_int {
                (*ptr_ics_info).num_swb_window = 0 as WORD16;
            }
        } else {
            value = ixheaacd_read_bits_buf(it_bit_buff, 7 as WORD) as WORD;
            (*ptr_ics_info).max_sfb = ((value as core::ffi::c_int
                & 0x7e as core::ffi::c_int) >> 1 as core::ffi::c_int) as WORD16;
        }
        if object_type != AOT_ER_AAC_LD as core::ffi::c_int
            && object_type != AOT_AAC_LTP as core::ffi::c_int
        {
            if value as core::ffi::c_int & 1 as core::ffi::c_int != 0 {
                return IA_XHEAAC_DEC_EXE_NONFATAL_PREDICTION_DATA_PRESENT as WORD16;
            }
        } else {
            (*ptr_ics_info).predictor_data_present = (value as core::ffi::c_int
                & 1 as core::ffi::c_int) as WORD16;
            if (*ptr_ics_info).predictor_data_present != 0 {
                let mut retval: WORD32 = AAC_DEC_OK;
                ixheaacd_init_ltp_object(&mut (*ptr_ics_info).ltp);
                if object_type < ER_OBJECT_START {
                    (*ptr_ics_info).ltp.data_present = ixheaacd_read_bits_buf(
                        it_bit_buff,
                        1 as WORD,
                    ) as UWORD8;
                    if (*ptr_ics_info).ltp.data_present as core::ffi::c_int
                        & 1 as core::ffi::c_int != 0
                    {
                        retval = ixheaacd_ltp_data(
                            object_type,
                            ptr_ics_info,
                            &mut (*ptr_ics_info).ltp,
                            it_bit_buff,
                            frame_size,
                        );
                        if retval > 0 as core::ffi::c_int {
                            return retval as WORD16;
                        }
                    }
                    if common_window != 0 {
                        ixheaacd_init_ltp_object(&mut (*ptr_ics_info).ltp2);
                        (*ptr_ics_info).ltp2.data_present = ixheaacd_read_bits_buf(
                            it_bit_buff,
                            1 as WORD,
                        ) as UWORD8;
                        if (*ptr_ics_info).ltp2.data_present as core::ffi::c_int
                            & 1 as core::ffi::c_int != 0
                        {
                            retval = ixheaacd_ltp_data(
                                object_type,
                                ptr_ics_info,
                                &mut (*ptr_ics_info).ltp2,
                                it_bit_buff,
                                frame_size,
                            );
                            if retval > 0 as core::ffi::c_int {
                                return retval as WORD16;
                            }
                        }
                    }
                }
                if object_type == AOT_ER_AAC_ELD as core::ffi::c_int
                    || object_type == AOT_ER_AAC_LD as core::ffi::c_int
                {
                    if common_window == 0 && object_type >= ER_OBJECT_START {
                        (*ptr_ics_info).ltp.data_present = ixheaacd_read_bits_buf(
                            it_bit_buff,
                            1 as WORD,
                        ) as UWORD8;
                        if (*ptr_ics_info).ltp.data_present as core::ffi::c_int
                            & 1 as core::ffi::c_int != 0
                        {
                            retval = ixheaacd_ltp_data(
                                object_type,
                                ptr_ics_info,
                                &mut (*ptr_ics_info).ltp,
                                it_bit_buff,
                                frame_size,
                            );
                            if retval < 0 as core::ffi::c_int {
                                return retval as WORD16;
                            }
                        }
                    }
                }
            }
        }
    } else {
        let mut num_groups: WORD32 = 0 as WORD32;
        let mut scale_factor_grouping: WORD32 = 0;
        (*ptr_ics_info).num_swb_window = *num_swb_window
            .offset(1 as core::ffi::c_int as isize) as WORD16;
        value = ixheaacd_read_bits_buf(it_bit_buff, 11 as WORD) as WORD;
        (*ptr_ics_info).max_sfb = ((value as core::ffi::c_int
            & 0x780 as core::ffi::c_int) >> 7 as core::ffi::c_int) as WORD16;
        scale_factor_grouping = (value as core::ffi::c_int & 0x7f as core::ffi::c_int)
            as WORD32;
        mask = 0x40 as core::ffi::c_int as WORD;
        i = 0 as core::ffi::c_int as WORD;
        while i < 7 as core::ffi::c_int {
            (*ptr_ics_info).window_group_length[i as usize] = 1 as WORD8;
            if scale_factor_grouping as WORD & mask != 0 {
                (*ptr_ics_info).window_group_length[num_groups as usize] = ((*ptr_ics_info)
                    .window_group_length[num_groups as usize] as core::ffi::c_int
                    + 1 as core::ffi::c_int) as WORD8;
            } else {
                num_groups = (num_groups as core::ffi::c_int + 1 as core::ffi::c_int)
                    as WORD32;
            }
            mask = mask >> 1 as core::ffi::c_int;
            i += 1;
        }
        (*ptr_ics_info).window_group_length[7 as core::ffi::c_int as usize] = 1 as WORD8;
        (*ptr_ics_info).num_window_groups = (num_groups as core::ffi::c_int
            + 1 as core::ffi::c_int) as WORD16;
    }
    if (*ptr_ics_info).max_sfb as core::ffi::c_int
        > (*ptr_ics_info).num_swb_window as core::ffi::c_int
    {
        return IA_XHEAAC_DEC_EXE_NONFATAL_EXCEEDS_SFB_TRANSMITTED as WORD16;
    }
    return AAC_DEC_OK as WORD16;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_individual_ch_stream(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut aac_dec_handle: *mut ia_aac_decoder_struct,
    mut num_ch: WORD32,
    mut frame_size: WORD32,
    mut total_channels: WORD32,
    mut object_type: WORD32,
    mut eld_specific_config: ia_eld_specific_config_struct,
    mut ele_type: WORD32,
) -> WORD16 {
    let mut error_code: WORD16 = AAC_DEC_OK as WORD16;
    let mut ch: WORD32 = 0;
    let mut crc_reg: WORD32 = 0 as WORD32;
    ch = 0 as core::ffi::c_int as WORD32;
    while ch < num_ch {
        let mut ptr_aac_dec_ch_info: *mut ia_aac_dec_channel_info_struct = (*aac_dec_handle)
            .pstr_aac_dec_ch_info[ch as usize];
        let mut ptr_ics_info: *mut ia_ics_info_struct = &mut (*ptr_aac_dec_ch_info)
            .str_ics_info;
        if ch == 1 as core::ffi::c_int {
            if (*(*it_bit_buff).pstr_adts_crc_info).crc_active as core::ffi::c_int
                == 1 as core::ffi::c_int
                && ((*(*it_bit_buff).pstr_adts_crc_info).no_reg as core::ffi::c_int)
                    < 7 as core::ffi::c_int
            {
                crc_reg = ixheaacd_adts_crc_start_reg(
                    (*it_bit_buff).pstr_adts_crc_info,
                    it_bit_buff,
                    CRC_ADTS_RAW_IIND_ICS,
                );
            }
        }
        (*ptr_aac_dec_ch_info).global_gain = ixheaacd_read_bits_buf(
            it_bit_buff,
            8 as WORD,
        ) as WORD16;
        if (*(*aac_dec_handle).pstr_aac_dec_ch_info[LEFT as usize]).common_window == 0 {
            error_code = ixheaacd_ics_read(
                it_bit_buff,
                ptr_ics_info,
                ((*aac_dec_handle).num_swb_window).as_mut_ptr(),
                object_type,
                (*(*aac_dec_handle).pstr_aac_dec_ch_info[LEFT as usize]).common_window
                    as WORD32,
                (*aac_dec_handle).samples_per_frame,
            );
            if ch == 1 as core::ffi::c_int {
                (*(*aac_dec_handle)
                    .pstr_aac_dec_ch_info[(ch as core::ffi::c_int
                    - 1 as core::ffi::c_int) as usize])
                    .str_ics_info
                    .ltp2
                    .lag = (*ptr_ics_info).ltp.lag;
            }
            if error_code != 0 {
                if (*it_bit_buff).cnt_bits < 0 as core::ffi::c_int {
                    error_code = IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES
                        as WORD16;
                }
                return error_code;
            }
        }
        error_code = ixheaacd_read_block_data(
            it_bit_buff,
            ptr_aac_dec_ch_info,
            (*aac_dec_handle).pstr_aac_tables,
            total_channels,
            frame_size,
            object_type,
            eld_specific_config.aac_spect_data_resil_flag,
            eld_specific_config.aac_sect_data_resil_flag,
            eld_specific_config.aac_sf_data_resil_flag,
            ele_type,
            (*aac_dec_handle).pstr_aac_dec_overlap_info[ch as usize],
        ) as WORD16;
        if error_code != 0 {
            if (*it_bit_buff).cnt_bits < 0 as core::ffi::c_int {
                error_code = IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES
                    as WORD16;
            }
            return error_code;
        }
        if ch == 0 as core::ffi::c_int {
            if object_type == AOT_ER_AAC_LD as core::ffi::c_int
                && (*(*aac_dec_handle).pstr_aac_dec_ch_info[LEFT as usize]).common_window
                    as core::ffi::c_int != 0 && ele_type == ID_CPE as core::ffi::c_int
            {
                let mut temp: IA_ERRORCODE = ixheaacd_ltp_decode(
                    it_bit_buff,
                    ptr_ics_info,
                    object_type,
                    (*aac_dec_handle).samples_per_frame,
                    1 as WORD32,
                );
                if temp != 0 as core::ffi::c_int {
                    return temp as WORD16;
                }
                (*(*aac_dec_handle)
                    .pstr_aac_dec_ch_info[(ch as core::ffi::c_int
                    + 1 as core::ffi::c_int) as usize])
                    .str_ics_info
                    .ltp
                    .lag = (*ptr_ics_info).ltp2.lag;
            }
        }
        if ch == 1 as core::ffi::c_int {
            if (*(*it_bit_buff).pstr_adts_crc_info).crc_active as core::ffi::c_int
                == 1 as core::ffi::c_int
            {
                ixheaacd_adts_crc_end_reg(
                    (*it_bit_buff).pstr_adts_crc_info,
                    it_bit_buff,
                    crc_reg,
                );
            }
        }
        ch += 1;
    }
    return error_code;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_read_ms_data(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut ptr_aac_dec_ch_info: *mut ia_aac_dec_channel_info_struct,
) -> VOID {
    let mut num_win_group: WORD32 = 0;
    let mut sfb: WORD32 = 0;
    let mut ms_mask_present: WORD32 = 0;
    let mut ptr_ms_used: *mut UWORD8 = &mut *(*((*(*ptr_aac_dec_ch_info)
        .pstr_stereo_info)
        .ms_used)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize))
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut UWORD8;
    let mut num_window_groups: WORD32 = (*ptr_aac_dec_ch_info)
        .str_ics_info
        .num_window_groups as WORD32;
    let mut max_sfb: WORD16 = (*ptr_aac_dec_ch_info).str_ics_info.max_sfb;
    ms_mask_present = ixheaacd_read_bits_buf(it_bit_buff, 2 as WORD);
    if ms_mask_present < 1 as core::ffi::c_int {
        memset(
            ptr_ms_used as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<UWORD8>() as size_t)
                .wrapping_mul(JOINT_STEREO_MAX_BANDS as size_t)
                .wrapping_mul(JOINT_STEREO_MAX_GROUPS as size_t),
        );
    } else if ms_mask_present == 1 as core::ffi::c_int {
        num_win_group = 0 as core::ffi::c_int as WORD32;
        while num_win_group < num_window_groups {
            sfb = 0 as core::ffi::c_int as WORD32;
            while sfb < max_sfb as core::ffi::c_int {
                (*(*ptr_aac_dec_ch_info).pstr_stereo_info)
                    .ms_used[num_win_group as usize][sfb as usize] = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    1 as WORD,
                ) as UWORD8;
                sfb += 1;
            }
            num_win_group += 1;
        }
    } else {
        num_win_group = 0 as core::ffi::c_int as WORD32;
        while num_win_group < num_window_groups {
            ptr_ms_used = &mut *(*((*(*ptr_aac_dec_ch_info).pstr_stereo_info).ms_used)
                .as_mut_ptr()
                .offset(num_win_group as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut UWORD8;
            memset(
                ptr_ms_used as *mut core::ffi::c_void,
                1 as core::ffi::c_int,
                (max_sfb as size_t)
                    .wrapping_mul(::core::mem::size_of::<UWORD8>() as size_t),
            );
            num_win_group += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_channel_pair_process(
    mut ptr_aac_dec_channel_info: *mut *mut ia_aac_dec_channel_info_struct,
    mut num_ch: WORD32,
    mut ptr_aac_tables: *mut ia_aac_dec_tables_struct,
    mut total_channels: WORD32,
    mut object_type: WORD32,
    mut aac_spect_data_resil_flag: WORD32,
    mut aac_sf_data_resil_flag: WORD32,
    mut in_data: *mut WORD32,
    mut out_data: *mut WORD32,
    mut self_ptr: *mut core::ffi::c_void,
) -> IA_ERRORCODE {
    let mut channel: WORD32 = 0;
    let mut err: IA_ERRORCODE = IA_NO_ERROR;
    let mut self_0: *mut ia_aac_decoder_struct = self_ptr as *mut ia_aac_decoder_struct;
    if aac_spect_data_resil_flag != 0
        && (object_type == AOT_ER_AAC_LD as core::ffi::c_int
            || object_type == AOT_ER_AAC_ELD as core::ffi::c_int
            || object_type == AOT_ER_AAC_LC as core::ffi::c_int)
    {
        channel = 0 as core::ffi::c_int as WORD32;
        while channel < num_ch {
            err = ixheaacd_cblock_inv_quant_spect_data(
                *ptr_aac_dec_channel_info.offset(channel as isize),
                ptr_aac_tables,
            ) as IA_ERRORCODE;
            if err != 0 {
                return err;
            }
            ixheaacd_cblock_scale_spect_data(
                *ptr_aac_dec_channel_info.offset(channel as isize),
                ptr_aac_tables,
                num_ch,
                object_type,
                aac_sf_data_resil_flag,
            );
            channel += 1;
        }
    }
    if num_ch > 1 as core::ffi::c_int {
        if (**ptr_aac_dec_channel_info.offset(LEFT as isize)).common_window != 0 {
            if (**ptr_aac_dec_channel_info.offset(LEFT as isize)).str_pns_info.pns_active
                as core::ffi::c_int != 0
                || (**ptr_aac_dec_channel_info.offset(RIGHT as isize))
                    .str_pns_info
                    .pns_active as core::ffi::c_int != 0
            {
                ixheaacd_map_ms_mask_pns(ptr_aac_dec_channel_info);
            }
            ixheaacd_ms_stereo_process(ptr_aac_dec_channel_info, ptr_aac_tables);
        }
        ixheaacd_intensity_stereo_process(
            ptr_aac_dec_channel_info,
            ptr_aac_tables,
            object_type,
            aac_sf_data_resil_flag,
            (**ptr_aac_dec_channel_info.offset(LEFT as isize)).str_ics_info.frame_length,
        );
    }
    channel = 0 as core::ffi::c_int as WORD32;
    while channel < num_ch {
        let mut p_spectrum: *mut WORD32 = (**ptr_aac_dec_channel_info
            .offset(channel as isize))
            .ptr_spec_coeff;
        if total_channels > 2 as core::ffi::c_int {
            if (**ptr_aac_dec_channel_info.offset(channel as isize))
                .str_ics_info
                .window_sequence as core::ffi::c_int != EIGHT_SHORT_SEQUENCE
            {
                let mut band_offsets: *mut WORD16 = ((*ptr_aac_tables).sfb_long_table)
                    .as_mut_ptr();
                let mut no_spec_coeff: WORD32 = *band_offsets
                    .offset(
                        (**ptr_aac_dec_channel_info.offset(channel as isize))
                            .str_ics_info
                            .max_sfb as isize,
                    ) as WORD32;
                ixheaacd_right_shift_block(p_spectrum, no_spec_coeff as WORD, 3 as WORD);
            } else {
                ixheaacd_right_shift_block(p_spectrum, 1024 as WORD, 3 as WORD);
            }
        }
        ixheaacd_pns_process(ptr_aac_dec_channel_info, channel, ptr_aac_tables);
        if object_type == AOT_ER_AAC_LD as core::ffi::c_int
            || object_type == AOT_AAC_LTP as core::ffi::c_int
        {
            if channel == 0 as core::ffi::c_int {
                let mut ltp1: *mut ltp_info = &mut (**ptr_aac_dec_channel_info
                    .offset(channel as isize))
                    .str_ics_info
                    .ltp;
                ixheaacd_lt_prediction(
                    *ptr_aac_dec_channel_info.offset(channel as isize),
                    ltp1,
                    p_spectrum,
                    ptr_aac_tables,
                    (*(*self_0).ptr_aac_dec_static_channel_info[LEFT as usize])
                        .overlap_add_data
                        .win_shape as UWORD16,
                    (*self_0).sampling_rate_index as UWORD32,
                    object_type as UWORD32,
                    (*self_0).samples_per_frame as UWORD32,
                    in_data,
                    out_data,
                );
            } else {
                let mut ltp2: *mut ltp_info = if (*(*self_0)
                    .pstr_aac_dec_ch_info[0 as core::ffi::c_int as usize])
                    .common_window as core::ffi::c_int != 0
                {
                    &mut (**ptr_aac_dec_channel_info
                        .offset(0 as core::ffi::c_int as isize))
                        .str_ics_info
                        .ltp2
                } else {
                    &mut (**ptr_aac_dec_channel_info
                        .offset(1 as core::ffi::c_int as isize))
                        .str_ics_info
                        .ltp
                };
                ixheaacd_lt_prediction(
                    *ptr_aac_dec_channel_info.offset(channel as isize),
                    ltp2,
                    p_spectrum,
                    ptr_aac_tables,
                    (*(*self_0).ptr_aac_dec_static_channel_info[RIGHT as usize])
                        .overlap_add_data
                        .win_shape as UWORD16,
                    (*self_0).sampling_rate_index as UWORD32,
                    object_type as UWORD32,
                    (*self_0).samples_per_frame as UWORD32,
                    in_data,
                    out_data,
                );
            }
        }
        if (**ptr_aac_dec_channel_info.offset(channel as isize))
            .str_tns_info
            .tns_data_present != 0
        {
            ixheaacd_aac_tns_process(
                *ptr_aac_dec_channel_info.offset(channel as isize),
                total_channels,
                ptr_aac_tables,
                object_type as WORD,
                1 as WORD32,
                0 as *mut WORD32,
            );
        }
        channel += 1;
    }
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_set_corr_info(
    mut ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
    mut pns_band: WORD16,
) -> VOID {
    let mut ptr_corr_info: *mut ia_pns_correlation_info_struct = (*ptr_aac_dec_channel_info)
        .pstr_pns_corr_info;
    (*ptr_corr_info)
        .correlated[(pns_band as core::ffi::c_int >> PNS_BAND_FLAGS_SHIFT) as usize] = ((*ptr_corr_info)
        .correlated[(pns_band as core::ffi::c_int >> PNS_BAND_FLAGS_SHIFT) as usize]
        as core::ffi::c_int
        | (1 as core::ffi::c_int)
            << (pns_band as core::ffi::c_int & PNS_BAND_FLAGS_MASK as core::ffi::c_int))
        as UWORD8;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_map_ms_mask_pns(
    mut ptr_aac_dec_channel_info: *mut *mut ia_aac_dec_channel_info_struct,
) -> VOID {
    let mut num_win_group: WORD32 = 0;
    let mut sfb: WORD32 = 0;
    num_win_group = 0 as core::ffi::c_int as WORD32;
    while num_win_group
        < (**ptr_aac_dec_channel_info.offset(LEFT as isize))
            .str_ics_info
            .num_window_groups as core::ffi::c_int
    {
        sfb = 0 as core::ffi::c_int as WORD32;
        while sfb
            < (**ptr_aac_dec_channel_info.offset(LEFT as isize)).str_ics_info.max_sfb
                as core::ffi::c_int
        {
            if (*(**ptr_aac_dec_channel_info.offset(LEFT as isize)).pstr_stereo_info)
                .ms_used[num_win_group as usize][sfb as usize] != 0
            {
                let mut pns_band: WORD16 = ((num_win_group << 4 as core::ffi::c_int)
                    + sfb) as WORD16;
                ixheaacd_set_corr_info(
                    *ptr_aac_dec_channel_info.offset(LEFT as isize),
                    pns_band,
                );
                if (**ptr_aac_dec_channel_info.offset(LEFT as isize))
                    .str_pns_info
                    .pns_used[pns_band as usize] as core::ffi::c_int != 0
                    && (**ptr_aac_dec_channel_info.offset(RIGHT as isize))
                        .str_pns_info
                        .pns_used[pns_band as usize] as core::ffi::c_int != 0
                {
                    let ref mut fresh0 = (*(**ptr_aac_dec_channel_info
                        .offset(LEFT as isize))
                        .pstr_stereo_info)
                        .ms_used[num_win_group as usize][sfb as usize];
                    *fresh0 = (*fresh0 as core::ffi::c_int ^ 1 as core::ffi::c_int)
                        as UWORD8;
                }
            }
            sfb += 1;
        }
        num_win_group += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_pulse_data_apply(
    mut ptr_pulse_info: *mut ia_pulse_info_struct,
    mut pulse_scratch: *mut WORD8,
    mut ptr_swb_offset: *const WORD16,
    mut object_type: WORD,
) -> VOID {
    let mut i: WORD = 0;
    let mut k: WORD32 = 0;
    memset(
        pulse_scratch as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<WORD32>() as size_t).wrapping_mul(256 as size_t),
    );
    if object_type != AOT_ER_AAC_ELD as core::ffi::c_int {
        if (*ptr_pulse_info).pulse_data_present != 0 {
            k = *ptr_swb_offset.offset((*ptr_pulse_info).pulse_start_band as isize)
                as WORD32;
            i = 0 as core::ffi::c_int as WORD;
            while i <= (*ptr_pulse_info).number_pulse as core::ffi::c_int {
                k = (k as core::ffi::c_int
                    + (*ptr_pulse_info).pulse_offset[i as usize] as core::ffi::c_int)
                    as WORD32;
                *pulse_scratch.offset(k as isize) = (*ptr_pulse_info)
                    .pulse_amp[i as usize];
                i += 1;
            }
        }
    } else {
        (*ptr_pulse_info).pulse_data_present = 0 as core::ffi::c_int as FLAG;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_read_spectral_data(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
    mut ptr_aac_tables: *mut ia_aac_dec_tables_struct,
    mut total_channels: WORD32,
    mut frame_size: WORD32,
    mut object_type: WORD32,
    mut aac_spect_data_resil_flag: WORD32,
    mut aac_sf_data_resil_flag: WORD32,
) -> IA_ERRORCODE {
    let mut sfb: WORD = 0;
    let mut max_sfb: WORD = 0;
    let mut num_win_grp: WORD = 0;
    let mut group_len: WORD = 0;
    let mut grp_offset: WORD = 0;
    let mut index: WORD = 0;
    let mut ptr_code_book: *mut WORD8 = 0 as *mut WORD8;
    let mut ptr_scale_factor: *mut WORD16 = 0 as *mut WORD16;
    let mut ptr_spec_coef: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr_ics_info: *mut ia_ics_info_struct = &mut (*ptr_aac_dec_channel_info)
        .str_ics_info;
    let mut swb_offset: *mut WORD16 = 0 as *mut WORD16;
    let mut maximum_bins_short: WORD32 = (*ptr_ics_info).frame_length as WORD32
        >> 3 as core::ffi::c_int;
    let mut ptr_spec_coef_out: *mut WORD32 = 0 as *mut WORD32;
    ptr_code_book = (*ptr_aac_dec_channel_info).ptr_code_book;
    ptr_scale_factor = (*ptr_aac_dec_channel_info).ptr_scale_factor;
    ptr_spec_coef = (*ptr_aac_dec_channel_info).ptr_spec_coeff;
    max_sfb = (*ptr_ics_info).max_sfb as WORD;
    swb_offset = (*ptr_aac_tables)
        .str_aac_sfb_info[(*ptr_ics_info).window_sequence as usize]
        .sfb_index;
    if aac_spect_data_resil_flag == 0 {
        if (*ptr_aac_dec_channel_info).str_ics_info.window_sequence as core::ffi::c_int
            != EIGHT_SHORT_SEQUENCE
        {
            let mut ptr_scratch: *mut WORD8 = 0 as *mut WORD8;
            if object_type == AOT_ER_AAC_ELD as core::ffi::c_int {
                ptr_scratch = (*ptr_aac_dec_channel_info).pulse_scratch as *mut WORD8;
            } else {
                ptr_scratch = (*ptr_aac_dec_channel_info).scratch_buf_ptr as *mut WORD8;
            }
            memset(
                ptr_spec_coef as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                (::core::mem::size_of::<WORD32>() as size_t).wrapping_mul(1024 as size_t),
            );
            ixheaacd_pulse_data_apply(
                &mut (*ptr_aac_dec_channel_info).str_pulse_info,
                ptr_scratch,
                swb_offset,
                object_type as WORD,
            );
            ptr_spec_coef_out = &mut *ptr_spec_coef
                .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
            sfb = 0 as core::ffi::c_int as WORD;
            while sfb < max_sfb {
                let mut ret_val: WORD = 0;
                let mut sfb_width: WORD32 = 0;
                let mut sect_cb: WORD32 = *ptr_code_book.offset(sfb as isize) as WORD32;
                let mut start: WORD = sfb;
                if object_type == AOT_ER_AAC_ELD as core::ffi::c_int
                    || object_type == AOT_ER_AAC_LD as core::ffi::c_int
                    || object_type == AOT_ER_AAC_LC as core::ffi::c_int
                {
                    if sect_cb >= 16 as core::ffi::c_int
                        && sect_cb <= 31 as core::ffi::c_int
                    {
                        sect_cb = 11 as core::ffi::c_int as WORD32;
                        *ptr_code_book.offset(sfb as isize) = sect_cb as WORD8;
                    }
                }
                while sfb < max_sfb
                    && *ptr_code_book.offset(sfb as isize) as core::ffi::c_int == sect_cb
                {
                    sfb += 1;
                }
                sfb_width = (*swb_offset.offset(sfb as isize) as core::ffi::c_int
                    - *swb_offset.offset(start as isize) as core::ffi::c_int) as WORD32;
                if sect_cb > ZERO_HCB && sect_cb < NOISE_HCB {
                    ret_val = ixheaacd_huffman_dec_word2(
                        it_bit_buff,
                        sect_cb,
                        sfb_width,
                        ptr_aac_tables,
                        ptr_spec_coef_out,
                        ptr_scratch,
                    );
                    if ret_val != 0 as core::ffi::c_int {
                        return IA_XHEAAC_DEC_EXE_NONFATAL_EXCEEDS_MAX_HUFFDEC_VAL
                            as WORD16 as IA_ERRORCODE;
                    }
                } else if (*ptr_aac_dec_channel_info).str_pulse_info.pulse_data_present
                    != 0
                {
                    ixheaacd_inverse_quantize(
                        ptr_spec_coef_out,
                        sfb_width as WORD,
                        ((*(*ptr_aac_tables).pstr_block_tables).ixheaacd_pow_table_Q13)
                            .as_mut_ptr(),
                        ptr_scratch,
                    );
                }
                ptr_scratch = ptr_scratch.offset(sfb_width as isize);
                ptr_spec_coef_out = ptr_spec_coef_out.offset(sfb_width as isize);
            }
            if object_type != AOT_ER_AAC_ELD as core::ffi::c_int
                && object_type != AOT_ER_AAC_LD as core::ffi::c_int
            {
                index = (1024 as core::ffi::c_int
                    - *swb_offset.offset(max_sfb as isize) as core::ffi::c_int) as WORD;
            } else {
                index = (frame_size as core::ffi::c_int
                    - *swb_offset.offset(max_sfb as isize) as core::ffi::c_int) as WORD;
            }
            if index < 0 as core::ffi::c_int {
                return -(1 as IA_ERRORCODE);
            }
        } else {
            memset(
                ptr_spec_coef as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                (::core::mem::size_of::<WORD32>() as size_t).wrapping_mul(1024 as size_t),
            );
            grp_offset = 0 as core::ffi::c_int as WORD;
            num_win_grp = 0 as core::ffi::c_int as WORD;
            while num_win_grp < (*ptr_ics_info).num_window_groups as core::ffi::c_int {
                let mut grp_len: WORD = (*ptr_ics_info)
                    .window_group_length[num_win_grp as usize] as WORD;
                if maximum_bins_short == 120 as core::ffi::c_int {
                    ptr_spec_coef_out = &mut *ptr_spec_coef
                        .offset((grp_offset as WORD32 * maximum_bins_short) as isize)
                        as *mut WORD32;
                } else {
                    ptr_spec_coef_out = &mut *ptr_spec_coef
                        .offset(
                            (grp_offset as core::ffi::c_int * MAX_BINS_SHORT) as isize,
                        ) as *mut WORD32;
                }
                let mut bands: WORD = num_win_grp * MAX_SCALE_FACTOR_BANDS_SHORT;
                sfb = 0 as core::ffi::c_int as WORD;
                while sfb < max_sfb {
                    let mut sect_cb_0: WORD = *ptr_code_book.offset(bands as isize)
                        as WORD;
                    let mut start_0: WORD = sfb;
                    let mut ret_val_0: WORD = 0;
                    if object_type == AOT_ER_AAC_ELD as core::ffi::c_int
                        || object_type == AOT_ER_AAC_LD as core::ffi::c_int
                        || object_type == AOT_ER_AAC_LC as core::ffi::c_int
                    {
                        if sect_cb_0 >= 16 as core::ffi::c_int
                            && sect_cb_0 <= 31 as core::ffi::c_int
                        {
                            sect_cb_0 = 11 as core::ffi::c_int as WORD;
                            *ptr_code_book.offset(bands as isize) = sect_cb_0 as WORD8;
                        }
                    }
                    while sfb < max_sfb
                        && *ptr_code_book.offset(bands as isize) as core::ffi::c_int
                            == sect_cb_0
                    {
                        sfb += 1;
                        bands += 1;
                    }
                    if sect_cb_0 > ZERO_HCB && sect_cb_0 < NOISE_HCB {
                        ret_val_0 = ixheaacd_decode_huffman(
                            it_bit_buff,
                            sect_cb_0 as WORD32,
                            ptr_spec_coef_out,
                            swb_offset,
                            start_0,
                            sfb,
                            grp_len,
                            ptr_aac_tables,
                            maximum_bins_short,
                        );
                        if ret_val_0 != 0 as core::ffi::c_int {
                            return IA_XHEAAC_DEC_EXE_NONFATAL_EXCEEDS_MAX_HUFFDEC_VAL
                                as WORD16 as IA_ERRORCODE;
                        }
                    }
                }
                grp_offset = grp_offset + grp_len;
                num_win_grp += 1;
            }
        }
        let mut ptr_scale_table: *mut WORD32 = 0 as *mut WORD32;
        if maximum_bins_short != 120 as core::ffi::c_int {
            ptr_scale_table = ((*(*ptr_aac_tables).pstr_block_tables).scale_table)
                .as_mut_ptr();
        } else {
            ptr_scale_table = ((*(*ptr_aac_tables).pstr_block_tables).scale_table_960)
                .as_mut_ptr();
        }
        let mut ptr_sfb_width: *mut WORD8 = (*ptr_aac_tables)
            .str_aac_sfb_info[(*ptr_ics_info).window_sequence as usize]
            .sfb_width;
        num_win_grp = 0 as core::ffi::c_int as WORD;
        while num_win_grp < (*ptr_ics_info).num_window_groups as core::ffi::c_int {
            group_len = 0 as core::ffi::c_int as WORD;
            while group_len
                < (*ptr_ics_info).window_group_length[num_win_grp as usize]
                    as core::ffi::c_int
            {
                (Some(ixheaacd_scale_factor_process.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    &mut *ptr_spec_coef.offset(0 as core::ffi::c_int as isize),
                    &mut *ptr_scale_factor.offset(0 as core::ffi::c_int as isize),
                    max_sfb,
                    ptr_sfb_width,
                    ptr_scale_table,
                    total_channels,
                    object_type,
                    aac_sf_data_resil_flag,
                );
                if maximum_bins_short == 120 as core::ffi::c_int {
                    ptr_spec_coef = ptr_spec_coef.offset(maximum_bins_short as isize);
                } else {
                    ptr_spec_coef = ptr_spec_coef.offset(MAX_BINS_SHORT as isize);
                }
                group_len += 1;
            }
            ptr_scale_factor = ptr_scale_factor
                .offset(MAX_SCALE_FACTOR_BANDS_SHORT as isize);
            num_win_grp += 1;
        }
    } else {
        let mut pstr_hcr_info: *mut ia_hcr_info_struct = &mut (*ptr_aac_dec_channel_info)
            .str_hcr_info;
        let mut error: WORD32 = 0 as WORD32;
        memset(
            ptr_spec_coef as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<WORD32>() as size_t).wrapping_mul(1024 as size_t),
        );
        if (*ptr_aac_dec_channel_info).reorder_spect_data_len as core::ffi::c_int
            != 0 as core::ffi::c_int
        {
            error = ixheaacd_huff_code_reorder_init(
                pstr_hcr_info,
                ptr_aac_dec_channel_info,
                ptr_aac_tables,
                it_bit_buff,
            ) as WORD32;
            if error != 0 as core::ffi::c_int {
                return IA_XHEAAC_DEC_EXE_NONFATAL_DECODE_FRAME_ERROR;
            }
            error = ixheaacd_hcr_decoder(
                pstr_hcr_info,
                ptr_aac_dec_channel_info,
                ptr_aac_tables,
                it_bit_buff,
            ) as WORD32;
            if error != 0 as core::ffi::c_int {
                ixheaacd_huff_mute_erroneous_lines(pstr_hcr_info);
            }
            if (*it_bit_buff).cnt_bits
                < (*ptr_aac_dec_channel_info).reorder_spect_data_len as core::ffi::c_int
            {
                longjmp(
                    (*(*it_bit_buff).xaac_jmp_buf).as_mut_ptr(),
                    IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES,
                );
            }
            (*it_bit_buff).cnt_bits
                += -((*ptr_aac_dec_channel_info).reorder_spect_data_len
                    as core::ffi::c_int);
            (*it_bit_buff).ptr_read_next = ((*it_bit_buff).ptr_bit_buf_base)
                .offset(
                    ((*it_bit_buff).size - (*it_bit_buff).cnt_bits
                        >> 3 as core::ffi::c_int) as isize,
                );
            (*it_bit_buff).bit_pos = ((*it_bit_buff).size as core::ffi::c_int
                - (*it_bit_buff).cnt_bits as core::ffi::c_int & 7 as core::ffi::c_int)
                as WORD32;
        } else {
            (*it_bit_buff).ptr_read_next = ((*it_bit_buff).ptr_bit_buf_base)
                .offset(
                    ((*it_bit_buff).size - (*it_bit_buff).cnt_bits
                        >> 3 as core::ffi::c_int) as isize,
                );
            (*it_bit_buff).bit_pos = ((*it_bit_buff).size as core::ffi::c_int
                - (*it_bit_buff).cnt_bits as core::ffi::c_int & 7 as core::ffi::c_int)
                as WORD32;
        }
    }
    return AAC_DEC_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_read_tns_data(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
) -> WORD16 {
    let mut win_size: WORD = 0;
    let mut window_per_frame: WORD = 0;
    let mut n_filt_bits: WORD = 0;
    let mut start_band_bits: WORD = 0;
    let mut order_bits: WORD = 0;
    let mut bottom: WORD32 = 0;
    let mut ptr_ics_info: *mut ia_ics_info_struct = &mut (*ptr_aac_dec_channel_info)
        .str_ics_info;
    let mut ptr_tns_info: *mut ia_tns_info_aac_struct = &mut (*ptr_aac_dec_channel_info)
        .str_tns_info;
    if (*ptr_ics_info).window_sequence as core::ffi::c_int != EIGHT_SHORT_SEQUENCE {
        n_filt_bits = 2 as core::ffi::c_int as WORD;
        start_band_bits = 6 as core::ffi::c_int as WORD;
        order_bits = 5 as core::ffi::c_int as WORD;
        window_per_frame = 1 as core::ffi::c_int as WORD;
    } else {
        n_filt_bits = 1 as core::ffi::c_int as WORD;
        start_band_bits = 4 as core::ffi::c_int as WORD;
        order_bits = 3 as core::ffi::c_int as WORD;
        window_per_frame = 8 as core::ffi::c_int as WORD;
    }
    bottom = (*ptr_ics_info).num_swb_window as WORD32;
    win_size = 0 as core::ffi::c_int as WORD;
    while win_size < window_per_frame {
        let mut n_filt: WORD = 0;
        let mut start_band: WORD = 0;
        let mut coef_res: WORD = 0;
        n_filt = ixheaacd_read_bits_buf(it_bit_buff, n_filt_bits) as WORD16 as WORD;
        (*ptr_tns_info).n_filt[win_size as usize] = n_filt as WORD8;
        if n_filt != 0 {
            let mut filt: WORD = 0;
            let mut top: WORD = 0;
            coef_res = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD) as WORD;
            top = bottom as WORD;
            filt = 0 as core::ffi::c_int as WORD;
            while filt < n_filt {
                let mut order: WORD = 0;
                let mut filter: *mut ia_filter_info_struct = &mut *(*((*ptr_tns_info)
                    .str_filter)
                    .as_mut_ptr()
                    .offset(win_size as isize))
                    .as_mut_ptr()
                    .offset(filt as isize) as *mut ia_filter_info_struct;
                start_band = ixheaacd_read_bits_buf(it_bit_buff, start_band_bits)
                    as WORD;
                if top < start_band {
                    top = start_band;
                }
                (*filter).start_band = (top - start_band) as WORD16;
                (*filter).stop_band = top as WORD16;
                top = (*filter).start_band as WORD;
                if ((*filter).start_band as core::ffi::c_int) < 0 as core::ffi::c_int {
                    (*filter).order = -(1 as core::ffi::c_int) as WORD8;
                    return IA_XHEAAC_DEC_EXE_FATAL_TNS_RANGE_ERROR as WORD32 as WORD16;
                }
                order = ixheaacd_read_bits_buf(it_bit_buff, order_bits) as WORD;
                (*filter).order = order as WORD8;
                if order as core::ffi::c_int - MAX_ORDER_LONG > 0 as core::ffi::c_int {
                    return IA_XHEAAC_DEC_EXE_NONFATAL_TNS_ORDER_ERROR as WORD16;
                }
                if order != 0 {
                    let mut i: WORD = 0;
                    let mut coef: WORD32 = 0;
                    let mut coef_compress: WORD32 = 0;
                    let mut resolution: WORD = 0;
                    let mut shift: WORD = 0;
                    (*filter).direction = (if ixheaacd_read_bits_buf(
                        it_bit_buff,
                        1 as WORD,
                    ) != 0
                    {
                        -(1 as core::ffi::c_int)
                    } else {
                        1 as core::ffi::c_int
                    }) as WORD8;
                    coef_compress = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
                    (*filter).resolution = coef_res as WORD8;
                    resolution = (coef_res as WORD32 + 3 as WORD32 - coef_compress)
                        as WORD;
                    shift = 32 as WORD - resolution;
                    i = 0 as core::ffi::c_int as WORD;
                    while i < order {
                        coef = ixheaacd_read_bits_buf(it_bit_buff, resolution);
                        coef = coef << shift;
                        (*filter).coef[i as usize] = (coef >> shift) as WORD8;
                        i += 1;
                    }
                }
                filt += 1;
            }
        }
        win_size += 1;
    }
    return AAC_DEC_OK as WORD16;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_inv_quant(
    mut px_quant: *mut WORD32,
    mut ixheaacd_pow_table_Q13: *mut WORD32,
) -> WORD32 {
    let mut q1: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut q_abs: WORD32 = 0;
    let mut interp: WORD16 = 0;
    let mut shift: WORD32 = 0;
    q_abs = *px_quant;
    if q_abs > 8191 as core::ffi::c_int + 32 as core::ffi::c_int {
        return IA_XHEAAC_DEC_EXE_NONFATAL_EXCEEDS_MAX_HUFFDEC_VAL;
    }
    if q_abs < 1024 as core::ffi::c_int {
        shift = 3 as core::ffi::c_int as WORD32;
    } else {
        shift = 6 as core::ffi::c_int as WORD32;
    }
    q1 = q_abs >> shift;
    interp = (q_abs - (q1 << shift)) as WORD16;
    temp = *ixheaacd_pow_table_Q13
        .offset((q1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
        - *ixheaacd_pow_table_Q13.offset(q1 as isize);
    temp = temp * interp as WORD32;
    temp = temp + (*ixheaacd_pow_table_Q13.offset(q1 as isize) << shift);
    if shift == 3 as core::ffi::c_int {
        temp = temp << 1 as core::ffi::c_int;
    } else {
        temp = temp << 2 as core::ffi::c_int;
    }
    *px_quant = temp;
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_scale_value_in_place(
    mut value: *mut WORD32,
    mut scalefactor: WORD32,
) {
    let mut newscale: WORD32 = 0;
    newscale = scalefactor;
    if newscale >= 0 as core::ffi::c_int {
        *value <<= newscale;
    } else {
        *value >>= -newscale;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_cblock_inv_quant_spect_data(
    mut ptr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
    mut ptr_aac_tables: *mut ia_aac_dec_tables_struct,
) -> WORD32 {
    let mut window: core::ffi::c_int = 0;
    let mut group: core::ffi::c_int = 0;
    let mut grp_win: core::ffi::c_int = 0;
    let mut band: core::ffi::c_int = 0;
    let mut err: IA_ERRORCODE = IA_NO_ERROR;
    let mut sf_bands_transmitted: core::ffi::c_int = (*ptr_aac_dec_channel_info)
        .str_ics_info
        .max_sfb as core::ffi::c_int;
    let mut ptr_code_book: *mut WORD8 = (*ptr_aac_dec_channel_info).ptr_code_book;
    let mut band_offsets: *const WORD16 = ixheaacd_getscalefactorbandoffsets(
        &mut (*ptr_aac_dec_channel_info).str_ics_info,
        ptr_aac_tables,
    );
    let mut ptr_pow_table_Q13: *mut WORD32 = ((*(*ptr_aac_tables).pstr_block_tables)
        .ixheaacd_pow_table_Q13)
        .as_mut_ptr();
    window = 0 as core::ffi::c_int;
    group = 0 as core::ffi::c_int;
    while group
        < (*ptr_aac_dec_channel_info).str_ics_info.num_window_groups as core::ffi::c_int
    {
        grp_win = 0 as core::ffi::c_int;
        while grp_win
            < (*ptr_aac_dec_channel_info)
                .str_ics_info
                .window_group_length[group as usize] as core::ffi::c_int
        {
            band = 0 as core::ffi::c_int;
            while band < sf_bands_transmitted {
                let mut ptr_spec_coef: *mut WORD32 = ((*ptr_aac_dec_channel_info)
                    .ptr_spec_coeff)
                    .offset(
                        (window as WORD32 * (*ptr_aac_dec_channel_info).granule_len)
                            as isize,
                    )
                    .offset(
                        *band_offsets.offset(band as isize) as core::ffi::c_int as isize,
                    );
                let mut num_lines: core::ffi::c_int = *band_offsets
                    .offset((band + 1 as core::ffi::c_int) as isize) as core::ffi::c_int
                    - *band_offsets.offset(band as isize) as core::ffi::c_int;
                let mut bnds: core::ffi::c_int = group * 16 as core::ffi::c_int + band;
                let mut i: core::ffi::c_int = 0;
                if !(*ptr_code_book.offset(bnds as isize) as core::ffi::c_int == ZERO_HCB
                    || *ptr_code_book.offset(bnds as isize) as core::ffi::c_int
                        == INTENSITY_HCB
                    || *ptr_code_book.offset(bnds as isize) as core::ffi::c_int
                        == INTENSITY_HCB2)
                {
                    if !(*ptr_code_book.offset(bnds as isize) as core::ffi::c_int
                        == NOISE_HCB)
                    {
                        i = 0 as core::ffi::c_int;
                        while i < num_lines {
                            let mut temp: WORD8 = 0 as WORD8;
                            let mut out1: WORD32 = *ptr_spec_coef.offset(i as isize);
                            if out1 <= 0 as core::ffi::c_int {
                                out1 = temp as WORD32 - out1;
                                if out1 > 127 as core::ffi::c_int {
                                    err = ixheaacd_inv_quant(&mut out1, ptr_pow_table_Q13)
                                        as IA_ERRORCODE;
                                    if err != 0 {
                                        return err as WORD32;
                                    }
                                } else {
                                    out1 = *ptr_pow_table_Q13.offset(out1 as isize);
                                }
                                *ptr_spec_coef.offset(i as isize) = -out1;
                            } else {
                                if out1 > 127 as core::ffi::c_int {
                                    err = ixheaacd_inv_quant(&mut out1, ptr_pow_table_Q13)
                                        as IA_ERRORCODE;
                                    if err != 0 {
                                        return err as WORD32;
                                    }
                                } else {
                                    out1 = *ptr_pow_table_Q13.offset(out1 as isize);
                                }
                                *ptr_spec_coef.offset(i as isize) = out1;
                            }
                            i += 1;
                        }
                    }
                }
                band += 1;
            }
            grp_win += 1;
            window += 1;
        }
        group += 1;
    }
    return AAC_DEC_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_init_ltp_object(mut ltp: *mut ltp_info) -> VOID {
    (*ltp).data_present = 0 as UWORD8;
    (*ltp).last_band = 0 as UWORD8;
    (*ltp).lag_update = 0 as UWORD8;
    (*ltp).coef = 0 as UWORD8;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_ltp_data(
    mut object_type: WORD32,
    mut ics: *mut ia_ics_info_struct,
    mut ltp: *mut ltp_info,
    mut it_bit_buf: *mut ia_bit_buf_struct,
    mut frame_len: WORD32,
) -> WORD32 {
    let mut sfb: UWORD8 = 0;
    let mut w: UWORD8 = 0;
    if object_type == AOT_ER_AAC_LD as core::ffi::c_int {
        (*ltp).lag_update = ixheaacd_read_bits_buf(it_bit_buf, 1 as WORD) as UWORD8;
        if (*ltp).lag_update != 0 {
            (*ltp).lag = ixheaacd_read_bits_buf(it_bit_buf, 10 as WORD) as UWORD16;
        }
    } else {
        (*ltp).lag = ixheaacd_read_bits_buf(it_bit_buf, 11 as WORD) as UWORD16;
    }
    if (*ltp).lag as core::ffi::c_int > frame_len << 1 as core::ffi::c_int {
        return -(1 as WORD32);
    }
    (*ltp).coef = ixheaacd_read_bits_buf(it_bit_buf, 3 as WORD) as UWORD8;
    if (*ics).window_sequence as core::ffi::c_int == EIGHT_SHORT_SEQUENCE {
        w = 0 as UWORD8;
        while (w as core::ffi::c_int) < 8 as core::ffi::c_int {
            (*ltp).short_used[w as usize] = ixheaacd_read_bits_buf(it_bit_buf, 1 as WORD)
                as UWORD8;
            if (*ltp).short_used[w as usize] as core::ffi::c_int & 1 as core::ffi::c_int
                != 0
            {
                (*ltp).short_lag_present[w as usize] = ixheaacd_read_bits_buf(
                    it_bit_buf,
                    1 as WORD,
                ) as UWORD8;
                if (*ltp).short_lag_present[w as usize] != 0 {
                    (*ltp).short_lag[w as usize] = ixheaacd_read_bits_buf(
                        it_bit_buf,
                        4 as WORD,
                    ) as UWORD8;
                }
            }
            w = w.wrapping_add(1);
        }
    } else {
        (*ltp).last_band = (if ((*ics).max_sfb as core::ffi::c_int) < MAX_LTP_SFB {
            (*ics).max_sfb as core::ffi::c_int
        } else {
            MAX_LTP_SFB
        }) as UWORD8;
        sfb = 0 as UWORD8;
        while (sfb as core::ffi::c_int) < (*ltp).last_band as core::ffi::c_int {
            (*ltp).long_used[sfb as usize] = ixheaacd_read_bits_buf(
                it_bit_buf,
                1 as WORD,
            ) as UWORD8;
            sfb = sfb.wrapping_add(1);
        }
    }
    if (*ics).frame_length as core::ffi::c_int == 480 as core::ffi::c_int {
        if (*ics).sampling_rate_index as core::ffi::c_int > 5 as core::ffi::c_int
            && (*ltp).last_band as core::ffi::c_int > MAX_LTP_SFB_SR_FIVE_PLUS_480
        {
            (*ltp).last_band = MAX_LTP_SFB_SR_FIVE_PLUS_480 as UWORD8;
        } else if (*ics).sampling_rate_index as core::ffi::c_int == 5 as core::ffi::c_int
            && (*ltp).last_band as core::ffi::c_int > MAX_LTP_SFB_SR_FIVE_480
        {
            (*ltp).last_band = MAX_LTP_SFB_SR_FIVE_480 as UWORD8;
        } else if ((*ics).sampling_rate_index as core::ffi::c_int)
            < 5 as core::ffi::c_int
            && (*ltp).last_band as core::ffi::c_int > MAX_LTP_SFB_SR_FIVE_LESS_480
        {
            (*ltp).last_band = MAX_LTP_SFB_SR_FIVE_LESS_480 as UWORD8;
        }
    } else if (*ics).frame_length as core::ffi::c_int == 512 as core::ffi::c_int {
        if (*ics).sampling_rate_index as core::ffi::c_int > 5 as core::ffi::c_int
            && (*ltp).last_band as core::ffi::c_int > MAX_LTP_SFB_SR_FIVE_PLUS_512
        {
            (*ltp).last_band = MAX_LTP_SFB_SR_FIVE_PLUS_512 as UWORD8;
        } else if (*ics).sampling_rate_index as core::ffi::c_int == 5 as core::ffi::c_int
            && (*ltp).last_band as core::ffi::c_int > MAX_LTP_SFB_SR_FIVE_512
        {
            (*ltp).last_band = MAX_LTP_SFB_SR_FIVE_512 as UWORD8;
        } else if ((*ics).sampling_rate_index as core::ffi::c_int)
            < 5 as core::ffi::c_int
            && (*ltp).last_band as core::ffi::c_int > MAX_LTP_SFB_SR_FIVE_LESS_512
        {
            (*ltp).last_band = MAX_LTP_SFB_SR_FIVE_LESS_512 as UWORD8;
        }
    }
    return 0 as WORD32;
}
