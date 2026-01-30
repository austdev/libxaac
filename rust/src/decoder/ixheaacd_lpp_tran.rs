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
    fn ixheaacd_invfilt_level_emphasis(
        hf_generator: *mut ia_sbr_hf_generator_struct,
        num_if_bands: WORD32,
        sbr_invf_mode: *mut WORD32,
        sbr_invf_mode_prev: *mut WORD32,
        bw_array: *mut WORD32,
    ) -> VOID;
    static mut ixheaacd_fix_div: Option<unsafe extern "C" fn(WORD32, WORD32) -> WORD32>;
    static mut ixheaacd_covariance_matrix_calc: Option<
        unsafe extern "C" fn(
            *mut WORD32,
            *mut ia_lpp_trans_cov_matrix,
            WORD32,
            WORD32,
        ) -> VOID,
    >;
    static mut ixheaacd_covariance_matrix_calc_960: Option<
        unsafe extern "C" fn(
            *mut WORD32,
            *mut ia_lpp_trans_cov_matrix,
            WORD32,
            WORD32,
        ) -> VOID,
    >;
    static mut ixheaacd_covariance_matrix_calc_2: Option<
        unsafe extern "C" fn(
            *mut ia_lpp_trans_cov_matrix,
            *mut WORD32,
            WORD32,
            WORD16,
        ) -> VOID,
    >;
}
pub type size_t = usize;
pub type WORD16 = core::ffi::c_short;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type WORD64 = core::ffi::c_longlong;
pub type VOID = ();
pub type FLAG = core::ffi::c_int;
pub type WORD = core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_scale_fact_struct {
    pub lb_scale: WORD16,
    pub st_lb_scale: WORD16,
    pub ov_lb_scale: WORD16,
    pub hb_scale: WORD16,
    pub ov_hb_scale: WORD16,
    pub st_syn_scale: WORD16,
    pub ps_scale: WORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_lpp_trans_cov_matrix {
    pub phi_11: WORD32,
    pub phi_22: WORD32,
    pub phi_01: WORD32,
    pub phi_02: WORD32,
    pub phi_12: WORD32,
    pub phi_01_im: WORD32,
    pub phi_02_im: WORD32,
    pub phi_12_im: WORD32,
    pub d: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_patch_param_struct {
    pub src_start_band: WORD16,
    pub src_end_band: WORD16,
    pub guard_start_band: WORD16,
    pub dst_start_band: WORD16,
    pub dst_end_band: WORD16,
    pub num_bands_in_patch: WORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_transposer_settings_struct {
    pub num_columns: WORD16,
    pub num_patches: WORD16,
    pub start_patch: WORD16,
    pub stop_patch: WORD16,
    pub bw_borders: [WORD16; 10],
    pub str_patch_param: [ia_patch_param_struct; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_hf_generator_struct {
    pub pstr_settings: *mut ia_transposer_settings_struct,
    pub bw_array_prev: [WORD32; 6],
    pub lpc_filt_states_real: [*mut WORD32; 2],
    pub lpc_filt_states_imag: [*mut WORD32; 2],
}
pub const AOT_ER_AAC_LD: AUDIO_OBJECT_TYPE = 23;
pub const AOT_ER_AAC_ELD: AUDIO_OBJECT_TYPE = 39;
pub type AUDIO_OBJECT_TYPE = core::ffi::c_uint;
pub const AOT_USAC: AUDIO_OBJECT_TYPE = 42;
pub const AOT_ESC: AUDIO_OBJECT_TYPE = 31;
pub const AOT_RSVD_31: AUDIO_OBJECT_TYPE = 31;
pub const AOT_RSVD_30: AUDIO_OBJECT_TYPE = 30;
pub const AOT_PS: AUDIO_OBJECT_TYPE = 29;
pub const AOT_RSVD_28: AUDIO_OBJECT_TYPE = 28;
pub const AOT_ER_PARA: AUDIO_OBJECT_TYPE = 27;
pub const AOT_ER_HILN: AUDIO_OBJECT_TYPE = 26;
pub const AOT_ER_HVXC: AUDIO_OBJECT_TYPE = 25;
pub const AOT_ER_CELP: AUDIO_OBJECT_TYPE = 24;
pub const AOT_ER_BSAC: AUDIO_OBJECT_TYPE = 22;
pub const AOT_ER_TWIN_VQ: AUDIO_OBJECT_TYPE = 21;
pub const AOT_ER_AAC_SCAL: AUDIO_OBJECT_TYPE = 20;
pub const AOT_ER_AAC_LTP: AUDIO_OBJECT_TYPE = 19;
pub const AOT_RSVD_18: AUDIO_OBJECT_TYPE = 18;
pub const AOT_ER_AAC_LC: AUDIO_OBJECT_TYPE = 17;
pub const AOT_ALG_SYNTH_AUD_FX: AUDIO_OBJECT_TYPE = 16;
pub const AOT_GEN_MIDI: AUDIO_OBJECT_TYPE = 15;
pub const AOT_WAV_TAB_SYNTH: AUDIO_OBJECT_TYPE = 14;
pub const AOT_MAIN_SYNTH: AUDIO_OBJECT_TYPE = 13;
pub const AOT_TTSI: AUDIO_OBJECT_TYPE = 12;
pub const AOT_RSVD_11: AUDIO_OBJECT_TYPE = 11;
pub const AOT_RSVD_10: AUDIO_OBJECT_TYPE = 10;
pub const AOT_HVXC: AUDIO_OBJECT_TYPE = 9;
pub const AOT_CELP: AUDIO_OBJECT_TYPE = 8;
pub const AOT_TWIN_VQ: AUDIO_OBJECT_TYPE = 7;
pub const AOT_AAC_SCAL: AUDIO_OBJECT_TYPE = 6;
pub const AOT_SBR: AUDIO_OBJECT_TYPE = 5;
pub const AOT_AAC_LTP: AUDIO_OBJECT_TYPE = 4;
pub const AOT_AAC_SSR: AUDIO_OBJECT_TYPE = 3;
pub const AOT_AAC_LC: AUDIO_OBJECT_TYPE = 2;
pub const AOT_AAC_MAIN: AUDIO_OBJECT_TYPE = 1;
pub const AOT_NULL_OBJECT: AUDIO_OBJECT_TYPE = 0;
pub const MAX_32: WORD32 = 0x7fffffff as core::ffi::c_long as WORD32;
pub const MIN_32: WORD32 = 0x80000000 as core::ffi::c_long as WORD32;
pub const MAX_16: WORD16 = 0x7fff as core::ffi::c_int as WORD16;
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
unsafe extern "C" fn ixheaac_shl32_dir(mut a: WORD32, mut b: WORD) -> WORD32 {
    let mut out_val: WORD32 = 0;
    if b < 0 as core::ffi::c_int {
        out_val = ixheaac_shr32(a, -b);
    } else {
        out_val = ixheaac_shl32(a, b);
    }
    return out_val;
}
#[inline]
unsafe extern "C" fn ixheaac_mult16x16in32(mut a: WORD16, mut b: WORD16) -> WORD32 {
    let mut product: WORD32 = 0;
    product = a as WORD32 * b as WORD32;
    return product;
}
#[inline]
unsafe extern "C" fn ixheaac_mult32x16hin32(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * (b >> 16 as core::ffi::c_int) as WORD64;
    result = (temp_result >> 16 as core::ffi::c_int) as WORD32;
    return result;
}
#[inline]
unsafe extern "C" fn ixheaac_mult16x16in32_shl(mut a: WORD16, mut b: WORD16) -> WORD32 {
    let mut product: WORD32 = 0;
    product = ixheaac_shl32(ixheaac_mult16x16in32(a, b), 1 as WORD);
    return product;
}
#[inline]
unsafe extern "C" fn ixheaac_add32(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut sum: WORD32 = 0;
    sum = a + b;
    return sum;
}
#[inline]
unsafe extern "C" fn ixheaac_sub32(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut diff: WORD32 = 0;
    diff = a - b;
    return diff;
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
unsafe extern "C" fn ixheaac_pnorm32(mut a: WORD32) -> WORD {
    let mut norm_val: WORD = 0;
    if a == 0 as core::ffi::c_int {
        norm_val = 31 as core::ffi::c_int as WORD;
    } else {
        norm_val = 0 as core::ffi::c_int as WORD;
        while a < 0x40000000 as core::ffi::c_long as WORD32 {
            a <<= 1 as core::ffi::c_int;
            norm_val += 1;
        }
    }
    return norm_val;
}
#[inline]
unsafe extern "C" fn ixheaac_abs32(mut a: WORD32) -> WORD32 {
    let mut abs_val: WORD32 = 0;
    abs_val = a;
    if a < 0 as core::ffi::c_int {
        abs_val = -a;
    }
    return abs_val;
}
#[inline]
unsafe extern "C" fn ixheaac_abs32_nrm(mut a: WORD32) -> WORD32 {
    let mut abs_val: WORD32 = 0;
    abs_val = a;
    if a < 0 as core::ffi::c_int {
        abs_val = !a;
    }
    return abs_val;
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
unsafe extern "C" fn ixheaac_sat16(mut op1: WORD32) -> WORD16 {
    let mut var_out: WORD16 = 0;
    if op1 as core::ffi::c_long > 0x7fff as core::ffi::c_long {
        var_out = MAX_16;
    } else if op1 < 0xffff8000 as core::ffi::c_long as WORD32 {
        var_out = -(32768 as core::ffi::c_int) as WORD16;
    } else {
        var_out = op1 as WORD16;
    }
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_add16(mut op1: WORD16, mut op2: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = (op1 as core::ffi::c_int + op2 as core::ffi::c_int) as WORD16;
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_sub16(mut op1: WORD16, mut op2: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = (op1 as core::ffi::c_int - op2 as core::ffi::c_int) as WORD16;
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_sub16_sat(mut op1: WORD16, mut op2: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    let mut diff: WORD32 = 0;
    diff = (op1 as core::ffi::c_int - op2 as core::ffi::c_int) as WORD32;
    var_out = ixheaac_sat16(diff);
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_mult16_shl_sat(mut op1: WORD16, mut op2: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    let mut temp: WORD32 = 0;
    temp = op1 as WORD32 * op2 as WORD32 >> 15 as core::ffi::c_int;
    var_out = ixheaac_sat16(temp);
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_shl16(mut op1: WORD16, mut shift: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = ((op1 as core::ffi::c_int) << shift as core::ffi::c_int) as WORD16;
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_max16(mut op1: WORD16, mut op2: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = (if op1 as core::ffi::c_int > op2 as core::ffi::c_int {
        op1 as core::ffi::c_int
    } else {
        op2 as core::ffi::c_int
    }) as WORD16;
    return var_out;
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
unsafe extern "C" fn ixheaac_mult32x16in32(mut a: WORD32, mut b: WORD16) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 16 as core::ffi::c_int) as WORD32;
    return result;
}
#[inline]
unsafe extern "C" fn ixheaac_mult32x16in32_shl_sat(
    mut a: WORD32,
    mut b: WORD16,
) -> WORD32 {
    let mut result: WORD32 = 0;
    if a == 0x80000000 as core::ffi::c_uint as WORD32
        && b as core::ffi::c_int
            == 0x8000 as core::ffi::c_int as WORD16 as core::ffi::c_int
    {
        result = 0x7fffffff as core::ffi::c_int;
    } else {
        result = ixheaac_mult32x16in32_shl(a, b);
    }
    return result;
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
unsafe extern "C" fn ixheaac_extract16h(mut var: WORD32) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = (var >> 16 as core::ffi::c_int) as WORD16;
    return var_out;
}
pub const SBR_UPSAMPLE_FAC: core::ffi::c_int = 2 as core::ffi::c_int;
pub const NO_SYNTHESIS_CHANNELS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const NO_ANALYSIS_CHANNELS: core::ffi::c_int = NO_SYNTHESIS_CHANNELS
    / SBR_UPSAMPLE_FAC;
pub const MAX_NOISE_ENVELOPES: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_NOISE_COEFFS: core::ffi::c_int = 5 as core::ffi::c_int;
pub const MAX_NUM_NOISE_VALUES: core::ffi::c_int = MAX_NOISE_ENVELOPES
    * MAX_NOISE_COEFFS;
pub const LPC_ORDER: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_NUM_PATCHES: core::ffi::c_int = 6 as core::ffi::c_int;
pub const LPC_SCALE_FACTOR: core::ffi::c_int = 2 as core::ffi::c_int;
pub const AUTO_CORR_LEN_1024: core::ffi::c_int = 38 as core::ffi::c_int;
pub const AUTO_CORR_LEN_960: core::ffi::c_int = 36 as core::ffi::c_int;
#[inline]
unsafe extern "C" fn ixheaacd_mac32x16hin32(
    mut a: WORD32,
    mut b: WORD32,
    mut c: WORD32,
) -> WORD32 {
    let mut result: WORD32 = 0;
    result = a + ixheaac_mult32x16hin32(b, c);
    return result;
}
#[inline]
unsafe extern "C" fn ixheaacd_macn32x16hin32(
    mut a: WORD32,
    mut b: WORD32,
    mut c: WORD32,
) -> WORD32 {
    let mut result: WORD32 = 0;
    result = a - ixheaac_mult32x16hin32(b, c);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_filterstep3(
    mut a0r: WORD16,
    mut a0i: WORD16,
    mut a1r: WORD16,
    mut a1i: WORD16,
    mut start_indx: WORD32,
    mut stop_idx: WORD32,
    mut low_band: WORD32,
    mut high_band: WORD32,
    mut qmf_buffer: *mut WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut prev1r: WORD32 = 0;
    let mut prev1i: WORD32 = 0;
    let mut prev2r: WORD32 = 0;
    let mut prev2i: WORD32 = 0;
    let mut coef1r: WORD16 = a0r;
    let mut coef1i: WORD16 = a0i;
    let mut coef2r: WORD16 = a1r;
    let mut coef2i: WORD16 = a1i;
    let mut p_src: *mut WORD32 = 0 as *mut WORD32;
    let mut p_dst: *mut WORD32 = 0 as *mut WORD32;
    let mut qmf_real: WORD32 = 0;
    let mut qmf_imag: WORD32 = 0;
    let mut curr: WORD32 = 0;
    let mut curi: WORD32 = 0;
    p_src = qmf_buffer
        .offset(low_band as isize)
        .offset((start_indx << 7 as core::ffi::c_int) as isize);
    prev2r = *p_src;
    p_src = p_src.offset(64 as core::ffi::c_int as isize);
    prev2i = *p_src;
    p_src = p_src.offset(64 as core::ffi::c_int as isize);
    prev1r = *p_src;
    p_src = p_src.offset(64 as core::ffi::c_int as isize);
    prev1i = *p_src;
    p_src = p_src.offset(64 as core::ffi::c_int as isize);
    p_dst = qmf_buffer
        .offset(high_band as isize)
        .offset(
            ((start_indx as core::ffi::c_int + 2 as core::ffi::c_int)
                << 7 as core::ffi::c_int) as isize,
        );
    i = stop_idx - start_indx;
    while i != 0 as core::ffi::c_int {
        let mut accu: WORD32 = 0;
        curr = *p_src;
        p_src = p_src.offset(64 as core::ffi::c_int as isize);
        curi = *p_src;
        p_src = p_src.offset(64 as core::ffi::c_int as isize);
        qmf_real = curr >> LPC_SCALE_FACTOR;
        qmf_imag = curi >> LPC_SCALE_FACTOR;
        accu = ixheaac_sub32(
            ixheaac_add32(
                ixheaac_sub32(
                    ixheaac_mult32x16in32(prev1r, coef1r),
                    ixheaac_mult32x16in32(prev1i, coef1i),
                ),
                ixheaac_mult32x16in32(prev2r, coef2r),
            ),
            ixheaac_mult32x16in32(prev2i, coef2i),
        );
        *p_dst = ixheaac_add32(qmf_real, accu << 1 as core::ffi::c_int);
        p_dst = p_dst.offset(64 as core::ffi::c_int as isize);
        accu = ixheaac_add32(
            ixheaac_add32_sat(
                ixheaac_add32_sat(
                    ixheaac_mult32x16in32(prev1r, coef1i),
                    ixheaac_mult32x16in32(prev1i, coef1r),
                ),
                ixheaac_mult32x16in32(prev2r, coef2i),
            ),
            ixheaac_mult32x16in32(prev2i, coef2r),
        );
        *p_dst = ixheaac_add32(qmf_imag, accu << 1 as core::ffi::c_int);
        p_dst = p_dst.offset(64 as core::ffi::c_int as isize);
        prev2r = prev1r;
        prev1r = curr;
        prev2i = prev1i;
        prev1i = curi;
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_covariance_matrix_calc_dec_960(
    mut sub_sign_xlow: *mut WORD32,
    mut cov_matrix: *mut ia_lpp_trans_cov_matrix,
    mut count: WORD32,
    mut len: WORD32,
) -> VOID {
    let mut j: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut ixheaacd_drc_offset: WORD32 = 2 as WORD32;
    let mut factor: WORD32 = 0;
    let mut max_val: WORD32 = 0;
    let mut q_factor: WORD32 = 0;
    let mut temp1: WORD32 = 0;
    let mut temp2: WORD32 = 0;
    let mut temp3: WORD32 = 0;
    let mut temp4: WORD32 = 0;
    let mut temp_buf_ptr: *mut WORD32 = sub_sign_xlow;
    temp3 = 0 as core::ffi::c_int as WORD32;
    k = count;
    while k > 0 as core::ffi::c_int {
        let mut t_phi_01: WORD32 = 0 as WORD32;
        let mut t_phi_02: WORD32 = 0 as WORD32;
        let mut t_phi_11: WORD32 = 0 as WORD32;
        let mut t_phi_12: WORD32 = 0 as WORD32;
        let mut t_phi_22: WORD32 = 0 as WORD32;
        factor = -(3 as core::ffi::c_int) as WORD32;
        j = ixheaacd_drc_offset;
        sub_sign_xlow = temp_buf_ptr;
        temp1 = ixheaac_shl32_dir(*sub_sign_xlow, factor as WORD);
        sub_sign_xlow = sub_sign_xlow.offset(64 as core::ffi::c_int as isize);
        temp2 = ixheaac_shl32_dir(*sub_sign_xlow, factor as WORD);
        sub_sign_xlow = sub_sign_xlow.offset(64 as core::ffi::c_int as isize);
        loop {
            j = (j as core::ffi::c_int + 3 as core::ffi::c_int) as WORD32;
            if !(j <= ixheaacd_drc_offset + len) {
                break;
            }
            temp3 = ixheaac_shl32_dir(*sub_sign_xlow, factor as WORD);
            sub_sign_xlow = sub_sign_xlow.offset(64 as core::ffi::c_int as isize);
            t_phi_01 += ixheaac_mult32x16hin32(temp3, temp2);
            t_phi_02 += ixheaac_mult32x16hin32(temp3, temp1);
            t_phi_11 += ixheaac_mult32x16hin32(temp2, temp2);
            temp1 = ixheaac_shl32_dir(*sub_sign_xlow, factor as WORD);
            sub_sign_xlow = sub_sign_xlow.offset(64 as core::ffi::c_int as isize);
            t_phi_01 += ixheaac_mult32x16hin32(temp1, temp3);
            t_phi_02 += ixheaac_mult32x16hin32(temp1, temp2);
            t_phi_11 += ixheaac_mult32x16hin32(temp3, temp3);
            temp2 = ixheaac_shl32_dir(*sub_sign_xlow, factor as WORD);
            sub_sign_xlow = sub_sign_xlow.offset(64 as core::ffi::c_int as isize);
            t_phi_01 += ixheaac_mult32x16hin32(temp2, temp1);
            t_phi_02 += ixheaac_mult32x16hin32(temp2, temp3);
            t_phi_11 += ixheaac_mult32x16hin32(temp1, temp1);
        }
        if AUTO_CORR_LEN_1024 == len {
            temp3 = ixheaac_shl32_dir(*sub_sign_xlow, factor as WORD);
            sub_sign_xlow = sub_sign_xlow.offset(64 as core::ffi::c_int as isize);
            t_phi_01 += ixheaac_mult32x16hin32(temp3, temp2);
            t_phi_02 += ixheaac_mult32x16hin32(temp3, temp1);
            t_phi_11 += ixheaac_mult32x16hin32(temp2, temp2);
            temp1 = ixheaac_shl32_dir(*sub_sign_xlow, factor as WORD);
            sub_sign_xlow = sub_sign_xlow.offset(64 as core::ffi::c_int as isize);
            t_phi_01 += ixheaac_mult32x16hin32(temp1, temp3);
            t_phi_02 += ixheaac_mult32x16hin32(temp1, temp2);
            t_phi_11 += ixheaac_mult32x16hin32(temp3, temp3);
        }
        temp2 = ixheaac_shl32_dir(*temp_buf_ptr, factor as WORD);
        temp4 = ixheaac_shl32_dir(
            *temp_buf_ptr.offset(64 as core::ffi::c_int as isize),
            factor as WORD,
        );
        if AUTO_CORR_LEN_960 == len {
            temp3 = ixheaac_shl32_dir(
                *sub_sign_xlow.offset(-(128 as core::ffi::c_int) as isize),
                factor as WORD,
            );
            temp1 = ixheaac_shl32_dir(
                *sub_sign_xlow.offset(-(64 as core::ffi::c_int) as isize),
                factor as WORD,
            );
        }
        t_phi_12 = t_phi_01 - ixheaac_mult32x16hin32(temp1, temp3)
            + ixheaac_mult32x16hin32(temp4, temp2);
        t_phi_22 = t_phi_11 - ixheaac_mult32x16hin32(temp3, temp3)
            + ixheaac_mult32x16hin32(temp2, temp2);
        max_val = ixheaac_abs32_nrm(t_phi_01);
        max_val = max_val | ixheaac_abs32_nrm(t_phi_02);
        max_val = max_val | ixheaac_abs32_nrm(t_phi_12);
        max_val = max_val | t_phi_11;
        max_val = max_val | t_phi_22;
        q_factor = ixheaac_pnorm32(max_val) as WORD32;
        (*cov_matrix).phi_11 = t_phi_11 << q_factor;
        (*cov_matrix).phi_22 = t_phi_22 << q_factor;
        (*cov_matrix).phi_01 = t_phi_01 << q_factor;
        (*cov_matrix).phi_02 = t_phi_02 << q_factor;
        (*cov_matrix).phi_12 = t_phi_12 << q_factor;
        (*cov_matrix).d = ixheaac_sub32_sat(
            ixheaac_mult32((*cov_matrix).phi_22, (*cov_matrix).phi_11),
            ixheaac_mult32((*cov_matrix).phi_12, (*cov_matrix).phi_12),
        );
        cov_matrix = cov_matrix.offset(1);
        temp_buf_ptr = temp_buf_ptr.offset(1);
        k -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_covariance_matrix_calc_dec(
    mut sub_sign_xlow: *mut WORD32,
    mut cov_matrix: *mut ia_lpp_trans_cov_matrix,
    mut count: WORD32,
    mut len: WORD32,
) -> VOID {
    let mut j: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut ixheaacd_drc_offset: WORD32 = 2 as WORD32;
    let mut factor: WORD32 = 0;
    let mut max_val: WORD32 = 0;
    let mut q_factor: WORD32 = 0;
    let mut temp1: WORD32 = 0;
    let mut temp2: WORD32 = 0;
    let mut temp3: WORD32 = 0;
    let mut temp4: WORD32 = 0;
    let mut temp_buf_ptr: *mut WORD32 = sub_sign_xlow;
    k = count;
    while k > 0 as core::ffi::c_int {
        let mut t_phi_01: WORD32 = 0 as WORD32;
        let mut t_phi_02: WORD32 = 0 as WORD32;
        let mut t_phi_11: WORD32 = 0 as WORD32;
        let mut t_phi_12: WORD32 = 0 as WORD32;
        let mut t_phi_22: WORD32 = 0 as WORD32;
        factor = -(3 as core::ffi::c_int) as WORD32;
        j = ixheaacd_drc_offset;
        sub_sign_xlow = temp_buf_ptr;
        temp1 = ixheaac_shl32_dir(*sub_sign_xlow, factor as WORD);
        sub_sign_xlow = sub_sign_xlow.offset(64 as core::ffi::c_int as isize);
        temp2 = ixheaac_shl32_dir(*sub_sign_xlow, factor as WORD);
        sub_sign_xlow = sub_sign_xlow.offset(64 as core::ffi::c_int as isize);
        loop {
            j = (j as core::ffi::c_int + 3 as core::ffi::c_int) as WORD32;
            if !(j <= ixheaacd_drc_offset + len) {
                break;
            }
            temp3 = ixheaac_shl32_dir(*sub_sign_xlow, factor as WORD);
            sub_sign_xlow = sub_sign_xlow.offset(64 as core::ffi::c_int as isize);
            t_phi_01 += ixheaac_mult32x16hin32(temp3, temp2);
            t_phi_02 += ixheaac_mult32x16hin32(temp3, temp1);
            t_phi_11 += ixheaac_mult32x16hin32(temp2, temp2);
            temp1 = ixheaac_shl32_dir(*sub_sign_xlow, factor as WORD);
            sub_sign_xlow = sub_sign_xlow.offset(64 as core::ffi::c_int as isize);
            t_phi_01 += ixheaac_mult32x16hin32(temp1, temp3);
            t_phi_02 += ixheaac_mult32x16hin32(temp1, temp2);
            t_phi_11 += ixheaac_mult32x16hin32(temp3, temp3);
            temp2 = ixheaac_shl32_dir(*sub_sign_xlow, factor as WORD);
            sub_sign_xlow = sub_sign_xlow.offset(64 as core::ffi::c_int as isize);
            t_phi_01 += ixheaac_mult32x16hin32(temp2, temp1);
            t_phi_02 += ixheaac_mult32x16hin32(temp2, temp3);
            t_phi_11 += ixheaac_mult32x16hin32(temp1, temp1);
        }
        if AUTO_CORR_LEN_960 != len {
            temp3 = ixheaac_shl32_dir(*sub_sign_xlow, factor as WORD);
            sub_sign_xlow = sub_sign_xlow.offset(64 as core::ffi::c_int as isize);
            t_phi_01 += ixheaac_mult32x16hin32(temp3, temp2);
            t_phi_02 += ixheaac_mult32x16hin32(temp3, temp1);
            t_phi_11 += ixheaac_mult32x16hin32(temp2, temp2);
            temp1 = ixheaac_shl32_dir(*sub_sign_xlow, factor as WORD);
            sub_sign_xlow = sub_sign_xlow.offset(64 as core::ffi::c_int as isize);
            t_phi_01 += ixheaac_mult32x16hin32(temp1, temp3);
            t_phi_02 += ixheaac_mult32x16hin32(temp1, temp2);
            t_phi_11 += ixheaac_mult32x16hin32(temp3, temp3);
        }
        if AUTO_CORR_LEN_960 == len {
            temp3 = ixheaac_shl32_dir(
                *sub_sign_xlow.offset(-(128 as core::ffi::c_int) as isize),
                factor as WORD,
            );
            temp3 = ixheaac_shl32_dir(
                *sub_sign_xlow.offset(-(64 as core::ffi::c_int) as isize),
                factor as WORD,
            );
        }
        temp2 = ixheaac_shl32_dir(*temp_buf_ptr, factor as WORD);
        temp4 = ixheaac_shl32_dir(
            *temp_buf_ptr.offset(64 as core::ffi::c_int as isize),
            factor as WORD,
        );
        t_phi_12 = t_phi_01 - ixheaac_mult32x16hin32(temp1, temp3)
            + ixheaac_mult32x16hin32(temp4, temp2);
        t_phi_22 = t_phi_11 - ixheaac_mult32x16hin32(temp3, temp3)
            + ixheaac_mult32x16hin32(temp2, temp2);
        max_val = ixheaac_abs32_nrm(t_phi_01);
        max_val = max_val | ixheaac_abs32_nrm(t_phi_02);
        max_val = max_val | ixheaac_abs32_nrm(t_phi_12);
        max_val = max_val | t_phi_11;
        max_val = max_val | t_phi_22;
        q_factor = ixheaac_pnorm32(max_val) as WORD32;
        (*cov_matrix).phi_11 = t_phi_11 << q_factor;
        (*cov_matrix).phi_22 = t_phi_22 << q_factor;
        (*cov_matrix).phi_01 = t_phi_01 << q_factor;
        (*cov_matrix).phi_02 = t_phi_02 << q_factor;
        (*cov_matrix).phi_12 = t_phi_12 << q_factor;
        (*cov_matrix).d = ixheaac_sub32_sat(
            ixheaac_mult32((*cov_matrix).phi_22, (*cov_matrix).phi_11),
            ixheaac_mult32((*cov_matrix).phi_12, (*cov_matrix).phi_12),
        );
        cov_matrix = cov_matrix.offset(1);
        temp_buf_ptr = temp_buf_ptr.offset(1);
        k -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_covariance_matrix_calc_2_dec(
    mut cov_matrix: *mut ia_lpp_trans_cov_matrix,
    mut real_buffer: *mut WORD32,
    mut num_bands: WORD32,
    mut slots: WORD16,
) -> VOID {
    let mut k: WORD32 = 0;
    let mut img_buffer: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr_real: *mut WORD32 = real_buffer;
    let mut pac_arr: *mut ia_lpp_trans_cov_matrix = cov_matrix;
    k = 0 as core::ffi::c_int as WORD32;
    while k < num_bands {
        let mut t_phi_11: WORD32 = 0 as WORD32;
        let mut t_phi_01: WORD32 = 0 as WORD32;
        let mut t_phi_01_i: WORD32 = 0 as WORD32;
        let mut prev_real: WORD32 = 0;
        let mut prev_imag: WORD32 = 0;
        let mut curr_real: WORD32 = 0;
        let mut curr_imag: WORD32 = 0;
        real_buffer = ptr_real;
        img_buffer = real_buffer.offset(64 as core::ffi::c_int as isize);
        cov_matrix = pac_arr;
        prev_real = *real_buffer.offset(-(128 as core::ffi::c_int) as isize);
        prev_imag = *img_buffer.offset(-(128 as core::ffi::c_int) as isize);
        curr_real = *real_buffer.offset(0 as core::ffi::c_int as isize);
        curr_imag = *img_buffer.offset(0 as core::ffi::c_int as isize);
        curr_real = ixheaac_shr32(curr_real, 3 as WORD);
        curr_imag = ixheaac_shr32(curr_imag, 3 as WORD);
        prev_real = ixheaac_shr32(prev_real, 3 as WORD);
        prev_imag = ixheaac_shr32(prev_imag, 3 as WORD);
        t_phi_01 = ixheaac_mult32x16hin32(curr_real, prev_real);
        t_phi_01 = ixheaacd_mac32x16hin32(t_phi_01, curr_imag, prev_imag);
        t_phi_01_i = ixheaac_mult32x16hin32(curr_imag, prev_real);
        t_phi_01_i = ixheaacd_macn32x16hin32(t_phi_01_i, curr_real, prev_imag);
        t_phi_11 = ixheaac_mult32x16hin32(prev_real, prev_real);
        t_phi_11 = ixheaacd_mac32x16hin32(t_phi_11, prev_imag, prev_imag);
        let mut n: WORD = 0;
        let mut real1: *mut WORD32 = &mut *real_buffer
            .offset(128 as core::ffi::c_int as isize) as *mut WORD32;
        let mut imag1: *mut WORD32 = &mut *img_buffer
            .offset(128 as core::ffi::c_int as isize) as *mut WORD32;
        prev_real = curr_real;
        prev_imag = curr_imag;
        n = (slots as core::ffi::c_int - 2 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as WORD;
        while n != 0 {
            curr_real = *real1;
            real1 = real1.offset(128 as core::ffi::c_int as isize);
            curr_imag = *imag1;
            imag1 = imag1.offset(128 as core::ffi::c_int as isize);
            curr_real = ixheaac_shr32(curr_real, 3 as WORD);
            curr_imag = ixheaac_shr32(curr_imag, 3 as WORD);
            t_phi_01 = ixheaacd_mac32x16hin32(t_phi_01, curr_real, prev_real);
            t_phi_01 = ixheaacd_mac32x16hin32(t_phi_01, curr_imag, prev_imag);
            t_phi_01_i = ixheaacd_mac32x16hin32(t_phi_01_i, curr_imag, prev_real);
            t_phi_01_i = ixheaacd_macn32x16hin32(t_phi_01_i, curr_real, prev_imag);
            t_phi_11 = ixheaacd_mac32x16hin32(t_phi_11, prev_real, prev_real);
            t_phi_11 = ixheaacd_mac32x16hin32(t_phi_11, prev_imag, prev_imag);
            prev_real = *real1;
            real1 = real1.offset(128 as core::ffi::c_int as isize);
            prev_imag = *imag1;
            imag1 = imag1.offset(128 as core::ffi::c_int as isize);
            prev_real = ixheaac_shr32(prev_real, 3 as WORD);
            prev_imag = ixheaac_shr32(prev_imag, 3 as WORD);
            t_phi_01 = ixheaacd_mac32x16hin32(t_phi_01, prev_real, curr_real);
            t_phi_01 = ixheaacd_mac32x16hin32(t_phi_01, prev_imag, curr_imag);
            t_phi_01_i = ixheaacd_mac32x16hin32(t_phi_01_i, prev_imag, curr_real);
            t_phi_01_i = ixheaacd_macn32x16hin32(t_phi_01_i, prev_real, curr_imag);
            t_phi_11 = ixheaacd_mac32x16hin32(t_phi_11, curr_real, curr_real);
            t_phi_11 = ixheaacd_mac32x16hin32(t_phi_11, curr_imag, curr_imag);
            n -= 1;
        }
        if slots as core::ffi::c_int & 0x1 as core::ffi::c_int != 0 {
            curr_real = *real1;
            curr_imag = *imag1;
            curr_real = ixheaac_shr32(curr_real, 3 as WORD);
            curr_imag = ixheaac_shr32(curr_imag, 3 as WORD);
            t_phi_01 = ixheaacd_mac32x16hin32(t_phi_01, curr_real, prev_real);
            t_phi_01 = ixheaacd_mac32x16hin32(t_phi_01, curr_imag, prev_imag);
            t_phi_01_i = ixheaacd_mac32x16hin32(t_phi_01_i, curr_imag, prev_real);
            t_phi_01_i = ixheaacd_macn32x16hin32(t_phi_01_i, curr_real, prev_imag);
            t_phi_11 = ixheaacd_mac32x16hin32(t_phi_11, prev_real, prev_real);
            t_phi_11 = ixheaacd_mac32x16hin32(t_phi_11, prev_imag, prev_imag);
        }
        let mut t_phi_22: WORD32 = t_phi_11;
        let mut curr_real_0: WORD32 = *real_buffer
            .offset((-(2 as core::ffi::c_int) * 128 as core::ffi::c_int) as isize);
        let mut curr_imag_0: WORD32 = *img_buffer
            .offset((-(2 as core::ffi::c_int) * 128 as core::ffi::c_int) as isize);
        curr_real_0 = ixheaac_shr32(curr_real_0, 3 as WORD);
        curr_imag_0 = ixheaac_shr32(curr_imag_0, 3 as WORD);
        t_phi_22 = ixheaacd_mac32x16hin32(t_phi_22, curr_real_0, curr_real_0);
        t_phi_22 = ixheaacd_mac32x16hin32(t_phi_22, curr_imag_0, curr_imag_0);
        curr_real_0 = *real_buffer
            .offset(
                ((slots as core::ffi::c_int - 2 as core::ffi::c_int)
                    * 128 as core::ffi::c_int) as isize,
            );
        curr_imag_0 = *img_buffer
            .offset(
                ((slots as core::ffi::c_int - 2 as core::ffi::c_int)
                    * 128 as core::ffi::c_int) as isize,
            );
        curr_real_0 = ixheaac_shr32(curr_real_0, 3 as WORD);
        curr_imag_0 = ixheaac_shr32(curr_imag_0, 3 as WORD);
        t_phi_11 = ixheaacd_mac32x16hin32(t_phi_11, curr_real_0, curr_real_0);
        t_phi_11 = ixheaacd_mac32x16hin32(t_phi_11, curr_imag_0, curr_imag_0);
        (*cov_matrix).phi_11 = t_phi_11;
        (*cov_matrix).phi_22 = t_phi_22;
        let mut t_phi_12: WORD32 = t_phi_01;
        t_phi_12 = ixheaacd_mac32x16hin32(
            t_phi_12,
            *real_buffer.offset(-(128 as core::ffi::c_int) as isize)
                >> 3 as core::ffi::c_int,
            *real_buffer
                .offset((-(2 as core::ffi::c_int) * 128 as core::ffi::c_int) as isize)
                >> 3 as core::ffi::c_int,
        );
        t_phi_12 = ixheaacd_mac32x16hin32(
            t_phi_12,
            *img_buffer.offset(-(128 as core::ffi::c_int) as isize)
                >> 3 as core::ffi::c_int,
            *img_buffer
                .offset((-(2 as core::ffi::c_int) * 128 as core::ffi::c_int) as isize)
                >> 3 as core::ffi::c_int,
        );
        t_phi_01 = ixheaacd_mac32x16hin32(
            t_phi_01,
            *real_buffer
                .offset(
                    ((slots as core::ffi::c_int - 1 as core::ffi::c_int)
                        * 128 as core::ffi::c_int) as isize,
                ) >> 3 as core::ffi::c_int,
            *real_buffer
                .offset(
                    ((slots as core::ffi::c_int - 2 as core::ffi::c_int)
                        * 128 as core::ffi::c_int) as isize,
                ) >> 3 as core::ffi::c_int,
        );
        t_phi_01 = ixheaacd_mac32x16hin32(
            t_phi_01,
            *img_buffer
                .offset(
                    ((slots as core::ffi::c_int - 1 as core::ffi::c_int)
                        * 128 as core::ffi::c_int) as isize,
                ) >> 3 as core::ffi::c_int,
            *img_buffer
                .offset(
                    ((slots as core::ffi::c_int - 2 as core::ffi::c_int)
                        * 128 as core::ffi::c_int) as isize,
                ) >> 3 as core::ffi::c_int,
        );
        (*cov_matrix).phi_01 = t_phi_01;
        (*cov_matrix).phi_12 = t_phi_12;
        let mut t_phi_12_i: WORD32 = t_phi_01_i;
        t_phi_12_i = ixheaacd_mac32x16hin32(
            t_phi_12_i,
            *img_buffer.offset(-(128 as core::ffi::c_int) as isize)
                >> 3 as core::ffi::c_int,
            *real_buffer
                .offset((-(2 as core::ffi::c_int) * 128 as core::ffi::c_int) as isize)
                >> 3 as core::ffi::c_int,
        );
        t_phi_12_i = ixheaacd_macn32x16hin32(
            t_phi_12_i,
            *real_buffer.offset(-(128 as core::ffi::c_int) as isize)
                >> 3 as core::ffi::c_int,
            *img_buffer
                .offset((-(2 as core::ffi::c_int) * 128 as core::ffi::c_int) as isize)
                >> 3 as core::ffi::c_int,
        );
        t_phi_01_i = ixheaacd_mac32x16hin32(
            t_phi_01_i,
            *img_buffer
                .offset(
                    ((slots as core::ffi::c_int - 1 as core::ffi::c_int)
                        * 128 as core::ffi::c_int) as isize,
                ) >> 3 as core::ffi::c_int,
            *real_buffer
                .offset(
                    ((slots as core::ffi::c_int - 2 as core::ffi::c_int)
                        * 128 as core::ffi::c_int) as isize,
                ) >> 3 as core::ffi::c_int,
        );
        t_phi_01_i = ixheaacd_macn32x16hin32(
            t_phi_01_i,
            *real_buffer
                .offset(
                    ((slots as core::ffi::c_int - 1 as core::ffi::c_int)
                        * 128 as core::ffi::c_int) as isize,
                ) >> 3 as core::ffi::c_int,
            *img_buffer
                .offset(
                    ((slots as core::ffi::c_int - 2 as core::ffi::c_int)
                        * 128 as core::ffi::c_int) as isize,
                ) >> 3 as core::ffi::c_int,
        );
        (*cov_matrix).phi_01_im = t_phi_01_i;
        (*cov_matrix).phi_12_im = t_phi_12_i;
        let mut n_0: WORD16 = 0;
        let mut len_by_4: WORD16 = 0;
        let mut p: WORD16 = 0;
        let mut t_phi_02: WORD32 = 0 as WORD32;
        let mut t_phi_02_i: WORD32 = 0 as WORD32;
        len_by_4 = (slots as core::ffi::c_int >> 2 as core::ffi::c_int) as WORD16;
        p = 0 as WORD16;
        n_0 = 0 as WORD16;
        while (n_0 as core::ffi::c_int) < len_by_4 as core::ffi::c_int {
            let mut real1_0: WORD32 = 0;
            let mut real2: WORD32 = 0;
            let mut imag1_0: WORD32 = 0;
            let mut imag2: WORD32 = 0;
            real1_0 = *real_buffer
                .offset((p as core::ffi::c_int * 128 as core::ffi::c_int) as isize);
            real2 = *real_buffer
                .offset(
                    ((p as core::ffi::c_int - 2 as core::ffi::c_int)
                        * 128 as core::ffi::c_int) as isize,
                );
            imag1_0 = *img_buffer
                .offset((p as core::ffi::c_int * 128 as core::ffi::c_int) as isize);
            imag2 = *img_buffer
                .offset(
                    ((p as core::ffi::c_int - 2 as core::ffi::c_int)
                        * 128 as core::ffi::c_int) as isize,
                );
            real1_0 = ixheaac_shr32(real1_0, 3 as WORD);
            real2 = ixheaac_shr32(real2, 3 as WORD);
            imag1_0 = ixheaac_shr32(imag1_0, 3 as WORD);
            imag2 = ixheaac_shr32(imag2, 3 as WORD);
            t_phi_02 = ixheaacd_mac32x16hin32(t_phi_02, real1_0, real2);
            t_phi_02 = ixheaacd_mac32x16hin32(t_phi_02, imag1_0, imag2);
            t_phi_02_i = ixheaacd_mac32x16hin32(t_phi_02_i, imag1_0, real2);
            t_phi_02_i = ixheaacd_macn32x16hin32(t_phi_02_i, real1_0, imag2);
            real1_0 = *real_buffer
                .offset(
                    ((p as core::ffi::c_int + 1 as core::ffi::c_int)
                        * 128 as core::ffi::c_int) as isize,
                );
            real2 = *real_buffer
                .offset(
                    ((p as core::ffi::c_int - 1 as core::ffi::c_int)
                        * 128 as core::ffi::c_int) as isize,
                );
            imag1_0 = *img_buffer
                .offset(
                    ((p as core::ffi::c_int + 1 as core::ffi::c_int)
                        * 128 as core::ffi::c_int) as isize,
                );
            imag2 = *img_buffer
                .offset(
                    ((p as core::ffi::c_int - 1 as core::ffi::c_int)
                        * 128 as core::ffi::c_int) as isize,
                );
            real1_0 = ixheaac_shr32(real1_0, 3 as WORD);
            real2 = ixheaac_shr32(real2, 3 as WORD);
            imag1_0 = ixheaac_shr32(imag1_0, 3 as WORD);
            imag2 = ixheaac_shr32(imag2, 3 as WORD);
            t_phi_02 = ixheaacd_mac32x16hin32(t_phi_02, real1_0, real2);
            t_phi_02 = ixheaacd_mac32x16hin32(t_phi_02, imag1_0, imag2);
            t_phi_02_i = ixheaacd_mac32x16hin32(t_phi_02_i, imag1_0, real2);
            t_phi_02_i = ixheaacd_macn32x16hin32(t_phi_02_i, real1_0, imag2);
            real1_0 = *real_buffer
                .offset(
                    ((p as core::ffi::c_int + 2 as core::ffi::c_int)
                        * 128 as core::ffi::c_int) as isize,
                );
            real2 = *real_buffer
                .offset((p as core::ffi::c_int * 128 as core::ffi::c_int) as isize);
            imag1_0 = *img_buffer
                .offset(
                    ((p as core::ffi::c_int + 2 as core::ffi::c_int)
                        * 128 as core::ffi::c_int) as isize,
                );
            imag2 = *img_buffer
                .offset((p as core::ffi::c_int * 128 as core::ffi::c_int) as isize);
            real1_0 = ixheaac_shr32(real1_0, 3 as WORD);
            real2 = ixheaac_shr32(real2, 3 as WORD);
            imag1_0 = ixheaac_shr32(imag1_0, 3 as WORD);
            imag2 = ixheaac_shr32(imag2, 3 as WORD);
            t_phi_02 = ixheaacd_mac32x16hin32(t_phi_02, real1_0, real2);
            t_phi_02 = ixheaacd_mac32x16hin32(t_phi_02, imag1_0, imag2);
            t_phi_02_i = ixheaacd_mac32x16hin32(t_phi_02_i, imag1_0, real2);
            t_phi_02_i = ixheaacd_macn32x16hin32(t_phi_02_i, real1_0, imag2);
            real1_0 = *real_buffer
                .offset(
                    ((p as core::ffi::c_int + 3 as core::ffi::c_int)
                        * 128 as core::ffi::c_int) as isize,
                );
            real2 = *real_buffer
                .offset(
                    ((p as core::ffi::c_int + 1 as core::ffi::c_int)
                        * 128 as core::ffi::c_int) as isize,
                );
            imag1_0 = *img_buffer
                .offset(
                    ((p as core::ffi::c_int + 3 as core::ffi::c_int)
                        * 128 as core::ffi::c_int) as isize,
                );
            imag2 = *img_buffer
                .offset(
                    ((p as core::ffi::c_int + 1 as core::ffi::c_int)
                        * 128 as core::ffi::c_int) as isize,
                );
            real1_0 = ixheaac_shr32(real1_0, 3 as WORD);
            real2 = ixheaac_shr32(real2, 3 as WORD);
            imag1_0 = ixheaac_shr32(imag1_0, 3 as WORD);
            imag2 = ixheaac_shr32(imag2, 3 as WORD);
            t_phi_02 = ixheaacd_mac32x16hin32(t_phi_02, real1_0, real2);
            t_phi_02 = ixheaacd_mac32x16hin32(t_phi_02, imag1_0, imag2);
            t_phi_02_i = ixheaacd_mac32x16hin32(t_phi_02_i, imag1_0, real2);
            t_phi_02_i = ixheaacd_macn32x16hin32(t_phi_02_i, real1_0, imag2);
            p = (p as core::ffi::c_int + 4 as core::ffi::c_int) as WORD16;
            n_0 += 1;
        }
        n_0 = ixheaac_shl16(len_by_4, 2 as WORD16);
        while (n_0 as core::ffi::c_int) < slots as core::ffi::c_int {
            let mut real1_1: WORD32 = 0;
            let mut real2_0: WORD32 = 0;
            let mut imag1_1: WORD32 = 0;
            let mut imag2_0: WORD32 = 0;
            real1_1 = *real_buffer
                .offset((n_0 as core::ffi::c_int * 128 as core::ffi::c_int) as isize);
            real2_0 = *real_buffer
                .offset(
                    ((n_0 as core::ffi::c_int - 2 as core::ffi::c_int)
                        * 128 as core::ffi::c_int) as isize,
                );
            imag1_1 = *img_buffer
                .offset((n_0 as core::ffi::c_int * 128 as core::ffi::c_int) as isize);
            imag2_0 = *img_buffer
                .offset(
                    ((n_0 as core::ffi::c_int - 2 as core::ffi::c_int)
                        * 128 as core::ffi::c_int) as isize,
                );
            real1_1 = ixheaac_shr32(real1_1, 3 as WORD);
            real2_0 = ixheaac_shr32(real2_0, 3 as WORD);
            imag1_1 = ixheaac_shr32(imag1_1, 3 as WORD);
            imag2_0 = ixheaac_shr32(imag2_0, 3 as WORD);
            t_phi_02 = ixheaacd_mac32x16hin32(t_phi_02, real1_1, real2_0);
            t_phi_02 = ixheaacd_mac32x16hin32(t_phi_02, imag1_1, imag2_0);
            t_phi_02_i = ixheaacd_mac32x16hin32(t_phi_02_i, imag1_1, real2_0);
            t_phi_02_i = ixheaacd_macn32x16hin32(t_phi_02_i, real1_1, imag2_0);
            n_0 += 1;
        }
        (*cov_matrix).phi_02 = t_phi_02;
        (*cov_matrix).phi_02_im = t_phi_02_i;
        ptr_real = ptr_real.offset(1);
        pac_arr = pac_arr.offset(1);
        k += 1;
    }
}
#[inline]
unsafe extern "C" fn ixheaacd_filt_step3_lp(
    mut len: WORD,
    mut coef1: WORD32,
    mut coef2: WORD32,
    mut pqmf_real_low: *mut WORD32,
    mut pqmf_real_high: *mut WORD32,
) -> VOID {
    let mut prev1: WORD32 = 0;
    let mut prev2: WORD32 = 0;
    let mut i: WORD32 = 0;
    prev2 = *pqmf_real_low;
    pqmf_real_low = pqmf_real_low.offset(64 as core::ffi::c_int as isize);
    prev1 = *pqmf_real_low;
    pqmf_real_low = pqmf_real_low.offset(64 as core::ffi::c_int as isize);
    i = len as WORD32;
    while i >= 0 as core::ffi::c_int {
        let mut curr: WORD32 = *pqmf_real_low;
        let mut temp: WORD32 = ixheaac_mult32x16hin32(prev2, coef2);
        pqmf_real_low = pqmf_real_low.offset(64 as core::ffi::c_int as isize);
        *pqmf_real_high = ixheaac_add32_sat(
            curr >> LPC_SCALE_FACTOR,
            ixheaacd_mac32x16hin32(temp, prev1, coef1) << 1 as core::ffi::c_int,
        );
        pqmf_real_high = pqmf_real_high.offset(64 as core::ffi::c_int as isize);
        prev2 = *pqmf_real_low;
        temp = ixheaac_mult32x16hin32(prev1, coef2);
        pqmf_real_low = pqmf_real_low.offset(64 as core::ffi::c_int as isize);
        *pqmf_real_high = ixheaac_add32_sat(
            prev2 >> LPC_SCALE_FACTOR,
            ixheaacd_mac32x16hin32(temp, curr, coef1) << 1 as core::ffi::c_int,
        );
        pqmf_real_high = pqmf_real_high.offset(64 as core::ffi::c_int as isize);
        prev1 = prev2;
        prev2 = curr;
        i -= 2 as core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_filter1_lp(
    mut hf_generator: *mut ia_sbr_hf_generator_struct,
    mut cov_matrix_seq: *mut ia_lpp_trans_cov_matrix,
    mut bw_array: *mut WORD32,
    mut degree_alias: *mut WORD16,
    mut start_idx: WORD32,
    mut stop_idx: WORD32,
    mut max_qmf_subband: WORD32,
    mut start_patch: WORD32,
    mut stop_patch: WORD32,
    mut sub_sig_x: *mut WORD32,
) -> VOID {
    let mut k1: WORD16 = 0;
    let mut k1_below: WORD16 = 0 as WORD16;
    let mut k1_below2: WORD16 = 0 as WORD16;
    let mut i: WORD32 = 0;
    let mut alpha_real: [WORD16; 2] = [0; 2];
    let mut low_band: WORD32 = 0;
    let mut high_band: WORD32 = 0;
    let mut patch: WORD32 = 0;
    let mut bw: WORD16 = 0 as WORD16;
    let mut a0r: WORD32 = 0;
    let mut a1r: WORD32 = 0;
    let mut num_patches: WORD = (*(*hf_generator).pstr_settings).num_patches as WORD;
    let mut patch_param: *mut ia_patch_param_struct = ((*(*hf_generator).pstr_settings)
        .str_patch_param)
        .as_mut_ptr();
    let mut bw_index: [WORD32; 6] = [0; 6];
    memset(
        bw_index.as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<WORD32>() as size_t).wrapping_mul(num_patches as size_t),
    );
    low_band = start_patch;
    while low_band < stop_patch {
        let mut p_cov_matrix: *mut ia_lpp_trans_cov_matrix = &mut *cov_matrix_seq
            .offset(low_band as isize) as *mut ia_lpp_trans_cov_matrix;
        alpha_real[1 as core::ffi::c_int as usize] = 0 as WORD16;
        alpha_real[0 as core::ffi::c_int as usize] = 0 as WORD16;
        if (*p_cov_matrix).d != 0 as core::ffi::c_int {
            let mut tmp_r: WORD32 = 0;
            let mut temp_real: WORD32 = 0;
            let mut modulus_d: WORD32 = 0;
            let mut inverse_d: WORD16 = 0;
            let mut norm_d: WORD32 = 0;
            norm_d = ixheaac_norm32((*p_cov_matrix).d) as WORD32;
            inverse_d = (Some(ixheaacd_fix_div.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(0x40000000 as WORD32, (*p_cov_matrix).d << norm_d) as WORD16;
            modulus_d = ixheaac_abs32((*p_cov_matrix).d);
            tmp_r = ixheaac_sub32_sat(
                ixheaac_mult32((*p_cov_matrix).phi_01, (*p_cov_matrix).phi_12),
                ixheaac_mult32((*p_cov_matrix).phi_02, (*p_cov_matrix).phi_11),
            ) >> LPC_SCALE_FACTOR;
            temp_real = ixheaac_abs32(tmp_r);
            if temp_real < modulus_d {
                alpha_real[1 as core::ffi::c_int as usize] = (ixheaac_mult32x16in32_shl_sat(
                    tmp_r,
                    inverse_d,
                ) << norm_d >> 15 as core::ffi::c_int) as WORD16;
            }
            tmp_r = ixheaac_sub32_sat(
                ixheaac_mult32((*p_cov_matrix).phi_02, (*p_cov_matrix).phi_12),
                ixheaac_mult32((*p_cov_matrix).phi_01, (*p_cov_matrix).phi_22),
            ) >> LPC_SCALE_FACTOR;
            temp_real = ixheaac_abs32(tmp_r);
            if temp_real < modulus_d {
                alpha_real[0 as core::ffi::c_int as usize] = (ixheaac_mult32x16in32_shl_sat(
                    tmp_r,
                    inverse_d,
                ) << norm_d >> 15 as core::ffi::c_int) as WORD16;
            }
        }
        if (*p_cov_matrix).phi_11 == 0 as core::ffi::c_int {
            k1 = 0 as WORD16;
        } else if ixheaac_abs32_sat((*p_cov_matrix).phi_01) >= (*p_cov_matrix).phi_11 {
            if (*p_cov_matrix).phi_01 < 0 as core::ffi::c_int {
                k1 = 0x7fff as WORD16;
            } else {
                k1 = -(0x8000 as core::ffi::c_int) as WORD16;
            }
        } else {
            k1 = -((Some(ixheaacd_fix_div.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )((*p_cov_matrix).phi_01, (*p_cov_matrix).phi_11) as WORD16
                as core::ffi::c_int) as WORD16;
        }
        if low_band > 1 as core::ffi::c_int {
            let mut deg: WORD16 = ixheaac_sub16_sat(
                0x7fff as WORD16,
                ixheaac_mult16_shl_sat(k1_below, k1_below),
            );
            *degree_alias.offset(low_band as isize) = 0 as WORD16;
            if low_band as core::ffi::c_int & 1 as core::ffi::c_int
                == 0 as core::ffi::c_int
                && (k1 as core::ffi::c_int) < 0 as core::ffi::c_int
            {
                if (k1_below as core::ffi::c_int) < 0 as core::ffi::c_int {
                    *degree_alias.offset(low_band as isize) = 0x7fff as WORD16;
                    if k1_below2 as core::ffi::c_int > 0 as core::ffi::c_int {
                        *degree_alias
                            .offset(
                                (low_band as core::ffi::c_int - 1 as core::ffi::c_int)
                                    as isize,
                            ) = deg;
                    }
                } else if k1_below2 as core::ffi::c_int > 0 as core::ffi::c_int {
                    *degree_alias.offset(low_band as isize) = deg;
                }
            }
            if low_band as core::ffi::c_int & 1 as core::ffi::c_int
                != 0 as core::ffi::c_int
                && k1 as core::ffi::c_int > 0 as core::ffi::c_int
            {
                if k1_below as core::ffi::c_int > 0 as core::ffi::c_int {
                    *degree_alias.offset(low_band as isize) = 0x7fff as WORD16;
                    if (k1_below2 as core::ffi::c_int) < 0 as core::ffi::c_int {
                        *degree_alias
                            .offset(
                                (low_band as core::ffi::c_int - 1 as core::ffi::c_int)
                                    as isize,
                            ) = deg;
                    }
                } else if (k1_below2 as core::ffi::c_int) < 0 as core::ffi::c_int {
                    *degree_alias.offset(low_band as isize) = deg;
                }
            }
        }
        k1_below2 = k1_below;
        k1_below = k1;
        patch = 0 as core::ffi::c_int as WORD32;
        while patch < num_patches {
            let mut p_loc_patch_param: *mut ia_patch_param_struct = &mut *patch_param
                .offset(patch as isize) as *mut ia_patch_param_struct;
            let mut bw_vec: WORD32 = 0;
            let mut bw_idx: WORD32 = 0;
            let mut alpha1: WORD16 = 0;
            let mut alpha2: WORD16 = 0;
            high_band = ((low_band as core::ffi::c_int
                + (*p_loc_patch_param).dst_end_band as core::ffi::c_int)
                << 8 as core::ffi::c_int >> 8 as core::ffi::c_int) as WORD32;
            if low_band < (*p_loc_patch_param).src_start_band as core::ffi::c_int
                || low_band >= (*p_loc_patch_param).src_end_band as core::ffi::c_int
                || high_band < max_qmf_subband
            {
                patch += 1;
            } else {
                bw_idx = bw_index[patch as usize];
                while high_band
                    >= (*(*hf_generator).pstr_settings).bw_borders[bw_idx as usize]
                        as core::ffi::c_int
                {
                    bw_idx += 1;
                    bw_index[patch as usize] = bw_idx;
                }
                bw_vec = *bw_array.offset(bw_idx as isize);
                alpha1 = alpha_real[0 as core::ffi::c_int as usize];
                alpha2 = alpha_real[1 as core::ffi::c_int as usize];
                bw = ixheaac_extract16h(bw_vec);
                a0r = ixheaac_mult16x16in32_shl(bw, alpha1);
                bw = ixheaac_mult16_shl_sat(bw, bw);
                a1r = ixheaac_mult16x16in32_shl(bw, alpha2);
                let mut p_sub_signal_xlow: *mut WORD32 = sub_sig_x
                    .offset(low_band as isize)
                    .offset((start_idx << 6 as core::ffi::c_int) as isize);
                let mut p_sub_signal_xhigh: *mut WORD32 = sub_sig_x
                    .offset(high_band as isize)
                    .offset(
                        ((start_idx as core::ffi::c_int + 2 as core::ffi::c_int)
                            << 6 as core::ffi::c_int) as isize,
                    );
                let mut len: WORD32 = stop_idx - start_idx - 1 as WORD32;
                if bw as core::ffi::c_int > 0 as core::ffi::c_int {
                    ixheaacd_filt_step3_lp(
                        len as WORD,
                        a0r,
                        a1r,
                        p_sub_signal_xlow,
                        p_sub_signal_xhigh,
                    );
                } else {
                    p_sub_signal_xlow = p_sub_signal_xlow
                        .offset(128 as core::ffi::c_int as isize);
                    i = len;
                    while i >= 0 as core::ffi::c_int {
                        *p_sub_signal_xhigh = *p_sub_signal_xlow >> LPC_SCALE_FACTOR;
                        p_sub_signal_xlow = p_sub_signal_xlow
                            .offset(64 as core::ffi::c_int as isize);
                        p_sub_signal_xhigh = p_sub_signal_xhigh
                            .offset(64 as core::ffi::c_int as isize);
                        i -= 1;
                    }
                }
                patch += 1;
            }
        }
        low_band += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_clr_subsamples(
    mut ptr_qmf_buf: *mut WORD32,
    mut num: WORD32,
    mut size: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    i = num;
    while i >= 0 as core::ffi::c_int {
        memset(
            ptr_qmf_buf as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<WORD32>() as size_t).wrapping_mul(size as size_t),
        );
        ptr_qmf_buf = ptr_qmf_buf.offset(64 as core::ffi::c_int as isize);
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_low_pow_hf_generator(
    mut hf_generator: *mut ia_sbr_hf_generator_struct,
    mut qmf_real: *mut *mut WORD32,
    mut degree_alias: *mut WORD16,
    mut start_idx: WORD32,
    mut stop_idx: WORD32,
    mut num_if_bands: WORD32,
    mut max_qmf_subband: WORD32,
    mut sbr_invf_mode: *mut WORD32,
    mut sbr_invf_mode_prev: *mut WORD32,
    mut norm_max: WORD32,
    mut sub_sig_x: *mut WORD32,
) -> VOID {
    let mut bw_array: [WORD32; 6] = [0; 6];
    let mut i: WORD32 = 0;
    let mut start_patch: WORD32 = 0;
    let mut stop_patch: WORD32 = 0;
    let mut low_band: WORD32 = 0;
    let mut high_band: WORD32 = 0;
    let mut patch_param: *mut ia_patch_param_struct = ((*(*hf_generator).pstr_settings)
        .str_patch_param)
        .as_mut_ptr();
    let mut patch: WORD32 = 0;
    let mut cov_matrix_seq: [ia_lpp_trans_cov_matrix; 32] = [ia_lpp_trans_cov_matrix {
        phi_11: 0,
        phi_22: 0,
        phi_01: 0,
        phi_02: 0,
        phi_12: 0,
        phi_01_im: 0,
        phi_02_im: 0,
        phi_12_im: 0,
        d: 0,
    }; 32];
    let mut actual_stop_band: WORD32 = 0;
    let mut num_patches: WORD32 = (*(*hf_generator).pstr_settings).num_patches as WORD32;
    let mut auto_corr_length: WORD32 = (*(*hf_generator).pstr_settings).num_columns
        as WORD32 + 6 as WORD32;
    stop_idx = (*(*hf_generator).pstr_settings).num_columns as WORD32 + stop_idx;
    ixheaacd_invfilt_level_emphasis(
        hf_generator,
        num_if_bands,
        sbr_invf_mode,
        sbr_invf_mode_prev,
        bw_array.as_mut_ptr(),
    );
    actual_stop_band = ixheaac_add16(
        (*patch_param
            .offset((num_patches as core::ffi::c_int - 1 as core::ffi::c_int) as isize))
            .dst_start_band,
        (*patch_param
            .offset((num_patches as core::ffi::c_int - 1 as core::ffi::c_int) as isize))
            .num_bands_in_patch,
    ) as WORD32;
    let mut p_qmf_real: *mut WORD32 = 0 as *mut WORD32;
    let mut len: WORD32 = 6 as WORD32;
    let mut num: WORD32 = 0;
    if len > stop_idx {
        len = stop_idx;
    }
    p_qmf_real = &mut *(*qmf_real.offset(start_idx as isize))
        .offset(actual_stop_band as isize) as *mut WORD32;
    num = (len as core::ffi::c_int - start_idx as core::ffi::c_int
        - 1 as core::ffi::c_int) as WORD32;
    ixheaacd_clr_subsamples(p_qmf_real, num, NO_SYNTHESIS_CHANNELS - actual_stop_band);
    if actual_stop_band < 32 as core::ffi::c_int {
        num = (stop_idx as core::ffi::c_int - len as core::ffi::c_int
            - 1 as core::ffi::c_int) as WORD32;
        p_qmf_real = &mut *(*qmf_real.offset(len as isize))
            .offset(actual_stop_band as isize) as *mut WORD32;
        ixheaacd_clr_subsamples(
            p_qmf_real,
            num,
            NO_ANALYSIS_CHANNELS - actual_stop_band,
        );
    }
    start_patch = ixheaac_max16(
        1 as WORD16,
        ixheaac_sub16((*(*hf_generator).pstr_settings).start_patch, 2 as WORD16),
    ) as WORD32;
    stop_patch = (*patch_param.offset(0 as core::ffi::c_int as isize)).dst_start_band
        as WORD32;
    let mut ptr: *mut WORD32 = &mut *sub_sig_x.offset(0 as core::ffi::c_int as isize)
        as *mut WORD32;
    let mut plpc_filt_states_real: *mut WORD32 = &mut *(*((*hf_generator)
        .lpc_filt_states_real)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize))
        .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
    i = LPC_ORDER as WORD32;
    while i != 0 as core::ffi::c_int {
        memcpy(
            ptr as *mut core::ffi::c_void,
            plpc_filt_states_real as *const core::ffi::c_void,
            (::core::mem::size_of::<WORD32>() as size_t)
                .wrapping_mul(stop_patch as size_t),
        );
        ptr = ptr.offset(NO_SYNTHESIS_CHANNELS as isize);
        plpc_filt_states_real = plpc_filt_states_real
            .offset(32 as core::ffi::c_int as isize);
        i -= 1;
    }
    if norm_max != 30 as core::ffi::c_int {
        if 30 as core::ffi::c_int
            == (*(*hf_generator).pstr_settings).num_columns as core::ffi::c_int
        {
            (Some(
                ixheaacd_covariance_matrix_calc_960.expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                sub_sig_x.offset(start_patch as isize),
                &mut *cov_matrix_seq.as_mut_ptr().offset(start_patch as isize),
                stop_patch - start_patch,
                auto_corr_length,
            );
        } else {
            (Some(ixheaacd_covariance_matrix_calc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                sub_sig_x.offset(start_patch as isize),
                &mut *cov_matrix_seq.as_mut_ptr().offset(start_patch as isize),
                stop_patch - start_patch,
                auto_corr_length,
            );
        }
    } else {
        memset(
            &mut *cov_matrix_seq.as_mut_ptr().offset(0 as core::ffi::c_int as isize)
                as *mut ia_lpp_trans_cov_matrix as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<ia_lpp_trans_cov_matrix>() as size_t)
                .wrapping_mul(stop_patch as size_t),
        );
    }
    ixheaacd_filter1_lp(
        hf_generator,
        cov_matrix_seq.as_mut_ptr(),
        bw_array.as_mut_ptr(),
        degree_alias,
        start_idx,
        stop_idx,
        max_qmf_subband,
        start_patch,
        stop_patch,
        sub_sig_x,
    );
    start_patch = (*(*hf_generator).pstr_settings).start_patch as WORD32;
    stop_patch = (*(*hf_generator).pstr_settings).stop_patch as WORD32;
    low_band = start_patch;
    while low_band < stop_patch {
        let mut src_start_band: WORD32 = 0;
        let mut src_end_band: WORD32 = 0;
        let mut dst_start_band: WORD32 = 0;
        patch = 0 as core::ffi::c_int as WORD32;
        while patch < num_patches {
            let mut ptr_loc_patch_param: *mut ia_patch_param_struct = &mut *patch_param
                .offset(patch as isize) as *mut ia_patch_param_struct;
            src_start_band = (*ptr_loc_patch_param).src_start_band as WORD32;
            src_end_band = (*ptr_loc_patch_param).src_end_band as WORD32;
            dst_start_band = (*ptr_loc_patch_param).dst_start_band as WORD32;
            high_band = (low_band as core::ffi::c_int
                + (*ptr_loc_patch_param).dst_end_band as core::ffi::c_int) as WORD32;
            if low_band < src_start_band || low_band >= src_end_band
                || high_band >= NO_SYNTHESIS_CHANNELS
            {
                patch += 1;
            } else {
                if high_band != dst_start_band {
                    *degree_alias.offset(high_band as isize) = *degree_alias
                        .offset(low_band as isize);
                }
                patch += 1;
            }
        }
        low_band += 1;
    }
    memcpy(
        ((*hf_generator).bw_array_prev).as_mut_ptr() as *mut core::ffi::c_void,
        bw_array.as_mut_ptr() as *const core::ffi::c_void,
        (::core::mem::size_of::<WORD32>() as size_t).wrapping_mul(num_if_bands as size_t),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hf_generator(
    mut hf_generator: *mut ia_sbr_hf_generator_struct,
    mut scale_factor: *mut ia_sbr_scale_fact_struct,
    mut qmf_real: *mut *mut WORD32,
    mut qmf_imag: *mut *mut WORD32,
    mut factor: WORD32,
    mut start_idx: WORD32,
    mut stop_idx: WORD32,
    mut num_if_bands: WORD32,
    mut max_qmf_subband: WORD32,
    mut sbr_invf_mode: *mut WORD32,
    mut sbr_invf_mode_prev: *mut WORD32,
    mut sub_sig_x: *mut WORD32,
    mut audio_object_type: WORD,
) -> VOID {
    let mut bw_index: [WORD32; 6] = [0; 6];
    let mut bw_array: [WORD32; 6] = [0; 6];
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut start_patch: WORD32 = 0;
    let mut stop_patch: WORD32 = 0;
    let mut low_band: WORD32 = 0;
    let mut high_band: WORD32 = 0;
    let mut patch_param: *mut ia_patch_param_struct = ((*(*hf_generator).pstr_settings)
        .str_patch_param)
        .as_mut_ptr();
    let mut patch: WORD32 = 0;
    let mut alpha_real: [WORD16; 2] = [0; 2];
    let mut a0r: WORD16 = 0;
    let mut a1r: WORD16 = 0;
    let mut alpha_imag: [WORD16; 2] = [0; 2];
    let mut a0i: WORD16 = 0 as WORD16;
    let mut a1i: WORD16 = 0 as WORD16;
    let mut bw: WORD16 = 0 as WORD16;
    let mut cov_matrix: ia_lpp_trans_cov_matrix = ia_lpp_trans_cov_matrix {
        phi_11: 0,
        phi_22: 0,
        phi_01: 0,
        phi_02: 0,
        phi_12: 0,
        phi_01_im: 0,
        phi_02_im: 0,
        phi_12_im: 0,
        d: 0,
    };
    let mut cov_matrix_seq: [ia_lpp_trans_cov_matrix; 32] = [ia_lpp_trans_cov_matrix {
        phi_11: 0,
        phi_22: 0,
        phi_01: 0,
        phi_02: 0,
        phi_12: 0,
        phi_01_im: 0,
        phi_02_im: 0,
        phi_12_im: 0,
        d: 0,
    }; 32];
    let mut common_scale: WORD32 = 0;
    let mut actual_stop_band: WORD32 = 0;
    let mut num_patches: WORD32 = (*(*hf_generator).pstr_settings).num_patches as WORD32;
    let mut auto_corr_length: WORD32 = (*(*hf_generator).pstr_settings).num_columns
        as WORD32 + 6 as WORD32;
    start_idx = start_idx * factor;
    stop_idx = (*(*hf_generator).pstr_settings).num_columns as WORD32
        + stop_idx * factor;
    ixheaacd_invfilt_level_emphasis(
        hf_generator,
        num_if_bands,
        sbr_invf_mode,
        sbr_invf_mode_prev,
        bw_array.as_mut_ptr(),
    );
    actual_stop_band = ixheaac_add16(
        (*patch_param
            .offset((num_patches as core::ffi::c_int - 1 as core::ffi::c_int) as isize))
            .dst_start_band,
        (*patch_param
            .offset((num_patches as core::ffi::c_int - 1 as core::ffi::c_int) as isize))
            .num_bands_in_patch,
    ) as WORD32;
    i = start_idx;
    while i < stop_idx {
        let mut p_qmf_real: *mut WORD32 = &mut *(*qmf_real.offset(i as isize))
            .offset(actual_stop_band as isize) as *mut WORD32;
        let mut p_qmf_imag: *mut WORD32 = &mut *(*qmf_imag.offset(i as isize))
            .offset(actual_stop_band as isize) as *mut WORD32;
        j = NO_SYNTHESIS_CHANNELS - actual_stop_band;
        while j != 0 as core::ffi::c_int {
            let fresh0 = p_qmf_real;
            p_qmf_real = p_qmf_real.offset(1);
            *fresh0 = 0 as core::ffi::c_int as WORD32;
            let fresh1 = p_qmf_imag;
            p_qmf_imag = p_qmf_imag.offset(1);
            *fresh1 = 0 as core::ffi::c_int as WORD32;
            j -= 1;
        }
        i += 1;
    }
    memset(
        bw_index.as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<WORD32>() as size_t).wrapping_mul(num_patches as size_t),
    );
    common_scale = ixheaac_min32(
        (*scale_factor).ov_lb_scale as WORD32,
        (*scale_factor).lb_scale as WORD32,
    );
    start_patch = (*(*hf_generator).pstr_settings).start_patch as WORD32;
    stop_patch = (*(*hf_generator).pstr_settings).stop_patch as WORD32;
    let mut ptr: *mut WORD32 = 0 as *mut WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < LPC_ORDER {
        ptr = sub_sig_x
            .offset(start_patch as isize)
            .offset((i as core::ffi::c_int * 128 as core::ffi::c_int) as isize);
        memcpy(
            ptr as *mut core::ffi::c_void,
            &mut *(*((*hf_generator).lpc_filt_states_real)
                .as_mut_ptr()
                .offset(i as isize))
                .offset(start_patch as isize) as *mut WORD32 as *const core::ffi::c_void,
            (::core::mem::size_of::<WORD32>() as size_t)
                .wrapping_mul((stop_patch - start_patch) as size_t),
        );
        memcpy(
            ptr.offset(64 as core::ffi::c_int as isize) as *mut core::ffi::c_void,
            &mut *(*((*hf_generator).lpc_filt_states_imag)
                .as_mut_ptr()
                .offset(i as isize))
                .offset(start_patch as isize) as *mut WORD32 as *const core::ffi::c_void,
            (::core::mem::size_of::<WORD32>() as size_t)
                .wrapping_mul((stop_patch - start_patch) as size_t),
        );
        i += 1;
    }
    if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
        && audio_object_type != AOT_ER_AAC_LD as core::ffi::c_int
    {
        if auto_corr_length == 36 as core::ffi::c_int {
            (Some(ixheaacd_covariance_matrix_calc_2.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                &mut *cov_matrix_seq.as_mut_ptr().offset(start_patch as isize),
                sub_sig_x
                    .offset(start_patch as isize)
                    .offset((LPC_ORDER * 128 as core::ffi::c_int) as isize),
                stop_patch - start_patch,
                auto_corr_length as WORD16,
            );
        } else {
            (Some(ixheaacd_covariance_matrix_calc_2.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                &mut *cov_matrix_seq.as_mut_ptr().offset(start_patch as isize),
                sub_sig_x
                    .offset(start_patch as isize)
                    .offset((LPC_ORDER * 128 as core::ffi::c_int) as isize),
                stop_patch - start_patch,
                38 as WORD16,
            );
        }
    } else if (*(*hf_generator).pstr_settings).num_columns as core::ffi::c_int
        == 15 as core::ffi::c_int
    {
        (Some(ixheaacd_covariance_matrix_calc_2.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            &mut *cov_matrix_seq.as_mut_ptr().offset(start_patch as isize),
            sub_sig_x
                .offset(start_patch as isize)
                .offset((LPC_ORDER * 128 as core::ffi::c_int) as isize),
            stop_patch - start_patch,
            (*(*hf_generator).pstr_settings).num_columns,
        );
    } else {
        (Some(ixheaacd_covariance_matrix_calc_2.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            &mut *cov_matrix_seq.as_mut_ptr().offset(start_patch as isize),
            sub_sig_x
                .offset(start_patch as isize)
                .offset((LPC_ORDER * 128 as core::ffi::c_int) as isize),
            stop_patch - start_patch,
            16 as WORD16,
        );
    }
    low_band = start_patch;
    while low_band < stop_patch {
        let mut reset_lpc_coeff: FLAG = 0 as FLAG;
        let mut max_val: WORD32 = 0;
        let mut q_shift: WORD32 = 0;
        let mut v: WORD32 = 0;
        max_val = ixheaac_abs32_nrm(cov_matrix_seq[low_band as usize].phi_01);
        max_val = max_val | ixheaac_abs32_nrm(cov_matrix_seq[low_band as usize].phi_02);
        max_val = max_val | ixheaac_abs32_nrm(cov_matrix_seq[low_band as usize].phi_12);
        max_val = max_val | cov_matrix_seq[low_band as usize].phi_11;
        max_val = max_val | cov_matrix_seq[low_band as usize].phi_22;
        max_val = max_val
            | ixheaac_abs32_nrm(cov_matrix_seq[low_band as usize].phi_01_im);
        max_val = max_val
            | ixheaac_abs32_nrm(cov_matrix_seq[low_band as usize].phi_02_im);
        max_val = max_val
            | ixheaac_abs32_nrm(cov_matrix_seq[low_band as usize].phi_12_im);
        q_shift = ixheaac_pnorm32(max_val) as WORD32;
        cov_matrix.phi_11 = cov_matrix_seq[low_band as usize].phi_11 << q_shift;
        cov_matrix.phi_22 = cov_matrix_seq[low_band as usize].phi_22 << q_shift;
        cov_matrix.phi_01 = cov_matrix_seq[low_band as usize].phi_01 << q_shift;
        cov_matrix.phi_02 = cov_matrix_seq[low_band as usize].phi_02 << q_shift;
        cov_matrix.phi_12 = cov_matrix_seq[low_band as usize].phi_12 << q_shift;
        cov_matrix.phi_01_im = cov_matrix_seq[low_band as usize].phi_01_im << q_shift;
        cov_matrix.phi_02_im = cov_matrix_seq[low_band as usize].phi_02_im << q_shift;
        cov_matrix.phi_12_im = cov_matrix_seq[low_band as usize].phi_12_im << q_shift;
        max_val = ixheaac_mult32(cov_matrix.phi_12, cov_matrix.phi_12);
        max_val = ixheaac_add32_sat(
            max_val,
            ixheaac_mult32(cov_matrix.phi_12_im, cov_matrix.phi_12_im),
        );
        v = ixheaac_sub32_sat(
            ixheaac_mult32(cov_matrix.phi_11, cov_matrix.phi_22),
            max_val,
        ) << 1 as core::ffi::c_int;
        cov_matrix.d = v;
        alpha_real[1 as core::ffi::c_int as usize] = 0 as WORD16;
        alpha_imag[1 as core::ffi::c_int as usize] = 0 as WORD16;
        if cov_matrix.d != 0 as core::ffi::c_int {
            let mut tmp_r: WORD32 = 0;
            let mut temp_real: WORD32 = 0;
            let mut modulus_d: WORD32 = 0;
            let mut tmp_i: WORD32 = 0;
            let mut temp_imag: WORD32 = 0;
            let mut inverse_d: WORD16 = 0;
            let mut norm_d: WORD32 = 0;
            norm_d = ixheaac_norm32(cov_matrix.d) as WORD32;
            inverse_d = (Some(ixheaacd_fix_div.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(0x40000000 as WORD32, cov_matrix.d << norm_d) as WORD16;
            modulus_d = ixheaac_abs32_sat(cov_matrix.d);
            tmp_r = ixheaac_sub32_sat(
                ixheaac_sub32_sat(
                    ixheaac_mult32(cov_matrix.phi_01, cov_matrix.phi_12),
                    ixheaac_mult32(cov_matrix.phi_01_im, cov_matrix.phi_12_im),
                ),
                ixheaac_mult32(cov_matrix.phi_02, cov_matrix.phi_11),
            ) >> LPC_SCALE_FACTOR - 1 as core::ffi::c_int;
            tmp_i = ixheaac_sub32_sat(
                ixheaac_add32_sat(
                    ixheaac_mult32(cov_matrix.phi_01_im, cov_matrix.phi_12),
                    ixheaac_mult32(cov_matrix.phi_01, cov_matrix.phi_12_im),
                ),
                ixheaac_mult32(cov_matrix.phi_02_im, cov_matrix.phi_11),
            ) >> LPC_SCALE_FACTOR - 1 as core::ffi::c_int;
            temp_imag = ixheaac_abs32_sat(tmp_i);
            temp_real = ixheaac_abs32_sat(tmp_r);
            if temp_real >= modulus_d {
                reset_lpc_coeff = 1 as core::ffi::c_int as FLAG;
            } else {
                alpha_real[1 as core::ffi::c_int as usize] = (ixheaac_mult32x16in32(
                    tmp_r,
                    inverse_d,
                ) << norm_d as core::ffi::c_int + 1 as core::ffi::c_int
                    >> 15 as core::ffi::c_int) as WORD16;
            }
            if temp_imag >= modulus_d {
                reset_lpc_coeff = 1 as core::ffi::c_int as FLAG;
            } else {
                alpha_imag[1 as core::ffi::c_int as usize] = (ixheaac_mult32x16in32(
                    tmp_i,
                    inverse_d,
                ) << norm_d as core::ffi::c_int + 1 as core::ffi::c_int
                    >> 15 as core::ffi::c_int) as WORD16;
            }
        }
        alpha_real[0 as core::ffi::c_int as usize] = 0 as WORD16;
        alpha_imag[0 as core::ffi::c_int as usize] = 0 as WORD16;
        if cov_matrix.phi_11 != 0 as core::ffi::c_int {
            let mut tmp_r_0: WORD32 = 0;
            let mut temp_real_0: WORD32 = 0;
            let mut tmp_i_0: WORD32 = 0 as WORD32;
            let mut temp_imag_0: WORD32 = 0 as WORD32;
            let mut inverse_r11: WORD16 = 0;
            let mut norm_r11: WORD32 = 0;
            norm_r11 = ixheaac_norm32(cov_matrix.phi_11) as WORD32;
            inverse_r11 = (Some(ixheaacd_fix_div.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(0x40000000 as WORD32, cov_matrix.phi_11 << norm_r11) as WORD16;
            tmp_r_0 = ixheaac_add32_sat(
                ixheaac_add32(
                    cov_matrix.phi_01 >> LPC_SCALE_FACTOR + 1 as core::ffi::c_int,
                    ixheaac_mult32x16in32(
                        cov_matrix.phi_12,
                        alpha_real[1 as core::ffi::c_int as usize],
                    ),
                ),
                ixheaac_mult32x16in32(
                    cov_matrix.phi_12_im,
                    alpha_imag[1 as core::ffi::c_int as usize],
                ),
            );
            tmp_i_0 = ixheaac_sub32_sat(
                ixheaac_add32(
                    cov_matrix.phi_01_im >> LPC_SCALE_FACTOR + 1 as core::ffi::c_int,
                    ixheaac_mult32x16in32(
                        cov_matrix.phi_12,
                        alpha_imag[1 as core::ffi::c_int as usize],
                    ),
                ),
                ixheaac_mult32x16in32(
                    cov_matrix.phi_12_im,
                    alpha_real[1 as core::ffi::c_int as usize],
                ),
            );
            tmp_r_0 = tmp_r_0 << 1 as core::ffi::c_int;
            tmp_i_0 = tmp_i_0 << 1 as core::ffi::c_int;
            temp_imag_0 = ixheaac_abs32_sat(tmp_i_0);
            temp_real_0 = ixheaac_abs32_sat(tmp_r_0);
            if temp_real_0 >= cov_matrix.phi_11 {
                reset_lpc_coeff = 1 as core::ffi::c_int as FLAG;
            } else {
                alpha_real[0 as core::ffi::c_int as usize] = (ixheaac_mult32x16in32(
                    ixheaac_sub32_sat(0 as WORD32, tmp_r_0),
                    inverse_r11,
                ) << norm_r11 as core::ffi::c_int + 1 as core::ffi::c_int
                    >> 15 as core::ffi::c_int) as WORD16;
            }
            if temp_imag_0 >= cov_matrix.phi_11 {
                reset_lpc_coeff = 1 as core::ffi::c_int as FLAG;
            } else {
                alpha_imag[0 as core::ffi::c_int as usize] = (ixheaac_mult32x16in32(
                    ixheaac_sub32_sat(0 as WORD32, tmp_i_0),
                    inverse_r11,
                ) << norm_r11 as core::ffi::c_int + 1 as core::ffi::c_int
                    >> 15 as core::ffi::c_int) as WORD16;
            }
        }
        if ixheaac_add32_sat(
            alpha_real[0 as core::ffi::c_int as usize] as WORD32
                * alpha_real[0 as core::ffi::c_int as usize] as WORD32,
            alpha_imag[0 as core::ffi::c_int as usize] as WORD32
                * alpha_imag[0 as core::ffi::c_int as usize] as WORD32,
        ) as core::ffi::c_long >= 0x40000000 as core::ffi::c_long
        {
            reset_lpc_coeff = 1 as core::ffi::c_int as FLAG;
        }
        if ixheaac_add32_sat(
            alpha_real[1 as core::ffi::c_int as usize] as WORD32
                * alpha_real[1 as core::ffi::c_int as usize] as WORD32,
            alpha_imag[1 as core::ffi::c_int as usize] as WORD32
                * alpha_imag[1 as core::ffi::c_int as usize] as WORD32,
        ) as core::ffi::c_long >= 0x40000000 as core::ffi::c_long
        {
            reset_lpc_coeff = 1 as core::ffi::c_int as FLAG;
        }
        if reset_lpc_coeff != 0 {
            alpha_real[0 as core::ffi::c_int as usize] = 0 as WORD16;
            alpha_real[1 as core::ffi::c_int as usize] = 0 as WORD16;
            alpha_imag[0 as core::ffi::c_int as usize] = 0 as WORD16;
            alpha_imag[1 as core::ffi::c_int as usize] = 0 as WORD16;
        }
        patch = 0 as core::ffi::c_int as WORD32;
        while patch < num_patches {
            high_band = (low_band as core::ffi::c_int
                + (*patch_param.offset(patch as isize)).dst_end_band as core::ffi::c_int)
                as WORD32;
            if low_band
                < (*patch_param.offset(patch as isize)).src_start_band
                    as core::ffi::c_int
                || low_band
                    >= (*patch_param.offset(patch as isize)).src_end_band
                        as core::ffi::c_int
            {
                patch += 1;
            } else if high_band < max_qmf_subband {
                patch += 1;
            } else {
                while bw_index[patch as usize] < MAX_NUM_PATCHES - 1 as core::ffi::c_int
                    && bw_index[patch as usize] < MAX_NUM_NOISE_VALUES
                    && high_band
                        >= (*(*hf_generator).pstr_settings)
                            .bw_borders[bw_index[patch as usize] as usize]
                            as core::ffi::c_int
                {
                    bw_index[patch as usize] += 1;
                }
                bw = ixheaac_extract16h(bw_array[bw_index[patch as usize] as usize]);
                a0r = ixheaac_mult16_shl_sat(
                    bw,
                    alpha_real[0 as core::ffi::c_int as usize],
                );
                a0i = ixheaac_mult16_shl_sat(
                    bw,
                    alpha_imag[0 as core::ffi::c_int as usize],
                );
                bw = ixheaac_mult16_shl_sat(bw, bw);
                a1r = ixheaac_mult16_shl_sat(
                    bw,
                    alpha_real[1 as core::ffi::c_int as usize],
                );
                a1i = ixheaac_mult16_shl_sat(
                    bw,
                    alpha_imag[1 as core::ffi::c_int as usize],
                );
                if bw as core::ffi::c_int > 0 as core::ffi::c_int {
                    ixheaacd_filterstep3(
                        a0r,
                        a0i,
                        a1r,
                        a1i,
                        start_idx,
                        stop_idx,
                        low_band,
                        high_band,
                        sub_sig_x,
                    );
                } else {
                    let mut p_src: *mut WORD32 = sub_sig_x
                        .offset(low_band as isize)
                        .offset(
                            ((start_idx as core::ffi::c_int + 2 as core::ffi::c_int)
                                << 7 as core::ffi::c_int) as isize,
                        );
                    let mut p_dst: *mut WORD32 = sub_sig_x
                        .offset(high_band as isize)
                        .offset(
                            ((start_idx as core::ffi::c_int + 2 as core::ffi::c_int)
                                << 7 as core::ffi::c_int) as isize,
                        );
                    i = stop_idx - start_idx;
                    while i != 0 as core::ffi::c_int {
                        *p_dst = *p_src >> LPC_SCALE_FACTOR;
                        p_src = p_src.offset(64 as core::ffi::c_int as isize);
                        p_dst = p_dst.offset(64 as core::ffi::c_int as isize);
                        *p_dst = *p_src >> LPC_SCALE_FACTOR;
                        p_src = p_src.offset(64 as core::ffi::c_int as isize);
                        p_dst = p_dst.offset(64 as core::ffi::c_int as isize);
                        i -= 1;
                    }
                }
                patch += 1;
            }
        }
        low_band += 1;
    }
    memcpy(
        ((*hf_generator).bw_array_prev).as_mut_ptr() as *mut core::ffi::c_void,
        bw_array.as_mut_ptr() as *const core::ffi::c_void,
        (::core::mem::size_of::<WORD32>() as size_t).wrapping_mul(num_if_bands as size_t),
    );
    (*scale_factor).hb_scale = (common_scale as core::ffi::c_int - LPC_SCALE_FACTOR)
        as WORD16;
}
