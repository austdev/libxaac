extern "C" {
    fn ixheaacd_pretwdct2(inp: *mut WORD32, out_fwd: *mut WORD32) -> VOID;
    fn ixheaacd_sbr_qmfanal32_winadd_eld(
        inp1: *mut WORD16,
        inp2: *mut WORD16,
        p_qmf1: *const WORD16,
        p_qmf2: *const WORD16,
        p_out: *mut WORD32,
    ) -> VOID;
    fn ixheaacd_sbr_qmfanal32_winadd_eld_32(
        inp1: *mut WORD32,
        inp2: *mut WORD32,
        p_qmf1: *const WORD32,
        p_qmf2: *const WORD32,
        p_out: *mut WORD32,
    ) -> VOID;
    fn ixheaacd_sbr_qmfanal32_winadd_eld_mps(
        inp1: *mut WORD32,
        inp2: *mut WORD32,
        p_qmf1: *const WORD32,
        p_qmf2: *const WORD32,
        p_out: *mut WORD32,
    ) -> VOID;
    fn ixheaacd_dct2_32(
        inp: *mut WORD32,
        out: *mut WORD32,
        qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
        filter_states: *mut WORD16,
    ) -> VOID;
    fn ixheaacd_fftposttw(
        out: *mut WORD32,
        qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
    ) -> VOID;
    fn ixheaacd_posttwdct2(
        inp: *mut WORD32,
        out_fwd: *mut WORD16,
        qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
    ) -> VOID;
    fn ixheaacd_complex_fft_p3(
        xr: *mut WORD32,
        xi: *mut WORD32,
        nlength: WORD32,
        fft_mode: WORD32,
        preshift: *mut WORD32,
    ) -> VOID;
    static mut ixheaacd_complex_fft_p2: Option<
        unsafe extern "C" fn(
            *mut WORD32,
            *mut WORD32,
            WORD32,
            WORD32,
            *mut WORD32,
        ) -> VOID,
    >;
    static ixheaacd_ldmps_polyphase_filter_coeff_fix: [WORD32; 1280];
}
pub type UWORD8 = core::ffi::c_uchar;
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
pub struct ia_qmf_dec_tables_struct {
    pub w_32: [WORD16; 60],
    pub w_16: [WORD16; 24],
    pub dig_rev_table2_32: [WORD32; 4],
    pub dig_rev_table4_16: [WORD32; 2],
    pub sbr_sin_cos_twiddle_l64: [WORD16; 64],
    pub sbr_alt_sin_twiddle_l64: [WORD16; 32],
    pub sbr_cos_sin_twiddle_ds_l32: [WORD16; 64],
    pub sbr_sin_cos_twiddle_l32: [WORD16; 32],
    pub sbr_alt_sin_twiddle_l32: [WORD16; 16],
    pub sbr_t_cos_sin_l32: [WORD16; 64],
    pub post_fft_tbl: [WORD16; 18],
    pub dct23_tw: [WORD16; 66],
    pub qmf_c: [WORD16; 1280],
    pub dig_rev_table2_128: [UWORD8; 4],
    pub w1024: [WORD32; 1536],
    pub esbr_qmf_c: [WORD32; 1280],
    pub esbr_qmf_c_24: [WORD32; 480],
    pub esbr_w_32: [WORD32; 60],
    pub esbr_w_16: [WORD32; 24],
    pub esbr_sin_cos_twiddle_l64: [WORD32; 64],
    pub esbr_alt_sin_twiddle_l64: [WORD32; 32],
    pub esbr_sin_cos_twiddle_l32: [WORD32; 32],
    pub esbr_alt_sin_twiddle_l32: [WORD32; 16],
    pub esbr_t_cos_sin_l32: [WORD32; 64],
    pub esbr_sin_cos_twiddle_l24: [WORD32; 24],
    pub esbr_alt_sin_twiddle_l24: [WORD32; 12],
    pub esbr_t_cos_sin_l24: [WORD32; 48],
    pub esbr_sin_cos_twiddle_l16: [WORD32; 16],
    pub esbr_alt_sin_twiddle_l16: [WORD32; 8],
    pub esbr_t_cos_sin_l16: [WORD32; 32],
    pub ixheaacd_sbr_t_cos_sin_l32_eld: [WORD16; 64],
    pub qmf_c_eld: [WORD16; 640],
    pub qmf_c_eld2: [WORD16; 640],
    pub qmf_c_eld3: [WORD16; 640],
    pub qmf_c_ldsbr_mps: [WORD32; 640],
    pub ixheaacd_sbr_synth_cos_sin_l32: [WORD16; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_qmf_filter_bank_struct {
    pub no_channels: WORD32,
    pub analy_win_coeff: *const WORD16,
    pub p_filter: *const WORD16,
    pub cos_twiddle: *const WORD16,
    pub sin_twiddle: *const WORD16,
    pub alt_sin_twiddle: *const WORD16,
    pub t_cos: *const WORD16,
    pub t_sin: *const WORD16,
    pub anal_filter_states: *mut WORD16,
    pub filter_states: *mut WORD16,
    pub num_time_slots: WORD16,
    pub lsb: WORD16,
    pub usb: WORD16,
    pub qmf_filter_state_size: WORD16,
    pub core_samples_buffer: *mut WORD16,
    pub ana_offset: WORD16,
    pub filter_pos: *mut WORD16,
    pub dummy_0: *mut WORD16,
    pub ixheaacd_drc_offset: WORD16,
    pub filter_pos_syn: *mut WORD16,
    pub dummy_1: *mut WORD16,
    pub analy_win_coeff_32: *mut WORD32,
    pub p_filter_32: *const WORD32,
    pub esbr_cos_twiddle: *const WORD32,
    pub esbr_alt_sin_twiddle: *const WORD32,
    pub esbr_t_cos: *const WORD32,
    pub anal_filter_states_32: *mut WORD32,
    pub state_new_samples_pos_low_32: *mut WORD32,
    pub filter_states_32: *mut WORD32,
    pub filter_pos_32: *mut WORD32,
    pub filter_pos_syn_32: *mut WORD32,
    pub fp1_anal: *mut WORD16,
    pub fp2_anal: *mut WORD16,
    pub filter_2: *mut WORD16,
    pub fp1_syn: *mut WORD16,
    pub fp2_syn: *mut WORD16,
    pub sixty4: WORD16,
    pub core_samples_buffer_32: *mut WORD32,
    pub fp1_anal_32: *mut WORD32,
    pub fp2_anal_32: *mut WORD32,
    pub filter_2_32: *mut WORD32,
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
pub const MAX_64: WORD64 = 0x7fffffffffffffff as core::ffi::c_long as WORD64;
pub const MIN_64: WORD64 = 0x8000000000000000 as core::ffi::c_ulong as WORD64;
pub const MAX_32: WORD32 = 0x7fffffff as core::ffi::c_long as WORD32;
pub const MIN_32: WORD32 = 0x80000000 as core::ffi::c_long as WORD32;
#[inline]
unsafe extern "C" fn ixheaac_min32(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut min_val: WORD32 = 0;
    min_val = if a < b { a } else { b };
    return min_val;
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
unsafe extern "C" fn ixheaac_mult16x16in32(mut a: WORD16, mut b: WORD16) -> WORD32 {
    let mut product: WORD32 = 0;
    product = a as WORD32 * b as WORD32;
    return product;
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
unsafe extern "C" fn ixheaac_round16(mut op1: WORD32) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = (ixheaac_add32_sat(op1, 0x8000 as WORD32) >> 16 as core::ffi::c_int)
        as WORD16;
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
unsafe extern "C" fn ixheaac_mult64(mut a: WORD32, mut b: WORD32) -> WORD64 {
    let mut result: WORD64 = 0;
    result = a as WORD64 * b as WORD64;
    return result;
}
#[inline]
unsafe extern "C" fn ixheaac_add64(mut a: WORD64, mut b: WORD64) -> WORD64 {
    let mut result: WORD64 = 0;
    result = a + b;
    return result;
}
#[inline]
unsafe extern "C" fn ixheaac_sub64(mut a: WORD64, mut b: WORD64) -> WORD64 {
    let mut diff: WORD64 = 0;
    diff = a - b;
    return diff;
}
#[inline]
unsafe extern "C" fn ixheaac_sub64_sat(mut a: WORD64, mut b: WORD64) -> WORD64 {
    let mut diff: WORD64 = 0;
    diff = ixheaac_sub64(a, b);
    if (a ^ b) & MIN_64 != 0 as WORD64 {
        if (diff ^ a) & MIN_64 != 0 {
            diff = if a < 0 as WORD64 { MIN_64 } else { MAX_64 };
        }
    }
    return diff;
}
pub const DCT3_LEN: core::ffi::c_int = 32 as core::ffi::c_int;
pub const LP_SHIFT_VAL: core::ffi::c_int = 7 as core::ffi::c_int;
pub const HQ_SHIFT_64: core::ffi::c_int = 4 as core::ffi::c_int;
pub const RADIXSHIFT: core::ffi::c_int = 1 as core::ffi::c_int;
pub const HQ_SHIFT_VAL: core::ffi::c_int = 4 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_dct3_32(
    mut input: *mut WORD32,
    mut output: *mut WORD32,
    mut main_twidle_fwd: *const WORD16,
    mut post_tbl: *const WORD16,
    mut w_16: *const WORD16,
    mut p_table: *const WORD32,
) -> VOID {
    let mut n: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut temp1: [WORD32; 6] = [0; 6];
    let mut temp2: [WORD32; 4] = [0; 4];
    let mut twid_re: WORD16 = 0;
    let mut twid_im: WORD16 = 0;
    let mut ptr_reverse: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr_forward: *mut WORD32 = 0 as *mut WORD32;
    let mut p_out: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr_out1: *mut WORD32 = 0 as *mut WORD32;
    let mut twidle_fwd: *const WORD16 = 0 as *const WORD16;
    let mut twidle_rev: *const WORD16 = 0 as *const WORD16;
    ptr_forward = &mut *input.offset(49 as core::ffi::c_int as isize) as *mut WORD32;
    ptr_reverse = &mut *input.offset(47 as core::ffi::c_int as isize) as *mut WORD32;
    p_out = output;
    twidle_fwd = main_twidle_fwd;
    twidle_fwd = twidle_fwd.offset(4 as core::ffi::c_int as isize);
    let fresh0 = p_out;
    p_out = p_out.offset(1);
    *fresh0 = *input.offset(48 as core::ffi::c_int as isize) >> LP_SHIFT_VAL;
    let fresh1 = p_out;
    p_out = p_out.offset(1);
    *fresh1 = 0 as core::ffi::c_int as WORD32;
    n = 1 as core::ffi::c_int as WORD32;
    while n < DCT3_LEN / 2 as core::ffi::c_int {
        let fresh2 = ptr_forward;
        ptr_forward = ptr_forward.offset(1);
        temp1[0 as core::ffi::c_int as usize] = *fresh2;
        let fresh3 = ptr_reverse;
        ptr_reverse = ptr_reverse.offset(-1);
        temp1[1 as core::ffi::c_int as usize] = *fresh3;
        temp1[0 as core::ffi::c_int as usize] = ixheaac_add32_sat(
            ixheaac_shr32(temp1[0 as core::ffi::c_int as usize], LP_SHIFT_VAL),
            ixheaac_shr32(temp1[1 as core::ffi::c_int as usize], LP_SHIFT_VAL),
        );
        temp1[2 as core::ffi::c_int as usize] = *ptr_forward
            .offset(-(33 as core::ffi::c_int as isize));
        temp1[3 as core::ffi::c_int as usize] = *ptr_reverse
            .offset(-(31 as core::ffi::c_int as isize));
        temp1[1 as core::ffi::c_int as usize] = ixheaac_sub32_sat(
            ixheaac_shr32(temp1[2 as core::ffi::c_int as usize], LP_SHIFT_VAL),
            ixheaac_shr32(temp1[3 as core::ffi::c_int as usize], LP_SHIFT_VAL),
        );
        let fresh4 = twidle_fwd;
        twidle_fwd = twidle_fwd.offset(1);
        twid_re = *fresh4;
        twid_im = *twidle_fwd;
        twidle_fwd = twidle_fwd.offset(3 as core::ffi::c_int as isize);
        let fresh5 = p_out;
        p_out = p_out.offset(1);
        *fresh5 = ixheaac_mult32x16in32(temp1[0 as core::ffi::c_int as usize], twid_re)
            + ixheaac_mult32x16in32(temp1[1 as core::ffi::c_int as usize], twid_im);
        let fresh6 = p_out;
        p_out = p_out.offset(1);
        *fresh6 = -ixheaac_mult32x16in32(temp1[1 as core::ffi::c_int as usize], twid_re)
            + ixheaac_mult32x16in32(temp1[0 as core::ffi::c_int as usize], twid_im);
        n += 1;
    }
    let fresh7 = twidle_fwd;
    twidle_fwd = twidle_fwd.offset(1);
    twid_re = *fresh7;
    twid_im = *twidle_fwd;
    twidle_fwd = twidle_fwd.offset(3 as core::ffi::c_int as isize);
    let fresh8 = ptr_reverse;
    ptr_reverse = ptr_reverse.offset(-1);
    temp1[1 as core::ffi::c_int as usize] = *fresh8;
    temp1[0 as core::ffi::c_int as usize] = *ptr_reverse
        .offset(-(31 as core::ffi::c_int as isize));
    temp1[1 as core::ffi::c_int as usize] = ixheaac_sub32_sat(
        ixheaac_shr32(temp1[1 as core::ffi::c_int as usize], LP_SHIFT_VAL),
        ixheaac_shr32(temp1[0 as core::ffi::c_int as usize], LP_SHIFT_VAL),
    );
    temp1[0 as core::ffi::c_int as usize] = temp1[1 as core::ffi::c_int as usize];
    temp2[2 as core::ffi::c_int as usize] = ixheaac_mult32x16in32(
        temp1[0 as core::ffi::c_int as usize],
        twid_re,
    ) + ixheaac_mult32x16in32(temp1[1 as core::ffi::c_int as usize], twid_im);
    temp2[3 as core::ffi::c_int as usize] = -ixheaac_mult32x16in32(
        temp1[1 as core::ffi::c_int as usize],
        twid_re,
    ) + ixheaac_mult32x16in32(temp1[0 as core::ffi::c_int as usize], twid_im);
    ptr_forward = output;
    ptr_reverse = &mut *output.offset((DCT3_LEN - 1 as core::ffi::c_int) as isize)
        as *mut WORD32;
    let fresh9 = ptr_forward;
    ptr_forward = ptr_forward.offset(1);
    temp2[0 as core::ffi::c_int as usize] = *fresh9;
    let fresh10 = ptr_forward;
    ptr_forward = ptr_forward.offset(-1);
    temp2[1 as core::ffi::c_int as usize] = *fresh10;
    temp1[0 as core::ffi::c_int as usize] = -temp2[1 as core::ffi::c_int as usize]
        - temp2[3 as core::ffi::c_int as usize];
    temp1[1 as core::ffi::c_int as usize] = temp2[0 as core::ffi::c_int as usize]
        - temp2[2 as core::ffi::c_int as usize];
    temp2[0 as core::ffi::c_int as usize] = temp2[0 as core::ffi::c_int as usize]
        + temp2[2 as core::ffi::c_int as usize] + temp1[0 as core::ffi::c_int as usize];
    temp2[1 as core::ffi::c_int as usize] = temp2[1 as core::ffi::c_int as usize]
        - temp2[3 as core::ffi::c_int as usize] + temp1[1 as core::ffi::c_int as usize];
    temp2[0 as core::ffi::c_int as usize] >>= 1 as core::ffi::c_int;
    temp2[1 as core::ffi::c_int as usize] >>= 1 as core::ffi::c_int;
    let fresh11 = ptr_forward;
    ptr_forward = ptr_forward.offset(1);
    *fresh11 = temp2[0 as core::ffi::c_int as usize];
    let fresh12 = ptr_forward;
    ptr_forward = ptr_forward.offset(1);
    *fresh12 = temp2[1 as core::ffi::c_int as usize];
    twidle_fwd = post_tbl.offset(2 as core::ffi::c_int as isize);
    twidle_rev = post_tbl.offset(14 as core::ffi::c_int as isize);
    n = 1 as core::ffi::c_int as WORD32;
    while n < DCT3_LEN / 4 as core::ffi::c_int {
        let fresh13 = ptr_forward;
        ptr_forward = ptr_forward.offset(1);
        temp2[0 as core::ffi::c_int as usize] = *fresh13;
        let fresh14 = ptr_forward;
        ptr_forward = ptr_forward.offset(-1);
        temp2[1 as core::ffi::c_int as usize] = *fresh14;
        let fresh15 = ptr_reverse;
        ptr_reverse = ptr_reverse.offset(-1);
        temp2[3 as core::ffi::c_int as usize] = *fresh15;
        let fresh16 = ptr_reverse;
        ptr_reverse = ptr_reverse.offset(1);
        temp2[2 as core::ffi::c_int as usize] = *fresh16;
        twid_re = *twidle_rev;
        twidle_rev = twidle_rev.offset(-(2 as core::ffi::c_int as isize));
        twid_im = *twidle_fwd;
        twidle_fwd = twidle_fwd.offset(2 as core::ffi::c_int as isize);
        temp1[0 as core::ffi::c_int as usize] = temp2[0 as core::ffi::c_int as usize]
            - temp2[2 as core::ffi::c_int as usize];
        temp1[1 as core::ffi::c_int as usize] = temp2[0 as core::ffi::c_int as usize]
            + temp2[2 as core::ffi::c_int as usize];
        temp1[2 as core::ffi::c_int as usize] = temp2[1 as core::ffi::c_int as usize]
            + temp2[3 as core::ffi::c_int as usize];
        temp1[3 as core::ffi::c_int as usize] = temp2[1 as core::ffi::c_int as usize]
            - temp2[3 as core::ffi::c_int as usize];
        temp1[4 as core::ffi::c_int as usize] = ixheaac_mult32x16in32(
            temp1[0 as core::ffi::c_int as usize],
            twid_re,
        ) + ixheaac_mult32x16in32(temp1[2 as core::ffi::c_int as usize], twid_im);
        temp1[5 as core::ffi::c_int as usize] = -ixheaac_mult32x16in32(
            temp1[2 as core::ffi::c_int as usize],
            twid_re,
        ) + ixheaac_mult32x16in32(temp1[0 as core::ffi::c_int as usize], twid_im);
        temp1[1 as core::ffi::c_int as usize] >>= 1 as core::ffi::c_int;
        temp1[3 as core::ffi::c_int as usize] >>= 1 as core::ffi::c_int;
        let fresh17 = ptr_forward;
        ptr_forward = ptr_forward.offset(1);
        *fresh17 = temp1[1 as core::ffi::c_int as usize]
            - temp1[4 as core::ffi::c_int as usize];
        let fresh18 = ptr_forward;
        ptr_forward = ptr_forward.offset(1);
        *fresh18 = temp1[3 as core::ffi::c_int as usize]
            + temp1[5 as core::ffi::c_int as usize];
        let fresh19 = ptr_reverse;
        ptr_reverse = ptr_reverse.offset(-1);
        *fresh19 = -temp1[3 as core::ffi::c_int as usize]
            + temp1[5 as core::ffi::c_int as usize];
        let fresh20 = ptr_reverse;
        ptr_reverse = ptr_reverse.offset(-1);
        *fresh20 = temp1[1 as core::ffi::c_int as usize]
            + temp1[4 as core::ffi::c_int as usize];
        n += 1;
    }
    let fresh21 = ptr_forward;
    ptr_forward = ptr_forward.offset(1);
    temp2[0 as core::ffi::c_int as usize] = *fresh21;
    let fresh22 = ptr_forward;
    ptr_forward = ptr_forward.offset(-1);
    temp2[1 as core::ffi::c_int as usize] = *fresh22;
    let fresh23 = ptr_reverse;
    ptr_reverse = ptr_reverse.offset(-1);
    temp2[3 as core::ffi::c_int as usize] = *fresh23;
    let fresh24 = ptr_reverse;
    ptr_reverse = ptr_reverse.offset(1);
    temp2[2 as core::ffi::c_int as usize] = *fresh24;
    twid_re = -(*twidle_rev as core::ffi::c_int) as WORD16;
    twidle_rev = twidle_rev.offset(-(2 as core::ffi::c_int as isize));
    twid_im = *twidle_fwd;
    twidle_fwd = twidle_fwd.offset(2 as core::ffi::c_int as isize);
    temp1[0 as core::ffi::c_int as usize] = temp2[0 as core::ffi::c_int as usize]
        - temp2[2 as core::ffi::c_int as usize];
    temp1[1 as core::ffi::c_int as usize] = temp2[0 as core::ffi::c_int as usize]
        + temp2[2 as core::ffi::c_int as usize];
    temp1[2 as core::ffi::c_int as usize] = temp2[1 as core::ffi::c_int as usize]
        + temp2[3 as core::ffi::c_int as usize];
    temp1[3 as core::ffi::c_int as usize] = temp2[1 as core::ffi::c_int as usize]
        - temp2[3 as core::ffi::c_int as usize];
    temp1[4 as core::ffi::c_int as usize] = ixheaac_mult32x16in32(
        temp1[0 as core::ffi::c_int as usize],
        twid_re,
    ) - ixheaac_mult32x16in32(temp1[2 as core::ffi::c_int as usize], twid_im);
    temp1[5 as core::ffi::c_int as usize] = ixheaac_mult32x16in32(
        temp1[2 as core::ffi::c_int as usize],
        twid_re,
    ) + ixheaac_mult32x16in32(temp1[0 as core::ffi::c_int as usize], twid_im);
    temp1[1 as core::ffi::c_int as usize] >>= 1 as core::ffi::c_int;
    temp1[3 as core::ffi::c_int as usize] >>= 1 as core::ffi::c_int;
    let fresh25 = ptr_forward;
    ptr_forward = ptr_forward.offset(1);
    *fresh25 = temp1[1 as core::ffi::c_int as usize]
        + temp1[4 as core::ffi::c_int as usize];
    let fresh26 = ptr_forward;
    ptr_forward = ptr_forward.offset(1);
    *fresh26 = temp1[3 as core::ffi::c_int as usize]
        + temp1[5 as core::ffi::c_int as usize];
    ixheaacd_radix4bfly(w_16, output, 1 as WORD32, 4 as WORD32);
    ixheaacd_postradixcompute4(input, output, p_table, 16 as WORD32);
    *output.offset(0 as core::ffi::c_int as isize) = *input
        .offset(0 as core::ffi::c_int as isize);
    *output.offset(2 as core::ffi::c_int as isize) = *input
        .offset(1 as core::ffi::c_int as isize);
    p_out = input.offset(2 as core::ffi::c_int as isize);
    ptr_forward = output.offset(1 as core::ffi::c_int as isize);
    ptr_reverse = output.offset(30 as core::ffi::c_int as isize);
    ptr_out1 = input.offset(18 as core::ffi::c_int as isize);
    k = (DCT3_LEN / 4 as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while k != 0 as core::ffi::c_int {
        let mut tempre: WORD32 = 0;
        let mut tempim: WORD32 = 0;
        let fresh27 = p_out;
        p_out = p_out.offset(1);
        tempre = *fresh27;
        let fresh28 = p_out;
        p_out = p_out.offset(1);
        tempim = *fresh28;
        *ptr_forward = tempim;
        ptr_forward = ptr_forward.offset(2 as core::ffi::c_int as isize);
        *ptr_forward = tempre;
        ptr_forward = ptr_forward.offset(2 as core::ffi::c_int as isize);
        let fresh29 = ptr_out1;
        ptr_out1 = ptr_out1.offset(1);
        tempre = *fresh29;
        let fresh30 = ptr_out1;
        ptr_out1 = ptr_out1.offset(1);
        tempim = *fresh30;
        *ptr_reverse = tempim;
        ptr_reverse = ptr_reverse.offset(-(2 as core::ffi::c_int as isize));
        *ptr_reverse = tempre;
        ptr_reverse = ptr_reverse.offset(-(2 as core::ffi::c_int as isize));
        k -= 1;
    }
    let mut tempre_0: WORD32 = 0;
    let mut tempim_0: WORD32 = 0;
    let fresh31 = p_out;
    p_out = p_out.offset(1);
    tempre_0 = *fresh31;
    let fresh32 = p_out;
    p_out = p_out.offset(1);
    tempim_0 = *fresh32;
    *ptr_forward = tempim_0;
    ptr_forward = ptr_forward.offset(2 as core::ffi::c_int as isize);
    *ptr_forward = tempre_0;
    ptr_forward = ptr_forward.offset(2 as core::ffi::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_dct2_64(
    mut x: *mut WORD32,
    mut X: *mut WORD32,
    mut qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
    mut filter_states: *mut WORD16,
) -> VOID {
    ixheaacd_pretwdct2(x, X);
    ixheaacd_radix4bfly(
        ((*qmf_dec_tables_ptr).w_32).as_mut_ptr(),
        X,
        1 as WORD32,
        8 as WORD32,
    );
    ixheaacd_radix4bfly(
        ((*qmf_dec_tables_ptr).w_32)
            .as_mut_ptr()
            .offset(48 as core::ffi::c_int as isize),
        X,
        4 as WORD32,
        2 as WORD32,
    );
    ixheaacd_postradixcompute2(
        x,
        X,
        ((*qmf_dec_tables_ptr).dig_rev_table2_32).as_mut_ptr(),
        32 as WORD32,
    );
    ixheaacd_fftposttw(x, qmf_dec_tables_ptr);
    ixheaacd_posttwdct2(x, filter_states, qmf_dec_tables_ptr);
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_cos_sin_mod(
    mut subband: *mut WORD32,
    mut qmf_bank: *mut ia_sbr_qmf_filter_bank_struct,
    mut p_twiddle: *mut WORD16,
    mut p_dig_rev_tbl: *mut WORD32,
) -> VOID {
    let mut re2: WORD32 = 0;
    let mut re3: WORD32 = 0;
    let mut wim: WORD16 = 0;
    let mut wre: WORD16 = 0;
    let mut i: WORD32 = 0;
    let mut M_2: WORD32 = 0;
    let mut M: WORD32 = ixheaac_shr32((*qmf_bank).no_channels, 1 as WORD);
    let mut p_sin: *const WORD16 = 0 as *const WORD16;
    let mut p_sin_cos: *const WORD16 = &*((*qmf_bank).cos_twiddle)
        .offset(0 as core::ffi::c_int as isize) as *const WORD16;
    let mut subband_tmp: [WORD32; 128] = [0; 128];
    let mut re: WORD32 = 0;
    let mut im: WORD32 = 0;
    let mut psubband: *mut WORD32 = 0 as *mut WORD32;
    let mut psubband1: *mut WORD32 = 0 as *mut WORD32;
    let mut psubband_t: *mut WORD32 = 0 as *mut WORD32;
    let mut psubband1_t: *mut WORD32 = 0 as *mut WORD32;
    let mut psubband2: *mut WORD32 = 0 as *mut WORD32;
    let mut psubband12: *mut WORD32 = 0 as *mut WORD32;
    let mut psubband_t2: *mut WORD32 = 0 as *mut WORD32;
    let mut psubband1_t2: *mut WORD32 = 0 as *mut WORD32;
    M_2 = ixheaac_shr32(M, 1 as WORD);
    psubband = &mut *subband.offset(0 as core::ffi::c_int as isize) as *mut WORD32;
    psubband1 = &mut *subband
        .offset(
            (2 as core::ffi::c_int * M as core::ffi::c_int - 1 as core::ffi::c_int)
                as isize,
        ) as *mut WORD32;
    psubband_t = subband_tmp.as_mut_ptr();
    psubband1_t = &mut *subband_tmp
        .as_mut_ptr()
        .offset(
            (2 as core::ffi::c_int * M as core::ffi::c_int - 1 as core::ffi::c_int)
                as isize,
        ) as *mut WORD32;
    psubband2 = &mut *subband.offset(64 as core::ffi::c_int as isize) as *mut WORD32;
    psubband12 = &mut *subband
        .offset(
            (2 as core::ffi::c_int * M as core::ffi::c_int - 1 as core::ffi::c_int
                + 64 as core::ffi::c_int) as isize,
        ) as *mut WORD32;
    psubband_t2 = &mut *subband_tmp.as_mut_ptr().offset(64 as core::ffi::c_int as isize)
        as *mut WORD32;
    psubband1_t2 = &mut *subband_tmp
        .as_mut_ptr()
        .offset(
            (2 as core::ffi::c_int * M as core::ffi::c_int - 1 as core::ffi::c_int
                + 64 as core::ffi::c_int) as isize,
        ) as *mut WORD32;
    i = ((M_2 as core::ffi::c_int >> 1 as core::ffi::c_int) - 1 as core::ffi::c_int)
        as WORD32;
    while i >= 0 as core::ffi::c_int {
        let fresh64 = psubband;
        psubband = psubband.offset(1);
        re = *fresh64;
        let fresh65 = psubband1;
        psubband1 = psubband1.offset(-1);
        im = *fresh65;
        let fresh66 = p_sin_cos;
        p_sin_cos = p_sin_cos.offset(1);
        wim = *fresh66;
        let fresh67 = p_sin_cos;
        p_sin_cos = p_sin_cos.offset(1);
        wre = *fresh67;
        let fresh68 = psubband_t;
        psubband_t = psubband_t.offset(1);
        *fresh68 = ixheaac_add32_sat(
            ixheaac_mult32x16in32(re, wre),
            ixheaac_mult32x16in32(im, wim),
        );
        let fresh69 = psubband_t;
        psubband_t = psubband_t.offset(1);
        *fresh69 = ixheaac_sub32_sat(
            ixheaac_mult32x16in32(im, wre),
            ixheaac_mult32x16in32(re, wim),
        );
        let fresh70 = psubband2;
        psubband2 = psubband2.offset(1);
        re = *fresh70;
        let fresh71 = psubband12;
        psubband12 = psubband12.offset(-1);
        im = *fresh71;
        let fresh72 = psubband_t2;
        psubband_t2 = psubband_t2.offset(1);
        *fresh72 = ixheaac_sub32_sat(
            ixheaac_mult32x16in32(im, wim),
            ixheaac_mult32x16in32(re, wre),
        );
        let fresh73 = psubband_t2;
        psubband_t2 = psubband_t2.offset(1);
        *fresh73 = ixheaac_add32_sat(
            ixheaac_mult32x16in32(re, wim),
            ixheaac_mult32x16in32(im, wre),
        );
        let fresh74 = psubband1;
        psubband1 = psubband1.offset(-1);
        re = *fresh74;
        let fresh75 = psubband;
        psubband = psubband.offset(1);
        im = *fresh75;
        let fresh76 = p_sin_cos;
        p_sin_cos = p_sin_cos.offset(1);
        wim = *fresh76;
        let fresh77 = p_sin_cos;
        p_sin_cos = p_sin_cos.offset(1);
        wre = *fresh77;
        let fresh78 = psubband1_t;
        psubband1_t = psubband1_t.offset(-1);
        *fresh78 = ixheaac_sub32_sat(
            ixheaac_mult32x16in32(im, wre),
            ixheaac_mult32x16in32(re, wim),
        );
        let fresh79 = psubband1_t;
        psubband1_t = psubband1_t.offset(-1);
        *fresh79 = ixheaac_add32_sat(
            ixheaac_mult32x16in32(re, wre),
            ixheaac_mult32x16in32(im, wim),
        );
        let fresh80 = psubband12;
        psubband12 = psubband12.offset(-1);
        re = *fresh80;
        let fresh81 = psubband2;
        psubband2 = psubband2.offset(1);
        im = *fresh81;
        let fresh82 = psubband1_t2;
        psubband1_t2 = psubband1_t2.offset(-1);
        *fresh82 = ixheaac_add32_sat(
            ixheaac_mult32x16in32(re, wim),
            ixheaac_mult32x16in32(im, wre),
        );
        let fresh83 = psubband1_t2;
        psubband1_t2 = psubband1_t2.offset(-1);
        *fresh83 = ixheaac_sub32_sat(
            ixheaac_mult32x16in32(im, wim),
            ixheaac_mult32x16in32(re, wre),
        );
        let fresh84 = psubband;
        psubband = psubband.offset(1);
        re = *fresh84;
        let fresh85 = psubband1;
        psubband1 = psubband1.offset(-1);
        im = *fresh85;
        let fresh86 = p_sin_cos;
        p_sin_cos = p_sin_cos.offset(1);
        wim = *fresh86;
        let fresh87 = p_sin_cos;
        p_sin_cos = p_sin_cos.offset(1);
        wre = *fresh87;
        let fresh88 = psubband_t;
        psubband_t = psubband_t.offset(1);
        *fresh88 = ixheaac_add32_sat(
            ixheaac_mult32x16in32(re, wre),
            ixheaac_mult32x16in32(im, wim),
        );
        let fresh89 = psubband_t;
        psubband_t = psubband_t.offset(1);
        *fresh89 = ixheaac_sub32_sat(
            ixheaac_mult32x16in32(im, wre),
            ixheaac_mult32x16in32(re, wim),
        );
        let fresh90 = psubband2;
        psubband2 = psubband2.offset(1);
        re = *fresh90;
        let fresh91 = psubband12;
        psubband12 = psubband12.offset(-1);
        im = *fresh91;
        let fresh92 = psubband_t2;
        psubband_t2 = psubband_t2.offset(1);
        *fresh92 = ixheaac_sub32_sat(
            ixheaac_mult32x16in32(im, wim),
            ixheaac_mult32x16in32(re, wre),
        );
        let fresh93 = psubband_t2;
        psubband_t2 = psubband_t2.offset(1);
        *fresh93 = ixheaac_add32_sat(
            ixheaac_mult32x16in32(re, wim),
            ixheaac_mult32x16in32(im, wre),
        );
        let fresh94 = psubband1;
        psubband1 = psubband1.offset(-1);
        re = *fresh94;
        let fresh95 = psubband;
        psubband = psubband.offset(1);
        im = *fresh95;
        let fresh96 = p_sin_cos;
        p_sin_cos = p_sin_cos.offset(1);
        wim = *fresh96;
        let fresh97 = p_sin_cos;
        p_sin_cos = p_sin_cos.offset(1);
        wre = *fresh97;
        let fresh98 = psubband1_t;
        psubband1_t = psubband1_t.offset(-1);
        *fresh98 = ixheaac_sub32_sat(
            ixheaac_mult32x16in32(im, wre),
            ixheaac_mult32x16in32(re, wim),
        );
        let fresh99 = psubband1_t;
        psubband1_t = psubband1_t.offset(-1);
        *fresh99 = ixheaac_add32_sat(
            ixheaac_mult32x16in32(re, wre),
            ixheaac_mult32x16in32(im, wim),
        );
        let fresh100 = psubband12;
        psubband12 = psubband12.offset(-1);
        re = *fresh100;
        let fresh101 = psubband2;
        psubband2 = psubband2.offset(1);
        im = *fresh101;
        let fresh102 = psubband1_t2;
        psubband1_t2 = psubband1_t2.offset(-1);
        *fresh102 = ixheaac_add32_sat(
            ixheaac_mult32x16in32(re, wim),
            ixheaac_mult32x16in32(im, wre),
        );
        let fresh103 = psubband1_t2;
        psubband1_t2 = psubband1_t2.offset(-1);
        *fresh103 = ixheaac_sub32_sat(
            ixheaac_mult32x16in32(im, wim),
            ixheaac_mult32x16in32(re, wre),
        );
        i -= 1;
    }
    if M == 32 as core::ffi::c_int {
        ixheaacd_radix4bfly(
            p_twiddle,
            subband_tmp.as_mut_ptr(),
            1 as WORD32,
            8 as WORD32,
        );
        ixheaacd_radix4bfly(
            p_twiddle.offset(48 as core::ffi::c_int as isize),
            subband_tmp.as_mut_ptr(),
            4 as WORD32,
            2 as WORD32,
        );
        ixheaacd_postradixcompute2(
            subband,
            subband_tmp.as_mut_ptr(),
            p_dig_rev_tbl,
            32 as WORD32,
        );
        ixheaacd_radix4bfly(
            p_twiddle,
            &mut *subband_tmp.as_mut_ptr().offset(64 as core::ffi::c_int as isize),
            1 as WORD32,
            8 as WORD32,
        );
        ixheaacd_radix4bfly(
            p_twiddle.offset(48 as core::ffi::c_int as isize),
            &mut *subband_tmp.as_mut_ptr().offset(64 as core::ffi::c_int as isize),
            4 as WORD32,
            2 as WORD32,
        );
        ixheaacd_postradixcompute2(
            &mut *subband.offset(64 as core::ffi::c_int as isize),
            &mut *subband_tmp.as_mut_ptr().offset(64 as core::ffi::c_int as isize),
            p_dig_rev_tbl,
            32 as WORD32,
        );
    } else {
        ixheaacd_radix4bfly(
            p_twiddle,
            subband_tmp.as_mut_ptr(),
            1 as WORD32,
            4 as WORD32,
        );
        ixheaacd_postradixcompute4(
            subband,
            subband_tmp.as_mut_ptr(),
            p_dig_rev_tbl,
            16 as WORD32,
        );
        ixheaacd_radix4bfly(
            p_twiddle,
            &mut *subband_tmp.as_mut_ptr().offset(64 as core::ffi::c_int as isize),
            1 as WORD32,
            4 as WORD32,
        );
        ixheaacd_postradixcompute4(
            &mut *subband.offset(64 as core::ffi::c_int as isize),
            &mut *subband_tmp.as_mut_ptr().offset(64 as core::ffi::c_int as isize),
            p_dig_rev_tbl,
            16 as WORD32,
        );
    }
    psubband = &mut *subband.offset(0 as core::ffi::c_int as isize) as *mut WORD32;
    psubband1 = &mut *subband
        .offset(
            (2 as core::ffi::c_int * M as core::ffi::c_int - 1 as core::ffi::c_int)
                as isize,
        ) as *mut WORD32;
    re = *psubband1;
    *psubband = *psubband >> 1 as core::ffi::c_int;
    psubband = psubband.offset(1);
    *psubband1 = ixheaac_negate32_sat(*psubband >> 1 as core::ffi::c_int);
    psubband1 = psubband1.offset(-1);
    p_sin = &*((*qmf_bank).alt_sin_twiddle).offset(0 as core::ffi::c_int as isize)
        as *const WORD16;
    let fresh104 = p_sin;
    p_sin = p_sin.offset(1);
    wim = *fresh104;
    let fresh105 = p_sin;
    p_sin = p_sin.offset(1);
    wre = *fresh105;
    im = *psubband1;
    let fresh106 = psubband1;
    psubband1 = psubband1.offset(-1);
    *fresh106 = ixheaac_add32_sat(
        ixheaac_mult32x16in32(re, wre),
        ixheaac_mult32x16in32(im, wim),
    );
    let fresh107 = psubband;
    psubband = psubband.offset(1);
    *fresh107 = ixheaac_sub32_sat(
        ixheaac_mult32x16in32(im, wre),
        ixheaac_mult32x16in32(re, wim),
    );
    psubband2 = &mut *subband.offset(64 as core::ffi::c_int as isize) as *mut WORD32;
    psubband12 = &mut *subband
        .offset(
            (2 as core::ffi::c_int * M as core::ffi::c_int - 1 as core::ffi::c_int
                + 64 as core::ffi::c_int) as isize,
        ) as *mut WORD32;
    re = *psubband12;
    let fresh108 = psubband12;
    psubband12 = psubband12.offset(-1);
    *fresh108 = ixheaac_negate32_sat(*psubband2 >> 1 as core::ffi::c_int);
    *psubband2 = *psubband2.offset(1 as core::ffi::c_int as isize)
        >> 1 as core::ffi::c_int;
    psubband2 = psubband2.offset(1);
    im = *psubband12;
    let fresh109 = psubband2;
    psubband2 = psubband2.offset(1);
    *fresh109 = ixheaac_negate32_sat(
        ixheaac_add32_sat(ixheaac_mult32x16in32(re, wre), ixheaac_mult32x16in32(im, wim)),
    );
    let fresh110 = psubband12;
    psubband12 = psubband12.offset(-1);
    *fresh110 = ixheaac_sub32_sat(
        ixheaac_mult32x16in32(re, wim),
        ixheaac_mult32x16in32(im, wre),
    );
    i = (M_2 as core::ffi::c_int - 2 as core::ffi::c_int) as WORD32;
    while i >= 0 as core::ffi::c_int {
        im = *psubband.offset(0 as core::ffi::c_int as isize);
        re = *psubband.offset(1 as core::ffi::c_int as isize);
        re2 = *psubband1;
        let fresh111 = psubband;
        psubband = psubband.offset(1);
        *fresh111 = ixheaac_add32_sat(
            ixheaac_mult32x16in32(re, wim),
            ixheaac_mult32x16in32(im, wre),
        );
        let fresh112 = psubband1;
        psubband1 = psubband1.offset(-1);
        *fresh112 = ixheaac_sub32_sat(
            ixheaac_mult32x16in32(im, wim),
            ixheaac_mult32x16in32(re, wre),
        );
        im = *psubband2.offset(0 as core::ffi::c_int as isize);
        re = *psubband2.offset(1 as core::ffi::c_int as isize);
        re3 = *psubband12;
        let fresh113 = psubband12;
        psubband12 = psubband12.offset(-1);
        *fresh113 = ixheaac_negate32_sat(
            ixheaac_add32_sat(
                ixheaac_mult32x16in32(re, wim),
                ixheaac_mult32x16in32(im, wre),
            ),
        );
        let fresh114 = psubband2;
        psubband2 = psubband2.offset(1);
        *fresh114 = ixheaac_sub32_sat(
            ixheaac_mult32x16in32(re, wre),
            ixheaac_mult32x16in32(im, wim),
        );
        let fresh115 = p_sin;
        p_sin = p_sin.offset(1);
        wim = *fresh115;
        let fresh116 = p_sin;
        p_sin = p_sin.offset(1);
        wre = *fresh116;
        im = *psubband1.offset(0 as core::ffi::c_int as isize);
        let fresh117 = psubband1;
        psubband1 = psubband1.offset(-1);
        *fresh117 = ixheaac_add32_sat(
            ixheaac_mult32x16in32(re2, wre),
            ixheaac_mult32x16in32(im, wim),
        );
        let fresh118 = psubband;
        psubband = psubband.offset(1);
        *fresh118 = ixheaac_sub32_sat(
            ixheaac_mult32x16in32(im, wre),
            ixheaac_mult32x16in32(re2, wim),
        );
        im = *psubband12.offset(0 as core::ffi::c_int as isize);
        let fresh119 = psubband2;
        psubband2 = psubband2.offset(1);
        *fresh119 = ixheaac_negate32_sat(
            ixheaac_add32_sat(
                ixheaac_mult32x16in32(re3, wre),
                ixheaac_mult32x16in32(im, wim),
            ),
        );
        let fresh120 = psubband12;
        psubband12 = psubband12.offset(-1);
        *fresh120 = ixheaac_sub32_sat(
            ixheaac_mult32x16in32(re3, wim),
            ixheaac_mult32x16in32(im, wre),
        );
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_fwd_modulation(
    mut p_time_in1: *const WORD32,
    mut real_subband: *mut WORD32,
    mut imag_subband: *mut WORD32,
    mut qmf_bank: *mut ia_sbr_qmf_filter_bank_struct,
    mut qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
    mut ld_mps_flag: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut p_time_in2: *const WORD32 = &*p_time_in1
        .offset(
            (2 as core::ffi::c_int * (*qmf_bank).no_channels as core::ffi::c_int
                - 1 as core::ffi::c_int) as isize,
        ) as *const WORD32;
    let mut temp1: WORD32 = 0;
    let mut temp2: WORD32 = 0;
    let mut t_real_subband: *mut WORD32 = real_subband;
    let mut t_imag_subband: *mut WORD32 = imag_subband;
    let mut tcos: *const WORD16 = 0 as *const WORD16;
    i = ((*qmf_bank).no_channels as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while i >= 0 as core::ffi::c_int {
        let fresh56 = p_time_in1;
        p_time_in1 = p_time_in1.offset(1);
        temp1 = ixheaac_shr32(*fresh56, HQ_SHIFT_VAL);
        let fresh57 = p_time_in2;
        p_time_in2 = p_time_in2.offset(-1);
        temp2 = ixheaac_shr32(*fresh57, HQ_SHIFT_VAL);
        let fresh58 = t_real_subband;
        t_real_subband = t_real_subband.offset(1);
        *fresh58 = ixheaac_sub32_sat(temp1, temp2);
        let fresh59 = t_imag_subband;
        t_imag_subband = t_imag_subband.offset(1);
        *fresh59 = ixheaac_add32_sat(temp1, temp2);
        i -= 1;
    }
    if (*qmf_bank).no_channels != 64 as core::ffi::c_int {
        ixheaacd_cos_sin_mod(
            real_subband,
            qmf_bank,
            ((*qmf_dec_tables_ptr).w_16).as_mut_ptr(),
            ((*qmf_dec_tables_ptr).dig_rev_table4_16).as_mut_ptr(),
        );
    } else {
        ixheaacd_cos_sin_mod(
            real_subband,
            qmf_bank,
            ((*qmf_dec_tables_ptr).w_32).as_mut_ptr(),
            ((*qmf_dec_tables_ptr).dig_rev_table2_32).as_mut_ptr(),
        );
    }
    if ld_mps_flag == 0 as core::ffi::c_int {
        tcos = (*qmf_bank).t_cos;
        i = ((*qmf_bank).usb as core::ffi::c_int - (*qmf_bank).lsb as core::ffi::c_int
            - 1 as core::ffi::c_int) as WORD32;
        while i >= 0 as core::ffi::c_int {
            let mut cosh: WORD16 = 0;
            let mut sinh: WORD16 = 0;
            let mut re: WORD32 = 0;
            let mut im: WORD32 = 0;
            re = *real_subband;
            im = *imag_subband;
            let fresh60 = tcos;
            tcos = tcos.offset(1);
            cosh = *fresh60;
            let fresh61 = tcos;
            tcos = tcos.offset(1);
            sinh = *fresh61;
            let fresh62 = real_subband;
            real_subband = real_subband.offset(1);
            *fresh62 = ixheaac_add32_sat(
                ixheaac_mult32x16in32_shl(re, cosh),
                ixheaac_mult32x16in32_shl(im, sinh),
            );
            let fresh63 = imag_subband;
            imag_subband = imag_subband.offset(1);
            *fresh63 = ixheaac_sub32_sat(
                ixheaac_mult32x16in32_shl(im, cosh),
                ixheaac_mult32x16in32_shl(re, sinh),
            );
            i -= 1;
        }
    } else {
        let mut i_band: WORD32 = 0;
        i = 0 as core::ffi::c_int as WORD32;
        while i
            < (if (64 as core::ffi::c_int) < (*qmf_bank).no_channels {
                64 as core::ffi::c_int
            } else {
                (*qmf_bank).no_channels as core::ffi::c_int
            })
        {
            i_band = *real_subband.offset(i as isize);
            *real_subband.offset(i as isize) = -*imag_subband.offset(i as isize);
            *imag_subband.offset(i as isize) = i_band;
            i_band = -*real_subband
                .offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
            *real_subband
                .offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = *imag_subband
                .offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
            *imag_subband
                .offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = i_band;
            i += 2 as core::ffi::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_sbr_qmfanal32_winadd(
    mut inp1: *mut WORD16,
    mut inp2: *mut WORD16,
    mut p_qmf1: *const WORD16,
    mut p_qmf2: *const WORD16,
    mut p_out: *mut WORD32,
) -> VOID {
    let mut n: WORD32 = 0;
    n = 0 as core::ffi::c_int as WORD32;
    while n < 32 as core::ffi::c_int {
        let mut accu: WORD32 = 0;
        accu = ixheaac_mult16x16in32(
            *inp1.offset((n as core::ffi::c_int + 0 as core::ffi::c_int) as isize),
            *p_qmf1
                .offset(
                    (2 as core::ffi::c_int
                        * (n as core::ffi::c_int + 0 as core::ffi::c_int)) as isize,
                ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp1.offset((n as core::ffi::c_int + 64 as core::ffi::c_int) as isize),
                *p_qmf1
                    .offset(
                        (2 as core::ffi::c_int
                            * (n as core::ffi::c_int + 64 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp1.offset((n as core::ffi::c_int + 128 as core::ffi::c_int) as isize),
                *p_qmf1
                    .offset(
                        (2 as core::ffi::c_int
                            * (n as core::ffi::c_int + 128 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp1.offset((n as core::ffi::c_int + 192 as core::ffi::c_int) as isize),
                *p_qmf1
                    .offset(
                        (2 as core::ffi::c_int
                            * (n as core::ffi::c_int + 192 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp1.offset((n as core::ffi::c_int + 256 as core::ffi::c_int) as isize),
                *p_qmf1
                    .offset(
                        (2 as core::ffi::c_int
                            * (n as core::ffi::c_int + 256 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        *p_out.offset(n as isize) = accu;
        accu = ixheaac_mult16x16in32(
            *inp1
                .offset(
                    (n as core::ffi::c_int + 1 as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ),
            *p_qmf1
                .offset(
                    (2 as core::ffi::c_int
                        * (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 0 as core::ffi::c_int)) as isize,
                ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp1
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 64 as core::ffi::c_int) as isize,
                    ),
                *p_qmf1
                    .offset(
                        (2 as core::ffi::c_int
                            * (n as core::ffi::c_int + 1 as core::ffi::c_int
                                + 64 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp1
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 128 as core::ffi::c_int) as isize,
                    ),
                *p_qmf1
                    .offset(
                        (2 as core::ffi::c_int
                            * (n as core::ffi::c_int + 1 as core::ffi::c_int
                                + 128 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp1
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 192 as core::ffi::c_int) as isize,
                    ),
                *p_qmf1
                    .offset(
                        (2 as core::ffi::c_int
                            * (n as core::ffi::c_int + 1 as core::ffi::c_int
                                + 192 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp1
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 256 as core::ffi::c_int) as isize,
                    ),
                *p_qmf1
                    .offset(
                        (2 as core::ffi::c_int
                            * (n as core::ffi::c_int + 1 as core::ffi::c_int
                                + 256 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        *p_out.offset((n as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = accu;
        accu = ixheaac_mult16x16in32(
            *inp2.offset((n as core::ffi::c_int + 0 as core::ffi::c_int) as isize),
            *p_qmf2
                .offset(
                    (2 as core::ffi::c_int
                        * (n as core::ffi::c_int + 0 as core::ffi::c_int)) as isize,
                ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp2.offset((n as core::ffi::c_int + 64 as core::ffi::c_int) as isize),
                *p_qmf2
                    .offset(
                        (2 as core::ffi::c_int
                            * (n as core::ffi::c_int + 64 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp2.offset((n as core::ffi::c_int + 128 as core::ffi::c_int) as isize),
                *p_qmf2
                    .offset(
                        (2 as core::ffi::c_int
                            * (n as core::ffi::c_int + 128 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp2.offset((n as core::ffi::c_int + 192 as core::ffi::c_int) as isize),
                *p_qmf2
                    .offset(
                        (2 as core::ffi::c_int
                            * (n as core::ffi::c_int + 192 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp2.offset((n as core::ffi::c_int + 256 as core::ffi::c_int) as isize),
                *p_qmf2
                    .offset(
                        (2 as core::ffi::c_int
                            * (n as core::ffi::c_int + 256 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        *p_out.offset((n as core::ffi::c_int + 32 as core::ffi::c_int) as isize) = accu;
        accu = ixheaac_mult16x16in32(
            *inp2
                .offset(
                    (n as core::ffi::c_int + 1 as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ),
            *p_qmf2
                .offset(
                    (2 as core::ffi::c_int
                        * (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 0 as core::ffi::c_int)) as isize,
                ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp2
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 64 as core::ffi::c_int) as isize,
                    ),
                *p_qmf2
                    .offset(
                        (2 as core::ffi::c_int
                            * (n as core::ffi::c_int + 1 as core::ffi::c_int
                                + 64 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp2
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 128 as core::ffi::c_int) as isize,
                    ),
                *p_qmf2
                    .offset(
                        (2 as core::ffi::c_int
                            * (n as core::ffi::c_int + 1 as core::ffi::c_int
                                + 128 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp2
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 192 as core::ffi::c_int) as isize,
                    ),
                *p_qmf2
                    .offset(
                        (2 as core::ffi::c_int
                            * (n as core::ffi::c_int + 1 as core::ffi::c_int
                                + 192 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        accu = ixheaac_add32_sat(
            accu,
            ixheaac_mult16x16in32(
                *inp2
                    .offset(
                        (n as core::ffi::c_int + 1 as core::ffi::c_int
                            + 256 as core::ffi::c_int) as isize,
                    ),
                *p_qmf2
                    .offset(
                        (2 as core::ffi::c_int
                            * (n as core::ffi::c_int + 1 as core::ffi::c_int
                                + 256 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        *p_out
            .offset(
                (n as core::ffi::c_int + 1 as core::ffi::c_int + 32 as core::ffi::c_int)
                    as isize,
            ) = accu;
        n += 2 as core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_cplx_anal_qmffilt(
    mut time_sample_buf: *const WORD16,
    mut sbr_scale_factor: *mut ia_sbr_scale_fact_struct,
    mut qmf_real: *mut *mut WORD32,
    mut qmf_imag: *mut *mut WORD32,
    mut qmf_bank: *mut ia_sbr_qmf_filter_bank_struct,
    mut qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
    mut ch_fac: WORD32,
    mut low_pow_flag: WORD32,
    mut audio_object_type: WORD,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut num_time_slots: WORD32 = (*qmf_bank).num_time_slots as WORD32;
    let mut analysis_buffer: [WORD32; 128] = [0 as core::ffi::c_int; 128];
    let mut filter_states: *mut WORD16 = (*qmf_bank).core_samples_buffer;
    let mut fp1: *mut WORD16 = 0 as *mut WORD16;
    let mut fp2: *mut WORD16 = 0 as *mut WORD16;
    let mut tmp: *mut WORD16 = 0 as *mut WORD16;
    let mut filter_1: *mut WORD16 = 0 as *mut WORD16;
    let mut filter_2: *mut WORD16 = 0 as *mut WORD16;
    let mut filt_ptr: *mut WORD16 = 0 as *mut WORD16;
    let mut start_slot: WORD32 = 0 as WORD32;
    if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
        && audio_object_type != AOT_ER_AAC_LD as core::ffi::c_int
    {
        (*qmf_bank).filter_pos = ((*qmf_bank).filter_pos)
            .offset(
                ((*qmf_dec_tables_ptr).qmf_c)
                    .as_mut_ptr()
                    .offset_from((*qmf_bank).analy_win_coeff) as core::ffi::c_long
                    as isize,
            );
        (*qmf_bank).analy_win_coeff = ((*qmf_dec_tables_ptr).qmf_c).as_mut_ptr();
    } else {
        (*qmf_bank).filter_pos = ((*qmf_bank).filter_pos)
            .offset(
                ((*qmf_dec_tables_ptr).qmf_c_eld3)
                    .as_mut_ptr()
                    .offset_from((*qmf_bank).analy_win_coeff) as core::ffi::c_long
                    as isize,
            );
        (*qmf_bank).analy_win_coeff = ((*qmf_dec_tables_ptr).qmf_c_eld3).as_mut_ptr();
    }
    filter_1 = (*qmf_bank).filter_pos;
    if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
        && audio_object_type != AOT_ER_AAC_LD as core::ffi::c_int
    {
        filter_2 = filter_1.offset(64 as core::ffi::c_int as isize);
    } else {
        filter_2 = filter_1.offset((*qmf_bank).no_channels as isize);
    }
    (*sbr_scale_factor).st_lb_scale = 0 as WORD16;
    (*sbr_scale_factor).lb_scale = -(10 as core::ffi::c_int) as WORD16;
    if low_pow_flag == 0 {
        if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
            && audio_object_type != AOT_ER_AAC_LD as core::ffi::c_int
        {
            (*sbr_scale_factor).lb_scale = -(8 as core::ffi::c_int) as WORD16;
        } else {
            (*sbr_scale_factor).lb_scale = -(9 as core::ffi::c_int) as WORD16;
        }
        if (*qmf_bank).no_channels != 64 as core::ffi::c_int {
            (*qmf_bank).cos_twiddle = ((*qmf_dec_tables_ptr).sbr_sin_cos_twiddle_l32)
                .as_mut_ptr();
            (*qmf_bank).alt_sin_twiddle = ((*qmf_dec_tables_ptr).sbr_alt_sin_twiddle_l32)
                .as_mut_ptr();
        } else {
            (*qmf_bank).cos_twiddle = ((*qmf_dec_tables_ptr).sbr_sin_cos_twiddle_l64)
                .as_mut_ptr();
            (*qmf_bank).alt_sin_twiddle = ((*qmf_dec_tables_ptr).sbr_alt_sin_twiddle_l64)
                .as_mut_ptr();
        }
        if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
            && audio_object_type != AOT_ER_AAC_LD as core::ffi::c_int
        {
            (*qmf_bank).t_cos = ((*qmf_dec_tables_ptr).sbr_t_cos_sin_l32).as_mut_ptr();
        } else {
            (*qmf_bank).t_cos = ((*qmf_dec_tables_ptr).ixheaacd_sbr_t_cos_sin_l32_eld)
                .as_mut_ptr();
        }
    }
    fp1 = (*qmf_bank).anal_filter_states;
    fp2 = ((*qmf_bank).anal_filter_states).offset((*qmf_bank).no_channels as isize);
    if audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int
        || audio_object_type == AOT_ER_AAC_LD as core::ffi::c_int
    {
        filter_2 = (*qmf_bank).filter_2;
        fp1 = (*qmf_bank).fp1_anal;
        fp2 = (*qmf_bank).fp2_anal;
    }
    i = start_slot;
    while i < num_time_slots + start_slot {
        k = 0 as core::ffi::c_int as WORD32;
        while k < (*qmf_bank).no_channels {
            *filter_states
                .offset(((*qmf_bank).no_channels - 1 as WORD32 - k) as isize) = *time_sample_buf
                .offset((ch_fac * k) as isize);
            k += 1;
        }
        if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
            && audio_object_type != AOT_ER_AAC_LD as core::ffi::c_int
        {
            ixheaacd_sbr_qmfanal32_winadd(
                fp1,
                fp2,
                filter_1,
                filter_2,
                analysis_buffer.as_mut_ptr(),
            );
        } else {
            ixheaacd_sbr_qmfanal32_winadd_eld(
                fp1,
                fp2,
                filter_1,
                filter_2,
                analysis_buffer.as_mut_ptr(),
            );
        }
        time_sample_buf = time_sample_buf
            .offset(((*qmf_bank).no_channels * ch_fac) as isize);
        filter_states = filter_states.offset(-((*qmf_bank).no_channels as isize));
        if filter_states < (*qmf_bank).anal_filter_states {
            filter_states = ((*qmf_bank).anal_filter_states)
                .offset(
                    ((*qmf_bank).no_channels * 10 as WORD32 - (*qmf_bank).no_channels)
                        as isize,
                );
        }
        tmp = fp1;
        fp1 = fp2;
        fp2 = tmp;
        if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
            && audio_object_type != AOT_ER_AAC_LD as core::ffi::c_int
        {
            filter_1 = filter_1.offset(64 as core::ffi::c_int as isize);
            filter_2 = filter_2.offset(64 as core::ffi::c_int as isize);
        } else {
            filter_1 = filter_1.offset((*qmf_bank).no_channels as isize);
            filter_2 = filter_2.offset((*qmf_bank).no_channels as isize);
        }
        filt_ptr = filter_1;
        filter_1 = filter_2;
        filter_2 = filt_ptr;
        if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
            && audio_object_type != AOT_ER_AAC_LD as core::ffi::c_int
        {
            if filter_2
                > ((*qmf_bank).analy_win_coeff).offset(640 as core::ffi::c_int as isize)
                    as *mut WORD16
            {
                filter_1 = (*qmf_bank).analy_win_coeff as *mut WORD16;
                filter_2 = ((*qmf_bank).analy_win_coeff as *mut WORD16)
                    .offset(64 as core::ffi::c_int as isize);
            }
        } else if filter_2
            > ((*qmf_bank).analy_win_coeff)
                .offset(
                    ((*qmf_bank).no_channels as core::ffi::c_int
                        * 10 as core::ffi::c_int) as isize,
                ) as *mut WORD16
        {
            filter_1 = (*qmf_bank).analy_win_coeff as *mut WORD16;
            filter_2 = ((*qmf_bank).analy_win_coeff as *mut WORD16)
                .offset((*qmf_bank).no_channels as isize);
        }
        if low_pow_flag == 0 {
            ixheaacd_fwd_modulation(
                analysis_buffer.as_mut_ptr(),
                *qmf_real.offset(i as isize),
                *qmf_imag.offset(i as isize),
                qmf_bank,
                qmf_dec_tables_ptr,
                0 as WORD32,
            );
        } else {
            ixheaacd_dct3_32(
                analysis_buffer.as_mut_ptr(),
                *qmf_real.offset(i as isize),
                ((*qmf_dec_tables_ptr).dct23_tw).as_mut_ptr(),
                ((*qmf_dec_tables_ptr).post_fft_tbl).as_mut_ptr(),
                ((*qmf_dec_tables_ptr).w_16).as_mut_ptr(),
                ((*qmf_dec_tables_ptr).dig_rev_table4_16).as_mut_ptr(),
            );
        }
        i += 1;
    }
    (*qmf_bank).filter_pos = filter_1;
    (*qmf_bank).core_samples_buffer = filter_states;
    if audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int
        || audio_object_type == AOT_ER_AAC_LD as core::ffi::c_int
    {
        (*qmf_bank).fp1_anal = fp1;
        (*qmf_bank).fp2_anal = fp2;
        (*qmf_bank).filter_2 = filter_2;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_cplx_anal_qmffilt_32(
    mut time_sample_buf: *const WORD32,
    mut sbr_scale_factor: *mut ia_sbr_scale_fact_struct,
    mut qmf_real: *mut *mut WORD32,
    mut qmf_imag: *mut *mut WORD32,
    mut qmf_bank: *mut ia_sbr_qmf_filter_bank_struct,
    mut qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
    mut ch_fac: WORD32,
    mut ldsbr_present: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut num_time_slots: WORD32 = (*qmf_bank).num_time_slots as WORD32;
    let mut analysis_buffer: [WORD32; 128] = [0; 128];
    let mut filter_states: *mut WORD32 = (*qmf_bank).core_samples_buffer_32;
    let mut fp1: *mut WORD32 = 0 as *mut WORD32;
    let mut fp2: *mut WORD32 = 0 as *mut WORD32;
    let mut tmp: *mut WORD32 = 0 as *mut WORD32;
    let mut filter_1: *mut WORD32 = 0 as *mut WORD32;
    let mut filter_2: *mut WORD32 = 0 as *mut WORD32;
    let mut filt_ptr: *mut WORD32 = 0 as *mut WORD32;
    let mut start_slot: WORD32 = 2 as WORD32;
    if ldsbr_present != 0 {
        (*qmf_bank).filter_pos_32 = ((*qmf_bank).filter_pos_32)
            .offset(
                ((*qmf_dec_tables_ptr).qmf_c_ldsbr_mps)
                    .as_mut_ptr()
                    .offset_from((*qmf_bank).analy_win_coeff_32) as core::ffi::c_long
                    as isize,
            );
        (*qmf_bank).analy_win_coeff_32 = ((*qmf_dec_tables_ptr).qmf_c_ldsbr_mps)
            .as_mut_ptr();
    } else {
        (*qmf_bank).filter_pos_32 = ((*qmf_bank).filter_pos_32)
            .offset(
                ixheaacd_ldmps_polyphase_filter_coeff_fix
                    .as_ptr()
                    .offset_from((*qmf_bank).analy_win_coeff_32) as core::ffi::c_long
                    as isize,
            );
        (*qmf_bank).analy_win_coeff_32 = ixheaacd_ldmps_polyphase_filter_coeff_fix
            .as_ptr() as *mut WORD32;
    }
    filter_1 = (*qmf_bank).filter_pos_32;
    filter_2 = filter_1.offset((*qmf_bank).no_channels as isize);
    (*sbr_scale_factor).st_lb_scale = 0 as WORD16;
    (*sbr_scale_factor).lb_scale = -(10 as core::ffi::c_int) as WORD16;
    (*sbr_scale_factor).lb_scale = -(9 as core::ffi::c_int) as WORD16;
    if (*qmf_bank).no_channels != 64 as core::ffi::c_int {
        (*qmf_bank).cos_twiddle = ((*qmf_dec_tables_ptr).sbr_sin_cos_twiddle_l32)
            .as_mut_ptr();
        (*qmf_bank).alt_sin_twiddle = ((*qmf_dec_tables_ptr).sbr_alt_sin_twiddle_l32)
            .as_mut_ptr();
    } else {
        (*qmf_bank).cos_twiddle = ((*qmf_dec_tables_ptr).sbr_sin_cos_twiddle_l64)
            .as_mut_ptr();
        (*qmf_bank).alt_sin_twiddle = ((*qmf_dec_tables_ptr).sbr_alt_sin_twiddle_l64)
            .as_mut_ptr();
    }
    (*qmf_bank).t_cos = ((*qmf_dec_tables_ptr).ixheaacd_sbr_t_cos_sin_l32_eld)
        .as_mut_ptr();
    fp1 = (*qmf_bank).anal_filter_states_32;
    fp2 = ((*qmf_bank).anal_filter_states_32).offset((*qmf_bank).no_channels as isize);
    filter_2 = (*qmf_bank).filter_2_32;
    fp1 = (*qmf_bank).fp1_anal_32;
    fp2 = (*qmf_bank).fp2_anal_32;
    i = start_slot;
    while i < num_time_slots + start_slot {
        k = 0 as core::ffi::c_int as WORD32;
        while k < (*qmf_bank).no_channels {
            *filter_states
                .offset(((*qmf_bank).no_channels - 1 as WORD32 - k) as isize) = *time_sample_buf
                .offset((ch_fac * k) as isize);
            k += 1;
        }
        if ldsbr_present != 0 {
            ixheaacd_sbr_qmfanal32_winadd_eld_32(
                fp1,
                fp2,
                filter_1,
                filter_2,
                analysis_buffer.as_mut_ptr(),
            );
        } else {
            ixheaacd_sbr_qmfanal32_winadd_eld_mps(
                fp1,
                fp2,
                filter_1,
                filter_2,
                analysis_buffer.as_mut_ptr(),
            );
        }
        time_sample_buf = time_sample_buf
            .offset(((*qmf_bank).no_channels * ch_fac) as isize);
        filter_states = filter_states.offset(-((*qmf_bank).no_channels as isize));
        if filter_states < (*qmf_bank).anal_filter_states_32 {
            filter_states = ((*qmf_bank).anal_filter_states_32)
                .offset(
                    ((*qmf_bank).no_channels * 10 as WORD32 - (*qmf_bank).no_channels)
                        as isize,
                );
        }
        tmp = fp1;
        fp1 = fp2;
        fp2 = tmp;
        filter_1 = filter_1.offset((*qmf_bank).no_channels as isize);
        filter_2 = filter_2.offset((*qmf_bank).no_channels as isize);
        filt_ptr = filter_1;
        filter_1 = filter_2;
        filter_2 = filt_ptr;
        if filter_2
            > ((*qmf_bank).analy_win_coeff_32)
                .offset(
                    ((*qmf_bank).no_channels as core::ffi::c_int
                        * 10 as core::ffi::c_int) as isize,
                )
        {
            filter_1 = (*qmf_bank).analy_win_coeff_32;
            filter_2 = ((*qmf_bank).analy_win_coeff_32)
                .offset((*qmf_bank).no_channels as isize);
        }
        ixheaacd_fwd_modulation(
            analysis_buffer.as_mut_ptr(),
            *qmf_real.offset(i as isize),
            *qmf_imag.offset(i as isize),
            qmf_bank,
            qmf_dec_tables_ptr,
            1 as WORD32,
        );
        i += 1;
    }
    (*qmf_bank).filter_pos_32 = filter_1;
    (*qmf_bank).core_samples_buffer_32 = filter_states;
    (*qmf_bank).fp1_anal_32 = fp1;
    (*qmf_bank).fp2_anal_32 = fp2;
    (*qmf_bank).filter_2_32 = filter_2;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_inv_modulation_lp(
    mut qmf_real: *mut WORD32,
    mut filter_states: *mut WORD16,
    mut syn_qmf: *mut ia_sbr_qmf_filter_bank_struct,
    mut qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
) -> VOID {
    let mut L: WORD32 = (*syn_qmf).no_channels;
    let M: WORD32 = L >> 1 as core::ffi::c_int;
    let mut dct_in: *mut WORD32 = qmf_real;
    let mut time_out: [WORD32; 128] = [0; 128];
    let mut ptime_out: *mut WORD32 = &mut *time_out
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
    if L == 64 as core::ffi::c_int {
        ixheaacd_dct2_64(
            dct_in,
            ptime_out,
            qmf_dec_tables_ptr,
            filter_states.offset(M as isize),
        );
    } else {
        ixheaacd_dct2_32(
            dct_in,
            time_out.as_mut_ptr(),
            qmf_dec_tables_ptr,
            filter_states,
        );
    }
    *filter_states.offset((3 as WORD32 * M) as isize) = 0 as WORD16;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_inv_emodulation(
    mut qmf_real: *mut WORD32,
    mut syn_qmf: *mut ia_sbr_qmf_filter_bank_struct,
    mut qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
) -> VOID {
    if (*syn_qmf).no_channels == 64 as core::ffi::c_int {
        ixheaacd_cos_sin_mod(
            qmf_real,
            syn_qmf,
            ((*qmf_dec_tables_ptr).w_32).as_mut_ptr(),
            ((*qmf_dec_tables_ptr).dig_rev_table2_32).as_mut_ptr(),
        );
    } else {
        ixheaacd_cos_sin_mod(
            qmf_real,
            syn_qmf,
            ((*qmf_dec_tables_ptr).w_16).as_mut_ptr(),
            ((*qmf_dec_tables_ptr).dig_rev_table4_16).as_mut_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_esbr_radix4bfly(
    mut w: *const WORD32,
    mut x: *mut WORD32,
    mut index1: WORD32,
    mut index: WORD32,
) -> VOID {
    let mut i: core::ffi::c_int = 0;
    let mut l1: WORD32 = 0;
    let mut l2: WORD32 = 0;
    let mut h2: WORD32 = 0;
    let mut fft_jmp: WORD32 = 0;
    let mut xt0_0: WORD32 = 0;
    let mut yt0_0: WORD32 = 0;
    let mut xt1_0: WORD32 = 0;
    let mut yt1_0: WORD32 = 0;
    let mut xt2_0: WORD32 = 0;
    let mut yt2_0: WORD32 = 0;
    let mut xh0_0: WORD32 = 0;
    let mut xh1_0: WORD32 = 0;
    let mut xh20_0: WORD32 = 0;
    let mut xh21_0: WORD32 = 0;
    let mut xl0_0: WORD32 = 0;
    let mut xl1_0: WORD32 = 0;
    let mut xl20_0: WORD32 = 0;
    let mut xl21_0: WORD32 = 0;
    let mut x_0: WORD32 = 0;
    let mut x_1: WORD32 = 0;
    let mut x_l1_0: WORD32 = 0;
    let mut x_l1_1: WORD32 = 0;
    let mut x_l2_0: WORD32 = 0;
    let mut x_l2_1: WORD32 = 0;
    let mut x_h2_0: WORD32 = 0;
    let mut x_h2_1: WORD32 = 0;
    let mut si10: WORD32 = 0;
    let mut si20: WORD32 = 0;
    let mut si30: WORD32 = 0;
    let mut co10: WORD32 = 0;
    let mut co20: WORD32 = 0;
    let mut co30: WORD32 = 0;
    let mut mul_1: WORD64 = 0;
    let mut mul_2: WORD64 = 0;
    let mut mul_3: WORD64 = 0;
    let mut mul_4: WORD64 = 0;
    let mut mul_5: WORD64 = 0;
    let mut mul_6: WORD64 = 0;
    let mut mul_7: WORD64 = 0;
    let mut mul_8: WORD64 = 0;
    let mut mul_9: WORD64 = 0;
    let mut mul_10: WORD64 = 0;
    let mut mul_11: WORD64 = 0;
    let mut mul_12: WORD64 = 0;
    let mut w_ptr: *const WORD32 = w;
    let mut i1: WORD32 = 0;
    h2 = index << 1 as core::ffi::c_int;
    l1 = index << 2 as core::ffi::c_int;
    l2 = (index << 2 as core::ffi::c_int) + (index << 1 as core::ffi::c_int);
    fft_jmp = 6 as WORD32 * index;
    i1 = 0 as core::ffi::c_int as WORD32;
    while i1 < index1 {
        i = 0 as core::ffi::c_int;
        while i < index {
            let fresh220 = w_ptr;
            w_ptr = w_ptr.offset(1);
            si10 = *fresh220;
            let fresh221 = w_ptr;
            w_ptr = w_ptr.offset(1);
            co10 = *fresh221;
            let fresh222 = w_ptr;
            w_ptr = w_ptr.offset(1);
            si20 = *fresh222;
            let fresh223 = w_ptr;
            w_ptr = w_ptr.offset(1);
            co20 = *fresh223;
            let fresh224 = w_ptr;
            w_ptr = w_ptr.offset(1);
            si30 = *fresh224;
            let fresh225 = w_ptr;
            w_ptr = w_ptr.offset(1);
            co30 = *fresh225;
            x_0 = *x.offset(0 as core::ffi::c_int as isize);
            x_h2_0 = *x.offset(h2 as isize);
            x_l1_0 = *x.offset(l1 as isize);
            x_l2_0 = *x.offset(l2 as isize);
            xh0_0 = ixheaac_add32_sat(x_0, x_l1_0);
            xl0_0 = ixheaac_sub32_sat(x_0, x_l1_0);
            xh20_0 = ixheaac_add32_sat(x_h2_0, x_l2_0);
            xl20_0 = ixheaac_sub32_sat(x_h2_0, x_l2_0);
            *x.offset(0 as core::ffi::c_int as isize) = ixheaac_add32_sat(xh0_0, xh20_0);
            xt0_0 = ixheaac_sub32_sat(xh0_0, xh20_0);
            x_1 = *x.offset(1 as core::ffi::c_int as isize);
            x_h2_1 = *x
                .offset((h2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
            x_l1_1 = *x
                .offset((l1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
            x_l2_1 = *x
                .offset((l2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
            xh1_0 = ixheaac_add32_sat(x_1, x_l1_1);
            xl1_0 = ixheaac_sub32_sat(x_1, x_l1_1);
            xh21_0 = ixheaac_add32_sat(x_h2_1, x_l2_1);
            xl21_0 = ixheaac_sub32_sat(x_h2_1, x_l2_1);
            *x.offset(1 as core::ffi::c_int as isize) = ixheaac_add32_sat(xh1_0, xh21_0);
            yt0_0 = ixheaac_sub32_sat(xh1_0, xh21_0);
            xt1_0 = ixheaac_add32_sat(xl0_0, xl21_0);
            xt2_0 = ixheaac_sub32_sat(xl0_0, xl21_0);
            yt2_0 = ixheaac_add32_sat(xl1_0, xl20_0);
            yt1_0 = ixheaac_sub32_sat(xl1_0, xl20_0);
            mul_11 = ixheaac_mult64(xt2_0, co30);
            mul_3 = ixheaac_mult64(yt2_0, si30);
            *x.offset(l2 as isize) = ((mul_3 + mul_11 >> 32 as core::ffi::c_int)
                as WORD32) << RADIXSHIFT;
            mul_5 = ixheaac_mult64(xt2_0, si30);
            mul_9 = ixheaac_mult64(yt2_0, co30);
            *x.offset((l2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = ((mul_9
                - mul_5 >> 32 as core::ffi::c_int) as WORD32) << RADIXSHIFT;
            mul_12 = ixheaac_mult64(xt0_0, co20);
            mul_2 = ixheaac_mult64(yt0_0, si20);
            *x.offset(l1 as isize) = ((mul_2 + mul_12 >> 32 as core::ffi::c_int)
                as WORD32) << RADIXSHIFT;
            mul_6 = ixheaac_mult64(xt0_0, si20);
            mul_8 = ixheaac_mult64(yt0_0, co20);
            *x.offset((l1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = ((mul_8
                - mul_6 >> 32 as core::ffi::c_int) as WORD32) << RADIXSHIFT;
            mul_4 = ixheaac_mult64(xt1_0, co10);
            mul_1 = ixheaac_mult64(yt1_0, si10);
            *x.offset(h2 as isize) = ((mul_1 + mul_4 >> 32 as core::ffi::c_int)
                as WORD32) << RADIXSHIFT;
            mul_10 = ixheaac_mult64(xt1_0, si10);
            mul_7 = ixheaac_mult64(yt1_0, co10);
            *x.offset((h2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = ((mul_7
                - mul_10 >> 32 as core::ffi::c_int) as WORD32) << RADIXSHIFT;
            x = x.offset(2 as core::ffi::c_int as isize);
            i += 1;
        }
        x = x.offset(fft_jmp as isize);
        w_ptr = w_ptr.offset(-(fft_jmp as isize));
        i1 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_esbr_postradixcompute2(
    mut ptr_y: *mut WORD32,
    mut ptr_x: *mut WORD32,
    mut pdig_rev_tbl: *const WORD32,
    mut npoints: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut h2: WORD32 = 0;
    let mut x_0: WORD32 = 0;
    let mut x_1: WORD32 = 0;
    let mut x_2: WORD32 = 0;
    let mut x_3: WORD32 = 0;
    let mut x_4: WORD32 = 0;
    let mut x_5: WORD32 = 0;
    let mut x_6: WORD32 = 0;
    let mut x_7: WORD32 = 0;
    let mut x_8: WORD32 = 0;
    let mut x_9: WORD32 = 0;
    let mut x_a: WORD32 = 0;
    let mut x_b: WORD32 = 0;
    let mut x_c: WORD32 = 0;
    let mut x_d: WORD32 = 0;
    let mut x_e: WORD32 = 0;
    let mut x_f: WORD32 = 0;
    let mut n00: WORD32 = 0;
    let mut n10: WORD32 = 0;
    let mut n20: WORD32 = 0;
    let mut n30: WORD32 = 0;
    let mut n01: WORD32 = 0;
    let mut n11: WORD32 = 0;
    let mut n21: WORD32 = 0;
    let mut n31: WORD32 = 0;
    let mut n02: WORD32 = 0;
    let mut n12: WORD32 = 0;
    let mut n22: WORD32 = 0;
    let mut n32: WORD32 = 0;
    let mut n03: WORD32 = 0;
    let mut n13: WORD32 = 0;
    let mut n23: WORD32 = 0;
    let mut n33: WORD32 = 0;
    let mut x2: *mut WORD32 = 0 as *mut WORD32;
    let mut x0: *mut WORD32 = 0 as *mut WORD32;
    let mut y0: *mut WORD32 = 0 as *mut WORD32;
    let mut y1: *mut WORD32 = 0 as *mut WORD32;
    let mut y2: *mut WORD32 = 0 as *mut WORD32;
    let mut y3: *mut WORD32 = 0 as *mut WORD32;
    y0 = ptr_y;
    y2 = ptr_y.offset(npoints as isize);
    x0 = ptr_x;
    x2 = ptr_x.offset((npoints >> 1 as core::ffi::c_int) as isize);
    y1 = y0.offset((npoints >> 2 as core::ffi::c_int) as isize);
    y3 = y2.offset((npoints >> 2 as core::ffi::c_int) as isize);
    k = 0 as core::ffi::c_int as WORD32;
    while k < 2 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < npoints >> 1 as core::ffi::c_int {
            let fresh226 = pdig_rev_tbl;
            pdig_rev_tbl = pdig_rev_tbl.offset(1);
            h2 = *fresh226 >> 2 as core::ffi::c_int;
            let fresh227 = x0;
            x0 = x0.offset(1);
            x_0 = *fresh227;
            let fresh228 = x0;
            x0 = x0.offset(1);
            x_1 = *fresh228;
            let fresh229 = x0;
            x0 = x0.offset(1);
            x_2 = *fresh229;
            let fresh230 = x0;
            x0 = x0.offset(1);
            x_3 = *fresh230;
            let fresh231 = x0;
            x0 = x0.offset(1);
            x_4 = *fresh231;
            let fresh232 = x0;
            x0 = x0.offset(1);
            x_5 = *fresh232;
            let fresh233 = x0;
            x0 = x0.offset(1);
            x_6 = *fresh233;
            let fresh234 = x0;
            x0 = x0.offset(1);
            x_7 = *fresh234;
            n00 = ixheaac_add32_sat(x_0, x_2);
            n01 = ixheaac_add32_sat(x_1, x_3);
            n20 = ixheaac_sub32_sat(x_0, x_2);
            n21 = ixheaac_sub32_sat(x_1, x_3);
            n10 = ixheaac_add32_sat(x_4, x_6);
            n11 = ixheaac_add32_sat(x_5, x_7);
            n30 = ixheaac_sub32_sat(x_4, x_6);
            n31 = ixheaac_sub32_sat(x_5, x_7);
            *y0.offset(h2 as isize) = n00;
            *y0.offset((h2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = n01;
            *y1.offset(h2 as isize) = n10;
            *y1.offset((h2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = n11;
            *y2.offset(h2 as isize) = n20;
            *y2.offset((h2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = n21;
            *y3.offset(h2 as isize) = n30;
            *y3.offset((h2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = n31;
            let fresh235 = x2;
            x2 = x2.offset(1);
            x_8 = *fresh235;
            let fresh236 = x2;
            x2 = x2.offset(1);
            x_9 = *fresh236;
            let fresh237 = x2;
            x2 = x2.offset(1);
            x_a = *fresh237;
            let fresh238 = x2;
            x2 = x2.offset(1);
            x_b = *fresh238;
            let fresh239 = x2;
            x2 = x2.offset(1);
            x_c = *fresh239;
            let fresh240 = x2;
            x2 = x2.offset(1);
            x_d = *fresh240;
            let fresh241 = x2;
            x2 = x2.offset(1);
            x_e = *fresh241;
            let fresh242 = x2;
            x2 = x2.offset(1);
            x_f = *fresh242;
            n02 = ixheaac_add32_sat(x_8, x_a);
            n03 = ixheaac_add32_sat(x_9, x_b);
            n22 = ixheaac_sub32_sat(x_8, x_a);
            n23 = ixheaac_sub32_sat(x_9, x_b);
            n12 = ixheaac_add32_sat(x_c, x_e);
            n13 = ixheaac_add32_sat(x_d, x_f);
            n32 = ixheaac_sub32_sat(x_c, x_e);
            n33 = ixheaac_sub32_sat(x_d, x_f);
            *y0.offset((h2 as core::ffi::c_int + 2 as core::ffi::c_int) as isize) = n02;
            *y0.offset((h2 as core::ffi::c_int + 3 as core::ffi::c_int) as isize) = n03;
            *y1.offset((h2 as core::ffi::c_int + 2 as core::ffi::c_int) as isize) = n12;
            *y1.offset((h2 as core::ffi::c_int + 3 as core::ffi::c_int) as isize) = n13;
            *y2.offset((h2 as core::ffi::c_int + 2 as core::ffi::c_int) as isize) = n22;
            *y2.offset((h2 as core::ffi::c_int + 3 as core::ffi::c_int) as isize) = n23;
            *y3.offset((h2 as core::ffi::c_int + 2 as core::ffi::c_int) as isize) = n32;
            *y3.offset((h2 as core::ffi::c_int + 3 as core::ffi::c_int) as isize) = n33;
            i += 8 as core::ffi::c_int;
        }
        x0 = x0.offset((npoints >> 1 as core::ffi::c_int) as isize);
        x2 = x2.offset((npoints >> 1 as core::ffi::c_int) as isize);
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_esbr_postradixcompute4(
    mut ptr_y: *mut WORD32,
    mut ptr_x: *mut WORD32,
    mut p_dig_rev_tbl: *const WORD32,
    mut npoints: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut h2: WORD32 = 0;
    let mut xh0_0: WORD32 = 0;
    let mut xh1_0: WORD32 = 0;
    let mut xl0_0: WORD32 = 0;
    let mut xl1_0: WORD32 = 0;
    let mut xh0_1: WORD32 = 0;
    let mut xh1_1: WORD32 = 0;
    let mut xl0_1: WORD32 = 0;
    let mut xl1_1: WORD32 = 0;
    let mut x_0: WORD32 = 0;
    let mut x_1: WORD32 = 0;
    let mut x_2: WORD32 = 0;
    let mut x_3: WORD32 = 0;
    let mut xh0_2: WORD32 = 0;
    let mut xh1_2: WORD32 = 0;
    let mut xl0_2: WORD32 = 0;
    let mut xl1_2: WORD32 = 0;
    let mut xh0_3: WORD32 = 0;
    let mut xh1_3: WORD32 = 0;
    let mut xl0_3: WORD32 = 0;
    let mut xl1_3: WORD32 = 0;
    let mut x_4: WORD32 = 0;
    let mut x_5: WORD32 = 0;
    let mut x_6: WORD32 = 0;
    let mut x_7: WORD32 = 0;
    let mut x_8: WORD32 = 0;
    let mut x_9: WORD32 = 0;
    let mut x_a: WORD32 = 0;
    let mut x_b: WORD32 = 0;
    let mut x_c: WORD32 = 0;
    let mut x_d: WORD32 = 0;
    let mut x_e: WORD32 = 0;
    let mut x_f: WORD32 = 0;
    let mut n00: WORD32 = 0;
    let mut n10: WORD32 = 0;
    let mut n20: WORD32 = 0;
    let mut n30: WORD32 = 0;
    let mut n01: WORD32 = 0;
    let mut n11: WORD32 = 0;
    let mut n21: WORD32 = 0;
    let mut n31: WORD32 = 0;
    let mut n02: WORD32 = 0;
    let mut n12: WORD32 = 0;
    let mut n22: WORD32 = 0;
    let mut n32: WORD32 = 0;
    let mut n03: WORD32 = 0;
    let mut n13: WORD32 = 0;
    let mut n23: WORD32 = 0;
    let mut n33: WORD32 = 0;
    let mut x2: *mut WORD32 = 0 as *mut WORD32;
    let mut x0: *mut WORD32 = 0 as *mut WORD32;
    let mut y0: *mut WORD32 = 0 as *mut WORD32;
    let mut y1: *mut WORD32 = 0 as *mut WORD32;
    let mut y2: *mut WORD32 = 0 as *mut WORD32;
    let mut y3: *mut WORD32 = 0 as *mut WORD32;
    y0 = ptr_y;
    y2 = ptr_y.offset(npoints as isize);
    x0 = ptr_x;
    x2 = ptr_x.offset((npoints >> 1 as core::ffi::c_int) as isize);
    y1 = y0.offset((npoints >> 1 as core::ffi::c_int) as isize);
    y3 = y2.offset((npoints >> 1 as core::ffi::c_int) as isize);
    k = 0 as core::ffi::c_int as WORD32;
    while k < 2 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < npoints >> 1 as core::ffi::c_int {
            let fresh203 = p_dig_rev_tbl;
            p_dig_rev_tbl = p_dig_rev_tbl.offset(1);
            h2 = *fresh203 >> 2 as core::ffi::c_int;
            let fresh204 = x0;
            x0 = x0.offset(1);
            x_0 = *fresh204;
            let fresh205 = x0;
            x0 = x0.offset(1);
            x_1 = *fresh205;
            let fresh206 = x0;
            x0 = x0.offset(1);
            x_2 = *fresh206;
            let fresh207 = x0;
            x0 = x0.offset(1);
            x_3 = *fresh207;
            let fresh208 = x0;
            x0 = x0.offset(1);
            x_4 = *fresh208;
            let fresh209 = x0;
            x0 = x0.offset(1);
            x_5 = *fresh209;
            let fresh210 = x0;
            x0 = x0.offset(1);
            x_6 = *fresh210;
            let fresh211 = x0;
            x0 = x0.offset(1);
            x_7 = *fresh211;
            xh0_0 = ixheaac_add32_sat(x_0, x_4);
            xh1_0 = ixheaac_add32_sat(x_1, x_5);
            xl0_0 = ixheaac_sub32_sat(x_0, x_4);
            xl1_0 = ixheaac_sub32_sat(x_1, x_5);
            xh0_1 = ixheaac_add32_sat(x_2, x_6);
            xh1_1 = ixheaac_add32_sat(x_3, x_7);
            xl0_1 = ixheaac_sub32_sat(x_2, x_6);
            xl1_1 = ixheaac_sub32_sat(x_3, x_7);
            n00 = ixheaac_add32_sat(xh0_0, xh0_1);
            n01 = ixheaac_add32_sat(xh1_0, xh1_1);
            n10 = ixheaac_add32_sat(xl0_0, xl1_1);
            n11 = ixheaac_sub32_sat(xl1_0, xl0_1);
            n20 = ixheaac_sub32_sat(xh0_0, xh0_1);
            n21 = ixheaac_sub32_sat(xh1_0, xh1_1);
            n30 = ixheaac_sub32_sat(xl0_0, xl1_1);
            n31 = ixheaac_add32_sat(xl1_0, xl0_1);
            *y0.offset(h2 as isize) = n00;
            *y0.offset((h2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = n01;
            *y1.offset(h2 as isize) = n10;
            *y1.offset((h2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = n11;
            *y2.offset(h2 as isize) = n20;
            *y2.offset((h2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = n21;
            *y3.offset(h2 as isize) = n30;
            *y3.offset((h2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = n31;
            let fresh212 = x2;
            x2 = x2.offset(1);
            x_8 = *fresh212;
            let fresh213 = x2;
            x2 = x2.offset(1);
            x_9 = *fresh213;
            let fresh214 = x2;
            x2 = x2.offset(1);
            x_a = *fresh214;
            let fresh215 = x2;
            x2 = x2.offset(1);
            x_b = *fresh215;
            let fresh216 = x2;
            x2 = x2.offset(1);
            x_c = *fresh216;
            let fresh217 = x2;
            x2 = x2.offset(1);
            x_d = *fresh217;
            let fresh218 = x2;
            x2 = x2.offset(1);
            x_e = *fresh218;
            let fresh219 = x2;
            x2 = x2.offset(1);
            x_f = *fresh219;
            xh0_2 = ixheaac_add32_sat(x_8, x_c);
            xh1_2 = ixheaac_add32_sat(x_9, x_d);
            xl0_2 = ixheaac_sub32_sat(x_8, x_c);
            xl1_2 = ixheaac_sub32_sat(x_9, x_d);
            xh0_3 = ixheaac_add32_sat(x_a, x_e);
            xh1_3 = ixheaac_add32_sat(x_b, x_f);
            xl0_3 = ixheaac_sub32_sat(x_a, x_e);
            xl1_3 = ixheaac_sub32_sat(x_b, x_f);
            n02 = ixheaac_add32_sat(xh0_2, xh0_3);
            n03 = ixheaac_add32_sat(xh1_2, xh1_3);
            n12 = ixheaac_add32_sat(xl0_2, xl1_3);
            n13 = ixheaac_sub32_sat(xl1_2, xl0_3);
            n22 = ixheaac_sub32_sat(xh0_2, xh0_3);
            n23 = ixheaac_sub32_sat(xh1_2, xh1_3);
            n32 = ixheaac_sub32_sat(xl0_2, xl1_3);
            n33 = ixheaac_add32_sat(xl1_2, xl0_3);
            *y0.offset((h2 as core::ffi::c_int + 2 as core::ffi::c_int) as isize) = n02;
            *y0.offset((h2 as core::ffi::c_int + 3 as core::ffi::c_int) as isize) = n03;
            *y1.offset((h2 as core::ffi::c_int + 2 as core::ffi::c_int) as isize) = n12;
            *y1.offset((h2 as core::ffi::c_int + 3 as core::ffi::c_int) as isize) = n13;
            *y2.offset((h2 as core::ffi::c_int + 2 as core::ffi::c_int) as isize) = n22;
            *y2.offset((h2 as core::ffi::c_int + 3 as core::ffi::c_int) as isize) = n23;
            *y3.offset((h2 as core::ffi::c_int + 2 as core::ffi::c_int) as isize) = n32;
            *y3.offset((h2 as core::ffi::c_int + 3 as core::ffi::c_int) as isize) = n33;
            i += 8 as core::ffi::c_int;
        }
        x0 = x0.offset((npoints >> 1 as core::ffi::c_int) as isize);
        x2 = x2.offset((npoints >> 1 as core::ffi::c_int) as isize);
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_esbr_cos_sin_mod(
    mut subband: *mut WORD32,
    mut qmf_bank: *mut ia_sbr_qmf_filter_bank_struct,
    mut p_twiddle: *mut WORD32,
    mut p_dig_rev_tbl: *mut WORD32,
) -> VOID {
    let mut z: WORD32 = 0;
    let mut temp: [WORD32; 128] = [0; 128];
    let mut scaleshift: WORD32 = 0 as WORD32;
    let mut re2: WORD32 = 0;
    let mut re3: WORD32 = 0;
    let mut wim: WORD32 = 0;
    let mut wre: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut M_2: WORD32 = 0;
    let mut M: WORD32 = ixheaac_shr32((*qmf_bank).no_channels, 1 as WORD);
    let mut p_sin: *const WORD32 = 0 as *const WORD32;
    let mut p_sin_cos: *const WORD32 = 0 as *const WORD32;
    let mut subband_tmp: [WORD32; 128] = [0; 128];
    let mut re: WORD32 = 0;
    let mut im: WORD32 = 0;
    let mut psubband: *mut WORD32 = 0 as *mut WORD32;
    let mut psubband1: *mut WORD32 = 0 as *mut WORD32;
    let mut psubband_t: *mut WORD32 = 0 as *mut WORD32;
    let mut psubband1_t: *mut WORD32 = 0 as *mut WORD32;
    let mut psubband2: *mut WORD32 = 0 as *mut WORD32;
    let mut psubband12: *mut WORD32 = 0 as *mut WORD32;
    let mut psubband_t2: *mut WORD32 = 0 as *mut WORD32;
    let mut psubband1_t2: *mut WORD32 = 0 as *mut WORD32;
    M_2 = ixheaac_shr32(M, 1 as WORD);
    p_sin_cos = (*qmf_bank).esbr_cos_twiddle;
    psubband = &mut *subband.offset(0 as core::ffi::c_int as isize) as *mut WORD32;
    psubband1 = &mut *subband
        .offset(
            (2 as core::ffi::c_int * M as core::ffi::c_int - 1 as core::ffi::c_int)
                as isize,
        ) as *mut WORD32;
    psubband_t = subband_tmp.as_mut_ptr();
    psubband1_t = &mut *subband_tmp
        .as_mut_ptr()
        .offset(
            (2 as core::ffi::c_int * M as core::ffi::c_int - 1 as core::ffi::c_int)
                as isize,
        ) as *mut WORD32;
    psubband2 = &mut *subband.offset(64 as core::ffi::c_int as isize) as *mut WORD32;
    psubband12 = &mut *subband
        .offset(
            (2 as core::ffi::c_int * M as core::ffi::c_int - 1 as core::ffi::c_int
                + 64 as core::ffi::c_int) as isize,
        ) as *mut WORD32;
    psubband_t2 = &mut *subband_tmp.as_mut_ptr().offset(64 as core::ffi::c_int as isize)
        as *mut WORD32;
    psubband1_t2 = &mut *subband_tmp
        .as_mut_ptr()
        .offset(
            (2 as core::ffi::c_int * M as core::ffi::c_int - 1 as core::ffi::c_int
                + 64 as core::ffi::c_int) as isize,
        ) as *mut WORD32;
    i = ((M_2 as core::ffi::c_int >> 1 as core::ffi::c_int) - 1 as core::ffi::c_int)
        as WORD32;
    while i >= 0 as core::ffi::c_int {
        let fresh146 = psubband;
        psubband = psubband.offset(1);
        re = *fresh146;
        let fresh147 = psubband1;
        psubband1 = psubband1.offset(-1);
        im = *fresh147;
        let fresh148 = p_sin_cos;
        p_sin_cos = p_sin_cos.offset(1);
        wim = *fresh148;
        let fresh149 = p_sin_cos;
        p_sin_cos = p_sin_cos.offset(1);
        wre = *fresh149;
        let fresh150 = psubband_t;
        psubband_t = psubband_t.offset(1);
        *fresh150 = (ixheaac_add64(ixheaac_mult64(re, wre), ixheaac_mult64(im, wim))
            >> 32 as core::ffi::c_int) as WORD32;
        let fresh151 = psubband_t;
        psubband_t = psubband_t.offset(1);
        *fresh151 = (ixheaac_sub64_sat(ixheaac_mult64(im, wre), ixheaac_mult64(re, wim))
            >> 32 as core::ffi::c_int) as WORD32;
        let fresh152 = psubband2;
        psubband2 = psubband2.offset(1);
        re = *fresh152;
        let fresh153 = psubband12;
        psubband12 = psubband12.offset(-1);
        im = *fresh153;
        let fresh154 = psubband_t2;
        psubband_t2 = psubband_t2.offset(1);
        *fresh154 = (ixheaac_sub64_sat(ixheaac_mult64(im, wim), ixheaac_mult64(re, wre))
            >> 32 as core::ffi::c_int) as WORD32;
        let fresh155 = psubband_t2;
        psubband_t2 = psubband_t2.offset(1);
        *fresh155 = (ixheaac_add64(ixheaac_mult64(re, wim), ixheaac_mult64(im, wre))
            >> 32 as core::ffi::c_int) as WORD32;
        let fresh156 = psubband1;
        psubband1 = psubband1.offset(-1);
        re = *fresh156;
        let fresh157 = psubband;
        psubband = psubband.offset(1);
        im = *fresh157;
        let fresh158 = p_sin_cos;
        p_sin_cos = p_sin_cos.offset(1);
        wim = *fresh158;
        let fresh159 = p_sin_cos;
        p_sin_cos = p_sin_cos.offset(1);
        wre = *fresh159;
        let fresh160 = psubband1_t;
        psubband1_t = psubband1_t.offset(-1);
        *fresh160 = (ixheaac_sub64_sat(ixheaac_mult64(im, wre), ixheaac_mult64(re, wim))
            >> 32 as core::ffi::c_int) as WORD32;
        let fresh161 = psubband1_t;
        psubband1_t = psubband1_t.offset(-1);
        *fresh161 = (ixheaac_add64(ixheaac_mult64(re, wre), ixheaac_mult64(im, wim))
            >> 32 as core::ffi::c_int) as WORD32;
        let fresh162 = psubband12;
        psubband12 = psubband12.offset(-1);
        re = *fresh162;
        let fresh163 = psubband2;
        psubband2 = psubband2.offset(1);
        im = *fresh163;
        let fresh164 = psubband1_t2;
        psubband1_t2 = psubband1_t2.offset(-1);
        *fresh164 = (ixheaac_add64(ixheaac_mult64(re, wim), ixheaac_mult64(im, wre))
            >> 32 as core::ffi::c_int) as WORD32;
        let fresh165 = psubband1_t2;
        psubband1_t2 = psubband1_t2.offset(-1);
        *fresh165 = (ixheaac_sub64_sat(ixheaac_mult64(im, wim), ixheaac_mult64(re, wre))
            >> 32 as core::ffi::c_int) as WORD32;
        let fresh166 = psubband;
        psubband = psubband.offset(1);
        re = *fresh166;
        let fresh167 = psubband1;
        psubband1 = psubband1.offset(-1);
        im = *fresh167;
        let fresh168 = p_sin_cos;
        p_sin_cos = p_sin_cos.offset(1);
        wim = *fresh168;
        let fresh169 = p_sin_cos;
        p_sin_cos = p_sin_cos.offset(1);
        wre = *fresh169;
        let fresh170 = psubband_t;
        psubband_t = psubband_t.offset(1);
        *fresh170 = (ixheaac_add64(ixheaac_mult64(re, wre), ixheaac_mult64(im, wim))
            >> 32 as core::ffi::c_int) as WORD32;
        let fresh171 = psubband_t;
        psubband_t = psubband_t.offset(1);
        *fresh171 = (ixheaac_sub64_sat(ixheaac_mult64(im, wre), ixheaac_mult64(re, wim))
            >> 32 as core::ffi::c_int) as WORD32;
        let fresh172 = psubband2;
        psubband2 = psubband2.offset(1);
        re = *fresh172;
        let fresh173 = psubband12;
        psubband12 = psubband12.offset(-1);
        im = *fresh173;
        let fresh174 = psubband_t2;
        psubband_t2 = psubband_t2.offset(1);
        *fresh174 = (ixheaac_sub64_sat(ixheaac_mult64(im, wim), ixheaac_mult64(re, wre))
            >> 32 as core::ffi::c_int) as WORD32;
        let fresh175 = psubband_t2;
        psubband_t2 = psubband_t2.offset(1);
        *fresh175 = (ixheaac_add64(ixheaac_mult64(re, wim), ixheaac_mult64(im, wre))
            >> 32 as core::ffi::c_int) as WORD32;
        let fresh176 = psubband1;
        psubband1 = psubband1.offset(-1);
        re = *fresh176;
        let fresh177 = psubband;
        psubband = psubband.offset(1);
        im = *fresh177;
        let fresh178 = p_sin_cos;
        p_sin_cos = p_sin_cos.offset(1);
        wim = *fresh178;
        let fresh179 = p_sin_cos;
        p_sin_cos = p_sin_cos.offset(1);
        wre = *fresh179;
        let fresh180 = psubband1_t;
        psubband1_t = psubband1_t.offset(-1);
        *fresh180 = (ixheaac_sub64_sat(ixheaac_mult64(im, wre), ixheaac_mult64(re, wim))
            >> 32 as core::ffi::c_int) as WORD32;
        let fresh181 = psubband1_t;
        psubband1_t = psubband1_t.offset(-1);
        *fresh181 = (ixheaac_add64(ixheaac_mult64(re, wre), ixheaac_mult64(im, wim))
            >> 32 as core::ffi::c_int) as WORD32;
        let fresh182 = psubband12;
        psubband12 = psubband12.offset(-1);
        re = *fresh182;
        let fresh183 = psubband2;
        psubband2 = psubband2.offset(1);
        im = *fresh183;
        let fresh184 = psubband1_t2;
        psubband1_t2 = psubband1_t2.offset(-1);
        *fresh184 = (ixheaac_add64(ixheaac_mult64(re, wim), ixheaac_mult64(im, wre))
            >> 32 as core::ffi::c_int) as WORD32;
        let fresh185 = psubband1_t2;
        psubband1_t2 = psubband1_t2.offset(-1);
        *fresh185 = (ixheaac_sub64_sat(ixheaac_mult64(im, wim), ixheaac_mult64(re, wre))
            >> 32 as core::ffi::c_int) as WORD32;
        i -= 1;
    }
    if M == 32 as core::ffi::c_int {
        ixheaacd_esbr_radix4bfly(
            p_twiddle,
            subband_tmp.as_mut_ptr(),
            1 as WORD32,
            8 as WORD32,
        );
        ixheaacd_esbr_radix4bfly(
            p_twiddle.offset(48 as core::ffi::c_int as isize),
            subband_tmp.as_mut_ptr(),
            4 as WORD32,
            2 as WORD32,
        );
        ixheaacd_esbr_postradixcompute2(
            subband,
            subband_tmp.as_mut_ptr(),
            p_dig_rev_tbl,
            32 as WORD32,
        );
        ixheaacd_esbr_radix4bfly(
            p_twiddle,
            &mut *subband_tmp.as_mut_ptr().offset(64 as core::ffi::c_int as isize),
            1 as WORD32,
            8 as WORD32,
        );
        ixheaacd_esbr_radix4bfly(
            p_twiddle.offset(48 as core::ffi::c_int as isize),
            &mut *subband_tmp.as_mut_ptr().offset(64 as core::ffi::c_int as isize),
            4 as WORD32,
            2 as WORD32,
        );
        ixheaacd_esbr_postradixcompute2(
            &mut *subband.offset(64 as core::ffi::c_int as isize),
            &mut *subband_tmp.as_mut_ptr().offset(64 as core::ffi::c_int as isize),
            p_dig_rev_tbl,
            32 as WORD32,
        );
    } else if M == 16 as core::ffi::c_int {
        ixheaacd_esbr_radix4bfly(
            p_twiddle,
            subband_tmp.as_mut_ptr(),
            1 as WORD32,
            4 as WORD32,
        );
        ixheaacd_esbr_postradixcompute4(
            subband,
            subband_tmp.as_mut_ptr(),
            p_dig_rev_tbl,
            16 as WORD32,
        );
        ixheaacd_esbr_radix4bfly(
            p_twiddle,
            &mut *subband_tmp.as_mut_ptr().offset(64 as core::ffi::c_int as isize),
            1 as WORD32,
            4 as WORD32,
        );
        ixheaacd_esbr_postradixcompute4(
            &mut *subband.offset(64 as core::ffi::c_int as isize),
            &mut *subband_tmp.as_mut_ptr().offset(64 as core::ffi::c_int as isize),
            p_dig_rev_tbl,
            16 as WORD32,
        );
    } else if M == 12 as core::ffi::c_int {
        z = 0 as core::ffi::c_int as WORD32;
        while z < (*qmf_bank).no_channels >> 1 as core::ffi::c_int {
            temp[z as usize] = subband_tmp[(2 as WORD32 * z) as usize];
            temp[(12 as WORD32 + z) as usize] = subband_tmp[(2 as core::ffi::c_int
                * z as core::ffi::c_int + 1 as core::ffi::c_int) as usize];
            z += 1;
        }
        ixheaacd_complex_fft_p3(
            temp.as_mut_ptr(),
            &mut *temp.as_mut_ptr().offset(12 as core::ffi::c_int as isize),
            12 as WORD32,
            -(1 as WORD32),
            &mut scaleshift,
        );
        z = 0 as core::ffi::c_int as WORD32;
        while z < (*qmf_bank).no_channels >> 1 as core::ffi::c_int {
            *subband.offset((2 as WORD32 * z) as isize) = temp[z as usize];
            *subband
                .offset(
                    (2 as core::ffi::c_int * z as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) = temp[(z as core::ffi::c_int + 12 as core::ffi::c_int) as usize];
            z += 1;
        }
        scaleshift = 0 as core::ffi::c_int as WORD32;
        z = 0 as core::ffi::c_int as WORD32;
        while z < (*qmf_bank).no_channels >> 1 as core::ffi::c_int {
            temp[z as usize] = subband_tmp[(64 as WORD32 + 2 as WORD32 * z) as usize];
            temp[(12 as WORD32 + z) as usize] = subband_tmp[(64 as core::ffi::c_int
                + 2 as core::ffi::c_int * z as core::ffi::c_int + 1 as core::ffi::c_int)
                as usize];
            z += 1;
        }
        ixheaacd_complex_fft_p3(
            temp.as_mut_ptr(),
            &mut *temp.as_mut_ptr().offset(12 as core::ffi::c_int as isize),
            12 as WORD32,
            -(1 as WORD32),
            &mut scaleshift,
        );
        z = 0 as core::ffi::c_int as WORD32;
        while z < (*qmf_bank).no_channels >> 1 as core::ffi::c_int {
            *subband.offset((64 as WORD32 + 2 as WORD32 * z) as isize) = temp[z
                as usize];
            *subband
                .offset(
                    (64 as core::ffi::c_int
                        + 2 as core::ffi::c_int * z as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) = temp[(z as core::ffi::c_int + 12 as core::ffi::c_int) as usize];
            z += 1;
        }
    } else {
        z = 0 as core::ffi::c_int as WORD32;
        while z < (*qmf_bank).no_channels >> 1 as core::ffi::c_int {
            temp[z as usize] = subband_tmp[(2 as WORD32 * z) as usize];
            temp[(8 as WORD32 + z) as usize] = subband_tmp[(2 as core::ffi::c_int
                * z as core::ffi::c_int + 1 as core::ffi::c_int) as usize];
            z += 1;
        }
        (Some(ixheaacd_complex_fft_p2.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            temp.as_mut_ptr(),
            &mut *temp.as_mut_ptr().offset(8 as core::ffi::c_int as isize),
            8 as WORD32,
            -(1 as WORD32),
            &mut scaleshift,
        );
        z = 0 as core::ffi::c_int as WORD32;
        while z < (*qmf_bank).no_channels >> 1 as core::ffi::c_int {
            *subband.offset((2 as WORD32 * z) as isize) = temp[z as usize] << scaleshift;
            *subband
                .offset(
                    (2 as core::ffi::c_int * z as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) = temp[(z as core::ffi::c_int + 8 as core::ffi::c_int) as usize]
                << scaleshift;
            z += 1;
        }
        scaleshift = 0 as core::ffi::c_int as WORD32;
        z = 0 as core::ffi::c_int as WORD32;
        while z < (*qmf_bank).no_channels >> 1 as core::ffi::c_int {
            temp[z as usize] = subband_tmp[(64 as WORD32 + 2 as WORD32 * z) as usize];
            temp[(8 as WORD32 + z) as usize] = subband_tmp[(64 as core::ffi::c_int
                + 2 as core::ffi::c_int * z as core::ffi::c_int + 1 as core::ffi::c_int)
                as usize];
            z += 1;
        }
        (Some(ixheaacd_complex_fft_p2.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            temp.as_mut_ptr(),
            &mut *temp.as_mut_ptr().offset(8 as core::ffi::c_int as isize),
            8 as WORD32,
            -(1 as WORD32),
            &mut scaleshift,
        );
        z = 0 as core::ffi::c_int as WORD32;
        while z < (*qmf_bank).no_channels >> 1 as core::ffi::c_int {
            *subband.offset((64 as WORD32 + 2 as WORD32 * z) as isize) = temp[z as usize]
                << scaleshift;
            *subband
                .offset(
                    (64 as core::ffi::c_int
                        + 2 as core::ffi::c_int * z as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) = temp[(8 as WORD32 + z) as usize] << scaleshift;
            z += 1;
        }
    }
    psubband = &mut *subband.offset(0 as core::ffi::c_int as isize) as *mut WORD32;
    psubband1 = &mut *subband
        .offset(
            (2 as core::ffi::c_int * M as core::ffi::c_int - 1 as core::ffi::c_int)
                as isize,
        ) as *mut WORD32;
    re = *psubband1;
    *psubband = *psubband >> 1 as core::ffi::c_int;
    psubband = psubband.offset(1);
    *psubband1 = ixheaac_negate32_sat(*psubband >> 1 as core::ffi::c_int);
    psubband1 = psubband1.offset(-1);
    p_sin = (*qmf_bank).esbr_alt_sin_twiddle;
    let fresh186 = p_sin;
    p_sin = p_sin.offset(1);
    wim = *fresh186;
    let fresh187 = p_sin;
    p_sin = p_sin.offset(1);
    wre = *fresh187;
    im = *psubband1;
    let fresh188 = psubband1;
    psubband1 = psubband1.offset(-1);
    *fresh188 = (ixheaac_add64(ixheaac_mult64(re, wre), ixheaac_mult64(im, wim))
        >> 32 as core::ffi::c_int) as WORD32;
    let fresh189 = psubband;
    psubband = psubband.offset(1);
    *fresh189 = (ixheaac_sub64_sat(ixheaac_mult64(im, wre), ixheaac_mult64(re, wim))
        >> 32 as core::ffi::c_int) as WORD32;
    psubband2 = &mut *subband.offset(64 as core::ffi::c_int as isize) as *mut WORD32;
    psubband12 = &mut *subband
        .offset(
            (2 as core::ffi::c_int * M as core::ffi::c_int - 1 as core::ffi::c_int
                + 64 as core::ffi::c_int) as isize,
        ) as *mut WORD32;
    re = *psubband12;
    let fresh190 = psubband12;
    psubband12 = psubband12.offset(-1);
    *fresh190 = ixheaac_negate32_sat(*psubband2 >> 1 as core::ffi::c_int);
    *psubband2 = *psubband2.offset(1 as core::ffi::c_int as isize)
        >> 1 as core::ffi::c_int;
    psubband2 = psubband2.offset(1);
    im = *psubband12;
    let fresh191 = psubband2;
    psubband2 = psubband2.offset(1);
    *fresh191 = ixheaac_negate32_sat(
        (ixheaac_add64(ixheaac_mult64(re, wre), ixheaac_mult64(im, wim))
            >> 32 as core::ffi::c_int) as WORD32,
    );
    let fresh192 = psubband12;
    psubband12 = psubband12.offset(-1);
    *fresh192 = (ixheaac_sub64_sat(ixheaac_mult64(re, wim), ixheaac_mult64(im, wre))
        >> 32 as core::ffi::c_int) as WORD32;
    i = (M_2 as core::ffi::c_int - 2 as core::ffi::c_int) as WORD32;
    while i >= 0 as core::ffi::c_int {
        im = *psubband.offset(0 as core::ffi::c_int as isize);
        re = *psubband.offset(1 as core::ffi::c_int as isize);
        re2 = *psubband1;
        let fresh193 = psubband;
        psubband = psubband.offset(1);
        *fresh193 = (ixheaac_add64(ixheaac_mult64(re, wim), ixheaac_mult64(im, wre))
            >> 32 as core::ffi::c_int) as WORD32;
        let fresh194 = psubband1;
        psubband1 = psubband1.offset(-1);
        *fresh194 = (ixheaac_sub64_sat(ixheaac_mult64(im, wim), ixheaac_mult64(re, wre))
            >> 32 as core::ffi::c_int) as WORD32;
        im = *psubband2.offset(0 as core::ffi::c_int as isize);
        re = *psubband2.offset(1 as core::ffi::c_int as isize);
        re3 = *psubband12;
        let fresh195 = psubband12;
        psubband12 = psubband12.offset(-1);
        *fresh195 = ixheaac_negate32_sat(
            (ixheaac_add64(ixheaac_mult64(re, wim), ixheaac_mult64(im, wre))
                >> 32 as core::ffi::c_int) as WORD32,
        );
        let fresh196 = psubband2;
        psubband2 = psubband2.offset(1);
        *fresh196 = (ixheaac_sub64_sat(ixheaac_mult64(re, wre), ixheaac_mult64(im, wim))
            >> 32 as core::ffi::c_int) as WORD32;
        let fresh197 = p_sin;
        p_sin = p_sin.offset(1);
        wim = *fresh197;
        let fresh198 = p_sin;
        p_sin = p_sin.offset(1);
        wre = *fresh198;
        im = *psubband1.offset(0 as core::ffi::c_int as isize);
        let fresh199 = psubband1;
        psubband1 = psubband1.offset(-1);
        *fresh199 = (ixheaac_add64(ixheaac_mult64(re2, wre), ixheaac_mult64(im, wim))
            >> 32 as core::ffi::c_int) as WORD32;
        let fresh200 = psubband;
        psubband = psubband.offset(1);
        *fresh200 = (ixheaac_sub64_sat(ixheaac_mult64(im, wre), ixheaac_mult64(re2, wim))
            >> 32 as core::ffi::c_int) as WORD32;
        im = *psubband12.offset(0 as core::ffi::c_int as isize);
        let fresh201 = psubband2;
        psubband2 = psubband2.offset(1);
        *fresh201 = ixheaac_negate32_sat(
            (ixheaac_add64(ixheaac_mult64(re3, wre), ixheaac_mult64(im, wim))
                >> 32 as core::ffi::c_int) as WORD32,
        );
        let fresh202 = psubband12;
        psubband12 = psubband12.offset(-1);
        *fresh202 = (ixheaac_sub64_sat(ixheaac_mult64(re3, wim), ixheaac_mult64(im, wre))
            >> 32 as core::ffi::c_int) as WORD32;
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_esbr_fwd_modulation(
    mut time_sample_buf: *const WORD32,
    mut real_subband: *mut WORD32,
    mut imag_subband: *mut WORD32,
    mut qmf_bank: *mut ia_sbr_qmf_filter_bank_struct,
    mut qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut time_sample_buf1: *const WORD32 = &*time_sample_buf
        .offset(
            (2 as core::ffi::c_int * (*qmf_bank).no_channels as core::ffi::c_int
                - 1 as core::ffi::c_int) as isize,
        ) as *const WORD32;
    let mut temp1: WORD32 = 0;
    let mut temp2: WORD32 = 0;
    let mut t_real_subband: *mut WORD32 = real_subband;
    let mut t_imag_subband: *mut WORD32 = imag_subband;
    let mut tcos: *const WORD32 = 0 as *const WORD32;
    i = ((*qmf_bank).no_channels as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while i >= 0 as core::ffi::c_int {
        let fresh138 = time_sample_buf;
        time_sample_buf = time_sample_buf.offset(1);
        temp1 = ixheaac_shr32(*fresh138, HQ_SHIFT_64);
        let fresh139 = time_sample_buf1;
        time_sample_buf1 = time_sample_buf1.offset(-1);
        temp2 = ixheaac_shr32(*fresh139, HQ_SHIFT_64);
        let fresh140 = t_real_subband;
        t_real_subband = t_real_subband.offset(1);
        *fresh140 = ixheaac_sub32_sat(temp1, temp2);
        let fresh141 = t_imag_subband;
        t_imag_subband = t_imag_subband.offset(1);
        *fresh141 = ixheaac_add32_sat(temp1, temp2);
        i -= 1;
    }
    ixheaacd_esbr_cos_sin_mod(
        real_subband,
        qmf_bank,
        ((*qmf_dec_tables_ptr).esbr_w_16).as_mut_ptr(),
        ((*qmf_dec_tables_ptr).dig_rev_table4_16).as_mut_ptr(),
    );
    tcos = (*qmf_bank).esbr_t_cos;
    i = ((*qmf_bank).usb as core::ffi::c_int - (*qmf_bank).lsb as core::ffi::c_int
        - 1 as core::ffi::c_int) as WORD32;
    while i >= 0 as core::ffi::c_int {
        let mut cosh: WORD32 = 0;
        let mut sinh: WORD32 = 0;
        let mut re: WORD32 = 0;
        let mut im: WORD32 = 0;
        re = *real_subband;
        im = *imag_subband;
        let fresh142 = tcos;
        tcos = tcos.offset(1);
        cosh = *fresh142;
        let fresh143 = tcos;
        tcos = tcos.offset(1);
        sinh = *fresh143;
        let fresh144 = real_subband;
        real_subband = real_subband.offset(1);
        *fresh144 = (ixheaac_add64(ixheaac_mult64(re, cosh), ixheaac_mult64(im, sinh))
            >> 31 as core::ffi::c_int) as WORD32;
        let fresh145 = imag_subband;
        imag_subband = imag_subband.offset(1);
        *fresh145 = (ixheaac_sub64_sat(
            ixheaac_mult64(im, cosh),
            ixheaac_mult64(re, sinh),
        ) >> 31 as core::ffi::c_int) as WORD32;
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_sbr_qmfsyn64_winadd(
    mut tmp1: *mut WORD16,
    mut tmp2: *mut WORD16,
    mut inp1: *mut WORD16,
    mut sample_buffer: *mut WORD16,
    mut shift: FLAG,
    mut ch_fac: WORD32,
) -> VOID {
    let mut k: WORD32 = 0;
    let mut rounding_fac: WORD32 = 0x8000 as WORD32;
    rounding_fac = rounding_fac >> shift;
    k = 0 as core::ffi::c_int as WORD32;
    while k < 64 as core::ffi::c_int {
        let mut syn_out: WORD32 = rounding_fac;
        syn_out = ixheaac_add32_sat(
            syn_out,
            ixheaac_mult16x16in32(
                *tmp1.offset((0 as WORD32 + k) as isize),
                *inp1.offset((k as core::ffi::c_int + 0 as core::ffi::c_int) as isize),
            ),
        );
        syn_out = ixheaac_add32_sat(
            syn_out,
            ixheaac_mult16x16in32(
                *tmp1.offset((256 as WORD32 + k) as isize),
                *inp1.offset((k as core::ffi::c_int + 128 as core::ffi::c_int) as isize),
            ),
        );
        syn_out = ixheaac_add32_sat(
            syn_out,
            ixheaac_mult16x16in32(
                *tmp1.offset((512 as WORD32 + k) as isize),
                *inp1.offset((k as core::ffi::c_int + 256 as core::ffi::c_int) as isize),
            ),
        );
        syn_out = ixheaac_add32_sat(
            syn_out,
            ixheaac_mult16x16in32(
                *tmp1.offset((768 as WORD32 + k) as isize),
                *inp1.offset((k as core::ffi::c_int + 384 as core::ffi::c_int) as isize),
            ),
        );
        syn_out = ixheaac_add32_sat(
            syn_out,
            ixheaac_mult16x16in32(
                *tmp1.offset((1024 as WORD32 + k) as isize),
                *inp1.offset((k as core::ffi::c_int + 512 as core::ffi::c_int) as isize),
            ),
        );
        syn_out = ixheaac_add32_sat(
            syn_out,
            ixheaac_mult16x16in32(
                *tmp2.offset((128 as WORD32 + k) as isize),
                *inp1.offset((k as core::ffi::c_int + 64 as core::ffi::c_int) as isize),
            ),
        );
        syn_out = ixheaac_add32_sat(
            syn_out,
            ixheaac_mult16x16in32(
                *tmp2.offset((384 as WORD32 + k) as isize),
                *inp1.offset((k as core::ffi::c_int + 192 as core::ffi::c_int) as isize),
            ),
        );
        syn_out = ixheaac_add32_sat(
            syn_out,
            ixheaac_mult16x16in32(
                *tmp2.offset((640 as WORD32 + k) as isize),
                *inp1.offset((k as core::ffi::c_int + 320 as core::ffi::c_int) as isize),
            ),
        );
        syn_out = ixheaac_add32_sat(
            syn_out,
            ixheaac_mult16x16in32(
                *tmp2.offset((896 as WORD32 + k) as isize),
                *inp1.offset((k as core::ffi::c_int + 448 as core::ffi::c_int) as isize),
            ),
        );
        syn_out = ixheaac_add32_sat(
            syn_out,
            ixheaac_mult16x16in32(
                *tmp2.offset((1152 as WORD32 + k) as isize),
                *inp1.offset((k as core::ffi::c_int + 576 as core::ffi::c_int) as isize),
            ),
        );
        *sample_buffer.offset((ch_fac * k) as isize) = (ixheaac_shl32_sat(
            syn_out,
            shift as WORD,
        ) >> 16 as core::ffi::c_int) as WORD16;
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_esbr_qmfsyn64_winadd(
    mut tmp1: *mut WORD32,
    mut tmp2: *mut WORD32,
    mut inp1: *mut WORD32,
    mut sample_buffer: *mut WORD32,
    mut ch_fac: WORD32,
) -> VOID {
    let mut k: WORD32 = 0;
    k = 0 as core::ffi::c_int as WORD32;
    while k < 64 as core::ffi::c_int {
        let mut syn_out: WORD64 = 0 as WORD64;
        syn_out = ixheaac_add64(
            syn_out,
            ixheaac_mult64(
                *tmp1.offset((0 as WORD32 + k) as isize),
                *inp1.offset((k as core::ffi::c_int + 0 as core::ffi::c_int) as isize),
            ),
        );
        syn_out = ixheaac_add64(
            syn_out,
            ixheaac_mult64(
                *tmp1.offset((256 as WORD32 + k) as isize),
                *inp1.offset((k as core::ffi::c_int + 128 as core::ffi::c_int) as isize),
            ),
        );
        syn_out = ixheaac_add64(
            syn_out,
            ixheaac_mult64(
                *tmp1.offset((512 as WORD32 + k) as isize),
                *inp1.offset((k as core::ffi::c_int + 256 as core::ffi::c_int) as isize),
            ),
        );
        syn_out = ixheaac_add64(
            syn_out,
            ixheaac_mult64(
                *tmp1.offset((768 as WORD32 + k) as isize),
                *inp1.offset((k as core::ffi::c_int + 384 as core::ffi::c_int) as isize),
            ),
        );
        syn_out = ixheaac_add64(
            syn_out,
            ixheaac_mult64(
                *tmp1.offset((1024 as WORD32 + k) as isize),
                *inp1.offset((k as core::ffi::c_int + 512 as core::ffi::c_int) as isize),
            ),
        );
        syn_out = ixheaac_add64(
            syn_out,
            ixheaac_mult64(
                *tmp2.offset((128 as WORD32 + k) as isize),
                *inp1.offset((k as core::ffi::c_int + 64 as core::ffi::c_int) as isize),
            ),
        );
        syn_out = ixheaac_add64(
            syn_out,
            ixheaac_mult64(
                *tmp2.offset((384 as WORD32 + k) as isize),
                *inp1.offset((k as core::ffi::c_int + 192 as core::ffi::c_int) as isize),
            ),
        );
        syn_out = ixheaac_add64(
            syn_out,
            ixheaac_mult64(
                *tmp2.offset((640 as WORD32 + k) as isize),
                *inp1.offset((k as core::ffi::c_int + 320 as core::ffi::c_int) as isize),
            ),
        );
        syn_out = ixheaac_add64(
            syn_out,
            ixheaac_mult64(
                *tmp2.offset((896 as WORD32 + k) as isize),
                *inp1.offset((k as core::ffi::c_int + 448 as core::ffi::c_int) as isize),
            ),
        );
        syn_out = ixheaac_add64(
            syn_out,
            ixheaac_mult64(
                *tmp2.offset((1152 as WORD32 + k) as isize),
                *inp1.offset((k as core::ffi::c_int + 576 as core::ffi::c_int) as isize),
            ),
        );
        *sample_buffer.offset((ch_fac * k) as isize) = (syn_out
            >> 31 as core::ffi::c_int) as WORD32;
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_esbr_qmfsyn32_winadd(
    mut tmp1: *mut WORD32,
    mut tmp2: *mut WORD32,
    mut inp1: *mut WORD32,
    mut sample_buffer: *mut WORD32,
    mut ch_fac: WORD32,
) -> VOID {
    let mut k: WORD32 = 0;
    k = 0 as core::ffi::c_int as WORD32;
    while k < 32 as core::ffi::c_int {
        let mut syn_out: WORD64 = 0 as WORD64;
        syn_out = ixheaac_add64(
            syn_out,
            ixheaac_mult64(
                *tmp1.offset((0 as WORD32 + k) as isize),
                *inp1
                    .offset(
                        (2 as core::ffi::c_int
                            * (k as core::ffi::c_int + 0 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        syn_out = ixheaac_add64(
            syn_out,
            ixheaac_mult64(
                *tmp1.offset((128 as WORD32 + k) as isize),
                *inp1
                    .offset(
                        (2 as core::ffi::c_int
                            * (k as core::ffi::c_int + 64 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        syn_out = ixheaac_add64(
            syn_out,
            ixheaac_mult64(
                *tmp1.offset((256 as WORD32 + k) as isize),
                *inp1
                    .offset(
                        (2 as core::ffi::c_int
                            * (k as core::ffi::c_int + 128 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        syn_out = ixheaac_add64(
            syn_out,
            ixheaac_mult64(
                *tmp1.offset((384 as WORD32 + k) as isize),
                *inp1
                    .offset(
                        (2 as core::ffi::c_int
                            * (k as core::ffi::c_int + 192 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        syn_out = ixheaac_add64(
            syn_out,
            ixheaac_mult64(
                *tmp1.offset((512 as WORD32 + k) as isize),
                *inp1
                    .offset(
                        (2 as core::ffi::c_int
                            * (k as core::ffi::c_int + 256 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        syn_out = ixheaac_add64(
            syn_out,
            ixheaac_mult64(
                *tmp2.offset((64 as WORD32 + k) as isize),
                *inp1
                    .offset(
                        (2 as core::ffi::c_int
                            * (k as core::ffi::c_int + 32 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        syn_out = ixheaac_add64(
            syn_out,
            ixheaac_mult64(
                *tmp2.offset((192 as WORD32 + k) as isize),
                *inp1
                    .offset(
                        (2 as core::ffi::c_int
                            * (k as core::ffi::c_int + 96 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        syn_out = ixheaac_add64(
            syn_out,
            ixheaac_mult64(
                *tmp2.offset((320 as WORD32 + k) as isize),
                *inp1
                    .offset(
                        (2 as core::ffi::c_int
                            * (k as core::ffi::c_int + 160 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        syn_out = ixheaac_add64(
            syn_out,
            ixheaac_mult64(
                *tmp2.offset((448 as WORD32 + k) as isize),
                *inp1
                    .offset(
                        (2 as core::ffi::c_int
                            * (k as core::ffi::c_int + 224 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        syn_out = ixheaac_add64(
            syn_out,
            ixheaac_mult64(
                *tmp2.offset((576 as WORD32 + k) as isize),
                *inp1
                    .offset(
                        (2 as core::ffi::c_int
                            * (k as core::ffi::c_int + 288 as core::ffi::c_int)) as isize,
                    ),
            ),
        );
        *sample_buffer.offset((ch_fac * k) as isize) = (syn_out
            >> 31 as core::ffi::c_int) as WORD32;
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_shiftrountine(
    mut qmf_real: *mut WORD32,
    mut qmf_imag: *mut WORD32,
    mut len: WORD32,
    mut common_shift: WORD32,
) -> VOID {
    let mut treal: WORD32 = 0;
    let mut timag: WORD32 = 0;
    let mut j: WORD32 = 0;
    if common_shift < 0 as core::ffi::c_int {
        let mut cshift: WORD32 = -common_shift;
        cshift = ixheaac_min32(cshift, 31 as WORD32);
        j = (len as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        while j >= 0 as core::ffi::c_int {
            treal = *qmf_real;
            timag = *qmf_imag;
            treal = ixheaac_shr32(treal, cshift as WORD);
            timag = ixheaac_shr32(timag, cshift as WORD);
            let fresh247 = qmf_real;
            qmf_real = qmf_real.offset(1);
            *fresh247 = treal;
            let fresh248 = qmf_imag;
            qmf_imag = qmf_imag.offset(1);
            *fresh248 = timag;
            j -= 1;
        }
    } else {
        j = (len as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        while j >= 0 as core::ffi::c_int {
            treal = ixheaac_shl32_sat(*qmf_real, common_shift as WORD);
            timag = ixheaac_shl32_sat(*qmf_imag, common_shift as WORD);
            let fresh249 = qmf_real;
            qmf_real = qmf_real.offset(1);
            *fresh249 = treal;
            let fresh250 = qmf_imag;
            qmf_imag = qmf_imag.offset(1);
            *fresh250 = timag;
            j -= 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_shiftrountine_with_rnd(
    mut qmf_real: *mut WORD32,
    mut qmf_imag: *mut WORD32,
    mut filter_states: *mut WORD16,
    mut len: WORD32,
    mut shift: WORD32,
) -> VOID {
    let mut filter_states_rev: *mut WORD16 = filter_states.offset(len as isize);
    let mut treal: WORD32 = 0;
    let mut timag: WORD32 = 0;
    let mut j: WORD32 = 0;
    j = (len as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while j >= 0 as core::ffi::c_int {
        let mut r1: WORD32 = 0;
        let mut r2: WORD32 = 0;
        let mut i1: WORD32 = 0;
        let mut i2: WORD32 = 0;
        i2 = *qmf_imag.offset(j as isize);
        r2 = *qmf_real.offset(j as isize);
        let fresh251 = qmf_real;
        qmf_real = qmf_real.offset(1);
        r1 = *fresh251;
        let fresh252 = qmf_imag;
        qmf_imag = qmf_imag.offset(1);
        i1 = *fresh252;
        timag = ixheaac_add32_sat(i1, r1);
        timag = ixheaac_shl32_sat(timag, shift as WORD);
        *filter_states_rev.offset(j as isize) = ixheaac_round16(timag);
        treal = ixheaac_sub32_sat(i2, r2);
        treal = ixheaac_shl32_sat(treal, shift as WORD);
        *filter_states.offset(j as isize) = ixheaac_round16(treal);
        treal = ixheaac_sub32_sat(i1, r1);
        treal = ixheaac_shl32_sat(treal, shift as WORD);
        let fresh253 = filter_states;
        filter_states = filter_states.offset(1);
        *fresh253 = ixheaac_round16(treal);
        timag = ixheaac_add32_sat(i2, r2);
        timag = ixheaac_shl32_sat(timag, shift as WORD);
        let fresh254 = filter_states_rev;
        filter_states_rev = filter_states_rev.offset(1);
        *fresh254 = ixheaac_round16(timag);
        j -= 2 as core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_shiftrountine_with_rnd_eld(
    mut qmf_real: *mut WORD32,
    mut qmf_imag: *mut WORD32,
    mut filter_states: *mut WORD16,
    mut len: WORD32,
    mut shift: WORD32,
) -> VOID {
    let mut filter_states_rev: *mut WORD16 = filter_states.offset(len as isize);
    let mut treal: WORD32 = 0;
    let mut timag: WORD32 = 0;
    let mut j: WORD32 = 0;
    j = (len as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while j >= 0 as core::ffi::c_int {
        let mut r1: WORD32 = 0;
        let mut r2: WORD32 = 0;
        let mut i1: WORD32 = 0;
        let mut i2: WORD32 = 0;
        i2 = *qmf_imag.offset(j as isize);
        r2 = *qmf_real.offset(j as isize);
        let fresh255 = qmf_real;
        qmf_real = qmf_real.offset(1);
        r1 = *fresh255;
        let fresh256 = qmf_imag;
        qmf_imag = qmf_imag.offset(1);
        i1 = *fresh256;
        timag = ixheaac_negate32_sat(ixheaac_add32_sat(i1, r1));
        timag = ixheaac_shl32_sat(timag, shift as WORD);
        *filter_states_rev.offset(j as isize) = ixheaac_round16(timag);
        treal = ixheaac_sub32_sat(r2, i2);
        treal = ixheaac_shl32_sat(treal, shift as WORD);
        *filter_states.offset(j as isize) = ixheaac_round16(treal);
        treal = ixheaac_sub32_sat(r1, i1);
        treal = ixheaac_shl32_sat(treal, shift as WORD);
        let fresh257 = filter_states;
        filter_states = filter_states.offset(1);
        *fresh257 = ixheaac_round16(treal);
        timag = ixheaac_negate32_sat(ixheaac_add32_sat(i2, r2));
        timag = ixheaac_shl32_sat(timag, shift as WORD);
        let fresh258 = filter_states_rev;
        filter_states_rev = filter_states_rev.offset(1);
        *fresh258 = ixheaac_round16(timag);
        j -= 2 as core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_shiftrountine_with_rnd_hq(
    mut qmf_real: *mut WORD32,
    mut qmf_imag: *mut WORD32,
    mut filter_states: *mut WORD32,
    mut len: WORD32,
    mut shift: WORD32,
) -> VOID {
    let mut filter_states_rev: *mut WORD32 = filter_states.offset(len as isize);
    let mut treal: WORD32 = 0;
    let mut timag: WORD32 = 0;
    let mut j: WORD32 = 0;
    j = (len as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while j >= 0 as core::ffi::c_int {
        let mut r1: WORD32 = 0;
        let mut r2: WORD32 = 0;
        let mut i1: WORD32 = 0;
        let mut i2: WORD32 = 0;
        i2 = *qmf_imag.offset(j as isize);
        r2 = *qmf_real.offset(j as isize);
        let fresh243 = qmf_real;
        qmf_real = qmf_real.offset(1);
        r1 = *fresh243;
        let fresh244 = qmf_imag;
        qmf_imag = qmf_imag.offset(1);
        i1 = *fresh244;
        timag = ixheaac_add32_sat(i1, r1);
        timag = ixheaac_shl32_sat(timag, shift as WORD);
        *filter_states_rev.offset(j as isize) = timag;
        treal = ixheaac_sub32_sat(i2, r2);
        treal = ixheaac_shl32_sat(treal, shift as WORD);
        *filter_states.offset(j as isize) = treal;
        treal = ixheaac_sub32_sat(i1, r1);
        treal = ixheaac_shl32_sat(treal, shift as WORD);
        let fresh245 = filter_states;
        filter_states = filter_states.offset(1);
        *fresh245 = treal;
        timag = ixheaac_add32_sat(i2, r2);
        timag = ixheaac_shl32_sat(timag, shift as WORD);
        let fresh246 = filter_states_rev;
        filter_states_rev = filter_states_rev.offset(1);
        *fresh246 = timag;
        j -= 2 as core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_radix4bfly(
    mut w: *const WORD16,
    mut x: *mut WORD32,
    mut index1: WORD32,
    mut index: WORD32,
) -> VOID {
    let mut i: core::ffi::c_int = 0;
    let mut l1: WORD32 = 0;
    let mut l2: WORD32 = 0;
    let mut h2: WORD32 = 0;
    let mut fft_jmp: WORD32 = 0;
    let mut xt0_0: WORD32 = 0;
    let mut yt0_0: WORD32 = 0;
    let mut xt1_0: WORD32 = 0;
    let mut yt1_0: WORD32 = 0;
    let mut xt2_0: WORD32 = 0;
    let mut yt2_0: WORD32 = 0;
    let mut xh0_0: WORD32 = 0;
    let mut xh1_0: WORD32 = 0;
    let mut xh20_0: WORD32 = 0;
    let mut xh21_0: WORD32 = 0;
    let mut xl0_0: WORD32 = 0;
    let mut xl1_0: WORD32 = 0;
    let mut xl20_0: WORD32 = 0;
    let mut xl21_0: WORD32 = 0;
    let mut x_0: WORD32 = 0;
    let mut x_1: WORD32 = 0;
    let mut x_l1_0: WORD32 = 0;
    let mut x_l1_1: WORD32 = 0;
    let mut x_l2_0: WORD32 = 0;
    let mut x_l2_1: WORD32 = 0;
    let mut x_h2_0: WORD32 = 0;
    let mut x_h2_1: WORD32 = 0;
    let mut si10: WORD16 = 0;
    let mut si20: WORD16 = 0;
    let mut si30: WORD16 = 0;
    let mut co10: WORD16 = 0;
    let mut co20: WORD16 = 0;
    let mut co30: WORD16 = 0;
    let mut mul_1: WORD32 = 0;
    let mut mul_2: WORD32 = 0;
    let mut mul_3: WORD32 = 0;
    let mut mul_4: WORD32 = 0;
    let mut mul_5: WORD32 = 0;
    let mut mul_6: WORD32 = 0;
    let mut mul_7: WORD32 = 0;
    let mut mul_8: WORD32 = 0;
    let mut mul_9: WORD32 = 0;
    let mut mul_10: WORD32 = 0;
    let mut mul_11: WORD32 = 0;
    let mut mul_12: WORD32 = 0;
    let mut w_ptr: *const WORD16 = w;
    let mut i1: WORD32 = 0;
    h2 = index << 1 as core::ffi::c_int;
    l1 = index << 2 as core::ffi::c_int;
    l2 = (index << 2 as core::ffi::c_int) + (index << 1 as core::ffi::c_int);
    fft_jmp = 6 as WORD32 * index;
    i1 = 0 as core::ffi::c_int as WORD32;
    while i1 < index1 {
        i = 0 as core::ffi::c_int;
        while i < index {
            let fresh50 = w_ptr;
            w_ptr = w_ptr.offset(1);
            si10 = *fresh50;
            let fresh51 = w_ptr;
            w_ptr = w_ptr.offset(1);
            co10 = *fresh51;
            let fresh52 = w_ptr;
            w_ptr = w_ptr.offset(1);
            si20 = *fresh52;
            let fresh53 = w_ptr;
            w_ptr = w_ptr.offset(1);
            co20 = *fresh53;
            let fresh54 = w_ptr;
            w_ptr = w_ptr.offset(1);
            si30 = *fresh54;
            let fresh55 = w_ptr;
            w_ptr = w_ptr.offset(1);
            co30 = *fresh55;
            x_0 = *x.offset(0 as core::ffi::c_int as isize);
            x_h2_0 = *x.offset(h2 as isize);
            x_l1_0 = *x.offset(l1 as isize);
            x_l2_0 = *x.offset(l2 as isize);
            xh0_0 = ixheaac_add32_sat(x_0, x_l1_0);
            xl0_0 = ixheaac_sub32_sat(x_0, x_l1_0);
            xh20_0 = ixheaac_add32_sat(x_h2_0, x_l2_0);
            xl20_0 = ixheaac_sub32_sat(x_h2_0, x_l2_0);
            *x.offset(0 as core::ffi::c_int as isize) = ixheaac_add32_sat(xh0_0, xh20_0);
            xt0_0 = ixheaac_sub32_sat(xh0_0, xh20_0);
            x_1 = *x.offset(1 as core::ffi::c_int as isize);
            x_h2_1 = *x
                .offset((h2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
            x_l1_1 = *x
                .offset((l1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
            x_l2_1 = *x
                .offset((l2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
            xh1_0 = ixheaac_add32_sat(x_1, x_l1_1);
            xl1_0 = ixheaac_sub32_sat(x_1, x_l1_1);
            xh21_0 = ixheaac_add32_sat(x_h2_1, x_l2_1);
            xl21_0 = ixheaac_sub32_sat(x_h2_1, x_l2_1);
            *x.offset(1 as core::ffi::c_int as isize) = ixheaac_add32_sat(xh1_0, xh21_0);
            yt0_0 = ixheaac_sub32_sat(xh1_0, xh21_0);
            xt1_0 = ixheaac_add32_sat(xl0_0, xl21_0);
            xt2_0 = ixheaac_sub32_sat(xl0_0, xl21_0);
            yt2_0 = ixheaac_add32_sat(xl1_0, xl20_0);
            yt1_0 = ixheaac_sub32_sat(xl1_0, xl20_0);
            mul_11 = ixheaac_mult32x16in32(xt2_0, co30);
            mul_3 = ixheaac_mult32x16in32(yt2_0, si30);
            *x.offset(l2 as isize) = mul_3 + mul_11 << RADIXSHIFT;
            mul_5 = ixheaac_mult32x16in32(xt2_0, si30);
            mul_9 = ixheaac_mult32x16in32(yt2_0, co30);
            *x.offset((l2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = mul_9
                - mul_5 << RADIXSHIFT;
            mul_12 = ixheaac_mult32x16in32(xt0_0, co20);
            mul_2 = ixheaac_mult32x16in32(yt0_0, si20);
            *x.offset(l1 as isize) = mul_2 + mul_12 << RADIXSHIFT;
            mul_6 = ixheaac_mult32x16in32(xt0_0, si20);
            mul_8 = ixheaac_mult32x16in32(yt0_0, co20);
            *x.offset((l1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = mul_8
                - mul_6 << RADIXSHIFT;
            mul_4 = ixheaac_mult32x16in32(xt1_0, co10);
            mul_1 = ixheaac_mult32x16in32(yt1_0, si10);
            *x.offset(h2 as isize) = mul_1 + mul_4 << RADIXSHIFT;
            mul_10 = ixheaac_mult32x16in32(xt1_0, si10);
            mul_7 = ixheaac_mult32x16in32(yt1_0, co10);
            *x.offset((h2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = mul_7
                - mul_10 << RADIXSHIFT;
            x = x.offset(2 as core::ffi::c_int as isize);
            i += 1;
        }
        x = x.offset(fft_jmp as isize);
        w_ptr = w_ptr.offset(-(fft_jmp as isize));
        i1 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_postradixcompute4(
    mut ptr_y: *mut WORD32,
    mut ptr_x: *mut WORD32,
    mut p_dig_rev_tbl: *const WORD32,
    mut npoints: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut h2: WORD32 = 0;
    let mut xh0_0: WORD32 = 0;
    let mut xh1_0: WORD32 = 0;
    let mut xl0_0: WORD32 = 0;
    let mut xl1_0: WORD32 = 0;
    let mut xh0_1: WORD32 = 0;
    let mut xh1_1: WORD32 = 0;
    let mut xl0_1: WORD32 = 0;
    let mut xl1_1: WORD32 = 0;
    let mut x_0: WORD32 = 0;
    let mut x_1: WORD32 = 0;
    let mut x_2: WORD32 = 0;
    let mut x_3: WORD32 = 0;
    let mut xh0_2: WORD32 = 0;
    let mut xh1_2: WORD32 = 0;
    let mut xl0_2: WORD32 = 0;
    let mut xl1_2: WORD32 = 0;
    let mut xh0_3: WORD32 = 0;
    let mut xh1_3: WORD32 = 0;
    let mut xl0_3: WORD32 = 0;
    let mut xl1_3: WORD32 = 0;
    let mut x_4: WORD32 = 0;
    let mut x_5: WORD32 = 0;
    let mut x_6: WORD32 = 0;
    let mut x_7: WORD32 = 0;
    let mut x_8: WORD32 = 0;
    let mut x_9: WORD32 = 0;
    let mut x_a: WORD32 = 0;
    let mut x_b: WORD32 = 0;
    let mut x_c: WORD32 = 0;
    let mut x_d: WORD32 = 0;
    let mut x_e: WORD32 = 0;
    let mut x_f: WORD32 = 0;
    let mut n00: WORD32 = 0;
    let mut n10: WORD32 = 0;
    let mut n20: WORD32 = 0;
    let mut n30: WORD32 = 0;
    let mut n01: WORD32 = 0;
    let mut n11: WORD32 = 0;
    let mut n21: WORD32 = 0;
    let mut n31: WORD32 = 0;
    let mut n02: WORD32 = 0;
    let mut n12: WORD32 = 0;
    let mut n22: WORD32 = 0;
    let mut n32: WORD32 = 0;
    let mut n03: WORD32 = 0;
    let mut n13: WORD32 = 0;
    let mut n23: WORD32 = 0;
    let mut n33: WORD32 = 0;
    let mut x2: *mut WORD32 = 0 as *mut WORD32;
    let mut x0: *mut WORD32 = 0 as *mut WORD32;
    let mut y0: *mut WORD32 = 0 as *mut WORD32;
    let mut y1: *mut WORD32 = 0 as *mut WORD32;
    let mut y2: *mut WORD32 = 0 as *mut WORD32;
    let mut y3: *mut WORD32 = 0 as *mut WORD32;
    y0 = ptr_y;
    y2 = ptr_y.offset(npoints as isize);
    x0 = ptr_x;
    x2 = ptr_x.offset((npoints >> 1 as core::ffi::c_int) as isize);
    y1 = y0.offset((npoints >> 1 as core::ffi::c_int) as isize);
    y3 = y2.offset((npoints >> 1 as core::ffi::c_int) as isize);
    k = 0 as core::ffi::c_int as WORD32;
    while k < 2 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < npoints >> 1 as core::ffi::c_int {
            let fresh33 = p_dig_rev_tbl;
            p_dig_rev_tbl = p_dig_rev_tbl.offset(1);
            h2 = *fresh33 >> 2 as core::ffi::c_int;
            let fresh34 = x0;
            x0 = x0.offset(1);
            x_0 = *fresh34;
            let fresh35 = x0;
            x0 = x0.offset(1);
            x_1 = *fresh35;
            let fresh36 = x0;
            x0 = x0.offset(1);
            x_2 = *fresh36;
            let fresh37 = x0;
            x0 = x0.offset(1);
            x_3 = *fresh37;
            let fresh38 = x0;
            x0 = x0.offset(1);
            x_4 = *fresh38;
            let fresh39 = x0;
            x0 = x0.offset(1);
            x_5 = *fresh39;
            let fresh40 = x0;
            x0 = x0.offset(1);
            x_6 = *fresh40;
            let fresh41 = x0;
            x0 = x0.offset(1);
            x_7 = *fresh41;
            xh0_0 = ixheaac_add32_sat(x_0, x_4);
            xh1_0 = ixheaac_add32_sat(x_1, x_5);
            xl0_0 = ixheaac_sub32_sat(x_0, x_4);
            xl1_0 = ixheaac_sub32_sat(x_1, x_5);
            xh0_1 = ixheaac_add32_sat(x_2, x_6);
            xh1_1 = ixheaac_add32_sat(x_3, x_7);
            xl0_1 = ixheaac_sub32_sat(x_2, x_6);
            xl1_1 = ixheaac_sub32_sat(x_3, x_7);
            n00 = ixheaac_add32_sat(xh0_0, xh0_1);
            n01 = ixheaac_add32_sat(xh1_0, xh1_1);
            n10 = ixheaac_add32_sat(xl0_0, xl1_1);
            n11 = ixheaac_sub32_sat(xl1_0, xl0_1);
            n20 = ixheaac_sub32_sat(xh0_0, xh0_1);
            n21 = ixheaac_sub32_sat(xh1_0, xh1_1);
            n30 = ixheaac_sub32_sat(xl0_0, xl1_1);
            n31 = ixheaac_add32_sat(xl1_0, xl0_1);
            *y0.offset(h2 as isize) = n00;
            *y0.offset((h2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = n01;
            *y1.offset(h2 as isize) = n10;
            *y1.offset((h2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = n11;
            *y2.offset(h2 as isize) = n20;
            *y2.offset((h2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = n21;
            *y3.offset(h2 as isize) = n30;
            *y3.offset((h2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = n31;
            let fresh42 = x2;
            x2 = x2.offset(1);
            x_8 = *fresh42;
            let fresh43 = x2;
            x2 = x2.offset(1);
            x_9 = *fresh43;
            let fresh44 = x2;
            x2 = x2.offset(1);
            x_a = *fresh44;
            let fresh45 = x2;
            x2 = x2.offset(1);
            x_b = *fresh45;
            let fresh46 = x2;
            x2 = x2.offset(1);
            x_c = *fresh46;
            let fresh47 = x2;
            x2 = x2.offset(1);
            x_d = *fresh47;
            let fresh48 = x2;
            x2 = x2.offset(1);
            x_e = *fresh48;
            let fresh49 = x2;
            x2 = x2.offset(1);
            x_f = *fresh49;
            xh0_2 = ixheaac_add32_sat(x_8, x_c);
            xh1_2 = ixheaac_add32_sat(x_9, x_d);
            xl0_2 = ixheaac_sub32_sat(x_8, x_c);
            xl1_2 = ixheaac_sub32_sat(x_9, x_d);
            xh0_3 = ixheaac_add32_sat(x_a, x_e);
            xh1_3 = ixheaac_add32_sat(x_b, x_f);
            xl0_3 = ixheaac_sub32_sat(x_a, x_e);
            xl1_3 = ixheaac_sub32_sat(x_b, x_f);
            n02 = ixheaac_add32_sat(xh0_2, xh0_3);
            n03 = ixheaac_add32_sat(xh1_2, xh1_3);
            n12 = ixheaac_add32_sat(xl0_2, xl1_3);
            n13 = ixheaac_sub32_sat(xl1_2, xl0_3);
            n22 = ixheaac_sub32_sat(xh0_2, xh0_3);
            n23 = ixheaac_sub32_sat(xh1_2, xh1_3);
            n32 = ixheaac_sub32_sat(xl0_2, xl1_3);
            n33 = ixheaac_add32_sat(xl1_2, xl0_3);
            *y0.offset((h2 as core::ffi::c_int + 2 as core::ffi::c_int) as isize) = n02;
            *y0.offset((h2 as core::ffi::c_int + 3 as core::ffi::c_int) as isize) = n03;
            *y1.offset((h2 as core::ffi::c_int + 2 as core::ffi::c_int) as isize) = n12;
            *y1.offset((h2 as core::ffi::c_int + 3 as core::ffi::c_int) as isize) = n13;
            *y2.offset((h2 as core::ffi::c_int + 2 as core::ffi::c_int) as isize) = n22;
            *y2.offset((h2 as core::ffi::c_int + 3 as core::ffi::c_int) as isize) = n23;
            *y3.offset((h2 as core::ffi::c_int + 2 as core::ffi::c_int) as isize) = n32;
            *y3.offset((h2 as core::ffi::c_int + 3 as core::ffi::c_int) as isize) = n33;
            i += 8 as core::ffi::c_int;
        }
        x0 = x0.offset((npoints >> 1 as core::ffi::c_int) as isize);
        x2 = x2.offset((npoints >> 1 as core::ffi::c_int) as isize);
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_postradixcompute2(
    mut ptr_y: *mut WORD32,
    mut ptr_x: *mut WORD32,
    mut pdig_rev_tbl: *const WORD32,
    mut npoints: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut h2: WORD32 = 0;
    let mut x_0: WORD32 = 0;
    let mut x_1: WORD32 = 0;
    let mut x_2: WORD32 = 0;
    let mut x_3: WORD32 = 0;
    let mut x_4: WORD32 = 0;
    let mut x_5: WORD32 = 0;
    let mut x_6: WORD32 = 0;
    let mut x_7: WORD32 = 0;
    let mut x_8: WORD32 = 0;
    let mut x_9: WORD32 = 0;
    let mut x_a: WORD32 = 0;
    let mut x_b: WORD32 = 0;
    let mut x_c: WORD32 = 0;
    let mut x_d: WORD32 = 0;
    let mut x_e: WORD32 = 0;
    let mut x_f: WORD32 = 0;
    let mut n00: WORD32 = 0;
    let mut n10: WORD32 = 0;
    let mut n20: WORD32 = 0;
    let mut n30: WORD32 = 0;
    let mut n01: WORD32 = 0;
    let mut n11: WORD32 = 0;
    let mut n21: WORD32 = 0;
    let mut n31: WORD32 = 0;
    let mut n02: WORD32 = 0;
    let mut n12: WORD32 = 0;
    let mut n22: WORD32 = 0;
    let mut n32: WORD32 = 0;
    let mut n03: WORD32 = 0;
    let mut n13: WORD32 = 0;
    let mut n23: WORD32 = 0;
    let mut n33: WORD32 = 0;
    let mut x2: *mut WORD32 = 0 as *mut WORD32;
    let mut x0: *mut WORD32 = 0 as *mut WORD32;
    let mut y0: *mut WORD32 = 0 as *mut WORD32;
    let mut y1: *mut WORD32 = 0 as *mut WORD32;
    let mut y2: *mut WORD32 = 0 as *mut WORD32;
    let mut y3: *mut WORD32 = 0 as *mut WORD32;
    y0 = ptr_y;
    y2 = ptr_y.offset(npoints as isize);
    x0 = ptr_x;
    x2 = ptr_x.offset((npoints >> 1 as core::ffi::c_int) as isize);
    y1 = y0.offset((npoints >> 2 as core::ffi::c_int) as isize);
    y3 = y2.offset((npoints >> 2 as core::ffi::c_int) as isize);
    k = 0 as core::ffi::c_int as WORD32;
    while k < 2 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < npoints >> 1 as core::ffi::c_int {
            let fresh121 = pdig_rev_tbl;
            pdig_rev_tbl = pdig_rev_tbl.offset(1);
            h2 = *fresh121 >> 2 as core::ffi::c_int;
            let fresh122 = x0;
            x0 = x0.offset(1);
            x_0 = *fresh122;
            let fresh123 = x0;
            x0 = x0.offset(1);
            x_1 = *fresh123;
            let fresh124 = x0;
            x0 = x0.offset(1);
            x_2 = *fresh124;
            let fresh125 = x0;
            x0 = x0.offset(1);
            x_3 = *fresh125;
            let fresh126 = x0;
            x0 = x0.offset(1);
            x_4 = *fresh126;
            let fresh127 = x0;
            x0 = x0.offset(1);
            x_5 = *fresh127;
            let fresh128 = x0;
            x0 = x0.offset(1);
            x_6 = *fresh128;
            let fresh129 = x0;
            x0 = x0.offset(1);
            x_7 = *fresh129;
            n00 = ixheaac_add32_sat(x_0, x_2);
            n01 = ixheaac_add32_sat(x_1, x_3);
            n20 = ixheaac_sub32_sat(x_0, x_2);
            n21 = ixheaac_sub32_sat(x_1, x_3);
            n10 = ixheaac_add32_sat(x_4, x_6);
            n11 = ixheaac_add32_sat(x_5, x_7);
            n30 = ixheaac_sub32_sat(x_4, x_6);
            n31 = ixheaac_sub32_sat(x_5, x_7);
            *y0.offset(h2 as isize) = n00;
            *y0.offset((h2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = n01;
            *y1.offset(h2 as isize) = n10;
            *y1.offset((h2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = n11;
            *y2.offset(h2 as isize) = n20;
            *y2.offset((h2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = n21;
            *y3.offset(h2 as isize) = n30;
            *y3.offset((h2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = n31;
            let fresh130 = x2;
            x2 = x2.offset(1);
            x_8 = *fresh130;
            let fresh131 = x2;
            x2 = x2.offset(1);
            x_9 = *fresh131;
            let fresh132 = x2;
            x2 = x2.offset(1);
            x_a = *fresh132;
            let fresh133 = x2;
            x2 = x2.offset(1);
            x_b = *fresh133;
            let fresh134 = x2;
            x2 = x2.offset(1);
            x_c = *fresh134;
            let fresh135 = x2;
            x2 = x2.offset(1);
            x_d = *fresh135;
            let fresh136 = x2;
            x2 = x2.offset(1);
            x_e = *fresh136;
            let fresh137 = x2;
            x2 = x2.offset(1);
            x_f = *fresh137;
            n02 = ixheaac_add32_sat(x_8, x_a);
            n03 = ixheaac_add32_sat(x_9, x_b);
            n22 = ixheaac_sub32_sat(x_8, x_a);
            n23 = ixheaac_sub32_sat(x_9, x_b);
            n12 = ixheaac_add32_sat(x_c, x_e);
            n13 = ixheaac_add32_sat(x_d, x_f);
            n32 = ixheaac_sub32_sat(x_c, x_e);
            n33 = ixheaac_sub32_sat(x_d, x_f);
            *y0.offset((h2 as core::ffi::c_int + 2 as core::ffi::c_int) as isize) = n02;
            *y0.offset((h2 as core::ffi::c_int + 3 as core::ffi::c_int) as isize) = n03;
            *y1.offset((h2 as core::ffi::c_int + 2 as core::ffi::c_int) as isize) = n12;
            *y1.offset((h2 as core::ffi::c_int + 3 as core::ffi::c_int) as isize) = n13;
            *y2.offset((h2 as core::ffi::c_int + 2 as core::ffi::c_int) as isize) = n22;
            *y2.offset((h2 as core::ffi::c_int + 3 as core::ffi::c_int) as isize) = n23;
            *y3.offset((h2 as core::ffi::c_int + 2 as core::ffi::c_int) as isize) = n32;
            *y3.offset((h2 as core::ffi::c_int + 3 as core::ffi::c_int) as isize) = n33;
            i += 8 as core::ffi::c_int;
        }
        x0 = x0.offset((npoints >> 1 as core::ffi::c_int) as isize);
        x2 = x2.offset((npoints >> 1 as core::ffi::c_int) as isize);
        k += 1;
    }
}
