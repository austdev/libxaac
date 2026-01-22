extern "C" {
    fn pow(__x: core::ffi::c_double, __y: core::ffi::c_double) -> core::ffi::c_double;
    fn floor(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn ixheaacd_byte_align(
        it_bit_buff: *mut ia_bit_buf_struct,
        ptr_byte_align_bits: *mut WORD32,
    ) -> VOID;
    fn ixheaacd_read_bits_buf(
        it_bit_buff: *mut ia_bit_buf_struct,
        no_of_bits: WORD,
    ) -> WORD32;
    fn ixheaacd_mps_ecdatapairdec(
        it_bit_buf: *mut ia_bit_buf_struct,
        aa_out_data: *mut [WORD32; 28],
        a_history: *mut WORD32,
        data_type: WORD32,
        set_idx: WORD32,
        start_band: WORD32,
        data_bands: WORD32,
        pair_flag: WORD32,
        coarse_flag: WORD32,
        independency_flag: WORD32,
        ldmps_flag: WORD32,
        heaac_mps_present: WORD32,
        ec_flag: WORD32,
    ) -> WORD32;
    fn ixheaacd_mps_huff_decode(
        it_bit_buf: *mut ia_bit_buf_struct,
        out_data: *mut WORD32,
        num_val: WORD32,
    ) -> VOID;
}
pub type WORD8 = core::ffi::c_schar;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD16 = core::ffi::c_short;
pub type UWORD16 = core::ffi::c_ushort;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type UWORD64 = core::ffi::c_ulonglong;
pub type FLOAT32 = core::ffi::c_float;
pub type VOID = ();
pub type FLAG = core::ffi::c_int;
pub type WORD = core::ffi::c_int;
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
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const MAX_PARAMETER_BANDS: core::ffi::c_int = 28 as core::ffi::c_int;
pub const QMF_BANDS_TO_HYBRID: core::ffi::c_int = 3 as core::ffi::c_int;
pub const MAX_TIME_SLOTS: core::ffi::c_int = 72 as core::ffi::c_int;
static mut ixheaacd_freq_res_table: [WORD32; 8] = [
    0 as core::ffi::c_int,
    28 as core::ffi::c_int,
    20 as core::ffi::c_int,
    14 as core::ffi::c_int,
    10 as core::ffi::c_int,
    7 as core::ffi::c_int,
    5 as core::ffi::c_int,
    4 as core::ffi::c_int,
];
static mut ixheaacd_freq_res_table_ld: [WORD32; 8] = [
    0 as core::ffi::c_int,
    23 as core::ffi::c_int,
    15 as core::ffi::c_int,
    12 as core::ffi::c_int,
    9 as core::ffi::c_int,
    7 as core::ffi::c_int,
    5 as core::ffi::c_int,
    4 as core::ffi::c_int,
];
static mut ixheaacd_hybrid_band_71_to_processing_band_4_map: [WORD32; 71] = [
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    1 as core::ffi::c_int,
    1 as core::ffi::c_int,
    1 as core::ffi::c_int,
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
];
static mut ixheaacd_hybrid_band_71_to_processing_band_5_map: [WORD32; 71] = [
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    1 as core::ffi::c_int,
    1 as core::ffi::c_int,
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
];
static mut ixheaacd_hybrid_band_71_to_processing_band_7_map: [WORD32; 71] = [
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
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    5 as core::ffi::c_int,
    5 as core::ffi::c_int,
    5 as core::ffi::c_int,
    5 as core::ffi::c_int,
    5 as core::ffi::c_int,
    5 as core::ffi::c_int,
    5 as core::ffi::c_int,
    5 as core::ffi::c_int,
    5 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
];
static mut ixheaacd_hybrid_band_71_to_processing_band_10_map: [WORD32; 71] = [
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    5 as core::ffi::c_int,
    5 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    7 as core::ffi::c_int,
    7 as core::ffi::c_int,
    7 as core::ffi::c_int,
    7 as core::ffi::c_int,
    7 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
];
static mut ixheaacd_hybrid_band_71_to_processing_band_14_map: [WORD32; 71] = [
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    5 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    7 as core::ffi::c_int,
    7 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    10 as core::ffi::c_int,
    10 as core::ffi::c_int,
    10 as core::ffi::c_int,
    10 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    12 as core::ffi::c_int,
    12 as core::ffi::c_int,
    12 as core::ffi::c_int,
    12 as core::ffi::c_int,
    12 as core::ffi::c_int,
    12 as core::ffi::c_int,
    12 as core::ffi::c_int,
    12 as core::ffi::c_int,
    12 as core::ffi::c_int,
    12 as core::ffi::c_int,
    12 as core::ffi::c_int,
    12 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
];
#[no_mangle]
pub static mut ixheaacd_hybrid_band_71_to_processing_band_20_map: [WORD32; 71] = [
    1 as core::ffi::c_int,
    0 as core::ffi::c_int,
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
    14 as core::ffi::c_int,
    15 as core::ffi::c_int,
    15 as core::ffi::c_int,
    15 as core::ffi::c_int,
    16 as core::ffi::c_int,
    16 as core::ffi::c_int,
    16 as core::ffi::c_int,
    16 as core::ffi::c_int,
    17 as core::ffi::c_int,
    17 as core::ffi::c_int,
    17 as core::ffi::c_int,
    17 as core::ffi::c_int,
    17 as core::ffi::c_int,
    18 as core::ffi::c_int,
    18 as core::ffi::c_int,
    18 as core::ffi::c_int,
    18 as core::ffi::c_int,
    18 as core::ffi::c_int,
    18 as core::ffi::c_int,
    18 as core::ffi::c_int,
    18 as core::ffi::c_int,
    18 as core::ffi::c_int,
    18 as core::ffi::c_int,
    18 as core::ffi::c_int,
    18 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
];
#[no_mangle]
pub static mut ixheaacd_hybrid_band_71_to_processing_band_28_map: [WORD32; 71] = [
    1 as core::ffi::c_int,
    0 as core::ffi::c_int,
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
    17 as core::ffi::c_int,
    18 as core::ffi::c_int,
    18 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    20 as core::ffi::c_int,
    20 as core::ffi::c_int,
    21 as core::ffi::c_int,
    21 as core::ffi::c_int,
    21 as core::ffi::c_int,
    22 as core::ffi::c_int,
    22 as core::ffi::c_int,
    22 as core::ffi::c_int,
    23 as core::ffi::c_int,
    23 as core::ffi::c_int,
    23 as core::ffi::c_int,
    23 as core::ffi::c_int,
    24 as core::ffi::c_int,
    24 as core::ffi::c_int,
    24 as core::ffi::c_int,
    24 as core::ffi::c_int,
    24 as core::ffi::c_int,
    25 as core::ffi::c_int,
    25 as core::ffi::c_int,
    25 as core::ffi::c_int,
    25 as core::ffi::c_int,
    25 as core::ffi::c_int,
    25 as core::ffi::c_int,
    26 as core::ffi::c_int,
    26 as core::ffi::c_int,
    26 as core::ffi::c_int,
    26 as core::ffi::c_int,
    26 as core::ffi::c_int,
    26 as core::ffi::c_int,
    26 as core::ffi::c_int,
    27 as core::ffi::c_int,
    27 as core::ffi::c_int,
    27 as core::ffi::c_int,
    27 as core::ffi::c_int,
    27 as core::ffi::c_int,
    27 as core::ffi::c_int,
    27 as core::ffi::c_int,
    27 as core::ffi::c_int,
    27 as core::ffi::c_int,
    27 as core::ffi::c_int,
    27 as core::ffi::c_int,
    27 as core::ffi::c_int,
    27 as core::ffi::c_int,
    27 as core::ffi::c_int,
    27 as core::ffi::c_int,
    27 as core::ffi::c_int,
];
#[no_mangle]
pub static mut ixheaacd_hybrid_band_64_to_processing_band_4_map: [WORD32; 71] = [
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    1 as core::ffi::c_int,
    1 as core::ffi::c_int,
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
];
#[no_mangle]
pub static mut ixheaacd_hybrid_band_64_to_processing_band_5_map: [WORD32; 71] = [
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
];
#[no_mangle]
pub static mut ixheaacd_hybrid_band_64_to_processing_band_7_map: [WORD32; 71] = [
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    5 as core::ffi::c_int,
    5 as core::ffi::c_int,
    5 as core::ffi::c_int,
    5 as core::ffi::c_int,
    5 as core::ffi::c_int,
    5 as core::ffi::c_int,
    5 as core::ffi::c_int,
    5 as core::ffi::c_int,
    5 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
];
#[no_mangle]
pub static mut ixheaacd_hybrid_band_64_to_processing_band_9_map: [WORD32; 71] = [
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    5 as core::ffi::c_int,
    5 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    7 as core::ffi::c_int,
    7 as core::ffi::c_int,
    7 as core::ffi::c_int,
    7 as core::ffi::c_int,
    7 as core::ffi::c_int,
    7 as core::ffi::c_int,
    7 as core::ffi::c_int,
    7 as core::ffi::c_int,
    7 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
];
#[no_mangle]
pub static mut ixheaacd_hybrid_band_64_to_processing_band_12_map: [WORD32; 71] = [
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    5 as core::ffi::c_int,
    5 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    7 as core::ffi::c_int,
    7 as core::ffi::c_int,
    7 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    10 as core::ffi::c_int,
    10 as core::ffi::c_int,
    10 as core::ffi::c_int,
    10 as core::ffi::c_int,
    10 as core::ffi::c_int,
    10 as core::ffi::c_int,
    10 as core::ffi::c_int,
    10 as core::ffi::c_int,
    10 as core::ffi::c_int,
    10 as core::ffi::c_int,
    10 as core::ffi::c_int,
    10 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
];
#[no_mangle]
pub static mut ixheaacd_hybrid_band_64_to_processing_band_15_map: [WORD32; 71] = [
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
    9 as core::ffi::c_int,
    10 as core::ffi::c_int,
    10 as core::ffi::c_int,
    10 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    11 as core::ffi::c_int,
    12 as core::ffi::c_int,
    12 as core::ffi::c_int,
    12 as core::ffi::c_int,
    12 as core::ffi::c_int,
    12 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
];
#[no_mangle]
pub static mut ixheaacd_hybrid_band_64_to_processing_band_23_map: [WORD32; 71] = [
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
    12 as core::ffi::c_int,
    13 as core::ffi::c_int,
    13 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    15 as core::ffi::c_int,
    15 as core::ffi::c_int,
    16 as core::ffi::c_int,
    16 as core::ffi::c_int,
    16 as core::ffi::c_int,
    17 as core::ffi::c_int,
    17 as core::ffi::c_int,
    17 as core::ffi::c_int,
    18 as core::ffi::c_int,
    18 as core::ffi::c_int,
    18 as core::ffi::c_int,
    18 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    19 as core::ffi::c_int,
    20 as core::ffi::c_int,
    20 as core::ffi::c_int,
    20 as core::ffi::c_int,
    20 as core::ffi::c_int,
    20 as core::ffi::c_int,
    20 as core::ffi::c_int,
    21 as core::ffi::c_int,
    21 as core::ffi::c_int,
    21 as core::ffi::c_int,
    21 as core::ffi::c_int,
    21 as core::ffi::c_int,
    21 as core::ffi::c_int,
    21 as core::ffi::c_int,
    22 as core::ffi::c_int,
    22 as core::ffi::c_int,
    22 as core::ffi::c_int,
    22 as core::ffi::c_int,
    22 as core::ffi::c_int,
    22 as core::ffi::c_int,
    22 as core::ffi::c_int,
    22 as core::ffi::c_int,
    22 as core::ffi::c_int,
    22 as core::ffi::c_int,
    22 as core::ffi::c_int,
    22 as core::ffi::c_int,
    22 as core::ffi::c_int,
    22 as core::ffi::c_int,
    22 as core::ffi::c_int,
    22 as core::ffi::c_int,
    22 as core::ffi::c_int,
    22 as core::ffi::c_int,
    22 as core::ffi::c_int,
    22 as core::ffi::c_int,
    22 as core::ffi::c_int,
    22 as core::ffi::c_int,
    22 as core::ffi::c_int,
];
static mut ixheaacd_mps_clip_gain_table: [FLOAT32; 8] = [
    1.000000f32,
    1.189207f32,
    1.414213f32,
    1.681792f32,
    2.000000f32,
    2.378414f32,
    2.828427f32,
    4.000000f32,
];
static mut ixheaacd_mps_stride_table: [WORD32; 4] = [
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    5 as core::ffi::c_int,
    28 as core::ffi::c_int,
];
static mut ixheaacd_cld_de_quant_table: [FLOAT32; 31] = [
    -150.0f64 as FLOAT32,
    -45.0f64 as FLOAT32,
    -40.0f64 as FLOAT32,
    -35.0f64 as FLOAT32,
    -30.0f64 as FLOAT32,
    -25.0f64 as FLOAT32,
    -22.0f64 as FLOAT32,
    -19.0f64 as FLOAT32,
    -16.0f64 as FLOAT32,
    -13.0f64 as FLOAT32,
    -10.0f64 as FLOAT32,
    -8.0f64 as FLOAT32,
    -6.0f64 as FLOAT32,
    -4.0f64 as FLOAT32,
    -2.0f64 as FLOAT32,
    0.0f64 as FLOAT32,
    2.0f64 as FLOAT32,
    4.0f64 as FLOAT32,
    6.0f64 as FLOAT32,
    8.0f64 as FLOAT32,
    10.0f64 as FLOAT32,
    13.0f64 as FLOAT32,
    16.0f64 as FLOAT32,
    19.0f64 as FLOAT32,
    22.0f64 as FLOAT32,
    25.0f64 as FLOAT32,
    30.0f64 as FLOAT32,
    35.0f64 as FLOAT32,
    40.0f64 as FLOAT32,
    45.0f64 as FLOAT32,
    150.0f64 as FLOAT32,
];
static mut ixheaacd_icc_de_quant_table: [FLOAT32; 8] = [
    1.0000f32,
    0.9370f32,
    0.84118f32,
    0.60092f32,
    0.36764f32,
    0.0f32,
    -0.5890f32,
    -0.9900f32,
];
#[no_mangle]
pub static mut ixheaacd_ipd_de_quant_table: [FLOAT32; 16] = [
    0.0f32,
    0.392699082f32,
    0.785398163f32,
    1.178097245f32,
    1.570796327f32,
    1.963495408f32,
    2.35619449f32,
    2.748893572f32,
    3.141592654f32,
    3.534291735f32,
    3.926990817f32,
    4.319689899f32,
    4.71238898f32,
    5.105088062f32,
    5.497787144f32,
    5.890486225f32,
];
#[no_mangle]
pub static mut ixheaacd_ipd_de_quant_table_q28: [WORD32; 16] = [
    0 as core::ffi::c_int,
    105414360 as core::ffi::c_int,
    210828720 as core::ffi::c_int,
    316243072 as core::ffi::c_int,
    421657440 as core::ffi::c_int,
    527071776 as core::ffi::c_int,
    632486144 as core::ffi::c_int,
    737900480 as core::ffi::c_int,
    843314880 as core::ffi::c_int,
    948729216 as core::ffi::c_int,
    1054143552 as core::ffi::c_int,
    1159557888 as core::ffi::c_int,
    1264972288 as core::ffi::c_int,
    1370386688 as core::ffi::c_int,
    1475800960 as core::ffi::c_int,
    1581215360 as core::ffi::c_int,
];
static mut ixheaacd_smoothing_time_table: [WORD32; 4] = [
    64 as core::ffi::c_int,
    128 as core::ffi::c_int,
    256 as core::ffi::c_int,
    512 as core::ffi::c_int,
];
static mut ixheaacd_inverse_smoothing_time_table: [FLOAT32; 4] = [
    1.0f32 / 64.0f32,
    1.0f32 / 128.0f32,
    1.0f32 / 256.0f32,
    1.0f32 / 512.0f32,
];
unsafe extern "C" fn bound_check(
    mut var: WORD32,
    mut lower_bound: WORD32,
    mut upper_bound: WORD32,
) -> WORD32 {
    var = if var < upper_bound { var } else { upper_bound };
    var = if var > lower_bound { var } else { lower_bound };
    return var;
}
unsafe extern "C" fn ixheaacd_longmult1(
    mut a: *mut UWORD16,
    mut b: UWORD16,
    mut d: *mut UWORD16,
    mut len: WORD32,
) -> VOID {
    let mut k: WORD32 = 0;
    let mut tmp: UWORD32 = 0;
    let mut b0: UWORD32 = b as UWORD32;
    tmp = (*a.offset(0 as core::ffi::c_int as isize) as UWORD32).wrapping_mul(b0);
    *d.offset(0 as core::ffi::c_int as isize) = tmp as UWORD16;
    k = 1 as core::ffi::c_int as WORD32;
    while k < len {
        tmp = (tmp >> 16 as core::ffi::c_int)
            .wrapping_add((*a.offset(k as isize) as UWORD32).wrapping_mul(b0));
        *d.offset(k as isize) = tmp as UWORD16;
        k += 1;
    }
}
unsafe extern "C" fn ixheaacd_longdiv(
    mut b: *mut UWORD16,
    mut a: UWORD16,
    mut d: *mut UWORD16,
    mut pr: *mut UWORD16,
    mut len: WORD32,
) -> VOID {
    let mut r: UWORD32 = 0;
    let mut tmp: UWORD32 = 0;
    let mut temp: UWORD32 = 0;
    let mut k: WORD32 = 0;
    if a as core::ffi::c_int == 0 as core::ffi::c_int {
        return;
    }
    r = 0 as UWORD32;
    k = (len as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while k >= 0 as core::ffi::c_int {
        tmp = (*b.offset(k as isize) as UWORD32)
            .wrapping_add(r << 16 as core::ffi::c_int);
        if tmp != 0 {
            *d.offset(k as isize) = tmp.wrapping_div(a as UWORD32) as UWORD16;
            temp = (*d.offset(k as isize) as core::ffi::c_int * a as core::ffi::c_int)
                as UWORD32;
            r = tmp.wrapping_sub(temp);
        } else {
            *d.offset(k as isize) = 0 as UWORD16;
        }
        k -= 1;
    }
    *pr = r as UWORD16;
}
unsafe extern "C" fn ixheaacd_longsub(
    mut a: *mut UWORD16,
    mut b: *mut UWORD16,
    mut lena: WORD32,
    mut lenb: WORD32,
) -> VOID {
    let mut h: WORD32 = 0;
    let mut carry: WORD32 = 0 as WORD32;
    if lenb > lena {
        return;
    }
    h = 0 as core::ffi::c_int as WORD32;
    while h < lenb {
        carry = carry
            + (*a.offset(h as isize) as core::ffi::c_int
                - *b.offset(h as isize) as core::ffi::c_int);
        *a.offset(h as isize) = carry as UWORD16;
        carry = carry >> 16 as core::ffi::c_int;
        h += 1;
    }
    while h < lena {
        carry = (*a.offset(h as isize) as UWORD32).wrapping_add(carry as UWORD32)
            as WORD32;
        *a.offset(h as isize) = carry as UWORD16;
        carry = carry >> 16 as core::ffi::c_int;
        h += 1;
    }
    if carry != 0 as core::ffi::c_int {
        return;
    }
}
unsafe extern "C" fn ixheaacd_longcompare(
    mut a: *mut UWORD16,
    mut b: *mut UWORD16,
    mut len: WORD32,
) -> WORD32 {
    let mut i: WORD32 = 0;
    i = (len as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while i > 0 as core::ffi::c_int {
        if *a.offset(i as isize) as core::ffi::c_int
            != *b.offset(i as isize) as core::ffi::c_int
        {
            break;
        }
        i -= 1;
    }
    return if *a.offset(i as isize) as core::ffi::c_int
        >= *b.offset(i as isize) as core::ffi::c_int
    {
        1 as WORD32
    } else {
        0 as WORD32
    };
}
unsafe extern "C" fn ixheaacd_mps_coarse2fine(
    mut data: *mut WORD32,
    mut data_type: WORD32,
    mut band_start: WORD32,
    mut ixheaacd_num_bands: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    i = band_start;
    while i < band_start + ixheaacd_num_bands {
        *data.offset(i as isize) <<= 1 as core::ffi::c_int;
        i += 1;
    }
    if data_type == CLD {
        i = band_start;
        while i < band_start + ixheaacd_num_bands {
            if *data.offset(i as isize) == -(14 as core::ffi::c_int) {
                *data.offset(i as isize) = -(15 as core::ffi::c_int) as WORD32;
            } else if *data.offset(i as isize) == 14 as core::ffi::c_int {
                *data.offset(i as isize) = 15 as core::ffi::c_int as WORD32;
            }
            i += 1;
        }
    }
}
unsafe extern "C" fn ixheaacd_mps_fine2coarse(
    mut data: *mut WORD32,
    mut ixheaacd_num_bands: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < ixheaacd_num_bands {
        let ref mut fresh0 = *data.offset(i as isize);
        *fresh0 /= 2 as core::ffi::c_int;
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_mps_getstridemap(
    mut freq_res_stride: WORD32,
    mut band_start: WORD32,
    mut band_stop: WORD32,
    mut strides: *mut WORD32,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut pb: WORD32 = 0;
    let mut ch_fac: WORD32 = 0;
    let mut data_bands: WORD32 = 0;
    let mut start_offset: WORD32 = 0;
    ch_fac = ixheaacd_mps_stride_table[freq_res_stride as usize];
    data_bands = ((band_stop as core::ffi::c_int - band_start as core::ffi::c_int
        - 1 as core::ffi::c_int) / ch_fac as core::ffi::c_int + 1 as core::ffi::c_int)
        as WORD32;
    *strides.offset(0 as core::ffi::c_int as isize) = band_start;
    pb = 1 as core::ffi::c_int as WORD32;
    while pb <= data_bands {
        *strides.offset(pb as isize) = *strides
            .offset((pb as core::ffi::c_int - 1 as core::ffi::c_int) as isize) + ch_fac;
        pb += 1;
    }
    start_offset = 0 as core::ffi::c_int as WORD32;
    while *strides.offset(data_bands as isize) > band_stop {
        if start_offset < data_bands {
            start_offset += 1;
        } else {
            start_offset = 1 as core::ffi::c_int as WORD32;
        }
        i = start_offset;
        while i <= data_bands {
            let ref mut fresh1 = *strides.offset(i as isize);
            *fresh1 -= 1;
            i += 1;
        }
    }
    return data_bands;
}
unsafe extern "C" fn ixheaacd_mps_ecdata_decoding(
    mut self_0: *mut ia_mps_dec_state_struct,
    mut bitstream: *mut ia_bit_buf_struct,
    mut data: *mut [WORD32; 28],
    mut datatype: WORD32,
) -> IA_ERRORCODE {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut pb: WORD32 = 0;
    let mut set_index: WORD32 = 0;
    let mut bs_data_pair: WORD32 = 0;
    let mut data_bands: WORD32 = 0;
    let mut old_quant_coarse_xxx: WORD32 = 0;
    let mut strides: [WORD32; 29] = [0 as core::ffi::c_int; 29];
    let mut band_stop: WORD32 = 0 as WORD32;
    let mut lastdata: *mut WORD32 = 0 as *mut WORD32;
    let mut frame_xxx_data: *mut ia_mps_data_struct = 0 as *mut ia_mps_data_struct;
    let mut default_val: WORD32 = 0 as WORD32;
    let mut err: IA_ERRORCODE = IA_NO_ERROR;
    let mut frame: *mut ia_mps_bs_frame = &mut (*self_0).bs_frame;
    if datatype == 0 as core::ffi::c_int {
        frame_xxx_data = &mut (*frame).cld_data;
        lastdata = ((*frame).cmp_cld_idx_prev).as_mut_ptr();
        band_stop = (*self_0).bs_param_bands;
    } else if datatype == 1 as core::ffi::c_int {
        frame_xxx_data = &mut (*frame).icc_data;
        lastdata = ((*frame).cmp_icc_idx_prev).as_mut_ptr();
        band_stop = (*self_0).bs_param_bands;
    } else if datatype == 2 as core::ffi::c_int {
        frame_xxx_data = &mut (*frame).ipd_data;
        lastdata = ((*frame).ipd_idx_data_prev).as_mut_ptr();
        band_stop = (*self_0).num_bands_ipd;
    } else {
        frame_xxx_data = &mut (*frame).cld_data;
        lastdata = ((*frame).cmp_cld_idx_prev).as_mut_ptr();
        band_stop = (*self_0).bs_param_bands;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*self_0).num_parameter_sets {
        (*frame_xxx_data).bs_xxx_data_mode[i as usize] = ixheaacd_read_bits_buf(
            bitstream,
            2 as WORD,
        );
        i += 1;
    }
    set_index = 0 as core::ffi::c_int as WORD32;
    bs_data_pair = 0 as core::ffi::c_int as WORD32;
    old_quant_coarse_xxx = (*frame_xxx_data).bs_quant_coarse_xxx_prev as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*self_0).num_parameter_sets {
        if (*frame_xxx_data).bs_xxx_data_mode[i as usize] == 0 as core::ffi::c_int {
            pb = 0 as core::ffi::c_int as WORD32;
            while pb < band_stop {
                *lastdata.offset(pb as isize) = default_val;
                pb += 1;
            }
            old_quant_coarse_xxx = 0 as core::ffi::c_int as WORD32;
        }
        if (*frame_xxx_data).bs_xxx_data_mode[i as usize] == 3 as core::ffi::c_int {
            if bs_data_pair != 0 {
                bs_data_pair = 0 as core::ffi::c_int as WORD32;
            } else {
                bs_data_pair = ixheaacd_read_bits_buf(bitstream, 1 as WORD);
                (*frame_xxx_data).bs_quant_coarse_xxx[set_index as usize] = ixheaacd_read_bits_buf(
                    bitstream,
                    1 as WORD,
                ) as WORD8;
                (*frame_xxx_data).bs_freq_res_stride_xxx[set_index as usize] = ixheaacd_read_bits_buf(
                    bitstream,
                    2 as WORD,
                );
                if (*frame_xxx_data).bs_quant_coarse_xxx[set_index as usize]
                    as core::ffi::c_int != old_quant_coarse_xxx
                {
                    if old_quant_coarse_xxx != 0 {
                        ixheaacd_mps_coarse2fine(
                            lastdata,
                            datatype,
                            0 as WORD32,
                            band_stop - 0 as WORD32,
                        );
                    } else {
                        ixheaacd_mps_fine2coarse(lastdata, band_stop);
                    }
                }
                data_bands = ixheaacd_mps_getstridemap(
                    (*frame_xxx_data).bs_freq_res_stride_xxx[set_index as usize],
                    0 as WORD32,
                    band_stop,
                    strides.as_mut_ptr(),
                );
                pb = 0 as core::ffi::c_int as WORD32;
                while pb < data_bands {
                    *lastdata.offset(pb as isize) = *lastdata
                        .offset(strides[pb as usize] as isize);
                    pb += 1;
                }
                err = ixheaacd_mps_ecdatapairdec(
                    bitstream,
                    data as *mut [WORD32; 28],
                    lastdata as *mut WORD32,
                    datatype,
                    set_index,
                    0 as WORD32,
                    data_bands,
                    bs_data_pair,
                    (*frame_xxx_data).bs_quant_coarse_xxx[set_index as usize] as WORD32,
                    (!((*frame).independency_flag as core::ffi::c_int != 0
                        && i == 0 as core::ffi::c_int)
                        || set_index > 0 as core::ffi::c_int) as core::ffi::c_int,
                    0 as WORD32,
                    0 as WORD32,
                    (*self_0).ec_flag as WORD32,
                ) as IA_ERRORCODE;
                if err != 0 {
                    return err;
                }
                pb = 0 as core::ffi::c_int as WORD32;
                while pb < data_bands {
                    j = strides[pb as usize];
                    while j
                        < strides[(pb as core::ffi::c_int + 1 as core::ffi::c_int)
                            as usize]
                    {
                        if datatype == IPD {
                            if (*frame_xxx_data).bs_quant_coarse_xxx[set_index as usize]
                                != 0
                            {
                                *lastdata.offset(j as isize) = ((*data
                                    .offset((set_index + bs_data_pair) as isize))[pb as usize]
                                    & 7 as core::ffi::c_int) as WORD32;
                            } else {
                                *lastdata.offset(j as isize) = ((*data
                                    .offset((set_index + bs_data_pair) as isize))[pb as usize]
                                    & 15 as core::ffi::c_int) as WORD32;
                            }
                        } else {
                            *lastdata.offset(j as isize) = (*data
                                .offset((set_index + bs_data_pair) as isize))[pb as usize];
                        }
                        j += 1;
                    }
                    pb += 1;
                }
                old_quant_coarse_xxx = (*frame_xxx_data)
                    .bs_quant_coarse_xxx[set_index as usize] as WORD32;
                if bs_data_pair != 0 {
                    (*frame_xxx_data)
                        .bs_quant_coarse_xxx[(set_index as core::ffi::c_int
                        + 1 as core::ffi::c_int) as usize] = (*frame_xxx_data)
                        .bs_quant_coarse_xxx[set_index as usize];
                    (*frame_xxx_data)
                        .bs_freq_res_stride_xxx[(set_index as core::ffi::c_int
                        + 1 as core::ffi::c_int) as usize] = (*frame_xxx_data)
                        .bs_freq_res_stride_xxx[set_index as usize];
                }
                set_index += bs_data_pair as core::ffi::c_int + 1 as core::ffi::c_int;
            }
        }
        i += 1;
    }
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_frame_parsing(
    mut self_0: *mut ia_mps_dec_state_struct,
    mut usac_independency_flag: WORD32,
    mut bitstream: *mut ia_bit_buf_struct,
) -> IA_ERRORCODE {
    let mut i: WORD32 = 0;
    let mut bs_frame_type: WORD32 = 0;
    let mut data_bands: WORD32 = 0;
    let mut bs_temp_shape_enable: WORD32 = 0;
    let mut num_of_temp_shape_ch: WORD32 = 0;
    let mut ps: WORD32 = 0;
    let mut pg: WORD32 = 0;
    let mut ts: WORD32 = 0;
    let mut pb: WORD32 = 0;
    let mut env_shape_data: [WORD32; 72] = [0; 72];
    let mut bits_param_slot: WORD32 = 0 as WORD32;
    let mut frame: *mut ia_mps_bs_frame = &mut (*self_0).bs_frame;
    let mut err: IA_ERRORCODE = IA_NO_ERROR;
    if (*self_0).parse_nxt_frame as core::ffi::c_int == 0 as core::ffi::c_int {
        return IA_NO_ERROR;
    }
    (*self_0).num_parameter_sets_prev = (*self_0).num_parameter_sets;
    if (*self_0).bs_high_rate_mode != 0 {
        bs_frame_type = ixheaacd_read_bits_buf(bitstream, 1 as WORD);
        (*self_0).num_parameter_sets = (ixheaacd_read_bits_buf(bitstream, 3 as WORD)
            as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
    } else {
        bs_frame_type = 0 as core::ffi::c_int as WORD32;
        (*self_0).num_parameter_sets = 1 as core::ffi::c_int as WORD32;
    }
    if (*self_0).time_slots == 32 as core::ffi::c_int {
        bits_param_slot = 5 as core::ffi::c_int as WORD32;
    } else if (*self_0).time_slots == 64 as core::ffi::c_int {
        bits_param_slot = 6 as core::ffi::c_int as WORD32;
    }
    if bs_frame_type != 0 {
        let mut prev_param_slot: WORD32 = -(1 as WORD32);
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*self_0).num_parameter_sets {
            (*self_0).param_slots[i as usize] = ixheaacd_read_bits_buf(
                bitstream,
                bits_param_slot as WORD,
            );
            if prev_param_slot >= (*self_0).param_slots[i as usize]
                || (*self_0).param_slots[i as usize] >= (*self_0).time_slots
            {
                return IA_FATAL_ERROR as IA_ERRORCODE;
            }
            prev_param_slot = (*self_0).param_slots[i as usize];
            i += 1;
        }
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*self_0).num_parameter_sets {
            (*self_0).param_slots[i as usize] = (((*self_0).time_slots
                as core::ffi::c_int * (i as core::ffi::c_int + 1 as core::ffi::c_int)
                + (*self_0).num_parameter_sets as core::ffi::c_int
                - 1 as core::ffi::c_int)
                / (*self_0).num_parameter_sets as core::ffi::c_int
                - 1 as core::ffi::c_int) as WORD32;
            i += 1;
        }
    }
    if usac_independency_flag == 0 {
        (*frame).independency_flag = ixheaacd_read_bits_buf(bitstream, 1 as WORD)
            as WORD8;
    } else {
        (*frame).independency_flag = 1 as WORD8;
    }
    err = ixheaacd_mps_ecdata_decoding(
        self_0,
        bitstream,
        ((*frame).cmp_cld_idx).as_mut_ptr(),
        CLD,
    );
    if err != 0 {
        return err;
    }
    err = ixheaacd_mps_ecdata_decoding(
        self_0,
        bitstream,
        ((*frame).cmp_icc_idx).as_mut_ptr(),
        ICC,
    );
    if err != 0 {
        return err;
    }
    if (*(*self_0).config).bs_phase_coding != 0 {
        (*self_0).bs_phase_mode = ixheaacd_read_bits_buf(bitstream, 1 as WORD);
        if (*self_0).bs_phase_mode == 0 {
            pb = 0 as core::ffi::c_int as WORD32;
            while pb < (*self_0).num_bands_ipd {
                (*frame).ipd_idx_data_prev[pb as usize] = 0 as core::ffi::c_int
                    as WORD32;
                i = 0 as core::ffi::c_int as WORD32;
                while i < (*self_0).num_parameter_sets {
                    (*frame).ipd_idx_data[i as usize][pb as usize] = 0
                        as core::ffi::c_int as WORD32;
                    (*self_0).bs_frame.ipd_idx[i as usize][pb as usize] = 0
                        as core::ffi::c_int as WORD32;
                    i += 1;
                }
                (*self_0).bs_frame.ipd_idx_prev[pb as usize] = 0 as core::ffi::c_int
                    as WORD32;
                pb += 1;
            }
            (*self_0).opd_smoothing_mode = 0 as core::ffi::c_int as WORD32;
        } else {
            (*self_0).opd_smoothing_mode = ixheaacd_read_bits_buf(bitstream, 1 as WORD);
            err = ixheaacd_mps_ecdata_decoding(
                self_0,
                bitstream,
                ((*frame).ipd_idx_data).as_mut_ptr(),
                IPD,
            );
            if err != 0 {
                return err;
            }
        }
    } else {
        (*self_0).bs_phase_mode = 0 as core::ffi::c_int as WORD32;
        pb = 0 as core::ffi::c_int as WORD32;
        while pb < (*self_0).num_bands_ipd {
            (*frame).ipd_idx_data_prev[pb as usize] = 0 as core::ffi::c_int as WORD32;
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*self_0).num_parameter_sets {
                (*frame).ipd_idx_data[i as usize][pb as usize] = 0 as core::ffi::c_int
                    as WORD32;
                (*self_0).bs_frame.ipd_idx[i as usize][pb as usize] = 0
                    as core::ffi::c_int as WORD32;
                i += 1;
            }
            (*self_0).bs_frame.ipd_idx_prev[pb as usize] = 0 as core::ffi::c_int
                as WORD32;
            pb += 1;
        }
        (*self_0).opd_smoothing_mode = 0 as core::ffi::c_int as WORD32;
    }
    if (*self_0).bs_high_rate_mode != 0 {
        ps = 0 as core::ffi::c_int as WORD32;
        while ps < (*self_0).num_parameter_sets {
            (*frame).bs_smooth_mode[ps as usize] = ixheaacd_read_bits_buf(
                bitstream,
                2 as WORD,
            );
            if (*frame).bs_smooth_mode[ps as usize] >= 2 as core::ffi::c_int {
                (*frame).bs_smooth_time[ps as usize] = ixheaacd_read_bits_buf(
                    bitstream,
                    2 as WORD,
                );
            }
            if (*frame).bs_smooth_mode[ps as usize] == 3 as core::ffi::c_int {
                (*frame).bs_freq_res_stride_smg[ps as usize] = ixheaacd_read_bits_buf(
                    bitstream,
                    2 as WORD,
                );
                data_bands = (((*self_0).bs_param_bands as core::ffi::c_int
                    - 1 as core::ffi::c_int)
                    / ixheaacd_mps_stride_table[(*frame)
                        .bs_freq_res_stride_smg[ps as usize] as usize]
                    + 1 as core::ffi::c_int) as WORD32;
                pg = 0 as core::ffi::c_int as WORD32;
                while pg < data_bands {
                    (*frame).bs_smg_data[ps as usize][pg as usize] = ixheaacd_read_bits_buf(
                        bitstream,
                        1 as WORD,
                    );
                    pg += 1;
                }
            }
            ps += 1;
        }
    } else {
        ps = 0 as core::ffi::c_int as WORD32;
        while ps < (*self_0).num_parameter_sets {
            (*frame).bs_smooth_mode[ps as usize] = 0 as core::ffi::c_int as WORD32;
            ps += 1;
        }
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < 2 as core::ffi::c_int {
        (*self_0).temp_shape_enable_ch_stp[i as usize] = 0 as core::ffi::c_int as WORD32;
        (*self_0).temp_shape_enable_ch_ges[i as usize] = 0 as core::ffi::c_int as WORD32;
        i += 1;
    }
    (*self_0).bs_tsd_enable = 0 as core::ffi::c_int as WORD32;
    if (*(*self_0).config).bs_temp_shape_config == 3 as UINT32 {
        (*self_0).bs_tsd_enable = ixheaacd_read_bits_buf(bitstream, 1 as WORD);
    } else if (*(*self_0).config).bs_temp_shape_config != 0 as UINT32 {
        bs_temp_shape_enable = ixheaacd_read_bits_buf(bitstream, 1 as WORD);
        if bs_temp_shape_enable != 0 {
            num_of_temp_shape_ch = 2 as core::ffi::c_int as WORD32;
            match (*(*self_0).config).bs_temp_shape_config {
                1 => {
                    i = 0 as core::ffi::c_int as WORD32;
                    while i < num_of_temp_shape_ch {
                        (*self_0).temp_shape_enable_ch_stp[i as usize] = ixheaacd_read_bits_buf(
                            bitstream,
                            1 as WORD,
                        );
                        i += 1;
                    }
                }
                2 => {
                    i = 0 as core::ffi::c_int as WORD32;
                    while i < num_of_temp_shape_ch {
                        (*self_0).temp_shape_enable_ch_ges[i as usize] = ixheaacd_read_bits_buf(
                            bitstream,
                            1 as WORD,
                        );
                        i += 1;
                    }
                    i = 0 as core::ffi::c_int as WORD32;
                    while i < num_of_temp_shape_ch {
                        if (*self_0).temp_shape_enable_ch_ges[i as usize] != 0 {
                            ixheaacd_mps_huff_decode(
                                bitstream,
                                env_shape_data.as_mut_ptr(),
                                (*self_0).time_slots,
                            );
                            ts = 0 as core::ffi::c_int as WORD32;
                            while ts < (*self_0).time_slots {
                                (*self_0).env_shape_data[i as usize][ts as usize] = pow(
                                    2 as core::ffi::c_int as core::ffi::c_double,
                                    (env_shape_data[ts as usize] as FLOAT32
                                        / ((*(*self_0).config).bs_env_quant_mode)
                                            .wrapping_add(2 as UINT32) as FLOAT32
                                        - 1 as core::ffi::c_int as FLOAT32) as core::ffi::c_double,
                                ) as FLOAT32;
                                ts += 1;
                            }
                        }
                        i += 1;
                    }
                }
                _ => return -(1 as IA_ERRORCODE),
            }
        }
    }
    if (*self_0).bs_tsd_enable != 0 {
        let mut s: [UWORD16; 4] = [0; 4];
        let mut s_64: UWORD64 = 0;
        let mut c: [UWORD16; 5] = [0; 5];
        let mut c_64: UWORD64 = 0;
        let mut b: UWORD16 = 0;
        let mut r: [UWORD16; 1] = [0; 1];
        static mut table_64: [UWORD16; 32] = [
            6 as core::ffi::c_int as UWORD16,
            11 as core::ffi::c_int as UWORD16,
            16 as core::ffi::c_int as UWORD16,
            20 as core::ffi::c_int as UWORD16,
            23 as core::ffi::c_int as UWORD16,
            27 as core::ffi::c_int as UWORD16,
            30 as core::ffi::c_int as UWORD16,
            33 as core::ffi::c_int as UWORD16,
            35 as core::ffi::c_int as UWORD16,
            38 as core::ffi::c_int as UWORD16,
            40 as core::ffi::c_int as UWORD16,
            42 as core::ffi::c_int as UWORD16,
            44 as core::ffi::c_int as UWORD16,
            46 as core::ffi::c_int as UWORD16,
            48 as core::ffi::c_int as UWORD16,
            49 as core::ffi::c_int as UWORD16,
            51 as core::ffi::c_int as UWORD16,
            52 as core::ffi::c_int as UWORD16,
            53 as core::ffi::c_int as UWORD16,
            55 as core::ffi::c_int as UWORD16,
            56 as core::ffi::c_int as UWORD16,
            57 as core::ffi::c_int as UWORD16,
            58 as core::ffi::c_int as UWORD16,
            58 as core::ffi::c_int as UWORD16,
            59 as core::ffi::c_int as UWORD16,
            60 as core::ffi::c_int as UWORD16,
            60 as core::ffi::c_int as UWORD16,
            60 as core::ffi::c_int as UWORD16,
            61 as core::ffi::c_int as UWORD16,
            61 as core::ffi::c_int as UWORD16,
            61 as core::ffi::c_int as UWORD16,
            61 as core::ffi::c_int as UWORD16,
        ];
        static mut table_32: [UWORD16; 16] = [
            5 as core::ffi::c_int as UWORD16,
            9 as core::ffi::c_int as UWORD16,
            13 as core::ffi::c_int as UWORD16,
            16 as core::ffi::c_int as UWORD16,
            18 as core::ffi::c_int as UWORD16,
            20 as core::ffi::c_int as UWORD16,
            22 as core::ffi::c_int as UWORD16,
            24 as core::ffi::c_int as UWORD16,
            25 as core::ffi::c_int as UWORD16,
            26 as core::ffi::c_int as UWORD16,
            27 as core::ffi::c_int as UWORD16,
            28 as core::ffi::c_int as UWORD16,
            29 as core::ffi::c_int as UWORD16,
            29 as core::ffi::c_int as UWORD16,
            30 as core::ffi::c_int as UWORD16,
            30 as core::ffi::c_int as UWORD16,
        ];
        let mut tab: *const core::ffi::c_ushort = 0 as *const core::ffi::c_ushort;
        let mut k: WORD32 = 0;
        let mut h: UWORD16 = 0;
        let mut nbits_tr_slots: WORD32 = 0 as WORD32;
        if (*self_0).time_slots == 32 as core::ffi::c_int {
            nbits_tr_slots = 4 as core::ffi::c_int as WORD32;
            tab = table_32.as_ptr() as *const core::ffi::c_ushort;
        } else if (*self_0).time_slots == 64 as core::ffi::c_int {
            nbits_tr_slots = 5 as core::ffi::c_int as WORD32;
            tab = table_64.as_ptr() as *const core::ffi::c_ushort;
        }
        (*self_0).tsd_num_tr_slots = ixheaacd_read_bits_buf(
            bitstream,
            nbits_tr_slots as WORD,
        );
        (*self_0).tsd_num_tr_slots += 1;
        (*self_0).tsd_codeword_len = *tab
            .offset(
                ((*self_0).tsd_num_tr_slots as core::ffi::c_int - 1 as core::ffi::c_int)
                    as isize,
            ) as WORD32;
        if (*self_0).tsd_codeword_len > 48 as core::ffi::c_int {
            s[3 as core::ffi::c_int as usize] = ixheaacd_read_bits_buf(
                bitstream,
                (*self_0).tsd_codeword_len as WORD - 48 as WORD,
            ) as UWORD16;
            s_64 = s[3 as core::ffi::c_int as usize] as UWORD64;
            s[2 as core::ffi::c_int as usize] = ixheaacd_read_bits_buf(
                bitstream,
                16 as WORD,
            ) as UWORD16;
            s_64 = s_64 << 16 as core::ffi::c_int
                | s[2 as core::ffi::c_int as usize] as UWORD64;
            s[1 as core::ffi::c_int as usize] = ixheaacd_read_bits_buf(
                bitstream,
                16 as WORD,
            ) as UWORD16;
            s_64 = s_64 << 16 as core::ffi::c_int
                | s[1 as core::ffi::c_int as usize] as UWORD64;
            s[0 as core::ffi::c_int as usize] = ixheaacd_read_bits_buf(
                bitstream,
                16 as WORD,
            ) as UWORD16;
            s_64 = s_64 << 16 as core::ffi::c_int
                | s[0 as core::ffi::c_int as usize] as UWORD64;
        } else if (*self_0).tsd_codeword_len > 32 as core::ffi::c_int {
            s[3 as core::ffi::c_int as usize] = 0 as UWORD16;
            s_64 = s[3 as core::ffi::c_int as usize] as UWORD64;
            s[2 as core::ffi::c_int as usize] = ixheaacd_read_bits_buf(
                bitstream,
                (*self_0).tsd_codeword_len as WORD - 32 as WORD,
            ) as UWORD16;
            s_64 = s_64 << 16 as core::ffi::c_int
                | s[2 as core::ffi::c_int as usize] as UWORD64;
            s[1 as core::ffi::c_int as usize] = ixheaacd_read_bits_buf(
                bitstream,
                16 as WORD,
            ) as UWORD16;
            s_64 = s_64 << 16 as core::ffi::c_int
                | s[1 as core::ffi::c_int as usize] as UWORD64;
            s[0 as core::ffi::c_int as usize] = ixheaacd_read_bits_buf(
                bitstream,
                16 as WORD,
            ) as UWORD16;
            s_64 = s_64 << 16 as core::ffi::c_int
                | s[0 as core::ffi::c_int as usize] as UWORD64;
        } else if (*self_0).tsd_codeword_len > 16 as core::ffi::c_int {
            s[3 as core::ffi::c_int as usize] = 0 as UWORD16;
            s_64 = s[3 as core::ffi::c_int as usize] as UWORD64;
            s[2 as core::ffi::c_int as usize] = 0 as UWORD16;
            s_64 = s_64 << 16 as core::ffi::c_int
                | s[2 as core::ffi::c_int as usize] as UWORD64;
            s[1 as core::ffi::c_int as usize] = ixheaacd_read_bits_buf(
                bitstream,
                (*self_0).tsd_codeword_len as WORD - 16 as WORD,
            ) as UWORD16;
            s_64 = s_64 << 16 as core::ffi::c_int
                | s[1 as core::ffi::c_int as usize] as UWORD64;
            s[0 as core::ffi::c_int as usize] = ixheaacd_read_bits_buf(
                bitstream,
                16 as WORD,
            ) as UWORD16;
            s_64 = s_64 << 16 as core::ffi::c_int
                | s[0 as core::ffi::c_int as usize] as UWORD64;
        } else {
            s[3 as core::ffi::c_int as usize] = 0 as UWORD16;
            s_64 = s[3 as core::ffi::c_int as usize] as UWORD64;
            s[2 as core::ffi::c_int as usize] = 0 as UWORD16;
            s_64 = s_64 << 16 as core::ffi::c_int
                | s[2 as core::ffi::c_int as usize] as UWORD64;
            s[1 as core::ffi::c_int as usize] = 0 as UWORD16;
            s_64 = s_64 << 16 as core::ffi::c_int
                | s[1 as core::ffi::c_int as usize] as UWORD64;
            s[0 as core::ffi::c_int as usize] = ixheaacd_read_bits_buf(
                bitstream,
                (*self_0).tsd_codeword_len as WORD,
            ) as UWORD16;
            s_64 = s_64 << 16 as core::ffi::c_int
                | s[0 as core::ffi::c_int as usize] as UWORD64;
        }
        let mut p: WORD32 = (*self_0).tsd_num_tr_slots;
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*self_0).time_slots {
            (*self_0).bs_tsd_sep_data[i as usize] = 0 as core::ffi::c_int as WORD32;
            i += 1;
        }
        k = ((*self_0).time_slots as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        while k >= 0 as core::ffi::c_int {
            if p > k {
                while k >= 0 as core::ffi::c_int {
                    (*self_0).bs_tsd_sep_data[k as usize] = 1 as core::ffi::c_int
                        as WORD32;
                    k -= 1;
                }
                break;
            } else {
                c[0 as core::ffi::c_int as usize] = (k as core::ffi::c_int
                    - p as core::ffi::c_int + 1 as core::ffi::c_int) as UWORD16;
                c_64 = c[0 as core::ffi::c_int as usize] as UWORD64;
                i = 1 as core::ffi::c_int as WORD32;
                while i < 5 as core::ffi::c_int {
                    c[i as usize] = 0 as UWORD16;
                    i += 1;
                }
                h = 2 as UWORD16;
                while h as core::ffi::c_int <= p {
                    b = (k as core::ffi::c_int - p as core::ffi::c_int
                        + h as core::ffi::c_int) as UWORD16;
                    c_64 = c_64
                        .wrapping_mul(
                            (b as core::ffi::c_int / h as core::ffi::c_int) as UWORD64,
                        );
                    ixheaacd_longmult1(c.as_mut_ptr(), b, c.as_mut_ptr(), 5 as WORD32);
                    b = h;
                    ixheaacd_longdiv(
                        c.as_mut_ptr(),
                        b,
                        c.as_mut_ptr(),
                        r.as_mut_ptr(),
                        5 as WORD32,
                    );
                    h = h.wrapping_add(1);
                }
                if ixheaacd_longcompare(s.as_mut_ptr(), c.as_mut_ptr(), 4 as WORD32) != 0
                {
                    ixheaacd_longsub(
                        s.as_mut_ptr(),
                        c.as_mut_ptr(),
                        4 as WORD32,
                        4 as WORD32,
                    );
                    (*self_0).bs_tsd_sep_data[k as usize] = 1 as core::ffi::c_int
                        as WORD32;
                    p -= 1;
                    if p == 0 as core::ffi::c_int {
                        break;
                    }
                }
                k -= 1;
            }
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*self_0).time_slots {
            if (*self_0).bs_tsd_sep_data[i as usize] != 0 {
                (*self_0).bs_tsd_tr_phase_data[i as usize] = ixheaacd_read_bits_buf(
                    bitstream,
                    3 as WORD,
                );
            }
            i += 1;
        }
    }
    (*self_0).parse_nxt_frame = 0 as WORD8;
    return err;
}
unsafe extern "C" fn ixheaacd_ld_mps_ecdata_decoding(
    mut self_0: *mut ia_mps_dec_state_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut data: *mut [WORD32; 28],
    mut datatype: WORD32,
    mut start_band: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut pb: WORD32 = 0;
    let mut set_index: WORD32 = 0;
    let mut bs_data_pair: WORD32 = 0;
    let mut data_bands: WORD32 = 0;
    let mut old_quant_coarse_xxx: WORD32 = 0;
    let mut strides: [WORD32; 29] = [0 as core::ffi::c_int; 29];
    let mut band_stop: WORD32 = 0 as WORD32;
    let mut lastdata: *mut WORD32 = 0 as *mut WORD32;
    let mut frame_xxx_data: *mut ia_mps_data_struct = 0 as *mut ia_mps_data_struct;
    let mut default_val: WORD32 = 0 as WORD32;
    let mut frame: *mut ia_mps_bs_frame = &mut (*self_0).bs_frame;
    if datatype == CLD {
        frame_xxx_data = &mut (*frame).cld_data;
        lastdata = ((*frame).cmp_cld_idx_prev).as_mut_ptr();
        band_stop = (*self_0).bs_param_bands;
    } else if datatype == ICC {
        frame_xxx_data = &mut (*frame).icc_data;
        lastdata = ((*frame).cmp_icc_idx_prev).as_mut_ptr();
        band_stop = (*self_0).bs_param_bands;
    } else if datatype == IPD {
        frame_xxx_data = &mut (*frame).ipd_data;
        lastdata = ((*frame).ipd_idx_data_prev).as_mut_ptr();
        band_stop = (*self_0).num_bands_ipd;
    } else {
        frame_xxx_data = &mut (*frame).cld_data;
        lastdata = ((*frame).cmp_cld_idx_prev).as_mut_ptr();
        band_stop = (*self_0).bs_param_bands;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*self_0).num_parameter_sets {
        (*frame_xxx_data).bs_xxx_data_mode[i as usize] = ixheaacd_read_bits_buf(
            it_bit_buff,
            2 as WORD,
        );
        i += 1;
    }
    set_index = 0 as core::ffi::c_int as WORD32;
    bs_data_pair = 0 as core::ffi::c_int as WORD32;
    old_quant_coarse_xxx = (*frame_xxx_data).bs_quant_coarse_xxx_prev as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*self_0).num_parameter_sets {
        if (*frame_xxx_data).bs_xxx_data_mode[i as usize] == 0 as core::ffi::c_int {
            pb = 0 as core::ffi::c_int as WORD32;
            while pb < band_stop {
                *lastdata.offset(pb as isize) = default_val;
                pb += 1;
            }
            old_quant_coarse_xxx = 0 as core::ffi::c_int as WORD32;
        }
        if (*frame_xxx_data).bs_xxx_data_mode[i as usize] == 3 as core::ffi::c_int {
            if bs_data_pair != 0 {
                bs_data_pair = 0 as core::ffi::c_int as WORD32;
            } else {
                bs_data_pair = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
                (*frame_xxx_data).bs_quant_coarse_xxx[set_index as usize] = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    1 as WORD,
                ) as WORD8;
                (*frame_xxx_data).bs_freq_res_stride_xxx[set_index as usize] = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    2 as WORD,
                );
                if (*frame_xxx_data).bs_quant_coarse_xxx[set_index as usize]
                    as core::ffi::c_int != old_quant_coarse_xxx
                {
                    if old_quant_coarse_xxx != 0 {
                        ixheaacd_mps_coarse2fine(
                            lastdata,
                            datatype,
                            0 as WORD32,
                            band_stop - 0 as WORD32,
                        );
                    } else {
                        ixheaacd_mps_fine2coarse(lastdata, band_stop);
                    }
                }
                data_bands = ixheaacd_mps_getstridemap(
                    (*frame_xxx_data).bs_freq_res_stride_xxx[set_index as usize],
                    start_band,
                    band_stop,
                    strides.as_mut_ptr(),
                );
                pb = 0 as core::ffi::c_int as WORD32;
                while pb < data_bands {
                    *lastdata.offset(pb as isize) = *lastdata
                        .offset(strides[pb as usize] as isize);
                    pb += 1;
                }
                ixheaacd_mps_ecdatapairdec(
                    it_bit_buff,
                    data as *mut [WORD32; 28],
                    lastdata as *mut WORD32,
                    datatype,
                    set_index,
                    0 as WORD32,
                    data_bands,
                    bs_data_pair,
                    (*frame_xxx_data).bs_quant_coarse_xxx[set_index as usize] as WORD32,
                    (!((*frame).independency_flag as core::ffi::c_int != 0
                        && i == 0 as core::ffi::c_int)
                        || set_index > 0 as core::ffi::c_int) as core::ffi::c_int,
                    1 as WORD32,
                    0 as WORD32,
                    (*self_0).ec_flag as WORD32,
                );
                pb = 0 as core::ffi::c_int as WORD32;
                while pb < data_bands {
                    j = strides[pb as usize];
                    while j
                        < strides[(pb as core::ffi::c_int + 1 as core::ffi::c_int)
                            as usize]
                    {
                        if datatype == IPD {
                            if (*frame_xxx_data).bs_quant_coarse_xxx[set_index as usize]
                                != 0
                            {
                                *lastdata.offset(j as isize) = ((*data
                                    .offset((set_index + bs_data_pair) as isize))[pb as usize]
                                    & 7 as core::ffi::c_int) as WORD32;
                            } else {
                                *lastdata.offset(j as isize) = ((*data
                                    .offset((set_index + bs_data_pair) as isize))[pb as usize]
                                    & 15 as core::ffi::c_int) as WORD32;
                            }
                        } else {
                            *lastdata.offset(j as isize) = (*data
                                .offset((set_index + bs_data_pair) as isize))[pb as usize];
                        }
                        j += 1;
                    }
                    pb += 1;
                }
                old_quant_coarse_xxx = (*frame_xxx_data)
                    .bs_quant_coarse_xxx[set_index as usize] as WORD32;
                if bs_data_pair != 0 {
                    (*frame_xxx_data)
                        .bs_quant_coarse_xxx[(set_index as core::ffi::c_int
                        + 1 as core::ffi::c_int) as usize] = (*frame_xxx_data)
                        .bs_quant_coarse_xxx[set_index as usize];
                    (*frame_xxx_data)
                        .bs_freq_res_stride_xxx[(set_index as core::ffi::c_int
                        + 1 as core::ffi::c_int) as usize] = (*frame_xxx_data)
                        .bs_freq_res_stride_xxx[set_index as usize];
                }
                set_index += bs_data_pair as core::ffi::c_int + 1 as core::ffi::c_int;
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_ld_mps_frame_parsing(
    mut self_0: *mut ia_mps_dec_state_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
) -> IA_ERRORCODE {
    let mut i: WORD32 = 0;
    let mut bs_frame_type: WORD32 = 0;
    let mut data_bands: WORD32 = 0;
    let mut bs_temp_shape_enable: WORD32 = 0;
    let mut num_of_temp_shape_ch: WORD32 = 0;
    let mut ps: WORD32 = 0;
    let mut pg: WORD32 = 0;
    let mut ts: WORD32 = 0;
    let mut ic: WORD32 = 0;
    let mut env_shape_data: [WORD32; 72] = [0; 72];
    let mut alignment: WORD32 = 0;
    let mut bits_param_slot: WORD32 = 0 as WORD32;
    let mut frame: *mut ia_mps_bs_frame = &mut (*self_0).bs_frame;
    alignment = (*it_bit_buff).cnt_bits;
    if (*self_0).parse_nxt_frame as core::ffi::c_int == 0 as core::ffi::c_int {
        return IA_NO_ERROR;
    }
    (*self_0).num_parameter_sets_prev = (*self_0).num_parameter_sets;
    bs_frame_type = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
    (*self_0).num_parameter_sets = (ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD)
        as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
    if (*self_0).time_slots == 32 as core::ffi::c_int {
        bits_param_slot = 5 as core::ffi::c_int as WORD32;
    } else if (*self_0).time_slots == 64 as core::ffi::c_int {
        bits_param_slot = 6 as core::ffi::c_int as WORD32;
    } else if (*self_0).time_slots == 8 as core::ffi::c_int {
        bits_param_slot = 3 as core::ffi::c_int as WORD32;
    } else if (*self_0).time_slots == 16 as core::ffi::c_int
        || (*self_0).time_slots == 15 as core::ffi::c_int
    {
        bits_param_slot = 4 as core::ffi::c_int as WORD32;
    }
    if bs_frame_type != 0 {
        let mut prev_param_slot: WORD32 = -(1 as WORD32);
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*self_0).num_parameter_sets {
            (*self_0).param_slots[i as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                bits_param_slot as WORD,
            );
            if prev_param_slot >= (*self_0).param_slots[i as usize]
                || (*self_0).param_slots[i as usize] >= (*self_0).time_slots
            {
                return IA_FATAL_ERROR as IA_ERRORCODE;
            }
            prev_param_slot = (*self_0).param_slots[i as usize];
            i += 1;
        }
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*self_0).num_parameter_sets {
            (*self_0).param_slots[i as usize] = (((*self_0).time_slots
                as core::ffi::c_int * (i as core::ffi::c_int + 1 as core::ffi::c_int)
                + (*self_0).num_parameter_sets as core::ffi::c_int
                - 1 as core::ffi::c_int)
                / (*self_0).num_parameter_sets as core::ffi::c_int
                - 1 as core::ffi::c_int) as WORD32;
            i += 1;
        }
    }
    (*frame).independency_flag = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD) as WORD8;
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*self_0).ldmps_config.num_ott_boxes {
        ixheaacd_ld_mps_ecdata_decoding(
            self_0,
            it_bit_buff,
            ((*frame).cmp_cld_idx).as_mut_ptr(),
            CLD,
            0 as WORD32,
        );
        i += 1;
    }
    if (*self_0).ldmps_config.bs_one_icc != 0 {
        ixheaacd_ld_mps_ecdata_decoding(
            self_0,
            it_bit_buff,
            ((*frame).cmp_icc_idx).as_mut_ptr(),
            ICC,
            0 as WORD32,
        );
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*self_0).ldmps_config.num_ott_boxes {
            if (*self_0).ldmps_config.ott_mode_lfe[i as usize] == 0 {
                ixheaacd_ld_mps_ecdata_decoding(
                    self_0,
                    it_bit_buff,
                    ((*frame).cmp_icc_idx).as_mut_ptr(),
                    ICC,
                    0 as WORD32,
                );
            }
            i += 1;
        }
    }
    ps = 0 as core::ffi::c_int as WORD32;
    while ps < (*self_0).num_parameter_sets {
        (*frame).bs_smooth_mode[ps as usize] = ixheaacd_read_bits_buf(
            it_bit_buff,
            2 as WORD,
        );
        if (*frame).bs_smooth_mode[ps as usize] >= 2 as core::ffi::c_int {
            (*frame).bs_smooth_time[ps as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                2 as WORD,
            );
        }
        if (*frame).bs_smooth_mode[ps as usize] == 3 as core::ffi::c_int {
            (*frame).bs_freq_res_stride_smg[ps as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                2 as WORD,
            );
            data_bands = ((ixheaacd_freq_res_table_ld[(*self_0).ldmps_config.bs_freq_res
                as usize] - 1 as core::ffi::c_int)
                / ixheaacd_mps_stride_table[(*frame).bs_freq_res_stride_smg[ps as usize]
                    as usize] + 1 as core::ffi::c_int) as WORD32;
            pg = 0 as core::ffi::c_int as WORD32;
            while pg < data_bands {
                (*frame).bs_smg_data[ps as usize][pg as usize] = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    1 as WORD,
                );
                pg += 1;
            }
        }
        ps += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < 2 as core::ffi::c_int {
        (*self_0).temp_shape_enable_ch_stp[i as usize] = 0 as core::ffi::c_int as WORD32;
        (*self_0).temp_shape_enable_ch_ges[i as usize] = 0 as core::ffi::c_int as WORD32;
        i += 1;
    }
    if (*self_0).ldmps_config.bs_temp_shape_config != 0 as UINT32 {
        bs_temp_shape_enable = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
        if bs_temp_shape_enable != 0 {
            num_of_temp_shape_ch = 2 as core::ffi::c_int as WORD32;
            match (*self_0).ldmps_config.bs_temp_shape_config {
                1 => {
                    i = 0 as core::ffi::c_int as WORD32;
                    while i < num_of_temp_shape_ch {
                        (*self_0).temp_shape_enable_ch_stp[i as usize] = ixheaacd_read_bits_buf(
                            it_bit_buff,
                            1 as WORD,
                        );
                        i += 1;
                    }
                }
                2 => {
                    i = 0 as core::ffi::c_int as WORD32;
                    while i < num_of_temp_shape_ch {
                        (*self_0).temp_shape_enable_ch_ges[i as usize] = ixheaacd_read_bits_buf(
                            it_bit_buff,
                            1 as WORD,
                        );
                        i += 1;
                    }
                    i = 0 as core::ffi::c_int as WORD32;
                    while i < num_of_temp_shape_ch {
                        if (*self_0).temp_shape_enable_ch_ges[i as usize] != 0 {
                            ixheaacd_mps_huff_decode(
                                it_bit_buff,
                                env_shape_data.as_mut_ptr(),
                                (*self_0).time_slots,
                            );
                            ts = 0 as core::ffi::c_int as WORD32;
                            while ts < (*self_0).time_slots {
                                (*self_0).env_shape_data[i as usize][ts as usize] = pow(
                                    2 as core::ffi::c_int as core::ffi::c_double,
                                    (env_shape_data[ts as usize] as FLOAT32
                                        / ((*self_0).ldmps_config.bs_env_quant_mode)
                                            .wrapping_add(2 as UINT32) as FLOAT32
                                        - 1 as core::ffi::c_int as FLOAT32) as core::ffi::c_double,
                                ) as FLOAT32;
                                ts += 1;
                            }
                        }
                        i += 1;
                    }
                }
                _ => {
                    if (*self_0).ec_flag as core::ffi::c_int == 0 as core::ffi::c_int {
                        return IA_FATAL_ERROR as IA_ERRORCODE;
                    }
                }
            }
        }
    }
    if (*self_0).ldmps_config.bs_arbitrary_downmix != 0 as UINT32 {
        ic = 0 as core::ffi::c_int as WORD32;
        while ic < (*self_0).ldmps_config.num_input_channels {
            ixheaacd_ld_mps_ecdata_decoding(
                self_0,
                it_bit_buff,
                ((*frame).cmp_cld_idx).as_mut_ptr(),
                CLD,
                0 as WORD32,
            );
            ic += 1;
        }
    }
    ixheaacd_byte_align(it_bit_buff, &mut alignment);
    while (*it_bit_buff).cnt_bits > 8 as core::ffi::c_int {
        ixheaacd_read_bits_buf(it_bit_buff, 8 as WORD);
    }
    ixheaacd_read_bits_buf(it_bit_buff, (*it_bit_buff).cnt_bits as WORD);
    return IA_NO_ERROR;
}
unsafe extern "C" fn ixheaacd_mps_createmapping(
    mut map: *mut WORD32,
    mut band_start: WORD32,
    mut band_stop: WORD32,
    mut ch_fac: WORD32,
) -> VOID {
    let mut input_bands: WORD32 = 0;
    let mut out_bands: WORD32 = 0;
    let mut bands_achived: WORD32 = 0;
    let mut bands_diff: WORD32 = 0;
    let mut incr: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut vdk: [WORD32; 29] = [0; 29];
    input_bands = band_stop - band_start;
    out_bands = ((input_bands as core::ffi::c_int - 1 as core::ffi::c_int)
        / ch_fac as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
    if out_bands < 1 as core::ffi::c_int {
        out_bands = 1 as core::ffi::c_int as WORD32;
    }
    bands_achived = out_bands * ch_fac;
    bands_diff = input_bands - bands_achived;
    i = 0 as core::ffi::c_int as WORD32;
    while i < out_bands {
        vdk[i as usize] = ch_fac;
        i += 1;
    }
    if bands_diff > 0 as core::ffi::c_int {
        incr = -(1 as core::ffi::c_int) as WORD32;
        k = (out_bands as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    } else {
        incr = 1 as core::ffi::c_int as WORD32;
        k = 0 as core::ffi::c_int as WORD32;
    }
    while bands_diff != 0 as core::ffi::c_int {
        vdk[k as usize] = vdk[k as usize] - incr;
        k = k + incr;
        bands_diff = bands_diff + incr;
        if k >= out_bands {
            if bands_diff > 0 as core::ffi::c_int {
                k = (out_bands as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
            } else if bands_diff < 0 as core::ffi::c_int {
                k = 0 as core::ffi::c_int as WORD32;
            }
        }
    }
    *map.offset(0 as core::ffi::c_int as isize) = band_start;
    i = 0 as core::ffi::c_int as WORD32;
    while i < out_bands {
        *map.offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = *map
            .offset(i as isize) + vdk[i as usize];
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_mps_mapfrequency(
    mut in_0: *mut WORD32,
    mut out: *mut WORD32,
    mut map: *mut WORD32,
    mut data_bands: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut band_start: WORD32 = 0;
    let mut band_stop: WORD32 = 0;
    let mut value: WORD32 = 0;
    let mut start_band_0: WORD32 = *map.offset(0 as core::ffi::c_int as isize);
    i = 0 as core::ffi::c_int as WORD32;
    while i < data_bands {
        value = *in_0.offset((i + start_band_0) as isize);
        band_start = *map.offset(i as isize);
        band_stop = *map
            .offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
        j = band_start;
        while j < band_stop {
            *out.offset(j as isize) = value;
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_mps_de_quantize(
    mut value: WORD32,
    mut param_type: WORD32,
) -> FLOAT32 {
    match param_type {
        CLD => {
            return ixheaacd_cld_de_quant_table[(value as core::ffi::c_int
                + 15 as core::ffi::c_int) as usize];
        }
        ICC => return ixheaacd_icc_de_quant_table[value as usize],
        IPD => {
            return ixheaacd_ipd_de_quant_table[(value as core::ffi::c_int
                & 15 as core::ffi::c_int) as usize];
        }
        _ => return 0 as core::ffi::c_int as FLOAT32,
    };
}
unsafe extern "C" fn ixheaacd_mps_mapindexdata(
    mut self_0: *mut ia_mps_dec_state_struct,
    mut frame_xxx_data: *mut ia_mps_data_struct,
    mut out_data: *mut [FLOAT32; 28],
    mut out_idx_data: *mut [WORD32; 28],
    mut cmp_idx_data: *mut [WORD32; 28],
    mut idx_prev: *mut WORD32,
    mut param_type: WORD32,
) -> IA_ERRORCODE {
    let mut interpolate_local: [WORD32; 9] = [0 as core::ffi::c_int; 9];
    let mut map: [WORD32; 29] = [0; 29];
    let mut set_index: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut band: WORD32 = 0;
    let mut parm_slot: WORD32 = 0;
    let mut data_bands: WORD32 = 0;
    let mut ch_fac: WORD32 = 0;
    let mut ps: WORD32 = 0;
    let mut i1: WORD32 = 0;
    let mut i2: WORD32 = 0;
    let mut x1: WORD32 = 0;
    let mut xi: WORD32 = 0;
    let mut x2: WORD32 = 0;
    let mut band_start: WORD32 = 0 as WORD32;
    let mut ext_frame_flag: WORD32 = (*self_0).ext_frame_flag;
    let mut param_slots: *mut WORD32 = ((*self_0).param_slots).as_mut_ptr();
    let mut num_parameter_sets: WORD32 = (*self_0).num_parameter_sets;
    let mut band_stop: WORD32 = (*self_0).bs_param_bands;
    let mut default_val: WORD32 = 0 as WORD32;
    set_index = 0 as core::ffi::c_int as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_parameter_sets {
        if (*frame_xxx_data).bs_xxx_data_mode[i as usize] == 0 as core::ffi::c_int {
            (*frame_xxx_data).quant_coarse_xxx_flag[i as usize] = 0 as core::ffi::c_int
                as WORD32;
            band = band_start;
            while band < band_stop {
                (*out_idx_data.offset(i as isize))[band as usize] = default_val;
                band += 1;
            }
            band = band_start;
            while band < band_stop {
                *idx_prev.offset(band as isize) = (*out_idx_data
                    .offset(i as isize))[band as usize];
                band += 1;
            }
            (*frame_xxx_data).bs_quant_coarse_xxx_prev = 0 as WORD8;
        }
        if (*frame_xxx_data).bs_xxx_data_mode[i as usize] == 1 as core::ffi::c_int {
            band = band_start;
            while band < band_stop {
                (*out_idx_data.offset(i as isize))[band as usize] = *idx_prev
                    .offset(band as isize);
                band += 1;
            }
            (*frame_xxx_data).quant_coarse_xxx_flag[i as usize] = (*frame_xxx_data)
                .bs_quant_coarse_xxx_prev as WORD32;
        }
        if (*frame_xxx_data).bs_xxx_data_mode[i as usize] == 2 as core::ffi::c_int {
            band = band_start;
            while band < band_stop {
                (*out_idx_data.offset(i as isize))[band as usize] = *idx_prev
                    .offset(band as isize);
                band += 1;
            }
            (*frame_xxx_data).quant_coarse_xxx_flag[i as usize] = (*frame_xxx_data)
                .bs_quant_coarse_xxx_prev as WORD32;
            interpolate_local[i as usize] = 1 as core::ffi::c_int as WORD32;
        } else {
            interpolate_local[i as usize] = 0 as core::ffi::c_int as WORD32;
        }
        if (*frame_xxx_data).bs_xxx_data_mode[i as usize] == 3 as core::ffi::c_int {
            parm_slot = i;
            ch_fac = ixheaacd_mps_stride_table[(*frame_xxx_data)
                .bs_freq_res_stride_xxx[set_index as usize] as usize];
            data_bands = ((band_stop as core::ffi::c_int - band_start as core::ffi::c_int
                - 1 as core::ffi::c_int) / ch_fac as core::ffi::c_int
                + 1 as core::ffi::c_int) as WORD32;
            ixheaacd_mps_createmapping(map.as_mut_ptr(), band_start, band_stop, ch_fac);
            ixheaacd_mps_mapfrequency(
                &mut *(*cmp_idx_data.offset(set_index as isize))
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize),
                &mut *(*out_idx_data.offset(parm_slot as isize))
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize),
                map.as_mut_ptr(),
                data_bands,
            );
            band = band_start;
            while band < band_stop {
                *idx_prev.offset(band as isize) = (*out_idx_data
                    .offset(parm_slot as isize))[band as usize];
                band += 1;
            }
            (*frame_xxx_data).bs_quant_coarse_xxx_prev = (*frame_xxx_data)
                .bs_quant_coarse_xxx[set_index as usize];
            (*frame_xxx_data).quant_coarse_xxx_flag[i as usize] = (*frame_xxx_data)
                .bs_quant_coarse_xxx[set_index as usize] as WORD32;
            set_index += 1;
        }
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_parameter_sets {
        if (*frame_xxx_data).quant_coarse_xxx_flag[i as usize] == 1 as core::ffi::c_int {
            ixheaacd_mps_coarse2fine(
                (*out_idx_data.offset(i as isize)).as_mut_ptr(),
                param_type,
                band_start,
                band_stop - band_start,
            );
            (*frame_xxx_data).quant_coarse_xxx_flag[i as usize] = 0 as core::ffi::c_int
                as WORD32;
        }
        i += 1;
    }
    i1 = -(1 as core::ffi::c_int) as WORD32;
    x1 = 0 as core::ffi::c_int as WORD32;
    i2 = 0 as core::ffi::c_int as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_parameter_sets {
        if interpolate_local[i as usize] != 1 as core::ffi::c_int {
            i1 = i;
        }
        i2 = i;
        while interpolate_local[i2 as usize] == 1 as core::ffi::c_int {
            i2 += 1;
        }
        if i1 == -(1 as core::ffi::c_int) {
            x1 = 0 as core::ffi::c_int as WORD32;
            i1 = 0 as core::ffi::c_int as WORD32;
        } else {
            x1 = *param_slots.offset(i1 as isize);
        }
        xi = *param_slots.offset(i as isize);
        x2 = *param_slots.offset(i2 as isize);
        if interpolate_local[i as usize] == 1 as core::ffi::c_int {
            if i2 < num_parameter_sets {
                if (*self_0).ec_flag as core::ffi::c_int == 0 as core::ffi::c_int {
                    return IA_FATAL_ERROR as IA_ERRORCODE;
                }
            }
            band = band_start;
            while band < band_stop {
                let mut yi: WORD32 = 0;
                let mut y1: WORD32 = 0;
                let mut y2: WORD32 = 0;
                yi = 0 as core::ffi::c_int as WORD32;
                y1 = (*out_idx_data.offset(i1 as isize))[band as usize];
                y2 = (*out_idx_data.offset(i2 as isize))[band as usize];
                if param_type == IPD {
                    if y2 - y1 > 8 as core::ffi::c_int {
                        y1 += 16 as core::ffi::c_int;
                    }
                    if y1 - y2 > 8 as core::ffi::c_int {
                        y2 += 16 as core::ffi::c_int;
                    }
                    if x2 != x1 {
                        yi = ((y1 as core::ffi::c_int
                            + (xi as core::ffi::c_int - x1 as core::ffi::c_int)
                                * (y2 as core::ffi::c_int - y1 as core::ffi::c_int)
                                / (x2 as core::ffi::c_int - x1 as core::ffi::c_int))
                            % 16 as core::ffi::c_int) as WORD32;
                    }
                } else if x2 != x1 {
                    yi = y1 + (xi - x1) * (y2 - y1) / (x2 - x1);
                }
                (*out_idx_data.offset(i as isize))[band as usize] = yi;
                band += 1;
            }
        }
        i += 1;
    }
    ps = 0 as core::ffi::c_int as WORD32;
    while ps < num_parameter_sets {
        band = band_start;
        while band < band_stop {
            if param_type == CLD {
                (*out_idx_data.offset(ps as isize))[band as usize] = bound_check(
                    (*out_idx_data.offset(ps as isize))[band as usize],
                    -(15 as WORD32),
                    15 as WORD32,
                );
            } else if param_type == ICC {
                (*out_idx_data.offset(ps as isize))[band as usize] = bound_check(
                    (*out_idx_data.offset(ps as isize))[band as usize],
                    0 as WORD32,
                    7 as WORD32,
                );
            }
            (*out_data.offset(ps as isize))[band as usize] = ixheaacd_mps_de_quantize(
                (*out_idx_data.offset(ps as isize))[band as usize],
                param_type,
            );
            band += 1;
        }
        ps += 1;
    }
    if ext_frame_flag != 0 {
        band = band_start;
        while band < band_stop {
            (*out_data.offset(num_parameter_sets as isize))[band as usize] = (*out_data
                .offset(
                    (num_parameter_sets as core::ffi::c_int - 1 as core::ffi::c_int)
                        as isize,
                ))[band as usize];
            (*out_idx_data.offset(num_parameter_sets as isize))[band as usize] = (*out_idx_data
                .offset(
                    (num_parameter_sets as core::ffi::c_int - 1 as core::ffi::c_int)
                        as isize,
                ))[band as usize];
            band += 1;
        }
    }
    return IA_NO_ERROR;
}
unsafe extern "C" fn ixheaacd_mps_dec_and_mapframeott(
    mut self_0: *mut ia_mps_dec_state_struct,
) -> IA_ERRORCODE {
    let mut cur_bit_stream_ptr: *mut ia_mps_bs_frame = &mut (*self_0).bs_frame;
    let mut err_code: IA_ERRORCODE = 0 as IA_ERRORCODE;
    err_code = ixheaacd_mps_mapindexdata(
        self_0,
        &mut (*cur_bit_stream_ptr).cld_data,
        ((*self_0).cld_data).as_mut_ptr(),
        ((*cur_bit_stream_ptr).cld_idx).as_mut_ptr(),
        ((*cur_bit_stream_ptr).cmp_cld_idx).as_mut_ptr(),
        ((*cur_bit_stream_ptr).cld_idx_pre).as_mut_ptr(),
        CLD,
    );
    if err_code != IA_NO_ERROR {
        return err_code;
    }
    err_code = ixheaacd_mps_mapindexdata(
        self_0,
        &mut (*cur_bit_stream_ptr).icc_data,
        ((*self_0).icc_data).as_mut_ptr(),
        ((*cur_bit_stream_ptr).icc_idx).as_mut_ptr(),
        ((*cur_bit_stream_ptr).cmp_icc_idx).as_mut_ptr(),
        ((*cur_bit_stream_ptr).icc_idx_pre).as_mut_ptr(),
        ICC,
    );
    if err_code != IA_NO_ERROR {
        return err_code;
    }
    if (*(*self_0).config).bs_phase_coding != 0 {
        err_code = ixheaacd_mps_mapindexdata(
            self_0,
            &mut (*cur_bit_stream_ptr).ipd_data,
            ((*self_0).ipd_data).as_mut_ptr(),
            ((*cur_bit_stream_ptr).ipd_idx).as_mut_ptr(),
            ((*cur_bit_stream_ptr).ipd_idx_data).as_mut_ptr(),
            ((*cur_bit_stream_ptr).ipd_idx_prev).as_mut_ptr(),
            IPD,
        );
        if err_code != IA_NO_ERROR {
            return err_code;
        }
    }
    return IA_NO_ERROR;
}
unsafe extern "C" fn ixheaacd_mps_dec_and_mapframesmg(
    mut self_0: *mut ia_mps_dec_state_struct,
) -> VOID {
    let mut ps: WORD32 = 0;
    let mut pb: WORD32 = 0;
    let mut pg: WORD32 = 0;
    let mut ch_fac: WORD32 = 0;
    let mut data_bands: WORD32 = 0;
    let mut param_band_start: WORD32 = 0;
    let mut param_band_stop: WORD32 = 0;
    let mut group_to_band: [WORD32; 29] = [0; 29];
    let mut frame: *mut ia_mps_bs_frame = &mut (*self_0).bs_frame;
    ps = 0 as core::ffi::c_int as WORD32;
    while ps < (*self_0).num_parameter_sets {
        match (*frame).bs_smooth_mode[ps as usize] {
            0 => {
                (*self_0).smoothing_time[ps as usize] = 256 as core::ffi::c_int
                    as WORD32;
                (*self_0).inv_smoothing_time[ps as usize] = 1.0f32 / 256.0f32;
                pb = 0 as core::ffi::c_int as WORD32;
                while pb < (*self_0).bs_param_bands {
                    (*self_0).smoothing_data[ps as usize][pb as usize] = 0
                        as core::ffi::c_int as WORD32;
                    pb += 1;
                }
            }
            1 => {
                if ps > 0 as core::ffi::c_int {
                    (*self_0).smoothing_time[ps as usize] = (*self_0)
                        .smoothing_time[(ps as core::ffi::c_int - 1 as core::ffi::c_int)
                        as usize];
                    (*self_0).inv_smoothing_time[ps as usize] = (*self_0)
                        .inv_smoothing_time[(ps as core::ffi::c_int
                        - 1 as core::ffi::c_int) as usize];
                } else {
                    (*self_0).smoothing_time[ps as usize] = (*self_0)
                        .smoothing_filt_state
                        .prev_smg_time;
                    (*self_0).inv_smoothing_time[ps as usize] = (*self_0)
                        .smoothing_filt_state
                        .inv_prev_smg_time;
                }
                pb = 0 as core::ffi::c_int as WORD32;
                while pb < (*self_0).bs_param_bands {
                    if ps > 0 as core::ffi::c_int {
                        (*self_0).smoothing_data[ps as usize][pb as usize] = (*self_0)
                            .smoothing_data[(ps as core::ffi::c_int
                            - 1 as core::ffi::c_int) as usize][pb as usize];
                    } else {
                        (*self_0).smoothing_data[ps as usize][pb as usize] = (*self_0)
                            .smoothing_filt_state
                            .prev_smg_data[pb as usize];
                    }
                    pb += 1;
                }
            }
            2 => {
                (*self_0).smoothing_time[ps as usize] = ixheaacd_smoothing_time_table[(*frame)
                    .bs_smooth_time[ps as usize] as usize];
                (*self_0).inv_smoothing_time[ps as usize] = ixheaacd_inverse_smoothing_time_table[(*frame)
                    .bs_smooth_time[ps as usize] as usize];
                pb = 0 as core::ffi::c_int as WORD32;
                while pb < (*self_0).bs_param_bands {
                    (*self_0).smoothing_data[ps as usize][pb as usize] = 1
                        as core::ffi::c_int as WORD32;
                    pb += 1;
                }
            }
            3 => {
                (*self_0).smoothing_time[ps as usize] = ixheaacd_smoothing_time_table[(*frame)
                    .bs_smooth_time[ps as usize] as usize];
                (*self_0).inv_smoothing_time[ps as usize] = ixheaacd_inverse_smoothing_time_table[(*frame)
                    .bs_smooth_time[ps as usize] as usize];
                ch_fac = ixheaacd_mps_stride_table[(*frame)
                    .bs_freq_res_stride_smg[ps as usize] as usize];
                data_bands = (((*self_0).bs_param_bands as core::ffi::c_int
                    - 1 as core::ffi::c_int) / ch_fac as core::ffi::c_int
                    + 1 as core::ffi::c_int) as WORD32;
                ixheaacd_mps_createmapping(
                    group_to_band.as_mut_ptr(),
                    0 as WORD32,
                    (*self_0).bs_param_bands,
                    ch_fac,
                );
                pg = 0 as core::ffi::c_int as WORD32;
                while pg < data_bands {
                    param_band_start = group_to_band[pg as usize];
                    param_band_stop = group_to_band[(pg as core::ffi::c_int
                        + 1 as core::ffi::c_int) as usize];
                    pb = param_band_start;
                    while pb < param_band_stop {
                        (*self_0).smoothing_data[ps as usize][pb as usize] = (*frame)
                            .bs_smg_data[ps as usize][pg as usize];
                        pb += 1;
                    }
                    pg += 1;
                }
            }
            _ => {}
        }
        ps += 1;
    }
    (*self_0).smoothing_filt_state.prev_smg_time = (*self_0)
        .smoothing_time[((*self_0).num_parameter_sets as core::ffi::c_int
        - 1 as core::ffi::c_int) as usize];
    (*self_0).smoothing_filt_state.inv_prev_smg_time = (*self_0)
        .inv_smoothing_time[((*self_0).num_parameter_sets as core::ffi::c_int
        - 1 as core::ffi::c_int) as usize];
    pb = 0 as core::ffi::c_int as WORD32;
    while pb < (*self_0).bs_param_bands {
        (*self_0).smoothing_filt_state.prev_smg_data[pb as usize] = (*self_0)
            .smoothing_data[((*self_0).num_parameter_sets as core::ffi::c_int
            - 1 as core::ffi::c_int) as usize][pb as usize];
        pb += 1;
    }
    if (*self_0).ext_frame_flag != 0 {
        (*self_0).smoothing_time[(*self_0).num_parameter_sets as usize] = (*self_0)
            .smoothing_time[((*self_0).num_parameter_sets as core::ffi::c_int
            - 1 as core::ffi::c_int) as usize];
        (*self_0).inv_smoothing_time[(*self_0).num_parameter_sets as usize] = (*self_0)
            .inv_smoothing_time[((*self_0).num_parameter_sets as core::ffi::c_int
            - 1 as core::ffi::c_int) as usize];
        pb = 0 as core::ffi::c_int as WORD32;
        while pb < (*self_0).bs_param_bands {
            (*self_0)
                .smoothing_data[(*self_0).num_parameter_sets as usize][pb as usize] = (*self_0)
                .smoothing_data[((*self_0).num_parameter_sets as core::ffi::c_int
                - 1 as core::ffi::c_int) as usize][pb as usize];
            pb += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_frame_decode(
    mut self_0: *mut ia_mps_dec_state_struct,
) -> IA_ERRORCODE {
    let mut i: WORD32 = 0;
    let mut err_code: IA_ERRORCODE = 0 as IA_ERRORCODE;
    if (*self_0).ldmps_config.ldmps_present_flag != 1 as UINT32 {
        if (*self_0).parse_nxt_frame as core::ffi::c_int == 1 as core::ffi::c_int {
            return err_code;
        }
    }
    (*self_0).ext_frame_flag = 0 as core::ffi::c_int as WORD32;
    if (*self_0)
        .param_slots[((*self_0).num_parameter_sets as core::ffi::c_int
        - 1 as core::ffi::c_int) as usize]
        != (*self_0).time_slots as core::ffi::c_int - 1 as core::ffi::c_int
    {
        (*self_0).ext_frame_flag = 1 as core::ffi::c_int as WORD32;
    }
    err_code = ixheaacd_mps_dec_and_mapframeott(self_0);
    if err_code != IA_NO_ERROR {
        return err_code;
    }
    ixheaacd_mps_dec_and_mapframesmg(self_0);
    if (*self_0).ext_frame_flag != 0 {
        (*self_0).num_parameter_sets += 1;
        (*self_0)
            .param_slots[((*self_0).num_parameter_sets as core::ffi::c_int
            - 1 as core::ffi::c_int) as usize] = ((*self_0).time_slots
            as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    }
    (*self_0).param_slot_diff[0 as core::ffi::c_int as usize] = ((*self_0)
        .param_slots[0 as core::ffi::c_int as usize] + 1 as core::ffi::c_int) as WORD32;
    if MAX_TIME_SLOTS < (*self_0).param_slot_diff[0 as core::ffi::c_int as usize] {
        if (*self_0).ec_flag as core::ffi::c_int == 0 as core::ffi::c_int {
            return -(1 as IA_ERRORCODE)
        } else {
            (*self_0).param_slot_diff[0 as core::ffi::c_int as usize] = MAX_TIME_SLOTS
                as WORD32;
        }
    }
    (*self_0).inv_param_slot_diff[0 as core::ffi::c_int as usize] = 1 as core::ffi::c_int
        as FLOAT32
        / (*self_0).param_slot_diff[0 as core::ffi::c_int as usize] as FLOAT32;
    (*self_0).inv_param_slot_diff_Q30[0 as core::ffi::c_int as usize] = floor(
        ((*self_0).inv_param_slot_diff[0 as core::ffi::c_int as usize]
            * 1073741824 as core::ffi::c_int as FLOAT32) as core::ffi::c_double + 0.5f64,
    ) as WORD32;
    i = 1 as core::ffi::c_int as WORD32;
    while i < (*self_0).num_parameter_sets {
        (*self_0).param_slot_diff[i as usize] = (*self_0).param_slots[i as usize]
            - (*self_0)
                .param_slots[(i as core::ffi::c_int - 1 as core::ffi::c_int) as usize];
        if MAX_TIME_SLOTS
            < (*self_0).param_slot_diff[0 as core::ffi::c_int as usize]
                + (*self_0).param_slot_diff[i as usize]
            || (*self_0).param_slot_diff[i as usize] == 0 as core::ffi::c_int
        {
            if (*self_0).ec_flag as core::ffi::c_int == 0 as core::ffi::c_int {
                return -(1 as IA_ERRORCODE)
            } else {
                (*self_0).param_slot_diff[i as usize] = 1 as core::ffi::c_int as WORD32;
                (*self_0).inv_param_slot_diff[i as usize] = 1 as core::ffi::c_int
                    as FLOAT32;
            }
        }
        (*self_0).inv_param_slot_diff[i as usize] = 1 as core::ffi::c_int as FLOAT32
            / (*self_0).param_slot_diff[i as usize] as FLOAT32;
        (*self_0).inv_param_slot_diff_Q30[i as usize] = floor(
            ((*self_0).inv_param_slot_diff[i as usize]
                * 1073741824 as core::ffi::c_int as FLOAT32) as core::ffi::c_double
                + 0.5f64,
        ) as WORD32;
        i += 1;
    }
    return IA_NO_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_header_decode(
    mut self_0: *mut ia_mps_dec_state_struct,
) -> WORD32 {
    let mut samp_freq: WORD32 = 0;
    let mut sampling_rate_tbl: [WORD32; 16] = [
        96000 as core::ffi::c_int,
        88200 as core::ffi::c_int,
        64000 as core::ffi::c_int,
        48000 as core::ffi::c_int,
        44100 as core::ffi::c_int,
        32000 as core::ffi::c_int,
        24000 as core::ffi::c_int,
        22050 as core::ffi::c_int,
        16000 as core::ffi::c_int,
        12000 as core::ffi::c_int,
        11025 as core::ffi::c_int,
        8000 as core::ffi::c_int,
        7350 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ];
    if (*self_0).ldmps_config.ldmps_present_flag == 1 as UINT32 {
        (*self_0).time_slots = ((*self_0).frame_length as core::ffi::c_int
            + 1 as core::ffi::c_int) as WORD32;
    } else {
        (*self_0).time_slots = (*self_0).frame_length;
    }
    (*self_0).bs_param_bands = ixheaacd_freq_res_table[(*(*self_0).config).bs_freq_res
        as usize];
    if (*self_0).ldmps_config.ldmps_present_flag == 1 as UINT32 {
        if (*self_0).ldmps_config.bs_sampling_freq_index == 15 as UINT32 {
            samp_freq = (*self_0).ldmps_config.bs_fampling_frequency as WORD32;
        } else {
            samp_freq = sampling_rate_tbl[(*self_0).ldmps_config.bs_sampling_freq_index
                as usize];
        }
        if (samp_freq as core::ffi::c_double) < 27713.0f64 {
            (*self_0).qmf_band_count = 32 as core::ffi::c_int as WORD32;
        } else if samp_freq as core::ffi::c_double >= 55426.0f64 {
            (*self_0).qmf_band_count = 128 as core::ffi::c_int as WORD32;
        } else {
            (*self_0).qmf_band_count = 64 as core::ffi::c_int as WORD32;
        }
    }
    if (*self_0).object_type == AOT_ER_AAC_ELD as core::ffi::c_int
        || (*self_0).object_type == AOT_ER_AAC_LD as core::ffi::c_int
    {
        (*self_0).bs_param_bands = ixheaacd_freq_res_table_ld[(*(*self_0).config)
            .bs_freq_res as usize];
        (*self_0).hyb_band_count_max = (*self_0).qmf_band_count;
    } else {
        (*self_0).hyb_band_count_max = ((*self_0).qmf_band_count as core::ffi::c_int
            - QMF_BANDS_TO_HYBRID + 10 as core::ffi::c_int) as WORD32;
    }
    if (*self_0).object_type == AOT_ER_AAC_ELD as core::ffi::c_int
        || (*self_0).object_type == AOT_ER_AAC_LD as core::ffi::c_int
    {
        match (*self_0).bs_param_bands {
            4 => {
                (*self_0).hyb_band_to_processing_band_table = ixheaacd_hybrid_band_64_to_processing_band_4_map
                    .as_ptr();
            }
            5 => {
                (*self_0).hyb_band_to_processing_band_table = ixheaacd_hybrid_band_64_to_processing_band_5_map
                    .as_ptr();
            }
            7 => {
                (*self_0).hyb_band_to_processing_band_table = ixheaacd_hybrid_band_64_to_processing_band_7_map
                    .as_ptr();
            }
            9 => {
                (*self_0).hyb_band_to_processing_band_table = ixheaacd_hybrid_band_64_to_processing_band_9_map
                    .as_ptr();
            }
            12 => {
                (*self_0).hyb_band_to_processing_band_table = ixheaacd_hybrid_band_64_to_processing_band_12_map
                    .as_ptr();
            }
            15 => {
                (*self_0).hyb_band_to_processing_band_table = ixheaacd_hybrid_band_64_to_processing_band_15_map
                    .as_ptr();
            }
            23 => {
                (*self_0).hyb_band_to_processing_band_table = ixheaacd_hybrid_band_64_to_processing_band_23_map
                    .as_ptr();
            }
            _ => {
                (*self_0).hyb_band_to_processing_band_table = 0 as *const WORD32;
                return -(1 as WORD32);
            }
        }
    } else {
        match (*self_0).bs_param_bands {
            4 => {
                (*self_0).hyb_band_to_processing_band_table = ixheaacd_hybrid_band_71_to_processing_band_4_map
                    .as_ptr();
            }
            5 => {
                (*self_0).hyb_band_to_processing_band_table = ixheaacd_hybrid_band_71_to_processing_band_5_map
                    .as_ptr();
            }
            7 => {
                (*self_0).hyb_band_to_processing_band_table = ixheaacd_hybrid_band_71_to_processing_band_7_map
                    .as_ptr();
            }
            10 => {
                (*self_0).hyb_band_to_processing_band_table = ixheaacd_hybrid_band_71_to_processing_band_10_map
                    .as_ptr();
            }
            14 => {
                (*self_0).hyb_band_to_processing_band_table = ixheaacd_hybrid_band_71_to_processing_band_14_map
                    .as_ptr();
            }
            20 => {
                (*self_0).hyb_band_to_processing_band_table = ixheaacd_hybrid_band_71_to_processing_band_20_map
                    .as_ptr();
            }
            28 => {
                (*self_0).hyb_band_to_processing_band_table = ixheaacd_hybrid_band_71_to_processing_band_28_map
                    .as_ptr();
            }
            _ => {
                (*self_0).hyb_band_to_processing_band_table = 0 as *const WORD32;
                return -(1 as WORD32);
            }
        }
    }
    (*self_0).in_ch_count = 1 as core::ffi::c_int as WORD32;
    (*self_0).out_ch_count = 2 as core::ffi::c_int as WORD32;
    (*self_0).input_gain = ixheaacd_mps_clip_gain_table[(*(*self_0).config)
        .bs_fixed_gain_dmx as usize];
    if (*(*self_0).config).bs_ott_bands_phase_present != 0 {
        (*self_0).num_bands_ipd = (*(*self_0).config).bs_ott_bands_phase as WORD32;
    } else if !((*self_0).object_type == AOT_ER_AAC_ELD as core::ffi::c_int
        || (*self_0).object_type == AOT_ER_AAC_LD as core::ffi::c_int)
    {
        match (*self_0).bs_param_bands {
            4 | 5 => {
                (*self_0).num_bands_ipd = 2 as core::ffi::c_int as WORD32;
            }
            7 => {
                (*self_0).num_bands_ipd = 3 as core::ffi::c_int as WORD32;
            }
            10 => {
                (*self_0).num_bands_ipd = 5 as core::ffi::c_int as WORD32;
            }
            14 => {
                (*self_0).num_bands_ipd = 7 as core::ffi::c_int as WORD32;
            }
            20 | 28 => {
                (*self_0).num_bands_ipd = 10 as core::ffi::c_int as WORD32;
            }
            _ => return -(1 as WORD32),
        }
    }
    if (*self_0).residual_coding != 0 {
        (*self_0).num_bands_ipd = if (*self_0).bs_residual_bands
            > (*self_0).num_bands_ipd
        {
            (*self_0).bs_residual_bands
        } else {
            (*self_0).num_bands_ipd
        };
        (*self_0).max_res_bands = 0 as core::ffi::c_int as WORD32;
        if (*self_0).bs_residual_present != 0 {
            (*self_0).res_bands = (*self_0).bs_residual_bands;
            if (*self_0).res_bands > (*self_0).max_res_bands {
                (*self_0).max_res_bands = (*self_0).res_bands;
            }
        } else {
            (*self_0).res_bands = 0 as core::ffi::c_int as WORD32;
        }
    }
    if (*self_0).num_bands_ipd > MAX_PARAMETER_BANDS {
        return -(1 as WORD32);
    }
    (*self_0).dir_sig_count = 1 as core::ffi::c_int as WORD32;
    (*self_0).decor_sig_count = 1 as core::ffi::c_int as WORD32;
    (*self_0).bs_high_rate_mode = (*(*self_0).config).bs_high_rate_mode as WORD32;
    (*self_0).pre_mix_req = 1 as core::ffi::c_int as WORD32;
    return 0 as WORD32;
}
pub const CLD: core::ffi::c_int = 0;
pub const ICC: core::ffi::c_int = 1;
pub const IPD: core::ffi::c_int = 2 as core::ffi::c_int;
pub const IA_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_FATAL_ERROR: core::ffi::c_uint = 0x80000000 as core::ffi::c_uint;
