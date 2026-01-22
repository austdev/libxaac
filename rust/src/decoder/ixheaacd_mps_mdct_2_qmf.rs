extern "C" {
    pub type ia_mps_dec_ducker_interface;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
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
pub const IA_XHEAAC_MPS_DEC_EXE_FATAL_INVALID_TIMESLOTS: core::ffi::c_uint = 0xffff9806
    as core::ffi::c_uint;
pub const MAX_32: WORD32 = 0x7fffffff as core::ffi::c_long as WORD32;
pub const MIN_32: WORD32 = 0x80000000 as core::ffi::c_long as WORD32;
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
unsafe extern "C" fn ixheaac_mac32x32in64(
    mut sum: WORD64,
    mut a: WORD32,
    mut b: WORD32,
) -> WORD64 {
    sum += a as WORD64 * b as WORD64;
    return sum;
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
pub const ONLY_LONG_SEQUENCE: core::ffi::c_int = 0;
pub const LONG_START_SEQUENCE: core::ffi::c_int = 1;
pub const EIGHT_SHORT_SEQUENCE: core::ffi::c_int = 2;
pub const LONG_STOP_SEQUENCE: core::ffi::c_int = 3;
pub const IA_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const MAX_TIME_SLOTS: core::ffi::c_int = 72 as core::ffi::c_int;
pub const AAC_FRAME_LENGTH: core::ffi::c_int = 1024 as core::ffi::c_int;
pub const AAC_SHORT_FRAME_LENGTH: core::ffi::c_int = 128 as core::ffi::c_int;
pub const MDCT_LENGTH_LO: core::ffi::c_int = 1024 as core::ffi::c_int;
pub const UPD_QMF_15: core::ffi::c_int = 15;
pub const UPD_QMF_16: core::ffi::c_int = 16;
pub const UPD_QMF_18: core::ffi::c_int = 18;
pub const UPD_QMF_24: core::ffi::c_int = 24;
pub const UPD_QMF_30: core::ffi::c_int = 30;
pub const UPD_QMF_32: core::ffi::c_int = 32;
pub const TSX2_4: core::ffi::c_int = 4;
pub const TSX2_6: core::ffi::c_int = 6;
pub const TSX2_8: core::ffi::c_int = 8;
pub const TSX2_30: core::ffi::c_int = 30;
pub const TSX2_32: core::ffi::c_int = 32;
pub const TSX2_36: core::ffi::c_int = 36;
pub const TSX2_48: core::ffi::c_int = 48;
pub const TSX2_60: core::ffi::c_int = 60;
pub const TSX2_64: core::ffi::c_int = 64;
pub const TS_2: core::ffi::c_int = 2;
pub const TS_4: core::ffi::c_int = 4;
pub const TS_MINUS_ONE_4: core::ffi::c_int = 3 as core::ffi::c_int;
pub const TS_MINUS_ONE_15: core::ffi::c_int = 14 as core::ffi::c_int;
pub const TS_MINUS_ONE_16: core::ffi::c_int = 15 as core::ffi::c_int;
pub const TS_MINUS_ONE_18: core::ffi::c_int = 17 as core::ffi::c_int;
pub const TS_MINUS_ONE_24: core::ffi::c_int = 23 as core::ffi::c_int;
pub const TS_MINUS_ONE_30: core::ffi::c_int = 29 as core::ffi::c_int;
pub const TS_MINUS_ONE_32: core::ffi::c_int = 31 as core::ffi::c_int;
pub const ZERO: core::ffi::c_int = 0;
pub const ONE_IN_Q15: core::ffi::c_int = 32768 as core::ffi::c_int;
pub const RES_CHXQMFXTSX4: core::ffi::c_int = 184320 as core::ffi::c_int;
pub const LOOP_COUNTER: core::ffi::c_int = 32 as core::ffi::c_int;
pub const ONE_BIT_MASK: core::ffi::c_int = 0x1 as core::ffi::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mdct2qmf_tables_init(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
) -> IA_ERRORCODE {
    let mut time_slots: WORD32 = 0;
    time_slots = (*pstr_mps_state).upd_qmf;
    if 32 as core::ffi::c_int == time_slots {
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[0 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_00)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[1 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_01)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[2 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_02)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[3 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_03)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[4 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_04)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[5 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_05)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[6 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_06)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[7 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_07)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[8 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_08)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[9 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_09)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[10 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_10)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[11 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_11)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[12 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_12)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[13 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_13)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[14 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_14)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[15 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_15)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[16 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_15)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[17 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_14)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[18 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_13)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[19 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_12)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[20 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_11)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[21 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_10)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[22 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_09)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[23 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_08)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[24 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_07)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[25 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_06)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[26 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_05)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[27 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_04)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[28 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_03)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[29 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_02)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[30 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_01)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[31 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_00)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[32 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_16)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[33 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_17)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[34 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_18)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[35 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_19)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[36 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_20)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[37 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_21)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[38 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_22)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[39 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_23)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[40 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_24)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[41 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_25)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[42 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_26)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[43 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_27)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[44 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_28)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[45 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_29)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[46 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_30)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[47 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_31)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[48 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_31)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[49 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_30)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[50 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_29)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[51 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_28)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[52 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_27)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[53 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_26)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[54 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_25)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[55 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_24)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[56 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_23)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[57 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_22)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[58 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_21)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[59 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_20)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[60 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_19)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[61 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_18)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[62 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_17)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_long[63 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_long_32_16)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_short[0 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_short_4_00)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_short[1 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_short_4_01)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_short[2 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_short_4_01)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_short[3 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_short_4_00)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_short[4 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_short_4_02)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_short[5 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_short_4_03)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_short[6 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_short_4_03)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_short[7 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_short_4_02)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_short[8 as core::ffi::c_int as usize] = 0 as *mut WORD16;
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_short[9 as core::ffi::c_int as usize] = 0 as *mut WORD16;
    } else {
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_short[0 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_short_2_00)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_short[1 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_short_2_00)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_short[2 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_short_2_01)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_short[3 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .mdct2qmfcos_table_ptr)
            .cos_table_short_2_01)
            .as_mut_ptr();
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_short[4 as core::ffi::c_int as usize] = 0 as *mut WORD16;
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_short[5 as core::ffi::c_int as usize] = 0 as *mut WORD16;
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_short[6 as core::ffi::c_int as usize] = 0 as *mut WORD16;
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_short[7 as core::ffi::c_int as usize] = 0 as *mut WORD16;
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_short[8 as core::ffi::c_int as usize] = 0 as *mut WORD16;
        (*pstr_mps_state)
            .ia_mps_dec_mdct2qmfcos_table
            .cos_table_short[9 as core::ffi::c_int as usize] = 0 as *mut WORD16;
        if 30 as core::ffi::c_int == time_slots {
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[0 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_00)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[1 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_01)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[2 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_02)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[3 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_03)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[4 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_04)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[5 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_05)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[6 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_06)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[7 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_07)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[8 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_08)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[9 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_09)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[10 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_10)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[11 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_11)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[12 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_12)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[13 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_13)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[14 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_14)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[15 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_14)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[16 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_13)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[17 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_12)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[18 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_11)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[19 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_10)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[20 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_09)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[21 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_08)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[22 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_07)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[23 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_06)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[24 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_05)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[25 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_04)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[26 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_03)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[27 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_02)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[28 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_01)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[29 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_00)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[30 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_15)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[31 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_16)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[32 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_17)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[33 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_18)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[34 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_19)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[35 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_20)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[36 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_21)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[37 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_22)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[38 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_23)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[39 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_24)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[40 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_25)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[41 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_26)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[42 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_27)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[43 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_28)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[44 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_29)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[45 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_29)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[46 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_28)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[47 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_27)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[48 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_26)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[49 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_25)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[50 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_24)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[51 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_23)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[52 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_22)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[53 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_21)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[54 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_20)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[55 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_19)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[56 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_18)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[57 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_17)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[58 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_16)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[59 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_30_15)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[60 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[61 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[62 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[63 as core::ffi::c_int as usize] = 0 as *mut WORD16;
        } else if 24 as core::ffi::c_int == time_slots {
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[0 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_00)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[1 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_01)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[2 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_02)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[3 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_03)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[4 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_04)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[5 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_05)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[6 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_06)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[7 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_07)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[8 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_08)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[9 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_09)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[10 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_10)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[11 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_11)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[12 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_11)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[13 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_10)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[14 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_09)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[15 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_08)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[16 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_07)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[17 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_06)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[18 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_05)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[19 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_04)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[20 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_03)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[21 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_02)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[22 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_01)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[23 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_00)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[24 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_12)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[25 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_13)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[26 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_14)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[27 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_15)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[28 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_16)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[29 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_17)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[30 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_18)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[31 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_19)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[32 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_20)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[33 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_21)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[34 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_22)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[35 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_23)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[36 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_23)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[37 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_22)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[38 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_21)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[39 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_20)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[40 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_19)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[41 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_18)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[42 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_17)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[43 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_16)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[44 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_15)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[45 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_14)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[46 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_13)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[47 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_24_12)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[48 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[49 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[50 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[51 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[52 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[53 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[54 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[55 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[56 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[57 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[58 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[59 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[60 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[61 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[62 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[63 as core::ffi::c_int as usize] = 0 as *mut WORD16;
        } else if 18 as core::ffi::c_int == time_slots {
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[0 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_00)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[1 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_01)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[2 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_02)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[3 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_03)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[4 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_04)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[5 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_05)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[6 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_06)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[7 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_07)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[8 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_08)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[9 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_08)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[10 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_07)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[11 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_06)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[12 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_05)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[13 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_04)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[14 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_03)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[15 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_02)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[16 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_01)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[17 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_00)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[18 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_09)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[19 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_10)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[20 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_11)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[21 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_12)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[22 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_13)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[23 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_14)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[24 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_15)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[25 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_16)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[26 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_17)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[27 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_17)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[28 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_16)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[29 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_15)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[30 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_14)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[31 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_13)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[32 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_12)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[33 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_11)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[34 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_10)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[35 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_18_09)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[36 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[37 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[38 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[39 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[40 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[41 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[42 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[43 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[44 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[45 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[46 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[47 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[48 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[49 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[50 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[51 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[52 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[53 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[54 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[55 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[56 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[57 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[58 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[59 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[60 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[61 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[62 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[63 as core::ffi::c_int as usize] = 0 as *mut WORD16;
        } else if 16 as core::ffi::c_int == time_slots {
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[0 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_00)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[1 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_01)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[2 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_02)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[3 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_03)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[4 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_04)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[5 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_05)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[6 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_06)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[7 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_07)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[8 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_07)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[9 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_06)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[10 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_05)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[11 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_04)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[12 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_03)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[13 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_02)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[14 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_01)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[15 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_00)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[16 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_08)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[17 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_09)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[18 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_10)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[19 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_11)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[20 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_12)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[21 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_13)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[22 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_14)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[23 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_15)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[24 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_15)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[25 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_14)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[26 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_13)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[27 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_12)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[28 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_11)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[29 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_10)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[30 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_09)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[31 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_16_08)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[32 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[33 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[34 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[35 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[36 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[37 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[38 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[39 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[40 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[41 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[42 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[43 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[44 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[45 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[46 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[47 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[48 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[49 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[50 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[51 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[52 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[53 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[54 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[55 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[56 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[57 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[58 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[59 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[60 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[61 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[62 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[63 as core::ffi::c_int as usize] = 0 as *mut WORD16;
        } else if 15 as core::ffi::c_int == time_slots {
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[0 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_00)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[1 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_01)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[2 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_02)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[3 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_03)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[4 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_04)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[5 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_05)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[6 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_06)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[7 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_06)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[8 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_05)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[9 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_04)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[10 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_03)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[11 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_02)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[12 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_01)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[13 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_00)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[14 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_07)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[15 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_08)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[16 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_09)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[17 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_10)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[18 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_11)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[19 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_12)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[20 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_13)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[21 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_14)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[22 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_14)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[23 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_13)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[24 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_12)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[25 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_11)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[26 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_10)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[27 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_09)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[28 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_08)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[29 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_long_15_07)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[30 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[31 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[32 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[33 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[34 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[35 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[36 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[37 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[38 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[39 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[40 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[41 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[42 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[43 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[44 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[45 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[46 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[47 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[48 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[49 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[50 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[51 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[52 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[53 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[54 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[55 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[56 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[57 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[58 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[59 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[60 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[61 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[62 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_long[63 as core::ffi::c_int as usize] = 0 as *mut WORD16;
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_short[4 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_short_3_00)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_short[5 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_short_3_00)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_short[6 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_short_3_01)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_short[7 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_short_3_02)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_short[8 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_short_3_02)
                .as_mut_ptr();
            (*pstr_mps_state)
                .ia_mps_dec_mdct2qmfcos_table
                .cos_table_short[9 as core::ffi::c_int as usize] = ((*(*pstr_mps_state)
                .ia_mps_dec_mps_table
                .mdct2qmfcos_table_ptr)
                .cos_table_short_3_01)
                .as_mut_ptr();
        } else if (*pstr_mps_state).residual_coding != 0 {
            return IA_XHEAAC_MPS_DEC_EXE_FATAL_INVALID_TIMESLOTS as IA_ERRORCODE
        }
    }
    return IA_NO_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mdct2qmf_create(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
) -> IA_ERRORCODE {
    let mut error_code: IA_ERRORCODE = IA_NO_ERROR;
    let mut qmf_residual_real: *mut WORD32 = (*(*pstr_mps_state).array_struct)
        .qmf_residual_real_pre;
    let mut qmf_residual_imag: *mut WORD32 = (*(*pstr_mps_state).array_struct)
        .qmf_residual_imag_pre;
    memset(
        qmf_residual_real as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        RES_CHXQMFXTSX4 as size_t,
    );
    memset(
        qmf_residual_imag as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        RES_CHXQMFXTSX4 as size_t,
    );
    error_code = ixheaacd_mdct2qmf_tables_init(pstr_mps_state);
    return error_code;
}
unsafe extern "C" fn ixheaacd_local_zero(l: WORD32, b: *mut WORD32) -> VOID {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < l {
        *b.offset(i as isize) = 0 as core::ffi::c_int as WORD32;
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_local_fold_out(
    s: *mut WORD32,
    lv: WORD32,
    w: *mut WORD32,
    l_w: WORD32,
    v_main: *mut WORD32,
    v_slave: *mut WORD32,
) -> VOID {
    let mut n: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut temp_1: WORD32 = 0;
    let mut w1: *mut WORD32 = 0 as *mut WORD32;
    let mut w2: *mut WORD32 = 0 as *mut WORD32;
    let mut w3: *mut WORD32 = 0 as *mut WORD32;
    let mut w4: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr1: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr2: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr3: *mut WORD32 = 0 as *mut WORD32;
    let mut m: WORD32 = l_w >> 1 as core::ffi::c_int;
    let mut l: WORD32 = lv / m;
    let mut m2w: WORD32 = m >> 1 as core::ffi::c_int;
    let mut m2a: WORD32 = m - m2w;
    i = m;
    while i < lv {
        n = i + l_w;
        while i < n {
            *s.offset(i as isize) = -*s.offset(i as isize);
            i += 1;
        }
        i += l_w;
    }
    w1 = &mut *w.offset(-m2a as isize) as *mut WORD32;
    w2 = &mut *w.offset(m2w as isize) as *mut WORD32;
    w3 = &mut *w.offset(m2w as isize) as *mut WORD32;
    w4 = &mut *w.offset((m + m2w) as isize) as *mut WORD32;
    n = 0 as core::ffi::c_int as WORD32;
    j = 0 as core::ffi::c_int as WORD32;
    k = m;
    while n < l as core::ffi::c_int - 1 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < m2a {
            *v_main.offset(k as isize) = ixheaacd_mps_mult32_shr_30(
                *w2.offset(i as isize),
                *s.offset(k as isize),
            );
            *v_slave.offset(j as isize) = ixheaacd_mps_mult32_shr_30(
                *w4.offset(i as isize),
                *s.offset(k as isize),
            );
            i += 1;
            j += 1;
            k += 1;
        }
        while i < m {
            *v_main.offset(j as isize) = ixheaacd_mps_mult32_shr_30(
                *w3.offset(i as isize),
                *s.offset(j as isize),
            );
            *v_slave.offset(k as isize) = ixheaacd_mps_mult32_shr_30(
                *w1.offset(i as isize),
                *s.offset(j as isize),
            );
            i += 1;
            j += 1;
            k += 1;
        }
        n += 1;
    }
    ptr1 = v_main;
    ptr2 = v_slave.offset(m as isize).offset(-(1 as core::ffi::c_int as isize));
    ptr3 = s;
    i = 0 as core::ffi::c_int as WORD32;
    while i < m2a {
        let fresh50 = ptr3;
        ptr3 = ptr3.offset(1);
        temp_1 = *fresh50;
        let fresh51 = w2;
        w2 = w2.offset(1);
        let fresh52 = ptr1;
        ptr1 = ptr1.offset(1);
        *fresh52 = ixheaacd_mps_mult32_shr_30(*fresh51, temp_1);
        let fresh53 = w4;
        w4 = w4.offset(1);
        let fresh54 = ptr2;
        ptr2 = ptr2.offset(-1);
        *fresh54 = ixheaacd_mps_mult32_shr_30(*fresh53, temp_1);
        i += 1;
    }
    j = l * m - m2w;
    k = (l as core::ffi::c_int * m as core::ffi::c_int - m2a as core::ffi::c_int
        - 1 as core::ffi::c_int) as WORD32;
    ptr3 = s.offset((l * m) as isize).offset(-(m as isize));
    ptr1 = v_main.offset(j as isize);
    ptr2 = v_slave.offset(k as isize);
    w1 = w1.offset(m2a as isize);
    while i < m {
        let fresh55 = ptr3;
        ptr3 = ptr3.offset(1);
        temp_1 = *fresh55;
        let fresh56 = w2;
        w2 = w2.offset(1);
        let fresh57 = ptr1;
        ptr1 = ptr1.offset(1);
        *fresh57 = ixheaacd_mps_mult32_shr_30(*fresh56, temp_1);
        let fresh58 = w1;
        w1 = w1.offset(1);
        let fresh59 = ptr2;
        ptr2 = ptr2.offset(-1);
        *fresh59 = ixheaacd_mps_mult32_shr_30(*fresh58, temp_1);
        i += 1;
        j += 1;
    }
}
unsafe extern "C" fn ixheaacd_local_imdet(
    mut x1: *mut WORD32,
    mut x2: *mut WORD32,
    scale1: *mut WORD32,
    val: WORD32,
    mut z_real: *mut WORD32,
    mut z_imag: *mut WORD32,
    mut ia_mps_dec_mdct2qmfcos_tab: *const ia_mps_dec_mdct2qmf_cos_table_struct,
    mut is_long: WORD32,
    mut scratch: *mut core::ffi::c_void,
) -> VOID {
    let mut lw: WORD32 = val << 1 as core::ffi::c_int;
    let mut offset: WORD32 = val - (val >> 1 as core::ffi::c_int);
    let mut temp_1: WORD32 = 0;
    let mut temp3: WORD32 = 0;
    let mut temp_2: WORD32 = 0;
    let mut z_real_2: *mut WORD32 = 0 as *mut WORD32;
    let mut z_imag_2: *mut WORD32 = 0 as *mut WORD32;
    let mut px1: *mut WORD32 = 0 as *mut WORD32;
    let mut px2: *mut WORD32 = 0 as *mut WORD32;
    let mut px3: *mut WORD32 = 0 as *mut WORD32;
    let mut px4: *mut WORD32 = 0 as *mut WORD32;
    let mut cp: *const WORD16 = 0 as *const WORD16;
    let mut sp: *const WORD16 = 0 as *const WORD16;
    let mut scale: *mut WORD32 = 0 as *mut WORD32;
    let mut cnt: WORD32 = val + (val >> 1 as core::ffi::c_int);
    let mut k: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut l: WORD32 = 0;
    let mut p_sum: *mut WORD32 = scratch as *mut WORD32;
    let mut p_diff: *mut WORD32 = (scratch as *mut WORD32)
        .offset(
            ((2048 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    z_real_2 = z_real.offset(lw as isize);
    z_imag_2 = z_imag.offset(lw as isize);
    l = 0 as core::ffi::c_int as WORD32;
    while l < LOOP_COUNTER {
        let mut sum: *mut WORD32 = p_sum;
        let mut diff: *mut WORD32 = p_diff;
        px1 = x1;
        px2 = x2.offset(val as isize).offset(-(1 as core::ffi::c_int as isize));
        px3 = x2.offset(val as isize);
        px4 = x1
            .offset((2 as WORD32 * val) as isize)
            .offset(-(1 as core::ffi::c_int as isize));
        n = 0 as core::ffi::c_int as WORD32;
        while n < val {
            let fresh37 = sum;
            sum = sum.offset(1);
            *fresh37 = ixheaac_add32_sat(*px1, *px2);
            let fresh38 = sum;
            sum = sum.offset(1);
            *fresh38 = ixheaac_add32_sat(*px3, *px4);
            let fresh39 = px1;
            px1 = px1.offset(1);
            let fresh40 = px2;
            px2 = px2.offset(-1);
            let fresh41 = diff;
            diff = diff.offset(1);
            *fresh41 = ixheaac_sub32_sat(*fresh39, *fresh40);
            let fresh42 = px3;
            px3 = px3.offset(1);
            let fresh43 = px4;
            px4 = px4.offset(-1);
            let fresh44 = diff;
            diff = diff.offset(1);
            *fresh44 = ixheaac_sub32_sat(*fresh42, *fresh43);
            n += 1;
        }
        scale = scale1;
        k = 0 as core::ffi::c_int as WORD32;
        while k < cnt {
            if 1 as core::ffi::c_int == is_long {
                cp = (*ia_mps_dec_mdct2qmfcos_tab).cos_table_long[k as usize];
                sp = cp.offset(val as isize);
            } else {
                cp = (*ia_mps_dec_mdct2qmfcos_tab)
                    .cos_table_short[(k + is_long) as usize];
                sp = cp.offset(val as isize);
            }
            sum = p_sum;
            diff = p_diff;
            let fresh45 = cp;
            cp = cp.offset(1);
            temp_1 = *fresh45 as WORD32;
            temp_2 = ixheaacd_mps_mult32_shr_15(temp_1, *sum);
            sum = sum.offset(1);
            *z_real = temp_2;
            temp_2 = ixheaacd_mps_mult32_shr_15(temp_1, *sum);
            sum = sum.offset(1);
            *z_real_2 = temp_2;
            sp = sp.offset(-1);
            temp_1 = *sp as WORD32;
            temp_2 = ixheaacd_mps_mult32_shr_15(temp_1, *diff);
            diff = diff.offset(1);
            *z_imag = temp_2;
            temp_2 = ixheaacd_mps_mult32_shr_15(temp_1, *diff);
            diff = diff.offset(1);
            *z_imag_2 = temp_2;
            n = 1 as core::ffi::c_int as WORD32;
            while n < val {
                let fresh46 = cp;
                cp = cp.offset(1);
                temp_1 = *fresh46 as WORD32;
                temp_2 = ixheaacd_mps_mult32_shr_15(temp_1, *sum);
                sum = sum.offset(1);
                *z_real = ixheaac_add32_sat(*z_real, temp_2);
                temp_2 = ixheaacd_mps_mult32_shr_15(temp_1, *sum);
                sum = sum.offset(1);
                *z_real_2 = ixheaac_add32_sat(*z_real_2, temp_2);
                sp = sp.offset(-1);
                temp_1 = *sp as WORD32;
                temp_2 = ixheaacd_mps_mult32_shr_15(temp_1, *diff);
                diff = diff.offset(1);
                *z_imag = ixheaac_add32_sat(*z_imag, temp_2);
                temp_2 = ixheaacd_mps_mult32_shr_15(temp_1, *diff);
                diff = diff.offset(1);
                *z_imag_2 = ixheaac_add32_sat(*z_imag_2, temp_2);
                n += 1;
            }
            z_real = z_real.offset(1);
            z_imag = z_imag.offset(1);
            z_real_2 = z_real_2.offset(1);
            z_imag_2 = z_imag_2.offset(1);
            k += 1;
        }
        z_real = z_real.offset(-(cnt as isize));
        z_real_2 = z_real_2.offset(-(cnt as isize));
        z_imag = z_imag.offset(-(cnt as isize));
        z_imag_2 = z_imag_2.offset(-(cnt as isize));
        j = 0 as core::ffi::c_int as WORD32;
        while j < cnt {
            *z_real = ixheaacd_mps_mult32_shr_15(*z_real, *scale);
            z_real = z_real.offset(1);
            *z_imag = ixheaacd_mps_mult32_shr_15(*z_imag, *scale);
            z_imag = z_imag.offset(1);
            *z_real_2 = ixheaacd_mps_mult32_shr_15(*z_real_2, *scale);
            z_real_2 = z_real_2.offset(1);
            *z_imag_2 = ixheaacd_mps_mult32_shr_15(*z_imag_2, *scale);
            scale = scale.offset(1);
            z_imag_2 = z_imag_2.offset(1);
            j += 1;
        }
        while k < lw {
            if 1 as core::ffi::c_int == is_long {
                cp = (*ia_mps_dec_mdct2qmfcos_tab).cos_table_long[k as usize];
                sp = cp.offset(val as isize);
            } else {
                cp = (*ia_mps_dec_mdct2qmfcos_tab)
                    .cos_table_short[(k + is_long) as usize];
                sp = cp.offset(val as isize);
            }
            sum = p_sum;
            diff = p_diff;
            let fresh47 = cp;
            cp = cp.offset(1);
            temp_1 = *fresh47 as WORD32;
            temp3 = ixheaacd_mps_mult32_shr_15(temp_1, *sum);
            sum = sum.offset(1);
            *z_real = temp3;
            temp3 = ixheaacd_mps_mult32_shr_15(temp_1, *sum);
            sum = sum.offset(1);
            *z_real_2 = temp3;
            sp = sp.offset(-1);
            temp_1 = *sp as WORD32;
            temp3 = ixheaacd_mps_mult32_shr_15(temp_1, *diff);
            diff = diff.offset(1);
            *z_imag = temp3;
            temp3 = ixheaacd_mps_mult32_shr_15(temp_1, *diff);
            diff = diff.offset(1);
            *z_imag_2 = temp3;
            n = 1 as core::ffi::c_int as WORD32;
            while n < val {
                let fresh48 = cp;
                cp = cp.offset(1);
                temp_1 = *fresh48 as WORD32;
                temp3 = ixheaacd_mps_mult32_shr_15(temp_1, *sum);
                sum = sum.offset(1);
                *z_real = ixheaac_add32_sat(*z_real, temp3);
                temp3 = ixheaacd_mps_mult32_shr_15(temp_1, *sum);
                sum = sum.offset(1);
                *z_real_2 = ixheaac_add32_sat(*z_real_2, temp3);
                sp = sp.offset(-1);
                temp_1 = *sp as WORD32;
                temp3 = ixheaacd_mps_mult32_shr_15(temp_1, *diff);
                diff = diff.offset(1);
                *z_imag = ixheaac_add32_sat(*z_imag, temp3);
                temp3 = ixheaacd_mps_mult32_shr_15(temp_1, *diff);
                diff = diff.offset(1);
                *z_imag_2 = ixheaac_add32_sat(*z_imag_2, temp3);
                n += 1;
            }
            z_real = z_real.offset(1);
            z_imag = z_imag.offset(1);
            z_real_2 = z_real_2.offset(1);
            z_imag_2 = z_imag_2.offset(1);
            k += 1;
        }
        z_real = z_real.offset(-(offset as isize));
        z_real_2 = z_real_2.offset(-(offset as isize));
        z_imag = z_imag.offset(-(offset as isize));
        z_imag_2 = z_imag_2.offset(-(offset as isize));
        j = 0 as core::ffi::c_int as WORD32;
        while j < offset {
            let fresh49 = scale;
            scale = scale.offset(1);
            temp_1 = (*fresh49 * -(1 as core::ffi::c_int)) as WORD32;
            *z_real = ixheaacd_mps_mult32_shr_15(*z_real, temp_1);
            z_real = z_real.offset(1);
            *z_imag = ixheaacd_mps_mult32_shr_15(*z_imag, temp_1);
            z_imag = z_imag.offset(1);
            *z_real_2 = ixheaacd_mps_mult32_shr_15(*z_real_2, temp_1);
            z_real_2 = z_real_2.offset(1);
            *z_imag_2 = ixheaacd_mps_mult32_shr_15(*z_imag_2, temp_1);
            z_imag_2 = z_imag_2.offset(1);
            j += 1;
        }
        x1 = x1.offset(lw as isize);
        x2 = x2.offset(lw as isize);
        z_real = z_real.offset(lw as isize);
        z_imag = z_imag.offset(lw as isize);
        z_imag_2 = z_imag_2.offset(lw as isize);
        z_real_2 = z_real_2.offset(lw as isize);
        l += 1;
    }
}
unsafe extern "C" fn ixheaacd_local_hybcmdct2qmf(
    v_main: *mut WORD32,
    v_slave: *mut WORD32,
    w: *mut WORD32,
    lw: WORD32,
    mut z_real: *mut WORD32,
    mut z_imag: *mut WORD32,
    mut ia_mps_dec_mdct2qmfcos_tab: *const ia_mps_dec_mdct2qmf_cos_table_struct,
    mut scratch: *mut core::ffi::c_void,
    mut is_long: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut start: WORD32 = 0 as WORD32;
    let mut m: WORD32 = lw >> 1 as core::ffi::c_int;
    match lw {
        TSX2_4 | TSX2_6 | TSX2_30 | TSX2_36 | TSX2_60 => {
            start = 1 as core::ffi::c_int as WORD32;
        }
        TSX2_8 | TSX2_32 | TSX2_48 | TSX2_64 => {
            start = 0 as core::ffi::c_int as WORD32;
        }
        _ => {}
    }
    ixheaacd_local_imdet(
        v_slave,
        v_main,
        w,
        m,
        z_real,
        z_imag,
        ia_mps_dec_mdct2qmfcos_tab,
        is_long,
        scratch,
    );
    i = start;
    while i < m << 7 as core::ffi::c_int {
        *z_imag.offset(i as isize) = -*z_imag.offset(i as isize);
        i += 2 as core::ffi::c_int;
    }
}
unsafe extern "C" fn ixheaacd_local_p_zero_ts15(
    b: *mut WORD32,
    mut src: *mut WORD32,
    mut l: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < 15 as core::ffi::c_int {
        let fresh73 = src;
        src = src.offset(1);
        *b.offset(i as isize) = *fresh73;
        i += 1;
    }
    if l != 15 as core::ffi::c_int {
        src = src.offset(-1);
        while i < l {
            src = src.offset(-1);
            *b.offset(i as isize) = *src;
            i += 1;
        }
    }
}
unsafe extern "C" fn ixheaacd_local_p_nonzero(
    b: *mut WORD32,
    mut src: *mut WORD32,
    mut l: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < l {
        let fresh71 = src;
        src = src.offset(-1);
        *b.offset(i as isize) = *fresh71;
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_local_p_zero(
    b: *mut WORD32,
    mut src: *mut WORD32,
    mut l: WORD32,
    mut upd_qmf: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < upd_qmf {
        let fresh72 = src;
        src = src.offset(1);
        *b.offset(i as isize) = *fresh72;
        i += 1;
    }
    if l != upd_qmf {
        while i < l {
            src = src.offset(-1);
            *b.offset(i as isize) = *src;
            i += 1;
        }
    }
}
unsafe extern "C" fn ixheaacd_local_sin(
    t: WORD32,
    p: WORD32,
    l: WORD32,
    b: *mut WORD32,
    mut mdct2qmf_tab: *mut ia_mps_dec_mdct2qmf_table_struct,
) -> VOID {
    let mut sin_ptr: *mut WORD32 = 0 as *mut WORD32;
    match t {
        TS_2 => {
            match p {
                ZERO => {
                    *b.offset(0 as core::ffi::c_int as isize) = 12540 as core::ffi::c_int
                        as WORD32;
                    *b.offset(1 as core::ffi::c_int as isize) = 30274 as core::ffi::c_int
                        as WORD32;
                    if l == TSX2_4 {
                        *b.offset(2 as core::ffi::c_int as isize) = *b
                            .offset(1 as core::ffi::c_int as isize);
                        *b.offset(3 as core::ffi::c_int as isize) = *b
                            .offset(0 as core::ffi::c_int as isize);
                    }
                }
                TS_2 => {
                    *b.offset(0 as core::ffi::c_int as isize) = 30274 as core::ffi::c_int
                        as WORD32;
                    *b.offset(1 as core::ffi::c_int as isize) = 12540 as core::ffi::c_int
                        as WORD32;
                }
                _ => {}
            }
        }
        TS_4 => {
            match p {
                ZERO => {
                    sin_ptr = ((*mdct2qmf_tab).local_sin_4).as_mut_ptr();
                    let fresh62 = sin_ptr;
                    sin_ptr = sin_ptr.offset(1);
                    *b.offset(0 as core::ffi::c_int as isize) = *fresh62;
                    let fresh63 = sin_ptr;
                    sin_ptr = sin_ptr.offset(1);
                    *b.offset(1 as core::ffi::c_int as isize) = *fresh63;
                    let fresh64 = sin_ptr;
                    sin_ptr = sin_ptr.offset(1);
                    *b.offset(2 as core::ffi::c_int as isize) = *fresh64;
                    *b.offset(3 as core::ffi::c_int as isize) = *sin_ptr;
                    if l == TSX2_8 {
                        let fresh65 = sin_ptr;
                        sin_ptr = sin_ptr.offset(-1);
                        *b.offset(4 as core::ffi::c_int as isize) = *fresh65;
                        let fresh66 = sin_ptr;
                        sin_ptr = sin_ptr.offset(-1);
                        *b.offset(5 as core::ffi::c_int as isize) = *fresh66;
                        let fresh67 = sin_ptr;
                        sin_ptr = sin_ptr.offset(-1);
                        *b.offset(6 as core::ffi::c_int as isize) = *fresh67;
                        *b.offset(7 as core::ffi::c_int as isize) = *sin_ptr;
                    }
                }
                TS_4 => {
                    sin_ptr = &mut *((*mdct2qmf_tab).local_sin_4)
                        .as_mut_ptr()
                        .offset(TS_MINUS_ONE_4 as isize) as *mut WORD32;
                    let fresh68 = sin_ptr;
                    sin_ptr = sin_ptr.offset(-1);
                    *b.offset(0 as core::ffi::c_int as isize) = *fresh68;
                    let fresh69 = sin_ptr;
                    sin_ptr = sin_ptr.offset(-1);
                    *b.offset(1 as core::ffi::c_int as isize) = *fresh69;
                    let fresh70 = sin_ptr;
                    sin_ptr = sin_ptr.offset(-1);
                    *b.offset(2 as core::ffi::c_int as isize) = *fresh70;
                    *b.offset(3 as core::ffi::c_int as isize) = *sin_ptr;
                }
                _ => {}
            }
        }
        UPD_QMF_15 => {
            match p {
                ZERO => {
                    sin_ptr = &mut *((*mdct2qmf_tab).local_sin_15)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as isize) as *mut WORD32;
                    ixheaacd_local_p_zero_ts15(b, sin_ptr, l);
                }
                UPD_QMF_15 => {
                    sin_ptr = &mut *((*mdct2qmf_tab).local_sin_15)
                        .as_mut_ptr()
                        .offset(TS_MINUS_ONE_15 as isize) as *mut WORD32;
                    ixheaacd_local_p_nonzero(b, sin_ptr, l);
                }
                _ => {}
            }
        }
        UPD_QMF_16 => {
            match p {
                ZERO => {
                    sin_ptr = ((*mdct2qmf_tab).local_sin_16).as_mut_ptr();
                    ixheaacd_local_p_zero(b, sin_ptr, l, UPD_QMF_16);
                }
                UPD_QMF_16 => {
                    sin_ptr = &mut *((*mdct2qmf_tab).local_sin_16)
                        .as_mut_ptr()
                        .offset(TS_MINUS_ONE_16 as isize) as *mut WORD32;
                    ixheaacd_local_p_nonzero(b, sin_ptr, l);
                }
                _ => {}
            }
        }
        UPD_QMF_18 => {
            match p {
                ZERO => {
                    sin_ptr = ((*mdct2qmf_tab).local_sin_18).as_mut_ptr();
                    ixheaacd_local_p_zero(b, sin_ptr, l, UPD_QMF_18);
                }
                UPD_QMF_18 => {
                    sin_ptr = &mut *((*mdct2qmf_tab).local_sin_18)
                        .as_mut_ptr()
                        .offset(TS_MINUS_ONE_18 as isize) as *mut WORD32;
                    ixheaacd_local_p_nonzero(b, sin_ptr, l);
                }
                _ => {}
            }
        }
        UPD_QMF_24 => {
            match p {
                ZERO => {
                    sin_ptr = ((*mdct2qmf_tab).local_sin_24).as_mut_ptr();
                    ixheaacd_local_p_zero(b, sin_ptr, l, UPD_QMF_24);
                }
                UPD_QMF_24 => {
                    sin_ptr = &mut *((*mdct2qmf_tab).local_sin_24)
                        .as_mut_ptr()
                        .offset(TS_MINUS_ONE_24 as isize) as *mut WORD32;
                    ixheaacd_local_p_nonzero(b, sin_ptr, l);
                }
                _ => {}
            }
        }
        UPD_QMF_30 => {
            match p {
                ZERO => {
                    sin_ptr = ((*mdct2qmf_tab).local_sin_30).as_mut_ptr();
                    ixheaacd_local_p_zero(b, sin_ptr, l, UPD_QMF_30);
                }
                UPD_QMF_30 => {
                    sin_ptr = &mut *((*mdct2qmf_tab).local_sin_30)
                        .as_mut_ptr()
                        .offset(TS_MINUS_ONE_30 as isize) as *mut WORD32;
                    ixheaacd_local_p_nonzero(b, sin_ptr, l);
                }
                _ => {}
            }
        }
        UPD_QMF_32 => {
            match p {
                ZERO => {
                    sin_ptr = ((*mdct2qmf_tab).local_sin_32).as_mut_ptr();
                    ixheaacd_local_p_zero(b, sin_ptr, l, UPD_QMF_32);
                }
                UPD_QMF_32 => {
                    sin_ptr = &mut *((*mdct2qmf_tab).local_sin_32)
                        .as_mut_ptr()
                        .offset(TS_MINUS_ONE_32 as isize) as *mut WORD32;
                    ixheaacd_local_p_nonzero(b, sin_ptr, l);
                }
                _ => {}
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn ixheaacd_local_one(l: WORD32, b: *mut WORD32) -> VOID {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < l {
        *b.offset(i as isize) = ONE_IN_Q15 as WORD32;
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_local_freq_win(
    l: WORD32,
    b: *mut WORD32,
    mut wf: *mut *const WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut odd_length: WORD32 = l & ONE_BIT_MASK;
    let mut temp_1: WORD32 = 0;
    let mut b_start: *mut WORD32 = 0 as *mut WORD32;
    let mut b_end: *mut WORD32 = 0 as *mut WORD32;
    b_start = b;
    b_end = b
        .offset((2 as WORD32 * l) as isize)
        .offset(-(1 as core::ffi::c_int as isize))
        .offset(-(odd_length as isize));
    i = 0 as core::ffi::c_int as WORD32;
    while i < l - odd_length {
        temp_1 = *(*wf.offset((l as core::ffi::c_int - 1 as core::ffi::c_int) as isize))
            .offset(i as isize);
        let fresh60 = b_start;
        b_start = b_start.offset(1);
        *fresh60 = temp_1;
        let fresh61 = b_end;
        b_end = b_end.offset(-1);
        *fresh61 = temp_1;
        i += 1;
    }
    if odd_length == 1 as core::ffi::c_int {
        *b_start = *(*wf
            .offset((l as core::ffi::c_int - 1 as core::ffi::c_int) as isize))
            .offset((l as core::ffi::c_int - 1 as core::ffi::c_int) as isize);
        *b_start.offset(l as isize) = 0 as core::ffi::c_int as WORD32;
    }
}
unsafe extern "C" fn ixheaacd_local_mdct_win(
    mut upd_qmf: WORD32,
    window_type: WORD32,
    wf: *mut WORD32,
    mut wf_tab: *mut *const WORD32,
    wt: *mut WORD32,
    mut mdct2qmf_tab: *mut ia_mps_dec_mdct2qmf_table_struct,
) -> VOID {
    let mut length: WORD32 = 0 as WORD32;
    let mut length_right: WORD32 = 0 as WORD32;
    let mut length_left: WORD32 = 0 as WORD32;
    let mut length_const1: WORD32 = 0 as WORD32;
    let mut length_const2: WORD32 = 0 as WORD32;
    match window_type {
        ONLY_LONG_SEQUENCE => {
            length = upd_qmf;
            ixheaacd_local_sin(
                upd_qmf,
                0 as WORD32,
                length << 1 as core::ffi::c_int,
                &mut *wt.offset(0 as core::ffi::c_int as isize),
                mdct2qmf_tab,
            );
        }
        LONG_START_SEQUENCE => {
            match upd_qmf {
                UPD_QMF_15 => {
                    length_const1 = 6 as core::ffi::c_int as WORD32;
                    length_const2 = 7 as core::ffi::c_int as WORD32;
                    length_right = 2 as core::ffi::c_int as WORD32;
                }
                UPD_QMF_16 | UPD_QMF_32 => {
                    length_const1 = 7 as WORD32 * (upd_qmf >> 4 as core::ffi::c_int);
                    length_const2 = length_const1;
                    length_right = upd_qmf >> 3 as core::ffi::c_int;
                }
                UPD_QMF_18 => {
                    length_const1 = 8 as core::ffi::c_int as WORD32;
                    length_const2 = length_const1;
                    length_right = 2 as core::ffi::c_int as WORD32;
                }
                UPD_QMF_24 => {
                    length_const1 = 11 as core::ffi::c_int as WORD32;
                    length_const2 = length_const1;
                    length_right = 2 as core::ffi::c_int as WORD32;
                }
                UPD_QMF_30 => {
                    length_const1 = 14 as core::ffi::c_int as WORD32;
                    length_const2 = length_const1;
                    length_right = 2 as core::ffi::c_int as WORD32;
                }
                _ => {
                    length_const1 = 6 as core::ffi::c_int as WORD32;
                    length_const2 = 7 as core::ffi::c_int as WORD32;
                    length_right = 2 as core::ffi::c_int as WORD32;
                }
            }
            ixheaacd_local_sin(
                upd_qmf,
                0 as WORD32,
                upd_qmf,
                &mut *wt.offset(0 as core::ffi::c_int as isize),
                mdct2qmf_tab,
            );
            ixheaacd_local_one(length_const1, &mut *wt.offset(upd_qmf as isize));
            ixheaacd_local_sin(
                length_right,
                length_right,
                length_right,
                &mut *wt.offset((upd_qmf + length_const1) as isize),
                mdct2qmf_tab,
            );
            ixheaacd_local_zero(
                length_const2,
                &mut *wt.offset((upd_qmf + length_const1 + length_right) as isize),
            );
            length = upd_qmf;
        }
        EIGHT_SHORT_SEQUENCE => {
            match upd_qmf {
                UPD_QMF_16 | UPD_QMF_32 => {
                    length = upd_qmf >> 3 as core::ffi::c_int;
                }
                UPD_QMF_15 | UPD_QMF_18 | UPD_QMF_30 | UPD_QMF_24 => {
                    length = 2 as core::ffi::c_int as WORD32;
                }
                _ => {}
            }
            ixheaacd_local_sin(
                length,
                0 as WORD32,
                2 as WORD32 * length,
                &mut *wt.offset(0 as core::ffi::c_int as isize),
                mdct2qmf_tab,
            );
        }
        LONG_STOP_SEQUENCE => {
            match upd_qmf {
                UPD_QMF_15 => {
                    length_const1 = 6 as core::ffi::c_int as WORD32;
                    length_const2 = 7 as core::ffi::c_int as WORD32;
                    length_left = 2 as core::ffi::c_int as WORD32;
                }
                UPD_QMF_16 | UPD_QMF_32 => {
                    length_const1 = 7 as WORD32 * (upd_qmf >> 4 as core::ffi::c_int);
                    length_const2 = length_const1;
                    length_left = upd_qmf >> 3 as core::ffi::c_int;
                }
                UPD_QMF_18 => {
                    length_const1 = 8 as core::ffi::c_int as WORD32;
                    length_const2 = length_const1;
                    length_left = 2 as core::ffi::c_int as WORD32;
                }
                UPD_QMF_24 => {
                    length_const1 = 11 as core::ffi::c_int as WORD32;
                    length_const2 = length_const1;
                    length_left = 2 as core::ffi::c_int as WORD32;
                }
                UPD_QMF_30 => {
                    length_const1 = 14 as core::ffi::c_int as WORD32;
                    length_const2 = length_const1;
                    length_left = 2 as core::ffi::c_int as WORD32;
                }
                _ => {}
            }
            ixheaacd_local_zero(
                length_const1,
                &mut *wt.offset(0 as core::ffi::c_int as isize),
            );
            ixheaacd_local_sin(
                length_left,
                0 as WORD32,
                length_left,
                &mut *wt.offset(length_const1 as isize),
                mdct2qmf_tab,
            );
            ixheaacd_local_one(
                length_const2,
                &mut *wt.offset((length_const1 + length_left) as isize),
            );
            ixheaacd_local_sin(
                upd_qmf,
                upd_qmf,
                upd_qmf,
                &mut *wt.offset((length_const1 + length_left + length_const2) as isize),
                mdct2qmf_tab,
            );
            length = upd_qmf;
        }
        _ => {}
    }
    ixheaacd_local_freq_win(
        length,
        &mut *wf.offset(0 as core::ffi::c_int as isize),
        wf_tab,
    );
    if upd_qmf == UPD_QMF_15 && window_type == EIGHT_SHORT_SEQUENCE {
        let mut length2: WORD32 = 3 as WORD32;
        ixheaacd_local_sin(
            length,
            0 as WORD32,
            length,
            &mut *wt.offset((length << 1 as core::ffi::c_int) as isize),
            mdct2qmf_tab,
        );
        ixheaacd_local_one(
            1 as WORD32,
            &mut *wt.offset((3 as WORD32 * length) as isize),
        );
        ixheaacd_local_sin(
            length,
            length,
            length,
            &mut *wt
                .offset(
                    (3 as core::ffi::c_int * length as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ),
            mdct2qmf_tab,
        );
        ixheaacd_local_zero(
            1 as WORD32,
            &mut *wt
                .offset(
                    (((length as core::ffi::c_int) << 2 as core::ffi::c_int)
                        + 1 as core::ffi::c_int) as isize,
                ),
        );
        ixheaacd_local_freq_win(
            length2,
            &mut *wf.offset((length << 1 as core::ffi::c_int) as isize),
            wf_tab,
        );
    }
}
unsafe extern "C" fn ixheaacd_get_gain(mut l: WORD32, mut gain: *mut WORD32) -> VOID {
    match l {
        TSX2_4 => {
            *gain = 16384 as core::ffi::c_int as WORD32;
        }
        TSX2_6 => {
            *gain = 13377 as core::ffi::c_int as WORD32;
        }
        TSX2_8 => {
            *gain = 11585 as core::ffi::c_int as WORD32;
        }
        TSX2_30 => {
            *gain = 5982 as core::ffi::c_int as WORD32;
        }
        TSX2_32 => {
            *gain = 5792 as core::ffi::c_int as WORD32;
        }
        TSX2_36 => {
            *gain = 5461 as core::ffi::c_int as WORD32;
        }
        TSX2_48 => {
            *gain = 4729 as core::ffi::c_int as WORD32;
        }
        TSX2_60 => {
            *gain = 4230 as core::ffi::c_int as WORD32;
        }
        TSX2_64 => {
            *gain = 4096 as core::ffi::c_int as WORD32;
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mdct2qmf_process(
    mut upd_qmf: WORD32,
    mdct_in: *mut WORD32,
    mut qmf_real_pre: *mut WORD32,
    mut qmf_real_post: *mut WORD32,
    mut qmf_imag_pre: *mut WORD32,
    mut qmf_imag_post: *mut WORD32,
    window_type: WORD32,
    mut qmf_global_offset: WORD32,
    mut ia_mps_dec_mps_table_ptr: *mut ia_mps_dec_mps_tables_struct,
    mut scratch: *mut core::ffi::c_void,
    mut time_slots: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut l: WORD32 = upd_qmf << 1 as core::ffi::c_int;
    let mut n: WORD32 = 0 as WORD32;
    let mut wf: *mut WORD32 = 0 as *mut WORD32;
    let mut wt: *mut WORD32 = 0 as *mut WORD32;
    let mut v1: *mut WORD32 = 0 as *mut WORD32;
    let mut v2: *mut WORD32 = 0 as *mut WORD32;
    let mut z1_real: *mut WORD32 = 0 as *mut WORD32;
    let mut z1_imag: *mut WORD32 = 0 as *mut WORD32;
    let mut twipost_real: *mut WORD32 = 0 as *mut WORD32;
    let mut twipost_imag: *mut WORD32 = 0 as *mut WORD32;
    let mut mdct_sf: *mut WORD32 = 0 as *mut WORD32;
    let mut mdct_sf_ptr: *mut WORD32 = 0 as *mut WORD32;
    let mut mdct_p: *mut WORD32 = 0 as *mut WORD32;
    let mut mdct_p2: *mut WORD32 = 0 as *mut WORD32;
    let mut temp_2: WORD32 = 0;
    let mut temp3: WORD32 = 0;
    let mut temp4: WORD32 = 0;
    let mut p_qmf_real_pre: *mut WORD32 = qmf_real_pre;
    let mut p_qmf_real_post: *mut WORD32 = qmf_real_post;
    let mut p_qmf_imag_pre: *mut WORD32 = qmf_imag_pre;
    let mut p_qmf_imag_post: *mut WORD32 = qmf_imag_post;
    let mut free_scratch: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut wf_tab: *mut *const WORD32 = ((*(*ia_mps_dec_mps_table_ptr).wf_tab_ptr).wf)
        .as_mut_ptr();
    let mut window_offset: WORD32 = 0 as WORD32;
    let mut mdct_offset: WORD32 = 0 as WORD32;
    let mut mdct_shift: WORD32 = AAC_SHORT_FRAME_LENGTH;
    let mut qmf_offset: WORD32 = 0 as WORD32;
    let mut qmf_shift: WORD32 = 0 as WORD32;
    let mut n_windows: WORD32 = 0 as WORD32;
    let mut mdct_length: WORD32 = MDCT_LENGTH_LO;
    let mut qmf_bands: WORD32 = 64 as WORD32;
    let mut ptr1: *const WORD32 = 0 as *const WORD32;
    let mut ptr2: *const WORD32 = 0 as *const WORD32;
    let mut is_long: WORD32 = 0;
    let mut zr: *mut WORD32 = 0 as *mut WORD32;
    let mut zi: *mut WORD32 = 0 as *mut WORD32;
    let mut a: *mut WORD32 = 0 as *mut WORD32;
    let mut scale: *mut WORD32 = 0 as *mut WORD32;
    let mut gain: WORD32 = 0 as WORD32;
    let mut wp: *mut WORD32 = 0 as *mut WORD32;
    let mut temp_prod: WORD64 = 0 as WORD64;
    wf = scratch as *mut WORD32;
    wt = wf
        .offset(
            ((144 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    v1 = wt
        .offset(
            ((144 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    v2 = v1
        .offset(
            ((((1024 as core::ffi::c_int) << 1 as core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    twipost_real = v2
        .offset(
            ((((1024 as core::ffi::c_int) << 1 as core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    twipost_imag = twipost_real
        .offset(
            ((64 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    mdct_sf = twipost_imag
        .offset(
            ((64 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    z1_real = mdct_sf
        .offset(
            (((3 as core::ffi::c_int * 128 as core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    z1_imag = z1_real
        .offset(
            ((9216 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    a = z1_imag
        .offset(
            ((9216 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    free_scratch = a
        .offset(
            ((64 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        ) as *mut core::ffi::c_void;
    scale = a;
    ixheaacd_local_mdct_win(
        upd_qmf,
        window_type,
        wf,
        wf_tab,
        wt,
        (*ia_mps_dec_mps_table_ptr).mdct2qmf_table_ptr,
    );
    match window_type {
        ONLY_LONG_SEQUENCE | LONG_START_SEQUENCE | LONG_STOP_SEQUENCE => {
            n = (upd_qmf as core::ffi::c_int * qmf_bands as core::ffi::c_int
                - MDCT_LENGTH_LO) as WORD32;
            if n > 0 as core::ffi::c_int {
                ixheaacd_local_zero(n, &mut *mdct_in.offset(MDCT_LENGTH_LO as isize));
            }
            mdct_length += n;
            ixheaacd_local_fold_out(mdct_in, mdct_length, wf, l, v1, v2);
            wp = wt;
            ixheaacd_get_gain(l, &mut gain);
            k = 0 as core::ffi::c_int as WORD32;
            while k < l {
                let fresh0 = scale;
                scale = scale.offset(1);
                *fresh0 = ixheaacd_mps_mult32_shr_15(gain, *wp);
                wp = wp.offset(1);
                k += 1;
            }
            ixheaacd_local_hybcmdct2qmf(
                v1,
                v2,
                a,
                l,
                z1_real,
                z1_imag,
                (*ia_mps_dec_mps_table_ptr).mdct2qmfcos_tab_ptr,
                free_scratch,
                1 as WORD32,
            );
            ptr1 = ((*(*ia_mps_dec_mps_table_ptr).mdct2qmf_table_ptr).twi_post_cos)
                .as_mut_ptr();
            ptr2 = ((*(*ia_mps_dec_mps_table_ptr).mdct2qmf_table_ptr).twi_post_sin)
                .as_mut_ptr();
            if qmf_global_offset < time_slots {
                if qmf_global_offset + l < time_slots {
                    i = 0 as core::ffi::c_int as WORD32;
                    while i < qmf_bands {
                        let fresh1 = ptr1;
                        ptr1 = ptr1.offset(1);
                        let mut cos_twi: WORD32 = *fresh1;
                        let fresh2 = ptr2;
                        ptr2 = ptr2.offset(1);
                        let mut sin_twi: WORD32 = *fresh2;
                        j = 0 as core::ffi::c_int as WORD32;
                        while j < l {
                            let fresh3 = z1_real;
                            z1_real = z1_real.offset(1);
                            temp3 = *fresh3;
                            let fresh4 = z1_imag;
                            z1_imag = z1_imag.offset(1);
                            temp4 = *fresh4;
                            temp_2 = j + qmf_global_offset;
                            temp_prod = *p_qmf_real_pre.offset(temp_2 as isize)
                                as WORD64;
                            temp_prod = ixheaac_mac32x32in64(temp_prod, cos_twi, temp3);
                            temp_prod = ixheaac_mac32x32in64(temp_prod, -sin_twi, temp4);
                            *p_qmf_real_pre.offset(temp_2 as isize) = ixheaac_sat64_32(
                                temp_prod,
                            );
                            temp_prod = *p_qmf_imag_pre.offset(temp_2 as isize)
                                as WORD64;
                            temp_prod = ixheaac_mac32x32in64(temp_prod, cos_twi, temp4);
                            temp_prod = ixheaac_mac32x32in64(temp_prod, sin_twi, temp3);
                            *p_qmf_imag_pre.offset(temp_2 as isize) = ixheaac_sat64_32(
                                temp_prod,
                            );
                            j += 1;
                        }
                        p_qmf_real_pre = p_qmf_real_pre.offset(MAX_TIME_SLOTS as isize);
                        p_qmf_imag_pre = p_qmf_imag_pre.offset(MAX_TIME_SLOTS as isize);
                        i += 1;
                    }
                } else {
                    i = 0 as core::ffi::c_int as WORD32;
                    while i < qmf_bands {
                        let fresh5 = ptr1;
                        ptr1 = ptr1.offset(1);
                        let mut cos_twi_0: WORD32 = *fresh5;
                        let fresh6 = ptr2;
                        ptr2 = ptr2.offset(1);
                        let mut sin_twi_0: WORD32 = *fresh6;
                        j = 0 as core::ffi::c_int as WORD32;
                        while j < l {
                            let fresh7 = z1_real;
                            z1_real = z1_real.offset(1);
                            temp3 = *fresh7;
                            let fresh8 = z1_imag;
                            z1_imag = z1_imag.offset(1);
                            temp4 = *fresh8;
                            temp_2 = j + qmf_global_offset;
                            if temp_2 < time_slots {
                                temp_prod = *p_qmf_real_pre.offset(temp_2 as isize)
                                    as WORD64;
                                temp_prod = ixheaac_mac32x32in64(
                                    temp_prod,
                                    cos_twi_0,
                                    temp3,
                                );
                                temp_prod = ixheaac_mac32x32in64(
                                    temp_prod,
                                    -sin_twi_0,
                                    temp4,
                                );
                                *p_qmf_real_pre.offset(temp_2 as isize) = ixheaac_sat64_32(
                                    temp_prod,
                                );
                                temp_prod = *p_qmf_imag_pre.offset(temp_2 as isize)
                                    as WORD64;
                                temp_prod = ixheaac_mac32x32in64(
                                    temp_prod,
                                    cos_twi_0,
                                    temp4,
                                );
                                temp_prod = ixheaac_mac32x32in64(
                                    temp_prod,
                                    sin_twi_0,
                                    temp3,
                                );
                                *p_qmf_imag_pre.offset(temp_2 as isize) = ixheaac_sat64_32(
                                    temp_prod,
                                );
                            } else {
                                temp_prod = *p_qmf_real_post
                                    .offset((temp_2 - time_slots) as isize) as WORD64;
                                temp_prod = ixheaac_mac32x32in64(
                                    temp_prod,
                                    cos_twi_0,
                                    temp3,
                                );
                                temp_prod = ixheaac_mac32x32in64(
                                    temp_prod,
                                    -sin_twi_0,
                                    temp4,
                                );
                                *p_qmf_real_post.offset((temp_2 - time_slots) as isize) = ixheaac_sat64_32(
                                    temp_prod,
                                );
                                temp_prod = *p_qmf_imag_post
                                    .offset((temp_2 - time_slots) as isize) as WORD64;
                                temp_prod = ixheaac_mac32x32in64(
                                    temp_prod,
                                    cos_twi_0,
                                    temp4,
                                );
                                temp_prod = ixheaac_mac32x32in64(
                                    temp_prod,
                                    sin_twi_0,
                                    temp3,
                                );
                                *p_qmf_imag_post.offset((temp_2 - time_slots) as isize) = ixheaac_sat64_32(
                                    temp_prod,
                                );
                            }
                            j += 1;
                        }
                        p_qmf_real_pre = p_qmf_real_pre.offset(MAX_TIME_SLOTS as isize);
                        p_qmf_real_post = p_qmf_real_post
                            .offset(MAX_TIME_SLOTS as isize);
                        p_qmf_imag_pre = p_qmf_imag_pre.offset(MAX_TIME_SLOTS as isize);
                        p_qmf_imag_post = p_qmf_imag_post
                            .offset(MAX_TIME_SLOTS as isize);
                        i += 1;
                    }
                }
            } else {
                i = 0 as core::ffi::c_int as WORD32;
                while i < qmf_bands {
                    let fresh9 = ptr1;
                    ptr1 = ptr1.offset(1);
                    let mut cos_twi_1: WORD32 = *fresh9;
                    let fresh10 = ptr2;
                    ptr2 = ptr2.offset(1);
                    let mut sin_twi_1: WORD32 = *fresh10;
                    j = 0 as core::ffi::c_int as WORD32;
                    while j < l {
                        let fresh11 = z1_real;
                        z1_real = z1_real.offset(1);
                        temp3 = *fresh11;
                        let fresh12 = z1_imag;
                        z1_imag = z1_imag.offset(1);
                        temp4 = *fresh12;
                        temp_2 = j + qmf_global_offset;
                        temp_prod = *p_qmf_real_post
                            .offset((temp_2 - time_slots) as isize) as WORD64;
                        temp_prod = ixheaac_mac32x32in64(temp_prod, cos_twi_1, temp3);
                        temp_prod = ixheaac_mac32x32in64(temp_prod, -sin_twi_1, temp4);
                        *p_qmf_real_post.offset((temp_2 - time_slots) as isize) = ixheaac_sat64_32(
                            temp_prod,
                        );
                        temp_prod = *p_qmf_imag_post
                            .offset((temp_2 - time_slots) as isize) as WORD64;
                        temp_prod = ixheaac_mac32x32in64(temp_prod, cos_twi_1, temp4);
                        temp_prod = ixheaac_mac32x32in64(temp_prod, sin_twi_1, temp3);
                        *p_qmf_imag_post.offset((temp_2 - time_slots) as isize) = ixheaac_sat64_32(
                            temp_prod,
                        );
                        j += 1;
                    }
                    p_qmf_real_post = p_qmf_real_post.offset(MAX_TIME_SLOTS as isize);
                    p_qmf_imag_post = p_qmf_imag_post.offset(MAX_TIME_SLOTS as isize);
                    i += 1;
                }
            }
        }
        EIGHT_SHORT_SEQUENCE => {
            match upd_qmf {
                UPD_QMF_15 => {
                    l = 4 as core::ffi::c_int as WORD32;
                    mdct_length = AAC_SHORT_FRAME_LENGTH as WORD32;
                    qmf_offset = 6 as core::ffi::c_int as WORD32;
                    qmf_shift = 2 as core::ffi::c_int as WORD32;
                    n_windows = 7 as core::ffi::c_int as WORD32;
                }
                UPD_QMF_16 | UPD_QMF_32 => {
                    n = ((upd_qmf as core::ffi::c_int - UPD_QMF_16)
                        * 8 as core::ffi::c_int) as WORD32;
                    mdct_length = AAC_SHORT_FRAME_LENGTH + n;
                    l = 2 as WORD32 * (upd_qmf >> 3 as core::ffi::c_int);
                    qmf_offset = 7 as WORD32 * upd_qmf >> 4 as core::ffi::c_int;
                    qmf_shift = upd_qmf >> 3 as core::ffi::c_int;
                    n_windows = 8 as core::ffi::c_int as WORD32;
                }
                UPD_QMF_18 => {
                    l = 4 as core::ffi::c_int as WORD32;
                    mdct_length = AAC_SHORT_FRAME_LENGTH as WORD32;
                    qmf_offset = 8 as core::ffi::c_int as WORD32;
                    qmf_shift = 2 as core::ffi::c_int as WORD32;
                    n_windows = 9 as core::ffi::c_int as WORD32;
                }
                UPD_QMF_24 => {
                    l = 4 as core::ffi::c_int as WORD32;
                    mdct_length = AAC_SHORT_FRAME_LENGTH as WORD32;
                    qmf_offset = 11 as core::ffi::c_int as WORD32;
                    qmf_shift = 2 as core::ffi::c_int as WORD32;
                    n_windows = 12 as core::ffi::c_int as WORD32;
                }
                UPD_QMF_30 => {
                    l = 4 as core::ffi::c_int as WORD32;
                    mdct_length = AAC_SHORT_FRAME_LENGTH as WORD32;
                    qmf_offset = 14 as core::ffi::c_int as WORD32;
                    qmf_shift = 2 as core::ffi::c_int as WORD32;
                    n_windows = 15 as core::ffi::c_int as WORD32;
                }
                _ => {
                    l = 4 as core::ffi::c_int as WORD32;
                    mdct_length = AAC_SHORT_FRAME_LENGTH as WORD32;
                    qmf_offset = 6 as core::ffi::c_int as WORD32;
                    qmf_shift = 2 as core::ffi::c_int as WORD32;
                    n_windows = 7 as core::ffi::c_int as WORD32;
                }
            }
            wp = wt;
            ixheaacd_get_gain(l, &mut gain);
            k = 0 as core::ffi::c_int as WORD32;
            while k < l {
                let fresh13 = scale;
                scale = scale.offset(1);
                *fresh13 = ixheaacd_mps_mult32_shr_15(gain, *wp);
                wp = wp.offset(1);
                k += 1;
            }
            k = 0 as core::ffi::c_int as WORD32;
            while k < n_windows {
                is_long = 0 as core::ffi::c_int as WORD32;
                mdct_sf_ptr = mdct_sf;
                match upd_qmf {
                    UPD_QMF_16 | UPD_QMF_32 => {
                        mdct_p = mdct_in.offset(mdct_offset as isize);
                        i = 0 as core::ffi::c_int as WORD32;
                        while i < AAC_SHORT_FRAME_LENGTH {
                            let fresh14 = mdct_p;
                            mdct_p = mdct_p.offset(1);
                            let fresh15 = mdct_sf_ptr;
                            mdct_sf_ptr = mdct_sf_ptr.offset(1);
                            *fresh15 = *fresh14;
                            i += 1;
                        }
                        ixheaacd_local_zero(
                            n,
                            &mut *mdct_sf.offset(AAC_SHORT_FRAME_LENGTH as isize),
                        );
                    }
                    UPD_QMF_15 => {
                        if k < n_windows as core::ffi::c_int - 1 as core::ffi::c_int {
                            mdct_p = mdct_in.offset(mdct_offset as isize);
                            i = 0 as core::ffi::c_int as WORD32;
                            while i < AAC_SHORT_FRAME_LENGTH {
                                let fresh16 = mdct_p;
                                mdct_p = mdct_p.offset(1);
                                let fresh17 = mdct_sf_ptr;
                                mdct_sf_ptr = mdct_sf_ptr.offset(1);
                                *fresh17 = *fresh16;
                                i += 1;
                            }
                        } else {
                            window_offset = l;
                            l = 6 as core::ffi::c_int as WORD32;
                            mdct_length = 192 as core::ffi::c_int as WORD32;
                            is_long = 4 as core::ffi::c_int as WORD32;
                            gain = 13377 as core::ffi::c_int as WORD32;
                            scale = a;
                            wp = wt.offset(window_offset as isize);
                            k = 0 as core::ffi::c_int as WORD32;
                            while k < l {
                                let fresh18 = scale;
                                scale = scale.offset(1);
                                *fresh18 = ixheaacd_mps_mult32_shr_15(gain, *wp);
                                wp = wp.offset(1);
                                k += 1;
                            }
                            mdct_p = mdct_in.offset(mdct_offset as isize);
                            mdct_p2 = mdct_in
                                .offset(mdct_offset as isize)
                                .offset(AAC_SHORT_FRAME_LENGTH as isize);
                            i = 0 as core::ffi::c_int as WORD32;
                            j = 0 as core::ffi::c_int as WORD32;
                            while i
                                < mdct_length as core::ffi::c_int / 2 as core::ffi::c_int
                            {
                                let fresh19 = mdct_p;
                                mdct_p = mdct_p.offset(1);
                                let fresh20 = mdct_sf_ptr;
                                mdct_sf_ptr = mdct_sf_ptr.offset(1);
                                *fresh20 = *fresh19;
                                let fresh21 = mdct_p2;
                                mdct_p2 = mdct_p2.offset(1);
                                let fresh22 = mdct_sf_ptr;
                                mdct_sf_ptr = mdct_sf_ptr.offset(1);
                                *fresh22 = *fresh21;
                                i += 1;
                            }
                        }
                    }
                    UPD_QMF_18 | UPD_QMF_24 | UPD_QMF_30 => {
                        mdct_p = mdct_in.offset(mdct_offset as isize);
                        i = 0 as core::ffi::c_int as WORD32;
                        while i < AAC_SHORT_FRAME_LENGTH {
                            let fresh23 = mdct_p;
                            mdct_p = mdct_p.offset(1);
                            let fresh24 = mdct_sf_ptr;
                            mdct_sf_ptr = mdct_sf_ptr.offset(1);
                            *fresh24 = *fresh23;
                            i += 1;
                        }
                    }
                    _ => {}
                }
                ixheaacd_local_fold_out(
                    mdct_sf,
                    mdct_length,
                    &mut *wf.offset(window_offset as isize),
                    l,
                    v1,
                    v2,
                );
                ixheaacd_local_hybcmdct2qmf(
                    v1,
                    v2,
                    a,
                    l,
                    z1_real,
                    z1_imag,
                    (*ia_mps_dec_mps_table_ptr).mdct2qmfcos_tab_ptr,
                    free_scratch,
                    is_long,
                );
                zr = z1_real;
                zi = z1_imag;
                ptr1 = ((*(*ia_mps_dec_mps_table_ptr).mdct2qmf_table_ptr).twi_post_cos)
                    .as_mut_ptr();
                ptr2 = ((*(*ia_mps_dec_mps_table_ptr).mdct2qmf_table_ptr).twi_post_sin)
                    .as_mut_ptr();
                temp_2 = qmf_offset + qmf_global_offset;
                if temp_2 < time_slots {
                    if temp_2 + l < time_slots {
                        p_qmf_real_pre = qmf_real_pre;
                        p_qmf_imag_pre = qmf_imag_pre;
                        i = 0 as core::ffi::c_int as WORD32;
                        while i < qmf_bands {
                            let fresh25 = ptr1;
                            ptr1 = ptr1.offset(1);
                            let mut cos_twi_2: WORD32 = *fresh25;
                            let fresh26 = ptr2;
                            ptr2 = ptr2.offset(1);
                            let mut sin_twi_2: WORD32 = *fresh26;
                            j = 0 as core::ffi::c_int as WORD32;
                            while j < l {
                                let fresh27 = zr;
                                zr = zr.offset(1);
                                temp3 = *fresh27;
                                let fresh28 = zi;
                                zi = zi.offset(1);
                                temp4 = *fresh28;
                                temp_prod = *p_qmf_real_pre.offset((temp_2 + j) as isize)
                                    as WORD64;
                                temp_prod = ixheaac_mac32x32in64(
                                    temp_prod,
                                    cos_twi_2,
                                    temp3,
                                );
                                temp_prod = ixheaac_mac32x32in64(
                                    temp_prod,
                                    -sin_twi_2,
                                    temp4,
                                );
                                *p_qmf_real_pre.offset((temp_2 + j) as isize) = ixheaac_sat64_32(
                                    temp_prod,
                                );
                                temp_prod = *p_qmf_imag_pre.offset((temp_2 + j) as isize)
                                    as WORD64;
                                temp_prod = ixheaac_mac32x32in64(
                                    temp_prod,
                                    cos_twi_2,
                                    temp4,
                                );
                                temp_prod = ixheaac_mac32x32in64(
                                    temp_prod,
                                    sin_twi_2,
                                    temp3,
                                );
                                *p_qmf_imag_pre.offset((temp_2 + j) as isize) = ixheaac_sat64_32(
                                    temp_prod,
                                );
                                j += 1;
                            }
                            p_qmf_real_pre = p_qmf_real_pre
                                .offset(MAX_TIME_SLOTS as isize);
                            p_qmf_imag_pre = p_qmf_imag_pre
                                .offset(MAX_TIME_SLOTS as isize);
                            i += 1;
                        }
                    } else {
                        p_qmf_real_pre = qmf_real_pre;
                        p_qmf_real_post = qmf_real_post;
                        p_qmf_imag_pre = qmf_imag_pre;
                        p_qmf_imag_post = qmf_imag_post;
                        i = 0 as core::ffi::c_int as WORD32;
                        while i < qmf_bands {
                            let fresh29 = ptr1;
                            ptr1 = ptr1.offset(1);
                            let mut cos_twi_3: WORD32 = *fresh29;
                            let fresh30 = ptr2;
                            ptr2 = ptr2.offset(1);
                            let mut sin_twi_3: WORD32 = *fresh30;
                            j = 0 as core::ffi::c_int as WORD32;
                            while j < l {
                                let fresh31 = zr;
                                zr = zr.offset(1);
                                temp3 = *fresh31;
                                let fresh32 = zi;
                                zi = zi.offset(1);
                                temp4 = *fresh32;
                                if temp_2 + j < time_slots {
                                    temp_prod = *p_qmf_real_pre.offset((temp_2 + j) as isize)
                                        as WORD64;
                                    temp_prod = ixheaac_mac32x32in64(
                                        temp_prod,
                                        cos_twi_3,
                                        temp3,
                                    );
                                    temp_prod = ixheaac_mac32x32in64(
                                        temp_prod,
                                        -sin_twi_3,
                                        temp4,
                                    );
                                    *p_qmf_real_pre.offset((temp_2 + j) as isize) = ixheaac_sat64_32(
                                        temp_prod,
                                    );
                                    temp_prod = *p_qmf_imag_pre.offset((temp_2 + j) as isize)
                                        as WORD64;
                                    temp_prod = ixheaac_mac32x32in64(
                                        temp_prod,
                                        cos_twi_3,
                                        temp4,
                                    );
                                    temp_prod = ixheaac_mac32x32in64(
                                        temp_prod,
                                        sin_twi_3,
                                        temp3,
                                    );
                                    *p_qmf_imag_pre.offset((temp_2 + j) as isize) = ixheaac_sat64_32(
                                        temp_prod,
                                    );
                                } else {
                                    temp_prod = *p_qmf_real_post
                                        .offset((temp_2 + j - time_slots) as isize) as WORD64;
                                    temp_prod = ixheaac_mac32x32in64(
                                        temp_prod,
                                        cos_twi_3,
                                        temp3,
                                    );
                                    temp_prod = ixheaac_mac32x32in64(
                                        temp_prod,
                                        -sin_twi_3,
                                        temp4,
                                    );
                                    *p_qmf_real_post
                                        .offset((temp_2 + j - time_slots) as isize) = ixheaac_sat64_32(
                                        temp_prod,
                                    );
                                    temp_prod = *p_qmf_imag_post
                                        .offset((temp_2 + j - time_slots) as isize) as WORD64;
                                    temp_prod = ixheaac_mac32x32in64(
                                        temp_prod,
                                        cos_twi_3,
                                        temp4,
                                    );
                                    temp_prod = ixheaac_mac32x32in64(
                                        temp_prod,
                                        sin_twi_3,
                                        temp3,
                                    );
                                    *p_qmf_imag_post
                                        .offset((temp_2 + j - time_slots) as isize) = ixheaac_sat64_32(
                                        temp_prod,
                                    );
                                }
                                j += 1;
                            }
                            p_qmf_real_pre = p_qmf_real_pre
                                .offset(MAX_TIME_SLOTS as isize);
                            p_qmf_imag_pre = p_qmf_imag_pre
                                .offset(MAX_TIME_SLOTS as isize);
                            p_qmf_real_post = p_qmf_real_post
                                .offset(MAX_TIME_SLOTS as isize);
                            p_qmf_imag_post = p_qmf_imag_post
                                .offset(MAX_TIME_SLOTS as isize);
                            i += 1;
                        }
                    }
                } else {
                    p_qmf_real_post = qmf_real_post;
                    p_qmf_imag_post = qmf_imag_post;
                    i = 0 as core::ffi::c_int as WORD32;
                    while i < qmf_bands {
                        let fresh33 = ptr1;
                        ptr1 = ptr1.offset(1);
                        let mut cos_twi_4: WORD32 = *fresh33;
                        let fresh34 = ptr2;
                        ptr2 = ptr2.offset(1);
                        let mut sin_twi_4: WORD32 = *fresh34;
                        j = 0 as core::ffi::c_int as WORD32;
                        while j < l {
                            let fresh35 = zr;
                            zr = zr.offset(1);
                            temp3 = *fresh35;
                            let fresh36 = zi;
                            zi = zi.offset(1);
                            temp4 = *fresh36;
                            temp_prod = *p_qmf_real_post
                                .offset((temp_2 + j - time_slots) as isize) as WORD64;
                            temp_prod = ixheaac_mac32x32in64(
                                temp_prod,
                                cos_twi_4,
                                temp3,
                            );
                            temp_prod = ixheaac_mac32x32in64(
                                temp_prod,
                                -sin_twi_4,
                                temp4,
                            );
                            *p_qmf_real_post
                                .offset((temp_2 + j - time_slots) as isize) = ixheaac_sat64_32(
                                temp_prod,
                            );
                            temp_prod = *p_qmf_imag_post
                                .offset((temp_2 + j - time_slots) as isize) as WORD64;
                            temp_prod = ixheaac_mac32x32in64(
                                temp_prod,
                                cos_twi_4,
                                temp4,
                            );
                            temp_prod = ixheaac_mac32x32in64(
                                temp_prod,
                                sin_twi_4,
                                temp3,
                            );
                            *p_qmf_imag_post
                                .offset((temp_2 + j - time_slots) as isize) = ixheaac_sat64_32(
                                temp_prod,
                            );
                            j += 1;
                        }
                        p_qmf_real_post = p_qmf_real_post
                            .offset(MAX_TIME_SLOTS as isize);
                        p_qmf_imag_post = p_qmf_imag_post
                            .offset(MAX_TIME_SLOTS as isize);
                        i += 1;
                    }
                }
                mdct_offset += mdct_shift;
                qmf_offset += qmf_shift;
                k += 1;
            }
        }
        _ => {}
    };
}
