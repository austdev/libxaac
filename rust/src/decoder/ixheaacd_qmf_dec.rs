extern "C" {
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn ixheaacd_init_rot_env(
        ptr_ps_dec: *mut ia_ps_dec_struct,
        env: WORD16,
        usb: WORD16,
        sbr_tables_ptr: *mut ia_sbr_tables_struct,
        cos_sin_lookup_tab: *const WORD16,
    ) -> VOID;
    fn ixheaacd_apply_ps(
        ptr_ps_dec: *mut ia_ps_dec_struct,
        real_buf_left: *mut *mut WORD32,
        imag_buf_left: *mut *mut WORD32,
        real_buf_right: *mut WORD32,
        imag_buf_right: *mut WORD32,
        sbr_scale_factor: *mut ia_sbr_scale_fact_struct,
        slot: WORD16,
        sbr_tables_ptr: *mut ia_sbr_tables_struct,
        no_col: WORD,
    ) -> VOID;
    fn ixheaacd_shiftrountine(
        qmf_real: *mut WORD32,
        qmf_imag: *mut WORD32,
        len: WORD32,
        common_shift: WORD32,
    ) -> VOID;
    fn ixheaacd_shiftrountine_with_rnd(
        qmf_real: *mut WORD32,
        qmf_imag: *mut WORD32,
        filter_states: *mut WORD16,
        len: WORD32,
        shift: WORD32,
    ) -> VOID;
    fn ixheaacd_radix4bfly(
        w: *const WORD16,
        x: *mut WORD32,
        npoints: WORD32,
        ch_fac: WORD32,
    ) -> VOID;
    fn ixheaacd_postradixcompute4(
        ptr_y: *mut WORD32,
        ptr_x: *mut WORD32,
        p_dig_rev_tbl: *const WORD32,
        npoints: WORD32,
    ) -> VOID;
    fn ixheaacd_shiftrountine_with_rnd_eld(
        qmf_real: *mut WORD32,
        qmf_imag: *mut WORD32,
        filter_states: *mut WORD16,
        len: WORD32,
        shift: WORD32,
    ) -> VOID;
    fn ixheaacd_sbr_qmfsyn64_winadd(
        tmp1: *mut WORD16,
        tmp2: *mut WORD16,
        inp1: *mut WORD16,
        sample_buffer: *mut WORD16,
        shift: FLAG,
        ch_fac: WORD32,
    ) -> VOID;
    fn ixheaacd_esbr_cos_sin_mod(
        subband: *mut WORD32,
        qmf_bank: *mut ia_sbr_qmf_filter_bank_struct,
        p_twiddle: *mut WORD32,
        p_dig_rev_tbl: *mut WORD32,
    ) -> VOID;
    fn ixheaacd_inv_modulation_lp(
        qmf_real: *mut WORD32,
        filter_states: *mut WORD16,
        syn_qmf: *mut ia_sbr_qmf_filter_bank_struct,
        qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
    ) -> VOID;
    fn ixheaacd_inv_emodulation(
        qmf_real: *mut WORD32,
        syn_qmf: *mut ia_sbr_qmf_filter_bank_struct,
        qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
    ) -> VOID;
    static mut ixheaacd_adjust_scale: Option<
        unsafe extern "C" fn(
            *mut *mut WORD32,
            *mut *mut WORD32,
            WORD32,
            WORD32,
            WORD32,
            WORD32,
            WORD32,
            FLAG,
        ) -> VOID,
    >;
}
pub type size_t = usize;
pub type WORD8 = core::ffi::c_schar;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD16 = core::ffi::c_short;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type WORD64 = core::ffi::c_longlong;
pub type FLOAT32 = core::ffi::c_float;
pub type VOID = ();
pub type FLAG = core::ffi::c_int;
pub type WORD = core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ixheaacd_misc_tables {
    pub trig_data: [WORD16; 513],
    pub sine_table8_16: [WORD16; 8],
    pub log_dual_is_table: [WORD16; 65],
    pub down_mix_martix: [[[WORD32; 8]; 2]; 4],
    pub cc_gain_scale: [WORD32; 4],
    pub inv_table: [WORD16; 256],
    pub sqrt_table: [WORD16; 257],
    pub dummy: WORD32,
    pub start_band: [[WORD32; 16]; 10],
    pub stop_band: [[WORD32; 16]; 10],
    pub stop_freq_table_fs40k_2: [WORD32; 14],
    pub stop_freq_table_fs40k_4: [WORD32; 14],
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
pub type REVERB_BUFFERS_RI = *mut [[WORD16; 64]; 3];
pub type REVERB_BUFFERS_CH_RI = [[[WORD16; 32]; 3]; 5];
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
pub const AOT_ER_AAC_ELD: AUDIO_OBJECT_TYPE = 39;
pub const AOT_ER_AAC_LD: AUDIO_OBJECT_TYPE = 23;
pub type AUDIO_OBJECT_TYPE = core::ffi::c_uint;
pub const AOT_USAC: AUDIO_OBJECT_TYPE = 42;
pub const AOT_ESC: AUDIO_OBJECT_TYPE = 31;
pub const AOT_RSVD_31: AUDIO_OBJECT_TYPE = 31;
pub const AOT_RSVD_30: AUDIO_OBJECT_TYPE = 30;
pub const AOT_PS: AUDIO_OBJECT_TYPE = 29;
pub const AOT_RSVD_28: AUDIO_OBJECT_TYPE = 28;
pub const AOT_ER_PARA: AUDIO_OBJECT_TYPE = 27;
pub const AOT_ER_HILN: AUDIO_OBJECT_TYPE = 26;
pub const AOT_ER_HVXC: AUDIO_OBJECT_TYPE = 25;
pub const AOT_ER_CELP: AUDIO_OBJECT_TYPE = 24;
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
pub const MAX_32: WORD32 = 0x7fffffff as core::ffi::c_long as WORD32;
pub const MIN_32: WORD32 = 0x80000000 as core::ffi::c_long as WORD32;
pub const MAX_16: WORD16 = 0x7fff as core::ffi::c_int as WORD16;
#[inline]
unsafe extern "C" fn ixheaac_min32(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut min_val: WORD32 = 0;
    min_val = if a < b { a } else { b };
    return min_val;
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
unsafe extern "C" fn ixheaac_shl32_sat(mut a: WORD32, mut b: WORD) -> WORD32 {
    let mut out_val: WORD32 = 0;
    if a > MAX_32 >> b {
        out_val = MAX_32;
    } else if a < MIN_32 >> b {
        out_val = MIN_32;
    } else {
        out_val = a << b;
    }
    return out_val;
}
#[inline]
unsafe extern "C" fn ixheaac_mult16x16in32(mut a: WORD16, mut b: WORD16) -> WORD32 {
    let mut product: WORD32 = 0;
    product = a as WORD32 * b as WORD32;
    return product;
}
#[inline]
unsafe extern "C" fn ixheaac_add32(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut sum: WORD32 = 0;
    sum = a + b;
    return sum;
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
unsafe extern "C" fn ixheaac_negate16(mut op1: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    if -(32768 as core::ffi::c_int) == op1 as core::ffi::c_int {
        var_out = MAX_16;
    } else {
        var_out = -(op1 as core::ffi::c_int) as WORD16;
    }
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_round16(mut op1: WORD32) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = (ixheaac_add32_sat(op1, 0x8000 as WORD32) >> 16 as core::ffi::c_int)
        as WORD16;
    return var_out;
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
unsafe extern "C" fn ixheaac_mult32x16in32_shl_sat(
    mut a: WORD32,
    mut b: WORD16,
) -> WORD32 {
    let mut result: WORD32 = 0;
    if a == 0x80000000 as core::ffi::c_uint as WORD32
        && b as core::ffi::c_int
            == 0x8000 as core::ffi::c_int as WORD16 as core::ffi::c_int
    {
        result = 0x7fffffff as core::ffi::c_int;
    } else {
        result = ixheaac_mult32x16in32_shl(a, b);
    }
    return result;
}
#[inline]
unsafe extern "C" fn ixheaac_mac32x16in32_shl_sat(
    mut a: WORD32,
    mut b: WORD32,
    mut c: WORD16,
) -> WORD32 {
    let mut result: WORD32 = 0;
    result = ixheaac_add32_sat(a, ixheaac_mult32x16in32_shl_sat(b, c));
    return result;
}
#[inline]
unsafe extern "C" fn ixheaac_mult64(mut a: WORD32, mut b: WORD32) -> WORD64 {
    let mut result: WORD64 = 0;
    result = a as WORD64 * b as WORD64;
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
#[inline]
unsafe extern "C" fn ixheaac_add64(mut a: WORD64, mut b: WORD64) -> WORD64 {
    let mut result: WORD64 = 0;
    result = a + b;
    return result;
}
#[inline]
unsafe extern "C" fn ixheaac_mul32_sh(
    mut a: WORD32,
    mut b: WORD32,
    mut shift: WORD8,
) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> shift as core::ffi::c_int) as WORD32;
    return result;
}
pub const NO_SYNTHESIS_CHANNELS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const NO_SYNTHESIS_CHANNELS_DOWN_SAMPLED: core::ffi::c_int = NO_SYNTHESIS_CHANNELS
    / 2 as core::ffi::c_int;
pub const DCT2_LEN: core::ffi::c_int = 64 as core::ffi::c_int;
#[inline]
unsafe extern "C" fn ixheaacd_mult32x32in32_shift25(
    mut a: WORD32,
    mut b: WORD32,
) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 25 as core::ffi::c_int) as WORD32;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_pretwdct2(
    mut inp: *mut WORD32,
    mut out_fwd: *mut WORD32,
) -> VOID {
    let mut n: WORD32 = 0;
    let mut out_rev: *mut WORD32 = out_fwd
        .offset(DCT2_LEN as isize)
        .offset(-(1 as core::ffi::c_int as isize));
    n = 0 as core::ffi::c_int as WORD32;
    while n < DCT2_LEN / 2 as core::ffi::c_int {
        *out_fwd = *inp;
        inp = inp.offset(1);
        *out_rev = *inp;
        out_fwd = out_fwd.offset(1);
        out_rev = out_rev.offset(-1);
        inp = inp.offset(1);
        n += 1;
    }
}
#[inline]
unsafe extern "C" fn ixheaacd_pretwdct2_32(
    mut inp: *mut WORD32,
    mut out_fwd: *mut WORD32,
    mut dct2_len: core::ffi::c_int,
) -> VOID {
    let mut n: WORD32 = 0;
    let mut out_rev: *mut WORD32 = out_fwd
        .offset(dct2_len as isize)
        .offset(-(1 as core::ffi::c_int as isize));
    n = (dct2_len / 2 as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while n >= 0 as core::ffi::c_int {
        *out_fwd = *inp;
        inp = inp.offset(1);
        *out_rev = *inp;
        out_fwd = out_fwd.offset(1);
        out_rev = out_rev.offset(-1);
        inp = inp.offset(1);
        n -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_fftposttw(
    mut out: *mut WORD32,
    mut qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
) -> VOID {
    let mut k: core::ffi::c_int = 0;
    let mut p_out_fwd: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr_out_rev: *mut WORD32 = 0 as *mut WORD32;
    let mut twidle_fwd: *const WORD16 = 0 as *const WORD16;
    let mut twidle_rev: *const WORD16 = 0 as *const WORD16;
    let mut in1: WORD32 = 0;
    let mut in2: WORD32 = 0;
    let mut val1: WORD32 = 0;
    let mut val2: WORD32 = 0;
    twidle_fwd = ((*qmf_dec_tables_ptr).post_fft_tbl)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as isize);
    twidle_rev = ((*qmf_dec_tables_ptr).post_fft_tbl)
        .as_mut_ptr()
        .offset(15 as core::ffi::c_int as isize);
    p_out_fwd = out;
    ptr_out_rev = out
        .offset(DCT2_LEN as isize)
        .offset(-(1 as core::ffi::c_int as isize));
    let fresh29 = p_out_fwd;
    p_out_fwd = p_out_fwd.offset(1);
    in1 = *fresh29 << 1 as core::ffi::c_int;
    let fresh30 = p_out_fwd;
    p_out_fwd = p_out_fwd.offset(-1);
    val1 = *fresh30 << 1 as core::ffi::c_int;
    let fresh31 = p_out_fwd;
    p_out_fwd = p_out_fwd.offset(1);
    *fresh31 = in1;
    let fresh32 = p_out_fwd;
    p_out_fwd = p_out_fwd.offset(1);
    *fresh32 = val1;
    k = 1 as core::ffi::c_int;
    while k <= DCT2_LEN / 4 as core::ffi::c_int {
        let mut temp: [WORD32; 4] = [0; 4];
        let mut twid_re: WORD16 = 0;
        let mut twid_im: WORD16 = 0;
        let fresh33 = p_out_fwd;
        p_out_fwd = p_out_fwd.offset(1);
        temp[0 as core::ffi::c_int as usize] = *fresh33;
        let fresh34 = p_out_fwd;
        p_out_fwd = p_out_fwd.offset(-1);
        temp[1 as core::ffi::c_int as usize] = *fresh34;
        let fresh35 = ptr_out_rev;
        ptr_out_rev = ptr_out_rev.offset(-1);
        temp[3 as core::ffi::c_int as usize] = *fresh35;
        let fresh36 = ptr_out_rev;
        ptr_out_rev = ptr_out_rev.offset(1);
        temp[2 as core::ffi::c_int as usize] = *fresh36;
        in2 = ixheaac_sub32_sat(
            temp[3 as core::ffi::c_int as usize],
            temp[1 as core::ffi::c_int as usize],
        );
        in1 = ixheaac_add32_sat(
            temp[3 as core::ffi::c_int as usize],
            temp[1 as core::ffi::c_int as usize],
        );
        temp[1 as core::ffi::c_int as usize] = ixheaac_sub32_sat(
            temp[0 as core::ffi::c_int as usize],
            temp[2 as core::ffi::c_int as usize],
        );
        temp[3 as core::ffi::c_int as usize] = ixheaac_add32_sat(
            temp[0 as core::ffi::c_int as usize],
            temp[2 as core::ffi::c_int as usize],
        );
        let fresh37 = twidle_fwd;
        twidle_fwd = twidle_fwd.offset(1);
        twid_re = *fresh37;
        let fresh38 = twidle_rev;
        twidle_rev = twidle_rev.offset(-1);
        twid_im = *fresh38;
        val1 = ixheaac_mult32x16in32(in1, twid_re)
            - ixheaac_mult32x16in32(temp[1 as core::ffi::c_int as usize], twid_im);
        val2 = ixheaac_mult32x16in32(temp[1 as core::ffi::c_int as usize], twid_re)
            + ixheaac_mult32x16in32(in1, twid_im);
        val1 = val1 << 1 as core::ffi::c_int;
        val2 = val2 << 1 as core::ffi::c_int;
        let fresh39 = p_out_fwd;
        p_out_fwd = p_out_fwd.offset(1);
        *fresh39 = ixheaac_add32_sat(temp[3 as core::ffi::c_int as usize], val1);
        let fresh40 = p_out_fwd;
        p_out_fwd = p_out_fwd.offset(1);
        *fresh40 = ixheaac_add32_sat(in2, val2);
        let fresh41 = ptr_out_rev;
        ptr_out_rev = ptr_out_rev.offset(-1);
        *fresh41 = ixheaac_sub32_sat(val2, in2);
        let fresh42 = ptr_out_rev;
        ptr_out_rev = ptr_out_rev.offset(-1);
        *fresh42 = ixheaac_sub32_sat(temp[3 as core::ffi::c_int as usize], val1);
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_posttwdct2(
    mut inp: *mut WORD32,
    mut out_fwd: *mut WORD16,
    mut qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
) -> VOID {
    let mut k: WORD32 = 0;
    let mut inp_re: WORD32 = 0;
    let mut inp_im: WORD32 = 0;
    let mut out_re: WORD32 = 0;
    let mut out_im: WORD32 = 0;
    let mut last_val: WORD32 = 0;
    let mut out_re1: WORD32 = 0;
    let mut out_fwd2: *mut WORD16 = 0 as *mut WORD16;
    let mut out_rev2: *mut WORD16 = 0 as *mut WORD16;
    let mut out_rev: *mut WORD16 = 0 as *mut WORD16;
    let mut twid_re: WORD16 = 0;
    let mut twid_im: WORD16 = 0;
    let mut twidle_fwd: *const WORD16 = 0 as *const WORD16;
    let mut re1: WORD16 = 0;
    let mut im1: WORD16 = 0;
    let mut im2: WORD16 = 0;
    out_rev = out_fwd
        .offset(DCT2_LEN as isize)
        .offset(-(1 as core::ffi::c_int as isize));
    out_rev2 = out_fwd.offset(-(1 as core::ffi::c_int as isize));
    out_fwd2 = out_fwd.offset(65 as core::ffi::c_int as isize);
    let fresh43 = inp;
    inp = inp.offset(1);
    out_re = *fresh43;
    let fresh44 = inp;
    inp = inp.offset(1);
    out_im = *fresh44;
    out_re1 = ixheaac_sat64_32(
        ixheaac_add64(out_re as WORD64, out_im as WORD64) >> 1 as core::ffi::c_int,
    );
    re1 = ixheaac_round16(ixheaac_shl32(out_re1, 5 as WORD - 1 as WORD));
    let fresh45 = out_fwd;
    out_fwd = out_fwd.offset(1);
    *fresh45 = re1;
    last_val = ixheaac_sub32_sat(out_re, out_im);
    twidle_fwd = ((*qmf_dec_tables_ptr).dct23_tw)
        .as_mut_ptr()
        .offset(2 as core::ffi::c_int as isize);
    k = (DCT2_LEN / 2 as core::ffi::c_int - 2 as core::ffi::c_int) as WORD32;
    while k >= 0 as core::ffi::c_int {
        let fresh46 = inp;
        inp = inp.offset(1);
        inp_re = *fresh46;
        let fresh47 = inp;
        inp = inp.offset(1);
        inp_im = *fresh47;
        let fresh48 = twidle_fwd;
        twidle_fwd = twidle_fwd.offset(1);
        twid_re = *fresh48;
        let fresh49 = twidle_fwd;
        twidle_fwd = twidle_fwd.offset(1);
        twid_im = *fresh49;
        out_re = ixheaac_sub32_sat(
            ixheaac_mult32x16in32(inp_re, twid_re),
            ixheaac_mult32x16in32(inp_im, twid_im),
        );
        out_im = ixheaac_add32_sat(
            ixheaac_mult32x16in32(inp_im, twid_re),
            ixheaac_mult32x16in32(inp_re, twid_im),
        );
        re1 = ixheaac_round16(ixheaac_shl32(out_re, 5 as WORD - 1 as WORD));
        im1 = ixheaac_round16(ixheaac_shl32(out_im, 5 as WORD - 1 as WORD));
        im2 = ixheaac_negate16(im1);
        let fresh50 = out_fwd;
        out_fwd = out_fwd.offset(1);
        *fresh50 = re1;
        let fresh51 = out_rev2;
        out_rev2 = out_rev2.offset(-1);
        *fresh51 = re1;
        let fresh52 = out_rev;
        out_rev = out_rev.offset(-1);
        *fresh52 = im1;
        let fresh53 = out_fwd2;
        out_fwd2 = out_fwd2.offset(1);
        *fresh53 = im2;
        k -= 1;
    }
    let fresh54 = twidle_fwd;
    twidle_fwd = twidle_fwd.offset(1);
    twid_re = *fresh54;
    out_re = ixheaac_mult32x16in32(last_val, twid_re);
    re1 = ixheaac_round16(ixheaac_shl32(out_re, 5 as WORD - 1 as WORD));
    let fresh55 = out_fwd;
    out_fwd = out_fwd.offset(1);
    *fresh55 = re1;
    let fresh56 = out_rev2;
    out_rev2 = out_rev2.offset(-1);
    *fresh56 = re1;
}
#[inline]
unsafe extern "C" fn ixheaacd_fftposttw_32(
    mut out: *mut WORD32,
    mut dct2_len: core::ffi::c_int,
    mut qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
) -> VOID {
    let mut k: core::ffi::c_int = 0;
    let mut ptr_out_fwd: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr_out_rev: *mut WORD32 = 0 as *mut WORD32;
    let mut twidle_fwd: *const WORD16 = 0 as *const WORD16;
    let mut twidle_rev: *const WORD16 = 0 as *const WORD16;
    let mut in1: WORD32 = 0;
    let mut in2: WORD32 = 0;
    let mut val1: WORD32 = 0;
    let mut val2: WORD32 = 0;
    twidle_fwd = ((*qmf_dec_tables_ptr).post_fft_tbl)
        .as_mut_ptr()
        .offset(2 as core::ffi::c_int as isize);
    twidle_rev = ((*qmf_dec_tables_ptr).post_fft_tbl)
        .as_mut_ptr()
        .offset(14 as core::ffi::c_int as isize);
    ptr_out_fwd = out;
    ptr_out_rev = out
        .offset(dct2_len as isize)
        .offset(-(1 as core::ffi::c_int as isize));
    let fresh17 = ptr_out_fwd;
    ptr_out_fwd = ptr_out_fwd.offset(1);
    in1 = *fresh17 << 1 as core::ffi::c_int;
    let fresh18 = ptr_out_fwd;
    ptr_out_fwd = ptr_out_fwd.offset(-1);
    val1 = *fresh18 << 1 as core::ffi::c_int;
    let fresh19 = ptr_out_fwd;
    ptr_out_fwd = ptr_out_fwd.offset(1);
    *fresh19 = in1;
    let fresh20 = ptr_out_fwd;
    ptr_out_fwd = ptr_out_fwd.offset(1);
    *fresh20 = val1;
    k = dct2_len / 4 as core::ffi::c_int - 1 as core::ffi::c_int;
    while k >= 0 as core::ffi::c_int {
        let mut temp0: WORD32 = 0;
        let mut temp1: WORD32 = 0;
        let mut temp2: WORD32 = 0;
        let mut temp3: WORD32 = 0;
        let mut twid_re: WORD16 = 0;
        let mut twid_im: WORD16 = 0;
        let fresh21 = ptr_out_fwd;
        ptr_out_fwd = ptr_out_fwd.offset(1);
        temp0 = *fresh21;
        let fresh22 = ptr_out_fwd;
        ptr_out_fwd = ptr_out_fwd.offset(-1);
        temp1 = *fresh22;
        let fresh23 = ptr_out_rev;
        ptr_out_rev = ptr_out_rev.offset(-1);
        temp3 = *fresh23;
        let fresh24 = ptr_out_rev;
        ptr_out_rev = ptr_out_rev.offset(1);
        temp2 = *fresh24;
        in1 = ixheaac_add32_sat(temp1, temp3);
        in2 = ixheaac_sub32_sat(temp3, temp1);
        temp1 = ixheaac_sub32_sat(temp0, temp2);
        temp3 = ixheaac_add32_sat(temp0, temp2);
        twid_re = *twidle_fwd;
        twidle_fwd = twidle_fwd.offset(2 as core::ffi::c_int as isize);
        twid_im = *twidle_rev;
        twidle_rev = twidle_rev.offset(-(2 as core::ffi::c_int as isize));
        val1 = ixheaac_mult32x16in32(in1, twid_re)
            - ixheaac_mult32x16in32(temp1, twid_im);
        val2 = ixheaac_mult32x16in32(temp1, twid_re)
            + ixheaac_mult32x16in32(in1, twid_im);
        val1 = val1 << 1 as core::ffi::c_int;
        val2 = val2 << 1 as core::ffi::c_int;
        let fresh25 = ptr_out_fwd;
        ptr_out_fwd = ptr_out_fwd.offset(1);
        *fresh25 = ixheaac_add32_sat(temp3, val1);
        let fresh26 = ptr_out_fwd;
        ptr_out_fwd = ptr_out_fwd.offset(1);
        *fresh26 = ixheaac_add32_sat(in2, val2);
        let fresh27 = ptr_out_rev;
        ptr_out_rev = ptr_out_rev.offset(-1);
        *fresh27 = ixheaac_sub32_sat(val2, in2);
        let fresh28 = ptr_out_rev;
        ptr_out_rev = ptr_out_rev.offset(-1);
        *fresh28 = ixheaac_sub32_sat(temp3, val1);
        k -= 1;
    }
}
#[inline]
unsafe extern "C" fn ixheaacd_posttwdct2_32(
    mut inp: *mut WORD32,
    mut out_fwd: *mut WORD16,
    mut qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
) -> VOID {
    let mut k: core::ffi::c_int = 0;
    let mut inp_re: WORD32 = 0;
    let mut out_re: WORD32 = 0;
    let mut out_im: WORD32 = 0;
    let mut last_val: WORD32 = 0;
    let mut out_re1: WORD32 = 0;
    let mut out_rev: *mut WORD16 = 0 as *mut WORD16;
    let mut out_rev2: *mut WORD16 = 0 as *mut WORD16;
    let mut out_fwd2: *mut WORD16 = 0 as *mut WORD16;
    let mut twid_re: WORD16 = 0;
    let mut twid_im: WORD16 = 0;
    let mut twidle_fwd: *const WORD16 = 0 as *const WORD16;
    let mut re1: WORD16 = 0;
    let mut im1: WORD16 = 0;
    let mut im2: WORD16 = 0;
    let mut rounding_fac: WORD32 = 0x8000 as WORD32;
    out_rev = out_fwd
        .offset(32 as core::ffi::c_int as isize)
        .offset(-(1 as core::ffi::c_int as isize));
    out_rev2 = out_fwd.offset(-(1 as core::ffi::c_int as isize));
    out_fwd2 = out_fwd
        .offset(32 as core::ffi::c_int as isize)
        .offset(1 as core::ffi::c_int as isize);
    *out_fwd.offset(32 as core::ffi::c_int as isize) = 0 as WORD16;
    let fresh4 = inp;
    inp = inp.offset(1);
    out_re = *fresh4;
    let fresh5 = inp;
    inp = inp.offset(1);
    out_im = *fresh5;
    out_re1 = ixheaac_sat64_32(
        ixheaac_add64(out_re as WORD64, out_im as WORD64) >> 1 as core::ffi::c_int,
    );
    re1 = ixheaac_round16(ixheaac_shl32_sat(out_re1, 5 as WORD - 1 as WORD));
    let fresh6 = out_fwd;
    out_fwd = out_fwd.offset(1);
    *fresh6 = re1;
    last_val = ixheaac_sub32_sat(out_re, out_im);
    twidle_fwd = ((*qmf_dec_tables_ptr).dct23_tw)
        .as_mut_ptr()
        .offset(4 as core::ffi::c_int as isize);
    k = 14 as core::ffi::c_int;
    while k >= 0 as core::ffi::c_int {
        let mut temp1: WORD32 = 0;
        let mut temp2: WORD32 = 0;
        let fresh7 = inp;
        inp = inp.offset(1);
        inp_re = *fresh7;
        let fresh8 = twidle_fwd;
        twidle_fwd = twidle_fwd.offset(1);
        twid_re = *fresh8;
        twid_im = *twidle_fwd;
        twidle_fwd = twidle_fwd.offset(3 as core::ffi::c_int as isize);
        temp1 = ixheaac_mult32x16in32(inp_re, twid_re);
        temp2 = ixheaac_mult32x16in32(inp_re, twid_im);
        let fresh9 = inp;
        inp = inp.offset(1);
        inp_re = *fresh9;
        out_re = ixheaac_sub32_sat(temp1, ixheaac_mult32x16in32(inp_re, twid_im));
        out_im = ixheaac_add32_sat(ixheaac_mult32x16in32(inp_re, twid_re), temp2);
        out_re = ixheaac_add32_sat(out_re, out_re);
        out_im = ixheaac_add32_sat(out_im, out_im);
        out_re = ixheaac_add32_sat(out_re, out_re);
        out_im = ixheaac_add32_sat(out_im, out_im);
        out_re = ixheaac_add32_sat(out_re, out_re);
        out_im = ixheaac_add32_sat(out_im, out_im);
        out_re = ixheaac_add32_sat(out_re, out_re);
        out_im = ixheaac_add32_sat(out_im, out_im);
        out_re = ixheaac_add32_sat(out_re, rounding_fac);
        out_im = ixheaac_add32_sat(out_im, rounding_fac);
        re1 = (out_re >> 16 as core::ffi::c_int) as WORD16;
        im1 = (out_im >> 16 as core::ffi::c_int) as WORD16;
        im2 = ixheaac_negate16(im1);
        let fresh10 = out_fwd;
        out_fwd = out_fwd.offset(1);
        *fresh10 = re1;
        let fresh11 = out_rev2;
        out_rev2 = out_rev2.offset(-1);
        *fresh11 = re1;
        let fresh12 = out_rev;
        out_rev = out_rev.offset(-1);
        *fresh12 = im1;
        let fresh13 = out_fwd2;
        out_fwd2 = out_fwd2.offset(1);
        *fresh13 = im2;
        k -= 1;
    }
    let fresh14 = twidle_fwd;
    twidle_fwd = twidle_fwd.offset(1);
    twid_re = *fresh14;
    out_re = ixheaac_mult32x16in32(last_val, twid_re);
    re1 = ixheaac_round16(ixheaac_shl32_sat(out_re, 5 as WORD - 1 as WORD));
    let fresh15 = out_fwd;
    out_fwd = out_fwd.offset(1);
    *fresh15 = re1;
    let fresh16 = out_rev2;
    out_rev2 = out_rev2.offset(-1);
    *fresh16 = re1;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_dct2_32(
    mut inp: *mut WORD32,
    mut out: *mut WORD32,
    mut qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
    mut filter_states: *mut WORD16,
) -> VOID {
    let mut output: *mut WORD32 = 0 as *mut WORD32;
    output = out.offset(16 as core::ffi::c_int as isize);
    filter_states = filter_states.offset(16 as core::ffi::c_int as isize);
    ixheaacd_pretwdct2_32(inp, output, 32 as core::ffi::c_int);
    ixheaacd_radix4bfly(
        ((*qmf_dec_tables_ptr).w_16).as_mut_ptr(),
        output,
        1 as WORD32,
        4 as WORD32,
    );
    ixheaacd_postradixcompute4(
        inp,
        output,
        ((*qmf_dec_tables_ptr).dig_rev_table4_16).as_mut_ptr(),
        16 as WORD32,
    );
    ixheaacd_fftposttw_32(inp, 32 as core::ffi::c_int, qmf_dec_tables_ptr);
    ixheaacd_posttwdct2_32(inp, filter_states, qmf_dec_tables_ptr);
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_sbr_qmfanal32_winadd_eld_mps(
    mut inp1: *mut WORD32,
    mut inp2: *mut WORD32,
    mut p_qmf1: *const WORD32,
    mut p_qmf2: *const WORD32,
    mut p_out: *mut WORD32,
) -> VOID {
    let mut n: WORD32 = 0;
    let mut resolution: WORD32 = 64 as WORD32;
    n = 0 as core::ffi::c_int as WORD32;
    while n < 64 as core::ffi::c_int {
        let mut accu: WORD32 = 0;
        accu = ixheaac_mul32_sh(
            *inp1.offset((n as core::ffi::c_int + 0 as core::ffi::c_int) as isize),
            *p_qmf1.offset((n as core::ffi::c_int + 0 as core::ffi::c_int) as isize),
            31 as WORD8,
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp1.offset((n + 2 as WORD32 * resolution) as isize),
                *p_qmf1.offset((n + 2 as WORD32 * resolution) as isize),
                31 as WORD8,
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp1.offset((n + 4 as WORD32 * resolution) as isize),
                *p_qmf1.offset((n + 4 as WORD32 * resolution) as isize),
                31 as WORD8,
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp1.offset((n + 6 as WORD32 * resolution) as isize),
                *p_qmf1.offset((n + 6 as WORD32 * resolution) as isize),
                31 as WORD8,
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp1.offset((n + 8 as WORD32 * resolution) as isize),
                *p_qmf1.offset((n + 8 as WORD32 * resolution) as isize),
                31 as WORD8,
            ),
        );
        *p_out.offset(n as isize) = accu;
        accu = ixheaac_mul32_sh(
            *inp1
                .offset(
                    (n as core::ffi::c_int + 1 as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ),
            *p_qmf1
                .offset(
                    (n as core::ffi::c_int + 1 as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ),
            31 as WORD8,
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp1.offset((n + 1 as WORD32 + 2 as WORD32 * resolution) as isize),
                *p_qmf1.offset((n + 1 as WORD32 + 2 as WORD32 * resolution) as isize),
                31 as WORD8,
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp1.offset((n + 1 as WORD32 + 4 as WORD32 * resolution) as isize),
                *p_qmf1.offset((n + 1 as WORD32 + 4 as WORD32 * resolution) as isize),
                31 as WORD8,
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp1.offset((n + 1 as WORD32 + 6 as WORD32 * resolution) as isize),
                *p_qmf1.offset((n + 1 as WORD32 + 6 as WORD32 * resolution) as isize),
                31 as WORD8,
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp1.offset((n + 1 as WORD32 + 8 as WORD32 * resolution) as isize),
                *p_qmf1.offset((n + 1 as WORD32 + 8 as WORD32 * resolution) as isize),
                31 as WORD8,
            ),
        );
        *p_out.offset((n as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = accu;
        accu = ixheaac_mul32_sh(
            *inp2.offset((n as core::ffi::c_int + 0 as core::ffi::c_int) as isize),
            *p_qmf2.offset((n as core::ffi::c_int + 0 as core::ffi::c_int) as isize),
            31 as WORD8,
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp2.offset((n + 2 as WORD32 * resolution) as isize),
                *p_qmf2.offset((n + 2 as WORD32 * resolution) as isize),
                31 as WORD8,
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp2.offset((n + 4 as WORD32 * resolution) as isize),
                *p_qmf2.offset((n + 4 as WORD32 * resolution) as isize),
                31 as WORD8,
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp2.offset((n + 6 as WORD32 * resolution) as isize),
                *p_qmf2.offset((n + 6 as WORD32 * resolution) as isize),
                31 as WORD8,
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp2.offset((n + 8 as WORD32 * resolution) as isize),
                *p_qmf2.offset((n + 8 as WORD32 * resolution) as isize),
                31 as WORD8,
            ),
        );
        *p_out.offset((n as core::ffi::c_int + 64 as core::ffi::c_int) as isize) = accu;
        accu = ixheaac_mul32_sh(
            *inp2
                .offset(
                    (n as core::ffi::c_int + 1 as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ),
            *p_qmf2
                .offset(
                    (n as core::ffi::c_int + 1 as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ),
            31 as WORD8,
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp2.offset((n + 1 as WORD32 + 2 as WORD32 * resolution) as isize),
                *p_qmf2.offset((n + 1 as WORD32 + 2 as WORD32 * resolution) as isize),
                31 as WORD8,
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp2.offset((n + 1 as WORD32 + 4 as WORD32 * resolution) as isize),
                *p_qmf2.offset((n + 1 as WORD32 + 4 as WORD32 * resolution) as isize),
                31 as WORD8,
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp2.offset((n + 1 as WORD32 + 6 as WORD32 * resolution) as isize),
                *p_qmf2.offset((n + 1 as WORD32 + 6 as WORD32 * resolution) as isize),
                31 as WORD8,
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp2.offset((n + 1 as WORD32 + 8 as WORD32 * resolution) as isize),
                *p_qmf2.offset((n + 1 as WORD32 + 8 as WORD32 * resolution) as isize),
                31 as WORD8,
            ),
        );
        *p_out
            .offset(
                (n as core::ffi::c_int + 1 as core::ffi::c_int + 64 as core::ffi::c_int)
                    as isize,
            ) = accu;
        n += 2 as core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_sbr_qmfanal32_winadd_eld_32(
    mut inp1: *mut WORD32,
    mut inp2: *mut WORD32,
    mut p_qmf1: *const WORD32,
    mut p_qmf2: *const WORD32,
    mut p_out: *mut WORD32,
) -> VOID {
    let mut n: WORD32 = 0;
    n = 0 as core::ffi::c_int as WORD32;
    while n < 32 as core::ffi::c_int {
        let mut accu: WORD32 = 0;
        accu = ixheaac_mul32_sh(
            *inp1.offset((n as core::ffi::c_int + 0 as core::ffi::c_int) as isize),
            *p_qmf1.offset((n as core::ffi::c_int + 0 as core::ffi::c_int) as isize),
            31 as WORD8,
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp1.offset((n as core::ffi::c_int + 64 as core::ffi::c_int) as isize),
                *p_qmf1
                    .offset((n as core::ffi::c_int + 64 as core::ffi::c_int) as isize),
                31 as WORD8,
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp1.offset((n as core::ffi::c_int + 128 as core::ffi::c_int) as isize),
                *p_qmf1
                    .offset((n as core::ffi::c_int + 128 as core::ffi::c_int) as isize),
                31 as WORD8,
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp1.offset((n as core::ffi::c_int + 192 as core::ffi::c_int) as isize),
                *p_qmf1
                    .offset((n as core::ffi::c_int + 192 as core::ffi::c_int) as isize),
                31 as WORD8,
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp1.offset((n as core::ffi::c_int + 256 as core::ffi::c_int) as isize),
                *p_qmf1
                    .offset((n as core::ffi::c_int + 256 as core::ffi::c_int) as isize),
                31 as WORD8,
            ),
        );
        *p_out.offset(n as isize) = accu;
        accu = ixheaac_mul32_sh(
            *inp1
                .offset(
                    (n as core::ffi::c_int + 1 as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ),
            *p_qmf1
                .offset(
                    (n as core::ffi::c_int + 1 as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ),
            31 as WORD8,
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp1
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 64 as core::ffi::c_int) as isize,
                    ),
                *p_qmf1
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 64 as core::ffi::c_int) as isize,
                    ),
                31 as WORD8,
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp1
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 128 as core::ffi::c_int) as isize,
                    ),
                *p_qmf1
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 128 as core::ffi::c_int) as isize,
                    ),
                31 as WORD8,
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp1
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 192 as core::ffi::c_int) as isize,
                    ),
                *p_qmf1
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 192 as core::ffi::c_int) as isize,
                    ),
                31 as WORD8,
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp1
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 256 as core::ffi::c_int) as isize,
                    ),
                *p_qmf1
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 256 as core::ffi::c_int) as isize,
                    ),
                31 as WORD8,
            ),
        );
        *p_out.offset((n as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = accu;
        accu = ixheaac_mul32_sh(
            *inp2.offset((n as core::ffi::c_int + 0 as core::ffi::c_int) as isize),
            *p_qmf2.offset((n as core::ffi::c_int + 0 as core::ffi::c_int) as isize),
            31 as WORD8,
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp2.offset((n as core::ffi::c_int + 64 as core::ffi::c_int) as isize),
                *p_qmf2
                    .offset((n as core::ffi::c_int + 64 as core::ffi::c_int) as isize),
                31 as WORD8,
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp2.offset((n as core::ffi::c_int + 128 as core::ffi::c_int) as isize),
                *p_qmf2
                    .offset((n as core::ffi::c_int + 128 as core::ffi::c_int) as isize),
                31 as WORD8,
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp2.offset((n as core::ffi::c_int + 192 as core::ffi::c_int) as isize),
                *p_qmf2
                    .offset((n as core::ffi::c_int + 192 as core::ffi::c_int) as isize),
                31 as WORD8,
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp2.offset((n as core::ffi::c_int + 256 as core::ffi::c_int) as isize),
                *p_qmf2
                    .offset((n as core::ffi::c_int + 256 as core::ffi::c_int) as isize),
                31 as WORD8,
            ),
        );
        *p_out.offset((n as core::ffi::c_int + 32 as core::ffi::c_int) as isize) = accu;
        accu = ixheaac_mul32_sh(
            *inp2
                .offset(
                    (n as core::ffi::c_int + 1 as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ),
            *p_qmf2
                .offset(
                    (n as core::ffi::c_int + 1 as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ),
            31 as WORD8,
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp2
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 64 as core::ffi::c_int) as isize,
                    ),
                *p_qmf2
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 64 as core::ffi::c_int) as isize,
                    ),
                31 as WORD8,
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp2
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 128 as core::ffi::c_int) as isize,
                    ),
                *p_qmf2
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 128 as core::ffi::c_int) as isize,
                    ),
                31 as WORD8,
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp2
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 192 as core::ffi::c_int) as isize,
                    ),
                *p_qmf2
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 192 as core::ffi::c_int) as isize,
                    ),
                31 as WORD8,
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mul32_sh(
                *inp2
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 256 as core::ffi::c_int) as isize,
                    ),
                *p_qmf2
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 256 as core::ffi::c_int) as isize,
                    ),
                31 as WORD8,
            ),
        );
        *p_out
            .offset(
                (n as core::ffi::c_int + 1 as core::ffi::c_int + 32 as core::ffi::c_int)
                    as isize,
            ) = accu;
        n += 2 as core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_sbr_qmfanal32_winadd_eld(
    mut inp1: *mut WORD16,
    mut inp2: *mut WORD16,
    mut p_qmf1: *const WORD16,
    mut p_qmf2: *const WORD16,
    mut p_out: *mut WORD32,
) -> VOID {
    let mut n: WORD32 = 0;
    n = 0 as core::ffi::c_int as WORD32;
    while n < 32 as core::ffi::c_int {
        let mut accu: WORD32 = 0;
        accu = ixheaac_mult16x16in32(
            *inp1.offset((n as core::ffi::c_int + 0 as core::ffi::c_int) as isize),
            *p_qmf1.offset((n as core::ffi::c_int + 0 as core::ffi::c_int) as isize),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp1.offset((n as core::ffi::c_int + 64 as core::ffi::c_int) as isize),
                *p_qmf1.offset((n as core::ffi::c_int + 64 as core::ffi::c_int) as isize),
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp1.offset((n as core::ffi::c_int + 128 as core::ffi::c_int) as isize),
                *p_qmf1
                    .offset((n as core::ffi::c_int + 128 as core::ffi::c_int) as isize),
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp1.offset((n as core::ffi::c_int + 192 as core::ffi::c_int) as isize),
                *p_qmf1
                    .offset((n as core::ffi::c_int + 192 as core::ffi::c_int) as isize),
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp1.offset((n as core::ffi::c_int + 256 as core::ffi::c_int) as isize),
                *p_qmf1
                    .offset((n as core::ffi::c_int + 256 as core::ffi::c_int) as isize),
            ),
        );
        *p_out.offset(n as isize) = accu;
        accu = ixheaac_mult16x16in32(
            *inp1
                .offset(
                    (n as core::ffi::c_int + 1 as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ),
            *p_qmf1
                .offset(
                    (n as core::ffi::c_int + 1 as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp1
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 64 as core::ffi::c_int) as isize,
                    ),
                *p_qmf1
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 64 as core::ffi::c_int) as isize,
                    ),
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp1
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 128 as core::ffi::c_int) as isize,
                    ),
                *p_qmf1
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 128 as core::ffi::c_int) as isize,
                    ),
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp1
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 192 as core::ffi::c_int) as isize,
                    ),
                *p_qmf1
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 192 as core::ffi::c_int) as isize,
                    ),
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp1
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 256 as core::ffi::c_int) as isize,
                    ),
                *p_qmf1
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 256 as core::ffi::c_int) as isize,
                    ),
            ),
        );
        *p_out.offset((n as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = accu;
        accu = ixheaac_mult16x16in32(
            *inp2.offset((n as core::ffi::c_int + 0 as core::ffi::c_int) as isize),
            *p_qmf2.offset((n as core::ffi::c_int + 0 as core::ffi::c_int) as isize),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp2.offset((n as core::ffi::c_int + 64 as core::ffi::c_int) as isize),
                *p_qmf2.offset((n as core::ffi::c_int + 64 as core::ffi::c_int) as isize),
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp2.offset((n as core::ffi::c_int + 128 as core::ffi::c_int) as isize),
                *p_qmf2
                    .offset((n as core::ffi::c_int + 128 as core::ffi::c_int) as isize),
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp2.offset((n as core::ffi::c_int + 192 as core::ffi::c_int) as isize),
                *p_qmf2
                    .offset((n as core::ffi::c_int + 192 as core::ffi::c_int) as isize),
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp2.offset((n as core::ffi::c_int + 256 as core::ffi::c_int) as isize),
                *p_qmf2
                    .offset((n as core::ffi::c_int + 256 as core::ffi::c_int) as isize),
            ),
        );
        *p_out.offset((n as core::ffi::c_int + 32 as core::ffi::c_int) as isize) = accu;
        accu = ixheaac_mult16x16in32(
            *inp2
                .offset(
                    (n as core::ffi::c_int + 1 as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ),
            *p_qmf2
                .offset(
                    (n as core::ffi::c_int + 1 as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp2
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 64 as core::ffi::c_int) as isize,
                    ),
                *p_qmf2
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 64 as core::ffi::c_int) as isize,
                    ),
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp2
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 128 as core::ffi::c_int) as isize,
                    ),
                *p_qmf2
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 128 as core::ffi::c_int) as isize,
                    ),
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp2
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 192 as core::ffi::c_int) as isize,
                    ),
                *p_qmf2
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 192 as core::ffi::c_int) as isize,
                    ),
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp2
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 256 as core::ffi::c_int) as isize,
                    ),
                *p_qmf2
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 256 as core::ffi::c_int) as isize,
                    ),
            ),
        );
        *p_out
            .offset(
                (n as core::ffi::c_int + 1 as core::ffi::c_int + 32 as core::ffi::c_int)
                    as isize,
            ) = accu;
        n += 2 as core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_esbr_qmfanal32_winadd(
    mut inp1: *mut WORD32,
    mut inp2: *mut WORD32,
    mut p_qmf1: *mut WORD32,
    mut p_qmf2: *mut WORD32,
    mut p_out: *mut WORD32,
    mut num_band_anal_qmf: WORD32,
) -> VOID {
    let mut n: WORD32 = 0;
    let mut accu: WORD64 = 0;
    if num_band_anal_qmf == 32 as core::ffi::c_int {
        n = 0 as core::ffi::c_int as WORD32;
        while n < num_band_anal_qmf {
            accu = ixheaac_mult64(
                *inp1.offset((n as core::ffi::c_int + 0 as core::ffi::c_int) as isize),
                *p_qmf1
                    .offset(
                        (2 as core::ffi::c_int
                            * (n as core::ffi::c_int + 0 as core::ffi::c_int)) as isize,
                    ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp1.offset((n + 2 as WORD32 * num_band_anal_qmf) as isize),
                    *p_qmf1
                        .offset(
                            (2 as WORD32 * (n + 2 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp1.offset((n + 4 as WORD32 * num_band_anal_qmf) as isize),
                    *p_qmf1
                        .offset(
                            (2 as WORD32 * (n + 4 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp1.offset((n + 6 as WORD32 * num_band_anal_qmf) as isize),
                    *p_qmf1
                        .offset(
                            (2 as WORD32 * (n + 6 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp1.offset((n + 8 as WORD32 * num_band_anal_qmf) as isize),
                    *p_qmf1
                        .offset(
                            (2 as WORD32 * (n + 8 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            *p_out.offset(n as isize) = (accu >> 31 as core::ffi::c_int) as WORD32;
            accu = ixheaac_mult64(
                *inp1
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 0 as core::ffi::c_int) as isize,
                    ),
                *p_qmf1
                    .offset(
                        (2 as core::ffi::c_int
                            * (n as core::ffi::c_int + 1 as core::ffi::c_int
                                + 0 as core::ffi::c_int)) as isize,
                    ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp1
                        .offset(
                            (n + 1 as WORD32 + 2 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                    *p_qmf1
                        .offset(
                            (2 as WORD32
                                * (n + 1 as WORD32 + 2 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp1
                        .offset(
                            (n + 1 as WORD32 + 4 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                    *p_qmf1
                        .offset(
                            (2 as WORD32
                                * (n + 1 as WORD32 + 4 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp1
                        .offset(
                            (n + 1 as WORD32 + 6 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                    *p_qmf1
                        .offset(
                            (2 as WORD32
                                * (n + 1 as WORD32 + 6 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp1
                        .offset(
                            (n + 1 as WORD32 + 8 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                    *p_qmf1
                        .offset(
                            (2 as WORD32
                                * (n + 1 as WORD32 + 8 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            *p_out.offset((n as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = (accu
                >> 31 as core::ffi::c_int) as WORD32;
            accu = ixheaac_mult64(
                *inp2.offset((n as core::ffi::c_int + 0 as core::ffi::c_int) as isize),
                *p_qmf2
                    .offset(
                        (2 as core::ffi::c_int
                            * (n as core::ffi::c_int + 0 as core::ffi::c_int)) as isize,
                    ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp2.offset((n + 2 as WORD32 * num_band_anal_qmf) as isize),
                    *p_qmf2
                        .offset(
                            (2 as WORD32 * (n + 2 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp2.offset((n + 4 as WORD32 * num_band_anal_qmf) as isize),
                    *p_qmf2
                        .offset(
                            (2 as WORD32 * (n + 4 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp2.offset((n + 6 as WORD32 * num_band_anal_qmf) as isize),
                    *p_qmf2
                        .offset(
                            (2 as WORD32 * (n + 6 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp2.offset((n + 8 as WORD32 * num_band_anal_qmf) as isize),
                    *p_qmf2
                        .offset(
                            (2 as WORD32 * (n + 8 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            *p_out.offset((n + num_band_anal_qmf) as isize) = (accu
                >> 31 as core::ffi::c_int) as WORD32;
            accu = ixheaac_mult64(
                *inp2
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 0 as core::ffi::c_int) as isize,
                    ),
                *p_qmf2
                    .offset(
                        (2 as core::ffi::c_int
                            * (n as core::ffi::c_int + 1 as core::ffi::c_int
                                + 0 as core::ffi::c_int)) as isize,
                    ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp2
                        .offset(
                            (n + 1 as WORD32 + 2 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                    *p_qmf2
                        .offset(
                            (2 as WORD32
                                * (n + 1 as WORD32 + 2 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp2
                        .offset(
                            (n + 1 as WORD32 + 4 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                    *p_qmf2
                        .offset(
                            (2 as WORD32
                                * (n + 1 as WORD32 + 4 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp2
                        .offset(
                            (n + 1 as WORD32 + 6 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                    *p_qmf2
                        .offset(
                            (2 as WORD32
                                * (n + 1 as WORD32 + 6 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp2
                        .offset(
                            (n + 1 as WORD32 + 8 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                    *p_qmf2
                        .offset(
                            (2 as WORD32
                                * (n + 1 as WORD32 + 8 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            *p_out.offset((n + 1 as WORD32 + num_band_anal_qmf) as isize) = (accu
                >> 31 as core::ffi::c_int) as WORD32;
            n += 2 as core::ffi::c_int;
        }
    } else if num_band_anal_qmf == 24 as core::ffi::c_int {
        n = 0 as core::ffi::c_int as WORD32;
        while n < num_band_anal_qmf {
            accu = ixheaac_mult64(
                *inp1.offset((n as core::ffi::c_int + 0 as core::ffi::c_int) as isize),
                *p_qmf1.offset((n as core::ffi::c_int + 0 as core::ffi::c_int) as isize),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp1.offset((n + 2 as WORD32 * num_band_anal_qmf) as isize),
                    *p_qmf1.offset((n + 2 as WORD32 * num_band_anal_qmf) as isize),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp1.offset((n + 4 as WORD32 * num_band_anal_qmf) as isize),
                    *p_qmf1.offset((n + 4 as WORD32 * num_band_anal_qmf) as isize),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp1.offset((n + 6 as WORD32 * num_band_anal_qmf) as isize),
                    *p_qmf1.offset((n + 6 as WORD32 * num_band_anal_qmf) as isize),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp1.offset((n + 8 as WORD32 * num_band_anal_qmf) as isize),
                    *p_qmf1.offset((n + 8 as WORD32 * num_band_anal_qmf) as isize),
                ),
            );
            *p_out.offset(n as isize) = (accu >> 31 as core::ffi::c_int) as WORD32;
            accu = ixheaac_mult64(
                *inp1
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 0 as core::ffi::c_int) as isize,
                    ),
                *p_qmf1
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 0 as core::ffi::c_int) as isize,
                    ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp1
                        .offset(
                            (n + 1 as WORD32 + 2 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                    *p_qmf1
                        .offset(
                            (n + 1 as WORD32 + 2 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp1
                        .offset(
                            (n + 1 as WORD32 + 4 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                    *p_qmf1
                        .offset(
                            (n + 1 as WORD32 + 4 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp1
                        .offset(
                            (n + 1 as WORD32 + 6 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                    *p_qmf1
                        .offset(
                            (n + 1 as WORD32 + 6 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp1
                        .offset(
                            (n + 1 as WORD32 + 8 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                    *p_qmf1
                        .offset(
                            (n + 1 as WORD32 + 8 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                ),
            );
            *p_out.offset((n as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = (accu
                >> 31 as core::ffi::c_int) as WORD32;
            accu = ixheaac_mult64(
                *inp2.offset((n as core::ffi::c_int + 0 as core::ffi::c_int) as isize),
                *p_qmf2.offset((n as core::ffi::c_int + 0 as core::ffi::c_int) as isize),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp2.offset((n + 2 as WORD32 * num_band_anal_qmf) as isize),
                    *p_qmf2.offset((n + 2 as WORD32 * num_band_anal_qmf) as isize),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp2.offset((n + 4 as WORD32 * num_band_anal_qmf) as isize),
                    *p_qmf2.offset((n + 4 as WORD32 * num_band_anal_qmf) as isize),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp2.offset((n + 6 as WORD32 * num_band_anal_qmf) as isize),
                    *p_qmf2.offset((n + 6 as WORD32 * num_band_anal_qmf) as isize),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp2.offset((n + 8 as WORD32 * num_band_anal_qmf) as isize),
                    *p_qmf2.offset((n + 8 as WORD32 * num_band_anal_qmf) as isize),
                ),
            );
            *p_out.offset((n + num_band_anal_qmf) as isize) = (accu
                >> 31 as core::ffi::c_int) as WORD32;
            accu = ixheaac_mult64(
                *inp2
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 0 as core::ffi::c_int) as isize,
                    ),
                *p_qmf2
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 0 as core::ffi::c_int) as isize,
                    ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp2
                        .offset(
                            (n + 1 as WORD32 + 2 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                    *p_qmf2
                        .offset(
                            (n + 1 as WORD32 + 2 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp2
                        .offset(
                            (n + 1 as WORD32 + 4 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                    *p_qmf2
                        .offset(
                            (n + 1 as WORD32 + 4 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp2
                        .offset(
                            (n + 1 as WORD32 + 6 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                    *p_qmf2
                        .offset(
                            (n + 1 as WORD32 + 6 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp2
                        .offset(
                            (n + 1 as WORD32 + 8 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                    *p_qmf2
                        .offset(
                            (n + 1 as WORD32 + 8 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                ),
            );
            *p_out.offset((n + 1 as WORD32 + num_band_anal_qmf) as isize) = (accu
                >> 31 as core::ffi::c_int) as WORD32;
            n += 2 as core::ffi::c_int;
        }
    } else {
        n = 0 as core::ffi::c_int as WORD32;
        while n < num_band_anal_qmf {
            accu = ixheaac_mult64(
                *inp1.offset((n as core::ffi::c_int + 0 as core::ffi::c_int) as isize),
                *p_qmf1
                    .offset(
                        (4 as core::ffi::c_int
                            * (n as core::ffi::c_int + 0 as core::ffi::c_int)) as isize,
                    ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp1.offset((n + 2 as WORD32 * num_band_anal_qmf) as isize),
                    *p_qmf1
                        .offset(
                            (4 as WORD32 * (n + 2 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp1.offset((n + 4 as WORD32 * num_band_anal_qmf) as isize),
                    *p_qmf1
                        .offset(
                            (4 as WORD32 * (n + 4 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp1.offset((n + 6 as WORD32 * num_band_anal_qmf) as isize),
                    *p_qmf1
                        .offset(
                            (4 as WORD32 * (n + 6 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp1.offset((n + 8 as WORD32 * num_band_anal_qmf) as isize),
                    *p_qmf1
                        .offset(
                            (4 as WORD32 * (n + 8 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            *p_out.offset(n as isize) = (accu >> 31 as core::ffi::c_int) as WORD32;
            accu = ixheaac_mult64(
                *inp1
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 0 as core::ffi::c_int) as isize,
                    ),
                *p_qmf1
                    .offset(
                        (4 as core::ffi::c_int
                            * (n as core::ffi::c_int + 1 as core::ffi::c_int
                                + 0 as core::ffi::c_int)) as isize,
                    ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp1
                        .offset(
                            (n + 1 as WORD32 + 2 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                    *p_qmf1
                        .offset(
                            (4 as WORD32
                                * (n + 1 as WORD32 + 2 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp1
                        .offset(
                            (n + 1 as WORD32 + 4 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                    *p_qmf1
                        .offset(
                            (4 as WORD32
                                * (n + 1 as WORD32 + 4 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp1
                        .offset(
                            (n + 1 as WORD32 + 6 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                    *p_qmf1
                        .offset(
                            (4 as WORD32
                                * (n + 1 as WORD32 + 6 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp1
                        .offset(
                            (n + 1 as WORD32 + 8 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                    *p_qmf1
                        .offset(
                            (4 as WORD32
                                * (n + 1 as WORD32 + 8 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            *p_out.offset((n as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = (accu
                >> 31 as core::ffi::c_int) as WORD32;
            accu = ixheaac_mult64(
                *inp2.offset((n as core::ffi::c_int + 0 as core::ffi::c_int) as isize),
                *p_qmf2
                    .offset(
                        (4 as core::ffi::c_int
                            * (n as core::ffi::c_int + 0 as core::ffi::c_int)) as isize,
                    ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp2.offset((n + 2 as WORD32 * num_band_anal_qmf) as isize),
                    *p_qmf2
                        .offset(
                            (4 as WORD32 * (n + 2 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp2.offset((n + 4 as WORD32 * num_band_anal_qmf) as isize),
                    *p_qmf2
                        .offset(
                            (4 as WORD32 * (n + 4 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp2.offset((n + 6 as WORD32 * num_band_anal_qmf) as isize),
                    *p_qmf2
                        .offset(
                            (4 as WORD32 * (n + 6 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp2.offset((n + 8 as WORD32 * num_band_anal_qmf) as isize),
                    *p_qmf2
                        .offset(
                            (4 as WORD32 * (n + 8 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            *p_out.offset((n + num_band_anal_qmf) as isize) = (accu
                >> 31 as core::ffi::c_int) as WORD32;
            accu = ixheaac_mult64(
                *inp2
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 0 as core::ffi::c_int) as isize,
                    ),
                *p_qmf2
                    .offset(
                        (4 as core::ffi::c_int
                            * (n as core::ffi::c_int + 1 as core::ffi::c_int
                                + 0 as core::ffi::c_int)) as isize,
                    ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp2
                        .offset(
                            (n + 1 as WORD32 + 2 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                    *p_qmf2
                        .offset(
                            (4 as WORD32
                                * (n + 1 as WORD32 + 2 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp2
                        .offset(
                            (n + 1 as WORD32 + 4 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                    *p_qmf2
                        .offset(
                            (4 as WORD32
                                * (n + 1 as WORD32 + 4 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp2
                        .offset(
                            (n + 1 as WORD32 + 6 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                    *p_qmf2
                        .offset(
                            (4 as WORD32
                                * (n + 1 as WORD32 + 6 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            accu = ixheaac_add64(
                accu,
                ixheaac_mult64(
                    *inp2
                        .offset(
                            (n + 1 as WORD32 + 8 as WORD32 * num_band_anal_qmf) as isize,
                        ),
                    *p_qmf2
                        .offset(
                            (4 as WORD32
                                * (n + 1 as WORD32 + 8 as WORD32 * num_band_anal_qmf))
                                as isize,
                        ),
                ),
            );
            *p_out.offset((n + 1 as WORD32 + num_band_anal_qmf) as isize) = (accu
                >> 31 as core::ffi::c_int) as WORD32;
            n += 2 as core::ffi::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_esbr_inv_modulation(
    mut qmf_real: *mut WORD32,
    mut syn_qmf: *mut ia_sbr_qmf_filter_bank_struct,
    mut qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
    mut no_synthesis_channels: WORD32,
) -> VOID {
    if no_synthesis_channels == NO_SYNTHESIS_CHANNELS_DOWN_SAMPLED {
        ixheaacd_esbr_cos_sin_mod(
            qmf_real,
            syn_qmf,
            ((*qmf_dec_tables_ptr).esbr_w_16).as_mut_ptr(),
            ((*qmf_dec_tables_ptr).dig_rev_table4_16).as_mut_ptr(),
        );
    } else {
        ixheaacd_esbr_cos_sin_mod(
            qmf_real,
            syn_qmf,
            ((*qmf_dec_tables_ptr).esbr_w_32).as_mut_ptr(),
            ((*qmf_dec_tables_ptr).dig_rev_table2_32).as_mut_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_sbr_qmfsyn32_winadd(
    mut tmp1: *mut WORD16,
    mut tmp2: *mut WORD16,
    mut inp1: *mut WORD16,
    mut sample_buffer: *mut WORD16,
    mut shift: FLAG,
    mut ch_fac: WORD32,
) -> VOID {
    let mut k: WORD32 = 0;
    let mut rounding_fac: WORD32 = 0x8000 as WORD32;
    rounding_fac = rounding_fac >> shift;
    k = 0 as core::ffi::c_int as WORD32;
    while k < 32 as core::ffi::c_int {
        let mut syn_out: WORD32 = rounding_fac;
        syn_out = ixheaac_add32(
            syn_out,
            ixheaac_mult16x16in32(
                *tmp1.offset((0 as WORD32 + k) as isize),
                *inp1
                    .offset(
                        (2 as core::ffi::c_int
                            * (k as core::ffi::c_int + 0 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        syn_out = ixheaac_add32(
            syn_out,
            ixheaac_mult16x16in32(
                *tmp1.offset((128 as WORD32 + k) as isize),
                *inp1
                    .offset(
                        (2 as core::ffi::c_int
                            * (k as core::ffi::c_int + 64 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        syn_out = ixheaac_add32(
            syn_out,
            ixheaac_mult16x16in32(
                *tmp1.offset((256 as WORD32 + k) as isize),
                *inp1
                    .offset(
                        (2 as core::ffi::c_int
                            * (k as core::ffi::c_int + 128 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        syn_out = ixheaac_add32(
            syn_out,
            ixheaac_mult16x16in32(
                *tmp1.offset((384 as WORD32 + k) as isize),
                *inp1
                    .offset(
                        (2 as core::ffi::c_int
                            * (k as core::ffi::c_int + 192 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        syn_out = ixheaac_add32(
            syn_out,
            ixheaac_mult16x16in32(
                *tmp1.offset((512 as WORD32 + k) as isize),
                *inp1
                    .offset(
                        (2 as core::ffi::c_int
                            * (k as core::ffi::c_int + 256 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        syn_out = ixheaac_add32(
            syn_out,
            ixheaac_mult16x16in32(
                *tmp2.offset((64 as WORD32 + k) as isize),
                *inp1
                    .offset(
                        (2 as core::ffi::c_int
                            * (k as core::ffi::c_int + 32 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        syn_out = ixheaac_add32(
            syn_out,
            ixheaac_mult16x16in32(
                *tmp2.offset((192 as WORD32 + k) as isize),
                *inp1
                    .offset(
                        (2 as core::ffi::c_int
                            * (k as core::ffi::c_int + 96 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        syn_out = ixheaac_add32(
            syn_out,
            ixheaac_mult16x16in32(
                *tmp2.offset((320 as WORD32 + k) as isize),
                *inp1
                    .offset(
                        (2 as core::ffi::c_int
                            * (k as core::ffi::c_int + 160 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        syn_out = ixheaac_add32(
            syn_out,
            ixheaac_mult16x16in32(
                *tmp2.offset((448 as WORD32 + k) as isize),
                *inp1
                    .offset(
                        (2 as core::ffi::c_int
                            * (k as core::ffi::c_int + 224 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        syn_out = ixheaac_add32(
            syn_out,
            ixheaac_mult16x16in32(
                *tmp2.offset((576 as WORD32 + k) as isize),
                *inp1
                    .offset(
                        (2 as core::ffi::c_int
                            * (k as core::ffi::c_int + 288 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        syn_out = ixheaac_add32_sat(syn_out, syn_out);
        if shift == 2 as core::ffi::c_int {
            syn_out = ixheaac_add32_sat(syn_out, syn_out);
        }
        *sample_buffer.offset((ch_fac * k) as isize) = (syn_out
            >> 16 as core::ffi::c_int) as WORD16;
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_sbr_pre_twiddle(
    mut p_xre: *mut WORD32,
    mut p_xim: *mut WORD32,
    mut p_twiddles: *mut WORD16,
) -> VOID {
    let mut k: core::ffi::c_int = 0;
    k = 62 as core::ffi::c_int;
    while k >= 0 as core::ffi::c_int {
        let mut x_re: WORD32 = *p_xre;
        let mut x_im: WORD32 = *p_xim;
        let fresh0 = p_twiddles;
        p_twiddles = p_twiddles.offset(1);
        let mut ixheaacd_cosine: WORD16 = *fresh0;
        let fresh1 = p_twiddles;
        p_twiddles = p_twiddles.offset(1);
        let mut ixheaacd_sine: WORD16 = *fresh1;
        let mut re: WORD32 = 0;
        let mut im: WORD32 = 0;
        re = ixheaac_mac32x16in32_shl_sat(
            ixheaac_mult32x16in32_shl(x_re, ixheaacd_cosine),
            x_im,
            ixheaacd_sine,
        );
        im = ixheaac_sub32_sat(
            ixheaac_mult32x16in32_shl(x_im, ixheaacd_cosine),
            ixheaac_mult32x16in32_shl(x_re, ixheaacd_sine),
        );
        let fresh2 = p_xre;
        p_xre = p_xre.offset(1);
        *fresh2 = re;
        let fresh3 = p_xim;
        p_xim = p_xim.offset(1);
        *fresh3 = im;
        k -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_cplx_synt_qmffilt(
    mut qmf_real: *mut *mut WORD32,
    mut qmf_imag: *mut *mut WORD32,
    mut split: WORD32,
    mut qmf_real_out: *mut *mut WORD32,
    mut qmf_imag_out: *mut *mut WORD32,
    mut sbr_scale_factor: *mut ia_sbr_scale_fact_struct,
    mut time_out: *mut WORD16,
    mut qmf_bank: *mut ia_sbr_qmf_filter_bank_struct,
    mut ptr_ps_dec: *mut ia_ps_dec_struct,
    mut active: FLAG,
    mut low_pow_flag: FLAG,
    mut sbr_tables_ptr: *mut ia_sbr_tables_struct,
    mut pstr_common_tables: *mut ixheaacd_misc_tables,
    mut ch_fac: WORD32,
    mut drc_on: FLAG,
    mut drc_sbr_factors: *mut [WORD32; 64],
    mut audio_object_type: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut code_scale_factor: WORD32 = 0;
    let mut scale_factor: WORD32 = 0;
    let mut out_scale_factor: WORD32 = 0;
    let mut low_band_scale_factor: WORD32 = 0;
    let mut high_band_scale_factor: WORD32 = 0;
    let mut filter_states: *mut WORD16 = (*qmf_bank).filter_states;
    let mut ptr_qmf_imag_temp: *mut *mut WORD32 = 0 as *mut *mut WORD32;
    let mut qmf_real2: [WORD32; 128] = [0; 128];
    let mut no_synthesis_channels: WORD32 = (*qmf_bank).no_channels;
    let mut p1: WORD32 = 0;
    let mut fp1: *mut WORD16 = 0 as *mut WORD16;
    let mut fp2: *mut WORD16 = 0 as *mut WORD16;
    let mut sixty4: WORD32 = NO_SYNTHESIS_CHANNELS;
    let mut thirty2: WORD32 = (*qmf_bank).no_channels;
    let mut filter_coeff: *mut WORD16 = 0 as *mut WORD16;
    let mut num_time_slots: WORD32 = (*qmf_bank).num_time_slots as WORD32;
    let mut ixheaacd_drc_offset: WORD32 = 0;
    let mut ov_lb_scale: WORD32 = (*sbr_scale_factor).ov_lb_scale as WORD32;
    let mut lb_scale: WORD32 = (*sbr_scale_factor).lb_scale as WORD32;
    let mut st_syn_scale: WORD32 = (*sbr_scale_factor).st_syn_scale as WORD32;
    let mut ov_lb_shift: WORD32 = 0;
    let mut lb_shift: WORD32 = 0;
    let mut hb_shift: WORD32 = 0;
    let mut qmf_real_tmp: *mut WORD32 = qmf_real2.as_mut_ptr();
    let mut qmf_imag_tmp: *mut WORD32 = &mut *qmf_real2
        .as_mut_ptr()
        .offset(NO_SYNTHESIS_CHANNELS as isize) as *mut WORD32;
    let mut env: WORD32 = 0 as WORD32;
    let mut common_shift: WORD32 = 0 as WORD32;
    if no_synthesis_channels == 32 as core::ffi::c_int {
        (*qmf_bank).cos_twiddle = ((*(*sbr_tables_ptr).qmf_dec_tables_ptr)
            .sbr_sin_cos_twiddle_l32)
            .as_mut_ptr();
        (*qmf_bank).alt_sin_twiddle = ((*(*sbr_tables_ptr).qmf_dec_tables_ptr)
            .sbr_alt_sin_twiddle_l32)
            .as_mut_ptr();
        (*qmf_bank).t_cos = ((*(*sbr_tables_ptr).qmf_dec_tables_ptr)
            .sbr_cos_sin_twiddle_ds_l32)
            .as_mut_ptr();
    } else {
        (*qmf_bank).cos_twiddle = ((*(*sbr_tables_ptr).qmf_dec_tables_ptr)
            .sbr_sin_cos_twiddle_l64)
            .as_mut_ptr();
        (*qmf_bank).alt_sin_twiddle = ((*(*sbr_tables_ptr).qmf_dec_tables_ptr)
            .sbr_alt_sin_twiddle_l64)
            .as_mut_ptr();
    }
    if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
        && audio_object_type != AOT_ER_AAC_LD as core::ffi::c_int
    {
        (*qmf_bank).filter_pos_syn = ((*qmf_bank).filter_pos_syn)
            .offset(
                ((*(*sbr_tables_ptr).qmf_dec_tables_ptr).qmf_c)
                    .as_mut_ptr()
                    .offset_from((*qmf_bank).p_filter) as core::ffi::c_long as isize,
            );
        (*qmf_bank).p_filter = ((*(*sbr_tables_ptr).qmf_dec_tables_ptr).qmf_c)
            .as_mut_ptr();
    } else {
        (*qmf_bank).filter_pos_syn = ((*qmf_bank).filter_pos_syn)
            .offset(
                ((*(*sbr_tables_ptr).qmf_dec_tables_ptr).qmf_c_eld)
                    .as_mut_ptr()
                    .offset_from((*qmf_bank).p_filter) as core::ffi::c_long as isize,
            );
        (*qmf_bank).p_filter = ((*(*sbr_tables_ptr).qmf_dec_tables_ptr).qmf_c_eld)
            .as_mut_ptr();
    }
    fp1 = &mut *filter_states.offset(0 as core::ffi::c_int as isize) as *mut WORD16;
    fp2 = fp1.offset(no_synthesis_channels as isize);
    if audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int
        || audio_object_type == AOT_ER_AAC_LD as core::ffi::c_int
    {
        fp1 = (*qmf_bank).fp1_syn;
        fp2 = (*qmf_bank).fp2_syn;
        sixty4 = (*qmf_bank).sixty4 as WORD32;
    }
    filter_coeff = (*qmf_bank).filter_pos_syn;
    if active != 0 {
        scale_factor = (*sbr_scale_factor).ps_scale as WORD32;
        code_scale_factor = scale_factor;
    } else {
        code_scale_factor = ixheaac_min32(lb_scale, ov_lb_scale);
        scale_factor = (*sbr_scale_factor).hb_scale as WORD32;
    }
    low_band_scale_factor = st_syn_scale - code_scale_factor;
    high_band_scale_factor = st_syn_scale - scale_factor;
    p1 = 0 as core::ffi::c_int as WORD32;
    if low_pow_flag != 0 {
        ov_lb_shift = (st_syn_scale as core::ffi::c_int - ov_lb_scale as core::ffi::c_int
            - 4 as core::ffi::c_int) as WORD32;
        lb_shift = (st_syn_scale as core::ffi::c_int - lb_scale as core::ffi::c_int
            - 4 as core::ffi::c_int) as WORD32;
        hb_shift = (high_band_scale_factor as core::ffi::c_int - 4 as core::ffi::c_int)
            as WORD32;
        out_scale_factor = -((*sbr_scale_factor).st_syn_scale as core::ffi::c_int
            - 1 as core::ffi::c_int) as WORD32;
        ptr_qmf_imag_temp = 0 as *mut *mut WORD32;
    } else {
        out_scale_factor = -((*sbr_scale_factor).st_syn_scale as core::ffi::c_int
            - 3 as core::ffi::c_int) as WORD32;
        if active != 0 {
            ov_lb_shift = (*sbr_scale_factor).ps_scale as WORD32 - ov_lb_scale;
            lb_shift = (*sbr_scale_factor).ps_scale as WORD32 - lb_scale;
            hb_shift = ((*sbr_scale_factor).ps_scale as core::ffi::c_int
                - (*sbr_scale_factor).hb_scale as core::ffi::c_int) as WORD32;
            common_shift = (low_band_scale_factor as core::ffi::c_int
                - 8 as core::ffi::c_int) as WORD32;
        } else {
            if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
                && audio_object_type != AOT_ER_AAC_LD as core::ffi::c_int
            {
                ov_lb_shift = (st_syn_scale as core::ffi::c_int
                    - ov_lb_scale as core::ffi::c_int - 8 as core::ffi::c_int) as WORD32;
                lb_shift = (st_syn_scale as core::ffi::c_int
                    - lb_scale as core::ffi::c_int - 8 as core::ffi::c_int) as WORD32;
                hb_shift = (high_band_scale_factor as core::ffi::c_int
                    - 8 as core::ffi::c_int) as WORD32;
            } else {
                ov_lb_shift = (st_syn_scale as core::ffi::c_int
                    - ov_lb_scale as core::ffi::c_int - 7 as core::ffi::c_int) as WORD32;
                lb_shift = (st_syn_scale as core::ffi::c_int
                    - lb_scale as core::ffi::c_int - 7 as core::ffi::c_int) as WORD32;
                hb_shift = (high_band_scale_factor as core::ffi::c_int
                    - 7 as core::ffi::c_int) as WORD32;
            }
            common_shift = 0 as core::ffi::c_int as WORD32;
        }
        ptr_qmf_imag_temp = qmf_imag;
    }
    if ov_lb_shift == lb_shift {
        (Some(ixheaacd_adjust_scale.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            qmf_real,
            ptr_qmf_imag_temp,
            0 as WORD32,
            (*qmf_bank).lsb as WORD32,
            0 as WORD32,
            num_time_slots,
            ov_lb_shift,
            low_pow_flag,
        );
    } else {
        (Some(ixheaacd_adjust_scale.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            qmf_real,
            ptr_qmf_imag_temp,
            0 as WORD32,
            (*qmf_bank).lsb as WORD32,
            0 as WORD32,
            split,
            ov_lb_shift,
            low_pow_flag,
        );
        (Some(ixheaacd_adjust_scale.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            qmf_real,
            ptr_qmf_imag_temp,
            0 as WORD32,
            (*qmf_bank).lsb as WORD32,
            split,
            num_time_slots,
            lb_shift,
            low_pow_flag,
        );
    }
    (Some(ixheaacd_adjust_scale.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        qmf_real,
        ptr_qmf_imag_temp,
        (*qmf_bank).lsb as WORD32,
        (*qmf_bank).usb as WORD32,
        0 as WORD32,
        num_time_slots,
        hb_shift,
        low_pow_flag,
    );
    ixheaacd_drc_offset = (*qmf_bank).ixheaacd_drc_offset as WORD32;
    if 1 as core::ffi::c_int == drc_on {
        i = 0 as core::ffi::c_int as WORD32;
        while i < num_time_slots {
            let mut loop_val: WORD32 = 0;
            loop_val = 0 as core::ffi::c_int as WORD32;
            while loop_val < 64 as core::ffi::c_int {
                *(*qmf_real.offset(i as isize)).offset(loop_val as isize) = ixheaacd_mult32x32in32_shift25(
                    *(*qmf_real.offset(i as isize)).offset(loop_val as isize),
                    (*drc_sbr_factors
                        .offset((6 as WORD32 + i) as isize))[loop_val as usize],
                );
                loop_val += 1;
            }
            i += 1;
        }
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_time_slots {
        j = 0 as core::ffi::c_int as WORD32;
        while j < no_synthesis_channels {
            *(*qmf_real_out.offset(i as isize)).offset(j as isize) = *(*qmf_real
                .offset(i as isize))
                .offset(j as isize);
            if low_pow_flag == 0 {
                *(*qmf_imag_out.offset(i as isize)).offset(j as isize) = *(*qmf_imag
                    .offset(i as isize))
                    .offset(j as isize);
            }
            j += 1;
        }
        i += 1;
    }
    if low_pow_flag != 0 {
        let mut fptemp: *mut WORD16 = 0 as *mut WORD16;
        let mut sbr_qmf_syn_winadd: Option<
            unsafe extern "C" fn(
                *mut WORD16,
                *mut WORD16,
                *mut WORD16,
                *mut WORD16,
                FLAG,
                WORD32,
            ) -> VOID,
        > = None;
        let mut qmf_tab_ptr: *mut ia_qmf_dec_tables_struct = (*sbr_tables_ptr)
            .qmf_dec_tables_ptr;
        if no_synthesis_channels == NO_SYNTHESIS_CHANNELS_DOWN_SAMPLED {
            sbr_qmf_syn_winadd = Some(
                ixheaacd_sbr_qmfsyn32_winadd
                    as unsafe extern "C" fn(
                        *mut WORD16,
                        *mut WORD16,
                        *mut WORD16,
                        *mut WORD16,
                        FLAG,
                        WORD32,
                    ) -> VOID,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut WORD16,
                        *mut WORD16,
                        *mut WORD16,
                        *mut WORD16,
                        FLAG,
                        WORD32,
                    ) -> VOID,
                >;
        } else {
            sbr_qmf_syn_winadd = Some(
                ixheaacd_sbr_qmfsyn64_winadd
                    as unsafe extern "C" fn(
                        *mut WORD16,
                        *mut WORD16,
                        *mut WORD16,
                        *mut WORD16,
                        FLAG,
                        WORD32,
                    ) -> VOID,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut WORD16,
                        *mut WORD16,
                        *mut WORD16,
                        *mut WORD16,
                        FLAG,
                        WORD32,
                    ) -> VOID,
                >;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < num_time_slots {
            ixheaacd_inv_modulation_lp(
                *qmf_real.offset(i as isize),
                &mut *filter_states.offset(ixheaacd_drc_offset as isize),
                qmf_bank,
                qmf_tab_ptr,
            );
            sbr_qmf_syn_winadd
                .expect(
                    "non-null function pointer",
                )(
                fp1,
                fp2,
                filter_coeff,
                &mut *time_out.offset((ch_fac * p1) as isize),
                2 as FLAG,
                ch_fac,
            );
            ixheaacd_drc_offset -= no_synthesis_channels << 1 as core::ffi::c_int;
            if ixheaacd_drc_offset < 0 as core::ffi::c_int {
                ixheaacd_drc_offset
                    += ((no_synthesis_channels as core::ffi::c_int)
                        << 1 as core::ffi::c_int) * 10 as core::ffi::c_int;
            }
            fptemp = fp1;
            fp1 = fp2;
            fp2 = fptemp;
            filter_coeff = filter_coeff.offset(64 as core::ffi::c_int as isize);
            if filter_coeff
                == ((*qmf_bank).p_filter).offset(640 as core::ffi::c_int as isize)
                    as *mut WORD16
            {
                filter_coeff = (*qmf_bank).p_filter as *mut WORD16;
            }
            p1 += no_synthesis_channels;
            i += 1;
        }
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i < num_time_slots {
            let mut t_qmf_imag: *mut WORD32 = 0 as *mut WORD32;
            t_qmf_imag = *qmf_imag.offset(i as isize);
            if active != 0 {
                if i == (*ptr_ps_dec).border_position[env as usize] as core::ffi::c_int {
                    ixheaacd_init_rot_env(
                        ptr_ps_dec,
                        env as WORD16,
                        (*qmf_bank).usb,
                        sbr_tables_ptr,
                        ((*pstr_common_tables).trig_data).as_ptr(),
                    );
                    env += 1;
                }
                ixheaacd_apply_ps(
                    ptr_ps_dec,
                    &mut *qmf_real.offset(i as isize),
                    &mut *qmf_imag.offset(i as isize),
                    qmf_real_tmp,
                    qmf_imag_tmp,
                    sbr_scale_factor,
                    i as WORD16,
                    sbr_tables_ptr,
                    num_time_slots as WORD,
                );
            }
            if 1 as core::ffi::c_int == drc_on {
                let mut loop_val_0: WORD32 = 0;
                loop_val_0 = 0 as core::ffi::c_int as WORD32;
                while loop_val_0 < 64 as core::ffi::c_int {
                    *(*qmf_real.offset(i as isize)).offset(loop_val_0 as isize) = ixheaacd_mult32x32in32_shift25(
                        *(*qmf_real.offset(i as isize)).offset(loop_val_0 as isize),
                        (*drc_sbr_factors
                            .offset((6 as WORD32 + i) as isize))[loop_val_0 as usize],
                    );
                    loop_val_0 += 1;
                }
            }
            if active != 0 {
                if common_shift != 0 {
                    ixheaacd_shiftrountine(
                        *qmf_real.offset(i as isize),
                        t_qmf_imag,
                        no_synthesis_channels,
                        common_shift,
                    );
                }
            }
            if audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int
                || audio_object_type == AOT_ER_AAC_LD as core::ffi::c_int
            {
                ixheaacd_sbr_pre_twiddle(
                    *qmf_real.offset(i as isize),
                    t_qmf_imag,
                    ((*(*sbr_tables_ptr).qmf_dec_tables_ptr)
                        .ixheaacd_sbr_synth_cos_sin_l32)
                        .as_mut_ptr(),
                );
            }
            ixheaacd_inv_emodulation(
                *qmf_real.offset(i as isize),
                qmf_bank,
                (*sbr_tables_ptr).qmf_dec_tables_ptr,
            );
            let mut temp_out_scale_fac: WORD32 = out_scale_factor + 1 as WORD32;
            if audio_object_type == AOT_ER_AAC_LD as core::ffi::c_int
                || audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int
            {
                temp_out_scale_fac = (temp_out_scale_fac as core::ffi::c_int
                    - 1 as core::ffi::c_int) as WORD32;
                ixheaacd_shiftrountine_with_rnd_eld(
                    *qmf_real.offset(i as isize),
                    t_qmf_imag,
                    &mut *filter_states.offset(ixheaacd_drc_offset as isize),
                    no_synthesis_channels,
                    temp_out_scale_fac,
                );
            } else {
                ixheaacd_shiftrountine_with_rnd(
                    *qmf_real.offset(i as isize),
                    t_qmf_imag,
                    &mut *filter_states.offset(ixheaacd_drc_offset as isize),
                    no_synthesis_channels,
                    temp_out_scale_fac,
                );
            }
            if no_synthesis_channels == NO_SYNTHESIS_CHANNELS_DOWN_SAMPLED {
                let mut temp: WORD32 = 1 as WORD32;
                if audio_object_type == AOT_ER_AAC_LD as core::ffi::c_int
                    || audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int
                {
                    temp = 2 as core::ffi::c_int as WORD32;
                }
                ixheaacd_sbr_qmfsyn32_winadd(
                    fp1,
                    fp2,
                    filter_coeff,
                    &mut *time_out.offset((ch_fac * p1) as isize),
                    temp as FLAG,
                    ch_fac,
                );
                fp1 = fp1.offset(thirty2 as isize);
                fp2 = fp2.offset(-(thirty2 as isize));
                thirty2 = -thirty2;
                ixheaacd_drc_offset -= 64 as core::ffi::c_int;
                if ixheaacd_drc_offset < 0 as core::ffi::c_int {
                    ixheaacd_drc_offset += 640 as core::ffi::c_int;
                }
            } else {
                let mut temp_0: WORD32 = 1 as WORD32;
                if audio_object_type == AOT_ER_AAC_LD as core::ffi::c_int
                    || audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int
                {
                    temp_0 = 2 as core::ffi::c_int as WORD32;
                }
                ixheaacd_sbr_qmfsyn64_winadd(
                    fp1,
                    fp2,
                    filter_coeff,
                    &mut *time_out.offset((ch_fac * p1) as isize),
                    temp_0 as FLAG,
                    ch_fac,
                );
                fp1 = fp1.offset(sixty4 as isize);
                fp2 = fp2.offset(-(sixty4 as isize));
                sixty4 = -sixty4;
                ixheaacd_drc_offset -= 128 as core::ffi::c_int;
                if ixheaacd_drc_offset < 0 as core::ffi::c_int {
                    ixheaacd_drc_offset += 1280 as core::ffi::c_int;
                }
            }
            filter_coeff = filter_coeff.offset(64 as core::ffi::c_int as isize);
            if filter_coeff
                == ((*qmf_bank).p_filter).offset(640 as core::ffi::c_int as isize)
                    as *mut WORD16
            {
                filter_coeff = (*qmf_bank).p_filter as *mut WORD16;
            }
            p1 += no_synthesis_channels;
            if active != 0 {
                memcpy(
                    *qmf_real.offset(i as isize) as *mut core::ffi::c_void,
                    qmf_real_tmp as *const core::ffi::c_void,
                    ((2 as WORD32 * no_synthesis_channels) as size_t)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
                );
            }
            i += 1;
        }
    }
    if audio_object_type == AOT_ER_AAC_LD as core::ffi::c_int
        || audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int
    {
        (*qmf_bank).fp1_syn = fp1;
        (*qmf_bank).fp2_syn = fp2;
        (*qmf_bank).sixty4 = sixty4 as WORD16;
    }
    (*qmf_bank).filter_pos_syn = filter_coeff;
    (*qmf_bank).ixheaacd_drc_offset = ixheaacd_drc_offset as WORD16;
}
