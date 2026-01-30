extern "C" {
    pub type ia_sbr_dec_inst_struct;
    fn pow(__x: core::ffi::c_double, __y: core::ffi::c_double) -> core::ffi::c_double;
    fn sqrt(__x: core::ffi::c_double) -> core::ffi::c_double;
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
    fn ixheaacd_acelp_mdct(
        ptr_in: *mut WORD32,
        ptr_out: *mut WORD32,
        preshift: *mut WORD32,
        length: WORD32,
        ptr_scratch: *mut WORD32,
    ) -> VOID;
    fn ixheaacd_acelp_mdct_main(
        usac_data: *mut ia_usac_data_struct,
        x: *mut WORD32,
        y: *mut WORD32,
        l: WORD32,
        m: WORD32,
        preshift: *mut WORD32,
    ) -> VOID;
    fn ixheaacd_mem_cpy(x: *const FLOAT32, y: *mut FLOAT32, n: WORD32) -> VOID;
    fn ixheaacd_vec_cnst_mul(
        a: FLOAT32,
        x: *mut FLOAT32,
        z: *mut FLOAT32,
        n: WORD32,
    ) -> VOID;
    fn ixheaacd_lpc_to_td(
        lpc_coeffs: *mut FLOAT32,
        lpc_order: WORD32,
        mdct_gains: *mut FLOAT32,
        lg: WORD32,
    ) -> VOID;
    fn ixheaacd_noise_shaping(
        x: *mut FLOAT32,
        lg: WORD32,
        fdns_npts: WORD32,
        old_gains: *mut FLOAT32,
        new_gains: *mut FLOAT32,
    ) -> VOID;
    static ixheaacd_sine_window96: [FLOAT32; 96];
    static ixheaacd_sine_window128: [FLOAT32; 128];
    static ixheaacd_sine_window192: [FLOAT32; 192];
    static ixheaacd__sine_window256: [FLOAT32; 256];
    fn ixheaacd_synthesis_tool_float(
        a: *mut FLOAT32,
        x: *mut FLOAT32,
        y: *mut FLOAT32,
        l: WORD32,
        mem: *mut FLOAT32,
    ) -> VOID;
    fn ixheaacd_residual_tool_float(
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
pub type UWORD64 = core::ffi::c_ulonglong;
pub type FLOAT32 = core::ffi::c_float;
pub type VOID = ();
pub type WORD = core::ffi::c_int;
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
pub const NUM_FRAMES: core::ffi::c_int = 4 as core::ffi::c_int;
pub const ORDER: core::ffi::c_int = 16 as core::ffi::c_int;
pub const FAC_LENGTH: core::ffi::c_int = 128 as core::ffi::c_int;
pub const FSCALE_DENOM: core::ffi::c_int = 12800 as core::ffi::c_int;
pub const FAC_FSCALE_MAX: core::ffi::c_int = 24000 as core::ffi::c_int;
pub const TMIN: core::ffi::c_int = 34 as core::ffi::c_int;
pub const TMAX: core::ffi::c_int = 27 as core::ffi::c_int + 6 as core::ffi::c_int * TMIN;
pub const INTER_LP_FIL_ORDER: core::ffi::c_int = 16 as core::ffi::c_int;
pub const MAX_PITCH: core::ffi::c_int = TMAX
    + 6 as core::ffi::c_int
        * ((FAC_FSCALE_MAX * TMIN + FSCALE_DENOM / 2 as core::ffi::c_int) / FSCALE_DENOM
            - TMIN);
pub const PREEMPH_FILT_FAC: core::ffi::c_float = 0.68f32;
pub const LEN_SUBFR: core::ffi::c_int = 64 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_lpc_coeff_wt_apply(
    mut a: *mut FLOAT32,
    mut ap: *mut FLOAT32,
) -> VOID {
    let mut f: FLOAT32 = 0.;
    let mut i: WORD32 = 0;
    *ap.offset(0 as core::ffi::c_int as isize) = *a
        .offset(0 as core::ffi::c_int as isize);
    f = 0.92f32 as FLOAT32;
    i = 1 as core::ffi::c_int as WORD32;
    while i <= 16 as core::ffi::c_int {
        *ap.offset(i as isize) = f * *a.offset(i as isize);
        f *= 0.92f32;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_float2fix(
    mut x: *mut FLOAT32,
    mut int_x: *mut WORD32,
    mut length: WORD32,
) -> WORD8 {
    let mut k: WORD32 = 0;
    let mut itemp: WORD32 = 0;
    let mut ftemp: FLOAT32 = 0.0f32;
    let mut shiftp: WORD8 = 0;
    k = 0 as core::ffi::c_int as WORD32;
    while k < length {
        if (if *x.offset(k as isize) < 0 as core::ffi::c_int as FLOAT32 {
            -*x.offset(k as isize)
        } else {
            *x.offset(k as isize)
        }) > ftemp
        {
            ftemp = if *x.offset(k as isize) < 0 as core::ffi::c_int as FLOAT32 {
                -*x.offset(k as isize)
            } else {
                *x.offset(k as isize)
            };
        }
        k += 1;
    }
    itemp = ftemp as WORD32;
    shiftp = ixheaac_norm32(itemp) as WORD8;
    k = 0 as core::ffi::c_int as WORD32;
    while k < length {
        *int_x.offset(k as isize) = (*x.offset(k as isize)
            * ((1 as core::ffi::c_int as WORD64) << shiftp as core::ffi::c_int)
                as FLOAT32) as WORD32;
        k += 1;
    }
    return shiftp;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_fix2float(
    mut int_xn1: *mut WORD32,
    mut xn1: *mut FLOAT32,
    mut length: WORD32,
    mut shiftp: *mut WORD8,
    mut preshift: *mut WORD32,
) -> VOID {
    let mut k: WORD32 = 0;
    let mut qfac: FLOAT32 = 0.;
    if *shiftp as WORD32 - *preshift > 0 as core::ffi::c_int {
        qfac = 1.0f32
            / ((1 as core::ffi::c_int as WORD64) << *shiftp as WORD32 - *preshift)
                as FLOAT32;
        k = 0 as core::ffi::c_int as WORD32;
        while k < length {
            *xn1.offset(k as isize) = *int_xn1.offset(k as isize) as FLOAT32 * qfac;
            k += 1;
        }
    } else {
        k = 0 as core::ffi::c_int as WORD32;
        while k < length {
            *xn1.offset(k as isize) = *int_xn1.offset(k as isize) as FLOAT32
                * ((1 as core::ffi::c_int as WORD64)
                    << *preshift - *shiftp as core::ffi::c_int) as FLOAT32;
            k += 1;
        }
    };
}
unsafe extern "C" fn ixheaacd_low_fq_deemphasis(
    mut x: *mut FLOAT32,
    mut lg: WORD32,
    mut gains: *mut FLOAT32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut i_max: WORD32 = 0;
    let mut max: FLOAT32 = 0.;
    let mut factor: FLOAT32 = 0.;
    let mut rm: FLOAT32 = 0.;
    k = 8 as core::ffi::c_int as WORD32;
    i_max = (lg as core::ffi::c_int / 4 as core::ffi::c_int) as WORD32;
    max = 0.01f32 as FLOAT32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < i_max {
        rm = 0.01f32 as FLOAT32;
        j = i;
        while j < i + k {
            rm += *x.offset(j as isize) * *x.offset(j as isize);
            j += 1;
        }
        if rm > max {
            max = rm;
        }
        i += k;
    }
    factor = 0.1f32 as FLOAT32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < i_max {
        rm = 0.01f32 as FLOAT32;
        j = i;
        while j < i + k {
            rm += *x.offset(j as isize) * *x.offset(j as isize);
            j += 1;
        }
        rm = sqrt((rm / max) as core::ffi::c_double) as FLOAT32;
        if rm > factor {
            factor = rm;
        }
        j = i;
        while j < i + k {
            *x.offset(j as isize) *= factor;
            j += 1;
        }
        *gains.offset((i / k) as isize) = factor;
        i += k;
    }
}
unsafe extern "C" fn ixheaacd_calc_max_pitch(
    mut x: *mut FLOAT32,
    mut n: WORD32,
) -> WORD32 {
    let mut max_m: FLOAT32 = 0.;
    let mut t_est: FLOAT32 = 0.;
    let mut i: WORD32 = 0;
    let mut i_max: WORD32 = 0;
    let mut pitch_tcx: WORD32 = 0;
    max_m = 0 as core::ffi::c_int as FLOAT32;
    i_max = 1 as core::ffi::c_int as WORD32;
    i = 1 as core::ffi::c_int as WORD32;
    while i < n {
        let mut mag: FLOAT32 = *x.offset((2 as WORD32 * i) as isize)
            * *x.offset((2 as WORD32 * i) as isize)
            + *x
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                )
                * *x
                    .offset(
                        (2 as core::ffi::c_int * i as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    );
        if mag > max_m {
            max_m = mag;
            i_max = i;
        }
        i += 1;
    }
    t_est = n as FLOAT32 / i_max as FLOAT32;
    if t_est >= 256 as core::ffi::c_int as FLOAT32 {
        pitch_tcx = 256 as core::ffi::c_int as WORD32;
    } else {
        let mut tmp_est: FLOAT32 = t_est;
        while tmp_est < 256 as core::ffi::c_int as FLOAT32 {
            tmp_est += t_est;
        }
        pitch_tcx = (tmp_est - t_est) as WORD32;
    }
    return pitch_tcx;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_tcx_mdct(
    mut usac_data: *mut ia_usac_data_struct,
    mut pstr_td_frame_data: *mut ia_td_frame_data_struct,
    mut frame_index: WORD32,
    mut lp_flt_coff_a: *mut FLOAT32,
    mut lg: WORD32,
    mut st: ia_usac_lpd_decoder_handle,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut mode: WORD32 = 0;
    let mut ptr_tcx_quant: *mut WORD32 = 0 as *mut WORD32;
    let mut tmp: FLOAT32 = 0.;
    let mut gain_tcx: FLOAT32 = 0.0f32;
    let mut noise_level: FLOAT32 = 0.;
    let mut energy: FLOAT32 = 0.;
    let mut temp: FLOAT32 = 0.;
    let mut ptr_a: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut i_ap: [FLOAT32; 17] = [0.; 17];
    let mut sine_window_prev: *const FLOAT32 = 0 as *const FLOAT32;
    let mut sine_window: *const FLOAT32 = 0 as *const FLOAT32;
    let mut fac_length_prev: WORD32 = 0;
    let mut alfd_gains: [FLOAT32; 32] = [
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
    ];
    let mut x: [FLOAT32; 1024] = [0.; 1024];
    let mut buf: [FLOAT32; 1040] = [0.; 1040];
    let mut int_x: [WORD32; 1280] = [0; 1280];
    let mut int_xn1: [WORD32; 1280] = [0; 1280];
    let mut gain1: [FLOAT32; 1024] = [0.; 1024];
    let mut gain2: [FLOAT32; 1024] = [0.; 1024];
    let mut xn_buf: [FLOAT32; 1280] = [0.; 1280];
    let mut xn: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut xn1: [FLOAT32; 256] = [0.; 256];
    let mut facwindow: [FLOAT32; 256] = [0.; 256];
    let mut TTT: WORD32 = 0;
    let mut shiftp: WORD8 = 0 as WORD8;
    let mut preshift: WORD32 = 0 as WORD32;
    let mut loop_count: WORD32 = 0 as WORD32;
    let mut exc: *mut FLOAT32 = &mut *((*usac_data).exc_buf)
        .as_mut_ptr()
        .offset(
            ((*usac_data).len_subfrm as core::ffi::c_int
                * frame_index as core::ffi::c_int + MAX_PITCH + INTER_LP_FIL_ORDER
                + 1 as core::ffi::c_int) as isize,
        ) as *mut FLOAT32;
    let mut synth: *mut FLOAT32 = &mut *((*usac_data).synth_buf)
        .as_mut_ptr()
        .offset(
            ((*usac_data).len_subfrm as core::ffi::c_int
                * frame_index as core::ffi::c_int + MAX_PITCH
                + (NUM_FRAMES * (*usac_data).num_subfrm as core::ffi::c_int
                    / 2 as core::ffi::c_int - 1 as core::ffi::c_int) * LEN_SUBFR)
                as isize,
        ) as *mut FLOAT32;
    let mut ptr_scratch: *mut WORD32 = &mut *((*usac_data).scratch_buffer)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
    let mut fac_length: WORD32 = (*usac_data).len_subfrm / 2 as WORD32;
    let mut err: WORD32 = 0 as WORD32;
    mode = lg / (*usac_data).len_subfrm;
    if mode > 2 as core::ffi::c_int {
        mode = 3 as core::ffi::c_int as WORD32;
    }
    if (*st).mode_prev == -(2 as core::ffi::c_int) {
        fac_length_prev = ((*usac_data).ccfl as core::ffi::c_int
            / 16 as core::ffi::c_int) as WORD32;
    } else {
        fac_length_prev = fac_length;
    }
    if fac_length == 96 as core::ffi::c_int {
        sine_window = ixheaacd_sine_window192.as_ptr();
    } else {
        sine_window = ixheaacd__sine_window256.as_ptr();
    }
    if fac_length_prev == 48 as core::ffi::c_int {
        sine_window_prev = ixheaacd_sine_window96.as_ptr();
    } else if fac_length_prev == 64 as core::ffi::c_int {
        sine_window_prev = ixheaacd_sine_window128.as_ptr();
    } else if fac_length_prev == 96 as core::ffi::c_int {
        sine_window_prev = ixheaacd_sine_window192.as_ptr();
    } else {
        sine_window_prev = ixheaacd__sine_window256.as_ptr();
    }
    xn = xn_buf.as_mut_ptr().offset(fac_length as isize);
    if (*st).mode_prev != 0 as core::ffi::c_int {
        if (*st).mode_prev > 0 as core::ffi::c_int {
            i = 0 as core::ffi::c_int as WORD32;
            while i < 2 as WORD32 * fac_length_prev {
                (*st)
                    .exc_prev[(i as core::ffi::c_int + fac_length as core::ffi::c_int
                    - fac_length_prev as core::ffi::c_int + 1 as core::ffi::c_int)
                    as usize]
                    *= *sine_window_prev
                        .offset(
                            (2 as WORD32 * fac_length_prev - 1 as WORD32 - i) as isize,
                        );
                i += 1;
            }
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < fac_length - fac_length_prev {
            (*st)
                .exc_prev[(i as core::ffi::c_int + fac_length as core::ffi::c_int
                + fac_length_prev as core::ffi::c_int + 1 as core::ffi::c_int)
                as usize] = 0.0f32 as FLOAT32;
            i += 1;
        }
    }
    if (*usac_data).frame_ok == 1 as core::ffi::c_int {
        noise_level = 0.0625f32
            * (8.0f32
                - (*pstr_td_frame_data).noise_factor[frame_index as usize] as FLOAT32);
        ptr_tcx_quant = ((*pstr_td_frame_data).x_tcx_invquant).as_mut_ptr();
        i = 0 as core::ffi::c_int as WORD32;
        while i < frame_index {
            ptr_tcx_quant = ptr_tcx_quant
                .offset((*pstr_td_frame_data).tcx_lg[i as usize] as isize);
            i += 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < lg {
            x[i as usize] = *ptr_tcx_quant.offset(i as isize) as FLOAT32;
            i += 1;
        }
        if (*usac_data).ec_flag != 0 {
            (*st).last_tcx_pitch = ixheaacd_calc_max_pitch(
                x.as_mut_ptr(),
                lg >> 5 as core::ffi::c_int,
            );
        }
        i = (lg as core::ffi::c_int / 6 as core::ffi::c_int) as WORD32;
        while i < lg {
            let mut k: WORD32 = 0;
            let mut max_k: WORD32 = if lg < i as core::ffi::c_int + 8 as core::ffi::c_int
            {
                lg
            } else {
                i + 8 as WORD32
            };
            let mut tmp_0: FLOAT32 = 0.0f32;
            k = i;
            while k < max_k {
                tmp_0
                    += (*ptr_tcx_quant.offset(k as isize)
                        * *ptr_tcx_quant.offset(k as isize)) as FLOAT32;
                k += 1;
            }
            if tmp_0 == 0.0f32 {
                k = i;
                while k < max_k {
                    x[k as usize] = noise_level
                        * ixheaacd_randomsign(
                            &mut *((*usac_data).seed_value)
                                .as_mut_ptr()
                                .offset((*usac_data).present_chan as isize),
                        );
                    k += 1;
                }
            }
            i += 8 as core::ffi::c_int;
        }
        ixheaacd_low_fq_deemphasis(x.as_mut_ptr(), lg, alfd_gains.as_mut_ptr());
        ixheaacd_lpc_coeff_wt_apply(
            lp_flt_coff_a.offset((ORDER + 1 as core::ffi::c_int) as isize),
            i_ap.as_mut_ptr(),
        );
        ixheaacd_lpc_to_td(
            i_ap.as_mut_ptr(),
            ORDER,
            gain1.as_mut_ptr(),
            (*usac_data).len_subfrm / 4 as WORD32,
        );
        ixheaacd_lpc_coeff_wt_apply(
            lp_flt_coff_a
                .offset(
                    (2 as core::ffi::c_int * (ORDER + 1 as core::ffi::c_int)) as isize,
                ),
            i_ap.as_mut_ptr(),
        );
        ixheaacd_lpc_to_td(
            i_ap.as_mut_ptr(),
            ORDER,
            gain2.as_mut_ptr(),
            (*usac_data).len_subfrm / 4 as WORD32,
        );
        energy = 0.01f32 as FLOAT32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < lg {
            energy += x[i as usize] * x[i as usize];
            i += 1;
        }
        temp = sqrt(energy as core::ffi::c_double) as FLOAT32 / lg as FLOAT32;
        gain_tcx = (pow(
            10.0f64,
            ((*pstr_td_frame_data).global_gain[frame_index as usize]
                as core::ffi::c_float / 28.0f32) as core::ffi::c_double,
        ) as core::ffi::c_float / (temp as core::ffi::c_float * 2.0f32)) as FLOAT32;
    }
    if (*usac_data).ec_flag != 0 {
        if (*usac_data).frame_ok == 1 as core::ffi::c_int {
            (*usac_data).past_gain_tcx[(*usac_data).present_chan as usize] = gain_tcx;
        } else {
            gain_tcx = (*usac_data).past_gain_tcx[(*usac_data).present_chan as usize];
        }
    }
    if (*usac_data).frame_ok == 1 as core::ffi::c_int {
        ixheaacd_noise_shaping(
            x.as_mut_ptr(),
            lg,
            (*usac_data).len_subfrm / 4 as WORD32,
            gain1.as_mut_ptr(),
            gain2.as_mut_ptr(),
        );
        shiftp = ixheaacd_float2fix(x.as_mut_ptr(), int_x.as_mut_ptr(), lg);
    }
    if (*usac_data).ec_flag == 1 as core::ffi::c_int {
        if (*st).mode_prev != 0 as core::ffi::c_int {
            if (*usac_data).frame_ok == 1 as core::ffi::c_int {
                memcpy(
                    ((*usac_data).tcx_spec_coeffs[(*usac_data).present_chan as usize])
                        .as_mut_ptr() as *mut core::ffi::c_void,
                    int_x.as_mut_ptr() as *const core::ffi::c_void,
                    (lg as size_t)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
                );
                (*usac_data).last_shiftp = shiftp as WORD32;
            } else {
                memcpy(
                    int_x.as_mut_ptr() as *mut core::ffi::c_void,
                    ((*usac_data).tcx_spec_coeffs[(*usac_data).present_chan as usize])
                        .as_mut_ptr() as *const core::ffi::c_void,
                    (lg as size_t)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
                );
                shiftp = (*usac_data).last_shiftp as WORD8;
            }
        }
    } else if lg as core::ffi::c_int & lg as core::ffi::c_int - 1 as core::ffi::c_int
        != 0
    {
        if lg != 48 as core::ffi::c_int && lg != 96 as core::ffi::c_int
            && lg != 192 as core::ffi::c_int && lg != 384 as core::ffi::c_int
            && lg != 768 as core::ffi::c_int
        {
            return -(1 as WORD32);
        }
    }
    ixheaacd_acelp_mdct_main(
        usac_data,
        int_x.as_mut_ptr(),
        int_xn1.as_mut_ptr(),
        2 as WORD32 * fac_length,
        lg - 2 as WORD32 * fac_length,
        &mut preshift,
    );
    ixheaacd_fix2float(
        int_xn1.as_mut_ptr(),
        xn_buf.as_mut_ptr(),
        lg + 2 as WORD32 * fac_length,
        &mut shiftp,
        &mut preshift,
    );
    ixheaacd_vec_cnst_mul(
        2.0f32 / lg as FLOAT32,
        xn_buf.as_mut_ptr(),
        xn_buf.as_mut_ptr(),
        lg + 2 as WORD32 * fac_length,
    );
    (*st).fac_gain = gain_tcx * 0.5f32
        * sqrt((fac_length as FLOAT32 / lg as FLOAT32) as core::ffi::c_double)
            as FLOAT32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < fac_length as core::ffi::c_int / 4 as core::ffi::c_int {
        (*st).fac_fd_data[i as usize] = alfd_gains[(i * lg / (8 as WORD32 * fac_length))
            as usize];
        i += 1;
    }
    if (*st).mode_prev == 0 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < fac_length_prev {
            facwindow[i as usize] = *sine_window_prev.offset(i as isize)
                * *sine_window_prev
                    .offset((2 as WORD32 * fac_length_prev - 1 as WORD32 - i) as isize);
            facwindow[(fac_length_prev + i) as usize] = 1.0f32
                - *sine_window_prev.offset((fac_length_prev + i) as isize)
                    * *sine_window_prev.offset((fac_length_prev + i) as isize);
            i += 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < fac_length as core::ffi::c_int / 2 as core::ffi::c_int {
            x[i as usize] = (*st).fac_gain
                * (*pstr_td_frame_data)
                    .fac[(frame_index * FAC_LENGTH + 2 as WORD32 * i) as usize]
                    as FLOAT32;
            x[(fac_length / 2 as WORD32 + i) as usize] = (*st).fac_gain
                * (*pstr_td_frame_data)
                    .fac[(frame_index as core::ffi::c_int * FAC_LENGTH
                    + fac_length as core::ffi::c_int
                    - 2 as core::ffi::c_int * i as core::ffi::c_int
                    - 1 as core::ffi::c_int) as usize] as FLOAT32;
            i += 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < fac_length as core::ffi::c_int / 8 as core::ffi::c_int {
            x[i as usize] *= (*st).fac_fd_data[(2 as WORD32 * i) as usize];
            x[(fac_length as core::ffi::c_int - i as core::ffi::c_int
                - 1 as core::ffi::c_int) as usize]
                *= (*st)
                    .fac_fd_data[(2 as core::ffi::c_int * i as core::ffi::c_int
                    + 1 as core::ffi::c_int) as usize];
            i += 1;
        }
        preshift = 0 as core::ffi::c_int as WORD32;
        shiftp = ixheaacd_float2fix(x.as_mut_ptr(), int_x.as_mut_ptr(), fac_length);
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
        ixheaacd_acelp_mdct(
            int_x.as_mut_ptr(),
            int_xn1.as_mut_ptr(),
            &mut preshift,
            fac_length,
            ptr_scratch,
        );
        ixheaacd_fix2float(
            int_xn1.as_mut_ptr(),
            xn1.as_mut_ptr(),
            fac_length,
            &mut shiftp,
            &mut preshift,
        );
        ixheaacd_vec_cnst_mul(
            2.0f32 / fac_length as FLOAT32,
            xn1.as_mut_ptr(),
            xn1.as_mut_ptr(),
            fac_length,
        );
        memset(
            xn1.as_mut_ptr().offset(fac_length as isize) as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (fac_length as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        ixheaacd_lpc_coeff_wt_apply(
            lp_flt_coff_a.offset((ORDER + 1 as core::ffi::c_int) as isize),
            i_ap.as_mut_ptr(),
        );
        ixheaacd_synthesis_tool_float(
            i_ap.as_mut_ptr(),
            xn1.as_mut_ptr(),
            xn1.as_mut_ptr(),
            2 as WORD32 * fac_length,
            xn1.as_mut_ptr().offset(fac_length as isize),
        );
        i = 0 as core::ffi::c_int as WORD32;
        while i < fac_length {
            temp = (*st).exc_prev[(1 as WORD32 + fac_length + i) as usize]
                * facwindow[(fac_length + i) as usize]
                + (*st).exc_prev[(fac_length - i) as usize]
                    * facwindow[(fac_length - 1 as WORD32 - i) as usize];
            xn1[i as usize] += temp;
            i += 1;
        }
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < lg + 2 as WORD32 * fac_length {
        xn_buf[i as usize] *= gain_tcx;
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < 2 as WORD32 * fac_length_prev {
        xn_buf[(i + fac_length - fac_length_prev) as usize]
            *= *sine_window_prev.offset(i as isize);
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < fac_length - fac_length_prev {
        xn_buf[i as usize] = 0.0f32 as FLOAT32;
        i += 1;
    }
    if (*st).mode_prev != 0 as core::ffi::c_int {
        i = fac_length - fac_length_prev;
        while i < fac_length + fac_length_prev {
            xn_buf[i as usize] += (*st).exc_prev[(1 as WORD32 + i) as usize];
            i += 1;
        }
    } else {
        i = fac_length - fac_length_prev;
        while i < fac_length + fac_length_prev {
            xn_buf[(i + fac_length) as usize] += xn1[i as usize];
            i += 1;
        }
    }
    ixheaacd_mem_cpy(
        xn_buf.as_mut_ptr().offset(lg as isize).offset(-(1 as core::ffi::c_int as isize))
            as *const FLOAT32,
        ((*st).exc_prev).as_mut_ptr(),
        1 as WORD32 + 2 as WORD32 * fac_length,
    );
    i = 0 as core::ffi::c_int as WORD32;
    while i < 2 as WORD32 * fac_length {
        xn_buf[(i + lg) as usize]
            *= *sine_window
                .offset((2 as WORD32 * fac_length - 1 as WORD32 - i) as isize);
        i += 1;
    }
    if (*st).mode_prev != 0 as core::ffi::c_int {
        ixheaacd_mem_cpy(
            xn_buf
                .as_mut_ptr()
                .offset(fac_length as isize)
                .offset(-(fac_length_prev as isize)) as *const FLOAT32,
            synth.offset(-(fac_length_prev as isize)),
            fac_length_prev,
        );
        i = 0 as core::ffi::c_int as WORD32;
        while i < ORDER + fac_length {
            buf[i as usize] = *synth.offset((i - ORDER - fac_length) as isize)
                - PREEMPH_FILT_FAC
                    * *synth
                        .offset(
                            (i as core::ffi::c_int - ORDER
                                - fac_length as core::ffi::c_int - 1 as core::ffi::c_int)
                                as isize,
                        );
            i += 1;
        }
        ptr_a = ((*st).lp_flt_coeff_a_prev).as_mut_ptr() as *mut FLOAT32;
        TTT = (fac_length as core::ffi::c_int % LEN_SUBFR) as WORD32;
        if TTT != 0 as core::ffi::c_int {
            ixheaacd_residual_tool_float(
                ptr_a,
                &mut *buf.as_mut_ptr().offset(ORDER as isize),
                &mut *exc.offset(-fac_length as isize),
                TTT,
                1 as WORD32,
            );
        }
        loop_count = ((fac_length as core::ffi::c_int - TTT as core::ffi::c_int)
            / LEN_SUBFR) as WORD32;
        ixheaacd_residual_tool_float(
            ptr_a,
            &mut *buf.as_mut_ptr().offset((ORDER + TTT) as isize),
            &mut *exc.offset((TTT - fac_length) as isize),
            LEN_SUBFR,
            loop_count,
        );
    }
    ixheaacd_mem_cpy(xn as *const FLOAT32, synth as *mut FLOAT32, lg);
    ixheaacd_mem_cpy(
        synth.offset(-(ORDER as isize)).offset(-(1 as core::ffi::c_int as isize))
            as *const FLOAT32,
        xn.offset(-(ORDER as isize)).offset(-(1 as core::ffi::c_int as isize)),
        ORDER + 1 as WORD32,
    );
    tmp = *xn.offset((-ORDER - 1 as core::ffi::c_int) as isize);
    ixheaacd_preemphsis_tool_float(
        xn.offset(-(ORDER as isize)),
        PREEMPH_FILT_FAC,
        ORDER + lg,
        tmp,
    );
    ptr_a = lp_flt_coff_a
        .offset((2 as core::ffi::c_int * (ORDER + 1 as core::ffi::c_int)) as isize)
        as *mut FLOAT32;
    ixheaacd_residual_tool_float(ptr_a, xn, exc, lg, 1 as WORD32);
    ixheaacd_mem_cpy(
        ptr_a as *const FLOAT32,
        ((*st).lp_flt_coeff_a_prev).as_mut_ptr() as *mut FLOAT32,
        ORDER + 1 as WORD32,
    );
    ixheaacd_mem_cpy(
        ptr_a as *const FLOAT32,
        ((*st).lp_flt_coeff_a_prev)
            .as_mut_ptr()
            .offset(ORDER as isize)
            .offset(1 as core::ffi::c_int as isize),
        ORDER + 1 as WORD32,
    );
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_randomsign(mut seed: *mut UWORD32) -> FLOAT32 {
    let mut sign: FLOAT32 = 0.0f32;
    *seed = (*seed as UWORD64)
        .wrapping_mul(69069 as core::ffi::c_int as UWORD64)
        .wrapping_add(5 as UWORD64) as UWORD32;
    if *seed & 0x10000 as UWORD32 > 0 as UWORD32 {
        sign = -1.0f32 as FLOAT32;
    } else {
        sign = 1.0f32 as FLOAT32;
    }
    return sign;
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
