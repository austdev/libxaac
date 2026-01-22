extern "C" {
    pub type ia_mps_dec_ducker_interface;
    fn ixheaacd_measure_tonality(
        pstr_mps_state: *mut ia_heaac_mps_state_struct,
        tonality: *mut WORD32,
    ) -> VOID;
}
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
pub type size_t = usize;
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
pub const MAX_PARAMETER_BANDS: core::ffi::c_int = 28 as core::ffi::c_int;
pub const MAX_M_INPUT: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_M_OUTPUT: core::ffi::c_int = 2 as core::ffi::c_int;
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
pub const MAX_RESIDUAL_CHANNELS_MPS: core::ffi::c_int = 10 as core::ffi::c_int;
pub const POINT_EIGHT_Q15: core::ffi::c_int = 26214 as core::ffi::c_int;
pub const PBXPS: core::ffi::c_int = 224 as core::ffi::c_int;
pub const INCHXPBXPS: core::ffi::c_int = 1344 as core::ffi::c_int;
pub const SMG_TIME_64: core::ffi::c_int = 64;
pub const SMG_TIME_128: core::ffi::c_int = 128;
pub const SMG_TIME_256: core::ffi::c_int = 256;
pub const SMG_TIME_512: core::ffi::c_int = 512;
pub const ONE_BY_128_IN_Q30: core::ffi::c_int = 8388608 as core::ffi::c_int;
pub const ONE_IN_Q30: core::ffi::c_int = 1073741824 as core::ffi::c_int;
pub const PI_IN_Q27: core::ffi::c_int = 421657440 as core::ffi::c_int;
pub const FIFTY_X_PI_BY_180_Q27: core::ffi::c_int = 117127067 as core::ffi::c_int;
pub const TWENTY_FIVE_X_PI_BY_180_Q27: core::ffi::c_int = 58563533 as core::ffi::c_int;
pub const Q28_VALUE: core::ffi::c_int = (1 as core::ffi::c_int)
    << 28 as core::ffi::c_int;
pub const Q28_FLOAT_VAL: FLOAT32 = ((1 as core::ffi::c_int) << 28 as core::ffi::c_int)
    as FLOAT32;
pub const ONE_BY_Q28_FLOAT_VAL: FLOAT32 = 1.0f32 / Q28_FLOAT_VAL;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_pre_matrix_mix_matrix_smoothing(
    mut self_0: *mut ia_mps_dec_state_struct,
) -> VOID {
    let mut smooth_band: WORD32 = 0;
    let mut delta: FLOAT32 = 0.;
    let mut one_minus_delta: FLOAT32 = 0.;
    let mut ps: WORD32 = 0 as WORD32;
    let mut pb: WORD32 = 0;
    let mut row: WORD32 = 0;
    let mut col: WORD32 = 0;
    let mut res_bands: WORD32 = 0 as WORD32;
    let mut p_smoothing_data: *mut WORD32 = 0 as *mut WORD32;
    if (*self_0).residual_coding != 0 {
        res_bands = (*self_0).max_res_bands;
    }
    p_smoothing_data = &mut *(*((*self_0).smoothing_data)
        .as_mut_ptr()
        .offset(ps as isize))
        .as_mut_ptr()
        .offset(res_bands as isize) as *mut WORD32;
    delta = (*self_0).param_slot_diff[ps as usize] as FLOAT32
        * (*self_0).inv_smoothing_time[ps as usize];
    one_minus_delta = 1.0f32 - delta;
    pb = res_bands;
    while pb < (*self_0).bs_param_bands {
        let fresh0 = p_smoothing_data;
        p_smoothing_data = p_smoothing_data.offset(1);
        smooth_band = *fresh0;
        if smooth_band != 0 {
            row = 0 as core::ffi::c_int as WORD32;
            while row < MAX_M_OUTPUT {
                col = 0 as core::ffi::c_int as WORD32;
                while col < MAX_M_INPUT {
                    (*self_0)
                        .m1_param_re[ps
                        as usize][pb as usize][row as usize][col as usize] = delta
                        * (*self_0)
                            .m1_param_re[ps
                            as usize][pb as usize][row as usize][col as usize]
                        + one_minus_delta
                            * (*self_0)
                                .m1_param_re_prev[pb as usize][row as usize][col as usize];
                    (*self_0)
                        .m1_param_im[ps
                        as usize][pb as usize][row as usize][col as usize] = delta
                        * (*self_0)
                            .m1_param_im[ps
                            as usize][pb as usize][row as usize][col as usize]
                        + one_minus_delta
                            * (*self_0)
                                .m1_param_im_prev[pb as usize][row as usize][col as usize];
                    (*self_0)
                        .m2_decor_re[ps
                        as usize][pb as usize][row as usize][col as usize] = delta
                        * (*self_0)
                            .m2_decor_re[ps
                            as usize][pb as usize][row as usize][col as usize]
                        + one_minus_delta
                            * (*self_0)
                                .m2_decor_re_prev[pb as usize][row as usize][col as usize];
                    (*self_0)
                        .m2_decor_im[ps
                        as usize][pb as usize][row as usize][col as usize] = delta
                        * (*self_0)
                            .m2_decor_im[ps
                            as usize][pb as usize][row as usize][col as usize]
                        + one_minus_delta
                            * (*self_0)
                                .m2_decor_im_prev[pb as usize][row as usize][col as usize];
                    (*self_0)
                        .m2_resid_re[ps
                        as usize][pb as usize][row as usize][col as usize] = delta
                        * (*self_0)
                            .m2_resid_re[ps
                            as usize][pb as usize][row as usize][col as usize]
                        + one_minus_delta
                            * (*self_0)
                                .m2_resid_re_prev[pb as usize][row as usize][col as usize];
                    (*self_0)
                        .m2_resid_im[ps
                        as usize][pb as usize][row as usize][col as usize] = delta
                        * (*self_0)
                            .m2_resid_im[ps
                            as usize][pb as usize][row as usize][col as usize]
                        + one_minus_delta
                            * (*self_0)
                                .m2_resid_im_prev[pb as usize][row as usize][col as usize];
                    col += 1;
                }
                row += 1;
            }
            (*self_0).pre_mix_req += 1;
        }
        pb += 1;
    }
    ps = 1 as core::ffi::c_int as WORD32;
    while ps < (*self_0).num_parameter_sets {
        delta = (*self_0).param_slot_diff[ps as usize] as FLOAT32
            * (*self_0).inv_smoothing_time[ps as usize];
        one_minus_delta = 1.0f32 - delta;
        p_smoothing_data = &mut *(*((*self_0).smoothing_data)
            .as_mut_ptr()
            .offset(ps as isize))
            .as_mut_ptr()
            .offset(res_bands as isize) as *mut WORD32;
        pb = res_bands;
        while pb < (*self_0).bs_param_bands {
            let fresh1 = p_smoothing_data;
            p_smoothing_data = p_smoothing_data.offset(1);
            smooth_band = *fresh1;
            if smooth_band != 0 {
                row = 0 as core::ffi::c_int as WORD32;
                while row < MAX_M_OUTPUT {
                    col = 0 as core::ffi::c_int as WORD32;
                    while col < MAX_M_INPUT {
                        (*self_0)
                            .m1_param_re[ps
                            as usize][pb as usize][row as usize][col as usize] = delta
                            * (*self_0)
                                .m1_param_re[ps
                                as usize][pb as usize][row as usize][col as usize]
                            + one_minus_delta
                                * (*self_0)
                                    .m1_param_re[(ps as core::ffi::c_int
                                    - 1 as core::ffi::c_int)
                                    as usize][pb as usize][row as usize][col as usize];
                        (*self_0)
                            .m1_param_im[ps
                            as usize][pb as usize][row as usize][col as usize] = delta
                            * (*self_0)
                                .m1_param_im[ps
                                as usize][pb as usize][row as usize][col as usize]
                            + one_minus_delta
                                * (*self_0)
                                    .m1_param_im[(ps as core::ffi::c_int
                                    - 1 as core::ffi::c_int)
                                    as usize][pb as usize][row as usize][col as usize];
                        (*self_0)
                            .m2_resid_re[ps
                            as usize][pb as usize][row as usize][col as usize] = delta
                            * (*self_0)
                                .m2_resid_re[ps
                                as usize][pb as usize][row as usize][col as usize]
                            + one_minus_delta
                                * (*self_0)
                                    .m2_resid_re[(ps as core::ffi::c_int
                                    - 1 as core::ffi::c_int)
                                    as usize][pb as usize][row as usize][col as usize];
                        (*self_0)
                            .m2_decor_re[ps
                            as usize][pb as usize][row as usize][col as usize] = delta
                            * (*self_0)
                                .m2_decor_re[ps
                                as usize][pb as usize][row as usize][col as usize]
                            + one_minus_delta
                                * (*self_0)
                                    .m2_decor_re[(ps as core::ffi::c_int
                                    - 1 as core::ffi::c_int)
                                    as usize][pb as usize][row as usize][col as usize];
                        (*self_0)
                            .m2_decor_im[ps
                            as usize][pb as usize][row as usize][col as usize] = delta
                            * (*self_0)
                                .m2_decor_im[ps
                                as usize][pb as usize][row as usize][col as usize]
                            + one_minus_delta
                                * (*self_0)
                                    .m2_decor_im[(ps as core::ffi::c_int
                                    - 1 as core::ffi::c_int)
                                    as usize][pb as usize][row as usize][col as usize];
                        (*self_0)
                            .m2_resid_im[ps
                            as usize][pb as usize][row as usize][col as usize] = delta
                            * (*self_0)
                                .m2_resid_im[ps
                                as usize][pb as usize][row as usize][col as usize]
                            + one_minus_delta
                                * (*self_0)
                                    .m2_resid_im[(ps as core::ffi::c_int
                                    - 1 as core::ffi::c_int)
                                    as usize][pb as usize][row as usize][col as usize];
                        col += 1;
                    }
                    row += 1;
                }
                (*self_0).pre_mix_req += 1;
            }
            pb += 1;
        }
        ps += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_smoothing_opd(
    mut self_0: *mut ia_mps_dec_state_struct,
) -> VOID {
    let mut ps: WORD32 = 0;
    let mut pb: WORD32 = 0;
    let mut delta: WORD32 = 0;
    let mut one_minus_delta: WORD32 = 0;
    if (*self_0).opd_smoothing_mode == 0 as core::ffi::c_int {
        pb = 0 as core::ffi::c_int as WORD32;
        while pb < (*self_0).bs_param_bands {
            (*self_0).opd_smooth.smooth_l_phase[pb as usize] = ((*self_0)
                .phase_l[((*self_0).num_parameter_sets as core::ffi::c_int
                - 1 as core::ffi::c_int) as usize][pb as usize] * Q28_VALUE as FLOAT32)
                as WORD32 >> 1 as core::ffi::c_int;
            (*self_0).opd_smooth.smooth_r_phase[pb as usize] = ((*self_0)
                .phase_r[((*self_0).num_parameter_sets as core::ffi::c_int
                - 1 as core::ffi::c_int) as usize][pb as usize] * Q28_VALUE as FLOAT32)
                as WORD32 >> 1 as core::ffi::c_int;
            pb += 1;
        }
        return;
    }
    ps = 0 as core::ffi::c_int as WORD32;
    while ps < (*self_0).num_parameter_sets {
        let mut thr: WORD32 = if (*self_0)
            .bs_frame
            .ipd_data
            .bs_quant_coarse_xxx[ps as usize] as core::ffi::c_int != 0
        {
            FIFTY_X_PI_BY_180_Q27
        } else {
            TWENTY_FIVE_X_PI_BY_180_Q27
        };
        delta = ((*self_0).param_slot_diff[ps as usize] * ONE_BY_128_IN_Q30) as WORD32;
        one_minus_delta = ONE_IN_Q30 - delta;
        pb = 0 as core::ffi::c_int as WORD32;
        while pb < (*self_0).bs_param_bands {
            let mut ltemp: WORD32 = 0;
            let mut rtemp: WORD32 = 0;
            let mut tmp: WORD32 = 0;
            ltemp = ((*self_0).phase_l[ps as usize][pb as usize] * Q28_FLOAT_VAL)
                as WORD32 >> 1 as core::ffi::c_int;
            rtemp = ((*self_0).phase_r[ps as usize][pb as usize] * Q28_FLOAT_VAL)
                as WORD32 >> 1 as core::ffi::c_int;
            while ltemp > (*self_0).opd_smooth.smooth_l_phase[pb as usize] + PI_IN_Q27 {
                ltemp -= 2 as core::ffi::c_int * PI_IN_Q27;
            }
            while ltemp < (*self_0).opd_smooth.smooth_l_phase[pb as usize] - PI_IN_Q27 {
                ltemp += 2 as core::ffi::c_int * PI_IN_Q27;
            }
            while rtemp > (*self_0).opd_smooth.smooth_r_phase[pb as usize] + PI_IN_Q27 {
                rtemp -= 2 as core::ffi::c_int * PI_IN_Q27;
            }
            while rtemp < (*self_0).opd_smooth.smooth_r_phase[pb as usize] - PI_IN_Q27 {
                rtemp += 2 as core::ffi::c_int * PI_IN_Q27;
            }
            (*self_0).opd_smooth.smooth_l_phase[pb as usize] = ixheaac_mult32_shl(
                delta,
                ltemp,
            )
                + ixheaac_mult32_shl(
                    one_minus_delta,
                    (*self_0).opd_smooth.smooth_l_phase[pb as usize],
                ) << 1 as core::ffi::c_int;
            (*self_0).opd_smooth.smooth_r_phase[pb as usize] = ixheaac_mult32_shl(
                delta,
                rtemp,
            )
                + ixheaac_mult32_shl(
                    one_minus_delta,
                    (*self_0).opd_smooth.smooth_r_phase[pb as usize],
                ) << 1 as core::ffi::c_int;
            tmp = ltemp - rtemp
                - ((*self_0).opd_smooth.smooth_l_phase[pb as usize]
                    - (*self_0).opd_smooth.smooth_r_phase[pb as usize]);
            while tmp > PI_IN_Q27 {
                tmp -= 2 as core::ffi::c_int * PI_IN_Q27;
            }
            while tmp < -PI_IN_Q27 {
                tmp += 2 as core::ffi::c_int * PI_IN_Q27;
            }
            if ixheaac_abs32(tmp) > thr {
                (*self_0).opd_smooth.smooth_l_phase[pb as usize] = ltemp;
                (*self_0).opd_smooth.smooth_r_phase[pb as usize] = rtemp;
            }
            while (*self_0).opd_smooth.smooth_l_phase[pb as usize]
                > 2 as core::ffi::c_int * PI_IN_Q27
            {
                (*self_0).opd_smooth.smooth_l_phase[pb as usize]
                    -= 2 as core::ffi::c_int * PI_IN_Q27;
            }
            while (*self_0).opd_smooth.smooth_l_phase[pb as usize]
                < 0 as core::ffi::c_int
            {
                (*self_0).opd_smooth.smooth_l_phase[pb as usize]
                    += 2 as core::ffi::c_int * PI_IN_Q27;
            }
            while (*self_0).opd_smooth.smooth_r_phase[pb as usize]
                > 2 as core::ffi::c_int * PI_IN_Q27
            {
                (*self_0).opd_smooth.smooth_r_phase[pb as usize]
                    -= 2 as core::ffi::c_int * PI_IN_Q27;
            }
            while (*self_0).opd_smooth.smooth_r_phase[pb as usize]
                < 0 as core::ffi::c_int
            {
                (*self_0).opd_smooth.smooth_r_phase[pb as usize]
                    += 2 as core::ffi::c_int * PI_IN_Q27;
            }
            (*self_0).phase_l[ps as usize][pb as usize] = ((*self_0)
                .opd_smooth
                .smooth_l_phase[pb as usize] << 1 as core::ffi::c_int) as FLOAT32
                * ONE_BY_Q28_FLOAT_VAL;
            (*self_0).phase_r[ps as usize][pb as usize] = ((*self_0)
                .opd_smooth
                .smooth_r_phase[pb as usize] << 1 as core::ffi::c_int) as FLOAT32
                * ONE_BY_Q28_FLOAT_VAL;
            pb += 1;
        }
        ps += 1;
    }
}
unsafe extern "C" fn ixheaacd_calc_filter_coeff(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
    mut ps: WORD32,
    mut delta: *mut WORD32,
) -> VOID {
    let mut d_slots: WORD32 = 0;
    let mut param_slot: *mut WORD32 = ((*(*pstr_mps_state).aux_struct).param_slot)
        .as_mut_ptr();
    let mut smg_time: *mut WORD32 = ((*(*pstr_mps_state).aux_struct).smg_time)
        .as_mut_ptr();
    if ps == 0 as core::ffi::c_int {
        d_slots = (*param_slot.offset(ps as isize) + 1 as core::ffi::c_int) as WORD32;
    } else {
        d_slots = *param_slot.offset(ps as isize)
            - *param_slot
                .offset((ps as core::ffi::c_int - 1 as core::ffi::c_int) as isize);
    }
    if (*pstr_mps_state).smooth_control != 0 {
        match *smg_time.offset(ps as isize) {
            SMG_TIME_64 => {
                *delta = d_slots << 9 as core::ffi::c_int;
            }
            SMG_TIME_128 => {
                *delta = d_slots << 8 as core::ffi::c_int;
            }
            SMG_TIME_256 => {
                *delta = d_slots << 7 as core::ffi::c_int;
            }
            SMG_TIME_512 => {
                *delta = d_slots << 6 as core::ffi::c_int;
            }
            _ => {}
        }
    } else {
        *delta = d_slots << 7 as core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_smooth_m1m2(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
) -> VOID {
    let mut curr_state: *mut ia_heaac_mps_state_struct = pstr_mps_state;
    let mut persistent_mem: *mut ia_mps_persistent_mem = &mut (*curr_state)
        .mps_persistent_mem;
    let mut p_aux_struct: *mut ia_mps_dec_auxilary_struct = (*pstr_mps_state).aux_struct;
    let mut m2_param: *mut ia_mps_dec_m2_param_struct = (*p_aux_struct).m2_param;
    let mut m1_param: *mut ia_mps_dec_m1_param_struct = (*(*pstr_mps_state).array_struct)
        .m1_param;
    let mut m1_param_real_prev: *mut WORD32 = (*persistent_mem).m1_param_real_prev;
    let mut m2_decor_real_prev: *mut WORD32 = (*persistent_mem).m2_decor_real_prev;
    let mut m2_resid_real_prev: *mut WORD32 = (*persistent_mem).m2_resid_real_prev;
    let mut num_parameter_bands: WORD32 = (*curr_state).num_parameter_bands;
    let mut num_direct_signals: WORD32 = (*curr_state).num_direct_signals;
    let mut num_decor_signals: WORD32 = (*curr_state).num_decor_signals;
    let mut m1_param_imag_present: WORD32 = (*curr_state).m1_param_imag_present;
    let mut m2_param_imag_present: WORD32 = (*curr_state).m2_param_imag_present;
    let mut col_counter: WORD32 = num_direct_signals + num_decor_signals;
    let mut num_parameter_sets: WORD32 = (*curr_state).num_parameter_sets;
    let mut num_output_channels: WORD32 = (*curr_state).num_output_channels;
    let mut num_v_channels: WORD32 = (*curr_state).num_v_channels;
    let mut num_x_channels: WORD32 = (*curr_state).num_x_channels;
    let mut smooth_control: WORD32 = (*curr_state).smooth_control;
    let mut smooth_config: WORD32 = (*curr_state).smooth_config;
    let mut resid_col_counter: WORD32 = 0;
    let mut smooth_band_arr: [[WORD32; 28]; 8] = [[0; 28]; 8];
    let mut delta: *mut WORD32 = 0 as *mut WORD32;
    let mut one_minus_delta: *mut WORD32 = 0 as *mut WORD32;
    let mut delta_ptr: *mut WORD32 = 0 as *mut WORD32;
    let mut one_minus_delta_ptr: *mut WORD32 = 0 as *mut WORD32;
    let mut param_r: *mut WORD32 = 0 as *mut WORD32;
    let mut param_i: *mut WORD32 = 0 as *mut WORD32;
    let mut param_prev_r: *mut WORD32 = 0 as *mut WORD32;
    let mut param_prev_i: *mut WORD32 = 0 as *mut WORD32;
    let mut ton: *mut WORD32 = 0 as *mut WORD32;
    let mut i: WORD32 = 0;
    let mut ps: WORD32 = 0;
    let mut pb: WORD32 = 0;
    let mut row: WORD32 = 0;
    let mut col: WORD32 = 0;
    let mut res_bands: WORD32 = 0 as WORD32;
    let mut idx: WORD32 = 0 as WORD32;
    let mut m2_decor_imag_prev: *mut WORD32 = (*persistent_mem).m2_decor_imag_prev;
    let mut m2_resid_imag_prev: *mut WORD32 = (*persistent_mem).m2_resid_imag_prev;
    let mut m1_param_imag_prev: *mut WORD32 = (*persistent_mem).m1_param_imag_prev;
    ton = (*pstr_mps_state).mps_scratch_mem_v as *mut WORD32;
    delta_ptr = ton
        .offset(
            ((28 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    delta = delta_ptr;
    one_minus_delta_ptr = delta
        .offset(
            ((8 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    one_minus_delta = one_minus_delta_ptr;
    param_r = ((*curr_state).res_bands).as_mut_ptr();
    if (*curr_state).residual_coding != 0 {
        i = 0 as core::ffi::c_int as WORD32;
        while i < MAX_RESIDUAL_CHANNELS_MPS {
            if *param_r.offset(i as isize) > res_bands {
                res_bands = *param_r.offset(i as isize);
            }
            i += 1;
        }
    }
    if (*curr_state).arbitrary_downmix == 2 as core::ffi::c_int {
        if res_bands < (*curr_state).arbdmx_residual_bands {
            res_bands = (*curr_state).arbdmx_residual_bands;
        }
    }
    if smooth_config != 0 {
        ixheaacd_measure_tonality(pstr_mps_state, ton);
    }
    ps = 0 as core::ffi::c_int as WORD32;
    while ps < num_parameter_sets {
        ixheaacd_calc_filter_coeff(pstr_mps_state, ps, delta);
        let fresh2 = delta;
        delta = delta.offset(1);
        let fresh3 = one_minus_delta;
        one_minus_delta = one_minus_delta.offset(1);
        *fresh3 = ((1 as WORD32) << 15 as core::ffi::c_int) - *fresh2;
        ps += 1;
    }
    if smooth_control != 0 {
        ps = 0 as core::ffi::c_int as WORD32;
        while ps < num_parameter_sets {
            if ps < 8 as core::ffi::c_int {
                pb = 0 as core::ffi::c_int as WORD32;
                while pb < num_parameter_bands {
                    smooth_band_arr[ps as usize][pb as usize] = (*(*pstr_mps_state)
                        .aux_struct)
                        .smg_data[ps as usize][pb as usize];
                    pb += 1;
                }
            }
            ps += 1;
        }
    } else if smooth_config != 0 {
        ps = 0 as core::ffi::c_int as WORD32;
        while ps < num_parameter_sets {
            pb = 0 as core::ffi::c_int as WORD32;
            while pb < num_parameter_bands {
                smooth_band_arr[ps as usize][pb as usize] = (*ton.offset(pb as isize)
                    > POINT_EIGHT_Q15) as core::ffi::c_int as WORD32;
                pb += 1;
            }
            ps += 1;
        }
    }
    if !(smooth_control == 0 as core::ffi::c_int
        && smooth_config == 0 as core::ffi::c_int)
    {
        if m1_param_imag_present != 0 {
            let mut ptr_r1: *mut WORD32 = &mut *(*(*(*((*m1_param).m1_param_real)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
            let mut ptr_i1: *mut WORD32 = &mut *(*(*(*((*m1_param).m1_param_imag)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
            row = 0 as core::ffi::c_int as WORD32;
            while row < num_v_channels {
                let mut ptr_r2: *mut WORD32 = ptr_r1;
                let mut ptr_i2: *mut WORD32 = ptr_i1;
                col = 0 as core::ffi::c_int as WORD32;
                while col < num_x_channels {
                    param_r = ptr_r2;
                    param_i = ptr_i2;
                    m1_param_real_prev = m1_param_real_prev.offset(res_bands as isize);
                    m1_param_imag_prev = m1_param_imag_prev.offset(res_bands as isize);
                    pb = res_bands;
                    while pb < num_parameter_bands {
                        if smooth_band_arr[0 as core::ffi::c_int as usize][pb as usize]
                            != 0
                        {
                            let mut acc: WORD64 = 0;
                            acc = *param_r.offset(pb as isize) as WORD64
                                * *delta_ptr as WORD64
                                + *m1_param_real_prev as WORD64
                                    * *one_minus_delta_ptr as WORD64;
                            acc >>= 15 as core::ffi::c_int;
                            *param_r.offset(pb as isize) = acc as WORD32;
                            acc = *param_i.offset(pb as isize) as WORD64
                                * *delta_ptr as WORD64
                                + *m1_param_imag_prev as WORD64
                                    * *one_minus_delta_ptr as WORD64;
                            acc >>= 15 as core::ffi::c_int;
                            *param_i.offset(pb as isize) = acc as WORD32;
                        }
                        m1_param_real_prev = m1_param_real_prev.offset(1);
                        m1_param_imag_prev = m1_param_imag_prev.offset(1);
                        pb += 1;
                    }
                    param_r = param_r.offset(MAX_PARAMETER_BANDS as isize);
                    param_i = param_i.offset(MAX_PARAMETER_BANDS as isize);
                    ps = 1 as core::ffi::c_int as WORD32;
                    while ps < num_parameter_sets {
                        let mut del: WORD32 = *delta_ptr.offset(ps as isize);
                        let mut one_minus_del: WORD32 = *one_minus_delta_ptr
                            .offset(ps as isize);
                        param_prev_r = param_r.offset(-(MAX_PARAMETER_BANDS as isize));
                        param_prev_i = param_i.offset(-(MAX_PARAMETER_BANDS as isize));
                        pb = res_bands;
                        while pb < num_parameter_bands {
                            if smooth_band_arr[ps as usize][pb as usize] != 0 {
                                let mut acc_0: WORD64 = 0;
                                acc_0 = *param_r.offset(pb as isize) as WORD64
                                    * del as WORD64
                                    + *param_prev_r.offset(pb as isize) as WORD64
                                        * one_minus_del as WORD64;
                                acc_0 >>= 15 as core::ffi::c_int;
                                *param_r.offset(pb as isize) = acc_0 as WORD32;
                                acc_0 = *param_i.offset(pb as isize) as WORD64
                                    * del as WORD64
                                    + *param_prev_i.offset(pb as isize) as WORD64
                                        * one_minus_del as WORD64;
                                acc_0 >>= 15 as core::ffi::c_int;
                                *param_i.offset(pb as isize) = acc_0 as WORD32;
                            }
                            pb += 1;
                        }
                        param_r = param_r.offset(MAX_PARAMETER_BANDS as isize);
                        param_i = param_i.offset(MAX_PARAMETER_BANDS as isize);
                        ps += 1;
                    }
                    ptr_r2 = ptr_r2.offset(PBXPS as isize);
                    ptr_i2 = ptr_i2.offset(PBXPS as isize);
                    col += 1;
                }
                ptr_r1 = ptr_r1.offset(INCHXPBXPS as isize);
                ptr_i1 = ptr_i1.offset(INCHXPBXPS as isize);
                row += 1;
            }
        } else {
            let mut ptr1: *mut WORD32 = m1_param as *mut WORD32;
            row = 0 as core::ffi::c_int as WORD32;
            while row < num_v_channels {
                let mut ptr2: *mut WORD32 = ptr1;
                col = 0 as core::ffi::c_int as WORD32;
                while col < num_x_channels {
                    let mut param_r_0: *mut WORD32 = ptr2;
                    let mut del_0: WORD32 = *delta_ptr
                        .offset(0 as core::ffi::c_int as isize);
                    let mut one_minus_del_0: WORD32 = *one_minus_delta_ptr
                        .offset(0 as core::ffi::c_int as isize);
                    m1_param_real_prev = m1_param_real_prev.offset(res_bands as isize);
                    pb = res_bands;
                    while pb < num_parameter_bands {
                        if smooth_band_arr[0 as core::ffi::c_int as usize][pb as usize]
                            != 0
                        {
                            let mut acc_1: WORD64 = 0;
                            acc_1 = *param_r_0.offset(pb as isize) as WORD64
                                * del_0 as WORD64
                                + *m1_param_real_prev as WORD64 * one_minus_del_0 as WORD64;
                            *param_r_0.offset(pb as isize) = (acc_1
                                >> 15 as core::ffi::c_int) as WORD32;
                        }
                        m1_param_real_prev = m1_param_real_prev.offset(1);
                        pb += 1;
                    }
                    param_r_0 = param_r_0.offset(MAX_PARAMETER_BANDS as isize);
                    ps = 1 as core::ffi::c_int as WORD32;
                    while ps < num_parameter_sets {
                        let mut del_1: WORD32 = *delta_ptr.offset(ps as isize);
                        let mut one_minus_del_1: WORD32 = *one_minus_delta_ptr
                            .offset(ps as isize);
                        param_prev_r = param_r_0.offset(-(MAX_PARAMETER_BANDS as isize));
                        pb = res_bands;
                        while pb < num_parameter_bands {
                            if smooth_band_arr[ps as usize][pb as usize] != 0 {
                                let mut acc_2: WORD64 = 0;
                                acc_2 = *param_r_0.offset(pb as isize) as WORD64
                                    * del_1 as WORD64
                                    + *param_prev_r.offset(pb as isize) as WORD64
                                        * one_minus_del_1 as WORD64;
                                *param_r_0.offset(pb as isize) = (acc_2
                                    >> 15 as core::ffi::c_int) as WORD32;
                            }
                            pb += 1;
                        }
                        param_r_0 = param_r_0.offset(MAX_PARAMETER_BANDS as isize);
                        ps += 1;
                    }
                    ptr2 = ptr2.offset(PBXPS as isize);
                    col += 1;
                }
                ptr1 = ptr1.offset(INCHXPBXPS as isize);
                row += 1;
            }
        }
        if (*curr_state).residual_coding != 0 {
            resid_col_counter = col_counter;
        } else {
            resid_col_counter = num_direct_signals;
        }
        idx = 0 as core::ffi::c_int as WORD32;
        if m2_param_imag_present != 0 {
            let mut ptr_r1_0: *mut WORD32 = &mut *(*(*((*m2_param).m2_resid_real)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
            let mut ptr_i1_0: *mut WORD32 = &mut *(*(*((*m2_param).m2_resid_imag)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
            row = 0 as core::ffi::c_int as WORD32;
            while row < num_output_channels {
                col = 0 as core::ffi::c_int as WORD32;
                while col < resid_col_counter {
                    if (*curr_state).m2_param_present[row as usize][col as usize]
                        & 2 as core::ffi::c_int != 0
                    {
                        let mut del_2: WORD32 = *delta_ptr;
                        let mut one_minus_del_2: WORD32 = *one_minus_delta_ptr;
                        param_r = ptr_r1_0;
                        param_i = ptr_i1_0;
                        m2_resid_real_prev = m2_resid_real_prev
                            .offset(res_bands as isize);
                        m2_resid_imag_prev = m2_resid_imag_prev
                            .offset(res_bands as isize);
                        pb = res_bands;
                        while pb < num_parameter_bands {
                            if smooth_band_arr[0 as core::ffi::c_int
                                as usize][pb as usize] != 0
                            {
                                let mut acc_3: WORD64 = 0;
                                acc_3 = *param_r.offset(pb as isize) as WORD64
                                    * del_2 as WORD64
                                    + *m2_resid_real_prev as WORD64 * one_minus_del_2 as WORD64;
                                acc_3 >>= 15 as core::ffi::c_int;
                                *param_r.offset(pb as isize) = acc_3 as WORD32;
                                acc_3 = *param_i.offset(pb as isize) as WORD64
                                    * del_2 as WORD64
                                    + *m2_resid_imag_prev as WORD64 * one_minus_del_2 as WORD64;
                                acc_3 >>= 15 as core::ffi::c_int;
                                *param_i.offset(pb as isize) = acc_3 as WORD32;
                            }
                            m2_resid_real_prev = m2_resid_real_prev.offset(1);
                            m2_resid_imag_prev = m2_resid_imag_prev.offset(1);
                            pb += 1;
                        }
                        param_r = param_r.offset(MAX_PARAMETER_BANDS as isize);
                        param_i = param_i.offset(MAX_PARAMETER_BANDS as isize);
                        ps = 1 as core::ffi::c_int as WORD32;
                        while ps < num_parameter_sets {
                            let mut del_3: WORD32 = *delta_ptr.offset(ps as isize);
                            let mut one_minus_del_3: WORD32 = *one_minus_delta_ptr
                                .offset(ps as isize);
                            param_prev_r = param_r
                                .offset(-(MAX_PARAMETER_BANDS as isize));
                            param_prev_i = param_i
                                .offset(-(MAX_PARAMETER_BANDS as isize));
                            pb = res_bands;
                            while pb < num_parameter_bands {
                                if smooth_band_arr[ps as usize][pb as usize] != 0 {
                                    let mut acc_4: WORD64 = 0;
                                    acc_4 = *param_r.offset(pb as isize) as WORD64
                                        * del_3 as WORD64
                                        + *param_prev_r.offset(pb as isize) as WORD64
                                            * one_minus_del_3 as WORD64;
                                    acc_4 >>= 15 as core::ffi::c_int;
                                    *param_r.offset(pb as isize) = acc_4 as WORD32;
                                    acc_4 = *param_i.offset(pb as isize) as WORD64
                                        * del_3 as WORD64
                                        + *param_prev_i.offset(pb as isize) as WORD64
                                            * one_minus_del_3 as WORD64;
                                    acc_4 >>= 15 as core::ffi::c_int;
                                    *param_i.offset(pb as isize) = acc_4 as WORD32;
                                }
                                pb += 1;
                            }
                            param_r = param_r.offset(MAX_PARAMETER_BANDS as isize);
                            param_i = param_i.offset(MAX_PARAMETER_BANDS as isize);
                            ps += 1;
                        }
                        idx += 1;
                        ptr_r1_0 = ptr_r1_0.offset(PBXPS as isize);
                        ptr_i1_0 = ptr_i1_0.offset(PBXPS as isize);
                    }
                    col += 1;
                }
                row += 1;
            }
            idx = 0 as core::ffi::c_int as WORD32;
            ptr_r1_0 = &mut *(*(*((*m2_param).m2_resid_real)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
            ptr_i1_0 = &mut *(*(*((*m2_param).m2_resid_imag)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
            row = 0 as core::ffi::c_int as WORD32;
            while row < num_output_channels {
                col = num_direct_signals;
                while col < col_counter {
                    if (*curr_state).m2_param_present[row as usize][col as usize]
                        & 1 as core::ffi::c_int != 0
                    {
                        let mut del_4: WORD32 = *delta_ptr;
                        let mut one_minus_del_4: WORD32 = *one_minus_delta_ptr;
                        m2_decor_real_prev = m2_decor_real_prev
                            .offset(res_bands as isize);
                        m2_decor_imag_prev = m2_decor_imag_prev
                            .offset(res_bands as isize);
                        param_r = ptr_r1_0;
                        param_i = ptr_i1_0;
                        pb = res_bands;
                        while pb < num_parameter_bands {
                            if smooth_band_arr[0 as core::ffi::c_int
                                as usize][pb as usize] != 0
                            {
                                let mut acc_5: WORD64 = 0;
                                acc_5 = *param_r.offset(pb as isize) as WORD64
                                    * del_4 as WORD64
                                    + *m2_decor_real_prev as WORD64 * one_minus_del_4 as WORD64;
                                acc_5 >>= 15 as core::ffi::c_int;
                                *param_r.offset(pb as isize) = acc_5 as WORD32;
                                acc_5 = *param_i.offset(pb as isize) as WORD64
                                    * del_4 as WORD64
                                    + *m2_decor_imag_prev as WORD64 * one_minus_del_4 as WORD64;
                                acc_5 >>= 15 as core::ffi::c_int;
                                *param_i.offset(pb as isize) = acc_5 as WORD32;
                            }
                            m2_decor_real_prev = m2_decor_real_prev.offset(1);
                            m2_decor_imag_prev = m2_decor_imag_prev.offset(1);
                            pb += 1;
                        }
                        param_r = param_r.offset(MAX_PARAMETER_BANDS as isize);
                        param_i = param_i.offset(MAX_PARAMETER_BANDS as isize);
                        ps = 1 as core::ffi::c_int as WORD32;
                        while ps < num_parameter_sets {
                            let mut del_5: WORD32 = *delta_ptr.offset(ps as isize);
                            let mut one_minus_del_5: WORD32 = *one_minus_delta_ptr
                                .offset(ps as isize);
                            param_prev_r = param_r
                                .offset(-(MAX_PARAMETER_BANDS as isize));
                            param_prev_i = param_i
                                .offset(-(MAX_PARAMETER_BANDS as isize));
                            pb = res_bands;
                            while pb < num_parameter_bands {
                                if smooth_band_arr[ps as usize][pb as usize] != 0 {
                                    let mut acc_6: WORD64 = 0;
                                    acc_6 = *param_r.offset(pb as isize) as WORD64
                                        * del_5 as WORD64
                                        + *param_prev_r.offset(pb as isize) as WORD64
                                            * one_minus_del_5 as WORD64;
                                    acc_6 >>= 15 as core::ffi::c_int;
                                    *param_r.offset(pb as isize) = acc_6 as WORD32;
                                    acc_6 = *param_i.offset(pb as isize) as WORD64
                                        * del_5 as WORD64
                                        + *param_prev_i.offset(pb as isize) as WORD64
                                            * one_minus_del_5 as WORD64;
                                    acc_6 >>= 15 as core::ffi::c_int;
                                    *param_i.offset(pb as isize) = acc_6 as WORD32;
                                }
                                pb += 1;
                            }
                            param_r = param_r.offset(MAX_PARAMETER_BANDS as isize);
                            param_i = param_i.offset(MAX_PARAMETER_BANDS as isize);
                            ps += 1;
                        }
                        idx += 1;
                        ptr_r1_0 = ptr_r1_0.offset(PBXPS as isize);
                        ptr_i1_0 = ptr_i1_0.offset(PBXPS as isize);
                    }
                    col += 1;
                }
                row += 1;
            }
        } else {
            let mut ptr1_0: *mut WORD32 = &mut *(*(*((*m2_param).m2_resid_real)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
            row = 0 as core::ffi::c_int as WORD32;
            while row < num_output_channels {
                col = 0 as core::ffi::c_int as WORD32;
                while col < resid_col_counter {
                    if (*curr_state).m2_param_present[row as usize][col as usize]
                        & 2 as core::ffi::c_int != 0
                    {
                        let mut ptr2_0: *mut WORD32 = ptr1_0;
                        let mut del_6: WORD32 = *delta_ptr;
                        let mut one_minus_del_6: WORD32 = *one_minus_delta_ptr;
                        m2_resid_real_prev = m2_resid_real_prev
                            .offset(res_bands as isize);
                        pb = res_bands;
                        while pb < num_parameter_bands {
                            if smooth_band_arr[0 as core::ffi::c_int
                                as usize][pb as usize] != 0
                            {
                                let mut acc_7: WORD64 = 0;
                                acc_7 = *ptr2_0.offset(pb as isize) as WORD64
                                    * del_6 as WORD64
                                    + *m2_resid_real_prev as WORD64 * one_minus_del_6 as WORD64;
                                acc_7 >>= 15 as core::ffi::c_int;
                                *ptr2_0.offset(pb as isize) = acc_7 as WORD32;
                            }
                            m2_resid_real_prev = m2_resid_real_prev.offset(1);
                            pb += 1;
                        }
                        ptr2_0 = ptr2_0.offset(MAX_PARAMETER_BANDS as isize);
                        ps = 1 as core::ffi::c_int as WORD32;
                        while ps < num_parameter_sets {
                            let mut del_7: WORD32 = *delta_ptr.offset(ps as isize);
                            let mut one_minus_del_7: WORD32 = *one_minus_delta_ptr
                                .offset(ps as isize);
                            param_prev_r = ptr2_0
                                .offset(-(MAX_PARAMETER_BANDS as isize));
                            pb = res_bands;
                            while pb < num_parameter_bands {
                                if smooth_band_arr[ps as usize][pb as usize] != 0 {
                                    let mut acc_8: WORD64 = 0;
                                    acc_8 = *ptr2_0.offset(pb as isize) as WORD64
                                        * del_7 as WORD64
                                        + *param_prev_r as WORD64 * one_minus_del_7 as WORD64;
                                    acc_8 >>= 15 as core::ffi::c_int;
                                    *ptr2_0.offset(pb as isize) = acc_8 as WORD32;
                                }
                                param_prev_r = param_prev_r.offset(1);
                                pb += 1;
                            }
                            ptr2_0 = ptr2_0.offset(MAX_PARAMETER_BANDS as isize);
                            ps += 1;
                        }
                        idx += 1;
                        ptr1_0 = ptr1_0.offset(PBXPS as isize);
                    }
                    col += 1;
                }
                row += 1;
            }
            idx = 0 as core::ffi::c_int as WORD32;
            ptr1_0 = &mut *(*(*((*m2_param).m2_decor_real)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
            row = 0 as core::ffi::c_int as WORD32;
            while row < num_output_channels {
                col = num_direct_signals;
                while col < col_counter {
                    if (*curr_state).m2_param_present[row as usize][col as usize]
                        & 1 as core::ffi::c_int != 0
                    {
                        let mut ptr2_1: *mut WORD32 = ptr1_0;
                        m2_decor_real_prev = m2_decor_real_prev
                            .offset(res_bands as isize);
                        param_r = &mut *(*(*((*m2_param).m2_decor_real)
                            .as_mut_ptr()
                            .offset(idx as isize))
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(res_bands as isize) as *mut WORD32;
                        pb = res_bands;
                        while pb < num_parameter_bands {
                            if smooth_band_arr[0 as core::ffi::c_int
                                as usize][pb as usize] != 0
                            {
                                let mut acc_9: WORD64 = 0;
                                acc_9 = *ptr2_1.offset(pb as isize) as WORD64
                                    * *delta_ptr as WORD64
                                    + *m2_decor_real_prev as WORD64
                                        * *one_minus_delta_ptr as WORD64;
                                acc_9 >>= 15 as core::ffi::c_int;
                                *ptr2_1.offset(pb as isize) = acc_9 as WORD32;
                            }
                            m2_decor_real_prev = m2_decor_real_prev.offset(1);
                            pb += 1;
                        }
                        ptr2_1 = ptr2_1.offset(MAX_PARAMETER_BANDS as isize);
                        ps = 1 as core::ffi::c_int as WORD32;
                        while ps < num_parameter_sets {
                            let mut del_8: WORD32 = *delta_ptr.offset(ps as isize);
                            let mut one_minus_del_8: WORD32 = *one_minus_delta_ptr
                                .offset(ps as isize);
                            param_prev_r = ptr2_1
                                .offset(-(MAX_PARAMETER_BANDS as isize));
                            pb = res_bands;
                            while pb < num_parameter_bands {
                                if smooth_band_arr[ps as usize][pb as usize] != 0 {
                                    let mut acc_10: WORD64 = 0;
                                    acc_10 = *ptr2_1.offset(pb as isize) as WORD64
                                        * del_8 as WORD64
                                        + *param_prev_r as WORD64 * one_minus_del_8 as WORD64;
                                    acc_10 >>= 15 as core::ffi::c_int;
                                    *ptr2_1.offset(pb as isize) = acc_10 as WORD32;
                                }
                                param_prev_r = param_prev_r.offset(1);
                                pb += 1;
                            }
                            ptr2_1 = ptr2_1.offset(MAX_PARAMETER_BANDS as isize);
                            ps += 1;
                        }
                        idx += 1;
                        ptr1_0 = ptr1_0.offset(PBXPS as isize);
                    }
                    col += 1;
                }
                row += 1;
            }
        }
    }
}
