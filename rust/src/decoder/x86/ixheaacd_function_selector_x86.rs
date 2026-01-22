extern "C" {
    fn ixheaacd_post_twiddle_dec(
        out_ptr: *mut WORD32,
        spec_data: *mut WORD32,
        ptr_imdct_tables: *mut ia_aac_dec_imdct_tables_struct,
        npoints: WORD32,
    ) -> VOID;
    fn ixheaacd_post_twid_overlap_add_dec(
        pcm_out: *mut WORD32,
        spec_data: *mut WORD32,
        ptr_imdct_tables: *mut ia_aac_dec_imdct_tables_struct,
        npoints: WORD,
        ptr_overlap_buf: *mut WORD32,
        q_shift: WORD16,
        window: *const WORD16,
        ch_fac: WORD16,
    ) -> VOID;
    fn ixheaacd_pretwiddle_compute_dec(
        spec_data1: *mut WORD32,
        spec_data2: *mut WORD32,
        out_ptr: *mut WORD32,
        ptr_imdct_tables: *mut ia_aac_dec_imdct_tables_struct,
        npoints4: WORD,
        neg_expo: WORD32,
    ) -> VOID;
    fn ixheaacd_pretwiddle_compute_960_dec(
        spec_data1: *mut WORD32,
        spec_data2: *mut WORD32,
        out_ptr: *mut WORD32,
        ptr_imdct_tables: *mut ia_aac_dec_imdct_tables_struct,
        npoints4: WORD,
        neg_expo: WORD32,
    ) -> VOID;
    fn ixheaacd_imdct_using_fft_dec(
        ptr_imdct_tables: *mut ia_aac_dec_imdct_tables_struct,
        npoints: WORD32,
        ptr_x: *mut WORD32,
        ptr_y: *mut WORD32,
    ) -> VOID;
    fn ixheaacd_dec_rearrange_short(
        ip: *mut WORD32,
        op: *mut WORD32,
        mdct_len_2: WORD32,
        re_arr_tab: *mut WORD16,
    ) -> VOID;
    fn ixheaacd_fft32x32_ld_dec(
        imdct_tables_ptr: *mut ia_aac_dec_imdct_tables_struct,
        npoints: WORD32,
        ptr_x: *mut WORD32,
        ptr_y: *mut WORD32,
    ) -> VOID;
    fn ixheaacd_neg_expo_inc_dec(neg_expo: WORD16) -> WORD16;
    fn ixheaacd_rearrange_dec(
        ip: *mut WORD32,
        op: *mut WORD32,
        mdct_len_2: WORD32,
        re_arr_tab: *mut UWORD8,
    ) -> VOID;
    fn ixheaacd_fft_15_ld_dec(
        inp: *mut WORD32,
        op: *mut WORD32,
        fft3out: *mut WORD32,
        re_arr_tab_sml_240_ptr: *mut UWORD8,
    ) -> VOID;
    fn ixheaacd_calc_max_spectral_line_dec(ptr_tmp: *mut WORD32, size: WORD32) -> WORD32;
    fn ixheaacd_tns_parcor_lpc_convert_dec(
        parcor: *mut WORD16,
        lpc: *mut WORD16,
        scale: *mut WORD16,
        order: WORD,
    ) -> VOID;
    fn ixheaacd_tns_ar_filter_dec(
        spectrum: *mut WORD32,
        size: WORD32,
        inc: WORD32,
        lpc: *mut WORD16,
        order: WORD32,
        shift_value: WORD32,
        scale_spec: WORD,
        ptr_filter_state: *mut WORD32,
    ) -> VOID;
    fn ixheaacd_tns_ar_filter_fixed_dec(
        spectrum: *mut WORD32,
        size: WORD32,
        inc: WORD32,
        lpc: *mut WORD32,
        order: WORD32,
        shift_value: WORD32,
        scale_spec: WORD,
    ) -> VOID;
    fn ixheaacd_covariance_matrix_calc_dec(
        sub_sign_xlow: *mut WORD32,
        cov_matrix: *mut ia_lpp_trans_cov_matrix,
        count: WORD32,
        auto_corr_length: WORD32,
    ) -> VOID;
    fn ixheaacd_covariance_matrix_calc_dec_960(
        sub_sign_xlow: *mut WORD32,
        cov_matrix: *mut ia_lpp_trans_cov_matrix,
        count: WORD32,
        auto_corr_length: WORD32,
    ) -> VOID;
    fn ixheaacd_covariance_matrix_calc_2_dec(
        cov_matrix: *mut ia_lpp_trans_cov_matrix,
        real_buffer: *mut WORD32,
        ixheaacd_num_bands: WORD32,
        slots: WORD16,
    ) -> VOID;
    fn ixheaacd_scale_factor_process_dec(
        x_invquant: *mut WORD32,
        scale_fact: *mut WORD16,
        no_band: WORD,
        width: *mut WORD8,
        scale_tables_ptr: *mut WORD32,
        total_channels: WORD32,
        object_type: WORD32,
        aac_sf_data_resil_flag: WORD32,
    ) -> VOID;
    fn ixheaacd_over_lap_add1_dec(
        coef: *mut WORD32,
        prev: *mut WORD32,
        out: *mut WORD32,
        window: *const WORD16,
        q_shift: WORD16,
        size: WORD16,
        ch_fac: WORD16,
    ) -> VOID;
    fn ixheaacd_over_lap_add2_dec(
        coef: *mut WORD32,
        prev: *mut WORD32,
        out: *mut WORD32,
        window: *const WORD16,
        q_shift: WORD16,
        size: WORD16,
        ch_fac: WORD16,
    ) -> VOID;
    fn ixheaacd_spec_to_overlapbuf_dec(
        ptr_overlap_buf: *mut WORD32,
        ptr_spec_coeff: *mut WORD32,
        q_shift: WORD32,
        size: WORD32,
    ) -> VOID;
    fn ixheaacd_overlap_buf_out_dec(
        out_samples: *mut WORD32,
        ptr_overlap_buf: *mut WORD32,
        size: WORD32,
        ch_fac: WORD16,
    ) -> VOID;
    fn ixheaacd_neg_shift_spec_dec(
        coef: *mut WORD32,
        out: *mut WORD32,
        q_shift: WORD16,
        ch_fac: WORD16,
    ) -> VOID;
    fn ixheaacd_overlap_out_copy_dec(
        out_samples: *mut WORD32,
        ptr_overlap_buf: *mut WORD32,
        ptr_overlap_buf1: *mut WORD32,
        ch_fac: WORD16,
        size_01: WORD16,
    ) -> VOID;
    fn ixheaacd_decorr_filter1_dec(
        ptr_ps_dec: *mut ia_ps_dec_struct,
        ps_tables_ptr: *mut ia_ps_tables_struct,
        transient_ratio: *mut WORD16,
    ) -> VOID;
    fn ixheaacd_decorr_filter2_dec(
        ptr_ps_dec: *mut ia_ps_dec_struct,
        p_buf_left_real: *mut WORD32,
        p_buf_left_imag: *mut WORD32,
        p_buf_right_real: *mut WORD32,
        p_buf_right_imag: *mut WORD32,
        ps_tables_ptr: *mut ia_ps_tables_struct,
        transient_ratio: *mut WORD16,
    ) -> VOID;
    fn ixheaacd_divide16_pos_dec(op1: WORD32, op2: WORD32) -> WORD32;
    fn ixheaacd_decorrelation_dec(
        ptr_ps_dec: *mut ia_ps_dec_struct,
        p_buf_left_real: *mut WORD32,
        p_buf_left_imag: *mut WORD32,
        p_buf_right_real: *mut WORD32,
        p_buf_right_imag: *mut WORD32,
        ps_tables_ptr: *mut ia_ps_tables_struct,
    ) -> VOID;
    fn ixheaacd_apply_rot_dec(
        ptr_ps_dec: *mut ia_ps_dec_struct,
        p_qmf_left_re: *mut WORD32,
        p_qmf_left_im: *mut WORD32,
        p_qmf_right_re: *mut WORD32,
        p_qmf_right_im: *mut WORD32,
        sbr_tables_ptr: *mut ia_sbr_tables_struct,
        ptr_res: *const WORD16,
    ) -> VOID;
    fn ixheaacd_fix_div_dec(divident: WORD32, divisor: WORD32) -> WORD32;
    fn ixheaacd_adjust_scale_dec(
        re: *mut *mut WORD32,
        im: *mut *mut WORD32,
        sub_band_start: WORD,
        num_sub_bands: WORD,
        start_pos: WORD,
        next_pos: WORD,
        shift: WORD,
        low_pow_flag: FLAG,
    ) -> VOID;
    fn ixheaacd_expsubbandsamples_dec(
        anal_buf_real_mant: *mut *mut WORD32,
        anal_buf_imag_mant: *mut *mut WORD32,
        sub_band_start: WORD,
        sub_band_end: WORD,
        start_pos: WORD,
        end_pos: WORD,
        low_pow_flag: FLAG,
    ) -> WORD16;
    fn ixheaacd_enery_calc_per_subband_dec(
        start_pos: WORD32,
        next_pos: WORD32,
        sub_band_start: WORD32,
        sub_band_end: WORD32,
        frame_exp: WORD32,
        nrg_est_mant: *mut WORD16,
        low_pow_flag: FLAG,
        ptr_sbr_tables: *mut ia_sbr_tables_struct,
        ptr_qmf_matrix: *mut WORD32,
    ) -> VOID;
    fn ixheaacd_harm_idx_zerotwolp_dec(
        ptr_real_buf: *mut WORD32,
        ptr_gain_buf: *mut WORD16,
        scale_change: WORD,
        ptr_sine_level_buf: *mut WORD16,
        ptr_rand_ph: *const WORD32,
        noise_lvl_me: *mut WORD16,
        num_sub_bands: WORD,
        noise_absc_flag: FLAG,
        harm_index: WORD32,
    ) -> VOID;
    fn ixheaacd_conv_ergtoamplitudelp_dec(
        bands: WORD32,
        noise_e: WORD16,
        nrg_sine: *mut WORD16,
        nrg_gain: *mut WORD16,
        noise_level_mant: *mut WORD16,
        sqrt_table: *mut WORD16,
    ) -> VOID;
    fn ixheaacd_conv_ergtoamplitude_dec(
        bands: WORD32,
        noise_e: WORD16,
        nrg_sine: *mut WORD16,
        nrg_gain: *mut WORD16,
        noise_level_mant: *mut WORD16,
        sqrt_table: *mut WORD16,
    ) -> VOID;
    fn ixheaacd_complex_fft_p2_dec(
        xr: *mut WORD32,
        xi: *mut WORD32,
        nlength: WORD32,
        fft_mode: WORD32,
        preshift: *mut WORD32,
    ) -> VOID;
    fn ixheaacd_calc_pre_twid_dec(
        ptr_x: *mut WORD32,
        r_ptr: *mut WORD32,
        i_ptr: *mut WORD32,
        nlength: WORD32,
        cos_ptr: *const WORD32,
        sin_ptr: *const WORD32,
    ) -> VOID;
    fn ixheaacd_calc_post_twid_dec(
        ptr_x: *mut WORD32,
        r_ptr: *mut WORD32,
        i_ptr: *mut WORD32,
        nlength: WORD32,
        cos_ptr: *const WORD32,
        sin_ptr: *const WORD32,
    ) -> VOID;
    fn ixheaacd_mps_synt_out_calc_dec(
        resolution: WORD32,
        out: *mut FLOAT32,
        state: *mut FLOAT32,
        filter_coeff: *const FLOAT32,
    ) -> VOID;
    fn ixheaacd_inv_dit_fft_8pt_dec(
        x: *mut WORD32,
        real: *mut WORD32,
        imag: *mut WORD32,
    ) -> VOID;
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
pub struct ia_aac_dec_imdct_tables_struct {
    pub cosine_array_2048_256: [WORD16; 514],
    pub dig_rev_table8_long: [WORD8; 64],
    pub dig_rev_table8_short: [WORD8; 8],
    pub fft_twiddle: [WORD32; 448],
    pub only_long_window_sine: [WORD16; 1024],
    pub only_long_window_kbd: [WORD16; 1024],
    pub only_short_window_sine: [WORD16; 128],
    pub only_short_window_kbd: [WORD16; 128],
    pub cosine_array_2048_256p: [WORD16; 514],
    pub w1024: [WORD32; 768],
    pub bit_rev_1024: [UWORD8; 256],
    pub bit_rev_512: [UWORD8; 64],
    pub bit_rev_128: [UWORD8; 16],
    pub bit_rev_32: [UWORD8; 4],
    pub w_256: [WORD32; 504],
    pub low_overlap_win: [WORD32; 512],
    pub window_sine_512: [WORD32; 512],
    pub cosine_array_1024: [WORD32; 512],
    pub low_overlap_win_480: [WORD32; 480],
    pub window_sine_480: [WORD32; 480],
    pub re_arr_tab_16: [UWORD8; 240],
    pub re_arr_tab_sml_240: [UWORD8; 240],
    pub cosine_array_960: [WORD32; 480],
    pub w_16: [WORD32; 24],
    pub window_sine_480_eld: [WORD16; 1920],
    pub window_sine_512_eld: [WORD16; 2048],
    pub only_long_window_sine_960: [WORD16; 960],
    pub only_long_window_kbd_960: [WORD16; 960],
    pub only_short_window_sine_120: [WORD16; 120],
    pub only_short_window_kbd_120: [WORD16; 120],
    pub re_arr_tab_32: [WORD16; 480],
    pub re_arr_tab_sml: [WORD16; 16],
    pub re_arr_tab_4: [WORD16; 60],
    pub re_arr_tab_15_4: [WORD16; 60],
    pub re_arr_tab_120: [WORD16; 60],
    pub re_arr_tab_5: [WORD16; 16],
    pub re_arr_tab_3: [WORD16; 16],
    pub re_arr_tab_sml_480: [WORD16; 480],
    pub cosine_array_1920: [WORD32; 960],
    pub w_512: [WORD16; 1020],
    pub w_32: [WORD16; 60],
    pub cosine_array_240: [WORD16; 120],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_lpp_trans_cov_matrix {
    pub phi_11: WORD32,
    pub phi_22: WORD32,
    pub phi_01: WORD32,
    pub phi_02: WORD32,
    pub phi_12: WORD32,
    pub phi_01_im: WORD32,
    pub phi_02_im: WORD32,
    pub phi_12_im: WORD32,
    pub d: WORD32,
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
#[no_mangle]
pub static mut ixheaacd_fix_div: Option<
    unsafe extern "C" fn(WORD32, WORD32) -> WORD32,
> = Some(ixheaacd_fix_div_dec as unsafe extern "C" fn(WORD32, WORD32) -> WORD32);
#[no_mangle]
pub static mut ixheaacd_covariance_matrix_calc: Option<
    unsafe extern "C" fn(
        *mut WORD32,
        *mut ia_lpp_trans_cov_matrix,
        WORD32,
        WORD32,
    ) -> VOID,
> = Some(
    ixheaacd_covariance_matrix_calc_dec
        as unsafe extern "C" fn(
            *mut WORD32,
            *mut ia_lpp_trans_cov_matrix,
            WORD32,
            WORD32,
        ) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_covariance_matrix_calc_2: Option<
    unsafe extern "C" fn(
        *mut ia_lpp_trans_cov_matrix,
        *mut WORD32,
        WORD32,
        WORD16,
    ) -> VOID,
> = Some(
    ixheaacd_covariance_matrix_calc_2_dec
        as unsafe extern "C" fn(
            *mut ia_lpp_trans_cov_matrix,
            *mut WORD32,
            WORD32,
            WORD16,
        ) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_over_lap_add1: Option<
    unsafe extern "C" fn(
        *mut WORD32,
        *mut WORD32,
        *mut WORD32,
        *const WORD16,
        WORD16,
        WORD16,
        WORD16,
    ) -> VOID,
> = Some(
    ixheaacd_over_lap_add1_dec
        as unsafe extern "C" fn(
            *mut WORD32,
            *mut WORD32,
            *mut WORD32,
            *const WORD16,
            WORD16,
            WORD16,
            WORD16,
        ) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_over_lap_add2: Option<
    unsafe extern "C" fn(
        *mut WORD32,
        *mut WORD32,
        *mut WORD32,
        *const WORD16,
        WORD16,
        WORD16,
        WORD16,
    ) -> VOID,
> = Some(
    ixheaacd_over_lap_add2_dec
        as unsafe extern "C" fn(
            *mut WORD32,
            *mut WORD32,
            *mut WORD32,
            *const WORD16,
            WORD16,
            WORD16,
            WORD16,
        ) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_decorr_filter2: Option<
    unsafe extern "C" fn(
        *mut ia_ps_dec_struct,
        *mut WORD32,
        *mut WORD32,
        *mut WORD32,
        *mut WORD32,
        *mut ia_ps_tables_struct,
        *mut WORD16,
    ) -> VOID,
> = Some(
    ixheaacd_decorr_filter2_dec
        as unsafe extern "C" fn(
            *mut ia_ps_dec_struct,
            *mut WORD32,
            *mut WORD32,
            *mut WORD32,
            *mut WORD32,
            *mut ia_ps_tables_struct,
            *mut WORD16,
        ) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_decorr_filter1: Option<
    unsafe extern "C" fn(
        *mut ia_ps_dec_struct,
        *mut ia_ps_tables_struct,
        *mut WORD16,
    ) -> VOID,
> = Some(
    ixheaacd_decorr_filter1_dec
        as unsafe extern "C" fn(
            *mut ia_ps_dec_struct,
            *mut ia_ps_tables_struct,
            *mut WORD16,
        ) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_divide16_pos: Option<
    unsafe extern "C" fn(WORD32, WORD32) -> WORD32,
> = Some(ixheaacd_divide16_pos_dec as unsafe extern "C" fn(WORD32, WORD32) -> WORD32);
#[no_mangle]
pub static mut ixheaacd_decorrelation: Option<
    unsafe extern "C" fn(
        *mut ia_ps_dec_struct,
        *mut WORD32,
        *mut WORD32,
        *mut WORD32,
        *mut WORD32,
        *mut ia_ps_tables_struct,
    ) -> VOID,
> = Some(
    ixheaacd_decorrelation_dec
        as unsafe extern "C" fn(
            *mut ia_ps_dec_struct,
            *mut WORD32,
            *mut WORD32,
            *mut WORD32,
            *mut WORD32,
            *mut ia_ps_tables_struct,
        ) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_apply_rot: Option<
    unsafe extern "C" fn(
        *mut ia_ps_dec_struct,
        *mut WORD32,
        *mut WORD32,
        *mut WORD32,
        *mut WORD32,
        *mut ia_sbr_tables_struct,
        *const WORD16,
    ) -> VOID,
> = Some(
    ixheaacd_apply_rot_dec
        as unsafe extern "C" fn(
            *mut ia_ps_dec_struct,
            *mut WORD32,
            *mut WORD32,
            *mut WORD32,
            *mut WORD32,
            *mut ia_sbr_tables_struct,
            *const WORD16,
        ) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_conv_ergtoamplitudelp: Option<
    unsafe extern "C" fn(
        WORD32,
        WORD16,
        *mut WORD16,
        *mut WORD16,
        *mut WORD16,
        *mut WORD16,
    ) -> VOID,
> = Some(
    ixheaacd_conv_ergtoamplitudelp_dec
        as unsafe extern "C" fn(
            WORD32,
            WORD16,
            *mut WORD16,
            *mut WORD16,
            *mut WORD16,
            *mut WORD16,
        ) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_conv_ergtoamplitude: Option<
    unsafe extern "C" fn(
        WORD32,
        WORD16,
        *mut WORD16,
        *mut WORD16,
        *mut WORD16,
        *mut WORD16,
    ) -> VOID,
> = Some(
    ixheaacd_conv_ergtoamplitude_dec
        as unsafe extern "C" fn(
            WORD32,
            WORD16,
            *mut WORD16,
            *mut WORD16,
            *mut WORD16,
            *mut WORD16,
        ) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_adjust_scale: Option<
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
> = Some(
    ixheaacd_adjust_scale_dec
        as unsafe extern "C" fn(
            *mut *mut WORD32,
            *mut *mut WORD32,
            WORD,
            WORD,
            WORD,
            WORD,
            WORD,
            FLAG,
        ) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_ixheaacd_expsubbandsamples: Option<
    unsafe extern "C" fn(
        *mut *mut WORD32,
        *mut *mut WORD32,
        WORD32,
        WORD32,
        WORD32,
        WORD32,
        FLAG,
    ) -> WORD16,
> = Some(
    ixheaacd_expsubbandsamples_dec
        as unsafe extern "C" fn(
            *mut *mut WORD32,
            *mut *mut WORD32,
            WORD,
            WORD,
            WORD,
            WORD,
            FLAG,
        ) -> WORD16,
);
#[no_mangle]
pub static mut ixheaacd_enery_calc_per_subband: Option<
    unsafe extern "C" fn(
        WORD32,
        WORD32,
        WORD32,
        WORD32,
        WORD32,
        *mut WORD16,
        FLAG,
        *mut ia_sbr_tables_struct,
        *mut WORD32,
    ) -> VOID,
> = Some(
    ixheaacd_enery_calc_per_subband_dec
        as unsafe extern "C" fn(
            WORD32,
            WORD32,
            WORD32,
            WORD32,
            WORD32,
            *mut WORD16,
            FLAG,
            *mut ia_sbr_tables_struct,
            *mut WORD32,
        ) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_harm_idx_zerotwolp: Option<
    unsafe extern "C" fn(
        *mut WORD32,
        *mut WORD16,
        WORD32,
        *mut WORD16,
        *const WORD32,
        *mut WORD16,
        WORD32,
        FLAG,
        WORD32,
    ) -> VOID,
> = Some(
    ixheaacd_harm_idx_zerotwolp_dec
        as unsafe extern "C" fn(
            *mut WORD32,
            *mut WORD16,
            WORD,
            *mut WORD16,
            *const WORD32,
            *mut WORD16,
            WORD,
            FLAG,
            WORD32,
        ) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_tns_ar_filter_fixed: Option<
    unsafe extern "C" fn(
        *mut WORD32,
        WORD32,
        WORD32,
        *mut WORD32,
        WORD32,
        WORD32,
        WORD,
    ) -> VOID,
> = Some(
    ixheaacd_tns_ar_filter_fixed_dec
        as unsafe extern "C" fn(
            *mut WORD32,
            WORD32,
            WORD32,
            *mut WORD32,
            WORD32,
            WORD32,
            WORD,
        ) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_tns_ar_filter: Option<
    unsafe extern "C" fn(
        *mut WORD32,
        WORD32,
        WORD32,
        *mut WORD16,
        WORD32,
        WORD32,
        WORD,
        *mut WORD32,
    ) -> VOID,
> = Some(
    ixheaacd_tns_ar_filter_dec
        as unsafe extern "C" fn(
            *mut WORD32,
            WORD32,
            WORD32,
            *mut WORD16,
            WORD32,
            WORD32,
            WORD,
            *mut WORD32,
        ) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_tns_parcor_lpc_convert: Option<
    unsafe extern "C" fn(*mut WORD16, *mut WORD16, *mut WORD16, WORD) -> VOID,
> = Some(
    ixheaacd_tns_parcor_lpc_convert_dec
        as unsafe extern "C" fn(*mut WORD16, *mut WORD16, *mut WORD16, WORD) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_calc_max_spectral_line: Option<
    unsafe extern "C" fn(*mut WORD32, WORD32) -> WORD32,
> = Some(
    ixheaacd_calc_max_spectral_line_dec
        as unsafe extern "C" fn(*mut WORD32, WORD32) -> WORD32,
);
#[no_mangle]
pub static mut ixheaacd_post_twiddle: Option<
    unsafe extern "C" fn(
        *mut WORD32,
        *mut WORD32,
        *mut ia_aac_dec_imdct_tables_struct,
        WORD,
    ) -> VOID,
> = Some(
    ixheaacd_post_twiddle_dec
        as unsafe extern "C" fn(
            *mut WORD32,
            *mut WORD32,
            *mut ia_aac_dec_imdct_tables_struct,
            WORD32,
        ) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_post_twid_overlap_add: Option<
    unsafe extern "C" fn(
        *mut WORD32,
        *mut WORD32,
        *mut ia_aac_dec_imdct_tables_struct,
        WORD,
        *mut WORD32,
        WORD16,
        *const WORD16,
        WORD16,
    ) -> VOID,
> = Some(
    ixheaacd_post_twid_overlap_add_dec
        as unsafe extern "C" fn(
            *mut WORD32,
            *mut WORD32,
            *mut ia_aac_dec_imdct_tables_struct,
            WORD,
            *mut WORD32,
            WORD16,
            *const WORD16,
            WORD16,
        ) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_neg_shift_spec: Option<
    unsafe extern "C" fn(*mut WORD32, *mut WORD32, WORD16, WORD16) -> VOID,
> = Some(
    ixheaacd_neg_shift_spec_dec
        as unsafe extern "C" fn(*mut WORD32, *mut WORD32, WORD16, WORD16) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_spec_to_overlapbuf: Option<
    unsafe extern "C" fn(*mut WORD32, *mut WORD32, WORD32, WORD32) -> VOID,
> = Some(
    ixheaacd_spec_to_overlapbuf_dec
        as unsafe extern "C" fn(*mut WORD32, *mut WORD32, WORD32, WORD32) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_overlap_buf_out: Option<
    unsafe extern "C" fn(*mut WORD32, *mut WORD32, WORD32, WORD16) -> VOID,
> = Some(
    ixheaacd_overlap_buf_out_dec
        as unsafe extern "C" fn(*mut WORD32, *mut WORD32, WORD32, WORD16) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_overlap_out_copy: Option<
    unsafe extern "C" fn(*mut WORD32, *mut WORD32, *mut WORD32, WORD16, WORD16) -> VOID,
> = Some(
    ixheaacd_overlap_out_copy_dec
        as unsafe extern "C" fn(
            *mut WORD32,
            *mut WORD32,
            *mut WORD32,
            WORD16,
            WORD16,
        ) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_pretwiddle_compute: Option<
    unsafe extern "C" fn(
        *mut WORD32,
        *mut WORD32,
        *mut WORD32,
        *mut ia_aac_dec_imdct_tables_struct,
        WORD,
        WORD32,
    ) -> VOID,
> = Some(
    ixheaacd_pretwiddle_compute_dec
        as unsafe extern "C" fn(
            *mut WORD32,
            *mut WORD32,
            *mut WORD32,
            *mut ia_aac_dec_imdct_tables_struct,
            WORD,
            WORD32,
        ) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_imdct_using_fft: Option<
    unsafe extern "C" fn(
        *mut ia_aac_dec_imdct_tables_struct,
        WORD32,
        *mut WORD32,
        *mut WORD32,
    ) -> VOID,
> = Some(
    ixheaacd_imdct_using_fft_dec
        as unsafe extern "C" fn(
            *mut ia_aac_dec_imdct_tables_struct,
            WORD32,
            *mut WORD32,
            *mut WORD32,
        ) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_complex_fft_p2: Option<
    unsafe extern "C" fn(*mut WORD32, *mut WORD32, WORD32, WORD32, *mut WORD32) -> VOID,
> = Some(
    ixheaacd_complex_fft_p2_dec
        as unsafe extern "C" fn(
            *mut WORD32,
            *mut WORD32,
            WORD32,
            WORD32,
            *mut WORD32,
        ) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_calc_pre_twid: Option<
    unsafe extern "C" fn(
        *mut WORD32,
        *mut WORD32,
        *mut WORD32,
        WORD32,
        *const WORD32,
        *const WORD32,
    ) -> VOID,
> = Some(
    ixheaacd_calc_pre_twid_dec
        as unsafe extern "C" fn(
            *mut WORD32,
            *mut WORD32,
            *mut WORD32,
            WORD32,
            *const WORD32,
            *const WORD32,
        ) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_calc_post_twid: Option<
    unsafe extern "C" fn(
        *mut WORD32,
        *mut WORD32,
        *mut WORD32,
        WORD32,
        *const WORD32,
        *const WORD32,
    ) -> VOID,
> = Some(
    ixheaacd_calc_post_twid_dec
        as unsafe extern "C" fn(
            *mut WORD32,
            *mut WORD32,
            *mut WORD32,
            WORD32,
            *const WORD32,
            *const WORD32,
        ) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_mps_synt_out_calc: Option<
    unsafe extern "C" fn(WORD32, *mut FLOAT32, *mut FLOAT32, *const FLOAT32) -> VOID,
> = Some(
    ixheaacd_mps_synt_out_calc_dec
        as unsafe extern "C" fn(
            WORD32,
            *mut FLOAT32,
            *mut FLOAT32,
            *const FLOAT32,
        ) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_fft_15_ld: Option<
    unsafe extern "C" fn(*mut WORD32, *mut WORD32, *mut WORD32, *mut UWORD8) -> VOID,
> = Some(
    ixheaacd_fft_15_ld_dec
        as unsafe extern "C" fn(
            *mut WORD32,
            *mut WORD32,
            *mut WORD32,
            *mut UWORD8,
        ) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_aac_ld_dec_rearrange: Option<
    unsafe extern "C" fn(*mut WORD32, *mut WORD32, WORD32, *mut UWORD8) -> VOID,
> = Some(
    ixheaacd_rearrange_dec
        as unsafe extern "C" fn(*mut WORD32, *mut WORD32, WORD32, *mut UWORD8) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_fft32x32_ld: Option<
    unsafe extern "C" fn(
        *mut ia_aac_dec_imdct_tables_struct,
        WORD32,
        *mut WORD32,
        *mut WORD32,
    ) -> VOID,
> = unsafe {
    Some(
        ixheaacd_fft32x32_ld_dec
            as unsafe extern "C" fn(
                *mut ia_aac_dec_imdct_tables_struct,
                WORD32,
                *mut WORD32,
                *mut WORD32,
            ) -> VOID,
    )
};
#[no_mangle]
pub static mut ixheaacd_fft32x32_ld2: Option<
    unsafe extern "C" fn(
        *mut ia_aac_dec_imdct_tables_struct,
        WORD32,
        *mut WORD32,
        *mut WORD32,
    ) -> VOID,
> = Some(
    ixheaacd_fft32x32_ld_dec
        as unsafe extern "C" fn(
            *mut ia_aac_dec_imdct_tables_struct,
            WORD32,
            *mut WORD32,
            *mut WORD32,
        ) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_neg_expo_inc: Option<unsafe extern "C" fn(WORD16) -> WORD16> = Some(
    ixheaacd_neg_expo_inc_dec as unsafe extern "C" fn(WORD16) -> WORD16,
);
#[no_mangle]
pub static mut ixheaacd_inv_dit_fft_8pt: Option<
    unsafe extern "C" fn(*mut WORD32, *mut WORD32, *mut WORD32) -> VOID,
> = Some(
    ixheaacd_inv_dit_fft_8pt_dec
        as unsafe extern "C" fn(*mut WORD32, *mut WORD32, *mut WORD32) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_scale_factor_process: Option<
    unsafe extern "C" fn(
        *mut WORD32,
        *mut WORD16,
        WORD,
        *mut WORD8,
        *mut WORD32,
        WORD32,
        WORD32,
        WORD32,
    ) -> VOID,
> = Some(
    ixheaacd_scale_factor_process_dec
        as unsafe extern "C" fn(
            *mut WORD32,
            *mut WORD16,
            WORD,
            *mut WORD8,
            *mut WORD32,
            WORD32,
            WORD32,
            WORD32,
        ) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_covariance_matrix_calc_960: Option<
    unsafe extern "C" fn(
        *mut WORD32,
        *mut ia_lpp_trans_cov_matrix,
        WORD32,
        WORD32,
    ) -> VOID,
> = Some(
    ixheaacd_covariance_matrix_calc_dec_960
        as unsafe extern "C" fn(
            *mut WORD32,
            *mut ia_lpp_trans_cov_matrix,
            WORD32,
            WORD32,
        ) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_aac_ld_dec_rearrange_960: Option<
    unsafe extern "C" fn(*mut WORD32, *mut WORD32, WORD32, *mut WORD16) -> VOID,
> = Some(
    ixheaacd_dec_rearrange_short
        as unsafe extern "C" fn(*mut WORD32, *mut WORD32, WORD32, *mut WORD16) -> VOID,
);
#[no_mangle]
pub static mut ixheaacd_pretwiddle_compute_960: Option<
    unsafe extern "C" fn(
        *mut WORD32,
        *mut WORD32,
        *mut WORD32,
        *mut ia_aac_dec_imdct_tables_struct,
        WORD,
        WORD32,
    ) -> VOID,
> = Some(
    ixheaacd_pretwiddle_compute_960_dec
        as unsafe extern "C" fn(
            *mut WORD32,
            *mut WORD32,
            *mut WORD32,
            *mut ia_aac_dec_imdct_tables_struct,
            WORD,
            WORD32,
        ) -> VOID,
);
