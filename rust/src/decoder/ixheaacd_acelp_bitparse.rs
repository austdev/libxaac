extern "C" {
    pub type ia_sbr_dec_inst_struct;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn ixheaacd_rotated_gosset_mtx_dec(
        qn: WORD32,
        code_book_idx: WORD32,
        kv: *mut WORD32,
        y: *mut WORD32,
    ) -> VOID;
    fn _setjmp(__env: *mut __jmp_buf_tag) -> core::ffi::c_int;
    fn longjmp(__env: *mut __jmp_buf_tag, __val: core::ffi::c_int) -> !;
    fn ixheaacd_read_bits_buf(
        it_bit_buff: *mut ia_bit_buf_struct,
        no_of_bits: WORD,
    ) -> WORD32;
    fn ixheaacd_reset_acelp_data_fix(
        usac_data: *mut ia_usac_data_struct,
        st: ia_usac_lpd_decoder_handle,
        ptr_ola_buff: *mut WORD32,
        last_was_short: WORD32,
        tw_mdct: WORD32,
    ) -> VOID;
    fn ixheaacd_usac_apply_ec(
        pstr_usac_data: *mut ia_usac_data_struct,
        pstr_samp_rate_info: *const ia_usac_samp_rate_info,
        ch: WORD32,
    ) -> VOID;
    fn ixheaacd_arith_data(
        pstr_td_frame_data: *mut ia_td_frame_data_struct,
        quant: *mut WORD32,
        usac_data: *mut ia_usac_data_struct,
        it_bit_buff: *mut ia_bit_buf_struct,
        first_tcx_flag: WORD32,
        k: WORD32,
    ) -> VOID;
    fn ixheaacd_lpd_dec(
        usac_data: *mut ia_usac_data_struct,
        st: ia_usac_lpd_decoder_handle,
        pstr_td_frame_data: *mut ia_td_frame_data_struct,
        fsynth: *mut FLOAT32,
        first_lpd_flag: WORD32,
        short_fac_flag: WORD32,
        bpf_control_info: WORD32,
    ) -> WORD32;
    static ixheaacd_samp_rate_info: [ia_usac_samp_rate_info; 0];
}
pub type size_t = usize;
pub type WORD8 = core::ffi::c_schar;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD16 = core::ffi::c_short;
pub type pWORD16 = *mut core::ffi::c_short;
pub type UWORD16 = core::ffi::c_ushort;
pub type WORD32 = core::ffi::c_int;
pub type pWORD32 = *mut core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type WORD64 = core::ffi::c_longlong;
pub type FLOAT32 = core::ffi::c_float;
pub type VOID = ();
pub type WORD = core::ffi::c_int;
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
pub const FAC_LENGTH: core::ffi::c_int = 128 as core::ffi::c_int;
pub const EIGHT_SHORT_SEQUENCE: core::ffi::c_int = 2 as core::ffi::c_int;
pub const LEN_SUBFR: core::ffi::c_int = 64 as core::ffi::c_int;
pub const MAX_32: WORD32 = 0x7fffffff as core::ffi::c_long as WORD32;
pub const MIN_32: WORD32 = 0x80000000 as core::ffi::c_long as WORD32;
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
pub const IA_FATAL_ERROR: core::ffi::c_uint = 0x80000000 as core::ffi::c_uint;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES: core::ffi::c_int = 0x1804
    as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_get_mode_lpc(
    mut lpc_set: WORD32,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut nk_mode: *mut WORD32,
) -> WORD32 {
    let mut mode_lpc: WORD32 = 0 as WORD32;
    match lpc_set {
        4 => {
            mode_lpc = 0 as core::ffi::c_int as WORD32;
        }
        0 | 2 => {
            mode_lpc = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
            if mode_lpc == 1 as core::ffi::c_int {
                *nk_mode = 3 as core::ffi::c_int as WORD32;
            }
        }
        1 => {
            if ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD) == 0 as core::ffi::c_int {
                *nk_mode = 2 as core::ffi::c_int as WORD32;
                mode_lpc = *nk_mode;
            } else if ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD)
                == 0 as core::ffi::c_int
            {
                *nk_mode = 0 as core::ffi::c_int as WORD32;
                mode_lpc = *nk_mode;
            } else {
                *nk_mode = 1 as core::ffi::c_int as WORD32;
                mode_lpc = *nk_mode;
            }
        }
        3 => {
            if ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD) == 0 as core::ffi::c_int {
                *nk_mode = 1 as core::ffi::c_int as WORD32;
                mode_lpc = *nk_mode;
            } else if ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD)
                == 0 as core::ffi::c_int
            {
                *nk_mode = 0 as core::ffi::c_int as WORD32;
                mode_lpc = *nk_mode;
            } else {
                if ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD)
                    == 0 as core::ffi::c_int
                {
                    mode_lpc = 2 as core::ffi::c_int as WORD32;
                } else {
                    mode_lpc = 3 as core::ffi::c_int as WORD32;
                }
                *nk_mode = 2 as core::ffi::c_int as WORD32;
            }
        }
        _ => {}
    }
    return mode_lpc;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_qn_data(
    mut nk_mode: WORD32,
    mut qn: *mut WORD32,
    mut it_bit_buff: *mut ia_bit_buf_struct,
) -> VOID {
    let mut k: WORD32 = 0;
    match nk_mode {
        1 => {
            k = 0 as core::ffi::c_int as WORD32;
            while k < 2 as core::ffi::c_int {
                while ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD)
                    == 1 as core::ffi::c_int
                {
                    let ref mut fresh0 = *qn.offset(k as isize);
                    *fresh0 += 1 as core::ffi::c_int;
                }
                if *qn.offset(k as isize) > 0 as core::ffi::c_int {
                    let ref mut fresh1 = *qn.offset(k as isize);
                    *fresh1 += 1 as core::ffi::c_int;
                }
                k += 1;
            }
        }
        0 | 2 | 3 => {
            k = 0 as core::ffi::c_int as WORD32;
            while k < 2 as core::ffi::c_int {
                *qn.offset(k as isize) = 2 as WORD32
                    + ixheaacd_read_bits_buf(it_bit_buff, 2 as WORD);
                k += 1;
            }
            if nk_mode == 2 as core::ffi::c_int {
                k = 0 as core::ffi::c_int as WORD32;
                while k < 2 as core::ffi::c_int {
                    if *qn.offset(k as isize) > 4 as core::ffi::c_int {
                        *qn.offset(k as isize) = 0 as core::ffi::c_int as WORD32;
                        while ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD)
                            == 1 as core::ffi::c_int
                        {
                            let ref mut fresh2 = *qn.offset(k as isize);
                            *fresh2 += 1 as core::ffi::c_int;
                        }
                        if *qn.offset(k as isize) > 0 as core::ffi::c_int {
                            let ref mut fresh3 = *qn.offset(k as isize);
                            *fresh3 += 4 as core::ffi::c_int;
                        }
                    }
                    k += 1;
                }
            } else {
                k = 0 as core::ffi::c_int as WORD32;
                while k < 2 as core::ffi::c_int {
                    if *qn.offset(k as isize) > 4 as core::ffi::c_int {
                        let mut qn_ext: WORD32 = 0 as WORD32;
                        while ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD)
                            == 1 as core::ffi::c_int
                        {
                            qn_ext += 1 as core::ffi::c_int;
                        }
                        match qn_ext {
                            0 => {
                                *qn.offset(k as isize) = 5 as core::ffi::c_int as WORD32;
                            }
                            1 => {
                                *qn.offset(k as isize) = 6 as core::ffi::c_int as WORD32;
                            }
                            2 => {
                                *qn.offset(k as isize) = 0 as core::ffi::c_int as WORD32;
                            }
                            _ => {
                                *qn.offset(k as isize) = (qn_ext as core::ffi::c_int
                                    + 4 as core::ffi::c_int) as WORD32;
                            }
                        }
                    }
                    k += 1;
                }
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_code_book_indices(
    mut pstr_td_frame_data: *mut ia_td_frame_data_struct,
    mut nk_mode: WORD32,
    mut pos: *mut WORD32,
    mut it_bit_buff: *mut ia_bit_buf_struct,
) -> VOID {
    let mut k: WORD32 = 0;
    let mut qn: [WORD32; 2] = [0 as core::ffi::c_int, 0 as core::ffi::c_int];
    let mut nk: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut i: WORD32 = 0;
    ixheaacd_qn_data(
        nk_mode,
        &mut *qn.as_mut_ptr().offset(0 as core::ffi::c_int as isize),
        it_bit_buff,
    );
    let fresh4 = *pos;
    *pos = *pos + 1;
    (*pstr_td_frame_data).lpc_first_approx_idx[fresh4 as usize] = qn[0
        as core::ffi::c_int as usize];
    let fresh5 = *pos;
    *pos = *pos + 1;
    (*pstr_td_frame_data).lpc_first_approx_idx[fresh5 as usize] = qn[1
        as core::ffi::c_int as usize];
    k = 0 as core::ffi::c_int as WORD32;
    while k < 2 as core::ffi::c_int {
        if qn[k as usize] > 0 as core::ffi::c_int {
            if qn[k as usize] > 4 as core::ffi::c_int {
                nk = ((qn[k as usize] - 3 as core::ffi::c_int) / 2 as core::ffi::c_int)
                    as WORD32;
                n = (qn[k as usize] - nk as core::ffi::c_int * 2 as core::ffi::c_int)
                    as WORD32;
            } else {
                nk = 0 as core::ffi::c_int as WORD32;
                n = qn[k as usize];
            }
            let fresh6 = *pos;
            *pos = *pos + 1;
            (*pstr_td_frame_data).lpc_first_approx_idx[fresh6 as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                4 as WORD * n as WORD,
            );
            i = 0 as core::ffi::c_int as WORD32;
            while i < 8 as core::ffi::c_int {
                let fresh7 = *pos;
                *pos = *pos + 1;
                (*pstr_td_frame_data).lpc_first_approx_idx[fresh7 as usize] = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    nk as WORD,
                );
                i += 1;
            }
        }
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_lpc_data(
    mut first_lpd_flag: WORD32,
    mut mod_0: *mut WORD32,
    mut pstr_td_frame_data: *mut ia_td_frame_data_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
) -> VOID {
    let mut mode_lpc: WORD32 = 0;
    let mut nk_mode: WORD32 = 0 as WORD32;
    let mut j: WORD32 = 0 as WORD32;
    mode_lpc = ixheaacd_get_mode_lpc(4 as WORD32, it_bit_buff, &mut nk_mode);
    let fresh8 = j;
    j = j + 1;
    (*pstr_td_frame_data).lpc_first_approx_idx[fresh8 as usize] = ixheaacd_read_bits_buf(
        it_bit_buff,
        8 as WORD,
    );
    ixheaacd_code_book_indices(pstr_td_frame_data, nk_mode, &mut j, it_bit_buff);
    if first_lpd_flag != 0 {
        mode_lpc = ixheaacd_get_mode_lpc(0 as WORD32, it_bit_buff, &mut nk_mode);
        let fresh9 = j;
        j = j + 1;
        (*pstr_td_frame_data).lpc_first_approx_idx[fresh9 as usize] = mode_lpc;
        if mode_lpc == 0 as core::ffi::c_int {
            let fresh10 = j;
            j = j + 1;
            (*pstr_td_frame_data).lpc_first_approx_idx[fresh10 as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                8 as WORD,
            );
        }
        ixheaacd_code_book_indices(pstr_td_frame_data, nk_mode, &mut j, it_bit_buff);
    }
    if *mod_0.offset(0 as core::ffi::c_int as isize) < 3 as core::ffi::c_int {
        mode_lpc = ixheaacd_get_mode_lpc(2 as WORD32, it_bit_buff, &mut nk_mode);
        let fresh11 = j;
        j = j + 1;
        (*pstr_td_frame_data).lpc_first_approx_idx[fresh11 as usize] = mode_lpc;
        if mode_lpc == 0 as core::ffi::c_int {
            let fresh12 = j;
            j = j + 1;
            (*pstr_td_frame_data).lpc_first_approx_idx[fresh12 as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                8 as WORD,
            );
        }
        ixheaacd_code_book_indices(pstr_td_frame_data, nk_mode, &mut j, it_bit_buff);
    }
    if *mod_0.offset(0 as core::ffi::c_int as isize) < 2 as core::ffi::c_int {
        mode_lpc = ixheaacd_get_mode_lpc(1 as WORD32, it_bit_buff, &mut nk_mode);
        let fresh13 = j;
        j = j + 1;
        (*pstr_td_frame_data).lpc_first_approx_idx[fresh13 as usize] = mode_lpc;
        if mode_lpc == 0 as core::ffi::c_int {
            let fresh14 = j;
            j = j + 1;
            (*pstr_td_frame_data).lpc_first_approx_idx[fresh14 as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                8 as WORD,
            );
        }
        if mode_lpc != 1 as core::ffi::c_int {
            ixheaacd_code_book_indices(pstr_td_frame_data, nk_mode, &mut j, it_bit_buff);
        }
    }
    if *mod_0.offset(2 as core::ffi::c_int as isize) < 2 as core::ffi::c_int {
        mode_lpc = ixheaacd_get_mode_lpc(3 as WORD32, it_bit_buff, &mut nk_mode);
        let fresh15 = j;
        j = j + 1;
        (*pstr_td_frame_data).lpc_first_approx_idx[fresh15 as usize] = mode_lpc;
        if mode_lpc == 0 as core::ffi::c_int {
            let fresh16 = j;
            j = j + 1;
            (*pstr_td_frame_data).lpc_first_approx_idx[fresh16 as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                8 as WORD,
            );
        }
        ixheaacd_code_book_indices(pstr_td_frame_data, nk_mode, &mut j, it_bit_buff);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_fac_decoding(
    mut fac_length: WORD32,
    mut k: WORD32,
    mut fac_prm: *mut WORD32,
    mut it_bit_buff: *mut ia_bit_buf_struct,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut qn: WORD32 = 0;
    let mut nk: WORD32 = 0;
    let mut kv: [WORD32; 8] = [0; 8];
    let mut code_book_index: core::ffi::c_long = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < fac_length {
        qn = 0 as core::ffi::c_int as WORD32;
        while ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD) == 1 as core::ffi::c_int {
            qn += 1 as core::ffi::c_int;
        }
        if qn != 0 as core::ffi::c_int {
            qn += 1 as core::ffi::c_int;
        }
        nk = 0 as core::ffi::c_int as WORD32;
        n = qn;
        if qn > 4 as core::ffi::c_int {
            nk = (qn as core::ffi::c_int - 3 as core::ffi::c_int
                >> 1 as core::ffi::c_int) as WORD32;
            n = (qn as core::ffi::c_int - nk as core::ffi::c_int * 2 as core::ffi::c_int)
                as WORD32;
        }
        code_book_index = ixheaacd_read_bits_buf(it_bit_buff, 4 as WORD * n as WORD)
            as core::ffi::c_long;
        j = 0 as core::ffi::c_int as WORD32;
        while j < 8 as core::ffi::c_int {
            kv[j as usize] = ixheaacd_read_bits_buf(it_bit_buff, nk as WORD);
            j += 1;
        }
        ixheaacd_rotated_gosset_mtx_dec(
            qn,
            code_book_index as WORD32,
            kv.as_mut_ptr(),
            &mut *fac_prm.offset((k * FAC_LENGTH + i) as isize),
        );
        i += 8 as core::ffi::c_int;
    }
}
#[no_mangle]
pub static mut ixheaacd_num_bites_celp_coding: [[UWORD8; 4]; 8] = [
    [
        5 as core::ffi::c_int as UWORD8,
        5 as core::ffi::c_int as UWORD8,
        5 as core::ffi::c_int as UWORD8,
        5 as core::ffi::c_int as UWORD8,
    ],
    [
        9 as core::ffi::c_int as UWORD8,
        9 as core::ffi::c_int as UWORD8,
        5 as core::ffi::c_int as UWORD8,
        5 as core::ffi::c_int as UWORD8,
    ],
    [
        9 as core::ffi::c_int as UWORD8,
        9 as core::ffi::c_int as UWORD8,
        9 as core::ffi::c_int as UWORD8,
        9 as core::ffi::c_int as UWORD8,
    ],
    [
        13 as core::ffi::c_int as UWORD8,
        13 as core::ffi::c_int as UWORD8,
        9 as core::ffi::c_int as UWORD8,
        9 as core::ffi::c_int as UWORD8,
    ],
    [
        13 as core::ffi::c_int as UWORD8,
        13 as core::ffi::c_int as UWORD8,
        13 as core::ffi::c_int as UWORD8,
        13 as core::ffi::c_int as UWORD8,
    ],
    [
        16 as core::ffi::c_int as UWORD8,
        16 as core::ffi::c_int as UWORD8,
        16 as core::ffi::c_int as UWORD8,
        16 as core::ffi::c_int as UWORD8,
    ],
    [
        1 as core::ffi::c_int as UWORD8,
        5 as core::ffi::c_int as UWORD8,
        1 as core::ffi::c_int as UWORD8,
        5 as core::ffi::c_int as UWORD8,
    ],
    [
        1 as core::ffi::c_int as UWORD8,
        5 as core::ffi::c_int as UWORD8,
        5 as core::ffi::c_int as UWORD8,
        5 as core::ffi::c_int as UWORD8,
    ],
];
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_acelp_decoding(
    mut k: WORD32,
    mut usac_data: *mut ia_usac_data_struct,
    mut pstr_td_frame_data: *mut ia_td_frame_data_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut chan: WORD32,
) -> VOID {
    let mut sfr: WORD32 = 0;
    let mut kk: WORD32 = 0;
    let mut nb_subfr: WORD32 = (*usac_data).num_subfrm;
    let mut ptr_num_bits: *const UWORD8 = &*(*ixheaacd_num_bites_celp_coding
        .as_ptr()
        .offset((*pstr_td_frame_data).acelp_core_mode as isize))
        .as_ptr()
        .offset(0 as core::ffi::c_int as isize) as *const UWORD8;
    chan = 0 as core::ffi::c_int as WORD32;
    (*pstr_td_frame_data).mean_energy[k as usize] = ixheaacd_read_bits_buf(
        it_bit_buff,
        2 as WORD,
    );
    sfr = 0 as core::ffi::c_int as WORD32;
    while sfr < nb_subfr {
        kk = k * 4 as WORD32 + sfr;
        if sfr == 0 as core::ffi::c_int
            || nb_subfr == 4 as core::ffi::c_int && sfr == 2 as core::ffi::c_int
        {
            (*pstr_td_frame_data).acb_index[kk as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                9 as WORD,
            );
        } else {
            (*pstr_td_frame_data).acb_index[kk as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                6 as WORD,
            );
        }
        (*pstr_td_frame_data).ltp_filtering_flag[kk as usize] = ixheaacd_read_bits_buf(
            it_bit_buff,
            1 as WORD,
        );
        if (*pstr_td_frame_data).acelp_core_mode == 5 as core::ffi::c_int {
            (*pstr_td_frame_data)
                .icb_index[kk as usize][0 as core::ffi::c_int as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                2 as WORD,
            );
            (*pstr_td_frame_data)
                .icb_index[kk as usize][1 as core::ffi::c_int as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                2 as WORD,
            );
            (*pstr_td_frame_data)
                .icb_index[kk as usize][2 as core::ffi::c_int as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                2 as WORD,
            );
            (*pstr_td_frame_data)
                .icb_index[kk as usize][3 as core::ffi::c_int as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                2 as WORD,
            );
            (*pstr_td_frame_data)
                .icb_index[kk as usize][4 as core::ffi::c_int as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                14 as WORD,
            );
            (*pstr_td_frame_data)
                .icb_index[kk as usize][5 as core::ffi::c_int as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                14 as WORD,
            );
            (*pstr_td_frame_data)
                .icb_index[kk as usize][6 as core::ffi::c_int as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                14 as WORD,
            );
            (*pstr_td_frame_data)
                .icb_index[kk as usize][7 as core::ffi::c_int as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                14 as WORD,
            );
        } else {
            (*pstr_td_frame_data)
                .icb_index[kk as usize][0 as core::ffi::c_int as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                *ptr_num_bits.offset(0 as core::ffi::c_int as isize) as WORD,
            );
            (*pstr_td_frame_data)
                .icb_index[kk as usize][1 as core::ffi::c_int as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                *ptr_num_bits.offset(1 as core::ffi::c_int as isize) as WORD,
            );
            (*pstr_td_frame_data)
                .icb_index[kk as usize][2 as core::ffi::c_int as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                *ptr_num_bits.offset(2 as core::ffi::c_int as isize) as WORD,
            );
            (*pstr_td_frame_data)
                .icb_index[kk as usize][3 as core::ffi::c_int as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                *ptr_num_bits.offset(3 as core::ffi::c_int as isize) as WORD,
            );
        }
        (*pstr_td_frame_data).gains[kk as usize] = ixheaacd_read_bits_buf(
            it_bit_buff,
            7 as WORD,
        );
        sfr += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_tcx_coding(
    mut usac_data: *mut ia_usac_data_struct,
    mut quant: pWORD32,
    mut k: WORD32,
    mut first_tcx_flag: WORD32,
    mut pstr_td_frame_data: *mut ia_td_frame_data_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
) -> VOID {
    (*pstr_td_frame_data).noise_factor[k as usize] = ixheaacd_read_bits_buf(
        it_bit_buff,
        3 as WORD,
    );
    (*pstr_td_frame_data).global_gain[k as usize] = ixheaacd_read_bits_buf(
        it_bit_buff,
        7 as WORD,
    );
    match (*pstr_td_frame_data).mod_0[k as usize] {
        1 => {
            (*pstr_td_frame_data).tcx_lg[k as usize] = (*usac_data).len_subfrm;
        }
        2 => {
            (*pstr_td_frame_data).tcx_lg[k as usize] = 2 as WORD32
                * (*usac_data).len_subfrm;
        }
        3 => {
            (*pstr_td_frame_data).tcx_lg[k as usize] = 4 as WORD32
                * (*usac_data).len_subfrm;
        }
        _ => {}
    }
    if first_tcx_flag != 0 {
        if (*usac_data).usac_independency_flg != 0 {
            (*pstr_td_frame_data).arith_reset_flag = 1 as core::ffi::c_int as WORD32;
        } else {
            (*pstr_td_frame_data).arith_reset_flag = ixheaacd_read_bits_buf(
                it_bit_buff,
                1 as WORD,
            );
        }
    }
    ixheaacd_arith_data(
        pstr_td_frame_data,
        quant as *mut WORD32,
        usac_data,
        it_bit_buff,
        first_tcx_flag,
        k,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_lpd_channel_stream(
    mut usac_data: *mut ia_usac_data_struct,
    mut pstr_td_frame_data: *mut ia_td_frame_data_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut synth: *mut FLOAT32,
) -> WORD32 {
    let mut lpd_mode: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut cnt: WORD32 = 0;
    let mut ii: WORD32 = 0;
    let mut first_tcx_flag: WORD32 = 0;
    let mut quant: *mut WORD32 = 0 as *mut WORD32;
    let mut core_mode_last: WORD32 = 0 as WORD32;
    let mut fac_data_present: WORD32 = 0;
    let mut fac_data: *mut WORD32 = 0 as *mut WORD32;
    let mut first_lpd_flag: WORD32 = 0 as WORD32;
    let mut short_fac_flag: WORD32 = 0;
    let mut bpf_control_info: WORD32 = 0 as WORD32;
    let mut chan: WORD32 = (*usac_data).present_chan;
    let mut last_lpd_mode: WORD32 = (*(*usac_data).str_tddec[chan as usize]).mode_prev;
    let mut err: WORD32 = 0 as WORD32;
    short_fac_flag = 0 as core::ffi::c_int as WORD32;
    let mut local: jmp_buf = [__jmp_buf_tag {
        __jmpbuf: [0; 8],
        __mask_was_saved: 0,
        __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1];
    if (*usac_data).ec_flag != 0 {
        err = _setjmp(local.as_mut_ptr()) as WORD32;
    }
    if (*usac_data).sampling_rate_idx <= 5 as core::ffi::c_int
        && (*usac_data).ec_flag != 0
    {
        (*usac_data).frame_ok = 0 as core::ffi::c_int as WORD32;
    }
    if err == 0 as core::ffi::c_int && (*usac_data).frame_ok == 1 as core::ffi::c_int {
        if (*usac_data).ec_flag != 0 {
            (*it_bit_buff).xaac_jmp_buf = &mut local;
        }
        (*pstr_td_frame_data).acelp_core_mode = ixheaacd_read_bits_buf(
            it_bit_buff,
            3 as WORD,
        );
        lpd_mode = ixheaacd_read_bits_buf(it_bit_buff, 5 as WORD);
        if lpd_mode > 25 as core::ffi::c_int || lpd_mode < 0 as core::ffi::c_int {
            if (*usac_data).ec_flag != 0 {
                longjmp(
                    (*(*it_bit_buff).xaac_jmp_buf).as_mut_ptr(),
                    IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES,
                );
            } else {
                return IA_FATAL_ERROR as WORD32
            }
        }
        if lpd_mode == 25 as core::ffi::c_int {
            (*pstr_td_frame_data).mod_0[3 as core::ffi::c_int as usize] = 3
                as core::ffi::c_int as WORD32;
            (*pstr_td_frame_data).mod_0[2 as core::ffi::c_int as usize] = (*pstr_td_frame_data)
                .mod_0[3 as core::ffi::c_int as usize];
            (*pstr_td_frame_data).mod_0[1 as core::ffi::c_int as usize] = (*pstr_td_frame_data)
                .mod_0[2 as core::ffi::c_int as usize];
            (*pstr_td_frame_data).mod_0[0 as core::ffi::c_int as usize] = (*pstr_td_frame_data)
                .mod_0[1 as core::ffi::c_int as usize];
        } else if lpd_mode == 24 as core::ffi::c_int {
            (*pstr_td_frame_data).mod_0[3 as core::ffi::c_int as usize] = 2
                as core::ffi::c_int as WORD32;
            (*pstr_td_frame_data).mod_0[2 as core::ffi::c_int as usize] = (*pstr_td_frame_data)
                .mod_0[3 as core::ffi::c_int as usize];
            (*pstr_td_frame_data).mod_0[1 as core::ffi::c_int as usize] = (*pstr_td_frame_data)
                .mod_0[2 as core::ffi::c_int as usize];
            (*pstr_td_frame_data).mod_0[0 as core::ffi::c_int as usize] = (*pstr_td_frame_data)
                .mod_0[1 as core::ffi::c_int as usize];
        } else if lpd_mode >= 20 as core::ffi::c_int {
            (*pstr_td_frame_data).mod_0[0 as core::ffi::c_int as usize] = (lpd_mode
                as core::ffi::c_int & 1 as core::ffi::c_int) as WORD32;
            (*pstr_td_frame_data).mod_0[1 as core::ffi::c_int as usize] = (lpd_mode
                as core::ffi::c_int >> 1 as core::ffi::c_int & 1 as core::ffi::c_int)
                as WORD32;
            (*pstr_td_frame_data).mod_0[3 as core::ffi::c_int as usize] = 2
                as core::ffi::c_int as WORD32;
            (*pstr_td_frame_data).mod_0[2 as core::ffi::c_int as usize] = (*pstr_td_frame_data)
                .mod_0[3 as core::ffi::c_int as usize];
        } else if lpd_mode >= 16 as core::ffi::c_int {
            (*pstr_td_frame_data).mod_0[1 as core::ffi::c_int as usize] = 2
                as core::ffi::c_int as WORD32;
            (*pstr_td_frame_data).mod_0[0 as core::ffi::c_int as usize] = (*pstr_td_frame_data)
                .mod_0[1 as core::ffi::c_int as usize];
            (*pstr_td_frame_data).mod_0[2 as core::ffi::c_int as usize] = (lpd_mode
                as core::ffi::c_int & 1 as core::ffi::c_int) as WORD32;
            (*pstr_td_frame_data).mod_0[3 as core::ffi::c_int as usize] = (lpd_mode
                as core::ffi::c_int >> 1 as core::ffi::c_int & 1 as core::ffi::c_int)
                as WORD32;
        } else {
            (*pstr_td_frame_data).mod_0[0 as core::ffi::c_int as usize] = (lpd_mode
                as core::ffi::c_int & 1 as core::ffi::c_int) as WORD32;
            (*pstr_td_frame_data).mod_0[1 as core::ffi::c_int as usize] = (lpd_mode
                as core::ffi::c_int >> 1 as core::ffi::c_int & 1 as core::ffi::c_int)
                as WORD32;
            (*pstr_td_frame_data).mod_0[2 as core::ffi::c_int as usize] = (lpd_mode
                as core::ffi::c_int >> 2 as core::ffi::c_int & 1 as core::ffi::c_int)
                as WORD32;
            (*pstr_td_frame_data).mod_0[3 as core::ffi::c_int as usize] = (lpd_mode
                as core::ffi::c_int >> 3 as core::ffi::c_int & 1 as core::ffi::c_int)
                as WORD32;
        }
        bpf_control_info = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
        core_mode_last = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
        fac_data_present = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
        first_lpd_flag = (if core_mode_last == 0 as core::ffi::c_int {
            1 as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        }) as WORD32;
        quant = ((*pstr_td_frame_data).x_tcx_invquant).as_mut_ptr();
        first_tcx_flag = 1 as core::ffi::c_int as WORD32;
        k = 0 as core::ffi::c_int as WORD32;
        while k < 4 as core::ffi::c_int {
            if k == 0 as core::ffi::c_int {
                if core_mode_last == 1 as core::ffi::c_int
                    && fac_data_present == 1 as core::ffi::c_int
                {
                    ixheaacd_fac_decoding(
                        (*usac_data).len_subfrm / 2 as WORD32,
                        k,
                        ((*pstr_td_frame_data).fac).as_mut_ptr(),
                        it_bit_buff,
                    );
                }
            } else if last_lpd_mode == 0 as core::ffi::c_int
                && (*pstr_td_frame_data).mod_0[k as usize] > 0 as core::ffi::c_int
                || last_lpd_mode > 0 as core::ffi::c_int
                    && (*pstr_td_frame_data).mod_0[k as usize] == 0 as core::ffi::c_int
            {
                ixheaacd_fac_decoding(
                    (*usac_data).len_subfrm / 2 as WORD32,
                    k,
                    ((*pstr_td_frame_data).fac).as_mut_ptr(),
                    it_bit_buff,
                );
            }
            if (*pstr_td_frame_data).mod_0[k as usize] == 0 as core::ffi::c_int {
                ixheaacd_acelp_decoding(
                    k,
                    usac_data,
                    pstr_td_frame_data,
                    it_bit_buff,
                    chan,
                );
                last_lpd_mode = 0 as core::ffi::c_int as WORD32;
                (*pstr_td_frame_data).tcx_lg[k as usize] = 0 as core::ffi::c_int
                    as WORD32;
                k += 1 as core::ffi::c_int;
            } else {
                ixheaacd_tcx_coding(
                    usac_data,
                    quant as pWORD32,
                    k,
                    first_tcx_flag,
                    pstr_td_frame_data,
                    it_bit_buff,
                );
                last_lpd_mode = (*pstr_td_frame_data).mod_0[k as usize];
                quant = quant.offset((*pstr_td_frame_data).tcx_lg[k as usize] as isize);
                cnt = ((1 as core::ffi::c_int)
                    << (*pstr_td_frame_data).mod_0[k as usize] - 1 as core::ffi::c_int)
                    as WORD32;
                ii = 0 as core::ffi::c_int as WORD32;
                while ii < cnt as core::ffi::c_int - 1 as core::ffi::c_int {
                    (*pstr_td_frame_data).tcx_lg[(k + 1 as WORD32 + ii) as usize] = 0
                        as core::ffi::c_int as WORD32;
                    ii += 1;
                }
                k += cnt;
                first_tcx_flag = 0 as core::ffi::c_int as WORD32;
            }
        }
        ixheaacd_lpc_data(
            first_lpd_flag,
            ((*pstr_td_frame_data).mod_0).as_mut_ptr(),
            pstr_td_frame_data,
            it_bit_buff,
        );
        if core_mode_last == 0 as core::ffi::c_int
            && fac_data_present == 1 as core::ffi::c_int
        {
            let mut fac_length: WORD32 = 0;
            short_fac_flag = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
            fac_length = (if short_fac_flag != 0 {
                (*usac_data).ccfl as core::ffi::c_int / 16 as core::ffi::c_int
            } else {
                (*usac_data).ccfl as core::ffi::c_int / 8 as core::ffi::c_int
            }) as WORD32;
            fac_data = ((*pstr_td_frame_data).fac_data).as_mut_ptr();
            *fac_data.offset(0 as core::ffi::c_int as isize) = ixheaacd_read_bits_buf(
                it_bit_buff,
                7 as WORD,
            );
            if (*pstr_td_frame_data).fac_data[0 as core::ffi::c_int as usize]
                < 0 as core::ffi::c_int
                || (*pstr_td_frame_data).fac_data[0 as core::ffi::c_int as usize]
                    > 128 as core::ffi::c_int
            {
                if (*usac_data).ec_flag != 0 {
                    longjmp(
                        (*(*it_bit_buff).xaac_jmp_buf).as_mut_ptr(),
                        IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES,
                    );
                } else {
                    return IA_FATAL_ERROR as WORD32
                }
            }
            ixheaacd_fac_decoding(
                fac_length,
                0 as WORD32,
                &mut *fac_data.offset(1 as core::ffi::c_int as isize),
                it_bit_buff,
            );
        }
        if fac_data_present == 0 as core::ffi::c_int {
            memset(
                &mut *((*pstr_td_frame_data).fac_data)
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize) as *mut WORD32
                    as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ::core::mem::size_of::<[WORD32; 129]>() as size_t,
            );
        }
        if (*usac_data).ec_flag != 0 && (*usac_data).frame_ok != 0 {
            (*usac_data).bpf_control_info = bpf_control_info;
            (*usac_data).core_mode_last = core_mode_last;
            (*usac_data).first_lpd_flag = first_lpd_flag;
        }
    } else {
        (*usac_data).frame_ok = 0 as core::ffi::c_int as WORD32;
    }
    if (*usac_data).ec_flag != 0 {
        (*usac_data).str_error_concealment[chan as usize].pstr_ec_scratch = &mut (*((*usac_data)
            .str_error_concealment)
            .as_mut_ptr()
            .offset(chan as isize))
            .str_ec_scratch as *mut ia_ec_scratch_str;
        ixheaacd_usac_apply_ec(
            usac_data,
            &*ixheaacd_samp_rate_info.as_ptr().offset(0 as core::ffi::c_int as isize),
            chan,
        );
        if (*usac_data).frame_ok == 0 as core::ffi::c_int {
            bpf_control_info = (*usac_data).bpf_control_info;
            core_mode_last = (*usac_data).core_mode_last;
            first_lpd_flag = (*usac_data).first_lpd_flag;
        }
    }
    if (*usac_data).frame_ok != 0 {
        err = ixheaacd_lpd_dec(
            usac_data,
            (*usac_data).str_tddec[chan as usize],
            pstr_td_frame_data,
            synth as *mut FLOAT32,
            first_lpd_flag,
            short_fac_flag,
            bpf_control_info,
        );
    } else if (*usac_data).ec_flag != 0 {
        (*usac_data).fac_data_present[chan as usize] = 0 as core::ffi::c_int as WORD32;
        err = ixheaacd_lpd_dec(
            usac_data,
            (*usac_data).str_tddec[chan as usize],
            &mut *((*usac_data).td_frame_data_prev).as_mut_ptr().offset(chan as isize),
            synth as *mut FLOAT32,
            first_lpd_flag,
            short_fac_flag,
            bpf_control_info,
        );
    }
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_tw_buff_update(
    mut usac_data: *mut ia_usac_data_struct,
    mut i: WORD32,
    mut st: ia_usac_lpd_decoder_handle,
) -> WORD32 {
    let mut p_ioverlap: *mut WORD32 = ((*usac_data).overlap_data_ptr[i as usize])
        .as_mut_ptr();
    let mut td_frame_prev: WORD32 = (*usac_data).td_frame_prev[i as usize];
    let mut window_sequence_last: WORD32 = (*usac_data).window_sequence_last[i as usize];
    let mut tw_mdct: WORD32 = (*usac_data).tw_mdct[0 as core::ffi::c_int as usize];
    if td_frame_prev == 0 {
        if tw_mdct != 0 {
            if (*usac_data).ec_flag == 0 as core::ffi::c_int {
                return -(1 as WORD32)
            } else {
                tw_mdct = 0 as core::ffi::c_int as WORD32;
            }
        } else {
            ixheaacd_reset_acelp_data_fix(
                usac_data,
                st,
                p_ioverlap,
                (window_sequence_last == EIGHT_SHORT_SEQUENCE) as core::ffi::c_int,
                0 as WORD32,
            );
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_td_frm_dec(
    mut usac_data: *mut ia_usac_data_struct,
    mut k: WORD32,
    mut mod0: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut lfac: WORD32 = 0 as WORD32;
    let mut p_out_idata: *mut WORD32 = ((*usac_data).output_data_ptr[k as usize])
        .as_mut_ptr();
    let mut p_ioverlap: *mut WORD32 = ((*usac_data).overlap_data_ptr[k as usize])
        .as_mut_ptr();
    let mut nlong: WORD32 = (*usac_data).ccfl;
    let mut window_sequence_last: WORD32 = (*usac_data).window_sequence_last[k as usize];
    let mut td_frame_prev: WORD32 = (*usac_data).td_frame_prev[k as usize];
    let mut tw_mdct: WORD32 = (*usac_data).tw_mdct[0 as core::ffi::c_int as usize];
    let mut nshort: WORD32 = nlong / 8 as WORD32;
    let mut p_in_idata: *mut WORD32 = p_out_idata;
    if td_frame_prev == 0 {
        if window_sequence_last == EIGHT_SHORT_SEQUENCE {
            lfac = (nshort as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
        } else {
            lfac = nshort;
        }
    }
    if td_frame_prev == 0 && mod0 == 0 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i
            < nlong as core::ffi::c_int / 2 as core::ffi::c_int
                - lfac as core::ffi::c_int - 64 as core::ffi::c_int
        {
            *p_in_idata.offset(i as isize) = 0 as core::ffi::c_int as WORD32;
            i += 1;
        }
    } else if td_frame_prev == 0 && mod0 > 0 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < nlong / 2 as WORD32 - lfac {
            *p_in_idata.offset(i as isize) = 0 as core::ffi::c_int as WORD32;
            i += 1;
        }
    }
    if tw_mdct != 0 {
        if td_frame_prev == 0 && mod0 == 0 as core::ffi::c_int {
            i = (nlong as core::ffi::c_int / 2 as core::ffi::c_int
                - lfac as core::ffi::c_int - 64 as core::ffi::c_int) as WORD32;
            while i < nlong as core::ffi::c_int / 2 as core::ffi::c_int {
                *p_ioverlap
                    .offset(
                        (i as core::ffi::c_int
                            + nlong as core::ffi::c_int / 2 as core::ffi::c_int) as isize,
                    ) = 0 as core::ffi::c_int as WORD32;
                i += 1;
            }
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < nlong as core::ffi::c_int / 2 as core::ffi::c_int {
            *p_out_idata.offset(i as isize) = *p_ioverlap.offset(i as isize)
                << 1 as core::ffi::c_int;
            *p_out_idata
                .offset(
                    (i as core::ffi::c_int
                        + nlong as core::ffi::c_int / 2 as core::ffi::c_int) as isize,
                ) = ixheaac_add32_sat(
                *p_ioverlap
                    .offset(
                        (i as core::ffi::c_int
                            + nlong as core::ffi::c_int / 2 as core::ffi::c_int) as isize,
                    ) << 1 as core::ffi::c_int,
                *p_in_idata.offset(i as isize),
            );
            *p_ioverlap.offset(i as isize) = ixheaac_add32_sat(
                *p_in_idata
                    .offset(
                        (i as core::ffi::c_int
                            + nlong as core::ffi::c_int / 2 as core::ffi::c_int) as isize,
                    ) >> 1 as core::ffi::c_int,
                *p_ioverlap.offset((i + nlong) as isize),
            );
            *p_ioverlap
                .offset(
                    (i as core::ffi::c_int
                        + nlong as core::ffi::c_int / 2 as core::ffi::c_int) as isize,
                ) = 0 as core::ffi::c_int as WORD32;
            *p_ioverlap.offset((i + nlong) as isize) = 0 as core::ffi::c_int as WORD32;
            *p_ioverlap
                .offset(
                    (i as core::ffi::c_int + nlong as core::ffi::c_int
                        + nlong as core::ffi::c_int / 2 as core::ffi::c_int) as isize,
                ) = 0 as core::ffi::c_int as WORD32;
            i += 1;
        }
    } else {
        if td_frame_prev == 0 && mod0 == 0 as core::ffi::c_int {
            i = (nlong as core::ffi::c_int / 2 as core::ffi::c_int
                - lfac as core::ffi::c_int - 64 as core::ffi::c_int) as WORD32;
            while i < nlong as core::ffi::c_int / 2 as core::ffi::c_int {
                *p_ioverlap.offset(i as isize) = 0 as core::ffi::c_int as WORD32;
                i += 1;
            }
        } else if td_frame_prev == 0 {
            i = nlong / 2 as WORD32 - lfac;
            while i < nlong {
                *p_ioverlap.offset(i as isize) = 0 as core::ffi::c_int as WORD32;
                i += 1;
            }
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < nlong {
            *p_out_idata.offset(i as isize) = ixheaac_add32_sat(
                *p_ioverlap.offset(i as isize) << 1 as core::ffi::c_int,
                *p_in_idata.offset(i as isize),
            );
            *p_ioverlap.offset(i as isize) = 0 as core::ffi::c_int as WORD32;
            i += 1;
        }
    };
}
