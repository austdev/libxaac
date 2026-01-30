extern "C" {
    pub type ia_sbr_dec_inst_struct;
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
    static ia_ec_interpolation_fac: [WORD16; 4];
    static ixheaacd_exc_fade_fac: [FLOAT32; 8];
    static lsf_init: [FLOAT32; 16];
    static ixheaacd_gamma_table: [FLOAT32; 17];
    fn ixheaacd_randomsign(seed: *mut UWORD32) -> FLOAT32;
    fn ixheaacd_synthesis_tool_float(
        a: *mut FLOAT32,
        x: *mut FLOAT32,
        y: *mut FLOAT32,
        l: WORD32,
        mem: *mut FLOAT32,
    ) -> VOID;
    fn ixheaacd_deemphsis_tool(signal: *mut FLOAT32, len: WORD32, mem: FLOAT32) -> VOID;
    fn ixheaacd_residual_tool_float(
        a: *mut FLOAT32,
        x: *mut FLOAT32,
        y: *mut FLOAT32,
        l: WORD32,
        loop_count: WORD32,
    ) -> VOID;
    fn ixheaacd_lsp_to_lp_conversion(lsp: *mut FLOAT32, a: *mut FLOAT32) -> VOID;
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
pub type WORD = core::ffi::c_int;
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
pub struct ia_usac_samp_rate_info {
    pub samp_rate: WORD32,
    pub num_sfb_1024: WORD32,
    pub ptr_sfb_1024: *const WORD16,
    pub num_sfb_128: WORD32,
    pub ptr_sfb_128: *const WORD16,
    pub num_sfb_960: WORD32,
    pub ptr_sfb_960: *const WORD16,
    pub num_sfb_120: WORD32,
    pub ptr_sfb_120: *const WORD16,
    pub num_sfb_768: WORD32,
    pub ptr_sfb_768: *const WORD16,
    pub num_sfb_96: WORD32,
    pub ptr_sfb_96: *const WORD16,
    pub short_fss_width: WORD32,
    pub long_fss_groups: WORD32,
    pub num_sfb_480: WORD32,
    pub ptr_sfb_480: *const WORD16,
    pub num_sfb_512: WORD32,
    pub ptr_sfb_512: *const WORD16,
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
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const MAX_32: WORD32 = 0x7fffffff as core::ffi::c_long as WORD32;
pub const MIN_32: WORD32 = 0x80000000 as core::ffi::c_long as WORD32;
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
unsafe extern "C" fn ixheaac_shl32_dir_sat(mut a: WORD32, mut b: WORD) -> WORD32 {
    let mut out_val: WORD32 = 0;
    if b < 0 as core::ffi::c_int {
        out_val = ixheaac_shr32(a, -b);
    } else {
        out_val = ixheaac_shl32_sat(a, b);
    }
    return out_val;
}
#[inline]
unsafe extern "C" fn ixheaac_mult32x16in32_sat(mut a: WORD32, mut b: WORD16) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    if temp_result < MIN_32 as WORD64 {
        result = MIN_32;
    } else if temp_result > MAX_32 as WORD64 {
        result = MAX_32;
    } else {
        result = temp_result as WORD32;
    }
    return result;
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
unsafe extern "C" fn ixheaac_mult32x32in32(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 16 as core::ffi::c_int) as WORD32;
    return result;
}
#[inline]
unsafe extern "C" fn ixheaac_mul32_sh(
    mut a: WORD32,
    mut b: WORD32,
    mut shift: WORD8,
) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> shift as core::ffi::c_int) as WORD32;
    return result;
}
pub const INT_BITS: core::ffi::c_int = 32 as core::ffi::c_int;
pub const ORDER: core::ffi::c_int = 16 as core::ffi::c_int;
pub const FSCALE_DENOM: core::ffi::c_int = 12800 as core::ffi::c_int;
pub const FAC_FSCALE_MAX: core::ffi::c_int = 24000 as core::ffi::c_int;
pub const TMIN: core::ffi::c_int = 34 as core::ffi::c_int;
pub const TMAX: core::ffi::c_int = 27 as core::ffi::c_int + 6 as core::ffi::c_int * TMIN;
pub const MAX_PITCH: core::ffi::c_int = TMAX
    + 6 as core::ffi::c_int
        * ((FAC_FSCALE_MAX * TMIN + FSCALE_DENOM / 2 as core::ffi::c_int) / FSCALE_DENOM
            - TMIN);
pub const PREEMPH_FILT_FAC: core::ffi::c_float = 0.68f32;
pub const WIN_LEN_768: core::ffi::c_int = 768 as core::ffi::c_int;
pub const CORE_MODE_FD: core::ffi::c_int = 0 as core::ffi::c_int;
pub const ONLY_LONG_SEQUENCE: core::ffi::c_int = 0 as core::ffi::c_int;
pub const LONG_START_SEQUENCE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const EIGHT_SHORT_SEQUENCE: core::ffi::c_int = 2 as core::ffi::c_int;
pub const LONG_STOP_SEQUENCE: core::ffi::c_int = 3 as core::ffi::c_int;
pub const LEN_SUBFR: core::ffi::c_int = 64 as core::ffi::c_int;
pub const NO_TRANSITION: core::ffi::c_int = 0 as core::ffi::c_int;
pub const TRANS_SHORT_LONG: core::ffi::c_int = 1 as core::ffi::c_int;
pub const FRAME_OKAY: core::ffi::c_int = 0 as core::ffi::c_int;
pub const FRAME_CONCEAL_SINGLE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const FRAME_FADE: core::ffi::c_int = 2 as core::ffi::c_int;
pub const FRAME_MUTE: core::ffi::c_int = 3 as core::ffi::c_int;
pub const MAX_FADE_FRAMES: core::ffi::c_int = 8 as core::ffi::c_int;
pub const MAX_SPEC_SCALE_LEN: core::ffi::c_int = 8 as core::ffi::c_int;
pub const BETA: core::ffi::c_float = 0.25f32;
pub const ONE_BETA: core::ffi::c_float = 0.75f32;
pub const BFI_FAC: core::ffi::c_float = 0.90f32;
pub const ONE_BFI_FAC: core::ffi::c_float = 0.10f32;
unsafe extern "C" fn ixheaacd_usac_ec_get_win_seq(mut prev_win_seq: WORD32) -> WORD32 {
    if prev_win_seq == LONG_START_SEQUENCE || prev_win_seq == EIGHT_SHORT_SEQUENCE {
        return LONG_STOP_SEQUENCE
    } else {
        return ONLY_LONG_SEQUENCE
    };
}
unsafe extern "C" fn ixheaacd_usac_flip_spec_sign(
    mut ptr_spec_coeff: *mut WORD32,
    mut samples_per_frame: WORD32,
    mut seed_value: *mut UWORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < samples_per_frame {
        *ptr_spec_coeff.offset(i as isize) = ixheaac_mult32x16in32_sat(
            *ptr_spec_coeff.offset(i as isize),
            ixheaacd_randomsign(seed_value) as WORD16,
        );
        i += 1;
    }
}
unsafe extern "C" fn iexheaace_ec_sfb_nrg_q(
    mut ptr_spectrum: *mut WORD32,
    mut pstr_ec_sfb: *mut ia_ec_sfb_str,
    mut win_seq: WORD32,
    mut win_trans: WORD32,
    mut ptr_sfb_enrg: *mut WORD32,
) -> VOID {
    let mut ptr_sfb_offset: *mut WORD16 = (*pstr_ec_sfb).ptr_sfb_long;
    let mut l: WORD32 = 0 as WORD32;
    let mut sfb: WORD32 = 0;
    let mut num_sfb: WORD32 = (*pstr_ec_sfb).num_sfb_long;
    match win_seq {
        EIGHT_SHORT_SEQUENCE => {
            if win_trans == NO_TRANSITION {
                num_sfb = (*pstr_ec_sfb).num_sfb_short;
                ptr_sfb_offset = (*pstr_ec_sfb).ptr_sfb_short;
                sfb = 0 as core::ffi::c_int as WORD32;
                while sfb < num_sfb {
                    let mut accu: WORD64 = 1 as core::ffi::c_int as WORD64;
                    let mut q_nrg: WORD32 = ((::core::mem::size_of::<WORD64>() as usize)
                        << 3 as core::ffi::c_int)
                        .wrapping_sub(
                            ixheaac_norm32(
                                *ptr_sfb_offset
                                    .offset(
                                        (sfb as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                                    ) as WORD32 - *ptr_sfb_offset.offset(sfb as isize) as WORD32,
                            ) as usize,
                        ) as WORD32;
                    while l
                        < *ptr_sfb_offset
                            .offset(
                                (sfb as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                            ) as core::ffi::c_int
                    {
                        accu
                            += ixheaac_mul32_sh(
                                *ptr_spectrum.offset(l as isize),
                                *ptr_spectrum.offset(l as isize),
                                q_nrg as WORD8,
                            ) as WORD64;
                        l += 1;
                    }
                    *ptr_sfb_enrg.offset(sfb as isize) = ixheaac_norm32(accu as WORD32)
                        as WORD32;
                    sfb += 1;
                }
            } else {
                num_sfb = (*pstr_ec_sfb).num_sfb_long;
                ptr_sfb_offset = (*pstr_ec_sfb).ptr_sfb_long;
                sfb = 0 as core::ffi::c_int as WORD32;
                while sfb < num_sfb {
                    let mut accu_0: WORD64 = 1 as core::ffi::c_int as WORD64;
                    let mut q_nrg_0: WORD32 = ((::core::mem::size_of::<WORD64>()
                        as usize) << 3 as core::ffi::c_int)
                        .wrapping_sub(
                            ixheaac_norm32(
                                *ptr_sfb_offset
                                    .offset(
                                        (sfb as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                                    ) as WORD32 - *ptr_sfb_offset.offset(sfb as isize) as WORD32,
                            ) as usize,
                        ) as WORD32;
                    while l
                        < *ptr_sfb_offset
                            .offset(
                                (sfb as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                            ) as core::ffi::c_int
                    {
                        accu_0
                            += ixheaac_mul32_sh(
                                *ptr_spectrum.offset((l >> 3 as core::ffi::c_int) as isize),
                                *ptr_spectrum.offset((l >> 3 as core::ffi::c_int) as isize),
                                q_nrg_0 as WORD8,
                            ) as WORD64;
                        l += 1;
                    }
                    *ptr_sfb_enrg.offset(sfb as isize) = ixheaac_norm32(accu_0 as WORD32)
                        as WORD32;
                    sfb += 1;
                }
            }
        }
        ONLY_LONG_SEQUENCE | LONG_START_SEQUENCE | LONG_STOP_SEQUENCE => {
            if win_trans == NO_TRANSITION {
                num_sfb = (*pstr_ec_sfb).num_sfb_long;
                ptr_sfb_offset = (*pstr_ec_sfb).ptr_sfb_long;
                sfb = 0 as core::ffi::c_int as WORD32;
                while sfb < num_sfb {
                    let mut accu_1: WORD64 = 1 as core::ffi::c_int as WORD64;
                    let mut q_nrg_1: WORD32 = ((::core::mem::size_of::<WORD64>()
                        as usize) << 3 as core::ffi::c_int)
                        .wrapping_sub(
                            ixheaac_norm32(
                                *ptr_sfb_offset
                                    .offset(
                                        (sfb as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                                    ) as WORD32 - *ptr_sfb_offset.offset(sfb as isize) as WORD32,
                            ) as usize,
                        ) as WORD32;
                    while l
                        < *ptr_sfb_offset
                            .offset(
                                (sfb as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                            ) as core::ffi::c_int
                    {
                        accu_1
                            += ixheaac_mul32_sh(
                                *ptr_spectrum.offset(l as isize),
                                *ptr_spectrum.offset(l as isize),
                                q_nrg_1 as WORD8,
                            ) as WORD64;
                        l += 1;
                    }
                    *ptr_sfb_enrg.offset(sfb as isize) = ixheaac_norm32(accu_1 as WORD32)
                        as WORD32;
                    sfb += 1;
                }
            } else {
                num_sfb = (*pstr_ec_sfb).num_sfb_short;
                ptr_sfb_offset = (*pstr_ec_sfb).ptr_sfb_short;
                sfb = 0 as core::ffi::c_int as WORD32;
                while sfb < num_sfb {
                    let mut accu_2: WORD64 = 1 as core::ffi::c_int as WORD64;
                    let mut q_nrg_2: WORD32 = ((::core::mem::size_of::<WORD64>()
                        as usize) << 3 as core::ffi::c_int)
                        .wrapping_sub(
                            ixheaac_norm32(
                                *ptr_sfb_offset
                                    .offset(
                                        (sfb as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                                    ) as WORD32 - *ptr_sfb_offset.offset(sfb as isize) as WORD32,
                            ) as usize,
                        ) as WORD32;
                    while l
                        < (*ptr_sfb_offset
                            .offset(
                                (sfb as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                            ) as core::ffi::c_int) << 3 as core::ffi::c_int
                    {
                        accu_2
                            += accu_2
                                + ixheaac_mul32_sh(
                                    *ptr_spectrum.offset(l as isize),
                                    *ptr_spectrum.offset(l as isize),
                                    q_nrg_2 as WORD8,
                                ) as WORD64 >> 3 as core::ffi::c_int;
                        l += 1;
                    }
                    *ptr_sfb_enrg.offset(sfb as isize) = ixheaac_norm32(accu_2 as WORD32)
                        as WORD32;
                    sfb += 1;
                }
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn ixheaacd_usac_ec_interpolate(
    mut ptr_spectrum: *mut WORD32,
    mut pq_spec_coeff_prev: *mut WORD16,
    mut pq_spec_coeff_act: *mut WORD16,
    mut pq_spec_coeff_out: *mut WORD16,
    mut ptr_nrg_prev: *mut WORD32,
    mut ptr_nrg_act: *mut WORD32,
    mut num_sfb: WORD32,
    mut ptr_sfb_offset: *mut WORD16,
) -> VOID {
    let mut sfb: WORD32 = 0;
    let mut l: WORD32 = 0 as WORD32;
    let mut fac_shift: WORD32 = 0;
    let mut fac_mod: WORD32 = 0;
    sfb = 0 as core::ffi::c_int as WORD32;
    while sfb < num_sfb {
        fac_shift = (*ptr_nrg_prev.offset(sfb as isize)
            - *ptr_nrg_act.offset(sfb as isize)
            + ((*pq_spec_coeff_act as core::ffi::c_int
                - *pq_spec_coeff_prev as core::ffi::c_int) << 1 as core::ffi::c_int))
            as WORD32;
        fac_mod = (fac_shift as core::ffi::c_int & 3 as core::ffi::c_int) as WORD32;
        fac_shift = ((fac_shift as core::ffi::c_int >> 2 as core::ffi::c_int)
            + 1 as core::ffi::c_int) as WORD32;
        fac_shift
            += *pq_spec_coeff_prev as core::ffi::c_int
                - ixheaac_max16(*pq_spec_coeff_prev, *pq_spec_coeff_act)
                    as core::ffi::c_int;
        fac_shift = ixheaac_max32(
            ixheaac_min32(fac_shift, INT_BITS - 1 as WORD32),
            -(INT_BITS - 1 as WORD32),
        );
        while l
            < *ptr_sfb_offset
                .offset((sfb as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                as core::ffi::c_int
        {
            let mut accu: WORD32 = ixheaac_shl32_sat(
                ixheaac_mult32x32in32(
                    *ptr_spectrum.offset(l as isize),
                    ia_ec_interpolation_fac[fac_mod as usize] as WORD32,
                ),
                1 as WORD,
            );
            *ptr_spectrum.offset(l as isize) = ixheaac_shl32_dir_sat(
                accu,
                fac_shift as WORD,
            );
            l += 1;
        }
        sfb += 1;
    }
    *pq_spec_coeff_out = ixheaac_max16(*pq_spec_coeff_prev, *pq_spec_coeff_act);
}
unsafe extern "C" fn ixheaacd_usac_ec_interpolate_frame(
    mut pstr_usac_data: *mut ia_usac_data_struct,
    mut pstr_ec_state: *mut ia_ec_state_str,
    mut pstr_samp_rate_info: *const ia_usac_samp_rate_info,
    mut frame_ok: WORD32,
    mut chn: WORD32,
) -> VOID {
    let mut frame_length: WORD32 = (*pstr_usac_data).ccfl;
    let mut ptr_spec_coeff: *mut WORD32 = (*pstr_usac_data).coef_fix[chn as usize];
    let mut ptr_spec_sf: *mut WORD16 = ((*pstr_usac_data).spec_scale[chn as usize])
        .as_mut_ptr();
    let mut i: WORD32 = 0;
    let mut pstr_ec_scratch: *mut ia_ec_scratch_str = (*pstr_ec_state).pstr_ec_scratch;
    let mut num_sfb_long: WORD16 = 0;
    let mut ptr_sfb_long: *mut WORD16 = 0 as *mut WORD16;
    let mut num_sfb_short: WORD16 = 0;
    let mut ptr_sfb_short: *mut WORD16 = 0 as *mut WORD16;
    if (*pstr_usac_data).core_mode == CORE_MODE_FD {
        num_sfb_long = (*pstr_samp_rate_info).num_sfb_1024 as WORD16;
        ptr_sfb_long = (*pstr_samp_rate_info).ptr_sfb_1024 as *mut WORD16;
        num_sfb_short = (*pstr_samp_rate_info).num_sfb_128 as WORD16;
        ptr_sfb_short = (*pstr_samp_rate_info).ptr_sfb_128 as *mut WORD16;
        if (*pstr_usac_data).ccfl == WIN_LEN_768 {
            num_sfb_long = (*pstr_samp_rate_info).num_sfb_768 as WORD16;
            ptr_sfb_long = (*pstr_samp_rate_info).ptr_sfb_768 as *mut WORD16;
            num_sfb_short = (*pstr_samp_rate_info).num_sfb_96 as WORD16;
            ptr_sfb_short = (*pstr_samp_rate_info).ptr_sfb_96 as *mut WORD16;
        }
        (*pstr_ec_state).str_ec_sfb.num_sfb_long = num_sfb_long as WORD32;
        (*pstr_ec_state).str_ec_sfb.num_sfb_long = num_sfb_long as WORD32;
        (*pstr_ec_state).str_ec_sfb.ptr_sfb_long = ptr_sfb_long;
        (*pstr_ec_state).str_ec_sfb.ptr_sfb_long = ptr_sfb_long;
        memset(
            ((*pstr_ec_scratch).prev_sfb_nrg).as_mut_ptr() as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<[WORD32; 51]>() as size_t,
        );
        memset(
            ((*pstr_ec_scratch).pres_sfb_nrg).as_mut_ptr() as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<[WORD32; 51]>() as size_t,
        );
        if frame_ok == 0 {
            (*pstr_usac_data).window_shape[chn as usize] = (*pstr_ec_state).win_shape
                as WORD32;
            (*pstr_usac_data).window_sequence[chn as usize] = (*pstr_ec_state).win_seq;
            memcpy(
                ptr_spec_coeff as *mut core::ffi::c_void,
                ((*pstr_ec_state).spectral_coeff).as_mut_ptr()
                    as *const core::ffi::c_void,
                (::core::mem::size_of::<WORD32>() as size_t)
                    .wrapping_mul(frame_length as size_t),
            );
            memcpy(
                ptr_spec_sf as *mut core::ffi::c_void,
                ((*pstr_ec_state).q_spec_coeff).as_mut_ptr() as *const core::ffi::c_void,
                ::core::mem::size_of::<[WORD16; 128]>() as size_t,
            );
        }
    }
    if (*pstr_ec_state).prev_frame_ok[1 as core::ffi::c_int as usize] == 0 {
        if frame_ok != 0
            && (*pstr_ec_state).prev_frame_ok[0 as core::ffi::c_int as usize] != 0
            && (*pstr_usac_data).core_mode == CORE_MODE_FD
        {
            if (*pstr_usac_data).window_sequence[chn as usize] == EIGHT_SHORT_SEQUENCE {
                let mut wnd: WORD32 = 0;
                if (*pstr_ec_state).win_seq == EIGHT_SHORT_SEQUENCE {
                    let mut num_sfb: WORD32 = num_sfb_short as WORD32;
                    let mut ptr_sfb_offset: *mut WORD16 = ptr_sfb_short;
                    (*pstr_usac_data).window_shape[chn as usize] = 1 as core::ffi::c_int
                        as WORD32;
                    (*pstr_usac_data).window_sequence[chn as usize] = EIGHT_SHORT_SEQUENCE
                        as WORD32;
                    wnd = 0 as core::ffi::c_int as WORD32;
                    while wnd < 8 as core::ffi::c_int {
                        iexheaace_ec_sfb_nrg_q(
                            &mut *ptr_spec_coeff
                                .offset(
                                    (wnd * (frame_length >> 3 as core::ffi::c_int)) as isize,
                                ),
                            &mut (*pstr_ec_state).str_ec_sfb,
                            EIGHT_SHORT_SEQUENCE,
                            NO_TRANSITION,
                            ((*pstr_ec_scratch).prev_sfb_nrg).as_mut_ptr(),
                        );
                        iexheaace_ec_sfb_nrg_q(
                            &mut *((*pstr_ec_state).spectral_coeff)
                                .as_mut_ptr()
                                .offset(
                                    (wnd * (frame_length >> 3 as core::ffi::c_int)) as isize,
                                ),
                            &mut (*pstr_ec_state).str_ec_sfb,
                            EIGHT_SHORT_SEQUENCE,
                            NO_TRANSITION,
                            ((*pstr_ec_scratch).pres_sfb_nrg).as_mut_ptr(),
                        );
                        ixheaacd_usac_ec_interpolate(
                            &mut *ptr_spec_coeff
                                .offset(
                                    (wnd as core::ffi::c_int
                                        * (frame_length as core::ffi::c_int
                                            / 8 as core::ffi::c_int)) as isize,
                                ),
                            &mut *ptr_spec_sf.offset(wnd as isize),
                            &mut *((*pstr_ec_state).q_spec_coeff)
                                .as_mut_ptr()
                                .offset(wnd as isize),
                            &mut *ptr_spec_sf.offset(wnd as isize),
                            ((*pstr_ec_scratch).prev_sfb_nrg).as_mut_ptr(),
                            ((*pstr_ec_scratch).pres_sfb_nrg).as_mut_ptr(),
                            num_sfb,
                            ptr_sfb_offset,
                        );
                        wnd += 1;
                    }
                } else {
                    let mut num_sfb_0: WORD32 = num_sfb_long as WORD32;
                    let mut ptr_sfb_offset_0: *mut WORD16 = ptr_sfb_long;
                    let mut q_spec_coeff_out: WORD16 = 0;
                    iexheaace_ec_sfb_nrg_q(
                        &mut *ptr_spec_coeff
                            .offset(
                                (frame_length - (frame_length >> 3 as core::ffi::c_int))
                                    as isize,
                            ),
                        &mut (*pstr_ec_state).str_ec_sfb,
                        EIGHT_SHORT_SEQUENCE,
                        TRANS_SHORT_LONG,
                        ((*pstr_ec_scratch).pres_sfb_nrg).as_mut_ptr(),
                    );
                    iexheaace_ec_sfb_nrg_q(
                        ((*pstr_ec_state).spectral_coeff).as_mut_ptr(),
                        &mut (*pstr_ec_state).str_ec_sfb,
                        ONLY_LONG_SEQUENCE,
                        NO_TRANSITION,
                        ((*pstr_ec_scratch).prev_sfb_nrg).as_mut_ptr(),
                    );
                    (*pstr_usac_data).window_shape[chn as usize] = 0 as core::ffi::c_int
                        as WORD32;
                    (*pstr_usac_data).window_sequence[chn as usize] = LONG_STOP_SEQUENCE
                        as WORD32;
                    memcpy(
                        &mut *ptr_spec_coeff.offset(0 as core::ffi::c_int as isize)
                            as *mut WORD32 as *mut core::ffi::c_void,
                        ((*pstr_ec_state).spectral_coeff).as_mut_ptr()
                            as *const core::ffi::c_void,
                        (frame_length as size_t)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
                    );
                    i = 0 as core::ffi::c_int as WORD32;
                    while i < 8 as core::ffi::c_int {
                        if *ptr_spec_sf.offset(i as isize) as core::ffi::c_int
                            > *ptr_spec_sf.offset(0 as core::ffi::c_int as isize)
                                as core::ffi::c_int
                        {
                            *ptr_spec_sf.offset(0 as core::ffi::c_int as isize) = *ptr_spec_sf
                                .offset(i as isize);
                        }
                        i += 1;
                    }
                    ixheaacd_usac_ec_interpolate(
                        ptr_spec_coeff,
                        &mut *((*pstr_ec_state).q_spec_coeff)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize),
                        &mut *ptr_spec_sf.offset(0 as core::ffi::c_int as isize),
                        &mut q_spec_coeff_out,
                        ((*pstr_ec_scratch).prev_sfb_nrg).as_mut_ptr(),
                        ((*pstr_ec_scratch).pres_sfb_nrg).as_mut_ptr(),
                        num_sfb_0,
                        ptr_sfb_offset_0,
                    );
                    *ptr_spec_sf.offset(0 as core::ffi::c_int as isize) = q_spec_coeff_out;
                }
            } else {
                let mut num_sfb_1: WORD32 = num_sfb_long as WORD32;
                let mut ptr_sfb_offset_1: *mut WORD16 = ptr_sfb_long;
                let mut q_spec_coeff_act: WORD16 = (*pstr_ec_state)
                    .q_spec_coeff[0 as core::ffi::c_int as usize];
                iexheaace_ec_sfb_nrg_q(
                    ptr_spec_coeff,
                    &mut (*pstr_ec_state).str_ec_sfb,
                    ONLY_LONG_SEQUENCE,
                    NO_TRANSITION,
                    ((*pstr_ec_scratch).prev_sfb_nrg).as_mut_ptr(),
                );
                if (*pstr_ec_state).win_seq == EIGHT_SHORT_SEQUENCE {
                    (*pstr_usac_data).window_shape[chn as usize] = 1 as core::ffi::c_int
                        as WORD32;
                    (*pstr_usac_data).window_sequence[chn as usize] = LONG_START_SEQUENCE
                        as WORD32;
                    i = 1 as core::ffi::c_int as WORD32;
                    while i < 8 as core::ffi::c_int {
                        if (*pstr_ec_state).q_spec_coeff[i as usize] as core::ffi::c_int
                            > q_spec_coeff_act as core::ffi::c_int
                        {
                            q_spec_coeff_act = (*pstr_ec_state).q_spec_coeff[i as usize];
                        }
                        i += 1;
                    }
                    iexheaace_ec_sfb_nrg_q(
                        ((*pstr_ec_state).spectral_coeff).as_mut_ptr(),
                        &mut (*pstr_ec_state).str_ec_sfb,
                        EIGHT_SHORT_SEQUENCE,
                        TRANS_SHORT_LONG,
                        ((*pstr_ec_scratch).pres_sfb_nrg).as_mut_ptr(),
                    );
                } else {
                    (*pstr_usac_data).window_shape[chn as usize] = 0 as core::ffi::c_int
                        as WORD32;
                    (*pstr_usac_data).window_sequence[chn as usize] = ONLY_LONG_SEQUENCE
                        as WORD32;
                    iexheaace_ec_sfb_nrg_q(
                        ((*pstr_ec_state).spectral_coeff).as_mut_ptr(),
                        &mut (*pstr_ec_state).str_ec_sfb,
                        ONLY_LONG_SEQUENCE,
                        NO_TRANSITION,
                        ((*pstr_ec_scratch).pres_sfb_nrg).as_mut_ptr(),
                    );
                }
                ixheaacd_usac_ec_interpolate(
                    ptr_spec_coeff,
                    &mut *ptr_spec_sf.offset(0 as core::ffi::c_int as isize),
                    &mut q_spec_coeff_act,
                    &mut *ptr_spec_sf.offset(0 as core::ffi::c_int as isize),
                    ((*pstr_ec_scratch).prev_sfb_nrg).as_mut_ptr(),
                    ((*pstr_ec_scratch).pres_sfb_nrg).as_mut_ptr(),
                    num_sfb_1,
                    ptr_sfb_offset_1,
                );
            }
        }
        ixheaacd_usac_flip_spec_sign(
            ptr_spec_coeff,
            frame_length,
            &mut *((*pstr_usac_data).seed_value).as_mut_ptr().offset(chn as isize),
        );
    }
    if FRAME_MUTE == (*pstr_ec_state).conceal_state {
        (*pstr_usac_data).window_shape[chn as usize] = (*pstr_ec_state).win_shape
            as WORD32;
        (*pstr_usac_data).window_sequence[chn as usize] = ixheaacd_usac_ec_get_win_seq(
            (*pstr_ec_state).win_seq,
        );
        (*pstr_ec_state).win_seq = (*pstr_usac_data).window_sequence[chn as usize];
        memset(
            ptr_spec_coeff as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (frame_length as size_t)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
        );
    }
}
unsafe extern "C" fn ixheaacd_usac_lpc_ec_state(
    mut pstr_ec_state: *mut ia_ec_state_str,
    mut frame_ok: WORD32,
) -> VOID {
    if frame_ok == 0 as core::ffi::c_int {
        if (*pstr_ec_state).fade_idx < MAX_FADE_FRAMES {
            (*pstr_ec_state).fade_idx += 1;
        }
        (*pstr_ec_state).conceal_state = FRAME_CONCEAL_SINGLE as WORD32;
    } else {
        if (*pstr_ec_state).fade_idx > 0 as core::ffi::c_int {
            (*pstr_ec_state).fade_idx -= 1;
        }
        (*pstr_ec_state).conceal_state = FRAME_OKAY as WORD32;
    }
    if (*pstr_ec_state).fade_idx >= MAX_FADE_FRAMES {
        (*pstr_ec_state).fade_idx = MAX_FADE_FRAMES as WORD32;
        (*pstr_ec_state).conceal_state = FRAME_MUTE as WORD32;
    }
    if (*pstr_ec_state).fade_idx < 0 as core::ffi::c_int {
        (*pstr_ec_state).fade_idx = 0 as core::ffi::c_int as WORD32;
    }
}
unsafe extern "C" fn ixheaacd_usac_ec_state(
    mut pstr_ec_state: *mut ia_ec_state_str,
    mut frame_ok: WORD32,
) -> VOID {
    let mut ec_state_val: WORD32 = ((*pstr_ec_state)
        .prev_frame_ok[0 as core::ffi::c_int as usize] << 2 as core::ffi::c_int)
        + ((*pstr_ec_state).prev_frame_ok[1 as core::ffi::c_int as usize]
            << 1 as core::ffi::c_int) + frame_ok;
    match ec_state_val {
        0 | 4 => {
            if (*pstr_ec_state).fade_idx < MAX_FADE_FRAMES {
                (*pstr_ec_state).fade_idx += 1;
            }
            (*pstr_ec_state).conceal_state = FRAME_CONCEAL_SINGLE as WORD32;
        }
        1 | 2 => {
            if (*pstr_ec_state).fade_idx > 0 as core::ffi::c_int {
                (*pstr_ec_state).fade_idx -= 1;
            }
            (*pstr_ec_state).conceal_state = FRAME_FADE as WORD32;
        }
        5 => {
            if (*pstr_ec_state).fade_idx > 0 as core::ffi::c_int {
                (*pstr_ec_state).fade_idx -= 1;
            }
            (*pstr_ec_state).conceal_state = FRAME_OKAY as WORD32;
        }
        3 | 6 | 7 => {
            if (*pstr_ec_state).fade_idx > 0 as core::ffi::c_int {
                (*pstr_ec_state).fade_idx -= 1;
            }
            (*pstr_ec_state).conceal_state = FRAME_OKAY as WORD32;
        }
        _ => {
            (*pstr_ec_state).conceal_state = FRAME_OKAY as WORD32;
        }
    }
    if (*pstr_ec_state).fade_idx > MAX_FADE_FRAMES {
        (*pstr_ec_state).fade_idx = MAX_FADE_FRAMES as WORD32;
    }
    if (*pstr_ec_state).fade_idx == MAX_FADE_FRAMES {
        (*pstr_ec_state).conceal_state = FRAME_MUTE as WORD32;
    }
    if (*pstr_ec_state).fade_idx < 0 as core::ffi::c_int {
        (*pstr_ec_state).fade_idx = 0 as core::ffi::c_int as WORD32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_usac_ec_init(
    mut pstr_ec_state: *mut ia_ec_state_str,
    mut core_coder_mode: WORD32,
) -> VOID {
    (*pstr_ec_state).win_shape = 1 as UWORD8;
    (*pstr_ec_state).win_seq = ONLY_LONG_SEQUENCE as WORD32;
    (*pstr_ec_state).prev_win_group_len = 1 as UWORD8;
    (*pstr_ec_state).conceal_state = FRAME_OKAY as WORD32;
    memset(
        ((*pstr_ec_state).spectral_coeff).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[WORD32; 1024]>() as size_t,
    );
    memset(
        ((*pstr_ec_state).q_spec_coeff).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[WORD16; 128]>() as size_t,
    );
    (*pstr_ec_state).prev_frame_ok[0 as core::ffi::c_int as usize] = 1
        as core::ffi::c_int as WORD32;
    (*pstr_ec_state).prev_frame_ok[1 as core::ffi::c_int as usize] = 1
        as core::ffi::c_int as WORD32;
    (*pstr_ec_state).fade_idx = 0 as core::ffi::c_int as WORD32;
    (*pstr_ec_state).prev_core_mode = core_coder_mode;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_usac_lpc_ec(
    mut lsp: *mut [FLOAT32; 16],
    mut lpc4_lsf: *mut FLOAT32,
    mut lsf_adaptive_mean: *mut FLOAT32,
    first_lpd_flag: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    if first_lpd_flag != 0 {
        memcpy(
            (*lsp.offset(0 as core::ffi::c_int as isize)).as_mut_ptr()
                as *mut core::ffi::c_void,
            lsf_init.as_ptr() as *const core::ffi::c_void,
            ::core::mem::size_of::<[FLOAT32; 16]>() as size_t,
        );
        memcpy(
            lpc4_lsf as *mut core::ffi::c_void,
            lsf_init.as_ptr() as *const core::ffi::c_void,
            ::core::mem::size_of::<[FLOAT32; 16]>() as size_t,
        );
    } else {
        memcpy(
            (*lsp.offset(0 as core::ffi::c_int as isize)).as_mut_ptr()
                as *mut core::ffi::c_void,
            lpc4_lsf as *const core::ffi::c_void,
            (ORDER as size_t).wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < ORDER {
        let mut lsf_mean: FLOAT32 = BETA * lsf_init[i as usize]
            + ONE_BETA * *lsf_adaptive_mean.offset(i as isize);
        (*lsp.offset(1 as core::ffi::c_int as isize))[i as usize] = BFI_FAC
            * *lpc4_lsf.offset(i as isize) + ONE_BFI_FAC * lsf_mean;
        i += 1;
    }
    j = 2 as core::ffi::c_int as WORD32;
    while j <= 4 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < ORDER {
            let mut lsf_mean_0: FLOAT32 = (BETA + j as FLOAT32 * ONE_BFI_FAC)
                * lsf_init[i as usize]
                + (ONE_BETA - j as FLOAT32 * ONE_BFI_FAC)
                    * *lsf_adaptive_mean.offset(i as isize);
            (*lsp.offset(j as isize))[i as usize] = BFI_FAC
                * (*lsp
                    .offset(
                        (j as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                    ))[i as usize] + ONE_BFI_FAC * lsf_mean_0;
            i += 1;
        }
        j += 1;
    }
    memcpy(
        lpc4_lsf as *mut core::ffi::c_void,
        (*lsp.offset(4 as core::ffi::c_int as isize)).as_mut_ptr()
            as *const core::ffi::c_void,
        (ORDER as size_t).wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_usac_ec_save_states(
    mut pstr_ec_state: *mut ia_ec_state_str,
    mut pstr_usac_data: *mut ia_usac_data_struct,
    mut ch: WORD32,
) -> VOID {
    if (*pstr_usac_data).core_mode == CORE_MODE_FD
        && ((*pstr_usac_data).frame_ok == 1 as core::ffi::c_int
            && (*pstr_ec_state).prev_frame_ok[1 as core::ffi::c_int as usize]
                == 1 as core::ffi::c_int)
    {
        let mut ptr_spec_coeff: *mut WORD32 = (*pstr_usac_data).coef_fix[ch as usize];
        let mut ptr_spec_scale: *mut WORD16 = ((*pstr_usac_data).spec_scale[ch as usize])
            .as_mut_ptr();
        let mut q_spec_coeff: [WORD16; 128] = [0; 128];
        let mut win_shape: UWORD8 = (*pstr_ec_state).win_shape;
        let mut win_shape_prev: UWORD8 = (*pstr_ec_state).win_shape_prev;
        let mut win_seq: WORD32 = (*pstr_ec_state).win_seq;
        let mut td_frame_prev: WORD32 = (*pstr_ec_state).td_frame_prev;
        let mut fac_data_present: WORD32 = (*pstr_ec_state).fac_data_present;
        let mut sfb_info: *mut ia_sfb_info_struct = (*pstr_usac_data)
            .pstr_usac_winmap[(*pstr_usac_data).window_sequence[ch as usize] as usize];
        let mut ptr_scratch_buf: *mut WORD32 = &mut *((*(*pstr_ec_state).pstr_ec_scratch)
            .spec_coeff)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
        memcpy(
            q_spec_coeff.as_mut_ptr() as *mut core::ffi::c_void,
            ((*pstr_ec_state).q_spec_coeff).as_mut_ptr() as *const core::ffi::c_void,
            ::core::mem::size_of::<[WORD16; 128]>() as size_t,
        );
        (*pstr_ec_state).win_seq = (*pstr_usac_data).window_sequence[ch as usize];
        (*pstr_ec_state).win_shape = (*pstr_usac_data).window_shape[ch as usize]
            as UWORD8;
        (*pstr_ec_state).td_frame_prev = (*pstr_usac_data).td_frame_prev[ch as usize];
        (*pstr_ec_state).fac_data_present = (*pstr_usac_data)
            .fac_data_present[ch as usize];
        (*pstr_ec_state).win_shape_prev = (*pstr_usac_data)
            .window_shape_prev[ch as usize] as UWORD8;
        (*pstr_ec_state).prev_win_group_len = (*sfb_info)
            .group_len[((*sfb_info).num_groups as core::ffi::c_int
            - 1 as core::ffi::c_int) as usize] as WORD32 as UWORD8;
        memcpy(
            ((*pstr_ec_state).q_spec_coeff).as_mut_ptr() as *mut core::ffi::c_void,
            ptr_spec_scale as *const core::ffi::c_void,
            ::core::mem::size_of::<[WORD16; 128]>() as size_t,
        );
        memcpy(
            ptr_scratch_buf as *mut core::ffi::c_void,
            ptr_spec_coeff as *const core::ffi::c_void,
            ((*pstr_usac_data).ccfl as size_t)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
        );
        memcpy(
            ptr_spec_coeff as *mut core::ffi::c_void,
            &mut *((*pstr_ec_state).spectral_coeff)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut WORD32
                as *const core::ffi::c_void,
            ((*pstr_usac_data).ccfl as size_t)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
        );
        memcpy(
            &mut *((*pstr_ec_state).spectral_coeff)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut WORD32
                as *mut core::ffi::c_void,
            ptr_scratch_buf as *const core::ffi::c_void,
            ((*pstr_usac_data).ccfl as size_t)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
        );
        if (*pstr_usac_data).first_frame == 0 {
            (*pstr_usac_data).window_sequence[ch as usize] = win_seq;
            (*pstr_usac_data).window_shape[ch as usize] = win_shape as WORD32;
            (*pstr_usac_data).td_frame_prev_ec[ch as usize] = td_frame_prev;
            (*pstr_usac_data).fac_data_present[ch as usize] = fac_data_present;
            (*pstr_usac_data).window_shape_prev[ch as usize] = win_shape_prev as WORD32;
        }
        memcpy(
            ptr_spec_scale as *mut core::ffi::c_void,
            q_spec_coeff.as_mut_ptr() as *const core::ffi::c_void,
            ::core::mem::size_of::<[WORD16; 128]>() as size_t,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_usac_apply_ec(
    mut pstr_usac_data: *mut ia_usac_data_struct,
    mut pstr_samp_rate_info: *const ia_usac_samp_rate_info,
    mut ch: WORD32,
) -> VOID {
    let mut frame_ok: WORD32 = (*pstr_usac_data).frame_ok;
    let mut pstr_ec_state: *mut ia_ec_state_str = &mut *((*pstr_usac_data)
        .str_error_concealment)
        .as_mut_ptr()
        .offset(ch as isize) as *mut ia_ec_state_str;
    if (*pstr_usac_data).core_mode == CORE_MODE_FD {
        if (*pstr_ec_state).win_shape as core::ffi::c_int
            == -(1 as core::ffi::c_int) as UWORD8 as core::ffi::c_int
        {
            (*pstr_ec_state).win_shape = (*pstr_usac_data).window_shape[ch as usize]
                as UWORD8;
        }
        ixheaacd_usac_ec_state(pstr_ec_state, frame_ok);
        if (*pstr_ec_state).conceal_state == FRAME_OKAY {
            (*pstr_ec_state).prev_core_mode = (*pstr_usac_data).core_mode;
            ixheaacd_usac_ec_save_states(pstr_ec_state, pstr_usac_data, ch);
        } else if (*pstr_ec_state).conceal_state == FRAME_CONCEAL_SINGLE {
            ixheaacd_usac_ec_interpolate_frame(
                pstr_usac_data,
                pstr_ec_state,
                pstr_samp_rate_info,
                frame_ok,
                ch,
            );
        }
        if frame_ok == 0 {
            let mut ptr_spec_coeff: *mut WORD32 = (*pstr_usac_data)
                .coef_fix[ch as usize];
            let mut ptr_spec_scale: *mut WORD16 = ((*pstr_usac_data)
                .spec_scale[ch as usize])
                .as_mut_ptr();
            (*pstr_usac_data).window_sequence[ch as usize] = (*pstr_ec_state).win_seq;
            (*pstr_usac_data).window_shape[ch as usize] = (*pstr_ec_state).win_shape
                as WORD32;
            if (*pstr_ec_state).conceal_state != FRAME_MUTE {
                memcpy(
                    ptr_spec_scale as *mut core::ffi::c_void,
                    ((*pstr_ec_state).q_spec_coeff).as_mut_ptr()
                        as *const core::ffi::c_void,
                    ::core::mem::size_of::<[WORD16; 128]>() as size_t,
                );
                memcpy(
                    ptr_spec_coeff as *mut core::ffi::c_void,
                    ((*pstr_ec_state).spectral_coeff).as_mut_ptr()
                        as *const core::ffi::c_void,
                    ::core::mem::size_of::<[WORD32; 1024]>() as size_t,
                );
            } else {
                memset(
                    ptr_spec_scale as *mut core::ffi::c_void,
                    0 as core::ffi::c_int,
                    (MAX_SPEC_SCALE_LEN as size_t)
                        .wrapping_mul(::core::mem::size_of::<WORD16>() as size_t),
                );
                memset(
                    ptr_spec_coeff as *mut core::ffi::c_void,
                    0 as core::ffi::c_int,
                    ((*pstr_usac_data).ccfl as size_t)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
                );
            }
        }
    } else {
        ixheaacd_usac_lpc_ec_state(pstr_ec_state, frame_ok);
        if (*pstr_ec_state).conceal_state == FRAME_OKAY {
            memcpy(
                ((*pstr_ec_state).lsf4).as_mut_ptr() as *mut core::ffi::c_void,
                ((*pstr_usac_data).lpc4_lsf).as_mut_ptr() as *const core::ffi::c_void,
                ::core::mem::size_of::<[FLOAT32; 16]>() as size_t,
            );
        } else if (*pstr_ec_state).conceal_state == FRAME_CONCEAL_SINGLE {
            let mut frame_length: WORD32 = (*pstr_usac_data).ccfl;
            let mut ptr_spec_coeff_0: *mut WORD32 = ((*pstr_usac_data)
                .tcx_spec_coeffs[ch as usize])
                .as_mut_ptr();
            ixheaacd_usac_flip_spec_sign(
                ptr_spec_coeff_0,
                frame_length,
                &mut *((*pstr_usac_data).seed_value).as_mut_ptr().offset(ch as isize),
            );
        } else {
            let mut ptr_spec_coeff_1: *mut WORD32 = ((*pstr_usac_data)
                .tcx_spec_coeffs[ch as usize])
                .as_mut_ptr();
            memset(
                ptr_spec_coeff_1 as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ((*pstr_usac_data).ccfl as size_t)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
            );
        }
        if frame_ok == 0 {
            memcpy(
                ((*pstr_usac_data).lpc4_lsf).as_mut_ptr() as *mut core::ffi::c_void,
                ((*pstr_ec_state).lsf4).as_mut_ptr() as *const core::ffi::c_void,
                ::core::mem::size_of::<[FLOAT32; 16]>() as size_t,
            );
        }
    }
    (*pstr_ec_state).prev_frame_ok[0 as core::ffi::c_int as usize] = (*pstr_ec_state)
        .prev_frame_ok[1 as core::ffi::c_int as usize];
    (*pstr_ec_state).prev_frame_ok[1 as core::ffi::c_int as usize] = frame_ok;
}
unsafe extern "C" fn ixheaacd_lpc_wt_tool(mut a: *mut FLOAT32, mut l: WORD32) -> VOID {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < l {
        *a.offset(i as isize) = *a.offset(i as isize) * ixheaacd_gamma_table[i as usize];
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_lpc_coef_gen_ec(
    mut lsf_old: *mut FLOAT32,
    mut lsf_new: *mut FLOAT32,
    mut a: *mut FLOAT32,
    mut m: WORD32,
) -> VOID {
    let mut lsf: [FLOAT32; 16] = [0.; 16];
    let mut ptr_a: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut inc: FLOAT32 = 0.;
    let mut fnew: FLOAT32 = 0.;
    let mut fold: FLOAT32 = 0.;
    let mut i: WORD32 = 0;
    ptr_a = a as *mut FLOAT32;
    inc = 1.0f32 / m as FLOAT32;
    fnew = 0.5f32 - 0.5f32 * inc;
    fold = 1.0f32 - fnew;
    i = 0 as core::ffi::c_int as WORD32;
    while i < ORDER {
        lsf[i as usize] = *lsf_old.offset(i as isize) * fold
            + *lsf_new.offset(i as isize) * fnew;
        i += 1;
    }
    ixheaacd_lsp_to_lp_conversion(lsf.as_mut_ptr(), ptr_a);
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_usac_tcx_ec(
    mut pstr_usac_data: *mut ia_usac_data_struct,
    mut st: ia_usac_lpd_decoder_handle,
    mut ptr_lsp_curr: *mut FLOAT32,
    mut frame_idx: WORD32,
    mut lp_flt_coff_a: *mut FLOAT32,
) -> VOID {
    let mut ch: WORD32 = (*pstr_usac_data).present_chan;
    let mut synth_buf: [FLOAT32; 272] = [0.; 272];
    let mut temp: FLOAT32 = 0.;
    let mut exc_buf: [FLOAT32; 684] = [0.; 684];
    let mut ptr_syn: *mut FLOAT32 = synth_buf.as_mut_ptr().offset(ORDER as isize);
    let mut ptr_exc: *mut FLOAT32 = exc_buf
        .as_mut_ptr()
        .offset(MAX_PITCH as isize)
        .offset(ORDER as isize)
        .offset(1 as core::ffi::c_int as isize);
    let mut est_fac_est: FLOAT32 = 0.1f32;
    let mut i: WORD32 = 0;
    let mut sf_idx: WORD32 = 0;
    let mut synth_sig_buf: [FLOAT32; 65] = [0.; 65];
    let mut synth_signal: *mut FLOAT32 = synth_sig_buf
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as isize);
    let mut num_lost_frames: WORD32 = (*pstr_usac_data).num_lost_lpd_frames[ch as usize];
    let mut len_subfrm: WORD32 = (*pstr_usac_data).len_subfrm;
    let mut past_tcx_gain: FLOAT32 = (*pstr_usac_data).past_gain_tcx[ch as usize];
    let mut l_div_part: WORD32 = MAX_PITCH + ORDER + 1 as WORD32 - len_subfrm;
    let mut synth: *mut FLOAT32 = ((*pstr_usac_data).synth_buf)
        .as_mut_ptr()
        .offset(MAX_PITCH as isize)
        .offset(-(LEN_SUBFR as isize));
    let mut ptr_synth: *mut FLOAT32 = &mut *synth
        .offset((512 as WORD32 + frame_idx * len_subfrm) as isize) as *mut FLOAT32;
    let mut syn_buf: [FLOAT32; 428] = [0.; 428];
    let mut ptr_syn_buf: *mut FLOAT32 = &mut *syn_buf.as_mut_ptr().offset(ORDER as isize)
        as *mut FLOAT32;
    memcpy(
        syn_buf.as_mut_ptr() as *mut core::ffi::c_void,
        &mut *ptr_synth.offset(-(MAX_PITCH + ORDER + 1 as core::ffi::c_int) as isize)
            as *mut FLOAT32 as *const core::ffi::c_void,
        ::core::mem::size_of::<[FLOAT32; 428]>() as size_t,
    );
    memcpy(
        ((*st).synth_prev_ec).as_mut_ptr() as *mut core::ffi::c_void,
        &mut *syn_buf.as_mut_ptr().offset((MAX_PITCH + 1 as core::ffi::c_int) as isize)
            as *mut FLOAT32 as *const core::ffi::c_void,
        ::core::mem::size_of::<[FLOAT32; 16]>() as size_t,
    );
    ixheaacd_residual_tool_float(
        ((*pstr_usac_data).lp_flt_coff_a_ec).as_mut_ptr(),
        ptr_syn_buf,
        ((*st).xcitation_prev).as_mut_ptr() as *mut FLOAT32,
        (*pstr_usac_data).len_subfrm,
        1 as WORD32,
    );
    ixheaacd_residual_tool_float(
        lp_flt_coff_a,
        &mut *syn_buf.as_mut_ptr().offset(l_div_part as isize),
        ((*st).xcitation_prev).as_mut_ptr().offset(l_div_part as isize),
        (*pstr_usac_data).len_subfrm,
        1 as WORD32,
    );
    if (*st).last_tcx_pitch > MAX_PITCH {
        (*st).last_tcx_pitch = MAX_PITCH as WORD32;
    }
    memcpy(
        synth_buf.as_mut_ptr() as *mut core::ffi::c_void,
        ((*st).synth_prev_ec).as_mut_ptr() as *const core::ffi::c_void,
        (ORDER as size_t).wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
    );
    memcpy(
        exc_buf.as_mut_ptr() as *mut core::ffi::c_void,
        ((*st).xcitation_prev).as_mut_ptr() as *const core::ffi::c_void,
        ((MAX_PITCH + ORDER + 1 as core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
    );
    if num_lost_frames <= 8 as core::ffi::c_int {
        est_fac_est = ixheaacd_exc_fade_fac[(num_lost_frames as core::ffi::c_int
            - 1 as core::ffi::c_int) as usize];
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < len_subfrm {
        *ptr_exc.offset(i as isize) = est_fac_est
            * *ptr_exc.offset((i - (*st).last_tcx_pitch) as isize);
        i += 1;
    }
    *synth_signal.offset(-(1 as core::ffi::c_int) as isize) = *ptr_exc
        .offset(-(1 as core::ffi::c_int) as isize);
    sf_idx = 0 as core::ffi::c_int as WORD32;
    while sf_idx < len_subfrm {
        let mut lp_coef: [FLOAT32; 17] = [0.; 17];
        ixheaacd_lpc_coef_gen_ec(
            ((*st).lspold).as_mut_ptr(),
            ptr_lsp_curr as *mut FLOAT32,
            lp_coef.as_mut_ptr(),
            len_subfrm / LEN_SUBFR,
        );
        ixheaacd_synthesis_tool_float(
            lp_coef.as_mut_ptr(),
            &mut *ptr_exc.offset(sf_idx as isize),
            &mut *ptr_syn.offset(sf_idx as isize),
            LEN_SUBFR,
            synth_buf.as_mut_ptr(),
        );
        ixheaacd_lpc_wt_tool(lp_coef.as_mut_ptr(), ORDER);
        ixheaacd_residual_tool_float(
            lp_coef.as_mut_ptr(),
            &mut *ptr_syn.offset(sf_idx as isize),
            synth_signal,
            LEN_SUBFR,
            1 as WORD32,
        );
        ixheaacd_deemphsis_tool(
            synth_signal,
            LEN_SUBFR,
            *synth_signal.offset(-(1 as core::ffi::c_int) as isize),
        );
        temp = est_fac_est * past_tcx_gain;
        i = 0 as core::ffi::c_int as WORD32;
        while i < LEN_SUBFR {
            if *synth_signal.offset(i as isize) > temp {
                *synth_signal.offset(i as isize) = temp;
            } else if *synth_signal.offset(i as isize) < -temp {
                *synth_signal.offset(i as isize) = -temp;
            }
            i += 1;
        }
        i = (LEN_SUBFR - 1 as core::ffi::c_int) as WORD32;
        while i >= 0 as core::ffi::c_int {
            *synth_signal.offset(i as isize) = *synth_signal.offset(i as isize)
                - PREEMPH_FILT_FAC
                    * *synth_signal
                        .offset(
                            (i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                        );
            i -= 1;
        }
        ixheaacd_synthesis_tool_float(
            lp_coef.as_mut_ptr(),
            synth_signal as *mut FLOAT32,
            &mut *ptr_syn.offset(sf_idx as isize),
            LEN_SUBFR,
            synth_buf.as_mut_ptr(),
        );
        memmove(
            &mut *ptr_synth.offset(sf_idx as isize) as *mut FLOAT32
                as *mut core::ffi::c_void,
            &mut *ptr_syn.offset(sf_idx as isize) as *mut FLOAT32
                as *const core::ffi::c_void,
            (LEN_SUBFR as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        sf_idx += LEN_SUBFR;
    }
    memcpy(
        ((*st).xcitation_prev).as_mut_ptr() as *mut core::ffi::c_void,
        ptr_exc
            .offset(len_subfrm as isize)
            .offset(-((MAX_PITCH + ORDER + 1 as core::ffi::c_int) as isize))
            as *const core::ffi::c_void,
        (::core::mem::size_of::<FLOAT32>() as size_t)
            .wrapping_mul((MAX_PITCH + ORDER + 1 as core::ffi::c_int) as size_t),
    );
    memcpy(
        ((*st).synth_prev_ec).as_mut_ptr() as *mut core::ffi::c_void,
        synth_buf.as_mut_ptr().offset(len_subfrm as isize) as *const core::ffi::c_void,
        (::core::mem::size_of::<FLOAT32>() as size_t).wrapping_mul(ORDER as size_t),
    );
}
