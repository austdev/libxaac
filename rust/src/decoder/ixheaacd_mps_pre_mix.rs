extern "C" {
    fn cos(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn sin(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
    fn ixheaacd_mps_smoothing_opd(self_0: *mut ia_mps_dec_state_struct) -> VOID;
    static ixheaacd_atan_table_Q28: [[[WORD32; 31]; 8]; 16];
    static ixheaacd_ipd_de_quant_table_q28: [WORD32; 16];
    static ixheaacd_im_weight: [[[FLOAT32; 31]; 8]; 16];
    static ixheaacd_re_weight: [[[FLOAT32; 31]; 8]; 16];
    static ixheaacd_beta: [[[FLOAT32; 31]; 8]; 16];
    static ixheaacd_weight: [[[FLOAT32; 31]; 8]; 16];
    static ixheaacd_c_l_table: [FLOAT32; 31];
    static ixheaacd_sin_table: [[FLOAT32; 31]; 8];
    static ixheaacd_cos_table: [[FLOAT32; 31]; 8];
    static ixheaacd_mps_gain_set_indx: [WORD32; 29];
}
pub type size_t = usize;
pub type WORD8 = core::ffi::c_schar;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD16 = core::ffi::c_short;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type FLOAT32 = core::ffi::c_float;
pub type VOID = ();
pub type FLAG = core::ffi::c_int;
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
pub const __ASSERT_FUNCTION: [core::ffi::c_char; 42] = unsafe {
    ::core::mem::transmute::<
        [u8; 42],
        [core::ffi::c_char; 42],
    >(*b"WORD32 ixheaacd_mps_phase_wraping(WORD32)\0")
};
pub const MAX_PARAMETER_BANDS: core::ffi::c_int = 28 as core::ffi::c_int;
pub const MAX_M_INPUT: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_M_OUTPUT: core::ffi::c_int = 2 as core::ffi::c_int;
pub const P_PI: core::ffi::c_double = 3.1415926535897932f64;
pub const PI_IN_Q28: core::ffi::c_int = 843314880 as core::ffi::c_int;
pub const Q28_FLOAT_VAL: FLOAT32 = ((1 as core::ffi::c_int) << 28 as core::ffi::c_int)
    as FLOAT32;
pub const ONE_BY_Q28_FLOAT_VAL: FLOAT32 = 1.0f32 / Q28_FLOAT_VAL;
unsafe extern "C" fn ixheaacd_mps_phase_wraping(mut phase: WORD32) -> WORD32 {
    let pi_2: WORD32 = 2 as WORD32 * PI_IN_Q28;
    while phase < 0 as core::ffi::c_int {
        phase += pi_2;
    }
    while phase >= pi_2 {
        phase -= pi_2;
    }
    if phase >= 0 as core::ffi::c_int && phase < pi_2 {} else {
        __assert_fail(
            b"(phase >= 0) && (phase < pi_2)\0" as *const u8 as *const core::ffi::c_char,
            b"/mnt/c/Users/bohdan.bolbot/Repositories/libxaac/decoder/ixheaacd_mps_pre_mix.c\0"
                as *const u8 as *const core::ffi::c_char,
            68 as core::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    }
    'c_6158: {
        if phase >= 0 as core::ffi::c_int && phase < pi_2 {} else {
            __assert_fail(
                b"(phase >= 0) && (phase < pi_2)\0" as *const u8
                    as *const core::ffi::c_char,
                b"/mnt/c/Users/bohdan.bolbot/Repositories/libxaac/decoder/ixheaacd_mps_pre_mix.c\0"
                    as *const u8 as *const core::ffi::c_char,
                68 as core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    return phase;
}
unsafe extern "C" fn ixheaacd_mps_buffer_pre_and_mix_matrix(
    mut self_0: *mut ia_mps_dec_state_struct,
) -> VOID {
    let mut pb: WORD32 = 0;
    let mut row: WORD32 = 0;
    let mut col: WORD32 = 0;
    pb = 0 as core::ffi::c_int as WORD32;
    while pb < (*self_0).bs_param_bands {
        row = 0 as core::ffi::c_int as WORD32;
        while row < MAX_M_INPUT {
            col = 0 as core::ffi::c_int as WORD32;
            while col < MAX_M_OUTPUT {
                (*self_0).m1_param_re_prev[pb as usize][row as usize][col as usize] = (*self_0)
                    .m1_param_re[((*self_0).num_parameter_sets_prev as core::ffi::c_int
                    - 1 as core::ffi::c_int)
                    as usize][pb as usize][row as usize][col as usize];
                (*self_0).m1_param_im_prev[pb as usize][row as usize][col as usize] = (*self_0)
                    .m1_param_im[((*self_0).num_parameter_sets_prev as core::ffi::c_int
                    - 1 as core::ffi::c_int)
                    as usize][pb as usize][row as usize][col as usize];
                (*self_0).m2_decor_re_prev[pb as usize][row as usize][col as usize] = (*self_0)
                    .m2_decor_re[((*self_0).num_parameter_sets_prev as core::ffi::c_int
                    - 1 as core::ffi::c_int)
                    as usize][pb as usize][row as usize][col as usize];
                (*self_0).m2_decor_im_prev[pb as usize][row as usize][col as usize] = (*self_0)
                    .m2_decor_im[((*self_0).num_parameter_sets_prev as core::ffi::c_int
                    - 1 as core::ffi::c_int)
                    as usize][pb as usize][row as usize][col as usize];
                (*self_0).m2_resid_re_prev[pb as usize][row as usize][col as usize] = (*self_0)
                    .m2_resid_re[((*self_0).num_parameter_sets_prev as core::ffi::c_int
                    - 1 as core::ffi::c_int)
                    as usize][pb as usize][row as usize][col as usize];
                (*self_0).m2_resid_im_prev[pb as usize][row as usize][col as usize] = (*self_0)
                    .m2_resid_im[((*self_0).num_parameter_sets_prev as core::ffi::c_int
                    - 1 as core::ffi::c_int)
                    as usize][pb as usize][row as usize][col as usize];
                col += 1;
            }
            row += 1;
        }
        pb += 1;
    }
    pb = 0 as core::ffi::c_int as WORD32;
    while pb < (*self_0).bs_param_bands {
        (*self_0).phase_l_prev[pb as usize] = (*self_0)
            .phase_l[((*self_0).num_parameter_sets_prev as core::ffi::c_int
            - 1 as core::ffi::c_int) as usize][pb as usize];
        (*self_0).phase_r_prev[pb as usize] = (*self_0)
            .phase_r[((*self_0).num_parameter_sets_prev as core::ffi::c_int
            - 1 as core::ffi::c_int) as usize][pb as usize];
        pb += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_pre_and_mix_matrix_calculation(
    mut self_0: *mut ia_mps_dec_state_struct,
) -> VOID {
    let mut ps: WORD32 = 0;
    let mut pb: WORD32 = 0;
    let mut curr_bit_stream: *mut ia_mps_bs_frame = &mut (*self_0).bs_frame;
    let mut h_imag: [FLOAT32; 56] = [0.; 56];
    let mut h_real: [FLOAT32; 168] = [0.; 168];
    ixheaacd_mps_buffer_pre_and_mix_matrix(self_0);
    ps = 0 as core::ffi::c_int as WORD32;
    while ps < (*self_0).num_parameter_sets {
        let mut h_im: *mut FLOAT32 = &mut *h_imag
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
        let mut h_re: *mut FLOAT32 = &mut *h_real
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
        memset(
            h_real.as_mut_ptr() as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ((6 as core::ffi::c_int * MAX_PARAMETER_BANDS) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        memset(
            h_imag.as_mut_ptr() as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ((2 as core::ffi::c_int * MAX_PARAMETER_BANDS) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        match (*(*self_0).config).bs_phase_coding {
            0 => {
                if (*self_0).residual_coding != 0 {
                    ixheaacd_mps_par2umx_pred(
                        self_0,
                        curr_bit_stream,
                        h_imag.as_mut_ptr(),
                        h_real.as_mut_ptr(),
                        ps,
                        (*self_0).res_bands,
                    );
                } else {
                    ixheaacd_mps_par2umx_ps(
                        self_0,
                        curr_bit_stream,
                        h_real.as_mut_ptr(),
                        ps,
                    );
                }
            }
            1 => {
                ixheaacd_mps_par2umx_ps_ipd_opd(
                    self_0,
                    curr_bit_stream,
                    h_real.as_mut_ptr(),
                    ps,
                );
            }
            2 => {
                ixheaacd_mps_par2umx_pred(
                    self_0,
                    curr_bit_stream,
                    h_imag.as_mut_ptr(),
                    h_real.as_mut_ptr(),
                    ps,
                    (*self_0).res_bands,
                );
            }
            _ => {}
        }
        pb = 0 as core::ffi::c_int as WORD32;
        while pb < (*self_0).bs_param_bands {
            (*self_0)
                .m1_param_re[ps
                as usize][pb
                as usize][0 as core::ffi::c_int
                as usize][0 as core::ffi::c_int as usize] = 1.0f32 as FLOAT32;
            (*self_0)
                .m1_param_re[ps
                as usize][pb
                as usize][1 as core::ffi::c_int
                as usize][0 as core::ffi::c_int as usize] = 1.0f32 as FLOAT32;
            (*self_0)
                .m1_param_im[ps
                as usize][pb
                as usize][0 as core::ffi::c_int
                as usize][0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
                as FLOAT32;
            (*self_0)
                .m1_param_im[ps
                as usize][pb
                as usize][1 as core::ffi::c_int
                as usize][0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
                as FLOAT32;
            let fresh0 = h_re;
            h_re = h_re.offset(1);
            (*self_0)
                .m2_resid_re[ps
                as usize][pb
                as usize][0 as core::ffi::c_int
                as usize][0 as core::ffi::c_int as usize] = *fresh0;
            let fresh1 = h_im;
            h_im = h_im.offset(1);
            (*self_0)
                .m2_resid_im[ps
                as usize][pb
                as usize][0 as core::ffi::c_int
                as usize][0 as core::ffi::c_int as usize] = *fresh1;
            (*self_0)
                .m2_resid_im[ps
                as usize][pb
                as usize][0 as core::ffi::c_int
                as usize][1 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
                as FLOAT32;
            let fresh2 = h_re;
            h_re = h_re.offset(1);
            (*self_0)
                .m2_resid_re[ps
                as usize][pb
                as usize][1 as core::ffi::c_int
                as usize][0 as core::ffi::c_int as usize] = *fresh2;
            let fresh3 = h_im;
            h_im = h_im.offset(1);
            (*self_0)
                .m2_resid_im[ps
                as usize][pb
                as usize][1 as core::ffi::c_int
                as usize][0 as core::ffi::c_int as usize] = *fresh3;
            (*self_0)
                .m2_resid_im[ps
                as usize][pb
                as usize][1 as core::ffi::c_int
                as usize][1 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
                as FLOAT32;
            (*self_0)
                .m2_decor_re[ps
                as usize][pb
                as usize][0 as core::ffi::c_int
                as usize][0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
                as FLOAT32;
            (*self_0)
                .m2_decor_im[ps
                as usize][pb
                as usize][0 as core::ffi::c_int
                as usize][0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
                as FLOAT32;
            let fresh4 = h_re;
            h_re = h_re.offset(1);
            (*self_0)
                .m2_decor_re[ps
                as usize][pb
                as usize][0 as core::ffi::c_int
                as usize][1 as core::ffi::c_int as usize] = *fresh4;
            (*self_0)
                .m2_decor_im[ps
                as usize][pb
                as usize][0 as core::ffi::c_int
                as usize][1 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
                as FLOAT32;
            (*self_0)
                .m2_decor_re[ps
                as usize][pb
                as usize][1 as core::ffi::c_int
                as usize][0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
                as FLOAT32;
            (*self_0)
                .m2_decor_im[ps
                as usize][pb
                as usize][1 as core::ffi::c_int
                as usize][0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
                as FLOAT32;
            let fresh5 = h_re;
            h_re = h_re.offset(1);
            (*self_0)
                .m2_decor_re[ps
                as usize][pb
                as usize][1 as core::ffi::c_int
                as usize][1 as core::ffi::c_int as usize] = *fresh5;
            (*self_0)
                .m2_decor_im[ps
                as usize][pb
                as usize][1 as core::ffi::c_int
                as usize][1 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
                as FLOAT32;
            let fresh6 = h_re;
            h_re = h_re.offset(1);
            (*self_0)
                .m2_resid_re[ps
                as usize][pb
                as usize][0 as core::ffi::c_int
                as usize][1 as core::ffi::c_int as usize] = *fresh6;
            let fresh7 = h_re;
            h_re = h_re.offset(1);
            (*self_0)
                .m2_resid_re[ps
                as usize][pb
                as usize][1 as core::ffi::c_int
                as usize][1 as core::ffi::c_int as usize] = *fresh7;
            pb += 1;
        }
        ps += 1;
    }
    ixheaacd_mps_smoothing_opd(self_0);
}
unsafe extern "C" fn ixheaacd_mps_par2umx_ps_core(
    mut cld: *mut WORD32,
    mut icc: *mut WORD32,
    mut ott_band_count: WORD32,
    mut h_real: *mut FLOAT32,
) -> VOID {
    let mut band: WORD32 = 0;
    let mut c_l_temp: FLOAT32 = 0.;
    let mut c_r_temp: FLOAT32 = 0.;
    let mut temp: FLOAT32 = 0.;
    let mut cld_idx: WORD32 = 0;
    let mut icc_idx: WORD32 = 0;
    band = 0 as core::ffi::c_int as WORD32;
    while band < ott_band_count {
        let fresh24 = cld;
        cld = cld.offset(1);
        cld_idx = (*fresh24 + 15 as core::ffi::c_int) as WORD32;
        let fresh25 = icc;
        icc = icc.offset(1);
        icc_idx = *fresh25;
        icc_idx = (icc_idx as core::ffi::c_int & 7 as core::ffi::c_int) as WORD32;
        c_l_temp = ixheaacd_c_l_table[cld_idx as usize];
        c_r_temp = ixheaacd_c_l_table[(30 as WORD32 - cld_idx) as usize];
        temp = ixheaacd_cos_table[icc_idx as usize][cld_idx as usize];
        let fresh26 = h_real;
        h_real = h_real.offset(1);
        *fresh26 = temp * c_l_temp;
        temp = ixheaacd_cos_table[icc_idx as usize][(30 as WORD32 - cld_idx) as usize];
        let fresh27 = h_real;
        h_real = h_real.offset(1);
        *fresh27 = temp * c_r_temp;
        temp = ixheaacd_sin_table[icc_idx as usize][cld_idx as usize];
        let fresh28 = h_real;
        h_real = h_real.offset(1);
        *fresh28 = temp * c_l_temp;
        temp = -ixheaacd_sin_table[icc_idx as usize][(30 as WORD32 - cld_idx) as usize];
        let fresh29 = h_real;
        h_real = h_real.offset(1);
        *fresh29 = temp * c_r_temp;
        h_real = h_real.offset(2 as core::ffi::c_int as isize);
        band += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_par2umx_ps(
    mut self_0: *mut ia_mps_dec_state_struct,
    mut curr_bit_stream: *mut ia_mps_bs_frame,
    mut h_real: *mut FLOAT32,
    mut param_set_idx: WORD32,
) -> VOID {
    ixheaacd_mps_par2umx_ps_core(
        ((*curr_bit_stream).cld_idx[param_set_idx as usize]).as_mut_ptr(),
        ((*curr_bit_stream).icc_idx[param_set_idx as usize]).as_mut_ptr(),
        (*self_0).bs_param_bands,
        h_real,
    );
}
unsafe extern "C" fn ixheaacd_mps_opd_calc(
    mut self_0: *mut ia_mps_dec_state_struct,
    mut curr_bit_stream: *mut ia_mps_bs_frame,
    mut param_set_idx: WORD32,
    mut opd: *mut WORD32,
) -> VOID {
    let mut band: WORD32 = 0;
    band = 0 as core::ffi::c_int as WORD32;
    while band < (*self_0).num_bands_ipd {
        let mut cld_idx: WORD32 = (*curr_bit_stream)
            .cld_idx[param_set_idx as usize][band as usize] + 15 as WORD32;
        let mut ipd_idx: WORD32 = (*curr_bit_stream)
            .ipd_idx[param_set_idx as usize][band as usize] & 15 as WORD32;
        let mut icc_idx: WORD32 = (*curr_bit_stream)
            .icc_idx[param_set_idx as usize][band as usize];
        if cld_idx == 15 as core::ffi::c_int && ipd_idx == 8 as core::ffi::c_int {
            *opd.offset(band as isize) = 0 as core::ffi::c_int as WORD32;
        } else {
            *opd.offset(band as isize) = ixheaacd_atan_table_Q28[ipd_idx
                as usize][icc_idx as usize][cld_idx as usize];
        }
        band += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_par2umx_ps_ipd_opd(
    mut self_0: *mut ia_mps_dec_state_struct,
    mut curr_bit_stream: *mut ia_mps_bs_frame,
    mut h_real: *mut FLOAT32,
    mut param_set_idx: WORD32,
) -> VOID {
    let mut opd: [WORD32; 28] = [0; 28];
    let mut ott_band_count: WORD32 = (*self_0).bs_param_bands;
    let mut num_bands_ipd: WORD32 = (*self_0).num_bands_ipd;
    let mut band: WORD32 = 0;
    ixheaacd_mps_par2umx_ps_core(
        ((*curr_bit_stream).cld_idx[param_set_idx as usize]).as_mut_ptr(),
        ((*curr_bit_stream).icc_idx[param_set_idx as usize]).as_mut_ptr(),
        ott_band_count,
        h_real,
    );
    if (*self_0).bs_phase_mode != 0 {
        ixheaacd_mps_opd_calc(self_0, curr_bit_stream, param_set_idx, opd.as_mut_ptr());
        band = 0 as core::ffi::c_int as WORD32;
        while band < num_bands_ipd {
            let mut ipd_idx: WORD32 = (*curr_bit_stream)
                .ipd_idx[param_set_idx as usize][band as usize] & 15 as WORD32;
            let mut ipd: WORD32 = ixheaacd_ipd_de_quant_table_q28[ipd_idx as usize];
            (*self_0).phase_l[param_set_idx as usize][band as usize] = ixheaacd_mps_phase_wraping(
                opd[band as usize],
            ) as FLOAT32 * ONE_BY_Q28_FLOAT_VAL;
            (*self_0).phase_r[param_set_idx as usize][band as usize] = ixheaacd_mps_phase_wraping(
                opd[band as usize] - ipd,
            ) as FLOAT32 * ONE_BY_Q28_FLOAT_VAL;
            band += 1;
        }
    } else {
        num_bands_ipd = 0 as core::ffi::c_int as WORD32;
    }
    band = num_bands_ipd;
    while band < ott_band_count {
        (*self_0).phase_l[param_set_idx as usize][band as usize] = 0 as core::ffi::c_int
            as FLOAT32;
        (*self_0).phase_r[param_set_idx as usize][band as usize] = 0 as core::ffi::c_int
            as FLOAT32;
        band += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_par2umx_pred(
    mut self_0: *mut ia_mps_dec_state_struct,
    mut curr_bit_stream: *mut ia_mps_bs_frame,
    mut h_imag: *mut FLOAT32,
    mut h_real: *mut FLOAT32,
    mut param_set_idx: WORD32,
    mut res_bands: WORD32,
) -> VOID {
    let mut band: WORD32 = 0;
    band = 0 as core::ffi::c_int as WORD32;
    while band < (*self_0).bs_param_bands {
        let mut cld_idx: WORD32 = (*curr_bit_stream)
            .cld_idx[param_set_idx as usize][band as usize] + 15 as WORD32;
        let mut icc_idx: WORD32 = (*curr_bit_stream)
            .icc_idx[param_set_idx as usize][band as usize];
        let mut ipd_idx: WORD32 = (*curr_bit_stream)
            .ipd_idx[param_set_idx as usize][band as usize] & 15 as WORD32;
        if band < (*self_0).num_bands_ipd && cld_idx == 15 as core::ffi::c_int
            && icc_idx == 0 as core::ffi::c_int && ipd_idx == 8 as core::ffi::c_int
        {
            let mut gain: FLOAT32 = 0.416666667f32;
            let fresh8 = h_imag;
            h_imag = h_imag.offset(1);
            *fresh8 = 0 as core::ffi::c_int as FLOAT32;
            let fresh9 = h_imag;
            h_imag = h_imag.offset(1);
            *fresh9 = 0 as core::ffi::c_int as FLOAT32;
            if band < res_bands {
                let fresh10 = h_real;
                h_real = h_real.offset(1);
                *fresh10 = gain;
                let fresh11 = h_real;
                h_real = h_real.offset(1);
                *fresh11 = gain;
                h_real = h_real.offset(2 as core::ffi::c_int as isize);
                let fresh12 = h_real;
                h_real = h_real.offset(1);
                *fresh12 = gain;
                let fresh13 = h_real;
                h_real = h_real.offset(1);
                *fresh13 = -gain;
            } else {
                let fresh14 = h_real;
                h_real = h_real.offset(1);
                *fresh14 = gain;
                let fresh15 = h_real;
                h_real = h_real.offset(1);
                *fresh15 = -gain;
                h_real = h_real.offset(4 as core::ffi::c_int as isize);
            }
        } else {
            let mut weight: FLOAT32 = 0.;
            let mut re_weight: FLOAT32 = 0.;
            let mut im_weight: FLOAT32 = 0.;
            weight = ixheaacd_weight[ipd_idx
                as usize][icc_idx as usize][cld_idx as usize];
            re_weight = ixheaacd_re_weight[ipd_idx
                as usize][icc_idx as usize][cld_idx as usize];
            im_weight = ixheaacd_im_weight[ipd_idx
                as usize][icc_idx as usize][cld_idx as usize];
            if band < (*self_0).num_bands_ipd {
                weight = ixheaacd_weight[ipd_idx
                    as usize][icc_idx as usize][cld_idx as usize];
                re_weight = ixheaacd_re_weight[ipd_idx
                    as usize][icc_idx as usize][cld_idx as usize];
                im_weight = ixheaacd_im_weight[ipd_idx
                    as usize][icc_idx as usize][cld_idx as usize];
            } else {
                weight = ixheaacd_weight[0 as core::ffi::c_int
                    as usize][icc_idx as usize][cld_idx as usize];
                re_weight = ixheaacd_re_weight[0 as core::ffi::c_int
                    as usize][icc_idx as usize][cld_idx as usize];
                im_weight = ixheaacd_im_weight[0 as core::ffi::c_int
                    as usize][icc_idx as usize][cld_idx as usize];
            }
            let fresh16 = h_real;
            h_real = h_real.offset(1);
            *fresh16 = weight - re_weight;
            let fresh17 = h_imag;
            h_imag = h_imag.offset(1);
            *fresh17 = -im_weight;
            let fresh18 = h_real;
            h_real = h_real.offset(1);
            *fresh18 = weight + re_weight;
            let fresh19 = h_imag;
            h_imag = h_imag.offset(1);
            *fresh19 = im_weight;
            if band < res_bands {
                h_real = h_real.offset(2 as core::ffi::c_int as isize);
                let fresh20 = h_real;
                h_real = h_real.offset(1);
                *fresh20 = weight;
                let fresh21 = h_real;
                h_real = h_real.offset(1);
                *fresh21 = -weight;
            } else {
                let mut beta: FLOAT32 = ixheaacd_beta[ipd_idx
                    as usize][icc_idx as usize][cld_idx as usize];
                let fresh22 = h_real;
                h_real = h_real.offset(1);
                *fresh22 = beta;
                let fresh23 = h_real;
                h_real = h_real.offset(1);
                *fresh23 = -beta;
                h_real = h_real.offset(2 as core::ffi::c_int as isize);
            }
        }
        band += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_apply_pre_matrix(
    mut self_0: *mut ia_mps_dec_state_struct,
) -> VOID {
    let mut ts: WORD32 = 0;
    let mut qs: WORD32 = 0;
    let mut row: WORD32 = 0;
    if (*self_0).pre_mix_req != 0 {
        ixheaacd_mps_upmix_interp_type1(
            ((*self_0).m1_param_re).as_mut_ptr(),
            ((*self_0).r_out_re_in_m1).as_mut_ptr(),
            ((*self_0).m1_param_re_prev).as_mut_ptr(),
            (*self_0).dir_sig_count + (*self_0).decor_sig_count,
            1 as WORD32,
            self_0,
            (*self_0).bs_high_rate_mode,
        );
        ts = 0 as core::ffi::c_int as WORD32;
        while ts < (*self_0).time_slots {
            qs = 0 as core::ffi::c_int as WORD32;
            while qs < 2 as core::ffi::c_int {
                let mut indx: WORD32 = *((*self_0).hyb_band_to_processing_band_table)
                    .offset(qs as isize);
                let mut real: FLOAT32 = (*self_0)
                    .hyb_in[0 as core::ffi::c_int as usize][qs as usize][ts as usize]
                    .re
                    * (*self_0)
                        .r_out_re_in_m1[ts
                        as usize][indx
                        as usize][0 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize];
                let mut imag: FLOAT32 = (*self_0)
                    .hyb_in[0 as core::ffi::c_int as usize][qs as usize][ts as usize]
                    .im
                    * (*self_0)
                        .r_out_re_in_m1[ts
                        as usize][indx
                        as usize][0 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize];
                row = 0 as core::ffi::c_int as WORD32;
                while row < (*self_0).dir_sig_count + (*self_0).decor_sig_count {
                    (*self_0).v[row as usize][ts as usize][qs as usize].re = real;
                    (*self_0).v[row as usize][ts as usize][qs as usize].im = imag;
                    row += 1;
                }
                qs += 1;
            }
            qs = 2 as core::ffi::c_int as WORD32;
            while qs < (*self_0).hyb_band_count[0 as core::ffi::c_int as usize] {
                let mut indx_0: WORD32 = *((*self_0).hyb_band_to_processing_band_table)
                    .offset(qs as isize);
                let mut real_0: FLOAT32 = (*self_0)
                    .hyb_in[0 as core::ffi::c_int as usize][qs as usize][ts as usize]
                    .re
                    * (*self_0)
                        .r_out_re_in_m1[ts
                        as usize][indx_0
                        as usize][0 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize];
                let mut imag_0: FLOAT32 = (*self_0)
                    .hyb_in[0 as core::ffi::c_int as usize][qs as usize][ts as usize]
                    .im
                    * (*self_0)
                        .r_out_re_in_m1[ts
                        as usize][indx_0
                        as usize][0 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize];
                row = 0 as core::ffi::c_int as WORD32;
                while row < (*self_0).dir_sig_count + (*self_0).decor_sig_count {
                    (*self_0).v[row as usize][ts as usize][qs as usize].re = real_0;
                    (*self_0).v[row as usize][ts as usize][qs as usize].im = imag_0;
                    row += 1;
                }
                qs += 1;
            }
            ts += 1;
        }
    } else {
        ts = 0 as core::ffi::c_int as WORD32;
        while ts < (*self_0).time_slots {
            qs = 0 as core::ffi::c_int as WORD32;
            while qs < (*self_0).hyb_band_count[0 as core::ffi::c_int as usize] {
                let mut real_1: FLOAT32 = (*self_0)
                    .hyb_in[0 as core::ffi::c_int as usize][qs as usize][ts as usize]
                    .re;
                let mut imag_1: FLOAT32 = (*self_0)
                    .hyb_in[0 as core::ffi::c_int as usize][qs as usize][ts as usize]
                    .im;
                row = 0 as core::ffi::c_int as WORD32;
                while row < (*self_0).dir_sig_count + (*self_0).decor_sig_count {
                    (*self_0).v[row as usize][ts as usize][qs as usize].re = real_1;
                    (*self_0).v[row as usize][ts as usize][qs as usize].im = imag_1;
                    row += 1;
                }
                qs += 1;
            }
            ts += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_apply_mix_matrix(
    mut self_0: *mut ia_mps_dec_state_struct,
) -> VOID {
    let mut ts: WORD32 = 0;
    let mut qs: WORD32 = 0;
    let mut row: WORD32 = 0;
    let mut col: WORD32 = 0;
    let mut complex_m2: WORD32 = ((*(*self_0).config).bs_phase_coding != 0 as UINT32)
        as core::ffi::c_int;
    let mut phase_interpolation: WORD32 = ((*(*self_0).config).bs_phase_coding
        == 1 as UINT32) as core::ffi::c_int;
    let mut num_col_iters: WORD32 = 0 as WORD32;
    ixheaacd_mps_upmix_interp(
        ((*self_0).m2_decor_re).as_mut_ptr(),
        ((*self_0).r_out_diff_re_in_m2).as_mut_ptr(),
        ((*self_0).m2_decor_re_prev).as_mut_ptr(),
        (*self_0).out_ch_count,
        (*self_0).dir_sig_count + (*self_0).decor_sig_count,
        self_0,
        1 as WORD32,
    );
    ixheaacd_mps_upmix_interp(
        ((*self_0).m2_resid_re).as_mut_ptr(),
        ((*self_0).r_out_re_in_m2).as_mut_ptr(),
        ((*self_0).m2_resid_re_prev).as_mut_ptr(),
        (*self_0).out_ch_count,
        (*self_0).dir_sig_count + (*self_0).decor_sig_count,
        self_0,
        1 as WORD32,
    );
    if complex_m2 != 0 && phase_interpolation == 0 {
        ixheaacd_mps_upmix_interp(
            ((*self_0).m2_decor_im).as_mut_ptr(),
            ((*self_0).r_out_diff_im_in_m2).as_mut_ptr(),
            ((*self_0).m2_decor_im_prev).as_mut_ptr(),
            (*self_0).out_ch_count,
            (*self_0).dir_sig_count + (*self_0).decor_sig_count,
            self_0,
            1 as WORD32,
        );
        ixheaacd_mps_upmix_interp(
            ((*self_0).m2_resid_im).as_mut_ptr(),
            ((*self_0).r_out_im_in_m2).as_mut_ptr(),
            ((*self_0).m2_resid_im_prev).as_mut_ptr(),
            (*self_0).out_ch_count,
            (*self_0).dir_sig_count + (*self_0).decor_sig_count,
            self_0,
            1 as WORD32,
        );
    }
    if phase_interpolation != 0 {
        ixheaacd_mps_phase_interpolation(
            ((*self_0).phase_l).as_mut_ptr(),
            ((*self_0).phase_r).as_mut_ptr(),
            ((*self_0).phase_l_prev).as_mut_ptr(),
            ((*self_0).phase_r_prev).as_mut_ptr(),
            ((*self_0).r_out_ph_re_in_m2).as_mut_ptr(),
            ((*self_0).r_out_ph_im_in_m2).as_mut_ptr(),
            self_0,
        );
        ts = 0 as core::ffi::c_int as WORD32;
        while ts < (*self_0).time_slots {
            let mut pb: WORD32 = 0;
            pb = 0 as core::ffi::c_int as WORD32;
            while pb < (*self_0).bs_param_bands {
                (*self_0)
                    .r_out_im_in_m2[ts
                    as usize][pb
                    as usize][0 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize] = (*self_0)
                    .r_out_re_in_m2[ts
                    as usize][pb
                    as usize][0 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize]
                    * (*self_0)
                        .r_out_ph_im_in_m2[ts
                        as usize][pb as usize][0 as core::ffi::c_int as usize];
                (*self_0)
                    .r_out_im_in_m2[ts
                    as usize][pb
                    as usize][0 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize] = (*self_0)
                    .r_out_re_in_m2[ts
                    as usize][pb
                    as usize][0 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize]
                    * (*self_0)
                        .r_out_ph_im_in_m2[ts
                        as usize][pb as usize][0 as core::ffi::c_int as usize];
                (*self_0)
                    .r_out_im_in_m2[ts
                    as usize][pb
                    as usize][1 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize] = (*self_0)
                    .r_out_re_in_m2[ts
                    as usize][pb
                    as usize][1 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize]
                    * (*self_0)
                        .r_out_ph_im_in_m2[ts
                        as usize][pb as usize][1 as core::ffi::c_int as usize];
                (*self_0)
                    .r_out_im_in_m2[ts
                    as usize][pb
                    as usize][1 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize] = (*self_0)
                    .r_out_re_in_m2[ts
                    as usize][pb
                    as usize][1 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize]
                    * (*self_0)
                        .r_out_ph_im_in_m2[ts
                        as usize][pb as usize][1 as core::ffi::c_int as usize];
                (*self_0)
                    .r_out_re_in_m2[ts
                    as usize][pb
                    as usize][0 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize] = (*self_0)
                    .r_out_re_in_m2[ts
                    as usize][pb
                    as usize][0 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize]
                    * (*self_0)
                        .r_out_ph_re_in_m2[ts
                        as usize][pb as usize][0 as core::ffi::c_int as usize];
                (*self_0)
                    .r_out_re_in_m2[ts
                    as usize][pb
                    as usize][0 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize] = (*self_0)
                    .r_out_re_in_m2[ts
                    as usize][pb
                    as usize][0 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize]
                    * (*self_0)
                        .r_out_ph_re_in_m2[ts
                        as usize][pb as usize][0 as core::ffi::c_int as usize];
                (*self_0)
                    .r_out_re_in_m2[ts
                    as usize][pb
                    as usize][1 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize] = (*self_0)
                    .r_out_re_in_m2[ts
                    as usize][pb
                    as usize][1 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize]
                    * (*self_0)
                        .r_out_ph_re_in_m2[ts
                        as usize][pb as usize][1 as core::ffi::c_int as usize];
                (*self_0)
                    .r_out_re_in_m2[ts
                    as usize][pb
                    as usize][1 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize] = (*self_0)
                    .r_out_re_in_m2[ts
                    as usize][pb
                    as usize][1 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize]
                    * (*self_0)
                        .r_out_ph_re_in_m2[ts
                        as usize][pb as usize][1 as core::ffi::c_int as usize];
                (*self_0)
                    .r_out_diff_im_in_m2[ts
                    as usize][pb
                    as usize][0 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
                    as FLOAT32;
                (*self_0)
                    .r_out_diff_im_in_m2[ts
                    as usize][pb
                    as usize][0 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize] = (*self_0)
                    .r_out_diff_re_in_m2[ts
                    as usize][pb
                    as usize][0 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize]
                    * (*self_0)
                        .r_out_ph_im_in_m2[ts
                        as usize][pb as usize][0 as core::ffi::c_int as usize];
                (*self_0)
                    .r_out_diff_im_in_m2[ts
                    as usize][pb
                    as usize][1 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
                    as FLOAT32;
                (*self_0)
                    .r_out_diff_im_in_m2[ts
                    as usize][pb
                    as usize][1 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize] = (*self_0)
                    .r_out_diff_re_in_m2[ts
                    as usize][pb
                    as usize][1 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize]
                    * (*self_0)
                        .r_out_ph_im_in_m2[ts
                        as usize][pb as usize][1 as core::ffi::c_int as usize];
                (*self_0)
                    .r_out_diff_re_in_m2[ts
                    as usize][pb
                    as usize][0 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
                    as FLOAT32;
                (*self_0)
                    .r_out_diff_re_in_m2[ts
                    as usize][pb
                    as usize][0 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize] = (*self_0)
                    .r_out_diff_re_in_m2[ts
                    as usize][pb
                    as usize][0 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize]
                    * (*self_0)
                        .r_out_ph_re_in_m2[ts
                        as usize][pb as usize][0 as core::ffi::c_int as usize];
                (*self_0)
                    .r_out_diff_re_in_m2[ts
                    as usize][pb
                    as usize][1 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
                    as FLOAT32;
                (*self_0)
                    .r_out_diff_re_in_m2[ts
                    as usize][pb
                    as usize][1 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize] = (*self_0)
                    .r_out_diff_re_in_m2[ts
                    as usize][pb
                    as usize][1 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize]
                    * (*self_0)
                        .r_out_ph_re_in_m2[ts
                        as usize][pb as usize][1 as core::ffi::c_int as usize];
                pb += 1;
            }
            ts += 1;
        }
    }
    if (*self_0).res_bands == 0 as core::ffi::c_int {
        num_col_iters = (*self_0).dir_sig_count;
    } else {
        num_col_iters = (*self_0).dir_sig_count + (*self_0).decor_sig_count;
    }
    ts = 0 as core::ffi::c_int as WORD32;
    while ts < (*self_0).time_slots {
        qs = 0 as core::ffi::c_int as WORD32;
        while qs < (*self_0).hyb_band_count_max {
            let mut indx: WORD32 = *((*self_0).hyb_band_to_processing_band_table)
                .offset(qs as isize);
            row = 0 as core::ffi::c_int as WORD32;
            while row < (*self_0).out_ch_count {
                let mut sum_re_dir: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
                let mut sum_im_dir: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
                col = 0 as core::ffi::c_int as WORD32;
                while col < num_col_iters {
                    sum_re_dir
                        += (*self_0).w_dir[col as usize][ts as usize][qs as usize].re
                            * (*self_0)
                                .r_out_re_in_m2[ts
                                as usize][indx as usize][row as usize][col as usize];
                    sum_im_dir
                        += (*self_0).w_dir[col as usize][ts as usize][qs as usize].im
                            * (*self_0)
                                .r_out_re_in_m2[ts
                                as usize][indx as usize][row as usize][col as usize];
                    col += 1;
                }
                (*self_0).hyb_dir_out[row as usize][ts as usize][qs as usize].re = sum_re_dir;
                (*self_0).hyb_dir_out[row as usize][ts as usize][qs as usize].im = sum_im_dir;
                (*self_0).hyb_diff_out[row as usize][ts as usize][qs as usize].re = (*self_0)
                    .w_diff[1 as core::ffi::c_int as usize][ts as usize][qs as usize]
                    .re
                    * (*self_0)
                        .r_out_diff_re_in_m2[ts
                        as usize][indx
                        as usize][row as usize][1 as core::ffi::c_int as usize];
                (*self_0).hyb_diff_out[row as usize][ts as usize][qs as usize].im = (*self_0)
                    .w_diff[1 as core::ffi::c_int as usize][ts as usize][qs as usize]
                    .im
                    * (*self_0)
                        .r_out_diff_re_in_m2[ts
                        as usize][indx
                        as usize][row as usize][1 as core::ffi::c_int as usize];
                row += 1;
            }
            qs += 1;
        }
        ts += 1;
    }
    if complex_m2 != 0 {
        if phase_interpolation != 0 {
            ts = 0 as core::ffi::c_int as WORD32;
            while ts < (*self_0).time_slots {
                qs = 0 as core::ffi::c_int as WORD32;
                while qs < 2 as core::ffi::c_int {
                    let mut indx_0: WORD32 = *((*self_0)
                        .hyb_band_to_processing_band_table)
                        .offset(qs as isize);
                    row = 0 as core::ffi::c_int as WORD32;
                    while row < (*self_0).out_ch_count {
                        let mut sum_re_dir_0: FLOAT32 = (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .re;
                        let mut sum_im_dir_0: FLOAT32 = (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .im;
                        col = 0 as core::ffi::c_int as WORD32;
                        while col < num_col_iters {
                            sum_re_dir_0
                                += (*self_0)
                                    .w_dir[col as usize][ts as usize][qs as usize]
                                    .im
                                    * (*self_0)
                                        .r_out_im_in_m2[ts
                                        as usize][indx_0 as usize][row as usize][col as usize];
                            sum_im_dir_0
                                -= (*self_0)
                                    .w_dir[col as usize][ts as usize][qs as usize]
                                    .re
                                    * (*self_0)
                                        .r_out_im_in_m2[ts
                                        as usize][indx_0 as usize][row as usize][col as usize];
                            col += 1;
                        }
                        (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .re = sum_re_dir_0;
                        (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .im = sum_im_dir_0;
                        (*self_0).hyb_diff_out[row as usize][ts as usize][qs as usize].re
                            += (*self_0)
                                .w_diff[1 as core::ffi::c_int
                                    as usize][ts as usize][qs as usize]
                                .im
                                * (*self_0)
                                    .r_out_diff_im_in_m2[ts
                                    as usize][indx_0
                                    as usize][row as usize][1 as core::ffi::c_int as usize];
                        (*self_0).hyb_diff_out[row as usize][ts as usize][qs as usize].im
                            -= (*self_0)
                                .w_diff[1 as core::ffi::c_int
                                    as usize][ts as usize][qs as usize]
                                .re
                                * (*self_0)
                                    .r_out_diff_im_in_m2[ts
                                    as usize][indx_0
                                    as usize][row as usize][1 as core::ffi::c_int as usize];
                        row += 1;
                    }
                    qs += 1;
                }
                qs = 2 as core::ffi::c_int as WORD32;
                while qs < (*self_0).hyb_band_count_max {
                    let mut indx_1: WORD32 = *((*self_0)
                        .hyb_band_to_processing_band_table)
                        .offset(qs as isize);
                    row = 0 as core::ffi::c_int as WORD32;
                    while row < (*self_0).out_ch_count {
                        let mut sum_re_dir_1: FLOAT32 = (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .re;
                        let mut sum_im_dir_1: FLOAT32 = (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .im;
                        col = 0 as core::ffi::c_int as WORD32;
                        while col < num_col_iters {
                            sum_re_dir_1
                                -= (*self_0)
                                    .w_dir[col as usize][ts as usize][qs as usize]
                                    .im
                                    * (*self_0)
                                        .r_out_im_in_m2[ts
                                        as usize][indx_1 as usize][row as usize][col as usize];
                            sum_im_dir_1
                                += (*self_0)
                                    .w_dir[col as usize][ts as usize][qs as usize]
                                    .re
                                    * (*self_0)
                                        .r_out_im_in_m2[ts
                                        as usize][indx_1 as usize][row as usize][col as usize];
                            col += 1;
                        }
                        (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .re = sum_re_dir_1;
                        (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .im = sum_im_dir_1;
                        (*self_0).hyb_diff_out[row as usize][ts as usize][qs as usize].re
                            -= (*self_0)
                                .w_diff[1 as core::ffi::c_int
                                    as usize][ts as usize][qs as usize]
                                .im
                                * (*self_0)
                                    .r_out_diff_im_in_m2[ts
                                    as usize][indx_1
                                    as usize][row as usize][1 as core::ffi::c_int as usize];
                        (*self_0).hyb_diff_out[row as usize][ts as usize][qs as usize].im
                            += (*self_0)
                                .w_diff[1 as core::ffi::c_int
                                    as usize][ts as usize][qs as usize]
                                .re
                                * (*self_0)
                                    .r_out_diff_im_in_m2[ts
                                    as usize][indx_1
                                    as usize][row as usize][1 as core::ffi::c_int as usize];
                        row += 1;
                    }
                    qs += 1;
                }
                ts += 1;
            }
        } else {
            let mut num_cols: WORD32 = if (*self_0).dir_sig_count
                + (*self_0).decor_sig_count > 1 as core::ffi::c_int
            {
                1 as WORD32
            } else {
                (*self_0).dir_sig_count + (*self_0).decor_sig_count
            };
            ts = 0 as core::ffi::c_int as WORD32;
            while ts < (*self_0).time_slots {
                qs = 0 as core::ffi::c_int as WORD32;
                while qs < 2 as core::ffi::c_int {
                    let mut indx_2: WORD32 = *((*self_0)
                        .hyb_band_to_processing_band_table)
                        .offset(qs as isize);
                    row = 0 as core::ffi::c_int as WORD32;
                    while row < (*self_0).out_ch_count {
                        let mut sum_re_dir_2: FLOAT32 = (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .re;
                        let mut sum_im_dir_2: FLOAT32 = (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .im;
                        if num_cols > 0 as core::ffi::c_int {
                            sum_re_dir_2
                                += (*self_0)
                                    .w_dir[0 as core::ffi::c_int
                                        as usize][ts as usize][qs as usize]
                                    .im
                                    * (*self_0)
                                        .r_out_im_in_m2[ts
                                        as usize][indx_2
                                        as usize][row as usize][0 as core::ffi::c_int as usize];
                            sum_im_dir_2
                                -= (*self_0)
                                    .w_dir[0 as core::ffi::c_int
                                        as usize][ts as usize][qs as usize]
                                    .re
                                    * (*self_0)
                                        .r_out_im_in_m2[ts
                                        as usize][indx_2
                                        as usize][row as usize][0 as core::ffi::c_int as usize];
                        }
                        (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .re = sum_re_dir_2;
                        (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .im = sum_im_dir_2;
                        row += 1;
                    }
                    qs += 1;
                }
                qs = 2 as core::ffi::c_int as WORD32;
                while qs < (*self_0).hyb_band_count_max {
                    let mut indx_3: WORD32 = *((*self_0)
                        .hyb_band_to_processing_band_table)
                        .offset(qs as isize);
                    row = 0 as core::ffi::c_int as WORD32;
                    while row < (*self_0).out_ch_count {
                        let mut sum_re_dir_3: FLOAT32 = (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .re;
                        let mut sum_im_dir_3: FLOAT32 = (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .im;
                        if num_cols > 0 as core::ffi::c_int {
                            sum_re_dir_3
                                -= (*self_0)
                                    .w_dir[0 as core::ffi::c_int
                                        as usize][ts as usize][qs as usize]
                                    .im
                                    * (*self_0)
                                        .r_out_im_in_m2[ts
                                        as usize][indx_3
                                        as usize][row as usize][0 as core::ffi::c_int as usize];
                            sum_im_dir_3
                                += (*self_0)
                                    .w_dir[0 as core::ffi::c_int
                                        as usize][ts as usize][qs as usize]
                                    .re
                                    * (*self_0)
                                        .r_out_im_in_m2[ts
                                        as usize][indx_3
                                        as usize][row as usize][0 as core::ffi::c_int as usize];
                        }
                        (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .re = sum_re_dir_3;
                        (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .im = sum_im_dir_3;
                        row += 1;
                    }
                    qs += 1;
                }
                ts += 1;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_apply_mix_matrix_type1(
    mut self_0: *mut ia_mps_dec_state_struct,
) -> VOID {
    let mut ts: WORD32 = 0;
    let mut qs: WORD32 = 0;
    let mut row: WORD32 = 0;
    ixheaacd_mps_upmix_interp_type2(
        ((*self_0).m2_decor_re).as_mut_ptr(),
        ((*self_0).r_out_diff_re_in_m2).as_mut_ptr(),
        ((*self_0).m2_decor_re_prev).as_mut_ptr(),
        (*self_0).out_ch_count,
        self_0,
        1 as WORD32,
    );
    ixheaacd_mps_upmix_interp_type2(
        ((*self_0).m2_resid_re).as_mut_ptr(),
        ((*self_0).r_out_re_in_m2).as_mut_ptr(),
        ((*self_0).m2_resid_re_prev).as_mut_ptr(),
        (*self_0).out_ch_count,
        self_0,
        0 as WORD32,
    );
    qs = 0 as core::ffi::c_int as WORD32;
    while qs < (*self_0).hyb_band_count[0 as core::ffi::c_int as usize] {
        let mut indx: WORD32 = *((*self_0).hyb_band_to_processing_band_table)
            .offset(qs as isize);
        ts = 0 as core::ffi::c_int as WORD32;
        while ts < (*self_0).time_slots {
            row = 0 as core::ffi::c_int as WORD32;
            while row < (*self_0).out_ch_count {
                (*self_0).hyb_dir_out[row as usize][ts as usize][qs as usize].re = (*self_0)
                    .w_dir[0 as core::ffi::c_int as usize][ts as usize][qs as usize]
                    .re
                    * (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][indx
                        as usize][row as usize][0 as core::ffi::c_int as usize];
                (*self_0).hyb_dir_out[row as usize][ts as usize][qs as usize].im = (*self_0)
                    .w_dir[0 as core::ffi::c_int as usize][ts as usize][qs as usize]
                    .im
                    * (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][indx
                        as usize][row as usize][0 as core::ffi::c_int as usize];
                (*self_0).hyb_diff_out[row as usize][ts as usize][qs as usize].re = (*self_0)
                    .w_diff[1 as core::ffi::c_int as usize][ts as usize][qs as usize]
                    .re
                    * (*self_0)
                        .r_out_diff_re_in_m2[ts
                        as usize][indx
                        as usize][row as usize][1 as core::ffi::c_int as usize];
                (*self_0).hyb_diff_out[row as usize][ts as usize][qs as usize].im = (*self_0)
                    .w_diff[1 as core::ffi::c_int as usize][ts as usize][qs as usize]
                    .im
                    * (*self_0)
                        .r_out_diff_re_in_m2[ts
                        as usize][indx
                        as usize][row as usize][1 as core::ffi::c_int as usize];
                row += 1;
            }
            ts += 1;
        }
        qs += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_apply_mix_matrix_type2(
    mut self_0: *mut ia_mps_dec_state_struct,
) -> VOID {
    let mut ts: WORD32 = 0;
    let mut qs: WORD32 = 0;
    let mut row: WORD32 = 0;
    let mut col: WORD32 = 0;
    let mut complex_m2: WORD32 = ((*(*self_0).config).bs_phase_coding != 0 as UINT32)
        as core::ffi::c_int;
    let mut phase_interpolation: WORD32 = ((*(*self_0).config).bs_phase_coding
        == 1 as UINT32) as core::ffi::c_int;
    let mut num_col_iters: WORD32 = 0 as WORD32;
    ixheaacd_mps_upmix_interp_type1(
        ((*self_0).m2_decor_re).as_mut_ptr(),
        ((*self_0).r_out_diff_re_in_m2).as_mut_ptr(),
        ((*self_0).m2_decor_re_prev).as_mut_ptr(),
        (*self_0).out_ch_count,
        (*self_0).dir_sig_count + (*self_0).decor_sig_count,
        self_0,
        1 as WORD32,
    );
    ixheaacd_mps_upmix_interp_type1(
        ((*self_0).m2_resid_re).as_mut_ptr(),
        ((*self_0).r_out_re_in_m2).as_mut_ptr(),
        ((*self_0).m2_resid_re_prev).as_mut_ptr(),
        (*self_0).out_ch_count,
        (*self_0).dir_sig_count + (*self_0).decor_sig_count,
        self_0,
        1 as WORD32,
    );
    if complex_m2 != 0 && phase_interpolation == 0 {
        ixheaacd_mps_upmix_interp_type1(
            ((*self_0).m2_decor_im).as_mut_ptr(),
            ((*self_0).r_out_diff_im_in_m2).as_mut_ptr(),
            ((*self_0).m2_decor_im_prev).as_mut_ptr(),
            (*self_0).out_ch_count,
            (*self_0).dir_sig_count + (*self_0).decor_sig_count,
            self_0,
            1 as WORD32,
        );
        ixheaacd_mps_upmix_interp_type1(
            ((*self_0).m2_resid_im).as_mut_ptr(),
            ((*self_0).r_out_im_in_m2).as_mut_ptr(),
            ((*self_0).m2_resid_im_prev).as_mut_ptr(),
            (*self_0).out_ch_count,
            (*self_0).dir_sig_count + (*self_0).decor_sig_count,
            self_0,
            1 as WORD32,
        );
    }
    if phase_interpolation != 0 {
        ixheaacd_mps_phase_interpolation(
            ((*self_0).phase_l).as_mut_ptr(),
            ((*self_0).phase_r).as_mut_ptr(),
            ((*self_0).phase_l_prev).as_mut_ptr(),
            ((*self_0).phase_r_prev).as_mut_ptr(),
            ((*self_0).r_out_ph_re_in_m2).as_mut_ptr(),
            ((*self_0).r_out_ph_im_in_m2).as_mut_ptr(),
            self_0,
        );
        ts = 0 as core::ffi::c_int as WORD32;
        while ts < (*self_0).time_slots {
            let mut pb: WORD32 = 0;
            pb = 0 as core::ffi::c_int as WORD32;
            while pb < (*self_0).bs_param_bands {
                (*self_0)
                    .r_out_im_in_m2[ts
                    as usize][pb
                    as usize][0 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize] = (*self_0)
                    .r_out_re_in_m2[ts
                    as usize][pb
                    as usize][0 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize]
                    * (*self_0)
                        .r_out_ph_im_in_m2[ts
                        as usize][pb as usize][0 as core::ffi::c_int as usize];
                (*self_0)
                    .r_out_im_in_m2[ts
                    as usize][pb
                    as usize][0 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize] = (*self_0)
                    .r_out_re_in_m2[ts
                    as usize][pb
                    as usize][0 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize]
                    * (*self_0)
                        .r_out_ph_im_in_m2[ts
                        as usize][pb as usize][0 as core::ffi::c_int as usize];
                (*self_0)
                    .r_out_im_in_m2[ts
                    as usize][pb
                    as usize][1 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize] = (*self_0)
                    .r_out_re_in_m2[ts
                    as usize][pb
                    as usize][1 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize]
                    * (*self_0)
                        .r_out_ph_im_in_m2[ts
                        as usize][pb as usize][1 as core::ffi::c_int as usize];
                (*self_0)
                    .r_out_im_in_m2[ts
                    as usize][pb
                    as usize][1 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize] = (*self_0)
                    .r_out_re_in_m2[ts
                    as usize][pb
                    as usize][1 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize]
                    * (*self_0)
                        .r_out_ph_im_in_m2[ts
                        as usize][pb as usize][1 as core::ffi::c_int as usize];
                (*self_0)
                    .r_out_re_in_m2[ts
                    as usize][pb
                    as usize][0 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize] = (*self_0)
                    .r_out_re_in_m2[ts
                    as usize][pb
                    as usize][0 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize]
                    * (*self_0)
                        .r_out_ph_re_in_m2[ts
                        as usize][pb as usize][0 as core::ffi::c_int as usize];
                (*self_0)
                    .r_out_re_in_m2[ts
                    as usize][pb
                    as usize][0 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize] = (*self_0)
                    .r_out_re_in_m2[ts
                    as usize][pb
                    as usize][0 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize]
                    * (*self_0)
                        .r_out_ph_re_in_m2[ts
                        as usize][pb as usize][0 as core::ffi::c_int as usize];
                (*self_0)
                    .r_out_re_in_m2[ts
                    as usize][pb
                    as usize][1 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize] = (*self_0)
                    .r_out_re_in_m2[ts
                    as usize][pb
                    as usize][1 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize]
                    * (*self_0)
                        .r_out_ph_re_in_m2[ts
                        as usize][pb as usize][1 as core::ffi::c_int as usize];
                (*self_0)
                    .r_out_re_in_m2[ts
                    as usize][pb
                    as usize][1 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize] = (*self_0)
                    .r_out_re_in_m2[ts
                    as usize][pb
                    as usize][1 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize]
                    * (*self_0)
                        .r_out_ph_re_in_m2[ts
                        as usize][pb as usize][1 as core::ffi::c_int as usize];
                (*self_0)
                    .r_out_diff_im_in_m2[ts
                    as usize][pb
                    as usize][0 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
                    as FLOAT32;
                (*self_0)
                    .r_out_diff_im_in_m2[ts
                    as usize][pb
                    as usize][0 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize] = (*self_0)
                    .r_out_diff_re_in_m2[ts
                    as usize][pb
                    as usize][0 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize]
                    * (*self_0)
                        .r_out_ph_im_in_m2[ts
                        as usize][pb as usize][0 as core::ffi::c_int as usize];
                (*self_0)
                    .r_out_diff_im_in_m2[ts
                    as usize][pb
                    as usize][1 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
                    as FLOAT32;
                (*self_0)
                    .r_out_diff_im_in_m2[ts
                    as usize][pb
                    as usize][1 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize] = (*self_0)
                    .r_out_diff_re_in_m2[ts
                    as usize][pb
                    as usize][1 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize]
                    * (*self_0)
                        .r_out_ph_im_in_m2[ts
                        as usize][pb as usize][1 as core::ffi::c_int as usize];
                (*self_0)
                    .r_out_diff_re_in_m2[ts
                    as usize][pb
                    as usize][0 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
                    as FLOAT32;
                (*self_0)
                    .r_out_diff_re_in_m2[ts
                    as usize][pb
                    as usize][0 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize] = (*self_0)
                    .r_out_diff_re_in_m2[ts
                    as usize][pb
                    as usize][0 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize]
                    * (*self_0)
                        .r_out_ph_re_in_m2[ts
                        as usize][pb as usize][0 as core::ffi::c_int as usize];
                (*self_0)
                    .r_out_diff_re_in_m2[ts
                    as usize][pb
                    as usize][1 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
                    as FLOAT32;
                (*self_0)
                    .r_out_diff_re_in_m2[ts
                    as usize][pb
                    as usize][1 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize] = (*self_0)
                    .r_out_diff_re_in_m2[ts
                    as usize][pb
                    as usize][1 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize]
                    * (*self_0)
                        .r_out_ph_re_in_m2[ts
                        as usize][pb as usize][1 as core::ffi::c_int as usize];
                pb += 1;
            }
            ts += 1;
        }
    }
    if (*self_0).res_bands == 0 as core::ffi::c_int {
        num_col_iters = (*self_0).dir_sig_count;
    } else {
        num_col_iters = (*self_0).dir_sig_count + (*self_0).decor_sig_count;
    }
    ts = 0 as core::ffi::c_int as WORD32;
    while ts < (*self_0).time_slots {
        qs = 0 as core::ffi::c_int as WORD32;
        while qs < (*self_0).hyb_band_count_max {
            let mut indx: WORD32 = *((*self_0).hyb_band_to_processing_band_table)
                .offset(qs as isize);
            row = 0 as core::ffi::c_int as WORD32;
            while row < (*self_0).out_ch_count {
                let mut sum_re_dir: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
                let mut sum_im_dir: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
                col = 0 as core::ffi::c_int as WORD32;
                while col < num_col_iters {
                    sum_re_dir
                        += (*self_0).w_dir[col as usize][ts as usize][qs as usize].re
                            * (*self_0)
                                .r_out_re_in_m2[ts
                                as usize][indx as usize][row as usize][col as usize];
                    sum_im_dir
                        += (*self_0).w_dir[col as usize][ts as usize][qs as usize].im
                            * (*self_0)
                                .r_out_re_in_m2[ts
                                as usize][indx as usize][row as usize][col as usize];
                    col += 1;
                }
                (*self_0).hyb_dir_out[row as usize][ts as usize][qs as usize].re = sum_re_dir;
                (*self_0).hyb_dir_out[row as usize][ts as usize][qs as usize].im = sum_im_dir;
                (*self_0).hyb_diff_out[row as usize][ts as usize][qs as usize].re = (*self_0)
                    .w_diff[1 as core::ffi::c_int as usize][ts as usize][qs as usize]
                    .re
                    * (*self_0)
                        .r_out_diff_re_in_m2[ts
                        as usize][indx
                        as usize][row as usize][1 as core::ffi::c_int as usize];
                (*self_0).hyb_diff_out[row as usize][ts as usize][qs as usize].im = (*self_0)
                    .w_diff[1 as core::ffi::c_int as usize][ts as usize][qs as usize]
                    .im
                    * (*self_0)
                        .r_out_diff_re_in_m2[ts
                        as usize][indx
                        as usize][row as usize][1 as core::ffi::c_int as usize];
                row += 1;
            }
            qs += 1;
        }
        ts += 1;
    }
    if complex_m2 != 0 {
        if phase_interpolation != 0 {
            ts = 0 as core::ffi::c_int as WORD32;
            while ts < (*self_0).time_slots {
                qs = 0 as core::ffi::c_int as WORD32;
                while qs < 2 as core::ffi::c_int {
                    let mut indx_0: WORD32 = *((*self_0)
                        .hyb_band_to_processing_band_table)
                        .offset(qs as isize);
                    row = 0 as core::ffi::c_int as WORD32;
                    while row < (*self_0).out_ch_count {
                        let mut sum_re_dir_0: FLOAT32 = (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .re;
                        let mut sum_im_dir_0: FLOAT32 = (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .im;
                        col = 0 as core::ffi::c_int as WORD32;
                        while col < num_col_iters {
                            sum_re_dir_0
                                += (*self_0)
                                    .w_dir[col as usize][ts as usize][qs as usize]
                                    .im
                                    * (*self_0)
                                        .r_out_im_in_m2[ts
                                        as usize][indx_0 as usize][row as usize][col as usize];
                            sum_im_dir_0
                                -= (*self_0)
                                    .w_dir[col as usize][ts as usize][qs as usize]
                                    .re
                                    * (*self_0)
                                        .r_out_im_in_m2[ts
                                        as usize][indx_0 as usize][row as usize][col as usize];
                            col += 1;
                        }
                        (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .re = sum_re_dir_0;
                        (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .im = sum_im_dir_0;
                        (*self_0).hyb_diff_out[row as usize][ts as usize][qs as usize].re
                            += (*self_0)
                                .w_diff[1 as core::ffi::c_int
                                    as usize][ts as usize][qs as usize]
                                .im
                                * (*self_0)
                                    .r_out_diff_im_in_m2[ts
                                    as usize][indx_0
                                    as usize][row as usize][1 as core::ffi::c_int as usize];
                        (*self_0).hyb_diff_out[row as usize][ts as usize][qs as usize].im
                            -= (*self_0)
                                .w_diff[1 as core::ffi::c_int
                                    as usize][ts as usize][qs as usize]
                                .re
                                * (*self_0)
                                    .r_out_diff_im_in_m2[ts
                                    as usize][indx_0
                                    as usize][row as usize][1 as core::ffi::c_int as usize];
                        row += 1;
                    }
                    qs += 1;
                }
                qs = 2 as core::ffi::c_int as WORD32;
                while qs < (*self_0).hyb_band_count_max {
                    let mut indx_1: WORD32 = *((*self_0)
                        .hyb_band_to_processing_band_table)
                        .offset(qs as isize);
                    row = 0 as core::ffi::c_int as WORD32;
                    while row < (*self_0).out_ch_count {
                        let mut sum_re_dir_1: FLOAT32 = (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .re;
                        let mut sum_im_dir_1: FLOAT32 = (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .im;
                        col = 0 as core::ffi::c_int as WORD32;
                        while col < num_col_iters {
                            sum_re_dir_1
                                -= (*self_0)
                                    .w_dir[col as usize][ts as usize][qs as usize]
                                    .im
                                    * (*self_0)
                                        .r_out_im_in_m2[ts
                                        as usize][indx_1 as usize][row as usize][col as usize];
                            sum_im_dir_1
                                += (*self_0)
                                    .w_dir[col as usize][ts as usize][qs as usize]
                                    .re
                                    * (*self_0)
                                        .r_out_im_in_m2[ts
                                        as usize][indx_1 as usize][row as usize][col as usize];
                            col += 1;
                        }
                        (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .re = sum_re_dir_1;
                        (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .im = sum_im_dir_1;
                        (*self_0).hyb_diff_out[row as usize][ts as usize][qs as usize].re
                            -= (*self_0)
                                .w_diff[1 as core::ffi::c_int
                                    as usize][ts as usize][qs as usize]
                                .im
                                * (*self_0)
                                    .r_out_diff_im_in_m2[ts
                                    as usize][indx_1
                                    as usize][row as usize][1 as core::ffi::c_int as usize];
                        (*self_0).hyb_diff_out[row as usize][ts as usize][qs as usize].im
                            += (*self_0)
                                .w_diff[1 as core::ffi::c_int
                                    as usize][ts as usize][qs as usize]
                                .re
                                * (*self_0)
                                    .r_out_diff_im_in_m2[ts
                                    as usize][indx_1
                                    as usize][row as usize][1 as core::ffi::c_int as usize];
                        row += 1;
                    }
                    qs += 1;
                }
                ts += 1;
            }
        } else {
            let mut num_cols: WORD32 = if (*self_0).dir_sig_count
                + (*self_0).decor_sig_count > 1 as core::ffi::c_int
            {
                1 as WORD32
            } else {
                (*self_0).dir_sig_count + (*self_0).decor_sig_count
            };
            ts = 0 as core::ffi::c_int as WORD32;
            while ts < (*self_0).time_slots {
                qs = 0 as core::ffi::c_int as WORD32;
                while qs < 2 as core::ffi::c_int {
                    let mut indx_2: WORD32 = *((*self_0)
                        .hyb_band_to_processing_band_table)
                        .offset(qs as isize);
                    row = 0 as core::ffi::c_int as WORD32;
                    while row < (*self_0).out_ch_count {
                        let mut sum_re_dir_2: FLOAT32 = (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .re;
                        let mut sum_im_dir_2: FLOAT32 = (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .im;
                        if num_cols > 0 as core::ffi::c_int {
                            sum_re_dir_2
                                += (*self_0)
                                    .w_dir[0 as core::ffi::c_int
                                        as usize][ts as usize][qs as usize]
                                    .im
                                    * (*self_0)
                                        .r_out_im_in_m2[ts
                                        as usize][indx_2
                                        as usize][row as usize][0 as core::ffi::c_int as usize];
                            sum_im_dir_2
                                -= (*self_0)
                                    .w_dir[0 as core::ffi::c_int
                                        as usize][ts as usize][qs as usize]
                                    .re
                                    * (*self_0)
                                        .r_out_im_in_m2[ts
                                        as usize][indx_2
                                        as usize][row as usize][0 as core::ffi::c_int as usize];
                        }
                        (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .re = sum_re_dir_2;
                        (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .im = sum_im_dir_2;
                        row += 1;
                    }
                    qs += 1;
                }
                qs = 2 as core::ffi::c_int as WORD32;
                while qs < (*self_0).hyb_band_count_max {
                    let mut indx_3: WORD32 = *((*self_0)
                        .hyb_band_to_processing_band_table)
                        .offset(qs as isize);
                    row = 0 as core::ffi::c_int as WORD32;
                    while row < (*self_0).out_ch_count {
                        let mut sum_re_dir_3: FLOAT32 = (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .re;
                        let mut sum_im_dir_3: FLOAT32 = (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .im;
                        if num_cols > 0 as core::ffi::c_int {
                            sum_re_dir_3
                                -= (*self_0)
                                    .w_dir[0 as core::ffi::c_int
                                        as usize][ts as usize][qs as usize]
                                    .im
                                    * (*self_0)
                                        .r_out_im_in_m2[ts
                                        as usize][indx_3
                                        as usize][row as usize][0 as core::ffi::c_int as usize];
                            sum_im_dir_3
                                += (*self_0)
                                    .w_dir[0 as core::ffi::c_int
                                        as usize][ts as usize][qs as usize]
                                    .re
                                    * (*self_0)
                                        .r_out_im_in_m2[ts
                                        as usize][indx_3
                                        as usize][row as usize][0 as core::ffi::c_int as usize];
                        }
                        (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .re = sum_re_dir_3;
                        (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .im = sum_im_dir_3;
                        row += 1;
                    }
                    qs += 1;
                }
                ts += 1;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_apply_mix_matrix_type3(
    mut self_0: *mut ia_mps_dec_state_struct,
) -> VOID {
    let mut ts: WORD32 = 0;
    let mut qs: WORD32 = 0;
    let mut row: WORD32 = 0;
    let mut col: WORD32 = 0;
    let mut complex_m2: WORD32 = ((*(*self_0).config).bs_phase_coding != 0 as UINT32)
        as core::ffi::c_int;
    let mut phase_interpolation: WORD32 = ((*(*self_0).config).bs_phase_coding
        == 1 as UINT32) as core::ffi::c_int;
    let mut num_col_iters: WORD32 = 0 as WORD32;
    if (*self_0).res_bands != 28 as core::ffi::c_int {
        ixheaacd_mps_upmix_interp_type2(
            ((*self_0).m2_decor_re).as_mut_ptr(),
            ((*self_0).r_out_diff_re_in_m2).as_mut_ptr(),
            ((*self_0).m2_decor_re_prev).as_mut_ptr(),
            (*self_0).out_ch_count,
            self_0,
            1 as WORD32,
        );
    }
    if (*self_0).res_bands == 0 as core::ffi::c_int {
        num_col_iters = (*self_0).dir_sig_count;
        ixheaacd_mps_upmix_interp_type2(
            ((*self_0).m2_resid_re).as_mut_ptr(),
            ((*self_0).r_out_re_in_m2).as_mut_ptr(),
            ((*self_0).m2_resid_re_prev).as_mut_ptr(),
            (*self_0).out_ch_count,
            self_0,
            0 as WORD32,
        );
    } else {
        num_col_iters = (*self_0).dir_sig_count + (*self_0).decor_sig_count;
        ixheaacd_mps_upmix_interp_type1(
            ((*self_0).m2_resid_re).as_mut_ptr(),
            ((*self_0).r_out_re_in_m2).as_mut_ptr(),
            ((*self_0).m2_resid_re_prev).as_mut_ptr(),
            (*self_0).out_ch_count,
            (*self_0).dir_sig_count + (*self_0).decor_sig_count,
            self_0,
            1 as WORD32,
        );
    }
    if complex_m2 != 0 && phase_interpolation == 0 {
        ixheaacd_mps_upmix_interp_type2(
            ((*self_0).m2_resid_im).as_mut_ptr(),
            ((*self_0).r_out_im_in_m2).as_mut_ptr(),
            ((*self_0).m2_resid_im_prev).as_mut_ptr(),
            (*self_0).out_ch_count,
            self_0,
            0 as WORD32,
        );
    }
    if phase_interpolation != 0 {
        ixheaacd_mps_phase_interpolation(
            ((*self_0).phase_l).as_mut_ptr(),
            ((*self_0).phase_r).as_mut_ptr(),
            ((*self_0).phase_l_prev).as_mut_ptr(),
            ((*self_0).phase_r_prev).as_mut_ptr(),
            ((*self_0).r_out_ph_re_in_m2).as_mut_ptr(),
            ((*self_0).r_out_ph_im_in_m2).as_mut_ptr(),
            self_0,
        );
        if (*self_0).res_bands == 0 as core::ffi::c_int {
            ts = 0 as core::ffi::c_int as WORD32;
            while ts < (*self_0).time_slots {
                let mut pb: WORD32 = 0;
                pb = 0 as core::ffi::c_int as WORD32;
                while pb < (*self_0).bs_param_bands {
                    (*self_0)
                        .r_out_im_in_m2[ts
                        as usize][pb
                        as usize][0 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize] = (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb
                        as usize][0 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize]
                        * (*self_0)
                            .r_out_ph_im_in_m2[ts
                            as usize][pb as usize][0 as core::ffi::c_int as usize];
                    (*self_0)
                        .r_out_im_in_m2[ts
                        as usize][pb
                        as usize][1 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize] = (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb
                        as usize][1 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize]
                        * (*self_0)
                            .r_out_ph_im_in_m2[ts
                            as usize][pb as usize][1 as core::ffi::c_int as usize];
                    (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb
                        as usize][0 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize] = (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb
                        as usize][0 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize]
                        * (*self_0)
                            .r_out_ph_re_in_m2[ts
                            as usize][pb as usize][0 as core::ffi::c_int as usize];
                    (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb
                        as usize][1 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize] = (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb
                        as usize][1 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize]
                        * (*self_0)
                            .r_out_ph_re_in_m2[ts
                            as usize][pb as usize][1 as core::ffi::c_int as usize];
                    (*self_0)
                        .r_out_diff_im_in_m2[ts
                        as usize][pb
                        as usize][0 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize] = (*self_0)
                        .r_out_diff_re_in_m2[ts
                        as usize][pb
                        as usize][0 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize]
                        * (*self_0)
                            .r_out_ph_im_in_m2[ts
                            as usize][pb as usize][0 as core::ffi::c_int as usize];
                    (*self_0)
                        .r_out_diff_im_in_m2[ts
                        as usize][pb
                        as usize][1 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize] = (*self_0)
                        .r_out_diff_re_in_m2[ts
                        as usize][pb
                        as usize][1 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize]
                        * (*self_0)
                            .r_out_ph_im_in_m2[ts
                            as usize][pb as usize][1 as core::ffi::c_int as usize];
                    (*self_0)
                        .r_out_diff_re_in_m2[ts
                        as usize][pb
                        as usize][0 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize] = (*self_0)
                        .r_out_diff_re_in_m2[ts
                        as usize][pb
                        as usize][0 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize]
                        * (*self_0)
                            .r_out_ph_re_in_m2[ts
                            as usize][pb as usize][0 as core::ffi::c_int as usize];
                    (*self_0)
                        .r_out_diff_re_in_m2[ts
                        as usize][pb
                        as usize][1 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize] = (*self_0)
                        .r_out_diff_re_in_m2[ts
                        as usize][pb
                        as usize][1 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize]
                        * (*self_0)
                            .r_out_ph_re_in_m2[ts
                            as usize][pb as usize][1 as core::ffi::c_int as usize];
                    pb += 1;
                }
                ts += 1;
            }
        } else if (*self_0).res_bands == 28 as core::ffi::c_int {
            ts = 0 as core::ffi::c_int as WORD32;
            while ts < (*self_0).time_slots {
                let mut pb_0: WORD32 = 0;
                pb_0 = 0 as core::ffi::c_int as WORD32;
                while pb_0 < (*self_0).bs_param_bands {
                    (*self_0)
                        .r_out_im_in_m2[ts
                        as usize][pb_0
                        as usize][0 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize] = (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb_0
                        as usize][0 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize]
                        * (*self_0)
                            .r_out_ph_im_in_m2[ts
                            as usize][pb_0 as usize][0 as core::ffi::c_int as usize];
                    (*self_0)
                        .r_out_im_in_m2[ts
                        as usize][pb_0
                        as usize][0 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize] = (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb_0
                        as usize][0 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize]
                        * (*self_0)
                            .r_out_ph_im_in_m2[ts
                            as usize][pb_0 as usize][0 as core::ffi::c_int as usize];
                    (*self_0)
                        .r_out_im_in_m2[ts
                        as usize][pb_0
                        as usize][1 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize] = (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb_0
                        as usize][1 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize]
                        * (*self_0)
                            .r_out_ph_im_in_m2[ts
                            as usize][pb_0 as usize][1 as core::ffi::c_int as usize];
                    (*self_0)
                        .r_out_im_in_m2[ts
                        as usize][pb_0
                        as usize][1 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize] = (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb_0
                        as usize][1 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize]
                        * (*self_0)
                            .r_out_ph_im_in_m2[ts
                            as usize][pb_0 as usize][1 as core::ffi::c_int as usize];
                    (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb_0
                        as usize][0 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize] = (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb_0
                        as usize][0 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize]
                        * (*self_0)
                            .r_out_ph_re_in_m2[ts
                            as usize][pb_0 as usize][0 as core::ffi::c_int as usize];
                    (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb_0
                        as usize][0 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize] = (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb_0
                        as usize][0 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize]
                        * (*self_0)
                            .r_out_ph_re_in_m2[ts
                            as usize][pb_0 as usize][0 as core::ffi::c_int as usize];
                    (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb_0
                        as usize][1 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize] = (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb_0
                        as usize][1 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize]
                        * (*self_0)
                            .r_out_ph_re_in_m2[ts
                            as usize][pb_0 as usize][1 as core::ffi::c_int as usize];
                    (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb_0
                        as usize][1 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize] = (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb_0
                        as usize][1 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize]
                        * (*self_0)
                            .r_out_ph_re_in_m2[ts
                            as usize][pb_0 as usize][1 as core::ffi::c_int as usize];
                    pb_0 += 1;
                }
                ts += 1;
            }
        } else {
            ts = 0 as core::ffi::c_int as WORD32;
            while ts < (*self_0).time_slots {
                let mut pb_1: WORD32 = 0;
                pb_1 = 0 as core::ffi::c_int as WORD32;
                while pb_1 < (*self_0).bs_param_bands {
                    (*self_0)
                        .r_out_im_in_m2[ts
                        as usize][pb_1
                        as usize][0 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize] = (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb_1
                        as usize][0 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize]
                        * (*self_0)
                            .r_out_ph_im_in_m2[ts
                            as usize][pb_1 as usize][0 as core::ffi::c_int as usize];
                    (*self_0)
                        .r_out_im_in_m2[ts
                        as usize][pb_1
                        as usize][0 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize] = (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb_1
                        as usize][0 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize]
                        * (*self_0)
                            .r_out_ph_im_in_m2[ts
                            as usize][pb_1 as usize][0 as core::ffi::c_int as usize];
                    (*self_0)
                        .r_out_im_in_m2[ts
                        as usize][pb_1
                        as usize][1 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize] = (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb_1
                        as usize][1 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize]
                        * (*self_0)
                            .r_out_ph_im_in_m2[ts
                            as usize][pb_1 as usize][1 as core::ffi::c_int as usize];
                    (*self_0)
                        .r_out_im_in_m2[ts
                        as usize][pb_1
                        as usize][1 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize] = (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb_1
                        as usize][1 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize]
                        * (*self_0)
                            .r_out_ph_im_in_m2[ts
                            as usize][pb_1 as usize][1 as core::ffi::c_int as usize];
                    (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb_1
                        as usize][0 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize] = (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb_1
                        as usize][0 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize]
                        * (*self_0)
                            .r_out_ph_re_in_m2[ts
                            as usize][pb_1 as usize][0 as core::ffi::c_int as usize];
                    (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb_1
                        as usize][0 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize] = (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb_1
                        as usize][0 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize]
                        * (*self_0)
                            .r_out_ph_re_in_m2[ts
                            as usize][pb_1 as usize][0 as core::ffi::c_int as usize];
                    (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb_1
                        as usize][1 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize] = (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb_1
                        as usize][1 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize]
                        * (*self_0)
                            .r_out_ph_re_in_m2[ts
                            as usize][pb_1 as usize][1 as core::ffi::c_int as usize];
                    (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb_1
                        as usize][1 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize] = (*self_0)
                        .r_out_re_in_m2[ts
                        as usize][pb_1
                        as usize][1 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize]
                        * (*self_0)
                            .r_out_ph_re_in_m2[ts
                            as usize][pb_1 as usize][1 as core::ffi::c_int as usize];
                    (*self_0)
                        .r_out_diff_im_in_m2[ts
                        as usize][pb_1
                        as usize][0 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize] = (*self_0)
                        .r_out_diff_re_in_m2[ts
                        as usize][pb_1
                        as usize][0 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize]
                        * (*self_0)
                            .r_out_ph_im_in_m2[ts
                            as usize][pb_1 as usize][0 as core::ffi::c_int as usize];
                    (*self_0)
                        .r_out_diff_im_in_m2[ts
                        as usize][pb_1
                        as usize][1 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize] = (*self_0)
                        .r_out_diff_re_in_m2[ts
                        as usize][pb_1
                        as usize][1 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize]
                        * (*self_0)
                            .r_out_ph_im_in_m2[ts
                            as usize][pb_1 as usize][1 as core::ffi::c_int as usize];
                    (*self_0)
                        .r_out_diff_re_in_m2[ts
                        as usize][pb_1
                        as usize][0 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize] = (*self_0)
                        .r_out_diff_re_in_m2[ts
                        as usize][pb_1
                        as usize][0 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize]
                        * (*self_0)
                            .r_out_ph_re_in_m2[ts
                            as usize][pb_1 as usize][0 as core::ffi::c_int as usize];
                    (*self_0)
                        .r_out_diff_re_in_m2[ts
                        as usize][pb_1
                        as usize][1 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize] = (*self_0)
                        .r_out_diff_re_in_m2[ts
                        as usize][pb_1
                        as usize][1 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize]
                        * (*self_0)
                            .r_out_ph_re_in_m2[ts
                            as usize][pb_1 as usize][1 as core::ffi::c_int as usize];
                    pb_1 += 1;
                }
                ts += 1;
            }
        }
    }
    if (*self_0).res_bands == 0 as core::ffi::c_int {
        ts = 0 as core::ffi::c_int as WORD32;
        while ts < (*self_0).time_slots {
            qs = 0 as core::ffi::c_int as WORD32;
            while qs < (*self_0).hyb_band_count[0 as core::ffi::c_int as usize] {
                let mut indx: WORD32 = *((*self_0).hyb_band_to_processing_band_table)
                    .offset(qs as isize);
                row = 0 as core::ffi::c_int as WORD32;
                while row < (*self_0).out_ch_count {
                    (*self_0).hyb_dir_out[row as usize][ts as usize][qs as usize].re = (*self_0)
                        .w_dir[0 as core::ffi::c_int as usize][ts as usize][qs as usize]
                        .re
                        * (*self_0)
                            .r_out_re_in_m2[ts
                            as usize][indx
                            as usize][row as usize][0 as core::ffi::c_int as usize];
                    (*self_0).hyb_dir_out[row as usize][ts as usize][qs as usize].im = (*self_0)
                        .w_dir[0 as core::ffi::c_int as usize][ts as usize][qs as usize]
                        .im
                        * (*self_0)
                            .r_out_re_in_m2[ts
                            as usize][indx
                            as usize][row as usize][0 as core::ffi::c_int as usize];
                    (*self_0).hyb_diff_out[row as usize][ts as usize][qs as usize].re = (*self_0)
                        .w_diff[1 as core::ffi::c_int as usize][ts as usize][qs as usize]
                        .re
                        * (*self_0)
                            .r_out_diff_re_in_m2[ts
                            as usize][indx
                            as usize][row as usize][1 as core::ffi::c_int as usize];
                    (*self_0).hyb_diff_out[row as usize][ts as usize][qs as usize].im = (*self_0)
                        .w_diff[1 as core::ffi::c_int as usize][ts as usize][qs as usize]
                        .im
                        * (*self_0)
                            .r_out_diff_re_in_m2[ts
                            as usize][indx
                            as usize][row as usize][1 as core::ffi::c_int as usize];
                    row += 1;
                }
                qs += 1;
            }
            ts += 1;
        }
    } else if (*self_0).res_bands == 28 as core::ffi::c_int {
        ts = 0 as core::ffi::c_int as WORD32;
        while ts < (*self_0).time_slots {
            qs = 0 as core::ffi::c_int as WORD32;
            while qs < (*self_0).hyb_band_count[1 as core::ffi::c_int as usize] {
                let mut indx_0: WORD32 = *((*self_0).hyb_band_to_processing_band_table)
                    .offset(qs as isize);
                row = 0 as core::ffi::c_int as WORD32;
                while row < (*self_0).out_ch_count {
                    let mut sum_re_dir: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
                    let mut sum_im_dir: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
                    col = 0 as core::ffi::c_int as WORD32;
                    while col < num_col_iters {
                        sum_re_dir
                            += (*self_0).w_dir[col as usize][ts as usize][qs as usize].re
                                * (*self_0)
                                    .r_out_re_in_m2[ts
                                    as usize][indx_0 as usize][row as usize][col as usize];
                        sum_im_dir
                            += (*self_0).w_dir[col as usize][ts as usize][qs as usize].im
                                * (*self_0)
                                    .r_out_re_in_m2[ts
                                    as usize][indx_0 as usize][row as usize][col as usize];
                        col += 1;
                    }
                    (*self_0).hyb_dir_out[row as usize][ts as usize][qs as usize].re = sum_re_dir;
                    (*self_0).hyb_dir_out[row as usize][ts as usize][qs as usize].im = sum_im_dir;
                    row += 1;
                }
                qs += 1;
            }
            while qs < (*self_0).hyb_band_count[0 as core::ffi::c_int as usize] {
                let mut indx_1: WORD32 = *((*self_0).hyb_band_to_processing_band_table)
                    .offset(qs as isize);
                row = 0 as core::ffi::c_int as WORD32;
                while row < (*self_0).out_ch_count {
                    (*self_0).hyb_dir_out[row as usize][ts as usize][qs as usize].re = (*self_0)
                        .w_dir[0 as core::ffi::c_int as usize][ts as usize][qs as usize]
                        .re
                        * (*self_0)
                            .r_out_re_in_m2[ts
                            as usize][indx_1
                            as usize][row as usize][0 as core::ffi::c_int as usize];
                    (*self_0).hyb_dir_out[row as usize][ts as usize][qs as usize].im = (*self_0)
                        .w_dir[0 as core::ffi::c_int as usize][ts as usize][qs as usize]
                        .im
                        * (*self_0)
                            .r_out_re_in_m2[ts
                            as usize][indx_1
                            as usize][row as usize][0 as core::ffi::c_int as usize];
                    row += 1;
                }
                qs += 1;
            }
            ts += 1;
        }
    } else {
        let mut dif_s: WORD32 = ixheaacd_mps_gain_set_indx[(*self_0).res_bands as usize];
        ts = 0 as core::ffi::c_int as WORD32;
        while ts < (*self_0).time_slots {
            qs = 0 as core::ffi::c_int as WORD32;
            while qs < dif_s {
                let mut indx_2: WORD32 = *((*self_0).hyb_band_to_processing_band_table)
                    .offset(qs as isize);
                row = 0 as core::ffi::c_int as WORD32;
                while row < (*self_0).out_ch_count {
                    let mut sum_re_dir_0: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
                    let mut sum_im_dir_0: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
                    col = 0 as core::ffi::c_int as WORD32;
                    while col < num_col_iters {
                        sum_re_dir_0
                            += (*self_0).w_dir[col as usize][ts as usize][qs as usize].re
                                * (*self_0)
                                    .r_out_re_in_m2[ts
                                    as usize][indx_2 as usize][row as usize][col as usize];
                        sum_im_dir_0
                            += (*self_0).w_dir[col as usize][ts as usize][qs as usize].im
                                * (*self_0)
                                    .r_out_re_in_m2[ts
                                    as usize][indx_2 as usize][row as usize][col as usize];
                        col += 1;
                    }
                    (*self_0).hyb_dir_out[row as usize][ts as usize][qs as usize].re = sum_re_dir_0;
                    (*self_0).hyb_dir_out[row as usize][ts as usize][qs as usize].im = sum_im_dir_0;
                    row += 1;
                }
                qs += 1;
            }
            while qs < (*self_0).hyb_band_count[1 as core::ffi::c_int as usize] {
                let mut indx_3: WORD32 = *((*self_0).hyb_band_to_processing_band_table)
                    .offset(qs as isize);
                row = 0 as core::ffi::c_int as WORD32;
                while row < (*self_0).out_ch_count {
                    let mut sum_re_dir_1: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
                    let mut sum_im_dir_1: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
                    col = 0 as core::ffi::c_int as WORD32;
                    while col < num_col_iters {
                        sum_re_dir_1
                            += (*self_0).w_dir[col as usize][ts as usize][qs as usize].re
                                * (*self_0)
                                    .r_out_re_in_m2[ts
                                    as usize][indx_3 as usize][row as usize][col as usize];
                        sum_im_dir_1
                            += (*self_0).w_dir[col as usize][ts as usize][qs as usize].im
                                * (*self_0)
                                    .r_out_re_in_m2[ts
                                    as usize][indx_3 as usize][row as usize][col as usize];
                        col += 1;
                    }
                    (*self_0).hyb_dir_out[row as usize][ts as usize][qs as usize].re = sum_re_dir_1;
                    (*self_0).hyb_dir_out[row as usize][ts as usize][qs as usize].im = sum_im_dir_1;
                    (*self_0).hyb_diff_out[row as usize][ts as usize][qs as usize].re = (*self_0)
                        .w_diff[1 as core::ffi::c_int as usize][ts as usize][qs as usize]
                        .re
                        * (*self_0)
                            .r_out_diff_re_in_m2[ts
                            as usize][indx_3
                            as usize][row as usize][1 as core::ffi::c_int as usize];
                    (*self_0).hyb_diff_out[row as usize][ts as usize][qs as usize].im = (*self_0)
                        .w_diff[1 as core::ffi::c_int as usize][ts as usize][qs as usize]
                        .im
                        * (*self_0)
                            .r_out_diff_re_in_m2[ts
                            as usize][indx_3
                            as usize][row as usize][1 as core::ffi::c_int as usize];
                    row += 1;
                }
                qs += 1;
            }
            while qs < (*self_0).hyb_band_count[0 as core::ffi::c_int as usize] {
                let mut indx_4: WORD32 = *((*self_0).hyb_band_to_processing_band_table)
                    .offset(qs as isize);
                row = 0 as core::ffi::c_int as WORD32;
                while row < (*self_0).out_ch_count {
                    (*self_0).hyb_dir_out[row as usize][ts as usize][qs as usize].re = (*self_0)
                        .w_dir[0 as core::ffi::c_int as usize][ts as usize][qs as usize]
                        .re
                        * (*self_0)
                            .r_out_re_in_m2[ts
                            as usize][indx_4
                            as usize][row as usize][0 as core::ffi::c_int as usize];
                    (*self_0).hyb_dir_out[row as usize][ts as usize][qs as usize].im = (*self_0)
                        .w_dir[0 as core::ffi::c_int as usize][ts as usize][qs as usize]
                        .im
                        * (*self_0)
                            .r_out_re_in_m2[ts
                            as usize][indx_4
                            as usize][row as usize][0 as core::ffi::c_int as usize];
                    (*self_0).hyb_diff_out[row as usize][ts as usize][qs as usize].re = (*self_0)
                        .w_diff[1 as core::ffi::c_int as usize][ts as usize][qs as usize]
                        .re
                        * (*self_0)
                            .r_out_diff_re_in_m2[ts
                            as usize][indx_4
                            as usize][row as usize][1 as core::ffi::c_int as usize];
                    (*self_0).hyb_diff_out[row as usize][ts as usize][qs as usize].im = (*self_0)
                        .w_diff[1 as core::ffi::c_int as usize][ts as usize][qs as usize]
                        .im
                        * (*self_0)
                            .r_out_diff_re_in_m2[ts
                            as usize][indx_4
                            as usize][row as usize][1 as core::ffi::c_int as usize];
                    row += 1;
                }
                qs += 1;
            }
            ts += 1;
        }
    }
    if complex_m2 != 0 {
        if phase_interpolation != 0 {
            ts = 0 as core::ffi::c_int as WORD32;
            while ts < (*self_0).time_slots {
                qs = 0 as core::ffi::c_int as WORD32;
                while qs < 2 as core::ffi::c_int {
                    let mut indx_5: WORD32 = *((*self_0)
                        .hyb_band_to_processing_band_table)
                        .offset(qs as isize);
                    row = 0 as core::ffi::c_int as WORD32;
                    while row < (*self_0).out_ch_count {
                        let mut sum_re_dir_2: FLOAT32 = (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .re;
                        let mut sum_im_dir_2: FLOAT32 = (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .im;
                        col = 0 as core::ffi::c_int as WORD32;
                        while col < num_col_iters {
                            sum_re_dir_2
                                += (*self_0)
                                    .w_dir[col as usize][ts as usize][qs as usize]
                                    .im
                                    * (*self_0)
                                        .r_out_im_in_m2[ts
                                        as usize][indx_5 as usize][row as usize][col as usize];
                            sum_im_dir_2
                                -= (*self_0)
                                    .w_dir[col as usize][ts as usize][qs as usize]
                                    .re
                                    * (*self_0)
                                        .r_out_im_in_m2[ts
                                        as usize][indx_5 as usize][row as usize][col as usize];
                            col += 1;
                        }
                        (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .re = sum_re_dir_2;
                        (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .im = sum_im_dir_2;
                        (*self_0).hyb_diff_out[row as usize][ts as usize][qs as usize].re
                            += (*self_0)
                                .w_diff[1 as core::ffi::c_int
                                    as usize][ts as usize][qs as usize]
                                .im
                                * (*self_0)
                                    .r_out_diff_im_in_m2[ts
                                    as usize][indx_5
                                    as usize][row as usize][1 as core::ffi::c_int as usize];
                        (*self_0).hyb_diff_out[row as usize][ts as usize][qs as usize].im
                            -= (*self_0)
                                .w_diff[1 as core::ffi::c_int
                                    as usize][ts as usize][qs as usize]
                                .re
                                * (*self_0)
                                    .r_out_diff_im_in_m2[ts
                                    as usize][indx_5
                                    as usize][row as usize][1 as core::ffi::c_int as usize];
                        row += 1;
                    }
                    qs += 1;
                }
                qs = 2 as core::ffi::c_int as WORD32;
                while qs < (*self_0).hyb_band_count_max {
                    let mut indx_6: WORD32 = *((*self_0)
                        .hyb_band_to_processing_band_table)
                        .offset(qs as isize);
                    row = 0 as core::ffi::c_int as WORD32;
                    while row < (*self_0).out_ch_count {
                        let mut sum_re_dir_3: FLOAT32 = (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .re;
                        let mut sum_im_dir_3: FLOAT32 = (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .im;
                        col = 0 as core::ffi::c_int as WORD32;
                        while col < num_col_iters {
                            sum_re_dir_3
                                -= (*self_0)
                                    .w_dir[col as usize][ts as usize][qs as usize]
                                    .im
                                    * (*self_0)
                                        .r_out_im_in_m2[ts
                                        as usize][indx_6 as usize][row as usize][col as usize];
                            sum_im_dir_3
                                += (*self_0)
                                    .w_dir[col as usize][ts as usize][qs as usize]
                                    .re
                                    * (*self_0)
                                        .r_out_im_in_m2[ts
                                        as usize][indx_6 as usize][row as usize][col as usize];
                            col += 1;
                        }
                        (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .re = sum_re_dir_3;
                        (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .im = sum_im_dir_3;
                        (*self_0).hyb_diff_out[row as usize][ts as usize][qs as usize].re
                            -= (*self_0)
                                .w_diff[1 as core::ffi::c_int
                                    as usize][ts as usize][qs as usize]
                                .im
                                * (*self_0)
                                    .r_out_diff_im_in_m2[ts
                                    as usize][indx_6
                                    as usize][row as usize][1 as core::ffi::c_int as usize];
                        (*self_0).hyb_diff_out[row as usize][ts as usize][qs as usize].im
                            += (*self_0)
                                .w_diff[1 as core::ffi::c_int
                                    as usize][ts as usize][qs as usize]
                                .re
                                * (*self_0)
                                    .r_out_diff_im_in_m2[ts
                                    as usize][indx_6
                                    as usize][row as usize][1 as core::ffi::c_int as usize];
                        row += 1;
                    }
                    qs += 1;
                }
                ts += 1;
            }
        } else {
            let mut num_cols: WORD32 = if (*self_0).dir_sig_count
                + (*self_0).decor_sig_count > 1 as core::ffi::c_int
            {
                1 as WORD32
            } else {
                (*self_0).dir_sig_count + (*self_0).decor_sig_count
            };
            ts = 0 as core::ffi::c_int as WORD32;
            while ts < (*self_0).time_slots {
                qs = 0 as core::ffi::c_int as WORD32;
                while qs < 2 as core::ffi::c_int {
                    let mut indx_7: WORD32 = *((*self_0)
                        .hyb_band_to_processing_band_table)
                        .offset(qs as isize);
                    row = 0 as core::ffi::c_int as WORD32;
                    while row < (*self_0).out_ch_count {
                        let mut sum_re_dir_4: FLOAT32 = (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .re;
                        let mut sum_im_dir_4: FLOAT32 = (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .im;
                        if num_cols > 0 as core::ffi::c_int {
                            sum_re_dir_4
                                += (*self_0)
                                    .w_dir[0 as core::ffi::c_int
                                        as usize][ts as usize][qs as usize]
                                    .im
                                    * (*self_0)
                                        .r_out_im_in_m2[ts
                                        as usize][indx_7
                                        as usize][row as usize][0 as core::ffi::c_int as usize];
                            sum_im_dir_4
                                -= (*self_0)
                                    .w_dir[0 as core::ffi::c_int
                                        as usize][ts as usize][qs as usize]
                                    .re
                                    * (*self_0)
                                        .r_out_im_in_m2[ts
                                        as usize][indx_7
                                        as usize][row as usize][0 as core::ffi::c_int as usize];
                        }
                        (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .re = sum_re_dir_4;
                        (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .im = sum_im_dir_4;
                        row += 1;
                    }
                    qs += 1;
                }
                qs = 2 as core::ffi::c_int as WORD32;
                while qs < (*self_0).hyb_band_count_max {
                    let mut indx_8: WORD32 = *((*self_0)
                        .hyb_band_to_processing_band_table)
                        .offset(qs as isize);
                    row = 0 as core::ffi::c_int as WORD32;
                    while row < (*self_0).out_ch_count {
                        let mut sum_re_dir_5: FLOAT32 = (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .re;
                        let mut sum_im_dir_5: FLOAT32 = (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .im;
                        if num_cols > 0 as core::ffi::c_int {
                            sum_re_dir_5
                                -= (*self_0)
                                    .w_dir[0 as core::ffi::c_int
                                        as usize][ts as usize][qs as usize]
                                    .im
                                    * (*self_0)
                                        .r_out_im_in_m2[ts
                                        as usize][indx_8
                                        as usize][row as usize][0 as core::ffi::c_int as usize];
                            sum_im_dir_5
                                += (*self_0)
                                    .w_dir[0 as core::ffi::c_int
                                        as usize][ts as usize][qs as usize]
                                    .re
                                    * (*self_0)
                                        .r_out_im_in_m2[ts
                                        as usize][indx_8
                                        as usize][row as usize][0 as core::ffi::c_int as usize];
                        }
                        (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .re = sum_re_dir_5;
                        (*self_0)
                            .hyb_dir_out[row as usize][ts as usize][qs as usize]
                            .im = sum_im_dir_5;
                        row += 1;
                    }
                    qs += 1;
                }
                ts += 1;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_upmix_interp(
    mut m_matrix: *mut [[[FLOAT32; 2]; 2]; 28],
    mut r_matrix_float: *mut [[[FLOAT32; 2]; 2]; 28],
    mut m_matrix_prev: *mut [[FLOAT32; 2]; 2],
    mut num_rows: WORD32,
    mut num_cols: WORD32,
    mut self_0: *mut ia_mps_dec_state_struct,
    mut bs_high_rate_mode: WORD32,
) -> VOID {
    let mut ts: WORD32 = 0;
    let mut ps: WORD32 = 0;
    let mut pb: WORD32 = 0;
    let mut row: WORD32 = 0;
    let mut col: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut ks: FLOAT32 = 0.;
    let mut ms: FLOAT32 = 0.;
    let mut ls: FLOAT32 = 0.;
    let mut fl_step: FLOAT32 = 0.;
    let mut fl_base: FLOAT32 = 0.;
    pb = 0 as core::ffi::c_int as WORD32;
    while pb < (*self_0).bs_param_bands {
        row = 0 as core::ffi::c_int as WORD32;
        while row < num_rows {
            col = 0 as core::ffi::c_int as WORD32;
            while col < num_cols {
                ts = 0 as core::ffi::c_int as WORD32;
                ps = 0 as core::ffi::c_int as WORD32;
                ks = (*self_0).inv_param_slot_diff[ps as usize];
                ms = (*m_matrix
                    .offset(ps as isize))[pb as usize][row as usize][col as usize];
                ls = (*m_matrix_prev.offset(pb as isize))[row as usize][col as usize];
                fl_step = ks * (ms - ls);
                fl_base = ls + fl_step;
                i = 1 as core::ffi::c_int as WORD32;
                while i <= (*self_0).param_slot_diff[0 as core::ffi::c_int as usize] {
                    (*r_matrix_float
                        .offset(ts as isize))[pb as usize][row as usize][col as usize] = fl_base;
                    fl_base += fl_step;
                    ts += 1;
                    i += 1;
                }
                if bs_high_rate_mode != 0 {
                    ps = 1 as core::ffi::c_int as WORD32;
                    while ps < (*self_0).num_parameter_sets {
                        ks = (*self_0).inv_param_slot_diff[ps as usize];
                        ms = (*m_matrix
                            .offset(
                                ps as isize,
                            ))[pb as usize][row as usize][col as usize];
                        ls = (*m_matrix
                            .offset(
                                (ps as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                            ))[pb as usize][row as usize][col as usize];
                        fl_step = ks * (ms - ls);
                        fl_base = ls + fl_step;
                        i = 1 as core::ffi::c_int as WORD32;
                        while i <= (*self_0).param_slot_diff[ps as usize] {
                            (*r_matrix_float
                                .offset(
                                    ts as isize,
                                ))[pb as usize][row as usize][col as usize] = fl_base;
                            fl_base += fl_step;
                            ts += 1;
                            i += 1;
                        }
                        ps += 1;
                    }
                }
                col += 1;
            }
            row += 1;
        }
        pb += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_upmix_interp_type1(
    mut m_matrix: *mut [[[FLOAT32; 2]; 2]; 28],
    mut r_matrix_float: *mut [[[FLOAT32; 2]; 2]; 28],
    mut m_matrix_prev: *mut [[FLOAT32; 2]; 2],
    mut num_rows: WORD32,
    mut num_cols: WORD32,
    mut self_0: *mut ia_mps_dec_state_struct,
    mut bs_high_rate_mode: WORD32,
) -> VOID {
    let mut ts: WORD32 = 0;
    let mut ps: WORD32 = 0;
    let mut pb: WORD32 = 0;
    let mut row: WORD32 = 0;
    let mut col: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut ks: FLOAT32 = 0.;
    let mut ms: FLOAT32 = 0.;
    let mut ls: FLOAT32 = 0.;
    let mut fl_step: FLOAT32 = 0.;
    let mut fl_base: FLOAT32 = 0.;
    pb = 0 as core::ffi::c_int as WORD32;
    while pb < (*self_0).bs_param_bands {
        row = 0 as core::ffi::c_int as WORD32;
        while row < num_rows {
            col = 0 as core::ffi::c_int as WORD32;
            while col < num_cols {
                ts = 0 as core::ffi::c_int as WORD32;
                ps = 0 as core::ffi::c_int as WORD32;
                ks = (*self_0).inv_param_slot_diff[ps as usize];
                ms = (*m_matrix
                    .offset(ps as isize))[pb as usize][row as usize][col as usize];
                ls = (*m_matrix_prev.offset(pb as isize))[row as usize][col as usize];
                fl_step = ks * (ms - ls);
                fl_base = ls + fl_step;
                i = 1 as core::ffi::c_int as WORD32;
                while i <= (*self_0).param_slot_diff[0 as core::ffi::c_int as usize] {
                    (*r_matrix_float
                        .offset(ts as isize))[pb as usize][row as usize][col as usize] = fl_base;
                    fl_base += fl_step;
                    ts += 1;
                    i += 1;
                }
                if bs_high_rate_mode != 0 {
                    ps = 1 as core::ffi::c_int as WORD32;
                    while ps < (*self_0).num_parameter_sets {
                        ks = (*self_0).inv_param_slot_diff[ps as usize];
                        ms = (*m_matrix
                            .offset(
                                ps as isize,
                            ))[pb as usize][row as usize][col as usize];
                        ls = (*m_matrix
                            .offset(
                                (ps as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                            ))[pb as usize][row as usize][col as usize];
                        fl_step = ks * (ms - ls);
                        fl_base = ls + fl_step;
                        i = 1 as core::ffi::c_int as WORD32;
                        while i <= (*self_0).param_slot_diff[ps as usize] {
                            (*r_matrix_float
                                .offset(
                                    ts as isize,
                                ))[pb as usize][row as usize][col as usize] = fl_base;
                            fl_base += fl_step;
                            ts += 1;
                            i += 1;
                        }
                        ps += 1;
                    }
                }
                col += 1;
            }
            row += 1;
        }
        pb += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_upmix_interp_type2(
    mut m_matrix: *mut [[[FLOAT32; 2]; 2]; 28],
    mut r_matrix_float: *mut [[[FLOAT32; 2]; 2]; 28],
    mut m_matrix_prev: *mut [[FLOAT32; 2]; 2],
    mut num_rows: WORD32,
    mut self_0: *mut ia_mps_dec_state_struct,
    mut col: WORD32,
) -> VOID {
    let mut ts: WORD32 = 0;
    let mut ps: WORD32 = 0;
    let mut pb: WORD32 = 0;
    let mut row: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut ks: FLOAT32 = 0.;
    let mut ms: FLOAT32 = 0.;
    let mut ls: FLOAT32 = 0.;
    let mut fl_step: FLOAT32 = 0.;
    let mut fl_base: FLOAT32 = 0.;
    pb = 0 as core::ffi::c_int as WORD32;
    while pb < (*self_0).bs_param_bands {
        row = 0 as core::ffi::c_int as WORD32;
        while row < num_rows {
            ts = 0 as core::ffi::c_int as WORD32;
            ps = 0 as core::ffi::c_int as WORD32;
            ks = (*self_0).inv_param_slot_diff[ps as usize];
            ms = (*m_matrix
                .offset(ps as isize))[pb as usize][row as usize][col as usize];
            ls = (*m_matrix_prev.offset(pb as isize))[row as usize][col as usize];
            fl_step = ks * (ms - ls);
            fl_base = ls + fl_step;
            i = 1 as core::ffi::c_int as WORD32;
            while i <= (*self_0).param_slot_diff[0 as core::ffi::c_int as usize] {
                (*r_matrix_float
                    .offset(ts as isize))[pb as usize][row as usize][col as usize] = fl_base;
                fl_base += fl_step;
                ts += 1;
                i += 1;
            }
            ps = 1 as core::ffi::c_int as WORD32;
            while ps < (*self_0).num_parameter_sets {
                ks = (*self_0).inv_param_slot_diff[ps as usize];
                ms = (*m_matrix
                    .offset(ps as isize))[pb as usize][row as usize][col as usize];
                ls = (*m_matrix
                    .offset(
                        (ps as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                    ))[pb as usize][row as usize][col as usize];
                fl_step = ks * (ms - ls);
                fl_base = ls + fl_step;
                i = 1 as core::ffi::c_int as WORD32;
                while i <= (*self_0).param_slot_diff[ps as usize] {
                    (*r_matrix_float
                        .offset(ts as isize))[pb as usize][row as usize][col as usize] = fl_base;
                    fl_base += fl_step;
                    ts += 1;
                    i += 1;
                }
                ps += 1;
            }
            row += 1;
        }
        pb += 1;
    }
}
unsafe extern "C" fn ixheaacd_mps_angle_interpolation(
    mut angle1: FLOAT32,
    mut angle2: FLOAT32,
    mut alpha: FLOAT32,
    mut step: *mut FLOAT32,
) -> FLOAT32 {
    while angle2 - angle1 > P_PI as FLOAT32 {
        angle1 = angle1 + 2.0f32 * P_PI as FLOAT32;
    }
    while angle1 - angle2 > P_PI as FLOAT32 {
        angle2 = angle2 + 2.0f32 * P_PI as FLOAT32;
    }
    *step = angle2 - angle1;
    return (1 as core::ffi::c_int as FLOAT32 - alpha) * angle1 + alpha * angle2;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_phase_interpolation(
    mut pl: *mut [FLOAT32; 28],
    mut pr: *mut [FLOAT32; 28],
    mut pl_prev: *mut FLOAT32,
    mut pr_prev: *mut FLOAT32,
    mut r_re: *mut [[FLOAT32; 2]; 28],
    mut r_im: *mut [[FLOAT32; 2]; 28],
    mut self_0: *mut ia_mps_dec_state_struct,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut ts: WORD32 = 0;
    let mut ps: WORD32 = 0;
    let mut pb: WORD32 = 0;
    let mut step_l: FLOAT32 = 0.;
    let mut step_r: FLOAT32 = 0.;
    let mut alpha: FLOAT32 = 0.;
    let mut tl: FLOAT32 = 0.;
    let mut tr: FLOAT32 = 0.;
    pb = 0 as core::ffi::c_int as WORD32;
    while pb < (*self_0).bs_param_bands {
        ps = 0 as core::ffi::c_int as WORD32;
        ts = 0 as core::ffi::c_int as WORD32;
        alpha = (*self_0).inv_param_slot_diff[ps as usize];
        tl = ixheaacd_mps_angle_interpolation(
            *pl_prev.offset(pb as isize),
            (*pl.offset(ps as isize))[pb as usize],
            alpha,
            &mut step_l,
        );
        tr = ixheaacd_mps_angle_interpolation(
            *pr_prev.offset(pb as isize),
            (*pr.offset(ps as isize))[pb as usize],
            alpha,
            &mut step_r,
        );
        step_l *= alpha;
        step_r *= alpha;
        i = 1 as core::ffi::c_int as WORD32;
        while i <= (*self_0).param_slot_diff[ps as usize] {
            (*r_re.offset(ts as isize))[pb as usize][0 as core::ffi::c_int as usize] = cos(
                tl as core::ffi::c_double,
            ) as FLOAT32;
            (*r_im.offset(ts as isize))[pb as usize][0 as core::ffi::c_int as usize] = sin(
                tl as core::ffi::c_double,
            ) as FLOAT32;
            tl += step_l;
            (*r_re.offset(ts as isize))[pb as usize][1 as core::ffi::c_int as usize] = cos(
                tr as core::ffi::c_double,
            ) as FLOAT32;
            (*r_im.offset(ts as isize))[pb as usize][1 as core::ffi::c_int as usize] = sin(
                tr as core::ffi::c_double,
            ) as FLOAT32;
            tr += step_r;
            ts += 1;
            i += 1;
        }
        ps = 1 as core::ffi::c_int as WORD32;
        while ps < (*self_0).num_parameter_sets {
            let mut alpha_0: FLOAT32 = (*self_0).inv_param_slot_diff[ps as usize];
            tl = ixheaacd_mps_angle_interpolation(
                (*pl
                    .offset(
                        (ps as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                    ))[pb as usize],
                (*pl.offset(ps as isize))[pb as usize],
                alpha_0,
                &mut step_l,
            );
            tr = ixheaacd_mps_angle_interpolation(
                (*pr
                    .offset(
                        (ps as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                    ))[pb as usize],
                (*pr.offset(ps as isize))[pb as usize],
                alpha_0,
                &mut step_r,
            );
            step_l *= alpha_0;
            step_r *= alpha_0;
            i = 1 as core::ffi::c_int as WORD32;
            while i <= (*self_0).param_slot_diff[ps as usize] {
                if ts < 72 as core::ffi::c_int && pb < 28 as core::ffi::c_int {
                    (*r_re
                        .offset(
                            ts as isize,
                        ))[pb as usize][0 as core::ffi::c_int as usize] = cos(
                        tl as core::ffi::c_double,
                    ) as FLOAT32;
                    (*r_im
                        .offset(
                            ts as isize,
                        ))[pb as usize][0 as core::ffi::c_int as usize] = sin(
                        tl as core::ffi::c_double,
                    ) as FLOAT32;
                    tl += step_l;
                    (*r_re
                        .offset(
                            ts as isize,
                        ))[pb as usize][1 as core::ffi::c_int as usize] = cos(
                        tr as core::ffi::c_double,
                    ) as FLOAT32;
                    (*r_im
                        .offset(
                            ts as isize,
                        ))[pb as usize][1 as core::ffi::c_int as usize] = sin(
                        tr as core::ffi::c_double,
                    ) as FLOAT32;
                    tr += step_r;
                }
                ts += 1;
                if ts > 71 as core::ffi::c_int {
                    ts = 0 as core::ffi::c_int as WORD32;
                    break;
                } else if pb > 27 as core::ffi::c_int {
                    pb = 0 as core::ffi::c_int as WORD32;
                    break;
                } else {
                    i += 1;
                }
            }
            ps += 1;
        }
        pb += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_init_pre_and_post_matrix(
    mut self_0: *mut ia_mps_dec_state_struct,
) -> VOID {
    memset(
        ((*self_0).m1_param_re_prev).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ((MAX_PARAMETER_BANDS * MAX_M_OUTPUT * MAX_M_INPUT) as size_t)
            .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
    );
    memset(
        ((*self_0).m1_param_im_prev).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ((MAX_PARAMETER_BANDS * MAX_M_OUTPUT * MAX_M_INPUT) as size_t)
            .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
    );
    memset(
        ((*self_0).m1_param_re_prev).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ((MAX_PARAMETER_BANDS * MAX_M_OUTPUT * MAX_M_INPUT) as size_t)
            .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
    );
    memset(
        ((*self_0).m2_decor_re_prev).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ((MAX_PARAMETER_BANDS * MAX_M_OUTPUT * MAX_M_INPUT) as size_t)
            .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
    );
    memset(
        ((*self_0).m2_resid_re_prev).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ((MAX_PARAMETER_BANDS * MAX_M_OUTPUT * MAX_M_INPUT) as size_t)
            .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
    );
    memset(
        ((*self_0).m2_resid_im_prev).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ((MAX_PARAMETER_BANDS * MAX_M_OUTPUT * MAX_M_INPUT) as size_t)
            .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
    );
}
