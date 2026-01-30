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
    fn log10(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn pow(__x: core::ffi::c_double, __y: core::ffi::c_double) -> core::ffi::c_double;
    fn fabs(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn ixheaacd_read_bits_buf(
        it_bit_buff: *mut ia_bit_buf_struct,
        no_of_bits: WORD,
    ) -> WORD32;
    fn ixheaacd_divideby2(op: WORD32) -> WORD16;
    fn ixheaacd_divideby3(op: WORD32) -> WORD16;
    fn ixheaacd_ssc_huff_dec(
        h: ia_huffman_data_type,
        it_bit_buff: *mut ia_bit_buf_struct,
    ) -> WORD32;
    static ixheaacd_num_bands: [WORD16; 3];
    fn ixheaacd_aac_shellsort(in_0: *mut WORD16, n: WORD32) -> VOID;
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
}
pub type size_t = usize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ixheaacd_lpp_trans_patch {
    pub num_patches: WORD32,
    pub start_subband: [WORD32; 7],
}
pub type ia_huffman_data_type = *const UWORD16;
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
pub const AOT_ER_AAC_LD: AUDIO_OBJECT_TYPE = 23;
pub const AOT_ER_AAC_ELD: AUDIO_OBJECT_TYPE = 39;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_auto_corr_ele_struct {
    pub phi_0_1_real: FLOAT32,
    pub phi_0_1_imag: FLOAT32,
    pub phi_0_2_real: FLOAT32,
    pub phi_0_2_imag: FLOAT32,
    pub phi_1_1: FLOAT32,
    pub phi_1_2_real: FLOAT32,
    pub phi_1_2_imag: FLOAT32,
    pub phi_2_2: FLOAT32,
    pub det: FLOAT32,
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
unsafe extern "C" fn ixheaac_add32(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut sum: WORD32 = 0;
    sum = a + b;
    return sum;
}
#[inline]
unsafe extern "C" fn ixheaac_abs16_sat(mut op1: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    if -(32768 as core::ffi::c_int) == op1 as core::ffi::c_int {
        var_out = MAX_16;
    } else if (op1 as core::ffi::c_int) < 0 as core::ffi::c_int {
        var_out = -(op1 as core::ffi::c_int) as WORD16;
    } else {
        var_out = op1;
    }
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
pub const MAX_NOISE_COEFFS: core::ffi::c_int = 5 as core::ffi::c_int;
pub const MAX_ENVELOPES: core::ffi::c_int = 8 as core::ffi::c_int;
pub const MAX_FREQ_COEFFS: core::ffi::c_int = 56 as core::ffi::c_int;
pub const IA_FATAL_ERROR: core::ffi::c_uint = 0x80000000 as core::ffi::c_uint;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const COUPLING_BAL: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_INVF_BANDS: core::ffi::c_int = MAX_NOISE_COEFFS;
static mut ixheaacd_new_bw_table: [[FLOAT32; 4]; 4] = [
    [0.00f32, 0.60f32, 0.90f32, 0.98f32],
    [0.60f32, 0.75f32, 0.90f32, 0.98f32],
    [0.00f32, 0.75f32, 0.90f32, 0.98f32],
    [0.00f32, 0.75f32, 0.90f32, 0.98f32],
];
static mut ixheaacd_inew_bw_table: [[WORD32; 4]; 4] = [
    [
        0 as core::ffi::c_int,
        0x4ccccccd as core::ffi::c_int,
        0x73333333 as core::ffi::c_int,
        0x7d70a3d7 as core::ffi::c_int,
    ],
    [
        0x4ccccccd as core::ffi::c_int,
        0x60000000 as core::ffi::c_int,
        0x73333333 as core::ffi::c_int,
        0x7d70a3d7 as core::ffi::c_int,
    ],
    [
        0 as core::ffi::c_int,
        0x60000000 as core::ffi::c_int,
        0x73333333 as core::ffi::c_int,
        0x7d70a3d7 as core::ffi::c_int,
    ],
    [
        0 as core::ffi::c_int,
        0x60000000 as core::ffi::c_int,
        0x73333333 as core::ffi::c_int,
        0x7d70a3d7 as core::ffi::c_int,
    ],
];
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_reset_sbrenvelope_calc(
    mut h_cal_env: *mut ia_sbr_calc_env_struct,
) -> VOID {
    (*h_cal_env).ph_index = 0 as WORD16;
    (*h_cal_env).filt_buf_noise_e = 0 as core::ffi::c_int as WORD32;
    (*h_cal_env).start_up = 1 as core::ffi::c_int as FLAG;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_derive_lim_band_tbl(
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut p_str_patch_param: *const ia_patch_param_struct,
    mut num_patches: WORD16,
    mut pstr_common_tables: *mut ixheaacd_misc_tables,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut k_1: WORD32 = 0;
    let mut nr_lim: WORD32 = 0;
    let mut patch_border_k: WORD32 = 0;
    let mut patch_border_k_1: WORD32 = 0;
    let mut temp_nr_lim: WORD32 = 0;
    let mut lim_table: [WORD16; 35] = [0; 35];
    let mut patch_borders: [WORD16; 7] = [0; 7];
    let mut kx: WORD16 = 0;
    let mut k2: WORD16 = 0;
    let mut temp: WORD16 = 0;
    let mut lim_bands: WORD16 = 0;
    let mut num_octaves: WORD16 = 0;
    let mut f_lim_tbl: *mut WORD16 = ((*(*ptr_header_data).pstr_freq_band_data)
        .freq_band_tbl_lim)
        .as_mut_ptr();
    let mut num_lf_bands: *mut WORD16 = &mut (*(*ptr_header_data).pstr_freq_band_data)
        .num_lf_bands;
    let mut f_low_tbl: *mut WORD16 = (*(*ptr_header_data).pstr_freq_band_data)
        .freq_band_table[LOW as usize];
    let mut num_low_bnd: WORD16 = (*(*ptr_header_data).pstr_freq_band_data)
        .num_sf_bands[LOW as usize];
    let mut limiter_bands: WORD16 = (*ptr_header_data).limiter_bands;
    let mut sub_band_start: WORD16 = *f_low_tbl.offset(0 as core::ffi::c_int as isize);
    let mut sub_band_end: WORD16 = *f_low_tbl.offset(num_low_bnd as isize);
    let limbnd_per_oct: [WORD16; 4] = [
        0x2000 as core::ffi::c_int as WORD16,
        0x2666 as core::ffi::c_int as WORD16,
        0x4000 as core::ffi::c_int as WORD16,
        0x6000 as core::ffi::c_int as WORD16,
    ];
    if limiter_bands as core::ffi::c_int == 0 as core::ffi::c_int {
        *f_lim_tbl.offset(0 as core::ffi::c_int as isize) = 0 as WORD16;
        *f_lim_tbl.offset(1 as core::ffi::c_int as isize) = (sub_band_end
            as core::ffi::c_int - sub_band_start as core::ffi::c_int) as WORD16;
        nr_lim = 1 as core::ffi::c_int as WORD32;
    } else {
        k = 0 as core::ffi::c_int as WORD32;
        while k < num_patches as core::ffi::c_int {
            patch_borders[k as usize] = ((*p_str_patch_param.offset(k as isize))
                .guard_start_band as core::ffi::c_int
                - sub_band_start as core::ffi::c_int) as WORD16;
            k += 1;
        }
        patch_borders[k as usize] = (sub_band_end as core::ffi::c_int
            - sub_band_start as core::ffi::c_int) as WORD16;
        k = 0 as core::ffi::c_int as WORD32;
        while k <= num_low_bnd as core::ffi::c_int {
            lim_table[k as usize] = (*f_low_tbl.offset(k as isize) as core::ffi::c_int
                - sub_band_start as core::ffi::c_int) as WORD16;
            k += 1;
        }
        k = 1 as core::ffi::c_int as WORD32;
        while k < num_patches as core::ffi::c_int {
            lim_table[(num_low_bnd as WORD32 + k) as usize] = patch_borders[k as usize];
            k += 1;
        }
        nr_lim = (num_low_bnd as core::ffi::c_int + num_patches as core::ffi::c_int
            - 1 as core::ffi::c_int) as WORD32;
        temp_nr_lim = nr_lim;
        ixheaacd_aac_shellsort(lim_table.as_mut_ptr(), temp_nr_lim + 1 as WORD32);
        k = 1 as core::ffi::c_int as WORD32;
        k_1 = 0 as core::ffi::c_int as WORD32;
        lim_bands = limbnd_per_oct[limiter_bands as usize];
        while k - temp_nr_lim <= 0 as core::ffi::c_int {
            k2 = (lim_table[k as usize] as core::ffi::c_int
                + sub_band_start as core::ffi::c_int) as WORD16;
            kx = (lim_table[k_1 as usize] as core::ffi::c_int
                + sub_band_start as core::ffi::c_int) as WORD16;
            num_octaves = (*pstr_common_tables).log_dual_is_table[k2 as usize];
            num_octaves = (num_octaves as core::ffi::c_int
                - (*pstr_common_tables).log_dual_is_table[kx as usize]
                    as core::ffi::c_int) as WORD16;
            temp = (lim_bands as WORD32 * num_octaves as WORD32
                >> 15 as core::ffi::c_int) as WORD16;
            if (temp as core::ffi::c_int) < 0x1f6 as core::ffi::c_int {
                if lim_table[k_1 as usize] as core::ffi::c_int
                    == lim_table[k as usize] as core::ffi::c_int
                {
                    lim_table[k as usize] = sub_band_end;
                    nr_lim = (nr_lim as core::ffi::c_int - 1 as core::ffi::c_int)
                        as WORD32;
                    k = (k as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
                    continue;
                } else {
                    patch_border_k = 0 as core::ffi::c_int as WORD32;
                    patch_border_k_1 = patch_border_k;
                    i = 0 as core::ffi::c_int as WORD32;
                    while i <= num_patches as core::ffi::c_int {
                        if lim_table[k as usize] as core::ffi::c_int
                            == patch_borders[i as usize] as core::ffi::c_int
                        {
                            patch_border_k = 1 as core::ffi::c_int as WORD32;
                        }
                        if lim_table[k_1 as usize] as core::ffi::c_int
                            == patch_borders[i as usize] as core::ffi::c_int
                        {
                            patch_border_k_1 = 1 as core::ffi::c_int as WORD32;
                        }
                        i += 1;
                    }
                    if patch_border_k == 0 {
                        lim_table[k as usize] = sub_band_end;
                        nr_lim = (nr_lim as core::ffi::c_int - 1 as core::ffi::c_int)
                            as WORD32;
                        k = (k as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
                        continue;
                    } else if patch_border_k_1 == 0 {
                        lim_table[k_1 as usize] = sub_band_end;
                        nr_lim = (nr_lim as core::ffi::c_int - 1 as core::ffi::c_int)
                            as WORD32;
                    }
                }
            }
            k_1 = k;
            k = (k as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
        }
        ixheaacd_aac_shellsort(lim_table.as_mut_ptr(), temp_nr_lim + 1 as WORD32);
        memcpy(
            f_lim_tbl as *mut core::ffi::c_void,
            lim_table.as_mut_ptr() as *const core::ffi::c_void,
            (::core::mem::size_of::<WORD16>() as size_t)
                .wrapping_mul(
                    (nr_lim as core::ffi::c_int + 1 as core::ffi::c_int) as size_t,
                ),
        );
    }
    *num_lf_bands = nr_lim as WORD16;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_lean_sbrconcealment(
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut ptr_sbr_data: *mut ia_sbr_frame_info_data_struct,
    mut ptr_prev_data: *mut ia_sbr_prev_frame_data_struct,
) -> VOID {
    let mut target: WORD32 = 0;
    let mut step: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut cur_start_pos: WORD16 = 0;
    let mut cur_stop_pos: WORD16 = 0;
    (*ptr_sbr_data).amp_res = (*ptr_prev_data).amp_res;
    (*ptr_sbr_data).coupling_mode = (*ptr_prev_data).coupling_mode;
    (*ptr_sbr_data).max_qmf_subband_aac = (*ptr_prev_data).max_qmf_subband_aac;
    memcpy(
        ((*ptr_sbr_data).sbr_invf_mode).as_mut_ptr() as *mut core::ffi::c_void,
        ((*ptr_prev_data).sbr_invf_mode).as_mut_ptr() as *const core::ffi::c_void,
        (::core::mem::size_of::<WORD32>() as size_t)
            .wrapping_mul(MAX_INVF_BANDS as size_t),
    );
    (*ptr_sbr_data).str_frame_info_details.num_env = 1 as WORD16;
    cur_start_pos = ((*ptr_prev_data).end_position as core::ffi::c_int
        - (*ptr_header_data).num_time_slots as core::ffi::c_int) as WORD16;
    cur_stop_pos = (*ptr_header_data).num_time_slots;
    (*ptr_sbr_data).str_frame_info_details.border_vec[0 as core::ffi::c_int as usize] = cur_start_pos;
    (*ptr_sbr_data).str_frame_info_details.border_vec[1 as core::ffi::c_int as usize] = cur_stop_pos;
    (*ptr_sbr_data)
        .str_frame_info_details
        .noise_border_vec[0 as core::ffi::c_int as usize] = cur_start_pos;
    (*ptr_sbr_data)
        .str_frame_info_details
        .noise_border_vec[1 as core::ffi::c_int as usize] = cur_stop_pos;
    (*ptr_sbr_data).str_frame_info_details.freq_res[0 as core::ffi::c_int as usize] = 1
        as WORD16;
    (*ptr_sbr_data).str_frame_info_details.transient_env = -(1 as core::ffi::c_int)
        as WORD16;
    (*ptr_sbr_data).str_frame_info_details.num_noise_env = 1 as WORD16;
    (*ptr_sbr_data).num_env_sfac = (*(*ptr_header_data).pstr_freq_band_data)
        .num_sf_bands[1 as core::ffi::c_int as usize];
    (*ptr_sbr_data).del_cod_dir_arr[0 as core::ffi::c_int as usize] = DTDF_DIR_TIME
        as WORD16;
    if (*ptr_sbr_data).coupling_mode == COUPLING_BAL {
        target = SBR_ENERGY_PAN_OFFSET as WORD32;
    } else {
        target = 0 as core::ffi::c_int as WORD32;
    }
    step = 1 as core::ffi::c_int as WORD32;
    if (*ptr_header_data).amp_res as core::ffi::c_int - SBR_AMPLITUDE_RESOLUTION_1_5
        == 0 as core::ffi::c_int
    {
        target = target << 1 as core::ffi::c_int;
        step = step << 1 as core::ffi::c_int;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*ptr_sbr_data).num_env_sfac as core::ffi::c_int {
        if (*ptr_prev_data).sfb_nrg_prev[i as usize] as core::ffi::c_int > target {
            (*ptr_sbr_data).int_env_sf_arr[i as usize] = -step as WORD16;
        } else {
            (*ptr_sbr_data).int_env_sf_arr[i as usize] = step as WORD16;
        }
        i += 1;
    }
    (*ptr_sbr_data).del_cod_dir_noise_arr[0 as core::ffi::c_int as usize] = DTDF_DIR_TIME
        as WORD16;
    memset(
        ((*ptr_sbr_data).int_noise_floor).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[WORD16; 10]>() as size_t,
    );
    memset(
        ((*ptr_sbr_data).add_harmonics).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<FLAG>() as size_t)
            .wrapping_mul(MAX_FREQ_COEFFS as size_t),
    );
}
unsafe extern "C" fn ixheaacd_find_closest_entry(
    mut goal_sb: WORD32,
    mut f_master_tbl: *mut WORD16,
    mut num_mf_bands: WORD16,
    mut direction: WORD16,
) -> WORD16 {
    let mut index: WORD32 = 0;
    if goal_sb
        <= *f_master_tbl.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
    {
        return *f_master_tbl.offset(0 as core::ffi::c_int as isize);
    }
    if goal_sb >= *f_master_tbl.offset(num_mf_bands as isize) as core::ffi::c_int {
        return *f_master_tbl.offset(num_mf_bands as isize);
    }
    if direction != 0 {
        index = 0 as core::ffi::c_int as WORD32;
        while (*f_master_tbl.offset(index as isize) as core::ffi::c_int) < goal_sb {
            index += 1;
        }
    } else {
        index = num_mf_bands as WORD32;
        while *f_master_tbl.offset(index as isize) as core::ffi::c_int > goal_sb {
            index -= 1;
        }
    }
    return *f_master_tbl.offset(index as isize);
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_reset_hf_generator(
    mut ptr_hf_gen_str: *mut ia_sbr_hf_generator_struct,
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut audio_object_type: WORD,
) -> WORD32 {
    let mut patch: WORD32 = 0;
    let mut sb: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut ptr_noise_freq_tbl: *mut WORD16 = 0 as *mut WORD16;
    let mut num_nf_bands: WORD32 = 0;
    let mut pstr_transposer_settings: *mut ia_transposer_settings_struct = (*ptr_hf_gen_str)
        .pstr_settings;
    let mut p_str_patch_param: *mut ia_patch_param_struct = ((*pstr_transposer_settings)
        .str_patch_param)
        .as_mut_ptr();
    let mut sub_band_start: WORD32 = (*(*ptr_header_data).pstr_freq_band_data)
        .sub_band_start as WORD32;
    let mut f_master_tbl: *mut WORD16 = ((*(*ptr_header_data).pstr_freq_band_data)
        .f_master_tbl)
        .as_mut_ptr();
    let mut num_mf_bands: WORD16 = (*(*ptr_header_data).pstr_freq_band_data)
        .num_mf_bands;
    let mut usb: WORD16 = (*(*ptr_header_data).pstr_freq_band_data).sub_band_end;
    let mut src_start_band: WORD32 = 0;
    let mut patch_stride: WORD32 = 0;
    let mut num_bands_in_patch: WORD32 = 0;
    let mut lsb: WORD32 = *f_master_tbl.offset(0 as core::ffi::c_int as isize) as WORD32;
    let mut xover_offset: WORD16 = (sub_band_start - lsb) as WORD16;
    let mut goal_sb: WORD16 = 0;
    let mut flag_break_1: WORD16 = 0 as WORD16;
    let mut fs: WORD32 = (*ptr_header_data).out_sampling_freq;
    if lsb < SHIFT_START_SB + 4 as core::ffi::c_int {
        return 1 as WORD32;
    }
    match fs {
        16000 | 22050 | 24000 | 32000 => {
            goal_sb = 64 as WORD16;
        }
        44100 => {
            goal_sb = 46 as WORD16;
        }
        48000 => {
            goal_sb = 43 as WORD16;
        }
        64000 => {
            goal_sb = 32 as WORD16;
        }
        88200 => {
            goal_sb = 23 as WORD16;
        }
        96000 => {
            goal_sb = 21 as WORD16;
        }
        _ => return 0 as WORD32,
    }
    goal_sb = ixheaacd_find_closest_entry(
        goal_sb as WORD32,
        f_master_tbl,
        num_mf_bands,
        1 as WORD16,
    );
    if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
        && audio_object_type != AOT_ER_AAC_LD as core::ffi::c_int
    {
        if (ixheaac_abs16_sat(
            (goal_sb as core::ffi::c_int - usb as core::ffi::c_int) as WORD16,
        ) as core::ffi::c_int) < 4 as core::ffi::c_int
        {
            goal_sb = usb;
        }
    }
    src_start_band = (SHIFT_START_SB + xover_offset as core::ffi::c_int) as WORD32;
    sb = (lsb as core::ffi::c_int + xover_offset as core::ffi::c_int) as WORD32;
    patch = 0 as core::ffi::c_int as WORD32;
    if (goal_sb as core::ffi::c_int) < sb && lsb > src_start_band {
        return -(1 as WORD32);
    }
    while (sb as core::ffi::c_int - usb as core::ffi::c_int) < 0 as core::ffi::c_int
        && patch < MAX_NUM_PATCHES
    {
        let mut ptr_loc_patch_param: *mut ia_patch_param_struct = &mut *p_str_patch_param
            .offset(patch as isize) as *mut ia_patch_param_struct;
        let mut abs_sb: WORD16 = 0;
        let mut flag_break: WORD16 = 0 as WORD16;
        (*ptr_loc_patch_param).guard_start_band = sb as WORD16;
        sb = (sb as core::ffi::c_int + GUARDBANDS) as WORD32;
        (*ptr_loc_patch_param).dst_start_band = sb as WORD16;
        num_bands_in_patch = goal_sb as WORD32 - sb;
        if num_bands_in_patch <= 0 as core::ffi::c_int
            && num_bands_in_patch - (lsb - src_start_band) < 0 as core::ffi::c_int
        {
            flag_break = 1 as WORD16;
        }
        if num_bands_in_patch - (lsb - src_start_band) >= 0 as core::ffi::c_int {
            patch_stride = sb - src_start_band;
            patch_stride = (patch_stride as core::ffi::c_int & !(1 as core::ffi::c_int))
                as WORD16 as WORD32;
            num_bands_in_patch = lsb - (sb - patch_stride);
            num_bands_in_patch = ixheaacd_find_closest_entry(
                sb + num_bands_in_patch,
                f_master_tbl,
                num_mf_bands,
                0 as WORD16,
            ) as WORD32;
            num_bands_in_patch -= sb;
        }
        patch_stride = num_bands_in_patch + sb - lsb;
        patch_stride = (patch_stride as core::ffi::c_int + 1 as core::ffi::c_int
            & !(1 as core::ffi::c_int)) as WORD16 as WORD32;
        if num_bands_in_patch > 0 as core::ffi::c_int {
            (*ptr_loc_patch_param).src_start_band = (sb - patch_stride) as WORD16;
            (*ptr_loc_patch_param).dst_end_band = patch_stride as WORD16;
            (*ptr_loc_patch_param).num_bands_in_patch = num_bands_in_patch as WORD16;
            (*ptr_loc_patch_param).src_end_band = ((*ptr_loc_patch_param).src_start_band
                as WORD32 + num_bands_in_patch) as WORD16;
            sb = (sb as core::ffi::c_int
                + (*ptr_loc_patch_param).num_bands_in_patch as core::ffi::c_int)
                as WORD32;
            patch += 1;
        }
        src_start_band = SHIFT_START_SB as WORD32;
        abs_sb = (ixheaac_abs16_sat(
            (sb as core::ffi::c_int - goal_sb as core::ffi::c_int) as WORD16,
        ) as core::ffi::c_int - 3 as core::ffi::c_int) as WORD16;
        if num_bands_in_patch <= 0 as core::ffi::c_int
            && flag_break_1 as core::ffi::c_int == 1 as core::ffi::c_int
        {
            break;
        }
        if (abs_sb as core::ffi::c_int) < 0 as core::ffi::c_int {
            goal_sb = usb;
        } else if flag_break as core::ffi::c_int == 1 as core::ffi::c_int {
            break;
        }
        if num_bands_in_patch <= 0 as core::ffi::c_int {
            flag_break_1 = 1 as WORD16;
        } else {
            flag_break_1 = 0 as WORD16;
        }
    }
    patch -= 1;
    if patch > 0 as core::ffi::c_int
        && ((*p_str_patch_param.offset(patch as isize)).num_bands_in_patch
            as core::ffi::c_int) < 3 as core::ffi::c_int
    {
        patch -= 1;
        sb = ((*p_str_patch_param.offset(patch as isize)).dst_start_band
            as core::ffi::c_int
            + (*p_str_patch_param.offset(patch as isize)).num_bands_in_patch
                as core::ffi::c_int) as WORD32;
    }
    if patch >= MAX_NUM_PATCHES {
        return -(1 as WORD32);
    }
    (*pstr_transposer_settings).num_patches = (patch as core::ffi::c_int
        + 1 as core::ffi::c_int) as WORD16;
    temp = 0 as core::ffi::c_int as WORD32;
    patch = 0 as core::ffi::c_int as WORD32;
    while patch < (*pstr_transposer_settings).num_patches as core::ffi::c_int {
        sb = ixheaac_min32(
            sb,
            (*p_str_patch_param.offset(patch as isize)).src_start_band as WORD32,
        );
        temp = ixheaac_max32(
            temp,
            (*p_str_patch_param.offset(patch as isize)).src_end_band as WORD32,
        );
        patch += 1;
    }
    if sb > temp {
        return IA_FATAL_ERROR as WORD32;
    }
    (*pstr_transposer_settings).start_patch = sb as WORD16;
    (*pstr_transposer_settings).stop_patch = temp as WORD16;
    ptr_noise_freq_tbl = ((*(*ptr_header_data).pstr_freq_band_data).freq_band_tbl_noise)
        .as_mut_ptr();
    num_nf_bands = (*(*ptr_header_data).pstr_freq_band_data).num_nf_bands as WORD32;
    memcpy(
        &mut *((*pstr_transposer_settings).bw_borders)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut WORD16
            as *mut core::ffi::c_void,
        &mut *ptr_noise_freq_tbl.offset(1 as core::ffi::c_int as isize) as *mut WORD16
            as *const core::ffi::c_void,
        (::core::mem::size_of::<WORD16>() as size_t).wrapping_mul(num_nf_bands as size_t),
    );
    memset(
        ((*ptr_hf_gen_str).bw_array_prev).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<WORD32>() as size_t)
            .wrapping_mul(MAX_NUM_PATCHES as size_t),
    );
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_rescale_x_overlap(
    mut ptr_sbr_dec: *mut ia_sbr_dec_struct,
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut ptr_frame_data: *mut ia_sbr_frame_info_data_struct,
    mut ptr_frame_data_prev: *mut ia_sbr_prev_frame_data_struct,
    mut pp_overlap_buffer_real: *mut *mut WORD32,
    mut pp_overlap_buffer_imag: *mut *mut WORD32,
    mut low_pow_flag: FLAG,
) -> VOID {
    let mut k: WORD32 = 0;
    let mut l: WORD32 = 0;
    let mut start_band: WORD32 = 0;
    let mut end_band: WORD32 = 0;
    let mut target_lsb: WORD32 = 0;
    let mut target_usb: WORD32 = 0;
    let mut source_scale: WORD32 = 0;
    let mut target_scale: WORD32 = 0;
    let mut delta_scale: WORD32 = 0;
    let mut reserve: WORD32 = 0;
    let mut old_lsb: WORD32 = (*ptr_frame_data_prev).max_qmf_subband_aac;
    let mut start_slot: WORD32 = (*ptr_header_data).time_step as WORD32
        * ((*ptr_frame_data_prev).end_position as WORD32
            - (*ptr_header_data).num_time_slots as WORD32);
    let mut new_lsb: WORD32 = (*ptr_frame_data).max_qmf_subband_aac;
    (*ptr_sbr_dec).str_codec_qmf_bank.usb = new_lsb as WORD16;
    (*ptr_sbr_dec).str_synthesis_qmf_bank.lsb = new_lsb as WORD16;
    start_band = ixheaac_min32(old_lsb, new_lsb);
    end_band = ixheaac_max32(old_lsb, new_lsb);
    if new_lsb != old_lsb && old_lsb > 0 as core::ffi::c_int {
        l = start_slot;
        while l < 6 as core::ffi::c_int {
            k = old_lsb;
            while k < new_lsb {
                *(*pp_overlap_buffer_real.offset(l as isize)).offset(k as isize) = 0
                    as WORD32;
                if low_pow_flag == 0 {
                    *(*pp_overlap_buffer_imag.offset(l as isize)).offset(k as isize) = 0
                        as WORD32;
                }
                k += 1;
            }
            l += 1;
        }
        if new_lsb > old_lsb {
            source_scale = (*ptr_sbr_dec).str_sbr_scale_fact.ov_hb_scale as WORD32;
            target_scale = (*ptr_sbr_dec).str_sbr_scale_fact.ov_lb_scale as WORD32;
            target_lsb = 0 as core::ffi::c_int as WORD32;
            target_usb = old_lsb;
        } else {
            source_scale = (*ptr_sbr_dec).str_sbr_scale_fact.ov_lb_scale as WORD32;
            target_scale = (*ptr_sbr_dec).str_sbr_scale_fact.ov_hb_scale as WORD32;
            target_lsb = old_lsb;
            target_usb = (*ptr_sbr_dec).str_synthesis_qmf_bank.usb as WORD32;
        }
        reserve = (Some(
            ixheaacd_ixheaacd_expsubbandsamples.expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            pp_overlap_buffer_real,
            pp_overlap_buffer_imag,
            start_band,
            end_band,
            0 as WORD32,
            start_slot,
            low_pow_flag,
        ) as WORD32;
        (Some(ixheaacd_adjust_scale.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            pp_overlap_buffer_real,
            pp_overlap_buffer_imag,
            start_band,
            end_band,
            0 as WORD32,
            start_slot,
            reserve,
            low_pow_flag,
        );
        source_scale += reserve;
        delta_scale = target_scale - source_scale;
        if delta_scale > 0 as core::ffi::c_int {
            delta_scale = -delta_scale;
            start_band = target_lsb;
            end_band = target_usb;
            if new_lsb > old_lsb {
                (*ptr_sbr_dec).str_sbr_scale_fact.ov_lb_scale = source_scale as WORD16;
            } else {
                (*ptr_sbr_dec).str_sbr_scale_fact.ov_hb_scale = source_scale as WORD16;
            }
        }
        (Some(ixheaacd_adjust_scale.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            pp_overlap_buffer_real,
            pp_overlap_buffer_imag,
            start_band,
            end_band,
            0 as WORD32,
            start_slot,
            delta_scale,
            low_pow_flag,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_map_sineflags(
    mut freq_band_table: *mut WORD16,
    mut num_sf_bands: WORD16,
    mut add_harmonics: *mut FLAG,
    mut harm_flags_prev: *mut WORD8,
    mut transient_env: WORD16,
    mut sine_mapped: *mut WORD8,
) -> VOID {
    let mut qmfband2: WORD32 = 0;
    let mut li: WORD32 = 0;
    let mut ui: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut low_subband_sec: WORD32 = 0;
    let mut oldflags: WORD32 = 0;
    low_subband_sec = ((*freq_band_table.offset(0 as core::ffi::c_int as isize)
        as core::ffi::c_int) << 1 as core::ffi::c_int) as WORD32;
    memset(
        sine_mapped as *mut core::ffi::c_void,
        MAX_ENVELOPES,
        (::core::mem::size_of::<WORD8>() as size_t)
            .wrapping_mul(MAX_FREQ_COEFFS as size_t),
    );
    i = (num_sf_bands as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while i >= 0 as core::ffi::c_int {
        oldflags = *harm_flags_prev as WORD32;
        let fresh1 = harm_flags_prev;
        harm_flags_prev = harm_flags_prev.offset(1);
        *fresh1 = *add_harmonics.offset(i as isize) as WORD8;
        if *add_harmonics.offset(i as isize) != 0 {
            li = *freq_band_table.offset(i as isize) as WORD32;
            ui = *freq_band_table
                .offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                as WORD32;
            qmfband2 = ui + li - low_subband_sec >> 1 as core::ffi::c_int;
            if oldflags != 0 {
                *sine_mapped.offset(qmfband2 as isize) = 0 as WORD8;
            } else {
                *sine_mapped.offset(qmfband2 as isize) = transient_env as WORD8;
            }
        }
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_map_34_params_to_20(mut params: *mut WORD16) -> VOID {
    *params.offset(0 as core::ffi::c_int as isize) = ixheaacd_divideby3(
        *params.offset(0 as core::ffi::c_int as isize) as WORD32
            + *params.offset(0 as core::ffi::c_int as isize) as WORD32
            + *params.offset(1 as core::ffi::c_int as isize) as WORD32,
    );
    *params.offset(1 as core::ffi::c_int as isize) = ixheaacd_divideby3(
        *params.offset(1 as core::ffi::c_int as isize) as WORD32
            + *params.offset(2 as core::ffi::c_int as isize) as WORD32
            + *params.offset(2 as core::ffi::c_int as isize) as WORD32,
    );
    *params.offset(2 as core::ffi::c_int as isize) = ixheaacd_divideby3(
        *params.offset(3 as core::ffi::c_int as isize) as WORD32
            + *params.offset(3 as core::ffi::c_int as isize) as WORD32
            + *params.offset(4 as core::ffi::c_int as isize) as WORD32,
    );
    *params.offset(3 as core::ffi::c_int as isize) = ixheaacd_divideby3(
        *params.offset(4 as core::ffi::c_int as isize) as WORD32
            + *params.offset(5 as core::ffi::c_int as isize) as WORD32
            + *params.offset(5 as core::ffi::c_int as isize) as WORD32,
    );
    *params.offset(4 as core::ffi::c_int as isize) = ixheaacd_divideby2(
        *params.offset(6 as core::ffi::c_int as isize) as WORD32
            + *params.offset(7 as core::ffi::c_int as isize) as WORD32,
    );
    *params.offset(5 as core::ffi::c_int as isize) = ixheaacd_divideby2(
        *params.offset(8 as core::ffi::c_int as isize) as WORD32
            + *params.offset(9 as core::ffi::c_int as isize) as WORD32,
    );
    *params.offset(6 as core::ffi::c_int as isize) = *params
        .offset(10 as core::ffi::c_int as isize);
    *params.offset(7 as core::ffi::c_int as isize) = *params
        .offset(11 as core::ffi::c_int as isize);
    *params.offset(8 as core::ffi::c_int as isize) = ixheaacd_divideby2(
        *params.offset(12 as core::ffi::c_int as isize) as WORD32
            + *params.offset(13 as core::ffi::c_int as isize) as WORD32,
    );
    *params.offset(9 as core::ffi::c_int as isize) = ixheaacd_divideby2(
        *params.offset(14 as core::ffi::c_int as isize) as WORD32
            + *params.offset(15 as core::ffi::c_int as isize) as WORD32,
    );
    *params.offset(10 as core::ffi::c_int as isize) = *params
        .offset(16 as core::ffi::c_int as isize);
    *params.offset(11 as core::ffi::c_int as isize) = *params
        .offset(17 as core::ffi::c_int as isize);
    *params.offset(12 as core::ffi::c_int as isize) = *params
        .offset(18 as core::ffi::c_int as isize);
    *params.offset(13 as core::ffi::c_int as isize) = *params
        .offset(19 as core::ffi::c_int as isize);
    *params.offset(14 as core::ffi::c_int as isize) = ixheaacd_divideby2(
        *params.offset(20 as core::ffi::c_int as isize) as WORD32
            + *params.offset(21 as core::ffi::c_int as isize) as WORD32,
    );
    *params.offset(15 as core::ffi::c_int as isize) = ixheaacd_divideby2(
        *params.offset(22 as core::ffi::c_int as isize) as WORD32
            + *params.offset(23 as core::ffi::c_int as isize) as WORD32,
    );
    *params.offset(16 as core::ffi::c_int as isize) = ixheaacd_divideby2(
        *params.offset(24 as core::ffi::c_int as isize) as WORD32
            + *params.offset(25 as core::ffi::c_int as isize) as WORD32,
    );
    *params.offset(17 as core::ffi::c_int as isize) = ixheaacd_divideby2(
        *params.offset(26 as core::ffi::c_int as isize) as WORD32
            + *params.offset(27 as core::ffi::c_int as isize) as WORD32,
    );
    *params.offset(18 as core::ffi::c_int as isize) = ixheaacd_divideby2(
        ixheaacd_divideby2(
            *params.offset(28 as core::ffi::c_int as isize) as WORD32
                + *params.offset(29 as core::ffi::c_int as isize) as WORD32
                + *params.offset(30 as core::ffi::c_int as isize) as WORD32
                + *params.offset(31 as core::ffi::c_int as isize) as WORD32,
        ) as WORD32,
    );
    *params.offset(19 as core::ffi::c_int as isize) = ixheaacd_divideby2(
        *params.offset(32 as core::ffi::c_int as isize) as WORD32
            + *params.offset(33 as core::ffi::c_int as isize) as WORD32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_read_ps_data(
    mut ptr_ps_dec: *mut ia_ps_dec_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut num_bits_left: WORD16,
    mut ps_tables_ptr: *mut ia_ps_tables_struct,
) -> IA_ERRORCODE {
    let mut b: WORD = 0;
    let mut e: WORD = 0;
    let mut temp: WORD = 0;
    let num_env_tab: [WORD16; 4] = [
        0 as core::ffi::c_int as WORD16,
        1 as core::ffi::c_int as WORD16,
        2 as core::ffi::c_int as WORD16,
        4 as core::ffi::c_int as WORD16,
    ];
    let mut cnt_bits: WORD = 0;
    let mut huffman_table: ia_huffman_data_type = 0 as *const UWORD16;
    let mut huffman_df_table: ia_huffman_data_type = 0 as *const UWORD16;
    let mut huffman_dt_table: ia_huffman_data_type = 0 as *const UWORD16;
    let mut enable_ps_header: FLAG = 0;
    if ptr_ps_dec.is_null() {
        return 0 as IA_ERRORCODE;
    }
    cnt_bits = (*it_bit_buff).cnt_bits as WORD;
    enable_ps_header = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD) as FLAG;
    if enable_ps_header != 0 {
        (*ptr_ps_dec).enable_iid = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD)
            as FLAG;
        if (*ptr_ps_dec).enable_iid != 0 {
            (*ptr_ps_dec).iid_mode = ixheaacd_read_bits_buf(it_bit_buff, 3 as WORD)
                as WORD16;
        }
        if (*ptr_ps_dec).iid_mode as core::ffi::c_int > 2 as core::ffi::c_int {
            (*ptr_ps_dec).iid_quant = 1 as core::ffi::c_int as FLAG;
            (*ptr_ps_dec).iid_mode = ((*ptr_ps_dec).iid_mode as core::ffi::c_int
                - 3 as core::ffi::c_int) as WORD16;
        } else {
            (*ptr_ps_dec).iid_quant = 0 as core::ffi::c_int as FLAG;
        }
        (*ptr_ps_dec).enable_icc = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD)
            as FLAG;
        if (*ptr_ps_dec).enable_icc != 0 {
            (*ptr_ps_dec).icc_mode = ixheaacd_read_bits_buf(it_bit_buff, 3 as WORD)
                as WORD16;
        }
        (*ptr_ps_dec).enable_ext = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD)
            as FLAG;
        if (*ptr_ps_dec).icc_mode as core::ffi::c_int > 2 as core::ffi::c_int {
            (*ptr_ps_dec).icc_mode = ((*ptr_ps_dec).icc_mode as core::ffi::c_int
                - 3 as core::ffi::c_int) as WORD16;
            (*ptr_ps_dec).use_pca_rot_flg = 1 as core::ffi::c_int as WORD32;
        } else {
            (*ptr_ps_dec).use_pca_rot_flg = 0 as core::ffi::c_int as WORD32;
        }
        (*ptr_ps_dec).freq_res_ipd = (*ptr_ps_dec).iid_mode as WORD32;
        if (*ptr_ps_dec).freq_res_ipd > 2 as core::ffi::c_int {
            return IA_FATAL_ERROR as IA_ERRORCODE;
        }
    }
    (*ptr_ps_dec).use_34_st_bands = 0 as core::ffi::c_int as WORD32;
    (*ptr_ps_dec).use_pca_rot_flg = 0 as core::ffi::c_int as WORD32;
    if (*ptr_ps_dec).enable_iid != 0
        && (*ptr_ps_dec).iid_mode as core::ffi::c_int > 2 as core::ffi::c_int
        || (*ptr_ps_dec).enable_icc != 0
            && (*ptr_ps_dec).icc_mode as core::ffi::c_int > 2 as core::ffi::c_int
    {
        (*ptr_ps_dec).ps_data_present = 0 as core::ffi::c_int as FLAG;
        num_bits_left = (num_bits_left as core::ffi::c_int
            - (cnt_bits as WORD32 - (*it_bit_buff).cnt_bits) as core::ffi::c_int)
            as WORD16;
        while num_bits_left as core::ffi::c_int > 8 as core::ffi::c_int {
            ixheaacd_read_bits_buf(it_bit_buff, 8 as WORD);
            num_bits_left = (num_bits_left as core::ffi::c_int - 8 as core::ffi::c_int)
                as WORD16;
        }
        if num_bits_left as core::ffi::c_int >= 0 as core::ffi::c_int {
            ixheaacd_read_bits_buf(it_bit_buff, num_bits_left as WORD);
        }
        return cnt_bits as IA_ERRORCODE - (*it_bit_buff).cnt_bits as IA_ERRORCODE;
    }
    (*ptr_ps_dec).frame_class = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
    temp = ixheaacd_read_bits_buf(it_bit_buff, 2 as WORD) as WORD;
    if (*ptr_ps_dec).frame_class == 0 as core::ffi::c_int {
        (*ptr_ps_dec).num_env = num_env_tab[temp as usize];
    } else {
        (*ptr_ps_dec).num_env = (1 as WORD + temp << 8 as core::ffi::c_int
            >> 8 as core::ffi::c_int) as WORD16;
        e = 1 as core::ffi::c_int as WORD;
        while e < (*ptr_ps_dec).num_env as core::ffi::c_int + 1 as core::ffi::c_int {
            (*ptr_ps_dec).border_position[e as usize] = ((ixheaacd_read_bits_buf(
                it_bit_buff,
                5 as WORD,
            ) as core::ffi::c_int + 1 as core::ffi::c_int) << 8 as core::ffi::c_int
                >> 8 as core::ffi::c_int) as WORD16;
            e += 1;
        }
    }
    if (*ptr_ps_dec).enable_iid != 0 {
        if (*ptr_ps_dec).iid_quant != 0 {
            huffman_df_table = &mut (*ps_tables_ptr).huff_iid_df_fine
                as *mut [WORD16; 60] as ia_huffman_data_type;
            huffman_dt_table = &mut (*ps_tables_ptr).huff_iid_dt_fine
                as *mut [WORD16; 60] as ia_huffman_data_type;
        } else {
            huffman_df_table = &mut (*ps_tables_ptr).huff_iid_df as *mut [WORD16; 28]
                as ia_huffman_data_type;
            huffman_dt_table = &mut (*ps_tables_ptr).huff_iid_dt as *mut [WORD16; 28]
                as ia_huffman_data_type;
        }
        e = 0 as core::ffi::c_int as WORD;
        while e < (*ptr_ps_dec).num_env as core::ffi::c_int {
            (*ptr_ps_dec).iid_dt[e as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                1 as WORD,
            );
            if (*ptr_ps_dec).iid_dt[e as usize] != 0 {
                huffman_table = huffman_dt_table;
            } else {
                huffman_table = huffman_df_table;
            }
            b = 0 as core::ffi::c_int as WORD;
            while b
                < ixheaacd_num_bands[(*ptr_ps_dec).iid_mode as usize] as core::ffi::c_int
            {
                (*ptr_ps_dec).iid_par_table[e as usize][b as usize] = ixheaacd_ssc_huff_dec(
                    huffman_table,
                    it_bit_buff,
                ) as WORD16;
                b += 1;
            }
            e += 1;
        }
    }
    if (*ptr_ps_dec).enable_icc != 0 {
        huffman_df_table = &mut (*ps_tables_ptr).huff_icc_df as *mut [WORD16; 14]
            as ia_huffman_data_type;
        huffman_dt_table = &mut (*ps_tables_ptr).huff_icc_dt as *mut [WORD16; 14]
            as ia_huffman_data_type;
        e = 0 as core::ffi::c_int as WORD;
        while e < (*ptr_ps_dec).num_env as core::ffi::c_int {
            (*ptr_ps_dec).icc_dt[e as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                1 as WORD,
            ) as FLAG;
            if (*ptr_ps_dec).icc_dt[e as usize] != 0 {
                huffman_table = huffman_dt_table;
            } else {
                huffman_table = huffman_df_table;
            }
            b = 0 as core::ffi::c_int as WORD;
            while b
                < ixheaacd_num_bands[(*ptr_ps_dec).icc_mode as usize] as core::ffi::c_int
            {
                (*ptr_ps_dec).icc_par_table[e as usize][b as usize] = ixheaacd_ssc_huff_dec(
                    huffman_table,
                    it_bit_buff,
                ) as WORD16;
                b += 1;
            }
            e += 1;
        }
    }
    if (*ptr_ps_dec).enable_ext != 0 {
        let mut cnt: WORD32 = 0;
        if (*it_bit_buff).cnt_bits < 4 as core::ffi::c_int {
            cnt = ixheaacd_read_bits_buf(it_bit_buff, (*it_bit_buff).cnt_bits as WORD);
        } else {
            cnt = ixheaacd_read_bits_buf(it_bit_buff, 4 as WORD);
        }
        if cnt == 15 as core::ffi::c_int {
            cnt += ixheaacd_read_bits_buf(it_bit_buff, 8 as WORD);
        }
        loop {
            let fresh0 = cnt;
            cnt = cnt - 1;
            if !(fresh0 != 0) {
                break;
            }
            ixheaacd_read_bits_buf(it_bit_buff, 8 as WORD);
        }
    }
    (*ptr_ps_dec).ps_data_present = 1 as core::ffi::c_int as FLAG;
    return cnt_bits as IA_ERRORCODE - (*it_bit_buff).cnt_bits as IA_ERRORCODE;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_invfilt_level_emphasis(
    mut ptr_hf_gen_str: *mut ia_sbr_hf_generator_struct,
    mut num_if_bands: WORD32,
    mut inv_filt_mode: *mut WORD32,
    mut inv_filt_mode_prev: *mut WORD32,
    mut bw_array: *mut WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut accu: WORD32 = 0;
    let mut w1: WORD16 = 0;
    let mut w2: WORD16 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_if_bands {
        *bw_array.offset(i as isize) = ixheaacd_inew_bw_table[*inv_filt_mode_prev
            .offset(i as isize) as usize][*inv_filt_mode.offset(i as isize) as usize];
        if *bw_array.offset(i as isize) < (*ptr_hf_gen_str).bw_array_prev[i as usize] {
            w1 = 0x6000 as WORD16;
            w2 = 0x2000 as WORD16;
        } else {
            w1 = 0x7400 as WORD16;
            w2 = 0xc00 as WORD16;
        }
        accu = ixheaac_add32(
            ixheaac_mult32x16in32_shl(*bw_array.offset(i as isize), w1),
            ixheaac_mult32x16in32_shl((*ptr_hf_gen_str).bw_array_prev[i as usize], w2),
        );
        if accu < 0x2000000 as core::ffi::c_int {
            accu = 0 as core::ffi::c_int as WORD32;
        }
        if accu >= 0x7f800000 as core::ffi::c_int {
            accu = 0x7f800000 as core::ffi::c_int as WORD32;
        }
        *bw_array.offset(i as isize) = accu;
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_esbr_calc_co_variance(
    mut pstr_auto_corr: *mut ia_auto_corr_ele_struct,
    mut vec_x_real: *mut [FLOAT32; 64],
    mut vec_x_imag: *mut [FLOAT32; 64],
    mut bd: WORD32,
    mut len: WORD32,
) -> VOID {
    let mut j: WORD32 = 0;
    let mut jminus1: WORD32 = 0;
    let mut jminus2: WORD32 = 0;
    memset(
        pstr_auto_corr as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<ia_auto_corr_ele_struct>() as size_t,
    );
    j = 0 as core::ffi::c_int as WORD32;
    while j < len {
        jminus1 = (j as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        jminus2 = (jminus1 as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        (*pstr_auto_corr).phi_0_1_real
            += (*vec_x_real.offset(j as isize))[bd as usize]
                * (*vec_x_real.offset(jminus1 as isize))[bd as usize]
                + (*vec_x_imag.offset(j as isize))[bd as usize]
                    * (*vec_x_imag.offset(jminus1 as isize))[bd as usize];
        (*pstr_auto_corr).phi_0_1_imag
            += (*vec_x_imag.offset(j as isize))[bd as usize]
                * (*vec_x_real.offset(jminus1 as isize))[bd as usize]
                - (*vec_x_real.offset(j as isize))[bd as usize]
                    * (*vec_x_imag.offset(jminus1 as isize))[bd as usize];
        (*pstr_auto_corr).phi_0_2_real
            += (*vec_x_real.offset(j as isize))[bd as usize]
                * (*vec_x_real.offset(jminus2 as isize))[bd as usize]
                + (*vec_x_imag.offset(j as isize))[bd as usize]
                    * (*vec_x_imag.offset(jminus2 as isize))[bd as usize];
        (*pstr_auto_corr).phi_0_2_imag
            += (*vec_x_imag.offset(j as isize))[bd as usize]
                * (*vec_x_real.offset(jminus2 as isize))[bd as usize]
                - (*vec_x_real.offset(j as isize))[bd as usize]
                    * (*vec_x_imag.offset(jminus2 as isize))[bd as usize];
        (*pstr_auto_corr).phi_1_1
            += (*vec_x_real.offset(jminus1 as isize))[bd as usize]
                * (*vec_x_real.offset(jminus1 as isize))[bd as usize]
                + (*vec_x_imag.offset(jminus1 as isize))[bd as usize]
                    * (*vec_x_imag.offset(jminus1 as isize))[bd as usize];
        (*pstr_auto_corr).phi_1_2_real
            += (*vec_x_real.offset(jminus1 as isize))[bd as usize]
                * (*vec_x_real.offset(jminus2 as isize))[bd as usize]
                + (*vec_x_imag.offset(jminus1 as isize))[bd as usize]
                    * (*vec_x_imag.offset(jminus2 as isize))[bd as usize];
        (*pstr_auto_corr).phi_1_2_imag
            += (*vec_x_imag.offset(jminus1 as isize))[bd as usize]
                * (*vec_x_real.offset(jminus2 as isize))[bd as usize]
                - (*vec_x_real.offset(jminus1 as isize))[bd as usize]
                    * (*vec_x_imag.offset(jminus2 as isize))[bd as usize];
        (*pstr_auto_corr).phi_2_2
            += (*vec_x_real.offset(jminus2 as isize))[bd as usize]
                * (*vec_x_real.offset(jminus2 as isize))[bd as usize]
                + (*vec_x_imag.offset(jminus2 as isize))[bd as usize]
                    * (*vec_x_imag.offset(jminus2 as isize))[bd as usize];
        j += 1;
    }
    (*pstr_auto_corr).det = ((*pstr_auto_corr).phi_1_1 as core::ffi::c_float
        * (*pstr_auto_corr).phi_2_2 as core::ffi::c_float
        - ((*pstr_auto_corr).phi_1_2_real as core::ffi::c_float
            * (*pstr_auto_corr).phi_1_2_real as core::ffi::c_float
            + (*pstr_auto_corr).phi_1_2_imag as core::ffi::c_float
                * (*pstr_auto_corr).phi_1_2_imag as core::ffi::c_float)
            * SBR_HF_RELAXATION_PARAM) as FLOAT32;
}
unsafe extern "C" fn ixheaacd_esbr_chirp_fac_calc(
    mut inv_filt_mode: *mut WORD32,
    mut inv_filt_mode_prev: *mut WORD32,
    mut num_if_bands: WORD32,
    mut bw_array: *mut FLOAT32,
    mut bw_array_prev: *mut FLOAT32,
) {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_if_bands {
        *bw_array.offset(i as isize) = ixheaacd_new_bw_table[*inv_filt_mode_prev
            .offset(i as isize) as usize][*inv_filt_mode.offset(i as isize) as usize];
        if *bw_array.offset(i as isize) < *bw_array_prev.offset(i as isize) {
            *bw_array.offset(i as isize) = 0.75000f32 * *bw_array.offset(i as isize)
                + 0.25000f32 * *bw_array_prev.offset(i as isize);
        } else {
            *bw_array.offset(i as isize) = 0.90625f32 * *bw_array.offset(i as isize)
                + 0.09375f32 * *bw_array_prev.offset(i as isize);
        }
        if (*bw_array.offset(i as isize) as core::ffi::c_double) < 0.015625f64 {
            *bw_array.offset(i as isize) = 0 as core::ffi::c_int as FLOAT32;
        }
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_gausssolve(
    mut n: WORD32,
    mut a: *mut [FLOAT32; 4],
    mut b: *mut FLOAT32,
    mut y: *mut FLOAT32,
) {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut imax: WORD32 = 0;
    let mut v: FLOAT32 = 0.;
    i = 0 as core::ffi::c_int as WORD32;
    while i < n {
        imax = i;
        k = (i as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
        while k < n {
            if fabs((*a.offset(k as isize))[i as usize] as core::ffi::c_double)
                > fabs((*a.offset(imax as isize))[i as usize] as core::ffi::c_double)
            {
                imax = k;
            }
            k += 1;
        }
        if imax != i {
            v = *b.offset(imax as isize);
            *b.offset(imax as isize) = *b.offset(i as isize);
            *b.offset(i as isize) = v;
            j = i;
            while j < n {
                v = (*a.offset(imax as isize))[j as usize];
                (*a.offset(imax as isize))[j as usize] = (*a
                    .offset(i as isize))[j as usize];
                (*a.offset(i as isize))[j as usize] = v;
                j += 1;
            }
        }
        v = (*a.offset(i as isize))[i as usize];
        *b.offset(i as isize) /= v;
        j = i;
        while j < n {
            (*a.offset(i as isize))[j as usize] /= v;
            j += 1;
        }
        k = (i as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
        while k < n {
            v = (*a.offset(k as isize))[i as usize];
            *b.offset(k as isize) -= v * *b.offset(i as isize);
            j = (i as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
            while j < n {
                (*a.offset(k as isize))[j as usize]
                    -= v * (*a.offset(i as isize))[j as usize];
                j += 1;
            }
            k += 1;
        }
        i += 1;
    }
    i = (n as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while i >= 0 as core::ffi::c_int {
        *y.offset(i as isize) = *b.offset(i as isize);
        j = (i as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
        while j < n {
            *y.offset(i as isize)
                -= (*a.offset(i as isize))[j as usize] * *y.offset(j as isize);
            j += 1;
        }
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_polyfit(
    mut n: WORD32,
    mut y: *mut FLOAT32,
    mut p: *mut FLOAT32,
) {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut a: [[FLOAT32; 4]; 4] = [[0.; 4]; 4];
    let mut b: [FLOAT32; 4] = [0.; 4];
    let mut v: [FLOAT32; 7] = [0.; 7];
    i = 0 as core::ffi::c_int as WORD32;
    while i <= MAXDEG {
        b[i as usize] = 0.0f32 as FLOAT32;
        j = 0 as core::ffi::c_int as WORD32;
        while j <= MAXDEG {
            a[i as usize][j as usize] = 0.0f32 as FLOAT32;
            j += 1;
        }
        i += 1;
    }
    k = 0 as core::ffi::c_int as WORD32;
    while k < n {
        v[0 as core::ffi::c_int as usize] = 1.0f32;
        i = 1 as core::ffi::c_int as WORD32;
        while i <= 2 as core::ffi::c_int * MAXDEG {
            v[i as usize] = k as FLOAT32
                * v[(i as core::ffi::c_int - 1 as core::ffi::c_int) as usize];
            i += 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i <= MAXDEG {
            b[i as usize] += v[(MAXDEG - i) as usize] * *y.offset(k as isize);
            j = 0 as core::ffi::c_int as WORD32;
            while j <= MAXDEG {
                a[i as usize][j as usize] += v[(2 as WORD32 * MAXDEG - i - j) as usize];
                j += 1;
            }
            i += 1;
        }
        k += 1;
    }
    ixheaacd_gausssolve(MAXDEG + 1 as WORD32, a.as_mut_ptr(), b.as_mut_ptr(), p);
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_pre_processing(
    mut ptr_src_buf_real: *mut [FLOAT32; 64],
    mut ptr_src_buf_imag: *mut [FLOAT32; 64],
    mut gain_vector: *mut FLOAT32,
    mut num_bands: WORD32,
    mut start_sample: WORD32,
    mut end_sample: WORD32,
) -> VOID {
    let mut k: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut poly_coeff: [FLOAT32; 4] = [0.; 4];
    let mut mean_enrg: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
    let mut low_env_slope: [FLOAT32; 64] = [0.; 64];
    let mut low_env: [FLOAT32; 64] = [
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
    let mut a0: FLOAT32 = 0.;
    let mut a1: FLOAT32 = 0.;
    let mut a2: FLOAT32 = 0.;
    let mut a3: FLOAT32 = 0.;
    if num_bands != 0 as core::ffi::c_int && end_sample != start_sample {
        k = 0 as core::ffi::c_int as WORD32;
        while k < num_bands {
            let mut temp: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
            i = start_sample;
            while i < end_sample {
                temp
                    += (*ptr_src_buf_real.offset(i as isize))[k as usize]
                        * (*ptr_src_buf_real.offset(i as isize))[k as usize]
                        + (*ptr_src_buf_imag.offset(i as isize))[k as usize]
                            * (*ptr_src_buf_imag.offset(i as isize))[k as usize];
                i += 1;
            }
            temp /= (end_sample - start_sample) as FLOAT32;
            low_env[k as usize] = (10 as core::ffi::c_int as core::ffi::c_double
                * log10(
                    (temp + 1 as core::ffi::c_int as FLOAT32) as core::ffi::c_double,
                )) as FLOAT32;
            mean_enrg = mean_enrg + low_env[k as usize];
            k += 1;
        }
        mean_enrg /= num_bands as FLOAT32;
    }
    ixheaacd_polyfit(num_bands, low_env.as_mut_ptr(), poly_coeff.as_mut_ptr());
    a0 = poly_coeff[0 as core::ffi::c_int as usize];
    a1 = poly_coeff[1 as core::ffi::c_int as usize];
    a2 = poly_coeff[2 as core::ffi::c_int as usize];
    a3 = poly_coeff[3 as core::ffi::c_int as usize];
    k = 0 as core::ffi::c_int as WORD32;
    while k < num_bands {
        let mut x_low_l: FLOAT32 = k as FLOAT32;
        let mut low_env_slope_l: FLOAT32 = a3;
        low_env_slope_l = low_env_slope_l + a2 * x_low_l;
        x_low_l = x_low_l * x_low_l;
        low_env_slope_l = low_env_slope_l + a1 * x_low_l;
        x_low_l = x_low_l * k as FLOAT32;
        low_env_slope_l = low_env_slope_l + a0 * x_low_l;
        low_env_slope[k as usize] = low_env_slope_l;
        k += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_bands {
        *gain_vector.offset(i as isize) = pow(
            10 as core::ffi::c_int as core::ffi::c_double,
            ((mean_enrg as core::ffi::c_float - low_env_slope[i as usize]) / 20.0f32)
                as core::ffi::c_double,
        ) as FLOAT32;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_generate_hf(
    mut ptr_src_buf_real: *mut [FLOAT32; 64],
    mut ptr_src_buf_imag: *mut [FLOAT32; 64],
    mut ptr_ph_vocod_buf_real: *mut [FLOAT32; 64],
    mut ptr_ph_vocod_buf_imag: *mut [FLOAT32; 64],
    mut ptr_dst_buf_real: *mut [FLOAT32; 64],
    mut ptr_dst_buf_imag: *mut [FLOAT32; 64],
    mut ptr_frame_data: *mut ia_sbr_frame_info_data_struct,
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut ldmps_present: WORD32,
    mut time_slots: WORD32,
    mut ec_flag: WORD32,
) -> WORD32 {
    let mut bw_index: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut k2: WORD32 = 0;
    let mut patch: WORD32 = 0 as WORD32;
    let mut co_var_len: WORD32 = 0;
    let mut start_sample: WORD32 = 0;
    let mut end_sample: WORD32 = 0;
    let mut goal_sb: WORD32 = 0;
    let mut sb: WORD32 = 0;
    let mut source_start_band: WORD32 = 0;
    let mut patch_stride: WORD32 = 0;
    let mut num_bands_in_patch: WORD32 = 0;
    let mut hbe_flag: WORD32 = (*ptr_header_data).hbe_flag as WORD32;
    let mut a0r: FLOAT32 = 0.;
    let mut a0i: FLOAT32 = 0.;
    let mut a1r: FLOAT32 = 0.;
    let mut a1i: FLOAT32 = 0.;
    let mut bw_array: [FLOAT32; 6] = [
        0 as core::ffi::c_int as FLOAT32,
        0.,
        0.,
        0.,
        0.,
        0.,
    ];
    let mut str_auto_corr: ia_auto_corr_ele_struct = ia_auto_corr_ele_struct {
        phi_0_1_real: 0.,
        phi_0_1_imag: 0.,
        phi_0_2_real: 0.,
        phi_0_2_imag: 0.,
        phi_1_1: 0.,
        phi_1_2_real: 0.,
        phi_1_2_imag: 0.,
        phi_2_2: 0.,
        det: 0.,
    };
    let mut ptr_invf_band_tbl: *mut WORD16 = &mut *((*(*ptr_header_data)
        .pstr_freq_band_data)
        .freq_band_tbl_noise)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as isize) as *mut WORD16;
    let mut num_if_bands: WORD32 = (*(*ptr_header_data).pstr_freq_band_data).num_nf_bands
        as WORD32;
    let mut sub_band_start: WORD32 = (*(*ptr_header_data).pstr_freq_band_data)
        .sub_band_start as WORD32;
    let mut f_master_tbl: *mut WORD16 = ((*(*ptr_header_data).pstr_freq_band_data)
        .f_master_tbl)
        .as_mut_ptr();
    let mut num_mf_bands: WORD32 = (*(*ptr_header_data).pstr_freq_band_data).num_mf_bands
        as WORD32;
    let mut inv_filt_mode: *mut WORD32 = ((*ptr_frame_data).sbr_invf_mode).as_mut_ptr();
    let mut inv_filt_mode_prev: *mut WORD32 = ((*ptr_frame_data).sbr_invf_mode_prev)
        .as_mut_ptr();
    let mut sbr_patching_mode: WORD32 = (*ptr_frame_data).sbr_patching_mode;
    let mut p_frame_info: *mut ia_frame_info_struct = &mut (*ptr_frame_data)
        .str_frame_info_details;
    let mut pre_proc_flag: WORD32 = (*ptr_header_data).pre_proc_flag as WORD32;
    let mut is_usf_4: WORD32 = (*ptr_header_data).is_usf_4;
    let mut fs: WORD32 = (*ptr_header_data).out_sampling_freq;
    let mut cov_count: WORD32 = 0;
    let mut lsb: WORD32 = *f_master_tbl.offset(0 as core::ffi::c_int as isize) as WORD32;
    let mut usb: WORD32 = *f_master_tbl.offset(num_mf_bands as isize) as WORD32;
    let mut xover_offset: WORD32 = sub_band_start
        - *f_master_tbl.offset(0 as core::ffi::c_int as isize) as WORD32;
    let mut bw: FLOAT32 = 0.0f32;
    let mut fac: FLOAT32 = 0.0f32;
    let mut gain: FLOAT32 = 0.;
    let mut gain_vector: [FLOAT32; 64] = [0.; 64];
    let mut slope_length: WORD32 = 0 as WORD32;
    let mut first_slot_offset: WORD32 = (*p_frame_info)
        .border_vec[0 as core::ffi::c_int as usize] as WORD32;
    let mut end_slot_offs: WORD32 = 0 as WORD32;
    let mut bw_array_prev: *mut FLOAT32 = ((*ptr_frame_data).bw_array_prev).as_mut_ptr();
    end_slot_offs = ((*p_frame_info).border_vec[(*p_frame_info).num_env as usize]
        as core::ffi::c_int - 16 as core::ffi::c_int) as WORD32;
    if ldmps_present == 1 as core::ffi::c_int {
        end_slot_offs = (*p_frame_info).border_vec[(*p_frame_info).num_env as usize]
            as WORD32 - time_slots;
    }
    if is_usf_4 != 0 {
        start_sample = (first_slot_offset as core::ffi::c_int * 4 as core::ffi::c_int)
            as WORD32;
        end_sample = (64 as core::ffi::c_int
            + end_slot_offs as core::ffi::c_int * 4 as core::ffi::c_int) as WORD32;
        co_var_len = 76 as core::ffi::c_int as WORD32;
    } else {
        start_sample = (first_slot_offset as core::ffi::c_int * 2 as core::ffi::c_int)
            as WORD32;
        end_sample = (32 as core::ffi::c_int
            + end_slot_offs as core::ffi::c_int * 2 as core::ffi::c_int) as WORD32;
        co_var_len = 38 as core::ffi::c_int as WORD32;
    }
    if ldmps_present == 1 as core::ffi::c_int {
        start_sample = 0 as core::ffi::c_int as WORD32;
        end_sample = time_slots;
        co_var_len = time_slots;
    }
    if pre_proc_flag != 0 {
        ixheaacd_pre_processing(
            ptr_src_buf_real,
            ptr_src_buf_imag,
            gain_vector.as_mut_ptr(),
            *f_master_tbl.offset(0 as core::ffi::c_int as isize) as WORD32,
            start_sample,
            end_sample,
        );
    }
    ixheaacd_esbr_chirp_fac_calc(
        inv_filt_mode,
        inv_filt_mode_prev,
        num_if_bands,
        bw_array.as_mut_ptr(),
        bw_array_prev,
    );
    i = start_sample;
    while i < end_sample {
        memset(
            (*ptr_dst_buf_real.offset(i as isize)).as_mut_ptr().offset(usb as isize)
                as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ((64 as WORD32 - usb) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        memset(
            (*ptr_dst_buf_imag.offset(i as isize)).as_mut_ptr().offset(usb as isize)
                as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ((64 as WORD32 - usb) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        i += 1;
    }
    if sbr_patching_mode != 0 || hbe_flag == 0 {
        let mut flag_break: WORD32 = 0 as WORD32;
        let mut alpha_real: [[FLOAT32; 2]; 64] = [
            [0 as core::ffi::c_int as FLOAT32, 0.],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
        ];
        let mut alpha_imag: [[FLOAT32; 2]; 64] = [
            [0 as core::ffi::c_int as FLOAT32, 0.],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
        ];
        if (*ptr_frame_data).mps_sbr_flag != 0 {
            cov_count = (if (*f_master_tbl.offset(0 as core::ffi::c_int as isize)
                as core::ffi::c_int) < (*ptr_frame_data).cov_count
            {
                *f_master_tbl.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
            } else {
                (*ptr_frame_data).cov_count as core::ffi::c_int
            }) as WORD32;
        } else {
            cov_count = *f_master_tbl.offset(0 as core::ffi::c_int as isize) as WORD32;
        }
        k = 1 as core::ffi::c_int as WORD32;
        while k < cov_count {
            ixheaacd_esbr_calc_co_variance(
                &mut str_auto_corr,
                &mut *ptr_src_buf_real.offset(0 as core::ffi::c_int as isize),
                &mut *ptr_src_buf_imag.offset(0 as core::ffi::c_int as isize),
                k,
                co_var_len,
            );
            if str_auto_corr.det == 0.0f32 {
                alpha_imag[k as usize][1 as core::ffi::c_int as usize] = 0
                    as core::ffi::c_int as FLOAT32;
                alpha_real[k as usize][1 as core::ffi::c_int as usize] = alpha_imag[k
                    as usize][1 as core::ffi::c_int as usize];
            } else {
                fac = 1.0f32 / str_auto_corr.det;
                alpha_real[k as usize][1 as core::ffi::c_int as usize] = (str_auto_corr
                    .phi_0_1_real * str_auto_corr.phi_1_2_real
                    - str_auto_corr.phi_0_1_imag * str_auto_corr.phi_1_2_imag
                    - str_auto_corr.phi_0_2_real * str_auto_corr.phi_1_1) * fac;
                alpha_imag[k as usize][1 as core::ffi::c_int as usize] = (str_auto_corr
                    .phi_0_1_imag * str_auto_corr.phi_1_2_real
                    + str_auto_corr.phi_0_1_real * str_auto_corr.phi_1_2_imag
                    - str_auto_corr.phi_0_2_imag * str_auto_corr.phi_1_1) * fac;
            }
            if str_auto_corr.phi_1_1 == 0 as core::ffi::c_int as FLOAT32 {
                alpha_imag[k as usize][0 as core::ffi::c_int as usize] = 0
                    as core::ffi::c_int as FLOAT32;
                alpha_real[k as usize][0 as core::ffi::c_int as usize] = alpha_imag[k
                    as usize][0 as core::ffi::c_int as usize];
            } else {
                fac = 1.0f32 / str_auto_corr.phi_1_1;
                alpha_real[k as usize][0 as core::ffi::c_int as usize] = -(str_auto_corr
                    .phi_0_1_real
                    + alpha_real[k as usize][1 as core::ffi::c_int as usize]
                        * str_auto_corr.phi_1_2_real
                    + alpha_imag[k as usize][1 as core::ffi::c_int as usize]
                        * str_auto_corr.phi_1_2_imag) * fac;
                alpha_imag[k as usize][0 as core::ffi::c_int as usize] = -(str_auto_corr
                    .phi_0_1_imag
                    + alpha_imag[k as usize][1 as core::ffi::c_int as usize]
                        * str_auto_corr.phi_1_2_real
                    - alpha_real[k as usize][1 as core::ffi::c_int as usize]
                        * str_auto_corr.phi_1_2_imag) * fac;
            }
            if alpha_real[k as usize][0 as core::ffi::c_int as usize]
                * alpha_real[k as usize][0 as core::ffi::c_int as usize]
                + alpha_imag[k as usize][0 as core::ffi::c_int as usize]
                    * alpha_imag[k as usize][0 as core::ffi::c_int as usize] >= 16.0f32
                || alpha_real[k as usize][1 as core::ffi::c_int as usize]
                    * alpha_real[k as usize][1 as core::ffi::c_int as usize]
                    + alpha_imag[k as usize][1 as core::ffi::c_int as usize]
                        * alpha_imag[k as usize][1 as core::ffi::c_int as usize]
                    >= 16.0f32
            {
                alpha_real[k as usize][0 as core::ffi::c_int as usize] = 0.0f32
                    as FLOAT32;
                alpha_imag[k as usize][0 as core::ffi::c_int as usize] = 0.0f32
                    as FLOAT32;
                alpha_real[k as usize][1 as core::ffi::c_int as usize] = 0.0f32
                    as FLOAT32;
                alpha_imag[k as usize][1 as core::ffi::c_int as usize] = 0.0f32
                    as FLOAT32;
            }
            k += 1;
        }
        goal_sb = (2.048e6f32 / fs as core::ffi::c_float + 0.5f32) as WORD32;
        let mut index: WORD32 = 0;
        if goal_sb < *f_master_tbl.offset(num_mf_bands as isize) as core::ffi::c_int {
            index = 0 as core::ffi::c_int as WORD32;
            while (*f_master_tbl.offset(index as isize) as core::ffi::c_int) < goal_sb {
                index += 1;
            }
            goal_sb = *f_master_tbl.offset(index as isize) as WORD32;
        } else {
            goal_sb = *f_master_tbl.offset(num_mf_bands as isize) as WORD32;
        }
        source_start_band = (xover_offset as core::ffi::c_int + 1 as core::ffi::c_int)
            as WORD32;
        sb = lsb + xover_offset;
        patch = 0 as core::ffi::c_int as WORD32;
        while sb < usb {
            if MAX_NUM_PATCHES <= patch {
                if ec_flag != 0 {
                    break;
                }
                return -(1 as WORD32);
            } else {
                (*ptr_frame_data).patch_param.start_subband[patch as usize] = sb;
                num_bands_in_patch = goal_sb - sb;
                if num_bands_in_patch >= lsb - source_start_band {
                    patch_stride = sb - source_start_band;
                    patch_stride = (patch_stride as core::ffi::c_int
                        & !(1 as core::ffi::c_int)) as WORD32;
                    num_bands_in_patch = lsb - (sb - patch_stride);
                    num_bands_in_patch = ixheaacd_find_closest_entry(
                        sb + num_bands_in_patch,
                        f_master_tbl,
                        num_mf_bands as WORD16,
                        0 as WORD16,
                    ) as WORD32 - sb;
                }
                patch_stride = num_bands_in_patch + sb - lsb;
                patch_stride = (patch_stride as core::ffi::c_int + 1 as core::ffi::c_int
                    & !(1 as core::ffi::c_int)) as WORD32;
                source_start_band = 1 as core::ffi::c_int as WORD32;
                if goal_sb - (sb + num_bands_in_patch) < 3 as core::ffi::c_int {
                    goal_sb = usb;
                }
                if num_bands_in_patch < 3 as core::ffi::c_int
                    && patch > 0 as core::ffi::c_int && sb + num_bands_in_patch == usb
                {
                    i = start_sample + slope_length;
                    while i < end_sample + slope_length {
                        k2 = sb;
                        while k2 < sb + num_bands_in_patch {
                            (*ptr_dst_buf_real.offset(i as isize))[k2 as usize] = 0.0f32
                                as FLOAT32;
                            (*ptr_dst_buf_imag.offset(i as isize))[k2 as usize] = 0.0f32
                                as FLOAT32;
                            k2 += 1;
                        }
                        i += 1;
                    }
                    break;
                } else {
                    if num_bands_in_patch < 0 as core::ffi::c_int
                        && flag_break == 1 as core::ffi::c_int
                    {
                        break;
                    }
                    if num_bands_in_patch < 0 as core::ffi::c_int {
                        flag_break = 1 as core::ffi::c_int as WORD32;
                    } else {
                        flag_break = 0 as core::ffi::c_int as WORD32;
                        k2 = sb;
                        while k2 < sb + num_bands_in_patch {
                            k = k2 - patch_stride;
                            bw_index = 0 as core::ffi::c_int as WORD32;
                            while k2
                                >= *ptr_invf_band_tbl.offset(bw_index as isize)
                                    as core::ffi::c_int
                            {
                                bw_index += 1;
                                if !(bw_index >= MAX_NOISE_COEFFS) {
                                    continue;
                                }
                                if ec_flag != 0 {
                                    bw_index = (MAX_NOISE_COEFFS - 1 as core::ffi::c_int)
                                        as WORD32;
                                    break;
                                } else {
                                    return -(1 as WORD32)
                                }
                            }
                            if bw_index >= MAX_NUM_PATCHES {
                                if ec_flag != 0 {
                                    bw_index = (MAX_NUM_PATCHES - 1 as core::ffi::c_int)
                                        as WORD32;
                                } else {
                                    return -(1 as WORD32)
                                }
                            }
                            bw = bw_array[bw_index as usize];
                            a0r = bw
                                * alpha_real[k as usize][0 as core::ffi::c_int as usize];
                            a0i = bw
                                * alpha_imag[k as usize][0 as core::ffi::c_int as usize];
                            bw *= bw;
                            a1r = bw
                                * alpha_real[k as usize][1 as core::ffi::c_int as usize];
                            a1i = bw
                                * alpha_imag[k as usize][1 as core::ffi::c_int as usize];
                            if pre_proc_flag != 0 {
                                gain = gain_vector[k as usize];
                            } else {
                                gain = 1.0f32 as FLOAT32;
                            }
                            i = start_sample + slope_length;
                            while i < end_sample + slope_length {
                                (*ptr_dst_buf_real.offset(i as isize))[k2 as usize] = (*ptr_src_buf_real
                                    .offset(i as isize))[k as usize] * gain;
                                (*ptr_dst_buf_imag.offset(i as isize))[k2 as usize] = (*ptr_src_buf_imag
                                    .offset(i as isize))[k as usize] * gain;
                                if bw > 0.0f32 {
                                    (*ptr_dst_buf_real.offset(i as isize))[k2 as usize]
                                        += (a0r
                                            * (*ptr_src_buf_real
                                                .offset(
                                                    (i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                                                ))[k as usize]
                                            - a0i
                                                * (*ptr_src_buf_imag
                                                    .offset(
                                                        (i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                                                    ))[k as usize]
                                            + a1r
                                                * (*ptr_src_buf_real
                                                    .offset(
                                                        (i as core::ffi::c_int - 2 as core::ffi::c_int) as isize,
                                                    ))[k as usize]
                                            - a1i
                                                * (*ptr_src_buf_imag
                                                    .offset(
                                                        (i as core::ffi::c_int - 2 as core::ffi::c_int) as isize,
                                                    ))[k as usize]) * gain;
                                    (*ptr_dst_buf_imag.offset(i as isize))[k2 as usize]
                                        += (a0i
                                            * (*ptr_src_buf_real
                                                .offset(
                                                    (i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                                                ))[k as usize]
                                            + a0r
                                                * (*ptr_src_buf_imag
                                                    .offset(
                                                        (i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                                                    ))[k as usize]
                                            + a1i
                                                * (*ptr_src_buf_real
                                                    .offset(
                                                        (i as core::ffi::c_int - 2 as core::ffi::c_int) as isize,
                                                    ))[k as usize]
                                            + a1r
                                                * (*ptr_src_buf_imag
                                                    .offset(
                                                        (i as core::ffi::c_int - 2 as core::ffi::c_int) as isize,
                                                    ))[k as usize]) * gain;
                                }
                                i += 1;
                            }
                            k2 += 1;
                        }
                        sb += num_bands_in_patch;
                        patch += 1;
                    }
                }
            }
        }
    }
    if !ptr_ph_vocod_buf_real.is_null() && !ptr_ph_vocod_buf_imag.is_null() {
        if hbe_flag != 0 && sbr_patching_mode == 0 {
            let mut alpha_real_0: [FLOAT32; 2] = [0.; 2];
            let mut alpha_imag_0: [FLOAT32; 2] = [0.; 2];
            bw_index = 0 as core::ffi::c_int as WORD32;
            patch = 1 as core::ffi::c_int as WORD32;
            k2 = sub_band_start;
            while k2 < *f_master_tbl.offset(num_mf_bands as isize) as core::ffi::c_int {
                ixheaacd_esbr_calc_co_variance(
                    &mut str_auto_corr,
                    &mut *ptr_ph_vocod_buf_real.offset(0 as core::ffi::c_int as isize),
                    &mut *ptr_ph_vocod_buf_imag.offset(0 as core::ffi::c_int as isize),
                    k2,
                    co_var_len,
                );
                if str_auto_corr.det == 0.0f32 {
                    alpha_imag_0[1 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
                        as FLOAT32;
                    alpha_real_0[1 as core::ffi::c_int as usize] = alpha_imag_0[1
                        as core::ffi::c_int as usize];
                } else {
                    fac = 1.0f32 / str_auto_corr.det;
                    alpha_real_0[1 as core::ffi::c_int as usize] = (str_auto_corr
                        .phi_0_1_real * str_auto_corr.phi_1_2_real
                        - str_auto_corr.phi_0_1_imag * str_auto_corr.phi_1_2_imag
                        - str_auto_corr.phi_0_2_real * str_auto_corr.phi_1_1) * fac;
                    alpha_imag_0[1 as core::ffi::c_int as usize] = (str_auto_corr
                        .phi_0_1_imag * str_auto_corr.phi_1_2_real
                        + str_auto_corr.phi_0_1_real * str_auto_corr.phi_1_2_imag
                        - str_auto_corr.phi_0_2_imag * str_auto_corr.phi_1_1) * fac;
                }
                if str_auto_corr.phi_1_1 == 0 as core::ffi::c_int as FLOAT32 {
                    alpha_imag_0[0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
                        as FLOAT32;
                    alpha_real_0[0 as core::ffi::c_int as usize] = alpha_imag_0[0
                        as core::ffi::c_int as usize];
                } else {
                    fac = 1.0f32 / str_auto_corr.phi_1_1;
                    alpha_real_0[0 as core::ffi::c_int as usize] = -(str_auto_corr
                        .phi_0_1_real
                        + alpha_real_0[1 as core::ffi::c_int as usize]
                            * str_auto_corr.phi_1_2_real
                        + alpha_imag_0[1 as core::ffi::c_int as usize]
                            * str_auto_corr.phi_1_2_imag) * fac;
                    alpha_imag_0[0 as core::ffi::c_int as usize] = -(str_auto_corr
                        .phi_0_1_imag
                        + alpha_imag_0[1 as core::ffi::c_int as usize]
                            * str_auto_corr.phi_1_2_real
                        - alpha_real_0[1 as core::ffi::c_int as usize]
                            * str_auto_corr.phi_1_2_imag) * fac;
                }
                if alpha_real_0[0 as core::ffi::c_int as usize]
                    * alpha_real_0[0 as core::ffi::c_int as usize]
                    + alpha_imag_0[0 as core::ffi::c_int as usize]
                        * alpha_imag_0[0 as core::ffi::c_int as usize] >= 16.0f32
                    || alpha_real_0[1 as core::ffi::c_int as usize]
                        * alpha_real_0[1 as core::ffi::c_int as usize]
                        + alpha_imag_0[1 as core::ffi::c_int as usize]
                            * alpha_imag_0[1 as core::ffi::c_int as usize] >= 16.0f32
                {
                    alpha_real_0[0 as core::ffi::c_int as usize] = 0.0f32 as FLOAT32;
                    alpha_imag_0[0 as core::ffi::c_int as usize] = 0.0f32 as FLOAT32;
                    alpha_real_0[1 as core::ffi::c_int as usize] = 0.0f32 as FLOAT32;
                    alpha_imag_0[1 as core::ffi::c_int as usize] = 0.0f32 as FLOAT32;
                }
                while k2
                    >= *ptr_invf_band_tbl.offset(bw_index as isize) as core::ffi::c_int
                {
                    bw_index += 1;
                    if !(bw_index >= MAX_NOISE_COEFFS) {
                        continue;
                    }
                    if ec_flag != 0 {
                        bw_index = (MAX_NOISE_COEFFS - 1 as core::ffi::c_int) as WORD32;
                        break;
                    } else {
                        return -(1 as WORD32)
                    }
                }
                if bw_index >= MAX_NUM_PATCHES {
                    if ec_flag != 0 {
                        bw_index = (MAX_NUM_PATCHES - 1 as core::ffi::c_int) as WORD32;
                    } else {
                        return -(1 as WORD32)
                    }
                }
                bw = bw_array[bw_index as usize];
                a0r = bw * alpha_real_0[0 as core::ffi::c_int as usize];
                a0i = bw * alpha_imag_0[0 as core::ffi::c_int as usize];
                bw *= bw;
                a1r = bw * alpha_real_0[1 as core::ffi::c_int as usize];
                a1i = bw * alpha_imag_0[1 as core::ffi::c_int as usize];
                if bw > 0.0f32 {
                    i = start_sample;
                    while i < end_sample {
                        let mut real1: FLOAT32 = 0.;
                        let mut imag1: FLOAT32 = 0.;
                        let mut real2: FLOAT32 = 0.;
                        let mut imag2: FLOAT32 = 0.;
                        let mut realTarget: FLOAT32 = 0.;
                        let mut imag_target: FLOAT32 = 0.;
                        realTarget = (*ptr_ph_vocod_buf_real
                            .offset(i as isize))[k2 as usize];
                        imag_target = (*ptr_ph_vocod_buf_imag
                            .offset(i as isize))[k2 as usize];
                        real1 = (*ptr_ph_vocod_buf_real
                            .offset(
                                (i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                            ))[k2 as usize];
                        imag1 = (*ptr_ph_vocod_buf_imag
                            .offset(
                                (i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                            ))[k2 as usize];
                        real2 = (*ptr_ph_vocod_buf_real
                            .offset(
                                (i as core::ffi::c_int - 2 as core::ffi::c_int) as isize,
                            ))[k2 as usize];
                        imag2 = (*ptr_ph_vocod_buf_imag
                            .offset(
                                (i as core::ffi::c_int - 2 as core::ffi::c_int) as isize,
                            ))[k2 as usize];
                        realTarget
                            += a0r * real1 - a0i * imag1 + (a1r * real2 - a1i * imag2);
                        imag_target
                            += a0i * real1 + a0r * imag1 + (a1i * real2 + a1r * imag2);
                        (*ptr_dst_buf_real.offset(i as isize))[k2 as usize] = realTarget;
                        (*ptr_dst_buf_imag.offset(i as isize))[k2 as usize] = imag_target;
                        i += 1;
                    }
                } else {
                    i = start_sample;
                    while i < end_sample {
                        (*ptr_dst_buf_real.offset(i as isize))[k2 as usize] = (*ptr_ph_vocod_buf_real
                            .offset(i as isize))[k2 as usize];
                        (*ptr_dst_buf_imag.offset(i as isize))[k2 as usize] = (*ptr_ph_vocod_buf_imag
                            .offset(i as isize))[k2 as usize];
                        i += 1;
                    }
                }
                k2 += 1;
            }
        }
    }
    if MAX_NUM_PATCHES + 1 as core::ffi::c_int <= patch {
        if ec_flag != 0 {
            patch = MAX_NUM_PATCHES as WORD32;
        } else {
            return -(1 as WORD32)
        }
    }
    (*ptr_frame_data).patch_param.num_patches = patch;
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_if_bands {
        *bw_array_prev.offset(i as isize) = bw_array[i as usize];
        i += 1;
    }
    return 0 as WORD32;
}
pub const MAX_NUM_PATCHES: core::ffi::c_int = 6 as core::ffi::c_int;
pub const GUARDBANDS: core::ffi::c_int = 0 as core::ffi::c_int;
pub const SHIFT_START_SB: core::ffi::c_int = 1 as core::ffi::c_int;
pub const MAX_16: WORD16 = 0x7fff as core::ffi::c_int as WORD16;
pub const SBR_AMPLITUDE_RESOLUTION_1_5: core::ffi::c_int = 0 as core::ffi::c_int;
pub const LOW: core::ffi::c_int = 0 as core::ffi::c_int;
pub const DTDF_DIR_TIME: core::ffi::c_int = 1 as core::ffi::c_int;
pub const MAXDEG: core::ffi::c_int = 3 as core::ffi::c_int;
pub const SBR_HF_RELAXATION_PARAM: core::ffi::c_float = 0.999999f32;
pub const SBR_ENERGY_PAN_OFFSET: core::ffi::c_int = 12 as core::ffi::c_int;
