extern "C" {
    pub type ia_mps_dec_ducker_interface;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn ixheaacd_get_dequant_tables(
        cld: *mut *mut WORD32,
        icc: *mut *mut WORD32,
        cpc: *mut *mut WORD32,
        ixheaacd_mps_dec_bitdec_tables: *mut ia_mps_dec_bitdec_tables_struct,
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
pub type LOOPIDX = core::ffi::c_int;
pub type WORD = core::ffi::c_int;
pub type LOOPINDEX = LOOPIDX;
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
pub const IA_XHEAAC_DEC_CONFIG_FATAL_INVALID_SAMPLE_RATE: core::ffi::c_uint = 0xffff8800
    as core::ffi::c_uint;
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
unsafe extern "C" fn ixheaac_mult32_shl(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 32 as core::ffi::c_int) as WORD32;
    return result << 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn ixheaac_sat64_32(mut a: WORD64) -> WORD32 {
    let mut result: WORD32 = 0;
    if a >= MAX_32 as WORD64 {
        result = MAX_32;
    } else if a <= MIN_32 as WORD64 {
        result = MIN_32;
    } else {
        result = a as WORD32;
    }
    return result;
}
pub const IA_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const MAX_PARAMETER_BANDS: core::ffi::c_int = 28 as core::ffi::c_int;
pub const ABS_THR_FIX: core::ffi::c_int = 35184 as core::ffi::c_int;
pub const MAX_NUM_QMF_BANDS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const MAX_HYBRID_BANDS: core::ffi::c_int = MAX_NUM_QMF_BANDS - 3 as core::ffi::c_int
    + 10 as core::ffi::c_int;
pub const MAX_OUTPUT_CHANNELS_MPS: core::ffi::c_int = 8 as core::ffi::c_int;
pub const SQRT_THREE_Q15: core::ffi::c_int = 56784 as core::ffi::c_int;
pub const ONE_IN_Q14: core::ffi::c_int = 16384 as core::ffi::c_int;
pub const ONE_IN_Q15: core::ffi::c_int = 32768 as core::ffi::c_int;
pub const ONE_IN_Q16: core::ffi::c_int = 65536 as core::ffi::c_int;
pub const THIRTY_IN_Q16: core::ffi::c_int = 1966080 as core::ffi::c_int;
pub const TSXHB: core::ffi::c_int = 5112 as core::ffi::c_int;
pub const ONE_BIT_MASK: core::ffi::c_int = 0x1 as core::ffi::c_int;
pub const TWO_BIT_MASK: core::ffi::c_int = 0x3 as core::ffi::c_int;
pub const THREE_BIT_MASK: core::ffi::c_int = 0x7 as core::ffi::c_int;
pub const FOUR_BIT_MASK: core::ffi::c_int = 0xf as core::ffi::c_int;
pub const FIVE_BIT_MASK: core::ffi::c_int = 0x1f as core::ffi::c_int;
pub const SIX_BIT_MASK: core::ffi::c_int = 0x3f as core::ffi::c_int;
pub const NORM32: core::ffi::c_int = 0x40000000 as core::ffi::c_int;
pub const INV_SQRT_2_Q31: core::ffi::c_int = 1518500250 as core::ffi::c_int;
pub const Q_SQRT_TAB: core::ffi::c_int = 15 as core::ffi::c_int;
pub const LOG2XQ17: core::ffi::c_longlong = 5171707904 as core::ffi::c_longlong;
pub const LOG_COEFF1: core::ffi::c_int = 27890 as core::ffi::c_int;
pub const LOG_COEFF2: core::ffi::c_int = 16262 as core::ffi::c_int;
pub const LOG_COEFF3: core::ffi::c_int = 7574 as core::ffi::c_int;
pub const LOG_COEFF4: core::ffi::c_int = 1786 as core::ffi::c_int;
#[inline]
unsafe extern "C" fn ixheaacd_mps_get_rshift_bits(mut a: WORD64) -> WORD32 {
    let mut temp_1: WORD32 = 0;
    let mut temp_2: WORD32 = 0;
    temp_1 = (a >> 32 as core::ffi::c_int) as WORD32;
    temp_2 = ixheaac_norm32(temp_1) as WORD32;
    if temp_2 < 31 as core::ffi::c_int {
        return 32 as WORD32 - temp_2
    } else {
        temp_2 = a as WORD32;
        if temp_1 ^ temp_2 < 0 as core::ffi::c_int {
            return 1 as WORD32
        } else {
            return 0 as WORD32
        }
    };
}
#[inline]
unsafe extern "C" fn ixheaacd_mps_sqrt(
    mut num: WORD32,
    mut q: *mut WORD16,
    mut sqrt_tab: *const WORD32,
) -> WORD32 {
    let mut index: WORD32 = 0;
    let mut answer: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut k: WORD = 0;
    if num == 0 as core::ffi::c_int {
        return 0 as WORD32;
    }
    k = ixheaac_norm32(num);
    temp = ixheaac_shr32(ixheaac_shl32(num, k), 21 as WORD);
    *q = (*q as core::ffi::c_int + k as core::ffi::c_int) as WORD16;
    index = (temp as core::ffi::c_int & 0x1ff as core::ffi::c_int) as WORD32;
    answer = *sqrt_tab.offset(index as isize);
    if *q as core::ffi::c_int & 1 as core::ffi::c_int != 0 {
        *q = (*q as core::ffi::c_int - 1 as core::ffi::c_int) as WORD16;
        answer = ixheaac_mult32_shl(answer, INV_SQRT_2_Q31);
    }
    *q = (*q as core::ffi::c_int >> 1 as core::ffi::c_int) as WORD16;
    *q = (*q as core::ffi::c_int + Q_SQRT_TAB) as WORD16;
    return answer;
}
#[inline]
unsafe extern "C" fn ixheaacd_mps_add32(
    mut a: WORD32,
    mut b: WORD32,
    mut q_a: *mut WORD16,
    mut q_b: WORD16,
) -> WORD32 {
    let mut temp_result: WORD64 = 0;
    if a == 0 as core::ffi::c_int || b == 0 as core::ffi::c_int {
        if b == 0 as core::ffi::c_int {
            return a
        } else {
            *q_a = q_b;
            return b;
        }
    }
    if *q_a as core::ffi::c_int > q_b as core::ffi::c_int {
        if *q_a as core::ffi::c_int - q_b as core::ffi::c_int > 31 as core::ffi::c_int {
            a = 0 as core::ffi::c_int as WORD32;
            *q_a = q_b;
        } else {
            a = a >> *q_a as core::ffi::c_int - q_b as core::ffi::c_int;
            *q_a = q_b;
        }
    } else if q_b as core::ffi::c_int - *q_a as core::ffi::c_int > 31 as core::ffi::c_int
    {
        b = 0 as core::ffi::c_int as WORD32;
    } else {
        b = b >> q_b as core::ffi::c_int - *q_a as core::ffi::c_int;
        q_b = *q_a;
    }
    temp_result = a as WORD64 + b as WORD64;
    if temp_result > 0x7fffffff as core::ffi::c_int as WORD64
        || temp_result < 0x80000000 as core::ffi::c_uint as WORD32 as WORD64
    {
        temp_result = temp_result >> 1 as core::ffi::c_int;
        *q_a = (*q_a as core::ffi::c_int - 1 as core::ffi::c_int) as WORD16;
    }
    return temp_result as WORD32;
}
#[inline]
unsafe extern "C" fn ixheaacd_mps_mult32(
    mut a: WORD32,
    mut b: WORD32,
    mut q_a: *mut WORD16,
    mut q_b: WORD16,
) -> WORD32 {
    let mut temp_result: WORD64 = 0;
    let mut temp: WORD32 = 0;
    if a == 0 as core::ffi::c_int || b == 0 as core::ffi::c_int {
        temp_result = 0 as WORD64;
        *q_a = 15 as WORD16;
        return temp_result as WORD32;
    }
    *q_a = (*q_a as core::ffi::c_int + q_b as core::ffi::c_int) as WORD16;
    temp_result = a as WORD64 * b as WORD64;
    temp = ixheaacd_mps_get_rshift_bits(temp_result);
    if 0 as core::ffi::c_int != temp {
        *q_a = (*q_a as core::ffi::c_int - temp as core::ffi::c_int) as WORD16;
        temp_result = temp_result >> temp;
    }
    return temp_result as WORD32;
}
#[inline]
unsafe extern "C" fn ixheaacd_mps_mult32_shr_16(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 16 as core::ffi::c_int) as WORD32;
    return result;
}
#[inline]
unsafe extern "C" fn ixheaacd_mps_mult32_shr_15(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = ixheaac_sat64_32(temp_result >> 15 as core::ffi::c_int);
    return result;
}
#[inline]
unsafe extern "C" fn ixheaacd_mps_div_32(
    mut a: WORD32,
    mut b: WORD32,
    mut q_format: *mut WORD16,
) -> WORD32 {
    let mut quotient: WORD32 = 0;
    let mut mantissa_nr: UWORD32 = 0;
    let mut mantissa_dr: UWORD32 = 0;
    let mut i: LOOPINDEX = 0;
    let mut q_nr: WORD = 0;
    let mut q_dr: WORD = 0;
    quotient = 0 as core::ffi::c_int as WORD32;
    if 0 as core::ffi::c_int == b {
        *q_format = 0 as WORD16;
        return a;
    }
    quotient = 0 as core::ffi::c_int as WORD32;
    q_nr = ixheaac_norm32(a);
    mantissa_nr = (a as UWORD32) << q_nr;
    q_dr = ixheaac_norm32(b);
    mantissa_dr = (b as UWORD32) << q_dr;
    *q_format = (30 as WORD + q_nr - q_dr) as WORD16;
    i = 0 as core::ffi::c_int as LOOPINDEX;
    while i < 31 as core::ffi::c_int {
        quotient <<= 1 as core::ffi::c_int;
        if mantissa_nr >= mantissa_dr {
            mantissa_nr = mantissa_nr.wrapping_sub(mantissa_dr);
            quotient += 1 as core::ffi::c_int;
        }
        mantissa_nr <<= 1 as core::ffi::c_int;
        i += 1;
    }
    if a ^ b < 0 as core::ffi::c_int {
        return -quotient;
    }
    return quotient;
}
#[inline]
unsafe extern "C" fn ixheaacd_mps_log10(mut a: WORD32, mut q_a: WORD16) -> WORD32 {
    let mut x: WORD32 = 0;
    let mut q_x: WORD16 = 0;
    let mut j: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut q_num: WORD16 = 0;
    q_num = ixheaac_norm32(a) as WORD16;
    a = a << q_num as core::ffi::c_int;
    x = ixheaacd_mps_div_32(a, NORM32, &mut q_x);
    if q_x as core::ffi::c_int > 16 as core::ffi::c_int {
        x = x >> q_x as core::ffi::c_int - 16 as core::ffi::c_int;
    } else {
        x = x << 16 as core::ffi::c_int - q_x as core::ffi::c_int;
    }
    q_num = (30 as core::ffi::c_int
        - (q_num as core::ffi::c_int + q_a as core::ffi::c_int)) as WORD16;
    j = (x as core::ffi::c_int - ONE_IN_Q16) as WORD32;
    k = ixheaacd_mps_mult32_shr_16(SQRT_THREE_Q15, j);
    temp = ixheaacd_mps_mult32_shr_16(j, j);
    k -= ixheaacd_mps_mult32_shr_16(LOG_COEFF1, temp);
    temp = ixheaacd_mps_mult32_shr_16(temp, j);
    k += ixheaacd_mps_mult32_shr_16(LOG_COEFF2, temp);
    temp = ixheaacd_mps_mult32_shr_16(temp, j);
    k -= ixheaacd_mps_mult32_shr_16(LOG_COEFF3, temp);
    temp = ixheaacd_mps_mult32_shr_16(temp, j);
    k += ixheaacd_mps_mult32_shr_16(LOG_COEFF4, temp);
    k += q_num as WORD32 * LOG2XQ17 as WORD32;
    return k >> 1 as core::ffi::c_int;
}
pub const HOP_SLOTS: core::ffi::c_int = 4 as core::ffi::c_int;
unsafe extern "C" fn ixheaacd_get_sampling_freq_idx(
    mut sampling_freq: WORD32,
    mut ixheaacd_mps_dec_bitdec_tables: *mut ia_mps_dec_bitdec_tables_struct,
    mut idx: *mut WORD32,
) -> IA_ERRORCODE {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < 13 as core::ffi::c_int {
        if (*ixheaacd_mps_dec_bitdec_tables).sampling_freq_table[i as usize]
            == sampling_freq
        {
            *idx = i;
            return IA_NO_ERROR;
        }
        i += 1;
    }
    *idx = 3 as core::ffi::c_int as WORD32;
    return IA_XHEAAC_DEC_CONFIG_FATAL_INVALID_SAMPLE_RATE as IA_ERRORCODE;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_init_blind(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
) -> VOID {
    let mut blind: *mut ia_mps_dec_blind_decoder_struct = (*pstr_mps_state)
        .mps_persistent_mem
        .blind_decoder;
    let mut p_blind_table: *mut ia_mps_dec_blind_tables_struct = (*pstr_mps_state)
        .ia_mps_dec_mps_table
        .blind_table_ptr;
    let mut p_aux_struct: *mut ia_mps_dec_auxilary_struct = (*pstr_mps_state).aux_struct;
    let mut temp_1: WORD32 = 0;
    let mut temp_2: WORD32 = 0;
    let mut q_64: WORD32 = 0;
    let mut q_32: WORD32 = 0;
    let mut q_16: WORD32 = 0;
    let mut q_8: WORD32 = 0;
    let mut q_4: WORD32 = 0;
    let mut q_2: WORD32 = 0;
    let mut r_64: WORD32 = 0;
    let mut r_32: WORD32 = 0;
    let mut r_16: WORD32 = 0;
    let mut r_8: WORD32 = 0;
    let mut r_4: WORD32 = 0;
    let mut r_2: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut qmf_bands: WORD32 = (*pstr_mps_state).qmf_bands;
    ixheaacd_get_sampling_freq_idx(
        (*pstr_mps_state).sampling_freq,
        (*pstr_mps_state).ia_mps_dec_mps_table.bitdec_table_ptr,
        &mut temp_1,
    );
    if (*pstr_mps_state).qmf_bands == 128 as core::ffi::c_int {
        (*blind).filter_coeff = (*p_blind_table).exp_128[temp_1 as usize];
    } else {
        q_64 = (if qmf_bands >> 6 as core::ffi::c_int != 0 {
            (*p_blind_table).exp_64[temp_1 as usize]
        } else {
            ONE_IN_Q15
        }) as WORD32;
        r_64 = qmf_bands as core::ffi::c_int & SIX_BIT_MASK;
        q_32 = (if r_64 >> 5 as core::ffi::c_int != 0 {
            (*p_blind_table).exp_32[temp_1 as usize]
        } else {
            ONE_IN_Q15
        }) as WORD32;
        r_32 = r_64 as core::ffi::c_int & FIVE_BIT_MASK;
        q_16 = (if r_32 >> 4 as core::ffi::c_int != 0 {
            (*p_blind_table).exp_16[temp_1 as usize]
        } else {
            ONE_IN_Q15
        }) as WORD32;
        r_16 = r_32 as core::ffi::c_int & FOUR_BIT_MASK;
        q_8 = (if r_16 >> 3 as core::ffi::c_int != 0 {
            (*p_blind_table).exp_8[temp_1 as usize]
        } else {
            ONE_IN_Q15
        }) as WORD32;
        r_8 = r_16 as core::ffi::c_int & THREE_BIT_MASK;
        q_4 = (if r_8 >> 2 as core::ffi::c_int != 0 {
            (*p_blind_table).exp_4[temp_1 as usize]
        } else {
            ONE_IN_Q15
        }) as WORD32;
        r_4 = r_8 as core::ffi::c_int & TWO_BIT_MASK;
        q_2 = (if r_4 >> 1 as core::ffi::c_int != 0 {
            (*p_blind_table).exp_2[temp_1 as usize]
        } else {
            ONE_IN_Q15
        }) as WORD32;
        r_2 = (if r_4 as core::ffi::c_int & ONE_BIT_MASK != 0 {
            (*p_blind_table).exp_1[temp_1 as usize]
        } else {
            ONE_IN_Q15
        }) as WORD32;
        temp_1 = ixheaacd_mps_mult32_shr_15(
            ixheaacd_mps_mult32_shr_15(q_64, q_32),
            q_16,
        );
        temp_2 = ixheaacd_mps_mult32_shr_15(ixheaacd_mps_mult32_shr_15(q_8, q_4), q_2);
        (*blind).filter_coeff = ixheaacd_mps_mult32_shr_15(
            ixheaacd_mps_mult32_shr_15(temp_1, temp_2),
            r_2,
        );
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < MAX_PARAMETER_BANDS {
        (*blind).excitation[0 as core::ffi::c_int as usize][i as usize] = ABS_THR_FIX
            as WORD32;
        (*blind).excitation[1 as core::ffi::c_int as usize][i as usize] = ABS_THR_FIX
            as WORD32;
        (*blind).excitation[2 as core::ffi::c_int as usize][i as usize] = ABS_THR_FIX
            as WORD32;
        (*blind).q_excitation[0 as core::ffi::c_int as usize][i as usize] = 15 as WORD16;
        (*blind).q_excitation[1 as core::ffi::c_int as usize][i as usize] = 15 as WORD16;
        (*blind).q_excitation[2 as core::ffi::c_int as usize][i as usize] = 15 as WORD16;
        i += 1;
    }
    memset(
        ((*p_aux_struct).temp_shape_enable_channel_stp).as_mut_ptr()
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (MAX_OUTPUT_CHANNELS_MPS as size_t)
            .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
    );
    memset(
        ((*p_aux_struct).temp_shape_enable_channel_ges).as_mut_ptr()
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (MAX_OUTPUT_CHANNELS_MPS as size_t)
            .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
    );
}
unsafe extern "C" fn ixheaacd_signal_2_parameters(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
    mut ps: WORD32,
) -> VOID {
    let mut blind: *mut ia_mps_dec_blind_decoder_struct = (*pstr_mps_state)
        .mps_persistent_mem
        .blind_decoder;
    let mut p_blind_table: *mut ia_mps_dec_blind_tables_struct = (*pstr_mps_state)
        .ia_mps_dec_mps_table
        .blind_table_ptr;
    let mut p_aux_struct: *mut ia_mps_dec_auxilary_struct = (*pstr_mps_state).aux_struct;
    let mut cld_index: WORD32 = 0;
    let mut icc_index: WORD32 = 0;
    let mut mesh: [[WORD32; 2]; 2] = [[0; 2]; 2];
    let mut pb: WORD32 = 0;
    let mut dequant_cld: *mut WORD32 = 0 as *mut WORD32;
    let mut dequant_icc: *mut WORD32 = 0 as *mut WORD32;
    let mut dequant_cpc: *mut WORD32 = 0 as *mut WORD32;
    let mut cld: WORD32 = 0;
    let mut icc: WORD32 = 0;
    let mut q_icc: WORD16 = 0;
    let mut cld_delta: WORD32 = 0;
    let mut icc_delta: WORD32 = 0;
    let mut q_icc_delta: WORD16 = 0;
    let mut temp_1: WORD32 = 0;
    let mut qtemp1: WORD16 = 0;
    let mut qtemp: WORD16 = 0;
    let mut sqrt_tab: *const WORD32 = ((*(*pstr_mps_state)
        .ia_mps_dec_mps_table
        .common_table_ptr)
        .sqrt_tab)
        .as_mut_ptr();
    let mut num_parameter_bands: WORD32 = (*pstr_mps_state).num_parameter_bands;
    ixheaacd_get_dequant_tables(
        &mut dequant_cld,
        &mut dequant_icc,
        &mut dequant_cpc,
        (*pstr_mps_state).ia_mps_dec_mps_table.bitdec_table_ptr,
    );
    pb = 0 as core::ffi::c_int as WORD32;
    while pb < num_parameter_bands {
        temp_1 = ixheaacd_mps_div_32(
            (*blind).excitation[0 as core::ffi::c_int as usize][pb as usize],
            (*blind).excitation[1 as core::ffi::c_int as usize][pb as usize],
            &mut qtemp1,
        );
        qtemp1 = (qtemp1 as core::ffi::c_int
            + (*blind).q_excitation[0 as core::ffi::c_int as usize][pb as usize]
                as core::ffi::c_int
            - (*blind).q_excitation[1 as core::ffi::c_int as usize][pb as usize]
                as core::ffi::c_int) as WORD16;
        cld = 10 as WORD32 * ixheaacd_mps_log10(temp_1, qtemp1);
        qtemp1 = (*blind).q_excitation[0 as core::ffi::c_int as usize][pb as usize];
        temp_1 = ixheaacd_mps_mult32(
            (*blind).excitation[0 as core::ffi::c_int as usize][pb as usize],
            (*blind).excitation[1 as core::ffi::c_int as usize][pb as usize],
            &mut qtemp1,
            (*blind).q_excitation[1 as core::ffi::c_int as usize][pb as usize],
        );
        temp_1 = ixheaacd_mps_sqrt(temp_1, &mut qtemp1, sqrt_tab);
        icc = ixheaacd_mps_div_32(
            (*blind).excitation[2 as core::ffi::c_int as usize][pb as usize],
            temp_1,
            &mut q_icc,
        );
        q_icc = (q_icc as core::ffi::c_int
            + (*blind).q_excitation[2 as core::ffi::c_int as usize][pb as usize]
                as core::ffi::c_int - qtemp1 as core::ffi::c_int) as WORD16;
        if if cld < 0 as core::ffi::c_int {
            -(cld as core::ffi::c_int)
        } else {
            (cld > THIRTY_IN_Q16) as core::ffi::c_int
        } != 0
        {
            cld_delta = THIRTY_IN_Q16 as WORD32;
        } else {
            cld_delta = ixheaac_abs32(cld);
        }
        q_icc_delta = q_icc;
        icc_delta = ixheaacd_mps_add32(icc, ONE_IN_Q15, &mut q_icc_delta, 15 as WORD16);
        icc_delta = ixheaacd_mps_mult32(
            icc_delta,
            10 as WORD32,
            &mut q_icc_delta,
            0 as WORD16,
        );
        temp_1 = cld_delta >> 16 as core::ffi::c_int;
        if temp_1 > 29 as core::ffi::c_int {
            cld_index = 29 as core::ffi::c_int as WORD32;
        } else {
            cld_index = temp_1;
        }
        temp_1 = icc_delta >> q_icc_delta as core::ffi::c_int;
        if temp_1 > 19 as core::ffi::c_int {
            icc_index = 19 as core::ffi::c_int as WORD32;
        } else {
            icc_index = temp_1;
        }
        cld_delta -= cld_index << 16 as core::ffi::c_int;
        icc_delta -= icc_index << q_icc_delta as core::ffi::c_int;
        mesh[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = ((*p_blind_table)
            .mesh_table
            .blind_cld_mesh[cld_index as usize][icc_index as usize]
            + 15 as core::ffi::c_int) as WORD32;
        mesh[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] = ((*p_blind_table)
            .mesh_table
            .blind_cld_mesh[cld_index
            as usize][(icc_index as core::ffi::c_int + 1 as core::ffi::c_int) as usize]
            + 15 as core::ffi::c_int) as WORD32;
        mesh[1 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = ((*p_blind_table)
            .mesh_table
            .blind_cld_mesh[(cld_index as core::ffi::c_int + 1 as core::ffi::c_int)
            as usize][icc_index as usize] + 15 as core::ffi::c_int) as WORD32;
        mesh[1 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] = ((*p_blind_table)
            .mesh_table
            .blind_cld_mesh[(cld_index as core::ffi::c_int + 1 as core::ffi::c_int)
            as usize][(icc_index as core::ffi::c_int + 1 as core::ffi::c_int) as usize]
            + 15 as core::ffi::c_int) as WORD32;
        qtemp1 = q_icc_delta;
        temp_1 = ixheaacd_mps_mult32(
            icc_delta,
            mesh[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                - mesh[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize],
            &mut qtemp1,
            0 as WORD16,
        );
        qtemp = 15 as WORD16;
        (*p_aux_struct)
            .ott_cld[0 as core::ffi::c_int as usize][ps as usize][pb as usize] = ixheaacd_mps_add32(
            mesh[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
                << 15 as core::ffi::c_int,
            temp_1,
            &mut qtemp,
            qtemp1,
        );
        qtemp1 = 16 as WORD16;
        temp_1 = ixheaacd_mps_mult32(
            cld_delta,
            mesh[1 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
                - mesh[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize],
            &mut qtemp1,
            0 as WORD16,
        );
        (*p_aux_struct)
            .ott_cld[0 as core::ffi::c_int as usize][ps as usize][pb as usize] = ixheaacd_mps_add32(
            (*p_aux_struct)
                .ott_cld[0 as core::ffi::c_int as usize][ps as usize][pb as usize],
            temp_1,
            &mut qtemp,
            qtemp1,
        );
        qtemp1 = q_icc_delta;
        temp_1 = ixheaacd_mps_mult32(icc_delta, cld_delta, &mut qtemp1, 16 as WORD16);
        temp_1 = ixheaacd_mps_mult32(
            temp_1,
            mesh[1 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                - mesh[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                - mesh[1 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
                + mesh[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize],
            &mut qtemp1,
            0 as WORD16,
        );
        (*p_aux_struct)
            .ott_cld[0 as core::ffi::c_int as usize][ps as usize][pb as usize] = ixheaacd_mps_add32(
            (*p_aux_struct)
                .ott_cld[0 as core::ffi::c_int as usize][ps as usize][pb as usize],
            temp_1,
            &mut qtemp,
            qtemp1,
        );
        (*p_aux_struct)
            .ott_cld[0 as core::ffi::c_int as usize][ps as usize][pb as usize] = ixheaacd_mps_add32(
            (*p_aux_struct)
                .ott_cld[0 as core::ffi::c_int as usize][ps as usize][pb as usize],
            ONE_IN_Q14,
            &mut qtemp,
            15 as WORD16,
        );
        (*p_aux_struct)
            .ott_cld[0 as core::ffi::c_int as usize][ps as usize][pb as usize] = *dequant_cld
            .offset(
                ((*p_aux_struct)
                    .ott_cld[0 as core::ffi::c_int as usize][ps as usize][pb as usize]
                    >> qtemp as core::ffi::c_int) as isize,
            );
        mesh[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = (*p_blind_table)
            .mesh_table
            .blind_icc_mesh[cld_index as usize][icc_index as usize];
        mesh[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] = (*p_blind_table)
            .mesh_table
            .blind_icc_mesh[cld_index
            as usize][(icc_index as core::ffi::c_int + 1 as core::ffi::c_int) as usize];
        mesh[1 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = (*p_blind_table)
            .mesh_table
            .blind_icc_mesh[(cld_index as core::ffi::c_int + 1 as core::ffi::c_int)
            as usize][icc_index as usize];
        mesh[1 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] = (*p_blind_table)
            .mesh_table
            .blind_icc_mesh[(cld_index as core::ffi::c_int + 1 as core::ffi::c_int)
            as usize][(icc_index as core::ffi::c_int + 1 as core::ffi::c_int) as usize];
        qtemp1 = q_icc_delta;
        temp_1 = ixheaacd_mps_mult32(
            icc_delta,
            mesh[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                - mesh[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize],
            &mut qtemp1,
            0 as WORD16,
        );
        qtemp = 15 as WORD16;
        (*p_aux_struct)
            .ott_icc[0 as core::ffi::c_int as usize][ps as usize][pb as usize] = ixheaacd_mps_add32(
            mesh[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
                << 15 as core::ffi::c_int,
            temp_1,
            &mut qtemp,
            qtemp1,
        );
        qtemp1 = 16 as WORD16;
        temp_1 = ixheaacd_mps_mult32(
            cld_delta,
            mesh[1 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
                - mesh[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize],
            &mut qtemp1,
            0 as WORD16,
        );
        (*p_aux_struct)
            .ott_icc[0 as core::ffi::c_int as usize][ps as usize][pb as usize] = ixheaacd_mps_add32(
            (*p_aux_struct)
                .ott_icc[0 as core::ffi::c_int as usize][ps as usize][pb as usize],
            temp_1,
            &mut qtemp,
            qtemp1,
        );
        qtemp1 = q_icc_delta;
        temp_1 = ixheaacd_mps_mult32(icc_delta, cld_delta, &mut qtemp1, 16 as WORD16);
        temp_1 = ixheaacd_mps_mult32(
            temp_1,
            mesh[1 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                - mesh[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                - mesh[1 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
                + mesh[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize],
            &mut qtemp1,
            0 as WORD16,
        );
        (*p_aux_struct)
            .ott_icc[0 as core::ffi::c_int as usize][ps as usize][pb as usize] = ixheaacd_mps_add32(
            (*p_aux_struct)
                .ott_icc[0 as core::ffi::c_int as usize][ps as usize][pb as usize],
            temp_1,
            &mut qtemp,
            qtemp1,
        );
        (*p_aux_struct)
            .ott_icc[0 as core::ffi::c_int as usize][ps as usize][pb as usize] = ixheaacd_mps_add32(
            (*p_aux_struct)
                .ott_icc[0 as core::ffi::c_int as usize][ps as usize][pb as usize],
            ONE_IN_Q14,
            &mut qtemp,
            15 as WORD16,
        );
        (*p_aux_struct)
            .ott_icc[0 as core::ffi::c_int as usize][ps as usize][pb as usize] = *dequant_icc
            .offset(
                ((*p_aux_struct)
                    .ott_icc[0 as core::ffi::c_int as usize][ps as usize][pb as usize]
                    >> qtemp as core::ffi::c_int) as isize,
            );
        mesh[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = ((*p_blind_table)
            .mesh_table
            .blind_cpc_1_mesh[cld_index as usize][icc_index as usize]
            + 20 as core::ffi::c_int) as WORD32;
        mesh[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] = ((*p_blind_table)
            .mesh_table
            .blind_cpc_1_mesh[cld_index
            as usize][(icc_index as core::ffi::c_int + 1 as core::ffi::c_int) as usize]
            + 20 as core::ffi::c_int) as WORD32;
        mesh[1 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = ((*p_blind_table)
            .mesh_table
            .blind_cpc_1_mesh[(cld_index as core::ffi::c_int + 1 as core::ffi::c_int)
            as usize][icc_index as usize] + 20 as core::ffi::c_int) as WORD32;
        mesh[1 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] = ((*p_blind_table)
            .mesh_table
            .blind_cpc_1_mesh[(cld_index as core::ffi::c_int + 1 as core::ffi::c_int)
            as usize][(icc_index as core::ffi::c_int + 1 as core::ffi::c_int) as usize]
            + 20 as core::ffi::c_int) as WORD32;
        qtemp1 = q_icc_delta;
        temp_1 = ixheaacd_mps_mult32(
            icc_delta,
            mesh[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                - mesh[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize],
            &mut qtemp1,
            0 as WORD16,
        );
        qtemp = 15 as WORD16;
        (*p_aux_struct)
            .ttt_cpc_1[0 as core::ffi::c_int as usize][ps as usize][pb as usize] = ixheaacd_mps_add32(
            mesh[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
                << 15 as core::ffi::c_int,
            temp_1,
            &mut qtemp,
            qtemp1,
        );
        qtemp1 = 16 as WORD16;
        temp_1 = ixheaacd_mps_mult32(
            cld_delta,
            mesh[1 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
                - mesh[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize],
            &mut qtemp1,
            0 as WORD16,
        );
        (*p_aux_struct)
            .ttt_cpc_1[0 as core::ffi::c_int as usize][ps as usize][pb as usize] = ixheaacd_mps_add32(
            (*p_aux_struct)
                .ttt_cpc_1[0 as core::ffi::c_int as usize][ps as usize][pb as usize],
            temp_1,
            &mut qtemp,
            qtemp1,
        );
        qtemp1 = q_icc_delta;
        temp_1 = ixheaacd_mps_mult32(icc_delta, cld_delta, &mut qtemp1, 16 as WORD16);
        temp_1 = ixheaacd_mps_mult32(
            temp_1,
            mesh[1 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                - mesh[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                - mesh[1 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
                + mesh[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize],
            &mut qtemp1,
            0 as WORD16,
        );
        (*p_aux_struct)
            .ttt_cpc_1[0 as core::ffi::c_int as usize][ps as usize][pb as usize] = ixheaacd_mps_add32(
            (*p_aux_struct)
                .ttt_cpc_1[0 as core::ffi::c_int as usize][ps as usize][pb as usize],
            temp_1,
            &mut qtemp,
            qtemp1,
        );
        (*p_aux_struct)
            .ttt_cpc_1[0 as core::ffi::c_int as usize][ps as usize][pb as usize] = ixheaacd_mps_add32(
            (*p_aux_struct)
                .ttt_cpc_1[0 as core::ffi::c_int as usize][ps as usize][pb as usize],
            ONE_IN_Q14,
            &mut qtemp,
            15 as WORD16,
        );
        (*p_aux_struct)
            .ttt_cpc_1[0 as core::ffi::c_int as usize][ps as usize][pb as usize] = *dequant_cpc
            .offset(
                ((*p_aux_struct)
                    .ttt_cpc_1[0 as core::ffi::c_int as usize][ps as usize][pb as usize]
                    >> qtemp as core::ffi::c_int) as isize,
            );
        mesh[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = ((*p_blind_table)
            .mesh_table
            .blind_cpc_2_mesh[cld_index as usize][icc_index as usize]
            + 20 as core::ffi::c_int) as WORD32;
        mesh[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] = ((*p_blind_table)
            .mesh_table
            .blind_cpc_2_mesh[cld_index
            as usize][(icc_index as core::ffi::c_int + 1 as core::ffi::c_int) as usize]
            + 20 as core::ffi::c_int) as WORD32;
        mesh[1 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = ((*p_blind_table)
            .mesh_table
            .blind_cpc_2_mesh[(cld_index as core::ffi::c_int + 1 as core::ffi::c_int)
            as usize][icc_index as usize] + 20 as core::ffi::c_int) as WORD32;
        mesh[1 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] = ((*p_blind_table)
            .mesh_table
            .blind_cpc_2_mesh[(cld_index as core::ffi::c_int + 1 as core::ffi::c_int)
            as usize][(icc_index as core::ffi::c_int + 1 as core::ffi::c_int) as usize]
            + 20 as core::ffi::c_int) as WORD32;
        qtemp1 = q_icc_delta;
        temp_1 = ixheaacd_mps_mult32(
            icc_delta,
            mesh[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                - mesh[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize],
            &mut qtemp1,
            0 as WORD16,
        );
        qtemp = 15 as WORD16;
        (*p_aux_struct)
            .ttt_cpc_2[0 as core::ffi::c_int as usize][ps as usize][pb as usize] = ixheaacd_mps_add32(
            (mesh[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
                < 15 as core::ffi::c_int) as core::ffi::c_int,
            temp_1,
            &mut qtemp,
            qtemp1,
        );
        qtemp1 = 16 as WORD16;
        temp_1 = ixheaacd_mps_mult32(
            cld_delta,
            mesh[1 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
                - mesh[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize],
            &mut qtemp1,
            0 as WORD16,
        );
        (*p_aux_struct)
            .ttt_cpc_2[0 as core::ffi::c_int as usize][ps as usize][pb as usize] = ixheaacd_mps_add32(
            (*p_aux_struct)
                .ttt_cpc_2[0 as core::ffi::c_int as usize][ps as usize][pb as usize],
            temp_1,
            &mut qtemp,
            qtemp1,
        );
        qtemp1 = q_icc_delta;
        temp_1 = ixheaacd_mps_mult32(icc_delta, cld_delta, &mut qtemp1, 16 as WORD16);
        temp_1 = ixheaacd_mps_mult32(
            temp_1,
            mesh[1 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                - mesh[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                - mesh[1 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
                + mesh[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize],
            &mut qtemp1,
            0 as WORD16,
        );
        (*p_aux_struct)
            .ttt_cpc_2[0 as core::ffi::c_int as usize][ps as usize][pb as usize] = ixheaacd_mps_add32(
            (*p_aux_struct)
                .ttt_cpc_2[0 as core::ffi::c_int as usize][ps as usize][pb as usize],
            temp_1,
            &mut qtemp,
            qtemp1,
        );
        (*p_aux_struct)
            .ttt_cpc_2[0 as core::ffi::c_int as usize][ps as usize][pb as usize] = ixheaacd_mps_add32(
            (*p_aux_struct)
                .ttt_cpc_2[0 as core::ffi::c_int as usize][ps as usize][pb as usize],
            ONE_IN_Q14,
            &mut qtemp,
            15 as WORD16,
        );
        (*p_aux_struct)
            .ttt_cpc_2[0 as core::ffi::c_int as usize][ps as usize][pb as usize] = *dequant_cpc
            .offset(
                ((*p_aux_struct)
                    .ttt_cpc_2[0 as core::ffi::c_int as usize][ps as usize][pb as usize]
                    >> qtemp as core::ffi::c_int) as isize,
            );
        if cld < 0 as core::ffi::c_int {
            cld = (*p_aux_struct)
                .ttt_cpc_2[0 as core::ffi::c_int as usize][ps as usize][pb as usize];
            (*p_aux_struct)
                .ttt_cpc_2[0 as core::ffi::c_int as usize][ps as usize][pb as usize] = (*p_aux_struct)
                .ttt_cpc_1[0 as core::ffi::c_int as usize][ps as usize][pb as usize];
            (*p_aux_struct)
                .ttt_cpc_1[0 as core::ffi::c_int as usize][ps as usize][pb as usize] = cld;
        }
        pb += 1;
    }
}
unsafe extern "C" fn ixheaacd_update_down_mix_state(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
    mut offset: WORD32,
) -> VOID {
    let mut blind: *mut ia_mps_dec_blind_decoder_struct = (*pstr_mps_state)
        .mps_persistent_mem
        .blind_decoder;
    let mut ts: WORD32 = 0;
    let mut hb: WORD32 = 0;
    let mut pb: WORD32 = 0;
    let mut excitation_0: *mut WORD32 = 0 as *mut WORD32;
    let mut excitation_1: *mut WORD32 = 0 as *mut WORD32;
    let mut excitation_2: *mut WORD32 = 0 as *mut WORD32;
    let mut q_excitation_0: *mut WORD16 = 0 as *mut WORD16;
    let mut q_excitation_1: *mut WORD16 = 0 as *mut WORD16;
    let mut q_excitation_2: *mut WORD16 = 0 as *mut WORD16;
    let mut temp_1: WORD32 = 0;
    let mut temp_2: WORD32 = 0;
    let mut qtemp1: WORD16 = 0;
    let mut qtemp2: WORD16 = 0;
    let mut p_x_real: *mut WORD32 = 0 as *mut WORD32;
    let mut p_x_imag: *mut WORD32 = 0 as *mut WORD32;
    let mut num_parameter_bands: WORD32 = (*pstr_mps_state).num_parameter_bands;
    let mut hybrid_bands: WORD32 = (*pstr_mps_state).hybrid_bands;
    excitation_0 = (*pstr_mps_state).mps_scratch_mem_v as *mut WORD32;
    q_excitation_0 = ((*pstr_mps_state).mps_scratch_mem_v as *mut WORD16)
        .offset(
            ((56 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD16>() as usize) as isize,
        );
    excitation_1 = excitation_0
        .offset(
            ((42 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    q_excitation_1 = q_excitation_0
        .offset(
            ((84 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD16>() as usize) as isize,
        );
    excitation_2 = excitation_1
        .offset(
            ((42 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    q_excitation_2 = q_excitation_1
        .offset(
            ((84 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD16>() as usize) as isize,
        );
    p_x_real = &mut *((*(*pstr_mps_state).array_struct).x_real)
        .offset((offset as core::ffi::c_int * MAX_HYBRID_BANDS) as isize) as *mut WORD32;
    p_x_imag = &mut *((*(*pstr_mps_state).array_struct).x_imag)
        .offset((offset as core::ffi::c_int * MAX_HYBRID_BANDS) as isize) as *mut WORD32;
    ts = 0 as core::ffi::c_int as WORD32;
    while ts < HOP_SLOTS {
        let mut x_real_0: *mut WORD32 = p_x_real;
        let mut x_imag_0: *mut WORD32 = p_x_imag;
        let mut x_real_1: *mut WORD32 = p_x_real.offset(TSXHB as isize);
        let mut x_imag_1: *mut WORD32 = p_x_imag.offset(TSXHB as isize);
        pb = 0 as core::ffi::c_int as WORD32;
        while pb < num_parameter_bands {
            *excitation_0.offset(pb as isize) = ABS_THR_FIX as WORD32;
            *excitation_1.offset(pb as isize) = ABS_THR_FIX as WORD32;
            *excitation_2.offset(pb as isize) = ABS_THR_FIX as WORD32;
            *q_excitation_0.offset(pb as isize) = 15 as WORD16;
            *q_excitation_1.offset(pb as isize) = 15 as WORD16;
            *q_excitation_2.offset(pb as isize) = 15 as WORD16;
            pb += 1;
        }
        hb = 0 as core::ffi::c_int as WORD32;
        while hb < hybrid_bands {
            let mut temp: WORD64 = 0;
            pb = (*pstr_mps_state).kernels[hb as usize] as WORD32;
            temp = *x_real_0 as WORD64 * *x_real_0 as WORD64
                + *x_imag_0 as WORD64 * *x_imag_0 as WORD64;
            temp >>= 10 as core::ffi::c_int;
            temp_1 = temp as WORD32;
            qtemp1 = 10 as WORD16;
            *excitation_0.offset(pb as isize) = ixheaacd_mps_add32(
                *excitation_0.offset(pb as isize),
                temp_1,
                &mut *q_excitation_0.offset(pb as isize),
                qtemp1,
            );
            temp = *x_real_1 as WORD64 * *x_real_1 as WORD64
                + *x_imag_1 as WORD64 * *x_imag_1 as WORD64;
            temp >>= 10 as core::ffi::c_int;
            temp_1 = temp as WORD32;
            qtemp1 = 10 as WORD16;
            *excitation_1.offset(pb as isize) = ixheaacd_mps_add32(
                *excitation_1.offset(pb as isize),
                temp_1,
                &mut *q_excitation_1.offset(pb as isize),
                qtemp1,
            );
            temp = *x_real_0 as WORD64 * *x_real_1 as WORD64
                + *x_imag_0 as WORD64 * *x_imag_1 as WORD64;
            temp >>= 10 as core::ffi::c_int;
            temp_1 = temp as WORD32;
            qtemp1 = 10 as WORD16;
            *excitation_2.offset(pb as isize) = ixheaacd_mps_add32(
                *excitation_2.offset(pb as isize),
                temp_1,
                &mut *q_excitation_2.offset(pb as isize),
                qtemp1,
            );
            x_real_0 = x_real_0.offset(1);
            x_imag_0 = x_imag_0.offset(1);
            x_real_1 = x_real_1.offset(1);
            x_imag_1 = x_imag_1.offset(1);
            hb += 1;
        }
        pb = 0 as core::ffi::c_int as WORD32;
        while pb < num_parameter_bands {
            (*blind).excitation[0 as core::ffi::c_int as usize][pb as usize] = ixheaacd_mps_mult32_shr_15(
                (*blind).excitation[0 as core::ffi::c_int as usize][pb as usize],
                (*blind).filter_coeff,
            );
            (*blind).excitation[1 as core::ffi::c_int as usize][pb as usize] = ixheaacd_mps_mult32_shr_15(
                (*blind).excitation[1 as core::ffi::c_int as usize][pb as usize],
                (*blind).filter_coeff,
            );
            (*blind).excitation[2 as core::ffi::c_int as usize][pb as usize] = ixheaacd_mps_mult32_shr_15(
                (*blind).excitation[2 as core::ffi::c_int as usize][pb as usize],
                (*blind).filter_coeff,
            );
            temp_1 = ONE_IN_Q15 - (*blind).filter_coeff;
            let fresh0 = q_excitation_0;
            q_excitation_0 = q_excitation_0.offset(1);
            qtemp2 = *fresh0;
            temp_2 = ixheaacd_mps_mult32_shr_15(temp_1, *excitation_0);
            excitation_0 = excitation_0.offset(1);
            (*blind).excitation[0 as core::ffi::c_int as usize][pb as usize] = ixheaacd_mps_add32(
                (*blind).excitation[0 as core::ffi::c_int as usize][pb as usize],
                temp_2,
                &mut *(*((*blind).q_excitation)
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize))
                    .as_mut_ptr()
                    .offset(pb as isize),
                qtemp2,
            );
            let fresh1 = q_excitation_1;
            q_excitation_1 = q_excitation_1.offset(1);
            qtemp2 = *fresh1;
            temp_2 = ixheaacd_mps_mult32_shr_15(temp_1, *excitation_1);
            excitation_1 = excitation_1.offset(1);
            (*blind).excitation[1 as core::ffi::c_int as usize][pb as usize] = ixheaacd_mps_add32(
                (*blind).excitation[1 as core::ffi::c_int as usize][pb as usize],
                temp_2,
                &mut *(*((*blind).q_excitation)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as isize))
                    .as_mut_ptr()
                    .offset(pb as isize),
                qtemp2,
            );
            let fresh2 = q_excitation_2;
            q_excitation_2 = q_excitation_2.offset(1);
            qtemp2 = *fresh2;
            temp_2 = ixheaacd_mps_mult32_shr_15(temp_1, *excitation_2);
            excitation_2 = excitation_2.offset(1);
            (*blind).excitation[2 as core::ffi::c_int as usize][pb as usize] = ixheaacd_mps_add32(
                (*blind).excitation[2 as core::ffi::c_int as usize][pb as usize],
                temp_2,
                &mut *(*((*blind).q_excitation)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as isize))
                    .as_mut_ptr()
                    .offset(pb as isize),
                qtemp2,
            );
            pb += 1;
        }
        *excitation_0.offset(pb as isize) -= num_parameter_bands;
        *excitation_1.offset(pb as isize) -= num_parameter_bands;
        *excitation_2.offset(pb as isize) -= num_parameter_bands;
        let ref mut fresh3 = *q_excitation_0.offset(pb as isize);
        *fresh3 = (*fresh3 as core::ffi::c_int - num_parameter_bands as core::ffi::c_int)
            as WORD16;
        let ref mut fresh4 = *q_excitation_1.offset(pb as isize);
        *fresh4 = (*fresh4 as core::ffi::c_int - num_parameter_bands as core::ffi::c_int)
            as WORD16;
        let ref mut fresh5 = *q_excitation_2.offset(pb as isize);
        *fresh5 = (*fresh5 as core::ffi::c_int - num_parameter_bands as core::ffi::c_int)
            as WORD16;
        p_x_real = p_x_real.offset(MAX_HYBRID_BANDS as isize);
        p_x_imag = p_x_imag.offset(MAX_HYBRID_BANDS as isize);
        ts += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_apply_blind(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
) -> VOID {
    let mut frame: *mut ia_mps_dec_spatial_bs_frame_struct = (*pstr_mps_state).bs_frame;
    let mut ts: WORD32 = 0;
    let mut ps: WORD32 = 0;
    let mut time_slots: WORD32 = (*pstr_mps_state).time_slots;
    let mut param_slot: *mut WORD32 = ((*(*pstr_mps_state).aux_struct).param_slot)
        .as_mut_ptr();
    ts = 0 as core::ffi::c_int as WORD32;
    ps = 0 as core::ffi::c_int as WORD32;
    while ts < time_slots {
        *param_slot.offset(ps as isize) = (ts as core::ffi::c_int + HOP_SLOTS
            - 1 as core::ffi::c_int) as WORD32;
        ixheaacd_signal_2_parameters(pstr_mps_state, ps);
        ixheaacd_update_down_mix_state(pstr_mps_state, ts);
        ts += HOP_SLOTS;
        ps += 1;
    }
    (*pstr_mps_state).num_parameter_sets_prev = ps;
    (*pstr_mps_state).num_parameter_sets = ps;
    (*frame).bs_independency_flag = 0 as core::ffi::c_int as WORD32;
    (*(*pstr_mps_state).aux_struct).num_ott_bands[0 as core::ffi::c_int as usize] = (*pstr_mps_state)
        .num_parameter_bands;
}
