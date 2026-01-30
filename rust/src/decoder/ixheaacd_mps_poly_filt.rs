extern "C" {
    fn ixheaacd_mps_synth_calc_fft(
        ptr_x: *mut FLOAT32,
        ptr_y: *mut FLOAT32,
        nPass: WORD32,
    ) -> VOID;
    fn ixheaacd_mps_complex_fft(
        xr: *mut FLOAT32,
        xi: *mut FLOAT32,
        nlength: WORD32,
    ) -> VOID;
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    static mut ixheaacd_mps_synt_out_calc: Option<
        unsafe extern "C" fn(WORD32, *mut FLOAT32, *mut FLOAT32, *const FLOAT32) -> VOID,
    >;
    static ixheaacd_mps_polyphase_filter_coeff: [FLOAT32; 640];
    static ixheaacd_mps_post_twid: [FLOAT32; 30];
    static ixheaacd_mps_pre_twid: [FLOAT32; 64];
    static ixheaacd_ldmps_polyphase_filter_coeff: [FLOAT32; 1280];
    static ixheaacd_ldmps_pre_twid: [FLOAT32; 32];
    static ixheaacd_mps_post_re_32: [FLOAT32; 64];
    static ixheaacd_mps_post_im_32: [FLOAT32; 64];
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
pub const MAX_NUM_QMF_BANDS_SAC: core::ffi::c_int = 128 as core::ffi::c_int;
pub const POLY_PHASE_SYNTH_SIZE: core::ffi::c_int = 1280 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_synt_init(mut state: *mut FLOAT32) -> VOID {
    memset(
        state as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<FLOAT32>() as size_t)
            .wrapping_mul(POLY_PHASE_SYNTH_SIZE as size_t),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_synt_post_fft_twiddle_dec(
    mut resolution: WORD32,
    mut fin_re: *mut FLOAT32,
    mut fin_im: *mut FLOAT32,
    mut table_re: *const FLOAT32,
    mut table_im: *const FLOAT32,
    mut state: *mut FLOAT32,
) -> VOID {
    let mut l: WORD32 = 0;
    l = 0 as core::ffi::c_int as WORD32;
    while l < 2 as WORD32 * resolution {
        *state
            .offset(
                (2 as core::ffi::c_int * resolution as core::ffi::c_int
                    - l as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
            ) = *fin_re.offset(l as isize) * *table_re.offset(l as isize)
            + *fin_im.offset(l as isize) * *table_im.offset(l as isize);
        l += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_synt_out_calc_dec(
    mut resolution: WORD32,
    mut out: *mut FLOAT32,
    mut state: *mut FLOAT32,
    mut filter_coeff: *const FLOAT32,
) -> VOID {
    let mut l: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut out1: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut out2: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut state1: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut state2: *mut FLOAT32 = 0 as *mut FLOAT32;
    out1 = out;
    out2 = out.offset(resolution as isize);
    state1 = state;
    state2 = state.offset((3 as WORD32 * resolution) as isize);
    k = 0 as core::ffi::c_int as WORD32;
    while k < 5 as core::ffi::c_int {
        l = 0 as core::ffi::c_int as WORD32;
        while l < resolution {
            let fresh0 = state1;
            state1 = state1.offset(1);
            let fresh1 = filter_coeff;
            filter_coeff = filter_coeff.offset(1);
            let fresh2 = out1;
            out1 = out1.offset(1);
            *fresh2 = *fresh0 * *fresh1;
            let fresh3 = state2;
            state2 = state2.offset(1);
            let fresh4 = filter_coeff;
            filter_coeff = filter_coeff.offset(1);
            let fresh5 = out2;
            out2 = out2.offset(1);
            *fresh5 = *fresh3 * *fresh4;
            l += 1;
        }
        out1 = out1.offset(resolution as isize);
        out2 = out2.offset(resolution as isize);
        state1 = state1.offset((3 as WORD32 * resolution) as isize);
        state2 = state2.offset((3 as WORD32 * resolution) as isize);
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_synt_out_calc_dec_ldmps(
    mut resolution: WORD32,
    mut out: *mut FLOAT32,
    mut state: *mut FLOAT32,
    mut filter_coeff: *const FLOAT32,
) -> VOID {
    let mut l: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut out1: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut out2: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut state1: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut state2: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut filter1: *const FLOAT32 = 0 as *const FLOAT32;
    let mut filter2: *const FLOAT32 = 0 as *const FLOAT32;
    filter1 = filter_coeff;
    filter2 = filter_coeff.offset(resolution as isize);
    out1 = out;
    out2 = out.offset(resolution as isize);
    state1 = state;
    state2 = state.offset((3 as WORD32 * resolution) as isize);
    k = 0 as core::ffi::c_int as WORD32;
    while k < 5 as core::ffi::c_int {
        l = 0 as core::ffi::c_int as WORD32;
        while l < resolution {
            let fresh6 = state1;
            state1 = state1.offset(1);
            let fresh7 = filter1;
            filter1 = filter1.offset(1);
            let fresh8 = out1;
            out1 = out1.offset(1);
            *fresh8 = *fresh6 * *fresh7;
            let fresh9 = state2;
            state2 = state2.offset(1);
            let fresh10 = filter2;
            filter2 = filter2.offset(1);
            let fresh11 = out2;
            out2 = out2.offset(1);
            *fresh11 = *fresh9 * *fresh10;
            l += 1;
        }
        filter1 = filter1.offset(resolution as isize);
        filter2 = filter2.offset(resolution as isize);
        out1 = out1.offset(resolution as isize);
        out2 = out2.offset(resolution as isize);
        state1 = state1.offset((3 as WORD32 * resolution) as isize);
        state2 = state2.offset((3 as WORD32 * resolution) as isize);
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_synt_out_calc_dec_ldmps_32(
    mut resolution: WORD32,
    mut out: *mut FLOAT32,
    mut state: *mut FLOAT32,
    mut filter_coeff: *const FLOAT32,
) -> VOID {
    let mut l: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut out1: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut out2: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut state1: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut state2: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut filter1: *const FLOAT32 = 0 as *const FLOAT32;
    let mut filter2: *const FLOAT32 = 0 as *const FLOAT32;
    filter1 = filter_coeff;
    filter2 = filter_coeff.offset((2 as WORD32 * resolution) as isize);
    out1 = out;
    out2 = out.offset(resolution as isize);
    state1 = state;
    state2 = state.offset((3 as WORD32 * resolution) as isize);
    k = 0 as core::ffi::c_int as WORD32;
    while k < 5 as core::ffi::c_int {
        l = 0 as core::ffi::c_int as WORD32;
        while l < resolution {
            let fresh12 = state1;
            state1 = state1.offset(1);
            let fresh13 = out1;
            out1 = out1.offset(1);
            *fresh13 = *fresh12
                * (*filter1.offset((2 as WORD32 * l) as isize)
                    + *filter1
                        .offset(
                            (2 as core::ffi::c_int * l as core::ffi::c_int
                                + 1 as core::ffi::c_int) as isize,
                        )) / 2 as core::ffi::c_int as FLOAT32;
            let fresh14 = state2;
            state2 = state2.offset(1);
            let fresh15 = out2;
            out2 = out2.offset(1);
            *fresh15 = *fresh14
                * (*filter2.offset((2 as WORD32 * l) as isize)
                    + *filter2
                        .offset(
                            (2 as core::ffi::c_int * l as core::ffi::c_int
                                + 1 as core::ffi::c_int) as isize,
                        )) / 2 as core::ffi::c_int as FLOAT32;
            l += 1;
        }
        filter1 = filter1.offset((4 as WORD32 * resolution) as isize);
        filter2 = filter2.offset((4 as WORD32 * resolution) as isize);
        out1 = out1.offset(resolution as isize);
        out2 = out2.offset(resolution as isize);
        state1 = state1.offset((3 as WORD32 * resolution) as isize);
        state2 = state2.offset((3 as WORD32 * resolution) as isize);
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_synth_pre_twidle(
    mut out_re: *mut FLOAT32,
    mut out_im: *mut FLOAT32,
    mut c_in: *mut FLOAT32,
    mut len: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut c_s: *mut FLOAT32 = c_in;
    let mut p_re_s: *mut FLOAT32 = out_re;
    let mut p_im_s: *mut FLOAT32 = out_im;
    let mut c_e: *mut FLOAT32 = c_in
        .offset((len << 1 as core::ffi::c_int) as isize)
        .offset(-(1 as core::ffi::c_int as isize));
    let mut p_im_e: *mut FLOAT32 = out_im
        .offset(len as isize)
        .offset(-(1 as core::ffi::c_int as isize));
    let mut p_re_e: *mut FLOAT32 = out_re
        .offset(len as isize)
        .offset(-(1 as core::ffi::c_int as isize));
    let mut prtw: *const FLOAT32 = ixheaacd_mps_pre_twid.as_ptr();
    i = 0 as core::ffi::c_int as WORD32;
    while i < len {
        let fresh16 = c_s;
        c_s = c_s.offset(1);
        *p_re_s = *fresh16 * *prtw;
        p_re_s = p_re_s.offset(1);
        let fresh17 = c_s;
        c_s = c_s.offset(-1);
        *p_im_s = -(*fresh17 * *prtw);
        p_im_s = p_im_s.offset(1);
        let fresh18 = c_e;
        c_e = c_e.offset(-1);
        *p_im_s = *fresh18 * *prtw;
        p_im_s = p_im_s.offset(-1);
        let fresh19 = c_e;
        c_e = c_e.offset(1);
        let fresh20 = prtw;
        prtw = prtw.offset(1);
        *p_re_s = *fresh19 * *fresh20;
        p_re_s = p_re_s.offset(-1);
        let fresh21 = c_e;
        c_e = c_e.offset(-1);
        *p_im_s += *fresh21 * *prtw;
        p_im_s = p_im_s.offset(1);
        let fresh22 = c_e;
        c_e = c_e.offset(-1);
        *p_re_s += *fresh22 * *prtw;
        p_re_s = p_re_s.offset(1);
        let fresh23 = c_s;
        c_s = c_s.offset(1);
        *p_re_s -= *fresh23 * *prtw;
        p_re_s = p_re_s.offset(1);
        let fresh24 = c_s;
        c_s = c_s.offset(1);
        let fresh25 = prtw;
        prtw = prtw.offset(1);
        *p_im_s += *fresh24 * *fresh25;
        p_im_s = p_im_s.offset(1);
        let fresh26 = c_e;
        c_e = c_e.offset(-1);
        *p_im_e = *fresh26 * *prtw;
        p_im_e = p_im_e.offset(-1);
        let fresh27 = c_e;
        c_e = c_e.offset(1);
        *p_re_e = -(*fresh27 * *prtw);
        p_re_e = p_re_e.offset(-1);
        let fresh28 = c_s;
        c_s = c_s.offset(1);
        *p_re_e = *fresh28 * *prtw;
        p_re_e = p_re_e.offset(1);
        let fresh29 = c_s;
        c_s = c_s.offset(-1);
        let fresh30 = prtw;
        prtw = prtw.offset(1);
        *p_im_e = *fresh29 * *fresh30;
        p_im_e = p_im_e.offset(1);
        let fresh31 = c_s;
        c_s = c_s.offset(1);
        *p_re_e += *fresh31 * *prtw;
        p_re_e = p_re_e.offset(-1);
        let fresh32 = c_s;
        c_s = c_s.offset(1);
        *p_im_e += *fresh32 * *prtw;
        p_im_e = p_im_e.offset(-1);
        let fresh33 = c_e;
        c_e = c_e.offset(-1);
        *p_im_e -= *fresh33 * *prtw;
        p_im_e = p_im_e.offset(-1);
        let fresh34 = c_e;
        c_e = c_e.offset(-1);
        let fresh35 = prtw;
        prtw = prtw.offset(1);
        *p_re_e += *fresh34 * *fresh35;
        p_re_e = p_re_e.offset(-1);
        i += 4 as core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_synth_post_twidle(
    mut state: *mut FLOAT32,
    mut out_re: *mut FLOAT32,
    mut out_im: *mut FLOAT32,
    mut len: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut x_0: FLOAT32 = 0.;
    let mut x_1: FLOAT32 = 0.;
    let mut x_2: FLOAT32 = 0.;
    let mut x_3: FLOAT32 = 0.;
    let mut p_re_e: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut p_im_e: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut potw: *const FLOAT32 = ixheaacd_mps_post_twid.as_ptr();
    let mut p_re_s: *mut FLOAT32 = out_re;
    let mut p_im_s: *mut FLOAT32 = out_im;
    p_re_e = p_re_s.offset((len as core::ffi::c_int - 2 as core::ffi::c_int) as isize);
    p_im_e = p_im_s.offset((len as core::ffi::c_int - 2 as core::ffi::c_int) as isize);
    x_0 = *p_re_e;
    x_1 = *p_re_e.offset(1 as core::ffi::c_int as isize);
    x_2 = *p_im_e;
    x_3 = *p_im_e.offset(1 as core::ffi::c_int as isize);
    *p_re_e.offset(1 as core::ffi::c_int as isize) = -*p_re_s
        .offset(1 as core::ffi::c_int as isize);
    *p_im_e.offset(1 as core::ffi::c_int as isize) = -*p_im_s;
    *p_im_s = *p_im_s.offset(1 as core::ffi::c_int as isize);
    i = 5 as core::ffi::c_int as WORD32;
    while i < len {
        let fresh36 = potw;
        potw = potw.offset(1);
        let mut twdr: FLOAT32 = *fresh36;
        let fresh37 = potw;
        potw = potw.offset(1);
        let mut twdi: FLOAT32 = *fresh37;
        let mut tmp: FLOAT32 = 0.;
        *p_re_e = x_0 * twdi;
        *p_re_e += x_1 * twdr;
        p_re_e = p_re_e.offset(-1);
        p_re_s = p_re_s.offset(1);
        *p_re_s = x_0 * twdr;
        *p_re_s -= x_1 * twdi;
        p_re_s = p_re_s.offset(1);
        let fresh38 = p_re_e;
        p_re_e = p_re_e.offset(-1);
        x_1 = *fresh38;
        let fresh39 = p_re_e;
        p_re_e = p_re_e.offset(1);
        x_0 = *fresh39;
        let fresh40 = p_re_s;
        p_re_s = p_re_s.offset(1);
        *p_re_e = *fresh40 * twdi;
        *p_re_e += -(*p_re_s * twdr);
        p_re_e = p_re_e.offset(-1);
        let fresh41 = p_re_s;
        p_re_s = p_re_s.offset(-1);
        tmp = *fresh41 * twdi;
        *p_re_s = tmp + *p_re_s * twdr;
        *p_im_e = -(x_2 * twdr);
        *p_im_e += x_3 * twdi;
        p_im_e = p_im_e.offset(-1);
        p_im_s = p_im_s.offset(1);
        *p_im_s = -(x_2 * twdi);
        *p_im_s -= x_3 * twdr;
        p_im_s = p_im_s.offset(1);
        let fresh42 = p_im_e;
        p_im_e = p_im_e.offset(-1);
        x_3 = *fresh42;
        let fresh43 = p_im_e;
        p_im_e = p_im_e.offset(1);
        x_2 = *fresh43;
        let fresh44 = p_im_s;
        p_im_s = p_im_s.offset(1);
        *p_im_e = -(*fresh44 * twdr);
        *p_im_e -= *p_im_s * twdi;
        p_im_e = p_im_e.offset(-1);
        let fresh45 = p_im_s;
        p_im_s = p_im_s.offset(-1);
        tmp = *fresh45 * twdr;
        *p_im_s = tmp - *p_im_s * twdi;
        i += 4 as core::ffi::c_int;
    }
    *p_re_e = 0.7071067f32 * (x_1 + x_0);
    *p_im_e = 0.7071067f32 * (x_3 - x_2);
    *p_re_s.offset(1 as core::ffi::c_int as isize) = -0.7071067f32 * (x_1 - x_0);
    *p_im_s.offset(1 as core::ffi::c_int as isize) = -0.7071067f32 * (x_3 + x_2);
    i = 0 as core::ffi::c_int as WORD32;
    while i < len {
        *state.offset(i as isize) = *out_im.offset(i as isize)
            - *out_re.offset(i as isize);
        *state.offset((len + i) as isize) = *out_im
            .offset(
                (len as core::ffi::c_int - i as core::ffi::c_int - 1 as core::ffi::c_int)
                    as isize,
            )
            + *out_re
                .offset(
                    (len as core::ffi::c_int - i as core::ffi::c_int
                        - 1 as core::ffi::c_int) as isize,
                );
        *state
            .offset(
                (len as core::ffi::c_int - i as core::ffi::c_int - 1 as core::ffi::c_int)
                    as isize,
            ) = *out_im
            .offset(
                (len as core::ffi::c_int - i as core::ffi::c_int - 1 as core::ffi::c_int)
                    as isize,
            )
            - *out_re
                .offset(
                    (len as core::ffi::c_int - i as core::ffi::c_int
                        - 1 as core::ffi::c_int) as isize,
                );
        *state
            .offset(
                (2 as core::ffi::c_int * len as core::ffi::c_int - i as core::ffi::c_int
                    - 1 as core::ffi::c_int) as isize,
            ) = *out_im.offset(i as isize) + *out_re.offset(i as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_synt_pre_twiddle_dec(
    mut ptr_in: *mut FLOAT32,
    mut table: *const FLOAT32,
    mut fin_re: *mut FLOAT32,
    mut fin_im: *mut FLOAT32,
    mut resolution: WORD32,
) -> VOID {
    let mut k: WORD32 = 0;
    let mut c_s: *mut FLOAT32 = ptr_in;
    let mut p_re_s: *mut FLOAT32 = fin_re;
    let mut p_im_s: *mut FLOAT32 = fin_im;
    let mut c_e: *mut FLOAT32 = ptr_in
        .offset((resolution << 1 as core::ffi::c_int) as isize)
        .offset(-(1 as core::ffi::c_int as isize));
    let mut p_im_e: *mut FLOAT32 = fin_im
        .offset(resolution as isize)
        .offset(-(1 as core::ffi::c_int as isize));
    let mut p_re_e: *mut FLOAT32 = fin_re
        .offset(resolution as isize)
        .offset(-(1 as core::ffi::c_int as isize));
    k = 0 as core::ffi::c_int as WORD32;
    while k < resolution {
        let fresh46 = c_s;
        c_s = c_s.offset(1);
        *p_re_s = *fresh46 * *table;
        *p_im_s = *c_s * *table;
        let fresh47 = c_e;
        c_e = c_e.offset(-1);
        *p_re_e = *fresh47 * *table;
        let fresh48 = table;
        table = table.offset(1);
        *p_im_e = -*c_e * *fresh48;
        let fresh49 = c_s;
        c_s = c_s.offset(-1);
        *p_re_s += *fresh49 * *table;
        let fresh50 = c_s;
        c_s = c_s.offset(1);
        *p_im_s += -*fresh50 * *table;
        p_re_s = p_re_s.offset(1);
        p_im_s = p_im_s.offset(1);
        c_s = c_s.offset(1);
        let fresh51 = c_e;
        c_e = c_e.offset(1);
        *p_re_e += *fresh51 * *table;
        let fresh52 = c_e;
        c_e = c_e.offset(-1);
        let fresh53 = table;
        table = table.offset(1);
        *p_im_e += *fresh52 * *fresh53;
        p_re_e = p_re_e.offset(-1);
        p_im_e = p_im_e.offset(-1);
        c_e = c_e.offset(-1);
        k += 2 as core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_synt_calc(
    mut self_0: *mut ia_mps_dec_state_struct,
) -> VOID {
    let mut k: WORD32 = 0;
    let mut l: WORD32 = 0;
    let mut ts: WORD32 = 0;
    let mut ch: WORD32 = 0;
    let mut state: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut tmp_state: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut out: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut filt_coeff: *const FLOAT32 = 0 as *const FLOAT32;
    let mut tmp_buf: *mut FLOAT32 = ((*self_0).tmp_buf).as_mut_ptr();
    let mut fin_re: [FLOAT32; 64] = [
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
    let mut fin_im: [FLOAT32; 64] = [
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
    let mut resolution: WORD32 = (*self_0).resolution;
    let mut m_resolution: WORD32 = resolution >> 1 as core::ffi::c_int;
    let mut ixheaacd_mps_post_re: *const FLOAT32 = 0 as *const FLOAT32;
    let mut ixheaacd_mps_post_im: *const FLOAT32 = 0 as *const FLOAT32;
    let mut ixheaacd_mps_synt_out_calc_pointer: Option<
        unsafe extern "C" fn(WORD32, *mut FLOAT32, *mut FLOAT32, *const FLOAT32) -> VOID,
    > = None;
    if (*self_0).ldmps_config.ldmps_present_flag != 0 {
        ixheaacd_mps_synt_out_calc_pointer = Some(
            ixheaacd_mps_synt_out_calc_dec_ldmps
                as unsafe extern "C" fn(
                    WORD32,
                    *mut FLOAT32,
                    *mut FLOAT32,
                    *const FLOAT32,
                ) -> VOID,
        )
            as Option<
                unsafe extern "C" fn(
                    WORD32,
                    *mut FLOAT32,
                    *mut FLOAT32,
                    *const FLOAT32,
                ) -> VOID,
            >;
        filt_coeff = ixheaacd_ldmps_polyphase_filter_coeff.as_ptr();
    } else {
        ixheaacd_mps_synt_out_calc_pointer = ixheaacd_mps_synt_out_calc;
        filt_coeff = ixheaacd_mps_polyphase_filter_coeff.as_ptr();
    }
    if (*self_0).qmf_band_count == 32 as core::ffi::c_int {
        ch = 0 as core::ffi::c_int as WORD32;
        while ch < (*self_0).out_ch_count {
            tmp_state = ((*self_0).qmf_filt_state[ch as usize]).as_mut_ptr();
            state = &mut *tmp_buf
                .offset(((*self_0).time_slots * 2 as WORD32 * resolution) as isize)
                as *mut FLOAT32;
            memcpy(
                state as *mut core::ffi::c_void,
                tmp_state as *const core::ffi::c_void,
                (::core::mem::size_of::<FLOAT32>() as size_t)
                    .wrapping_mul(18 as size_t)
                    .wrapping_mul(resolution as size_t),
            );
            out = &mut *tmp_buf
                .offset((74 as core::ffi::c_int * MAX_NUM_QMF_BANDS_SAC) as isize)
                as *mut FLOAT32;
            ixheaacd_mps_post_re = ixheaacd_mps_post_re_32.as_ptr();
            ixheaacd_mps_post_im = ixheaacd_mps_post_im_32.as_ptr();
            ts = 0 as core::ffi::c_int as WORD32;
            while ts < (*self_0).time_slots {
                state = state.offset(-((2 as WORD32 * resolution) as isize));
                ixheaacd_mps_synt_pre_twiddle_dec(
                    &mut (*(*(*((*self_0).qmf_out_dir).as_mut_ptr().offset(ch as isize))
                        .as_mut_ptr()
                        .offset(ts as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .re,
                    ixheaacd_ldmps_pre_twid.as_ptr(),
                    fin_re.as_mut_ptr(),
                    fin_im.as_mut_ptr(),
                    resolution,
                );
                k = resolution;
                while k < 2 as WORD32 * resolution {
                    fin_re[k as usize] = 0 as core::ffi::c_int as FLOAT32;
                    fin_im[k as usize] = 0 as core::ffi::c_int as FLOAT32;
                    k += 1;
                }
                ixheaacd_mps_complex_fft(
                    fin_re.as_mut_ptr(),
                    fin_im.as_mut_ptr(),
                    2 as WORD32 * resolution,
                );
                ixheaacd_mps_synt_post_fft_twiddle_dec(
                    resolution,
                    fin_re.as_mut_ptr(),
                    fin_im.as_mut_ptr(),
                    ixheaacd_mps_post_re,
                    ixheaacd_mps_post_im,
                    state,
                );
                ixheaacd_mps_synt_out_calc_dec_ldmps_32(
                    resolution,
                    out,
                    state,
                    filt_coeff,
                );
                k = 0 as core::ffi::c_int as WORD32;
                while k < resolution {
                    let mut acc: FLOAT32 = *out.offset(k as isize);
                    l = 1 as core::ffi::c_int as WORD32;
                    while l < 10 as core::ffi::c_int {
                        acc += *out.offset((resolution * l + k) as isize);
                        l += 1;
                    }
                    (*((*self_0).output_buffer)
                        .offset(
                            ch as isize,
                        ))[((*self_0).qmf_band_count * ts + k) as usize] = acc;
                    k += 1;
                }
                ts += 1;
            }
            memcpy(
                tmp_state as *mut core::ffi::c_void,
                state as *const core::ffi::c_void,
                (::core::mem::size_of::<FLOAT32>() as size_t)
                    .wrapping_mul(18 as size_t)
                    .wrapping_mul(resolution as size_t),
            );
            ch += 1;
        }
    } else {
        ch = 0 as core::ffi::c_int as WORD32;
        while ch < (*self_0).out_ch_count {
            tmp_state = ((*self_0).qmf_filt_state[ch as usize]).as_mut_ptr();
            state = &mut *tmp_buf
                .offset(((*self_0).time_slots * 2 as WORD32 * resolution) as isize)
                as *mut FLOAT32;
            memcpy(
                state as *mut core::ffi::c_void,
                tmp_state as *const core::ffi::c_void,
                (::core::mem::size_of::<FLOAT32>() as size_t)
                    .wrapping_mul(18 as size_t)
                    .wrapping_mul(resolution as size_t),
            );
            out = &mut *tmp_buf
                .offset((74 as core::ffi::c_int * MAX_NUM_QMF_BANDS_SAC) as isize)
                as *mut FLOAT32;
            ts = 0 as core::ffi::c_int as WORD32;
            while ts < (*self_0).time_slots {
                state = state.offset(-((2 as WORD32 * resolution) as isize));
                ixheaacd_mps_synth_pre_twidle(
                    fin_re.as_mut_ptr(),
                    fin_im.as_mut_ptr(),
                    &mut (*(*(*((*self_0).qmf_out_dir).as_mut_ptr().offset(ch as isize))
                        .as_mut_ptr()
                        .offset(ts as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .re,
                    resolution,
                );
                ixheaacd_mps_synth_calc_fft(
                    fin_re.as_mut_ptr(),
                    fin_im.as_mut_ptr(),
                    m_resolution,
                );
                ixheaacd_mps_synth_post_twidle(
                    state,
                    fin_re.as_mut_ptr(),
                    fin_im.as_mut_ptr(),
                    resolution,
                );
                (Some(
                    ixheaacd_mps_synt_out_calc_pointer
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(resolution, out, state, filt_coeff);
                k = 0 as core::ffi::c_int as WORD32;
                while k < resolution {
                    let mut acc_0: FLOAT32 = *out.offset(k as isize);
                    l = 1 as core::ffi::c_int as WORD32;
                    while l < 10 as core::ffi::c_int {
                        acc_0 += *out.offset((resolution * l + k) as isize);
                        l += 1;
                    }
                    (*((*self_0).output_buffer)
                        .offset(
                            ch as isize,
                        ))[((*self_0).qmf_band_count * ts + k) as usize] = acc_0;
                    k += 1;
                }
                ts += 1;
            }
            memcpy(
                tmp_state as *mut core::ffi::c_void,
                state as *const core::ffi::c_void,
                (::core::mem::size_of::<FLOAT32>() as size_t)
                    .wrapping_mul(18 as size_t)
                    .wrapping_mul(resolution as size_t),
            );
            ch += 1;
        }
    };
}
