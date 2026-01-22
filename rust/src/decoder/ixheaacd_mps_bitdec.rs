extern "C" {
    pub type ia_mps_dec_ducker_interface;
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn ixheaacd_byte_align(
        it_bit_buff: *mut ia_bit_buf_struct,
        ptr_byte_align_bits: *mut WORD32,
    ) -> VOID;
    fn ixheaacd_read_bits_buf(
        it_bit_buff: *mut ia_bit_buf_struct,
        no_of_bits: WORD,
    ) -> WORD32;
    fn ixheaacd_res_read_ics(
        it_bit_buf: *mut ia_bit_buf_struct,
        p_aac_decoder_channel_info: *mut *mut ia_mps_dec_residual_channel_info_struct,
        num_ch: WORD32,
        aac_tables_ptr: *mut ia_mps_dec_residual_aac_tables_struct,
        tot_sf_bands_ls: *mut WORD8,
    ) -> WORD16;
    fn ixheaacd_mps_ecdatapairdec(
        it_bit_buf: *mut ia_bit_buf_struct,
        aa_out_data: *mut [WORD32; 28],
        a_history: *mut WORD32,
        data_type: WORD32,
        set_idx: WORD32,
        start_band: WORD32,
        data_bands: WORD32,
        pair_flag: WORD32,
        coarse_flag: WORD32,
        independency_flag: WORD32,
        ldmps_flag: WORD32,
        heaac_mps_present: WORD32,
        ec_flag: WORD32,
    ) -> WORD32;
    fn ixheaacd_mps_huff_decode(
        it_bit_buf: *mut ia_bit_buf_struct,
        out_data: *mut WORD32,
        num_val: WORD32,
    ) -> VOID;
    fn ixheaacd_res_ctns_apply(
        p_aac_decoder_channel_info: *mut ia_mps_dec_residual_channel_info_struct,
        max_sfb: WORD16,
        aac_tables_ptr: *mut ia_mps_dec_residual_aac_tables_struct,
    ) -> VOID;
}
pub type size_t = usize;
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
pub type IA_ERRORCODE = WORD32;
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
pub const EXT_TYPE_2: C2RustUnnamed_0 = 2;
pub const IN_CH_6: C2RustUnnamed_2 = 6;
pub const IN_CH_2: C2RustUnnamed_2 = 2;
pub const IN_CH_1: C2RustUnnamed_2 = 1;
pub const EXT_TYPE_1: C2RustUnnamed_0 = 1;
pub const EXT_TYPE_0: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub type C2RustUnnamed_1 = core::ffi::c_uint;
pub const QUANT_MODE_2: C2RustUnnamed_1 = 2;
pub const QUANT_MODE_1: C2RustUnnamed_1 = 1;
pub const QUANT_MODE_0: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = core::ffi::c_uint;
pub type C2RustUnnamed_3 = core::ffi::c_uint;
pub const TREE_7572: C2RustUnnamed_3 = 6;
pub const TREE_7571: C2RustUnnamed_3 = 5;
pub const TREE_7272: C2RustUnnamed_3 = 4;
pub const TREE_7271: C2RustUnnamed_3 = 3;
pub const TREE_525: C2RustUnnamed_3 = 2;
pub const TREE_5152: C2RustUnnamed_3 = 1;
pub const TREE_5151: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_4 = core::ffi::c_uint;
pub const PARAMETER_BANDS_40: C2RustUnnamed_4 = 40;
pub const PARAMETER_BANDS_28: C2RustUnnamed_4 = 28;
pub const PARAMETER_BANDS_20: C2RustUnnamed_4 = 20;
pub const PARAMETER_BANDS_14: C2RustUnnamed_4 = 14;
pub const PARAMETER_BANDS_10: C2RustUnnamed_4 = 10;
pub const PARAMETER_BANDS_7: C2RustUnnamed_4 = 7;
pub const PARAMETER_BANDS_5: C2RustUnnamed_4 = 5;
pub const PARAMETER_BANDS_4: C2RustUnnamed_4 = 4;
pub type C2RustUnnamed_5 = core::ffi::c_uint;
pub const SMOOTH_MODE_3: C2RustUnnamed_5 = 3;
pub const SMOOTH_MODE_2: C2RustUnnamed_5 = 2;
pub const SMOOTH_MODE_1: C2RustUnnamed_5 = 1;
pub const SMOOTH_MODE_0: C2RustUnnamed_5 = 0;
pub type C2RustUnnamed_6 = core::ffi::c_uint;
pub const TTT_MODE_5: C2RustUnnamed_6 = 5;
pub const TTT_MODE_4: C2RustUnnamed_6 = 4;
pub const TTT_MODE_3: C2RustUnnamed_6 = 3;
pub const TTT_MODE_2: C2RustUnnamed_6 = 2;
pub const TTT_MODE_1: C2RustUnnamed_6 = 1;
pub const TTT_MODE_0: C2RustUnnamed_6 = 0;
pub const IA_XHEAAC_MPS_DEC_EXE_NONFATAL_INVALID_SMOOTH_MODE: core::ffi::c_int = 0x1810
    as core::ffi::c_int;
pub const IA_XHEAAC_MPS_DEC_EXE_NONFATAL_INVALID_EXTENSION_TYPE: core::ffi::c_int = 0x1812
    as core::ffi::c_int;
pub const IA_XHEAAC_MPS_DEC_EXE_NONFATAL_INVALID_QMF_UPDATE: core::ffi::c_int = 0x1813
    as core::ffi::c_int;
pub const IA_XHEAAC_MPS_DEC_EXE_FATAL_UNSUPPRORTED_TREE_CONFIG: core::ffi::c_uint = 0xffff9805
    as core::ffi::c_uint;
pub const IA_XHEAAC_MPS_DEC_EXE_FATAL_INVALID_DEQUANT_PARAM: core::ffi::c_uint = 0xffff9807
    as core::ffi::c_uint;
pub const IA_XHEAAC_MPS_DEC_EXE_FATAL_INVALID_QUANT_MODE: core::ffi::c_uint = 0xffff9808
    as core::ffi::c_uint;
pub const IA_XHEAAC_MPS_DEC_EXE_FATAL_INVALID_INPUT_CHANNEL: core::ffi::c_uint = 0xffff9809
    as core::ffi::c_uint;
pub const IA_XHEAAC_MPS_DEC_EXE_FATAL_NONZERO_BIT: core::ffi::c_uint = 0xffff980a
    as core::ffi::c_uint;
pub const IA_XHEAAC_MPS_DEC_EXE_FATAL_INVALID_WINDOW_SEQUENCE: core::ffi::c_uint = 0xffff980b
    as core::ffi::c_uint;
pub const IA_XHEAAC_MPS_DEC_EXE_FATAL_INVALID_TEMPORAL_SHAPING_CONFIG: core::ffi::c_uint = 0xffff980c
    as core::ffi::c_uint;
pub const IA_XHEAAC_MPS_DEC_EXE_FATAL_INVALID_HRTF_SET: core::ffi::c_uint = 0xffff980d
    as core::ffi::c_uint;
pub const IA_XHEAAC_MPS_DEC_EXE_FATAL_INVALID_TTT_MODE: core::ffi::c_uint = 0xffff980e
    as core::ffi::c_uint;
pub const IA_XHEAAC_MPS_DEC_EXE_FATAL_INVALID_PARAMETER_SETS: core::ffi::c_uint = 0xffff9810
    as core::ffi::c_uint;
pub const IA_XHEAAC_MPS_DEC_EXE_FATAL_INVALID_MPS_PARAM: core::ffi::c_uint = 0xffff9811
    as core::ffi::c_uint;
pub const IA_XHEAAC_MPS_DEC_EXE_FATAL_INVALID_PARAMETER_BANDS: core::ffi::c_uint = 0xffff9812
    as core::ffi::c_uint;
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
unsafe extern "C" fn ixheaac_add32(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut sum: WORD32 = 0;
    sum = a + b;
    return sum;
}
#[inline]
unsafe extern "C" fn ixheaac_mul32_sh(
    mut a: WORD32,
    mut b: WORD32,
    mut shift: WORD8,
) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> shift as core::ffi::c_int) as WORD32;
    return result;
}
pub const EIGHT_SHORT_SEQUENCE: core::ffi::c_int = 2 as core::ffi::c_int;
pub const IA_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_FATAL_ERROR: core::ffi::c_uint = 0x80000000 as core::ffi::c_uint;
pub const MAX_PARAMETER_BANDS: core::ffi::c_int = 28 as core::ffi::c_int;
pub const MAX_NUM_OTT: core::ffi::c_int = 5 as core::ffi::c_int;
pub const MAX_NUM_EXT_TYPES: core::ffi::c_int = 8 as core::ffi::c_int;
pub const QMF_BANDS_TO_HYBRID: core::ffi::c_int = 3 as core::ffi::c_int;
pub const MAX_ARBITRARY_TREE_LEVELS: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_ARBITRARY_TREE_INDEX: core::ffi::c_int = ((1 as core::ffi::c_int)
    << MAX_ARBITRARY_TREE_LEVELS + 1 as core::ffi::c_int) - 1 as core::ffi::c_int;
pub const MAX_RES_SAMP_FREQ_IDX: core::ffi::c_int = 11 as core::ffi::c_int;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const MAX_INPUT_CHANNELS_MPS: core::ffi::c_int = 6 as core::ffi::c_int;
pub const MAX_PARAMETER_SETS: core::ffi::c_int = 8 as core::ffi::c_int;
pub const MAX_OUTPUT_CHANNELS_AT_MPS: core::ffi::c_int = 8 as core::ffi::c_int;
pub const FIVE_POINT_ONE_CHANNEL_MASK: core::ffi::c_int = 0x3f as core::ffi::c_int;
pub const SEVEN_POINT_ONE_CHANNEL_MASK1: core::ffi::c_int = 0xff as core::ffi::c_int;
pub const SEVEN_POINT_ONE_CHANNEL_MASK2: core::ffi::c_int = 0x33f as core::ffi::c_int;
pub const ONE_IN_Q24: core::ffi::c_int = 16777216 as core::ffi::c_int;
pub const ONE_IN_Q27: core::ffi::c_int = 134217728 as core::ffi::c_int;
pub const ONE_FORTYNINE_Q15: core::ffi::c_int = 4882432 as core::ffi::c_int;
pub const MINUS_POINT_NINE_EIGHT_Q15: core::ffi::c_int = -(32113 as core::ffi::c_int);
pub const MAX_PSXPB: core::ffi::c_int = 224 as core::ffi::c_int;
pub const RFX2XMDCTCOEF: core::ffi::c_int = 8192 as core::ffi::c_int;
pub const ONE_BIT_MASK: core::ffi::c_int = 0x1 as core::ffi::c_int;
pub const TWO_BIT_MASK: core::ffi::c_int = 0x3 as core::ffi::c_int;
pub const THREE_BIT_MASK: core::ffi::c_int = 0x7 as core::ffi::c_int;
pub const FOUR_BIT_MASK: core::ffi::c_int = 0xf as core::ffi::c_int;
pub const FIVE_BIT_MASK: core::ffi::c_int = 0x1f as core::ffi::c_int;
pub const SEVEN_BIT_MASK: core::ffi::c_int = 0x7f as core::ffi::c_int;
pub const CLD: core::ffi::c_int = 0 as core::ffi::c_int;
pub const ICC: core::ffi::c_int = 1 as core::ffi::c_int;
pub const CPC: core::ffi::c_int = 3 as core::ffi::c_int;
pub const AAC_FRAME_LENGTH: core::ffi::c_int = 1024 as core::ffi::c_int;
pub const UPD_QMF_15: core::ffi::c_int = 15 as core::ffi::c_int;
pub const UPD_QMF_16: core::ffi::c_int = 16 as core::ffi::c_int;
pub const UPD_QMF_18: core::ffi::c_int = 18 as core::ffi::c_int;
pub const UPD_QMF_24: core::ffi::c_int = 24 as core::ffi::c_int;
pub const UPD_QMF_30: core::ffi::c_int = 30 as core::ffi::c_int;
pub const UPD_QMF_32: core::ffi::c_int = 32 as core::ffi::c_int;
pub const MAX_QMF_BUF_LEN: core::ffi::c_int = 78 as core::ffi::c_int;
static mut ixheaacd_freq_res_table: [WORD32; 8] = [
    0 as core::ffi::c_int,
    28 as core::ffi::c_int,
    20 as core::ffi::c_int,
    14 as core::ffi::c_int,
    10 as core::ffi::c_int,
    7 as core::ffi::c_int,
    5 as core::ffi::c_int,
    4 as core::ffi::c_int,
];
unsafe extern "C" fn ixheaacd_bound_check(
    mut var: WORD32,
    mut lower_bound: WORD32,
    mut upper_bound: WORD32,
) -> WORD32 {
    var = if var < upper_bound { var } else { upper_bound };
    var = if var > lower_bound { var } else { lower_bound };
    return var;
}
unsafe extern "C" fn ixheaacd_mps_check_index_bounds(
    mut output_idx_data: *mut [[WORD32; 28]; 8],
    mut num_parameter_sets: WORD32,
    mut start_band: WORD32,
    mut stop_band: WORD32,
    mut param_type: WORD32,
    mut xtt_idx: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut band: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_parameter_sets {
        band = start_band;
        while band < stop_band {
            if param_type == CLD {
                (*output_idx_data.offset(xtt_idx as isize))[i as usize][band as usize] = ixheaacd_bound_check(
                    (*output_idx_data
                        .offset(xtt_idx as isize))[i as usize][band as usize],
                    -(15 as WORD32),
                    15 as WORD32,
                );
            } else if param_type == ICC {
                (*output_idx_data.offset(xtt_idx as isize))[i as usize][band as usize] = ixheaacd_bound_check(
                    (*output_idx_data
                        .offset(xtt_idx as isize))[i as usize][band as usize],
                    0 as WORD32,
                    7 as WORD32,
                );
            }
            band += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_parse_extension_config(
    mut config: *mut ia_mps_spatial_bs_config_struct,
    mut num_ott_boxes: WORD32,
    mut num_ttt_boxes: WORD32,
    mut num_out_chan: WORD32,
    mut bits_available: WORD32,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut ixheaacd_mps_dec_bitdec_tables: *mut ia_mps_dec_bitdec_tables_struct,
) -> IA_ERRORCODE {
    let mut i: WORD32 = 0;
    let mut ch: WORD32 = 0;
    let mut idx: WORD32 = 0;
    let mut tmp: WORD32 = 0;
    let mut tmp_open: WORD32 = 0;
    let mut sac_ext_len: WORD32 = 0;
    let mut bits_read: WORD32 = 0;
    let mut n_fill_bits: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut ba: WORD32 = bits_available;
    (*config).sac_ext_cnt = 0 as core::ffi::c_int as WORD32;
    while ba >= 8 as core::ffi::c_int {
        ba -= 8 as core::ffi::c_int;
        temp = ixheaacd_read_bits_buf(it_bit_buff, 8 as WORD);
        (*config).bs_sac_ext_type[(*config).sac_ext_cnt as usize] = (temp
            as core::ffi::c_int >> 4 as core::ffi::c_int & FOUR_BIT_MASK) as WORD32;
        sac_ext_len = (temp as core::ffi::c_int & FOUR_BIT_MASK) as WORD32;
        if sac_ext_len == 15 as core::ffi::c_int {
            sac_ext_len += ixheaacd_read_bits_buf(it_bit_buff, 8 as WORD);
            ba -= 8 as core::ffi::c_int;
            if sac_ext_len == 15 as core::ffi::c_int + 255 as core::ffi::c_int {
                sac_ext_len += ixheaacd_read_bits_buf(it_bit_buff, 16 as WORD);
                ba -= 16 as core::ffi::c_int;
            }
        }
        tmp = (((((*it_bit_buff).ptr_read_next)
            .offset_from((*it_bit_buff).ptr_bit_buf_base) as core::ffi::c_long
            + 1 as core::ffi::c_long) << 3 as core::ffi::c_int)
            - ((*it_bit_buff).bit_pos as core::ffi::c_int + 1 as core::ffi::c_int)
                as core::ffi::c_long) as WORD32;
        match (*config).bs_sac_ext_type[(*config).sac_ext_cnt as usize] {
            0 => {
                (*config).bs_residual_coding = 1 as core::ffi::c_int as WORD32;
                temp = ixheaacd_read_bits_buf(it_bit_buff, 6 as WORD);
                (*config).bs_residual_sampling_freq_index = (temp as core::ffi::c_int
                    >> 2 as core::ffi::c_int & FOUR_BIT_MASK) as WORD32;
                if (*config).bs_residual_sampling_freq_index > MAX_RES_SAMP_FREQ_IDX {
                    return IA_FATAL_ERROR as IA_ERRORCODE;
                }
                (*config).bs_residual_frames_per_spatial_frame = (temp
                    as core::ffi::c_int & TWO_BIT_MASK) as WORD32;
                i = 0 as core::ffi::c_int as WORD32;
                while i < num_ott_boxes + num_ttt_boxes {
                    (*config).bs_residual_present[i as usize] = ixheaacd_read_bits_buf(
                        it_bit_buff,
                        1 as WORD,
                    );
                    if (*config).bs_residual_present[i as usize] != 0 {
                        (*config).bs_residual_bands[i as usize] = ixheaacd_read_bits_buf(
                            it_bit_buff,
                            5 as WORD,
                        );
                        if (*config).bs_residual_bands[i as usize] > MAX_PARAMETER_BANDS
                        {
                            return IA_FATAL_ERROR as IA_ERRORCODE;
                        }
                    }
                    i += 1;
                }
            }
            1 => {
                (*config).bs_arbitrary_downmix = 2 as core::ffi::c_int as WORD32;
                temp = ixheaacd_read_bits_buf(it_bit_buff, 11 as WORD);
                (*config).bs_arbitrary_downmix_residual_sampling_freq_index = (temp
                    as core::ffi::c_int >> 7 as core::ffi::c_int & FOUR_BIT_MASK)
                    as WORD32;
                if (*config).bs_arbitrary_downmix_residual_sampling_freq_index
                    > MAX_RES_SAMP_FREQ_IDX
                {
                    return IA_FATAL_ERROR as IA_ERRORCODE;
                }
                (*config).bs_arbitrary_downmix_residual_frames_per_spatial_frame = (temp
                    as core::ffi::c_int >> 5 as core::ffi::c_int & TWO_BIT_MASK)
                    as WORD32;
                (*config).bs_arbitrary_downmix_residual_bands = (temp as core::ffi::c_int
                    & FIVE_BIT_MASK) as WORD32;
                if (*config).bs_arbitrary_downmix_residual_bands
                    >= ixheaacd_freq_res_table[(*config).bs_freq_res as usize]
                {
                    return IA_FATAL_ERROR as IA_ERRORCODE;
                }
            }
            2 => {
                (*config).arbitrary_tree = 1 as core::ffi::c_int as WORD32;
                (*config).num_out_chan_at = 0 as core::ffi::c_int as WORD32;
                (*config).num_ott_boxes_at = 0 as core::ffi::c_int as WORD32;
                ch = 0 as core::ffi::c_int as WORD32;
                while ch < num_out_chan {
                    tmp_open = 1 as core::ffi::c_int as WORD32;
                    idx = 0 as core::ffi::c_int as WORD32;
                    while tmp_open > 0 as core::ffi::c_int {
                        (*config).bs_ott_box_present_at[ch as usize][idx as usize] = ixheaacd_read_bits_buf(
                            it_bit_buff,
                            1 as WORD,
                        );
                        if (*config).bs_ott_box_present_at[ch as usize][idx as usize]
                            != 0
                        {
                            (*config).num_ott_boxes_at += 1;
                            tmp_open += 1;
                        } else {
                            (*config).num_out_chan_at += 1;
                            tmp_open -= 1;
                        }
                        if (*config).num_ott_boxes_at >= 56 as core::ffi::c_int {
                            return IA_FATAL_ERROR as IA_ERRORCODE;
                        }
                        if (*config).num_out_chan_at > MAX_OUTPUT_CHANNELS_AT_MPS {
                            return IA_FATAL_ERROR as IA_ERRORCODE;
                        }
                        idx += 1;
                        if idx >= MAX_ARBITRARY_TREE_INDEX {
                            return IA_FATAL_ERROR as IA_ERRORCODE;
                        }
                    }
                    ch += 1;
                }
                i = 0 as core::ffi::c_int as WORD32;
                while i < (*config).num_ott_boxes_at {
                    temp = ixheaacd_read_bits_buf(it_bit_buff, 2 as WORD);
                    (*config).bs_ott_default_cld_at[i as usize] = (temp
                        as core::ffi::c_int >> 1 as core::ffi::c_int & ONE_BIT_MASK)
                        as WORD32;
                    (*config).bs_ott_mode_lfe_at[i as usize] = (temp as core::ffi::c_int
                        & ONE_BIT_MASK) as WORD32;
                    if (*config).bs_ott_mode_lfe_at[i as usize] != 0 {
                        (*config).bs_ott_bands_at[i as usize] = ixheaacd_read_bits_buf(
                            it_bit_buff,
                            5 as WORD,
                        );
                        if (*config).bs_ott_bands_at[i as usize] > MAX_PARAMETER_BANDS {
                            return IA_FATAL_ERROR as IA_ERRORCODE;
                        }
                    } else {
                        (*config).bs_ott_bands_at[i as usize] = (*ixheaacd_mps_dec_bitdec_tables)
                            .freq_res_table[(*config).bs_freq_res as usize];
                    }
                    i += 1;
                }
                i = 0 as core::ffi::c_int as WORD32;
                while i < (*config).num_out_chan_at {
                    (*config).bs_output_channel_pos_at[i as usize] = ixheaacd_read_bits_buf(
                        it_bit_buff,
                        5 as WORD,
                    );
                    i += 1;
                }
            }
            _ => return IA_XHEAAC_MPS_DEC_EXE_NONFATAL_INVALID_EXTENSION_TYPE,
        }
        bits_read = (((((*it_bit_buff).ptr_read_next)
            .offset_from((*it_bit_buff).ptr_bit_buf_base) as core::ffi::c_long
            + 1 as core::ffi::c_long) << 3 as core::ffi::c_int)
            - ((*it_bit_buff).bit_pos as core::ffi::c_int + 1 as core::ffi::c_int)
                as core::ffi::c_long - tmp as core::ffi::c_long) as WORD32;
        n_fill_bits = 8 as WORD32 * sac_ext_len - bits_read;
        while n_fill_bits > 7 as core::ffi::c_int {
            ixheaacd_read_bits_buf(it_bit_buff, 8 as WORD);
            n_fill_bits -= 8 as core::ffi::c_int;
        }
        if n_fill_bits > 0 as core::ffi::c_int {
            ixheaacd_read_bits_buf(it_bit_buff, n_fill_bits as WORD);
        }
        ba -= (8 as WORD32 * sac_ext_len) as core::ffi::c_int;
        (*config).sac_ext_cnt += 1;
        if (*config).sac_ext_cnt >= MAX_NUM_EXT_TYPES {
            return IA_FATAL_ERROR as IA_ERRORCODE;
        }
    }
    return IA_NO_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_parse_specific_config(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
    mut sac_header_len: WORD32,
) -> IA_ERRORCODE {
    let mut err_code: IA_ERRORCODE = IA_NO_ERROR;
    let mut config: *mut ia_mps_spatial_bs_config_struct = &mut (*pstr_mps_state)
        .bs_config;
    let mut curr_state: *mut ia_heaac_mps_state_struct = pstr_mps_state;
    let mut p_tree_property_table: *const ia_mps_dec_tree_properties_struct = ((*(*pstr_mps_state)
        .ia_mps_dec_mps_table
        .bitdec_table_ptr)
        .tree_property_table)
        .as_mut_ptr();
    let mut mps_bit_buf: *mut ia_bit_buf_struct = (*pstr_mps_state).ptr_mps_bit_buff
        as *mut ia_bit_buf_struct;
    let mut i: WORD32 = 0;
    let mut hc: WORD32 = 0;
    let mut hb: WORD32 = 0;
    let mut num_header_bits: WORD32 = 0;
    let mut ott_mode_lfe: [WORD32; 5] = [0; 5];
    let mut tmp: WORD32 = (((((*mps_bit_buf).ptr_read_next)
        .offset_from((*mps_bit_buf).ptr_bit_buf_base) as core::ffi::c_long
        + 1 as core::ffi::c_long) << 3 as core::ffi::c_int)
        - ((*mps_bit_buf).bit_pos as core::ffi::c_int + 1 as core::ffi::c_int)
            as core::ffi::c_long) as WORD32;
    let mut bits_available: WORD32 = sac_header_len << 3 as core::ffi::c_int;
    let mut temp: WORD32 = 0;
    let mut alignment_bits: WORD32 = 0 as WORD32;
    (*config).bs_sampling_freq_index = ixheaacd_read_bits_buf(mps_bit_buf, 4 as WORD);
    if (*config).bs_sampling_freq_index == 15 as core::ffi::c_int {
        (*config).bs_sampling_frequency = ixheaacd_read_bits_buf(
            mps_bit_buf,
            24 as WORD,
        );
    }
    temp = ixheaacd_read_bits_buf(mps_bit_buf, 14 as WORD);
    (*config).bs_frame_length = (temp as core::ffi::c_int >> 7 as core::ffi::c_int
        & SEVEN_BIT_MASK) as WORD32;
    if (*config).bs_frame_length >= MAX_QMF_BUF_LEN - 1 as core::ffi::c_int {
        return IA_FATAL_ERROR as IA_ERRORCODE;
    }
    (*config).bs_freq_res = (temp as core::ffi::c_int >> 4 as core::ffi::c_int
        & THREE_BIT_MASK) as WORD32;
    if (*config).bs_freq_res == 0 as core::ffi::c_int {
        return IA_XHEAAC_MPS_DEC_EXE_FATAL_INVALID_PARAMETER_BANDS as IA_ERRORCODE;
    }
    (*config).bs_tree_config = (temp as core::ffi::c_int & FOUR_BIT_MASK) as WORD32;
    if (*config).bs_tree_config >= 7 as core::ffi::c_int {
        return IA_FATAL_ERROR as IA_ERRORCODE;
    }
    if (*config).bs_tree_config != 15 as core::ffi::c_int {
        (*curr_state).num_ott_boxes = (*p_tree_property_table
            .offset((*config).bs_tree_config as isize))
            .num_ott_boxes;
        (*curr_state).num_ttt_boxes = (*p_tree_property_table
            .offset((*config).bs_tree_config as isize))
            .num_ttt_boxes;
        (*curr_state).num_input_channels = (*p_tree_property_table
            .offset((*config).bs_tree_config as isize))
            .num_input_channels;
        (*curr_state).num_output_channels = (*p_tree_property_table
            .offset((*config).bs_tree_config as isize))
            .num_output_channels;
        i = 0 as core::ffi::c_int as WORD32;
        while i < MAX_NUM_OTT {
            ott_mode_lfe[i as usize] = (*p_tree_property_table
                .offset((*config).bs_tree_config as isize))
                .ott_mode_lfe[i as usize];
            i += 1;
        }
    }
    temp = ixheaacd_read_bits_buf(mps_bit_buf, 19 as WORD);
    (*config).bs_quant_mode = (temp as core::ffi::c_int >> 17 as core::ffi::c_int
        & TWO_BIT_MASK) as WORD32;
    (*config).bs_one_icc = (temp as core::ffi::c_int >> 16 as core::ffi::c_int
        & ONE_BIT_MASK) as WORD32;
    (*config).bs_arbitrary_downmix = (temp as core::ffi::c_int >> 15 as core::ffi::c_int
        & ONE_BIT_MASK) as WORD32;
    (*config).bs_fixed_gain_sur = (temp as core::ffi::c_int >> 12 as core::ffi::c_int
        & THREE_BIT_MASK) as WORD32;
    if (*config).bs_fixed_gain_sur >= 5 as core::ffi::c_int {
        return IA_FATAL_ERROR as IA_ERRORCODE;
    }
    (*config).bs_fixed_gain_lfe = (temp as core::ffi::c_int >> 9 as core::ffi::c_int
        & THREE_BIT_MASK) as WORD32;
    if (*config).bs_fixed_gain_lfe >= 5 as core::ffi::c_int {
        return IA_FATAL_ERROR as IA_ERRORCODE;
    }
    (*config).bs_fixed_gain_dmx = (temp as core::ffi::c_int >> 6 as core::ffi::c_int
        & THREE_BIT_MASK) as WORD32;
    (*config).bs_matrix_mode = (temp as core::ffi::c_int >> 5 as core::ffi::c_int
        & ONE_BIT_MASK) as WORD32;
    (*config).bs_temp_shape_config = (temp as core::ffi::c_int >> 3 as core::ffi::c_int
        & TWO_BIT_MASK) as WORD32;
    if (*config).bs_temp_shape_config == 3 as core::ffi::c_int {
        return IA_FATAL_ERROR as IA_ERRORCODE;
    }
    (*config).bs_decorr_config = (temp as core::ffi::c_int >> 1 as core::ffi::c_int
        & TWO_BIT_MASK) as WORD32;
    (*config).bs_3d_audio_mode = (temp as core::ffi::c_int & ONE_BIT_MASK) as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*curr_state).num_ott_boxes {
        if ott_mode_lfe[i as usize] != 0 {
            (*config).bs_ott_bands[i as usize] = ixheaacd_read_bits_buf(
                mps_bit_buf,
                5 as WORD,
            );
            if (*config).bs_ott_bands[i as usize] > MAX_PARAMETER_BANDS {
                return IA_FATAL_ERROR as IA_ERRORCODE;
            }
        }
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*curr_state).num_ttt_boxes {
        temp = ixheaacd_read_bits_buf(mps_bit_buf, 4 as WORD);
        (*config).bs_ttt_dual_mode[i as usize] = (temp as core::ffi::c_int
            >> 3 as core::ffi::c_int & ONE_BIT_MASK) as WORD32;
        (*config).bs_ttt_mode_low[i as usize] = (temp as core::ffi::c_int
            & THREE_BIT_MASK) as WORD32;
        if (*config).bs_ttt_dual_mode[i as usize] != 0 {
            temp = ixheaacd_read_bits_buf(mps_bit_buf, 8 as WORD);
            (*config).bs_ttt_mode_high[i as usize] = (temp as core::ffi::c_int
                >> 5 as core::ffi::c_int & THREE_BIT_MASK) as WORD32;
            (*config).bs_ttt_bands_low[i as usize] = (temp as core::ffi::c_int
                & FIVE_BIT_MASK) as WORD32;
            if (*config).bs_ttt_bands_low[i as usize] > MAX_PARAMETER_BANDS {
                return IA_FATAL_ERROR as IA_ERRORCODE;
            }
        }
        i += 1;
    }
    if (*config).bs_temp_shape_config == 2 as core::ffi::c_int {
        (*config).bs_env_quant_mode = ixheaacd_read_bits_buf(mps_bit_buf, 1 as WORD);
    }
    if (*config).bs_3d_audio_mode != 0 {
        (*config).bs_3d_audio_hrtf_set = ixheaacd_read_bits_buf(mps_bit_buf, 2 as WORD);
        if (*config).bs_3d_audio_hrtf_set == 0 as core::ffi::c_int {
            temp = ixheaacd_read_bits_buf(mps_bit_buf, 4 as WORD);
            (*config).bs_hrtf_freq_res = (temp as core::ffi::c_int
                >> 1 as core::ffi::c_int & THREE_BIT_MASK) as WORD32;
            (*config).bs_hrtf_num_chan = 5 as core::ffi::c_int as WORD32;
            (*config).bs_hrtf_asymmetric = (temp as core::ffi::c_int & ONE_BIT_MASK)
                as WORD32;
            (*config).hrtf_num_band = (*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .bitdec_table_ptr)
                .freq_res_table[(*config).bs_hrtf_freq_res as usize];
            hc = 0 as core::ffi::c_int as WORD32;
            while hc < (*config).bs_hrtf_num_chan {
                hb = 0 as core::ffi::c_int as WORD32;
                while hb < (*config).hrtf_num_band {
                    (*config).bs_hrtf_level_left[hc as usize][hb as usize] = ixheaacd_read_bits_buf(
                        mps_bit_buf,
                        6 as WORD,
                    );
                    hb += 1;
                }
                hb = 0 as core::ffi::c_int as WORD32;
                while hb < (*config).hrtf_num_band {
                    (*config).bs_hrtf_level_right[hc as usize][hb as usize] = if (*config)
                        .bs_hrtf_asymmetric != 0
                    {
                        ixheaacd_read_bits_buf(mps_bit_buf, 6 as WORD)
                    } else {
                        (*config).bs_hrtf_level_left[hc as usize][hb as usize]
                    };
                    hb += 1;
                }
                (*config).bs_hrtf_phase[hc as usize] = ixheaacd_read_bits_buf(
                    mps_bit_buf,
                    1 as WORD,
                );
                hb = 0 as core::ffi::c_int as WORD32;
                while hb < (*config).hrtf_num_band {
                    (*config).bs_hrtf_phase_lr[hc as usize][hb as usize] = (if (*config)
                        .bs_hrtf_phase[hc as usize] != 0
                    {
                        ixheaacd_read_bits_buf(mps_bit_buf, 6 as WORD)
                            as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    }) as WORD32;
                    hb += 1;
                }
                hc += 1;
            }
        }
    }
    ixheaacd_byte_align(mps_bit_buf, &mut alignment_bits);
    num_header_bits = (((((*mps_bit_buf).ptr_read_next)
        .offset_from((*mps_bit_buf).ptr_bit_buf_base) as core::ffi::c_long
        + 1 as core::ffi::c_long) << 3 as core::ffi::c_int)
        - ((*mps_bit_buf).bit_pos as core::ffi::c_int + 1 as core::ffi::c_int)
            as core::ffi::c_long - tmp as core::ffi::c_long) as WORD32;
    bits_available -= num_header_bits;
    err_code = ixheaacd_parse_extension_config(
        config,
        (*curr_state).num_ott_boxes,
        (*curr_state).num_ttt_boxes,
        (*curr_state).num_output_channels,
        bits_available,
        mps_bit_buf,
        (*pstr_mps_state).ia_mps_dec_mps_table.bitdec_table_ptr,
    );
    if err_code != IA_NO_ERROR {
        return err_code;
    }
    return IA_NO_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_default_specific_config(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
    mut sampling_freq: WORD32,
) -> IA_ERRORCODE {
    let mut config: *mut ia_mps_spatial_bs_config_struct = &mut (*pstr_mps_state)
        .bs_config;
    let mut curr_state: *mut ia_heaac_mps_state_struct = pstr_mps_state;
    let mut p_tree_property_table: *const ia_mps_dec_tree_properties_struct = ((*(*pstr_mps_state)
        .ia_mps_dec_mps_table
        .bitdec_table_ptr)
        .tree_property_table)
        .as_mut_ptr();
    let mut i: WORD32 = 0;
    let mut ott_mode_lfe: [WORD32; 5] = [0; 5];
    (*config).bs_sampling_freq_index = 15 as core::ffi::c_int as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < 15 as core::ffi::c_int {
        if sampling_freq
            == (*(*pstr_mps_state).ia_mps_dec_mps_table.bitdec_table_ptr)
                .sampling_freq_table[i as usize]
        {
            (*config).bs_sampling_freq_index = i;
        }
        i += 1;
    }
    if (*config).bs_sampling_freq_index == 15 as core::ffi::c_int {
        (*config).bs_sampling_frequency = sampling_freq;
    }
    (*config).bs_frame_length = 31 as core::ffi::c_int as WORD32;
    (*config).bs_freq_res = 1 as core::ffi::c_int as WORD32;
    (*config).bs_tree_config = 2 as core::ffi::c_int as WORD32;
    if (*config).bs_tree_config > 5 as core::ffi::c_int {
        return IA_XHEAAC_MPS_DEC_EXE_FATAL_UNSUPPRORTED_TREE_CONFIG as IA_ERRORCODE;
    }
    if (*config).bs_tree_config != 15 as core::ffi::c_int {
        (*curr_state).num_ott_boxes = (*p_tree_property_table
            .offset((*config).bs_tree_config as isize))
            .num_ott_boxes;
        (*curr_state).num_ttt_boxes = (*p_tree_property_table
            .offset((*config).bs_tree_config as isize))
            .num_ttt_boxes;
        (*curr_state).num_input_channels = (*p_tree_property_table
            .offset((*config).bs_tree_config as isize))
            .num_input_channels;
        (*curr_state).num_output_channels = (*p_tree_property_table
            .offset((*config).bs_tree_config as isize))
            .num_output_channels;
        memcpy(
            ott_mode_lfe.as_mut_ptr() as *mut core::ffi::c_void,
            ((*p_tree_property_table.offset((*config).bs_tree_config as isize))
                .ott_mode_lfe)
                .as_ptr() as *const core::ffi::c_void,
            (MAX_NUM_OTT as size_t)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
        );
    }
    (*config).bs_quant_mode = 0 as core::ffi::c_int as WORD32;
    (*config).bs_one_icc = 0 as core::ffi::c_int as WORD32;
    (*config).bs_arbitrary_downmix = 0 as core::ffi::c_int as WORD32;
    (*config).bs_residual_coding = 0 as core::ffi::c_int as WORD32;
    (*config).bs_smooth_config = 0 as core::ffi::c_int as WORD32;
    (*config).bs_fixed_gain_sur = 2 as core::ffi::c_int as WORD32;
    (*config).bs_fixed_gain_lfe = 1 as core::ffi::c_int as WORD32;
    (*config).bs_fixed_gain_dmx = 0 as core::ffi::c_int as WORD32;
    (*config).bs_matrix_mode = 1 as core::ffi::c_int as WORD32;
    (*config).bs_temp_shape_config = 0 as core::ffi::c_int as WORD32;
    (*config).bs_decorr_config = 0 as core::ffi::c_int as WORD32;
    if (*config).bs_tree_config == 15 as core::ffi::c_int {
        return IA_XHEAAC_MPS_DEC_EXE_FATAL_UNSUPPRORTED_TREE_CONFIG as IA_ERRORCODE;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*curr_state).num_ott_boxes {
        if ott_mode_lfe[i as usize] != 0 {
            (*config).bs_ott_bands[i as usize] = 28 as core::ffi::c_int as WORD32;
        }
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*curr_state).num_ttt_boxes {
        (*config).bs_ttt_dual_mode[i as usize] = 0 as core::ffi::c_int as WORD32;
        (*config).bs_ttt_mode_low[i as usize] = 1 as core::ffi::c_int as WORD32;
        if (*config).bs_ttt_dual_mode[i as usize] != 0 {
            (*config).bs_ttt_mode_high[i as usize] = 1 as core::ffi::c_int as WORD32;
            (*config).bs_ttt_bands_low[i as usize] = 28 as core::ffi::c_int as WORD32;
        }
        i += 1;
    }
    return IA_NO_ERROR;
}
unsafe extern "C" fn ixheaacd_coarse_2_fine(
    mut data: *mut WORD32,
    mut data_type: WORD32,
    mut start_band: WORD32,
    mut num_bands: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    i = start_band;
    while i < start_band + num_bands {
        *data.offset(i as isize) <<= 1 as core::ffi::c_int;
        i += 1;
    }
    if data_type == CLD {
        i = start_band;
        while i < start_band + num_bands {
            if *data.offset(i as isize) == -(14 as core::ffi::c_int) {
                *data.offset(i as isize) = -(15 as core::ffi::c_int) as WORD32;
            } else if *data.offset(i as isize) == 14 as core::ffi::c_int {
                *data.offset(i as isize) = 15 as core::ffi::c_int as WORD32;
            }
            i += 1;
        }
    }
}
unsafe extern "C" fn ixheaacd_fine_2_coarse(
    mut data: *mut WORD32,
    mut start_band: WORD32,
    mut num_bands: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    i = start_band;
    while i < start_band + num_bands {
        *data.offset(i as isize) >>= 1 as core::ffi::c_int;
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_get_stride_map(
    mut freq_res_stride: WORD32,
    mut start_band: WORD32,
    mut stop_band: WORD32,
    mut a_strides: *mut WORD32,
    mut ixheaacd_mps_dec_bitdec_tables: *mut ia_mps_dec_bitdec_tables_struct,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut pb: WORD32 = 0;
    let mut pb_stride: WORD32 = 0;
    let mut data_bands: WORD32 = 0;
    let mut str_offset: WORD32 = 0;
    pb_stride = (*ixheaacd_mps_dec_bitdec_tables)
        .pb_stride_table[freq_res_stride as usize];
    data_bands = ((stop_band as core::ffi::c_int - start_band as core::ffi::c_int
        - 1 as core::ffi::c_int) / pb_stride as core::ffi::c_int + 1 as core::ffi::c_int)
        as WORD32;
    *a_strides.offset(0 as core::ffi::c_int as isize) = start_band;
    pb = 1 as core::ffi::c_int as WORD32;
    while pb <= data_bands {
        *a_strides.offset(pb as isize) = *a_strides
            .offset((pb as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
            + pb_stride;
        pb += 1;
    }
    str_offset = 0 as core::ffi::c_int as WORD32;
    while *a_strides.offset(data_bands as isize) > stop_band {
        if str_offset < data_bands {
            str_offset += 1;
        }
        i = str_offset;
        while i <= data_bands {
            let ref mut fresh10 = *a_strides.offset(i as isize);
            *fresh10 -= 1;
            i += 1;
        }
    }
    return data_bands;
}
unsafe extern "C" fn ixheaacd_ec_data_dec(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
    mut ll_data: *mut ia_mps_dec_lossless_data_struct,
    mut data: *mut [[WORD32; 28]; 8],
    mut lastdata: *mut [WORD32; 28],
    mut datatype: WORD32,
    mut box_idx: WORD32,
    mut param_idx: WORD32,
    mut start_band: WORD32,
    mut stop_band: WORD32,
) -> IA_ERRORCODE {
    let mut error_code: IA_ERRORCODE = IA_NO_ERROR;
    let mut i: WORD32 = 0;
    let mut pb: WORD32 = 0;
    let mut data_sets: WORD32 = 0;
    let mut set_idx: WORD32 = 0;
    let mut bs_data_pair: WORD32 = 0;
    let mut data_bands: WORD32 = 0;
    let mut old_quant_coarse_xxx: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut a_strides: [WORD32; 29] = [0 as core::ffi::c_int; 29];
    let mut frame: *mut ia_mps_dec_spatial_bs_frame_struct = (*pstr_mps_state).bs_frame;
    let mut mps_bit_buf: *mut ia_bit_buf_struct = (*pstr_mps_state).ptr_mps_bit_buff
        as *mut ia_bit_buf_struct;
    data_sets = 0 as core::ffi::c_int as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*pstr_mps_state).num_parameter_sets {
        (*ll_data).bs_xxx_data_mode[param_idx as usize][i as usize] = ixheaacd_read_bits_buf(
            mps_bit_buf,
            2 as WORD,
        );
        if (*ll_data).bs_xxx_data_mode[param_idx as usize][i as usize]
            == 3 as core::ffi::c_int
        {
            data_sets += 1;
        }
        i += 1;
    }
    set_idx = 0 as core::ffi::c_int as WORD32;
    old_quant_coarse_xxx = (*ll_data).bs_quant_coarse_xxx_prev[param_idx as usize];
    while set_idx < data_sets {
        temp = ixheaacd_read_bits_buf(mps_bit_buf, 4 as WORD);
        bs_data_pair = (temp as core::ffi::c_int >> 3 as core::ffi::c_int & ONE_BIT_MASK)
            as WORD32;
        (*ll_data).bs_quant_coarse_xxx[param_idx as usize][set_idx as usize] = (temp
            as core::ffi::c_int >> 2 as core::ffi::c_int & ONE_BIT_MASK) as WORD32;
        (*ll_data).bs_freq_res_stride_xxx[param_idx as usize][set_idx as usize] = (temp
            as core::ffi::c_int & TWO_BIT_MASK) as WORD32;
        if set_idx == 7 as core::ffi::c_int && bs_data_pair == 1 as core::ffi::c_int {
            if (*pstr_mps_state).ec_flag != 0 {
                bs_data_pair = 0 as core::ffi::c_int as WORD32;
            } else {
                return IA_FATAL_ERROR as IA_ERRORCODE
            }
        }
        if (*ll_data).bs_quant_coarse_xxx[param_idx as usize][set_idx as usize]
            != old_quant_coarse_xxx
        {
            if old_quant_coarse_xxx != 0 {
                ixheaacd_coarse_2_fine(
                    (*lastdata.offset(box_idx as isize)).as_mut_ptr(),
                    datatype,
                    start_band,
                    stop_band - start_band,
                );
            } else {
                ixheaacd_fine_2_coarse(
                    (*lastdata.offset(box_idx as isize)).as_mut_ptr(),
                    start_band,
                    stop_band - start_band,
                );
            }
        }
        data_bands = ixheaacd_get_stride_map(
            (*ll_data).bs_freq_res_stride_xxx[param_idx as usize][set_idx as usize],
            start_band,
            stop_band,
            a_strides.as_mut_ptr(),
            (*pstr_mps_state).ia_mps_dec_mps_table.bitdec_table_ptr,
        );
        pb = 0 as core::ffi::c_int as WORD32;
        while pb < data_bands {
            (*lastdata.offset(box_idx as isize))[(start_band + pb) as usize] = (*lastdata
                .offset(box_idx as isize))[a_strides[pb as usize] as usize];
            pb += 1;
        }
        error_code = ixheaacd_mps_ecdatapairdec(
            mps_bit_buf,
            (*data.offset(box_idx as isize)).as_mut_ptr(),
            (*lastdata.offset(box_idx as isize)).as_mut_ptr(),
            datatype,
            set_idx,
            start_band,
            data_bands,
            bs_data_pair,
            (*ll_data).bs_quant_coarse_xxx[param_idx as usize][set_idx as usize],
            ((*frame).bs_independency_flag == 0 || set_idx > 0 as core::ffi::c_int)
                as core::ffi::c_int,
            0 as WORD32,
            1 as WORD32,
            (*pstr_mps_state).ec_flag,
        ) as IA_ERRORCODE;
        if error_code != IA_NO_ERROR {
            return error_code;
        }
        if datatype == CLD {
            let mut band: WORD32 = 0;
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*pstr_mps_state).num_parameter_sets {
                band = start_band;
                while band < stop_band {
                    if (*data.offset(box_idx as isize))[i as usize][band as usize]
                        > 15 as core::ffi::c_int
                        || (*data.offset(box_idx as isize))[i as usize][band as usize]
                            < -(15 as core::ffi::c_int)
                    {
                        return IA_FATAL_ERROR as IA_ERRORCODE;
                    }
                    band += 1;
                }
                i += 1;
            }
        } else if datatype == ICC {
            let mut band_0: WORD32 = 0;
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*pstr_mps_state).num_parameter_sets {
                band_0 = start_band;
                while band_0 < stop_band {
                    if (*data.offset(box_idx as isize))[i as usize][band_0 as usize]
                        > 7 as core::ffi::c_int
                        || (*data.offset(box_idx as isize))[i as usize][band_0 as usize]
                            < 0 as core::ffi::c_int
                    {
                        return IA_FATAL_ERROR as IA_ERRORCODE;
                    }
                    band_0 += 1;
                }
                i += 1;
            }
        }
        pb = 0 as core::ffi::c_int as WORD32;
        while pb < data_bands {
            i = a_strides[pb as usize];
            while i
                < a_strides[(pb as core::ffi::c_int + 1 as core::ffi::c_int) as usize]
            {
                (*lastdata.offset(box_idx as isize))[i as usize] = (*data
                    .offset(
                        box_idx as isize,
                    ))[(set_idx + bs_data_pair) as usize][(start_band + pb) as usize];
                i += 1;
            }
            pb += 1;
        }
        old_quant_coarse_xxx = (*ll_data)
            .bs_quant_coarse_xxx[param_idx as usize][set_idx as usize];
        if bs_data_pair != 0 {
            (*ll_data)
                .bs_quant_coarse_xxx[param_idx
                as usize][(set_idx as core::ffi::c_int + 1 as core::ffi::c_int)
                as usize] = (*ll_data)
                .bs_quant_coarse_xxx[param_idx as usize][set_idx as usize];
            (*ll_data)
                .bs_freq_res_stride_xxx[param_idx
                as usize][(set_idx as core::ffi::c_int + 1 as core::ffi::c_int)
                as usize] = (*ll_data)
                .bs_freq_res_stride_xxx[param_idx as usize][set_idx as usize];
        }
        set_idx += bs_data_pair as core::ffi::c_int + 1 as core::ffi::c_int;
    }
    return error_code;
}
unsafe extern "C" fn ixheaacd_parse_arbitrary_downmix_data(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
) -> IA_ERRORCODE {
    let mut frame: *mut ia_mps_dec_spatial_bs_frame_struct = (*pstr_mps_state).bs_frame;
    let mut error_code: IA_ERRORCODE = IA_NO_ERROR;
    let mut offset: WORD32 = (*pstr_mps_state).num_ott_boxes
        + 4 as WORD32 * (*pstr_mps_state).num_ttt_boxes;
    let mut num_input_channels: WORD32 = (*pstr_mps_state).num_input_channels;
    let mut bitstream_parameter_bands: WORD32 = (*pstr_mps_state)
        .bitstream_parameter_bands;
    let mut ch: WORD32 = 0;
    ch = 0 as core::ffi::c_int as WORD32;
    while ch < num_input_channels {
        error_code = ixheaacd_ec_data_dec(
            pstr_mps_state,
            &mut (*frame).cld_lossless_data,
            ((*frame).cmp_arbdmx_gain_idx).as_mut_ptr(),
            ((*frame).cmp_arbdmx_gain_idx_prev).as_mut_ptr(),
            CLD,
            ch,
            offset + ch,
            0 as WORD32,
            bitstream_parameter_bands,
        );
        if error_code != IA_NO_ERROR {
            return error_code;
        }
        ch += 1;
    }
    return error_code;
}
unsafe extern "C" fn ixheaacd_decode_icc_diff_code(
    mut it_bit_buff: *mut ia_bit_buf_struct,
) -> WORD32 {
    let mut value: WORD32 = 0 as WORD32;
    let mut count: WORD32 = 0 as WORD32;
    while ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD) == 0 as core::ffi::c_int
        && {
            let fresh13 = count;
            count = count + 1;
            fresh13 < 7 as core::ffi::c_int
        }
    {
        value += 1;
    }
    return value;
}
unsafe extern "C" fn ixheaacd_parse_residual_data(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
) -> IA_ERRORCODE {
    let mut ich: WORD32 = 0;
    let mut ch: WORD32 = 0;
    let mut rfpsf: WORD32 = 0;
    let mut ps: WORD32 = 0;
    let mut pb: WORD32 = 0;
    let mut aac_tables_ptr: *mut ia_mps_dec_residual_aac_tables_struct = (*pstr_mps_state)
        .ia_mps_dec_mps_table
        .aac_tab as *mut ia_mps_dec_residual_aac_tables_struct;
    let mut i: WORD32 = 0;
    let mut frame: *mut ia_mps_dec_spatial_bs_frame_struct = (*pstr_mps_state).bs_frame;
    let mut config: *mut ia_mps_spatial_bs_config_struct = &mut (*pstr_mps_state)
        .bs_config;
    let mut num_ott_boxes: WORD32 = (*pstr_mps_state).num_ott_boxes;
    let mut num_parameter_sets: WORD32 = (*pstr_mps_state).num_parameter_sets;
    let mut residual_frames_per_spatial_frame: WORD32 = (*pstr_mps_state)
        .residual_frames_per_spatial_frame;
    let mut upd_qmf: WORD32 = (*pstr_mps_state).upd_qmf;
    let mut loop_counter: WORD32 = num_ott_boxes + (*pstr_mps_state).num_ttt_boxes;
    let mut p_mdct_res: *mut WORD32 = 0 as *mut WORD32;
    let mut p_res_mdct: *mut WORD32 = (*(*pstr_mps_state).array_struct).res_mdct;
    let mut mps_bit_buf: *mut ia_bit_buf_struct = (*pstr_mps_state).ptr_mps_bit_buff
        as *mut ia_bit_buf_struct;
    let mut error_code: WORD16 = IA_NO_ERROR as WORD16;
    ich = 0 as core::ffi::c_int as WORD32;
    while ich < loop_counter {
        ch = ich;
        p_mdct_res = p_res_mdct;
        if (*config).bs_residual_bands[ch as usize] > 0 as core::ffi::c_int {
            if ch < num_ott_boxes {
                ps = 0 as core::ffi::c_int as WORD32;
                while ps < num_parameter_sets {
                    (*frame).res_data.bs_icc_diff_present[ch as usize][ps as usize] = ixheaacd_read_bits_buf(
                        mps_bit_buf,
                        1 as WORD,
                    );
                    if (*frame).res_data.bs_icc_diff_present[ch as usize][ps as usize]
                        != 0
                    {
                        pb = 0 as core::ffi::c_int as WORD32;
                        while pb < (*config).bs_residual_bands[ch as usize] {
                            (*frame)
                                .res_data
                                .bs_icc_diff[ch as usize][ps as usize][pb as usize] = ixheaacd_decode_icc_diff_code(
                                mps_bit_buf,
                            );
                            (*frame)
                                .ott_icc_diff_idx[ch as usize][ps as usize][pb as usize] = (*frame)
                                .res_data
                                .bs_icc_diff[ch as usize][ps as usize][pb as usize];
                            pb += 1;
                        }
                    }
                    ps += 1;
                }
            }
            p_mdct_res = p_res_mdct;
            rfpsf = 0 as core::ffi::c_int as WORD32;
            while rfpsf < residual_frames_per_spatial_frame {
                error_code = ixheaacd_res_read_ics(
                    mps_bit_buf,
                    ((*pstr_mps_state).p_aac_decoder_channel_info).as_mut_ptr(),
                    1 as WORD32,
                    aac_tables_ptr,
                    ((*pstr_mps_state).tot_sf_bands_ls).as_mut_ptr(),
                );
                if error_code != 0 {
                    if (*pstr_mps_state).ec_flag != 0 {
                        (*pstr_mps_state).frame_ok = 0 as core::ffi::c_int as WORD32;
                    } else {
                        return error_code as IA_ERRORCODE
                    }
                }
                if 1 as core::ffi::c_int
                    == (*(*pstr_mps_state)
                        .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                        .tns_data
                        .tns_data_present
                {
                    ixheaacd_res_ctns_apply(
                        (*pstr_mps_state)
                            .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize],
                        (*(*pstr_mps_state)
                            .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                            .ics_info
                            .max_sf_bands,
                        aac_tables_ptr,
                    );
                }
                (*pstr_mps_state).res_block_type[ch as usize][rfpsf as usize] = (*(*pstr_mps_state)
                    .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                    .ics_info
                    .window_sequence as WORD32;
                i = 0 as core::ffi::c_int as WORD32;
                while i < AAC_FRAME_LENGTH {
                    let fresh11 = p_mdct_res;
                    p_mdct_res = p_mdct_res.offset(1);
                    *fresh11 = *((*(*pstr_mps_state)
                        .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                        .p_spectral_coefficient)
                        .offset(i as isize);
                    i += 1;
                }
                if (*(*pstr_mps_state)
                    .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                    .ics_info
                    .window_sequence as core::ffi::c_int == EIGHT_SHORT_SEQUENCE
                    && (upd_qmf == UPD_QMF_18 || upd_qmf == UPD_QMF_24
                        || upd_qmf == UPD_QMF_30)
                {
                    error_code = ixheaacd_res_read_ics(
                        mps_bit_buf,
                        ((*pstr_mps_state).p_aac_decoder_channel_info).as_mut_ptr(),
                        1 as WORD32,
                        aac_tables_ptr,
                        ((*pstr_mps_state).tot_sf_bands_ls).as_mut_ptr(),
                    );
                    if error_code != 0 {
                        if (*pstr_mps_state).ec_flag != 0 {
                            (*pstr_mps_state).frame_ok = 0 as core::ffi::c_int as WORD32;
                        } else {
                            return error_code as IA_ERRORCODE
                        }
                    }
                    if 1 as core::ffi::c_int
                        == (*(*pstr_mps_state)
                            .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                            .tns_data
                            .tns_data_present
                    {
                        ixheaacd_res_ctns_apply(
                            (*pstr_mps_state)
                                .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize],
                            (*(*pstr_mps_state)
                                .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                                .ics_info
                                .max_sf_bands,
                            aac_tables_ptr,
                        );
                    }
                    i = 0 as core::ffi::c_int as WORD32;
                    while i < AAC_FRAME_LENGTH {
                        let fresh12 = p_mdct_res;
                        p_mdct_res = p_mdct_res.offset(1);
                        *fresh12 = *((*(*pstr_mps_state)
                            .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                            .p_spectral_coefficient)
                            .offset(i as isize);
                        i += 1;
                    }
                }
                rfpsf += 1;
            }
        }
        p_res_mdct = p_res_mdct.offset(RFX2XMDCTCOEF as isize);
        ich += 1;
    }
    return IA_NO_ERROR;
}
unsafe extern "C" fn ixheaacd_parse_extension_frame(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
) -> IA_ERRORCODE {
    let mut i: WORD32 = 0;
    let mut fr: WORD32 = 0;
    let mut gr: WORD32 = 0;
    let mut offset: WORD32 = 0;
    let mut ch: WORD32 = 0;
    let mut ext_num: WORD32 = 0;
    let mut sac_ext_type: WORD32 = 0;
    let mut sac_ext_len: WORD32 = 0;
    let mut tmp: WORD32 = 0;
    let mut bits_read: WORD32 = 0;
    let mut n_fill_bits: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut channel_grouping: [WORD32; 6] = [0; 6];
    let mut frame: *mut ia_mps_dec_spatial_bs_frame_struct = (*pstr_mps_state).bs_frame;
    let mut p_bs_config: *mut ia_mps_spatial_bs_config_struct = &mut (*pstr_mps_state)
        .bs_config;
    let mut aac_tables_ptr: *mut ia_mps_dec_residual_aac_tables_struct = (*pstr_mps_state)
        .ia_mps_dec_mps_table
        .aac_tab as *mut ia_mps_dec_residual_aac_tables_struct;
    let mut arbdmx_upd_qmf: WORD32 = (*pstr_mps_state).arbdmx_upd_qmf;
    let mut num_ott_boxes: WORD32 = (*pstr_mps_state).num_ott_boxes;
    let mut num_input_channels: WORD32 = (*pstr_mps_state).num_input_channels;
    let mut num_ttt_boxes: WORD32 = (*pstr_mps_state).num_ttt_boxes;
    let mut arbdmx_frames_per_spatial_frame: WORD32 = (*pstr_mps_state)
        .arbdmx_frames_per_spatial_frame;
    let mut p_res_mdct: *mut WORD32 = 0 as *mut WORD32;
    let mut p_mdct_res: *mut WORD32 = 0 as *mut WORD32;
    let mut sfidx: WORD32 = 0;
    let mut free_scratch: *mut core::ffi::c_void = (*pstr_mps_state).mps_scratch_mem_v;
    let mut p_sfband_info_tab: *mut ia_mps_dec_residual_sfband_info_struct = &mut (*pstr_mps_state)
        .sfband_info_tab;
    let mut mps_bit_buf: *mut ia_bit_buf_struct = (*pstr_mps_state).ptr_mps_bit_buff
        as *mut ia_bit_buf_struct;
    let mut error_code: IA_ERRORCODE = IA_NO_ERROR;
    ch = 0 as core::ffi::c_int as WORD32;
    while ch < 2 as core::ffi::c_int {
        (*pstr_mps_state).p_aac_decoder_channel_info[ch as usize] = free_scratch
            as *mut ia_mps_dec_residual_channel_info_struct;
        free_scratch = (free_scratch as *mut WORD8)
            .offset(
                ((::core::mem::size_of::<ia_mps_dec_residual_channel_info_struct>()
                    as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    as isize,
            ) as *mut core::ffi::c_void;
        (*pstr_mps_state).p_aac_decoder_dynamic_data_init[ch as usize] = free_scratch
            as *mut ia_mps_dec_residual_dynamic_data_struct;
        free_scratch = (free_scratch as *mut WORD8)
            .offset(
                ((::core::mem::size_of::<ia_mps_dec_residual_dynamic_data_struct>()
                    as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    as isize,
            ) as *mut core::ffi::c_void;
        (*(*pstr_mps_state).p_aac_decoder_channel_info[ch as usize]).p_scale_factor = ((*(*pstr_mps_state)
            .p_aac_decoder_dynamic_data_init[ch as usize])
            .a_scale_factor)
            .as_mut_ptr();
        (*(*pstr_mps_state).p_aac_decoder_channel_info[ch as usize]).p_code_book = ((*(*pstr_mps_state)
            .p_aac_decoder_dynamic_data_init[ch as usize])
            .a_code_book)
            .as_mut_ptr();
        (*(*pstr_mps_state).p_aac_decoder_channel_info[ch as usize])
            .p_spectral_coefficient = free_scratch as *mut WORD32;
        free_scratch = (free_scratch as *mut WORD8)
            .offset(
                (4096 as core::ffi::c_int
                    + (8 as core::ffi::c_int - 1 as core::ffi::c_int)
                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int)) as isize,
            ) as *mut core::ffi::c_void;
        (*(*pstr_mps_state).p_aac_decoder_channel_info[ch as usize]).p_tns_scratch = free_scratch
            as *mut WORD32;
        free_scratch = (free_scratch as *mut WORD8)
            .offset(
                (4096 as core::ffi::c_int
                    + (8 as core::ffi::c_int - 1 as core::ffi::c_int)
                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int)) as isize,
            ) as *mut core::ffi::c_void;
        (*(*pstr_mps_state).p_aac_decoder_channel_info[ch as usize])
            .ics_info
            .frame_length = AAC_FRAME_LENGTH as WORD16;
        (*(*pstr_mps_state).p_aac_decoder_channel_info[ch as usize]).common_window = 0
            as WORD16;
        ch += 1;
    }
    if (*pstr_mps_state).arbitrary_downmix == 2 as core::ffi::c_int {
        sfidx = (*p_bs_config).bs_arbitrary_downmix_residual_sampling_freq_index;
    } else {
        sfidx = (*p_bs_config).bs_residual_sampling_freq_index;
    }
    let mut psfb_idx: [*mut WORD16; 2] = [0 as *mut WORD16; 2];
    let mut psfb_width: [*const WORD8; 2] = [0 as *const WORD8; 2];
    let mut width_idx: WORD = 0;
    let mut j: WORD32 = 0;
    (*(*pstr_mps_state).p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
        .ics_info
        .sampling_rate_index = sfidx as WORD16;
    psfb_idx[0 as core::ffi::c_int as usize] = ((*p_sfband_info_tab).sfb_long_idx)
        .as_mut_ptr();
    psfb_idx[1 as core::ffi::c_int as usize] = ((*p_sfband_info_tab).sfb_short_idx)
        .as_mut_ptr();
    psfb_width[0 as core::ffi::c_int as usize] = (*aac_tables_ptr)
        .scale_factor_bands_long[sfidx as usize];
    psfb_width[1 as core::ffi::c_int as usize] = (*aac_tables_ptr)
        .scale_factor_bands_short[sfidx as usize];
    j = 1 as core::ffi::c_int as WORD32;
    while j >= 0 as core::ffi::c_int {
        let mut ptr_w: *const WORD8 = psfb_width[j as usize];
        let mut ptr_i: *mut WORD16 = psfb_idx[j as usize];
        width_idx = 0 as core::ffi::c_int as WORD;
        let fresh0 = ptr_i;
        ptr_i = ptr_i.offset(1);
        *fresh0 = width_idx as WORD16;
        loop {
            let fresh1 = ptr_w;
            ptr_w = ptr_w.offset(1);
            width_idx += *fresh1 as core::ffi::c_int;
            let fresh2 = ptr_i;
            ptr_i = ptr_i.offset(1);
            *fresh2 = width_idx as WORD16;
            if !(*ptr_w as core::ffi::c_int != -(1 as core::ffi::c_int)) {
                break;
            }
        }
        (*pstr_mps_state).tot_sf_bands_ls[j as usize] = ptr_w
            .offset_from(psfb_width[j as usize]) as core::ffi::c_long as WORD8;
        j -= 1;
    }
    (*aac_tables_ptr).sfb_index_long = ((*p_sfband_info_tab).sfb_long_idx).as_mut_ptr();
    (*aac_tables_ptr).sfb_index_short = ((*p_sfband_info_tab).sfb_short_idx)
        .as_mut_ptr();
    (*aac_tables_ptr).sfb_index_long_width = psfb_width[0 as core::ffi::c_int as usize]
        as *mut WORD8;
    (*aac_tables_ptr).sfb_index_short_width = psfb_width[1 as core::ffi::c_int as usize]
        as *mut WORD8;
    ext_num = 0 as core::ffi::c_int as WORD32;
    while ext_num < (*p_bs_config).sac_ext_cnt {
        sac_ext_type = (*p_bs_config).bs_sac_ext_type[ext_num as usize];
        if sac_ext_type < 12 as core::ffi::c_int {
            sac_ext_len = ixheaacd_read_bits_buf(mps_bit_buf, 8 as WORD);
            if sac_ext_len == 255 as core::ffi::c_int {
                sac_ext_len += ixheaacd_read_bits_buf(mps_bit_buf, 16 as WORD);
            }
            tmp = (((((*mps_bit_buf).ptr_read_next)
                .offset_from((*mps_bit_buf).ptr_bit_buf_base) as core::ffi::c_long
                + 1 as core::ffi::c_long) << 3 as core::ffi::c_int)
                - ((*mps_bit_buf).bit_pos as core::ffi::c_int + 1 as core::ffi::c_int)
                    as core::ffi::c_long) as WORD32;
            match sac_ext_type {
                0 => {
                    error_code = ixheaacd_parse_residual_data(pstr_mps_state);
                    if error_code != 0 {
                        if (*pstr_mps_state).ec_flag != 0 {
                            (*pstr_mps_state).frame_ok = 0 as core::ffi::c_int as WORD32;
                        } else {
                            return error_code
                        }
                    }
                }
                1 => {
                    match num_input_channels {
                        1 => {
                            channel_grouping[0 as core::ffi::c_int as usize] = 1
                                as core::ffi::c_int as WORD32;
                        }
                        2 => {
                            channel_grouping[0 as core::ffi::c_int as usize] = 2
                                as core::ffi::c_int as WORD32;
                        }
                        6 => {
                            channel_grouping[0 as core::ffi::c_int as usize] = 2
                                as core::ffi::c_int as WORD32;
                            channel_grouping[1 as core::ffi::c_int as usize] = 2
                                as core::ffi::c_int as WORD32;
                            channel_grouping[2 as core::ffi::c_int as usize] = 2
                                as core::ffi::c_int as WORD32;
                        }
                        _ => {
                            return IA_XHEAAC_MPS_DEC_EXE_FATAL_INVALID_INPUT_CHANNEL
                                as IA_ERRORCODE;
                        }
                    }
                    offset = num_ott_boxes + num_ttt_boxes;
                    p_res_mdct = ((*(*pstr_mps_state).array_struct).res_mdct)
                        .offset((offset as core::ffi::c_int * RFX2XMDCTCOEF) as isize);
                    ch = 0 as core::ffi::c_int as WORD32;
                    gr = 0 as core::ffi::c_int as WORD32;
                    while ch < num_input_channels {
                        p_mdct_res = p_res_mdct;
                        temp = ixheaacd_read_bits_buf(mps_bit_buf, 2 as WORD);
                        (*frame).bs_arbitrary_downmix_residual_abs[ch as usize] = (temp
                            as core::ffi::c_int >> 1 as core::ffi::c_int & ONE_BIT_MASK)
                            as WORD32;
                        (*frame)
                            .bs_arbitrary_downmix_residual_alpha_update_set[ch
                            as usize] = (temp as core::ffi::c_int & ONE_BIT_MASK)
                            as WORD32;
                        if channel_grouping[gr as usize] == 1 as core::ffi::c_int {
                            fr = 0 as core::ffi::c_int as WORD32;
                            while fr < arbdmx_frames_per_spatial_frame {
                                error_code = ixheaacd_res_read_ics(
                                    mps_bit_buf,
                                    ((*pstr_mps_state).p_aac_decoder_channel_info).as_mut_ptr(),
                                    1 as WORD32,
                                    aac_tables_ptr,
                                    ((*pstr_mps_state).tot_sf_bands_ls).as_mut_ptr(),
                                ) as IA_ERRORCODE;
                                if error_code != 0 {
                                    if (*pstr_mps_state).ec_flag != 0 {
                                        (*pstr_mps_state).frame_ok = 0 as core::ffi::c_int
                                            as WORD32;
                                    } else {
                                        return error_code
                                    }
                                }
                                if 1 as core::ffi::c_int
                                    == (*(*pstr_mps_state)
                                        .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                                        .tns_data
                                        .tns_data_present
                                {
                                    ixheaacd_res_ctns_apply(
                                        (*pstr_mps_state)
                                            .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize],
                                        (*(*pstr_mps_state)
                                            .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                                            .ics_info
                                            .max_sf_bands,
                                        aac_tables_ptr,
                                    );
                                }
                                (*pstr_mps_state)
                                    .res_block_type[(offset + ch) as usize][fr as usize] = (*(*pstr_mps_state)
                                    .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                                    .ics_info
                                    .window_sequence as WORD32;
                                i = 0 as core::ffi::c_int as WORD32;
                                while i < AAC_FRAME_LENGTH {
                                    let fresh3 = p_mdct_res;
                                    p_mdct_res = p_mdct_res.offset(1);
                                    *fresh3 = *((*(*pstr_mps_state)
                                        .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                                        .p_spectral_coefficient)
                                        .offset(i as isize);
                                    i += 1;
                                }
                                if (*(*pstr_mps_state)
                                    .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                                    .ics_info
                                    .window_sequence as core::ffi::c_int == EIGHT_SHORT_SEQUENCE
                                    && (arbdmx_upd_qmf == UPD_QMF_18
                                        || arbdmx_upd_qmf == UPD_QMF_24
                                        || arbdmx_upd_qmf == UPD_QMF_30)
                                {
                                    error_code = ixheaacd_res_read_ics(
                                        mps_bit_buf,
                                        ((*pstr_mps_state).p_aac_decoder_channel_info).as_mut_ptr(),
                                        1 as WORD32,
                                        aac_tables_ptr,
                                        ((*pstr_mps_state).tot_sf_bands_ls).as_mut_ptr(),
                                    ) as IA_ERRORCODE;
                                    if error_code != 0 {
                                        if (*pstr_mps_state).ec_flag != 0 {
                                            (*pstr_mps_state).frame_ok = 0 as core::ffi::c_int
                                                as WORD32;
                                        } else {
                                            return error_code
                                        }
                                    }
                                    if 1 as core::ffi::c_int
                                        == (*(*pstr_mps_state)
                                            .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                                            .tns_data
                                            .tns_data_present
                                    {
                                        ixheaacd_res_ctns_apply(
                                            (*pstr_mps_state)
                                                .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize],
                                            (*(*pstr_mps_state)
                                                .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                                                .ics_info
                                                .max_sf_bands,
                                            aac_tables_ptr,
                                        );
                                    }
                                    i = 0 as core::ffi::c_int as WORD32;
                                    while i < AAC_FRAME_LENGTH {
                                        let fresh4 = p_mdct_res;
                                        p_mdct_res = p_mdct_res.offset(1);
                                        *fresh4 = *((*(*pstr_mps_state)
                                            .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                                            .p_spectral_coefficient)
                                            .offset(i as isize);
                                        i += 1;
                                    }
                                }
                                fr += 1;
                            }
                            p_res_mdct = p_res_mdct.offset(RFX2XMDCTCOEF as isize);
                        } else {
                            (*frame)
                                .bs_arbitrary_downmix_residual_abs[(ch as core::ffi::c_int
                                + 1 as core::ffi::c_int) as usize] = (*frame)
                                .bs_arbitrary_downmix_residual_abs[ch as usize];
                            (*frame)
                                .bs_arbitrary_downmix_residual_alpha_update_set[(ch
                                as core::ffi::c_int + 1 as core::ffi::c_int) as usize] = (*frame)
                                .bs_arbitrary_downmix_residual_alpha_update_set[ch
                                as usize];
                            fr = 0 as core::ffi::c_int as WORD32;
                            while fr < arbdmx_frames_per_spatial_frame {
                                let mut res_mdct_1: *mut WORD32 = p_mdct_res
                                    .offset(RFX2XMDCTCOEF as isize);
                                let mut temp_0: WORD32 = 0;
                                let mut win1: WORD32 = 0;
                                let mut win2: WORD32 = 0;
                                temp_0 = ixheaacd_read_bits_buf(mps_bit_buf, 4 as WORD);
                                temp_0 = ixheaacd_read_bits_buf(mps_bit_buf, 1 as WORD);
                                if temp_0 != 0 as core::ffi::c_int {
                                    return IA_XHEAAC_MPS_DEC_EXE_FATAL_NONZERO_BIT
                                        as IA_ERRORCODE;
                                }
                                error_code = ixheaacd_res_read_ics(
                                    mps_bit_buf,
                                    ((*pstr_mps_state).p_aac_decoder_channel_info).as_mut_ptr(),
                                    1 as WORD32,
                                    aac_tables_ptr,
                                    ((*pstr_mps_state).tot_sf_bands_ls).as_mut_ptr(),
                                ) as IA_ERRORCODE;
                                if error_code != 0 {
                                    if (*pstr_mps_state).ec_flag != 0 {
                                        (*pstr_mps_state).frame_ok = 0 as core::ffi::c_int
                                            as WORD32;
                                    } else {
                                        return error_code
                                    }
                                }
                                if 1 as core::ffi::c_int
                                    == (*(*pstr_mps_state)
                                        .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                                        .tns_data
                                        .tns_data_present
                                {
                                    ixheaacd_res_ctns_apply(
                                        (*pstr_mps_state)
                                            .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize],
                                        (*(*pstr_mps_state)
                                            .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                                            .ics_info
                                            .max_sf_bands,
                                        aac_tables_ptr,
                                    );
                                }
                                win1 = (*(*pstr_mps_state)
                                    .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                                    .ics_info
                                    .window_sequence as WORD32;
                                (*pstr_mps_state)
                                    .res_block_type[(offset + ch) as usize][fr as usize] = (*(*pstr_mps_state)
                                    .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                                    .ics_info
                                    .window_sequence as WORD32;
                                i = 0 as core::ffi::c_int as WORD32;
                                while i < AAC_FRAME_LENGTH {
                                    let fresh5 = p_mdct_res;
                                    p_mdct_res = p_mdct_res.offset(1);
                                    *fresh5 = *((*(*pstr_mps_state)
                                        .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                                        .p_spectral_coefficient)
                                        .offset(i as isize);
                                    i += 1;
                                }
                                error_code = ixheaacd_res_read_ics(
                                    mps_bit_buf,
                                    ((*pstr_mps_state).p_aac_decoder_channel_info).as_mut_ptr(),
                                    1 as WORD32,
                                    aac_tables_ptr,
                                    ((*pstr_mps_state).tot_sf_bands_ls).as_mut_ptr(),
                                ) as IA_ERRORCODE;
                                if error_code != 0 {
                                    if (*pstr_mps_state).ec_flag != 0 {
                                        (*pstr_mps_state).frame_ok = 0 as core::ffi::c_int
                                            as WORD32;
                                    } else {
                                        return error_code
                                    }
                                }
                                if 1 as core::ffi::c_int
                                    == (*(*pstr_mps_state)
                                        .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                                        .tns_data
                                        .tns_data_present
                                {
                                    ixheaacd_res_ctns_apply(
                                        (*pstr_mps_state)
                                            .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize],
                                        (*(*pstr_mps_state)
                                            .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                                            .ics_info
                                            .max_sf_bands,
                                        aac_tables_ptr,
                                    );
                                }
                                win2 = (*(*pstr_mps_state)
                                    .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                                    .ics_info
                                    .window_sequence as WORD32;
                                i = 0 as core::ffi::c_int as WORD32;
                                while i < AAC_FRAME_LENGTH {
                                    let fresh6 = res_mdct_1;
                                    res_mdct_1 = res_mdct_1.offset(1);
                                    *fresh6 = *((*(*pstr_mps_state)
                                        .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                                        .p_spectral_coefficient)
                                        .offset(i as isize);
                                    i += 1;
                                }
                                if win1 != win2 {
                                    return IA_XHEAAC_MPS_DEC_EXE_FATAL_INVALID_WINDOW_SEQUENCE
                                        as IA_ERRORCODE;
                                }
                                if win1 == EIGHT_SHORT_SEQUENCE
                                    && (arbdmx_upd_qmf == UPD_QMF_18
                                        || arbdmx_upd_qmf == UPD_QMF_24
                                        || arbdmx_upd_qmf == UPD_QMF_30)
                                {
                                    temp_0 = ixheaacd_read_bits_buf(mps_bit_buf, 4 as WORD);
                                    temp_0 = ixheaacd_read_bits_buf(mps_bit_buf, 1 as WORD);
                                    if temp_0 != 0 as core::ffi::c_int {
                                        return IA_XHEAAC_MPS_DEC_EXE_FATAL_NONZERO_BIT
                                            as IA_ERRORCODE;
                                    }
                                    error_code = ixheaacd_res_read_ics(
                                        mps_bit_buf,
                                        ((*pstr_mps_state).p_aac_decoder_channel_info).as_mut_ptr(),
                                        1 as WORD32,
                                        aac_tables_ptr,
                                        ((*pstr_mps_state).tot_sf_bands_ls).as_mut_ptr(),
                                    ) as IA_ERRORCODE;
                                    if error_code != 0 {
                                        if (*pstr_mps_state).ec_flag != 0 {
                                            (*pstr_mps_state).frame_ok = 0 as core::ffi::c_int
                                                as WORD32;
                                        } else {
                                            return error_code
                                        }
                                    }
                                    if 1 as core::ffi::c_int
                                        == (*(*pstr_mps_state)
                                            .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                                            .tns_data
                                            .tns_data_present
                                    {
                                        ixheaacd_res_ctns_apply(
                                            (*pstr_mps_state)
                                                .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize],
                                            (*(*pstr_mps_state)
                                                .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                                                .ics_info
                                                .max_sf_bands,
                                            aac_tables_ptr,
                                        );
                                    }
                                    win1 = (*(*pstr_mps_state)
                                        .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                                        .ics_info
                                        .window_sequence as WORD32;
                                    i = 0 as core::ffi::c_int as WORD32;
                                    while i < AAC_FRAME_LENGTH {
                                        let fresh7 = p_mdct_res;
                                        p_mdct_res = p_mdct_res.offset(1);
                                        *fresh7 = *((*(*pstr_mps_state)
                                            .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                                            .p_spectral_coefficient)
                                            .offset(i as isize);
                                        i += 1;
                                    }
                                    error_code = ixheaacd_res_read_ics(
                                        mps_bit_buf,
                                        ((*pstr_mps_state).p_aac_decoder_channel_info).as_mut_ptr(),
                                        1 as WORD32,
                                        aac_tables_ptr,
                                        ((*pstr_mps_state).tot_sf_bands_ls).as_mut_ptr(),
                                    ) as IA_ERRORCODE;
                                    if error_code != 0 {
                                        if (*pstr_mps_state).ec_flag != 0 {
                                            (*pstr_mps_state).frame_ok = 0 as core::ffi::c_int
                                                as WORD32;
                                        } else {
                                            return error_code
                                        }
                                    }
                                    if 1 as core::ffi::c_int
                                        == (*(*pstr_mps_state)
                                            .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                                            .tns_data
                                            .tns_data_present
                                    {
                                        ixheaacd_res_ctns_apply(
                                            (*pstr_mps_state)
                                                .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize],
                                            (*(*pstr_mps_state)
                                                .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                                                .ics_info
                                                .max_sf_bands,
                                            aac_tables_ptr,
                                        );
                                    }
                                    win2 = (*(*pstr_mps_state)
                                        .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                                        .ics_info
                                        .window_sequence as WORD32;
                                    i = 0 as core::ffi::c_int as WORD32;
                                    while i < AAC_FRAME_LENGTH {
                                        let fresh8 = res_mdct_1;
                                        res_mdct_1 = res_mdct_1.offset(1);
                                        *fresh8 = *((*(*pstr_mps_state)
                                            .p_aac_decoder_channel_info[0 as core::ffi::c_int as usize])
                                            .p_spectral_coefficient)
                                            .offset(i as isize);
                                        i += 1;
                                    }
                                    if win1 != win2 {
                                        return IA_XHEAAC_MPS_DEC_EXE_FATAL_INVALID_WINDOW_SEQUENCE
                                            as IA_ERRORCODE;
                                    }
                                }
                                fr += 1;
                            }
                            p_res_mdct = p_res_mdct.offset(RFX2XMDCTCOEF as isize);
                        }
                        let fresh9 = gr;
                        gr = gr + 1;
                        ch += channel_grouping[fresh9 as usize];
                    }
                }
                2 => {
                    i = 0 as core::ffi::c_int as WORD32;
                    while i < (*p_bs_config).num_ott_boxes_at {
                        error_code = ixheaacd_ec_data_dec(
                            pstr_mps_state,
                            &mut (*frame).cld_lossless_data,
                            ((*frame).cmp_ott_cld_idx).as_mut_ptr(),
                            ((*frame).cmp_ott_cld_idx_prev).as_mut_ptr(),
                            CLD,
                            num_ott_boxes + i,
                            num_ott_boxes + i,
                            0 as WORD32,
                            (*p_bs_config).bs_ott_bands_at[i as usize],
                        );
                        if error_code != IA_NO_ERROR {
                            return error_code;
                        }
                        i += 1;
                    }
                }
                _ => return IA_XHEAAC_MPS_DEC_EXE_NONFATAL_INVALID_EXTENSION_TYPE,
            }
            bits_read = (((((*mps_bit_buf).ptr_read_next)
                .offset_from((*mps_bit_buf).ptr_bit_buf_base) as core::ffi::c_long
                + 1 as core::ffi::c_long) << 3 as core::ffi::c_int)
                - ((*mps_bit_buf).bit_pos as core::ffi::c_int + 1 as core::ffi::c_int)
                    as core::ffi::c_long - tmp as core::ffi::c_long) as WORD32;
            n_fill_bits = (sac_ext_len << 3 as core::ffi::c_int) - bits_read;
            while n_fill_bits > 7 as core::ffi::c_int {
                ixheaacd_read_bits_buf(mps_bit_buf, 8 as WORD);
                n_fill_bits -= 8 as core::ffi::c_int;
            }
            if n_fill_bits > 0 as core::ffi::c_int {
                ixheaacd_read_bits_buf(mps_bit_buf, n_fill_bits as WORD);
            }
        }
        ext_num += 1;
    }
    return IA_NO_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_parse_frame(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
) -> IA_ERRORCODE {
    let mut p_bs_config: *mut ia_mps_spatial_bs_config_struct = &mut (*pstr_mps_state)
        .bs_config;
    let mut p_aux_struct: *mut ia_mps_dec_auxilary_struct = (*pstr_mps_state).aux_struct;
    let mut i: WORD32 = 0;
    let mut bs_framing_type: WORD32 = 0;
    let mut prev_param_slot: WORD32 = 0;
    let mut data_bands: WORD32 = 0;
    let mut bs_temp_shape_enable: WORD32 = 0;
    let mut num_temp_shape_chan: WORD32 = 0;
    let mut ttt_off: WORD32 = 0;
    let mut ps: WORD32 = 0;
    let mut pg: WORD32 = 0;
    let mut ts: WORD32 = 0;
    let mut pb: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut bs_env_shape_data: *mut WORD32 = (*pstr_mps_state).mps_scratch_mem_v
        as *mut WORD32;
    let mut reciprocal_tab: *const WORD32 = ((*(*pstr_mps_state)
        .ia_mps_dec_mps_table
        .m1_m2_table_ptr)
        .reciprocal)
        .as_mut_ptr();
    let mut num_parameter_sets: WORD32 = 0;
    let mut frame: *mut ia_mps_dec_spatial_bs_frame_struct = (*pstr_mps_state).bs_frame;
    let mut bitdec_table: *mut ia_mps_dec_bitdec_tables_struct = (*pstr_mps_state)
        .ia_mps_dec_mps_table
        .bitdec_table_ptr;
    let mut bs_num_output_channels: WORD32 = (*bitdec_table)
        .tree_property_table[(*pstr_mps_state).tree_config as usize]
        .num_output_channels;
    let mut time_slots: WORD32 = (*pstr_mps_state).time_slots;
    let mut bitstream_parameter_bands: WORD32 = (*pstr_mps_state)
        .bitstream_parameter_bands;
    let mut b_ott_bands: *mut WORD32 = ((*pstr_mps_state).bitstream_ott_bands)
        .as_mut_ptr();
    let mut param_slot: *mut WORD32 = ((*(*pstr_mps_state).aux_struct).param_slot)
        .as_mut_ptr();
    let mut num_ott_boxes: WORD32 = (*pstr_mps_state).num_ott_boxes;
    let mut reciprocal: WORD32 = 0;
    let mut alignment_bits: WORD32 = 0 as WORD32;
    let mut mps_bit_buf: *mut ia_bit_buf_struct = (*pstr_mps_state).ptr_mps_bit_buff
        as *mut ia_bit_buf_struct;
    let mut error_code: IA_ERRORCODE = IA_NO_ERROR;
    if (*pstr_mps_state).parse_next_bitstream_frame == 0 as core::ffi::c_int {
        return IA_NO_ERROR;
    }
    temp = ixheaacd_read_bits_buf(mps_bit_buf, 4 as WORD);
    bs_framing_type = (temp as core::ffi::c_int >> 3 as core::ffi::c_int & ONE_BIT_MASK)
        as WORD32;
    num_parameter_sets = ((temp as core::ffi::c_int & THREE_BIT_MASK)
        + 1 as core::ffi::c_int) as WORD32;
    (*pstr_mps_state).num_parameter_sets = num_parameter_sets;
    reciprocal = *reciprocal_tab
        .offset(
            (num_parameter_sets as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
        );
    prev_param_slot = -(1 as core::ffi::c_int) as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_parameter_sets {
        if bs_framing_type != 0 {
            let mut bits_param_slot: WORD32 = 0 as WORD32;
            while (1 as core::ffi::c_int) << bits_param_slot
                < time_slots - num_parameter_sets + i - prev_param_slot
            {
                bits_param_slot += 1;
            }
            *param_slot.offset(i as isize) = (if bits_param_slot != 0 {
                prev_param_slot as core::ffi::c_int + 1 as core::ffi::c_int
                    + ixheaacd_read_bits_buf(mps_bit_buf, bits_param_slot as WORD)
                        as core::ffi::c_int
            } else {
                prev_param_slot as core::ffi::c_int + 1 as core::ffi::c_int
            }) as WORD32;
            prev_param_slot = *param_slot.offset(i as isize);
        } else {
            let mut temp_0: WORD64 = (time_slots as core::ffi::c_int
                * (i as core::ffi::c_int + 1 as core::ffi::c_int)
                + num_parameter_sets as core::ffi::c_int - 1 as core::ffi::c_int)
                as WORD64 * reciprocal as WORD64 >> 28 as core::ffi::c_int;
            *param_slot.offset(i as isize) = (temp_0 - 1 as WORD64) as WORD32;
        }
        i += 1;
    }
    (*frame).bs_independency_flag = ixheaacd_read_bits_buf(mps_bit_buf, 1 as WORD);
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_ott_boxes {
        error_code = ixheaacd_ec_data_dec(
            pstr_mps_state,
            &mut (*frame).cld_lossless_data,
            ((*frame).cmp_ott_cld_idx).as_mut_ptr(),
            ((*frame).cmp_ott_cld_idx_prev).as_mut_ptr(),
            CLD,
            i,
            i,
            0 as WORD32,
            *b_ott_bands.offset(i as isize),
        );
        if error_code != IA_NO_ERROR {
            return error_code;
        }
        i += 1;
    }
    if (*pstr_mps_state).one_icc != 0 {
        error_code = ixheaacd_ec_data_dec(
            pstr_mps_state,
            &mut (*frame).icc_lossless_data,
            ((*frame).cmp_ott_icc_idx).as_mut_ptr(),
            ((*frame).cmp_ott_icc_idx_prev).as_mut_ptr(),
            ICC,
            0 as WORD32,
            0 as WORD32,
            0 as WORD32,
            bitstream_parameter_bands,
        );
        if error_code != IA_NO_ERROR {
            return error_code;
        }
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i < num_ott_boxes {
            if (*pstr_mps_state).ott_mode_lfe[i as usize] == 0 {
                error_code = ixheaacd_ec_data_dec(
                    pstr_mps_state,
                    &mut (*frame).icc_lossless_data,
                    ((*frame).cmp_ott_icc_idx).as_mut_ptr(),
                    ((*frame).cmp_ott_icc_idx_prev).as_mut_ptr(),
                    ICC,
                    i,
                    i,
                    0 as WORD32,
                    *b_ott_bands.offset(i as isize),
                );
                if error_code != IA_NO_ERROR {
                    return error_code;
                }
            }
            i += 1;
        }
    }
    ttt_off = num_ott_boxes;
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*pstr_mps_state).num_ttt_boxes {
        if (*p_aux_struct).ttt_config[0 as core::ffi::c_int as usize][i as usize].mode
            < 2 as core::ffi::c_int
        {
            error_code = ixheaacd_ec_data_dec(
                pstr_mps_state,
                &mut (*frame).cpc_lossless_data,
                ((*frame).cmp_ttt_cpc_1_idx).as_mut_ptr(),
                ((*frame).cmp_ttt_cpc_1_idx_prev).as_mut_ptr(),
                CPC,
                i,
                ttt_off + 4 as WORD32 * i,
                (*p_aux_struct)
                    .ttt_config[0 as core::ffi::c_int as usize][i as usize]
                    .bitstream_start_band,
                (*p_aux_struct)
                    .ttt_config[0 as core::ffi::c_int as usize][i as usize]
                    .bitstream_stop_band,
            );
            if error_code != IA_NO_ERROR {
                return error_code;
            }
            error_code = ixheaacd_ec_data_dec(
                pstr_mps_state,
                &mut (*frame).cpc_lossless_data,
                ((*frame).cmp_ttt_cpc_2_idx).as_mut_ptr(),
                ((*frame).cmp_ttt_cpc_2_idx_prev).as_mut_ptr(),
                CPC,
                i,
                ttt_off + 4 as WORD32 * i + 1 as WORD32,
                (*p_aux_struct)
                    .ttt_config[0 as core::ffi::c_int as usize][i as usize]
                    .bitstream_start_band,
                (*p_aux_struct)
                    .ttt_config[0 as core::ffi::c_int as usize][i as usize]
                    .bitstream_stop_band,
            );
            if error_code != IA_NO_ERROR {
                return error_code;
            }
            error_code = ixheaacd_ec_data_dec(
                pstr_mps_state,
                &mut (*frame).icc_lossless_data,
                ((*frame).cmp_ttt_icc_idx).as_mut_ptr(),
                ((*frame).cmp_ttt_icc_idx_prev).as_mut_ptr(),
                ICC,
                i,
                ttt_off + 4 as WORD32 * i,
                (*p_aux_struct)
                    .ttt_config[0 as core::ffi::c_int as usize][i as usize]
                    .bitstream_start_band,
                (*p_aux_struct)
                    .ttt_config[0 as core::ffi::c_int as usize][i as usize]
                    .bitstream_stop_band,
            );
            if error_code != IA_NO_ERROR {
                return error_code;
            }
        } else {
            error_code = ixheaacd_ec_data_dec(
                pstr_mps_state,
                &mut (*frame).cld_lossless_data,
                ((*frame).cmp_ttt_cld_1_idx).as_mut_ptr(),
                ((*frame).cmp_ttt_cld_1_idx_prev).as_mut_ptr(),
                CLD,
                i,
                ttt_off + 4 as WORD32 * i,
                (*p_aux_struct)
                    .ttt_config[0 as core::ffi::c_int as usize][i as usize]
                    .bitstream_start_band,
                (*p_aux_struct)
                    .ttt_config[0 as core::ffi::c_int as usize][i as usize]
                    .bitstream_stop_band,
            );
            if error_code != IA_NO_ERROR {
                return error_code;
            }
            error_code = ixheaacd_ec_data_dec(
                pstr_mps_state,
                &mut (*frame).cld_lossless_data,
                ((*frame).cmp_ttt_cld_2_idx).as_mut_ptr(),
                ((*frame).cmp_ttt_cld_2_idx_prev).as_mut_ptr(),
                CLD,
                i,
                ttt_off + 4 as WORD32 * i + 1 as WORD32,
                (*p_aux_struct)
                    .ttt_config[0 as core::ffi::c_int as usize][i as usize]
                    .bitstream_start_band,
                (*p_aux_struct)
                    .ttt_config[0 as core::ffi::c_int as usize][i as usize]
                    .bitstream_stop_band,
            );
            if error_code != IA_NO_ERROR {
                return error_code;
            }
        }
        if (*p_aux_struct)
            .ttt_config[1 as core::ffi::c_int as usize][i as usize]
            .bitstream_start_band
            < (*p_aux_struct)
                .ttt_config[1 as core::ffi::c_int as usize][i as usize]
                .bitstream_stop_band
        {
            if (*p_aux_struct)
                .ttt_config[1 as core::ffi::c_int as usize][i as usize]
                .mode < 2 as core::ffi::c_int
            {
                error_code = ixheaacd_ec_data_dec(
                    pstr_mps_state,
                    &mut (*frame).cpc_lossless_data,
                    ((*frame).cmp_ttt_cpc_1_idx).as_mut_ptr(),
                    ((*frame).cmp_ttt_cpc_1_idx_prev).as_mut_ptr(),
                    CPC,
                    i,
                    ttt_off + 4 as WORD32 * i + 2 as WORD32,
                    (*p_aux_struct)
                        .ttt_config[1 as core::ffi::c_int as usize][i as usize]
                        .bitstream_start_band,
                    (*p_aux_struct)
                        .ttt_config[1 as core::ffi::c_int as usize][i as usize]
                        .bitstream_stop_band,
                );
                if error_code != IA_NO_ERROR {
                    return error_code;
                }
                error_code = ixheaacd_ec_data_dec(
                    pstr_mps_state,
                    &mut (*frame).cpc_lossless_data,
                    ((*frame).cmp_ttt_cpc_2_idx).as_mut_ptr(),
                    ((*frame).cmp_ttt_cpc_2_idx_prev).as_mut_ptr(),
                    CPC,
                    i,
                    ttt_off + 4 as WORD32 * i + 3 as WORD32,
                    (*p_aux_struct)
                        .ttt_config[1 as core::ffi::c_int as usize][i as usize]
                        .bitstream_start_band,
                    (*p_aux_struct)
                        .ttt_config[1 as core::ffi::c_int as usize][i as usize]
                        .bitstream_stop_band,
                );
                if error_code != IA_NO_ERROR {
                    return error_code;
                }
                error_code = ixheaacd_ec_data_dec(
                    pstr_mps_state,
                    &mut (*frame).icc_lossless_data,
                    ((*frame).cmp_ttt_icc_idx).as_mut_ptr(),
                    ((*frame).cmp_ttt_icc_idx_prev).as_mut_ptr(),
                    ICC,
                    i,
                    ttt_off + 4 as WORD32 * i + 2 as WORD32,
                    (*p_aux_struct)
                        .ttt_config[1 as core::ffi::c_int as usize][i as usize]
                        .bitstream_start_band,
                    (*p_aux_struct)
                        .ttt_config[1 as core::ffi::c_int as usize][i as usize]
                        .bitstream_stop_band,
                );
                if error_code != IA_NO_ERROR {
                    return error_code;
                }
            } else {
                error_code = ixheaacd_ec_data_dec(
                    pstr_mps_state,
                    &mut (*frame).cld_lossless_data,
                    ((*frame).cmp_ttt_cld_1_idx).as_mut_ptr(),
                    ((*frame).cmp_ttt_cld_1_idx_prev).as_mut_ptr(),
                    CLD,
                    i,
                    ttt_off + 4 as WORD32 * i + 2 as WORD32,
                    (*p_aux_struct)
                        .ttt_config[1 as core::ffi::c_int as usize][i as usize]
                        .bitstream_start_band,
                    (*p_aux_struct)
                        .ttt_config[1 as core::ffi::c_int as usize][i as usize]
                        .bitstream_stop_band,
                );
                if error_code != IA_NO_ERROR {
                    return error_code;
                }
                error_code = ixheaacd_ec_data_dec(
                    pstr_mps_state,
                    &mut (*frame).cld_lossless_data,
                    ((*frame).cmp_ttt_cld_2_idx).as_mut_ptr(),
                    ((*frame).cmp_ttt_cld_2_idx_prev).as_mut_ptr(),
                    CLD,
                    i,
                    ttt_off + 4 as WORD32 * i + 3 as WORD32,
                    (*p_aux_struct)
                        .ttt_config[1 as core::ffi::c_int as usize][i as usize]
                        .bitstream_start_band,
                    (*p_aux_struct)
                        .ttt_config[1 as core::ffi::c_int as usize][i as usize]
                        .bitstream_stop_band,
                );
                if error_code != IA_NO_ERROR {
                    return error_code;
                }
            }
        }
        i += 1;
    }
    (*frame).bs_smooth_control = 1 as core::ffi::c_int as WORD32;
    if (*frame).bs_smooth_control != 0 {
        ps = 0 as core::ffi::c_int as WORD32;
        while ps < num_parameter_sets {
            (*frame).bs_smooth_mode[ps as usize] = ixheaacd_read_bits_buf(
                mps_bit_buf,
                2 as WORD,
            );
            if (*frame).bs_smooth_mode[ps as usize] > 3 as core::ffi::c_int
                || (*frame).bs_smooth_mode[ps as usize] < 0 as core::ffi::c_int
            {
                return IA_XHEAAC_MPS_DEC_EXE_NONFATAL_INVALID_SMOOTH_MODE;
            }
            if (*frame).bs_smooth_mode[ps as usize] >= 2 as core::ffi::c_int {
                (*frame).bs_smooth_time[ps as usize] = ixheaacd_read_bits_buf(
                    mps_bit_buf,
                    2 as WORD,
                );
            }
            if (*frame).bs_smooth_mode[ps as usize] == 3 as core::ffi::c_int {
                (*frame).bs_freq_res_stride_smg[ps as usize] = ixheaacd_read_bits_buf(
                    mps_bit_buf,
                    2 as WORD,
                );
                data_bands = ((bitstream_parameter_bands as core::ffi::c_int
                    - 1 as core::ffi::c_int)
                    / (*bitdec_table)
                        .pb_stride_table[(*frame).bs_freq_res_stride_smg[ps as usize]
                        as usize] + 1 as core::ffi::c_int) as WORD32;
                pg = 0 as core::ffi::c_int as WORD32;
                while pg < data_bands {
                    (*frame).bs_smg_data[ps as usize][pg as usize] = ixheaacd_read_bits_buf(
                        mps_bit_buf,
                        1 as WORD,
                    );
                    pg += 1;
                }
            }
            ps += 1;
        }
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < bs_num_output_channels {
        (*p_aux_struct).temp_shape_enable_channel_stp[i as usize] = 0 as core::ffi::c_int
            as WORD32;
        (*p_aux_struct).temp_shape_enable_channel_ges[i as usize] = 0 as core::ffi::c_int
            as WORD32;
        i += 1;
    }
    if (*p_bs_config).bs_temp_shape_config != 0 as core::ffi::c_int {
        bs_temp_shape_enable = ixheaacd_read_bits_buf(mps_bit_buf, 1 as WORD);
        if bs_temp_shape_enable != 0 {
            num_temp_shape_chan = (*bitdec_table)
                .temp_shape_chan_table[((*p_bs_config).bs_temp_shape_config
                as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize][(*p_bs_config).bs_tree_config as usize];
            match (*pstr_mps_state).temp_shape_config {
                1 => {
                    i = 0 as core::ffi::c_int as WORD32;
                    while i < num_temp_shape_chan {
                        (*p_aux_struct).temp_shape_enable_channel_stp[i as usize] = ixheaacd_read_bits_buf(
                            mps_bit_buf,
                            1 as WORD,
                        );
                        i += 1;
                    }
                }
                2 => {
                    i = 0 as core::ffi::c_int as WORD32;
                    while i < num_temp_shape_chan {
                        (*p_aux_struct).temp_shape_enable_channel_ges[i as usize] = ixheaacd_read_bits_buf(
                            mps_bit_buf,
                            1 as WORD,
                        );
                        i += 1;
                    }
                    i = 0 as core::ffi::c_int as WORD32;
                    while i < num_temp_shape_chan {
                        if (*p_aux_struct).temp_shape_enable_channel_ges[i as usize] != 0
                        {
                            let mut envshape_data: *const WORD32 = &mut *(*((*bitdec_table)
                                .envshape_data)
                                .as_mut_ptr()
                                .offset((*pstr_mps_state).env_quant_mode as isize))
                                .as_mut_ptr()
                                .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
                            ixheaacd_mps_huff_decode(
                                mps_bit_buf,
                                bs_env_shape_data,
                                time_slots,
                            );
                            ts = 0 as core::ffi::c_int as WORD32;
                            while ts < time_slots {
                                (*p_aux_struct).env_shape_data[i as usize][ts as usize] = *envshape_data
                                    .offset(*bs_env_shape_data.offset(ts as isize) as isize);
                                ts += 1;
                            }
                        }
                        i += 1;
                    }
                }
                _ => {
                    return IA_XHEAAC_MPS_DEC_EXE_FATAL_INVALID_TEMPORAL_SHAPING_CONFIG
                        as IA_ERRORCODE;
                }
            }
        }
    }
    if (*pstr_mps_state).up_mix_type == 2 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < bs_num_output_channels {
            (*p_aux_struct).temp_shape_enable_channel_stp[i as usize] = 0
                as core::ffi::c_int as WORD32;
            (*p_aux_struct).temp_shape_enable_channel_ges[i as usize] = 0
                as core::ffi::c_int as WORD32;
            i += 1;
        }
    }
    if (*pstr_mps_state).arbitrary_downmix != 0 as core::ffi::c_int {
        ixheaacd_parse_arbitrary_downmix_data(pstr_mps_state);
    }
    ixheaacd_byte_align(mps_bit_buf, &mut alignment_bits);
    error_code = ixheaacd_parse_extension_frame(pstr_mps_state);
    if error_code != 0 {
        if (*pstr_mps_state).ec_flag != 0 {
            (*pstr_mps_state).frame_ok = 0 as core::ffi::c_int as WORD32;
        } else {
            return error_code
        }
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_ott_boxes {
        ps = 0 as core::ffi::c_int as WORD32;
        while ps < num_parameter_sets {
            if (*frame).res_data.bs_icc_diff_present[i as usize][ps as usize] == 0
                || (*pstr_mps_state).up_mix_type == 2 as core::ffi::c_int
                || (*pstr_mps_state).up_mix_type == 3 as core::ffi::c_int
            {
                pb = 0 as core::ffi::c_int as WORD32;
                while pb < bitstream_parameter_bands {
                    (*(*pstr_mps_state).bs_frame)
                        .ott_icc_diff_idx[i as usize][ps as usize][pb as usize] = 0
                        as core::ffi::c_int as WORD32;
                    pb += 1;
                }
            }
            ps += 1;
        }
        i += 1;
    }
    (*pstr_mps_state).parse_next_bitstream_frame = 1 as core::ffi::c_int as WORD32;
    return IA_NO_ERROR;
}
unsafe extern "C" fn ixheaacd_create_mapping(
    mut a_map: *mut WORD32,
    mut start_band: WORD32,
    mut stop_band: WORD32,
    mut stride: WORD32,
    mut scratch: *mut core::ffi::c_void,
) -> VOID {
    let mut in_bands: WORD32 = 0;
    let mut out_bands: WORD32 = 0;
    let mut bands_achived: WORD32 = 0;
    let mut bands_diff: WORD32 = 0;
    let mut incr: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut v_dk: *mut WORD32 = 0 as *mut WORD32;
    in_bands = stop_band - start_band;
    out_bands = ((in_bands as core::ffi::c_int - 1 as core::ffi::c_int)
        / stride as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
    v_dk = scratch as *mut WORD32;
    if out_bands < 1 as core::ffi::c_int {
        out_bands = 1 as core::ffi::c_int as WORD32;
    }
    bands_achived = out_bands * stride;
    bands_diff = in_bands - bands_achived;
    i = 0 as core::ffi::c_int as WORD32;
    while i < out_bands {
        *v_dk.offset(i as isize) = stride;
        i += 1;
    }
    if bands_diff > 0 as core::ffi::c_int {
        incr = -(1 as core::ffi::c_int) as WORD32;
        k = (out_bands as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    } else {
        incr = 1 as core::ffi::c_int as WORD32;
        k = 0 as core::ffi::c_int as WORD32;
    }
    while bands_diff != 0 as core::ffi::c_int {
        *v_dk.offset(k as isize) = *v_dk.offset(k as isize) - incr;
        k = k + incr;
        bands_diff = bands_diff + incr;
        if k >= out_bands {
            if bands_diff > 0 as core::ffi::c_int {
                k = (out_bands as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
            } else if bands_diff < 0 as core::ffi::c_int {
                k = 0 as core::ffi::c_int as WORD32;
            }
        }
    }
    *a_map.offset(0 as core::ffi::c_int as isize) = start_band;
    i = 0 as core::ffi::c_int as WORD32;
    while i < out_bands {
        *a_map.offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = *a_map
            .offset(i as isize) + *v_dk.offset(i as isize);
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_map_frequency(
    mut p_input: *mut WORD32,
    mut p_output: *mut WORD32,
    mut p_map: *mut WORD32,
    mut data_bands: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut start_band: WORD32 = 0;
    let mut stop_band: WORD32 = 0;
    let mut value: WORD32 = 0;
    let mut start_band_0: WORD32 = *p_map.offset(0 as core::ffi::c_int as isize);
    i = 0 as core::ffi::c_int as WORD32;
    while i < data_bands {
        value = *p_input.offset((i + start_band_0) as isize);
        start_band = *p_map.offset(i as isize);
        stop_band = *p_map
            .offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
        j = start_band;
        while j < stop_band {
            *p_output.offset(j as isize) = value;
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_deq_coarse(
    mut value: WORD32,
    mut param_type: WORD32,
    mut dequant: *mut WORD32,
    mut ixheaacd_mps_dec_bitdec_tables: *mut ia_mps_dec_bitdec_tables_struct,
) -> IA_ERRORCODE {
    match param_type {
        CLD => {
            if value >= 8 as core::ffi::c_int || value < -(7 as core::ffi::c_int) {
                return IA_FATAL_ERROR as IA_ERRORCODE;
            }
            *dequant = (*ixheaacd_mps_dec_bitdec_tables)
                .dequant_cld_coarse[(value as core::ffi::c_int + 7 as core::ffi::c_int)
                as usize];
        }
        ICC => {
            if value >= 8 as core::ffi::c_int || value < 0 as core::ffi::c_int {
                return IA_FATAL_ERROR as IA_ERRORCODE;
            }
            *dequant = (*ixheaacd_mps_dec_bitdec_tables).dequant_icc[value as usize];
        }
        CPC => {
            if value >= 16 as core::ffi::c_int || value < -(10 as core::ffi::c_int) {
                return IA_FATAL_ERROR as IA_ERRORCODE;
            }
            *dequant = (*ixheaacd_mps_dec_bitdec_tables)
                .dequant_cpc_coarse[(value as core::ffi::c_int + 10 as core::ffi::c_int)
                as usize];
        }
        _ => return IA_XHEAAC_MPS_DEC_EXE_FATAL_INVALID_DEQUANT_PARAM as IA_ERRORCODE,
    }
    return IA_NO_ERROR;
}
unsafe extern "C" fn ia_mps_dec_deq(
    mut value: WORD32,
    mut param_type: WORD32,
    mut dequant: *mut WORD32,
    mut ixheaacd_mps_dec_bitdec_tables: *mut ia_mps_dec_bitdec_tables_struct,
) -> IA_ERRORCODE {
    match param_type {
        CLD => {
            if value >= 16 as core::ffi::c_int || value < -(15 as core::ffi::c_int) {
                return IA_FATAL_ERROR as IA_ERRORCODE;
            }
            *dequant = (*ixheaacd_mps_dec_bitdec_tables)
                .dequant_cld[(value as core::ffi::c_int + 15 as core::ffi::c_int)
                as usize];
        }
        ICC => {
            if value >= 8 as core::ffi::c_int || value < 0 as core::ffi::c_int {
                return IA_FATAL_ERROR as IA_ERRORCODE;
            }
            *dequant = (*ixheaacd_mps_dec_bitdec_tables).dequant_icc[value as usize];
        }
        CPC => {
            if value >= 32 as core::ffi::c_int || value < -(20 as core::ffi::c_int) {
                return IA_FATAL_ERROR as IA_ERRORCODE;
            }
            *dequant = (*ixheaacd_mps_dec_bitdec_tables)
                .dequant_cpc[(value as core::ffi::c_int + 20 as core::ffi::c_int)
                as usize];
        }
        _ => return IA_XHEAAC_MPS_DEC_EXE_FATAL_INVALID_DEQUANT_PARAM as IA_ERRORCODE,
    }
    return IA_NO_ERROR;
}
unsafe extern "C" fn ixheaacd_factor_funct(
    mut ott_vs_tot_db: WORD32,
    mut quant_mode: WORD32,
    mut factor: *mut WORD32,
) -> IA_ERRORCODE {
    let mut db_diff: WORD32 = 0;
    let mut x_linear: WORD32 = 0 as WORD32;
    let mut maxfactor: WORD32 = 0 as WORD32;
    let mut constfact: WORD32 = 0;
    if ott_vs_tot_db > 0 as core::ffi::c_int {
        return IA_XHEAAC_MPS_DEC_EXE_FATAL_INVALID_MPS_PARAM as IA_ERRORCODE;
    }
    db_diff = -ott_vs_tot_db;
    match quant_mode {
        0 => return 16777216 as IA_ERRORCODE,
        1 => {
            x_linear = 1024 as core::ffi::c_int as WORD32;
            maxfactor = 83886080 as core::ffi::c_int as WORD32;
            constfact = 3277 as core::ffi::c_int as WORD32;
        }
        2 => {
            x_linear = 1024 as core::ffi::c_int as WORD32;
            maxfactor = 134217728 as core::ffi::c_int as WORD32;
            constfact = 4779 as core::ffi::c_int as WORD32;
        }
        _ => return IA_XHEAAC_MPS_DEC_EXE_FATAL_INVALID_QUANT_MODE as IA_ERRORCODE,
    }
    if db_diff > x_linear << 5 as core::ffi::c_int {
        let mut db_diff_fix: WORD32 = db_diff >> 5 as core::ffi::c_int;
        *factor = ((db_diff_fix as core::ffi::c_int - x_linear)
            * constfact as core::ffi::c_int + ONE_IN_Q24) as WORD32;
    } else {
        *factor = ONE_IN_Q24 as WORD32;
    }
    *factor = if maxfactor < *factor { maxfactor } else { *factor };
    return IA_NO_ERROR;
}
unsafe extern "C" fn ixheaacd_factor_cld(
    mut idx: *mut WORD32,
    mut ott_vs_tot_db: WORD32,
    mut ott_vs_tot_db_1: *mut WORD32,
    mut ott_vs_tot_db_2: *mut WORD32,
    mut quant_mode: WORD32,
    mut ixheaacd_mps_dec_bitdec_tables: *mut ia_mps_dec_bitdec_tables_struct,
) -> VOID {
    let mut factor: WORD32 = 0 as WORD32;
    let mut c1: WORD32 = 0;
    let mut c2: WORD32 = 0;
    let mut cld_idx: WORD32 = 0;
    ixheaacd_factor_funct(ott_vs_tot_db, quant_mode, &mut factor);
    cld_idx = ixheaac_mul32_sh(*idx, factor, 23 as WORD8);
    cld_idx = ixheaac_shr32(ixheaac_add32(cld_idx, 1 as WORD32), 1 as WORD);
    cld_idx = (if cld_idx < 15 as core::ffi::c_int {
        cld_idx as core::ffi::c_int
    } else {
        15 as core::ffi::c_int
    }) as WORD32;
    cld_idx = (if cld_idx > -(15 as core::ffi::c_int) {
        cld_idx as core::ffi::c_int
    } else {
        -(15 as core::ffi::c_int)
    }) as WORD32;
    *idx = cld_idx;
    c1 = (*ixheaacd_mps_dec_bitdec_tables)
        .factor_cld_tab_1[(*idx + 15 as core::ffi::c_int) as usize];
    c2 = (*ixheaacd_mps_dec_bitdec_tables)
        .factor_cld_tab_1[(15 as WORD32 - *idx) as usize];
    *ott_vs_tot_db_1 = c1 + ott_vs_tot_db;
    *ott_vs_tot_db_2 = c2 + ott_vs_tot_db;
}
unsafe extern "C" fn ixheaacd_map_index_data(
    mut ll_data: *mut ia_mps_dec_lossless_data_struct,
    mut output_data: *mut [[WORD32; 28]; 8],
    mut output_idx_data: *mut [[WORD32; 28]; 8],
    mut cmp_idx_data: *mut [[WORD32; 28]; 8],
    mut diff_idx_data: *mut [[WORD32; 28]; 8],
    mut xtt_idx: WORD32,
    mut idx_prev: *mut [WORD32; 28],
    mut param_idx: WORD32,
    mut param_type: WORD32,
    mut start_band: WORD32,
    mut stop_band: WORD32,
    mut default_value: WORD32,
    mut num_parameter_sets: WORD32,
    mut param_slot: *mut WORD32,
    mut extend_frame: WORD32,
    mut quant_mode: WORD32,
    mut ott_vs_tot_db_in: *mut WORD32,
    mut ott_vs_tot_db_1: *mut WORD32,
    mut ott_vs_tot_db_2: *mut WORD32,
    mut ixheaacd_mps_dec_bitdec_tables: *mut ia_mps_dec_bitdec_tables_struct,
    mut scratch: *mut core::ffi::c_void,
) -> IA_ERRORCODE {
    let mut a_param_slots: *mut WORD32 = 0 as *mut WORD32;
    let mut a_interpolate: *mut WORD32 = 0 as *mut WORD32;
    let mut data_sets: WORD32 = 0;
    let mut a_map: *mut WORD32 = 0 as *mut WORD32;
    let mut free_scratch: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut set_idx: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut band: WORD32 = 0;
    let mut parm_slot: WORD32 = 0;
    let mut data_bands: WORD32 = 0;
    let mut stride: WORD32 = 0;
    let mut ps: WORD32 = 0;
    let mut pb: WORD32 = 0;
    let mut i1: WORD32 = 0;
    let mut i2: WORD32 = 0;
    let mut x1: WORD32 = 0;
    let mut xi: WORD32 = 0;
    let mut x2: WORD32 = 0;
    let mut db_in: *mut WORD32 = 0 as *mut WORD32;
    let mut db_1: *mut WORD32 = 0 as *mut WORD32;
    let mut db_2: *mut WORD32 = 0 as *mut WORD32;
    let mut error_code: IA_ERRORCODE = IA_NO_ERROR;
    db_in = ott_vs_tot_db_in;
    db_1 = ott_vs_tot_db_1;
    db_2 = ott_vs_tot_db_2;
    a_param_slots = scratch as *mut WORD32;
    a_interpolate = a_param_slots
        .offset(
            ((8 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    a_map = a_interpolate
        .offset(
            ((8 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    free_scratch = a_map
        .offset(
            ((29 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        ) as *mut core::ffi::c_void;
    data_sets = 0 as core::ffi::c_int as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_parameter_sets {
        if (*ll_data).bs_xxx_data_mode[param_idx as usize][i as usize]
            == 3 as core::ffi::c_int
        {
            *a_param_slots.offset(data_sets as isize) = i;
            data_sets += 1;
        }
        i += 1;
    }
    set_idx = 0 as core::ffi::c_int as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_parameter_sets {
        if (*ll_data).bs_xxx_data_mode[param_idx as usize][i as usize]
            == 0 as core::ffi::c_int
        {
            (*ll_data).no_cmp_quant_coarse_xxx[param_idx as usize][i as usize] = 0
                as core::ffi::c_int as WORD32;
            band = start_band;
            while band < stop_band {
                (*output_idx_data.offset(xtt_idx as isize))[i as usize][band as usize] = default_value;
                band += 1;
            }
            band = start_band;
            while band < stop_band {
                (*idx_prev.offset(xtt_idx as isize))[band as usize] = (*output_idx_data
                    .offset(xtt_idx as isize))[i as usize][band as usize];
                band += 1;
            }
        }
        if (*ll_data).bs_xxx_data_mode[param_idx as usize][i as usize]
            == 1 as core::ffi::c_int
        {
            band = start_band;
            while band < stop_band {
                (*output_idx_data.offset(xtt_idx as isize))[i as usize][band as usize] = (*idx_prev
                    .offset(xtt_idx as isize))[band as usize];
                band += 1;
            }
            (*ll_data).no_cmp_quant_coarse_xxx[param_idx as usize][i as usize] = (*ll_data)
                .bs_quant_coarse_xxx_prev[param_idx as usize];
        }
        if (*ll_data).bs_xxx_data_mode[param_idx as usize][i as usize]
            == 2 as core::ffi::c_int
        {
            band = start_band;
            while band < stop_band {
                (*output_idx_data.offset(xtt_idx as isize))[i as usize][band as usize] = (*idx_prev
                    .offset(xtt_idx as isize))[band as usize];
                band += 1;
            }
            *a_interpolate.offset(i as isize) = 1 as core::ffi::c_int as WORD32;
        } else {
            *a_interpolate.offset(i as isize) = 0 as core::ffi::c_int as WORD32;
        }
        if (*ll_data).bs_xxx_data_mode[param_idx as usize][i as usize]
            == 3 as core::ffi::c_int
        {
            parm_slot = *a_param_slots.offset(set_idx as isize);
            stride = (*ixheaacd_mps_dec_bitdec_tables)
                .pb_stride_table[(*ll_data)
                .bs_freq_res_stride_xxx[param_idx as usize][set_idx as usize] as usize];
            data_bands = ((stop_band as core::ffi::c_int - start_band as core::ffi::c_int
                - 1 as core::ffi::c_int) / stride as core::ffi::c_int
                + 1 as core::ffi::c_int) as WORD32;
            ixheaacd_create_mapping(
                a_map as *mut WORD32,
                start_band,
                stop_band,
                stride,
                free_scratch,
            );
            ixheaacd_map_frequency(
                &mut *(*(*cmp_idx_data.offset(xtt_idx as isize))
                    .as_mut_ptr()
                    .offset(set_idx as isize))
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize),
                &mut *(*(*output_idx_data.offset(xtt_idx as isize))
                    .as_mut_ptr()
                    .offset(parm_slot as isize))
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize),
                a_map,
                data_bands,
            );
            band = start_band;
            while band < stop_band {
                (*idx_prev.offset(xtt_idx as isize))[band as usize] = (*output_idx_data
                    .offset(xtt_idx as isize))[parm_slot as usize][band as usize];
                band += 1;
            }
            (*ll_data).bs_quant_coarse_xxx_prev[param_idx as usize] = (*ll_data)
                .bs_quant_coarse_xxx[param_idx as usize][set_idx as usize];
            (*ll_data).no_cmp_quant_coarse_xxx[param_idx as usize][i as usize] = (*ll_data)
                .bs_quant_coarse_xxx[param_idx as usize][set_idx as usize];
            set_idx += 1;
        }
        if !diff_idx_data.is_null() {
            band = start_band;
            while band < stop_band {
                (*output_idx_data.offset(xtt_idx as isize))[i as usize][band as usize]
                    += (*diff_idx_data
                        .offset(xtt_idx as isize))[i as usize][band as usize];
                band += 1;
            }
        }
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_parameter_sets {
        if *a_interpolate.offset(i as isize) != 1 as core::ffi::c_int {
            if (*ll_data).no_cmp_quant_coarse_xxx[param_idx as usize][i as usize]
                == 1 as core::ffi::c_int
            {
                band = start_band;
                while band < stop_band {
                    error_code = ixheaacd_deq_coarse(
                        (*output_idx_data
                            .offset(xtt_idx as isize))[i as usize][band as usize],
                        param_type,
                        &mut *(*(*output_data.offset(xtt_idx as isize))
                            .as_mut_ptr()
                            .offset(i as isize))
                            .as_mut_ptr()
                            .offset(band as isize),
                        ixheaacd_mps_dec_bitdec_tables,
                    );
                    if error_code != 0 {
                        return error_code;
                    }
                    band += 1;
                }
            } else {
                band = start_band;
                while band < stop_band {
                    error_code = ia_mps_dec_deq(
                        (*output_idx_data
                            .offset(xtt_idx as isize))[i as usize][band as usize],
                        param_type,
                        &mut *(*(*output_data.offset(xtt_idx as isize))
                            .as_mut_ptr()
                            .offset(i as isize))
                            .as_mut_ptr()
                            .offset(band as isize),
                        ixheaacd_mps_dec_bitdec_tables,
                    );
                    if error_code != 0 {
                        return error_code;
                    }
                    band += 1;
                }
            }
        }
        i += 1;
    }
    if quant_mode != 0 && param_type == CLD {
        if db_in.is_null() || db_1.is_null() || db_2.is_null() {
            return IA_XHEAAC_MPS_DEC_EXE_FATAL_INVALID_MPS_PARAM as IA_ERRORCODE;
        }
        ps = 0 as core::ffi::c_int as WORD32;
        while ps < num_parameter_sets {
            if *a_interpolate.offset(ps as isize) != 1 as core::ffi::c_int {
                if (*ll_data).no_cmp_quant_coarse_xxx[param_idx as usize][ps as usize]
                    != 0
                {
                    ixheaacd_coarse_2_fine(
                        ((*output_idx_data.offset(xtt_idx as isize))[ps as usize])
                            .as_mut_ptr(),
                        param_type,
                        start_band,
                        stop_band - start_band,
                    );
                }
                pb = start_band;
                while pb < stop_band {
                    (*ll_data)
                        .no_cmp_quant_coarse_xxx[param_idx as usize][ps as usize] = 1
                        as core::ffi::c_int as WORD32;
                    let fresh14 = db_in;
                    db_in = db_in.offset(1);
                    let fresh15 = db_1;
                    db_1 = db_1.offset(1);
                    let fresh16 = db_2;
                    db_2 = db_2.offset(1);
                    ixheaacd_factor_cld(
                        &mut *(*(*output_idx_data.offset(xtt_idx as isize))
                            .as_mut_ptr()
                            .offset(ps as isize))
                            .as_mut_ptr()
                            .offset(pb as isize),
                        *fresh14,
                        fresh15,
                        fresh16,
                        quant_mode,
                        ixheaacd_mps_dec_bitdec_tables,
                    );
                    ia_mps_dec_deq(
                        (*output_idx_data
                            .offset(xtt_idx as isize))[ps as usize][pb as usize],
                        param_type,
                        &mut *(*(*output_data.offset(xtt_idx as isize))
                            .as_mut_ptr()
                            .offset(ps as isize))
                            .as_mut_ptr()
                            .offset(pb as isize),
                        ixheaacd_mps_dec_bitdec_tables,
                    );
                    pb += 1;
                }
            }
            ps += 1;
        }
    }
    i1 = 0 as core::ffi::c_int as WORD32;
    x1 = 0 as core::ffi::c_int as WORD32;
    i2 = 0 as core::ffi::c_int as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_parameter_sets {
        if *a_interpolate.offset(i as isize) != 1 as core::ffi::c_int {
            i1 = i;
        }
        i2 = i;
        while *a_interpolate.offset(i2 as isize) == 1 as core::ffi::c_int {
            i2 += 1;
        }
        x1 = *param_slot.offset(i1 as isize);
        xi = *param_slot.offset(i as isize);
        x2 = *param_slot.offset(i2 as isize);
        if *a_interpolate.offset(i as isize) == 1 as core::ffi::c_int {
            if i2 >= num_parameter_sets {
                return IA_XHEAAC_MPS_DEC_EXE_FATAL_INVALID_PARAMETER_SETS
                    as IA_ERRORCODE;
            }
            if (*ll_data).no_cmp_quant_coarse_xxx[param_idx as usize][i1 as usize] != 0 {
                ixheaacd_coarse_2_fine(
                    ((*output_idx_data.offset(xtt_idx as isize))[i1 as usize])
                        .as_mut_ptr(),
                    param_type,
                    start_band,
                    stop_band - start_band,
                );
            }
            if (*ll_data).no_cmp_quant_coarse_xxx[param_idx as usize][i2 as usize] != 0 {
                ixheaacd_coarse_2_fine(
                    ((*output_idx_data.offset(xtt_idx as isize))[i2 as usize])
                        .as_mut_ptr(),
                    param_type,
                    start_band,
                    stop_band - start_band,
                );
            }
            band = start_band;
            while band < stop_band {
                let mut yi: WORD32 = 0 as WORD32;
                let mut y1: WORD32 = 0;
                let mut y2: WORD32 = 0;
                y1 = (*output_idx_data
                    .offset(xtt_idx as isize))[i1 as usize][band as usize];
                y2 = (*output_idx_data
                    .offset(xtt_idx as isize))[i2 as usize][band as usize];
                if x2 != x1 {
                    yi = y1 + (xi - x1) * (y2 - y1) / (x2 - x1);
                }
                (*output_idx_data.offset(xtt_idx as isize))[i as usize][band as usize] = yi;
                ia_mps_dec_deq(
                    (*output_idx_data
                        .offset(xtt_idx as isize))[i as usize][band as usize],
                    param_type,
                    &mut *(*(*output_data.offset(xtt_idx as isize))
                        .as_mut_ptr()
                        .offset(i as isize))
                        .as_mut_ptr()
                        .offset(band as isize),
                    ixheaacd_mps_dec_bitdec_tables,
                );
                band += 1;
            }
        }
        i += 1;
    }
    ixheaacd_mps_check_index_bounds(
        output_idx_data,
        num_parameter_sets,
        start_band,
        stop_band,
        param_type,
        xtt_idx,
    );
    if extend_frame != 0 {
        band = start_band;
        while band < stop_band {
            (*output_data
                .offset(xtt_idx as isize))[num_parameter_sets as usize][band as usize] = (*output_data
                .offset(
                    xtt_idx as isize,
                ))[(num_parameter_sets as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize][band as usize];
            (*output_idx_data
                .offset(xtt_idx as isize))[num_parameter_sets as usize][band as usize] = (*output_idx_data
                .offset(
                    xtt_idx as isize,
                ))[(num_parameter_sets as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize][band as usize];
            band += 1;
        }
    }
    return IA_NO_ERROR;
}
unsafe extern "C" fn ixheaacd_get_parameters_mapping(
    mut bs_parameter_bands: WORD32,
    mut mapping: *mut WORD32,
    mut ixheaacd_mps_dec_bitdec_tables: *mut ia_mps_dec_bitdec_tables_struct,
) -> VOID {
    match bs_parameter_bands {
        4 => {
            mapping = ((*ixheaacd_mps_dec_bitdec_tables).map_table.mapping_4_to_28)
                .as_mut_ptr();
        }
        5 => {
            mapping = ((*ixheaacd_mps_dec_bitdec_tables).map_table.mapping_5_to_28)
                .as_mut_ptr();
        }
        7 => {
            mapping = ((*ixheaacd_mps_dec_bitdec_tables).map_table.mapping_7_to_28)
                .as_mut_ptr();
        }
        10 => {
            mapping = ((*ixheaacd_mps_dec_bitdec_tables).map_table.mapping_10_to_28)
                .as_mut_ptr();
        }
        14 => {
            mapping = ((*ixheaacd_mps_dec_bitdec_tables).map_table.mapping_14_to_28)
                .as_mut_ptr();
        }
        20 => {
            mapping = ((*ixheaacd_mps_dec_bitdec_tables).map_table.mapping_20_to_28)
                .as_mut_ptr();
        }
        28 | _ => {}
    };
}
unsafe extern "C" fn ixheaacd_map_number_of_bands_to_28_bands(
    mut bands: WORD32,
    mut bs_parameter_bands: WORD32,
    mut bands28: *mut WORD32,
    mut ixheaacd_mps_dec_bitdec_tables: *mut ia_mps_dec_bitdec_tables_struct,
) -> VOID {
    let mut mapping: *mut WORD32 = 0 as *mut WORD32;
    let mut pb: WORD32 = 0;
    *bands28 = bands;
    ixheaacd_get_parameters_mapping(
        bs_parameter_bands,
        mapping,
        ixheaacd_mps_dec_bitdec_tables,
    );
    if !mapping.is_null() {
        pb = 0 as core::ffi::c_int as WORD32;
        while pb < MAX_PARAMETER_BANDS {
            if *mapping.offset(pb as isize) == bands {
                break;
            }
            pb += 1;
        }
        *bands28 = pb;
    }
}
unsafe extern "C" fn ixheaacd_map_data_to_28_bands(
    mut data: *mut WORD32,
    mut bs_parameter_bands: WORD32,
    mut ixheaacd_mps_dec_bitdec_tables: *mut ia_mps_dec_bitdec_tables_struct,
) -> VOID {
    let mut mapping: *mut WORD32 = 0 as *mut WORD32;
    let mut pb: WORD32 = 0;
    ixheaacd_get_parameters_mapping(
        bs_parameter_bands,
        mapping,
        ixheaacd_mps_dec_bitdec_tables,
    );
    if !mapping.is_null() {
        pb = (MAX_PARAMETER_BANDS - 1 as core::ffi::c_int) as WORD32;
        while pb >= 0 as core::ffi::c_int {
            *data.offset(pb as isize) = *data
                .offset(*mapping.offset(pb as isize) as isize);
            pb -= 1;
        }
    }
}
unsafe extern "C" fn ixheaacd_decode_and_map_frame_ott(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
) -> IA_ERRORCODE {
    let mut error_code: IA_ERRORCODE = IA_NO_ERROR;
    let mut p_cur_bs: *mut ia_mps_dec_spatial_bs_frame_struct = 0
        as *mut ia_mps_dec_spatial_bs_frame_struct;
    let mut curr_state: *mut ia_heaac_mps_state_struct = pstr_mps_state;
    let mut p_aux_struct: *mut ia_mps_dec_auxilary_struct = (*pstr_mps_state).aux_struct;
    let mut bitdec_table: *mut ia_mps_dec_bitdec_tables_struct = (*pstr_mps_state)
        .ia_mps_dec_mps_table
        .bitdec_table_ptr;
    let mut param_slot: *mut WORD32 = ((*(*pstr_mps_state).aux_struct).param_slot)
        .as_mut_ptr();
    let mut i: WORD32 = 0;
    let mut num_parameter_sets: WORD32 = 0;
    let mut ott_idx: WORD32 = 0;
    let mut band: WORD32 = 0;
    let mut num_ott_boxes: WORD32 = 0;
    let mut free_scratch: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut ps: WORD32 = 0;
    let mut pb: WORD32 = 0;
    let mut tot_db: *mut WORD32 = 0 as *mut WORD32;
    let mut ott_vs_tot_db_fc: *mut WORD32 = 0 as *mut WORD32;
    let mut ott_vs_tot_db_s: *mut WORD32 = 0 as *mut WORD32;
    let mut ott_vs_tot_db_f: *mut WORD32 = 0 as *mut WORD32;
    let mut ott_vs_tot_db_c: *mut WORD32 = 0 as *mut WORD32;
    let mut ott_vs_tot_db_lr: *mut WORD32 = 0 as *mut WORD32;
    let mut ott_vs_tot_db_l: *mut WORD32 = 0 as *mut WORD32;
    let mut ott_vs_tot_db_r: *mut WORD32 = 0 as *mut WORD32;
    let mut tmp1: *mut WORD32 = 0 as *mut WORD32;
    let mut tmp2: *mut WORD32 = 0 as *mut WORD32;
    let mut bitstream_parameter_bands: WORD32 = (*curr_state).bitstream_parameter_bands;
    let mut b_ott_bands: *mut WORD32 = ((*curr_state).bitstream_ott_bands).as_mut_ptr();
    let mut ott_cld_default: *mut WORD32 = ((*curr_state).ott_cld_default).as_mut_ptr();
    let mut parameter_sets: WORD32 = (*curr_state).num_parameter_sets;
    let mut extend_frame: WORD32 = (*curr_state).extend_frame;
    let mut quant_mode: WORD32 = (*curr_state).quant_mode;
    tot_db = (*pstr_mps_state).mps_scratch_mem_v as *mut WORD32;
    ott_vs_tot_db_fc = tot_db
        .offset(
            ((224 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    ott_vs_tot_db_s = ott_vs_tot_db_fc
        .offset(
            ((224 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    ott_vs_tot_db_f = ott_vs_tot_db_s
        .offset(
            ((224 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    ott_vs_tot_db_c = ott_vs_tot_db_f
        .offset(
            ((224 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    ott_vs_tot_db_lr = ott_vs_tot_db_c
        .offset(
            ((224 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    ott_vs_tot_db_l = ott_vs_tot_db_lr
        .offset(
            ((224 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    ott_vs_tot_db_r = ott_vs_tot_db_l
        .offset(
            ((224 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    tmp1 = ott_vs_tot_db_r
        .offset(
            ((224 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    tmp2 = tmp1
        .offset(
            ((224 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    free_scratch = tmp2
        .offset(
            ((224 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        ) as *mut core::ffi::c_void;
    p_cur_bs = (*pstr_mps_state).bs_frame;
    num_ott_boxes = (*curr_state).num_ott_boxes;
    pb = MAX_PSXPB as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < pb {
        *tot_db.offset(i as isize) = 0 as core::ffi::c_int as WORD32;
        i += 1;
    }
    match (*curr_state).tree_config {
        0 => {
            i = 0 as core::ffi::c_int as WORD32;
            error_code = ixheaacd_map_index_data(
                &mut (*p_cur_bs).cld_lossless_data,
                ((*p_aux_struct).ott_cld).as_mut_ptr(),
                ((*p_cur_bs).ott_cld_idx).as_mut_ptr(),
                ((*p_cur_bs).cmp_ott_cld_idx).as_mut_ptr(),
                0 as *mut [[WORD32; 28]; 8],
                i,
                ((*p_cur_bs).ott_cld_idx_prev).as_mut_ptr(),
                i,
                CLD,
                0 as WORD32,
                *b_ott_bands.offset(i as isize),
                *ott_cld_default.offset(i as isize),
                parameter_sets,
                param_slot,
                extend_frame,
                quant_mode,
                tot_db,
                ott_vs_tot_db_fc,
                ott_vs_tot_db_s,
                bitdec_table,
                free_scratch,
            );
            if error_code != 0 {
                return error_code;
            }
            i = 1 as core::ffi::c_int as WORD32;
            error_code = ixheaacd_map_index_data(
                &mut (*p_cur_bs).cld_lossless_data,
                ((*p_aux_struct).ott_cld).as_mut_ptr(),
                ((*p_cur_bs).ott_cld_idx).as_mut_ptr(),
                ((*p_cur_bs).cmp_ott_cld_idx).as_mut_ptr(),
                0 as *mut [[WORD32; 28]; 8],
                i,
                ((*p_cur_bs).ott_cld_idx_prev).as_mut_ptr(),
                i,
                CLD,
                0 as WORD32,
                *b_ott_bands.offset(i as isize),
                *ott_cld_default.offset(i as isize),
                parameter_sets,
                param_slot,
                extend_frame,
                quant_mode,
                ott_vs_tot_db_fc,
                ott_vs_tot_db_f,
                ott_vs_tot_db_c,
                bitdec_table,
                free_scratch,
            );
            if error_code != 0 {
                return error_code;
            }
            i = 2 as core::ffi::c_int as WORD32;
            error_code = ixheaacd_map_index_data(
                &mut (*p_cur_bs).cld_lossless_data,
                ((*p_aux_struct).ott_cld).as_mut_ptr(),
                ((*p_cur_bs).ott_cld_idx).as_mut_ptr(),
                ((*p_cur_bs).cmp_ott_cld_idx).as_mut_ptr(),
                0 as *mut [[WORD32; 28]; 8],
                i,
                ((*p_cur_bs).ott_cld_idx_prev).as_mut_ptr(),
                i,
                CLD,
                0 as WORD32,
                *b_ott_bands.offset(i as isize),
                *ott_cld_default.offset(i as isize),
                parameter_sets,
                param_slot,
                extend_frame,
                quant_mode,
                ott_vs_tot_db_s,
                tmp1,
                tmp2,
                bitdec_table,
                free_scratch,
            );
            if error_code != 0 {
                return error_code;
            }
            i = 3 as core::ffi::c_int as WORD32;
            error_code = ixheaacd_map_index_data(
                &mut (*p_cur_bs).cld_lossless_data,
                ((*p_aux_struct).ott_cld).as_mut_ptr(),
                ((*p_cur_bs).ott_cld_idx).as_mut_ptr(),
                ((*p_cur_bs).cmp_ott_cld_idx).as_mut_ptr(),
                0 as *mut [[WORD32; 28]; 8],
                i,
                ((*p_cur_bs).ott_cld_idx_prev).as_mut_ptr(),
                i,
                CLD,
                0 as WORD32,
                *b_ott_bands.offset(i as isize),
                *ott_cld_default.offset(i as isize),
                parameter_sets,
                param_slot,
                extend_frame,
                quant_mode,
                ott_vs_tot_db_f,
                tmp1,
                tmp2,
                bitdec_table,
                free_scratch,
            );
            if error_code != 0 {
                return error_code;
            }
            i = 4 as core::ffi::c_int as WORD32;
            error_code = ixheaacd_map_index_data(
                &mut (*p_cur_bs).cld_lossless_data,
                ((*p_aux_struct).ott_cld).as_mut_ptr(),
                ((*p_cur_bs).ott_cld_idx).as_mut_ptr(),
                ((*p_cur_bs).cmp_ott_cld_idx).as_mut_ptr(),
                0 as *mut [[WORD32; 28]; 8],
                i,
                ((*p_cur_bs).ott_cld_idx_prev).as_mut_ptr(),
                i,
                CLD,
                0 as WORD32,
                *b_ott_bands.offset(i as isize),
                *ott_cld_default.offset(i as isize),
                parameter_sets,
                param_slot,
                extend_frame,
                quant_mode,
                tot_db,
                tmp1,
                tmp2,
                bitdec_table,
                free_scratch,
            );
            if error_code != 0 {
                return error_code;
            }
        }
        1 => {
            i = 0 as core::ffi::c_int as WORD32;
            error_code = ixheaacd_map_index_data(
                &mut (*p_cur_bs).cld_lossless_data,
                ((*p_aux_struct).ott_cld).as_mut_ptr(),
                ((*p_cur_bs).ott_cld_idx).as_mut_ptr(),
                ((*p_cur_bs).cmp_ott_cld_idx).as_mut_ptr(),
                0 as *mut [[WORD32; 28]; 8],
                i,
                ((*p_cur_bs).ott_cld_idx_prev).as_mut_ptr(),
                i,
                CLD,
                0 as WORD32,
                *b_ott_bands.offset(i as isize),
                *ott_cld_default.offset(i as isize),
                parameter_sets,
                param_slot,
                extend_frame,
                quant_mode,
                tot_db,
                ott_vs_tot_db_lr,
                ott_vs_tot_db_c,
                bitdec_table,
                free_scratch,
            );
            if error_code != 0 {
                return error_code;
            }
            i = 1 as core::ffi::c_int as WORD32;
            error_code = ixheaacd_map_index_data(
                &mut (*p_cur_bs).cld_lossless_data,
                ((*p_aux_struct).ott_cld).as_mut_ptr(),
                ((*p_cur_bs).ott_cld_idx).as_mut_ptr(),
                ((*p_cur_bs).cmp_ott_cld_idx).as_mut_ptr(),
                0 as *mut [[WORD32; 28]; 8],
                i,
                ((*p_cur_bs).ott_cld_idx_prev).as_mut_ptr(),
                i,
                CLD,
                0 as WORD32,
                *b_ott_bands.offset(i as isize),
                *ott_cld_default.offset(i as isize),
                parameter_sets,
                param_slot,
                extend_frame,
                quant_mode,
                ott_vs_tot_db_lr,
                ott_vs_tot_db_l,
                ott_vs_tot_db_r,
                bitdec_table,
                free_scratch,
            );
            if error_code != 0 {
                return error_code;
            }
            i = 2 as core::ffi::c_int as WORD32;
            error_code = ixheaacd_map_index_data(
                &mut (*p_cur_bs).cld_lossless_data,
                ((*p_aux_struct).ott_cld).as_mut_ptr(),
                ((*p_cur_bs).ott_cld_idx).as_mut_ptr(),
                ((*p_cur_bs).cmp_ott_cld_idx).as_mut_ptr(),
                0 as *mut [[WORD32; 28]; 8],
                i,
                ((*p_cur_bs).ott_cld_idx_prev).as_mut_ptr(),
                i,
                CLD,
                0 as WORD32,
                *b_ott_bands.offset(i as isize),
                *ott_cld_default.offset(i as isize),
                parameter_sets,
                param_slot,
                extend_frame,
                quant_mode,
                tot_db,
                tmp1,
                tmp2,
                bitdec_table,
                free_scratch,
            );
            if error_code != 0 {
                return error_code;
            }
            i = 3 as core::ffi::c_int as WORD32;
            error_code = ixheaacd_map_index_data(
                &mut (*p_cur_bs).cld_lossless_data,
                ((*p_aux_struct).ott_cld).as_mut_ptr(),
                ((*p_cur_bs).ott_cld_idx).as_mut_ptr(),
                ((*p_cur_bs).cmp_ott_cld_idx).as_mut_ptr(),
                0 as *mut [[WORD32; 28]; 8],
                i,
                ((*p_cur_bs).ott_cld_idx_prev).as_mut_ptr(),
                i,
                CLD,
                0 as WORD32,
                *b_ott_bands.offset(i as isize),
                *ott_cld_default.offset(i as isize),
                parameter_sets,
                param_slot,
                extend_frame,
                quant_mode,
                ott_vs_tot_db_l,
                tmp1,
                tmp2,
                bitdec_table,
                free_scratch,
            );
            if error_code != 0 {
                return error_code;
            }
            i = 4 as core::ffi::c_int as WORD32;
            error_code = ixheaacd_map_index_data(
                &mut (*p_cur_bs).cld_lossless_data,
                ((*p_aux_struct).ott_cld).as_mut_ptr(),
                ((*p_cur_bs).ott_cld_idx).as_mut_ptr(),
                ((*p_cur_bs).cmp_ott_cld_idx).as_mut_ptr(),
                0 as *mut [[WORD32; 28]; 8],
                i,
                ((*p_cur_bs).ott_cld_idx_prev).as_mut_ptr(),
                i,
                CLD,
                0 as WORD32,
                *b_ott_bands.offset(i as isize),
                *ott_cld_default.offset(i as isize),
                parameter_sets,
                param_slot,
                extend_frame,
                quant_mode,
                ott_vs_tot_db_r,
                tmp1,
                tmp2,
                bitdec_table,
                free_scratch,
            );
            if error_code != 0 {
                return error_code;
            }
        }
        _ => {
            i = 0 as core::ffi::c_int as WORD32;
            while i < num_ott_boxes {
                error_code = ixheaacd_map_index_data(
                    &mut (*p_cur_bs).cld_lossless_data,
                    ((*p_aux_struct).ott_cld).as_mut_ptr(),
                    ((*p_cur_bs).ott_cld_idx).as_mut_ptr(),
                    ((*p_cur_bs).cmp_ott_cld_idx).as_mut_ptr(),
                    0 as *mut [[WORD32; 28]; 8],
                    i,
                    ((*p_cur_bs).ott_cld_idx_prev).as_mut_ptr(),
                    i,
                    CLD,
                    0 as WORD32,
                    *b_ott_bands.offset(i as isize),
                    *ott_cld_default.offset(i as isize),
                    parameter_sets,
                    param_slot,
                    extend_frame,
                    if (*curr_state).tree_config == TREE_525 as core::ffi::c_int {
                        0 as WORD32
                    } else {
                        quant_mode
                    },
                    0 as *mut WORD32,
                    0 as *mut WORD32,
                    0 as *mut WORD32,
                    bitdec_table,
                    free_scratch,
                );
                if error_code != 0 {
                    return error_code;
                }
                i += 1;
            }
        }
    }
    if (*curr_state).one_icc == 1 as core::ffi::c_int {
        if extend_frame == 0 as core::ffi::c_int {
            num_parameter_sets = parameter_sets;
        } else {
            num_parameter_sets = (parameter_sets as core::ffi::c_int
                + 1 as core::ffi::c_int) as WORD32;
        }
        ott_idx = 1 as core::ffi::c_int as WORD32;
        while ott_idx < num_ott_boxes {
            if (*curr_state).ott_mode_lfe[ott_idx as usize] == 0 as core::ffi::c_int {
                i = 0 as core::ffi::c_int as WORD32;
                while i < num_parameter_sets {
                    band = 0 as core::ffi::c_int as WORD32;
                    while band < bitstream_parameter_bands {
                        (*p_cur_bs)
                            .cmp_ott_icc_idx[ott_idx
                            as usize][i as usize][band as usize] = (*p_cur_bs)
                            .cmp_ott_icc_idx[0 as core::ffi::c_int
                            as usize][i as usize][band as usize];
                        band += 1;
                    }
                    i += 1;
                }
            }
            ott_idx += 1;
        }
        ott_idx = 0 as core::ffi::c_int as WORD32;
        while ott_idx < num_ott_boxes {
            if (*curr_state).ott_mode_lfe[ott_idx as usize] == 0 as core::ffi::c_int {
                error_code = ixheaacd_map_index_data(
                    &mut (*p_cur_bs).icc_lossless_data,
                    ((*p_aux_struct).ott_icc).as_mut_ptr(),
                    ((*p_cur_bs).ott_icc_idx).as_mut_ptr(),
                    ((*p_cur_bs).cmp_ott_icc_idx).as_mut_ptr(),
                    ((*p_cur_bs).ott_icc_diff_idx).as_mut_ptr(),
                    ott_idx,
                    ((*p_cur_bs).ott_icc_idx_prev).as_mut_ptr(),
                    0 as WORD32,
                    ICC,
                    0 as WORD32,
                    *b_ott_bands.offset(ott_idx as isize),
                    (*curr_state).icc_default,
                    parameter_sets,
                    param_slot,
                    extend_frame,
                    quant_mode,
                    0 as *mut WORD32,
                    0 as *mut WORD32,
                    0 as *mut WORD32,
                    bitdec_table,
                    free_scratch,
                );
                if error_code != 0 {
                    return error_code;
                }
            }
            ott_idx += 1;
        }
    } else {
        ott_idx = 0 as core::ffi::c_int as WORD32;
        while ott_idx < num_ott_boxes {
            if (*curr_state).ott_mode_lfe[ott_idx as usize] == 0 as core::ffi::c_int {
                error_code = ixheaacd_map_index_data(
                    &mut (*p_cur_bs).icc_lossless_data,
                    ((*p_aux_struct).ott_icc).as_mut_ptr(),
                    ((*p_cur_bs).ott_icc_idx).as_mut_ptr(),
                    ((*p_cur_bs).cmp_ott_icc_idx).as_mut_ptr(),
                    ((*p_cur_bs).ott_icc_diff_idx).as_mut_ptr(),
                    ott_idx,
                    ((*p_cur_bs).ott_icc_idx_prev).as_mut_ptr(),
                    ott_idx,
                    ICC,
                    0 as WORD32,
                    *b_ott_bands.offset(ott_idx as isize),
                    (*curr_state).icc_default,
                    parameter_sets,
                    param_slot,
                    extend_frame,
                    quant_mode,
                    0 as *mut WORD32,
                    0 as *mut WORD32,
                    0 as *mut WORD32,
                    bitdec_table,
                    free_scratch,
                );
                if error_code != 0 {
                    return error_code;
                }
            }
            ott_idx += 1;
        }
    }
    if (*curr_state).up_mix_type == 2 as core::ffi::c_int {
        let mut num_parameter_sets_0: WORD32 = parameter_sets;
        if extend_frame != 0 {
            num_parameter_sets_0 += 1;
        }
        ott_idx = 0 as core::ffi::c_int as WORD32;
        while ott_idx < (*curr_state).num_ott_boxes {
            ps = 0 as core::ffi::c_int as WORD32;
            while ps < num_parameter_sets_0 {
                ixheaacd_map_data_to_28_bands(
                    ((*p_aux_struct).ott_cld[ott_idx as usize][ps as usize])
                        .as_mut_ptr(),
                    bitstream_parameter_bands,
                    bitdec_table,
                );
                ixheaacd_map_data_to_28_bands(
                    ((*p_aux_struct).ott_icc[ott_idx as usize][ps as usize])
                        .as_mut_ptr(),
                    bitstream_parameter_bands,
                    bitdec_table,
                );
                ps += 1;
            }
            ott_idx += 1;
        }
    }
    return error_code;
}
unsafe extern "C" fn ixheaacd_decode_and_map_frame_ttt(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
) -> IA_ERRORCODE {
    let mut error_code: IA_ERRORCODE = IA_NO_ERROR;
    let mut p_cur_bs: *mut ia_mps_dec_spatial_bs_frame_struct = 0
        as *mut ia_mps_dec_spatial_bs_frame_struct;
    let mut bitdec_table: *mut ia_mps_dec_bitdec_tables_struct = (*pstr_mps_state)
        .ia_mps_dec_mps_table
        .bitdec_table_ptr;
    let mut p_aux_struct: *mut ia_mps_dec_auxilary_struct = (*pstr_mps_state).aux_struct;
    let mut param_slot: *mut WORD32 = ((*(*pstr_mps_state).aux_struct).param_slot)
        .as_mut_ptr();
    let mut num_bands: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut offset: WORD32 = 0;
    let mut num_ttt_boxes: WORD32 = 0;
    let mut free_scratch: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    p_cur_bs = (*pstr_mps_state).bs_frame;
    num_bands = (*pstr_mps_state).bitstream_parameter_bands;
    offset = (*pstr_mps_state).num_ott_boxes;
    num_ttt_boxes = (*pstr_mps_state).num_ttt_boxes;
    free_scratch = (*pstr_mps_state).mps_scratch_mem_v;
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_ttt_boxes {
        j = 0 as core::ffi::c_int as WORD32;
        while j < 2 as core::ffi::c_int
            && (*p_aux_struct).ttt_config[j as usize][i as usize].start_band
                < (*p_aux_struct).ttt_config[j as usize][i as usize].stop_band
        {
            if (*p_aux_struct).ttt_config[j as usize][i as usize].mode
                < 2 as core::ffi::c_int
            {
                error_code = ixheaacd_map_index_data(
                    &mut (*p_cur_bs).cpc_lossless_data,
                    ((*p_aux_struct).ttt_cpc_1).as_mut_ptr(),
                    ((*p_cur_bs).ttt_cpc_1_idx).as_mut_ptr(),
                    ((*p_cur_bs).cmp_ttt_cpc_1_idx).as_mut_ptr(),
                    0 as *mut [[WORD32; 28]; 8],
                    i,
                    ((*p_cur_bs).ttt_cpc_1_idx_prev).as_mut_ptr(),
                    offset + 4 as WORD32 * i + 2 as WORD32 * j,
                    CPC,
                    (*p_aux_struct)
                        .ttt_config[j as usize][i as usize]
                        .bitstream_start_band,
                    (*p_aux_struct)
                        .ttt_config[j as usize][i as usize]
                        .bitstream_stop_band,
                    (*pstr_mps_state).cpc_default,
                    (*pstr_mps_state).num_parameter_sets,
                    param_slot,
                    (*pstr_mps_state).extend_frame,
                    (*pstr_mps_state).quant_mode,
                    0 as *mut WORD32,
                    0 as *mut WORD32,
                    0 as *mut WORD32,
                    bitdec_table,
                    free_scratch,
                );
                if error_code != 0 {
                    return error_code;
                }
                error_code = ixheaacd_map_index_data(
                    &mut (*p_cur_bs).cpc_lossless_data,
                    ((*p_aux_struct).ttt_cpc_2).as_mut_ptr(),
                    ((*p_cur_bs).ttt_cpc_2_idx).as_mut_ptr(),
                    ((*p_cur_bs).cmp_ttt_cpc_2_idx).as_mut_ptr(),
                    0 as *mut [[WORD32; 28]; 8],
                    i,
                    ((*p_cur_bs).ttt_cpc_2_idx_prev).as_mut_ptr(),
                    offset + 4 as WORD32 * i + 1 as WORD32 + 2 as WORD32 * j,
                    CPC,
                    (*p_aux_struct)
                        .ttt_config[j as usize][i as usize]
                        .bitstream_start_band,
                    (*p_aux_struct)
                        .ttt_config[j as usize][i as usize]
                        .bitstream_stop_band,
                    (*pstr_mps_state).cpc_default,
                    (*pstr_mps_state).num_parameter_sets,
                    param_slot,
                    (*pstr_mps_state).extend_frame,
                    (*pstr_mps_state).quant_mode,
                    0 as *mut WORD32,
                    0 as *mut WORD32,
                    0 as *mut WORD32,
                    bitdec_table,
                    free_scratch,
                );
                if error_code != 0 {
                    return error_code;
                }
                error_code = ixheaacd_map_index_data(
                    &mut (*p_cur_bs).icc_lossless_data,
                    ((*p_aux_struct).ttt_icc).as_mut_ptr(),
                    ((*p_cur_bs).ttt_icc_idx).as_mut_ptr(),
                    ((*p_cur_bs).cmp_ttt_icc_idx).as_mut_ptr(),
                    0 as *mut [[WORD32; 28]; 8],
                    i,
                    ((*p_cur_bs).ttt_icc_idx_prev).as_mut_ptr(),
                    offset + 4 as WORD32 * i + 2 as WORD32 * j,
                    ICC,
                    (*p_aux_struct)
                        .ttt_config[j as usize][i as usize]
                        .bitstream_start_band,
                    (*p_aux_struct)
                        .ttt_config[j as usize][i as usize]
                        .bitstream_stop_band,
                    (*pstr_mps_state).icc_default,
                    (*pstr_mps_state).num_parameter_sets,
                    param_slot,
                    (*pstr_mps_state).extend_frame,
                    (*pstr_mps_state).quant_mode,
                    0 as *mut WORD32,
                    0 as *mut WORD32,
                    0 as *mut WORD32,
                    bitdec_table,
                    free_scratch,
                );
                if error_code != 0 {
                    return error_code;
                }
            } else {
                error_code = ixheaacd_map_index_data(
                    &mut (*p_cur_bs).cld_lossless_data,
                    ((*p_aux_struct).ttt_cld_1).as_mut_ptr(),
                    ((*p_cur_bs).ttt_cld_1_idx).as_mut_ptr(),
                    ((*p_cur_bs).cmp_ttt_cld_1_idx).as_mut_ptr(),
                    0 as *mut [[WORD32; 28]; 8],
                    i,
                    ((*p_cur_bs).ttt_cld_1_idx_prev).as_mut_ptr(),
                    offset + 4 as WORD32 * i + 2 as WORD32 * j,
                    CLD,
                    (*p_aux_struct)
                        .ttt_config[j as usize][i as usize]
                        .bitstream_start_band,
                    (*p_aux_struct)
                        .ttt_config[j as usize][i as usize]
                        .bitstream_stop_band,
                    (*pstr_mps_state).ttt_cld_1_default[i as usize],
                    (*pstr_mps_state).num_parameter_sets,
                    param_slot,
                    (*pstr_mps_state).extend_frame,
                    (*pstr_mps_state).quant_mode,
                    0 as *mut WORD32,
                    0 as *mut WORD32,
                    0 as *mut WORD32,
                    bitdec_table,
                    free_scratch,
                );
                if error_code != 0 {
                    return error_code;
                }
                error_code = ixheaacd_map_index_data(
                    &mut (*p_cur_bs).cld_lossless_data,
                    ((*p_aux_struct).ttt_cld_2).as_mut_ptr(),
                    ((*p_cur_bs).ttt_cld_2_idx).as_mut_ptr(),
                    ((*p_cur_bs).cmp_ttt_cld_2_idx).as_mut_ptr(),
                    0 as *mut [[WORD32; 28]; 8],
                    i,
                    ((*p_cur_bs).ttt_cld_2_idx_prev).as_mut_ptr(),
                    offset + 4 as WORD32 * i + 1 as WORD32 + 2 as WORD32 * j,
                    CLD,
                    (*p_aux_struct)
                        .ttt_config[j as usize][i as usize]
                        .bitstream_start_band,
                    (*p_aux_struct)
                        .ttt_config[j as usize][i as usize]
                        .bitstream_stop_band,
                    (*pstr_mps_state).ttt_cld_2_default[i as usize],
                    (*pstr_mps_state).num_parameter_sets,
                    param_slot,
                    (*pstr_mps_state).extend_frame,
                    (*pstr_mps_state).quant_mode,
                    0 as *mut WORD32,
                    0 as *mut WORD32,
                    0 as *mut WORD32,
                    bitdec_table,
                    free_scratch,
                );
                if error_code != 0 {
                    return error_code;
                }
            }
            if (*pstr_mps_state).up_mix_type == 2 as core::ffi::c_int {
                let mut num_parameter_sets: WORD32 = (*pstr_mps_state)
                    .num_parameter_sets;
                let mut ps: WORD32 = 0;
                if (*pstr_mps_state).extend_frame != 0 {
                    num_parameter_sets += 1;
                }
                ps = 0 as core::ffi::c_int as WORD32;
                while ps < num_parameter_sets {
                    ixheaacd_map_data_to_28_bands(
                        ((*p_aux_struct).ttt_cpc_1[i as usize][ps as usize])
                            .as_mut_ptr(),
                        num_bands,
                        bitdec_table,
                    );
                    ixheaacd_map_data_to_28_bands(
                        ((*p_aux_struct).ttt_cpc_2[i as usize][ps as usize])
                            .as_mut_ptr(),
                        num_bands,
                        bitdec_table,
                    );
                    ixheaacd_map_data_to_28_bands(
                        ((*p_aux_struct).ttt_cld_1[i as usize][ps as usize])
                            .as_mut_ptr(),
                        num_bands,
                        bitdec_table,
                    );
                    ixheaacd_map_data_to_28_bands(
                        ((*p_aux_struct).ttt_cld_2[i as usize][ps as usize])
                            .as_mut_ptr(),
                        num_bands,
                        bitdec_table,
                    );
                    ixheaacd_map_data_to_28_bands(
                        ((*p_aux_struct).ttt_icc[i as usize][ps as usize]).as_mut_ptr(),
                        num_bands,
                        bitdec_table,
                    );
                    ps += 1;
                }
            }
            j += 1;
        }
        i += 1;
    }
    return error_code;
}
unsafe extern "C" fn ixheaacd_decode_and_map_frame_smg(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
) -> VOID {
    let mut smooth_state: *mut ia_mps_dec_smoothing_state_struct = (*pstr_mps_state)
        .mps_persistent_mem
        .smooth_state;
    let mut bitdec_table: *mut ia_mps_dec_bitdec_tables_struct = (*pstr_mps_state)
        .ia_mps_dec_mps_table
        .bitdec_table_ptr;
    let mut p_aux_struct: *mut ia_mps_dec_auxilary_struct = (*pstr_mps_state).aux_struct;
    let mut smg_time: *mut WORD32 = ((*p_aux_struct).smg_time).as_mut_ptr();
    let mut ps: WORD32 = 0;
    let mut pb: WORD32 = 0;
    let mut pg: WORD32 = 0;
    let mut pb_stride: WORD32 = 0;
    let mut data_bands: WORD32 = 0;
    let mut pb_start: WORD32 = 0;
    let mut pb_stop: WORD32 = 0;
    let mut a_group_to_band: *mut WORD32 = 0 as *mut WORD32;
    let mut free_scratch: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut frame: *mut ia_mps_dec_spatial_bs_frame_struct = (*pstr_mps_state).bs_frame;
    (*pstr_mps_state).smooth_control = (*frame).bs_smooth_control;
    a_group_to_band = (*pstr_mps_state).mps_scratch_mem_v as *mut WORD32;
    free_scratch = a_group_to_band
        .offset(
            ((29 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        ) as *mut core::ffi::c_void;
    if (*pstr_mps_state).smooth_control != 0 {
        ps = 0 as core::ffi::c_int as WORD32;
        while ps < (*pstr_mps_state).num_parameter_sets {
            match (*frame).bs_smooth_mode[ps as usize] {
                0 => {
                    *smg_time.offset(ps as isize) = 256 as core::ffi::c_int as WORD32;
                    pb = 0 as core::ffi::c_int as WORD32;
                    while pb < (*pstr_mps_state).bitstream_parameter_bands {
                        (*p_aux_struct).smg_data[ps as usize][pb as usize] = 0
                            as core::ffi::c_int as WORD32;
                        pb += 1;
                    }
                }
                1 => {
                    if ps > 0 as core::ffi::c_int {
                        *smg_time.offset(ps as isize) = *smg_time
                            .offset(
                                (ps as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                            );
                    } else {
                        *smg_time.offset(ps as isize) = (*smooth_state).prev_smg_time;
                    }
                    pb = 0 as core::ffi::c_int as WORD32;
                    while pb < (*pstr_mps_state).bitstream_parameter_bands {
                        if ps > 0 as core::ffi::c_int {
                            (*p_aux_struct).smg_data[ps as usize][pb as usize] = (*p_aux_struct)
                                .smg_data[(ps as core::ffi::c_int - 1 as core::ffi::c_int)
                                as usize][pb as usize];
                        } else {
                            (*p_aux_struct).smg_data[ps as usize][pb as usize] = (*smooth_state)
                                .prev_smg_data[pb as usize];
                        }
                        pb += 1;
                    }
                }
                2 => {
                    *smg_time.offset(ps as isize) = (*bitdec_table)
                        .smg_time_table[(*frame).bs_smooth_time[ps as usize] as usize];
                    pb = 0 as core::ffi::c_int as WORD32;
                    while pb < (*pstr_mps_state).bitstream_parameter_bands {
                        (*p_aux_struct).smg_data[ps as usize][pb as usize] = 1
                            as core::ffi::c_int as WORD32;
                        pb += 1;
                    }
                }
                3 => {
                    *smg_time.offset(ps as isize) = (*bitdec_table)
                        .smg_time_table[(*frame).bs_smooth_time[ps as usize] as usize];
                    pb_stride = (*bitdec_table)
                        .pb_stride_table[(*frame).bs_freq_res_stride_smg[ps as usize]
                        as usize];
                    data_bands = (((*pstr_mps_state).bitstream_parameter_bands
                        as core::ffi::c_int - 1 as core::ffi::c_int)
                        / pb_stride as core::ffi::c_int + 1 as core::ffi::c_int)
                        as WORD32;
                    ixheaacd_create_mapping(
                        a_group_to_band as *mut WORD32,
                        0 as WORD32,
                        (*pstr_mps_state).bitstream_parameter_bands,
                        pb_stride,
                        free_scratch,
                    );
                    pg = 0 as core::ffi::c_int as WORD32;
                    while pg < data_bands {
                        pb_start = *a_group_to_band.offset(pg as isize);
                        pb_stop = *a_group_to_band
                            .offset(
                                (pg as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                            );
                        pb = pb_start;
                        while pb < pb_stop {
                            (*p_aux_struct).smg_data[ps as usize][pb as usize] = (*frame)
                                .bs_smg_data[ps as usize][pg as usize];
                            pb += 1;
                        }
                        pg += 1;
                    }
                }
                _ => {}
            }
            ps += 1;
        }
        (*smooth_state).prev_smg_time = *smg_time
            .offset(
                ((*pstr_mps_state).num_parameter_sets as core::ffi::c_int
                    - 1 as core::ffi::c_int) as isize,
            );
        pb = 0 as core::ffi::c_int as WORD32;
        while pb < (*pstr_mps_state).bitstream_parameter_bands {
            (*smooth_state).prev_smg_data[pb as usize] = (*p_aux_struct)
                .smg_data[((*pstr_mps_state).num_parameter_sets as core::ffi::c_int
                - 1 as core::ffi::c_int) as usize][pb as usize];
            pb += 1;
        }
        if (*pstr_mps_state).extend_frame != 0 {
            *smg_time.offset((*pstr_mps_state).num_parameter_sets as isize) = *smg_time
                .offset(
                    ((*pstr_mps_state).num_parameter_sets as core::ffi::c_int
                        - 1 as core::ffi::c_int) as isize,
                );
            pb = 0 as core::ffi::c_int as WORD32;
            while pb < (*pstr_mps_state).bitstream_parameter_bands {
                (*p_aux_struct)
                    .smg_data[(*pstr_mps_state).num_parameter_sets
                    as usize][pb as usize] = (*p_aux_struct)
                    .smg_data[((*pstr_mps_state).num_parameter_sets as core::ffi::c_int
                    - 1 as core::ffi::c_int) as usize][pb as usize];
                pb += 1;
            }
        }
        if (*pstr_mps_state).up_mix_type == 2 as core::ffi::c_int {
            let mut mapping: *mut WORD32 = 0 as *mut WORD32;
            ixheaacd_get_parameters_mapping(
                (*pstr_mps_state).bitstream_parameter_bands,
                mapping,
                bitdec_table,
            );
            if !mapping.is_null() {
                let mut num_parameter_sets: WORD32 = (*pstr_mps_state)
                    .num_parameter_sets;
                if (*pstr_mps_state).extend_frame != 0 {
                    num_parameter_sets += 1;
                }
                ps = 0 as core::ffi::c_int as WORD32;
                while ps < num_parameter_sets {
                    pb = (MAX_PARAMETER_BANDS - 1 as core::ffi::c_int) as WORD32;
                    while pb >= 0 as core::ffi::c_int {
                        (*p_aux_struct).smg_data[ps as usize][pb as usize] = (*p_aux_struct)
                            .smg_data[ps
                            as usize][*mapping.offset(pb as isize) as usize];
                        pb -= 1;
                    }
                    ps += 1;
                }
            }
        }
    }
}
unsafe extern "C" fn ixheaacd_decode_and_map_frame_arbdmx(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
) -> IA_ERRORCODE {
    let mut error_code: IA_ERRORCODE = IA_NO_ERROR;
    let mut frame: *mut ia_mps_dec_spatial_bs_frame_struct = (*pstr_mps_state).bs_frame;
    let mut bitdec_table: *mut ia_mps_dec_bitdec_tables_struct = (*pstr_mps_state)
        .ia_mps_dec_mps_table
        .bitdec_table_ptr;
    let mut p_aux_struct: *mut ia_mps_dec_auxilary_struct = (*pstr_mps_state).aux_struct;
    let mut param_slot: *mut WORD32 = ((*p_aux_struct).param_slot).as_mut_ptr();
    let mut offset: WORD32 = (*pstr_mps_state).num_ott_boxes
        + 4 as WORD32 * (*pstr_mps_state).num_ttt_boxes;
    let mut ch: WORD32 = 0;
    let mut scratch: *mut core::ffi::c_void = (*pstr_mps_state).mps_scratch_mem_v;
    ch = 0 as core::ffi::c_int as WORD32;
    while ch < (*pstr_mps_state).num_input_channels {
        error_code = ixheaacd_map_index_data(
            &mut (*frame).cld_lossless_data,
            ((*p_aux_struct).arbdmx_gain).as_mut_ptr(),
            ((*frame).arbdmx_gain_idx).as_mut_ptr(),
            ((*frame).cmp_arbdmx_gain_idx).as_mut_ptr(),
            0 as *mut [[WORD32; 28]; 8],
            ch,
            ((*frame).arbdmx_gain_idx_prev).as_mut_ptr(),
            offset + ch,
            CLD,
            0 as WORD32,
            (*pstr_mps_state).bitstream_parameter_bands,
            (*pstr_mps_state).arbdmx_gain_default,
            (*pstr_mps_state).num_parameter_sets,
            param_slot,
            (*pstr_mps_state).extend_frame,
            0 as WORD32,
            0 as *mut WORD32,
            0 as *mut WORD32,
            0 as *mut WORD32,
            bitdec_table,
            scratch,
        );
        if error_code != 0 {
            return error_code;
        }
        (*p_aux_struct).arbdmx_residual_abs[ch as usize] = (*frame)
            .bs_arbitrary_downmix_residual_abs[ch as usize];
        (*p_aux_struct).arbdmx_alpha_upd_set[ch as usize] = (*frame)
            .bs_arbitrary_downmix_residual_alpha_update_set[ch as usize];
        if (*pstr_mps_state).up_mix_type == 2 as core::ffi::c_int {
            let mut num_parameter_sets: WORD32 = (*pstr_mps_state).num_parameter_sets;
            let mut ps: WORD32 = 0;
            if (*pstr_mps_state).extend_frame != 0 {
                num_parameter_sets += 1;
            }
            ps = 0 as core::ffi::c_int as WORD32;
            while ps < num_parameter_sets {
                ixheaacd_map_data_to_28_bands(
                    ((*p_aux_struct).arbdmx_gain[ch as usize][ps as usize]).as_mut_ptr(),
                    (*pstr_mps_state).bitstream_parameter_bands,
                    bitdec_table,
                );
                ps += 1;
            }
        }
        ch += 1;
    }
    return error_code;
}
unsafe extern "C" fn ixheaacd_decode_and_map_frame_arb_tree(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
) -> IA_ERRORCODE {
    let mut error_code: IA_ERRORCODE = IA_NO_ERROR;
    let mut p_cur_bs: *mut ia_mps_dec_spatial_bs_frame_struct = (*pstr_mps_state)
        .bs_frame;
    let mut p_config: *mut ia_mps_spatial_bs_config_struct = &mut (*pstr_mps_state)
        .bs_config;
    let mut p_aux_struct: *mut ia_mps_dec_auxilary_struct = (*pstr_mps_state).aux_struct;
    let mut param_slot: *mut WORD32 = ((*p_aux_struct).param_slot).as_mut_ptr();
    let mut offset: WORD32 = (*pstr_mps_state).num_ott_boxes;
    let mut scratch: *mut core::ffi::c_void = (*pstr_mps_state).mps_scratch_mem_v;
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*p_config).num_ott_boxes_at {
        error_code = ixheaacd_map_index_data(
            &mut (*p_cur_bs).cld_lossless_data,
            ((*p_aux_struct).ott_cld).as_mut_ptr(),
            ((*p_cur_bs).ott_cld_idx).as_mut_ptr(),
            ((*p_cur_bs).cmp_ott_cld_idx).as_mut_ptr(),
            0 as *mut [[WORD32; 28]; 8],
            offset + i,
            ((*p_cur_bs).ott_cld_idx_prev).as_mut_ptr(),
            offset + i,
            CLD,
            0 as WORD32,
            (*p_config).bs_ott_bands_at[i as usize],
            (*p_config).bs_ott_default_cld_at[i as usize],
            (*pstr_mps_state).num_parameter_sets,
            param_slot,
            (*pstr_mps_state).extend_frame,
            (*pstr_mps_state).quant_mode,
            0 as *mut WORD32,
            0 as *mut WORD32,
            0 as *mut WORD32,
            (*pstr_mps_state).ia_mps_dec_mps_table.bitdec_table_ptr,
            scratch,
        );
        if error_code != 0 {
            return error_code;
        }
        i += 1;
    }
    return error_code;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_decode_frame(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
) -> IA_ERRORCODE {
    let mut error_code: IA_ERRORCODE = IA_NO_ERROR;
    let mut p_bs_config: *mut ia_mps_spatial_bs_config_struct = &mut (*pstr_mps_state)
        .bs_config;
    let mut param_slot: *mut WORD32 = ((*(*pstr_mps_state).aux_struct).param_slot)
        .as_mut_ptr();
    (*pstr_mps_state).extend_frame = 0 as core::ffi::c_int as WORD32;
    if *param_slot
        .offset(
            ((*pstr_mps_state).num_parameter_sets as core::ffi::c_int
                - 1 as core::ffi::c_int) as isize,
        ) != (*pstr_mps_state).time_slots as core::ffi::c_int - 1 as core::ffi::c_int
    {
        (*pstr_mps_state).extend_frame = 1 as core::ffi::c_int as WORD32;
    }
    if (*pstr_mps_state).extend_frame != 0 {
        if (*pstr_mps_state).num_parameter_sets == MAX_PARAMETER_SETS {
            if (*pstr_mps_state).ec_flag != 0 {
                (*pstr_mps_state).num_parameter_sets = 1 as core::ffi::c_int as WORD32;
            } else {
                return IA_FATAL_ERROR as IA_ERRORCODE
            }
        }
    }
    error_code = ixheaacd_decode_and_map_frame_ott(pstr_mps_state);
    if error_code != 0 {
        if (*pstr_mps_state).ec_flag != 0 {
            (*pstr_mps_state).frame_ok = 0 as core::ffi::c_int as WORD32;
            let mut idx: WORD32 = 0 as WORD32;
            while idx < MAX_NUM_OTT {
                ixheaacd_mps_check_index_bounds(
                    ((*(*pstr_mps_state).bs_frame).ott_cld_idx).as_mut_ptr(),
                    (*pstr_mps_state).num_parameter_sets,
                    0 as WORD32,
                    (*pstr_mps_state).bitstream_ott_bands[idx as usize],
                    CLD,
                    idx,
                );
                ixheaacd_mps_check_index_bounds(
                    ((*(*pstr_mps_state).bs_frame).ott_icc_idx).as_mut_ptr(),
                    (*pstr_mps_state).num_parameter_sets,
                    0 as WORD32,
                    (*pstr_mps_state).bitstream_ott_bands[idx as usize],
                    ICC,
                    idx,
                );
                idx += 1;
            }
        } else {
            return error_code
        }
    }
    error_code = ixheaacd_decode_and_map_frame_ttt(pstr_mps_state);
    if error_code != 0 {
        if (*pstr_mps_state).ec_flag != 0 {
            (*pstr_mps_state).frame_ok = 0 as core::ffi::c_int as WORD32;
            ixheaacd_mps_check_index_bounds(
                ((*(*pstr_mps_state).bs_frame).ttt_icc_idx).as_mut_ptr(),
                (*pstr_mps_state).num_parameter_sets,
                0 as WORD32,
                MAX_PARAMETER_BANDS,
                ICC,
                0 as WORD32,
            );
        } else {
            return error_code
        }
    }
    ixheaacd_decode_and_map_frame_smg(pstr_mps_state);
    if (*p_bs_config).arbitrary_tree != 0 as core::ffi::c_int {
        error_code = ixheaacd_decode_and_map_frame_arb_tree(pstr_mps_state);
        if error_code != 0 {
            if (*pstr_mps_state).ec_flag != 0 {
                (*pstr_mps_state).frame_ok = 0 as core::ffi::c_int as WORD32;
                let mut idx_0: WORD32 = 0 as WORD32;
                while idx_0 < MAX_NUM_OTT {
                    ixheaacd_mps_check_index_bounds(
                        ((*(*pstr_mps_state).bs_frame).ott_cld_idx).as_mut_ptr(),
                        (*pstr_mps_state).num_parameter_sets,
                        0 as WORD32,
                        MAX_PARAMETER_BANDS,
                        CLD,
                        idx_0,
                    );
                    idx_0 += 1;
                }
            } else {
                return error_code
            }
        }
    }
    if (*pstr_mps_state).arbitrary_downmix != 0 as core::ffi::c_int {
        error_code = ixheaacd_decode_and_map_frame_arbdmx(pstr_mps_state);
        if error_code != 0 {
            if (*pstr_mps_state).ec_flag != 0 {
                (*pstr_mps_state).frame_ok = 0 as core::ffi::c_int as WORD32;
                let mut idx_1: WORD32 = 0 as WORD32;
                while idx_1 < MAX_INPUT_CHANNELS_MPS {
                    ixheaacd_mps_check_index_bounds(
                        ((*(*pstr_mps_state).bs_frame).arbdmx_gain_idx).as_mut_ptr(),
                        (*pstr_mps_state).num_parameter_sets,
                        0 as WORD32,
                        MAX_PARAMETER_BANDS,
                        CLD,
                        idx_1,
                    );
                    idx_1 += 1;
                }
            } else {
                return error_code
            }
        }
    }
    if (*pstr_mps_state).extend_frame != 0 {
        (*pstr_mps_state).num_parameter_sets += 1;
        *param_slot
            .offset(
                ((*pstr_mps_state).num_parameter_sets as core::ffi::c_int
                    - 1 as core::ffi::c_int) as isize,
            ) = ((*pstr_mps_state).time_slots as core::ffi::c_int
            - 1 as core::ffi::c_int) as WORD32;
    }
    return IA_NO_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_set_current_state_parameters(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
) -> IA_ERRORCODE {
    let mut i: WORD32 = 0;
    let mut config: *mut ia_mps_spatial_bs_config_struct = &mut (*pstr_mps_state)
        .bs_config;
    let mut p_bs_config: *mut ia_mps_spatial_bs_config_struct = &mut (*pstr_mps_state)
        .bs_config;
    let mut curr_state: *mut ia_heaac_mps_state_struct = pstr_mps_state;
    let mut bitdec_table: *mut ia_mps_dec_bitdec_tables_struct = (*pstr_mps_state)
        .ia_mps_dec_mps_table
        .bitdec_table_ptr;
    let mut p_aux_struct: *mut ia_mps_dec_auxilary_struct = (*pstr_mps_state).aux_struct;
    let mut b_ott_bands: *mut WORD32 = ((*curr_state).bitstream_ott_bands).as_mut_ptr();
    if (*config).bs_sampling_freq_index == 15 as core::ffi::c_int {
        (*curr_state).sampling_freq = (*config).bs_sampling_frequency;
    } else {
        (*curr_state).sampling_freq = (*bitdec_table)
            .sampling_freq_table[(*config).bs_sampling_freq_index as usize];
    }
    (*curr_state).time_slots = ((*config).bs_frame_length as core::ffi::c_int
        + 1 as core::ffi::c_int) as WORD32;
    (*curr_state).frame_length = (*curr_state).time_slots * (*curr_state).qmf_bands;
    (*curr_state).bitstream_parameter_bands = (*bitdec_table)
        .freq_res_table[(*config).bs_freq_res as usize];
    (*curr_state).hybrid_bands = ((*curr_state).qmf_bands as core::ffi::c_int
        - QMF_BANDS_TO_HYBRID + 10 as core::ffi::c_int) as WORD32;
    (*curr_state).tp_hyb_band_border = 12 as core::ffi::c_int as WORD32;
    if (*curr_state).hybrid_bands > 71 as core::ffi::c_int {
        return IA_FATAL_ERROR as IA_ERRORCODE;
    }
    if (*curr_state).up_mix_type == 2 as core::ffi::c_int {
        (*curr_state).num_parameter_bands = MAX_PARAMETER_BANDS as WORD32;
    } else {
        (*curr_state).num_parameter_bands = (*curr_state).bitstream_parameter_bands;
    }
    match (*curr_state).num_parameter_bands {
        4 => {
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*curr_state).hybrid_bands {
                (*curr_state).kernels[i as usize] = (*bitdec_table)
                    .kernel_table
                    .kernels_4_to_71[i as usize] as size_t;
                i += 1;
            }
        }
        5 => {
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*curr_state).hybrid_bands {
                (*curr_state).kernels[i as usize] = (*bitdec_table)
                    .kernel_table
                    .kernels_5_to_71[i as usize] as size_t;
                i += 1;
            }
        }
        7 => {
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*curr_state).hybrid_bands {
                (*curr_state).kernels[i as usize] = (*bitdec_table)
                    .kernel_table
                    .kernels_7_to_71[i as usize] as size_t;
                i += 1;
            }
        }
        10 => {
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*curr_state).hybrid_bands {
                (*curr_state).kernels[i as usize] = (*bitdec_table)
                    .kernel_table
                    .kernels_10_to_71[i as usize] as size_t;
                i += 1;
            }
        }
        14 => {
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*curr_state).hybrid_bands {
                (*curr_state).kernels[i as usize] = (*bitdec_table)
                    .kernel_table
                    .kernels_14_to_71[i as usize] as size_t;
                i += 1;
            }
        }
        20 => {
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*curr_state).hybrid_bands {
                (*curr_state).kernels[i as usize] = (*bitdec_table)
                    .kernel_table
                    .kernels_20_to_71[i as usize] as size_t;
                i += 1;
            }
        }
        28 => {
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*curr_state).hybrid_bands {
                (*curr_state).kernels[i as usize] = (*bitdec_table)
                    .kernel_table
                    .kernels_28_to_71[i as usize] as size_t;
                i += 1;
            }
        }
        _ => return IA_XHEAAC_MPS_DEC_EXE_FATAL_INVALID_PARAMETER_BANDS as IA_ERRORCODE,
    }
    (*curr_state).tree_config = (*config).bs_tree_config;
    match (*curr_state).tree_config {
        0 | 1 | 2 => {
            (*config).ui_channel_mask = FIVE_POINT_ONE_CHANNEL_MASK as WORD32;
        }
        3 | 5 => {
            (*config).ui_channel_mask = SEVEN_POINT_ONE_CHANNEL_MASK1 as WORD32;
        }
        4 | 6 => {
            (*config).ui_channel_mask = SEVEN_POINT_ONE_CHANNEL_MASK2 as WORD32;
        }
        _ => return IA_XHEAAC_MPS_DEC_EXE_FATAL_UNSUPPRORTED_TREE_CONFIG as IA_ERRORCODE,
    }
    (*curr_state).num_ott_boxes = (*bitdec_table)
        .tree_property_table[(*curr_state).tree_config as usize]
        .num_ott_boxes;
    (*curr_state).num_ttt_boxes = (*bitdec_table)
        .tree_property_table[(*curr_state).tree_config as usize]
        .num_ttt_boxes;
    (*curr_state).num_input_channels = (*bitdec_table)
        .tree_property_table[(*curr_state).tree_config as usize]
        .num_input_channels;
    (*curr_state).num_output_channels = (*bitdec_table)
        .tree_property_table[(*curr_state).tree_config as usize]
        .num_output_channels;
    (*curr_state).quant_mode = (*config).bs_quant_mode;
    (*curr_state).one_icc = (*config).bs_one_icc;
    (*curr_state).arbitrary_downmix = (*config).bs_arbitrary_downmix;
    (*curr_state).residual_coding = (*config).bs_residual_coding;
    (*curr_state).smooth_config = (*config).bs_smooth_config;
    (*curr_state).mtx_inversion = (*config).bs_matrix_mode;
    (*curr_state).temp_shape_config = (*config).bs_temp_shape_config;
    (*curr_state).decorr_config = (*config).bs_decorr_config;
    (*curr_state).env_quant_mode = (*config).bs_env_quant_mode;
    (*curr_state).lfe_gain = (*bitdec_table)
        .lfe_gain_table[(*config).bs_fixed_gain_lfe as usize];
    (*curr_state).surround_gain = (*bitdec_table)
        .surround_gain_table[(*config).bs_fixed_gain_sur as usize];
    (*curr_state).clip_protect_gain = (*bitdec_table)
        .clip_gain_table[(*config).bs_fixed_gain_dmx as usize];
    if (*curr_state).up_mix_type == 2 as core::ffi::c_int {
        (*curr_state).num_output_channels = 2 as core::ffi::c_int as WORD32;
        (*curr_state).decorr_config = 0 as core::ffi::c_int as WORD32;
    }
    if (*curr_state).up_mix_type == 3 as core::ffi::c_int {
        (*curr_state).num_output_channels = 2 as core::ffi::c_int as WORD32;
    }
    if (*p_bs_config).arbitrary_tree == 1 as core::ffi::c_int {
        (*curr_state).num_output_channels_at = (*p_bs_config).num_out_chan_at;
    } else {
        (*curr_state).num_output_channels_at = (*curr_state).num_output_channels;
    }
    (*p_bs_config).ui_out_channels = (*curr_state).num_output_channels_at as UWORD32;
    (*curr_state)._3d_stereo_inversion = (*config).bs_3d_audio_mode;
    if (*curr_state).mtx_inversion == 1 as core::ffi::c_int
        || (*curr_state)._3d_stereo_inversion == 1 as core::ffi::c_int
    {
        (*curr_state).m1_param_imag_present = 1 as core::ffi::c_int as WORD32;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*curr_state).num_ott_boxes {
        if (*bitdec_table)
            .tree_property_table[(*curr_state).tree_config as usize]
            .ott_mode_lfe[i as usize] != 0
        {
            *b_ott_bands.offset(i as isize) = (*config).bs_ott_bands[i as usize];
            (*curr_state).ott_mode_lfe[i as usize] = 1 as core::ffi::c_int as WORD32;
        } else {
            *b_ott_bands.offset(i as isize) = (*curr_state).bitstream_parameter_bands;
            (*curr_state).ott_mode_lfe[i as usize] = 0 as core::ffi::c_int as WORD32;
        }
        if (*curr_state).up_mix_type == 2 as core::ffi::c_int {
            ixheaacd_map_number_of_bands_to_28_bands(
                *b_ott_bands.offset(i as isize),
                (*curr_state).bitstream_parameter_bands,
                &mut *((*p_aux_struct).num_ott_bands).as_mut_ptr().offset(i as isize),
                bitdec_table,
            );
        } else {
            (*p_aux_struct).num_ott_bands[i as usize] = *b_ott_bands.offset(i as isize);
        }
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*curr_state).num_ttt_boxes {
        (*p_aux_struct).ttt_config[0 as core::ffi::c_int as usize][i as usize].mode = (*config)
            .bs_ttt_mode_low[i as usize];
        (*p_aux_struct).ttt_config[1 as core::ffi::c_int as usize][i as usize].mode = (*config)
            .bs_ttt_mode_high[i as usize];
        (*p_aux_struct)
            .ttt_config[0 as core::ffi::c_int as usize][i as usize]
            .bitstream_start_band = 0 as core::ffi::c_int as WORD32;
        (*p_aux_struct)
            .ttt_config[1 as core::ffi::c_int as usize][i as usize]
            .bitstream_stop_band = (*curr_state).bitstream_parameter_bands;
        if (*config).bs_ttt_dual_mode[i as usize] != 0 {
            (*p_aux_struct)
                .ttt_config[0 as core::ffi::c_int as usize][i as usize]
                .bitstream_stop_band = (*config).bs_ttt_bands_low[i as usize];
            (*p_aux_struct)
                .ttt_config[1 as core::ffi::c_int as usize][i as usize]
                .bitstream_start_band = (*config).bs_ttt_bands_low[i as usize];
        } else {
            (*p_aux_struct)
                .ttt_config[0 as core::ffi::c_int as usize][i as usize]
                .bitstream_stop_band = (*curr_state).bitstream_parameter_bands;
            (*p_aux_struct)
                .ttt_config[1 as core::ffi::c_int as usize][i as usize]
                .bitstream_start_band = (*curr_state).bitstream_parameter_bands;
        }
        if (*curr_state).up_mix_type == 2 as core::ffi::c_int {
            ixheaacd_map_number_of_bands_to_28_bands(
                (*p_aux_struct)
                    .ttt_config[0 as core::ffi::c_int as usize][i as usize]
                    .bitstream_start_band,
                (*curr_state).bitstream_parameter_bands,
                &mut (*(*((*p_aux_struct).ttt_config)
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize))
                    .as_mut_ptr()
                    .offset(i as isize))
                    .start_band,
                bitdec_table,
            );
            ixheaacd_map_number_of_bands_to_28_bands(
                (*p_aux_struct)
                    .ttt_config[0 as core::ffi::c_int as usize][i as usize]
                    .bitstream_stop_band,
                (*curr_state).bitstream_parameter_bands,
                &mut (*(*((*p_aux_struct).ttt_config)
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize))
                    .as_mut_ptr()
                    .offset(i as isize))
                    .stop_band,
                bitdec_table,
            );
            ixheaacd_map_number_of_bands_to_28_bands(
                (*p_aux_struct)
                    .ttt_config[1 as core::ffi::c_int as usize][i as usize]
                    .bitstream_start_band,
                (*curr_state).bitstream_parameter_bands,
                &mut (*(*((*p_aux_struct).ttt_config)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as isize))
                    .as_mut_ptr()
                    .offset(i as isize))
                    .start_band,
                bitdec_table,
            );
            ixheaacd_map_number_of_bands_to_28_bands(
                (*p_aux_struct)
                    .ttt_config[1 as core::ffi::c_int as usize][i as usize]
                    .bitstream_stop_band,
                (*curr_state).bitstream_parameter_bands,
                &mut (*(*((*p_aux_struct).ttt_config)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as isize))
                    .as_mut_ptr()
                    .offset(i as isize))
                    .stop_band,
                bitdec_table,
            );
        } else {
            (*p_aux_struct)
                .ttt_config[0 as core::ffi::c_int as usize][i as usize]
                .start_band = (*p_aux_struct)
                .ttt_config[0 as core::ffi::c_int as usize][i as usize]
                .bitstream_start_band;
            (*p_aux_struct)
                .ttt_config[0 as core::ffi::c_int as usize][i as usize]
                .stop_band = (*p_aux_struct)
                .ttt_config[0 as core::ffi::c_int as usize][i as usize]
                .bitstream_stop_band;
            (*p_aux_struct)
                .ttt_config[1 as core::ffi::c_int as usize][i as usize]
                .start_band = (*p_aux_struct)
                .ttt_config[1 as core::ffi::c_int as usize][i as usize]
                .bitstream_start_band;
            (*p_aux_struct)
                .ttt_config[1 as core::ffi::c_int as usize][i as usize]
                .stop_band = (*p_aux_struct)
                .ttt_config[1 as core::ffi::c_int as usize][i as usize]
                .bitstream_stop_band;
        }
        i += 1;
    }
    (*curr_state).residual_coding = (*config).bs_residual_coding;
    (*curr_state).num_residual_signals = 0 as core::ffi::c_int as WORD32;
    if (*curr_state).residual_coding != 0 {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*curr_state).num_ttt_boxes + (*curr_state).num_ott_boxes {
            if (*config).bs_residual_present[i as usize] != 0 {
                (*curr_state).res_bands[i as usize] = (*config)
                    .bs_residual_bands[i as usize];
                (*curr_state).num_residual_signals += 1;
            } else {
                (*curr_state).res_bands[i as usize] = 0 as core::ffi::c_int as WORD32;
            }
            if (*curr_state).up_mix_type == 2 as core::ffi::c_int
                || (*curr_state).up_mix_type == 3 as core::ffi::c_int
            {
                (*curr_state).res_bands[i as usize] = 0 as core::ffi::c_int as WORD32;
            }
            i += 1;
        }
    }
    (*curr_state).residual_frames_per_spatial_frame = ((*p_bs_config)
        .bs_residual_frames_per_spatial_frame as core::ffi::c_int
        + 1 as core::ffi::c_int) as WORD32;
    if (*curr_state).residual_frames_per_spatial_frame > 0 as core::ffi::c_int {
        let mut reciprocal_tab: *const WORD32 = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .m1_m2_table_ptr)
            .reciprocal)
            .as_mut_ptr();
        let mut temp: WORD64 = ((*p_bs_config).bs_frame_length as core::ffi::c_int
            + 1 as core::ffi::c_int) as WORD64
            * *reciprocal_tab
                .offset((*p_bs_config).bs_residual_frames_per_spatial_frame as isize)
                as WORD64 >> 28 as core::ffi::c_int;
        (*curr_state).upd_qmf = temp as WORD32;
        if (*curr_state).upd_qmf != UPD_QMF_15 && (*curr_state).upd_qmf != UPD_QMF_16
            && (*curr_state).upd_qmf != UPD_QMF_32 && (*curr_state).upd_qmf != UPD_QMF_18
            && (*curr_state).upd_qmf != UPD_QMF_30 && (*curr_state).upd_qmf != UPD_QMF_24
        {
            return IA_XHEAAC_MPS_DEC_EXE_NONFATAL_INVALID_QMF_UPDATE;
        }
    }
    (*curr_state).arbdmx_residual_bands = (*config).bs_arbitrary_downmix_residual_bands;
    (*curr_state).arbdmx_frames_per_spatial_frame = ((*config)
        .bs_arbitrary_downmix_residual_frames_per_spatial_frame as core::ffi::c_int
        + 1 as core::ffi::c_int) as WORD32;
    if (*curr_state).arbdmx_frames_per_spatial_frame > 0 as core::ffi::c_int {
        (*curr_state).arbdmx_upd_qmf = (*curr_state).time_slots
            / (*curr_state).arbdmx_frames_per_spatial_frame;
        if (*curr_state).arbdmx_upd_qmf != UPD_QMF_15
            && (*curr_state).arbdmx_upd_qmf != UPD_QMF_16
            && (*curr_state).arbdmx_upd_qmf != UPD_QMF_32
            && (*curr_state).arbdmx_upd_qmf != UPD_QMF_18
            && (*curr_state).arbdmx_upd_qmf != UPD_QMF_30
            && (*curr_state).arbdmx_upd_qmf != UPD_QMF_24
        {
            return IA_XHEAAC_MPS_DEC_EXE_NONFATAL_INVALID_QMF_UPDATE;
        }
        if (*curr_state).arbdmx_upd_qmf as core::ffi::c_float * 1.5f32
            > ((*curr_state).upd_qmf as core::ffi::c_int * 2 as core::ffi::c_int)
                as core::ffi::c_float
        {
            return IA_XHEAAC_MPS_DEC_EXE_NONFATAL_INVALID_QMF_UPDATE;
        }
    }
    (*curr_state).cpc_default = 10 as core::ffi::c_int as WORD32;
    (*curr_state).ttt_cld_1_default[0 as core::ffi::c_int as usize] = 15
        as core::ffi::c_int as WORD32;
    (*curr_state).ttt_cld_2_default[0 as core::ffi::c_int as usize] = 0
        as core::ffi::c_int as WORD32;
    (*curr_state).icc_default = 0 as core::ffi::c_int as WORD32;
    (*curr_state).arbdmx_gain_default = 0 as core::ffi::c_int as WORD32;
    if (*curr_state)._3d_stereo_inversion != 0 {
        if (*config).bs_3d_audio_hrtf_set == 0 as core::ffi::c_int {
            return IA_XHEAAC_MPS_DEC_EXE_FATAL_INVALID_HRTF_SET as IA_ERRORCODE
        } else {
            return IA_XHEAAC_MPS_DEC_EXE_FATAL_INVALID_HRTF_SET as IA_ERRORCODE
        }
    }
    match (*curr_state).tree_config {
        0 => {
            (*curr_state).num_direct_signals = 1 as core::ffi::c_int as WORD32;
            (*curr_state).num_decor_signals = 4 as core::ffi::c_int as WORD32;
            if (*curr_state).up_mix_type == 2 as core::ffi::c_int {
                (*curr_state).num_decor_signals = 1 as core::ffi::c_int as WORD32;
            }
            if (*curr_state).up_mix_type == 3 as core::ffi::c_int {
                (*curr_state).num_decor_signals = 3 as core::ffi::c_int as WORD32;
            }
            (*curr_state).num_x_channels = 1 as core::ffi::c_int as WORD32;
            if (*curr_state).arbitrary_downmix == 2 as core::ffi::c_int {
                (*curr_state).num_x_channels += 1 as core::ffi::c_int;
            }
            (*curr_state).num_v_channels = (*curr_state).num_direct_signals
                + (*curr_state).num_decor_signals;
            (*curr_state).num_w_channels = (*curr_state).num_v_channels;
            (*curr_state).w_start_residual_idx = 0 as core::ffi::c_int as WORD32;
            (*curr_state).ott_cld_default[0 as core::ffi::c_int as usize] = 15
                as core::ffi::c_int as WORD32;
            (*curr_state).ott_cld_default[1 as core::ffi::c_int as usize] = 15
                as core::ffi::c_int as WORD32;
            (*curr_state).ott_cld_default[2 as core::ffi::c_int as usize] = 0
                as core::ffi::c_int as WORD32;
            (*curr_state).ott_cld_default[3 as core::ffi::c_int as usize] = 0
                as core::ffi::c_int as WORD32;
            (*curr_state).ott_cld_default[4 as core::ffi::c_int as usize] = 15
                as core::ffi::c_int as WORD32;
        }
        1 => {
            (*curr_state).num_direct_signals = 1 as core::ffi::c_int as WORD32;
            (*curr_state).num_decor_signals = 4 as core::ffi::c_int as WORD32;
            if (*curr_state).up_mix_type == 2 as core::ffi::c_int {
                (*curr_state).num_decor_signals = 1 as core::ffi::c_int as WORD32;
            }
            if (*curr_state).up_mix_type == 3 as core::ffi::c_int {
                (*curr_state).num_decor_signals = 2 as core::ffi::c_int as WORD32;
            }
            (*curr_state).num_x_channels = 1 as core::ffi::c_int as WORD32;
            if (*curr_state).arbitrary_downmix == 2 as core::ffi::c_int {
                (*curr_state).num_x_channels += 1 as core::ffi::c_int;
            }
            (*curr_state).num_v_channels = (*curr_state).num_direct_signals
                + (*curr_state).num_decor_signals;
            (*curr_state).num_w_channels = (*curr_state).num_v_channels;
            (*curr_state).w_start_residual_idx = 0 as core::ffi::c_int as WORD32;
            (*curr_state).ott_cld_default[0 as core::ffi::c_int as usize] = 15
                as core::ffi::c_int as WORD32;
            (*curr_state).ott_cld_default[1 as core::ffi::c_int as usize] = 0
                as core::ffi::c_int as WORD32;
            (*curr_state).ott_cld_default[2 as core::ffi::c_int as usize] = 15
                as core::ffi::c_int as WORD32;
            (*curr_state).ott_cld_default[3 as core::ffi::c_int as usize] = 15
                as core::ffi::c_int as WORD32;
            (*curr_state).ott_cld_default[4 as core::ffi::c_int as usize] = 15
                as core::ffi::c_int as WORD32;
        }
        2 => {
            (*curr_state).num_direct_signals = 3 as core::ffi::c_int as WORD32;
            i = 0 as core::ffi::c_int as WORD32;
            while i < 2 as core::ffi::c_int {
                match (*p_aux_struct)
                    .ttt_config[i as usize][0 as core::ffi::c_int as usize]
                    .mode
                {
                    0 => {
                        (*p_aux_struct)
                            .ttt_config[i as usize][0 as core::ffi::c_int as usize]
                            .use_ttt_decorr = 1 as core::ffi::c_int as WORD32;
                        (*curr_state).num_decor_signals = 3 as core::ffi::c_int
                            as WORD32;
                    }
                    1 | 2 | 3 | 4 | 5 => {
                        (*p_aux_struct)
                            .ttt_config[i as usize][0 as core::ffi::c_int as usize]
                            .use_ttt_decorr = 0 as core::ffi::c_int as WORD32;
                        (*curr_state).num_decor_signals = 2 as core::ffi::c_int
                            as WORD32;
                    }
                    _ => {
                        if (*p_bs_config).bs_ttt_mode_low[0 as core::ffi::c_int as usize]
                            <= 1 as core::ffi::c_int
                        {
                            return IA_XHEAAC_MPS_DEC_EXE_FATAL_INVALID_TTT_MODE
                                as IA_ERRORCODE;
                        }
                    }
                }
                i += 1;
            }
            if (*curr_state).residual_coding == 1 as core::ffi::c_int {
                (*curr_state).num_x_channels = 3 as core::ffi::c_int as WORD32;
            } else {
                (*curr_state).num_x_channels = 2 as core::ffi::c_int as WORD32;
            }
            if (*curr_state).arbitrary_downmix == 2 as core::ffi::c_int {
                (*curr_state).num_x_channels = 5 as core::ffi::c_int as WORD32;
            }
            if (*curr_state).up_mix_type == 2 as core::ffi::c_int {
                (*curr_state).num_direct_signals = 2 as core::ffi::c_int as WORD32;
                (*curr_state).num_decor_signals = 0 as core::ffi::c_int as WORD32;
                (*curr_state).num_x_channels = 2 as core::ffi::c_int as WORD32;
                if (*curr_state).arbitrary_downmix == 2 as core::ffi::c_int {
                    (*curr_state).num_direct_signals = 4 as core::ffi::c_int as WORD32;
                    (*curr_state).num_x_channels = 5 as core::ffi::c_int as WORD32;
                }
            }
            (*curr_state).num_v_channels = (*curr_state).num_direct_signals
                + (*curr_state).num_decor_signals;
            (*curr_state).num_w_channels = (*curr_state).num_v_channels;
            (*curr_state).w_start_residual_idx = 1 as core::ffi::c_int as WORD32;
            (*curr_state).ott_cld_default[0 as core::ffi::c_int as usize] = 15
                as core::ffi::c_int as WORD32;
            (*curr_state).ott_cld_default[1 as core::ffi::c_int as usize] = 15
                as core::ffi::c_int as WORD32;
            (*curr_state).ott_cld_default[2 as core::ffi::c_int as usize] = 15
                as core::ffi::c_int as WORD32;
        }
        3 | 4 => {
            (*curr_state).num_direct_signals = 3 as core::ffi::c_int as WORD32;
            i = 0 as core::ffi::c_int as WORD32;
            while i < 2 as core::ffi::c_int {
                match (*p_aux_struct)
                    .ttt_config[i as usize][0 as core::ffi::c_int as usize]
                    .mode
                {
                    0 => {
                        (*p_aux_struct)
                            .ttt_config[i as usize][0 as core::ffi::c_int as usize]
                            .use_ttt_decorr = 1 as core::ffi::c_int as WORD32;
                        (*curr_state).num_decor_signals = 5 as core::ffi::c_int
                            as WORD32;
                    }
                    1 | 2 | 3 | 4 | 5 => {
                        (*p_aux_struct)
                            .ttt_config[i as usize][0 as core::ffi::c_int as usize]
                            .use_ttt_decorr = 0 as core::ffi::c_int as WORD32;
                        (*curr_state).num_decor_signals = 5 as core::ffi::c_int
                            as WORD32;
                    }
                    _ => {
                        if (*p_bs_config).bs_ttt_mode_low[0 as core::ffi::c_int as usize]
                            <= 1 as core::ffi::c_int
                        {
                            return IA_XHEAAC_MPS_DEC_EXE_FATAL_INVALID_TTT_MODE
                                as IA_ERRORCODE;
                        }
                    }
                }
                i += 1;
            }
            if (*curr_state).residual_coding == 1 as core::ffi::c_int {
                (*curr_state).num_x_channels = 3 as core::ffi::c_int as WORD32;
            } else {
                (*curr_state).num_x_channels = 2 as core::ffi::c_int as WORD32;
            }
            if (*curr_state).arbitrary_downmix == 2 as core::ffi::c_int {
                (*curr_state).num_x_channels = 5 as core::ffi::c_int as WORD32;
            }
            if (*curr_state).up_mix_type == 2 as core::ffi::c_int {
                (*curr_state).num_direct_signals = 2 as core::ffi::c_int as WORD32;
                (*curr_state).num_decor_signals = 0 as core::ffi::c_int as WORD32;
                (*curr_state).num_x_channels = 2 as core::ffi::c_int as WORD32;
                if (*curr_state).arbitrary_downmix == 2 as core::ffi::c_int {
                    (*curr_state).num_direct_signals = 4 as core::ffi::c_int as WORD32;
                    (*curr_state).num_x_channels = 5 as core::ffi::c_int as WORD32;
                }
            }
            (*curr_state).num_v_channels = (*curr_state).num_direct_signals
                + (*curr_state).num_decor_signals;
            (*curr_state).num_w_channels = (*curr_state).num_v_channels;
            (*curr_state).w_start_residual_idx = 1 as core::ffi::c_int as WORD32;
            (*curr_state).ott_cld_default[0 as core::ffi::c_int as usize] = 15
                as core::ffi::c_int as WORD32;
            (*curr_state).ott_cld_default[1 as core::ffi::c_int as usize] = 15
                as core::ffi::c_int as WORD32;
            (*curr_state).ott_cld_default[2 as core::ffi::c_int as usize] = 15
                as core::ffi::c_int as WORD32;
            (*curr_state).ott_cld_default[3 as core::ffi::c_int as usize] = 15
                as core::ffi::c_int as WORD32;
            (*curr_state).ott_cld_default[4 as core::ffi::c_int as usize] = 15
                as core::ffi::c_int as WORD32;
        }
        5 | 6 => {
            (*curr_state).num_direct_signals = 6 as core::ffi::c_int as WORD32;
            (*curr_state).num_decor_signals = 2 as core::ffi::c_int as WORD32;
            (*curr_state).num_x_channels = 6 as core::ffi::c_int as WORD32;
            (*curr_state).num_v_channels = (*curr_state).num_direct_signals
                + (*curr_state).num_decor_signals;
            (*curr_state).num_w_channels = (*curr_state).num_v_channels;
            (*curr_state).w_start_residual_idx = 0 as core::ffi::c_int as WORD32;
            (*curr_state).ott_cld_default[0 as core::ffi::c_int as usize] = 15
                as core::ffi::c_int as WORD32;
            (*curr_state).ott_cld_default[1 as core::ffi::c_int as usize] = 15
                as core::ffi::c_int as WORD32;
        }
        _ => return IA_XHEAAC_MPS_DEC_EXE_FATAL_UNSUPPRORTED_TREE_CONFIG as IA_ERRORCODE,
    }
    return IA_NO_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_get_dequant_tables(
    mut cld: *mut *mut WORD32,
    mut icc: *mut *mut WORD32,
    mut cpc: *mut *mut WORD32,
    mut ixheaacd_mps_dec_bitdec_tables: *mut ia_mps_dec_bitdec_tables_struct,
) -> VOID {
    *cld = ((*ixheaacd_mps_dec_bitdec_tables).dequant_cld).as_mut_ptr();
    *icc = ((*ixheaacd_mps_dec_bitdec_tables).dequant_icc).as_mut_ptr();
    *cpc = ((*ixheaacd_mps_dec_bitdec_tables).dequant_cpc).as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_quantize_cld(
    mut v: WORD32,
    mut ixheaacd_mps_dec_bitdec_tables: *mut ia_mps_dec_bitdec_tables_struct,
) -> WORD32 {
    let mut i: WORD32 = 1 as WORD32;
    let mut temp_1: WORD32 = 0;
    let mut vmin: WORD32 = (*ixheaacd_mps_dec_bitdec_tables)
        .dequant_cld[0 as core::ffi::c_int as usize];
    let mut dmin: WORD32 = if v - vmin < 0 as core::ffi::c_int {
        -v - vmin
    } else {
        v - vmin
    };
    loop {
        temp_1 = if v - (*ixheaacd_mps_dec_bitdec_tables).dequant_cld[i as usize]
            < 0 as core::ffi::c_int
        {
            -v - (*ixheaacd_mps_dec_bitdec_tables).dequant_cld[i as usize]
        } else {
            v - (*ixheaacd_mps_dec_bitdec_tables).dequant_cld[i as usize]
        };
        if temp_1 < dmin {
            dmin = temp_1;
            vmin = (*ixheaacd_mps_dec_bitdec_tables).dequant_cld[i as usize];
        }
        let fresh18 = i;
        i = i + 1;
        if !((*ixheaacd_mps_dec_bitdec_tables).dequant_cld[fresh18 as usize]
            < ONE_FORTYNINE_Q15)
        {
            break;
        }
    }
    return vmin;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_quantize_icc(
    mut v: WORD32,
    mut ixheaacd_mps_dec_bitdec_tables: *mut ia_mps_dec_bitdec_tables_struct,
) -> WORD32 {
    let mut i: WORD32 = 1 as WORD32;
    let mut temp_1: WORD32 = 0;
    let mut vmin: WORD32 = (*ixheaacd_mps_dec_bitdec_tables)
        .dequant_icc[0 as core::ffi::c_int as usize];
    let mut dmin: WORD32 = if v - vmin < 0 as core::ffi::c_int {
        -v - vmin
    } else {
        v - vmin
    };
    loop {
        temp_1 = if v - (*ixheaacd_mps_dec_bitdec_tables).dequant_icc[i as usize]
            < 0 as core::ffi::c_int
        {
            -v - (*ixheaacd_mps_dec_bitdec_tables).dequant_icc[i as usize]
        } else {
            v - (*ixheaacd_mps_dec_bitdec_tables).dequant_icc[i as usize]
        };
        if temp_1 < dmin {
            dmin = temp_1;
            vmin = (*ixheaacd_mps_dec_bitdec_tables).dequant_icc[i as usize];
        }
        let fresh17 = i;
        i = i + 1;
        if !((*ixheaacd_mps_dec_bitdec_tables).dequant_icc[fresh17 as usize]
            > MINUS_POINT_NINE_EIGHT_Q15)
        {
            break;
        }
    }
    return vmin;
}
