extern "C" {
    pub type ia_sbr_dec_inst_struct;
    fn acos(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn cos(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn sqrt(__x: core::ffi::c_double) -> core::ffi::c_double;
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
    fn ixheaacd_memset(x: *mut FLOAT32, n: WORD32) -> VOID;
    fn ixheaacd_mem_cpy(x: *const FLOAT32, y: *mut FLOAT32, n: WORD32) -> VOID;
    fn ixheaacd_fwd_alias_cancel_tool(
        usac_data: *mut ia_usac_data_struct,
        pstr_td_frame_data: *mut ia_td_frame_data_struct,
        fac_length: WORD32,
        iaq: *mut FLOAT32,
        gain: WORD32,
    ) -> VOID;
    static ixheaacd_fir_lp_filt: [FLOAT32; 13];
    fn ixheaacd_usac_lpc_ec(
        lsp: *mut [FLOAT32; 16],
        lpc4_lsf: *mut FLOAT32,
        lsf_adaptive_mean: *mut FLOAT32,
        first_lpd_flag: WORD32,
    ) -> VOID;
    fn ixheaacd_usac_tcx_ec(
        pstr_usac_data: *mut ia_usac_data_struct,
        st: ia_usac_lpd_decoder_handle,
        ptr_lsp_curr: *mut FLOAT32,
        frame_idx: WORD32,
        lp_flt_coff_a: *mut FLOAT32,
    ) -> VOID;
    static ixheaacd_sine_win_128: [WORD32; 128];
    static ixheaacd_sine_win_192: [WORD32; 192];
    static ixheaacd_sine_win_96: [WORD32; 96];
    static ixheaacd_sine_win_256: [WORD32; 256];
    fn ixheaacd_residual_tool_float1(
        a: *mut FLOAT32,
        x: *mut FLOAT32,
        y: *mut FLOAT32,
        l: WORD32,
        loop_count: WORD32,
    ) -> VOID;
    fn ixheaacd_preemphsis_tool_float(
        signal: *mut FLOAT32,
        mu: FLOAT32,
        len: WORD32,
        mem: FLOAT32,
    ) -> VOID;
    fn ixheaacd_acelp_alias_cnx(
        usac_data: *mut ia_usac_data_struct,
        pstr_td_frame_data: *mut ia_td_frame_data_struct,
        k: WORD32,
        A: *mut FLOAT32,
        stab_fac: FLOAT32,
        st: ia_usac_lpd_decoder_handle,
    ) -> VOID;
    fn ixheaacd_tcx_mdct(
        usac_data: *mut ia_usac_data_struct,
        pstr_td_frame_data: *mut ia_td_frame_data_struct,
        frame_index: WORD32,
        A: *mut FLOAT32,
        long_frame: WORD32,
        st: ia_usac_lpd_decoder_handle,
    ) -> WORD32;
    fn ixheaacd_alg_vec_dequant(
        pstr_td_frame_data: *mut ia_td_frame_data_struct,
        first_lpd_flag: WORD32,
        lsf: *mut FLOAT32,
        mod_0: *mut WORD32,
        ec_flag: WORD32,
    ) -> VOID;
    fn ixheaacd_interpolation_lsp_params(
        lsp_old: *mut FLOAT32,
        lsp_new: *mut FLOAT32,
        lp_flt_coff_a: *mut FLOAT32,
        nb_subfr: WORD32,
    ) -> VOID;
    fn ixheaacd_lpc_coef_gen(
        lsf_old: *mut FLOAT32,
        lsf_new: *mut FLOAT32,
        a: *mut FLOAT32,
        nb_subfr: WORD32,
        m: WORD32,
    ) -> VOID;
}
pub type size_t = usize;
pub type WORD8 = core::ffi::c_schar;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD16 = core::ffi::c_short;
pub type pWORD16 = *mut core::ffi::c_short;
pub type UWORD16 = core::ffi::c_ushort;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type WORD64 = core::ffi::c_longlong;
pub type FLOAT32 = core::ffi::c_float;
pub type VOID = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_tns_filter_struct {
    pub start_band: WORD32,
    pub stop_band: WORD32,
    pub order: WORD32,
    pub direction: WORD32,
    pub coef_compress: WORD32,
    pub coef: [WORD16; 31],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_tns_info_struct {
    pub n_filt: WORD32,
    pub coef_res: WORD32,
    pub str_filter: [ia_tns_filter_struct; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_tns_frame_info_struct {
    pub n_subblocks: WORD32,
    pub str_tns_info: [ia_tns_info_struct; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_td_frame_data_struct {
    pub acelp_core_mode: WORD32,
    pub mod_0: [WORD32; 4],
    pub fac: [WORD32; 512],
    pub fac_data: [WORD32; 129],
    pub mean_energy: [WORD32; 4],
    pub acb_index: [WORD32; 16],
    pub noise_factor: [WORD32; 4],
    pub global_gain: [WORD32; 4],
    pub arith_reset_flag: WORD32,
    pub x_tcx_invquant: [WORD32; 1024],
    pub tcx_lg: [WORD32; 16],
    pub ltp_filtering_flag: [WORD32; 16],
    pub icb_index: [[WORD32; 8]; 16],
    pub gains: [WORD32; 16],
    pub mode_lpc: [WORD32; 4],
    pub lpc_first_approx_idx: [WORD32; 110],
    pub lsp_coeff: [[FLOAT32; 16]; 5],
    pub lsf_adaptive_mean_cand: [FLOAT32; 16],
    pub lsf_adaptive_mean: [FLOAT32; 16],
    pub lpc4_lsf: [FLOAT32; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sfb_info_struct {
    pub islong: WORD32,
    pub max_win_len: WORD32,
    pub samp_per_bk: WORD32,
    pub sfb_per_bk: WORD32,
    pub bins_per_sbk: WORD32,
    pub sfb_per_sbk: WORD32,
    pub ptr_sfb_tbl: *const WORD16,
    pub sfb_width: pWORD16,
    pub sfb_idx_tbl: [WORD16; 125],
    pub num_groups: WORD32,
    pub group_len: [WORD16; 8],
}
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
pub type ia_handle_sbr_dec_inst_struct = *mut ia_sbr_dec_inst_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_ec_sfb_str {
    pub num_sfb_long: WORD32,
    pub num_sfb_short: WORD32,
    pub ptr_sfb_long: *mut WORD16,
    pub ptr_sfb_short: *mut WORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_ec_scratch_str {
    pub prev_sfb_nrg: [WORD32; 51],
    pub pres_sfb_nrg: [WORD32; 51],
    pub spec_coeff: [WORD32; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_ec_state_str {
    pub spectral_coeff: [WORD32; 1024],
    pub q_spec_coeff: [WORD16; 128],
    pub prev_frame_ok: [WORD32; 2],
    pub win_shape: UWORD8,
    pub win_shape_prev: UWORD8,
    pub win_seq: WORD32,
    pub td_frame_prev: WORD32,
    pub fac_data_present: WORD32,
    pub prev_win_group_len: UWORD8,
    pub conceal_state: WORD32,
    pub prev_core_mode: WORD32,
    pub fade_idx: WORD32,
    pub lsf4: [FLOAT32; 16],
    pub str_ec_sfb: ia_ec_sfb_str,
    pub pstr_ec_scratch: *mut ia_ec_scratch_str,
    pub str_ec_scratch: ia_ec_scratch_str,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_usac_lpd_decoder {
    pub mode_prev: WORD32,
    pub synth_prev: [core::ffi::c_float; 859],
    pub xcitation_prev: [core::ffi::c_float; 428],
    pub pitch_prev: [core::ffi::c_int; 7],
    pub gain_prev: [core::ffi::c_float; 7],
    pub lp_flt_coeff_a_prev: [core::ffi::c_float; 34],
    pub exc_prev: [FLOAT32; 257],
    pub bpf_prev: [FLOAT32; 76],
    pub ilspold: [WORD32; 16],
    pub fac_gain: FLOAT32,
    pub fac_fd_data: [FLOAT32; 32],
    pub lsf_prev: [FLOAT32; 16],
    pub lspold: [FLOAT32; 16],
    pub lsfold_first: [WORD32; 16],
    pub gain_threshold: FLOAT32,
    pub fscale: WORD32,
    pub fd_synth_buf: [FLOAT32; 785],
    pub fd_synth: *mut FLOAT32,
    pub bpf_active_prev: WORD32,
    pub last_tcx_pitch: WORD32,
    pub synth_prev_ec: [FLOAT32; 16],
}
pub type ia_usac_lpd_decoder_handle = *mut ia_usac_lpd_decoder;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_usac_data_main_struct {
    pub time_sample_vector: [[FLOAT32; 4096]; 6],
    pub time_sample_vector_prev: [[FLOAT32; 4096]; 6],
    pub input_data_ptr: [[WORD32; 4096]; 6],
    pub overlap_data_ptr: [[WORD32; 4096]; 6],
    pub output_data_ptr: [[WORD32; 4096]; 6],
    pub window_shape: [WORD32; 6],
    pub window_shape_prev: [WORD32; 6],
    pub window_sequence: [WORD32; 6],
    pub window_sequence_last: [WORD32; 6],
    pub output_samples: WORD32,
    pub sbr_ratio_idx: WORD32,
    pub usac_independency_flg: WORD32,
    pub sampling_rate_idx: WORD32,
    pub audio_object_type: WORD32,
    pub down_samp_sbr: WORD32,
    pub sbr_mode: WORD32,
    pub tw_mdct: [WORD32; 68],
    pub mps_pseudo_lr: [WORD32; 68],
    pub td_frame_prev: [WORD32; 6],
    pub warp_sum: [[FLOAT32; 2]; 6],
    pub warp_cont_mem: [[FLOAT32; 3072]; 6],
    pub prev_sample_pos: [[FLOAT32; 3072]; 6],
    pub prev_tw_trans_len: [[FLOAT32; 2]; 6],
    pub prev_tw_start_stop: [[WORD32; 2]; 6],
    pub prev_warped_time_sample_vector: [[FLOAT32; 3072]; 6],
    pub lpc_prev: [[FLOAT32; 17]; 6],
    pub acelp_in: [[FLOAT32; 257]; 6],
    pub alpha_q_re: [[WORD32; 136]; 8],
    pub alpha_q_im: [[WORD32; 136]; 8],
    pub cplx_pred_used: [[UWORD8; 136]; 8],
    pub alpha_q_re_prev: [WORD32; 136],
    pub alpha_q_im_prev: [WORD32; 136],
    pub dmx_re_prev: [WORD32; 1024],
    pub sbr_scratch_mem_base: *mut core::ffi::c_void,
    pub coef_fix: [*mut WORD32; 6],
    pub coef: [*mut FLOAT32; 6],
    pub ms_used: [*mut UWORD8; 6],
    pub coef_save: [*mut WORD32; 6],
    pub factors: [*mut WORD16; 6],
    pub group_dis: [*mut UWORD8; 6],
    pub tw_data_present: [WORD32; 6],
    pub tw_ratio: [*mut WORD32; 6],
    pub pstr_tns: [*mut ia_tns_frame_info_struct; 6],
    pub str_tddec: [ia_usac_lpd_decoder_handle; 6],
    pub arith_prev_n: [WORD32; 6],
    pub c_prev: [[WORD8; 516]; 6],
    pub c: [[WORD8; 516]; 6],
    pub noise_filling_config: [WORD32; 16],
    pub seed_value: [UWORD32; 6],
    pub present_chan: WORD32,
    pub fac_data_present: [WORD32; 6],
    pub fac_data: [[WORD32; 129]; 6],
    pub pstr_sfb_info: [*mut ia_sfb_info_struct; 6],
    pub str_only_long_info: ia_sfb_info_struct,
    pub str_eight_short_info: ia_sfb_info_struct,
    pub pstr_usac_winmap: [*mut ia_sfb_info_struct; 5],
    pub sfb_width_short: [WORD16; 16],
    pub ccfl: WORD32,
    pub len_subfrm: WORD32,
    pub num_subfrm: WORD32,
    pub pstr_esbr_dec: ia_handle_sbr_dec_inst_struct,
    pub esbr_bit_str: [ia_aac_dec_sbr_bitstream_struct; 2],
    pub x_ac_dec: [WORD32; 1024],
    pub scratch_buffer: [WORD32; 1024],
    pub synth_buf: [FLOAT32; 1883],
    pub exc_buf: [FLOAT32; 1453],
    pub lp_flt_coff: [FLOAT32; 290],
    pub pitch: [WORD32; 25],
    pub pitch_gain: [FLOAT32; 25],
    pub huffman_code_book_scl: *mut UWORD16,
    pub huffman_code_book_scl_index: *mut UWORD32,
    pub tns_coeff3_32: *mut WORD32,
    pub tns_coeff4_32: *mut WORD32,
    pub tns_max_bands_tbl_usac: *mut [[WORD32; 2]; 16],
    pub sfb_width_long: [WORD16; 64],
    pub usac_flag: WORD32,
    pub arr_coef_fix: [[WORD32; 1152]; 6],
    pub arr_coef: [[FLOAT32; 1152]; 6],
    pub arr_coef_save: [[WORD32; 1152]; 6],
    pub arr_factors: [[WORD16; 128]; 6],
    pub arr_group_dis: [[UWORD8; 8]; 6],
    pub arr_tw_ratio: [[WORD32; 16]; 6],
    pub arr_ms_used: [[UWORD8; 128]; 6],
    pub arr_str_tddec: [ia_usac_lpd_decoder; 6],
    pub arr_str_tns: [ia_tns_frame_info_struct; 6],
    pub enh_sbr: WORD32,
    pub esbr_hq: WORD32,
    pub enh_sbr_ps: WORD32,
    pub drc_config_changed: WORD32,
    pub core_mode: WORD32,
    pub frame_ok: WORD32,
    pub sbr_parse_err_flag: WORD32,
    pub last_frame_ok: WORD32,
    pub ec_flag: WORD32,
    pub first_frame: WORD32,
    pub sbr_parse_complete: WORD32,
    pub max_sfb: [UWORD8; 2],
    pub num_ch_out: WORD32,
    pub spec_scale: [[WORD16; 128]; 6],
    pub str_error_concealment: [ia_ec_state_str; 6],
    pub pstr_td_frame: *mut ia_td_frame_data_struct,
    pub sampling_rate: WORD32,
    pub td_frame_prev_ec: [WORD32; 6],
    pub lsp_coeff: [[FLOAT32; 16]; 5],
    pub lsf_adaptive_mean_cand: [FLOAT32; 16],
    pub lsf_adaptive_mean: [FLOAT32; 16],
    pub lpc4_lsf: [FLOAT32; 16],
    pub bpf_control_info: WORD32,
    pub first_lpd_flag: WORD32,
    pub short_fac_flag: WORD32,
    pub core_mode_last: WORD32,
    pub stability_factor_old: FLOAT32,
    pub num_lost_lpd_frames: [WORD32; 6],
    pub pitch_lag_old: WORD32,
    pub pitch_lag_frac_old: WORD32,
    pub pitch_lag: WORD32,
    pub pitch_lag_frac: WORD32,
    pub seed_ace: WORD16,
    pub pstr_ec_state: *mut ia_ec_state_str,
    pub past_pitch_gain: FLOAT32,
    pub past_gain_code: FLOAT32,
    pub past_gain_tcx: [FLOAT32; 6],
    pub tcx_spec_coeffs: [[WORD32; 1280]; 6],
    pub lspold_ec: [FLOAT32; 16],
    pub lp_flt_coff_a_ec: [FLOAT32; 17],
    pub td_frame_data_prev: [ia_td_frame_data_struct; 6],
    pub last_shiftp: WORD32,
}
pub type ia_usac_data_struct = ia_usac_data_main_struct;
pub const LEN_SUPERFRAME: core::ffi::c_int = 1024 as core::ffi::c_int;
pub const LEN_FRAME: core::ffi::c_int = 256 as core::ffi::c_int;
pub const NUM_FRAMES: core::ffi::c_int = 4 as core::ffi::c_int;
pub const ORDER: core::ffi::c_int = 16 as core::ffi::c_int;
pub const FAC_LENGTH: core::ffi::c_int = 128 as core::ffi::c_int;
pub const NUM_SUBFR_SUPERFRAME: core::ffi::c_int = 16 as core::ffi::c_int;
pub const NUM_SUBFR_SUPERFRAME_BY2: core::ffi::c_int = NUM_SUBFR_SUPERFRAME
    / 2 as core::ffi::c_int;
pub const SYNTH_DELAY_LMAX: core::ffi::c_int = (NUM_SUBFR_SUPERFRAME_BY2
    - 1 as core::ffi::c_int) * LEN_SUBFR;
pub const FSCALE_DENOM: core::ffi::c_int = 12800 as core::ffi::c_int;
pub const FAC_FSCALE_MAX: core::ffi::c_int = 24000 as core::ffi::c_int;
pub const FILTER_DELAY: core::ffi::c_int = 12 as core::ffi::c_int;
pub const TMIN: core::ffi::c_int = 34 as core::ffi::c_int;
pub const TMAX: core::ffi::c_int = 27 as core::ffi::c_int + 6 as core::ffi::c_int * TMIN;
pub const INTER_LP_FIL_ORDER: core::ffi::c_int = 16 as core::ffi::c_int;
pub const MAX_PITCH: core::ffi::c_int = TMAX
    + 6 as core::ffi::c_int
        * ((FAC_FSCALE_MAX * TMIN + FSCALE_DENOM / 2 as core::ffi::c_int) / FSCALE_DENOM
            - TMIN);
pub const PREEMPH_FILT_FAC: core::ffi::c_float = 0.68f32;
pub const WIN_SEL_0: core::ffi::c_int = 0 as core::ffi::c_int;
pub const EIGHT_SHORT_SEQUENCE: core::ffi::c_int = 2 as core::ffi::c_int;
pub const LEN_SUBFR: core::ffi::c_int = 64 as core::ffi::c_int;
pub const PI: core::ffi::c_double = 3.14159265358979323846264338327950288f64;
pub const FREQ_MAX_F: core::ffi::c_float = 6400.0f32;
#[no_mangle]
pub static mut ixheaacd_pow_10_i_by_128: [WORD32; 128] = [
    16384 as core::ffi::c_int,
    17788 as core::ffi::c_int,
    19312 as core::ffi::c_int,
    20968 as core::ffi::c_int,
    22765 as core::ffi::c_int,
    24716 as core::ffi::c_int,
    26835 as core::ffi::c_int,
    29135 as core::ffi::c_int,
    31632 as core::ffi::c_int,
    34343 as core::ffi::c_int,
    37287 as core::ffi::c_int,
    40483 as core::ffi::c_int,
    43953 as core::ffi::c_int,
    47720 as core::ffi::c_int,
    51810 as core::ffi::c_int,
    56251 as core::ffi::c_int,
    61072 as core::ffi::c_int,
    66307 as core::ffi::c_int,
    71990 as core::ffi::c_int,
    78161 as core::ffi::c_int,
    84860 as core::ffi::c_int,
    92134 as core::ffi::c_int,
    100030 as core::ffi::c_int,
    108604 as core::ffi::c_int,
    117913 as core::ffi::c_int,
    128019 as core::ffi::c_int,
    138992 as core::ffi::c_int,
    150905 as core::ffi::c_int,
    163840 as core::ffi::c_int,
    177882 as core::ffi::c_int,
    193129 as core::ffi::c_int,
    209682 as core::ffi::c_int,
    227654 as core::ffi::c_int,
    247167 as core::ffi::c_int,
    268352 as core::ffi::c_int,
    291353 as core::ffi::c_int,
    316325 as core::ffi::c_int,
    343438 as core::ffi::c_int,
    372874 as core::ffi::c_int,
    404834 as core::ffi::c_int,
    439532 as core::ffi::c_int,
    477205 as core::ffi::c_int,
    518107 as core::ffi::c_int,
    562515 as core::ffi::c_int,
    610728 as core::ffi::c_int,
    663075 as core::ffi::c_int,
    719908 as core::ffi::c_int,
    781612 as core::ffi::c_int,
    848605 as core::ffi::c_int,
    921340 as core::ffi::c_int,
    1000309 as core::ffi::c_int,
    1086046 as core::ffi::c_int,
    1179133 as core::ffi::c_int,
    1280197 as core::ffi::c_int,
    1389925 as core::ffi::c_int,
    1509057 as core::ffi::c_int,
    1638400 as core::ffi::c_int,
    1778829 as core::ffi::c_int,
    1931294 as core::ffi::c_int,
    2096827 as core::ffi::c_int,
    2276549 as core::ffi::c_int,
    2471675 as core::ffi::c_int,
    2683525 as core::ffi::c_int,
    2913532 as core::ffi::c_int,
    3163255 as core::ffi::c_int,
    3434381 as core::ffi::c_int,
    3728745 as core::ffi::c_int,
    4048340 as core::ffi::c_int,
    4395328 as core::ffi::c_int,
    4772057 as core::ffi::c_int,
    5181075 as core::ffi::c_int,
    5625151 as core::ffi::c_int,
    6107289 as core::ffi::c_int,
    6630752 as core::ffi::c_int,
    7199081 as core::ffi::c_int,
    7816122 as core::ffi::c_int,
    8486051 as core::ffi::c_int,
    9213400 as core::ffi::c_int,
    10003091 as core::ffi::c_int,
    10860467 as core::ffi::c_int,
    11791330 as core::ffi::c_int,
    12801978 as core::ffi::c_int,
    13899250 as core::ffi::c_int,
    15090570 as core::ffi::c_int,
    16384000 as core::ffi::c_int,
    17788290 as core::ffi::c_int,
    19312945 as core::ffi::c_int,
    20968279 as core::ffi::c_int,
    22765494 as core::ffi::c_int,
    24716750 as core::ffi::c_int,
    26835250 as core::ffi::c_int,
    29135329 as core::ffi::c_int,
    31632551 as core::ffi::c_int,
    34343813 as core::ffi::c_int,
    37287459 as core::ffi::c_int,
    40483409 as core::ffi::c_int,
    43953287 as core::ffi::c_int,
    47720573 as core::ffi::c_int,
    51810757 as core::ffi::c_int,
    56251515 as core::ffi::c_int,
    61072895 as core::ffi::c_int,
    66307521 as core::ffi::c_int,
    71990813 as core::ffi::c_int,
    78161226 as core::ffi::c_int,
    84860513 as core::ffi::c_int,
    92134002 as core::ffi::c_int,
    100030911 as core::ffi::c_int,
    108604672 as core::ffi::c_int,
    117913300 as core::ffi::c_int,
    128019781 as core::ffi::c_int,
    138992500 as core::ffi::c_int,
    150905703 as core::ffi::c_int,
    163840000 as core::ffi::c_int,
    177882909 as core::ffi::c_int,
    193129453 as core::ffi::c_int,
    209682794 as core::ffi::c_int,
    227654941 as core::ffi::c_int,
    247167501 as core::ffi::c_int,
    268352504 as core::ffi::c_int,
    291353298 as core::ffi::c_int,
    316325515 as core::ffi::c_int,
    343438130 as core::ffi::c_int,
    372874596 as core::ffi::c_int,
    404834095 as core::ffi::c_int,
    439532879 as core::ffi::c_int,
    477205734 as core::ffi::c_int,
    518107571 as core::ffi::c_int,
    562515151 as core::ffi::c_int,
];
#[inline]
unsafe extern "C" fn ixheaacd_mult32_m(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 31 as core::ffi::c_int) as WORD32;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_reset_acelp_data_fix(
    mut usac_data: *mut ia_usac_data_struct,
    mut st: ia_usac_lpd_decoder_handle,
    mut ptr_overlap_buf: *mut WORD32,
    mut was_last_short: WORD32,
    mut tw_mdct: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    if was_last_short == 1 as core::ffi::c_int {
        (*st).mode_prev = -(2 as core::ffi::c_int) as WORD32;
    } else {
        (*st).mode_prev = -(1 as core::ffi::c_int) as WORD32;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < NUM_SUBFR_SUPERFRAME_BY2 - 1 as core::ffi::c_int {
        (*st).pitch_prev[i as usize] = 64 as core::ffi::c_int;
        (*st).gain_prev[i as usize] = 0 as core::ffi::c_int as core::ffi::c_float;
        i += 1;
    }
    (*st).bpf_active_prev = 0 as core::ffi::c_int as WORD32;
    if !ptr_overlap_buf.is_null() && tw_mdct == 0 {
        let mut ptr_window_coeff: *const WORD32 = 0 as *const WORD32;
        let mut fac_length: WORD32 = 0;
        if was_last_short != 0 {
            fac_length = ((*usac_data).ccfl as core::ffi::c_int / 16 as core::ffi::c_int)
                as WORD32;
        } else {
            fac_length = ((*usac_data).len_subfrm as core::ffi::c_int
                / 2 as core::ffi::c_int) as WORD32;
        }
        if fac_length == 48 as core::ffi::c_int {
            ptr_window_coeff = ixheaacd_sine_win_96.as_ptr();
        } else if fac_length == 64 as core::ffi::c_int {
            ptr_window_coeff = ixheaacd_sine_win_128.as_ptr();
        } else if fac_length == 96 as core::ffi::c_int {
            ptr_window_coeff = ixheaacd_sine_win_192.as_ptr();
        } else {
            ptr_window_coeff = ixheaacd_sine_win_256.as_ptr();
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < 2 as WORD32 * fac_length {
            *ptr_overlap_buf
                .offset(((*usac_data).ccfl / 2 as WORD32 - fac_length + i) as isize) = ixheaacd_mult32_m(
                *ptr_overlap_buf
                    .offset(((*usac_data).ccfl / 2 as WORD32 - fac_length + i) as isize),
                *ptr_window_coeff
                    .offset((2 as WORD32 * fac_length - 1 as WORD32 - i) as isize),
            );
            i += 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*usac_data).ccfl / 2 as WORD32 - fac_length {
            *ptr_overlap_buf
                .offset(((*usac_data).ccfl / 2 as WORD32 + fac_length + i) as isize) = 0
                as core::ffi::c_int as WORD32;
            i += 1;
        }
        if !ptr_overlap_buf.is_null() {
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*usac_data).len_subfrm / 2 as WORD32 - fac_length {
                (*st).exc_prev[i as usize] = 0.0f32 as FLOAT32;
                i += 1;
            }
            i = 0 as core::ffi::c_int as WORD32;
            while i
                < 2 as core::ffi::c_int * fac_length as core::ffi::c_int
                    + 1 as core::ffi::c_int
            {
                (*st)
                    .exc_prev[((*usac_data).len_subfrm / 2 as WORD32 - fac_length + i)
                    as usize] = (*ptr_overlap_buf
                    .offset(
                        (i as core::ffi::c_int
                            + (*usac_data).ccfl as core::ffi::c_int
                                / 2 as core::ffi::c_int - fac_length as core::ffi::c_int
                            - 1 as core::ffi::c_int) as isize,
                    ) as core::ffi::c_float
                    / 16384 as core::ffi::c_int as core::ffi::c_float) as FLOAT32;
                i += 1;
            }
        } else {
            ixheaacd_memset(
                ((*st).exc_prev).as_mut_ptr(),
                1 as WORD32 + 2 as WORD32 * FAC_LENGTH,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_fix2flt_data(
    mut usac_data: *mut ia_usac_data_struct,
    mut st: ia_usac_lpd_decoder_handle,
    mut k: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut fac_length: WORD32 = 0;
    let mut window_sequence_last: WORD32 = (*usac_data).window_sequence_last[k as usize];
    let mut p_ola_buffer: *mut WORD32 = ((*usac_data).overlap_data_ptr[k as usize])
        .as_mut_ptr();
    if window_sequence_last == EIGHT_SHORT_SEQUENCE {
        fac_length = ((*usac_data).ccfl as core::ffi::c_int / 16 as core::ffi::c_int)
            as WORD32;
    } else {
        fac_length = ((*usac_data).len_subfrm as core::ffi::c_int
            / 2 as core::ffi::c_int) as WORD32;
    }
    ixheaacd_memset(
        ((*st).lp_flt_coeff_a_prev).as_mut_ptr() as *mut FLOAT32,
        2 as WORD32 * (ORDER + 1 as WORD32),
    );
    ixheaacd_memset(
        ((*st).xcitation_prev).as_mut_ptr() as *mut FLOAT32,
        MAX_PITCH + INTER_LP_FIL_ORDER + 1 as WORD32,
    );
    ixheaacd_memset(
        ((*st).synth_prev).as_mut_ptr() as *mut FLOAT32,
        MAX_PITCH + SYNTH_DELAY_LMAX,
    );
    ixheaacd_memset(((*st).bpf_prev).as_mut_ptr(), FILTER_DELAY + LEN_SUBFR);
    (*st).gain_threshold = 0.0f32 as FLOAT32;
    if !p_ola_buffer.is_null() {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*usac_data).len_subfrm / 2 as WORD32 - fac_length {
            (*st).exc_prev[i as usize] = 0 as core::ffi::c_int as FLOAT32;
            i += 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i
            < 2 as core::ffi::c_int * fac_length as core::ffi::c_int
                + 1 as core::ffi::c_int
        {
            (*st)
                .exc_prev[((*usac_data).len_subfrm / 2 as WORD32 - fac_length + i)
                as usize] = (*p_ola_buffer
                .offset(
                    (i as core::ffi::c_int
                        + (*usac_data).ccfl as core::ffi::c_int / 2 as core::ffi::c_int
                        - fac_length as core::ffi::c_int - 1 as core::ffi::c_int)
                        as isize,
                ) as core::ffi::c_double / 16384.0f64) as FLOAT32;
            i += 1;
        }
    } else {
        ixheaacd_memset(
            ((*st).exc_prev).as_mut_ptr(),
            1 as WORD32 + 2 as WORD32 * FAC_LENGTH,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_init_acelp_data(
    mut usac_data: *mut ia_usac_data_struct,
    mut st: ia_usac_lpd_decoder_handle,
) -> VOID {
    ixheaacd_reset_acelp_data_fix(
        usac_data,
        st,
        0 as *mut WORD32,
        0 as WORD32,
        0 as WORD32,
    );
}
pub const PI_BY_6400: core::ffi::c_double = PI / 6400.0f64;
pub const SCALE1: core::ffi::c_double = 6400.0f64 / PI;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_lsp_2_lsf_conversion(
    mut lsp: *mut core::ffi::c_float,
    mut lsf: *mut core::ffi::c_float,
    mut m: WORD32,
) {
    let mut i: core::ffi::c_short = 0;
    i = 0 as core::ffi::c_short;
    while (i as core::ffi::c_int) < m {
        *lsf.offset(i as isize) = (acos(*lsp.offset(i as isize) as core::ffi::c_double)
            * SCALE1) as core::ffi::c_float;
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_lsf_2_lsp_conversion_float(
    mut lsf: *mut FLOAT32,
    mut lsp: *mut FLOAT32,
    mut m: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < m {
        *lsp.offset(i as isize) = cos(
            *lsf.offset(i as isize) as core::ffi::c_double * PI_BY_6400,
        ) as FLOAT32;
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_bass_post_filter(
    mut synth_sig: *mut FLOAT32,
    mut pitch: *mut WORD32,
    mut pitch_gain: *mut FLOAT32,
    mut synth_out: *mut FLOAT32,
    mut len_fr: WORD32,
    mut len2: WORD32,
    mut bpf_prev: *mut FLOAT32,
    mut ec_flag: WORD32,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut sf: WORD32 = 0;
    let mut num_subfr: WORD32 = 0;
    let mut pitch_lag: WORD32 = 0;
    let mut lg: WORD32 = 0;
    let mut x_energy: FLOAT32 = 0.;
    let mut xy_corr: FLOAT32 = 0.;
    let mut y_energy: FLOAT32 = 0.;
    let mut norm_corr: FLOAT32 = 0.;
    let mut energy: FLOAT32 = 0.;
    let mut gain: FLOAT32 = 0.;
    let mut tmp: FLOAT32 = 0.;
    let mut alpha: FLOAT32 = 0.;
    let mut noise_buf: [FLOAT32; 140] = [0.; 140];
    let mut noise_tmp1: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut noise_tmp2: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut x: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut y: *mut FLOAT32 = 0 as *mut FLOAT32;
    noise_tmp1 = noise_buf.as_mut_ptr().offset(FILTER_DELAY as isize);
    noise_tmp2 = noise_buf
        .as_mut_ptr()
        .offset(FILTER_DELAY as isize)
        .offset(LEN_SUBFR as isize);
    memcpy(
        synth_out as *mut core::ffi::c_void,
        synth_sig.offset(-(LEN_SUBFR as isize)) as *const core::ffi::c_void,
        (len_fr as size_t).wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
    );
    if len_fr as core::ffi::c_int % 64 as core::ffi::c_int != 0 {
        memset(
            synth_out.offset(len_fr as isize) as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ((LEN_SUBFR - len_fr as core::ffi::c_int % 64 as core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
    }
    sf = 0 as core::ffi::c_int as WORD32;
    num_subfr = 0 as core::ffi::c_int as WORD32;
    while num_subfr < len_fr {
        pitch_lag = *pitch.offset(sf as isize);
        gain = *pitch_gain.offset(sf as isize);
        if (pitch_lag >> 1 as core::ffi::c_int) + 96 as WORD32 - num_subfr > MAX_PITCH {
            if ec_flag != 0 {
                pitch_lag = ((MAX_PITCH + num_subfr as core::ffi::c_int
                    - 96 as core::ffi::c_int) << 1 as core::ffi::c_int) as WORD32;
            } else {
                return -(1 as WORD32)
            }
        }
        if gain > 1.0f32 {
            gain = 1.0f32 as FLOAT32;
        }
        if gain < 0.0f32 {
            gain = 0.0f32 as FLOAT32;
        }
        x = &mut *synth_sig
            .offset((num_subfr as core::ffi::c_int - 96 as core::ffi::c_int) as isize)
            as *mut FLOAT32;
        y = &mut *synth_sig
            .offset(
                (num_subfr as core::ffi::c_int
                    - pitch_lag as core::ffi::c_int / 2 as core::ffi::c_int
                    - 96 as core::ffi::c_int) as isize,
            ) as *mut FLOAT32;
        x_energy = 0.01f32 as FLOAT32;
        xy_corr = 0.01f32 as FLOAT32;
        y_energy = 0.01f32 as FLOAT32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < LEN_SUBFR + 96 as core::ffi::c_int {
            x_energy += *x.offset(i as isize) * *x.offset(i as isize);
            xy_corr += *x.offset(i as isize) * *y.offset(i as isize);
            y_energy += *y.offset(i as isize) * *y.offset(i as isize);
            i += 1;
        }
        norm_corr = xy_corr
            / sqrt((x_energy * y_energy) as core::ffi::c_double) as FLOAT32;
        if norm_corr > 0.95f32 {
            pitch_lag >>= 1 as core::ffi::c_int;
        }
        lg = len_fr + len2 - pitch_lag - num_subfr;
        if lg < 0 as core::ffi::c_int {
            lg = 0 as core::ffi::c_int as WORD32;
        }
        if lg > LEN_SUBFR {
            lg = LEN_SUBFR as WORD32;
        }
        if pitch_lag > MAX_PITCH {
            if ec_flag != 0 {
                pitch_lag = MAX_PITCH as WORD32;
            } else {
                return -(1 as WORD32)
            }
        }
        if gain > 0 as core::ffi::c_int as FLOAT32 {
            if lg > 0 as core::ffi::c_int {
                tmp = 0.01f32 as FLOAT32;
                i = 0 as core::ffi::c_int as WORD32;
                while i < lg {
                    tmp
                        += *synth_sig.offset((i + num_subfr) as isize)
                            * *synth_sig.offset((i + num_subfr) as isize);
                    i += 1;
                }
                energy = 0.01f32 as FLOAT32;
                i = 0 as core::ffi::c_int as WORD32;
                while i < lg {
                    energy
                        += *synth_sig.offset((i + num_subfr + pitch_lag) as isize)
                            * *synth_sig.offset((i + num_subfr + pitch_lag) as isize);
                    i += 1;
                }
                tmp = sqrt((tmp / energy) as core::ffi::c_double) as FLOAT32;
                if tmp < gain {
                    gain = tmp;
                }
            }
            alpha = 0.5f32 * gain;
            i = 0 as core::ffi::c_int as WORD32;
            while i < lg {
                *noise_tmp2.offset(i as isize) = alpha
                    * (*synth_sig.offset((i + num_subfr) as isize)
                        - 0.5f32
                            * *synth_sig.offset((i + num_subfr - pitch_lag) as isize)
                        - 0.5f32
                            * *synth_sig.offset((i + num_subfr + pitch_lag) as isize));
                i += 1;
            }
            i = lg;
            while i < LEN_SUBFR {
                *noise_tmp2.offset(i as isize) = alpha
                    * (*synth_sig.offset((i + num_subfr) as isize)
                        - *synth_sig.offset((i + num_subfr - pitch_lag) as isize));
                i += 1;
            }
        } else {
            memset(
                noise_tmp2 as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                (LEN_SUBFR as size_t)
                    .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
            );
        }
        memcpy(
            noise_buf.as_mut_ptr() as *mut core::ffi::c_void,
            bpf_prev as *const core::ffi::c_void,
            ((FILTER_DELAY + LEN_SUBFR) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        memcpy(
            bpf_prev as *mut core::ffi::c_void,
            noise_buf.as_mut_ptr().offset(LEN_SUBFR as isize)
                as *const core::ffi::c_void,
            ((FILTER_DELAY + LEN_SUBFR) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        i = 0 as core::ffi::c_int as WORD32;
        while i < LEN_SUBFR {
            tmp = ixheaacd_fir_lp_filt[0 as core::ffi::c_int as usize]
                * *noise_tmp1.offset(i as isize);
            j = 1 as core::ffi::c_int as WORD32;
            while j <= FILTER_DELAY {
                tmp
                    += ixheaacd_fir_lp_filt[j as usize]
                        * (*noise_tmp1.offset((i - j) as isize)
                            + *noise_tmp1.offset((i + j) as isize));
                j += 1;
            }
            *synth_out.offset((i + num_subfr) as isize) -= tmp;
            i += 1;
        }
        num_subfr += LEN_SUBFR;
        sf += 1;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_reorder_lsf(
    mut lsf: *mut core::ffi::c_float,
    mut min_dist: core::ffi::c_float,
    mut n: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut lsf_min: core::ffi::c_float = 0.;
    lsf_min = min_dist;
    i = 0 as core::ffi::c_int;
    while i < n {
        if *lsf.offset(i as isize) < lsf_min {
            *lsf.offset(i as isize) = lsf_min;
        }
        lsf_min = *lsf.offset(i as isize) + min_dist;
        i += 1;
    }
    lsf_min = FREQ_MAX_F - min_dist;
    i = n - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        if *lsf.offset(i as isize) > lsf_min {
            *lsf.offset(i as isize) = lsf_min;
        }
        lsf_min = *lsf.offset(i as isize) - min_dist;
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_lpd_dec(
    mut usac_data: *mut ia_usac_data_struct,
    mut st: ia_usac_lpd_decoder_handle,
    mut pstr_td_frame_data: *mut ia_td_frame_data_struct,
    mut fsynth: *mut FLOAT32,
    mut first_lpd_flag: WORD32,
    mut short_fac_flag: WORD32,
    mut bpf_control_info: WORD32,
) -> WORD32 {
    let mut synth_buf: *mut FLOAT32 = ((*usac_data).synth_buf).as_mut_ptr();
    let mut xcitation_buff: *mut FLOAT32 = ((*usac_data).exc_buf).as_mut_ptr();
    let mut lsp_curr: [FLOAT32; 16] = [0.; 16];
    let mut lsf_curr: [FLOAT32; 16] = [0.; 16];
    let mut lp_flt_coff_a: *mut FLOAT32 = ((*usac_data).lp_flt_coff).as_mut_ptr();
    let mut synth: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut xcitation_curr: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut pitch: *mut WORD32 = ((*usac_data).pitch).as_mut_ptr();
    let mut pitch_gain: *mut FLOAT32 = ((*usac_data).pitch_gain).as_mut_ptr();
    let mut lsf_flt: [FLOAT32; 144] = [0.; 144];
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut tp: WORD32 = 0;
    let mut mode: WORD32 = 0;
    let mut mod_0: *mut WORD32 = 0 as *mut WORD32;
    let mut gain: FLOAT32 = 0.;
    let mut stability_factor: FLOAT32 = 0.0f32;
    let mut tmp: FLOAT32 = 0.;
    let mut synth_corr: FLOAT32 = 0.;
    let mut synth_energy: FLOAT32 = 0.;
    let mut len_fr: WORD32 = 0;
    let mut len_subfrm: WORD32 = 0;
    let mut num_subfr: WORD32 = 0;
    let mut num_subfr_in_superfr: WORD32 = 0;
    let mut num_subfr_by2: WORD32 = 0;
    let mut synth_delay: WORD32 = 0;
    let mut num_samples: WORD32 = 0 as WORD32;
    let mut ptr_scratch: *mut WORD32 = &mut *((*usac_data).scratch_buffer)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
    let mut subfr_len: WORD32 = 0 as WORD32;
    let mut n_subfr: WORD32 = 0 as WORD32;
    let mut err: WORD32 = 0 as WORD32;
    let mut ch: WORD32 = (*usac_data).present_chan;
    len_fr = (*usac_data).ccfl;
    len_subfrm = (*usac_data).len_subfrm;
    num_subfr = (*usac_data).num_subfrm;
    num_subfr_in_superfr = NUM_FRAMES * num_subfr;
    num_subfr_by2 = (num_subfr_in_superfr as core::ffi::c_int / 2 as core::ffi::c_int
        - 1 as core::ffi::c_int) as WORD32;
    synth_delay = (num_subfr_by2 as core::ffi::c_int * LEN_SUBFR) as WORD32;
    synth = synth_buf.offset(MAX_PITCH as isize).offset(synth_delay as isize);
    ixheaacd_mem_cpy(
        ((*st).synth_prev).as_mut_ptr() as *const FLOAT32,
        synth_buf as *mut FLOAT32,
        MAX_PITCH + synth_delay,
    );
    ixheaacd_memset(synth, SYNTH_DELAY_LMAX + LEN_SUPERFRAME - synth_delay);
    xcitation_curr = xcitation_buff
        .offset(MAX_PITCH as isize)
        .offset(INTER_LP_FIL_ORDER as isize)
        .offset(1 as core::ffi::c_int as isize);
    ixheaacd_mem_cpy(
        ((*st).xcitation_prev).as_mut_ptr() as *const FLOAT32,
        xcitation_buff as *mut FLOAT32,
        MAX_PITCH + INTER_LP_FIL_ORDER + 1 as WORD32,
    );
    memset(
        xcitation_curr as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<FLOAT32>() as size_t)
            .wrapping_mul((LEN_SUPERFRAME + 1 as core::ffi::c_int) as size_t),
    );
    mod_0 = ((*pstr_td_frame_data).mod_0).as_mut_ptr();
    if (*usac_data).frame_ok == 1 as core::ffi::c_int {
        (*usac_data).num_lost_lpd_frames[(*usac_data).present_chan as usize] = 0
            as core::ffi::c_int as WORD32;
    }
    if (*usac_data).ec_flag != 0 && (*usac_data).frame_ok == 0 as core::ffi::c_int {
        ixheaacd_usac_lpc_ec(
            ((*usac_data).lsp_coeff).as_mut_ptr(),
            ((*usac_data).lpc4_lsf).as_mut_ptr(),
            ((*usac_data).lsf_adaptive_mean).as_mut_ptr(),
            first_lpd_flag,
        );
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_subfr_by2 {
        *pitch.offset(i as isize) = (*st).pitch_prev[i as usize] as WORD32;
        *pitch_gain.offset(i as isize) = (*st).gain_prev[i as usize] as FLOAT32;
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_subfr_in_superfr {
        *pitch.offset((i + num_subfr_by2) as isize) = 64 as core::ffi::c_int as WORD32;
        *pitch_gain.offset((i + num_subfr_by2) as isize) = 0.0f32 as FLOAT32;
        i += 1;
    }
    if (*usac_data).frame_ok != 0 {
        if first_lpd_flag == 0 {
            ixheaacd_lsp_2_lsf_conversion(
                ((*st).lspold).as_mut_ptr() as *mut core::ffi::c_float,
                lsf_flt.as_mut_ptr() as *mut core::ffi::c_float,
                ORDER,
            );
        }
        ixheaacd_alg_vec_dequant(
            pstr_td_frame_data,
            first_lpd_flag,
            lsf_flt.as_mut_ptr(),
            ((*pstr_td_frame_data).mod_0).as_mut_ptr(),
            (*usac_data).ec_flag,
        );
    }
    if (*usac_data).ec_flag != 0 && (*usac_data).frame_ok == 0 {
        i = 0 as core::ffi::c_int as WORD32;
        while i < 5 as core::ffi::c_int {
            memcpy(
                &mut *lsf_flt
                    .as_mut_ptr()
                    .offset((i as core::ffi::c_int * ORDER) as isize) as *mut FLOAT32
                    as *mut core::ffi::c_void,
                &mut *((*usac_data).lsp_coeff).as_mut_ptr().offset(i as isize)
                    as *mut [FLOAT32; 16] as *const core::ffi::c_void,
                (ORDER as size_t)
                    .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
            );
            i += 1;
        }
    }
    if first_lpd_flag != 0 {
        ixheaacd_mem_cpy(
            &mut *lsf_flt.as_mut_ptr().offset(0 as core::ffi::c_int as isize)
                as *mut FLOAT32 as *const FLOAT32,
            ((*st).lsf_prev).as_mut_ptr(),
            ORDER,
        );
        ixheaacd_lsf_2_lsp_conversion_float(
            ((*st).lsf_prev).as_mut_ptr(),
            ((*st).lspold).as_mut_ptr(),
            ORDER,
        );
    }
    if first_lpd_flag != 0
        && *mod_0.offset(0 as core::ffi::c_int as isize) == 0 as core::ffi::c_int
        || first_lpd_flag != 0
            && *mod_0.offset(1 as core::ffi::c_int as isize) == 0 as core::ffi::c_int
        || first_lpd_flag != 0
            && *mod_0.offset(2 as core::ffi::c_int as isize) == 0 as core::ffi::c_int
            && len_subfrm != LEN_FRAME
    {
        let mut lp_flt_coeff_a: [FLOAT32; 153] = [0.; 153];
        let mut tmp_buf: [FLOAT32; 784] = [0.; 784];
        let mut tmp_res_buf: [FLOAT32; 768] = [0.; 768];
        let mut tmp_0: *mut FLOAT32 = &mut *tmp_buf
            .as_mut_ptr()
            .offset(LEN_FRAME as isize) as *mut FLOAT32;
        let mut ptr_tmp: *mut FLOAT32 = &mut *tmp_res_buf
            .as_mut_ptr()
            .offset(LEN_FRAME as isize) as *mut FLOAT32;
        let mut tmp_start: WORD32 = 0;
        let mut mem: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
        let mut gain_0: WORD32 = 0;
        let mut length: WORD32 = 0;
        ixheaacd_interpolation_lsp_params(
            ((*st).lspold).as_mut_ptr(),
            ((*st).lspold).as_mut_ptr(),
            lp_flt_coeff_a.as_mut_ptr(),
            8 as WORD32,
        );
        memcpy(
            ((*st).lp_flt_coeff_a_prev).as_mut_ptr() as *mut core::ffi::c_void,
            lp_flt_coeff_a.as_mut_ptr() as *const core::ffi::c_void,
            ((ORDER + 1 as core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        memcpy(
            ((*st).lp_flt_coeff_a_prev)
                .as_mut_ptr()
                .offset(ORDER as isize)
                .offset(1 as core::ffi::c_int as isize) as *mut core::ffi::c_void,
            lp_flt_coeff_a.as_mut_ptr() as *const core::ffi::c_void,
            ((ORDER + 1 as core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        if *mod_0.offset(0 as core::ffi::c_int as isize) == 0 as core::ffi::c_int {
            let mut fac_length: WORD32 = 0;
            if short_fac_flag != 0 {
                fac_length = (len_subfrm as core::ffi::c_int * NUM_FRAMES
                    / 16 as core::ffi::c_int) as WORD32;
            } else {
                fac_length = (len_subfrm as core::ffi::c_int / 2 as core::ffi::c_int)
                    as WORD32;
            }
            if (*usac_data).frame_ok == 0 as core::ffi::c_int {
                memset(
                    &mut *((*pstr_td_frame_data).fac_data)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize) as *mut WORD32
                        as *mut core::ffi::c_void,
                    0 as core::ffi::c_int,
                    ::core::mem::size_of::<[WORD32; 129]>() as size_t,
                );
            }
            gain_0 = ixheaacd_pow_10_i_by_128[(*pstr_td_frame_data)
                .fac_data[0 as core::ffi::c_int as usize] as usize];
            memcpy(
                ptr_scratch as *mut core::ffi::c_void,
                &mut *((*pstr_td_frame_data).fac_data)
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize) as *mut WORD32
                    as *const core::ffi::c_void,
                (129 as size_t).wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
            );
            i = 0 as core::ffi::c_int as WORD32;
            while i < fac_length as core::ffi::c_int / 2 as core::ffi::c_int {
                (*pstr_td_frame_data).fac_data[i as usize] = *ptr_scratch
                    .offset(
                        (2 as core::ffi::c_int * i as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    ) << 16 as core::ffi::c_int;
                (*pstr_td_frame_data)
                    .fac_data[(fac_length / 2 as WORD32 + i) as usize] = *ptr_scratch
                    .offset((fac_length - 2 as WORD32 * i) as isize)
                    << 16 as core::ffi::c_int;
                i += 1;
            }
            if (*usac_data).ec_flag == 0 as core::ffi::c_int {
                if fac_length as core::ffi::c_int
                    & fac_length as core::ffi::c_int - 1 as core::ffi::c_int != 0
                {
                    if fac_length != 48 as core::ffi::c_int
                        && fac_length != 96 as core::ffi::c_int
                        && fac_length != 192 as core::ffi::c_int
                        && fac_length != 384 as core::ffi::c_int
                        && fac_length != 768 as core::ffi::c_int
                    {
                        return -(1 as WORD32);
                    }
                }
            }
            ixheaacd_fwd_alias_cancel_tool(
                usac_data,
                pstr_td_frame_data,
                fac_length,
                lp_flt_coeff_a.as_mut_ptr(),
                gain_0,
            );
            memset(
                &mut *(*((*usac_data).overlap_data_ptr)
                    .as_mut_ptr()
                    .offset((*usac_data).present_chan as isize))
                    .as_mut_ptr()
                    .offset(
                        (len_fr as core::ffi::c_int / 2 as core::ffi::c_int) as isize,
                    ) as *mut WORD32 as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                (fac_length as size_t)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
            );
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < 2 as WORD32 * len_subfrm {
            *((*st).fd_synth).offset((ORDER + i) as isize) = ((*usac_data)
                .overlap_data_ptr[(*usac_data).present_chan as usize][i as usize]
                as FLOAT32 as core::ffi::c_double / 16384.0f64) as FLOAT32;
            i += 1;
        }
        num_samples = (if 2 as WORD32 * len_subfrm
            < 27 as WORD32 + 6 as WORD32 * 34 as WORD32
                + 6 as WORD32
                    * ((24000 as WORD32 * 34 as WORD32 + 12800 as WORD32 / 2 as WORD32)
                        / 12800 as WORD32 - 34 as WORD32) + synth_delay
        {
            2 as core::ffi::c_int * len_subfrm as core::ffi::c_int
        } else {
            27 as core::ffi::c_int + 6 as core::ffi::c_int * 34 as core::ffi::c_int
                + 6 as core::ffi::c_int
                    * ((24000 as core::ffi::c_int * 34 as core::ffi::c_int
                        + 12800 as core::ffi::c_int / 2 as core::ffi::c_int)
                        / 12800 as core::ffi::c_int - 34 as core::ffi::c_int)
                + synth_delay as core::ffi::c_int
        }) as WORD32;
        ixheaacd_mem_cpy(
            ((*st).fd_synth).offset(ORDER as isize) as *const FLOAT32,
            synth.offset(-((2 as WORD32 * len_subfrm) as isize)),
            2 as WORD32 * len_subfrm,
        );
        ixheaacd_preemphsis_tool_float(
            ((*st).fd_synth).offset(ORDER as isize),
            PREEMPH_FILT_FAC,
            2 as WORD32 * len_subfrm,
            mem,
        );
        ixheaacd_memset(tmp_0, ORDER);
        ixheaacd_mem_cpy(
            ((*st).fd_synth).offset(ORDER as isize) as *const FLOAT32,
            tmp_0.offset(ORDER as isize),
            2 as WORD32 * len_subfrm,
        );
        tmp_start = 0 as core::ffi::c_int as WORD32;
        ixheaacd_memset(
            ptr_tmp.offset(-(len_subfrm as isize)),
            3 as WORD32 * len_subfrm,
        );
        memset(
            (*st).fd_synth as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (ORDER as size_t).wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
        );
        length = ((2 as core::ffi::c_int * len_subfrm as core::ffi::c_int
            - tmp_start as core::ffi::c_int) / LEN_SUBFR) as WORD32;
        ixheaacd_residual_tool_float1(
            lp_flt_coeff_a.as_mut_ptr(),
            &mut *((*st).fd_synth).offset((ORDER + tmp_start) as isize),
            &mut *ptr_tmp.offset(tmp_start as isize),
            LEN_SUBFR,
            length,
        );
        if *mod_0.offset(0 as core::ffi::c_int as isize) != 0 as core::ffi::c_int
            && (len_subfrm == LEN_FRAME
                || *mod_0.offset(1 as core::ffi::c_int as isize)
                    != 0 as core::ffi::c_int)
        {
            num_samples = (if len_subfrm
                < 27 as core::ffi::c_int + 6 as core::ffi::c_int * 34 as core::ffi::c_int
                    + 6 as core::ffi::c_int
                        * ((24000 as core::ffi::c_int * 34 as core::ffi::c_int
                            + 12800 as core::ffi::c_int / 2 as core::ffi::c_int)
                            / 12800 as core::ffi::c_int - 34 as core::ffi::c_int)
                    + 16 as core::ffi::c_int + 1 as core::ffi::c_int
            {
                len_subfrm as core::ffi::c_int
            } else {
                27 as core::ffi::c_int + 6 as core::ffi::c_int * 34 as core::ffi::c_int
                    + 6 as core::ffi::c_int
                        * ((24000 as core::ffi::c_int * 34 as core::ffi::c_int
                            + 12800 as core::ffi::c_int / 2 as core::ffi::c_int)
                            / 12800 as core::ffi::c_int - 34 as core::ffi::c_int)
                    + 16 as core::ffi::c_int + 1 as core::ffi::c_int
            }) as WORD32;
        } else {
            num_samples = (if 2 as WORD32 * len_subfrm
                < 27 as core::ffi::c_int + 6 as core::ffi::c_int * 34 as core::ffi::c_int
                    + 6 as core::ffi::c_int
                        * ((24000 as core::ffi::c_int * 34 as core::ffi::c_int
                            + 12800 as core::ffi::c_int / 2 as core::ffi::c_int)
                            / 12800 as core::ffi::c_int - 34 as core::ffi::c_int)
                    + 16 as core::ffi::c_int + 1 as core::ffi::c_int
            {
                2 as core::ffi::c_int * len_subfrm as core::ffi::c_int
            } else {
                27 as core::ffi::c_int + 6 as core::ffi::c_int * 34 as core::ffi::c_int
                    + 6 as core::ffi::c_int
                        * ((24000 as core::ffi::c_int * 34 as core::ffi::c_int
                            + 12800 as core::ffi::c_int / 2 as core::ffi::c_int)
                            / 12800 as core::ffi::c_int - 34 as core::ffi::c_int)
                    + 16 as core::ffi::c_int + 1 as core::ffi::c_int
            }) as WORD32;
        }
        ixheaacd_mem_cpy(
            ptr_tmp
                .offset((2 as WORD32 * len_subfrm) as isize)
                .offset(-(num_samples as isize)) as *const FLOAT32,
            xcitation_curr.offset(-(num_samples as isize)),
            num_samples,
        );
    }
    k = 0 as core::ffi::c_int as WORD32;
    while k < 4 as core::ffi::c_int {
        if (*usac_data).ec_flag != 0 && (*usac_data).frame_ok == 0 as core::ffi::c_int {
            if *mod_0.offset(k as isize) != 0 as core::ffi::c_int
                && (*usac_data).frame_ok == 0 as core::ffi::c_int
                && (*usac_data)
                    .str_error_concealment[ch as usize]
                    .prev_frame_ok[0 as core::ffi::c_int as usize]
                    == 0 as core::ffi::c_int && k == 0 as core::ffi::c_int
            {
                memcpy(
                    ((*st).lspold).as_mut_ptr() as *mut core::ffi::c_void,
                    ((*usac_data).lspold_ec).as_mut_ptr() as *const core::ffi::c_void,
                    ::core::mem::size_of::<[FLOAT32; 16]>() as size_t,
                );
            }
            (*usac_data).num_lost_lpd_frames[(*usac_data).present_chan as usize] += 1;
        }
        mode = *mod_0.offset(k as isize);
        if (*st).mode_prev == 0 as core::ffi::c_int && mode > 0 as core::ffi::c_int
            && (k != 0 as core::ffi::c_int
                || (*st).bpf_active_prev == 1 as core::ffi::c_int)
        {
            i = k * num_subfr + num_subfr_by2;
            let ref mut fresh0 = *pitch.offset(i as isize);
            *fresh0 = *pitch
                .offset((i as core::ffi::c_int - 1 as core::ffi::c_int) as isize);
            *pitch.offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = *fresh0;
            let ref mut fresh1 = *pitch_gain.offset(i as isize);
            *fresh1 = *pitch_gain
                .offset((i as core::ffi::c_int - 1 as core::ffi::c_int) as isize);
            *pitch_gain
                .offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = *fresh1;
        }
        if (*usac_data).frame_ok == 0 as core::ffi::c_int {
            memcpy(
                lsf_curr.as_mut_ptr() as *mut core::ffi::c_void,
                &mut *lsf_flt
                    .as_mut_ptr()
                    .offset(
                        ((k as core::ffi::c_int + 1 as core::ffi::c_int) * ORDER)
                            as isize,
                    ) as *mut FLOAT32 as *const core::ffi::c_void,
                (ORDER as size_t)
                    .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
            );
        } else if mode == 0 as core::ffi::c_int || mode == 1 as core::ffi::c_int {
            memcpy(
                lsf_curr.as_mut_ptr() as *mut core::ffi::c_void,
                &mut *lsf_flt
                    .as_mut_ptr()
                    .offset(
                        ((k as core::ffi::c_int + 1 as core::ffi::c_int) * ORDER)
                            as isize,
                    ) as *mut FLOAT32 as *const core::ffi::c_void,
                (ORDER as size_t)
                    .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
            );
        } else if mode == 2 as core::ffi::c_int {
            memcpy(
                lsf_curr.as_mut_ptr() as *mut core::ffi::c_void,
                &mut *lsf_flt
                    .as_mut_ptr()
                    .offset(
                        ((k as core::ffi::c_int + 2 as core::ffi::c_int) * ORDER)
                            as isize,
                    ) as *mut FLOAT32 as *const core::ffi::c_void,
                (ORDER as size_t)
                    .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
            );
        } else {
            memcpy(
                lsf_curr.as_mut_ptr() as *mut core::ffi::c_void,
                &mut *lsf_flt
                    .as_mut_ptr()
                    .offset(
                        ((k as core::ffi::c_int + 4 as core::ffi::c_int) * ORDER)
                            as isize,
                    ) as *mut FLOAT32 as *const core::ffi::c_void,
                (ORDER as size_t)
                    .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
            );
        }
        ixheaacd_lsf_2_lsp_conversion_float(
            lsf_curr.as_mut_ptr(),
            lsp_curr.as_mut_ptr(),
            ORDER,
        );
        if (*usac_data).frame_ok != 0 {
            tmp = 0.0f32 as FLOAT32;
            i = 0 as core::ffi::c_int as WORD32;
            while i < ORDER {
                tmp
                    += (lsf_curr[i as usize] - (*st).lsf_prev[i as usize])
                        * (lsf_curr[i as usize] - (*st).lsf_prev[i as usize]);
                i += 1;
            }
            stability_factor = 1.25f32 - tmp as core::ffi::c_float / 400000.0f32;
            if stability_factor > 1.0f32 {
                stability_factor = 1.0f32 as FLOAT32;
            }
            if stability_factor < 0.0f32 {
                stability_factor = 0.0f32 as FLOAT32;
            }
            if (*usac_data).ec_flag != 0 {
                (*usac_data).stability_factor_old = stability_factor;
            }
        }
        if (*usac_data).ec_flag != 0 && (*usac_data).frame_ok == 0 {
            stability_factor = (*usac_data).stability_factor_old;
        }
        if (*usac_data).frame_ok == 0 as core::ffi::c_int {
            mode = (*st).mode_prev;
        }
        if (*usac_data).frame_ok == 1 as core::ffi::c_int
            && mode == 0 as core::ffi::c_int
            || (*usac_data).frame_ok == 0 as core::ffi::c_int
                && ((*st).mode_prev == 0 as core::ffi::c_int
                    || (*st).mode_prev == 1 as core::ffi::c_int)
        {
            ixheaacd_interpolation_lsp_params(
                ((*st).lspold).as_mut_ptr(),
                lsp_curr.as_mut_ptr(),
                lp_flt_coff_a as *mut FLOAT32,
                num_subfr,
            );
            if (*usac_data).frame_ok == 1 as core::ffi::c_int
                || (*usac_data).frame_ok == 0 as core::ffi::c_int
                    && (*st).mode_prev == 0 as core::ffi::c_int
            {
                ixheaacd_acelp_alias_cnx(
                    usac_data,
                    pstr_td_frame_data,
                    k,
                    lp_flt_coff_a as *mut FLOAT32,
                    stability_factor,
                    st,
                );
            }
            if (*usac_data).frame_ok == 0 as core::ffi::c_int
                && (*st).mode_prev == 1 as core::ffi::c_int
            {
                ixheaacd_usac_tcx_ec(
                    usac_data,
                    st,
                    lsp_curr.as_mut_ptr(),
                    k,
                    lp_flt_coff_a,
                );
            }
            if (*st).mode_prev != 0 as core::ffi::c_int && bpf_control_info != 0 {
                i = k * num_subfr + num_subfr_by2;
                *pitch
                    .offset((i as core::ffi::c_int - 1 as core::ffi::c_int) as isize) = *pitch
                    .offset(i as isize);
                *pitch_gain
                    .offset((i as core::ffi::c_int - 1 as core::ffi::c_int) as isize) = *pitch_gain
                    .offset(i as isize);
                if (*st).mode_prev != -(2 as core::ffi::c_int) {
                    *pitch
                        .offset(
                            (i as core::ffi::c_int - 2 as core::ffi::c_int) as isize,
                        ) = *pitch.offset(i as isize);
                    *pitch_gain
                        .offset(
                            (i as core::ffi::c_int - 2 as core::ffi::c_int) as isize,
                        ) = *pitch_gain.offset(i as isize);
                }
            }
            k += 1;
        } else {
            if mode == 1 as core::ffi::c_int {
                subfr_len = len_subfrm;
                n_subfr = num_subfr;
            } else if mode == 2 as core::ffi::c_int {
                subfr_len = len_subfrm << 1 as core::ffi::c_int;
                n_subfr = (num_subfr_in_superfr as core::ffi::c_int
                    / 2 as core::ffi::c_int) as WORD32;
            } else if mode == 3 as core::ffi::c_int {
                subfr_len = len_subfrm << 2 as core::ffi::c_int;
                n_subfr = num_subfr_in_superfr;
            } else if (*usac_data).frame_ok == 0 as core::ffi::c_int {
                mode = 3 as core::ffi::c_int as WORD32;
                subfr_len = len_subfrm << 2 as core::ffi::c_int;
                n_subfr = num_subfr_in_superfr;
            }
            ixheaacd_lpc_coef_gen(
                ((*st).lspold).as_mut_ptr(),
                lsp_curr.as_mut_ptr(),
                lp_flt_coff_a as *mut FLOAT32,
                n_subfr,
                ORDER,
            );
            err = ixheaacd_tcx_mdct(
                usac_data,
                pstr_td_frame_data,
                k,
                lp_flt_coff_a as *mut FLOAT32,
                subfr_len,
                st,
            );
            if err != 0 {
                return err;
            }
            if (*usac_data).frame_ok == 1 as core::ffi::c_int
                && k == 2 as core::ffi::c_int
            {
                memcpy(
                    ((*usac_data).lp_flt_coff_a_ec).as_mut_ptr()
                        as *mut core::ffi::c_void,
                    &mut *lp_flt_coff_a
                        .offset(
                            (k as core::ffi::c_int * (ORDER + 1 as core::ffi::c_int))
                                as isize,
                        ) as *mut FLOAT32 as *const core::ffi::c_void,
                    ::core::mem::size_of::<[FLOAT32; 17]>() as size_t,
                );
            }
            k
                += (1 as core::ffi::c_int)
                    << mode as core::ffi::c_int - 1 as core::ffi::c_int;
        }
        (*st).mode_prev = mode;
        if (*usac_data).frame_ok == 0 as core::ffi::c_int {
            memcpy(
                ((*usac_data).lspold_ec).as_mut_ptr() as *mut core::ffi::c_void,
                ((*st).lspold).as_mut_ptr() as *const core::ffi::c_void,
                ::core::mem::size_of::<[FLOAT32; 16]>() as size_t,
            );
        }
        ixheaacd_mem_cpy(
            lsp_curr.as_mut_ptr() as *const FLOAT32,
            ((*st).lspold).as_mut_ptr(),
            ORDER,
        );
        ixheaacd_mem_cpy(
            lsf_curr.as_mut_ptr() as *const FLOAT32,
            ((*st).lsf_prev).as_mut_ptr(),
            ORDER,
        );
    }
    ixheaacd_mem_cpy(
        xcitation_buff.offset(len_fr as isize) as *const FLOAT32,
        ((*st).xcitation_prev).as_mut_ptr() as *mut FLOAT32,
        MAX_PITCH + INTER_LP_FIL_ORDER + 1 as WORD32,
    );
    ixheaacd_mem_cpy(
        synth_buf.offset(len_fr as isize) as *const FLOAT32,
        ((*st).synth_prev).as_mut_ptr() as *mut FLOAT32,
        MAX_PITCH + synth_delay,
    );
    if bpf_control_info == 0 {
        if *mod_0.offset(0 as core::ffi::c_int as isize) != 0 as core::ffi::c_int
            && (*st).bpf_active_prev != 0
        {
            i = 2 as core::ffi::c_int as WORD32;
            while i < num_subfr_in_superfr {
                *pitch_gain.offset((num_subfr_by2 + i) as isize) = 0.0f32;
                i += 1;
            }
        } else {
            i = 0 as core::ffi::c_int as WORD32;
            while i < num_subfr_in_superfr {
                *pitch_gain.offset((num_subfr_by2 + i) as isize) = 0.0f32;
                i += 1;
            }
        }
    }
    (*st).bpf_active_prev = bpf_control_info;
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_subfr_by2 {
        (*st).pitch_prev[i as usize] = *pitch.offset((num_subfr_in_superfr + i) as isize)
            as core::ffi::c_int;
        (*st).gain_prev[i as usize] = *pitch_gain
            .offset((num_subfr_in_superfr + i) as isize) as core::ffi::c_float;
        i += 1;
    }
    synth = synth_buf.offset(MAX_PITCH as isize);
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_subfr_in_superfr {
        tp = *pitch.offset(i as isize);
        gain = *pitch_gain.offset(i as isize);
        if gain > 0.0f32 {
            synth_corr = 0.0f32 as FLOAT32;
            synth_energy = 1e-6f32 as FLOAT32;
            if i as core::ffi::c_int * LEN_SUBFR + LEN_SUBFR > LEN_SUPERFRAME
                || i * LEN_SUBFR + LEN_SUBFR - tp > LEN_SUPERFRAME
            {
                if (*usac_data).ec_flag != 0 {
                    tp = (LEN_SUPERFRAME - LEN_SUBFR - i as core::ffi::c_int * LEN_SUBFR)
                        as WORD32;
                } else {
                    return -(1 as WORD32)
                }
            }
            k = 0 as core::ffi::c_int as WORD32;
            while k < LEN_SUBFR {
                synth_corr
                    += *synth.offset((i * LEN_SUBFR + k) as isize)
                        * *synth.offset((i * LEN_SUBFR - tp + k) as isize);
                synth_energy
                    += *synth.offset((i * LEN_SUBFR - tp + k) as isize)
                        * *synth.offset((i * LEN_SUBFR - tp + k) as isize);
                k += 1;
            }
            *pitch_gain.offset(i as isize) = synth_corr / synth_energy;
        }
        i += 1;
    }
    if *mod_0.offset(3 as core::ffi::c_int as isize) == 0 as core::ffi::c_int {
        err = ixheaacd_bass_post_filter(
            synth,
            pitch,
            pitch_gain,
            fsynth as *mut FLOAT32,
            len_fr,
            synth_delay,
            ((*st).bpf_prev).as_mut_ptr(),
            (*usac_data).ec_flag,
        );
    } else {
        err = ixheaacd_bass_post_filter(
            synth,
            pitch,
            pitch_gain,
            fsynth as *mut FLOAT32,
            len_fr,
            synth_delay - len_subfrm / 2 as WORD32,
            ((*st).bpf_prev).as_mut_ptr(),
            (*usac_data).ec_flag,
        );
    }
    if err != 0 {
        return err;
    }
    if (*usac_data).ec_flag != 0 && (*usac_data).frame_ok != 0 {
        memcpy(
            ((*usac_data).lpc4_lsf).as_mut_ptr() as *mut core::ffi::c_void,
            ((*pstr_td_frame_data).lpc4_lsf).as_mut_ptr() as *const core::ffi::c_void,
            ::core::mem::size_of::<[FLOAT32; 16]>() as size_t,
        );
        memcpy(
            ((*usac_data).str_error_concealment[ch as usize].lsf4).as_mut_ptr()
                as *mut core::ffi::c_void,
            ((*usac_data).lpc4_lsf).as_mut_ptr() as *const core::ffi::c_void,
            ::core::mem::size_of::<[FLOAT32; 16]>() as size_t,
        );
        memcpy(
            ((*usac_data).lsf_adaptive_mean).as_mut_ptr() as *mut core::ffi::c_void,
            ((*pstr_td_frame_data).lsf_adaptive_mean_cand).as_mut_ptr()
                as *const core::ffi::c_void,
            ::core::mem::size_of::<[FLOAT32; 16]>() as size_t,
        );
    }
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_lpd_dec_update(
    mut tddec: ia_usac_lpd_decoder_handle,
    mut usac_data: *mut ia_usac_data_struct,
    mut i_ch: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut ptr_overlap: *mut WORD32 = &mut *(*((*usac_data).overlap_data_ptr)
        .as_mut_ptr()
        .offset(i_ch as isize))
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
    let mut len_fr: WORD32 = 0;
    let mut lpd_sbf_len: WORD32 = 0;
    let mut lpd_delay: WORD32 = 0;
    let mut num_subfr_by2: WORD32 = 0;
    let mut synth_delay: WORD32 = 0;
    let mut fac_length: WORD32 = 0;
    if (*usac_data).tw_mdct[0 as core::ffi::c_int as usize] != 0 {
        ptr_overlap = &mut *(*((*usac_data).overlap_data_ptr)
            .as_mut_ptr()
            .offset(i_ch as isize))
            .as_mut_ptr()
            .offset(
                ((*usac_data).ccfl as core::ffi::c_int / 2 as core::ffi::c_int) as isize,
            ) as *mut WORD32;
    }
    len_fr = (*usac_data).ccfl;
    lpd_sbf_len = (NUM_FRAMES * (*usac_data).num_subfrm as core::ffi::c_int
        / 2 as core::ffi::c_int) as WORD32;
    lpd_delay = (lpd_sbf_len as core::ffi::c_int * LEN_SUBFR) as WORD32;
    num_subfr_by2 = (lpd_sbf_len as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    synth_delay = (num_subfr_by2 as core::ffi::c_int * LEN_SUBFR) as WORD32;
    fac_length = ((*usac_data).len_subfrm as core::ffi::c_int / 2 as core::ffi::c_int)
        as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < LEN_SUBFR + synth_delay {
        *ptr_overlap.offset(i as isize) = ((*tddec)
            .synth_prev[(MAX_PITCH - 64 as WORD32 + i) as usize] as core::ffi::c_double
            * 16384.0f64) as WORD32;
        i += 1;
    }
    ptr_overlap = ptr_overlap.offset((LEN_SUBFR + synth_delay - fac_length) as isize);
    k = 0 as core::ffi::c_int as WORD32;
    while k < 2 as WORD32 * fac_length {
        *ptr_overlap.offset(k as isize) = ((*tddec)
            .exc_prev[(k as core::ffi::c_int + 1 as core::ffi::c_int) as usize]
            as core::ffi::c_double * 16384.0f64) as WORD32;
        k += 1;
    }
    ptr_overlap = &mut *(*((*usac_data).overlap_data_ptr)
        .as_mut_ptr()
        .offset(i_ch as isize))
        .as_mut_ptr()
        .offset((lpd_delay + fac_length) as isize) as *mut WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < len_fr - lpd_delay - fac_length {
        *ptr_overlap.offset(i as isize) = 0 as core::ffi::c_int as WORD32;
        i += 1;
    }
    (*usac_data).window_shape[i_ch as usize] = WIN_SEL_0 as WORD32;
    (*usac_data).window_sequence_last[i_ch as usize] = EIGHT_SHORT_SEQUENCE as WORD32;
    (*usac_data).td_frame_prev[i_ch as usize] = 1 as core::ffi::c_int as WORD32;
    if (*tddec).mode_prev == 0 as core::ffi::c_int {
        memmove(
            ((*usac_data).lpc_prev[i_ch as usize]).as_mut_ptr()
                as *mut core::ffi::c_void,
            &mut *((*tddec).lp_flt_coeff_a_prev)
                .as_mut_ptr()
                .offset((ORDER + 1 as core::ffi::c_int) as isize)
                as *mut core::ffi::c_float as *const core::ffi::c_void,
            ((ORDER + 1 as core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        memmove(
            ((*usac_data).acelp_in[i_ch as usize]).as_mut_ptr()
                as *mut core::ffi::c_void,
            ((*tddec).exc_prev).as_mut_ptr() as *const core::ffi::c_void,
            ((1 as core::ffi::c_int + 2 as core::ffi::c_int * FAC_LENGTH) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_lpd_bpf_fix(
    mut usac_data: *mut ia_usac_data_struct,
    mut is_short_flag: WORD32,
    mut out_buffer: *mut FLOAT32,
    mut st: ia_usac_lpd_decoder_handle,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut tp: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut synth_buf: [core::ffi::c_float; 1883] = [0.; 1883];
    let mut signal_out: [core::ffi::c_float; 1024] = [0.; 1024];
    let mut synth: *mut core::ffi::c_float = 0 as *mut core::ffi::c_float;
    let mut synth_corr: core::ffi::c_float = 0.;
    let mut synth_energy: core::ffi::c_float = 0.;
    let mut pitch: [WORD32; 11] = [0; 11];
    let mut pitch_gain: [core::ffi::c_float; 11] = [0.; 11];
    let mut len_fr: WORD32 = 0;
    let mut lpd_sbf_len: WORD32 = 0;
    let mut num_subfr_by2: WORD32 = 0;
    let mut synth_delay: WORD32 = 0;
    let mut err: WORD32 = 0 as WORD32;
    len_fr = (*usac_data).ccfl;
    lpd_sbf_len = (NUM_FRAMES * (*usac_data).num_subfrm as core::ffi::c_int
        / 2 as core::ffi::c_int) as WORD32;
    num_subfr_by2 = (lpd_sbf_len as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    synth_delay = (num_subfr_by2 as core::ffi::c_int * LEN_SUBFR) as WORD32;
    ixheaacd_memset(
        synth_buf.as_mut_ptr() as *mut FLOAT32,
        MAX_PITCH + synth_delay + len_fr,
    );
    ixheaacd_mem_cpy(
        ((*st).synth_prev).as_mut_ptr() as *const FLOAT32,
        synth_buf.as_mut_ptr() as *mut FLOAT32,
        MAX_PITCH + synth_delay,
    );
    ixheaacd_mem_cpy(
        out_buffer as *const FLOAT32,
        synth_buf
            .as_mut_ptr()
            .offset(MAX_PITCH as isize)
            .offset(-(64 as core::ffi::c_int as isize)),
        synth_delay + len_fr + 64 as WORD32,
    );
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_subfr_by2 {
        pitch[i as usize] = (*st).pitch_prev[i as usize] as WORD32;
        pitch_gain[i as usize] = (*st).gain_prev[i as usize];
        i += 1;
    }
    i = num_subfr_by2;
    while i < lpd_sbf_len as core::ffi::c_int + 3 as core::ffi::c_int {
        pitch[i as usize] = 64 as core::ffi::c_int as WORD32;
        pitch_gain[i as usize] = 0.0f32;
        i += 1;
    }
    if (*st).mode_prev == 0 as core::ffi::c_int {
        pitch[num_subfr_by2 as usize] = pitch[(num_subfr_by2 as core::ffi::c_int
            - 1 as core::ffi::c_int) as usize];
        pitch_gain[num_subfr_by2 as usize] = pitch_gain[(num_subfr_by2
            as core::ffi::c_int - 1 as core::ffi::c_int) as usize];
        if is_short_flag == 0 {
            pitch[(num_subfr_by2 as core::ffi::c_int + 1 as core::ffi::c_int)
                as usize] = pitch[num_subfr_by2 as usize];
            pitch_gain[(num_subfr_by2 as core::ffi::c_int + 1 as core::ffi::c_int)
                as usize] = pitch_gain[num_subfr_by2 as usize];
        }
    }
    synth = synth_buf.as_mut_ptr().offset(MAX_PITCH as isize);
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_subfr_by2 as core::ffi::c_int + 2 as core::ffi::c_int {
        tp = pitch[i as usize];
        if i as core::ffi::c_int * LEN_SUBFR + MAX_PITCH < tp {
            if (*usac_data).ec_flag == 0 as core::ffi::c_int {
                return -(1 as WORD32)
            } else {
                tp = (MAX_PITCH - i as core::ffi::c_int * LEN_SUBFR) as WORD32;
            }
        } else if i * LEN_SUBFR + MAX_PITCH - tp >= 1883 as core::ffi::c_int
            || i as core::ffi::c_int * LEN_SUBFR + LEN_SUBFR > LEN_SUPERFRAME
            || i * LEN_SUBFR + LEN_SUBFR - tp > LEN_SUPERFRAME
        {
            if (*usac_data).ec_flag == 0 as core::ffi::c_int {
                return -(1 as WORD32)
            } else {
                tp = (i as core::ffi::c_int * LEN_SUBFR + MAX_PITCH
                    - 1882 as core::ffi::c_int) as WORD32;
            }
        }
        if pitch_gain[i as usize] > 0.0f32 {
            synth_corr = 0.0f32;
            synth_energy = 1e-6f32;
            k = 0 as core::ffi::c_int as WORD32;
            while k < LEN_SUBFR {
                synth_corr
                    += *synth.offset((i * LEN_SUBFR + k) as isize)
                        * *synth.offset((i * LEN_SUBFR - tp + k) as isize);
                synth_energy
                    += *synth.offset((i * LEN_SUBFR - tp + k) as isize)
                        * *synth.offset((i * LEN_SUBFR - tp + k) as isize);
                k += 1;
            }
            pitch_gain[i as usize] = synth_corr / synth_energy;
        }
        i += 1;
    }
    err = ixheaacd_bass_post_filter(
        synth as *mut FLOAT32,
        pitch.as_mut_ptr(),
        pitch_gain.as_mut_ptr() as *mut FLOAT32,
        signal_out.as_mut_ptr() as *mut FLOAT32,
        (lpd_sbf_len + 2 as WORD32) * LEN_SUBFR + LEN_SUBFR,
        len_fr - (lpd_sbf_len + 4 as WORD32) * LEN_SUBFR,
        ((*st).bpf_prev).as_mut_ptr(),
        (*usac_data).ec_flag,
    );
    if err != 0 as core::ffi::c_int {
        return err;
    }
    ixheaacd_mem_cpy(
        signal_out.as_mut_ptr() as *const FLOAT32,
        out_buffer,
        (lpd_sbf_len + 2 as WORD32) * LEN_SUBFR + LEN_SUBFR,
    );
    return err;
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
