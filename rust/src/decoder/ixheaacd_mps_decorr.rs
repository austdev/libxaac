extern "C" {
    fn sqrt(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    static ixheaacd_mps_gain_set_indx: [WORD32; 29];
    static mut ixheaacd_hybrid_band_71_to_processing_band_28_map: [WORD32; 71];
    static mut ixheaacd_hybrid_band_64_to_processing_band_23_map: [WORD32; 71];
    fn ixheaacd_get_qmf_sb(
        hybrid_subband: WORD32,
        ixheaacd_mps_dec_mdct2qmf_table: *const ia_mps_dec_mdct2qmf_table_struct,
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
pub type VOID = ();
pub type FLAG = core::ffi::c_int;
pub type LOOPIDX = core::ffi::c_int;
pub type WORD = core::ffi::c_int;
pub type LOOPINDEX = LOOPIDX;
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
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const REVERB_BAND_3: C2RustUnnamed_0 = 3;
pub const REVERB_BAND_2: C2RustUnnamed_0 = 2;
pub const REVERB_BAND_1: C2RustUnnamed_0 = 1;
pub const REVERB_BAND_0: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = core::ffi::c_uint;
pub const DECOR_CONFIG_2: C2RustUnnamed_1 = 2;
pub const DECOR_CONFIG_1: C2RustUnnamed_1 = 1;
pub const DECOR_CONFIG_0: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_duck_instance_struct {
    pub hybrid_bands: WORD32,
    pub parameter_bands: WORD32,
    pub alpha: WORD32,
    pub one_minus_alpha: WORD32,
    pub gamma: WORD32,
    pub abs_thr: WORD32,
    pub qalpha: WORD16,
    pub qgamma: WORD16,
    pub smooth_direct_nrg: [WORD32; 28],
    pub smooth_reverb_nrg: [WORD32; 28],
    pub q_smooth_direct_nrg: [WORD16; 28],
    pub q_smooth_reverb_nrg: [WORD16; 28],
}
pub const INV_SQRT_2_Q31: core::ffi::c_int = 1518500250 as core::ffi::c_int;
pub const Q_SQRT_TAB: core::ffi::c_int = 15 as core::ffi::c_int;
pub const ONE_BY_FIVE_Q16: core::ffi::c_int = 13107 as core::ffi::c_int;
pub const TWO_PI_IN_Q15: core::ffi::c_int = 205887 as core::ffi::c_int;
pub const ONE_IN_Q14: core::ffi::c_int = 16384 as core::ffi::c_int;
pub const ONE_IN_Q15: core::ffi::c_int = 32768 as core::ffi::c_int;
pub const MAX_PARAMETER_BANDS: core::ffi::c_int = 28 as core::ffi::c_int;
pub const TRIG_TABLE_CONV_FAC: core::ffi::c_int = 326 as core::ffi::c_int;
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
pub const DECOR_ALPHA: core::ffi::c_float = 0.8f32;
pub const ABS_THR: core::ffi::c_float = 1e-9f32
    * 32768 as core::ffi::c_int as core::ffi::c_float
    * 32768 as core::ffi::c_int as core::ffi::c_float;
pub const MAX_NUM_QMF_BANDS_MPS_NEW: core::ffi::c_int = 64 as core::ffi::c_int;
pub const DECORR_FILTER_ORDER_BAND_0: core::ffi::c_int = 20 as core::ffi::c_int;
pub const DECORR_FILTER_ORDER_BAND_1: core::ffi::c_int = 15 as core::ffi::c_int;
pub const DECORR_FILTER_ORDER_BAND_2: core::ffi::c_int = 6 as core::ffi::c_int;
pub const DECORR_FILTER_ORDER_BAND_3: core::ffi::c_int = 3 as core::ffi::c_int;
pub const MAX_DECORR_FILTER_ORDER: core::ffi::c_int = 20 as core::ffi::c_int;
pub const DECORR_FILT_0_ORD: core::ffi::c_int = 10 as core::ffi::c_int;
pub const DECORR_FILT_1_ORD: core::ffi::c_int = 8 as core::ffi::c_int;
pub const DECORR_FILT_2_ORD: core::ffi::c_int = 3 as core::ffi::c_int;
pub const DECORR_FILT_3_ORD: core::ffi::c_int = 2 as core::ffi::c_int;
pub const NO_RES_BANDS: core::ffi::c_int = -(1 as core::ffi::c_int);
pub const MAX_HYBRID_BANDS_MPS: core::ffi::c_int = MAX_NUM_QMF_BANDS_MPS_NEW
    - 3 as core::ffi::c_int + 10 as core::ffi::c_int;
pub const ABS_THR_FIX: core::ffi::c_int = 35184 as core::ffi::c_int;
pub const MAX_NUM_QMF_BANDS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const MAX_HYBRID_BANDS: core::ffi::c_int = MAX_NUM_QMF_BANDS - 3 as core::ffi::c_int
    + 10 as core::ffi::c_int;
pub const ONE_MINUS_DECOR_ALPHA: core::ffi::c_float = 1 as core::ffi::c_int
    as core::ffi::c_float - DECOR_ALPHA;
pub const DECOR_GAMMA: core::ffi::c_float = 1.5f32;
pub const DUCK_ALPHA: core::ffi::c_int = 26214 as core::ffi::c_int;
pub const DUCK_GAMMA: core::ffi::c_int = 24576 as core::ffi::c_int;
pub const DUCK_ONEMINUSALPHA: core::ffi::c_int = 6554 as core::ffi::c_int;
static mut ixheaacd_decorr_delay: [WORD32; 4] = [
    11 as core::ffi::c_int,
    10 as core::ffi::c_int,
    5 as core::ffi::c_int,
    2 as core::ffi::c_int,
];
static mut ixheaacd_decorr_delay_ldmps: [WORD32; 4] = [
    8 as core::ffi::c_int,
    7 as core::ffi::c_int,
    2 as core::ffi::c_int,
    1 as core::ffi::c_int,
];
static mut ixheaacd_qmf_split_freq_0: [WORD32; 4] = [
    3 as core::ffi::c_int,
    15 as core::ffi::c_int,
    24 as core::ffi::c_int,
    65 as core::ffi::c_int,
];
static mut ixheaacd_qmf_split_freq_1: [WORD32; 4] = [
    3 as core::ffi::c_int,
    50 as core::ffi::c_int,
    65 as core::ffi::c_int,
    65 as core::ffi::c_int,
];
static mut ixheaacd_qmf_split_freq_2: [WORD32; 4] = [
    0 as core::ffi::c_int,
    15 as core::ffi::c_int,
    65 as core::ffi::c_int,
    65 as core::ffi::c_int,
];
static mut ixheaacd_qmf_split_freq_0_ldmps: [WORD32; 4] = [
    0 as core::ffi::c_int,
    15 as core::ffi::c_int,
    24 as core::ffi::c_int,
    65 as core::ffi::c_int,
];
static mut ixheaacd_qmf_split_freq_1_ldmps: [WORD32; 4] = [
    0 as core::ffi::c_int,
    50 as core::ffi::c_int,
    65 as core::ffi::c_int,
    65 as core::ffi::c_int,
];
static mut ixheaacd_qmf_split_freq_2_ldmps: [WORD32; 4] = [
    0 as core::ffi::c_int,
    15 as core::ffi::c_int,
    65 as core::ffi::c_int,
    65 as core::ffi::c_int,
];
static mut ixheaacd_lattice_coeff_0_filt_den_coeff: [FLOAT32; 11] = [
    1.000000f32,
    -0.314818f32,
    -0.256828f32,
    -0.173641f32,
    -0.115077f32,
    0.000599f32,
    0.033343f32,
    0.122672f32,
    -0.356362f32,
    0.128058f32,
    0.089800f32,
];
static mut ixheaacd_lattice_coeff_0_filt_num_coeff: [FLOAT32; 11] = [
    0.089800f32,
    0.128058f32,
    -0.356362f32,
    0.122672f32,
    0.033343f32,
    0.000599f32,
    -0.115077f32,
    -0.173641f32,
    -0.256828f32,
    -0.314818f32,
    1.000000f32,
];
static mut ixheaacd_lattice_coeff_1_filt_den_coeff: [FLOAT32; 9] = [
    1.000000f32,
    -0.287137f32,
    -0.088940f32,
    0.123204f32,
    -0.126111f32,
    0.064218f32,
    0.045768f32,
    -0.016264f32,
    -0.122100f32,
];
static mut ixheaacd_lattice_coeff_1_filt_num_coeff: [FLOAT32; 9] = [
    -0.122100f32,
    -0.016264f32,
    0.045768f32,
    0.064218f32,
    -0.126111f32,
    0.123204f32,
    -0.088940f32,
    -0.287137f32,
    1.000000f32,
];
static mut ixheaacd_lattice_coeff_2_filt_den_coeff: [FLOAT32; 4] = [
    1.000000f32,
    0.129403f32,
    -0.032633f32,
    0.035700f32,
];
static mut ixheaacd_lattice_coeff_2_filt_num_coeff: [FLOAT32; 4] = [
    0.035700f32,
    -0.032633f32,
    0.129403f32,
    1.000000f32,
];
static mut ixheaacd_lattice_coeff_3_filt_den_coeff: [FLOAT32; 3] = [
    1.000000f32,
    0.034742f32,
    -0.013000f32,
];
static mut ixheaacd_lattice_coeff_3_filt_num_coeff: [FLOAT32; 3] = [
    -0.013000f32,
    0.034742f32,
    1.000000f32,
];
static mut ixheaacd_lattice_coeff_1_filt_num_ldmps: [FLOAT32; 16] = [
    0.3355999887f32,
    0.0024894588f32,
    -0.1572290659f32,
    0.2807503343f32,
    -0.1942857355f32,
    0.3840600252f32,
    -0.4084388912f32,
    -0.1750483066f32,
    0.5559588671f32,
    -0.4935829639f32,
    0.0567415841f32,
    -0.0658148378f32,
    0.3378961682f32,
    0.2284426540f32,
    -0.7025330663f32,
    1.0000000000f32,
];
static mut ixheaacd_lattice_coeff_1_filt_den_ldmps: [FLOAT32; 16] = [
    1.0000000000f32,
    -0.7025330663f32,
    0.2284426540f32,
    0.3378961682f32,
    -0.0658148378f32,
    0.0567415841f32,
    -0.4935829639f32,
    0.5559588671f32,
    -0.1750483066f32,
    -0.4084388912f32,
    0.3840600252f32,
    -0.1942857355f32,
    0.2807503343f32,
    -0.1572290659f32,
    0.0024894588f32,
    0.3355999887f32,
];
static mut ixheaacd_lattice_coeff_2_filt_num_ldmps: [FLOAT32; 7] = [
    -0.4623999894f32,
    0.2341193259f32,
    0.5163637400f32,
    -0.0253488291f32,
    -0.2871030867f32,
    0.0153170601f32,
    1.0000000000f32,
];
static mut ixheaacd_lattice_coeff_2_filt_den_ldmps: [FLOAT32; 7] = [
    1.0000000000f32,
    0.0153170601f32,
    -0.2871030867f32,
    -0.0253488291f32,
    0.5163637400f32,
    0.2341193259f32,
    -0.4623999894f32,
];
static mut ixheaacd_lattice_coeff_3_filt_num_ldmps: [FLOAT32; 4] = [
    0.2468000054f32,
    0.0207958221f32,
    -0.3898491263f32,
    1.0000000000f32,
];
static mut ixheaacd_lattice_coeff_3_filt_den_ldmps: [FLOAT32; 4] = [
    1.0000000000f32,
    -0.3898491263f32,
    0.0207958221f32,
    0.2468000054f32,
];
static mut ixheaacd_hybrid_to_qmf_map: [WORD32; 71] = [
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    4 as core::ffi::c_int,
    5 as core::ffi::c_int,
    6 as core::ffi::c_int,
    7 as core::ffi::c_int,
    8 as core::ffi::c_int,
    9 as core::ffi::c_int,
    10 as core::ffi::c_int,
    11 as core::ffi::c_int,
    12 as core::ffi::c_int,
    13 as core::ffi::c_int,
    14 as core::ffi::c_int,
    15 as core::ffi::c_int,
    16 as core::ffi::c_int,
    17 as core::ffi::c_int,
    18 as core::ffi::c_int,
    19 as core::ffi::c_int,
    20 as core::ffi::c_int,
    21 as core::ffi::c_int,
    22 as core::ffi::c_int,
    23 as core::ffi::c_int,
    24 as core::ffi::c_int,
    25 as core::ffi::c_int,
    26 as core::ffi::c_int,
    27 as core::ffi::c_int,
    28 as core::ffi::c_int,
    29 as core::ffi::c_int,
    30 as core::ffi::c_int,
    31 as core::ffi::c_int,
    32 as core::ffi::c_int,
    33 as core::ffi::c_int,
    34 as core::ffi::c_int,
    35 as core::ffi::c_int,
    36 as core::ffi::c_int,
    37 as core::ffi::c_int,
    38 as core::ffi::c_int,
    39 as core::ffi::c_int,
    40 as core::ffi::c_int,
    41 as core::ffi::c_int,
    42 as core::ffi::c_int,
    43 as core::ffi::c_int,
    44 as core::ffi::c_int,
    45 as core::ffi::c_int,
    46 as core::ffi::c_int,
    47 as core::ffi::c_int,
    48 as core::ffi::c_int,
    49 as core::ffi::c_int,
    50 as core::ffi::c_int,
    51 as core::ffi::c_int,
    52 as core::ffi::c_int,
    53 as core::ffi::c_int,
    54 as core::ffi::c_int,
    55 as core::ffi::c_int,
    56 as core::ffi::c_int,
    57 as core::ffi::c_int,
    58 as core::ffi::c_int,
    59 as core::ffi::c_int,
    60 as core::ffi::c_int,
    61 as core::ffi::c_int,
    62 as core::ffi::c_int,
    63 as core::ffi::c_int,
];
static mut ixheaacd_hybrid_to_qmf_map_ldmps: [WORD32; 71] = [
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    4 as core::ffi::c_int,
    5 as core::ffi::c_int,
    6 as core::ffi::c_int,
    7 as core::ffi::c_int,
    8 as core::ffi::c_int,
    9 as core::ffi::c_int,
    10 as core::ffi::c_int,
    11 as core::ffi::c_int,
    12 as core::ffi::c_int,
    13 as core::ffi::c_int,
    14 as core::ffi::c_int,
    15 as core::ffi::c_int,
    16 as core::ffi::c_int,
    17 as core::ffi::c_int,
    18 as core::ffi::c_int,
    19 as core::ffi::c_int,
    20 as core::ffi::c_int,
    21 as core::ffi::c_int,
    22 as core::ffi::c_int,
    23 as core::ffi::c_int,
    24 as core::ffi::c_int,
    25 as core::ffi::c_int,
    26 as core::ffi::c_int,
    27 as core::ffi::c_int,
    28 as core::ffi::c_int,
    29 as core::ffi::c_int,
    30 as core::ffi::c_int,
    31 as core::ffi::c_int,
    32 as core::ffi::c_int,
    33 as core::ffi::c_int,
    34 as core::ffi::c_int,
    35 as core::ffi::c_int,
    36 as core::ffi::c_int,
    37 as core::ffi::c_int,
    38 as core::ffi::c_int,
    39 as core::ffi::c_int,
    40 as core::ffi::c_int,
    41 as core::ffi::c_int,
    42 as core::ffi::c_int,
    43 as core::ffi::c_int,
    44 as core::ffi::c_int,
    45 as core::ffi::c_int,
    46 as core::ffi::c_int,
    47 as core::ffi::c_int,
    48 as core::ffi::c_int,
    49 as core::ffi::c_int,
    50 as core::ffi::c_int,
    51 as core::ffi::c_int,
    52 as core::ffi::c_int,
    53 as core::ffi::c_int,
    54 as core::ffi::c_int,
    55 as core::ffi::c_int,
    56 as core::ffi::c_int,
    57 as core::ffi::c_int,
    58 as core::ffi::c_int,
    59 as core::ffi::c_int,
    60 as core::ffi::c_int,
    61 as core::ffi::c_int,
    62 as core::ffi::c_int,
    63 as core::ffi::c_int,
    64 as core::ffi::c_int,
    65 as core::ffi::c_int,
    66 as core::ffi::c_int,
    67 as core::ffi::c_int,
    68 as core::ffi::c_int,
    69 as core::ffi::c_int,
    70 as core::ffi::c_int,
];
unsafe extern "C" fn ixheaacd_mps_decor_filt_init(
    mut self_0: *mut ia_mps_decor_filt_struct,
    mut reverb_band: WORD32,
    mut object_type: WORD32,
) -> VOID {
    if object_type == AOT_ER_AAC_ELD as core::ffi::c_int
        || object_type == AOT_ER_AAC_LD as core::ffi::c_int
    {
        match reverb_band {
            0 => {
                (*self_0).den_len = (DECORR_FILTER_ORDER_BAND_0 + 1 as core::ffi::c_int)
                    as WORD32;
                (*self_0).num_len = (*self_0).den_len;
                (*self_0).num = 0 as *const FLOAT32;
                (*self_0).den = 0 as *const FLOAT32;
            }
            1 => {
                (*self_0).den_len = (DECORR_FILTER_ORDER_BAND_1 + 1 as core::ffi::c_int)
                    as WORD32;
                (*self_0).num_len = (*self_0).den_len;
                (*self_0).num = ixheaacd_lattice_coeff_1_filt_num_ldmps.as_ptr();
                (*self_0).den = ixheaacd_lattice_coeff_1_filt_den_ldmps.as_ptr();
            }
            2 => {
                (*self_0).den_len = (DECORR_FILTER_ORDER_BAND_2 + 1 as core::ffi::c_int)
                    as WORD32;
                (*self_0).num_len = (*self_0).den_len;
                (*self_0).num = ixheaacd_lattice_coeff_2_filt_num_ldmps.as_ptr();
                (*self_0).den = ixheaacd_lattice_coeff_2_filt_den_ldmps.as_ptr();
            }
            3 => {
                (*self_0).den_len = (DECORR_FILTER_ORDER_BAND_3 + 1 as core::ffi::c_int)
                    as WORD32;
                (*self_0).num_len = (*self_0).den_len;
                (*self_0).num = ixheaacd_lattice_coeff_3_filt_num_ldmps.as_ptr();
                (*self_0).den = ixheaacd_lattice_coeff_3_filt_den_ldmps.as_ptr();
            }
            _ => {}
        }
    } else {
        match reverb_band {
            0 => {
                (*self_0).den_len = (DECORR_FILT_0_ORD + 1 as core::ffi::c_int)
                    as WORD32;
                (*self_0).num_len = (*self_0).den_len;
                (*self_0).num = ixheaacd_lattice_coeff_0_filt_num_coeff.as_ptr();
                (*self_0).den = ixheaacd_lattice_coeff_0_filt_den_coeff.as_ptr();
            }
            1 => {
                (*self_0).den_len = (DECORR_FILT_1_ORD + 1 as core::ffi::c_int)
                    as WORD32;
                (*self_0).num_len = (*self_0).den_len;
                (*self_0).num = ixheaacd_lattice_coeff_1_filt_num_coeff.as_ptr();
                (*self_0).den = ixheaacd_lattice_coeff_1_filt_den_coeff.as_ptr();
            }
            2 => {
                (*self_0).den_len = (DECORR_FILT_2_ORD + 1 as core::ffi::c_int)
                    as WORD32;
                (*self_0).num_len = (*self_0).den_len;
                (*self_0).num = ixheaacd_lattice_coeff_2_filt_num_coeff.as_ptr();
                (*self_0).den = ixheaacd_lattice_coeff_2_filt_den_coeff.as_ptr();
            }
            3 => {
                (*self_0).den_len = (DECORR_FILT_3_ORD + 1 as core::ffi::c_int)
                    as WORD32;
                (*self_0).num_len = (*self_0).den_len;
                (*self_0).num = ixheaacd_lattice_coeff_3_filt_num_coeff.as_ptr();
                (*self_0).den = ixheaacd_lattice_coeff_3_filt_den_coeff.as_ptr();
            }
            _ => {}
        }
    }
    (*self_0).state_len = (*self_0).num_len;
    memset(
        ((*self_0).state).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<ia_cmplx_flt_struct>() as size_t)
            .wrapping_mul((MAX_DECORR_FILTER_ORDER + 1 as core::ffi::c_int) as size_t),
    );
}
unsafe extern "C" fn ixheaacd_mps_allpass_apply(
    mut self_0: *mut ia_mps_decor_filt_struct,
    mut input: *mut ia_cmplx_flt_struct,
    mut len: WORD32,
    mut output: *mut ia_cmplx_flt_struct,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < len {
        (*output.offset(i as isize)).re = (*self_0)
            .state[0 as core::ffi::c_int as usize]
            .re
            + (*input.offset(i as isize)).re
                * *((*self_0).num).offset(0 as core::ffi::c_int as isize);
        (*output.offset(i as isize)).im = (*self_0)
            .state[0 as core::ffi::c_int as usize]
            .im
            + (*input.offset(i as isize)).im
                * *((*self_0).num).offset(0 as core::ffi::c_int as isize);
        j = 1 as core::ffi::c_int as WORD32;
        while j < (*self_0).num_len {
            (*self_0)
                .state[(j as core::ffi::c_int - 1 as core::ffi::c_int) as usize]
                .re = (*self_0).state[j as usize].re
                + *((*self_0).num).offset(j as isize) * (*input.offset(i as isize)).re
                - *((*self_0).den).offset(j as isize) * (*output.offset(i as isize)).re;
            (*self_0)
                .state[(j as core::ffi::c_int - 1 as core::ffi::c_int) as usize]
                .im = (*self_0).state[j as usize].im
                + *((*self_0).num).offset(j as isize) * (*input.offset(i as isize)).im
                - *((*self_0).den).offset(j as isize) * (*output.offset(i as isize)).im;
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_mps_decor_energy_adjustment(
    mut handle: *mut ixheaacd_mps_decor_energy_adjust_filt_struct,
    mut in_0: *mut [ia_cmplx_flt_struct; 71],
    mut out: *mut [ia_cmplx_flt_struct; 71],
    mut time_slots: WORD32,
    mut res_bands: WORD32,
    mut ldmps_present: WORD32,
) -> VOID {
    let mut self_0: *mut ixheaacd_mps_decor_energy_adjust_filt_struct = handle;
    let mut in_energy: [FLOAT32; 28] = [
        0 as core::ffi::c_int as FLOAT32,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ];
    let mut out_energy: [FLOAT32; 28] = [
        0 as core::ffi::c_int as FLOAT32,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ];
    let mut gain: [FLOAT32; 28] = [0.; 28];
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut loop_counter: WORD32 = 0;
    let mut ptr_hybrid_band: *mut WORD32 = 0 as *mut WORD32;
    if ldmps_present == 1 as core::ffi::c_int {
        ptr_hybrid_band = ixheaacd_hybrid_band_64_to_processing_band_23_map.as_mut_ptr();
    } else {
        ptr_hybrid_band = ixheaacd_hybrid_band_71_to_processing_band_28_map.as_mut_ptr();
    }
    let mut start_param_band: WORD32 = 0 as WORD32;
    let mut start_bin: WORD32 = 0 as WORD32;
    if res_bands != NO_RES_BANDS {
        start_bin = ixheaacd_mps_gain_set_indx[res_bands as usize];
        start_param_band = res_bands;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < time_slots {
        memset(
            in_energy.as_mut_ptr() as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<FLOAT32>() as size_t)
                .wrapping_mul(MAX_PARAMETER_BANDS as size_t),
        );
        memset(
            out_energy.as_mut_ptr() as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<FLOAT32>() as size_t)
                .wrapping_mul(MAX_PARAMETER_BANDS as size_t),
        );
        j = start_bin;
        while j < (*self_0).num_bins {
            k = *ptr_hybrid_band.offset(j as isize);
            in_energy[k as usize]
                += (*in_0.offset(i as isize))[j as usize].re
                    * (*in_0.offset(i as isize))[j as usize].re
                    + (*in_0.offset(i as isize))[j as usize].im
                        * (*in_0.offset(i as isize))[j as usize].im;
            out_energy[k as usize]
                += (*out.offset(i as isize))[j as usize].re
                    * (*out.offset(i as isize))[j as usize].re
                    + (*out.offset(i as isize))[j as usize].im
                        * (*out.offset(i as isize))[j as usize].im;
            j += 1;
        }
        loop_counter = MAX_PARAMETER_BANDS as WORD32;
        k = start_param_band;
        while k < loop_counter {
            (*self_0).smooth_in_energy[k as usize] = ((*self_0)
                .smooth_in_energy[k as usize] * DECOR_ALPHA
                + in_energy[k as usize] * ONE_MINUS_DECOR_ALPHA) as FLOAT32;
            (*self_0).smooth_out_energy[k as usize] = ((*self_0)
                .smooth_out_energy[k as usize] * DECOR_ALPHA
                + out_energy[k as usize] * ONE_MINUS_DECOR_ALPHA) as FLOAT32;
            gain[k as usize] = 1.0f32 as FLOAT32;
            if (*self_0).smooth_out_energy[k as usize]
                > (*self_0).smooth_in_energy[k as usize] * DECOR_GAMMA
            {
                gain[k as usize] = sqrt(
                    ((*self_0).smooth_in_energy[k as usize] * DECOR_GAMMA
                        / ((*self_0).smooth_out_energy[k as usize] + ABS_THR))
                        as core::ffi::c_double,
                ) as FLOAT32;
            }
            if (*self_0).smooth_in_energy[k as usize]
                > (*self_0).smooth_out_energy[k as usize] * DECOR_GAMMA
            {
                gain[k as usize] = (if 2.0f32
                    < sqrt(
                        ((*self_0).smooth_in_energy[k as usize]
                            / (1.5f32 * (*self_0).smooth_out_energy[k as usize]
                                + 1e-9f32 * 32768 as core::ffi::c_int as core::ffi::c_float
                                    * 32768 as core::ffi::c_int as core::ffi::c_float))
                            as core::ffi::c_double,
                    ) as FLOAT32
                {
                    2.0f32
                } else {
                    sqrt(
                        ((*self_0).smooth_in_energy[k as usize]
                            / (1.5f32 * (*self_0).smooth_out_energy[k as usize]
                                + 1e-9f32 * 32768 as core::ffi::c_int as core::ffi::c_float
                                    * 32768 as core::ffi::c_int as core::ffi::c_float))
                            as core::ffi::c_double,
                    ) as core::ffi::c_float
                }) as FLOAT32;
            }
            k += 1;
        }
        j = start_bin;
        while j < (*self_0).num_bins {
            k = *ptr_hybrid_band.offset(j as isize);
            (*out.offset(i as isize))[j as usize].re *= gain[k as usize];
            (*out.offset(i as isize))[j as usize].im *= gain[k as usize];
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_decor_init(
    mut self_0: *mut ia_mps_decor_struct,
    mut subbands: WORD32,
    mut decor_config: WORD32,
    mut object_type: WORD32,
) -> IA_ERRORCODE {
    let mut i: WORD32 = 0;
    let mut reverb_band: WORD32 = 0;
    let mut splitfreq: *const WORD32 = 0 as *const WORD32;
    let mut ptr_ixheaacd_hybrid_to_qmf_map: *const WORD32 = 0 as *const WORD32;
    let mut ptr_decorr_delay: *const WORD32 = 0 as *const WORD32;
    if object_type == AOT_ER_AAC_ELD as core::ffi::c_int
        || object_type == AOT_ER_AAC_LD as core::ffi::c_int
    {
        ptr_ixheaacd_hybrid_to_qmf_map = ixheaacd_hybrid_to_qmf_map_ldmps.as_ptr();
        ptr_decorr_delay = ixheaacd_decorr_delay_ldmps.as_ptr();
        match decor_config {
            0 => {
                splitfreq = ixheaacd_qmf_split_freq_0_ldmps.as_ptr();
            }
            1 => {
                splitfreq = ixheaacd_qmf_split_freq_1_ldmps.as_ptr();
            }
            2 => {
                splitfreq = ixheaacd_qmf_split_freq_2_ldmps.as_ptr();
            }
            _ => return IA_FATAL_ERROR as IA_ERRORCODE,
        }
    } else {
        ptr_ixheaacd_hybrid_to_qmf_map = ixheaacd_hybrid_to_qmf_map.as_ptr();
        ptr_decorr_delay = ixheaacd_decorr_delay.as_ptr();
        match decor_config {
            0 => {
                splitfreq = ixheaacd_qmf_split_freq_0.as_ptr();
            }
            1 => {
                splitfreq = ixheaacd_qmf_split_freq_1.as_ptr();
            }
            2 => {
                splitfreq = ixheaacd_qmf_split_freq_2.as_ptr();
            }
            _ => return IA_FATAL_ERROR as IA_ERRORCODE,
        }
    }
    (*self_0).num_bins = subbands;
    if (*self_0).num_bins > MAX_HYBRID_BANDS_MPS {
        return IA_FATAL_ERROR as IA_ERRORCODE;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*self_0).num_bins {
        reverb_band = 0 as core::ffi::c_int as WORD32;
        while reverb_band < 3 as core::ffi::c_int
            && *ptr_ixheaacd_hybrid_to_qmf_map.offset(i as isize)
                >= *splitfreq.offset(reverb_band as isize) - 1 as core::ffi::c_int
        {
            reverb_band += 1;
        }
        (*self_0).delay_sample_count[i as usize] = *ptr_decorr_delay
            .offset(reverb_band as isize);
        ixheaacd_mps_decor_filt_init(
            &mut *((*self_0).filter).as_mut_ptr().offset(i as isize),
            reverb_band,
            object_type,
        );
        i += 1;
    }
    (*self_0).decor_nrg_smooth.num_bins = (*self_0).num_bins;
    return IA_NO_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_decor_apply(
    mut self_0: *mut ia_mps_decor_struct,
    mut in_0: *mut [ia_cmplx_flt_struct; 71],
    mut out: *mut [ia_cmplx_flt_struct; 71],
    mut length: WORD32,
    mut res_bands: WORD32,
    mut ldmps_present: WORD32,
) -> VOID {
    let mut idx: WORD32 = 0;
    let mut sb_sample: WORD32 = 0;
    let mut index: WORD32 = 0 as WORD32;
    let mut scratch: [ia_cmplx_flt_struct; 72] = [ia_cmplx_flt_struct {
        re: 0.,
        im: 0.,
    }; 72];
    if res_bands != NO_RES_BANDS {
        index = ixheaacd_mps_gain_set_indx[res_bands as usize];
    }
    idx = index;
    while idx < (*self_0).num_bins {
        sb_sample = 0 as core::ffi::c_int as WORD32;
        while sb_sample < length {
            (*self_0)
                .decor_delay_buffer[idx
                    as usize][((*self_0).delay_sample_count[idx as usize] + sb_sample)
                    as usize]
                .re = (*in_0.offset(sb_sample as isize))[idx as usize].re;
            (*self_0)
                .decor_delay_buffer[idx
                    as usize][((*self_0).delay_sample_count[idx as usize] + sb_sample)
                    as usize]
                .im = (*in_0.offset(sb_sample as isize))[idx as usize].im;
            sb_sample += 1;
        }
        ixheaacd_mps_allpass_apply(
            &mut *((*self_0).filter).as_mut_ptr().offset(idx as isize),
            ((*self_0).decor_delay_buffer[idx as usize]).as_mut_ptr(),
            length,
            scratch.as_mut_ptr(),
        );
        sb_sample = 0 as core::ffi::c_int as WORD32;
        while sb_sample < length {
            (*out.offset(sb_sample as isize))[idx as usize].re = scratch[sb_sample
                    as usize]
                .re;
            (*out.offset(sb_sample as isize))[idx as usize].im = scratch[sb_sample
                    as usize]
                .im;
            sb_sample += 1;
        }
        sb_sample = 0 as core::ffi::c_int as WORD32;
        while sb_sample < (*self_0).delay_sample_count[idx as usize] {
            (*self_0).decor_delay_buffer[idx as usize][sb_sample as usize].re = (*self_0)
                .decor_delay_buffer[idx as usize][(length + sb_sample) as usize]
                .re;
            (*self_0).decor_delay_buffer[idx as usize][sb_sample as usize].im = (*self_0)
                .decor_delay_buffer[idx as usize][(length + sb_sample) as usize]
                .im;
            sb_sample += 1;
        }
        idx += 1;
    }
    ixheaacd_mps_decor_energy_adjustment(
        &mut (*self_0).decor_nrg_smooth,
        in_0,
        out,
        length,
        res_bands,
        ldmps_present,
    );
}
unsafe extern "C" fn ixheaacd_convert_lattice_coefs_complex(
    order: WORD32,
    rfc_real: *const WORD32,
    rfc_imag: *const WORD32,
    apar_real: *mut WORD32,
    apar_imag: *mut WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut tmp_real: [WORD32; 21] = [0; 21];
    let mut tmp_imag: [WORD32; 21] = [0; 21];
    let mut temp: WORD64 = 0;
    *apar_real.offset(0 as core::ffi::c_int as isize) = 32768 as core::ffi::c_int
        as WORD32;
    *apar_imag.offset(0 as core::ffi::c_int as isize) = 0 as core::ffi::c_int as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < order {
        *apar_real.offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = *rfc_real
            .offset(i as isize);
        *apar_imag.offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = *rfc_imag
            .offset(i as isize);
        j = 0 as core::ffi::c_int as WORD32;
        while j < i {
            temp = *rfc_real.offset(i as isize) as WORD64
                * tmp_real[(i as core::ffi::c_int - j as core::ffi::c_int
                    - 1 as core::ffi::c_int) as usize] as WORD64
                + *rfc_imag.offset(i as isize) as WORD64
                    * tmp_imag[(i as core::ffi::c_int - j as core::ffi::c_int
                        - 1 as core::ffi::c_int) as usize] as WORD64;
            temp >>= 15 as core::ffi::c_int;
            *apar_real
                .offset((j as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = ixheaac_add32(
                tmp_real[j as usize],
                temp as WORD32,
            );
            temp = *rfc_real.offset(i as isize) as WORD64
                * tmp_imag[(i as core::ffi::c_int - j as core::ffi::c_int
                    - 1 as core::ffi::c_int) as usize] as WORD64
                + *rfc_imag.offset(i as isize) as WORD64
                    * tmp_real[(i as core::ffi::c_int - j as core::ffi::c_int
                        - 1 as core::ffi::c_int) as usize] as WORD64;
            temp >>= 15 as core::ffi::c_int;
            *apar_imag
                .offset((j as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = ixheaac_sub32(
                tmp_imag[j as usize],
                temp as WORD32,
            );
            j += 1;
        }
        j = 0 as core::ffi::c_int as WORD32;
        while j <= i {
            tmp_real[j as usize] = *apar_real
                .offset((j as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
            tmp_imag[j as usize] = *apar_imag
                .offset((j as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_decorr_filt_create(
    mut self_0: *mut ia_mps_dec_decorr_filter_instance_struct,
    decorr_seed: WORD32,
    qmf_band: WORD32,
    reverb_band: WORD32,
    dec_type: WORD32,
    mut ia_mps_dec_mps_table: *mut ia_mps_dec_mps_tables_struct,
) -> IA_ERRORCODE {
    let mut error_code: IA_ERRORCODE = IA_NO_ERROR;
    let mut i: WORD32 = 0;
    let mut lattice_coeff: *const WORD32 = 0 as *const WORD32;
    let mut lattice_coeff_real: [WORD32; 20] = [0; 20];
    let mut lattice_coeff_imag: [WORD32; 20] = [0; 20];
    let mut temp_1: WORD32 = 0;
    if self_0.is_null() {
        error_code = IA_FATAL_ERROR as IA_ERRORCODE;
    }
    if error_code == IA_NO_ERROR {
        match reverb_band {
            0 => {
                (*self_0).den_length = (DECORR_FILTER_ORDER_BAND_0
                    + 1 as core::ffi::c_int) as WORD32;
                (*self_0).num_length = (*self_0).den_length;
                lattice_coeff = &mut *(*((*(*ia_mps_dec_mps_table).decor_table_ptr)
                    .lattice_coeff_0)
                    .as_mut_ptr()
                    .offset(decorr_seed as isize))
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
            }
            1 => {
                (*self_0).den_length = (DECORR_FILTER_ORDER_BAND_1
                    + 1 as core::ffi::c_int) as WORD32;
                (*self_0).num_length = (*self_0).den_length;
                lattice_coeff = &mut *(*((*(*ia_mps_dec_mps_table).decor_table_ptr)
                    .lattice_coeff_1)
                    .as_mut_ptr()
                    .offset(decorr_seed as isize))
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
            }
            2 => {
                (*self_0).den_length = (DECORR_FILTER_ORDER_BAND_2
                    + 1 as core::ffi::c_int) as WORD32;
                (*self_0).num_length = (*self_0).den_length;
                lattice_coeff = &mut *(*((*(*ia_mps_dec_mps_table).decor_table_ptr)
                    .lattice_coeff_2)
                    .as_mut_ptr()
                    .offset(decorr_seed as isize))
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
            }
            3 => {
                (*self_0).den_length = (DECORR_FILTER_ORDER_BAND_3
                    + 1 as core::ffi::c_int) as WORD32;
                (*self_0).num_length = (*self_0).den_length;
                lattice_coeff = &mut *(*((*(*ia_mps_dec_mps_table).decor_table_ptr)
                    .lattice_coeff_3)
                    .as_mut_ptr()
                    .offset(decorr_seed as isize))
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
            }
            _ => return IA_FATAL_ERROR as IA_ERRORCODE,
        }
        (*self_0).state_length = if (*self_0).num_length > (*self_0).den_length {
            (*self_0).num_length
        } else {
            (*self_0).den_length
        };
    }
    if error_code == IA_NO_ERROR {
        let mut cos_tab: *const WORD32 = ((*(*ia_mps_dec_mps_table).hybrid_table_ptr)
            .cosine_array)
            .as_mut_ptr();
        let mut sin_tab: *const WORD32 = ((*(*ia_mps_dec_mps_table).hybrid_table_ptr)
            .sine_array)
            .as_mut_ptr();
        if dec_type == 1 as core::ffi::c_int {
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*self_0).num_length as core::ffi::c_int - 1 as core::ffi::c_int {
                temp_1 = qmf_band
                    * (*(*ia_mps_dec_mps_table).decor_table_ptr)
                        .lattice_delta_phi[decorr_seed as usize][i as usize]
                    >> 1 as core::ffi::c_int;
                lattice_coeff_real[i as usize] = ixheaacd_mps_mult32_shr_15(
                    ixheaacd_mps_cos(temp_1, cos_tab),
                    *lattice_coeff.offset(i as isize),
                );
                lattice_coeff_imag[i as usize] = ixheaacd_mps_mult32_shr_15(
                    ixheaacd_mps_sin(temp_1, sin_tab),
                    *lattice_coeff.offset(i as isize),
                );
                i += 1;
            }
            ixheaacd_convert_lattice_coefs_complex(
                (*self_0).num_length - 1 as WORD32,
                lattice_coeff_real.as_mut_ptr(),
                lattice_coeff_imag.as_mut_ptr(),
                (*self_0).denominator_real,
                (*self_0).denominator_imag,
            );
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*self_0).num_length {
                *((*self_0).numerator_real).offset(i as isize) = *((*self_0)
                    .denominator_real)
                    .offset(((*self_0).num_length - 1 as WORD32 - i) as isize);
                *((*self_0).numerator_imag).offset(i as isize) = -*((*self_0)
                    .denominator_imag)
                    .offset(((*self_0).num_length - 1 as WORD32 - i) as isize);
                i += 1;
            }
            (*self_0).complex = 1 as core::ffi::c_int as WORD32;
        } else {
            match reverb_band {
                0 => {
                    (*self_0).denominator_real = &mut *(*((*(*ia_mps_dec_mps_table)
                        .decor_table_ptr)
                        .den_coef_0)
                        .as_mut_ptr()
                        .offset(decorr_seed as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
                }
                1 => {
                    (*self_0).denominator_real = &mut *(*((*(*ia_mps_dec_mps_table)
                        .decor_table_ptr)
                        .den_coef_1)
                        .as_mut_ptr()
                        .offset(decorr_seed as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
                }
                2 => {
                    (*self_0).denominator_real = &mut *(*((*(*ia_mps_dec_mps_table)
                        .decor_table_ptr)
                        .den_coef_2)
                        .as_mut_ptr()
                        .offset(decorr_seed as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
                }
                3 => {
                    (*self_0).denominator_real = &mut *(*((*(*ia_mps_dec_mps_table)
                        .decor_table_ptr)
                        .den_coef_3)
                        .as_mut_ptr()
                        .offset(decorr_seed as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
                }
                _ => return IA_FATAL_ERROR as IA_ERRORCODE,
            }
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*self_0).num_length {
                *((*self_0).numerator_real).offset(i as isize) = *((*self_0)
                    .denominator_real)
                    .offset(((*self_0).num_length - 1 as WORD32 - i) as isize);
                i += 1;
            }
            (*self_0).complex = 0 as core::ffi::c_int as WORD32;
        }
    }
    return error_code;
}
unsafe extern "C" fn ixheaacd_decorr_filt_apply(
    self_0: *mut ia_mps_dec_decorr_filter_instance_struct,
    length: WORD32,
    input_real: *const WORD32,
    input_imag: *const WORD32,
    p_output_real: *mut WORD32,
    p_output_imag: *mut WORD32,
) -> VOID {
    let mut temp_1: WORD32 = 0;
    let mut temp_2: WORD32 = 0;
    let mut temp3: WORD32 = 0;
    let mut temp4: WORD32 = 0;
    let mut temp5: WORD32 = 0;
    let mut temp6: WORD32 = 0;
    let mut temp7: WORD32 = 0;
    let mut temp8: WORD32 = 0;
    let mut state_real: *mut WORD32 = 0 as *mut WORD32;
    let mut state_imag: *mut WORD32 = 0 as *mut WORD32;
    let mut numerator_real: *mut WORD32 = 0 as *mut WORD32;
    let mut denominator_real: *mut WORD32 = 0 as *mut WORD32;
    let mut output_real: *mut WORD32 = p_output_real;
    let mut output_imag: *mut WORD32 = p_output_imag;
    let mut common_part: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    common_part = (*self_0).num_length;
    state_real = (*self_0).state_real;
    state_imag = (*self_0).state_imag;
    numerator_real = (*self_0).numerator_real;
    denominator_real = (*self_0).denominator_real;
    i = 0 as core::ffi::c_int as WORD32;
    while i < length {
        temp5 = *input_real.offset(i as isize);
        temp6 = *input_imag.offset(i as isize);
        temp_1 = ixheaacd_mps_mult32_shr_14(
            temp5,
            *numerator_real.offset(0 as core::ffi::c_int as isize),
        );
        temp_2 = ixheaacd_mps_mult32_shr_14(
            temp6,
            *numerator_real.offset(0 as core::ffi::c_int as isize),
        );
        *output_real = ixheaac_add32_sat(
            temp_1,
            *state_real.offset(0 as core::ffi::c_int as isize),
        );
        *output_imag = ixheaac_add32_sat(
            temp_2,
            *state_imag.offset(0 as core::ffi::c_int as isize),
        );
        temp7 = *output_real;
        temp8 = *output_imag;
        output_real = output_real.offset(MAX_HYBRID_BANDS as isize);
        output_imag = output_imag.offset(MAX_HYBRID_BANDS as isize);
        j = 1 as core::ffi::c_int as WORD32;
        while j < common_part {
            temp_1 = ixheaacd_mps_mult32x16_shr_16(
                temp5,
                *numerator_real.offset(j as isize),
            );
            temp3 = ixheaacd_mps_mult32x16_shr_16(
                temp6,
                *numerator_real.offset(j as isize),
            );
            temp_2 = ixheaacd_mps_mult32x16_shr_16(
                temp7,
                *denominator_real.offset(j as isize),
            );
            temp4 = ixheaacd_mps_mult32x16_shr_16(
                temp8,
                *denominator_real.offset(j as isize),
            );
            temp_1 -= temp_2;
            *state_real
                .offset((j as core::ffi::c_int - 1 as core::ffi::c_int) as isize) = ixheaac_add32_sat(
                *state_real.offset(j as isize),
                temp_1 << 2 as core::ffi::c_int,
            );
            temp3 -= temp4;
            *state_imag
                .offset((j as core::ffi::c_int - 1 as core::ffi::c_int) as isize) = ixheaac_add32_sat(
                *state_imag.offset(j as isize),
                temp3 << 2 as core::ffi::c_int,
            );
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_ducker_apply_71(
    face: *mut ia_mps_dec_ducker_interface,
    time_slots: WORD32,
    mut input_real: *const WORD32,
    mut input_imag: *const WORD32,
    mut output_real: *mut WORD32,
    mut output_imag: *mut WORD32,
    mut ia_mps_dec_mps_table_ptr: *mut ia_mps_dec_mps_tables_struct,
    mut scratch: *mut core::ffi::c_void,
) -> VOID {
    let mut self_0: *mut ia_mps_dec_duck_instance_struct = &mut *face
        .offset(1 as core::ffi::c_int as isize) as *mut ia_mps_dec_ducker_interface
        as *mut ia_mps_dec_duck_instance_struct;
    let mut duck_gain: *mut WORD32 = 0 as *mut WORD32;
    let mut gain: WORD32 = 0;
    let mut qgain: WORD16 = 0;
    let mut direct_nrg: [WORD64; 28] = [0; 28];
    let mut reverb_nrg: [WORD64; 28] = [0; 28];
    let mut q_duck_gain: *mut WORD16 = 0 as *mut WORD16;
    let mut ts: WORD32 = 0;
    let mut qs: WORD32 = 0;
    let mut pb: WORD32 = 0;
    let mut qtemp1: WORD16 = 0;
    let mut qtemp2: WORD16 = 0;
    let mut qtemp3: WORD16 = 0;
    let mut temp_1: WORD32 = 0;
    let mut temp_2: WORD32 = 0;
    let mut temp3: WORD32 = 0;
    let mut p_input_real: *const WORD32 = 0 as *const WORD32;
    let mut p_input_imag: *const WORD32 = 0 as *const WORD32;
    let mut hybrid_2_param_28: *const WORD32 = ((*(*ia_mps_dec_mps_table_ptr)
        .m1_m2_table_ptr)
        .hybrid_2_param_28)
        .as_mut_ptr();
    let mut sqrt_tab: *const WORD32 = ((*(*ia_mps_dec_mps_table_ptr).common_table_ptr)
        .sqrt_tab)
        .as_mut_ptr();
    let mut smooth_direct_nrg: *mut WORD32 = ((*self_0).smooth_direct_nrg).as_mut_ptr();
    let mut q_smooth_direct_nrg: *mut WORD16 = ((*self_0).q_smooth_direct_nrg)
        .as_mut_ptr();
    let mut smooth_reverb_nrg: *mut WORD32 = ((*self_0).smooth_reverb_nrg).as_mut_ptr();
    let mut q_smooth_reverb_nrg: *mut WORD16 = ((*self_0).q_smooth_reverb_nrg)
        .as_mut_ptr();
    let mut parameter_bands: WORD32 = (*self_0).parameter_bands;
    let mut p_output_real: *mut WORD32 = 0 as *mut WORD32;
    let mut p_output_imag: *mut WORD32 = 0 as *mut WORD32;
    let mut num_bands_2: WORD32 = (*self_0).hybrid_bands;
    let mut v1: WORD32 = 0;
    let mut v2: WORD32 = 0;
    let mut v3: WORD32 = 0;
    let mut v4: WORD32 = 0;
    let mut one_by_5: WORD16 = ONE_BY_FIVE_Q16 as WORD16;
    duck_gain = scratch as *mut WORD32;
    q_duck_gain = (scratch as *mut WORD16)
        .offset(
            ((56 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD16>() as usize) as isize,
        );
    p_input_real = input_real;
    p_input_imag = input_imag;
    p_output_real = output_real;
    p_output_imag = output_imag;
    ts = 0 as core::ffi::c_int as WORD32;
    while ts < time_slots {
        memset(
            direct_nrg.as_mut_ptr() as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<[WORD64; 28]>() as size_t,
        );
        memset(
            reverb_nrg.as_mut_ptr() as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<[WORD64; 28]>() as size_t,
        );
        qs = 0 as core::ffi::c_int as WORD32;
        while qs < 55 as core::ffi::c_int {
            v1 = *p_input_real.offset(qs as isize);
            v2 = *p_input_imag.offset(qs as isize);
            v3 = *p_output_real.offset(qs as isize);
            v4 = *p_output_imag.offset(qs as isize);
            pb = *hybrid_2_param_28.offset(qs as isize);
            direct_nrg[pb as usize] = ixheaac_add64_sat(
                direct_nrg[pb as usize],
                ixheaac_mult32x32in64(v1, v1),
            );
            direct_nrg[pb as usize] = ixheaac_add64_sat(
                direct_nrg[pb as usize],
                ixheaac_mult32x32in64(v2, v2),
            );
            reverb_nrg[pb as usize] = ixheaac_add64_sat(
                reverb_nrg[pb as usize],
                ixheaac_mult32x32in64(v3, v3),
            );
            reverb_nrg[pb as usize] = ixheaac_add64_sat(
                reverb_nrg[pb as usize],
                ixheaac_mult32x32in64(v4, v4),
            );
            qs += 1;
        }
        while qs < num_bands_2 {
            v1 = *p_input_real.offset(qs as isize);
            v2 = *p_input_imag.offset(qs as isize);
            v3 = *p_output_real.offset(qs as isize);
            v4 = *p_output_imag.offset(qs as isize);
            direct_nrg[27 as core::ffi::c_int as usize] = ixheaac_add64_sat(
                direct_nrg[27 as core::ffi::c_int as usize],
                ixheaac_mult32x32in64(v1, v1),
            );
            direct_nrg[27 as core::ffi::c_int as usize] = ixheaac_add64_sat(
                direct_nrg[27 as core::ffi::c_int as usize],
                ixheaac_mult32x32in64(v2, v2),
            );
            reverb_nrg[27 as core::ffi::c_int as usize] = ixheaac_add64_sat(
                reverb_nrg[27 as core::ffi::c_int as usize],
                ixheaac_mult32x32in64(v3, v3),
            );
            reverb_nrg[27 as core::ffi::c_int as usize] = ixheaac_add64_sat(
                reverb_nrg[27 as core::ffi::c_int as usize],
                ixheaac_mult32x32in64(v4, v4),
            );
            qs += 1;
        }
        pb = 0 as core::ffi::c_int as WORD32;
        while pb < parameter_bands {
            let mut qtemp: WORD16 = 0;
            let mut qtemp_1: WORD16 = 0;
            temp_1 = ixheaacd_mps_narrow(direct_nrg[pb as usize], &mut qtemp);
            temp_2 = *smooth_direct_nrg.offset(pb as isize) << 2 as core::ffi::c_int;
            temp3 = ixheaacd_mps_add32(
                temp_2,
                temp_1,
                &mut *q_smooth_direct_nrg.offset(pb as isize),
                qtemp,
            );
            *smooth_direct_nrg.offset(pb as isize) = ixheaacd_mps_mult32x16_shr_16(
                temp3,
                one_by_5 as WORD32,
            );
            temp_1 = ixheaacd_mps_narrow(reverb_nrg[pb as usize], &mut qtemp);
            temp_2 = *smooth_reverb_nrg.offset(pb as isize) << 2 as core::ffi::c_int;
            temp3 = ixheaacd_mps_add32(
                temp_2,
                temp_1,
                &mut *q_smooth_reverb_nrg.offset(pb as isize),
                qtemp,
            );
            *smooth_reverb_nrg.offset(pb as isize) = ixheaacd_mps_mult32x16_shr_16(
                temp3,
                one_by_5 as WORD32,
            );
            qtemp1 = (*q_smooth_reverb_nrg.offset(pb as isize) as core::ffi::c_int
                - 1 as core::ffi::c_int) as WORD16;
            temp_1 = ((*smooth_reverb_nrg.offset(pb as isize) >> 2 as core::ffi::c_int)
                * 3 as core::ffi::c_int) as WORD32;
            qtemp = *q_smooth_direct_nrg.offset(pb as isize);
            temp3 = *smooth_direct_nrg.offset(pb as isize);
            if ixheaacd_mps_comp(temp3, temp_1, &mut qtemp, qtemp1) != 0 {
                temp_2 = ixheaacd_mps_div_32(temp3, temp_1, &mut qtemp2);
                qtemp2 = (qtemp2 as core::ffi::c_int + qtemp as core::ffi::c_int
                    - qtemp1 as core::ffi::c_int) as WORD16;
                if temp_1 == 0 as core::ffi::c_int {
                    qtemp2 = qtemp;
                }
                temp3 = (if qtemp2 as core::ffi::c_int > 28 as core::ffi::c_int {
                    MAX_32
                } else {
                    (4 as core::ffi::c_int) << qtemp2 as core::ffi::c_int
                }) as WORD32;
                if temp_2 > temp3 {
                    *duck_gain = (ONE_IN_Q15 - 1 as core::ffi::c_int) as WORD32;
                    let fresh6 = q_duck_gain;
                    q_duck_gain = q_duck_gain.offset(1);
                    *fresh6 = 14 as WORD16;
                } else {
                    *duck_gain = ixheaacd_mps_sqrt(temp_2, &mut qtemp2, sqrt_tab);
                    let fresh7 = q_duck_gain;
                    q_duck_gain = q_duck_gain.offset(1);
                    *fresh7 = qtemp2;
                }
                duck_gain = duck_gain.offset(1);
            } else {
                *duck_gain = (ONE_IN_Q14 - 1 as core::ffi::c_int) as WORD32;
                qtemp = (*q_smooth_direct_nrg.offset(pb as isize) as core::ffi::c_int
                    - 1 as core::ffi::c_int) as WORD16;
                temp_1 = ((*smooth_direct_nrg.offset(pb as isize)
                    >> 2 as core::ffi::c_int) * 3 as core::ffi::c_int) as WORD32;
                qtemp_1 = *q_smooth_reverb_nrg.offset(pb as isize);
                temp_2 = *smooth_reverb_nrg.offset(pb as isize);
                if ixheaacd_mps_comp(temp_2, temp_1, &mut qtemp_1, qtemp) != 0 {
                    temp3 = ixheaacd_mps_div_32(temp_1, temp_2, &mut qtemp3);
                    qtemp3 = (qtemp3 as core::ffi::c_int + qtemp as core::ffi::c_int
                        - qtemp_1 as core::ffi::c_int) as WORD16;
                    *duck_gain = ixheaacd_mps_sqrt(temp3, &mut qtemp3, sqrt_tab);
                    *q_duck_gain = qtemp3;
                }
                duck_gain = duck_gain.offset(1);
                q_duck_gain = q_duck_gain.offset(1);
            }
            pb += 1;
        }
        duck_gain = duck_gain.offset(-(parameter_bands as isize));
        q_duck_gain = q_duck_gain.offset(-(parameter_bands as isize));
        qs = 0 as core::ffi::c_int as WORD32;
        while qs < 55 as core::ffi::c_int {
            pb = *hybrid_2_param_28.offset(qs as isize);
            gain = *duck_gain.offset(pb as isize);
            if !(gain == 16383 as core::ffi::c_int) {
                qgain = *q_duck_gain.offset(pb as isize);
                *p_output_real.offset(qs as isize) = ixheaacd_mps_mult32_shr_n(
                    *p_output_real.offset(qs as isize),
                    gain,
                    qgain,
                );
                *p_output_imag.offset(qs as isize) = ixheaacd_mps_mult32_shr_n(
                    *p_output_imag.offset(qs as isize),
                    gain,
                    qgain,
                );
            }
            qs += 1;
        }
        gain = *duck_gain.offset(27 as core::ffi::c_int as isize);
        if gain != 16383 as core::ffi::c_int {
            qgain = *q_duck_gain.offset(27 as core::ffi::c_int as isize);
            while qs < num_bands_2 {
                *p_output_real.offset(qs as isize) = ixheaacd_mps_mult32_shr_n(
                    *p_output_real.offset(qs as isize),
                    gain,
                    qgain,
                );
                *p_output_imag.offset(qs as isize) = ixheaacd_mps_mult32_shr_n(
                    *p_output_imag.offset(qs as isize),
                    gain,
                    qgain,
                );
                qs += 1;
            }
        }
        p_input_real = p_input_real.offset(MAX_HYBRID_BANDS as isize);
        p_input_imag = p_input_imag.offset(MAX_HYBRID_BANDS as isize);
        p_output_real = p_output_real.offset(MAX_HYBRID_BANDS as isize);
        p_output_imag = p_output_imag.offset(MAX_HYBRID_BANDS as isize);
        ts += 1;
    }
}
unsafe extern "C" fn ixheaacd_ducker_apply(
    face: *mut ia_mps_dec_ducker_interface,
    time_slots: WORD32,
    mut input_real: *const WORD32,
    mut input_imag: *const WORD32,
    mut output_real: *mut WORD32,
    mut output_imag: *mut WORD32,
    mut ia_mps_dec_mps_table_ptr: *mut ia_mps_dec_mps_tables_struct,
    mut scratch: *mut core::ffi::c_void,
) -> VOID {
    let mut self_0: *mut ia_mps_dec_duck_instance_struct = &mut *face
        .offset(1 as core::ffi::c_int as isize) as *mut ia_mps_dec_ducker_interface
        as *mut ia_mps_dec_duck_instance_struct;
    let mut duck_gain: *mut WORD32 = 0 as *mut WORD32;
    let mut gain: WORD32 = 0;
    let mut qgain: WORD16 = 0;
    let mut direct_nrg: [WORD64; 28] = [0; 28];
    let mut reverb_nrg: [WORD64; 28] = [0; 28];
    let mut q_duck_gain: *mut WORD16 = 0 as *mut WORD16;
    let mut ts: WORD32 = 0;
    let mut qs: WORD32 = 0;
    let mut pb: WORD32 = 0;
    let mut qtemp1: WORD16 = 0;
    let mut qtemp2: WORD16 = 0;
    let mut qtemp3: WORD16 = 0;
    let mut temp_1: WORD32 = 0;
    let mut temp_2: WORD32 = 0;
    let mut temp3: WORD32 = 0;
    let mut p_input_real: *const WORD32 = 0 as *const WORD32;
    let mut p_input_imag: *const WORD32 = 0 as *const WORD32;
    let mut hybrid_2_param_28: *const WORD32 = ((*(*ia_mps_dec_mps_table_ptr)
        .m1_m2_table_ptr)
        .hybrid_2_param_28)
        .as_mut_ptr();
    let mut sqrt_tab: *const WORD32 = ((*(*ia_mps_dec_mps_table_ptr).common_table_ptr)
        .sqrt_tab)
        .as_mut_ptr();
    let mut smooth_direct_nrg: *mut WORD32 = ((*self_0).smooth_direct_nrg).as_mut_ptr();
    let mut q_smooth_direct_nrg: *mut WORD16 = ((*self_0).q_smooth_direct_nrg)
        .as_mut_ptr();
    let mut smooth_reverb_nrg: *mut WORD32 = ((*self_0).smooth_reverb_nrg).as_mut_ptr();
    let mut q_smooth_reverb_nrg: *mut WORD16 = ((*self_0).q_smooth_reverb_nrg)
        .as_mut_ptr();
    let mut parameter_bands: WORD32 = (*self_0).parameter_bands;
    let mut p_output_real: *mut WORD32 = 0 as *mut WORD32;
    let mut p_output_imag: *mut WORD32 = 0 as *mut WORD32;
    let mut num_bands_2: WORD32 = (*self_0).hybrid_bands;
    let mut v1: WORD32 = 0;
    let mut v2: WORD32 = 0;
    let mut v3: WORD32 = 0;
    let mut v4: WORD32 = 0;
    let mut one_by_5: WORD16 = ONE_BY_FIVE_Q16 as WORD16;
    duck_gain = scratch as *mut WORD32;
    q_duck_gain = (scratch as *mut WORD16)
        .offset(
            ((56 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD16>() as usize) as isize,
        );
    p_input_real = input_real;
    p_input_imag = input_imag;
    p_output_real = output_real;
    p_output_imag = output_imag;
    ts = 0 as core::ffi::c_int as WORD32;
    while ts < time_slots {
        memset(
            direct_nrg.as_mut_ptr() as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<[WORD64; 28]>() as size_t,
        );
        memset(
            reverb_nrg.as_mut_ptr() as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<[WORD64; 28]>() as size_t,
        );
        qs = 0 as core::ffi::c_int as WORD32;
        while qs < num_bands_2 {
            v1 = *p_input_real.offset(qs as isize);
            v2 = *p_input_imag.offset(qs as isize);
            v3 = *p_output_real.offset(qs as isize);
            v4 = *p_output_imag.offset(qs as isize);
            pb = *hybrid_2_param_28.offset(qs as isize);
            direct_nrg[pb as usize] = ixheaac_add64_sat(
                direct_nrg[pb as usize],
                ixheaac_mult32x32in64(v1, v1),
            );
            direct_nrg[pb as usize] = ixheaac_add64_sat(
                direct_nrg[pb as usize],
                ixheaac_mult32x32in64(v2, v2),
            );
            reverb_nrg[pb as usize] = ixheaac_add64_sat(
                reverb_nrg[pb as usize],
                ixheaac_mult32x32in64(v3, v3),
            );
            reverb_nrg[pb as usize] = ixheaac_add64_sat(
                reverb_nrg[pb as usize],
                ixheaac_mult32x32in64(v4, v4),
            );
            qs += 1;
        }
        pb = 0 as core::ffi::c_int as WORD32;
        while pb < parameter_bands {
            let mut qtemp: WORD16 = 0;
            let mut qtemp_1: WORD16 = 0;
            temp_1 = ixheaacd_mps_narrow(direct_nrg[pb as usize], &mut qtemp);
            temp_2 = *smooth_direct_nrg.offset(pb as isize) << 2 as core::ffi::c_int;
            temp3 = ixheaacd_mps_add32(
                temp_2,
                temp_1,
                &mut *q_smooth_direct_nrg.offset(pb as isize),
                qtemp,
            );
            *smooth_direct_nrg.offset(pb as isize) = ixheaacd_mps_mult32x16_shr_16(
                temp3,
                one_by_5 as WORD32,
            );
            temp_1 = ixheaacd_mps_narrow(reverb_nrg[pb as usize], &mut qtemp);
            temp_2 = *smooth_reverb_nrg.offset(pb as isize) << 2 as core::ffi::c_int;
            temp3 = ixheaacd_mps_add32(
                temp_2,
                temp_1,
                &mut *q_smooth_reverb_nrg.offset(pb as isize),
                qtemp,
            );
            *smooth_reverb_nrg.offset(pb as isize) = ixheaacd_mps_mult32x16_shr_16(
                temp3,
                one_by_5 as WORD32,
            );
            qtemp1 = (*q_smooth_reverb_nrg.offset(pb as isize) as core::ffi::c_int
                - 1 as core::ffi::c_int) as WORD16;
            temp_1 = ((*smooth_reverb_nrg.offset(pb as isize) >> 2 as core::ffi::c_int)
                * 3 as core::ffi::c_int) as WORD32;
            qtemp = *q_smooth_direct_nrg.offset(pb as isize);
            temp3 = *smooth_direct_nrg.offset(pb as isize);
            if ixheaacd_mps_comp(temp3, temp_1, &mut qtemp, qtemp1) != 0 {
                temp_2 = ixheaacd_mps_div_32(temp3, temp_1, &mut qtemp2);
                qtemp2 = (qtemp2 as core::ffi::c_int + qtemp as core::ffi::c_int
                    - qtemp1 as core::ffi::c_int) as WORD16;
                if temp_1 == 0 as core::ffi::c_int {
                    qtemp2 = qtemp;
                }
                temp3 = (if qtemp2 as core::ffi::c_int > 28 as core::ffi::c_int {
                    MAX_32
                } else {
                    (4 as core::ffi::c_int) << qtemp2 as core::ffi::c_int
                }) as WORD32;
                if temp_2 > temp3 {
                    *duck_gain = 32767 as core::ffi::c_int as WORD32;
                    let fresh4 = q_duck_gain;
                    q_duck_gain = q_duck_gain.offset(1);
                    *fresh4 = 14 as WORD16;
                } else {
                    *duck_gain = ixheaacd_mps_sqrt(temp_2, &mut qtemp2, sqrt_tab);
                    let fresh5 = q_duck_gain;
                    q_duck_gain = q_duck_gain.offset(1);
                    *fresh5 = qtemp2;
                }
                duck_gain = duck_gain.offset(1);
            } else {
                *duck_gain = 16383 as core::ffi::c_int as WORD32;
                qtemp = (*q_smooth_direct_nrg.offset(pb as isize) as core::ffi::c_int
                    - 1 as core::ffi::c_int) as WORD16;
                temp_1 = ((*smooth_direct_nrg.offset(pb as isize)
                    >> 2 as core::ffi::c_int) * 3 as core::ffi::c_int) as WORD32;
                qtemp_1 = *q_smooth_reverb_nrg.offset(pb as isize);
                temp_2 = *smooth_reverb_nrg.offset(pb as isize);
                if ixheaacd_mps_comp(temp_2, temp_1, &mut qtemp_1, qtemp) != 0 {
                    temp3 = ixheaacd_mps_div_32(temp_1, temp_2, &mut qtemp3);
                    qtemp3 = (qtemp3 as core::ffi::c_int + qtemp as core::ffi::c_int
                        - qtemp_1 as core::ffi::c_int) as WORD16;
                    *duck_gain = ixheaacd_mps_sqrt(temp3, &mut qtemp3, sqrt_tab);
                    *q_duck_gain = qtemp3;
                }
                duck_gain = duck_gain.offset(1);
                q_duck_gain = q_duck_gain.offset(1);
            }
            pb += 1;
        }
        duck_gain = duck_gain.offset(-(parameter_bands as isize));
        q_duck_gain = q_duck_gain.offset(-(parameter_bands as isize));
        qs = 0 as core::ffi::c_int as WORD32;
        while qs < num_bands_2 {
            pb = *hybrid_2_param_28.offset(qs as isize);
            gain = *duck_gain.offset(pb as isize);
            if !(gain == 16383 as core::ffi::c_int) {
                qgain = *q_duck_gain.offset(pb as isize);
                *p_output_real.offset(qs as isize) = ixheaacd_mps_mult32_shr_n(
                    *p_output_real.offset(qs as isize),
                    gain,
                    qgain,
                );
                *p_output_imag.offset(qs as isize) = ixheaacd_mps_mult32_shr_n(
                    *p_output_imag.offset(qs as isize),
                    gain,
                    qgain,
                );
            }
            qs += 1;
        }
        p_input_real = p_input_real.offset(MAX_HYBRID_BANDS as isize);
        p_input_imag = p_input_imag.offset(MAX_HYBRID_BANDS as isize);
        p_output_real = p_output_real.offset(MAX_HYBRID_BANDS as isize);
        p_output_imag = p_output_imag.offset(MAX_HYBRID_BANDS as isize);
        ts += 1;
    }
}
unsafe extern "C" fn ixheaacd_ducker_create(
    face: *mut ia_mps_dec_ducker_interface,
    hybrid_bands: WORD32,
) -> IA_ERRORCODE {
    let mut self_0: *mut ia_mps_dec_duck_instance_struct = 0
        as *mut ia_mps_dec_duck_instance_struct;
    let mut error_code: IA_ERRORCODE = IA_NO_ERROR;
    let mut i: WORD32 = 0;
    if face.is_null() {
        error_code = IA_FATAL_ERROR as IA_ERRORCODE;
    }
    if error_code == IA_NO_ERROR {
        self_0 = &mut *face.offset(1 as core::ffi::c_int as isize)
            as *mut ia_mps_dec_ducker_interface as *mut ia_mps_dec_duck_instance_struct;
        (*self_0).hybrid_bands = hybrid_bands;
        (*self_0).parameter_bands = MAX_PARAMETER_BANDS as WORD32;
        (*self_0).alpha = DUCK_ALPHA as WORD32;
        (*self_0).one_minus_alpha = DUCK_ONEMINUSALPHA as WORD32;
        (*self_0).gamma = DUCK_GAMMA as WORD32;
        (*self_0).abs_thr = ABS_THR_FIX as WORD32;
        (*self_0).hybrid_bands = hybrid_bands;
        (*self_0).parameter_bands = MAX_PARAMETER_BANDS as WORD32;
        (*self_0).qalpha = 15 as WORD16;
        (*self_0).qgamma = 14 as WORD16;
        if hybrid_bands == 71 as core::ffi::c_int {
            (*face).apply = Some(
                ixheaacd_ducker_apply_71
                    as unsafe extern "C" fn(
                        *mut ia_mps_dec_ducker_interface,
                        WORD32,
                        *const WORD32,
                        *const WORD32,
                        *mut WORD32,
                        *mut WORD32,
                        *mut ia_mps_dec_mps_tables_struct,
                        *mut core::ffi::c_void,
                    ) -> VOID,
            )
                as Option<
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
                >;
        } else {
            (*face).apply = Some(
                ixheaacd_ducker_apply
                    as unsafe extern "C" fn(
                        *mut ia_mps_dec_ducker_interface,
                        WORD32,
                        *const WORD32,
                        *const WORD32,
                        *mut WORD32,
                        *mut WORD32,
                        *mut ia_mps_dec_mps_tables_struct,
                        *mut core::ffi::c_void,
                    ) -> VOID,
            )
                as Option<
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
                >;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < MAX_PARAMETER_BANDS {
            (*self_0).q_smooth_direct_nrg[i as usize] = 31 as WORD16;
            (*self_0).q_smooth_reverb_nrg[i as usize] = 31 as WORD16;
            i += 1;
        }
    }
    return error_code;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_decorr_create(
    mut self_0: ia_mps_dec_decorr_dec_handle,
    mut subbands: WORD32,
    mut seed: WORD32,
    mut dec_type: WORD32,
    mut decorr_config: WORD32,
    mut ia_mps_dec_mps_table_ptr: *mut ia_mps_dec_mps_tables_struct,
) -> WORD32 {
    let mut error_code: IA_ERRORCODE = IA_NO_ERROR;
    let mut i: WORD32 = 0;
    let mut reverb_band: WORD32 = 0;
    let mut rev_split_freq: *const WORD32 = 0 as *const WORD32;
    match decorr_config {
        0 => {
            rev_split_freq = ((*(*ia_mps_dec_mps_table_ptr).decor_table_ptr)
                .rev_table
                .rev_split_freq_0)
                .as_mut_ptr();
        }
        1 => {
            rev_split_freq = ((*(*ia_mps_dec_mps_table_ptr).decor_table_ptr)
                .rev_table
                .rev_split_freq_1)
                .as_mut_ptr();
        }
        2 => {
            rev_split_freq = ((*(*ia_mps_dec_mps_table_ptr).decor_table_ptr)
                .rev_table
                .rev_split_freq_2)
                .as_mut_ptr();
        }
        _ => return IA_FATAL_ERROR as WORD32,
    }
    if error_code == IA_NO_ERROR {
        (*self_0).decorr_seed = seed;
        (*self_0).numbins = subbands;
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*self_0).numbins {
            reverb_band = 0 as core::ffi::c_int as WORD32;
            while reverb_band < 3 as core::ffi::c_int
                && ixheaacd_get_qmf_sb(i, (*ia_mps_dec_mps_table_ptr).mdct2qmf_table_ptr)
                    >= *rev_split_freq.offset(reverb_band as isize)
                        - 1 as core::ffi::c_int
            {
                reverb_band += 1;
            }
            (*self_0).no_sample_delay[i as usize] = (*(*ia_mps_dec_mps_table_ptr)
                .decor_table_ptr)
                .rev_table
                .rev_delay[reverb_band as usize][(*self_0).decorr_seed as usize];
            error_code = ixheaacd_decorr_filt_create(
                (*self_0).filter[i as usize],
                (*self_0).decorr_seed,
                ixheaacd_get_qmf_sb(i, (*ia_mps_dec_mps_table_ptr).mdct2qmf_table_ptr),
                reverb_band,
                dec_type,
                ia_mps_dec_mps_table_ptr,
            );
            i += 1;
        }
        if error_code == IA_NO_ERROR {
            error_code = ixheaacd_ducker_create((*self_0).ducker, (*self_0).numbins);
        }
    }
    return error_code as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_decorr_apply(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
    mut length: WORD32,
    mut input_real: *mut WORD32,
    mut input_imag: *mut WORD32,
    mut output_real: *mut WORD32,
    mut output_imag: *mut WORD32,
    mut index: WORD32,
) -> VOID {
    let mut l: WORD32 = index - (*pstr_mps_state).num_direct_signals;
    let mut decorr_ptr: ia_mps_dec_decorr_dec_handle = (*pstr_mps_state)
        .ap_decor[l as usize];
    let mut idx: WORD32 = 0;
    let mut sb_sample: WORD32 = 0;
    let mut p_input_real: *mut WORD32 = 0 as *mut WORD32;
    let mut p_input_re: *mut WORD32 = 0 as *mut WORD32;
    let mut p_input_imag: *mut WORD32 = 0 as *mut WORD32;
    let mut p_input_im: *mut WORD32 = 0 as *mut WORD32;
    let mut p_output_real: *mut WORD32 = 0 as *mut WORD32;
    let mut p_output_imag: *mut WORD32 = 0 as *mut WORD32;
    let mut p_output_re: *mut WORD32 = 0 as *mut WORD32;
    let mut p_output_im: *mut WORD32 = 0 as *mut WORD32;
    let mut delay_buffer_real: *mut WORD32 = 0 as *mut WORD32;
    let mut delay_buffer_imag: *mut WORD32 = 0 as *mut WORD32;
    let mut length1: WORD32 = 0;
    let mut free_scratch: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    free_scratch = ((*pstr_mps_state).mps_scratch_mem_v as *mut WORD32)
        .offset(
            ((144 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        ) as *mut core::ffi::c_void;
    if !decorr_ptr.is_null() {
        p_input_real = input_real;
        p_input_imag = input_imag;
        p_output_real = output_real;
        p_output_imag = output_imag;
        idx = 0 as core::ffi::c_int as WORD32;
        while idx < (*decorr_ptr).numbins {
            p_input_re = p_input_real;
            p_input_im = p_input_imag;
            p_output_re = p_output_real;
            p_output_im = p_output_imag;
            length1 = length - (*decorr_ptr).no_sample_delay[idx as usize];
            delay_buffer_real = &mut *(*((*decorr_ptr).delay_buffer_real)
                .offset(idx as isize))
                .offset(
                    *((*decorr_ptr).no_sample_delay).as_mut_ptr().offset(idx as isize)
                        as isize,
                ) as *mut WORD32;
            delay_buffer_imag = &mut *(*((*decorr_ptr).delay_buffer_imag)
                .offset(idx as isize))
                .offset(
                    *((*decorr_ptr).no_sample_delay).as_mut_ptr().offset(idx as isize)
                        as isize,
                ) as *mut WORD32;
            sb_sample = 0 as core::ffi::c_int as WORD32;
            while sb_sample < length1 {
                *delay_buffer_real.offset(sb_sample as isize) = *p_input_re;
                let fresh0 = delay_buffer_imag;
                delay_buffer_imag = delay_buffer_imag.offset(1);
                *fresh0 = *p_input_im;
                p_input_re = p_input_re.offset(MAX_HYBRID_BANDS as isize);
                p_input_im = p_input_im.offset(MAX_HYBRID_BANDS as isize);
                sb_sample += 1;
            }
            let fresh1 = p_output_re;
            p_output_re = p_output_re.offset(1);
            let fresh2 = p_output_im;
            p_output_im = p_output_im.offset(1);
            ixheaacd_decorr_filt_apply(
                (*decorr_ptr).filter[idx as usize],
                length,
                *((*decorr_ptr).delay_buffer_real).offset(idx as isize),
                *((*decorr_ptr).delay_buffer_imag).offset(idx as isize),
                fresh1,
                fresh2,
            );
            length1 = (*decorr_ptr).no_sample_delay[idx as usize];
            delay_buffer_real = &mut *(*((*decorr_ptr).delay_buffer_real)
                .offset(idx as isize))
                .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
            delay_buffer_imag = &mut *(*((*decorr_ptr).delay_buffer_imag)
                .offset(idx as isize))
                .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
            sb_sample = 0 as core::ffi::c_int as WORD32;
            while sb_sample < length1 {
                *delay_buffer_real.offset(sb_sample as isize) = *p_input_re;
                p_input_re = p_input_re.offset(MAX_HYBRID_BANDS as isize);
                let fresh3 = delay_buffer_imag;
                delay_buffer_imag = delay_buffer_imag.offset(1);
                *fresh3 = *p_input_im;
                p_input_im = p_input_im.offset(MAX_HYBRID_BANDS as isize);
                sb_sample += 1;
            }
            p_input_real = p_input_real.offset(1);
            p_input_imag = p_input_imag.offset(1);
            p_output_real = p_output_real.offset(1);
            p_output_imag = p_output_imag.offset(1);
            idx += 1;
        }
        ((*(*decorr_ptr).ducker).apply)
            .expect(
                "non-null function pointer",
            )(
            (*decorr_ptr).ducker,
            length,
            input_real,
            input_imag,
            output_real,
            output_imag,
            &mut (*pstr_mps_state).ia_mps_dec_mps_table,
            free_scratch,
        );
    }
}
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
unsafe extern "C" fn ixheaac_add32(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut sum: WORD32 = 0;
    sum = a + b;
    return sum;
}
#[inline]
unsafe extern "C" fn ixheaac_sub32(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut diff: WORD32 = 0;
    diff = a - b;
    return diff;
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
unsafe extern "C" fn ixheaac_mult32_shl(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 32 as core::ffi::c_int) as WORD32;
    return result << 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn ixheaac_mult32x32in64(mut a: WORD32, mut b: WORD32) -> WORD64 {
    let mut result: WORD64 = 0;
    result = a as WORD64 * b as WORD64;
    return result;
}
#[inline]
unsafe extern "C" fn ixheaac_add64_sat(mut a: WORD64, mut b: WORD64) -> WORD64 {
    let mut result: WORD64 = 0;
    let mut comp: WORD64 = 0;
    result = if a < 0 as WORD64 { MIN_64 } else { MAX_64 };
    comp = result - a;
    if (a < 0 as WORD64) as core::ffi::c_int == (b > comp) as core::ffi::c_int {
        result = a + b;
    }
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
pub const MAX_64: WORD64 = 0x7fffffffffffffff as core::ffi::c_long as WORD64;
pub const MIN_64: WORD64 = 0x8000000000000000 as core::ffi::c_ulong as WORD64;
pub const MAX_32: WORD32 = 0x7fffffff as core::ffi::c_long as WORD32;
pub const MIN_32: WORD32 = 0x80000000 as core::ffi::c_long as WORD32;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const IA_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_FATAL_ERROR: core::ffi::c_uint = 0x80000000 as core::ffi::c_uint;
#[inline]
unsafe extern "C" fn ixheaacd_mps_narrow(
    mut a: WORD64,
    mut qfac: *mut WORD16,
) -> WORD32 {
    let mut x: WORD32 = 0;
    x = ixheaacd_mps_get_rshift_bits(a);
    *qfac = (20 as WORD32 - x) as WORD16;
    return (a >> x) as WORD32;
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
unsafe extern "C" fn ixheaacd_mps_mult32x16_shr_16(
    mut a: WORD32,
    mut b: WORD32,
) -> WORD32 {
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
unsafe extern "C" fn ixheaacd_mps_mult32_shr_14(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 16 as core::ffi::c_int) as WORD32;
    result = result << 2 as core::ffi::c_int;
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
unsafe extern "C" fn ixheaacd_mps_cos(
    mut a: WORD32,
    mut cosine_tab: *const WORD32,
) -> WORD32 {
    let mut temp_result: WORD32 = 0;
    if a < 0 as core::ffi::c_int {
        a = -a;
    }
    a = (a as core::ffi::c_int % TWO_PI_IN_Q15) as WORD32;
    temp_result = *cosine_tab
        .offset(
            (a as core::ffi::c_int * TRIG_TABLE_CONV_FAC >> 15 as core::ffi::c_int)
                as isize,
        );
    return temp_result;
}
#[inline]
unsafe extern "C" fn ixheaacd_mps_sin(
    mut a: WORD32,
    mut sine_tab: *const WORD32,
) -> WORD32 {
    let mut temp_result: WORD32 = 0;
    let mut flag: WORD32 = 0 as WORD32;
    if a < 0 as core::ffi::c_int {
        a = -a;
        flag = 1 as core::ffi::c_int as WORD32;
    }
    a = (a as core::ffi::c_int % TWO_PI_IN_Q15) as WORD32;
    temp_result = *sine_tab
        .offset(
            (a as core::ffi::c_int * TRIG_TABLE_CONV_FAC >> 15 as core::ffi::c_int)
                as isize,
        );
    if flag != 0 {
        temp_result = -temp_result;
    }
    return temp_result;
}
#[inline]
unsafe extern "C" fn ixheaacd_mps_comp(
    mut a: WORD32,
    mut b: WORD32,
    mut q_a: *mut WORD16,
    mut q_b: WORD16,
) -> WORD32 {
    if a == 0 as core::ffi::c_int || b == 0 as core::ffi::c_int {
        if a == 0 as core::ffi::c_int {
            if b < 0 as core::ffi::c_int {
                return 1 as WORD32
            } else {
                return 0 as WORD32
            }
        } else if b == 0 as core::ffi::c_int {
            if a > 0 as core::ffi::c_int {
                return 1 as WORD32
            } else {
                return 0 as WORD32
            }
        }
    }
    if *q_a as core::ffi::c_int > q_b as core::ffi::c_int {
        a = a >> *q_a as core::ffi::c_int - q_b as core::ffi::c_int;
    } else {
        b = b >> q_b as core::ffi::c_int - *q_a as core::ffi::c_int;
    }
    if a > b { return 1 as WORD32 } else { return 0 as WORD32 };
}
