extern "C" {
    pub type ia_mps_dec_ducker_interface;
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
pub type size_t = usize;
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
unsafe extern "C" fn ixheaac_add32_sat3(
    mut a: WORD32,
    mut b: WORD32,
    mut c: WORD32,
) -> WORD32 {
    let mut sum: WORD64 = 0;
    sum = a as WORD64 + b as WORD64;
    sum = sum + c as WORD64;
    if sum > MAX_32 as WORD64 {
        sum = MAX_32 as WORD64;
    }
    if sum < MIN_32 as WORD64 {
        sum = MIN_32 as WORD64;
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
unsafe extern "C" fn ixheaac_negate32_sat(mut a: WORD32) -> WORD32 {
    let mut neg_val: WORD32 = 0;
    if a == MIN_32 {
        neg_val = MAX_32;
    } else {
        neg_val = -a;
    }
    return neg_val;
}
#[inline]
unsafe extern "C" fn ixheaac_div32(
    mut a: WORD32,
    mut b: WORD32,
    mut q_format: *mut WORD,
) -> WORD32 {
    let mut quotient: WORD32 = 0;
    let mut mantissa_nr: UWORD32 = 0;
    let mut mantissa_dr: UWORD32 = 0;
    let mut sign: WORD16 = 0 as WORD16;
    let mut i: LOOPINDEX = 0;
    let mut q_nr: WORD = 0;
    let mut q_dr: WORD = 0;
    if a < 0 as core::ffi::c_int && 0 as core::ffi::c_int != b {
        a = -a;
        sign = (sign as core::ffi::c_int ^ -(1 as core::ffi::c_int)) as WORD16;
    }
    if b < 0 as core::ffi::c_int {
        b = -b;
        sign = (sign as core::ffi::c_int ^ -(1 as core::ffi::c_int)) as WORD16;
    }
    if 0 as core::ffi::c_int == b {
        *q_format = 0 as core::ffi::c_int as WORD;
        return a;
    }
    quotient = 0 as core::ffi::c_int as WORD32;
    q_nr = ixheaac_norm32(a);
    mantissa_nr = (a as UWORD32) << q_nr;
    q_dr = ixheaac_norm32(b);
    mantissa_dr = (b as UWORD32) << q_dr;
    *q_format = 30 as WORD + q_nr - q_dr;
    i = 0 as core::ffi::c_int as LOOPINDEX;
    while i < 31 as core::ffi::c_int {
        quotient = quotient << 1 as core::ffi::c_int;
        if mantissa_nr >= mantissa_dr {
            mantissa_nr = mantissa_nr.wrapping_sub(mantissa_dr);
            quotient += 1 as core::ffi::c_int;
        }
        mantissa_nr = mantissa_nr << 1 as core::ffi::c_int;
        i += 1;
    }
    if (sign as core::ffi::c_int) < 0 as core::ffi::c_int {
        quotient = -quotient;
    }
    return quotient;
}
#[inline]
unsafe extern "C" fn ixheaac_mult32x16in32(mut a: WORD32, mut b: WORD16) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 16 as core::ffi::c_int) as WORD32;
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
pub const SQRT_TWO_Q15: core::ffi::c_int = 46341 as core::ffi::c_int;
pub const ONE_BY_THREE_Q15: core::ffi::c_int = 10923 as core::ffi::c_int;
pub const TWO_BY_THREE_Q15: core::ffi::c_int = 21845 as core::ffi::c_int;
pub const MINUS_ONE_BY_THREE_Q15: core::ffi::c_int = -(10923 as core::ffi::c_int);
pub const ONE_BY_SQRT_2_Q15: core::ffi::c_int = 23170 as core::ffi::c_int;
pub const ONE_BY_SQRT_8_Q15: core::ffi::c_int = 11585 as core::ffi::c_int;
pub const MINUS_ONE_IN_Q15: core::ffi::c_int = -(32768 as core::ffi::c_int);
pub const MINUS_ONE_IN_Q14: core::ffi::c_int = -(16384 as core::ffi::c_int);
pub const ONE_IN_Q14: core::ffi::c_int = 16384 as core::ffi::c_int;
pub const ONE_IN_Q15: core::ffi::c_int = 32768 as core::ffi::c_int;
pub const ONE_IN_Q16: core::ffi::c_int = 65536 as core::ffi::c_int;
pub const ONE_BY_SQRT_3_Q15: core::ffi::c_int = 18919 as core::ffi::c_int;
pub const INV_SQRT_2_Q31: core::ffi::c_int = 1518500250 as core::ffi::c_int;
pub const Q_SQRT_TAB: core::ffi::c_int = 15 as core::ffi::c_int;
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
unsafe extern "C" fn ixheaacd_mps_mult32_shr_n(
    mut a: WORD32,
    mut b: WORD32,
    mut n: WORD16,
) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> n as core::ffi::c_int) as WORD32;
    return result;
}
#[inline]
unsafe extern "C" fn ixheaacd_mps_mult32_shr_30(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 30 as core::ffi::c_int) as WORD32;
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
unsafe extern "C" fn ixheaacd_mps_convert_to_qn(
    mut temp: WORD32,
    mut qtemp: WORD16,
    mut n: WORD16,
) -> WORD32 {
    let mut result: WORD64 = 0;
    if qtemp as core::ffi::c_int == n as core::ffi::c_int {
        return temp
    } else if qtemp as core::ffi::c_int > n as core::ffi::c_int {
        temp = (temp as WORD64 >> qtemp as core::ffi::c_int - n as core::ffi::c_int)
            as WORD32;
    } else {
        result = ((temp as WORD64) << n as core::ffi::c_int - qtemp as core::ffi::c_int)
            as WORD32 as WORD64;
        if result > 0x7fffffff as core::ffi::c_int as WORD64
            || result < 0x80000000 as core::ffi::c_uint as WORD32 as WORD64
        {
            return 0 as WORD32
        } else {
            temp = result as WORD32;
        }
    }
    return temp;
}
#[inline]
unsafe extern "C" fn ixheaacd_mps_div32_in_q15(
    mut num: WORD32,
    mut den: WORD32,
) -> WORD32 {
    let mut quotient: WORD32 = 0;
    let mut q_quotient: WORD16 = 0;
    quotient = ixheaacd_mps_div_32(num, den, &mut q_quotient);
    quotient = ixheaacd_mps_convert_to_qn(quotient, q_quotient, 15 as WORD16);
    return quotient;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_get_matrix_inversion_weights(
    mut iid_lf_ls_idx: WORD32,
    mut iid_rf_rs_idx: WORD32,
    mut prediction_mode: WORD32,
    mut c1: WORD32,
    mut c2: WORD32,
    mut weight1: *mut WORD32,
    mut weight2: *mut WORD32,
    mut ia_mps_dec_mps_table_ptr: *mut ia_mps_dec_mps_tables_struct,
) -> VOID {
    let mut temp: WORD32 = 0;
    let mut temp_1: WORD32 = 0;
    let mut temp_2: WORD32 = 0;
    let mut temp_one: WORD32 = 0;
    let mut qtemp: WORD16 = 0;
    let mut w1: WORD32 = (*(*ia_mps_dec_mps_table_ptr).m1_m2_table_ptr)
        .cld_tab_2[(iid_lf_ls_idx as core::ffi::c_int + 15 as core::ffi::c_int)
        as usize];
    let mut w2: WORD32 = (*(*ia_mps_dec_mps_table_ptr).m1_m2_table_ptr)
        .cld_tab_2[(iid_rf_rs_idx as core::ffi::c_int + 15 as core::ffi::c_int)
        as usize];
    if prediction_mode == 1 as core::ffi::c_int {
        if if c1 < 0 as core::ffi::c_int {
            -(c1 as core::ffi::c_int)
        } else {
            (c1 >= ONE_IN_Q15) as core::ffi::c_int
        } != 0
        {
            c1 = ONE_IN_Q15 as WORD32;
        } else if c1 < MINUS_ONE_IN_Q14 && c1 > MINUS_ONE_IN_Q15 {
            c1 = MINUS_ONE_IN_Q15 - (c1 << 1 as core::ffi::c_int);
        } else {
            qtemp = 15 as WORD16;
            temp = ixheaacd_mps_mult32(TWO_BY_THREE_Q15, c1, &mut qtemp, 15 as WORD16);
            temp = ixheaacd_mps_convert_to_qn(temp, qtemp, 15 as WORD16);
            c1 = ONE_BY_THREE_Q15 + temp;
        }
        if if c2 < 0 as core::ffi::c_int {
            -(c2 as core::ffi::c_int)
        } else {
            (c2 >= ONE_IN_Q15) as core::ffi::c_int
        } != 0
        {
            c2 = ONE_IN_Q15 as WORD32;
        } else if c2 < MINUS_ONE_IN_Q14 && c2 > MINUS_ONE_IN_Q15 {
            c2 = MINUS_ONE_IN_Q15 - (c2 << 1 as core::ffi::c_int);
        } else {
            qtemp = 15 as WORD16;
            temp = ixheaacd_mps_mult32(TWO_BY_THREE_Q15, c2, &mut qtemp, 15 as WORD16);
            temp = ixheaacd_mps_convert_to_qn(temp, qtemp, 15 as WORD16);
            qtemp = 15 as WORD16;
            temp = ixheaacd_mps_add32(temp, ONE_BY_THREE_Q15, &mut qtemp, 15 as WORD16);
            c2 = ixheaacd_mps_convert_to_qn(temp, qtemp, 15 as WORD16);
        }
    } else {
        let mut c1p: WORD32 = 0;
        let mut c2p: WORD32 = 0;
        let mut acc: WORD64 = 0;
        let mut cld_tab_3: *const WORD32 = ((*(*ia_mps_dec_mps_table_ptr)
            .m1_m2_table_ptr)
            .cld_tab_3)
            .as_mut_ptr();
        let mut sqrt_tab: *const WORD32 = ((*(*ia_mps_dec_mps_table_ptr)
            .common_table_ptr)
            .sqrt_tab)
            .as_mut_ptr();
        c1p = *cld_tab_3
            .offset((c1 as core::ffi::c_int + 15 as core::ffi::c_int) as isize);
        c2p = *cld_tab_3
            .offset((c2 as core::ffi::c_int + 15 as core::ffi::c_int) as isize);
        acc = c1p as WORD64 * c2p as WORD64;
        acc >>= 15 as core::ffi::c_int;
        temp = acc as WORD32;
        temp_1 = ONE_IN_Q15 + c2p << 1 as core::ffi::c_int;
        acc += temp_1 as WORD64;
        temp_2 = acc as WORD32;
        temp = ixheaacd_mps_div_32(temp, temp_2, &mut qtemp);
        c1 = ixheaacd_mps_sqrt(temp, &mut qtemp, sqrt_tab);
        c1 = ixheaacd_mps_convert_to_qn(c1, qtemp, 15 as WORD16);
        temp_2 = ixheaac_add32_sat(c1p, temp_1);
        temp = ixheaacd_mps_div_32(c1p, temp_2, &mut qtemp);
        c2 = ixheaacd_mps_sqrt(temp, &mut qtemp, sqrt_tab);
        c2 = ixheaacd_mps_convert_to_qn(c2, qtemp, 15 as WORD16);
    }
    temp_one = ONE_IN_Q15 as WORD32;
    if ixheaac_norm32(w1) == 0 as core::ffi::c_int {
        temp_one = ONE_IN_Q14 as WORD32;
        w1 = w1 >> 1 as core::ffi::c_int;
    }
    temp_1 = temp_one + w1;
    temp_2 = ixheaacd_mps_mult32_shr_15(c1, w1);
    *weight1 = ixheaacd_mps_div32_in_q15(temp_2, temp_1);
    if ixheaac_norm32(w2) == 0 as core::ffi::c_int {
        temp_one = ONE_IN_Q14 as WORD32;
        w2 = w2 >> 1 as core::ffi::c_int;
    }
    temp_1 = temp_one + w2;
    temp_2 = ixheaacd_mps_mult32_shr_15(c2, w2);
    *weight2 = ixheaacd_mps_div32_in_q15(temp_2, temp_1);
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_invert_matrix(
    mut weight1: WORD32,
    mut weight2: WORD32,
    mut h_real: *mut [WORD32; 2],
    mut h_imag: *mut [WORD32; 2],
    mut common_tab_ptr: *const ia_mps_dec_common_tables_struct,
) -> VOID {
    let mut h11_f_real: WORD32 = 0;
    let mut h12_f_real: WORD32 = 0;
    let mut h21_f_real: WORD32 = 0;
    let mut h22_f_real: WORD32 = 0;
    let mut h11_f_imag: WORD32 = 0;
    let mut h12_f_imag: WORD32 = 0;
    let mut h21_f_imag: WORD32 = 0;
    let mut h22_f_imag: WORD32 = 0;
    let mut inv_norm_real: WORD32 = 0;
    let mut inv_norm_imag: WORD32 = 0;
    let mut inv_norm: WORD32 = 0;
    let mut len1: WORD32 = 0;
    let mut len2: WORD32 = 0;
    let mut q_len1: WORD16 = 0 as WORD16;
    let mut q_len2: WORD16 = 0 as WORD16;
    let mut acc1: WORD64 = 0;
    let mut acc2: WORD64 = 0;
    len1 = ixheaacd_mps_sqrt(
        ONE_IN_Q15 - (weight1 << 1 as core::ffi::c_int)
            + (weight1 * weight1 >> 14 as core::ffi::c_int),
        &mut q_len1,
        ((*common_tab_ptr).sqrt_tab).as_ptr(),
    );
    len2 = ixheaacd_mps_sqrt(
        ONE_IN_Q15 - (weight2 << 1 as core::ffi::c_int)
            + (weight2 * weight2 >> 14 as core::ffi::c_int),
        &mut q_len2,
        ((*common_tab_ptr).sqrt_tab).as_ptr(),
    );
    len1 = ixheaacd_mps_convert_to_qn(len1, q_len1, 15 as WORD16);
    len2 = ixheaacd_mps_convert_to_qn(len2, q_len2, 15 as WORD16);
    h11_f_real = ixheaacd_mps_div32_in_q15(ONE_IN_Q15 - weight1, len1);
    h11_f_imag = ixheaacd_mps_div32_in_q15(weight1, len1);
    h22_f_imag = -ixheaacd_mps_div32_in_q15(weight2, len2);
    h12_f_real = 0 as core::ffi::c_int as WORD32;
    h12_f_imag = ixheaacd_mps_mult32_shr_15(h22_f_imag, ONE_BY_SQRT_3_Q15);
    h21_f_real = 0 as core::ffi::c_int as WORD32;
    h21_f_imag = ixheaacd_mps_mult32_shr_15(h11_f_imag, -(18919 as WORD32));
    h22_f_real = ixheaacd_mps_div32_in_q15(ONE_IN_Q15 - weight2, len2);
    acc1 = h11_f_real as WORD64 * h22_f_real as WORD64
        - h11_f_imag as WORD64 * h22_f_imag as WORD64;
    acc1 >>= 15 as core::ffi::c_int;
    acc2 = h12_f_real as WORD64 * h21_f_real as WORD64
        - h12_f_imag as WORD64 * h21_f_imag as WORD64;
    acc2 >>= 15 as core::ffi::c_int;
    inv_norm_real = (acc1 - acc2) as WORD32;
    acc1 = h11_f_real as WORD64 * h22_f_imag as WORD64
        + h11_f_imag as WORD64 * h22_f_real as WORD64;
    acc1 >>= 15 as core::ffi::c_int;
    acc2 = h12_f_real as WORD64 * h21_f_imag as WORD64
        + h12_f_imag as WORD64 * h21_f_real as WORD64;
    acc2 >>= 15 as core::ffi::c_int;
    inv_norm_imag = (acc1 + acc2) as WORD32;
    acc1 = inv_norm_real as WORD64 * inv_norm_real as WORD64
        + inv_norm_imag as WORD64 * inv_norm_imag as WORD64;
    acc1 >>= 15 as core::ffi::c_int;
    inv_norm = acc1 as WORD32;
    inv_norm_real = ixheaacd_mps_div32_in_q15(inv_norm_real, inv_norm);
    inv_norm_imag = -ixheaacd_mps_div32_in_q15(inv_norm_imag, inv_norm);
    acc1 = h22_f_real as WORD64 * inv_norm_real as WORD64
        - h22_f_imag as WORD64 * inv_norm_imag as WORD64;
    acc1 >>= 15 as core::ffi::c_int;
    (*h_real.offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize] = acc1
        as WORD32;
    acc1 = h22_f_real as WORD64 * inv_norm_imag as WORD64
        + h22_f_imag as WORD64 * inv_norm_real as WORD64;
    acc1 >>= 15 as core::ffi::c_int;
    (*h_imag.offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize] = acc1
        as WORD32;
    acc1 = h12_f_imag as WORD64 * inv_norm_imag as WORD64
        - h12_f_real as WORD64 * inv_norm_real as WORD64;
    acc1 >>= 15 as core::ffi::c_int;
    (*h_real.offset(0 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize] = acc1
        as WORD32;
    acc1 = h12_f_real as WORD64 * inv_norm_imag as WORD64
        + h12_f_imag as WORD64 * inv_norm_real as WORD64;
    acc1 = -(acc1 >> 15 as core::ffi::c_int);
    (*h_imag.offset(0 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize] = acc1
        as WORD32;
    acc1 = h21_f_imag as WORD64 * inv_norm_imag as WORD64
        - h21_f_real as WORD64 * inv_norm_real as WORD64;
    acc1 >>= 15 as core::ffi::c_int;
    (*h_real.offset(1 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize] = acc1
        as WORD32;
    acc1 = h21_f_real as WORD64 * inv_norm_imag as WORD64
        + h21_f_imag as WORD64 * inv_norm_real as WORD64;
    acc1 = -(acc1 >> 15 as core::ffi::c_int);
    (*h_imag.offset(1 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize] = acc1
        as WORD32;
    acc1 = h11_f_real as WORD64 * inv_norm_real as WORD64
        - h11_f_imag as WORD64 * inv_norm_imag as WORD64;
    acc1 >>= 15 as core::ffi::c_int;
    (*h_real.offset(1 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize] = acc1
        as WORD32;
    acc1 = h11_f_real as WORD64 * inv_norm_imag as WORD64
        + h11_f_imag as WORD64 * inv_norm_real as WORD64;
    acc1 >>= 15 as core::ffi::c_int;
    (*h_imag.offset(1 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize] = acc1
        as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_dequant_icc_band(
    mut iccband: WORD32,
    mut cldband: WORD32,
) -> WORD32 {
    if iccband < 6 as core::ffi::c_int {
        return iccband;
    }
    if iccband == 6 as core::ffi::c_int {
        if cldband > 9 as core::ffi::c_int && cldband < 21 as core::ffi::c_int {
            match cldband {
                10 | 20 => return 10 as WORD32,
                11 | 19 => return 11 as WORD32,
                12 | 18 => return 12 as WORD32,
                13 | 17 => return 13 as WORD32,
                14 | 16 => return 14 as WORD32,
                15 => return 15 as WORD32,
                _ => return iccband,
            }
        } else {
            return iccband
        }
    }
    if 7 as core::ffi::c_int == iccband {
        if cldband > 7 as core::ffi::c_int && cldband < 23 as core::ffi::c_int {
            match cldband {
                8 | 22 => return 8 as WORD32,
                9 | 21 => return 9 as WORD32,
                10 | 20 => return 10 as WORD32,
                11 | 19 => return 11 as WORD32,
                12 | 18 => return 12 as WORD32,
                13 | 17 => return 13 as WORD32,
                14 | 16 => return 14 as WORD32,
                15 => return 15 as WORD32,
                _ => return iccband,
            }
        } else {
            return iccband
        }
    } else {
        return iccband
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_dequant_cld_band(mut cld: WORD32) -> WORD32 {
    match cld {
        -4915200 => return 0 as WORD32,
        -1474560 => return 1 as WORD32,
        -1310720 => return 2 as WORD32,
        -1146880 => return 3 as WORD32,
        -983040 => return 4 as WORD32,
        -819200 => return 5 as WORD32,
        -720896 => return 6 as WORD32,
        -622592 => return 7 as WORD32,
        -524288 => return 8 as WORD32,
        -425984 => return 9 as WORD32,
        -327680 => return 10 as WORD32,
        -262144 => return 11 as WORD32,
        -196608 => return 12 as WORD32,
        -131072 => return 13 as WORD32,
        -65536 => return 14 as WORD32,
        0 => return 15 as WORD32,
        65536 => return 16 as WORD32,
        131072 => return 17 as WORD32,
        196608 => return 18 as WORD32,
        262144 => return 19 as WORD32,
        327680 => return 20 as WORD32,
        425984 => return 21 as WORD32,
        524288 => return 22 as WORD32,
        622592 => return 23 as WORD32,
        720896 => return 24 as WORD32,
        819200 => return 25 as WORD32,
        983040 => return 26 as WORD32,
        1146880 => return 27 as WORD32,
        1310720 => return 28 as WORD32,
        1474560 => return 29 as WORD32,
        4915200 => return 30 as WORD32,
        _ => return 0 as WORD32,
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_param_2_umx_ps_core_tables(
    mut cld: *mut WORD32,
    mut icc: *mut WORD32,
    mut num_ott_bands: WORD32,
    mut res_bands: WORD32,
    mut h11: *mut WORD32,
    mut h12: *mut WORD32,
    mut h21: *mut WORD32,
    mut h22: *mut WORD32,
    mut h12_res: *mut WORD32,
    mut h22_res: *mut WORD32,
    mut c_l: *mut WORD16,
    mut c_r: *mut WORD16,
    mut ixheaacd_mps_dec_m1_m2_tables: *const ia_mps_dec_m1_m2_tables_struct,
) -> VOID {
    let mut band: WORD32 = 0;
    let mut quant_band_cld: WORD32 = 0;
    let mut quant_band_icc: WORD32 = 0;
    band = 0 as core::ffi::c_int as WORD32;
    while band < num_ott_bands {
        quant_band_cld = ixheaacd_dequant_cld_band(*cld.offset(band as isize));
        *c_l.offset(band as isize) = (*ixheaacd_mps_dec_m1_m2_tables)
            .c_l_table[quant_band_cld as usize] as WORD16;
        *c_r.offset(band as isize) = (*ixheaacd_mps_dec_m1_m2_tables)
            .c_l_table[(30 as WORD32 - quant_band_cld) as usize] as WORD16;
        band += 1;
    }
    band = 0 as core::ffi::c_int as WORD32;
    while band < num_ott_bands {
        if band < res_bands {
            quant_band_cld = ixheaacd_dequant_cld_band(*cld.offset(band as isize));
            quant_band_icc = ixheaacd_dequant_icc_band(
                *icc.offset(band as isize),
                quant_band_cld,
            );
            *h11.offset(band as isize) = (*ixheaacd_mps_dec_m1_m2_tables)
                .cos_table[quant_band_icc as usize][quant_band_cld as usize];
            *h11.offset(band as isize) = ixheaacd_mps_mult32_shr_15(
                *h11.offset(band as isize),
                *c_l.offset(band as isize) as WORD32,
            );
            *h21.offset(band as isize) = (*ixheaacd_mps_dec_m1_m2_tables)
                .cos_table[quant_band_icc
                as usize][(30 as WORD32 - quant_band_cld) as usize];
            *h21.offset(band as isize) = ixheaacd_mps_mult32_shr_15(
                *h21.offset(band as isize),
                *c_r.offset(band as isize) as WORD32,
            );
            *h12.offset(band as isize) = 0 as core::ffi::c_int as WORD32;
            *h22.offset(band as isize) = 0 as core::ffi::c_int as WORD32;
            *h12_res.offset(band as isize) = ONE_IN_Q15 as WORD32;
            *h22_res.offset(band as isize) = MINUS_ONE_IN_Q15 as WORD32;
        } else {
            quant_band_cld = ixheaacd_dequant_cld_band(*cld.offset(band as isize));
            if quant_band_cld < 0 as core::ffi::c_int
                || quant_band_cld >= 31 as core::ffi::c_int
            {
                quant_band_cld = 30 as core::ffi::c_int as WORD32;
            }
            quant_band_icc = *icc.offset(band as isize);
            if quant_band_icc < 0 as core::ffi::c_int
                || quant_band_icc >= 8 as core::ffi::c_int
            {
                quant_band_icc = 7 as core::ffi::c_int as WORD32;
            }
            *h11.offset(band as isize) = (*ixheaacd_mps_dec_m1_m2_tables)
                .cos_table[quant_band_icc as usize][quant_band_cld as usize];
            *h11.offset(band as isize) = ixheaacd_mps_mult32_shr_15(
                *h11.offset(band as isize),
                *c_l.offset(band as isize) as WORD32,
            );
            *h21.offset(band as isize) = (*ixheaacd_mps_dec_m1_m2_tables)
                .cos_table[quant_band_icc
                as usize][(30 as WORD32 - quant_band_cld) as usize];
            *h21.offset(band as isize) = ixheaacd_mps_mult32_shr_15(
                *h21.offset(band as isize),
                *c_r.offset(band as isize) as WORD32,
            );
            *h12.offset(band as isize) = (*ixheaacd_mps_dec_m1_m2_tables)
                .sin_table[quant_band_icc as usize][quant_band_cld as usize];
            *h12.offset(band as isize) = ixheaacd_mps_mult32_shr_15(
                *h12.offset(band as isize),
                *c_l.offset(band as isize) as WORD32,
            );
            *h22.offset(band as isize) = -(*ixheaacd_mps_dec_m1_m2_tables)
                .sin_table[quant_band_icc
                as usize][(30 as WORD32 - quant_band_cld) as usize];
            *h22.offset(band as isize) = ixheaacd_mps_mult32_shr_15(
                *h22.offset(band as isize),
                *c_r.offset(band as isize) as WORD32,
            );
            *h12_res.offset(band as isize) = 0 as core::ffi::c_int as WORD32;
            *h22_res.offset(band as isize) = 0 as core::ffi::c_int as WORD32;
        }
        band += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_param_2_umx_ps(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
    mut h11: *mut WORD32,
    mut h12: *mut WORD32,
    mut h21: *mut WORD32,
    mut h22: *mut WORD32,
    mut h12_res: *mut WORD32,
    mut h22_res: *mut WORD32,
    mut c_l: *mut WORD16,
    mut c_r: *mut WORD16,
    mut ott_box_indx: WORD32,
    mut parameter_set_indx: WORD32,
    mut res_bands: WORD32,
) -> VOID {
    let mut band: WORD32 = 0;
    let mut p_cur_bs: *mut ia_mps_dec_spatial_bs_frame_struct = (*pstr_mps_state)
        .bs_frame;
    let mut p_aux_struct: *mut ia_mps_dec_auxilary_struct = (*pstr_mps_state).aux_struct;
    let mut num_parameter_bands: WORD32 = (*pstr_mps_state).num_parameter_bands;
    ixheaacd_param_2_umx_ps_core_tables(
        ((*p_aux_struct).ott_cld[ott_box_indx as usize][parameter_set_indx as usize])
            .as_mut_ptr(),
        ((*p_cur_bs).ott_icc_idx[ott_box_indx as usize][parameter_set_indx as usize])
            .as_mut_ptr(),
        (*p_aux_struct).num_ott_bands[ott_box_indx as usize],
        res_bands,
        h11 as *mut WORD32,
        h12 as *mut WORD32,
        h21 as *mut WORD32,
        h22 as *mut WORD32,
        h12_res as *mut WORD32,
        h22_res as *mut WORD32,
        c_l as *mut WORD16,
        c_r as *mut WORD16,
        (*pstr_mps_state).ia_mps_dec_mps_table.m1_m2_table_ptr,
    );
    band = (*p_aux_struct).num_ott_bands[ott_box_indx as usize];
    while band < num_parameter_bands {
        let ref mut fresh0 = *h22_res.offset(band as isize);
        *fresh0 = 0 as core::ffi::c_int as WORD32;
        let ref mut fresh1 = *h12_res.offset(band as isize);
        *fresh1 = *fresh0;
        let ref mut fresh2 = *h22.offset(band as isize);
        *fresh2 = *fresh1;
        let ref mut fresh3 = *h12.offset(band as isize);
        *fresh3 = *fresh2;
        let ref mut fresh4 = *h21.offset(band as isize);
        *fresh4 = *fresh3;
        *h11.offset(band as isize) = *fresh4;
        band += 1;
    }
}
unsafe extern "C" fn ixheaacd_dequant_one_by_icc(mut icc: WORD32) -> WORD32 {
    match icc {
        32768 => return 32768 as WORD32,
        30704 => return 34971 as WORD32,
        27564 => return 38955 as WORD32,
        19691 => return 54530 as WORD32,
        12047 => return 89131 as WORD32,
        0 => return 0 as WORD32,
        -19300 => return -(55633 as WORD32),
        -32440 => return -(33099 as WORD32),
        _ => return 0 as WORD32,
    };
}
unsafe extern "C" fn ixheaacd_map_cld_index(mut cld_val: WORD32) -> WORD16 {
    let mut temp: WORD32 = cld_val;
    let mut idx: WORD16 = 0 as WORD16;
    if cld_val == 0 as core::ffi::c_int {
        return 15 as WORD16
    } else {
        if cld_val < 0 as core::ffi::c_int {
            temp = -cld_val;
        }
        match temp {
            150 => {
                idx = 15 as WORD16;
            }
            45 => {
                idx = 14 as WORD16;
            }
            40 => {
                idx = 13 as WORD16;
            }
            35 => {
                idx = 12 as WORD16;
            }
            30 => {
                idx = 11 as WORD16;
            }
            25 => {
                idx = 10 as WORD16;
            }
            22 => {
                idx = 9 as WORD16;
            }
            19 => {
                idx = 8 as WORD16;
            }
            16 => {
                idx = 7 as WORD16;
            }
            13 => {
                idx = 6 as WORD16;
            }
            10 => {
                idx = 5 as WORD16;
            }
            8 => {
                idx = 4 as WORD16;
            }
            6 => {
                idx = 3 as WORD16;
            }
            4 => {
                idx = 2 as WORD16;
            }
            2 => {
                idx = 1 as WORD16;
            }
            _ => {
                idx = 0 as WORD16;
            }
        }
    }
    return (if cld_val >= 0 as core::ffi::c_int {
        idx as core::ffi::c_int + 15 as core::ffi::c_int
    } else {
        15 as core::ffi::c_int - idx as core::ffi::c_int
    }) as WORD16;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_calculate_ttt(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
    mut ps: WORD32,
    mut pb: WORD32,
    mut ttt_mode: WORD32,
    mut m_ttt: *mut [WORD32; 3],
) -> VOID {
    let mut p_aux_struct: *mut ia_mps_dec_auxilary_struct = (*pstr_mps_state).aux_struct;
    let mut col: WORD32 = 0;
    if ttt_mode < 2 as core::ffi::c_int {
        (*m_ttt
            .offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize] = ((*p_aux_struct)
            .ttt_cpc_1[0 as core::ffi::c_int as usize][ps as usize][pb as usize]
            + ONE_IN_Q16) as WORD32;
        (*m_ttt
            .offset(0 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize] = ((*p_aux_struct)
            .ttt_cpc_2[0 as core::ffi::c_int as usize][ps as usize][pb as usize]
            - ONE_IN_Q15) as WORD32;
        (*m_ttt
            .offset(1 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize] = ((*p_aux_struct)
            .ttt_cpc_1[0 as core::ffi::c_int as usize][ps as usize][pb as usize]
            - ONE_IN_Q15) as WORD32;
        (*m_ttt
            .offset(1 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize] = ((*p_aux_struct)
            .ttt_cpc_2[0 as core::ffi::c_int as usize][ps as usize][pb as usize]
            + ONE_IN_Q16) as WORD32;
        (*m_ttt
            .offset(2 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize] = ONE_IN_Q15
            - (*p_aux_struct)
                .ttt_cpc_1[0 as core::ffi::c_int as usize][ps as usize][pb as usize];
        (*m_ttt
            .offset(2 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize] = ONE_IN_Q15
            - (*p_aux_struct)
                .ttt_cpc_2[0 as core::ffi::c_int as usize][ps as usize][pb as usize];
        if pb >= (*pstr_mps_state).res_bands[3 as core::ffi::c_int as usize] {
            let mut one_by_icc: WORD32 = 0;
            one_by_icc = ixheaacd_dequant_one_by_icc(
                (*p_aux_struct)
                    .ttt_icc[0 as core::ffi::c_int as usize][ps as usize][pb as usize],
            );
            (*m_ttt
                .offset(
                    0 as core::ffi::c_int as isize,
                ))[0 as core::ffi::c_int as usize] = ixheaacd_mps_mult32_shr_15(
                (*m_ttt
                    .offset(
                        0 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize],
                one_by_icc,
            );
            (*m_ttt
                .offset(
                    0 as core::ffi::c_int as isize,
                ))[1 as core::ffi::c_int as usize] = ixheaacd_mps_mult32_shr_15(
                (*m_ttt
                    .offset(
                        0 as core::ffi::c_int as isize,
                    ))[1 as core::ffi::c_int as usize],
                one_by_icc,
            );
            (*m_ttt
                .offset(
                    1 as core::ffi::c_int as isize,
                ))[0 as core::ffi::c_int as usize] = ixheaacd_mps_mult32_shr_15(
                (*m_ttt
                    .offset(
                        1 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize],
                one_by_icc,
            );
            (*m_ttt
                .offset(
                    1 as core::ffi::c_int as isize,
                ))[1 as core::ffi::c_int as usize] = ixheaacd_mps_mult32_shr_15(
                (*m_ttt
                    .offset(
                        1 as core::ffi::c_int as isize,
                    ))[1 as core::ffi::c_int as usize],
                one_by_icc,
            );
            (*m_ttt
                .offset(
                    2 as core::ffi::c_int as isize,
                ))[0 as core::ffi::c_int as usize] = ixheaacd_mps_mult32_shr_15(
                (*m_ttt
                    .offset(
                        2 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize],
                one_by_icc,
            );
            (*m_ttt
                .offset(
                    2 as core::ffi::c_int as isize,
                ))[1 as core::ffi::c_int as usize] = ixheaacd_mps_mult32_shr_15(
                (*m_ttt
                    .offset(
                        2 as core::ffi::c_int as isize,
                    ))[1 as core::ffi::c_int as usize],
                one_by_icc,
            );
        }
        (*m_ttt
            .offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize] = ixheaac_mult32x16in32(
            (*m_ttt
                .offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize],
            TWO_BY_THREE_Q15 as WORD16,
        );
        (*m_ttt
            .offset(0 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize] = ixheaac_mult32x16in32(
            (*m_ttt
                .offset(0 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize],
            TWO_BY_THREE_Q15 as WORD16,
        );
        (*m_ttt
            .offset(1 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize] = ixheaac_mult32x16in32(
            (*m_ttt
                .offset(1 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize],
            TWO_BY_THREE_Q15 as WORD16,
        );
        (*m_ttt
            .offset(1 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize] = ixheaac_mult32x16in32(
            (*m_ttt
                .offset(1 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize],
            TWO_BY_THREE_Q15 as WORD16,
        );
        (*m_ttt
            .offset(2 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize] = ixheaac_mult32x16in32(
            (*m_ttt
                .offset(2 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize],
            TWO_BY_THREE_Q15 as WORD16,
        );
        (*m_ttt
            .offset(2 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize] = ixheaac_mult32x16in32(
            (*m_ttt
                .offset(2 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize],
            TWO_BY_THREE_Q15 as WORD16,
        );
    } else {
        let mut center_wiener: WORD32 = 0;
        let mut center_subtraction: WORD32 = 0;
        let mut c1d: WORD32 = 0;
        let mut c2d: WORD32 = 0;
        let mut prod: WORD64 = 0;
        let mut w11: WORD32 = 0;
        let mut w00: WORD32 = 0;
        let mut w20: WORD32 = 0;
        let mut w21: WORD32 = 0;
        let mut q_w11: WORD16 = 0;
        let mut q_w00: WORD16 = 0;
        let mut q_w20: WORD16 = 0;
        let mut q_w21: WORD16 = 0;
        let mut ten_cld_by_10: *const WORD32 = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .m1_m2_table_ptr)
            .ten_cld_by_10)
            .as_mut_ptr();
        let mut p_cur_bs: *mut ia_mps_dec_spatial_bs_frame_struct = (*pstr_mps_state)
            .bs_frame;
        let mut index: WORD16 = ixheaacd_map_cld_index(
            (*p_aux_struct)
                .ttt_cld_1[0 as core::ffi::c_int as usize][ps as usize][pb as usize]
                >> 15 as core::ffi::c_int,
        );
        c1d = *ten_cld_by_10.offset(index as isize);
        index = ixheaacd_map_cld_index(
            (*p_aux_struct)
                .ttt_cld_2[0 as core::ffi::c_int as usize][ps as usize][pb as usize]
                >> 15 as core::ffi::c_int,
        );
        c2d = *ten_cld_by_10.offset(index as isize);
        if (*p_cur_bs)
            .cmp_ttt_cld_1_idx[0 as core::ffi::c_int as usize][ps as usize][pb as usize]
            == 15 as core::ffi::c_int
            || (*p_cur_bs)
                .cmp_ttt_cld_2_idx[0 as core::ffi::c_int
                as usize][ps as usize][pb as usize] == 15 as core::ffi::c_int
        {
            if (*p_cur_bs)
                .cmp_ttt_cld_1_idx[0 as core::ffi::c_int
                as usize][ps as usize][pb as usize] == 15 as core::ffi::c_int
            {
                if (*p_cur_bs)
                    .cmp_ttt_cld_2_idx[0 as core::ffi::c_int
                    as usize][ps as usize][pb as usize] == -(15 as core::ffi::c_int)
                {
                    w00 = ONE_BY_SQRT_2_Q15 as WORD32;
                    w20 = ONE_BY_SQRT_8_Q15 as WORD32;
                } else {
                    w00 = ONE_IN_Q15 as WORD32;
                    w20 = 0 as core::ffi::c_int as WORD32;
                }
                if (*p_cur_bs)
                    .cmp_ttt_cld_2_idx[0 as core::ffi::c_int
                    as usize][ps as usize][pb as usize] == 15 as core::ffi::c_int
                {
                    w11 = ONE_BY_SQRT_2_Q15 as WORD32;
                    w21 = ONE_BY_SQRT_8_Q15 as WORD32;
                } else {
                    w11 = ONE_IN_Q15 as WORD32;
                    w21 = 0 as core::ffi::c_int as WORD32;
                }
                (*m_ttt
                    .offset(
                        0 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize] = w00;
                (*m_ttt
                    .offset(
                        2 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize] = w20;
                (*m_ttt
                    .offset(
                        2 as core::ffi::c_int as isize,
                    ))[1 as core::ffi::c_int as usize] = w21;
                (*m_ttt
                    .offset(
                        1 as core::ffi::c_int as isize,
                    ))[1 as core::ffi::c_int as usize] = w11;
            }
            if (*p_cur_bs)
                .cmp_ttt_cld_2_idx[0 as core::ffi::c_int
                as usize][ps as usize][pb as usize] == 15 as core::ffi::c_int
            {
                let mut w00_cld2_15: *const WORD32 = ((*(*pstr_mps_state)
                    .ia_mps_dec_mps_table
                    .m1_m2_table_ptr)
                    .w00_cld2_15)
                    .as_mut_ptr();
                if (*p_cur_bs)
                    .cmp_ttt_cld_1_idx[0 as core::ffi::c_int
                    as usize][ps as usize][pb as usize] == 15 as core::ffi::c_int
                {
                    w11 = ONE_BY_SQRT_2_Q15 as WORD32;
                    w21 = ONE_BY_SQRT_8_Q15 as WORD32;
                } else {
                    w11 = 0 as core::ffi::c_int as WORD32;
                    w21 = ONE_IN_Q14 as WORD32;
                }
                w00 = *w00_cld2_15
                    .offset(
                        ((*p_cur_bs)
                            .cmp_ttt_cld_1_idx[0 as core::ffi::c_int
                            as usize][ps as usize][pb as usize] + 15 as core::ffi::c_int)
                            as isize,
                    );
                w20 = (*w00_cld2_15
                    .offset(
                        (15 as WORD32
                            - (*p_cur_bs)
                                .cmp_ttt_cld_1_idx[0 as core::ffi::c_int
                                as usize][ps as usize][pb as usize]) as isize,
                    ) / 2 as core::ffi::c_int) as WORD32;
                (*m_ttt
                    .offset(
                        0 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize] = w00;
                (*m_ttt
                    .offset(
                        2 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize] = w20;
                (*m_ttt
                    .offset(
                        2 as core::ffi::c_int as isize,
                    ))[1 as core::ffi::c_int as usize] = w21;
                (*m_ttt
                    .offset(
                        1 as core::ffi::c_int as isize,
                    ))[1 as core::ffi::c_int as usize] = w11;
            }
            (*m_ttt
                .offset(
                    0 as core::ffi::c_int as isize,
                ))[1 as core::ffi::c_int as usize] = 0 as core::ffi::c_int as WORD32;
            (*m_ttt
                .offset(
                    1 as core::ffi::c_int as isize,
                ))[0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int as WORD32;
        } else {
            let mut temporary: WORD32 = 0;
            let mut sqrt_tab: *const WORD32 = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .common_table_ptr)
                .sqrt_tab)
                .as_mut_ptr();
            prod = ixheaacd_mps_mult32_shr_15(c1d, c2d) as WORD64;
            temporary = ixheaac_add32_sat(ONE_IN_Q15, c2d);
            temporary = ixheaac_add32_sat(temporary, prod as WORD32);
            w00 = ixheaacd_mps_div_32(prod as WORD32, temporary, &mut q_w00);
            w11 = ixheaacd_mps_div_32(
                c1d,
                ixheaac_add32_sat3(c1d, c2d, ONE_IN_Q15),
                &mut q_w11,
            );
            w20 = ixheaacd_mps_div_32(
                ixheaac_add32_sat(c2d, ONE_IN_Q15),
                ixheaac_add32_sat3(ONE_IN_Q15, prod as WORD32, c2d),
                &mut q_w20,
            );
            w21 = ixheaacd_mps_div_32(
                ixheaac_add32_sat(c2d, ONE_IN_Q15),
                ixheaac_add32_sat3(c1d, c2d, ONE_IN_Q15),
                &mut q_w21,
            );
            (*m_ttt
                .offset(
                    0 as core::ffi::c_int as isize,
                ))[0 as core::ffi::c_int as usize] = ixheaacd_mps_sqrt(
                w00,
                &mut q_w00,
                sqrt_tab,
            );
            (*m_ttt
                .offset(
                    0 as core::ffi::c_int as isize,
                ))[0 as core::ffi::c_int as usize] = ixheaacd_mps_convert_to_qn(
                (*m_ttt
                    .offset(
                        0 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize],
                q_w00,
                15 as WORD16,
            );
            (*m_ttt
                .offset(
                    0 as core::ffi::c_int as isize,
                ))[1 as core::ffi::c_int as usize] = 0 as core::ffi::c_int as WORD32;
            (*m_ttt
                .offset(
                    1 as core::ffi::c_int as isize,
                ))[0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int as WORD32;
            (*m_ttt
                .offset(
                    1 as core::ffi::c_int as isize,
                ))[1 as core::ffi::c_int as usize] = ixheaacd_mps_sqrt(
                w11,
                &mut q_w11,
                sqrt_tab,
            );
            (*m_ttt
                .offset(
                    1 as core::ffi::c_int as isize,
                ))[1 as core::ffi::c_int as usize] = ixheaacd_mps_convert_to_qn(
                (*m_ttt
                    .offset(
                        1 as core::ffi::c_int as isize,
                    ))[1 as core::ffi::c_int as usize],
                q_w11,
                15 as WORD16,
            );
            (*m_ttt
                .offset(
                    2 as core::ffi::c_int as isize,
                ))[0 as core::ffi::c_int as usize] = ixheaacd_mps_sqrt(
                w20,
                &mut q_w20,
                sqrt_tab,
            ) >> 1 as core::ffi::c_int;
            (*m_ttt
                .offset(
                    2 as core::ffi::c_int as isize,
                ))[0 as core::ffi::c_int as usize] = ixheaacd_mps_convert_to_qn(
                (*m_ttt
                    .offset(
                        2 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize],
                q_w20,
                15 as WORD16,
            );
            (*m_ttt
                .offset(
                    2 as core::ffi::c_int as isize,
                ))[1 as core::ffi::c_int as usize] = ixheaacd_mps_sqrt(
                w21,
                &mut q_w21,
                sqrt_tab,
            ) >> 1 as core::ffi::c_int;
            (*m_ttt
                .offset(
                    2 as core::ffi::c_int as isize,
                ))[1 as core::ffi::c_int as usize] = ixheaacd_mps_convert_to_qn(
                (*m_ttt
                    .offset(
                        2 as core::ffi::c_int as isize,
                    ))[1 as core::ffi::c_int as usize],
                q_w21,
                15 as WORD16,
            );
            if (*p_aux_struct)
                .ttt_cld_1[0 as core::ffi::c_int as usize][ps as usize][pb as usize]
                == 4915200 as core::ffi::c_int
            {
                (*m_ttt
                    .offset(
                        0 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize] = 32767 as core::ffi::c_int
                    as WORD32;
                (*m_ttt
                    .offset(
                        1 as core::ffi::c_int as isize,
                    ))[1 as core::ffi::c_int as usize] = 32767 as core::ffi::c_int
                    as WORD32;
                (*m_ttt
                    .offset(
                        2 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int as WORD32;
                (*m_ttt
                    .offset(
                        2 as core::ffi::c_int as isize,
                    ))[1 as core::ffi::c_int as usize] = 0 as core::ffi::c_int as WORD32;
            }
        }
        center_wiener = 0 as core::ffi::c_int as WORD32;
        center_subtraction = (ttt_mode == 2 as core::ffi::c_int
            || ttt_mode == 3 as core::ffi::c_int) as core::ffi::c_int as WORD32;
        if center_wiener != 0 {
            let mut cld_1_idx: WORD32 = (*p_cur_bs)
                .cmp_ttt_cld_1_idx[0 as core::ffi::c_int
                as usize][ps as usize][pb as usize];
            let mut cld_2_idx: WORD32 = (*p_cur_bs)
                .cmp_ttt_cld_2_idx[0 as core::ffi::c_int
                as usize][ps as usize][pb as usize];
            if cld_1_idx == 15 as core::ffi::c_int && cld_2_idx == 15 as core::ffi::c_int
            {
                (*m_ttt
                    .offset(
                        2 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int as WORD32;
                (*m_ttt
                    .offset(
                        2 as core::ffi::c_int as isize,
                    ))[1 as core::ffi::c_int as usize] = ONE_BY_SQRT_2_Q15 as WORD32;
            } else if cld_1_idx == 15 as core::ffi::c_int {
                if cld_2_idx == -(15 as core::ffi::c_int) {
                    (*m_ttt
                        .offset(
                            2 as core::ffi::c_int as isize,
                        ))[0 as core::ffi::c_int as usize] = ONE_BY_SQRT_2_Q15 as WORD32;
                } else {
                    (*m_ttt
                        .offset(
                            2 as core::ffi::c_int as isize,
                        ))[0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
                        as WORD32;
                }
                (*m_ttt
                    .offset(
                        2 as core::ffi::c_int as isize,
                    ))[1 as core::ffi::c_int as usize] = 0 as core::ffi::c_int as WORD32;
            } else if cld_2_idx == 15 as core::ffi::c_int {
                (*m_ttt
                    .offset(
                        2 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int as WORD32;
                (*m_ttt
                    .offset(
                        2 as core::ffi::c_int as isize,
                    ))[1 as core::ffi::c_int as usize] = ONE_IN_Q15 as WORD32;
            } else {
                let mut temp: WORD32 = 0;
                let mut q_temp: WORD16 = 0;
                let mut sqrt_tab_0: *const WORD32 = ((*(*pstr_mps_state)
                    .ia_mps_dec_mps_table
                    .common_table_ptr)
                    .sqrt_tab)
                    .as_mut_ptr();
                prod = (ixheaacd_mps_mult32_shr_15(c2d, c2d + c1d + ONE_IN_Q16)
                    as core::ffi::c_int + ONE_IN_Q15) as WORD64;
                temp = ixheaacd_mps_div_32(ONE_IN_Q15, prod as WORD32, &mut q_temp);
                (*m_ttt
                    .offset(
                        2 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize] = ixheaacd_mps_sqrt(
                    temp,
                    &mut q_temp,
                    sqrt_tab_0,
                );
                (*m_ttt
                    .offset(
                        2 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize] = ixheaacd_mps_convert_to_qn(
                    (*m_ttt
                        .offset(
                            2 as core::ffi::c_int as isize,
                        ))[0 as core::ffi::c_int as usize],
                    q_temp,
                    15 as WORD16,
                );
                (*m_ttt
                    .offset(
                        2 as core::ffi::c_int as isize,
                    ))[1 as core::ffi::c_int as usize] = ixheaacd_mps_mult32_shr_15(
                    c2d,
                    (*m_ttt
                        .offset(
                            2 as core::ffi::c_int as isize,
                        ))[0 as core::ffi::c_int as usize],
                );
            }
        }
        if center_subtraction != 0 {
            let mut wl1: WORD32 = 0;
            let mut wl2: WORD32 = 0;
            let mut wr1: WORD32 = 0;
            let mut wr2: WORD32 = 0;
            let mut q_wl1: WORD16 = 0;
            let mut q_wr1: WORD16 = 0;
            let mut cld_1_idx_0: WORD32 = (*p_cur_bs)
                .cmp_ttt_cld_1_idx[0 as core::ffi::c_int
                as usize][ps as usize][pb as usize];
            let mut cld_2_idx_0: WORD32 = (*p_cur_bs)
                .cmp_ttt_cld_2_idx[0 as core::ffi::c_int
                as usize][ps as usize][pb as usize];
            if cld_1_idx_0 == 15 as core::ffi::c_int
                && cld_2_idx_0 == 15 as core::ffi::c_int
            {
                (*m_ttt
                    .offset(
                        0 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize] = ONE_IN_Q15 as WORD32;
                (*m_ttt
                    .offset(
                        0 as core::ffi::c_int as isize,
                    ))[1 as core::ffi::c_int as usize] = MINUS_ONE_IN_Q14 as WORD32;
                (*m_ttt
                    .offset(
                        1 as core::ffi::c_int as isize,
                    ))[1 as core::ffi::c_int as usize] = ONE_BY_SQRT_2_Q15 as WORD32;
                (*m_ttt
                    .offset(
                        1 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int as WORD32;
            } else if cld_1_idx_0 == 15 as core::ffi::c_int {
                if cld_2_idx_0 == -(15 as core::ffi::c_int) {
                    (*m_ttt
                        .offset(
                            0 as core::ffi::c_int as isize,
                        ))[0 as core::ffi::c_int as usize] = ONE_BY_SQRT_2_Q15 as WORD32;
                    (*m_ttt
                        .offset(
                            1 as core::ffi::c_int as isize,
                        ))[0 as core::ffi::c_int as usize] = MINUS_ONE_IN_Q14 as WORD32;
                } else {
                    (*m_ttt
                        .offset(
                            0 as core::ffi::c_int as isize,
                        ))[0 as core::ffi::c_int as usize] = ONE_IN_Q15 as WORD32;
                    (*m_ttt
                        .offset(
                            1 as core::ffi::c_int as isize,
                        ))[0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
                        as WORD32;
                }
                (*m_ttt
                    .offset(
                        0 as core::ffi::c_int as isize,
                    ))[1 as core::ffi::c_int as usize] = 0 as core::ffi::c_int as WORD32;
                (*m_ttt
                    .offset(
                        1 as core::ffi::c_int as isize,
                    ))[1 as core::ffi::c_int as usize] = ONE_IN_Q15 as WORD32;
            } else if cld_2_idx_0 == 15 as core::ffi::c_int {
                (*m_ttt
                    .offset(
                        0 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize] = ONE_IN_Q15 as WORD32;
                (*m_ttt
                    .offset(
                        0 as core::ffi::c_int as isize,
                    ))[1 as core::ffi::c_int as usize] = MINUS_ONE_IN_Q15 as WORD32;
                (*m_ttt
                    .offset(
                        1 as core::ffi::c_int as isize,
                    ))[1 as core::ffi::c_int as usize] = 0 as core::ffi::c_int as WORD32;
                (*m_ttt
                    .offset(
                        1 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int as WORD32;
            } else {
                let mut temp_0: WORD32 = 0;
                let mut temp_1: WORD32 = 0;
                let mut q_a: WORD32 = 0;
                let mut q_c: WORD16 = 0;
                let mut q_l: WORD16 = 0;
                let mut q_r: WORD16 = 0;
                let mut q_temp_0: WORD16 = 0;
                let mut q_temp1: WORD16 = 0;
                let mut c: WORD32 = 0;
                let mut r: WORD32 = 0;
                let mut l: WORD32 = 0;
                let mut sqrt_tab_1: *const WORD32 = ((*(*pstr_mps_state)
                    .ia_mps_dec_mps_table
                    .common_table_ptr)
                    .sqrt_tab)
                    .as_mut_ptr();
                c = ixheaacd_mps_div_32(
                    ONE_IN_Q15,
                    ixheaac_add32_sat(c1d, ONE_IN_Q15),
                    &mut q_c,
                );
                r = ixheaacd_mps_div_32(
                    c1d,
                    ixheaac_add32_sat(c2d, ONE_IN_Q15),
                    &mut q_r,
                );
                r = ixheaacd_mps_mult32_shr_30(r, c);
                q_r = (q_r as core::ffi::c_int + q_c as core::ffi::c_int
                    - 30 as core::ffi::c_int) as WORD16;
                l = ixheaacd_mps_mult32_shr_30(c2d, r);
                q_l = (q_r as core::ffi::c_int - 15 as core::ffi::c_int) as WORD16;
                temp_0 = ixheaacd_mps_div_32(r, l, &mut q_temp_0);
                q_temp_0 = (q_temp_0 as core::ffi::c_int
                    + (q_r as core::ffi::c_int - q_l as core::ffi::c_int)) as WORD16;
                if q_temp_0 as core::ffi::c_int > 28 as core::ffi::c_int {
                    temp_0 = temp_0
                        >> q_temp_0 as core::ffi::c_int - 28 as core::ffi::c_int;
                    q_temp_0 = 28 as WORD16;
                }
                temp_0
                    += ((1 as core::ffi::c_int) << q_temp_0 as core::ffi::c_int)
                        - 1 as core::ffi::c_int;
                temp_0 = ixheaac_add32_sat(
                    ixheaacd_mps_mult32_shr_n(
                        c,
                        temp_0,
                        (q_c as core::ffi::c_int + q_temp_0 as core::ffi::c_int
                            - q_r as core::ffi::c_int) as WORD16,
                    ),
                    r,
                );
                q_temp_0 = q_r;
                if q_c as core::ffi::c_int > q_r as core::ffi::c_int {
                    temp_1 = r
                        + (c >> q_c as core::ffi::c_int - q_r as core::ffi::c_int);
                    q_temp1 = q_r;
                } else {
                    temp_1 = ixheaac_add32_sat(
                        r >> q_r as core::ffi::c_int - q_c as core::ffi::c_int,
                        c,
                    );
                    q_temp1 = q_c;
                }
                temp_0 = ixheaac_div32(temp_1, temp_0, &mut q_a);
                q_wl1 = (q_a as core::ffi::c_int + q_temp1 as core::ffi::c_int
                    - q_temp_0 as core::ffi::c_int) as WORD16;
                wl1 = ixheaacd_mps_sqrt(temp_0, &mut q_wl1, sqrt_tab_1);
                (*m_ttt
                    .offset(
                        0 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize] = ixheaacd_mps_convert_to_qn(
                    wl1,
                    q_wl1,
                    15 as WORD16,
                );
                temp_0 = ixheaac_div32(wl1, temp_1, &mut q_a);
                q_temp_0 = (q_a as core::ffi::c_int
                    + (q_wl1 as core::ffi::c_int - q_temp1 as core::ffi::c_int))
                    as WORD16;
                wl2 = ixheaacd_mps_mult32_shr_n(
                    c,
                    temp_0,
                    (q_c as core::ffi::c_int + q_temp_0 as core::ffi::c_int
                        - 15 as core::ffi::c_int) as WORD16,
                );
                (*m_ttt
                    .offset(
                        0 as core::ffi::c_int as isize,
                    ))[1 as core::ffi::c_int as usize] = ixheaac_negate32_sat(wl2);
                temp_0 = ixheaacd_mps_div_32(l, r, &mut q_temp_0);
                q_temp_0 = (q_temp_0 as core::ffi::c_int
                    + (q_l as core::ffi::c_int - q_r as core::ffi::c_int)) as WORD16;
                if q_temp_0 as core::ffi::c_int > 28 as core::ffi::c_int {
                    temp_0 = temp_0
                        >> q_temp_0 as core::ffi::c_int - 28 as core::ffi::c_int;
                    q_temp_0 = 28 as WORD16;
                }
                temp_0 = ixheaac_add32_sat(
                    ((1 as WORD32) << q_temp_0 as core::ffi::c_int) - 1 as WORD32,
                    temp_0,
                );
                temp_0 = ixheaac_add32_sat(
                    ixheaacd_mps_mult32_shr_n(
                        c,
                        temp_0,
                        (q_c as core::ffi::c_int + q_temp_0 as core::ffi::c_int
                            - q_l as core::ffi::c_int) as WORD16,
                    ),
                    l,
                );
                q_temp_0 = q_l;
                if q_c as core::ffi::c_int > q_l as core::ffi::c_int {
                    temp_1 = l
                        + (c >> q_c as core::ffi::c_int - q_l as core::ffi::c_int);
                    q_temp1 = q_l;
                } else {
                    temp_1 = ixheaac_add32_sat(
                        l >> q_l as core::ffi::c_int - q_c as core::ffi::c_int,
                        c,
                    );
                    q_temp1 = q_c;
                }
                temp_0 = ixheaac_div32(temp_1, temp_0, &mut q_a);
                q_wr1 = (q_a as core::ffi::c_int + q_temp1 as core::ffi::c_int
                    - q_temp_0 as core::ffi::c_int) as WORD16;
                wr1 = ixheaacd_mps_sqrt(temp_0, &mut q_wr1, sqrt_tab_1);
                (*m_ttt
                    .offset(
                        1 as core::ffi::c_int as isize,
                    ))[1 as core::ffi::c_int as usize] = ixheaacd_mps_convert_to_qn(
                    wr1,
                    q_wr1,
                    15 as WORD16,
                );
                temp_0 = ixheaac_div32(wr1, temp_1, &mut q_a);
                q_temp_0 = (q_a as core::ffi::c_int
                    + (q_wl1 as core::ffi::c_int - q_temp1 as core::ffi::c_int))
                    as WORD16;
                wr2 = ixheaacd_mps_mult32_shr_n(
                    c,
                    temp_0,
                    (q_c as core::ffi::c_int + q_temp_0 as core::ffi::c_int
                        - 15 as core::ffi::c_int) as WORD16,
                );
                (*m_ttt
                    .offset(
                        1 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize] = ixheaac_negate32_sat(wr2);
            }
        }
    }
    (*m_ttt.offset(0 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize] = ONE_BY_THREE_Q15
        as WORD32;
    (*m_ttt.offset(1 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize] = ONE_BY_THREE_Q15
        as WORD32;
    (*m_ttt.offset(2 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize] = MINUS_ONE_BY_THREE_Q15
        as WORD32;
    col = 0 as core::ffi::c_int as WORD32;
    while col < 3 as core::ffi::c_int {
        (*m_ttt.offset(2 as core::ffi::c_int as isize))[col as usize] = ixheaacd_mps_mult32_shr_15(
            (*m_ttt.offset(2 as core::ffi::c_int as isize))[col as usize],
            SQRT_TWO_Q15,
        );
        col += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_calculate_mtx_inv(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
    mut ps: WORD32,
    mut pb: WORD32,
    mut mode: WORD32,
    mut h_real: *mut [WORD32; 2],
    mut h_imag: *mut [WORD32; 2],
) -> VOID {
    let mut p_aux_struct: *mut ia_mps_dec_auxilary_struct = (*pstr_mps_state).aux_struct;
    let mut weight1: WORD32 = 0;
    let mut weight2: WORD32 = 0;
    let mut p_cur_bs: *mut ia_mps_dec_spatial_bs_frame_struct = (*pstr_mps_state)
        .bs_frame;
    if mode < 2 as core::ffi::c_int {
        ixheaacd_get_matrix_inversion_weights(
            (*p_cur_bs)
                .ott_cld_idx[1 as core::ffi::c_int as usize][ps as usize][pb as usize],
            (*p_cur_bs)
                .ott_cld_idx[2 as core::ffi::c_int as usize][ps as usize][pb as usize],
            1 as WORD32,
            (*p_aux_struct)
                .ttt_cpc_1[0 as core::ffi::c_int as usize][ps as usize][pb as usize],
            (*p_aux_struct)
                .ttt_cpc_2[0 as core::ffi::c_int as usize][ps as usize][pb as usize],
            &mut weight1,
            &mut weight2,
            &mut (*pstr_mps_state).ia_mps_dec_mps_table,
        );
    } else {
        ixheaacd_get_matrix_inversion_weights(
            (*p_cur_bs)
                .ott_cld_idx[1 as core::ffi::c_int as usize][ps as usize][pb as usize],
            (*p_cur_bs)
                .ott_cld_idx[2 as core::ffi::c_int as usize][ps as usize][pb as usize],
            0 as WORD32,
            (*p_cur_bs)
                .cmp_ttt_cld_1_idx[0 as core::ffi::c_int
                as usize][ps as usize][pb as usize],
            (*p_cur_bs)
                .cmp_ttt_cld_2_idx[0 as core::ffi::c_int
                as usize][ps as usize][pb as usize],
            &mut weight1,
            &mut weight2,
            &mut (*pstr_mps_state).ia_mps_dec_mps_table,
        );
    }
    ixheaacd_invert_matrix(
        weight1,
        weight2,
        h_real,
        h_imag,
        (*pstr_mps_state).ia_mps_dec_mps_table.common_table_ptr,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_calculate_arb_dmx_mtx(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
    mut ps: WORD32,
    mut pb: WORD32,
    mut g_real: *mut WORD32,
) -> VOID {
    let mut ch: WORD32 = 0;
    let mut gain: WORD32 = 0;
    let mut arbdmx_alpha_prev: *mut WORD32 = (*pstr_mps_state)
        .mps_persistent_mem
        .arbdmx_alpha_prev;
    let mut arbdmx_alpha_upd_set: *mut WORD32 = ((*(*pstr_mps_state).aux_struct)
        .arbdmx_alpha_upd_set)
        .as_mut_ptr();
    let mut arbdmx_alpha: *mut WORD32 = ((*(*pstr_mps_state).aux_struct).arbdmx_alpha)
        .as_mut_ptr();
    let mut n_ch_in: WORD32 = (*pstr_mps_state).num_input_channels;
    let mut temp_1: WORD32 = 0;
    ch = 0 as core::ffi::c_int as WORD32;
    while ch < n_ch_in {
        temp_1 = ((*(*pstr_mps_state).bs_frame)
            .arbdmx_gain_idx[ch as usize][ps as usize][pb as usize]
            + 15 as core::ffi::c_int) as WORD32;
        gain = (*(*pstr_mps_state).ia_mps_dec_mps_table.m1_m2_table_ptr)
            .dec_pow[temp_1 as usize];
        if pb < (*pstr_mps_state).arbdmx_residual_bands {
            if ps == 0 as core::ffi::c_int
                && *arbdmx_alpha_upd_set.offset(ch as isize) == 1 as core::ffi::c_int
            {
                *g_real.offset(ch as isize) = ixheaacd_mps_mult32_shr_15(
                    *arbdmx_alpha_prev,
                    gain,
                );
            } else {
                *g_real.offset(ch as isize) = ixheaacd_mps_mult32_shr_15(
                    *arbdmx_alpha.offset(ch as isize),
                    gain,
                );
            }
        } else {
            *g_real.offset(ch as isize) = gain;
        }
        arbdmx_alpha_prev = arbdmx_alpha_prev.offset(1);
        ch += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_quantize(mut cld: WORD32) -> WORD32 {
    match cld {
        -150 => return -(15 as WORD32),
        -45 => return -(14 as WORD32),
        -40 => return -(13 as WORD32),
        -35 => return -(12 as WORD32),
        -30 => return -(11 as WORD32),
        -25 => return -(10 as WORD32),
        -22 => return -(9 as WORD32),
        -19 => return -(8 as WORD32),
        -16 => return -(7 as WORD32),
        -13 => return -(6 as WORD32),
        -10 => return -(5 as WORD32),
        -8 => return -(4 as WORD32),
        -6 => return -(3 as WORD32),
        -4 => return -(2 as WORD32),
        -2 => return -(1 as WORD32),
        0 => return 0 as WORD32,
        2 => return 1 as WORD32,
        4 => return 2 as WORD32,
        6 => return 3 as WORD32,
        8 => return 4 as WORD32,
        10 => return 5 as WORD32,
        13 => return 6 as WORD32,
        16 => return 7 as WORD32,
        19 => return 8 as WORD32,
        22 => return 9 as WORD32,
        25 => return 10 as WORD32,
        30 => return 11 as WORD32,
        35 => return 12 as WORD32,
        40 => return 13 as WORD32,
        45 => return 14 as WORD32,
        150 => return 15 as WORD32,
        _ => return 0 as WORD32,
    };
}
