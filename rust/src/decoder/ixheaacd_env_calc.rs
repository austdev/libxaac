extern "C" {
    fn ixheaacd_fix_mant_div(
        a_m: WORD16,
        b_m: WORD16,
        ptr_result: *mut WORD16,
        pstr_common_tables: *mut ixheaacd_misc_tables,
    ) -> WORD32;
    fn ixheaacd_fix_mant_exp_sqrt(mant: *mut WORD16, sqrt_table: *mut WORD16) -> VOID;
    fn ixheaacd_adj_timeslot(
        ptr_buf_real: *mut WORD32,
        ptr_buf_imag: *mut WORD32,
        ptr_filt_buf: *mut WORD16,
        ptr_filt_buf_noise: *mut WORD16,
        ptr_gain_buf: *mut WORD16,
        ptr_noise_floor: *mut WORD16,
        ptr_sine_lvl_buf: *mut WORD16,
        noise_floor_exp: WORD16,
        ptr_harm_index: *mut WORD16,
        sub_band_start: WORD16,
        num_sub_bands: WORD16,
        scale_change: WORD16,
        smooth_ratio: WORD16,
        num_noise_flg: FLAG,
        ptr_phase_index: *mut WORD16,
        ptr_sbr_tables: *mut ia_sbr_tables_struct,
    ) -> VOID;
    fn ixheaacd_map_sineflags(
        freq_band_table: *mut WORD16,
        num_sf_bands: WORD16,
        add_harmonics: *mut FLAG,
        harm_flags_prev: *mut WORD8,
        transient_env: WORD16,
        sine_mapped_matrix: *mut WORD8,
    ) -> VOID;
    static mut ixheaacd_conv_ergtoamplitudelp: Option<
        unsafe extern "C" fn(
            WORD32,
            WORD16,
            *mut WORD16,
            *mut WORD16,
            *mut WORD16,
            *mut WORD16,
        ) -> VOID,
    >;
    static mut ixheaacd_conv_ergtoamplitude: Option<
        unsafe extern "C" fn(
            WORD32,
            WORD16,
            *mut WORD16,
            *mut WORD16,
            *mut WORD16,
            *mut WORD16,
        ) -> VOID,
    >;
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
    static mut ixheaacd_ixheaacd_expsubbandsamples: Option<
        unsafe extern "C" fn(
            *mut *mut WORD32,
            *mut *mut WORD32,
            WORD32,
            WORD32,
            WORD32,
            WORD32,
            FLAG,
        ) -> WORD16,
    >;
    static mut ixheaacd_enery_calc_per_subband: Option<
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
    >;
    static mut ixheaacd_harm_idx_zerotwolp: Option<
        unsafe extern "C" fn(
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
    >;
}
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
pub type IA_ERRORCODE = WORD32;
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
pub struct ia_sbr_calc_env_struct {
    pub filt_buf_me: *mut WORD16,
    pub filt_buf_noise_m: *mut WORD16,
    pub filt_buf_noise_e: WORD32,
    pub start_up: FLAG,
    pub ph_index: WORD16,
    pub tansient_env_prev: WORD16,
    pub harm_flags_prev: [WORD8; 56],
    pub harm_index: WORD16,
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
pub struct ixheaacd_lpp_trans_patch {
    pub num_patches: WORD32,
    pub start_subband: [WORD32; 7],
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
unsafe extern "C" fn ixheaac_shr32_dir(mut a: WORD32, mut b: WORD) -> WORD32 {
    let mut out_val: WORD32 = 0;
    if b < 0 as core::ffi::c_int {
        out_val = ixheaac_shl32(a, -b);
    } else {
        out_val = ixheaac_shr32(a, b);
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
unsafe extern "C" fn ixheaac_mult16x16in32_shl_sat(
    mut a: WORD16,
    mut b: WORD16,
) -> WORD32 {
    let mut product: WORD32 = 0;
    product = a as WORD32 * b as WORD32;
    if product != 0x40000000 as core::ffi::c_long as WORD32 {
        product = ixheaac_shl32(product, 1 as WORD);
    } else {
        product = MAX_32;
    }
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
unsafe extern "C" fn ixheaac_abs32(mut a: WORD32) -> WORD32 {
    let mut abs_val: WORD32 = 0;
    abs_val = a;
    if a < 0 as core::ffi::c_int {
        abs_val = -a;
    }
    return abs_val;
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
unsafe extern "C" fn ixheaac_mac16x16in32_sat(
    mut a: WORD32,
    mut b: WORD16,
    mut c: WORD16,
) -> WORD32 {
    let mut acc: WORD32 = 0;
    acc = ixheaac_mult16x16in32(b, c);
    acc = ixheaac_add32_sat(a, acc);
    return acc;
}
#[inline]
unsafe extern "C" fn ixheaac_mac16x16in32_shl_sat(
    mut a: WORD32,
    mut b: WORD16,
    mut c: WORD16,
) -> WORD32 {
    let mut acc: WORD32 = 0;
    acc = ixheaac_mult16x16in32_shl_sat(b, c);
    acc = ixheaac_add32_sat(a, acc);
    return acc;
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
unsafe extern "C" fn ixheaac_sub16(mut op1: WORD16, mut op2: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = (op1 as core::ffi::c_int - op2 as core::ffi::c_int) as WORD16;
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_mult16_shl(mut op1: WORD16, mut op2: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = (op1 as WORD32 * op2 as WORD32 >> 15 as core::ffi::c_int) as WORD16;
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_mult16_shl_sat(mut op1: WORD16, mut op2: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    let mut temp: WORD32 = 0;
    temp = op1 as WORD32 * op2 as WORD32 >> 15 as core::ffi::c_int;
    var_out = ixheaac_sat16(temp);
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_shl16_sat(mut op1: WORD16, mut shift: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    let mut temp: WORD32 = 0;
    if shift as core::ffi::c_int > 15 as core::ffi::c_int {
        shift = 15 as WORD16;
    }
    temp = (op1 as core::ffi::c_int) << shift as core::ffi::c_int;
    var_out = ixheaac_sat16(temp);
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_shr16(mut op1: WORD16, mut shift: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = (op1 as core::ffi::c_int >> shift as core::ffi::c_int) as WORD16;
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_max16(mut op1: WORD16, mut op2: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = (if op1 as core::ffi::c_int > op2 as core::ffi::c_int {
        op1 as core::ffi::c_int
    } else {
        op2 as core::ffi::c_int
    }) as WORD16;
    return var_out;
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
unsafe extern "C" fn ixheaac_extract16h(mut var: WORD32) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = (var >> 16 as core::ffi::c_int) as WORD16;
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_extract16l(mut var: WORD32) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = var as WORD16;
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
#[inline]
unsafe extern "C" fn ixheaac_shl32_dir_sat_limit(mut a: WORD32, mut b: WORD) -> WORD32 {
    let mut out_val: WORD32 = 0;
    if b < 0 as core::ffi::c_int {
        b = -b;
        b = ixheaac_min32(b as WORD32, 31 as WORD32) as WORD;
        out_val = ixheaac_shr32(a, b);
    } else {
        out_val = ixheaac_shl32_sat(a, b);
    }
    return out_val;
}
pub const SHORT_BITS: core::ffi::c_int = 16 as core::ffi::c_int;
pub const SBR_UPSAMPLE_FAC: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_FRAME_SIZE: core::ffi::c_int = 1024 as core::ffi::c_int;
pub const NO_SYNTHESIS_CHANNELS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const NO_ANALYSIS_CHANNELS: core::ffi::c_int = NO_SYNTHESIS_CHANNELS
    / SBR_UPSAMPLE_FAC;
pub const MAX_NOISE_ENVELOPES: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SBR_TIME_STEP: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_GAIN_EXP: core::ffi::c_int = 34 as core::ffi::c_int;
pub const MAX_COLS: core::ffi::c_int = MAX_FRAME_SIZE / NO_ANALYSIS_CHANNELS;
pub const MAX_OV_COLS: core::ffi::c_int = 6 as core::ffi::c_int;
pub const MAX_ENV_COLS: core::ffi::c_int = MAX_COLS + MAX_OV_COLS;
pub const SBR_TIME_SLOTS: core::ffi::c_int = MAX_COLS / SBR_TIME_STEP;
pub const PS_STEREO: core::ffi::c_int = 3 as core::ffi::c_int;
pub const SBR_NF_NO_RANDOM_VAL: core::ffi::c_int = 512 as core::ffi::c_int;
pub const IA_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_FATAL_ERROR: core::ffi::c_uint = 0x80000000 as core::ffi::c_uint;
pub const EXP_BITS: core::ffi::c_int = 6 as core::ffi::c_int;
pub const MASK_M: core::ffi::c_int = (((1 as core::ffi::c_int) << SHORT_BITS - EXP_BITS)
    - 1 as core::ffi::c_int) << EXP_BITS;
pub const MASK_FOR_EXP: core::ffi::c_int = ((1 as core::ffi::c_int) << EXP_BITS)
    - 1 as core::ffi::c_int;
pub const NRG_EXP_OFFSET: core::ffi::c_int = 16 as core::ffi::c_int;
pub const NOISE_EXP_OFFSET: core::ffi::c_int = 38 as core::ffi::c_int;
pub const HIGH: core::ffi::c_int = 1 as core::ffi::c_int;
pub const FACTOR: core::ffi::c_int = 0x10b0000 as core::ffi::c_int
    * 2 as core::ffi::c_int;
unsafe extern "C" fn ixheaacd_alias_reduction(
    mut deg_patched: *mut WORD16,
    mut nrg_gain: *mut WORD16,
    mut nrg_est: *mut WORD16,
    mut alias_red_buf: *mut WORD8,
    mut num_sub_bands: WORD32,
    mut pstr_common_tables: *mut ixheaacd_misc_tables,
) -> VOID {
    let mut group: WORD32 = 0;
    let mut grouping: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut num_groups: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut f_group_vec: [WORD16; 56] = [0; 56];
    let mut ptr_f_group_vec: *mut WORD16 = 0 as *mut WORD16;
    grouping = 0 as core::ffi::c_int as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    k = 0 as core::ffi::c_int as WORD32;
    while k < num_sub_bands as core::ffi::c_int - 1 as core::ffi::c_int {
        if *deg_patched.offset((k as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
            as core::ffi::c_int != 0 as core::ffi::c_int
            && *alias_red_buf.offset(k as isize) as core::ffi::c_int != 0
        {
            if grouping == 0 as core::ffi::c_int {
                f_group_vec[i as usize] = k as WORD16;
                grouping = 1 as core::ffi::c_int as WORD32;
                i += 1;
            } else if f_group_vec[(i as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize] as core::ffi::c_int + 3 as core::ffi::c_int == k
            {
                f_group_vec[i as usize] = (k as core::ffi::c_int + 1 as core::ffi::c_int)
                    as WORD16;
                grouping = 0 as core::ffi::c_int as WORD32;
                i += 1;
            }
        } else if grouping != 0 {
            grouping = 0 as core::ffi::c_int as WORD32;
            f_group_vec[i as usize] = k as WORD16;
            if *alias_red_buf.offset(k as isize) != 0 {
                f_group_vec[i as usize] = (k as core::ffi::c_int + 1 as core::ffi::c_int)
                    as WORD16;
            }
            i += 1;
        }
        k += 1;
    }
    if grouping != 0 {
        f_group_vec[i as usize] = num_sub_bands as WORD16;
        i += 1;
    }
    num_groups = i >> 1 as core::ffi::c_int;
    ptr_f_group_vec = f_group_vec.as_mut_ptr();
    group = num_groups;
    while group != 0 as core::ffi::c_int {
        let mut nrg_amp_mant: WORD16 = 0;
        let mut nrg_amp_exp: WORD16 = 0;
        let mut nrgMod_m: WORD16 = 0;
        let mut nrgMod_e: WORD16 = 0;
        let mut grp_gain_mant: WORD16 = 0;
        let mut grp_gain_exp: WORD16 = 0;
        let mut compensation_m: WORD16 = 0;
        let mut compensation_e: WORD16 = 0;
        let mut nrg_mod_mant: WORD32 = 0;
        let mut nrg_mod_exp: WORD32 = 0;
        let fresh16 = ptr_f_group_vec;
        ptr_f_group_vec = ptr_f_group_vec.offset(1);
        let mut start_grp: WORD32 = *fresh16 as WORD32;
        let fresh17 = ptr_f_group_vec;
        ptr_f_group_vec = ptr_f_group_vec.offset(1);
        let mut stop_grp: WORD32 = *fresh17 as WORD32;
        ixheaacd_avggain_calc(
            nrg_est,
            nrg_gain,
            start_grp,
            stop_grp,
            &mut nrg_amp_mant,
            &mut nrg_amp_exp,
            &mut grp_gain_mant,
            &mut grp_gain_exp,
            pstr_common_tables,
            1 as WORD32,
        );
        nrg_mod_mant = 0 as core::ffi::c_int as WORD32;
        nrg_mod_exp = 0 as core::ffi::c_int as WORD32;
        let mut ptr_nrg_gain_mant: *mut WORD16 = &mut *nrg_gain
            .offset((2 as WORD32 * start_grp) as isize) as *mut WORD16;
        k = start_grp;
        while k < stop_grp {
            let mut tmp_mant: WORD32 = 0;
            let mut tmp_gain_mant: WORD32 = 0;
            let mut gain_m: WORD32 = 0;
            let mut tmp_e: WORD32 = 0;
            let mut tmp_gain_exp: WORD32 = 0;
            let mut one_minus_alpha: WORD16 = 0;
            let mut alpha: WORD16 = *deg_patched.offset(k as isize);
            if k < num_sub_bands as core::ffi::c_int - 1 as core::ffi::c_int {
                alpha = ixheaac_max16(
                    *deg_patched
                        .offset(
                            (k as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                        ),
                    alpha,
                );
            }
            gain_m = (alpha as core::ffi::c_int * grp_gain_mant as core::ffi::c_int)
                as WORD32;
            one_minus_alpha = (0x7fff as core::ffi::c_int - alpha as core::ffi::c_int)
                as WORD16;
            tmp_gain_mant = *ptr_nrg_gain_mant as WORD32;
            tmp_gain_exp = *ptr_nrg_gain_mant.offset(1 as core::ffi::c_int as isize)
                as WORD32;
            let mut exp_diff: WORD32 = 0;
            tmp_gain_mant = one_minus_alpha as WORD32 * tmp_gain_mant
                >> 15 as core::ffi::c_int;
            exp_diff = grp_gain_exp as WORD32 - tmp_gain_exp;
            if exp_diff >= 0 as core::ffi::c_int {
                tmp_gain_exp = grp_gain_exp as WORD32;
                tmp_gain_mant = ixheaac_shr32(tmp_gain_mant, exp_diff as WORD);
                tmp_gain_mant = (gain_m >> 15 as core::ffi::c_int) + tmp_gain_mant;
            } else {
                tmp_gain_mant = ixheaac_shr32(gain_m, 15 as WORD - exp_diff as WORD)
                    + tmp_gain_mant;
            }
            let fresh18 = ptr_nrg_gain_mant;
            ptr_nrg_gain_mant = ptr_nrg_gain_mant.offset(1);
            *fresh18 = tmp_gain_mant as WORD16;
            let fresh19 = ptr_nrg_gain_mant;
            ptr_nrg_gain_mant = ptr_nrg_gain_mant.offset(1);
            *fresh19 = tmp_gain_exp as WORD16;
            tmp_mant = (tmp_gain_mant as core::ffi::c_int
                * *nrg_est.offset((2 as WORD32 * k) as isize) as core::ffi::c_int
                >> 16 as core::ffi::c_int) as WORD32;
            tmp_e = (tmp_gain_exp as core::ffi::c_int
                + *nrg_est
                    .offset(
                        (2 as core::ffi::c_int * k as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    ) as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
            let mut exp_diff_0: WORD32 = 0;
            exp_diff_0 = tmp_e - nrg_mod_exp;
            if exp_diff_0 >= 0 as core::ffi::c_int {
                nrg_mod_mant = tmp_mant
                    + ixheaac_shr32(nrg_mod_mant, exp_diff_0 as WORD);
                nrg_mod_exp = tmp_e;
            } else {
                exp_diff_0 = -exp_diff_0;
                nrg_mod_mant = ixheaac_shr32(tmp_mant, exp_diff_0 as WORD)
                    + nrg_mod_mant;
            }
            k += 1;
        }
        let mut norm_val: WORD32 = 0;
        norm_val = (16 as WORD - ixheaac_pnorm32(nrg_mod_mant)) as WORD32;
        if norm_val > 0 as core::ffi::c_int {
            nrg_mod_mant >>= norm_val;
            nrg_mod_exp += norm_val;
        }
        nrgMod_m = nrg_mod_mant as WORD16;
        nrgMod_e = nrg_mod_exp as WORD16;
        compensation_e = ixheaacd_fix_mant_div(
            nrg_amp_mant,
            nrgMod_m,
            &mut compensation_m,
            pstr_common_tables,
        ) as WORD16;
        compensation_e = (compensation_e as core::ffi::c_int
            + (nrg_amp_exp as core::ffi::c_int - nrgMod_e as core::ffi::c_int
                + 1 as core::ffi::c_int + 1 as core::ffi::c_int)) as WORD16;
        let mut ptr_nrg_gain_mant_0: *mut WORD16 = &mut *nrg_gain
            .offset((2 as WORD32 * start_grp) as isize) as *mut WORD16;
        k = stop_grp - start_grp;
        while k != 0 as core::ffi::c_int {
            let mut temp1: WORD16 = 0;
            let mut temp2: WORD16 = 0;
            temp1 = *ptr_nrg_gain_mant_0;
            temp2 = *ptr_nrg_gain_mant_0.offset(1 as core::ffi::c_int as isize);
            temp1 = (temp1 as core::ffi::c_int * compensation_m as core::ffi::c_int
                >> 16 as core::ffi::c_int) as WORD16;
            temp2 = (temp2 as core::ffi::c_int + compensation_e as core::ffi::c_int)
                as WORD16;
            let fresh20 = ptr_nrg_gain_mant_0;
            ptr_nrg_gain_mant_0 = ptr_nrg_gain_mant_0.offset(1);
            *fresh20 = temp1;
            let fresh21 = ptr_nrg_gain_mant_0;
            ptr_nrg_gain_mant_0 = ptr_nrg_gain_mant_0.offset(1);
            *fresh21 = temp2;
            k -= 1;
        }
        group -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_noiselimiting(
    mut pstr_freq_band_data: *mut ia_freq_band_data_struct,
    mut skip_bands: WORD32,
    mut ptr_enrg_orig: *mut WORD16,
    mut nrg_est: *mut WORD16,
    mut nrg_gain: *mut WORD16,
    mut noise_level_mant: *mut WORD16,
    mut nrg_sine: *mut WORD16,
    mut ptr_limit_gain_table: *mut WORD16,
    mut noise_absc_flag: FLAG,
    mut pstr_common_tables: *mut ixheaacd_misc_tables,
) -> VOID {
    let mut c: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut temp_val: WORD32 = 0;
    let fresh26 = ptr_limit_gain_table;
    ptr_limit_gain_table = ptr_limit_gain_table.offset(1);
    let mut limit_gain_mant: WORD16 = *fresh26;
    let mut limit_gain_exp: WORD16 = *ptr_limit_gain_table;
    c = 0 as core::ffi::c_int as WORD32;
    while c < (*pstr_freq_band_data).num_lf_bands as core::ffi::c_int {
        let mut max_gain_mant: WORD16 = 0;
        let mut sum_orig_mant: WORD16 = 0;
        let mut sum_orig_exp: WORD16 = 0;
        let mut max_gain_exp: WORD16 = 0;
        let mut max_temp: WORD32 = 0;
        let mut start_band: WORD32 = 0 as WORD32;
        let mut stop_band: WORD32 = 0 as WORD32;
        if (*pstr_freq_band_data).freq_band_tbl_lim[c as usize] as core::ffi::c_int
            > skip_bands
        {
            start_band = (*pstr_freq_band_data).freq_band_tbl_lim[c as usize] as WORD32
                - skip_bands;
        }
        if (*pstr_freq_band_data)
            .freq_band_tbl_lim[(c as core::ffi::c_int + 1 as core::ffi::c_int) as usize]
            as core::ffi::c_int > skip_bands
        {
            stop_band = (*pstr_freq_band_data)
                .freq_band_tbl_lim[(c as core::ffi::c_int + 1 as core::ffi::c_int)
                as usize] as WORD32 - skip_bands;
        }
        if start_band < stop_band {
            ixheaacd_avggain_calc(
                ptr_enrg_orig,
                nrg_est,
                start_band,
                stop_band,
                &mut sum_orig_mant,
                &mut sum_orig_exp,
                &mut max_gain_mant,
                &mut max_gain_exp,
                pstr_common_tables,
                0 as WORD32,
            );
            max_temp = ixheaac_mult16x16in32_shl(max_gain_mant, limit_gain_mant);
            max_gain_exp = (max_gain_exp as core::ffi::c_int
                + limit_gain_exp as core::ffi::c_int) as WORD16;
            temp_val = ixheaac_norm32(max_temp) as WORD32;
            max_gain_exp = (max_gain_exp as WORD32 - temp_val) as WORD16;
            max_gain_mant = (max_temp << temp_val >> 16 as core::ffi::c_int) as WORD16;
            if max_gain_exp as core::ffi::c_int >= MAX_GAIN_EXP {
                max_gain_mant = 0x3000 as WORD16;
                max_gain_exp = MAX_GAIN_EXP as WORD16;
            }
            let mut ptr_nrg_gain: *mut WORD16 = &mut *nrg_gain
                .offset((2 as WORD32 * start_band) as isize) as *mut WORD16;
            let mut p_noise_level: *mut WORD16 = &mut *noise_level_mant
                .offset((2 as WORD32 * start_band) as isize) as *mut WORD16;
            k = stop_band - start_band;
            while k != 0 as core::ffi::c_int {
                let mut noise_amp_mant: WORD16 = 0;
                let mut noise_amp_exp: WORD16 = 0;
                let mut t_gain_mant: WORD16 = *ptr_nrg_gain;
                let mut t_gain_exp: WORD16 = *ptr_nrg_gain
                    .offset(1 as core::ffi::c_int as isize);
                if t_gain_exp as core::ffi::c_int > max_gain_exp as core::ffi::c_int
                    || t_gain_exp as core::ffi::c_int == max_gain_exp as core::ffi::c_int
                        && t_gain_mant as core::ffi::c_int
                            > max_gain_mant as core::ffi::c_int
                {
                    noise_amp_exp = ixheaacd_fix_mant_div(
                        max_gain_mant,
                        t_gain_mant,
                        &mut noise_amp_mant,
                        pstr_common_tables,
                    ) as WORD16;
                    noise_amp_exp = (noise_amp_exp as core::ffi::c_int
                        + (max_gain_exp as core::ffi::c_int
                            - t_gain_exp as core::ffi::c_int + 1 as core::ffi::c_int))
                        as WORD16;
                    *p_noise_level = ixheaac_extract16h(
                        ixheaac_shl32_dir_sat_limit(
                            ixheaac_mult16x16in32_shl(*p_noise_level, noise_amp_mant),
                            noise_amp_exp as WORD,
                        ),
                    );
                    *ptr_nrg_gain = max_gain_mant;
                    *ptr_nrg_gain.offset(1 as core::ffi::c_int as isize) = max_gain_exp;
                }
                ptr_nrg_gain = ptr_nrg_gain.offset(2 as core::ffi::c_int as isize);
                p_noise_level = p_noise_level.offset(2 as core::ffi::c_int as isize);
                k -= 1;
            }
            let mut boost_gain_mant: WORD16 = 0;
            let mut boost_gain_exp: WORD16 = 0;
            let mut accu_m: WORD16 = 0;
            let mut accu_e: WORD16 = 0;
            let mut accu_m_t: WORD32 = 0;
            let mut accu_e_t: WORD32 = 0;
            let mut ptr_nrg_gain_0: *mut WORD16 = &mut *nrg_gain
                .offset((2 as WORD32 * start_band) as isize) as *mut WORD16;
            let mut ptr_enrg_est_buf: *mut WORD16 = &mut *nrg_est
                .offset((2 as WORD32 * start_band) as isize) as *mut WORD16;
            let mut p_noise_level_0: *mut WORD16 = &mut *noise_level_mant
                .offset((2 as WORD32 * start_band) as isize) as *mut WORD16;
            let mut p_nrg_sine: *mut WORD16 = &mut *nrg_sine
                .offset((2 as WORD32 * start_band) as isize) as *mut WORD16;
            accu_m_t = 0 as core::ffi::c_int as WORD32;
            accu_e_t = 0 as core::ffi::c_int as WORD32;
            k = stop_band - start_band;
            while k != 0 as core::ffi::c_int {
                let mut tmp_mant: WORD32 = 0;
                let mut tmp_e: WORD32 = 0;
                let fresh27 = ptr_nrg_gain_0;
                ptr_nrg_gain_0 = ptr_nrg_gain_0.offset(1);
                tmp_mant = *fresh27 as WORD32;
                let fresh28 = ptr_nrg_gain_0;
                ptr_nrg_gain_0 = ptr_nrg_gain_0.offset(1);
                tmp_e = *fresh28 as WORD32;
                let fresh29 = ptr_enrg_est_buf;
                ptr_enrg_est_buf = ptr_enrg_est_buf.offset(1);
                tmp_mant = (tmp_mant as core::ffi::c_int * *fresh29 as core::ffi::c_int)
                    as WORD32;
                let fresh30 = ptr_enrg_est_buf;
                ptr_enrg_est_buf = ptr_enrg_est_buf.offset(1);
                tmp_e = (tmp_e as core::ffi::c_int + *fresh30 as core::ffi::c_int)
                    as WORD32;
                tmp_mant = tmp_mant >> 15 as core::ffi::c_int;
                let mut exp_diff: WORD32 = 0;
                exp_diff = tmp_e - accu_e_t;
                if exp_diff >= 0 as core::ffi::c_int {
                    accu_m_t = tmp_mant + ixheaac_shr32(accu_m_t, exp_diff as WORD);
                    accu_e_t = tmp_e;
                } else {
                    exp_diff = -exp_diff;
                    accu_m_t = ixheaac_shr32(tmp_mant, exp_diff as WORD) + accu_m_t;
                }
                if *p_nrg_sine.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                    != 0 as core::ffi::c_int
                {
                    let mut exp_diff_0: WORD32 = *p_nrg_sine
                        .offset(1 as core::ffi::c_int as isize) as WORD32 - accu_e_t;
                    if exp_diff_0 >= 0 as core::ffi::c_int {
                        accu_m_t = *p_nrg_sine.offset(0 as core::ffi::c_int as isize)
                            as WORD32 + ixheaac_shr32(accu_m_t, exp_diff_0 as WORD);
                        accu_e_t = *p_nrg_sine.offset(1 as core::ffi::c_int as isize)
                            as WORD32;
                    } else {
                        exp_diff_0 = -exp_diff_0;
                        accu_m_t = accu_m_t
                            + ixheaac_shr32(
                                *p_nrg_sine.offset(0 as core::ffi::c_int as isize)
                                    as WORD32,
                                exp_diff_0 as WORD,
                            );
                    }
                } else if noise_absc_flag == 0 as core::ffi::c_int {
                    let mut exp_diff_1: WORD32 = *p_noise_level_0
                        .offset(1 as core::ffi::c_int as isize) as WORD32 - accu_e_t;
                    if exp_diff_1 >= 0 as core::ffi::c_int {
                        accu_m_t = *p_noise_level_0
                            .offset(0 as core::ffi::c_int as isize) as WORD32
                            + ixheaac_shr32(accu_m_t, exp_diff_1 as WORD);
                        accu_e_t = *p_noise_level_0
                            .offset(1 as core::ffi::c_int as isize) as WORD32;
                    } else {
                        exp_diff_1 = -exp_diff_1;
                        accu_m_t = accu_m_t
                            + ixheaac_shr32(
                                *p_noise_level_0.offset(0 as core::ffi::c_int as isize)
                                    as WORD32,
                                exp_diff_1 as WORD,
                            );
                    }
                }
                p_noise_level_0 = p_noise_level_0.offset(2 as core::ffi::c_int as isize);
                p_nrg_sine = p_nrg_sine.offset(2 as core::ffi::c_int as isize);
                k -= 1;
            }
            let mut norm_val: WORD32 = 0;
            norm_val = (16 as WORD - ixheaac_norm32(accu_m_t)) as WORD32;
            if norm_val > 0 as core::ffi::c_int {
                accu_m_t >>= norm_val;
                accu_e_t += norm_val;
            }
            accu_m = accu_m_t as WORD16;
            accu_e = accu_e_t as WORD16;
            boost_gain_exp = ixheaacd_fix_mant_div(
                sum_orig_mant,
                accu_m,
                &mut boost_gain_mant,
                pstr_common_tables,
            ) as WORD16;
            boost_gain_exp = (boost_gain_exp as core::ffi::c_int
                + (sum_orig_exp as core::ffi::c_int - accu_e as core::ffi::c_int
                    + 1 as core::ffi::c_int)) as WORD16;
            if boost_gain_exp as core::ffi::c_int > 2 as core::ffi::c_int
                || boost_gain_exp as core::ffi::c_int == 2 as core::ffi::c_int
                    && boost_gain_mant as core::ffi::c_int > 0x5061 as core::ffi::c_int
            {
                boost_gain_mant = 0x5061 as WORD16;
                boost_gain_exp = 2 as WORD16;
            }
            ptr_nrg_gain_0 = &mut *nrg_gain.offset((2 as WORD32 * start_band) as isize)
                as *mut WORD16;
            p_noise_level_0 = &mut *noise_level_mant
                .offset((2 as WORD32 * start_band) as isize) as *mut WORD16;
            p_nrg_sine = &mut *nrg_sine.offset((2 as WORD32 * start_band) as isize)
                as *mut WORD16;
            k = stop_band - start_band;
            while k != 0 as core::ffi::c_int {
                let mut temp1: WORD16 = 0;
                let mut temp2: WORD16 = 0;
                let mut temp3: WORD16 = 0;
                temp1 = *ptr_nrg_gain_0;
                temp2 = *p_nrg_sine;
                temp3 = *p_noise_level_0;
                temp1 = ixheaac_mult16_shl(temp1, boost_gain_mant);
                temp2 = ixheaac_mult16_shl(temp2, boost_gain_mant);
                temp3 = ixheaac_mult16_shl(temp3, boost_gain_mant);
                let fresh31 = ptr_nrg_gain_0;
                ptr_nrg_gain_0 = ptr_nrg_gain_0.offset(1);
                *fresh31 = temp1;
                let fresh32 = p_nrg_sine;
                p_nrg_sine = p_nrg_sine.offset(1);
                *fresh32 = temp2;
                let fresh33 = p_noise_level_0;
                p_noise_level_0 = p_noise_level_0.offset(1);
                *fresh33 = temp3;
                temp1 = *ptr_nrg_gain_0;
                temp2 = *p_nrg_sine;
                temp3 = *p_noise_level_0;
                temp1 = (temp1 as core::ffi::c_int + boost_gain_exp as core::ffi::c_int)
                    as WORD16;
                temp2 = (temp2 as core::ffi::c_int + boost_gain_exp as core::ffi::c_int)
                    as WORD16;
                temp3 = (temp3 as core::ffi::c_int + boost_gain_exp as core::ffi::c_int)
                    as WORD16;
                let fresh34 = ptr_nrg_gain_0;
                ptr_nrg_gain_0 = ptr_nrg_gain_0.offset(1);
                *fresh34 = temp1;
                let fresh35 = p_nrg_sine;
                p_nrg_sine = p_nrg_sine.offset(1);
                *fresh35 = temp2;
                let fresh36 = p_noise_level_0;
                p_noise_level_0 = p_noise_level_0.offset(1);
                *fresh36 = temp3;
                k -= 1;
            }
        }
        c += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_conv_ergtoamplitudelp_dec(
    mut bands: WORD32,
    mut noise_e: WORD16,
    mut nrg_sine: *mut WORD16,
    mut nrg_gain: *mut WORD16,
    mut noise_level_mant: *mut WORD16,
    mut sqrt_table: *mut WORD16,
) -> VOID {
    let mut k: WORD32 = 0;
    k = 0 as core::ffi::c_int as WORD32;
    while k < bands {
        let mut shift: WORD32 = 0;
        ixheaacd_fix_mant_exp_sqrt(
            &mut *nrg_sine.offset((2 as WORD32 * k) as isize),
            sqrt_table,
        );
        ixheaacd_fix_mant_exp_sqrt(
            &mut *nrg_gain.offset((2 as WORD32 * k) as isize),
            sqrt_table,
        );
        ixheaacd_fix_mant_exp_sqrt(
            &mut *noise_level_mant.offset((2 as WORD32 * k) as isize),
            sqrt_table,
        );
        shift = (noise_e as core::ffi::c_int
            - *noise_level_mant
                .offset(
                    (2 as core::ffi::c_int * k as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) as core::ffi::c_int) as WORD32;
        shift = (shift as core::ffi::c_int - 4 as core::ffi::c_int) as WORD32;
        if shift > 0 as core::ffi::c_int {
            *noise_level_mant.offset((2 as WORD32 * k) as isize) = (*noise_level_mant
                .offset((2 as WORD32 * k) as isize) as core::ffi::c_int >> shift)
                as WORD16;
        } else {
            *noise_level_mant.offset((2 as WORD32 * k) as isize) = ((*noise_level_mant
                .offset((2 as WORD32 * k) as isize) as core::ffi::c_int) << -shift)
                as WORD16;
        }
        shift = (*nrg_sine
            .offset(
                (2 as core::ffi::c_int * k as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            ) as core::ffi::c_int - noise_e as core::ffi::c_int) as WORD32;
        if shift > 0 as core::ffi::c_int {
            *nrg_sine.offset((2 as WORD32 * k) as isize) = ixheaac_shl16_sat(
                *nrg_sine.offset((2 as WORD32 * k) as isize),
                shift as WORD16,
            );
        } else {
            *nrg_sine.offset((2 as WORD32 * k) as isize) = ixheaac_shr16(
                *nrg_sine.offset((2 as WORD32 * k) as isize),
                -shift as WORD16,
            );
        }
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_conv_ergtoamplitude_dec(
    mut bands: WORD32,
    mut noise_e: WORD16,
    mut nrg_sine: *mut WORD16,
    mut nrg_gain: *mut WORD16,
    mut noise_level_mant: *mut WORD16,
    mut sqrt_table: *mut WORD16,
) -> VOID {
    let mut k: WORD32 = 0;
    k = 0 as core::ffi::c_int as WORD32;
    while k < bands {
        let mut shift: WORD32 = 0;
        ixheaacd_fix_mant_exp_sqrt(
            &mut *nrg_sine.offset((2 as WORD32 * k) as isize),
            sqrt_table,
        );
        ixheaacd_fix_mant_exp_sqrt(
            &mut *nrg_gain.offset((2 as WORD32 * k) as isize),
            sqrt_table,
        );
        ixheaacd_fix_mant_exp_sqrt(
            &mut *noise_level_mant.offset((2 as WORD32 * k) as isize),
            sqrt_table,
        );
        shift = (noise_e as core::ffi::c_int
            - *noise_level_mant
                .offset(
                    (2 as core::ffi::c_int * k as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) as core::ffi::c_int) as WORD32;
        shift = (shift as core::ffi::c_int - 4 as core::ffi::c_int) as WORD32;
        if shift > 0 as core::ffi::c_int {
            if shift > 31 as core::ffi::c_int {
                shift = 31 as core::ffi::c_int as WORD32;
            }
            *noise_level_mant.offset((2 as WORD32 * k) as isize) = (*noise_level_mant
                .offset((2 as WORD32 * k) as isize) as core::ffi::c_int >> shift)
                as WORD16;
        } else {
            if shift < -(31 as core::ffi::c_int) {
                shift = -(31 as core::ffi::c_int) as WORD32;
            }
            *noise_level_mant.offset((2 as WORD32 * k) as isize) = ((*noise_level_mant
                .offset((2 as WORD32 * k) as isize) as core::ffi::c_int) << -shift)
                as WORD16;
        }
        k += 1;
    }
}
#[inline]
unsafe extern "C" fn ixheaacd_adapt_noise_gain_calc(
    mut ptr_sbr_calc_env: *mut ia_sbr_calc_env_struct,
    mut noise_e: WORD32,
    mut num_sub_bands: WORD32,
    mut skip_bands: WORD32,
    mut nrg_gain: *mut WORD16,
    mut noise_level_mant: *mut WORD16,
    mut nrg_sine: *mut WORD16,
    mut start_pos: WORD32,
    mut end_pos: WORD32,
    mut input_e: WORD32,
    mut adj_e: WORD32,
    mut final_e: WORD32,
    mut sub_band_start: WORD32,
    mut lb_scale: WORD32,
    mut noise_absc_flag: FLAG,
    mut smooth_length: WORD32,
    mut anal_buf_real_mant: *mut *mut WORD32,
    mut anal_buf_imag_mant: *mut *mut WORD32,
    mut low_pow_flag: WORD32,
    mut ptr_sbr_tables: *mut ia_sbr_tables_struct,
    mut max_cols: WORD16,
) -> VOID {
    let mut l: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut scale_change: WORD32 = 0;
    let mut bands: WORD32 = num_sub_bands - skip_bands;
    let mut ptr_filt_buf: *mut WORD16 = 0 as *mut WORD16;
    let mut ptr_filt_buf_noise: *mut WORD16 = 0 as *mut WORD16;
    let mut ptr_gain: *mut WORD16 = &mut *nrg_gain.offset(0 as core::ffi::c_int as isize)
        as *mut WORD16;
    if (*ptr_sbr_calc_env).start_up != 0 {
        let mut ptr_noise: *mut WORD16 = noise_level_mant;
        (*ptr_sbr_calc_env).start_up = 0 as core::ffi::c_int as FLAG;
        (*ptr_sbr_calc_env).filt_buf_noise_e = noise_e;
        ptr_filt_buf = &mut *((*ptr_sbr_calc_env).filt_buf_me)
            .offset((skip_bands as core::ffi::c_int * 2 as core::ffi::c_int) as isize)
            as *mut WORD16;
        ptr_filt_buf_noise = &mut *((*ptr_sbr_calc_env).filt_buf_noise_m)
            .offset(skip_bands as isize) as *mut WORD16;
        k = bands;
        while k != 0 as core::ffi::c_int {
            let fresh1 = ptr_gain;
            ptr_gain = ptr_gain.offset(1);
            let mut temp1: WORD16 = *fresh1;
            let fresh2 = ptr_gain;
            ptr_gain = ptr_gain.offset(1);
            let mut temp2: WORD16 = *fresh2;
            let mut temp3: WORD16 = *ptr_noise;
            ptr_noise = ptr_noise.offset(2 as core::ffi::c_int as isize);
            let fresh3 = ptr_filt_buf;
            ptr_filt_buf = ptr_filt_buf.offset(1);
            *fresh3 = temp1;
            let fresh4 = ptr_filt_buf;
            ptr_filt_buf = ptr_filt_buf.offset(1);
            *fresh4 = temp2;
            let fresh5 = ptr_filt_buf_noise;
            ptr_filt_buf_noise = ptr_filt_buf_noise.offset(1);
            *fresh5 = temp3;
            k -= 1;
        }
    } else {
        ixheaacd_equalize_filt_buff_exp(
            &mut *((*ptr_sbr_calc_env).filt_buf_me)
                .offset((2 as WORD32 * skip_bands) as isize),
            nrg_gain,
            bands,
        );
    }
    l = start_pos;
    while l < end_pos {
        if max_cols as core::ffi::c_int != 30 as core::ffi::c_int {
            if l < MAX_COLS {
                scale_change = adj_e - input_e;
            } else {
                scale_change = final_e - input_e;
                if l == MAX_COLS && start_pos < MAX_COLS {
                    let mut diff: WORD32 = final_e - noise_e;
                    noise_e = final_e;
                    ixheaacd_noise_level_rescaling(
                        noise_level_mant,
                        diff,
                        bands,
                        2 as WORD32,
                    );
                }
            }
        } else if l < max_cols as core::ffi::c_int {
            scale_change = adj_e - input_e;
        } else {
            scale_change = final_e - input_e;
            if l == max_cols as core::ffi::c_int
                && start_pos < max_cols as core::ffi::c_int
            {
                let mut diff_0: WORD32 = final_e - noise_e;
                noise_e = final_e;
                ixheaacd_noise_level_rescaling(
                    noise_level_mant,
                    diff_0,
                    bands,
                    2 as WORD32,
                );
            }
        }
        ixheaacd_noise_level_rescaling(
            (*ptr_sbr_calc_env).filt_buf_noise_m,
            (*ptr_sbr_calc_env).filt_buf_noise_e - noise_e,
            num_sub_bands,
            1 as WORD32,
        );
        (*ptr_sbr_calc_env).filt_buf_noise_e = noise_e;
        let mut anal_buf_real_m_l: *mut WORD32 = 0 as *mut WORD32;
        anal_buf_real_m_l = *anal_buf_real_mant.offset(l as isize);
        if low_pow_flag != 0 {
            let mut index: WORD32 = (*ptr_sbr_calc_env).ph_index as WORD32;
            let mut harm_index: WORD32 = (*ptr_sbr_calc_env).harm_index as WORD32;
            let mut freq_inv_flag: WORD32 = sub_band_start & 1 as WORD32;
            let mut ptr_real_buf: *mut WORD32 = &mut *anal_buf_real_m_l
                .offset(sub_band_start as isize) as *mut WORD32;
            let mut ptr_rand_ph: *const WORD32 = &mut *((*ptr_sbr_tables).sbr_rand_ph)
                .offset((index as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                as *mut WORD32;
            (*ptr_sbr_calc_env).ph_index = (index as core::ffi::c_int
                + num_sub_bands as core::ffi::c_int
                & SBR_NF_NO_RANDOM_VAL - 1 as core::ffi::c_int) as WORD16;
            (*ptr_sbr_calc_env).harm_index = (harm_index as core::ffi::c_int
                + 1 as core::ffi::c_int & 3 as core::ffi::c_int) as WORD16;
            if harm_index as core::ffi::c_int & 0x1 as core::ffi::c_int == 0 {
                (Some(ixheaacd_harm_idx_zerotwolp.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    ptr_real_buf,
                    nrg_gain,
                    scale_change as WORD,
                    nrg_sine,
                    ptr_rand_ph,
                    noise_level_mant,
                    num_sub_bands as WORD,
                    noise_absc_flag,
                    harm_index,
                );
            } else {
                let mut noise: WORD32 = noise_e - 16 as WORD32 - lb_scale;
                freq_inv_flag = (freq_inv_flag == 0) as core::ffi::c_int as WORD32;
                freq_inv_flag = (((freq_inv_flag as core::ffi::c_int)
                    << 1 as core::ffi::c_int) - 1 as core::ffi::c_int) as WORD32;
                if harm_index == 3 as core::ffi::c_int {
                    freq_inv_flag = -freq_inv_flag;
                }
                ixheaacd_harm_idx_onethreelp(
                    ptr_real_buf,
                    nrg_gain,
                    scale_change,
                    nrg_sine,
                    ptr_rand_ph,
                    noise_level_mant,
                    num_sub_bands,
                    noise_absc_flag,
                    freq_inv_flag,
                    noise,
                    sub_band_start,
                );
            }
        } else {
            let mut smooth_ratio: WORD16 = 0;
            let mut anal_buf_imag_m_l: *mut WORD32 = 0 as *mut WORD32;
            anal_buf_imag_m_l = *anal_buf_imag_mant.offset(l as isize);
            if l - start_pos < smooth_length {
                smooth_ratio = (*(*ptr_sbr_tables).env_calc_tables_ptr)
                    .sbr_smooth_filter[(l - start_pos) as usize];
            } else {
                smooth_ratio = 0 as WORD16;
            }
            ixheaacd_adj_timeslot(
                &mut *anal_buf_real_m_l.offset(sub_band_start as isize),
                &mut *anal_buf_imag_m_l.offset(sub_band_start as isize),
                &mut *((*ptr_sbr_calc_env).filt_buf_me)
                    .offset((2 as WORD32 * skip_bands) as isize),
                &mut *((*ptr_sbr_calc_env).filt_buf_noise_m).offset(skip_bands as isize),
                nrg_gain,
                noise_level_mant,
                nrg_sine,
                (noise_e as core::ffi::c_int - 16 as core::ffi::c_int) as WORD16,
                &mut (*ptr_sbr_calc_env).harm_index,
                sub_band_start as WORD16,
                bands as WORD16,
                scale_change as WORD16,
                smooth_ratio,
                noise_absc_flag,
                &mut (*ptr_sbr_calc_env).ph_index,
                ptr_sbr_tables,
            );
        }
        l += 1;
    }
    ixheaacd_filt_buf_update(
        ((*ptr_sbr_calc_env).filt_buf_me).offset((2 as WORD32 * skip_bands) as isize),
        ((*ptr_sbr_calc_env).filt_buf_noise_m).offset(skip_bands as isize),
        nrg_gain,
        noise_level_mant,
        bands,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_calc_subband_gains(
    mut pstr_freq_band_data: *mut ia_freq_band_data_struct,
    mut ptr_frame_data: *mut ia_sbr_frame_info_data_struct,
    mut freq_res: WORD32,
    mut ptr_noise_floor: *mut WORD16,
    mut num_sf_bands: WORD32,
    mut mvalue: WORD32,
    mut env: WORD32,
    mut sine_mapped_matrix: *mut WORD8,
    mut alias_red_buf: *mut WORD8,
    mut ptr_enrg_orig: *mut WORD16,
    mut nrg_sine: *mut WORD16,
    mut nrg_est: *mut WORD16,
    mut nrg_gain: *mut WORD16,
    mut noise_level_mant: *mut WORD16,
    mut noise_absc_flag: FLAG,
    mut pstr_common_tables: *mut ixheaacd_misc_tables,
) -> VOID {
    let mut ptr_freq_band_tbl: *mut WORD16 = (*pstr_freq_band_data)
        .freq_band_table[freq_res as usize];
    let mut ui_noise: WORD32 = (*pstr_freq_band_data)
        .freq_band_tbl_noise[1 as core::ffi::c_int as usize] as WORD32;
    let mut nb_idx: WORD32 = 0 as WORD32;
    let mut tmp_noise_mant: WORD16 = 0;
    let mut tmp_noise_exp: WORD16 = 0;
    let mut ptr_sine_mapped: *mut WORD8 = sine_mapped_matrix;
    let mut sub_band_start: WORD32 = (*pstr_freq_band_data).sub_band_start as WORD32;
    let mut skip_bands: WORD32 = (*ptr_frame_data).max_qmf_subband_aac - sub_band_start;
    let mut ptr_sine_mapped_1: *mut WORD8 = &mut *sine_mapped_matrix
        .offset(skip_bands as isize) as *mut WORD8;
    let mut k: WORD32 = 0;
    let mut c: WORD32 = 0 as WORD32;
    let mut j: WORD32 = 0;
    let mut ptr_env_sf_arr: *mut WORD16 = &mut *((*ptr_frame_data).int_env_sf_arr)
        .as_mut_ptr()
        .offset(mvalue as isize) as *mut WORD16;
    let mut ptr_alias_red_buf: *mut WORD8 = &mut *alias_red_buf
        .offset(
            (*ptr_freq_band_tbl.offset(0 as core::ffi::c_int as isize) as WORD32
                - sub_band_start) as isize,
        ) as *mut WORD8;
    tmp_noise_mant = (*ptr_noise_floor.offset(nb_idx as isize) as core::ffi::c_int
        & MASK_M) as WORD16;
    tmp_noise_exp = ((*ptr_noise_floor.offset(nb_idx as isize) as core::ffi::c_int
        & MASK_FOR_EXP) as WORD16 as core::ffi::c_int - NOISE_EXP_OFFSET) as WORD16;
    j = 0 as core::ffi::c_int as WORD32;
    while j < num_sf_bands {
        let mut sine_present_flag: WORD8 = 0;
        let mut tmp_nrg_ref_exp: WORD16 = 0;
        let mut tmp_nrg_ref_mant: WORD16 = 0;
        let fresh37 = ptr_freq_band_tbl;
        ptr_freq_band_tbl = ptr_freq_band_tbl.offset(1);
        let mut li: WORD16 = *fresh37;
        let mut ui: WORD16 = *ptr_freq_band_tbl;
        let fresh38 = ptr_env_sf_arr;
        ptr_env_sf_arr = ptr_env_sf_arr.offset(1);
        let mut env_sf_val: WORD16 = *fresh38;
        tmp_nrg_ref_exp = ((env_sf_val as core::ffi::c_int
            & MASK_FOR_EXP as WORD16 as core::ffi::c_int) - NRG_EXP_OFFSET) as WORD16;
        tmp_nrg_ref_mant = (env_sf_val as core::ffi::c_int & MASK_M) as WORD16;
        sine_present_flag = 0 as WORD8;
        k = li as WORD32;
        while k < ui as core::ffi::c_int {
            let fresh39 = ptr_sine_mapped;
            ptr_sine_mapped = ptr_sine_mapped.offset(1);
            if env >= *fresh39 as core::ffi::c_int {
                sine_present_flag = 1 as WORD8;
            }
            k += 1;
        }
        k = li as WORD32;
        while k < ui as core::ffi::c_int {
            let fresh40 = ptr_alias_red_buf;
            ptr_alias_red_buf = ptr_alias_red_buf.offset(1);
            *fresh40 = (sine_present_flag == 0) as core::ffi::c_int as WORD8;
            if k >= ui_noise {
                nb_idx += 1;
                ui_noise = (*pstr_freq_band_data)
                    .freq_band_tbl_noise[(nb_idx as core::ffi::c_int
                    + 1 as core::ffi::c_int) as usize] as WORD32;
                tmp_noise_mant = (*ptr_noise_floor.offset(nb_idx as isize)
                    as core::ffi::c_int & MASK_M) as WORD16;
                tmp_noise_exp = ((*ptr_noise_floor.offset(nb_idx as isize)
                    as core::ffi::c_int & MASK_FOR_EXP) as WORD16 as core::ffi::c_int
                    - NOISE_EXP_OFFSET) as WORD16;
            }
            if k >= (*ptr_frame_data).max_qmf_subband_aac {
                *ptr_enrg_orig.offset((2 as WORD32 * c) as isize) = tmp_nrg_ref_mant;
                *ptr_enrg_orig
                    .offset(
                        (2 as core::ffi::c_int * c as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    ) = tmp_nrg_ref_exp;
                *nrg_sine.offset((2 as WORD32 * c) as isize) = 0 as WORD16;
                *nrg_sine
                    .offset(
                        (2 as core::ffi::c_int * c as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    ) = 0 as WORD16;
                ixheaacd_subbandgain_calc(
                    tmp_nrg_ref_mant,
                    tmp_noise_mant,
                    *nrg_est.offset((2 as WORD32 * c) as isize),
                    *nrg_est
                        .offset(
                            (2 as core::ffi::c_int * c as core::ffi::c_int
                                + 1 as core::ffi::c_int) as isize,
                        ),
                    tmp_noise_exp,
                    tmp_nrg_ref_exp,
                    sine_present_flag as FLAG,
                    if env >= *ptr_sine_mapped_1.offset(c as isize) as core::ffi::c_int {
                        1 as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    },
                    noise_absc_flag,
                    &mut *nrg_gain.offset((2 as WORD32 * c) as isize),
                    &mut *noise_level_mant.offset((2 as WORD32 * c) as isize),
                    &mut *nrg_sine.offset((2 as WORD32 * c) as isize),
                    pstr_common_tables,
                );
                c += 1;
            }
            k += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_calc_sbrenvelope(
    mut ptr_sbr_scale_fac: *mut ia_sbr_scale_fact_struct,
    mut ptr_sbr_calc_env: *mut ia_sbr_calc_env_struct,
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut ptr_frame_data: *mut ia_sbr_frame_info_data_struct,
    mut ptr_frame_data_prev: *mut ia_sbr_prev_frame_data_struct,
    mut anal_buf_real_mant: *mut *mut WORD32,
    mut anal_buf_imag_mant: *mut *mut WORD32,
    mut deg_patched: *mut WORD16,
    mut low_pow_flag: FLAG,
    mut ptr_sbr_tables: *mut ia_sbr_tables_struct,
    mut pstr_common_tables: *mut ixheaacd_misc_tables,
    mut ptr_qmf_matrix: *mut WORD32,
    mut audio_object_type: WORD32,
) -> IA_ERRORCODE {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut m: WORD32 = 0;
    let mut noise_floor_idx: WORD32 = 0;
    let mut start_pos: WORD32 = 0;
    let mut end_pos: WORD32 = 0;
    let mut freq_res: WORD32 = 0;
    let mut num_env: WORD32 = (*ptr_frame_data).str_frame_info_details.num_env as WORD32;
    let mut ptr_border_vec: *mut WORD16 = ((*ptr_frame_data)
        .str_frame_info_details
        .border_vec)
        .as_mut_ptr();
    let mut err_code: IA_ERRORCODE = IA_NO_ERROR;
    let mut ptr_noise_floor: *mut WORD16 = 0 as *mut WORD16;
    let mut pstr_freq_band_data: *mut ia_freq_band_data_struct = (*ptr_header_data)
        .pstr_freq_band_data;
    let mut noise_absc_flag: FLAG = 0;
    let mut smooth_length: WORD32 = 0;
    let mut num_sf_bands: *const WORD16 = ((*pstr_freq_band_data).num_sf_bands)
        .as_mut_ptr();
    let num_nf_bands: WORD32 = (*pstr_freq_band_data).num_nf_bands as WORD32;
    let mut sub_band_start: WORD32 = (*pstr_freq_band_data).sub_band_start as WORD32;
    let mut sub_band_end: WORD32 = (*pstr_freq_band_data).sub_band_end as WORD32;
    let mut num_timeslots: WORD16 = (*ptr_header_data).num_time_slots;
    let mut max_cols: WORD16 = ((*ptr_header_data).num_time_slots as core::ffi::c_int
        * 2 as core::ffi::c_int) as WORD16;
    let mut num_sub_bands: WORD32 = 0;
    let mut skip_bands: WORD32 = 0;
    let mut bands: WORD32 = 0;
    let mut num_cols: WORD = 0;
    let mut first_start: WORD32 = 0;
    let mut ptr_sbr_lim_gain: *mut WORD16 = 0 as *mut WORD16;
    let mut max_sfb_nrg_exp: WORD32 = 0;
    let mut ptr_enrg_orig: *mut WORD16 = 0 as *mut WORD16;
    let mut input_e: WORD32 = 0;
    let mut ov_adj_e: WORD32 = 0;
    let mut adj_e: WORD32 = 0;
    let mut output_e: WORD32 = 0;
    let mut final_e: WORD32 = 0;
    let mut noise_e: WORD16 = 0;
    let mut lb_scale: WORD16 = 0;
    let mut nrg_est: [WORD16; 112] = [0; 112];
    let mut nrg_gain: [WORD16; 112] = [0; 112];
    let mut noise_level_mant: [WORD16; 112] = [0; 112];
    let mut nrg_sine: [WORD16; 112] = [0; 112];
    let mut sine_mapped_matrix: [WORD8; 56] = [0; 56];
    let mut alias_red_buf: [WORD8; 64] = [0; 64];
    ptr_noise_floor = ((*ptr_frame_data).int_noise_floor).as_mut_ptr();
    ptr_enrg_orig = (ptr_frame_data as *mut WORD8)
        .offset(
            (((::core::mem::size_of::<ia_sbr_frame_info_data_struct>() as usize)
                .wrapping_add(7 as usize) >> 3 as core::ffi::c_int)
                << 3 as core::ffi::c_int) as isize,
        ) as *mut WORD16;
    num_env = (*ptr_frame_data).str_frame_info_details.num_env as WORD32;
    ptr_border_vec = ((*ptr_frame_data).str_frame_info_details.border_vec).as_mut_ptr();
    num_sub_bands = sub_band_end - sub_band_start;
    skip_bands = (*ptr_frame_data).max_qmf_subband_aac - sub_band_start;
    ixheaacd_map_sineflags(
        (*pstr_freq_band_data).freq_band_table[HIGH as usize],
        (*pstr_freq_band_data).num_sf_bands[HIGH as usize],
        ((*ptr_frame_data).add_harmonics).as_mut_ptr(),
        ((*ptr_sbr_calc_env).harm_flags_prev).as_mut_ptr(),
        (*ptr_frame_data).str_frame_info_details.transient_env,
        sine_mapped_matrix.as_mut_ptr(),
    );
    adj_e = 0 as core::ffi::c_int as WORD32;
    let mut max_noise: WORD16 = 0;
    let mut first_band: WORD32 = 0;
    if (*ptr_frame_data_prev).max_qmf_subband_aac > (*ptr_frame_data).max_qmf_subband_aac
    {
        first_band = (*ptr_frame_data_prev).max_qmf_subband_aac - sub_band_start;
    } else {
        first_band = (*ptr_frame_data).max_qmf_subband_aac - sub_band_start;
    }
    max_noise = 0 as WORD16;
    i = first_band;
    while i < num_sub_bands {
        if *((*ptr_sbr_calc_env).filt_buf_noise_m).offset(i as isize) as core::ffi::c_int
            > max_noise as core::ffi::c_int
        {
            max_noise = *((*ptr_sbr_calc_env).filt_buf_noise_m).offset(i as isize);
        }
        i += 1;
    }
    adj_e = ((*ptr_sbr_calc_env).filt_buf_noise_e as core::ffi::c_int
        - ixheaac_norm32(max_noise as WORD32) as core::ffi::c_int
        - 16 as core::ffi::c_int) as WORD32;
    final_e = 0 as core::ffi::c_int as WORD32;
    let mut ptr_env_sf_buf: *mut WORD16 = ((*ptr_frame_data).int_env_sf_arr)
        .as_mut_ptr();
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_env {
        let mut temp_val: WORD32 = 0;
        max_sfb_nrg_exp = (NRG_EXP_OFFSET - SHORT_BITS) as WORD32;
        freq_res = (*ptr_frame_data).str_frame_info_details.freq_res[i as usize]
            as WORD32;
        j = 0 as core::ffi::c_int as WORD32;
        while j < *num_sf_bands.offset(freq_res as isize) as core::ffi::c_int {
            let fresh0 = ptr_env_sf_buf;
            ptr_env_sf_buf = ptr_env_sf_buf.offset(1);
            temp_val = (*fresh0 as core::ffi::c_int & MASK_FOR_EXP) as WORD32;
            if temp_val > max_sfb_nrg_exp {
                max_sfb_nrg_exp = temp_val;
            }
            j += 1;
        }
        max_sfb_nrg_exp = (max_sfb_nrg_exp as core::ffi::c_int - NRG_EXP_OFFSET)
            as WORD32;
        temp_val = (max_sfb_nrg_exp as core::ffi::c_int + 13 as core::ffi::c_int
            >> 1 as core::ffi::c_int) as WORD32;
        if num_timeslots as core::ffi::c_int != 15 as core::ffi::c_int {
            if (*ptr_border_vec.offset(i as isize) as core::ffi::c_int) < SBR_TIME_SLOTS
            {
                if temp_val > adj_e {
                    adj_e = temp_val as WORD16 as WORD32;
                }
            }
            if *ptr_border_vec
                .offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                as core::ffi::c_int > SBR_TIME_SLOTS
            {
                if temp_val > final_e {
                    final_e = temp_val as WORD16 as WORD32;
                }
            }
        } else {
            if (*ptr_border_vec.offset(i as isize) as core::ffi::c_int)
                < num_timeslots as core::ffi::c_int
            {
                if temp_val > adj_e {
                    adj_e = temp_val as WORD16 as WORD32;
                }
            }
            if *ptr_border_vec
                .offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                as core::ffi::c_int > num_timeslots as core::ffi::c_int
            {
                if temp_val > final_e {
                    final_e = temp_val as WORD16 as WORD32;
                }
            }
        }
        i += 1;
    }
    m = 0 as core::ffi::c_int as WORD32;
    noise_floor_idx = 0 as core::ffi::c_int as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_env {
        if audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int
            || audio_object_type == AOT_ER_AAC_LD as core::ffi::c_int
        {
            start_pos = *ptr_border_vec.offset(i as isize) as WORD32;
            end_pos = *ptr_border_vec
                .offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                as WORD32;
        } else {
            start_pos = (SBR_TIME_STEP
                * *ptr_border_vec.offset(i as isize) as core::ffi::c_int) as WORD32;
            end_pos = (SBR_TIME_STEP
                * *ptr_border_vec
                    .offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_int) as WORD32;
        }
        if start_pos >= MAX_ENV_COLS || end_pos > MAX_ENV_COLS {
            return IA_FATAL_ERROR as IA_ERRORCODE;
        }
        freq_res = (*ptr_frame_data).str_frame_info_details.freq_res[i as usize]
            as WORD32;
        if noise_floor_idx >= MAX_NOISE_ENVELOPES {
            return IA_FATAL_ERROR as IA_ERRORCODE;
        }
        if *ptr_border_vec.offset(i as isize) as core::ffi::c_int
            == (*ptr_frame_data)
                .str_frame_info_details
                .noise_border_vec[(noise_floor_idx as core::ffi::c_int
                + 1 as core::ffi::c_int) as usize] as core::ffi::c_int
        {
            ptr_noise_floor = ptr_noise_floor.offset(num_nf_bands as isize);
            noise_floor_idx += 1;
        }
        if i
            == (*ptr_frame_data).str_frame_info_details.transient_env as core::ffi::c_int
            || i == (*ptr_sbr_calc_env).tansient_env_prev as core::ffi::c_int
        {
            noise_absc_flag = 1 as core::ffi::c_int as FLAG;
            smooth_length = 0 as core::ffi::c_int as WORD32;
        } else {
            noise_absc_flag = 0 as core::ffi::c_int as FLAG;
            smooth_length = ((1 as core::ffi::c_int
                - (*ptr_header_data).smoothing_mode as core::ffi::c_int)
                << 2 as core::ffi::c_int) as WORD32;
        }
        input_e = (15 as core::ffi::c_int
            - (*ptr_sbr_scale_fac).hb_scale as core::ffi::c_int) as WORD32;
        if (*ptr_header_data).interpol_freq != 0 {
            (Some(ixheaacd_enery_calc_per_subband.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                start_pos,
                end_pos,
                (*ptr_frame_data).max_qmf_subband_aac,
                sub_band_end,
                input_e,
                nrg_est.as_mut_ptr(),
                low_pow_flag,
                ptr_sbr_tables,
                ptr_qmf_matrix,
            );
        } else {
            ixheaacd_enery_calc_persfb(
                anal_buf_real_mant,
                anal_buf_imag_mant,
                *num_sf_bands.offset(freq_res as isize) as WORD32,
                (*pstr_freq_band_data).freq_band_table[freq_res as usize],
                start_pos,
                end_pos,
                (*ptr_frame_data).max_qmf_subband_aac,
                input_e,
                nrg_est.as_mut_ptr(),
                low_pow_flag,
                ptr_sbr_tables,
            );
        }
        if (*((*pstr_freq_band_data).freq_band_table[freq_res as usize])
            .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
            < (*pstr_freq_band_data).sub_band_start as core::ffi::c_int
        {
            (*pstr_freq_band_data).sub_band_start = *((*pstr_freq_band_data)
                .freq_band_table[freq_res as usize])
                .offset(0 as core::ffi::c_int as isize);
            return IA_FATAL_ERROR as IA_ERRORCODE;
        }
        ixheaacd_calc_subband_gains(
            pstr_freq_band_data,
            ptr_frame_data,
            freq_res,
            ptr_noise_floor,
            *num_sf_bands.offset(freq_res as isize) as WORD32,
            m,
            i,
            sine_mapped_matrix.as_mut_ptr(),
            alias_red_buf.as_mut_ptr(),
            ptr_enrg_orig,
            nrg_sine.as_mut_ptr(),
            nrg_est.as_mut_ptr(),
            nrg_gain.as_mut_ptr(),
            noise_level_mant.as_mut_ptr(),
            noise_absc_flag,
            pstr_common_tables,
        );
        m += *num_sf_bands.offset(freq_res as isize) as core::ffi::c_int;
        ptr_sbr_lim_gain = &mut *((*(*ptr_sbr_tables).env_calc_tables_ptr)
            .sbr_lim_gains_m)
            .as_mut_ptr()
            .offset(
                (2 as core::ffi::c_int
                    * (*ptr_header_data).limiter_gains as core::ffi::c_int) as isize,
            ) as *mut WORD16;
        ixheaacd_noiselimiting(
            pstr_freq_band_data,
            skip_bands,
            ptr_enrg_orig,
            nrg_est.as_mut_ptr(),
            nrg_gain.as_mut_ptr(),
            noise_level_mant.as_mut_ptr(),
            nrg_sine.as_mut_ptr(),
            ptr_sbr_lim_gain,
            noise_absc_flag,
            pstr_common_tables,
        );
        if low_pow_flag != 0 {
            ixheaacd_alias_reduction(
                deg_patched.offset(sub_band_start as isize),
                nrg_gain.as_mut_ptr(),
                nrg_est.as_mut_ptr(),
                alias_red_buf.as_mut_ptr(),
                num_sub_bands,
                pstr_common_tables,
            );
        }
        if max_cols as core::ffi::c_int != 30 as core::ffi::c_int {
            if start_pos < MAX_COLS {
                noise_e = adj_e as WORD16;
            } else {
                noise_e = final_e as WORD16;
            }
        } else if start_pos < max_cols as core::ffi::c_int {
            noise_e = adj_e as WORD16;
        } else {
            noise_e = final_e as WORD16;
        }
        bands = num_sub_bands - skip_bands;
        if low_pow_flag != 0 {
            (Some(ixheaacd_conv_ergtoamplitudelp.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                bands,
                noise_e,
                nrg_sine.as_mut_ptr(),
                nrg_gain.as_mut_ptr(),
                noise_level_mant.as_mut_ptr(),
                ((*pstr_common_tables).sqrt_table).as_ptr() as *mut WORD16,
            );
        } else {
            (Some(ixheaacd_conv_ergtoamplitude.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                bands,
                noise_e,
                nrg_sine.as_mut_ptr(),
                nrg_gain.as_mut_ptr(),
                noise_level_mant.as_mut_ptr(),
                ((*pstr_common_tables).sqrt_table).as_ptr() as *mut WORD16,
            );
        }
        lb_scale = ixheaac_sub16(15 as WORD16, (*ptr_sbr_scale_fac).lb_scale);
        ixheaacd_adapt_noise_gain_calc(
            ptr_sbr_calc_env,
            noise_e as WORD32,
            num_sub_bands,
            skip_bands,
            nrg_gain.as_mut_ptr(),
            noise_level_mant.as_mut_ptr(),
            nrg_sine.as_mut_ptr(),
            start_pos,
            end_pos,
            input_e,
            adj_e,
            final_e,
            (*ptr_frame_data).max_qmf_subband_aac,
            lb_scale as WORD32,
            noise_absc_flag,
            smooth_length,
            anal_buf_real_mant,
            anal_buf_imag_mant,
            low_pow_flag as WORD32,
            ptr_sbr_tables,
            max_cols,
        );
        i += 1;
    }
    first_start = (*ptr_border_vec.offset(0 as core::ffi::c_int as isize)
        as core::ffi::c_int * SBR_TIME_STEP) as WORD32;
    let mut ov_reserve: WORD32 = 0;
    let mut reserve: WORD32 = 0;
    reserve = 0 as core::ffi::c_int as WORD32;
    ov_reserve = reserve;
    if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int {
        if (*ptr_header_data).channel_mode == PS_STEREO {
            ov_reserve = (Some(
                ixheaacd_ixheaacd_expsubbandsamples.expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                anal_buf_real_mant,
                anal_buf_imag_mant,
                (*ptr_frame_data).max_qmf_subband_aac,
                sub_band_end,
                0 as WORD32,
                first_start,
                low_pow_flag,
            ) as WORD32;
            if max_cols as core::ffi::c_int != 30 as core::ffi::c_int {
                reserve = (Some(
                    ixheaacd_ixheaacd_expsubbandsamples
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    anal_buf_real_mant,
                    anal_buf_imag_mant,
                    (*ptr_frame_data).max_qmf_subband_aac,
                    sub_band_end,
                    first_start,
                    MAX_COLS,
                    low_pow_flag,
                ) as WORD32;
            } else {
                reserve = (Some(
                    ixheaacd_ixheaacd_expsubbandsamples
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    anal_buf_real_mant,
                    anal_buf_imag_mant,
                    (*ptr_frame_data).max_qmf_subband_aac,
                    sub_band_end,
                    first_start,
                    max_cols as WORD32,
                    low_pow_flag,
                ) as WORD32;
            }
        }
    }
    output_e = 0 as core::ffi::c_int as WORD32;
    ov_adj_e = (15 as core::ffi::c_int
        - (*ptr_sbr_scale_fac).ov_hb_scale as core::ffi::c_int) as WORD32;
    if ov_adj_e - ov_reserve > adj_e - reserve {
        output_e = ov_adj_e - ov_reserve;
    } else {
        output_e = adj_e - reserve;
    }
    (Some(ixheaacd_adjust_scale.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        anal_buf_real_mant,
        anal_buf_imag_mant,
        (*ptr_frame_data).max_qmf_subband_aac,
        sub_band_end,
        0 as WORD32,
        first_start,
        ov_adj_e - output_e,
        low_pow_flag,
    );
    num_cols = ((*ptr_header_data).num_time_slots as core::ffi::c_int
        * (*ptr_header_data).time_step as core::ffi::c_int) as WORD;
    (Some(ixheaacd_adjust_scale.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        anal_buf_real_mant,
        anal_buf_imag_mant,
        (*ptr_frame_data).max_qmf_subband_aac,
        sub_band_end,
        first_start,
        num_cols as WORD32,
        adj_e - output_e,
        low_pow_flag,
    );
    (*ptr_sbr_scale_fac).hb_scale = (15 as WORD32 - output_e) as WORD16;
    (*ptr_sbr_scale_fac).ov_hb_scale = (15 as WORD32 - final_e) as WORD16;
    if (*ptr_frame_data).str_frame_info_details.transient_env as core::ffi::c_int
        == num_env
    {
        (*ptr_sbr_calc_env).tansient_env_prev = 0 as WORD16;
    } else {
        (*ptr_sbr_calc_env).tansient_env_prev = -(1 as core::ffi::c_int) as WORD16;
    }
    return err_code;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_equalize_filt_buff_exp(
    mut ptr_filt_buf: *mut WORD16,
    mut nrg_gain: *mut WORD16,
    mut subbands: WORD32,
) -> VOID {
    let mut band: WORD32 = 0;
    let mut diff: WORD32 = 0;
    let mut gain_m: WORD32 = 0;
    let mut gain_e: WORD32 = 0;
    let mut filt_buf_mant: WORD32 = 0;
    let mut filt_buf_exp: WORD32 = 0;
    band = (subbands as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while band >= 0 as core::ffi::c_int {
        filt_buf_exp = *ptr_filt_buf.offset(1 as core::ffi::c_int as isize) as WORD32;
        gain_e = *nrg_gain.offset(1 as core::ffi::c_int as isize) as WORD32;
        filt_buf_mant = *ptr_filt_buf as WORD32;
        gain_m = *nrg_gain as WORD32;
        diff = gain_e - filt_buf_exp;
        if diff >= 0 as core::ffi::c_int {
            *ptr_filt_buf.offset(1 as core::ffi::c_int as isize) = gain_e as WORD16;
            *ptr_filt_buf = (*ptr_filt_buf as core::ffi::c_int >> diff) as WORD16;
        } else {
            let mut reserve: WORD32 = 0;
            reserve = (ixheaac_norm32(filt_buf_mant) as core::ffi::c_int
                - 16 as core::ffi::c_int) as WORD32;
            if diff + reserve >= 0 as core::ffi::c_int {
                *ptr_filt_buf = (filt_buf_mant << -diff) as WORD16;
                *ptr_filt_buf.offset(1 as core::ffi::c_int as isize) = (filt_buf_exp
                    + diff) as WORD16;
            } else {
                let mut shift: WORD32 = 0;
                *ptr_filt_buf = (filt_buf_mant << reserve) as WORD16;
                *ptr_filt_buf.offset(1 as core::ffi::c_int as isize) = (filt_buf_exp
                    - reserve) as WORD16;
                shift = -(reserve + diff);
                *nrg_gain = (gain_m >> shift) as WORD16;
                *nrg_gain.offset(1 as core::ffi::c_int as isize) = (*nrg_gain
                    .offset(1 as core::ffi::c_int as isize) as WORD32 + shift) as WORD16;
            }
        }
        nrg_gain = nrg_gain.offset(2 as core::ffi::c_int as isize);
        ptr_filt_buf = ptr_filt_buf.offset(2 as core::ffi::c_int as isize);
        band -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_filt_buf_update(
    mut ptr_filt_buf: *mut WORD16,
    mut ptr_filt_buf_noise: *mut WORD16,
    mut nrg_gain: *mut WORD16,
    mut noise_level_mant: *mut WORD16,
    mut num_sub_bands: WORD32,
) -> VOID {
    let mut k: WORD32 = 0;
    let mut temp1: WORD32 = 0;
    let mut temp2: WORD32 = 0;
    k = (num_sub_bands as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while k >= 0 as core::ffi::c_int {
        temp1 = *nrg_gain as WORD32;
        nrg_gain = nrg_gain.offset(2 as core::ffi::c_int as isize);
        temp2 = *noise_level_mant as WORD32;
        noise_level_mant = noise_level_mant.offset(2 as core::ffi::c_int as isize);
        *ptr_filt_buf = temp1 as WORD16;
        ptr_filt_buf = ptr_filt_buf.offset(2 as core::ffi::c_int as isize);
        let fresh6 = ptr_filt_buf_noise;
        ptr_filt_buf_noise = ptr_filt_buf_noise.offset(1);
        *fresh6 = temp2 as WORD16;
        k -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_noise_level_rescaling(
    mut noise_level_mant: *mut WORD16,
    mut diff: WORD32,
    mut num_sub_bands: WORD32,
    mut ixheaacd_drc_offset: WORD32,
) -> VOID {
    let mut k: WORD32 = 0;
    if diff > 0 as core::ffi::c_int {
        k = (num_sub_bands as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        while k >= 0 as core::ffi::c_int {
            *noise_level_mant = (*noise_level_mant as core::ffi::c_int >> diff)
                as WORD16;
            noise_level_mant = noise_level_mant.offset(ixheaacd_drc_offset as isize);
            k -= 1;
        }
    } else if diff < 0 as core::ffi::c_int {
        diff = -diff;
        k = (num_sub_bands as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        while k >= 0 as core::ffi::c_int {
            *noise_level_mant = ((*noise_level_mant as core::ffi::c_int) << diff)
                as WORD16;
            noise_level_mant = noise_level_mant.offset(ixheaacd_drc_offset as isize);
            k -= 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_adjust_scale_dec(
    mut re: *mut *mut WORD32,
    mut im: *mut *mut WORD32,
    mut sub_band_start: WORD32,
    mut sub_band_end: WORD32,
    mut start_pos: WORD32,
    mut next_pos: WORD32,
    mut shift: WORD32,
    mut low_pow_flag: FLAG,
) -> VOID {
    let mut k: WORD32 = 0;
    let mut l: WORD32 = 0;
    if shift != 0 as core::ffi::c_int {
        let mut num_sub_bands: WORD32 = sub_band_end - sub_band_start;
        shift = ixheaac_min32(shift, 31 as WORD32);
        shift = ixheaac_max32(shift, -(31 as WORD32));
        if low_pow_flag != 0 {
            if shift > 0 as core::ffi::c_int {
                l = start_pos;
                while l < next_pos {
                    let mut ptr: *mut WORD32 = (*re.offset(l as isize))
                        .offset(sub_band_start as isize);
                    k = (num_sub_bands as core::ffi::c_int - 1 as core::ffi::c_int)
                        as WORD32;
                    while k >= 0 as core::ffi::c_int {
                        *ptr = *ptr << shift;
                        ptr = ptr.offset(1);
                        k -= 1;
                    }
                    l += 1;
                }
            } else {
                shift = -shift;
                l = start_pos;
                while l < next_pos {
                    let mut ptr_0: *mut WORD32 = (*re.offset(l as isize))
                        .offset(sub_band_start as isize);
                    k = (num_sub_bands as core::ffi::c_int - 1 as core::ffi::c_int)
                        as WORD32;
                    while k >= 0 as core::ffi::c_int {
                        *ptr_0 = *ptr_0 >> shift;
                        ptr_0 = ptr_0.offset(1);
                        k -= 1;
                    }
                    l += 1;
                }
            }
        } else if shift > 0 as core::ffi::c_int {
            l = start_pos;
            while l < next_pos {
                let mut ptr_1: *mut WORD32 = (*re.offset(l as isize))
                    .offset(sub_band_start as isize);
                let mut pti: *mut WORD32 = (*im.offset(l as isize))
                    .offset(sub_band_start as isize);
                k = num_sub_bands;
                while k > 0 as core::ffi::c_int {
                    *ptr_1 = *ptr_1 << shift;
                    *pti = *pti << shift;
                    pti = pti.offset(1);
                    ptr_1 = ptr_1.offset(1);
                    k -= 1;
                }
                l += 1;
            }
        } else {
            shift = -shift;
            l = start_pos;
            while l < next_pos {
                let mut ptr_2: *mut WORD32 = (*re.offset(l as isize))
                    .offset(sub_band_start as isize);
                let mut pti_0: *mut WORD32 = (*im.offset(l as isize))
                    .offset(sub_band_start as isize);
                k = num_sub_bands;
                while k > 0 as core::ffi::c_int {
                    *ptr_2 = *ptr_2 >> shift;
                    *pti_0 = *pti_0 >> shift;
                    ptr_2 = ptr_2.offset(1);
                    pti_0 = pti_0.offset(1);
                    k -= 1;
                }
                l += 1;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_expsubbandsamples_dec(
    mut re: *mut *mut WORD32,
    mut im: *mut *mut WORD32,
    mut sub_band_start: WORD32,
    mut sub_band_end: WORD32,
    mut start_pos: WORD32,
    mut next_pos: WORD32,
    mut low_pow_flag: FLAG,
) -> WORD16 {
    let mut l: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut max_shift: WORD16 = 0;
    let mut value: WORD32 = 0;
    let mut max_abs: WORD32 = 0;
    let mut num_sub_bands: WORD32 = 0;
    let mut ptr_real: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr_imag: *mut WORD32 = 0 as *mut WORD32;
    max_abs = 1 as core::ffi::c_int as WORD32;
    num_sub_bands = sub_band_end - sub_band_start;
    if low_pow_flag != 0 {
        l = start_pos;
        while l < next_pos {
            let mut temp_real: WORD32 = 0;
            ptr_real = (*re.offset(l as isize)).offset(sub_band_start as isize);
            let fresh43 = ptr_real;
            ptr_real = ptr_real.offset(1);
            temp_real = *fresh43;
            k = num_sub_bands;
            while k > 0 as core::ffi::c_int {
                value = ixheaac_abs32_nrm(temp_real);
                max_abs |= value;
                let fresh44 = ptr_real;
                ptr_real = ptr_real.offset(1);
                temp_real = *fresh44;
                k -= 1;
            }
            l += 1;
        }
        max_shift = ixheaac_pnorm32(max_abs) as WORD16;
    } else {
        l = start_pos;
        while l < next_pos {
            ptr_real = (*re.offset(l as isize)).offset(sub_band_start as isize);
            ptr_imag = (*im.offset(l as isize)).offset(sub_band_start as isize);
            k = num_sub_bands;
            while k > 0 as core::ffi::c_int {
                let fresh45 = ptr_real;
                ptr_real = ptr_real.offset(1);
                let mut temp_real_0: WORD32 = *fresh45;
                let fresh46 = ptr_imag;
                ptr_imag = ptr_imag.offset(1);
                let mut tempIm: WORD32 = *fresh46;
                temp_real_0 = ixheaac_abs32_nrm(temp_real_0);
                max_abs |= temp_real_0;
                tempIm = ixheaac_abs32_nrm(tempIm);
                max_abs |= tempIm;
                k -= 1;
            }
            l += 1;
        }
        max_shift = ixheaac_pnorm32(max_abs) as WORD16;
    }
    return max_shift;
}
pub const SHIFT_BEFORE_SQUARE: core::ffi::c_int = 4 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_enery_calc_per_subband_dec(
    mut start_pos: WORD32,
    mut next_pos: WORD32,
    mut sub_band_start: WORD32,
    mut sub_band_end: WORD32,
    mut frame_exp: WORD32,
    mut nrg_est: *mut WORD16,
    mut low_pow_flag: FLAG,
    mut ptr_sbr_tables: *mut ia_sbr_tables_struct,
    mut ptr_qmf_matrix: *mut WORD32,
) -> VOID {
    let mut temp: WORD16 = 0;
    let mut inv_width: WORD16 = 0;
    let mut sum_m: WORD16 = 0;
    let mut accu: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut l: WORD32 = 0;
    let mut pre_shift_val: WORD32 = 0;
    let mut shift: WORD32 = 0;
    let mut p_real: *mut WORD32 = 0 as *mut WORD32;
    let mut max_shift_gap: WORD32 = SHIFT_BEFORE_SQUARE;
    let mut extra_shift: WORD32 = 0 as WORD32;
    let mut num_cols: WORD32 = next_pos - start_pos;
    if low_pow_flag != 0 {
        max_shift_gap -= 1 as core::ffi::c_int;
        p_real = ptr_qmf_matrix
            .offset(sub_band_start as isize)
            .offset((start_pos << 6 as core::ffi::c_int) as isize);
        extra_shift += 1;
    } else {
        p_real = ptr_qmf_matrix
            .offset(sub_band_start as isize)
            .offset((start_pos << 7 as core::ffi::c_int) as isize);
        num_cols = num_cols << 1 as core::ffi::c_int;
    }
    inv_width = (*(*ptr_sbr_tables).env_calc_tables_ptr)
        .sbr_inv_int_table[(next_pos - start_pos) as usize];
    frame_exp = frame_exp << 1 as core::ffi::c_int;
    let mut ptr: *mut WORD32 = 0 as *mut WORD32;
    k = sub_band_start;
    while k < sub_band_end {
        let mut max_val: WORD32 = 1 as WORD32;
        ptr = p_real;
        l = num_cols;
        while l != 0 as core::ffi::c_int {
            let mut value: WORD32 = ixheaac_abs32_nrm(*ptr);
            ptr = ptr.offset(64 as core::ffi::c_int as isize);
            max_val = ixheaac_max32(value, max_val);
            value = ixheaac_abs32_nrm(*ptr);
            ptr = ptr.offset(64 as core::ffi::c_int as isize);
            max_val = ixheaac_max32(value, max_val);
            l -= 2 as core::ffi::c_int;
        }
        pre_shift_val = ixheaac_pnorm32(max_val) as WORD32 - max_shift_gap;
        accu = 0 as WORD32;
        shift = 16 as WORD32 - pre_shift_val;
        ptr = p_real;
        if shift > 0 as core::ffi::c_int {
            l = num_cols;
            while l != 0 as core::ffi::c_int {
                temp = (*ptr >> shift) as WORD16;
                ptr = ptr.offset(64 as core::ffi::c_int as isize);
                accu += temp as core::ffi::c_int * temp as core::ffi::c_int;
                temp = (*ptr >> shift) as WORD16;
                ptr = ptr.offset(64 as core::ffi::c_int as isize);
                accu += temp as core::ffi::c_int * temp as core::ffi::c_int;
                l -= 2 as core::ffi::c_int;
            }
        } else {
            l = num_cols;
            while l != 0 as core::ffi::c_int {
                temp = (*ptr << -shift) as WORD16;
                ptr = ptr.offset(64 as core::ffi::c_int as isize);
                accu += temp as core::ffi::c_int * temp as core::ffi::c_int;
                temp = (*ptr << -shift) as WORD16;
                ptr = ptr.offset(64 as core::ffi::c_int as isize);
                accu += temp as core::ffi::c_int * temp as core::ffi::c_int;
                l -= 2 as core::ffi::c_int;
            }
        }
        if accu as core::ffi::c_long != 0 as core::ffi::c_long {
            shift = -ixheaac_pnorm32(accu) as WORD32;
            sum_m = ixheaac_shr32_dir_sat_limit(accu, 16 as WORD + shift as WORD)
                as WORD16;
            let fresh47 = nrg_est;
            nrg_est = nrg_est.offset(1);
            *fresh47 = ixheaac_mult16_shl_sat(sum_m, inv_width);
            shift = shift - (pre_shift_val << 1 as core::ffi::c_int);
            shift += extra_shift;
            let fresh48 = nrg_est;
            nrg_est = nrg_est.offset(1);
            *fresh48 = (frame_exp as core::ffi::c_int + shift as core::ffi::c_int
                + 1 as core::ffi::c_int) as WORD16;
        } else {
            let fresh49 = nrg_est;
            nrg_est = nrg_est.offset(1);
            *fresh49 = 0 as WORD16;
            let fresh50 = nrg_est;
            nrg_est = nrg_est.offset(1);
            *fresh50 = 0 as WORD16;
        }
        p_real = p_real.offset(1);
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_enery_calc_persfb(
    mut anal_buf_real: *mut *mut WORD32,
    mut anal_buf_imag: *mut *mut WORD32,
    mut num_sf_bands: WORD32,
    mut freq_band_table: *mut WORD16,
    mut start_pos: WORD32,
    mut next_pos: WORD32,
    mut max_qmf_subband_aac: WORD32,
    mut frame_exp: WORD32,
    mut nrg_est: *mut WORD16,
    mut low_pow_flag: FLAG,
    mut ptr_sbr_tables: *mut ia_sbr_tables_struct,
) -> VOID {
    let mut inv_width: WORD16 = 0;
    let mut pre_shift_val: WORD32 = 0;
    let mut shift: WORD32 = 0;
    let mut sum_e: WORD32 = 0;
    let mut sum_m: WORD16 = 0;
    let mut j: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut l: WORD32 = 0;
    let mut li: WORD32 = 0;
    let mut ui: WORD32 = 0;
    let mut accu_line: WORD32 = 0;
    let mut accumulate: WORD32 = 0;
    let mut extra_shift: WORD32 = 10 as WORD32;
    inv_width = (*(*ptr_sbr_tables).env_calc_tables_ptr)
        .sbr_inv_int_table[(next_pos - start_pos) as usize];
    frame_exp = frame_exp << 1 as core::ffi::c_int;
    if low_pow_flag != 0 {
        extra_shift += 1;
    }
    j = 0 as core::ffi::c_int as WORD32;
    while j < num_sf_bands {
        li = *freq_band_table.offset(j as isize) as WORD32;
        if li >= max_qmf_subband_aac {
            ui = *freq_band_table
                .offset((j as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                as WORD32;
            pre_shift_val = (Some(
                ixheaacd_ixheaacd_expsubbandsamples.expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                anal_buf_real,
                anal_buf_imag,
                li,
                ui,
                start_pos,
                next_pos,
                low_pow_flag,
            ) as WORD32;
            pre_shift_val = (pre_shift_val as core::ffi::c_int - SHIFT_BEFORE_SQUARE)
                as WORD32;
            accumulate = 0 as core::ffi::c_int as WORD32;
            k = li;
            while k < ui {
                let mut pre_shift1: WORD32 = 16 as WORD32 - pre_shift_val;
                accu_line = 0 as WORD32;
                pre_shift1 = (if pre_shift1 < 31 as core::ffi::c_int {
                    pre_shift1 as core::ffi::c_int
                } else {
                    31 as core::ffi::c_int
                }) as WORD32;
                let mut ptr: *mut WORD32 = &mut *(*anal_buf_real
                    .offset(start_pos as isize))
                    .offset(k as isize) as *mut WORD32;
                let mut inc: WORD32 = (low_pow_flag == 0) as core::ffi::c_int;
                l = next_pos - start_pos << inc;
                while l != 0 as core::ffi::c_int {
                    let mut temp: WORD16 = 0;
                    temp = ixheaac_extract16l(
                        ixheaac_shr32_dir(*ptr, pre_shift1 as WORD),
                    );
                    ptr = ptr.offset(64 as core::ffi::c_int as isize);
                    accu_line = ixheaac_mac16x16in32_sat(accu_line, temp, temp);
                    l -= 1;
                }
                accumulate = ixheaac_add32_sat(
                    accumulate,
                    ixheaac_shr32(accu_line, 9 as WORD),
                );
                k += 1;
            }
            shift = ixheaac_pnorm32(accumulate) as WORD32;
            sum_m = ixheaac_extract16l(
                ixheaac_shr32_dir_sat_limit(accumulate, 16 as WORD - shift as WORD),
            );
            if sum_m as core::ffi::c_int == 0 as core::ffi::c_int {
                sum_e = 0 as core::ffi::c_int as WORD32;
            } else {
                sum_m = ixheaac_mult16_shl_sat(sum_m, inv_width);
                sum_m = ixheaac_mult16_shl_sat(
                    sum_m,
                    (*(*ptr_sbr_tables).env_calc_tables_ptr)
                        .sbr_inv_int_table[(ui - li) as usize],
                );
                sum_e = frame_exp + extra_shift - shift;
                sum_e = sum_e - (pre_shift_val << 1 as core::ffi::c_int);
            }
            k = li;
            while k < ui {
                let fresh41 = nrg_est;
                nrg_est = nrg_est.offset(1);
                *fresh41 = sum_m;
                let fresh42 = nrg_est;
                nrg_est = nrg_est.offset(1);
                *fresh42 = sum_e as WORD16;
                k += 1;
            }
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_subbandgain_calc(
    mut e_orig_mant_matrix: WORD16,
    mut tmp_noise_mant: WORD16,
    mut nrg_est_mant: WORD16,
    mut nrg_est_exp: WORD16,
    mut tmp_noise_exp: WORD16,
    mut nrg_ref_exp: WORD16,
    mut sine_present_flag: FLAG,
    mut sine_mapped_matrix: FLAG,
    mut noise_absc_flag: FLAG,
    mut ptr_nrg_gain_mant: *mut WORD16,
    mut ptr_noise_floor_mant: *mut WORD16,
    mut ptr_nrg_sine_m: *mut WORD16,
    mut pstr_common_tables: *mut ixheaacd_misc_tables,
) -> VOID {
    let mut var1_mant: WORD16 = 0;
    let mut var1_exp: WORD16 = 0;
    let mut var2_mant: WORD16 = 0;
    let mut var2_exp: WORD16 = 0;
    let mut var3_mant: WORD16 = 0;
    let mut var3_exp: WORD16 = 0;
    let mut temp: WORD32 = 0;
    if nrg_est_mant as core::ffi::c_int == 0 as core::ffi::c_int {
        nrg_est_mant = 0x4000 as WORD16;
        nrg_est_exp = 1 as WORD16;
    }
    var1_mant = ixheaac_mult16_shl_sat(e_orig_mant_matrix, tmp_noise_mant);
    var1_exp = (nrg_ref_exp as core::ffi::c_int + tmp_noise_exp as core::ffi::c_int)
        as WORD16;
    let mut accu: WORD32 = 0;
    let mut exp_diff: WORD32 = 0;
    exp_diff = (tmp_noise_exp as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    if exp_diff >= 0 as core::ffi::c_int {
        accu = tmp_noise_mant as WORD32
            + ixheaac_shr32(0x4000 as WORD32, exp_diff as WORD);
        var2_exp = tmp_noise_exp;
    } else {
        exp_diff = -exp_diff;
        accu = (ixheaac_shr32(tmp_noise_mant as WORD32, exp_diff as WORD)
            as core::ffi::c_int + 0x4000 as core::ffi::c_int) as WORD32;
        var2_exp = 1 as WORD16;
    }
    if ixheaac_abs32(accu) >= 0x8000 as core::ffi::c_int {
        accu = accu >> 1 as core::ffi::c_int;
        var2_exp += 1;
    }
    var2_mant = accu as WORD16;
    temp = ixheaacd_fix_mant_div(
        var1_mant,
        var2_mant,
        ptr_noise_floor_mant,
        pstr_common_tables,
    );
    *ptr_noise_floor_mant.offset(1 as core::ffi::c_int as isize) = (temp
        as core::ffi::c_int
        + (var1_exp as core::ffi::c_int - var2_exp as core::ffi::c_int)
        + 1 as core::ffi::c_int) as WORD16;
    if sine_present_flag != 0 || noise_absc_flag == 0 {
        var3_mant = ixheaac_mult16_shl_sat(var2_mant, nrg_est_mant);
        var3_exp = (var2_exp as core::ffi::c_int + nrg_est_exp as core::ffi::c_int)
            as WORD16;
    } else {
        var3_mant = nrg_est_mant;
        var3_exp = nrg_est_exp;
    }
    if sine_present_flag == 0 as core::ffi::c_int {
        var1_mant = e_orig_mant_matrix;
        var1_exp = nrg_ref_exp;
    }
    temp = ixheaacd_fix_mant_div(
        var1_mant,
        var3_mant,
        ptr_nrg_gain_mant,
        pstr_common_tables,
    );
    *ptr_nrg_gain_mant.offset(1 as core::ffi::c_int as isize) = (temp as core::ffi::c_int
        + (var1_exp as core::ffi::c_int - var3_exp as core::ffi::c_int)
        + 1 as core::ffi::c_int) as WORD16;
    if sine_present_flag != 0 && sine_mapped_matrix != 0 {
        temp = ixheaacd_fix_mant_div(
            e_orig_mant_matrix,
            var2_mant,
            ptr_nrg_sine_m,
            pstr_common_tables,
        );
        *ptr_nrg_sine_m.offset(1 as core::ffi::c_int as isize) = (temp
            as core::ffi::c_int
            + (nrg_ref_exp as core::ffi::c_int - var2_exp as core::ffi::c_int)
            + 1 as core::ffi::c_int) as WORD16;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_avggain_calc(
    mut ptr_enrg_orig: *mut WORD16,
    mut nrg_est: *mut WORD16,
    mut sub_band_start: WORD32,
    mut sub_band_end: WORD32,
    mut ptr_enrg_orig_mant: *mut WORD16,
    mut ptr_sum_ref_exp: *mut WORD16,
    mut ptr_avg_gain_mant: *mut WORD16,
    mut ptr_avg_gain_exp: *mut WORD16,
    mut pstr_common_tables: *mut ixheaacd_misc_tables,
    mut flag: WORD32,
) -> VOID {
    let mut sum_orig_mant: WORD16 = 0;
    let mut sum_orig_exp: WORD16 = 0;
    let mut sum_est_mant: WORD16 = 0;
    let mut sum_est_exp: WORD16 = 0;
    let mut accu_sum_orig_mant: WORD32 = 0;
    let mut accu_sum_orig_exp: WORD32 = 0;
    let mut accu_sum_est_mant: WORD32 = 0;
    let mut accu_sum_est_exp: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut ptr_enrg_orig_buf: *mut WORD16 = 0 as *mut WORD16;
    let mut ptr_enrg_est_buf: *mut WORD16 = 0 as *mut WORD16;
    accu_sum_orig_mant = 0 as core::ffi::c_int as WORD32;
    accu_sum_orig_exp = 0 as core::ffi::c_int as WORD32;
    accu_sum_est_mant = 0 as core::ffi::c_int as WORD32;
    accu_sum_est_exp = 0 as core::ffi::c_int as WORD32;
    ptr_enrg_orig_buf = &mut *ptr_enrg_orig
        .offset((sub_band_start << 1 as core::ffi::c_int) as isize) as *mut WORD16;
    ptr_enrg_est_buf = &mut *nrg_est
        .offset((sub_band_start << 1 as core::ffi::c_int) as isize) as *mut WORD16;
    k = sub_band_end - sub_band_start;
    while k != 0 as core::ffi::c_int {
        let mut tmp_mant: WORD16 = 0;
        let mut tmp_e: WORD16 = 0;
        let mut tmp2_m: WORD16 = 0;
        let mut tmp2_e: WORD16 = 0;
        let fresh22 = ptr_enrg_orig_buf;
        ptr_enrg_orig_buf = ptr_enrg_orig_buf.offset(1);
        tmp_mant = *fresh22;
        let fresh23 = ptr_enrg_orig_buf;
        ptr_enrg_orig_buf = ptr_enrg_orig_buf.offset(1);
        tmp_e = *fresh23;
        let fresh24 = ptr_enrg_est_buf;
        ptr_enrg_est_buf = ptr_enrg_est_buf.offset(1);
        tmp2_m = *fresh24;
        let fresh25 = ptr_enrg_est_buf;
        ptr_enrg_est_buf = ptr_enrg_est_buf.offset(1);
        tmp2_e = *fresh25;
        let mut exp_diff: WORD32 = 0;
        exp_diff = tmp_e as WORD32 - accu_sum_orig_exp;
        if exp_diff >= 0 as core::ffi::c_int {
            accu_sum_orig_mant = tmp_mant as WORD32
                + ixheaac_shr32(accu_sum_orig_mant, exp_diff as WORD);
            accu_sum_orig_exp = tmp_e as WORD32;
        } else {
            exp_diff = -exp_diff;
            accu_sum_orig_mant = ixheaac_shr32(tmp_mant as WORD32, exp_diff as WORD)
                + accu_sum_orig_mant;
        }
        if flag != 0 {
            tmp_mant = (tmp_mant as core::ffi::c_int * tmp2_m as core::ffi::c_int
                >> 16 as core::ffi::c_int) as WORD16;
            tmp_e = (tmp_e as core::ffi::c_int + tmp2_e as core::ffi::c_int
                + 1 as core::ffi::c_int) as WORD16;
        } else {
            tmp_mant = tmp2_m;
            tmp_e = tmp2_e;
        }
        let mut exp_diff_0: WORD32 = 0;
        exp_diff_0 = tmp_e as WORD32 - accu_sum_est_exp;
        if exp_diff_0 >= 0 as core::ffi::c_int {
            accu_sum_est_mant = tmp_mant as WORD32
                + ixheaac_shr32(accu_sum_est_mant, exp_diff_0 as WORD);
            accu_sum_est_exp = tmp_e as WORD32;
        } else {
            exp_diff_0 = -exp_diff_0;
            accu_sum_est_mant = ixheaac_shr32(tmp_mant as WORD32, exp_diff_0 as WORD)
                + accu_sum_est_mant;
        }
        k -= 1;
    }
    let mut norm_val: WORD32 = 0;
    norm_val = (16 as WORD - ixheaac_pnorm32(accu_sum_orig_mant)) as WORD32;
    if norm_val > 0 as core::ffi::c_int {
        accu_sum_orig_mant >>= norm_val;
        accu_sum_orig_exp += norm_val;
    }
    norm_val = (16 as WORD - ixheaac_pnorm32(accu_sum_est_mant)) as WORD32;
    if norm_val > 0 as core::ffi::c_int {
        accu_sum_est_mant >>= norm_val;
        accu_sum_est_exp += norm_val;
    }
    if flag == 0 {
        sum_orig_mant = accu_sum_orig_mant as WORD16;
        sum_orig_exp = accu_sum_orig_exp as WORD16;
        sum_est_mant = accu_sum_est_mant as WORD16;
        sum_est_exp = accu_sum_est_exp as WORD16;
    } else {
        sum_est_mant = accu_sum_orig_mant as WORD16;
        sum_est_exp = accu_sum_orig_exp as WORD16;
        sum_orig_mant = accu_sum_est_mant as WORD16;
        sum_orig_exp = accu_sum_est_exp as WORD16;
    }
    temp = ixheaacd_fix_mant_div(
        sum_orig_mant,
        sum_est_mant,
        ptr_avg_gain_mant,
        pstr_common_tables,
    );
    *ptr_avg_gain_exp = (temp as core::ffi::c_int
        + (sum_orig_exp as core::ffi::c_int - sum_est_exp as core::ffi::c_int)
        + 1 as core::ffi::c_int) as WORD16;
    *ptr_enrg_orig_mant = sum_orig_mant;
    *ptr_sum_ref_exp = sum_orig_exp;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_harm_idx_zerotwolp_dec(
    mut ptr_real_buf: *mut WORD32,
    mut ptr_gain_buf: *mut WORD16,
    mut scale_change: WORD32,
    mut ptr_sine_level_buf: *mut WORD16,
    mut ptr_rand_ph: *const WORD32,
    mut noise_level_mant: *mut WORD16,
    mut num_sub_bands: WORD32,
    mut noise_absc_flag: FLAG,
    mut harm_index: WORD32,
) -> VOID {
    let mut shift: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut signal_real: WORD32 = 0;
    let mut sine_level: WORD32 = 0;
    scale_change = (scale_change as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    if noise_absc_flag == 0 {
        k = 0 as core::ffi::c_int as WORD32;
        while k < num_sub_bands {
            let fresh51 = ptr_gain_buf;
            ptr_gain_buf = ptr_gain_buf.offset(1);
            signal_real = ixheaac_mult32x16in32(*ptr_real_buf, *fresh51);
            let fresh52 = ptr_gain_buf;
            ptr_gain_buf = ptr_gain_buf.offset(1);
            shift = *fresh52 as WORD32 - scale_change;
            if shift > 0 as core::ffi::c_int {
                signal_real = signal_real << shift;
            } else {
                signal_real = signal_real >> -shift;
            }
            sine_level = ((*ptr_sine_level_buf.offset((2 as WORD32 * k) as isize)
                as core::ffi::c_int) << 16 as core::ffi::c_int) as WORD32;
            if sine_level == 0 as core::ffi::c_int {
                let fresh53 = ptr_real_buf;
                ptr_real_buf = ptr_real_buf.offset(1);
                *fresh53 = ixheaac_mac16x16in32_shl_sat(
                    signal_real,
                    ixheaac_extract16h(*ptr_rand_ph.offset(k as isize)),
                    *noise_level_mant.offset((2 as WORD32 * k) as isize),
                );
            } else if harm_index == 0 as core::ffi::c_int {
                let fresh54 = ptr_real_buf;
                ptr_real_buf = ptr_real_buf.offset(1);
                *fresh54 = ixheaac_add32_sat(signal_real, sine_level);
            } else {
                let fresh55 = ptr_real_buf;
                ptr_real_buf = ptr_real_buf.offset(1);
                *fresh55 = ixheaac_sub32_sat(signal_real, sine_level);
            }
            k += 1;
        }
    } else {
        k = 0 as core::ffi::c_int as WORD32;
        while k < num_sub_bands {
            let fresh56 = ptr_gain_buf;
            ptr_gain_buf = ptr_gain_buf.offset(1);
            signal_real = ixheaac_mult32x16in32(*ptr_real_buf, *fresh56);
            let fresh57 = ptr_gain_buf;
            ptr_gain_buf = ptr_gain_buf.offset(1);
            shift = *fresh57 as WORD32 - scale_change;
            if shift > 0 as core::ffi::c_int {
                signal_real = signal_real << shift;
            } else {
                signal_real = signal_real >> -shift;
            }
            sine_level = ((*ptr_sine_level_buf.offset((2 as WORD32 * k) as isize)
                as core::ffi::c_int) << 16 as core::ffi::c_int) as WORD32;
            if harm_index == 0 as core::ffi::c_int {
                let fresh58 = ptr_real_buf;
                ptr_real_buf = ptr_real_buf.offset(1);
                *fresh58 = ixheaac_add32_sat(signal_real, sine_level);
            } else {
                let fresh59 = ptr_real_buf;
                ptr_real_buf = ptr_real_buf.offset(1);
                *fresh59 = ixheaac_sub32_sat(signal_real, sine_level);
            }
            k += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_harm_idx_onethreelp(
    mut ptr_real_buf: *mut WORD32,
    mut ptr_gain_buf: *mut WORD16,
    mut scale_change: WORD32,
    mut ptr_sine_level_buf: *mut WORD16,
    mut ptr_rand_ph: *const WORD32,
    mut noise_level_mant: *mut WORD16,
    mut num_sub_bands: WORD32,
    mut noise_absc_flag: FLAG,
    mut freq_inv_flag: WORD32,
    mut noise_e: WORD32,
    mut sub_band_start: WORD32,
) -> VOID {
    let mut shift: WORD32 = 0;
    let mut k: WORD32 = 0 as WORD32;
    let mut signal_real: WORD32 = 0;
    let mut temp_mult: WORD32 = 0;
    let mut temp_mult2: WORD32 = 0;
    let mut sine_level: WORD16 = 0;
    let mut sine_level_prev: WORD16 = 0;
    let mut sine_level_next: WORD16 = 0;
    let mut tone_count: WORD32 = 0 as WORD32;
    let mut tmp: WORD16 = 0;
    scale_change = (scale_change as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    let fresh7 = ptr_gain_buf;
    ptr_gain_buf = ptr_gain_buf.offset(1);
    signal_real = ixheaac_mult32x16in32(*ptr_real_buf, *fresh7);
    let fresh8 = ptr_gain_buf;
    ptr_gain_buf = ptr_gain_buf.offset(1);
    shift = *fresh8 as WORD32 - scale_change;
    if shift > 0 as core::ffi::c_int {
        signal_real = signal_real << shift;
    } else {
        signal_real = signal_real >> -shift;
    }
    sine_level = *ptr_sine_level_buf
        .offset((2 as core::ffi::c_int * 0 as core::ffi::c_int) as isize);
    if num_sub_bands > 1 as core::ffi::c_int {
        sine_level_next = *ptr_sine_level_buf
            .offset((2 as core::ffi::c_int * 1 as core::ffi::c_int) as isize);
    } else {
        sine_level_next = 0 as WORD16;
    }
    if *ptr_sine_level_buf
        .offset((2 as core::ffi::c_int * 0 as core::ffi::c_int) as isize)
        as core::ffi::c_int != 0 as core::ffi::c_int
    {
        tone_count += 1;
    } else if noise_absc_flag == 0 {
        signal_real = ixheaac_mac16x16in32_shl_sat(
            signal_real,
            ixheaac_extract16h(*ptr_rand_ph.offset(k as isize)),
            *noise_level_mant,
        );
    }
    noise_level_mant = noise_level_mant.offset(2 as core::ffi::c_int as isize);
    temp_mult2 = ixheaac_mult32x16in32(FACTOR, sine_level_next);
    temp_mult = ixheaac_mult32x16in32(FACTOR, sine_level);
    tmp = noise_e as WORD16;
    if tmp as core::ffi::c_int > 0 as core::ffi::c_int {
        temp_mult = ixheaac_shl32(temp_mult, tmp as WORD);
    } else {
        temp_mult = ixheaac_shr32(temp_mult, -(tmp as WORD));
    }
    if freq_inv_flag < 0 as core::ffi::c_int {
        *ptr_real_buf.offset(-(1 as core::ffi::c_int as isize)) = ixheaac_add32_sat(
            *ptr_real_buf.offset(-(1 as core::ffi::c_int as isize)),
            temp_mult,
        );
        signal_real = ixheaac_sub32_sat(signal_real, temp_mult2);
    } else {
        *ptr_real_buf.offset(-(1 as core::ffi::c_int as isize)) = ixheaac_sub32_sat(
            *ptr_real_buf.offset(-(1 as core::ffi::c_int as isize)),
            temp_mult,
        );
        signal_real = ixheaac_add32_sat(signal_real, temp_mult2);
    }
    let fresh9 = ptr_real_buf;
    ptr_real_buf = ptr_real_buf.offset(1);
    *fresh9 = signal_real;
    num_sub_bands = (num_sub_bands as core::ffi::c_int - 1 as core::ffi::c_int)
        as WORD32;
    k = 1 as core::ffi::c_int as WORD32;
    while k < num_sub_bands {
        let fresh10 = ptr_gain_buf;
        ptr_gain_buf = ptr_gain_buf.offset(1);
        let mut gain_m: WORD16 = *fresh10;
        let fresh11 = ptr_gain_buf;
        ptr_gain_buf = ptr_gain_buf.offset(1);
        let mut gain_e: WORD16 = *fresh11;
        let mut q_real: WORD32 = *ptr_real_buf;
        signal_real = ixheaac_mult32x16in32(q_real, gain_m);
        shift = gain_e as WORD32 - scale_change;
        if shift >= 0 as core::ffi::c_int {
            signal_real = signal_real << shift;
        } else {
            signal_real = signal_real >> -shift;
        }
        sine_level_prev = sine_level;
        sine_level = sine_level_next;
        if sine_level as core::ffi::c_int != 0 as core::ffi::c_int {
            tone_count += 1;
        }
        sine_level_next = *ptr_sine_level_buf
            .offset(
                (2 as core::ffi::c_int * (k as core::ffi::c_int + 1 as core::ffi::c_int))
                    as isize,
            );
        if noise_absc_flag == 0
            && sine_level as core::ffi::c_int == 0 as core::ffi::c_int
        {
            signal_real = ixheaac_mac16x16in32_shl_sat(
                signal_real,
                ixheaac_extract16h(*ptr_rand_ph.offset(k as isize)),
                *noise_level_mant,
            );
        }
        noise_level_mant = noise_level_mant.offset(2 as core::ffi::c_int as isize);
        if tone_count <= 16 as core::ffi::c_int {
            let mut temp_mult_0: WORD32 = 0;
            let mut add_sine: WORD32 = ixheaac_mult32x16in32(
                FACTOR,
                ixheaac_sub16(sine_level_prev, sine_level_next),
            );
            temp_mult_0 = add_sine * freq_inv_flag;
            signal_real = ixheaac_add32_sat(signal_real, temp_mult_0);
        }
        let fresh12 = ptr_real_buf;
        ptr_real_buf = ptr_real_buf.offset(1);
        *fresh12 = signal_real;
        freq_inv_flag = -freq_inv_flag;
        k += 1;
    }
    freq_inv_flag = (freq_inv_flag as core::ffi::c_int + 1 as core::ffi::c_int
        >> 1 as core::ffi::c_int) as WORD32;
    if num_sub_bands > 0 as core::ffi::c_int {
        let mut temp_mult_sine: WORD32 = 0;
        let fresh13 = ptr_gain_buf;
        ptr_gain_buf = ptr_gain_buf.offset(1);
        signal_real = ixheaac_mult32x16in32(*ptr_real_buf, *fresh13);
        shift = *ptr_gain_buf as WORD32 - scale_change;
        if shift > 0 as core::ffi::c_int {
            signal_real = signal_real << shift;
        } else {
            signal_real = signal_real >> -shift;
        }
        temp_mult_sine = ixheaac_mult32x16in32(FACTOR, sine_level);
        sine_level = sine_level_next;
        if sine_level as core::ffi::c_int != 0 as core::ffi::c_int {
            tone_count += 1;
        } else if noise_absc_flag == 0 {
            signal_real = ixheaac_mac16x16in32_shl_sat(
                signal_real,
                ixheaac_extract16h(*ptr_rand_ph.offset(k as isize)),
                *noise_level_mant,
            );
        }
        if tone_count <= 16 as core::ffi::c_int {
            temp_mult2 = ixheaac_mult32x16in32(FACTOR, sine_level);
            if freq_inv_flag != 0 {
                let fresh14 = ptr_real_buf;
                ptr_real_buf = ptr_real_buf.offset(1);
                *fresh14 = ixheaac_add32_sat(signal_real, temp_mult_sine);
                if k + sub_band_start < 62 as core::ffi::c_int {
                    *ptr_real_buf = ixheaac_sub32_sat(*ptr_real_buf, temp_mult2);
                }
            } else {
                let fresh15 = ptr_real_buf;
                ptr_real_buf = ptr_real_buf.offset(1);
                *fresh15 = ixheaac_sub32_sat(signal_real, temp_mult_sine);
                if k + sub_band_start < 62 as core::ffi::c_int {
                    *ptr_real_buf = ixheaac_add32_sat(*ptr_real_buf, temp_mult2);
                }
            }
        } else {
            *ptr_real_buf = signal_real;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_harm_idx_zerotwo(
    mut noise_absc_flag: FLAG,
    mut num_sub_bands: WORD16,
    mut ptr_real_buf: *mut WORD32,
    mut ptr_imag: *mut WORD32,
    mut smoothed_gain: *mut WORD16,
    mut smoothed_noise: *mut WORD16,
    mut factor: WORD32,
    mut ptr_gain_buf: *mut WORD16,
    mut scale_change: WORD16,
    mut ptr_rand_ph: *const WORD32,
    mut ptr_sine_level_buf: *mut WORD16,
    mut noise_e: WORD16,
    mut harm_index: WORD32,
) -> VOID {
    let mut k: WORD32 = 0;
    let mut signal_real: WORD32 = 0;
    let mut sig_imag: WORD32 = 0;
    let mut shift: WORD32 = 0;
    let mut sine_level: WORD32 = 0;
    ptr_gain_buf = ptr_gain_buf.offset(1);
    k = 0 as core::ffi::c_int as WORD32;
    while k < num_sub_bands as core::ffi::c_int {
        signal_real = ixheaac_mult32x16in32(
            *ptr_real_buf,
            *smoothed_gain.offset(0 as core::ffi::c_int as isize),
        );
        sig_imag = ixheaac_mult32x16in32(
            *ptr_imag,
            *smoothed_gain.offset(0 as core::ffi::c_int as isize),
        );
        shift = ixheaac_sub16(*ptr_gain_buf, scale_change) as WORD32;
        ptr_gain_buf = ptr_gain_buf.offset(2 as core::ffi::c_int as isize);
        if shift > 0 as core::ffi::c_int {
            signal_real = ixheaac_shl32(signal_real, shift as WORD);
            sig_imag = ixheaac_shl32(sig_imag, shift as WORD);
        } else {
            shift = -shift;
            signal_real = ixheaac_shr32(signal_real, shift as WORD);
            sig_imag = ixheaac_shr32(sig_imag, shift as WORD);
        }
        ptr_rand_ph = ptr_rand_ph.offset(1);
        if *ptr_sine_level_buf as core::ffi::c_int != 0 as core::ffi::c_int {
            let mut tmp: WORD32 = ixheaac_sub16(
                *ptr_sine_level_buf.offset(1 as core::ffi::c_int as isize),
                noise_e,
            ) as WORD32;
            if tmp > 0 as core::ffi::c_int {
                sine_level = ixheaac_shl32(
                    *ptr_sine_level_buf.offset(0 as core::ffi::c_int as isize) as WORD32,
                    tmp as WORD,
                );
            } else {
                sine_level = ixheaac_shr32(
                    *ptr_sine_level_buf.offset(0 as core::ffi::c_int as isize) as WORD32,
                    tmp as WORD,
                );
            }
            if harm_index == 0 as core::ffi::c_int {
                *ptr_real_buf = ixheaac_add32_sat(signal_real, sine_level);
            } else {
                *ptr_real_buf = ixheaac_sub32_sat(signal_real, sine_level);
            }
            *ptr_imag = sig_imag;
        } else if noise_absc_flag == 0 {
            let mut random: WORD32 = *ptr_rand_ph;
            let mut noise: WORD16 = *smoothed_noise
                .offset(0 as core::ffi::c_int as isize);
            *ptr_real_buf = ixheaac_mac16x16in32_shl_sat(
                signal_real,
                ixheaac_extract16h(random),
                noise,
            );
            *ptr_imag = ixheaac_mac16x16in32_shl_sat(
                sig_imag,
                ixheaac_extract16l(random),
                noise,
            );
        } else {
            *ptr_real_buf = signal_real;
            *ptr_imag = sig_imag;
        }
        smoothed_noise = smoothed_noise.offset(factor as isize);
        smoothed_gain = smoothed_gain.offset(2 as core::ffi::c_int as isize);
        ptr_sine_level_buf = ptr_sine_level_buf.offset(2 as core::ffi::c_int as isize);
        ptr_real_buf = ptr_real_buf.offset(1);
        ptr_imag = ptr_imag.offset(1);
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_harm_idx_onethree(
    mut noise_absc_flag: FLAG,
    mut num_sub_bands: WORD16,
    mut ptr_real_buf: *mut WORD32,
    mut ptr_imag: *mut WORD32,
    mut smoothed_gain: *mut WORD16,
    mut smoothed_noise: *mut WORD16,
    mut factor: WORD32,
    mut ptr_gain_buf: *mut WORD16,
    mut scale_change: WORD16,
    mut ptr_rand_ph: *const WORD32,
    mut ptr_sine_level_buf: *mut WORD16,
    mut noise_e: WORD16,
    mut freq_inv_flag: WORD32,
    mut harm_index: WORD32,
) -> VOID {
    let mut k: WORD32 = 0;
    let mut signal_real: WORD32 = 0;
    let mut sig_imag: WORD32 = 0;
    let mut shift: WORD32 = 0;
    let mut sine_level: WORD32 = 0;
    ptr_gain_buf = ptr_gain_buf.offset(1);
    if harm_index == 1 as core::ffi::c_int {
        freq_inv_flag = (freq_inv_flag == 0) as core::ffi::c_int as WORD32;
    }
    k = 0 as core::ffi::c_int as WORD32;
    while k < num_sub_bands as core::ffi::c_int {
        signal_real = ixheaac_mult32x16in32(
            *ptr_real_buf,
            *smoothed_gain.offset(0 as core::ffi::c_int as isize),
        );
        sig_imag = ixheaac_mult32x16in32(
            *ptr_imag,
            *smoothed_gain.offset(0 as core::ffi::c_int as isize),
        );
        shift = ixheaac_sub16(*ptr_gain_buf, scale_change) as WORD32;
        ptr_gain_buf = ptr_gain_buf.offset(2 as core::ffi::c_int as isize);
        if shift > 0 as core::ffi::c_int {
            signal_real = ixheaac_shl32(signal_real, shift as WORD);
            sig_imag = ixheaac_shl32(sig_imag, shift as WORD);
        } else {
            shift = -shift;
            signal_real = ixheaac_shr32(signal_real, shift as WORD);
            sig_imag = ixheaac_shr32(sig_imag, shift as WORD);
        }
        ptr_rand_ph = ptr_rand_ph.offset(1);
        if *ptr_sine_level_buf as core::ffi::c_int != 0 as core::ffi::c_int {
            let mut tmp: WORD32 = ixheaac_sub16(
                *ptr_sine_level_buf.offset(1 as core::ffi::c_int as isize),
                noise_e,
            ) as WORD32;
            if tmp > 0 as core::ffi::c_int {
                sine_level = ixheaac_shl32(
                    *ptr_sine_level_buf.offset(0 as core::ffi::c_int as isize) as WORD32,
                    tmp as WORD,
                );
            } else {
                sine_level = ixheaac_shr32(
                    *ptr_sine_level_buf.offset(0 as core::ffi::c_int as isize) as WORD32,
                    -(tmp as WORD),
                );
            }
            *ptr_real_buf = signal_real;
            if freq_inv_flag != 0 {
                *ptr_imag = ixheaac_add32_sat(sig_imag, sine_level);
            } else {
                *ptr_imag = ixheaac_sub32_sat(sig_imag, sine_level);
            }
        } else if noise_absc_flag == 0 {
            let mut random: WORD32 = *ptr_rand_ph;
            let mut noise: WORD16 = *smoothed_noise
                .offset(0 as core::ffi::c_int as isize);
            *ptr_real_buf = ixheaac_mac16x16in32_shl_sat(
                signal_real,
                ixheaac_extract16h(random),
                noise,
            );
            *ptr_imag = ixheaac_mac16x16in32_shl_sat(
                sig_imag,
                ixheaac_extract16l(random),
                noise,
            );
        } else {
            *ptr_real_buf = signal_real;
            *ptr_imag = sig_imag;
        }
        freq_inv_flag = (freq_inv_flag == 0) as core::ffi::c_int as WORD32;
        smoothed_gain = smoothed_gain.offset(2 as core::ffi::c_int as isize);
        smoothed_noise = smoothed_noise.offset(factor as isize);
        ptr_sine_level_buf = ptr_sine_level_buf.offset(2 as core::ffi::c_int as isize);
        ptr_real_buf = ptr_real_buf.offset(1);
        ptr_imag = ptr_imag.offset(1);
        k += 1;
    }
}
