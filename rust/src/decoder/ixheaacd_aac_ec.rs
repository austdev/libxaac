extern "C" {
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
    static ia_ec_interpolation_fac: [WORD16; 4];
}
pub type size_t = usize;
pub type WORD8 = core::ffi::c_schar;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD16 = core::ffi::c_short;
pub type UWORD16 = core::ffi::c_ushort;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type WORD64 = core::ffi::c_longlong;
pub type FLOAT32 = core::ffi::c_float;
pub type VOID = ();
pub type FLAG = core::ffi::c_int;
pub type WORD = core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_pns_correlation_info_struct {
    pub correlated: [UWORD8; 16],
    pub random_vector: [WORD32; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_pns_rand_vec_struct {
    pub current_seed: WORD32,
    pub pns_frame_number: WORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_pns_info_struct {
    pub pns_used: [UWORD8; 128],
    pub noise_energy: WORD16,
    pub pns_active: UWORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_pulse_info_struct {
    pub pulse_data_present: FLAG,
    pub number_pulse: WORD16,
    pub pulse_start_band: WORD16,
    pub pulse_offset: [WORD8; 4],
    pub pulse_amp: [WORD8; 4],
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
pub struct ltp_info {
    pub last_band: UWORD8,
    pub data_present: UWORD8,
    pub lag: UWORD16,
    pub lag_update: UWORD8,
    pub coef: UWORD8,
    pub long_used: [UWORD8; 51],
    pub short_used: [UWORD8; 8],
    pub short_lag_present: [UWORD8; 8],
    pub short_lag: [UWORD8; 8],
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
pub struct ia_ics_info_struct {
    pub window_shape: WORD16,
    pub window_sequence: WORD16,
    pub max_sfb: WORD16,
    pub num_swb_window: WORD16,
    pub sampling_rate_index: WORD16,
    pub num_window_groups: WORD16,
    pub window_group_length: [WORD8; 8],
    pub frame_length: WORD16,
    pub frame_size: WORD32,
    pub predictor_data_present: WORD16,
    pub ltp: ltp_info,
    pub ltp2: ltp_info,
    pub qshift_adj: WORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_stereo_info_struct {
    pub ms_used: [[UWORD8; 64]; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_filter_info_struct {
    pub start_band: WORD16,
    pub stop_band: WORD16,
    pub direction: WORD8,
    pub resolution: WORD8,
    pub order: WORD8,
    pub coef: [WORD8; 31],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_tns_info_aac_struct {
    pub tns_data_present: FLAG,
    pub n_filt: [WORD8; 8],
    pub str_filter: [[ia_filter_info_struct; 3]; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_rvlc_info_struct {
    pub sf_concealment: WORD32,
    pub rev_global_gain: WORD32,
    pub rvlc_sf_len: WORD16,
    pub dpcm_noise_nrg: WORD32,
    pub sf_esc_present: WORD32,
    pub rvlc_esc_len: WORD16,
    pub dpcm_noise_last_pos: WORD32,
    pub dpcm_is_last_pos: WORD32,
    pub rvlc_sf_fwd_len: WORD16,
    pub rvlc_sf_bwd_len: WORD16,
    pub ptr_rvl_bit_cnt: *mut WORD16,
    pub ptr_rvl_bit_str_idx: *mut UWORD16,
    pub num_wind_grps: WORD16,
    pub max_sfb_transmitted: WORD16,
    pub first_noise_group: UWORD8,
    pub first_noise_band: UWORD8,
    pub direction: UWORD8,
    pub rvl_fwd_bit_str_idx: UWORD16,
    pub rvl_bwd_bit_str_idx: UWORD16,
    pub esc_bit_str_idx: UWORD16,
    pub ptr_huff_tree_rvl_cw: *const UWORD32,
    pub ptr_huff_tree_rvl_esc: *const UWORD32,
    pub num_fwd_esc_words_decoded: UWORD8,
    pub num_bwd_esc_words_decoded: UWORD8,
    pub num_esc_words_decoded: UWORD8,
    pub noise_used: WORD8,
    pub intensity_used: WORD8,
    pub sf_used: WORD8,
    pub firt_scale_fac: WORD16,
    pub last_scale_fac: WORD16,
    pub first_nrg: WORD16,
    pub last_nrg: WORD16,
    pub is_first: WORD16,
    pub is_last: WORD16,
    pub rvlc_err_log: UWORD32,
    pub conceal_min: WORD16,
    pub conceal_max: WORD16,
    pub conceal_min_esc: WORD16,
    pub conceal_max_esc: WORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_huff_code_reorder_io_struct {
    pub err_log: UWORD32,
    pub ptr_quant_spec_coeff_base: *mut WORD32,
    pub quant_spec_coeff_idx: WORD32,
    pub reordered_spec_data_len: WORD16,
    pub num_sect: WORD16,
    pub ptr_num_line_in_sect: *mut WORD16,
    pub bit_str_idx: UWORD16,
    pub longest_cw_len: WORD8,
    pub ptr_cb: *mut UWORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_huff_code_reorder_cb_pairs_struct {
    pub ptr_min_cb_pair_tbl: *const UWORD8,
    pub ptr_max_cb_pair_tbl: *const UWORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_huff_code_reorder_tbl_struct {
    pub ptr_lav_tbl: *const UWORD16,
    pub ptr_max_cw_len_tbl: *const UWORD8,
    pub ptr_cb_dimension_tbl: *const UWORD8,
    pub ptr_cb_dim_shift_tbl: *const UWORD8,
    pub ptr_cb_sign_tbl: *const UWORD8,
    pub ptr_cb_priority: *const UWORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_huff_code_reord_seg_info_struct {
    pub num_segment: WORD32,
    pub segment_offset: UWORD32,
    pub arr_temp_values: [WORD32; 1024],
    pub arr_seg_start_l: [UWORD16; 512],
    pub arr_seg_start_r: [UWORD16; 512],
    pub p_remaining_bits_in_seg: [WORD8; 512],
    pub code: [WORD32; 512],
    pub code_extra: [WORD32; 512],
    pub p_num_bits: [WORD8; 512],
    pub read_direction: UWORD8,
    pub is_decoded: [WORD32; 512],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_huff_code_reord_sect_info_struct {
    pub num_code_word: UWORD32,
    pub current_codeword: UWORD32,
    pub num_sorted_section: UWORD32,
    pub ptr_num_cw_in_sect: [UWORD16; 256],
    pub ptr_num_sorted_cw_in_sect: [UWORD16; 256],
    pub ptr_num_ext_sorted_cw_in_sect: [UWORD16; 270],
    pub num_ext_sorted_cw_in_sect_idx: WORD32,
    pub ptr_num_ext_sorted_sect_in_sets: [UWORD16; 14],
    pub num_ext_sorted_sect_in_sets_idx: WORD32,
    pub ptr_reorder_offset: [UWORD16; 256],
    pub ptr_sorted_cb: [UWORD8; 256],
    pub ptr_ext_sorted_cw: [UWORD8; 270],
    pub ext_sorted_cw_idx: WORD32,
    pub ptr_ext_sorted_sect_max_cb_len: [UWORD8; 270],
    pub ext_sorted_sect_max_cb_len_idx: WORD32,
    pub ptr_cb_switch: [UWORD8; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_hcr_non_pcw_sideinfo_struct {
    pub ptr_result_base: *mut WORD32,
    pub res_ptr_idx: [UWORD16; 256],
    pub cw_offset: UWORD32,
    pub ptr_cb: [UWORD8; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_hcr_info_struct {
    pub str_dec_io: ia_huff_code_reorder_io_struct,
    pub codebook_pairs: ia_huff_code_reorder_cb_pairs_struct,
    pub table_info: ia_huff_code_reorder_tbl_struct,
    pub str_segment_info: ia_huff_code_reord_seg_info_struct,
    pub sect_info: ia_huff_code_reord_sect_info_struct,
    pub str_non_pcw_side_info: ia_hcr_non_pcw_sideinfo_struct,
    pub global_hcr_type: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_channel_info_struct {
    pub ptr_scale_factor: *mut WORD16,
    pub ptr_code_book: *mut WORD8,
    pub ptr_spec_coeff: *mut WORD32,
    pub pstr_stereo_info: *mut ia_stereo_info_struct,
    pub pstr_pns_corr_info: *mut ia_pns_correlation_info_struct,
    pub pstr_pns_rand_vec_data: *mut ia_pns_rand_vec_struct,
    pub str_ics_info: ia_ics_info_struct,
    pub str_tns_info: ia_tns_info_aac_struct,
    pub str_pulse_info: ia_pulse_info_struct,
    pub str_pns_info: ia_pns_info_struct,
    pub common_window: WORD16,
    pub element_instance_tag: WORD16,
    pub global_gain: WORD16,
    pub scratch_buf_ptr: *mut WORD32,
    pub pulse_scratch: *mut WORD32,
    pub ptr_rvlc_info: ia_rvlc_info_struct,
    pub str_hcr_info: ia_hcr_info_struct,
    pub reorder_spect_data_len: WORD16,
    pub longest_cw_len: WORD8,
    pub rvlc_scf_esc_arr: [WORD16; 128],
    pub rvlc_scf_fwd_arr: [WORD16; 128],
    pub rvlc_scf_bwd_arr: [WORD16; 128],
    pub rvlc_intensity_used: WORD8,
    pub num_line_in_sec4_hcr_arr: [WORD16; 256],
    pub cb4_hcr_arr: [UWORD8; 256],
    pub number_sect: WORD32,
    pub granule_len: WORD32,
    pub rvlc_curr_sf_flag: WORD16,
    pub ltp_buf: *mut WORD16,
    pub ltp_lag: UWORD16,
}
pub const NO_TRANSITION: core::ffi::c_int = 0 as core::ffi::c_int;
pub const TRANS_SHORT_LONG: core::ffi::c_int = 1 as core::ffi::c_int;
pub const FRAME_OKAY: core::ffi::c_int = 0 as core::ffi::c_int;
pub const FRAME_CONCEAL_SINGLE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const FRAME_FADE: core::ffi::c_int = 2 as core::ffi::c_int;
pub const FRAME_MUTE: core::ffi::c_int = 3 as core::ffi::c_int;
pub const MAX_FADE_FRAMES: core::ffi::c_int = 8 as core::ffi::c_int;
pub const MAX_SPEC_SCALE_LEN: core::ffi::c_int = 8 as core::ffi::c_int;
pub const CONCEAL_NOT_DEFINED: UWORD8 = -(1 as core::ffi::c_int) as UWORD8;
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
unsafe extern "C" fn ixheaac_mult32x16in32_shl(mut a: WORD32, mut b: WORD16) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 16 as core::ffi::c_int) as WORD32;
    return result << 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn ixheaac_mult32(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 32 as core::ffi::c_int) as WORD32;
    return result;
}
pub const LEN_SUPERFRAME: core::ffi::c_int = 1024 as core::ffi::c_int;
unsafe extern "C" fn ixheaacd_aac_ec_get_win_seq(mut prev_win_seq: WORD32) -> WORD32 {
    let mut new_win_seq: WORD32 = ONLY_LONG_SEQUENCE;
    if prev_win_seq == LONG_START_SEQUENCE || prev_win_seq == EIGHT_SHORT_SEQUENCE {
        new_win_seq = LONG_STOP_SEQUENCE as WORD32;
    }
    return new_win_seq;
}
unsafe extern "C" fn ixheaacd_aac_ec_flip_spec_sign(
    mut ptr_spec_coeff: *mut WORD32,
    mut num_samples: WORD32,
) -> VOID {
    let mut idx: WORD32 = 0;
    let mut random_value: WORD32 = 0;
    idx = 0 as core::ffi::c_int as WORD32;
    while idx < num_samples {
        random_value = *ptr_spec_coeff.offset(idx as isize) ^ idx;
        if random_value as core::ffi::c_int & 1 as core::ffi::c_int
            == 0 as core::ffi::c_int
        {
            *ptr_spec_coeff.offset(idx as isize) = ixheaac_negate32_sat(
                *ptr_spec_coeff.offset(idx as isize),
            );
        }
        idx += 1;
    }
}
unsafe extern "C" fn ixheaacd_aac_ec_store(
    mut pstr_ec_state: *mut ia_ec_state_str,
    mut pstr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
    mut pstr_ics_info: *mut ia_ics_info_struct,
) -> VOID {
    let mut ptr_spec_coeff: *mut WORD32 = (*pstr_aac_dec_channel_info).ptr_spec_coeff;
    let mut ptr_spec_scale: *mut WORD16 = (*pstr_aac_dec_channel_info).ptr_scale_factor;
    let mut win_shape: UWORD8 = (*pstr_ec_state).win_shape;
    let mut win_seq: WORD32 = (*pstr_ec_state).win_seq;
    let mut q_spec_scale: [WORD16; 8] = [0; 8];
    let mut ptr_temp_spec_coeff: *mut WORD32 = &mut *((*pstr_ec_state)
        .str_ec_scratch
        .spec_coeff)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
    memcpy(
        q_spec_scale.as_mut_ptr() as *mut core::ffi::c_void,
        ((*pstr_ec_state).q_spec_coeff).as_mut_ptr() as *const core::ffi::c_void,
        ::core::mem::size_of::<[WORD16; 8]>() as size_t,
    );
    (*pstr_ec_state).win_seq = (*pstr_ics_info).window_sequence as WORD32;
    (*pstr_ec_state).win_shape = (*pstr_ics_info).window_shape as UWORD8;
    (*pstr_ec_state).prev_win_group_len = *((*pstr_ics_info).window_group_length)
        .as_mut_ptr()
        .offset((*pstr_ics_info).num_window_groups as core::ffi::c_int as isize)
        .offset(-(1 as core::ffi::c_int as isize)) as UWORD8;
    memcpy(
        ((*pstr_ec_state).q_spec_coeff).as_mut_ptr() as *mut core::ffi::c_void,
        ptr_spec_scale as *const core::ffi::c_void,
        ::core::mem::size_of::<[WORD16; 128]>() as size_t,
    );
    memcpy(
        ptr_temp_spec_coeff as *mut core::ffi::c_void,
        ptr_spec_coeff as *const core::ffi::c_void,
        (LEN_SUPERFRAME as size_t)
            .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
    );
    memcpy(
        ptr_spec_coeff as *mut core::ffi::c_void,
        ((*pstr_ec_state).spectral_coeff).as_mut_ptr() as *const core::ffi::c_void,
        (LEN_SUPERFRAME as size_t)
            .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
    );
    memcpy(
        ((*pstr_ec_state).spectral_coeff).as_mut_ptr() as *mut core::ffi::c_void,
        ptr_temp_spec_coeff as *const core::ffi::c_void,
        ::core::mem::size_of::<[WORD32; 1024]>() as size_t,
    );
    (*pstr_ics_info).window_sequence = win_seq as WORD16;
    (*pstr_ics_info).window_shape = win_shape as WORD16;
    memcpy(
        ptr_spec_scale as *mut core::ffi::c_void,
        q_spec_scale.as_mut_ptr() as *const core::ffi::c_void,
        (MAX_SPEC_SCALE_LEN as size_t)
            .wrapping_mul(::core::mem::size_of::<WORD16>() as size_t),
    );
}
unsafe extern "C" fn ixheaacd_aac_ec_calc_sfb_nrg(
    mut ptr_spec_coeff: *mut WORD32,
    mut pstr_samp_rate_info: *const ia_usac_samp_rate_info,
    win_seq: WORD32,
    mut win_trans: WORD32,
    mut ptr_sfb_energy: *mut WORD32,
) -> VOID {
    let mut ptr_sfb_offset: *const WORD16 = 0 as *const WORD16;
    let mut line: WORD32 = 0 as WORD32;
    let mut sfb: WORD32 = 0;
    let mut total_scale_factor_bands: WORD32 = 0 as WORD32;
    match win_seq {
        EIGHT_SHORT_SEQUENCE => {
            if win_trans == NO_TRANSITION {
                total_scale_factor_bands = ((*pstr_samp_rate_info).num_sfb_128
                    as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
                ptr_sfb_offset = (*pstr_samp_rate_info).ptr_sfb_128;
                sfb = 0 as core::ffi::c_int as WORD32;
                while sfb < total_scale_factor_bands {
                    let mut accu: WORD32 = 1 as WORD32;
                    let mut q_nrg: WORD32 = 0;
                    if sfb == 0 as core::ffi::c_int {
                        q_nrg = ((::core::mem::size_of::<WORD32>() as usize)
                            << 3 as core::ffi::c_int)
                            .wrapping_sub(
                                ixheaac_norm32(
                                    *ptr_sfb_offset.offset(sfb as isize) as WORD32 - 0 as WORD32,
                                ) as usize,
                            ) as WORD32;
                        while line
                            < *ptr_sfb_offset.offset(sfb as isize) as core::ffi::c_int
                        {
                            accu
                                += ixheaac_mult32(
                                    *ptr_spec_coeff.offset(line as isize),
                                    *ptr_spec_coeff.offset(line as isize),
                                ) >> q_nrg;
                            line += 1;
                        }
                        *ptr_sfb_energy.offset(sfb as isize) = ixheaac_norm32(accu)
                            as WORD32;
                    }
                    q_nrg = ((::core::mem::size_of::<WORD32>() as usize)
                        << 3 as core::ffi::c_int)
                        .wrapping_sub(
                            ixheaac_norm32(
                                *ptr_sfb_offset
                                    .offset(
                                        (sfb as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                                    ) as WORD32 - *ptr_sfb_offset.offset(sfb as isize) as WORD32,
                            ) as usize,
                        ) as WORD32;
                    while line
                        < *ptr_sfb_offset
                            .offset(
                                (sfb as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                            ) as core::ffi::c_int
                    {
                        accu
                            += ixheaac_mult32(
                                *ptr_spec_coeff.offset(line as isize),
                                *ptr_spec_coeff.offset(line as isize),
                            ) >> q_nrg;
                        line += 1;
                    }
                    *ptr_sfb_energy.offset(sfb as isize) = ixheaac_norm32(accu)
                        as WORD32;
                    sfb += 1;
                }
            } else {
                total_scale_factor_bands = ((*pstr_samp_rate_info).num_sfb_1024
                    as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
                ptr_sfb_offset = (*pstr_samp_rate_info).ptr_sfb_1024;
                sfb = 0 as core::ffi::c_int as WORD32;
                while sfb < total_scale_factor_bands {
                    let mut accu_0: WORD32 = 1 as WORD32;
                    let mut q_nrg_0: WORD32 = 0;
                    if sfb == 0 as core::ffi::c_int {
                        q_nrg_0 = ((::core::mem::size_of::<WORD32>() as usize)
                            << 3 as core::ffi::c_int)
                            .wrapping_sub(
                                ixheaac_norm32(
                                    *ptr_sfb_offset.offset(sfb as isize) as WORD32 - 0 as WORD32,
                                ) as usize,
                            ) as WORD32;
                        while line
                            < *ptr_sfb_offset.offset(sfb as isize) as core::ffi::c_int
                        {
                            accu_0
                                += ixheaac_mult32(
                                    *ptr_spec_coeff
                                        .offset((line >> 3 as core::ffi::c_int) as isize),
                                    *ptr_spec_coeff
                                        .offset((line >> 3 as core::ffi::c_int) as isize),
                                ) >> q_nrg_0;
                            line += 1;
                        }
                        *ptr_sfb_energy.offset(sfb as isize) = ixheaac_norm32(accu_0)
                            as WORD32;
                    }
                    q_nrg_0 = ((::core::mem::size_of::<WORD32>() as usize)
                        << 3 as core::ffi::c_int)
                        .wrapping_sub(
                            ixheaac_norm32(
                                *ptr_sfb_offset
                                    .offset(
                                        (sfb as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                                    ) as WORD32 - *ptr_sfb_offset.offset(sfb as isize) as WORD32,
                            ) as usize,
                        ) as WORD32;
                    while line
                        < *ptr_sfb_offset
                            .offset(
                                (sfb as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                            ) as core::ffi::c_int
                    {
                        accu_0
                            += ixheaac_mult32(
                                *ptr_spec_coeff
                                    .offset((line >> 3 as core::ffi::c_int) as isize),
                                *ptr_spec_coeff
                                    .offset((line >> 3 as core::ffi::c_int) as isize),
                            ) >> q_nrg_0;
                        line += 1;
                    }
                    *ptr_sfb_energy.offset(sfb as isize) = ixheaac_norm32(accu_0)
                        as WORD32;
                    sfb += 1;
                }
            }
        }
        ONLY_LONG_SEQUENCE | LONG_START_SEQUENCE | LONG_STOP_SEQUENCE => {
            if win_trans == NO_TRANSITION {
                total_scale_factor_bands = ((*pstr_samp_rate_info).num_sfb_1024
                    as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
                ptr_sfb_offset = (*pstr_samp_rate_info).ptr_sfb_1024;
                sfb = 0 as core::ffi::c_int as WORD32;
                while sfb < total_scale_factor_bands {
                    let mut accu_1: WORD32 = 1 as WORD32;
                    let mut q_nrg_1: WORD32 = 0;
                    if sfb == 0 as core::ffi::c_int {
                        q_nrg_1 = ((::core::mem::size_of::<WORD32>() as usize)
                            << 3 as core::ffi::c_int)
                            .wrapping_sub(
                                ixheaac_norm32(
                                    *ptr_sfb_offset.offset(sfb as isize) as WORD32 - 0 as WORD32,
                                ) as usize,
                            ) as WORD32;
                        while line
                            < *ptr_sfb_offset.offset(sfb as isize) as core::ffi::c_int
                        {
                            accu_1
                                += ixheaac_mult32(
                                    *ptr_spec_coeff.offset(line as isize),
                                    *ptr_spec_coeff.offset(line as isize),
                                ) >> q_nrg_1;
                            line += 1;
                        }
                        *ptr_sfb_energy.offset(sfb as isize) = ixheaac_norm32(accu_1)
                            as WORD32;
                    }
                    q_nrg_1 = ((::core::mem::size_of::<WORD32>() as usize)
                        << 3 as core::ffi::c_int)
                        .wrapping_sub(
                            ixheaac_norm32(
                                *ptr_sfb_offset
                                    .offset(
                                        (sfb as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                                    ) as WORD32 - *ptr_sfb_offset.offset(sfb as isize) as WORD32,
                            ) as usize,
                        ) as WORD32;
                    while line
                        < *ptr_sfb_offset
                            .offset(
                                (sfb as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                            ) as core::ffi::c_int
                    {
                        accu_1
                            += ixheaac_mult32(
                                *ptr_spec_coeff.offset(line as isize),
                                *ptr_spec_coeff.offset(line as isize),
                            ) >> q_nrg_1;
                        line += 1;
                    }
                    *ptr_sfb_energy.offset(sfb as isize) = ixheaac_norm32(accu_1)
                        as WORD32;
                    sfb += 1;
                }
            } else {
                total_scale_factor_bands = ((*pstr_samp_rate_info).num_sfb_128
                    as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
                ptr_sfb_offset = (*pstr_samp_rate_info).ptr_sfb_128;
                sfb = 0 as core::ffi::c_int as WORD32;
                while sfb < total_scale_factor_bands {
                    let mut accu_2: WORD32 = 1 as WORD32;
                    let mut q_nrg_2: WORD32 = 0;
                    if sfb == 0 as core::ffi::c_int {
                        q_nrg_2 = ((::core::mem::size_of::<WORD32>() as usize)
                            << 3 as core::ffi::c_int)
                            .wrapping_sub(
                                ixheaac_norm32(
                                    *ptr_sfb_offset.offset(sfb as isize) as WORD32 - 0 as WORD32,
                                ) as usize,
                            ) as WORD32;
                        while line
                            < (*ptr_sfb_offset.offset(sfb as isize) as core::ffi::c_int)
                                << 3 as core::ffi::c_int
                        {
                            accu_2
                                += accu_2
                                    + (ixheaac_mult32(
                                        *ptr_spec_coeff.offset(line as isize),
                                        *ptr_spec_coeff.offset(line as isize),
                                    ) >> q_nrg_2) >> 3 as core::ffi::c_int;
                            line += 1;
                        }
                        *ptr_sfb_energy.offset(sfb as isize) = ixheaac_norm32(accu_2)
                            as WORD32;
                    }
                    q_nrg_2 = ((::core::mem::size_of::<WORD32>() as usize)
                        << 3 as core::ffi::c_int)
                        .wrapping_sub(
                            ixheaac_norm32(
                                *ptr_sfb_offset
                                    .offset(
                                        (sfb as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                                    ) as WORD32 - *ptr_sfb_offset.offset(sfb as isize) as WORD32,
                            ) as usize,
                        ) as WORD32;
                    while line
                        < (*ptr_sfb_offset
                            .offset(
                                (sfb as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                            ) as core::ffi::c_int) << 3 as core::ffi::c_int
                    {
                        accu_2
                            += accu_2
                                + (ixheaac_mult32(
                                    *ptr_spec_coeff.offset(line as isize),
                                    *ptr_spec_coeff.offset(line as isize),
                                ) >> q_nrg_2) >> 3 as core::ffi::c_int;
                        line += 1;
                    }
                    *ptr_sfb_energy.offset(sfb as isize) = ixheaac_norm32(accu_2)
                        as WORD32;
                    sfb += 1;
                }
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn ixheaacd_aac_ec_interpolate(
    mut ptr_spec_coeff: *mut WORD32,
    mut ptr_spec_scale_prev: *mut WORD16,
    mut ptr_spec_scale_act: *mut WORD16,
    mut ptr_spec_scale_out: *mut WORD16,
    mut nrg_prev: *mut WORD32,
    mut nrg_act: *mut WORD32,
    mut num_sfb: WORD32,
    mut ptr_sfb_offset: *const WORD16,
) -> VOID {
    let mut sfb: WORD32 = 0;
    let mut line: WORD32 = 0 as WORD32;
    let mut fac_shift: WORD32 = 0;
    let mut fac_mod: WORD32 = 0;
    sfb = 0 as core::ffi::c_int as WORD32;
    while sfb < num_sfb {
        fac_shift = (*nrg_prev.offset(sfb as isize) - *nrg_act.offset(sfb as isize)
            + ((*ptr_spec_scale_act as core::ffi::c_int
                - *ptr_spec_scale_prev as core::ffi::c_int) << 1 as core::ffi::c_int))
            as WORD32;
        fac_mod = (fac_shift as core::ffi::c_int & 3 as core::ffi::c_int) as WORD32;
        fac_shift = ((fac_shift as core::ffi::c_int >> 2 as core::ffi::c_int)
            + 1 as core::ffi::c_int) as WORD32;
        fac_shift
            += *ptr_spec_scale_prev as core::ffi::c_int
                - (if *ptr_spec_scale_prev as core::ffi::c_int
                    > *ptr_spec_scale_act as core::ffi::c_int
                {
                    *ptr_spec_scale_prev as core::ffi::c_int
                } else {
                    *ptr_spec_scale_act as core::ffi::c_int
                });
        fac_shift = (if (if fac_shift < 32 as core::ffi::c_int - 1 as core::ffi::c_int {
            fac_shift as core::ffi::c_int
        } else {
            32 as core::ffi::c_int - 1 as core::ffi::c_int
        }) > -(32 as core::ffi::c_int - 1 as core::ffi::c_int)
        {
            if fac_shift < 32 as core::ffi::c_int - 1 as core::ffi::c_int {
                fac_shift as core::ffi::c_int
            } else {
                32 as core::ffi::c_int - 1 as core::ffi::c_int
            }
        } else {
            -(32 as core::ffi::c_int - 1 as core::ffi::c_int)
        }) as WORD32;
        while line < *ptr_sfb_offset.offset(sfb as isize) as core::ffi::c_int {
            let mut accu: WORD32 = ixheaac_mult32x16in32_shl(
                *ptr_spec_coeff.offset(line as isize),
                ia_ec_interpolation_fac[fac_mod as usize],
            );
            *ptr_spec_coeff.offset(line as isize) = ixheaac_shl32_dir_sat(
                accu,
                fac_shift as WORD,
            );
            line += 1;
        }
        sfb += 1;
    }
    *ptr_spec_scale_out = (if *ptr_spec_scale_prev as core::ffi::c_int
        > *ptr_spec_scale_act as core::ffi::c_int
    {
        *ptr_spec_scale_prev as core::ffi::c_int
    } else {
        *ptr_spec_scale_act as core::ffi::c_int
    }) as WORD16;
}
unsafe extern "C" fn ixheaacd_aac_ec_state(
    mut pstr_ec_state: *mut ia_ec_state_str,
    mut frame_status: WORD32,
) -> VOID {
    let mut ec_state_val: WORD32 = ((*pstr_ec_state)
        .prev_frame_ok[0 as core::ffi::c_int as usize] << 2 as core::ffi::c_int)
        + ((*pstr_ec_state).prev_frame_ok[1 as core::ffi::c_int as usize]
            << 1 as core::ffi::c_int) + frame_status;
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
unsafe extern "C" fn ixheaacd_aac_ec_interpolate_frame(
    mut pstr_ec_state: *mut ia_ec_state_str,
    mut pstr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
    mut pstr_samp_rate_info: *const ia_usac_samp_rate_info,
    num_samples: WORD32,
    frame_status: WORD32,
    mut pstr_ics_info: *mut ia_ics_info_struct,
) -> VOID {
    let mut ptr_spec_coeff: *mut WORD32 = (*pstr_aac_dec_channel_info).ptr_spec_coeff;
    let mut ptr_spec_scale: *mut WORD16 = (*pstr_aac_dec_channel_info).ptr_scale_factor;
    let mut sfb_nrg_prev: [WORD32; 64] = [0; 64];
    let mut sfb_nrg_act: [WORD32; 64] = [0; 64];
    let mut idx: WORD32 = 0;
    memset(
        sfb_nrg_prev.as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[WORD32; 64]>() as size_t,
    );
    memset(
        sfb_nrg_act.as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[WORD32; 64]>() as size_t,
    );
    if frame_status == 0 {
        (*pstr_ics_info).window_shape = (*pstr_ec_state).win_shape as WORD16;
        (*pstr_ics_info).window_sequence = (*pstr_ec_state).win_seq as WORD16;
        idx = 0 as core::ffi::c_int as WORD32;
        while idx < num_samples {
            *ptr_spec_coeff.offset(idx as isize) = (*pstr_ec_state)
                .spectral_coeff[idx as usize];
            idx += 1;
        }
        memcpy(
            ptr_spec_scale as *mut core::ffi::c_void,
            ((*pstr_ec_state).q_spec_coeff).as_mut_ptr() as *const core::ffi::c_void,
            (8 as size_t).wrapping_mul(::core::mem::size_of::<WORD16>() as size_t),
        );
    }
    if (*pstr_ec_state).prev_frame_ok[1 as core::ffi::c_int as usize] == 0 {
        if frame_status != 0
            && (*pstr_ec_state).prev_frame_ok[0 as core::ffi::c_int as usize] != 0
        {
            if (*pstr_ics_info).window_sequence as core::ffi::c_int
                == EIGHT_SHORT_SEQUENCE
            {
                let mut window: WORD32 = 0;
                if (*pstr_ec_state).win_seq == EIGHT_SHORT_SEQUENCE {
                    let mut total_scale_factor_bands: WORD32 = (*pstr_samp_rate_info)
                        .num_sfb_128 - 1 as WORD32;
                    let mut ptr_sfb_offset: *const WORD16 = (*pstr_samp_rate_info)
                        .ptr_sfb_128;
                    (*pstr_ics_info).window_shape = 1 as WORD16;
                    (*pstr_ics_info).window_sequence = EIGHT_SHORT_SEQUENCE as WORD16;
                    window = 0 as core::ffi::c_int as WORD32;
                    while window < 8 as core::ffi::c_int {
                        ixheaacd_aac_ec_calc_sfb_nrg(
                            &mut *ptr_spec_coeff
                                .offset(
                                    (window as core::ffi::c_int
                                        * (num_samples as core::ffi::c_int / 8 as core::ffi::c_int))
                                        as isize,
                                ),
                            pstr_samp_rate_info,
                            EIGHT_SHORT_SEQUENCE,
                            NO_TRANSITION,
                            sfb_nrg_prev.as_mut_ptr(),
                        );
                        ixheaacd_aac_ec_calc_sfb_nrg(
                            &mut *((*pstr_ec_state).spectral_coeff)
                                .as_mut_ptr()
                                .offset(
                                    (window as core::ffi::c_int
                                        * (num_samples as core::ffi::c_int / 8 as core::ffi::c_int))
                                        as isize,
                                ),
                            pstr_samp_rate_info,
                            EIGHT_SHORT_SEQUENCE,
                            NO_TRANSITION,
                            sfb_nrg_act.as_mut_ptr(),
                        );
                        ixheaacd_aac_ec_interpolate(
                            &mut *ptr_spec_coeff
                                .offset(
                                    (window as core::ffi::c_int
                                        * (num_samples as core::ffi::c_int / 8 as core::ffi::c_int))
                                        as isize,
                                ),
                            &mut *ptr_spec_scale.offset(window as isize),
                            &mut *((*pstr_ec_state).q_spec_coeff)
                                .as_mut_ptr()
                                .offset(window as isize),
                            &mut *ptr_spec_scale.offset(window as isize),
                            sfb_nrg_prev.as_mut_ptr(),
                            sfb_nrg_act.as_mut_ptr(),
                            total_scale_factor_bands,
                            ptr_sfb_offset,
                        );
                        window += 1;
                    }
                } else {
                    let mut total_scale_factor_bands_0: WORD32 = (*pstr_samp_rate_info)
                        .num_sfb_1024 - 1 as WORD32;
                    let mut ptr_sfb_offset_0: *const WORD16 = (*pstr_samp_rate_info)
                        .ptr_sfb_1024;
                    let mut spec_scale_out: WORD16 = 0;
                    ixheaacd_aac_ec_calc_sfb_nrg(
                        &mut *ptr_spec_coeff
                            .offset(
                                (num_samples as core::ffi::c_int
                                    - num_samples as core::ffi::c_int / 8 as core::ffi::c_int)
                                    as isize,
                            ),
                        pstr_samp_rate_info,
                        EIGHT_SHORT_SEQUENCE,
                        TRANS_SHORT_LONG,
                        sfb_nrg_act.as_mut_ptr(),
                    );
                    ixheaacd_aac_ec_calc_sfb_nrg(
                        ((*pstr_ec_state).spectral_coeff).as_mut_ptr(),
                        pstr_samp_rate_info,
                        ONLY_LONG_SEQUENCE,
                        NO_TRANSITION,
                        sfb_nrg_prev.as_mut_ptr(),
                    );
                    (*pstr_ics_info).window_shape = 0 as WORD16;
                    (*pstr_ics_info).window_sequence = LONG_STOP_SEQUENCE as WORD16;
                    idx = 0 as core::ffi::c_int as WORD32;
                    while idx < num_samples {
                        *ptr_spec_coeff.offset(idx as isize) = (*pstr_ec_state)
                            .spectral_coeff[idx as usize];
                        idx += 1;
                    }
                    idx = 0 as core::ffi::c_int as WORD32;
                    while idx < 8 as core::ffi::c_int {
                        if *ptr_spec_scale.offset(idx as isize) as core::ffi::c_int
                            > *ptr_spec_scale.offset(0 as core::ffi::c_int as isize)
                                as core::ffi::c_int
                        {
                            *ptr_spec_scale.offset(0 as core::ffi::c_int as isize) = *ptr_spec_scale
                                .offset(idx as isize);
                        }
                        idx += 1;
                    }
                    ixheaacd_aac_ec_interpolate(
                        ptr_spec_coeff,
                        &mut *((*pstr_ec_state).q_spec_coeff)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize),
                        &mut *ptr_spec_scale.offset(0 as core::ffi::c_int as isize),
                        &mut spec_scale_out,
                        sfb_nrg_prev.as_mut_ptr(),
                        sfb_nrg_act.as_mut_ptr(),
                        total_scale_factor_bands_0,
                        ptr_sfb_offset_0,
                    );
                    *ptr_spec_scale.offset(0 as core::ffi::c_int as isize) = spec_scale_out;
                }
            } else {
                let mut total_scale_factor_bands_1: WORD32 = (*pstr_samp_rate_info)
                    .num_sfb_1024 - 1 as WORD32;
                let mut ptr_sfb_offset_1: *const WORD16 = (*pstr_samp_rate_info)
                    .ptr_sfb_1024;
                let mut spec_scale_act: WORD16 = (*pstr_ec_state)
                    .q_spec_coeff[0 as core::ffi::c_int as usize];
                ixheaacd_aac_ec_calc_sfb_nrg(
                    ptr_spec_coeff,
                    pstr_samp_rate_info,
                    ONLY_LONG_SEQUENCE,
                    NO_TRANSITION,
                    sfb_nrg_prev.as_mut_ptr(),
                );
                if (*pstr_ec_state).win_seq == EIGHT_SHORT_SEQUENCE {
                    (*pstr_ics_info).window_shape = 1 as WORD16;
                    (*pstr_ics_info).window_sequence = LONG_START_SEQUENCE as WORD16;
                    idx = 1 as core::ffi::c_int as WORD32;
                    while idx < 8 as core::ffi::c_int {
                        if (*pstr_ec_state).q_spec_coeff[idx as usize]
                            as core::ffi::c_int > spec_scale_act as core::ffi::c_int
                        {
                            spec_scale_act = (*pstr_ec_state).q_spec_coeff[idx as usize];
                        }
                        idx += 1;
                    }
                    ixheaacd_aac_ec_calc_sfb_nrg(
                        ((*pstr_ec_state).spectral_coeff).as_mut_ptr(),
                        pstr_samp_rate_info,
                        EIGHT_SHORT_SEQUENCE,
                        TRANS_SHORT_LONG,
                        sfb_nrg_act.as_mut_ptr(),
                    );
                } else {
                    (*pstr_ics_info).window_shape = 0 as WORD16;
                    (*pstr_ics_info).window_sequence = ONLY_LONG_SEQUENCE as WORD16;
                    ixheaacd_aac_ec_calc_sfb_nrg(
                        ((*pstr_ec_state).spectral_coeff).as_mut_ptr(),
                        pstr_samp_rate_info,
                        ONLY_LONG_SEQUENCE,
                        NO_TRANSITION,
                        sfb_nrg_act.as_mut_ptr(),
                    );
                }
                ixheaacd_aac_ec_interpolate(
                    ptr_spec_coeff,
                    &mut *ptr_spec_scale.offset(0 as core::ffi::c_int as isize),
                    &mut spec_scale_act,
                    &mut *ptr_spec_scale.offset(0 as core::ffi::c_int as isize),
                    sfb_nrg_prev.as_mut_ptr(),
                    sfb_nrg_act.as_mut_ptr(),
                    total_scale_factor_bands_1,
                    ptr_sfb_offset_1,
                );
            }
        }
        ixheaacd_aac_ec_flip_spec_sign(ptr_spec_coeff, num_samples);
    }
    if FRAME_MUTE == (*pstr_ec_state).conceal_state {
        (*pstr_ics_info).window_shape = (*pstr_ec_state).win_shape as WORD16;
        (*pstr_ics_info).window_sequence = ixheaacd_aac_ec_get_win_seq(
            (*pstr_ec_state).win_seq,
        ) as WORD16;
        (*pstr_ec_state).win_seq = (*pstr_ics_info).window_sequence as WORD32;
        memset(
            ptr_spec_coeff as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (num_samples as size_t)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_aac_ec_init(
    mut pstr_ec_state: *mut ia_ec_state_str,
) -> VOID {
    (*pstr_ec_state).win_shape = CONCEAL_NOT_DEFINED;
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
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_aac_apply_ec(
    mut pstr_ec_state: *mut ia_ec_state_str,
    mut pstr_aac_dec_channel_info: *mut ia_aac_dec_channel_info_struct,
    mut pstr_samp_rate_info: *const ia_usac_samp_rate_info,
    num_samples: WORD32,
    mut pstr_ics_info: *mut ia_ics_info_struct,
    frame_status: WORD32,
) -> VOID {
    if (*pstr_ec_state).win_shape as core::ffi::c_int
        == CONCEAL_NOT_DEFINED as core::ffi::c_int
    {
        (*pstr_ec_state).win_shape = (*pstr_ics_info).window_shape as UWORD8;
    }
    if frame_status != 0
        && (*pstr_ec_state).prev_frame_ok[1 as core::ffi::c_int as usize] != 0
    {
        ixheaacd_aac_ec_store(pstr_ec_state, pstr_aac_dec_channel_info, pstr_ics_info);
    }
    ixheaacd_aac_ec_state(pstr_ec_state, frame_status);
    ixheaacd_aac_ec_interpolate_frame(
        pstr_ec_state,
        pstr_aac_dec_channel_info,
        pstr_samp_rate_info,
        num_samples,
        frame_status,
        pstr_ics_info,
    );
    (*pstr_ec_state).prev_frame_ok[0 as core::ffi::c_int as usize] = (*pstr_ec_state)
        .prev_frame_ok[1 as core::ffi::c_int as usize];
    (*pstr_ec_state).prev_frame_ok[1 as core::ffi::c_int as usize] = frame_status;
}
pub const MAX_32: WORD32 = 0x7fffffff as core::ffi::c_long as WORD32;
pub const MIN_32: WORD32 = 0x80000000 as core::ffi::c_long as WORD32;
pub const ONLY_LONG_SEQUENCE: core::ffi::c_int = 0 as core::ffi::c_int;
pub const LONG_START_SEQUENCE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const EIGHT_SHORT_SEQUENCE: core::ffi::c_int = 2 as core::ffi::c_int;
pub const LONG_STOP_SEQUENCE: core::ffi::c_int = 3 as core::ffi::c_int;
