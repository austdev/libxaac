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
    fn ixheaacd_create_init_bit_buf(
        it_bit_buff: *mut ia_bit_buf_struct,
        ptr_bit_buf_base: *mut UWORD8,
        bit_buf_size: WORD32,
    ) -> VOID;
    fn ixheaacd_read_bits_buf(
        it_bit_buff: *mut ia_bit_buf_struct,
        no_of_bits: WORD,
    ) -> WORD32;
    fn ixheaacd_decode_ps_data(
        ptr_ps_dec: *mut ia_ps_dec_struct,
        frame_size: WORD32,
    ) -> VOID;
    fn ixheaacd_sbr_read_sce(
        ptr_header_data: *mut ia_sbr_header_data_struct,
        ptr_frame_data: *mut ia_sbr_frame_info_data_struct,
        ptr_ps_dec: *mut ia_ps_dec_struct,
        it_bit_buff: *mut ia_bit_buf_struct,
        ptr_sbr_tables: *mut ia_sbr_tables_struct,
        audio_object_type: WORD,
        ec_flag: WORD32,
    ) -> IA_ERRORCODE;
    fn ixheaacd_sbr_read_cpe(
        ptr_header_data: *mut ia_sbr_header_data_struct,
        ptr_frame_data: *mut *mut ia_sbr_frame_info_data_struct,
        itt_bit_buf: *mut ia_bit_buf_struct,
        ptr_sbr_tables: *mut ia_sbr_tables_struct,
        audio_object_type: WORD,
    ) -> IA_ERRORCODE;
    fn ixheaacd_sbr_read_header_data(
        ptr_sbr_header: *mut ia_sbr_header_data_struct,
        it_bit_buf: *mut ia_bit_buf_struct,
        stereo_flag: FLAG,
        ptr_sbr_dflt_header: *mut ia_sbr_header_data_struct,
    ) -> WORD32;
    fn ixheaacd_calc_frq_bnd_tbls(
        ptr_header_data: *mut ia_sbr_header_data_struct,
        pstr_common_tables: *mut ixheaacd_misc_tables,
    ) -> WORD32;
    fn ixheaacd_reset_sbrenvelope_calc(
        ptr_calc_env: *mut ia_sbr_calc_env_struct,
    ) -> VOID;
    fn ixheaacd_derive_lim_band_tbl(
        ptr_header_data: *mut ia_sbr_header_data_struct,
        p_str_patch_param: *const ia_patch_param_struct,
        num_patches: WORD16,
        pstr_common_tables: *mut ixheaacd_misc_tables,
    ) -> VOID;
    fn ixheaacd_reset_hf_generator(
        hf_generator: *mut ia_sbr_hf_generator_struct,
        ptr_header_data: *mut ia_sbr_header_data_struct,
        audio_obj_type: WORD,
    ) -> WORD32;
    fn ixheaacd_sbr_dec(
        ptr_sbr_dec: *mut ia_sbr_dec_struct,
        ptr_time_data: *mut WORD16,
        ptr_header_data: *mut ia_sbr_header_data_struct,
        ptr_frame_data: *mut ia_sbr_frame_info_data_struct,
        ptr_frame_data_prev: *mut ia_sbr_prev_frame_data_struct,
        ptr_ps_dec: *mut ia_ps_dec_struct,
        ptr_qmf_synth_bank_r: *mut ia_sbr_qmf_filter_bank_struct,
        ptr_sbr_sf_r: *mut ia_sbr_scale_fact_struct,
        apply_processing: FLAG,
        low_pow_flag: FLAG,
        ptr_work_buf_core: *mut WORD32,
        sbr_tables_ptr: *mut ia_sbr_tables_struct,
        pstr_common_tables: *mut ixheaacd_misc_tables,
        ch_fac: WORD,
        ptr_pvc_data_str: *mut ia_pvc_data_struct,
        drc_on: FLAG,
        drc_sbr_factors: *mut [WORD32; 64],
        audio_object_type: WORD32,
        ldmps_present: WORD32,
        self_0: *mut core::ffi::c_void,
        heaac_mps_present: WORD32,
        ec_flag: WORD32,
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
    fn ixheaacd_qmf_hbe_data_reinit(
        ptr_hbe_transposer_str: *mut ia_esbr_hbe_txposer_struct,
        ptr_freq_band_tbl: *mut *mut WORD16,
        ptr_num_sf_bands: *mut WORD16,
        upsamp_4_flag: WORD32,
    ) -> WORD32;
    fn ixheaacd_dft_hbe_data_reinit(
        ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
        p_freq_band_tab: *mut *mut WORD16,
        p_num_sfb: *mut WORD16,
    ) -> WORD32;
    fn ixheaacd_sbr_read_pvc_sce(
        ptr_frame_data: *mut ia_sbr_frame_info_data_struct,
        it_bit_buff: *mut ia_bit_buf_struct,
        hbe_flag: WORD32,
        ptr_pvc_data: *mut ia_pvc_data_struct,
        sbr_tables_ptr: *mut ia_sbr_tables_struct,
        ptr_header_data: *mut ia_sbr_header_data_struct,
    ) -> WORD32;
    fn ixheaacd_esbr_dec(
        ptr_sbr_dec: *mut ia_sbr_dec_struct,
        ptr_header_data: *mut ia_sbr_header_data_struct,
        ptr_frame_data: *mut ia_sbr_frame_info_data_struct,
        apply_processing: FLAG,
        low_pow_flag: FLAG,
        sbr_tables_ptr: *mut ia_sbr_tables_struct,
        ch_fac: WORD32,
    ) -> WORD32;
    fn ixheaacd_hbe_repl_spec(
        x_over_qmf: *mut WORD32,
        qmf_buf_real: *mut [FLOAT32; 64],
        qmf_buf_imag: *mut [FLOAT32; 64],
        no_bins: WORD32,
        max_stretch: WORD32,
    ) -> VOID;
    fn ixheaacd_dec_sbrdata(
        ptr_header_data_ch_0: *mut ia_sbr_header_data_struct,
        ptr_header_data_ch_1: *mut ia_sbr_header_data_struct,
        ptr_sbr_data_ch_0: *mut ia_sbr_frame_info_data_struct,
        ptr_prev_data_ch_0: *mut ia_sbr_prev_frame_data_struct,
        ptr_sbr_data_ch_1: *mut ia_sbr_frame_info_data_struct,
        ptr_prev_data_ch_1: *mut ia_sbr_prev_frame_data_struct,
        ptr_common_tables: *mut ixheaacd_misc_tables,
        ldmps_present: WORD32,
        audio_object_type: WORD32,
        ec_flag: WORD32,
    ) -> IA_ERRORCODE;
    fn ixheaacd_dec_sbrdata_for_pvc(
        ptr_header_data: *mut ia_sbr_header_data_struct,
        ptr_sbr_data: *mut ia_sbr_frame_info_data_struct,
        ptr_prev_data: *mut ia_sbr_prev_frame_data_struct,
        audio_object_type: WORD32,
    ) -> IA_ERRORCODE;
    fn ixheaacd_sbr_crccheck(
        it_bit_buff: *mut ia_bit_buf_struct,
        crc_bits_len: WORD32,
    ) -> FLAG;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ixheaac_drc_data_struct {
    pub prog_ref_level: WORD32,
    pub n_mdct_bands: [WORD16; 16],
    pub drc_fac: [WORD16; 16],
    pub drc_fac_dvb: [WORD16; 16],
    pub drc_exp: WORD8,
    pub short_block: UWORD8,
    pub drc_interp_scheme: UWORD8,
    pub n_drc_bands: UWORD8,
    pub new_prog_ref_level: UWORD8,
    pub new_drc_fac: UWORD8,
    pub prev_interp_scheme: UWORD8,
    pub drc_factors_sbr: [[WORD32; 64]; 64],
    pub drc_factors_sbr_lat: [[WORD32; 64]; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ixheaac_drc_bs_data_struct {
    pub b_channel_on: [UWORD8; 8],
    pub prog_ref_level_present: UWORD8,
    pub prog_ref_level: UWORD8,
    pub drc_num_bands: UWORD8,
    pub drc_band_top: [UWORD8; 16],
    pub dyn_rng_dlbl: [WORD8; 16],
    pub dyn_rng_dlbl_dvb: [WORD8; 16],
    pub max_dyn_rng_dlbl: WORD8,
    pub drc_interpolation_scheme: UWORD8,
    pub drc_data_type: WORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_dec_struct {
    pub str_drc_bs_data: [ixheaac_drc_bs_data_struct; 10],
    pub str_drc_channel_data: [ixheaac_drc_data_struct; 10],
    pub drc_ref_level: WORD16,
    pub drc_def_level: WORD16,
    pub drc_channel_next_index: [UWORD8; 10],
    pub sbr_allowed: UWORD8,
    pub sbr_found: UWORD8,
    pub drc_element_found: UWORD8,
    pub max_audio_channels: UWORD8,
    pub length_history: UWORD8,
    pub num_drc_elements: UWORD8,
    pub state: WORD32,
    pub target_ref_level: WORD32,
    pub prog_ref_level: WORD32,
    pub cut_factor: WORD32,
    pub boost_factor: WORD32,
    pub drc_dig_norm: FLAG,
    pub drc_on: FLAG,
    pub dvb_anc_data_present: FLAG,
    pub dvb_anc_data_pos: WORD32,
    pub pres_mode: WORD32,
    pub heavy_mode: WORD32,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_element_stream_struct {
    pub sbr_ele_id: WORD32,
    pub extension_type: WORD32,
    pub size_payload: WORD32,
    pub ptr_sbr_data: *mut WORD8,
    pub ptr_prev_sbr_data: *mut WORD8,
    pub prev_size_payload: WORD32,
    pub frame_error_flag: [WORD32; 2],
    pub use_frame_slot: WORD32,
    pub prev_sbr_ele_id: WORD32,
    pub prev_extension_type: WORD32,
    pub size_payload_old: WORD32,
    pub sbr_prev_data: [WORD8; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_sbr_bitstream_struct {
    pub no_elements: WORD16,
    pub str_sbr_ele: [ia_sbr_element_stream_struct; 1],
}
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const PVC_SBR: C2RustUnnamed_0 = 2;
pub const ORIG_SBR: C2RustUnnamed_0 = 1;
pub const UNKNOWN_SBR: C2RustUnnamed_0 = 0;
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
pub struct ia_sbr_scr_struct {
    pub ptr_work_buf_core: *mut core::ffi::c_void,
}
pub const AOT_ER_AAC_LD: AUDIO_OBJECT_TYPE = 23;
pub const AOT_ER_AAC_ELD: AUDIO_OBJECT_TYPE = 39;
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
pub const AOT_USAC: AUDIO_OBJECT_TYPE = 42;
pub type AUDIO_OBJECT_TYPE = core::ffi::c_uint;
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
pub const IA_XHEAAC_DEC_EXE_NONFATAL_SBR_PARSE_ERROR: core::ffi::c_int = 0x1816
    as core::ffi::c_int;
pub const SBR_CYC_REDCY_CHK_BITS: core::ffi::c_int = 10 as core::ffi::c_int;
pub const SBR_HF_ADJ_OFFSET: core::ffi::c_int = 2 as core::ffi::c_int;
pub const LEN_NIBBLE: core::ffi::c_int = 4 as core::ffi::c_int;
pub const FRAME_OK: core::ffi::c_int = 0 as core::ffi::c_int;
pub const FRAME_ERROR: core::ffi::c_int = 1 as core::ffi::c_int;
pub const FRAME_ERROR_ALLSLOTS: core::ffi::c_int = 2;
pub const SBR_EXTENSION_CRC: core::ffi::c_int = 14 as core::ffi::c_int;
pub const SBRDEC_OK: core::ffi::c_int = 0 as core::ffi::c_int;
pub const SBR_UPSAMPLE_FAC: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SBR_UPSAMPLE_IDX_4_1: core::ffi::c_int = 3 as core::ffi::c_int;
pub const MAX_FRAME_SIZE: core::ffi::c_int = 1024 as core::ffi::c_int;
pub const QMF_FILTER_STATE_SYN_SIZE: core::ffi::c_int = 1280 as core::ffi::c_int;
pub const QMF_FILTER_STATE_ANA_SIZE: core::ffi::c_int = 320 as core::ffi::c_int;
pub const NO_SYNTHESIS_CHANNELS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const NO_ANALYSIS_CHANNELS: core::ffi::c_int = NO_SYNTHESIS_CHANNELS
    / SBR_UPSAMPLE_FAC;
pub const QMF_FILTER_STATE_SYN_SIZE_DOWN_SAMPLED: core::ffi::c_int = QMF_FILTER_STATE_SYN_SIZE
    / 2 as core::ffi::c_int;
pub const MAX_OV_COLS: core::ffi::c_int = 6 as core::ffi::c_int;
pub const SBR_NOT_INITIALIZED: core::ffi::c_int = 0 as core::ffi::c_int;
pub const UPSAMPLING: core::ffi::c_int = 1 as core::ffi::c_int;
pub const SBR_ACTIVE: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SBR_MONO: core::ffi::c_int = 1 as core::ffi::c_int;
pub const SBR_STEREO: core::ffi::c_int = 2 as core::ffi::c_int;
pub const PS_STEREO: core::ffi::c_int = 3 as core::ffi::c_int;
pub const SBR_RESET: core::ffi::c_int = 1 as core::ffi::c_int;
pub const IA_FATAL_ERROR: core::ffi::c_uint = 0x80000000 as core::ffi::c_uint;
pub const COUPLING_OFF: core::ffi::c_int = 0 as core::ffi::c_int;
unsafe extern "C" fn ixheaacd_ec_set_frame_error_flag(
    mut pstr_sbr_element: *mut ia_sbr_element_stream_struct,
    mut value: WORD32,
) -> VOID {
    if !pstr_sbr_element.is_null() {
        match value {
            FRAME_ERROR_ALLSLOTS => {
                (*pstr_sbr_element).frame_error_flag[0 as core::ffi::c_int as usize] = FRAME_ERROR
                    as WORD32;
                (*pstr_sbr_element).frame_error_flag[1 as core::ffi::c_int as usize] = FRAME_ERROR
                    as WORD32;
            }
            _ => {
                (*pstr_sbr_element)
                    .frame_error_flag[(*pstr_sbr_element).use_frame_slot as usize] = value;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_downmix_to_monosbr(
    mut core_sample_buf: *mut WORD16,
    mut ch_fac: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut ptr1: *mut WORD16 = &mut *core_sample_buf
        .offset(0 as core::ffi::c_int as isize) as *mut WORD16;
    let mut ptr2: *mut WORD16 = &mut *core_sample_buf
        .offset(1 as core::ffi::c_int as isize) as *mut WORD16;
    i = (MAX_FRAME_SIZE - 1 as core::ffi::c_int) as WORD32;
    while i >= 0 as core::ffi::c_int {
        *ptr1 = ((*ptr1 as core::ffi::c_int >> 1 as core::ffi::c_int)
            + (*ptr2 as core::ffi::c_int >> 1 as core::ffi::c_int)) as WORD16;
        ptr1 = ptr1.offset(ch_fac as isize);
        ptr2 = ptr2.offset(ch_fac as isize);
        i -= 1;
    }
}
unsafe extern "C" fn ixheaacd_sbr_dec_reset(
    mut ptr_sbr_dec: *mut ia_sbr_dec_struct,
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut low_pow_flag: FLAG,
    mut pstr_common_tables: *mut ixheaacd_misc_tables,
    mut pitch_in_bins: WORD32,
    mut audio_object_type: WORD32,
    mut ptr_work_buf_core: *mut WORD32,
) -> WORD32 {
    let mut old_lsb: WORD32 = 0;
    let mut new_lsb: WORD32 = 0;
    let mut l: WORD32 = 0;
    let mut err: WORD32 = 0 as WORD32;
    let mut num_time_slots: WORD32 = (*ptr_sbr_dec).str_codec_qmf_bank.num_time_slots
        as WORD32;
    let mut upsample_ratio_idx: WORD32 = (*ptr_header_data).sbr_ratio_idx;
    let mut op_delay: WORD32 = 6 as WORD32 + SBR_HF_ADJ_OFFSET;
    let mut hbe_flag: WORD32 = (*ptr_header_data).hbe_flag as WORD32;
    let mut usac_flag: WORD32 = (*ptr_header_data).usac_flag;
    if (*ptr_header_data).is_usf_4 != 0 {
        op_delay = (op_delay as core::ffi::c_int + 6 as core::ffi::c_int) as WORD32;
    }
    ixheaacd_reset_sbrenvelope_calc(&mut (*ptr_sbr_dec).str_sbr_calc_env);
    new_lsb = (*(*ptr_header_data).pstr_freq_band_data).sub_band_start as WORD32;
    (*ptr_sbr_dec).str_synthesis_qmf_bank.lsb = new_lsb as WORD16;
    (*ptr_sbr_dec).str_synthesis_qmf_bank.usb = (*(*ptr_header_data).pstr_freq_band_data)
        .sub_band_end;
    old_lsb = (*ptr_sbr_dec).str_synthesis_qmf_bank.lsb as WORD32;
    (*ptr_sbr_dec).str_codec_qmf_bank.lsb = 0 as WORD16;
    (*ptr_sbr_dec).str_codec_qmf_bank.usb = old_lsb as WORD16;
    let mut plpc_filt_states_real: *mut WORD32 = &mut *(*((*ptr_sbr_dec)
        .str_hf_generator
        .lpc_filt_states_real)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize))
        .offset(old_lsb as isize) as *mut WORD32;
    let mut plpc_filt_states_real_1: *mut WORD32 = &mut *(*((*ptr_sbr_dec)
        .str_hf_generator
        .lpc_filt_states_real)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as isize))
        .offset(old_lsb as isize) as *mut WORD32;
    let mut plpc_filt_states_imag: *mut WORD32 = &mut *(*((*ptr_sbr_dec)
        .str_hf_generator
        .lpc_filt_states_imag)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize))
        .offset(old_lsb as isize) as *mut WORD32;
    let mut plpc_filt_states_imag_1: *mut WORD32 = &mut *(*((*ptr_sbr_dec)
        .str_hf_generator
        .lpc_filt_states_imag)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as isize))
        .offset(old_lsb as isize) as *mut WORD32;
    l = (new_lsb as core::ffi::c_int - old_lsb as core::ffi::c_int
        - 1 as core::ffi::c_int) as WORD32;
    while l >= 0 as core::ffi::c_int {
        let fresh0 = plpc_filt_states_real_1;
        plpc_filt_states_real_1 = plpc_filt_states_real_1.offset(1);
        *fresh0 = 0 as WORD32;
        let fresh1 = plpc_filt_states_real;
        plpc_filt_states_real = plpc_filt_states_real.offset(1);
        *fresh1 = *fresh0;
        if low_pow_flag == 0 {
            let fresh2 = plpc_filt_states_imag_1;
            plpc_filt_states_imag_1 = plpc_filt_states_imag_1.offset(1);
            *fresh2 = 0 as WORD32;
            let fresh3 = plpc_filt_states_imag;
            plpc_filt_states_imag = plpc_filt_states_imag.offset(1);
            *fresh3 = *fresh2;
        }
        l -= 1;
    }
    let mut start_band: WORD32 = 0;
    let mut stop_band: WORD32 = 0;
    let mut start_slot: WORD32 = SBR_HF_ADJ_OFFSET;
    let mut k: WORD32 = 0;
    start_band = (*(*ptr_header_data).pstr_freq_band_data).qmf_sb_prev as WORD32;
    stop_band = (*(*ptr_header_data).pstr_freq_band_data).sub_band_start as WORD32;
    if usac_flag != 0 && hbe_flag == 0 {
        l = 0 as core::ffi::c_int as WORD32;
        while l < SBR_HF_ADJ_OFFSET {
            k = start_band;
            while k < stop_band {
                (*ptr_sbr_dec).qmf_buf_real[l as usize][k as usize] = 0.0f32;
                (*ptr_sbr_dec).qmf_buf_imag[l as usize][k as usize] = 0.0f32;
                k += 1;
            }
            l += 1;
        }
        l = start_slot;
        while l < op_delay {
            k = start_band;
            while k < stop_band {
                (*ptr_sbr_dec).qmf_buf_real[l as usize][k as usize] = 0.0f32;
                (*ptr_sbr_dec).qmf_buf_imag[l as usize][k as usize] = 0.0f32;
                k += 1;
            }
            l += 1;
        }
    }
    if !((*ptr_sbr_dec).p_hbe_txposer).is_null() && (usac_flag != 0 || hbe_flag != 0) {
        let mut k_0: WORD32 = 0;
        let mut i: WORD32 = 0;
        let mut dft_hbe_flag: WORD32 = (*ptr_header_data).esbr_hq as WORD32;
        if dft_hbe_flag == 1 as core::ffi::c_int {
            err = ixheaacd_dft_hbe_data_reinit(
                (*ptr_sbr_dec).p_hbe_txposer,
                ((*(*ptr_header_data).pstr_freq_band_data).freq_band_table).as_mut_ptr(),
                ((*(*ptr_header_data).pstr_freq_band_data).num_sf_bands).as_mut_ptr(),
            );
        } else {
            err = ixheaacd_qmf_hbe_data_reinit(
                (*ptr_sbr_dec).p_hbe_txposer,
                ((*(*ptr_header_data).pstr_freq_band_data).freq_band_table).as_mut_ptr(),
                ((*(*ptr_header_data).pstr_freq_band_data).num_sf_bands).as_mut_ptr(),
                (*ptr_header_data).is_usf_4,
            );
        }
        if err != 0 {
            return err;
        }
        k_0 = 0 as core::ffi::c_int as WORD32;
        while k_0 < 2 as core::ffi::c_int {
            if !(upsample_ratio_idx == SBR_UPSAMPLE_IDX_4_1
                && k_0 == 0 as core::ffi::c_int)
            {
                let mut xpos_delay: WORD32 = num_time_slots * k_0;
                if upsample_ratio_idx == SBR_UPSAMPLE_IDX_4_1 {
                    xpos_delay = (num_time_slots as core::ffi::c_int
                        * k_0 as core::ffi::c_int - 32 as core::ffi::c_int) as WORD32;
                }
                i = 0 as core::ffi::c_int as WORD32;
                while i < 8 as core::ffi::c_int {
                    memmove(
                        ((*ptr_sbr_dec).ph_vocod_qmf_real[i as usize]).as_mut_ptr()
                            as *mut core::ffi::c_void,
                        ((*ptr_sbr_dec).ph_vocod_qmf_real[(num_time_slots + i) as usize])
                            .as_mut_ptr() as *const core::ffi::c_void,
                        (64 as size_t)
                            .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                    );
                    memmove(
                        ((*ptr_sbr_dec).ph_vocod_qmf_imag[i as usize]).as_mut_ptr()
                            as *mut core::ffi::c_void,
                        ((*ptr_sbr_dec).ph_vocod_qmf_imag[(num_time_slots + i) as usize])
                            .as_mut_ptr() as *const core::ffi::c_void,
                        (64 as size_t)
                            .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                    );
                    i += 1;
                }
                if dft_hbe_flag == 1 as core::ffi::c_int {
                    err = ixheaacd_dft_hbe_apply(
                        (*ptr_sbr_dec).p_hbe_txposer,
                        ((*ptr_sbr_dec).qmf_buf_real)
                            .as_mut_ptr()
                            .offset(op_delay as isize)
                            .offset(xpos_delay as isize),
                        ((*ptr_sbr_dec).qmf_buf_imag)
                            .as_mut_ptr()
                            .offset(op_delay as isize)
                            .offset(xpos_delay as isize),
                        num_time_slots,
                        ((*ptr_sbr_dec).ph_vocod_qmf_real)
                            .as_mut_ptr()
                            .offset(op_delay as isize),
                        ((*ptr_sbr_dec).ph_vocod_qmf_imag)
                            .as_mut_ptr()
                            .offset(op_delay as isize),
                        pitch_in_bins,
                        ptr_work_buf_core as *mut FLOAT32,
                    );
                    if err != 0 {
                        return err;
                    }
                } else {
                    err = ixheaacd_qmf_hbe_apply(
                        (*ptr_sbr_dec).p_hbe_txposer,
                        ((*ptr_sbr_dec).qmf_buf_real)
                            .as_mut_ptr()
                            .offset(op_delay as isize)
                            .offset(xpos_delay as isize),
                        ((*ptr_sbr_dec).qmf_buf_imag)
                            .as_mut_ptr()
                            .offset(op_delay as isize)
                            .offset(xpos_delay as isize),
                        num_time_slots,
                        ((*ptr_sbr_dec).ph_vocod_qmf_real)
                            .as_mut_ptr()
                            .offset(op_delay as isize),
                        ((*ptr_sbr_dec).ph_vocod_qmf_imag)
                            .as_mut_ptr()
                            .offset(op_delay as isize),
                        pitch_in_bins,
                        ptr_header_data,
                    );
                    if err != 0 {
                        return err;
                    }
                }
                if upsample_ratio_idx == SBR_UPSAMPLE_IDX_4_1 {
                    ixheaacd_hbe_repl_spec(
                        &mut *((*(*ptr_sbr_dec).p_hbe_txposer).x_over_qmf)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize),
                        ((*ptr_sbr_dec).ph_vocod_qmf_real)
                            .as_mut_ptr()
                            .offset(op_delay as isize),
                        ((*ptr_sbr_dec).ph_vocod_qmf_imag)
                            .as_mut_ptr()
                            .offset(op_delay as isize),
                        num_time_slots,
                        (*(*ptr_sbr_dec).p_hbe_txposer).max_stretch,
                    );
                }
            }
            k_0 += 1;
        }
    }
    if usac_flag == 0 {
        err
            |= ixheaacd_reset_hf_generator(
                &mut (*ptr_sbr_dec).str_hf_generator,
                ptr_header_data,
                audio_object_type as WORD,
            );
        ixheaacd_derive_lim_band_tbl(
            ptr_header_data,
            ((*(*ptr_sbr_dec).str_hf_generator.pstr_settings).str_patch_param)
                .as_mut_ptr(),
            (*(*ptr_sbr_dec).str_hf_generator.pstr_settings).num_patches,
            pstr_common_tables,
        );
    }
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_prepare_upsamp(
    mut ptr_header_data: *mut *mut ia_sbr_header_data_struct,
    mut pstr_sbr_channel: *mut *mut ia_sbr_channel_struct,
    mut num_channels: WORD32,
) -> VOID {
    let mut lr: WORD32 = 0;
    let mut sbr_qmf_bank: *mut ia_sbr_qmf_filter_bank_struct = 0
        as *mut ia_sbr_qmf_filter_bank_struct;
    lr = 0 as core::ffi::c_int as WORD32;
    while lr < num_channels {
        (*(**ptr_header_data.offset(lr as isize)).pstr_freq_band_data).sub_band_start = NO_ANALYSIS_CHANNELS
            as WORD16;
        (*(**ptr_header_data.offset(lr as isize)).pstr_freq_band_data).sub_band_end = NO_SYNTHESIS_CHANNELS
            as WORD16;
        sbr_qmf_bank = &mut (**pstr_sbr_channel.offset(lr as isize))
            .str_sbr_dec
            .str_synthesis_qmf_bank;
        (*sbr_qmf_bank).lsb = NO_ANALYSIS_CHANNELS as WORD16;
        (*sbr_qmf_bank).usb = NO_SYNTHESIS_CHANNELS as WORD16;
        sbr_qmf_bank = &mut (**pstr_sbr_channel.offset(lr as isize))
            .str_sbr_dec
            .str_codec_qmf_bank;
        (*sbr_qmf_bank).lsb = 0 as WORD16;
        (*sbr_qmf_bank).usb = NO_ANALYSIS_CHANNELS as WORD16;
        (**ptr_header_data.offset(lr as isize)).sync_state = UPSAMPLING as WORD32;
        lr += 1;
    }
}
unsafe extern "C" fn ixheaacd_copy_prev_ps_params(
    mut ps_config_curr: *mut ia_ps_dec_struct,
    mut ps_config_prev: *mut ia_ps_dec_config_struct,
    mut frame_status: WORD32,
) -> VOID {
    if frame_status == 0 as core::ffi::c_int {
        (*ps_config_curr).enable_iid = (*ps_config_prev).enable_iid;
        (*ps_config_curr).iid_mode = (*ps_config_prev).iid_mode;
        (*ps_config_curr).enable_icc = (*ps_config_prev).enable_icc;
        (*ps_config_curr).icc_mode = (*ps_config_prev).icc_mode;
        (*ps_config_curr).frame_class = (*ps_config_prev).frame_class;
        (*ps_config_curr).freq_res_ipd = (*ps_config_prev).freq_res_ipd;
        memcpy(
            ((*ps_config_curr).border_position).as_mut_ptr() as *mut core::ffi::c_void,
            ((*ps_config_prev).border_position).as_mut_ptr() as *const core::ffi::c_void,
            ::core::mem::size_of::<[WORD16; 7]>() as size_t,
        );
        memcpy(
            ((*ps_config_curr).iid_dt).as_mut_ptr() as *mut core::ffi::c_void,
            ((*ps_config_prev).iid_dt).as_mut_ptr() as *const core::ffi::c_void,
            ::core::mem::size_of::<[FLAG; 5]>() as size_t,
        );
        memcpy(
            ((*ps_config_curr).iid_par_table).as_mut_ptr() as *mut core::ffi::c_void,
            ((*ps_config_prev).iid_par_table).as_mut_ptr() as *const core::ffi::c_void,
            ::core::mem::size_of::<[[WORD16; 34]; 7]>() as size_t,
        );
        memcpy(
            ((*ps_config_curr).icc_dt).as_mut_ptr() as *mut core::ffi::c_void,
            ((*ps_config_prev).icc_dt).as_mut_ptr() as *const core::ffi::c_void,
            ::core::mem::size_of::<[FLAG; 5]>() as size_t,
        );
        memcpy(
            ((*ps_config_curr).icc_par_table).as_mut_ptr() as *mut core::ffi::c_void,
            ((*ps_config_prev).icc_par_table).as_mut_ptr() as *const core::ffi::c_void,
            ::core::mem::size_of::<[[WORD16; 34]; 7]>() as size_t,
        );
    } else {
        (*ps_config_prev).enable_iid = (*ps_config_curr).enable_iid;
        (*ps_config_prev).iid_mode = (*ps_config_curr).iid_mode;
        (*ps_config_prev).enable_icc = (*ps_config_curr).enable_icc;
        (*ps_config_prev).icc_mode = (*ps_config_curr).icc_mode;
        (*ps_config_prev).frame_class = (*ps_config_curr).frame_class;
        (*ps_config_prev).freq_res_ipd = (*ps_config_curr).freq_res_ipd;
        memcpy(
            ((*ps_config_prev).border_position).as_mut_ptr() as *mut core::ffi::c_void,
            ((*ps_config_curr).border_position).as_mut_ptr() as *const core::ffi::c_void,
            ::core::mem::size_of::<[WORD16; 7]>() as size_t,
        );
        memcpy(
            ((*ps_config_prev).iid_dt).as_mut_ptr() as *mut core::ffi::c_void,
            ((*ps_config_curr).iid_dt).as_mut_ptr() as *const core::ffi::c_void,
            ::core::mem::size_of::<[FLAG; 5]>() as size_t,
        );
        memcpy(
            ((*ps_config_prev).iid_par_table).as_mut_ptr() as *mut core::ffi::c_void,
            ((*ps_config_curr).iid_par_table).as_mut_ptr() as *const core::ffi::c_void,
            ::core::mem::size_of::<[[WORD16; 34]; 7]>() as size_t,
        );
        memcpy(
            ((*ps_config_prev).icc_dt).as_mut_ptr() as *mut core::ffi::c_void,
            ((*ps_config_curr).icc_dt).as_mut_ptr() as *const core::ffi::c_void,
            ::core::mem::size_of::<[FLAG; 5]>() as size_t,
        );
        memcpy(
            ((*ps_config_prev).icc_par_table).as_mut_ptr() as *mut core::ffi::c_void,
            ((*ps_config_curr).icc_par_table).as_mut_ptr() as *const core::ffi::c_void,
            ::core::mem::size_of::<[[WORD16; 34]; 7]>() as size_t,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_applysbr(
    mut self_0: ia_handle_sbr_dec_inst_struct,
    mut p_sbr_bit_stream: *mut ia_aac_dec_sbr_bitstream_struct,
    mut core_sample_buf: *mut WORD16,
    mut codec_num_channels: *mut WORD16,
    mut frame_status: FLAG,
    mut down_samp_flag: FLAG,
    mut down_mix_flag: FLAG,
    mut sbr_scratch_struct: *mut ia_sbr_scr_struct,
    mut ps_enable: WORD32,
    mut ch_fac: WORD32,
    mut slot_element: WORD32,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut pstr_drc_dec: *mut ia_drc_dec_struct,
    mut eld_sbr_flag: WORD,
    mut audio_object_type: WORD32,
    mut init_flag: WORD32,
    mut ldmps_present: WORD32,
    mut frame_size: WORD32,
    mut heaac_mps_present: WORD32,
    mut ec_flag: WORD32,
    mut first_frame: FLAG,
) -> IA_ERRORCODE {
    let mut k: WORD32 = 0;
    let mut prev_ps_flag: FLAG = 0 as FLAG;
    let mut ps_flag: FLAG = 0 as FLAG;
    let mut stereo: FLAG = 0 as FLAG;
    let mut low_pow_flag: FLAG = 0 as FLAG;
    let mut header_flag: FLAG = 1 as FLAG;
    let mut dual_mono: FLAG = 0 as FLAG;
    let mut err: WORD32 = 0 as WORD32;
    let mut num_channels: WORD32 = *codec_num_channels as WORD32;
    let mut prev_stereo: FLAG = 0;
    let mut ele_channels: WORD32 = 0 as WORD32;
    let mut num_elements: WORD32 = (*p_sbr_bit_stream).no_elements as WORD32;
    let mut usac_flag: WORD32 = (*self_0).aot_usac_flag as WORD32;
    let mut total_bits_left: WORD32 = 0 as WORD32;
    let mut pstr_sbr_channel: [*mut ia_sbr_channel_struct; 2] = [0
        as *mut ia_sbr_channel_struct; 2];
    let mut ptr_header_data: [*mut ia_sbr_header_data_struct; 2] = [0
        as *mut ia_sbr_header_data_struct; 2];
    let mut initial_sync_state: WORD32 = 0;
    let mut ptr_sbr_dflt_header: *mut ia_sbr_header_data_struct = &mut (*self_0)
        .str_sbr_dflt_header as *mut ia_sbr_header_data_struct;
    let mut ptr_frame_data: [*mut ia_sbr_frame_info_data_struct; 2] = [0
        as *mut ia_sbr_frame_info_data_struct; 2];
    (*self_0).num_delay_frames = 1 as core::ffi::c_int as WORD32;
    (*self_0).ptr_mps_data = 0 as *mut WORD8;
    if ec_flag == 0 || usac_flag == 0 {
        k = 0 as core::ffi::c_int as WORD32;
        while k < 2 as core::ffi::c_int {
            ptr_frame_data[k as usize] = (*self_0).frame_buffer[k as usize]
                as *mut ia_sbr_frame_info_data_struct;
            pstr_sbr_channel[k as usize] = (*self_0).pstr_sbr_channel[k as usize];
            ptr_header_data[k as usize] = (*self_0).pstr_sbr_header[k as usize];
            if audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int {
                (*ptr_frame_data[k as usize]).eld_sbr_flag = eld_sbr_flag as WORD32;
                (*ptr_frame_data[k as usize]).num_time_slots = (*ptr_header_data[0
                    as core::ffi::c_int as usize])
                    .num_time_slots as WORD32;
            }
            (*ptr_frame_data[k as usize]).usac_independency_flag = (*self_0)
                .usac_independency_flag;
            (*ptr_frame_data[k as usize]).mps_sbr_flag = (if (*self_0).stereo_config_idx
                == 3 as core::ffi::c_int
            {
                1 as core::ffi::c_int
            } else {
                0 as core::ffi::c_int
            }) as FLAG;
            (*ptr_frame_data[k as usize]).stereo_config_idx = (*self_0).stereo_config_idx
                as WORD32;
            (*ptr_frame_data[k as usize]).inter_tes_flag = (*self_0).inter_tes_flag;
            (*ptr_frame_data[k as usize]).sbr_mode = (*self_0).sbr_mode;
            if usac_flag == 0 {
                (*ptr_frame_data[k as usize]).usac_independency_flag = 0
                    as core::ffi::c_int as FLAG;
                (*ptr_frame_data[k as usize]).mps_sbr_flag = 0 as core::ffi::c_int
                    as FLAG;
                (*ptr_frame_data[k as usize]).stereo_config_idx = -(1
                    as core::ffi::c_int) as WORD32;
                (*ptr_frame_data[k as usize]).inter_tes_flag = 0 as core::ffi::c_int
                    as FLAG;
                (*ptr_frame_data[k as usize]).sbr_mode = ORIG_SBR as core::ffi::c_int
                    as FLAG;
            }
            k += 1;
        }
        if init_flag != 0 {
            (*ptr_frame_data[1 as core::ffi::c_int as usize]).reset_flag = 1
                as core::ffi::c_int as FLAG;
            (*ptr_frame_data[0 as core::ffi::c_int as usize]).reset_flag = 1
                as core::ffi::c_int as FLAG;
        }
        k = 0 as core::ffi::c_int as WORD32;
        while k < *codec_num_channels as core::ffi::c_int {
            (*ptr_header_data[k as usize]).usac_flag = (*self_0).aot_usac_flag as WORD32;
            (*ptr_header_data[k as usize]).enh_sbr = (*self_0).enh_sbr;
            (*ptr_header_data[k as usize]).enh_sbr_ps = ((*self_0).enh_sbr_ps
                as core::ffi::c_int
                | ((*ptr_header_data[k as usize]).channel_mode == PS_STEREO)
                    as core::ffi::c_int) as FLAG;
            (*ptr_header_data[k as usize]).usac_independency_flag = (*self_0)
                .usac_independency_flag as WORD32;
            (*ptr_header_data[k as usize]).hbe_flag = (*self_0).hbe_flag;
            (*ptr_header_data[k as usize]).pvc_flag = (*self_0).pvc_flag;
            if usac_flag == 0 {
                (*ptr_header_data[k as usize]).usac_independency_flag = 0
                    as core::ffi::c_int as WORD32;
                (*ptr_header_data[k as usize]).hbe_flag = 0 as core::ffi::c_int as FLAG;
                (*ptr_header_data[k as usize]).pvc_flag = 0 as core::ffi::c_int as FLAG;
            }
            if (*self_0).enh_sbr != 0 {
                (*ptr_header_data[k as usize]).esbr_hq = (*self_0).esbr_hq;
            }
            if usac_flag == 0
                && !(audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int
                    || audio_object_type == AOT_ER_AAC_LD as core::ffi::c_int)
                && (*self_0).enh_sbr != 0
            {
                (*ptr_header_data[k as usize]).hbe_flag = 1 as core::ffi::c_int as FLAG;
            }
            k += 1;
        }
        initial_sync_state = (*ptr_header_data[0 as core::ffi::c_int as usize])
            .sync_state;
        low_pow_flag = (usac_flag == 0) as core::ffi::c_int as FLAG;
        (*(*self_0).pstr_sbr_tables).sbr_rand_ph = ((*(*(*self_0).pstr_sbr_tables)
            .env_calc_tables_ptr)
            .sbr_rand_ph)
            .as_mut_ptr();
        if ps_enable != 0 {
            if num_channels == 1 as core::ffi::c_int {
                low_pow_flag = 0 as core::ffi::c_int as FLAG;
            }
        }
        if audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int
            || ps_enable != 0 && heaac_mps_present != 0
        {
            low_pow_flag = 0 as core::ffi::c_int as FLAG;
        }
        prev_stereo = ((*ptr_header_data[0 as core::ffi::c_int as usize]).channel_mode
            == SBR_STEREO) as core::ffi::c_int as FLAG;
        if ps_enable != 0 {
            prev_ps_flag = ((*ptr_header_data[0 as core::ffi::c_int as usize])
                .channel_mode == PS_STEREO) as core::ffi::c_int as FLAG;
        }
        (*ptr_header_data[0 as core::ffi::c_int as usize]).err_flag_prev = (*ptr_header_data[0
            as core::ffi::c_int as usize])
            .err_flag;
        if (*p_sbr_bit_stream).no_elements as core::ffi::c_int == 0 as core::ffi::c_int {
            frame_status = 0 as core::ffi::c_int as FLAG;
            (*ptr_header_data[0 as core::ffi::c_int as usize]).sync_state = UPSAMPLING
                as WORD32;
            if num_channels == 2 as core::ffi::c_int {
                (*ptr_header_data[1 as core::ffi::c_int as usize]).sync_state = UPSAMPLING
                    as WORD32;
            }
        }
        if usac_flag != 0 {
            if (*p_sbr_bit_stream).no_elements as core::ffi::c_int != 0
                && (*((*p_sbr_bit_stream).str_sbr_ele).as_mut_ptr()).size_payload
                    > 0 as core::ffi::c_int
            {
                num_elements = (*p_sbr_bit_stream).no_elements as WORD32;
            } else {
                num_elements = 0 as core::ffi::c_int as WORD32;
            }
        }
        let mut current_block_260: u64;
        k = 0 as core::ffi::c_int as WORD32;
        while k < num_elements {
            let mut local_bit_buf: ia_bit_buf_struct = {
                let mut init = ia_bit_buf_struct {
                    ptr_bit_buf_base: 0 as *mut UWORD8,
                    ptr_bit_buf_end: 0 as *mut UWORD8,
                    ptr_read_next: 0 as *mut UWORD8,
                    bit_pos: 0,
                    cnt_bits: 0,
                    size: 0,
                    adts_header_present: 0,
                    crc_check: 0,
                    protection_absent: 0,
                    no_raw_data_blocks: 0,
                    str_adts_crc_info: ia_adts_crc_info_struct {
                        crc_active: 0,
                        no_reg: 0,
                        file_value: 0,
                        crc_lookup: [0; 256],
                        str_crc_reg_data: [ia_crc_reg_data_struct {
                            active: 0,
                            buf_size: 0,
                            max_bits: 0,
                            bit_cnt: 0,
                            bit_buf_cnt: 0,
                            str_bit_buf: ia_crc_bit_buf_struct {
                                ptr_bit_buf_base: 0 as *mut UWORD8,
                                ptr_bit_buf_end: 0 as *mut UWORD8,
                                ptr_read_next: 0 as *mut UWORD8,
                                bit_pos: 0,
                                cnt_bits: 0,
                                size: 0,
                            },
                        }; 7],
                    },
                    pstr_adts_crc_info: 0 as *mut ia_adts_crc_info_struct,
                    initial_cnt_bits: 0,
                    audio_mux_align: 0,
                    bit_count: 0,
                    valid_bits: 0,
                    byte: 0,
                    byte_ptr: 0 as *mut UWORD8,
                    ptr_start: 0 as *mut UWORD8,
                    write_bit_count: 0,
                    max_size: 0,
                    xaac_jmp_buf: 0 as *mut jmp_buf,
                };
                init
            };
            let mut ptr_bit_str_ele: *mut ia_sbr_element_stream_struct = &mut *((*p_sbr_bit_stream)
                .str_sbr_ele)
                .as_mut_ptr()
                .offset(k as isize) as *mut ia_sbr_element_stream_struct;
            ele_channels = (if (*((*p_sbr_bit_stream).str_sbr_ele)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .sbr_ele_id == SBR_ID_CPE as core::ffi::c_int
            {
                2 as core::ffi::c_int
            } else {
                1 as core::ffi::c_int
            }) as WORD32;
            if frame_status == 0
                && (*ptr_header_data[k as usize]).sync_state == SBR_ACTIVE
                && ec_flag != 0
            {
                ixheaacd_ec_set_frame_error_flag(ptr_bit_str_ele, FRAME_ERROR_ALLSLOTS);
            }
            match (*ptr_bit_str_ele).sbr_ele_id {
                0 | 2 => {
                    if num_channels == 2 as core::ffi::c_int {
                        dual_mono = 1 as core::ffi::c_int as FLAG;
                    }
                    stereo = 0 as core::ffi::c_int as FLAG;
                }
                1 => {
                    stereo = 1 as core::ffi::c_int as FLAG;
                    ptr_header_data[1 as core::ffi::c_int as usize] = ptr_header_data[0
                        as core::ffi::c_int as usize];
                    memcpy(
                        (*self_0).pstr_sbr_header[1 as core::ffi::c_int as usize]
                            as *mut core::ffi::c_void,
                        (*self_0).pstr_sbr_header[0 as core::ffi::c_int as usize]
                            as *const core::ffi::c_void,
                        ::core::mem::size_of::<ia_sbr_header_data_struct>() as size_t,
                    );
                }
                _ => {
                    frame_status = 0 as core::ffi::c_int as FLAG;
                }
            }
            if frame_status != 0 {
                if usac_flag == 0 {
                    if !(audio_object_type == AOT_ER_AAC_LD as core::ffi::c_int
                        || audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int)
                        && (*self_0).enh_sbr != 0
                    {
                        let mut tmp: [WORD8; 1024] = [0; 1024];
                        let mut tmp_payload: WORD32 = 0;
                        memcpy(
                            &mut *tmp.as_mut_ptr().offset(0 as core::ffi::c_int as isize)
                                as *mut WORD8 as *mut core::ffi::c_void,
                            (*ptr_bit_str_ele).ptr_sbr_data as *const core::ffi::c_void,
                            (*ptr_bit_str_ele).size_payload as size_t,
                        );
                        memcpy(
                            (*ptr_bit_str_ele).ptr_sbr_data as *mut core::ffi::c_void,
                            (*ptr_bit_str_ele).ptr_prev_sbr_data
                                as *const core::ffi::c_void,
                            (*ptr_bit_str_ele).prev_size_payload as size_t,
                        );
                        memcpy(
                            (*ptr_bit_str_ele).ptr_prev_sbr_data
                                as *mut core::ffi::c_void,
                            &mut *tmp.as_mut_ptr().offset(0 as core::ffi::c_int as isize)
                                as *mut WORD8 as *const core::ffi::c_void,
                            (*ptr_bit_str_ele).size_payload as size_t,
                        );
                        tmp_payload = (*ptr_bit_str_ele).size_payload;
                        (*ptr_bit_str_ele).size_payload = (*ptr_bit_str_ele)
                            .prev_size_payload;
                        (*ptr_bit_str_ele).prev_size_payload = tmp_payload;
                    }
                    if (*ptr_bit_str_ele).size_payload == 0 {
                        current_block_260 = 317151059986244064;
                    } else {
                        ixheaacd_create_init_bit_buf(
                            &mut local_bit_buf,
                            (*ptr_bit_str_ele).ptr_sbr_data as *mut UWORD8,
                            (*ptr_bit_str_ele).size_payload,
                        );
                        it_bit_buff = &mut local_bit_buf as *mut ia_bit_buf_struct;
                        (*it_bit_buff).xaac_jmp_buf = (*self_0).xaac_jmp_buf;
                        total_bits_left = (*it_bit_buff).cnt_bits;
                        if audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int {
                            if eld_sbr_flag != 1 as core::ffi::c_int {
                                ixheaacd_read_bits_buf(&mut local_bit_buf, LEN_NIBBLE);
                            }
                        } else {
                            ixheaacd_read_bits_buf(&mut local_bit_buf, LEN_NIBBLE);
                        }
                        current_block_260 = 9500030526577190060;
                    }
                } else {
                    current_block_260 = 9500030526577190060;
                }
                match current_block_260 {
                    317151059986244064 => {}
                    _ => {
                        if (*ptr_bit_str_ele).extension_type == SBR_EXTENSION_CRC {
                            let mut crc_bits: WORD32 = 0 as WORD32;
                            let mut crc_check_flag: WORD32 = 0 as WORD32;
                            crc_check_flag = 1 as core::ffi::c_int as WORD32;
                            crc_bits = ((((*ptr_bit_str_ele).size_payload
                                as core::ffi::c_int - 1 as core::ffi::c_int)
                                << 3 as core::ffi::c_int)
                                + (4 as core::ffi::c_int - SBR_CYC_REDCY_CHK_BITS))
                                as WORD32;
                            if crc_bits < 0 as core::ffi::c_int {
                                crc_check_flag = 0 as core::ffi::c_int as WORD32;
                                frame_status = 0 as core::ffi::c_int as FLAG;
                            }
                            if crc_check_flag != 0 {
                                frame_status = ixheaacd_sbr_crccheck(it_bit_buff, crc_bits);
                            }
                        }
                        if usac_flag == 0 {
                            header_flag = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD)
                                as FLAG;
                        }
                        if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int {
                            if header_flag != 0 {
                                header_flag = ixheaacd_sbr_read_header_data(
                                    ptr_header_data[k as usize],
                                    it_bit_buff,
                                    stereo,
                                    ptr_sbr_dflt_header,
                                ) as FLAG;
                                if usac_flag != 0 {
                                    if (*(*self_0).ptr_pvc_data_str).prev_pvc_mode
                                        as core::ffi::c_int == 0 as core::ffi::c_int
                                        && (*ptr_header_data[k as usize]).pvc_mode
                                            as core::ffi::c_int != 0 as core::ffi::c_int
                                    {
                                        (*(*self_0).ptr_pvc_data_str).prev_pvc_id = 0 as UWORD16;
                                    }
                                    (*(*self_0).ptr_pvc_data_str).prev_pvc_mode = (*ptr_header_data[k
                                        as usize])
                                        .pvc_mode;
                                    if (*ptr_header_data[k as usize]).pvc_mode
                                        as core::ffi::c_int == 0 as core::ffi::c_int
                                    {
                                        (*ptr_frame_data[k as usize]).sbr_mode = ORIG_SBR
                                            as core::ffi::c_int as FLAG;
                                    } else {
                                        (*ptr_frame_data[k as usize]).sbr_mode = PVC_SBR
                                            as core::ffi::c_int as FLAG;
                                    }
                                }
                                if header_flag == SBR_RESET {
                                    err = ixheaacd_calc_frq_bnd_tbls(
                                        ptr_header_data[k as usize],
                                        (*self_0).pstr_common_tables,
                                    );
                                    if err == 0 {
                                        let mut lr: WORD32 = 0;
                                        let mut lr1: WORD32 = if ps_enable != 0 {
                                            2 as WORD32
                                        } else {
                                            num_channels
                                        };
                                        lr = 0 as core::ffi::c_int as WORD32;
                                        while lr < lr1 {
                                            if ldmps_present != 1 as core::ffi::c_int {
                                                (*ptr_frame_data[lr as usize]).reset_flag = 1
                                                    as core::ffi::c_int as FLAG;
                                            }
                                            if SBR_NOT_INITIALIZED
                                                == (*ptr_header_data[lr as usize]).sync_state
                                                && usac_flag == 0
                                            {
                                                (*ptr_frame_data[lr as usize]).sbr_patching_mode = 1
                                                    as core::ffi::c_int as WORD32;
                                                (*ptr_frame_data[lr as usize]).over_sampling_flag = 0
                                                    as core::ffi::c_int as WORD32;
                                                (*ptr_frame_data[lr as usize]).pitch_in_bins = 0
                                                    as core::ffi::c_int as WORD32;
                                                (*ptr_header_data[lr as usize]).pre_proc_flag = 0 as WORD16;
                                            }
                                            err
                                                |= ixheaacd_sbr_dec_reset(
                                                    &mut (**pstr_sbr_channel.as_mut_ptr().offset(lr as isize))
                                                        .str_sbr_dec,
                                                    ptr_header_data[k as usize],
                                                    low_pow_flag,
                                                    (*self_0).pstr_common_tables,
                                                    (*ptr_frame_data[k as usize]).pitch_in_bins,
                                                    audio_object_type,
                                                    (*sbr_scratch_struct).ptr_work_buf_core as *mut WORD32,
                                                );
                                            if err < 0 as core::ffi::c_int {
                                                return err as IA_ERRORCODE;
                                            }
                                            lr += 1;
                                        }
                                    }
                                    if err == 0 as core::ffi::c_int {
                                        (*ptr_header_data[k as usize]).sync_state = SBR_ACTIVE
                                            as WORD32;
                                    }
                                }
                            }
                        } else {
                            if header_flag != 0 {
                                header_flag = ixheaacd_sbr_read_header_data(
                                    ptr_header_data[k as usize],
                                    it_bit_buff,
                                    stereo,
                                    ptr_sbr_dflt_header,
                                ) as FLAG;
                                if usac_flag != 0 {
                                    if (*(*self_0).ptr_pvc_data_str).prev_pvc_mode
                                        as core::ffi::c_int == 0 as core::ffi::c_int
                                        && (*ptr_header_data[k as usize]).pvc_mode
                                            as core::ffi::c_int != 0 as core::ffi::c_int
                                    {
                                        (*(*self_0).ptr_pvc_data_str).prev_pvc_id = 0 as UWORD16;
                                    }
                                    (*(*self_0).ptr_pvc_data_str).prev_pvc_mode = (*ptr_header_data[k
                                        as usize])
                                        .pvc_mode;
                                    if (*ptr_header_data[k as usize]).pvc_mode
                                        as core::ffi::c_int == 0 as core::ffi::c_int
                                    {
                                        (*ptr_frame_data[k as usize]).sbr_mode = ORIG_SBR
                                            as core::ffi::c_int as FLAG;
                                    } else {
                                        (*ptr_frame_data[k as usize]).sbr_mode = PVC_SBR
                                            as core::ffi::c_int as FLAG;
                                    }
                                }
                                if header_flag == SBR_RESET {
                                    err = ixheaacd_calc_frq_bnd_tbls(
                                        ptr_header_data[k as usize],
                                        (*self_0).pstr_common_tables,
                                    );
                                    if err != 0 {
                                        return err as IA_ERRORCODE;
                                    }
                                }
                            }
                            let mut lr_0: WORD32 = 0;
                            let mut lr1_0: WORD32 = if ps_enable != 0 {
                                2 as WORD32
                            } else {
                                num_channels
                            };
                            lr_0 = 0 as core::ffi::c_int as WORD32;
                            while lr_0 < lr1_0 {
                                if ldmps_present != 1 as core::ffi::c_int {
                                    (*ptr_frame_data[lr_0 as usize]).reset_flag = 1
                                        as core::ffi::c_int as FLAG;
                                }
                                if (*ptr_header_data[k as usize]).status != 0 {
                                    err
                                        |= ixheaacd_sbr_dec_reset(
                                            &mut (**pstr_sbr_channel.as_mut_ptr().offset(lr_0 as isize))
                                                .str_sbr_dec,
                                            ptr_header_data[k as usize],
                                            low_pow_flag,
                                            (*self_0).pstr_common_tables,
                                            (*ptr_frame_data[k as usize]).pitch_in_bins,
                                            audio_object_type,
                                            (*sbr_scratch_struct).ptr_work_buf_core as *mut WORD32,
                                        );
                                    if err < 0 as core::ffi::c_int {
                                        return err as IA_ERRORCODE;
                                    }
                                }
                                lr_0 += 1;
                            }
                            (*ptr_header_data[k as usize]).status = 0 as core::ffi::c_int
                                as WORD32;
                            if err == 0 as core::ffi::c_int {
                                (*ptr_header_data[k as usize]).sync_state = SBR_ACTIVE
                                    as WORD32;
                            }
                        }
                        current_block_260 = 6644752249785531703;
                    }
                }
            } else {
                current_block_260 = 6644752249785531703;
            }
            match current_block_260 {
                6644752249785531703 => {
                    if err != 0
                        || (*ptr_header_data[k as usize]).sync_state
                            == SBR_NOT_INITIALIZED
                    {
                        let mut lr1_1: WORD32 = if ps_enable != 0 {
                            2 as WORD32
                        } else {
                            num_channels
                        };
                        ixheaacd_prepare_upsamp(
                            ptr_header_data.as_mut_ptr(),
                            pstr_sbr_channel.as_mut_ptr(),
                            lr1_1,
                        );
                        if err != 0 {
                            return err as IA_ERRORCODE;
                        }
                    }
                    if frame_status != 0
                        && (*ptr_header_data[k as usize]).sync_state == SBR_ACTIVE
                    {
                        if stereo != 0 {
                            frame_status = ixheaacd_sbr_read_cpe(
                                ptr_header_data[0 as core::ffi::c_int as usize],
                                ptr_frame_data.as_mut_ptr(),
                                it_bit_buff,
                                (*self_0).pstr_sbr_tables,
                                audio_object_type as WORD,
                            ) as FLAG;
                            if usac_flag != 0 && frame_status == 0 as core::ffi::c_int {
                                return -(1 as IA_ERRORCODE);
                            }
                            if frame_status < 0 as core::ffi::c_int {
                                return frame_status as IA_ERRORCODE;
                            }
                        } else {
                            if ps_enable != 0 {
                                if down_mix_flag != 0 {
                                    (*(*self_0).pstr_ps_stereo_dec).force_mono = 1
                                        as core::ffi::c_int as FLAG;
                                } else {
                                    (*(*self_0).pstr_ps_stereo_dec).force_mono = 0
                                        as core::ffi::c_int as FLAG;
                                }
                            } else {
                                (*self_0).pstr_ps_stereo_dec = 0 as *mut ia_ps_dec_struct;
                            }
                            if (*ptr_frame_data[k as usize]).sbr_mode
                                == ORIG_SBR as core::ffi::c_int
                            {
                                frame_status = ixheaacd_sbr_read_sce(
                                    ptr_header_data[k as usize],
                                    ptr_frame_data[k as usize],
                                    (*self_0).pstr_ps_stereo_dec,
                                    it_bit_buff,
                                    (*self_0).pstr_sbr_tables,
                                    audio_object_type as WORD,
                                    ec_flag,
                                ) as FLAG;
                                if usac_flag != 0 && frame_status == 0 as core::ffi::c_int {
                                    return -(1 as IA_ERRORCODE);
                                }
                                if frame_status < 0 as core::ffi::c_int {
                                    return frame_status as IA_ERRORCODE;
                                }
                                if ec_flag != 0 && !((*self_0).pstr_ps_stereo_dec).is_null()
                                {
                                    ixheaacd_copy_prev_ps_params(
                                        (*self_0).pstr_ps_stereo_dec,
                                        &mut (*self_0).str_ps_config_prev,
                                        frame_status as WORD32,
                                    );
                                }
                            } else if (*ptr_frame_data[k as usize]).sbr_mode
                                == PVC_SBR as core::ffi::c_int
                            {
                                frame_status = ixheaacd_sbr_read_pvc_sce(
                                    ptr_frame_data[k as usize],
                                    it_bit_buff,
                                    (*ptr_header_data[k as usize]).hbe_flag as WORD32,
                                    (*self_0).ptr_pvc_data_str,
                                    (*self_0).pstr_sbr_tables,
                                    ptr_header_data[k as usize],
                                ) as FLAG;
                                if frame_status < 0 as core::ffi::c_int {
                                    return frame_status as IA_ERRORCODE;
                                }
                            }
                        }
                        (*ptr_header_data[k as usize]).enh_sbr_ps = ((*self_0).enh_sbr_ps
                            as core::ffi::c_int
                            | ((*ptr_header_data[0 as core::ffi::c_int as usize])
                                .channel_mode == PS_STEREO) as core::ffi::c_int) as FLAG;
                        if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
                            && audio_object_type != AOT_USAC as core::ffi::c_int
                        {
                            let mut total_bits_read: WORD32 = 0;
                            total_bits_read = (*it_bit_buff).size
                                - (*it_bit_buff).cnt_bits;
                            if total_bits_read
                                > (*ptr_bit_str_ele).size_payload << 3 as core::ffi::c_int
                                || total_bits_read
                                    < (((*ptr_bit_str_ele).size_payload as core::ffi::c_int)
                                        << 3 as core::ffi::c_int) - 8 as core::ffi::c_int
                            {
                                frame_status = 0 as core::ffi::c_int as FLAG;
                            }
                            if ec_flag != 0 {
                                if frame_status == 0 {
                                    ixheaacd_ec_set_frame_error_flag(
                                        ptr_bit_str_ele,
                                        FRAME_ERROR,
                                    );
                                } else {
                                    ixheaacd_ec_set_frame_error_flag(ptr_bit_str_ele, FRAME_OK);
                                }
                            }
                        }
                    }
                    if ldmps_present == 1 as core::ffi::c_int && !it_bit_buff.is_null() {
                        let mut bits_decoded: WORD32 = (*it_bit_buff).size
                            - (*it_bit_buff).cnt_bits;
                        (*self_0).ptr_mps_data = (*it_bit_buff).ptr_read_next
                            as *mut WORD8;
                        (*self_0).left_mps_bits = total_bits_left - bits_decoded;
                        (*self_0).mps_bits_pos = (*it_bit_buff).bit_pos;
                    }
                    if ec_flag != 0 {
                        if frame_status != 0 && init_flag == 0 {
                            (*ptr_bit_str_ele).use_frame_slot = (((*ptr_bit_str_ele)
                                .use_frame_slot as core::ffi::c_int + 1 as core::ffi::c_int)
                                % ((*self_0).num_delay_frames as core::ffi::c_int
                                    + 1 as core::ffi::c_int)) as WORD32;
                        }
                        if (*ptr_header_data[0 as core::ffi::c_int as usize]).sync_state
                            == SBR_ACTIVE
                        {
                            (*ptr_header_data[k as usize]).err_flag = (*ptr_bit_str_ele)
                                .frame_error_flag[(*ptr_bit_str_ele).use_frame_slot
                                as usize] as FLAG;
                        }
                    }
                }
                _ => {}
            }
            k += 1;
        }
        if usac_flag == 0 {
            if frame_status == 0
                || (*ptr_header_data[0 as core::ffi::c_int as usize]).sync_state
                    != SBR_ACTIVE
                || (*ptr_header_data[0 as core::ffi::c_int as usize]).err_flag != 0
            {
                (*ptr_header_data[0 as core::ffi::c_int as usize]).err_flag = 1
                    as core::ffi::c_int as FLAG;
                stereo = (if num_channels == 2 as core::ffi::c_int {
                    1 as core::ffi::c_int
                } else {
                    0 as core::ffi::c_int
                }) as FLAG;
                if (*ptr_header_data[0 as core::ffi::c_int as usize]).channel_mode
                    == 0 as core::ffi::c_int
                {
                    (*ptr_header_data[0 as core::ffi::c_int as usize]).channel_mode = (if stereo
                        != 0
                    {
                        SBR_STEREO
                    } else {
                        SBR_MONO
                    }) as WORD32;
                }
            }
            if !(stereo != 0 || dual_mono != 0) {
                (*ptr_frame_data[0 as core::ffi::c_int as usize]).coupling_mode = COUPLING_OFF
                    as WORD32;
                (*ptr_frame_data[1 as core::ffi::c_int as usize]).coupling_mode = COUPLING_OFF
                    as WORD32;
            }
            if (*ptr_header_data[0 as core::ffi::c_int as usize]).sync_state
                == SBR_NOT_INITIALIZED && (ec_flag == 0 || init_flag != 0)
            {
                let mut lr1_2: WORD32 = if ps_enable != 0 {
                    2 as WORD32
                } else {
                    num_channels
                };
                ixheaacd_prepare_upsamp(
                    ptr_header_data.as_mut_ptr(),
                    pstr_sbr_channel.as_mut_ptr(),
                    lr1_2,
                );
            }
        }
        if (*ptr_header_data[0 as core::ffi::c_int as usize]).sync_state == SBR_ACTIVE {
            if (*ptr_frame_data[0 as core::ffi::c_int as usize]).sbr_mode
                == PVC_SBR as core::ffi::c_int
            {
                err = ixheaacd_dec_sbrdata_for_pvc(
                    ptr_header_data[0 as core::ffi::c_int as usize],
                    ptr_frame_data[0 as core::ffi::c_int as usize],
                    (*pstr_sbr_channel[0 as core::ffi::c_int as usize])
                        .pstr_prev_frame_data,
                    audio_object_type,
                ) as WORD32;
                if err != 0 {
                    return err as IA_ERRORCODE;
                }
            } else if (*ptr_frame_data[0 as core::ffi::c_int as usize]).sbr_mode
                == ORIG_SBR as core::ffi::c_int
            {
                err = ixheaacd_dec_sbrdata(
                    ptr_header_data[0 as core::ffi::c_int as usize],
                    ptr_header_data[1 as core::ffi::c_int as usize],
                    ptr_frame_data[0 as core::ffi::c_int as usize],
                    (*pstr_sbr_channel[0 as core::ffi::c_int as usize])
                        .pstr_prev_frame_data,
                    if stereo != 0 || dual_mono != 0 {
                        ptr_frame_data[1 as core::ffi::c_int as usize]
                    } else {
                        0 as *mut ia_sbr_frame_info_data_struct
                    },
                    if stereo != 0 || dual_mono != 0 {
                        (*pstr_sbr_channel[1 as core::ffi::c_int as usize])
                            .pstr_prev_frame_data
                    } else {
                        0 as *mut ia_sbr_prev_frame_data_struct
                    },
                    (*self_0).pstr_common_tables,
                    ldmps_present,
                    audio_object_type,
                    ec_flag,
                ) as WORD32;
                if err != 0 {
                    return err as IA_ERRORCODE;
                }
            }
            if (*ptr_header_data[0 as core::ffi::c_int as usize]).channel_mode
                == PS_STEREO
                && (audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
                    && audio_object_type != AOT_ER_AAC_LD as core::ffi::c_int)
            {
                ixheaacd_decode_ps_data((*self_0).pstr_ps_stereo_dec, frame_size);
                ps_flag = 1 as core::ffi::c_int as FLAG;
                (*self_0).ps_present = ps_flag;
            }
            if (*ptr_header_data[0 as core::ffi::c_int as usize]).enh_sbr_ps != 0
                && (*self_0).enh_sbr != 0
            {
                ps_flag = 1 as core::ffi::c_int as FLAG;
                (*self_0).ps_present = ps_flag;
            }
            (*ptr_frame_data[0 as core::ffi::c_int as usize]).max_qmf_subband_aac = (*(*ptr_header_data[0
                as core::ffi::c_int as usize])
                .pstr_freq_band_data)
                .sub_band_start as WORD32;
            if stereo != 0 {
                (*ptr_frame_data[1 as core::ffi::c_int as usize]).max_qmf_subband_aac = (*(*ptr_header_data[1
                    as core::ffi::c_int as usize])
                    .pstr_freq_band_data)
                    .sub_band_start as WORD32;
            }
            if ldmps_present == 1 as core::ffi::c_int {
                (*ptr_frame_data[0 as core::ffi::c_int as usize]).rate = 1
                    as core::ffi::c_int as WORD32;
                if stereo != 0 {
                    (*ptr_frame_data[1 as core::ffi::c_int as usize]).rate = 1
                        as core::ffi::c_int as WORD32;
                }
            }
        }
        if audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int && ec_flag != 0 {
            if initial_sync_state == SBR_NOT_INITIALIZED
                && (*ptr_header_data[0 as core::ffi::c_int as usize]).err_flag != 0
            {
                (*ptr_header_data[0 as core::ffi::c_int as usize]).sync_state = SBR_NOT_INITIALIZED
                    as WORD32;
            }
        } else if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int {
            if initial_sync_state == SBR_NOT_INITIALIZED
                && (*ptr_header_data[0 as core::ffi::c_int as usize]).err_flag != 0
            {
                (*ptr_header_data[0 as core::ffi::c_int as usize]).sync_state = SBR_NOT_INITIALIZED
                    as WORD32;
            }
        } else {
            (*ptr_header_data[0 as core::ffi::c_int as usize]).sync_state = SBR_ACTIVE
                as WORD32;
        }
        if num_channels == 2 as core::ffi::c_int && !(stereo != 0 || dual_mono != 0) {
            ixheaacd_downmix_to_monosbr(
                &mut *core_sample_buf.offset(slot_element as isize),
                ch_fac,
            );
        }
        if prev_stereo == 0 && prev_ps_flag == 0 && ps_flag != 0 {
            let mut copy_size: WORD32 = 0;
            if down_samp_flag != 0 {
                copy_size = QMF_FILTER_STATE_SYN_SIZE_DOWN_SAMPLED as WORD32;
            } else {
                copy_size = QMF_FILTER_STATE_SYN_SIZE as WORD32;
            }
            memcpy(
                (*pstr_sbr_channel[1 as core::ffi::c_int as usize])
                    .str_sbr_dec
                    .str_synthesis_qmf_bank
                    .filter_states as *mut core::ffi::c_void,
                (*pstr_sbr_channel[0 as core::ffi::c_int as usize])
                    .str_sbr_dec
                    .str_synthesis_qmf_bank
                    .filter_states as *const core::ffi::c_void,
                (copy_size as size_t)
                    .wrapping_mul(::core::mem::size_of::<WORD16>() as size_t),
            );
            (*pstr_sbr_channel[1 as core::ffi::c_int as usize])
                .str_sbr_dec
                .str_sbr_scale_fact
                .st_syn_scale = (*pstr_sbr_channel[0 as core::ffi::c_int as usize])
                .str_sbr_dec
                .str_sbr_scale_fact
                .st_syn_scale;
        }
        if prev_stereo == 0 && stereo != 0 && num_channels == 2 as core::ffi::c_int
            && audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
        {
            let mut copy_size_0: WORD32 = 0;
            if down_samp_flag != 0 {
                copy_size_0 = QMF_FILTER_STATE_SYN_SIZE_DOWN_SAMPLED as WORD32;
            } else {
                copy_size_0 = QMF_FILTER_STATE_SYN_SIZE as WORD32;
            }
            memcpy(
                (*pstr_sbr_channel[1 as core::ffi::c_int as usize])
                    .str_sbr_dec
                    .str_synthesis_qmf_bank
                    .filter_states as *mut core::ffi::c_void,
                (*pstr_sbr_channel[0 as core::ffi::c_int as usize])
                    .str_sbr_dec
                    .str_synthesis_qmf_bank
                    .filter_states as *const core::ffi::c_void,
                (copy_size_0 as size_t)
                    .wrapping_mul(::core::mem::size_of::<WORD16>() as size_t),
            );
            (*pstr_sbr_channel[1 as core::ffi::c_int as usize])
                .str_sbr_dec
                .str_sbr_scale_fact
                .st_syn_scale = (*pstr_sbr_channel[0 as core::ffi::c_int as usize])
                .str_sbr_dec
                .str_sbr_scale_fact
                .st_syn_scale;
            memcpy(
                (*pstr_sbr_channel[1 as core::ffi::c_int as usize])
                    .str_sbr_dec
                    .str_codec_qmf_bank
                    .anal_filter_states as *mut core::ffi::c_void,
                (*pstr_sbr_channel[0 as core::ffi::c_int as usize])
                    .str_sbr_dec
                    .str_codec_qmf_bank
                    .anal_filter_states as *const core::ffi::c_void,
                (QMF_FILTER_STATE_ANA_SIZE as size_t)
                    .wrapping_mul(::core::mem::size_of::<WORD16>() as size_t),
            );
            (*pstr_sbr_channel[1 as core::ffi::c_int as usize])
                .str_sbr_dec
                .str_sbr_scale_fact
                .st_lb_scale = (*pstr_sbr_channel[0 as core::ffi::c_int as usize])
                .str_sbr_dec
                .str_sbr_scale_fact
                .st_lb_scale;
            memcpy(
                (*pstr_sbr_channel[1 as core::ffi::c_int as usize])
                    .str_sbr_dec
                    .ptr_sbr_overlap_buf as *mut core::ffi::c_void,
                (*pstr_sbr_channel[0 as core::ffi::c_int as usize])
                    .str_sbr_dec
                    .ptr_sbr_overlap_buf as *const core::ffi::c_void,
                ((MAX_OV_COLS * NO_SYNTHESIS_CHANNELS) as size_t)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
            );
            (*pstr_sbr_channel[1 as core::ffi::c_int as usize])
                .str_sbr_dec
                .str_sbr_scale_fact
                .ov_lb_scale = (*pstr_sbr_channel[0 as core::ffi::c_int as usize])
                .str_sbr_dec
                .str_sbr_scale_fact
                .ov_lb_scale;
            (*pstr_sbr_channel[1 as core::ffi::c_int as usize])
                .str_sbr_dec
                .str_sbr_scale_fact
                .ov_hb_scale = (*pstr_sbr_channel[0 as core::ffi::c_int as usize])
                .str_sbr_dec
                .str_sbr_scale_fact
                .ov_hb_scale;
        }
        (*pstr_sbr_channel[0 as core::ffi::c_int as usize])
            .str_sbr_dec
            .time_sample_buf = (*self_0).time_sample_buf[0 as core::ffi::c_int as usize];
        if !((*self_0).pstr_ps_stereo_dec).is_null()
            && (ps_enable != 0 || (*self_0).enh_sbr_ps != 0) && (*self_0).enh_sbr != 0
        {
            (*(*self_0).pstr_ps_stereo_dec)
                .pp_qmf_buf_real[0 as core::ffi::c_int as usize] = (*pstr_sbr_channel[0
                as core::ffi::c_int as usize])
                .str_sbr_dec
                .pp_qmf_buf_real;
            (*(*self_0).pstr_ps_stereo_dec)
                .pp_qmf_buf_imag[0 as core::ffi::c_int as usize] = (*pstr_sbr_channel[0
                as core::ffi::c_int as usize])
                .str_sbr_dec
                .pp_qmf_buf_imag;
            (*(*self_0).pstr_ps_stereo_dec)
                .pp_qmf_buf_real[1 as core::ffi::c_int as usize] = (*pstr_sbr_channel[1
                as core::ffi::c_int as usize])
                .str_sbr_dec
                .pp_qmf_buf_real;
            (*(*self_0).pstr_ps_stereo_dec)
                .pp_qmf_buf_imag[1 as core::ffi::c_int as usize] = (*pstr_sbr_channel[1
                as core::ffi::c_int as usize])
                .str_sbr_dec
                .pp_qmf_buf_imag;
            (*(*self_0).pstr_ps_stereo_dec)
                .time_sample_buf[0 as core::ffi::c_int as usize] = (*self_0)
                .time_sample_buf[0 as core::ffi::c_int as usize];
            (*(*self_0).pstr_ps_stereo_dec)
                .time_sample_buf[1 as core::ffi::c_int as usize] = (*self_0)
                .time_sample_buf[1 as core::ffi::c_int as usize];
        }
    } else {
        k = 0 as core::ffi::c_int as WORD32;
        while k < 2 as core::ffi::c_int {
            ptr_frame_data[k as usize] = (*self_0).frame_buffer[k as usize]
                as *mut ia_sbr_frame_info_data_struct;
            pstr_sbr_channel[k as usize] = (*self_0).pstr_sbr_channel[k as usize];
            ptr_header_data[k as usize] = (*self_0).pstr_sbr_header[k as usize];
            (*ptr_header_data[k as usize]).usac_flag = (*self_0).aot_usac_flag as WORD32;
            k += 1;
        }
        if (*p_sbr_bit_stream).no_elements as core::ffi::c_int != 0
            && (*((*p_sbr_bit_stream).str_sbr_ele).as_mut_ptr()).size_payload
                > 0 as core::ffi::c_int
        {
            num_elements = (*p_sbr_bit_stream).no_elements as WORD32;
        } else {
            num_elements = 0 as core::ffi::c_int as WORD32;
        }
        k = 0 as core::ffi::c_int as WORD32;
        while k < num_elements {
            let mut ptr_bit_str_ele_0: *mut ia_sbr_element_stream_struct = &mut *((*p_sbr_bit_stream)
                .str_sbr_ele)
                .as_mut_ptr()
                .offset(k as isize) as *mut ia_sbr_element_stream_struct;
            ele_channels = (if (*((*p_sbr_bit_stream).str_sbr_ele)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .sbr_ele_id == SBR_ID_CPE as core::ffi::c_int
            {
                2 as core::ffi::c_int
            } else {
                1 as core::ffi::c_int
            }) as WORD32;
            match (*ptr_bit_str_ele_0).sbr_ele_id {
                0 | 2 => {
                    if num_channels == 2 as core::ffi::c_int {
                        dual_mono = 1 as core::ffi::c_int as FLAG;
                    }
                    stereo = 0 as core::ffi::c_int as FLAG;
                }
                1 => {
                    stereo = 1 as core::ffi::c_int as FLAG;
                    ptr_header_data[1 as core::ffi::c_int as usize] = ptr_header_data[0
                        as core::ffi::c_int as usize];
                    memcpy(
                        (*self_0).pstr_sbr_header[1 as core::ffi::c_int as usize]
                            as *mut core::ffi::c_void,
                        (*self_0).pstr_sbr_header[0 as core::ffi::c_int as usize]
                            as *const core::ffi::c_void,
                        ::core::mem::size_of::<ia_sbr_header_data_struct>() as size_t,
                    );
                }
                _ => {
                    frame_status = 0 as core::ffi::c_int as FLAG;
                }
            }
            k += 1;
        }
    }
    if ec_flag != 0 {
        k = 0 as core::ffi::c_int as WORD32;
        while k < 2 as core::ffi::c_int {
            if (*pstr_sbr_channel[k as usize]).str_sbr_dec.band_count
                == 0 as core::ffi::c_int
            {
                (*pstr_sbr_channel[k as usize]).str_sbr_dec.band_count = (*pstr_sbr_channel[k
                    as usize])
                    .str_sbr_dec
                    .str_codec_qmf_bank
                    .no_channels;
            }
            k += 1;
        }
    }
    if ec_flag != 0 && usac_flag != 0 && first_frame == 0
        && (*self_0).sbr_parse_complete == 0
    {
        return IA_FATAL_ERROR as IA_ERRORCODE;
    }
    if ec_flag == 0 || first_frame == 0 || init_flag != 0 {
        if pstr_drc_dec.is_null() {
            let mut err_code: WORD32 = 0 as WORD32;
            err_code = ixheaacd_sbr_dec(
                &mut (**pstr_sbr_channel
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize))
                    .str_sbr_dec,
                core_sample_buf.offset(slot_element as isize),
                ptr_header_data[0 as core::ffi::c_int as usize],
                ptr_frame_data[0 as core::ffi::c_int as usize],
                (*pstr_sbr_channel[0 as core::ffi::c_int as usize]).pstr_prev_frame_data,
                (*self_0).pstr_ps_stereo_dec,
                &mut (**pstr_sbr_channel
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as isize))
                    .str_sbr_dec
                    .str_synthesis_qmf_bank,
                &mut (**pstr_sbr_channel
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as isize))
                    .str_sbr_dec
                    .str_sbr_scale_fact,
                ((*ptr_header_data[0 as core::ffi::c_int as usize]).sync_state
                    == SBR_ACTIVE) as core::ffi::c_int,
                low_pow_flag,
                (*sbr_scratch_struct).ptr_work_buf_core as *mut WORD32,
                (*self_0).pstr_sbr_tables,
                (*self_0).pstr_common_tables,
                ch_fac as WORD,
                (*self_0).ptr_pvc_data_str,
                0 as FLAG,
                0 as *mut [WORD32; 64],
                audio_object_type,
                ldmps_present,
                self_0 as *mut core::ffi::c_void,
                heaac_mps_present,
                ec_flag,
            );
            if err_code != 0 {
                return err_code as IA_ERRORCODE;
            }
            if (*self_0).enh_sbr != 0 {
                if (*self_0).enh_sbr_ps == 0 {
                    if (*ptr_header_data[0 as core::ffi::c_int as usize]).sync_state
                        == SBR_ACTIVE
                        && (*ptr_frame_data[0 as core::ffi::c_int as usize]).mps_sbr_flag
                            == 0 && ch_fac != 2 as core::ffi::c_int
                    {
                        (*((*ptr_header_data[0 as core::ffi::c_int as usize])
                            .pstr_freq_band_data)
                            .offset(0 as core::ffi::c_int as isize))
                            .qmf_sb_prev = (*(*ptr_header_data[0 as core::ffi::c_int
                            as usize])
                            .pstr_freq_band_data)
                            .sub_band_start;
                    }
                } else if (*ptr_header_data[0 as core::ffi::c_int as usize]).sync_state
                    == SBR_ACTIVE
                {
                    (*((*ptr_header_data[0 as core::ffi::c_int as usize])
                        .pstr_freq_band_data)
                        .offset(0 as core::ffi::c_int as isize))
                        .qmf_sb_prev = (*(*ptr_header_data[0 as core::ffi::c_int
                        as usize])
                        .pstr_freq_band_data)
                        .sub_band_start;
                }
            }
        } else {
            let mut err_code_0: WORD32 = 0 as WORD32;
            err_code_0 = ixheaacd_sbr_dec(
                &mut (**pstr_sbr_channel
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize))
                    .str_sbr_dec,
                core_sample_buf.offset(slot_element as isize),
                ptr_header_data[0 as core::ffi::c_int as usize],
                ptr_frame_data[0 as core::ffi::c_int as usize],
                (*pstr_sbr_channel[0 as core::ffi::c_int as usize]).pstr_prev_frame_data,
                (*self_0).pstr_ps_stereo_dec,
                &mut (**pstr_sbr_channel
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as isize))
                    .str_sbr_dec
                    .str_synthesis_qmf_bank,
                &mut (**pstr_sbr_channel
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as isize))
                    .str_sbr_dec
                    .str_sbr_scale_fact,
                ((*ptr_header_data[0 as core::ffi::c_int as usize]).sync_state
                    == SBR_ACTIVE) as core::ffi::c_int,
                low_pow_flag,
                (*sbr_scratch_struct).ptr_work_buf_core as *mut WORD32,
                (*self_0).pstr_sbr_tables,
                (*self_0).pstr_common_tables,
                ch_fac as WORD,
                (*self_0).ptr_pvc_data_str,
                (*pstr_drc_dec).drc_on,
                ((*pstr_drc_dec)
                    .str_drc_channel_data[0 as core::ffi::c_int as usize]
                    .drc_factors_sbr)
                    .as_mut_ptr(),
                audio_object_type,
                ldmps_present,
                self_0 as *mut core::ffi::c_void,
                heaac_mps_present,
                ec_flag,
            );
            if err_code_0 != 0 {
                return err_code_0 as IA_ERRORCODE;
            }
            if (*self_0).enh_sbr != 0 {
                if (*self_0).enh_sbr_ps == 0 {
                    if (*ptr_header_data[0 as core::ffi::c_int as usize]).sync_state
                        == SBR_ACTIVE
                        && (*ptr_frame_data[0 as core::ffi::c_int as usize]).mps_sbr_flag
                            == 0 && num_channels != 2 as core::ffi::c_int
                    {
                        (*((*ptr_header_data[0 as core::ffi::c_int as usize])
                            .pstr_freq_band_data)
                            .offset(0 as core::ffi::c_int as isize))
                            .qmf_sb_prev = (*(*ptr_header_data[0 as core::ffi::c_int
                            as usize])
                            .pstr_freq_band_data)
                            .sub_band_start;
                    }
                } else if (*ptr_header_data[0 as core::ffi::c_int as usize]).sync_state
                    == SBR_ACTIVE
                {
                    (*((*ptr_header_data[0 as core::ffi::c_int as usize])
                        .pstr_freq_band_data)
                        .offset(0 as core::ffi::c_int as isize))
                        .qmf_sb_prev = (*(*ptr_header_data[0 as core::ffi::c_int
                        as usize])
                        .pstr_freq_band_data)
                        .sub_band_start;
                }
            }
        }
        if down_mix_flag == 0 && (stereo != 0 || dual_mono != 0)
            && num_channels == 2 as core::ffi::c_int
        {
            (*pstr_sbr_channel[1 as core::ffi::c_int as usize])
                .str_sbr_dec
                .time_sample_buf = (*self_0)
                .time_sample_buf[1 as core::ffi::c_int as usize];
            if ele_channels == 1 as core::ffi::c_int && usac_flag != 0 {
                let mut err_code_1: WORD32 = ixheaacd_esbr_dec(
                    &mut (**pstr_sbr_channel
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as isize))
                        .str_sbr_dec,
                    ptr_header_data[1 as core::ffi::c_int as usize],
                    ptr_frame_data[1 as core::ffi::c_int as usize],
                    ((*ptr_header_data[1 as core::ffi::c_int as usize]).sync_state
                        == SBR_ACTIVE) as core::ffi::c_int,
                    low_pow_flag,
                    (*self_0).pstr_sbr_tables,
                    ch_fac,
                );
                if err_code_1 != 0 {
                    return err_code_1 as IA_ERRORCODE;
                }
            } else if pstr_drc_dec.is_null() {
                let mut err_code_2: WORD32 = ixheaacd_sbr_dec(
                    &mut (**pstr_sbr_channel
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as isize))
                        .str_sbr_dec,
                    core_sample_buf
                        .offset(slot_element as isize)
                        .offset(1 as core::ffi::c_int as isize),
                    ptr_header_data[1 as core::ffi::c_int as usize],
                    ptr_frame_data[1 as core::ffi::c_int as usize],
                    (*pstr_sbr_channel[1 as core::ffi::c_int as usize])
                        .pstr_prev_frame_data,
                    0 as *mut ia_ps_dec_struct,
                    0 as *mut ia_sbr_qmf_filter_bank_struct,
                    0 as *mut ia_sbr_scale_fact_struct,
                    ((*ptr_header_data[1 as core::ffi::c_int as usize]).sync_state
                        == SBR_ACTIVE) as core::ffi::c_int,
                    low_pow_flag,
                    (*sbr_scratch_struct).ptr_work_buf_core as *mut WORD32,
                    (*self_0).pstr_sbr_tables,
                    (*self_0).pstr_common_tables,
                    ch_fac as WORD,
                    (*self_0).ptr_pvc_data_str,
                    0 as FLAG,
                    0 as *mut [WORD32; 64],
                    audio_object_type,
                    ldmps_present,
                    self_0 as *mut core::ffi::c_void,
                    heaac_mps_present,
                    ec_flag,
                );
                if err_code_2 != 0 {
                    return err_code_2 as IA_ERRORCODE;
                }
                if (*self_0).enh_sbr != 0 {
                    if (*self_0).enh_sbr_ps == 0 {
                        if (*ptr_header_data[1 as core::ffi::c_int as usize]).sync_state
                            == SBR_ACTIVE
                            && (*ptr_frame_data[0 as core::ffi::c_int as usize])
                                .mps_sbr_flag == 0
                        {
                            (*((*ptr_header_data[1 as core::ffi::c_int as usize])
                                .pstr_freq_band_data)
                                .offset(0 as core::ffi::c_int as isize))
                                .qmf_sb_prev = (*(*ptr_header_data[1 as core::ffi::c_int
                                as usize])
                                .pstr_freq_band_data)
                                .sub_band_start;
                        }
                    } else if (*ptr_header_data[1 as core::ffi::c_int as usize])
                        .sync_state == SBR_ACTIVE
                    {
                        (*((*ptr_header_data[1 as core::ffi::c_int as usize])
                            .pstr_freq_band_data)
                            .offset(0 as core::ffi::c_int as isize))
                            .qmf_sb_prev = (*(*ptr_header_data[1 as core::ffi::c_int
                            as usize])
                            .pstr_freq_band_data)
                            .sub_band_start;
                    }
                }
            } else {
                let mut err_code_3: WORD32 = ixheaacd_sbr_dec(
                    &mut (**pstr_sbr_channel
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as isize))
                        .str_sbr_dec,
                    core_sample_buf
                        .offset(slot_element as isize)
                        .offset(1 as core::ffi::c_int as isize),
                    ptr_header_data[1 as core::ffi::c_int as usize],
                    ptr_frame_data[1 as core::ffi::c_int as usize],
                    (*pstr_sbr_channel[1 as core::ffi::c_int as usize])
                        .pstr_prev_frame_data,
                    0 as *mut ia_ps_dec_struct,
                    0 as *mut ia_sbr_qmf_filter_bank_struct,
                    0 as *mut ia_sbr_scale_fact_struct,
                    ((*ptr_header_data[1 as core::ffi::c_int as usize]).sync_state
                        == SBR_ACTIVE) as core::ffi::c_int,
                    low_pow_flag,
                    (*sbr_scratch_struct).ptr_work_buf_core as *mut WORD32,
                    (*self_0).pstr_sbr_tables,
                    (*self_0).pstr_common_tables,
                    ch_fac as WORD,
                    (*self_0).ptr_pvc_data_str,
                    (*pstr_drc_dec).drc_on,
                    ((*pstr_drc_dec)
                        .str_drc_channel_data[1 as core::ffi::c_int as usize]
                        .drc_factors_sbr)
                        .as_mut_ptr(),
                    audio_object_type,
                    ldmps_present,
                    self_0 as *mut core::ffi::c_void,
                    heaac_mps_present,
                    ec_flag,
                );
                if err_code_3 != 0 {
                    return err_code_3 as IA_ERRORCODE;
                }
                if (*self_0).enh_sbr != 0 {
                    if (*self_0).enh_sbr_ps == 0 {
                        if (*ptr_header_data[1 as core::ffi::c_int as usize]).sync_state
                            == SBR_ACTIVE
                            && (*ptr_frame_data[0 as core::ffi::c_int as usize])
                                .mps_sbr_flag == 0
                        {
                            (*((*ptr_header_data[1 as core::ffi::c_int as usize])
                                .pstr_freq_band_data)
                                .offset(0 as core::ffi::c_int as isize))
                                .qmf_sb_prev = (*(*ptr_header_data[1 as core::ffi::c_int
                                as usize])
                                .pstr_freq_band_data)
                                .sub_band_start;
                        }
                    } else if (*ptr_header_data[1 as core::ffi::c_int as usize])
                        .sync_state == SBR_ACTIVE
                    {
                        (*((*ptr_header_data[1 as core::ffi::c_int as usize])
                            .pstr_freq_band_data)
                            .offset(0 as core::ffi::c_int as isize))
                            .qmf_sb_prev = (*(*ptr_header_data[1 as core::ffi::c_int
                            as usize])
                            .pstr_freq_band_data)
                            .sub_band_start;
                    }
                }
            }
        } else if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
            && audio_object_type != AOT_ER_AAC_LD as core::ffi::c_int
        {
            if (*ptr_header_data[0 as core::ffi::c_int as usize]).channel_mode as WORD16
                as core::ffi::c_int - 3 as core::ffi::c_int == 0 as core::ffi::c_int
            {
                num_channels = 2 as core::ffi::c_int as WORD32;
            }
            if (*ptr_header_data[0 as core::ffi::c_int as usize]).enh_sbr_ps != 0
                && (*self_0).enh_sbr != 0
            {
                num_channels = 2 as core::ffi::c_int as WORD32;
            }
        }
        *codec_num_channels = num_channels as WORD16;
        (*self_0).sbr_mode = (*ptr_frame_data[0 as core::ffi::c_int as usize]).sbr_mode;
        if audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int
            || audio_object_type == AOT_ER_AAC_LD as core::ffi::c_int
            || (*self_0).enh_sbr == 0
        {
            if !pstr_drc_dec.is_null() {
                let mut i: WORD32 = 0;
                let mut j: WORD32 = 0;
                i = 0 as core::ffi::c_int as WORD32;
                while i < *codec_num_channels as core::ffi::c_int {
                    j = 0 as core::ffi::c_int as WORD32;
                    while j < 32 as core::ffi::c_int {
                        memcpy(
                            ((*pstr_drc_dec)
                                .str_drc_channel_data[i as usize]
                                .drc_factors_sbr[j as usize])
                                .as_mut_ptr() as *mut core::ffi::c_void,
                            ((*pstr_drc_dec)
                                .str_drc_channel_data[i as usize]
                                .drc_factors_sbr[(j as core::ffi::c_int
                                + 32 as core::ffi::c_int) as usize])
                                .as_mut_ptr() as *const core::ffi::c_void,
                            (64 as size_t)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
                        );
                        j += 1;
                    }
                    i += 1;
                }
            }
        }
        if ec_flag != 0 {
            (*self_0).band_count[0 as core::ffi::c_int as usize] = (*pstr_sbr_channel[0
                as core::ffi::c_int as usize])
                .str_sbr_dec
                .band_count;
            (*self_0).band_count[1 as core::ffi::c_int as usize] = (*pstr_sbr_channel[1
                as core::ffi::c_int as usize])
                .str_sbr_dec
                .band_count;
        }
    }
    return SBRDEC_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_parse_sbr(
    mut self_0: ia_handle_sbr_dec_inst_struct,
    mut p_sbr_bit_stream: *mut ia_aac_dec_sbr_bitstream_struct,
    mut codec_num_channels: *mut WORD16,
    mut frame_status: FLAG,
    mut sbr_scratch_struct: *mut ia_sbr_scr_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut audio_object_type: WORD32,
) -> IA_ERRORCODE {
    let mut k: WORD32 = 0;
    let mut ps_flag: FLAG = 0 as FLAG;
    let mut stereo: FLAG = 0 as FLAG;
    let mut low_pow_flag: FLAG = 0 as FLAG;
    let mut header_flag: FLAG = 1 as FLAG;
    let mut dual_mono: FLAG = 0 as FLAG;
    let mut err: WORD32 = 0 as WORD32;
    let mut num_channels: WORD32 = *codec_num_channels as WORD32;
    let mut prev_stereo: FLAG = 0;
    let mut num_elements: WORD32 = (*p_sbr_bit_stream).no_elements as WORD32;
    let mut usac_flag: WORD32 = (*self_0).aot_usac_flag as WORD32;
    let mut pstr_sbr_channel: [*mut ia_sbr_channel_struct; 2] = [0
        as *mut ia_sbr_channel_struct; 2];
    let mut ptr_header_data: [*mut ia_sbr_header_data_struct; 2] = [0
        as *mut ia_sbr_header_data_struct; 2];
    let mut initial_sync_state: WORD32 = 0;
    let mut ptr_sbr_dflt_header: *mut ia_sbr_header_data_struct = &mut (*self_0)
        .str_sbr_dflt_header as *mut ia_sbr_header_data_struct;
    let mut ptr_frame_data: [*mut ia_sbr_frame_info_data_struct; 2] = [0
        as *mut ia_sbr_frame_info_data_struct; 2];
    (*self_0).num_delay_frames = 1 as core::ffi::c_int as WORD32;
    (*self_0).ptr_mps_data = 0 as *mut WORD8;
    if usac_flag != 0 && (*self_0).ec_flag != 0 {
        k = 0 as core::ffi::c_int as WORD32;
        while k < 2 as core::ffi::c_int {
            ptr_frame_data[k as usize] = (*self_0).frame_buffer[k as usize]
                as *mut ia_sbr_frame_info_data_struct;
            pstr_sbr_channel[k as usize] = (*self_0).pstr_sbr_channel[k as usize];
            ptr_header_data[k as usize] = (*self_0).pstr_sbr_header[k as usize];
            (*ptr_frame_data[k as usize]).usac_independency_flag = (*self_0)
                .usac_independency_flag;
            (*ptr_frame_data[k as usize]).mps_sbr_flag = (if (*self_0).stereo_config_idx
                == 3 as core::ffi::c_int
            {
                1 as core::ffi::c_int
            } else {
                0 as core::ffi::c_int
            }) as FLAG;
            (*ptr_frame_data[k as usize]).stereo_config_idx = (*self_0).stereo_config_idx
                as WORD32;
            (*ptr_frame_data[k as usize]).inter_tes_flag = (*self_0).inter_tes_flag;
            (*ptr_frame_data[k as usize]).sbr_mode = (*self_0).sbr_mode;
            k += 1;
        }
        k = 0 as core::ffi::c_int as WORD32;
        while k < *codec_num_channels as core::ffi::c_int {
            (*ptr_header_data[k as usize]).usac_flag = (*self_0).aot_usac_flag as WORD32;
            (*ptr_header_data[k as usize]).enh_sbr = (*self_0).enh_sbr;
            (*ptr_header_data[k as usize]).enh_sbr_ps = ((*self_0).enh_sbr_ps
                as core::ffi::c_int
                | ((*ptr_header_data[k as usize]).channel_mode == PS_STEREO)
                    as core::ffi::c_int) as FLAG;
            (*ptr_header_data[k as usize]).usac_independency_flag = (*self_0)
                .usac_independency_flag as WORD32;
            (*ptr_header_data[k as usize]).hbe_flag = (*self_0).hbe_flag;
            (*ptr_header_data[k as usize]).pvc_flag = (*self_0).pvc_flag;
            (*ptr_header_data[k as usize]).esbr_hq = (*self_0).esbr_hq;
            k += 1;
        }
        initial_sync_state = (*ptr_header_data[0 as core::ffi::c_int as usize])
            .sync_state;
        low_pow_flag = 0 as core::ffi::c_int as FLAG;
        (*(*self_0).pstr_sbr_tables).sbr_rand_ph = ((*(*(*self_0).pstr_sbr_tables)
            .env_calc_tables_ptr)
            .sbr_rand_ph)
            .as_mut_ptr();
        prev_stereo = ((*ptr_header_data[0 as core::ffi::c_int as usize]).channel_mode
            == SBR_STEREO) as core::ffi::c_int as FLAG;
        (*ptr_header_data[0 as core::ffi::c_int as usize]).err_flag_prev = (*ptr_header_data[0
            as core::ffi::c_int as usize])
            .err_flag;
        if (*p_sbr_bit_stream).no_elements as core::ffi::c_int == 0 as core::ffi::c_int {
            frame_status = 0 as core::ffi::c_int as FLAG;
            (*ptr_header_data[0 as core::ffi::c_int as usize]).sync_state = UPSAMPLING
                as WORD32;
            if num_channels == 2 as core::ffi::c_int {
                (*ptr_header_data[1 as core::ffi::c_int as usize]).sync_state = UPSAMPLING
                    as WORD32;
            }
        }
        if (*p_sbr_bit_stream).no_elements as core::ffi::c_int != 0
            && (*((*p_sbr_bit_stream).str_sbr_ele).as_mut_ptr()).size_payload
                > 0 as core::ffi::c_int
        {
            num_elements = (*p_sbr_bit_stream).no_elements as WORD32;
        } else {
            num_elements = 0 as core::ffi::c_int as WORD32;
        }
        k = 0 as core::ffi::c_int as WORD32;
        while k < num_elements {
            let mut ptr_bit_str_ele: *mut ia_sbr_element_stream_struct = &mut *((*p_sbr_bit_stream)
                .str_sbr_ele)
                .as_mut_ptr()
                .offset(k as isize) as *mut ia_sbr_element_stream_struct;
            match (*ptr_bit_str_ele).sbr_ele_id {
                0 | 2 => {
                    if num_channels == 2 as core::ffi::c_int {
                        dual_mono = 1 as core::ffi::c_int as FLAG;
                    }
                    stereo = 0 as core::ffi::c_int as FLAG;
                }
                1 => {
                    stereo = 1 as core::ffi::c_int as FLAG;
                    ptr_header_data[1 as core::ffi::c_int as usize] = ptr_header_data[0
                        as core::ffi::c_int as usize];
                    memcpy(
                        (*self_0).pstr_sbr_header[1 as core::ffi::c_int as usize]
                            as *mut core::ffi::c_void,
                        (*self_0).pstr_sbr_header[0 as core::ffi::c_int as usize]
                            as *const core::ffi::c_void,
                        ::core::mem::size_of::<ia_sbr_header_data_struct>() as size_t,
                    );
                }
                _ => {
                    frame_status = 0 as core::ffi::c_int as FLAG;
                }
            }
            if frame_status != 0 {
                if (*ptr_bit_str_ele).extension_type == SBR_EXTENSION_CRC {
                    let mut crc_bits: WORD32 = 0 as WORD32;
                    let mut crc_check_flag: WORD32 = 0 as WORD32;
                    crc_check_flag = 1 as core::ffi::c_int as WORD32;
                    crc_bits = ((((*ptr_bit_str_ele).size_payload as core::ffi::c_int
                        - 1 as core::ffi::c_int) << 3 as core::ffi::c_int)
                        + (4 as core::ffi::c_int - SBR_CYC_REDCY_CHK_BITS)) as WORD32;
                    if crc_bits < 0 as core::ffi::c_int {
                        crc_check_flag = 0 as core::ffi::c_int as WORD32;
                        frame_status = 0 as core::ffi::c_int as FLAG;
                    }
                    if crc_check_flag != 0 && frame_status == 1 as core::ffi::c_int {
                        frame_status = ixheaacd_sbr_crccheck(it_bit_buff, crc_bits);
                    }
                }
                if frame_status == 1 as core::ffi::c_int {
                    header_flag = ixheaacd_sbr_read_header_data(
                        ptr_header_data[k as usize],
                        it_bit_buff,
                        stereo,
                        ptr_sbr_dflt_header,
                    ) as FLAG;
                }
                if usac_flag != 0 {
                    if (*(*self_0).ptr_pvc_data_str).prev_pvc_mode as core::ffi::c_int
                        == 0 as core::ffi::c_int
                        && (*ptr_header_data[k as usize]).pvc_mode as core::ffi::c_int
                            != 0 as core::ffi::c_int
                    {
                        (*(*self_0).ptr_pvc_data_str).prev_pvc_id = 0 as UWORD16;
                    }
                    (*(*self_0).ptr_pvc_data_str).prev_pvc_mode = (*ptr_header_data[k
                        as usize])
                        .pvc_mode;
                    if (*ptr_header_data[k as usize]).pvc_mode as core::ffi::c_int
                        == 0 as core::ffi::c_int
                    {
                        (*ptr_frame_data[k as usize]).sbr_mode = ORIG_SBR
                            as core::ffi::c_int as FLAG;
                    } else {
                        (*ptr_frame_data[k as usize]).sbr_mode = PVC_SBR
                            as core::ffi::c_int as FLAG;
                    }
                }
                if header_flag == SBR_RESET {
                    err = ixheaacd_calc_frq_bnd_tbls(
                        ptr_header_data[k as usize],
                        (*self_0).pstr_common_tables,
                    );
                    if err == 0 {
                        let mut lr: WORD32 = 0;
                        let mut lr1: WORD32 = num_channels;
                        lr = 0 as core::ffi::c_int as WORD32;
                        while lr < lr1 {
                            (*ptr_frame_data[lr as usize]).reset_flag = 1
                                as core::ffi::c_int as FLAG;
                            if SBR_NOT_INITIALIZED
                                == (*ptr_header_data[lr as usize]).sync_state
                                && usac_flag == 0
                            {
                                (*ptr_frame_data[lr as usize]).sbr_patching_mode = 1
                                    as core::ffi::c_int as WORD32;
                                (*ptr_frame_data[lr as usize]).over_sampling_flag = 0
                                    as core::ffi::c_int as WORD32;
                                (*ptr_frame_data[lr as usize]).pitch_in_bins = 0
                                    as core::ffi::c_int as WORD32;
                                (*ptr_header_data[lr as usize]).pre_proc_flag = 0 as WORD16;
                            }
                            err
                                |= ixheaacd_sbr_dec_reset(
                                    &mut (**pstr_sbr_channel.as_mut_ptr().offset(lr as isize))
                                        .str_sbr_dec,
                                    ptr_header_data[k as usize],
                                    low_pow_flag,
                                    (*self_0).pstr_common_tables,
                                    (*ptr_frame_data[k as usize]).pitch_in_bins,
                                    audio_object_type,
                                    (*sbr_scratch_struct).ptr_work_buf_core as *mut WORD32,
                                );
                            if err < 0 as core::ffi::c_int {
                                if (*self_0).ec_flag != 0 {
                                    (*self_0).frame_ok = 0 as core::ffi::c_int as FLAG;
                                } else {
                                    return err as IA_ERRORCODE
                                }
                            }
                            lr += 1;
                        }
                    }
                    if err == 0 as core::ffi::c_int {
                        (*ptr_header_data[k as usize]).sync_state = SBR_ACTIVE as WORD32;
                    }
                }
            }
            if err != 0
                || (*ptr_header_data[k as usize]).sync_state == SBR_NOT_INITIALIZED
            {
                let mut lr1_0: WORD32 = num_channels;
                ixheaacd_prepare_upsamp(
                    ptr_header_data.as_mut_ptr(),
                    pstr_sbr_channel.as_mut_ptr(),
                    lr1_0,
                );
                if err != 0 && (*self_0).ec_flag == 0 {
                    return err as IA_ERRORCODE;
                }
            }
            if frame_status != 0
                && (*ptr_header_data[k as usize]).sync_state == SBR_ACTIVE
            {
                if stereo != 0 {
                    frame_status = ixheaacd_sbr_read_cpe(
                        ptr_header_data[0 as core::ffi::c_int as usize],
                        ptr_frame_data.as_mut_ptr(),
                        it_bit_buff,
                        (*self_0).pstr_sbr_tables,
                        audio_object_type as WORD,
                    ) as FLAG;
                    if frame_status < 0 as core::ffi::c_int {
                        return frame_status as IA_ERRORCODE;
                    }
                } else {
                    (*self_0).pstr_ps_stereo_dec = 0 as *mut ia_ps_dec_struct;
                    if (*ptr_frame_data[k as usize]).sbr_mode
                        == ORIG_SBR as core::ffi::c_int
                    {
                        frame_status = ixheaacd_sbr_read_sce(
                            ptr_header_data[k as usize],
                            ptr_frame_data[k as usize],
                            (*self_0).pstr_ps_stereo_dec,
                            it_bit_buff,
                            (*self_0).pstr_sbr_tables,
                            audio_object_type as WORD,
                            0 as WORD32,
                        ) as FLAG;
                        if frame_status < 0 as core::ffi::c_int {
                            return frame_status as IA_ERRORCODE;
                        }
                    } else if (*ptr_frame_data[k as usize]).sbr_mode
                        == PVC_SBR as core::ffi::c_int
                    {
                        frame_status = ixheaacd_sbr_read_pvc_sce(
                            ptr_frame_data[k as usize],
                            it_bit_buff,
                            0 as WORD32,
                            (*self_0).ptr_pvc_data_str,
                            (*self_0).pstr_sbr_tables,
                            ptr_header_data[k as usize],
                        ) as FLAG;
                        if frame_status < 0 as core::ffi::c_int {
                            return frame_status as IA_ERRORCODE;
                        }
                    }
                }
                (*ptr_header_data[k as usize]).enh_sbr_ps = ((*self_0).enh_sbr_ps
                    as core::ffi::c_int
                    | ((*ptr_header_data[0 as core::ffi::c_int as usize]).channel_mode
                        == PS_STEREO) as core::ffi::c_int) as FLAG;
                let mut total_bits_read: WORD32 = 0;
                total_bits_read = (*it_bit_buff).size - (*it_bit_buff).cnt_bits;
                if total_bits_read
                    > (*ptr_bit_str_ele).size_payload << 3 as core::ffi::c_int
                    || total_bits_read
                        < (((*ptr_bit_str_ele).size_payload as core::ffi::c_int)
                            << 3 as core::ffi::c_int) - 8 as core::ffi::c_int
                {
                    frame_status = 0 as core::ffi::c_int as FLAG;
                }
            } else {
                if frame_status != 0 && (*self_0).ec_flag != 0 {
                    err = IA_XHEAAC_DEC_EXE_NONFATAL_SBR_PARSE_ERROR as WORD32;
                    (*self_0).sbr_parse_err_flag = 1 as core::ffi::c_int as FLAG;
                }
                if frame_status == 0 {
                    ixheaacd_ec_set_frame_error_flag(ptr_bit_str_ele, FRAME_ERROR);
                } else {
                    ixheaacd_ec_set_frame_error_flag(ptr_bit_str_ele, FRAME_OK);
                }
            }
            k += 1;
        }
        if (*ptr_header_data[0 as core::ffi::c_int as usize]).sync_state == SBR_ACTIVE {
            if (*ptr_frame_data[0 as core::ffi::c_int as usize]).sbr_mode
                == PVC_SBR as core::ffi::c_int
            {
                err = ixheaacd_dec_sbrdata_for_pvc(
                    ptr_header_data[0 as core::ffi::c_int as usize],
                    ptr_frame_data[0 as core::ffi::c_int as usize],
                    (*pstr_sbr_channel[0 as core::ffi::c_int as usize])
                        .pstr_prev_frame_data,
                    audio_object_type,
                ) as WORD32;
                if err != 0 {
                    if (*self_0).ec_flag != 0 {
                        (*self_0).frame_ok = 0 as core::ffi::c_int as FLAG;
                    } else {
                        return err as IA_ERRORCODE
                    }
                }
            } else if (*ptr_frame_data[0 as core::ffi::c_int as usize]).sbr_mode
                == ORIG_SBR as core::ffi::c_int
            {
                err = ixheaacd_dec_sbrdata(
                    ptr_header_data[0 as core::ffi::c_int as usize],
                    ptr_header_data[1 as core::ffi::c_int as usize],
                    ptr_frame_data[0 as core::ffi::c_int as usize],
                    (*pstr_sbr_channel[0 as core::ffi::c_int as usize])
                        .pstr_prev_frame_data,
                    if stereo != 0 || dual_mono != 0 {
                        ptr_frame_data[1 as core::ffi::c_int as usize]
                    } else {
                        0 as *mut ia_sbr_frame_info_data_struct
                    },
                    if stereo != 0 || dual_mono != 0 {
                        (*pstr_sbr_channel[1 as core::ffi::c_int as usize])
                            .pstr_prev_frame_data
                    } else {
                        0 as *mut ia_sbr_prev_frame_data_struct
                    },
                    (*self_0).pstr_common_tables,
                    0 as WORD32,
                    audio_object_type,
                    (*self_0).ec_flag as WORD32,
                ) as WORD32;
                if err != 0 {
                    if (*self_0).ec_flag != 0 {
                        (*self_0).frame_ok = 0 as core::ffi::c_int as FLAG;
                    } else {
                        return err as IA_ERRORCODE
                    }
                }
            }
            if (*ptr_header_data[0 as core::ffi::c_int as usize]).channel_mode
                == PS_STEREO
                && (audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
                    && audio_object_type != AOT_ER_AAC_LD as core::ffi::c_int)
            {
                ixheaacd_decode_ps_data((*self_0).pstr_ps_stereo_dec, 1024 as WORD32);
                ps_flag = 1 as core::ffi::c_int as FLAG;
                (*self_0).ps_present = ps_flag;
            }
            if (*ptr_header_data[0 as core::ffi::c_int as usize]).enh_sbr_ps != 0 {
                ps_flag = 1 as core::ffi::c_int as FLAG;
                (*self_0).ps_present = ps_flag;
            }
            (*ptr_frame_data[0 as core::ffi::c_int as usize]).max_qmf_subband_aac = (*(*ptr_header_data[0
                as core::ffi::c_int as usize])
                .pstr_freq_band_data)
                .sub_band_start as WORD32;
            if stereo != 0 {
                (*ptr_frame_data[1 as core::ffi::c_int as usize]).max_qmf_subband_aac = (*(*ptr_header_data[1
                    as core::ffi::c_int as usize])
                    .pstr_freq_band_data)
                    .sub_band_start as WORD32;
            }
        }
        if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int {
            if initial_sync_state == SBR_NOT_INITIALIZED
                && (*ptr_header_data[0 as core::ffi::c_int as usize]).err_flag != 0
            {
                (*ptr_header_data[0 as core::ffi::c_int as usize]).sync_state = SBR_NOT_INITIALIZED
                    as WORD32;
            }
        } else {
            (*ptr_header_data[0 as core::ffi::c_int as usize]).sync_state = SBR_ACTIVE
                as WORD32;
        }
        if prev_stereo == 0 && stereo != 0 && num_channels == 2 as core::ffi::c_int
            && audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
        {
            memcpy(
                (*pstr_sbr_channel[1 as core::ffi::c_int as usize])
                    .str_sbr_dec
                    .str_synthesis_qmf_bank
                    .filter_states as *mut core::ffi::c_void,
                (*pstr_sbr_channel[0 as core::ffi::c_int as usize])
                    .str_sbr_dec
                    .str_synthesis_qmf_bank
                    .filter_states as *const core::ffi::c_void,
                (QMF_FILTER_STATE_SYN_SIZE as size_t)
                    .wrapping_mul(::core::mem::size_of::<WORD16>() as size_t),
            );
            (*pstr_sbr_channel[1 as core::ffi::c_int as usize])
                .str_sbr_dec
                .str_sbr_scale_fact
                .st_syn_scale = (*pstr_sbr_channel[0 as core::ffi::c_int as usize])
                .str_sbr_dec
                .str_sbr_scale_fact
                .st_syn_scale;
            memcpy(
                (*pstr_sbr_channel[1 as core::ffi::c_int as usize])
                    .str_sbr_dec
                    .str_codec_qmf_bank
                    .anal_filter_states as *mut core::ffi::c_void,
                (*pstr_sbr_channel[0 as core::ffi::c_int as usize])
                    .str_sbr_dec
                    .str_codec_qmf_bank
                    .anal_filter_states as *const core::ffi::c_void,
                (QMF_FILTER_STATE_ANA_SIZE as size_t)
                    .wrapping_mul(::core::mem::size_of::<WORD16>() as size_t),
            );
            (*pstr_sbr_channel[1 as core::ffi::c_int as usize])
                .str_sbr_dec
                .str_sbr_scale_fact
                .st_lb_scale = (*pstr_sbr_channel[0 as core::ffi::c_int as usize])
                .str_sbr_dec
                .str_sbr_scale_fact
                .st_lb_scale;
            memcpy(
                (*pstr_sbr_channel[1 as core::ffi::c_int as usize])
                    .str_sbr_dec
                    .ptr_sbr_overlap_buf as *mut core::ffi::c_void,
                (*pstr_sbr_channel[0 as core::ffi::c_int as usize])
                    .str_sbr_dec
                    .ptr_sbr_overlap_buf as *const core::ffi::c_void,
                ((MAX_OV_COLS * NO_SYNTHESIS_CHANNELS) as size_t)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
            );
            (*pstr_sbr_channel[1 as core::ffi::c_int as usize])
                .str_sbr_dec
                .str_sbr_scale_fact
                .ov_lb_scale = (*pstr_sbr_channel[0 as core::ffi::c_int as usize])
                .str_sbr_dec
                .str_sbr_scale_fact
                .ov_lb_scale;
            (*pstr_sbr_channel[1 as core::ffi::c_int as usize])
                .str_sbr_dec
                .str_sbr_scale_fact
                .ov_hb_scale = (*pstr_sbr_channel[0 as core::ffi::c_int as usize])
                .str_sbr_dec
                .str_sbr_scale_fact
                .ov_hb_scale;
        }
        (*pstr_sbr_channel[0 as core::ffi::c_int as usize])
            .str_sbr_dec
            .time_sample_buf = (*self_0).time_sample_buf[0 as core::ffi::c_int as usize];
        (*self_0).sbr_parse_complete = 1 as core::ffi::c_int as FLAG;
    }
    return err as IA_ERRORCODE;
}
