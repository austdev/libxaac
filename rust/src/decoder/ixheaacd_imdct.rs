extern "C" {
    pub type ia_sbr_dec_inst_struct;
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
    static ixheaacd_pre_post_twid_cos_512: [WORD32; 512];
    static ixheaacd_pre_post_twid_sin_512: [WORD32; 512];
    static ixheaacd_pre_post_twid_cos_384: [WORD32; 384];
    static ixheaacd_pre_post_twid_sin_384: [WORD32; 384];
    static ixheaacd_pre_post_twid_cos_64: [WORD32; 64];
    static ixheaacd_pre_post_twid_sin_64: [WORD32; 64];
    static ixheaacd_pre_post_twid_cos_48: [WORD32; 48];
    static ixheaacd_pre_post_twid_sin_48: [WORD32; 48];
    static ixheaacd_power_10_table: [FLOAT64; 28];
    static ixheaacd_samp_rate_info: [ia_usac_samp_rate_info; 0];
    fn ixheaacd_complex_fft(
        data_r: *mut WORD32,
        data_i: *mut WORD32,
        len: WORD32,
        fft_mode: WORD32,
        preshift: *mut WORD32,
    ) -> VOID;
    fn ixheaacd_fr_alias_cnx_fix(
        x_in: *mut WORD32,
        len_subfr: WORD32,
        lfac: WORD32,
        iaq: *mut WORD32,
        izir: *mut WORD32,
        ifacdec: *mut WORD32,
        qshift1: *mut WORD8,
        qshift2: WORD8,
        qshift3: WORD8,
        preshift: *mut WORD32,
        ptr_scratch: *mut WORD32,
    ) -> VOID;
    fn ixheaacd_lpd_bpf_fix(
        usac_data: *mut ia_usac_data_struct,
        is_short: WORD32,
        out_buffer: *mut FLOAT32,
        st: ia_usac_lpd_decoder_handle,
    ) -> WORD32;
    fn ixheaacd_usac_apply_ec(
        pstr_usac_data: *mut ia_usac_data_struct,
        pstr_samp_rate_info: *const ia_usac_samp_rate_info,
        ch: WORD32,
    ) -> VOID;
    fn ixheaacd_calc_window(
        win: *mut *mut WORD32,
        len: WORD32,
        wfun_select: WORD32,
        ec_flag: WORD32,
    ) -> WORD32;
    fn ixheaacd_combine_fac(
        src1: *mut WORD32,
        src2: *mut WORD32,
        dest: *mut WORD32,
        len: WORD32,
        shift1: WORD8,
        shift2: WORD8,
    ) -> VOID;
    fn ixheaacd_windowing_long1(
        src1: *mut WORD32,
        src2: *mut WORD32,
        win_fwd: *const WORD32,
        win_rev: *const WORD32,
        dest: *mut WORD32,
        vlen: WORD32,
        shift1: WORD8,
        shift2: WORD8,
    ) -> WORD8;
    fn ixheaacd_windowing_long2(
        src1: *mut WORD32,
        win_fwd: *const WORD32,
        fac_data_out: *mut WORD32,
        over_lap: *mut WORD32,
        p_out_buffer: *mut WORD32,
        ixheaacd_drc_offset: *mut offset_lengths,
        shift1: WORD8,
        shift2: WORD8,
        shift3: WORD8,
    ) -> WORD8;
    fn ixheaacd_windowing_long3(
        src1: *mut WORD32,
        win_fwd: *const WORD32,
        over_lap: *mut WORD32,
        p_out_buffer: *mut WORD32,
        win_rev: *const WORD32,
        ixheaacd_drc_offset: *mut offset_lengths,
        shift1: WORD8,
        shift2: WORD8,
    ) -> WORD8;
    fn ixheaacd_windowing_short1(
        src1: *mut WORD32,
        src2: *mut WORD32,
        fp: *mut WORD32,
        ixheaacd_drc_offset: *mut offset_lengths,
        shiftp: WORD8,
        shift_olap: WORD8,
    ) -> VOID;
    fn ixheaacd_windowing_short2(
        src1: *mut WORD32,
        win_fwd: *mut WORD32,
        fp: *mut WORD32,
        ixheaacd_drc_offset: *mut offset_lengths,
        shiftp: WORD8,
        shift_olap: WORD8,
    ) -> VOID;
    fn ixheaacd_windowing_short3(
        src1: *mut WORD32,
        win_rev: *mut WORD32,
        fp: *mut WORD32,
        nshort: WORD32,
        shiftp: WORD8,
        shift_olap: WORD8,
    ) -> WORD8;
    fn ixheaacd_windowing_short4(
        src1: *mut WORD32,
        win_fwd: *mut WORD32,
        fp: *mut WORD32,
        win_fwd1: *mut WORD32,
        nshort: WORD32,
        flag: WORD32,
        shiftp: WORD8,
        shift_olap: WORD8,
        output_q: WORD8,
    ) -> WORD8;
    fn ixheaacd_scale_down(
        dest: *mut WORD32,
        src: *mut WORD32,
        len: WORD32,
        shift1: WORD8,
        shift2: WORD8,
    ) -> VOID;
    fn ixheaacd_scale_down_adj(
        dest: *mut WORD32,
        src: *mut WORD32,
        len: WORD32,
        shift1: WORD8,
        shift2: WORD8,
    ) -> VOID;
    static mut ixheaacd_calc_pre_twid: Option<
        unsafe extern "C" fn(
            *mut WORD32,
            *mut WORD32,
            *mut WORD32,
            WORD32,
            *const WORD32,
            *const WORD32,
        ) -> VOID,
    >;
    static mut ixheaacd_calc_post_twid: Option<
        unsafe extern "C" fn(
            *mut WORD32,
            *mut WORD32,
            *mut WORD32,
            WORD32,
            *const WORD32,
            *const WORD32,
        ) -> VOID,
    >;
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
pub type FLOAT64 = core::ffi::c_double;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct offset_lengths {
    pub lfac: WORD32,
    pub n_flat_ls: WORD32,
    pub n_trans_ls: WORD32,
    pub n_long: WORD32,
    pub n_short: WORD32,
}
pub const LEN_FRAME: core::ffi::c_int = 256 as core::ffi::c_int;
pub const ORDER: core::ffi::c_int = 16 as core::ffi::c_int;
pub const FAC_LENGTH: core::ffi::c_int = 128 as core::ffi::c_int;
pub const ONLY_LONG_SEQUENCE: core::ffi::c_int = 0;
pub const LONG_START_SEQUENCE: core::ffi::c_int = 1;
pub const EIGHT_SHORT_SEQUENCE: core::ffi::c_int = 2 as core::ffi::c_int;
pub const LONG_STOP_SEQUENCE: core::ffi::c_int = 3;
pub const STOP_START_SEQUENCE: core::ffi::c_int = 4;
pub const MAX_32: WORD32 = 0x7fffffff as core::ffi::c_long as WORD32;
pub const MIN_32: WORD32 = 0x80000000 as core::ffi::c_long as WORD32;
unsafe extern "C" fn ixheaacd_calc_max_spectralline(
    mut p_in_ibuffer: *mut WORD32,
    mut n: WORD32,
) -> WORD32 {
    let mut k: WORD32 = 0;
    let mut shiftp: WORD32 = 0;
    let mut itemp: WORD32 = 0 as WORD32;
    k = 0 as core::ffi::c_int as WORD32;
    while k < n {
        if ixheaac_abs32_sat(*p_in_ibuffer.offset(k as isize)) > itemp {
            itemp = ixheaac_abs32_sat(*p_in_ibuffer.offset(k as isize));
        }
        k += 1;
    }
    shiftp = ixheaac_norm32(itemp) as WORD32;
    return shiftp;
}
unsafe extern "C" fn ixheaacd_normalize(
    mut buff: *mut WORD32,
    mut shift: WORD32,
    mut len: WORD,
) {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < len {
        *buff.offset(i as isize) = *buff.offset(i as isize) << shift;
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_pow10(mut input: WORD32) -> FLOAT32 {
    let mut output: FLOAT32 = 1 as core::ffi::c_int as FLOAT32;
    while input > 0 as core::ffi::c_int {
        output *= 10 as core::ffi::c_int as FLOAT32;
        input -= 1;
    }
    return output;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_calc_pre_twid_dec(
    mut ptr_x: *mut WORD32,
    mut r_ptr: *mut WORD32,
    mut i_ptr: *mut WORD32,
    mut nlength: WORD32,
    mut cos_ptr: *const WORD32,
    mut sin_ptr: *const WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut ptr_y: *mut WORD32 = 0 as *mut WORD32;
    ptr_y = &mut *ptr_x
        .offset(
            (2 as core::ffi::c_int * nlength as core::ffi::c_int - 1 as core::ffi::c_int)
                as isize,
        ) as *mut WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < nlength {
        let fresh0 = r_ptr;
        r_ptr = r_ptr.offset(1);
        *fresh0 = ixheaac_mult32(ixheaac_negate32_sat(*ptr_x), *cos_ptr)
            - ixheaac_mult32(*ptr_y, *sin_ptr);
        let fresh1 = cos_ptr;
        cos_ptr = cos_ptr.offset(1);
        let fresh2 = sin_ptr;
        sin_ptr = sin_ptr.offset(1);
        let fresh3 = i_ptr;
        i_ptr = i_ptr.offset(1);
        *fresh3 = ixheaac_mult32(*ptr_y, *fresh1) - ixheaac_mult32(*ptr_x, *fresh2);
        ptr_x = ptr_x.offset(2 as core::ffi::c_int as isize);
        ptr_y = ptr_y.offset(-(2 as core::ffi::c_int as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_calc_post_twid_dec(
    mut xptr: *mut WORD32,
    mut r_ptr: *mut WORD32,
    mut i_ptr: *mut WORD32,
    mut nlength: WORD32,
    mut cos_ptr: *const WORD32,
    mut sin_ptr: *const WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut yptr: *mut WORD32 = 0 as *mut WORD32;
    yptr = &mut *xptr
        .offset(
            (2 as core::ffi::c_int * nlength as core::ffi::c_int - 1 as core::ffi::c_int)
                as isize,
        ) as *mut WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < nlength {
        *xptr = -(ixheaac_mult32(*r_ptr.offset(i as isize), *cos_ptr)
            - ixheaac_mult32(*i_ptr.offset(i as isize), *sin_ptr));
        let fresh4 = cos_ptr;
        cos_ptr = cos_ptr.offset(1);
        let fresh5 = sin_ptr;
        sin_ptr = sin_ptr.offset(1);
        *yptr = -(ixheaac_mult32(*i_ptr.offset(i as isize), *fresh4)
            + ixheaac_mult32(*r_ptr.offset(i as isize), *fresh5));
        xptr = xptr.offset(2 as core::ffi::c_int as isize);
        yptr = yptr.offset(-(2 as core::ffi::c_int as isize));
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_fft_based_imdct(
    mut data: *mut WORD32,
    mut npoints: WORD32,
    mut preshift: *mut WORD32,
    mut tmp_data: *mut WORD32,
) -> VOID {
    let mut data_r: *mut WORD32 = 0 as *mut WORD32;
    let mut data_i: *mut WORD32 = 0 as *mut WORD32;
    let mut nlength: WORD32 = npoints >> 1 as core::ffi::c_int;
    let mut cos_ptr: *const WORD32 = 0 as *const WORD32;
    let mut sin_ptr: *const WORD32 = 0 as *const WORD32;
    data_r = tmp_data;
    data_i = tmp_data.offset(512 as core::ffi::c_int as isize);
    if nlength == 512 as core::ffi::c_int {
        cos_ptr = ixheaacd_pre_post_twid_cos_512.as_ptr();
        sin_ptr = ixheaacd_pre_post_twid_sin_512.as_ptr();
    } else if nlength == 384 as core::ffi::c_int {
        cos_ptr = ixheaacd_pre_post_twid_cos_384.as_ptr();
        sin_ptr = ixheaacd_pre_post_twid_sin_384.as_ptr();
    } else if nlength == 64 as core::ffi::c_int {
        cos_ptr = ixheaacd_pre_post_twid_cos_64.as_ptr();
        sin_ptr = ixheaacd_pre_post_twid_sin_64.as_ptr();
    } else if nlength == 48 as core::ffi::c_int {
        cos_ptr = ixheaacd_pre_post_twid_cos_48.as_ptr();
        sin_ptr = ixheaacd_pre_post_twid_sin_48.as_ptr();
    } else {
        cos_ptr = ixheaacd_pre_post_twid_cos_48.as_ptr();
        sin_ptr = ixheaacd_pre_post_twid_sin_48.as_ptr();
    }
    (Some(ixheaacd_calc_pre_twid.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(data, data_r, data_i, nlength, cos_ptr, sin_ptr);
    ixheaacd_complex_fft(data_r, data_i, nlength, 1 as WORD32, preshift);
    (Some(ixheaacd_calc_post_twid.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(data, data_r, data_i, nlength, cos_ptr, sin_ptr);
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_acelp_imdct(
    mut imdct_in: *mut WORD32,
    mut npoints: WORD32,
    mut qshift: *mut WORD8,
    mut tmp_data: *mut WORD32,
) -> VOID {
    let mut preshift: WORD32 = 0 as WORD32;
    let mut i: WORD32 = 0;
    let mut k: WORD32 = npoints / 2 as WORD32;
    while (k as core::ffi::c_int & 1 as core::ffi::c_int == 0 as core::ffi::c_int)
        as core::ffi::c_int & (k != 1 as core::ffi::c_int) as core::ffi::c_int != 0
    {
        k = k >> 1 as core::ffi::c_int;
        preshift += 1;
    }
    if k != 1 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < npoints as core::ffi::c_int / 2 as core::ffi::c_int {
            *imdct_in.offset(i as isize) = ((*imdct_in.offset(i as isize)
                / 3 as core::ffi::c_int) << 1 as core::ffi::c_int) as WORD32;
            i += 1;
        }
        preshift += 1;
    }
    ixheaacd_fft_based_imdct(imdct_in, npoints / 2 as WORD32, &mut preshift, tmp_data);
    preshift += 2 as core::ffi::c_int;
    *qshift = (*qshift as core::ffi::c_int - preshift as core::ffi::c_int) as WORD8;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_cal_fac_data(
    mut usac_data: *mut ia_usac_data_struct,
    mut i_ch: WORD32,
    mut n_long: WORD32,
    mut lfac: WORD32,
    mut fac_idata: *mut WORD32,
    mut q_fac: *mut WORD8,
) -> IA_ERRORCODE {
    let mut gain_fac: WORD32 = 0;
    let mut scale: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut i_aq: *mut WORD32 = 0 as *mut WORD32;
    let mut itemp: WORD32 = 0 as WORD32;
    let mut izir: *mut WORD32 = 0 as *mut WORD32;
    let mut int_aq: [WORD32; 17] = [0 as core::ffi::c_int; 17];
    let mut intzir: [WORD32; 512] = [0 as core::ffi::c_int; 512];
    let mut x_in: [WORD32; 128] = [0 as core::ffi::c_int; 128];
    let mut gain: FLOAT32 = 0.;
    let mut ztemp: FLOAT32 = 0.;
    let mut ftemp: FLOAT32 = 0.;
    let mut pow10: FLOAT32 = 0.;
    let mut rem10: FLOAT32 = 0.;
    let mut qfac1: FLOAT32 = 0.;
    let mut qshift1: WORD8 = 0 as WORD8;
    let mut qshift2: WORD8 = 0 as WORD8;
    let mut qshift3: WORD8 = 0 as WORD8;
    let mut preshift: WORD32 = 0 as WORD32;
    let mut last_lpc: *mut FLOAT32 = ((*usac_data).lpc_prev[i_ch as usize]).as_mut_ptr();
    let mut acelp_in: *mut FLOAT32 = ((*usac_data).acelp_in[i_ch as usize]).as_mut_ptr();
    let mut fac_data: *mut WORD32 = ((*usac_data).fac_data[i_ch as usize]).as_mut_ptr();
    let mut ptr_scratch: *mut WORD32 = &mut *((*usac_data).scratch_buffer)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
    let mut quo: WORD32 = *fac_data.offset(0 as core::ffi::c_int as isize)
        / 28 as WORD32;
    let mut rem: WORD32 = *fac_data.offset(0 as core::ffi::c_int as isize)
        % 28 as WORD32;
    pow10 = ixheaacd_pow10(quo);
    rem10 = ixheaacd_power_10_table[rem as usize] as FLOAT32;
    gain = pow10 * rem10;
    scale = ixheaac_norm32(
        ((if gain < 0 as core::ffi::c_int as FLOAT32 { -gain } else { gain })
            + 1 as core::ffi::c_int as FLOAT32) as WORD32,
    );
    gain_fac = (gain * ((1 as core::ffi::c_int as WORD64) << scale) as FLOAT32)
        as WORD32;
    scale += 4 as core::ffi::c_int;
    qfac1 = 1.0f32 / gain;
    if !acelp_in.is_null() {
        izir = intzir.as_mut_ptr();
        ftemp = 0.0f32;
        k = 0 as core::ffi::c_int as WORD32;
        while k < n_long as core::ffi::c_int / 4 as core::ffi::c_int {
            ztemp = *acelp_in.offset(k as isize) * qfac1;
            if (if ztemp < 0 as core::ffi::c_int as FLOAT32 { -ztemp } else { ztemp })
                > ftemp
            {
                ftemp = if ztemp < 0 as core::ffi::c_int as FLOAT32 {
                    -ztemp
                } else {
                    ztemp
                };
            }
            k += 1;
        }
        itemp = ftemp as WORD32;
        qshift3 = ixheaac_norm32(itemp) as WORD8;
        k = 0 as core::ffi::c_int as WORD32;
        while k < n_long as core::ffi::c_int / 4 as core::ffi::c_int {
            *izir.offset(k as isize) = (*acelp_in.offset(k as isize) * qfac1
                * ((1 as core::ffi::c_int as WORD64) << qshift3 as core::ffi::c_int)
                    as FLOAT32) as WORD32;
            k += 1;
        }
    } else {
        izir = 0 as *mut WORD32;
    }
    if !last_lpc.is_null() {
        ftemp = 0.0f32;
        i_aq = int_aq.as_mut_ptr();
        k = 0 as core::ffi::c_int as WORD32;
        while k < ORDER + 1 as core::ffi::c_int {
            if (if *last_lpc.offset(k as isize) < 0 as core::ffi::c_int as FLOAT32 {
                -*last_lpc.offset(k as isize)
            } else {
                *last_lpc.offset(k as isize)
            }) > ftemp
            {
                ftemp = if *last_lpc.offset(k as isize)
                    < 0 as core::ffi::c_int as FLOAT32
                {
                    -*last_lpc.offset(k as isize)
                } else {
                    *last_lpc.offset(k as isize)
                };
            }
            k += 1;
        }
        itemp = ftemp as WORD32;
        qshift2 = ixheaac_norm32(itemp) as WORD8;
        k = 0 as core::ffi::c_int as WORD32;
        while k < ORDER + 1 as core::ffi::c_int {
            *i_aq.offset(k as isize) = (*last_lpc.offset(k as isize)
                * ((1 as core::ffi::c_int as WORD64) << qshift2 as core::ffi::c_int)
                    as FLOAT32) as WORD32;
            k += 1;
        }
    } else {
        i_aq = 0 as *mut WORD32;
    }
    k = 0 as core::ffi::c_int as WORD32;
    while k < lfac {
        if ixheaac_abs32_sat(
            *fac_data.offset((k as core::ffi::c_int + 1 as core::ffi::c_int) as isize),
        ) > itemp
        {
            itemp = ixheaac_abs32_sat(
                *fac_data
                    .offset((k as core::ffi::c_int + 1 as core::ffi::c_int) as isize),
            );
        }
        k += 1;
    }
    qshift1 = ixheaac_norm32(itemp) as WORD8;
    k = 0 as core::ffi::c_int as WORD32;
    while k < lfac {
        *fac_data.offset((k as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = (*fac_data
            .offset((k as core::ffi::c_int + 1 as core::ffi::c_int) as isize) as FLOAT32
            * ((1 as core::ffi::c_int as WORD64) << qshift1 as core::ffi::c_int)
                as FLOAT32) as WORD32;
        k += 1;
    }
    k = 0 as core::ffi::c_int as WORD32;
    while k < lfac as core::ffi::c_int / 2 as core::ffi::c_int {
        x_in[k as usize] = *fac_data
            .offset(
                (2 as core::ffi::c_int * k as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            );
        x_in[(lfac / 2 as WORD32 + k) as usize] = *fac_data
            .offset((lfac - 2 as WORD32 * k) as isize);
        k += 1;
    }
    if FAC_LENGTH < lfac {
        if (*usac_data).ec_flag == 0 as core::ffi::c_int {
            return -(1 as IA_ERRORCODE)
        } else {
            lfac = FAC_LENGTH as WORD32;
        }
    }
    if (n_long as core::ffi::c_int / 8 as core::ffi::c_int) < lfac {
        if (*usac_data).ec_flag == 0 as core::ffi::c_int {
            return -(1 as IA_ERRORCODE)
        } else {
            lfac = (n_long as core::ffi::c_int / 8 as core::ffi::c_int) as WORD32;
        }
    }
    if n_long as core::ffi::c_int / 8 as core::ffi::c_int + 1 as core::ffi::c_int
        > 2 as core::ffi::c_int * LEN_FRAME - lfac as core::ffi::c_int
            - 1 as core::ffi::c_int
    {
        if (*usac_data).ec_flag == 0 as core::ffi::c_int {
            return -(1 as IA_ERRORCODE)
        } else {
            lfac = (2 as core::ffi::c_int * LEN_FRAME
                - n_long as core::ffi::c_int / 8 as core::ffi::c_int) as WORD32;
        }
    }
    if lfac as core::ffi::c_int & lfac as core::ffi::c_int - 1 as core::ffi::c_int != 0 {
        if lfac != 48 as core::ffi::c_int && lfac != 96 as core::ffi::c_int
            && lfac != 192 as core::ffi::c_int && lfac != 384 as core::ffi::c_int
            && lfac != 768 as core::ffi::c_int
        {
            if (*usac_data).ec_flag == 0 as core::ffi::c_int {
                return -(1 as IA_ERRORCODE)
            } else {
                lfac = 48 as core::ffi::c_int as WORD32;
            }
        }
    }
    ixheaacd_fr_alias_cnx_fix(
        x_in.as_mut_ptr(),
        n_long / 4 as WORD32,
        lfac,
        i_aq,
        izir,
        fac_idata.offset(16 as core::ffi::c_int as isize),
        &mut qshift1,
        qshift2,
        qshift3,
        &mut preshift,
        ptr_scratch,
    );
    preshift += 4 as core::ffi::c_int;
    *q_fac = (qshift1 as WORD32 - preshift) as WORD8;
    if !acelp_in.is_null() {
        k = 0 as core::ffi::c_int as WORD32;
        while k < 2 as WORD32 * lfac {
            *fac_idata.offset(k as isize) = ixheaac_mul32_sh(
                *fac_idata
                    .offset((k as core::ffi::c_int + 16 as core::ffi::c_int) as isize),
                gain_fac,
                scale as WORD8,
            );
            k += 1;
        }
    }
    return IA_NO_ERROR;
}
unsafe extern "C" fn ixheaacd_fd_imdct_short(
    mut usac_data: *mut ia_usac_data_struct,
    mut i_ch: WORD32,
    mut fac_data_out: *mut WORD32,
    mut ixheaacd_drc_offset: *mut offset_lengths,
    mut fac_q: WORD8,
) -> IA_ERRORCODE {
    let mut qfac: FLOAT32 = 0.;
    let mut overlap_data_buf: [WORD32; 2048] = [0 as core::ffi::c_int; 2048];
    let mut window_short: *mut WORD32 = 0 as *mut WORD32;
    let mut k: WORD32 = 0;
    let mut window_short_prev_ptr: *mut WORD32 = 0 as *mut WORD32;
    let mut overlap_data: *mut WORD32 = 0 as *mut WORD32;
    let mut fp: *mut WORD32 = 0 as *mut WORD32;
    let mut p_overlap_ibuffer: *mut WORD32 = ((*usac_data)
        .overlap_data_ptr[i_ch as usize])
        .as_mut_ptr();
    let mut p_in_ibuffer: *mut WORD32 = (*usac_data).coef_fix[i_ch as usize];
    let mut p_out_buffer: *mut FLOAT32 = ((*usac_data).time_sample_vector[i_ch as usize])
        .as_mut_ptr();
    let mut p_out_ibuffer: *mut WORD32 = ((*usac_data).output_data_ptr[i_ch as usize])
        .as_mut_ptr();
    let mut scratch_mem: *mut WORD32 = ((*usac_data).scratch_buffer).as_mut_ptr();
    let mut td_frame_prev: WORD32 = (*usac_data).td_frame_prev[i_ch as usize];
    let mut fac_apply: WORD32 = (*usac_data).fac_data_present[i_ch as usize];
    let mut shiftp: WORD8 = 0;
    let mut input_q: WORD8 = 0;
    let mut output_q: WORD8 = 0;
    let mut shift_olap: WORD8 = 14 as WORD8;
    let mut max_shift: WORD32 = 0;
    let mut window_select: WORD32 = (*usac_data).window_shape[i_ch as usize];
    let mut window_select_prev: WORD32 = (*usac_data).window_shape_prev[i_ch as usize];
    let mut st: ia_usac_lpd_decoder_handle = (*usac_data).str_tddec[i_ch as usize];
    let mut err_code: WORD32 = 0 as WORD32;
    if (*usac_data).ec_flag != 0 {
        td_frame_prev = (*usac_data).td_frame_prev_ec[i_ch as usize];
    } else if (*ixheaacd_drc_offset).n_short as core::ffi::c_int
        & (*ixheaacd_drc_offset).n_short as core::ffi::c_int - 1 as core::ffi::c_int != 0
    {
        if (*ixheaacd_drc_offset).n_short != 48 as core::ffi::c_int
            && (*ixheaacd_drc_offset).n_short != 96 as core::ffi::c_int
            && (*ixheaacd_drc_offset).n_short != 192 as core::ffi::c_int
            && (*ixheaacd_drc_offset).n_short != 384 as core::ffi::c_int
            && (*ixheaacd_drc_offset).n_short != 768 as core::ffi::c_int
        {
            return -(1 as IA_ERRORCODE);
        }
    }
    max_shift = ixheaacd_calc_max_spectralline(
        p_in_ibuffer,
        (*ixheaacd_drc_offset).n_long,
    );
    ixheaacd_normalize(p_in_ibuffer, max_shift, (*ixheaacd_drc_offset).n_long as WORD);
    shiftp = (max_shift as core::ffi::c_int + 6 as core::ffi::c_int) as WORD8;
    input_q = shiftp;
    memcpy(
        overlap_data_buf.as_mut_ptr() as *mut core::ffi::c_void,
        p_overlap_ibuffer as *const core::ffi::c_void,
        (::core::mem::size_of::<WORD32>() as size_t)
            .wrapping_mul((*ixheaacd_drc_offset).n_long as size_t),
    );
    overlap_data = overlap_data_buf.as_mut_ptr();
    fp = overlap_data.offset((*ixheaacd_drc_offset).n_flat_ls as isize);
    k = 0 as core::ffi::c_int as WORD32;
    while k < 8 as core::ffi::c_int {
        shiftp = input_q;
        ixheaacd_acelp_imdct(
            p_in_ibuffer.offset((k * (*ixheaacd_drc_offset).n_short) as isize),
            2 as WORD32 * (*ixheaacd_drc_offset).n_short,
            &mut shiftp,
            scratch_mem,
        );
        k += 1;
    }
    max_shift = ixheaacd_calc_max_spectralline(
        p_in_ibuffer,
        (*ixheaacd_drc_offset).n_long,
    );
    ixheaacd_normalize(
        p_in_ibuffer,
        max_shift - 1 as WORD32,
        (*ixheaacd_drc_offset).n_long as WORD,
    );
    shiftp = (shiftp as core::ffi::c_int
        + (max_shift as core::ffi::c_int - 1 as core::ffi::c_int)) as WORD8;
    if shiftp as core::ffi::c_int - shift_olap as core::ffi::c_int
        > 31 as core::ffi::c_int
    {
        shiftp = (31 as core::ffi::c_int + shift_olap as core::ffi::c_int) as WORD8;
    }
    err_code = ixheaacd_calc_window(
        &mut window_short,
        (*ixheaacd_drc_offset).n_short,
        window_select,
        (*usac_data).ec_flag,
    );
    if err_code == -(1 as core::ffi::c_int) {
        return err_code as IA_ERRORCODE;
    }
    err_code = ixheaacd_calc_window(
        &mut window_short_prev_ptr,
        (*ixheaacd_drc_offset).n_trans_ls,
        window_select_prev,
        (*usac_data).ec_flag,
    );
    if err_code == -(1 as core::ffi::c_int) {
        return err_code as IA_ERRORCODE;
    }
    if fac_apply != 0 {
        ixheaacd_windowing_short1(
            p_in_ibuffer
                .offset(
                    ((*ixheaacd_drc_offset).n_short as core::ffi::c_int
                        / 2 as core::ffi::c_int) as isize,
                ),
            window_short_prev_ptr,
            fp,
            ixheaacd_drc_offset,
            shiftp,
            shift_olap,
        );
    } else {
        ixheaacd_windowing_short2(
            p_in_ibuffer
                .offset(
                    ((*ixheaacd_drc_offset).n_short as core::ffi::c_int
                        / 2 as core::ffi::c_int) as isize,
                ),
            window_short_prev_ptr,
            fp,
            ixheaacd_drc_offset,
            shiftp,
            shift_olap,
        );
    }
    output_q = ixheaacd_windowing_short3(
        p_in_ibuffer,
        window_short
            .offset((*ixheaacd_drc_offset).n_short as isize)
            .offset(-(1 as core::ffi::c_int as isize)),
        fp.offset((*ixheaacd_drc_offset).n_short as isize),
        (*ixheaacd_drc_offset).n_short,
        shiftp,
        shift_olap,
    );
    p_in_ibuffer = p_in_ibuffer.offset((*ixheaacd_drc_offset).n_short as isize);
    fp = fp.offset((*ixheaacd_drc_offset).n_short as isize);
    window_short_prev_ptr = window_short;
    k = 1 as core::ffi::c_int as WORD32;
    while k < 7 as core::ffi::c_int {
        output_q = ixheaacd_windowing_short4(
            p_in_ibuffer,
            window_short_prev_ptr,
            fp,
            window_short_prev_ptr
                .offset((*ixheaacd_drc_offset).n_short as isize)
                .offset(-(1 as core::ffi::c_int as isize)),
            (*ixheaacd_drc_offset).n_short,
            1 as WORD32,
            shiftp,
            shift_olap,
            output_q,
        );
        p_in_ibuffer = p_in_ibuffer.offset((*ixheaacd_drc_offset).n_short as isize);
        fp = fp.offset((*ixheaacd_drc_offset).n_short as isize);
        window_short_prev_ptr = window_short;
        k += 1;
    }
    output_q = ixheaacd_windowing_short4(
        p_in_ibuffer,
        window_short_prev_ptr,
        fp,
        window_short_prev_ptr
            .offset((*ixheaacd_drc_offset).n_short as isize)
            .offset(-(1 as core::ffi::c_int as isize)),
        (*ixheaacd_drc_offset).n_short,
        0 as WORD32,
        shiftp,
        shift_olap,
        output_q,
    );
    p_in_ibuffer = p_in_ibuffer.offset((*ixheaacd_drc_offset).n_short as isize);
    fp = fp.offset((*ixheaacd_drc_offset).n_short as isize);
    if fac_apply != 0 {
        ixheaacd_combine_fac(
            overlap_data
                .offset((*ixheaacd_drc_offset).n_flat_ls as isize)
                .offset((*ixheaacd_drc_offset).lfac as isize),
            fac_data_out,
            overlap_data
                .offset((*ixheaacd_drc_offset).n_flat_ls as isize)
                .offset((*ixheaacd_drc_offset).lfac as isize),
            2 as WORD32 * (*ixheaacd_drc_offset).lfac,
            output_q,
            fac_q,
        );
    }
    memset(
        overlap_data
            .offset((2 as WORD32 * (*ixheaacd_drc_offset).n_long) as isize)
            .offset(-((*ixheaacd_drc_offset).n_flat_ls as isize))
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<WORD32>() as size_t)
            .wrapping_mul((*ixheaacd_drc_offset).n_flat_ls as size_t),
    );
    ixheaacd_scale_down(
        overlap_data,
        overlap_data,
        (*ixheaacd_drc_offset).n_flat_ls,
        shift_olap,
        output_q,
    );
    ixheaacd_scale_down(
        p_overlap_ibuffer,
        overlap_data.offset((*ixheaacd_drc_offset).n_long as isize),
        (*ixheaacd_drc_offset).n_long,
        output_q,
        shift_olap,
    );
    ixheaacd_scale_down(
        p_out_ibuffer,
        overlap_data,
        (*ixheaacd_drc_offset).n_long,
        output_q,
        15 as WORD8,
    );
    if td_frame_prev != 0 {
        qfac = 1.0f32 / ((1 as core::ffi::c_int) << 15 as core::ffi::c_int) as FLOAT32;
        k = 0 as core::ffi::c_int as WORD32;
        while k < (*ixheaacd_drc_offset).n_long {
            *p_out_buffer.offset(k as isize) = *p_out_ibuffer.offset(k as isize)
                as FLOAT32 * qfac;
            k += 1;
        }
        err_code = ixheaacd_lpd_bpf_fix(
            usac_data,
            1 as WORD32,
            p_out_buffer as *mut FLOAT32,
            st,
        );
        if err_code != 0 as core::ffi::c_int {
            return err_code as IA_ERRORCODE;
        }
        k = 0 as core::ffi::c_int as WORD32;
        while k < (*ixheaacd_drc_offset).n_long {
            *p_out_ibuffer.offset(k as isize) = (*p_out_buffer.offset(k as isize)
                * ((1 as core::ffi::c_int) << 15 as core::ffi::c_int) as FLOAT32)
                as WORD32;
            k += 1;
        }
    }
    return 0 as IA_ERRORCODE;
}
unsafe extern "C" fn ixheaacd_fd_imdct_long(
    mut usac_data: *mut ia_usac_data_struct,
    mut i_ch: WORD32,
    mut fac_idata: *mut WORD32,
    mut ixheaacd_drc_offset: *mut offset_lengths,
    mut fac_q: WORD8,
) -> IA_ERRORCODE {
    let mut qfac: FLOAT32 = 0.;
    let mut window_long_prev: *mut WORD32 = 0 as *mut WORD32;
    let mut k: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut window_short_prev_ptr: *mut WORD32 = 0 as *mut WORD32;
    let mut p_in_ibuffer: *mut WORD32 = (*usac_data).coef_fix[i_ch as usize];
    let mut p_overlap_ibuffer: *mut WORD32 = ((*usac_data)
        .overlap_data_ptr[i_ch as usize])
        .as_mut_ptr();
    let mut p_out_ibuffer: *mut WORD32 = ((*usac_data).output_data_ptr[i_ch as usize])
        .as_mut_ptr();
    let mut p_out_buffer: *mut FLOAT32 = ((*usac_data).time_sample_vector[i_ch as usize])
        .as_mut_ptr();
    let mut scratch_mem: *mut WORD32 = ((*usac_data).scratch_buffer).as_mut_ptr();
    let mut n_long: WORD32 = (*usac_data).ccfl;
    let mut td_frame_prev: WORD32 = (*usac_data).td_frame_prev[i_ch as usize];
    let mut fac_apply: WORD32 = (*usac_data).fac_data_present[i_ch as usize];
    let mut shiftp: WORD8 = 0;
    let mut output_q: WORD8 = 0 as WORD8;
    let mut shift_olap: WORD8 = 14 as WORD8;
    let mut max_shift: WORD32 = 0;
    let mut window_sequence: WORD32 = (*usac_data).window_sequence[i_ch as usize];
    let mut window_select_prev: WORD32 = (*usac_data).window_shape_prev[i_ch as usize];
    let mut st: ia_usac_lpd_decoder_handle = (*usac_data).str_tddec[i_ch as usize];
    let mut err_code: WORD32 = 0 as WORD32;
    if (*usac_data).ec_flag != 0 {
        td_frame_prev = (*usac_data).td_frame_prev_ec[i_ch as usize];
    } else if (*ixheaacd_drc_offset).n_long as core::ffi::c_int
        & (*ixheaacd_drc_offset).n_long as core::ffi::c_int - 1 as core::ffi::c_int != 0
    {
        if (*ixheaacd_drc_offset).n_long != 48 as core::ffi::c_int
            && (*ixheaacd_drc_offset).n_long != 96 as core::ffi::c_int
            && (*ixheaacd_drc_offset).n_long != 192 as core::ffi::c_int
            && (*ixheaacd_drc_offset).n_long != 384 as core::ffi::c_int
            && (*ixheaacd_drc_offset).n_long != 768 as core::ffi::c_int
        {
            return -(1 as IA_ERRORCODE);
        }
    }
    max_shift = ixheaacd_calc_max_spectralline(
        p_in_ibuffer,
        (*ixheaacd_drc_offset).n_long,
    );
    ixheaacd_normalize(p_in_ibuffer, max_shift, (*ixheaacd_drc_offset).n_long as WORD);
    shiftp = (max_shift as core::ffi::c_int + 6 as core::ffi::c_int) as WORD8;
    ixheaacd_acelp_imdct(
        p_in_ibuffer,
        2 as WORD32 * (*ixheaacd_drc_offset).n_long,
        &mut shiftp,
        scratch_mem,
    );
    max_shift = ixheaacd_calc_max_spectralline(
        p_in_ibuffer,
        (*ixheaacd_drc_offset).n_long,
    );
    ixheaacd_normalize(
        p_in_ibuffer,
        max_shift - 1 as WORD32,
        (*ixheaacd_drc_offset).n_long as WORD,
    );
    shiftp = (shiftp as core::ffi::c_int
        + (max_shift as core::ffi::c_int - 1 as core::ffi::c_int)) as WORD8;
    if shiftp as core::ffi::c_int - shift_olap as core::ffi::c_int
        > 31 as core::ffi::c_int
    {
        shiftp = (31 as core::ffi::c_int + shift_olap as core::ffi::c_int) as WORD8;
    }
    match window_sequence {
        ONLY_LONG_SEQUENCE | LONG_START_SEQUENCE => {
            err_code = ixheaacd_calc_window(
                &mut window_long_prev,
                (*ixheaacd_drc_offset).n_long,
                window_select_prev,
                (*usac_data).ec_flag,
            );
            if err_code == -(1 as core::ffi::c_int) {
                return err_code as IA_ERRORCODE;
            }
            output_q = ixheaacd_windowing_long1(
                p_in_ibuffer
                    .offset(
                        (n_long as core::ffi::c_int / 2 as core::ffi::c_int) as isize,
                    ),
                p_overlap_ibuffer,
                window_long_prev,
                window_long_prev
                    .offset((*ixheaacd_drc_offset).n_long as isize)
                    .offset(-(1 as core::ffi::c_int as isize)),
                p_out_ibuffer,
                (*ixheaacd_drc_offset).n_long,
                shiftp,
                shift_olap,
            );
        }
        STOP_START_SEQUENCE | LONG_STOP_SEQUENCE => {
            err_code = ixheaacd_calc_window(
                &mut window_short_prev_ptr,
                (*ixheaacd_drc_offset).n_trans_ls,
                window_select_prev,
                (*usac_data).ec_flag,
            );
            if err_code == -(1 as core::ffi::c_int) {
                return err_code as IA_ERRORCODE;
            }
            if fac_apply != 0 {
                output_q = ixheaacd_windowing_long2(
                    p_in_ibuffer
                        .offset(
                            (n_long as core::ffi::c_int / 2 as core::ffi::c_int) as isize,
                        ),
                    window_short_prev_ptr,
                    fac_idata,
                    p_overlap_ibuffer,
                    p_out_ibuffer,
                    ixheaacd_drc_offset,
                    shiftp,
                    shift_olap,
                    fac_q,
                );
            } else {
                output_q = ixheaacd_windowing_long3(
                    p_in_ibuffer
                        .offset(
                            (n_long as core::ffi::c_int / 2 as core::ffi::c_int) as isize,
                        ),
                    window_short_prev_ptr,
                    p_overlap_ibuffer,
                    p_out_ibuffer,
                    window_short_prev_ptr
                        .offset((*ixheaacd_drc_offset).n_trans_ls as isize)
                        .offset(-(1 as core::ffi::c_int as isize)),
                    ixheaacd_drc_offset,
                    shiftp,
                    shift_olap,
                );
            }
        }
        _ => {}
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*ixheaacd_drc_offset).n_long as core::ffi::c_int / 2 as core::ffi::c_int {
        if shiftp as core::ffi::c_int > shift_olap as core::ffi::c_int {
            *p_overlap_ibuffer
                .offset(((*ixheaacd_drc_offset).n_long / 2 as WORD32 + i) as isize) = ixheaac_negate32_sat(
                *p_in_ibuffer.offset(i as isize),
            ) >> shiftp as core::ffi::c_int - shift_olap as core::ffi::c_int;
            *p_overlap_ibuffer
                .offset(
                    ((*ixheaacd_drc_offset).n_long as core::ffi::c_int
                        / 2 as core::ffi::c_int - i as core::ffi::c_int
                        - 1 as core::ffi::c_int) as isize,
                ) = ixheaac_negate32_sat(*p_in_ibuffer.offset(i as isize))
                >> shiftp as core::ffi::c_int - shift_olap as core::ffi::c_int;
        } else {
            *p_overlap_ibuffer
                .offset(((*ixheaacd_drc_offset).n_long / 2 as WORD32 + i) as isize) = ixheaac_negate32_sat(
                *p_in_ibuffer.offset(i as isize),
            ) >> shift_olap as core::ffi::c_int - shiftp as core::ffi::c_int;
            *p_overlap_ibuffer
                .offset(
                    ((*ixheaacd_drc_offset).n_long as core::ffi::c_int
                        / 2 as core::ffi::c_int - i as core::ffi::c_int
                        - 1 as core::ffi::c_int) as isize,
                ) = ixheaac_negate32_sat(*p_in_ibuffer.offset(i as isize))
                >> shift_olap as core::ffi::c_int - shiftp as core::ffi::c_int;
        }
        i += 1;
    }
    ixheaacd_scale_down_adj(
        p_out_ibuffer,
        p_out_ibuffer,
        (*ixheaacd_drc_offset).n_long,
        output_q,
        15 as WORD8,
    );
    if td_frame_prev != 0 {
        qfac = 1.0f32 / ((1 as core::ffi::c_int) << 15 as core::ffi::c_int) as FLOAT32;
        k = 0 as core::ffi::c_int as WORD32;
        while k < (*ixheaacd_drc_offset).n_long {
            *p_out_buffer.offset(k as isize) = *p_out_ibuffer.offset(k as isize)
                as FLOAT32 * qfac;
            k += 1;
        }
        err_code = ixheaacd_lpd_bpf_fix(
            usac_data,
            0 as WORD32,
            p_out_buffer as *mut FLOAT32,
            st,
        );
        if err_code != 0 as core::ffi::c_int {
            return err_code as IA_ERRORCODE;
        }
        k = 0 as core::ffi::c_int as WORD32;
        while k < (*ixheaacd_drc_offset).n_long {
            *p_out_ibuffer.offset(k as isize) = (*p_out_buffer.offset(k as isize)
                * ((1 as core::ffi::c_int) << 15 as core::ffi::c_int) as FLOAT32)
                as WORD32;
            k += 1;
        }
    }
    return 0 as IA_ERRORCODE;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_fd_frm_dec(
    mut usac_data: *mut ia_usac_data_struct,
    mut i_ch: WORD32,
) -> WORD32 {
    let mut fac_idata: [WORD32; 272] = [0; 272];
    let mut ixheaacd_drc_offset: offset_lengths = offset_lengths {
        lfac: 0,
        n_flat_ls: 0,
        n_trans_ls: 0,
        n_long: 0,
        n_short: 0,
    };
    let mut fac_q: WORD8 = 0 as WORD8;
    let mut err: IA_ERRORCODE = IA_NO_ERROR;
    let mut td_frame_prev: WORD32 = 0;
    let mut fac_apply: WORD32 = 0;
    let mut window_sequence: WORD32 = 0;
    if (*usac_data).ec_flag != 0 {
        (*usac_data).str_error_concealment[i_ch as usize].pstr_ec_scratch = &mut (*((*usac_data)
            .str_error_concealment)
            .as_mut_ptr()
            .offset(i_ch as isize))
            .str_ec_scratch as *mut ia_ec_scratch_str;
        (*usac_data).core_mode = 0 as core::ffi::c_int as WORD32;
        ixheaacd_usac_apply_ec(
            usac_data,
            &*ixheaacd_samp_rate_info.as_ptr().offset(0 as core::ffi::c_int as isize),
            i_ch,
        );
    }
    if (*usac_data).ec_flag != 0 {
        td_frame_prev = (*usac_data).td_frame_prev_ec[i_ch as usize];
    } else {
        td_frame_prev = (*usac_data).td_frame_prev[i_ch as usize];
    }
    fac_apply = (*usac_data).fac_data_present[i_ch as usize];
    window_sequence = (*usac_data).window_sequence[i_ch as usize];
    ixheaacd_drc_offset.n_long = (*usac_data).ccfl;
    ixheaacd_drc_offset.n_short = ixheaacd_drc_offset.n_long >> 3 as core::ffi::c_int;
    memset(
        fac_idata.as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[WORD32; 272]>() as size_t,
    );
    if td_frame_prev != 0 {
        if window_sequence == EIGHT_SHORT_SEQUENCE {
            ixheaacd_drc_offset.lfac = ixheaacd_drc_offset.n_long
                >> 4 as core::ffi::c_int;
        } else {
            ixheaacd_drc_offset.lfac = ixheaacd_drc_offset.n_long
                >> 3 as core::ffi::c_int;
        }
        ixheaacd_drc_offset.n_flat_ls = (ixheaacd_drc_offset.n_long as core::ffi::c_int
            - ixheaacd_drc_offset.lfac as core::ffi::c_int * 2 as core::ffi::c_int
            >> 1 as core::ffi::c_int) as WORD32;
        ixheaacd_drc_offset.n_trans_ls = ixheaacd_drc_offset.lfac
            << 1 as core::ffi::c_int;
    } else {
        ixheaacd_drc_offset.lfac = FAC_LENGTH as WORD32;
        ixheaacd_drc_offset.n_flat_ls = ixheaacd_drc_offset.n_long
            - ixheaacd_drc_offset.n_short >> 1 as core::ffi::c_int;
        ixheaacd_drc_offset.n_trans_ls = ixheaacd_drc_offset.n_short;
    }
    if fac_apply != 0 && (*usac_data).frame_ok == 1 as core::ffi::c_int {
        err = ixheaacd_cal_fac_data(
            usac_data,
            i_ch,
            ixheaacd_drc_offset.n_long,
            ixheaacd_drc_offset.lfac,
            fac_idata.as_mut_ptr(),
            &mut fac_q,
        );
        if err != 0 {
            return err as WORD32;
        }
    }
    if window_sequence != EIGHT_SHORT_SEQUENCE {
        err = ixheaacd_fd_imdct_long(
            usac_data,
            i_ch,
            fac_idata.as_mut_ptr(),
            &mut ixheaacd_drc_offset,
            fac_q,
        );
        if err != 0 {
            return err as WORD32;
        }
    } else {
        err = ixheaacd_fd_imdct_short(
            usac_data,
            i_ch,
            fac_idata.as_mut_ptr(),
            &mut ixheaacd_drc_offset,
            fac_q,
        );
        if err != 0 {
            return err as WORD32;
        }
    }
    return err as WORD32;
}
pub const IA_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
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
unsafe extern "C" fn ixheaac_negate32_sat(mut a: WORD32) -> WORD32 {
    let mut neg_val: WORD32 = 0;
    if a == MIN_32 {
        neg_val = MAX_32;
    } else {
        neg_val = -a;
    }
    return neg_val;
}
#[inline]
unsafe extern "C" fn ixheaac_mult32(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 32 as core::ffi::c_int) as WORD32;
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
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
