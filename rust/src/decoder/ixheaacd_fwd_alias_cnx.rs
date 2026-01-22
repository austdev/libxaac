extern "C" {
    pub type ia_sbr_dec_inst_struct;
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
    static ixheaacd_sine_win_128: [WORD32; 128];
    static ixheaacd_sine_win_192: [WORD32; 192];
    static ixheaacd_sine_win_96: [WORD32; 96];
    static ixheaacd_sine_win_256: [WORD32; 256];
    fn ixheaacd_synthesis_tool_float1(
        a: *mut FLOAT32,
        x: *mut FLOAT32,
        l: WORD32,
    ) -> VOID;
    fn ixheaacd_lpc_coeff_wt_apply(a: *mut FLOAT32, ap: *mut FLOAT32) -> VOID;
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
pub const MAX_32: WORD32 = 0x7fffffff as core::ffi::c_long as WORD32;
pub const MIN_32: WORD32 = 0x80000000 as core::ffi::c_long as WORD32;
#[inline]
unsafe extern "C" fn ixheaacd_mult32_m(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 31 as core::ffi::c_int) as WORD32;
    return result;
}
unsafe extern "C" fn ixheaacd_weighted_synthesis_filter(
    mut a: *mut WORD32,
    mut ap: *mut WORD32,
) -> VOID {
    let mut f: WORD32 = 0;
    let mut i: WORD32 = 0;
    *ap.offset(0 as core::ffi::c_int as isize) = *a
        .offset(0 as core::ffi::c_int as isize);
    f = IGAMMA1 as WORD32;
    i = 1 as core::ffi::c_int as WORD32;
    while i <= ORDER {
        *ap.offset(i as isize) = ixheaacd_mult32_m(f, *a.offset(i as isize));
        f = ixheaacd_mult32_m(f, IGAMMA1);
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_synthesis_tool(
    mut a: *mut WORD32,
    mut x: *mut WORD32,
    mut l: WORD32,
    mut qshift: WORD32,
    mut preshift: *mut WORD32,
) -> VOID {
    let mut s: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < l {
        s = *x.offset(i as isize);
        j = 1 as core::ffi::c_int as WORD32;
        while j <= ORDER {
            s = ixheaac_sub32_sat(
                s,
                ixheaac_mul32_sh(
                    *a.offset(j as isize),
                    *x.offset((i - j) as isize),
                    qshift as WORD8,
                ),
            );
            s = ixheaac_sub32_sat(
                s,
                ixheaac_mul32_sh(
                    *a.offset((j as core::ffi::c_int + 1 as core::ffi::c_int) as isize),
                    *x
                        .offset(
                            (i as core::ffi::c_int
                                - (j as core::ffi::c_int + 1 as core::ffi::c_int)) as isize,
                        ),
                    qshift as WORD8,
                ),
            );
            s = ixheaac_sub32_sat(
                s,
                ixheaac_mul32_sh(
                    *a.offset((j as core::ffi::c_int + 2 as core::ffi::c_int) as isize),
                    *x
                        .offset(
                            (i as core::ffi::c_int
                                - (j as core::ffi::c_int + 2 as core::ffi::c_int)) as isize,
                        ),
                    qshift as WORD8,
                ),
            );
            s = ixheaac_sub32_sat(
                s,
                ixheaac_mul32_sh(
                    *a.offset((j as core::ffi::c_int + 3 as core::ffi::c_int) as isize),
                    *x
                        .offset(
                            (i as core::ffi::c_int
                                - (j as core::ffi::c_int + 3 as core::ffi::c_int)) as isize,
                        ),
                    qshift as WORD8,
                ),
            );
            j += 4 as core::ffi::c_int;
        }
        *x.offset(i as isize) = s;
        i += 1;
    }
    *preshift += 1;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_fwd_alias_cancel_tool(
    mut usac_data: *mut ia_usac_data_struct,
    mut pstr_td_frame_data: *mut ia_td_frame_data_struct,
    mut fac_length: WORD32,
    mut lp_filt_coeff: *mut FLOAT32,
    mut gain: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut lp_filt_coeff_a: [FLOAT32; 17] = [0.; 17];
    let mut qshift: WORD32 = 0 as WORD32;
    let mut x_in: *mut WORD32 = ((*pstr_td_frame_data).fac_data).as_mut_ptr();
    let mut ptr_scratch: *mut WORD32 = &mut *((*usac_data).scratch_buffer)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
    let mut fac_signal: *mut WORD32 = &mut *((*usac_data).x_ac_dec)
        .as_mut_ptr()
        .offset(16 as core::ffi::c_int as isize) as *mut WORD32;
    let mut fac_signal_flt: [FLOAT32; 144] = [0.; 144];
    let mut ptr_fac_signal_flt: *mut FLOAT32 = &mut *fac_signal_flt
        .as_mut_ptr()
        .offset(16 as core::ffi::c_int as isize) as *mut FLOAT32;
    let mut ptr_overlap_buf: *mut WORD32 = &mut *(*((*usac_data).overlap_data_ptr)
        .as_mut_ptr()
        .offset((*usac_data).present_chan as isize))
        .as_mut_ptr()
        .offset(((*usac_data).ccfl / 2 as WORD32 - fac_length) as isize) as *mut WORD32;
    memset(
        fac_signal.offset(-(16 as core::ffi::c_int as isize)) as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (ORDER as size_t).wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
    );
    ixheaacd_acelp_mdct(x_in, fac_signal, &mut qshift, fac_length, ptr_scratch);
    ixheaacd_lpc_coeff_wt_apply(lp_filt_coeff, lp_filt_coeff_a.as_mut_ptr());
    i = 0 as core::ffi::c_int as WORD32;
    while i < fac_length {
        *ptr_fac_signal_flt.offset(i as isize) = *fac_signal.offset(i as isize)
            as FLOAT32 / ((1 as core::ffi::c_int) << 16 as WORD32 - qshift) as FLOAT32;
        i += 1;
    }
    memset(
        ptr_fac_signal_flt.offset(-(16 as core::ffi::c_int as isize))
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (16 as size_t).wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
    );
    ixheaacd_synthesis_tool_float1(
        lp_filt_coeff_a.as_mut_ptr(),
        ptr_fac_signal_flt as *mut FLOAT32,
        fac_length,
    );
    i = 0 as core::ffi::c_int as WORD32;
    while i < fac_length {
        *fac_signal.offset(i as isize) = (*ptr_fac_signal_flt.offset(i as isize)
            * ((1 as core::ffi::c_int) << 16 as WORD32 - qshift) as FLOAT32) as WORD32;
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < fac_length {
        *ptr_overlap_buf.offset(i as isize) = ixheaac_add32_sat(
            *ptr_overlap_buf.offset(i as isize),
            ixheaac_mul32_sh(
                *fac_signal.offset(i as isize),
                gain,
                (16 as WORD32 - qshift) as WORD8,
            ),
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_fr_alias_cnx_fix(
    mut x_in: *mut WORD32,
    mut len: WORD32,
    mut fac_length: WORD32,
    mut lp_filt_coeff: *mut WORD32,
    mut izir: *mut WORD32,
    mut fac_data_out: *mut WORD32,
    mut qshift1: *mut WORD8,
    mut qshift2: WORD8,
    mut qshift3: WORD8,
    mut preshift: *mut WORD32,
    mut ptr_scratch: *mut WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut sine_window: *const WORD32 = 0 as *const WORD32;
    let mut fac_window: [WORD32; 256] = [0; 256];
    let mut lp_filt_coeff_a: [WORD32; 17] = [0; 17];
    if fac_length == 48 as core::ffi::c_int {
        sine_window = ixheaacd_sine_win_96.as_ptr();
    } else if fac_length == 64 as core::ffi::c_int {
        sine_window = ixheaacd_sine_win_128.as_ptr();
    } else if fac_length == 96 as core::ffi::c_int {
        sine_window = ixheaacd_sine_win_192.as_ptr();
    } else {
        sine_window = ixheaacd_sine_win_256.as_ptr();
    }
    if !lp_filt_coeff.is_null() && !fac_data_out.is_null() {
        memset(
            fac_data_out.offset(-(16 as core::ffi::c_int as isize))
                as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (ORDER as size_t).wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
        );
        ixheaacd_acelp_mdct(x_in, fac_data_out, preshift, fac_length, ptr_scratch);
        ixheaacd_weighted_synthesis_filter(lp_filt_coeff, lp_filt_coeff_a.as_mut_ptr());
        memset(
            fac_data_out.offset(fac_length as isize) as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (fac_length as size_t)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
        );
        ixheaacd_synthesis_tool(
            lp_filt_coeff_a.as_mut_ptr(),
            fac_data_out as *mut WORD32,
            2 as WORD32 * fac_length,
            qshift2 as WORD32,
            preshift,
        );
        if !izir.is_null() {
            i = 0 as core::ffi::c_int as WORD32;
            while i < fac_length {
                fac_window[i as usize] = ixheaacd_mult32_m(
                    *sine_window.offset(i as isize),
                    *sine_window
                        .offset((2 as WORD32 * fac_length - 1 as WORD32 - i) as isize),
                );
                fac_window[(fac_length + i) as usize] = 2147483647 as WORD32
                    - ixheaacd_mult32_m(
                        *sine_window.offset((fac_length + i) as isize),
                        *sine_window.offset((fac_length + i) as isize),
                    );
                i += 1;
            }
            i = 0 as core::ffi::c_int as WORD32;
            while i < fac_length {
                let mut temp1: WORD32 = 0;
                let mut temp2: WORD32 = 0;
                temp1 = ixheaac_mul32_sh(
                    *izir.offset((1 as WORD32 + len / 2 as WORD32 + i) as isize),
                    fac_window[(fac_length + i) as usize],
                    (qshift3 as core::ffi::c_int - *qshift1 as core::ffi::c_int
                        + 31 as core::ffi::c_int
                        + *preshift as WORD8 as core::ffi::c_int) as core::ffi::c_char
                        as WORD8,
                );
                temp2 = ixheaac_mul32_sh(
                    *izir
                        .offset(
                            (1 as WORD32 + len / 2 as WORD32 - 1 as WORD32 - i) as isize,
                        ),
                    fac_window[(fac_length - 1 as WORD32 - i) as usize],
                    (qshift3 as core::ffi::c_int - *qshift1 as core::ffi::c_int
                        + 31 as core::ffi::c_int
                        + *preshift as WORD8 as core::ffi::c_int) as core::ffi::c_char
                        as WORD8,
                );
                *fac_data_out.offset(i as isize) = ixheaac_add32_sat3(
                    *fac_data_out.offset(i as isize) / 2 as WORD32,
                    temp1,
                    temp2,
                );
                *fac_data_out.offset((fac_length + i) as isize) = (*fac_data_out
                    .offset((fac_length + i) as isize) / 2 as core::ffi::c_int)
                    as WORD32;
                i += 1;
            }
        }
    }
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
unsafe extern "C" fn ixheaac_add32_sat3(
    mut a: WORD32,
    mut b: WORD32,
    mut c: WORD32,
) -> WORD32 {
    let mut sum: WORD64 = 0;
    sum = a as WORD64 + b as WORD64;
    sum = sum + c as WORD64;
    if sum > MAX_32 as WORD64 {
        sum = MAX_32 as WORD64;
    }
    if sum < MIN_32 as WORD64 {
        sum = MIN_32 as WORD64;
    }
    return sum as WORD32;
}
#[inline]
unsafe extern "C" fn ixheaac_sub32_sat(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut diff: WORD64 = 0;
    diff = a as WORD64 - b as WORD64;
    if diff >= MAX_32 as WORD64 {
        return MAX_32;
    }
    if diff <= MIN_32 as WORD64 {
        return MIN_32;
    }
    return diff as WORD32;
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const ORDER: core::ffi::c_int = 16 as core::ffi::c_int;
pub const IGAMMA1: core::ffi::c_int = 1975684956 as core::ffi::c_int;
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
