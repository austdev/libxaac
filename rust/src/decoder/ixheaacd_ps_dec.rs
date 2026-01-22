extern "C" {
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn ixheaacd_scale_ps_states(
        ptr_ps_dec: *mut ia_ps_dec_struct,
        scale: WORD16,
    ) -> VOID;
    static mut ixheaacd_decorr_filter1: Option<
        unsafe extern "C" fn(
            *mut ia_ps_dec_struct,
            *mut ia_ps_tables_struct,
            *mut WORD16,
        ) -> VOID,
    >;
    static mut ixheaacd_decorr_filter2: Option<
        unsafe extern "C" fn(
            *mut ia_ps_dec_struct,
            *mut WORD32,
            *mut WORD32,
            *mut WORD32,
            *mut WORD32,
            *mut ia_ps_tables_struct,
            *mut WORD16,
        ) -> VOID,
    >;
    static mut ixheaacd_divide16_pos: Option<
        unsafe extern "C" fn(WORD32, WORD32) -> WORD32,
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
unsafe extern "C" fn ixheaac_max32(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut max_val: WORD32 = 0;
    max_val = if a > b { a } else { b };
    return max_val;
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
unsafe extern "C" fn ixheaac_mult16x16in32_shl(mut a: WORD16, mut b: WORD16) -> WORD32 {
    let mut product: WORD32 = 0;
    product = ixheaac_shl32(ixheaac_mult16x16in32(a, b), 1 as WORD);
    return product;
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
unsafe extern "C" fn ixheaac_pnorm32(mut a: WORD32) -> WORD {
    let mut norm_val: WORD = 0;
    if a == 0 as core::ffi::c_int {
        norm_val = 31 as core::ffi::c_int as WORD;
    } else {
        norm_val = 0 as core::ffi::c_int as WORD;
        while a < 0x40000000 as core::ffi::c_long as WORD32 {
            a <<= 1 as core::ffi::c_int;
            norm_val += 1;
        }
    }
    return norm_val;
}
#[inline]
unsafe extern "C" fn ixheaac_abs32_nrm(mut a: WORD32) -> WORD32 {
    let mut abs_val: WORD32 = 0;
    abs_val = a;
    if a < 0 as core::ffi::c_int {
        abs_val = !a;
    }
    return abs_val;
}
#[inline]
unsafe extern "C" fn ixheaac_abs32_sat(mut a: WORD32) -> WORD32 {
    let mut abs_val: WORD32 = 0;
    abs_val = a;
    if a == MIN_32 {
        abs_val = MAX_32;
    } else if a < 0 as core::ffi::c_int {
        abs_val = -a;
    }
    return abs_val;
}
#[inline]
unsafe extern "C" fn ixheaac_sat16(mut op1: WORD32) -> WORD16 {
    let mut var_out: WORD16 = 0;
    if op1 as core::ffi::c_long > 0x7fff as core::ffi::c_long {
        var_out = MAX_16;
    } else if op1 < 0xffff8000 as core::ffi::c_long as WORD32 {
        var_out = -(32768 as core::ffi::c_int) as WORD16;
    } else {
        var_out = op1 as WORD16;
    }
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_add16(mut op1: WORD16, mut op2: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = (op1 as core::ffi::c_int + op2 as core::ffi::c_int) as WORD16;
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_sub16(mut op1: WORD16, mut op2: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = (op1 as core::ffi::c_int - op2 as core::ffi::c_int) as WORD16;
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_sub16_sat(mut op1: WORD16, mut op2: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    let mut diff: WORD32 = 0;
    diff = (op1 as core::ffi::c_int - op2 as core::ffi::c_int) as WORD32;
    var_out = ixheaac_sat16(diff);
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_mult16_shl(mut op1: WORD16, mut op2: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = (op1 as WORD32 * op2 as WORD32 >> 15 as core::ffi::c_int) as WORD16;
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_shr16(mut op1: WORD16, mut shift: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = (op1 as core::ffi::c_int >> shift as core::ffi::c_int) as WORD16;
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_abs16(mut op1: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    if (op1 as core::ffi::c_int) < 0 as core::ffi::c_int {
        var_out = -(op1 as core::ffi::c_int) as WORD16;
    } else {
        var_out = op1;
    }
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_min16(mut op1: WORD16, mut op2: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = (if (op1 as core::ffi::c_int) < op2 as core::ffi::c_int {
        op1 as core::ffi::c_int
    } else {
        op2 as core::ffi::c_int
    }) as WORD16;
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
unsafe extern "C" fn ixheaac_shr32_arr(
    mut word32_arr: *mut WORD32,
    mut shift: WORD16,
    mut n: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < n {
        *word32_arr = ixheaac_shr32(*word32_arr, shift as WORD);
        word32_arr = word32_arr.offset(1);
        i += 1;
    }
}
#[inline]
unsafe extern "C" fn ixheaac_shl32_arr_sat(
    mut word32_arr: *mut WORD32,
    mut shift: WORD16,
    mut n: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < n {
        *word32_arr = ixheaac_shl32_sat(*word32_arr, shift as WORD);
        word32_arr = word32_arr.offset(1);
        i += 1;
    }
}
#[inline]
unsafe extern "C" fn ixheaac_shr16_arr(
    mut word16_arr: *mut WORD16,
    mut shift: WORD16,
    mut n: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < n {
        *word16_arr = ixheaac_shr16(*word16_arr, shift);
        word16_arr = word16_arr.offset(1);
        i += 1;
    }
}
#[inline]
unsafe extern "C" fn ixheaac_extract16l(mut var: WORD32) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = var as WORD16;
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_deposit16h_in32(mut var: WORD16) -> WORD32 {
    let mut var_out: WORD32 = 0;
    var_out = (var as WORD32) << 16 as core::ffi::c_int;
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_shr32_dir_sat_limit(mut a: WORD32, mut b: WORD) -> WORD32 {
    let mut out_val: WORD32 = 0;
    if b < 0 as core::ffi::c_int {
        out_val = ixheaac_shl32_sat(a, -b);
    } else {
        b = ixheaac_min32(b as WORD32, 31 as WORD32) as WORD;
        out_val = ixheaac_shr32(a, b);
    }
    return out_val;
}
pub const NO_SYNTHESIS_CHANNELS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const NUM_SER_AP_LINKS: core::ffi::c_int = 3 as core::ffi::c_int;
pub const SUBQMF_GROUPS: core::ffi::c_int = 10 as core::ffi::c_int;
pub const QMF_GROUPS: core::ffi::c_int = 12 as core::ffi::c_int;
pub const NO_IID_GROUPS: core::ffi::c_int = SUBQMF_GROUPS + QMF_GROUPS;
pub const NUM_IID_LEVELS: core::ffi::c_int = 7 as core::ffi::c_int;
pub const NUM_IID_LEVELS_FINE: core::ffi::c_int = 15 as core::ffi::c_int;
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
pub const PEAK_DECAYING_FACT: core::ffi::c_int = 0x620a as core::ffi::c_int;
pub const PSC_SQRT05F: core::ffi::c_int = 0x5a82 as core::ffi::c_int;
pub const NUM_OF_BINS: core::ffi::c_int = 20 as core::ffi::c_int;
#[inline]
unsafe extern "C" fn ixheaacd_shl16_saturate(
    mut op1: WORD16,
    mut shift: WORD16,
) -> WORD16 {
    let mut var_out: WORD16 = 0;
    let mut temp: WORD32 = 0;
    temp = (op1 as core::ffi::c_int) << shift as core::ffi::c_int;
    var_out = ixheaac_sat16(temp);
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaacd_shl16_arr_saturate(
    mut word16_arr: *mut WORD16,
    mut shift: WORD16,
    mut n: WORD,
) -> VOID {
    let mut i: WORD = 0;
    i = (n as core::ffi::c_int - 1 as core::ffi::c_int) as WORD;
    while i >= 0 as core::ffi::c_int {
        *word16_arr = ixheaacd_shl16_saturate(*word16_arr, shift);
        word16_arr = word16_arr.offset(1);
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_scale_short_vec_left(
    mut word16_arr: *mut WORD16,
    mut n: WORD32,
    mut shift: WORD16,
) -> VOID {
    ixheaacd_shl16_arr_saturate(word16_arr, shift, n as WORD);
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_scale_int_vec_left(
    mut word32_arr: *mut WORD32,
    mut n: WORD32,
    mut shift: WORD16,
) -> VOID {
    ixheaac_shl32_arr_sat(word32_arr, shift, n);
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_scale_int_vec_right(
    mut word32_arr: *mut WORD32,
    mut n: WORD32,
    mut shift: WORD16,
) -> VOID {
    ixheaac_shr32_arr(word32_arr, shift, n);
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_scale_short_vec_right(
    mut word16_arr: *mut WORD16,
    mut n: WORD32,
    mut shift: WORD16,
) -> VOID {
    ixheaac_shr16_arr(word16_arr, shift, n);
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_calc_max(
    mut array: *mut WORD16,
    mut size: WORD32,
) -> WORD32 {
    let mut n: WORD = 0;
    let mut max_val: WORD32 = 0 as WORD32;
    let mut temp1: WORD16 = 0;
    let mut temp2: WORD16 = 0;
    n = size as WORD;
    while n != 0 as core::ffi::c_int {
        let fresh10 = array;
        array = array.offset(1);
        temp1 = *fresh10;
        let fresh11 = array;
        array = array.offset(1);
        temp2 = *fresh11;
        max_val = max_val | ixheaac_abs32_nrm(temp1 as WORD32);
        max_val = max_val | ixheaac_abs32_nrm(temp2 as WORD32);
        n -= 1;
    }
    return max_val;
}
unsafe extern "C" fn ixheaacd_get_ps_scale(
    mut ptr_ps_dec: *mut ia_ps_dec_struct,
) -> WORD {
    let mut i: WORD = 0;
    let mut m: WORD = 0;
    let mut n: WORD = 0;
    let mut len: WORD = 0;
    let mut max_val: WORD32 = 0 as WORD32;
    let mut ptr_re: *mut WORD16 = 0 as *mut WORD16;
    let mut ptr_re_temp: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr_im: *mut WORD32 = 0 as *mut WORD32;
    m = 0 as core::ffi::c_int as WORD;
    while m < 2 as core::ffi::c_int {
        ptr_re = &mut *(*((*ptr_ps_dec).delay_buf_qmf_ap_re_im).offset(m as isize))
            .as_mut_ptr()
            .offset((2 as core::ffi::c_int * 3 as core::ffi::c_int) as isize)
            as *mut WORD16;
        max_val |= ixheaacd_calc_max(ptr_re, NUM_OF_QUAD_MIRROR_FILTER_ALL_PASS_CHNLS);
        m += 1;
    }
    ptr_re = (*ptr_ps_dec).delay_buf_qmf_ld_re_im as *mut WORD16;
    max_val |= ixheaacd_calc_max(ptr_re, HIGH_DEL * SMALL_DEL_STRT);
    ptr_re = (*ptr_ps_dec).delay_buf_qmf_sd_re_im as *mut WORD16;
    max_val
        |= ixheaacd_calc_max(
            ptr_re,
            SMALL_DEL
                * (NUM_OF_QUAD_MIRROR_FILTER_ICC_CHNLS
                    - (NUM_OF_QUAD_MIRROR_FILTER_ALL_PASS_CHNLS + SMALL_DEL_STRT)),
        );
    ptr_re = ((*ptr_ps_dec).delay_buf_qmf_sub_re_im).as_mut_ptr() as *mut WORD16;
    max_val |= ixheaacd_calc_max(ptr_re, 16 as WORD32 * DEL_ALL_PASS);
    i = 0 as core::ffi::c_int as WORD;
    while i < NUM_SER_AP_LINKS {
        m = 0 as core::ffi::c_int as WORD;
        while m < (*ptr_ps_dec).delay_sample_ser[i as usize] as core::ffi::c_int {
            ptr_re = &mut *(*(*((*ptr_ps_dec).delay_buf_qmf_ser_re_im)
                .offset(m as isize))
                .as_mut_ptr()
                .offset(i as isize))
                .as_mut_ptr()
                .offset((2 as core::ffi::c_int * 3 as core::ffi::c_int) as isize)
                as *mut WORD16;
            max_val
                |= ixheaacd_calc_max(ptr_re, NUM_OF_QUAD_MIRROR_FILTER_ALL_PASS_CHNLS);
            m += 1;
        }
        i += 1;
    }
    ptr_re = ((*ptr_ps_dec).delay_buf_qmf_sub_ser_re_im).as_mut_ptr() as *mut WORD16;
    max_val |= ixheaacd_calc_max(ptr_re, NUM_SER_AP_LINKS * 5 as WORD32 * 16 as WORD32);
    max_val = max_val << 16 as core::ffi::c_int;
    len = (*ptr_ps_dec).str_hybrid.ptr_qmf_buf as WORD;
    i = 0 as core::ffi::c_int as WORD;
    while i < NO_QMF_CHANNELS_IN_HYBRID {
        ptr_re_temp = &mut *(*((*ptr_ps_dec).str_hybrid.ptr_qmf_buf_re)
            .as_mut_ptr()
            .offset(i as isize))
            .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
        ptr_im = &mut *(*((*ptr_ps_dec).str_hybrid.ptr_qmf_buf_im)
            .as_mut_ptr()
            .offset(i as isize))
            .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
        n = len;
        while n != 0 as core::ffi::c_int {
            let fresh8 = ptr_re_temp;
            ptr_re_temp = ptr_re_temp.offset(1);
            let mut temp3: WORD32 = *fresh8;
            let fresh9 = ptr_im;
            ptr_im = ptr_im.offset(1);
            let mut temp4: WORD32 = *fresh9;
            max_val = max_val | ixheaac_abs32_nrm(temp3);
            max_val = max_val | ixheaac_abs32_nrm(temp4);
            n -= 1;
        }
        i += 1;
    }
    return ixheaac_pnorm32(max_val);
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_init_ps_scale(
    mut ptr_ps_dec: *mut ia_ps_dec_struct,
    mut sbr_scale_factor: *mut ia_sbr_scale_fact_struct,
) -> VOID {
    let mut reserve: WORD32 = 0;
    let mut change: WORD32 = 0;
    let mut temp: WORD16 = 0;
    reserve = ixheaacd_get_ps_scale(ptr_ps_dec) as WORD32;
    (*ptr_ps_dec).delay_buffer_scale = ((*ptr_ps_dec).delay_buffer_scale as WORD32
        + reserve) as WORD16;
    temp = ixheaac_min16((*sbr_scale_factor).lb_scale, (*sbr_scale_factor).ov_lb_scale);
    temp = ixheaac_min16(temp, (*sbr_scale_factor).hb_scale);
    temp = ixheaac_min16(temp, (*ptr_ps_dec).delay_buffer_scale);
    (*sbr_scale_factor).ps_scale = (temp as core::ffi::c_int - 1 as core::ffi::c_int)
        as WORD16;
    change = ((*sbr_scale_factor).ps_scale as core::ffi::c_int
        - (*ptr_ps_dec).delay_buffer_scale as core::ffi::c_int) as WORD32;
    change = change + reserve;
    ixheaacd_scale_ps_states(ptr_ps_dec, change as WORD16);
    (*ptr_ps_dec).delay_buffer_scale = (*sbr_scale_factor).ps_scale;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_divide16_pos_dec(
    mut op1: WORD32,
    mut op2: WORD32,
) -> WORD32 {
    let mut v: UWORD32 = 0;
    let mut u: UWORD32 = 0;
    let mut k: WORD = 0;
    let mut nrm: WORD = 0;
    nrm = ixheaac_norm32(op2);
    u = (op1 << nrm) as UWORD32;
    v = (op2 << nrm) as UWORD32;
    u = (u as core::ffi::c_uint & 0xffff0000 as core::ffi::c_uint) as UWORD32;
    v = (v as core::ffi::c_uint & 0xffff0000 as core::ffi::c_uint) as UWORD32;
    if u != 0 as UWORD32 {
        k = 16 as core::ffi::c_int as WORD;
        while k > 0 as core::ffi::c_int {
            if u >= v {
                u = (u.wrapping_sub(v) << 1 as core::ffi::c_int)
                    .wrapping_add(1 as UWORD32);
            } else {
                u = u << 1 as core::ffi::c_int;
            }
            k -= 1;
        }
    }
    return u as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_decorr_filter1_dec(
    mut ptr_ps_dec: *mut ia_ps_dec_struct,
    mut ps_tables_ptr: *mut ia_ps_tables_struct,
    mut transient_ratio: *mut WORD16,
) -> VOID {
    let mut sb: WORD = 0;
    let mut m: WORD = 0;
    let mut delay_buf_idx: WORD16 = 0;
    let mut p_delay_buf_sub_re_im: *mut WORD16 = 0 as *mut WORD16;
    let mut p_frac_delay_phase_fac_ser_re_im: *mut WORD16 = 0 as *mut WORD16;
    let mut p_frac_delay_phase_fac_ser_re_im1: *mut WORD16 = 0 as *mut WORD16;
    let mut p_frac_delay_phase_fac_re_im: *const WORD16 = 0 as *const WORD16;
    let mut p_delay_buf_ser_sub_re_im: *mut REVERB_BUFFERS_CH_RI = 0
        as *mut REVERB_BUFFERS_CH_RI;
    let mut p_left_real: *mut WORD32 = 0 as *mut WORD32;
    let mut p_left_imag: *mut WORD32 = 0 as *mut WORD32;
    let mut p_right_real: *mut WORD32 = 0 as *mut WORD32;
    let mut p_right_imag: *mut WORD32 = 0 as *mut WORD32;
    p_frac_delay_phase_fac_re_im = &mut *((*ps_tables_ptr)
        .frac_delay_phase_fac_qmf_sub_re_im)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut WORD16;
    p_delay_buf_ser_sub_re_im = &mut (*ptr_ps_dec).delay_buf_qmf_sub_ser_re_im;
    p_frac_delay_phase_fac_ser_re_im = &mut *(*((*ps_tables_ptr)
        .frac_delay_phase_fac_qmf_sub_ser_re_im)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize))
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut WORD16;
    p_left_real = (*ptr_ps_dec).ptr_hyb_left_re;
    p_left_imag = (*ptr_ps_dec).ptr_hyb_left_im;
    p_right_real = (*ptr_ps_dec).ptr_hyb_right_re;
    p_right_imag = (*ptr_ps_dec).ptr_hyb_right_im;
    delay_buf_idx = (*ptr_ps_dec).delay_buf_idx;
    p_delay_buf_sub_re_im = &mut *(*((*ptr_ps_dec).delay_buf_qmf_sub_re_im)
        .as_mut_ptr()
        .offset(delay_buf_idx as isize))
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut WORD16;
    sb = 0 as core::ffi::c_int as WORD;
    while sb < SUBQMF_GROUPS {
        let mut real_tmp: WORD16 = 0;
        let mut imag_tmp: WORD16 = 0;
        let mut real_tmp0: WORD16 = 0;
        let mut imag_tmp0: WORD16 = 0;
        let mut real_in: WORD16 = 0;
        let mut imag_in: WORD16 = 0;
        let mut bin: WORD16 = 0;
        real_tmp0 = *p_delay_buf_sub_re_im.offset(0 as core::ffi::c_int as isize);
        imag_tmp0 = *p_delay_buf_sub_re_im.offset(1 as core::ffi::c_int as isize);
        real_in = (ixheaac_sub32_sat(
            ixheaac_mult16x16in32(
                real_tmp0,
                *p_frac_delay_phase_fac_re_im.offset(0 as core::ffi::c_int as isize),
            ),
            ixheaac_mult16x16in32(
                imag_tmp0,
                *p_frac_delay_phase_fac_re_im.offset(1 as core::ffi::c_int as isize),
            ),
        ) >> 15 as core::ffi::c_int) as WORD16;
        imag_in = (ixheaac_add32_sat(
            ixheaac_mult16x16in32(
                real_tmp0,
                *p_frac_delay_phase_fac_re_im.offset(1 as core::ffi::c_int as isize),
            ),
            ixheaac_mult16x16in32(
                imag_tmp0,
                *p_frac_delay_phase_fac_re_im.offset(0 as core::ffi::c_int as isize),
            ),
        ) >> 15 as core::ffi::c_int) as WORD16;
        let fresh0 = p_delay_buf_sub_re_im;
        p_delay_buf_sub_re_im = p_delay_buf_sub_re_im.offset(1);
        *fresh0 = ixheaac_round16(*p_left_real.offset(sb as isize));
        let fresh1 = p_delay_buf_sub_re_im;
        p_delay_buf_sub_re_im = p_delay_buf_sub_re_im.offset(1);
        *fresh1 = ixheaac_round16(*p_left_imag.offset(sb as isize));
        p_frac_delay_phase_fac_re_im = p_frac_delay_phase_fac_re_im
            .offset(2 as core::ffi::c_int as isize);
        p_frac_delay_phase_fac_ser_re_im1 = p_frac_delay_phase_fac_ser_re_im;
        p_frac_delay_phase_fac_ser_re_im = p_frac_delay_phase_fac_ser_re_im
            .offset(2 as core::ffi::c_int as isize);
        m = 0 as core::ffi::c_int as WORD;
        while m < NUM_SER_AP_LINKS {
            let mut decay: WORD16 = 0;
            let mut delay_buf_idx_ser: WORD16 = 0;
            delay_buf_idx_ser = (*ptr_ps_dec).delay_buf_idx_ser[m as usize];
            real_tmp0 = (*p_delay_buf_ser_sub_re_im)[delay_buf_idx_ser
                as usize][m as usize][(2 as WORD * sb) as usize];
            imag_tmp0 = (*p_delay_buf_ser_sub_re_im)[delay_buf_idx_ser
                as usize][m
                as usize][(2 as core::ffi::c_int * sb as core::ffi::c_int
                + 1 as core::ffi::c_int) as usize];
            real_tmp = (ixheaac_sub32_sat(
                ixheaac_mult16x16in32(
                    real_tmp0,
                    *p_frac_delay_phase_fac_ser_re_im1
                        .offset(0 as core::ffi::c_int as isize),
                ),
                ixheaac_mult16x16in32(
                    imag_tmp0,
                    *p_frac_delay_phase_fac_ser_re_im1
                        .offset(1 as core::ffi::c_int as isize),
                ),
            ) >> 15 as core::ffi::c_int) as WORD16;
            imag_tmp = (ixheaac_add32_sat(
                ixheaac_mult16x16in32(
                    real_tmp0,
                    *p_frac_delay_phase_fac_ser_re_im1
                        .offset(1 as core::ffi::c_int as isize),
                ),
                ixheaac_mult16x16in32(
                    imag_tmp0,
                    *p_frac_delay_phase_fac_ser_re_im1
                        .offset(0 as core::ffi::c_int as isize),
                ),
            ) >> 15 as core::ffi::c_int) as WORD16;
            decay = (*ps_tables_ptr).rev_link_decay_ser[m as usize];
            real_tmp = ixheaac_sub16(real_tmp, ixheaac_mult16_shl(real_in, decay));
            imag_tmp = ixheaac_sub16(imag_tmp, ixheaac_mult16_shl(imag_in, decay));
            (*p_delay_buf_ser_sub_re_im)[delay_buf_idx_ser
                as usize][m
                as usize][(sb as core::ffi::c_int * 2 as core::ffi::c_int) as usize] = ixheaac_add16(
                real_in,
                ixheaac_mult16_shl(real_tmp, decay),
            );
            (*p_delay_buf_ser_sub_re_im)[delay_buf_idx_ser
                as usize][m
                as usize][(sb as core::ffi::c_int * 2 as core::ffi::c_int
                + 1 as core::ffi::c_int) as usize] = ixheaac_add16(
                imag_in,
                ixheaac_mult16_shl(imag_tmp, decay),
            );
            real_in = real_tmp;
            imag_in = imag_tmp;
            p_frac_delay_phase_fac_ser_re_im1 = p_frac_delay_phase_fac_ser_re_im1
                .offset(32 as core::ffi::c_int as isize);
            m += 1;
        }
        bin = (*ps_tables_ptr).hybrid_to_bin[sb as usize];
        *p_right_real.offset(sb as isize) = ixheaac_mult16x16in32_shl(
            real_in,
            *transient_ratio.offset(bin as isize),
        );
        *p_right_imag.offset(sb as isize) = ixheaac_mult16x16in32_shl(
            imag_in,
            *transient_ratio.offset(bin as isize),
        );
        sb += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_decorr_filter2_dec(
    mut ptr_ps_dec: *mut ia_ps_dec_struct,
    mut p_buf_left_real: *mut WORD32,
    mut p_buf_left_imag: *mut WORD32,
    mut p_buf_right_real: *mut WORD32,
    mut p_buf_right_imag: *mut WORD32,
    mut ps_tables_ptr: *mut ia_ps_tables_struct,
    mut transient_ratio: *mut WORD16,
) -> VOID {
    let mut sb: WORD = 0;
    let mut di: WORD = 0;
    let mut sb_delay: WORD = 0;
    let mut m: WORD = 0;
    let mut bin: WORD = 0;
    let mut p_left_real: *mut WORD32 = 0 as *mut WORD32;
    let mut p_left_imag: *mut WORD32 = 0 as *mut WORD32;
    let mut p_right_real: *mut WORD32 = 0 as *mut WORD32;
    let mut p_right_imag: *mut WORD32 = 0 as *mut WORD32;
    let mut delay_buf_idx: WORD16 = 0;
    let mut p_delay_buf_ser_re_im: *mut REVERB_BUFFERS_RI = 0 as *mut REVERB_BUFFERS_RI;
    let mut p_delay_buf_ap_re_im: *mut WORD16 = 0 as *mut WORD16;
    let mut p_frac_delay_phase_fac_re_im: *const WORD16 = 0 as *const WORD16;
    let mut p_frac_delay_phase_fac_ser_ap_re_im: *mut WORD16 = 0 as *mut WORD16;
    let mut p_frac_delay_phase_fac_ser_ap_re_im_temp: *mut WORD16 = 0 as *mut WORD16;
    p_left_real = p_buf_left_real;
    p_left_imag = p_buf_left_imag;
    p_right_real = p_buf_right_real;
    p_right_imag = p_buf_right_imag;
    p_delay_buf_ser_re_im = &mut (*ptr_ps_dec).delay_buf_qmf_ser_re_im;
    p_frac_delay_phase_fac_re_im = ((*ps_tables_ptr).frac_delay_phase_fac_qmf_re_im)
        .as_mut_ptr();
    p_frac_delay_phase_fac_ser_ap_re_im = &mut *(*((*ps_tables_ptr)
        .frac_delay_phase_fac_qmf_ser_re_im)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize))
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut WORD16;
    delay_buf_idx = (*ptr_ps_dec).delay_buf_idx;
    p_delay_buf_ap_re_im = &mut *(*((*ptr_ps_dec).delay_buf_qmf_ap_re_im)
        .offset(delay_buf_idx as isize))
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut WORD16;
    p_frac_delay_phase_fac_re_im = p_frac_delay_phase_fac_re_im
        .offset(6 as core::ffi::c_int as isize);
    p_frac_delay_phase_fac_ser_ap_re_im = p_frac_delay_phase_fac_ser_ap_re_im
        .offset(6 as core::ffi::c_int as isize);
    p_delay_buf_ap_re_im = p_delay_buf_ap_re_im.offset(6 as core::ffi::c_int as isize);
    sb = 3 as core::ffi::c_int as WORD;
    di = 9 as core::ffi::c_int as WORD;
    while sb < 23 as core::ffi::c_int {
        let mut real_tmp: WORD16 = 0;
        let mut imag_tmp: WORD16 = 0;
        let mut real_tmp0: WORD16 = 0;
        let mut imag_tmp0: WORD16 = 0;
        let mut real_in: WORD16 = 0;
        let mut imag_in: WORD16 = 0;
        sb_delay = sb;
        real_tmp0 = *p_delay_buf_ap_re_im.offset(0 as core::ffi::c_int as isize);
        imag_tmp0 = *p_delay_buf_ap_re_im.offset(1 as core::ffi::c_int as isize);
        real_in = (ixheaac_sub32_sat(
            ixheaac_mult16x16in32(
                real_tmp0,
                *p_frac_delay_phase_fac_re_im.offset(0 as core::ffi::c_int as isize),
            ),
            ixheaac_mult16x16in32(
                imag_tmp0,
                *p_frac_delay_phase_fac_re_im.offset(1 as core::ffi::c_int as isize),
            ),
        ) >> 15 as core::ffi::c_int) as WORD16;
        imag_in = (ixheaac_add32_sat(
            ixheaac_mult16x16in32(
                real_tmp0,
                *p_frac_delay_phase_fac_re_im.offset(1 as core::ffi::c_int as isize),
            ),
            ixheaac_mult16x16in32(
                imag_tmp0,
                *p_frac_delay_phase_fac_re_im.offset(0 as core::ffi::c_int as isize),
            ),
        ) >> 15 as core::ffi::c_int) as WORD16;
        let fresh2 = p_delay_buf_ap_re_im;
        p_delay_buf_ap_re_im = p_delay_buf_ap_re_im.offset(1);
        *fresh2 = ixheaac_round16(*p_left_real.offset(sb as isize));
        let fresh3 = p_delay_buf_ap_re_im;
        p_delay_buf_ap_re_im = p_delay_buf_ap_re_im.offset(1);
        *fresh3 = ixheaac_round16(*p_left_imag.offset(sb as isize));
        p_frac_delay_phase_fac_re_im = p_frac_delay_phase_fac_re_im
            .offset(2 as core::ffi::c_int as isize);
        p_frac_delay_phase_fac_ser_ap_re_im_temp = p_frac_delay_phase_fac_ser_ap_re_im;
        p_frac_delay_phase_fac_ser_ap_re_im = p_frac_delay_phase_fac_ser_ap_re_im
            .offset(2 as core::ffi::c_int as isize);
        m = 0 as core::ffi::c_int as WORD;
        while m < NUM_SER_AP_LINKS {
            let mut decay: WORD16 = 0;
            let mut delay_buf_idx_ser: WORD16 = 0;
            delay_buf_idx_ser = (*ptr_ps_dec).delay_buf_idx_ser[m as usize];
            real_tmp0 = (*(*p_delay_buf_ser_re_im)
                .offset(
                    delay_buf_idx_ser as isize,
                ))[m
                as usize][(sb_delay as core::ffi::c_int * 2 as core::ffi::c_int)
                as usize];
            imag_tmp0 = (*(*p_delay_buf_ser_re_im)
                .offset(
                    delay_buf_idx_ser as isize,
                ))[m
                as usize][(sb_delay as core::ffi::c_int * 2 as core::ffi::c_int
                + 1 as core::ffi::c_int) as usize];
            real_tmp = (ixheaac_sub32_sat(
                ixheaac_mult16x16in32(
                    real_tmp0,
                    *p_frac_delay_phase_fac_ser_ap_re_im_temp
                        .offset(0 as core::ffi::c_int as isize),
                ),
                ixheaac_mult16x16in32(
                    imag_tmp0,
                    *p_frac_delay_phase_fac_ser_ap_re_im_temp
                        .offset(1 as core::ffi::c_int as isize),
                ),
            ) >> 15 as core::ffi::c_int) as WORD16;
            imag_tmp = (ixheaac_add32_sat(
                ixheaac_mult16x16in32(
                    real_tmp0,
                    *p_frac_delay_phase_fac_ser_ap_re_im_temp
                        .offset(1 as core::ffi::c_int as isize),
                ),
                ixheaac_mult16x16in32(
                    imag_tmp0,
                    *p_frac_delay_phase_fac_ser_ap_re_im_temp
                        .offset(0 as core::ffi::c_int as isize),
                ),
            ) >> 15 as core::ffi::c_int) as WORD16;
            decay = (*ps_tables_ptr).decay_scale_factor[di as usize];
            real_tmp = ixheaac_sub16(real_tmp, ixheaac_mult16_shl(real_in, decay));
            imag_tmp = ixheaac_sub16(imag_tmp, ixheaac_mult16_shl(imag_in, decay));
            (*(*p_delay_buf_ser_re_im)
                .offset(
                    delay_buf_idx_ser as isize,
                ))[m
                as usize][(sb_delay as core::ffi::c_int * 2 as core::ffi::c_int)
                as usize] = ixheaac_add16(real_in, ixheaac_mult16_shl(real_tmp, decay));
            (*(*p_delay_buf_ser_re_im)
                .offset(
                    delay_buf_idx_ser as isize,
                ))[m
                as usize][(sb_delay as core::ffi::c_int * 2 as core::ffi::c_int
                + 1 as core::ffi::c_int) as usize] = ixheaac_add16(
                imag_in,
                ixheaac_mult16_shl(imag_tmp, decay),
            );
            real_in = real_tmp;
            imag_in = imag_tmp;
            p_frac_delay_phase_fac_ser_ap_re_im_temp = p_frac_delay_phase_fac_ser_ap_re_im_temp
                .offset(64 as core::ffi::c_int as isize);
            m += 1;
            di += 1;
        }
        bin = (*ps_tables_ptr).delay_to_bin[sb_delay as usize] as WORD;
        *p_right_real.offset(sb as isize) = ixheaac_mult16x16in32_shl(
            real_in,
            *transient_ratio.offset(bin as isize),
        );
        *p_right_imag.offset(sb as isize) = ixheaac_mult16x16in32_shl(
            imag_in,
            *transient_ratio.offset(bin as isize),
        );
        sb += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_decorrelation_dec(
    mut ptr_ps_dec: *mut ia_ps_dec_struct,
    mut p_buf_left_real: *mut WORD32,
    mut p_buf_left_imag: *mut WORD32,
    mut p_buf_right_real: *mut WORD32,
    mut p_buf_right_imag: *mut WORD32,
    mut ps_tables_ptr: *mut ia_ps_tables_struct,
) -> VOID {
    let mut sb: WORD = 0;
    let mut gr: WORD = 0;
    let mut bin: WORD = 0;
    let mut sband: WORD = 0;
    let mut maxsband: WORD = 0;
    let mut peak_diff: WORD32 = 0;
    let mut nrg: WORD32 = 0;
    let mut power_buf: [WORD32; 20] = [0; 20];
    let mut transient_ratio: [WORD16; 21] = [0; 21];
    let mut p_left_real: *mut WORD32 = 0 as *mut WORD32;
    let mut p_left_imag: *mut WORD32 = 0 as *mut WORD32;
    let mut p_right_real: *mut WORD32 = 0 as *mut WORD32;
    let mut p_right_imag: *mut WORD32 = 0 as *mut WORD32;
    let mut p_delay_buf_re_im_ld: *mut WORD16 = 0 as *mut WORD16;
    let mut p_delay_buf_re_im_sd: *mut WORD16 = 0 as *mut WORD16;
    let mut usb: WORD = (*ptr_ps_dec).usb as WORD;
    let mut delay_buf_idx: WORD16 = 0;
    p_left_real = (*ptr_ps_dec).ptr_hyb_left_re;
    p_left_imag = (*ptr_ps_dec).ptr_hyb_left_im;
    p_right_real = (*ptr_ps_dec).ptr_hyb_right_re;
    p_right_imag = (*ptr_ps_dec).ptr_hyb_right_im;
    let mut re0: WORD32 = 0;
    let mut im0: WORD32 = 0;
    let mut re1: WORD32 = 0;
    let mut im1: WORD32 = 0;
    re0 = *p_left_real.offset(0 as core::ffi::c_int as isize);
    im0 = *p_left_imag.offset(0 as core::ffi::c_int as isize);
    re1 = *p_left_real.offset(5 as core::ffi::c_int as isize);
    im1 = *p_left_imag.offset(5 as core::ffi::c_int as isize);
    power_buf[0 as core::ffi::c_int as usize] = ixheaac_mult32x16in32(
        re0,
        (re0 >> 16 as core::ffi::c_int) as WORD16,
    );
    power_buf[0 as core::ffi::c_int as usize] = ixheaac_add32_sat(
        power_buf[0 as core::ffi::c_int as usize],
        ixheaac_mult32x16in32(im0, (im0 >> 16 as core::ffi::c_int) as WORD16),
    );
    power_buf[0 as core::ffi::c_int as usize] = ixheaac_add32_sat(
        power_buf[0 as core::ffi::c_int as usize],
        ixheaac_mult32x16in32(re1, (re1 >> 16 as core::ffi::c_int) as WORD16),
    );
    power_buf[0 as core::ffi::c_int as usize] = ixheaac_add32_sat(
        power_buf[0 as core::ffi::c_int as usize],
        ixheaac_mult32x16in32(im1, (im1 >> 16 as core::ffi::c_int) as WORD16),
    );
    re0 = *p_left_real.offset(4 as core::ffi::c_int as isize);
    im0 = *p_left_imag.offset(4 as core::ffi::c_int as isize);
    re1 = *p_left_real.offset(1 as core::ffi::c_int as isize);
    im1 = *p_left_imag.offset(1 as core::ffi::c_int as isize);
    power_buf[1 as core::ffi::c_int as usize] = ixheaac_mult32x16in32(
        re0,
        (re0 >> 16 as core::ffi::c_int) as WORD16,
    );
    power_buf[1 as core::ffi::c_int as usize] = ixheaac_add32_sat(
        power_buf[1 as core::ffi::c_int as usize],
        ixheaac_mult32x16in32(im0, (im0 >> 16 as core::ffi::c_int) as WORD16),
    );
    power_buf[1 as core::ffi::c_int as usize] = ixheaac_add32_sat(
        power_buf[1 as core::ffi::c_int as usize],
        ixheaac_mult32x16in32(re1, (re1 >> 16 as core::ffi::c_int) as WORD16),
    );
    power_buf[1 as core::ffi::c_int as usize] = ixheaac_add32_sat(
        power_buf[1 as core::ffi::c_int as usize],
        ixheaac_mult32x16in32(im1, (im1 >> 16 as core::ffi::c_int) as WORD16),
    );
    bin = (4 as core::ffi::c_int - 2 as core::ffi::c_int) as WORD;
    gr = 4 as core::ffi::c_int as WORD;
    while gr < SUBQMF_GROUPS {
        let mut re: WORD32 = 0;
        let mut im: WORD32 = 0;
        sb = (*ps_tables_ptr).borders_group[gr as usize] as WORD;
        re = *p_left_real.offset(sb as isize);
        im = *p_left_imag.offset(sb as isize);
        power_buf[bin as usize] = ixheaac_mult32x16in32(
            re,
            (re >> 16 as core::ffi::c_int) as WORD16,
        );
        power_buf[bin as usize] = ixheaac_add32_sat(
            power_buf[bin as usize],
            ixheaac_mult32x16in32(im, (im >> 16 as core::ffi::c_int) as WORD16),
        );
        bin += 1;
        gr += 1;
    }
    p_left_real = p_buf_left_real;
    p_left_imag = p_buf_left_imag;
    bin = (NO_QMF_CHANNELS_IN_HYBRID + 5 as core::ffi::c_int) as WORD;
    sband = NO_QMF_CHANNELS_IN_HYBRID as WORD;
    while sband < NO_QMF_CHANNELS_IN_HYBRID + 6 as core::ffi::c_int {
        let mut re_0: WORD32 = *p_left_real.offset(sband as isize);
        let mut im_0: WORD32 = *p_left_imag.offset(sband as isize);
        power_buf[bin as usize] = ixheaac_mult32x16in32(
            re_0,
            (re_0 >> 16 as core::ffi::c_int) as WORD16,
        );
        power_buf[bin as usize] = ixheaac_add32_sat(
            power_buf[bin as usize],
            ixheaac_mult32x16in32(im_0, (im_0 >> 16 as core::ffi::c_int) as WORD16),
        );
        bin += 1;
        sband += 1;
    }
    bin = (16 as core::ffi::c_int - 2 as core::ffi::c_int) as WORD;
    gr = 16 as core::ffi::c_int as WORD;
    while gr < NO_IID_GROUPS {
        let mut accu: WORD32 = 0 as WORD32;
        let mut tmp: WORD32 = 0;
        let mut re_1: WORD32 = 0;
        let mut im_1: WORD32 = 0;
        maxsband = ixheaac_min32(
            usb as WORD32,
            (*ps_tables_ptr)
                .borders_group[(gr as core::ffi::c_int + 1 as core::ffi::c_int) as usize]
                as WORD32,
        ) as WORD;
        sband = (*ps_tables_ptr).borders_group[gr as usize] as WORD;
        while sband < maxsband {
            re_1 = *p_left_real.offset(sband as isize);
            im_1 = *p_left_imag.offset(sband as isize);
            tmp = ixheaac_mult32x16in32(
                re_1,
                (re_1 >> 16 as core::ffi::c_int) as WORD16,
            );
            tmp = ixheaac_add32_sat(
                tmp,
                ixheaac_mult32x16in32(im_1, (im_1 >> 16 as core::ffi::c_int) as WORD16),
            );
            tmp = tmp
                >> (*ps_tables_ptr)
                    .group_shift[(gr as core::ffi::c_int
                    - (SUBQMF_GROUPS + 6 as core::ffi::c_int)) as usize]
                    as core::ffi::c_int;
            accu = ixheaac_add32_sat(accu, tmp);
            sband += 1;
        }
        power_buf[bin as usize] = accu;
        bin += 1;
        gr += 1;
    }
    p_left_real = (*ptr_ps_dec).ptr_hyb_left_re;
    p_left_imag = (*ptr_ps_dec).ptr_hyb_left_im;
    bin = 0 as core::ffi::c_int as WORD;
    while bin < NUM_OF_BINS {
        power_buf[bin as usize] = ixheaac_shl32(power_buf[bin as usize], 1 as WORD);
        power_buf[bin as usize] = ixheaac_max32(0 as WORD32, power_buf[bin as usize]);
        *((*ptr_ps_dec).peak_decay_diff).offset(bin as isize) = ixheaac_mult32x16in32_shl(
            *((*ptr_ps_dec).peak_decay_diff).offset(bin as isize),
            PEAK_DECAYING_FACT as WORD16,
        );
        *((*ptr_ps_dec).peak_decay_diff).offset(bin as isize) = ixheaac_max32(
            *((*ptr_ps_dec).peak_decay_diff).offset(bin as isize),
            power_buf[bin as usize],
        );
        peak_diff = ixheaac_add32_sat(
            ixheaac_mult32x16in32_shl(
                *((*ptr_ps_dec).peak_decay_diff_prev).offset(bin as isize),
                0x6000 as WORD16,
            ),
            ixheaac_sub32_sat(
                *((*ptr_ps_dec).peak_decay_diff).offset(bin as isize),
                power_buf[bin as usize],
            ) >> 2 as core::ffi::c_int,
        );
        *((*ptr_ps_dec).peak_decay_diff_prev).offset(bin as isize) = peak_diff;
        nrg = ixheaac_add32_sat(
            ixheaac_mult32x16in32_shl(
                *((*ptr_ps_dec).energy_prev).offset(bin as isize),
                0x6000 as WORD16,
            ),
            power_buf[bin as usize] >> 2 as core::ffi::c_int,
        );
        *((*ptr_ps_dec).energy_prev).offset(bin as isize) = nrg;
        peak_diff = ixheaac_add32_sat(peak_diff, peak_diff >> 1 as core::ffi::c_int);
        if peak_diff <= nrg {
            transient_ratio[bin as usize] = 0x7fff as WORD16;
        } else {
            transient_ratio[bin as usize] = ixheaac_extract16l(
                (Some(ixheaacd_divide16_pos.expect("non-null function pointer")))
                    .expect("non-null function pointer")(nrg, peak_diff),
            );
        }
        bin += 1;
    }
    (Some(ixheaacd_decorr_filter1.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ptr_ps_dec, ps_tables_ptr, transient_ratio.as_mut_ptr());
    transient_ratio[20 as core::ffi::c_int as usize] = 0 as WORD16;
    (Some(ixheaacd_decorr_filter2.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        ptr_ps_dec,
        p_buf_left_real,
        p_buf_left_imag,
        p_buf_right_real,
        p_buf_right_imag,
        ps_tables_ptr,
        transient_ratio.as_mut_ptr(),
    );
    let mut trans_ratio: WORD16 = transient_ratio[18 as core::ffi::c_int as usize];
    p_left_real = p_buf_left_real;
    p_left_imag = p_buf_left_imag;
    p_right_real = p_buf_right_real;
    p_right_imag = p_buf_right_imag;
    maxsband = ixheaac_min32(
        usb as WORD16 as WORD32,
        (*ps_tables_ptr).borders_group[21 as core::ffi::c_int as usize] as WORD32,
    ) as WORD;
    delay_buf_idx = (*ptr_ps_dec).delay_buf_idx_long;
    p_delay_buf_re_im_ld = &mut *(*((*ptr_ps_dec).delay_buf_qmf_ld_re_im)
        .offset(delay_buf_idx as isize))
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut WORD16;
    sband = (*ps_tables_ptr).borders_group[20 as core::ffi::c_int as usize] as WORD;
    while sband < maxsband {
        let mut real_in: WORD16 = 0;
        let mut imag_in: WORD16 = 0;
        real_in = *p_delay_buf_re_im_ld.offset(0 as core::ffi::c_int as isize);
        imag_in = *p_delay_buf_re_im_ld.offset(1 as core::ffi::c_int as isize);
        let fresh4 = p_delay_buf_re_im_ld;
        p_delay_buf_re_im_ld = p_delay_buf_re_im_ld.offset(1);
        *fresh4 = ixheaac_round16(*p_left_real.offset(sband as isize));
        let fresh5 = p_delay_buf_re_im_ld;
        p_delay_buf_re_im_ld = p_delay_buf_re_im_ld.offset(1);
        *fresh5 = ixheaac_round16(*p_left_imag.offset(sband as isize));
        *p_right_real.offset(sband as isize) = ixheaac_mult16x16in32_shl(
            real_in,
            trans_ratio,
        );
        *p_right_imag.offset(sband as isize) = ixheaac_mult16x16in32_shl(
            imag_in,
            trans_ratio,
        );
        sband += 1;
    }
    (*ptr_ps_dec).delay_buf_idx_long = ixheaac_add16(
        (*ptr_ps_dec).delay_buf_idx_long,
        1 as WORD16,
    );
    if (*ptr_ps_dec).delay_buf_idx_long as core::ffi::c_int >= 14 as core::ffi::c_int {
        (*ptr_ps_dec).delay_buf_idx_long = 0 as WORD16;
    }
    p_delay_buf_re_im_sd = &mut *(*((*ptr_ps_dec).delay_buf_qmf_sd_re_im)
        .offset(0 as core::ffi::c_int as isize))
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut WORD16;
    trans_ratio = transient_ratio[19 as core::ffi::c_int as usize];
    maxsband = ixheaac_min32(
        usb as WORD16 as WORD32,
        (*ps_tables_ptr).borders_group[22 as core::ffi::c_int as usize] as WORD32,
    ) as WORD;
    sband = (*ps_tables_ptr).borders_group[21 as core::ffi::c_int as usize] as WORD;
    while sband < maxsband {
        let mut real_in_0: WORD16 = 0;
        let mut imag_in_0: WORD16 = 0;
        real_in_0 = *p_delay_buf_re_im_sd.offset(0 as core::ffi::c_int as isize);
        imag_in_0 = *p_delay_buf_re_im_sd.offset(1 as core::ffi::c_int as isize);
        let fresh6 = p_delay_buf_re_im_sd;
        p_delay_buf_re_im_sd = p_delay_buf_re_im_sd.offset(1);
        *fresh6 = ixheaac_round16(*p_left_real.offset(sband as isize));
        let fresh7 = p_delay_buf_re_im_sd;
        p_delay_buf_re_im_sd = p_delay_buf_re_im_sd.offset(1);
        *fresh7 = ixheaac_round16(*p_left_imag.offset(sband as isize));
        *p_right_real.offset(sband as isize) = ixheaac_mult16x16in32_shl(
            real_in_0,
            trans_ratio,
        );
        *p_right_imag.offset(sband as isize) = ixheaac_mult16x16in32_shl(
            imag_in_0,
            trans_ratio,
        );
        sband += 1;
    }
    sband = usb;
    while sband < NO_SYNTHESIS_CHANNELS {
        *p_right_real.offset(sband as isize) = 0 as core::ffi::c_int as WORD32;
        *p_right_imag.offset(sband as isize) = 0 as core::ffi::c_int as WORD32;
        sband += 1;
    }
    (*ptr_ps_dec).delay_buf_idx = ((*ptr_ps_dec).delay_buf_idx as core::ffi::c_int
        + 1 as core::ffi::c_int) as WORD16;
    if (*ptr_ps_dec).delay_buf_idx as core::ffi::c_int >= DEL_ALL_PASS {
        (*ptr_ps_dec).delay_buf_idx = 0 as WORD16;
    }
    let mut delay_m: WORD = 0;
    delay_m = 0 as core::ffi::c_int as WORD;
    while delay_m < NUM_SER_AP_LINKS {
        (*ptr_ps_dec).delay_buf_idx_ser[delay_m as usize] = ((*ptr_ps_dec)
            .delay_buf_idx_ser[delay_m as usize] as core::ffi::c_int
            + 1 as core::ffi::c_int) as WORD16;
        if (*ptr_ps_dec).delay_buf_idx_ser[delay_m as usize] as core::ffi::c_int
            >= (*ptr_ps_dec).delay_sample_ser[delay_m as usize] as core::ffi::c_int
        {
            (*ptr_ps_dec).delay_buf_idx_ser[delay_m as usize] = 0 as WORD16;
        }
        delay_m += 1;
    }
}
unsafe extern "C" fn ixheaacd_cos512(
    mut phi_by_4: WORD,
    mut cos_sin_lookup_tab: *const WORD16,
) -> WORD16 {
    let mut index: WORD = 0;
    index = ixheaac_round16(ixheaac_abs32_sat(phi_by_4 as WORD32)) as WORD;
    index = (index as core::ffi::c_int & 0x3ff as core::ffi::c_int) as WORD;
    if index < 512 as core::ffi::c_int {
        return *cos_sin_lookup_tab.offset((512 as WORD - index) as isize)
    } else {
        return -(*cos_sin_lookup_tab
            .offset((index as core::ffi::c_int - 512 as core::ffi::c_int) as isize)
            as core::ffi::c_int) as WORD16
    };
}
unsafe extern "C" fn ixheaacd_sin512(
    mut phi_by_4: WORD,
    mut cos_sin_lookup_tab: *const WORD16,
) -> WORD16 {
    let mut index: WORD = 0;
    index = ixheaac_round16(phi_by_4 as WORD32) as WORD;
    if index < 0 as core::ffi::c_int {
        index = (-(index as core::ffi::c_int) & 0x3ff as core::ffi::c_int) as WORD;
        if index < 512 as core::ffi::c_int {
            return -(*cos_sin_lookup_tab.offset(index as isize) as core::ffi::c_int)
                as WORD16
        } else {
            return -(*cos_sin_lookup_tab.offset((1024 as WORD - index) as isize)
                as core::ffi::c_int) as WORD16
        }
    } else {
        index = (index as core::ffi::c_int & 0x3ff as core::ffi::c_int) as WORD;
        if index < 512 as core::ffi::c_int {
            return *cos_sin_lookup_tab.offset(index as isize)
        } else {
            return *cos_sin_lookup_tab.offset((1024 as WORD - index) as isize)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_init_rot_env(
    mut ptr_ps_dec: *mut ia_ps_dec_struct,
    mut env: WORD16,
    mut usb: WORD16,
    mut sbr_tables_ptr: *mut ia_sbr_tables_struct,
    mut cos_sin_lookup_tab: *const WORD16,
) -> VOID {
    let mut group: WORD = 0;
    let mut bin: WORD = 0;
    let mut num_iid_steps: WORD = 0;
    let mut c2: WORD16 = 0;
    let mut c1: WORD16 = 0;
    let mut alpha: WORD32 = 0;
    let mut beta: WORD32 = 0;
    let mut h11: WORD16 = 0;
    let mut h12: WORD16 = 0;
    let mut h21: WORD16 = 0;
    let mut h22: WORD16 = 0;
    let mut inv_env_len: WORD16 = 0;
    let mut p_scale_factors: *const WORD16 = 0 as *const WORD16;
    let mut p_iid_idx: *mut WORD16 = 0 as *mut WORD16;
    let mut indexplusa: WORD = 0;
    let mut indexminusa: WORD = 0;
    let rescale: WORD32 = (0x517cc1b as WORD32) << 1 as core::ffi::c_int;
    if env as core::ffi::c_int == 0 as core::ffi::c_int {
        let mut usb_prev: WORD = (*ptr_ps_dec).usb as WORD;
        let mut ptr_tmp: *mut WORD16 = 0 as *mut WORD16;
        (*ptr_ps_dec).usb = usb;
        if usb as core::ffi::c_int > usb_prev && usb_prev != 0 {
            let mut i: WORD = 0;
            let mut j: WORD = 0;
            let mut delay: WORD = 0;
            let mut offset1: WORD = 0;
            let mut ixheaacd_drc_offset: WORD = if (usb as core::ffi::c_int)
                < NUM_OF_QUAD_MIRROR_FILTER_ALL_PASS_CHNLS
            {
                usb as WORD
            } else {
                NUM_OF_QUAD_MIRROR_FILTER_ALL_PASS_CHNLS
            };
            if ixheaacd_drc_offset > usb_prev {
                i = 0 as core::ffi::c_int as WORD;
                while i < NUM_SER_AP_LINKS {
                    j = 0 as core::ffi::c_int as WORD;
                    while j
                        < (*ptr_ps_dec).delay_sample_ser[i as usize] as core::ffi::c_int
                    {
                        ptr_tmp = &mut *(*(*((*ptr_ps_dec).delay_buf_qmf_ser_re_im)
                            .offset(j as isize))
                            .as_mut_ptr()
                            .offset(i as isize))
                            .as_mut_ptr()
                            .offset(
                                (usb_prev as core::ffi::c_int * 2 as core::ffi::c_int)
                                    as isize,
                            ) as *mut WORD16;
                        memset(
                            ptr_tmp as *mut core::ffi::c_void,
                            0 as core::ffi::c_int,
                            (::core::mem::size_of::<WORD16>() as size_t)
                                .wrapping_mul((ixheaacd_drc_offset - usb_prev) as size_t)
                                .wrapping_mul(2 as size_t),
                        );
                        j += 1;
                    }
                    i += 1;
                }
            }
            offset1 = (if (usb as core::ffi::c_int)
                < NUM_OF_QUAD_MIRROR_FILTER_ALL_PASS_CHNLS + SMALL_DEL_STRT
            {
                usb as core::ffi::c_int
            } else {
                NUM_OF_QUAD_MIRROR_FILTER_ALL_PASS_CHNLS + SMALL_DEL_STRT
            }) as WORD;
            delay = HIGH_DEL as WORD;
            if offset1 >= ixheaacd_drc_offset && offset1 <= SMALL_DEL_STRT {
                i = 0 as core::ffi::c_int as WORD;
                while i < delay {
                    ptr_tmp = &mut *(*((*ptr_ps_dec).delay_buf_qmf_ld_re_im)
                        .offset(i as isize))
                        .as_mut_ptr()
                        .offset(
                            (ixheaacd_drc_offset as core::ffi::c_int
                                * 2 as core::ffi::c_int) as isize,
                        ) as *mut WORD16;
                    memset(
                        ptr_tmp as *mut core::ffi::c_void,
                        0 as core::ffi::c_int,
                        (::core::mem::size_of::<WORD16>() as size_t)
                            .wrapping_mul(2 as size_t)
                            .wrapping_mul((offset1 - ixheaacd_drc_offset) as size_t),
                    );
                    i += 1;
                }
            }
            delay = SMALL_DEL as WORD;
            if usb as core::ffi::c_int >= offset1
                && usb as core::ffi::c_int <= 16 as core::ffi::c_int
            {
                i = 0 as core::ffi::c_int as WORD;
                while i < delay {
                    ptr_tmp = &mut *(*((*ptr_ps_dec).delay_buf_qmf_sd_re_im)
                        .offset(i as isize))
                        .as_mut_ptr()
                        .offset(
                            (offset1 as core::ffi::c_int * 2 as core::ffi::c_int)
                                as isize,
                        ) as *mut WORD16;
                    memset(
                        ptr_tmp as *mut core::ffi::c_void,
                        0 as core::ffi::c_int,
                        (::core::mem::size_of::<WORD16>() as size_t)
                            .wrapping_mul(2 as size_t)
                            .wrapping_mul((usb as WORD - offset1) as size_t),
                    );
                    i += 1;
                }
            }
        }
    }
    if (*ptr_ps_dec).iid_quant != 0 {
        num_iid_steps = NUM_IID_LEVELS_FINE as WORD;
        p_scale_factors = ((*(*sbr_tables_ptr).ps_tables_ptr).scale_factors_fine)
            .as_mut_ptr();
    } else {
        num_iid_steps = NUM_IID_LEVELS as WORD;
        p_scale_factors = ((*(*sbr_tables_ptr).ps_tables_ptr).scale_factors)
            .as_mut_ptr();
    }
    inv_env_len = (*(*sbr_tables_ptr).env_calc_tables_ptr)
        .sbr_inv_int_table[ixheaac_abs16(
        ixheaac_sub16_sat(
            (*ptr_ps_dec)
                .border_position[(env as core::ffi::c_int + 1 as core::ffi::c_int)
                as usize],
            (*ptr_ps_dec).border_position[env as usize],
        ),
    ) as usize];
    p_iid_idx = &mut *(*((*ptr_ps_dec).iid_par_table).as_mut_ptr().offset(env as isize))
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut WORD16;
    group = 0 as core::ffi::c_int as WORD;
    while group < NO_IID_GROUPS {
        let mut bplusa: WORD16 = 0;
        let mut bminusa: WORD16 = 0;
        let mut num_iid_idx: WORD = 0;
        let mut num_icc_idx: WORD = 0;
        bin = (*(*sbr_tables_ptr).ps_tables_ptr).group_to_bin[group as usize] as WORD;
        num_iid_idx = *p_iid_idx.offset(bin as isize) as WORD;
        num_icc_idx = *p_iid_idx
            .offset((bin as core::ffi::c_int + 238 as core::ffi::c_int) as isize)
            as WORD;
        c1 = *p_scale_factors.offset((num_iid_steps + num_iid_idx) as isize);
        c2 = *p_scale_factors.offset((num_iid_steps - num_iid_idx) as isize);
        beta = ixheaac_mult32x16in32_shl(
            ixheaac_mult16x16in32_shl(
                (*(*sbr_tables_ptr).ps_tables_ptr).alpha_values[num_icc_idx as usize],
                ixheaac_sub16(c1, c2),
            ),
            PSC_SQRT05F as WORD16,
        );
        alpha = ixheaac_shr32_dir_sat_limit(
            ixheaac_deposit16h_in32(
                (*(*sbr_tables_ptr).ps_tables_ptr).alpha_values[num_icc_idx as usize],
            ),
            1 as WORD,
        );
        bplusa = ixheaac_round16(ixheaac_add32_sat(beta, alpha));
        bminusa = ixheaac_round16(ixheaac_sub32_sat(beta, alpha));
        indexplusa = ixheaac_mult32x16in32(rescale, bplusa) as WORD;
        indexminusa = ixheaac_mult32x16in32(rescale, bminusa) as WORD;
        h11 = ixheaac_mult16_shl(ixheaacd_cos512(indexplusa, cos_sin_lookup_tab), c2);
        h12 = ixheaac_mult16_shl(ixheaacd_cos512(indexminusa, cos_sin_lookup_tab), c1);
        h21 = ixheaac_mult16_shl(ixheaacd_sin512(indexplusa, cos_sin_lookup_tab), c2);
        h22 = ixheaac_mult16_shl(ixheaacd_sin512(indexminusa, cos_sin_lookup_tab), c1);
        (*ptr_ps_dec)
            .delta_h11_h12[(2 as core::ffi::c_int * group as core::ffi::c_int
            + 0 as core::ffi::c_int) as usize] = ixheaac_mult16_shl(
            inv_env_len,
            ixheaac_sub16(
                h11,
                (*ptr_ps_dec)
                    .h11_h12_vec[(2 as core::ffi::c_int * group as core::ffi::c_int
                    + 0 as core::ffi::c_int) as usize],
            ),
        );
        (*ptr_ps_dec)
            .delta_h11_h12[(2 as core::ffi::c_int * group as core::ffi::c_int
            + 1 as core::ffi::c_int) as usize] = ixheaac_mult16_shl(
            inv_env_len,
            ixheaac_sub16(
                h12,
                (*ptr_ps_dec)
                    .h11_h12_vec[(2 as core::ffi::c_int * group as core::ffi::c_int
                    + 1 as core::ffi::c_int) as usize],
            ),
        );
        (*ptr_ps_dec)
            .delta_h21_h22[(2 as core::ffi::c_int * group as core::ffi::c_int
            + 0 as core::ffi::c_int) as usize] = ixheaac_mult16_shl(
            inv_env_len,
            ixheaac_sub16(
                h21,
                (*ptr_ps_dec)
                    .h21_h22_vec[(2 as core::ffi::c_int * group as core::ffi::c_int
                    + 0 as core::ffi::c_int) as usize],
            ),
        );
        (*ptr_ps_dec)
            .delta_h21_h22[(2 as core::ffi::c_int * group as core::ffi::c_int
            + 1 as core::ffi::c_int) as usize] = ixheaac_mult16_shl(
            inv_env_len,
            ixheaac_sub16(
                h22,
                (*ptr_ps_dec)
                    .h21_h22_vec[(2 as core::ffi::c_int * group as core::ffi::c_int
                    + 1 as core::ffi::c_int) as usize],
            ),
        );
        (*ptr_ps_dec)
            .H11_H12[(2 as core::ffi::c_int * group as core::ffi::c_int
            + 0 as core::ffi::c_int) as usize] = (*ptr_ps_dec)
            .h11_h12_vec[(2 as core::ffi::c_int * group as core::ffi::c_int
            + 0 as core::ffi::c_int) as usize];
        (*ptr_ps_dec)
            .H11_H12[(2 as core::ffi::c_int * group as core::ffi::c_int
            + 1 as core::ffi::c_int) as usize] = (*ptr_ps_dec)
            .h11_h12_vec[(2 as core::ffi::c_int * group as core::ffi::c_int
            + 1 as core::ffi::c_int) as usize];
        (*ptr_ps_dec)
            .H21_H22[(2 as core::ffi::c_int * group as core::ffi::c_int
            + 0 as core::ffi::c_int) as usize] = (*ptr_ps_dec)
            .h21_h22_vec[(2 as core::ffi::c_int * group as core::ffi::c_int
            + 0 as core::ffi::c_int) as usize];
        (*ptr_ps_dec)
            .H21_H22[(2 as core::ffi::c_int * group as core::ffi::c_int
            + 1 as core::ffi::c_int) as usize] = (*ptr_ps_dec)
            .h21_h22_vec[(2 as core::ffi::c_int * group as core::ffi::c_int
            + 1 as core::ffi::c_int) as usize];
        (*ptr_ps_dec)
            .h11_h12_vec[(2 as core::ffi::c_int * group as core::ffi::c_int
            + 0 as core::ffi::c_int) as usize] = h11;
        (*ptr_ps_dec)
            .h11_h12_vec[(2 as core::ffi::c_int * group as core::ffi::c_int
            + 1 as core::ffi::c_int) as usize] = h12;
        (*ptr_ps_dec)
            .h21_h22_vec[(2 as core::ffi::c_int * group as core::ffi::c_int
            + 0 as core::ffi::c_int) as usize] = h21;
        (*ptr_ps_dec)
            .h21_h22_vec[(2 as core::ffi::c_int * group as core::ffi::c_int
            + 1 as core::ffi::c_int) as usize] = h22;
        group += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_apply_rot_dec(
    mut ptr_ps_dec: *mut ia_ps_dec_struct,
    mut p_qmf_left_re: *mut WORD32,
    mut p_qmf_left_im: *mut WORD32,
    mut p_qmf_right_re: *mut WORD32,
    mut p_qmf_right_im: *mut WORD32,
    mut sbr_tables_ptr: *mut ia_sbr_tables_struct,
    mut ptr_res: *const WORD16,
) -> VOID {
    let mut group: WORD = 0;
    let mut subband: WORD = 0;
    let mut max_subband: WORD = 0;
    let mut usb: WORD = 0;
    let mut k: WORD = 0;
    let mut p_hyb_left_re: *mut WORD32 = 0 as *mut WORD32;
    let mut p_hyb_left_re1: *mut WORD32 = 0 as *mut WORD32;
    let mut p_hyb_left_im: *mut WORD32 = 0 as *mut WORD32;
    let mut p_hyb_left_im1: *mut WORD32 = 0 as *mut WORD32;
    let mut p_hyb_right_re: *mut WORD32 = 0 as *mut WORD32;
    let mut p_hyb_right_re1: *mut WORD32 = 0 as *mut WORD32;
    let mut p_hyb_right_im: *mut WORD32 = 0 as *mut WORD32;
    let mut p_hyb_right_im1: *mut WORD32 = 0 as *mut WORD32;
    let mut temp_left_real: WORD32 = 0;
    let mut temp_left_imag: WORD32 = 0;
    let mut temp_right_real: WORD32 = 0;
    let mut temp_right_imag: WORD32 = 0;
    let mut hybrid_resol: WORD16 = 0;
    let mut tmp_real: WORD32 = 0;
    let mut tmp_img: WORD32 = 0;
    let mut tmp_real1: WORD32 = 0;
    let mut tmp_img1: WORD32 = 0;
    let mut H11_H12: [WORD16; 256] = [
        0 as core::ffi::c_int as WORD16,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    usb = (*ptr_ps_dec).usb as WORD;
    p_hyb_left_re1 = (*ptr_ps_dec).ptr_hyb_left_re;
    p_hyb_left_im1 = (*ptr_ps_dec).ptr_hyb_left_im;
    p_hyb_right_re1 = (*ptr_ps_dec).ptr_hyb_right_re;
    p_hyb_right_im1 = (*ptr_ps_dec).ptr_hyb_right_im;
    group = 0 as core::ffi::c_int as WORD;
    while group < NO_IID_GROUPS {
        (*ptr_ps_dec)
            .H11_H12[(2 as core::ffi::c_int * group as core::ffi::c_int
            + 0 as core::ffi::c_int) as usize] = ixheaac_add16(
            (*ptr_ps_dec)
                .H11_H12[(2 as core::ffi::c_int * group as core::ffi::c_int
                + 0 as core::ffi::c_int) as usize],
            (*ptr_ps_dec)
                .delta_h11_h12[(2 as core::ffi::c_int * group as core::ffi::c_int
                + 0 as core::ffi::c_int) as usize],
        );
        (*ptr_ps_dec)
            .H11_H12[(2 as core::ffi::c_int * group as core::ffi::c_int
            + 1 as core::ffi::c_int) as usize] = ixheaac_add16(
            (*ptr_ps_dec)
                .H11_H12[(2 as core::ffi::c_int * group as core::ffi::c_int
                + 1 as core::ffi::c_int) as usize],
            (*ptr_ps_dec)
                .delta_h11_h12[(2 as core::ffi::c_int * group as core::ffi::c_int
                + 1 as core::ffi::c_int) as usize],
        );
        (*ptr_ps_dec)
            .H21_H22[(2 as core::ffi::c_int * group as core::ffi::c_int
            + 0 as core::ffi::c_int) as usize] = ixheaac_add16(
            (*ptr_ps_dec)
                .H21_H22[(2 as core::ffi::c_int * group as core::ffi::c_int
                + 0 as core::ffi::c_int) as usize],
            (*ptr_ps_dec)
                .delta_h21_h22[(2 as core::ffi::c_int * group as core::ffi::c_int
                + 0 as core::ffi::c_int) as usize],
        );
        (*ptr_ps_dec)
            .H21_H22[(2 as core::ffi::c_int * group as core::ffi::c_int
            + 1 as core::ffi::c_int) as usize] = ixheaac_add16(
            (*ptr_ps_dec)
                .H21_H22[(2 as core::ffi::c_int * group as core::ffi::c_int
                + 1 as core::ffi::c_int) as usize],
            (*ptr_ps_dec)
                .delta_h21_h22[(2 as core::ffi::c_int * group as core::ffi::c_int
                + 1 as core::ffi::c_int) as usize],
        );
        group += 1;
    }
    subband = 0 as core::ffi::c_int as WORD;
    while subband < SUBQMF_GROUPS {
        temp_left_real = ixheaac_add32_sat(
            ixheaac_mult32x16in32(
                *p_hyb_left_re1.offset(subband as isize),
                (*ptr_ps_dec)
                    .H11_H12[(2 as core::ffi::c_int * subband as core::ffi::c_int
                    + 0 as core::ffi::c_int) as usize],
            ),
            ixheaac_mult32x16in32(
                *p_hyb_right_re1.offset(subband as isize),
                (*ptr_ps_dec)
                    .H21_H22[(2 as core::ffi::c_int * subband as core::ffi::c_int
                    + 0 as core::ffi::c_int) as usize],
            ),
        );
        temp_left_imag = ixheaac_add32_sat(
            ixheaac_mult32x16in32(
                *p_hyb_left_im1.offset(subband as isize),
                (*ptr_ps_dec)
                    .H11_H12[(2 as core::ffi::c_int * subband as core::ffi::c_int
                    + 0 as core::ffi::c_int) as usize],
            ),
            ixheaac_mult32x16in32(
                *p_hyb_right_im1.offset(subband as isize),
                (*ptr_ps_dec)
                    .H21_H22[(2 as core::ffi::c_int * subband as core::ffi::c_int
                    + 0 as core::ffi::c_int) as usize],
            ),
        );
        temp_right_real = ixheaac_add32_sat(
            ixheaac_mult32x16in32(
                *p_hyb_left_re1.offset(subband as isize),
                (*ptr_ps_dec)
                    .H11_H12[(2 as core::ffi::c_int * subband as core::ffi::c_int
                    + 1 as core::ffi::c_int) as usize],
            ),
            ixheaac_mult32x16in32(
                *p_hyb_right_re1.offset(subband as isize),
                (*ptr_ps_dec)
                    .H21_H22[(2 as core::ffi::c_int * subband as core::ffi::c_int
                    + 1 as core::ffi::c_int) as usize],
            ),
        );
        temp_right_imag = ixheaac_add32_sat(
            ixheaac_mult32x16in32(
                *p_hyb_left_im1.offset(subband as isize),
                (*ptr_ps_dec)
                    .H11_H12[(2 as core::ffi::c_int * subband as core::ffi::c_int
                    + 1 as core::ffi::c_int) as usize],
            ),
            ixheaac_mult32x16in32(
                *p_hyb_right_im1.offset(subband as isize),
                (*ptr_ps_dec)
                    .H21_H22[(2 as core::ffi::c_int * subband as core::ffi::c_int
                    + 1 as core::ffi::c_int) as usize],
            ),
        );
        *p_hyb_left_re1.offset(subband as isize) = ixheaac_shl32(
            temp_left_real,
            2 as WORD,
        );
        *p_hyb_left_im1.offset(subband as isize) = ixheaac_shl32(
            temp_left_imag,
            2 as WORD,
        );
        *p_hyb_right_re1.offset(subband as isize) = ixheaac_shl32(
            temp_right_real,
            2 as WORD,
        );
        *p_hyb_right_im1.offset(subband as isize) = ixheaac_shl32(
            temp_right_imag,
            2 as WORD,
        );
        subband += 1;
    }
    p_hyb_left_re = p_qmf_left_re;
    p_hyb_left_im = p_qmf_left_im;
    p_hyb_right_re = p_qmf_right_re;
    p_hyb_right_im = p_qmf_right_im;
    let mut h11_h12_src: *mut WORD32 = ((*ptr_ps_dec).H11_H12).as_mut_ptr()
        as *mut WORD32;
    let mut h21_h22_src: *mut WORD32 = ((*ptr_ps_dec).H21_H22).as_mut_ptr()
        as *mut WORD32;
    let mut h11_h12_dst: *mut WORD32 = H11_H12.as_mut_ptr() as *mut WORD32;
    group = SUBQMF_GROUPS as WORD;
    while group < NO_IID_GROUPS {
        max_subband = ixheaac_min32(
            usb as WORD32,
            (*(*sbr_tables_ptr).ps_tables_ptr)
                .borders_group[(group as core::ffi::c_int + 1 as core::ffi::c_int)
                as usize] as WORD32,
        ) as WORD;
        subband = (*(*sbr_tables_ptr).ps_tables_ptr).borders_group[group as usize]
            as WORD;
        while subband < max_subband {
            *h11_h12_dst.offset((2 as WORD * subband) as isize) = *h11_h12_src
                .offset(group as isize);
            *h11_h12_dst
                .offset(
                    (2 as core::ffi::c_int * subband as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) = *h21_h22_src.offset(group as isize);
            subband += 1;
        }
        group += 1;
    }
    subband = 0 as core::ffi::c_int as WORD;
    while subband < NO_QMF_CHANNELS_IN_HYBRID {
        let fresh12 = p_hyb_left_re1;
        p_hyb_left_re1 = p_hyb_left_re1.offset(1);
        tmp_real = *fresh12;
        let fresh13 = p_hyb_left_im1;
        p_hyb_left_im1 = p_hyb_left_im1.offset(1);
        tmp_img = *fresh13;
        let fresh14 = p_hyb_right_re1;
        p_hyb_right_re1 = p_hyb_right_re1.offset(1);
        tmp_real1 = *fresh14;
        let fresh15 = p_hyb_right_im1;
        p_hyb_right_im1 = p_hyb_right_im1.offset(1);
        tmp_img1 = *fresh15;
        let fresh16 = ptr_res;
        ptr_res = ptr_res.offset(1);
        hybrid_resol = ixheaac_min16(*fresh16, 6 as WORD16);
        k = (hybrid_resol as core::ffi::c_int - 2 as core::ffi::c_int) as WORD;
        while k >= 0 as core::ffi::c_int {
            let fresh17 = p_hyb_left_re1;
            p_hyb_left_re1 = p_hyb_left_re1.offset(1);
            tmp_real = ixheaac_add32_sat(tmp_real, *fresh17);
            let fresh18 = p_hyb_left_im1;
            p_hyb_left_im1 = p_hyb_left_im1.offset(1);
            tmp_img = ixheaac_add32_sat(tmp_img, *fresh18);
            let fresh19 = p_hyb_right_re1;
            p_hyb_right_re1 = p_hyb_right_re1.offset(1);
            tmp_real1 = ixheaac_add32_sat(tmp_real1, *fresh19);
            let fresh20 = p_hyb_right_im1;
            p_hyb_right_im1 = p_hyb_right_im1.offset(1);
            tmp_img1 = ixheaac_add32_sat(tmp_img1, *fresh20);
            k -= 1;
        }
        *p_hyb_left_re.offset(subband as isize) = tmp_real;
        *p_hyb_left_im.offset(subband as isize) = tmp_img;
        *p_hyb_right_re.offset(subband as isize) = tmp_real1;
        *p_hyb_right_im.offset(subband as isize) = tmp_img1;
        subband += 1;
    }
    while subband < usb {
        temp_left_real = ixheaac_add32_sat(
            ixheaac_mult32x16in32(
                *p_hyb_left_re.offset(subband as isize),
                H11_H12[(4 as core::ffi::c_int * subband as core::ffi::c_int
                    + 0 as core::ffi::c_int) as usize],
            ),
            ixheaac_mult32x16in32(
                *p_hyb_right_re.offset(subband as isize),
                H11_H12[(4 as core::ffi::c_int * subband as core::ffi::c_int
                    + 2 as core::ffi::c_int) as usize],
            ),
        );
        temp_left_imag = ixheaac_add32_sat(
            ixheaac_mult32x16in32(
                *p_hyb_left_im.offset(subband as isize),
                H11_H12[(4 as core::ffi::c_int * subband as core::ffi::c_int
                    + 0 as core::ffi::c_int) as usize],
            ),
            ixheaac_mult32x16in32(
                *p_hyb_right_im.offset(subband as isize),
                H11_H12[(4 as core::ffi::c_int * subband as core::ffi::c_int
                    + 2 as core::ffi::c_int) as usize],
            ),
        );
        temp_right_real = ixheaac_add32_sat(
            ixheaac_mult32x16in32(
                *p_hyb_left_re.offset(subband as isize),
                H11_H12[(4 as core::ffi::c_int * subband as core::ffi::c_int
                    + 1 as core::ffi::c_int) as usize],
            ),
            ixheaac_mult32x16in32(
                *p_hyb_right_re.offset(subband as isize),
                H11_H12[(4 as core::ffi::c_int * subband as core::ffi::c_int
                    + 3 as core::ffi::c_int) as usize],
            ),
        );
        temp_right_imag = ixheaac_add32_sat(
            ixheaac_mult32x16in32(
                *p_hyb_left_im.offset(subband as isize),
                H11_H12[(4 as core::ffi::c_int * subband as core::ffi::c_int
                    + 1 as core::ffi::c_int) as usize],
            ),
            ixheaac_mult32x16in32(
                *p_hyb_right_im.offset(subband as isize),
                H11_H12[(4 as core::ffi::c_int * subband as core::ffi::c_int
                    + 3 as core::ffi::c_int) as usize],
            ),
        );
        *p_hyb_left_re.offset(subband as isize) = ixheaac_shl32(
            temp_left_real,
            2 as WORD,
        );
        *p_hyb_left_im.offset(subband as isize) = ixheaac_shl32(
            temp_left_imag,
            2 as WORD,
        );
        *p_hyb_right_re.offset(subband as isize) = ixheaac_shl32(
            temp_right_real,
            2 as WORD,
        );
        *p_hyb_right_im.offset(subband as isize) = ixheaac_shl32(
            temp_right_imag,
            2 as WORD,
        );
        subband += 1;
    }
}
