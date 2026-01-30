extern "C" {
    pub type ia_sbr_dec_inst_struct;
    fn log10(__x: core::ffi::c_double) -> core::ffi::c_double;
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
    fn ixheaacd_float2fix(x: *mut FLOAT32, int_x: *mut WORD32, length: WORD32) -> WORD8;
    fn ixheaacd_fix2float(
        int_xn1: *mut WORD32,
        xn1: *mut FLOAT32,
        length: WORD32,
        shiftp: *mut WORD8,
        preshift: *mut WORD32,
    ) -> VOID;
    fn ixheaacd_acelp_mdct(
        ptr_in: *mut WORD32,
        ptr_out: *mut WORD32,
        preshift: *mut WORD32,
        length: WORD32,
        ptr_scratch: *mut WORD32,
    ) -> VOID;
    fn ixheaacd_mem_cpy(x: *const FLOAT32, y: *mut FLOAT32, n: WORD32) -> VOID;
    fn ixheaacd_vec_cnst_mul(
        a: FLOAT32,
        x: *mut FLOAT32,
        z: *mut FLOAT32,
        n: WORD32,
    ) -> VOID;
    static ixheaacd_interpol_filt: [FLOAT32; 65];
    static ixheaacd_int_leave_gain_table: [FLOAT32; 256];
    fn ixheaacd_synthesis_tool_float(
        a: *mut FLOAT32,
        x: *mut FLOAT32,
        y: *mut FLOAT32,
        l: WORD32,
        mem: *mut FLOAT32,
    ) -> VOID;
    fn ixheaacd_synthesis_tool_float1(
        a: *mut FLOAT32,
        x: *mut FLOAT32,
        l: WORD32,
    ) -> VOID;
    fn ixheaacd_lpc_wt_synthesis_tool(
        a: *mut FLOAT32,
        x: *mut FLOAT32,
        l: WORD32,
    ) -> VOID;
    fn ixheaacd_deemphsis_tool(signal: *mut FLOAT32, len: WORD32, mem: FLOAT32) -> VOID;
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
pub const NUM_FRAMES: core::ffi::c_int = 4 as core::ffi::c_int;
pub const ORDER: core::ffi::c_int = 16 as core::ffi::c_int;
pub const FAC_LENGTH: core::ffi::c_int = 128 as core::ffi::c_int;
pub const FSCALE_DENOM: core::ffi::c_int = 12800 as core::ffi::c_int;
pub const FAC_FSCALE_MAX: core::ffi::c_int = 24000 as core::ffi::c_int;
pub const TMIN: core::ffi::c_int = 34 as core::ffi::c_int;
pub const TFR2: core::ffi::c_int = 162 as core::ffi::c_int - TMIN;
pub const TFR1: core::ffi::c_int = 160 as core::ffi::c_int;
pub const TMAX: core::ffi::c_int = 27 as core::ffi::c_int + 6 as core::ffi::c_int * TMIN;
pub const UP_SAMP: core::ffi::c_int = 4 as core::ffi::c_int;
pub const INTER_LP_FIL_ORDER: core::ffi::c_int = 16 as core::ffi::c_int;
pub const MAX_PITCH: core::ffi::c_int = TMAX
    + 6 as core::ffi::c_int
        * ((FAC_FSCALE_MAX * TMIN + FSCALE_DENOM / 2 as core::ffi::c_int) / FSCALE_DENOM
            - TMIN);
pub const PREEMPH_FILT_FAC: core::ffi::c_float = 0.68f32;
pub const TILT_CODE: core::ffi::c_float = 0.3f32;
pub const LEN_SUBFR: core::ffi::c_int = 64 as core::ffi::c_int;
pub const F_PIT_SHARP: core::ffi::c_float = 0.85f32;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_acelp_pitch_sharpening(
    mut x: *mut FLOAT32,
    mut pit_lag: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    i = pit_lag;
    while i < LEN_SUBFR {
        let ref mut fresh2 = *x.offset(i as isize);
        *fresh2 += *x.offset((i - pit_lag) as isize) * F_PIT_SHARP;
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_acelp_decode_1sp_per_track(
    mut idx_1p: WORD32,
    mut M: WORD32,
    mut ixheaacd_drc_offset: WORD32,
    mut track: WORD32,
    mut code_vec: *mut FLOAT32,
) -> VOID {
    let mut sign_index: WORD32 = 0;
    let mut mask: WORD32 = 0;
    let mut m: WORD32 = 0;
    let mut sp_pos: WORD32 = 0;
    mask = (((1 as core::ffi::c_int) << M) - 1 as core::ffi::c_int) as WORD32;
    sp_pos = (idx_1p & mask) + ixheaacd_drc_offset;
    sign_index = (idx_1p as core::ffi::c_int >> M & 1 as core::ffi::c_int) as WORD32;
    m = (sp_pos << 2 as core::ffi::c_int) + track;
    if sign_index == 1 as core::ffi::c_int {
        *code_vec.offset(m as isize) = (*code_vec.offset(m as isize) - 1.0f32)
            as FLOAT32;
    } else {
        *code_vec.offset(m as isize) = (*code_vec.offset(m as isize) + 1.0f32)
            as FLOAT32;
    };
}
unsafe extern "C" fn ixheaacd_acelp_decode_2sp_per_track(
    mut idx_2p: WORD32,
    mut M: WORD32,
    mut ixheaacd_drc_offset: WORD32,
    mut track: WORD32,
    mut code_vec: *mut FLOAT32,
) -> VOID {
    let mut sign_index: WORD32 = 0;
    let mut mask: WORD32 = 0;
    let mut m0: WORD32 = 0;
    let mut m1: WORD32 = 0;
    let mut sp_pos: [WORD32; 2] = [0; 2];
    mask = (((1 as core::ffi::c_int) << M) - 1 as core::ffi::c_int) as WORD32;
    sp_pos[0 as core::ffi::c_int as usize] = (idx_2p >> M & mask) + ixheaacd_drc_offset;
    sp_pos[1 as core::ffi::c_int as usize] = (idx_2p & mask) + ixheaacd_drc_offset;
    sign_index = (idx_2p as core::ffi::c_int >> 2 as WORD32 * M & 1 as core::ffi::c_int)
        as WORD32;
    m0 = (sp_pos[0 as core::ffi::c_int as usize] << 2 as core::ffi::c_int) + track;
    m1 = (sp_pos[1 as core::ffi::c_int as usize] << 2 as core::ffi::c_int) + track;
    if sp_pos[1 as core::ffi::c_int as usize] - sp_pos[0 as core::ffi::c_int as usize]
        < 0 as core::ffi::c_int
    {
        if sign_index == 1 as core::ffi::c_int {
            *code_vec.offset(m0 as isize) = (*code_vec.offset(m0 as isize) - 1.0f32)
                as FLOAT32;
            *code_vec.offset(m1 as isize) = (*code_vec.offset(m1 as isize) + 1.0f32)
                as FLOAT32;
        } else {
            *code_vec.offset(m0 as isize) = (*code_vec.offset(m0 as isize) + 1.0f32)
                as FLOAT32;
            *code_vec.offset(m1 as isize) = (*code_vec.offset(m1 as isize) - 1.0f32)
                as FLOAT32;
        }
    } else if sign_index == 1 as core::ffi::c_int {
        *code_vec.offset(m0 as isize) = (*code_vec.offset(m0 as isize) - 1.0f32)
            as FLOAT32;
        *code_vec.offset(m1 as isize) = (*code_vec.offset(m1 as isize) - 1.0f32)
            as FLOAT32;
    } else {
        *code_vec.offset(m0 as isize) = (*code_vec.offset(m0 as isize) + 1.0f32)
            as FLOAT32;
        *code_vec.offset(m1 as isize) = (*code_vec.offset(m1 as isize) + 1.0f32)
            as FLOAT32;
    };
}
unsafe extern "C" fn ixheaacd_acelp_decode_3sp_per_track(
    mut idx_3p: WORD32,
    mut M: WORD32,
    mut ixheaacd_drc_offset: WORD32,
    mut track: WORD32,
    mut code_vec: *mut FLOAT32,
) -> VOID {
    let mut j: WORD32 = 0;
    let mut mask: WORD32 = 0;
    let mut idx_2p: WORD32 = 0;
    let mut idx_1p: WORD32 = 0;
    mask = (((1 as core::ffi::c_int)
        << 2 as core::ffi::c_int * M as core::ffi::c_int - 1 as core::ffi::c_int)
        - 1 as core::ffi::c_int) as WORD32;
    idx_2p = idx_3p & mask;
    j = ixheaacd_drc_offset;
    if idx_3p as core::ffi::c_int
        >> 2 as core::ffi::c_int * M as core::ffi::c_int - 1 as core::ffi::c_int
        & 1 as core::ffi::c_int == 1 as core::ffi::c_int
    {
        j += (1 as core::ffi::c_int) << M as core::ffi::c_int - 1 as core::ffi::c_int;
    }
    ixheaacd_acelp_decode_2sp_per_track(idx_2p, M - 1 as WORD32, j, track, code_vec);
    mask = (((1 as core::ffi::c_int) << M as core::ffi::c_int + 1 as core::ffi::c_int)
        - 1 as core::ffi::c_int) as WORD32;
    idx_1p = idx_3p >> 2 as WORD32 * M & mask;
    ixheaacd_acelp_decode_1sp_per_track(idx_1p, M, ixheaacd_drc_offset, track, code_vec);
}
unsafe extern "C" fn ixheaacd_d_acelp_decode_4sp_per_track_section(
    mut index: WORD32,
    mut ixheaacd_drc_offset: WORD32,
    mut track: WORD32,
    mut code_vec: *mut FLOAT32,
) -> VOID {
    let mut j: WORD32 = 0;
    let mut idx_2p: WORD32 = 0;
    idx_2p = (index as core::ffi::c_int & 31 as core::ffi::c_int) as WORD32;
    j = ixheaacd_drc_offset;
    if index as core::ffi::c_int >> 5 as core::ffi::c_int & 1 as core::ffi::c_int
        == 1 as core::ffi::c_int
    {
        j += 4 as core::ffi::c_int;
    }
    ixheaacd_acelp_decode_2sp_per_track(idx_2p, 2 as WORD32, j, track, code_vec);
    idx_2p = (index as core::ffi::c_int >> 6 as core::ffi::c_int
        & 127 as core::ffi::c_int) as WORD32;
    ixheaacd_acelp_decode_2sp_per_track(
        idx_2p,
        3 as WORD32,
        ixheaacd_drc_offset,
        track,
        code_vec,
    );
}
unsafe extern "C" fn ixheaacd_acelp_decode_4sp_per_track(
    mut idx_4p: WORD32,
    mut track: WORD32,
    mut code_vec: *mut FLOAT32,
) -> VOID {
    let mut idx_1p: WORD32 = 0;
    let mut idx_2p: WORD32 = 0;
    let mut idx_3p: WORD32 = 0;
    match idx_4p as core::ffi::c_int >> 14 as core::ffi::c_int & 3 as core::ffi::c_int {
        0 => {
            if idx_4p as core::ffi::c_int >> 13 as core::ffi::c_int
                & 1 as core::ffi::c_int == 0 as core::ffi::c_int
            {
                ixheaacd_d_acelp_decode_4sp_per_track_section(
                    idx_4p,
                    0 as WORD32,
                    track,
                    code_vec,
                );
            } else {
                ixheaacd_d_acelp_decode_4sp_per_track_section(
                    idx_4p,
                    8 as WORD32,
                    track,
                    code_vec,
                );
            }
        }
        1 => {
            idx_1p = idx_4p >> 10 as core::ffi::c_int;
            ixheaacd_acelp_decode_1sp_per_track(
                idx_1p,
                3 as WORD32,
                0 as WORD32,
                track,
                code_vec,
            );
            ixheaacd_acelp_decode_3sp_per_track(
                idx_4p,
                3 as WORD32,
                8 as WORD32,
                track,
                code_vec,
            );
        }
        2 => {
            idx_2p = idx_4p >> 7 as core::ffi::c_int;
            ixheaacd_acelp_decode_2sp_per_track(
                idx_2p,
                3 as WORD32,
                0 as WORD32,
                track,
                code_vec,
            );
            ixheaacd_acelp_decode_2sp_per_track(
                idx_4p,
                3 as WORD32,
                8 as WORD32,
                track,
                code_vec,
            );
        }
        3 => {
            idx_3p = idx_4p >> 4 as core::ffi::c_int;
            ixheaacd_acelp_decode_3sp_per_track(
                idx_3p,
                3 as WORD32,
                0 as WORD32,
                track,
                code_vec,
            );
            ixheaacd_acelp_decode_1sp_per_track(
                idx_4p,
                3 as WORD32,
                8 as WORD32,
                track,
                code_vec,
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn ixheaacd_d_acelp_add_pulse(
    mut pos: *mut WORD32,
    mut nb_pulse: WORD32,
    mut track: WORD32,
    mut code: *mut FLOAT32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    k = 0 as core::ffi::c_int as WORD32;
    while k < nb_pulse {
        i = ((*pos.offset(k as isize) & 16 as WORD32 - 1 as WORD32)
            << 2 as core::ffi::c_int) + track;
        if *pos.offset(k as isize) & 16 as core::ffi::c_int == 0 as core::ffi::c_int {
            *code.offset(i as isize) = (*code.offset(i as isize) + 1.0f32) as WORD16
                as FLOAT32;
        } else {
            *code.offset(i as isize) = (*code.offset(i as isize) - 1.0f32) as WORD16
                as FLOAT32;
        }
        k += 1;
    }
}
unsafe extern "C" fn ixheaacd_d_acelp_decode_1p_n1(
    mut index: WORD32,
    mut N: WORD32,
    mut ixheaacd_drc_offset: WORD32,
    mut pos: *mut WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut pos1: WORD32 = 0;
    let mut mask: WORD32 = 0;
    mask = (((1 as core::ffi::c_int) << N) - 1 as core::ffi::c_int) as WORD32;
    pos1 = (index & mask) + ixheaacd_drc_offset;
    i = (index as core::ffi::c_int >> N & 1 as core::ffi::c_int) as WORD32;
    if i == 1 as core::ffi::c_int {
        pos1 += 16 as core::ffi::c_int;
    }
    *pos.offset(0 as core::ffi::c_int as isize) = pos1;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_acelp_decode_pulses_per_track(
    mut cb_index: *mut WORD32,
    code_bits: WORD16,
    mut code_vec: *mut FLOAT32,
) -> VOID {
    let mut track_idx: WORD32 = 0;
    let mut index: WORD32 = 0;
    let mut ixheaacd_drc_offset: WORD32 = 0;
    let mut pos: [WORD32; 6] = [0; 6];
    let mut i: WORD32 = 0;
    memset(
        code_vec as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (64 as size_t).wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
    );
    if code_bits as core::ffi::c_int == 12 as core::ffi::c_int {
        track_idx = 0 as core::ffi::c_int as WORD32;
        while track_idx < 4 as core::ffi::c_int {
            ixheaacd_drc_offset = *cb_index
                .offset(
                    (2 as core::ffi::c_int
                        * (track_idx as core::ffi::c_int / 2 as core::ffi::c_int))
                        as isize,
                );
            index = *cb_index
                .offset(
                    (2 as core::ffi::c_int
                        * (track_idx as core::ffi::c_int / 2 as core::ffi::c_int)
                        + 1 as core::ffi::c_int) as isize,
                );
            ixheaacd_d_acelp_decode_1p_n1(
                index,
                4 as WORD32,
                0 as WORD32,
                pos.as_mut_ptr(),
            );
            ixheaacd_d_acelp_add_pulse(
                pos.as_mut_ptr(),
                1 as WORD32,
                2 as WORD32 * ixheaacd_drc_offset + track_idx / 2 as WORD32,
                code_vec,
            );
            track_idx += 2 as core::ffi::c_int;
        }
    } else if code_bits as core::ffi::c_int == 16 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        let fresh3 = i;
        i = i + 1;
        ixheaacd_drc_offset = *cb_index.offset(fresh3 as isize);
        ixheaacd_drc_offset = (if ixheaacd_drc_offset == 0 as core::ffi::c_int {
            1 as core::ffi::c_int
        } else {
            3 as core::ffi::c_int
        }) as WORD32;
        track_idx = 0 as core::ffi::c_int as WORD32;
        while track_idx < 4 as core::ffi::c_int {
            if track_idx != ixheaacd_drc_offset {
                let fresh4 = i;
                i = i + 1;
                index = *cb_index.offset(fresh4 as isize);
                ixheaacd_d_acelp_decode_1p_n1(
                    index,
                    4 as WORD32,
                    0 as WORD32,
                    pos.as_mut_ptr(),
                );
                ixheaacd_d_acelp_add_pulse(
                    pos.as_mut_ptr(),
                    1 as WORD32,
                    track_idx,
                    code_vec,
                );
            }
            track_idx += 1;
        }
    } else if code_bits as core::ffi::c_int == 20 as core::ffi::c_int {
        track_idx = 0 as core::ffi::c_int as WORD32;
        while track_idx < 4 as core::ffi::c_int {
            index = *cb_index.offset(track_idx as isize);
            ixheaacd_acelp_decode_1sp_per_track(
                index,
                4 as WORD32,
                0 as WORD32,
                track_idx,
                code_vec,
            );
            track_idx += 1;
        }
    } else if code_bits as core::ffi::c_int == 28 as core::ffi::c_int {
        track_idx = 0 as core::ffi::c_int as WORD32;
        while track_idx < 2 as core::ffi::c_int {
            index = *cb_index.offset(track_idx as isize);
            ixheaacd_acelp_decode_2sp_per_track(
                index,
                4 as WORD32,
                0 as WORD32,
                track_idx,
                code_vec,
            );
            track_idx += 1;
        }
        track_idx = 2 as core::ffi::c_int as WORD32;
        while track_idx < 4 as core::ffi::c_int {
            index = *cb_index.offset(track_idx as isize);
            ixheaacd_acelp_decode_1sp_per_track(
                index,
                4 as WORD32,
                0 as WORD32,
                track_idx,
                code_vec,
            );
            track_idx += 1;
        }
    } else if code_bits as core::ffi::c_int == 36 as core::ffi::c_int {
        track_idx = 0 as core::ffi::c_int as WORD32;
        while track_idx < 4 as core::ffi::c_int {
            index = *cb_index.offset(track_idx as isize);
            ixheaacd_acelp_decode_2sp_per_track(
                index,
                4 as WORD32,
                0 as WORD32,
                track_idx,
                code_vec,
            );
            track_idx += 1;
        }
    } else if code_bits as core::ffi::c_int == 44 as core::ffi::c_int {
        track_idx = 0 as core::ffi::c_int as WORD32;
        while track_idx < 2 as core::ffi::c_int {
            index = *cb_index.offset(track_idx as isize);
            ixheaacd_acelp_decode_3sp_per_track(
                index,
                4 as WORD32,
                0 as WORD32,
                track_idx,
                code_vec,
            );
            track_idx += 1;
        }
        track_idx = 2 as core::ffi::c_int as WORD32;
        while track_idx < 4 as core::ffi::c_int {
            index = *cb_index.offset(track_idx as isize);
            ixheaacd_acelp_decode_2sp_per_track(
                index,
                4 as WORD32,
                0 as WORD32,
                track_idx,
                code_vec,
            );
            track_idx += 1;
        }
    } else if code_bits as core::ffi::c_int == 52 as core::ffi::c_int {
        track_idx = 0 as core::ffi::c_int as WORD32;
        while track_idx < 4 as core::ffi::c_int {
            index = *cb_index.offset(track_idx as isize);
            ixheaacd_acelp_decode_3sp_per_track(
                index,
                4 as WORD32,
                0 as WORD32,
                track_idx,
                code_vec,
            );
            track_idx += 1;
        }
    } else if code_bits as core::ffi::c_int == 64 as core::ffi::c_int {
        track_idx = 0 as core::ffi::c_int as WORD32;
        while track_idx < 4 as core::ffi::c_int {
            index = (*cb_index.offset(track_idx as isize) << 14 as core::ffi::c_int)
                + *cb_index
                    .offset(
                        (track_idx as core::ffi::c_int + 4 as core::ffi::c_int) as isize,
                    );
            ixheaacd_acelp_decode_4sp_per_track(index, track_idx, code_vec);
            track_idx += 1;
        }
    }
}
unsafe extern "C" fn ixheaacd_acelp_decode_gains(
    mut index: WORD32,
    mut code_vec: *mut FLOAT32,
    mut pitch_gain: *mut FLOAT32,
    mut codebook_gain: *mut FLOAT32,
    mut mean_exc_energy: FLOAT32,
    mut energy: *mut FLOAT32,
) {
    let mut i: WORD32 = 0;
    let mut avg_innov_energy: FLOAT32 = 0.;
    let mut est_gain: FLOAT32 = 0.;
    let mut gain_table: *const FLOAT32 = ixheaacd_int_leave_gain_table.as_ptr();
    avg_innov_energy = 0.01f32 as FLOAT32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < LEN_SUBFR {
        avg_innov_energy += *code_vec.offset(i as isize) * *code_vec.offset(i as isize);
        i += 1;
    }
    *energy = avg_innov_energy;
    avg_innov_energy = (10.0f64
        * log10((avg_innov_energy / LEN_SUBFR as FLOAT32) as core::ffi::c_double))
        as FLOAT32;
    est_gain = mean_exc_energy - avg_innov_energy;
    est_gain = pow(10.0f64, 0.05f64 * est_gain as core::ffi::c_double) as FLOAT32;
    *pitch_gain = *gain_table
        .offset((index as core::ffi::c_int * 2 as core::ffi::c_int) as isize);
    *codebook_gain = *gain_table
        .offset(
            (index as core::ffi::c_int * 2 as core::ffi::c_int + 1 as core::ffi::c_int)
                as isize,
        ) * est_gain;
}
unsafe extern "C" fn ixheaacd_acelp_decode_gains_with_ec(
    mut index: WORD32,
    mut code_vec: *mut FLOAT32,
    mut pitch_gain: *mut FLOAT32,
    mut codebook_gain: *mut FLOAT32,
    mut mean_exc_energy: FLOAT32,
    mut energy: *mut FLOAT32,
    mut past_pitch_gain: *mut FLOAT32,
    mut past_gain_code: *mut FLOAT32,
    mut bfi: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut avg_innov_energy: FLOAT32 = 0.;
    let mut est_gain: FLOAT32 = 0.;
    let mut gain_inov: FLOAT32 = 0.;
    let mut gain_table: *const FLOAT32 = ixheaacd_int_leave_gain_table.as_ptr();
    avg_innov_energy = 0.01f32 as FLOAT32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < LEN_SUBFR {
        avg_innov_energy += *code_vec.offset(i as isize) * *code_vec.offset(i as isize);
        i += 1;
    }
    *energy = avg_innov_energy;
    gain_inov = (1 as core::ffi::c_int as core::ffi::c_double
        / sqrt((avg_innov_energy / LEN_SUBFR as FLOAT32) as core::ffi::c_double))
        as FLOAT32;
    if bfi != 0 {
        let mut tgpit: FLOAT32 = *past_pitch_gain;
        if tgpit > 0.95f32 {
            tgpit = 0.95f32 as FLOAT32;
        } else if tgpit < 0.5f32 {
            tgpit = 0.5f32 as FLOAT32;
        }
        *pitch_gain = tgpit;
        tgpit = (tgpit as core::ffi::c_float * 0.95f32) as FLOAT32;
        *past_pitch_gain = tgpit;
        tgpit = 1.4f32 - tgpit;
        tgpit = *past_gain_code * tgpit;
        *codebook_gain = tgpit * gain_inov;
        *past_gain_code = tgpit;
        return;
    }
    avg_innov_energy = (10.0f64
        * log10((avg_innov_energy / LEN_SUBFR as FLOAT32) as core::ffi::c_double))
        as FLOAT32;
    est_gain = mean_exc_energy - avg_innov_energy;
    est_gain = pow(10.0f64, 0.05f64 * est_gain as core::ffi::c_double) as FLOAT32;
    if bfi == 0 {
        *pitch_gain = *gain_table
            .offset((index as core::ffi::c_int * 2 as core::ffi::c_int) as isize);
        *past_pitch_gain = *pitch_gain;
    }
    *codebook_gain = *gain_table
        .offset(
            (index as core::ffi::c_int * 2 as core::ffi::c_int + 1 as core::ffi::c_int)
                as isize,
        ) * est_gain;
    *past_gain_code = *codebook_gain / gain_inov;
}
unsafe extern "C" fn ixheaacd_cb_exc_calc(
    mut xcitation_curr: *mut FLOAT32,
    mut pitch_lag: WORD32,
    mut frac: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut s: FLOAT32 = 0.;
    let mut x0: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut x1: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut x2: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut c1: *const FLOAT32 = 0 as *const FLOAT32;
    let mut c2: *const FLOAT32 = 0 as *const FLOAT32;
    x0 = &mut *xcitation_curr.offset(-pitch_lag as isize) as *mut FLOAT32;
    frac = -frac;
    if frac < 0 as core::ffi::c_int {
        frac += UP_SAMP;
        x0 = x0.offset(-1);
    }
    j = 0 as core::ffi::c_int as WORD32;
    while j < LEN_SUBFR + 1 as core::ffi::c_int {
        let fresh5 = x0;
        x0 = x0.offset(1);
        x1 = fresh5;
        x2 = x1.offset(1 as core::ffi::c_int as isize);
        c1 = &*ixheaacd_interpol_filt.as_ptr().offset(frac as isize) as *const FLOAT32;
        c2 = &*ixheaacd_interpol_filt.as_ptr().offset((UP_SAMP - frac) as isize)
            as *const FLOAT32;
        s = 0.0f32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < INTER_LP_FIL_ORDER {
            let fresh6 = x1;
            x1 = x1.offset(-1);
            let fresh7 = x2;
            x2 = x2.offset(1);
            s += *fresh6 * *c1 + *fresh7 * *c2;
            i += 1;
            c1 = c1.offset(UP_SAMP as isize);
            c2 = c2.offset(UP_SAMP as isize);
        }
        *xcitation_curr.offset(j as isize) = s;
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_acelp_alias_cnx(
    mut usac_data: *mut ia_usac_data_struct,
    mut pstr_td_frame_data: *mut ia_td_frame_data_struct,
    mut k: WORD32,
    mut lp_filt_coeff: *mut FLOAT32,
    mut stability_factor: FLOAT32,
    mut st: ia_usac_lpd_decoder_handle,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut subfr_idx: WORD32 = 0;
    let mut pitch_lag: WORD32 = 0 as WORD32;
    let mut pitch_lag_frac: WORD32 = 0 as WORD32;
    let mut index: WORD32 = 0;
    let mut pitch_flag: WORD32 = 0;
    let mut pitch_lag_max: WORD32 = 0;
    let mut pitch_lag_min: WORD32 = 0 as WORD32;
    let mut tmp: FLOAT32 = 0.;
    let mut pitch_gain: FLOAT32 = 0.;
    let mut gain_code: FLOAT32 = 0.;
    let mut voicing_factor: FLOAT32 = 0.;
    let mut r_v: FLOAT32 = 0.;
    let mut innov_energy: FLOAT32 = 0.;
    let mut pitch_energy: FLOAT32 = 0.;
    let mut mean_ener_code: FLOAT32 = 0.;
    let mut gain_smooth: FLOAT32 = 0.;
    let mut gain_code0: FLOAT32 = 0.;
    let mut cpe: FLOAT32 = 0.;
    let mut code: [FLOAT32; 64] = [
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
    let mut synth_temp: [FLOAT32; 144] = [
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
    let mut post_process_exc: [FLOAT32; 64] = [
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
    let mut gain_smooth_factor: FLOAT32 = 0.;
    let mut ptr_lp_filt_coeff: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut pitch_min: WORD32 = 0;
    let mut pitch_fr2: WORD32 = 0;
    let mut pitch_fr1: WORD32 = 0;
    let mut pitch_max: WORD32 = 0;
    let mut subfr_nb: WORD32 = 0 as WORD32;
    let num_codebits_table: [WORD16; 8] = [
        20 as core::ffi::c_int as WORD16,
        28 as core::ffi::c_int as WORD16,
        36 as core::ffi::c_int as WORD16,
        44 as core::ffi::c_int as WORD16,
        52 as core::ffi::c_int as WORD16,
        64 as core::ffi::c_int as WORD16,
        12 as core::ffi::c_int as WORD16,
        16 as core::ffi::c_int as WORD16,
    ];
    let mut x: [FLOAT32; 128] = [
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
        0.,
    ];
    let mut xn2: [FLOAT32; 272] = [
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
    let mut int_x: [WORD32; 128] = [0 as core::ffi::c_int; 128];
    let mut TTT: WORD32 = 0;
    let mut len_subfr: WORD32 = (*usac_data).len_subfrm;
    let mut fac_length: WORD32 = 0;
    let mut shiftp: WORD8 = 0;
    let mut preshift: WORD32 = 0;
    let mut ptr_scratch: *mut WORD32 = &mut *((*usac_data).scratch_buffer)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
    let mut int_xn2: *mut WORD32 = &mut *((*usac_data).x_ac_dec)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
    let mut loop_count: WORD32 = 0 as WORD32;
    let mut core_mode: WORD32 = (*pstr_td_frame_data).acelp_core_mode;
    let mut synth_signal: *mut FLOAT32 = &mut *((*usac_data).synth_buf)
        .as_mut_ptr()
        .offset(
            (len_subfr as core::ffi::c_int * k as core::ffi::c_int + MAX_PITCH
                + (NUM_FRAMES * (*usac_data).num_subfrm as core::ffi::c_int
                    / 2 as core::ffi::c_int - 1 as core::ffi::c_int) * LEN_SUBFR)
                as isize,
        ) as *mut FLOAT32;
    let mut xcitation_curr: *mut FLOAT32 = &mut *((*usac_data).exc_buf)
        .as_mut_ptr()
        .offset(
            (len_subfr as core::ffi::c_int * k as core::ffi::c_int + MAX_PITCH
                + (INTER_LP_FIL_ORDER + 1 as core::ffi::c_int)) as isize,
        ) as *mut FLOAT32;
    let mut ptr_pitch_gain: *mut FLOAT32 = &mut *((*usac_data).pitch_gain)
        .as_mut_ptr()
        .offset(
            (k as core::ffi::c_int * (*usac_data).num_subfrm as core::ffi::c_int
                + (NUM_FRAMES * (*usac_data).num_subfrm as core::ffi::c_int
                    / 2 as core::ffi::c_int - 1 as core::ffi::c_int)) as isize,
        ) as *mut FLOAT32;
    let mut ptr_pitch: *mut WORD32 = &mut *((*usac_data).pitch)
        .as_mut_ptr()
        .offset(
            (k as core::ffi::c_int * (*usac_data).num_subfrm as core::ffi::c_int
                + (NUM_FRAMES * (*usac_data).num_subfrm as core::ffi::c_int
                    / 2 as core::ffi::c_int - 1 as core::ffi::c_int)) as isize,
        ) as *mut WORD32;
    fac_length = (len_subfr as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
    let mut bfi: WORD32 = if (*usac_data)
        .num_lost_lpd_frames[(*usac_data).present_chan as usize] > 0 as core::ffi::c_int
    {
        1 as WORD32
    } else {
        0 as WORD32
    };
    let mut i_offset: WORD32 = ((*(*usac_data)
        .str_tddec[(*usac_data).present_chan as usize])
        .fscale * TMIN + FSCALE_DENOM / 2 as WORD32) / FSCALE_DENOM - TMIN;
    let pitch_max_val: WORD32 = TMAX + 6 as WORD32 * i_offset;
    let mut code_t: [WORD16; 64] = [0; 64];
    if (*st).mode_prev > 0 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < fac_length as core::ffi::c_int / 2 as core::ffi::c_int {
            x[i as usize] = (*st).fac_gain
                * (*pstr_td_frame_data).fac[(k * FAC_LENGTH + 2 as WORD32 * i) as usize]
                    as FLOAT32;
            x[(fac_length / 2 as WORD32 + i) as usize] = (*st).fac_gain
                * (*pstr_td_frame_data)
                    .fac[(k as core::ffi::c_int * FAC_LENGTH
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
        ixheaacd_acelp_mdct(
            int_x.as_mut_ptr(),
            int_xn2,
            &mut preshift,
            fac_length,
            ptr_scratch,
        );
        ixheaacd_fix2float(
            int_xn2,
            xn2.as_mut_ptr().offset(fac_length as isize),
            fac_length,
            &mut shiftp,
            &mut preshift,
        );
        ixheaacd_vec_cnst_mul(
            2.0f32 / fac_length as FLOAT32,
            xn2.as_mut_ptr().offset(fac_length as isize),
            xn2.as_mut_ptr().offset(fac_length as isize),
            fac_length,
        );
        memset(
            xn2.as_mut_ptr() as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (fac_length as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        ixheaacd_lpc_wt_synthesis_tool(
            ((*st).lp_flt_coeff_a_prev).as_mut_ptr() as *mut FLOAT32,
            xn2.as_mut_ptr().offset(fac_length as isize),
            fac_length,
        );
        i = 0 as core::ffi::c_int as WORD32;
        while i < 2 as WORD32 * fac_length {
            xn2[i as usize]
                += *synth_signal.offset((i - 2 as WORD32 * fac_length) as isize);
            i += 1;
        }
        memcpy(
            synth_signal.offset(-(fac_length as isize)) as *mut core::ffi::c_void,
            xn2.as_mut_ptr().offset(fac_length as isize) as *const core::ffi::c_void,
            (fac_length as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        tmp = 0.0f32;
        ixheaacd_preemphsis_tool_float(
            xn2.as_mut_ptr(),
            PREEMPH_FILT_FAC,
            2 as WORD32 * fac_length,
            tmp,
        );
        ptr_lp_filt_coeff = ((*st).lp_flt_coeff_a_prev).as_mut_ptr() as *mut FLOAT32;
        TTT = (fac_length as core::ffi::c_int % LEN_SUBFR) as WORD32;
        if TTT != 0 as core::ffi::c_int {
            ixheaacd_residual_tool_float(
                ptr_lp_filt_coeff,
                &mut *xn2.as_mut_ptr().offset(fac_length as isize),
                &mut *xcitation_curr
                    .offset((fac_length - 2 as WORD32 * fac_length) as isize),
                TTT,
                1 as WORD32,
            );
            ptr_lp_filt_coeff = ptr_lp_filt_coeff
                .offset((ORDER + 1 as core::ffi::c_int) as isize);
        }
        loop_count = ((fac_length as core::ffi::c_int + TTT as core::ffi::c_int)
            / LEN_SUBFR) as WORD32;
        ixheaacd_residual_tool_float(
            ptr_lp_filt_coeff,
            &mut *xn2.as_mut_ptr().offset((fac_length + TTT) as isize),
            &mut *xcitation_curr.offset((TTT - fac_length) as isize),
            LEN_SUBFR,
            loop_count,
        );
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < ORDER {
        synth_temp[i as usize] = *synth_signal
            .offset((i as core::ffi::c_int - ORDER) as isize)
            - PREEMPH_FILT_FAC
                * *synth_signal
                    .offset(
                        (i as core::ffi::c_int - ORDER - 1 as core::ffi::c_int) as isize,
                    );
        i += 1;
    }
    i = (((*st).fscale as core::ffi::c_int * TMIN + FSCALE_DENOM / 2 as core::ffi::c_int)
        / FSCALE_DENOM - TMIN) as WORD32;
    pitch_min = TMIN + i;
    pitch_fr2 = TFR2 - i;
    pitch_fr1 = TFR1 as WORD32;
    pitch_max = TMAX + 6 as WORD32 * i;
    ptr_lp_filt_coeff = lp_filt_coeff as *mut FLOAT32;
    subfr_idx = 0 as core::ffi::c_int as WORD32;
    while subfr_idx < len_subfr {
        pitch_flag = subfr_idx;
        if len_subfr == 256 as core::ffi::c_int
            && subfr_idx == 2 as core::ffi::c_int * LEN_SUBFR
        {
            pitch_flag = 0 as core::ffi::c_int as WORD32;
        }
        index = (*pstr_td_frame_data).acb_index[(k * 4 as WORD32 + subfr_nb) as usize];
        if pitch_flag == 0 as core::ffi::c_int {
            if index
                < (pitch_fr2 as core::ffi::c_int - pitch_min as core::ffi::c_int)
                    * 4 as core::ffi::c_int
            {
                pitch_lag = (pitch_min as core::ffi::c_int
                    + index as core::ffi::c_int / 4 as core::ffi::c_int) as WORD32;
                pitch_lag_frac = (index as core::ffi::c_int
                    - (pitch_lag as core::ffi::c_int - pitch_min as core::ffi::c_int)
                        * 4 as core::ffi::c_int) as WORD32;
            } else if index
                < (pitch_fr2 as core::ffi::c_int - pitch_min as core::ffi::c_int)
                    * 4 as core::ffi::c_int
                    + (pitch_fr1 as core::ffi::c_int - pitch_fr2 as core::ffi::c_int)
                        * 2 as core::ffi::c_int
            {
                index
                    -= (pitch_fr2 as core::ffi::c_int - pitch_min as core::ffi::c_int)
                        * 4 as core::ffi::c_int;
                pitch_lag = (pitch_fr2 as core::ffi::c_int
                    + index as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
                pitch_lag_frac = (index as core::ffi::c_int
                    - (pitch_lag as core::ffi::c_int - pitch_fr2 as core::ffi::c_int)
                        * 2 as core::ffi::c_int) as WORD32;
                pitch_lag_frac *= 2 as core::ffi::c_int;
            } else {
                pitch_lag = (index as core::ffi::c_int + pitch_fr1 as core::ffi::c_int
                    - (pitch_fr2 as core::ffi::c_int - pitch_min as core::ffi::c_int)
                        * 4 as core::ffi::c_int
                    - (pitch_fr1 as core::ffi::c_int - pitch_fr2 as core::ffi::c_int)
                        * 2 as core::ffi::c_int) as WORD32;
                pitch_lag_frac = 0 as core::ffi::c_int as WORD32;
            }
            pitch_lag_min = (pitch_lag as core::ffi::c_int - 8 as core::ffi::c_int)
                as WORD32;
            if pitch_lag_min < pitch_min {
                pitch_lag_min = pitch_min;
            }
            pitch_lag_max = (pitch_lag_min as core::ffi::c_int + 15 as core::ffi::c_int)
                as WORD32;
            if pitch_lag_max > pitch_max {
                pitch_lag_max = pitch_max;
                pitch_lag_min = (pitch_lag_max as core::ffi::c_int
                    - 15 as core::ffi::c_int) as WORD32;
            }
        } else {
            pitch_lag = (pitch_lag_min as core::ffi::c_int
                + index as core::ffi::c_int / 4 as core::ffi::c_int) as WORD32;
            pitch_lag_frac = (index as core::ffi::c_int
                - (pitch_lag as core::ffi::c_int - pitch_lag_min as core::ffi::c_int)
                    * 4 as core::ffi::c_int) as WORD32;
        }
        if (*usac_data).ec_flag != 0 {
            if bfi != 0 {
                if (*usac_data).pitch_lag_old >= pitch_max_val {
                    (*usac_data).pitch_lag_old = (pitch_max_val as core::ffi::c_int
                        - 5 as core::ffi::c_int) as WORD32;
                }
                pitch_lag = (*usac_data).pitch_lag_old;
                pitch_lag_frac = (*usac_data).pitch_lag_frac_old;
            }
        }
        ixheaacd_cb_exc_calc(
            &mut *xcitation_curr.offset(subfr_idx as isize),
            pitch_lag,
            pitch_lag_frac,
        );
        mean_ener_code = ((*pstr_td_frame_data).mean_energy[k as usize]
            as core::ffi::c_float * 12.0f32 + 18.0f32) as FLOAT32;
        if (*pstr_td_frame_data)
            .ltp_filtering_flag[(k * 4 as WORD32 + subfr_nb) as usize]
            == 0 as core::ffi::c_int
        {
            i = 0 as core::ffi::c_int as WORD32;
            while i < LEN_SUBFR {
                code[i as usize] = (0.18f64
                    * *xcitation_curr.offset((i - 1 as WORD32 + subfr_idx) as isize)
                        as core::ffi::c_double
                    + 0.64f64
                        * *xcitation_curr.offset((i + subfr_idx) as isize)
                            as core::ffi::c_double
                    + 0.18f64
                        * *xcitation_curr.offset((i + 1 as WORD32 + subfr_idx) as isize)
                            as core::ffi::c_double) as FLOAT32;
                i += 1;
            }
            ixheaacd_mem_cpy(
                code.as_mut_ptr() as *const FLOAT32,
                &mut *xcitation_curr.offset(subfr_idx as isize),
                LEN_SUBFR,
            );
        }
        if (*usac_data).frame_ok == 1 as core::ffi::c_int {
            ixheaacd_acelp_decode_pulses_per_track(
                &mut *(*((*pstr_td_frame_data).icb_index)
                    .as_mut_ptr()
                    .offset((k * 4 as WORD32 + subfr_nb) as isize))
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize),
                num_codebits_table[core_mode as usize],
                code.as_mut_ptr(),
            );
        } else if (*usac_data).ec_flag != 0 {
            let mut idx: WORD32 = 0;
            if bfi != 0 {
                idx = 0 as core::ffi::c_int as WORD32;
                while idx < LEN_SUBFR {
                    (*usac_data).seed_ace = (((*usac_data).seed_ace as core::ffi::c_int
                        * 31821 as core::ffi::c_int >> 1 as core::ffi::c_int)
                        + 13849 as core::ffi::c_int) as WORD16;
                    code_t[idx as usize] = ((*usac_data).seed_ace as core::ffi::c_int
                        >> 4 as core::ffi::c_int) as WORD16;
                    code[idx as usize] = code_t[idx as usize] as FLOAT32
                        / 512 as core::ffi::c_int as FLOAT32;
                    idx += 1;
                }
            }
        }
        tmp = 0.0f32;
        ixheaacd_preemphsis_tool_float(code.as_mut_ptr(), TILT_CODE, LEN_SUBFR, tmp);
        i = pitch_lag;
        if pitch_lag_frac > 2 as core::ffi::c_int {
            i += 1;
        }
        if i >= 0 as core::ffi::c_int {
            ixheaacd_acelp_pitch_sharpening(code.as_mut_ptr(), i);
        }
        index = (*pstr_td_frame_data).gains[(k * 4 as WORD32 + subfr_nb) as usize];
        if (*usac_data).ec_flag != 0 {
            ixheaacd_acelp_decode_gains_with_ec(
                index,
                code.as_mut_ptr(),
                &mut pitch_gain,
                &mut gain_code,
                mean_ener_code,
                &mut innov_energy,
                &mut (*usac_data).past_pitch_gain,
                &mut (*usac_data).past_gain_code,
                bfi,
            );
        } else {
            ixheaacd_acelp_decode_gains(
                index,
                code.as_mut_ptr(),
                &mut pitch_gain,
                &mut gain_code,
                mean_ener_code,
                &mut innov_energy,
            );
        }
        pitch_energy = 0.0f32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < LEN_SUBFR {
            pitch_energy
                += *xcitation_curr.offset((i + subfr_idx) as isize)
                    * *xcitation_curr.offset((i + subfr_idx) as isize);
            i += 1;
        }
        pitch_energy *= pitch_gain * pitch_gain;
        innov_energy *= gain_code * gain_code;
        r_v = (pitch_energy - innov_energy) / (pitch_energy + innov_energy);
        i = 0 as core::ffi::c_int as WORD32;
        while i < LEN_SUBFR {
            post_process_exc[i as usize] = pitch_gain
                * *xcitation_curr.offset((i + subfr_idx) as isize);
            i += 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < LEN_SUBFR {
            *xcitation_curr.offset((i + subfr_idx) as isize) = pitch_gain
                * *xcitation_curr.offset((i + subfr_idx) as isize)
                + gain_code * code[i as usize];
            i += 1;
        }
        i = pitch_lag;
        if pitch_lag_frac > 2 as core::ffi::c_int {
            i += 1;
        }
        if i > pitch_max {
            i = pitch_max;
        }
        let fresh0 = ptr_pitch;
        ptr_pitch = ptr_pitch.offset(1);
        *fresh0 = i;
        let fresh1 = ptr_pitch_gain;
        ptr_pitch_gain = ptr_pitch_gain.offset(1);
        *fresh1 = pitch_gain;
        voicing_factor = (0.5f64 * (1.0f64 - r_v as core::ffi::c_double)) as FLOAT32;
        gain_smooth_factor = stability_factor * voicing_factor;
        gain_code0 = gain_code;
        if gain_code0 < (*st).gain_threshold {
            gain_code0 = (gain_code0 as core::ffi::c_double * 1.19f64) as FLOAT32;
            if gain_code0 > (*st).gain_threshold {
                gain_code0 = (*st).gain_threshold;
            }
        } else {
            gain_code0 = (gain_code0 as core::ffi::c_double / 1.19f64) as FLOAT32;
            if gain_code0 < (*st).gain_threshold {
                gain_code0 = (*st).gain_threshold;
            }
        }
        (*st).gain_threshold = gain_code0;
        gain_smooth = ((gain_smooth_factor * gain_code0) as core::ffi::c_double
            + (1.0f64 - gain_smooth_factor as core::ffi::c_double)
                * gain_code as core::ffi::c_double) as FLOAT32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < LEN_SUBFR {
            code[i as usize] *= gain_smooth;
            i += 1;
        }
        cpe = (0.125f64 * (1.0f64 + r_v as core::ffi::c_double)) as FLOAT32;
        post_process_exc[0 as core::ffi::c_int as usize]
            += code[0 as core::ffi::c_int as usize]
                - cpe * code[1 as core::ffi::c_int as usize];
        i = 1 as core::ffi::c_int as WORD32;
        while i < LEN_SUBFR - 1 as core::ffi::c_int {
            post_process_exc[i as usize]
                += code[i as usize]
                    - cpe
                        * (code[(i as core::ffi::c_int - 1 as core::ffi::c_int) as usize]
                            + code[(i as core::ffi::c_int + 1 as core::ffi::c_int)
                                as usize]);
            i += 1;
        }
        post_process_exc[(LEN_SUBFR - 1 as core::ffi::c_int) as usize]
            += code[(LEN_SUBFR - 1 as core::ffi::c_int) as usize]
                - cpe * code[(LEN_SUBFR - 2 as core::ffi::c_int) as usize];
        ixheaacd_synthesis_tool_float(
            ptr_lp_filt_coeff as *mut FLOAT32,
            post_process_exc.as_mut_ptr(),
            &mut *synth_signal.offset(subfr_idx as isize),
            LEN_SUBFR,
            synth_temp.as_mut_ptr(),
        );
        memcpy(
            synth_temp.as_mut_ptr() as *mut core::ffi::c_void,
            &mut *synth_signal
                .offset((subfr_idx as core::ffi::c_int + LEN_SUBFR - ORDER) as isize)
                as *mut FLOAT32 as *const core::ffi::c_void,
            (ORDER as size_t).wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        ptr_lp_filt_coeff = ptr_lp_filt_coeff
            .offset((ORDER + 1 as core::ffi::c_int) as isize);
        subfr_nb += 1;
        subfr_idx += LEN_SUBFR;
    }
    ixheaacd_deemphsis_tool(
        synth_signal,
        len_subfr,
        *synth_signal.offset(-(1 as core::ffi::c_int) as isize),
    );
    memset(
        synth_temp.as_mut_ptr().offset(16 as core::ffi::c_int as isize)
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (128 as size_t).wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
    );
    ixheaacd_synthesis_tool_float1(
        ptr_lp_filt_coeff as *mut FLOAT32,
        synth_temp.as_mut_ptr().offset(16 as core::ffi::c_int as isize),
        128 as WORD32,
    );
    ptr_lp_filt_coeff = ptr_lp_filt_coeff
        .offset(-((2 as core::ffi::c_int * (ORDER + 1 as core::ffi::c_int)) as isize));
    memcpy(
        ((*st).lp_flt_coeff_a_prev).as_mut_ptr() as *mut core::ffi::c_void,
        ptr_lp_filt_coeff as *const core::ffi::c_void,
        ((2 as core::ffi::c_int * (ORDER + 1 as core::ffi::c_int)) as size_t)
            .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
    );
    memcpy(
        ((*st).exc_prev).as_mut_ptr() as *mut core::ffi::c_void,
        synth_signal
            .offset(len_subfr as isize)
            .offset(-((1 as WORD32 + fac_length) as isize)) as *const core::ffi::c_void,
        ((1 as WORD32 + fac_length) as size_t)
            .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
    );
    memcpy(
        ((*st).exc_prev)
            .as_mut_ptr()
            .offset(1 as core::ffi::c_int as isize)
            .offset(fac_length as isize) as *mut core::ffi::c_void,
        synth_temp.as_mut_ptr().offset(16 as core::ffi::c_int as isize)
            as *const core::ffi::c_void,
        (fac_length as size_t).wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
    );
    ixheaacd_deemphsis_tool(
        ((*st).exc_prev)
            .as_mut_ptr()
            .offset(1 as core::ffi::c_int as isize)
            .offset(fac_length as isize),
        fac_length,
        *synth_signal
            .offset((len_subfr as core::ffi::c_int - 1 as core::ffi::c_int) as isize),
    );
    if (*usac_data).ec_flag != 0 {
        (*usac_data).pitch_lag_old = pitch_lag;
        (*usac_data).pitch_lag_frac_old = pitch_lag_frac;
    }
}
