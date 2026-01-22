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
    fn longjmp(__env: *mut __jmp_buf_tag, __val: core::ffi::c_int) -> !;
    fn ixheaacd_read_bits_buf(
        it_bit_buff: *mut ia_bit_buf_struct,
        no_of_bits: WORD,
    ) -> WORD32;
    fn ixheaacd_aac_showbits_32(
        ptr_read_next: *mut UWORD8,
        cnt_bits: WORD32,
        increment: *mut WORD32,
    ) -> UWORD32;
    fn ixheaacd_aac_read_byte_corr1(
        ptr_read_next: *mut *mut UWORD8,
        ptr_bit_pos: *mut WORD32,
        readword: *mut WORD32,
        p_bit_buf_end: *mut UWORD8,
    ) -> VOID;
    fn ixheaacd_ac_spectral_data(
        usac_data: *mut ia_usac_data_struct,
        max_spec_coefficients: WORD32,
        noise_level: WORD32,
        noise_offset: WORD32,
        arth_size: WORD32,
        it_bit_buff: *mut ia_bit_buf_struct,
        max_sfb: UWORD8,
        reset: WORD32,
        noise_filling: WORD32,
        ch: WORD32,
    ) -> WORD32;
    fn ixheaacd_ics_info(
        usac_data: *mut ia_usac_data_struct,
        widx: WORD32,
        max_sfb: *mut UWORD8,
        it_bit_buff: *mut ia_bit_buf_struct,
        window_sequence_last: WORD32,
    ) -> WORD32;
    fn ixheaacd_fac_decoding(
        fac_len: WORD32,
        k: WORD32,
        fac_prm: *mut WORD32,
        it_bit_buff: *mut ia_bit_buf_struct,
    ) -> VOID;
    fn ixheaacd_huffman_decode(
        it_bit_buff: WORD32,
        h_index: *mut WORD16,
        len: *mut WORD16,
        input_table: *const UWORD16,
        idx_table: *const UWORD32,
    ) -> VOID;
}
pub type size_t = usize;
pub type WORD8 = core::ffi::c_schar;
pub type UWORD8 = core::ffi::c_uchar;
pub type pUWORD8 = *mut core::ffi::c_uchar;
pub type WORD16 = core::ffi::c_short;
pub type pWORD16 = *mut core::ffi::c_short;
pub type UWORD16 = core::ffi::c_ushort;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
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
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_calc_grp_offset(
    mut ptr_sfb_info: *mut ia_sfb_info_struct,
    mut group: pUWORD8,
) -> VOID {
    let mut group_offset: WORD32 = 0;
    let mut group_idx: WORD32 = 0;
    let mut ixheaacd_drc_offset: WORD32 = 0;
    let mut group_offset_p: *mut WORD16 = 0 as *mut WORD16;
    let mut sfb: WORD32 = 0;
    let mut len: WORD32 = 0;
    group_offset = 0 as core::ffi::c_int as WORD32;
    group_idx = 0 as core::ffi::c_int as WORD32;
    loop {
        (*ptr_sfb_info).group_len[group_idx as usize] = (*group
            .offset(group_idx as isize) as WORD32 - group_offset) as WORD16;
        group_offset = *group.offset(group_idx as isize) as WORD32;
        group_idx += 1;
        if !(group_offset < 8 as core::ffi::c_int) {
            break;
        }
    }
    (*ptr_sfb_info).num_groups = group_idx;
    group_offset_p = ((*ptr_sfb_info).sfb_idx_tbl).as_mut_ptr();
    ixheaacd_drc_offset = 0 as core::ffi::c_int as WORD32;
    group_idx = 0 as core::ffi::c_int as WORD32;
    while group_idx < (*ptr_sfb_info).num_groups {
        len = (*ptr_sfb_info).group_len[group_idx as usize] as WORD32;
        sfb = 0 as core::ffi::c_int as WORD32;
        while sfb < (*ptr_sfb_info).sfb_per_sbk {
            ixheaacd_drc_offset
                += (*((*ptr_sfb_info).sfb_width).offset(sfb as isize) as WORD32 * len)
                    as core::ffi::c_int;
            let fresh0 = group_offset_p;
            group_offset_p = group_offset_p.offset(1);
            *fresh0 = ixheaacd_drc_offset as WORD16;
            sfb += 1;
        }
        group_idx += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_read_tns_u(
    mut ptr_sfb_info: *mut ia_sfb_info_struct,
    mut pstr_tns_frame_info: *mut ia_tns_frame_info_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
) -> VOID {
    let mut j: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut top: WORD32 = 0;
    let mut coef_res: WORD32 = 0;
    let mut resolution: WORD32 = 0;
    let mut compress: WORD32 = 0;
    let mut short_flag: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut sp: *mut WORD16 = 0 as *mut WORD16;
    let mut tmp: WORD16 = 0;
    let mut s_mask: WORD16 = 0;
    let mut n_mask: WORD16 = 0;
    let mut tns_filt: *mut ia_tns_filter_struct = 0 as *mut ia_tns_filter_struct;
    let mut pstr_tns_info: *mut ia_tns_info_struct = 0 as *mut ia_tns_info_struct;
    static mut sgn_mask: [WORD16; 3] = [
        0x2 as core::ffi::c_int as WORD16,
        0x4 as core::ffi::c_int as WORD16,
        0x8 as core::ffi::c_int as WORD16,
    ];
    static mut neg_mask: [WORD16; 3] = [
        0xfffc as core::ffi::c_int as WORD16,
        0xfff8 as core::ffi::c_int as WORD16,
        0xfff0 as core::ffi::c_int as WORD16,
    ];
    let mut n_filt_bits: WORD16 = 0;
    let mut start_band_bits: WORD16 = 0;
    let mut order_bits: WORD16 = 0;
    short_flag = ((*ptr_sfb_info).islong == 0) as core::ffi::c_int as WORD32;
    (*pstr_tns_frame_info).n_subblocks = (*ptr_sfb_info).max_win_len;
    if short_flag == 0 {
        n_filt_bits = 2 as WORD16;
        start_band_bits = 6 as WORD16;
        order_bits = 4 as WORD16;
    } else {
        n_filt_bits = 1 as WORD16;
        start_band_bits = 4 as WORD16;
        order_bits = 3 as WORD16;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*pstr_tns_frame_info).n_subblocks {
        pstr_tns_info = &mut *((*pstr_tns_frame_info).str_tns_info)
            .as_mut_ptr()
            .offset(i as isize) as *mut ia_tns_info_struct;
        (*pstr_tns_info).n_filt = ixheaacd_read_bits_buf(
            it_bit_buff,
            n_filt_bits as WORD,
        );
        if !((*pstr_tns_info).n_filt == 0) {
            coef_res = (ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD)
                as core::ffi::c_int + 3 as core::ffi::c_int) as WORD32;
            (*pstr_tns_info).coef_res = coef_res;
            top = (*ptr_sfb_info).sfb_per_sbk;
            tns_filt = &mut *((*pstr_tns_info).str_filter)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut ia_tns_filter_struct;
            j = (*pstr_tns_info).n_filt;
            while j > 0 as core::ffi::c_int {
                (*tns_filt).stop_band = top;
                (*tns_filt).start_band = top
                    - ixheaacd_read_bits_buf(it_bit_buff, start_band_bits as WORD);
                top = (*tns_filt).start_band;
                (*tns_filt).order = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    order_bits as WORD,
                );
                if (*tns_filt).order != 0 {
                    (*tns_filt).direction = ixheaacd_read_bits_buf(
                        it_bit_buff,
                        1 as WORD,
                    );
                    compress = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
                    resolution = coef_res - compress;
                    s_mask = sgn_mask[(resolution as core::ffi::c_int
                        - 2 as core::ffi::c_int) as usize];
                    n_mask = neg_mask[(resolution as core::ffi::c_int
                        - 2 as core::ffi::c_int) as usize];
                    sp = ((*tns_filt).coef).as_mut_ptr();
                    k = (*tns_filt).order;
                    while k > 0 as core::ffi::c_int {
                        tmp = ixheaacd_read_bits_buf(it_bit_buff, resolution as WORD)
                            as WORD16;
                        let fresh1 = sp;
                        sp = sp.offset(1);
                        *fresh1 = (if tmp as core::ffi::c_int
                            & s_mask as core::ffi::c_int != 0
                        {
                            tmp as core::ffi::c_int | n_mask as core::ffi::c_int
                        } else {
                            tmp as core::ffi::c_int
                        }) as WORD16;
                        k -= 1;
                    }
                }
                tns_filt = tns_filt.offset(1);
                j -= 1;
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_scale_factor_data(
    mut info: *mut ia_sfb_info_struct,
    mut tot_sfb: WORD32,
    mut max_sfb: WORD32,
    mut sfb_per_sbk: WORD32,
    mut ptr_code_book: *mut WORD8,
) -> VOID {
    let mut band: WORD = 0;
    let mut sect_cb: WORD = 0;
    let mut sect_len: WORD = 0;
    let mut ptr_codebook: *mut WORD8 = ptr_code_book;
    let mut temp_ptr_codebook: *mut WORD8 = ptr_codebook;
    let mut win_group: WORD32 = (*info).max_win_len;
    memset(ptr_codebook as *mut core::ffi::c_void, 0 as core::ffi::c_int, 128 as size_t);
    band = 0 as core::ffi::c_int as WORD;
    while band < tot_sfb || win_group != 0 as core::ffi::c_int {
        sect_cb = 11 as core::ffi::c_int as WORD;
        sect_len = max_sfb as WORD;
        band = (band as WORD32 + sfb_per_sbk) as WORD;
        sect_len = (sect_len as core::ffi::c_int - 1 as core::ffi::c_int) as WORD;
        while sect_len >= 0 as core::ffi::c_int {
            let fresh8 = temp_ptr_codebook;
            temp_ptr_codebook = temp_ptr_codebook.offset(1);
            *fresh8 = sect_cb as WORD8;
            sect_len -= 1;
        }
        ptr_codebook = ptr_codebook.offset(16 as core::ffi::c_int as isize);
        temp_ptr_codebook = ptr_codebook;
        win_group -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_win_seq_select(
    mut window_sequence_curr: WORD32,
    mut window_sequence_last: WORD32,
) -> WORD32 {
    let mut window_sequence: WORD32 = 0;
    match window_sequence_curr {
        ONLY_LONG_SEQUENCE => {
            window_sequence = ONLY_LONG_SEQUENCE as WORD32;
        }
        LONG_START_SEQUENCE => {
            if window_sequence_last == LONG_START_SEQUENCE
                || window_sequence_last == EIGHT_SHORT_SEQUENCE
                || window_sequence_last == STOP_START_SEQUENCE
            {
                window_sequence = STOP_START_SEQUENCE as WORD32;
            } else {
                window_sequence = LONG_START_SEQUENCE as WORD32;
            }
        }
        LONG_STOP_SEQUENCE => {
            window_sequence = LONG_STOP_SEQUENCE as WORD32;
        }
        EIGHT_SHORT_SEQUENCE => {
            window_sequence = EIGHT_SHORT_SEQUENCE as WORD32;
        }
        _ => return -(1 as WORD32),
    }
    return window_sequence;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_tns_reset(
    mut ptr_sfb_info: *mut ia_sfb_info_struct,
    mut pstr_tns_frame_info: *mut ia_tns_frame_info_struct,
) -> VOID {
    let mut s: WORD32 = 0;
    (*pstr_tns_frame_info).n_subblocks = (*ptr_sfb_info).max_win_len;
    s = 0 as core::ffi::c_int as WORD32;
    while s < (*pstr_tns_frame_info).n_subblocks {
        (*pstr_tns_frame_info).str_tns_info[s as usize].n_filt = 0 as core::ffi::c_int
            as WORD32;
        s += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_section_data(
    mut usac_data: *mut ia_usac_data_struct,
    mut g_bs: *mut ia_bit_buf_struct,
    mut info: *mut ia_sfb_info_struct,
    mut global_gain: WORD16,
    mut factors: pWORD16,
    mut groups: pUWORD8,
    mut ptr_code_book: *mut WORD8,
) -> VOID {
    let mut band: WORD32 = 0;
    let mut position: WORD16 = 0 as WORD16;
    let mut group: WORD32 = 0;
    let mut factor: WORD16 = global_gain;
    let mut temp_codebook_ptr: *mut WORD8 = 0 as *mut WORD8;
    let mut ptr_scale_fac: *mut WORD16 = 0 as *mut WORD16;
    let mut temp_ptr_scale_fac: *mut WORD16 = 0 as *mut WORD16;
    let mut norm_val: WORD16 = 0;
    let mut window_grps: WORD32 = 0;
    let mut trans_sfb: WORD32 = 0;
    let mut index: WORD16 = 0;
    let mut length: WORD16 = 0;
    let mut hscf: *const UWORD16 = (*usac_data).huffman_code_book_scl;
    let mut idx_tab: *const UWORD32 = (*usac_data).huffman_code_book_scl_index;
    let mut start_bit_pos: WORD32 = (*g_bs).bit_pos;
    let mut start_read_pos: *mut UWORD8 = (*g_bs).ptr_read_next;
    let mut ptr_read_next: *mut UWORD8 = (*g_bs).ptr_read_next;
    let mut bit_pos: WORD32 = 7 as WORD32 - (*g_bs).bit_pos;
    let mut is_1_group: WORD32 = 1 as WORD32;
    let mut bb: WORD32 = 0 as WORD32;
    let mut i: WORD32 = 0;
    let mut increment: WORD32 = 0;
    let mut read_word: WORD32 = ixheaacd_aac_showbits_32(
        ptr_read_next,
        (*g_bs).cnt_bits,
        &mut increment,
    ) as WORD32;
    ptr_read_next = ((*g_bs).ptr_read_next).offset(increment as isize);
    trans_sfb = (*info).sfb_per_sbk;
    temp_ptr_scale_fac = factors as *mut WORD16;
    window_grps = (*info).max_win_len;
    memset(
        factors as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        MAXBANDS as core::ffi::c_int as size_t,
    );
    band = (trans_sfb as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    group = 0 as core::ffi::c_int as WORD32;
    while group < window_grps {
        temp_codebook_ptr = &mut *ptr_code_book
            .offset((group as core::ffi::c_int * 16 as core::ffi::c_int) as isize)
            as *mut WORD8;
        ptr_scale_fac = temp_ptr_scale_fac;
        let fresh3 = groups;
        groups = groups.offset(1);
        group = *fresh3 as WORD32;
        band = (trans_sfb as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        while band >= 0 as core::ffi::c_int {
            let fresh4 = temp_codebook_ptr;
            temp_codebook_ptr = temp_codebook_ptr.offset(1);
            let mut cb_num: WORD32 = *fresh4 as WORD32;
            if band == trans_sfb as core::ffi::c_int - 1 as core::ffi::c_int
                && is_1_group == 1 as core::ffi::c_int
            {
                *temp_ptr_scale_fac = factor;
                temp_ptr_scale_fac = temp_ptr_scale_fac.offset(1);
            } else if cb_num == ZERO_HCB {
                let fresh5 = temp_ptr_scale_fac;
                temp_ptr_scale_fac = temp_ptr_scale_fac.offset(1);
                *fresh5 = 0 as WORD16;
            } else {
                let mut pns_band: WORD32 = 0;
                let mut curr_energy: WORD16 = 0 as WORD16;
                let mut read_word1: UWORD32 = 0;
                read_word1 = (read_word << bit_pos) as UWORD32;
                ixheaacd_huffman_decode(
                    read_word1 as WORD32,
                    &mut index,
                    &mut length,
                    hscf,
                    idx_tab,
                );
                bit_pos += length as core::ffi::c_int;
                ixheaacd_aac_read_byte_corr1(
                    &mut ptr_read_next,
                    &mut bit_pos,
                    &mut read_word,
                    (*g_bs).ptr_bit_buf_end,
                );
                norm_val = (index as core::ffi::c_int - 60 as core::ffi::c_int)
                    as WORD16;
                if cb_num > NOISE_HCB {
                    position = (position as core::ffi::c_int
                        + norm_val as core::ffi::c_int) as WORD16;
                    let fresh6 = temp_ptr_scale_fac;
                    temp_ptr_scale_fac = temp_ptr_scale_fac.offset(1);
                    *fresh6 = -(position as core::ffi::c_int) as WORD16;
                } else if cb_num < NOISE_HCB {
                    factor = (factor as core::ffi::c_int + norm_val as core::ffi::c_int)
                        as WORD16;
                    let fresh7 = temp_ptr_scale_fac;
                    temp_ptr_scale_fac = temp_ptr_scale_fac.offset(1);
                    *fresh7 = factor;
                } else {
                    curr_energy = (curr_energy as core::ffi::c_int
                        + norm_val as core::ffi::c_int) as WORD16;
                    pns_band = (((group as core::ffi::c_int) << 4 as core::ffi::c_int)
                        + trans_sfb as core::ffi::c_int - band as core::ffi::c_int
                        - 1 as core::ffi::c_int) as WORD32;
                    *temp_ptr_scale_fac.offset(pns_band as isize) = curr_energy;
                    temp_ptr_scale_fac = temp_ptr_scale_fac.offset(1);
                }
            }
            band -= 1;
        }
        is_1_group = 0 as core::ffi::c_int as WORD32;
        if (*info).islong == 0 {
            bb += 1;
            while bb < group {
                i = 0 as core::ffi::c_int as WORD32;
                while i < trans_sfb {
                    *ptr_scale_fac.offset((i + trans_sfb) as isize) = *ptr_scale_fac
                        .offset(i as isize);
                    i += 1;
                }
                temp_ptr_scale_fac = temp_ptr_scale_fac.offset(trans_sfb as isize);
                ptr_scale_fac = ptr_scale_fac.offset(trans_sfb as isize);
                bb += 1;
            }
        }
    }
    ptr_read_next = ptr_read_next.offset(-(increment as isize));
    ixheaacd_aac_read_byte_corr1(
        &mut ptr_read_next,
        &mut bit_pos,
        &mut read_word,
        (*g_bs).ptr_bit_buf_end,
    );
    (*g_bs).ptr_read_next = ptr_read_next;
    (*g_bs).bit_pos = 7 as WORD32 - bit_pos;
    let mut bits_consumed: WORD32 = 0;
    bits_consumed = (((((*g_bs).ptr_read_next).offset_from(start_read_pos)
        as core::ffi::c_long) << 3 as core::ffi::c_int)
        + (start_bit_pos - (*g_bs).bit_pos) as core::ffi::c_long) as WORD32;
    (*g_bs).cnt_bits -= bits_consumed;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_fd_channel_stream(
    mut usac_data: *mut ia_usac_data_struct,
    mut pstr_core_coder: *mut ia_usac_tmp_core_coder_struct,
    mut max_sfb: *mut UWORD8,
    mut window_sequence_last: WORD32,
    mut chn: WORD32,
    mut noise_filling: WORD32,
    mut ch: WORD32,
    mut it_bit_buff: *mut ia_bit_buf_struct,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut tot_sfb: WORD32 = 0;
    let mut noise_level: WORD32 = 0 as WORD32;
    let mut arith_reset_flag: WORD32 = 0;
    let mut arth_size: WORD32 = 0;
    let mut global_gain: WORD16 = 0;
    let mut max_spec_coefficients: WORD32 = 0;
    let mut err_code: WORD32 = 0 as WORD32;
    let mut noise_offset: WORD32 = 0;
    let mut fac_data: *mut WORD32 = 0 as *mut WORD32;
    let mut info: *mut ia_sfb_info_struct = 0 as *mut ia_sfb_info_struct;
    let mut ptr_code_book: *mut WORD8 = &mut (*usac_data).scratch_buffer
        as *mut [WORD32; 1024] as *mut WORD8;
    global_gain = ixheaacd_read_bits_buf(it_bit_buff, 8 as WORD) as WORD16;
    if noise_filling != 0 {
        noise_level = ixheaacd_read_bits_buf(it_bit_buff, 3 as WORD);
        noise_offset = ixheaacd_read_bits_buf(it_bit_buff, 5 as WORD);
    } else {
        noise_level = 0 as core::ffi::c_int as WORD32;
        noise_offset = 0 as core::ffi::c_int as WORD32;
    }
    if (*pstr_core_coder).common_window == 0 {
        err_code = ixheaacd_ics_info(
            usac_data,
            chn,
            max_sfb,
            it_bit_buff,
            window_sequence_last,
        );
        if err_code == -(1 as core::ffi::c_int) {
            if (*usac_data).ec_flag != 0 {
                memcpy(
                    ((*usac_data).max_sfb).as_mut_ptr() as *mut core::ffi::c_void,
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
    }
    info = (*usac_data).pstr_sfb_info[chn as usize];
    if (*pstr_core_coder).common_tw == 0
        && (*usac_data).tw_mdct[0 as core::ffi::c_int as usize] == 1 as core::ffi::c_int
    {
        (*usac_data).tw_data_present[chn as usize] = ixheaacd_read_bits_buf(
            it_bit_buff,
            1 as WORD,
        );
        if (*usac_data).tw_data_present[chn as usize] != 0 {
            let mut i_0: WORD32 = 0;
            i_0 = 0 as core::ffi::c_int as WORD32;
            while i_0 < NUM_TW_NODES {
                *((*usac_data).tw_ratio[chn as usize]).offset(i_0 as isize) = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    3 as WORD,
                );
                i_0 += 1;
            }
        }
    }
    if *max_sfb as core::ffi::c_int == 0 as core::ffi::c_int {
        tot_sfb = 0 as core::ffi::c_int as WORD32;
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        tot_sfb = (*info).sfb_per_sbk;
        loop {
            let fresh2 = i;
            i = i + 1;
            if !((*((*usac_data).group_dis[chn as usize]).offset(fresh2 as isize)
                as core::ffi::c_int) < (*info).max_win_len)
            {
                break;
            }
            tot_sfb += (*info).sfb_per_sbk;
        }
    }
    ixheaacd_scale_factor_data(
        info,
        tot_sfb,
        *max_sfb as WORD32,
        (*info).sfb_per_sbk,
        ptr_code_book,
    );
    if (*it_bit_buff).ptr_read_next
        > ((*it_bit_buff).ptr_bit_buf_end).offset(-(3 as core::ffi::c_int as isize))
        && (*it_bit_buff).size == (*it_bit_buff).max_size
    {
        return -(1 as WORD32);
    }
    ixheaacd_section_data(
        usac_data,
        it_bit_buff,
        info,
        global_gain,
        (*usac_data).factors[chn as usize],
        (*usac_data).group_dis[chn as usize],
        ptr_code_book,
    );
    if (*pstr_core_coder).tns_data_present[ch as usize] == 0 as core::ffi::c_int {
        ixheaacd_tns_reset(info, (*usac_data).pstr_tns[chn as usize]);
    }
    if (*pstr_core_coder).tns_data_present[ch as usize] == 1 as core::ffi::c_int {
        ixheaacd_read_tns_u(info, (*usac_data).pstr_tns[chn as usize], it_bit_buff);
    }
    if *max_sfb as core::ffi::c_int > 0 as core::ffi::c_int {
        max_spec_coefficients = ((*info)
            .sfb_idx_tbl[(*max_sfb as core::ffi::c_int - 1 as core::ffi::c_int) as usize]
            as core::ffi::c_int
            / (*info).group_len[0 as core::ffi::c_int as usize] as core::ffi::c_int)
            as WORD32;
    } else {
        max_spec_coefficients = 0 as core::ffi::c_int as WORD32;
    }
    if (*usac_data).usac_independency_flg != 0 {
        arith_reset_flag = 1 as core::ffi::c_int as WORD32;
    } else {
        arith_reset_flag = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
    }
    match (*usac_data).window_sequence[chn as usize] {
        EIGHT_SHORT_SEQUENCE => {
            arth_size = ((*usac_data).ccfl as core::ffi::c_int / 8 as core::ffi::c_int)
                as WORD32;
        }
        _ => {
            arth_size = (*usac_data).ccfl;
        }
    }
    err_code = ixheaacd_ac_spectral_data(
        usac_data,
        max_spec_coefficients,
        noise_level,
        noise_offset,
        arth_size,
        it_bit_buff,
        *max_sfb,
        arith_reset_flag,
        noise_filling,
        chn,
    );
    if err_code != 0 as core::ffi::c_int {
        return err_code;
    }
    (*usac_data).fac_data_present[chn as usize] = ixheaacd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    );
    if (*usac_data).fac_data_present[chn as usize] != 0 {
        let mut fac_len: WORD32 = 0;
        if (*usac_data).window_sequence[chn as usize] == EIGHT_SHORT_SEQUENCE {
            fac_len = ((*usac_data).ccfl as core::ffi::c_int / 16 as core::ffi::c_int)
                as WORD32;
        } else {
            fac_len = ((*usac_data).ccfl as core::ffi::c_int / 8 as core::ffi::c_int)
                as WORD32;
        }
        fac_data = ((*usac_data).fac_data[chn as usize]).as_mut_ptr();
        *fac_data.offset(0 as core::ffi::c_int as isize) = ixheaacd_read_bits_buf(
            it_bit_buff,
            7 as WORD,
        );
        ixheaacd_fac_decoding(
            fac_len,
            0 as WORD32,
            &mut *fac_data.offset(1 as core::ffi::c_int as isize),
            it_bit_buff,
        );
    }
    return 0 as WORD32;
}
pub const ZERO_HCB: core::ffi::c_int = 0 as core::ffi::c_int;
pub const NOISE_HCB: core::ffi::c_int = 13 as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES: core::ffi::c_int = 0x1804
    as core::ffi::c_int;
pub const NUM_TW_NODES: core::ffi::c_int = 16 as core::ffi::c_int;
pub const ONLY_LONG_SEQUENCE: core::ffi::c_int = 0 as core::ffi::c_int;
pub const LONG_START_SEQUENCE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const EIGHT_SHORT_SEQUENCE: core::ffi::c_int = 2 as core::ffi::c_int;
pub const LONG_STOP_SEQUENCE: core::ffi::c_int = 3 as core::ffi::c_int;
pub const STOP_START_SEQUENCE: core::ffi::c_int = 4 as core::ffi::c_int;
