extern "C" {
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn ixheaacd_decorr_apply(
        pstr_mps_state: *mut ia_heaac_mps_state_struct,
        length: WORD32,
        input_real: *mut WORD32,
        input_imag: *mut WORD32,
        output_real: *mut WORD32,
        output_imag: *mut WORD32,
        index: WORD32,
    ) -> VOID;
    fn ixheaacd_apply_ana_hyb_filt_bank_merge_res_decor(
        hyb_state: *mut ia_mps_dec_thyb_filter_state_struct,
        m_qmf_real: *mut WORD32,
        m_qmf_imag: *mut WORD32,
        nr_bands: WORD32,
        nr_samples: WORD32,
        m_hybrid_real: *mut WORD32,
        m_hybrid_imag: *mut WORD32,
        hyb_tab_ptr: *const ia_mps_dec_hybrid_tables_struct,
    ) -> VOID;
    fn ixheaacd_apply_ana_hyb_filt_bank_create_x(
        hyb_state: *mut ia_mps_dec_thyb_filter_state_struct,
        m_qmf_real: *mut WORD32,
        m_qmf_imag: *mut WORD32,
        nr_bands: WORD32,
        nr_samples: WORD32,
        m_hybrid_real: *mut WORD32,
        m_hybrid_imag: *mut WORD32,
        hyb_tab_ptr: *const ia_mps_dec_hybrid_tables_struct,
    ) -> VOID;
    fn ixheaacd_apply_ana_hyb_filt_bank_create_x_res(
        hyb_state: *mut ia_mps_dec_thyb_filter_state_struct,
        m_qmf_real: *mut WORD32,
        m_qmf_imag: *mut WORD32,
        nr_bands: WORD32,
        nr_samples: WORD32,
        m_hybrid_real: *mut WORD32,
        m_hybrid_imag: *mut WORD32,
        indx: *mut size_t,
        res: WORD32,
        hyb_bands: WORD32,
        num_parameter_bands: WORD32,
        counter: *mut WORD32,
        hyb_tab_ptr: *const ia_mps_dec_hybrid_tables_struct,
    ) -> VOID;
    fn ixheaacd_mdct2qmf_process(
        upd_qmf: WORD32,
        mdct_in: *mut WORD32,
        qmf_real_pre: *mut WORD32,
        qmf_real_post: *mut WORD32,
        qmf_imag_pre: *mut WORD32,
        qmf_imag_post: *mut WORD32,
        window_type: WORD32,
        qmf_global_offset: WORD32,
        ia_mps_dec_mps_table_ptr: *mut ia_mps_dec_mps_tables_struct,
        scratch: *mut core::ffi::c_void,
        time_slots: WORD32,
    ) -> VOID;
    fn ixheaacd_get_res_idx(
        pstr_mps_state: *mut ia_heaac_mps_state_struct,
        row: WORD32,
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
pub type VOID = ();
pub type FLAG = core::ffi::c_int;
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
pub struct ia_mps_dec_ducker_interface {
    pub apply: Option<
        unsafe extern "C" fn(
            *mut ia_mps_dec_ducker_interface,
            WORD32,
            *const WORD32,
            *const WORD32,
            *mut WORD32,
            *mut WORD32,
            *mut ia_mps_dec_mps_tables_struct,
            *mut core::ffi::c_void,
        ) -> VOID,
    >,
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
pub const MAX_TIME_SLOTS: core::ffi::c_int = 72 as core::ffi::c_int;
pub const MAX_NUM_QMF_BANDS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const MAX_HYBRID_BANDS: core::ffi::c_int = MAX_NUM_QMF_BANDS - 3 as core::ffi::c_int
    + 10 as core::ffi::c_int;
pub const ONE_BY_SQRT_TWO_Q30: core::ffi::c_int = 759250125 as core::ffi::c_int;
pub const QBXTS: core::ffi::c_int = 4608 as core::ffi::c_int;
pub const TSXHB: core::ffi::c_int = 5112 as core::ffi::c_int;
pub const RFX2XMDCTCOEF: core::ffi::c_int = 8192 as core::ffi::c_int;
pub const MDCTCOEFX2: core::ffi::c_int = 2048 as core::ffi::c_int;
pub const TSXHBX5: core::ffi::c_int = 25560 as core::ffi::c_int;
#[inline]
unsafe extern "C" fn ixheaacd_mps_mult32_shr_30(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 30 as core::ffi::c_int) as WORD32;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mdct_2_qmf(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
) -> VOID {
    let mut p_array_struct: *mut ia_mps_dec_reuse_array_struct = (*pstr_mps_state)
        .array_struct;
    let mut ch: WORD32 = 0;
    let mut rfpsf: WORD32 = 0;
    let mut qb: WORD32 = 0;
    let mut qmf_global_offset: WORD32 = 0;
    let mut time_slots: WORD32 = (*pstr_mps_state).time_slots;
    let mut time_slots_x4: WORD32 = time_slots << 2 as core::ffi::c_int;
    let mut qmf_bands: WORD32 = (*pstr_mps_state).qmf_bands;
    let mut p_qmf_residual_real_post: *mut WORD32 = 0 as *mut WORD32;
    let mut p_qmf_residual_imag_post: *mut WORD32 = 0 as *mut WORD32;
    let mut scratch: *mut core::ffi::c_void = (*pstr_mps_state).mps_scratch_mem_v;
    if (*pstr_mps_state).up_mix_type != 2 as core::ffi::c_int {
        let mut num_ch: WORD32 = (*pstr_mps_state).num_ott_boxes
            + (*pstr_mps_state).num_ttt_boxes;
        let mut rfpsf_max: WORD32 = (*pstr_mps_state).residual_frames_per_spatial_frame;
        let mut upd_qmf: WORD32 = (*pstr_mps_state).upd_qmf;
        let mut qmf_residual_real_pre: *mut WORD32 = (*p_array_struct)
            .qmf_residual_real_pre;
        let mut qmf_residual_real_post: *mut WORD32 = (*p_array_struct)
            .qmf_residual_real_post;
        let mut qmf_residual_imag_pre: *mut WORD32 = (*p_array_struct)
            .qmf_residual_imag_pre;
        let mut qmf_residual_imag_post: *mut WORD32 = (*p_array_struct)
            .qmf_residual_imag_post;
        let mut p_res_mdct: *mut WORD32 = (*p_array_struct).res_mdct;
        ch = 0 as core::ffi::c_int as WORD32;
        while ch < num_ch {
            if (*pstr_mps_state).bs_config.bs_residual_present[ch as usize] != 0 {
                let mut res_mdct: *mut WORD32 = p_res_mdct;
                qmf_global_offset = 0 as core::ffi::c_int as WORD32;
                p_qmf_residual_real_post = qmf_residual_real_post;
                p_qmf_residual_imag_post = qmf_residual_imag_post;
                qb = 0 as core::ffi::c_int as WORD32;
                while qb < qmf_bands {
                    memset(
                        p_qmf_residual_real_post as *mut core::ffi::c_void,
                        0 as core::ffi::c_int,
                        time_slots_x4 as size_t,
                    );
                    memset(
                        p_qmf_residual_imag_post as *mut core::ffi::c_void,
                        0 as core::ffi::c_int,
                        time_slots_x4 as size_t,
                    );
                    p_qmf_residual_real_post = p_qmf_residual_real_post
                        .offset(MAX_TIME_SLOTS as isize);
                    p_qmf_residual_imag_post = p_qmf_residual_imag_post
                        .offset(MAX_TIME_SLOTS as isize);
                    qb += 1;
                }
                rfpsf = 0 as core::ffi::c_int as WORD32;
                while rfpsf < rfpsf_max {
                    ixheaacd_mdct2qmf_process(
                        upd_qmf,
                        res_mdct,
                        qmf_residual_real_pre,
                        qmf_residual_real_post,
                        qmf_residual_imag_pre,
                        qmf_residual_imag_post,
                        (*pstr_mps_state).res_block_type[ch as usize][rfpsf as usize],
                        qmf_global_offset,
                        &mut (*pstr_mps_state).ia_mps_dec_mps_table,
                        scratch,
                        time_slots,
                    );
                    qmf_global_offset += upd_qmf;
                    res_mdct = res_mdct.offset(MDCTCOEFX2 as isize);
                    rfpsf += 1;
                }
            }
            qmf_residual_real_pre = qmf_residual_real_pre.offset(QBXTS as isize);
            qmf_residual_imag_pre = qmf_residual_imag_pre.offset(QBXTS as isize);
            qmf_residual_real_post = qmf_residual_real_post.offset(QBXTS as isize);
            qmf_residual_imag_post = qmf_residual_imag_post.offset(QBXTS as isize);
            p_res_mdct = p_res_mdct.offset(RFX2XMDCTCOEF as isize);
            ch += 1;
        }
    }
    if (*pstr_mps_state).arbitrary_downmix == 2 as core::ffi::c_int {
        let mut arbdmx_upd_qmf: WORD32 = (*pstr_mps_state).arbdmx_upd_qmf;
        let mut offset: WORD32 = (*pstr_mps_state).num_ott_boxes
            + (*pstr_mps_state).num_ttt_boxes;
        let mut in_ch: WORD32 = (*pstr_mps_state).num_input_channels;
        let mut rfpsf_max_0: WORD32 = (*pstr_mps_state).arbdmx_frames_per_spatial_frame;
        let mut qmf_residual_real_pre_0: *mut WORD32 = ((*p_array_struct)
            .qmf_residual_real_pre)
            .offset((offset as core::ffi::c_int * QBXTS) as isize);
        let mut qmf_residual_imag_pre_0: *mut WORD32 = ((*p_array_struct)
            .qmf_residual_imag_pre)
            .offset((offset as core::ffi::c_int * QBXTS) as isize);
        let mut qmf_residual_real_post_0: *mut WORD32 = ((*p_array_struct)
            .qmf_residual_real_post)
            .offset((offset as core::ffi::c_int * QBXTS) as isize);
        let mut qmf_residual_imag_post_0: *mut WORD32 = ((*p_array_struct)
            .qmf_residual_imag_post)
            .offset((offset as core::ffi::c_int * QBXTS) as isize);
        let mut p_res_mdct_0: *mut WORD32 = ((*p_array_struct).res_mdct)
            .offset((offset as core::ffi::c_int * RFX2XMDCTCOEF) as isize);
        ch = 0 as core::ffi::c_int as WORD32;
        while ch < in_ch {
            let mut res_mdct_0: *mut WORD32 = p_res_mdct_0;
            qmf_global_offset = 0 as core::ffi::c_int as WORD32;
            p_qmf_residual_real_post = qmf_residual_real_post_0;
            p_qmf_residual_imag_post = qmf_residual_imag_post_0;
            qb = 0 as core::ffi::c_int as WORD32;
            while qb < qmf_bands {
                memset(
                    p_qmf_residual_real_post as *mut core::ffi::c_void,
                    0 as core::ffi::c_int,
                    time_slots_x4 as size_t,
                );
                memset(
                    p_qmf_residual_imag_post as *mut core::ffi::c_void,
                    0 as core::ffi::c_int,
                    time_slots_x4 as size_t,
                );
                p_qmf_residual_real_post = p_qmf_residual_real_post
                    .offset(MAX_TIME_SLOTS as isize);
                p_qmf_residual_imag_post = p_qmf_residual_imag_post
                    .offset(MAX_TIME_SLOTS as isize);
                qb += 1;
            }
            rfpsf = 0 as core::ffi::c_int as WORD32;
            while rfpsf < rfpsf_max_0 {
                ixheaacd_mdct2qmf_process(
                    arbdmx_upd_qmf,
                    res_mdct_0,
                    qmf_residual_real_pre_0,
                    qmf_residual_real_post_0,
                    qmf_residual_imag_pre_0,
                    qmf_residual_imag_post_0,
                    (*pstr_mps_state)
                        .res_block_type[(offset + ch) as usize][rfpsf as usize],
                    qmf_global_offset,
                    &mut (*pstr_mps_state).ia_mps_dec_mps_table,
                    scratch,
                    time_slots,
                );
                qmf_global_offset += arbdmx_upd_qmf;
                res_mdct_0 = res_mdct_0.offset(MDCTCOEFX2 as isize);
                rfpsf += 1;
            }
            qmf_residual_real_pre_0 = qmf_residual_real_pre_0.offset(QBXTS as isize);
            qmf_residual_imag_pre_0 = qmf_residual_imag_pre_0.offset(QBXTS as isize);
            qmf_residual_imag_post_0 = qmf_residual_imag_post_0.offset(QBXTS as isize);
            qmf_residual_real_post_0 = qmf_residual_real_post_0.offset(QBXTS as isize);
            p_res_mdct_0 = p_res_mdct_0.offset(RFX2XMDCTCOEF as isize);
            ch += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hybrid_qmf_analysis(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
) -> VOID {
    let mut ch: WORD32 = 0;
    let mut in_ch: WORD32 = (*pstr_mps_state).num_input_channels;
    let mut num_ott_boxes: WORD32 = (*pstr_mps_state).num_ott_boxes;
    let mut num_ttt_boxes: WORD32 = (*pstr_mps_state).num_ttt_boxes;
    let mut num_input_channels: WORD32 = in_ch;
    let mut qmf_bands: WORD32 = (*pstr_mps_state).qmf_bands;
    let mut time_slots: WORD32 = (*pstr_mps_state).time_slots;
    let mut hybrid_bands: WORD32 = (*pstr_mps_state).hybrid_bands;
    let mut num_parameter_bands: WORD32 = (*pstr_mps_state).num_parameter_bands;
    let mut kernels: *mut size_t = ((*pstr_mps_state).kernels).as_mut_ptr();
    let mut res_bands: *mut WORD32 = ((*pstr_mps_state).res_bands).as_mut_ptr();
    let mut index: *mut WORD32 = ((*pstr_mps_state).index).as_mut_ptr();
    let mut hyb_filter_state: *mut ia_mps_dec_thyb_filter_state_struct = (*pstr_mps_state)
        .mps_persistent_mem
        .hyb_filter_state;
    let mut p_array_struct: *mut ia_mps_dec_reuse_array_struct = (*pstr_mps_state)
        .array_struct;
    let mut hybrid_table_ptr: *mut ia_mps_dec_hybrid_tables_struct = (*pstr_mps_state)
        .ia_mps_dec_mps_table
        .hybrid_table_ptr;
    let mut p_buf_real: *mut WORD32 = (*p_array_struct).buf_real;
    let mut p_buf_imag: *mut WORD32 = (*p_array_struct).buf_imag;
    let mut p_x_real: *mut WORD32 = (*p_array_struct).x_real;
    let mut p_x_imag: *mut WORD32 = (*p_array_struct).x_imag;
    ch = 0 as core::ffi::c_int as WORD32;
    while ch < in_ch {
        ixheaacd_apply_ana_hyb_filt_bank_create_x(
            &mut *hyb_filter_state.offset(ch as isize),
            p_buf_real,
            p_buf_imag,
            qmf_bands,
            time_slots,
            p_x_real,
            p_x_imag,
            hybrid_table_ptr,
        );
        (*pstr_mps_state).index[ch as usize] = hybrid_bands;
        p_buf_real = p_buf_real.offset(TSXHB as isize);
        p_buf_imag = p_buf_imag.offset(TSXHB as isize);
        p_x_real = p_x_real.offset(TSXHB as isize);
        p_x_imag = p_x_imag.offset(TSXHB as isize);
        ch += 1;
    }
    if (*pstr_mps_state).residual_coding != 0
        && (*pstr_mps_state).up_mix_type != 2 as core::ffi::c_int
    {
        let mut qmf_residual_real: *mut WORD32 = (*p_array_struct).qmf_residual_real_pre;
        let mut qmf_residual_imag: *mut WORD32 = (*p_array_struct).qmf_residual_imag_pre;
        let mut p_dry_real: *mut WORD32 = (*p_array_struct).w_dry_real;
        let mut p_dry_imag: *mut WORD32 = (*p_array_struct).w_dry_imag;
        ch = 0 as core::ffi::c_int as WORD32;
        while ch < num_ott_boxes {
            if *res_bands.offset(ch as isize) > 0 as core::ffi::c_int {
                ixheaacd_apply_ana_hyb_filt_bank_merge_res_decor(
                    &mut *hyb_filter_state.offset((ch + num_input_channels) as isize),
                    qmf_residual_real,
                    qmf_residual_imag,
                    qmf_bands,
                    time_slots,
                    p_dry_real,
                    p_dry_imag,
                    hybrid_table_ptr,
                );
            }
            qmf_residual_real = qmf_residual_real.offset(QBXTS as isize);
            qmf_residual_imag = qmf_residual_imag.offset(QBXTS as isize);
            p_dry_real = p_dry_real.offset(TSXHB as isize);
            p_dry_imag = p_dry_imag.offset(TSXHB as isize);
            ch += 1;
        }
        ch = num_ott_boxes;
        while ch < num_ott_boxes + num_ttt_boxes {
            if *res_bands.offset(ch as isize) > 0 as core::ffi::c_int {
                ixheaacd_apply_ana_hyb_filt_bank_create_x_res(
                    &mut *hyb_filter_state.offset((ch + num_input_channels) as isize),
                    qmf_residual_real,
                    qmf_residual_imag,
                    qmf_bands,
                    time_slots,
                    p_x_real,
                    p_x_imag,
                    kernels,
                    *res_bands.offset(ch as isize),
                    hybrid_bands,
                    num_parameter_bands,
                    &mut *index.offset(in_ch as isize),
                    hybrid_table_ptr,
                );
            } else {
                *index.offset(in_ch as isize) = 0 as core::ffi::c_int as WORD32;
            }
            qmf_residual_real = qmf_residual_real.offset(QBXTS as isize);
            qmf_residual_imag = qmf_residual_imag.offset(QBXTS as isize);
            p_x_real = p_x_real.offset(TSXHB as isize);
            p_x_imag = p_x_imag.offset(TSXHB as isize);
            ch += 1;
            in_ch += 1;
        }
    }
    in_ch = num_input_channels + num_ttt_boxes;
    if (*pstr_mps_state).arbitrary_downmix == 2 as core::ffi::c_int {
        let mut offset: WORD32 = num_ott_boxes + num_ttt_boxes;
        let mut qmf_residual_real_0: *mut WORD32 = ((*p_array_struct)
            .qmf_residual_real_pre)
            .offset((offset as core::ffi::c_int * QBXTS) as isize);
        let mut qmf_residual_imag_0: *mut WORD32 = ((*p_array_struct)
            .qmf_residual_imag_pre)
            .offset((offset as core::ffi::c_int * QBXTS) as isize);
        p_x_real = ((*p_array_struct).x_real)
            .offset((in_ch as core::ffi::c_int * TSXHB) as isize);
        p_x_imag = ((*p_array_struct).x_imag)
            .offset((in_ch as core::ffi::c_int * TSXHB) as isize);
        ch = 0 as core::ffi::c_int as WORD32;
        while ch < num_input_channels {
            ixheaacd_apply_ana_hyb_filt_bank_create_x_res(
                &mut *hyb_filter_state
                    .offset((offset + ch + num_input_channels) as isize),
                qmf_residual_real_0,
                qmf_residual_imag_0,
                qmf_bands,
                time_slots,
                p_x_real,
                p_x_imag,
                kernels,
                (*pstr_mps_state).arbdmx_residual_bands,
                hybrid_bands,
                num_parameter_bands,
                &mut *index.offset(in_ch as isize),
                hybrid_table_ptr,
            );
            qmf_residual_real_0 = qmf_residual_real_0.offset(QBXTS as isize);
            qmf_residual_imag_0 = qmf_residual_imag_0.offset(QBXTS as isize);
            p_x_real = p_x_real.offset(TSXHB as isize);
            p_x_imag = p_x_imag.offset(TSXHB as isize);
            ch += 1;
            in_ch += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_merge_res_decor(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
) -> VOID {
    let mut ts: WORD32 = 0;
    let mut qs: WORD32 = 0;
    let mut row: WORD32 = 0;
    let mut res: WORD32 = 0;
    let mut temp_1: WORD32 = 0;
    let mut idx: *mut size_t = 0 as *mut size_t;
    let mut p_array_struct: *mut ia_mps_dec_reuse_array_struct = (*pstr_mps_state)
        .array_struct;
    let mut p_aux_struct: *mut ia_mps_dec_auxilary_struct = (*pstr_mps_state).aux_struct;
    let mut time_slots: WORD32 = (*pstr_mps_state).time_slots;
    let mut hybrid_bands: WORD32 = (*pstr_mps_state).hybrid_bands;
    let mut num_direct_signals: WORD32 = (*pstr_mps_state).num_direct_signals;
    let mut num_w_channels: WORD32 = (*pstr_mps_state).num_w_channels;
    let mut num_parameter_bands: WORD32 = (*pstr_mps_state).num_parameter_bands;
    let mut kernels_ptr: *mut size_t = ((*pstr_mps_state).kernels).as_mut_ptr();
    let mut p_buf_real: *mut WORD32 = 0 as *mut WORD32;
    let mut p_buf_imag: *mut WORD32 = 0 as *mut WORD32;
    let mut p_buf_re: *mut WORD32 = 0 as *mut WORD32;
    let mut p_buf_im: *mut WORD32 = 0 as *mut WORD32;
    let mut buf_real_ch4: *mut WORD32 = 0 as *mut WORD32;
    let mut buf_imag_ch4: *mut WORD32 = 0 as *mut WORD32;
    let mut buf_real_ch3: *mut WORD32 = 0 as *mut WORD32;
    let mut buf_imag_ch3: *mut WORD32 = 0 as *mut WORD32;
    p_buf_real = ((*p_array_struct).buffer_real).offset(TSXHBX5 as isize);
    p_buf_imag = ((*p_array_struct).buffer_imag).offset(TSXHBX5 as isize);
    ts = 0 as core::ffi::c_int as WORD32;
    while ts < time_slots {
        p_buf_re = p_buf_real;
        p_buf_im = p_buf_imag;
        buf_real_ch4 = p_buf_real.offset(-(TSXHB as isize));
        buf_imag_ch4 = p_buf_imag.offset(-(TSXHB as isize));
        buf_real_ch3 = buf_real_ch4.offset(-(TSXHB as isize));
        buf_imag_ch3 = buf_imag_ch4.offset(-(TSXHB as isize));
        qs = 0 as core::ffi::c_int as WORD32;
        while qs < hybrid_bands {
            if *kernels_ptr.offset(qs as isize)
                < (*p_aux_struct)
                    .ttt_config[0 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize]
                    .stop_band as UWORD32 as size_t
                && (*p_aux_struct)
                    .ttt_config[0 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize]
                    .use_ttt_decorr != 0
                || *kernels_ptr.offset(qs as isize)
                    >= (*p_aux_struct)
                        .ttt_config[1 as core::ffi::c_int
                            as usize][0 as core::ffi::c_int as usize]
                        .start_band as UWORD32 as size_t
                    && (*p_aux_struct)
                        .ttt_config[1 as core::ffi::c_int
                            as usize][0 as core::ffi::c_int as usize]
                        .use_ttt_decorr != 0
            {
                temp_1 = ONE_BY_SQRT_TWO_Q30;
                *p_buf_re = ixheaacd_mps_mult32_shr_30(*p_buf_re, temp_1);
                *p_buf_re += *buf_real_ch3 + *buf_real_ch4;
                *p_buf_im = ixheaacd_mps_mult32_shr_30(*p_buf_im, temp_1);
                *p_buf_im += *buf_imag_ch3 + *buf_imag_ch4;
            }
            p_buf_re = p_buf_re.offset(1);
            p_buf_im = p_buf_im.offset(1);
            buf_real_ch4 = buf_real_ch4.offset(1);
            buf_imag_ch4 = buf_imag_ch4.offset(1);
            buf_real_ch3 = buf_real_ch3.offset(1);
            buf_imag_ch3 = buf_imag_ch3.offset(1);
            qs += 1;
        }
        p_buf_real = p_buf_real.offset(MAX_HYBRID_BANDS as isize);
        p_buf_imag = p_buf_imag.offset(MAX_HYBRID_BANDS as isize);
        ts += 1;
    }
    if (*pstr_mps_state).residual_coding != 0 {
        row = num_direct_signals;
        while row < num_w_channels {
            let mut resband: WORD32 = 0;
            res = ixheaacd_get_res_idx(pstr_mps_state, row);
            resband = (*pstr_mps_state).res_bands[res as usize];
            if resband == 1 as core::ffi::c_int
                && (num_parameter_bands == 20 as core::ffi::c_int
                    || num_parameter_bands == 28 as core::ffi::c_int)
            {
                (*pstr_mps_state).index[res as usize] = 3 as core::ffi::c_int as WORD32;
            } else {
                idx = &mut *kernels_ptr.offset(0 as core::ffi::c_int as isize)
                    as *mut size_t;
                qs = 0 as core::ffi::c_int as WORD32;
                while qs < hybrid_bands {
                    let fresh0 = idx;
                    idx = idx.offset(1);
                    if *fresh0 >= resband as size_t {
                        (*pstr_mps_state).index[res as usize] = qs;
                        qs = hybrid_bands;
                    }
                    qs += 1;
                }
            }
            row += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_create_w(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
) -> VOID {
    let mut k: WORD32 = 0;
    let mut p_array_struct: *mut ia_mps_dec_reuse_array_struct = (*pstr_mps_state)
        .array_struct;
    let mut num_direct_signals: WORD32 = (*pstr_mps_state).num_direct_signals;
    let mut counter: WORD32 = num_direct_signals + (*pstr_mps_state).num_decor_signals;
    let mut time_slots: WORD32 = (*pstr_mps_state).time_slots;
    let mut offset: WORD32 = num_direct_signals * TSXHB;
    let mut p_buffer_real: *mut WORD32 = ((*p_array_struct).buf_real)
        .offset(offset as isize);
    let mut p_buffer_imag: *mut WORD32 = ((*p_array_struct).buf_imag)
        .offset(offset as isize);
    let mut p_buf_real: *mut WORD32 = ((*p_array_struct).buffer_real)
        .offset(offset as isize);
    let mut p_buf_imag: *mut WORD32 = ((*p_array_struct).buffer_imag)
        .offset(offset as isize);
    k = num_direct_signals;
    while k < counter {
        ixheaacd_decorr_apply(
            pstr_mps_state,
            time_slots,
            p_buffer_real,
            p_buffer_imag,
            p_buf_real,
            p_buf_imag,
            k,
        );
        p_buffer_real = p_buffer_real.offset(TSXHB as isize);
        p_buffer_imag = p_buffer_imag.offset(TSXHB as isize);
        p_buf_real = p_buf_real.offset(TSXHB as isize);
        p_buf_imag = p_buf_imag.offset(TSXHB as isize);
        k += 1;
    }
    ixheaacd_merge_res_decor(pstr_mps_state);
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_update_buffers(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
) -> VOID {
    let mut p_array_struct: *mut ia_mps_dec_reuse_array_struct = (*pstr_mps_state)
        .array_struct;
    let mut temp_addr: *mut WORD32 = (*p_array_struct).qmf_residual_real_post;
    (*p_array_struct).qmf_residual_real_post = (*p_array_struct).qmf_residual_real_pre;
    (*p_array_struct).qmf_residual_real_pre = temp_addr;
    temp_addr = (*p_array_struct).qmf_residual_imag_post;
    (*p_array_struct).qmf_residual_imag_post = (*p_array_struct).qmf_residual_imag_pre;
    (*p_array_struct).qmf_residual_imag_pre = temp_addr;
    (*p_array_struct).buffer_real = (*p_array_struct).qmf_residual_real_post;
    (*p_array_struct).buffer_imag = (*p_array_struct).qmf_residual_imag_post;
    (*p_array_struct).m1_param = (*p_array_struct).buffer_real
        as *mut ia_mps_dec_m1_param_struct;
}
