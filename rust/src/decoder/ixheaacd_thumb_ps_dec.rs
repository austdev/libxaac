extern "C" {
    fn ixheaacd_scale_short_vec_left(
        word16_arr: *mut WORD16,
        n: WORD32,
        shift: WORD16,
    ) -> VOID;
    fn ixheaacd_scale_int_vec_left(
        word32_arr: *mut WORD32,
        n: WORD32,
        shift: WORD16,
    ) -> VOID;
    fn ixheaacd_scale_int_vec_right(
        word32_arr: *mut WORD32,
        n: WORD32,
        shift: WORD16,
    ) -> VOID;
    fn ixheaacd_scale_short_vec_right(
        word16_arr: *mut WORD16,
        n: WORD32,
        shift: WORD16,
    ) -> VOID;
    fn ixheaacd_hybrid_analysis(
        ptr_qmf_real: *const WORD32,
        ptr_hyb_real: *mut WORD32,
        ptr_hyb_imag: *mut WORD32,
        ptr_hybrid: *mut ia_hybrid_struct,
        scale: WORD16,
        sbr_tables_ptr: *mut ia_sbr_tables_struct,
    ) -> VOID;
    static mut ixheaacd_decorrelation: Option<
        unsafe extern "C" fn(
            *mut ia_ps_dec_struct,
            *mut WORD32,
            *mut WORD32,
            *mut WORD32,
            *mut WORD32,
            *mut ia_ps_tables_struct,
        ) -> VOID,
    >;
    static mut ixheaacd_apply_rot: Option<
        unsafe extern "C" fn(
            *mut ia_ps_dec_struct,
            *mut WORD32,
            *mut WORD32,
            *mut WORD32,
            *mut WORD32,
            *mut ia_sbr_tables_struct,
            *const WORD16,
        ) -> VOID,
    >;
}
pub type WORD8 = core::ffi::c_schar;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD16 = core::ffi::c_short;
pub type WORD32 = core::ffi::c_int;
pub type FLOAT32 = core::ffi::c_float;
pub type VOID = ();
pub type FLAG = core::ffi::c_int;
pub type WORD = core::ffi::c_int;
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
pub struct ia_sbr_tables_struct {
    pub env_calc_tables_ptr: *mut ia_env_calc_tables_struct,
    pub qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
    pub env_extr_tables_ptr: *mut ia_env_extr_tables_struct,
    pub ps_tables_ptr: *mut ia_ps_tables_struct,
    pub sbr_rand_ph: *mut WORD32,
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
pub struct ia_env_calc_tables_struct {
    pub sbr_lim_gains_m: [WORD16; 8],
    pub sbr_lim_bands_per_octave_q13: [WORD16; 4],
    pub sbr_smooth_filter: [WORD16; 4],
    pub sbr_inv_int_table: [WORD16; 49],
    pub sbr_rand_ph: [WORD32; 568],
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
pub struct ia_ps_dec_struct {
    pub delay_buf_qmf_ap_re_im: *mut [WORD16; 64],
    pub delay_buf_qmf_ld_re_im: *mut [WORD16; 24],
    pub delay_buf_qmf_sd_re_im: *mut [WORD16; 64],
    pub delay_buf_idx_ser: [WORD16; 3],
    pub delay_sample_ser: [WORD16; 3],
    pub delay_buf_qmf_ser_re_im: REVERB_BUFFERS_RI,
    pub delay_buf_idx: WORD16,
    pub delay_buf_idx_long: WORD16,
    pub peak_decay_diff: *mut WORD32,
    pub energy_prev: *mut WORD32,
    pub peak_decay_diff_prev: *mut WORD32,
    pub ptr_hyb_left_re: *mut WORD32,
    pub ptr_hyb_left_im: *mut WORD32,
    pub ptr_hyb_right_re: *mut WORD32,
    pub ptr_hyb_right_im: *mut WORD32,
    pub delay_buf_qmf_sub_re_im: [[WORD16; 32]; 2],
    pub delay_buf_qmf_sub_ser_re_im: REVERB_BUFFERS_CH_RI,
    pub h11_h12_vec: [WORD16; 48],
    pub h21_h22_vec: [WORD16; 48],
    pub H11_H12: [WORD16; 48],
    pub H21_H22: [WORD16; 48],
    pub delta_h11_h12: [WORD16; 48],
    pub delta_h21_h22: [WORD16; 48],
    pub force_mono: FLAG,
    pub delay_buffer_scale: WORD16,
    pub usb: WORD16,
    pub iid_par_prev: [WORD16; 34],
    pub icc_par_prev: [WORD16; 34],
    pub ps_data_present: FLAG,
    pub enable_iid: FLAG,
    pub enable_icc: FLAG,
    pub enable_ext: FLAG,
    pub iid_mode: WORD16,
    pub icc_mode: WORD16,
    pub iid_quant: FLAG,
    pub frame_class: FLAG,
    pub num_env: WORD16,
    pub border_position: [WORD16; 7],
    pub iid_dt: [FLAG; 5],
    pub icc_dt: [FLAG; 5],
    pub iid_par_table: [[WORD16; 34]; 7],
    pub icc_par_table: [[WORD16; 34]; 7],
    pub str_hybrid: ia_hybrid_struct,
    pub hyb_left_re: [[FLOAT32; 32]; 32],
    pub hyb_left_im: [[FLOAT32; 32]; 32],
    pub hyb_right_re: [[FLOAT32; 32]; 32],
    pub hyb_right_im: [[FLOAT32; 32]; 32],
    pub h11_re_vec: [FLOAT32; 34],
    pub h11_im_vec: [FLOAT32; 34],
    pub h12_re_vec: [FLOAT32; 34],
    pub h12_im_vec: [FLOAT32; 34],
    pub h21_re_vec: [FLOAT32; 34],
    pub h21_im_vec: [FLOAT32; 34],
    pub h22_re_vec: [FLOAT32; 34],
    pub h22_im_vec: [FLOAT32; 34],
    pub h11_re_prev: [FLOAT32; 34],
    pub h11_im_prev: [FLOAT32; 34],
    pub h12_re_prev: [FLOAT32; 34],
    pub h12_im_prev: [FLOAT32; 34],
    pub h21_re_prev: [FLOAT32; 34],
    pub h21_im_prev: [FLOAT32; 34],
    pub h22_re_prev: [FLOAT32; 34],
    pub h22_im_prev: [FLOAT32; 34],
    pub qmf_delay_buf_re: [[FLOAT32; 64]; 14],
    pub qmf_delay_buf_im: [[FLOAT32; 64]; 14],
    pub sub_qmf_delay_buf_re: [[FLOAT32; 64]; 14],
    pub sub_qmf_delay_buf_im: [[FLOAT32; 64]; 14],
    pub ser_qmf_delay_buf_re: [[[FLOAT32; 64]; 5]; 3],
    pub ser_qmf_delay_buf_im: [[[FLOAT32; 64]; 5]; 3],
    pub ptr_hybrid: *mut ia_hybrid_flt_struct,
    pub str_flt_hybrid20: ia_hybrid_flt_struct,
    pub str_flt_hybrid34: ia_hybrid_flt_struct,
    pub use_34_st_bands: WORD32,
    pub use_34_st_bands_prev: WORD32,
    pub ps_mode: WORD32,
    pub ptr_group_borders: *mut WORD32,
    pub num_groups: WORD32,
    pub num_sub_qmf_groups: WORD32,
    pub num_bins: WORD32,
    pub first_delay_gr: WORD32,
    pub ptr_bins_group_map: *mut WORD32,
    pub num_sub_samples: WORD32,
    pub num_chans: WORD32,
    pub use_pca_rot_flg: WORD32,
    pub freq_res_ipd: WORD32,
    pub delay_qmf_delay_buf_idx: [WORD32; 64],
    pub delay_qmf_delay_num_samp: [WORD32; 64],
    pub peak_decay_fast_bin: [FLOAT32; 34],
    pub prev_nrg_bin: [FLOAT32; 34],
    pub prev_peak_diff_bin: [FLOAT32; 34],
    pub ipd_idx_map_1: [WORD32; 17],
    pub opd_idx_map_1: [WORD32; 17],
    pub ipd_idx_map_2: [WORD32; 17],
    pub opd_idx_map_2: [WORD32; 17],
    pub ipd_idx_map: [[WORD32; 17]; 5],
    pub opd_idx_map: [[WORD32; 17]; 5],
    pub ser_sub_qmf_dealy_buf_re: [[[FLOAT32; 64]; 5]; 3],
    pub ser_sub_qmf_dealy_buf_im: [[[FLOAT32; 64]; 5]; 3],
    pub hyb_work_re_20: [FLOAT32; 44],
    pub hyb_work_im_20: [FLOAT32; 44],
    pub hyb_qmf_buf_re_20: [[FLOAT32; 12]; 5],
    pub hyb_qmf_buf_im_20: [[FLOAT32; 12]; 5],
    pub hyb_temp_re_20: [[FLOAT32; 64]; 32],
    pub hyb_temp_im_20: [[FLOAT32; 64]; 32],
    pub hyb_work_re_34: [FLOAT32; 44],
    pub hyb_work_im_34: [FLOAT32; 44],
    pub hyb_qmf_buf_re_34: [[FLOAT32; 12]; 5],
    pub hyb_qmf_buf_im_34: [[FLOAT32; 12]; 5],
    pub hyb_temp_re_34: [[FLOAT32; 64]; 32],
    pub hyb_temp_im_34: [[FLOAT32; 64]; 32],
    pub pp_qmf_buf_real: [*mut *mut FLOAT32; 2],
    pub pp_qmf_buf_imag: [*mut *mut FLOAT32; 2],
    pub time_sample_buf: [*mut FLOAT32; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_hybrid_flt_struct {
    pub num_qmf_bands: WORD32,
    pub frame_size: WORD32,
    pub ptr_resol: *mut WORD16,
    pub ptr_work_re: *mut FLOAT32,
    pub ptr_work_im: *mut FLOAT32,
    pub ptr_qmf_buf_re: *mut [FLOAT32; 12],
    pub ptr_qmf_buf_im: *mut [FLOAT32; 12],
    pub ptr_temp_re: *mut [FLOAT32; 64],
    pub ptr_temp_im: *mut [FLOAT32; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_hybrid_struct {
    pub ptr_resol: *const WORD16,
    pub ptr_qmf_buf: WORD8,
    pub ptr_work_re: *mut WORD32,
    pub ptr_work_im: *mut WORD32,
    pub ptr_qmf_buf_re: [*mut WORD32; 3],
    pub ptr_qmf_buf_im: [*mut WORD32; 3],
    pub ptr_temp_re: *mut WORD32,
    pub ptr_temp_im: *mut WORD32,
}
pub type REVERB_BUFFERS_CH_RI = [[[WORD16; 32]; 3]; 5];
pub type REVERB_BUFFERS_RI = *mut [[WORD16; 64]; 3];
pub const MAX_OV_COLS: core::ffi::c_int = 6 as core::ffi::c_int;
pub const NUM_SER_AP_LINKS: core::ffi::c_int = 3 as core::ffi::c_int;
pub const HYBRID_FILTER_DELAY: core::ffi::c_int = 6 as core::ffi::c_int;
pub const NO_QMF_CHANNELS_IN_HYBRID: core::ffi::c_int = 3 as core::ffi::c_int;
pub const NUM_OF_QUAD_MIRROR_FILTER_CHNLS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const NUM_OF_ALL_PASS_CHNLS: core::ffi::c_int = 23 as core::ffi::c_int;
pub const NUM_OF_DEL_CHNLS: core::ffi::c_int = NUM_OF_QUAD_MIRROR_FILTER_CHNLS
    - NUM_OF_ALL_PASS_CHNLS;
pub const DEL_ALL_PASS: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SMALL_DEL_STRT: core::ffi::c_int = 12 as core::ffi::c_int;
pub const SMALL_DEL: core::ffi::c_int = 1 as core::ffi::c_int;
pub const HIGH_DEL: core::ffi::c_int = 14 as core::ffi::c_int;
pub const NUM_OF_QUAD_MIRROR_FILTER_ALL_PASS_CHNLS: core::ffi::c_int = NUM_OF_ALL_PASS_CHNLS
    - NO_QMF_CHANNELS_IN_HYBRID;
pub const NUM_OF_QUAD_MIRROR_FILTER_ICC_CHNLS: core::ffi::c_int = NUM_OF_QUAD_MIRROR_FILTER_ALL_PASS_CHNLS
    + NUM_OF_DEL_CHNLS;
pub const NUM_OF_BINS: core::ffi::c_int = 20 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_apply_ps(
    mut ptr_ps_dec: *mut ia_ps_dec_struct,
    mut p_buf_left_real: *mut *mut WORD32,
    mut p_buf_left_imag: *mut *mut WORD32,
    mut p_buf_right_real: *mut WORD32,
    mut p_buf_right_imag: *mut WORD32,
    mut sbr_scale_factor: *mut ia_sbr_scale_fact_struct,
    mut slot: WORD16,
    mut sbr_tables_ptr: *mut ia_sbr_tables_struct,
    mut no_col: WORD,
) -> VOID {
    let mut shiftdelay: WORD16 = 0;
    if no_col != 30 as core::ffi::c_int {
        shiftdelay = (if (slot as core::ffi::c_int)
            < 32 as core::ffi::c_int - MAX_OV_COLS
        {
            0 as core::ffi::c_int
        } else {
            (*sbr_scale_factor).lb_scale as core::ffi::c_int
                - (*sbr_scale_factor).ps_scale as core::ffi::c_int
        }) as WORD16;
    } else {
        shiftdelay = (if (slot as core::ffi::c_int)
            < no_col as core::ffi::c_int - MAX_OV_COLS
        {
            0 as core::ffi::c_int
        } else {
            (*sbr_scale_factor).lb_scale as core::ffi::c_int
                - (*sbr_scale_factor).ps_scale as core::ffi::c_int
        }) as WORD16;
    }
    ixheaacd_hybrid_analysis(
        *p_buf_left_real.offset(HYBRID_FILTER_DELAY as isize),
        (*ptr_ps_dec).ptr_hyb_left_re,
        (*ptr_ps_dec).ptr_hyb_left_im,
        &mut (*ptr_ps_dec).str_hybrid,
        shiftdelay,
        sbr_tables_ptr,
    );
    (Some(ixheaacd_decorrelation.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        ptr_ps_dec,
        *p_buf_left_real.offset(0 as core::ffi::c_int as isize),
        *p_buf_left_imag.offset(0 as core::ffi::c_int as isize),
        p_buf_right_real,
        p_buf_right_imag,
        (*sbr_tables_ptr).ps_tables_ptr,
    );
    (Some(ixheaacd_apply_rot.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        ptr_ps_dec,
        *p_buf_left_real.offset(0 as core::ffi::c_int as isize),
        *p_buf_left_imag.offset(0 as core::ffi::c_int as isize),
        p_buf_right_real,
        p_buf_right_imag,
        sbr_tables_ptr,
        (*ptr_ps_dec).str_hybrid.ptr_resol,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_scale_ps_states(
    mut ptr_ps_dec: *mut ia_ps_dec_struct,
    mut scale: WORD16,
) -> VOID {
    let mut i: WORD = 0;
    let mut m: WORD = 0;
    let mut delay: WORD32 = 0;
    if scale as core::ffi::c_int > 0 as core::ffi::c_int {
        let mut scale1: WORD16 = scale;
        if scale as core::ffi::c_int > 15 as core::ffi::c_int {
            scale1 = 15 as WORD16;
        }
        m = 0 as core::ffi::c_int as WORD;
        while m < 2 as core::ffi::c_int {
            ixheaacd_scale_short_vec_left(
                &mut *(*((*ptr_ps_dec).delay_buf_qmf_ap_re_im).offset(m as isize))
                    .as_mut_ptr()
                    .offset((3 as core::ffi::c_int * 2 as core::ffi::c_int) as isize)
                    as *mut WORD16,
                2 as WORD32 * NUM_OF_QUAD_MIRROR_FILTER_ALL_PASS_CHNLS,
                scale1,
            );
            m += 1;
        }
        delay = (2 as core::ffi::c_int * HIGH_DEL * SMALL_DEL_STRT
            + 2 as core::ffi::c_int * SMALL_DEL
                * (NUM_OF_QUAD_MIRROR_FILTER_ICC_CHNLS
                    - (NUM_OF_QUAD_MIRROR_FILTER_ALL_PASS_CHNLS + SMALL_DEL_STRT)))
            as WORD32;
        ixheaacd_scale_short_vec_left(
            (*ptr_ps_dec).delay_buf_qmf_ld_re_im as *mut WORD16,
            delay,
            scale1,
        );
        delay = (2 as core::ffi::c_int * 16 as core::ffi::c_int * DEL_ALL_PASS
            + 2 as core::ffi::c_int * NUM_SER_AP_LINKS * 5 as core::ffi::c_int
                * 16 as core::ffi::c_int) as WORD32;
        ixheaacd_scale_short_vec_left(
            ((*ptr_ps_dec).delay_buf_qmf_sub_re_im).as_mut_ptr() as *mut WORD16,
            delay,
            scale1,
        );
        i = 0 as core::ffi::c_int as WORD;
        while i < NUM_SER_AP_LINKS {
            m = 0 as core::ffi::c_int as WORD;
            while m < (*ptr_ps_dec).delay_sample_ser[i as usize] as core::ffi::c_int {
                ixheaacd_scale_short_vec_left(
                    &mut *(*(*((*ptr_ps_dec).delay_buf_qmf_ser_re_im).offset(m as isize))
                        .as_mut_ptr()
                        .offset(i as isize))
                        .as_mut_ptr()
                        .offset((2 as core::ffi::c_int * 3 as core::ffi::c_int) as isize)
                        as *mut WORD16,
                    2 as WORD32 * NUM_OF_QUAD_MIRROR_FILTER_ALL_PASS_CHNLS,
                    scale1,
                );
                m += 1;
            }
            i += 1;
        }
        ixheaacd_scale_int_vec_left(
            (*ptr_ps_dec).str_hybrid.ptr_qmf_buf_re[0 as core::ffi::c_int as usize],
            2 as WORD32 * NO_QMF_CHANNELS_IN_HYBRID
                * (*ptr_ps_dec).str_hybrid.ptr_qmf_buf as WORD32,
            scale,
        );
        scale = (scale as core::ffi::c_int + scale as core::ffi::c_int) as WORD16;
        ixheaacd_scale_int_vec_left(
            (*ptr_ps_dec).peak_decay_diff,
            3 as WORD32 * NUM_OF_BINS,
            scale,
        );
    } else if scale as core::ffi::c_int != 0 as core::ffi::c_int {
        scale = -(scale as core::ffi::c_int) as WORD16;
        m = 0 as core::ffi::c_int as WORD;
        while m < 2 as core::ffi::c_int {
            ixheaacd_scale_short_vec_right(
                &mut *(*((*ptr_ps_dec).delay_buf_qmf_ap_re_im).offset(m as isize))
                    .as_mut_ptr()
                    .offset((3 as core::ffi::c_int * 2 as core::ffi::c_int) as isize)
                    as *mut WORD16,
                2 as WORD32 * NUM_OF_QUAD_MIRROR_FILTER_ALL_PASS_CHNLS,
                scale,
            );
            m += 1;
        }
        delay = (2 as core::ffi::c_int * HIGH_DEL * SMALL_DEL_STRT
            + 2 as core::ffi::c_int * SMALL_DEL
                * (NUM_OF_QUAD_MIRROR_FILTER_ICC_CHNLS
                    - (NUM_OF_QUAD_MIRROR_FILTER_ALL_PASS_CHNLS + SMALL_DEL_STRT)))
            as WORD32;
        ixheaacd_scale_short_vec_right(
            (*ptr_ps_dec).delay_buf_qmf_ld_re_im as *mut WORD16,
            delay,
            scale,
        );
        delay = (2 as core::ffi::c_int * 16 as core::ffi::c_int * DEL_ALL_PASS
            + 2 as core::ffi::c_int * NUM_SER_AP_LINKS * 5 as core::ffi::c_int
                * 16 as core::ffi::c_int) as WORD32;
        ixheaacd_scale_short_vec_right(
            ((*ptr_ps_dec).delay_buf_qmf_sub_re_im).as_mut_ptr() as *mut WORD16,
            delay,
            scale,
        );
        i = 0 as core::ffi::c_int as WORD;
        while i < NUM_SER_AP_LINKS {
            m = 0 as core::ffi::c_int as WORD;
            while m < (*ptr_ps_dec).delay_sample_ser[i as usize] as core::ffi::c_int {
                ixheaacd_scale_short_vec_right(
                    &mut *(*(*((*ptr_ps_dec).delay_buf_qmf_ser_re_im).offset(m as isize))
                        .as_mut_ptr()
                        .offset(i as isize))
                        .as_mut_ptr()
                        .offset((3 as core::ffi::c_int * 2 as core::ffi::c_int) as isize)
                        as *mut WORD16,
                    2 as WORD32 * NUM_OF_QUAD_MIRROR_FILTER_ALL_PASS_CHNLS,
                    scale,
                );
                m += 1;
            }
            i += 1;
        }
        ixheaacd_scale_int_vec_right(
            (*ptr_ps_dec).str_hybrid.ptr_qmf_buf_re[0 as core::ffi::c_int as usize],
            2 as WORD32 * NO_QMF_CHANNELS_IN_HYBRID
                * (*ptr_ps_dec).str_hybrid.ptr_qmf_buf as WORD32,
            scale,
        );
        scale = (scale as core::ffi::c_int + scale as core::ffi::c_int) as WORD16;
        ixheaacd_scale_int_vec_right(
            (*ptr_ps_dec).peak_decay_diff,
            3 as WORD32 * NUM_OF_BINS,
            scale,
        );
    }
}
