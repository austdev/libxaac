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
    fn _setjmp(__env: *mut __jmp_buf_tag) -> core::ffi::c_int;
    fn longjmp(__env: *mut __jmp_buf_tag, __val: core::ffi::c_int) -> !;
    fn ixheaacd_read_bits_buf(
        it_bit_buff: *mut ia_bit_buf_struct,
        no_of_bits: WORD,
    ) -> WORD32;
    static mut ixheaacd_book: ia_huff_code_book_struct;
    fn ixheaacd_huff_codeword(
        h: *const ia_huff_code_word_struct,
        data_present: UWORD16,
        it_bit_buff: *mut ia_bit_buf_struct,
    ) -> WORD32;
    static ia_ec_fade_factors: [FLOAT32; 9];
    fn ixheaacd_tns_apply(
        usac_data: *mut ia_usac_data_struct,
        spec: *mut WORD32,
        nbands: WORD32,
        pstr_sfb_info: *mut ia_sfb_info_struct,
        pstr_tns: *mut ia_tns_frame_info_struct,
    ) -> IA_ERRORCODE;
    fn ixheaacd_tw_buff_update(
        usac_data: *mut ia_usac_data_struct,
        i: WORD32,
        st: ia_usac_lpd_decoder_handle,
    ) -> WORD32;
    fn ixheaacd_fix2flt_data(
        usac_data: *mut ia_usac_data_struct,
        st: ia_usac_lpd_decoder_handle,
        k: WORD32,
    ) -> VOID;
    fn ixheaacd_td_frm_dec(
        usac_data: *mut ia_usac_data_struct,
        k: WORD32,
        mod0: WORD32,
    ) -> VOID;
    fn ixheaacd_fd_frm_dec(usac_data: *mut ia_usac_data_struct, i_ch: WORD32) -> WORD32;
    fn ixheaacd_calc_grp_offset(
        pstr_sfb_info: *mut ia_sfb_info_struct,
        group: *mut UWORD8,
    ) -> VOID;
    fn ixheaacd_read_tns_u(
        pstr_sfb_info: *mut ia_sfb_info_struct,
        pstr_tns_frame_info: *mut ia_tns_frame_info_struct,
        it_bit_buff: *mut ia_bit_buf_struct,
    ) -> VOID;
    fn ixheaacd_lpd_channel_stream(
        usac_data: *mut ia_usac_data_struct,
        pstr_td_frame_data: *mut ia_td_frame_data_struct,
        it_bit_buff: *mut ia_bit_buf_struct,
        synth: *mut FLOAT32,
    ) -> WORD32;
    fn ixheaacd_win_seq_select(
        window_sequence_curr: WORD32,
        window_sequence_last: WORD32,
    ) -> WORD32;
    fn ixheaacd_fd_channel_stream(
        usac_data: *mut ia_usac_data_struct,
        pstr_core_coder: *mut ia_usac_tmp_core_coder_struct,
        max_sfb: *mut UWORD8,
        window_sequence_last: WORD32,
        chn: WORD32,
        noise_filling_config: WORD32,
        ch: WORD32,
        it_bit_buff: *mut ia_bit_buf_struct,
    ) -> WORD32;
    fn ixheaacd_lpd_dec(
        usac_data: *mut ia_usac_data_struct,
        st: ia_usac_lpd_decoder_handle,
        pstr_td_frame_data: *mut ia_td_frame_data_struct,
        fsynth: *mut FLOAT32,
        first_lpd_flag: WORD32,
        short_fac_flag: WORD32,
        bpf_control_info: WORD32,
    ) -> WORD32;
    fn ixheaacd_lpd_dec_update(
        tddec: ia_usac_lpd_decoder_handle,
        usac_data: *mut ia_usac_data_struct,
        i_ch: WORD32,
    ) -> VOID;
    fn ixheaacd_usac_apply_ec(
        pstr_usac_data: *mut ia_usac_data_struct,
        pstr_samp_rate_info: *const ia_usac_samp_rate_info,
        ch: WORD32,
    ) -> VOID;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [core::ffi::c_ulong; 16],
}
pub type __jmp_buf = [core::ffi::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: core::ffi::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
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
pub type C2RustUnnamed = core::ffi::c_uint;
pub const LEN_PC_COMM: C2RustUnnamed = 8;
pub const LEN_SAMP_IDX: C2RustUnnamed = 4;
pub const LEN_MAX_SFBL: C2RustUnnamed = 6;
pub const LEN_MAX_SFBS: C2RustUnnamed = 4;
pub const LEN_TAG: C2RustUnnamed = 4;
pub const SF_OFFSET: C2RustUnnamed = 100;
pub const MIDFAC: C2RustUnnamed = 60;
pub const MAXFAC: C2RustUnnamed = 121;
pub const MAXBANDS: C2RustUnnamed = 128;
pub const MAX_SBK: C2RustUnnamed = 8;
pub const NSHORT: C2RustUnnamed = 8;
pub const SN4: C2RustUnnamed = 64;
pub const LN4: C2RustUnnamed = 512;
pub const SN2: C2RustUnnamed = 128;
pub const LN2: C2RustUnnamed = 1024;
pub const SN: C2RustUnnamed = 256;
pub const LN: C2RustUnnamed = 2048;
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
pub struct ia_huff_code_word_struct {
    pub index: WORD32,
    pub len: WORD32,
    pub code_word: UWORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_huff_code_book_struct {
    pub num: WORD32,
    pub dim: WORD32,
    pub lav: WORD32,
    pub lav_incr_esc: WORD32,
    pub huff_mode: WORD32,
    pub off: WORD32,
    pub sign_code_book: WORD32,
    pub max_code_word_len: UWORD16,
    pub pstr_huff_code_word: *const ia_huff_code_word_struct,
    pub code_book_tbl: *const WORD16,
    pub idx_tbl: *const WORD32,
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
pub struct ia_usac_tmp_core_coder_struct {
    pub core_mode: [WORD32; 2],
    pub common_tw: WORD32,
    pub common_window: WORD32,
    pub tns_data_present: [WORD32; 2],
    pub tns_active: WORD32,
    pub common_tns: WORD32,
    pub tns_on_lr: WORD32,
    pub tns_present_both: WORD32,
    pub common_max_sfb: WORD32,
    pub max_sfb: [UWORD8; 2],
    pub max_sfb_ste: WORD32,
    pub pred_dir: WORD32,
    pub complex_coef: WORD32,
    pub use_prev_frame: WORD32,
    pub ms_mask_present: [UWORD8; 2],
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const BLOCK_LEN_LONG: core::ffi::c_int = 1024 as core::ffi::c_int;
pub const WIN_SEL_0: core::ffi::c_int = 0 as core::ffi::c_int;
pub const NUM_TW_NODES: core::ffi::c_int = 16 as core::ffi::c_int;
pub const CORE_MODE_FD: core::ffi::c_int = 0 as core::ffi::c_int;
pub const CORE_MODE_LPD: core::ffi::c_int = 1 as core::ffi::c_int;
pub const ONLY_LONG_SEQUENCE: core::ffi::c_int = 0;
pub const LONG_START_SEQUENCE: core::ffi::c_int = 1;
pub const EIGHT_SHORT_SEQUENCE: core::ffi::c_int = 2 as core::ffi::c_int;
pub const LONG_STOP_SEQUENCE: core::ffi::c_int = 3;
pub const STOP_START_SEQUENCE: core::ffi::c_int = 4;
pub const NSFB_SHORT: core::ffi::c_int = 16 as core::ffi::c_int;
pub const MAX_SHORT_IN_LONG_BLOCK: core::ffi::c_int = 8 as core::ffi::c_int;
pub const SFB_NUM_MAX: core::ffi::c_int = (NSFB_SHORT + 1 as core::ffi::c_int)
    * MAX_SHORT_IN_LONG_BLOCK;
pub const SFB_PER_PRED_BAND: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_FADE_FRAMES: core::ffi::c_int = 8 as core::ffi::c_int;
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
unsafe extern "C" fn ixheaac_mult32x32in64(mut a: WORD32, mut b: WORD32) -> WORD64 {
    let mut result: WORD64 = 0;
    result = a as WORD64 * b as WORD64;
    return result;
}
#[inline]
unsafe extern "C" fn ixheaac_mac32x32in64(
    mut sum: WORD64,
    mut a: WORD32,
    mut b: WORD32,
) -> WORD64 {
    sum += a as WORD64 * b as WORD64;
    return sum;
}
#[inline]
unsafe extern "C" fn ixheaac_mac32x32in64_7(
    mut a: *const WORD32,
    mut b: *const WORD16,
) -> WORD64 {
    let mut sum: WORD64 = 0;
    sum = *a.offset(0 as core::ffi::c_int as isize) as WORD64
        * *b.offset(0 as core::ffi::c_int as isize) as WORD64;
    sum
        += *a.offset(1 as core::ffi::c_int as isize) as WORD64
            * *b.offset(1 as core::ffi::c_int as isize) as WORD64;
    sum
        += *a.offset(2 as core::ffi::c_int as isize) as WORD64
            * *b.offset(2 as core::ffi::c_int as isize) as WORD64;
    sum
        += *a.offset(3 as core::ffi::c_int as isize) as WORD64
            * *b.offset(3 as core::ffi::c_int as isize) as WORD64;
    sum
        += *a.offset(4 as core::ffi::c_int as isize) as WORD64
            * *b.offset(4 as core::ffi::c_int as isize) as WORD64;
    sum
        += *a.offset(5 as core::ffi::c_int as isize) as WORD64
            * *b.offset(5 as core::ffi::c_int as isize) as WORD64;
    sum
        += *a.offset(6 as core::ffi::c_int as isize) as WORD64
            * *b.offset(6 as core::ffi::c_int as isize) as WORD64;
    return sum;
}
#[inline]
unsafe extern "C" fn ixheaac_mac32x32in64_n(
    mut sum: WORD64,
    mut a: *const WORD32,
    mut b: *const WORD16,
    mut n: WORD32,
) -> WORD64 {
    let mut k: WORD32 = 0;
    sum
        += *a.offset(0 as core::ffi::c_int as isize) as WORD64
            * *b.offset(0 as core::ffi::c_int as isize) as WORD64;
    k = 1 as core::ffi::c_int as WORD32;
    while k < n {
        sum += *a.offset(k as isize) as WORD64 * *b.offset(k as isize) as WORD64;
        k += 1;
    }
    return sum;
}
#[inline]
unsafe extern "C" fn ixheaac_sat64_32(mut a: WORD64) -> WORD32 {
    let mut result: WORD32 = 0;
    if a >= MAX_32 as WORD64 {
        result = MAX_32;
    } else if a <= MIN_32 as WORD64 {
        result = MIN_32;
    } else {
        result = a as WORD32;
    }
    return result;
}
pub const ID_USAC_LFE: core::ffi::c_int = 2 as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES: core::ffi::c_int = 0x1804
    as core::ffi::c_int;
#[no_mangle]
pub static mut ixheaacd_mdst_fcoeff_long_sin: [WORD16; 7] = [
    0 as core::ffi::c_int as WORD16,
    0 as core::ffi::c_int as WORD16,
    -(16384 as core::ffi::c_int) as WORD16,
    0 as core::ffi::c_int as WORD16,
    16384 as core::ffi::c_int as WORD16,
    0 as core::ffi::c_int as WORD16,
    0 as core::ffi::c_int as WORD16,
];
#[no_mangle]
pub static mut ixheaacd_mdst_fcoeff_long_kbd: [WORD16; 7] = [
    -(2998 as core::ffi::c_int) as WORD16,
    0 as core::ffi::c_int as WORD16,
    -(19052 as core::ffi::c_int) as WORD16,
    0 as core::ffi::c_int as WORD16,
    19052 as core::ffi::c_int as WORD16,
    0 as core::ffi::c_int as WORD16,
    2998 as core::ffi::c_int as WORD16,
];
#[no_mangle]
pub static mut ixheaacd_mdst_fcoeff_long_sin_kbd: [WORD16; 7] = [
    -(1499 as core::ffi::c_int) as WORD16,
    -(1876 as core::ffi::c_int) as WORD16,
    -(17718 as core::ffi::c_int) as WORD16,
    0 as core::ffi::c_int as WORD16,
    17718 as core::ffi::c_int as WORD16,
    1876 as core::ffi::c_int as WORD16,
    1499 as core::ffi::c_int as WORD16,
];
#[no_mangle]
pub static mut ixheaacd_mdst_fcoeff_long_kbd_sin: [WORD16; 7] = [
    -(1499 as core::ffi::c_int) as WORD16,
    1876 as core::ffi::c_int as WORD16,
    -(17718 as core::ffi::c_int) as WORD16,
    0 as core::ffi::c_int as WORD16,
    17718 as core::ffi::c_int as WORD16,
    -(1876 as core::ffi::c_int) as WORD16,
    1499 as core::ffi::c_int as WORD16,
];
#[no_mangle]
pub static mut ixheaacd_mdst_fcoeff_longshort_curr: [[*const WORD16; 2]; 2] = unsafe {
    [
        [
            ixheaacd_mdst_fcoeff_long_sin.as_ptr(),
            ixheaacd_mdst_fcoeff_long_sin_kbd.as_ptr(),
        ],
        [
            ixheaacd_mdst_fcoeff_long_kbd_sin.as_ptr(),
            ixheaacd_mdst_fcoeff_long_kbd.as_ptr(),
        ],
    ]
};
#[no_mangle]
pub static mut ixheaacd_mdst_fcoeff_start_sin: [WORD16; 7] = [
    -(3364 as core::ffi::c_int) as WORD16,
    -(3401 as core::ffi::c_int) as WORD16,
    -(18584 as core::ffi::c_int) as WORD16,
    0 as core::ffi::c_int as WORD16,
    18584 as core::ffi::c_int as WORD16,
    3401 as core::ffi::c_int as WORD16,
    3364 as core::ffi::c_int as WORD16,
];
#[no_mangle]
pub static mut ixheaacd_mdst_fcoeff_start_kbd: [WORD16; 7] = [
    -(4932 as core::ffi::c_int) as WORD16,
    -(1572 as core::ffi::c_int) as WORD16,
    -(19942 as core::ffi::c_int) as WORD16,
    0 as core::ffi::c_int as WORD16,
    19942 as core::ffi::c_int as WORD16,
    1572 as core::ffi::c_int as WORD16,
    4932 as core::ffi::c_int as WORD16,
];
#[no_mangle]
pub static mut ixheaacd_mdst_fcoeff_start_sin_kbd: [WORD16; 7] = [
    -(3433 as core::ffi::c_int) as WORD16,
    -(3447 as core::ffi::c_int) as WORD16,
    -(18608 as core::ffi::c_int) as WORD16,
    0 as core::ffi::c_int as WORD16,
    18608 as core::ffi::c_int as WORD16,
    3447 as core::ffi::c_int as WORD16,
    3433 as core::ffi::c_int as WORD16,
];
#[no_mangle]
pub static mut ixheaacd_mdst_fcoeff_start_kbd_sin: [WORD16; 7] = [
    -(4863 as core::ffi::c_int) as WORD16,
    -(1525 as core::ffi::c_int) as WORD16,
    -(19918 as core::ffi::c_int) as WORD16,
    0 as core::ffi::c_int as WORD16,
    19918 as core::ffi::c_int as WORD16,
    1525 as core::ffi::c_int as WORD16,
    4863 as core::ffi::c_int as WORD16,
];
#[no_mangle]
pub static mut ixheaacd_mdst_fcoeff_start_curr: [[*const WORD16; 2]; 2] = unsafe {
    [
        [
            ixheaacd_mdst_fcoeff_start_sin.as_ptr(),
            ixheaacd_mdst_fcoeff_start_sin_kbd.as_ptr(),
        ],
        [
            ixheaacd_mdst_fcoeff_start_kbd_sin.as_ptr(),
            ixheaacd_mdst_fcoeff_start_kbd.as_ptr(),
        ],
    ]
};
#[no_mangle]
pub static mut ixheaacd_mdst_fcoeff_stop_sin: [WORD16; 7] = [
    -(3364 as core::ffi::c_int) as WORD16,
    3401 as core::ffi::c_int as WORD16,
    -(18584 as core::ffi::c_int) as WORD16,
    0 as core::ffi::c_int as WORD16,
    18584 as core::ffi::c_int as WORD16,
    -(3401 as core::ffi::c_int) as WORD16,
    3364 as core::ffi::c_int as WORD16,
];
#[no_mangle]
pub static mut ixheaacd_mdst_fcoeff_stop_kbd: [WORD16; 7] = [
    -(4932 as core::ffi::c_int) as WORD16,
    1572 as core::ffi::c_int as WORD16,
    -(19942 as core::ffi::c_int) as WORD16,
    0 as core::ffi::c_int as WORD16,
    19942 as core::ffi::c_int as WORD16,
    -(1572 as core::ffi::c_int) as WORD16,
    4932 as core::ffi::c_int as WORD16,
];
#[no_mangle]
pub static mut ixheaacd_mdst_fcoeff_stop_sin_kbd: [WORD16; 7] = [
    -(4863 as core::ffi::c_int) as WORD16,
    1525 as core::ffi::c_int as WORD16,
    -(19918 as core::ffi::c_int) as WORD16,
    0 as core::ffi::c_int as WORD16,
    19918 as core::ffi::c_int as WORD16,
    -(1525 as core::ffi::c_int) as WORD16,
    4863 as core::ffi::c_int as WORD16,
];
#[no_mangle]
pub static mut ixheaacd_mdst_fcoeff_stop_kbd_sin: [WORD16; 7] = [
    -(3433 as core::ffi::c_int) as WORD16,
    3447 as core::ffi::c_int as WORD16,
    -(18608 as core::ffi::c_int) as WORD16,
    0 as core::ffi::c_int as WORD16,
    18608 as core::ffi::c_int as WORD16,
    -(3447 as core::ffi::c_int) as WORD16,
    3433 as core::ffi::c_int as WORD16,
];
#[no_mangle]
pub static mut ixheaacd_mdst_fcoeff_stop_cur: [[*const WORD16; 2]; 2] = unsafe {
    [
        [
            ixheaacd_mdst_fcoeff_stop_sin.as_ptr(),
            ixheaacd_mdst_fcoeff_stop_sin_kbd.as_ptr(),
        ],
        [
            ixheaacd_mdst_fcoeff_stop_kbd_sin.as_ptr(),
            ixheaacd_mdst_fcoeff_stop_kbd.as_ptr(),
        ],
    ]
};
#[no_mangle]
pub static mut ixheaacd_mdst_fcoeff_stopstart_sin: [WORD16; 7] = [
    -(6728 as core::ffi::c_int) as WORD16,
    0 as core::ffi::c_int as WORD16,
    -(20785 as core::ffi::c_int) as WORD16,
    0 as core::ffi::c_int as WORD16,
    20785 as core::ffi::c_int as WORD16,
    0 as core::ffi::c_int as WORD16,
    6728 as core::ffi::c_int as WORD16,
];
#[no_mangle]
pub static mut ixheaacd_mdst_fcoeff_stopstart_kbd: [WORD16; 7] = [
    -(6866 as core::ffi::c_int) as WORD16,
    -(0 as core::ffi::c_int) as WORD16,
    -(20831 as core::ffi::c_int) as WORD16,
    0 as core::ffi::c_int as WORD16,
    20831 as core::ffi::c_int as WORD16,
    0 as core::ffi::c_int as WORD16,
    6866 as core::ffi::c_int as WORD16,
];
#[no_mangle]
pub static mut ixheaacd_mdst_fcoeff_stopstart_sin_kbd: [WORD16; 7] = [
    -(6797 as core::ffi::c_int) as WORD16,
    -(46 as core::ffi::c_int) as WORD16,
    -(20808 as core::ffi::c_int) as WORD16,
    0 as core::ffi::c_int as WORD16,
    20808 as core::ffi::c_int as WORD16,
    46 as core::ffi::c_int as WORD16,
    6797 as core::ffi::c_int as WORD16,
];
#[no_mangle]
pub static mut ixheaacd_mdst_fcoeff_stopstart_kbd_sin: [WORD16; 7] = [
    -(6797 as core::ffi::c_int) as WORD16,
    46 as core::ffi::c_int as WORD16,
    -(20808 as core::ffi::c_int) as WORD16,
    0 as core::ffi::c_int as WORD16,
    20808 as core::ffi::c_int as WORD16,
    46 as core::ffi::c_int as WORD16,
    6797 as core::ffi::c_int as WORD16,
];
#[no_mangle]
pub static mut ixheaacd_mdst_fcoeff_stopstart_cur: [[*const WORD16; 2]; 2] = unsafe {
    [
        [
            ixheaacd_mdst_fcoeff_stopstart_sin.as_ptr(),
            ixheaacd_mdst_fcoeff_stopstart_sin_kbd.as_ptr(),
        ],
        [
            ixheaacd_mdst_fcoeff_stopstart_kbd_sin.as_ptr(),
            ixheaacd_mdst_fcoeff_stopstart_kbd.as_ptr(),
        ],
    ]
};
#[no_mangle]
pub static mut ixheaacd_mdst_fcoeff_l_s_start_left_sin: [WORD16; 7] = [
    -(0 as core::ffi::c_int) as WORD16,
    3477 as core::ffi::c_int as WORD16,
    8192 as core::ffi::c_int as WORD16,
    10430 as core::ffi::c_int as WORD16,
    8192 as core::ffi::c_int as WORD16,
    3477 as core::ffi::c_int as WORD16,
    -(0 as core::ffi::c_int) as WORD16,
];
#[no_mangle]
pub static mut ixheaacd_mdst_fcoeff_l_s_start_left_kbd: [WORD16; 7] = [
    1950 as core::ffi::c_int as WORD16,
    4054 as core::ffi::c_int as WORD16,
    6114 as core::ffi::c_int as WORD16,
    6982 as core::ffi::c_int as WORD16,
    6114 as core::ffi::c_int as WORD16,
    4054 as core::ffi::c_int as WORD16,
    1950 as core::ffi::c_int as WORD16,
];
#[no_mangle]
pub static mut ixheaacd_mdst_fcoeff_stop_stopstart_left_sin: [WORD16; 7] = [
    1262 as core::ffi::c_int as WORD16,
    1285 as core::ffi::c_int as WORD16,
    1299 as core::ffi::c_int as WORD16,
    1304 as core::ffi::c_int as WORD16,
    1299 as core::ffi::c_int as WORD16,
    1285 as core::ffi::c_int as WORD16,
    1262 as core::ffi::c_int as WORD16,
];
#[no_mangle]
pub static mut ixheaacd_mdst_fcoeff_stop_stopstart_left_kbd: [WORD16; 7] = [
    857 as core::ffi::c_int as WORD16,
    866 as core::ffi::c_int as WORD16,
    871 as core::ffi::c_int as WORD16,
    873 as core::ffi::c_int as WORD16,
    871 as core::ffi::c_int as WORD16,
    866 as core::ffi::c_int as WORD16,
    857 as core::ffi::c_int as WORD16,
];
#[no_mangle]
pub static mut ixheaacd_mdst_fcoeff_l_s_start_left_prev: [*const WORD16; 2] = unsafe {
    [
        ixheaacd_mdst_fcoeff_l_s_start_left_sin.as_ptr(),
        ixheaacd_mdst_fcoeff_l_s_start_left_kbd.as_ptr(),
    ]
};
#[no_mangle]
pub static mut ixheaacd_mdst_fcoeff_stop_stopstart_left_prev: [*const WORD16; 2] = unsafe {
    [
        ixheaacd_mdst_fcoeff_stop_stopstart_left_sin.as_ptr(),
        ixheaacd_mdst_fcoeff_stop_stopstart_left_kbd.as_ptr(),
    ]
};
pub const ONE_BY_TWO_POW_15: core::ffi::c_double = 0.000030517578125f64;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_usac_cplx_save_prev(
    mut info: *mut ia_sfb_info_struct,
    mut l_spec: *mut WORD32,
    mut r_spec: *mut WORD32,
    mut l_spec_prev: *mut WORD32,
    mut r_spec_prev: *mut WORD32,
) {
    let mut ixheaacd_drc_offset: WORD32 = 0;
    ixheaacd_drc_offset = (*info).samp_per_bk - (*info).bins_per_sbk;
    memcpy(
        l_spec_prev.offset(ixheaacd_drc_offset as isize) as *mut core::ffi::c_void,
        l_spec.offset(ixheaacd_drc_offset as isize) as *const core::ffi::c_void,
        (::core::mem::size_of::<WORD32>() as size_t)
            .wrapping_mul((*info).bins_per_sbk as size_t),
    );
    memcpy(
        r_spec_prev.offset(ixheaacd_drc_offset as isize) as *mut core::ffi::c_void,
        r_spec.offset(ixheaacd_drc_offset as isize) as *const core::ffi::c_void,
        (::core::mem::size_of::<WORD32>() as size_t)
            .wrapping_mul((*info).bins_per_sbk as size_t),
    );
}
unsafe extern "C" fn ixheaacd_cplx_pred_data(
    mut usac_data: *mut ia_usac_data_struct,
    mut pstr_core_coder: *mut ia_usac_tmp_core_coder_struct,
    mut num_window_groups: WORD32,
    mut it_bit_buff: *mut ia_bit_buf_struct,
) -> VOID {
    let mut ptr_huff_code_book: *mut ia_huff_code_book_struct = &mut ixheaacd_book;
    let mut ptr_huff_code_word: *const ia_huff_code_word_struct = (*ptr_huff_code_book)
        .pstr_huff_code_word;
    let mut cplx_pred_all: WORD32 = 0;
    let mut delta_code_time: WORD32 = 0;
    let mut g: WORD32 = 0;
    let mut sfb: WORD32 = 0;
    let mut dpcm_alpha: WORD32 = 0;
    let mut last_alpha_q_re: WORD32 = 0;
    let mut last_alpha_q_im: WORD32 = 0;
    let mut max_sfb_ste: UWORD8 = (*pstr_core_coder).max_sfb_ste as UWORD8;
    let mut alpha_q_re: *mut [WORD32; 136] = ((*usac_data).alpha_q_re).as_mut_ptr()
        as *mut [WORD32; 136];
    let mut alpha_q_im: *mut [WORD32; 136] = ((*usac_data).alpha_q_im).as_mut_ptr()
        as *mut [WORD32; 136];
    let mut alpha_q_re_prev: *mut WORD32 = ((*usac_data).alpha_q_re_prev).as_mut_ptr();
    let mut alpha_q_im_prev: *mut WORD32 = ((*usac_data).alpha_q_im_prev).as_mut_ptr();
    let mut cplx_pred_used: *mut [UWORD8; 136] = ((*usac_data).cplx_pred_used)
        .as_mut_ptr() as *mut [UWORD8; 136];
    cplx_pred_all = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
    if cplx_pred_all == 0 as core::ffi::c_int {
        g = 0 as core::ffi::c_int as WORD32;
        while g < num_window_groups {
            sfb = 0 as core::ffi::c_int as WORD32;
            while sfb < max_sfb_ste as core::ffi::c_int {
                (*cplx_pred_used.offset(g as isize))[sfb as usize] = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    1 as WORD,
                ) as UWORD8;
                if (sfb as core::ffi::c_int + 1 as core::ffi::c_int)
                    < max_sfb_ste as core::ffi::c_int
                {
                    (*cplx_pred_used
                        .offset(
                            g as isize,
                        ))[(sfb as core::ffi::c_int + 1 as core::ffi::c_int) as usize] = (*cplx_pred_used
                        .offset(g as isize))[sfb as usize];
                }
                sfb += SFB_PER_PRED_BAND;
            }
            sfb = max_sfb_ste as WORD32;
            while sfb < SFB_NUM_MAX {
                (*cplx_pred_used.offset(g as isize))[sfb as usize] = 0 as UWORD8;
                sfb += 1;
            }
            g += 1;
        }
    } else {
        g = 0 as core::ffi::c_int as WORD32;
        while g < num_window_groups {
            sfb = 0 as core::ffi::c_int as WORD32;
            while sfb < max_sfb_ste as core::ffi::c_int {
                (*cplx_pred_used.offset(g as isize))[sfb as usize] = 1 as UWORD8;
                sfb += 1;
            }
            sfb = max_sfb_ste as WORD32;
            while sfb < SFB_NUM_MAX {
                (*cplx_pred_used.offset(g as isize))[sfb as usize] = 0 as UWORD8;
                sfb += 1;
            }
            g += 1;
        }
    }
    (*pstr_core_coder).pred_dir = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
    (*pstr_core_coder).complex_coef = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
    if (*pstr_core_coder).complex_coef != 0 {
        if (*usac_data).usac_independency_flg != 0 {
            (*pstr_core_coder).use_prev_frame = 0 as core::ffi::c_int as WORD32;
        } else {
            (*pstr_core_coder).use_prev_frame = ixheaacd_read_bits_buf(
                it_bit_buff,
                1 as WORD,
            );
        }
    }
    if (*usac_data).usac_independency_flg != 0 {
        delta_code_time = 0 as core::ffi::c_int as WORD32;
    } else {
        delta_code_time = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
    }
    g = 0 as core::ffi::c_int as WORD32;
    while g < num_window_groups {
        sfb = 0 as core::ffi::c_int as WORD32;
        while sfb < max_sfb_ste as core::ffi::c_int {
            if delta_code_time == 1 as core::ffi::c_int {
                last_alpha_q_re = *alpha_q_re_prev.offset(sfb as isize);
                last_alpha_q_im = *alpha_q_im_prev.offset(sfb as isize);
            } else if sfb > 0 as core::ffi::c_int {
                last_alpha_q_re = (*alpha_q_re
                    .offset(
                        g as isize,
                    ))[(sfb as core::ffi::c_int - 1 as core::ffi::c_int) as usize];
                last_alpha_q_im = (*alpha_q_im
                    .offset(
                        g as isize,
                    ))[(sfb as core::ffi::c_int - 1 as core::ffi::c_int) as usize];
            } else {
                last_alpha_q_im = 0 as core::ffi::c_int as WORD32;
                last_alpha_q_re = last_alpha_q_im;
            }
            if (*cplx_pred_used.offset(g as isize))[sfb as usize] as core::ffi::c_int
                == 1 as core::ffi::c_int
            {
                dpcm_alpha = (-(ixheaacd_huff_codeword(
                    ptr_huff_code_word,
                    0 as UWORD16,
                    it_bit_buff,
                ) as core::ffi::c_int) + 60 as core::ffi::c_int) as WORD32;
                (*alpha_q_re.offset(g as isize))[sfb as usize] = dpcm_alpha
                    + last_alpha_q_re;
                if (*pstr_core_coder).complex_coef != 0 {
                    dpcm_alpha = (-(ixheaacd_huff_codeword(
                        ptr_huff_code_word,
                        0 as UWORD16,
                        it_bit_buff,
                    ) as core::ffi::c_int) + 60 as core::ffi::c_int) as WORD32;
                    (*alpha_q_im.offset(g as isize))[sfb as usize] = dpcm_alpha
                        + last_alpha_q_im;
                } else {
                    (*alpha_q_im.offset(g as isize))[sfb as usize] = 0
                        as core::ffi::c_int as WORD32;
                }
            } else {
                (*alpha_q_re.offset(g as isize))[sfb as usize] = 0 as core::ffi::c_int
                    as WORD32;
                (*alpha_q_im.offset(g as isize))[sfb as usize] = 0 as core::ffi::c_int
                    as WORD32;
            }
            if (sfb as core::ffi::c_int + 1 as core::ffi::c_int)
                < max_sfb_ste as core::ffi::c_int
            {
                (*alpha_q_re
                    .offset(
                        g as isize,
                    ))[(sfb as core::ffi::c_int + 1 as core::ffi::c_int) as usize] = (*alpha_q_re
                    .offset(g as isize))[sfb as usize];
                (*alpha_q_im
                    .offset(
                        g as isize,
                    ))[(sfb as core::ffi::c_int + 1 as core::ffi::c_int) as usize] = (*alpha_q_im
                    .offset(g as isize))[sfb as usize];
            }
            *alpha_q_re_prev.offset(sfb as isize) = (*alpha_q_re
                .offset(g as isize))[sfb as usize];
            *alpha_q_im_prev.offset(sfb as isize) = (*alpha_q_im
                .offset(g as isize))[sfb as usize];
            sfb += SFB_PER_PRED_BAND;
        }
        sfb = max_sfb_ste as WORD32;
        while sfb < SFB_NUM_MAX {
            (*alpha_q_re.offset(g as isize))[sfb as usize] = 0 as core::ffi::c_int
                as WORD32;
            (*alpha_q_im.offset(g as isize))[sfb as usize] = 0 as core::ffi::c_int
                as WORD32;
            *alpha_q_re_prev.offset(sfb as isize) = 0 as core::ffi::c_int as WORD32;
            *alpha_q_im_prev.offset(sfb as isize) = 0 as core::ffi::c_int as WORD32;
            sfb += 1;
        }
        g += 1;
    }
}
unsafe extern "C" fn ixheaacd_read_ms_mask(
    mut usac_data: *mut ia_usac_data_struct,
    mut pstr_core_coder: *mut ia_usac_tmp_core_coder_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut chn: WORD32,
) -> WORD32 {
    let mut g: WORD32 = 0;
    let mut sfb: WORD32 = 0;
    let mut ms_mask_present: WORD32 = 0;
    let mut sfb_group: *mut UWORD8 = (*usac_data).group_dis[chn as usize];
    let mut max_sfb: UWORD8 = (*pstr_core_coder).max_sfb_ste as UWORD8;
    let mut ms_used: *mut UWORD8 = (*usac_data).ms_used[chn as usize];
    let mut info: *mut ia_sfb_info_struct = (*usac_data).pstr_sfb_info[chn as usize];
    ms_mask_present = ixheaacd_read_bits_buf(it_bit_buff, 2 as WORD);
    match ms_mask_present {
        1 => {
            g = 0 as core::ffi::c_int as WORD32;
            while g < (*info).max_win_len {
                sfb = 0 as core::ffi::c_int as WORD32;
                while sfb < max_sfb as core::ffi::c_int {
                    *ms_used = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD) as UWORD8;
                    ms_used = ms_used.offset(1);
                    sfb += 1;
                }
                while sfb < (*info).sfb_per_sbk {
                    *ms_used = 0 as UWORD8;
                    ms_used = ms_used.offset(1);
                    sfb += 1;
                }
                let fresh2 = sfb_group;
                sfb_group = sfb_group.offset(1);
                g = *fresh2 as WORD32;
            }
        }
        2 => {
            g = 0 as core::ffi::c_int as WORD32;
            while g < (*info).max_win_len {
                sfb = 0 as core::ffi::c_int as WORD32;
                while sfb < (*info).sfb_per_sbk {
                    let fresh3 = ms_used;
                    ms_used = ms_used.offset(1);
                    *fresh3 = 1 as UWORD8;
                    sfb += 1;
                }
                let fresh4 = sfb_group;
                sfb_group = sfb_group.offset(1);
                g = *fresh4 as WORD32;
            }
        }
        3 => {
            ixheaacd_cplx_pred_data(
                usac_data,
                pstr_core_coder,
                (*info).num_groups,
                it_bit_buff,
            );
            return 3 as WORD32;
        }
        0 | _ => {}
    }
    sfb = 0 as core::ffi::c_int as WORD32;
    while sfb < SFB_NUM_MAX {
        (*usac_data).alpha_q_re_prev[sfb as usize] = 0 as core::ffi::c_int as WORD32;
        (*usac_data).alpha_q_im_prev[sfb as usize] = 0 as core::ffi::c_int as WORD32;
        sfb += 1;
    }
    return ms_mask_present;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_ms_stereo(
    mut usac_data: *mut ia_usac_data_struct,
    mut r_spec: *mut WORD32,
    mut l_spec: *mut WORD32,
    mut chn: WORD32,
    mut nband: WORD32,
) -> VOID {
    let mut temp_r: WORD32 = 0;
    let mut temp_l: WORD32 = 0;
    let mut sfb: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut grp: WORD32 = 0;
    let mut grp_len: WORD32 = 0;
    let mut ptr_sfb_info: *mut ia_sfb_info_struct = (*usac_data)
        .pstr_sfb_info[chn as usize];
    let mut ms_used: *mut UWORD8 = (*usac_data).ms_used[chn as usize];
    let mut ixheaacd_drc_offset: WORD32 = 0 as WORD32;
    grp = 0 as core::ffi::c_int as WORD32;
    while grp < (*ptr_sfb_info).num_groups {
        grp_len = 0 as core::ffi::c_int as WORD32;
        while grp_len < (*ptr_sfb_info).group_len[grp as usize] as core::ffi::c_int {
            ixheaacd_drc_offset = 0 as core::ffi::c_int as WORD32;
            sfb = 0 as core::ffi::c_int as WORD32;
            while sfb < nband {
                ixheaacd_drc_offset
                    += *((*ptr_sfb_info).sfb_width).offset(sfb as isize)
                        as core::ffi::c_int;
                if *ms_used.offset(sfb as isize) != 0 {
                    k = 0 as core::ffi::c_int as WORD32;
                    while k
                        < *((*ptr_sfb_info).sfb_width).offset(sfb as isize)
                            as core::ffi::c_int
                    {
                        temp_r = *r_spec;
                        temp_l = *l_spec;
                        *l_spec = ixheaac_add32_sat(temp_r, temp_l);
                        *r_spec = ixheaac_sub32_sat(temp_l, temp_r);
                        r_spec = r_spec.offset(1);
                        l_spec = l_spec.offset(1);
                        k += 1;
                    }
                } else {
                    r_spec = r_spec
                        .offset(
                            *((*ptr_sfb_info).sfb_width).offset(sfb as isize)
                                as core::ffi::c_int as isize,
                        );
                    l_spec = l_spec
                        .offset(
                            *((*ptr_sfb_info).sfb_width).offset(sfb as isize)
                                as core::ffi::c_int as isize,
                        );
                }
                sfb += 1;
            }
            l_spec = l_spec
                .offset((*ptr_sfb_info).bins_per_sbk as isize)
                .offset(-(ixheaacd_drc_offset as isize));
            r_spec = r_spec
                .offset((*ptr_sfb_info).bins_per_sbk as isize)
                .offset(-(ixheaacd_drc_offset as isize));
            grp_len += 1;
        }
        ms_used = ms_used.offset((*ptr_sfb_info).sfb_per_sbk as isize);
        grp += 1;
    }
}
unsafe extern "C" fn ixheaacd_filter_and_add(
    mut in_0: *const WORD32,
    length: WORD32,
    mut filter: *const WORD16,
    mut out: *mut WORD32,
    factor_even: WORD32,
    factor_odd: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut sum: WORD64 = 0;
    sum = ixheaac_mult32x32in64(
        *in_0.offset(2 as core::ffi::c_int as isize),
        *filter.offset(0 as core::ffi::c_int as isize) as WORD32,
    );
    sum = ixheaac_mac32x32in64(
        sum,
        *in_0.offset(1 as core::ffi::c_int as isize),
        *filter.offset(1 as core::ffi::c_int as isize) as WORD32,
    );
    sum = ixheaac_mac32x32in64(
        sum,
        *in_0.offset(0 as core::ffi::c_int as isize),
        *filter.offset(2 as core::ffi::c_int as isize) as WORD32,
    );
    sum = ixheaac_mac32x32in64_n(
        sum,
        &*in_0.offset(0 as core::ffi::c_int as isize),
        &*filter.offset(3 as core::ffi::c_int as isize),
        4 as WORD32,
    );
    *out = ixheaac_add32_sat(
        *out,
        ixheaac_sat64_32(sum * factor_even as WORD64 >> 15 as core::ffi::c_int),
    );
    out = out.offset(1);
    sum = ixheaac_mult32x32in64(
        *in_0.offset(1 as core::ffi::c_int as isize),
        *filter.offset(0 as core::ffi::c_int as isize) as WORD32,
    );
    sum = ixheaac_mac32x32in64(
        sum,
        *in_0.offset(0 as core::ffi::c_int as isize),
        *filter.offset(1 as core::ffi::c_int as isize) as WORD32,
    );
    sum = ixheaac_mac32x32in64_n(
        sum,
        &*in_0.offset(0 as core::ffi::c_int as isize),
        &*filter.offset(2 as core::ffi::c_int as isize),
        5 as WORD32,
    );
    *out = ixheaac_add32_sat(
        *out,
        ixheaac_sat64_32(sum * factor_odd as WORD64 >> 15 as core::ffi::c_int),
    );
    out = out.offset(1);
    sum = ixheaac_mult32x32in64(
        *in_0.offset(0 as core::ffi::c_int as isize),
        *filter.offset(0 as core::ffi::c_int as isize) as WORD32,
    );
    sum = ixheaac_mac32x32in64_n(
        sum,
        &*in_0.offset(0 as core::ffi::c_int as isize),
        &*filter.offset(1 as core::ffi::c_int as isize),
        6 as WORD32,
    );
    *out = ixheaac_add32_sat(
        *out,
        ixheaac_sat64_32(sum * factor_even as WORD64 >> 15 as core::ffi::c_int),
    );
    out = out.offset(1);
    i = 3 as core::ffi::c_int as WORD32;
    while i < length as core::ffi::c_int - 4 as core::ffi::c_int {
        sum = 0 as WORD64;
        sum = ixheaac_mac32x32in64_7(
            &*in_0.offset((i as core::ffi::c_int - 3 as core::ffi::c_int) as isize),
            filter,
        );
        *out = ixheaac_add32_sat(
            *out,
            ixheaac_sat64_32(sum * factor_odd as WORD64 >> 15 as core::ffi::c_int),
        );
        out = out.offset(1);
        sum = 0 as WORD64;
        sum = ixheaac_mac32x32in64_7(
            &*in_0.offset((i as core::ffi::c_int - 2 as core::ffi::c_int) as isize),
            filter,
        );
        *out = ixheaac_add32_sat(
            *out,
            ixheaac_sat64_32(sum * factor_even as WORD64 >> 15 as core::ffi::c_int),
        );
        out = out.offset(1);
        i += 2 as core::ffi::c_int;
    }
    i = (length as core::ffi::c_int - 3 as core::ffi::c_int) as WORD32;
    sum = 0 as WORD64;
    sum = ixheaac_mac32x32in64_n(
        sum,
        &*in_0.offset((i as core::ffi::c_int - 3 as core::ffi::c_int) as isize),
        filter,
        6 as WORD32,
    );
    sum = ixheaac_mac32x32in64(
        sum,
        *in_0.offset((i as core::ffi::c_int + 2 as core::ffi::c_int) as isize),
        *filter.offset(6 as core::ffi::c_int as isize) as WORD32,
    );
    *out = ixheaac_add32_sat(
        *out,
        ixheaac_sat64_32(sum * factor_odd as WORD64 >> 15 as core::ffi::c_int),
    );
    out = out.offset(1);
    i = (length as core::ffi::c_int - 2 as core::ffi::c_int) as WORD32;
    sum = 0 as WORD64;
    sum = ixheaac_mac32x32in64_n(
        sum,
        &*in_0.offset((i as core::ffi::c_int - 3 as core::ffi::c_int) as isize),
        filter,
        5 as WORD32,
    );
    sum = ixheaac_mac32x32in64(
        sum,
        *in_0.offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize),
        *filter.offset(5 as core::ffi::c_int as isize) as WORD32,
    );
    sum = ixheaac_mac32x32in64(
        sum,
        *in_0.offset(i as isize),
        *filter.offset(6 as core::ffi::c_int as isize) as WORD32,
    );
    *out = ixheaac_add32_sat(
        *out,
        ixheaac_sat64_32(sum * factor_even as WORD64 >> 15 as core::ffi::c_int),
    );
    out = out.offset(1);
    i = (length as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    sum = 0 as WORD64;
    sum = ixheaac_mac32x32in64_n(
        sum,
        &*in_0.offset((i as core::ffi::c_int - 3 as core::ffi::c_int) as isize),
        filter,
        4 as WORD32,
    );
    sum = ixheaac_mac32x32in64(
        sum,
        *in_0.offset(i as isize),
        *filter.offset(4 as core::ffi::c_int as isize) as WORD32,
    );
    sum = ixheaac_mac32x32in64(
        sum,
        *in_0.offset((i as core::ffi::c_int - 1 as core::ffi::c_int) as isize),
        *filter.offset(5 as core::ffi::c_int as isize) as WORD32,
    );
    sum = ixheaac_mac32x32in64(
        sum,
        *in_0.offset((i as core::ffi::c_int - 2 as core::ffi::c_int) as isize),
        *filter.offset(6 as core::ffi::c_int as isize) as WORD32,
    );
    *out = ixheaac_add32_sat(
        *out,
        ixheaac_sat64_32(sum * factor_odd as WORD64 >> 15 as core::ffi::c_int),
    );
}
unsafe extern "C" fn ixheaacd_estimate_dmx_im(
    mut dmx_re: *const WORD32,
    mut dmx_re_prev: *const WORD32,
    mut dmx_im: *mut WORD32,
    mut pstr_sfb_info: *mut ia_sfb_info_struct,
    mut window: WORD32,
    w_shape: WORD32,
    prev_w_shape: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut mdst_fcoeff_curr: *const WORD16 = 0 as *const WORD16;
    let mut mdst_fcoeff_prev: *const WORD16 = 0 as *const WORD16;
    match window {
        ONLY_LONG_SEQUENCE | EIGHT_SHORT_SEQUENCE => {
            mdst_fcoeff_curr = ixheaacd_mdst_fcoeff_longshort_curr[prev_w_shape
                as usize][w_shape as usize];
            mdst_fcoeff_prev = ixheaacd_mdst_fcoeff_l_s_start_left_prev[prev_w_shape
                as usize];
        }
        LONG_START_SEQUENCE => {
            mdst_fcoeff_curr = ixheaacd_mdst_fcoeff_start_curr[prev_w_shape
                as usize][w_shape as usize];
            mdst_fcoeff_prev = ixheaacd_mdst_fcoeff_l_s_start_left_prev[prev_w_shape
                as usize];
        }
        LONG_STOP_SEQUENCE => {
            mdst_fcoeff_curr = ixheaacd_mdst_fcoeff_stop_cur[prev_w_shape
                as usize][w_shape as usize];
            mdst_fcoeff_prev = ixheaacd_mdst_fcoeff_stop_stopstart_left_prev[prev_w_shape
                as usize];
        }
        STOP_START_SEQUENCE => {
            mdst_fcoeff_curr = ixheaacd_mdst_fcoeff_stopstart_cur[prev_w_shape
                as usize][w_shape as usize];
            mdst_fcoeff_prev = ixheaacd_mdst_fcoeff_stop_stopstart_left_prev[prev_w_shape
                as usize];
        }
        _ => {
            mdst_fcoeff_curr = ixheaacd_mdst_fcoeff_stopstart_cur[prev_w_shape
                as usize][w_shape as usize];
            mdst_fcoeff_prev = ixheaacd_mdst_fcoeff_stop_stopstart_left_prev[prev_w_shape
                as usize];
        }
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*pstr_sfb_info).max_win_len {
        ixheaacd_filter_and_add(
            dmx_re,
            (*pstr_sfb_info).bins_per_sbk,
            mdst_fcoeff_curr,
            dmx_im,
            1 as WORD32,
            1 as WORD32,
        );
        if !dmx_re_prev.is_null() {
            ixheaacd_filter_and_add(
                dmx_re_prev,
                (*pstr_sfb_info).bins_per_sbk,
                mdst_fcoeff_prev,
                dmx_im,
                -(1 as WORD32),
                1 as WORD32,
            );
        }
        dmx_re_prev = dmx_re;
        dmx_re = dmx_re.offset((*pstr_sfb_info).bins_per_sbk as isize);
        dmx_im = dmx_im.offset((*pstr_sfb_info).bins_per_sbk as isize);
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_cplx_pred_upmixing(
    mut usac_data: *mut ia_usac_data_struct,
    mut l_spec: *mut WORD32,
    mut r_spec: *mut WORD32,
    mut pstr_core_coder: *mut ia_usac_tmp_core_coder_struct,
    mut chn: WORD32,
) -> VOID {
    let mut pstr_sfb_info: *mut ia_sfb_info_struct = (*usac_data)
        .pstr_sfb_info[chn as usize];
    let mut dmx_re: *mut WORD32 = &mut *((*usac_data).scratch_buffer)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
    let mut dmx_im: *mut WORD32 = &mut *((*usac_data).x_ac_dec)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
    let mut grp: WORD32 = 0;
    let mut sfb: WORD32 = 0;
    let mut grp_len: WORD32 = 0;
    let mut i: WORD32 = 0 as WORD32;
    let mut k: WORD32 = 0;
    let mut dmx_re_prev: *mut WORD32 = ((*usac_data).dmx_re_prev).as_mut_ptr();
    let mut alpha_q_re: *mut [WORD32; 136] = ((*usac_data).alpha_q_re).as_mut_ptr()
        as *mut [WORD32; 136];
    let mut alpha_q_im: *mut [WORD32; 136] = ((*usac_data).alpha_q_im).as_mut_ptr()
        as *mut [WORD32; 136];
    let mut cplx_pred_used: *mut [UWORD8; 136] = ((*usac_data).cplx_pred_used)
        .as_mut_ptr() as *mut [UWORD8; 136];
    let mut alpha_q_re_temp: WORD32 = 0;
    let mut alpha_q_im_temp: WORD32 = 0;
    let mut factor: WORD32 = 1 as WORD32;
    if (*pstr_core_coder).pred_dir != 0 {
        factor = -(1 as core::ffi::c_int) as WORD32;
    }
    grp = 0 as core::ffi::c_int as WORD32;
    while grp < (*pstr_sfb_info).num_groups {
        grp_len = 0 as core::ffi::c_int as WORD32;
        while grp_len < (*pstr_sfb_info).group_len[grp as usize] as core::ffi::c_int {
            sfb = 0 as core::ffi::c_int as WORD32;
            while sfb < (*pstr_sfb_info).sfb_per_sbk {
                if (*cplx_pred_used.offset(grp as isize))[sfb as usize]
                    as core::ffi::c_int == 1 as core::ffi::c_int
                {
                    memcpy(
                        &mut *dmx_re.offset(i as isize) as *mut WORD32
                            as *mut core::ffi::c_void,
                        &mut *l_spec.offset(i as isize) as *mut WORD32
                            as *const core::ffi::c_void,
                        (*((*pstr_sfb_info).sfb_width).offset(sfb as isize) as size_t)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
                    );
                    i
                        += *((*pstr_sfb_info).sfb_width).offset(sfb as isize)
                            as core::ffi::c_int;
                } else {
                    k = 0 as core::ffi::c_int as WORD32;
                    while k
                        < *((*pstr_sfb_info).sfb_width).offset(sfb as isize)
                            as core::ffi::c_int
                    {
                        *dmx_re.offset(i as isize) = (*l_spec.offset(i as isize)
                            as WORD64
                            + factor as WORD64 * *r_spec.offset(i as isize) as WORD64
                            >> 1 as core::ffi::c_int) as WORD32;
                        k += 1;
                        i += 1;
                    }
                }
                sfb += 1;
            }
            grp_len += 1;
        }
        grp += 1;
    }
    memset(
        dmx_im as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<WORD32>() as size_t)
            .wrapping_mul(BLOCK_LEN_LONG as size_t),
    );
    if (*pstr_core_coder).complex_coef != 0 {
        let mut p_dmx_re_prev: *mut WORD32 = if (*pstr_core_coder).use_prev_frame != 0 {
            dmx_re_prev
        } else {
            0 as *mut WORD32
        };
        ixheaacd_estimate_dmx_im(
            dmx_re,
            p_dmx_re_prev,
            dmx_im,
            pstr_sfb_info,
            (*usac_data).window_sequence[chn as usize],
            (*usac_data).window_shape[chn as usize],
            (*usac_data).window_shape_prev[chn as usize],
        );
        grp = 0 as core::ffi::c_int as WORD32;
        i = 0 as core::ffi::c_int as WORD32;
        while grp < (*pstr_sfb_info).num_groups {
            grp_len = 0 as core::ffi::c_int as WORD32;
            while grp_len < (*pstr_sfb_info).group_len[grp as usize] as core::ffi::c_int
            {
                sfb = 0 as core::ffi::c_int as WORD32;
                while sfb < (*pstr_sfb_info).sfb_per_sbk {
                    alpha_q_re_temp = ixheaac_sat64_32(
                        ixheaac_mult32x32in64(
                            (*alpha_q_re.offset(grp as isize))[sfb as usize],
                            1677722 as WORD32,
                        ),
                    );
                    alpha_q_im_temp = ixheaac_sat64_32(
                        ixheaac_mult32x32in64(
                            (*alpha_q_im.offset(grp as isize))[sfb as usize],
                            1677722 as WORD32,
                        ),
                    );
                    if (*cplx_pred_used.offset(grp as isize))[sfb as usize] != 0 {
                        k = 0 as core::ffi::c_int as WORD32;
                        while k
                            < *((*pstr_sfb_info).sfb_width).offset(sfb as isize)
                                as core::ffi::c_int
                        {
                            let mut mid_side: WORD32 = ixheaac_sub32_sat(
                                ixheaac_sub32_sat(
                                    *r_spec.offset(i as isize),
                                    (ixheaac_mult32x32in64(
                                        alpha_q_re_temp,
                                        *l_spec.offset(i as isize),
                                    ) >> 24 as core::ffi::c_int) as WORD32,
                                ),
                                (ixheaac_mult32x32in64(
                                    alpha_q_im_temp,
                                    *dmx_im.offset(i as isize),
                                ) >> 24 as core::ffi::c_int) as WORD32,
                            );
                            *r_spec.offset(i as isize) = (ixheaac_sat64_32(
                                factor as WORD64,
                            ) as WORD64
                                * ixheaac_sub32_sat(*l_spec.offset(i as isize), mid_side)
                                    as WORD64) as WORD32;
                            *l_spec.offset(i as isize) = ixheaac_add32_sat(
                                *l_spec.offset(i as isize),
                                mid_side,
                            );
                            k += 1;
                            i += 1;
                        }
                    } else {
                        i
                            += *((*pstr_sfb_info).sfb_width).offset(sfb as isize)
                                as core::ffi::c_int;
                    }
                    sfb += 1;
                }
                grp_len += 1;
            }
            grp += 1;
        }
    } else {
        grp = 0 as core::ffi::c_int as WORD32;
        i = 0 as core::ffi::c_int as WORD32;
        while grp < (*pstr_sfb_info).num_groups {
            grp_len = 0 as core::ffi::c_int as WORD32;
            while grp_len < (*pstr_sfb_info).group_len[grp as usize] as core::ffi::c_int
            {
                sfb = 0 as core::ffi::c_int as WORD32;
                while sfb < (*pstr_sfb_info).sfb_per_sbk {
                    alpha_q_re_temp = ixheaac_sat64_32(
                        ixheaac_mult32x32in64(
                            (*alpha_q_re.offset(grp as isize))[sfb as usize],
                            1677722 as WORD32,
                        ),
                    );
                    if (*cplx_pred_used.offset(grp as isize))[sfb as usize] != 0 {
                        k = 0 as core::ffi::c_int as WORD32;
                        while k
                            < *((*pstr_sfb_info).sfb_width).offset(sfb as isize)
                                as core::ffi::c_int
                        {
                            let mut mid_side_0: WORD32 = ixheaac_sub32_sat(
                                *r_spec.offset(i as isize),
                                (ixheaac_mult32x32in64(
                                    alpha_q_re_temp,
                                    *l_spec.offset(i as isize),
                                ) >> 24 as core::ffi::c_int) as WORD32,
                            );
                            *r_spec.offset(i as isize) = (ixheaac_sat64_32(
                                factor as WORD64,
                            ) as WORD64
                                * ixheaac_sub32_sat(*l_spec.offset(i as isize), mid_side_0)
                                    as WORD64) as WORD32;
                            *l_spec.offset(i as isize) = ixheaac_add32_sat(
                                *l_spec.offset(i as isize),
                                mid_side_0,
                            );
                            k += 1;
                            i += 1;
                        }
                    } else {
                        i
                            += *((*pstr_sfb_info).sfb_width).offset(sfb as isize)
                                as core::ffi::c_int;
                    }
                    sfb += 1;
                }
                grp_len += 1;
            }
            grp += 1;
        }
    };
}
unsafe extern "C" fn ixheaacd_cplx_prev_mdct_dmx(
    mut pstr_sfb_info: *mut ia_sfb_info_struct,
    mut l_spec: *mut WORD32,
    mut r_spec: *mut WORD32,
    mut dmx_re_prev: *mut WORD32,
    mut pred_dir: WORD32,
) -> VOID {
    let mut offs: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut factor: WORD32 = 1 as WORD32;
    if pred_dir != 0 {
        factor = -(1 as core::ffi::c_int) as WORD32;
    }
    offs = (*pstr_sfb_info).samp_per_bk - (*pstr_sfb_info).bins_per_sbk;
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*pstr_sfb_info).bins_per_sbk {
        *dmx_re_prev.offset(i as isize) = (*l_spec.offset((i + offs) as isize) as WORD64
            + factor as WORD64 * *r_spec.offset((i + offs) as isize) as WORD64
            >> 1 as core::ffi::c_int) as WORD32;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_ics_info(
    mut usac_data: *mut ia_usac_data_struct,
    mut chn: WORD32,
    mut max_sfb: *mut UWORD8,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut window_sequence_last: WORD32,
) -> WORD32 {
    let mut win: WORD32 = 0;
    let mut mask: WORD32 = 0x40 as WORD32;
    let mut scf_group_ptr: *mut UWORD8 = (*usac_data).group_dis[chn as usize];
    win = ixheaacd_read_bits_buf(it_bit_buff, 2 as WORD);
    (*usac_data).window_sequence[chn as usize] = ixheaacd_win_seq_select(
        win,
        window_sequence_last,
    );
    win = (*usac_data).window_sequence[chn as usize];
    if win == -(1 as core::ffi::c_int) {
        return -(1 as WORD32);
    }
    (*usac_data).pstr_sfb_info[chn as usize] = (*usac_data)
        .pstr_usac_winmap[(*usac_data).window_sequence[chn as usize] as usize];
    (*usac_data).window_shape[chn as usize] = ixheaacd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    );
    if (*(*usac_data).pstr_usac_winmap[win as usize]).islong != 0 {
        *max_sfb = ixheaacd_read_bits_buf(it_bit_buff, 6 as WORD) as UWORD8;
        *scf_group_ptr = 1 as UWORD8;
    } else {
        let mut i: WORD32 = 0;
        let mut scale_factor_grouping: WORD32 = 0;
        *max_sfb = ixheaacd_read_bits_buf(it_bit_buff, 4 as WORD) as UWORD8;
        scale_factor_grouping = ixheaacd_read_bits_buf(it_bit_buff, 7 as WORD);
        i = 1 as core::ffi::c_int as WORD32;
        while i < 8 as core::ffi::c_int {
            if scale_factor_grouping & mask == 0 {
                let fresh0 = scf_group_ptr;
                scf_group_ptr = scf_group_ptr.offset(1);
                *fresh0 = i as UWORD8;
            }
            mask = mask >> 1 as core::ffi::c_int;
            i += 1;
        }
        let fresh1 = scf_group_ptr;
        scf_group_ptr = scf_group_ptr.offset(1);
        *fresh1 = i as UWORD8;
        ixheaacd_calc_grp_offset(
            (*usac_data).pstr_usac_winmap[win as usize],
            &mut *(*((*usac_data).group_dis).as_mut_ptr().offset(chn as isize))
                .offset(0 as core::ffi::c_int as isize),
        );
    }
    if *max_sfb as core::ffi::c_int
        > (*(*usac_data).pstr_sfb_info[chn as usize]).sfb_per_sbk
    {
        *max_sfb = (*(*usac_data).pstr_sfb_info[chn as usize]).sfb_per_sbk as UWORD8;
        return -(1 as WORD32);
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_core_coder_data(
    mut id: WORD32,
    mut usac_data: *mut ia_usac_data_struct,
    mut elem_idx: WORD32,
    mut chan_offset: WORD32,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut nr_core_coder_channels: WORD32,
) -> WORD32 {
    let mut err_code: WORD32 = 0 as WORD32;
    let mut k: WORD32 = 0 as WORD32;
    let mut ch: WORD32 = 0 as WORD32;
    let mut chn: WORD32 = 0 as WORD32;
    let mut left: WORD32 = 0 as WORD32;
    let mut right: WORD32 = 0 as WORD32;
    let mut str_tmp_core_coder: ia_usac_tmp_core_coder_struct = {
        let mut init = ia_usac_tmp_core_coder_struct {
            core_mode: [0 as core::ffi::c_int; 2],
            common_tw: 0,
            common_window: 0,
            tns_data_present: [0; 2],
            tns_active: 0,
            common_tns: 0,
            tns_on_lr: 0,
            tns_present_both: 0,
            common_max_sfb: 0,
            max_sfb: [0; 2],
            max_sfb_ste: 0,
            pred_dir: 0,
            complex_coef: 0,
            use_prev_frame: 0,
            ms_mask_present: [0; 2],
        };
        init
    };
    let mut pstr_core_coder: *mut ia_usac_tmp_core_coder_struct = &mut str_tmp_core_coder;
    let mut td_frame: ia_td_frame_data_struct = ia_td_frame_data_struct {
        acelp_core_mode: 0,
        mod_0: [0; 4],
        fac: [0; 512],
        fac_data: [0; 129],
        mean_energy: [0; 4],
        acb_index: [0; 16],
        noise_factor: [0; 4],
        global_gain: [0; 4],
        arith_reset_flag: 0,
        x_tcx_invquant: [0; 1024],
        tcx_lg: [0; 16],
        ltp_filtering_flag: [0; 16],
        icb_index: [[0; 8]; 16],
        gains: [0; 16],
        mode_lpc: [0; 4],
        lpc_first_approx_idx: [0; 110],
        lsp_coeff: [[0.; 16]; 5],
        lsf_adaptive_mean_cand: [0.; 16],
        lsf_adaptive_mean: [0.; 16],
        lpc4_lsf: [0.; 16],
    };
    let mut local: jmp_buf = [__jmp_buf_tag {
        __jmpbuf: [0; 8],
        __mask_was_saved: 0,
        __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1];
    if (*usac_data).ec_flag != 0 {
        err_code = _setjmp(local.as_mut_ptr()) as WORD32;
        (*it_bit_buff).xaac_jmp_buf = &mut local;
    }
    if err_code == 0 as core::ffi::c_int
        && ((*usac_data).ec_flag == 0 as core::ffi::c_int
            || (*usac_data).frame_ok == 1 as core::ffi::c_int
                && (*usac_data).ec_flag == 1 as core::ffi::c_int)
    {
        memset(
            &mut td_frame as *mut ia_td_frame_data_struct as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<ia_td_frame_data_struct>() as size_t,
        );
        (*pstr_core_coder).tns_on_lr = 0 as core::ffi::c_int as WORD32;
        (*pstr_core_coder).pred_dir = 0 as core::ffi::c_int as WORD32;
        if id != ID_USAC_LFE {
            ch = 0 as core::ffi::c_int as WORD32;
            while ch < nr_core_coder_channels {
                (*pstr_core_coder).core_mode[ch as usize] = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    1 as WORD,
                );
                ch += 1;
            }
        } else {
            ch = 0 as core::ffi::c_int as WORD32;
            while ch < nr_core_coder_channels {
                (*pstr_core_coder).core_mode[ch as usize] = 0 as core::ffi::c_int
                    as WORD32;
                ch += 1;
            }
        }
        if nr_core_coder_channels == 2 as core::ffi::c_int
            && (*pstr_core_coder).core_mode[0 as core::ffi::c_int as usize]
                == 0 as core::ffi::c_int
            && (*pstr_core_coder).core_mode[1 as core::ffi::c_int as usize]
                == 0 as core::ffi::c_int
        {
            (*pstr_core_coder).tns_active = ixheaacd_read_bits_buf(
                it_bit_buff,
                1 as WORD,
            );
            (*pstr_core_coder).common_window = ixheaacd_read_bits_buf(
                it_bit_buff,
                1 as WORD,
            );
            if (*pstr_core_coder).common_window != 0 {
                left = chan_offset;
                right = (chan_offset as core::ffi::c_int + 1 as core::ffi::c_int)
                    as WORD32;
                err_code = ixheaacd_ics_info(
                    usac_data,
                    left,
                    &mut *((*pstr_core_coder).max_sfb)
                        .as_mut_ptr()
                        .offset(left as isize),
                    it_bit_buff,
                    (*usac_data).window_sequence_last[left as usize],
                );
                if err_code == -(1 as core::ffi::c_int) {
                    if (*usac_data).ec_flag != 0 {
                        memcpy(
                            ((*usac_data).max_sfb).as_mut_ptr()
                                as *mut core::ffi::c_void,
                            ((*pstr_core_coder).max_sfb).as_mut_ptr()
                                as *const core::ffi::c_void,
                            ::core::mem::size_of::<[UWORD8; 2]>() as size_t,
                        );
                        longjmp(
                            (*(*it_bit_buff).xaac_jmp_buf).as_mut_ptr(),
                            IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES,
                        );
                    } else {
                        return err_code
                    }
                }
                (*pstr_core_coder).common_max_sfb = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    1 as WORD,
                );
                if (*pstr_core_coder).common_max_sfb == 0 as core::ffi::c_int {
                    if (*usac_data).window_sequence[left as usize]
                        == EIGHT_SHORT_SEQUENCE
                    {
                        (*pstr_core_coder).max_sfb[right as usize] = ixheaacd_read_bits_buf(
                            it_bit_buff,
                            4 as WORD,
                        ) as UWORD8;
                    } else {
                        (*pstr_core_coder).max_sfb[right as usize] = ixheaacd_read_bits_buf(
                            it_bit_buff,
                            6 as WORD,
                        ) as UWORD8;
                    }
                } else {
                    (*pstr_core_coder).max_sfb[right as usize] = (*pstr_core_coder)
                        .max_sfb[left as usize];
                }
                (*pstr_core_coder).max_sfb_ste = (if (*pstr_core_coder)
                    .max_sfb[left as usize] as core::ffi::c_int
                    > (*pstr_core_coder).max_sfb[right as usize] as core::ffi::c_int
                {
                    (*pstr_core_coder).max_sfb[left as usize] as core::ffi::c_int
                } else {
                    (*pstr_core_coder).max_sfb[right as usize] as core::ffi::c_int
                }) as WORD32;
                (*usac_data).window_sequence[right as usize] = (*usac_data)
                    .window_sequence[left as usize];
                (*usac_data).window_shape[right as usize] = (*usac_data)
                    .window_shape[left as usize];
                memcpy(
                    &mut *(*((*usac_data).group_dis).as_mut_ptr().offset(right as isize))
                        .offset(0 as core::ffi::c_int as isize) as *mut UWORD8
                        as *mut core::ffi::c_void,
                    &mut *(*((*usac_data).group_dis).as_mut_ptr().offset(left as isize))
                        .offset(0 as core::ffi::c_int as isize) as *mut UWORD8
                        as *const core::ffi::c_void,
                    8 as size_t,
                );
                (*usac_data).pstr_sfb_info[right as usize] = (*usac_data)
                    .pstr_sfb_info[left as usize];
                if (*pstr_core_coder).max_sfb[right as usize] as core::ffi::c_int
                    > (*(*usac_data).pstr_sfb_info[right as usize]).sfb_per_sbk
                {
                    (*pstr_core_coder).max_sfb[right as usize] = (*(*usac_data)
                        .pstr_sfb_info[right as usize])
                        .sfb_per_sbk as UWORD8;
                }
                (*pstr_core_coder).ms_mask_present[0 as core::ffi::c_int as usize] = ixheaacd_read_ms_mask(
                    usac_data,
                    pstr_core_coder,
                    it_bit_buff,
                    left,
                ) as UWORD8;
            } else {
                left = chan_offset;
                right = (chan_offset as core::ffi::c_int + 1 as core::ffi::c_int)
                    as WORD32;
                (*pstr_core_coder).ms_mask_present[0 as core::ffi::c_int as usize] = 0
                    as UWORD8;
                (*pstr_core_coder).ms_mask_present[1 as core::ffi::c_int as usize] = 0
                    as UWORD8;
                k = 0 as core::ffi::c_int as WORD32;
                while k < SFB_NUM_MAX {
                    (*usac_data).alpha_q_re_prev[k as usize] = 0 as core::ffi::c_int
                        as WORD32;
                    (*usac_data).alpha_q_im_prev[k as usize] = 0 as core::ffi::c_int
                        as WORD32;
                    k += 1;
                }
            }
            if (*usac_data).tw_mdct[elem_idx as usize] == 1 as core::ffi::c_int {
                (*pstr_core_coder).common_tw = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    1 as WORD,
                );
                if (*pstr_core_coder).common_tw == 1 as core::ffi::c_int {
                    (*usac_data).tw_data_present[left as usize] = ixheaacd_read_bits_buf(
                        it_bit_buff,
                        1 as WORD,
                    );
                    (*usac_data).tw_data_present[right as usize] = (*usac_data)
                        .tw_data_present[left as usize];
                    if (*usac_data).tw_data_present[left as usize] != 0 {
                        k = 0 as core::ffi::c_int as WORD32;
                        while k < NUM_TW_NODES {
                            *((*usac_data).tw_ratio[left as usize]).offset(k as isize) = ixheaacd_read_bits_buf(
                                it_bit_buff,
                                3 as WORD,
                            );
                            *((*usac_data).tw_ratio[right as usize])
                                .offset(k as isize) = *((*usac_data)
                                .tw_ratio[left as usize])
                                .offset(k as isize);
                            k += 1;
                        }
                    }
                }
            }
            if (*pstr_core_coder).tns_active != 0 {
                if (*pstr_core_coder).common_window != 0 {
                    (*pstr_core_coder).common_tns = ixheaacd_read_bits_buf(
                        it_bit_buff,
                        1 as WORD,
                    );
                } else {
                    (*pstr_core_coder).common_tns = 0 as core::ffi::c_int as WORD32;
                }
                (*pstr_core_coder).tns_on_lr = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    1 as WORD,
                );
                if (*pstr_core_coder).common_tns != 0 {
                    ixheaacd_read_tns_u(
                        (*usac_data).pstr_sfb_info[0 as core::ffi::c_int as usize],
                        &mut *(*((*usac_data).pstr_tns)
                            .as_mut_ptr()
                            .offset(left as isize))
                            .offset(0 as core::ffi::c_int as isize),
                        it_bit_buff,
                    );
                    memcpy(
                        &mut *(*((*usac_data).pstr_tns)
                            .as_mut_ptr()
                            .offset(right as isize))
                            .offset(0 as core::ffi::c_int as isize)
                            as *mut ia_tns_frame_info_struct as *mut core::ffi::c_void,
                        &mut *(*((*usac_data).pstr_tns)
                            .as_mut_ptr()
                            .offset(left as isize))
                            .offset(0 as core::ffi::c_int as isize)
                            as *mut ia_tns_frame_info_struct as *const core::ffi::c_void,
                        ::core::mem::size_of::<ia_tns_frame_info_struct>() as size_t,
                    );
                    (*pstr_core_coder)
                        .tns_data_present[0 as core::ffi::c_int as usize] = 2
                        as core::ffi::c_int as WORD32;
                    (*pstr_core_coder)
                        .tns_data_present[1 as core::ffi::c_int as usize] = 2
                        as core::ffi::c_int as WORD32;
                } else {
                    (*pstr_core_coder).tns_present_both = ixheaacd_read_bits_buf(
                        it_bit_buff,
                        1 as WORD,
                    );
                    if (*pstr_core_coder).tns_present_both != 0 {
                        (*pstr_core_coder)
                            .tns_data_present[0 as core::ffi::c_int as usize] = 1
                            as core::ffi::c_int as WORD32;
                        (*pstr_core_coder)
                            .tns_data_present[1 as core::ffi::c_int as usize] = 1
                            as core::ffi::c_int as WORD32;
                    } else {
                        (*pstr_core_coder)
                            .tns_data_present[1 as core::ffi::c_int as usize] = ixheaacd_read_bits_buf(
                            it_bit_buff,
                            1 as WORD,
                        );
                        (*pstr_core_coder)
                            .tns_data_present[0 as core::ffi::c_int as usize] = 1
                            as WORD32
                            - (*pstr_core_coder)
                                .tns_data_present[1 as core::ffi::c_int as usize];
                    }
                }
            } else {
                (*pstr_core_coder).common_tns = 0 as core::ffi::c_int as WORD32;
                (*pstr_core_coder).tns_data_present[0 as core::ffi::c_int as usize] = 0
                    as core::ffi::c_int as WORD32;
                (*pstr_core_coder).tns_data_present[1 as core::ffi::c_int as usize] = 0
                    as core::ffi::c_int as WORD32;
            }
        } else {
            (*pstr_core_coder).common_window = 0 as core::ffi::c_int as WORD32;
            (*pstr_core_coder).common_tw = 0 as core::ffi::c_int as WORD32;
            left = chan_offset;
            right = chan_offset;
            if nr_core_coder_channels == 2 as core::ffi::c_int {
                right = (chan_offset as core::ffi::c_int + 1 as core::ffi::c_int)
                    as WORD32;
            }
        }
        ch = 0 as core::ffi::c_int as WORD32;
        chn = chan_offset;
        while ch < nr_core_coder_channels {
            if (*pstr_core_coder).core_mode[chn as usize] == CORE_MODE_LPD
                && (*usac_data).td_frame_prev[chn as usize] == CORE_MODE_FD
                && (*usac_data).ec_flag != 0
            {
                memcpy(
                    (*usac_data).coef_fix[chn as usize] as *mut core::ffi::c_void,
                    ((*usac_data).str_error_concealment[chn as usize].spectral_coeff)
                        .as_mut_ptr() as *const core::ffi::c_void,
                    ::core::mem::size_of::<[WORD32; 1024]>() as size_t,
                );
                memcpy(
                    ((*usac_data).spec_scale[chn as usize]).as_mut_ptr()
                        as *mut core::ffi::c_void,
                    ((*usac_data).str_error_concealment[chn as usize].q_spec_coeff)
                        .as_mut_ptr() as *const core::ffi::c_void,
                    ::core::mem::size_of::<[WORD16; 128]>() as size_t,
                );
                err_code = ixheaacd_fd_frm_dec(usac_data, chn);
                if err_code == -(1 as core::ffi::c_int) {
                    return err_code;
                }
                k = 0 as core::ffi::c_int as WORD32;
                while k < (*usac_data).ccfl {
                    (*usac_data).time_sample_vector[chn as usize][k as usize] = (*usac_data)
                        .output_data_ptr[chn as usize][k as usize] as FLOAT32
                        * 0.000030517578125f64 as FLOAT32;
                    k += 1;
                }
                memcpy(
                    ((*usac_data).time_sample_vector_prev[chn as usize]).as_mut_ptr()
                        as *mut core::ffi::c_void,
                    ((*usac_data).time_sample_vector[chn as usize]).as_mut_ptr()
                        as *const core::ffi::c_void,
                    ((*usac_data).ccfl as size_t)
                        .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                );
                (*usac_data).window_sequence[ch as usize] = (*usac_data)
                    .str_error_concealment[ch as usize]
                    .win_seq;
                (*usac_data).window_shape[ch as usize] = (*usac_data)
                    .str_error_concealment[ch as usize]
                    .win_shape as WORD32;
                (*usac_data).window_shape_prev[ch as usize] = (*usac_data)
                    .window_shape[ch as usize];
                (*usac_data).window_sequence_last[ch as usize] = (*usac_data)
                    .window_sequence[ch as usize];
            }
            if (*pstr_core_coder).core_mode[ch as usize] == 1 as core::ffi::c_int {
                err_code = ixheaacd_tw_buff_update(
                    usac_data,
                    chn,
                    (*usac_data).str_tddec[chn as usize],
                );
                if err_code == -(1 as core::ffi::c_int) {
                    return err_code;
                }
                if (*usac_data).td_frame_prev[chn as usize] == 0 {
                    ixheaacd_fix2flt_data(
                        usac_data,
                        (*usac_data).str_tddec[chn as usize],
                        chn,
                    );
                }
                k = 0 as core::ffi::c_int as WORD32;
                while k < (*usac_data).ccfl {
                    (*usac_data).time_sample_vector[chn as usize][k as usize] = (*usac_data)
                        .output_data_ptr[chn as usize][k as usize] as FLOAT32
                        * 0.000030517578125f64 as FLOAT32;
                    k += 1;
                }
                (*usac_data).present_chan = chn;
                err_code = ixheaacd_lpd_channel_stream(
                    usac_data,
                    &mut td_frame,
                    it_bit_buff,
                    ((*usac_data).time_sample_vector[chn as usize]).as_mut_ptr(),
                );
                if err_code == -(1 as core::ffi::c_int) {
                    return err_code;
                }
                if (*usac_data).ec_flag != 0 {
                    (*it_bit_buff).xaac_jmp_buf = &mut local;
                }
                if (*usac_data).ec_flag != 0 && (*usac_data).frame_ok != 0 {
                    memcpy(
                        &mut *((*usac_data).td_frame_data_prev)
                            .as_mut_ptr()
                            .offset(chn as isize) as *mut ia_td_frame_data_struct
                            as *mut core::ffi::c_void,
                        &mut td_frame as *mut ia_td_frame_data_struct
                            as *const core::ffi::c_void,
                        ::core::mem::size_of::<ia_td_frame_data_struct>() as size_t,
                    );
                    (*usac_data).core_mode = CORE_MODE_LPD as WORD32;
                }
                k = 0 as core::ffi::c_int as WORD32;
                while k < (*usac_data).ccfl {
                    (*usac_data).output_data_ptr[chn as usize][k as usize] = ((*usac_data)
                        .time_sample_vector[chn as usize][k as usize]
                        * ((1 as core::ffi::c_int as WORD64) << 15 as core::ffi::c_int)
                            as FLOAT32) as WORD32;
                    k += 1;
                }
                (*usac_data).window_shape[chn as usize] = WIN_SEL_0 as WORD32;
                ixheaacd_td_frm_dec(
                    usac_data,
                    chn,
                    td_frame.mod_0[0 as core::ffi::c_int as usize],
                );
                (*usac_data).window_shape_prev[chn as usize] = (*usac_data)
                    .window_shape[chn as usize];
                (*usac_data).window_sequence_last[chn as usize] = EIGHT_SHORT_SEQUENCE
                    as WORD32;
            } else {
                memset(
                    (*usac_data).coef_fix[chn as usize] as *mut core::ffi::c_void,
                    0 as core::ffi::c_int,
                    (LN2 as core::ffi::c_int as size_t)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
                );
                if !((*usac_data).str_tddec[chn as usize]).is_null()
                    && (*usac_data).td_frame_prev[chn as usize] != 0
                {
                    if (*usac_data).ec_flag != 0 {
                        memcpy(
                            ((*usac_data).time_sample_vector[chn as usize]).as_mut_ptr()
                                as *mut core::ffi::c_void,
                            ((*usac_data).time_sample_vector_prev[chn as usize])
                                .as_mut_ptr() as *const core::ffi::c_void,
                            ((*usac_data).ccfl as size_t)
                                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                        );
                    }
                    ixheaacd_lpd_dec_update(
                        (*usac_data).str_tddec[chn as usize],
                        usac_data,
                        chn,
                    );
                }
                if id != ID_USAC_LFE {
                    if nr_core_coder_channels == 1 as core::ffi::c_int
                        || (*pstr_core_coder).core_mode[0 as core::ffi::c_int as usize]
                            != (*pstr_core_coder)
                                .core_mode[1 as core::ffi::c_int as usize]
                    {
                        (*pstr_core_coder).tns_data_present[ch as usize] = ixheaacd_read_bits_buf(
                            it_bit_buff,
                            1 as WORD,
                        );
                    }
                }
                err_code = ixheaacd_fd_channel_stream(
                    usac_data,
                    pstr_core_coder,
                    &mut *((*pstr_core_coder).max_sfb).as_mut_ptr().offset(ch as isize),
                    (*usac_data).window_sequence_last[chn as usize],
                    chn,
                    (*usac_data).noise_filling_config[elem_idx as usize],
                    ch,
                    it_bit_buff,
                );
                if err_code == -(1 as core::ffi::c_int) {
                    return err_code;
                }
            }
            ch += 1;
            chn += 1;
        }
        if (*pstr_core_coder).core_mode[0 as core::ffi::c_int as usize] == CORE_MODE_FD
            && (*pstr_core_coder).core_mode[1 as core::ffi::c_int as usize]
                == CORE_MODE_FD && nr_core_coder_channels == 2 as core::ffi::c_int
        {
            ixheaacd_cplx_prev_mdct_dmx(
                (*usac_data).pstr_sfb_info[left as usize],
                (*usac_data).coef_save[left as usize],
                (*usac_data).coef_save[right as usize],
                ((*usac_data).dmx_re_prev).as_mut_ptr(),
                (*pstr_core_coder).pred_dir,
            );
        }
        if (*pstr_core_coder).tns_on_lr == 0 as core::ffi::c_int && id != ID_USAC_LFE {
            ch = 0 as core::ffi::c_int as WORD32;
            chn = left;
            while chn <= right {
                if (*pstr_core_coder).core_mode[ch as usize] == CORE_MODE_FD {
                    err_code = ixheaacd_tns_apply(
                        usac_data,
                        (*usac_data).coef_fix[chn as usize],
                        (*pstr_core_coder).max_sfb[ch as usize] as WORD32,
                        (*usac_data).pstr_sfb_info[chn as usize],
                        (*usac_data).pstr_tns[chn as usize],
                    ) as WORD32;
                    if err_code != 0 {
                        return err_code;
                    }
                }
                ch += 1;
                chn += 1;
            }
        }
        if nr_core_coder_channels == 2 as core::ffi::c_int
            && (*pstr_core_coder).core_mode[0 as core::ffi::c_int as usize]
                == 0 as core::ffi::c_int
            && (*pstr_core_coder).core_mode[1 as core::ffi::c_int as usize]
                == 0 as core::ffi::c_int
        {
            if (*pstr_core_coder).ms_mask_present[0 as core::ffi::c_int as usize]
                as core::ffi::c_int == 3 as core::ffi::c_int
            {
                ixheaacd_cplx_pred_upmixing(
                    usac_data,
                    (*usac_data).coef_fix[left as usize],
                    (*usac_data).coef_fix[right as usize],
                    pstr_core_coder,
                    left,
                );
            } else if (*pstr_core_coder).ms_mask_present[0 as core::ffi::c_int as usize]
                as core::ffi::c_int > 0 as core::ffi::c_int
            {
                ixheaacd_ms_stereo(
                    usac_data,
                    (*usac_data).coef_fix[right as usize],
                    (*usac_data).coef_fix[left as usize],
                    left,
                    if (*pstr_core_coder).max_sfb[right as usize] as core::ffi::c_int
                        > (*pstr_core_coder).max_sfb[left as usize] as core::ffi::c_int
                    {
                        (*pstr_core_coder).max_sfb[right as usize] as WORD32
                    } else {
                        (*pstr_core_coder).max_sfb[left as usize] as WORD32
                    },
                );
            }
            if (*pstr_core_coder).tns_on_lr != 0 {
                ch = 0 as core::ffi::c_int as WORD32;
                chn = left;
                while chn <= right {
                    if (*pstr_core_coder).core_mode[ch as usize] == CORE_MODE_FD {
                        err_code = ixheaacd_tns_apply(
                            usac_data,
                            (*usac_data).coef_fix[chn as usize],
                            (*pstr_core_coder).max_sfb[ch as usize] as WORD32,
                            (*usac_data).pstr_sfb_info[chn as usize],
                            (*usac_data).pstr_tns[chn as usize],
                        ) as WORD32;
                        if err_code != 0 {
                            return err_code;
                        }
                    }
                    ch += 1;
                    chn += 1;
                }
            }
            ixheaacd_usac_cplx_save_prev(
                (*usac_data).pstr_sfb_info[left as usize],
                (*usac_data).coef_fix[left as usize],
                (*usac_data).coef_fix[right as usize],
                (*usac_data).coef_save[left as usize],
                (*usac_data).coef_save[right as usize],
            );
        }
        if (*usac_data).ec_flag != 0 {
            chn = left;
            while chn <= right {
                if (*pstr_core_coder).core_mode[chn as usize] == CORE_MODE_FD
                    && (*usac_data).td_frame_prev[chn as usize] == CORE_MODE_LPD
                {
                    memcpy(
                        ((*usac_data).str_error_concealment[chn as usize].spectral_coeff)
                            .as_mut_ptr() as *mut core::ffi::c_void,
                        (*usac_data).coef_fix[chn as usize] as *const core::ffi::c_void,
                        ::core::mem::size_of::<[WORD32; 1024]>() as size_t,
                    );
                    memcpy(
                        ((*usac_data).str_error_concealment[chn as usize].q_spec_coeff)
                            .as_mut_ptr() as *mut core::ffi::c_void,
                        ((*usac_data).spec_scale[chn as usize]).as_mut_ptr()
                            as *const core::ffi::c_void,
                        ::core::mem::size_of::<[WORD16; 128]>() as size_t,
                    );
                    (*usac_data).str_error_concealment[chn as usize].win_seq = (*usac_data)
                        .window_sequence[chn as usize];
                    (*usac_data).str_error_concealment[chn as usize].win_shape = (*usac_data)
                        .window_shape[chn as usize] as UWORD8;
                    (*usac_data).str_error_concealment[chn as usize].win_shape_prev = (*usac_data)
                        .window_shape_prev[chn as usize] as UWORD8;
                    (*usac_data).str_error_concealment[chn as usize].td_frame_prev = (*usac_data)
                        .td_frame_prev[chn as usize];
                    (*usac_data).str_error_concealment[chn as usize].fac_data_present = (*usac_data)
                        .fac_data_present[chn as usize];
                }
                chn += 1;
            }
            if (*usac_data).frame_ok != 0 && (*usac_data).ec_flag != 0 {
                memcpy(
                    ((*usac_data).max_sfb).as_mut_ptr() as *mut core::ffi::c_void,
                    ((*pstr_core_coder).max_sfb).as_mut_ptr()
                        as *const core::ffi::c_void,
                    ::core::mem::size_of::<[UWORD8; 2]>() as size_t,
                );
            }
        }
    } else {
        left = chan_offset;
        right = chan_offset;
        if nr_core_coder_channels == 2 as core::ffi::c_int {
            right = (chan_offset as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
        }
        if (*usac_data).ec_flag == 1 as core::ffi::c_int {
            let mut err: WORD32 = 0 as WORD32;
            (*usac_data).frame_ok = 0 as core::ffi::c_int as WORD32;
            ch = left;
            while ch <= right {
                if (*usac_data).td_frame_prev[ch as usize] == CORE_MODE_LPD {
                    (*usac_data).fac_data_present[ch as usize] = 0 as core::ffi::c_int
                        as WORD32;
                    (*usac_data).str_error_concealment[ch as usize].pstr_ec_scratch = &mut (*((*usac_data)
                        .str_error_concealment)
                        .as_mut_ptr()
                        .offset(ch as isize))
                        .str_ec_scratch as *mut ia_ec_scratch_str;
                    (*usac_data).core_mode = (*usac_data).td_frame_prev[ch as usize];
                    (*usac_data).present_chan = ch;
                    ixheaacd_usac_apply_ec(
                        usac_data,
                        0 as *const ia_usac_samp_rate_info,
                        ch,
                    );
                    err = ixheaacd_lpd_dec(
                        usac_data,
                        (*usac_data).str_tddec[ch as usize],
                        &mut *((*usac_data).td_frame_data_prev)
                            .as_mut_ptr()
                            .offset(ch as isize),
                        ((*usac_data).time_sample_vector[ch as usize]).as_mut_ptr(),
                        (*usac_data).first_lpd_flag,
                        0 as WORD32,
                        (*usac_data).bpf_control_info,
                    );
                    if err != 0 {
                        return err;
                    }
                    k = 0 as core::ffi::c_int as WORD32;
                    while k < (*usac_data).ccfl {
                        (*usac_data).output_data_ptr[ch as usize][k as usize] = ((*usac_data)
                            .time_sample_vector[ch as usize][k as usize]
                            * ((1 as core::ffi::c_int as WORD64)
                                << 15 as core::ffi::c_int) as FLOAT32) as WORD32;
                        k += 1;
                    }
                    (*usac_data).window_shape[ch as usize] = WIN_SEL_0 as WORD32;
                    (*usac_data).window_shape_prev[ch as usize] = (*usac_data)
                        .window_shape[ch as usize];
                    (*usac_data).window_sequence_last[ch as usize] = EIGHT_SHORT_SEQUENCE
                        as WORD32;
                } else {
                    (*pstr_core_coder).core_mode[ch as usize] = CORE_MODE_FD as WORD32;
                }
                ch += 1;
            }
        }
    }
    ch = left;
    while ch <= right {
        let mut ptr_scratch: *mut FLOAT32 = ((*usac_data)
            .str_error_concealment[ch as usize]
            .str_ec_scratch
            .spec_coeff)
            .as_mut_ptr() as *mut FLOAT32;
        if (*pstr_core_coder).core_mode[ch as usize] != CORE_MODE_LPD
            && (*usac_data).td_frame_prev[ch as usize] != CORE_MODE_LPD
            && (*usac_data).ec_flag != 0
            || (*pstr_core_coder).core_mode[ch as usize] == CORE_MODE_FD
                && (*usac_data).ec_flag == 0 as core::ffi::c_int
        {
            if (*usac_data).tw_mdct[elem_idx as usize] != 0 {
                err_code = -(1 as core::ffi::c_int) as WORD32;
                return err_code;
            } else {
                if (*usac_data).frame_ok == 0 as core::ffi::c_int {
                    (*usac_data).fac_data_present[ch as usize] = 0 as core::ffi::c_int
                        as WORD32;
                }
                err_code = ixheaacd_fd_frm_dec(usac_data, ch);
                if err_code == -(1 as core::ffi::c_int) {
                    return err_code;
                }
                if (*usac_data).ec_flag != 0 {
                    if (*usac_data).str_error_concealment[ch as usize].fade_idx
                        < MAX_FADE_FRAMES
                    {
                        let mut fade_fac: FLOAT32 = 0.000030517578125f64 as FLOAT32
                            * ia_ec_fade_factors[(*usac_data)
                                .str_error_concealment[ch as usize]
                                .fade_idx as usize];
                        k = 0 as core::ffi::c_int as WORD32;
                        while k < (*usac_data).ccfl {
                            (*usac_data).time_sample_vector[ch as usize][k as usize] = (*usac_data)
                                .output_data_ptr[ch as usize][k as usize] as FLOAT32
                                * fade_fac;
                            k += 1;
                        }
                    } else {
                        memset(
                            &mut *(*((*usac_data).time_sample_vector)
                                .as_mut_ptr()
                                .offset(ch as isize))
                                .as_mut_ptr()
                                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                                as *mut core::ffi::c_void,
                            0 as core::ffi::c_int,
                            ((*usac_data).ccfl as size_t)
                                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                        );
                    }
                } else {
                    k = 0 as core::ffi::c_int as WORD32;
                    while k < (*usac_data).ccfl {
                        (*usac_data).time_sample_vector[ch as usize][k as usize] = (*usac_data)
                            .output_data_ptr[ch as usize][k as usize] as FLOAT32
                            * 0.000030517578125f64 as FLOAT32;
                        k += 1;
                    }
                }
            }
            (*usac_data).window_shape_prev[ch as usize] = (*usac_data)
                .window_shape[ch as usize];
            (*usac_data).window_sequence_last[ch as usize] = (*usac_data)
                .window_sequence[ch as usize];
        } else if (*usac_data).ec_flag != 0 {
            (*usac_data)
                .str_error_concealment[ch as usize]
                .prev_frame_ok[0 as core::ffi::c_int as usize] = (*usac_data)
                .str_error_concealment[ch as usize]
                .prev_frame_ok[1 as core::ffi::c_int as usize];
            (*usac_data)
                .str_error_concealment[ch as usize]
                .prev_frame_ok[1 as core::ffi::c_int as usize] = (*usac_data).frame_ok;
            if (*usac_data).str_error_concealment[ch as usize].fade_idx < MAX_FADE_FRAMES
            {
                let mut fade_fac_0: FLOAT32 = 0.000030517578125f64 as FLOAT32
                    * ia_ec_fade_factors[(*usac_data)
                        .str_error_concealment[ch as usize]
                        .fade_idx as usize];
                k = 0 as core::ffi::c_int as WORD32;
                while k < (*usac_data).ccfl {
                    (*usac_data).time_sample_vector[ch as usize][k as usize] = (*usac_data)
                        .output_data_ptr[ch as usize][k as usize] as FLOAT32
                        * fade_fac_0;
                    k += 1;
                }
            } else {
                memset(
                    &mut *(*((*usac_data).time_sample_vector)
                        .as_mut_ptr()
                        .offset(ch as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                        as *mut core::ffi::c_void,
                    0 as core::ffi::c_int,
                    ((*usac_data).ccfl as size_t)
                        .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                );
            }
            memcpy(
                ptr_scratch as *mut core::ffi::c_void,
                ((*usac_data).time_sample_vector[ch as usize]).as_mut_ptr()
                    as *const core::ffi::c_void,
                ((*usac_data).ccfl as size_t)
                    .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
            );
            memcpy(
                ((*usac_data).time_sample_vector[ch as usize]).as_mut_ptr()
                    as *mut core::ffi::c_void,
                ((*usac_data).time_sample_vector_prev[ch as usize]).as_mut_ptr()
                    as *const core::ffi::c_void,
                ((*usac_data).ccfl as size_t)
                    .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
            );
            memcpy(
                ((*usac_data).time_sample_vector_prev[ch as usize]).as_mut_ptr()
                    as *mut core::ffi::c_void,
                ptr_scratch as *const core::ffi::c_void,
                ((*usac_data).ccfl as size_t)
                    .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
            );
        } else {
            k = 0 as core::ffi::c_int as WORD32;
            while k < (*usac_data).ccfl {
                (*usac_data).time_sample_vector[ch as usize][k as usize] = (*usac_data)
                    .output_data_ptr[ch as usize][k as usize] as FLOAT32
                    * 0.000030517578125f64 as FLOAT32;
                k += 1;
            }
        }
        if (*usac_data).ec_flag != 0 {
            (*usac_data).window_sequence[ch as usize] = (*usac_data)
                .str_error_concealment[ch as usize]
                .win_seq;
            (*usac_data).window_shape[ch as usize] = (*usac_data)
                .str_error_concealment[ch as usize]
                .win_shape as WORD32;
            if (*usac_data).first_frame == 0 as core::ffi::c_int {
                (*usac_data).window_shape_prev[ch as usize] = (*usac_data)
                    .window_shape[ch as usize];
                (*usac_data).window_sequence_last[ch as usize] = (*usac_data)
                    .window_sequence[ch as usize];
            }
        }
        ch += 1;
    }
    if (*usac_data).ec_flag != 0 {
        (*usac_data).first_frame = 0 as core::ffi::c_int as WORD32;
        if (*usac_data).frame_ok == 1 as core::ffi::c_int {
            ch = 0 as core::ffi::c_int as WORD32;
            chn = left;
            while chn <= right {
                (*usac_data).td_frame_prev[chn as usize] = (*pstr_core_coder)
                    .core_mode[ch as usize];
                chn += 1;
                ch += 1;
            }
        }
    } else {
        ch = 0 as core::ffi::c_int as WORD32;
        chn = left;
        while chn <= right {
            (*usac_data).td_frame_prev[chn as usize] = (*pstr_core_coder)
                .core_mode[ch as usize];
            chn += 1;
            ch += 1;
        }
    }
    return 0 as WORD32;
}
