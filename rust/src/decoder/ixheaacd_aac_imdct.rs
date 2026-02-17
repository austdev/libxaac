extern "C" {
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    static mut ixheaacd_calc_max_spectral_line: Option<
        unsafe extern "C" fn(*mut WORD32, WORD32) -> WORD32,
    >;
    static mut ixheaacd_pretwiddle_compute: Option<
        unsafe extern "C" fn(
            *mut WORD32,
            *mut WORD32,
            *mut WORD32,
            *mut ia_aac_dec_imdct_tables_struct,
            WORD,
            WORD32,
        ) -> VOID,
    >;
    static mut ixheaacd_imdct_using_fft: Option<
        unsafe extern "C" fn(
            *mut ia_aac_dec_imdct_tables_struct,
            WORD32,
            *mut WORD32,
            *mut WORD32,
        ) -> VOID,
    >;
    static mut ixheaacd_fft_15_ld: Option<
        unsafe extern "C" fn(*mut WORD32, *mut WORD32, *mut WORD32, *mut UWORD8) -> VOID,
    >;
    static mut ixheaacd_aac_ld_dec_rearrange: Option<
        unsafe extern "C" fn(*mut WORD32, *mut WORD32, WORD32, *mut UWORD8) -> VOID,
    >;
    static mut ixheaacd_aac_ld_dec_rearrange_960: Option<
        unsafe extern "C" fn(*mut WORD32, *mut WORD32, WORD32, *mut WORD16) -> VOID,
    >;
    static mut ixheaacd_fft32x32_ld: Option<
        unsafe extern "C" fn(
            *mut ia_aac_dec_imdct_tables_struct,
            WORD32,
            *mut WORD32,
            *mut WORD32,
        ) -> VOID,
    >;
    static mut ixheaacd_fft32x32_ld2: Option<
        unsafe extern "C" fn(
            *mut ia_aac_dec_imdct_tables_struct,
            WORD32,
            *mut WORD32,
            *mut WORD32,
        ) -> VOID,
    >;
    static mut ixheaacd_neg_expo_inc: Option<unsafe extern "C" fn(WORD16) -> WORD16>;
}
pub type WORD8 = core::ffi::c_schar;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD16 = core::ffi::c_short;
pub type UWORD16 = core::ffi::c_ushort;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type WORD64 = core::ffi::c_longlong;
pub type VOID = ();
pub type WORD = core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_imdct_tables_struct {
    pub cosine_array_2048_256: [WORD16; 514],
    pub dig_rev_table8_long: [WORD8; 64],
    pub dig_rev_table8_short: [WORD8; 8],
    pub fft_twiddle: [WORD32; 448],
    pub only_long_window_sine: [WORD16; 1024],
    pub only_long_window_kbd: [WORD16; 1024],
    pub only_short_window_sine: [WORD16; 128],
    pub only_short_window_kbd: [WORD16; 128],
    pub cosine_array_2048_256p: [WORD16; 514],
    pub w1024: [WORD32; 768],
    pub bit_rev_1024: [UWORD8; 256],
    pub bit_rev_512: [UWORD8; 64],
    pub bit_rev_128: [UWORD8; 16],
    pub bit_rev_32: [UWORD8; 4],
    pub w_256: [WORD32; 504],
    pub low_overlap_win: [WORD32; 512],
    pub window_sine_512: [WORD32; 512],
    pub cosine_array_1024: [WORD32; 512],
    pub low_overlap_win_480: [WORD32; 480],
    pub window_sine_480: [WORD32; 480],
    pub re_arr_tab_16: [UWORD8; 240],
    pub re_arr_tab_sml_240: [UWORD8; 240],
    pub cosine_array_960: [WORD32; 480],
    pub w_16: [WORD32; 24],
    pub window_sine_480_eld: [WORD16; 1920],
    pub window_sine_512_eld: [WORD16; 2048],
    pub only_long_window_sine_960: [WORD16; 960],
    pub only_long_window_kbd_960: [WORD16; 960],
    pub only_short_window_sine_120: [WORD16; 120],
    pub only_short_window_kbd_120: [WORD16; 120],
    pub re_arr_tab_32: [WORD16; 480],
    pub re_arr_tab_sml: [WORD16; 16],
    pub re_arr_tab_4: [WORD16; 60],
    pub re_arr_tab_15_4: [WORD16; 60],
    pub re_arr_tab_120: [WORD16; 60],
    pub re_arr_tab_5: [WORD16; 16],
    pub re_arr_tab_3: [WORD16; 16],
    pub re_arr_tab_sml_480: [WORD16; 480],
    pub cosine_array_1920: [WORD32; 960],
    pub w_512: [WORD16; 1020],
    pub w_32: [WORD16; 60],
    pub cosine_array_240: [WORD16; 120],
}
pub const AOT_ER_AAC_ELD: AUDIO_OBJECT_TYPE = 39;
pub type size_t = usize;
pub const AOT_ER_AAC_LD: AUDIO_OBJECT_TYPE = 23;
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
unsafe extern "C" fn ixheaac_shr32_dir_sat(mut a: WORD32, mut b: WORD) -> WORD32 {
    let mut out_val: WORD32 = 0;
    if b < 0 as core::ffi::c_int {
        out_val = ixheaac_shl32_sat(a, -b);
    } else {
        out_val = ixheaac_shr32(a, b);
    }
    return out_val;
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
unsafe extern "C" fn ixheaac_shr32_sat(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut out_val: WORD32 = 0;
    b = ((b << 24 as core::ffi::c_int) as UWORD32 >> 24 as core::ffi::c_int) as WORD32;
    if b >= 31 as core::ffi::c_int {
        if a < 0 as core::ffi::c_int {
            out_val = -(1 as core::ffi::c_int) as WORD32;
        } else {
            out_val = 0 as core::ffi::c_int as WORD32;
        }
    } else if b <= 0 as core::ffi::c_int {
        return a
    } else {
        a = ixheaac_add32_sat(
            a,
            (1 as WORD32) << b as core::ffi::c_int - 1 as core::ffi::c_int,
        );
        out_val = a >> b;
    }
    return out_val;
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
unsafe extern "C" fn ixheaac_mult32x32in32(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 16 as core::ffi::c_int) as WORD32;
    return result;
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
unsafe extern "C" fn ixheaac_mult32(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 32 as core::ffi::c_int) as WORD32;
    return result;
}
#[inline]
unsafe extern "C" fn ixheaac_mac32x16in32(
    mut a: WORD32,
    mut b: WORD32,
    mut c: WORD16,
) -> WORD32 {
    let mut result: WORD32 = 0;
    result = a + ixheaac_mult32x16in32(b, c);
    return result;
}
#[no_mangle]
pub static mut rev_dig: [WORD32; 4] = [
    0 as core::ffi::c_int,
    8 as core::ffi::c_int,
    2 as core::ffi::c_int,
    10 as core::ffi::c_int,
];
pub const MDCT_LEN: core::ffi::c_int = 480 as core::ffi::c_int;
pub const MDCT_LEN_BY2: core::ffi::c_int = 240 as core::ffi::c_int;
pub const FFT5: core::ffi::c_int = 5 as core::ffi::c_int;
pub const FFT16: core::ffi::c_int = 16 as core::ffi::c_int;
pub const FFT4: core::ffi::c_int = 4 as core::ffi::c_int;
pub const FFT3: core::ffi::c_int = 3 as core::ffi::c_int;
pub const FFT15: core::ffi::c_int = 15 as core::ffi::c_int;
pub const FFT16X2: core::ffi::c_int = 32 as core::ffi::c_int;
pub const MDCT_LEN_960: core::ffi::c_int = 960 as core::ffi::c_int;
#[no_mangle]
pub static mut ixheaacd_fft5out: [WORD32; 30] = [0; 30];
#[inline]
unsafe extern "C" fn ixheaacd_mult32x16lin32(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64
        * ((b as core::ffi::c_int & 0xffff as core::ffi::c_int) << 16 as core::ffi::c_int
            >> 16 as core::ffi::c_int) as WORD64;
    result = (temp_result >> 16 as core::ffi::c_int) as WORD32;
    return result;
}
#[inline]
unsafe extern "C" fn ixheaacd_mac32x16lin32(
    mut a: WORD32,
    mut b: WORD32,
    mut c: WORD32,
) -> WORD32 {
    let mut result: WORD32 = 0;
    result = a + ixheaacd_mult32x16lin32(b, c);
    return result;
}
#[inline]
unsafe extern "C" fn ixheaacd_mult32x16lin32_sat(
    mut a: WORD32,
    mut b: WORD32,
) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64
        * ((b as core::ffi::c_int & 0xffff as core::ffi::c_int) << 16 as core::ffi::c_int
            >> 16 as core::ffi::c_int) as WORD64;
    if temp_result < MIN_32 as WORD64 {
        result = MIN_32;
    } else if temp_result > MAX_32 as WORD64 {
        result = MAX_32;
    } else {
        result = temp_result as WORD32;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_neg_expo_inc_dec(mut neg_expo: WORD16) -> WORD16 {
    return (neg_expo as core::ffi::c_int + 2 as core::ffi::c_int) as WORD16;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_neg_expo_inc_arm(mut neg_expo: WORD16) -> WORD16 {
    return (neg_expo as core::ffi::c_int + 3 as core::ffi::c_int) as WORD16;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_pretwiddle_compute_960_dec(
    mut spec_data1: *mut WORD32,
    mut spec_data2: *mut WORD32,
    mut out_ptr: *mut WORD32,
    mut ptr_imdct_tables: *mut ia_aac_dec_imdct_tables_struct,
    mut npoints4: WORD,
    mut neg_expo: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut tempr: WORD32 = 0;
    let mut tempi: WORD32 = 0;
    let mut c: WORD16 = 0;
    let mut c1: WORD16 = 0;
    let mut s: WORD16 = 0;
    let mut s1: WORD16 = 0;
    let mut out_ptr1: *mut WORD32 = out_ptr
        .offset(
            (((npoints4 as core::ffi::c_int) << 2 as core::ffi::c_int)
                - 1 as core::ffi::c_int) as isize,
        );
    let mut cos_sin_ptr: *const WORD16 = ((*ptr_imdct_tables).cosine_array_240)
        .as_mut_ptr();
    i = 0 as core::ffi::c_int as WORD32;
    while i < npoints4 {
        let fresh109 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        c = *fresh109;
        let fresh110 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        s = *fresh110;
        let fresh111 = spec_data1;
        spec_data1 = spec_data1.offset(1);
        tempr = *fresh111;
        let fresh112 = spec_data2;
        spec_data2 = spec_data2.offset(-1);
        tempi = *fresh112;
        *out_ptr = ixheaac_mac32x16in32(ixheaac_mult32x16in32(tempr, c), tempi, s);
        *out_ptr = ixheaac_shl32(*out_ptr, neg_expo as WORD);
        out_ptr = out_ptr.offset(1);
        *out_ptr = ixheaac_sub32(
            ixheaac_mult32x16in32(tempi, c),
            ixheaac_mult32x16in32(tempr, s),
        );
        *out_ptr = ixheaac_shl32(*out_ptr, neg_expo as WORD);
        out_ptr = out_ptr.offset(1);
        let fresh113 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        c1 = *fresh113;
        let fresh114 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        s1 = *fresh114;
        let fresh115 = spec_data1;
        spec_data1 = spec_data1.offset(1);
        tempi = *fresh115;
        let fresh116 = spec_data2;
        spec_data2 = spec_data2.offset(-1);
        tempr = *fresh116;
        *out_ptr1 = ixheaac_sub32(
            ixheaac_mult32x16in32(tempi, c1),
            ixheaac_mult32x16in32(tempr, s1),
        );
        *out_ptr1 = ixheaac_shl32(*out_ptr1, neg_expo as WORD);
        out_ptr1 = out_ptr1.offset(-1);
        *out_ptr1 = ixheaac_mac32x16in32(ixheaac_mult32x16in32(tempr, c1), tempi, s1);
        *out_ptr1 = ixheaac_shl32(*out_ptr1, neg_expo as WORD);
        out_ptr1 = out_ptr1.offset(-1);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_pretwiddle_compute_dec(
    mut spec_data1: *mut WORD32,
    mut spec_data2: *mut WORD32,
    mut out_ptr: *mut WORD32,
    mut ptr_imdct_tables: *mut ia_aac_dec_imdct_tables_struct,
    mut npoints4: WORD,
    mut neg_expo: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut tempr: WORD32 = 0;
    let mut tempi: WORD32 = 0;
    let mut tempr1: WORD32 = 0;
    let mut tempi1: WORD32 = 0;
    let mut npoints2: WORD32 = npoints4 as WORD32 * 2 as WORD32;
    let mut out_ptr1: *mut WORD32 = out_ptr
        .offset((npoints2 << 1 as core::ffi::c_int) as isize)
        .offset(-(1 as core::ffi::c_int as isize));
    let mut cos_sin_ptr: *const WORD16 = ((*ptr_imdct_tables).cosine_array_2048_256)
        .as_mut_ptr();
    let mut cos: WORD16 = 0 as WORD16;
    let mut cos1: WORD16 = 0 as WORD16;
    let mut sin: WORD16 = 0 as WORD16;
    let mut sin1: WORD16 = 0 as WORD16;
    if neg_expo < 0 as core::ffi::c_int {
        neg_expo = -neg_expo;
        if npoints4 == 256 as core::ffi::c_int {
            let fresh83 = cos_sin_ptr;
            cos_sin_ptr = cos_sin_ptr.offset(1);
            cos = *fresh83;
            let fresh84 = cos_sin_ptr;
            cos_sin_ptr = cos_sin_ptr.offset(1);
            sin = *fresh84;
        } else if npoints4 == 32 as core::ffi::c_int {
            let fresh85 = cos_sin_ptr;
            cos_sin_ptr = cos_sin_ptr.offset(1);
            cos = *fresh85;
            sin = *cos_sin_ptr;
            cos_sin_ptr = cos_sin_ptr.offset(15 as core::ffi::c_int as isize);
        }
        let fresh86 = spec_data1;
        spec_data1 = spec_data1.offset(1);
        tempr = *fresh86;
        let fresh87 = spec_data2;
        spec_data2 = spec_data2.offset(-1);
        tempi = *fresh87;
        *out_ptr = ixheaac_mac32x16in32(ixheaac_mult32x16in32(tempr, cos), tempi, sin);
        *out_ptr = ixheaac_shl32(*out_ptr, neg_expo as WORD);
        out_ptr = out_ptr.offset(1);
        *out_ptr = ixheaac_sub32(
            ixheaac_mult32x16in32(tempi, cos),
            ixheaac_mult32x16in32(tempr, sin),
        );
        *out_ptr = ixheaac_shl32(*out_ptr, neg_expo as WORD);
        out_ptr = out_ptr.offset(1);
        i = 0 as core::ffi::c_int as WORD32;
        while i < npoints4 as core::ffi::c_int - 1 as core::ffi::c_int {
            if npoints4 == 256 as core::ffi::c_int {
                let fresh88 = cos_sin_ptr;
                cos_sin_ptr = cos_sin_ptr.offset(1);
                sin = *fresh88;
                let fresh89 = cos_sin_ptr;
                cos_sin_ptr = cos_sin_ptr.offset(1);
                cos = *fresh89;
            } else if npoints4 == 32 as core::ffi::c_int {
                let fresh90 = cos_sin_ptr;
                cos_sin_ptr = cos_sin_ptr.offset(1);
                sin = *fresh90;
                cos = *cos_sin_ptr;
                cos_sin_ptr = cos_sin_ptr.offset(15 as core::ffi::c_int as isize);
            }
            let fresh91 = spec_data1;
            spec_data1 = spec_data1.offset(1);
            tempi1 = *fresh91;
            let fresh92 = spec_data1;
            spec_data1 = spec_data1.offset(1);
            tempr = *fresh92;
            let fresh93 = spec_data2;
            spec_data2 = spec_data2.offset(-1);
            tempr1 = *fresh93;
            let fresh94 = spec_data2;
            spec_data2 = spec_data2.offset(-1);
            tempi = *fresh94;
            *out_ptr1 = ixheaac_sub32(
                ixheaac_mult32x16in32(tempi1, cos),
                ixheaac_mult32x16in32(tempr1, sin),
            );
            *out_ptr1 = ixheaac_shl32(*out_ptr1, neg_expo as WORD);
            out_ptr1 = out_ptr1.offset(-1);
            *out_ptr1 = ixheaac_mac32x16in32(
                ixheaac_mult32x16in32(tempr1, cos),
                tempi1,
                sin,
            );
            *out_ptr1 = ixheaac_shl32(*out_ptr1, neg_expo as WORD);
            out_ptr1 = out_ptr1.offset(-1);
            *out_ptr = ixheaac_mac32x16in32(
                ixheaac_mult32x16in32(tempr, sin),
                tempi,
                cos,
            );
            *out_ptr = ixheaac_shl32(*out_ptr, neg_expo as WORD);
            out_ptr = out_ptr.offset(1);
            *out_ptr = ixheaac_sub32(
                ixheaac_mult32x16in32(tempi, sin),
                ixheaac_mult32x16in32(tempr, cos),
            );
            *out_ptr = ixheaac_shl32(*out_ptr, neg_expo as WORD);
            out_ptr = out_ptr.offset(1);
            i += 1;
        }
        let fresh95 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        cos1 = *fresh95;
        sin1 = *cos_sin_ptr;
        tempr1 = *spec_data2;
        tempi1 = *spec_data1;
        *out_ptr1 = ixheaac_sub32(
            ixheaac_mult32x16in32(tempi1, cos1),
            ixheaac_mult32x16in32(tempr1, sin1),
        );
        *out_ptr1 = ixheaac_shl32(*out_ptr1, neg_expo as WORD);
        out_ptr1 = out_ptr1.offset(-1);
        *out_ptr1 = ixheaac_mac32x16in32(
            ixheaac_mult32x16in32(tempr1, cos1),
            tempi1,
            sin1,
        );
        *out_ptr1 = ixheaac_shl32(*out_ptr1, neg_expo as WORD);
        out_ptr1 = out_ptr1.offset(-1);
    } else {
        if npoints4 == 256 as core::ffi::c_int {
            let fresh96 = cos_sin_ptr;
            cos_sin_ptr = cos_sin_ptr.offset(1);
            cos = *fresh96;
            let fresh97 = cos_sin_ptr;
            cos_sin_ptr = cos_sin_ptr.offset(1);
            sin = *fresh97;
        } else if npoints4 == 32 as core::ffi::c_int {
            let fresh98 = cos_sin_ptr;
            cos_sin_ptr = cos_sin_ptr.offset(1);
            cos = *fresh98;
            sin = *cos_sin_ptr;
            cos_sin_ptr = cos_sin_ptr.offset(15 as core::ffi::c_int as isize);
        }
        let fresh99 = spec_data1;
        spec_data1 = spec_data1.offset(1);
        tempr = *fresh99;
        let fresh100 = spec_data2;
        spec_data2 = spec_data2.offset(-1);
        tempi = *fresh100;
        *out_ptr = ixheaac_mac32x16in32(ixheaac_mult32x16in32(tempr, cos), tempi, sin);
        *out_ptr = ixheaac_shr32(*out_ptr, neg_expo as WORD);
        out_ptr = out_ptr.offset(1);
        *out_ptr = ixheaac_sub32(
            ixheaac_mult32x16in32(tempi, cos),
            ixheaac_mult32x16in32(tempr, sin),
        );
        *out_ptr = ixheaac_shr32(*out_ptr, neg_expo as WORD);
        out_ptr = out_ptr.offset(1);
        i = 0 as core::ffi::c_int as WORD32;
        while i < npoints4 as core::ffi::c_int - 1 as core::ffi::c_int {
            if npoints4 == 256 as core::ffi::c_int {
                let fresh101 = cos_sin_ptr;
                cos_sin_ptr = cos_sin_ptr.offset(1);
                sin = *fresh101;
                let fresh102 = cos_sin_ptr;
                cos_sin_ptr = cos_sin_ptr.offset(1);
                cos = *fresh102;
            } else if npoints4 == 32 as core::ffi::c_int {
                let fresh103 = cos_sin_ptr;
                cos_sin_ptr = cos_sin_ptr.offset(1);
                sin = *fresh103;
                cos = *cos_sin_ptr;
                cos_sin_ptr = cos_sin_ptr.offset(15 as core::ffi::c_int as isize);
            }
            let fresh104 = spec_data1;
            spec_data1 = spec_data1.offset(1);
            tempi1 = *fresh104;
            let fresh105 = spec_data1;
            spec_data1 = spec_data1.offset(1);
            tempr = *fresh105;
            let fresh106 = spec_data2;
            spec_data2 = spec_data2.offset(-1);
            tempr1 = *fresh106;
            let fresh107 = spec_data2;
            spec_data2 = spec_data2.offset(-1);
            tempi = *fresh107;
            *out_ptr1 = ixheaac_sub32(
                ixheaac_mult32x16in32(tempi1, cos),
                ixheaac_mult32x16in32(tempr1, sin),
            );
            *out_ptr1 = ixheaac_shr32(*out_ptr1, neg_expo as WORD);
            out_ptr1 = out_ptr1.offset(-1);
            *out_ptr1 = ixheaac_mac32x16in32(
                ixheaac_mult32x16in32(tempr1, cos),
                tempi1,
                sin,
            );
            *out_ptr1 = ixheaac_shr32(*out_ptr1, neg_expo as WORD);
            out_ptr1 = out_ptr1.offset(-1);
            *out_ptr = ixheaac_mac32x16in32(
                ixheaac_mult32x16in32(tempr, sin),
                tempi,
                cos,
            );
            *out_ptr = ixheaac_shr32(*out_ptr, neg_expo as WORD);
            out_ptr = out_ptr.offset(1);
            *out_ptr = ixheaac_sub32(
                ixheaac_mult32x16in32(tempi, sin),
                ixheaac_mult32x16in32(tempr, cos),
            );
            *out_ptr = ixheaac_shr32(*out_ptr, neg_expo as WORD);
            out_ptr = out_ptr.offset(1);
            i += 1;
        }
        let fresh108 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        cos1 = *fresh108;
        sin1 = *cos_sin_ptr;
        tempr1 = *spec_data2;
        tempi1 = *spec_data1;
        *out_ptr1 = ixheaac_sub32(
            ixheaac_mult32x16in32(tempi1, cos1),
            ixheaac_mult32x16in32(tempr1, sin1),
        );
        *out_ptr1 = ixheaac_shr32(*out_ptr1, neg_expo as WORD);
        out_ptr1 = out_ptr1.offset(-1);
        *out_ptr1 = ixheaac_mac32x16in32(
            ixheaac_mult32x16in32(tempr1, cos1),
            tempi1,
            sin1,
        );
        *out_ptr1 = ixheaac_shr32(*out_ptr1, neg_expo as WORD);
        out_ptr1 = out_ptr1.offset(-1);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_post_twiddle_dec(
    mut out_ptr: *mut WORD32,
    mut spec_data: *mut WORD32,
    mut ptr_imdct_tables: *mut ia_aac_dec_imdct_tables_struct,
    mut npoints: WORD,
) -> VOID {
    let mut i: WORD = 0;
    let mut cos: WORD16 = 0;
    let mut cos1: WORD16 = 0;
    let mut sin: WORD16 = 0;
    let mut sin1: WORD16 = 0;
    let mut spec_data1: *mut WORD32 = spec_data
        .offset(npoints as isize)
        .offset(-(1 as core::ffi::c_int as isize));
    let mut out_ptr1: *mut WORD32 = out_ptr
        .offset(npoints as isize)
        .offset(-(1 as core::ffi::c_int as isize));
    let mut adjust: WORD16 = 50 as WORD16;
    let mut adjust1: WORD16 = -(50 as core::ffi::c_int) as WORD16;
    let mut cos_sin_ptr: *const WORD16 = ((*ptr_imdct_tables).cosine_array_2048_256)
        .as_mut_ptr();
    if npoints == 1024 as core::ffi::c_int {
        let mut tempr: WORD32 = 0;
        let mut tempi: WORD32 = 0;
        let mut outi: WORD32 = 0;
        let mut outr: WORD32 = 0;
        let mut temp1: WORD32 = 0;
        let mut temp2: WORD32 = 0;
        let fresh26 = spec_data;
        spec_data = spec_data.offset(1);
        tempr = *fresh26;
        let fresh27 = spec_data;
        spec_data = spec_data.offset(1);
        tempi = *fresh27;
        cos = *cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        sin = *cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        outi = ixheaac_sub32(
            ixheaac_mult32x16in32(tempr, sin),
            ixheaac_mult32x16in32(tempi, cos),
        );
        outr = ixheaac_mac32x16in32(ixheaac_mult32x16in32(tempr, cos), tempi, sin);
        temp1 = ixheaac_mult32x16in32(outi, adjust1);
        temp2 = ixheaac_mult32x16in32(outr, adjust);
        outr = outr + temp1;
        outi = outi + temp2;
        let fresh28 = out_ptr1;
        out_ptr1 = out_ptr1.offset(-1);
        *fresh28 = outi;
        let fresh29 = out_ptr;
        out_ptr = out_ptr.offset(1);
        *fresh29 = outr;
        i = 0 as core::ffi::c_int as WORD;
        while i
            < npoints as core::ffi::c_int / 2 as core::ffi::c_int - 2 as core::ffi::c_int
        {
            let fresh30 = cos_sin_ptr;
            cos_sin_ptr = cos_sin_ptr.offset(1);
            sin = *fresh30;
            let fresh31 = cos_sin_ptr;
            cos_sin_ptr = cos_sin_ptr.offset(1);
            cos = *fresh31;
            let fresh32 = spec_data1;
            spec_data1 = spec_data1.offset(-1);
            tempi = *fresh32;
            let fresh33 = spec_data1;
            spec_data1 = spec_data1.offset(-1);
            tempr = *fresh33;
            outi = ixheaac_sub32(
                ixheaac_mult32x16in32(tempr, sin),
                ixheaac_mult32x16in32(tempi, cos),
            );
            outr = ixheaac_mac32x16in32(ixheaac_mult32x16in32(tempr, cos), tempi, sin);
            temp1 = ixheaac_mult32x16in32(outi, adjust1);
            temp2 = ixheaac_mult32x16in32(outr, adjust);
            outr = outr + temp1;
            outi = outi + temp2;
            let fresh34 = out_ptr;
            out_ptr = out_ptr.offset(1);
            *fresh34 = outi;
            let fresh35 = out_ptr1;
            out_ptr1 = out_ptr1.offset(-1);
            *fresh35 = outr;
            i += 1;
            let fresh36 = spec_data;
            spec_data = spec_data.offset(1);
            tempr = *fresh36;
            let fresh37 = spec_data;
            spec_data = spec_data.offset(1);
            tempi = *fresh37;
            outi = ixheaac_sub32(
                ixheaac_mult32x16in32(tempr, cos),
                ixheaac_mult32x16in32(tempi, sin),
            );
            outr = ixheaac_mac32x16in32(ixheaac_mult32x16in32(tempr, sin), tempi, cos);
            temp1 = ixheaac_mult32x16in32(outi, adjust1);
            temp2 = ixheaac_mult32x16in32(outr, adjust);
            outr = outr + temp1;
            outi = outi + temp2;
            let fresh38 = out_ptr1;
            out_ptr1 = out_ptr1.offset(-1);
            *fresh38 = outi;
            let fresh39 = out_ptr;
            out_ptr = out_ptr.offset(1);
            *fresh39 = outr;
            i += 1;
        }
        let fresh40 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        cos1 = *fresh40;
        sin1 = *cos_sin_ptr;
        let fresh41 = spec_data1;
        spec_data1 = spec_data1.offset(-1);
        tempi = *fresh41;
        let fresh42 = spec_data1;
        spec_data1 = spec_data1.offset(-1);
        tempr = *fresh42;
        outi = ixheaac_sub32(
            ixheaac_mult32x16in32(tempr, sin1),
            ixheaac_mult32x16in32(tempi, cos1),
        );
        outr = ixheaac_mac32x16in32(ixheaac_mult32x16in32(tempr, cos1), tempi, sin1);
        temp1 = ixheaac_mult32x16in32(outi, adjust1);
        temp2 = ixheaac_mult32x16in32(outr, adjust);
        outr = outr + temp1;
        outi = outi + temp2;
        let fresh43 = out_ptr;
        out_ptr = out_ptr.offset(1);
        *fresh43 = outi;
        let fresh44 = out_ptr1;
        out_ptr1 = out_ptr1.offset(-1);
        *fresh44 = outr;
    } else if npoints == 128 as core::ffi::c_int {
        let mut tempr_0: WORD32 = 0;
        let mut tempi_0: WORD32 = 0;
        let mut outi_0: WORD32 = 0;
        let mut outr_0: WORD32 = 0;
        let mut temp1_0: WORD32 = 0;
        let mut temp2_0: WORD32 = 0;
        let fresh45 = spec_data;
        spec_data = spec_data.offset(1);
        tempr_0 = *fresh45;
        let fresh46 = spec_data;
        spec_data = spec_data.offset(1);
        tempi_0 = *fresh46;
        let fresh47 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        cos = *fresh47;
        sin = *cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(15 as core::ffi::c_int as isize);
        outi_0 = ixheaac_sub32(
            ixheaac_mult32x16in32(tempr_0, sin),
            ixheaac_mult32x16in32(tempi_0, cos),
        );
        outr_0 = ixheaac_mac32x16in32(ixheaac_mult32x16in32(tempr_0, cos), tempi_0, sin);
        temp1_0 = ixheaac_mult32x16in32(
            outi_0,
            -((201 as core::ffi::c_int) << 1 as core::ffi::c_int) as WORD16,
        );
        temp2_0 = ixheaac_mult32x16in32(
            outr_0,
            ((201 as core::ffi::c_int) << 1 as core::ffi::c_int) as WORD16,
        );
        outr_0 = outr_0 + temp1_0;
        outi_0 = outi_0 + temp2_0;
        let fresh48 = out_ptr1;
        out_ptr1 = out_ptr1.offset(-1);
        *fresh48 = outi_0;
        let fresh49 = out_ptr;
        out_ptr = out_ptr.offset(1);
        *fresh49 = outr_0;
        i = 0 as core::ffi::c_int as WORD;
        while i
            < npoints as core::ffi::c_int / 2 as core::ffi::c_int - 2 as core::ffi::c_int
        {
            let fresh50 = cos_sin_ptr;
            cos_sin_ptr = cos_sin_ptr.offset(1);
            sin = *fresh50;
            cos = *cos_sin_ptr;
            cos_sin_ptr = cos_sin_ptr.offset(15 as core::ffi::c_int as isize);
            let fresh51 = spec_data1;
            spec_data1 = spec_data1.offset(-1);
            tempi_0 = *fresh51;
            let fresh52 = spec_data1;
            spec_data1 = spec_data1.offset(-1);
            tempr_0 = *fresh52;
            outi_0 = ixheaac_sub32(
                ixheaac_mult32x16in32(tempr_0, sin),
                ixheaac_mult32x16in32(tempi_0, cos),
            );
            outr_0 = ixheaac_mac32x16in32(
                ixheaac_mult32x16in32(tempr_0, cos),
                tempi_0,
                sin,
            );
            temp1_0 = ixheaac_mult32x16in32(
                outi_0,
                -((201 as core::ffi::c_int) << 1 as core::ffi::c_int) as WORD16,
            );
            temp2_0 = ixheaac_mult32x16in32(
                outr_0,
                ((201 as core::ffi::c_int) << 1 as core::ffi::c_int) as WORD16,
            );
            outr_0 = outr_0 + temp1_0;
            outi_0 = outi_0 + temp2_0;
            let fresh53 = out_ptr;
            out_ptr = out_ptr.offset(1);
            *fresh53 = outi_0;
            let fresh54 = out_ptr1;
            out_ptr1 = out_ptr1.offset(-1);
            *fresh54 = outr_0;
            i += 1;
            let fresh55 = spec_data;
            spec_data = spec_data.offset(1);
            tempr_0 = *fresh55;
            let fresh56 = spec_data;
            spec_data = spec_data.offset(1);
            tempi_0 = *fresh56;
            outi_0 = ixheaac_sub32(
                ixheaac_mult32x16in32(tempr_0, cos),
                ixheaac_mult32x16in32(tempi_0, sin),
            );
            outr_0 = ixheaac_mac32x16in32(
                ixheaac_mult32x16in32(tempr_0, sin),
                tempi_0,
                cos,
            );
            temp1_0 = ixheaac_mult32x16in32(
                outi_0,
                -((201 as core::ffi::c_int) << 1 as core::ffi::c_int) as WORD16,
            );
            temp2_0 = ixheaac_mult32x16in32(
                outr_0,
                ((201 as core::ffi::c_int) << 1 as core::ffi::c_int) as WORD16,
            );
            outr_0 = outr_0 + temp1_0;
            outi_0 = outi_0 + temp2_0;
            let fresh57 = out_ptr1;
            out_ptr1 = out_ptr1.offset(-1);
            *fresh57 = outi_0;
            let fresh58 = out_ptr;
            out_ptr = out_ptr.offset(1);
            *fresh58 = outr_0;
            i += 1;
        }
        let fresh59 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        cos1 = *fresh59;
        sin1 = *cos_sin_ptr;
        let fresh60 = spec_data1;
        spec_data1 = spec_data1.offset(-1);
        tempi_0 = *fresh60;
        let fresh61 = spec_data1;
        spec_data1 = spec_data1.offset(-1);
        tempr_0 = *fresh61;
        outi_0 = ixheaac_sub32(
            ixheaac_mult32x16in32(tempr_0, sin1),
            ixheaac_mult32x16in32(tempi_0, cos1),
        );
        outr_0 = ixheaac_mac32x16in32(
            ixheaac_mult32x16in32(tempr_0, cos1),
            tempi_0,
            sin1,
        );
        temp1_0 = ixheaac_mult32x16in32(
            outi_0,
            -((201 as core::ffi::c_int) << 1 as core::ffi::c_int) as WORD16,
        );
        temp2_0 = ixheaac_mult32x16in32(
            outr_0,
            ((201 as core::ffi::c_int) << 1 as core::ffi::c_int) as WORD16,
        );
        outr_0 = outr_0 + temp1_0;
        outi_0 = outi_0 + temp2_0;
        let fresh62 = out_ptr;
        out_ptr = out_ptr.offset(1);
        *fresh62 = outi_0;
        let fresh63 = out_ptr1;
        out_ptr1 = out_ptr1.offset(-1);
        *fresh63 = outr_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_post_twid_overlap_add_dec(
    mut pcm_out: *mut WORD32,
    mut spec_data: *mut WORD32,
    mut ptr_imdct_tables: *mut ia_aac_dec_imdct_tables_struct,
    mut npoints: WORD,
    mut ptr_overlap_buf: *mut WORD32,
    mut q_shift: WORD16,
    mut window: *const WORD16,
    mut ch_fac: WORD16,
) -> VOID {
    let mut i: WORD = 0;
    let mut cos: WORD16 = 0;
    let mut cos1: WORD16 = 0;
    let mut sin: WORD16 = 0;
    let mut sin1: WORD16 = 0;
    let mut size: WORD32 = npoints as WORD32 / 2 as WORD32;
    let mut pcmout1: *mut WORD32 = pcm_out.offset((ch_fac as WORD32 * size) as isize);
    let mut cos_sin_ptr: *const WORD16 = ((*ptr_imdct_tables).cosine_array_2048_256)
        .as_mut_ptr();
    pcm_out = pcmout1.offset(-(ch_fac as core::ffi::c_int as isize)) as *mut WORD32;
    spec_data = spec_data.offset(size as isize);
    if q_shift as core::ffi::c_int > 0 as core::ffi::c_int {
        let mut tempr: WORD32 = 0;
        let mut tempi: WORD32 = 0;
        let mut outr: WORD32 = 0;
        let mut outi: WORD32 = 0;
        let mut win1: WORD32 = 0;
        let mut accu: WORD32 = 0;
        let mut temp1: WORD32 = 0;
        let mut temp2: WORD32 = 0;
        let mut adjust: WORD16 = 0;
        let mut adjust1: WORD16 = 0;
        let mut overlap_data: WORD32 = 0;
        tempr = *spec_data.offset(-(size as isize));
        tempi = *spec_data
            .offset(-(size as isize))
            .offset(1 as core::ffi::c_int as isize);
        adjust = 50 as WORD16;
        adjust1 = -(50 as core::ffi::c_int) as WORD16;
        let fresh64 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        cos = *fresh64;
        let fresh65 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        sin = *fresh65;
        outi = ixheaac_sub32(
            ixheaac_mult32x16in32(tempr, sin),
            ixheaac_mult32x16in32(tempi, cos),
        );
        outr = ixheaac_add32(
            ixheaac_mult32x16in32(tempr, cos),
            ixheaac_mult32x16in32(tempi, sin),
        );
        overlap_data = *ptr_overlap_buf;
        temp1 = ixheaac_mult32x16in32(outi, adjust1);
        temp2 = ixheaac_mult32x16in32(outr, adjust);
        outr = outr + temp1;
        outi = outi + temp2;
        let fresh66 = ptr_overlap_buf;
        ptr_overlap_buf = ptr_overlap_buf.offset(1);
        *fresh66 = ixheaac_shr32_sat(outr, 16 as WORD32 - q_shift as WORD32);
        win1 = *(window as *mut WORD32)
            .offset(size as isize)
            .offset(-(1 as core::ffi::c_int as isize));
        accu = ixheaac_sub32_sat(
            ixheaac_shl32_sat(ixheaacd_mult32x16lin32(outi, win1), q_shift as WORD),
            ixheaacd_mult32x16lin32_sat(
                overlap_data,
                (win1 >> 16 as core::ffi::c_int) as WORD16 as WORD32,
            ),
        );
        *pcm_out = accu;
        pcm_out = pcm_out.offset(-(ch_fac as core::ffi::c_int as isize));
        accu = ixheaac_sub32_sat(
            ixheaac_shl32_sat(
                ixheaac_mult32x16hin32(ixheaac_negate32_sat(outi), win1),
                q_shift as WORD,
            ),
            ixheaacd_mult32x16lin32_sat(overlap_data, win1 as WORD16 as WORD32),
        );
        *pcmout1 = accu;
        pcmout1 = pcmout1.offset(ch_fac as core::ffi::c_int as isize);
        i = (size as core::ffi::c_int - 2 as core::ffi::c_int) as WORD;
        while i != 0 as core::ffi::c_int {
            let fresh67 = cos_sin_ptr;
            cos_sin_ptr = cos_sin_ptr.offset(1);
            sin = *fresh67;
            let fresh68 = cos_sin_ptr;
            cos_sin_ptr = cos_sin_ptr.offset(1);
            cos = *fresh68;
            tempr = *spec_data.offset(i as isize);
            tempi = *spec_data.offset(i as isize).offset(1 as core::ffi::c_int as isize);
            outr = ixheaac_add32(
                ixheaac_mult32x16in32(tempr, cos),
                ixheaac_mult32x16in32(tempi, sin),
            );
            outi = ixheaac_sub32(
                ixheaac_mult32x16in32(tempr, sin),
                ixheaac_mult32x16in32(tempi, cos),
            );
            temp1 = ixheaac_mult32x16in32(outi, adjust1);
            temp2 = ixheaac_mult32x16in32(outr, adjust);
            outr = outr + temp1;
            outi = outi + temp2;
            overlap_data = *ptr_overlap_buf;
            let fresh69 = ptr_overlap_buf;
            ptr_overlap_buf = ptr_overlap_buf.offset(1);
            *fresh69 = ixheaac_shr32_sat(outi, 16 as WORD32 - q_shift as WORD32);
            win1 = *(window as *mut WORD32).offset(i as isize);
            accu = ixheaac_sub32_sat(
                ixheaac_shl32_sat(ixheaacd_mult32x16lin32(outr, win1), q_shift as WORD),
                ixheaacd_mult32x16lin32_sat(
                    overlap_data,
                    (win1 >> 16 as core::ffi::c_int) as WORD16 as WORD32,
                ),
            );
            *pcm_out = accu;
            pcm_out = pcm_out.offset(-(ch_fac as core::ffi::c_int as isize));
            accu = ixheaac_sub32_sat(
                ixheaac_shl32_sat(
                    ixheaac_mult32x16hin32(ixheaac_negate32_sat(outr), win1),
                    q_shift as WORD,
                ),
                ixheaacd_mult32x16lin32_sat(overlap_data, win1 as WORD16 as WORD32),
            );
            *pcmout1 = accu;
            pcmout1 = pcmout1.offset(ch_fac as core::ffi::c_int as isize);
            tempr = *spec_data.offset(-(i as isize));
            tempi = *spec_data
                .offset(-(i as isize))
                .offset(1 as core::ffi::c_int as isize);
            i -= 2 as core::ffi::c_int;
            outi = ixheaac_sub32(
                ixheaac_mult32x16in32(tempr, cos),
                ixheaac_mult32x16in32(tempi, sin),
            );
            outr = ixheaac_add32(
                ixheaac_mult32x16in32(tempr, sin),
                ixheaac_mult32x16in32(tempi, cos),
            );
            overlap_data = *ptr_overlap_buf;
            temp1 = ixheaac_mult32x16in32(outi, adjust1);
            temp2 = ixheaac_mult32x16in32(outr, adjust);
            outr = outr + temp1;
            outi = outi + temp2;
            let fresh70 = ptr_overlap_buf;
            ptr_overlap_buf = ptr_overlap_buf.offset(1);
            *fresh70 = ixheaac_shr32_sat(outr, 16 as WORD32 - q_shift as WORD32);
            win1 = *(window as *mut WORD32)
                .offset(i as isize)
                .offset(1 as core::ffi::c_int as isize);
            accu = ixheaac_sub32_sat(
                ixheaac_shl32_sat(ixheaacd_mult32x16lin32(outi, win1), q_shift as WORD),
                ixheaacd_mult32x16lin32_sat(
                    overlap_data,
                    (win1 >> 16 as core::ffi::c_int) as WORD16 as WORD32,
                ),
            );
            *pcm_out = accu;
            pcm_out = pcm_out.offset(-(ch_fac as core::ffi::c_int as isize));
            accu = ixheaac_sub32_sat(
                ixheaac_shl32_sat(
                    ixheaac_mult32x16hin32(ixheaac_negate32_sat(outi), win1),
                    q_shift as WORD,
                ),
                ixheaacd_mult32x16lin32_sat(overlap_data, win1 as WORD16 as WORD32),
            );
            *pcmout1 = accu;
            pcmout1 = pcmout1.offset(ch_fac as core::ffi::c_int as isize);
        }
        let fresh71 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        cos1 = *fresh71;
        sin1 = *cos_sin_ptr;
        tempr = *spec_data.offset(i as isize);
        tempi = *spec_data.offset(i as isize).offset(1 as core::ffi::c_int as isize);
        outr = ixheaac_add32(
            ixheaac_mult32x16in32(tempr, cos1),
            ixheaac_mult32x16in32(tempi, sin1),
        );
        outi = ixheaac_sub32(
            ixheaac_mult32x16in32(tempr, sin1),
            ixheaac_mult32x16in32(tempi, cos1),
        );
        temp1 = ixheaac_mult32x16in32(outi, adjust1);
        temp2 = ixheaac_mult32x16in32(outr, adjust);
        outr = outr + temp1;
        outi = outi + temp2;
        overlap_data = *ptr_overlap_buf;
        let fresh72 = ptr_overlap_buf;
        ptr_overlap_buf = ptr_overlap_buf.offset(1);
        *fresh72 = ixheaac_shr32_sat(outi, 16 as WORD32 - q_shift as WORD32);
        win1 = *(window as *mut WORD32).offset(i as isize);
        accu = ixheaac_sub32_sat(
            ixheaac_shl32_sat(ixheaacd_mult32x16lin32(outr, win1), q_shift as WORD),
            ixheaacd_mult32x16lin32_sat(
                overlap_data,
                (win1 >> 16 as core::ffi::c_int) as WORD16 as WORD32,
            ),
        );
        *pcm_out = accu;
        pcm_out = pcm_out.offset(-(ch_fac as core::ffi::c_int as isize));
        accu = ixheaac_sub32_sat(
            ixheaac_shl32_sat(
                ixheaac_mult32x16hin32(ixheaac_negate32_sat(outr), win1),
                q_shift as WORD,
            ),
            ixheaacd_mult32x16lin32_sat(overlap_data, win1 as WORD16 as WORD32),
        );
        *pcmout1 = accu;
        pcmout1 = pcmout1.offset(ch_fac as core::ffi::c_int as isize);
    } else {
        q_shift = -(q_shift as core::ffi::c_int) as WORD16;
        let mut tempr_0: WORD32 = 0;
        let mut tempi_0: WORD32 = 0;
        let mut temp1_0: WORD32 = 0;
        let mut temp2_0: WORD32 = 0;
        let mut outr_0: WORD32 = 0;
        let mut outi_0: WORD32 = 0;
        let mut win1_0: WORD32 = 0;
        let mut accu_0: WORD32 = 0;
        let mut adjust_0: WORD16 = 0;
        let mut adjust1_0: WORD16 = 0;
        let mut overlap_data_0: WORD16 = 0;
        tempr_0 = *spec_data.offset(-(size as isize));
        tempi_0 = *spec_data
            .offset(-(size as isize))
            .offset(1 as core::ffi::c_int as isize);
        adjust_0 = 50 as WORD16;
        adjust1_0 = -(50 as core::ffi::c_int) as WORD16;
        let fresh73 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        cos = *fresh73;
        let fresh74 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        sin = *fresh74;
        outi_0 = ixheaac_sub32(
            ixheaac_mult32x16in32(tempr_0, sin),
            ixheaac_mult32x16in32(tempi_0, cos),
        );
        outr_0 = ixheaac_add32(
            ixheaac_mult32x16in32(tempr_0, cos),
            ixheaac_mult32x16in32(tempi_0, sin),
        );
        overlap_data_0 = *ptr_overlap_buf as WORD16;
        temp1_0 = ixheaac_mult32x16in32(outi_0, adjust1_0);
        temp2_0 = ixheaac_mult32x16in32(outr_0, adjust_0);
        outr_0 = outr_0 + temp1_0;
        outi_0 = outi_0 + temp2_0;
        let fresh75 = ptr_overlap_buf;
        ptr_overlap_buf = ptr_overlap_buf.offset(1);
        *fresh75 = ixheaac_shr32_sat(outr_0, 16 as WORD32 + q_shift as WORD32);
        win1_0 = *(window as *mut WORD32)
            .offset(size as isize)
            .offset(-(1 as core::ffi::c_int as isize));
        accu_0 = ixheaac_sub32_sat(
            ixheaac_shr32(ixheaacd_mult32x16lin32(outi_0, win1_0), q_shift as WORD),
            ixheaacd_mult32x16lin32_sat(
                overlap_data_0 as WORD32,
                (win1_0 >> 16 as core::ffi::c_int) as WORD16 as WORD32,
            ),
        );
        *pcm_out = accu_0;
        pcm_out = pcm_out.offset(-(ch_fac as core::ffi::c_int as isize));
        accu_0 = ixheaac_sub32_sat(
            ixheaac_shr32(
                ixheaac_mult32x16hin32(ixheaac_negate32_sat(outi_0), win1_0),
                q_shift as WORD,
            ),
            ixheaacd_mult32x16lin32_sat(
                overlap_data_0 as WORD32,
                win1_0 as WORD16 as WORD32,
            ),
        );
        *pcmout1 = accu_0;
        pcmout1 = pcmout1.offset(ch_fac as core::ffi::c_int as isize);
        i = (size as core::ffi::c_int - 2 as core::ffi::c_int) as WORD;
        while i != 0 as core::ffi::c_int {
            let fresh76 = cos_sin_ptr;
            cos_sin_ptr = cos_sin_ptr.offset(1);
            sin = *fresh76;
            let fresh77 = cos_sin_ptr;
            cos_sin_ptr = cos_sin_ptr.offset(1);
            cos = *fresh77;
            tempr_0 = *spec_data.offset(i as isize);
            tempi_0 = *spec_data
                .offset(i as isize)
                .offset(1 as core::ffi::c_int as isize);
            outr_0 = ixheaac_add32(
                ixheaac_mult32x16in32(tempr_0, cos),
                ixheaac_mult32x16in32(tempi_0, sin),
            );
            outi_0 = ixheaac_sub32(
                ixheaac_mult32x16in32(tempr_0, sin),
                ixheaac_mult32x16in32(tempi_0, cos),
            );
            overlap_data_0 = *ptr_overlap_buf as WORD16;
            temp1_0 = ixheaac_mult32x16in32(outi_0, adjust1_0);
            temp2_0 = ixheaac_mult32x16in32(outr_0, adjust_0);
            outr_0 = outr_0 + temp1_0;
            outi_0 = outi_0 + temp2_0;
            let fresh78 = ptr_overlap_buf;
            ptr_overlap_buf = ptr_overlap_buf.offset(1);
            *fresh78 = ixheaac_shr32_sat(outi_0, 16 as WORD32 + q_shift as WORD32);
            win1_0 = *(window as *mut WORD32).offset(i as isize);
            accu_0 = ixheaac_sub32_sat(
                ixheaac_shr32(ixheaacd_mult32x16lin32(outr_0, win1_0), q_shift as WORD),
                ixheaacd_mult32x16lin32_sat(
                    overlap_data_0 as WORD32,
                    (win1_0 >> 16 as core::ffi::c_int) as WORD16 as WORD32,
                ),
            );
            *pcm_out = accu_0;
            pcm_out = pcm_out.offset(-(ch_fac as core::ffi::c_int as isize));
            accu_0 = ixheaac_sub32_sat(
                ixheaac_shr32(
                    ixheaac_mult32x16hin32(ixheaac_negate32_sat(outr_0), win1_0),
                    q_shift as WORD,
                ),
                ixheaacd_mult32x16lin32_sat(
                    overlap_data_0 as WORD32,
                    win1_0 as WORD16 as WORD32,
                ),
            );
            *pcmout1 = accu_0;
            pcmout1 = pcmout1.offset(ch_fac as core::ffi::c_int as isize);
            tempr_0 = *spec_data.offset(-(i as isize));
            tempi_0 = *spec_data
                .offset(-(i as isize))
                .offset(1 as core::ffi::c_int as isize);
            i -= 2 as core::ffi::c_int;
            outi_0 = ixheaac_sub32(
                ixheaac_mult32x16in32(tempr_0, cos),
                ixheaac_mult32x16in32(tempi_0, sin),
            );
            outr_0 = ixheaac_add32(
                ixheaac_mult32x16in32(tempr_0, sin),
                ixheaac_mult32x16in32(tempi_0, cos),
            );
            overlap_data_0 = *ptr_overlap_buf as WORD16;
            temp1_0 = ixheaac_mult32x16in32(outi_0, adjust1_0);
            temp2_0 = ixheaac_mult32x16in32(outr_0, adjust_0);
            outr_0 = outr_0 + temp1_0;
            outi_0 = outi_0 + temp2_0;
            let fresh79 = ptr_overlap_buf;
            ptr_overlap_buf = ptr_overlap_buf.offset(1);
            *fresh79 = ixheaac_shr32_sat(outr_0, 16 as WORD32 + q_shift as WORD32);
            win1_0 = *(window as *mut WORD32)
                .offset(i as isize)
                .offset(1 as core::ffi::c_int as isize);
            accu_0 = ixheaac_sub32_sat(
                ixheaac_shr32(ixheaacd_mult32x16lin32(outi_0, win1_0), q_shift as WORD),
                ixheaacd_mult32x16lin32_sat(
                    overlap_data_0 as WORD32,
                    (win1_0 >> 16 as core::ffi::c_int) as WORD16 as WORD32,
                ),
            );
            *pcm_out = accu_0;
            pcm_out = pcm_out.offset(-(ch_fac as core::ffi::c_int as isize));
            accu_0 = ixheaac_sub32_sat(
                ixheaac_shr32(
                    ixheaac_mult32x16hin32(ixheaac_negate32_sat(outi_0), win1_0),
                    q_shift as WORD,
                ),
                ixheaacd_mult32x16lin32_sat(
                    overlap_data_0 as WORD32,
                    win1_0 as WORD16 as WORD32,
                ),
            );
            *pcmout1 = accu_0;
            pcmout1 = pcmout1.offset(ch_fac as core::ffi::c_int as isize);
        }
        let fresh80 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        cos1 = *fresh80;
        let fresh81 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        sin1 = *fresh81;
        tempr_0 = *spec_data.offset(i as isize);
        tempi_0 = *spec_data.offset(i as isize).offset(1 as core::ffi::c_int as isize);
        outr_0 = ixheaac_add32(
            ixheaac_mult32x16in32(tempr_0, cos1),
            ixheaac_mult32x16in32(tempi_0, sin1),
        );
        outi_0 = ixheaac_sub32(
            ixheaac_mult32x16in32(tempr_0, sin1),
            ixheaac_mult32x16in32(tempi_0, cos1),
        );
        overlap_data_0 = *ptr_overlap_buf as WORD16;
        temp1_0 = ixheaac_mult32x16in32(outi_0, adjust1_0);
        temp2_0 = ixheaac_mult32x16in32(outr_0, adjust_0);
        outr_0 = outr_0 + temp1_0;
        outi_0 = outi_0 + temp2_0;
        let fresh82 = ptr_overlap_buf;
        ptr_overlap_buf = ptr_overlap_buf.offset(1);
        *fresh82 = ixheaac_shr32_sat(outi_0, 16 as WORD32 + q_shift as WORD32);
        win1_0 = *(window as *mut WORD32).offset(i as isize);
        accu_0 = ixheaac_sub32_sat(
            ixheaac_shr32(ixheaacd_mult32x16lin32(outr_0, win1_0), q_shift as WORD),
            ixheaacd_mult32x16lin32_sat(
                overlap_data_0 as WORD32,
                (win1_0 >> 16 as core::ffi::c_int) as WORD16 as WORD32,
            ),
        );
        *pcm_out = accu_0;
        pcm_out = pcm_out.offset(-(ch_fac as core::ffi::c_int as isize));
        accu_0 = ixheaac_sub32_sat(
            ixheaac_shr32(
                ixheaac_mult32x16hin32(ixheaac_negate32_sat(outr_0), win1_0),
                q_shift as WORD,
            ),
            ixheaacd_mult32x16lin32_sat(
                overlap_data_0 as WORD32,
                win1_0 as WORD16 as WORD32,
            ),
        );
        *pcmout1 = accu_0;
        pcmout1 = pcmout1.offset(ch_fac as core::ffi::c_int as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_imdct_using_fft_dec(
    mut ptr_imdct_tables: *mut ia_aac_dec_imdct_tables_struct,
    mut npoints: WORD32,
    mut ptr_x: *mut WORD32,
    mut ptr_y: *mut WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut k1: WORD32 = 0;
    let mut n_stages: WORD32 = 0;
    let mut x0r: WORD32 = 0;
    let mut x0i: WORD32 = 0;
    let mut x1r: WORD32 = 0;
    let mut x1i: WORD32 = 0;
    let mut x2r: WORD32 = 0;
    let mut x2i: WORD32 = 0;
    let mut x3r: WORD32 = 0;
    let mut x3i: WORD32 = 0;
    let mut x4r: WORD32 = 0;
    let mut x4i: WORD32 = 0;
    let mut x5r: WORD32 = 0;
    let mut x5i: WORD32 = 0;
    let mut x6r: WORD32 = 0;
    let mut x6i: WORD32 = 0;
    let mut x7r: WORD32 = 0;
    let mut x7i: WORD32 = 0;
    let mut del: WORD32 = 0;
    let mut nodespacing: WORD32 = 0;
    let mut in_loop_cnt: WORD32 = 0;
    let mut tmp: WORD32 = 0;
    let mut twiddle_val: WORD32 = 0;
    let mut ptr_tmp: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr_twiddle: *const WORD32 = 0 as *const WORD32;
    let mut ptr_dig_rev_table: *mut WORD8 = 0 as *mut WORD8;
    n_stages = ixheaac_norm32(npoints) as WORD32;
    n_stages = ((30 as core::ffi::c_int - n_stages as core::ffi::c_int)
        / 3 as core::ffi::c_int) as WORD32;
    ptr_tmp = ptr_y;
    ptr_twiddle = ((*ptr_imdct_tables).fft_twiddle).as_mut_ptr();
    ptr_dig_rev_table = if npoints << 1 as core::ffi::c_int == 1024 as core::ffi::c_int {
        ((*ptr_imdct_tables).dig_rev_table8_long).as_mut_ptr()
    } else {
        ((*ptr_imdct_tables).dig_rev_table8_short).as_mut_ptr()
    };
    i = npoints;
    while i != 0 as core::ffi::c_int {
        let mut data: *mut WORD32 = ptr_x;
        let fresh117 = ptr_dig_rev_table;
        ptr_dig_rev_table = ptr_dig_rev_table.offset(1);
        data = data
            .offset(((*fresh117 as core::ffi::c_int) << 1 as core::ffi::c_int) as isize);
        x0r = *data;
        x0i = *data.offset(1 as core::ffi::c_int as isize);
        data = data.offset((npoints >> 1 as core::ffi::c_int) as isize);
        x2r = *data;
        x2i = *data.offset(1 as core::ffi::c_int as isize);
        data = data.offset((npoints >> 1 as core::ffi::c_int) as isize);
        x4r = *data;
        x4i = *data.offset(1 as core::ffi::c_int as isize);
        data = data.offset((npoints >> 1 as core::ffi::c_int) as isize);
        x6r = *data;
        x6i = *data.offset(1 as core::ffi::c_int as isize);
        data = data
            .offset(-((5 as WORD32 * (npoints >> 2 as core::ffi::c_int)) as isize));
        x0r = x0r + x4r;
        x0i = x0i + x4i;
        x4r = x0r - (x4r << 1 as core::ffi::c_int);
        x4i = x0i - (x4i << 1 as core::ffi::c_int);
        x2r = x2r + x6r;
        x2i = x2i + x6i;
        x6r = x2r - (x6r << 1 as core::ffi::c_int);
        x6i = x2i - (x6i << 1 as core::ffi::c_int);
        x0r = x0r + x2r;
        x0i = x0i + x2i;
        x2r = x0r - (x2r << 1 as core::ffi::c_int);
        x2i = x0i - (x2i << 1 as core::ffi::c_int);
        x4r = x4r + x6i;
        x4i = x4i - x6r;
        tmp = x6r;
        x6r = x4r - (x6i << 1 as core::ffi::c_int);
        x6i = x4i + (tmp << 1 as core::ffi::c_int);
        x1r = *data;
        x1i = *data.offset(1 as core::ffi::c_int as isize);
        data = data.offset((npoints >> 1 as core::ffi::c_int) as isize);
        x3r = *data;
        x3i = *data.offset(1 as core::ffi::c_int as isize);
        data = data.offset((npoints >> 1 as core::ffi::c_int) as isize);
        x5r = *data;
        x5i = *data.offset(1 as core::ffi::c_int as isize);
        data = data.offset((npoints >> 1 as core::ffi::c_int) as isize);
        x7r = *data;
        x7i = *data.offset(1 as core::ffi::c_int as isize);
        data = data
            .offset(-((7 as WORD32 * (npoints >> 2 as core::ffi::c_int)) as isize));
        x1r = x1r + x5r;
        x1i = x1i + x5i;
        x5r = x1r - (x5r << 1 as core::ffi::c_int);
        x5i = x1i - (x5i << 1 as core::ffi::c_int);
        x3r = x3r + x7r;
        x3i = x3i + x7i;
        x7r = x3r - (x7r << 1 as core::ffi::c_int);
        x7i = x3i - (x7i << 1 as core::ffi::c_int);
        x1r = x1r + x3r;
        x1i = x1i + x3i;
        x3r = x1r - (x3r << 1 as core::ffi::c_int);
        x3i = x1i - (x3i << 1 as core::ffi::c_int);
        x5r = x5r + x5i;
        x5i = x5r - (x5i << 1 as core::ffi::c_int);
        x7r = x7r + x7i;
        x7i = x7r - (x7i << 1 as core::ffi::c_int);
        x7i = x5r - x7i;
        x5r = x7i - (x5r << 1 as core::ffi::c_int);
        x5i = x7r - x5i;
        x7r = x5i - (x7r << 1 as core::ffi::c_int);
        x7i = x7i << 1 as core::ffi::c_int;
        x5r = x5r << 1 as core::ffi::c_int;
        x5i = x5i << 1 as core::ffi::c_int;
        x7r = x7r << 1 as core::ffi::c_int;
        x0r = x0r + x1r;
        x0i = x0i + x1i;
        x1r = x0r - (x1r << 1 as core::ffi::c_int);
        x1i = x0i - (x1i << 1 as core::ffi::c_int);
        x2r = x2r + x3i;
        tmp = x2r - (x3i << 1 as core::ffi::c_int);
        x2i = x2i - x3r;
        x3i = x2i + (x3r << 1 as core::ffi::c_int);
        *ptr_tmp = x0r;
        *ptr_tmp.offset(1 as core::ffi::c_int as isize) = x0i;
        ptr_tmp = ptr_tmp.offset(4 as core::ffi::c_int as isize);
        *ptr_tmp = x2r;
        *ptr_tmp.offset(1 as core::ffi::c_int as isize) = x2i;
        ptr_tmp = ptr_tmp.offset(4 as core::ffi::c_int as isize);
        *ptr_tmp = x1r;
        *ptr_tmp.offset(1 as core::ffi::c_int as isize) = x1i;
        ptr_tmp = ptr_tmp.offset(4 as core::ffi::c_int as isize);
        *ptr_tmp = tmp;
        *ptr_tmp.offset(1 as core::ffi::c_int as isize) = x3i;
        ptr_tmp = ptr_tmp.offset(-(10 as core::ffi::c_int as isize));
        tmp = 0x5a82 as core::ffi::c_int as WORD32;
        x7i = x4r + ixheaacd_mult32x16lin32(x7i, tmp);
        x4r = x7i - (x4r << 1 as core::ffi::c_int);
        x7r = x4i + ixheaacd_mult32x16lin32(x7r, tmp);
        x4i = x7r - (x4i << 1 as core::ffi::c_int);
        x5i = x6r + ixheaacd_mult32x16lin32(x5i, tmp);
        x6r = x5i - (x6r << 1 as core::ffi::c_int);
        x5r = x6i + ixheaacd_mult32x16lin32(x5r, tmp);
        x6i = x5r - (x6i << 1 as core::ffi::c_int);
        *ptr_tmp = x7i;
        *ptr_tmp.offset(1 as core::ffi::c_int as isize) = x7r;
        ptr_tmp = ptr_tmp.offset(4 as core::ffi::c_int as isize);
        *ptr_tmp = x5i;
        *ptr_tmp.offset(1 as core::ffi::c_int as isize) = x5r;
        ptr_tmp = ptr_tmp.offset(4 as core::ffi::c_int as isize);
        *ptr_tmp = -x4r;
        *ptr_tmp.offset(1 as core::ffi::c_int as isize) = -x4i;
        ptr_tmp = ptr_tmp.offset(4 as core::ffi::c_int as isize);
        *ptr_tmp = -x6r;
        *ptr_tmp.offset(1 as core::ffi::c_int as isize) = -x6i;
        ptr_tmp = ptr_tmp.offset(2 as core::ffi::c_int as isize);
        i -= 8 as core::ffi::c_int;
    }
    del = 8 as core::ffi::c_int as WORD32;
    nodespacing = 64 as core::ffi::c_int as WORD32;
    in_loop_cnt = npoints >> 6 as core::ffi::c_int;
    k1 = (n_stages as core::ffi::c_int - 2 as core::ffi::c_int) as WORD32;
    while k1 > 0 as core::ffi::c_int {
        let mut data_0: *mut WORD32 = ptr_y;
        let mut twiddles: *const WORD32 = 0 as *const WORD32;
        i = 0 as core::ffi::c_int as WORD32;
        while i != npoints {
            data_0 = ptr_y.offset((i << 1 as core::ffi::c_int) as isize);
            x0r = *data_0;
            x0i = *data_0.offset(1 as core::ffi::c_int as isize);
            data_0 = data_0.offset((del << 2 as core::ffi::c_int) as isize);
            x2r = *data_0;
            x2i = *data_0.offset(1 as core::ffi::c_int as isize);
            data_0 = data_0.offset((del << 2 as core::ffi::c_int) as isize);
            x4r = *data_0;
            x4i = *data_0.offset(1 as core::ffi::c_int as isize);
            data_0 = data_0.offset((del << 2 as core::ffi::c_int) as isize);
            x6r = *data_0;
            x6i = *data_0.offset(1 as core::ffi::c_int as isize);
            data_0 = data_0
                .offset(-((5 as WORD32 * (del << 1 as core::ffi::c_int)) as isize));
            x0r = x0r + x4r;
            x0i = x0i + x4i;
            x4r = x0r - (x4r << 1 as core::ffi::c_int);
            x4i = x0i - (x4i << 1 as core::ffi::c_int);
            x2r = x2r + x6r;
            x2i = x2i + x6i;
            x6r = x2r - (x6r << 1 as core::ffi::c_int);
            x6i = x2i - (x6i << 1 as core::ffi::c_int);
            x0r = x0r + x2r;
            x0i = x0i + x2i;
            x2r = x0r - (x2r << 1 as core::ffi::c_int);
            x2i = x0i - (x2i << 1 as core::ffi::c_int);
            x4r = x4r + x6i;
            x4i = x4i - x6r;
            tmp = x6r;
            x6r = x4r - (x6i << 1 as core::ffi::c_int);
            x6i = x4i + (tmp << 1 as core::ffi::c_int);
            x1r = *data_0;
            x1i = *data_0.offset(1 as core::ffi::c_int as isize);
            data_0 = data_0.offset((del << 2 as core::ffi::c_int) as isize);
            x3r = *data_0;
            x3i = *data_0.offset(1 as core::ffi::c_int as isize);
            data_0 = data_0.offset((del << 2 as core::ffi::c_int) as isize);
            x5r = *data_0;
            x5i = *data_0.offset(1 as core::ffi::c_int as isize);
            data_0 = data_0.offset((del << 2 as core::ffi::c_int) as isize);
            x7r = *data_0;
            x7i = *data_0.offset(1 as core::ffi::c_int as isize);
            data_0 = data_0
                .offset(-((7 as WORD32 * (del << 1 as core::ffi::c_int)) as isize));
            x1r = x1r + x5r;
            x1i = x1i + x5i;
            x5r = x1r - (x5r << 1 as core::ffi::c_int);
            x5i = x1i - (x5i << 1 as core::ffi::c_int);
            x3r = x3r + x7r;
            x3i = x3i + x7i;
            x7r = x3r - (x7r << 1 as core::ffi::c_int);
            x7i = x3i - (x7i << 1 as core::ffi::c_int);
            x1r = x1r + x3r;
            x1i = x1i + x3i;
            x3r = x1r - (x3r << 1 as core::ffi::c_int);
            x3i = x1i - (x3i << 1 as core::ffi::c_int);
            x5r = x5r + x5i;
            x5i = x5r - (x5i << 1 as core::ffi::c_int);
            x7r = x7r + x7i;
            x7i = x7r - (x7i << 1 as core::ffi::c_int);
            x7i = x5r - x7i;
            x5r = x7i - (x5r << 1 as core::ffi::c_int);
            x5i = x7r - x5i;
            x7r = x5i - (x7r << 1 as core::ffi::c_int);
            x7i = x7i << 1 as core::ffi::c_int;
            x5r = x5r << 1 as core::ffi::c_int;
            x5i = x5i << 1 as core::ffi::c_int;
            x7r = x7r << 1 as core::ffi::c_int;
            x0r = x0r + x1r;
            x0i = x0i + x1i;
            x1r = x0r - (x1r << 1 as core::ffi::c_int);
            x1i = x0i - (x1i << 1 as core::ffi::c_int);
            x2r = x2r + x3i;
            tmp = x2r - (x3i << 1 as core::ffi::c_int);
            x2i = x2i - x3r;
            x3i = x2i + (x3r << 1 as core::ffi::c_int);
            *data_0 = x0r;
            *data_0.offset(1 as core::ffi::c_int as isize) = x0i;
            data_0 = data_0.offset((del << 2 as core::ffi::c_int) as isize);
            *data_0 = x2r;
            *data_0.offset(1 as core::ffi::c_int as isize) = x2i;
            data_0 = data_0.offset((del << 2 as core::ffi::c_int) as isize);
            *data_0 = x1r;
            *data_0.offset(1 as core::ffi::c_int as isize) = x1i;
            data_0 = data_0.offset((del << 2 as core::ffi::c_int) as isize);
            *data_0 = tmp;
            *data_0.offset(1 as core::ffi::c_int as isize) = x3i;
            data_0 = data_0
                .offset(-((5 as WORD32 * (del << 1 as core::ffi::c_int)) as isize));
            tmp = 0x5a82 as core::ffi::c_int as WORD32;
            x7i = x4r + ixheaacd_mult32x16lin32(x7i, tmp);
            x4r = x7i - (x4r << 1 as core::ffi::c_int);
            x7r = x4i + ixheaacd_mult32x16lin32(x7r, tmp);
            x4i = x7r - (x4i << 1 as core::ffi::c_int);
            x5i = x6r + ixheaacd_mult32x16lin32(x5i, tmp);
            x6r = x5i - (x6r << 1 as core::ffi::c_int);
            x5r = x6i + ixheaacd_mult32x16lin32(x5r, tmp);
            x6i = x5r - (x6i << 1 as core::ffi::c_int);
            *data_0 = x7i;
            *data_0.offset(1 as core::ffi::c_int as isize) = x7r;
            data_0 = data_0.offset((del << 2 as core::ffi::c_int) as isize);
            *data_0 = x5i;
            *data_0.offset(1 as core::ffi::c_int as isize) = x5r;
            data_0 = data_0.offset((del << 2 as core::ffi::c_int) as isize);
            *data_0 = -x4r;
            *data_0.offset(1 as core::ffi::c_int as isize) = -x4i;
            data_0 = data_0.offset((del << 2 as core::ffi::c_int) as isize);
            *data_0 = -x6r;
            *data_0.offset(1 as core::ffi::c_int as isize) = -x6i;
            data_0 = data_0
                .offset(-((7 as WORD32 * (del << 1 as core::ffi::c_int)) as isize));
            i += (8 as WORD32 * del) as core::ffi::c_int;
        }
        twiddles = ptr_twiddle;
        data_0 = ptr_y;
        j = nodespacing;
        while j < nodespacing * del {
            data_0 = data_0.offset(2 as core::ffi::c_int as isize);
            k = in_loop_cnt;
            while k != 0 as core::ffi::c_int {
                data_0 = data_0.offset((del << 2 as core::ffi::c_int) as isize);
                x2r = *data_0;
                x2i = *data_0.offset(1 as core::ffi::c_int as isize);
                data_0 = data_0.offset((del << 2 as core::ffi::c_int) as isize);
                x4r = *data_0;
                x4i = *data_0.offset(1 as core::ffi::c_int as isize);
                data_0 = data_0.offset((del << 2 as core::ffi::c_int) as isize);
                x6r = *data_0;
                x6i = *data_0.offset(1 as core::ffi::c_int as isize);
                data_0 = data_0
                    .offset(-((6 as WORD32 * (del << 1 as core::ffi::c_int)) as isize));
                twiddles = twiddles.offset((j >> 2 as core::ffi::c_int) as isize);
                twiddle_val = *twiddles;
                tmp = ixheaacd_mult32x16lin32(x2r, twiddle_val)
                    - ixheaac_mult32x16hin32(x2i, twiddle_val);
                x2i = ixheaacd_mac32x16lin32(
                    ixheaac_mult32x16hin32(x2r, twiddle_val),
                    x2i,
                    twiddle_val,
                ) << 1 as core::ffi::c_int;
                x2r = tmp << 1 as core::ffi::c_int;
                twiddles = twiddles.offset((j >> 2 as core::ffi::c_int) as isize);
                twiddle_val = *twiddles;
                tmp = ixheaacd_mult32x16lin32(x4r, twiddle_val)
                    - ixheaac_mult32x16hin32(x4i, twiddle_val);
                x4i = ixheaacd_mac32x16lin32(
                    ixheaac_mult32x16hin32(x4r, twiddle_val),
                    x4i,
                    twiddle_val,
                ) << 1 as core::ffi::c_int;
                x4r = tmp << 1 as core::ffi::c_int;
                twiddles = twiddles.offset((j >> 2 as core::ffi::c_int) as isize);
                twiddle_val = *twiddles;
                tmp = ixheaacd_mult32x16lin32(x6r, twiddle_val)
                    - ixheaac_mult32x16hin32(x6i, twiddle_val);
                x6i = ixheaacd_mac32x16lin32(
                    ixheaac_mult32x16hin32(x6r, twiddle_val),
                    x6i,
                    twiddle_val,
                ) << 1 as core::ffi::c_int;
                x6r = tmp << 1 as core::ffi::c_int;
                x0r = *data_0;
                x0i = *data_0.offset(1 as core::ffi::c_int as isize);
                data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                x0r = x0r + x4r;
                x0i = x0i + x4i;
                x4r = x0r - (x4r << 1 as core::ffi::c_int);
                x4i = x0i - (x4i << 1 as core::ffi::c_int);
                x2r = x2r + x6r;
                x2i = x2i + x6i;
                x6r = x2r - (x6r << 1 as core::ffi::c_int);
                x6i = x2i - (x6i << 1 as core::ffi::c_int);
                x0r = x0r + x2r;
                x0i = x0i + x2i;
                x2r = x0r - (x2r << 1 as core::ffi::c_int);
                x2i = x0i - (x2i << 1 as core::ffi::c_int);
                x4r = x4r + x6i;
                x4i = x4i - x6r;
                tmp = x6r;
                x6r = x4r - (x6i << 1 as core::ffi::c_int);
                x6i = x4i + (tmp << 1 as core::ffi::c_int);
                x1r = *data_0;
                x1i = *data_0.offset(1 as core::ffi::c_int as isize);
                data_0 = data_0.offset((del << 2 as core::ffi::c_int) as isize);
                twiddles = twiddles
                    .offset(-((5 as WORD32 * (j >> 3 as core::ffi::c_int)) as isize));
                twiddle_val = *twiddles;
                tmp = ixheaacd_mult32x16lin32(x1r, twiddle_val)
                    - ixheaac_mult32x16hin32(x1i, twiddle_val);
                x1i = ixheaacd_mac32x16lin32(
                    ixheaac_mult32x16hin32(x1r, twiddle_val),
                    x1i,
                    twiddle_val,
                ) << 1 as core::ffi::c_int;
                x1r = tmp << 1 as core::ffi::c_int;
                x3r = *data_0;
                x3i = *data_0.offset(1 as core::ffi::c_int as isize);
                data_0 = data_0.offset((del << 2 as core::ffi::c_int) as isize);
                twiddles = twiddles.offset((j >> 2 as core::ffi::c_int) as isize);
                twiddle_val = *twiddles;
                tmp = ixheaacd_mult32x16lin32(x3r, twiddle_val)
                    - ixheaac_mult32x16hin32(x3i, twiddle_val);
                x3i = ixheaacd_mac32x16lin32(
                    ixheaac_mult32x16hin32(x3r, twiddle_val),
                    x3i,
                    twiddle_val,
                );
                x3r = tmp;
                x5r = *data_0;
                x5i = *data_0.offset(1 as core::ffi::c_int as isize);
                data_0 = data_0.offset((del << 2 as core::ffi::c_int) as isize);
                twiddles = twiddles.offset((j >> 2 as core::ffi::c_int) as isize);
                twiddle_val = *twiddles;
                tmp = ixheaacd_mult32x16lin32(x5r, twiddle_val)
                    - ixheaac_mult32x16hin32(x5i, twiddle_val);
                x5i = ixheaacd_mac32x16lin32(
                    ixheaac_mult32x16hin32(x5r, twiddle_val),
                    x5i,
                    twiddle_val,
                );
                x5r = tmp;
                x7r = *data_0;
                x7i = *data_0.offset(1 as core::ffi::c_int as isize);
                data_0 = data_0
                    .offset(-((7 as WORD32 * (del << 1 as core::ffi::c_int)) as isize));
                twiddles = twiddles.offset((j >> 2 as core::ffi::c_int) as isize);
                twiddle_val = *twiddles;
                twiddles = twiddles
                    .offset(-((7 as WORD32 * (j >> 3 as core::ffi::c_int)) as isize));
                tmp = ixheaacd_mult32x16lin32(x7r, twiddle_val)
                    - ixheaac_mult32x16hin32(x7i, twiddle_val);
                x7i = ixheaacd_mac32x16lin32(
                    ixheaac_mult32x16hin32(x7r, twiddle_val),
                    x7i,
                    twiddle_val,
                );
                x7r = tmp;
                x1r = x1r + (x5r << 1 as core::ffi::c_int);
                x1i = x1i + (x5i << 1 as core::ffi::c_int);
                x5r = x1r - (x5r << 2 as core::ffi::c_int);
                x5i = x1i - (x5i << 2 as core::ffi::c_int);
                x3r = x3r + x7r;
                x3i = x3i + x7i;
                x7r = x3r - (x7r << 1 as core::ffi::c_int);
                x7i = x3i - (x7i << 1 as core::ffi::c_int);
                x1r = x1r + (x3r << 1 as core::ffi::c_int);
                x1i = x1i + (x3i << 1 as core::ffi::c_int);
                x3r = x1r - (x3r << 2 as core::ffi::c_int);
                x3i = x1i - (x3i << 2 as core::ffi::c_int);
                x5r = x5r + x5i;
                x5i = x5r - (x5i << 1 as core::ffi::c_int);
                x7r = x7r + x7i;
                x7i = x7r - (x7i << 1 as core::ffi::c_int);
                x7i = x5r - (x7i << 1 as core::ffi::c_int);
                x5r = x7i - (x5r << 1 as core::ffi::c_int);
                x5i = (x7r << 1 as core::ffi::c_int) - x5i;
                x7r = x5i - (x7r << 2 as core::ffi::c_int);
                x7i = x7i << 1 as core::ffi::c_int;
                x5r = x5r << 1 as core::ffi::c_int;
                x5i = x5i << 1 as core::ffi::c_int;
                x7r = x7r << 1 as core::ffi::c_int;
                x0r = x0r + x1r;
                x0i = x0i + x1i;
                x1r = x0r - (x1r << 1 as core::ffi::c_int);
                x1i = x0i - (x1i << 1 as core::ffi::c_int);
                x2r = x2r + x3i;
                tmp = x2r - (x3i << 1 as core::ffi::c_int);
                x2i = x2i - x3r;
                x3i = x2i + (x3r << 1 as core::ffi::c_int);
                *data_0 = x0r;
                *data_0.offset(1 as core::ffi::c_int as isize) = x0i;
                data_0 = data_0.offset((del << 2 as core::ffi::c_int) as isize);
                *data_0 = x2r;
                *data_0.offset(1 as core::ffi::c_int as isize) = x2i;
                data_0 = data_0.offset((del << 2 as core::ffi::c_int) as isize);
                *data_0 = x1r;
                *data_0.offset(1 as core::ffi::c_int as isize) = x1i;
                data_0 = data_0.offset((del << 2 as core::ffi::c_int) as isize);
                *data_0 = tmp;
                *data_0.offset(1 as core::ffi::c_int as isize) = x3i;
                data_0 = data_0
                    .offset(-((5 as WORD32 * (del << 1 as core::ffi::c_int)) as isize));
                tmp = 0x5a82 as core::ffi::c_int as WORD32;
                x7i = x4r + ixheaacd_mult32x16lin32(x7i, tmp);
                x4r = x7i - (x4r << 1 as core::ffi::c_int);
                x7r = x4i + ixheaacd_mult32x16lin32(x7r, tmp);
                x4i = x7r - (x4i << 1 as core::ffi::c_int);
                x5i = x6r + ixheaacd_mult32x16lin32(x5i, tmp);
                x6r = x5i - (x6r << 1 as core::ffi::c_int);
                x5r = x6i + ixheaacd_mult32x16lin32(x5r, tmp);
                x6i = x5r - (x6i << 1 as core::ffi::c_int);
                *data_0 = x7i;
                *data_0.offset(1 as core::ffi::c_int as isize) = x7r;
                data_0 = data_0.offset((del << 2 as core::ffi::c_int) as isize);
                *data_0 = x5i;
                *data_0.offset(1 as core::ffi::c_int as isize) = x5r;
                data_0 = data_0.offset((del << 2 as core::ffi::c_int) as isize);
                *data_0 = -x4r;
                *data_0.offset(1 as core::ffi::c_int as isize) = -x4i;
                data_0 = data_0.offset((del << 2 as core::ffi::c_int) as isize);
                *data_0 = -x6r;
                *data_0.offset(1 as core::ffi::c_int as isize) = -x6i;
                data_0 = data_0
                    .offset(-((7 as WORD32 * (del << 1 as core::ffi::c_int)) as isize));
                data_0 = data_0.offset((del << 4 as core::ffi::c_int) as isize);
                k -= 1;
            }
            data_0 = data_0.offset(-((npoints << 1 as core::ffi::c_int) as isize));
            j += nodespacing;
        }
        nodespacing >>= 3 as core::ffi::c_int;
        del <<= 3 as core::ffi::c_int;
        in_loop_cnt >>= 3 as core::ffi::c_int;
        k1 -= 1;
    }
    let mut data_1: *mut WORD32 = ptr_y;
    let mut twiddles_0: *const WORD32 = 0 as *const WORD32;
    twiddles_0 = ptr_twiddle;
    data_1 = ptr_y;
    data_1 = data_1.offset(-(2 as core::ffi::c_int as isize));
    j = 0 as core::ffi::c_int as WORD32;
    while j < nodespacing * del {
        data_1 = data_1.offset(2 as core::ffi::c_int as isize);
        data_1 = data_1.offset((del << 2 as core::ffi::c_int) as isize);
        x2r = *data_1;
        x2i = *data_1.offset(1 as core::ffi::c_int as isize);
        data_1 = data_1.offset((del << 2 as core::ffi::c_int) as isize);
        x4r = *data_1;
        x4i = *data_1.offset(1 as core::ffi::c_int as isize);
        data_1 = data_1.offset((del << 2 as core::ffi::c_int) as isize);
        x6r = *data_1;
        x6i = *data_1.offset(1 as core::ffi::c_int as isize);
        data_1 = data_1
            .offset(-((6 as WORD32 * (del << 1 as core::ffi::c_int)) as isize));
        twiddles_0 = twiddles_0.offset((j >> 2 as core::ffi::c_int) as isize);
        twiddle_val = *twiddles_0;
        tmp = ixheaacd_mult32x16lin32(x2r, twiddle_val)
            - ixheaac_mult32x16hin32(x2i, twiddle_val);
        x2i = ixheaacd_mac32x16lin32(
            ixheaac_mult32x16hin32(x2r, twiddle_val),
            x2i,
            twiddle_val,
        ) << 1 as core::ffi::c_int;
        x2r = tmp << 1 as core::ffi::c_int;
        twiddles_0 = twiddles_0.offset((j >> 2 as core::ffi::c_int) as isize);
        twiddle_val = *twiddles_0;
        tmp = ixheaacd_mult32x16lin32(x4r, twiddle_val)
            - ixheaac_mult32x16hin32(x4i, twiddle_val);
        x4i = ixheaacd_mac32x16lin32(
            ixheaac_mult32x16hin32(x4r, twiddle_val),
            x4i,
            twiddle_val,
        ) << 1 as core::ffi::c_int;
        x4r = tmp << 1 as core::ffi::c_int;
        twiddles_0 = twiddles_0.offset((j >> 2 as core::ffi::c_int) as isize);
        twiddle_val = *twiddles_0;
        tmp = ixheaacd_mult32x16lin32(x6r, twiddle_val)
            - ixheaac_mult32x16hin32(x6i, twiddle_val);
        x6i = ixheaacd_mac32x16lin32(
            ixheaac_mult32x16hin32(x6r, twiddle_val),
            x6i,
            twiddle_val,
        ) << 1 as core::ffi::c_int;
        x6r = tmp << 1 as core::ffi::c_int;
        x0r = *data_1;
        x0i = *data_1.offset(1 as core::ffi::c_int as isize);
        data_1 = data_1.offset((del << 1 as core::ffi::c_int) as isize);
        x0r = x0r + x4r;
        x0i = x0i + x4i;
        x4r = x0r - (x4r << 1 as core::ffi::c_int);
        x4i = x0i - (x4i << 1 as core::ffi::c_int);
        x2r = x2r + x6r;
        x2i = x2i + x6i;
        x6r = x2r - (x6r << 1 as core::ffi::c_int);
        x6i = x2i - (x6i << 1 as core::ffi::c_int);
        x0r = x0r + x2r;
        x0i = x0i + x2i;
        x2r = x0r - (x2r << 1 as core::ffi::c_int);
        x2i = x0i - (x2i << 1 as core::ffi::c_int);
        x4r = x4r + x6i;
        x4i = x4i - x6r;
        tmp = x6r;
        x6r = x4r - (x6i << 1 as core::ffi::c_int);
        x6i = x4i + (tmp << 1 as core::ffi::c_int);
        x1r = *data_1;
        x1i = *data_1.offset(1 as core::ffi::c_int as isize);
        data_1 = data_1.offset((del << 2 as core::ffi::c_int) as isize);
        twiddles_0 = twiddles_0
            .offset(-((5 as WORD32 * (j >> 3 as core::ffi::c_int)) as isize));
        twiddle_val = *twiddles_0;
        tmp = ixheaacd_mult32x16lin32(x1r, twiddle_val)
            - ixheaac_mult32x16hin32(x1i, twiddle_val);
        x1i = ixheaacd_mac32x16lin32(
            ixheaac_mult32x16hin32(x1r, twiddle_val),
            x1i,
            twiddle_val,
        ) << 1 as core::ffi::c_int;
        x1r = tmp << 1 as core::ffi::c_int;
        x3r = *data_1;
        x3i = *data_1.offset(1 as core::ffi::c_int as isize);
        data_1 = data_1.offset((del << 2 as core::ffi::c_int) as isize);
        twiddles_0 = twiddles_0.offset((j >> 2 as core::ffi::c_int) as isize);
        twiddle_val = *twiddles_0;
        tmp = ixheaacd_mult32x16lin32(x3r, twiddle_val)
            - ixheaac_mult32x16hin32(x3i, twiddle_val);
        x3i = ixheaacd_mac32x16lin32(
            ixheaac_mult32x16hin32(x3r, twiddle_val),
            x3i,
            twiddle_val,
        );
        x3r = tmp;
        x5r = *data_1;
        x5i = *data_1.offset(1 as core::ffi::c_int as isize);
        data_1 = data_1.offset((del << 2 as core::ffi::c_int) as isize);
        twiddles_0 = twiddles_0.offset((j >> 2 as core::ffi::c_int) as isize);
        twiddle_val = *twiddles_0;
        tmp = ixheaacd_mult32x16lin32(x5r, twiddle_val)
            - ixheaac_mult32x16hin32(x5i, twiddle_val);
        x5i = ixheaacd_mac32x16lin32(
            ixheaac_mult32x16hin32(x5r, twiddle_val),
            x5i,
            twiddle_val,
        );
        x5r = tmp;
        x7r = *data_1;
        x7i = *data_1.offset(1 as core::ffi::c_int as isize);
        data_1 = data_1
            .offset(-((7 as WORD32 * (del << 1 as core::ffi::c_int)) as isize));
        twiddles_0 = twiddles_0.offset((j >> 2 as core::ffi::c_int) as isize);
        twiddle_val = *twiddles_0;
        twiddles_0 = twiddles_0
            .offset(-((7 as WORD32 * (j >> 3 as core::ffi::c_int)) as isize));
        tmp = ixheaacd_mult32x16lin32(x7r, twiddle_val)
            - ixheaac_mult32x16hin32(x7i, twiddle_val);
        x7i = ixheaacd_mac32x16lin32(
            ixheaac_mult32x16hin32(x7r, twiddle_val),
            x7i,
            twiddle_val,
        );
        x7r = tmp;
        x1r = x1r + (x5r << 1 as core::ffi::c_int);
        x1i = x1i + (x5i << 1 as core::ffi::c_int);
        x5r = x1r - (x5r << 2 as core::ffi::c_int);
        x5i = x1i - (x5i << 2 as core::ffi::c_int);
        x3r = x3r + x7r;
        x3i = x3i + x7i;
        x7r = x3r - (x7r << 1 as core::ffi::c_int);
        x7i = x3i - (x7i << 1 as core::ffi::c_int);
        x1r = x1r + (x3r << 1 as core::ffi::c_int);
        x1i = x1i + (x3i << 1 as core::ffi::c_int);
        x3r = x1r - (x3r << 2 as core::ffi::c_int);
        x3i = x1i - (x3i << 2 as core::ffi::c_int);
        x5r = x5r + x5i;
        x5i = x5r - (x5i << 1 as core::ffi::c_int);
        x7r = x7r + x7i;
        x7i = x7r - (x7i << 1 as core::ffi::c_int);
        x7i = x5r - (x7i << 1 as core::ffi::c_int);
        x5r = x7i - (x5r << 1 as core::ffi::c_int);
        x5i = (x7r << 1 as core::ffi::c_int) - x5i;
        x7r = x5i - (x7r << 2 as core::ffi::c_int);
        x7i = x7i << 1 as core::ffi::c_int;
        x5r = x5r << 1 as core::ffi::c_int;
        x5i = x5i << 1 as core::ffi::c_int;
        x7r = x7r << 1 as core::ffi::c_int;
        x0r = x0r + x1r;
        x0i = x0i + x1i;
        x1r = x0r - (x1r << 1 as core::ffi::c_int);
        x1i = x0i - (x1i << 1 as core::ffi::c_int);
        x2r = x2r + x3i;
        tmp = x2r - (x3i << 1 as core::ffi::c_int);
        x2i = x2i - x3r;
        x3i = x2i + (x3r << 1 as core::ffi::c_int);
        *data_1 = x0r;
        *data_1.offset(1 as core::ffi::c_int as isize) = x0i;
        data_1 = data_1.offset((del << 2 as core::ffi::c_int) as isize);
        *data_1 = x2r;
        *data_1.offset(1 as core::ffi::c_int as isize) = x2i;
        data_1 = data_1.offset((del << 2 as core::ffi::c_int) as isize);
        *data_1 = x1r;
        *data_1.offset(1 as core::ffi::c_int as isize) = x1i;
        data_1 = data_1.offset((del << 2 as core::ffi::c_int) as isize);
        *data_1 = tmp;
        *data_1.offset(1 as core::ffi::c_int as isize) = x3i;
        data_1 = data_1
            .offset(-((5 as WORD32 * (del << 1 as core::ffi::c_int)) as isize));
        tmp = 0x5a82 as core::ffi::c_int as WORD32;
        x7i = x4r + ixheaacd_mult32x16lin32(x7i, tmp);
        x4r = x7i - (x4r << 1 as core::ffi::c_int);
        x7r = x4i + ixheaacd_mult32x16lin32(x7r, tmp);
        x4i = x7r - (x4i << 1 as core::ffi::c_int);
        x5i = x6r + ixheaacd_mult32x16lin32(x5i, tmp);
        x6r = x5i - (x6r << 1 as core::ffi::c_int);
        x5r = x6i + ixheaacd_mult32x16lin32(x5r, tmp);
        x6i = x5r - (x6i << 1 as core::ffi::c_int);
        *data_1 = x7i;
        *data_1.offset(1 as core::ffi::c_int as isize) = x7r;
        data_1 = data_1.offset((del << 2 as core::ffi::c_int) as isize);
        *data_1 = x5i;
        *data_1.offset(1 as core::ffi::c_int as isize) = x5r;
        data_1 = data_1.offset((del << 2 as core::ffi::c_int) as isize);
        *data_1 = -x4r;
        *data_1.offset(1 as core::ffi::c_int as isize) = -x4i;
        data_1 = data_1.offset((del << 2 as core::ffi::c_int) as isize);
        *data_1 = -x6r;
        *data_1.offset(1 as core::ffi::c_int as isize) = -x6i;
        data_1 = data_1
            .offset(-((7 as WORD32 * (del << 1 as core::ffi::c_int)) as isize));
        data_1 = data_1.offset((del << 4 as core::ffi::c_int) as isize);
        data_1 = data_1.offset(-((npoints << 1 as core::ffi::c_int) as isize));
        j += nodespacing;
    }
    nodespacing >>= 3 as core::ffi::c_int;
    del <<= 3 as core::ffi::c_int;
    in_loop_cnt >>= 3 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_inverse_transform_960(
    mut spec_data: *mut WORD32,
    mut scratch: *mut WORD32,
    mut ptr_imdct_tables: *mut ia_aac_dec_imdct_tables_struct,
    mut expo: WORD32,
    mut imdct_scale: *mut WORD32,
) -> VOID {
    let mut n: WORD32 = 0;
    let mut Nd2: WORD32 = 0;
    let mut const_mltfac: WORD16 = 0;
    let mut neg_expo: WORD32 = 0;
    let mut i: WORD32 = 0;
    n = 120 as core::ffi::c_int as WORD32;
    Nd2 = n >> 1 as core::ffi::c_int;
    neg_expo = 4 as core::ffi::c_int as WORD32;
    ixheaacd_pre_twiddle_120(
        spec_data as *mut WORD32,
        scratch as *mut WORD32,
        n,
        ((*ptr_imdct_tables).cosine_array_240).as_mut_ptr(),
        neg_expo - expo,
    );
    ixheaacd_fft_120(
        ptr_imdct_tables,
        Nd2,
        spec_data as *mut WORD32,
        scratch as *mut WORD32,
    );
    neg_expo += 2 as core::ffi::c_int;
    *imdct_scale = (neg_expo as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
    ixheaacd_post_twiddle_120(
        spec_data,
        scratch,
        ((*ptr_imdct_tables).cosine_array_240).as_mut_ptr(),
        n as WORD,
    );
    const_mltfac = 17476 as WORD16;
    i = 0 as core::ffi::c_int as WORD32;
    while i < 120 as core::ffi::c_int {
        *spec_data.offset(i as isize) = ixheaac_mult32x16in32_shl(
            *spec_data.offset(i as isize),
            const_mltfac,
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_inverse_transform(
    mut spec_data: *mut WORD32,
    mut scratch: *mut WORD32,
    mut ptr_imdct_tables: *mut ia_aac_dec_imdct_tables_struct,
    mut expo: WORD32,
    mut npoints: WORD32,
) -> WORD32 {
    (Some(ixheaacd_pretwiddle_compute.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        spec_data as *mut WORD32,
        spec_data.offset(npoints as isize).offset(-(1 as core::ffi::c_int as isize)),
        scratch as *mut WORD32,
        ptr_imdct_tables,
        npoints as WORD >> 2 as core::ffi::c_int,
        expo,
    );
    (Some(ixheaacd_imdct_using_fft.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        ptr_imdct_tables,
        npoints >> 1 as core::ffi::c_int,
        scratch as *mut WORD32,
        spec_data as *mut WORD32,
    );
    expo += 2 as core::ffi::c_int;
    return expo;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mdct_960(
    mut inp: *mut WORD32,
    mut scratch: *mut WORD32,
    mut mdct_scale: *mut WORD32,
    mut mdct_flag: WORD32,
    mut imdct_tables_ptr: *mut ia_aac_dec_imdct_tables_struct,
) -> VOID {
    let mut expo: WORD32 = 0;
    let mut neg_expo: WORD32 = 0 as WORD32;
    let mut k: WORD32 = 0;
    let mut const_mltfac: WORD16 = 17476 as WORD16;
    expo = ((Some(ixheaacd_calc_max_spectral_line.expect("non-null function pointer")))
        .expect("non-null function pointer")(inp, MDCT_LEN_960) as core::ffi::c_int
        - 1 as core::ffi::c_int) as WORD32;
    memcpy(
        scratch as *mut core::ffi::c_void,
        inp as *const core::ffi::c_void,
        (::core::mem::size_of::<WORD32>() as size_t).wrapping_mul(MDCT_LEN_960 as size_t),
    );
    neg_expo = 7 as WORD32 - expo;
    ixheaacd_pre_twiddle_960(
        inp,
        scratch,
        MDCT_LEN_960,
        ((*imdct_tables_ptr).cosine_array_1920).as_mut_ptr(),
        neg_expo,
    );
    ixheaacd_fft_960(inp, scratch, imdct_tables_ptr);
    ixheaacd_post_twiddle_960(
        inp as *mut WORD32,
        scratch as *mut WORD32,
        ((*imdct_tables_ptr).cosine_array_1920).as_mut_ptr(),
        MDCT_LEN_960,
    );
    if 0 as core::ffi::c_int == mdct_flag {
        let mut data: *mut WORD32 = inp;
        k = (MDCT_LEN_960 - 1 as core::ffi::c_int) as WORD32;
        while k >= 0 as core::ffi::c_int {
            *data = ixheaac_mult32x16in32_shl(*data, const_mltfac);
            data = data.offset(1);
            *data = ixheaac_mult32x16in32_shl(*data, const_mltfac);
            data = data.offset(1);
            k -= 2 as core::ffi::c_int;
        }
    }
    *mdct_scale = (neg_expo as core::ffi::c_int + 1 as core::ffi::c_int
        + 1 as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mdct_480_ld(
    mut inp: *mut WORD32,
    mut scratch: *mut WORD32,
    mut mdct_scale: *mut WORD32,
    mut mdct_flag: WORD32,
    mut imdct_tables_ptr: *mut ia_aac_dec_imdct_tables_struct,
    mut object_type: WORD32,
) -> VOID {
    let mut expo: WORD32 = 0;
    let mut neg_expo: WORD32 = 0 as WORD32;
    let mut k: WORD32 = 0;
    let mut const_mltfac: WORD32 = 1145324612 as WORD32;
    expo = ((Some(ixheaacd_calc_max_spectral_line.expect("non-null function pointer")))
        .expect("non-null function pointer")(inp, MDCT_LEN) as core::ffi::c_int
        - 1 as core::ffi::c_int) as WORD32;
    memcpy(
        scratch as *mut core::ffi::c_void,
        inp as *const core::ffi::c_void,
        (::core::mem::size_of::<WORD32>() as size_t).wrapping_mul(MDCT_LEN as size_t),
    );
    neg_expo = 7 as WORD32 - expo;
    ixheaacd_pre_twiddle(
        inp,
        scratch,
        480 as WORD32,
        ((*imdct_tables_ptr).cosine_array_960).as_mut_ptr(),
        neg_expo,
    );
    ixheaacd_fft_480_ld(inp, scratch, imdct_tables_ptr);
    if object_type == AOT_ER_AAC_LD as core::ffi::c_int {
        ixheaacd_post_twiddle_ld(
            inp as *mut WORD32,
            scratch as *mut WORD32,
            ((*imdct_tables_ptr).cosine_array_960).as_mut_ptr(),
            480 as WORD,
        );
    } else if object_type == AOT_ER_AAC_ELD as core::ffi::c_int {
        ixheaacd_post_twiddle_eld(
            inp.offset(480 as core::ffi::c_int as isize),
            scratch as *mut WORD32,
            ((*imdct_tables_ptr).cosine_array_960).as_mut_ptr(),
            480 as WORD,
        );
    }
    if 0 as core::ffi::c_int == mdct_flag {
        let mut data: *mut WORD32 = inp;
        if object_type != AOT_ER_AAC_ELD as core::ffi::c_int {
            k = (MDCT_LEN - 1 as core::ffi::c_int) as WORD32;
            while k >= 0 as core::ffi::c_int {
                *data = ixheaac_mult32_shl(*data, const_mltfac);
                data = data.offset(1);
                *data = ixheaac_mult32_shl(*data, const_mltfac);
                data = data.offset(1);
                k -= 2 as core::ffi::c_int;
            }
            neg_expo += 1 as core::ffi::c_int;
        } else {
            data = inp.offset(480 as core::ffi::c_int as isize);
            k = ((MDCT_LEN << 1 as core::ffi::c_int) - 1 as core::ffi::c_int) as WORD32;
            while k >= 0 as core::ffi::c_int {
                *data = ixheaac_mult32_shl(*data, const_mltfac);
                data = data.offset(1);
                *data = ixheaac_mult32_shl(*data, const_mltfac);
                data = data.offset(1);
                k -= 2 as core::ffi::c_int;
            }
            neg_expo += 1 as core::ffi::c_int;
        }
    }
    *mdct_scale = (neg_expo as core::ffi::c_int + 3 as core::ffi::c_int) as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_inverse_transform_512(
    mut data: *mut WORD32,
    mut temp: *mut WORD32,
    mut imdct_scale: *mut WORD32,
    mut cos_sin_ptr: *mut WORD32,
    mut imdct_tables_ptr: *mut ia_aac_dec_imdct_tables_struct,
    mut object_type: WORD32,
) -> VOID {
    let mut n: WORD32 = 0;
    let mut npoints_2: WORD32 = 0;
    let mut expo: WORD16 = 0;
    let mut neg_expo: WORD16 = 0;
    n = 512 as core::ffi::c_int as WORD32;
    npoints_2 = n >> 1 as core::ffi::c_int;
    expo = ((Some(ixheaacd_calc_max_spectral_line.expect("non-null function pointer")))
        .expect("non-null function pointer")(data as *mut WORD32, n) as core::ffi::c_int
        - 1 as core::ffi::c_int) as WORD16;
    memcpy(
        temp as *mut core::ffi::c_void,
        data as *const core::ffi::c_void,
        (::core::mem::size_of::<WORD32>() as size_t).wrapping_mul(n as size_t),
    );
    neg_expo = (7 as core::ffi::c_int - expo as core::ffi::c_int) as WORD16;
    ixheaacd_pre_twiddle(
        data as *mut WORD32,
        temp as *mut WORD32,
        n,
        cos_sin_ptr,
        neg_expo as WORD32,
    );
    (Some(ixheaacd_fft32x32_ld.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(imdct_tables_ptr, npoints_2, data as *mut WORD32, temp as *mut WORD32);
    neg_expo = (Some(ixheaacd_neg_expo_inc.expect("non-null function pointer")))
        .expect("non-null function pointer")(neg_expo);
    *imdct_scale = (neg_expo as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
    if object_type == AOT_ER_AAC_ELD as core::ffi::c_int {
        ixheaacd_post_twiddle_eld(data.offset(n as isize), temp, cos_sin_ptr, n as WORD);
    } else {
        ixheaacd_post_twiddle_ld(data, temp, cos_sin_ptr, n as WORD);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_fft_960(
    mut inp: *mut WORD32,
    mut op: *mut WORD32,
    mut imdct_tables_ptr: *mut ia_aac_dec_imdct_tables_struct,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut buf1: *mut WORD32 = 0 as *mut WORD32;
    let mut buf2: *mut WORD32 = 0 as *mut WORD32;
    let mut re_arr_tab_sml_480_ptr: *mut WORD16 = 0 as *mut WORD16;
    (Some(ixheaacd_aac_ld_dec_rearrange_960.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(inp, op, 480 as WORD32, ((*imdct_tables_ptr).re_arr_tab_32).as_mut_ptr());
    buf1 = op;
    buf2 = inp;
    i = 0 as core::ffi::c_int as WORD32;
    while i < FFT15 {
        ixheaacd_fft_32_points(
            ((*imdct_tables_ptr).w_32).as_mut_ptr(),
            32 as WORD32,
            buf1,
            buf2,
        );
        buf1 = buf1.offset((FFT16X2 * 2 as core::ffi::c_int) as isize);
        buf2 = buf2.offset((FFT16X2 * 2 as core::ffi::c_int) as isize);
        i += 1;
    }
    re_arr_tab_sml_480_ptr = ((*imdct_tables_ptr).re_arr_tab_sml_480).as_mut_ptr();
    buf1 = inp;
    i = 0 as core::ffi::c_int as WORD32;
    while i < FFT16 * 2 as core::ffi::c_int {
        ixheaacd_ld_dec_fft_15_opt(
            buf1,
            op,
            ixheaacd_fft5out.as_mut_ptr(),
            re_arr_tab_sml_480_ptr,
        );
        buf1 = buf1.offset(2 as core::ffi::c_int as isize);
        re_arr_tab_sml_480_ptr = re_arr_tab_sml_480_ptr.offset(FFT15 as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_fft_32_points(
    mut ptr_w: *mut WORD16,
    mut npoints: WORD32,
    mut ptr_x: *mut WORD32,
    mut ptr_y: *mut WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut l1: WORD32 = 0;
    let mut l2: WORD32 = 0;
    let mut h2: WORD32 = 0;
    let mut predj: WORD32 = 0;
    let mut tw_offset: WORD32 = 0;
    let mut stride: WORD32 = 0;
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
    let mut w: *mut WORD16 = 0 as *mut WORD16;
    let mut x: *mut WORD32 = 0 as *mut WORD32;
    let mut x2: *mut WORD32 = 0 as *mut WORD32;
    let mut x0: *mut WORD32 = 0 as *mut WORD32;
    let mut y0: *mut WORD32 = 0 as *mut WORD32;
    let mut y1: *mut WORD32 = 0 as *mut WORD32;
    let mut y2: *mut WORD32 = 0 as *mut WORD32;
    let mut y3: *mut WORD32 = 0 as *mut WORD32;
    let mut n0: WORD32 = 0;
    let mut j0: WORD32 = 0;
    let mut radix: WORD32 = 0;
    let mut norm: WORD32 = 0;
    radix = 2 as core::ffi::c_int as WORD32;
    norm = 25 as core::ffi::c_int as WORD32;
    stride = 32 as core::ffi::c_int as WORD32;
    tw_offset = 0 as core::ffi::c_int as WORD32;
    fft_jmp = 192 as core::ffi::c_int as WORD32;
    while stride > radix {
        j = 0 as core::ffi::c_int as WORD32;
        fft_jmp >>= 2 as core::ffi::c_int;
        h2 = stride >> 1 as core::ffi::c_int;
        l1 = stride;
        l2 = stride + (stride >> 1 as core::ffi::c_int);
        x = ptr_x;
        w = ptr_w.offset(tw_offset as isize);
        tw_offset += fft_jmp;
        i = 0 as core::ffi::c_int as WORD32;
        while i < npoints {
            co10 = *w.offset((j as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
            si10 = *w.offset((j as core::ffi::c_int + 0 as core::ffi::c_int) as isize);
            co20 = *w.offset((j as core::ffi::c_int + 3 as core::ffi::c_int) as isize);
            si20 = *w.offset((j as core::ffi::c_int + 2 as core::ffi::c_int) as isize);
            co30 = *w.offset((j as core::ffi::c_int + 5 as core::ffi::c_int) as isize);
            si30 = *w.offset((j as core::ffi::c_int + 4 as core::ffi::c_int) as isize);
            x_0 = *x.offset(0 as core::ffi::c_int as isize);
            x_1 = *x.offset(1 as core::ffi::c_int as isize);
            x_l1_0 = *x.offset(l1 as isize);
            x_l1_1 = *x
                .offset((l1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
            x_l2_0 = *x.offset(l2 as isize);
            x_l2_1 = *x
                .offset((l2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
            x_h2_0 = *x.offset(h2 as isize);
            x_h2_1 = *x
                .offset((h2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
            xh0_0 = ixheaac_add32_sat(x_0, x_l1_0);
            xh1_0 = ixheaac_add32_sat(x_1, x_l1_1);
            xl0_0 = ixheaac_sub32_sat(x_0, x_l1_0);
            xl1_0 = ixheaac_sub32_sat(x_1, x_l1_1);
            xh20_0 = ixheaac_add32_sat(x_h2_0, x_l2_0);
            xh21_0 = ixheaac_add32_sat(x_h2_1, x_l2_1);
            xl20_0 = ixheaac_sub32_sat(x_h2_0, x_l2_0);
            xl21_0 = ixheaac_sub32_sat(x_h2_1, x_l2_1);
            x0 = x;
            x2 = x0;
            j += 6 as core::ffi::c_int;
            x = x.offset(2 as core::ffi::c_int as isize);
            predj = j - fft_jmp;
            if predj == 0 {
                x = x.offset(fft_jmp as isize);
            }
            if predj == 0 {
                j = 0 as core::ffi::c_int as WORD32;
            }
            *x0.offset(0 as core::ffi::c_int as isize) = ixheaac_add32_sat(
                xh0_0,
                xh20_0,
            );
            *x0.offset(1 as core::ffi::c_int as isize) = ixheaac_add32_sat(
                xh1_0,
                xh21_0,
            );
            xt0_0 = ixheaac_sub32_sat(xh0_0, xh20_0);
            yt0_0 = ixheaac_sub32_sat(xh1_0, xh21_0);
            xt1_0 = ixheaac_add32_sat(xl0_0, xl21_0);
            yt2_0 = ixheaac_add32_sat(xl1_0, xl20_0);
            xt2_0 = ixheaac_sub32_sat(xl0_0, xl21_0);
            yt1_0 = ixheaac_sub32_sat(xl1_0, xl20_0);
            *x2.offset(h2 as isize) = ixheaac_add32_sat(
                (si10 as core::ffi::c_int
                    * (yt1_0 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                        as core::ffi::c_int + 0x4000 as core::ffi::c_int
                    >> 15 as core::ffi::c_int)
                    + ((si10 as core::ffi::c_int
                        * (yt1_0 >> 16 as core::ffi::c_int) as WORD16
                            as core::ffi::c_int) << 1 as core::ffi::c_int),
                (co10 as core::ffi::c_int
                    * (xt1_0 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                        as core::ffi::c_int + 0x4000 as core::ffi::c_int
                    >> 15 as core::ffi::c_int)
                    + ((co10 as core::ffi::c_int
                        * (xt1_0 >> 16 as core::ffi::c_int) as WORD16
                            as core::ffi::c_int) << 1 as core::ffi::c_int),
            );
            *x2.offset((h2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = ixheaac_sub32_sat(
                (co10 as core::ffi::c_int
                    * (yt1_0 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                        as core::ffi::c_int + 0x4000 as core::ffi::c_int
                    >> 15 as core::ffi::c_int)
                    + ((co10 as core::ffi::c_int
                        * (yt1_0 >> 16 as core::ffi::c_int) as WORD16
                            as core::ffi::c_int) << 1 as core::ffi::c_int),
                (si10 as core::ffi::c_int
                    * (xt1_0 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                        as core::ffi::c_int + 0x4000 as core::ffi::c_int
                    >> 15 as core::ffi::c_int)
                    + ((si10 as core::ffi::c_int
                        * (xt1_0 >> 16 as core::ffi::c_int) as WORD16
                            as core::ffi::c_int) << 1 as core::ffi::c_int),
            );
            *x2.offset(l1 as isize) = ixheaac_add32_sat(
                (si20 as core::ffi::c_int
                    * (yt0_0 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                        as core::ffi::c_int + 0x4000 as core::ffi::c_int
                    >> 15 as core::ffi::c_int)
                    + ((si20 as core::ffi::c_int
                        * (yt0_0 >> 16 as core::ffi::c_int) as WORD16
                            as core::ffi::c_int) << 1 as core::ffi::c_int),
                (co20 as core::ffi::c_int
                    * (xt0_0 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                        as core::ffi::c_int + 0x4000 as core::ffi::c_int
                    >> 15 as core::ffi::c_int)
                    + ((co20 as core::ffi::c_int
                        * (xt0_0 >> 16 as core::ffi::c_int) as WORD16
                            as core::ffi::c_int) << 1 as core::ffi::c_int),
            );
            *x2.offset((l1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = ixheaac_sub32_sat(
                (co20 as core::ffi::c_int
                    * (yt0_0 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                        as core::ffi::c_int + 0x4000 as core::ffi::c_int
                    >> 15 as core::ffi::c_int)
                    + ((co20 as core::ffi::c_int
                        * (yt0_0 >> 16 as core::ffi::c_int) as WORD16
                            as core::ffi::c_int) << 1 as core::ffi::c_int),
                (si20 as core::ffi::c_int
                    * (xt0_0 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                        as core::ffi::c_int + 0x4000 as core::ffi::c_int
                    >> 15 as core::ffi::c_int)
                    + ((si20 as core::ffi::c_int
                        * (xt0_0 >> 16 as core::ffi::c_int) as WORD16
                            as core::ffi::c_int) << 1 as core::ffi::c_int),
            );
            yt0_0 = (si20 as core::ffi::c_int
                * (yt0_0 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                    as core::ffi::c_int + 0x4000 as core::ffi::c_int
                >> 15 as core::ffi::c_int)
                + ((si20 as core::ffi::c_int
                    * (yt0_0 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int)
                    << 1 as core::ffi::c_int);
            *x2.offset(l2 as isize) = ixheaac_add32_sat(
                (si30 as core::ffi::c_int
                    * (yt2_0 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                        as core::ffi::c_int + 0x4000 as core::ffi::c_int
                    >> 15 as core::ffi::c_int)
                    + ((si30 as core::ffi::c_int
                        * (yt2_0 >> 16 as core::ffi::c_int) as WORD16
                            as core::ffi::c_int) << 1 as core::ffi::c_int),
                (co30 as core::ffi::c_int
                    * (xt2_0 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                        as core::ffi::c_int + 0x4000 as core::ffi::c_int
                    >> 15 as core::ffi::c_int)
                    + ((co30 as core::ffi::c_int
                        * (xt2_0 >> 16 as core::ffi::c_int) as WORD16
                            as core::ffi::c_int) << 1 as core::ffi::c_int),
            );
            *x2.offset((l2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = ixheaac_sub32_sat(
                (co30 as core::ffi::c_int
                    * (yt2_0 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                        as core::ffi::c_int + 0x4000 as core::ffi::c_int
                    >> 15 as core::ffi::c_int)
                    + ((co30 as core::ffi::c_int
                        * (yt2_0 >> 16 as core::ffi::c_int) as WORD16
                            as core::ffi::c_int) << 1 as core::ffi::c_int),
                (si30 as core::ffi::c_int
                    * (xt2_0 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                        as core::ffi::c_int + 0x4000 as core::ffi::c_int
                    >> 15 as core::ffi::c_int)
                    + ((si30 as core::ffi::c_int
                        * (xt2_0 >> 16 as core::ffi::c_int) as WORD16
                            as core::ffi::c_int) << 1 as core::ffi::c_int),
            );
            yt2_0 = (si30 as core::ffi::c_int
                * (yt2_0 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                    as core::ffi::c_int + 0x4000 as core::ffi::c_int
                >> 15 as core::ffi::c_int)
                + ((si30 as core::ffi::c_int
                    * (yt2_0 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int)
                    << 1 as core::ffi::c_int);
            i += 4 as core::ffi::c_int;
        }
        stride >>= 2 as core::ffi::c_int;
    }
    y0 = ptr_y;
    y2 = ptr_y.offset(npoints as isize);
    x0 = ptr_x;
    x2 = ptr_x.offset((npoints >> 1 as core::ffi::c_int) as isize);
    y1 = y0.offset((npoints >> 2 as core::ffi::c_int) as isize);
    y3 = y2.offset((npoints >> 2 as core::ffi::c_int) as isize);
    l1 = (norm as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
    j0 = 8 as core::ffi::c_int as WORD32;
    n0 = npoints >> 1 as core::ffi::c_int;
    j = 0 as core::ffi::c_int as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < 4 as core::ffi::c_int {
        let mut t1: core::ffi::c_int = 0;
        let mut t2: core::ffi::c_int = 0;
        h2 = rev_dig[i as usize];
        t1 = (h2 << 1 as core::ffi::c_int) as core::ffi::c_int;
        t2 = t1 + 1 as core::ffi::c_int;
        *y0.offset(t1 as isize) = ixheaac_add32_sat(
            *x0.offset(0 as core::ffi::c_int as isize),
            *x0.offset(2 as core::ffi::c_int as isize),
        );
        *y2.offset(t1 as isize) = ixheaac_sub32_sat(
            *x0.offset(0 as core::ffi::c_int as isize),
            *x0.offset(2 as core::ffi::c_int as isize),
        );
        *y0.offset(t2 as isize) = ixheaac_add32_sat(
            *x0.offset(1 as core::ffi::c_int as isize),
            *x0.offset(3 as core::ffi::c_int as isize),
        );
        *y2.offset(t2 as isize) = ixheaac_sub32_sat(
            *x0.offset(1 as core::ffi::c_int as isize),
            *x0.offset(3 as core::ffi::c_int as isize),
        );
        *y1.offset(t1 as isize) = ixheaac_add32_sat(
            *x0.offset(4 as core::ffi::c_int as isize),
            *x0.offset(6 as core::ffi::c_int as isize),
        );
        *y3.offset(t1 as isize) = ixheaac_sub32_sat(
            *x0.offset(4 as core::ffi::c_int as isize),
            *x0.offset(6 as core::ffi::c_int as isize),
        );
        *y1.offset(t2 as isize) = ixheaac_add32_sat(
            *x0.offset(5 as core::ffi::c_int as isize),
            *x0.offset(7 as core::ffi::c_int as isize),
        );
        *y3.offset(t2 as isize) = ixheaac_sub32_sat(
            *x0.offset(5 as core::ffi::c_int as isize),
            *x0.offset(7 as core::ffi::c_int as isize),
        );
        x0 = x0.offset(8 as core::ffi::c_int as isize);
        t1 += 2 as core::ffi::c_int;
        t2 += 2 as core::ffi::c_int;
        *y0.offset(t1 as isize) = ixheaac_add32_sat(
            *x2.offset(0 as core::ffi::c_int as isize),
            *x2.offset(2 as core::ffi::c_int as isize),
        );
        *y2.offset(t1 as isize) = ixheaac_sub32_sat(
            *x2.offset(0 as core::ffi::c_int as isize),
            *x2.offset(2 as core::ffi::c_int as isize),
        );
        *y0.offset(t2 as isize) = ixheaac_add32_sat(
            *x2.offset(1 as core::ffi::c_int as isize),
            *x2.offset(3 as core::ffi::c_int as isize),
        );
        *y2.offset(t2 as isize) = ixheaac_sub32_sat(
            *x2.offset(1 as core::ffi::c_int as isize),
            *x2.offset(3 as core::ffi::c_int as isize),
        );
        *y1.offset(t1 as isize) = ixheaac_add32_sat(
            *x2.offset(4 as core::ffi::c_int as isize),
            *x2.offset(6 as core::ffi::c_int as isize),
        );
        *y3.offset(t1 as isize) = ixheaac_sub32_sat(
            *x2.offset(4 as core::ffi::c_int as isize),
            *x2.offset(6 as core::ffi::c_int as isize),
        );
        *y1.offset(t2 as isize) = ixheaac_add32_sat(
            *x2.offset(5 as core::ffi::c_int as isize),
            *x2.offset(7 as core::ffi::c_int as isize),
        );
        *y3.offset(t2 as isize) = ixheaac_sub32_sat(
            *x2.offset(5 as core::ffi::c_int as isize),
            *x2.offset(7 as core::ffi::c_int as isize),
        );
        x2 = x2.offset(8 as core::ffi::c_int as isize);
        j += j0;
        if j == n0 {
            j += n0;
            x0 = x0.offset((npoints >> 1 as core::ffi::c_int) as isize);
            x2 = x2.offset((npoints >> 1 as core::ffi::c_int) as isize);
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_dec_rearrange_short(
    mut ip: *mut WORD32,
    mut op: *mut WORD32,
    mut mdct_len_2: WORD32,
    mut re_arr_tab: *mut WORD16,
) -> VOID {
    let mut n: WORD32 = 0;
    let mut i: WORD32 = 0 as WORD32;
    n = 0 as core::ffi::c_int as WORD32;
    while n < mdct_len_2 {
        let mut idx: WORD32 = (*re_arr_tab.offset(n as isize) as WORD32)
            << 1 as core::ffi::c_int;
        let fresh12 = i;
        i = i + 1;
        *op.offset(fresh12 as isize) = *ip.offset(idx as isize);
        let fresh13 = i;
        i = i + 1;
        *op.offset(fresh13 as isize) = *ip
            .offset((idx as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_ld_dec_fft_15_opt(
    mut inp: *mut WORD32,
    mut op: *mut WORD32,
    mut fft3out: *mut WORD32,
    mut ptr_re_arr_tab_sml_240: *mut WORD16,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut idx: WORD32 = 0;
    let mut buf1: *mut WORD32 = 0 as *mut WORD32;
    let mut buf2: *mut WORD32 = 0 as *mut WORD32;
    let mut buf1a: *mut WORD32 = 0 as *mut WORD32;
    let mut add_r: WORD32 = 0;
    let mut sub_r: WORD32 = 0;
    let mut add_i: WORD32 = 0;
    let mut sub_i: WORD32 = 0;
    let mut x_01_r: WORD32 = 0;
    let mut x_01_i: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut p1: WORD32 = 0;
    let mut p2: WORD32 = 0;
    let mut p3: WORD32 = 0;
    let mut p4: WORD32 = 0;
    let mut sinmu: WORD32 = 1859775393 as WORD32;
    let mut c_51: WORD32 = 2042378317 as WORD32;
    let mut c_52: WORD32 = -(1652318768 as WORD32);
    let mut c_53: WORD32 = -(780119100 as WORD32);
    let mut c_54: WORD32 = 1200479854 as WORD32;
    let mut c_55: WORD32 = -(1342177280 as WORD32);
    let mut r1: WORD32 = 0;
    let mut r2: WORD32 = 0;
    let mut r3: WORD32 = 0;
    let mut r4: WORD32 = 0;
    let mut s1: WORD32 = 0;
    let mut s2: WORD32 = 0;
    let mut s3: WORD32 = 0;
    let mut s4: WORD32 = 0;
    let mut t: WORD32 = 0;
    let mut temp1: WORD32 = 0;
    let mut temp2: WORD32 = 0;
    let mut fft3outptr: *mut WORD32 = fft3out;
    let mut xr_0: WORD32 = 0;
    let mut xr_1: WORD32 = 0;
    let mut xr_2: WORD32 = 0;
    let mut xi_0: WORD32 = 0;
    let mut xi_1: WORD32 = 0;
    let mut xi_2: WORD32 = 0;
    buf2 = fft3out;
    buf1a = fft3out;
    buf1 = buf1a;
    n = 0 as core::ffi::c_int as WORD32;
    let fresh118 = buf1;
    buf1 = buf1.offset(1);
    *fresh118 = *inp.offset(0 as core::ffi::c_int as isize);
    let fresh119 = buf1;
    buf1 = buf1.offset(1);
    *fresh119 = *inp.offset(1 as core::ffi::c_int as isize);
    let fresh120 = buf1;
    buf1 = buf1.offset(1);
    *fresh120 = *inp.offset(192 as core::ffi::c_int as isize);
    let fresh121 = buf1;
    buf1 = buf1.offset(1);
    *fresh121 = *inp.offset(193 as core::ffi::c_int as isize);
    let fresh122 = buf1;
    buf1 = buf1.offset(1);
    *fresh122 = *inp.offset(384 as core::ffi::c_int as isize);
    let fresh123 = buf1;
    buf1 = buf1.offset(1);
    *fresh123 = *inp.offset(385 as core::ffi::c_int as isize);
    let fresh124 = buf1;
    buf1 = buf1.offset(1);
    *fresh124 = *inp.offset(576 as core::ffi::c_int as isize);
    let fresh125 = buf1;
    buf1 = buf1.offset(1);
    *fresh125 = *inp.offset(577 as core::ffi::c_int as isize);
    let fresh126 = buf1;
    buf1 = buf1.offset(1);
    *fresh126 = *inp.offset(768 as core::ffi::c_int as isize);
    let fresh127 = buf1;
    buf1 = buf1.offset(1);
    *fresh127 = *inp.offset(769 as core::ffi::c_int as isize);
    r1 = ixheaac_add32_sat(
        *buf1a.offset(2 as core::ffi::c_int as isize),
        *buf1a.offset(8 as core::ffi::c_int as isize),
    );
    r4 = ixheaac_sub32_sat(
        *buf1a.offset(2 as core::ffi::c_int as isize),
        *buf1a.offset(8 as core::ffi::c_int as isize),
    );
    r3 = ixheaac_add32_sat(
        *buf1a.offset(4 as core::ffi::c_int as isize),
        *buf1a.offset(6 as core::ffi::c_int as isize),
    );
    r2 = ixheaac_sub32_sat(
        *buf1a.offset(4 as core::ffi::c_int as isize),
        *buf1a.offset(6 as core::ffi::c_int as isize),
    );
    t = ixheaac_mult32_shl(ixheaac_sub32_sat(r1, r3), c_54);
    r1 = ixheaac_add32_sat(r1, r3);
    temp1 = ixheaac_add32_sat(*buf1a.offset(0 as core::ffi::c_int as isize), r1);
    r1 = ixheaac_add32_sat(temp1, ixheaac_mult32_shl(r1, c_55) << 1 as core::ffi::c_int);
    r3 = ixheaac_sub32_sat(r1, t);
    r1 = ixheaac_add32_sat(r1, t);
    t = ixheaac_mult32_shl(ixheaac_add32_sat(r4, r2), c_51);
    r4 = ixheaac_add32_sat(t, ixheaac_mult32_shl(r4, c_52) << 1 as core::ffi::c_int);
    r2 = ixheaac_add32_sat(t, ixheaac_mult32_shl(r2, c_53));
    s1 = ixheaac_add32_sat(
        *buf1a.offset(3 as core::ffi::c_int as isize),
        *buf1a.offset(9 as core::ffi::c_int as isize),
    );
    s4 = ixheaac_sub32_sat(
        *buf1a.offset(3 as core::ffi::c_int as isize),
        *buf1a.offset(9 as core::ffi::c_int as isize),
    );
    s3 = ixheaac_add32_sat(
        *buf1a.offset(5 as core::ffi::c_int as isize),
        *buf1a.offset(7 as core::ffi::c_int as isize),
    );
    s2 = ixheaac_sub32_sat(
        *buf1a.offset(5 as core::ffi::c_int as isize),
        *buf1a.offset(7 as core::ffi::c_int as isize),
    );
    t = ixheaac_mult32_shl(ixheaac_sub32_sat(s1, s3), c_54);
    s1 = ixheaac_add32_sat(s1, s3);
    temp2 = ixheaac_add32_sat(*buf1a.offset(1 as core::ffi::c_int as isize), s1);
    s1 = ixheaac_add32_sat(temp2, ixheaac_mult32_shl(s1, c_55) << 1 as core::ffi::c_int);
    s3 = ixheaac_sub32_sat(s1, t);
    s1 = ixheaac_add32_sat(s1, t);
    t = ixheaac_mult32_shl(ixheaac_add32_sat(s4, s2), c_51);
    s4 = ixheaac_add32_sat(t, ixheaac_mult32_shl(s4, c_52) << 1 as core::ffi::c_int);
    s2 = ixheaac_add32_sat(t, ixheaac_mult32_shl(s2, c_53));
    let fresh128 = buf2;
    buf2 = buf2.offset(1);
    *fresh128 = temp1;
    let fresh129 = buf2;
    buf2 = buf2.offset(1);
    *fresh129 = temp2;
    let fresh130 = buf2;
    buf2 = buf2.offset(1);
    *fresh130 = ixheaac_add32_sat(r1, s2);
    let fresh131 = buf2;
    buf2 = buf2.offset(1);
    *fresh131 = ixheaac_sub32_sat(s1, r2);
    let fresh132 = buf2;
    buf2 = buf2.offset(1);
    *fresh132 = ixheaac_sub32_sat(r3, s4);
    let fresh133 = buf2;
    buf2 = buf2.offset(1);
    *fresh133 = ixheaac_add32_sat(s3, r4);
    let fresh134 = buf2;
    buf2 = buf2.offset(1);
    *fresh134 = ixheaac_add32_sat(r3, s4);
    let fresh135 = buf2;
    buf2 = buf2.offset(1);
    *fresh135 = ixheaac_sub32_sat(s3, r4);
    let fresh136 = buf2;
    buf2 = buf2.offset(1);
    *fresh136 = ixheaac_sub32_sat(r1, s2);
    let fresh137 = buf2;
    buf2 = buf2.offset(1);
    *fresh137 = ixheaac_add32_sat(s1, r2);
    buf1a = buf1;
    let fresh138 = buf1;
    buf1 = buf1.offset(1);
    *fresh138 = *inp.offset(320 as core::ffi::c_int as isize);
    let fresh139 = buf1;
    buf1 = buf1.offset(1);
    *fresh139 = *inp.offset(321 as core::ffi::c_int as isize);
    let fresh140 = buf1;
    buf1 = buf1.offset(1);
    *fresh140 = *inp.offset(512 as core::ffi::c_int as isize);
    let fresh141 = buf1;
    buf1 = buf1.offset(1);
    *fresh141 = *inp.offset(513 as core::ffi::c_int as isize);
    let fresh142 = buf1;
    buf1 = buf1.offset(1);
    *fresh142 = *inp.offset(704 as core::ffi::c_int as isize);
    let fresh143 = buf1;
    buf1 = buf1.offset(1);
    *fresh143 = *inp.offset(705 as core::ffi::c_int as isize);
    let fresh144 = buf1;
    buf1 = buf1.offset(1);
    *fresh144 = *inp.offset(896 as core::ffi::c_int as isize);
    let fresh145 = buf1;
    buf1 = buf1.offset(1);
    *fresh145 = *inp.offset(897 as core::ffi::c_int as isize);
    let fresh146 = buf1;
    buf1 = buf1.offset(1);
    *fresh146 = *inp.offset(128 as core::ffi::c_int as isize);
    let fresh147 = buf1;
    buf1 = buf1.offset(1);
    *fresh147 = *inp.offset(129 as core::ffi::c_int as isize);
    r1 = ixheaac_add32_sat(
        *buf1a.offset(2 as core::ffi::c_int as isize),
        *buf1a.offset(8 as core::ffi::c_int as isize),
    );
    r4 = ixheaac_sub32_sat(
        *buf1a.offset(2 as core::ffi::c_int as isize),
        *buf1a.offset(8 as core::ffi::c_int as isize),
    );
    r3 = ixheaac_add32_sat(
        *buf1a.offset(4 as core::ffi::c_int as isize),
        *buf1a.offset(6 as core::ffi::c_int as isize),
    );
    r2 = ixheaac_sub32_sat(
        *buf1a.offset(4 as core::ffi::c_int as isize),
        *buf1a.offset(6 as core::ffi::c_int as isize),
    );
    t = ixheaac_mult32_shl(ixheaac_sub32_sat(r1, r3), c_54);
    r1 = ixheaac_add32_sat(r1, r3);
    temp1 = ixheaac_add32_sat(*buf1a.offset(0 as core::ffi::c_int as isize), r1);
    r1 = ixheaac_add32_sat(temp1, ixheaac_mult32_shl(r1, c_55) << 1 as core::ffi::c_int);
    r3 = ixheaac_sub32_sat(r1, t);
    r1 = ixheaac_add32_sat(r1, t);
    t = ixheaac_mult32_shl(ixheaac_add32_sat(r4, r2), c_51);
    r4 = ixheaac_add32_sat(t, ixheaac_mult32_shl(r4, c_52) << 1 as core::ffi::c_int);
    r2 = ixheaac_add32_sat(t, ixheaac_mult32_shl(r2, c_53));
    s1 = ixheaac_add32_sat(
        *buf1a.offset(3 as core::ffi::c_int as isize),
        *buf1a.offset(9 as core::ffi::c_int as isize),
    );
    s4 = ixheaac_sub32_sat(
        *buf1a.offset(3 as core::ffi::c_int as isize),
        *buf1a.offset(9 as core::ffi::c_int as isize),
    );
    s3 = ixheaac_add32_sat(
        *buf1a.offset(5 as core::ffi::c_int as isize),
        *buf1a.offset(7 as core::ffi::c_int as isize),
    );
    s2 = ixheaac_sub32_sat(
        *buf1a.offset(5 as core::ffi::c_int as isize),
        *buf1a.offset(7 as core::ffi::c_int as isize),
    );
    t = ixheaac_mult32_shl(ixheaac_sub32_sat(s1, s3), c_54);
    s1 = ixheaac_add32_sat(s1, s3);
    temp2 = ixheaac_add32_sat(*buf1a.offset(1 as core::ffi::c_int as isize), s1);
    s1 = ixheaac_add32_sat(temp2, ixheaac_mult32_shl(s1, c_55) << 1 as core::ffi::c_int);
    s3 = ixheaac_sub32_sat(s1, t);
    s1 = ixheaac_add32_sat(s1, t);
    t = ixheaac_mult32_shl(ixheaac_add32_sat(s4, s2), c_51);
    s4 = ixheaac_add32_sat(t, ixheaac_mult32_shl(s4, c_52) << 1 as core::ffi::c_int);
    s2 = ixheaac_add32_sat(t, ixheaac_mult32_shl(s2, c_53));
    let fresh148 = buf2;
    buf2 = buf2.offset(1);
    *fresh148 = temp1;
    let fresh149 = buf2;
    buf2 = buf2.offset(1);
    *fresh149 = temp2;
    let fresh150 = buf2;
    buf2 = buf2.offset(1);
    *fresh150 = ixheaac_add32_sat(r1, s2);
    let fresh151 = buf2;
    buf2 = buf2.offset(1);
    *fresh151 = ixheaac_sub32_sat(s1, r2);
    let fresh152 = buf2;
    buf2 = buf2.offset(1);
    *fresh152 = ixheaac_sub32_sat(r3, s4);
    let fresh153 = buf2;
    buf2 = buf2.offset(1);
    *fresh153 = ixheaac_add32_sat(s3, r4);
    let fresh154 = buf2;
    buf2 = buf2.offset(1);
    *fresh154 = ixheaac_add32_sat(r3, s4);
    let fresh155 = buf2;
    buf2 = buf2.offset(1);
    *fresh155 = ixheaac_sub32_sat(s3, r4);
    let fresh156 = buf2;
    buf2 = buf2.offset(1);
    *fresh156 = ixheaac_sub32_sat(r1, s2);
    let fresh157 = buf2;
    buf2 = buf2.offset(1);
    *fresh157 = ixheaac_add32_sat(s1, r2);
    buf1a = buf1;
    let fresh158 = buf1;
    buf1 = buf1.offset(1);
    *fresh158 = *inp.offset(640 as core::ffi::c_int as isize);
    let fresh159 = buf1;
    buf1 = buf1.offset(1);
    *fresh159 = *inp.offset(641 as core::ffi::c_int as isize);
    let fresh160 = buf1;
    buf1 = buf1.offset(1);
    *fresh160 = *inp.offset(832 as core::ffi::c_int as isize);
    let fresh161 = buf1;
    buf1 = buf1.offset(1);
    *fresh161 = *inp.offset(833 as core::ffi::c_int as isize);
    let fresh162 = buf1;
    buf1 = buf1.offset(1);
    *fresh162 = *inp.offset(64 as core::ffi::c_int as isize);
    let fresh163 = buf1;
    buf1 = buf1.offset(1);
    *fresh163 = *inp.offset(65 as core::ffi::c_int as isize);
    let fresh164 = buf1;
    buf1 = buf1.offset(1);
    *fresh164 = *inp.offset(256 as core::ffi::c_int as isize);
    let fresh165 = buf1;
    buf1 = buf1.offset(1);
    *fresh165 = *inp.offset(257 as core::ffi::c_int as isize);
    let fresh166 = buf1;
    buf1 = buf1.offset(1);
    *fresh166 = *inp.offset(448 as core::ffi::c_int as isize);
    let fresh167 = buf1;
    buf1 = buf1.offset(1);
    *fresh167 = *inp.offset(449 as core::ffi::c_int as isize);
    r1 = ixheaac_add32_sat(
        *buf1a.offset(2 as core::ffi::c_int as isize),
        *buf1a.offset(8 as core::ffi::c_int as isize),
    );
    r4 = ixheaac_sub32_sat(
        *buf1a.offset(2 as core::ffi::c_int as isize),
        *buf1a.offset(8 as core::ffi::c_int as isize),
    );
    r3 = ixheaac_add32_sat(
        *buf1a.offset(4 as core::ffi::c_int as isize),
        *buf1a.offset(6 as core::ffi::c_int as isize),
    );
    r2 = ixheaac_sub32_sat(
        *buf1a.offset(4 as core::ffi::c_int as isize),
        *buf1a.offset(6 as core::ffi::c_int as isize),
    );
    t = ixheaac_mult32_shl(ixheaac_sub32_sat(r1, r3), c_54);
    r1 = ixheaac_add32_sat(r1, r3);
    temp1 = ixheaac_add32_sat(*buf1a.offset(0 as core::ffi::c_int as isize), r1);
    r1 = ixheaac_add32_sat(temp1, ixheaac_mult32_shl(r1, c_55) << 1 as core::ffi::c_int);
    r3 = ixheaac_sub32_sat(r1, t);
    r1 = ixheaac_add32_sat(r1, t);
    t = ixheaac_mult32_shl(ixheaac_add32_sat(r4, r2), c_51);
    r4 = ixheaac_add32_sat(t, ixheaac_mult32_shl(r4, c_52) << 1 as core::ffi::c_int);
    r2 = ixheaac_add32_sat(t, ixheaac_mult32_shl(r2, c_53));
    s1 = ixheaac_add32_sat(
        *buf1a.offset(3 as core::ffi::c_int as isize),
        *buf1a.offset(9 as core::ffi::c_int as isize),
    );
    s4 = ixheaac_sub32_sat(
        *buf1a.offset(3 as core::ffi::c_int as isize),
        *buf1a.offset(9 as core::ffi::c_int as isize),
    );
    s3 = ixheaac_add32_sat(
        *buf1a.offset(5 as core::ffi::c_int as isize),
        *buf1a.offset(7 as core::ffi::c_int as isize),
    );
    s2 = ixheaac_sub32_sat(
        *buf1a.offset(5 as core::ffi::c_int as isize),
        *buf1a.offset(7 as core::ffi::c_int as isize),
    );
    t = ixheaac_mult32_shl(ixheaac_sub32_sat(s1, s3), c_54);
    s1 = ixheaac_add32_sat(s1, s3);
    temp2 = ixheaac_add32_sat(*buf1a.offset(1 as core::ffi::c_int as isize), s1);
    s1 = ixheaac_add32_sat(temp2, ixheaac_mult32_shl(s1, c_55) << 1 as core::ffi::c_int);
    s3 = ixheaac_sub32_sat(s1, t);
    s1 = ixheaac_add32_sat(s1, t);
    t = ixheaac_mult32_shl(ixheaac_add32_sat(s4, s2), c_51);
    s4 = ixheaac_add32_sat(t, ixheaac_mult32_shl(s4, c_52) << 1 as core::ffi::c_int);
    s2 = ixheaac_add32_sat(t, ixheaac_mult32_shl(s2, c_53));
    let fresh168 = buf2;
    buf2 = buf2.offset(1);
    *fresh168 = temp1;
    let fresh169 = buf2;
    buf2 = buf2.offset(1);
    *fresh169 = temp2;
    let fresh170 = buf2;
    buf2 = buf2.offset(1);
    *fresh170 = ixheaac_add32_sat(r1, s2);
    let fresh171 = buf2;
    buf2 = buf2.offset(1);
    *fresh171 = ixheaac_sub32_sat(s1, r2);
    let fresh172 = buf2;
    buf2 = buf2.offset(1);
    *fresh172 = ixheaac_sub32_sat(r3, s4);
    let fresh173 = buf2;
    buf2 = buf2.offset(1);
    *fresh173 = ixheaac_add32_sat(s3, r4);
    let fresh174 = buf2;
    buf2 = buf2.offset(1);
    *fresh174 = ixheaac_add32_sat(r3, s4);
    let fresh175 = buf2;
    buf2 = buf2.offset(1);
    *fresh175 = ixheaac_sub32_sat(s3, r4);
    let fresh176 = buf2;
    buf2 = buf2.offset(1);
    *fresh176 = ixheaac_sub32_sat(r1, s2);
    let fresh177 = buf2;
    buf2 = buf2.offset(1);
    *fresh177 = ixheaac_add32_sat(s1, r2);
    buf1a = buf1;
    n = 0 as core::ffi::c_int as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < FFT5 {
        xr_0 = *fft3outptr.offset(0 as core::ffi::c_int as isize);
        xi_0 = *fft3outptr.offset(1 as core::ffi::c_int as isize);
        xr_1 = *fft3outptr.offset(10 as core::ffi::c_int as isize);
        xi_1 = *fft3outptr.offset(11 as core::ffi::c_int as isize);
        xr_2 = *fft3outptr.offset(20 as core::ffi::c_int as isize);
        xi_2 = *fft3outptr.offset(21 as core::ffi::c_int as isize);
        x_01_r = ixheaac_add32_sat(xr_0, xr_1);
        x_01_i = ixheaac_add32_sat(xi_0, xi_1);
        add_r = ixheaac_add32_sat(xr_1, xr_2);
        add_i = ixheaac_add32_sat(xi_1, xi_2);
        sub_r = ixheaac_sub32_sat(xr_1, xr_2);
        sub_i = ixheaac_sub32_sat(xi_1, xi_2);
        p1 = add_r >> 1 as core::ffi::c_int;
        p2 = ixheaac_mult32_shl(sub_i, sinmu);
        p3 = ixheaac_mult32_shl(sub_r, sinmu);
        p4 = add_i >> 1 as core::ffi::c_int;
        temp = ixheaac_sub32_sat(xr_0, p1);
        temp1 = ixheaac_add32_sat(xi_0, p3);
        temp2 = ixheaac_sub32_sat(xi_0, p3);
        let fresh178 = n;
        n = n + 1;
        idx = ((*ptr_re_arr_tab_sml_240.offset(fresh178 as isize) as core::ffi::c_int)
            << 1 as core::ffi::c_int) as WORD32;
        *op.offset(idx as isize) = ixheaac_add32_sat(x_01_r, xr_2);
        *op.offset((idx as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = ixheaac_add32_sat(
            x_01_i,
            xi_2,
        );
        let fresh179 = n;
        n = n + 1;
        idx = ((*ptr_re_arr_tab_sml_240.offset(fresh179 as isize) as core::ffi::c_int)
            << 1 as core::ffi::c_int) as WORD32;
        *op.offset(idx as isize) = ixheaac_add32_sat(temp, p2);
        *op.offset((idx as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = ixheaac_sub32_sat(
            temp2,
            p4,
        );
        let fresh180 = n;
        n = n + 1;
        idx = ((*ptr_re_arr_tab_sml_240.offset(fresh180 as isize) as core::ffi::c_int)
            << 1 as core::ffi::c_int) as WORD32;
        *op.offset(idx as isize) = ixheaac_sub32_sat(temp, p2);
        *op.offset((idx as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = ixheaac_sub32_sat(
            temp1,
            p4,
        );
        fft3outptr = fft3outptr.offset(2 as core::ffi::c_int as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_fft_120(
    mut imdct_tables_ptr: *mut ia_aac_dec_imdct_tables_struct,
    mut npoints: WORD32,
    mut ptr_x: *mut WORD32,
    mut ptr_y: *mut WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut buf1: *mut WORD32 = 0 as *mut WORD32;
    let mut buf2: *mut WORD32 = 0 as *mut WORD32;
    let mut inp: *mut WORD32 = 0 as *mut WORD32;
    let mut op: *mut WORD32 = 0 as *mut WORD32;
    inp = ptr_x;
    op = ptr_y;
    ixheaacd_dec_rearrange_short(
        inp,
        op,
        60 as WORD32,
        ((*imdct_tables_ptr).re_arr_tab_4).as_mut_ptr(),
    );
    buf1 = op;
    buf2 = inp;
    i = 0 as core::ffi::c_int as WORD32;
    while i < FFT15 {
        let mut x_0: WORD32 = 0;
        let mut x_1: WORD32 = 0;
        let mut x_2: WORD32 = 0;
        let mut x_3: WORD32 = 0;
        let mut x_4: WORD32 = 0;
        let mut x_5: WORD32 = 0;
        let mut x_6: WORD32 = 0;
        let mut x_7: WORD32 = 0;
        let mut y0: *mut WORD32 = 0 as *mut WORD32;
        let mut y1: *mut WORD32 = 0 as *mut WORD32;
        let mut y2: *mut WORD32 = 0 as *mut WORD32;
        let mut y3: *mut WORD32 = 0 as *mut WORD32;
        let mut x0: *mut WORD32 = 0 as *mut WORD32;
        let mut xh0_0: WORD32 = 0;
        let mut xh1_0: WORD32 = 0;
        let mut xh0_1: WORD32 = 0;
        let mut xh1_1: WORD32 = 0;
        let mut xl0_0: WORD32 = 0;
        let mut xl1_0: WORD32 = 0;
        let mut xl0_1: WORD32 = 0;
        let mut xl1_1: WORD32 = 0;
        let mut h2: WORD32 = 0;
        let mut n00: WORD32 = 0;
        let mut n01: WORD32 = 0;
        let mut n10: WORD32 = 0;
        let mut n11: WORD32 = 0;
        let mut n20: WORD32 = 0;
        let mut n21: WORD32 = 0;
        let mut n30: WORD32 = 0;
        let mut n31: WORD32 = 0;
        ptr_x = buf1;
        ptr_y = buf2;
        npoints = 4 as core::ffi::c_int as WORD32;
        h2 = 0 as core::ffi::c_int as WORD32;
        y0 = ptr_y;
        y2 = ptr_y.offset(npoints as isize);
        x0 = ptr_x;
        y1 = y0.offset((npoints >> 1 as core::ffi::c_int) as isize);
        y3 = y2.offset((npoints >> 1 as core::ffi::c_int) as isize);
        x_0 = *x0.offset(0 as core::ffi::c_int as isize);
        x_1 = *x0.offset(1 as core::ffi::c_int as isize);
        x_2 = *x0.offset(2 as core::ffi::c_int as isize);
        x_3 = *x0.offset(3 as core::ffi::c_int as isize);
        x_4 = *x0.offset(4 as core::ffi::c_int as isize);
        x_5 = *x0.offset(5 as core::ffi::c_int as isize);
        x_6 = *x0.offset(6 as core::ffi::c_int as isize);
        x_7 = *x0.offset(7 as core::ffi::c_int as isize);
        x0 = x0.offset(8 as core::ffi::c_int as isize);
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
        *y0.offset((2 as WORD32 * h2) as isize) = n00;
        *y0
            .offset(
                (2 as core::ffi::c_int * h2 as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            ) = n01;
        *y1.offset((2 as WORD32 * h2) as isize) = n10;
        *y1
            .offset(
                (2 as core::ffi::c_int * h2 as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            ) = n11;
        *y2.offset((2 as WORD32 * h2) as isize) = n20;
        *y2
            .offset(
                (2 as core::ffi::c_int * h2 as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            ) = n21;
        *y3.offset((2 as WORD32 * h2) as isize) = n30;
        *y3
            .offset(
                (2 as core::ffi::c_int * h2 as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            ) = n31;
        buf1 = buf1.offset((FFT4 * 2 as core::ffi::c_int) as isize);
        buf2 = buf2.offset((FFT4 * 2 as core::ffi::c_int) as isize);
        i += 1;
    }
    ixheaacd_dec_rearrange_short(
        inp,
        op,
        60 as WORD32,
        ((*imdct_tables_ptr).re_arr_tab_15_4).as_mut_ptr(),
    );
    buf1 = op;
    buf2 = inp;
    i = 0 as core::ffi::c_int as WORD32;
    while i < FFT4 {
        ixheaacd_fft_960_15(buf1, buf2, imdct_tables_ptr);
        buf1 = buf1.offset((FFT15 * 2 as core::ffi::c_int) as isize);
        buf2 = buf2.offset((FFT15 * 2 as core::ffi::c_int) as isize);
        i += 1;
    }
    ixheaacd_dec_rearrange_short(
        inp,
        op,
        60 as WORD32,
        ((*imdct_tables_ptr).re_arr_tab_120).as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_fft_960_15(
    mut inp: *mut WORD32,
    mut op: *mut WORD32,
    mut imdct_tables_ptr: *mut ia_aac_dec_imdct_tables_struct,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut buf1: *mut WORD32 = 0 as *mut WORD32;
    let mut buf2: *mut WORD32 = 0 as *mut WORD32;
    ixheaacd_dec_rearrange_short(
        inp,
        op,
        FFT15,
        ((*imdct_tables_ptr).re_arr_tab_5).as_mut_ptr(),
    );
    buf1 = op;
    buf2 = inp;
    i = 0 as core::ffi::c_int as WORD32;
    while i < FFT3 {
        ixheaacd_fft_5(buf1, buf2);
        buf1 = buf1.offset((FFT5 * 2 as core::ffi::c_int) as isize);
        buf2 = buf2.offset((FFT5 * 2 as core::ffi::c_int) as isize);
        i += 1;
    }
    ixheaacd_dec_rearrange_short(
        inp,
        op,
        FFT15,
        ((*imdct_tables_ptr).re_arr_tab_3).as_mut_ptr(),
    );
    buf1 = op;
    buf2 = inp;
    i = 0 as core::ffi::c_int as WORD32;
    while i < FFT5 {
        ixheaacd_fft_3(buf1, buf2);
        buf1 = buf1.offset((FFT3 * 2 as core::ffi::c_int) as isize);
        buf2 = buf2.offset((FFT3 * 2 as core::ffi::c_int) as isize);
        i += 1;
    }
    ixheaacd_dec_rearrange_short(
        inp,
        op,
        FFT15,
        ((*imdct_tables_ptr).re_arr_tab_sml).as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_fft_3(
    mut inp: *mut WORD32,
    mut op: *mut WORD32,
) -> VOID {
    let mut add_r: WORD32 = 0;
    let mut sub_r: WORD32 = 0;
    let mut add_i: WORD32 = 0;
    let mut sub_i: WORD32 = 0;
    let mut x_01_r: WORD32 = 0;
    let mut x_01_i: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut p1: WORD32 = 0;
    let mut p2: WORD32 = 0;
    let mut p3: WORD32 = 0;
    let mut p4: WORD32 = 0;
    let mut sinmu: WORD32 = 1859775393 as WORD32;
    x_01_r = ixheaac_add32_sat(
        *inp.offset(0 as core::ffi::c_int as isize),
        *inp.offset(2 as core::ffi::c_int as isize),
    );
    x_01_i = ixheaac_add32_sat(
        *inp.offset(1 as core::ffi::c_int as isize),
        *inp.offset(3 as core::ffi::c_int as isize),
    );
    add_r = ixheaac_add32_sat(
        *inp.offset(2 as core::ffi::c_int as isize),
        *inp.offset(4 as core::ffi::c_int as isize),
    );
    add_i = ixheaac_add32_sat(
        *inp.offset(3 as core::ffi::c_int as isize),
        *inp.offset(5 as core::ffi::c_int as isize),
    );
    sub_r = ixheaac_sub32_sat(
        *inp.offset(2 as core::ffi::c_int as isize),
        *inp.offset(4 as core::ffi::c_int as isize),
    );
    sub_i = ixheaac_sub32_sat(
        *inp.offset(3 as core::ffi::c_int as isize),
        *inp.offset(5 as core::ffi::c_int as isize),
    );
    p1 = add_r >> 1 as core::ffi::c_int;
    p2 = ixheaac_mult32_shl(sub_i, sinmu);
    p3 = ixheaac_mult32_shl(sub_r, sinmu);
    p4 = add_i >> 1 as core::ffi::c_int;
    temp = ixheaac_sub32_sat(*inp.offset(0 as core::ffi::c_int as isize), p1);
    *op.offset(0 as core::ffi::c_int as isize) = ixheaac_add32_sat(
        x_01_r,
        *inp.offset(4 as core::ffi::c_int as isize),
    );
    *op.offset(1 as core::ffi::c_int as isize) = ixheaac_add32_sat(
        x_01_i,
        *inp.offset(5 as core::ffi::c_int as isize),
    );
    *op.offset(2 as core::ffi::c_int as isize) = ixheaac_add32_sat(temp, p2);
    *op.offset(3 as core::ffi::c_int as isize) = ixheaac_sub32_sat(
        ixheaac_sub32_sat(*inp.offset(1 as core::ffi::c_int as isize), p3),
        p4,
    );
    *op.offset(4 as core::ffi::c_int as isize) = ixheaac_sub32_sat(temp, p2);
    *op.offset(5 as core::ffi::c_int as isize) = ixheaac_sub32_sat(
        ixheaac_add32_sat(*inp.offset(1 as core::ffi::c_int as isize), p3),
        p4,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_fft_5(
    mut inp: *mut WORD32,
    mut op: *mut WORD32,
) -> VOID {
    let mut c_51: WORD32 = 2042378317 as WORD32;
    let mut c_52: WORD32 = -(1652318768 as WORD32);
    let mut c_53: WORD32 = -(780119100 as WORD32);
    let mut c_54: WORD32 = 1200479854 as WORD32;
    let mut c_55: WORD32 = -(1342177280 as WORD32);
    let mut r1: WORD32 = 0;
    let mut r2: WORD32 = 0;
    let mut r3: WORD32 = 0;
    let mut r4: WORD32 = 0;
    let mut s1: WORD32 = 0;
    let mut s2: WORD32 = 0;
    let mut s3: WORD32 = 0;
    let mut s4: WORD32 = 0;
    let mut t: WORD32 = 0;
    let mut temp1: WORD32 = 0;
    let mut temp2: WORD32 = 0;
    r1 = ixheaac_add32_sat(
        *inp.offset(2 as core::ffi::c_int as isize),
        *inp.offset(8 as core::ffi::c_int as isize),
    );
    r4 = ixheaac_sub32_sat(
        *inp.offset(2 as core::ffi::c_int as isize),
        *inp.offset(8 as core::ffi::c_int as isize),
    );
    r3 = ixheaac_add32_sat(
        *inp.offset(4 as core::ffi::c_int as isize),
        *inp.offset(6 as core::ffi::c_int as isize),
    );
    r2 = ixheaac_sub32_sat(
        *inp.offset(4 as core::ffi::c_int as isize),
        *inp.offset(6 as core::ffi::c_int as isize),
    );
    t = ixheaac_mult32_shl(ixheaac_sub32_sat(r1, r3), c_54);
    r1 = ixheaac_add32_sat(r1, r3);
    temp1 = ixheaac_add32_sat(*inp.offset(0 as core::ffi::c_int as isize), r1);
    r1 = ixheaac_add32_sat(
        temp1,
        ixheaac_shl32_sat(ixheaac_mult32_shl(r1, c_55), 1 as WORD),
    );
    r3 = ixheaac_sub32_sat(r1, t);
    r1 = ixheaac_add32_sat(r1, t);
    t = ixheaac_mult32_shl(ixheaac_add32_sat(r4, r2), c_51);
    r4 = ixheaac_add32_sat(
        t,
        ixheaac_shl32_sat(ixheaac_mult32_shl(r4, c_52), 1 as WORD),
    );
    r2 = ixheaac_add32_sat(t, ixheaac_mult32_shl(r2, c_53));
    s1 = ixheaac_add32_sat(
        *inp.offset(3 as core::ffi::c_int as isize),
        *inp.offset(9 as core::ffi::c_int as isize),
    );
    s4 = ixheaac_sub32_sat(
        *inp.offset(3 as core::ffi::c_int as isize),
        *inp.offset(9 as core::ffi::c_int as isize),
    );
    s3 = ixheaac_add32_sat(
        *inp.offset(5 as core::ffi::c_int as isize),
        *inp.offset(7 as core::ffi::c_int as isize),
    );
    s2 = ixheaac_sub32_sat(
        *inp.offset(5 as core::ffi::c_int as isize),
        *inp.offset(7 as core::ffi::c_int as isize),
    );
    t = ixheaac_mult32_shl(ixheaac_sub32_sat(s1, s3), c_54);
    s1 = ixheaac_add32_sat(s1, s3);
    temp2 = ixheaac_add32_sat(*inp.offset(1 as core::ffi::c_int as isize), s1);
    s1 = ixheaac_add32_sat(
        temp2,
        ixheaac_shl32_sat(ixheaac_mult32_shl(s1, c_55), 1 as WORD),
    );
    s3 = ixheaac_sub32_sat(s1, t);
    s1 = ixheaac_add32_sat(s1, t);
    t = ixheaac_mult32_shl(ixheaac_add32_sat(s4, s2), c_51);
    s4 = ixheaac_add32_sat(
        t,
        ixheaac_shl32_sat(ixheaac_mult32_shl(s4, c_52), 1 as WORD),
    );
    s2 = ixheaac_add32_sat(t, ixheaac_mult32_shl(s2, c_53));
    *op.offset(0 as core::ffi::c_int as isize) = temp1;
    *op.offset(1 as core::ffi::c_int as isize) = temp2;
    *op.offset(2 as core::ffi::c_int as isize) = ixheaac_add32_sat(r1, s2);
    *op.offset(3 as core::ffi::c_int as isize) = ixheaac_sub32_sat(s1, r2);
    *op.offset(4 as core::ffi::c_int as isize) = ixheaac_sub32_sat(r3, s4);
    *op.offset(5 as core::ffi::c_int as isize) = ixheaac_add32_sat(s3, r4);
    *op.offset(6 as core::ffi::c_int as isize) = ixheaac_add32_sat(r3, s4);
    *op.offset(7 as core::ffi::c_int as isize) = ixheaac_sub32_sat(s3, r4);
    *op.offset(8 as core::ffi::c_int as isize) = ixheaac_sub32_sat(r1, s2);
    *op.offset(9 as core::ffi::c_int as isize) = ixheaac_add32_sat(s1, r2);
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_fft_480_ld(
    mut inp: *mut WORD32,
    mut op: *mut WORD32,
    mut imdct_tables_ptr: *mut ia_aac_dec_imdct_tables_struct,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut buf1: *mut WORD32 = 0 as *mut WORD32;
    let mut buf2: *mut WORD32 = 0 as *mut WORD32;
    let mut re_arr_tab_sml_240_ptr: *mut UWORD8 = 0 as *mut UWORD8;
    (Some(ixheaacd_aac_ld_dec_rearrange.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(inp, op, MDCT_LEN_BY2, ((*imdct_tables_ptr).re_arr_tab_16).as_mut_ptr());
    buf1 = op;
    buf2 = inp;
    i = 0 as core::ffi::c_int as WORD32;
    while i < FFT15 {
        (Some(ixheaacd_fft32x32_ld2.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(imdct_tables_ptr, 16 as WORD32, buf1, buf2);
        buf1 = buf1.offset(32 as core::ffi::c_int as isize);
        buf2 = buf2.offset(32 as core::ffi::c_int as isize);
        i += 1;
    }
    re_arr_tab_sml_240_ptr = ((*imdct_tables_ptr).re_arr_tab_sml_240).as_mut_ptr();
    buf1 = inp;
    i = 0 as core::ffi::c_int as WORD32;
    while i < FFT16 {
        (Some(ixheaacd_fft_15_ld.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(buf1, op, ixheaacd_fft5out.as_mut_ptr(), re_arr_tab_sml_240_ptr);
        re_arr_tab_sml_240_ptr = re_arr_tab_sml_240_ptr.offset(FFT15 as isize);
        buf1 = buf1.offset(2 as core::ffi::c_int as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_pre_twiddle_960(
    mut xptr: *mut WORD32,
    mut data: *mut WORD32,
    mut n: WORD32,
    mut cos_sin_ptr: *mut WORD32,
    mut neg_expo: WORD32,
) -> VOID {
    let mut npoints_4: WORD = 0;
    let mut i: WORD = 0;
    let mut tempr: WORD32 = 0;
    let mut tempi: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut c: WORD32 = 0;
    let mut c1: WORD32 = 0;
    let mut s: WORD32 = 0;
    let mut s1: WORD32 = 0;
    let mut in_ptr1: *mut WORD32 = 0 as *mut WORD32;
    let mut in_ptr2: *mut WORD32 = 0 as *mut WORD32;
    let mut xprt1: *mut WORD32 = xptr
        .offset((n as core::ffi::c_int - 1 as core::ffi::c_int) as isize);
    npoints_4 = (n >> 2 as core::ffi::c_int) as WORD;
    in_ptr1 = data;
    in_ptr2 = data.offset(n as isize).offset(-(1 as core::ffi::c_int as isize));
    i = 0 as core::ffi::c_int as WORD;
    while i < npoints_4 {
        let fresh197 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        c = *fresh197;
        let fresh198 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        s = *fresh198;
        let fresh199 = in_ptr1;
        in_ptr1 = in_ptr1.offset(1);
        tempr = *fresh199;
        let fresh200 = in_ptr2;
        in_ptr2 = in_ptr2.offset(-1);
        tempi = *fresh200;
        temp = -ixheaac_add32(
            ixheaac_mult32x32in32(tempr, c),
            ixheaac_mult32x32in32(tempi, s),
        );
        let fresh201 = xptr;
        xptr = xptr.offset(1);
        *fresh201 = ixheaac_shr32_dir_sat(temp, neg_expo as WORD);
        temp = -ixheaac_sub32(
            ixheaac_mult32x32in32(tempi, c),
            ixheaac_mult32x32in32(tempr, s),
        );
        let fresh202 = xptr;
        xptr = xptr.offset(1);
        *fresh202 = ixheaac_shr32_dir_sat(temp, neg_expo as WORD);
        let fresh203 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        c1 = *fresh203;
        let fresh204 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        s1 = *fresh204;
        let fresh205 = in_ptr1;
        in_ptr1 = in_ptr1.offset(1);
        tempi = *fresh205;
        let fresh206 = in_ptr2;
        in_ptr2 = in_ptr2.offset(-1);
        tempr = *fresh206;
        temp = -ixheaac_sub32(
            ixheaac_mult32x32in32(tempi, c1),
            ixheaac_mult32x32in32(tempr, s1),
        );
        let fresh207 = xprt1;
        xprt1 = xprt1.offset(-1);
        *fresh207 = ixheaac_shr32_dir_sat(temp, neg_expo as WORD);
        temp = -ixheaac_add32(
            ixheaac_mult32x32in32(tempr, c1),
            ixheaac_mult32x32in32(tempi, s1),
        );
        let fresh208 = xprt1;
        xprt1 = xprt1.offset(-1);
        *fresh208 = ixheaac_shr32_dir_sat(temp, neg_expo as WORD);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_pre_twiddle_120(
    mut xptr: *mut WORD32,
    mut data: *mut WORD32,
    mut n: WORD32,
    mut cos_sin_ptr: *mut WORD16,
    mut neg_expo: WORD32,
) -> VOID {
    let mut npoints_4: WORD = 0;
    let mut i: WORD = 0;
    let mut tempr: WORD32 = 0;
    let mut tempi: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut c: WORD16 = 0;
    let mut c1: WORD16 = 0;
    let mut s: WORD16 = 0;
    let mut s1: WORD16 = 0;
    let mut in_ptr1: *mut WORD32 = 0 as *mut WORD32;
    let mut in_ptr2: *mut WORD32 = 0 as *mut WORD32;
    let mut xprt1: *mut WORD32 = xptr
        .offset((n as core::ffi::c_int - 1 as core::ffi::c_int) as isize);
    npoints_4 = (n >> 2 as core::ffi::c_int) as WORD;
    in_ptr1 = data;
    in_ptr2 = data.offset(n as isize).offset(-(1 as core::ffi::c_int as isize));
    i = 0 as core::ffi::c_int as WORD;
    while i < npoints_4 {
        let fresh14 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        c = *fresh14;
        let fresh15 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        s = *fresh15;
        let fresh16 = in_ptr1;
        in_ptr1 = in_ptr1.offset(1);
        tempr = *fresh16;
        let fresh17 = in_ptr2;
        in_ptr2 = in_ptr2.offset(-1);
        tempi = *fresh17;
        temp = -ixheaac_add32(
            ixheaac_mult32x16in32(tempr, c),
            ixheaac_mult32x16in32(tempi, s),
        );
        let fresh18 = xptr;
        xptr = xptr.offset(1);
        *fresh18 = ixheaac_shr32_dir_sat(temp, neg_expo as WORD);
        temp = -ixheaac_sub32(
            ixheaac_mult32x16in32(tempi, c),
            ixheaac_mult32x16in32(tempr, s),
        );
        let fresh19 = xptr;
        xptr = xptr.offset(1);
        *fresh19 = ixheaac_shr32_dir_sat(temp, neg_expo as WORD);
        let fresh20 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        c1 = *fresh20;
        let fresh21 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        s1 = *fresh21;
        let fresh22 = in_ptr1;
        in_ptr1 = in_ptr1.offset(1);
        tempi = *fresh22;
        let fresh23 = in_ptr2;
        in_ptr2 = in_ptr2.offset(-1);
        tempr = *fresh23;
        temp = -ixheaac_sub32(
            ixheaac_mult32x16in32(tempi, c1),
            ixheaac_mult32x16in32(tempr, s1),
        );
        let fresh24 = xprt1;
        xprt1 = xprt1.offset(-1);
        *fresh24 = ixheaac_shr32_dir_sat(temp, neg_expo as WORD);
        temp = -ixheaac_add32(
            ixheaac_mult32x16in32(tempr, c1),
            ixheaac_mult32x16in32(tempi, s1),
        );
        let fresh25 = xprt1;
        xprt1 = xprt1.offset(-1);
        *fresh25 = ixheaac_shr32_dir_sat(temp, neg_expo as WORD);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_pre_twiddle(
    mut xptr: *mut WORD32,
    mut data: *mut WORD32,
    mut n: WORD32,
    mut cos_sin_ptr: *mut WORD32,
    mut neg_expo: WORD32,
) -> VOID {
    let mut npoints_4: WORD = 0;
    let mut i: WORD = 0;
    let mut tempr: WORD32 = 0;
    let mut tempi: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut c: WORD32 = 0;
    let mut c1: WORD32 = 0;
    let mut s: WORD32 = 0;
    let mut s1: WORD32 = 0;
    let mut in_ptr1: *mut WORD32 = 0 as *mut WORD32;
    let mut in_ptr2: *mut WORD32 = 0 as *mut WORD32;
    npoints_4 = (n >> 2 as core::ffi::c_int) as WORD;
    in_ptr1 = data;
    in_ptr2 = data.offset(n as isize).offset(-(1 as core::ffi::c_int as isize));
    if neg_expo >= 0 as core::ffi::c_int {
        i = (npoints_4 as core::ffi::c_int - 1 as core::ffi::c_int) as WORD;
        while i >= 0 as core::ffi::c_int {
            let fresh181 = cos_sin_ptr;
            cos_sin_ptr = cos_sin_ptr.offset(1);
            c = *fresh181;
            let fresh182 = cos_sin_ptr;
            cos_sin_ptr = cos_sin_ptr.offset(1);
            c1 = *fresh182;
            let fresh183 = cos_sin_ptr;
            cos_sin_ptr = cos_sin_ptr.offset(1);
            s = *fresh183;
            let fresh184 = cos_sin_ptr;
            cos_sin_ptr = cos_sin_ptr.offset(1);
            s1 = *fresh184;
            tempr = *in_ptr1;
            tempi = *in_ptr2;
            in_ptr1 = in_ptr1.offset(2 as core::ffi::c_int as isize);
            in_ptr2 = in_ptr2.offset(-(2 as core::ffi::c_int as isize));
            temp = -ixheaac_add32(ixheaac_mult32(tempr, c), ixheaac_mult32(tempi, s));
            let fresh185 = xptr;
            xptr = xptr.offset(1);
            *fresh185 = ixheaac_shr32(temp, neg_expo as WORD);
            temp = ixheaac_sub32(ixheaac_mult32(tempr, s), ixheaac_mult32(tempi, c));
            let fresh186 = xptr;
            xptr = xptr.offset(1);
            *fresh186 = ixheaac_shr32(temp, neg_expo as WORD);
            tempr = *in_ptr1;
            tempi = *in_ptr2;
            in_ptr1 = in_ptr1.offset(2 as core::ffi::c_int as isize);
            in_ptr2 = in_ptr2.offset(-(2 as core::ffi::c_int as isize));
            temp = -ixheaac_add32(ixheaac_mult32(tempr, c1), ixheaac_mult32(tempi, s1));
            let fresh187 = xptr;
            xptr = xptr.offset(1);
            *fresh187 = ixheaac_shr32(temp, neg_expo as WORD);
            temp = ixheaac_sub32(ixheaac_mult32(tempr, s1), ixheaac_mult32(tempi, c1));
            let fresh188 = xptr;
            xptr = xptr.offset(1);
            *fresh188 = ixheaac_shr32(temp, neg_expo as WORD);
            i -= 1;
        }
    } else {
        neg_expo = -neg_expo;
        i = (npoints_4 as core::ffi::c_int - 1 as core::ffi::c_int) as WORD;
        while i >= 0 as core::ffi::c_int {
            let fresh189 = cos_sin_ptr;
            cos_sin_ptr = cos_sin_ptr.offset(1);
            c = *fresh189;
            let fresh190 = cos_sin_ptr;
            cos_sin_ptr = cos_sin_ptr.offset(1);
            c1 = *fresh190;
            let fresh191 = cos_sin_ptr;
            cos_sin_ptr = cos_sin_ptr.offset(1);
            s = *fresh191;
            let fresh192 = cos_sin_ptr;
            cos_sin_ptr = cos_sin_ptr.offset(1);
            s1 = *fresh192;
            tempr = *in_ptr1;
            tempi = *in_ptr2;
            in_ptr1 = in_ptr1.offset(2 as core::ffi::c_int as isize);
            in_ptr2 = in_ptr2.offset(-(2 as core::ffi::c_int as isize));
            temp = -ixheaac_add32(ixheaac_mult32(tempr, c), ixheaac_mult32(tempi, s));
            let fresh193 = xptr;
            xptr = xptr.offset(1);
            *fresh193 = ixheaac_shl32(temp, neg_expo as WORD);
            temp = ixheaac_sub32(ixheaac_mult32(tempr, s), ixheaac_mult32(tempi, c));
            let fresh194 = xptr;
            xptr = xptr.offset(1);
            *fresh194 = ixheaac_shl32(temp, neg_expo as WORD);
            tempr = *in_ptr1;
            tempi = *in_ptr2;
            in_ptr1 = in_ptr1.offset(2 as core::ffi::c_int as isize);
            in_ptr2 = in_ptr2.offset(-(2 as core::ffi::c_int as isize));
            temp = -ixheaac_add32(ixheaac_mult32(tempr, c1), ixheaac_mult32(tempi, s1));
            let fresh195 = xptr;
            xptr = xptr.offset(1);
            *fresh195 = ixheaac_shl32(temp, neg_expo as WORD);
            temp = ixheaac_sub32(ixheaac_mult32(tempr, s1), ixheaac_mult32(tempi, c1));
            let fresh196 = xptr;
            xptr = xptr.offset(1);
            *fresh196 = ixheaac_shl32(temp, neg_expo as WORD);
            i -= 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_post_twiddle_120(
    mut out: *mut WORD32,
    mut x: *mut WORD32,
    mut cos_sin_ptr: *const WORD16,
    mut m: WORD,
) -> VOID {
    let mut i: WORD = 0;
    let mut c: WORD16 = 0;
    let mut c1: WORD16 = 0;
    let mut s: WORD16 = 0;
    let mut s1: WORD16 = 0;
    let mut tempr: WORD32 = 0;
    let mut tempi: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut in_ptr2: *mut WORD32 = x
        .offset((m as core::ffi::c_int - 1 as core::ffi::c_int) as isize);
    let mut in_ptr1: *mut WORD32 = x as *mut WORD32;
    let mut xptr: *mut WORD32 = out as *mut WORD32;
    let mut xptr1: *mut WORD32 = out
        .offset((m as core::ffi::c_int - 1 as core::ffi::c_int) as isize);
    i = 0 as core::ffi::c_int as WORD;
    while i < m {
        let fresh0 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        c = *fresh0;
        let fresh1 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        s = *fresh1;
        let fresh2 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        c1 = *fresh2;
        let fresh3 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        s1 = *fresh3;
        let fresh4 = in_ptr1;
        in_ptr1 = in_ptr1.offset(1);
        tempr = *fresh4;
        let fresh5 = in_ptr1;
        in_ptr1 = in_ptr1.offset(1);
        tempi = *fresh5;
        temp = -ixheaac_sub32_sat(
            ixheaac_mult32x16in32(tempr, s),
            ixheaac_mult32x16in32(tempi, c),
        );
        let fresh6 = xptr1;
        xptr1 = xptr1.offset(-1);
        *fresh6 = temp;
        temp = -ixheaac_add32_sat(
            ixheaac_mult32x16in32(tempr, c),
            ixheaac_mult32x16in32(tempi, s),
        );
        let fresh7 = xptr;
        xptr = xptr.offset(1);
        *fresh7 = temp;
        let fresh8 = in_ptr2;
        in_ptr2 = in_ptr2.offset(-1);
        tempi = *fresh8;
        let fresh9 = in_ptr2;
        in_ptr2 = in_ptr2.offset(-1);
        tempr = *fresh9;
        temp = -ixheaac_sub32_sat(
            ixheaac_mult32x16in32(tempr, s1),
            ixheaac_mult32x16in32(tempi, c1),
        );
        let fresh10 = xptr;
        xptr = xptr.offset(1);
        *fresh10 = temp;
        temp = -ixheaac_add32_sat(
            ixheaac_mult32x16in32(tempr, c1),
            ixheaac_mult32x16in32(tempi, s1),
        );
        let fresh11 = xptr1;
        xptr1 = xptr1.offset(-1);
        *fresh11 = temp;
        i += 4 as core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_post_twiddle_960(
    mut out: *mut WORD32,
    mut x: *mut WORD32,
    mut cos_sin_ptr: *const WORD32,
    mut m: WORD,
) -> VOID {
    let mut i: WORD = 0;
    let mut c: WORD32 = 0;
    let mut c1: WORD32 = 0;
    let mut s: WORD32 = 0;
    let mut s1: WORD32 = 0;
    let mut tempr: WORD32 = 0;
    let mut tempi: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut in_ptr2: *mut WORD32 = x
        .offset((m as core::ffi::c_int - 1 as core::ffi::c_int) as isize);
    let mut in_ptr1: *mut WORD32 = x as *mut WORD32;
    let mut xptr: *mut WORD32 = out as *mut WORD32;
    let mut xptr1: *mut WORD32 = out
        .offset((m as core::ffi::c_int - 1 as core::ffi::c_int) as isize);
    i = 0 as core::ffi::c_int as WORD;
    while i < m {
        let fresh217 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        c = *fresh217;
        let fresh218 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        s = *fresh218;
        let fresh219 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        c1 = *fresh219;
        let fresh220 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        s1 = *fresh220;
        let fresh221 = in_ptr1;
        in_ptr1 = in_ptr1.offset(1);
        tempr = *fresh221;
        let fresh222 = in_ptr1;
        in_ptr1 = in_ptr1.offset(1);
        tempi = *fresh222;
        temp = -ixheaac_sub32_sat(
            ixheaac_mult32x32in32(tempr, s),
            ixheaac_mult32x32in32(tempi, c),
        );
        let fresh223 = xptr1;
        xptr1 = xptr1.offset(-1);
        *fresh223 = temp;
        temp = -ixheaac_add32_sat(
            ixheaac_mult32x32in32(tempr, c),
            ixheaac_mult32x32in32(tempi, s),
        );
        let fresh224 = xptr;
        xptr = xptr.offset(1);
        *fresh224 = temp;
        let fresh225 = in_ptr2;
        in_ptr2 = in_ptr2.offset(-1);
        tempi = *fresh225;
        let fresh226 = in_ptr2;
        in_ptr2 = in_ptr2.offset(-1);
        tempr = *fresh226;
        temp = -ixheaac_sub32_sat(
            ixheaac_mult32x32in32(tempr, s1),
            ixheaac_mult32x32in32(tempi, c1),
        );
        let fresh227 = xptr;
        xptr = xptr.offset(1);
        *fresh227 = temp;
        temp = -ixheaac_add32_sat(
            ixheaac_mult32x32in32(tempr, c1),
            ixheaac_mult32x32in32(tempi, s1),
        );
        let fresh228 = xptr1;
        xptr1 = xptr1.offset(-1);
        *fresh228 = temp;
        i += 4 as core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_post_twiddle_ld(
    mut out: *mut WORD32,
    mut x: *mut WORD32,
    mut cos_sin_ptr: *const WORD32,
    mut m: WORD,
) -> VOID {
    let mut i: WORD = 0;
    let mut ptr_x: *mut WORD32 = &mut *x.offset(0 as core::ffi::c_int as isize)
        as *mut WORD32;
    let mut ptr_out: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr_out1: *mut WORD32 = 0 as *mut WORD32;
    ptr_out = &mut *out.offset(0 as core::ffi::c_int as isize) as *mut WORD32;
    ptr_out1 = &mut *out.offset((m as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
        as *mut WORD32;
    i = ((m as core::ffi::c_int >> 2 as core::ffi::c_int) - 1 as core::ffi::c_int)
        as WORD;
    while i >= 0 as core::ffi::c_int {
        let mut c: WORD32 = 0;
        let mut c1: WORD32 = 0;
        let mut s: WORD32 = 0;
        let mut s1: WORD32 = 0;
        let mut re: WORD32 = 0;
        let mut im: WORD32 = 0;
        let fresh209 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        c = *fresh209;
        let fresh210 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        c1 = *fresh210;
        let fresh211 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        s = *fresh211;
        let fresh212 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        s1 = *fresh212;
        let fresh213 = ptr_x;
        ptr_x = ptr_x.offset(1);
        re = *fresh213;
        let fresh214 = ptr_x;
        ptr_x = ptr_x.offset(1);
        im = *fresh214;
        *ptr_out1 = ixheaac_sub32(ixheaac_mult32(im, c), ixheaac_mult32(re, s));
        *ptr_out = -ixheaac_add32(ixheaac_mult32(re, c), ixheaac_mult32(im, s));
        ptr_out = ptr_out.offset(2 as core::ffi::c_int as isize);
        ptr_out1 = ptr_out1.offset(-(2 as core::ffi::c_int as isize));
        let fresh215 = ptr_x;
        ptr_x = ptr_x.offset(1);
        re = *fresh215;
        let fresh216 = ptr_x;
        ptr_x = ptr_x.offset(1);
        im = *fresh216;
        *ptr_out1 = ixheaac_sub32(ixheaac_mult32(im, c1), ixheaac_mult32(re, s1));
        *ptr_out = -ixheaac_add32(ixheaac_mult32(re, c1), ixheaac_mult32(im, s1));
        ptr_out = ptr_out.offset(2 as core::ffi::c_int as isize);
        ptr_out1 = ptr_out1.offset(-(2 as core::ffi::c_int as isize));
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_post_twiddle_eld(
    mut out: *mut WORD32,
    mut x: *mut WORD32,
    mut cos_sin_ptr: *const WORD32,
    mut m: WORD,
) -> VOID {
    let mut i: WORD = 0 as WORD;
    let mut ptr_x: *mut WORD32 = &mut *x.offset(0 as core::ffi::c_int as isize)
        as *mut WORD32;
    let mut ptr_out_767: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr_out_256: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr_out_768: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr_out_255: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr_out_0: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr_out_1279: *mut WORD32 = 0 as *mut WORD32;
    let mut tempr: WORD32 = 0;
    let mut tempi: WORD32 = 0;
    ptr_out_767 = &mut *out
        .offset((m + (m >> 1 as core::ffi::c_int) - 1 as WORD - 2 as WORD * i) as isize)
        as *mut WORD32;
    ptr_out_256 = &mut *out
        .offset(((m >> 1 as core::ffi::c_int) + 2 as WORD * i) as isize) as *mut WORD32;
    ptr_out_768 = &mut *out
        .offset((m + (m >> 1 as core::ffi::c_int) + 2 as WORD * i) as isize)
        as *mut WORD32;
    ptr_out_255 = &mut *out
        .offset(((m >> 1 as core::ffi::c_int) - 1 as WORD - 2 as WORD * i) as isize)
        as *mut WORD32;
    i = 0 as core::ffi::c_int as WORD;
    while i < m >> 3 as core::ffi::c_int {
        let mut c: WORD32 = 0;
        let mut c1: WORD32 = 0;
        let mut s: WORD32 = 0;
        let mut s1: WORD32 = 0;
        let mut re: WORD32 = 0;
        let mut im: WORD32 = 0;
        let fresh229 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        c = *fresh229;
        let fresh230 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        c1 = *fresh230;
        let fresh231 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        s = *fresh231;
        let fresh232 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        s1 = *fresh232;
        let fresh233 = ptr_x;
        ptr_x = ptr_x.offset(1);
        re = *fresh233;
        let fresh234 = ptr_x;
        ptr_x = ptr_x.offset(1);
        im = *fresh234;
        tempi = ixheaac_sub32(ixheaac_mult32(im, c), ixheaac_mult32(re, s));
        tempr = -ixheaac_add32(ixheaac_mult32(re, c), ixheaac_mult32(im, s));
        *ptr_out_767 = tempr;
        *ptr_out_256 = tempi;
        *ptr_out_768 = *ptr_out_767;
        *ptr_out_255 = -*ptr_out_256;
        ptr_out_256 = ptr_out_256.offset(2 as core::ffi::c_int as isize);
        ptr_out_767 = ptr_out_767.offset(-(2 as core::ffi::c_int as isize));
        ptr_out_768 = ptr_out_768.offset(2 as core::ffi::c_int as isize);
        ptr_out_255 = ptr_out_255.offset(-(2 as core::ffi::c_int as isize));
        let fresh235 = ptr_x;
        ptr_x = ptr_x.offset(1);
        re = *fresh235;
        let fresh236 = ptr_x;
        ptr_x = ptr_x.offset(1);
        im = *fresh236;
        tempi = ixheaac_sub32(ixheaac_mult32(im, c1), ixheaac_mult32(re, s1));
        tempr = -ixheaac_add32(ixheaac_mult32(re, c1), ixheaac_mult32(im, s1));
        *ptr_out_767 = tempr;
        *ptr_out_256 = tempi;
        *ptr_out_768 = *ptr_out_767;
        *ptr_out_255 = -*ptr_out_256;
        ptr_out_256 = ptr_out_256.offset(2 as core::ffi::c_int as isize);
        ptr_out_767 = ptr_out_767.offset(-(2 as core::ffi::c_int as isize));
        ptr_out_768 = ptr_out_768.offset(2 as core::ffi::c_int as isize);
        ptr_out_255 = ptr_out_255.offset(-(2 as core::ffi::c_int as isize));
        i += 1;
    }
    ptr_out_0 = &mut *out
        .offset((2 as WORD * 2 as WORD * i - (m >> 1 as core::ffi::c_int)) as isize)
        as *mut WORD32;
    ptr_out_1279 = &mut *out
        .offset(
            (m + m + (m >> 1 as core::ffi::c_int) - 1 as WORD
                - 2 as WORD * 2 as WORD * i) as isize,
        ) as *mut WORD32;
    while i < m >> 2 as core::ffi::c_int {
        let mut c_0: WORD32 = 0;
        let mut c1_0: WORD32 = 0;
        let mut s_0: WORD32 = 0;
        let mut s1_0: WORD32 = 0;
        let mut re_0: WORD32 = 0;
        let mut im_0: WORD32 = 0;
        let fresh237 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        c_0 = *fresh237;
        let fresh238 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        c1_0 = *fresh238;
        let fresh239 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        s_0 = *fresh239;
        let fresh240 = cos_sin_ptr;
        cos_sin_ptr = cos_sin_ptr.offset(1);
        s1_0 = *fresh240;
        let fresh241 = ptr_x;
        ptr_x = ptr_x.offset(1);
        re_0 = *fresh241;
        let fresh242 = ptr_x;
        ptr_x = ptr_x.offset(1);
        im_0 = *fresh242;
        tempi = ixheaac_sub32(ixheaac_mult32(im_0, c_0), ixheaac_mult32(re_0, s_0));
        tempr = -ixheaac_add32(ixheaac_mult32(re_0, c_0), ixheaac_mult32(im_0, s_0));
        *ptr_out_767 = tempr;
        *ptr_out_256 = tempi;
        *ptr_out_0 = -*ptr_out_767;
        *ptr_out_1279 = *ptr_out_256;
        ptr_out_256 = ptr_out_256.offset(2 as core::ffi::c_int as isize);
        ptr_out_767 = ptr_out_767.offset(-(2 as core::ffi::c_int as isize));
        ptr_out_0 = ptr_out_0.offset(2 as core::ffi::c_int as isize);
        ptr_out_1279 = ptr_out_1279.offset(-(2 as core::ffi::c_int as isize));
        let fresh243 = ptr_x;
        ptr_x = ptr_x.offset(1);
        re_0 = *fresh243;
        let fresh244 = ptr_x;
        ptr_x = ptr_x.offset(1);
        im_0 = *fresh244;
        tempi = ixheaac_sub32(ixheaac_mult32(im_0, c1_0), ixheaac_mult32(re_0, s1_0));
        tempr = -ixheaac_add32(ixheaac_mult32(re_0, c1_0), ixheaac_mult32(im_0, s1_0));
        *ptr_out_767 = tempr;
        *ptr_out_256 = tempi;
        *ptr_out_0 = -*ptr_out_767;
        *ptr_out_1279 = *ptr_out_256;
        ptr_out_256 = ptr_out_256.offset(2 as core::ffi::c_int as isize);
        ptr_out_767 = ptr_out_767.offset(-(2 as core::ffi::c_int as isize));
        ptr_out_0 = ptr_out_0.offset(2 as core::ffi::c_int as isize);
        ptr_out_1279 = ptr_out_1279.offset(-(2 as core::ffi::c_int as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_fft32x32_ld_dec(
    mut imdct_tables_ptr: *mut ia_aac_dec_imdct_tables_struct,
    mut npoints: WORD32,
    mut ptr_x: *mut WORD32,
    mut ptr_y: *mut WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut l1: WORD32 = 0;
    let mut l2: WORD32 = 0;
    let mut h2: WORD32 = 0;
    let mut predj: WORD32 = 0;
    let mut tw_offset: WORD32 = 0;
    let mut stride: WORD32 = 0;
    let mut fft_jmp: WORD32 = 0;
    let mut k: WORD32 = 0;
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
    let mut xh0_1: WORD32 = 0;
    let mut xh1_1: WORD32 = 0;
    let mut xl0_1: WORD32 = 0;
    let mut xl1_1: WORD32 = 0;
    let mut x_0: WORD32 = 0;
    let mut x_1: WORD32 = 0;
    let mut x_2: WORD32 = 0;
    let mut x_3: WORD32 = 0;
    let mut x_l1_0: WORD32 = 0;
    let mut x_l1_1: WORD32 = 0;
    let mut x_l2_0: WORD32 = 0;
    let mut x_l2_1: WORD32 = 0;
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
    let mut x_h2_0: WORD32 = 0;
    let mut x_h2_1: WORD32 = 0;
    let mut x_8: WORD32 = 0;
    let mut x_9: WORD32 = 0;
    let mut x_a: WORD32 = 0;
    let mut x_b: WORD32 = 0;
    let mut x_c: WORD32 = 0;
    let mut x_d: WORD32 = 0;
    let mut x_e: WORD32 = 0;
    let mut x_f: WORD32 = 0;
    let mut si10: WORD32 = 0;
    let mut si20: WORD32 = 0;
    let mut si30: WORD32 = 0;
    let mut co10: WORD32 = 0;
    let mut co20: WORD32 = 0;
    let mut co30: WORD32 = 0;
    let mut w: *mut WORD32 = 0 as *mut WORD32;
    let mut x: *mut WORD32 = 0 as *mut WORD32;
    let mut x2: *mut WORD32 = 0 as *mut WORD32;
    let mut x0: *mut WORD32 = 0 as *mut WORD32;
    let mut y0: *mut WORD32 = 0 as *mut WORD32;
    let mut y1: *mut WORD32 = 0 as *mut WORD32;
    let mut y2: *mut WORD32 = 0 as *mut WORD32;
    let mut y3: *mut WORD32 = 0 as *mut WORD32;
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
    let mut n0: WORD32 = 0;
    let mut j0: WORD32 = 0;
    let mut radix: WORD32 = 0;
    let mut norm: WORD32 = 0;
    let mut m: WORD32 = 0;
    let mut ptr_w: *mut WORD32 = 0 as *mut WORD32;
    if npoints == 256 as core::ffi::c_int {
        ptr_w = ((*imdct_tables_ptr).w_256).as_mut_ptr();
    } else if npoints == 32 as core::ffi::c_int {
        ptr_w = ((*imdct_tables_ptr).w_32).as_mut_ptr() as *mut WORD32;
    } else {
        ptr_w = ((*imdct_tables_ptr).w_16).as_mut_ptr();
    }
    i = 31 as core::ffi::c_int as WORD32;
    m = 1 as core::ffi::c_int as WORD32;
    while npoints as core::ffi::c_int & (1 as core::ffi::c_int) << i
        == 0 as core::ffi::c_int
    {
        i -= 1;
        m += 1;
    }
    radix = (if m as core::ffi::c_int & 1 as core::ffi::c_int != 0 {
        2 as core::ffi::c_int
    } else {
        4 as core::ffi::c_int
    }) as WORD32;
    norm = (m as core::ffi::c_int - 2 as core::ffi::c_int) as WORD32;
    stride = npoints;
    tw_offset = 0 as core::ffi::c_int as WORD32;
    fft_jmp = 6 as WORD32 * stride;
    while stride > radix {
        j = 0 as core::ffi::c_int as WORD32;
        fft_jmp >>= 2 as core::ffi::c_int;
        h2 = stride >> 1 as core::ffi::c_int;
        l1 = stride;
        l2 = stride + (stride >> 1 as core::ffi::c_int);
        x = ptr_x;
        w = ptr_w.offset(tw_offset as isize);
        tw_offset += fft_jmp;
        stride >>= 2 as core::ffi::c_int;
        i = 0 as core::ffi::c_int as WORD32;
        while i < npoints {
            co10 = *w.offset((j as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
            si10 = *w.offset((j as core::ffi::c_int + 0 as core::ffi::c_int) as isize);
            co20 = *w.offset((j as core::ffi::c_int + 3 as core::ffi::c_int) as isize);
            si20 = *w.offset((j as core::ffi::c_int + 2 as core::ffi::c_int) as isize);
            co30 = *w.offset((j as core::ffi::c_int + 5 as core::ffi::c_int) as isize);
            si30 = *w.offset((j as core::ffi::c_int + 4 as core::ffi::c_int) as isize);
            x_0 = *x.offset(0 as core::ffi::c_int as isize);
            x_1 = *x.offset(1 as core::ffi::c_int as isize);
            x_l1_0 = *x.offset(l1 as isize);
            x_l1_1 = *x
                .offset((l1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
            x_l2_0 = *x.offset(l2 as isize);
            x_l2_1 = *x
                .offset((l2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
            x_h2_0 = *x.offset(h2 as isize);
            x_h2_1 = *x
                .offset((h2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
            xh0_0 = ixheaac_add32_sat(x_0, x_l1_0);
            xh1_0 = ixheaac_add32_sat(x_1, x_l1_1);
            xl0_0 = ixheaac_sub32_sat(x_0, x_l1_0);
            xl1_0 = ixheaac_sub32_sat(x_1, x_l1_1);
            xh20_0 = ixheaac_add32_sat(x_h2_0, x_l2_0);
            xh21_0 = ixheaac_add32_sat(x_h2_1, x_l2_1);
            xl20_0 = ixheaac_sub32_sat(x_h2_0, x_l2_0);
            xl21_0 = ixheaac_sub32_sat(x_h2_1, x_l2_1);
            x0 = x;
            x2 = x0;
            j += 6 as core::ffi::c_int;
            x = x.offset(2 as core::ffi::c_int as isize);
            predj = j - fft_jmp;
            if predj == 0 {
                x = x.offset(fft_jmp as isize);
            }
            if predj == 0 {
                j = 0 as core::ffi::c_int as WORD32;
            }
            *x0.offset(0 as core::ffi::c_int as isize) = ixheaac_add32_sat(
                xh0_0,
                xh20_0,
            );
            *x0.offset(1 as core::ffi::c_int as isize) = ixheaac_add32_sat(
                xh1_0,
                xh21_0,
            );
            xt0_0 = ixheaac_sub32_sat(xh0_0, xh20_0);
            yt0_0 = ixheaac_sub32_sat(xh1_0, xh21_0);
            xt1_0 = ixheaac_add32_sat(xl0_0, xl21_0);
            yt2_0 = ixheaac_add32_sat(xl1_0, xl20_0);
            xt2_0 = ixheaac_sub32_sat(xl0_0, xl21_0);
            yt1_0 = ixheaac_sub32_sat(xl1_0, xl20_0);
            *x2.offset(h2 as isize) = (((si10 >> 16 as core::ffi::c_int) as WORD16
                as core::ffi::c_int
                * (yt1_0 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                    as core::ffi::c_int + 0x4000 as core::ffi::c_int
                >> 15 as core::ffi::c_int)
                + (((si10 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                    * (yt1_0 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int)
                    << 1 as core::ffi::c_int)
                + (((co10 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                    * (xt1_0 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                        as core::ffi::c_int + 0x4000 as core::ffi::c_int
                    >> 15 as core::ffi::c_int)
                    + (((co10 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                        * (xt1_0 >> 16 as core::ffi::c_int) as WORD16
                            as core::ffi::c_int) << 1 as core::ffi::c_int))
                + (((si10 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                    as core::ffi::c_int
                    * (yt1_0 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                    + (co10 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                        as core::ffi::c_int
                        * (xt1_0 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                    + 0x8000 as core::ffi::c_int >> 16 as core::ffi::c_int)
                    << 1 as core::ffi::c_int)) as WORD32;
            *x2.offset((h2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = (((co10
                >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                * (yt1_0 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                    as core::ffi::c_int + 0x4000 as core::ffi::c_int
                >> 15 as core::ffi::c_int)
                + (((co10 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                    * (yt1_0 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int)
                    << 1 as core::ffi::c_int)
                - (((si10 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                    * (xt1_0 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                        as core::ffi::c_int + 0x4000 as core::ffi::c_int
                    >> 15 as core::ffi::c_int)
                    + (((si10 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                        * (xt1_0 >> 16 as core::ffi::c_int) as WORD16
                            as core::ffi::c_int) << 1 as core::ffi::c_int))
                + (((co10 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                    as core::ffi::c_int
                    * (yt1_0 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                    - (si10 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                        as core::ffi::c_int
                        * (xt1_0 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                    + 0x8000 as core::ffi::c_int >> 16 as core::ffi::c_int)
                    << 1 as core::ffi::c_int)) as WORD32;
            *x2.offset(l1 as isize) = (((si20 >> 16 as core::ffi::c_int) as WORD16
                as core::ffi::c_int
                * (yt0_0 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                    as core::ffi::c_int + 0x4000 as core::ffi::c_int
                >> 15 as core::ffi::c_int)
                + (((si20 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                    * (yt0_0 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int)
                    << 1 as core::ffi::c_int)
                + (((co20 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                    * (xt0_0 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                        as core::ffi::c_int + 0x4000 as core::ffi::c_int
                    >> 15 as core::ffi::c_int)
                    + (((co20 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                        * (xt0_0 >> 16 as core::ffi::c_int) as WORD16
                            as core::ffi::c_int) << 1 as core::ffi::c_int))
                + (((si20 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                    as core::ffi::c_int
                    * (yt0_0 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                    + (co20 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                        as core::ffi::c_int
                        * (xt0_0 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                    + 0x8000 as core::ffi::c_int >> 16 as core::ffi::c_int)
                    << 1 as core::ffi::c_int)) as WORD32;
            *x2.offset((l1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = (((co20
                >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                * (yt0_0 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                    as core::ffi::c_int + 0x4000 as core::ffi::c_int
                >> 15 as core::ffi::c_int)
                + (((co20 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                    * (yt0_0 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int)
                    << 1 as core::ffi::c_int)
                - (((si20 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                    * (xt0_0 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                        as core::ffi::c_int + 0x4000 as core::ffi::c_int
                    >> 15 as core::ffi::c_int)
                    + (((si20 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                        * (xt0_0 >> 16 as core::ffi::c_int) as WORD16
                            as core::ffi::c_int) << 1 as core::ffi::c_int))
                + (((co20 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                    as core::ffi::c_int
                    * (yt0_0 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                    - (si20 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                        as core::ffi::c_int
                        * (xt0_0 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                    + 0x8000 as core::ffi::c_int >> 16 as core::ffi::c_int)
                    << 1 as core::ffi::c_int)) as WORD32;
            *x2.offset(l2 as isize) = (((si30 >> 16 as core::ffi::c_int) as WORD16
                as core::ffi::c_int
                * (yt2_0 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                    as core::ffi::c_int + 0x4000 as core::ffi::c_int
                >> 15 as core::ffi::c_int)
                + (((si30 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                    * (yt2_0 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int)
                    << 1 as core::ffi::c_int)
                + (((co30 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                    * (xt2_0 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                        as core::ffi::c_int + 0x4000 as core::ffi::c_int
                    >> 15 as core::ffi::c_int)
                    + (((co30 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                        * (xt2_0 >> 16 as core::ffi::c_int) as WORD16
                            as core::ffi::c_int) << 1 as core::ffi::c_int))
                + (((si30 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                    as core::ffi::c_int
                    * (yt2_0 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                    + (co30 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                        as core::ffi::c_int
                        * (xt2_0 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                    + 0x8000 as core::ffi::c_int >> 16 as core::ffi::c_int)
                    << 1 as core::ffi::c_int)) as WORD32;
            *x2.offset((l2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = (((co30
                >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                * (yt2_0 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                    as core::ffi::c_int + 0x4000 as core::ffi::c_int
                >> 15 as core::ffi::c_int)
                + (((co30 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                    * (yt2_0 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int)
                    << 1 as core::ffi::c_int)
                - (((si30 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                    * (xt2_0 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                        as core::ffi::c_int + 0x4000 as core::ffi::c_int
                    >> 15 as core::ffi::c_int)
                    + (((si30 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                        * (xt2_0 >> 16 as core::ffi::c_int) as WORD16
                            as core::ffi::c_int) << 1 as core::ffi::c_int))
                + (((co30 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                    as core::ffi::c_int
                    * (yt2_0 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                    - (si30 as core::ffi::c_int & 0xffff as core::ffi::c_int) as UWORD16
                        as core::ffi::c_int
                        * (xt2_0 >> 16 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                    + 0x8000 as core::ffi::c_int >> 16 as core::ffi::c_int)
                    << 1 as core::ffi::c_int)) as WORD32;
            i += 4 as core::ffi::c_int;
        }
    }
    y0 = ptr_y;
    y2 = ptr_y.offset(npoints as isize);
    x0 = ptr_x;
    x2 = ptr_x.offset((npoints >> 1 as core::ffi::c_int) as isize);
    if radix == 2 as core::ffi::c_int {
        y1 = y0.offset((npoints >> 2 as core::ffi::c_int) as isize);
        y3 = y2.offset((npoints >> 2 as core::ffi::c_int) as isize);
        l1 = (norm as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
        j0 = 8 as core::ffi::c_int as WORD32;
        n0 = npoints >> 1 as core::ffi::c_int;
    } else {
        y1 = y0.offset((npoints >> 1 as core::ffi::c_int) as isize);
        y3 = y2.offset((npoints >> 1 as core::ffi::c_int) as isize);
        l1 = (norm as core::ffi::c_int + 2 as core::ffi::c_int) as WORD32;
        j0 = 4 as core::ffi::c_int as WORD32;
        n0 = npoints >> 2 as core::ffi::c_int;
    }
    j = 0 as core::ffi::c_int as WORD32;
    k = 0 as core::ffi::c_int as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < npoints {
        if npoints == 32 as core::ffi::c_int {
            let fresh245 = k;
            k = k + 1;
            h2 = rev_dig[fresh245 as usize];
        } else {
            let mut val: core::ffi::c_uint = j as core::ffi::c_uint;
            val = (val & 0x33333333 as core::ffi::c_uint) << 2 as core::ffi::c_int
                | (val & !(0x33333333 as core::ffi::c_int) as core::ffi::c_uint)
                    >> 2 as core::ffi::c_int;
            val = (val & 0xf0f0f0f as core::ffi::c_uint) << 4 as core::ffi::c_int
                | (val & !(0xf0f0f0f as core::ffi::c_int) as core::ffi::c_uint)
                    >> 4 as core::ffi::c_int;
            val = (val & 0xff00ff as core::ffi::c_uint) << 8 as core::ffi::c_int
                | (val & !(0xff00ff as core::ffi::c_int) as core::ffi::c_uint)
                    >> 8 as core::ffi::c_int;
            val = (val & 0xffff as core::ffi::c_uint) << 16 as core::ffi::c_int
                | (val & !(0xffff as core::ffi::c_int) as core::ffi::c_uint)
                    >> 16 as core::ffi::c_int;
            h2 = (val >> l1) as WORD32;
        }
        x_0 = *x0.offset(0 as core::ffi::c_int as isize);
        x_1 = *x0.offset(1 as core::ffi::c_int as isize);
        x_2 = *x0.offset(2 as core::ffi::c_int as isize);
        x_3 = *x0.offset(3 as core::ffi::c_int as isize);
        x_4 = *x0.offset(4 as core::ffi::c_int as isize);
        x_5 = *x0.offset(5 as core::ffi::c_int as isize);
        x_6 = *x0.offset(6 as core::ffi::c_int as isize);
        x_7 = *x0.offset(7 as core::ffi::c_int as isize);
        x0 = x0.offset(8 as core::ffi::c_int as isize);
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
        if radix == 2 as core::ffi::c_int {
            n00 = ixheaac_add32_sat(x_0, x_2);
            n01 = ixheaac_add32_sat(x_1, x_3);
            n20 = ixheaac_sub32_sat(x_0, x_2);
            n21 = ixheaac_sub32_sat(x_1, x_3);
            n10 = ixheaac_add32_sat(x_4, x_6);
            n11 = ixheaac_add32_sat(x_5, x_7);
            n30 = ixheaac_sub32_sat(x_4, x_6);
            n31 = ixheaac_sub32_sat(x_5, x_7);
        }
        *y0.offset((2 as WORD32 * h2) as isize) = n00;
        *y0
            .offset(
                (2 as core::ffi::c_int * h2 as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            ) = n01;
        *y1.offset((2 as WORD32 * h2) as isize) = n10;
        *y1
            .offset(
                (2 as core::ffi::c_int * h2 as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            ) = n11;
        *y2.offset((2 as WORD32 * h2) as isize) = n20;
        *y2
            .offset(
                (2 as core::ffi::c_int * h2 as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            ) = n21;
        *y3.offset((2 as WORD32 * h2) as isize) = n30;
        *y3
            .offset(
                (2 as core::ffi::c_int * h2 as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            ) = n31;
        x_8 = *x2.offset(0 as core::ffi::c_int as isize);
        x_9 = *x2.offset(1 as core::ffi::c_int as isize);
        x_a = *x2.offset(2 as core::ffi::c_int as isize);
        x_b = *x2.offset(3 as core::ffi::c_int as isize);
        x_c = *x2.offset(4 as core::ffi::c_int as isize);
        x_d = *x2.offset(5 as core::ffi::c_int as isize);
        x_e = *x2.offset(6 as core::ffi::c_int as isize);
        x_f = *x2.offset(7 as core::ffi::c_int as isize);
        x2 = x2.offset(8 as core::ffi::c_int as isize);
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
        if radix == 2 as core::ffi::c_int {
            n02 = ixheaac_add32_sat(x_8, x_a);
            n03 = ixheaac_add32_sat(x_9, x_b);
            n22 = ixheaac_sub32_sat(x_8, x_a);
            n23 = ixheaac_sub32_sat(x_9, x_b);
            n12 = ixheaac_add32_sat(x_c, x_e);
            n13 = ixheaac_add32_sat(x_d, x_f);
            n32 = ixheaac_sub32_sat(x_c, x_e);
            n33 = ixheaac_sub32_sat(x_d, x_f);
        }
        *y0
            .offset(
                (2 as core::ffi::c_int * h2 as core::ffi::c_int + 2 as core::ffi::c_int)
                    as isize,
            ) = n02;
        *y0
            .offset(
                (2 as core::ffi::c_int * h2 as core::ffi::c_int + 3 as core::ffi::c_int)
                    as isize,
            ) = n03;
        *y1
            .offset(
                (2 as core::ffi::c_int * h2 as core::ffi::c_int + 2 as core::ffi::c_int)
                    as isize,
            ) = n12;
        *y1
            .offset(
                (2 as core::ffi::c_int * h2 as core::ffi::c_int + 3 as core::ffi::c_int)
                    as isize,
            ) = n13;
        *y2
            .offset(
                (2 as core::ffi::c_int * h2 as core::ffi::c_int + 2 as core::ffi::c_int)
                    as isize,
            ) = n22;
        *y2
            .offset(
                (2 as core::ffi::c_int * h2 as core::ffi::c_int + 3 as core::ffi::c_int)
                    as isize,
            ) = n23;
        *y3
            .offset(
                (2 as core::ffi::c_int * h2 as core::ffi::c_int + 2 as core::ffi::c_int)
                    as isize,
            ) = n32;
        *y3
            .offset(
                (2 as core::ffi::c_int * h2 as core::ffi::c_int + 3 as core::ffi::c_int)
                    as isize,
            ) = n33;
        j += j0;
        if j == n0 {
            j += n0;
            x0 = x0.offset((npoints >> 1 as core::ffi::c_int) as isize);
            x2 = x2.offset((npoints >> 1 as core::ffi::c_int) as isize);
        }
        i += 8 as core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_rearrange_dec(
    mut ip: *mut WORD32,
    mut op: *mut WORD32,
    mut mdct_len_2: WORD32,
    mut re_arr_tab: *mut UWORD8,
) -> VOID {
    let mut n: WORD32 = 0;
    let mut i: WORD32 = 0 as WORD32;
    n = 0 as core::ffi::c_int as WORD32;
    while n < mdct_len_2 {
        let mut idx: WORD32 = (*re_arr_tab.offset(n as isize) as WORD32)
            << 1 as core::ffi::c_int;
        let fresh246 = i;
        i = i + 1;
        *op.offset(fresh246 as isize) = *ip.offset(idx as isize);
        let fresh247 = i;
        i = i + 1;
        *op.offset(fresh247 as isize) = *ip
            .offset((idx as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_fft_15_ld_dec(
    mut inp: *mut WORD32,
    mut op: *mut WORD32,
    mut fft3out: *mut WORD32,
    mut re_arr_tab_sml_240_ptr: *mut UWORD8,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut idx: WORD32 = 0;
    let mut buf1: *mut WORD32 = 0 as *mut WORD32;
    let mut buf2: *mut WORD32 = 0 as *mut WORD32;
    let mut buf1a: *mut WORD32 = 0 as *mut WORD32;
    let mut add_r: WORD32 = 0;
    let mut sub_r: WORD32 = 0;
    let mut add_i: WORD32 = 0;
    let mut sub_i: WORD32 = 0;
    let mut x01_real: WORD32 = 0;
    let mut x_01_imag: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut p1: WORD32 = 0;
    let mut p2: WORD32 = 0;
    let mut p3: WORD32 = 0;
    let mut p4: WORD32 = 0;
    let mut sinmu: WORD32 = 1859775393 as WORD32;
    let mut cos_51: WORD32 = 2042378317 as WORD32;
    let mut cos_52: WORD32 = -(1652318768 as WORD32);
    let mut cos_53: WORD32 = -(780119100 as WORD32);
    let mut cos_54: WORD32 = 1200479854 as WORD32;
    let mut cos_55: WORD32 = -(1342177280 as WORD32);
    let mut r1: WORD32 = 0;
    let mut r2: WORD32 = 0;
    let mut r3: WORD32 = 0;
    let mut r4: WORD32 = 0;
    let mut s1: WORD32 = 0;
    let mut s2: WORD32 = 0;
    let mut s3: WORD32 = 0;
    let mut s4: WORD32 = 0;
    let mut t: WORD32 = 0;
    let mut temp1: WORD32 = 0;
    let mut temp2: WORD32 = 0;
    let mut fft3outptr: *mut WORD32 = fft3out;
    let mut xr_0: WORD32 = 0;
    let mut xr_1: WORD32 = 0;
    let mut xr_2: WORD32 = 0;
    let mut xi_0: WORD32 = 0;
    let mut xi_1: WORD32 = 0;
    let mut xi_2: WORD32 = 0;
    buf2 = fft3out;
    buf1a = fft3out;
    buf1 = buf1a;
    n = 0 as core::ffi::c_int as WORD32;
    let fresh248 = buf1;
    buf1 = buf1.offset(1);
    *fresh248 = *inp.offset(0 as core::ffi::c_int as isize);
    let fresh249 = buf1;
    buf1 = buf1.offset(1);
    *fresh249 = *inp.offset(1 as core::ffi::c_int as isize);
    let fresh250 = buf1;
    buf1 = buf1.offset(1);
    *fresh250 = *inp.offset(96 as core::ffi::c_int as isize);
    let fresh251 = buf1;
    buf1 = buf1.offset(1);
    *fresh251 = *inp.offset(97 as core::ffi::c_int as isize);
    let fresh252 = buf1;
    buf1 = buf1.offset(1);
    *fresh252 = *inp.offset(192 as core::ffi::c_int as isize);
    let fresh253 = buf1;
    buf1 = buf1.offset(1);
    *fresh253 = *inp.offset(193 as core::ffi::c_int as isize);
    let fresh254 = buf1;
    buf1 = buf1.offset(1);
    *fresh254 = *inp.offset(288 as core::ffi::c_int as isize);
    let fresh255 = buf1;
    buf1 = buf1.offset(1);
    *fresh255 = *inp.offset(289 as core::ffi::c_int as isize);
    let fresh256 = buf1;
    buf1 = buf1.offset(1);
    *fresh256 = *inp.offset(384 as core::ffi::c_int as isize);
    let fresh257 = buf1;
    buf1 = buf1.offset(1);
    *fresh257 = *inp.offset(385 as core::ffi::c_int as isize);
    r1 = ixheaac_add32_sat(
        *buf1a.offset(2 as core::ffi::c_int as isize),
        *buf1a.offset(8 as core::ffi::c_int as isize),
    );
    r4 = ixheaac_sub32_sat(
        *buf1a.offset(2 as core::ffi::c_int as isize),
        *buf1a.offset(8 as core::ffi::c_int as isize),
    );
    r3 = ixheaac_add32_sat(
        *buf1a.offset(4 as core::ffi::c_int as isize),
        *buf1a.offset(6 as core::ffi::c_int as isize),
    );
    r2 = ixheaac_sub32_sat(
        *buf1a.offset(4 as core::ffi::c_int as isize),
        *buf1a.offset(6 as core::ffi::c_int as isize),
    );
    t = ixheaac_mult32_shl(ixheaac_sub32_sat(r1, r3), cos_54);
    r1 = ixheaac_add32_sat(r1, r3);
    temp1 = ixheaac_add32_sat(*buf1a.offset(0 as core::ffi::c_int as isize), r1);
    r1 = ixheaac_add32_sat(
        temp1,
        ixheaac_shl32_sat(ixheaac_mult32_shl(r1, cos_55), 1 as WORD),
    );
    r3 = ixheaac_sub32_sat(r1, t);
    r1 = ixheaac_add32_sat(r1, t);
    t = ixheaac_mult32_shl(ixheaac_add32_sat(r4, r2), cos_51);
    r4 = ixheaac_add32_sat(
        t,
        ixheaac_shl32_sat(ixheaac_mult32_shl(r4, cos_52), 1 as WORD),
    );
    r2 = ixheaac_add32_sat(t, ixheaac_mult32_shl(r2, cos_53));
    s1 = ixheaac_add32_sat(
        *buf1a.offset(3 as core::ffi::c_int as isize),
        *buf1a.offset(9 as core::ffi::c_int as isize),
    );
    s4 = ixheaac_sub32_sat(
        *buf1a.offset(3 as core::ffi::c_int as isize),
        *buf1a.offset(9 as core::ffi::c_int as isize),
    );
    s3 = ixheaac_add32_sat(
        *buf1a.offset(5 as core::ffi::c_int as isize),
        *buf1a.offset(7 as core::ffi::c_int as isize),
    );
    s2 = ixheaac_sub32_sat(
        *buf1a.offset(5 as core::ffi::c_int as isize),
        *buf1a.offset(7 as core::ffi::c_int as isize),
    );
    t = ixheaac_mult32_shl(ixheaac_sub32_sat(s1, s3), cos_54);
    s1 = ixheaac_add32_sat(s1, s3);
    temp2 = ixheaac_add32_sat(*buf1a.offset(1 as core::ffi::c_int as isize), s1);
    s1 = ixheaac_add32_sat(
        temp2,
        ixheaac_shl32_sat(ixheaac_mult32_shl(s1, cos_55), 1 as WORD),
    );
    s3 = ixheaac_sub32_sat(s1, t);
    s1 = ixheaac_add32_sat(s1, t);
    t = ixheaac_mult32_shl(ixheaac_add32_sat(s4, s2), cos_51);
    s4 = ixheaac_add32_sat(
        t,
        ixheaac_shl32_sat(ixheaac_mult32_shl(s4, cos_52), 1 as WORD),
    );
    s2 = ixheaac_add32_sat(t, ixheaac_mult32_shl(s2, cos_53));
    let fresh258 = buf2;
    buf2 = buf2.offset(1);
    *fresh258 = temp1;
    let fresh259 = buf2;
    buf2 = buf2.offset(1);
    *fresh259 = temp2;
    let fresh260 = buf2;
    buf2 = buf2.offset(1);
    *fresh260 = ixheaac_add32_sat(r1, s2);
    let fresh261 = buf2;
    buf2 = buf2.offset(1);
    *fresh261 = ixheaac_sub32_sat(s1, r2);
    let fresh262 = buf2;
    buf2 = buf2.offset(1);
    *fresh262 = ixheaac_sub32_sat(r3, s4);
    let fresh263 = buf2;
    buf2 = buf2.offset(1);
    *fresh263 = ixheaac_add32_sat(s3, r4);
    let fresh264 = buf2;
    buf2 = buf2.offset(1);
    *fresh264 = ixheaac_add32_sat(r3, s4);
    let fresh265 = buf2;
    buf2 = buf2.offset(1);
    *fresh265 = ixheaac_sub32_sat(s3, r4);
    let fresh266 = buf2;
    buf2 = buf2.offset(1);
    *fresh266 = ixheaac_sub32_sat(r1, s2);
    let fresh267 = buf2;
    buf2 = buf2.offset(1);
    *fresh267 = ixheaac_add32_sat(s1, r2);
    buf1a = buf1;
    let fresh268 = buf1;
    buf1 = buf1.offset(1);
    *fresh268 = *inp.offset(160 as core::ffi::c_int as isize);
    let fresh269 = buf1;
    buf1 = buf1.offset(1);
    *fresh269 = *inp.offset(161 as core::ffi::c_int as isize);
    let fresh270 = buf1;
    buf1 = buf1.offset(1);
    *fresh270 = *inp.offset(256 as core::ffi::c_int as isize);
    let fresh271 = buf1;
    buf1 = buf1.offset(1);
    *fresh271 = *inp.offset(257 as core::ffi::c_int as isize);
    let fresh272 = buf1;
    buf1 = buf1.offset(1);
    *fresh272 = *inp.offset(352 as core::ffi::c_int as isize);
    let fresh273 = buf1;
    buf1 = buf1.offset(1);
    *fresh273 = *inp.offset(353 as core::ffi::c_int as isize);
    let fresh274 = buf1;
    buf1 = buf1.offset(1);
    *fresh274 = *inp.offset(448 as core::ffi::c_int as isize);
    let fresh275 = buf1;
    buf1 = buf1.offset(1);
    *fresh275 = *inp.offset(449 as core::ffi::c_int as isize);
    let fresh276 = buf1;
    buf1 = buf1.offset(1);
    *fresh276 = *inp.offset(64 as core::ffi::c_int as isize);
    let fresh277 = buf1;
    buf1 = buf1.offset(1);
    *fresh277 = *inp.offset(65 as core::ffi::c_int as isize);
    r1 = ixheaac_add32_sat(
        *buf1a.offset(2 as core::ffi::c_int as isize),
        *buf1a.offset(8 as core::ffi::c_int as isize),
    );
    r4 = ixheaac_sub32_sat(
        *buf1a.offset(2 as core::ffi::c_int as isize),
        *buf1a.offset(8 as core::ffi::c_int as isize),
    );
    r3 = ixheaac_add32_sat(
        *buf1a.offset(4 as core::ffi::c_int as isize),
        *buf1a.offset(6 as core::ffi::c_int as isize),
    );
    r2 = ixheaac_sub32_sat(
        *buf1a.offset(4 as core::ffi::c_int as isize),
        *buf1a.offset(6 as core::ffi::c_int as isize),
    );
    t = ixheaac_mult32_shl(ixheaac_sub32_sat(r1, r3), cos_54);
    r1 = ixheaac_add32_sat(r1, r3);
    temp1 = ixheaac_add32_sat(*buf1a.offset(0 as core::ffi::c_int as isize), r1);
    r1 = ixheaac_add32_sat(
        temp1,
        ixheaac_shl32_sat(ixheaac_mult32_shl(r1, cos_55), 1 as WORD),
    );
    r3 = ixheaac_sub32_sat(r1, t);
    r1 = ixheaac_add32_sat(r1, t);
    t = ixheaac_mult32_shl(ixheaac_add32_sat(r4, r2), cos_51);
    r4 = ixheaac_add32_sat(
        t,
        ixheaac_shl32_sat(ixheaac_mult32_shl(r4, cos_52), 1 as WORD),
    );
    r2 = ixheaac_add32_sat(t, ixheaac_mult32_shl(r2, cos_53));
    s1 = ixheaac_add32_sat(
        *buf1a.offset(3 as core::ffi::c_int as isize),
        *buf1a.offset(9 as core::ffi::c_int as isize),
    );
    s4 = ixheaac_sub32_sat(
        *buf1a.offset(3 as core::ffi::c_int as isize),
        *buf1a.offset(9 as core::ffi::c_int as isize),
    );
    s3 = ixheaac_add32_sat(
        *buf1a.offset(5 as core::ffi::c_int as isize),
        *buf1a.offset(7 as core::ffi::c_int as isize),
    );
    s2 = ixheaac_sub32_sat(
        *buf1a.offset(5 as core::ffi::c_int as isize),
        *buf1a.offset(7 as core::ffi::c_int as isize),
    );
    t = ixheaac_mult32_shl(ixheaac_sub32_sat(s1, s3), cos_54);
    s1 = ixheaac_add32_sat(s1, s3);
    temp2 = ixheaac_add32_sat(*buf1a.offset(1 as core::ffi::c_int as isize), s1);
    s1 = ixheaac_add32_sat(
        temp2,
        ixheaac_shl32_sat(ixheaac_mult32_shl(s1, cos_55), 1 as WORD),
    );
    s3 = ixheaac_sub32_sat(s1, t);
    s1 = ixheaac_add32_sat(s1, t);
    t = ixheaac_mult32_shl(ixheaac_add32_sat(s4, s2), cos_51);
    s4 = ixheaac_add32_sat(
        t,
        ixheaac_shl32_sat(ixheaac_mult32_shl(s4, cos_52), 1 as WORD),
    );
    s2 = ixheaac_add32_sat(t, ixheaac_mult32_shl(s2, cos_53));
    let fresh278 = buf2;
    buf2 = buf2.offset(1);
    *fresh278 = temp1;
    let fresh279 = buf2;
    buf2 = buf2.offset(1);
    *fresh279 = temp2;
    let fresh280 = buf2;
    buf2 = buf2.offset(1);
    *fresh280 = ixheaac_add32_sat(r1, s2);
    let fresh281 = buf2;
    buf2 = buf2.offset(1);
    *fresh281 = ixheaac_sub32_sat(s1, r2);
    let fresh282 = buf2;
    buf2 = buf2.offset(1);
    *fresh282 = ixheaac_sub32_sat(r3, s4);
    let fresh283 = buf2;
    buf2 = buf2.offset(1);
    *fresh283 = ixheaac_add32_sat(s3, r4);
    let fresh284 = buf2;
    buf2 = buf2.offset(1);
    *fresh284 = ixheaac_add32_sat(r3, s4);
    let fresh285 = buf2;
    buf2 = buf2.offset(1);
    *fresh285 = ixheaac_sub32_sat(s3, r4);
    let fresh286 = buf2;
    buf2 = buf2.offset(1);
    *fresh286 = ixheaac_sub32_sat(r1, s2);
    let fresh287 = buf2;
    buf2 = buf2.offset(1);
    *fresh287 = ixheaac_add32_sat(s1, r2);
    buf1a = buf1;
    let fresh288 = buf1;
    buf1 = buf1.offset(1);
    *fresh288 = *inp.offset(320 as core::ffi::c_int as isize);
    let fresh289 = buf1;
    buf1 = buf1.offset(1);
    *fresh289 = *inp.offset(321 as core::ffi::c_int as isize);
    let fresh290 = buf1;
    buf1 = buf1.offset(1);
    *fresh290 = *inp.offset(416 as core::ffi::c_int as isize);
    let fresh291 = buf1;
    buf1 = buf1.offset(1);
    *fresh291 = *inp.offset(417 as core::ffi::c_int as isize);
    let fresh292 = buf1;
    buf1 = buf1.offset(1);
    *fresh292 = *inp.offset(32 as core::ffi::c_int as isize);
    let fresh293 = buf1;
    buf1 = buf1.offset(1);
    *fresh293 = *inp.offset(33 as core::ffi::c_int as isize);
    let fresh294 = buf1;
    buf1 = buf1.offset(1);
    *fresh294 = *inp.offset(128 as core::ffi::c_int as isize);
    let fresh295 = buf1;
    buf1 = buf1.offset(1);
    *fresh295 = *inp.offset(129 as core::ffi::c_int as isize);
    let fresh296 = buf1;
    buf1 = buf1.offset(1);
    *fresh296 = *inp.offset(224 as core::ffi::c_int as isize);
    let fresh297 = buf1;
    buf1 = buf1.offset(1);
    *fresh297 = *inp.offset(225 as core::ffi::c_int as isize);
    r1 = ixheaac_add32_sat(
        *buf1a.offset(2 as core::ffi::c_int as isize),
        *buf1a.offset(8 as core::ffi::c_int as isize),
    );
    r4 = ixheaac_sub32_sat(
        *buf1a.offset(2 as core::ffi::c_int as isize),
        *buf1a.offset(8 as core::ffi::c_int as isize),
    );
    r3 = ixheaac_add32_sat(
        *buf1a.offset(4 as core::ffi::c_int as isize),
        *buf1a.offset(6 as core::ffi::c_int as isize),
    );
    r2 = ixheaac_sub32_sat(
        *buf1a.offset(4 as core::ffi::c_int as isize),
        *buf1a.offset(6 as core::ffi::c_int as isize),
    );
    t = ixheaac_mult32_shl(ixheaac_sub32_sat(r1, r3), cos_54);
    r1 = ixheaac_add32_sat(r1, r3);
    temp1 = ixheaac_add32_sat(*buf1a.offset(0 as core::ffi::c_int as isize), r1);
    r1 = ixheaac_add32_sat(
        temp1,
        ixheaac_shl32_sat(ixheaac_mult32_shl(r1, cos_55), 1 as WORD),
    );
    r3 = ixheaac_sub32_sat(r1, t);
    r1 = ixheaac_add32_sat(r1, t);
    t = ixheaac_mult32_shl(ixheaac_add32_sat(r4, r2), cos_51);
    r4 = ixheaac_add32_sat(
        t,
        ixheaac_shl32_sat(ixheaac_mult32_shl(r4, cos_52), 1 as WORD),
    );
    r2 = ixheaac_add32_sat(t, ixheaac_mult32_shl(r2, cos_53));
    s1 = ixheaac_add32_sat(
        *buf1a.offset(3 as core::ffi::c_int as isize),
        *buf1a.offset(9 as core::ffi::c_int as isize),
    );
    s4 = ixheaac_sub32_sat(
        *buf1a.offset(3 as core::ffi::c_int as isize),
        *buf1a.offset(9 as core::ffi::c_int as isize),
    );
    s3 = ixheaac_add32_sat(
        *buf1a.offset(5 as core::ffi::c_int as isize),
        *buf1a.offset(7 as core::ffi::c_int as isize),
    );
    s2 = ixheaac_sub32_sat(
        *buf1a.offset(5 as core::ffi::c_int as isize),
        *buf1a.offset(7 as core::ffi::c_int as isize),
    );
    t = ixheaac_mult32_shl(ixheaac_sub32_sat(s1, s3), cos_54);
    s1 = ixheaac_add32_sat(s1, s3);
    temp2 = ixheaac_add32_sat(*buf1a.offset(1 as core::ffi::c_int as isize), s1);
    s1 = ixheaac_add32_sat(
        temp2,
        ixheaac_shl32_sat(ixheaac_mult32_shl(s1, cos_55), 1 as WORD),
    );
    s3 = ixheaac_sub32_sat(s1, t);
    s1 = ixheaac_add32_sat(s1, t);
    t = ixheaac_mult32_shl(ixheaac_add32_sat(s4, s2), cos_51);
    s4 = ixheaac_add32_sat(
        t,
        ixheaac_shl32_sat(ixheaac_mult32_shl(s4, cos_52), 1 as WORD),
    );
    s2 = ixheaac_add32_sat(t, ixheaac_mult32_shl(s2, cos_53));
    let fresh298 = buf2;
    buf2 = buf2.offset(1);
    *fresh298 = temp1;
    let fresh299 = buf2;
    buf2 = buf2.offset(1);
    *fresh299 = temp2;
    let fresh300 = buf2;
    buf2 = buf2.offset(1);
    *fresh300 = ixheaac_add32_sat(r1, s2);
    let fresh301 = buf2;
    buf2 = buf2.offset(1);
    *fresh301 = ixheaac_sub32_sat(s1, r2);
    let fresh302 = buf2;
    buf2 = buf2.offset(1);
    *fresh302 = ixheaac_sub32_sat(r3, s4);
    let fresh303 = buf2;
    buf2 = buf2.offset(1);
    *fresh303 = ixheaac_add32_sat(s3, r4);
    let fresh304 = buf2;
    buf2 = buf2.offset(1);
    *fresh304 = ixheaac_add32_sat(r3, s4);
    let fresh305 = buf2;
    buf2 = buf2.offset(1);
    *fresh305 = ixheaac_sub32_sat(s3, r4);
    let fresh306 = buf2;
    buf2 = buf2.offset(1);
    *fresh306 = ixheaac_sub32_sat(r1, s2);
    let fresh307 = buf2;
    buf2 = buf2.offset(1);
    *fresh307 = ixheaac_add32_sat(s1, r2);
    buf1a = buf1;
    n = 0 as core::ffi::c_int as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < FFT5 {
        xr_0 = *fft3outptr.offset(0 as core::ffi::c_int as isize);
        xi_0 = *fft3outptr.offset(1 as core::ffi::c_int as isize);
        xr_1 = *fft3outptr.offset(10 as core::ffi::c_int as isize);
        xi_1 = *fft3outptr.offset(11 as core::ffi::c_int as isize);
        xr_2 = *fft3outptr.offset(20 as core::ffi::c_int as isize);
        xi_2 = *fft3outptr.offset(21 as core::ffi::c_int as isize);
        x01_real = ixheaac_add32_sat(xr_0, xr_1);
        x_01_imag = ixheaac_add32_sat(xi_0, xi_1);
        add_r = ixheaac_add32_sat(xr_1, xr_2);
        add_i = ixheaac_add32_sat(xi_1, xi_2);
        sub_r = ixheaac_sub32_sat(xr_1, xr_2);
        sub_i = ixheaac_sub32_sat(xi_1, xi_2);
        p1 = add_r >> 1 as core::ffi::c_int;
        p2 = ixheaac_mult32_shl(sub_i, sinmu);
        p3 = ixheaac_mult32_shl(sub_r, sinmu);
        p4 = add_i >> 1 as core::ffi::c_int;
        temp = ixheaac_sub32_sat(xr_0, p1);
        temp1 = ixheaac_add32_sat(xi_0, p3);
        temp2 = ixheaac_sub32_sat(xi_0, p3);
        let fresh308 = n;
        n = n + 1;
        idx = ((*re_arr_tab_sml_240_ptr.offset(fresh308 as isize) as core::ffi::c_int)
            << 1 as core::ffi::c_int) as WORD32;
        *op.offset(idx as isize) = ixheaac_add32_sat(x01_real, xr_2);
        *op.offset((idx as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = ixheaac_add32_sat(
            x_01_imag,
            xi_2,
        );
        let fresh309 = n;
        n = n + 1;
        idx = ((*re_arr_tab_sml_240_ptr.offset(fresh309 as isize) as core::ffi::c_int)
            << 1 as core::ffi::c_int) as WORD32;
        *op.offset(idx as isize) = ixheaac_add32_sat(temp, p2);
        *op.offset((idx as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = ixheaac_sub32_sat(
            temp2,
            p4,
        );
        let fresh310 = n;
        n = n + 1;
        idx = ((*re_arr_tab_sml_240_ptr.offset(fresh310 as isize) as core::ffi::c_int)
            << 1 as core::ffi::c_int) as WORD32;
        *op.offset(idx as isize) = ixheaac_sub32_sat(temp, p2);
        *op.offset((idx as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = ixheaac_sub32_sat(
            temp1,
            p4,
        );
        fft3outptr = fft3outptr.offset(2 as core::ffi::c_int as isize);
        i += 1;
    }
}
