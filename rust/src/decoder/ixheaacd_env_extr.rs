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
    fn ixheaacd_read_bits_buf(
        it_bit_buff: *mut ia_bit_buf_struct,
        no_of_bits: WORD,
    ) -> WORD32;
    fn ixheaacd_show_bits_buf(
        it_bit_buff: *mut ia_bit_buf_struct,
        no_of_bits: WORD,
    ) -> WORD32;
    fn ixheaacd_read_ps_data(
        ptr_ps_dec: *mut ia_ps_dec_struct,
        it_bit_buff: *mut ia_bit_buf_struct,
        n_bits_left: WORD16,
        ps_tables_ptr: *mut ia_ps_tables_struct,
    ) -> IA_ERRORCODE;
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
pub type C2RustUnnamed = core::ffi::c_uint;
pub const SBR_ID_END: C2RustUnnamed = 8;
pub const SBR_ID_FIL: C2RustUnnamed = 7;
pub const SBR_ID_PCE: C2RustUnnamed = 6;
pub const SBR_ID_DSE: C2RustUnnamed = 5;
pub const SBR_ID_LFE: C2RustUnnamed = 4;
pub const SBR_ID_LCS: C2RustUnnamed = 3;
pub const SBR_ID_CCE: C2RustUnnamed = 2;
pub const SBR_ID_CPE: C2RustUnnamed = 1;
pub const SBR_ID_SCE: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const PVC_SBR: C2RustUnnamed_0 = 2;
pub const ORIG_SBR: C2RustUnnamed_0 = 1;
pub const UNKNOWN_SBR: C2RustUnnamed_0 = 0;
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
pub const SBR_UPSAMPLE_FAC: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SBR_UPSAMPLE_IDX_2_1: core::ffi::c_int = 1 as core::ffi::c_int;
pub const MAX_FRAME_SIZE: core::ffi::c_int = 1024 as core::ffi::c_int;
pub const NO_SYNTHESIS_CHANNELS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const NO_ANALYSIS_CHANNELS: core::ffi::c_int = NO_SYNTHESIS_CHANNELS
    / SBR_UPSAMPLE_FAC;
pub const MAX_NOISE_ENVELOPES: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_ENVELOPES: core::ffi::c_int = 8 as core::ffi::c_int;
pub const MAX_FREQ_COEFFS: core::ffi::c_int = 56 as core::ffi::c_int;
pub const SBR_TIME_STEP: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_NUM_ENVELOPE_VALUES: core::ffi::c_int = MAX_ENVELOPES * MAX_FREQ_COEFFS;
pub const MAX_COLS: core::ffi::c_int = MAX_FRAME_SIZE / NO_ANALYSIS_CHANNELS;
pub const MAX_OV_COLS: core::ffi::c_int = 6 as core::ffi::c_int;
pub const SBR_TIME_SLOTS: core::ffi::c_int = MAX_COLS / SBR_TIME_STEP;
pub const SBR_OV_SLOTS: core::ffi::c_int = MAX_OV_COLS / SBR_TIME_STEP;
#[inline]
unsafe extern "C" fn ixheaac_add16(mut op1: WORD16, mut op2: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = (op1 as core::ffi::c_int + op2 as core::ffi::c_int) as WORD16;
    return var_out;
}
pub const IA_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_FATAL_ERROR: core::ffi::c_uint = 0x80000000 as core::ffi::c_uint;
pub const SBR_AMPLITUDE_RESOLUTION_1_5: core::ffi::c_int = 0 as core::ffi::c_int;
pub const SBR_AMPLITUDE_RESOLUTION_3_0: core::ffi::c_int = 1 as core::ffi::c_int;
pub const HIGH: core::ffi::c_int = 1 as core::ffi::c_int;
pub const DTDF_DIR_FREQ: core::ffi::c_int = 0 as core::ffi::c_int;
pub const SBR_SAMP_FEQ_LVL_DEF: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SBR_CHANGE_LVL_DEF: core::ffi::c_int = 1 as core::ffi::c_int;
pub const SBR_NOISE_BND_DEF: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SBR_BND_LIMIT_DEF: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SBR_GAIN_LIMIT_DEF: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SBR_INTERPOL_SAMP_FEQ_DEF: core::ffi::c_int = 1 as core::ffi::c_int;
pub const SBR_SMOOTH_LEN_DEF: core::ffi::c_int = 1 as core::ffi::c_int;
pub const SBR_AMPLITUDE_RESOLUTION_BITS: core::ffi::c_int = 1 as core::ffi::c_int;
pub const SBR_BEGIN_SAMP_FREQ_BITS: core::ffi::c_int = 4 as core::ffi::c_int;
pub const SBR_END_SAMP_FREQ_BITS: core::ffi::c_int = 4 as core::ffi::c_int;
pub const SBR_CROSS_OVER_BND_BITS: core::ffi::c_int = 3 as core::ffi::c_int;
pub const ESBR_CROSS_OVER_BND_BITS: core::ffi::c_int = 4 as core::ffi::c_int;
pub const ESBR_PRE_FLAT_BITS: core::ffi::c_int = 1 as core::ffi::c_int;
pub const ESBR_PVC_MODE_BITS: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SBR_HDR_EXTR_1_BITS: core::ffi::c_int = 1 as core::ffi::c_int;
pub const SBR_HDR_EXTR_2_BITS: core::ffi::c_int = 1 as core::ffi::c_int;
pub const SBR_SAMP_FREQ_LVL_BITS: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SBR_CHANGE_LVL_BITS: core::ffi::c_int = 1 as core::ffi::c_int;
pub const SBR_NOISE_BND_BITS: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SBR_BND_LIMIT_BITS: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SBR_GAIN_LIMIT_BITS: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SBR_INTERPOL_SAMP_FREQ_BITS: core::ffi::c_int = 1 as core::ffi::c_int;
pub const SBR_SMOOTH_LEN_BITS: core::ffi::c_int = 1 as core::ffi::c_int;
pub const SBR_HDR_RESERV_BITS: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SBR_SCE_RESERV_BITS: core::ffi::c_int = 4 as core::ffi::c_int;
pub const SBR_COUPLNG_MODE_BITS: core::ffi::c_int = 1 as core::ffi::c_int;
pub const SBR_INVERSE_FILT_MODE_BITS: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SBR_ENLARGED_DATA_BITS: core::ffi::c_int = 1 as core::ffi::c_int;
pub const SBR_CONT_SIZE_BITS: core::ffi::c_int = 4 as core::ffi::c_int;
pub const SBR_CONT_ESC_CNT_BITS: core::ffi::c_int = 8 as core::ffi::c_int;
pub const SBR_CONT_ID_BITS: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SBR_DEL_COD_DIR_BITS: core::ffi::c_int = 1 as core::ffi::c_int;
pub const SBR_ADD_SINE_FLAG_BITS: core::ffi::c_int = 1 as core::ffi::c_int;
pub const SBR_BEGIN_ENVN_BITS_AMPLITUDE_RESOLUTION_3_0: core::ffi::c_int = 6
    as core::ffi::c_int;
pub const SBR_BEGIN_ENVN_BITS_BALNCE_AMPLITUDE_RESOLUTION_3_0: core::ffi::c_int = 5
    as core::ffi::c_int;
pub const SBR_BEGIN_NOISE_BITS_AMPLITUDE_RESOLUTION_3_0: core::ffi::c_int = 5
    as core::ffi::c_int;
pub const SBR_BEGIN_NOISE_BITS_BALNCE_AMPLITUDE_RESOLUTION_3_0: core::ffi::c_int = 5
    as core::ffi::c_int;
pub const SBR_BEGIN_ENVN_BITS_AMPLITUDE_RESOLUTION_1_5: core::ffi::c_int = 7
    as core::ffi::c_int;
pub const SBR_BEGIN_ENVN_BITS_BALNCE_AMPLITUDE_RESOLUTION_1_5: core::ffi::c_int = 6
    as core::ffi::c_int;
pub const SBR_FRAME_CLASS_BITS: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SBR_VAR_BORD_BITS: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SBR_FRQ_RES_BITS: core::ffi::c_int = 1 as core::ffi::c_int;
pub const ESBR_PATCHING_MODE_BITS: core::ffi::c_int = 1 as core::ffi::c_int;
pub const ESBR_OVERSAMPLING_FLAG_BITS: core::ffi::c_int = 1 as core::ffi::c_int;
pub const ESBR_PITCHIN_FLAG_BITS: core::ffi::c_int = 1 as core::ffi::c_int;
pub const ESBR_PITCHIN_BINS_BITS: core::ffi::c_int = 7 as core::ffi::c_int;
pub const ESBR_INVF_MODE_BITS: core::ffi::c_int = 2 as core::ffi::c_int;
pub const ESBR_DOMAIN_BITS: core::ffi::c_int = 1 as core::ffi::c_int;
pub const PVC_NUM_TIME_SLOTS: core::ffi::c_int = 16 as core::ffi::c_int;
pub const PVC_DIV_MODE_BITS: core::ffi::c_int = 3 as core::ffi::c_int;
pub const PVC_NS_MODE_BITS: core::ffi::c_int = 1 as core::ffi::c_int;
pub const PVC_GRID_INFO_BITS: core::ffi::c_int = 1 as core::ffi::c_int;
pub const PVC_REUSE_PVC_ID_BITS: core::ffi::c_int = 1 as core::ffi::c_int;
pub const PVC_ID_BITS: core::ffi::c_int = 7 as core::ffi::c_int;
pub const ESC_SIN_POS: core::ffi::c_int = 31 as core::ffi::c_int;
pub const LD_TRAN: core::ffi::c_int = 1;
pub const LD_ENV_TBL_480: core::ffi::c_int = 15 as core::ffi::c_int;
pub const SBR_TRAN_BITS: core::ffi::c_int = 4 as core::ffi::c_int;
pub const SBRLD_CLA_BITS: core::ffi::c_int = 1 as core::ffi::c_int;
pub const SBR_ENVT_NUMENV: core::ffi::c_int = 0 as core::ffi::c_int;
pub const SBR_ENVT_TRANIDX: core::ffi::c_int = 3 as core::ffi::c_int;
pub const SBR_RES_BITS: core::ffi::c_int = 1 as core::ffi::c_int;
pub const SBR_REL_BITS: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SBR_ENV_BITS: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SBR_NUM_BITS: core::ffi::c_int = 2 as core::ffi::c_int;
pub const FIXFIX: core::ffi::c_int = 0 as core::ffi::c_int;
pub const FIXVAR: core::ffi::c_int = 1 as core::ffi::c_int;
pub const VARFIX: core::ffi::c_int = 2 as core::ffi::c_int;
pub const VARVAR: core::ffi::c_int = 3;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const UPSAMPLING: core::ffi::c_int = 1 as core::ffi::c_int;
pub const SBR_ACTIVE: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SBR_MONO: core::ffi::c_int = 1 as core::ffi::c_int;
pub const SBR_STEREO: core::ffi::c_int = 2 as core::ffi::c_int;
pub const PS_STEREO: core::ffi::c_int = 3 as core::ffi::c_int;
pub const SBR_RESET: core::ffi::c_int = 1 as core::ffi::c_int;
pub const COUPLING_OFF: core::ffi::c_int = 0 as core::ffi::c_int;
pub const COUPLING_LEVEL: core::ffi::c_int = 1 as core::ffi::c_int;
pub const COUPLING_BAL: core::ffi::c_int = 2 as core::ffi::c_int;
static mut ixheaacd_ld_env_table_512: [[core::ffi::c_int; 4]; 16] = [
    [
        2 as core::ffi::c_int,
        4 as core::ffi::c_int,
        -(1 as core::ffi::c_int),
        0 as core::ffi::c_int,
    ],
    [
        2 as core::ffi::c_int,
        5 as core::ffi::c_int,
        -(1 as core::ffi::c_int),
        0 as core::ffi::c_int,
    ],
    [
        3 as core::ffi::c_int,
        2 as core::ffi::c_int,
        6 as core::ffi::c_int,
        1 as core::ffi::c_int,
    ],
    [
        3 as core::ffi::c_int,
        3 as core::ffi::c_int,
        7 as core::ffi::c_int,
        1 as core::ffi::c_int,
    ],
    [
        3 as core::ffi::c_int,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        1 as core::ffi::c_int,
    ],
    [
        3 as core::ffi::c_int,
        5 as core::ffi::c_int,
        9 as core::ffi::c_int,
        1 as core::ffi::c_int,
    ],
    [
        3 as core::ffi::c_int,
        6 as core::ffi::c_int,
        10 as core::ffi::c_int,
        1 as core::ffi::c_int,
    ],
    [
        3 as core::ffi::c_int,
        7 as core::ffi::c_int,
        11 as core::ffi::c_int,
        1 as core::ffi::c_int,
    ],
    [
        3 as core::ffi::c_int,
        8 as core::ffi::c_int,
        12 as core::ffi::c_int,
        1 as core::ffi::c_int,
    ],
    [
        3 as core::ffi::c_int,
        9 as core::ffi::c_int,
        13 as core::ffi::c_int,
        1 as core::ffi::c_int,
    ],
    [
        3 as core::ffi::c_int,
        10 as core::ffi::c_int,
        14 as core::ffi::c_int,
        1 as core::ffi::c_int,
    ],
    [
        2 as core::ffi::c_int,
        11 as core::ffi::c_int,
        -(1 as core::ffi::c_int),
        1 as core::ffi::c_int,
    ],
    [
        2 as core::ffi::c_int,
        12 as core::ffi::c_int,
        -(1 as core::ffi::c_int),
        1 as core::ffi::c_int,
    ],
    [
        2 as core::ffi::c_int,
        13 as core::ffi::c_int,
        -(1 as core::ffi::c_int),
        1 as core::ffi::c_int,
    ],
    [
        2 as core::ffi::c_int,
        14 as core::ffi::c_int,
        -(1 as core::ffi::c_int),
        1 as core::ffi::c_int,
    ],
    [
        2 as core::ffi::c_int,
        15 as core::ffi::c_int,
        -(1 as core::ffi::c_int),
        1 as core::ffi::c_int,
    ],
];
static mut ixheaacd_ld_env_table_480: [[core::ffi::c_int; 4]; 15] = [
    [
        2 as core::ffi::c_int,
        4 as core::ffi::c_int,
        -(1 as core::ffi::c_int),
        0 as core::ffi::c_int,
    ],
    [
        2 as core::ffi::c_int,
        5 as core::ffi::c_int,
        -(1 as core::ffi::c_int),
        0 as core::ffi::c_int,
    ],
    [
        3 as core::ffi::c_int,
        2 as core::ffi::c_int,
        6 as core::ffi::c_int,
        1 as core::ffi::c_int,
    ],
    [
        3 as core::ffi::c_int,
        3 as core::ffi::c_int,
        7 as core::ffi::c_int,
        1 as core::ffi::c_int,
    ],
    [
        3 as core::ffi::c_int,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        1 as core::ffi::c_int,
    ],
    [
        3 as core::ffi::c_int,
        5 as core::ffi::c_int,
        9 as core::ffi::c_int,
        1 as core::ffi::c_int,
    ],
    [
        3 as core::ffi::c_int,
        6 as core::ffi::c_int,
        10 as core::ffi::c_int,
        1 as core::ffi::c_int,
    ],
    [
        3 as core::ffi::c_int,
        7 as core::ffi::c_int,
        11 as core::ffi::c_int,
        1 as core::ffi::c_int,
    ],
    [
        3 as core::ffi::c_int,
        8 as core::ffi::c_int,
        12 as core::ffi::c_int,
        1 as core::ffi::c_int,
    ],
    [
        3 as core::ffi::c_int,
        9 as core::ffi::c_int,
        13 as core::ffi::c_int,
        1 as core::ffi::c_int,
    ],
    [
        2 as core::ffi::c_int,
        10 as core::ffi::c_int,
        -(1 as core::ffi::c_int),
        1 as core::ffi::c_int,
    ],
    [
        2 as core::ffi::c_int,
        11 as core::ffi::c_int,
        -(1 as core::ffi::c_int),
        1 as core::ffi::c_int,
    ],
    [
        2 as core::ffi::c_int,
        12 as core::ffi::c_int,
        -(1 as core::ffi::c_int),
        1 as core::ffi::c_int,
    ],
    [
        2 as core::ffi::c_int,
        13 as core::ffi::c_int,
        -(1 as core::ffi::c_int),
        1 as core::ffi::c_int,
    ],
    [
        2 as core::ffi::c_int,
        14 as core::ffi::c_int,
        -(1 as core::ffi::c_int),
        1 as core::ffi::c_int,
    ],
];
static mut ixheaacd_ld_env_table_time_slot: [core::ffi::c_int; 7] = [
    8 as core::ffi::c_int,
    5 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
];
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_cnt_leading_ones(mut a: WORD32) -> WORD32 {
    let mut count: WORD32 = 0 as WORD32;
    while a != 0 {
        if !(a as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0) {
            break;
        }
        count += 1;
        a = a << 1 as core::ffi::c_int;
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_huffman_decode(
    mut it_bit_buff: WORD32,
    mut h_index: *mut WORD16,
    mut len: *mut WORD16,
    mut input_table: *const UWORD16,
    mut idx_table: *const UWORD32,
) -> VOID {
    let mut temp: UWORD32 = 0 as UWORD32;
    let mut temp1: UWORD32 = 0 as UWORD32;
    let mut found: WORD32 = 0 as WORD32;
    let mut mask: UWORD32 = 0x80000000 as UWORD32;
    let mut clo: WORD32 = 0;
    let mut MAX_LEN: WORD32 = 0;
    let mut ixheaacd_drc_offset: WORD32 = 0 as WORD32;
    let mut length: WORD32 = 0;
    let mut cwrd: UWORD32 = 0;
    let mut len_end: WORD32 = 0;
    MAX_LEN = *input_table.offset(0 as core::ffi::c_int as isize) as WORD32;
    mask = mask
        .wrapping_sub(((1 as core::ffi::c_int) << 31 as WORD32 - MAX_LEN) as UWORD32);
    mask = mask << 1 as core::ffi::c_int;
    temp = it_bit_buff as UWORD32 & mask;
    len_end = *input_table.offset(0 as core::ffi::c_int as isize) as WORD32;
    clo = ixheaacd_cnt_leading_ones(temp as WORD32);
    loop {
        ixheaacd_drc_offset = (*idx_table.offset(clo as isize) >> 20 as core::ffi::c_int
            & 0xff as UWORD32) as WORD32;
        length = (*input_table
            .offset(
                (ixheaacd_drc_offset as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            ) as core::ffi::c_int & 0x1f as core::ffi::c_int) as WORD32;
        cwrd = *idx_table.offset(clo as isize) & 0xfffff as UWORD32;
        temp1 = temp >> 32 as WORD32 - length;
        if temp1 <= cwrd {
            ixheaacd_drc_offset = (ixheaacd_drc_offset as UWORD32)
                .wrapping_sub(cwrd.wrapping_sub(temp1)) as WORD32;
            found = 1 as core::ffi::c_int as WORD32;
        } else {
            len_end = (len_end as UWORD32)
                .wrapping_add(
                    *idx_table.offset(clo as isize) >> 28 as core::ffi::c_int
                        & 0xf as UWORD32,
                ) as WORD32;
            clo = len_end;
        }
        if !(found == 0) {
            break;
        }
    }
    *h_index = (*input_table
        .offset(
            (ixheaacd_drc_offset as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
        ) as core::ffi::c_int >> 5 as core::ffi::c_int) as WORD16;
    *len = length as WORD16;
}
unsafe extern "C" fn ixheaacd_read_esbr_pvc_envelope(
    mut ptr_pvc_data: *mut ia_pvc_data_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut indepFlag: WORD32,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut fixed_length: WORD32 = 0 as WORD32;
    let mut num_grid_info: WORD32 = 0 as WORD32;
    let mut grid_info: WORD32 = 0;
    let mut div_mode: UWORD8 = 0;
    let mut ns_mode: UWORD8 = 0;
    let mut pvc_id: [UWORD16; 17] = [0; 17];
    let mut num_length: UWORD8 = 0;
    let mut length: UWORD8 = 0;
    let mut reuse_pvc_id: UWORD8 = 0;
    let mut sum_length: WORD32 = 0 as WORD32;
    let mut length_bits: WORD32 = 4 as WORD32;
    let mut pvc_id_bits: UWORD8 = PVC_ID_BITS as UWORD8;
    let mut err: IA_ERRORCODE = IA_NO_ERROR;
    div_mode = ixheaacd_read_bits_buf(it_bit_buff, PVC_DIV_MODE_BITS) as UWORD8;
    ns_mode = ixheaacd_read_bits_buf(it_bit_buff, PVC_NS_MODE_BITS) as UWORD8;
    if (*ptr_pvc_data).pvc_mode as core::ffi::c_int == 3 as core::ffi::c_int {
        pvc_id_bits = 0 as UWORD8;
    }
    if div_mode as core::ffi::c_int <= 3 as core::ffi::c_int {
        num_length = div_mode;
        if indepFlag != 0 {
            reuse_pvc_id = 0 as UWORD8;
        } else {
            reuse_pvc_id = ixheaacd_read_bits_buf(it_bit_buff, PVC_REUSE_PVC_ID_BITS)
                as UWORD8;
        }
        if reuse_pvc_id as core::ffi::c_int == 1 as core::ffi::c_int {
            pvc_id[0 as core::ffi::c_int as usize] = (*ptr_pvc_data).prev_pvc_id;
        } else {
            pvc_id[0 as core::ffi::c_int as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                pvc_id_bits as WORD,
            ) as UWORD16;
        }
        k = 1 as core::ffi::c_int as WORD32;
        if num_length != 0 {
            sum_length = 0 as core::ffi::c_int as WORD32;
            i = 0 as core::ffi::c_int as WORD32;
            while i < num_length as core::ffi::c_int {
                if sum_length >= 13 as core::ffi::c_int {
                    length_bits = 1 as core::ffi::c_int as WORD32;
                } else if sum_length >= 11 as core::ffi::c_int {
                    length_bits = 2 as core::ffi::c_int as WORD32;
                } else if sum_length >= 7 as core::ffi::c_int {
                    length_bits = 3 as core::ffi::c_int as WORD32;
                } else {
                    length_bits = 4 as core::ffi::c_int as WORD32;
                }
                length = ixheaacd_read_bits_buf(it_bit_buff, length_bits as WORD)
                    as UWORD8;
                length = (length as core::ffi::c_int + 1 as core::ffi::c_int) as UWORD8;
                sum_length += length as core::ffi::c_int;
                if k as core::ffi::c_int + length as core::ffi::c_int
                    - 1 as core::ffi::c_int > PVC_NUM_TIME_SLOTS
                {
                    return -(1 as WORD32);
                }
                j = 1 as core::ffi::c_int as WORD32;
                while j < length as core::ffi::c_int {
                    pvc_id[k as usize] = pvc_id[(k as core::ffi::c_int
                        - 1 as core::ffi::c_int) as usize];
                    j += 1;
                    k += 1;
                }
                let fresh15 = k;
                k = k + 1;
                pvc_id[fresh15 as usize] = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    pvc_id_bits as WORD,
                ) as UWORD16;
                i += 1;
            }
        }
        while k < 16 as core::ffi::c_int {
            pvc_id[k as usize] = pvc_id[(k as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize];
            k += 1;
        }
    } else {
        match div_mode as core::ffi::c_int {
            4 => {
                num_grid_info = 2 as core::ffi::c_int as WORD32;
                fixed_length = 8 as core::ffi::c_int as WORD32;
            }
            5 => {
                num_grid_info = 4 as core::ffi::c_int as WORD32;
                fixed_length = 4 as core::ffi::c_int as WORD32;
            }
            6 => {
                num_grid_info = 8 as core::ffi::c_int as WORD32;
                fixed_length = 2 as core::ffi::c_int as WORD32;
            }
            7 => {
                num_grid_info = 16 as core::ffi::c_int as WORD32;
                fixed_length = 1 as core::ffi::c_int as WORD32;
            }
            _ => {}
        }
        if indepFlag != 0 {
            grid_info = 1 as core::ffi::c_int as WORD32;
        } else {
            grid_info = ixheaacd_read_bits_buf(it_bit_buff, PVC_GRID_INFO_BITS);
        }
        if grid_info != 0 {
            pvc_id[0 as core::ffi::c_int as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                pvc_id_bits as WORD,
            ) as UWORD16;
        } else {
            pvc_id[0 as core::ffi::c_int as usize] = (*ptr_pvc_data).prev_pvc_id;
        }
        j = 1 as core::ffi::c_int as WORD32;
        k = 1 as core::ffi::c_int as WORD32;
        while j < fixed_length {
            pvc_id[k as usize] = pvc_id[(k as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize];
            j += 1;
            k += 1;
        }
        i = 1 as core::ffi::c_int as WORD32;
        while i < num_grid_info {
            grid_info = ixheaacd_read_bits_buf(it_bit_buff, PVC_GRID_INFO_BITS);
            if grid_info == 1 as core::ffi::c_int {
                let fresh16 = k;
                k = k + 1;
                pvc_id[fresh16 as usize] = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    pvc_id_bits as WORD,
                ) as UWORD16;
            } else {
                pvc_id[k as usize] = pvc_id[(k as core::ffi::c_int
                    - 1 as core::ffi::c_int) as usize];
                k += 1;
            }
            j = 1 as core::ffi::c_int as WORD32;
            while j < fixed_length {
                pvc_id[k as usize] = pvc_id[(k as core::ffi::c_int
                    - 1 as core::ffi::c_int) as usize];
                j += 1;
                k += 1;
            }
            i += 1;
        }
    }
    (*ptr_pvc_data).div_mode = div_mode;
    (*ptr_pvc_data).ns_mode = ns_mode;
    i = 0 as core::ffi::c_int as WORD32;
    while i < PVC_NUM_TIME_SLOTS {
        (*ptr_pvc_data).pvc_id[i as usize] = pvc_id[i as usize];
        i += 1;
    }
    return err as WORD32;
}
unsafe extern "C" fn ixheaacd_pvc_env_dtdf_data(
    mut ptr_frame_data: *mut ia_sbr_frame_info_data_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut usac_independency_flag: WORD32 = (*ptr_frame_data).usac_independency_flag
        as WORD32;
    let mut bs_num_noise: WORD32 = (*ptr_frame_data).str_frame_info_details.num_noise_env
        as WORD32;
    if usac_independency_flag != 0 {
        (*ptr_frame_data).del_cod_dir_noise_arr[0 as core::ffi::c_int as usize] = 0
            as WORD16;
    } else {
        (*ptr_frame_data).del_cod_dir_noise_arr[0 as core::ffi::c_int as usize] = ixheaacd_read_bits_buf(
            it_bit_buff,
            ESBR_DOMAIN_BITS,
        ) as WORD16;
    }
    i = 1 as core::ffi::c_int as WORD32;
    while i < bs_num_noise {
        (*ptr_frame_data).del_cod_dir_noise_arr[i as usize] = ixheaacd_read_bits_buf(
            it_bit_buff,
            ESBR_DOMAIN_BITS,
        ) as WORD16;
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_read_sbr_addi_data(
    mut ptr_frame_data: *mut ia_sbr_frame_info_data_struct,
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut flag: WORD32 = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
    (*ptr_frame_data).sin_start_for_cur_top = (*ptr_frame_data).sin_start_for_next_top;
    (*ptr_frame_data).sin_len_for_cur_top = (*ptr_frame_data).sin_len_for_next_top;
    (*ptr_frame_data).sin_start_for_next_top = 0 as core::ffi::c_int as WORD32;
    (*ptr_frame_data).sin_len_for_next_top = 0 as core::ffi::c_int as WORD32;
    if flag != 0 {
        i = 0 as core::ffi::c_int as WORD32;
        while i
            < (*(*ptr_header_data).pstr_freq_band_data).num_sf_bands[HIGH as usize]
                as core::ffi::c_int
        {
            (*ptr_frame_data).add_harmonics[i as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                1 as WORD,
            ) as FLAG;
            i += 1;
        }
        if (*ptr_frame_data).pvc_mode != 0 as core::ffi::c_int {
            (*ptr_frame_data).sine_position = ESC_SIN_POS as WORD32;
            (*ptr_frame_data).bs_sin_pos_present = ixheaacd_read_bits_buf(
                it_bit_buff,
                1 as WORD,
            );
            if (*ptr_frame_data).bs_sin_pos_present == 1 as core::ffi::c_int {
                (*ptr_frame_data).sine_position = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    5 as WORD,
                );
            }
            if (*ptr_frame_data).var_len > 0 as core::ffi::c_int {
                if (*ptr_frame_data).sine_position > 16 as core::ffi::c_int {
                    if (*ptr_frame_data).sine_position == 31 as core::ffi::c_int {
                        (*ptr_frame_data).sin_start_for_next_top = 0 as core::ffi::c_int
                            as WORD32;
                        (*ptr_frame_data).sin_len_for_next_top = (*ptr_frame_data)
                            .var_len;
                    } else if (*ptr_frame_data).var_len as core::ffi::c_int
                        + 16 as core::ffi::c_int == (*ptr_frame_data).sine_position
                    {
                        (*ptr_frame_data).sin_start_for_next_top = 0 as core::ffi::c_int
                            as WORD32;
                        (*ptr_frame_data).sin_len_for_next_top = (*ptr_frame_data)
                            .var_len;
                    } else {
                        (*ptr_frame_data).sin_start_for_next_top = ((*ptr_frame_data)
                            .sine_position as core::ffi::c_int - 16 as core::ffi::c_int)
                            as WORD32;
                        (*ptr_frame_data).sin_len_for_next_top = (*ptr_frame_data)
                            .var_len;
                    }
                } else {
                    (*ptr_frame_data).sin_start_for_next_top = 0 as core::ffi::c_int
                        as WORD32;
                    (*ptr_frame_data).sin_len_for_next_top = (*ptr_frame_data).var_len;
                }
            } else {
                (*ptr_frame_data).sin_start_for_next_top = 0 as core::ffi::c_int
                    as WORD32;
                (*ptr_frame_data).sin_len_for_next_top = 0 as core::ffi::c_int as WORD32;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_ssc_huff_dec(
    mut t_huff: ia_huffman_data_type,
    mut it_bit_buff: *mut ia_bit_buf_struct,
) -> WORD32 {
    let mut index: WORD32 = 0;
    let mut value: WORD32 = 0;
    let mut bit: WORD32 = 0;
    let mut cw: WORD16 = 0;
    index = 0 as core::ffi::c_int as WORD32;
    while index >= 0 as core::ffi::c_int {
        cw = *t_huff.offset(index as isize) as WORD16;
        bit = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
        if bit != 0 {
            let mut sign: WORD = cw as WORD & 0x80 as WORD;
            if sign != 0 {
                index = (cw as core::ffi::c_uint | 0xffffff80 as core::ffi::c_uint)
                    as WORD32;
            } else {
                index = (cw as core::ffi::c_int & 0x7f as core::ffi::c_int) as WORD32;
            }
        } else {
            index = (cw as core::ffi::c_int >> 8 as core::ffi::c_int) as WORD32;
        }
    }
    value = (index as core::ffi::c_int + 64 as core::ffi::c_int) as WORD32;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_sbr_read_header_data(
    mut pstr_sbr_header: *mut ia_sbr_header_data_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut stereo_flag: FLAG,
    mut pstr_sbr_dflt_header: *mut ia_sbr_header_data_struct,
) -> WORD32 {
    let mut prev_header_info: ia_sbr_header_data_struct = ia_sbr_header_data_struct {
        sync_state: 0,
        err_flag: 0,
        err_flag_prev: 0,
        num_time_slots: 0,
        time_step: 0,
        core_frame_size: 0,
        out_sampling_freq: 0,
        channel_mode: 0,
        amp_res: 0,
        start_freq: 0,
        stop_freq: 0,
        xover_band: 0,
        freq_scale: 0,
        alter_scale: 0,
        noise_bands: 0,
        limiter_bands: 0,
        limiter_gains: 0,
        interpol_freq: 0,
        smoothing_mode: 0,
        pstr_freq_band_data: 0 as *mut ia_freq_band_data_struct,
        header_extra_1: 0,
        header_extra_2: 0,
        pre_proc_flag: 0,
        status: 0,
        sbr_ratio_idx: 0,
        upsamp_fac: 0,
        is_usf_4: 0,
        output_framesize: 0,
        usac_independency_flag: 0,
        pvc_flag: 0,
        hbe_flag: 0,
        esbr_start_up: 0,
        esbr_start_up_pvc: 0,
        usac_flag: 0,
        pvc_mode: 0,
        enh_sbr: 0,
        esbr_hq: 0,
        enh_sbr_ps: 0,
        eld_sbr: 0,
    };
    prev_header_info.start_freq = 0 as WORD16;
    prev_header_info.noise_bands = 0 as WORD16;
    let mut header_extra_1: FLAG = 0 as FLAG;
    let mut header_extra_2: FLAG = 0 as FLAG;
    let mut tmp: WORD32 = 0;
    let mut usac_independency_flag: WORD32 = (*pstr_sbr_header).usac_independency_flag;
    let mut use_dflt_hdr: WORD32 = 0 as WORD32;
    let mut header_present: WORD32 = 1 as WORD32;
    let mut usac_flag: WORD32 = (*pstr_sbr_header).usac_flag;
    if usac_flag == 0 {
        memcpy(
            &mut prev_header_info as *mut ia_sbr_header_data_struct
                as *mut core::ffi::c_void,
            pstr_sbr_header as *const core::ffi::c_void,
            ::core::mem::size_of::<ia_sbr_header_data_struct>() as size_t,
        );
        tmp = ixheaacd_read_bits_buf(
            it_bit_buff,
            SBR_AMPLITUDE_RESOLUTION_BITS + SBR_BEGIN_SAMP_FREQ_BITS
                + SBR_END_SAMP_FREQ_BITS + SBR_CROSS_OVER_BND_BITS,
        );
        (*pstr_sbr_header).amp_res = ((tmp as core::ffi::c_int
            & 0x800 as core::ffi::c_int)
            >> SBR_BEGIN_SAMP_FREQ_BITS + SBR_END_SAMP_FREQ_BITS
                + SBR_CROSS_OVER_BND_BITS) as WORD16;
        (*pstr_sbr_header).start_freq = ((tmp as core::ffi::c_int
            & 0x780 as core::ffi::c_int)
            >> SBR_END_SAMP_FREQ_BITS + SBR_CROSS_OVER_BND_BITS) as WORD16;
        (*pstr_sbr_header).stop_freq = ((tmp as core::ffi::c_int
            & 0x78 as core::ffi::c_int) >> 3 as core::ffi::c_int) as WORD16;
        (*pstr_sbr_header).xover_band = (tmp as core::ffi::c_int
            & 0x7 as core::ffi::c_int) as WORD16;
        tmp = ixheaacd_read_bits_buf(
            it_bit_buff,
            SBR_HDR_RESERV_BITS + SBR_HDR_EXTR_1_BITS + SBR_HDR_EXTR_2_BITS,
        );
        header_extra_1 = (tmp as core::ffi::c_int & 0x2 as core::ffi::c_int)
            >> 1 as core::ffi::c_int;
        header_extra_2 = tmp as core::ffi::c_int & 0x1 as core::ffi::c_int;
        if stereo_flag != 0 {
            (*pstr_sbr_header).channel_mode = SBR_STEREO as WORD32;
        } else {
            (*pstr_sbr_header).channel_mode = SBR_MONO as WORD32;
        }
    } else {
        let mut info_present: WORD32 = 0 as WORD32;
        if (*pstr_sbr_header).sync_state == SBR_ACTIVE {
            memcpy(
                &mut prev_header_info as *mut ia_sbr_header_data_struct
                    as *mut core::ffi::c_void,
                pstr_sbr_header as *const core::ffi::c_void,
                ::core::mem::size_of::<ia_sbr_header_data_struct>() as size_t,
            );
        }
        if usac_independency_flag != 0 {
            header_present = 1 as core::ffi::c_int as WORD32;
            info_present = 1 as core::ffi::c_int as WORD32;
        } else {
            info_present = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
            if info_present != 0 {
                header_present = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
            } else {
                header_present = 0 as core::ffi::c_int as WORD32;
            }
        }
        if info_present != 0 {
            tmp = ixheaacd_read_bits_buf(
                it_bit_buff,
                SBR_AMPLITUDE_RESOLUTION_BITS + ESBR_CROSS_OVER_BND_BITS
                    + ESBR_PRE_FLAT_BITS,
            );
            (*pstr_sbr_header).amp_res = ((tmp as core::ffi::c_int
                & 0x20 as core::ffi::c_int)
                >> ESBR_CROSS_OVER_BND_BITS + ESBR_PRE_FLAT_BITS) as WORD16;
            (*pstr_sbr_header).xover_band = ((tmp as core::ffi::c_int
                & 0x1e as core::ffi::c_int) >> 1 as core::ffi::c_int) as WORD16;
            (*pstr_sbr_header).pre_proc_flag = (tmp as core::ffi::c_int
                & 0x1 as core::ffi::c_int) as WORD16;
            if (*pstr_sbr_header).pvc_flag != 0 {
                (*pstr_sbr_header).pvc_mode = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    ESBR_PVC_MODE_BITS,
                ) as UWORD8;
            } else {
                (*pstr_sbr_header).pvc_mode = 0 as UWORD8;
            }
        }
        if header_present != 0 {
            use_dflt_hdr = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
            if use_dflt_hdr != 0 {
                (*pstr_sbr_header).start_freq = (*pstr_sbr_dflt_header).start_freq;
                (*pstr_sbr_header).stop_freq = (*pstr_sbr_dflt_header).stop_freq;
                (*pstr_sbr_header).header_extra_1 = (*pstr_sbr_dflt_header)
                    .header_extra_1;
                (*pstr_sbr_header).header_extra_2 = (*pstr_sbr_dflt_header)
                    .header_extra_2;
                (*pstr_sbr_header).freq_scale = (*pstr_sbr_dflt_header).freq_scale;
                (*pstr_sbr_header).alter_scale = (*pstr_sbr_dflt_header).alter_scale;
                (*pstr_sbr_header).noise_bands = (*pstr_sbr_dflt_header).noise_bands;
                (*pstr_sbr_header).limiter_bands = (*pstr_sbr_dflt_header).limiter_bands;
                (*pstr_sbr_header).limiter_gains = (*pstr_sbr_dflt_header).limiter_gains;
                (*pstr_sbr_header).interpol_freq = (*pstr_sbr_dflt_header).interpol_freq;
                (*pstr_sbr_header).smoothing_mode = (*pstr_sbr_dflt_header)
                    .smoothing_mode;
            } else {
                tmp = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    SBR_BEGIN_SAMP_FREQ_BITS + SBR_END_SAMP_FREQ_BITS
                        + SBR_HDR_EXTR_1_BITS + SBR_HDR_EXTR_2_BITS,
                );
                (*pstr_sbr_header).start_freq = ((tmp as core::ffi::c_int
                    & 0x3c0 as core::ffi::c_int)
                    >> SBR_END_SAMP_FREQ_BITS + SBR_HDR_EXTR_1_BITS
                        + SBR_HDR_EXTR_2_BITS) as WORD16;
                (*pstr_sbr_header).stop_freq = ((tmp as core::ffi::c_int
                    & 0x3c as core::ffi::c_int)
                    >> SBR_HDR_EXTR_1_BITS + SBR_HDR_EXTR_2_BITS) as WORD16;
                (*pstr_sbr_header).header_extra_1 = ((tmp as core::ffi::c_int
                    & 0x2 as core::ffi::c_int) >> 1 as core::ffi::c_int) as WORD16;
                (*pstr_sbr_header).header_extra_2 = (tmp as core::ffi::c_int
                    & 0x1 as core::ffi::c_int) as WORD16;
                header_extra_1 = (*pstr_sbr_header).header_extra_1 as FLAG;
                header_extra_2 = (*pstr_sbr_header).header_extra_2 as FLAG;
            }
        }
    }
    if use_dflt_hdr == 0 && header_present != 0 {
        if header_extra_1 != 0 {
            tmp = ixheaacd_read_bits_buf(
                it_bit_buff,
                SBR_SAMP_FREQ_LVL_BITS + SBR_CHANGE_LVL_BITS + SBR_NOISE_BND_BITS,
            );
            (*pstr_sbr_header).freq_scale = ((tmp as core::ffi::c_int
                & 0x18 as core::ffi::c_int) >> SBR_CHANGE_LVL_BITS + SBR_NOISE_BND_BITS)
                as WORD16;
            (*pstr_sbr_header).alter_scale = ((tmp as core::ffi::c_int
                & 0x4 as core::ffi::c_int) >> 2 as core::ffi::c_int) as WORD16;
            (*pstr_sbr_header).noise_bands = (tmp as core::ffi::c_int
                & 0x3 as core::ffi::c_int) as WORD16;
        } else {
            (*pstr_sbr_header).freq_scale = SBR_SAMP_FEQ_LVL_DEF as WORD16;
            (*pstr_sbr_header).alter_scale = SBR_CHANGE_LVL_DEF as WORD16;
            (*pstr_sbr_header).noise_bands = SBR_NOISE_BND_DEF as WORD16;
        }
        if header_extra_2 != 0 {
            tmp = ixheaacd_read_bits_buf(
                it_bit_buff,
                SBR_BND_LIMIT_BITS + SBR_GAIN_LIMIT_BITS + SBR_INTERPOL_SAMP_FREQ_BITS
                    + SBR_SMOOTH_LEN_BITS,
            );
            (*pstr_sbr_header).limiter_bands = ((tmp as core::ffi::c_int
                & 0x30 as core::ffi::c_int)
                >> SBR_GAIN_LIMIT_BITS + SBR_INTERPOL_SAMP_FREQ_BITS
                    + SBR_SMOOTH_LEN_BITS) as WORD16;
            (*pstr_sbr_header).limiter_gains = ((tmp as core::ffi::c_int
                & 0xc as core::ffi::c_int)
                >> SBR_INTERPOL_SAMP_FREQ_BITS + SBR_SMOOTH_LEN_BITS) as WORD16;
            (*pstr_sbr_header).interpol_freq = ((tmp as core::ffi::c_int
                & 0x2 as core::ffi::c_int) >> 1 as core::ffi::c_int) as WORD16;
            (*pstr_sbr_header).smoothing_mode = (tmp as core::ffi::c_int
                & 0x1 as core::ffi::c_int) as WORD16;
        } else {
            (*pstr_sbr_header).limiter_bands = SBR_BND_LIMIT_DEF as WORD16;
            (*pstr_sbr_header).limiter_gains = SBR_GAIN_LIMIT_DEF as WORD16;
            (*pstr_sbr_header).interpol_freq = SBR_INTERPOL_SAMP_FEQ_DEF as WORD16;
            (*pstr_sbr_header).smoothing_mode = SBR_SMOOTH_LEN_DEF as WORD16;
        }
    }
    if (*pstr_sbr_header).sync_state != SBR_ACTIVE
        || prev_header_info.start_freq as core::ffi::c_int
            != (*pstr_sbr_header).start_freq as core::ffi::c_int
        || prev_header_info.stop_freq as core::ffi::c_int
            != (*pstr_sbr_header).stop_freq as core::ffi::c_int
        || prev_header_info.xover_band as core::ffi::c_int
            != (*pstr_sbr_header).xover_band as core::ffi::c_int
        || prev_header_info.freq_scale as core::ffi::c_int
            != (*pstr_sbr_header).freq_scale as core::ffi::c_int
        || prev_header_info.alter_scale as core::ffi::c_int
            != (*pstr_sbr_header).alter_scale as core::ffi::c_int
        || prev_header_info.noise_bands as core::ffi::c_int
            != (*pstr_sbr_header).noise_bands as core::ffi::c_int
    {
        return SBR_RESET;
    }
    return 0 as WORD32;
}
unsafe extern "C" fn ixheaacd_sbr_sin_coding_data(
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut ptr_frame_data: *mut ia_sbr_frame_info_data_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
) -> VOID {
    let mut p_add_harmonic: *mut FLAG = ((*ptr_frame_data).add_harmonics).as_mut_ptr();
    let mut i: WORD32 = 0;
    i = (*(*ptr_header_data).pstr_freq_band_data).num_sf_bands[HIGH as usize] as WORD32;
    loop {
        let fresh4 = p_add_harmonic;
        p_add_harmonic = p_add_harmonic.offset(1);
        *fresh4 = ixheaacd_read_bits_buf(it_bit_buff, SBR_ADD_SINE_FLAG_BITS);
        i -= 1;
        if !(i != 0 as core::ffi::c_int) {
            break;
        }
    };
}
unsafe extern "C" fn ixheaacd_validate_frame_info(
    mut pstr_frame_info: *mut ia_frame_info_struct,
    mut num_time_slots: WORD16,
    mut audio_object_type: WORD,
) -> WORD16 {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut start_pos: WORD32 = 0;
    let mut end_pos: WORD32 = 0;
    let mut transient_env: WORD32 = 0;
    let mut start_pos_noise: WORD32 = 0;
    let mut end_pos_noise: WORD32 = 0;
    let mut num_env_sf: WORD32 = 0;
    let mut num_noise_env: WORD32 = 0;
    num_env_sf = (*pstr_frame_info).num_env as WORD32;
    num_noise_env = (*pstr_frame_info).num_noise_env as WORD32;
    if num_env_sf < 1 as core::ffi::c_int || num_env_sf > MAX_ENVELOPES {
        return 0 as WORD16;
    }
    if num_noise_env > MAX_NOISE_ENVELOPES {
        return 0 as WORD16;
    }
    start_pos = (*pstr_frame_info).border_vec[0 as core::ffi::c_int as usize] as WORD32;
    end_pos = (*pstr_frame_info).border_vec[num_env_sf as usize] as WORD32;
    transient_env = (*pstr_frame_info).transient_env as WORD32;
    if transient_env > num_env_sf {
        return 0 as WORD16;
    }
    start_pos_noise = (*pstr_frame_info).noise_border_vec[0 as core::ffi::c_int as usize]
        as WORD32;
    end_pos_noise = (*pstr_frame_info).noise_border_vec[num_noise_env as usize]
        as WORD32;
    if start_pos < 0 as core::ffi::c_int || start_pos >= end_pos {
        return 0 as WORD16;
    }
    if start_pos > SBR_OV_SLOTS {
        return 0 as WORD16;
    }
    if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
        && audio_object_type != AOT_ER_AAC_LD as core::ffi::c_int
    {
        if num_time_slots as core::ffi::c_int != 15 as core::ffi::c_int {
            if end_pos < SBR_TIME_SLOTS {
                return 0 as WORD16;
            }
        } else if end_pos < num_time_slots as core::ffi::c_int {
            return 0 as WORD16
        }
    } else if end_pos < num_time_slots as core::ffi::c_int {
        return 0 as WORD16
    }
    if num_time_slots as core::ffi::c_int != 15 as core::ffi::c_int {
        if end_pos
            > 1024 as core::ffi::c_int / (64 as core::ffi::c_int / 2 as core::ffi::c_int)
                / 2 as core::ffi::c_int + 6 as core::ffi::c_int / 2 as core::ffi::c_int
        {
            return 0 as WORD16;
        }
    } else if end_pos
        > num_time_slots as core::ffi::c_int
            + 6 as core::ffi::c_int / 2 as core::ffi::c_int
    {
        return 0 as WORD16
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_env_sf {
        if (*pstr_frame_info).border_vec[i as usize] as core::ffi::c_int
            > (*pstr_frame_info)
                .border_vec[(i as core::ffi::c_int + 1 as core::ffi::c_int) as usize]
                as core::ffi::c_int
        {
            return 0 as WORD16;
        }
        i += 1;
    }
    if num_env_sf == 1 as core::ffi::c_int && num_noise_env > 1 as core::ffi::c_int {
        return 0 as WORD16;
    }
    if start_pos != start_pos_noise || end_pos != end_pos_noise {
        return 0 as WORD16;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_noise_env {
        start_pos_noise = (*pstr_frame_info).noise_border_vec[i as usize] as WORD32;
        j = 0 as core::ffi::c_int as WORD32;
        while j < num_env_sf {
            if (*pstr_frame_info).border_vec[j as usize] as core::ffi::c_int
                == start_pos_noise
            {
                break;
            }
            j += 1;
        }
        if j == num_env_sf {
            return 0 as WORD16;
        }
        i += 1;
    }
    return 1 as WORD16;
}
unsafe extern "C" fn ixheaacd_read_enh_sbr_data(
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut p_frame_data: *mut core::ffi::c_void,
    mut ele_id: WORD32,
) -> WORD16 {
    let mut tmp: WORD32 = 0 as WORD32;
    let mut num_bits_read: WORD16 = 0 as WORD16;
    tmp = ixheaacd_read_bits_buf(it_bit_buff, ESBR_PRE_FLAT_BITS);
    (*ptr_header_data).pre_proc_flag = tmp as WORD16;
    num_bits_read = (num_bits_read as core::ffi::c_int + ESBR_PRE_FLAT_BITS) as WORD16;
    if ele_id == SBR_ID_SCE as core::ffi::c_int {
        let mut ptr_frame_data: *mut ia_sbr_frame_info_data_struct = p_frame_data
            as *mut ia_sbr_frame_info_data_struct;
        tmp = ixheaacd_read_bits_buf(it_bit_buff, ESBR_PATCHING_MODE_BITS);
        (*ptr_frame_data).sbr_patching_mode = tmp;
        num_bits_read = (num_bits_read as core::ffi::c_int + ESBR_PATCHING_MODE_BITS)
            as WORD16;
        if tmp == 0 as core::ffi::c_int {
            tmp = ixheaacd_read_bits_buf(it_bit_buff, ESBR_OVERSAMPLING_FLAG_BITS);
            (*ptr_frame_data).over_sampling_flag = tmp;
            num_bits_read = (num_bits_read as core::ffi::c_int
                + ESBR_OVERSAMPLING_FLAG_BITS) as WORD16;
            tmp = ixheaacd_read_bits_buf(it_bit_buff, ESBR_PITCHIN_FLAG_BITS);
            num_bits_read = (num_bits_read as core::ffi::c_int + ESBR_PITCHIN_FLAG_BITS)
                as WORD16;
            if tmp != 0 {
                tmp = ixheaacd_read_bits_buf(it_bit_buff, ESBR_PITCHIN_BINS_BITS);
                (*ptr_frame_data).pitch_in_bins = tmp;
                num_bits_read = (num_bits_read as core::ffi::c_int
                    + ESBR_PITCHIN_BINS_BITS) as WORD16;
            } else {
                (*ptr_frame_data).pitch_in_bins = 0 as core::ffi::c_int as WORD32;
            }
        } else {
            (*ptr_frame_data).over_sampling_flag = 0 as core::ffi::c_int as WORD32;
            (*ptr_frame_data).pitch_in_bins = 0 as core::ffi::c_int as WORD32;
        }
    } else if ele_id == SBR_ID_CPE as core::ffi::c_int {
        let mut ptr_frame_data_0: *mut *mut ia_sbr_frame_info_data_struct = p_frame_data
            as *mut *mut ia_sbr_frame_info_data_struct;
        if (**ptr_frame_data_0.offset(0 as core::ffi::c_int as isize)).coupling_mode != 0
        {
            let ref mut fresh0 = (**ptr_frame_data_0
                .offset(1 as core::ffi::c_int as isize))
                .sbr_patching_mode;
            *fresh0 = ixheaacd_read_bits_buf(it_bit_buff, ESBR_PATCHING_MODE_BITS);
            (**ptr_frame_data_0.offset(0 as core::ffi::c_int as isize))
                .sbr_patching_mode = *fresh0;
            num_bits_read = (num_bits_read as core::ffi::c_int + ESBR_PATCHING_MODE_BITS)
                as WORD16;
            if (**ptr_frame_data_0.offset(0 as core::ffi::c_int as isize))
                .sbr_patching_mode == 0 as core::ffi::c_int
            {
                let ref mut fresh1 = (**ptr_frame_data_0
                    .offset(1 as core::ffi::c_int as isize))
                    .over_sampling_flag;
                *fresh1 = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    ESBR_OVERSAMPLING_FLAG_BITS,
                );
                (**ptr_frame_data_0.offset(0 as core::ffi::c_int as isize))
                    .over_sampling_flag = *fresh1;
                num_bits_read = (num_bits_read as core::ffi::c_int
                    + ESBR_OVERSAMPLING_FLAG_BITS) as WORD16;
                num_bits_read = (num_bits_read as core::ffi::c_int
                    + ESBR_PITCHIN_FLAG_BITS) as WORD16;
                if ixheaacd_read_bits_buf(it_bit_buff, ESBR_PITCHIN_FLAG_BITS) != 0 {
                    let ref mut fresh2 = (**ptr_frame_data_0
                        .offset(1 as core::ffi::c_int as isize))
                        .pitch_in_bins;
                    *fresh2 = ixheaacd_read_bits_buf(
                        it_bit_buff,
                        ESBR_PITCHIN_BINS_BITS,
                    );
                    (**ptr_frame_data_0.offset(0 as core::ffi::c_int as isize))
                        .pitch_in_bins = *fresh2;
                    num_bits_read = (num_bits_read as core::ffi::c_int
                        + ESBR_PITCHIN_BINS_BITS) as WORD16;
                } else {
                    (**ptr_frame_data_0.offset(0 as core::ffi::c_int as isize))
                        .pitch_in_bins = 0 as core::ffi::c_int as WORD32;
                    (**ptr_frame_data_0.offset(1 as core::ffi::c_int as isize))
                        .pitch_in_bins = 0 as core::ffi::c_int as WORD32;
                }
            } else {
                (**ptr_frame_data_0.offset(0 as core::ffi::c_int as isize))
                    .over_sampling_flag = 0 as core::ffi::c_int as WORD32;
                (**ptr_frame_data_0.offset(0 as core::ffi::c_int as isize))
                    .pitch_in_bins = 0 as core::ffi::c_int as WORD32;
                (**ptr_frame_data_0.offset(1 as core::ffi::c_int as isize))
                    .over_sampling_flag = 0 as core::ffi::c_int as WORD32;
                (**ptr_frame_data_0.offset(1 as core::ffi::c_int as isize))
                    .pitch_in_bins = 0 as core::ffi::c_int as WORD32;
            }
        } else {
            (**ptr_frame_data_0.offset(0 as core::ffi::c_int as isize))
                .sbr_patching_mode = ixheaacd_read_bits_buf(
                it_bit_buff,
                ESBR_PATCHING_MODE_BITS,
            );
            num_bits_read = (num_bits_read as core::ffi::c_int + ESBR_PATCHING_MODE_BITS)
                as WORD16;
            if (**ptr_frame_data_0.offset(0 as core::ffi::c_int as isize))
                .sbr_patching_mode == 0 as core::ffi::c_int
            {
                (**ptr_frame_data_0.offset(0 as core::ffi::c_int as isize))
                    .over_sampling_flag = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    ESBR_OVERSAMPLING_FLAG_BITS,
                );
                num_bits_read = (num_bits_read as core::ffi::c_int
                    + ESBR_OVERSAMPLING_FLAG_BITS) as WORD16;
                num_bits_read = (num_bits_read as core::ffi::c_int
                    + ESBR_PITCHIN_FLAG_BITS) as WORD16;
                if ixheaacd_read_bits_buf(it_bit_buff, ESBR_PITCHIN_FLAG_BITS) != 0 {
                    (**ptr_frame_data_0.offset(0 as core::ffi::c_int as isize))
                        .pitch_in_bins = ixheaacd_read_bits_buf(
                        it_bit_buff,
                        ESBR_PITCHIN_BINS_BITS,
                    );
                    num_bits_read = (num_bits_read as core::ffi::c_int
                        + ESBR_PITCHIN_BINS_BITS) as WORD16;
                } else {
                    (**ptr_frame_data_0.offset(0 as core::ffi::c_int as isize))
                        .pitch_in_bins = 0 as core::ffi::c_int as WORD32;
                }
            } else {
                (**ptr_frame_data_0.offset(0 as core::ffi::c_int as isize))
                    .over_sampling_flag = 0 as core::ffi::c_int as WORD32;
                (**ptr_frame_data_0.offset(0 as core::ffi::c_int as isize))
                    .pitch_in_bins = 0 as core::ffi::c_int as WORD32;
            }
            (**ptr_frame_data_0.offset(1 as core::ffi::c_int as isize))
                .sbr_patching_mode = ixheaacd_read_bits_buf(
                it_bit_buff,
                ESBR_PATCHING_MODE_BITS,
            );
            num_bits_read = (num_bits_read as core::ffi::c_int + ESBR_PATCHING_MODE_BITS)
                as WORD16;
            if (**ptr_frame_data_0.offset(1 as core::ffi::c_int as isize))
                .sbr_patching_mode == 0 as core::ffi::c_int
            {
                (**ptr_frame_data_0.offset(1 as core::ffi::c_int as isize))
                    .over_sampling_flag = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    ESBR_OVERSAMPLING_FLAG_BITS,
                );
                num_bits_read = (num_bits_read as core::ffi::c_int
                    + ESBR_OVERSAMPLING_FLAG_BITS) as WORD16;
                num_bits_read = (num_bits_read as core::ffi::c_int
                    + ESBR_PITCHIN_FLAG_BITS) as WORD16;
                if ixheaacd_read_bits_buf(it_bit_buff, ESBR_PITCHIN_FLAG_BITS) != 0 {
                    (**ptr_frame_data_0.offset(1 as core::ffi::c_int as isize))
                        .pitch_in_bins = ixheaacd_read_bits_buf(
                        it_bit_buff,
                        ESBR_PITCHIN_BINS_BITS,
                    );
                    num_bits_read = (num_bits_read as core::ffi::c_int
                        + ESBR_PITCHIN_BINS_BITS) as WORD16;
                } else {
                    (**ptr_frame_data_0.offset(1 as core::ffi::c_int as isize))
                        .pitch_in_bins = 0 as core::ffi::c_int as WORD32;
                }
            } else {
                let ref mut fresh3 = (**ptr_frame_data_0
                    .offset(1 as core::ffi::c_int as isize))
                    .pitch_in_bins;
                *fresh3 = 0 as core::ffi::c_int as WORD32;
                (**ptr_frame_data_0.offset(1 as core::ffi::c_int as isize))
                    .over_sampling_flag = *fresh3;
            }
        }
    }
    if (num_bits_read as core::ffi::c_int) < 6 as core::ffi::c_int {
        ixheaacd_read_bits_buf(it_bit_buff, 6 as WORD - num_bits_read as WORD);
        num_bits_read = 6 as WORD16;
    }
    return num_bits_read;
}
unsafe extern "C" fn ixheaacd_read_extn_data(
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut ptr_ps_dec: *mut ia_ps_dec_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut ps_tables_ptr: *mut ia_ps_tables_struct,
    mut p_frame_data: *mut core::ffi::c_void,
    mut ele_id: WORD32,
) -> IA_ERRORCODE {
    let mut i: WORD = 0;
    let mut extended_data: WORD = 0;
    let mut no_bits_left: WORD = 0;
    extended_data = ixheaacd_read_bits_buf(it_bit_buff, SBR_ENLARGED_DATA_BITS) as WORD;
    if extended_data != 0 {
        let mut cnt: WORD = 0;
        let mut ps_read: FLAG = 0;
        ps_read = 0 as core::ffi::c_int as FLAG;
        cnt = ixheaacd_read_bits_buf(it_bit_buff, SBR_CONT_SIZE_BITS) as WORD;
        if cnt == ((1 as core::ffi::c_int) << SBR_CONT_SIZE_BITS) - 1 as core::ffi::c_int
        {
            cnt = (cnt as WORD32
                + ixheaacd_read_bits_buf(it_bit_buff, SBR_CONT_ESC_CNT_BITS)) as WORD;
        }
        no_bits_left = cnt << 3 as core::ffi::c_int;
        (*ptr_header_data).hbe_flag = ((*ptr_header_data).usac_flag == 0)
            as core::ffi::c_int as FLAG;
        (*ptr_header_data).sbr_ratio_idx = SBR_UPSAMPLE_IDX_2_1 as WORD32;
        while no_bits_left > 7 as core::ffi::c_int {
            let mut extension_id: WORD = ixheaacd_read_bits_buf(
                it_bit_buff,
                SBR_CONT_ID_BITS,
            ) as WORD;
            if extension_id == EXTENSION_ID_ENHSBR_CODING
                && (*ptr_header_data).enh_sbr == 0
            {
                extension_id = -(1 as core::ffi::c_int) as WORD;
            }
            no_bits_left = (no_bits_left as core::ffi::c_int - SBR_CONT_ID_BITS) as WORD;
            let mut current_block_32: u64;
            match extension_id {
                EXTENSION_ID_PS_CODING => {
                    if ptr_ps_dec.is_null() {
                        return 0 as IA_ERRORCODE;
                    }
                    if !((*ptr_ps_dec).force_mono != 0 || ps_read != 0) {
                        let mut ret_val: IA_ERRORCODE = ixheaacd_read_ps_data(
                            ptr_ps_dec,
                            it_bit_buff,
                            no_bits_left as WORD16,
                            ps_tables_ptr,
                        );
                        if ret_val as core::ffi::c_uint == IA_FATAL_ERROR {
                            return ret_val
                        } else {
                            no_bits_left = (no_bits_left as IA_ERRORCODE - ret_val)
                                as WORD;
                        }
                        if no_bits_left < 0 as core::ffi::c_int {
                            return 0 as IA_ERRORCODE;
                        }
                        (*ptr_header_data).channel_mode = PS_STEREO as WORD32;
                        ps_read = 1 as core::ffi::c_int as FLAG;
                        current_block_32 = 17281240262373992796;
                    } else {
                        current_block_32 = 10652014663920648156;
                    }
                }
                EXTENSION_ID_ENHSBR_CODING => {
                    current_block_32 = 10652014663920648156;
                }
                _ => {
                    cnt = no_bits_left >> 3 as core::ffi::c_int;
                    i = (cnt as core::ffi::c_int - 1 as core::ffi::c_int) as WORD;
                    while i >= 0 as core::ffi::c_int {
                        ixheaacd_read_bits_buf(it_bit_buff, 8 as WORD);
                        i -= 1;
                    }
                    no_bits_left = no_bits_left - (cnt << 3 as core::ffi::c_int);
                    current_block_32 = 17281240262373992796;
                }
            }
            match current_block_32 {
                10652014663920648156 => {
                    no_bits_left = (no_bits_left as core::ffi::c_int
                        - ixheaacd_read_enh_sbr_data(
                            ptr_header_data,
                            it_bit_buff,
                            p_frame_data,
                            ele_id,
                        ) as core::ffi::c_int) as WORD;
                    (*ptr_header_data).hbe_flag = 1 as core::ffi::c_int as FLAG;
                    (*ptr_header_data).sbr_ratio_idx = SBR_UPSAMPLE_IDX_2_1 as WORD32;
                }
                _ => {}
            }
        }
        if no_bits_left < 0 as core::ffi::c_int {
            return 0 as IA_ERRORCODE;
        }
        ixheaacd_read_bits_buf(it_bit_buff, no_bits_left);
    }
    return 0 as IA_ERRORCODE;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_sbr_read_pvc_sce(
    mut ptr_frame_data: *mut ia_sbr_frame_info_data_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut hbe_flag: WORD32,
    mut ptr_pvc_data: *mut ia_pvc_data_struct,
    mut ptr_sbr_tables: *mut ia_sbr_tables_struct,
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut err_code: WORD32 = 0 as WORD32;
    let mut env_extr_tables_ptr: *mut ia_env_extr_tables_struct = (*ptr_sbr_tables)
        .env_extr_tables_ptr;
    let mut usac_independency_flag: WORD32 = (*ptr_frame_data).usac_independency_flag
        as WORD32;
    if hbe_flag != 0 {
        (*ptr_frame_data).sbr_patching_mode = ixheaacd_read_bits_buf(
            it_bit_buff,
            ESBR_PATCHING_MODE_BITS,
        );
        if (*ptr_frame_data).sbr_patching_mode == 0 as core::ffi::c_int {
            (*ptr_frame_data).over_sampling_flag = ixheaacd_read_bits_buf(
                it_bit_buff,
                ESBR_OVERSAMPLING_FLAG_BITS,
            );
            if ixheaacd_read_bits_buf(it_bit_buff, ESBR_PITCHIN_FLAG_BITS) != 0 {
                (*ptr_frame_data).pitch_in_bins = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    ESBR_PITCHIN_BINS_BITS,
                );
            } else {
                (*ptr_frame_data).pitch_in_bins = 0 as core::ffi::c_int as WORD32;
            }
        } else {
            (*ptr_frame_data).pitch_in_bins = 0 as core::ffi::c_int as WORD32;
            (*ptr_frame_data).over_sampling_flag = (*ptr_frame_data).pitch_in_bins;
        }
    }
    err_code = ixheaacd_pvc_time_freq_grid_info(it_bit_buff, ptr_frame_data);
    if err_code != 0 {
        return err_code;
    }
    (*ptr_pvc_data).prev_sbr_mode = PVC_SBR as core::ffi::c_int as WORD32;
    ixheaacd_pvc_env_dtdf_data(ptr_frame_data, it_bit_buff);
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*(*ptr_header_data).pstr_freq_band_data).num_nf_bands as core::ffi::c_int
    {
        (*ptr_frame_data).sbr_invf_mode_prev[i as usize] = (*ptr_frame_data)
            .sbr_invf_mode[i as usize];
        (*ptr_frame_data).sbr_invf_mode[i as usize] = ixheaacd_read_bits_buf(
            it_bit_buff,
            ESBR_INVF_MODE_BITS,
        );
        i += 1;
    }
    (*ptr_pvc_data).pvc_mode = (*ptr_header_data).pvc_mode;
    err_code = ixheaacd_read_esbr_pvc_envelope(
        ptr_pvc_data,
        it_bit_buff,
        usac_independency_flag,
    );
    if err_code != 0 {
        return err_code;
    }
    ixheaacd_read_sbr_noise_floor_data(
        ptr_header_data,
        ptr_frame_data,
        it_bit_buff,
        env_extr_tables_ptr,
    );
    memset(
        ((*ptr_frame_data).add_harmonics).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ((*(*ptr_header_data).pstr_freq_band_data).num_sf_bands[HIGH as usize] as size_t)
            .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
    );
    (*ptr_frame_data).pvc_mode = (*ptr_header_data).pvc_mode as WORD32;
    ixheaacd_read_sbr_addi_data(ptr_frame_data, ptr_header_data, it_bit_buff);
    (*ptr_frame_data).coupling_mode = COUPLING_OFF as WORD32;
    return err_code;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_sbr_read_sce(
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut ptr_frame_data: *mut ia_sbr_frame_info_data_struct,
    mut ptr_ps_dec: *mut ia_ps_dec_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut ptr_sbr_tables: *mut ia_sbr_tables_struct,
    mut audio_object_type: WORD,
    mut ec_flag: WORD32,
) -> IA_ERRORCODE {
    let mut bit: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut hbe_flag: WORD32 = (*ptr_header_data).hbe_flag as WORD32;
    let mut num_if_bands: WORD32 = (*(*ptr_header_data).pstr_freq_band_data).num_if_bands
        as WORD32;
    let mut usac_flag: WORD32 = (*ptr_header_data).usac_flag;
    let mut env_extr_tables_ptr: *mut ia_env_extr_tables_struct = (*ptr_sbr_tables)
        .env_extr_tables_ptr;
    let mut err: IA_ERRORCODE = IA_NO_ERROR;
    (*ptr_frame_data).coupling_mode = COUPLING_OFF as WORD32;
    if usac_flag == 0 {
        bit = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
        if bit != 0 {
            ixheaacd_read_bits_buf(it_bit_buff, SBR_SCE_RESERV_BITS);
        }
        if audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int
            || audio_object_type == AOT_ER_AAC_LD as core::ffi::c_int
        {
            if (*ptr_frame_data).eld_sbr_flag == 1 as core::ffi::c_int {
                err = ixheaacd_extract_frame_info_ld(it_bit_buff, ptr_frame_data);
                if err != 0 {
                    return err;
                }
            }
        } else if ixheaacd_sbr_time_freq_grid_info(
            it_bit_buff,
            ptr_frame_data,
            env_extr_tables_ptr,
            (*ptr_header_data).num_time_slots,
        ) == 0
        {
            return 0 as IA_ERRORCODE
        }
        if ixheaacd_validate_frame_info(
            &mut (*ptr_frame_data).str_frame_info_details,
            (*ptr_header_data).num_time_slots,
            audio_object_type,
        ) == 0
        {
            return 0 as IA_ERRORCODE;
        }
    } else {
        if hbe_flag != 0 {
            (*ptr_frame_data).sbr_patching_mode = ixheaacd_read_bits_buf(
                it_bit_buff,
                ESBR_PATCHING_MODE_BITS,
            );
            if (*ptr_frame_data).sbr_patching_mode == 0 as core::ffi::c_int {
                (*ptr_frame_data).over_sampling_flag = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    ESBR_OVERSAMPLING_FLAG_BITS,
                );
                if ixheaacd_read_bits_buf(it_bit_buff, ESBR_PITCHIN_FLAG_BITS) != 0 {
                    (*ptr_frame_data).pitch_in_bins = ixheaacd_read_bits_buf(
                        it_bit_buff,
                        ESBR_PITCHIN_BINS_BITS,
                    );
                } else {
                    (*ptr_frame_data).pitch_in_bins = 0 as core::ffi::c_int as WORD32;
                }
            } else {
                (*ptr_frame_data).pitch_in_bins = 0 as core::ffi::c_int as WORD32;
                (*ptr_frame_data).over_sampling_flag = (*ptr_frame_data).pitch_in_bins;
            }
        }
        (*ptr_frame_data).num_time_slots = (*ptr_header_data).num_time_slots as WORD32;
        if ixheaacd_sbr_time_freq_grid_info(
            it_bit_buff,
            ptr_frame_data,
            env_extr_tables_ptr,
            (*ptr_header_data).num_time_slots,
        ) == 0
        {
            return 0 as IA_ERRORCODE;
        }
        if ixheaacd_validate_frame_info(
            &mut (*ptr_frame_data).str_frame_info_details,
            (*ptr_header_data).num_time_slots,
            audio_object_type,
        ) == 0
        {
            return 0 as IA_ERRORCODE;
        }
        (*ptr_frame_data).prev_sbr_mode = ORIG_SBR as core::ffi::c_int as FLAG;
    }
    ixheaacd_sbr_env_dtdf_data(
        ptr_frame_data,
        it_bit_buff,
        (*ptr_header_data).usac_flag,
    );
    if (*ptr_frame_data).del_cod_dir_arr[0 as core::ffi::c_int as usize]
        as core::ffi::c_int == DTDF_DIR_FREQ
    {
        (*ptr_header_data).err_flag = 0 as core::ffi::c_int as FLAG;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_if_bands {
        (*ptr_frame_data).sbr_invf_mode_prev[i as usize] = (*ptr_frame_data)
            .sbr_invf_mode[i as usize];
        (*ptr_frame_data).sbr_invf_mode[i as usize] = ixheaacd_read_bits_buf(
            it_bit_buff,
            SBR_INVERSE_FILT_MODE_BITS,
        );
        i += 1;
    }
    if ixheaacd_read_sbr_env_data(
        ptr_header_data,
        ptr_frame_data,
        it_bit_buff,
        env_extr_tables_ptr,
        audio_object_type,
    ) == 0
    {
        return 0 as IA_ERRORCODE;
    }
    ixheaacd_read_sbr_noise_floor_data(
        ptr_header_data,
        ptr_frame_data,
        it_bit_buff,
        env_extr_tables_ptr,
    );
    if usac_flag != 0 {
        memset(
            ((*ptr_frame_data).add_harmonics).as_mut_ptr() as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ((*(*ptr_header_data).pstr_freq_band_data)
                .num_sf_bands[1 as core::ffi::c_int as usize] as size_t)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
        );
        (*ptr_frame_data).coupling_mode = COUPLING_OFF as WORD32;
    }
    bit = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD) as WORD32;
    if bit != 0 {
        ixheaacd_sbr_sin_coding_data(ptr_header_data, ptr_frame_data, it_bit_buff);
    } else {
        memset(
            ((*ptr_frame_data).add_harmonics).as_mut_ptr() as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<FLAG>() as size_t)
                .wrapping_mul(MAX_FREQ_COEFFS as size_t),
        );
    }
    if usac_flag == 0 {
        let mut err_0: IA_ERRORCODE = ixheaacd_read_extn_data(
            ptr_header_data,
            ptr_ps_dec,
            it_bit_buff,
            (*ptr_sbr_tables).ps_tables_ptr,
            ptr_frame_data as *mut core::ffi::c_void,
            SBR_ID_SCE as core::ffi::c_int as WORD32,
        );
        if err_0 as core::ffi::c_uint == IA_FATAL_ERROR {
            if ec_flag != 0 { return 0 as IA_ERRORCODE } else { return err_0 }
        }
    }
    return 1 as IA_ERRORCODE;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_sbr_read_cpe(
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut ptr_frame_data: *mut *mut ia_sbr_frame_info_data_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut ptr_sbr_tables: *mut ia_sbr_tables_struct,
    mut audio_object_type: WORD,
) -> IA_ERRORCODE {
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut bit: WORD32 = 0;
    let mut num_ch: WORD32 = 2 as WORD32;
    let mut num_if_bands: WORD32 = (*(*ptr_header_data).pstr_freq_band_data).num_if_bands
        as WORD32;
    let mut hbe_flag: WORD32 = (*ptr_header_data).hbe_flag as WORD32;
    let mut usac_flag: WORD32 = (*ptr_header_data).usac_flag;
    let mut err: IA_ERRORCODE = IA_NO_ERROR;
    let mut env_extr_tables_ptr: *mut ia_env_extr_tables_struct = (*ptr_sbr_tables)
        .env_extr_tables_ptr;
    bit = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
    if usac_flag != 0 {
        if bit != 0 {
            if hbe_flag != 0 {
                let ref mut fresh10 = (**ptr_frame_data
                    .offset(1 as core::ffi::c_int as isize))
                    .sbr_patching_mode;
                *fresh10 = ixheaacd_read_bits_buf(it_bit_buff, ESBR_PATCHING_MODE_BITS);
                (**ptr_frame_data.offset(0 as core::ffi::c_int as isize))
                    .sbr_patching_mode = *fresh10;
                if (**ptr_frame_data.offset(0 as core::ffi::c_int as isize))
                    .sbr_patching_mode == 0 as core::ffi::c_int
                {
                    let ref mut fresh11 = (**ptr_frame_data
                        .offset(1 as core::ffi::c_int as isize))
                        .over_sampling_flag;
                    *fresh11 = ixheaacd_read_bits_buf(
                        it_bit_buff,
                        ESBR_OVERSAMPLING_FLAG_BITS,
                    );
                    (**ptr_frame_data.offset(0 as core::ffi::c_int as isize))
                        .over_sampling_flag = *fresh11;
                    if ixheaacd_read_bits_buf(it_bit_buff, ESBR_PITCHIN_FLAG_BITS) != 0 {
                        let ref mut fresh12 = (**ptr_frame_data
                            .offset(1 as core::ffi::c_int as isize))
                            .pitch_in_bins;
                        *fresh12 = ixheaacd_read_bits_buf(
                            it_bit_buff,
                            ESBR_PITCHIN_BINS_BITS,
                        );
                        (**ptr_frame_data.offset(0 as core::ffi::c_int as isize))
                            .pitch_in_bins = *fresh12;
                    } else {
                        let ref mut fresh13 = (**ptr_frame_data
                            .offset(1 as core::ffi::c_int as isize))
                            .pitch_in_bins;
                        *fresh13 = 0 as core::ffi::c_int as WORD32;
                        (**ptr_frame_data.offset(0 as core::ffi::c_int as isize))
                            .pitch_in_bins = *fresh13;
                    }
                } else {
                    (**ptr_frame_data.offset(0 as core::ffi::c_int as isize))
                        .over_sampling_flag = 0 as core::ffi::c_int as WORD32;
                    (**ptr_frame_data.offset(1 as core::ffi::c_int as isize))
                        .over_sampling_flag = 0 as core::ffi::c_int as WORD32;
                    (**ptr_frame_data.offset(0 as core::ffi::c_int as isize))
                        .pitch_in_bins = 0 as core::ffi::c_int as WORD32;
                    (**ptr_frame_data.offset(1 as core::ffi::c_int as isize))
                        .pitch_in_bins = 0 as core::ffi::c_int as WORD32;
                }
            }
            (**ptr_frame_data.offset(0 as core::ffi::c_int as isize)).coupling_mode = COUPLING_LEVEL
                as WORD32;
            (**ptr_frame_data.offset(1 as core::ffi::c_int as isize)).coupling_mode = COUPLING_BAL
                as WORD32;
        } else {
            if hbe_flag != 0 {
                (**ptr_frame_data.offset(0 as core::ffi::c_int as isize))
                    .sbr_patching_mode = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    ESBR_PATCHING_MODE_BITS,
                );
                if (**ptr_frame_data.offset(0 as core::ffi::c_int as isize))
                    .sbr_patching_mode == 0 as core::ffi::c_int
                {
                    (**ptr_frame_data.offset(0 as core::ffi::c_int as isize))
                        .over_sampling_flag = ixheaacd_read_bits_buf(
                        it_bit_buff,
                        ESBR_OVERSAMPLING_FLAG_BITS,
                    );
                    if ixheaacd_read_bits_buf(it_bit_buff, ESBR_PITCHIN_FLAG_BITS) != 0 {
                        (**ptr_frame_data.offset(0 as core::ffi::c_int as isize))
                            .pitch_in_bins = ixheaacd_read_bits_buf(
                            it_bit_buff,
                            ESBR_PITCHIN_BINS_BITS,
                        );
                    } else {
                        (**ptr_frame_data.offset(0 as core::ffi::c_int as isize))
                            .pitch_in_bins = 0 as core::ffi::c_int as WORD32;
                    }
                } else {
                    (**ptr_frame_data.offset(0 as core::ffi::c_int as isize))
                        .over_sampling_flag = 0 as core::ffi::c_int as WORD32;
                    (**ptr_frame_data.offset(0 as core::ffi::c_int as isize))
                        .pitch_in_bins = 0 as core::ffi::c_int as WORD32;
                }
                (**ptr_frame_data.offset(1 as core::ffi::c_int as isize))
                    .sbr_patching_mode = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    ESBR_PATCHING_MODE_BITS,
                );
                if (**ptr_frame_data.offset(1 as core::ffi::c_int as isize))
                    .sbr_patching_mode == 0 as core::ffi::c_int
                {
                    (**ptr_frame_data.offset(1 as core::ffi::c_int as isize))
                        .over_sampling_flag = ixheaacd_read_bits_buf(
                        it_bit_buff,
                        ESBR_OVERSAMPLING_FLAG_BITS,
                    );
                    if ixheaacd_read_bits_buf(it_bit_buff, ESBR_PITCHIN_FLAG_BITS) != 0 {
                        (**ptr_frame_data.offset(1 as core::ffi::c_int as isize))
                            .pitch_in_bins = ixheaacd_read_bits_buf(
                            it_bit_buff,
                            ESBR_PITCHIN_BINS_BITS,
                        );
                    } else {
                        (**ptr_frame_data.offset(1 as core::ffi::c_int as isize))
                            .pitch_in_bins = 0 as core::ffi::c_int as WORD32;
                    }
                } else {
                    let ref mut fresh14 = (**ptr_frame_data
                        .offset(1 as core::ffi::c_int as isize))
                        .pitch_in_bins;
                    *fresh14 = 0 as core::ffi::c_int as WORD32;
                    (**ptr_frame_data.offset(1 as core::ffi::c_int as isize))
                        .over_sampling_flag = *fresh14;
                }
            }
            (**ptr_frame_data.offset(0 as core::ffi::c_int as isize)).coupling_mode = COUPLING_OFF
                as WORD32;
            (**ptr_frame_data.offset(1 as core::ffi::c_int as isize)).coupling_mode = COUPLING_OFF
                as WORD32;
        }
    } else {
        if bit != 0 {
            ixheaacd_read_bits_buf(
                it_bit_buff,
                SBR_SCE_RESERV_BITS + SBR_SCE_RESERV_BITS,
            );
        }
        if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
            && (*ptr_header_data).channel_mode != SBR_STEREO
        {
            (*ptr_header_data).sync_state = UPSAMPLING as WORD32;
            return 0 as IA_ERRORCODE;
        }
        bit = ixheaacd_read_bits_buf(it_bit_buff, SBR_COUPLNG_MODE_BITS);
        if bit != 0 {
            (**ptr_frame_data.offset(0 as core::ffi::c_int as isize)).coupling_mode = COUPLING_LEVEL
                as WORD32;
            (**ptr_frame_data.offset(1 as core::ffi::c_int as isize)).coupling_mode = COUPLING_BAL
                as WORD32;
        } else {
            (**ptr_frame_data.offset(0 as core::ffi::c_int as isize)).coupling_mode = COUPLING_OFF
                as WORD32;
            (**ptr_frame_data.offset(1 as core::ffi::c_int as isize)).coupling_mode = COUPLING_OFF
                as WORD32;
        }
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_ch {
        (**ptr_frame_data.offset(i as isize)).num_time_slots = (*ptr_header_data)
            .num_time_slots as WORD32;
        if audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int
            || audio_object_type == AOT_ER_AAC_LD as core::ffi::c_int
        {
            if (**ptr_frame_data.offset(i as isize)).eld_sbr_flag
                == 1 as core::ffi::c_int
            {
                err = ixheaacd_extract_frame_info_ld(
                    it_bit_buff,
                    *ptr_frame_data.offset(i as isize),
                );
                if err != 0 {
                    return err;
                }
            }
        } else if ixheaacd_sbr_time_freq_grid_info(
            it_bit_buff,
            *ptr_frame_data.offset(i as isize),
            env_extr_tables_ptr,
            (*ptr_header_data).num_time_slots,
        ) == 0
        {
            return 0 as IA_ERRORCODE
        }
        if ixheaacd_validate_frame_info(
            &mut (**ptr_frame_data.offset(i as isize)).str_frame_info_details,
            (*ptr_header_data).num_time_slots,
            audio_object_type,
        ) == 0
        {
            return 0 as IA_ERRORCODE;
        }
        if (**ptr_frame_data.offset(0 as core::ffi::c_int as isize)).coupling_mode != 0 {
            memcpy(
                &mut (**ptr_frame_data.offset(1 as core::ffi::c_int as isize))
                    .str_frame_info_details as *mut ia_frame_info_struct
                    as *mut core::ffi::c_void,
                &mut (**ptr_frame_data.offset(0 as core::ffi::c_int as isize))
                    .str_frame_info_details as *mut ia_frame_info_struct
                    as *const core::ffi::c_void,
                ::core::mem::size_of::<ia_frame_info_struct>() as size_t,
            );
            if audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int
                || audio_object_type == AOT_ER_AAC_LD as core::ffi::c_int
            {
                (**ptr_frame_data.offset(1 as core::ffi::c_int as isize)).amp_res = (**ptr_frame_data
                    .offset(0 as core::ffi::c_int as isize))
                    .amp_res;
            }
            num_ch = 1 as core::ffi::c_int as WORD32;
        }
        i += 1;
    }
    if (**ptr_frame_data.offset(0 as core::ffi::c_int as isize)).coupling_mode != 0
        && usac_flag != 0
    {
        ixheaacd_sbr_env_dtdf_data(
            *ptr_frame_data.offset(0 as core::ffi::c_int as isize),
            it_bit_buff,
            (*ptr_header_data).usac_flag,
        );
        ixheaacd_sbr_env_dtdf_data(
            *ptr_frame_data.offset(1 as core::ffi::c_int as isize),
            it_bit_buff,
            (*ptr_header_data).usac_flag,
        );
        i = 0 as core::ffi::c_int as WORD32;
        while i
            < (*(*ptr_header_data).pstr_freq_band_data).num_if_bands as core::ffi::c_int
        {
            (**ptr_frame_data.offset(0 as core::ffi::c_int as isize))
                .sbr_invf_mode_prev[i as usize] = (**ptr_frame_data
                .offset(0 as core::ffi::c_int as isize))
                .sbr_invf_mode[i as usize];
            (**ptr_frame_data.offset(1 as core::ffi::c_int as isize))
                .sbr_invf_mode_prev[i as usize] = (**ptr_frame_data
                .offset(1 as core::ffi::c_int as isize))
                .sbr_invf_mode[i as usize];
            (**ptr_frame_data.offset(0 as core::ffi::c_int as isize))
                .sbr_invf_mode[i as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                ESBR_INVF_MODE_BITS,
            );
            (**ptr_frame_data.offset(1 as core::ffi::c_int as isize))
                .sbr_invf_mode[i as usize] = (**ptr_frame_data
                .offset(0 as core::ffi::c_int as isize))
                .sbr_invf_mode[i as usize];
            i += 1;
        }
        if ixheaacd_read_sbr_env_data(
            ptr_header_data,
            *ptr_frame_data.offset(0 as core::ffi::c_int as isize),
            it_bit_buff,
            env_extr_tables_ptr,
            audio_object_type,
        ) == 0
        {
            return 0 as IA_ERRORCODE;
        }
        ixheaacd_read_sbr_noise_floor_data(
            ptr_header_data,
            *ptr_frame_data.offset(0 as core::ffi::c_int as isize),
            it_bit_buff,
            env_extr_tables_ptr,
        );
        if ixheaacd_read_sbr_env_data(
            ptr_header_data,
            *ptr_frame_data.offset(1 as core::ffi::c_int as isize),
            it_bit_buff,
            env_extr_tables_ptr,
            audio_object_type,
        ) == 0
        {
            return 0 as IA_ERRORCODE;
        }
        ixheaacd_read_sbr_noise_floor_data(
            ptr_header_data,
            *ptr_frame_data.offset(1 as core::ffi::c_int as isize),
            it_bit_buff,
            env_extr_tables_ptr,
        );
        memset(
            ((**ptr_frame_data.offset(0 as core::ffi::c_int as isize)).add_harmonics)
                .as_mut_ptr() as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ((*(*ptr_header_data).pstr_freq_band_data)
                .num_sf_bands[1 as core::ffi::c_int as usize] as size_t)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
        );
        memset(
            ((**ptr_frame_data.offset(1 as core::ffi::c_int as isize)).add_harmonics)
                .as_mut_ptr() as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ((*(*ptr_header_data).pstr_freq_band_data)
                .num_sf_bands[1 as core::ffi::c_int as usize] as size_t)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
        );
    } else {
        ixheaacd_sbr_env_dtdf_data(
            *ptr_frame_data.offset(0 as core::ffi::c_int as isize),
            it_bit_buff,
            (*ptr_header_data).usac_flag,
        );
        ixheaacd_sbr_env_dtdf_data(
            *ptr_frame_data.offset(1 as core::ffi::c_int as isize),
            it_bit_buff,
            (*ptr_header_data).usac_flag,
        );
        if (**ptr_frame_data.offset(0 as core::ffi::c_int as isize))
            .del_cod_dir_arr[0 as core::ffi::c_int as usize] as core::ffi::c_int
            == DTDF_DIR_FREQ
            && (**ptr_frame_data.offset(1 as core::ffi::c_int as isize))
                .del_cod_dir_arr[0 as core::ffi::c_int as usize] as core::ffi::c_int
                == DTDF_DIR_FREQ
        {
            (*ptr_header_data).err_flag = 0 as core::ffi::c_int as FLAG;
        }
        k = 0 as core::ffi::c_int as WORD32;
        while k < num_ch {
            i = 0 as core::ffi::c_int as WORD32;
            while i < num_if_bands {
                (**ptr_frame_data.offset(k as isize)).sbr_invf_mode_prev[i as usize] = (**ptr_frame_data
                    .offset(k as isize))
                    .sbr_invf_mode[i as usize];
                (**ptr_frame_data.offset(k as isize)).sbr_invf_mode[i as usize] = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    SBR_INVERSE_FILT_MODE_BITS,
                );
                i += 1;
            }
            k += 1;
        }
        if (**ptr_frame_data.offset(0 as core::ffi::c_int as isize)).coupling_mode != 0 {
            memcpy(
                ((**ptr_frame_data.offset(1 as core::ffi::c_int as isize))
                    .sbr_invf_mode_prev)
                    .as_mut_ptr() as *mut core::ffi::c_void,
                ((**ptr_frame_data.offset(1 as core::ffi::c_int as isize)).sbr_invf_mode)
                    .as_mut_ptr() as *const core::ffi::c_void,
                (::core::mem::size_of::<WORD32>() as size_t)
                    .wrapping_mul(num_if_bands as size_t),
            );
            memcpy(
                ((**ptr_frame_data.offset(1 as core::ffi::c_int as isize)).sbr_invf_mode)
                    .as_mut_ptr() as *mut core::ffi::c_void,
                ((**ptr_frame_data.offset(0 as core::ffi::c_int as isize)).sbr_invf_mode)
                    .as_mut_ptr() as *const core::ffi::c_void,
                (::core::mem::size_of::<WORD32>() as size_t)
                    .wrapping_mul(num_if_bands as size_t),
            );
            if ixheaacd_read_sbr_env_data(
                ptr_header_data,
                *ptr_frame_data.offset(0 as core::ffi::c_int as isize),
                it_bit_buff,
                env_extr_tables_ptr,
                audio_object_type,
            ) == 0
            {
                return 0 as IA_ERRORCODE;
            }
            ixheaacd_read_sbr_noise_floor_data(
                ptr_header_data,
                *ptr_frame_data.offset(0 as core::ffi::c_int as isize),
                it_bit_buff,
                env_extr_tables_ptr,
            );
            if ixheaacd_read_sbr_env_data(
                ptr_header_data,
                *ptr_frame_data.offset(1 as core::ffi::c_int as isize),
                it_bit_buff,
                env_extr_tables_ptr,
                audio_object_type,
            ) == 0
            {
                return 0 as IA_ERRORCODE;
            }
        } else {
            if ixheaacd_read_sbr_env_data(
                ptr_header_data,
                *ptr_frame_data.offset(0 as core::ffi::c_int as isize),
                it_bit_buff,
                env_extr_tables_ptr,
                audio_object_type,
            ) == 0
            {
                return 0 as IA_ERRORCODE;
            }
            if ixheaacd_read_sbr_env_data(
                ptr_header_data,
                *ptr_frame_data.offset(1 as core::ffi::c_int as isize),
                it_bit_buff,
                env_extr_tables_ptr,
                audio_object_type,
            ) == 0
            {
                return 0 as IA_ERRORCODE;
            }
            ixheaacd_read_sbr_noise_floor_data(
                ptr_header_data,
                *ptr_frame_data.offset(0 as core::ffi::c_int as isize),
                it_bit_buff,
                env_extr_tables_ptr,
            );
        }
        ixheaacd_read_sbr_noise_floor_data(
            ptr_header_data,
            *ptr_frame_data.offset(1 as core::ffi::c_int as isize),
            it_bit_buff,
            env_extr_tables_ptr,
        );
    }
    bit = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD) as WORD32;
    if bit != 0 {
        ixheaacd_sbr_sin_coding_data(
            ptr_header_data,
            *ptr_frame_data.offset(0 as core::ffi::c_int as isize),
            it_bit_buff,
        );
    } else {
        memset(
            ((**ptr_frame_data.offset(0 as core::ffi::c_int as isize)).add_harmonics)
                .as_mut_ptr() as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<FLAG>() as size_t)
                .wrapping_mul(MAX_FREQ_COEFFS as size_t),
        );
    }
    bit = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD) as WORD32;
    if bit != 0 {
        ixheaacd_sbr_sin_coding_data(
            ptr_header_data,
            *ptr_frame_data.offset(1 as core::ffi::c_int as isize),
            it_bit_buff,
        );
    } else {
        memset(
            ((**ptr_frame_data.offset(1 as core::ffi::c_int as isize)).add_harmonics)
                .as_mut_ptr() as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<FLAG>() as size_t)
                .wrapping_mul(MAX_FREQ_COEFFS as size_t),
        );
    }
    if usac_flag == 0 {
        let mut err_0: IA_ERRORCODE = ixheaacd_read_extn_data(
            ptr_header_data,
            0 as *mut ia_ps_dec_struct,
            it_bit_buff,
            (*ptr_sbr_tables).ps_tables_ptr,
            ptr_frame_data as *mut core::ffi::c_void,
            SBR_ID_CPE as core::ffi::c_int as WORD32,
        );
        if err_0 as core::ffi::c_uint == IA_FATAL_ERROR {
            return err_0;
        }
    }
    return 1 as IA_ERRORCODE;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_sbr_env_dtdf_data(
    mut ptr_frame_data: *mut ia_sbr_frame_info_data_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut usac_flag: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut num_env: WORD32 = (*ptr_frame_data).str_frame_info_details.num_env as WORD32;
    let mut num_noise_env: WORD32 = (*ptr_frame_data)
        .str_frame_info_details
        .num_noise_env as WORD32;
    let mut p_coding_dir_vec: *mut WORD16 = ((*ptr_frame_data).del_cod_dir_arr)
        .as_mut_ptr();
    let mut p_coding_dir_noise_vec: *mut WORD16 = ((*ptr_frame_data)
        .del_cod_dir_noise_arr)
        .as_mut_ptr();
    let mut usac_independency_flag: WORD32 = (*ptr_frame_data).usac_independency_flag
        as WORD32;
    if usac_flag != 0 {
        if usac_independency_flag != 0 {
            *p_coding_dir_vec = 0 as WORD16;
            p_coding_dir_vec = p_coding_dir_vec.offset(1);
        } else {
            *p_coding_dir_vec = ixheaacd_read_bits_buf(it_bit_buff, SBR_DEL_COD_DIR_BITS)
                as WORD16;
            p_coding_dir_vec = p_coding_dir_vec.offset(1);
        }
        i = (num_env as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        while i >= 1 as core::ffi::c_int {
            let fresh6 = p_coding_dir_vec;
            p_coding_dir_vec = p_coding_dir_vec.offset(1);
            *fresh6 = ixheaacd_read_bits_buf(it_bit_buff, SBR_DEL_COD_DIR_BITS)
                as WORD16;
            i -= 1;
        }
        if usac_independency_flag != 0 {
            *p_coding_dir_noise_vec = 0 as WORD16;
            p_coding_dir_noise_vec = p_coding_dir_noise_vec.offset(1);
        } else {
            *p_coding_dir_noise_vec = ixheaacd_read_bits_buf(
                it_bit_buff,
                SBR_DEL_COD_DIR_BITS,
            ) as WORD16;
            p_coding_dir_noise_vec = p_coding_dir_noise_vec.offset(1);
        }
        i = (num_noise_env as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        while i >= 1 as core::ffi::c_int {
            let fresh7 = p_coding_dir_noise_vec;
            p_coding_dir_noise_vec = p_coding_dir_noise_vec.offset(1);
            *fresh7 = ixheaacd_read_bits_buf(it_bit_buff, SBR_DEL_COD_DIR_BITS)
                as WORD16;
            i -= 1;
        }
    } else {
        i = (num_env as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        while i >= 0 as core::ffi::c_int {
            let fresh8 = p_coding_dir_vec;
            p_coding_dir_vec = p_coding_dir_vec.offset(1);
            *fresh8 = ixheaacd_read_bits_buf(it_bit_buff, SBR_DEL_COD_DIR_BITS)
                as WORD16;
            i -= 1;
        }
        i = (num_noise_env as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        while i >= 0 as core::ffi::c_int {
            let fresh9 = p_coding_dir_noise_vec;
            p_coding_dir_noise_vec = p_coding_dir_noise_vec.offset(1);
            *fresh9 = ixheaacd_read_bits_buf(it_bit_buff, SBR_DEL_COD_DIR_BITS)
                as WORD16;
            i -= 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_read_env_data(
    mut ptr_frame_data: *mut ia_sbr_frame_info_data_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut hcb_t: ia_huffman_data_type,
    mut hcb_f: ia_huffman_data_type,
    mut idx_t: *mut WORD32,
    mut idx_f: *mut WORD32,
    mut no_band: *mut WORD16,
    mut num_env: WORD32,
    mut env_data_tbl_comp_factor: WORD32,
    mut start_bits: WORD32,
    mut start_bits_balance: WORD32,
    mut num_noise_env: WORD32,
    mut lav: WORD32,
    mut usac_flag: WORD32,
) -> VOID {
    let mut j: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut ixheaacd_drc_offset: WORD32 = 0 as WORD32;
    let mut coupling_mode: WORD32 = (*ptr_frame_data).coupling_mode;
    let mut delta: WORD32 = 0;
    let mut bits: WORD32 = 0;
    let mut shift: WORD32 = 0;
    let mut p_coding_dir_vec: *mut WORD16 = 0 as *mut WORD16;
    let mut p_sbr_sf: *mut WORD16 = 0 as *mut WORD16;
    let mut index: WORD16 = 0;
    let mut length: WORD16 = 0;
    let mut readword: WORD32 = 0;
    let mut p_sbr_sf_float: *mut FLOAT32 = 0 as *mut FLOAT32;
    if num_noise_env != 0 {
        p_coding_dir_vec = ((*ptr_frame_data).del_cod_dir_noise_arr).as_mut_ptr();
        p_sbr_sf = ((*ptr_frame_data).int_noise_floor).as_mut_ptr();
        p_sbr_sf_float = ((*ptr_frame_data).flt_noise_floor).as_mut_ptr();
    } else {
        p_coding_dir_vec = ((*ptr_frame_data).del_cod_dir_arr).as_mut_ptr();
        p_sbr_sf = ((*ptr_frame_data).int_env_sf_arr).as_mut_ptr();
        p_sbr_sf_float = ((*ptr_frame_data).flt_env_sf_arr).as_mut_ptr();
    }
    if coupling_mode == COUPLING_BAL {
        bits = start_bits_balance;
        shift = env_data_tbl_comp_factor;
    } else {
        bits = start_bits;
        shift = 0 as core::ffi::c_int as WORD32;
    }
    j = 0 as core::ffi::c_int as WORD32;
    while j < num_env {
        let mut h: ia_huffman_data_type = 0 as *const UWORD16;
        let mut idx_tab: *const WORD32 = 0 as *const WORD32;
        let mut dtdf_dir_flag: WORD32 = *p_coding_dir_vec.offset(j as isize) as WORD32;
        if dtdf_dir_flag == DTDF_DIR_FREQ {
            *p_sbr_sf.offset(ixheaacd_drc_offset as isize) = (ixheaacd_read_bits_buf(
                it_bit_buff,
                bits as WORD,
            ) << shift) as WORD16;
            *p_sbr_sf_float.offset(ixheaacd_drc_offset as isize) = *p_sbr_sf
                .offset(ixheaacd_drc_offset as isize) as FLOAT32;
            h = hcb_f;
            idx_tab = idx_f;
        } else {
            h = hcb_t;
            idx_tab = idx_t;
        }
        i = 1 as WORD32 - dtdf_dir_flag;
        while i < *no_band.offset(j as isize) as core::ffi::c_int {
            if (*it_bit_buff).cnt_bits < 20 as core::ffi::c_int {
                readword = ixheaacd_show_bits_buf(
                    it_bit_buff,
                    (*it_bit_buff).cnt_bits as WORD,
                );
                readword = readword << 32 as WORD32 - (*it_bit_buff).cnt_bits;
            } else {
                readword = ixheaacd_show_bits_buf(it_bit_buff, 20 as WORD);
                readword = readword << 12 as core::ffi::c_int;
            }
            ixheaacd_huffman_decode(
                readword,
                &mut index,
                &mut length,
                h,
                idx_tab as *const UWORD32,
            );
            delta = index as WORD32 - lav;
            ixheaacd_read_bits_buf(it_bit_buff, length as WORD);
            *p_sbr_sf.offset((ixheaacd_drc_offset + i) as isize) = (delta
                << env_data_tbl_comp_factor) as WORD16;
            *p_sbr_sf_float.offset((ixheaacd_drc_offset + i) as isize) = *p_sbr_sf
                .offset((ixheaacd_drc_offset + i) as isize) as FLOAT32;
            i += 1;
        }
        if usac_flag != 0 && num_noise_env == 0 as core::ffi::c_int {
            (*ptr_frame_data).inter_temp_shape_mode[j as usize] = 0 as core::ffi::c_int
                as WORD32;
            if (*ptr_frame_data).inter_tes_flag != 0 {
                let mut flag: WORD32 = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
                if flag != 0 {
                    (*ptr_frame_data).inter_temp_shape_mode[j as usize] = ixheaacd_read_bits_buf(
                        it_bit_buff,
                        2 as WORD,
                    );
                }
            }
        }
        ixheaacd_drc_offset += *no_band.offset(j as isize) as core::ffi::c_int;
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_read_sbr_noise_floor_data(
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut ptr_frame_data: *mut ia_sbr_frame_info_data_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut env_extr_tables_ptr: *mut ia_env_extr_tables_struct,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut coupling_mode: WORD32 = 0;
    let mut num_noise_bands: [WORD16; 2] = [0; 2];
    let mut hcb_noise_env: ia_huffman_data_type = 0 as *const UWORD16;
    let mut hcb_noise: ia_huffman_data_type = 0 as *const UWORD16;
    let mut idx_noise_env: *mut WORD32 = 0 as *mut WORD32;
    let mut idx_noise: *mut WORD32 = 0 as *mut WORD32;
    let mut lav: WORD32 = 0;
    let mut env_data_tbl_comp_factor: WORD32 = 0;
    let mut start_bits: WORD32 = 0;
    let mut start_bits_balance: WORD32 = 0;
    let mut num_noise_env: WORD32 = (*ptr_frame_data)
        .str_frame_info_details
        .num_noise_env as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_noise_env {
        num_noise_bands[i as usize] = (*(*ptr_header_data).pstr_freq_band_data)
            .num_nf_bands;
        i += 1;
    }
    start_bits = SBR_BEGIN_NOISE_BITS_AMPLITUDE_RESOLUTION_3_0 as WORD32;
    start_bits_balance = SBR_BEGIN_NOISE_BITS_BALNCE_AMPLITUDE_RESOLUTION_3_0 as WORD32;
    coupling_mode = (*ptr_frame_data).coupling_mode;
    if coupling_mode == COUPLING_BAL {
        lav = 12 as core::ffi::c_int as WORD32;
        hcb_noise = &mut (*env_extr_tables_ptr)
            .ixheaacd_t_huffman_noise_bal_3_0db_inp_table as *mut [WORD16; 26]
            as ia_huffman_data_type;
        idx_noise = ((*env_extr_tables_ptr).ixheaacd_t_huffman_noise_bal_3_0db_idx_table)
            .as_mut_ptr();
        hcb_noise_env = &mut (*env_extr_tables_ptr)
            .ixheaacd_f_huffman_env_bal_3_0db_inp_table as *mut [WORD16; 26]
            as ia_huffman_data_type;
        idx_noise_env = ((*env_extr_tables_ptr)
            .ixheaacd_f_huffman_env_bal_3_0db_idx_table)
            .as_mut_ptr();
        env_data_tbl_comp_factor = 1 as core::ffi::c_int as WORD32;
    } else {
        lav = 31 as core::ffi::c_int as WORD32;
        hcb_noise = &mut (*env_extr_tables_ptr).ixheaacd_t_huffman_noise_3_0db_inp_table
            as *mut [WORD16; 64] as ia_huffman_data_type;
        idx_noise = ((*env_extr_tables_ptr).ixheaacd_t_huffman_noise_3_0db_idx_table)
            .as_mut_ptr();
        hcb_noise_env = &mut (*env_extr_tables_ptr)
            .ixheaacd_f_huffman_env_3_0db_inp_table as *mut [WORD16; 64]
            as ia_huffman_data_type;
        idx_noise_env = ((*env_extr_tables_ptr).ixheaacd_f_huffman_env_3_0db_idx_table)
            .as_mut_ptr();
        env_data_tbl_comp_factor = 0 as core::ffi::c_int as WORD32;
    }
    ixheaacd_read_env_data(
        ptr_frame_data,
        it_bit_buff,
        hcb_noise,
        hcb_noise_env,
        idx_noise,
        idx_noise_env,
        &mut *num_noise_bands.as_mut_ptr().offset(0 as core::ffi::c_int as isize),
        num_noise_env,
        env_data_tbl_comp_factor,
        start_bits,
        start_bits_balance,
        1 as WORD32,
        lav,
        (*ptr_header_data).usac_flag,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_read_sbr_env_data(
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut ptr_frame_data: *mut ia_sbr_frame_info_data_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut env_extr_tables_ptr: *mut ia_env_extr_tables_struct,
    mut audio_object_type: WORD,
) -> WORD16 {
    let mut coupling_mode: WORD32 = (*ptr_frame_data).coupling_mode;
    let mut idx_t: *mut WORD32 = 0 as *mut WORD32;
    let mut idx_f: *mut WORD32 = 0 as *mut WORD32;
    let mut lav: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut no_band: [WORD16; 8] = [0; 8];
    let mut amp_res: WORD32 = 0;
    let mut num_env: WORD32 = 0;
    let mut env_data_tbl_comp_factor: WORD32 = 0;
    let mut start_bits: WORD32 = 0;
    let mut start_bits_balance: WORD32 = 0;
    let mut p_freq_res: *mut WORD16 = ((*ptr_frame_data).str_frame_info_details.freq_res)
        .as_mut_ptr();
    let mut p_num_sf_bands: *mut WORD16 = ((*(*ptr_header_data).pstr_freq_band_data)
        .num_sf_bands)
        .as_mut_ptr();
    let mut hcb_t: ia_huffman_data_type = 0 as *const UWORD16;
    let mut hcb_f: ia_huffman_data_type = 0 as *const UWORD16;
    amp_res = (*ptr_header_data).amp_res as WORD32;
    num_env = (*ptr_frame_data).str_frame_info_details.num_env as WORD32;
    (*ptr_frame_data).num_env_sfac = 0 as WORD16;
    if (*ptr_frame_data).str_frame_info_details.frame_class as core::ffi::c_int == FIXFIX
        && num_env == 1 as core::ffi::c_int
    {
        if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
            && audio_object_type != AOT_ER_AAC_LD as core::ffi::c_int
        {
            amp_res = SBR_AMPLITUDE_RESOLUTION_1_5 as WORD32;
        } else {
            amp_res = (*ptr_frame_data).amp_res as WORD32;
        }
    }
    (*ptr_frame_data).amp_res = amp_res as WORD16;
    if amp_res == SBR_AMPLITUDE_RESOLUTION_3_0 {
        start_bits = SBR_BEGIN_ENVN_BITS_AMPLITUDE_RESOLUTION_3_0 as WORD32;
        start_bits_balance = SBR_BEGIN_ENVN_BITS_BALNCE_AMPLITUDE_RESOLUTION_3_0
            as WORD32;
    } else {
        start_bits = SBR_BEGIN_ENVN_BITS_AMPLITUDE_RESOLUTION_1_5 as WORD32;
        start_bits_balance = SBR_BEGIN_ENVN_BITS_BALNCE_AMPLITUDE_RESOLUTION_1_5
            as WORD32;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_env {
        let fresh5 = p_freq_res;
        p_freq_res = p_freq_res.offset(1);
        no_band[i as usize] = *p_num_sf_bands.offset(*fresh5 as isize);
        (*ptr_frame_data).num_env_sfac = ixheaac_add16(
            (*ptr_frame_data).num_env_sfac,
            no_band[i as usize],
        );
        i += 1;
    }
    if (*ptr_frame_data).num_env_sfac as core::ffi::c_int > MAX_NUM_ENVELOPE_VALUES {
        return 0 as WORD16;
    }
    if coupling_mode == COUPLING_BAL {
        env_data_tbl_comp_factor = 1 as core::ffi::c_int as WORD32;
        if amp_res == SBR_AMPLITUDE_RESOLUTION_1_5 {
            lav = 24 as core::ffi::c_int as WORD32;
            hcb_t = &mut (*env_extr_tables_ptr)
                .ixheaacd_t_huffman_env_bal_1_5db_inp_table as *mut [WORD16; 50]
                as ia_huffman_data_type;
            idx_t = ((*env_extr_tables_ptr).ixheaacd_t_huffman_env_bal_1_5db_idx_table)
                .as_mut_ptr();
            hcb_f = &mut (*env_extr_tables_ptr)
                .ixheaacd_f_huffman_env_bal_1_5db_inp_table as *mut [WORD16; 50]
                as ia_huffman_data_type;
            idx_f = ((*env_extr_tables_ptr).ixheaacd_f_huffman_env_bal_1_5db_idx_table)
                .as_mut_ptr();
        } else {
            lav = 12 as core::ffi::c_int as WORD32;
            hcb_t = &mut (*env_extr_tables_ptr)
                .ixheaacd_t_huffman_env_bal_3_0db_inp_table as *mut [WORD16; 26]
                as ia_huffman_data_type;
            idx_t = ((*env_extr_tables_ptr).ixheaacd_t_huffman_env_bal_3_0db_idx_table)
                .as_mut_ptr();
            hcb_f = &mut (*env_extr_tables_ptr)
                .ixheaacd_f_huffman_env_bal_3_0db_inp_table as *mut [WORD16; 26]
                as ia_huffman_data_type;
            idx_f = ((*env_extr_tables_ptr).ixheaacd_f_huffman_env_bal_3_0db_idx_table)
                .as_mut_ptr();
        }
    } else {
        env_data_tbl_comp_factor = 0 as core::ffi::c_int as WORD32;
        if amp_res == SBR_AMPLITUDE_RESOLUTION_1_5 {
            lav = 60 as core::ffi::c_int as WORD32;
            hcb_t = &mut (*env_extr_tables_ptr).ixheaacd_t_huffman_env_1_5db_inp_table
                as *mut [WORD16; 122] as ia_huffman_data_type;
            idx_t = ((*env_extr_tables_ptr).ixheaacd_t_huffman_env_1_5db_idx_table)
                .as_mut_ptr();
            hcb_f = &mut (*env_extr_tables_ptr).ixheaacd_f_huffman_env_1_5db_inp_table
                as *mut [WORD16; 122] as ia_huffman_data_type;
            idx_f = ((*env_extr_tables_ptr).ixheaacd_f_huffman_env_1_5db_idx_table)
                .as_mut_ptr();
        } else {
            lav = 31 as core::ffi::c_int as WORD32;
            hcb_t = &mut (*env_extr_tables_ptr).ixheaacd_t_huffman_env_3_0db_inp_table
                as *mut [WORD16; 64] as ia_huffman_data_type;
            idx_t = ((*env_extr_tables_ptr).ixheaacd_t_huffman_env_3_0db_idx_table)
                .as_mut_ptr();
            hcb_f = &mut (*env_extr_tables_ptr).ixheaacd_f_huffman_env_3_0db_inp_table
                as *mut [WORD16; 64] as ia_huffman_data_type;
            idx_f = ((*env_extr_tables_ptr).ixheaacd_f_huffman_env_3_0db_idx_table)
                .as_mut_ptr();
        }
    }
    ixheaacd_read_env_data(
        ptr_frame_data,
        it_bit_buff,
        hcb_t,
        hcb_f,
        idx_t,
        idx_f,
        &mut *no_band.as_mut_ptr().offset(0 as core::ffi::c_int as isize),
        num_env,
        env_data_tbl_comp_factor,
        start_bits,
        start_bits_balance,
        0 as WORD32,
        lav,
        (*ptr_header_data).usac_flag,
    );
    return 1 as WORD16;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_extract_frame_info_ld(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut h_frame_data: *mut ia_sbr_frame_info_data_struct,
) -> IA_ERRORCODE {
    let mut abs_bord_lead: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut num_rel_lead: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut bs_num_env: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut frame_class: core::ffi::c_int = 0;
    let mut temp: core::ffi::c_int = 0;
    let mut env: core::ffi::c_int = 0;
    let mut k: core::ffi::c_int = 0;
    let mut abs_bord_trail: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut middle_bord: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut bs_num_noise: core::ffi::c_int = 0;
    let mut transient_env_temp: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut bs_transient_position: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut err: IA_ERRORCODE = IA_NO_ERROR;
    let mut time_border: [WORD16; 9] = [0; 9];
    let mut time_border_noise: [WORD16; 3] = [0; 3];
    let mut f: [WORD16; 9] = [0; 9];
    let mut rel_bord_lead: [core::ffi::c_int; 7] = [0 as core::ffi::c_int; 7];
    let mut v_frame_info: *mut ia_frame_info_struct = &mut (*h_frame_data)
        .str_frame_info_details;
    let mut numTimeSlots: core::ffi::c_int = (*h_frame_data).num_time_slots
        as core::ffi::c_int;
    frame_class = ixheaacd_read_bits_buf(it_bit_buff, SBRLD_CLA_BITS)
        as core::ffi::c_int;
    (*v_frame_info).frame_class = frame_class as WORD16;
    match frame_class {
        FIXFIX => {
            temp = ixheaacd_read_bits_buf(it_bit_buff, SBR_ENV_BITS) as core::ffi::c_int;
            bs_num_env = (1 as core::ffi::c_int) << temp;
            if bs_num_env == 1 as core::ffi::c_int {
                (*h_frame_data).amp_res = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    SBR_AMPLITUDE_RESOLUTION_BITS,
                ) as WORD16;
            }
            f[0 as core::ffi::c_int as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                SBR_RES_BITS,
            ) as WORD16;
            env = 1 as core::ffi::c_int;
            while env < bs_num_env {
                f[env as usize] = f[0 as core::ffi::c_int as usize];
                env += 1;
            }
        }
        LD_TRAN => {
            bs_transient_position = ixheaacd_read_bits_buf(it_bit_buff, SBR_TRAN_BITS)
                as core::ffi::c_int;
            (*v_frame_info).frame_class = 0 as WORD16;
            if numTimeSlots != 16 as core::ffi::c_int
                && bs_transient_position >= LD_ENV_TBL_480
            {
                return -(1 as IA_ERRORCODE);
            }
            bs_num_env = if numTimeSlots == 16 as core::ffi::c_int {
                ixheaacd_ld_env_table_512[bs_transient_position
                    as usize][SBR_ENVT_NUMENV as usize]
            } else {
                ixheaacd_ld_env_table_480[bs_transient_position
                    as usize][SBR_ENVT_NUMENV as usize]
            };
            env = 0 as core::ffi::c_int;
            while env < bs_num_env {
                f[env as usize] = ixheaacd_read_bits_buf(it_bit_buff, SBR_RES_BITS)
                    as WORD16;
                env += 1;
            }
        }
        _ => {}
    }
    match frame_class {
        FIXFIX => {
            abs_bord_lead = 0 as core::ffi::c_int;
            abs_bord_trail = numTimeSlots;
            num_rel_lead = bs_num_env - 1 as core::ffi::c_int;
            k = 0 as core::ffi::c_int;
            while k < num_rel_lead {
                rel_bord_lead[k as usize] = ixheaacd_ld_env_table_time_slot[(num_rel_lead
                    - 1 as core::ffi::c_int) as usize];
                k += 1;
            }
            time_border[0 as core::ffi::c_int as usize] = abs_bord_lead as WORD16;
            time_border[bs_num_env as usize] = abs_bord_trail as WORD16;
            env = 1 as core::ffi::c_int;
            while env <= num_rel_lead {
                time_border[env as usize] = abs_bord_lead as WORD16;
                k = 0 as core::ffi::c_int;
                while k <= env - 1 as core::ffi::c_int {
                    time_border[env as usize] = (time_border[env as usize]
                        as core::ffi::c_int + rel_bord_lead[k as usize]) as WORD16;
                    k += 1;
                }
                env += 1;
            }
        }
        LD_TRAN => {
            time_border[0 as core::ffi::c_int as usize] = 0 as WORD16;
            time_border[bs_num_env as usize] = numTimeSlots as WORD16;
            k = 1 as core::ffi::c_int;
            while k < bs_num_env {
                time_border[k as usize] = (if numTimeSlots == 16 as core::ffi::c_int {
                    ixheaacd_ld_env_table_512[bs_transient_position as usize][k as usize]
                } else {
                    ixheaacd_ld_env_table_480[bs_transient_position as usize][k as usize]
                }) as WORD16;
                k += 1;
            }
        }
        _ => {
            time_border[0 as core::ffi::c_int as usize] = 0 as WORD16;
        }
    }
    match frame_class {
        FIXFIX => {
            middle_bord = bs_num_env / 2 as core::ffi::c_int;
        }
        LD_TRAN => {
            middle_bord = 1 as core::ffi::c_int;
        }
        _ => {}
    }
    time_border_noise[0 as core::ffi::c_int as usize] = time_border[0 as core::ffi::c_int
        as usize];
    if bs_num_env > 1 as core::ffi::c_int {
        time_border_noise[1 as core::ffi::c_int as usize] = time_border[middle_bord
            as usize];
        time_border_noise[2 as core::ffi::c_int as usize] = time_border[bs_num_env
            as usize];
        bs_num_noise = 2 as core::ffi::c_int;
    } else {
        time_border_noise[1 as core::ffi::c_int as usize] = time_border[bs_num_env
            as usize];
        bs_num_noise = 1 as core::ffi::c_int;
    }
    match frame_class {
        FIXFIX => {
            transient_env_temp = -(1 as core::ffi::c_int);
        }
        LD_TRAN => {
            transient_env_temp = if numTimeSlots == 16 as core::ffi::c_int {
                ixheaacd_ld_env_table_512[bs_transient_position
                    as usize][SBR_ENVT_TRANIDX as usize]
            } else {
                ixheaacd_ld_env_table_480[bs_transient_position
                    as usize][SBR_ENVT_TRANIDX as usize]
            };
        }
        _ => {}
    }
    (*v_frame_info).num_env = bs_num_env as WORD16;
    memcpy(
        ((*v_frame_info).border_vec).as_mut_ptr() as *mut core::ffi::c_void,
        time_border.as_mut_ptr() as *const core::ffi::c_void,
        ((bs_num_env + 1 as core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<WORD16>() as size_t),
    );
    memcpy(
        ((*v_frame_info).freq_res).as_mut_ptr() as *mut core::ffi::c_void,
        f.as_mut_ptr() as *const core::ffi::c_void,
        (bs_num_env as size_t).wrapping_mul(::core::mem::size_of::<WORD16>() as size_t),
    );
    (*v_frame_info).transient_env = transient_env_temp as WORD16;
    (*v_frame_info).num_noise_env = bs_num_noise as WORD16;
    memcpy(
        ((*v_frame_info).noise_border_vec).as_mut_ptr() as *mut core::ffi::c_void,
        time_border_noise.as_mut_ptr() as *const core::ffi::c_void,
        ((bs_num_noise + 1 as core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<WORD16>() as size_t),
    );
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_pvc_time_freq_grid_info(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut ptr_frame_data: *mut ia_sbr_frame_info_data_struct,
) -> WORD32 {
    let mut bs_num_env: WORD32 = 0 as WORD32;
    let mut bs_num_noise: WORD32 = 0 as WORD32;
    let mut time_border: [WORD32; 9] = [0; 9];
    let mut time_border_noise: [WORD32; 3] = [0; 3];
    let mut pvc_time_border: [WORD32; 9] = [0; 9];
    let mut pvc_time_border_noise: [WORD32; 3] = [0; 3];
    let mut bs_freq_res: [WORD32; 9] = [0; 9];
    let mut var_len: WORD32 = 0;
    let mut p_frame_info: *mut ia_frame_info_struct = &mut (*ptr_frame_data)
        .str_frame_info_details;
    let mut pvc_frame_info: *mut ia_frame_info_struct = &mut (*ptr_frame_data)
        .str_pvc_frame_info;
    let mut i: WORD32 = 0;
    let mut prev_sbr_mode: WORD32 = (*ptr_frame_data).prev_sbr_mode as WORD32;
    let mut tmp: WORD32 = 0;
    let mut bs_noise_pos: WORD32 = 0;
    bs_noise_pos = ixheaacd_read_bits_buf(it_bit_buff, 4 as WORD);
    tmp = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
    if tmp == 0 as core::ffi::c_int {
        (*ptr_frame_data).var_len = 0 as core::ffi::c_int as WORD32;
    } else {
        tmp = ixheaacd_read_bits_buf(it_bit_buff, 2 as WORD);
        (*ptr_frame_data).var_len = (tmp as core::ffi::c_int + 1 as core::ffi::c_int)
            as WORD32;
        if (*ptr_frame_data).var_len > 3 as core::ffi::c_int {
            return -(1 as WORD32);
        }
    }
    var_len = (*ptr_frame_data).var_len;
    if (*p_frame_info).num_env as core::ffi::c_int > 0 as core::ffi::c_int {
        time_border[0 as core::ffi::c_int as usize] = ((*p_frame_info)
            .border_vec[(*p_frame_info).num_env as usize] as core::ffi::c_int
            - 16 as core::ffi::c_int) as WORD32;
    } else {
        time_border[0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int as WORD32;
    }
    if time_border[0 as core::ffi::c_int as usize] < 0 as core::ffi::c_int {
        return -(1 as WORD32);
    }
    pvc_time_border[0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int as WORD32;
    bs_freq_res[0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int as WORD32;
    if (*ptr_frame_data).prev_sbr_mode == 0 as core::ffi::c_int {
        pvc_time_border[0 as core::ffi::c_int as usize] = time_border[0
            as core::ffi::c_int as usize];
    }
    if bs_noise_pos == 0 as core::ffi::c_int {
        time_border[1 as core::ffi::c_int as usize] = 16 as WORD32 + var_len;
        pvc_time_border[1 as core::ffi::c_int as usize] = 16 as core::ffi::c_int
            as WORD32;
        bs_num_noise = 1 as core::ffi::c_int as WORD32;
        bs_num_env = 1 as core::ffi::c_int as WORD32;
    } else {
        time_border[1 as core::ffi::c_int as usize] = bs_noise_pos;
        pvc_time_border[1 as core::ffi::c_int as usize] = bs_noise_pos;
        time_border[2 as core::ffi::c_int as usize] = 16 as WORD32 + var_len;
        pvc_time_border[2 as core::ffi::c_int as usize] = 16 as core::ffi::c_int
            as WORD32;
        bs_freq_res[1 as core::ffi::c_int as usize] = 0 as core::ffi::c_int as WORD32;
        bs_num_noise = 2 as core::ffi::c_int as WORD32;
        bs_num_env = 2 as core::ffi::c_int as WORD32;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < 3 as core::ffi::c_int {
        time_border_noise[i as usize] = time_border[i as usize];
        pvc_time_border_noise[i as usize] = pvc_time_border[i as usize];
        i += 1;
    }
    if prev_sbr_mode == ORIG_SBR as core::ffi::c_int {
        pvc_time_border[0 as core::ffi::c_int as usize] = time_border[0
            as core::ffi::c_int as usize];
        pvc_time_border_noise[0 as core::ffi::c_int as usize] = time_border[0
            as core::ffi::c_int as usize];
    }
    (*pvc_frame_info).num_env = bs_num_env as WORD16;
    i = 0 as core::ffi::c_int as WORD32;
    while i < bs_num_env as core::ffi::c_int + 1 as core::ffi::c_int {
        (*pvc_frame_info).border_vec[i as usize] = pvc_time_border[i as usize] as WORD16;
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < bs_num_env {
        (*pvc_frame_info).freq_res[i as usize] = bs_freq_res[i as usize] as WORD16;
        i += 1;
    }
    (*pvc_frame_info).transient_env = -(1 as core::ffi::c_int) as WORD16;
    (*pvc_frame_info).num_noise_env = bs_num_noise as WORD16;
    i = 0 as core::ffi::c_int as WORD32;
    while i < bs_num_noise as core::ffi::c_int + 1 as core::ffi::c_int {
        (*pvc_frame_info).noise_border_vec[i as usize] = pvc_time_border_noise[i
            as usize] as WORD16;
        i += 1;
    }
    (*p_frame_info).num_env = bs_num_env as WORD16;
    i = 0 as core::ffi::c_int as WORD32;
    while i < bs_num_env as core::ffi::c_int + 1 as core::ffi::c_int {
        (*p_frame_info).border_vec[i as usize] = time_border[i as usize] as WORD16;
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < bs_num_env {
        (*p_frame_info).freq_res[i as usize] = bs_freq_res[i as usize] as WORD16;
        i += 1;
    }
    (*p_frame_info).transient_env = -(1 as core::ffi::c_int) as WORD16;
    (*p_frame_info).num_noise_env = bs_num_noise as WORD16;
    i = 0 as core::ffi::c_int as WORD32;
    while i < bs_num_noise as core::ffi::c_int + 1 as core::ffi::c_int {
        (*p_frame_info).noise_border_vec[i as usize] = time_border_noise[i as usize]
            as WORD16;
        i += 1;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_sbr_time_freq_grid_info(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut ptr_frame_data: *mut ia_sbr_frame_info_data_struct,
    mut env_extr_tables_ptr: *mut ia_env_extr_tables_struct,
    mut number_of_time_slots: WORD16,
) -> WORD16 {
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut bs_num_rel: WORD32 = 0 as WORD32;
    let mut bs_pointer_bits: WORD32 = 0 as WORD32;
    let mut bs_num_env: WORD32 = 0 as WORD32;
    let mut border: WORD32 = 0;
    let mut bs_pointer: WORD32 = 0;
    let mut bs_var_bord: WORD32 = 0 as WORD32;
    let mut temp: WORD32 = 0 as WORD32;
    let mut freq_res_0: WORD32 = 0 as WORD32;
    let mut frame_class: WORD32 = 0;
    let mut abs_bord_lead: WORD32 = 0;
    let mut abs_bord_trail: WORD32 = 0;
    let mut num_rel_trail: WORD32 = 0;
    let mut num_rel_lead: WORD32 = 0;
    static mut pointer_bits_array: [WORD32; 7] = [
        1 as core::ffi::c_int,
        2 as core::ffi::c_int,
        2 as core::ffi::c_int,
        3 as core::ffi::c_int,
        3 as core::ffi::c_int,
        3 as core::ffi::c_int,
        3 as core::ffi::c_int,
    ];
    let mut p_fixfix_tab: *mut ia_frame_info_struct = 0 as *mut ia_frame_info_struct;
    let mut p_frame_info: *mut ia_frame_info_struct = &mut (*ptr_frame_data)
        .str_frame_info_details;
    frame_class = ixheaacd_read_bits_buf(it_bit_buff, SBR_FRAME_CLASS_BITS);
    (*p_frame_info).frame_class = frame_class as WORD16;
    match frame_class {
        FIXFIX => {
            temp = ixheaacd_read_bits_buf(it_bit_buff, SBR_ENV_BITS + SBR_FRQ_RES_BITS);
            bs_num_env = ((temp as core::ffi::c_int & 0x6 as core::ffi::c_int)
                >> SBR_FRQ_RES_BITS) as WORD32;
            if number_of_time_slots as core::ffi::c_int != 15 as core::ffi::c_int {
                p_fixfix_tab = &mut *((*env_extr_tables_ptr).sbr_frame_info1_2_4_16)
                    .as_mut_ptr()
                    .offset(bs_num_env as isize) as *mut ia_frame_info_struct;
            } else {
                if bs_num_env > 2 as core::ffi::c_int {
                    return 0 as WORD16;
                }
                p_fixfix_tab = &mut *((*env_extr_tables_ptr).sbr_frame_info1_2_4_16)
                    .as_mut_ptr()
                    .offset(
                        (bs_num_env as core::ffi::c_int + 4 as core::ffi::c_int) as isize,
                    ) as *mut ia_frame_info_struct;
            }
            memcpy(
                p_frame_info as *mut core::ffi::c_void,
                p_fixfix_tab as *const core::ffi::c_void,
                ::core::mem::size_of::<ia_frame_info_struct>() as size_t,
            );
            bs_num_env = ((1 as core::ffi::c_int) << bs_num_env) as WORD32;
            freq_res_0 = (temp as core::ffi::c_int & 0x1 as core::ffi::c_int) as WORD32;
            if freq_res_0 == 0 {
                memset(
                    &mut *((*p_frame_info).freq_res)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize) as *mut WORD16
                        as *mut core::ffi::c_void,
                    0 as core::ffi::c_int,
                    (::core::mem::size_of::<WORD16>() as size_t)
                        .wrapping_mul(bs_num_env as size_t),
                );
            }
        }
        FIXVAR => {
            bs_var_bord = ixheaacd_read_bits_buf(
                it_bit_buff,
                SBR_VAR_BORD_BITS + SBR_NUM_BITS,
            );
            bs_num_rel = (bs_var_bord as core::ffi::c_int & 3 as core::ffi::c_int)
                as WORD32;
            bs_var_bord = bs_var_bord >> SBR_NUM_BITS;
            bs_num_env = (bs_num_rel as core::ffi::c_int + 1 as core::ffi::c_int)
                as WORD32;
            (*p_frame_info).border_vec[0 as core::ffi::c_int as usize] = 0 as WORD16;
            if number_of_time_slots as core::ffi::c_int != 15 as core::ffi::c_int {
                border = (bs_var_bord as core::ffi::c_int + SBR_TIME_SLOTS) as WORD32;
            } else {
                border = (bs_var_bord as core::ffi::c_int
                    + number_of_time_slots as core::ffi::c_int) as WORD32;
            }
            (*p_frame_info).border_vec[bs_num_env as usize] = border as WORD16;
            k = bs_num_rel;
            while k > 0 as core::ffi::c_int {
                temp = ixheaacd_read_bits_buf(it_bit_buff, SBR_REL_BITS);
                border = (border as core::ffi::c_int
                    - (((temp as core::ffi::c_int) << 1 as core::ffi::c_int)
                        + 2 as core::ffi::c_int)) as WORD32;
                if border < 0 as core::ffi::c_int {
                    border = 0 as core::ffi::c_int as WORD32;
                }
                (*p_frame_info).border_vec[k as usize] = border as WORD16;
                k -= 1;
            }
            bs_pointer_bits = pointer_bits_array[bs_num_rel as usize];
            bs_pointer = ixheaacd_read_bits_buf(it_bit_buff, bs_pointer_bits as WORD);
            if bs_pointer as core::ffi::c_int
                - (bs_num_rel as core::ffi::c_int + 1 as core::ffi::c_int)
                > 0 as core::ffi::c_int
            {
                return 0 as WORD16;
            }
            k = bs_num_rel;
            while k >= 0 as core::ffi::c_int {
                (*p_frame_info).freq_res[k as usize] = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    SBR_FRQ_RES_BITS,
                ) as WORD16;
                k -= 1;
            }
            if bs_pointer != 0 {
                (*p_frame_info).transient_env = (bs_num_env + 1 as WORD32 - bs_pointer)
                    as WORD16;
            } else {
                (*p_frame_info).transient_env = -(1 as core::ffi::c_int) as WORD16;
            }
            if bs_pointer == 0 as core::ffi::c_int || bs_pointer == 1 as core::ffi::c_int
            {
                (*p_frame_info).noise_border_vec[1 as core::ffi::c_int as usize] = (*p_frame_info)
                    .border_vec[bs_num_rel as usize];
            } else {
                (*p_frame_info).noise_border_vec[1 as core::ffi::c_int as usize] = (*p_frame_info)
                    .border_vec[(*p_frame_info).transient_env as usize];
            }
        }
        VARFIX => {
            bs_var_bord = ixheaacd_read_bits_buf(
                it_bit_buff,
                SBR_VAR_BORD_BITS + SBR_NUM_BITS,
            );
            bs_num_rel = (bs_var_bord as core::ffi::c_int & 3 as core::ffi::c_int)
                as WORD32;
            bs_var_bord = bs_var_bord >> SBR_NUM_BITS;
            bs_num_env = (bs_num_rel as core::ffi::c_int + 1 as core::ffi::c_int)
                as WORD32;
            border = bs_var_bord;
            (*p_frame_info).border_vec[0 as core::ffi::c_int as usize] = border
                as WORD16;
            k = 1 as core::ffi::c_int as WORD32;
            while k <= bs_num_rel {
                temp = ixheaacd_read_bits_buf(it_bit_buff, SBR_REL_BITS);
                border = (border as core::ffi::c_int
                    + (((temp as core::ffi::c_int) << 1 as core::ffi::c_int)
                        + 2 as core::ffi::c_int)) as WORD32;
                if number_of_time_slots as core::ffi::c_int != 15 as core::ffi::c_int {
                    if border > SBR_TIME_SLOTS {
                        border = SBR_TIME_SLOTS as WORD32;
                    }
                } else if border > number_of_time_slots as core::ffi::c_int {
                    border = number_of_time_slots as WORD32;
                }
                (*p_frame_info).border_vec[k as usize] = border as WORD16;
                k += 1;
            }
            if number_of_time_slots as core::ffi::c_int != 15 as core::ffi::c_int {
                (*p_frame_info).border_vec[k as usize] = SBR_TIME_SLOTS as WORD16;
            } else {
                (*p_frame_info).border_vec[k as usize] = number_of_time_slots;
            }
            bs_pointer_bits = pointer_bits_array[bs_num_rel as usize];
            bs_pointer = ixheaacd_read_bits_buf(it_bit_buff, bs_pointer_bits as WORD);
            if bs_pointer as core::ffi::c_int
                - (bs_num_rel as core::ffi::c_int + 1 as core::ffi::c_int)
                > 0 as core::ffi::c_int
            {
                return 0 as WORD16;
            }
            if bs_pointer == 0 as core::ffi::c_int
                || bs_pointer as core::ffi::c_int - 1 as core::ffi::c_int
                    == 0 as core::ffi::c_int
            {
                (*p_frame_info).transient_env = -(1 as core::ffi::c_int) as WORD16;
            } else {
                (*p_frame_info).transient_env = (bs_pointer as core::ffi::c_int
                    - 1 as core::ffi::c_int) as WORD16;
            }
            k = 0 as core::ffi::c_int as WORD32;
            while k <= bs_num_rel {
                (*p_frame_info).freq_res[k as usize] = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    SBR_FRQ_RES_BITS,
                ) as WORD16;
                k += 1;
            }
            match bs_pointer {
                0 => {
                    (*p_frame_info).noise_border_vec[1 as core::ffi::c_int as usize] = (*p_frame_info)
                        .border_vec[1 as core::ffi::c_int as usize];
                }
                1 => {
                    (*p_frame_info).noise_border_vec[1 as core::ffi::c_int as usize] = (*p_frame_info)
                        .border_vec[bs_num_rel as usize];
                }
                _ => {
                    (*p_frame_info).noise_border_vec[1 as core::ffi::c_int as usize] = (*p_frame_info)
                        .border_vec[(*p_frame_info).transient_env as WORD32 as usize];
                }
            }
        }
        VARVAR => {
            abs_bord_lead = ixheaacd_read_bits_buf(
                it_bit_buff,
                2 as WORD * SBR_VAR_BORD_BITS + 2 as WORD * SBR_NUM_BITS,
            );
            if number_of_time_slots as core::ffi::c_int != 15 as core::ffi::c_int {
                abs_bord_trail = (((abs_bord_lead as core::ffi::c_int
                    & 0x30 as core::ffi::c_int) >> 2 as core::ffi::c_int * SBR_NUM_BITS)
                    + SBR_TIME_SLOTS) as WORD32;
            } else {
                abs_bord_trail = (((abs_bord_lead as core::ffi::c_int
                    & 0x30 as core::ffi::c_int) >> 2 as core::ffi::c_int * SBR_NUM_BITS)
                    + number_of_time_slots as core::ffi::c_int) as WORD32;
            }
            num_rel_trail = ((abs_bord_lead as core::ffi::c_int
                & 0xc as core::ffi::c_int) >> SBR_NUM_BITS) as WORD32;
            num_rel_lead = (abs_bord_lead as core::ffi::c_int & 0x3 as core::ffi::c_int)
                as WORD32;
            abs_bord_lead = abs_bord_lead
                >> SBR_VAR_BORD_BITS + 2 as core::ffi::c_int * SBR_NUM_BITS;
            bs_num_env = (num_rel_trail as core::ffi::c_int
                + num_rel_lead as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
            border = abs_bord_lead;
            (*p_frame_info).border_vec[0 as core::ffi::c_int as usize] = border
                as WORD16;
            k = 1 as core::ffi::c_int as WORD32;
            while k <= num_rel_trail {
                temp = ixheaacd_read_bits_buf(it_bit_buff, SBR_REL_BITS);
                border = (border as core::ffi::c_int
                    + (((temp as core::ffi::c_int) << 1 as core::ffi::c_int)
                        + 2 as core::ffi::c_int)) as WORD32;
                (*p_frame_info).border_vec[k as usize] = border as WORD16;
                k += 1;
            }
            border = abs_bord_trail;
            i = bs_num_env;
            (*p_frame_info).border_vec[i as usize] = border as WORD16;
            k = 0 as core::ffi::c_int as WORD32;
            while k < num_rel_lead {
                temp = ixheaacd_read_bits_buf(it_bit_buff, SBR_REL_BITS);
                border = (border as core::ffi::c_int
                    - (((temp as core::ffi::c_int) << 1 as core::ffi::c_int)
                        + 2 as core::ffi::c_int)) as WORD32;
                i -= 1;
                (*p_frame_info).border_vec[i as usize] = border as WORD16;
                k += 1;
            }
            bs_pointer_bits = pointer_bits_array[(num_rel_trail + num_rel_lead)
                as usize];
            bs_pointer = ixheaacd_read_bits_buf(it_bit_buff, bs_pointer_bits as WORD);
            if bs_pointer as core::ffi::c_int
                - (num_rel_trail as core::ffi::c_int + num_rel_lead as core::ffi::c_int
                    + 1 as core::ffi::c_int) > 0 as core::ffi::c_int
            {
                return 0 as WORD16;
            }
            if bs_pointer != 0 {
                (*p_frame_info).transient_env = (bs_num_env + 1 as WORD32 - bs_pointer)
                    as WORD16;
            } else {
                (*p_frame_info).transient_env = -(1 as core::ffi::c_int) as WORD16;
            }
            k = 0 as core::ffi::c_int as WORD32;
            while k < bs_num_env {
                (*p_frame_info).freq_res[k as usize] = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    SBR_FRQ_RES_BITS,
                ) as WORD16;
                k += 1;
            }
            (*p_frame_info).noise_border_vec[0 as core::ffi::c_int as usize] = abs_bord_lead
                as WORD16;
            if bs_num_env == 1 as core::ffi::c_int {
                (*p_frame_info).noise_border_vec[1 as core::ffi::c_int as usize] = abs_bord_trail
                    as WORD16;
            } else {
                if bs_pointer == 0 as core::ffi::c_int
                    || bs_pointer as core::ffi::c_int - 1 as core::ffi::c_int
                        == 0 as core::ffi::c_int
                {
                    (*p_frame_info).noise_border_vec[1 as core::ffi::c_int as usize] = (*p_frame_info)
                        .border_vec[(bs_num_env as core::ffi::c_int
                        - 1 as core::ffi::c_int) as usize];
                } else {
                    (*p_frame_info).noise_border_vec[1 as core::ffi::c_int as usize] = (*p_frame_info)
                        .border_vec[(*p_frame_info).transient_env as WORD32 as usize];
                }
                (*p_frame_info).noise_border_vec[2 as core::ffi::c_int as usize] = abs_bord_trail
                    as WORD16;
            }
        }
        _ => {}
    }
    (*p_frame_info).num_env = bs_num_env as WORD16;
    if bs_num_env == 1 as core::ffi::c_int {
        (*p_frame_info).num_noise_env = 1 as WORD16;
    } else {
        (*p_frame_info).num_noise_env = 2 as WORD16;
    }
    if frame_class == VARFIX || frame_class == FIXVAR {
        (*p_frame_info).noise_border_vec[0 as core::ffi::c_int as usize] = (*p_frame_info)
            .border_vec[0 as core::ffi::c_int as usize];
        (*p_frame_info).noise_border_vec[(*p_frame_info).num_noise_env as usize] = (*p_frame_info)
            .border_vec[bs_num_env as usize];
    }
    return 1 as WORD16;
}
pub const EXTENSION_ID_PS_CODING: core::ffi::c_int = 2;
pub const EXTENSION_ID_ENHSBR_CODING: core::ffi::c_int = 3;
