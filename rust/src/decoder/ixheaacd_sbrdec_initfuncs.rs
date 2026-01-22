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
    fn ixheaacd_create_ps_esbr_dec(
        ptr_ps_dec_struct: *mut ia_ps_dec_struct,
        ptr_ps_tables: *mut ia_ps_tables_struct,
        noQmfChans: UWORD32,
        num_sub_samples: UWORD32,
        ps_mode: WORD32,
    ) -> WORD32;
    fn ixheaacd_calc_frq_bnd_tbls(
        ptr_header_data: *mut ia_sbr_header_data_struct,
        pstr_common_tables: *mut ixheaacd_misc_tables,
    ) -> WORD32;
    fn ixheaacd_reset_sbrenvelope_calc(
        ptr_calc_env: *mut ia_sbr_calc_env_struct,
    ) -> VOID;
    static ixheaacd_ldmps_polyphase_filter_coeff_fix: [WORD32; 1280];
}
pub type size_t = usize;
pub type WORD8 = core::ffi::c_schar;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD16 = core::ffi::c_short;
pub type UWORD16 = core::ffi::c_ushort;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type FLOAT32 = core::ffi::c_float;
pub type VOID = ();
pub type FLAG = core::ffi::c_int;
pub type WORD = core::ffi::c_int;
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
pub type C2RustUnnamed = core::ffi::c_uint;
pub const PVC_SBR: C2RustUnnamed = 2;
pub const ORIG_SBR: C2RustUnnamed = 1;
pub const UNKNOWN_SBR: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_dec_inst_struct {
    pub pstr_ps_stereo_dec: *mut ia_ps_dec_struct,
    pub str_ps_config_prev: ia_ps_dec_config_struct,
    pub ps_present: FLAG,
    pub pstr_sbr_channel: [*mut ia_sbr_channel_struct; 2],
    pub pstr_sbr_header: [*mut ia_sbr_header_data_struct; 2],
    pub pstr_freq_band_data: [*mut ia_freq_band_data_struct; 2],
    pub pstr_sbr_tables: *mut ia_sbr_tables_struct,
    pub pstr_common_tables: *mut ixheaacd_misc_tables,
    pub ptr_pvc_data_str: *mut ia_pvc_data_struct,
    pub hbe_txposer_buffers: *mut core::ffi::c_void,
    pub time_sample_buf: [*mut FLOAT32; 2],
    pub scratch_mem_v: *mut core::ffi::c_void,
    pub frame_buffer: [*mut core::ffi::c_void; 2],
    pub str_sbr_dflt_header: ia_sbr_header_data_struct,
    pub stereo_config_idx: FLAG,
    pub usac_independency_flag: FLAG,
    pub pvc_flag: FLAG,
    pub hbe_flag: FLAG,
    pub sbr_mode: FLAG,
    pub prev_sbr_mode: FLAG,
    pub inter_tes_flag: FLAG,
    pub aot_usac_flag: FLAG,
    pub band_count: [WORD32; 2],
    pub xaac_jmp_buf: *mut jmp_buf,
    pub ptr_mps_data: *mut WORD8,
    pub left_mps_bits: WORD32,
    pub mps_bits_pos: WORD32,
    pub esbr_hq: FLAG,
    pub enh_sbr: FLAG,
    pub enh_sbr_ps: FLAG,
    pub eld_sbr: FLAG,
    pub num_delay_frames: WORD32,
    pub sbr_parse_err_flag: FLAG,
    pub frame_ok: FLAG,
    pub ec_flag: FLAG,
    pub first_frame: FLAG,
    pub prev_usac_independency_flag: FLAG,
    pub sbr_parse_complete: FLAG,
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
pub struct ia_pvc_data_struct {
    pub pvc_mode: UWORD8,
    pub div_mode: UWORD8,
    pub ns_mode: UWORD8,
    pub num_time_slots: UWORD8,
    pub nb_high: UWORD8,
    pub nb_high_per_grp: UWORD8,
    pub prev_pvc_flg: UWORD8,
    pub prev_pvc_mode: UWORD8,
    pub pvc_rate: UWORD8,
    pub prev_pvc_rate: UWORD8,
    pub dummy: WORD16,
    pub pvc_id: [UWORD16; 16],
    pub prev_pvc_id: UWORD16,
    pub p_pred_coeff_tab_1: *mut WORD8,
    pub p_pred_coeff_tab_2: *mut WORD8,
    pub p_pvc_id_boundary: *mut UWORD8,
    pub prev_first_bnd_idx: WORD16,
    pub prev_sbr_mode: WORD32,
    pub p_smth_wind_coeff: *mut FLOAT32,
    pub p_q_fac: *mut FLOAT32,
    pub esg: [[FLOAT32; 3]; 31],
    pub smooth_esg_arr: [FLOAT32; 48],
    pub sbr_range_esg_arr: [FLOAT32; 128],
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
pub struct ia_sbr_channel_struct {
    pub pstr_prev_frame_data: *mut ia_sbr_prev_frame_data_struct,
    pub str_sbr_dec: ia_sbr_dec_struct,
    pub output_frame_size: WORD32,
    pub sync_state: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_dec_struct {
    pub ptr_sbr_overlap_buf: *mut WORD32,
    pub drc_factors_sbr: *mut *mut WORD32,
    pub str_codec_qmf_bank: ia_sbr_qmf_filter_bank_struct,
    pub str_synthesis_qmf_bank: ia_sbr_qmf_filter_bank_struct,
    pub str_sbr_calc_env: ia_sbr_calc_env_struct,
    pub str_hf_generator: ia_sbr_hf_generator_struct,
    pub str_sbr_scale_fact: ia_sbr_scale_fact_struct,
    pub max_samp_val: WORD32,
    pub band_count: WORD32,
    pub p_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
    pub core_sample_buf: [FLOAT32; 2624],
    pub core_sample_buf_sbr: [WORD16; 2624],
    pub ph_vocod_qmf_real: [[FLOAT32; 64]; 78],
    pub ph_vocod_qmf_imag: [[FLOAT32; 64]; 78],
    pub sbr_qmf_out_real: [[FLOAT32; 64]; 78],
    pub sbr_qmf_out_imag: [[FLOAT32; 64]; 78],
    pub qmf_buf_real: [[FLOAT32; 64]; 142],
    pub qmf_buf_imag: [[FLOAT32; 64]; 142],
    pub mps_qmf_buf_real: [[FLOAT32; 64]; 78],
    pub mps_qmf_buf_imag: [[FLOAT32; 64]; 78],
    pub mps_sbr_qmf_buf_real: [[FLOAT32; 64]; 78],
    pub mps_sbr_qmf_buf_imag: [[FLOAT32; 64]; 78],
    pub sbr_scratch_local: [WORD32; 256],
    pub scratch_buff: [FLOAT32; 320],
    pub qmf_energy_buf: [[FLOAT32; 32]; 64],
    pub pvc_qmf_enrg_arr: [FLOAT32; 512],
    pub pp_qmf_buf_real: *mut *mut FLOAT32,
    pub pp_qmf_buf_imag: *mut *mut FLOAT32,
    pub p_arr_qmf_buf_real: [*mut WORD32; 38],
    pub p_arr_qmf_buf_imag: [*mut WORD32; 38],
    pub time_sample_buf: *mut FLOAT32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_esbr_hbe_txposer_struct {
    pub x_over_qmf: [WORD32; 6],
    pub max_stretch: WORD32,
    pub core_frame_length: WORD32,
    pub hbe_qmf_in_len: WORD32,
    pub hbe_qmf_out_len: WORD32,
    pub no_bins: WORD32,
    pub start_band: WORD32,
    pub end_band: WORD32,
    pub upsamp_4_flag: WORD32,
    pub synth_buf_offset: WORD32,
    pub ptr_input_buf: *mut FLOAT32,
    pub qmf_in_buf: *mut *mut FLOAT32,
    pub qmf_out_buf: *mut *mut FLOAT32,
    pub k_start: WORD32,
    pub synth_size: WORD32,
    pub synth_buf: [FLOAT32; 1280],
    pub analy_buf: [FLOAT32; 640],
    pub synth_wind_coeff: *mut FLOAT32,
    pub analy_wind_coeff: *mut FLOAT32,
    pub synth_cos_tab: *mut FLOAT32,
    pub analy_cos_sin_tab: *mut FLOAT32,
    pub norm_qmf_in_buf: [[FLOAT32; 128]; 46],
    pub ixheaacd_real_synth_fft: Option<
        unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32) -> VOID,
    >,
    pub ixheaacd_cmplx_anal_fft: Option<
        unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32) -> VOID,
    >,
    pub esbr_hq: WORD32,
    pub in_hop_size: WORD32,
    pub fft_size: [WORD32; 2],
    pub anal_window: *mut FLOAT32,
    pub synth_window: *mut FLOAT32,
    pub ptr_spectrum: *mut FLOAT32,
    pub ptr_spectrum_tx: *mut FLOAT32,
    pub mag: *mut FLOAT32,
    pub phase: *mut FLOAT32,
    pub ptr_output_buf: *mut FLOAT32,
    pub ana_fft_size: [WORD32; 2],
    pub syn_fft_size: [WORD32; 2],
    pub out_hop_size: WORD32,
    pub analy_size: WORD32,
    pub x_over_bin: [[WORD32; 2]; 4],
    pub a_start: WORD32,
    pub spectrum_buf: [FLOAT32; 1536],
    pub spectrum_transposed_buf: [FLOAT32; 1536],
    pub mag_buf: [FLOAT32; 1536],
    pub phase_buf: [FLOAT32; 1536],
    pub output_buf: [FLOAT32; 4096],
    pub fd_win_buf: [[[FLOAT32; 1536]; 3]; 3],
    pub analysis_window_buf: [FLOAT32; 1024],
    pub synthesis_window_buf: [FLOAT32; 1024],
    pub oversampling_flag: WORD32,
    pub str_dft_hbe_anal_coeff: ia_dft_hbe_anal_coeff,
    pub ixheaacd_hbe_anal_fft: Option<
        unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32, WORD32) -> VOID,
    >,
    pub ixheaacd_hbe_synth_ifft: Option<
        unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32, WORD32) -> VOID,
    >,
    pub syn_cos_sin_tab: *mut FLOAT32,
    pub ana_cos_sin_tab: *mut FLOAT32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_dft_hbe_anal_coeff {
    pub real: [[FLOAT32; 128]; 64],
    pub imag: [[FLOAT32; 128]; 64],
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
pub struct ia_sbr_hf_generator_struct {
    pub pstr_settings: *mut ia_transposer_settings_struct,
    pub bw_array_prev: [WORD32; 6],
    pub lpc_filt_states_real: [*mut WORD32; 2],
    pub lpc_filt_states_imag: [*mut WORD32; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_transposer_settings_struct {
    pub num_columns: WORD16,
    pub num_patches: WORD16,
    pub start_patch: WORD16,
    pub stop_patch: WORD16,
    pub bw_borders: [WORD16; 10],
    pub str_patch_param: [ia_patch_param_struct; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_patch_param_struct {
    pub src_start_band: WORD16,
    pub src_end_band: WORD16,
    pub guard_start_band: WORD16,
    pub dst_start_band: WORD16,
    pub dst_end_band: WORD16,
    pub num_bands_in_patch: WORD16,
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
pub struct ia_ps_dec_config_struct {
    pub enable_iid: FLAG,
    pub enable_icc: FLAG,
    pub iid_mode: WORD16,
    pub icc_mode: WORD16,
    pub frame_class: FLAG,
    pub freq_res_ipd: WORD32,
    pub border_position: [WORD16; 7],
    pub iid_dt: [FLAG; 5],
    pub icc_dt: [FLAG; 5],
    pub iid_par_table: [[WORD16; 34]; 7],
    pub icc_par_table: [[WORD16; 34]; 7],
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
pub type ia_handle_sbr_dec_inst_struct = *mut ia_sbr_dec_inst_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_pers_struct {
    pub sbr_qmf_analy_states: *mut WORD16,
    pub sbr_qmf_analy_states_32: *mut WORD32,
    pub sbr_qmf_synth_states: *mut WORD16,
    pub sbr_qmf_synth_states_32: *mut WORD32,
    pub sbr_lpc_filter_states_real: [*mut *mut WORD32; 2],
    pub sbr_lpc_filter_states_imag: [*mut *mut WORD32; 2],
    pub ptr_sbr_overlap_buf: [*mut WORD32; 2],
    pub str_sbr_dec_inst: ia_sbr_dec_inst_struct,
    pub str_sbr_tran_settings: ia_transposer_settings_struct,
    pub sbr_smooth_gain_buf: [*mut WORD16; 2],
    pub sbr_smooth_noise_buf: [*mut WORD16; 2],
    pub pstr_prev_frame_data: [*mut ia_sbr_prev_frame_data_struct; 2],
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
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[inline]
unsafe extern "C" fn ixheaac_extract16l(mut var: WORD32) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = var as WORD16;
    return var_out;
}
pub const INT_BITS: core::ffi::c_int = 32 as core::ffi::c_int;
pub const LOW: core::ffi::c_int = 0 as core::ffi::c_int;
pub const HIGH: core::ffi::c_int = 1 as core::ffi::c_int;
pub const NO_QMF_SYNTH_CHANNELS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const TWICE_QMF_SYNTH_CHANNELS_NUM: core::ffi::c_int = 128 as core::ffi::c_int;
pub const MAX_QMF_BUF_LEN: core::ffi::c_int = 78 as core::ffi::c_int;
pub const MAXNRSBRCHANNELS: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SBR_UPSAMPLE_FAC: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SBR_UPSAMPLE_IDX_0_0: core::ffi::c_int = 0;
pub const SBR_UPSAMPLE_IDX_2_1: core::ffi::c_int = 1;
pub const SBR_UPSAMPLE_IDX_8_3: core::ffi::c_int = 2;
pub const SBR_UPSAMPLE_IDX_4_1: core::ffi::c_int = 3;
pub const MAX_FRAME_SIZE: core::ffi::c_int = 1024 as core::ffi::c_int;
pub const QMF_FILTER_STATE_SYN_SIZE: core::ffi::c_int = 1280 as core::ffi::c_int;
pub const QMF_FILTER_STATE_ANA_SIZE: core::ffi::c_int = 320 as core::ffi::c_int;
pub const NO_SYNTHESIS_CHANNELS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const NO_ANALYSIS_CHANNELS: core::ffi::c_int = NO_SYNTHESIS_CHANNELS
    / SBR_UPSAMPLE_FAC;
pub const NO_SYNTHESIS_CHANNELS_DOWN_SAMPLED: core::ffi::c_int = NO_SYNTHESIS_CHANNELS
    / 2 as core::ffi::c_int;
pub const QMF_FILTER_STATE_SYN_SIZE_DOWN_SAMPLED: core::ffi::c_int = QMF_FILTER_STATE_SYN_SIZE
    / 2 as core::ffi::c_int;
pub const MAX_NOISE_COEFFS: core::ffi::c_int = 5 as core::ffi::c_int;
pub const MAX_FREQ_COEFFS: core::ffi::c_int = 56 as core::ffi::c_int;
pub const LPC_ORDER: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_COLS: core::ffi::c_int = MAX_FRAME_SIZE / NO_ANALYSIS_CHANNELS;
pub const MAX_OV_COLS: core::ffi::c_int = 6 as core::ffi::c_int;
pub const MAX_ENV_COLS: core::ffi::c_int = MAX_COLS + MAX_OV_COLS;
pub const SBR_FREQ_SCALE_DEFAULT: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SBR_ALTER_SCALE_DEFAULT: core::ffi::c_int = 1 as core::ffi::c_int;
pub const SBR_NOISE_BANDS_DEFAULT: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SBR_LIMITER_BANDS_DEFAULT: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SBR_LIMITER_GAINS_DEFAULT: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SBR_INTERPOL_FREQ_DEFAULT: core::ffi::c_int = 1 as core::ffi::c_int;
pub const SBR_SMOOTHING_LENGTH_DEFAULT: core::ffi::c_int = 1 as core::ffi::c_int;
pub const NUM_SER_AP_LINKS: core::ffi::c_int = 3 as core::ffi::c_int;
pub const SUBQMF_GROUPS: core::ffi::c_int = 10 as core::ffi::c_int;
pub const QMF_GROUPS: core::ffi::c_int = 12 as core::ffi::c_int;
pub const NO_IID_GROUPS: core::ffi::c_int = SUBQMF_GROUPS + QMF_GROUPS;
pub const HYBRID_FILTER_LENGTH: core::ffi::c_int = 13 as core::ffi::c_int;
pub const NO_QMF_CHANNELS_IN_HYBRID: core::ffi::c_int = 3 as core::ffi::c_int;
pub const NO_HYBRID_CHANNELS_HIGH: core::ffi::c_int = 8 as core::ffi::c_int;
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
pub const NUM_SUB_SAMPLES_960: core::ffi::c_int = 30 as core::ffi::c_int;
pub const CORE_CODEC_FRAME_SIZE: core::ffi::c_int = 1024 as core::ffi::c_int;
pub const NUM_SUB_SAMPLES: core::ffi::c_int = CORE_CODEC_FRAME_SIZE
    / 32 as core::ffi::c_int;
pub const COUPLING_OFF: core::ffi::c_int = 0 as core::ffi::c_int;
pub const MAX_INVF_BANDS: core::ffi::c_int = MAX_NOISE_COEFFS;
pub const FD_OVERSAMPLING_FAC: core::ffi::c_float = 1.5f32;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_getsize_sbr_persistent() -> WORD32 {
    return ((::core::mem::size_of::<ia_sbr_pers_struct>() as usize)
        .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize) as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_esbr_hbe_data_init(
    mut pstr_esbr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
    num_aac_samples: WORD32,
    mut samp_fac_4_flag: WORD32,
    num_out_samples: WORD32,
    mut persistent_hbe_mem: *mut core::ffi::c_void,
    mut total_persistant: *mut WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut used_persistent: WORD32 = 0 as WORD32;
    if !pstr_esbr_hbe_txposer.is_null() {
        memset(
            pstr_esbr_hbe_txposer as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<ia_esbr_hbe_txposer_struct>() as size_t)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
        );
        (*pstr_esbr_hbe_txposer).core_frame_length = num_aac_samples;
        (*pstr_esbr_hbe_txposer).no_bins = (num_out_samples as core::ffi::c_int
            / NO_QMF_SYNTH_CHANNELS) as WORD32;
        (*pstr_esbr_hbe_txposer).hbe_qmf_in_len = (*pstr_esbr_hbe_txposer).no_bins;
        (*pstr_esbr_hbe_txposer).hbe_qmf_out_len = 2 as WORD32
            * (*pstr_esbr_hbe_txposer).hbe_qmf_in_len;
        (*pstr_esbr_hbe_txposer).ptr_input_buf = persistent_hbe_mem as *mut WORD8
            as *mut FLOAT32;
        used_persistent = (used_persistent as core::ffi::c_ulong)
            .wrapping_add(
                ((num_aac_samples as core::ffi::c_int + NO_QMF_SYNTH_CHANNELS) as usize)
                    .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize)
                    as core::ffi::c_ulong,
            ) as WORD32 as WORD32;
        (*pstr_esbr_hbe_txposer).qmf_in_buf = (persistent_hbe_mem as *mut WORD8)
            .offset(used_persistent as isize) as *mut *mut FLOAT32;
        used_persistent = (used_persistent as core::ffi::c_ulong)
            .wrapping_add(
                ((*pstr_esbr_hbe_txposer).hbe_qmf_in_len as usize)
                    .wrapping_mul(::core::mem::size_of::<*mut FLOAT32>() as usize)
                    as core::ffi::c_ulong,
            ) as WORD32 as WORD32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*pstr_esbr_hbe_txposer).hbe_qmf_in_len {
            let ref mut fresh6 = *((*pstr_esbr_hbe_txposer).qmf_in_buf)
                .offset(i as isize);
            *fresh6 = (persistent_hbe_mem as *mut WORD8).offset(used_persistent as isize)
                as *mut FLOAT32;
            used_persistent = (used_persistent as core::ffi::c_ulong)
                .wrapping_add(
                    (TWICE_QMF_SYNTH_CHANNELS_NUM as usize)
                        .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize)
                        as core::ffi::c_ulong,
                ) as WORD32 as WORD32;
            i += 1;
        }
        (*pstr_esbr_hbe_txposer).qmf_out_buf = (persistent_hbe_mem as *mut WORD8)
            .offset(used_persistent as isize) as *mut *mut FLOAT32;
        used_persistent = (used_persistent as core::ffi::c_ulong)
            .wrapping_add(
                ((*pstr_esbr_hbe_txposer).hbe_qmf_out_len as usize)
                    .wrapping_mul(::core::mem::size_of::<*mut FLOAT32>() as usize)
                    as core::ffi::c_ulong,
            ) as WORD32 as WORD32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*pstr_esbr_hbe_txposer).hbe_qmf_out_len {
            let ref mut fresh7 = *((*pstr_esbr_hbe_txposer).qmf_out_buf)
                .offset(i as isize);
            *fresh7 = (persistent_hbe_mem as *mut WORD8).offset(used_persistent as isize)
                as *mut FLOAT32;
            used_persistent = (used_persistent as core::ffi::c_ulong)
                .wrapping_add(
                    (TWICE_QMF_SYNTH_CHANNELS_NUM as usize)
                        .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize)
                        as core::ffi::c_ulong,
                ) as WORD32 as WORD32;
            i += 1;
        }
        (*pstr_esbr_hbe_txposer).upsamp_4_flag = samp_fac_4_flag;
        if !pstr_esbr_hbe_txposer.is_null() {
            (*pstr_esbr_hbe_txposer).fft_size[0 as core::ffi::c_int as usize] = num_aac_samples;
            (*pstr_esbr_hbe_txposer).fft_size[1 as core::ffi::c_int as usize] = (FD_OVERSAMPLING_FAC
                * num_aac_samples as core::ffi::c_float) as WORD32;
            (*pstr_esbr_hbe_txposer).ptr_spectrum = &mut *((*pstr_esbr_hbe_txposer)
                .spectrum_buf)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
            (*pstr_esbr_hbe_txposer).ptr_spectrum_tx = &mut *((*pstr_esbr_hbe_txposer)
                .spectrum_transposed_buf)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
            (*pstr_esbr_hbe_txposer).mag = &mut *((*pstr_esbr_hbe_txposer).mag_buf)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
            (*pstr_esbr_hbe_txposer).phase = &mut *((*pstr_esbr_hbe_txposer).phase_buf)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
            (*pstr_esbr_hbe_txposer).ptr_output_buf = &mut *((*pstr_esbr_hbe_txposer)
                .output_buf)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
        }
    }
    *total_persistant = used_persistent;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_set_sbr_persistent_table_pointer(
    mut sbr_persistent_mem_v: *mut core::ffi::c_void,
    mut sbr_tables_ptr: *mut ia_sbr_tables_struct,
    mut pstr_common_tables: *mut ixheaacd_misc_tables,
) -> VOID {
    let mut sbr_persistent_mem: *mut ia_sbr_pers_struct = sbr_persistent_mem_v
        as *mut ia_sbr_pers_struct;
    (*sbr_persistent_mem).str_sbr_dec_inst.pstr_sbr_tables = sbr_tables_ptr;
    (*sbr_persistent_mem).str_sbr_dec_inst.pstr_common_tables = pstr_common_tables;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_set_sbr_persistent_buffers(
    mut sbr_persistent_mem_v: *mut core::ffi::c_void,
    mut persistent_used: *mut WORD32,
    mut num_channel: WORD32,
    mut ps_enable: WORD,
) -> VOID {
    let mut i: WORD32 = 0 as WORD32;
    let mut used_persistent: WORD32 = *persistent_used;
    let mut temp: WORD32 = 0;
    let mut temp1: WORD32 = 0;
    let mut temp2: WORD32 = 0;
    let mut temp3: WORD32 = 0;
    let mut sbr_persistent_mem: *mut ia_sbr_pers_struct = sbr_persistent_mem_v
        as *mut ia_sbr_pers_struct;
    let mut p_str_sbr_dec_inst: *mut ia_sbr_dec_inst_struct = &mut (*sbr_persistent_mem)
        .str_sbr_dec_inst;
    num_channel = (if 2 as core::ffi::c_int > num_channel {
        2 as core::ffi::c_int
    } else {
        num_channel as core::ffi::c_int
    }) as WORD32;
    memset(
        sbr_persistent_mem as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<ia_sbr_pers_struct>() as size_t)
            .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t)
            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
    );
    (*sbr_persistent_mem).sbr_qmf_analy_states = (sbr_persistent_mem_v as *mut WORD8)
        .offset(used_persistent as isize) as *mut WORD16;
    temp = (num_channel as usize)
        .wrapping_mul(
            ((320 as core::ffi::c_int
                + 2 as core::ffi::c_int
                    * (64 as core::ffi::c_int / 2 as core::ffi::c_int)) as usize)
                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
        ) as WORD32;
    used_persistent += temp;
    (*sbr_persistent_mem).sbr_qmf_analy_states_32 = (sbr_persistent_mem_v as *mut WORD8)
        .offset(used_persistent as isize) as *mut WORD32;
    temp1 = (num_channel as usize)
        .wrapping_mul(
            ((320 as core::ffi::c_int
                + 2 as core::ffi::c_int
                    * (64 as core::ffi::c_int / 2 as core::ffi::c_int)) as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
        ) as WORD32;
    used_persistent += temp1;
    (*sbr_persistent_mem).sbr_qmf_synth_states = (sbr_persistent_mem_v as *mut WORD8)
        .offset(used_persistent as isize) as *mut WORD16;
    temp2 = (num_channel as usize)
        .wrapping_mul(
            ((1280 as core::ffi::c_int + 2 as core::ffi::c_int * 64 as core::ffi::c_int)
                as usize)
                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
        ) as WORD32;
    used_persistent += temp2;
    (*sbr_persistent_mem).sbr_qmf_synth_states_32 = (sbr_persistent_mem_v as *mut WORD8)
        .offset(used_persistent as isize) as *mut WORD32;
    temp3 = (num_channel as usize)
        .wrapping_mul(
            ((1280 as core::ffi::c_int + 2 as core::ffi::c_int * 64 as core::ffi::c_int)
                as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
        ) as WORD32;
    used_persistent += temp3;
    memset(
        (*sbr_persistent_mem).sbr_qmf_analy_states as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (temp + temp1 + temp2 + temp3) as size_t,
    );
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_channel {
        (*sbr_persistent_mem).ptr_sbr_overlap_buf[i as usize] = (sbr_persistent_mem_v
            as *mut WORD8)
            .offset(used_persistent as isize) as *mut WORD32;
        if ps_enable != 0 {
            memset(
                (*sbr_persistent_mem).ptr_sbr_overlap_buf[i as usize]
                    as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                (2 as size_t)
                    .wrapping_mul(
                        ((6 as core::ffi::c_int * 64 as core::ffi::c_int) as size_t)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                            )
                            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                    ),
            );
            used_persistent = (used_persistent as core::ffi::c_ulong)
                .wrapping_add(
                    (2 as usize)
                        .wrapping_mul(
                            ((6 as core::ffi::c_int * 64 as core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) as core::ffi::c_ulong,
                ) as WORD32 as WORD32;
        } else {
            memset(
                (*sbr_persistent_mem).ptr_sbr_overlap_buf[i as usize]
                    as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ((6 as core::ffi::c_int * 64 as core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
            );
            used_persistent = (used_persistent as core::ffi::c_ulong)
                .wrapping_add(
                    (((6 as core::ffi::c_int * 64 as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        as core::ffi::c_ulong,
                ) as WORD32 as WORD32;
        }
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_channel {
        let mut j: WORD32 = 0;
        (*sbr_persistent_mem).sbr_lpc_filter_states_real[i as usize] = (sbr_persistent_mem_v
            as *mut WORD8)
            .offset(used_persistent as isize) as *mut *mut WORD32;
        used_persistent = (used_persistent as core::ffi::c_ulong)
            .wrapping_add(
                ((2 as usize)
                    .wrapping_mul(::core::mem::size_of::<*mut WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    as core::ffi::c_ulong,
            ) as WORD32 as WORD32;
        j = 0 as core::ffi::c_int as WORD32;
        while j < LPC_ORDER {
            let ref mut fresh0 = *((*sbr_persistent_mem)
                .sbr_lpc_filter_states_real[i as usize])
                .offset(j as isize);
            *fresh0 = (sbr_persistent_mem_v as *mut WORD8)
                .offset(used_persistent as isize) as *mut WORD32;
            used_persistent = (used_persistent as core::ffi::c_ulong)
                .wrapping_add(
                    (((64 as core::ffi::c_int / 2 as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        as core::ffi::c_ulong,
                ) as WORD32 as WORD32;
            memset(
                *((*sbr_persistent_mem).sbr_lpc_filter_states_real[i as usize])
                    .offset(j as isize) as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ((64 as core::ffi::c_int / 2 as core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
            );
            j += 1;
        }
        i += 1;
    }
    if ps_enable != 0 {
        i = 0 as core::ffi::c_int as WORD32;
        while i < num_channel {
            let mut j_0: WORD32 = 0;
            (*sbr_persistent_mem).sbr_lpc_filter_states_imag[i as usize] = (sbr_persistent_mem_v
                as *mut WORD8)
                .offset(used_persistent as isize) as *mut *mut WORD32;
            used_persistent = (used_persistent as core::ffi::c_ulong)
                .wrapping_add(
                    ((2 as usize)
                        .wrapping_mul(::core::mem::size_of::<*mut WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        as core::ffi::c_ulong,
                ) as WORD32 as WORD32;
            j_0 = 0 as core::ffi::c_int as WORD32;
            while j_0 < LPC_ORDER {
                let ref mut fresh1 = *((*sbr_persistent_mem)
                    .sbr_lpc_filter_states_imag[i as usize])
                    .offset(j_0 as isize);
                *fresh1 = (sbr_persistent_mem_v as *mut WORD8)
                    .offset(used_persistent as isize) as *mut WORD32;
                used_persistent = (used_persistent as core::ffi::c_ulong)
                    .wrapping_add(
                        (((64 as core::ffi::c_int / 2 as core::ffi::c_int) as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            )
                            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                            as core::ffi::c_ulong,
                    ) as WORD32 as WORD32;
                memset(
                    *((*sbr_persistent_mem).sbr_lpc_filter_states_imag[i as usize])
                        .offset(j_0 as isize) as *mut core::ffi::c_void,
                    0 as core::ffi::c_int,
                    ((64 as core::ffi::c_int / 2 as core::ffi::c_int) as size_t)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                );
                j_0 += 1;
            }
            i += 1;
        }
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_channel {
        let mut initial_used: WORD32 = used_persistent;
        let mut temp_used: WORD32 = used_persistent;
        (*sbr_persistent_mem).sbr_smooth_gain_buf[i as usize] = (sbr_persistent_mem_v
            as *mut WORD8)
            .offset(temp_used as isize) as *mut WORD16;
        temp_used = (temp_used as core::ffi::c_ulong)
            .wrapping_add(
                (2 as usize)
                    .wrapping_mul(
                        (56 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) as core::ffi::c_ulong,
            ) as WORD32 as WORD32;
        (*sbr_persistent_mem).sbr_smooth_noise_buf[i as usize] = (sbr_persistent_mem_v
            as *mut WORD8)
            .offset(temp_used as isize) as *mut WORD16;
        temp_used = (temp_used as core::ffi::c_ulong)
            .wrapping_add(
                ((56 as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    as core::ffi::c_ulong,
            ) as WORD32 as WORD32;
        (*p_str_sbr_dec_inst).pstr_freq_band_data[i as usize] = (sbr_persistent_mem_v
            as *mut WORD8)
            .offset(temp_used as isize) as *mut core::ffi::c_void
            as *mut ia_freq_band_data_struct;
        temp_used = (temp_used as core::ffi::c_ulong)
            .wrapping_add(
                ((::core::mem::size_of::<ia_freq_band_data_struct>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    as core::ffi::c_ulong,
            ) as WORD32 as WORD32;
        (*sbr_persistent_mem).pstr_prev_frame_data[i as usize] = (sbr_persistent_mem_v
            as *mut WORD8)
            .offset(temp_used as isize) as *mut core::ffi::c_void
            as *mut ia_sbr_prev_frame_data_struct;
        temp_used = (temp_used as core::ffi::c_ulong)
            .wrapping_add(
                ((::core::mem::size_of::<ia_sbr_prev_frame_data_struct>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    as core::ffi::c_ulong,
            ) as WORD32 as WORD32;
        (*p_str_sbr_dec_inst).pstr_sbr_channel[i as usize] = (sbr_persistent_mem_v
            as *mut WORD8)
            .offset(temp_used as isize) as *mut core::ffi::c_void
            as *mut ia_sbr_channel_struct;
        temp_used = (temp_used as core::ffi::c_ulong)
            .wrapping_add(
                ((::core::mem::size_of::<ia_sbr_channel_struct>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    as core::ffi::c_ulong,
            ) as WORD32 as WORD32;
        (*p_str_sbr_dec_inst).pstr_sbr_header[i as usize] = (sbr_persistent_mem_v
            as *mut WORD8)
            .offset(temp_used as isize) as *mut core::ffi::c_void
            as *mut ia_sbr_header_data_struct;
        temp_used = (temp_used as core::ffi::c_ulong)
            .wrapping_add(
                ((::core::mem::size_of::<ia_sbr_header_data_struct>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    as core::ffi::c_ulong,
            ) as WORD32 as WORD32;
        memset(
            (*sbr_persistent_mem).sbr_smooth_gain_buf[i as usize]
                as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (temp_used - initial_used) as size_t,
        );
        used_persistent = temp_used;
        i += 1;
    }
    if ps_enable != 0 {
        (*p_str_sbr_dec_inst).pstr_ps_stereo_dec = (sbr_persistent_mem_v as *mut WORD8)
            .offset(used_persistent as isize) as *mut ia_ps_dec_struct;
        memset(
            (*p_str_sbr_dec_inst).pstr_ps_stereo_dec as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<ia_ps_dec_struct>() as size_t)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
        );
        used_persistent = (used_persistent as core::ffi::c_ulong)
            .wrapping_add(
                ((::core::mem::size_of::<ia_ps_dec_struct>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    as core::ffi::c_ulong,
            ) as WORD32 as WORD32;
    }
    (*p_str_sbr_dec_inst).frame_buffer[0 as core::ffi::c_int as usize] = (sbr_persistent_mem_v
        as *mut WORD8)
        .offset(used_persistent as isize) as *mut core::ffi::c_void;
    memset(
        (*p_str_sbr_dec_inst).frame_buffer[0 as core::ffi::c_int as usize],
        0 as core::ffi::c_int,
        (::core::mem::size_of::<ia_sbr_frame_info_data_struct>() as size_t)
            .wrapping_add(
                (56 as size_t)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t)
                    .wrapping_mul(2 as size_t),
            )
            .wrapping_add(8 as size_t)
            .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t)
            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
    );
    used_persistent = (used_persistent as usize)
        .wrapping_add(
            (::core::mem::size_of::<ia_sbr_frame_info_data_struct>() as usize)
                .wrapping_add(
                    (56 as usize).wrapping_mul(::core::mem::size_of::<WORD32>() as usize),
                )
                .wrapping_add(8 as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
        ) as WORD32;
    (*p_str_sbr_dec_inst).frame_buffer[1 as core::ffi::c_int as usize] = (sbr_persistent_mem_v
        as *mut WORD8)
        .offset(used_persistent as isize) as *mut core::ffi::c_void;
    memset(
        (*p_str_sbr_dec_inst).frame_buffer[1 as core::ffi::c_int as usize],
        0 as core::ffi::c_int,
        (::core::mem::size_of::<ia_sbr_frame_info_data_struct>() as size_t)
            .wrapping_add(
                (56 as size_t)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t)
                    .wrapping_mul(2 as size_t),
            )
            .wrapping_add(8 as size_t)
            .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t)
            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
    );
    used_persistent = (used_persistent as usize)
        .wrapping_add(
            (::core::mem::size_of::<ia_sbr_frame_info_data_struct>() as usize)
                .wrapping_add(
                    (56 as usize).wrapping_mul(::core::mem::size_of::<WORD32>() as usize),
                )
                .wrapping_add(8 as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
        ) as WORD32;
    let mut index: WORD32 = 0 as WORD32;
    (*p_str_sbr_dec_inst).ptr_pvc_data_str = (sbr_persistent_mem_v as *mut WORD8)
        .offset(used_persistent as isize) as *mut ia_pvc_data_struct;
    memset(
        (*p_str_sbr_dec_inst).ptr_pvc_data_str as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<ia_pvc_data_struct>() as size_t)
            .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t)
            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
    );
    used_persistent = (used_persistent as core::ffi::c_ulong)
        .wrapping_add(
            ((::core::mem::size_of::<ia_pvc_data_struct>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    (*(*p_str_sbr_dec_inst).pstr_sbr_channel[0 as core::ffi::c_int as usize])
        .str_sbr_dec
        .p_hbe_txposer = (sbr_persistent_mem_v as *mut WORD8)
        .offset(used_persistent as isize) as *mut ia_esbr_hbe_txposer_struct;
    memset(
        (*(*p_str_sbr_dec_inst).pstr_sbr_channel[0 as core::ffi::c_int as usize])
            .str_sbr_dec
            .p_hbe_txposer as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<ia_esbr_hbe_txposer_struct>() as size_t)
            .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t)
            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
    );
    used_persistent = (used_persistent as core::ffi::c_ulong)
        .wrapping_add(
            ((::core::mem::size_of::<ia_esbr_hbe_txposer_struct>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    if num_channel == 2 as core::ffi::c_int {
        (*(*p_str_sbr_dec_inst).pstr_sbr_channel[1 as core::ffi::c_int as usize])
            .str_sbr_dec
            .p_hbe_txposer = (sbr_persistent_mem_v as *mut WORD8)
            .offset(used_persistent as isize) as *mut ia_esbr_hbe_txposer_struct;
        memset(
            (*(*p_str_sbr_dec_inst).pstr_sbr_channel[1 as core::ffi::c_int as usize])
                .str_sbr_dec
                .p_hbe_txposer as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<ia_esbr_hbe_txposer_struct>() as size_t)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
        );
        used_persistent = (used_persistent as core::ffi::c_ulong)
            .wrapping_add(
                ((::core::mem::size_of::<ia_esbr_hbe_txposer_struct>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    as core::ffi::c_ulong,
            ) as WORD32 as WORD32;
    }
    (*p_str_sbr_dec_inst).hbe_txposer_buffers = (sbr_persistent_mem_v as *mut WORD8)
        .offset(used_persistent as isize) as *mut core::ffi::c_void;
    memset(
        (*p_str_sbr_dec_inst).hbe_txposer_buffers as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (num_channel as size_t)
            .wrapping_mul(
                (64 as size_t)
                    .wrapping_mul(::core::mem::size_of::<*mut FLOAT32>() as size_t)
                    .wrapping_add(
                        ((2 as core::ffi::c_int * 64 as core::ffi::c_int) as size_t)
                            .wrapping_mul(
                                ::core::mem::size_of::<*mut FLOAT32>() as size_t,
                            ),
                    )
                    .wrapping_add(
                        ((64 as core::ffi::c_int * 64 as core::ffi::c_int) as size_t)
                            .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                    )
                    .wrapping_add(
                        ((64 as core::ffi::c_int * 64 as core::ffi::c_int) as size_t)
                            .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                    )
                    .wrapping_add(
                        ((64 as core::ffi::c_int * 2 as core::ffi::c_int
                            * 64 as core::ffi::c_int) as size_t)
                            .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                    )
                    .wrapping_add(
                        ((64 as core::ffi::c_int * 2 as core::ffi::c_int
                            * 64 as core::ffi::c_int) as size_t)
                            .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                    )
                    .wrapping_add(
                        ((1024 as core::ffi::c_int + 64 as core::ffi::c_int) as size_t)
                            .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                    )
                    .wrapping_add(
                        (((1024 as core::ffi::c_int + 64 as core::ffi::c_int)
                            * 2 as core::ffi::c_int) as size_t)
                            .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                    )
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
            ),
    );
    used_persistent = (used_persistent as core::ffi::c_ulong)
        .wrapping_add(
            (num_channel as usize)
                .wrapping_mul(
                    (64 as usize)
                        .wrapping_mul(::core::mem::size_of::<*mut FLOAT32>() as usize)
                        .wrapping_add(
                            ((2 as core::ffi::c_int * 64 as core::ffi::c_int) as usize)
                                .wrapping_mul(
                                    ::core::mem::size_of::<*mut FLOAT32>() as usize,
                                ),
                        )
                        .wrapping_add(
                            ((64 as core::ffi::c_int * 64 as core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize),
                        )
                        .wrapping_add(
                            ((64 as core::ffi::c_int * 64 as core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize),
                        )
                        .wrapping_add(
                            ((64 as core::ffi::c_int * 2 as core::ffi::c_int
                                * 64 as core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize),
                        )
                        .wrapping_add(
                            ((64 as core::ffi::c_int * 2 as core::ffi::c_int
                                * 64 as core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize),
                        )
                        .wrapping_add(
                            ((1024 as core::ffi::c_int + 64 as core::ffi::c_int)
                                as usize)
                                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize),
                        )
                        .wrapping_add(
                            (((1024 as core::ffi::c_int + 64 as core::ffi::c_int)
                                * 2 as core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize),
                        )
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    (*(*p_str_sbr_dec_inst).pstr_sbr_channel[0 as core::ffi::c_int as usize])
        .str_sbr_dec
        .pp_qmf_buf_real = (sbr_persistent_mem_v as *mut WORD8)
        .offset(used_persistent as isize) as *mut *mut FLOAT32;
    memset(
        (*(*p_str_sbr_dec_inst).pstr_sbr_channel[0 as core::ffi::c_int as usize])
            .str_sbr_dec
            .pp_qmf_buf_real as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (78 as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut FLOAT32>() as size_t)
            .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t)
            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
    );
    used_persistent = (used_persistent as core::ffi::c_ulong)
        .wrapping_add(
            ((78 as usize)
                .wrapping_mul(::core::mem::size_of::<*mut FLOAT32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    (*(*p_str_sbr_dec_inst).pstr_sbr_channel[0 as core::ffi::c_int as usize])
        .str_sbr_dec
        .pp_qmf_buf_imag = (sbr_persistent_mem_v as *mut WORD8)
        .offset(used_persistent as isize) as *mut *mut FLOAT32;
    memset(
        (*(*p_str_sbr_dec_inst).pstr_sbr_channel[0 as core::ffi::c_int as usize])
            .str_sbr_dec
            .pp_qmf_buf_imag as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (78 as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut FLOAT32>() as size_t)
            .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t)
            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
    );
    used_persistent = (used_persistent as core::ffi::c_ulong)
        .wrapping_add(
            ((78 as usize)
                .wrapping_mul(::core::mem::size_of::<*mut FLOAT32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    if num_channel == 2 as core::ffi::c_int {
        (*(*p_str_sbr_dec_inst).pstr_sbr_channel[1 as core::ffi::c_int as usize])
            .str_sbr_dec
            .pp_qmf_buf_real = (sbr_persistent_mem_v as *mut WORD8)
            .offset(used_persistent as isize) as *mut *mut FLOAT32;
        memset(
            (*(*p_str_sbr_dec_inst).pstr_sbr_channel[1 as core::ffi::c_int as usize])
                .str_sbr_dec
                .pp_qmf_buf_real as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (78 as size_t)
                .wrapping_mul(::core::mem::size_of::<*mut FLOAT32>() as size_t)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
        );
        used_persistent = (used_persistent as core::ffi::c_ulong)
            .wrapping_add(
                ((78 as usize)
                    .wrapping_mul(::core::mem::size_of::<*mut FLOAT32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    as core::ffi::c_ulong,
            ) as WORD32 as WORD32;
        (*(*p_str_sbr_dec_inst).pstr_sbr_channel[1 as core::ffi::c_int as usize])
            .str_sbr_dec
            .pp_qmf_buf_imag = (sbr_persistent_mem_v as *mut WORD8)
            .offset(used_persistent as isize) as *mut *mut FLOAT32;
        memset(
            (*(*p_str_sbr_dec_inst).pstr_sbr_channel[1 as core::ffi::c_int as usize])
                .str_sbr_dec
                .pp_qmf_buf_imag as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (78 as size_t)
                .wrapping_mul(::core::mem::size_of::<*mut FLOAT32>() as size_t)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
        );
        used_persistent = (used_persistent as core::ffi::c_ulong)
            .wrapping_add(
                ((78 as usize)
                    .wrapping_mul(::core::mem::size_of::<*mut FLOAT32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    as core::ffi::c_ulong,
            ) as WORD32 as WORD32;
    }
    index = 0 as core::ffi::c_int as WORD32;
    while index < MAX_QMF_BUF_LEN {
        let ref mut fresh2 = *((*(*p_str_sbr_dec_inst)
            .pstr_sbr_channel[0 as core::ffi::c_int as usize])
            .str_sbr_dec
            .pp_qmf_buf_real)
            .offset(index as isize);
        *fresh2 = (sbr_persistent_mem_v as *mut WORD8).offset(used_persistent as isize)
            as *mut FLOAT32;
        used_persistent = (used_persistent as core::ffi::c_ulong)
            .wrapping_add(
                ((78 as usize)
                    .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    as core::ffi::c_ulong,
            ) as WORD32 as WORD32;
        index += 1;
    }
    index = 0 as core::ffi::c_int as WORD32;
    while index < MAX_QMF_BUF_LEN {
        let ref mut fresh3 = *((*(*p_str_sbr_dec_inst)
            .pstr_sbr_channel[0 as core::ffi::c_int as usize])
            .str_sbr_dec
            .pp_qmf_buf_imag)
            .offset(index as isize);
        *fresh3 = (sbr_persistent_mem_v as *mut WORD8).offset(used_persistent as isize)
            as *mut FLOAT32;
        used_persistent = (used_persistent as core::ffi::c_ulong)
            .wrapping_add(
                ((78 as usize)
                    .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    as core::ffi::c_ulong,
            ) as WORD32 as WORD32;
        index += 1;
    }
    if num_channel == 2 as core::ffi::c_int {
        index = 0 as core::ffi::c_int as WORD32;
        while index < MAX_QMF_BUF_LEN {
            let ref mut fresh4 = *((*(*p_str_sbr_dec_inst)
                .pstr_sbr_channel[1 as core::ffi::c_int as usize])
                .str_sbr_dec
                .pp_qmf_buf_real)
                .offset(index as isize);
            *fresh4 = (sbr_persistent_mem_v as *mut WORD8)
                .offset(used_persistent as isize) as *mut FLOAT32;
            used_persistent = (used_persistent as core::ffi::c_ulong)
                .wrapping_add(
                    ((78 as usize)
                        .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        as core::ffi::c_ulong,
                ) as WORD32 as WORD32;
            index += 1;
        }
        index = 0 as core::ffi::c_int as WORD32;
        while index < MAX_QMF_BUF_LEN {
            let ref mut fresh5 = *((*(*p_str_sbr_dec_inst)
                .pstr_sbr_channel[1 as core::ffi::c_int as usize])
                .str_sbr_dec
                .pp_qmf_buf_imag)
                .offset(index as isize);
            *fresh5 = (sbr_persistent_mem_v as *mut WORD8)
                .offset(used_persistent as isize) as *mut FLOAT32;
            used_persistent = (used_persistent as core::ffi::c_ulong)
                .wrapping_add(
                    ((78 as usize)
                        .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        as core::ffi::c_ulong,
                ) as WORD32 as WORD32;
            index += 1;
        }
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < MAX_ENV_COLS {
        (*(*p_str_sbr_dec_inst).pstr_sbr_channel[0 as core::ffi::c_int as usize])
            .str_sbr_dec
            .p_arr_qmf_buf_real[i as usize] = (sbr_persistent_mem_v as *mut WORD8)
            .offset(used_persistent as isize) as *mut WORD32;
        memset(
            (*(*p_str_sbr_dec_inst).pstr_sbr_channel[0 as core::ffi::c_int as usize])
                .str_sbr_dec
                .p_arr_qmf_buf_real[i as usize] as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (78 as size_t)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
        );
        used_persistent = (used_persistent as core::ffi::c_ulong)
            .wrapping_add(
                ((78 as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    as core::ffi::c_ulong,
            ) as WORD32 as WORD32;
        (*(*p_str_sbr_dec_inst).pstr_sbr_channel[0 as core::ffi::c_int as usize])
            .str_sbr_dec
            .p_arr_qmf_buf_imag[i as usize] = (sbr_persistent_mem_v as *mut WORD8)
            .offset(used_persistent as isize) as *mut WORD32;
        memset(
            (*(*p_str_sbr_dec_inst).pstr_sbr_channel[0 as core::ffi::c_int as usize])
                .str_sbr_dec
                .p_arr_qmf_buf_imag[i as usize] as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (78 as size_t)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
        );
        used_persistent = (used_persistent as core::ffi::c_ulong)
            .wrapping_add(
                ((78 as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    as core::ffi::c_ulong,
            ) as WORD32 as WORD32;
        i += 1;
    }
    if num_channel == 2 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < MAX_ENV_COLS {
            (*(*p_str_sbr_dec_inst).pstr_sbr_channel[1 as core::ffi::c_int as usize])
                .str_sbr_dec
                .p_arr_qmf_buf_real[i as usize] = (sbr_persistent_mem_v as *mut WORD8)
                .offset(used_persistent as isize) as *mut WORD32;
            memset(
                (*(*p_str_sbr_dec_inst).pstr_sbr_channel[1 as core::ffi::c_int as usize])
                    .str_sbr_dec
                    .p_arr_qmf_buf_real[i as usize] as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                (78 as size_t)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
            );
            used_persistent = (used_persistent as core::ffi::c_ulong)
                .wrapping_add(
                    ((78 as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        as core::ffi::c_ulong,
                ) as WORD32 as WORD32;
            (*(*p_str_sbr_dec_inst).pstr_sbr_channel[1 as core::ffi::c_int as usize])
                .str_sbr_dec
                .p_arr_qmf_buf_imag[i as usize] = (sbr_persistent_mem_v as *mut WORD8)
                .offset(used_persistent as isize) as *mut WORD32;
            memset(
                (*(*p_str_sbr_dec_inst).pstr_sbr_channel[1 as core::ffi::c_int as usize])
                    .str_sbr_dec
                    .p_arr_qmf_buf_imag[i as usize] as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                (78 as size_t)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
            );
            used_persistent = (used_persistent as core::ffi::c_ulong)
                .wrapping_add(
                    ((78 as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        as core::ffi::c_ulong,
                ) as WORD32 as WORD32;
            i += 1;
        }
    }
    *persistent_used = used_persistent;
}
#[inline]
unsafe extern "C" fn ixheaacd_init_headerdata(
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut sample_rate_dec: WORD32,
    mut samp_per_frame: WORD32,
    mut freq_band_data: *mut ia_freq_band_data_struct,
    mut sbr_tables: *mut ia_sbr_tables_struct,
    mut audio_obj_type: WORD,
) -> VOID {
    let mut pstr_freq_band_data: *mut ia_freq_band_data_struct = freq_band_data;
    let mut tmp: WORD32 = 0;
    if audio_obj_type != AOT_ER_AAC_ELD as core::ffi::c_int {
        memcpy(
            ptr_header_data as *mut core::ffi::c_void,
            &mut (*(*sbr_tables).env_extr_tables_ptr).str_sbr_default_header
                as *mut ia_sbr_header_data_struct as *const core::ffi::c_void,
            ::core::mem::size_of::<ia_sbr_header_data_struct>() as size_t,
        );
    }
    if audio_obj_type == AOT_ER_AAC_ELD as core::ffi::c_int {
        (*ptr_header_data).time_step = ((*ptr_header_data).time_step as core::ffi::c_int
            - 1 as core::ffi::c_int) as WORD16;
    }
    (*pstr_freq_band_data).freq_band_table[LOW as usize] = ((*pstr_freq_band_data)
        .freq_band_tbl_lo)
        .as_mut_ptr();
    (*pstr_freq_band_data).freq_band_table[HIGH as usize] = ((*pstr_freq_band_data)
        .freq_band_tbl_hi)
        .as_mut_ptr();
    (*ptr_header_data).pstr_freq_band_data = pstr_freq_band_data;
    (*ptr_header_data).core_frame_size = samp_per_frame as WORD16;
    (*ptr_header_data).out_sampling_freq = sample_rate_dec << 1 as core::ffi::c_int;
    if audio_obj_type != AOT_ER_AAC_ELD as core::ffi::c_int {
        tmp = ((*ptr_header_data).time_step as core::ffi::c_int + 4 as core::ffi::c_int)
            as WORD32;
        if tmp < 0 as core::ffi::c_int {
            (*ptr_header_data).num_time_slots = ixheaac_extract16l(
                samp_per_frame << -tmp,
            );
        } else {
            (*ptr_header_data).num_time_slots = ixheaac_extract16l(
                samp_per_frame >> tmp,
            );
        }
    } else {
        (*ptr_header_data).time_step = 1 as WORD16;
        (*ptr_header_data).num_time_slots = (samp_per_frame as core::ffi::c_int
            / 32 as core::ffi::c_int
            >> (*ptr_header_data).time_step as core::ffi::c_int - 1 as core::ffi::c_int)
            as WORD16;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_setesbr_flags(
    mut sbr_persistent_mem_v: *mut core::ffi::c_void,
    mut pvc_flag: FLAG,
    mut hbe_flag: FLAG,
    mut inter_tes_flag: FLAG,
) -> VOID {
    let mut sbr_persistent_mem: *mut ia_sbr_pers_struct = 0 as *mut ia_sbr_pers_struct;
    sbr_persistent_mem = sbr_persistent_mem_v as *mut ia_sbr_pers_struct;
    (*sbr_persistent_mem).str_sbr_dec_inst.hbe_flag = hbe_flag;
    (*sbr_persistent_mem).str_sbr_dec_inst.pvc_flag = pvc_flag;
    (*sbr_persistent_mem).str_sbr_dec_inst.inter_tes_flag = inter_tes_flag;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_init_sbr(
    mut sample_rate_dec: WORD32,
    mut samp_per_frame: WORD32,
    mut down_sample_flag: *mut FLAG,
    mut sbr_persistent_mem_v: *mut core::ffi::c_void,
    mut ptr_overlap_buf: *mut WORD32,
    mut channel: WORD32,
    mut ps_enable: WORD32,
    mut sbr_ratio_idx: WORD32,
    mut output_frame_size: WORD32,
    mut use_hbe: *mut WORD32,
    mut p_usac_dflt_header: *mut core::ffi::c_void,
    mut str_sbr_config: ia_sbr_header_data_struct,
    mut audio_object_type: WORD32,
    mut ldmps_present: WORD32,
    mut ldsbr_present: WORD32,
) -> ia_handle_sbr_dec_inst_struct {
    let mut i: WORD16 = 0;
    let mut err: WORD16 = 0;
    let mut ptr_header_data: [*mut ia_sbr_header_data_struct; 2] = [0
        as *mut ia_sbr_header_data_struct; 2];
    let mut ptr_sbr_dec: [*mut ia_sbr_dec_struct; 2] = [0 as *mut ia_sbr_dec_struct; 2];
    let mut qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct = 0
        as *mut ia_qmf_dec_tables_struct;
    let mut sbr_persistent_mem: *mut ia_sbr_pers_struct = 0 as *mut ia_sbr_pers_struct;
    sbr_persistent_mem = sbr_persistent_mem_v as *mut ia_sbr_pers_struct;
    ptr_sbr_dec[0 as core::ffi::c_int as usize] = &mut (**((*sbr_persistent_mem)
        .str_sbr_dec_inst
        .pstr_sbr_channel)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize))
        .str_sbr_dec;
    ptr_sbr_dec[1 as core::ffi::c_int as usize] = &mut (**((*sbr_persistent_mem)
        .str_sbr_dec_inst
        .pstr_sbr_channel)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as isize))
        .str_sbr_dec;
    qmf_dec_tables_ptr = (*(*sbr_persistent_mem).str_sbr_dec_inst.pstr_sbr_tables)
        .qmf_dec_tables_ptr;
    if sample_rate_dec > 48000 as core::ffi::c_int {
        *down_sample_flag = 1 as core::ffi::c_int as FLAG;
    }
    i = 0 as WORD16;
    while (i as core::ffi::c_int) < channel {
        if audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int {
            memcpy(
                (*sbr_persistent_mem).str_sbr_dec_inst.pstr_sbr_header[i as usize]
                    as *mut core::ffi::c_void,
                &mut str_sbr_config as *mut ia_sbr_header_data_struct
                    as *const core::ffi::c_void,
                ::core::mem::size_of::<ia_sbr_header_data_struct>() as size_t,
            );
        }
        ptr_header_data[i as usize] = (*sbr_persistent_mem)
            .str_sbr_dec_inst
            .pstr_sbr_header[i as usize];
        ixheaacd_init_headerdata(
            ptr_header_data[i as usize],
            sample_rate_dec,
            samp_per_frame,
            (*sbr_persistent_mem).str_sbr_dec_inst.pstr_freq_band_data[i as usize],
            (*sbr_persistent_mem).str_sbr_dec_inst.pstr_sbr_tables,
            audio_object_type as WORD,
        );
        err = ixheaacd_create_sbrdec(
            (*sbr_persistent_mem).str_sbr_dec_inst.pstr_common_tables,
            (*sbr_persistent_mem).str_sbr_dec_inst.pstr_sbr_channel[i as usize],
            ptr_header_data[i as usize],
            i,
            *down_sample_flag,
            sbr_persistent_mem as *mut core::ffi::c_void,
            ps_enable as WORD,
            audio_object_type as WORD,
            ldmps_present,
            ldsbr_present,
        );
        (*ptr_header_data[i as usize]).status = 1 as core::ffi::c_int as WORD32;
        (*ptr_sbr_dec[i as usize]).band_count = 64 as core::ffi::c_int as WORD32;
        (*((*ptr_header_data[i as usize]).pstr_freq_band_data)
            .offset(0 as core::ffi::c_int as isize))
            .qmf_sb_prev = 64 as WORD16;
        (*((*ptr_header_data[i as usize]).pstr_freq_band_data)
            .offset(1 as core::ffi::c_int as isize))
            .qmf_sb_prev = 64 as WORD16;
        if err != 0 {
            return 0 as ia_handle_sbr_dec_inst_struct;
        }
        i += 1;
    }
    if channel != 1 as core::ffi::c_int {
        if ps_enable != 0 {
            if audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int {
                ixheaacd_create_psdec(
                    (*sbr_persistent_mem).str_sbr_dec_inst.pstr_ps_stereo_dec,
                    sbr_persistent_mem as *mut core::ffi::c_void,
                    &mut *ptr_overlap_buf
                        .offset(
                            (512 as core::ffi::c_int * 4 as core::ffi::c_int) as isize,
                        ),
                    samp_per_frame,
                );
            } else {
                ixheaacd_create_psdec(
                    (*sbr_persistent_mem).str_sbr_dec_inst.pstr_ps_stereo_dec,
                    sbr_persistent_mem as *mut core::ffi::c_void,
                    ptr_overlap_buf,
                    samp_per_frame,
                );
            }
        }
    }
    if !use_hbe.is_null()
        && !(audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int
            || audio_object_type == AOT_ER_AAC_LD as core::ffi::c_int)
    {
        let mut ptr_sbr_dflt_header: *mut ia_sbr_header_data_struct = &mut (*sbr_persistent_mem)
            .str_sbr_dec_inst
            .str_sbr_dflt_header;
        let mut ptr_usac_dflt_header: *mut ia_sbr_header_data_struct = p_usac_dflt_header
            as *mut ia_sbr_header_data_struct;
        let mut p_str_sbr_dec_inst: *mut ia_sbr_dec_inst_struct = &mut (*sbr_persistent_mem)
            .str_sbr_dec_inst;
        let mut hbe_txposer_buffers: *mut core::ffi::c_void = (*p_str_sbr_dec_inst)
            .hbe_txposer_buffers;
        ptr_header_data[0 as core::ffi::c_int as usize] = (*p_str_sbr_dec_inst)
            .pstr_sbr_header[0 as core::ffi::c_int as usize];
        ptr_header_data[1 as core::ffi::c_int as usize] = (*p_str_sbr_dec_inst)
            .pstr_sbr_header[1 as core::ffi::c_int as usize];
        (*ptr_header_data[0 as core::ffi::c_int as usize]).sbr_ratio_idx = sbr_ratio_idx;
        (*ptr_header_data[0 as core::ffi::c_int as usize]).output_framesize = output_frame_size;
        (*(*ptr_header_data[0 as core::ffi::c_int as usize]).pstr_freq_band_data)
            .sub_band_start = 64 as WORD16;
        (*ptr_header_data[0 as core::ffi::c_int as usize]).esbr_start_up = 1
            as core::ffi::c_int as WORD32;
        (*ptr_header_data[0 as core::ffi::c_int as usize]).esbr_start_up_pvc = 1
            as core::ffi::c_int as WORD32;
        if channel > 1 as core::ffi::c_int {
            (*ptr_header_data[1 as core::ffi::c_int as usize]).sbr_ratio_idx = sbr_ratio_idx;
            (*ptr_header_data[1 as core::ffi::c_int as usize]).output_framesize = output_frame_size;
            (*(*ptr_header_data[1 as core::ffi::c_int as usize]).pstr_freq_band_data)
                .sub_band_start = 64 as WORD16;
            (*ptr_header_data[1 as core::ffi::c_int as usize]).esbr_start_up = 1
                as core::ffi::c_int as WORD32;
            (*ptr_header_data[1 as core::ffi::c_int as usize]).esbr_start_up_pvc = 1
                as core::ffi::c_int as WORD32;
        }
        if !hbe_txposer_buffers.is_null() {
            let mut persistant_used: WORD32 = 0 as WORD32;
            ixheaacd_esbr_hbe_data_init(
                (*ptr_sbr_dec[0 as core::ffi::c_int as usize]).p_hbe_txposer,
                samp_per_frame,
                if sbr_ratio_idx == SBR_UPSAMPLE_IDX_4_1 {
                    1 as WORD32
                } else {
                    0 as WORD32
                },
                output_frame_size,
                hbe_txposer_buffers,
                &mut persistant_used,
            );
            hbe_txposer_buffers = (hbe_txposer_buffers as *mut WORD8)
                .offset(
                    ((64 as usize)
                        .wrapping_mul(::core::mem::size_of::<*mut FLOAT32>() as usize)
                        .wrapping_add(
                            ((2 as core::ffi::c_int * 64 as core::ffi::c_int) as usize)
                                .wrapping_mul(
                                    ::core::mem::size_of::<*mut FLOAT32>() as usize,
                                ),
                        )
                        .wrapping_add(
                            ((64 as core::ffi::c_int * 64 as core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize),
                        )
                        .wrapping_add(
                            ((64 as core::ffi::c_int * 64 as core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize),
                        )
                        .wrapping_add(
                            ((64 as core::ffi::c_int * 2 as core::ffi::c_int
                                * 64 as core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize),
                        )
                        .wrapping_add(
                            ((64 as core::ffi::c_int * 2 as core::ffi::c_int
                                * 64 as core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize),
                        )
                        .wrapping_add(
                            ((1024 as core::ffi::c_int + 64 as core::ffi::c_int)
                                as usize)
                                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize),
                        )
                        .wrapping_add(
                            (((1024 as core::ffi::c_int + 64 as core::ffi::c_int)
                                * 2 as core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize),
                        )
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        as isize,
                ) as *mut core::ffi::c_void;
            ixheaacd_esbr_hbe_data_init(
                (*ptr_sbr_dec[1 as core::ffi::c_int as usize]).p_hbe_txposer,
                samp_per_frame,
                if sbr_ratio_idx == SBR_UPSAMPLE_IDX_4_1 {
                    1 as WORD32
                } else {
                    0 as WORD32
                },
                output_frame_size,
                hbe_txposer_buffers,
                &mut persistant_used,
            );
        }
        (*(*p_str_sbr_dec_inst).ptr_pvc_data_str).prev_first_bnd_idx = -(1
            as core::ffi::c_int) as WORD16;
        (*(*p_str_sbr_dec_inst).ptr_pvc_data_str).prev_pvc_rate = -(1
            as core::ffi::c_int) as UWORD8;
        (*(*p_str_sbr_dec_inst).ptr_pvc_data_str).prev_sbr_mode = UNKNOWN_SBR
            as core::ffi::c_int as WORD32;
        (*(*p_str_sbr_dec_inst).pstr_sbr_channel[0 as core::ffi::c_int as usize])
            .str_sbr_dec
            .str_codec_qmf_bank
            .num_time_slots = (output_frame_size as core::ffi::c_int
            / 64 as core::ffi::c_int) as WORD16;
        (*(*p_str_sbr_dec_inst).pstr_sbr_channel[1 as core::ffi::c_int as usize])
            .str_sbr_dec
            .str_codec_qmf_bank
            .num_time_slots = (output_frame_size as core::ffi::c_int
            / 64 as core::ffi::c_int) as WORD16;
        (*ptr_header_data[0 as core::ffi::c_int as usize]).core_frame_size = samp_per_frame
            as WORD16;
        (*ptr_header_data[1 as core::ffi::c_int as usize]).core_frame_size = samp_per_frame
            as WORD16;
        match sbr_ratio_idx {
            SBR_UPSAMPLE_IDX_0_0 => {
                (*ptr_sbr_dec[0 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .no_channels = 32 as core::ffi::c_int as WORD32;
                (*ptr_sbr_dec[0 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .esbr_cos_twiddle = ((*qmf_dec_tables_ptr).esbr_sin_cos_twiddle_l32)
                    .as_mut_ptr();
                (*ptr_sbr_dec[0 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .esbr_alt_sin_twiddle = ((*qmf_dec_tables_ptr)
                    .esbr_alt_sin_twiddle_l32)
                    .as_mut_ptr();
                (*ptr_sbr_dec[0 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .esbr_t_cos = ((*qmf_dec_tables_ptr).esbr_t_cos_sin_l32)
                    .as_mut_ptr();
                (*ptr_header_data[0 as core::ffi::c_int as usize]).is_usf_4 = 0
                    as core::ffi::c_int as WORD32;
                (*ptr_header_data[0 as core::ffi::c_int as usize]).upsamp_fac = 1
                    as core::ffi::c_int as WORD32;
                (*ptr_sbr_dec[1 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .no_channels = 32 as core::ffi::c_int as WORD32;
                (*ptr_sbr_dec[1 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .esbr_cos_twiddle = ((*qmf_dec_tables_ptr).esbr_sin_cos_twiddle_l32)
                    .as_mut_ptr();
                (*ptr_sbr_dec[1 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .esbr_alt_sin_twiddle = ((*qmf_dec_tables_ptr)
                    .esbr_alt_sin_twiddle_l32)
                    .as_mut_ptr();
                (*ptr_sbr_dec[1 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .esbr_t_cos = ((*qmf_dec_tables_ptr).esbr_t_cos_sin_l32)
                    .as_mut_ptr();
                (*ptr_header_data[1 as core::ffi::c_int as usize]).is_usf_4 = 0
                    as core::ffi::c_int as WORD32;
                (*ptr_header_data[1 as core::ffi::c_int as usize]).upsamp_fac = 1
                    as core::ffi::c_int as WORD32;
            }
            SBR_UPSAMPLE_IDX_2_1 => {
                (*ptr_sbr_dec[0 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .no_channels = 32 as core::ffi::c_int as WORD32;
                (*ptr_sbr_dec[0 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .esbr_cos_twiddle = ((*qmf_dec_tables_ptr).esbr_sin_cos_twiddle_l32)
                    .as_mut_ptr();
                (*ptr_sbr_dec[0 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .esbr_alt_sin_twiddle = ((*qmf_dec_tables_ptr)
                    .esbr_alt_sin_twiddle_l32)
                    .as_mut_ptr();
                (*ptr_sbr_dec[0 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .esbr_t_cos = ((*qmf_dec_tables_ptr).esbr_t_cos_sin_l32)
                    .as_mut_ptr();
                (*ptr_header_data[0 as core::ffi::c_int as usize]).is_usf_4 = 0
                    as core::ffi::c_int as WORD32;
                (*ptr_header_data[0 as core::ffi::c_int as usize]).upsamp_fac = 2
                    as core::ffi::c_int as WORD32;
                (*ptr_sbr_dec[1 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .no_channels = 32 as core::ffi::c_int as WORD32;
                (*ptr_sbr_dec[1 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .esbr_cos_twiddle = ((*qmf_dec_tables_ptr).esbr_sin_cos_twiddle_l32)
                    .as_mut_ptr();
                (*ptr_sbr_dec[1 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .esbr_alt_sin_twiddle = ((*qmf_dec_tables_ptr)
                    .esbr_alt_sin_twiddle_l32)
                    .as_mut_ptr();
                (*ptr_sbr_dec[1 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .esbr_t_cos = ((*qmf_dec_tables_ptr).esbr_t_cos_sin_l32)
                    .as_mut_ptr();
                (*ptr_header_data[1 as core::ffi::c_int as usize]).is_usf_4 = 0
                    as core::ffi::c_int as WORD32;
                (*ptr_header_data[1 as core::ffi::c_int as usize]).upsamp_fac = 2
                    as core::ffi::c_int as WORD32;
            }
            SBR_UPSAMPLE_IDX_8_3 => {
                (*ptr_sbr_dec[0 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .no_channels = 24 as core::ffi::c_int as WORD32;
                (*ptr_sbr_dec[0 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .filter_pos_32 = ((*qmf_dec_tables_ptr).esbr_qmf_c_24).as_mut_ptr();
                (*ptr_sbr_dec[0 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .analy_win_coeff_32 = ((*qmf_dec_tables_ptr).esbr_qmf_c_24)
                    .as_mut_ptr();
                (*ptr_sbr_dec[0 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .esbr_cos_twiddle = ((*qmf_dec_tables_ptr).esbr_sin_cos_twiddle_l24)
                    .as_mut_ptr();
                (*ptr_sbr_dec[0 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .esbr_alt_sin_twiddle = ((*qmf_dec_tables_ptr)
                    .esbr_alt_sin_twiddle_l24)
                    .as_mut_ptr();
                (*ptr_sbr_dec[0 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .esbr_t_cos = ((*qmf_dec_tables_ptr).esbr_t_cos_sin_l24)
                    .as_mut_ptr();
                (*ptr_header_data[0 as core::ffi::c_int as usize]).is_usf_4 = 0
                    as core::ffi::c_int as WORD32;
                (*ptr_header_data[0 as core::ffi::c_int as usize]).upsamp_fac = 2
                    as core::ffi::c_int as WORD32;
                (*ptr_sbr_dec[1 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .no_channels = 24 as core::ffi::c_int as WORD32;
                (*ptr_sbr_dec[1 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .filter_pos_32 = ((*qmf_dec_tables_ptr).esbr_qmf_c_24).as_mut_ptr();
                (*ptr_sbr_dec[1 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .analy_win_coeff_32 = ((*qmf_dec_tables_ptr).esbr_qmf_c_24)
                    .as_mut_ptr();
                (*ptr_sbr_dec[1 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .esbr_cos_twiddle = ((*qmf_dec_tables_ptr).esbr_sin_cos_twiddle_l24)
                    .as_mut_ptr();
                (*ptr_sbr_dec[1 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .esbr_alt_sin_twiddle = ((*qmf_dec_tables_ptr)
                    .esbr_alt_sin_twiddle_l24)
                    .as_mut_ptr();
                (*ptr_sbr_dec[1 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .esbr_t_cos = ((*qmf_dec_tables_ptr).esbr_t_cos_sin_l24)
                    .as_mut_ptr();
                (*ptr_header_data[1 as core::ffi::c_int as usize]).is_usf_4 = 0
                    as core::ffi::c_int as WORD32;
                (*ptr_header_data[1 as core::ffi::c_int as usize]).upsamp_fac = 2
                    as core::ffi::c_int as WORD32;
            }
            SBR_UPSAMPLE_IDX_4_1 => {
                (*ptr_sbr_dec[0 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .no_channels = 16 as core::ffi::c_int as WORD32;
                (*ptr_sbr_dec[0 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .esbr_cos_twiddle = ((*qmf_dec_tables_ptr).esbr_sin_cos_twiddle_l16)
                    .as_mut_ptr();
                (*ptr_sbr_dec[0 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .esbr_alt_sin_twiddle = ((*qmf_dec_tables_ptr)
                    .esbr_alt_sin_twiddle_l16)
                    .as_mut_ptr();
                (*ptr_sbr_dec[0 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .esbr_t_cos = ((*qmf_dec_tables_ptr).esbr_t_cos_sin_l16)
                    .as_mut_ptr();
                (*ptr_header_data[0 as core::ffi::c_int as usize]).is_usf_4 = 1
                    as core::ffi::c_int as WORD32;
                (*ptr_header_data[0 as core::ffi::c_int as usize]).upsamp_fac = 4
                    as core::ffi::c_int as WORD32;
                (*ptr_header_data[0 as core::ffi::c_int as usize]).out_sampling_freq = ((*ptr_header_data[0
                    as core::ffi::c_int as usize])
                    .out_sampling_freq as core::ffi::c_int * 2 as core::ffi::c_int)
                    as WORD32;
                (*ptr_sbr_dec[1 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .no_channels = 16 as core::ffi::c_int as WORD32;
                (*ptr_sbr_dec[1 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .esbr_cos_twiddle = ((*qmf_dec_tables_ptr).esbr_sin_cos_twiddle_l16)
                    .as_mut_ptr();
                (*ptr_sbr_dec[1 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .esbr_alt_sin_twiddle = ((*qmf_dec_tables_ptr)
                    .esbr_alt_sin_twiddle_l16)
                    .as_mut_ptr();
                (*ptr_sbr_dec[1 as core::ffi::c_int as usize])
                    .str_codec_qmf_bank
                    .esbr_t_cos = ((*qmf_dec_tables_ptr).esbr_t_cos_sin_l16)
                    .as_mut_ptr();
                (*ptr_header_data[1 as core::ffi::c_int as usize]).is_usf_4 = 1
                    as core::ffi::c_int as WORD32;
                (*ptr_header_data[1 as core::ffi::c_int as usize]).upsamp_fac = 4
                    as core::ffi::c_int as WORD32;
                (*ptr_header_data[1 as core::ffi::c_int as usize]).out_sampling_freq = ((*ptr_header_data[1
                    as core::ffi::c_int as usize])
                    .out_sampling_freq as core::ffi::c_int * 2 as core::ffi::c_int)
                    as WORD32;
            }
            _ => {}
        }
        if !ptr_usac_dflt_header.is_null() {
            (*ptr_sbr_dflt_header).start_freq = (*ptr_usac_dflt_header).start_freq;
            (*ptr_sbr_dflt_header).stop_freq = (*ptr_usac_dflt_header).stop_freq;
            if (*ptr_usac_dflt_header).header_extra_1 != 0 {
                (*ptr_sbr_dflt_header).freq_scale = (*ptr_usac_dflt_header).freq_scale;
                (*ptr_sbr_dflt_header).alter_scale = (*ptr_usac_dflt_header).alter_scale;
                (*ptr_sbr_dflt_header).noise_bands = (*ptr_usac_dflt_header).noise_bands;
            } else {
                (*ptr_sbr_dflt_header).freq_scale = SBR_FREQ_SCALE_DEFAULT as WORD16;
                (*ptr_sbr_dflt_header).alter_scale = SBR_ALTER_SCALE_DEFAULT as WORD16;
                (*ptr_sbr_dflt_header).noise_bands = SBR_NOISE_BANDS_DEFAULT as WORD16;
            }
            if (*ptr_usac_dflt_header).header_extra_2 != 0 {
                (*ptr_sbr_dflt_header).limiter_bands = (*ptr_usac_dflt_header)
                    .limiter_bands;
                (*ptr_sbr_dflt_header).limiter_gains = (*ptr_usac_dflt_header)
                    .limiter_gains;
                (*ptr_sbr_dflt_header).interpol_freq = (*ptr_usac_dflt_header)
                    .interpol_freq;
                (*ptr_sbr_dflt_header).smoothing_mode = (*ptr_usac_dflt_header)
                    .smoothing_mode;
            } else {
                (*ptr_sbr_dflt_header).limiter_bands = SBR_LIMITER_BANDS_DEFAULT
                    as WORD16;
                (*ptr_sbr_dflt_header).limiter_gains = SBR_LIMITER_GAINS_DEFAULT
                    as WORD16;
                (*ptr_sbr_dflt_header).interpol_freq = SBR_INTERPOL_FREQ_DEFAULT
                    as WORD16;
                (*ptr_sbr_dflt_header).smoothing_mode = SBR_SMOOTHING_LENGTH_DEFAULT
                    as WORD16;
            }
        }
    }
    return &mut (*sbr_persistent_mem).str_sbr_dec_inst;
}
#[inline]
unsafe extern "C" fn ixheaacd_create_sbr_env_calc(
    mut pstr_common_table: *mut ixheaacd_misc_tables,
    mut hs: *mut ia_sbr_calc_env_struct,
    mut chan: WORD16,
    mut sbr_persistent_mem_v: *mut core::ffi::c_void,
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut audio_object_type: WORD,
) -> WORD16 {
    let mut err: WORD16 = 0;
    let mut sbr_persistent_mem: *mut ia_sbr_pers_struct = sbr_persistent_mem_v
        as *mut ia_sbr_pers_struct;
    err = 0 as WORD16;
    memset(
        &mut *((*hs).harm_flags_prev).as_mut_ptr().offset(0 as core::ffi::c_int as isize)
            as *mut WORD8 as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<WORD8>() as size_t)
            .wrapping_mul(MAX_FREQ_COEFFS as size_t),
    );
    (*hs).harm_index = 0 as WORD16;
    (*hs).filt_buf_me = (*sbr_persistent_mem).sbr_smooth_gain_buf[chan as usize];
    (*hs).filt_buf_noise_m = (*sbr_persistent_mem).sbr_smooth_noise_buf[chan as usize];
    (*hs).tansient_env_prev = -(1 as core::ffi::c_int) as WORD16;
    ixheaacd_reset_sbrenvelope_calc(hs);
    if chan as core::ffi::c_int == 0 as core::ffi::c_int
        && audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int
    {
        err = ixheaacd_calc_frq_bnd_tbls(ptr_header_data, pstr_common_table) as WORD16;
    }
    return err;
}
#[inline]
unsafe extern "C" fn ixheaacd_init_sbr_prev_framedata(
    mut ptr_prev_data: *mut ia_sbr_prev_frame_data_struct,
    mut time_slots: WORD16,
) -> VOID {
    let mut psfb_nrg_prev: *mut WORD16 = ((*ptr_prev_data).sfb_nrg_prev).as_mut_ptr();
    let mut psfb_noise_level: *mut WORD16 = ((*ptr_prev_data).prev_noise_level)
        .as_mut_ptr();
    let mut ppsbr_invf_mode: *mut WORD32 = ((*ptr_prev_data).sbr_invf_mode).as_mut_ptr();
    memset(
        psfb_nrg_prev as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<WORD16>() as size_t).wrapping_mul(56 as size_t),
    );
    memset(
        psfb_noise_level as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<WORD16>() as size_t).wrapping_mul(5 as size_t),
    );
    memset(
        ppsbr_invf_mode as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<WORD32>() as size_t)
            .wrapping_mul(MAX_INVF_BANDS as size_t),
    );
    (*ptr_prev_data).end_position = time_slots;
    (*ptr_prev_data).coupling_mode = COUPLING_OFF as WORD32;
    (*ptr_prev_data).amp_res = 0 as WORD16;
    (*ptr_prev_data).max_qmf_subband_aac = 0 as core::ffi::c_int as WORD32;
}
#[inline]
unsafe extern "C" fn ixheaacd_create_hyb_filterbank(
    mut ptr_hybrid: *mut ia_hybrid_struct,
    mut p_ptr: *mut *mut WORD32,
    mut sbr_tables_ptr: *mut ia_sbr_tables_struct,
) -> VOID {
    let mut i: WORD16 = 0;
    let mut ptr: *mut WORD32 = *p_ptr;
    (*ptr_hybrid).ptr_resol = ((*(*sbr_tables_ptr).ps_tables_ptr).hyb_resol)
        .as_mut_ptr();
    (*ptr_hybrid).ptr_qmf_buf = (HYBRID_FILTER_LENGTH - 1 as core::ffi::c_int) as WORD8;
    (*ptr_hybrid).ptr_temp_re = ptr;
    ptr = ptr.offset(NO_HYBRID_CHANNELS_HIGH as isize);
    (*ptr_hybrid).ptr_temp_im = ptr;
    ptr = ptr.offset(NO_HYBRID_CHANNELS_HIGH as isize);
    memset(
        (*ptr_hybrid).ptr_temp_re as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ((2 as core::ffi::c_int * NO_HYBRID_CHANNELS_HIGH) as size_t)
            .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
    );
    (*ptr_hybrid).ptr_work_re = ptr;
    ptr = ptr.offset(16 as core::ffi::c_int as isize);
    (*ptr_hybrid).ptr_work_im = ptr;
    ptr = ptr.offset(16 as core::ffi::c_int as isize);
    i = 0 as WORD16;
    while (i as core::ffi::c_int) < NO_QMF_CHANNELS_IN_HYBRID {
        (*ptr_hybrid).ptr_qmf_buf_re[i as usize] = ptr;
        ptr = ptr.offset((*ptr_hybrid).ptr_qmf_buf as core::ffi::c_int as isize);
        (*ptr_hybrid).ptr_qmf_buf_im[i as usize] = ptr;
        ptr = ptr.offset((*ptr_hybrid).ptr_qmf_buf as core::ffi::c_int as isize);
        memset(
            (*ptr_hybrid).ptr_qmf_buf_re[i as usize] as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ((2 as core::ffi::c_int * (*ptr_hybrid).ptr_qmf_buf as core::ffi::c_int)
                as size_t)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
        );
        i += 1;
    }
    *p_ptr = ptr;
}
#[inline]
unsafe extern "C" fn ixheaacd_create_hf_generator(
    mut ptr_hf_gen_str: *mut ia_sbr_hf_generator_struct,
    mut num_columns: WORD16,
    mut chan: WORD16,
    mut sbr_persistent_mem_v: *mut core::ffi::c_void,
    mut ps_enable: WORD32,
) -> VOID {
    let mut i: WORD16 = 0;
    let mut sbr_persistent_mem: *mut ia_sbr_pers_struct = sbr_persistent_mem_v
        as *mut ia_sbr_pers_struct;
    (*ptr_hf_gen_str).pstr_settings = &mut (*sbr_persistent_mem).str_sbr_tran_settings;
    (*ptr_hf_gen_str).lpc_filt_states_real[0 as core::ffi::c_int as usize] = *((*sbr_persistent_mem)
        .sbr_lpc_filter_states_real[chan as usize])
        .offset(0 as core::ffi::c_int as isize);
    (*ptr_hf_gen_str).lpc_filt_states_real[1 as core::ffi::c_int as usize] = *((*sbr_persistent_mem)
        .sbr_lpc_filter_states_real[chan as usize])
        .offset(1 as core::ffi::c_int as isize);
    if ps_enable != 0 {
        (*ptr_hf_gen_str).lpc_filt_states_imag[0 as core::ffi::c_int as usize] = *((*sbr_persistent_mem)
            .sbr_lpc_filter_states_imag[chan as usize])
            .offset(0 as core::ffi::c_int as isize);
        (*ptr_hf_gen_str).lpc_filt_states_imag[1 as core::ffi::c_int as usize] = *((*sbr_persistent_mem)
            .sbr_lpc_filter_states_imag[chan as usize])
            .offset(1 as core::ffi::c_int as isize);
    }
    i = 0 as WORD16;
    while (i as core::ffi::c_int) < LPC_ORDER {
        if !((*ptr_hf_gen_str).lpc_filt_states_real[i as usize]).is_null() {
            memset(
                (*ptr_hf_gen_str).lpc_filt_states_real[i as usize]
                    as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                (NO_ANALYSIS_CHANNELS as size_t)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
            );
        }
        if ps_enable != 0 {
            memset(
                (*ptr_hf_gen_str).lpc_filt_states_imag[i as usize]
                    as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                (NO_ANALYSIS_CHANNELS as size_t)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
            );
        }
        i += 1;
    }
    if chan as core::ffi::c_int == 0 as core::ffi::c_int {
        (*(*ptr_hf_gen_str).pstr_settings).num_columns = num_columns;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_create_psdec(
    mut ptr_ps_dec: *mut ia_ps_dec_struct,
    mut sbr_persistent_mem_v: *mut core::ffi::c_void,
    mut ptr_overlap_buf: *mut WORD32,
    mut frame_size: WORD32,
) -> VOID {
    let mut sbr_persistent_mem: *mut ia_sbr_pers_struct = sbr_persistent_mem_v
        as *mut ia_sbr_pers_struct;
    let mut ptr1: *mut WORD16 = &mut *(*((*sbr_persistent_mem).ptr_sbr_overlap_buf)
        .as_mut_ptr()
        .offset((MAXNRSBRCHANNELS - 1 as core::ffi::c_int) as isize))
        .offset(0 as core::ffi::c_int as isize) as *mut WORD32 as *mut WORD16;
    let mut ptr2: *mut WORD32 = &mut *ptr_overlap_buf
        .offset(512 as core::ffi::c_int as isize) as *mut WORD32;
    let mut initial_ptr: *mut WORD16 = 0 as *mut WORD16;
    let mut delay: WORD16 = 0;
    let mut temp: WORD32 = 0;
    let mut sbr_tables_ptr: *mut ia_sbr_tables_struct = (*sbr_persistent_mem)
        .str_sbr_dec_inst
        .pstr_sbr_tables;
    memset(
        ptr_ps_dec as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<ia_ps_dec_struct>() as size_t,
    );
    (*ptr_ps_dec).ps_data_present = 0 as core::ffi::c_int as FLAG;
    (*ptr_ps_dec).enable_iid = 0 as core::ffi::c_int as FLAG;
    (*ptr_ps_dec).enable_icc = 0 as core::ffi::c_int as FLAG;
    (*ptr_ps_dec).enable_ext = 0 as core::ffi::c_int as FLAG;
    (*ptr_ps_dec).iid_mode = 0 as WORD16;
    (*ptr_ps_dec).icc_mode = 0 as WORD16;
    (*ptr_ps_dec).ptr_hyb_left_re = ptr2;
    ptr2 = ptr2.offset(16 as core::ffi::c_int as isize);
    (*ptr_ps_dec).ptr_hyb_left_im = ptr2;
    ptr2 = ptr2.offset(16 as core::ffi::c_int as isize);
    (*ptr_ps_dec).ptr_hyb_right_re = ptr2;
    ptr2 = ptr2.offset(16 as core::ffi::c_int as isize);
    (*ptr_ps_dec).ptr_hyb_right_im = ptr2;
    ptr2 = ptr2.offset(16 as core::ffi::c_int as isize);
    memset(
        (*ptr_ps_dec).ptr_hyb_left_re as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<WORD32>() as size_t)
            .wrapping_mul(16 as size_t)
            .wrapping_mul(4 as size_t),
    );
    ixheaacd_create_hyb_filterbank(
        &mut (*ptr_ps_dec).str_hybrid,
        &mut ptr2,
        sbr_tables_ptr,
    );
    (*ptr_ps_dec).peak_decay_diff = ptr2;
    ptr2 = ptr2.offset(NUM_OF_BINS as isize);
    (*ptr_ps_dec).energy_prev = ptr2;
    ptr2 = ptr2.offset(NUM_OF_BINS as isize);
    (*ptr_ps_dec).peak_decay_diff_prev = ptr2;
    ptr2 = ptr2.offset(NUM_OF_BINS as isize);
    memset(
        (*ptr_ps_dec).peak_decay_diff as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (3 as size_t)
            .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t)
            .wrapping_mul(NUM_OF_BINS as size_t),
    );
    (*ptr_ps_dec).delay_buf_idx = 0 as WORD16;
    (*ptr_ps_dec).delay_buf_idx_long = 0 as WORD16;
    memset(
        ((*ptr_ps_dec).delay_buf_qmf_sub_re_im).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ((2 as core::ffi::c_int * 16 as core::ffi::c_int * DEL_ALL_PASS) as size_t)
            .wrapping_mul(::core::mem::size_of::<WORD16>() as size_t),
    );
    memset(
        ((*ptr_ps_dec).delay_buf_qmf_sub_ser_re_im).as_mut_ptr()
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ((2 as core::ffi::c_int * 16 as core::ffi::c_int * NUM_SER_AP_LINKS
            * 5 as core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<WORD16>() as size_t),
    );
    initial_ptr = ptr1;
    (*ptr_ps_dec).delay_buf_qmf_ser_re_im = ptr1 as *mut core::ffi::c_void
        as REVERB_BUFFERS_RI;
    ptr1 = ptr1
        .offset(
            (2 as core::ffi::c_int * NUM_SER_AP_LINKS * 32 as core::ffi::c_int
                * 5 as core::ffi::c_int) as isize,
        );
    delay = 2 as WORD16;
    (*ptr_ps_dec).delay_buf_qmf_ap_re_im = ptr1 as *mut core::ffi::c_void
        as *mut [WORD16; 64];
    ptr1 = ptr1
        .offset(
            (2 as core::ffi::c_int * delay as core::ffi::c_int * 32 as core::ffi::c_int)
                as isize,
        );
    delay = HIGH_DEL as WORD16;
    (*ptr_ps_dec).delay_buf_qmf_ld_re_im = ptr1 as *mut core::ffi::c_void
        as *mut [WORD16; 24];
    ptr1 = ptr1
        .offset(
            (2 as core::ffi::c_int * delay as core::ffi::c_int * SMALL_DEL_STRT) as isize,
        );
    delay = SMALL_DEL as WORD16;
    (*ptr_ps_dec).delay_buf_qmf_sd_re_im = ptr1 as *mut core::ffi::c_void
        as *mut [WORD16; 64];
    ptr1 = ptr1
        .offset(
            (2 as core::ffi::c_int * delay as core::ffi::c_int
                * (NUM_OF_QUAD_MIRROR_FILTER_ICC_CHNLS
                    - (NUM_OF_QUAD_MIRROR_FILTER_ALL_PASS_CHNLS + SMALL_DEL_STRT)))
                as isize,
        );
    temp = ptr1.offset_from(initial_ptr) as core::ffi::c_long as WORD32;
    memset(
        (*ptr_ps_dec).delay_buf_qmf_ser_re_im as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (temp as size_t).wrapping_mul(::core::mem::size_of::<WORD16>() as size_t),
    );
    memset(
        ((*ptr_ps_dec).delay_buf_idx_ser).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (NUM_SER_AP_LINKS as size_t)
            .wrapping_mul(::core::mem::size_of::<WORD16>() as size_t),
    );
    memcpy(
        ((*ptr_ps_dec).delay_sample_ser).as_mut_ptr() as *mut core::ffi::c_void,
        ((*(*sbr_tables_ptr).ps_tables_ptr).rev_link_delay_ser).as_mut_ptr()
            as *const core::ffi::c_void,
        (NUM_SER_AP_LINKS as size_t)
            .wrapping_mul(::core::mem::size_of::<WORD16>() as size_t),
    );
    memset(
        ((*ptr_ps_dec).h11_h12_vec).as_mut_ptr() as *mut core::ffi::c_void,
        0xff as core::ffi::c_int,
        (((NO_IID_GROUPS + 2 as core::ffi::c_int) * 2 as core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<WORD16>() as size_t),
    );
    memset(
        ((*ptr_ps_dec).h21_h22_vec).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[WORD16; 48]>() as size_t,
    );
    if frame_size == 960 as core::ffi::c_int {
        (*ptr_ps_dec).num_sub_samples = NUM_SUB_SAMPLES_960 as WORD32;
    } else {
        (*ptr_ps_dec).num_sub_samples = NUM_SUB_SAMPLES as WORD32;
    }
    ixheaacd_create_ps_esbr_dec(
        ptr_ps_dec,
        (*sbr_tables_ptr).ps_tables_ptr,
        64 as UWORD32,
        (*ptr_ps_dec).num_sub_samples as UWORD32,
        0 as WORD32,
    );
}
#[inline]
unsafe extern "C" fn ixheaacd_create_cplx_anal_qmfbank(
    mut ptr_sbr_qmf: *mut ia_sbr_qmf_filter_bank_struct,
    mut sbr_scale_factor: *mut ia_sbr_scale_fact_struct,
    mut no_bins: WORD16,
    mut usb: WORD16,
    mut chan: WORD16,
    mut sbr_qmf_analy_states: *mut WORD16,
    mut sbr_qmf_analy_states_32: *mut WORD32,
    mut qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
    mut audio_object_type: WORD32,
    mut ldmps_present: WORD32,
    mut no_ldsbr: WORD32,
) -> VOID {
    memset(
        ptr_sbr_qmf as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<ia_sbr_qmf_filter_bank_struct>() as size_t,
    );
    (*ptr_sbr_qmf).analy_win_coeff_32 = ((*qmf_dec_tables_ptr).esbr_qmf_c).as_mut_ptr();
    if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
        && audio_object_type != AOT_ER_AAC_LD as core::ffi::c_int
    {
        (*ptr_sbr_qmf).analy_win_coeff = ((*qmf_dec_tables_ptr).qmf_c).as_mut_ptr();
    } else {
        (*ptr_sbr_qmf).analy_win_coeff = ((*qmf_dec_tables_ptr).qmf_c_eld3).as_mut_ptr();
        if ldmps_present == 1 as core::ffi::c_int {
            (*ptr_sbr_qmf).analy_win_coeff_32 = ((*qmf_dec_tables_ptr).qmf_c_ldsbr_mps)
                .as_mut_ptr();
        }
        if no_ldsbr == 1 as core::ffi::c_int {
            (*ptr_sbr_qmf).analy_win_coeff_32 = ixheaacd_ldmps_polyphase_filter_coeff_fix
                .as_ptr() as *mut WORD32;
        }
    }
    (*ptr_sbr_qmf).no_channels = NO_ANALYSIS_CHANNELS as WORD32;
    if no_ldsbr != 0 {
        (*ptr_sbr_qmf).no_channels = 64 as core::ffi::c_int as WORD32;
    }
    (*ptr_sbr_qmf).num_time_slots = no_bins;
    (*ptr_sbr_qmf).lsb = 0 as WORD16;
    (*ptr_sbr_qmf).usb = usb;
    (*ptr_sbr_qmf).anal_filter_states = &mut *sbr_qmf_analy_states
        .offset(
            (if chan as core::ffi::c_int != 0 {
                QMF_FILTER_STATE_ANA_SIZE
            } else {
                0 as core::ffi::c_int
            }) as isize,
        ) as *mut WORD16;
    memset(
        (*ptr_sbr_qmf).anal_filter_states as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<WORD16>() as size_t)
            .wrapping_mul(QMF_FILTER_STATE_ANA_SIZE as size_t),
    );
    (*ptr_sbr_qmf).anal_filter_states_32 = &mut *sbr_qmf_analy_states_32
        .offset(
            (if chan as core::ffi::c_int != 0 {
                QMF_FILTER_STATE_ANA_SIZE
            } else {
                0 as core::ffi::c_int
            }) as isize,
        ) as *mut WORD32;
    memset(
        (*ptr_sbr_qmf).anal_filter_states_32 as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<WORD32>() as size_t)
            .wrapping_mul(QMF_FILTER_STATE_ANA_SIZE as size_t),
    );
    (*ptr_sbr_qmf).core_samples_buffer = (*ptr_sbr_qmf).anal_filter_states;
    (*ptr_sbr_qmf).core_samples_buffer_32 = (*ptr_sbr_qmf).anal_filter_states_32;
    (*ptr_sbr_qmf).filter_pos_32 = ((*qmf_dec_tables_ptr).esbr_qmf_c).as_mut_ptr();
    (*ptr_sbr_qmf).state_new_samples_pos_low_32 = (*ptr_sbr_qmf).anal_filter_states_32;
    if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
        && audio_object_type != AOT_ER_AAC_LD as core::ffi::c_int
    {
        (*ptr_sbr_qmf).filter_pos = ((*qmf_dec_tables_ptr).qmf_c).as_mut_ptr();
    } else {
        (*ptr_sbr_qmf).filter_pos = ((*qmf_dec_tables_ptr).qmf_c_eld3).as_mut_ptr();
        if ldmps_present == 1 as core::ffi::c_int {
            (*ptr_sbr_qmf).filter_pos_32 = ((*qmf_dec_tables_ptr).qmf_c_ldsbr_mps)
                .as_mut_ptr();
        }
        if no_ldsbr == 1 as core::ffi::c_int {
            (*ptr_sbr_qmf).filter_pos_32 = ixheaacd_ldmps_polyphase_filter_coeff_fix
                .as_ptr() as *mut WORD32;
        }
    }
    (*sbr_scale_factor).st_lb_scale = 0 as WORD16;
    (*sbr_scale_factor).st_syn_scale = -(6 as core::ffi::c_int) as WORD16;
    if audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int
        || audio_object_type == AOT_ER_AAC_LD as core::ffi::c_int
    {
        (*ptr_sbr_qmf).filter_2_32 = ((*ptr_sbr_qmf).filter_pos_32)
            .offset((*ptr_sbr_qmf).no_channels as isize);
        (*ptr_sbr_qmf).fp1_anal_32 = (*ptr_sbr_qmf).anal_filter_states_32;
        (*ptr_sbr_qmf).fp2_anal_32 = ((*ptr_sbr_qmf).anal_filter_states_32)
            .offset((*ptr_sbr_qmf).no_channels as isize);
        (*ptr_sbr_qmf).filter_2 = ((*ptr_sbr_qmf).filter_pos)
            .offset((*ptr_sbr_qmf).no_channels as isize);
        (*ptr_sbr_qmf).fp1_anal = (*ptr_sbr_qmf).anal_filter_states;
        (*ptr_sbr_qmf).fp2_anal = ((*ptr_sbr_qmf).anal_filter_states)
            .offset((*ptr_sbr_qmf).no_channels as isize);
    }
}
#[inline]
unsafe extern "C" fn ixheaacd_create_cplx_synt_qmfbank(
    mut ptr_sbr_qmf: *mut ia_sbr_qmf_filter_bank_struct,
    mut no_bins: WORD16,
    mut lsb: WORD16,
    mut usb: WORD16,
    mut chan: WORD16,
    mut down_sample_flag: FLAG,
    mut sbr_qmf_synth_states: *mut WORD16,
    mut sbr_qmf_synth_states_32: *mut WORD32,
    mut qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
    mut audio_object_type: WORD32,
) -> VOID {
    let mut L: WORD32 = 0;
    let mut qmf_filter_state_size: WORD32 = 0;
    memset(
        ptr_sbr_qmf as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<ia_sbr_qmf_filter_bank_struct>() as size_t,
    );
    if down_sample_flag != 0 {
        L = NO_SYNTHESIS_CHANNELS_DOWN_SAMPLED as WORD32;
        qmf_filter_state_size = QMF_FILTER_STATE_SYN_SIZE_DOWN_SAMPLED as WORD32;
        (*ptr_sbr_qmf).usb = NO_SYNTHESIS_CHANNELS_DOWN_SAMPLED as WORD16;
    } else {
        L = NO_SYNTHESIS_CHANNELS as WORD32;
        qmf_filter_state_size = QMF_FILTER_STATE_SYN_SIZE as WORD32;
        (*ptr_sbr_qmf).usb = usb;
    }
    (*ptr_sbr_qmf).ixheaacd_drc_offset = 0 as WORD16;
    if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
        && audio_object_type != AOT_ER_AAC_LD as core::ffi::c_int
    {
        (*ptr_sbr_qmf).filter_pos_syn = ((*qmf_dec_tables_ptr).qmf_c).as_mut_ptr();
        (*ptr_sbr_qmf).p_filter = ((*qmf_dec_tables_ptr).qmf_c).as_mut_ptr();
    } else {
        (*ptr_sbr_qmf).filter_pos_syn = ((*qmf_dec_tables_ptr).qmf_c_eld).as_mut_ptr();
        (*ptr_sbr_qmf).p_filter = ((*qmf_dec_tables_ptr).qmf_c_eld).as_mut_ptr();
    }
    (*ptr_sbr_qmf).filter_pos_syn_32 = ((*qmf_dec_tables_ptr).esbr_qmf_c).as_mut_ptr();
    (*ptr_sbr_qmf).p_filter_32 = ((*qmf_dec_tables_ptr).esbr_qmf_c).as_mut_ptr();
    (*ptr_sbr_qmf).no_channels = L;
    (*ptr_sbr_qmf).qmf_filter_state_size = qmf_filter_state_size as WORD16;
    (*ptr_sbr_qmf).num_time_slots = no_bins;
    (*ptr_sbr_qmf).lsb = lsb;
    (*ptr_sbr_qmf).filter_states = &mut *sbr_qmf_synth_states
        .offset(
            (if chan as core::ffi::c_int != 0 {
                qmf_filter_state_size as core::ffi::c_int
            } else {
                0 as core::ffi::c_int
            }) as isize,
        ) as *mut WORD16;
    memset(
        (*ptr_sbr_qmf).filter_states as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<WORD16>() as size_t)
            .wrapping_mul(qmf_filter_state_size as size_t),
    );
    (*ptr_sbr_qmf).filter_states_32 = &mut *sbr_qmf_synth_states_32
        .offset(
            (if chan as core::ffi::c_int != 0 {
                qmf_filter_state_size as core::ffi::c_int
            } else {
                0 as core::ffi::c_int
            }) as isize,
        ) as *mut WORD32;
    memset(
        (*ptr_sbr_qmf).filter_states_32 as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<WORD32>() as size_t)
            .wrapping_mul(qmf_filter_state_size as size_t),
    );
    if audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int
        || audio_object_type == AOT_ER_AAC_LD as core::ffi::c_int
    {
        (*ptr_sbr_qmf).fp1_syn = (*ptr_sbr_qmf).filter_states;
        (*ptr_sbr_qmf).fp2_syn = ((*ptr_sbr_qmf).filter_states)
            .offset((*ptr_sbr_qmf).no_channels as isize);
        (*ptr_sbr_qmf).sixty4 = NO_SYNTHESIS_CHANNELS as WORD16;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_create_sbrdec(
    mut pstr_common_table: *mut ixheaacd_misc_tables,
    mut ptr_sbr_channel: *mut ia_sbr_channel_struct,
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut chan: WORD16,
    mut down_sample_flag: FLAG,
    mut sbr_persistent_mem_v: *mut core::ffi::c_void,
    mut ps_enable: WORD,
    mut audio_object_type: WORD,
    mut ldmps_present: WORD32,
    mut ldsbr_present: WORD32,
) -> WORD16 {
    let mut err: WORD16 = 0;
    let mut time_slots: WORD16 = 0;
    let mut no_bins: WORD16 = 0;
    let mut sbr_persistent_mem: *mut ia_sbr_pers_struct = sbr_persistent_mem_v
        as *mut ia_sbr_pers_struct;
    let mut hs: *mut ia_sbr_dec_struct = &mut (*ptr_sbr_channel).str_sbr_dec;
    time_slots = (*ptr_header_data).num_time_slots;
    no_bins = (time_slots as core::ffi::c_int
        * (*ptr_header_data).time_step as core::ffi::c_int) as WORD16;
    (*hs).str_sbr_scale_fact.ov_lb_scale = (INT_BITS - 1 as core::ffi::c_int) as WORD16;
    (*hs).str_sbr_scale_fact.hb_scale = (INT_BITS - 1 as core::ffi::c_int) as WORD16;
    (*hs).str_sbr_scale_fact.ov_hb_scale = (INT_BITS - 1 as core::ffi::c_int) as WORD16;
    (*hs).str_sbr_scale_fact.st_syn_scale = (INT_BITS - 1 as core::ffi::c_int) as WORD16;
    (*ptr_sbr_channel).pstr_prev_frame_data = (*sbr_persistent_mem)
        .pstr_prev_frame_data[chan as usize];
    err = ixheaacd_create_sbr_env_calc(
        pstr_common_table,
        &mut (*hs).str_sbr_calc_env,
        chan,
        sbr_persistent_mem as *mut core::ffi::c_void,
        ptr_header_data,
        audio_object_type,
    );
    if err != 0 {
        return -(1 as core::ffi::c_int) as WORD16;
    }
    ixheaacd_create_cplx_anal_qmfbank(
        &mut (*hs).str_codec_qmf_bank,
        &mut (*hs).str_sbr_scale_fact,
        no_bins,
        (*(*ptr_header_data).pstr_freq_band_data).sub_band_start,
        chan,
        (*sbr_persistent_mem).sbr_qmf_analy_states,
        (*sbr_persistent_mem).sbr_qmf_analy_states_32,
        (*(*sbr_persistent_mem).str_sbr_dec_inst.pstr_sbr_tables).qmf_dec_tables_ptr,
        audio_object_type as WORD32,
        ldmps_present,
        ldsbr_present,
    );
    ixheaacd_create_cplx_synt_qmfbank(
        &mut (*hs).str_synthesis_qmf_bank,
        no_bins,
        (*(*ptr_header_data).pstr_freq_band_data).sub_band_start,
        (*(*ptr_header_data).pstr_freq_band_data).sub_band_end,
        chan,
        down_sample_flag,
        (*sbr_persistent_mem).sbr_qmf_synth_states,
        (*sbr_persistent_mem).sbr_qmf_synth_states_32,
        (*(*sbr_persistent_mem).str_sbr_dec_inst.pstr_sbr_tables).qmf_dec_tables_ptr,
        audio_object_type as WORD32,
    );
    ixheaacd_init_sbr_prev_framedata(
        (*ptr_sbr_channel).pstr_prev_frame_data,
        time_slots,
    );
    ixheaacd_create_hf_generator(
        &mut (*hs).str_hf_generator,
        (*hs).str_codec_qmf_bank.num_time_slots,
        chan,
        sbr_persistent_mem as *mut core::ffi::c_void,
        ps_enable as WORD32,
    );
    (*hs).ptr_sbr_overlap_buf = (*sbr_persistent_mem).ptr_sbr_overlap_buf[chan as usize];
    return 0 as WORD16;
}
