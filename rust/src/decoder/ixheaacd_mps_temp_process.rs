extern "C" {
    pub type ia_mps_dec_ducker_interface;
    fn sqrt(__x: core::ffi::c_double) -> core::ffi::c_double;
    static ixheaacd_mps_gain_set_indx: [WORD32; 29];
    fn ixheaacd_mps_qmf_hyb_synthesis(self_0: *mut ia_mps_dec_state_struct) -> VOID;
    fn ixheaacd_mps_synt_calc(self_0: *mut ia_mps_dec_state_struct) -> VOID;
    fn ixheaacd_sbr_dec_from_mps(
        p_mps_qmf_output: *mut FLOAT32,
        p_sbr_dec: *mut core::ffi::c_void,
        p_sbr_frame: *mut core::ffi::c_void,
        p_sbr_header: *mut core::ffi::c_void,
        ec_flag: WORD32,
    ) -> WORD32;
    fn ixheaacd_get_ch_idx(
        pstr_mps_state: *mut ia_heaac_mps_state_struct,
        row: WORD32,
        index: *mut WORD32,
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
pub type LOOPIDX = core::ffi::c_int;
pub type WORD = core::ffi::c_int;
pub type LOOPINDEX = LOOPIDX;
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
pub struct ixheaacd_lpp_trans_patch {
    pub num_patches: WORD32,
    pub start_subband: [WORD32; 7],
}
pub const AOT_ER_AAC_LD: AUDIO_OBJECT_TYPE = 23;
pub const AOT_ER_AAC_ELD: AUDIO_OBJECT_TYPE = 39;
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
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const TREE_7572: C2RustUnnamed_0 = 6;
pub const TREE_7571: C2RustUnnamed_0 = 5;
pub const TREE_7272: C2RustUnnamed_0 = 4;
pub const TREE_7271: C2RustUnnamed_0 = 3;
pub const TREE_525: C2RustUnnamed_0 = 2;
pub const TREE_5152: C2RustUnnamed_0 = 1;
pub const TREE_5151: C2RustUnnamed_0 = 0;
pub const ABS_THR: core::ffi::c_float = 1e-9f32
    * 32768 as core::ffi::c_int as core::ffi::c_float
    * 32768 as core::ffi::c_int as core::ffi::c_float;
pub const ABS_THR_FIX: core::ffi::c_int = 35184 as core::ffi::c_int;
pub const MAX_NUM_QMF_BANDS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const MAX_HYBRID_BANDS: core::ffi::c_int = MAX_NUM_QMF_BANDS - 3 as core::ffi::c_int
    + 10 as core::ffi::c_int;
pub const MAX_OUTPUT_CHANNELS_MPS: core::ffi::c_int = 8 as core::ffi::c_int;
pub const BP_SIZE: core::ffi::c_int = 25 as core::ffi::c_int;
pub const HP_SIZE: core::ffi::c_int = 9 as core::ffi::c_int;
pub const STP_LPF_COEFF1: core::ffi::c_float = 0.950f32;
pub const STP_LPF_COEFF2: core::ffi::c_float = 0.450f32;
pub const STP_UPDATE_ENERGY_RATE: core::ffi::c_int = 32 as core::ffi::c_int;
pub const STP_DAMP: core::ffi::c_float = 0.1f32;
static mut ixheaacd_bp: [FLOAT32; 25] = [
    0.0000f32,
    0.0005f32,
    0.0092f32,
    0.0587f32,
    0.2580f32,
    0.7392f32,
    0.9791f32,
    0.9993f32,
    1.0000f32,
    1.0000f32,
    1.0000f32,
    1.0000f32,
    0.9999f32,
    0.9984f32,
    0.9908f32,
    0.9639f32,
    0.8952f32,
    0.7711f32,
    0.6127f32,
    0.4609f32,
    0.3391f32,
    0.2493f32,
    0.1848f32,
    0.1387f32,
    0.1053f32,
];
static mut ixheaacd_gf: [FLOAT32; 25] = [
    0.0f32,
    0.0f32,
    0.0f32,
    0.0f32,
    0.0f32,
    0.0f32,
    1e-008f32,
    8.1e-007f32,
    3.61e-006f32,
    8.41e-006f32,
    1.6e-005f32,
    2.704e-005f32,
    3.969e-005f32,
    5.625e-005f32,
    7.396e-005f32,
    9.801e-005f32,
    0.00012321f32,
    0.00015625f32,
    0.00019881f32,
    0.00024964f32,
    0.00032041f32,
    0.00041209f32,
    0.00053824f32,
    0.00070756f32,
    0.00094249f32,
];
unsafe extern "C" fn ixheaacd_mps_temp_process_scale_calc(
    mut self_0: *mut ia_mps_dec_state_struct,
    mut ts: WORD32,
    mut scale: *mut FLOAT32,
) -> VOID {
    let mut dir_energy: FLOAT32 = 0.;
    let mut diff_energy: [FLOAT32; 2] = [0.; 2];
    let mut temp: FLOAT32 = 0.;
    let mut ch: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut left_ch: WORD32 = 0 as WORD32;
    let mut right_ch: WORD32 = 1 as WORD32;
    if (*self_0).subband_var.init_flag == 0 as core::ffi::c_int {
        ch = 0 as core::ffi::c_int as WORD32;
        while ch < 2 as core::ffi::c_int {
            (*self_0).subband_var.tp_scale_last[ch as usize] = 1.0f32 as FLOAT32;
            (*self_0).subband_var.nrg_diff_prev[ch as usize] = (32768 as core::ffi::c_int
                * 32768 as core::ffi::c_int) as FLOAT32;
            ch += 1;
        }
        (*self_0).subband_var.nrg_dir_prev = (32768 as core::ffi::c_int
            * 32768 as core::ffi::c_int) as FLOAT32;
        (*self_0).subband_var.init_flag = 1 as core::ffi::c_int as WORD32;
    }
    if (*self_0).subband_var.update_old_ener == STP_UPDATE_ENERGY_RATE {
        (*self_0).subband_var.update_old_ener = 1 as core::ffi::c_int as WORD32;
        (*self_0).subband_var.nrg_dir_prev = (*self_0).subband_var.nrg_dir;
        ch = 0 as core::ffi::c_int as WORD32;
        while ch < (*self_0).out_ch_count {
            (*self_0).subband_var.nrg_diff_prev[ch as usize] = (*self_0)
                .subband_var
                .nrg_diff[ch as usize];
            ch += 1;
        }
    } else {
        (*self_0).subband_var.update_old_ener += 1;
    }
    dir_energy = 0 as core::ffi::c_int as FLOAT32;
    n = 6 as core::ffi::c_int as WORD32;
    while n < BP_SIZE {
        let mut dir_left_re: FLOAT32 = (*self_0)
            .hyb_dir_out[left_ch
                as usize][ts
                as usize][(n as core::ffi::c_int + 7 as core::ffi::c_int) as usize]
            .re;
        let mut dir_right_re: FLOAT32 = (*self_0)
            .hyb_dir_out[right_ch
                as usize][ts
                as usize][(n as core::ffi::c_int + 7 as core::ffi::c_int) as usize]
            .re;
        let mut dir_left_im: FLOAT32 = (*self_0)
            .hyb_dir_out[left_ch
                as usize][ts
                as usize][(n as core::ffi::c_int + 7 as core::ffi::c_int) as usize]
            .im;
        let mut dir_right_im: FLOAT32 = (*self_0)
            .hyb_dir_out[right_ch
                as usize][ts
                as usize][(n as core::ffi::c_int + 7 as core::ffi::c_int) as usize]
            .im;
        temp = (dir_left_re + dir_right_re) * (dir_left_re + dir_right_re)
            + (dir_left_im + dir_right_im) * (dir_left_im + dir_right_im);
        dir_energy
            += temp * ixheaacd_bp[n as usize] * ixheaacd_bp[n as usize]
                * ixheaacd_gf[n as usize] * ixheaacd_gf[n as usize];
        n += 1;
    }
    (*self_0).subband_var.nrg_dir = ((STP_LPF_COEFF1 * (*self_0).subband_var.nrg_dir)
        as core::ffi::c_double
        + (1.0f64 - STP_LPF_COEFF1 as core::ffi::c_double)
            * dir_energy as core::ffi::c_double) as FLOAT32;
    dir_energy /= (*self_0).subband_var.nrg_dir_prev as core::ffi::c_float + ABS_THR;
    ch = 0 as core::ffi::c_int as WORD32;
    while ch < (*self_0).out_ch_count {
        diff_energy[ch as usize] = 0 as core::ffi::c_int as FLOAT32;
        n = 6 as core::ffi::c_int as WORD32;
        while n < BP_SIZE {
            let mut diff_re: FLOAT32 = (*self_0)
                .hyb_diff_out[ch
                    as usize][ts
                    as usize][(n as core::ffi::c_int + 7 as core::ffi::c_int) as usize]
                .re;
            let mut diff_im: FLOAT32 = (*self_0)
                .hyb_diff_out[ch
                    as usize][ts
                    as usize][(n as core::ffi::c_int + 7 as core::ffi::c_int) as usize]
                .im;
            temp = diff_re * diff_re + diff_im * diff_im;
            diff_energy[ch as usize]
                += temp * ixheaacd_bp[n as usize] * ixheaacd_bp[n as usize]
                    * ixheaacd_gf[n as usize] * ixheaacd_gf[n as usize];
            n += 1;
        }
        (*self_0).subband_var.nrg_diff[ch as usize] = ((STP_LPF_COEFF1
            * (*self_0).subband_var.nrg_diff[ch as usize]) as core::ffi::c_double
            + (1.0f64 - STP_LPF_COEFF1 as core::ffi::c_double)
                * diff_energy[ch as usize] as core::ffi::c_double) as FLOAT32;
        diff_energy[ch as usize]
            /= (*self_0).subband_var.nrg_diff_prev[ch as usize] + ABS_THR;
        ch += 1;
    }
    *scale.offset(left_ch as isize) = sqrt(
        dir_energy as core::ffi::c_double
            / (diff_energy[left_ch as usize] as core::ffi::c_double + 1e-9f64),
    ) as FLOAT32;
    *scale.offset(right_ch as isize) = sqrt(
        dir_energy as core::ffi::c_double
            / (diff_energy[right_ch as usize] as core::ffi::c_double + 1e-9f64),
    ) as FLOAT32;
    ch = 0 as core::ffi::c_int as WORD32;
    while ch < (*self_0).out_ch_count {
        *scale.offset(ch as isize) = STP_DAMP
            + (1 as core::ffi::c_int as FLOAT32 - STP_DAMP) * *scale.offset(ch as isize);
        ch += 1;
    }
    ch = 0 as core::ffi::c_int as WORD32;
    while ch < (*self_0).out_ch_count {
        *scale.offset(ch as isize) = (if (if *scale.offset(ch as isize)
            > (1.0f64 / 2.82f64) as FLOAT32
        {
            *scale.offset(ch as isize)
        } else {
            (1.0f64 / 2.82f64) as FLOAT32
        }) < 2.82f32
        {
            if *scale.offset(ch as isize) > (1.0f64 / 2.82f64) as FLOAT32 {
                *scale.offset(ch as isize)
            } else {
                (1.0f64 / 2.82f64) as core::ffi::c_float
            }
        } else {
            2.82f32
        }) as FLOAT32;
        ch += 1;
    }
    ch = 0 as core::ffi::c_int as WORD32;
    while ch < (*self_0).out_ch_count {
        *scale.offset(ch as isize) = ((STP_LPF_COEFF2 * *scale.offset(ch as isize))
            as core::ffi::c_double
            + (1.0f64 - STP_LPF_COEFF2 as core::ffi::c_double)
                * (*self_0).subband_var.tp_scale_last[ch as usize]
                    as core::ffi::c_double) as FLOAT32;
        (*self_0).subband_var.tp_scale_last[ch as usize] = *scale.offset(ch as isize);
        ch += 1;
    }
}
unsafe extern "C" fn ixheaacd_mps_subbandtp(
    mut self_0: *mut ia_mps_dec_state_struct,
    mut ts: WORD32,
) -> VOID {
    let mut scale: [FLOAT32; 2] = [0.; 2];
    let mut ch: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut no_scaling: WORD32 = 0;
    let mut temp: FLOAT32 = 0.;
    let ixheaacd_hybrid_to_qmf_map: [WORD32; 10] = [
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
    ];
    let ixheaacd_hybrid_to_qmf_map_ldmps: [WORD32; 3] = [
        0 as core::ffi::c_int,
        1 as core::ffi::c_int,
        2 as core::ffi::c_int,
    ];
    let mut ptr_ixheaacd_hybrid_to_qmf_map: *const WORD32 = 0 as *const WORD32;
    let mut loop_counter: WORD32 = 0 as WORD32;
    if (*self_0).ldmps_config.ldmps_present_flag != 0 {
        ptr_ixheaacd_hybrid_to_qmf_map = ixheaacd_hybrid_to_qmf_map_ldmps.as_ptr();
        loop_counter = 3 as core::ffi::c_int as WORD32;
    } else {
        ptr_ixheaacd_hybrid_to_qmf_map = ixheaacd_hybrid_to_qmf_map.as_ptr();
        loop_counter = 10 as core::ffi::c_int as WORD32;
    }
    ixheaacd_mps_temp_process_scale_calc(self_0, ts, scale.as_mut_ptr());
    ch = 0 as core::ffi::c_int as WORD32;
    while ch < (*self_0).out_ch_count {
        no_scaling = 1 as core::ffi::c_int as WORD32;
        if (*(*self_0).config).bs_temp_shape_config == 1 as UINT32
            || (*(*self_0).config).bs_temp_shape_config == 2 as UINT32
        {
            no_scaling = ((*self_0).temp_shape_enable_ch_stp[ch as usize] == 0)
                as core::ffi::c_int as WORD32;
        }
        if no_scaling == 1 as core::ffi::c_int {
            n = 0 as core::ffi::c_int as WORD32;
            while n < (*self_0).hyb_band_count_max {
                (*self_0).hyb_dir_out[ch as usize][ts as usize][n as usize].re
                    += (*self_0).hyb_diff_out[ch as usize][ts as usize][n as usize].re;
                (*self_0).hyb_dir_out[ch as usize][ts as usize][n as usize].im
                    += (*self_0).hyb_diff_out[ch as usize][ts as usize][n as usize].im;
                n += 1;
            }
        } else {
            if (*self_0).ldmps_config.ldmps_present_flag != 0 {
                n = 0 as core::ffi::c_int as WORD32;
                while n < 3 as core::ffi::c_int {
                    temp = scale[ch as usize]
                        * ixheaacd_bp[*ptr_ixheaacd_hybrid_to_qmf_map.offset(n as isize)
                            as usize];
                    (*self_0).hyb_dir_out[ch as usize][ts as usize][n as usize].re
                        += (*self_0)
                            .hyb_diff_out[ch as usize][ts as usize][n as usize]
                            .re * temp;
                    (*self_0).hyb_dir_out[ch as usize][ts as usize][n as usize].im
                        += (*self_0)
                            .hyb_diff_out[ch as usize][ts as usize][n as usize]
                            .im * temp;
                    n += 1;
                }
            } else {
                n = 0 as core::ffi::c_int as WORD32;
                while n < loop_counter {
                    temp = scale[ch as usize]
                        * ixheaacd_bp[*ptr_ixheaacd_hybrid_to_qmf_map.offset(n as isize)
                            as usize];
                    (*self_0).hyb_dir_out[ch as usize][ts as usize][n as usize].re
                        += (*self_0)
                            .hyb_diff_out[ch as usize][ts as usize][n as usize]
                            .re * temp;
                    (*self_0).hyb_dir_out[ch as usize][ts as usize][n as usize].im
                        += (*self_0)
                            .hyb_diff_out[ch as usize][ts as usize][n as usize]
                            .im * temp;
                    n += 1;
                }
            }
            n = 7 as core::ffi::c_int as WORD32;
            while n < HP_SIZE - 3 as core::ffi::c_int + 10 as core::ffi::c_int {
                temp = scale[ch as usize]
                    * ixheaacd_bp[(n as core::ffi::c_int + 3 as core::ffi::c_int
                        - 10 as core::ffi::c_int) as usize];
                (*self_0).hyb_dir_out[ch as usize][ts as usize][n as usize].re
                    += (*self_0).hyb_diff_out[ch as usize][ts as usize][n as usize].re
                        * temp;
                (*self_0).hyb_dir_out[ch as usize][ts as usize][n as usize].im
                    += (*self_0).hyb_diff_out[ch as usize][ts as usize][n as usize].im
                        * temp;
                n += 1;
            }
            while n < (*self_0).hyb_band_count_max {
                temp = scale[ch as usize];
                (*self_0).hyb_dir_out[ch as usize][ts as usize][n as usize].re
                    += (*self_0).hyb_diff_out[ch as usize][ts as usize][n as usize].re
                        * temp;
                (*self_0).hyb_dir_out[ch as usize][ts as usize][n as usize].im
                    += (*self_0).hyb_diff_out[ch as usize][ts as usize][n as usize].im
                        * temp;
                n += 1;
            }
        }
        ch += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_temp_process(
    mut self_0: *mut ia_mps_dec_state_struct,
) -> WORD32 {
    let mut ch: WORD32 = 0;
    let mut ts: WORD32 = 0;
    let mut hyb: WORD32 = 0;
    let mut err: WORD32 = 0 as WORD32;
    let mut ptr_frame_data: *mut ia_sbr_frame_info_data_struct = (*self_0)
        .p_sbr_frame[0 as core::ffi::c_int as usize]
        as *mut ia_sbr_frame_info_data_struct;
    if (*self_0).res_bands != 28 as core::ffi::c_int {
        if (*(*self_0).config).bs_temp_shape_config == 1 as UINT32 {
            let mut dif_s: WORD32 = if (*self_0).res_bands == 0 as core::ffi::c_int {
                0 as WORD32
            } else {
                ixheaacd_mps_gain_set_indx[(*self_0).res_bands as usize]
            };
            ch = 0 as core::ffi::c_int as WORD32;
            while ch < (*self_0).out_ch_count {
                ts = 0 as core::ffi::c_int as WORD32;
                while ts < (*self_0).time_slots {
                    hyb = dif_s;
                    while hyb < HYBRID_BAND_BORDER {
                        (*self_0).hyb_dir_out[ch as usize][ts as usize][hyb as usize].re
                            += (*self_0)
                                .hyb_diff_out[ch as usize][ts as usize][hyb as usize]
                                .re;
                        (*self_0).hyb_dir_out[ch as usize][ts as usize][hyb as usize].im
                            += (*self_0)
                                .hyb_diff_out[ch as usize][ts as usize][hyb as usize]
                                .im;
                        (*self_0)
                            .hyb_diff_out[ch as usize][ts as usize][hyb as usize]
                            .re = 0 as core::ffi::c_int as FLOAT32;
                        (*self_0)
                            .hyb_diff_out[ch as usize][ts as usize][hyb as usize]
                            .im = 0 as core::ffi::c_int as FLOAT32;
                        hyb += 1;
                    }
                    ts += 1;
                }
                ch += 1;
            }
            ts = 0 as core::ffi::c_int as WORD32;
            while ts < (*self_0).time_slots {
                ixheaacd_mps_subbandtp(self_0, ts);
                ts += 1;
            }
        } else {
            let mut dif_s_0: WORD32 = if (*self_0).res_bands == 0 as core::ffi::c_int {
                0 as WORD32
            } else {
                ixheaacd_mps_gain_set_indx[(*self_0).res_bands as usize]
            };
            ch = 0 as core::ffi::c_int as WORD32;
            while ch < (*self_0).out_ch_count {
                ts = 0 as core::ffi::c_int as WORD32;
                while ts < (*self_0).time_slots {
                    hyb = dif_s_0;
                    while hyb < (*self_0).hyb_band_count_max {
                        (*self_0).hyb_dir_out[ch as usize][ts as usize][hyb as usize].re
                            += (*self_0)
                                .hyb_diff_out[ch as usize][ts as usize][hyb as usize]
                                .re;
                        (*self_0).hyb_dir_out[ch as usize][ts as usize][hyb as usize].im
                            += (*self_0)
                                .hyb_diff_out[ch as usize][ts as usize][hyb as usize]
                                .im;
                        hyb += 1;
                    }
                    ts += 1;
                }
                ch += 1;
            }
        }
    }
    ixheaacd_mps_qmf_hyb_synthesis(self_0);
    if (*self_0).ldmps_config.ldmps_present_flag != 1 as UINT32 {
        ch = 0 as core::ffi::c_int as WORD32;
        while ch < (*self_0).out_ch_count {
            err = ixheaacd_sbr_dec_from_mps(
                &mut (*(*(*((*self_0).qmf_out_dir).as_mut_ptr().offset(ch as isize))
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize))
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize))
                    .re,
                (*self_0).p_sbr_dec[ch as usize],
                (*self_0).p_sbr_frame[ch as usize],
                (*self_0).p_sbr_header[ch as usize],
                (*self_0).ec_flag as WORD32,
            );
            if err != 0 {
                return err;
            }
            ch += 1;
        }
    }
    if (*self_0).object_type == AOT_ER_AAC_ELD as core::ffi::c_int
        || (*self_0).object_type == AOT_ER_AAC_LD as core::ffi::c_int
    {
        (*self_0).synth_count = (*self_0).hyb_band_count[0 as core::ffi::c_int as usize];
    } else if (*ptr_frame_data).mps_sbr_flag != 0 {
        (*self_0).synth_count = (*(*(*ptr_frame_data).pstr_sbr_header)
            .pstr_freq_band_data)
            .sub_band_end as WORD32;
    } else {
        (*self_0).synth_count = (*self_0).band_count[0 as core::ffi::c_int as usize];
    }
    ixheaacd_mps_synt_calc(self_0);
    return err;
}
unsafe extern "C" fn ixheaacd_subband_tp(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
    mut ts: WORD32,
) -> VOID {
    let mut tp_process_table_ptr: *mut ia_mps_dec_tp_process_tables_struct = (*pstr_mps_state)
        .ia_mps_dec_mps_table
        .tp_process_table_ptr;
    let mut sqrt_tab: *const WORD32 = ((*(*pstr_mps_state)
        .ia_mps_dec_mps_table
        .common_table_ptr)
        .sqrt_tab)
        .as_mut_ptr();
    let mut sub_band_tp: *mut ia_mps_dec_subband_tp_params_struct = (*pstr_mps_state)
        .mps_persistent_mem
        .sub_band_params;
    let mut p_array_struct: *mut ia_mps_dec_reuse_array_struct = (*pstr_mps_state)
        .array_struct;
    let mut temp_1: WORD32 = 0;
    let mut temp_2: WORD32 = 0;
    let mut qtemp1: WORD16 = 0;
    let mut qtemp2: WORD16 = 0;
    let mut qmf_output_real_dry: *mut WORD32 = 0 as *mut WORD32;
    let mut qmf_output_imag_dry: *mut WORD32 = 0 as *mut WORD32;
    let mut qmf_output_real_wet: *mut WORD32 = 0 as *mut WORD32;
    let mut qmf_output_imag_wet: *mut WORD32 = 0 as *mut WORD32;
    let mut dmx_real: *mut WORD32 = 0 as *mut WORD32;
    let mut dmx_imag: *mut WORD32 = 0 as *mut WORD32;
    let mut dry_ener: *mut WORD32 = 0 as *mut WORD32;
    let mut wet_ener: *mut WORD32 = 0 as *mut WORD32;
    let mut q_dry_ener: *mut WORD16 = 0 as *mut WORD16;
    let mut q_wet_ener: *mut WORD16 = 0 as *mut WORD16;
    let mut p_buffer_real: *mut WORD32 = 0 as *mut WORD32;
    let mut p_buffer_imag: *mut WORD32 = 0 as *mut WORD32;
    let mut p_buffer_re: *mut WORD32 = 0 as *mut WORD32;
    let mut p_buffer_im: *mut WORD32 = 0 as *mut WORD32;
    let mut p_buf_real: *mut WORD32 = 0 as *mut WORD32;
    let mut p_buf_imag: *mut WORD32 = 0 as *mut WORD32;
    let mut p_buf_re: *mut WORD32 = 0 as *mut WORD32;
    let mut p_buf_im: *mut WORD32 = 0 as *mut WORD32;
    let mut scale: *mut WORD32 = 0 as *mut WORD32;
    let mut q_scale: *mut WORD16 = 0 as *mut WORD16;
    let mut damp: WORD32 = 0;
    let mut one_minus_damp: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut prev_tp_scale: *mut WORD32 = ((*sub_band_tp).prev_tp_scale).as_mut_ptr();
    let mut old_wet_ener: *mut WORD32 = ((*sub_band_tp).old_wet_ener).as_mut_ptr();
    let mut q_old_wet_ener: *mut WORD16 = ((*sub_band_tp).q_old_wet_ener).as_mut_ptr();
    let mut run_wet_ener: *mut WORD32 = ((*sub_band_tp).run_wet_ener).as_mut_ptr();
    let mut q_run_wet_ener: *mut WORD16 = ((*sub_band_tp).q_run_wet_ener).as_mut_ptr();
    let mut old_dry_ener: *mut WORD32 = ((*sub_band_tp).old_dry_ener).as_mut_ptr();
    let mut q_old_dry_ener: *mut WORD16 = ((*sub_band_tp).q_old_dry_ener).as_mut_ptr();
    let mut run_dry_ener: *mut WORD32 = ((*sub_band_tp).run_dry_ener).as_mut_ptr();
    let mut q_run_dry_ener: *mut WORD16 = ((*sub_band_tp).q_run_dry_ener).as_mut_ptr();
    let mut hyb_output_real_dry: *mut WORD32 = 0 as *mut WORD32;
    let mut hyb_output_imag_dry: *mut WORD32 = 0 as *mut WORD32;
    let mut p_hyb_out_dry_real: *mut WORD32 = 0 as *mut WORD32;
    let mut p_hyb_out_dry_imag: *mut WORD32 = 0 as *mut WORD32;
    let mut ch: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut no_scaling: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0 as WORD32;
    let mut offset: WORD32 = 0;
    let mut i_lf: WORD32 = 0 as WORD32;
    let mut i_rf: WORD32 = 0 as WORD32;
    let mut i_c: WORD32 = 0 as WORD32;
    let mut i_lfe: WORD32 = 0 as WORD32;
    let mut i_ls: WORD32 = 0 as WORD32;
    let mut i_rs: WORD32 = 0 as WORD32;
    let mut i_al: WORD32 = 0 as WORD32;
    let mut i_ar: WORD32 = 0 as WORD32;
    let mut loop_counter: WORD32 = 0 as WORD32;
    let mut num_input_channels: WORD32 = (*pstr_mps_state).num_input_channels;
    let mut num_output_channels: WORD32 = (*pstr_mps_state).num_output_channels;
    let mut hybrid_bands: WORD32 = (*pstr_mps_state).hybrid_bands;
    let mut tree_config: WORD32 = (*pstr_mps_state).tree_config;
    let mut dmx_real64: WORD64 = 0 as WORD64;
    let mut dmx_imag64: WORD64 = 0 as WORD64;
    dry_ener = (*pstr_mps_state).mps_scratch_mem_v as *mut WORD32;
    q_dry_ener = ((*pstr_mps_state).mps_scratch_mem_v as *mut WORD16)
        .offset(
            ((12 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD16>() as usize) as isize,
        );
    wet_ener = dry_ener
        .offset(
            ((9 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    q_wet_ener = q_dry_ener
        .offset(
            ((32 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD16>() as usize) as isize,
        );
    scale = wet_ener
        .offset(
            ((20 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    q_scale = q_wet_ener
        .offset(
            ((41 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD16>() as usize) as isize,
        );
    dmx_real = scale
        .offset(
            ((20 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    dmx_imag = dmx_real
        .offset(
            ((150 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    qmf_output_real_dry = dmx_imag
        .offset(
            ((150 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    qmf_output_imag_dry = qmf_output_real_dry
        .offset(
            ((512 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    qmf_output_real_wet = qmf_output_imag_dry
        .offset(
            ((512 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    qmf_output_imag_wet = qmf_output_real_wet
        .offset(
            ((512 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as isize,
        );
    if (*sub_band_tp).update_old_ener == STP_UPDATE_ENERGY_RATE {
        (*sub_band_tp).update_old_ener = 1 as core::ffi::c_int as WORD32;
        ch = 0 as core::ffi::c_int as WORD32;
        while ch < num_input_channels {
            *old_dry_ener.offset(ch as isize) = *run_dry_ener.offset(ch as isize);
            *q_old_dry_ener.offset(ch as isize) = *q_run_dry_ener.offset(ch as isize);
            ch += 1;
        }
        ch = 0 as core::ffi::c_int as WORD32;
        while ch < num_output_channels {
            *old_wet_ener.offset(ch as isize) = *run_wet_ener.offset(ch as isize);
            *q_old_wet_ener.offset(ch as isize) = *q_run_wet_ener.offset(ch as isize);
            ch += 1;
        }
    } else {
        (*sub_band_tp).update_old_ener += 1;
    }
    ch = 0 as core::ffi::c_int as WORD32;
    while ch < MAX_OUTPUT_CHANNELS_MPS {
        *scale.offset(ch as isize) = ONE_IN_Q15 as WORD32;
        *q_scale.offset(ch as isize) = 15 as WORD16;
        ch += 1;
    }
    match tree_config {
        0 => {
            i_lf = 0 as core::ffi::c_int as WORD32;
            i_rf = 1 as core::ffi::c_int as WORD32;
            i_c = 2 as core::ffi::c_int as WORD32;
            i_lfe = 3 as core::ffi::c_int as WORD32;
            i_ls = 4 as core::ffi::c_int as WORD32;
            i_rs = 5 as core::ffi::c_int as WORD32;
            loop_counter = 6 as core::ffi::c_int as WORD32;
        }
        1 => {
            i_lf = 0 as core::ffi::c_int as WORD32;
            i_rf = 2 as core::ffi::c_int as WORD32;
            i_c = 4 as core::ffi::c_int as WORD32;
            i_lfe = 5 as core::ffi::c_int as WORD32;
            i_ls = 1 as core::ffi::c_int as WORD32;
            i_rs = 3 as core::ffi::c_int as WORD32;
            loop_counter = 5 as core::ffi::c_int as WORD32;
        }
        2 => {
            i_lf = 0 as core::ffi::c_int as WORD32;
            i_rf = 2 as core::ffi::c_int as WORD32;
            i_c = 4 as core::ffi::c_int as WORD32;
            i_lfe = 5 as core::ffi::c_int as WORD32;
            i_ls = 1 as core::ffi::c_int as WORD32;
            i_rs = 3 as core::ffi::c_int as WORD32;
            loop_counter = 4 as core::ffi::c_int as WORD32;
        }
        3 | 4 | 6 => {
            i_lf = 0 as core::ffi::c_int as WORD32;
            i_rf = 3 as core::ffi::c_int as WORD32;
            i_c = 6 as core::ffi::c_int as WORD32;
            i_lfe = 7 as core::ffi::c_int as WORD32;
            i_ls = 2 as core::ffi::c_int as WORD32;
            i_rs = 5 as core::ffi::c_int as WORD32;
            i_al = 1 as core::ffi::c_int as WORD32;
            i_ar = 4 as core::ffi::c_int as WORD32;
            loop_counter = 6 as core::ffi::c_int as WORD32;
        }
        5 => {
            i_lf = 0 as core::ffi::c_int as WORD32;
            i_rf = 3 as core::ffi::c_int as WORD32;
            i_c = 6 as core::ffi::c_int as WORD32;
            i_lfe = 7 as core::ffi::c_int as WORD32;
            i_ls = 2 as core::ffi::c_int as WORD32;
            i_rs = 5 as core::ffi::c_int as WORD32;
            i_al = 1 as core::ffi::c_int as WORD32;
            i_ar = 4 as core::ffi::c_int as WORD32;
            loop_counter = 5 as core::ffi::c_int as WORD32;
        }
        _ => {}
    }
    offset = (ts as core::ffi::c_int * MAX_HYBRID_BANDS) as WORD32;
    p_buffer_real = ((*p_array_struct).buf_real)
        .offset(offset as isize)
        .offset(HYBRID_BAND_BORDER as isize);
    p_buffer_imag = ((*p_array_struct).buf_imag)
        .offset(offset as isize)
        .offset(HYBRID_BAND_BORDER as isize);
    ch = 0 as core::ffi::c_int as WORD32;
    while ch < num_output_channels {
        p_buffer_re = p_buffer_real;
        p_buffer_im = p_buffer_imag;
        i = QMF_OUT_START_IDX as WORD32;
        while i < BP_SIZE {
            let fresh64 = p_buffer_re;
            p_buffer_re = p_buffer_re.offset(1);
            let fresh65 = qmf_output_real_wet;
            qmf_output_real_wet = qmf_output_real_wet.offset(1);
            *fresh65 = *fresh64;
            let fresh66 = p_buffer_im;
            p_buffer_im = p_buffer_im.offset(1);
            let fresh67 = qmf_output_imag_wet;
            qmf_output_imag_wet = qmf_output_imag_wet.offset(1);
            *fresh67 = *fresh66;
            i += 1;
        }
        p_buffer_real = p_buffer_real.offset(TSXHB as isize);
        p_buffer_imag = p_buffer_imag.offset(TSXHB as isize);
        ch += 1;
    }
    i = QMF_OUT_OFFSET * num_output_channels;
    qmf_output_real_wet = qmf_output_real_wet.offset(-(i as isize));
    qmf_output_imag_wet = qmf_output_imag_wet.offset(-(i as isize));
    p_buffer_re = qmf_output_real_dry;
    p_buffer_im = qmf_output_imag_dry;
    hyb_output_real_dry = ((*p_array_struct).hyb_output_real_dry)
        .offset((ts as core::ffi::c_int * MAX_HYBRID_BANDS) as isize)
        .offset(6 as core::ffi::c_int as isize);
    hyb_output_imag_dry = ((*p_array_struct).hyb_output_imag_dry)
        .offset((ts as core::ffi::c_int * MAX_HYBRID_BANDS) as isize)
        .offset(6 as core::ffi::c_int as isize);
    ch = 0 as core::ffi::c_int as WORD32;
    while ch < loop_counter {
        let fresh68 = p_buffer_re;
        p_buffer_re = p_buffer_re.offset(1);
        *fresh68 = ixheaac_add32_sat(
            *hyb_output_real_dry.offset(0 as core::ffi::c_int as isize),
            *hyb_output_real_dry.offset(1 as core::ffi::c_int as isize),
        );
        let fresh69 = p_buffer_im;
        p_buffer_im = p_buffer_im.offset(1);
        *fresh69 = ixheaac_add32_sat(
            *hyb_output_imag_dry.offset(0 as core::ffi::c_int as isize),
            *hyb_output_imag_dry.offset(1 as core::ffi::c_int as isize),
        );
        hyb_output_real_dry = hyb_output_real_dry.offset(TSXHB as isize);
        hyb_output_imag_dry = hyb_output_imag_dry.offset(TSXHB as isize);
        ch += 1;
    }
    hyb_output_real_dry = ((*p_array_struct).hyb_output_real_dry)
        .offset((ts as core::ffi::c_int * MAX_HYBRID_BANDS) as isize)
        .offset(8 as core::ffi::c_int as isize);
    hyb_output_imag_dry = ((*p_array_struct).hyb_output_imag_dry)
        .offset((ts as core::ffi::c_int * MAX_HYBRID_BANDS) as isize)
        .offset(8 as core::ffi::c_int as isize);
    ch = 0 as core::ffi::c_int as WORD32;
    while ch < loop_counter {
        let fresh70 = p_buffer_re;
        p_buffer_re = p_buffer_re.offset(1);
        *fresh70 = ixheaac_add32_sat(
            *hyb_output_real_dry.offset(0 as core::ffi::c_int as isize),
            *hyb_output_real_dry.offset(1 as core::ffi::c_int as isize),
        );
        let fresh71 = p_buffer_im;
        p_buffer_im = p_buffer_im.offset(1);
        *fresh71 = ixheaac_add32_sat(
            *hyb_output_imag_dry.offset(0 as core::ffi::c_int as isize),
            *hyb_output_imag_dry.offset(1 as core::ffi::c_int as isize),
        );
        hyb_output_real_dry = hyb_output_real_dry.offset(TSXHB as isize);
        hyb_output_imag_dry = hyb_output_imag_dry.offset(TSXHB as isize);
        ch += 1;
    }
    p_hyb_out_dry_real = ((*p_array_struct).hyb_output_real_dry)
        .offset((ts as core::ffi::c_int * MAX_HYBRID_BANDS) as isize)
        .offset(10 as core::ffi::c_int as isize);
    p_hyb_out_dry_imag = ((*p_array_struct).hyb_output_imag_dry)
        .offset((ts as core::ffi::c_int * MAX_HYBRID_BANDS) as isize)
        .offset(10 as core::ffi::c_int as isize);
    i = 3 as core::ffi::c_int as WORD32;
    while i < BP_SIZE {
        hyb_output_real_dry = p_hyb_out_dry_real;
        hyb_output_imag_dry = p_hyb_out_dry_imag;
        ch = 0 as core::ffi::c_int as WORD32;
        while ch < loop_counter {
            let fresh72 = p_buffer_re;
            p_buffer_re = p_buffer_re.offset(1);
            *fresh72 = *hyb_output_real_dry;
            let fresh73 = p_buffer_im;
            p_buffer_im = p_buffer_im.offset(1);
            *fresh73 = *hyb_output_imag_dry;
            hyb_output_real_dry = hyb_output_real_dry.offset(TSXHB as isize);
            hyb_output_imag_dry = hyb_output_imag_dry.offset(TSXHB as isize);
            ch += 1;
        }
        p_hyb_out_dry_real = p_hyb_out_dry_real.offset(1);
        p_hyb_out_dry_imag = p_hyb_out_dry_imag.offset(1);
        i += 1;
    }
    n = 1 as core::ffi::c_int as WORD32;
    while n < BP_SIZE {
        match tree_config {
            0 => {
                let fresh74 = qmf_output_real_dry;
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                dmx_real64 = ixheaac_mult32x32in64(
                    *fresh74,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh75 = qmf_output_real_dry;
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                dmx_real64 = ixheaac_mac32x32in64(
                    dmx_real64,
                    *fresh75,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh76 = qmf_output_real_dry;
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                dmx_real64 = ixheaac_mac32x32in64(
                    dmx_real64,
                    *fresh76,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                let fresh77 = qmf_output_real_dry;
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                dmx_real64 = ixheaac_mac32x32in64(
                    dmx_real64,
                    *fresh77,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh78 = qmf_output_real_dry;
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                dmx_real64 = ixheaac_mac32x32in64(
                    dmx_real64,
                    *fresh78,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                *dmx_real = (dmx_real64 >> 30 as core::ffi::c_int) as WORD32;
                dmx_real = dmx_real.offset(1);
                dmx_real = dmx_real.offset(1);
            }
            1 => {
                let fresh79 = qmf_output_real_dry;
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                dmx_real64 = ixheaac_mult32x32in64(
                    *fresh79,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh80 = qmf_output_real_dry;
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                dmx_real64 = ixheaac_mac32x32in64(
                    dmx_real64,
                    *fresh80,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh81 = qmf_output_real_dry;
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                dmx_real64 = ixheaac_mac32x32in64(
                    dmx_real64,
                    *fresh81,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh82 = qmf_output_real_dry;
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                dmx_real64 = ixheaac_mac32x32in64(
                    dmx_real64,
                    *fresh82,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh83 = qmf_output_real_dry;
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                dmx_real64 = ixheaac_mac32x32in64(
                    dmx_real64,
                    *fresh83,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                *dmx_real = (dmx_real64 >> 30 as core::ffi::c_int) as WORD32;
                dmx_real = dmx_real.offset(1);
                dmx_real = dmx_real.offset(1);
            }
            2 => {
                let fresh84 = qmf_output_real_dry;
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                dmx_real64 = ixheaac_mult32x32in64(
                    *fresh84,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh85 = qmf_output_real_dry;
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                dmx_real64 = ixheaac_mac32x32in64(
                    dmx_real64,
                    *fresh85,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                *dmx_real = (dmx_real64 >> 30 as core::ffi::c_int) as WORD32;
                dmx_real = dmx_real.offset(1);
                let fresh86 = qmf_output_real_dry;
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                dmx_real64 = ixheaac_mult32x32in64(
                    *fresh86,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh87 = qmf_output_real_dry;
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                dmx_real64 = ixheaac_mac32x32in64(
                    dmx_real64,
                    *fresh87,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                *dmx_real = (dmx_real64 >> 30 as core::ffi::c_int) as WORD32;
                dmx_real = dmx_real.offset(1);
            }
            3 | 4 => {
                let fresh88 = qmf_output_real_dry;
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                dmx_real64 = ixheaac_mult32x32in64(
                    *fresh88,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh89 = qmf_output_real_dry;
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                dmx_real64 = ixheaac_mac32x32in64(
                    dmx_real64,
                    *fresh89,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh90 = qmf_output_real_dry;
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                dmx_real64 = ixheaac_mac32x32in64(
                    dmx_real64,
                    *fresh90,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                *dmx_real = (dmx_real64 >> 30 as core::ffi::c_int) as WORD32;
                dmx_real = dmx_real.offset(1);
                let fresh91 = qmf_output_real_dry;
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                dmx_real64 = ixheaac_mult32x32in64(
                    *fresh91,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh92 = qmf_output_real_dry;
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                dmx_real64 = ixheaac_mac32x32in64(
                    dmx_real64,
                    *fresh92,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh93 = qmf_output_real_dry;
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                dmx_real64 = ixheaac_mac32x32in64(
                    dmx_real64,
                    *fresh93,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                *dmx_real = (dmx_real64 >> 30 as core::ffi::c_int) as WORD32;
                dmx_real = dmx_real.offset(1);
            }
            5 => {
                let fresh94 = qmf_output_real_dry;
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                dmx_real64 = ixheaac_mult32x32in64(
                    *fresh94,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh95 = qmf_output_real_dry;
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                dmx_real64 = ixheaac_mac32x32in64(
                    dmx_real64,
                    *fresh95,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                *dmx_real = (dmx_real64 >> 30 as core::ffi::c_int) as WORD32;
                dmx_real = dmx_real.offset(1);
                let fresh96 = qmf_output_real_dry;
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                dmx_real64 = ixheaac_mult32x32in64(
                    *fresh96,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh97 = qmf_output_real_dry;
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                dmx_real64 = ixheaac_mac32x32in64(
                    dmx_real64,
                    *fresh97,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                *dmx_real = (dmx_real64 >> 30 as core::ffi::c_int) as WORD32;
                dmx_real = dmx_real.offset(1);
            }
            6 => {
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                let fresh98 = qmf_output_real_dry;
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                dmx_real64 = ixheaac_mult32x32in64(
                    *fresh98,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh99 = qmf_output_real_dry;
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                dmx_real64 = ixheaac_mac32x32in64(
                    dmx_real64,
                    *fresh99,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                *dmx_real = (dmx_real64 >> 30 as core::ffi::c_int) as WORD32;
                dmx_real = dmx_real.offset(1);
                let fresh100 = qmf_output_real_dry;
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                dmx_real64 = ixheaac_mult32x32in64(
                    *fresh100,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh101 = qmf_output_real_dry;
                qmf_output_real_dry = qmf_output_real_dry.offset(1);
                dmx_real64 = ixheaac_mac32x32in64(
                    dmx_real64,
                    *fresh101,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                *dmx_real = (dmx_real64 >> 30 as core::ffi::c_int) as WORD32;
                dmx_real = dmx_real.offset(1);
            }
            _ => {}
        }
        n += 1;
    }
    dmx_real = dmx_real.offset(-(DMX_OFFSET as isize));
    n = 1 as core::ffi::c_int as WORD32;
    while n < BP_SIZE {
        match tree_config {
            0 => {
                let fresh102 = qmf_output_imag_dry;
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                dmx_imag64 = ixheaac_mult32x32in64(
                    *fresh102,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh103 = qmf_output_imag_dry;
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                dmx_imag64 = ixheaac_mac32x32in64(
                    dmx_imag64,
                    *fresh103,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh104 = qmf_output_imag_dry;
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                dmx_imag64 = ixheaac_mac32x32in64(
                    dmx_imag64,
                    *fresh104,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                let fresh105 = qmf_output_imag_dry;
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                dmx_imag64 = ixheaac_mac32x32in64(
                    dmx_imag64,
                    *fresh105,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh106 = qmf_output_imag_dry;
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                dmx_imag64 = ixheaac_mac32x32in64(
                    dmx_imag64,
                    *fresh106,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                *dmx_imag = (dmx_imag64 >> 30 as core::ffi::c_int) as WORD32;
                dmx_imag = dmx_imag.offset(1);
                dmx_imag = dmx_imag.offset(1);
            }
            1 => {
                let fresh107 = qmf_output_imag_dry;
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                dmx_imag64 = ixheaac_mult32x32in64(
                    *fresh107,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh108 = qmf_output_imag_dry;
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                dmx_imag64 = ixheaac_mac32x32in64(
                    dmx_imag64,
                    *fresh108,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh109 = qmf_output_imag_dry;
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                dmx_imag64 = ixheaac_mac32x32in64(
                    dmx_imag64,
                    *fresh109,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh110 = qmf_output_imag_dry;
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                dmx_imag64 = ixheaac_mac32x32in64(
                    dmx_imag64,
                    *fresh110,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh111 = qmf_output_imag_dry;
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                dmx_imag64 = ixheaac_mac32x32in64(
                    dmx_imag64,
                    *fresh111,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                *dmx_imag = (dmx_imag64 >> 30 as core::ffi::c_int) as WORD32;
                dmx_imag = dmx_imag.offset(1);
                dmx_imag = dmx_imag.offset(1);
            }
            2 => {
                let fresh112 = qmf_output_imag_dry;
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                dmx_imag64 = ixheaac_mult32x32in64(
                    *fresh112,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh113 = qmf_output_imag_dry;
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                dmx_imag64 = ixheaac_mac32x32in64(
                    dmx_imag64,
                    *fresh113,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                *dmx_imag = (dmx_imag64 >> 30 as core::ffi::c_int) as WORD32;
                dmx_imag = dmx_imag.offset(1);
                let fresh114 = qmf_output_imag_dry;
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                dmx_imag64 = ixheaac_mult32x32in64(
                    *fresh114,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh115 = qmf_output_imag_dry;
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                dmx_imag64 = ixheaac_mac32x32in64(
                    dmx_imag64,
                    *fresh115,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                *dmx_imag = (dmx_imag64 >> 30 as core::ffi::c_int) as WORD32;
                dmx_imag = dmx_imag.offset(1);
            }
            3 | 4 => {
                let fresh116 = qmf_output_imag_dry;
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                dmx_imag64 = ixheaac_mult32x32in64(
                    *fresh116,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh117 = qmf_output_imag_dry;
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                dmx_imag64 = ixheaac_mac32x32in64(
                    dmx_imag64,
                    *fresh117,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh118 = qmf_output_imag_dry;
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                dmx_imag64 = ixheaac_mac32x32in64(
                    dmx_imag64,
                    *fresh118,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                *dmx_imag = (dmx_imag64 >> 30 as core::ffi::c_int) as WORD32;
                dmx_imag = dmx_imag.offset(1);
                let fresh119 = qmf_output_imag_dry;
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                dmx_imag64 = ixheaac_mult32x32in64(
                    *fresh119,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh120 = qmf_output_imag_dry;
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                dmx_imag64 = ixheaac_mac32x32in64(
                    dmx_imag64,
                    *fresh120,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh121 = qmf_output_imag_dry;
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                dmx_imag64 = ixheaac_mac32x32in64(
                    dmx_imag64,
                    *fresh121,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                *dmx_imag = (dmx_imag64 >> 30 as core::ffi::c_int) as WORD32;
                dmx_imag = dmx_imag.offset(1);
            }
            5 => {
                let fresh122 = qmf_output_imag_dry;
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                dmx_imag64 = ixheaac_mult32x32in64(
                    *fresh122,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh123 = qmf_output_imag_dry;
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                dmx_imag64 = ixheaac_mac32x32in64(
                    dmx_imag64,
                    *fresh123,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                *dmx_imag = (dmx_imag64 >> 30 as core::ffi::c_int) as WORD32;
                dmx_imag = dmx_imag.offset(1);
                let fresh124 = qmf_output_imag_dry;
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                dmx_imag64 = ixheaac_mult32x32in64(
                    *fresh124,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh125 = qmf_output_imag_dry;
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                dmx_imag64 = ixheaac_mac32x32in64(
                    dmx_imag64,
                    *fresh125,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                *dmx_imag = (dmx_imag64 >> 30 as core::ffi::c_int) as WORD32;
                dmx_imag = dmx_imag.offset(1);
            }
            6 => {
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                let fresh126 = qmf_output_imag_dry;
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                dmx_imag64 = ixheaac_mult32x32in64(
                    *fresh126,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh127 = qmf_output_imag_dry;
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                dmx_imag64 = ixheaac_mac32x32in64(
                    dmx_imag64,
                    *fresh127,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                *dmx_imag = (dmx_imag64 >> 30 as core::ffi::c_int) as WORD32;
                dmx_imag = dmx_imag.offset(1);
                let fresh128 = qmf_output_imag_dry;
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                dmx_imag64 = ixheaac_mult32x32in64(
                    *fresh128,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                let fresh129 = qmf_output_imag_dry;
                qmf_output_imag_dry = qmf_output_imag_dry.offset(1);
                dmx_imag64 = ixheaac_mac32x32in64(
                    dmx_imag64,
                    *fresh129,
                    (*tp_process_table_ptr).bpxgf[n as usize],
                );
                *dmx_imag = (dmx_imag64 >> 30 as core::ffi::c_int) as WORD32;
                dmx_imag = dmx_imag.offset(1);
            }
            _ => {}
        }
        n += 1;
    }
    dmx_imag = dmx_imag.offset(-(DMX_OFFSET as isize));
    ch = 0 as core::ffi::c_int as WORD32;
    while ch
        < (if (2 as core::ffi::c_int) < num_input_channels {
            2 as core::ffi::c_int
        } else {
            num_input_channels as core::ffi::c_int
        })
    {
        *dry_ener.offset(ch as isize) = 0 as core::ffi::c_int as WORD32;
        *q_dry_ener.offset(ch as isize) = 15 as WORD16;
        n = 1 as core::ffi::c_int as WORD32;
        while n < BP_SIZE {
            qtemp1 = 10 as WORD16;
            temp_1 = ixheaacd_mps_mult32(*dmx_real, *dmx_real, &mut qtemp1, qtemp1);
            dmx_real = dmx_real.offset(2 as core::ffi::c_int as isize);
            *dry_ener.offset(ch as isize) = ixheaacd_mps_add32(
                *dry_ener.offset(ch as isize),
                temp_1,
                &mut *q_dry_ener.offset(ch as isize),
                qtemp1,
            );
            qtemp1 = 10 as WORD16;
            temp_1 = ixheaacd_mps_mult32(*dmx_imag, *dmx_imag, &mut qtemp1, qtemp1);
            dmx_imag = dmx_imag.offset(2 as core::ffi::c_int as isize);
            *dry_ener.offset(ch as isize) = ixheaacd_mps_add32(
                *dry_ener.offset(ch as isize),
                temp_1,
                &mut *q_dry_ener.offset(ch as isize),
                qtemp1,
            );
            n += 1;
        }
        dmx_real = dmx_real.offset(-(DMX_OFFSET_MINUS_ONE as isize));
        dmx_imag = dmx_imag.offset(-(DMX_OFFSET_MINUS_ONE as isize));
        temp_1 = ixheaacd_mps_mult32_shr_15(
            *run_dry_ener.offset(ch as isize),
            STP_LPF_COEFF1_FIX,
        );
        temp_2 = (ONE_IN_Q15 - STP_LPF_COEFF1_FIX) as WORD32;
        temp_2 = ixheaacd_mps_mult32_shr_15(temp_2, *dry_ener.offset(ch as isize));
        *run_dry_ener.offset(ch as isize) = ixheaacd_mps_add32(
            temp_1,
            temp_2,
            &mut *q_run_dry_ener.offset(ch as isize),
            *q_dry_ener.offset(ch as isize),
        );
        qtemp1 = *q_old_dry_ener.offset(ch as isize);
        temp_1 = ixheaacd_mps_add32(
            *old_dry_ener.offset(ch as isize),
            ABS_THR_FIX,
            &mut qtemp1,
            15 as WORD16,
        );
        *dry_ener.offset(ch as isize) = ixheaacd_mps_div_32(
            *dry_ener.offset(ch as isize),
            temp_1,
            &mut qtemp2,
        );
        *q_dry_ener.offset(ch as isize) = (qtemp2 as core::ffi::c_int
            + *q_dry_ener.offset(ch as isize) as core::ffi::c_int
            - qtemp1 as core::ffi::c_int) as WORD16;
        ch += 1;
    }
    ch = 0 as core::ffi::c_int as WORD32;
    while ch < num_output_channels {
        if !(ch == i_lfe) {
            if !(tree_config >= TREE_525 as core::ffi::c_int && ch == i_c) {
                if !(tree_config == TREE_7571 as core::ffi::c_int
                    && (ch == i_ls || ch == i_rs))
                {
                    if !(tree_config == TREE_7572 as core::ffi::c_int
                        && (ch == i_lf || ch == i_rf))
                    {
                        *wet_ener.offset(ch as isize) = 0 as core::ffi::c_int as WORD32;
                        *q_wet_ener.offset(ch as isize) = 15 as WORD16;
                        *wet_ener.offset(ch as isize) = 0 as core::ffi::c_int as WORD32;
                        *q_wet_ener.offset(ch as isize) = 15 as WORD16;
                        n = FIVE as WORD32;
                        while n < BP_SIZE {
                            qtemp1 = 10 as WORD16;
                            temp_1 = ixheaacd_mps_mult32(
                                *qmf_output_real_wet,
                                *qmf_output_real_wet,
                                &mut qtemp1,
                                qtemp1,
                            );
                            qmf_output_real_wet = qmf_output_real_wet.offset(1);
                            qtemp2 = 10 as WORD16;
                            temp_2 = ixheaacd_mps_mult32(
                                *qmf_output_imag_wet,
                                *qmf_output_imag_wet,
                                &mut qtemp2,
                                qtemp2,
                            );
                            qmf_output_imag_wet = qmf_output_imag_wet.offset(1);
                            temp_1 = ixheaacd_mps_add32(
                                temp_1,
                                temp_2,
                                &mut qtemp1,
                                qtemp2,
                            );
                            temp_1 = ixheaacd_mps_mult32(
                                temp_1,
                                (*tp_process_table_ptr).bp2xgf2[n as usize],
                                &mut qtemp1,
                                57 as WORD16,
                            );
                            *wet_ener.offset(ch as isize) = ixheaacd_mps_add32(
                                *wet_ener.offset(ch as isize),
                                temp_1,
                                &mut *q_wet_ener.offset(ch as isize),
                                qtemp1,
                            );
                            n += 1;
                        }
                        temp_1 = ixheaacd_mps_mult32_shr_15(
                            *run_wet_ener.offset(ch as isize),
                            STP_LPF_COEFF1_FIX,
                        );
                        temp_2 = (ONE_IN_Q15 - STP_LPF_COEFF1_FIX) as WORD32;
                        temp_2 = ixheaacd_mps_mult32_shr_15(
                            temp_2,
                            *wet_ener.offset(ch as isize),
                        );
                        *run_wet_ener.offset(ch as isize) = ixheaacd_mps_add32(
                            temp_1,
                            temp_2,
                            &mut *q_run_wet_ener.offset(ch as isize),
                            *q_wet_ener.offset(ch as isize),
                        );
                        qtemp1 = *q_old_wet_ener.offset(ch as isize);
                        temp_1 = ixheaacd_mps_add32(
                            *old_wet_ener.offset(ch as isize),
                            ABS_THR_FIX,
                            &mut qtemp1,
                            15 as WORD16,
                        );
                        *wet_ener.offset(ch as isize) = ixheaacd_mps_div_32(
                            *wet_ener.offset(ch as isize),
                            temp_1,
                            &mut qtemp2,
                        );
                        *q_wet_ener.offset(ch as isize) = (qtemp2 as core::ffi::c_int
                            + *q_wet_ener.offset(ch as isize) as core::ffi::c_int
                            - qtemp1 as core::ffi::c_int) as WORD16;
                    }
                }
            }
        }
        ch += 1;
    }
    damp = POINT_ONE_Q15 as WORD32;
    one_minus_damp = POINT_NINE_Q15 as WORD32;
    match tree_config {
        0 | 1 => {
            if *wet_ener.offset(i_lf as isize) != 0 as core::ffi::c_int {
                *scale.offset(i_lf as isize) = ixheaacd_mps_div_32(
                    *dry_ener.offset(0 as core::ffi::c_int as isize),
                    *wet_ener.offset(i_lf as isize),
                    &mut qtemp2,
                );
                *q_scale.offset(i_lf as isize) = (qtemp2 as core::ffi::c_int
                    + *q_dry_ener.offset(0 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                    - *q_wet_ener.offset(i_lf as isize) as core::ffi::c_int) as WORD16;
            } else {
                temp_1 = ixheaac_norm32(*dry_ener.offset(0 as core::ffi::c_int as isize))
                    as WORD32;
                *scale.offset(i_lf as isize) = *dry_ener
                    .offset(0 as core::ffi::c_int as isize) << temp_1;
                *q_scale.offset(i_lf as isize) = (*q_dry_ener
                    .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                    + temp_1 as core::ffi::c_int - 30 as core::ffi::c_int) as WORD16;
            }
            *scale.offset(i_lf as isize) = ixheaacd_mps_sqrt(
                *scale.offset(i_lf as isize),
                &mut *q_scale.offset(i_lf as isize),
                sqrt_tab,
            );
            if *wet_ener.offset(i_rf as isize) != 0 as core::ffi::c_int {
                *scale.offset(i_rf as isize) = ixheaacd_mps_div_32(
                    *dry_ener.offset(0 as core::ffi::c_int as isize),
                    *wet_ener.offset(i_rf as isize),
                    &mut qtemp2,
                );
                *q_scale.offset(i_rf as isize) = (qtemp2 as core::ffi::c_int
                    + *q_dry_ener.offset(0 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                    - *q_wet_ener.offset(i_rf as isize) as core::ffi::c_int) as WORD16;
            } else {
                temp_1 = ixheaac_norm32(*dry_ener.offset(0 as core::ffi::c_int as isize))
                    as WORD32;
                *scale.offset(i_rf as isize) = *dry_ener
                    .offset(0 as core::ffi::c_int as isize) << temp_1;
                *q_scale.offset(i_rf as isize) = (*q_dry_ener
                    .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                    + temp_1 as core::ffi::c_int - 30 as core::ffi::c_int) as WORD16;
            }
            *scale.offset(i_rf as isize) = ixheaacd_mps_sqrt(
                *scale.offset(i_rf as isize),
                &mut *q_scale.offset(i_rf as isize),
                sqrt_tab,
            );
            if *wet_ener.offset(i_c as isize) != 0 as core::ffi::c_int {
                *scale.offset(i_c as isize) = ixheaacd_mps_div_32(
                    *dry_ener.offset(0 as core::ffi::c_int as isize),
                    *wet_ener.offset(i_c as isize),
                    &mut qtemp2,
                );
                *q_scale.offset(i_c as isize) = (qtemp2 as core::ffi::c_int
                    + *q_dry_ener.offset(0 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                    - *q_wet_ener.offset(i_c as isize) as core::ffi::c_int) as WORD16;
            } else {
                temp_1 = ixheaac_norm32(*dry_ener.offset(0 as core::ffi::c_int as isize))
                    as WORD32;
                *scale.offset(i_c as isize) = *dry_ener
                    .offset(0 as core::ffi::c_int as isize) << temp_1;
                *q_scale.offset(i_c as isize) = (*q_dry_ener
                    .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                    + temp_1 as core::ffi::c_int - 30 as core::ffi::c_int) as WORD16;
            }
            *scale.offset(i_c as isize) = ixheaacd_mps_sqrt(
                *scale.offset(i_c as isize),
                &mut *q_scale.offset(i_c as isize),
                sqrt_tab,
            );
            if *wet_ener.offset(i_ls as isize) != 0 as core::ffi::c_int {
                *scale.offset(i_ls as isize) = ixheaacd_mps_div_32(
                    *dry_ener.offset(0 as core::ffi::c_int as isize),
                    *wet_ener.offset(i_ls as isize),
                    &mut qtemp2,
                );
                *q_scale.offset(i_ls as isize) = (qtemp2 as core::ffi::c_int
                    + *q_dry_ener.offset(0 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                    - *q_wet_ener.offset(i_ls as isize) as core::ffi::c_int) as WORD16;
            } else {
                temp_1 = ixheaac_norm32(*dry_ener.offset(0 as core::ffi::c_int as isize))
                    as WORD32;
                *scale.offset(i_ls as isize) = *dry_ener
                    .offset(0 as core::ffi::c_int as isize) << temp_1;
                *q_scale.offset(i_ls as isize) = (*q_dry_ener
                    .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                    + temp_1 as core::ffi::c_int - 30 as core::ffi::c_int) as WORD16;
            }
            *scale.offset(i_ls as isize) = ixheaacd_mps_sqrt(
                *scale.offset(i_ls as isize),
                &mut *q_scale.offset(i_ls as isize),
                sqrt_tab,
            );
            if *wet_ener.offset(i_rs as isize) != 0 as core::ffi::c_int {
                *scale.offset(i_rs as isize) = ixheaacd_mps_div_32(
                    *dry_ener.offset(0 as core::ffi::c_int as isize),
                    *wet_ener.offset(i_rs as isize),
                    &mut qtemp2,
                );
                *q_scale.offset(i_rs as isize) = (qtemp2 as core::ffi::c_int
                    + *q_dry_ener.offset(0 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                    - *q_wet_ener.offset(i_rs as isize) as core::ffi::c_int) as WORD16;
            } else {
                temp_1 = ixheaac_norm32(*dry_ener.offset(0 as core::ffi::c_int as isize))
                    as WORD32;
                *scale.offset(i_rs as isize) = *dry_ener
                    .offset(0 as core::ffi::c_int as isize) << temp_1;
                *q_scale.offset(i_rs as isize) = (*q_dry_ener
                    .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                    + temp_1 as core::ffi::c_int - 30 as core::ffi::c_int) as WORD16;
            }
            *scale.offset(i_rs as isize) = ixheaacd_mps_sqrt(
                *scale.offset(i_rs as isize),
                &mut *q_scale.offset(i_rs as isize),
                sqrt_tab,
            );
            ch = 0 as core::ffi::c_int as WORD32;
            while ch < 6 as core::ffi::c_int {
                if !(ch == 3 as core::ffi::c_int && tree_config == 0 as core::ffi::c_int)
                {
                    temp_1 = ixheaacd_mps_mult32_shr_15(
                        *scale.offset(ch as isize),
                        one_minus_damp,
                    );
                    *scale.offset(ch as isize) = ixheaacd_mps_add32(
                        temp_1,
                        damp,
                        &mut *q_scale.offset(ch as isize),
                        15 as WORD16,
                    );
                    *scale.offset(ch as isize) = ixheaacd_mps_convert_to_qn(
                        *scale.offset(ch as isize),
                        *q_scale.offset(ch as isize),
                        15 as WORD16,
                    );
                    if *scale.offset(ch as isize) > STP_SCALE_LIMIT_FIX {
                        *scale.offset(ch as isize) = STP_SCALE_LIMIT_FIX as WORD32;
                    }
                    if *scale.offset(ch as isize) < ONE_BY_STP_SCALE_LIMIT {
                        *scale.offset(ch as isize) = ONE_BY_STP_SCALE_LIMIT as WORD32;
                    }
                }
                ch += 1;
            }
        }
        2 => {
            if *wet_ener.offset(i_lf as isize) != 0 as core::ffi::c_int {
                *scale.offset(i_lf as isize) = ixheaacd_mps_div_32(
                    *dry_ener.offset(0 as core::ffi::c_int as isize),
                    *wet_ener.offset(i_lf as isize),
                    &mut qtemp2,
                );
                *q_scale.offset(i_lf as isize) = (qtemp2 as core::ffi::c_int
                    + *q_dry_ener.offset(0 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                    - *q_wet_ener.offset(i_lf as isize) as core::ffi::c_int) as WORD16;
            } else {
                temp_1 = ixheaac_norm32(*dry_ener.offset(0 as core::ffi::c_int as isize))
                    as WORD32;
                *scale.offset(i_lf as isize) = *dry_ener
                    .offset(0 as core::ffi::c_int as isize) << temp_1;
                *q_scale.offset(i_lf as isize) = (*q_dry_ener
                    .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                    + temp_1 as core::ffi::c_int - 30 as core::ffi::c_int) as WORD16;
            }
            *scale.offset(i_lf as isize) = ixheaacd_mps_sqrt(
                *scale.offset(i_lf as isize),
                &mut *q_scale.offset(i_lf as isize),
                sqrt_tab,
            );
            if *wet_ener.offset(i_rf as isize) != 0 as core::ffi::c_int {
                *scale.offset(i_rf as isize) = ixheaacd_mps_div_32(
                    *dry_ener.offset(1 as core::ffi::c_int as isize),
                    *wet_ener.offset(i_rf as isize),
                    &mut qtemp2,
                );
                *q_scale.offset(i_rf as isize) = (qtemp2 as core::ffi::c_int
                    + *q_dry_ener.offset(1 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                    - *q_wet_ener.offset(i_rf as isize) as core::ffi::c_int) as WORD16;
            } else {
                temp_1 = ixheaac_norm32(*dry_ener.offset(1 as core::ffi::c_int as isize))
                    as WORD32;
                *scale.offset(i_rf as isize) = *dry_ener
                    .offset(1 as core::ffi::c_int as isize) << temp_1;
                *q_scale.offset(i_rf as isize) = (*q_dry_ener
                    .offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                    + temp_1 as core::ffi::c_int - 30 as core::ffi::c_int) as WORD16;
            }
            *scale.offset(i_rf as isize) = ixheaacd_mps_sqrt(
                *scale.offset(i_rf as isize),
                &mut *q_scale.offset(i_rf as isize),
                sqrt_tab,
            );
            if *wet_ener.offset(i_ls as isize) != 0 as core::ffi::c_int {
                *scale.offset(i_ls as isize) = ixheaacd_mps_div_32(
                    *dry_ener.offset(0 as core::ffi::c_int as isize),
                    *wet_ener.offset(i_ls as isize),
                    &mut qtemp2,
                );
                *q_scale.offset(i_ls as isize) = (qtemp2 as core::ffi::c_int
                    + *q_dry_ener.offset(0 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                    - *q_wet_ener.offset(i_ls as isize) as core::ffi::c_int) as WORD16;
            } else {
                temp_1 = ixheaac_norm32(*dry_ener.offset(0 as core::ffi::c_int as isize))
                    as WORD32;
                *scale.offset(i_ls as isize) = *dry_ener
                    .offset(0 as core::ffi::c_int as isize) << temp_1;
                *q_scale.offset(i_ls as isize) = (*q_dry_ener
                    .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                    + temp_1 as core::ffi::c_int - 30 as core::ffi::c_int) as WORD16;
            }
            *scale.offset(i_ls as isize) = ixheaacd_mps_sqrt(
                *scale.offset(i_ls as isize),
                &mut *q_scale.offset(i_ls as isize),
                sqrt_tab,
            );
            if *wet_ener.offset(i_rs as isize) != 0 as core::ffi::c_int {
                *scale.offset(i_rs as isize) = ixheaacd_mps_div_32(
                    *dry_ener.offset(1 as core::ffi::c_int as isize),
                    *wet_ener.offset(i_rs as isize),
                    &mut qtemp2,
                );
                *q_scale.offset(i_rs as isize) = (qtemp2 as core::ffi::c_int
                    + *q_dry_ener.offset(1 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                    - *q_wet_ener.offset(i_rs as isize) as core::ffi::c_int) as WORD16;
            } else {
                temp_1 = ixheaac_norm32(*dry_ener.offset(1 as core::ffi::c_int as isize))
                    as WORD32;
                *scale.offset(i_rs as isize) = *dry_ener
                    .offset(1 as core::ffi::c_int as isize) << temp_1;
                *q_scale.offset(i_rs as isize) = (*q_dry_ener
                    .offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                    + temp_1 as core::ffi::c_int - 30 as core::ffi::c_int) as WORD16;
            }
            *scale.offset(i_rs as isize) = ixheaacd_mps_sqrt(
                *scale.offset(i_rs as isize),
                &mut *q_scale.offset(i_rs as isize),
                sqrt_tab,
            );
            ch = 0 as core::ffi::c_int as WORD32;
            while ch < 4 as core::ffi::c_int {
                temp_1 = ixheaacd_mps_mult32_shr_15(
                    *scale.offset(ch as isize),
                    one_minus_damp,
                );
                *scale.offset(ch as isize) = ixheaacd_mps_add32(
                    temp_1,
                    damp,
                    &mut *q_scale.offset(ch as isize),
                    15 as WORD16,
                );
                *scale.offset(ch as isize) = ixheaacd_mps_convert_to_qn(
                    *scale.offset(ch as isize),
                    *q_scale.offset(ch as isize),
                    15 as WORD16,
                );
                if *scale.offset(ch as isize) > STP_SCALE_LIMIT_FIX {
                    *scale.offset(ch as isize) = STP_SCALE_LIMIT_FIX as WORD32;
                }
                if *scale.offset(ch as isize) < ONE_BY_STP_SCALE_LIMIT {
                    *scale.offset(ch as isize) = ONE_BY_STP_SCALE_LIMIT as WORD32;
                }
                ch += 1;
            }
        }
        3 | 4 => {
            if *wet_ener.offset(i_lf as isize) != 0 as core::ffi::c_int {
                *scale.offset(i_lf as isize) = ixheaacd_mps_div_32(
                    *dry_ener.offset(0 as core::ffi::c_int as isize),
                    *wet_ener.offset(i_lf as isize),
                    &mut qtemp2,
                );
                *q_scale.offset(i_lf as isize) = (qtemp2 as core::ffi::c_int
                    + *q_dry_ener.offset(0 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                    - *q_wet_ener.offset(i_lf as isize) as core::ffi::c_int) as WORD16;
            } else {
                temp_1 = ixheaac_norm32(*dry_ener.offset(0 as core::ffi::c_int as isize))
                    as WORD32;
                *scale.offset(i_lf as isize) = *dry_ener
                    .offset(0 as core::ffi::c_int as isize) << temp_1;
                *q_scale.offset(i_lf as isize) = (*q_dry_ener
                    .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                    + temp_1 as core::ffi::c_int - 30 as core::ffi::c_int) as WORD16;
            }
            *scale.offset(i_lf as isize) = ixheaacd_mps_sqrt(
                *scale.offset(i_lf as isize),
                &mut *q_scale.offset(i_lf as isize),
                sqrt_tab,
            );
            if *wet_ener.offset(i_rf as isize) != 0 as core::ffi::c_int {
                *scale.offset(i_rf as isize) = ixheaacd_mps_div_32(
                    *dry_ener.offset(1 as core::ffi::c_int as isize),
                    *wet_ener.offset(i_rf as isize),
                    &mut qtemp2,
                );
                *q_scale.offset(i_rf as isize) = (qtemp2 as core::ffi::c_int
                    + *q_dry_ener.offset(1 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                    - *q_wet_ener.offset(i_rf as isize) as core::ffi::c_int) as WORD16;
            } else {
                temp_1 = ixheaac_norm32(*dry_ener.offset(1 as core::ffi::c_int as isize))
                    as WORD32;
                *scale.offset(i_rf as isize) = *dry_ener
                    .offset(1 as core::ffi::c_int as isize) << temp_1;
                *q_scale.offset(i_rf as isize) = (*q_dry_ener
                    .offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                    + temp_1 as core::ffi::c_int - 30 as core::ffi::c_int) as WORD16;
            }
            *scale.offset(i_rf as isize) = ixheaacd_mps_sqrt(
                *scale.offset(i_rf as isize),
                &mut *q_scale.offset(i_rf as isize),
                sqrt_tab,
            );
            if *wet_ener.offset(i_ls as isize) != 0 as core::ffi::c_int {
                *scale.offset(i_ls as isize) = ixheaacd_mps_div_32(
                    *dry_ener.offset(0 as core::ffi::c_int as isize),
                    *wet_ener.offset(i_ls as isize),
                    &mut qtemp2,
                );
                *q_scale.offset(i_ls as isize) = (qtemp2 as core::ffi::c_int
                    + *q_dry_ener.offset(0 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                    - *q_wet_ener.offset(i_ls as isize) as core::ffi::c_int) as WORD16;
            } else {
                temp_1 = ixheaac_norm32(*dry_ener.offset(0 as core::ffi::c_int as isize))
                    as WORD32;
                *scale.offset(i_ls as isize) = *dry_ener
                    .offset(0 as core::ffi::c_int as isize) << temp_1;
                *q_scale.offset(i_ls as isize) = (*q_dry_ener
                    .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                    + temp_1 as core::ffi::c_int - 30 as core::ffi::c_int) as WORD16;
            }
            *scale.offset(i_ls as isize) = ixheaacd_mps_sqrt(
                *scale.offset(i_ls as isize),
                &mut *q_scale.offset(i_ls as isize),
                sqrt_tab,
            );
            if *wet_ener.offset(i_rs as isize) != 0 as core::ffi::c_int {
                *scale.offset(i_rs as isize) = ixheaacd_mps_div_32(
                    *dry_ener.offset(1 as core::ffi::c_int as isize),
                    *wet_ener.offset(i_rs as isize),
                    &mut qtemp2,
                );
                *q_scale.offset(i_rs as isize) = (qtemp2 as core::ffi::c_int
                    + *q_dry_ener.offset(1 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                    - *q_wet_ener.offset(i_rs as isize) as core::ffi::c_int) as WORD16;
            } else {
                temp_1 = ixheaac_norm32(*dry_ener.offset(1 as core::ffi::c_int as isize))
                    as WORD32;
                *scale.offset(i_rs as isize) = *dry_ener
                    .offset(1 as core::ffi::c_int as isize) << temp_1;
                *q_scale.offset(i_rs as isize) = (*q_dry_ener
                    .offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                    + temp_1 as core::ffi::c_int - 30 as core::ffi::c_int) as WORD16;
            }
            *scale.offset(i_rs as isize) = ixheaacd_mps_sqrt(
                *scale.offset(i_rs as isize),
                &mut *q_scale.offset(i_rs as isize),
                sqrt_tab,
            );
            if *wet_ener.offset(i_al as isize) != 0 as core::ffi::c_int {
                *scale.offset(i_al as isize) = ixheaacd_mps_div_32(
                    *dry_ener.offset(0 as core::ffi::c_int as isize),
                    *wet_ener.offset(i_al as isize),
                    &mut qtemp2,
                );
                *q_scale.offset(i_al as isize) = (qtemp2 as core::ffi::c_int
                    + *q_dry_ener.offset(0 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                    - *q_wet_ener.offset(i_al as isize) as core::ffi::c_int) as WORD16;
            } else {
                temp_1 = ixheaac_norm32(*dry_ener.offset(0 as core::ffi::c_int as isize))
                    as WORD32;
                *scale.offset(i_al as isize) = *dry_ener
                    .offset(0 as core::ffi::c_int as isize) << temp_1;
                *q_scale.offset(i_al as isize) = (*q_dry_ener
                    .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                    + temp_1 as core::ffi::c_int - 30 as core::ffi::c_int) as WORD16;
            }
            *scale.offset(i_al as isize) = ixheaacd_mps_sqrt(
                *scale.offset(i_al as isize),
                &mut *q_scale.offset(i_al as isize),
                sqrt_tab,
            );
            if *wet_ener.offset(i_ar as isize) != 0 as core::ffi::c_int {
                *scale.offset(i_ar as isize) = ixheaacd_mps_div_32(
                    *dry_ener.offset(1 as core::ffi::c_int as isize),
                    *wet_ener.offset(i_ar as isize),
                    &mut qtemp2,
                );
                *q_scale.offset(i_ar as isize) = (qtemp2 as core::ffi::c_int
                    + *q_dry_ener.offset(1 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                    - *q_wet_ener.offset(i_ar as isize) as core::ffi::c_int) as WORD16;
            } else {
                temp_1 = ixheaac_norm32(*dry_ener.offset(1 as core::ffi::c_int as isize))
                    as WORD32;
                *scale.offset(i_ar as isize) = *dry_ener
                    .offset(1 as core::ffi::c_int as isize) << temp_1;
                *q_scale.offset(i_ar as isize) = (*q_dry_ener
                    .offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                    + temp_1 as core::ffi::c_int - 30 as core::ffi::c_int) as WORD16;
            }
            *scale.offset(i_ar as isize) = ixheaacd_mps_sqrt(
                *scale.offset(i_ar as isize),
                &mut *q_scale.offset(i_ar as isize),
                sqrt_tab,
            );
            ch = 0 as core::ffi::c_int as WORD32;
            while ch < 6 as core::ffi::c_int {
                temp_1 = ixheaacd_mps_mult32_shr_15(
                    *scale.offset(ch as isize),
                    one_minus_damp,
                );
                *scale.offset(ch as isize) = ixheaacd_mps_add32(
                    temp_1,
                    damp,
                    &mut *q_scale.offset(ch as isize),
                    15 as WORD16,
                );
                *scale.offset(ch as isize) = ixheaacd_mps_convert_to_qn(
                    *scale.offset(ch as isize),
                    *q_scale.offset(ch as isize),
                    15 as WORD16,
                );
                if *scale.offset(ch as isize) > STP_SCALE_LIMIT_FIX {
                    *scale.offset(ch as isize) = STP_SCALE_LIMIT_FIX as WORD32;
                }
                if *scale.offset(ch as isize) < ONE_BY_STP_SCALE_LIMIT {
                    *scale.offset(ch as isize) = ONE_BY_STP_SCALE_LIMIT as WORD32;
                }
                ch += 1;
            }
        }
        5 => {
            if *wet_ener.offset(i_lf as isize) != 0 as core::ffi::c_int {
                *scale.offset(i_lf as isize) = ixheaacd_mps_div_32(
                    *dry_ener.offset(0 as core::ffi::c_int as isize),
                    *wet_ener.offset(i_lf as isize),
                    &mut qtemp2,
                );
                *q_scale.offset(i_lf as isize) = (qtemp2 as core::ffi::c_int
                    + *q_dry_ener.offset(0 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                    - *q_wet_ener.offset(i_lf as isize) as core::ffi::c_int) as WORD16;
            } else {
                temp_1 = ixheaac_norm32(*dry_ener.offset(0 as core::ffi::c_int as isize))
                    as WORD32;
                *scale.offset(i_lf as isize) = *dry_ener
                    .offset(0 as core::ffi::c_int as isize) << temp_1;
                *q_scale.offset(i_lf as isize) = (*q_dry_ener
                    .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                    + temp_1 as core::ffi::c_int - 30 as core::ffi::c_int) as WORD16;
            }
            *scale.offset(i_lf as isize) = ixheaacd_mps_sqrt(
                *scale.offset(i_lf as isize),
                &mut *q_scale.offset(i_lf as isize),
                sqrt_tab,
            );
            if *wet_ener.offset(i_rf as isize) != 0 as core::ffi::c_int {
                *scale.offset(i_rf as isize) = ixheaacd_mps_div_32(
                    *dry_ener.offset(1 as core::ffi::c_int as isize),
                    *wet_ener.offset(i_rf as isize),
                    &mut qtemp2,
                );
                *q_scale.offset(i_rf as isize) = (qtemp2 as core::ffi::c_int
                    + *q_dry_ener.offset(1 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                    - *q_wet_ener.offset(i_rf as isize) as core::ffi::c_int) as WORD16;
            } else {
                temp_1 = ixheaac_norm32(*dry_ener.offset(1 as core::ffi::c_int as isize))
                    as WORD32;
                *scale.offset(i_rf as isize) = *dry_ener
                    .offset(1 as core::ffi::c_int as isize) << temp_1;
                *q_scale.offset(i_rf as isize) = (*q_dry_ener
                    .offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                    + temp_1 as core::ffi::c_int - 30 as core::ffi::c_int) as WORD16;
            }
            *scale.offset(i_rf as isize) = ixheaacd_mps_sqrt(
                *scale.offset(i_rf as isize),
                &mut *q_scale.offset(i_rf as isize),
                sqrt_tab,
            );
            if *wet_ener.offset(i_al as isize) != 0 as core::ffi::c_int {
                *scale.offset(i_al as isize) = ixheaacd_mps_div_32(
                    *dry_ener.offset(0 as core::ffi::c_int as isize),
                    *wet_ener.offset(i_al as isize),
                    &mut qtemp2,
                );
                *q_scale.offset(i_al as isize) = (qtemp2 as core::ffi::c_int
                    + *q_dry_ener.offset(0 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                    - *q_wet_ener.offset(i_al as isize) as core::ffi::c_int) as WORD16;
            } else {
                temp_1 = ixheaac_norm32(*dry_ener.offset(0 as core::ffi::c_int as isize))
                    as WORD32;
                *scale.offset(i_al as isize) = *dry_ener
                    .offset(0 as core::ffi::c_int as isize) << temp_1;
                *q_scale.offset(i_al as isize) = (*q_dry_ener
                    .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                    + temp_1 as core::ffi::c_int - 30 as core::ffi::c_int) as WORD16;
            }
            *scale.offset(i_al as isize) = ixheaacd_mps_sqrt(
                *scale.offset(i_al as isize),
                &mut *q_scale.offset(i_al as isize),
                sqrt_tab,
            );
            if *wet_ener.offset(i_ar as isize) != 0 as core::ffi::c_int {
                *scale.offset(i_ar as isize) = ixheaacd_mps_div_32(
                    *dry_ener.offset(1 as core::ffi::c_int as isize),
                    *wet_ener.offset(i_ar as isize),
                    &mut qtemp2,
                );
                *q_scale.offset(i_ar as isize) = (qtemp2 as core::ffi::c_int
                    + *q_dry_ener.offset(1 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                    - *q_wet_ener.offset(i_ar as isize) as core::ffi::c_int) as WORD16;
            } else {
                temp_1 = ixheaac_norm32(*dry_ener.offset(1 as core::ffi::c_int as isize))
                    as WORD32;
                *scale.offset(i_ar as isize) = *dry_ener
                    .offset(1 as core::ffi::c_int as isize) << temp_1;
                *q_scale.offset(i_ar as isize) = (*q_dry_ener
                    .offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                    + temp_1 as core::ffi::c_int - 30 as core::ffi::c_int) as WORD16;
            }
            *scale.offset(i_ar as isize) = ixheaacd_mps_sqrt(
                *scale.offset(i_ar as isize),
                &mut *q_scale.offset(i_ar as isize),
                sqrt_tab,
            );
            ch = 0 as core::ffi::c_int as WORD32;
            while ch < FIVE {
                if !(ch == 2 as core::ffi::c_int) {
                    temp_1 = ixheaacd_mps_mult32_shr_15(
                        *scale.offset(ch as isize),
                        one_minus_damp,
                    );
                    *scale.offset(ch as isize) = ixheaacd_mps_add32(
                        temp_1,
                        damp,
                        &mut *q_scale.offset(ch as isize),
                        15 as WORD16,
                    );
                    *scale.offset(ch as isize) = ixheaacd_mps_convert_to_qn(
                        *scale.offset(ch as isize),
                        *q_scale.offset(ch as isize),
                        15 as WORD16,
                    );
                    if *scale.offset(ch as isize) > STP_SCALE_LIMIT_FIX {
                        *scale.offset(ch as isize) = STP_SCALE_LIMIT_FIX as WORD32;
                    }
                    if *scale.offset(ch as isize) < ONE_BY_STP_SCALE_LIMIT {
                        *scale.offset(ch as isize) = ONE_BY_STP_SCALE_LIMIT as WORD32;
                    }
                }
                ch += 1;
            }
        }
        6 => {
            if *wet_ener.offset(i_ls as isize) != 0 as core::ffi::c_int {
                *scale.offset(i_ls as isize) = ixheaacd_mps_div_32(
                    *dry_ener.offset(0 as core::ffi::c_int as isize),
                    *wet_ener.offset(i_ls as isize),
                    &mut qtemp2,
                );
                *q_scale.offset(i_ls as isize) = (qtemp2 as core::ffi::c_int
                    + *q_dry_ener.offset(0 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                    - *q_wet_ener.offset(i_ls as isize) as core::ffi::c_int) as WORD16;
            } else {
                temp_1 = ixheaac_norm32(*dry_ener.offset(0 as core::ffi::c_int as isize))
                    as WORD32;
                *scale.offset(i_ls as isize) = *dry_ener
                    .offset(0 as core::ffi::c_int as isize) << temp_1;
                *q_scale.offset(i_ls as isize) = (*q_dry_ener
                    .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                    + temp_1 as core::ffi::c_int - 30 as core::ffi::c_int) as WORD16;
            }
            *scale.offset(i_ls as isize) = ixheaacd_mps_sqrt(
                *scale.offset(i_ls as isize),
                &mut *q_scale.offset(i_ls as isize),
                sqrt_tab,
            );
            if *wet_ener.offset(i_rs as isize) != 0 as core::ffi::c_int {
                *scale.offset(i_rs as isize) = ixheaacd_mps_div_32(
                    *dry_ener.offset(1 as core::ffi::c_int as isize),
                    *wet_ener.offset(i_rs as isize),
                    &mut qtemp2,
                );
                *q_scale.offset(i_rs as isize) = (qtemp2 as core::ffi::c_int
                    + *q_dry_ener.offset(1 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                    - *q_wet_ener.offset(i_rs as isize) as core::ffi::c_int) as WORD16;
            } else {
                temp_1 = ixheaac_norm32(*dry_ener.offset(1 as core::ffi::c_int as isize))
                    as WORD32;
                *scale.offset(i_rs as isize) = *dry_ener
                    .offset(1 as core::ffi::c_int as isize) << temp_1;
                *q_scale.offset(i_rs as isize) = (*q_dry_ener
                    .offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                    + temp_1 as core::ffi::c_int - 30 as core::ffi::c_int) as WORD16;
            }
            *scale.offset(i_rs as isize) = ixheaacd_mps_sqrt(
                *scale.offset(i_rs as isize),
                &mut *q_scale.offset(i_rs as isize),
                sqrt_tab,
            );
            if *wet_ener.offset(i_al as isize) != 0 as core::ffi::c_int {
                *scale.offset(i_al as isize) = ixheaacd_mps_div_32(
                    *dry_ener.offset(0 as core::ffi::c_int as isize),
                    *wet_ener.offset(i_al as isize),
                    &mut qtemp2,
                );
                *q_scale.offset(i_al as isize) = (qtemp2 as core::ffi::c_int
                    + *q_dry_ener.offset(0 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                    - *q_wet_ener.offset(i_al as isize) as core::ffi::c_int) as WORD16;
            } else {
                temp_1 = ixheaac_norm32(*dry_ener.offset(0 as core::ffi::c_int as isize))
                    as WORD32;
                *scale.offset(i_al as isize) = *dry_ener
                    .offset(0 as core::ffi::c_int as isize) << temp_1;
                *q_scale.offset(i_al as isize) = (*q_dry_ener
                    .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                    + temp_1 as core::ffi::c_int - 30 as core::ffi::c_int) as WORD16;
            }
            *scale.offset(i_al as isize) = ixheaacd_mps_sqrt(
                *scale.offset(i_al as isize),
                &mut *q_scale.offset(i_al as isize),
                sqrt_tab,
            );
            if *wet_ener.offset(i_ar as isize) != 0 as core::ffi::c_int {
                *scale.offset(i_ar as isize) = ixheaacd_mps_div_32(
                    *dry_ener.offset(1 as core::ffi::c_int as isize),
                    *wet_ener.offset(i_ar as isize),
                    &mut qtemp2,
                );
                *q_scale.offset(i_ar as isize) = (qtemp2 as core::ffi::c_int
                    + *q_dry_ener.offset(1 as core::ffi::c_int as isize)
                        as core::ffi::c_int
                    - *q_wet_ener.offset(i_ar as isize) as core::ffi::c_int) as WORD16;
            } else {
                temp_1 = ixheaac_norm32(*dry_ener.offset(1 as core::ffi::c_int as isize))
                    as WORD32;
                *scale.offset(i_ar as isize) = *dry_ener
                    .offset(1 as core::ffi::c_int as isize) << temp_1;
                *q_scale.offset(i_ar as isize) = (*q_dry_ener
                    .offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                    + temp_1 as core::ffi::c_int - 30 as core::ffi::c_int) as WORD16;
            }
            *scale.offset(i_ar as isize) = ixheaacd_mps_sqrt(
                *scale.offset(i_ar as isize),
                &mut *q_scale.offset(i_ar as isize),
                sqrt_tab,
            );
            ch = 0 as core::ffi::c_int as WORD32;
            while ch < 6 as core::ffi::c_int {
                if !(ch == 3 as core::ffi::c_int || ch == 0 as core::ffi::c_int) {
                    temp_1 = ixheaacd_mps_mult32_shr_15(
                        *scale.offset(ch as isize),
                        one_minus_damp,
                    );
                    *scale.offset(ch as isize) = ixheaacd_mps_add32(
                        temp_1,
                        damp,
                        &mut *q_scale.offset(ch as isize),
                        15 as WORD16,
                    );
                    *scale.offset(ch as isize) = ixheaacd_mps_convert_to_qn(
                        *scale.offset(ch as isize),
                        *q_scale.offset(ch as isize),
                        15 as WORD16,
                    );
                    if *scale.offset(ch as isize) > STP_SCALE_LIMIT_FIX {
                        *scale.offset(ch as isize) = STP_SCALE_LIMIT_FIX as WORD32;
                    }
                    if *scale.offset(ch as isize) < ONE_BY_STP_SCALE_LIMIT {
                        *scale.offset(ch as isize) = ONE_BY_STP_SCALE_LIMIT as WORD32;
                    }
                }
                ch += 1;
            }
        }
        _ => {}
    }
    ch = 0 as core::ffi::c_int as WORD32;
    while ch < num_output_channels {
        temp_1 = ixheaacd_mps_mult32_shr_15(
            STP_LPF_COEFF2_FIX,
            *scale.offset(ch as isize),
        );
        temp_2 = ixheaacd_mps_mult32_shr_15(
            ONE_MINUS_STP_LPF_COEFF2,
            *prev_tp_scale.offset(ch as isize),
        );
        *scale.offset(ch as isize) = ixheaac_add32_sat(temp_1, temp_2);
        *prev_tp_scale.offset(ch as isize) = *scale.offset(ch as isize);
        ch += 1;
    }
    offset = (ts as core::ffi::c_int * MAX_HYBRID_BANDS) as WORD32;
    p_buffer_real = ((*p_array_struct).buf_real)
        .offset(offset as isize)
        .offset(HYBRID_BAND_BORDER as isize);
    p_buffer_imag = ((*p_array_struct).buf_imag)
        .offset(offset as isize)
        .offset(HYBRID_BAND_BORDER as isize);
    p_buf_real = ((*p_array_struct).buffer_real)
        .offset(offset as isize)
        .offset(FIVE as isize);
    p_buf_imag = ((*p_array_struct).buffer_imag)
        .offset(offset as isize)
        .offset(FIVE as isize);
    p_hyb_out_dry_real = ((*p_array_struct).hyb_output_real_dry)
        .offset((ts as core::ffi::c_int * MAX_HYBRID_BANDS) as isize)
        .offset(HYBRID_BAND_BORDER as isize);
    p_hyb_out_dry_imag = ((*p_array_struct).hyb_output_imag_dry)
        .offset((ts as core::ffi::c_int * MAX_HYBRID_BANDS) as isize)
        .offset(HYBRID_BAND_BORDER as isize);
    ch = 0 as core::ffi::c_int as WORD32;
    while ch < num_output_channels {
        no_scaling = 1 as core::ffi::c_int as WORD32;
        ixheaacd_get_ch_idx(pstr_mps_state, ch, &mut i);
        if i != -(1 as core::ffi::c_int) {
            no_scaling = ((*(*pstr_mps_state).aux_struct)
                .temp_shape_enable_channel_stp[i as usize] == 0) as core::ffi::c_int
                as WORD32;
        }
        p_buffer_re = p_buffer_real;
        p_buffer_im = p_buffer_imag;
        p_buf_re = p_buf_real;
        p_buf_im = p_buf_imag;
        hyb_output_real_dry = p_hyb_out_dry_real;
        hyb_output_imag_dry = p_hyb_out_dry_imag;
        if no_scaling == 1 as core::ffi::c_int {
            n = HYBRID_BAND_BORDER as WORD32;
            while n < HP_SIZE + QMF_TO_HYB_OFFSET {
                let fresh130 = hyb_output_real_dry;
                hyb_output_real_dry = hyb_output_real_dry.offset(1);
                let fresh131 = p_buffer_re;
                p_buffer_re = p_buffer_re.offset(1);
                let fresh132 = p_buf_re;
                p_buf_re = p_buf_re.offset(1);
                *fresh132 = ixheaac_add32_sat(*fresh130, *fresh131);
                let fresh133 = hyb_output_imag_dry;
                hyb_output_imag_dry = hyb_output_imag_dry.offset(1);
                let fresh134 = p_buffer_im;
                p_buffer_im = p_buffer_im.offset(1);
                let fresh135 = p_buf_im;
                p_buf_im = p_buf_im.offset(1);
                *fresh135 = ixheaac_add32_sat(*fresh133, *fresh134);
                n += 1;
            }
            while n < hybrid_bands {
                temp = (if no_scaling != 0 {
                    ONE_IN_Q15
                } else {
                    *scale.offset(ch as isize)
                }) as WORD32;
                let fresh136 = hyb_output_real_dry;
                hyb_output_real_dry = hyb_output_real_dry.offset(1);
                let fresh137 = p_buffer_re;
                p_buffer_re = p_buffer_re.offset(1);
                let fresh138 = p_buf_re;
                p_buf_re = p_buf_re.offset(1);
                *fresh138 = ixheaac_add32_sat(*fresh136, *fresh137);
                let fresh139 = hyb_output_imag_dry;
                hyb_output_imag_dry = hyb_output_imag_dry.offset(1);
                let fresh140 = p_buffer_im;
                p_buffer_im = p_buffer_im.offset(1);
                let fresh141 = p_buf_im;
                p_buf_im = p_buf_im.offset(1);
                *fresh141 = ixheaac_add32_sat(*fresh139, *fresh140);
                n += 1;
                k += 1;
            }
        } else {
            n = HYBRID_BAND_BORDER as WORD32;
            while n < HP_SIZE + QMF_TO_HYB_OFFSET {
                temp = ixheaacd_mps_mult32_shr_30(
                    *scale.offset(ch as isize),
                    (*tp_process_table_ptr)
                        .bp[(n as core::ffi::c_int - QMF_TO_HYB_OFFSET) as usize],
                );
                let fresh142 = hyb_output_real_dry;
                hyb_output_real_dry = hyb_output_real_dry.offset(1);
                let fresh143 = p_buf_re;
                p_buf_re = p_buf_re.offset(1);
                *fresh143 = ixheaac_add32_sat(
                    *fresh142,
                    ixheaacd_mps_mult32_shr_15(temp, *p_buffer_re),
                );
                p_buffer_re = p_buffer_re.offset(1);
                let fresh144 = hyb_output_imag_dry;
                hyb_output_imag_dry = hyb_output_imag_dry.offset(1);
                let fresh145 = p_buf_im;
                p_buf_im = p_buf_im.offset(1);
                *fresh145 = ixheaac_add32_sat(
                    *fresh144,
                    ixheaacd_mps_mult32_shr_15(temp, *p_buffer_im),
                );
                p_buffer_im = p_buffer_im.offset(1);
                n += 1;
            }
            while n < hybrid_bands {
                temp = (if no_scaling != 0 {
                    ONE_IN_Q15
                } else {
                    *scale.offset(ch as isize)
                }) as WORD32;
                let fresh146 = hyb_output_real_dry;
                hyb_output_real_dry = hyb_output_real_dry.offset(1);
                let fresh147 = p_buf_re;
                p_buf_re = p_buf_re.offset(1);
                *fresh147 = ixheaac_add32_sat(
                    *fresh146,
                    ixheaacd_mps_mult32_shr_15(temp, *p_buffer_re),
                );
                p_buffer_re = p_buffer_re.offset(1);
                let fresh148 = hyb_output_imag_dry;
                hyb_output_imag_dry = hyb_output_imag_dry.offset(1);
                let fresh149 = p_buf_im;
                p_buf_im = p_buf_im.offset(1);
                *fresh149 = ixheaac_add32_sat(
                    *fresh148,
                    ixheaacd_mps_mult32_shr_15(temp, *p_buffer_im),
                );
                p_buffer_im = p_buffer_im.offset(1);
                n += 1;
                k += 1;
            }
        }
        p_buffer_real = p_buffer_real.offset(TSXHB as isize);
        p_buffer_imag = p_buffer_imag.offset(TSXHB as isize);
        p_buf_real = p_buf_real.offset(TSXHB as isize);
        p_buf_imag = p_buf_imag.offset(TSXHB as isize);
        p_hyb_out_dry_real = p_hyb_out_dry_real.offset(TSXHB as isize);
        p_hyb_out_dry_imag = p_hyb_out_dry_imag.offset(TSXHB as isize);
        ch += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_tp_process(
    mut pstr_mps_state: *mut ia_heaac_mps_state_struct,
) -> VOID {
    let mut ch: WORD32 = 0;
    let mut ts: WORD32 = 0;
    let mut hyb: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut temp_1: WORD32 = 0;
    let mut temp_2: WORD32 = 0;
    let mut syn: *mut ia_mps_dec_synthesis_interface = (*pstr_mps_state).syn;
    let mut hyb_output_real_wet: *mut WORD32 = 0 as *mut WORD32;
    let mut hyb_output_imag_wet: *mut WORD32 = 0 as *mut WORD32;
    let mut hyb_output_real_dry: *mut WORD32 = 0 as *mut WORD32;
    let mut hyb_output_imag_dry: *mut WORD32 = 0 as *mut WORD32;
    let mut p_buffer_real: *mut WORD32 = 0 as *mut WORD32;
    let mut p_buffer_imag: *mut WORD32 = 0 as *mut WORD32;
    let mut p_buffer_re: *mut WORD32 = 0 as *mut WORD32;
    let mut p_buffer_im: *mut WORD32 = 0 as *mut WORD32;
    let mut p_buf_real: *mut WORD32 = 0 as *mut WORD32;
    let mut p_buf_imag: *mut WORD32 = 0 as *mut WORD32;
    let mut p_buf_re: *mut WORD32 = 0 as *mut WORD32;
    let mut p_buf_im: *mut WORD32 = 0 as *mut WORD32;
    let mut buf_real: *mut WORD32 = 0 as *mut WORD32;
    let mut buf_imag: *mut WORD32 = 0 as *mut WORD32;
    let mut num_output_channels: WORD32 = (*pstr_mps_state).num_output_channels;
    let mut time_slots: WORD32 = (*pstr_mps_state).time_slots;
    let mut qmf_bands: WORD32 = (*pstr_mps_state).qmf_bands;
    let mut num_output_channels_at: WORD32 = (*pstr_mps_state).num_output_channels_at;
    let mut tree_config: WORD32 = (*pstr_mps_state).tree_config;
    let mut up_mix_type: WORD32 = (*pstr_mps_state).up_mix_type;
    let mut tp_hyb_band_border: WORD32 = (*pstr_mps_state).tp_hyb_band_border;
    let mut p_array_struct: *mut ia_mps_dec_reuse_array_struct = (*pstr_mps_state)
        .array_struct;
    let mut p_hyb_out_dry_real: *mut WORD32 = (*p_array_struct).hyb_output_real_dry;
    let mut p_hyb_out_dry_imag: *mut WORD32 = (*p_array_struct).hyb_output_imag_dry;
    let mut p_hyb_out_dry_re: *mut WORD32 = 0 as *mut WORD32;
    let mut p_hyb_out_dry_im: *mut WORD32 = 0 as *mut WORD32;
    let mut p_time_out: *mut WORD32 = 0 as *mut WORD32;
    p_buffer_real = (*p_array_struct).buf_real;
    p_buffer_imag = (*p_array_struct).buf_imag;
    p_buf_real = (*p_array_struct).buffer_real;
    p_buf_imag = (*p_array_struct).buffer_imag;
    if (*pstr_mps_state).scaling_enable == 0 {
        ch = 0 as core::ffi::c_int as WORD32;
        while ch < num_output_channels {
            p_buffer_re = p_buffer_real;
            p_buffer_im = p_buffer_imag;
            p_buf_re = p_buf_real;
            p_buf_im = p_buf_imag;
            p_hyb_out_dry_re = p_hyb_out_dry_real;
            p_hyb_out_dry_im = p_hyb_out_dry_imag;
            ts = 0 as core::ffi::c_int as WORD32;
            while ts < time_slots {
                hyb_output_real_wet = p_buffer_re;
                hyb_output_imag_wet = p_buffer_im;
                hyb_output_real_dry = p_hyb_out_dry_re;
                hyb_output_imag_dry = p_hyb_out_dry_im;
                buf_real = p_buf_re;
                buf_imag = p_buf_im;
                let fresh0 = hyb_output_real_dry;
                hyb_output_real_dry = hyb_output_real_dry.offset(1);
                let fresh1 = hyb_output_real_wet;
                hyb_output_real_wet = hyb_output_real_wet.offset(1);
                temp_1 = ixheaac_add32_sat(*fresh0, *fresh1);
                let fresh2 = hyb_output_imag_dry;
                hyb_output_imag_dry = hyb_output_imag_dry.offset(1);
                let fresh3 = hyb_output_imag_wet;
                hyb_output_imag_wet = hyb_output_imag_wet.offset(1);
                temp_2 = ixheaac_add32_sat(*fresh2, *fresh3);
                n = 1 as core::ffi::c_int as WORD32;
                while n < 6 as core::ffi::c_int {
                    let fresh4 = hyb_output_real_dry;
                    hyb_output_real_dry = hyb_output_real_dry.offset(1);
                    temp_1 = ixheaac_add32_sat(temp_1, *fresh4);
                    let fresh5 = hyb_output_real_wet;
                    hyb_output_real_wet = hyb_output_real_wet.offset(1);
                    temp_1 = ixheaac_add32_sat(temp_1, *fresh5);
                    let fresh6 = hyb_output_imag_dry;
                    hyb_output_imag_dry = hyb_output_imag_dry.offset(1);
                    temp_2 = ixheaac_add32_sat(temp_2, *fresh6);
                    let fresh7 = hyb_output_imag_wet;
                    hyb_output_imag_wet = hyb_output_imag_wet.offset(1);
                    temp_2 = ixheaac_add32_sat(temp_2, *fresh7);
                    n += 1;
                }
                let fresh8 = buf_real;
                buf_real = buf_real.offset(1);
                *fresh8 = temp_1;
                let fresh9 = buf_imag;
                buf_imag = buf_imag.offset(1);
                *fresh9 = temp_2;
                let fresh10 = hyb_output_real_dry;
                hyb_output_real_dry = hyb_output_real_dry.offset(1);
                let fresh11 = hyb_output_real_wet;
                hyb_output_real_wet = hyb_output_real_wet.offset(1);
                temp = ixheaac_add32_sat(*fresh10, *fresh11);
                let fresh12 = hyb_output_real_dry;
                hyb_output_real_dry = hyb_output_real_dry.offset(1);
                *buf_real = ixheaac_add32_sat(temp, *fresh12);
                let fresh13 = hyb_output_real_wet;
                hyb_output_real_wet = hyb_output_real_wet.offset(1);
                *buf_real = ixheaac_add32_sat(*buf_real, *fresh13);
                let fresh14 = hyb_output_imag_dry;
                hyb_output_imag_dry = hyb_output_imag_dry.offset(1);
                let fresh15 = hyb_output_imag_wet;
                hyb_output_imag_wet = hyb_output_imag_wet.offset(1);
                temp = ixheaac_add32_sat(*fresh14, *fresh15);
                let fresh16 = hyb_output_imag_dry;
                hyb_output_imag_dry = hyb_output_imag_dry.offset(1);
                *buf_imag = ixheaac_add32_sat(temp, *fresh16);
                let fresh17 = hyb_output_imag_wet;
                hyb_output_imag_wet = hyb_output_imag_wet.offset(1);
                *buf_imag = ixheaac_add32_sat(*buf_imag, *fresh17);
                buf_real = buf_real.offset(1);
                buf_imag = buf_imag.offset(1);
                let fresh18 = hyb_output_real_dry;
                hyb_output_real_dry = hyb_output_real_dry.offset(1);
                let fresh19 = hyb_output_real_wet;
                hyb_output_real_wet = hyb_output_real_wet.offset(1);
                temp = ixheaac_add32_sat(*fresh18, *fresh19);
                let fresh20 = hyb_output_real_dry;
                hyb_output_real_dry = hyb_output_real_dry.offset(1);
                *buf_real = ixheaac_add32_sat(temp, *fresh20);
                let fresh21 = hyb_output_real_wet;
                hyb_output_real_wet = hyb_output_real_wet.offset(1);
                *buf_real = ixheaac_add32_sat(*buf_real, *fresh21);
                let fresh22 = hyb_output_imag_dry;
                hyb_output_imag_dry = hyb_output_imag_dry.offset(1);
                let fresh23 = hyb_output_imag_wet;
                hyb_output_imag_wet = hyb_output_imag_wet.offset(1);
                temp = ixheaac_add32_sat(*fresh22, *fresh23);
                let fresh24 = hyb_output_imag_dry;
                hyb_output_imag_dry = hyb_output_imag_dry.offset(1);
                *buf_imag = ixheaac_add32_sat(temp, *fresh24);
                let fresh25 = hyb_output_imag_wet;
                hyb_output_imag_wet = hyb_output_imag_wet.offset(1);
                *buf_imag = ixheaac_add32_sat(*buf_imag, *fresh25);
                buf_real = buf_real.offset(1);
                buf_imag = buf_imag.offset(1);
                n = 0 as core::ffi::c_int as WORD32;
                while n < qmf_bands {
                    let fresh26 = hyb_output_real_dry;
                    hyb_output_real_dry = hyb_output_real_dry.offset(1);
                    let fresh27 = hyb_output_real_wet;
                    hyb_output_real_wet = hyb_output_real_wet.offset(1);
                    let fresh28 = buf_real;
                    buf_real = buf_real.offset(1);
                    *fresh28 = ixheaac_add32_sat(*fresh26, *fresh27);
                    let fresh29 = hyb_output_imag_dry;
                    hyb_output_imag_dry = hyb_output_imag_dry.offset(1);
                    let fresh30 = hyb_output_imag_wet;
                    hyb_output_imag_wet = hyb_output_imag_wet.offset(1);
                    let fresh31 = buf_imag;
                    buf_imag = buf_imag.offset(1);
                    *fresh31 = ixheaac_add32_sat(*fresh29, *fresh30);
                    n += 1;
                }
                p_buffer_re = p_buffer_re.offset(MAX_HYBRID_BANDS as isize);
                p_buffer_im = p_buffer_im.offset(MAX_HYBRID_BANDS as isize);
                p_buf_re = p_buf_re.offset(MAX_HYBRID_BANDS as isize);
                p_buf_im = p_buf_im.offset(MAX_HYBRID_BANDS as isize);
                p_hyb_out_dry_re = p_hyb_out_dry_re.offset(MAX_HYBRID_BANDS as isize);
                p_hyb_out_dry_im = p_hyb_out_dry_im.offset(MAX_HYBRID_BANDS as isize);
                ts += 1;
            }
            p_buffer_real = p_buffer_real.offset(TSXHB as isize);
            p_buffer_imag = p_buffer_imag.offset(TSXHB as isize);
            p_buf_real = p_buf_real.offset(TSXHB as isize);
            p_buf_imag = p_buf_imag.offset(TSXHB as isize);
            p_hyb_out_dry_real = p_hyb_out_dry_real.offset(TSXHB as isize);
            p_hyb_out_dry_imag = p_hyb_out_dry_imag.offset(TSXHB as isize);
            ch += 1;
        }
    } else {
        ch = 0 as core::ffi::c_int as WORD32;
        while ch < num_output_channels {
            p_buffer_re = p_buffer_real;
            p_buffer_im = p_buffer_imag;
            p_buf_re = p_buf_real;
            p_buf_im = p_buf_imag;
            p_hyb_out_dry_re = p_hyb_out_dry_real;
            p_hyb_out_dry_im = p_hyb_out_dry_imag;
            ts = 0 as core::ffi::c_int as WORD32;
            while ts < time_slots {
                hyb_output_real_wet = p_buffer_re;
                hyb_output_imag_wet = p_buffer_im;
                hyb_output_real_dry = p_hyb_out_dry_re;
                hyb_output_imag_dry = p_hyb_out_dry_im;
                buf_real = p_buf_re;
                buf_imag = p_buf_im;
                let fresh32 = hyb_output_real_dry;
                hyb_output_real_dry = hyb_output_real_dry.offset(1);
                let fresh33 = hyb_output_real_wet;
                hyb_output_real_wet = hyb_output_real_wet.offset(1);
                temp_1 = ixheaac_add32_sat(*fresh32, *fresh33);
                let fresh34 = hyb_output_imag_dry;
                hyb_output_imag_dry = hyb_output_imag_dry.offset(1);
                let fresh35 = hyb_output_imag_wet;
                hyb_output_imag_wet = hyb_output_imag_wet.offset(1);
                temp_2 = ixheaac_add32_sat(*fresh34, *fresh35);
                n = 1 as core::ffi::c_int as WORD32;
                while n < 6 as core::ffi::c_int {
                    let fresh36 = hyb_output_real_dry;
                    hyb_output_real_dry = hyb_output_real_dry.offset(1);
                    temp_1 = ixheaac_add32_sat(temp_1, *fresh36);
                    let fresh37 = hyb_output_real_wet;
                    hyb_output_real_wet = hyb_output_real_wet.offset(1);
                    temp_1 = ixheaac_add32_sat(temp_1, *fresh37);
                    let fresh38 = hyb_output_imag_dry;
                    hyb_output_imag_dry = hyb_output_imag_dry.offset(1);
                    temp_2 = ixheaac_add32_sat(temp_2, *fresh38);
                    let fresh39 = hyb_output_imag_wet;
                    hyb_output_imag_wet = hyb_output_imag_wet.offset(1);
                    temp_2 = ixheaac_add32_sat(temp_2, *fresh39);
                    n += 1;
                }
                let fresh40 = buf_real;
                buf_real = buf_real.offset(1);
                *fresh40 = temp_1;
                let fresh41 = buf_imag;
                buf_imag = buf_imag.offset(1);
                *fresh41 = temp_2;
                let fresh42 = hyb_output_real_dry;
                hyb_output_real_dry = hyb_output_real_dry.offset(1);
                let fresh43 = hyb_output_real_wet;
                hyb_output_real_wet = hyb_output_real_wet.offset(1);
                *buf_real = ixheaac_add32_sat(*fresh42, *fresh43);
                let fresh44 = hyb_output_real_dry;
                hyb_output_real_dry = hyb_output_real_dry.offset(1);
                *buf_real = ixheaac_add32_sat(*buf_real, *fresh44);
                let fresh45 = hyb_output_real_wet;
                hyb_output_real_wet = hyb_output_real_wet.offset(1);
                *buf_real = ixheaac_add32_sat(*buf_real, *fresh45);
                let fresh46 = hyb_output_imag_dry;
                hyb_output_imag_dry = hyb_output_imag_dry.offset(1);
                let fresh47 = hyb_output_imag_wet;
                hyb_output_imag_wet = hyb_output_imag_wet.offset(1);
                *buf_imag = ixheaac_add32_sat(*fresh46, *fresh47);
                let fresh48 = hyb_output_imag_dry;
                hyb_output_imag_dry = hyb_output_imag_dry.offset(1);
                *buf_imag = ixheaac_add32_sat(*buf_imag, *fresh48);
                let fresh49 = hyb_output_imag_wet;
                hyb_output_imag_wet = hyb_output_imag_wet.offset(1);
                *buf_imag = ixheaac_add32_sat(*buf_imag, *fresh49);
                buf_real = buf_real.offset(1);
                buf_imag = buf_imag.offset(1);
                let fresh50 = hyb_output_real_dry;
                hyb_output_real_dry = hyb_output_real_dry.offset(1);
                let fresh51 = hyb_output_real_wet;
                hyb_output_real_wet = hyb_output_real_wet.offset(1);
                *buf_real = ixheaac_add32_sat(*fresh50, *fresh51);
                let fresh52 = hyb_output_real_dry;
                hyb_output_real_dry = hyb_output_real_dry.offset(1);
                *buf_real = ixheaac_add32_sat(*buf_real, *fresh52);
                let fresh53 = hyb_output_real_wet;
                hyb_output_real_wet = hyb_output_real_wet.offset(1);
                *buf_real = ixheaac_add32_sat(*buf_real, *fresh53);
                let fresh54 = hyb_output_imag_dry;
                hyb_output_imag_dry = hyb_output_imag_dry.offset(1);
                let fresh55 = hyb_output_imag_wet;
                hyb_output_imag_wet = hyb_output_imag_wet.offset(1);
                *buf_imag = ixheaac_add32_sat(*fresh54, *fresh55);
                let fresh56 = hyb_output_imag_dry;
                hyb_output_imag_dry = hyb_output_imag_dry.offset(1);
                *buf_imag = ixheaac_add32_sat(*buf_imag, *fresh56);
                let fresh57 = hyb_output_imag_wet;
                hyb_output_imag_wet = hyb_output_imag_wet.offset(1);
                *buf_imag = ixheaac_add32_sat(*buf_imag, *fresh57);
                buf_real = buf_real.offset(1);
                buf_imag = buf_imag.offset(1);
                hyb = 3 as core::ffi::c_int as WORD32;
                while hyb < tp_hyb_band_border as core::ffi::c_int - QMF_TO_HYB_OFFSET {
                    let fresh58 = hyb_output_real_dry;
                    hyb_output_real_dry = hyb_output_real_dry.offset(1);
                    let fresh59 = hyb_output_real_wet;
                    hyb_output_real_wet = hyb_output_real_wet.offset(1);
                    let fresh60 = buf_real;
                    buf_real = buf_real.offset(1);
                    *fresh60 = ixheaac_add32_sat(*fresh58, *fresh59);
                    let fresh61 = hyb_output_imag_dry;
                    hyb_output_imag_dry = hyb_output_imag_dry.offset(1);
                    let fresh62 = hyb_output_imag_wet;
                    hyb_output_imag_wet = hyb_output_imag_wet.offset(1);
                    let fresh63 = buf_imag;
                    buf_imag = buf_imag.offset(1);
                    *fresh63 = ixheaac_add32_sat(*fresh61, *fresh62);
                    hyb += 1;
                }
                p_buffer_re = p_buffer_re.offset(MAX_HYBRID_BANDS as isize);
                p_buffer_im = p_buffer_im.offset(MAX_HYBRID_BANDS as isize);
                p_buf_re = p_buf_re.offset(MAX_HYBRID_BANDS as isize);
                p_buf_im = p_buf_im.offset(MAX_HYBRID_BANDS as isize);
                p_hyb_out_dry_re = p_hyb_out_dry_re.offset(MAX_HYBRID_BANDS as isize);
                p_hyb_out_dry_im = p_hyb_out_dry_im.offset(MAX_HYBRID_BANDS as isize);
                ts += 1;
            }
            p_buffer_real = p_buffer_real.offset(TSXHB as isize);
            p_buffer_imag = p_buffer_imag.offset(TSXHB as isize);
            p_buf_real = p_buf_real.offset(TSXHB as isize);
            p_buf_imag = p_buf_imag.offset(TSXHB as isize);
            p_hyb_out_dry_real = p_hyb_out_dry_real.offset(TSXHB as isize);
            p_hyb_out_dry_imag = p_hyb_out_dry_imag.offset(TSXHB as isize);
            ch += 1;
        }
        ts = 0 as core::ffi::c_int as WORD32;
        while ts < time_slots {
            ixheaacd_subband_tp(pstr_mps_state, ts);
            ts += 1;
        }
    }
    if (*pstr_mps_state).bs_config.arbitrary_tree == 0
        && (up_mix_type != 2 as core::ffi::c_int && up_mix_type != 3 as core::ffi::c_int)
    {
        let mut time_out_5xxx: *mut WORD32 = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .tp_process_table_ptr)
            .time_out_idx_5xxx)
            .as_mut_ptr();
        let mut time_out_7xxx: *mut WORD32 = ((*(*pstr_mps_state)
            .ia_mps_dec_mps_table
            .tp_process_table_ptr)
            .time_out_idx_7xxx)
            .as_mut_ptr();
        p_buf_real = (*p_array_struct).buffer_real;
        p_buf_imag = (*p_array_struct).buffer_imag;
        ch = 0 as core::ffi::c_int as WORD32;
        while ch < num_output_channels_at {
            let mut tempch: WORD32 = 0 as WORD32;
            match tree_config {
                0 => {
                    tempch = ch;
                }
                1 | 2 => {
                    tempch = *time_out_5xxx.offset(ch as isize);
                }
                3 | 4 | 5 | 6 => {
                    tempch = *time_out_7xxx.offset(ch as isize);
                }
                _ => {}
            }
            p_time_out = ((*p_array_struct).time_out)
                .offset((tempch as core::ffi::c_int * QBXTS) as isize);
            ((*syn).syn_filter_bank)
                .expect(
                    "non-null function pointer",
                )(
                &mut (*pstr_mps_state).syn_qmf_bank,
                p_buf_real,
                p_buf_imag,
                p_time_out,
                ch,
                qmf_bands,
                time_slots,
                (*pstr_mps_state).ia_mps_dec_mps_table.qmf_table_ptr,
            );
            p_buf_real = p_buf_real.offset(TSXHB as isize);
            p_buf_imag = p_buf_imag.offset(TSXHB as isize);
            ch += 1;
        }
    } else {
        p_time_out = (*p_array_struct).time_out;
        ch = 0 as core::ffi::c_int as WORD32;
        while ch < num_output_channels_at {
            ((*syn).syn_filter_bank)
                .expect(
                    "non-null function pointer",
                )(
                &mut (*pstr_mps_state).syn_qmf_bank,
                p_buf_real,
                p_buf_imag,
                p_time_out,
                ch,
                qmf_bands,
                time_slots,
                (*pstr_mps_state).ia_mps_dec_mps_table.qmf_table_ptr,
            );
            p_buf_real = p_buf_real.offset(TSXHB as isize);
            p_buf_imag = p_buf_imag.offset(TSXHB as isize);
            p_time_out = p_time_out.offset(QBXTS as isize);
            ch += 1;
        }
    };
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
unsafe extern "C" fn ixheaac_mult32_shl(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 32 as core::ffi::c_int) as WORD32;
    return result << 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn ixheaac_mult32x32in64(mut a: WORD32, mut b: WORD32) -> WORD64 {
    let mut result: WORD64 = 0;
    result = a as WORD64 * b as WORD64;
    return result;
}
#[inline]
unsafe extern "C" fn ixheaac_mac32x32in64(
    mut sum: WORD64,
    mut a: WORD32,
    mut b: WORD32,
) -> WORD64 {
    sum += a as WORD64 * b as WORD64;
    return sum;
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
pub const INV_SQRT_2_Q31: core::ffi::c_int = 1518500250 as core::ffi::c_int;
pub const Q_SQRT_TAB: core::ffi::c_int = 15 as core::ffi::c_int;
#[inline]
unsafe extern "C" fn ixheaacd_mps_get_rshift_bits(mut a: WORD64) -> WORD32 {
    let mut temp_1: WORD32 = 0;
    let mut temp_2: WORD32 = 0;
    temp_1 = (a >> 32 as core::ffi::c_int) as WORD32;
    temp_2 = ixheaac_norm32(temp_1) as WORD32;
    if temp_2 < 31 as core::ffi::c_int {
        return 32 as WORD32 - temp_2
    } else {
        temp_2 = a as WORD32;
        if temp_1 ^ temp_2 < 0 as core::ffi::c_int {
            return 1 as WORD32
        } else {
            return 0 as WORD32
        }
    };
}
#[inline]
unsafe extern "C" fn ixheaacd_mps_sqrt(
    mut num: WORD32,
    mut q: *mut WORD16,
    mut sqrt_tab: *const WORD32,
) -> WORD32 {
    let mut index: WORD32 = 0;
    let mut answer: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut k: WORD = 0;
    if num == 0 as core::ffi::c_int {
        return 0 as WORD32;
    }
    k = ixheaac_norm32(num);
    temp = ixheaac_shr32(ixheaac_shl32(num, k), 21 as WORD);
    *q = (*q as core::ffi::c_int + k as core::ffi::c_int) as WORD16;
    index = (temp as core::ffi::c_int & 0x1ff as core::ffi::c_int) as WORD32;
    answer = *sqrt_tab.offset(index as isize);
    if *q as core::ffi::c_int & 1 as core::ffi::c_int != 0 {
        *q = (*q as core::ffi::c_int - 1 as core::ffi::c_int) as WORD16;
        answer = ixheaac_mult32_shl(answer, INV_SQRT_2_Q31);
    }
    *q = (*q as core::ffi::c_int >> 1 as core::ffi::c_int) as WORD16;
    *q = (*q as core::ffi::c_int + Q_SQRT_TAB) as WORD16;
    return answer;
}
#[inline]
unsafe extern "C" fn ixheaacd_mps_add32(
    mut a: WORD32,
    mut b: WORD32,
    mut q_a: *mut WORD16,
    mut q_b: WORD16,
) -> WORD32 {
    let mut temp_result: WORD64 = 0;
    if a == 0 as core::ffi::c_int || b == 0 as core::ffi::c_int {
        if b == 0 as core::ffi::c_int {
            return a
        } else {
            *q_a = q_b;
            return b;
        }
    }
    if *q_a as core::ffi::c_int > q_b as core::ffi::c_int {
        if *q_a as core::ffi::c_int - q_b as core::ffi::c_int > 31 as core::ffi::c_int {
            a = 0 as core::ffi::c_int as WORD32;
            *q_a = q_b;
        } else {
            a = a >> *q_a as core::ffi::c_int - q_b as core::ffi::c_int;
            *q_a = q_b;
        }
    } else if q_b as core::ffi::c_int - *q_a as core::ffi::c_int > 31 as core::ffi::c_int
    {
        b = 0 as core::ffi::c_int as WORD32;
    } else {
        b = b >> q_b as core::ffi::c_int - *q_a as core::ffi::c_int;
        q_b = *q_a;
    }
    temp_result = a as WORD64 + b as WORD64;
    if temp_result > 0x7fffffff as core::ffi::c_int as WORD64
        || temp_result < 0x80000000 as core::ffi::c_uint as WORD32 as WORD64
    {
        temp_result = temp_result >> 1 as core::ffi::c_int;
        *q_a = (*q_a as core::ffi::c_int - 1 as core::ffi::c_int) as WORD16;
    }
    return temp_result as WORD32;
}
#[inline]
unsafe extern "C" fn ixheaacd_mps_mult32(
    mut a: WORD32,
    mut b: WORD32,
    mut q_a: *mut WORD16,
    mut q_b: WORD16,
) -> WORD32 {
    let mut temp_result: WORD64 = 0;
    let mut temp: WORD32 = 0;
    if a == 0 as core::ffi::c_int || b == 0 as core::ffi::c_int {
        temp_result = 0 as WORD64;
        *q_a = 15 as WORD16;
        return temp_result as WORD32;
    }
    *q_a = (*q_a as core::ffi::c_int + q_b as core::ffi::c_int) as WORD16;
    temp_result = a as WORD64 * b as WORD64;
    temp = ixheaacd_mps_get_rshift_bits(temp_result);
    if 0 as core::ffi::c_int != temp {
        *q_a = (*q_a as core::ffi::c_int - temp as core::ffi::c_int) as WORD16;
        temp_result = temp_result >> temp;
    }
    return temp_result as WORD32;
}
#[inline]
unsafe extern "C" fn ixheaacd_mps_mult32_shr_30(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 30 as core::ffi::c_int) as WORD32;
    return result;
}
#[inline]
unsafe extern "C" fn ixheaacd_mps_mult32_shr_15(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = ixheaac_sat64_32(temp_result >> 15 as core::ffi::c_int);
    return result;
}
#[inline]
unsafe extern "C" fn ixheaacd_mps_div_32(
    mut a: WORD32,
    mut b: WORD32,
    mut q_format: *mut WORD16,
) -> WORD32 {
    let mut quotient: WORD32 = 0;
    let mut mantissa_nr: UWORD32 = 0;
    let mut mantissa_dr: UWORD32 = 0;
    let mut i: LOOPINDEX = 0;
    let mut q_nr: WORD = 0;
    let mut q_dr: WORD = 0;
    quotient = 0 as core::ffi::c_int as WORD32;
    if 0 as core::ffi::c_int == b {
        *q_format = 0 as WORD16;
        return a;
    }
    quotient = 0 as core::ffi::c_int as WORD32;
    q_nr = ixheaac_norm32(a);
    mantissa_nr = (a as UWORD32) << q_nr;
    q_dr = ixheaac_norm32(b);
    mantissa_dr = (b as UWORD32) << q_dr;
    *q_format = (30 as WORD + q_nr - q_dr) as WORD16;
    i = 0 as core::ffi::c_int as LOOPINDEX;
    while i < 31 as core::ffi::c_int {
        quotient <<= 1 as core::ffi::c_int;
        if mantissa_nr >= mantissa_dr {
            mantissa_nr = mantissa_nr.wrapping_sub(mantissa_dr);
            quotient += 1 as core::ffi::c_int;
        }
        mantissa_nr <<= 1 as core::ffi::c_int;
        i += 1;
    }
    if a ^ b < 0 as core::ffi::c_int {
        return -quotient;
    }
    return quotient;
}
unsafe extern "C" fn ixheaacd_mps_convert_to_qn(
    mut temp: WORD32,
    mut qtemp: WORD16,
    mut n: WORD16,
) -> WORD32 {
    let mut result: WORD64 = 0;
    if qtemp as core::ffi::c_int == n as core::ffi::c_int {
        return temp
    } else if qtemp as core::ffi::c_int > n as core::ffi::c_int {
        temp = (temp as WORD64 >> qtemp as core::ffi::c_int - n as core::ffi::c_int)
            as WORD32;
    } else {
        result = ((temp as WORD64) << n as core::ffi::c_int - qtemp as core::ffi::c_int)
            as WORD32 as WORD64;
        if result > 0x7fffffff as core::ffi::c_int as WORD64
            || result < 0x80000000 as core::ffi::c_uint as WORD32 as WORD64
        {
            return 0 as WORD32
        } else {
            temp = result as WORD32;
        }
    }
    return temp;
}
pub const STP_LPF_COEFF1_FIX: core::ffi::c_int = 31130 as core::ffi::c_int;
pub const STP_LPF_COEFF2_FIX: core::ffi::c_int = 14746 as core::ffi::c_int;
pub const ONE_MINUS_STP_LPF_COEFF2: core::ffi::c_int = 18022 as core::ffi::c_int;
pub const STP_SCALE_LIMIT_FIX: core::ffi::c_int = 92406 as core::ffi::c_int;
pub const ONE_BY_STP_SCALE_LIMIT: core::ffi::c_int = 11620 as core::ffi::c_int;
pub const QMF_TO_HYB_OFFSET: core::ffi::c_int = 7 as core::ffi::c_int;
pub const DMX_OFFSET: core::ffi::c_int = 48 as core::ffi::c_int;
pub const DMX_OFFSET_MINUS_ONE: core::ffi::c_int = 47 as core::ffi::c_int;
pub const QMF_OUT_START_IDX: core::ffi::c_int = 5 as core::ffi::c_int;
pub const QMF_OUT_OFFSET: core::ffi::c_int = 20 as core::ffi::c_int;
pub const HYBRID_BAND_BORDER: core::ffi::c_int = 12 as core::ffi::c_int;
pub const FIVE: core::ffi::c_int = 5 as core::ffi::c_int;
pub const MAX_32: WORD32 = 0x7fffffff as core::ffi::c_long as WORD32;
pub const MIN_32: WORD32 = 0x80000000 as core::ffi::c_long as WORD32;
pub const ONE_IN_Q15: core::ffi::c_int = 32768 as core::ffi::c_int;
pub const POINT_ONE_Q15: core::ffi::c_int = 3277 as core::ffi::c_int;
pub const POINT_NINE_Q15: core::ffi::c_int = 29491 as core::ffi::c_int;
pub const QBXTS: core::ffi::c_int = 4608 as core::ffi::c_int;
pub const TSXHB: core::ffi::c_int = 5112 as core::ffi::c_int;
