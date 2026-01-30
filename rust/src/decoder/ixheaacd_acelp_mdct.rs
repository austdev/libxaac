extern "C" {
    pub type ia_sbr_dec_inst_struct;
    fn ixheaacd_complex_fft(
        data_r: *mut WORD32,
        data_i: *mut WORD32,
        len: WORD32,
        fft_mode: WORD32,
        preshift: *mut WORD32,
    ) -> VOID;
    static ixheaacd_pre_post_twid_cos_sin_512: [[WORD32; 512]; 4];
    static ixheaacd_pre_post_twid_cos_sin_384: [[WORD32; 384]; 4];
    static ixheaacd_pre_post_twid_cos_sin_256: [[WORD32; 256]; 4];
    static ixheaacd_pre_post_twid_cos_sin_192: [[WORD32; 192]; 4];
    static ixheaacd_pre_post_twid_cos_sin_128: [[WORD32; 128]; 4];
    static ixheaacd_pre_post_twid_cos_sin_96: [[WORD32; 96]; 4];
    static ixheaacd_pre_post_twid_cos_sin_64: [[WORD32; 64]; 4];
    static ixheaacd_pre_post_twid_cos_sin_48: [[WORD32; 48]; 4];
    static ixheaacd_pre_post_twid_cos_sin_32: [[WORD32; 32]; 4];
    static ixheaacd_pre_post_twid_cos_sin_24: [[WORD32; 24]; 4];
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
#[inline]
unsafe extern "C" fn ixheaacd_mul_sub64_sat_32(
    mut a: WORD32,
    mut b: WORD32,
    mut c: WORD32,
    mut d: WORD32,
) -> WORD32 {
    let mut diff: WORD64 = 0;
    let mut temp_result1: WORD64 = 0;
    let mut temp_result2: WORD64 = 0;
    temp_result1 = a as WORD64 * c as WORD64;
    temp_result2 = b as WORD64 * d as WORD64;
    diff = temp_result1 - temp_result2 >> 32 as core::ffi::c_int;
    if diff >= 2147483647 as WORD64 {
        diff = 2147483647 as WORD64;
    } else if diff
        <= (-(2147483647 as core::ffi::c_int) - 1 as core::ffi::c_int) as WORD64
    {
        diff = (-(2147483647 as core::ffi::c_int) - 1 as core::ffi::c_int) as WORD64;
    }
    return diff as WORD32;
}
#[inline]
unsafe extern "C" fn ixheaacd_mul_add64_sat_32(
    mut a: WORD32,
    mut b: WORD32,
    mut c: WORD32,
    mut d: WORD32,
) -> WORD32 {
    let mut sum: WORD64 = 0;
    let mut temp_result1: WORD64 = 0;
    let mut temp_result2: WORD64 = 0;
    temp_result1 = a as WORD64 * c as WORD64;
    temp_result2 = b as WORD64 * d as WORD64;
    sum = temp_result1 + temp_result2 >> 32 as core::ffi::c_int;
    if sum >= 2147483647 as WORD64 {
        sum = 2147483647 as WORD64;
    } else if sum
        <= (-(2147483647 as core::ffi::c_int) - 1 as core::ffi::c_int) as WORD64
    {
        sum = (-(2147483647 as core::ffi::c_int) - 1 as core::ffi::c_int) as WORD64;
    }
    return sum as WORD32;
}
unsafe extern "C" fn ixheaacd_pre_twid(
    mut in_0: *mut WORD32,
    mut r_ptr: *mut WORD32,
    mut i_ptr: *mut WORD32,
    mut nlength: WORD32,
    mut ptr_pre_cos_sin: *const WORD32,
) {
    let mut i: WORD32 = 0;
    let mut cos_ptr: *const WORD32 = &*ptr_pre_cos_sin
        .offset(0 as core::ffi::c_int as isize) as *const WORD32;
    let mut sin_ptr: *const WORD32 = &*ptr_pre_cos_sin.offset(nlength as isize)
        as *const WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < nlength {
        let fresh0 = r_ptr;
        r_ptr = r_ptr.offset(1);
        *fresh0 = ixheaacd_mul_sub64_sat_32(
            *in_0.offset(i as isize),
            *in_0.offset((nlength + i) as isize),
            *cos_ptr.offset(i as isize),
            *sin_ptr.offset(i as isize),
        );
        let fresh1 = i_ptr;
        i_ptr = i_ptr.offset(1);
        *fresh1 = ixheaacd_mul_add64_sat_32(
            *in_0.offset(i as isize),
            *in_0.offset((nlength + i) as isize),
            *sin_ptr.offset(i as isize),
            *cos_ptr.offset(i as isize),
        );
        let fresh2 = r_ptr;
        r_ptr = r_ptr.offset(1);
        *fresh2 = ixheaacd_mul_sub64_sat_32(
            *in_0.offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize),
            *in_0
                .offset(
                    (nlength as core::ffi::c_int + i as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ),
            *cos_ptr.offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize),
            *sin_ptr.offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize),
        );
        let fresh3 = i_ptr;
        i_ptr = i_ptr.offset(1);
        *fresh3 = ixheaacd_mul_add64_sat_32(
            *in_0.offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize),
            *in_0
                .offset(
                    (nlength as core::ffi::c_int + i as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ),
            *sin_ptr.offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize),
            *cos_ptr.offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize),
        );
        let fresh4 = r_ptr;
        r_ptr = r_ptr.offset(1);
        *fresh4 = ixheaacd_mul_sub64_sat_32(
            *in_0.offset((i as core::ffi::c_int + 2 as core::ffi::c_int) as isize),
            *in_0
                .offset(
                    (nlength as core::ffi::c_int + i as core::ffi::c_int
                        + 2 as core::ffi::c_int) as isize,
                ),
            *cos_ptr.offset((i as core::ffi::c_int + 2 as core::ffi::c_int) as isize),
            *sin_ptr.offset((i as core::ffi::c_int + 2 as core::ffi::c_int) as isize),
        );
        let fresh5 = i_ptr;
        i_ptr = i_ptr.offset(1);
        *fresh5 = ixheaacd_mul_add64_sat_32(
            *in_0.offset((i as core::ffi::c_int + 2 as core::ffi::c_int) as isize),
            *in_0
                .offset(
                    (nlength as core::ffi::c_int + i as core::ffi::c_int
                        + 2 as core::ffi::c_int) as isize,
                ),
            *sin_ptr.offset((i as core::ffi::c_int + 2 as core::ffi::c_int) as isize),
            *cos_ptr.offset((i as core::ffi::c_int + 2 as core::ffi::c_int) as isize),
        );
        let fresh6 = r_ptr;
        r_ptr = r_ptr.offset(1);
        *fresh6 = ixheaacd_mul_sub64_sat_32(
            *in_0.offset((i as core::ffi::c_int + 3 as core::ffi::c_int) as isize),
            *in_0
                .offset(
                    (nlength as core::ffi::c_int + i as core::ffi::c_int
                        + 3 as core::ffi::c_int) as isize,
                ),
            *cos_ptr.offset((i as core::ffi::c_int + 3 as core::ffi::c_int) as isize),
            *sin_ptr.offset((i as core::ffi::c_int + 3 as core::ffi::c_int) as isize),
        );
        let fresh7 = i_ptr;
        i_ptr = i_ptr.offset(1);
        *fresh7 = ixheaacd_mul_add64_sat_32(
            *in_0.offset((i as core::ffi::c_int + 3 as core::ffi::c_int) as isize),
            *in_0
                .offset(
                    (nlength as core::ffi::c_int + i as core::ffi::c_int
                        + 3 as core::ffi::c_int) as isize,
                ),
            *sin_ptr.offset((i as core::ffi::c_int + 3 as core::ffi::c_int) as isize),
            *cos_ptr.offset((i as core::ffi::c_int + 3 as core::ffi::c_int) as isize),
        );
        i += 4 as core::ffi::c_int;
    }
}
unsafe extern "C" fn ixheaacd_post_twid(
    mut data_re: *mut WORD32,
    mut data_im: *mut WORD32,
    mut out: *mut WORD32,
    mut nlength: WORD32,
    mut ptr_post_cos_sin: *const WORD32,
) {
    let mut i: WORD32 = 0;
    let mut cos_ptr: *const WORD32 = &*ptr_post_cos_sin
        .offset((nlength as core::ffi::c_int * 2 as core::ffi::c_int) as isize)
        as *const WORD32;
    let mut sin_ptr: *const WORD32 = &*ptr_post_cos_sin
        .offset((nlength as core::ffi::c_int * 3 as core::ffi::c_int) as isize)
        as *const WORD32;
    let mut out_ptr: *mut WORD32 = &mut *out
        .offset(
            (2 as core::ffi::c_int * nlength as core::ffi::c_int - 1 as core::ffi::c_int)
                as isize,
        ) as *mut WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < nlength {
        *out.offset(0 as core::ffi::c_int as isize) = ixheaacd_mul_sub64_sat_32(
            *data_re.offset(i as isize),
            *data_im.offset(i as isize),
            *cos_ptr.offset(i as isize),
            *sin_ptr.offset(i as isize),
        );
        *out_ptr.offset(0 as core::ffi::c_int as isize) = -ixheaacd_mul_add64_sat_32(
            *data_re.offset(i as isize),
            *data_im.offset(i as isize),
            *sin_ptr.offset(i as isize),
            *cos_ptr.offset(i as isize),
        );
        *out.offset(2 as core::ffi::c_int as isize) = ixheaacd_mul_sub64_sat_32(
            *data_re.offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize),
            *data_im.offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize),
            *cos_ptr.offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize),
            *sin_ptr.offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize),
        );
        *out_ptr.offset(-(2 as core::ffi::c_int) as isize) = -ixheaacd_mul_add64_sat_32(
            *data_re.offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize),
            *data_im.offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize),
            *sin_ptr.offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize),
            *cos_ptr.offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize),
        );
        *out.offset(4 as core::ffi::c_int as isize) = ixheaacd_mul_sub64_sat_32(
            *data_re.offset((i as core::ffi::c_int + 2 as core::ffi::c_int) as isize),
            *data_im.offset((i as core::ffi::c_int + 2 as core::ffi::c_int) as isize),
            *cos_ptr.offset((i as core::ffi::c_int + 2 as core::ffi::c_int) as isize),
            *sin_ptr.offset((i as core::ffi::c_int + 2 as core::ffi::c_int) as isize),
        );
        *out_ptr.offset(-(4 as core::ffi::c_int) as isize) = -ixheaacd_mul_add64_sat_32(
            *data_re.offset((i as core::ffi::c_int + 2 as core::ffi::c_int) as isize),
            *data_im.offset((i as core::ffi::c_int + 2 as core::ffi::c_int) as isize),
            *sin_ptr.offset((i as core::ffi::c_int + 2 as core::ffi::c_int) as isize),
            *cos_ptr.offset((i as core::ffi::c_int + 2 as core::ffi::c_int) as isize),
        );
        *out.offset(6 as core::ffi::c_int as isize) = ixheaacd_mul_sub64_sat_32(
            *data_re.offset((i as core::ffi::c_int + 3 as core::ffi::c_int) as isize),
            *data_im.offset((i as core::ffi::c_int + 3 as core::ffi::c_int) as isize),
            *cos_ptr.offset((i as core::ffi::c_int + 3 as core::ffi::c_int) as isize),
            *sin_ptr.offset((i as core::ffi::c_int + 3 as core::ffi::c_int) as isize),
        );
        *out_ptr.offset(-(6 as core::ffi::c_int) as isize) = -ixheaacd_mul_add64_sat_32(
            *data_re.offset((i as core::ffi::c_int + 3 as core::ffi::c_int) as isize),
            *data_im.offset((i as core::ffi::c_int + 3 as core::ffi::c_int) as isize),
            *sin_ptr.offset((i as core::ffi::c_int + 3 as core::ffi::c_int) as isize),
            *cos_ptr.offset((i as core::ffi::c_int + 3 as core::ffi::c_int) as isize),
        );
        out = out.offset(8 as core::ffi::c_int as isize);
        out_ptr = out_ptr.offset(-(8 as core::ffi::c_int as isize));
        i += 4 as core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_acelp_mdct(
    mut ptr_in: *mut WORD32,
    mut ptr_out: *mut WORD32,
    mut preshift: *mut WORD32,
    mut length: WORD32,
    mut ptr_scratch: *mut WORD32,
) -> VOID {
    let mut ptr_data_r: *mut WORD32 = ptr_scratch;
    let mut ptr_data_i: *mut WORD32 = ptr_scratch
        .offset(512 as core::ffi::c_int as isize);
    let mut ptr_pre_post_twid: *const WORD32 = 0 as *const WORD32;
    match length {
        1024 => {
            ptr_pre_post_twid = &*(*ixheaacd_pre_post_twid_cos_sin_512
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const WORD32;
        }
        768 => {
            ptr_pre_post_twid = &*(*ixheaacd_pre_post_twid_cos_sin_384
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const WORD32;
        }
        512 => {
            ptr_pre_post_twid = &*(*ixheaacd_pre_post_twid_cos_sin_256
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const WORD32;
        }
        384 => {
            ptr_pre_post_twid = &*(*ixheaacd_pre_post_twid_cos_sin_192
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const WORD32;
        }
        256 => {
            ptr_pre_post_twid = &*(*ixheaacd_pre_post_twid_cos_sin_128
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const WORD32;
        }
        192 => {
            ptr_pre_post_twid = &*(*ixheaacd_pre_post_twid_cos_sin_96
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const WORD32;
        }
        128 => {
            ptr_pre_post_twid = &*(*ixheaacd_pre_post_twid_cos_sin_64
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const WORD32;
        }
        96 => {
            ptr_pre_post_twid = &*(*ixheaacd_pre_post_twid_cos_sin_48
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const WORD32;
        }
        64 => {
            ptr_pre_post_twid = &*(*ixheaacd_pre_post_twid_cos_sin_32
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const WORD32;
        }
        48 => {
            ptr_pre_post_twid = &*(*ixheaacd_pre_post_twid_cos_sin_24
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const WORD32;
        }
        _ => {
            ptr_pre_post_twid = &*(*ixheaacd_pre_post_twid_cos_sin_24
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const WORD32;
        }
    }
    ixheaacd_pre_twid(
        ptr_in,
        ptr_data_r,
        ptr_data_i,
        length / 2 as WORD32,
        ptr_pre_post_twid,
    );
    ixheaacd_complex_fft(
        ptr_data_r,
        ptr_data_i,
        length / 2 as WORD32,
        -(1 as WORD32),
        preshift,
    );
    *preshift += 1 as core::ffi::c_int;
    ixheaacd_post_twid(
        ptr_data_r,
        ptr_data_i,
        ptr_out,
        length / 2 as WORD32,
        ptr_pre_post_twid,
    );
    *preshift += 1 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_acelp_mdct_main(
    mut usac_data: *mut ia_usac_data_struct,
    mut in_0: *mut WORD32,
    mut out: *mut WORD32,
    mut l: WORD32,
    mut m: WORD32,
    mut preshift: *mut WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut ptr_scratch: *mut WORD32 = &mut *((*usac_data).scratch_buffer)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
    let mut output_buffer: *mut WORD32 = &mut *((*usac_data).x_ac_dec)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
    ixheaacd_acelp_mdct(in_0, output_buffer, preshift, l + m, ptr_scratch);
    i = 0 as core::ffi::c_int as WORD32;
    while i < m as core::ffi::c_int / 2 as core::ffi::c_int {
        *out.offset((l + m / 2 as WORD32 - 1 as WORD32 - i) as isize) = -*output_buffer
            .offset((m / 2 as WORD32 + l / 2 as WORD32 + i) as isize);
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < l as core::ffi::c_int / 2 as core::ffi::c_int {
        *out.offset(i as isize) = *output_buffer
            .offset((m + l / 2 as WORD32 + i) as isize);
        *out.offset((l - 1 as WORD32 - i) as isize) = -*output_buffer
            .offset((m + l / 2 as WORD32 + i) as isize);
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < m as core::ffi::c_int / 2 as core::ffi::c_int {
        *out.offset((l + m / 2 as WORD32 + i) as isize) = -*output_buffer
            .offset((m / 2 as WORD32 + l / 2 as WORD32 - 1 as WORD32 - i) as isize);
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < l as core::ffi::c_int / 2 as core::ffi::c_int {
        *out.offset((l + m + i) as isize) = -*output_buffer
            .offset((l / 2 as WORD32 - 1 as WORD32 - i) as isize);
        *out.offset((2 as WORD32 * l + m - 1 as WORD32 - i) as isize) = -*output_buffer
            .offset((l / 2 as WORD32 - 1 as WORD32 - i) as isize);
        i += 1;
    }
}
