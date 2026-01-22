extern "C" {
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
    fn pow(__x: core::ffi::c_double, __y: core::ffi::c_double) -> core::ffi::c_double;
    fn ixheaacd_harm_idx_zerotwo(
        noise_absc_flag: FLAG,
        num_subband: WORD16,
        ptr_real_buf: *mut WORD32,
        ptr_im: *mut WORD32,
        smoothed_gain: *mut WORD16,
        smoothed_noise: *mut WORD16,
        factor: WORD,
        ptr_gain_buf: *mut WORD16,
        scale_change: WORD16,
        ptr_rand_ph: *const WORD32,
        ptr_sine_level_buf: *mut WORD16,
        noise_e: WORD16,
        harm_index: WORD32,
    ) -> VOID;
    fn ixheaacd_harm_idx_onethree(
        noise_absc_flag: FLAG,
        num_subband: WORD16,
        ptr_real_buf: *mut WORD32,
        ptr_im: *mut WORD32,
        smoothed_gain: *mut WORD16,
        smoothed_noise: *mut WORD16,
        factor: WORD,
        ptr_gain_buf: *mut WORD16,
        scale_change: WORD16,
        ptr_rand_ph: *const WORD32,
        ptr_sine_level_buf: *mut WORD16,
        noise_e: WORD16,
        freq_inv_flag: WORD,
        harm_index: WORD32,
    ) -> VOID;
    fn ixheaacd_lean_sbrconcealment(
        ptr_header_data: *mut ia_sbr_header_data_struct,
        ptr_sbr_data: *mut ia_sbr_frame_info_data_struct,
        ptr_prev_data: *mut ia_sbr_prev_frame_data_struct,
    ) -> VOID;
    fn ixheaacd_fix_mant_exp_add(
        a_m: WORD16,
        a_e: WORD16,
        b_m: WORD16,
        b_e: WORD16,
        ptr_sum_mant: *mut WORD16,
        ptr_sum_exp: *mut WORD16,
    ) -> VOID;
    fn ixheaacd_fix_mant_div(
        a_m: WORD16,
        b_m: WORD16,
        ptr_result: *mut WORD16,
        pstr_common_tables: *mut ixheaacd_misc_tables,
    ) -> WORD32;
}
pub type size_t = usize;
pub type WORD8 = core::ffi::c_schar;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD16 = core::ffi::c_short;
pub type WORD32 = core::ffi::c_int;
pub type FLOAT32 = core::ffi::c_float;
pub type VOID = ();
pub type FLAG = core::ffi::c_int;
pub type WORD = core::ffi::c_int;
pub type IA_ERRORCODE = WORD32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ixheaacd_lpp_trans_patch {
    pub num_patches: WORD32,
    pub start_subband: [WORD32; 7],
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
pub struct ia_sbr_prev_frame_data_struct {
    pub sfb_nrg_prev: [WORD16; 56],
    pub prev_noise_level: [WORD16; 5],
    pub amp_res: WORD16,
    pub end_position: WORD16,
    pub max_qmf_subband_aac: WORD32,
    pub coupling_mode: WORD32,
    pub sbr_invf_mode: [WORD32; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_frame_info_data_struct {
    pub num_env_sfac: WORD16,
    pub str_frame_info_details: ia_frame_info_struct,
    pub del_cod_dir_arr: [WORD16; 8],
    pub del_cod_dir_noise_arr: [WORD16; 2],
    pub sbr_invf_mode: [WORD32; 10],
    pub coupling_mode: WORD32,
    pub amp_res: WORD16,
    pub max_qmf_subband_aac: WORD32,
    pub add_harmonics: [FLAG; 56],
    pub int_env_sf_arr: [WORD16; 448],
    pub int_noise_floor: [WORD16; 10],
    pub num_noise_sfac: WORD32,
    pub str_pvc_frame_info: ia_frame_info_struct,
    pub env_short_flag_prev: WORD32,
    pub pstr_sbr_header: *mut ia_sbr_header_data_struct,
    pub num_time_slots: WORD32,
    pub rate: WORD32,
    pub sbr_patching_mode: WORD32,
    pub prev_sbr_patching_mode: WORD32,
    pub over_sampling_flag: WORD32,
    pub pitch_in_bins: WORD32,
    pub pvc_mode: WORD32,
    pub cov_count: WORD32,
    pub sbr_invf_mode_prev: [WORD32; 10],
    pub flt_env_sf_arr: [FLOAT32; 448],
    pub flt_noise_floor: [FLOAT32; 10],
    pub sfb_nrg_prev: [FLOAT32; 56],
    pub prev_noise_level: [FLOAT32; 10],
    pub inter_temp_shape_mode: [WORD32; 8],
    pub var_len: WORD32,
    pub bs_sin_pos_present: WORD32,
    pub sine_position: WORD32,
    pub sin_start_for_next_top: WORD32,
    pub sin_len_for_next_top: WORD32,
    pub sin_start_for_cur_top: WORD32,
    pub sin_len_for_cur_top: WORD32,
    pub var_len_id_prev: WORD32,
    pub str_frame_info_prev: ia_frame_info_struct,
    pub bw_array_prev: [FLOAT32; 6],
    pub patch_param: ixheaacd_lpp_trans_patch,
    pub harm_index: WORD32,
    pub phase_index: WORD32,
    pub harm_flag_prev: [WORD8; 64],
    pub e_gain: [[FLOAT32; 64]; 5],
    pub noise_buf: [[FLOAT32; 64]; 5],
    pub lim_table: [[WORD32; 13]; 4],
    pub gate_mode: [WORD32; 4],
    pub harm_flag_varlen_prev: [WORD8; 64],
    pub harm_flag_varlen: [WORD8; 64],
    pub qmapped_pvc: [[FLOAT32; 48]; 64],
    pub env_tmp: [[FLOAT32; 48]; 64],
    pub noise_level_pvc: [[FLOAT32; 48]; 64],
    pub nrg_est_pvc: [[FLOAT32; 48]; 64],
    pub nrg_ref_pvc: [[FLOAT32; 48]; 64],
    pub nrg_gain_pvc: [[FLOAT32; 48]; 64],
    pub nrg_tone_pvc: [[FLOAT32; 48]; 64],
    pub stereo_config_idx: WORD32,
    pub reset_flag: FLAG,
    pub mps_sbr_flag: FLAG,
    pub usac_independency_flag: FLAG,
    pub inter_tes_flag: FLAG,
    pub sbr_mode: FLAG,
    pub prev_sbr_mode: FLAG,
    pub eld_sbr_flag: WORD32,
}
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
pub const AOT_ER_AAC_ELD: AUDIO_OBJECT_TYPE = 39;
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
pub const MAX_NOISE_ENVELOPES: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_NOISE_COEFFS: core::ffi::c_int = 5 as core::ffi::c_int;
pub const MAX_NUM_NOISE_VALUES: core::ffi::c_int = MAX_NOISE_ENVELOPES
    * MAX_NOISE_COEFFS;
pub const MAX_FREQ_COEFFS: core::ffi::c_int = 56 as core::ffi::c_int;
#[inline]
unsafe extern "C" fn ixheaac_min32(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut min_val: WORD32 = 0;
    min_val = if a < b { a } else { b };
    return min_val;
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
unsafe extern "C" fn ixheaac_sub16_sat(mut op1: WORD16, mut op2: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    let mut diff: WORD32 = 0;
    diff = (op1 as core::ffi::c_int - op2 as core::ffi::c_int) as WORD32;
    var_out = ixheaac_sat16(diff);
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_mult16(mut op1: WORD16, mut op2: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = (op1 as WORD32 * op2 as WORD32 >> 16 as core::ffi::c_int) as WORD16;
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
pub const NOISE_FLOOR_OFFSET_INT: core::ffi::c_int = 6 as core::ffi::c_int;
pub const LOW: core::ffi::c_int = 0 as core::ffi::c_int;
pub const HIGH: core::ffi::c_int = 1 as core::ffi::c_int;
pub const DTDF_DIR_TIME: core::ffi::c_int = 1 as core::ffi::c_int;
pub const DTDF_DIR_FREQ: core::ffi::c_int = 0 as core::ffi::c_int;
pub const NOISE_FLOOR_OFFSET: core::ffi::c_float = 6.0f32;
pub const SBR_ENERGY_PAN_OFFSET: core::ffi::c_int = 12 as core::ffi::c_int;
pub const SBR_ENV_SF_MAX_VAL_1_5: core::ffi::c_int = 70 as core::ffi::c_int;
pub const MAX_NOISE_FLOOR_FAC_VAL: core::ffi::c_int = 35 as core::ffi::c_int;
pub const MIN_NOISE_FLOOR_FAC_VAL: core::ffi::c_int = 0 as core::ffi::c_int;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const SBR_NF_NO_RANDOM_VAL: core::ffi::c_int = 512 as core::ffi::c_int;
pub const ENV_EXP_FRACT: core::ffi::c_int = 0 as core::ffi::c_int;
pub const EXP_BITS: core::ffi::c_int = 6 as core::ffi::c_int;
pub const MASK_M: core::ffi::c_int = (((1 as core::ffi::c_int) << SHORT_BITS - EXP_BITS)
    - 1 as core::ffi::c_int) << EXP_BITS;
pub const MASK_FOR_EXP: core::ffi::c_int = ((1 as core::ffi::c_int) << EXP_BITS)
    - 1 as core::ffi::c_int;
pub const ROUNDING: core::ffi::c_int = (1 as core::ffi::c_int)
    << EXP_BITS - 1 as core::ffi::c_int;
pub const NRG_EXP_OFFSET: core::ffi::c_int = 16 as core::ffi::c_int;
pub const NOISE_EXP_OFFSET: core::ffi::c_int = 38 as core::ffi::c_int;
pub const COUPLING_LEVEL: core::ffi::c_int = 1 as core::ffi::c_int;
pub const COUPLING_BAL: core::ffi::c_int = 2 as core::ffi::c_int;
unsafe extern "C" fn ixheaacd_dequant_esbr_env_data(
    mut ptr_env_sf: *mut FLOAT32,
    mut num_env_sf: WORD32,
    mut num_noise_fac: WORD32,
    mut amp_res: WORD32,
    mut ptr_noise_floor: *mut FLOAT32,
) -> VOID {
    let mut i: WORD32 = 0;
    let array: [FLOAT32; 2] = [0.5f32, 1.0f32];
    let mut a_flt: FLOAT32 = array[amp_res as usize];
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_env_sf {
        *ptr_env_sf.offset(i as isize) = (pow(
            2 as core::ffi::c_int as core::ffi::c_double,
            (*ptr_env_sf.offset(i as isize) * a_flt) as core::ffi::c_double,
        ) * 64 as core::ffi::c_int as core::ffi::c_double) as FLOAT32;
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_noise_fac {
        let mut temp: FLOAT32 = *ptr_noise_floor.offset(i as isize);
        temp = NOISE_FLOOR_OFFSET - temp;
        temp = pow(2.0f64, temp as core::ffi::c_double) as FLOAT32;
        *ptr_noise_floor.offset(i as isize) = temp;
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_dequant_pvc_env_data(
    mut num_noise_fac: WORD32,
    mut ptr_noise_floor: *mut FLOAT32,
) -> VOID {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_noise_fac {
        let mut temp: FLOAT32 = *ptr_noise_floor.offset(i as isize);
        temp = NOISE_FLOOR_OFFSET - temp;
        temp = pow(2.0f64, temp as core::ffi::c_double) as FLOAT32;
        *ptr_noise_floor.offset(i as isize) = temp;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_map_res_energy(
    mut curr_val: WORD16,
    mut prev_data: *mut WORD16,
    mut ixheaacd_drc_offset: WORD32,
    mut index: WORD32,
    mut res: WORD32,
) -> VOID {
    if res == LOW {
        if ixheaacd_drc_offset >= 0 as core::ffi::c_int {
            if index < ixheaacd_drc_offset {
                *prev_data.offset(index as isize) = curr_val;
            } else {
                let mut index_2: WORD32 = 0;
                index_2 = index + index - ixheaacd_drc_offset;
                *prev_data.offset(index_2 as isize) = curr_val;
                *prev_data
                    .offset(
                        (index_2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                    ) = curr_val;
            }
        } else {
            ixheaacd_drc_offset = -ixheaacd_drc_offset;
            if index < ixheaacd_drc_offset {
                let mut index_3: WORD32 = 0;
                index_3 = index + index + index;
                *prev_data.offset(index_3 as isize) = curr_val;
                *prev_data
                    .offset(
                        (index_3 as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                    ) = curr_val;
                *prev_data
                    .offset(
                        (index_3 as core::ffi::c_int + 2 as core::ffi::c_int) as isize,
                    ) = curr_val;
            } else {
                let mut index_2_0: WORD32 = 0;
                index_2_0 = index + index + ixheaacd_drc_offset;
                *prev_data.offset(index_2_0 as isize) = curr_val;
                *prev_data
                    .offset(
                        (index_2_0 as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                    ) = curr_val;
            }
        }
    } else {
        *prev_data.offset(index as isize) = curr_val;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_process_del_cod_env_data(
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut ptr_sbr_data: *mut ia_sbr_frame_info_data_struct,
    mut ptr_prev_data: *mut ia_sbr_prev_frame_data_struct,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut dtdf_dir: WORD32 = 0;
    let mut num_sf_bands: WORD32 = 0;
    let mut band: WORD32 = 0;
    let mut freq_res: WORD32 = 0;
    let mut temp_val: WORD16 = 0;
    let mut ptr_prev_env_sf: *mut WORD16 = ((*ptr_prev_data).sfb_nrg_prev).as_mut_ptr();
    let mut ptr_env_sf: *mut WORD16 = ((*ptr_sbr_data).int_env_sf_arr).as_mut_ptr();
    let mut ptr_env_sf_float: *mut FLOAT32 = ((*ptr_sbr_data).flt_env_sf_arr)
        .as_mut_ptr();
    let mut ixheaacd_drc_offset: WORD32 = 0;
    ixheaacd_drc_offset = ((((*(*ptr_header_data).pstr_freq_band_data)
        .num_sf_bands[LOW as usize] as core::ffi::c_int) << 1 as core::ffi::c_int)
        - (*(*ptr_header_data).pstr_freq_band_data).num_sf_bands[HIGH as usize]
            as core::ffi::c_int) as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*ptr_sbr_data).str_frame_info_details.num_env as core::ffi::c_int {
        dtdf_dir = (*ptr_sbr_data).del_cod_dir_arr[i as usize] as WORD32;
        freq_res = (*ptr_sbr_data).str_frame_info_details.freq_res[i as usize] as WORD32;
        num_sf_bands = (*(*ptr_header_data).pstr_freq_band_data)
            .num_sf_bands[freq_res as usize] as WORD32;
        band = num_sf_bands;
        if dtdf_dir == DTDF_DIR_FREQ {
            ixheaacd_map_res_energy(
                *ptr_env_sf,
                ptr_prev_env_sf,
                ixheaacd_drc_offset,
                0 as WORD32,
                freq_res,
            );
            ptr_env_sf = ptr_env_sf.offset(1);
            ptr_env_sf_float = ptr_env_sf_float.offset(1);
            band = 1 as core::ffi::c_int as WORD32;
            while band < num_sf_bands {
                *ptr_env_sf = (*ptr_env_sf as core::ffi::c_int
                    + *ptr_env_sf.offset(-(1 as core::ffi::c_int as isize))
                        as core::ffi::c_int) as WORD16;
                ixheaacd_map_res_energy(
                    *ptr_env_sf,
                    ptr_prev_env_sf,
                    ixheaacd_drc_offset,
                    band,
                    freq_res,
                );
                *ptr_env_sf_float = *ptr_env_sf as FLOAT32;
                ptr_env_sf_float = ptr_env_sf_float.offset(1);
                ptr_env_sf = ptr_env_sf.offset(1);
                band += 1;
            }
        } else if freq_res == LOW {
            if ixheaacd_drc_offset < 0 as core::ffi::c_int {
                let mut tar: WORD32 = 0;
                let mut index_3: WORD32 = 0;
                ixheaacd_drc_offset = -ixheaacd_drc_offset;
                tar = ixheaac_min32(ixheaacd_drc_offset, band);
                band = 0 as core::ffi::c_int as WORD32;
                while band < tar {
                    index_3 = band + band + band;
                    temp_val = (*ptr_env_sf as core::ffi::c_int
                        + *ptr_prev_env_sf.offset(index_3 as isize) as core::ffi::c_int)
                        as WORD16;
                    *ptr_prev_env_sf.offset(index_3 as isize) = temp_val;
                    *ptr_prev_env_sf
                        .offset(
                            (index_3 as core::ffi::c_int + 1 as core::ffi::c_int)
                                as isize,
                        ) = temp_val;
                    *ptr_prev_env_sf
                        .offset(
                            (index_3 as core::ffi::c_int + 2 as core::ffi::c_int)
                                as isize,
                        ) = temp_val;
                    let fresh5 = ptr_env_sf;
                    ptr_env_sf = ptr_env_sf.offset(1);
                    *fresh5 = temp_val;
                    *ptr_env_sf_float = temp_val as FLOAT32;
                    ptr_env_sf_float = ptr_env_sf_float.offset(1);
                    band += 1;
                }
                while band < num_sf_bands {
                    index_3 = (band << 1 as core::ffi::c_int) + ixheaacd_drc_offset;
                    temp_val = (*ptr_env_sf as core::ffi::c_int
                        + *ptr_prev_env_sf.offset(index_3 as isize) as core::ffi::c_int)
                        as WORD16;
                    *ptr_prev_env_sf.offset(index_3 as isize) = temp_val;
                    *ptr_prev_env_sf
                        .offset(
                            (index_3 as core::ffi::c_int + 1 as core::ffi::c_int)
                                as isize,
                        ) = temp_val;
                    let fresh6 = ptr_env_sf;
                    ptr_env_sf = ptr_env_sf.offset(1);
                    *fresh6 = temp_val;
                    *ptr_env_sf_float = temp_val as FLOAT32;
                    ptr_env_sf_float = ptr_env_sf_float.offset(1);
                    band += 1;
                }
            } else {
                let mut tar_0: WORD32 = 0;
                let mut index_2: WORD32 = 0;
                let mut ptr2: *mut WORD16 = ptr_prev_env_sf;
                tar_0 = ixheaac_min32(ixheaacd_drc_offset, band);
                band = 0 as core::ffi::c_int as WORD32;
                while band < tar_0 {
                    *ptr_env_sf = (*ptr_env_sf as core::ffi::c_int
                        + *ptr2 as core::ffi::c_int) as WORD16;
                    *ptr2 = *ptr_env_sf;
                    *ptr_env_sf_float = *ptr_env_sf as FLOAT32;
                    ptr_env_sf_float = ptr_env_sf_float.offset(1);
                    ptr2 = ptr2.offset(1);
                    ptr_env_sf = ptr_env_sf.offset(1);
                    band += 1;
                }
                while band < num_sf_bands {
                    index_2 = if band < ixheaacd_drc_offset {
                        band
                    } else {
                        (band << 1 as core::ffi::c_int) - ixheaacd_drc_offset
                    };
                    temp_val = (*ptr_env_sf as core::ffi::c_int
                        + *ptr_prev_env_sf.offset(index_2 as isize) as core::ffi::c_int)
                        as WORD16;
                    *ptr_prev_env_sf.offset(index_2 as isize) = temp_val;
                    *ptr_prev_env_sf
                        .offset(
                            (index_2 as core::ffi::c_int + 1 as core::ffi::c_int)
                                as isize,
                        ) = temp_val;
                    let fresh7 = ptr_env_sf;
                    ptr_env_sf = ptr_env_sf.offset(1);
                    *fresh7 = temp_val;
                    *ptr_env_sf_float = temp_val as FLOAT32;
                    ptr_env_sf_float = ptr_env_sf_float.offset(1);
                    band += 1;
                }
            }
        } else {
            let mut ptr2_0: *mut WORD16 = ptr_prev_env_sf;
            band = (num_sf_bands as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
            while band >= 0 as core::ffi::c_int {
                *ptr_env_sf = (*ptr_env_sf as core::ffi::c_int
                    + *ptr2_0 as core::ffi::c_int) as WORD16;
                *ptr2_0 = *ptr_env_sf;
                *ptr_env_sf_float = *ptr_env_sf as FLOAT32;
                ptr_env_sf_float = ptr_env_sf_float.offset(1);
                ptr2_0 = ptr2_0.offset(1);
                ptr_env_sf = ptr_env_sf.offset(1);
                band -= 1;
            }
            band = num_sf_bands;
        }
        i += 1;
    }
}
#[inline]
unsafe extern "C" fn ixheaacd_wrong_timing_compensate(
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut ptr_sbr_data: *mut ia_sbr_frame_info_data_struct,
    mut ptr_prev_data: *mut ia_sbr_prev_frame_data_struct,
    mut pstr_common_tables: *mut ixheaacd_misc_tables,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut num_env_sf: WORD32 = 0;
    let mut p_frame_info: *mut ia_frame_info_struct = &mut (*ptr_sbr_data)
        .str_frame_info_details;
    let mut num_sf_bands: *mut WORD16 = ((*(*ptr_header_data).pstr_freq_band_data)
        .num_sf_bands)
        .as_mut_ptr();
    let mut start_pos_est: WORD32 = 0;
    let mut ref_len: WORD32 = 0;
    let mut new_len: WORD32 = 0;
    let mut shift: WORD32 = 0;
    let mut delta_exp: WORD16 = 0;
    start_pos_est = ((*ptr_prev_data).end_position as core::ffi::c_int
        - (*ptr_header_data).num_time_slots as core::ffi::c_int) as WORD32;
    ref_len = ((*p_frame_info).border_vec[1 as core::ffi::c_int as usize]
        as core::ffi::c_int
        - (*p_frame_info).border_vec[0 as core::ffi::c_int as usize] as core::ffi::c_int)
        as WORD32;
    new_len = (*p_frame_info).border_vec[1 as core::ffi::c_int as usize] as WORD32
        - start_pos_est;
    if new_len <= 0 as core::ffi::c_int {
        new_len = ref_len;
        start_pos_est = (*p_frame_info).border_vec[0 as core::ffi::c_int as usize]
            as WORD32;
    }
    delta_exp = (*pstr_common_tables).log_dual_is_table[ref_len as usize];
    delta_exp = (delta_exp as core::ffi::c_int
        - (*pstr_common_tables).log_dual_is_table[new_len as usize] as core::ffi::c_int)
        as WORD16;
    shift = (SHORT_BITS - ENV_EXP_FRACT - 3 as core::ffi::c_int
        - (*ptr_sbr_data).amp_res as core::ffi::c_int) as WORD32;
    delta_exp = ixheaac_shr16(delta_exp, shift as WORD16);
    (*p_frame_info).border_vec[0 as core::ffi::c_int as usize] = start_pos_est as WORD16;
    (*p_frame_info).noise_border_vec[0 as core::ffi::c_int as usize] = start_pos_est
        as WORD16;
    if start_pos_est < 0 as core::ffi::c_int {
        return -(1 as WORD32);
    }
    if (*ptr_sbr_data).coupling_mode != COUPLING_BAL {
        num_env_sf = (if (*p_frame_info).freq_res[0 as core::ffi::c_int as usize]
            as core::ffi::c_int != 0
        {
            *num_sf_bands.offset(HIGH as isize) as core::ffi::c_int
        } else {
            *num_sf_bands.offset(LOW as isize) as core::ffi::c_int
        }) as WORD32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < num_env_sf {
            (*ptr_sbr_data).int_env_sf_arr[i as usize] = ((*ptr_sbr_data)
                .int_env_sf_arr[i as usize] as core::ffi::c_int
                + delta_exp as core::ffi::c_int) as WORD16;
            (*ptr_sbr_data).flt_env_sf_arr[i as usize] = (*ptr_sbr_data)
                .int_env_sf_arr[i as usize] as FLOAT32;
            i += 1;
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_check_env_data(
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut ptr_sbr_data: *mut ia_sbr_frame_info_data_struct,
    mut ptr_prev_data: *mut ia_sbr_prev_frame_data_struct,
) -> WORD16 {
    let mut ptr_evn_sf: *mut WORD16 = ((*ptr_sbr_data).int_env_sf_arr).as_mut_ptr();
    let mut ptr_evn_sf_float: *mut FLOAT32 = ((*ptr_sbr_data).flt_env_sf_arr)
        .as_mut_ptr();
    let mut ptr_prev_evn_sf: *mut WORD16 = ((*ptr_prev_data).sfb_nrg_prev).as_mut_ptr();
    let mut i: WORD32 = 0;
    let mut error_code: FLAG = 0 as FLAG;
    let mut sbr_max_env_sf: WORD16 = 0;
    let mut amp_res: WORD32 = (*ptr_sbr_data).amp_res as WORD32;
    sbr_max_env_sf = (SBR_ENV_SF_MAX_VAL_1_5 >> amp_res) as WORD16;
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*ptr_sbr_data).num_env_sfac as core::ffi::c_int {
        if *ptr_evn_sf.offset(i as isize) as core::ffi::c_int
            > sbr_max_env_sf as core::ffi::c_int
        {
            error_code = 1 as core::ffi::c_int as FLAG;
        }
        if (*ptr_evn_sf.offset(i as isize) as core::ffi::c_int) < 0 as core::ffi::c_int {
            *ptr_evn_sf.offset(i as isize) = 0 as WORD16;
            *ptr_evn_sf_float.offset(i as isize) = 0 as core::ffi::c_int as FLOAT32;
        }
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i
        < (*(*ptr_header_data).pstr_freq_band_data).num_sf_bands[HIGH as usize]
            as core::ffi::c_int
    {
        if (*ptr_prev_evn_sf.offset(i as isize) as core::ffi::c_int)
            < 0 as core::ffi::c_int
        {
            *ptr_prev_evn_sf.offset(i as isize) = 0 as WORD16;
        } else if *ptr_prev_evn_sf.offset(i as isize) as core::ffi::c_int
            > sbr_max_env_sf as core::ffi::c_int
        {
            *ptr_prev_evn_sf.offset(i as isize) = sbr_max_env_sf;
        }
        i += 1;
    }
    return error_code as WORD16;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_dequant_env_data(
    mut ptr_sbr_data: *mut ia_sbr_frame_info_data_struct,
    mut amp_res: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut num_env_sf: WORD32 = (*ptr_sbr_data).num_env_sfac as WORD32;
    let mut mantissa: WORD32 = 0;
    let mut amp_res_1: WORD32 = 0;
    let mut exponent: WORD32 = 0;
    let mut exp_add: WORD32 = 7 as WORD32 + NRG_EXP_OFFSET;
    let mut ptr_env_sf: *mut WORD16 = ((*ptr_sbr_data).int_env_sf_arr).as_mut_ptr();
    let mant_arr: [WORD32; 2] = [0x4000 as core::ffi::c_int, 0x5a80 as core::ffi::c_int];
    amp_res_1 = 1 as WORD32 - amp_res;
    i = (num_env_sf as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while i >= 0 as core::ffi::c_int {
        exponent = *ptr_env_sf as WORD32;
        mantissa = mant_arr[(exponent & amp_res_1) as usize];
        exponent = exponent >> amp_res_1;
        exponent = exponent + exp_add;
        let fresh4 = ptr_env_sf;
        ptr_env_sf = ptr_env_sf.offset(1);
        *fresh4 = (mantissa as core::ffi::c_int
            | exponent as core::ffi::c_int & MASK_FOR_EXP) as WORD16;
        i -= 1;
    }
}
#[inline]
unsafe extern "C" fn ixheaacd_limit_noise_floor_fac(
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut ptr_sbr_data: *mut ia_sbr_frame_info_data_struct,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut tot_nf_bands: WORD32 = 0;
    let mut value: WORD32 = 0;
    let mut num_nf_bands: WORD32 = 0;
    let mut ptr_noise_floor: *mut WORD16 = 0 as *mut WORD16;
    let mut ptr_noise_floor_flt: *mut FLOAT32 = 0 as *mut FLOAT32;
    num_nf_bands = (*(*ptr_header_data).pstr_freq_band_data).num_nf_bands as WORD32;
    tot_nf_bands = (*ptr_sbr_data).str_frame_info_details.num_noise_env as WORD32
        * num_nf_bands;
    ptr_noise_floor = ((*ptr_sbr_data).int_noise_floor).as_mut_ptr();
    ptr_noise_floor_flt = &mut *((*ptr_sbr_data).flt_noise_floor)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
    i = (tot_nf_bands as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while i >= 0 as core::ffi::c_int {
        value = *ptr_noise_floor as WORD32;
        if value > MAX_NOISE_FLOOR_FAC_VAL {
            *ptr_noise_floor = MAX_NOISE_FLOOR_FAC_VAL as WORD16;
            *ptr_noise_floor_flt = MAX_NOISE_FLOOR_FAC_VAL as FLOAT32;
        } else if value < MIN_NOISE_FLOOR_FAC_VAL {
            *ptr_noise_floor = MIN_NOISE_FLOOR_FAC_VAL as WORD16;
            *ptr_noise_floor_flt = MIN_NOISE_FLOOR_FAC_VAL as FLOAT32;
        }
        ptr_noise_floor = ptr_noise_floor.offset(1);
        ptr_noise_floor_flt = ptr_noise_floor_flt.offset(1);
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_add_arr(
    mut ptr1: *mut WORD16,
    mut ptr2: *mut WORD16,
    mut num: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    i = (num as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while i >= 0 as core::ffi::c_int {
        *ptr2 = (*ptr2 as core::ffi::c_int + *ptr1 as core::ffi::c_int) as WORD16;
        ptr2 = ptr2.offset(1);
        ptr1 = ptr1.offset(1);
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_add_farr(
    mut ptr1: *mut FLOAT32,
    mut ptr2: *mut FLOAT32,
    mut num: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    i = (num as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while i >= 0 as core::ffi::c_int {
        *ptr2 = *ptr2 + *ptr1;
        ptr2 = ptr2.offset(1);
        ptr1 = ptr1.offset(1);
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_calc_noise_floor(
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut ptr_sbr_data: *mut ia_sbr_frame_info_data_struct,
    mut ptr_prev_data: *mut ia_sbr_prev_frame_data_struct,
    mut audio_object_type: WORD32,
) -> IA_ERRORCODE {
    let mut i: WORD32 = 0;
    let mut num_nf_bands: WORD32 = 0;
    let mut num_noise_env: WORD32 = 0;
    let mut ixheaacd_drc_offset: WORD32 = 0;
    let mut ptr_noise_floor: *mut WORD16 = ((*ptr_sbr_data).int_noise_floor)
        .as_mut_ptr();
    let mut ptr_prev_noise_floor: *mut WORD16 = ((*ptr_prev_data).prev_noise_level)
        .as_mut_ptr();
    let mut ptr1: *mut WORD16 = 0 as *mut WORD16;
    let mut ptr2: *mut WORD16 = 0 as *mut WORD16;
    let mut f_ptr1: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut f_ptr2: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut num: WORD32 = 0;
    let mut ptr_noise_floor_float: *mut FLOAT32 = ((*ptr_sbr_data).flt_noise_floor)
        .as_mut_ptr();
    num_nf_bands = (*(*ptr_header_data).pstr_freq_band_data).num_nf_bands as WORD32;
    num_noise_env = (*ptr_sbr_data).str_frame_info_details.num_noise_env as WORD32;
    if (*ptr_sbr_data).del_cod_dir_noise_arr[0 as core::ffi::c_int as usize]
        as core::ffi::c_int == DTDF_DIR_FREQ
    {
        let fresh8 = ptr_noise_floor;
        ptr_noise_floor = ptr_noise_floor.offset(1);
        ptr1 = fresh8;
        ptr2 = ptr_noise_floor;
        num = (num_nf_bands as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        f_ptr1 = &mut *ptr_noise_floor_float.offset(0 as core::ffi::c_int as isize)
            as *mut FLOAT32;
        f_ptr2 = &mut *ptr_noise_floor_float.offset(1 as core::ffi::c_int as isize)
            as *mut FLOAT32;
    } else {
        ptr1 = ptr_prev_noise_floor;
        ptr2 = ((*ptr_sbr_data).int_noise_floor).as_mut_ptr();
        f_ptr1 = &mut *((*ptr_sbr_data).prev_noise_level)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
        f_ptr2 = &mut *ptr_noise_floor_float.offset(0 as core::ffi::c_int as isize)
            as *mut FLOAT32;
        num = num_nf_bands;
    }
    ixheaacd_add_arr(ptr1, ptr2, num);
    ixheaacd_add_farr(f_ptr1, f_ptr2, num);
    if num_noise_env > 1 as core::ffi::c_int {
        if (*ptr_sbr_data).del_cod_dir_noise_arr[1 as core::ffi::c_int as usize]
            as core::ffi::c_int == DTDF_DIR_FREQ
        {
            ptr1 = &mut *((*ptr_sbr_data).int_noise_floor)
                .as_mut_ptr()
                .offset(num_nf_bands as isize) as *mut WORD16;
            ptr2 = &mut *((*ptr_sbr_data).int_noise_floor)
                .as_mut_ptr()
                .offset(
                    (num_nf_bands as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                ) as *mut WORD16;
            f_ptr1 = &mut *((*ptr_sbr_data).flt_noise_floor)
                .as_mut_ptr()
                .offset(num_nf_bands as isize) as *mut FLOAT32;
            f_ptr2 = &mut *((*ptr_sbr_data).flt_noise_floor)
                .as_mut_ptr()
                .offset(
                    (num_nf_bands as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                ) as *mut FLOAT32;
            num = (num_nf_bands as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        } else {
            ptr1 = &mut *((*ptr_sbr_data).int_noise_floor)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut WORD16;
            ptr2 = &mut *((*ptr_sbr_data).int_noise_floor)
                .as_mut_ptr()
                .offset(num_nf_bands as isize) as *mut WORD16;
            f_ptr1 = &mut *((*ptr_sbr_data).flt_noise_floor)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
            f_ptr2 = &mut *((*ptr_sbr_data).flt_noise_floor)
                .as_mut_ptr()
                .offset(num_nf_bands as isize) as *mut FLOAT32;
            num = num_nf_bands;
        }
        ixheaacd_add_arr(ptr1, ptr2, num);
        ixheaacd_add_farr(f_ptr1, f_ptr2, num);
    }
    ixheaacd_limit_noise_floor_fac(ptr_header_data, ptr_sbr_data);
    ixheaacd_drc_offset = (num_nf_bands as core::ffi::c_int
        * (num_noise_env as core::ffi::c_int - 1 as core::ffi::c_int)) as WORD32;
    if ixheaacd_drc_offset < 0 as core::ffi::c_int
        || ixheaacd_drc_offset >= MAX_NUM_NOISE_VALUES
    {
        return IA_FATAL_ERROR as IA_ERRORCODE;
    }
    ptr1 = &mut *((*ptr_sbr_data).int_noise_floor)
        .as_mut_ptr()
        .offset(ixheaacd_drc_offset as isize) as *mut WORD16;
    ptr2 = ptr_prev_noise_floor;
    memcpy(
        ptr2 as *mut core::ffi::c_void,
        ptr1 as *const core::ffi::c_void,
        (::core::mem::size_of::<WORD16>() as size_t).wrapping_mul(num_nf_bands as size_t),
    );
    if (*ptr_header_data).usac_flag == 0 {
        i = 0 as core::ffi::c_int as WORD32;
        while i < num_nf_bands {
            (*ptr_sbr_data).prev_noise_level[i as usize] = *ptr_prev_noise_floor
                .offset(i as isize) as FLOAT32;
            i += 1;
        }
    }
    if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
        && (*ptr_header_data).enh_sbr != 0
    {
        let mut noise_floor_exp: WORD32 = 0;
        let mut tot_nf_bands: WORD32 = 0;
        tot_nf_bands = num_nf_bands * num_noise_env;
        ptr_noise_floor = &mut *((*ptr_sbr_data).int_noise_floor)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut WORD16;
        i = 0 as core::ffi::c_int as WORD32;
        while i < tot_nf_bands {
            noise_floor_exp = (NOISE_FLOOR_OFFSET_INT + 1 as core::ffi::c_int
                + NOISE_EXP_OFFSET - *ptr_noise_floor as core::ffi::c_int) as WORD32;
            let fresh9 = ptr_noise_floor_float;
            ptr_noise_floor_float = ptr_noise_floor_float.offset(1);
            *fresh9 = *ptr_noise_floor as FLOAT32;
            let fresh10 = ptr_noise_floor;
            ptr_noise_floor = ptr_noise_floor.offset(1);
            *fresh10 = (0x4000 as core::ffi::c_int
                + (noise_floor_exp as core::ffi::c_int & MASK_FOR_EXP)) as WORD16;
            i += 1;
        }
    } else if (*ptr_sbr_data).coupling_mode != COUPLING_BAL {
        let mut noise_floor_exp_0: WORD32 = 0;
        let mut tot_nf_bands_0: WORD32 = 0;
        tot_nf_bands_0 = num_nf_bands * num_noise_env;
        ptr_noise_floor = &mut *((*ptr_sbr_data).int_noise_floor)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut WORD16;
        i = 0 as core::ffi::c_int as WORD32;
        while i < tot_nf_bands_0 {
            noise_floor_exp_0 = (NOISE_FLOOR_OFFSET_INT + 1 as core::ffi::c_int
                + NOISE_EXP_OFFSET - *ptr_noise_floor as core::ffi::c_int) as WORD32;
            let fresh11 = ptr_noise_floor_float;
            ptr_noise_floor_float = ptr_noise_floor_float.offset(1);
            *fresh11 = *ptr_noise_floor as FLOAT32;
            let fresh12 = ptr_noise_floor;
            ptr_noise_floor = ptr_noise_floor.offset(1);
            *fresh12 = (0x4000 as core::ffi::c_int
                + (noise_floor_exp_0 as core::ffi::c_int & MASK_FOR_EXP)) as WORD16;
            i += 1;
        }
    }
    return IA_NO_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_dec_sbrdata_for_pvc(
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut ptr_sbr_data: *mut ia_sbr_frame_info_data_struct,
    mut ptr_prev_data: *mut ia_sbr_prev_frame_data_struct,
    mut audio_object_type: WORD32,
) -> IA_ERRORCODE {
    let mut err: WORD32 = 0 as WORD32;
    err = ixheaacd_calc_noise_floor(
        ptr_header_data,
        ptr_sbr_data,
        ptr_prev_data,
        audio_object_type,
    ) as WORD32;
    if err != 0 {
        return err as IA_ERRORCODE;
    }
    if (*ptr_sbr_data).coupling_mode == 0 {
        (*ptr_sbr_data).num_noise_sfac = ((*(*ptr_header_data).pstr_freq_band_data)
            .num_nf_bands as core::ffi::c_int
            * (*ptr_sbr_data).str_frame_info_details.num_noise_env as core::ffi::c_int)
            as WORD32;
        ixheaacd_dequant_pvc_env_data(
            (*ptr_sbr_data).num_noise_sfac,
            ((*ptr_sbr_data).flt_noise_floor).as_mut_ptr(),
        );
    }
    return IA_NO_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_sbr_env_dequant_coup_fix(
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut ptr_data_left: *mut ia_sbr_frame_info_data_struct,
    mut ptr_data_right: *mut ia_sbr_frame_info_data_struct,
    mut pstr_common_tables: *mut ixheaacd_misc_tables,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut num_env_sf: WORD32 = (*ptr_data_left).num_env_sfac as WORD32;
    let mut temp_left_mant: WORD16 = 0;
    let mut temp_right_mant: WORD16 = 0;
    let mut temp_right_plus1_mant: WORD16 = 0;
    let mut new_left_mant: WORD16 = 0;
    let mut new_right_mant: WORD16 = 0;
    let mut temp_left_exp: WORD16 = 0;
    let mut temp_right_exp: WORD16 = 0;
    let mut temp_rightplus1_exp: WORD16 = 0;
    let mut new_left_exp: WORD16 = 0;
    let mut new_right_exp: WORD16 = 0;
    let mut i_end: WORD32 = 0;
    let mut r_data: *mut WORD16 = ((*ptr_data_right).int_env_sf_arr).as_mut_ptr();
    let mut l_data: *mut WORD16 = ((*ptr_data_left).int_env_sf_arr).as_mut_ptr();
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_env_sf {
        temp_right_mant = (*r_data as core::ffi::c_int & MASK_M) as WORD16;
        temp_right_exp = (*r_data as core::ffi::c_int & MASK_FOR_EXP) as WORD16;
        temp_right_exp = (temp_right_exp as core::ffi::c_int
            - (18 as core::ffi::c_int + 16 as core::ffi::c_int)) as WORD16;
        temp_left_mant = (*l_data as core::ffi::c_int & MASK_M) as WORD16;
        temp_left_exp = (*l_data as core::ffi::c_int & MASK_FOR_EXP) as WORD16;
        temp_left_exp = (temp_left_exp as core::ffi::c_int - 16 as core::ffi::c_int)
            as WORD16;
        ixheaacd_fix_mant_exp_add(
            temp_right_mant,
            temp_right_exp,
            0x4000 as WORD16,
            1 as WORD16,
            &mut temp_right_plus1_mant,
            &mut temp_rightplus1_exp,
        );
        new_right_exp = ixheaacd_fix_mant_div(
            temp_left_mant,
            temp_right_plus1_mant,
            &mut new_right_mant,
            pstr_common_tables,
        ) as WORD16;
        new_right_exp = (new_right_exp as core::ffi::c_int
            + (temp_left_exp as core::ffi::c_int
                - temp_rightplus1_exp as core::ffi::c_int + 2 as core::ffi::c_int))
            as WORD16;
        new_left_mant = ixheaac_mult16_shl(temp_right_mant, new_right_mant);
        new_left_exp = (temp_right_exp as core::ffi::c_int
            + new_right_exp as core::ffi::c_int) as WORD16;
        let fresh0 = r_data;
        r_data = r_data.offset(1);
        *fresh0 = ((new_right_mant as core::ffi::c_int + ROUNDING & MASK_M)
            + (new_right_exp as core::ffi::c_int + NRG_EXP_OFFSET & MASK_FOR_EXP))
            as WORD16;
        let fresh1 = l_data;
        l_data = l_data.offset(1);
        *fresh1 = ((new_left_mant as core::ffi::c_int + ROUNDING & MASK_M)
            + (new_left_exp as core::ffi::c_int + NRG_EXP_OFFSET & MASK_FOR_EXP))
            as WORD16;
        i += 1;
    }
    i_end = ((*(*ptr_header_data).pstr_freq_band_data).num_nf_bands as core::ffi::c_int
        * (*ptr_data_left).str_frame_info_details.num_noise_env as core::ffi::c_int)
        as WORD32;
    r_data = ((*ptr_data_right).int_noise_floor).as_mut_ptr();
    l_data = ((*ptr_data_left).int_noise_floor).as_mut_ptr();
    i = (i_end as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while i >= 0 as core::ffi::c_int {
        temp_left_exp = ((*l_data as core::ffi::c_int
            & (((1 as core::ffi::c_int) << 6 as core::ffi::c_int)
                - 1 as core::ffi::c_int) as WORD16 as core::ffi::c_int) as WORD16
            as core::ffi::c_int - 38 as core::ffi::c_int) as WORD16;
        temp_right_exp = (*r_data as core::ffi::c_int - 12 as core::ffi::c_int)
            as WORD16;
        ixheaacd_fix_mant_exp_add(
            0x4000 as WORD16,
            ixheaac_add16(1 as WORD16, temp_right_exp),
            0x4000 as WORD16,
            1 as WORD16,
            &mut temp_right_plus1_mant,
            &mut temp_rightplus1_exp,
        );
        new_right_exp = ixheaacd_fix_mant_div(
            0x4000 as WORD16,
            temp_right_plus1_mant,
            &mut new_right_mant,
            pstr_common_tables,
        ) as WORD16;
        new_right_exp = (new_right_exp as core::ffi::c_int
            + (temp_left_exp as core::ffi::c_int
                - temp_rightplus1_exp as core::ffi::c_int + 2 as core::ffi::c_int))
            as WORD16;
        new_left_mant = new_right_mant;
        new_left_exp = (new_right_exp as core::ffi::c_int
            + temp_right_exp as core::ffi::c_int) as WORD16;
        let fresh2 = r_data;
        r_data = r_data.offset(1);
        *fresh2 = ((new_right_mant as core::ffi::c_int + ROUNDING & MASK_M)
            + (new_right_exp as core::ffi::c_int + NOISE_EXP_OFFSET & MASK_FOR_EXP))
            as WORD16;
        let fresh3 = l_data;
        l_data = l_data.offset(1);
        *fresh3 = ((new_left_mant as core::ffi::c_int + ROUNDING & MASK_M)
            + (new_left_exp as core::ffi::c_int + NOISE_EXP_OFFSET & MASK_FOR_EXP))
            as WORD16;
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_sbr_env_dequant_coup(
    mut ptr_data_ch_0: *mut ia_sbr_frame_info_data_struct,
    mut ptr_data_ch_1: *mut ia_sbr_frame_info_data_struct,
) -> VOID {
    let mut ptr_env_sf_left: *mut FLOAT32 = ((*ptr_data_ch_0).flt_env_sf_arr)
        .as_mut_ptr();
    let mut ptr_env_sf_right: *mut FLOAT32 = ((*ptr_data_ch_1).flt_env_sf_arr)
        .as_mut_ptr();
    let mut ptr_noise_floor_left: *mut FLOAT32 = ((*ptr_data_ch_0).flt_noise_floor)
        .as_mut_ptr();
    let mut ptr_noise_floor_right: *mut FLOAT32 = ((*ptr_data_ch_1).flt_noise_floor)
        .as_mut_ptr();
    let mut num_env_sf: WORD32 = (*ptr_data_ch_0).num_env_sfac as WORD32;
    let mut num_noise_fac: WORD32 = (*ptr_data_ch_0).num_noise_sfac;
    let mut amp_res: WORD32 = (*ptr_data_ch_0).amp_res as WORD32;
    let mut i: WORD32 = 0;
    let mut temp_l: FLOAT32 = 0.;
    let mut temp_r: FLOAT32 = 0.;
    let pan_offset: [FLOAT32; 2] = [24.0f32, 12.0f32];
    let a_arr: [FLOAT32; 2] = [0.5f32, 1.0f32];
    let mut a: FLOAT32 = a_arr[amp_res as usize];
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_env_sf {
        temp_l = *ptr_env_sf_left.offset(i as isize);
        temp_r = *ptr_env_sf_right.offset(i as isize);
        *ptr_env_sf_left.offset(i as isize) = (64 as core::ffi::c_int
            as core::ffi::c_double
            * (pow(
                2 as core::ffi::c_int as core::ffi::c_double,
                (temp_l * a + 1 as core::ffi::c_int as FLOAT32) as core::ffi::c_double,
            )
                / (1 as core::ffi::c_int as core::ffi::c_double
                    + pow(
                        2 as core::ffi::c_int as core::ffi::c_double,
                        ((pan_offset[amp_res as usize] - temp_r) * a)
                            as core::ffi::c_double,
                    )))) as FLOAT32;
        *ptr_env_sf_right.offset(i as isize) = (64 as core::ffi::c_int
            as core::ffi::c_double
            * (pow(
                2 as core::ffi::c_int as core::ffi::c_double,
                (temp_l * a + 1 as core::ffi::c_int as FLOAT32) as core::ffi::c_double,
            )
                / (1 as core::ffi::c_int as core::ffi::c_double
                    + pow(
                        2 as core::ffi::c_int as core::ffi::c_double,
                        ((temp_r - pan_offset[amp_res as usize]) * a)
                            as core::ffi::c_double,
                    )))) as FLOAT32;
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_noise_fac {
        temp_l = *ptr_noise_floor_left.offset(i as isize);
        temp_r = *ptr_noise_floor_right.offset(i as isize);
        *ptr_noise_floor_left.offset(i as isize) = (pow(
            2 as core::ffi::c_int as core::ffi::c_double,
            (NOISE_FLOOR_OFFSET - temp_l as core::ffi::c_float
                + 1 as core::ffi::c_int as core::ffi::c_float) as core::ffi::c_double,
        )
            / (1 as core::ffi::c_int as core::ffi::c_double
                + pow(
                    2 as core::ffi::c_int as core::ffi::c_double,
                    (pan_offset[1 as core::ffi::c_int as usize] - temp_r)
                        as core::ffi::c_double,
                ))) as FLOAT32;
        *ptr_noise_floor_right.offset(i as isize) = (pow(
            2 as core::ffi::c_int as core::ffi::c_double,
            (NOISE_FLOOR_OFFSET - temp_l as core::ffi::c_float
                + 1 as core::ffi::c_int as core::ffi::c_float) as core::ffi::c_double,
        )
            / (1 as core::ffi::c_int as core::ffi::c_double
                + pow(
                    2 as core::ffi::c_int as core::ffi::c_double,
                    (temp_r - pan_offset[1 as core::ffi::c_int as usize])
                        as core::ffi::c_double,
                ))) as FLOAT32;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_dec_sbrdata(
    mut ptr_header_data_ch_0: *mut ia_sbr_header_data_struct,
    mut ptr_header_data_ch_1: *mut ia_sbr_header_data_struct,
    mut ptr_sbr_data_ch_0: *mut ia_sbr_frame_info_data_struct,
    mut ptr_prev_data_ch_0: *mut ia_sbr_prev_frame_data_struct,
    mut ptr_sbr_data_ch_1: *mut ia_sbr_frame_info_data_struct,
    mut ptr_prev_data_ch_1: *mut ia_sbr_prev_frame_data_struct,
    mut ptr_common_tables: *mut ixheaacd_misc_tables,
    mut ldmps_present: WORD32,
    mut audio_object_type: WORD32,
    mut ec_flag: WORD32,
) -> IA_ERRORCODE {
    let mut error_code: FLAG = 0;
    let mut err: WORD32 = 0 as WORD32;
    let mut temp_sfb_nrg_prev: [WORD16; 56] = [0; 56];
    let mut usac_flag: WORD32 = (*ptr_header_data_ch_0).usac_flag
        | (*ptr_header_data_ch_0).enh_sbr as WORD32;
    if ec_flag != 0 {
        memcpy(
            temp_sfb_nrg_prev.as_mut_ptr() as *mut core::ffi::c_void,
            ((*ptr_prev_data_ch_0).sfb_nrg_prev).as_mut_ptr()
                as *const core::ffi::c_void,
            (MAX_FREQ_COEFFS as size_t)
                .wrapping_mul(::core::mem::size_of::<WORD16>() as size_t),
        );
    }
    err = ixheaacd_dec_envelope(
        ptr_header_data_ch_0,
        ptr_sbr_data_ch_0,
        ptr_prev_data_ch_0,
        ptr_prev_data_ch_1,
        ptr_common_tables,
        audio_object_type,
        ec_flag,
    ) as WORD32;
    if err != 0 {
        return err as IA_ERRORCODE;
    }
    err = ixheaacd_calc_noise_floor(
        ptr_header_data_ch_0,
        ptr_sbr_data_ch_0,
        ptr_prev_data_ch_0,
        audio_object_type,
    ) as WORD32;
    if err != 0 {
        return err as IA_ERRORCODE;
    }
    if (*ptr_sbr_data_ch_0).coupling_mode == 0
        && (usac_flag != 0 && audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int)
        || ldmps_present != 0
    {
        (*ptr_sbr_data_ch_0).num_noise_sfac = ((*(*ptr_header_data_ch_0)
            .pstr_freq_band_data)
            .num_nf_bands as core::ffi::c_int
            * (*ptr_sbr_data_ch_0).str_frame_info_details.num_noise_env
                as core::ffi::c_int) as WORD32;
        ixheaacd_dequant_esbr_env_data(
            ((*ptr_sbr_data_ch_0).flt_env_sf_arr).as_mut_ptr(),
            (*ptr_sbr_data_ch_0).num_env_sfac as WORD32,
            (*ptr_sbr_data_ch_0).num_noise_sfac,
            (*ptr_sbr_data_ch_0).amp_res as WORD32,
            ((*ptr_sbr_data_ch_0).flt_noise_floor).as_mut_ptr(),
        );
    }
    if !ptr_sbr_data_ch_1.is_null() {
        error_code = (*ptr_header_data_ch_0).err_flag;
        err = ixheaacd_dec_envelope(
            ptr_header_data_ch_1,
            ptr_sbr_data_ch_1,
            ptr_prev_data_ch_1,
            ptr_prev_data_ch_0,
            ptr_common_tables,
            audio_object_type,
            ec_flag,
        ) as WORD32;
        if err != 0 {
            return err as IA_ERRORCODE;
        }
        err = ixheaacd_calc_noise_floor(
            ptr_header_data_ch_1,
            ptr_sbr_data_ch_1,
            ptr_prev_data_ch_1,
            audio_object_type,
        ) as WORD32;
        if err != 0 {
            return err as IA_ERRORCODE;
        }
        if (*ptr_sbr_data_ch_1).coupling_mode == 0
            && (usac_flag != 0
                && audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int)
        {
            (*ptr_sbr_data_ch_1).num_noise_sfac = ((*(*ptr_header_data_ch_1)
                .pstr_freq_band_data)
                .num_nf_bands as core::ffi::c_int
                * (*ptr_sbr_data_ch_1).str_frame_info_details.num_noise_env
                    as core::ffi::c_int) as WORD32;
            ixheaacd_dequant_esbr_env_data(
                ((*ptr_sbr_data_ch_1).flt_env_sf_arr).as_mut_ptr(),
                (*ptr_sbr_data_ch_1).num_env_sfac as WORD32,
                (*ptr_sbr_data_ch_1).num_noise_sfac,
                (*ptr_sbr_data_ch_1).amp_res as WORD32,
                ((*ptr_sbr_data_ch_1).flt_noise_floor).as_mut_ptr(),
            );
        }
        if ec_flag != 0 {
            if (*ptr_header_data_ch_0).usac_flag == 0
                || audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int
            {
                if error_code == 0 && (*ptr_header_data_ch_0).err_flag != 0 {
                    memcpy(
                        ((*ptr_prev_data_ch_0).sfb_nrg_prev).as_mut_ptr()
                            as *mut core::ffi::c_void,
                        temp_sfb_nrg_prev.as_mut_ptr() as *const core::ffi::c_void,
                        (MAX_FREQ_COEFFS as size_t)
                            .wrapping_mul(::core::mem::size_of::<WORD16>() as size_t),
                    );
                    err = ixheaacd_dec_envelope(
                        ptr_header_data_ch_0,
                        ptr_sbr_data_ch_0,
                        ptr_prev_data_ch_0,
                        ptr_prev_data_ch_1,
                        ptr_common_tables,
                        audio_object_type,
                        ec_flag,
                    ) as WORD32;
                    if err != 0 {
                        return err as IA_ERRORCODE;
                    }
                }
            }
        } else if usac_flag == 0
            || audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int
        {
            if error_code == 0 && (*ptr_header_data_ch_0).err_flag != 0 {
                err = ixheaacd_dec_envelope(
                    ptr_header_data_ch_0,
                    ptr_sbr_data_ch_0,
                    ptr_prev_data_ch_0,
                    ptr_prev_data_ch_1,
                    ptr_common_tables,
                    audio_object_type,
                    ec_flag,
                ) as WORD32;
                if err != 0 {
                    return err as IA_ERRORCODE;
                }
            }
        }
        if (*ptr_sbr_data_ch_0).coupling_mode != 0 {
            (*ptr_sbr_data_ch_0).num_noise_sfac = ((*(*ptr_header_data_ch_1)
                .pstr_freq_band_data)
                .num_nf_bands as core::ffi::c_int
                * (*ptr_sbr_data_ch_1).str_frame_info_details.num_noise_env
                    as core::ffi::c_int) as WORD32;
            ixheaacd_sbr_env_dequant_coup_fix(
                ptr_header_data_ch_0,
                ptr_sbr_data_ch_0,
                ptr_sbr_data_ch_1,
                ptr_common_tables,
            );
            ixheaacd_sbr_env_dequant_coup(ptr_sbr_data_ch_0, ptr_sbr_data_ch_1);
        }
    }
    return 0 as IA_ERRORCODE;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_dec_envelope(
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut ptr_sbr_data: *mut ia_sbr_frame_info_data_struct,
    mut ptr_prev_data_ch_0: *mut ia_sbr_prev_frame_data_struct,
    mut ptr_prev_data_ch_1: *mut ia_sbr_prev_frame_data_struct,
    mut pstr_common_tables: *mut ixheaacd_misc_tables,
    mut audio_object_type: WORD32,
    mut ec_flag: WORD32,
) -> IA_ERRORCODE {
    let mut error_code: FLAG = 0;
    let mut err: WORD32 = 0;
    let mut env_sf_local_arr: [WORD16; 56] = [0; 56];
    let mut enh_sbr: WORD32 = (*ptr_header_data).enh_sbr as WORD32;
    let mut usac_flag: WORD32 = enh_sbr | (*ptr_header_data).usac_flag;
    let mut temp_1: WORD32 = (*ptr_prev_data_ch_0).end_position as WORD32
        - (*ptr_header_data).num_time_slots as WORD32;
    if temp_1 < 0 as core::ffi::c_int {
        return IA_FATAL_ERROR as IA_ERRORCODE;
    }
    temp_1 = (*ptr_sbr_data)
        .str_frame_info_details
        .border_vec[0 as core::ffi::c_int as usize] as WORD32 - temp_1;
    if (*ptr_header_data).err_flag_prev == 0 && (*ptr_header_data).err_flag == 0
        && temp_1 != 0 as core::ffi::c_int
    {
        if (*ptr_sbr_data).del_cod_dir_arr[0 as core::ffi::c_int as usize]
            as core::ffi::c_int == DTDF_DIR_TIME
        {
            (*ptr_header_data).err_flag = 1 as core::ffi::c_int as FLAG;
        } else {
            (*ptr_header_data).err_flag_prev = 1 as core::ffi::c_int as FLAG;
        }
    }
    if ec_flag != 0 {
        if (*ptr_header_data).err_flag_prev != 0 && (*ptr_header_data).err_flag == 0 {
            if (*ptr_sbr_data).del_cod_dir_arr[0 as core::ffi::c_int as usize]
                as core::ffi::c_int != 0 as core::ffi::c_int
            {
                (*ptr_header_data).err_flag = 1 as core::ffi::c_int as FLAG;
            }
        }
    }
    if ec_flag != 0 && (*ptr_header_data).err_flag != 0
        && ((*ptr_header_data).usac_flag == 0
            || audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int)
    {
        ixheaacd_lean_sbrconcealment(ptr_header_data, ptr_sbr_data, ptr_prev_data_ch_0);
        ixheaacd_process_del_cod_env_data(
            ptr_header_data,
            ptr_sbr_data,
            ptr_prev_data_ch_0,
        );
    } else if (*ptr_header_data).err_flag != 0
        && (usac_flag == 0 || audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int)
    {
        ixheaacd_lean_sbrconcealment(ptr_header_data, ptr_sbr_data, ptr_prev_data_ch_0);
        ixheaacd_process_del_cod_env_data(
            ptr_header_data,
            ptr_sbr_data,
            ptr_prev_data_ch_0,
        );
    } else {
        let mut num: WORD32 = (*(*ptr_header_data).pstr_freq_band_data)
            .num_sf_bands[HIGH as usize] as WORD32;
        if (*ptr_header_data).err_flag_prev != 0
            && (usac_flag == 0
                || audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int)
        {
            let mut ptr1: *mut WORD16 = 0 as *mut WORD16;
            let mut ptr2: *mut WORD16 = 0 as *mut WORD16;
            let mut i: WORD32 = 0;
            err = ixheaacd_wrong_timing_compensate(
                ptr_header_data,
                ptr_sbr_data,
                ptr_prev_data_ch_0,
                pstr_common_tables,
            );
            if err != 0 {
                return err as IA_ERRORCODE;
            }
            if (*ptr_sbr_data).coupling_mode
                != (*ptr_prev_data_ch_0).coupling_mode as WORD16 as core::ffi::c_int
            {
                if (*ptr_prev_data_ch_0).coupling_mode == COUPLING_BAL {
                    memcpy(
                        ((*ptr_prev_data_ch_0).sfb_nrg_prev).as_mut_ptr()
                            as *mut core::ffi::c_void,
                        ((*ptr_prev_data_ch_1).sfb_nrg_prev).as_mut_ptr()
                            as *const core::ffi::c_void,
                        (::core::mem::size_of::<WORD16>() as size_t)
                            .wrapping_mul(num as size_t),
                    );
                } else if (*ptr_sbr_data).coupling_mode == COUPLING_LEVEL {
                    ptr1 = ((*ptr_prev_data_ch_0).sfb_nrg_prev).as_mut_ptr();
                    ptr2 = ((*ptr_prev_data_ch_1).sfb_nrg_prev).as_mut_ptr();
                    i = 0 as core::ffi::c_int as WORD32;
                    while i < num {
                        *ptr1 = (*ptr1 as core::ffi::c_int + *ptr2 as core::ffi::c_int
                            >> 1 as core::ffi::c_int) as WORD16;
                        ptr2 = ptr2.offset(1);
                        ptr1 = ptr1.offset(1);
                        i += 1;
                    }
                } else if (*ptr_sbr_data).coupling_mode == COUPLING_BAL {
                    memset(
                        ((*ptr_prev_data_ch_0).sfb_nrg_prev).as_mut_ptr()
                            as *mut core::ffi::c_void,
                        SBR_ENERGY_PAN_OFFSET,
                        (::core::mem::size_of::<WORD16>() as size_t)
                            .wrapping_mul(num as size_t),
                    );
                }
            }
        }
        memcpy(
            env_sf_local_arr.as_mut_ptr() as *mut core::ffi::c_void,
            ((*ptr_prev_data_ch_0).sfb_nrg_prev).as_mut_ptr()
                as *const core::ffi::c_void,
            (::core::mem::size_of::<WORD16>() as size_t)
                .wrapping_mul(MAX_FREQ_COEFFS as size_t),
        );
        ixheaacd_process_del_cod_env_data(
            ptr_header_data,
            ptr_sbr_data,
            ptr_prev_data_ch_0,
        );
        if usac_flag == 0 || audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int {
            error_code = ixheaacd_check_env_data(
                ptr_header_data,
                ptr_sbr_data,
                ptr_prev_data_ch_0,
            ) as FLAG;
            if error_code != 0 {
                (*ptr_header_data).err_flag = 1 as core::ffi::c_int as FLAG;
                memcpy(
                    ((*ptr_prev_data_ch_0).sfb_nrg_prev).as_mut_ptr()
                        as *mut core::ffi::c_void,
                    env_sf_local_arr.as_mut_ptr() as *const core::ffi::c_void,
                    (::core::mem::size_of::<WORD16>() as size_t)
                        .wrapping_mul(MAX_FREQ_COEFFS as size_t),
                );
                err = ixheaacd_dec_envelope(
                    ptr_header_data,
                    ptr_sbr_data,
                    ptr_prev_data_ch_0,
                    ptr_prev_data_ch_1,
                    pstr_common_tables,
                    audio_object_type,
                    ec_flag,
                ) as WORD32;
                if err != 0 {
                    return err as IA_ERRORCODE;
                }
                return 0 as IA_ERRORCODE;
            }
        }
    }
    if ec_flag != 0 {
        if (*ptr_header_data).usac_flag == 0
            || audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int
        {
            ixheaacd_dequant_env_data(ptr_sbr_data, (*ptr_sbr_data).amp_res as WORD32);
        }
    } else if usac_flag == 0 || audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int {
        ixheaacd_dequant_env_data(ptr_sbr_data, (*ptr_sbr_data).amp_res as WORD32);
    }
    return 0 as IA_ERRORCODE;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_adj_timeslot(
    mut ptr_buf_real: *mut WORD32,
    mut ptr_buf_imag: *mut WORD32,
    mut ptr_filt_buf: *mut WORD16,
    mut ptr_filt_buf_noise: *mut WORD16,
    mut ptr_gain_buf: *mut WORD16,
    mut ptr_noise_floor: *mut WORD16,
    mut ptr_sine_lvl_buf: *mut WORD16,
    mut noise_floor_exp: WORD16,
    mut ptr_harm_index: *mut WORD16,
    mut sub_band_start: WORD16,
    mut num_sub_bands: WORD16,
    mut scale_change: WORD16,
    mut smooth_ratio: WORD16,
    mut num_noise_flg: FLAG,
    mut ptr_phase_index: *mut WORD16,
    mut ptr_sbr_tables: *mut ia_sbr_tables_struct,
) -> VOID {
    let mut k: WORD16 = 0;
    let mut ptr_smoothed_gain: *mut WORD16 = 0 as *mut WORD16;
    let mut ptr_smoothed_noise: *mut WORD16 = 0 as *mut WORD16;
    let mut direct_ratio: WORD16 = 0;
    let mut index: WORD32 = *ptr_phase_index as WORD32;
    let mut harm_idx: WORD32 = *ptr_harm_index as WORD32;
    let mut freq_inv_flag: WORD32 = 0;
    let mut ptr_rand_ph_buf: *const WORD32 = 0 as *const WORD32;
    let mut factor: WORD32 = 0 as WORD32;
    direct_ratio = ixheaac_sub16_sat(0x7fff as WORD16, smooth_ratio);
    freq_inv_flag = (sub_band_start as core::ffi::c_int & 1 as core::ffi::c_int)
        as WORD32;
    scale_change = (scale_change as core::ffi::c_int - 1 as core::ffi::c_int) as WORD16;
    ptr_rand_ph_buf = &mut *((*ptr_sbr_tables).sbr_rand_ph).offset(index as isize)
        as *mut WORD32;
    *ptr_phase_index = (index as core::ffi::c_int + num_sub_bands as core::ffi::c_int
        & SBR_NF_NO_RANDOM_VAL - 1 as core::ffi::c_int) as WORD16;
    if smooth_ratio != 0 {
        let mut ptr_filt_buf_local: *mut WORD16 = &mut *ptr_filt_buf
            .offset(0 as core::ffi::c_int as isize) as *mut WORD16;
        let mut ptr_gain_buf_local: *mut WORD16 = &mut *ptr_gain_buf
            .offset(0 as core::ffi::c_int as isize) as *mut WORD16;
        let mut ptr_filt_noise_local: *mut WORD16 = &mut *ptr_filt_buf_noise
            .offset(0 as core::ffi::c_int as isize) as *mut WORD16;
        let mut ptr_noise_floor_local: *mut WORD16 = &mut *ptr_noise_floor
            .offset(0 as core::ffi::c_int as isize) as *mut WORD16;
        let mut tmp: WORD16 = 0;
        let mut tmp1: WORD16 = 0;
        k = 0 as WORD16;
        while (k as core::ffi::c_int) < num_sub_bands as core::ffi::c_int {
            let fresh13 = ptr_gain_buf_local;
            ptr_gain_buf_local = ptr_gain_buf_local.offset(1);
            tmp = (ixheaac_mult16(smooth_ratio, *ptr_filt_buf_local) as core::ffi::c_int
                + ixheaac_mult16(direct_ratio, *fresh13) as core::ffi::c_int) as WORD16;
            ptr_gain_buf_local = ptr_gain_buf_local.offset(1);
            let fresh14 = ptr_noise_floor_local;
            ptr_noise_floor_local = ptr_noise_floor_local.offset(1);
            tmp1 = (ixheaac_mult16(smooth_ratio, *ptr_filt_noise_local)
                as core::ffi::c_int
                + ixheaac_mult16(direct_ratio, *fresh14) as core::ffi::c_int) as WORD16;
            ptr_noise_floor_local = ptr_noise_floor_local.offset(1);
            let fresh15 = ptr_filt_buf_local;
            ptr_filt_buf_local = ptr_filt_buf_local.offset(1);
            *fresh15 = ((tmp as core::ffi::c_int) << 1 as core::ffi::c_int) as WORD16;
            ptr_filt_buf_local = ptr_filt_buf_local.offset(1);
            let fresh16 = ptr_filt_noise_local;
            ptr_filt_noise_local = ptr_filt_noise_local.offset(1);
            *fresh16 = ((tmp1 as core::ffi::c_int) << 1 as core::ffi::c_int) as WORD16;
            k += 1;
        }
        ptr_smoothed_gain = ptr_filt_buf;
        ptr_smoothed_noise = ptr_filt_buf_noise;
        factor = 1 as core::ffi::c_int as WORD32;
    } else {
        ptr_smoothed_gain = ptr_gain_buf;
        ptr_smoothed_noise = ptr_noise_floor;
        factor = 2 as core::ffi::c_int as WORD32;
    }
    match harm_idx {
        0 | 2 => {
            ixheaacd_harm_idx_zerotwo(
                num_noise_flg,
                num_sub_bands,
                ptr_buf_real,
                ptr_buf_imag,
                ptr_smoothed_gain,
                ptr_smoothed_noise,
                factor as WORD,
                ptr_gain_buf,
                scale_change,
                ptr_rand_ph_buf,
                ptr_sine_lvl_buf,
                noise_floor_exp,
                harm_idx,
            );
        }
        1 | 3 => {
            ixheaacd_harm_idx_onethree(
                num_noise_flg,
                num_sub_bands,
                ptr_buf_real,
                ptr_buf_imag,
                ptr_smoothed_gain,
                ptr_smoothed_noise,
                factor as WORD,
                ptr_gain_buf,
                scale_change,
                ptr_rand_ph_buf,
                ptr_sine_lvl_buf,
                noise_floor_exp,
                freq_inv_flag as WORD,
                harm_idx,
            );
        }
        _ => {}
    }
    *ptr_harm_index = (harm_idx as core::ffi::c_int + 1 as core::ffi::c_int
        & 3 as core::ffi::c_int) as WORD16;
}
pub const IA_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_FATAL_ERROR: core::ffi::c_uint = 0x80000000 as core::ffi::c_uint;
pub const MAX_16: WORD16 = 0x7fff as core::ffi::c_int as WORD16;
pub const SHORT_BITS: core::ffi::c_int = 16 as core::ffi::c_int;
