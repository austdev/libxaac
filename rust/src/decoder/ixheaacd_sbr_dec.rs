extern "C" {
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memmove(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn ixheaacd_low_pow_hf_generator(
        hf_generator: *mut ia_sbr_hf_generator_struct,
        qmf_buff_re: *mut *mut WORD32,
        degree_alias: *mut WORD16,
        first_slot_offset: WORD,
        last_slot_offset: WORD,
        num_if_bands: WORD,
        max_qmf_subband_aac: WORD,
        sbr_invf_mode: *mut WORD32,
        sbr_invf_mode_prev: *mut WORD32,
        norm_max: WORD32,
        ptr_qmf_matrix: *mut WORD32,
    ) -> VOID;
    fn ixheaacd_hf_generator(
        hf_generator: *mut ia_sbr_hf_generator_struct,
        sbr_scale_factor: *mut ia_sbr_scale_fact_struct,
        qmf_buff_re: *mut *mut WORD32,
        qmf_buff_im: *mut *mut WORD32,
        time_step: WORD,
        first_slot_offset: WORD,
        last_slot_offset: WORD,
        num_if_bands: WORD,
        max_qmf_subband_aac: WORD,
        sbr_invf_mode: *mut WORD32,
        sbr_invf_mode_prev: *mut WORD32,
        ptr_qmf_matrix: *mut WORD32,
        audio_object_type: WORD,
    ) -> VOID;
    fn ixheaacd_init_ps_scale(
        ptr_ps_dec: *mut ia_ps_dec_struct,
        sbr_scale_factor: *mut ia_sbr_scale_fact_struct,
    ) -> VOID;
    fn ixheaacd_esbr_apply_ps(
        ptr_ps_dec: *mut ia_ps_dec_struct,
        pp_qmf_buf_re_left: *mut *mut FLOAT32,
        pp_qmf_buf_im_left: *mut *mut FLOAT32,
        pp_qmf_buf_re_right: *mut *mut FLOAT32,
        pp_qmf_buf_im_right: *mut *mut FLOAT32,
        usb: WORD32,
        ptr_ps_tables: *mut ia_ps_tables_struct,
        num_time_slot: WORD32,
    ) -> VOID;
    fn ixheaacd_cplx_anal_qmffilt(
        time_inp: *const WORD16,
        sbr_scale_factor: *mut ia_sbr_scale_fact_struct,
        qmf_real: *mut *mut WORD32,
        qmf_imag: *mut *mut WORD32,
        qmf_bank: *mut ia_sbr_qmf_filter_bank_struct,
        qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
        ch_fac: WORD,
        low_pow_flag: WORD32,
        audio_object_type: WORD,
    ) -> VOID;
    fn ixheaacd_cplx_anal_qmffilt_32(
        time_inp: *const WORD32,
        sbr_scale_factor: *mut ia_sbr_scale_fact_struct,
        qmf_real: *mut *mut WORD32,
        qmf_imag: *mut *mut WORD32,
        qmf_bank: *mut ia_sbr_qmf_filter_bank_struct,
        qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
        ch_fac: WORD,
        ldsbr_present: WORD32,
    ) -> VOID;
    fn ixheaacd_cplx_synt_qmffilt(
        qmf_real: *mut *mut WORD32,
        qmf_im: *mut *mut WORD32,
        split_slot: WORD32,
        qmf_real_out: *mut *mut WORD32,
        qmf_imag_out: *mut *mut WORD32,
        sbr_scale_factor: *mut ia_sbr_scale_fact_struct,
        time_out: *mut WORD16,
        qmf_bank: *mut ia_sbr_qmf_filter_bank_struct,
        ptr_ps_dec: *mut ia_ps_dec_struct,
        active: FLAG,
        low_pow_flag: FLAG,
        sbr_tables_ptr: *mut ia_sbr_tables_struct,
        pstr_common_tables: *mut ixheaacd_misc_tables,
        ch_fac: WORD,
        drc_on: FLAG,
        drc_sbr_factors: *mut [WORD32; 64],
        audio_object_type: WORD32,
    ) -> VOID;
    fn ixheaacd_esbr_qmfanal32_winadd(
        inp1: *mut WORD32,
        inp2: *mut WORD32,
        tmp_qmf_1: *mut WORD32,
        tmp_qmf_2: *mut WORD32,
        out: *mut WORD32,
        num_band: WORD32,
    ) -> VOID;
    fn ixheaacd_esbr_fwd_modulation(
        time_in: *const WORD32,
        r_subband: *mut WORD32,
        i_subband: *mut WORD32,
        qmf_bank: *mut ia_sbr_qmf_filter_bank_struct,
        qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
    ) -> VOID;
    fn ixheaacd_esbr_inv_modulation(
        qmf_real: *mut WORD32,
        syn_qmf: *mut ia_sbr_qmf_filter_bank_struct,
        qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
        no_synthesis_channels: WORD32,
    ) -> VOID;
    fn ixheaacd_shiftrountine_with_rnd_hq(
        qmf_real: *mut WORD32,
        qmf_imag: *mut WORD32,
        filter_states: *mut WORD32,
        len: WORD32,
        shift: WORD32,
    ) -> VOID;
    fn ixheaacd_esbr_qmfsyn64_winadd(
        tmp1: *mut WORD32,
        tmp2: *mut WORD32,
        inp1: *mut WORD32,
        sample_buffer: *mut WORD32,
        ch_fac: WORD32,
    ) -> VOID;
    fn ixheaacd_esbr_qmfsyn32_winadd(
        tmp1: *mut WORD32,
        tmp2: *mut WORD32,
        inp1: *mut WORD32,
        sample_buffer: *mut WORD32,
        ch_fac: WORD32,
    ) -> VOID;
    fn ixheaacd_calc_sbrenvelope(
        sbr_scale_factor: *mut ia_sbr_scale_fact_struct,
        ptr_sbr_calc_env: *mut ia_sbr_calc_env_struct,
        ptr_header_data: *mut ia_sbr_header_data_struct,
        ptr_frame_data: *mut ia_sbr_frame_info_data_struct,
        ptr_prev_frame_data: *mut ia_sbr_prev_frame_data_struct,
        anal_buf_real_mant: *mut *mut WORD32,
        anal_buf_imag_mant: *mut *mut WORD32,
        degree_alias: *mut WORD16,
        low_pow_flag: FLAG,
        ptr_sbr_tables: *mut ia_sbr_tables_struct,
        pstr_common_tables: *mut ixheaacd_misc_tables,
        ptr_qmf_matrix: *mut WORD32,
        audio_object_type: WORD32,
    ) -> IA_ERRORCODE;
    fn ixheaacd_pvc_process(
        ptr_pvc_data: *mut ia_pvc_data_struct,
        first_bnd_idx: WORD16,
        first_pvc_timelost: WORD32,
        a_qmfl: *mut FLOAT32,
        a_qmfh: *mut FLOAT32,
    ) -> WORD32;
    fn ixheaacd_qmf_hbe_apply(
        h_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
        qmf_buf_real: *mut [FLOAT32; 64],
        qmf_buf_imag: *mut [FLOAT32; 64],
        num_columns: WORD32,
        pv_qmf_buf_real: *mut [FLOAT32; 64],
        pv_qmf_buf_imag: *mut [FLOAT32; 64],
        pitch_in_bins: WORD32,
        ptr_header_data: *mut ia_sbr_header_data_struct,
    ) -> WORD32;
    fn ixheaacd_dft_hbe_apply(
        ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
        qmf_buf_real: *mut [FLOAT32; 64],
        qmf_buf_imag: *mut [FLOAT32; 64],
        num_columns: WORD32,
        pv_qmf_buf_real: *mut [FLOAT32; 64],
        pv_qmf_buf_imag: *mut [FLOAT32; 64],
        pitch_in_bins: WORD32,
        dft_hbe_scratch_buf: *mut FLOAT32,
    ) -> WORD32;
    fn ixheaacd_sbr_env_calc(
        frame_data: *mut ia_sbr_frame_info_data_struct,
        input_real: *mut [FLOAT32; 64],
        input_imag: *mut [FLOAT32; 64],
        input_real1: *mut [FLOAT32; 64],
        input_imag1: *mut [FLOAT32; 64],
        x_over_qmf: *mut WORD32,
        scratch_buff: *mut FLOAT32,
        env_out: *mut FLOAT32,
        ldmps_present: WORD32,
        ec_flag: WORD32,
    ) -> WORD32;
    fn ixheaacd_generate_hf(
        ptr_src_buf_real: *mut [FLOAT32; 64],
        ptr_src_buf_imag: *mut [FLOAT32; 64],
        ptr_ph_vocod_buf_real: *mut [FLOAT32; 64],
        ptr_ph_vocod_buf_imag: *mut [FLOAT32; 64],
        ptr_dst_buf_real: *mut [FLOAT32; 64],
        ptr_dst_buf_imag: *mut [FLOAT32; 64],
        ptr_frame_data: *mut ia_sbr_frame_info_data_struct,
        ptr_header_data: *mut ia_sbr_header_data_struct,
        audio_object_type: WORD32,
        time_slots: WORD32,
        ec_flag: WORD32,
    ) -> WORD32;
    fn ixheaacd_clr_subsamples(
        ptr_qmf_buf: *mut WORD32,
        num: WORD32,
        size: WORD32,
    ) -> VOID;
    fn ixheaacd_rescale_x_overlap(
        ptr_sbr_dec: *mut ia_sbr_dec_struct,
        ptr_header_data: *mut ia_sbr_header_data_struct,
        ptr_frame_data: *mut ia_sbr_frame_info_data_struct,
        ptr_frame_data_prev: *mut ia_sbr_prev_frame_data_struct,
        pp_overlap_buffer_real: *mut *mut WORD32,
        pp_overlap_buffer_imag: *mut *mut WORD32,
        low_pow_flag: FLAG,
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
pub type FLOAT32 = core::ffi::c_float;
pub type VOID = ();
pub type FLAG = core::ffi::c_int;
pub type WORD = core::ffi::c_int;
pub type IA_ERRORCODE = WORD32;
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
pub const AOT_AAC_LC: AUDIO_OBJECT_TYPE = 2;
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
pub const AOT_AAC_MAIN: AUDIO_OBJECT_TYPE = 1;
pub const AOT_NULL_OBJECT: AUDIO_OBJECT_TYPE = 0;
pub const Q25: core::ffi::c_int = 33554432 as core::ffi::c_int;
pub const MAX_16: WORD16 = 0x7fff as core::ffi::c_int as WORD16;
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
unsafe extern "C" fn ixheaac_sub16_sat(mut op1: WORD16, mut op2: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    let mut diff: WORD32 = 0;
    diff = (op1 as core::ffi::c_int - op2 as core::ffi::c_int) as WORD32;
    var_out = ixheaac_sat16(diff);
    return var_out;
}
pub const SBR_UPSAMPLE_FAC: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SBR_UPSAMPLE_IDX_4_1: core::ffi::c_int = 3 as core::ffi::c_int;
pub const MAX_FRAME_SIZE: core::ffi::c_int = 1024 as core::ffi::c_int;
pub const NO_SYNTHESIS_CHANNELS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const NO_ANALYSIS_CHANNELS: core::ffi::c_int = NO_SYNTHESIS_CHANNELS
    / SBR_UPSAMPLE_FAC;
pub const NO_SYNTHESIS_CHANNELS_DOWN_SAMPLED: core::ffi::c_int = NO_SYNTHESIS_CHANNELS
    / 2 as core::ffi::c_int;
pub const NO_SYN_ANA_CHANNELS: core::ffi::c_int = NO_SYNTHESIS_CHANNELS
    - NO_ANALYSIS_CHANNELS;
pub const LPC_ORDER: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_COLS: core::ffi::c_int = MAX_FRAME_SIZE / NO_ANALYSIS_CHANNELS;
pub const MAX_OV_COLS: core::ffi::c_int = 6 as core::ffi::c_int;
pub const MAX_ENV_COLS: core::ffi::c_int = MAX_COLS + MAX_OV_COLS;
pub const MAX_ENV_COLS_960: core::ffi::c_int = 30 as core::ffi::c_int + MAX_OV_COLS;
pub const MAX_NUM_PATCHES: core::ffi::c_int = 6 as core::ffi::c_int;
pub const PS_STEREO: core::ffi::c_int = 3 as core::ffi::c_int;
pub const HYBRID_FILTER_DELAY: core::ffi::c_int = 6 as core::ffi::c_int;
pub const IA_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_FATAL_ERROR: core::ffi::c_uint = 0x80000000 as core::ffi::c_uint;
pub const SBR_HF_ADJ_OFFSET: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MPS_SBR_DELAY: core::ffi::c_int = 6 as core::ffi::c_int;
pub const MPS_SBR_DELAY_960: core::ffi::c_int = 5 as core::ffi::c_int;
pub const ESBR_HBE_DELAY_OFFSET: core::ffi::c_int = 32 as core::ffi::c_int;
pub const ESBR_HBE_DELAY_OFFSET_960: core::ffi::c_int = 30 as core::ffi::c_int;
pub const MAX_NUM_QMF_BANDS_ESBR: core::ffi::c_int = 128 as core::ffi::c_int;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_qmf_enrg_calc(
    mut ptr_sbr_dec: *mut ia_sbr_dec_struct,
    mut upsample_ratio_idx: WORD32,
    mut low_pow_flag: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    if upsample_ratio_idx == SBR_UPSAMPLE_IDX_4_1 {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as core::ffi::c_int {
            j = 0 as core::ffi::c_int as WORD32;
            while j < 16 as core::ffi::c_int {
                (*ptr_sbr_dec).qmf_energy_buf[i as usize][j as usize] = (*ptr_sbr_dec)
                    .qmf_buf_real[(2 as WORD32 + i) as usize][j as usize]
                    * (*ptr_sbr_dec)
                        .qmf_buf_real[(2 as WORD32 + i) as usize][j as usize];
                if low_pow_flag == 0 {
                    (*ptr_sbr_dec).qmf_energy_buf[i as usize][j as usize]
                        += (*ptr_sbr_dec)
                            .qmf_buf_imag[(2 as WORD32 + i) as usize][j as usize]
                            * (*ptr_sbr_dec)
                                .qmf_buf_imag[(2 as WORD32 + i) as usize][j as usize];
                }
                j += 1;
            }
            i += 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < 16 as core::ffi::c_int {
            j = 0 as core::ffi::c_int as WORD32;
            while j < 16 as core::ffi::c_int {
                (*ptr_sbr_dec).pvc_qmf_enrg_arr[(32 as WORD32 * i + j) as usize] = (((*ptr_sbr_dec)
                    .qmf_energy_buf[(4 as core::ffi::c_int * i as core::ffi::c_int
                    + 0 as core::ffi::c_int) as usize][j as usize]
                    + (*ptr_sbr_dec)
                        .qmf_energy_buf[(4 as core::ffi::c_int * i as core::ffi::c_int
                        + 1 as core::ffi::c_int) as usize][j as usize]
                    + (*ptr_sbr_dec)
                        .qmf_energy_buf[(4 as core::ffi::c_int * i as core::ffi::c_int
                        + 2 as core::ffi::c_int) as usize][j as usize]
                    + (*ptr_sbr_dec)
                        .qmf_energy_buf[(4 as core::ffi::c_int * i as core::ffi::c_int
                        + 3 as core::ffi::c_int) as usize][j as usize]) * 0.25f32)
                    as FLOAT32;
                j += 1;
            }
            i += 1;
        }
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as core::ffi::c_int {
            j = 0 as core::ffi::c_int as WORD32;
            while j < 32 as core::ffi::c_int {
                (*ptr_sbr_dec).qmf_energy_buf[i as usize][j as usize] = (*ptr_sbr_dec)
                    .qmf_buf_real[(2 as WORD32 + i) as usize][j as usize]
                    * (*ptr_sbr_dec)
                        .qmf_buf_real[(2 as WORD32 + i) as usize][j as usize];
                if low_pow_flag == 0 {
                    (*ptr_sbr_dec).qmf_energy_buf[i as usize][j as usize]
                        += (*ptr_sbr_dec)
                            .qmf_buf_imag[(2 as WORD32 + i) as usize][j as usize]
                            * (*ptr_sbr_dec)
                                .qmf_buf_imag[(2 as WORD32 + i) as usize][j as usize];
                }
                j += 1;
            }
            i += 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < 16 as core::ffi::c_int {
            j = 0 as core::ffi::c_int as WORD32;
            while j < 32 as core::ffi::c_int {
                (*ptr_sbr_dec).pvc_qmf_enrg_arr[(32 as WORD32 * i + j) as usize] = (((*ptr_sbr_dec)
                    .qmf_energy_buf[(2 as core::ffi::c_int * i as core::ffi::c_int
                    + 0 as core::ffi::c_int) as usize][j as usize]
                    + (*ptr_sbr_dec)
                        .qmf_energy_buf[(2 as core::ffi::c_int * i as core::ffi::c_int
                        + 1 as core::ffi::c_int) as usize][j as usize]) * 0.5f32)
                    as FLOAT32;
                j += 1;
            }
            i += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hbe_repl_spec(
    mut x_over_qmf: *mut WORD32,
    mut qmf_buf_real: *mut [FLOAT32; 64],
    mut qmf_buf_imag: *mut [FLOAT32; 64],
    mut no_bins: WORD32,
    mut max_stretch: WORD32,
) -> VOID {
    let mut patch_bands: WORD32 = 0;
    let mut patch: WORD32 = 0;
    let mut band: WORD32 = 0;
    let mut col: WORD32 = 0;
    let mut target: WORD32 = 0;
    let mut source_bands: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut num_patches: WORD32 = 0 as WORD32;
    i = 1 as core::ffi::c_int as WORD32;
    while i < MAX_NUM_PATCHES {
        if *x_over_qmf.offset(i as isize) != 0 as core::ffi::c_int {
            num_patches += 1;
        }
        i += 1;
    }
    patch = (max_stretch as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while patch < num_patches {
        patch_bands = *x_over_qmf
            .offset((patch as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
            - *x_over_qmf.offset(patch as isize);
        target = *x_over_qmf.offset(patch as isize);
        source_bands = *x_over_qmf
            .offset((max_stretch as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
            - *x_over_qmf
                .offset(
                    (max_stretch as core::ffi::c_int - 2 as core::ffi::c_int) as isize,
                );
        while patch_bands > 0 as core::ffi::c_int {
            let mut ixheaacd_num_bands: WORD32 = source_bands;
            let mut start_band: WORD32 = *x_over_qmf
                .offset(
                    (max_stretch as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                ) - 1 as WORD32;
            if target + ixheaacd_num_bands
                >= *x_over_qmf
                    .offset((patch as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
            {
                ixheaacd_num_bands = *x_over_qmf
                    .offset((patch as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                    - target;
            }
            if (target as core::ffi::c_int + ixheaacd_num_bands as core::ffi::c_int
                - 1 as core::ffi::c_int & 1 as core::ffi::c_int)
                + (*x_over_qmf
                    .offset(
                        (max_stretch as core::ffi::c_int - 1 as core::ffi::c_int)
                            as isize,
                    ) - 1 as core::ffi::c_int & 1 as core::ffi::c_int)
                & 1 as core::ffi::c_int != 0
            {
                if ixheaacd_num_bands == source_bands {
                    ixheaacd_num_bands -= 1;
                } else {
                    start_band -= 1;
                }
            }
            if ixheaacd_num_bands == 0 {
                break;
            }
            col = 0 as core::ffi::c_int as WORD32;
            while col < no_bins {
                let mut i_0: WORD32 = 0 as WORD32;
                band = (target as core::ffi::c_int
                    + ixheaacd_num_bands as core::ffi::c_int - 1 as core::ffi::c_int)
                    as WORD32;
                if 64 as core::ffi::c_int <= band {
                    band = 63 as core::ffi::c_int as WORD32;
                }
                if *x_over_qmf
                    .offset((patch as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                    <= band
                {
                    band = (*x_over_qmf
                        .offset(
                            (patch as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                        ) - 1 as core::ffi::c_int) as WORD32;
                }
                i_0 = 0 as core::ffi::c_int as WORD32;
                while i_0 < ixheaacd_num_bands {
                    (*qmf_buf_real.offset(col as isize))[band as usize] = (*qmf_buf_real
                        .offset(col as isize))[(start_band - i_0) as usize];
                    (*qmf_buf_imag.offset(col as isize))[band as usize] = (*qmf_buf_imag
                        .offset(col as isize))[(start_band - i_0) as usize];
                    i_0 += 1;
                    band -= 1;
                }
                col += 1;
            }
            target += ixheaacd_num_bands;
            patch_bands -= ixheaacd_num_bands;
        }
        patch += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_esbr_analysis_filt_block(
    mut ptr_sbr_dec: *mut ia_sbr_dec_struct,
    mut sbr_tables_ptr: *mut ia_sbr_tables_struct,
    mut op_delay: WORD32,
) -> VOID {
    let mut core_coder_samples: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut ptr_filt_states: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr_filt_states_1: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr_filt_states_2: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr_temp: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr_win_coeffs_1: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr_win_coeffs_2: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr_win_coeffs: *mut WORD32 = 0 as *mut WORD32;
    let mut ploc_qmf_buf_real: *mut WORD32 = 0 as *mut WORD32;
    let mut ploc_qmf_buf_imag: *mut WORD32 = 0 as *mut WORD32;
    let mut local_qmf_buffer: [WORD32; 128] = [0 as core::ffi::c_int; 128];
    let mut anal_buf: [WORD32; 64] = [0; 64];
    let mut idx: WORD32 = 0;
    let mut z: WORD32 = 0;
    let mut core_syn_ch_index: WORD32 = 0;
    let mut gain: FLOAT32 = 0.;
    let mut filt_offset: WORD32 = 0;
    let mut num_columns: WORD32 = 0;
    let mut qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct = (*sbr_tables_ptr)
        .qmf_dec_tables_ptr;
    let mut pstr_qmf_anal_bank: *mut ia_sbr_qmf_filter_bank_struct = &mut (*ptr_sbr_dec)
        .str_codec_qmf_bank;
    core_coder_samples = (*ptr_sbr_dec).time_sample_buf;
    ptr_filt_states = (*pstr_qmf_anal_bank).state_new_samples_pos_low_32;
    ptr_win_coeffs_1 = (*pstr_qmf_anal_bank).filter_pos_32;
    num_columns = (*pstr_qmf_anal_bank).no_channels;
    match num_columns {
        16 => {
            ptr_win_coeffs_2 = ptr_win_coeffs_1.offset(64 as core::ffi::c_int as isize);
            gain = 128.0f32 as FLOAT32;
            filt_offset = 64 as core::ffi::c_int as WORD32;
        }
        24 => {
            ptr_win_coeffs_2 = ptr_win_coeffs_1.offset(24 as core::ffi::c_int as isize);
            gain = 12.0f32 as FLOAT32;
            filt_offset = 24 as core::ffi::c_int as WORD32;
        }
        32 => {
            ptr_win_coeffs_2 = ptr_win_coeffs_1.offset(64 as core::ffi::c_int as isize);
            gain = 256.0f32 as FLOAT32;
            filt_offset = 64 as core::ffi::c_int as WORD32;
        }
        _ => {
            ptr_win_coeffs_2 = ptr_win_coeffs_1.offset(64 as core::ffi::c_int as isize);
            gain = 256.0f32 as FLOAT32;
            filt_offset = 64 as core::ffi::c_int as WORD32;
        }
    }
    gain = 1.0f32 / gain;
    (*pstr_qmf_anal_bank).usb = num_columns as WORD16;
    ploc_qmf_buf_real = &mut *local_qmf_buffer
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
    ploc_qmf_buf_imag = &mut *local_qmf_buffer
        .as_mut_ptr()
        .offset(64 as core::ffi::c_int as isize) as *mut WORD32;
    ptr_filt_states_1 = (*pstr_qmf_anal_bank).anal_filter_states_32;
    ptr_filt_states_2 = ((*pstr_qmf_anal_bank).anal_filter_states_32)
        .offset(num_columns as isize);
    idx = 0 as core::ffi::c_int as WORD32;
    while idx < (*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as core::ffi::c_int {
        z = 0 as core::ffi::c_int as WORD32;
        while z < num_columns {
            *ptr_filt_states.offset((num_columns - 1 as WORD32 - z) as isize) = (*core_coder_samples
                .offset(z as isize)
                * ((1 as core::ffi::c_int) << 15 as core::ffi::c_int) as FLOAT32)
                as WORD32;
            z += 1;
        }
        ixheaacd_esbr_qmfanal32_winadd(
            ptr_filt_states_1,
            ptr_filt_states_2,
            ptr_win_coeffs_1,
            ptr_win_coeffs_2,
            anal_buf.as_mut_ptr(),
            num_columns,
        );
        core_coder_samples = core_coder_samples.offset(num_columns as isize);
        ptr_filt_states = ptr_filt_states.offset(-(num_columns as isize));
        if ptr_filt_states < (*pstr_qmf_anal_bank).anal_filter_states_32 {
            ptr_filt_states = ((*pstr_qmf_anal_bank).anal_filter_states_32)
                .offset((10 as WORD32 * num_columns) as isize)
                .offset(-(num_columns as isize));
        }
        ptr_temp = ptr_filt_states_1;
        ptr_filt_states_1 = ptr_filt_states_2;
        ptr_filt_states_2 = ptr_temp;
        ptr_win_coeffs_1 = ptr_win_coeffs_1.offset(filt_offset as isize);
        ptr_win_coeffs_2 = ptr_win_coeffs_2.offset(filt_offset as isize);
        ptr_win_coeffs = ptr_win_coeffs_1;
        ptr_win_coeffs_1 = ptr_win_coeffs_2;
        ptr_win_coeffs_2 = ptr_win_coeffs;
        if ptr_win_coeffs_2
            > ((*pstr_qmf_anal_bank).analy_win_coeff_32)
                .offset(
                    (filt_offset as core::ffi::c_int * 10 as core::ffi::c_int) as isize,
                )
        {
            ptr_win_coeffs_1 = (*pstr_qmf_anal_bank).analy_win_coeff_32;
            ptr_win_coeffs_2 = ((*pstr_qmf_anal_bank).analy_win_coeff_32)
                .offset(filt_offset as isize);
        }
        ixheaacd_esbr_fwd_modulation(
            anal_buf.as_mut_ptr(),
            &mut *ploc_qmf_buf_real.offset(0 as core::ffi::c_int as isize),
            &mut *ploc_qmf_buf_imag.offset(0 as core::ffi::c_int as isize),
            pstr_qmf_anal_bank,
            qmf_dec_tables_ptr,
        );
        core_syn_ch_index = num_columns;
        z = 0 as core::ffi::c_int as WORD32;
        while z < core_syn_ch_index {
            (*ptr_sbr_dec).qmf_buf_real[(op_delay + idx) as usize][z as usize] = *ploc_qmf_buf_real
                .offset(z as isize) as FLOAT32 * gain;
            (*ptr_sbr_dec).qmf_buf_imag[(op_delay + idx) as usize][z as usize] = *ploc_qmf_buf_imag
                .offset(z as isize) as FLOAT32 * gain;
            z += 1;
        }
        idx += 1;
    }
    (*pstr_qmf_anal_bank).filter_pos_32 = ptr_win_coeffs_1;
    (*pstr_qmf_anal_bank).state_new_samples_pos_low_32 = ptr_filt_states;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_esbr_synthesis_regrp(
    mut qmf_buf_real: *mut FLOAT32,
    mut qmf_buf_imag: *mut FLOAT32,
    mut ptr_sbr_dec: *mut ia_sbr_dec_struct,
    mut ptr_frame_data: *mut ia_sbr_frame_info_data_struct,
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut stereo_config_idx: WORD32,
    mut apply_processing: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut stop_border: WORD32 = 0 as WORD32;
    let mut num_anal_bands: WORD32 = (*ptr_sbr_dec).str_codec_qmf_bank.no_channels;
    let mut x_over_band: WORD32 = num_anal_bands;
    if apply_processing != 0 {
        if (*ptr_header_data).sbr_ratio_idx == SBR_UPSAMPLE_IDX_4_1 {
            stop_border = (4 as core::ffi::c_int
                * (*ptr_frame_data)
                    .str_frame_info_details
                    .border_vec[0 as core::ffi::c_int as usize] as core::ffi::c_int)
                as WORD32;
        } else {
            stop_border = (2 as core::ffi::c_int
                * (*ptr_frame_data)
                    .str_frame_info_details
                    .border_vec[0 as core::ffi::c_int as usize] as core::ffi::c_int)
                as WORD32;
        }
        x_over_band = (*(*ptr_header_data).pstr_freq_band_data).qmf_sb_prev as WORD32;
    }
    if stereo_config_idx > 0 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < stop_border {
            k = 0 as core::ffi::c_int as WORD32;
            while k < 3 as core::ffi::c_int {
                let fresh10 = qmf_buf_real;
                qmf_buf_real = qmf_buf_real.offset(1);
                *fresh10 = (*ptr_sbr_dec)
                    .qmf_buf_real[(2 as core::ffi::c_int + i as core::ffi::c_int
                    + HYBRID_FILTER_DELAY) as usize][k as usize];
                let fresh11 = qmf_buf_imag;
                qmf_buf_imag = qmf_buf_imag.offset(1);
                *fresh11 = (*ptr_sbr_dec)
                    .qmf_buf_imag[(2 as core::ffi::c_int + i as core::ffi::c_int
                    + HYBRID_FILTER_DELAY) as usize][k as usize];
                k += 1;
            }
            while k < x_over_band {
                let fresh12 = qmf_buf_real;
                qmf_buf_real = qmf_buf_real.offset(1);
                *fresh12 = (*ptr_sbr_dec)
                    .qmf_buf_real[(2 as WORD32 + i) as usize][k as usize];
                let fresh13 = qmf_buf_imag;
                qmf_buf_imag = qmf_buf_imag.offset(1);
                *fresh13 = (*ptr_sbr_dec)
                    .qmf_buf_imag[(2 as WORD32 + i) as usize][k as usize];
                k += 1;
            }
            while k < 64 as core::ffi::c_int {
                let fresh14 = qmf_buf_real;
                qmf_buf_real = qmf_buf_real.offset(1);
                *fresh14 = (*ptr_sbr_dec)
                    .sbr_qmf_out_real[(2 as WORD32 + i) as usize][k as usize];
                let fresh15 = qmf_buf_imag;
                qmf_buf_imag = qmf_buf_imag.offset(1);
                *fresh15 = (*ptr_sbr_dec)
                    .sbr_qmf_out_imag[(2 as WORD32 + i) as usize][k as usize];
                k += 1;
            }
            qmf_buf_real = qmf_buf_real.offset(14 as core::ffi::c_int as isize);
            qmf_buf_imag = qmf_buf_imag.offset(14 as core::ffi::c_int as isize);
            i += 1;
        }
        x_over_band = (*(*ptr_header_data).pstr_freq_band_data).sub_band_start as WORD32;
        while i < (*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as core::ffi::c_int {
            k = 0 as core::ffi::c_int as WORD32;
            while k < 3 as core::ffi::c_int {
                let fresh16 = qmf_buf_real;
                qmf_buf_real = qmf_buf_real.offset(1);
                *fresh16 = (*ptr_sbr_dec)
                    .qmf_buf_real[(2 as core::ffi::c_int + i as core::ffi::c_int
                    + HYBRID_FILTER_DELAY) as usize][k as usize];
                let fresh17 = qmf_buf_imag;
                qmf_buf_imag = qmf_buf_imag.offset(1);
                *fresh17 = (*ptr_sbr_dec)
                    .qmf_buf_imag[(2 as core::ffi::c_int + i as core::ffi::c_int
                    + HYBRID_FILTER_DELAY) as usize][k as usize];
                k += 1;
            }
            while k < x_over_band {
                let fresh18 = qmf_buf_real;
                qmf_buf_real = qmf_buf_real.offset(1);
                *fresh18 = (*ptr_sbr_dec)
                    .qmf_buf_real[(2 as WORD32 + i) as usize][k as usize];
                let fresh19 = qmf_buf_imag;
                qmf_buf_imag = qmf_buf_imag.offset(1);
                *fresh19 = (*ptr_sbr_dec)
                    .qmf_buf_imag[(2 as WORD32 + i) as usize][k as usize];
                k += 1;
            }
            while k < 64 as core::ffi::c_int {
                let fresh20 = qmf_buf_real;
                qmf_buf_real = qmf_buf_real.offset(1);
                *fresh20 = (*ptr_sbr_dec)
                    .sbr_qmf_out_real[(2 as WORD32 + i) as usize][k as usize];
                let fresh21 = qmf_buf_imag;
                qmf_buf_imag = qmf_buf_imag.offset(1);
                *fresh21 = (*ptr_sbr_dec)
                    .sbr_qmf_out_imag[(2 as WORD32 + i) as usize][k as usize];
                k += 1;
            }
            qmf_buf_real = qmf_buf_real.offset(14 as core::ffi::c_int as isize);
            qmf_buf_imag = qmf_buf_imag.offset(14 as core::ffi::c_int as isize);
            i += 1;
        }
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i < stop_border {
            k = 0 as core::ffi::c_int as WORD32;
            while k < x_over_band {
                let fresh22 = qmf_buf_real;
                qmf_buf_real = qmf_buf_real.offset(1);
                *fresh22 = (*ptr_sbr_dec)
                    .qmf_buf_real[(2 as WORD32 + i) as usize][k as usize];
                let fresh23 = qmf_buf_imag;
                qmf_buf_imag = qmf_buf_imag.offset(1);
                *fresh23 = (*ptr_sbr_dec)
                    .qmf_buf_imag[(2 as WORD32 + i) as usize][k as usize];
                k += 1;
            }
            while k < 64 as core::ffi::c_int {
                let fresh24 = qmf_buf_real;
                qmf_buf_real = qmf_buf_real.offset(1);
                *fresh24 = (*ptr_sbr_dec)
                    .sbr_qmf_out_real[(2 as WORD32 + i) as usize][k as usize];
                let fresh25 = qmf_buf_imag;
                qmf_buf_imag = qmf_buf_imag.offset(1);
                *fresh25 = (*ptr_sbr_dec)
                    .sbr_qmf_out_imag[(2 as WORD32 + i) as usize][k as usize];
                k += 1;
            }
            qmf_buf_real = qmf_buf_real.offset(14 as core::ffi::c_int as isize);
            qmf_buf_imag = qmf_buf_imag.offset(14 as core::ffi::c_int as isize);
            i += 1;
        }
        x_over_band = (*(*ptr_header_data).pstr_freq_band_data).sub_band_start as WORD32;
        while i < (*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as core::ffi::c_int {
            k = 0 as core::ffi::c_int as WORD32;
            while k < x_over_band {
                let fresh26 = qmf_buf_real;
                qmf_buf_real = qmf_buf_real.offset(1);
                *fresh26 = (*ptr_sbr_dec)
                    .qmf_buf_real[(2 as WORD32 + i) as usize][k as usize];
                let fresh27 = qmf_buf_imag;
                qmf_buf_imag = qmf_buf_imag.offset(1);
                *fresh27 = (*ptr_sbr_dec)
                    .qmf_buf_imag[(2 as WORD32 + i) as usize][k as usize];
                k += 1;
            }
            while k < 64 as core::ffi::c_int {
                let fresh28 = qmf_buf_real;
                qmf_buf_real = qmf_buf_real.offset(1);
                *fresh28 = (*ptr_sbr_dec)
                    .sbr_qmf_out_real[(2 as WORD32 + i) as usize][k as usize];
                let fresh29 = qmf_buf_imag;
                qmf_buf_imag = qmf_buf_imag.offset(1);
                *fresh29 = (*ptr_sbr_dec)
                    .sbr_qmf_out_imag[(2 as WORD32 + i) as usize][k as usize];
                k += 1;
            }
            qmf_buf_real = qmf_buf_real.offset(14 as core::ffi::c_int as isize);
            qmf_buf_imag = qmf_buf_imag.offset(14 as core::ffi::c_int as isize);
            i += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_esbr_synthesis_regrp(
    mut qmf_buf_real: *mut FLOAT32,
    mut qmf_buf_imag: *mut FLOAT32,
    mut ptr_sbr_dec: *mut ia_sbr_dec_struct,
    mut stereo_config_idx: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut num_anal_bands: WORD32 = (*ptr_sbr_dec).str_codec_qmf_bank.no_channels;
    let mut x_over_band: WORD32 = num_anal_bands;
    if stereo_config_idx > 0 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as core::ffi::c_int {
            k = 0 as core::ffi::c_int as WORD32;
            while k < 3 as core::ffi::c_int {
                let fresh0 = qmf_buf_real;
                qmf_buf_real = qmf_buf_real.offset(1);
                *fresh0 = (*ptr_sbr_dec)
                    .qmf_buf_real[(2 as core::ffi::c_int + i as core::ffi::c_int
                    + HYBRID_FILTER_DELAY) as usize][k as usize];
                let fresh1 = qmf_buf_imag;
                qmf_buf_imag = qmf_buf_imag.offset(1);
                *fresh1 = (*ptr_sbr_dec)
                    .qmf_buf_imag[(2 as core::ffi::c_int + i as core::ffi::c_int
                    + HYBRID_FILTER_DELAY) as usize][k as usize];
                k += 1;
            }
            while k < x_over_band {
                let fresh2 = qmf_buf_real;
                qmf_buf_real = qmf_buf_real.offset(1);
                *fresh2 = (*ptr_sbr_dec)
                    .qmf_buf_real[(2 as WORD32 + i) as usize][k as usize];
                let fresh3 = qmf_buf_imag;
                qmf_buf_imag = qmf_buf_imag.offset(1);
                *fresh3 = (*ptr_sbr_dec)
                    .qmf_buf_imag[(2 as WORD32 + i) as usize][k as usize];
                k += 1;
            }
            while k < 64 as core::ffi::c_int {
                let fresh4 = qmf_buf_real;
                qmf_buf_real = qmf_buf_real.offset(1);
                *fresh4 = 0 as core::ffi::c_int as FLOAT32;
                let fresh5 = qmf_buf_imag;
                qmf_buf_imag = qmf_buf_imag.offset(1);
                *fresh5 = 0 as core::ffi::c_int as FLOAT32;
                k += 1;
            }
            qmf_buf_real = qmf_buf_real.offset(14 as core::ffi::c_int as isize);
            qmf_buf_imag = qmf_buf_imag.offset(14 as core::ffi::c_int as isize);
            i += 1;
        }
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as core::ffi::c_int {
            k = 0 as core::ffi::c_int as WORD32;
            while k < x_over_band {
                let fresh6 = qmf_buf_real;
                qmf_buf_real = qmf_buf_real.offset(1);
                *fresh6 = (*ptr_sbr_dec)
                    .qmf_buf_real[(2 as WORD32 + i) as usize][k as usize];
                let fresh7 = qmf_buf_imag;
                qmf_buf_imag = qmf_buf_imag.offset(1);
                *fresh7 = (*ptr_sbr_dec)
                    .qmf_buf_imag[(2 as WORD32 + i) as usize][k as usize];
                k += 1;
            }
            while k < 64 as core::ffi::c_int {
                let fresh8 = qmf_buf_real;
                qmf_buf_real = qmf_buf_real.offset(1);
                *fresh8 = 0.0f32 as FLOAT32;
                let fresh9 = qmf_buf_imag;
                qmf_buf_imag = qmf_buf_imag.offset(1);
                *fresh9 = 0.0f32 as FLOAT32;
                k += 1;
            }
            qmf_buf_real = qmf_buf_real.offset(14 as core::ffi::c_int as isize);
            qmf_buf_imag = qmf_buf_imag.offset(14 as core::ffi::c_int as isize);
            i += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_esbr_synthesis_filt_block(
    mut ptr_sbr_dec: *mut ia_sbr_dec_struct,
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut ptr_frame_data: *mut ia_sbr_frame_info_data_struct,
    mut apply_processing: WORD32,
    mut qmf_buf_real: *mut *mut FLOAT32,
    mut qmf_buf_imag: *mut *mut FLOAT32,
    mut stereo_config_idx: WORD32,
    mut sbr_tables_ptr: *mut ia_sbr_tables_struct,
    mut mps_sbr_flag: WORD32,
    mut ch_fac: WORD32,
    mut ps_enable: WORD32,
    mut skip_re_grouping: WORD32,
    mut ptr_ps_dec: *mut ia_ps_dec_struct,
    mut drc_on: FLAG,
    mut drc_sbr_factors: *mut [WORD32; 64],
) -> VOID {
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut ptr_filt_states: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr_filt_states_1: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr_filt_states_2: *mut WORD32 = 0 as *mut WORD32;
    let mut filter_l: *mut WORD32 = 0 as *mut WORD32;
    let mut ploc_qmf_buf_real: *mut WORD32 = 0 as *mut WORD32;
    let mut ploc_qmf_buf_imag: *mut WORD32 = 0 as *mut WORD32;
    let mut out_scalefactor: WORD32 = 0;
    let mut sixty4: WORD32 = 0;
    let mut thrity2: WORD32 = 0;
    let mut no_synthesis_channels: WORD32 = 0;
    let mut ixheaacd_drc_offset: WORD32 = 0;
    let mut syn_buffer: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut local_qmf_buffer: *mut WORD32 = ((*ptr_sbr_dec).sbr_scratch_local)
        .as_mut_ptr();
    let mut time_out: *mut WORD32 = &mut *((*ptr_sbr_dec).sbr_scratch_local)
        .as_mut_ptr()
        .offset(128 as core::ffi::c_int as isize) as *mut WORD32;
    let mut time_sample_buf: *mut FLOAT32 = 0 as *mut FLOAT32;
    if ps_enable != 0 {
        time_sample_buf = (*ptr_ps_dec).time_sample_buf[0 as core::ffi::c_int as usize];
    } else {
        time_sample_buf = (*ptr_sbr_dec).time_sample_buf;
    }
    let mut qmf_bank: *mut ia_sbr_qmf_filter_bank_struct = &mut (*ptr_sbr_dec)
        .str_synthesis_qmf_bank;
    let mut qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct = (*sbr_tables_ptr)
        .qmf_dec_tables_ptr;
    if skip_re_grouping == 0 {
        if mps_sbr_flag == 0 {
            ixheaacd_esbr_synthesis_regrp(
                &mut *(*qmf_buf_real.offset(0 as core::ffi::c_int as isize))
                    .offset(0 as core::ffi::c_int as isize),
                &mut *(*qmf_buf_imag.offset(0 as core::ffi::c_int as isize))
                    .offset(0 as core::ffi::c_int as isize),
                ptr_sbr_dec,
                ptr_frame_data,
                ptr_header_data,
                stereo_config_idx,
                apply_processing,
            );
            if ps_enable != 0 {
                let mut factor: FLOAT32 = 1.0f32;
                i = (*ptr_ps_dec).num_sub_samples;
                while i < (*ptr_ps_dec).num_sub_samples + 6 as core::ffi::c_int {
                    k = 0 as core::ffi::c_int as WORD32;
                    while k < 5 as core::ffi::c_int {
                        if drc_on != 0 {
                            if (*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots
                                as core::ffi::c_int == 30 as core::ffi::c_int
                            {
                                factor = (*drc_sbr_factors
                                    .offset(
                                        (i as core::ffi::c_int + 30 as core::ffi::c_int
                                            - 25 as core::ffi::c_int) as isize,
                                    ))[k as usize] as FLOAT32 / Q25 as FLOAT32;
                            } else {
                                factor = (*drc_sbr_factors
                                    .offset(
                                        (i as core::ffi::c_int + 32 as core::ffi::c_int
                                            - 26 as core::ffi::c_int) as isize,
                                    ))[k as usize] as FLOAT32 / Q25 as FLOAT32;
                            }
                        }
                        *(*((*ptr_ps_dec)
                            .pp_qmf_buf_real[0 as core::ffi::c_int as usize])
                            .offset(i as isize))
                            .offset(k as isize) = factor
                            * (*ptr_sbr_dec)
                                .qmf_buf_real[(SBR_HF_ADJ_OFFSET + i) as usize][k as usize];
                        *(*((*ptr_ps_dec)
                            .pp_qmf_buf_imag[0 as core::ffi::c_int as usize])
                            .offset(i as isize))
                            .offset(k as isize) = factor
                            * (*ptr_sbr_dec)
                                .qmf_buf_imag[(SBR_HF_ADJ_OFFSET + i) as usize][k as usize];
                        k += 1;
                    }
                    i += 1;
                }
            }
            if ps_enable != 0 && apply_processing != 0 {
                let mut usb: WORD32 = (*(*ptr_header_data).pstr_freq_band_data)
                    .sub_band_end as WORD32;
                ixheaacd_esbr_apply_ps(
                    ptr_ps_dec,
                    (*ptr_ps_dec).pp_qmf_buf_real[0 as core::ffi::c_int as usize],
                    (*ptr_ps_dec).pp_qmf_buf_imag[0 as core::ffi::c_int as usize],
                    (*ptr_ps_dec).pp_qmf_buf_real[1 as core::ffi::c_int as usize],
                    (*ptr_ps_dec).pp_qmf_buf_imag[1 as core::ffi::c_int as usize],
                    usb,
                    (*sbr_tables_ptr).ps_tables_ptr,
                    (*ptr_header_data).num_time_slots as WORD32,
                );
            } else if ps_enable != 0 {
                i = 0 as core::ffi::c_int as WORD32;
                while i
                    < (*ptr_header_data).num_time_slots as core::ffi::c_int
                        * 2 as core::ffi::c_int
                {
                    k = 0 as core::ffi::c_int as WORD32;
                    while k < 64 as core::ffi::c_int {
                        *(*((*ptr_ps_dec)
                            .pp_qmf_buf_real[1 as core::ffi::c_int as usize])
                            .offset(i as isize))
                            .offset(k as isize) = *(*((*ptr_ps_dec)
                            .pp_qmf_buf_real[0 as core::ffi::c_int as usize])
                            .offset(i as isize))
                            .offset(k as isize);
                        *(*((*ptr_ps_dec)
                            .pp_qmf_buf_imag[1 as core::ffi::c_int as usize])
                            .offset(i as isize))
                            .offset(k as isize) = *(*((*ptr_ps_dec)
                            .pp_qmf_buf_imag[0 as core::ffi::c_int as usize])
                            .offset(i as isize))
                            .offset(k as isize);
                        k += 1;
                    }
                    i += 1;
                }
            }
        } else {
            ixheaacd_mps_esbr_synthesis_regrp(
                &mut *(*qmf_buf_real.offset(0 as core::ffi::c_int as isize))
                    .offset(0 as core::ffi::c_int as isize),
                &mut *(*qmf_buf_imag.offset(0 as core::ffi::c_int as isize))
                    .offset(0 as core::ffi::c_int as isize),
                ptr_sbr_dec,
                stereo_config_idx,
            );
        }
    } else if ps_enable != 0 {
        time_sample_buf = (*ptr_ps_dec).time_sample_buf[1 as core::ffi::c_int as usize];
    }
    if drc_on != 0 {
        let mut factor_0: FLOAT32 = 1.0f32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as core::ffi::c_int {
            k = 0 as core::ffi::c_int as WORD32;
            while k < 64 as core::ffi::c_int {
                if (*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as core::ffi::c_int
                    == 30 as core::ffi::c_int
                {
                    factor_0 = (*drc_sbr_factors
                        .offset(
                            (i as core::ffi::c_int + 30 as core::ffi::c_int
                                - 25 as core::ffi::c_int) as isize,
                        ))[k as usize] as FLOAT32 / Q25 as FLOAT32;
                } else {
                    factor_0 = (*drc_sbr_factors
                        .offset(
                            (i as core::ffi::c_int + 32 as core::ffi::c_int
                                - 26 as core::ffi::c_int) as isize,
                        ))[k as usize] as FLOAT32 / Q25 as FLOAT32;
                }
                *(*qmf_buf_real.offset(i as isize)).offset(k as isize) *= factor_0;
                *(*qmf_buf_imag.offset(i as isize)).offset(k as isize) *= factor_0;
                k += 1;
            }
            i += 1;
        }
    }
    if stereo_config_idx <= 0 as core::ffi::c_int {
        out_scalefactor = 5 as core::ffi::c_int as WORD32;
        no_synthesis_channels = (*qmf_bank).no_channels;
        sixty4 = NO_SYNTHESIS_CHANNELS as WORD32;
        thrity2 = (*qmf_bank).no_channels;
        if no_synthesis_channels == NO_SYNTHESIS_CHANNELS_DOWN_SAMPLED {
            (*qmf_bank).esbr_cos_twiddle = ((*qmf_dec_tables_ptr)
                .esbr_sin_cos_twiddle_l32)
                .as_mut_ptr();
            (*qmf_bank).esbr_alt_sin_twiddle = ((*qmf_dec_tables_ptr)
                .esbr_alt_sin_twiddle_l32)
                .as_mut_ptr();
        } else {
            (*qmf_bank).esbr_cos_twiddle = ((*qmf_dec_tables_ptr)
                .esbr_sin_cos_twiddle_l64)
                .as_mut_ptr();
            (*qmf_bank).esbr_alt_sin_twiddle = ((*qmf_dec_tables_ptr)
                .esbr_alt_sin_twiddle_l64)
                .as_mut_ptr();
        }
        (*qmf_bank).filter_pos_syn_32 = ((*qmf_bank).filter_pos_syn_32)
            .offset(
                ((*qmf_dec_tables_ptr).esbr_qmf_c)
                    .as_mut_ptr()
                    .offset_from((*qmf_bank).p_filter_32) as core::ffi::c_long as isize,
            );
        (*qmf_bank).p_filter_32 = ((*qmf_dec_tables_ptr).esbr_qmf_c).as_mut_ptr();
        ptr_filt_states = (*qmf_bank).filter_states_32;
        ptr_filt_states_1 = &mut *ptr_filt_states.offset(0 as core::ffi::c_int as isize)
            as *mut WORD32;
        ptr_filt_states_2 = ptr_filt_states_1.offset(no_synthesis_channels as isize);
        filter_l = (*qmf_bank).filter_pos_syn_32;
        ixheaacd_drc_offset = (*qmf_bank).ixheaacd_drc_offset as WORD32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as core::ffi::c_int {
            k = 0 as core::ffi::c_int as WORD32;
            while k < 64 as core::ffi::c_int {
                *local_qmf_buffer
                    .offset((k as core::ffi::c_int + 0 as core::ffi::c_int) as isize) = (*(*qmf_buf_real
                    .offset(i as isize))
                    .offset(k as isize) * 64 as core::ffi::c_int as FLOAT32) as WORD32;
                *local_qmf_buffer
                    .offset((k as core::ffi::c_int + 64 as core::ffi::c_int) as isize) = (*(*qmf_buf_imag
                    .offset(i as isize))
                    .offset(k as isize) * 64 as core::ffi::c_int as FLOAT32) as WORD32;
                k += 1;
            }
            ploc_qmf_buf_real = local_qmf_buffer;
            ploc_qmf_buf_imag = local_qmf_buffer.offset(64 as core::ffi::c_int as isize);
            ixheaacd_esbr_inv_modulation(
                ploc_qmf_buf_real,
                &mut (*ptr_sbr_dec).str_synthesis_qmf_bank,
                (*sbr_tables_ptr).qmf_dec_tables_ptr,
                no_synthesis_channels,
            );
            ixheaacd_shiftrountine_with_rnd_hq(
                ploc_qmf_buf_real,
                ploc_qmf_buf_imag,
                &mut *ptr_filt_states.offset(ixheaacd_drc_offset as isize),
                no_synthesis_channels,
                out_scalefactor + 1 as WORD32,
            );
            if no_synthesis_channels == NO_SYNTHESIS_CHANNELS_DOWN_SAMPLED {
                ixheaacd_esbr_qmfsyn32_winadd(
                    ptr_filt_states_1,
                    ptr_filt_states_2,
                    filter_l,
                    &mut *time_out.offset(0 as core::ffi::c_int as isize),
                    ch_fac,
                );
                if mps_sbr_flag == 0 {
                    syn_buffer = time_sample_buf
                        .offset(
                            (i as core::ffi::c_int * 32 as core::ffi::c_int) as isize,
                        );
                } else {
                    syn_buffer = ((*ptr_sbr_dec).time_sample_buf)
                        .offset(
                            (i as core::ffi::c_int * 32 as core::ffi::c_int) as isize,
                        );
                }
                k = 0 as core::ffi::c_int as WORD32;
                while k < 32 as core::ffi::c_int {
                    *syn_buffer.offset(k as isize) = *time_out.offset(k as isize)
                        as FLOAT32
                        / ((1 as core::ffi::c_int) << 16 as core::ffi::c_int) as FLOAT32;
                    k += 1;
                }
                ptr_filt_states_1 = ptr_filt_states_1.offset(thrity2 as isize);
                ptr_filt_states_2 = ptr_filt_states_2.offset(-(thrity2 as isize));
                thrity2 = -thrity2;
                ixheaacd_drc_offset -= 64 as core::ffi::c_int;
                if ixheaacd_drc_offset < 0 as core::ffi::c_int {
                    ixheaacd_drc_offset += 640 as core::ffi::c_int;
                }
            } else {
                ixheaacd_esbr_qmfsyn64_winadd(
                    ptr_filt_states_1,
                    ptr_filt_states_2,
                    filter_l,
                    &mut *time_out.offset(0 as core::ffi::c_int as isize),
                    ch_fac,
                );
                if mps_sbr_flag == 0 {
                    syn_buffer = time_sample_buf
                        .offset(
                            (i as core::ffi::c_int * 64 as core::ffi::c_int) as isize,
                        );
                } else {
                    syn_buffer = ((*ptr_sbr_dec).time_sample_buf)
                        .offset(
                            (i as core::ffi::c_int * 64 as core::ffi::c_int) as isize,
                        );
                }
                k = 0 as core::ffi::c_int as WORD32;
                while k < 64 as core::ffi::c_int {
                    *syn_buffer.offset(k as isize) = *time_out.offset(k as isize)
                        as FLOAT32
                        / ((1 as core::ffi::c_int) << 16 as core::ffi::c_int) as FLOAT32;
                    k += 1;
                }
                ptr_filt_states_1 = ptr_filt_states_1.offset(sixty4 as isize);
                ptr_filt_states_2 = ptr_filt_states_2.offset(-(sixty4 as isize));
                sixty4 = -sixty4;
                ixheaacd_drc_offset -= 128 as core::ffi::c_int;
                if ixheaacd_drc_offset < 0 as core::ffi::c_int {
                    ixheaacd_drc_offset += 1280 as core::ffi::c_int;
                }
            }
            filter_l = filter_l.offset(64 as core::ffi::c_int as isize);
            if filter_l
                == ((*qmf_bank).p_filter_32).offset(640 as core::ffi::c_int as isize)
                    as *mut WORD32
            {
                filter_l = (*qmf_bank).p_filter_32 as *mut WORD32;
            }
            i += 1;
        }
        (*qmf_bank).filter_pos_syn_32 = filter_l;
        (*qmf_bank).ixheaacd_drc_offset = ixheaacd_drc_offset as WORD16;
    }
    if mps_sbr_flag == 0 {
        (*ptr_frame_data).reset_flag = 0 as core::ffi::c_int as FLAG;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_sbr_dec(
    mut ptr_sbr_dec: *mut ia_sbr_dec_struct,
    mut ptr_time_data: *mut WORD16,
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut ptr_frame_data: *mut ia_sbr_frame_info_data_struct,
    mut ptr_frame_data_prev: *mut ia_sbr_prev_frame_data_struct,
    mut ptr_ps_dec: *mut ia_ps_dec_struct,
    mut ptr_qmf_synth_bank_r: *mut ia_sbr_qmf_filter_bank_struct,
    mut ptr_sbr_sf_r: *mut ia_sbr_scale_fact_struct,
    mut apply_processing: FLAG,
    mut low_pow_flag: FLAG,
    mut ptr_work_buf_core: *mut WORD32,
    mut sbr_tables_ptr: *mut ia_sbr_tables_struct,
    mut pstr_common_tables: *mut ixheaacd_misc_tables,
    mut ch_fac: WORD,
    mut ptr_pvc_data: *mut ia_pvc_data_struct,
    mut drc_on: FLAG,
    mut drc_sbr_factors: *mut [WORD32; 64],
    mut audio_object_type: WORD32,
    mut ldmps_present: WORD32,
    mut self_0: *mut core::ffi::c_void,
    mut heaac_mps_present: WORD32,
    mut ec_flag: WORD32,
) -> WORD32 {
    let mut i: WORD = 0;
    let mut j: WORD = 0;
    let mut k: WORD = 0;
    let mut slot: WORD = 0;
    let mut reserve: WORD = 0;
    let mut save_lb_scale: WORD = 0;
    let mut op_delay: WORD = 0;
    let mut err_code: IA_ERRORCODE = IA_NO_ERROR;
    let mut p_arr_qmf_buf_real: [*mut WORD32; 38] = [
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
    ];
    let mut p_arr_qmf_buf_imag: [*mut WORD32; 38] = [
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
        0 as *mut WORD32,
    ];
    let mut ptr: *mut WORD32 = 0 as *mut WORD32;
    let mut hbe_flag: WORD = (*ptr_header_data).hbe_flag as WORD;
    let mut pp_qmf_buf_real: *mut *mut FLOAT32 = 0 as *mut *mut FLOAT32;
    let mut pp_qmf_buf_imag: *mut *mut FLOAT32 = 0 as *mut *mut FLOAT32;
    let mut pvc_dec_out_buf: [FLOAT32; 1024] = [0.; 1024];
    let mut upsample_ratio_idx: WORD = (*ptr_header_data).sbr_ratio_idx as WORD;
    let mut no_bins: WORD = 0;
    let mut mps_sbr_flag: WORD = (*ptr_frame_data).mps_sbr_flag as WORD;
    let mut stereo_config_idx: WORD = (*ptr_frame_data).stereo_config_idx as WORD;
    let mut sbr_mode: WORD = (*ptr_frame_data).sbr_mode as WORD;
    let mut usac_flag: WORD = (*ptr_header_data).usac_flag as WORD;
    let mut add_slot: WORD = 0 as WORD;
    let mut pvc_qmf_enrg_arr: *mut FLOAT32 = ((*ptr_sbr_dec).pvc_qmf_enrg_arr)
        .as_mut_ptr();
    let mut dft_hbe_flag: WORD32 = (*ptr_header_data).esbr_hq as WORD32;
    let mut esbr_hbe_delay_offsets: WORD32 = 0;
    if (*ptr_header_data).num_time_slots as core::ffi::c_int == 15 as core::ffi::c_int {
        esbr_hbe_delay_offsets = ESBR_HBE_DELAY_OFFSET_960 as WORD32;
    } else {
        esbr_hbe_delay_offsets = ESBR_HBE_DELAY_OFFSET as WORD32;
    }
    memset(
        pvc_dec_out_buf.as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (1024 as size_t).wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
    );
    memset(
        pvc_qmf_enrg_arr as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (512 as size_t).wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
    );
    if audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int {
        op_delay = 0 as core::ffi::c_int as WORD;
    } else {
        op_delay = 6 as core::ffi::c_int as WORD;
    }
    if ldmps_present == 1 as core::ffi::c_int {
        add_slot = SBR_HF_ADJ_OFFSET as WORD;
    }
    if !(audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int
        || audio_object_type == AOT_ER_AAC_LD as core::ffi::c_int)
        && (*ptr_header_data).enh_sbr != 0
    {
        ch_fac = 1 as core::ffi::c_int as WORD;
        pp_qmf_buf_real = (*ptr_sbr_dec).pp_qmf_buf_real;
        pp_qmf_buf_imag = (*ptr_sbr_dec).pp_qmf_buf_imag;
        if upsample_ratio_idx == SBR_UPSAMPLE_IDX_4_1 {
            op_delay = (2 as core::ffi::c_int * 6 as core::ffi::c_int) as WORD;
        }
    }
    no_bins = ((*ptr_header_data).num_time_slots as core::ffi::c_int
        * (*ptr_header_data).time_step as core::ffi::c_int) as WORD;
    if audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int
        || audio_object_type == AOT_ER_AAC_LD as core::ffi::c_int
        || (*ptr_header_data).enh_sbr == 0
    {
        let mut num: WORD32 = op_delay as WORD32;
        let mut ptr_pers_qmf_real: *mut WORD32 = (*ptr_sbr_dec).ptr_sbr_overlap_buf;
        let mut p_scr_qmf_real: *mut WORD32 = ptr_work_buf_core
            .offset(
                ((2 as core::ffi::c_int)
                    << 6 as core::ffi::c_int + (low_pow_flag == 0) as core::ffi::c_int)
                    as isize,
            );
        if (*ptr_header_data).num_time_slots as core::ffi::c_int
            != 15 as core::ffi::c_int
        {
            if no_bins < LPC_ORDER || no_bins + op_delay > MAX_ENV_COLS {
                if ec_flag != 0 {
                    no_bins = LPC_ORDER as WORD;
                } else {
                    return -(1 as WORD32)
                }
            }
        } else if no_bins < LPC_ORDER || no_bins + op_delay > MAX_ENV_COLS_960 {
            if ec_flag != 0 {
                no_bins = LPC_ORDER as WORD;
            } else {
                return -(1 as WORD32)
            }
        }
        if low_pow_flag == 0 {
            num = num << 1 as core::ffi::c_int;
        }
        if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int {
            memcpy(
                p_scr_qmf_real as *mut core::ffi::c_void,
                ptr_pers_qmf_real as *const core::ffi::c_void,
                (::core::mem::size_of::<WORD32>() as size_t)
                    .wrapping_mul(NO_SYNTHESIS_CHANNELS as size_t)
                    .wrapping_mul(num as size_t),
            );
        }
        ptr = p_scr_qmf_real;
        slot = 0 as core::ffi::c_int as WORD;
        while slot < op_delay + no_bins + add_slot {
            p_arr_qmf_buf_real[slot as usize] = ptr;
            ptr = ptr.offset(NO_SYNTHESIS_CHANNELS as isize);
            if low_pow_flag == 0 {
                p_arr_qmf_buf_imag[slot as usize] = ptr;
                ptr = ptr.offset(NO_SYNTHESIS_CHANNELS as isize);
            }
            slot += 1;
        }
        (*ptr_sbr_dec).str_sbr_scale_fact.lb_scale = 0 as WORD16;
        if apply_processing != 0 {
            ixheaacd_rescale_x_overlap(
                ptr_sbr_dec,
                ptr_header_data,
                ptr_frame_data,
                ptr_frame_data_prev,
                p_arr_qmf_buf_real.as_mut_ptr(),
                p_arr_qmf_buf_imag.as_mut_ptr(),
                low_pow_flag,
            );
        }
    }
    if audio_object_type == AOT_AAC_LC as core::ffi::c_int
        && heaac_mps_present == 1 as core::ffi::c_int && (*ptr_header_data).enh_sbr != 0
    {
        let mut num_anal_bands: WORD32 = (*ptr_sbr_dec).str_codec_qmf_bank.no_channels;
        let mut frame_move: WORD32 = 9 as WORD32 * num_anal_bands;
        let mut core_frame_size: WORD32 = (*ptr_header_data).core_frame_size as WORD32;
        memcpy(
            &mut *((*ptr_sbr_dec).core_sample_buf)
                .as_mut_ptr()
                .offset(core_frame_size as isize) as *mut FLOAT32
                as *mut core::ffi::c_void,
            &mut *((*ptr_sbr_dec).time_sample_buf)
                .offset((core_frame_size - frame_move) as isize) as *mut FLOAT32
                as *const core::ffi::c_void,
            (frame_move as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        memmove(
            &mut *((*ptr_sbr_dec).time_sample_buf).offset(frame_move as isize)
                as *mut FLOAT32 as *mut core::ffi::c_void,
            &mut *((*ptr_sbr_dec).time_sample_buf).offset(0 as core::ffi::c_int as isize)
                as *mut FLOAT32 as *const core::ffi::c_void,
            (core_frame_size - frame_move) as size_t,
        );
        memcpy(
            &mut *((*ptr_sbr_dec).time_sample_buf).offset(0 as core::ffi::c_int as isize)
                as *mut FLOAT32 as *mut core::ffi::c_void,
            &mut *((*ptr_sbr_dec).core_sample_buf)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                as *const core::ffi::c_void,
            (frame_move as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        memcpy(
            &mut *((*ptr_sbr_dec).core_sample_buf)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                as *mut core::ffi::c_void,
            &mut *((*ptr_sbr_dec).core_sample_buf)
                .as_mut_ptr()
                .offset(core_frame_size as isize) as *mut FLOAT32
                as *const core::ffi::c_void,
            (frame_move as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
    }
    if audio_object_type == AOT_AAC_LC as core::ffi::c_int
        && heaac_mps_present == 1 as core::ffi::c_int && (*ptr_header_data).enh_sbr == 0
    {
        let mut num_anal_bands_0: WORD32 = (*ptr_sbr_dec).str_codec_qmf_bank.no_channels;
        let mut frame_move_0: WORD32 = 9 as WORD32 * num_anal_bands_0;
        let mut core_frame_size_0: WORD32 = (*ptr_header_data).core_frame_size as WORD32;
        memcpy(
            &mut *((*ptr_sbr_dec).core_sample_buf_sbr)
                .as_mut_ptr()
                .offset(core_frame_size_0 as isize) as *mut WORD16
                as *mut core::ffi::c_void,
            &mut *ptr_time_data.offset((core_frame_size_0 - frame_move_0) as isize)
                as *mut WORD16 as *const core::ffi::c_void,
            (frame_move_0 as size_t)
                .wrapping_mul(::core::mem::size_of::<WORD16>() as size_t),
        );
        memmove(
            &mut *ptr_time_data.offset(frame_move_0 as isize) as *mut WORD16
                as *mut core::ffi::c_void,
            &mut *ptr_time_data.offset(0 as core::ffi::c_int as isize) as *mut WORD16
                as *const core::ffi::c_void,
            (core_frame_size_0 - frame_move_0) as size_t,
        );
        memcpy(
            &mut *ptr_time_data.offset(0 as core::ffi::c_int as isize) as *mut WORD16
                as *mut core::ffi::c_void,
            &mut *((*ptr_sbr_dec).core_sample_buf_sbr)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut WORD16
                as *const core::ffi::c_void,
            (frame_move_0 as size_t)
                .wrapping_mul(::core::mem::size_of::<WORD16>() as size_t),
        );
        memcpy(
            &mut *((*ptr_sbr_dec).core_sample_buf_sbr)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut WORD16
                as *mut core::ffi::c_void,
            &mut *((*ptr_sbr_dec).core_sample_buf_sbr)
                .as_mut_ptr()
                .offset(core_frame_size_0 as isize) as *mut WORD16
                as *const core::ffi::c_void,
            (frame_move_0 as size_t)
                .wrapping_mul(::core::mem::size_of::<WORD16>() as size_t),
        );
    }
    if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
        && audio_object_type != AOT_ER_AAC_LD as core::ffi::c_int
        && (*ptr_header_data).enh_sbr != 0
    {
        let mut codec_x_delay: WORD32 = 0 as WORD32;
        if hbe_flag != 0 || usac_flag == 0 {
            codec_x_delay = esbr_hbe_delay_offsets;
        }
        if upsample_ratio_idx == SBR_UPSAMPLE_IDX_4_1 {
            codec_x_delay = 2 as WORD32 * codec_x_delay;
        }
        if (*ptr_header_data).num_time_slots as core::ffi::c_int
            != 15 as core::ffi::c_int
        {
            if mps_sbr_flag != 0 {
                op_delay = MPS_SBR_DELAY as WORD;
            }
        } else if mps_sbr_flag != 0 {
            op_delay = MPS_SBR_DELAY_960 as WORD;
        }
        memmove(
            &mut *(*((*ptr_sbr_dec).qmf_buf_real)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                as *mut core::ffi::c_void,
            &mut *(*((*ptr_sbr_dec).qmf_buf_real)
                .as_mut_ptr()
                .offset((*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                as *const core::ffi::c_void,
            ((op_delay as WORD32 + SBR_HF_ADJ_OFFSET + codec_x_delay) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t)
                .wrapping_mul(64 as size_t),
        );
        memmove(
            &mut *(*((*ptr_sbr_dec).qmf_buf_imag)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                as *mut core::ffi::c_void,
            &mut *(*((*ptr_sbr_dec).qmf_buf_imag)
                .as_mut_ptr()
                .offset((*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                as *const core::ffi::c_void,
            ((op_delay as WORD32 + SBR_HF_ADJ_OFFSET + codec_x_delay) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t)
                .wrapping_mul(64 as size_t),
        );
        memmove(
            &mut *(*((*ptr_sbr_dec).sbr_qmf_out_real)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                as *mut core::ffi::c_void,
            &mut *(*((*ptr_sbr_dec).sbr_qmf_out_real)
                .as_mut_ptr()
                .offset((*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                as *const core::ffi::c_void,
            ((op_delay as core::ffi::c_int + SBR_HF_ADJ_OFFSET) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t)
                .wrapping_mul(64 as size_t),
        );
        memmove(
            &mut *(*((*ptr_sbr_dec).sbr_qmf_out_imag)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                as *mut core::ffi::c_void,
            &mut *(*((*ptr_sbr_dec).sbr_qmf_out_imag)
                .as_mut_ptr()
                .offset((*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                as *const core::ffi::c_void,
            ((op_delay as core::ffi::c_int + SBR_HF_ADJ_OFFSET) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t)
                .wrapping_mul(64 as size_t),
        );
        if hbe_flag != 0 {
            memmove(
                &mut *(*((*ptr_sbr_dec).ph_vocod_qmf_real)
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize))
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                    as *mut core::ffi::c_void,
                &mut *(*((*ptr_sbr_dec).ph_vocod_qmf_real)
                    .as_mut_ptr()
                    .offset((*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as isize))
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                    as *const core::ffi::c_void,
                (64 as size_t)
                    .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t)
                    .wrapping_mul(
                        (op_delay as core::ffi::c_int + SBR_HF_ADJ_OFFSET) as size_t,
                    ),
            );
            memmove(
                ((*ptr_sbr_dec).ph_vocod_qmf_imag).as_mut_ptr()
                    as *mut core::ffi::c_void,
                ((*ptr_sbr_dec).ph_vocod_qmf_imag)
                    .as_mut_ptr()
                    .offset(
                        (*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots
                            as core::ffi::c_int as isize,
                    ) as *const core::ffi::c_void,
                (64 as size_t)
                    .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t)
                    .wrapping_mul(
                        (op_delay as core::ffi::c_int + SBR_HF_ADJ_OFFSET) as size_t,
                    ),
            );
            if usac_flag == 0 {
                let mut qmf_sb_prev: WORD32 = (*(*ptr_header_data).pstr_freq_band_data)
                    .qmf_sb_prev as WORD32;
                i = SBR_HF_ADJ_OFFSET as WORD;
                while i < op_delay as core::ffi::c_int + SBR_HF_ADJ_OFFSET {
                    memset(
                        &mut *(*((*ptr_sbr_dec).qmf_buf_real)
                            .as_mut_ptr()
                            .offset(i as isize))
                            .as_mut_ptr()
                            .offset(qmf_sb_prev as isize) as *mut FLOAT32
                            as *mut core::ffi::c_void,
                        0 as core::ffi::c_int,
                        (64 as WORD32 - qmf_sb_prev) as size_t,
                    );
                    memset(
                        &mut *(*((*ptr_sbr_dec).qmf_buf_imag)
                            .as_mut_ptr()
                            .offset(i as isize))
                            .as_mut_ptr()
                            .offset(qmf_sb_prev as isize) as *mut FLOAT32
                            as *mut core::ffi::c_void,
                        0 as core::ffi::c_int,
                        (64 as WORD32 - qmf_sb_prev) as size_t,
                    );
                    i += 1;
                }
            }
        }
        ixheaacd_esbr_analysis_filt_block(
            ptr_sbr_dec,
            sbr_tables_ptr,
            op_delay as WORD32 + codec_x_delay + SBR_HF_ADJ_OFFSET,
        );
        if hbe_flag != 0 && apply_processing != 0 {
            if dft_hbe_flag == 1 as core::ffi::c_int {
                let mut err_code_0: WORD32 = 0 as WORD32;
                (*(*ptr_sbr_dec).p_hbe_txposer).oversampling_flag = (*ptr_frame_data)
                    .over_sampling_flag;
                err_code_0 = ixheaacd_dft_hbe_apply(
                    (*ptr_sbr_dec).p_hbe_txposer,
                    ((*ptr_sbr_dec).qmf_buf_real)
                        .as_mut_ptr()
                        .offset(
                            (op_delay as core::ffi::c_int + SBR_HF_ADJ_OFFSET) as isize,
                        )
                        .offset(esbr_hbe_delay_offsets as isize),
                    ((*ptr_sbr_dec).qmf_buf_imag)
                        .as_mut_ptr()
                        .offset(
                            (op_delay as core::ffi::c_int + SBR_HF_ADJ_OFFSET) as isize,
                        )
                        .offset(esbr_hbe_delay_offsets as isize),
                    (*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as WORD32,
                    ((*ptr_sbr_dec).ph_vocod_qmf_real)
                        .as_mut_ptr()
                        .offset(
                            (op_delay as core::ffi::c_int + SBR_HF_ADJ_OFFSET) as isize,
                        ),
                    ((*ptr_sbr_dec).ph_vocod_qmf_imag)
                        .as_mut_ptr()
                        .offset(
                            (op_delay as core::ffi::c_int + SBR_HF_ADJ_OFFSET) as isize,
                        ),
                    (*ptr_frame_data).pitch_in_bins,
                    ptr_work_buf_core as *mut FLOAT32,
                );
                if err_code_0 != 0 {
                    return err_code_0;
                }
            } else {
                let mut err_code_1: WORD32 = ixheaacd_qmf_hbe_apply(
                    (*ptr_sbr_dec).p_hbe_txposer,
                    ((*ptr_sbr_dec).qmf_buf_real)
                        .as_mut_ptr()
                        .offset(
                            (op_delay as core::ffi::c_int + SBR_HF_ADJ_OFFSET) as isize,
                        )
                        .offset(esbr_hbe_delay_offsets as isize),
                    ((*ptr_sbr_dec).qmf_buf_imag)
                        .as_mut_ptr()
                        .offset(
                            (op_delay as core::ffi::c_int + SBR_HF_ADJ_OFFSET) as isize,
                        )
                        .offset(esbr_hbe_delay_offsets as isize),
                    (*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as WORD32,
                    ((*ptr_sbr_dec).ph_vocod_qmf_real)
                        .as_mut_ptr()
                        .offset(
                            (op_delay as core::ffi::c_int + SBR_HF_ADJ_OFFSET) as isize,
                        ),
                    ((*ptr_sbr_dec).ph_vocod_qmf_imag)
                        .as_mut_ptr()
                        .offset(
                            (op_delay as core::ffi::c_int + SBR_HF_ADJ_OFFSET) as isize,
                        ),
                    (*ptr_frame_data).pitch_in_bins,
                    ptr_header_data,
                );
                if err_code_1 != 0 {
                    return err_code_1;
                }
                if upsample_ratio_idx == SBR_UPSAMPLE_IDX_4_1 {
                    ixheaacd_hbe_repl_spec(
                        &mut *((*(*ptr_sbr_dec).p_hbe_txposer).x_over_qmf)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize),
                        ((*ptr_sbr_dec).ph_vocod_qmf_real)
                            .as_mut_ptr()
                            .offset(
                                (op_delay as core::ffi::c_int + SBR_HF_ADJ_OFFSET) as isize,
                            ),
                        ((*ptr_sbr_dec).ph_vocod_qmf_imag)
                            .as_mut_ptr()
                            .offset(
                                (op_delay as core::ffi::c_int + SBR_HF_ADJ_OFFSET) as isize,
                            ),
                        (*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as WORD32,
                        (*(*ptr_sbr_dec).p_hbe_txposer).max_stretch,
                    );
                }
            }
        }
        if mps_sbr_flag == 0 && apply_processing != 0 {
            err_code = ixheaacd_generate_hf(
                ((*ptr_sbr_dec).qmf_buf_real)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as isize),
                ((*ptr_sbr_dec).qmf_buf_imag)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as isize),
                ((*ptr_sbr_dec).ph_vocod_qmf_real)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as isize),
                ((*ptr_sbr_dec).ph_vocod_qmf_imag)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as isize),
                ((*ptr_sbr_dec).sbr_qmf_out_real)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as isize),
                ((*ptr_sbr_dec).sbr_qmf_out_imag)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as isize),
                ptr_frame_data,
                ptr_header_data,
                ldmps_present,
                (*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as WORD32,
                ec_flag,
            ) as IA_ERRORCODE;
            if err_code != 0 {
                return err_code as WORD32;
            }
            (*ptr_pvc_data).pvc_rate = (*ptr_header_data).upsamp_fac as UWORD8;
            if sbr_mode == PVC_SBR as core::ffi::c_int {
                ixheaacd_qmf_enrg_calc(
                    ptr_sbr_dec,
                    upsample_ratio_idx as WORD32,
                    low_pow_flag as WORD32,
                );
                if ec_flag != 0 {
                    (*ptr_pvc_data).pvc_mode = 1 as UWORD8;
                }
                err_code = ixheaacd_pvc_process(
                    ptr_pvc_data,
                    (*(*ptr_header_data).pstr_freq_band_data).sub_band_start,
                    (*ptr_frame_data)
                        .str_pvc_frame_info
                        .border_vec[0 as core::ffi::c_int as usize] as WORD32,
                    &mut *pvc_qmf_enrg_arr.offset(0 as core::ffi::c_int as isize),
                    &mut *pvc_dec_out_buf
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize),
                ) as IA_ERRORCODE;
                if err_code != 0 {
                    return err_code as WORD32;
                }
                (*ptr_pvc_data).prev_pvc_flg = 1 as UWORD8;
            } else {
                memset(
                    pvc_dec_out_buf.as_mut_ptr() as *mut core::ffi::c_void,
                    0 as core::ffi::c_int,
                    (1024 as size_t)
                        .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                );
                (*ptr_pvc_data).prev_pvc_flg = 0 as UWORD8;
            }
            (*ptr_pvc_data).prev_first_bnd_idx = (*(*ptr_header_data)
                .pstr_freq_band_data)
                .sub_band_start;
            (*ptr_pvc_data).prev_pvc_rate = (*ptr_pvc_data).pvc_rate;
            (*ptr_frame_data).pstr_sbr_header = ptr_header_data;
            err_code = ixheaacd_sbr_env_calc(
                ptr_frame_data,
                ((*ptr_sbr_dec).sbr_qmf_out_real)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as isize),
                ((*ptr_sbr_dec).sbr_qmf_out_imag)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as isize),
                ((*ptr_sbr_dec).qmf_buf_real)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as isize),
                ((*ptr_sbr_dec).qmf_buf_imag)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as isize),
                if (*ptr_header_data).hbe_flag == 0 as core::ffi::c_int {
                    0 as *mut WORD32
                } else {
                    ((*(*ptr_sbr_dec).p_hbe_txposer).x_over_qmf).as_mut_ptr()
                },
                ((*ptr_sbr_dec).scratch_buff).as_mut_ptr(),
                pvc_dec_out_buf.as_mut_ptr(),
                ldmps_present,
                ec_flag,
            ) as IA_ERRORCODE;
            if err_code != 0 {
                return err_code as WORD32;
            }
        } else {
            i = 0 as core::ffi::c_int as WORD;
            while i < 64 as core::ffi::c_int {
                memset(
                    ((*ptr_sbr_dec).sbr_qmf_out_real[i as usize]).as_mut_ptr()
                        as *mut core::ffi::c_void,
                    0 as core::ffi::c_int,
                    (64 as size_t)
                        .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                );
                memset(
                    ((*ptr_sbr_dec).sbr_qmf_out_imag[i as usize]).as_mut_ptr()
                        as *mut core::ffi::c_void,
                    0 as core::ffi::c_int,
                    (64 as size_t)
                        .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                );
                i += 1;
            }
        }
        if mps_sbr_flag == 0 {
            (*ptr_sbr_dec).band_count = (*(*ptr_header_data).pstr_freq_band_data)
                .sub_band_end as WORD32;
        } else {
            (*ptr_sbr_dec).band_count = (*ptr_sbr_dec).str_codec_qmf_bank.no_channels;
        }
        ixheaacd_esbr_synthesis_filt_block(
            ptr_sbr_dec,
            ptr_header_data,
            ptr_frame_data,
            apply_processing as WORD32,
            pp_qmf_buf_real,
            pp_qmf_buf_imag,
            stereo_config_idx as WORD32,
            sbr_tables_ptr,
            mps_sbr_flag as WORD32,
            ch_fac as WORD32,
            ((*ptr_header_data).channel_mode == PS_STEREO
                || (*ptr_header_data).enh_sbr_ps != 0) as core::ffi::c_int,
            0 as WORD32,
            ptr_ps_dec,
            drc_on,
            drc_sbr_factors,
        );
        if (*ptr_header_data).enh_sbr_ps != 0
            || (*ptr_header_data).channel_mode == PS_STEREO
        {
            pp_qmf_buf_real = (*ptr_ps_dec)
                .pp_qmf_buf_real[1 as core::ffi::c_int as usize];
            pp_qmf_buf_imag = (*ptr_ps_dec)
                .pp_qmf_buf_imag[1 as core::ffi::c_int as usize];
            ixheaacd_esbr_synthesis_filt_block(
                &mut (**((*(self_0 as ia_handle_sbr_dec_inst_struct)).pstr_sbr_channel)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as isize))
                    .str_sbr_dec as *mut ia_sbr_dec_struct,
                &mut *((*(self_0 as ia_handle_sbr_dec_inst_struct)).pstr_sbr_header)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as isize)
                    as *mut *mut ia_sbr_header_data_struct
                    as *mut ia_sbr_header_data_struct,
                &mut *((*(self_0 as ia_handle_sbr_dec_inst_struct)).frame_buffer)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as isize)
                    as *mut *mut core::ffi::c_void as *mut ia_sbr_frame_info_data_struct,
                apply_processing as WORD32,
                pp_qmf_buf_real,
                pp_qmf_buf_imag,
                stereo_config_idx as WORD32,
                sbr_tables_ptr,
                mps_sbr_flag as WORD32,
                ch_fac as WORD32,
                ((*ptr_header_data).channel_mode == PS_STEREO
                    || (*ptr_header_data).enh_sbr_ps != 0) as core::ffi::c_int,
                1 as WORD32,
                ptr_ps_dec,
                drc_on,
                drc_sbr_factors,
            );
        }
        if apply_processing != 0 && ec_flag != 0 {
            let mut border_vec: *mut WORD16 = ((*ptr_frame_data)
                .str_frame_info_details
                .border_vec)
                .as_mut_ptr();
            (*ptr_frame_data_prev).end_position = *border_vec
                .offset((*ptr_frame_data).str_frame_info_details.num_env as isize);
        }
        (*ptr_frame_data).prev_sbr_mode = sbr_mode as FLAG;
        return 0 as WORD32;
    }
    if ldmps_present != 0 {
        if (*ptr_sbr_dec).str_codec_qmf_bank.no_channels > 32 as core::ffi::c_int {
            if ec_flag != 0 {
                (*ptr_sbr_dec).str_codec_qmf_bank.no_channels = 32 as core::ffi::c_int
                    as WORD32;
            } else {
                return IA_FATAL_ERROR as WORD32
            }
        }
        ixheaacd_cplx_anal_qmffilt_32(
            ptr_time_data as *mut WORD32,
            &mut (*ptr_sbr_dec).str_sbr_scale_fact,
            &mut *p_arr_qmf_buf_real.as_mut_ptr().offset(op_delay as isize),
            &mut *p_arr_qmf_buf_imag.as_mut_ptr().offset(op_delay as isize),
            &mut (*ptr_sbr_dec).str_codec_qmf_bank,
            (*sbr_tables_ptr).qmf_dec_tables_ptr,
            ch_fac,
            1 as WORD32,
        );
    } else {
        ixheaacd_cplx_anal_qmffilt(
            ptr_time_data,
            &mut (*ptr_sbr_dec).str_sbr_scale_fact,
            &mut *p_arr_qmf_buf_real.as_mut_ptr().offset(op_delay as isize),
            &mut *p_arr_qmf_buf_imag.as_mut_ptr().offset(op_delay as isize),
            &mut (*ptr_sbr_dec).str_codec_qmf_bank,
            (*sbr_tables_ptr).qmf_dec_tables_ptr,
            ch_fac,
            low_pow_flag as WORD32,
            audio_object_type as WORD,
        );
    }
    if ldmps_present == 1 as core::ffi::c_int {
        j = SBR_HF_ADJ_OFFSET as WORD;
        while j
            < (*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as core::ffi::c_int
                + SBR_HF_ADJ_OFFSET
        {
            k = 0 as core::ffi::c_int as WORD;
            while k < 64 as core::ffi::c_int {
                let mut scale: WORD32 = 7 as WORD32;
                (*ptr_sbr_dec).mps_qmf_buf_real[j as usize][k as usize] = 0.0f32
                    as FLOAT32;
                (*ptr_sbr_dec).mps_qmf_buf_imag[j as usize][k as usize] = 0.0f32
                    as FLOAT32;
                if k < (*ptr_sbr_dec).str_codec_qmf_bank.usb as core::ffi::c_int {
                    (*ptr_sbr_dec).mps_qmf_buf_real[j as usize][k as usize]
                        += *(p_arr_qmf_buf_real[j as usize]).offset(k as isize)
                            as FLOAT32 / ((1 as core::ffi::c_int) << scale) as FLOAT32;
                    (*ptr_sbr_dec).mps_qmf_buf_imag[j as usize][k as usize]
                        += *(p_arr_qmf_buf_imag[j as usize]).offset(k as isize)
                            as FLOAT32 / ((1 as core::ffi::c_int) << scale) as FLOAT32;
                }
                k += 1;
            }
            j += 1;
        }
    }
    let mut shift1: WORD = 0;
    let mut shift2: WORD = 0;
    let mut min_shift: WORD = 0;
    let mut shift_over: WORD = 0;
    let mut reserve_ov1: WORD = 0;
    let mut reserve_ov2: WORD = 0;
    let mut reservea: [WORD; 2] = [0; 2];
    let mut i_0: WORD = 0 as WORD;
    let mut usb: WORD = (*ptr_sbr_dec).str_codec_qmf_bank.usb as WORD;
    let mut iter_val: WORD = 1 as WORD;
    if audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int
        || audio_object_type == AOT_ER_AAC_LD as core::ffi::c_int
    {
        iter_val = 0 as core::ffi::c_int as WORD;
    }
    loop {
        let mut t1: WORD = op_delay;
        let mut t2: WORD = no_bins + op_delay;
        if i_0 != 0 {
            t1 = 0 as core::ffi::c_int as WORD;
            t2 = op_delay;
        }
        reservea[i_0 as usize] = (Some(
            ixheaacd_ixheaacd_expsubbandsamples.expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            p_arr_qmf_buf_real.as_mut_ptr(),
            p_arr_qmf_buf_imag.as_mut_ptr(),
            0 as WORD32,
            usb as WORD32,
            t1 as WORD32,
            t2 as WORD32,
            low_pow_flag,
        ) as WORD;
        i_0 += 1;
        if !(i_0 <= iter_val) {
            break;
        }
    }
    reserve = reservea[0 as core::ffi::c_int as usize];
    if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
        && audio_object_type != AOT_ER_AAC_LD as core::ffi::c_int
    {
        reserve_ov1 = reservea[1 as core::ffi::c_int as usize];
    } else {
        reserve_ov1 = reserve;
    }
    (*ptr_sbr_dec).max_samp_val = ixheaac_min32(
        reserve as WORD32,
        reserve_ov1 as WORD32,
    );
    reserve_ov2 = (Some(
        ixheaacd_ixheaacd_expsubbandsamples.expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(
        ((*ptr_sbr_dec).str_hf_generator.lpc_filt_states_real).as_mut_ptr(),
        ((*ptr_sbr_dec).str_hf_generator.lpc_filt_states_imag).as_mut_ptr(),
        0 as WORD32,
        usb as WORD32,
        0 as WORD32,
        LPC_ORDER,
        low_pow_flag,
    ) as WORD;
    reserve_ov1 = ixheaac_min32(reserve_ov1 as WORD32, reserve_ov2 as WORD32) as WORD;
    shift1 = (*ptr_sbr_dec).str_sbr_scale_fact.lb_scale as WORD + reserve;
    shift2 = (*ptr_sbr_dec).str_sbr_scale_fact.ov_lb_scale as WORD + reserve_ov1;
    min_shift = ixheaac_min32(shift1 as WORD32, shift2 as WORD32) as WORD;
    shift_over = shift2 - min_shift;
    reserve -= shift1 - min_shift;
    (*ptr_sbr_dec).str_sbr_scale_fact.ov_lb_scale = ((*ptr_sbr_dec)
        .str_sbr_scale_fact
        .ov_lb_scale as core::ffi::c_int
        + (reserve_ov1 - shift_over) as core::ffi::c_int) as WORD16;
    (Some(ixheaacd_adjust_scale.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        p_arr_qmf_buf_real.as_mut_ptr(),
        p_arr_qmf_buf_imag.as_mut_ptr(),
        0 as WORD32,
        usb as WORD32,
        0 as WORD32,
        op_delay as WORD32,
        reserve_ov1 as WORD32 - shift_over as WORD32,
        low_pow_flag,
    );
    (Some(ixheaacd_adjust_scale.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        p_arr_qmf_buf_real.as_mut_ptr(),
        p_arr_qmf_buf_imag.as_mut_ptr(),
        0 as WORD32,
        usb as WORD32,
        op_delay as WORD32,
        no_bins as WORD32 + op_delay as WORD32,
        reserve as WORD32,
        low_pow_flag,
    );
    (Some(ixheaacd_adjust_scale.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        ((*ptr_sbr_dec).str_hf_generator.lpc_filt_states_real).as_mut_ptr(),
        ((*ptr_sbr_dec).str_hf_generator.lpc_filt_states_imag).as_mut_ptr(),
        0 as WORD32,
        usb as WORD32,
        0 as WORD32,
        LPC_ORDER,
        reserve_ov1 as WORD32 - shift_over as WORD32,
        low_pow_flag,
    );
    (*ptr_sbr_dec).str_sbr_scale_fact.lb_scale = ((*ptr_sbr_dec)
        .str_sbr_scale_fact
        .lb_scale as core::ffi::c_int + reserve as core::ffi::c_int) as WORD16;
    save_lb_scale = (*ptr_sbr_dec).str_sbr_scale_fact.lb_scale as WORD;
    let mut num_0: WORD32 = no_bins as WORD32;
    let mut p_loc_qmf_real: *mut WORD32 = &mut *(*p_arr_qmf_buf_real
        .as_mut_ptr()
        .offset(op_delay as isize))
        .offset(NO_ANALYSIS_CHANNELS as isize) as *mut WORD32;
    if low_pow_flag == 0 {
        num_0 = num_0 << 1 as core::ffi::c_int;
    }
    ixheaacd_clr_subsamples(
        p_loc_qmf_real,
        num_0 - 1 as WORD32,
        NO_SYNTHESIS_CHANNELS - NO_ANALYSIS_CHANNELS,
    );
    if apply_processing != 0 {
        let mut degree_alias: [WORD16; 64] = [0; 64];
        let mut border_vec_0: *mut WORD16 = ((*ptr_frame_data)
            .str_frame_info_details
            .border_vec)
            .as_mut_ptr();
        if low_pow_flag != 0 {
            memset(
                degree_alias.as_mut_ptr() as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                (NO_SYNTHESIS_CHANNELS as size_t)
                    .wrapping_mul(::core::mem::size_of::<WORD16>() as size_t),
            );
        }
        if low_pow_flag != 0 {
            let mut com_low_band_scale: WORD32 = 0;
            ixheaacd_low_pow_hf_generator(
                &mut (*ptr_sbr_dec).str_hf_generator,
                p_arr_qmf_buf_real.as_mut_ptr(),
                degree_alias.as_mut_ptr(),
                *border_vec_0.offset(0 as core::ffi::c_int as isize) as WORD
                    * (*ptr_header_data).time_step as WORD,
                (*ptr_header_data).time_step as WORD
                    * ixheaac_sub16_sat(
                        *border_vec_0
                            .offset(
                                (*ptr_frame_data).str_frame_info_details.num_env as isize,
                            ),
                        (*ptr_header_data).num_time_slots,
                    ) as WORD,
                (*(*ptr_header_data).pstr_freq_band_data).num_if_bands as WORD,
                (*ptr_frame_data).max_qmf_subband_aac as WORD,
                ((*ptr_frame_data).sbr_invf_mode).as_mut_ptr(),
                ((*ptr_frame_data_prev).sbr_invf_mode).as_mut_ptr(),
                (*ptr_sbr_dec).max_samp_val,
                ptr_work_buf_core,
            );
            com_low_band_scale = ixheaac_min32(
                (*ptr_sbr_dec).str_sbr_scale_fact.ov_lb_scale as WORD32,
                (*ptr_sbr_dec).str_sbr_scale_fact.lb_scale as WORD32,
            );
            (*ptr_sbr_dec).str_sbr_scale_fact.hb_scale = (com_low_band_scale
                as core::ffi::c_int - 2 as core::ffi::c_int) as WORD16;
        } else if ldmps_present == 1 as core::ffi::c_int {
            err_code = ixheaacd_generate_hf(
                ((*ptr_sbr_dec).mps_qmf_buf_real)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as isize),
                ((*ptr_sbr_dec).mps_qmf_buf_imag)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as isize),
                ((*ptr_sbr_dec).ph_vocod_qmf_real)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as isize),
                ((*ptr_sbr_dec).ph_vocod_qmf_imag)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as isize),
                ((*ptr_sbr_dec).sbr_qmf_out_real)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as isize),
                ((*ptr_sbr_dec).sbr_qmf_out_imag)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as isize),
                ptr_frame_data,
                ptr_header_data,
                ldmps_present,
                (*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as WORD32,
                ec_flag,
            ) as IA_ERRORCODE;
            if err_code != 0 {
                return err_code as WORD32;
            }
        } else {
            ixheaacd_hf_generator(
                &mut (*ptr_sbr_dec).str_hf_generator,
                &mut (*ptr_sbr_dec).str_sbr_scale_fact,
                p_arr_qmf_buf_real.as_mut_ptr(),
                p_arr_qmf_buf_imag.as_mut_ptr(),
                (*ptr_header_data).time_step as WORD,
                *border_vec_0.offset(0 as core::ffi::c_int as isize) as WORD,
                ixheaac_sub16_sat(
                    *border_vec_0
                        .offset(
                            (*ptr_frame_data).str_frame_info_details.num_env as isize,
                        ),
                    (*ptr_header_data).num_time_slots,
                ) as WORD,
                (*(*ptr_header_data).pstr_freq_band_data).num_if_bands as WORD,
                (*ptr_frame_data).max_qmf_subband_aac as WORD,
                ((*ptr_frame_data).sbr_invf_mode).as_mut_ptr(),
                ((*ptr_frame_data_prev).sbr_invf_mode).as_mut_ptr(),
                ptr_work_buf_core,
                audio_object_type as WORD,
            );
        }
        if ldmps_present == 1 as core::ffi::c_int {
            (*ptr_frame_data).pstr_sbr_header = ptr_header_data;
            err_code = ixheaacd_sbr_env_calc(
                ptr_frame_data,
                ((*ptr_sbr_dec).sbr_qmf_out_real)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as isize),
                ((*ptr_sbr_dec).sbr_qmf_out_imag)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as isize),
                ((*ptr_sbr_dec).qmf_buf_real)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as isize),
                ((*ptr_sbr_dec).qmf_buf_imag)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as isize),
                0 as *mut WORD32,
                ((*ptr_sbr_dec).scratch_buff).as_mut_ptr(),
                pvc_dec_out_buf.as_mut_ptr(),
                ldmps_present,
                ec_flag,
            ) as IA_ERRORCODE;
            j = 0 as core::ffi::c_int as WORD;
            while j
                < (*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as core::ffi::c_int
                    + 2 as core::ffi::c_int
            {
                k = (*ptr_sbr_dec).str_codec_qmf_bank.usb as WORD;
                while k < 64 as core::ffi::c_int {
                    (*ptr_sbr_dec).mps_qmf_buf_real[j as usize][k as usize]
                        += (*ptr_sbr_dec).sbr_qmf_out_real[j as usize][k as usize];
                    (*ptr_sbr_dec).mps_qmf_buf_imag[j as usize][k as usize]
                        += (*ptr_sbr_dec).sbr_qmf_out_imag[j as usize][k as usize];
                    k += 1;
                }
                j += 1;
            }
        } else {
            err_code = ixheaacd_calc_sbrenvelope(
                &mut (*ptr_sbr_dec).str_sbr_scale_fact,
                &mut (*ptr_sbr_dec).str_sbr_calc_env,
                ptr_header_data,
                ptr_frame_data,
                ptr_frame_data_prev,
                p_arr_qmf_buf_real.as_mut_ptr(),
                p_arr_qmf_buf_imag.as_mut_ptr(),
                degree_alias.as_mut_ptr(),
                low_pow_flag,
                sbr_tables_ptr,
                pstr_common_tables,
                ptr_work_buf_core
                    .offset(
                        (LPC_ORDER
                            << 6 as core::ffi::c_int
                                + (low_pow_flag == 0) as core::ffi::c_int) as isize,
                    ),
                audio_object_type,
            );
            if err_code != 0 {
                return err_code as WORD32;
            }
        }
        memcpy(
            ((*ptr_frame_data_prev).sbr_invf_mode).as_mut_ptr()
                as *mut core::ffi::c_void,
            ((*ptr_frame_data).sbr_invf_mode).as_mut_ptr() as *const core::ffi::c_void,
            ((*(*ptr_header_data).pstr_freq_band_data).num_if_bands as size_t)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
        );
        (*ptr_frame_data_prev).coupling_mode = (*ptr_frame_data).coupling_mode;
        (*ptr_frame_data_prev).max_qmf_subband_aac = (*ptr_frame_data)
            .max_qmf_subband_aac;
        (*ptr_frame_data_prev).end_position = *border_vec_0
            .offset((*ptr_frame_data).str_frame_info_details.num_env as isize);
        (*ptr_frame_data_prev).amp_res = (*ptr_frame_data).amp_res;
    } else {
        (*ptr_sbr_dec).str_sbr_scale_fact.hb_scale = save_lb_scale as WORD16;
    }
    if low_pow_flag == 0 {
        i = 0 as core::ffi::c_int as WORD;
        while i < LPC_ORDER {
            let mut p_loc_qmf_real_0: *mut WORD32 = &mut *(*p_arr_qmf_buf_real
                .as_mut_ptr()
                .offset((no_bins - LPC_ORDER + i) as isize))
                .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
            let mut p_loc_qmf_imag: *mut WORD32 = &mut *(*p_arr_qmf_buf_imag
                .as_mut_ptr()
                .offset((no_bins - LPC_ORDER + i) as isize))
                .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
            let mut plpc_filt_states_real: *mut WORD32 = &mut *(*((*ptr_sbr_dec)
                .str_hf_generator
                .lpc_filt_states_real)
                .as_mut_ptr()
                .offset(i as isize))
                .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
            let mut plpc_filt_states_imag: *mut WORD32 = &mut *(*((*ptr_sbr_dec)
                .str_hf_generator
                .lpc_filt_states_imag)
                .as_mut_ptr()
                .offset(i as isize))
                .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
            memcpy(
                plpc_filt_states_real as *mut core::ffi::c_void,
                p_loc_qmf_real_0 as *const core::ffi::c_void,
                (::core::mem::size_of::<WORD32>() as size_t)
                    .wrapping_mul((*ptr_sbr_dec).str_codec_qmf_bank.usb as size_t),
            );
            memcpy(
                plpc_filt_states_imag as *mut core::ffi::c_void,
                p_loc_qmf_imag as *const core::ffi::c_void,
                (::core::mem::size_of::<WORD32>() as size_t)
                    .wrapping_mul((*ptr_sbr_dec).str_codec_qmf_bank.usb as size_t),
            );
            i += 1;
        }
    } else {
        i = 0 as core::ffi::c_int as WORD;
        while i < LPC_ORDER {
            let mut p_loc_qmf_real_1: *mut WORD32 = &mut *(*p_arr_qmf_buf_real
                .as_mut_ptr()
                .offset((no_bins - LPC_ORDER + i) as isize))
                .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
            let mut plpc_filt_states_real_0: *mut WORD32 = &mut *(*((*ptr_sbr_dec)
                .str_hf_generator
                .lpc_filt_states_real)
                .as_mut_ptr()
                .offset(i as isize))
                .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
            memcpy(
                plpc_filt_states_real_0 as *mut core::ffi::c_void,
                p_loc_qmf_real_1 as *const core::ffi::c_void,
                (::core::mem::size_of::<WORD32>() as size_t)
                    .wrapping_mul((*ptr_sbr_dec).str_codec_qmf_bank.usb as size_t),
            );
            i += 1;
        }
    }
    if apply_processing != 0 && (*ptr_header_data).channel_mode == PS_STEREO
        && (audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
            && audio_object_type != AOT_ER_AAC_LD as core::ffi::c_int)
    {
        let mut ps_scale: WORD32 = 0;
        ixheaacd_init_ps_scale(ptr_ps_dec, &mut (*ptr_sbr_dec).str_sbr_scale_fact);
        ixheaacd_cplx_synt_qmffilt(
            p_arr_qmf_buf_real.as_mut_ptr(),
            p_arr_qmf_buf_imag.as_mut_ptr(),
            op_delay as WORD32,
            ((*ptr_sbr_dec).p_arr_qmf_buf_real).as_mut_ptr(),
            ((*ptr_sbr_dec).p_arr_qmf_buf_imag).as_mut_ptr(),
            &mut (*ptr_sbr_dec).str_sbr_scale_fact,
            ptr_time_data,
            &mut (*ptr_sbr_dec).str_synthesis_qmf_bank,
            ptr_ps_dec,
            1 as FLAG,
            0 as FLAG,
            sbr_tables_ptr,
            pstr_common_tables,
            ch_fac,
            drc_on,
            drc_sbr_factors,
            audio_object_type,
        );
        ps_scale = (*ptr_sbr_dec).str_sbr_scale_fact.ps_scale as WORD32;
        (*ptr_sbr_sf_r).ov_lb_scale = ps_scale as WORD16;
        (*ptr_sbr_sf_r).lb_scale = ps_scale as WORD16;
        (*ptr_sbr_sf_r).hb_scale = ps_scale as WORD16;
        ixheaacd_cplx_synt_qmffilt(
            p_arr_qmf_buf_real.as_mut_ptr(),
            p_arr_qmf_buf_imag.as_mut_ptr(),
            op_delay as WORD32,
            ((*ptr_sbr_dec).p_arr_qmf_buf_real).as_mut_ptr(),
            ((*ptr_sbr_dec).p_arr_qmf_buf_imag).as_mut_ptr(),
            ptr_sbr_sf_r,
            ptr_time_data.offset(1 as core::ffi::c_int as isize),
            ptr_qmf_synth_bank_r,
            ptr_ps_dec,
            0 as FLAG,
            0 as FLAG,
            sbr_tables_ptr,
            pstr_common_tables,
            ch_fac,
            drc_on,
            drc_sbr_factors,
            audio_object_type,
        );
    } else {
        ixheaacd_cplx_synt_qmffilt(
            p_arr_qmf_buf_real.as_mut_ptr(),
            p_arr_qmf_buf_imag.as_mut_ptr(),
            op_delay as WORD32,
            ((*ptr_sbr_dec).p_arr_qmf_buf_real).as_mut_ptr(),
            ((*ptr_sbr_dec).p_arr_qmf_buf_imag).as_mut_ptr(),
            &mut (*ptr_sbr_dec).str_sbr_scale_fact,
            ptr_time_data,
            &mut (*ptr_sbr_dec).str_synthesis_qmf_bank,
            ptr_ps_dec,
            0 as FLAG,
            low_pow_flag,
            sbr_tables_ptr,
            pstr_common_tables,
            ch_fac,
            drc_on,
            drc_sbr_factors,
            audio_object_type,
        );
    }
    let mut num_1: WORD32 = op_delay as WORD32;
    if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int {
        let mut p_loc_qmf_real_2: *mut WORD32 = (*ptr_sbr_dec).ptr_sbr_overlap_buf;
        let mut p_loc_qmf_real_1_0: *mut WORD32 = &mut *(*p_arr_qmf_buf_real
            .as_mut_ptr()
            .offset(no_bins as isize))
            .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
        memcpy(
            p_loc_qmf_real_2 as *mut core::ffi::c_void,
            p_loc_qmf_real_1_0 as *const core::ffi::c_void,
            (::core::mem::size_of::<WORD32>() as size_t)
                .wrapping_mul(NO_SYNTHESIS_CHANNELS as size_t)
                .wrapping_mul(num_1 as size_t),
        );
    }
    if low_pow_flag == 0 {
        num_1 = num_1 << 1 as core::ffi::c_int;
    }
    if ldmps_present == 1 as core::ffi::c_int {
        memmove(
            &mut *(*((*ptr_sbr_dec).mps_qmf_buf_real)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                as *mut core::ffi::c_void,
            &mut *(*((*ptr_sbr_dec).mps_qmf_buf_real)
                .as_mut_ptr()
                .offset((*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                as *const core::ffi::c_void,
            (SBR_HF_ADJ_OFFSET as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t)
                .wrapping_mul(64 as size_t),
        );
        memmove(
            &mut *(*((*ptr_sbr_dec).mps_qmf_buf_imag)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                as *mut core::ffi::c_void,
            &mut *(*((*ptr_sbr_dec).mps_qmf_buf_imag)
                .as_mut_ptr()
                .offset((*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                as *const core::ffi::c_void,
            (SBR_HF_ADJ_OFFSET as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t)
                .wrapping_mul(64 as size_t),
        );
    }
    (*ptr_sbr_dec).str_sbr_scale_fact.ov_lb_scale = save_lb_scale as WORD16;
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_esbr_dec(
    mut ptr_sbr_dec: *mut ia_sbr_dec_struct,
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut ptr_frame_data: *mut ia_sbr_frame_info_data_struct,
    mut apply_processing: FLAG,
    mut low_pow_flag: FLAG,
    mut ptr_sbr_tables: *mut ia_sbr_tables_struct,
    mut ch_fac: WORD,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut op_delay: WORD32 = 0;
    let mut codec_x_delay: WORD32 = 0 as WORD32;
    let mut pp_qmf_buf_real: *mut *mut FLOAT32 = (*ptr_sbr_dec).pp_qmf_buf_real;
    let mut pp_qmf_buf_imag: *mut *mut FLOAT32 = (*ptr_sbr_dec).pp_qmf_buf_imag;
    let mut upsample_ratio_idx: WORD32 = (*ptr_header_data).sbr_ratio_idx;
    let mut mps_sbr_flag: WORD32 = (*ptr_frame_data).mps_sbr_flag as WORD32;
    let mut stereo_config_idx: WORD32 = (*ptr_frame_data).stereo_config_idx;
    let mut hbe_flag: WORD32 = (*ptr_header_data).hbe_flag as WORD32;
    let mut sbr_mode: WORD32 = (*ptr_frame_data).sbr_mode as WORD32;
    op_delay = 6 as core::ffi::c_int as WORD32;
    if upsample_ratio_idx == SBR_UPSAMPLE_IDX_4_1 {
        op_delay = (2 as core::ffi::c_int * 6 as core::ffi::c_int) as WORD32;
    }
    (*ptr_sbr_dec).str_sbr_scale_fact.lb_scale = 0 as WORD16;
    if hbe_flag != 0 {
        codec_x_delay = 32 as core::ffi::c_int as WORD32;
    }
    if upsample_ratio_idx == SBR_UPSAMPLE_IDX_4_1 {
        codec_x_delay = 2 as WORD32 * codec_x_delay;
    }
    memmove(
        &mut *(*((*ptr_sbr_dec).qmf_buf_real)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
            as *mut core::ffi::c_void,
        &mut *(*((*ptr_sbr_dec).qmf_buf_real)
            .as_mut_ptr()
            .offset((*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as isize))
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
            as *const core::ffi::c_void,
        ((op_delay + SBR_HF_ADJ_OFFSET + codec_x_delay) as size_t)
            .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t)
            .wrapping_mul(64 as size_t),
    );
    memmove(
        &mut *(*((*ptr_sbr_dec).qmf_buf_imag)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
            as *mut core::ffi::c_void,
        &mut *(*((*ptr_sbr_dec).qmf_buf_imag)
            .as_mut_ptr()
            .offset((*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as isize))
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
            as *const core::ffi::c_void,
        ((op_delay + SBR_HF_ADJ_OFFSET + codec_x_delay) as size_t)
            .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t)
            .wrapping_mul(64 as size_t),
    );
    memmove(
        &mut *(*((*ptr_sbr_dec).sbr_qmf_out_real)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
            as *mut core::ffi::c_void,
        &mut *(*((*ptr_sbr_dec).sbr_qmf_out_real)
            .as_mut_ptr()
            .offset((*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as isize))
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
            as *const core::ffi::c_void,
        ((op_delay as core::ffi::c_int + SBR_HF_ADJ_OFFSET) as size_t)
            .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t)
            .wrapping_mul(64 as size_t),
    );
    memmove(
        &mut *(*((*ptr_sbr_dec).sbr_qmf_out_imag)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
            as *mut core::ffi::c_void,
        &mut *(*((*ptr_sbr_dec).sbr_qmf_out_imag)
            .as_mut_ptr()
            .offset((*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as isize))
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
            as *const core::ffi::c_void,
        ((op_delay as core::ffi::c_int + SBR_HF_ADJ_OFFSET) as size_t)
            .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t)
            .wrapping_mul(64 as size_t),
    );
    if hbe_flag != 0 {
        memmove(
            &mut *(*((*ptr_sbr_dec).ph_vocod_qmf_real)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                as *mut core::ffi::c_void,
            &mut *(*((*ptr_sbr_dec).ph_vocod_qmf_real)
                .as_mut_ptr()
                .offset((*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                as *const core::ffi::c_void,
            (64 as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t)
                .wrapping_mul(
                    (op_delay as core::ffi::c_int + SBR_HF_ADJ_OFFSET) as size_t,
                ),
        );
        memmove(
            ((*ptr_sbr_dec).ph_vocod_qmf_imag).as_mut_ptr() as *mut core::ffi::c_void,
            ((*ptr_sbr_dec).ph_vocod_qmf_imag)
                .as_mut_ptr()
                .offset(
                    (*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as core::ffi::c_int
                        as isize,
                ) as *const core::ffi::c_void,
            (64 as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t)
                .wrapping_mul(
                    (op_delay as core::ffi::c_int + SBR_HF_ADJ_OFFSET) as size_t,
                ),
        );
    }
    ixheaacd_esbr_analysis_filt_block(
        ptr_sbr_dec,
        ptr_sbr_tables,
        op_delay + codec_x_delay + SBR_HF_ADJ_OFFSET,
    );
    if hbe_flag != 0 {
        let mut err: WORD32 = ixheaacd_qmf_hbe_apply(
            (*ptr_sbr_dec).p_hbe_txposer,
            ((*ptr_sbr_dec).qmf_buf_real)
                .as_mut_ptr()
                .offset((op_delay as core::ffi::c_int + SBR_HF_ADJ_OFFSET) as isize)
                .offset(ESBR_HBE_DELAY_OFFSET as isize),
            ((*ptr_sbr_dec).qmf_buf_imag)
                .as_mut_ptr()
                .offset((op_delay as core::ffi::c_int + SBR_HF_ADJ_OFFSET) as isize)
                .offset(ESBR_HBE_DELAY_OFFSET as isize),
            (*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as WORD32,
            ((*ptr_sbr_dec).ph_vocod_qmf_real)
                .as_mut_ptr()
                .offset((op_delay as core::ffi::c_int + SBR_HF_ADJ_OFFSET) as isize),
            ((*ptr_sbr_dec).ph_vocod_qmf_imag)
                .as_mut_ptr()
                .offset((op_delay as core::ffi::c_int + SBR_HF_ADJ_OFFSET) as isize),
            (*ptr_frame_data).pitch_in_bins,
            ptr_header_data,
        );
        if err != 0 {
            return err;
        }
        if upsample_ratio_idx == SBR_UPSAMPLE_IDX_4_1 {
            ixheaacd_hbe_repl_spec(
                &mut *((*(*ptr_sbr_dec).p_hbe_txposer).x_over_qmf)
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize),
                ((*ptr_sbr_dec).ph_vocod_qmf_real)
                    .as_mut_ptr()
                    .offset((op_delay as core::ffi::c_int + SBR_HF_ADJ_OFFSET) as isize),
                ((*ptr_sbr_dec).ph_vocod_qmf_imag)
                    .as_mut_ptr()
                    .offset((op_delay as core::ffi::c_int + SBR_HF_ADJ_OFFSET) as isize),
                (*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as WORD32,
                (*(*ptr_sbr_dec).p_hbe_txposer).max_stretch,
            );
        }
    }
    ixheaacd_qmf_enrg_calc(ptr_sbr_dec, upsample_ratio_idx, low_pow_flag as WORD32);
    i = 0 as core::ffi::c_int as WORD32;
    while i < 64 as core::ffi::c_int {
        memset(
            ((*ptr_sbr_dec).sbr_qmf_out_real[i as usize]).as_mut_ptr()
                as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (64 as size_t).wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        memset(
            ((*ptr_sbr_dec).sbr_qmf_out_imag[i as usize]).as_mut_ptr()
                as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (64 as size_t).wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        i += 1;
    }
    (*ptr_sbr_dec).band_count = (*ptr_sbr_dec).str_codec_qmf_bank.no_channels;
    ixheaacd_esbr_synthesis_filt_block(
        ptr_sbr_dec,
        ptr_header_data,
        ptr_frame_data,
        apply_processing as WORD32,
        pp_qmf_buf_real,
        pp_qmf_buf_imag,
        stereo_config_idx,
        ptr_sbr_tables,
        mps_sbr_flag,
        ch_fac as WORD32,
        0 as WORD32,
        0 as WORD32,
        0 as *mut ia_ps_dec_struct,
        0 as FLAG,
        0 as *mut [WORD32; 64],
    );
    (*ptr_frame_data).prev_sbr_mode = sbr_mode as FLAG;
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_sbr_dec_from_mps(
    mut p_mps_qmf_output: *mut FLOAT32,
    mut p_sbr_dec: *mut core::ffi::c_void,
    mut p_sbr_frame: *mut core::ffi::c_void,
    mut p_sbr_header: *mut core::ffi::c_void,
    mut ec_flag: WORD32,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut ptr_frame_data: *mut ia_sbr_frame_info_data_struct = p_sbr_frame
        as *mut ia_sbr_frame_info_data_struct;
    let mut ptr_header_data: *mut ia_sbr_header_data_struct = p_sbr_header
        as *mut ia_sbr_header_data_struct;
    let mut ptr_sbr_dec: *mut ia_sbr_dec_struct = p_sbr_dec as *mut ia_sbr_dec_struct;
    let mut p_frame_info: *mut ia_frame_info_struct = &mut (*ptr_frame_data)
        .str_frame_info_details;
    let mut no_bins: WORD32 = 0;
    let mut upsample_ratio_idx: WORD32 = (*ptr_header_data).sbr_ratio_idx;
    let mut op_delay: WORD32 = 6 as WORD32 + SBR_HF_ADJ_OFFSET;
    let mut num_anal_bands: WORD32 = 40 as WORD32;
    let mut mps_sbr_flag: WORD32 = (*ptr_frame_data).mps_sbr_flag as WORD32;
    let mut err: WORD32 = 0 as WORD32;
    if (*ptr_header_data).is_usf_4 != 0 {
        op_delay += 6 as core::ffi::c_int;
    }
    num_anal_bands = num_anal_bands - (upsample_ratio_idx << 3 as core::ffi::c_int);
    if mps_sbr_flag == 0 {
        return 0 as WORD32
    } else {
        (*ptr_frame_data).cov_count = (*ptr_sbr_dec).str_codec_qmf_bank.no_channels;
    }
    no_bins = ((*ptr_header_data).output_framesize as core::ffi::c_int
        / 64 as core::ffi::c_int) as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < no_bins {
        let mut p_loc_mps_qmf_output: *mut FLOAT32 = p_mps_qmf_output
            .offset(
                (i as core::ffi::c_int
                    * (MAX_NUM_QMF_BANDS_ESBR * 2 as core::ffi::c_int)) as isize,
            );
        k = 0 as core::ffi::c_int as WORD32;
        while k
            < (*(*ptr_header_data).pstr_freq_band_data).sub_band_start
                as core::ffi::c_int
        {
            let fresh30 = p_loc_mps_qmf_output;
            p_loc_mps_qmf_output = p_loc_mps_qmf_output.offset(1);
            (*ptr_sbr_dec).mps_qmf_buf_real[(op_delay + i) as usize][k as usize] = *fresh30;
            let fresh31 = p_loc_mps_qmf_output;
            p_loc_mps_qmf_output = p_loc_mps_qmf_output.offset(1);
            (*ptr_sbr_dec).mps_qmf_buf_imag[(op_delay + i) as usize][k as usize] = *fresh31;
            (*ptr_sbr_dec)
                .mps_sbr_qmf_buf_real[(SBR_HF_ADJ_OFFSET + i) as usize][k as usize] = (*ptr_sbr_dec)
                .qmf_buf_real[(SBR_HF_ADJ_OFFSET + i) as usize][k as usize];
            (*ptr_sbr_dec)
                .mps_sbr_qmf_buf_imag[(SBR_HF_ADJ_OFFSET + i) as usize][k as usize] = (*ptr_sbr_dec)
                .qmf_buf_real[(SBR_HF_ADJ_OFFSET + i) as usize][k as usize];
            k += 1;
        }
        i += 1;
    }
    if (*ptr_frame_data).reset_flag != 0 {
        let mut l: WORD32 = 0;
        let mut start_band: WORD32 = (*(*ptr_header_data).pstr_freq_band_data)
            .qmf_sb_prev as WORD32;
        let mut end_band: WORD32 = num_anal_bands;
        let mut start_slot: WORD32 = SBR_HF_ADJ_OFFSET
            + (*ptr_frame_data).rate
                * (*p_frame_info).border_vec[0 as core::ffi::c_int as usize] as WORD32;
        l = start_slot;
        while l < op_delay {
            k = start_band;
            while k < end_band {
                (*ptr_sbr_dec).mps_qmf_buf_real[l as usize][k as usize] = 0.0f32;
                (*ptr_sbr_dec).mps_qmf_buf_imag[l as usize][k as usize] = 0.0f32;
                k += 1;
            }
            l += 1;
        }
        l = 0 as core::ffi::c_int as WORD32;
        while l < SBR_HF_ADJ_OFFSET {
            k = start_band;
            while k < end_band {
                (*ptr_sbr_dec).mps_qmf_buf_real[l as usize][k as usize] = 0.0f32;
                (*ptr_sbr_dec).mps_qmf_buf_imag[l as usize][k as usize] = 0.0f32;
                k += 1;
            }
            l += 1;
        }
    }
    (*(*ptr_header_data).pstr_freq_band_data).qmf_sb_prev = (*(*ptr_header_data)
        .pstr_freq_band_data)
        .sub_band_start;
    err = ixheaacd_generate_hf(
        ((*ptr_sbr_dec).mps_qmf_buf_real)
            .as_mut_ptr()
            .offset(SBR_HF_ADJ_OFFSET as isize),
        ((*ptr_sbr_dec).mps_qmf_buf_imag)
            .as_mut_ptr()
            .offset(SBR_HF_ADJ_OFFSET as isize),
        0 as *mut [FLOAT32; 64],
        0 as *mut [FLOAT32; 64],
        ((*ptr_sbr_dec).mps_sbr_qmf_buf_real)
            .as_mut_ptr()
            .offset(SBR_HF_ADJ_OFFSET as isize),
        ((*ptr_sbr_dec).mps_sbr_qmf_buf_imag)
            .as_mut_ptr()
            .offset(SBR_HF_ADJ_OFFSET as isize),
        ptr_frame_data,
        ptr_header_data,
        0 as WORD32,
        (*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots as WORD32,
        ec_flag,
    );
    if err != 0 {
        return err;
    }
    (*ptr_frame_data).pstr_sbr_header = ptr_header_data;
    (*ptr_frame_data).sbr_mode = ORIG_SBR as core::ffi::c_int as FLAG;
    (*ptr_frame_data).prev_sbr_mode = ORIG_SBR as core::ffi::c_int as FLAG;
    err = ixheaacd_sbr_env_calc(
        ptr_frame_data,
        ((*ptr_sbr_dec).mps_sbr_qmf_buf_real)
            .as_mut_ptr()
            .offset(SBR_HF_ADJ_OFFSET as isize),
        ((*ptr_sbr_dec).mps_sbr_qmf_buf_imag)
            .as_mut_ptr()
            .offset(SBR_HF_ADJ_OFFSET as isize),
        ((*ptr_sbr_dec).mps_qmf_buf_real)
            .as_mut_ptr()
            .offset(SBR_HF_ADJ_OFFSET as isize),
        ((*ptr_sbr_dec).mps_qmf_buf_imag)
            .as_mut_ptr()
            .offset(SBR_HF_ADJ_OFFSET as isize),
        if (*ptr_header_data).hbe_flag == 0 as core::ffi::c_int {
            0 as *mut WORD32
        } else {
            ((*(*ptr_sbr_dec).p_hbe_txposer).x_over_qmf).as_mut_ptr()
        },
        ((*ptr_sbr_dec).scratch_buff).as_mut_ptr(),
        0 as *mut FLOAT32,
        0 as WORD32,
        ec_flag,
    );
    if err != 0 {
        return err;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < no_bins {
        let mut p_loc_mps_qmf_output_0: *mut FLOAT32 = p_mps_qmf_output
            .offset(
                (i as core::ffi::c_int
                    * (MAX_NUM_QMF_BANDS_ESBR * 2 as core::ffi::c_int)) as isize,
            );
        k = 0 as core::ffi::c_int as WORD32;
        while k
            < (*(*ptr_header_data).pstr_freq_band_data).sub_band_start
                as core::ffi::c_int
        {
            let fresh32 = p_loc_mps_qmf_output_0;
            p_loc_mps_qmf_output_0 = p_loc_mps_qmf_output_0.offset(1);
            *fresh32 = (*ptr_sbr_dec)
                .mps_qmf_buf_real[(SBR_HF_ADJ_OFFSET + i) as usize][k as usize];
            let fresh33 = p_loc_mps_qmf_output_0;
            p_loc_mps_qmf_output_0 = p_loc_mps_qmf_output_0.offset(1);
            *fresh33 = (*ptr_sbr_dec)
                .mps_qmf_buf_imag[(SBR_HF_ADJ_OFFSET + i) as usize][k as usize];
            k += 1;
        }
        k = (*(*ptr_header_data).pstr_freq_band_data).sub_band_start as WORD32;
        while k < 64 as core::ffi::c_int {
            let fresh34 = p_loc_mps_qmf_output_0;
            p_loc_mps_qmf_output_0 = p_loc_mps_qmf_output_0.offset(1);
            *fresh34 = (*ptr_sbr_dec)
                .mps_sbr_qmf_buf_real[(SBR_HF_ADJ_OFFSET + i) as usize][k as usize];
            let fresh35 = p_loc_mps_qmf_output_0;
            p_loc_mps_qmf_output_0 = p_loc_mps_qmf_output_0.offset(1);
            *fresh35 = (*ptr_sbr_dec)
                .mps_sbr_qmf_buf_imag[(SBR_HF_ADJ_OFFSET + i) as usize][k as usize];
            k += 1;
        }
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < op_delay {
        memmove(
            ((*ptr_sbr_dec).mps_qmf_buf_real[i as usize]).as_mut_ptr()
                as *mut core::ffi::c_void,
            ((*ptr_sbr_dec).mps_qmf_buf_real[(no_bins + i) as usize]).as_mut_ptr()
                as *const core::ffi::c_void,
            (64 as size_t).wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        memmove(
            ((*ptr_sbr_dec).mps_qmf_buf_imag[i as usize]).as_mut_ptr()
                as *mut core::ffi::c_void,
            ((*ptr_sbr_dec).mps_qmf_buf_imag[(no_bins + i) as usize]).as_mut_ptr()
                as *const core::ffi::c_void,
            (64 as size_t).wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        memmove(
            ((*ptr_sbr_dec).mps_sbr_qmf_buf_real[i as usize]).as_mut_ptr()
                as *mut core::ffi::c_void,
            ((*ptr_sbr_dec).mps_sbr_qmf_buf_real[(no_bins + i) as usize]).as_mut_ptr()
                as *const core::ffi::c_void,
            (64 as size_t).wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        memmove(
            ((*ptr_sbr_dec).mps_sbr_qmf_buf_imag[i as usize]).as_mut_ptr()
                as *mut core::ffi::c_void,
            ((*ptr_sbr_dec).mps_sbr_qmf_buf_imag[(no_bins + i) as usize]).as_mut_ptr()
                as *const core::ffi::c_void,
            (64 as size_t).wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        i += 1;
    }
    (*ptr_frame_data).reset_flag = 0 as core::ffi::c_int as FLAG;
    return err;
}
