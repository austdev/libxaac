extern "C" {
    pub type ia_mps_dec_ducker_interface;
    fn memmove(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
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
pub type ia_mps_dec_synthesis_interface_handle = *mut ia_mps_dec_synthesis_interface;
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
pub const IA_XHEAAC_MPS_DEC_INIT_NONFATAL_INVALID_QMF_BAND: core::ffi::c_int = 0x1001
    as core::ffi::c_int;
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
unsafe extern "C" fn ixheaac_mult32x16in32_shl(mut a: WORD32, mut b: WORD16) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 16 as core::ffi::c_int) as WORD32;
    return result << 1 as core::ffi::c_int;
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
pub const QMF_BANDS_32: core::ffi::c_int = 32;
pub const QMF_BANDS_64: core::ffi::c_int = 64;
pub const QMF_BANDS_128: core::ffi::c_int = 128;
pub const MAX_NUM_QMF_BANDS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const MAX_HYBRID_BANDS: core::ffi::c_int = MAX_NUM_QMF_BANDS - 3 as core::ffi::c_int
    + 10 as core::ffi::c_int;
pub const QMF_FILTER_STATE_SYN_SIZE_MPS: core::ffi::c_int = 576 as core::ffi::c_int;
pub const INV_SQRT2_Q15: core::ffi::c_int = 23170 as core::ffi::c_int;
pub const COS_3PI_BY_8_Q15: core::ffi::c_int = 12540 as core::ffi::c_int;
pub const SQRT2PLUS1_Q13: core::ffi::c_int = 19777 as core::ffi::c_int;
pub const SQRT2MINUS1_Q15: core::ffi::c_int = 13573 as core::ffi::c_int;
pub const COS_PI_BY_8_Q15: core::ffi::c_int = 30274 as core::ffi::c_int;
pub const ONE_BIT_MASK: core::ffi::c_int = 0x1 as core::ffi::c_int;
pub const WORD_LENGTH: core::ffi::c_int = 32 as core::ffi::c_int;
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
pub const IA_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
unsafe extern "C" fn ixheaacd_fft32(
    mut vec: *mut WORD32,
    mut fft_c: *const WORD16,
) -> VOID {
    let mut tmp0: WORD32 = 0;
    let mut tmp1: WORD32 = 0;
    let mut tmp2: WORD32 = 0;
    let mut tmp3: WORD32 = 0;
    let mut tmp4: WORD32 = 0;
    let mut tmp5: WORD32 = 0;
    let mut tmp6: WORD32 = 0;
    let mut tmp7: WORD32 = 0;
    let mut tmp8: WORD32 = 0;
    let mut tmp9: WORD32 = 0;
    let mut tmp10: WORD32 = 0;
    let mut tmp11: WORD32 = 0;
    let mut tmp12: WORD32 = 0;
    let mut tmp13: WORD32 = 0;
    let mut tmp14: WORD32 = 0;
    let mut tmp15: WORD32 = 0;
    let mut temp10: WORD32 = 0;
    let mut temp11: WORD32 = 0;
    let mut temp12: WORD32 = 0;
    let mut temp13: WORD32 = 0;
    let mut temp14: WORD32 = 0;
    let mut temp15: WORD32 = 0;
    let mut temp16: WORD32 = 0;
    let mut temp17: WORD32 = 0;
    let mut temp18: WORD32 = 0;
    let mut temp19: WORD32 = 0;
    let mut temp110: WORD32 = 0;
    let mut temp111: WORD32 = 0;
    let mut temp112: WORD32 = 0;
    let mut temp113: WORD32 = 0;
    let mut temp114: WORD32 = 0;
    let mut temp115: WORD32 = 0;
    let mut temp20: WORD32 = 0;
    let mut temp21: WORD32 = 0;
    let mut temp22: WORD32 = 0;
    let mut temp23: WORD32 = 0;
    let mut temp24: WORD32 = 0;
    let mut temp25: WORD32 = 0;
    let mut temp26: WORD32 = 0;
    let mut temp27: WORD32 = 0;
    let mut temp28: WORD32 = 0;
    let mut temp29: WORD32 = 0;
    let mut temp210: WORD32 = 0;
    let mut temp211: WORD32 = 0;
    let mut temp212: WORD32 = 0;
    let mut temp213: WORD32 = 0;
    let mut temp214: WORD32 = 0;
    let mut temp215: WORD32 = 0;
    let mut temp30: WORD32 = 0;
    let mut temp31: WORD32 = 0;
    let mut temp32: WORD32 = 0;
    let mut temp33: WORD32 = 0;
    let mut temp34: WORD32 = 0;
    let mut temp35: WORD32 = 0;
    let mut temp36: WORD32 = 0;
    let mut temp37: WORD32 = 0;
    let mut temp38: WORD32 = 0;
    let mut temp39: WORD32 = 0;
    let mut temp310: WORD32 = 0;
    let mut temp311: WORD32 = 0;
    let mut temp312: WORD32 = 0;
    let mut temp313: WORD32 = 0;
    let mut temp314: WORD32 = 0;
    let mut temp315: WORD32 = 0;
    let mut temp316: WORD32 = 0;
    let mut temp317: WORD32 = 0;
    let mut temp318: WORD32 = 0;
    let mut temp319: WORD32 = 0;
    let mut temp320: WORD32 = 0;
    let mut temp321: WORD32 = 0;
    let mut temp322: WORD32 = 0;
    let mut temp323: WORD32 = 0;
    let mut temp324: WORD32 = 0;
    let mut temp325: WORD32 = 0;
    let mut temp326: WORD32 = 0;
    let mut temp327: WORD32 = 0;
    let mut temp328: WORD32 = 0;
    let mut temp329: WORD32 = 0;
    let mut temp330: WORD32 = 0;
    let mut temp331: WORD32 = 0;
    let mut temp40: WORD32 = 0;
    let mut temp41: WORD32 = 0;
    let mut temp42: WORD32 = 0;
    let mut temp43: WORD32 = 0;
    let mut temp44: WORD32 = 0;
    let mut temp45: WORD32 = 0;
    let mut temp46: WORD32 = 0;
    let mut temp47: WORD32 = 0;
    let mut temp48: WORD32 = 0;
    let mut temp49: WORD32 = 0;
    let mut temp410: WORD32 = 0;
    let mut temp411: WORD32 = 0;
    let mut temp412: WORD32 = 0;
    let mut temp413: WORD32 = 0;
    let mut temp414: WORD32 = 0;
    let mut temp415: WORD32 = 0;
    temp20 = ixheaac_sub32_sat(
        *vec.offset(2 as core::ffi::c_int as isize),
        *vec.offset(34 as core::ffi::c_int as isize),
    );
    temp21 = ixheaac_sub32_sat(
        *vec.offset(3 as core::ffi::c_int as isize),
        *vec.offset(35 as core::ffi::c_int as isize),
    );
    temp30 = ixheaac_add32_sat(
        *vec.offset(0 as core::ffi::c_int as isize),
        *vec.offset(32 as core::ffi::c_int as isize),
    );
    temp31 = ixheaac_add32_sat(
        *vec.offset(1 as core::ffi::c_int as isize),
        *vec.offset(33 as core::ffi::c_int as isize),
    );
    temp32 = ixheaac_add32_sat(
        *vec.offset(2 as core::ffi::c_int as isize),
        *vec.offset(34 as core::ffi::c_int as isize),
    );
    temp33 = ixheaac_add32_sat(
        *vec.offset(3 as core::ffi::c_int as isize),
        *vec.offset(35 as core::ffi::c_int as isize),
    );
    temp22 = ixheaac_sub32_sat(
        *vec.offset(6 as core::ffi::c_int as isize),
        *vec.offset(38 as core::ffi::c_int as isize),
    );
    temp23 = ixheaac_sub32_sat(
        *vec.offset(7 as core::ffi::c_int as isize),
        *vec.offset(39 as core::ffi::c_int as isize),
    );
    temp34 = ixheaac_add32_sat(
        *vec.offset(4 as core::ffi::c_int as isize),
        *vec.offset(36 as core::ffi::c_int as isize),
    );
    temp35 = ixheaac_add32_sat(
        *vec.offset(5 as core::ffi::c_int as isize),
        *vec.offset(37 as core::ffi::c_int as isize),
    );
    temp36 = ixheaac_add32_sat(
        *vec.offset(6 as core::ffi::c_int as isize),
        *vec.offset(38 as core::ffi::c_int as isize),
    );
    temp37 = ixheaac_add32_sat(
        *vec.offset(7 as core::ffi::c_int as isize),
        *vec.offset(39 as core::ffi::c_int as isize),
    );
    temp24 = ixheaac_sub32_sat(
        *vec.offset(10 as core::ffi::c_int as isize),
        *vec.offset(42 as core::ffi::c_int as isize),
    );
    temp25 = ixheaac_sub32_sat(
        *vec.offset(11 as core::ffi::c_int as isize),
        *vec.offset(43 as core::ffi::c_int as isize),
    );
    temp38 = ixheaac_add32_sat(
        *vec.offset(8 as core::ffi::c_int as isize),
        *vec.offset(40 as core::ffi::c_int as isize),
    );
    temp39 = ixheaac_add32_sat(
        *vec.offset(9 as core::ffi::c_int as isize),
        *vec.offset(41 as core::ffi::c_int as isize),
    );
    temp310 = ixheaac_add32_sat(
        *vec.offset(10 as core::ffi::c_int as isize),
        *vec.offset(42 as core::ffi::c_int as isize),
    );
    temp311 = ixheaac_add32_sat(
        *vec.offset(11 as core::ffi::c_int as isize),
        *vec.offset(43 as core::ffi::c_int as isize),
    );
    temp26 = ixheaac_sub32_sat(
        *vec.offset(14 as core::ffi::c_int as isize),
        *vec.offset(46 as core::ffi::c_int as isize),
    );
    temp27 = ixheaac_sub32_sat(
        *vec.offset(15 as core::ffi::c_int as isize),
        *vec.offset(47 as core::ffi::c_int as isize),
    );
    temp312 = ixheaac_add32_sat(
        *vec.offset(12 as core::ffi::c_int as isize),
        *vec.offset(44 as core::ffi::c_int as isize),
    );
    temp313 = ixheaac_add32_sat(
        *vec.offset(13 as core::ffi::c_int as isize),
        *vec.offset(45 as core::ffi::c_int as isize),
    );
    temp314 = ixheaac_add32_sat(
        *vec.offset(14 as core::ffi::c_int as isize),
        *vec.offset(46 as core::ffi::c_int as isize),
    );
    temp315 = ixheaac_add32_sat(
        *vec.offset(15 as core::ffi::c_int as isize),
        *vec.offset(47 as core::ffi::c_int as isize),
    );
    temp28 = ixheaac_sub32_sat(
        *vec.offset(18 as core::ffi::c_int as isize),
        *vec.offset(50 as core::ffi::c_int as isize),
    );
    temp29 = ixheaac_sub32_sat(
        *vec.offset(19 as core::ffi::c_int as isize),
        *vec.offset(51 as core::ffi::c_int as isize),
    );
    temp316 = ixheaac_add32_sat(
        *vec.offset(16 as core::ffi::c_int as isize),
        *vec.offset(48 as core::ffi::c_int as isize),
    );
    temp317 = ixheaac_add32_sat(
        *vec.offset(17 as core::ffi::c_int as isize),
        *vec.offset(49 as core::ffi::c_int as isize),
    );
    temp318 = ixheaac_add32_sat(
        *vec.offset(18 as core::ffi::c_int as isize),
        *vec.offset(50 as core::ffi::c_int as isize),
    );
    temp319 = ixheaac_add32_sat(
        *vec.offset(19 as core::ffi::c_int as isize),
        *vec.offset(51 as core::ffi::c_int as isize),
    );
    temp210 = ixheaac_sub32_sat(
        *vec.offset(22 as core::ffi::c_int as isize),
        *vec.offset(54 as core::ffi::c_int as isize),
    );
    temp211 = ixheaac_sub32_sat(
        *vec.offset(23 as core::ffi::c_int as isize),
        *vec.offset(55 as core::ffi::c_int as isize),
    );
    temp320 = ixheaac_add32_sat(
        *vec.offset(20 as core::ffi::c_int as isize),
        *vec.offset(52 as core::ffi::c_int as isize),
    );
    temp321 = ixheaac_add32_sat(
        *vec.offset(21 as core::ffi::c_int as isize),
        *vec.offset(53 as core::ffi::c_int as isize),
    );
    temp322 = ixheaac_add32_sat(
        *vec.offset(22 as core::ffi::c_int as isize),
        *vec.offset(54 as core::ffi::c_int as isize),
    );
    temp323 = ixheaac_add32_sat(
        *vec.offset(23 as core::ffi::c_int as isize),
        *vec.offset(55 as core::ffi::c_int as isize),
    );
    temp212 = ixheaac_sub32_sat(
        *vec.offset(26 as core::ffi::c_int as isize),
        *vec.offset(58 as core::ffi::c_int as isize),
    );
    temp213 = ixheaac_sub32_sat(
        *vec.offset(27 as core::ffi::c_int as isize),
        *vec.offset(59 as core::ffi::c_int as isize),
    );
    temp324 = ixheaac_add32_sat(
        *vec.offset(24 as core::ffi::c_int as isize),
        *vec.offset(56 as core::ffi::c_int as isize),
    );
    temp325 = ixheaac_add32_sat(
        *vec.offset(25 as core::ffi::c_int as isize),
        *vec.offset(57 as core::ffi::c_int as isize),
    );
    temp326 = ixheaac_add32_sat(
        *vec.offset(26 as core::ffi::c_int as isize),
        *vec.offset(58 as core::ffi::c_int as isize),
    );
    temp327 = ixheaac_add32_sat(
        *vec.offset(27 as core::ffi::c_int as isize),
        *vec.offset(59 as core::ffi::c_int as isize),
    );
    temp214 = ixheaac_sub32_sat(
        *vec.offset(30 as core::ffi::c_int as isize),
        *vec.offset(62 as core::ffi::c_int as isize),
    );
    temp215 = ixheaac_sub32_sat(
        *vec.offset(31 as core::ffi::c_int as isize),
        *vec.offset(63 as core::ffi::c_int as isize),
    );
    temp328 = ixheaac_add32_sat(
        *vec.offset(28 as core::ffi::c_int as isize),
        *vec.offset(60 as core::ffi::c_int as isize),
    );
    temp329 = ixheaac_add32_sat(
        *vec.offset(29 as core::ffi::c_int as isize),
        *vec.offset(61 as core::ffi::c_int as isize),
    );
    temp330 = ixheaac_add32_sat(
        *vec.offset(30 as core::ffi::c_int as isize),
        *vec.offset(62 as core::ffi::c_int as isize),
    );
    temp331 = ixheaac_add32_sat(
        *vec.offset(31 as core::ffi::c_int as isize),
        *vec.offset(63 as core::ffi::c_int as isize),
    );
    temp41 = ixheaac_negate32_sat(ixheaac_add32_sat(temp20, temp214));
    temp42 = ixheaac_sub32_sat(temp20, temp214);
    temp40 = ixheaac_add32_sat(temp21, temp215);
    temp43 = ixheaac_sub32_sat(temp21, temp215);
    temp45 = ixheaac_negate32_sat(ixheaac_add32_sat(temp22, temp212));
    temp46 = ixheaac_sub32_sat(temp22, temp212);
    temp44 = ixheaac_add32_sat(temp23, temp213);
    temp47 = ixheaac_sub32_sat(temp23, temp213);
    temp49 = ixheaac_negate32_sat(ixheaac_add32_sat(temp24, temp210));
    temp410 = ixheaac_sub32_sat(temp24, temp210);
    temp48 = ixheaac_add32_sat(temp25, temp211);
    temp411 = ixheaac_sub32_sat(temp25, temp211);
    temp413 = ixheaac_negate32_sat(ixheaac_add32_sat(temp26, temp28));
    temp414 = ixheaac_sub32_sat(temp26, temp28);
    temp412 = ixheaac_add32_sat(temp27, temp29);
    temp415 = ixheaac_sub32_sat(temp27, temp29);
    temp20 = ixheaac_add32_sat(
        ixheaac_add32_sat(
            ixheaac_add32_sat(
                ixheaac_mult32x16in32_shl(
                    temp40,
                    *fft_c.offset(3 as core::ffi::c_int as isize),
                ),
                ixheaac_mult32x16in32_shl(
                    temp44,
                    *fft_c.offset(2 as core::ffi::c_int as isize),
                ),
            ),
            ixheaac_mult32x16in32_shl(
                temp48,
                *fft_c.offset(1 as core::ffi::c_int as isize),
            ),
        ),
        ixheaac_mult32x16in32_shl(temp412, *fft_c.offset(0 as core::ffi::c_int as isize)),
    );
    temp24 = ixheaac_sub32_sat(
        ixheaac_add32_sat(
            ixheaac_add32_sat(
                ixheaac_mult32x16in32_shl(
                    temp40,
                    *fft_c.offset(2 as core::ffi::c_int as isize),
                ),
                ixheaac_mult32x16in32_shl(
                    temp44,
                    *fft_c.offset(0 as core::ffi::c_int as isize),
                ),
            ),
            ixheaac_mult32x16in32_shl(
                temp48,
                *fft_c.offset(3 as core::ffi::c_int as isize),
            ),
        ),
        ixheaac_mult32x16in32_shl(temp412, *fft_c.offset(1 as core::ffi::c_int as isize)),
    );
    temp28 = ixheaac_add32_sat(
        ixheaac_sub32_sat(
            ixheaac_add32_sat(
                ixheaac_mult32x16in32_shl(
                    temp40,
                    *fft_c.offset(1 as core::ffi::c_int as isize),
                ),
                ixheaac_mult32x16in32_shl(
                    temp44,
                    *fft_c.offset(3 as core::ffi::c_int as isize),
                ),
            ),
            ixheaac_mult32x16in32_shl(
                temp48,
                *fft_c.offset(0 as core::ffi::c_int as isize),
            ),
        ),
        ixheaac_mult32x16in32_shl(temp412, *fft_c.offset(2 as core::ffi::c_int as isize)),
    );
    temp212 = ixheaac_sub32_sat(
        ixheaac_add32_sat(
            ixheaac_sub32_sat(
                ixheaac_mult32x16in32_shl(
                    temp40,
                    *fft_c.offset(0 as core::ffi::c_int as isize),
                ),
                ixheaac_mult32x16in32_shl(
                    temp44,
                    *fft_c.offset(1 as core::ffi::c_int as isize),
                ),
            ),
            ixheaac_mult32x16in32_shl(
                temp48,
                *fft_c.offset(2 as core::ffi::c_int as isize),
            ),
        ),
        ixheaac_mult32x16in32_shl(temp412, *fft_c.offset(3 as core::ffi::c_int as isize)),
    );
    temp21 = ixheaac_add32_sat(
        ixheaac_add32_sat(
            ixheaac_add32_sat(
                ixheaac_mult32x16in32_shl(
                    temp41,
                    *fft_c.offset(3 as core::ffi::c_int as isize),
                ),
                ixheaac_mult32x16in32_shl(
                    temp45,
                    *fft_c.offset(2 as core::ffi::c_int as isize),
                ),
            ),
            ixheaac_mult32x16in32_shl(
                temp49,
                *fft_c.offset(1 as core::ffi::c_int as isize),
            ),
        ),
        ixheaac_mult32x16in32_shl(temp413, *fft_c.offset(0 as core::ffi::c_int as isize)),
    );
    temp25 = ixheaac_sub32_sat(
        ixheaac_add32_sat(
            ixheaac_add32_sat(
                ixheaac_mult32x16in32_shl(
                    temp41,
                    *fft_c.offset(2 as core::ffi::c_int as isize),
                ),
                ixheaac_mult32x16in32_shl(
                    temp45,
                    *fft_c.offset(0 as core::ffi::c_int as isize),
                ),
            ),
            ixheaac_mult32x16in32_shl(
                temp49,
                *fft_c.offset(3 as core::ffi::c_int as isize),
            ),
        ),
        ixheaac_mult32x16in32_shl(temp413, *fft_c.offset(1 as core::ffi::c_int as isize)),
    );
    temp29 = ixheaac_add32_sat(
        ixheaac_sub32_sat(
            ixheaac_add32_sat(
                ixheaac_mult32x16in32_shl(
                    temp41,
                    *fft_c.offset(1 as core::ffi::c_int as isize),
                ),
                ixheaac_mult32x16in32_shl(
                    temp45,
                    *fft_c.offset(3 as core::ffi::c_int as isize),
                ),
            ),
            ixheaac_mult32x16in32_shl(
                temp49,
                *fft_c.offset(0 as core::ffi::c_int as isize),
            ),
        ),
        ixheaac_mult32x16in32_shl(temp413, *fft_c.offset(2 as core::ffi::c_int as isize)),
    );
    temp213 = ixheaac_sub32_sat(
        ixheaac_add32_sat(
            ixheaac_sub32_sat(
                ixheaac_mult32x16in32_shl(
                    temp41,
                    *fft_c.offset(0 as core::ffi::c_int as isize),
                ),
                ixheaac_mult32x16in32_shl(
                    temp45,
                    *fft_c.offset(1 as core::ffi::c_int as isize),
                ),
            ),
            ixheaac_mult32x16in32_shl(
                temp49,
                *fft_c.offset(2 as core::ffi::c_int as isize),
            ),
        ),
        ixheaac_mult32x16in32_shl(temp413, *fft_c.offset(3 as core::ffi::c_int as isize)),
    );
    temp22 = ixheaac_add32_sat(
        ixheaac_add32_sat(
            ixheaac_add32_sat(
                ixheaac_mult32x16in32_shl(
                    temp42,
                    *fft_c.offset(0 as core::ffi::c_int as isize),
                ),
                ixheaac_mult32x16in32_shl(
                    temp46,
                    *fft_c.offset(1 as core::ffi::c_int as isize),
                ),
            ),
            ixheaac_mult32x16in32_shl(
                temp410,
                *fft_c.offset(2 as core::ffi::c_int as isize),
            ),
        ),
        ixheaac_mult32x16in32_shl(temp414, *fft_c.offset(3 as core::ffi::c_int as isize)),
    );
    temp26 = ixheaac_sub32_sat(
        ixheaac_sub32_sat(
            ixheaac_sub32_sat(
                ixheaac_mult32x16in32_shl(
                    temp42,
                    *fft_c.offset(1 as core::ffi::c_int as isize),
                ),
                ixheaac_mult32x16in32_shl(
                    temp46,
                    *fft_c.offset(3 as core::ffi::c_int as isize),
                ),
            ),
            ixheaac_mult32x16in32_shl(
                temp410,
                *fft_c.offset(0 as core::ffi::c_int as isize),
            ),
        ),
        ixheaac_mult32x16in32_shl(temp414, *fft_c.offset(2 as core::ffi::c_int as isize)),
    );
    temp210 = ixheaac_add32_sat(
        ixheaac_add32_sat(
            ixheaac_sub32_sat(
                ixheaac_mult32x16in32_shl(
                    temp42,
                    *fft_c.offset(2 as core::ffi::c_int as isize),
                ),
                ixheaac_mult32x16in32_shl(
                    temp46,
                    *fft_c.offset(0 as core::ffi::c_int as isize),
                ),
            ),
            ixheaac_mult32x16in32_shl(
                temp410,
                *fft_c.offset(3 as core::ffi::c_int as isize),
            ),
        ),
        ixheaac_mult32x16in32_shl(temp414, *fft_c.offset(1 as core::ffi::c_int as isize)),
    );
    temp214 = ixheaac_sub32_sat(
        ixheaac_add32_sat(
            ixheaac_sub32_sat(
                ixheaac_mult32x16in32_shl(
                    temp42,
                    *fft_c.offset(3 as core::ffi::c_int as isize),
                ),
                ixheaac_mult32x16in32_shl(
                    temp46,
                    *fft_c.offset(2 as core::ffi::c_int as isize),
                ),
            ),
            ixheaac_mult32x16in32_shl(
                temp410,
                *fft_c.offset(1 as core::ffi::c_int as isize),
            ),
        ),
        ixheaac_mult32x16in32_shl(temp414, *fft_c.offset(0 as core::ffi::c_int as isize)),
    );
    temp23 = ixheaac_add32_sat(
        ixheaac_add32_sat(
            ixheaac_add32_sat(
                ixheaac_mult32x16in32_shl(
                    temp43,
                    *fft_c.offset(0 as core::ffi::c_int as isize),
                ),
                ixheaac_mult32x16in32_shl(
                    temp47,
                    *fft_c.offset(1 as core::ffi::c_int as isize),
                ),
            ),
            ixheaac_mult32x16in32_shl(
                temp411,
                *fft_c.offset(2 as core::ffi::c_int as isize),
            ),
        ),
        ixheaac_mult32x16in32_shl(temp415, *fft_c.offset(3 as core::ffi::c_int as isize)),
    );
    temp27 = ixheaac_sub32_sat(
        ixheaac_sub32_sat(
            ixheaac_sub32_sat(
                ixheaac_mult32x16in32_shl(
                    temp43,
                    *fft_c.offset(1 as core::ffi::c_int as isize),
                ),
                ixheaac_mult32x16in32_shl(
                    temp47,
                    *fft_c.offset(3 as core::ffi::c_int as isize),
                ),
            ),
            ixheaac_mult32x16in32_shl(
                temp411,
                *fft_c.offset(0 as core::ffi::c_int as isize),
            ),
        ),
        ixheaac_mult32x16in32_shl(temp415, *fft_c.offset(2 as core::ffi::c_int as isize)),
    );
    temp211 = ixheaac_add32_sat(
        ixheaac_add32_sat(
            ixheaac_sub32_sat(
                ixheaac_mult32x16in32_shl(
                    temp43,
                    *fft_c.offset(2 as core::ffi::c_int as isize),
                ),
                ixheaac_mult32x16in32_shl(
                    temp47,
                    *fft_c.offset(0 as core::ffi::c_int as isize),
                ),
            ),
            ixheaac_mult32x16in32_shl(
                temp411,
                *fft_c.offset(3 as core::ffi::c_int as isize),
            ),
        ),
        ixheaac_mult32x16in32_shl(temp415, *fft_c.offset(1 as core::ffi::c_int as isize)),
    );
    temp215 = ixheaac_sub32_sat(
        ixheaac_add32_sat(
            ixheaac_sub32_sat(
                ixheaac_mult32x16in32_shl(
                    temp43,
                    *fft_c.offset(3 as core::ffi::c_int as isize),
                ),
                ixheaac_mult32x16in32_shl(
                    temp47,
                    *fft_c.offset(2 as core::ffi::c_int as isize),
                ),
            ),
            ixheaac_mult32x16in32_shl(
                temp411,
                *fft_c.offset(1 as core::ffi::c_int as isize),
            ),
        ),
        ixheaac_mult32x16in32_shl(temp415, *fft_c.offset(0 as core::ffi::c_int as isize)),
    );
    temp40 = ixheaac_add32_sat(temp20, temp22);
    temp414 = ixheaac_sub32_sat(temp20, temp22);
    temp41 = ixheaac_add32_sat(temp21, temp23);
    temp415 = ixheaac_sub32_sat(temp21, temp23);
    temp42 = ixheaac_add32_sat(temp24, temp26);
    temp412 = ixheaac_sub32_sat(temp24, temp26);
    temp43 = ixheaac_add32_sat(temp25, temp27);
    temp413 = ixheaac_sub32_sat(temp25, temp27);
    temp44 = ixheaac_add32_sat(temp28, temp210);
    temp410 = ixheaac_sub32_sat(temp28, temp210);
    temp45 = ixheaac_add32_sat(temp29, temp211);
    temp411 = ixheaac_sub32_sat(temp29, temp211);
    temp46 = ixheaac_add32_sat(temp212, temp214);
    temp48 = ixheaac_sub32_sat(temp212, temp214);
    temp47 = ixheaac_add32_sat(temp213, temp215);
    temp49 = ixheaac_sub32_sat(temp213, temp215);
    temp10 = ixheaac_add32_sat(temp30, temp316);
    temp11 = ixheaac_add32_sat(temp31, temp317);
    temp12 = ixheaac_add32_sat(temp32, temp318);
    temp13 = ixheaac_add32_sat(temp33, temp319);
    temp14 = ixheaac_add32_sat(temp34, temp320);
    temp15 = ixheaac_add32_sat(temp35, temp321);
    temp16 = ixheaac_add32_sat(temp36, temp322);
    temp17 = ixheaac_add32_sat(temp37, temp323);
    temp18 = ixheaac_add32_sat(temp38, temp324);
    temp19 = ixheaac_add32_sat(temp39, temp325);
    temp110 = ixheaac_add32_sat(temp310, temp326);
    temp111 = ixheaac_add32_sat(temp311, temp327);
    temp112 = ixheaac_add32_sat(temp312, temp328);
    temp113 = ixheaac_add32_sat(temp313, temp329);
    temp114 = ixheaac_add32_sat(temp314, temp330);
    temp115 = ixheaac_add32_sat(temp315, temp331);
    tmp0 = ixheaac_add32_sat(temp10, temp18);
    tmp2 = ixheaac_sub32_sat(temp10, temp18);
    tmp1 = ixheaac_add32_sat(temp11, temp19);
    tmp3 = ixheaac_sub32_sat(temp11, temp19);
    tmp4 = ixheaac_add32_sat(temp12, temp110);
    tmp6 = ixheaac_sub32_sat(temp12, temp110);
    tmp5 = ixheaac_add32_sat(temp13, temp111);
    tmp7 = ixheaac_sub32_sat(temp13, temp111);
    tmp8 = ixheaac_add32_sat(temp14, temp112);
    tmp10 = ixheaac_sub32_sat(temp14, temp112);
    tmp9 = ixheaac_add32_sat(temp15, temp113);
    tmp11 = ixheaac_sub32_sat(temp15, temp113);
    tmp12 = ixheaac_add32_sat(temp16, temp114);
    tmp14 = ixheaac_sub32_sat(temp16, temp114);
    tmp13 = ixheaac_add32_sat(temp17, temp115);
    tmp15 = ixheaac_sub32_sat(temp17, temp115);
    temp20 = ixheaac_add32_sat(tmp0, tmp8);
    temp24 = ixheaac_sub32_sat(tmp0, tmp8);
    temp21 = ixheaac_add32_sat(tmp1, tmp9);
    temp25 = ixheaac_sub32_sat(tmp1, tmp9);
    temp28 = ixheaac_sub32_sat(tmp2, tmp11);
    temp210 = ixheaac_add32_sat(tmp2, tmp11);
    temp29 = ixheaac_add32_sat(tmp3, tmp10);
    temp211 = ixheaac_sub32_sat(tmp3, tmp10);
    temp22 = ixheaac_add32_sat(tmp4, tmp12);
    temp27 = ixheaac_sub32_sat(tmp4, tmp12);
    temp23 = ixheaac_add32_sat(tmp5, tmp13);
    temp26 = ixheaac_sub32_sat(tmp13, tmp5);
    tmp1 = ixheaac_add32_sat(tmp6, tmp14);
    tmp2 = ixheaac_sub32_sat(tmp6, tmp14);
    tmp0 = ixheaac_add32_sat(tmp7, tmp15);
    tmp3 = ixheaac_sub32_sat(tmp7, tmp15);
    temp212 = ixheaac_mult32x16in32_shl(
        ixheaac_add32_sat(tmp0, tmp2),
        INV_SQRT2_Q15 as WORD16,
    );
    temp214 = ixheaac_mult32x16in32_shl(
        ixheaac_sub32_sat(tmp0, tmp2),
        INV_SQRT2_Q15 as WORD16,
    );
    temp213 = ixheaac_mult32x16in32_shl(
        ixheaac_sub32_sat(tmp3, tmp1),
        INV_SQRT2_Q15 as WORD16,
    );
    temp215 = ixheaac_mult32x16in32_shl(
        ixheaac_add32_sat(tmp1, tmp3),
        -INV_SQRT2_Q15 as WORD16,
    );
    temp10 = ixheaac_sub32_sat(temp30, temp316);
    temp11 = ixheaac_sub32_sat(temp31, temp317);
    temp12 = ixheaac_sub32_sat(temp32, temp318);
    temp13 = ixheaac_sub32_sat(temp33, temp319);
    temp14 = ixheaac_sub32_sat(temp34, temp320);
    temp15 = ixheaac_sub32_sat(temp35, temp321);
    temp16 = ixheaac_sub32_sat(temp36, temp322);
    temp17 = ixheaac_sub32_sat(temp37, temp323);
    temp18 = ixheaac_sub32_sat(temp38, temp324);
    temp19 = ixheaac_sub32_sat(temp39, temp325);
    temp110 = ixheaac_sub32_sat(temp310, temp326);
    temp111 = ixheaac_sub32_sat(temp311, temp327);
    temp112 = ixheaac_sub32_sat(temp312, temp328);
    temp113 = ixheaac_sub32_sat(temp313, temp329);
    temp114 = ixheaac_sub32_sat(temp314, temp330);
    temp115 = ixheaac_sub32_sat(temp315, temp331);
    temp30 = ixheaac_add32_sat(temp20, temp22);
    temp316 = ixheaac_sub32_sat(temp20, temp22);
    temp31 = ixheaac_add32_sat(temp21, temp23);
    temp317 = ixheaac_sub32_sat(temp21, temp23);
    temp38 = ixheaac_sub32_sat(temp24, temp26);
    temp324 = ixheaac_add32_sat(temp24, temp26);
    temp39 = ixheaac_sub32_sat(temp25, temp27);
    temp325 = ixheaac_add32_sat(temp25, temp27);
    temp312 = ixheaac_add32_sat(temp28, temp214);
    temp328 = ixheaac_sub32_sat(temp28, temp214);
    temp313 = ixheaac_add32_sat(temp29, temp215);
    temp329 = ixheaac_sub32_sat(temp29, temp215);
    temp34 = ixheaac_add32_sat(temp210, temp212);
    temp320 = ixheaac_sub32_sat(temp210, temp212);
    temp35 = ixheaac_add32_sat(temp211, temp213);
    temp321 = ixheaac_sub32_sat(temp211, temp213);
    tmp9 = ixheaac_mult32x16in32_shl(
        ixheaac_add32_sat(temp12, temp114),
        -COS_3PI_BY_8_Q15 as WORD16,
    );
    tmp10 = ixheaac_mult32x16in32_shl(
        ixheaac_sub32_sat(temp12, temp114),
        COS_PI_BY_8_Q15 as WORD16,
    );
    tmp8 = ixheaac_mult32x16in32_shl(
        ixheaac_add32_sat(temp13, temp115),
        COS_3PI_BY_8_Q15 as WORD16,
    );
    tmp11 = ixheaac_mult32x16in32_shl(
        ixheaac_sub32_sat(temp13, temp115),
        COS_PI_BY_8_Q15 as WORD16,
    );
    tmp5 = ixheaac_mult32x16in32_shl(
        ixheaac_add32_sat(temp14, temp112),
        -INV_SQRT2_Q15 as WORD16,
    );
    tmp6 = ixheaac_mult32x16in32_shl(
        ixheaac_sub32_sat(temp14, temp112),
        INV_SQRT2_Q15 as WORD16,
    );
    tmp4 = ixheaac_mult32x16in32_shl(
        ixheaac_add32_sat(temp15, temp113),
        INV_SQRT2_Q15 as WORD16,
    );
    tmp7 = ixheaac_mult32x16in32_shl(
        ixheaac_sub32_sat(temp15, temp113),
        INV_SQRT2_Q15 as WORD16,
    );
    tmp13 = ixheaac_mult32x16in32_shl(
        ixheaac_add32_sat(temp16, temp110),
        -COS_PI_BY_8_Q15 as WORD16,
    );
    tmp14 = ixheaac_mult32x16in32_shl(
        ixheaac_sub32_sat(temp16, temp110),
        COS_3PI_BY_8_Q15 as WORD16,
    );
    tmp12 = ixheaac_mult32x16in32_shl(
        ixheaac_add32_sat(temp17, temp111),
        COS_PI_BY_8_Q15 as WORD16,
    );
    tmp15 = ixheaac_mult32x16in32_shl(
        ixheaac_sub32_sat(temp17, temp111),
        COS_3PI_BY_8_Q15 as WORD16,
    );
    temp12 = ixheaac_sub32_sat(
        ixheaac_shl32(ixheaac_mult32x16in32(tmp8, SQRT2PLUS1_Q13 as WORD16), 3 as WORD),
        ixheaac_mult32x16in32_shl(tmp12, SQRT2MINUS1_Q15 as WORD16),
    );
    temp13 = ixheaac_sub32_sat(
        ixheaac_shl32(ixheaac_mult32x16in32(tmp9, SQRT2PLUS1_Q13 as WORD16), 3 as WORD),
        ixheaac_mult32x16in32_shl(tmp13, SQRT2MINUS1_Q15 as WORD16),
    );
    temp14 = ixheaac_sub32_sat(
        ixheaac_mult32x16in32_shl(tmp10, SQRT2MINUS1_Q15 as WORD16),
        ixheaac_shl32(ixheaac_mult32x16in32(tmp14, SQRT2PLUS1_Q13 as WORD16), 3 as WORD),
    );
    temp15 = ixheaac_sub32_sat(
        ixheaac_mult32x16in32_shl(tmp11, SQRT2MINUS1_Q15 as WORD16),
        ixheaac_shl32(ixheaac_mult32x16in32(tmp15, SQRT2PLUS1_Q13 as WORD16), 3 as WORD),
    );
    tmp8 = ixheaac_add32_sat(tmp8, tmp12);
    tmp9 = ixheaac_add32_sat(tmp9, tmp13);
    tmp10 = ixheaac_add32_sat(tmp10, tmp14);
    tmp11 = ixheaac_add32_sat(tmp11, tmp15);
    temp16 = ixheaac_add32_sat(temp10, tmp4);
    temp110 = ixheaac_sub32_sat(temp10, tmp4);
    temp17 = ixheaac_add32_sat(temp11, tmp5);
    temp111 = ixheaac_sub32_sat(temp11, tmp5);
    temp112 = ixheaac_sub32_sat(tmp6, temp19);
    temp114 = ixheaac_add32_sat(tmp6, temp19);
    temp113 = ixheaac_add32_sat(temp18, tmp7);
    temp115 = ixheaac_sub32_sat(temp18, tmp7);
    tmp0 = ixheaac_sub32_sat(temp16, temp114);
    tmp2 = ixheaac_add32_sat(temp16, temp114);
    tmp1 = ixheaac_add32_sat(temp17, temp115);
    tmp3 = ixheaac_sub32_sat(temp17, temp115);
    tmp4 = ixheaac_add32_sat(temp110, temp112);
    tmp6 = ixheaac_sub32_sat(temp110, temp112);
    tmp5 = ixheaac_add32_sat(temp111, temp113);
    tmp7 = ixheaac_sub32_sat(temp111, temp113);
    temp110 = ixheaac_add32_sat(tmp8, tmp10);
    tmp10 = ixheaac_sub32_sat(tmp8, tmp10);
    temp111 = ixheaac_add32_sat(tmp9, tmp11);
    tmp11 = ixheaac_sub32_sat(tmp9, tmp11);
    tmp12 = ixheaac_add32_sat(temp12, temp14);
    tmp14 = ixheaac_sub32_sat(temp12, temp14);
    tmp13 = ixheaac_add32_sat(temp13, temp15);
    tmp15 = ixheaac_sub32_sat(temp13, temp15);
    temp32 = ixheaac_add32_sat(tmp2, temp110);
    temp318 = ixheaac_sub32_sat(tmp2, temp110);
    temp33 = ixheaac_add32_sat(tmp3, temp111);
    temp319 = ixheaac_sub32_sat(tmp3, temp111);
    temp36 = ixheaac_add32_sat(tmp0, tmp12);
    temp322 = ixheaac_sub32_sat(tmp0, tmp12);
    temp37 = ixheaac_add32_sat(tmp1, tmp13);
    temp323 = ixheaac_sub32_sat(tmp1, tmp13);
    temp314 = ixheaac_add32_sat(tmp4, tmp10);
    temp330 = ixheaac_sub32_sat(tmp4, tmp10);
    temp315 = ixheaac_add32_sat(tmp5, tmp11);
    temp331 = ixheaac_sub32_sat(tmp5, tmp11);
    temp310 = ixheaac_add32_sat(tmp6, tmp14);
    temp326 = ixheaac_sub32_sat(tmp6, tmp14);
    temp311 = ixheaac_add32_sat(tmp7, tmp15);
    temp327 = ixheaac_sub32_sat(tmp7, tmp15);
    temp10 = ixheaac_sub32_sat(
        *vec.offset(0 as core::ffi::c_int as isize),
        *vec.offset(32 as core::ffi::c_int as isize),
    );
    temp11 = ixheaac_sub32_sat(
        *vec.offset(1 as core::ffi::c_int as isize),
        *vec.offset(33 as core::ffi::c_int as isize),
    );
    temp12 = ixheaac_sub32_sat(
        *vec.offset(4 as core::ffi::c_int as isize),
        *vec.offset(36 as core::ffi::c_int as isize),
    );
    temp13 = ixheaac_sub32_sat(
        *vec.offset(5 as core::ffi::c_int as isize),
        *vec.offset(37 as core::ffi::c_int as isize),
    );
    temp14 = ixheaac_sub32_sat(
        *vec.offset(8 as core::ffi::c_int as isize),
        *vec.offset(40 as core::ffi::c_int as isize),
    );
    temp15 = ixheaac_sub32_sat(
        *vec.offset(9 as core::ffi::c_int as isize),
        *vec.offset(41 as core::ffi::c_int as isize),
    );
    temp16 = ixheaac_sub32_sat(
        *vec.offset(12 as core::ffi::c_int as isize),
        *vec.offset(44 as core::ffi::c_int as isize),
    );
    temp17 = ixheaac_sub32_sat(
        *vec.offset(13 as core::ffi::c_int as isize),
        *vec.offset(45 as core::ffi::c_int as isize),
    );
    temp18 = ixheaac_sub32_sat(
        *vec.offset(16 as core::ffi::c_int as isize),
        *vec.offset(48 as core::ffi::c_int as isize),
    );
    temp19 = ixheaac_sub32_sat(
        *vec.offset(17 as core::ffi::c_int as isize),
        *vec.offset(49 as core::ffi::c_int as isize),
    );
    temp110 = ixheaac_sub32_sat(
        *vec.offset(20 as core::ffi::c_int as isize),
        *vec.offset(52 as core::ffi::c_int as isize),
    );
    temp111 = ixheaac_sub32_sat(
        *vec.offset(21 as core::ffi::c_int as isize),
        *vec.offset(53 as core::ffi::c_int as isize),
    );
    temp112 = ixheaac_sub32_sat(
        *vec.offset(24 as core::ffi::c_int as isize),
        *vec.offset(56 as core::ffi::c_int as isize),
    );
    temp113 = ixheaac_sub32_sat(
        *vec.offset(25 as core::ffi::c_int as isize),
        *vec.offset(57 as core::ffi::c_int as isize),
    );
    temp114 = ixheaac_sub32_sat(
        *vec.offset(28 as core::ffi::c_int as isize),
        *vec.offset(60 as core::ffi::c_int as isize),
    );
    temp115 = ixheaac_sub32_sat(
        *vec.offset(29 as core::ffi::c_int as isize),
        *vec.offset(61 as core::ffi::c_int as isize),
    );
    tmp9 = ixheaac_mult32x16in32_shl(
        ixheaac_add32_sat(temp12, temp114),
        -COS_3PI_BY_8_Q15 as WORD16,
    );
    tmp10 = ixheaac_mult32x16in32_shl(
        ixheaac_sub32_sat(temp12, temp114),
        COS_PI_BY_8_Q15 as WORD16,
    );
    tmp8 = ixheaac_mult32x16in32_shl(
        ixheaac_add32_sat(temp13, temp115),
        COS_3PI_BY_8_Q15 as WORD16,
    );
    tmp11 = ixheaac_mult32x16in32_shl(
        ixheaac_sub32_sat(temp13, temp115),
        COS_PI_BY_8_Q15 as WORD16,
    );
    tmp5 = ixheaac_mult32x16in32_shl(
        ixheaac_add32_sat(temp14, temp112),
        -INV_SQRT2_Q15 as WORD16,
    );
    tmp6 = ixheaac_mult32x16in32_shl(
        ixheaac_sub32_sat(temp14, temp112),
        INV_SQRT2_Q15 as WORD16,
    );
    tmp4 = ixheaac_mult32x16in32_shl(
        ixheaac_add32_sat(temp15, temp113),
        INV_SQRT2_Q15 as WORD16,
    );
    tmp7 = ixheaac_mult32x16in32_shl(
        ixheaac_sub32_sat(temp15, temp113),
        INV_SQRT2_Q15 as WORD16,
    );
    tmp13 = ixheaac_mult32x16in32_shl(
        ixheaac_add32_sat(temp16, temp110),
        -COS_PI_BY_8_Q15 as WORD16,
    );
    tmp14 = ixheaac_mult32x16in32_shl(
        ixheaac_sub32_sat(temp16, temp110),
        COS_3PI_BY_8_Q15 as WORD16,
    );
    tmp12 = ixheaac_mult32x16in32_shl(
        ixheaac_add32_sat(temp17, temp111),
        COS_PI_BY_8_Q15 as WORD16,
    );
    tmp15 = ixheaac_mult32x16in32_shl(
        ixheaac_sub32_sat(temp17, temp111),
        COS_3PI_BY_8_Q15 as WORD16,
    );
    temp12 = ixheaac_sub32_sat(
        ixheaac_shl32(ixheaac_mult32x16in32(tmp8, SQRT2PLUS1_Q13 as WORD16), 3 as WORD),
        ixheaac_mult32x16in32_shl(tmp12, SQRT2MINUS1_Q15 as WORD16),
    );
    temp13 = ixheaac_sub32_sat(
        ixheaac_shl32(ixheaac_mult32x16in32(tmp9, SQRT2PLUS1_Q13 as WORD16), 3 as WORD),
        ixheaac_mult32x16in32_shl(tmp13, SQRT2MINUS1_Q15 as WORD16),
    );
    temp14 = ixheaac_sub32_sat(
        ixheaac_mult32x16in32_shl(tmp10, SQRT2MINUS1_Q15 as WORD16),
        ixheaac_shl32(ixheaac_mult32x16in32(tmp14, SQRT2PLUS1_Q13 as WORD16), 3 as WORD),
    );
    temp15 = ixheaac_sub32_sat(
        ixheaac_mult32x16in32_shl(tmp11, SQRT2MINUS1_Q15 as WORD16),
        ixheaac_shl32(ixheaac_mult32x16in32(tmp15, SQRT2PLUS1_Q13 as WORD16), 3 as WORD),
    );
    tmp8 = ixheaac_add32_sat(tmp8, tmp12);
    tmp9 = ixheaac_add32_sat(tmp9, tmp13);
    tmp10 = ixheaac_add32_sat(tmp10, tmp14);
    tmp11 = ixheaac_add32_sat(tmp11, tmp15);
    temp16 = ixheaac_add32_sat(temp10, tmp4);
    temp110 = ixheaac_sub32_sat(temp10, tmp4);
    temp17 = ixheaac_add32_sat(temp11, tmp5);
    temp111 = ixheaac_sub32_sat(temp11, tmp5);
    temp112 = ixheaac_sub32_sat(tmp6, temp19);
    temp114 = ixheaac_add32_sat(tmp6, temp19);
    temp113 = ixheaac_add32_sat(temp18, tmp7);
    temp115 = ixheaac_sub32_sat(temp18, tmp7);
    tmp0 = ixheaac_sub32_sat(temp16, temp114);
    tmp2 = ixheaac_add32_sat(temp16, temp114);
    tmp1 = ixheaac_add32_sat(temp17, temp115);
    tmp3 = ixheaac_sub32_sat(temp17, temp115);
    tmp4 = ixheaac_add32_sat(temp110, temp112);
    tmp6 = ixheaac_sub32_sat(temp110, temp112);
    tmp5 = ixheaac_add32_sat(temp111, temp113);
    tmp7 = ixheaac_sub32_sat(temp111, temp113);
    temp110 = ixheaac_add32_sat(tmp8, tmp10);
    tmp10 = ixheaac_sub32_sat(tmp8, tmp10);
    temp111 = ixheaac_add32_sat(tmp9, tmp11);
    tmp11 = ixheaac_sub32_sat(tmp9, tmp11);
    tmp12 = ixheaac_add32_sat(temp12, temp14);
    tmp14 = ixheaac_sub32_sat(temp12, temp14);
    tmp13 = ixheaac_add32_sat(temp13, temp15);
    tmp15 = ixheaac_sub32_sat(temp13, temp15);
    temp10 = ixheaac_add32_sat(tmp2, temp110);
    temp18 = ixheaac_sub32_sat(tmp2, temp110);
    temp11 = ixheaac_add32_sat(tmp3, temp111);
    temp19 = ixheaac_sub32_sat(tmp3, temp111);
    temp12 = ixheaac_add32_sat(tmp0, tmp12);
    temp110 = ixheaac_sub32_sat(tmp0, tmp12);
    temp13 = ixheaac_add32_sat(tmp1, tmp13);
    temp111 = ixheaac_sub32_sat(tmp1, tmp13);
    temp16 = ixheaac_add32_sat(tmp4, tmp10);
    temp114 = ixheaac_sub32_sat(tmp4, tmp10);
    temp17 = ixheaac_add32_sat(tmp5, tmp11);
    temp115 = ixheaac_sub32_sat(tmp5, tmp11);
    temp14 = ixheaac_add32_sat(tmp6, tmp14);
    temp112 = ixheaac_sub32_sat(tmp6, tmp14);
    temp15 = ixheaac_add32_sat(tmp7, tmp15);
    temp113 = ixheaac_sub32_sat(tmp7, tmp15);
    let fresh10 = vec;
    vec = vec.offset(1);
    *fresh10 = temp30;
    let fresh11 = vec;
    vec = vec.offset(1);
    *fresh11 = temp31;
    let fresh12 = vec;
    vec = vec.offset(1);
    *fresh12 = ixheaac_add32_sat(temp10, temp40);
    let fresh13 = vec;
    vec = vec.offset(1);
    *fresh13 = ixheaac_add32_sat(temp11, temp41);
    let fresh14 = vec;
    vec = vec.offset(1);
    *fresh14 = temp32;
    let fresh15 = vec;
    vec = vec.offset(1);
    *fresh15 = temp33;
    let fresh16 = vec;
    vec = vec.offset(1);
    *fresh16 = ixheaac_add32_sat(temp12, temp42);
    let fresh17 = vec;
    vec = vec.offset(1);
    *fresh17 = ixheaac_add32_sat(temp13, temp43);
    let fresh18 = vec;
    vec = vec.offset(1);
    *fresh18 = temp34;
    let fresh19 = vec;
    vec = vec.offset(1);
    *fresh19 = temp35;
    let fresh20 = vec;
    vec = vec.offset(1);
    *fresh20 = ixheaac_add32_sat(temp14, temp44);
    let fresh21 = vec;
    vec = vec.offset(1);
    *fresh21 = ixheaac_add32_sat(temp15, temp45);
    let fresh22 = vec;
    vec = vec.offset(1);
    *fresh22 = temp36;
    let fresh23 = vec;
    vec = vec.offset(1);
    *fresh23 = temp37;
    let fresh24 = vec;
    vec = vec.offset(1);
    *fresh24 = ixheaac_add32_sat(temp16, temp46);
    let fresh25 = vec;
    vec = vec.offset(1);
    *fresh25 = ixheaac_add32_sat(temp17, temp47);
    let fresh26 = vec;
    vec = vec.offset(1);
    *fresh26 = temp38;
    let fresh27 = vec;
    vec = vec.offset(1);
    *fresh27 = temp39;
    let fresh28 = vec;
    vec = vec.offset(1);
    *fresh28 = ixheaac_add32_sat(temp18, temp48);
    let fresh29 = vec;
    vec = vec.offset(1);
    *fresh29 = ixheaac_add32_sat(temp19, temp49);
    let fresh30 = vec;
    vec = vec.offset(1);
    *fresh30 = temp310;
    let fresh31 = vec;
    vec = vec.offset(1);
    *fresh31 = temp311;
    let fresh32 = vec;
    vec = vec.offset(1);
    *fresh32 = ixheaac_add32_sat(temp110, temp410);
    let fresh33 = vec;
    vec = vec.offset(1);
    *fresh33 = ixheaac_add32_sat(temp111, temp411);
    let fresh34 = vec;
    vec = vec.offset(1);
    *fresh34 = temp312;
    let fresh35 = vec;
    vec = vec.offset(1);
    *fresh35 = temp313;
    let fresh36 = vec;
    vec = vec.offset(1);
    *fresh36 = ixheaac_add32_sat(temp112, temp412);
    let fresh37 = vec;
    vec = vec.offset(1);
    *fresh37 = ixheaac_add32_sat(temp113, temp413);
    let fresh38 = vec;
    vec = vec.offset(1);
    *fresh38 = temp314;
    let fresh39 = vec;
    vec = vec.offset(1);
    *fresh39 = temp315;
    let fresh40 = vec;
    vec = vec.offset(1);
    *fresh40 = ixheaac_add32_sat(temp114, temp414);
    let fresh41 = vec;
    vec = vec.offset(1);
    *fresh41 = ixheaac_add32_sat(temp115, temp415);
    let fresh42 = vec;
    vec = vec.offset(1);
    *fresh42 = temp316;
    let fresh43 = vec;
    vec = vec.offset(1);
    *fresh43 = temp317;
    let fresh44 = vec;
    vec = vec.offset(1);
    *fresh44 = ixheaac_sub32_sat(temp10, temp40);
    let fresh45 = vec;
    vec = vec.offset(1);
    *fresh45 = ixheaac_sub32_sat(temp11, temp41);
    let fresh46 = vec;
    vec = vec.offset(1);
    *fresh46 = temp318;
    let fresh47 = vec;
    vec = vec.offset(1);
    *fresh47 = temp319;
    let fresh48 = vec;
    vec = vec.offset(1);
    *fresh48 = ixheaac_sub32_sat(temp12, temp42);
    let fresh49 = vec;
    vec = vec.offset(1);
    *fresh49 = ixheaac_sub32_sat(temp13, temp43);
    let fresh50 = vec;
    vec = vec.offset(1);
    *fresh50 = temp320;
    let fresh51 = vec;
    vec = vec.offset(1);
    *fresh51 = temp321;
    let fresh52 = vec;
    vec = vec.offset(1);
    *fresh52 = ixheaac_sub32_sat(temp14, temp44);
    let fresh53 = vec;
    vec = vec.offset(1);
    *fresh53 = ixheaac_sub32_sat(temp15, temp45);
    let fresh54 = vec;
    vec = vec.offset(1);
    *fresh54 = temp322;
    let fresh55 = vec;
    vec = vec.offset(1);
    *fresh55 = temp323;
    let fresh56 = vec;
    vec = vec.offset(1);
    *fresh56 = ixheaac_sub32_sat(temp16, temp46);
    let fresh57 = vec;
    vec = vec.offset(1);
    *fresh57 = ixheaac_sub32_sat(temp17, temp47);
    let fresh58 = vec;
    vec = vec.offset(1);
    *fresh58 = temp324;
    let fresh59 = vec;
    vec = vec.offset(1);
    *fresh59 = temp325;
    let fresh60 = vec;
    vec = vec.offset(1);
    *fresh60 = ixheaac_sub32_sat(temp18, temp48);
    let fresh61 = vec;
    vec = vec.offset(1);
    *fresh61 = ixheaac_sub32_sat(temp19, temp49);
    let fresh62 = vec;
    vec = vec.offset(1);
    *fresh62 = temp326;
    let fresh63 = vec;
    vec = vec.offset(1);
    *fresh63 = temp327;
    let fresh64 = vec;
    vec = vec.offset(1);
    *fresh64 = ixheaac_sub32_sat(temp110, temp410);
    let fresh65 = vec;
    vec = vec.offset(1);
    *fresh65 = ixheaac_sub32_sat(temp111, temp411);
    let fresh66 = vec;
    vec = vec.offset(1);
    *fresh66 = temp328;
    let fresh67 = vec;
    vec = vec.offset(1);
    *fresh67 = temp329;
    let fresh68 = vec;
    vec = vec.offset(1);
    *fresh68 = ixheaac_sub32_sat(temp112, temp412);
    let fresh69 = vec;
    vec = vec.offset(1);
    *fresh69 = ixheaac_sub32_sat(temp113, temp413);
    let fresh70 = vec;
    vec = vec.offset(1);
    *fresh70 = temp330;
    let fresh71 = vec;
    vec = vec.offset(1);
    *fresh71 = temp331;
    let fresh72 = vec;
    vec = vec.offset(1);
    *fresh72 = ixheaac_sub32_sat(temp114, temp414);
    let fresh73 = vec;
    vec = vec.offset(1);
    *fresh73 = ixheaac_sub32_sat(temp115, temp415);
}
unsafe extern "C" fn ixheaacd_cos_mod(
    mut subband: *mut WORD32,
    mut qmf_table_ptr: *mut ia_mps_dec_qmf_tables_struct,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut m: WORD32 = 0;
    let mut wim: WORD16 = 0;
    let mut wre: WORD16 = 0;
    let mut temp_1: WORD32 = 0;
    let mut ptr1: *const WORD16 = 0 as *const WORD16;
    let mut ptr2: *const WORD16 = 0 as *const WORD16;
    let mut ptr3: *const WORD16 = 0 as *const WORD16;
    let mut ptr4: *const WORD16 = 0 as *const WORD16;
    let mut re1: WORD32 = 0;
    let mut im1: WORD32 = 0;
    let mut re2: WORD32 = 0;
    let mut im2: WORD32 = 0;
    m = WORD_LENGTH as WORD32;
    ptr1 = ((*qmf_table_ptr).sbr_sin_twiddle).as_mut_ptr();
    ptr2 = ((*qmf_table_ptr).sbr_cos_twiddle).as_mut_ptr();
    ptr3 = ((*qmf_table_ptr).sbr_sin_twiddle)
        .as_mut_ptr()
        .offset(31 as core::ffi::c_int as isize);
    ptr4 = ((*qmf_table_ptr).sbr_cos_twiddle)
        .as_mut_ptr()
        .offset(31 as core::ffi::c_int as isize);
    i = 0 as core::ffi::c_int as WORD32;
    while i < 16 as core::ffi::c_int {
        temp_1 = i << 1 as core::ffi::c_int;
        re1 = *subband.offset(temp_1 as isize);
        im2 = *subband
            .offset((temp_1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
        re2 = *subband.offset((62 as WORD32 - temp_1) as isize);
        im1 = *subband.offset((63 as WORD32 - temp_1) as isize);
        let fresh74 = ptr1;
        ptr1 = ptr1.offset(1);
        wim = *fresh74;
        let fresh75 = ptr2;
        ptr2 = ptr2.offset(1);
        wre = *fresh75;
        *subband.offset(temp_1 as isize) = ixheaac_add32_sat(
            ixheaac_mult32x16in32_shl(im1, wim),
            ixheaac_mult32x16in32_shl(re1, wre),
        );
        *subband.offset((temp_1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = ixheaac_sub32_sat(
            ixheaac_mult32x16in32_shl(im1, wre),
            ixheaac_mult32x16in32_shl(re1, wim),
        );
        let fresh76 = ptr3;
        ptr3 = ptr3.offset(-1);
        wim = *fresh76;
        let fresh77 = ptr4;
        ptr4 = ptr4.offset(-1);
        wre = *fresh77;
        *subband.offset((62 as WORD32 - temp_1) as isize) = ixheaac_add32_sat(
            ixheaac_mult32x16in32_shl(im2, wim),
            ixheaac_mult32x16in32_shl(re2, wre),
        );
        *subband.offset((63 as WORD32 - temp_1) as isize) = ixheaac_sub32_sat(
            ixheaac_mult32x16in32_shl(im2, wre),
            ixheaac_mult32x16in32_shl(re2, wim),
        );
        i += 1;
    }
    ixheaacd_fft32(subband, ((*qmf_table_ptr).fft_c).as_mut_ptr());
    ptr1 = ((*qmf_table_ptr).sbr_alt_sin_twiddle).as_mut_ptr();
    ptr2 = ((*qmf_table_ptr).sbr_alt_sin_twiddle).as_mut_ptr().offset(m as isize);
    let fresh78 = ptr1;
    ptr1 = ptr1.offset(1);
    wim = *fresh78;
    let fresh79 = ptr2;
    ptr2 = ptr2.offset(-1);
    wre = *fresh79;
    i = 0 as core::ffi::c_int as WORD32;
    while i < 16 as core::ffi::c_int {
        temp_1 = i << 1 as core::ffi::c_int;
        re1 = *subband.offset(temp_1 as isize);
        im1 = *subband
            .offset((temp_1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
        re2 = *subband.offset((62 as WORD32 - temp_1) as isize);
        im2 = *subband.offset((63 as WORD32 - temp_1) as isize);
        *subband.offset(temp_1 as isize) = ixheaac_add32_sat(
            ixheaac_mult32x16in32_shl(re1, wre),
            ixheaac_mult32x16in32_shl(im1, wim),
        );
        *subband.offset((63 as WORD32 - temp_1) as isize) = ixheaac_sub32_sat(
            ixheaac_mult32x16in32_shl(re1, wim),
            ixheaac_mult32x16in32_shl(im1, wre),
        );
        let fresh80 = ptr1;
        ptr1 = ptr1.offset(1);
        wim = *fresh80;
        let fresh81 = ptr2;
        ptr2 = ptr2.offset(-1);
        wre = *fresh81;
        *subband.offset((62 as WORD32 - temp_1) as isize) = ixheaac_add32_sat(
            ixheaac_mult32x16in32_shl(re2, wim),
            ixheaac_mult32x16in32_shl(im2, wre),
        );
        *subband.offset((temp_1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = ixheaac_sub32_sat(
            ixheaac_mult32x16in32_shl(re2, wre),
            ixheaac_mult32x16in32_shl(im2, wim),
        );
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_sin_mod(
    mut subband: *mut WORD32,
    mut qmf_table_ptr: *mut ia_mps_dec_qmf_tables_struct,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut m: WORD32 = 0;
    let mut wre: WORD16 = 0;
    let mut wim: WORD16 = 0;
    let mut temp_1: WORD32 = 0;
    let mut re1: WORD32 = 0;
    let mut im1: WORD32 = 0;
    let mut re2: WORD32 = 0;
    let mut im2: WORD32 = 0;
    let mut ptr1: *const WORD16 = 0 as *const WORD16;
    let mut ptr2: *const WORD16 = 0 as *const WORD16;
    let mut ptr3: *const WORD16 = 0 as *const WORD16;
    let mut ptr4: *const WORD16 = 0 as *const WORD16;
    ptr1 = ((*qmf_table_ptr).sbr_sin_twiddle).as_mut_ptr();
    ptr2 = ((*qmf_table_ptr).sbr_cos_twiddle).as_mut_ptr();
    ptr3 = ((*qmf_table_ptr).sbr_sin_twiddle)
        .as_mut_ptr()
        .offset(31 as core::ffi::c_int as isize);
    ptr4 = ((*qmf_table_ptr).sbr_cos_twiddle)
        .as_mut_ptr()
        .offset(31 as core::ffi::c_int as isize);
    m = WORD_LENGTH as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < 16 as core::ffi::c_int {
        temp_1 = i << 1 as core::ffi::c_int;
        re1 = *subband.offset(temp_1 as isize);
        im2 = *subband
            .offset((temp_1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
        re2 = *subband.offset((62 as WORD32 - temp_1) as isize);
        im1 = *subband.offset((63 as WORD32 - temp_1) as isize);
        let fresh2 = ptr1;
        ptr1 = ptr1.offset(1);
        wre = *fresh2;
        let fresh3 = ptr2;
        ptr2 = ptr2.offset(1);
        wim = *fresh3;
        *subband.offset((temp_1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = ixheaac_add32_sat(
            ixheaac_mult32x16in32_shl(im1, wim),
            ixheaac_mult32x16in32_shl(re1, wre),
        );
        *subband.offset(temp_1 as isize) = ixheaac_sub32_sat(
            ixheaac_mult32x16in32_shl(im1, wre),
            ixheaac_mult32x16in32_shl(re1, wim),
        );
        let fresh4 = ptr3;
        ptr3 = ptr3.offset(-1);
        wre = *fresh4;
        let fresh5 = ptr4;
        ptr4 = ptr4.offset(-1);
        wim = *fresh5;
        *subband.offset((63 as WORD32 - temp_1) as isize) = ixheaac_add32_sat(
            ixheaac_mult32x16in32_shl(im2, wim),
            ixheaac_mult32x16in32_shl(re2, wre),
        );
        *subband.offset((62 as WORD32 - temp_1) as isize) = ixheaac_sub32_sat(
            ixheaac_mult32x16in32_shl(im2, wre),
            ixheaac_mult32x16in32_shl(re2, wim),
        );
        i += 1;
    }
    ixheaacd_fft32(subband, ((*qmf_table_ptr).fft_c).as_mut_ptr());
    ptr1 = ((*qmf_table_ptr).sbr_alt_sin_twiddle).as_mut_ptr();
    ptr2 = ((*qmf_table_ptr).sbr_alt_sin_twiddle).as_mut_ptr().offset(m as isize);
    let fresh6 = ptr1;
    ptr1 = ptr1.offset(1);
    wim = *fresh6;
    let fresh7 = ptr2;
    ptr2 = ptr2.offset(-1);
    wre = *fresh7;
    i = 0 as core::ffi::c_int as WORD32;
    while i < 16 as core::ffi::c_int {
        temp_1 = i << 1 as core::ffi::c_int;
        re1 = *subband.offset(temp_1 as isize);
        im1 = *subband
            .offset((temp_1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
        re2 = *subband.offset((62 as WORD32 - temp_1) as isize);
        im2 = *subband.offset((63 as WORD32 - temp_1) as isize);
        *subband.offset((63 as WORD32 - temp_1) as isize) = ixheaac_negate32_sat(
            ixheaac_add32_sat(
                ixheaac_mult32x16in32_shl(re1, wre),
                ixheaac_mult32x16in32_shl(im1, wim),
            ),
        );
        *subband.offset(temp_1 as isize) = ixheaac_negate32_sat(
            ixheaac_sub32_sat(
                ixheaac_mult32x16in32_shl(re1, wim),
                ixheaac_mult32x16in32_shl(im1, wre),
            ),
        );
        let fresh8 = ptr1;
        ptr1 = ptr1.offset(1);
        wim = *fresh8;
        let fresh9 = ptr2;
        ptr2 = ptr2.offset(-1);
        wre = *fresh9;
        *subband.offset((temp_1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = ixheaac_negate32_sat(
            ixheaac_add32_sat(
                ixheaac_mult32x16in32_shl(re2, wim),
                ixheaac_mult32x16in32_shl(im2, wre),
            ),
        );
        *subband.offset((62 as WORD32 - temp_1) as isize) = ixheaac_negate32_sat(
            ixheaac_sub32_sat(
                ixheaac_mult32x16in32_shl(re2, wre),
                ixheaac_mult32x16in32_shl(im2, wim),
            ),
        );
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_inverse_modulation(
    mut qmf_real: *mut WORD32,
    mut qmf_imag: *mut WORD32,
    mut qmf_table_ptr: *mut ia_mps_dec_qmf_tables_struct,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut ptr1: *const WORD16 = 0 as *const WORD16;
    let mut ptr2: *const WORD16 = 0 as *const WORD16;
    let mut ptr3: *const WORD16 = 0 as *const WORD16;
    let mut ptr4: *const WORD16 = 0 as *const WORD16;
    let mut fft: *const WORD16 = ((*qmf_table_ptr).fft_c).as_mut_ptr();
    let mut wre: WORD16 = 0;
    let mut wim: WORD16 = 0;
    let mut re1: WORD32 = 0;
    let mut im1: WORD32 = 0;
    let mut re2: WORD32 = 0;
    let mut im2: WORD32 = 0;
    let mut re12: WORD32 = 0;
    let mut im12: WORD32 = 0;
    let mut re22: WORD32 = 0;
    let mut im22: WORD32 = 0;
    let mut temp_1: WORD32 = 0;
    ptr1 = ((*qmf_table_ptr).sbr_sin_twiddle).as_mut_ptr();
    ptr2 = ((*qmf_table_ptr).sbr_cos_twiddle).as_mut_ptr();
    ptr3 = ptr1.offset(31 as core::ffi::c_int as isize);
    ptr4 = ptr2.offset(31 as core::ffi::c_int as isize);
    i = 0 as core::ffi::c_int as WORD32;
    while i < 16 as core::ffi::c_int {
        temp_1 = i << 1 as core::ffi::c_int;
        re1 = *qmf_real.offset(temp_1 as isize);
        im1 = *qmf_real.offset((63 as WORD32 - temp_1) as isize);
        let fresh95 = ptr1;
        ptr1 = ptr1.offset(1);
        wim = *fresh95;
        let fresh96 = ptr2;
        ptr2 = ptr2.offset(1);
        wre = *fresh96;
        *qmf_real.offset(temp_1 as isize) = ixheaac_add32_sat(
            ixheaac_mult32x16in32_shl(im1, wim),
            ixheaac_mult32x16in32_shl(re1, wre),
        );
        re12 = *qmf_imag.offset(temp_1 as isize);
        im12 = *qmf_imag.offset((63 as WORD32 - temp_1) as isize);
        *qmf_imag.offset(temp_1 as isize) = ixheaac_sub32_sat(
            ixheaac_mult32x16in32_shl(im12, wim),
            ixheaac_mult32x16in32_shl(re12, wre),
        );
        im2 = *qmf_real
            .offset((temp_1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
        *qmf_real
            .offset((temp_1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = ixheaac_sub32_sat(
            ixheaac_mult32x16in32_shl(im1, wre),
            ixheaac_mult32x16in32_shl(re1, wim),
        );
        im22 = *qmf_imag
            .offset((temp_1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
        *qmf_imag
            .offset((temp_1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = ixheaac_add32_sat(
            ixheaac_mult32x16in32_shl(im12, wre),
            ixheaac_mult32x16in32_shl(re12, wim),
        );
        let fresh97 = ptr3;
        ptr3 = ptr3.offset(-1);
        wim = *fresh97;
        let fresh98 = ptr4;
        ptr4 = ptr4.offset(-1);
        wre = *fresh98;
        re2 = *qmf_real.offset((62 as WORD32 - temp_1) as isize);
        *qmf_real.offset((62 as WORD32 - temp_1) as isize) = ixheaac_add32_sat(
            ixheaac_mult32x16in32_shl(im2, wim),
            ixheaac_mult32x16in32_shl(re2, wre),
        );
        re22 = *qmf_imag.offset((62 as WORD32 - temp_1) as isize);
        *qmf_imag.offset((62 as WORD32 - temp_1) as isize) = ixheaac_sub32_sat(
            ixheaac_mult32x16in32_shl(im22, wim),
            ixheaac_mult32x16in32_shl(re22, wre),
        );
        *qmf_real.offset((63 as WORD32 - temp_1) as isize) = ixheaac_sub32_sat(
            ixheaac_mult32x16in32_shl(im2, wre),
            ixheaac_mult32x16in32_shl(re2, wim),
        );
        *qmf_imag.offset((63 as WORD32 - temp_1) as isize) = ixheaac_add32_sat(
            ixheaac_mult32x16in32_shl(im22, wre),
            ixheaac_mult32x16in32_shl(re22, wim),
        );
        i += 1;
    }
    ixheaacd_fft32(qmf_real, fft);
    ixheaacd_fft32(qmf_imag, fft);
    ptr1 = ((*qmf_table_ptr).sbr_alt_sin_twiddle).as_mut_ptr();
    ptr2 = ptr1.offset(32 as core::ffi::c_int as isize);
    let fresh99 = ptr1;
    ptr1 = ptr1.offset(1);
    wim = *fresh99;
    let fresh100 = ptr2;
    ptr2 = ptr2.offset(-1);
    wre = *fresh100;
    i = 0 as core::ffi::c_int as WORD32;
    while i < 16 as core::ffi::c_int {
        temp_1 = i << 1 as core::ffi::c_int;
        re1 = *qmf_real.offset(temp_1 as isize);
        im1 = *qmf_real
            .offset((temp_1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
        re12 = *qmf_imag.offset(temp_1 as isize);
        im12 = *qmf_imag
            .offset((temp_1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
        *qmf_real.offset(temp_1 as isize) = ixheaac_sub32_sat(
            ixheaac_mult32x16in32_shl(ixheaac_sub32_sat(im12, re1), wre),
            ixheaac_mult32x16in32_shl(ixheaac_add32_sat(im1, re12), wim),
        );
        *qmf_imag.offset(temp_1 as isize) = ixheaac_add32_sat(
            ixheaac_mult32x16in32_shl(ixheaac_add32_sat(im12, re1), wre),
            ixheaac_mult32x16in32_shl(ixheaac_sub32_sat(im1, re12), wim),
        );
        im2 = *qmf_real.offset((63 as WORD32 - temp_1) as isize);
        im22 = *qmf_imag.offset((63 as WORD32 - temp_1) as isize);
        *qmf_real.offset((63 as WORD32 - temp_1) as isize) = ixheaac_sub32_sat(
            ixheaac_mult32x16in32_shl(ixheaac_sub32_sat(im1, re12), wre),
            ixheaac_mult32x16in32_shl(ixheaac_add32_sat(im12, re1), wim),
        );
        *qmf_imag.offset((63 as WORD32 - temp_1) as isize) = ixheaac_sub32_sat(
            ixheaac_mult32x16in32_shl(ixheaac_sub32_sat(re1, im12), wim),
            ixheaac_mult32x16in32_shl(ixheaac_add32_sat(im1, re12), wre),
        );
        let fresh101 = ptr1;
        ptr1 = ptr1.offset(1);
        wim = *fresh101;
        let fresh102 = ptr2;
        ptr2 = ptr2.offset(-1);
        wre = *fresh102;
        re2 = *qmf_real.offset((62 as WORD32 - temp_1) as isize);
        re22 = *qmf_imag.offset((62 as WORD32 - temp_1) as isize);
        *qmf_real
            .offset((temp_1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = ixheaac_sub32_sat(
            ixheaac_mult32x16in32_shl(ixheaac_sub32_sat(im2, re22), wim),
            ixheaac_mult32x16in32_shl(ixheaac_add32_sat(im22, re2), wre),
        );
        *qmf_imag
            .offset((temp_1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = ixheaac_sub32_sat(
            ixheaac_mult32x16in32_shl(ixheaac_sub32_sat(re2, im22), wre),
            ixheaac_mult32x16in32_shl(ixheaac_add32_sat(im2, re22), wim),
        );
        *qmf_real.offset((62 as WORD32 - temp_1) as isize) = ixheaac_sub32_sat(
            ixheaac_mult32x16in32_shl(ixheaac_sub32_sat(im22, re2), wim),
            ixheaac_mult32x16in32_shl(ixheaac_add32_sat(im2, re22), wre),
        );
        *qmf_imag.offset((62 as WORD32 - temp_1) as isize) = ixheaac_add32_sat(
            ixheaac_mult32x16in32_shl(ixheaac_add32_sat(re2, im22), wim),
            ixheaac_mult32x16in32_shl(ixheaac_sub32_sat(im2, re22), wre),
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_calculate_syn_filt_bank_res64(
    mut syn: *mut ia_mps_dec_qmf_syn_filter_bank,
    mut sr: *mut WORD32,
    mut si: *mut WORD32,
    mut time_sig: *mut WORD32,
    mut channel: WORD32,
    mut resolution: WORD32,
    mut nr_samples: WORD32,
    mut qmf_table_ptr: *mut ia_mps_dec_qmf_tables_struct,
) -> VOID {
    let mut j: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut synth_buf: *mut WORD32 = 0 as *mut WORD32;
    let mut syn_buf_p1: *mut WORD32 = 0 as *mut WORD32;
    let mut syn_buf_p2: *mut WORD32 = 0 as *mut WORD32;
    let mut syn_buf_p3: *mut WORD32 = 0 as *mut WORD32;
    let mut val: WORD32 = 0;
    let mut p_filter_1: *const WORD32 = 0 as *const WORD32;
    let mut p_filter_6: *const WORD32 = 0 as *const WORD32;
    let mut p_filter_2: *const WORD32 = 0 as *const WORD32;
    let mut p_filter_7: *const WORD32 = 0 as *const WORD32;
    let mut p_filter_3: *const WORD32 = 0 as *const WORD32;
    let mut p_filter_8: *const WORD32 = 0 as *const WORD32;
    let mut p_filter_4: *const WORD32 = 0 as *const WORD32;
    let mut p_filter_9: *const WORD32 = 0 as *const WORD32;
    let mut p_filter_5: *const WORD32 = 0 as *const WORD32;
    let mut p_filter_10: *const WORD32 = 0 as *const WORD32;
    let mut p_sr: *mut WORD32 = 0 as *mut WORD32;
    let mut p_si: *mut WORD32 = 0 as *mut WORD32;
    let mut sbr_qmf_states_synthesis: *mut WORD32 = (*syn).sbr_qmf_states_synthesis;
    synth_buf = &mut *sbr_qmf_states_synthesis
        .offset((channel as core::ffi::c_int * QMF_FILTER_STATE_SYN_SIZE_MPS) as isize)
        as *mut WORD32;
    p_sr = sr;
    p_si = si;
    k = 0 as core::ffi::c_int as WORD32;
    while k < nr_samples {
        let mut new_samp: *mut WORD32 = p_si;
        let mut new_samp2: *mut WORD32 = 0 as *mut WORD32;
        ixheaacd_inverse_modulation(p_sr, p_si, qmf_table_ptr);
        p_filter_1 = (*syn).p_filter_syn;
        p_filter_2 = p_filter_1.offset(64 as core::ffi::c_int as isize);
        p_filter_3 = p_filter_2.offset(65 as core::ffi::c_int as isize);
        p_filter_4 = p_filter_3.offset(65 as core::ffi::c_int as isize);
        p_filter_5 = p_filter_4.offset(65 as core::ffi::c_int as isize);
        syn_buf_p1 = &mut *synth_buf.offset(63 as core::ffi::c_int as isize)
            as *mut WORD32;
        val = *p_sr;
        let mut val1: WORD32 = *p_si.offset(63 as core::ffi::c_int as isize);
        syn_buf_p2 = &mut *synth_buf.offset(63 as core::ffi::c_int as isize)
            as *mut WORD32;
        let fresh103 = time_sig;
        time_sig = time_sig.offset(1);
        *fresh103 = ixheaac_add32_sat(
            *syn_buf_p1.offset(512 as core::ffi::c_int as isize),
            ixheaacd_mps_mult32_shr_30(
                *p_filter_5.offset(65 as core::ffi::c_int as isize),
                val,
            ),
        );
        *syn_buf_p1.offset(512 as core::ffi::c_int as isize) = ixheaac_add32_sat(
            *syn_buf_p2.offset(448 as core::ffi::c_int as isize),
            ixheaacd_mps_mult32_shr_30(
                *p_filter_5.offset(64 as core::ffi::c_int as isize),
                val1,
            ),
        );
        let fresh104 = p_filter_5;
        p_filter_5 = p_filter_5.offset(1);
        *syn_buf_p2.offset(448 as core::ffi::c_int as isize) = ixheaac_add32_sat(
            *syn_buf_p1.offset(384 as core::ffi::c_int as isize),
            ixheaacd_mps_mult32_shr_30(*fresh104, val),
        );
        *syn_buf_p1.offset(384 as core::ffi::c_int as isize) = ixheaac_add32_sat(
            *syn_buf_p2.offset(320 as core::ffi::c_int as isize),
            ixheaacd_mps_mult32_shr_30(
                *p_filter_4.offset(64 as core::ffi::c_int as isize),
                val1,
            ),
        );
        let fresh105 = p_filter_4;
        p_filter_4 = p_filter_4.offset(1);
        *syn_buf_p2.offset(320 as core::ffi::c_int as isize) = ixheaac_add32_sat(
            *syn_buf_p1.offset(256 as core::ffi::c_int as isize),
            ixheaacd_mps_mult32_shr_30(*fresh105, val),
        );
        *syn_buf_p1.offset(256 as core::ffi::c_int as isize) = ixheaac_add32_sat(
            *syn_buf_p2.offset(192 as core::ffi::c_int as isize),
            ixheaacd_mps_mult32_shr_30(
                *p_filter_3.offset(64 as core::ffi::c_int as isize),
                val1,
            ),
        );
        let fresh106 = p_filter_3;
        p_filter_3 = p_filter_3.offset(1);
        *syn_buf_p2.offset(192 as core::ffi::c_int as isize) = ixheaac_add32_sat(
            *syn_buf_p1.offset(128 as core::ffi::c_int as isize),
            ixheaacd_mps_mult32_shr_30(*fresh106, val),
        );
        *syn_buf_p1.offset(128 as core::ffi::c_int as isize) = ixheaac_add32_sat(
            *syn_buf_p2.offset(64 as core::ffi::c_int as isize),
            ixheaacd_mps_mult32_shr_30(
                *p_filter_2.offset(64 as core::ffi::c_int as isize),
                val1,
            ),
        );
        let fresh107 = p_filter_2;
        p_filter_2 = p_filter_2.offset(1);
        *syn_buf_p2.offset(64 as core::ffi::c_int as isize) = ixheaac_add32_sat(
            *syn_buf_p1.offset(0 as core::ffi::c_int as isize),
            ixheaacd_mps_mult32_shr_30(*fresh107, val),
        );
        *syn_buf_p1.offset(0 as core::ffi::c_int as isize) = ixheaacd_mps_mult32_shr_30(
            *p_filter_1.offset(63 as core::ffi::c_int as isize),
            val1,
        );
        p_filter_6 = p_filter_1.offset(62 as core::ffi::c_int as isize);
        p_filter_7 = p_filter_2.offset(62 as core::ffi::c_int as isize);
        p_filter_8 = p_filter_3.offset(62 as core::ffi::c_int as isize);
        p_filter_9 = p_filter_4.offset(62 as core::ffi::c_int as isize);
        p_filter_10 = p_filter_5.offset(62 as core::ffi::c_int as isize);
        time_sig = time_sig.offset(62 as core::ffi::c_int as isize);
        syn_buf_p2 = synth_buf;
        syn_buf_p3 = syn_buf_p2;
        new_samp2 = p_sr.offset(63 as core::ffi::c_int as isize);
        j = 0 as core::ffi::c_int as WORD32;
        while j < resolution as core::ffi::c_int - 1 as core::ffi::c_int {
            let fresh108 = p_filter_6;
            p_filter_6 = p_filter_6.offset(-1);
            let fresh109 = time_sig;
            time_sig = time_sig.offset(-1);
            *fresh109 = ixheaac_add32_sat(
                *syn_buf_p3.offset(512 as core::ffi::c_int as isize),
                ixheaacd_mps_mult32_shr_30(*fresh108, *new_samp2),
            );
            let fresh110 = p_filter_5;
            p_filter_5 = p_filter_5.offset(1);
            *syn_buf_p3.offset(512 as core::ffi::c_int as isize) = ixheaac_add32_sat(
                *syn_buf_p2.offset(448 as core::ffi::c_int as isize),
                ixheaacd_mps_mult32_shr_30(*fresh110, *new_samp),
            );
            let fresh111 = p_filter_7;
            p_filter_7 = p_filter_7.offset(-1);
            *syn_buf_p2.offset(448 as core::ffi::c_int as isize) = ixheaac_add32_sat(
                *syn_buf_p3.offset(384 as core::ffi::c_int as isize),
                ixheaacd_mps_mult32_shr_30(*fresh111, *new_samp2),
            );
            let fresh112 = p_filter_4;
            p_filter_4 = p_filter_4.offset(1);
            *syn_buf_p3.offset(384 as core::ffi::c_int as isize) = ixheaac_add32_sat(
                *syn_buf_p2.offset(320 as core::ffi::c_int as isize),
                ixheaacd_mps_mult32_shr_30(*fresh112, *new_samp),
            );
            let fresh113 = p_filter_8;
            p_filter_8 = p_filter_8.offset(-1);
            *syn_buf_p2.offset(320 as core::ffi::c_int as isize) = ixheaac_add32_sat(
                *syn_buf_p3.offset(256 as core::ffi::c_int as isize),
                ixheaacd_mps_mult32_shr_30(*fresh113, *new_samp2),
            );
            let fresh114 = p_filter_3;
            p_filter_3 = p_filter_3.offset(1);
            *syn_buf_p3.offset(256 as core::ffi::c_int as isize) = ixheaac_add32_sat(
                *syn_buf_p2.offset(192 as core::ffi::c_int as isize),
                ixheaacd_mps_mult32_shr_30(*fresh114, *new_samp),
            );
            let fresh115 = p_filter_9;
            p_filter_9 = p_filter_9.offset(-1);
            *syn_buf_p2.offset(192 as core::ffi::c_int as isize) = ixheaac_add32_sat(
                *syn_buf_p3.offset(128 as core::ffi::c_int as isize),
                ixheaacd_mps_mult32_shr_30(*fresh115, *new_samp2),
            );
            let fresh116 = p_filter_2;
            p_filter_2 = p_filter_2.offset(1);
            *syn_buf_p3.offset(128 as core::ffi::c_int as isize) = ixheaac_add32_sat(
                *syn_buf_p2.offset(64 as core::ffi::c_int as isize),
                ixheaacd_mps_mult32_shr_30(*fresh116, *new_samp),
            );
            let fresh117 = p_filter_10;
            p_filter_10 = p_filter_10.offset(-1);
            *syn_buf_p2.offset(64 as core::ffi::c_int as isize) = ixheaac_add32_sat(
                *syn_buf_p3.offset(0 as core::ffi::c_int as isize),
                ixheaacd_mps_mult32_shr_30(*fresh117, *new_samp2),
            );
            let fresh118 = p_filter_1;
            p_filter_1 = p_filter_1.offset(1);
            *syn_buf_p3.offset(0 as core::ffi::c_int as isize) = ixheaacd_mps_mult32_shr_30(
                *fresh118,
                *new_samp,
            );
            new_samp = new_samp.offset(1);
            syn_buf_p2 = syn_buf_p2.offset(1);
            new_samp2 = new_samp2.offset(-1);
            syn_buf_p3 = syn_buf_p3.offset(1);
            j += 1;
        }
        time_sig = time_sig.offset(64 as core::ffi::c_int as isize);
        p_sr = p_sr.offset(MAX_HYBRID_BANDS as isize);
        p_si = p_si.offset(MAX_HYBRID_BANDS as isize);
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_calculate_syn_filt_bank(
    mut syn: *mut ia_mps_dec_qmf_syn_filter_bank,
    mut sr: *mut WORD32,
    mut si: *mut WORD32,
    mut time_sig: *mut WORD32,
    mut channel: WORD32,
    mut resolution: WORD32,
    mut nr_samples: WORD32,
    mut qmf_table_ptr: *mut ia_mps_dec_qmf_tables_struct,
) -> VOID {
    let mut j: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut synth_buf: *mut WORD32 = 0 as *mut WORD32;
    let mut p_sr: *mut WORD32 = 0 as *mut WORD32;
    let mut p_si: *mut WORD32 = 0 as *mut WORD32;
    let mut buf_ptr: *mut WORD32 = 0 as *mut WORD32;
    let mut resx2: WORD32 = resolution << 1 as core::ffi::c_int;
    let mut sbr_qmf_states_synthesis: *mut WORD32 = (*syn).sbr_qmf_states_synthesis;
    synth_buf = &mut *sbr_qmf_states_synthesis
        .offset((channel as core::ffi::c_int * QMF_FILTER_STATE_SYN_SIZE_MPS) as isize)
        as *mut WORD32;
    p_sr = sr;
    p_si = si;
    k = 0 as core::ffi::c_int as WORD32;
    while k < nr_samples {
        let mut new_samp: *mut WORD32 = p_si.offset(63 as core::ffi::c_int as isize);
        let mut p_filter_1: *const WORD32 = (*syn).p_filter_syn;
        let mut p_filter_2: *const WORD32 = p_filter_1
            .offset(65 as core::ffi::c_int as isize);
        let mut p_filter_3: *const WORD32 = p_filter_2
            .offset(65 as core::ffi::c_int as isize);
        let mut p_filter_4: *const WORD32 = p_filter_3
            .offset(65 as core::ffi::c_int as isize);
        let mut p_filter_5: *const WORD32 = p_filter_4
            .offset(65 as core::ffi::c_int as isize);
        ixheaacd_inverse_modulation(p_sr, p_si, qmf_table_ptr);
        j = 0 as core::ffi::c_int as WORD32;
        while j < resolution {
            let fresh82 = p_filter_1;
            p_filter_1 = p_filter_1.offset(1);
            *synth_buf.offset(j as isize) = ixheaacd_mps_mult32_shr_30(
                *fresh82,
                *new_samp,
            );
            let fresh83 = p_filter_2;
            p_filter_2 = p_filter_2.offset(1);
            *synth_buf.offset((resx2 + j) as isize) = ixheaac_add32_sat(
                *synth_buf.offset((resx2 + j) as isize),
                ixheaacd_mps_mult32_shr_30(*fresh83, *new_samp),
            );
            let fresh84 = p_filter_3;
            p_filter_3 = p_filter_3.offset(1);
            *synth_buf.offset((resx2 * 2 as WORD32 + j) as isize) = ixheaac_add32_sat(
                *synth_buf.offset((resx2 * 2 as WORD32 + j) as isize),
                ixheaacd_mps_mult32_shr_30(*fresh84, *new_samp),
            );
            let fresh85 = p_filter_4;
            p_filter_4 = p_filter_4.offset(1);
            *synth_buf.offset((resx2 * 3 as WORD32 + j) as isize) = ixheaac_add32_sat(
                *synth_buf.offset((resx2 * 3 as WORD32 + j) as isize),
                ixheaacd_mps_mult32_shr_30(*fresh85, *new_samp),
            );
            let fresh86 = p_filter_5;
            p_filter_5 = p_filter_5.offset(1);
            *synth_buf.offset((resx2 * 4 as WORD32 + j) as isize) = ixheaac_add32_sat(
                *synth_buf.offset((resx2 * 4 as WORD32 + j) as isize),
                ixheaacd_mps_mult32_shr_30(*fresh86, *new_samp),
            );
            new_samp = new_samp.offset(-1);
            j += 1;
        }
        let fresh87 = p_filter_1;
        p_filter_1 = p_filter_1.offset(1);
        *synth_buf
            .offset((resx2 as core::ffi::c_int - 1 as core::ffi::c_int) as isize) = ixheaac_add32_sat(
            *synth_buf
                .offset((resx2 as core::ffi::c_int - 1 as core::ffi::c_int) as isize),
            ixheaacd_mps_mult32_shr_30(*fresh87, *p_sr),
        );
        let fresh88 = p_filter_2;
        p_filter_2 = p_filter_2.offset(1);
        *synth_buf
            .offset(
                (resx2 as core::ffi::c_int * 2 as core::ffi::c_int
                    - 1 as core::ffi::c_int) as isize,
            ) = ixheaac_add32_sat(
            *synth_buf
                .offset(
                    (resx2 as core::ffi::c_int * 2 as core::ffi::c_int
                        - 1 as core::ffi::c_int) as isize,
                ),
            ixheaacd_mps_mult32_shr_30(*fresh88, *p_sr),
        );
        let fresh89 = p_filter_3;
        p_filter_3 = p_filter_3.offset(1);
        *synth_buf
            .offset(
                (3 as core::ffi::c_int * resx2 as core::ffi::c_int
                    - 1 as core::ffi::c_int) as isize,
            ) = ixheaac_add32_sat(
            *synth_buf
                .offset(
                    (3 as core::ffi::c_int * resx2 as core::ffi::c_int
                        - 1 as core::ffi::c_int) as isize,
                ),
            ixheaacd_mps_mult32_shr_30(*fresh89, *p_sr),
        );
        let fresh90 = p_filter_4;
        p_filter_4 = p_filter_4.offset(1);
        *synth_buf
            .offset(
                (4 as core::ffi::c_int * resx2 as core::ffi::c_int
                    - 1 as core::ffi::c_int) as isize,
            ) = ixheaac_add32_sat(
            *synth_buf
                .offset(
                    (4 as core::ffi::c_int * resx2 as core::ffi::c_int
                        - 1 as core::ffi::c_int) as isize,
                ),
            ixheaacd_mps_mult32_shr_30(*fresh90, *p_sr),
        );
        let fresh91 = p_filter_5;
        p_filter_5 = p_filter_5.offset(1);
        let fresh92 = time_sig;
        time_sig = time_sig.offset(1);
        *fresh92 = ixheaac_add32_sat(
            *synth_buf
                .offset(
                    (5 as core::ffi::c_int * resx2 as core::ffi::c_int
                        - 1 as core::ffi::c_int) as isize,
                ),
            ixheaacd_mps_mult32_shr_30(*fresh91, *p_sr),
        );
        p_filter_1 = p_filter_1.offset(-(2 as core::ffi::c_int as isize));
        p_filter_2 = p_filter_2.offset(-(2 as core::ffi::c_int as isize));
        p_filter_3 = p_filter_3.offset(-(2 as core::ffi::c_int as isize));
        p_filter_4 = p_filter_4.offset(-(2 as core::ffi::c_int as isize));
        p_filter_5 = p_filter_5.offset(-(2 as core::ffi::c_int as isize));
        new_samp = p_sr
            .offset(resolution as isize)
            .offset(-(1 as core::ffi::c_int as isize));
        j = 0 as core::ffi::c_int as WORD32;
        while j < resolution as core::ffi::c_int - 1 as core::ffi::c_int {
            p_filter_5 = p_filter_5.offset(-1);
            *synth_buf.offset((resolution + j) as isize) = ixheaac_add32_sat(
                *synth_buf.offset((resolution + j) as isize),
                ixheaacd_mps_mult32_shr_30(*p_filter_5, *new_samp),
            );
            p_filter_4 = p_filter_4.offset(-1);
            *synth_buf.offset((resolution * 3 as WORD32 + j) as isize) = ixheaac_add32_sat(
                *synth_buf.offset((resolution * 3 as WORD32 + j) as isize),
                ixheaacd_mps_mult32_shr_30(*p_filter_4, *new_samp),
            );
            p_filter_3 = p_filter_3.offset(-1);
            *synth_buf.offset((resolution * 5 as WORD32 + j) as isize) = ixheaac_add32_sat(
                *synth_buf.offset((resolution * 5 as WORD32 + j) as isize),
                ixheaacd_mps_mult32_shr_30(*p_filter_3, *new_samp),
            );
            p_filter_2 = p_filter_2.offset(-1);
            *synth_buf.offset((resolution * 7 as WORD32 + j) as isize) = ixheaac_add32_sat(
                *synth_buf.offset((resolution * 7 as WORD32 + j) as isize),
                ixheaacd_mps_mult32_shr_30(*p_filter_2, *new_samp),
            );
            p_filter_1 = p_filter_1.offset(-1);
            *synth_buf.offset((resolution * 9 as WORD32 + j) as isize) = ixheaac_add32_sat(
                *synth_buf.offset((resolution * 9 as WORD32 + j) as isize),
                ixheaacd_mps_mult32_shr_30(*p_filter_1, *new_samp),
            );
            new_samp = new_samp.offset(-1);
            j += 1;
        }
        buf_ptr = synth_buf
            .offset((9 as WORD32 * resolution) as isize)
            .offset(resolution as isize)
            .offset(-(2 as core::ffi::c_int as isize));
        j = 0 as core::ffi::c_int as WORD32;
        while j < resolution as core::ffi::c_int - 1 as core::ffi::c_int {
            let fresh93 = buf_ptr;
            buf_ptr = buf_ptr.offset(-1);
            let fresh94 = time_sig;
            time_sig = time_sig.offset(1);
            *fresh94 = *fresh93;
            j += 1;
        }
        memmove(
            synth_buf.offset(resolution as isize) as *mut core::ffi::c_void,
            synth_buf as *const core::ffi::c_void,
            ((9 as WORD32 * resolution) as size_t)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
        );
        p_sr = p_sr.offset(MAX_HYBRID_BANDS as isize);
        p_si = p_si.offset(MAX_HYBRID_BANDS as isize);
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_syn_filt_bank_init(
    mut self_0: ia_mps_dec_synthesis_interface_handle,
    mut resolution: WORD32,
) -> IA_ERRORCODE {
    match resolution {
        QMF_BANDS_32 => {
            (*self_0).syn_filter_bank = Some(
                ixheaacd_calculate_syn_filt_bank
                    as unsafe extern "C" fn(
                        *mut ia_mps_dec_qmf_syn_filter_bank,
                        *mut WORD32,
                        *mut WORD32,
                        *mut WORD32,
                        WORD32,
                        WORD32,
                        WORD32,
                        *mut ia_mps_dec_qmf_tables_struct,
                    ) -> VOID,
            )
                as Option<
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
                >;
        }
        QMF_BANDS_64 => {
            (*self_0).syn_filter_bank = Some(
                ixheaacd_calculate_syn_filt_bank_res64
                    as unsafe extern "C" fn(
                        *mut ia_mps_dec_qmf_syn_filter_bank,
                        *mut WORD32,
                        *mut WORD32,
                        *mut WORD32,
                        WORD32,
                        WORD32,
                        WORD32,
                        *mut ia_mps_dec_qmf_tables_struct,
                    ) -> VOID,
            )
                as Option<
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
                >;
        }
        QMF_BANDS_128 => {
            (*self_0).syn_filter_bank = Some(
                ixheaacd_calculate_syn_filt_bank
                    as unsafe extern "C" fn(
                        *mut ia_mps_dec_qmf_syn_filter_bank,
                        *mut WORD32,
                        *mut WORD32,
                        *mut WORD32,
                        WORD32,
                        WORD32,
                        WORD32,
                        *mut ia_mps_dec_qmf_tables_struct,
                    ) -> VOID,
            )
                as Option<
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
                >;
        }
        _ => return IA_XHEAAC_MPS_DEC_INIT_NONFATAL_INVALID_QMF_BAND,
    }
    return IA_NO_ERROR;
}
unsafe extern "C" fn ia_mps_enc_fwd_mod(
    mut time_in: *mut WORD32,
    mut r_subband: *mut WORD32,
    mut i_subband: *mut WORD32,
    mut qmf_table_ptr: *mut ia_mps_dec_qmf_tables_struct,
) -> VOID {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < 64 as core::ffi::c_int {
        *r_subband.offset(i as isize) = *time_in.offset(i as isize)
            - *time_in.offset((127 as WORD32 - i) as isize);
        *i_subband.offset(i as isize) = *time_in.offset(i as isize)
            + *time_in.offset((127 as WORD32 - i) as isize);
        i += 1;
    }
    ixheaacd_cos_mod(r_subband, qmf_table_ptr);
    ixheaacd_sin_mod(i_subband, qmf_table_ptr);
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_calc_ana_filt_bank(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
    mut time_in: *mut WORD16,
    mut r_analysis: *mut WORD32,
    mut i_analysis: *mut WORD32,
    mut channel: WORD32,
) -> VOID {
    let mut qmf_bank: *mut ia_mps_dec_qmf_ana_filter_bank = &mut *((*pstr_mps_state)
        .qmf_bank)
        .as_mut_ptr()
        .offset(channel as isize) as *mut ia_mps_dec_qmf_ana_filter_bank;
    let mut qmf_table_ptr: *mut ia_mps_dec_qmf_tables_struct = (*pstr_mps_state)
        .ia_mps_dec_mps_table
        .qmf_table_ptr;
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut m: WORD32 = 0;
    let mut syn_buffer: *mut WORD32 = (*pstr_mps_state).mps_scratch_mem_v as *mut WORD32;
    let mut accu1: WORD64 = 0 as WORD64;
    let mut accu2: WORD64 = 0 as WORD64;
    let mut flag: WORD16 = 0;
    let mut fp1: *mut WORD32 = 0 as *mut WORD32;
    let mut fp2: *mut WORD32 = 0 as *mut WORD32;
    let mut temp: *mut WORD32 = 0 as *mut WORD32;
    let mut start_co_eff_ptr_l: *const WORD32 = 0 as *const WORD32;
    let mut start_co_eff_ptr_r: *const WORD32 = 0 as *const WORD32;
    let mut ptr_pf_l: *const WORD32 = 0 as *const WORD32;
    let mut ptr_pf_r: *const WORD32 = 0 as *const WORD32;
    let mut qmf_states_curr_pos: *mut WORD32 = 0 as *mut WORD32;
    let mut offset: WORD32 = 0 as WORD32;
    let mut n_channels: WORD32 = (*pstr_mps_state).num_input_channels;
    let mut nr_samples: WORD32 = (*pstr_mps_state).time_slots;
    let mut qmf_bands: WORD32 = (*pstr_mps_state).qmf_bands;
    let mut shift: WORD32 = (*pstr_mps_state).bits_per_sample - 16 as WORD32;
    let mut gain: WORD32 = (*pstr_mps_state).clip_protect_gain;
    let mut p_ana_real: *mut WORD32 = r_analysis;
    let mut p_ana_imag: *mut WORD32 = i_analysis;
    let mut p_ana_re: *mut WORD32 = 0 as *mut WORD32;
    let mut p_ana_im: *mut WORD32 = 0 as *mut WORD32;
    flag = (*qmf_bank).flag;
    if flag == 0 {
        fp1 = (*qmf_bank).qmf_states_buffer;
        fp2 = ((*qmf_bank).qmf_states_buffer).offset(qmf_bands as isize);
    } else {
        fp2 = (*qmf_bank).qmf_states_buffer;
        fp1 = ((*qmf_bank).qmf_states_buffer).offset(qmf_bands as isize);
    }
    (*qmf_bank).qmf_states_curr_pos = ((*qmf_bank).qmf_states_buffer)
        .offset(((*qmf_bank).offset as WORD32 * qmf_bands) as isize);
    offset = (*qmf_bank).offset as WORD32;
    start_co_eff_ptr_l = ((*qmf_bank).ref_co_eff_ptr_l)
        .offset((*qmf_bank).offset_l as isize);
    start_co_eff_ptr_r = ((*qmf_bank).ref_co_eff_ptr_r)
        .offset(-((*qmf_bank).offset_r as isize));
    i = 0 as core::ffi::c_int as WORD32;
    while i < nr_samples {
        let mut pcoz: *const WORD16 = ((*qmf_table_ptr).ia_qmf_anl_addt_cos)
            .as_mut_ptr();
        let mut psin: *const WORD16 = ((*qmf_table_ptr).ia_qmf_anl_addt_sin)
            .as_mut_ptr();
        qmf_states_curr_pos = (*qmf_bank).qmf_states_curr_pos;
        p_ana_re = p_ana_real;
        p_ana_im = p_ana_imag;
        temp = fp1;
        fp1 = fp2;
        fp2 = temp;
        if flag != 0 {
            start_co_eff_ptr_l = start_co_eff_ptr_l.offset(-1);
            if start_co_eff_ptr_l == (*qmf_bank).ref_co_eff_ptr_l {
                start_co_eff_ptr_l = start_co_eff_ptr_l
                    .offset(5 as core::ffi::c_int as isize);
            }
        } else {
            start_co_eff_ptr_r = start_co_eff_ptr_r.offset(1);
            if start_co_eff_ptr_r == (*qmf_bank).ref_co_eff_ptr_r {
                start_co_eff_ptr_r = start_co_eff_ptr_r
                    .offset(-(5 as core::ffi::c_int as isize));
            }
        }
        flag += 1;
        if flag as core::ffi::c_int & ONE_BIT_MASK == 0 as core::ffi::c_int {
            flag = 0 as WORD16;
        }
        if shift == 0 as core::ffi::c_int {
            k = 0 as core::ffi::c_int as WORD32;
            while k < qmf_bands {
                *qmf_states_curr_pos.offset(k as isize) = ixheaacd_mps_mult32_shr_15(
                    *time_in
                        .offset((n_channels * (i * qmf_bands + k) + channel) as isize)
                        as WORD32,
                    gain,
                );
                k += 1;
            }
        } else {
            k = 0 as core::ffi::c_int as WORD32;
            while k < qmf_bands {
                let mut temp_0: WORD32 = 0;
                temp_0 = ixheaacd_mps_mult32_shr_15(
                    *time_in
                        .offset((n_channels * (i * qmf_bands + k) + channel) as isize)
                        as WORD32,
                    gain,
                );
                *qmf_states_curr_pos.offset(k as isize) = temp_0 >> shift;
                k += 1;
            }
        }
        ptr_pf_l = start_co_eff_ptr_l;
        ptr_pf_r = start_co_eff_ptr_r;
        k = 0 as core::ffi::c_int as WORD32;
        while k < qmf_bands {
            accu1 = *ptr_pf_l.offset(0 as core::ffi::c_int as isize) as WORD64
                * *fp1.offset(k as isize) as WORD64;
            accu1
                += *ptr_pf_l.offset(1 as core::ffi::c_int as isize) as WORD64
                    * *fp1.offset((128 as WORD32 + k) as isize) as WORD64;
            accu1
                += *ptr_pf_l.offset(2 as core::ffi::c_int as isize) as WORD64
                    * *fp1.offset((256 as WORD32 + k) as isize) as WORD64;
            accu1
                += *ptr_pf_l.offset(3 as core::ffi::c_int as isize) as WORD64
                    * *fp1.offset((384 as WORD32 + k) as isize) as WORD64;
            accu1
                += *ptr_pf_l.offset(4 as core::ffi::c_int as isize) as WORD64
                    * *fp1.offset((512 as WORD32 + k) as isize) as WORD64;
            accu2 = *ptr_pf_r.offset(-(1 as core::ffi::c_int) as isize) as WORD64
                * *fp2.offset(k as isize) as WORD64;
            accu2
                += *ptr_pf_r.offset(-(2 as core::ffi::c_int) as isize) as WORD64
                    * *fp2.offset((128 as WORD32 + k) as isize) as WORD64;
            accu2
                += *ptr_pf_r.offset(-(3 as core::ffi::c_int) as isize) as WORD64
                    * *fp2.offset((256 as WORD32 + k) as isize) as WORD64;
            accu2
                += *ptr_pf_r.offset(-(4 as core::ffi::c_int) as isize) as WORD64
                    * *fp2.offset((384 as WORD32 + k) as isize) as WORD64;
            accu2
                += *ptr_pf_r.offset(-(5 as core::ffi::c_int) as isize) as WORD64
                    * *fp2.offset((512 as WORD32 + k) as isize) as WORD64;
            *syn_buffer
                .offset(
                    ((qmf_bands << 1 as core::ffi::c_int) - 1 as WORD32 - k) as isize,
                ) = (accu1 >> 21 as core::ffi::c_int) as WORD32;
            *syn_buffer.offset((qmf_bands - 1 as WORD32 - k) as isize) = (accu2
                >> 21 as core::ffi::c_int) as WORD32;
            ptr_pf_l = ptr_pf_l.offset(10 as core::ffi::c_int as isize);
            ptr_pf_r = ptr_pf_r.offset(-(10 as core::ffi::c_int as isize));
            k += 1;
        }
        ia_mps_enc_fwd_mod(syn_buffer, p_ana_re, p_ana_im, qmf_table_ptr);
        m = 0 as core::ffi::c_int as WORD32;
        while m < qmf_bands >> 1 as core::ffi::c_int {
            let mut a_cos: WORD32 = 0;
            let mut b_cos: WORD32 = 0;
            let mut a_sin: WORD32 = 0;
            let mut b_sin: WORD32 = 0;
            let mut a_cos1: WORD32 = 0;
            let mut b_cos1: WORD32 = 0;
            let mut a_sin1: WORD32 = 0;
            let mut b_sin1: WORD32 = 0;
            let fresh0 = pcoz;
            pcoz = pcoz.offset(1);
            let mut coz: WORD16 = *fresh0;
            let fresh1 = psin;
            psin = psin.offset(1);
            let mut sin: WORD16 = *fresh1;
            a_cos = ixheaac_mult32x16in32(*p_ana_re.offset(m as isize), coz);
            b_sin = ixheaac_mult32x16in32(*p_ana_im.offset(m as isize), sin);
            b_cos = ixheaac_mult32x16in32(*p_ana_im.offset(m as isize), coz);
            a_sin = ixheaac_mult32x16in32(*p_ana_re.offset(m as isize), sin);
            *p_ana_re.offset(m as isize) = a_cos + b_sin << 1 as core::ffi::c_int;
            *p_ana_im.offset(m as isize) = b_cos - a_sin << 1 as core::ffi::c_int;
            a_cos1 = ixheaac_mult32x16in32(
                *p_ana_re.offset((qmf_bands - 1 as WORD32 - m) as isize),
                coz,
            );
            b_sin1 = ixheaac_mult32x16in32(
                *p_ana_im.offset((qmf_bands - 1 as WORD32 - m) as isize),
                sin,
            );
            a_sin1 = ixheaac_mult32x16in32(
                *p_ana_re.offset((qmf_bands - 1 as WORD32 - m) as isize),
                sin,
            );
            *p_ana_re.offset((qmf_bands - 1 as WORD32 - m) as isize) = -a_cos1 + b_sin1
                << 1 as core::ffi::c_int;
            b_cos1 = ixheaac_mult32x16in32(
                *p_ana_im.offset((qmf_bands - 1 as WORD32 - m) as isize),
                coz,
            );
            *p_ana_im.offset((qmf_bands - 1 as WORD32 - m) as isize) = -b_cos1 - a_sin1
                << 1 as core::ffi::c_int;
            m += 1;
        }
        (*qmf_bank).qmf_states_curr_pos = ((*qmf_bank).qmf_states_curr_pos)
            .offset(qmf_bands as isize);
        offset += 1;
        if offset == 10 as core::ffi::c_int {
            offset = 0 as core::ffi::c_int as WORD32;
            (*qmf_bank).qmf_states_curr_pos = (*qmf_bank).qmf_states_buffer;
        }
        p_ana_real = p_ana_real.offset(MAX_NUM_QMF_BANDS as isize);
        p_ana_imag = p_ana_imag.offset(MAX_NUM_QMF_BANDS as isize);
        i += 1;
    }
    (*qmf_bank).offset_l = start_co_eff_ptr_l.offset_from((*qmf_bank).ref_co_eff_ptr_l)
        as core::ffi::c_long as WORD32;
    (*qmf_bank).offset_r = ((*qmf_bank).ref_co_eff_ptr_r).offset_from(start_co_eff_ptr_r)
        as core::ffi::c_long as WORD32;
    (*qmf_bank).flag = flag;
    (*qmf_bank).offset = offset as WORD16;
}
