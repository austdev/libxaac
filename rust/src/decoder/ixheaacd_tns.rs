extern "C" {
    pub type ia_sbr_dec_inst_struct;
    static mut ixheaacd_calc_max_spectral_line: Option<
        unsafe extern "C" fn(*mut WORD32, WORD32) -> WORD32,
    >;
}
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
pub type IA_ERRORCODE = WORD32;
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
pub const TNS_MAX_ORDER: core::ffi::c_int = 31 as core::ffi::c_int;
pub const MAX_64: WORD64 = 0x7fffffffffffffff as core::ffi::c_long as WORD64;
pub const MIN_64: WORD64 = 0x8000000000000000 as core::ffi::c_ulong as WORD64;
pub const MAX_32: WORD32 = 0x7fffffff as core::ffi::c_long as WORD32;
pub const MIN_32: WORD32 = 0x80000000 as core::ffi::c_long as WORD32;
unsafe extern "C" fn ixheaacd_tns_dec_coef_usac(
    mut usac_data: *mut ia_usac_data_struct,
    mut filter: *mut ia_tns_filter_struct,
    mut coef_res: WORD32,
    mut par_coeff: *mut WORD32,
) -> VOID {
    let mut resolution: WORD32 = 0;
    let mut ptr_par_coeff: *mut WORD32 = par_coeff;
    let mut tns_coeff_ptr: *const WORD32 = 0 as *const WORD32;
    let mut ixheaacd_drc_offset: WORD32 = 4 as WORD32;
    let mut ptr_coeff: *mut WORD16 = ((*filter).coef).as_mut_ptr();
    let mut order: WORD32 = 0;
    resolution = (coef_res as core::ffi::c_int - 3 as core::ffi::c_int) as WORD32;
    tns_coeff_ptr = (*usac_data).tns_coeff3_32;
    if resolution != 0 {
        tns_coeff_ptr = (*usac_data).tns_coeff4_32;
        ixheaacd_drc_offset = ixheaacd_drc_offset << 1 as core::ffi::c_int;
    }
    order = (*filter).order;
    loop {
        let fresh0 = ptr_coeff;
        ptr_coeff = ptr_coeff.offset(1);
        let mut temp: WORD16 = *fresh0;
        let fresh1 = ptr_par_coeff;
        ptr_par_coeff = ptr_par_coeff.offset(1);
        *fresh1 = *tns_coeff_ptr.offset((temp as WORD32 + ixheaacd_drc_offset) as isize);
        order -= 1;
        if !(order != 0 as core::ffi::c_int) {
            break;
        }
    };
}
unsafe extern "C" fn ixheaacd_tns_parcor_lpc_convert_usac(
    mut parcor: *mut WORD32,
    mut lpc_coeff: *mut WORD32,
    mut scale: *mut WORD32,
    mut order: WORD,
) -> VOID {
    let mut i: WORD = 0;
    let mut j: WORD = 0;
    let mut status: WORD = 0;
    let mut accu: WORD32 = 0;
    let mut temp_buf1: [WORD32; 32] = [0; 32];
    let mut temp_buf2: [WORD32; 32] = [0; 32];
    let mut accu1: WORD32 = 0;
    let mut accu2: WORD32 = 0;
    status = 1 as core::ffi::c_int as WORD;
    *scale = 1 as core::ffi::c_int as WORD32;
    while status != 0 {
        status = 0 as core::ffi::c_int as WORD;
        i = TNS_MAX_ORDER as WORD;
        while i >= 0 as core::ffi::c_int {
            temp_buf1[i as usize] = 0 as core::ffi::c_int as WORD32;
            temp_buf2[i as usize] = 0 as core::ffi::c_int as WORD32;
            i -= 1;
        }
        accu1 = (0x40000000 as core::ffi::c_int >> *scale - 1 as core::ffi::c_int)
            as WORD32;
        i = 0 as core::ffi::c_int as WORD;
        while i <= order {
            accu = accu1;
            j = 0 as core::ffi::c_int as WORD;
            while j < order {
                temp_buf2[j as usize] = accu1;
                accu1 = ixheaac_add32_sat(
                    accu1,
                    ixheaac_mult32_shl_sat(
                        *parcor.offset(j as isize),
                        temp_buf1[j as usize],
                    ),
                );
                if ixheaac_abs32_sat(accu1) == 0x7fffffff as core::ffi::c_int {
                    status = 1 as core::ffi::c_int as WORD;
                }
                j += 1;
            }
            j = (order as core::ffi::c_int - 1 as core::ffi::c_int) as WORD;
            while j >= 0 as core::ffi::c_int {
                accu2 = temp_buf1[j as usize];
                accu2 = ixheaac_add32_sat(
                    accu2,
                    ixheaac_mult32_shl_sat(
                        *parcor.offset(j as isize),
                        temp_buf2[j as usize],
                    ),
                );
                temp_buf1[(j as core::ffi::c_int + 1 as core::ffi::c_int) as usize] = accu2;
                if ixheaac_abs32_sat(accu2) == 0x7fffffff as core::ffi::c_int {
                    status = 1 as core::ffi::c_int as WORD;
                }
                j -= 1;
            }
            temp_buf1[0 as core::ffi::c_int as usize] = accu;
            *lpc_coeff.offset(i as isize) = accu1;
            accu1 = 0 as core::ffi::c_int as WORD32;
            i += 1;
        }
        accu1 = (status as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        if accu1 == 0 as core::ffi::c_int {
            *scale = (*scale + 1 as core::ffi::c_int) as WORD32;
        }
    }
}
unsafe extern "C" fn ixheaacd_tns_ar_filter_usac(
    mut spectrum: *mut WORD32,
    mut size: WORD32,
    mut inc: WORD32,
    mut lpc_coeff: *mut WORD32,
    mut order: WORD32,
    mut shift_value: WORD32,
    mut ptr_filter_state: *mut WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut y: WORD32 = 0;
    let mut acc: WORD64 = 0;
    if order as core::ffi::c_int & 3 as core::ffi::c_int != 0 as core::ffi::c_int {
        i = (order as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
        while i
            < (order as core::ffi::c_uint & 0xfffffffc as core::ffi::c_uint)
                as core::ffi::c_int + 4 as core::ffi::c_int
        {
            *lpc_coeff.offset(i as isize) = 0 as core::ffi::c_int as WORD32;
            i += 1;
        }
        *lpc_coeff.offset(i as isize) = 0 as core::ffi::c_int as WORD32;
        order = (order as core::ffi::c_uint & 0xfffffffc as core::ffi::c_uint)
            .wrapping_add(4 as core::ffi::c_uint) as WORD32;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < order {
        y = *spectrum;
        acc = 0 as WORD64;
        j = i;
        while j > 0 as core::ffi::c_int {
            acc = ixheaac_add64_sat(
                acc,
                ixheaac_mult64(
                    *ptr_filter_state
                        .offset(
                            (j as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                        ),
                    *lpc_coeff.offset(j as isize),
                ),
            );
            *ptr_filter_state.offset(j as isize) = *ptr_filter_state
                .offset((j as core::ffi::c_int - 1 as core::ffi::c_int) as isize);
            j -= 1;
        }
        y = ixheaac_sub32_sat(y, (acc >> 31 as core::ffi::c_int) as WORD32);
        *ptr_filter_state.offset(0 as core::ffi::c_int as isize) = ixheaac_shl32(
            y,
            shift_value as WORD,
        );
        *spectrum = y;
        spectrum = spectrum.offset(inc as isize);
        i += 1;
    }
    i = order;
    while i < size {
        y = *spectrum;
        acc = 0 as WORD64;
        j = order;
        while j > 0 as core::ffi::c_int {
            acc = ixheaac_add64_sat(
                acc,
                ixheaac_mult64(
                    *ptr_filter_state
                        .offset(
                            (j as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                        ),
                    *lpc_coeff.offset(j as isize),
                ),
            );
            *ptr_filter_state.offset(j as isize) = *ptr_filter_state
                .offset((j as core::ffi::c_int - 1 as core::ffi::c_int) as isize);
            j -= 1;
        }
        y = ixheaac_sub32_sat(y, (acc >> 31 as core::ffi::c_int) as WORD32);
        *ptr_filter_state.offset(0 as core::ffi::c_int as isize) = ixheaac_shl32(
            y,
            shift_value as WORD,
        );
        *spectrum = y;
        spectrum = spectrum.offset(inc as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_tns_apply(
    mut usac_data: *mut ia_usac_data_struct,
    mut spec: *mut WORD32,
    mut nbands: WORD32,
    mut pstr_sfb_info: *mut ia_sfb_info_struct,
    mut pstr_tns: *mut ia_tns_frame_info_struct,
) -> IA_ERRORCODE {
    let mut f: WORD32 = 0;
    let mut start: WORD32 = 0;
    let mut stop: WORD32 = 0;
    let mut size: WORD32 = 0;
    let mut inc: WORD32 = 0;
    let mut n_filt: WORD32 = 0;
    let mut coef_res: WORD32 = 0;
    let mut order: WORD32 = 0;
    let mut direction: WORD32 = 0;
    let mut ptr_spec: *mut WORD32 = 0 as *mut WORD32;
    let mut scale_spec: WORD32 = 0;
    let mut scale_lpc: WORD32 = 0;
    let mut guard_band: WORD32 = 0;
    let mut shift: WORD32 = 0;
    let mut lpc_coeff: [WORD32; 32] = [0; 32];
    let mut par_coeff: [WORD32; 32] = [0; 32];
    let mut filt: *mut ia_tns_filter_struct = 0 as *mut ia_tns_filter_struct;
    let mut sfb_top: *const WORD16 = 0 as *const WORD16;
    let mut nbins: WORD32 = if (*pstr_sfb_info).islong != 0 {
        1024 as WORD32
    } else {
        128 as WORD32
    };
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut idx: WORD32 = 0;
    idx = (if (*pstr_sfb_info).islong != 0 {
        0 as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    }) as WORD32;
    ptr_spec = &mut *((*usac_data).scratch_buffer)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
    j = 0 as core::ffi::c_int as WORD32;
    while j < (*pstr_tns).n_subblocks {
        sfb_top = (*pstr_sfb_info).ptr_sfb_tbl;
        i = 0 as core::ffi::c_int as WORD32;
        while i < nbins {
            *ptr_spec.offset(i as isize) = *spec.offset(i as isize);
            i += 1;
        }
        if (*pstr_tns).str_tns_info[j as usize].n_filt != 0 {
            n_filt = (*pstr_tns).str_tns_info[j as usize].n_filt;
            f = 0 as core::ffi::c_int as WORD32;
            while f < n_filt {
                let mut tmp: WORD32 = 0;
                coef_res = (*pstr_tns).str_tns_info[j as usize].coef_res;
                filt = &mut *((*((*pstr_tns).str_tns_info)
                    .as_mut_ptr()
                    .offset(j as isize))
                    .str_filter)
                    .as_mut_ptr()
                    .offset(f as isize) as *mut ia_tns_filter_struct;
                order = (*filt).order;
                direction = (*filt).direction;
                start = (*filt).start_band;
                stop = (*filt).stop_band;
                if !(order == 0) {
                    ixheaacd_tns_dec_coef_usac(
                        usac_data,
                        filt,
                        coef_res,
                        par_coeff.as_mut_ptr(),
                    );
                    ixheaacd_tns_parcor_lpc_convert_usac(
                        par_coeff.as_mut_ptr(),
                        lpc_coeff.as_mut_ptr(),
                        &mut scale_lpc,
                        (*filt).order as WORD,
                    );
                    tmp = (*(*usac_data)
                        .tns_max_bands_tbl_usac)[(*usac_data).sampling_rate_idx
                        as usize][idx as usize];
                    start = ixheaac_min32(start, tmp);
                    start = ixheaac_min32(start, nbands);
                    if start > (*pstr_sfb_info).sfb_per_sbk {
                        return -(1 as IA_ERRORCODE);
                    }
                    start = (if start > 0 as core::ffi::c_int {
                        *sfb_top
                            .offset(
                                (start as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                            ) as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    }) as WORD32;
                    stop = ixheaac_min32(stop, tmp);
                    stop = ixheaac_min32(stop, nbands);
                    if stop > (*pstr_sfb_info).sfb_per_sbk {
                        return -(1 as IA_ERRORCODE);
                    }
                    stop = (if stop > 0 as core::ffi::c_int {
                        *sfb_top
                            .offset(
                                (stop as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                            ) as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    }) as WORD32;
                    guard_band = (31 as WORD - ixheaac_norm32((*filt).order)) as WORD32;
                    size = stop - start;
                    if !(size <= 0 as core::ffi::c_int) {
                        if direction != 0 {
                            inc = -(1 as core::ffi::c_int) as WORD32;
                            shift = (stop as core::ffi::c_int - 1 as core::ffi::c_int)
                                as WORD32;
                        } else {
                            inc = 1 as core::ffi::c_int as WORD32;
                            shift = start;
                        }
                        let mut ptr_temp: *mut WORD32 = ptr_spec.offset(start as isize);
                        scale_spec = (Some(
                            ixheaacd_calc_max_spectral_line
                                .expect("non-null function pointer"),
                        ))
                            .expect("non-null function pointer")(ptr_temp, size);
                        scale_spec = scale_spec - guard_band - scale_lpc;
                        if scale_spec > 0 as core::ffi::c_int {
                            ixheaacd_tns_ar_filter_usac(
                                &mut *ptr_spec.offset(shift as isize),
                                size,
                                inc,
                                lpc_coeff.as_mut_ptr(),
                                (*filt).order,
                                scale_lpc,
                                ((*usac_data).x_ac_dec).as_mut_ptr(),
                            );
                        } else {
                            let mut ptr_temp_0: *mut WORD32 = ptr_spec
                                .offset(start as isize);
                            scale_spec = -scale_spec;
                            scale_spec = ixheaac_min32(scale_spec, 31 as WORD32);
                            i = size;
                            while i != 0 as core::ffi::c_int {
                                *ptr_temp_0 = *ptr_temp_0 >> scale_spec;
                                ptr_temp_0 = ptr_temp_0.offset(1);
                                i -= 1;
                            }
                            ixheaacd_tns_ar_filter_usac(
                                &mut *ptr_spec.offset(shift as isize),
                                size,
                                inc,
                                lpc_coeff.as_mut_ptr(),
                                (*filt).order,
                                scale_lpc,
                                ((*usac_data).x_ac_dec).as_mut_ptr(),
                            );
                            ptr_temp_0 = ptr_spec.offset(start as isize);
                            i = size;
                            loop {
                                *ptr_temp_0 = *ptr_temp_0 << scale_spec;
                                ptr_temp_0 = ptr_temp_0.offset(1);
                                i -= 1;
                                if !(i != 0 as core::ffi::c_int) {
                                    break;
                                }
                            }
                        }
                        i = start;
                        while i <= stop as core::ffi::c_int - 1 as core::ffi::c_int {
                            *spec.offset(i as isize) = *ptr_spec.offset(i as isize);
                            i += 1;
                        }
                    }
                }
                f += 1;
            }
        }
        spec = spec.offset((*pstr_sfb_info).bins_per_sbk as isize);
        j += 1;
    }
    return 0 as IA_ERRORCODE;
}
#[inline]
unsafe extern "C" fn ixheaac_min32(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut min_val: WORD32 = 0;
    min_val = if a < b { a } else { b };
    return min_val;
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
unsafe extern "C" fn ixheaac_abs32_sat(mut a: WORD32) -> WORD32 {
    let mut abs_val: WORD32 = 0;
    abs_val = a;
    if a == MIN_32 {
        abs_val = MAX_32;
    } else if a < 0 as core::ffi::c_int {
        abs_val = -a;
    }
    return abs_val;
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
unsafe extern "C" fn ixheaac_mult32_shl_sat(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    if a == 0x80000000 as core::ffi::c_uint as WORD32
        && b == 0x80000000 as core::ffi::c_uint as WORD32
    {
        result = 0x7fffffff as core::ffi::c_int as WORD32;
    } else {
        result = ixheaac_mult32_shl(a, b);
    }
    return result;
}
#[inline]
unsafe extern "C" fn ixheaac_mult64(mut a: WORD32, mut b: WORD32) -> WORD64 {
    let mut result: WORD64 = 0;
    result = a as WORD64 * b as WORD64;
    return result;
}
#[inline]
unsafe extern "C" fn ixheaac_add64_sat(mut a: WORD64, mut b: WORD64) -> WORD64 {
    let mut result: WORD64 = 0;
    let mut comp: WORD64 = 0;
    result = if a < 0 as WORD64 { MIN_64 } else { MAX_64 };
    comp = result - a;
    if (a < 0 as WORD64) as core::ffi::c_int == (b > comp) as core::ffi::c_int {
        result = a + b;
    }
    return result;
}
