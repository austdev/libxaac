extern "C" {
    static mut ixheaacd_complex_fft_p2: Option<
        unsafe extern "C" fn(
            *mut WORD32,
            *mut WORD32,
            WORD32,
            WORD32,
            *mut WORD32,
        ) -> VOID,
    >;
    static ixheaacd_twiddle_table_fft_32x32: [WORD32; 514];
    static ixheaacd_twiddle_table_fft: [FLOAT32; 514];
    static ixheaacd_twiddle_table_fft_flt: [FLOAT32; 16];
    static ixheaacd_twiddle_table_3pr: [WORD32; 1155];
    static ixheaacd_twiddle_table_3pi: [WORD32; 1155];
    static ixheaacd_mps_dig_rev: [WORD8; 8];
}
pub type WORD8 = core::ffi::c_schar;
pub type WORD32 = core::ffi::c_int;
pub type WORD64 = core::ffi::c_longlong;
pub type FLOAT32 = core::ffi::c_float;
pub type VOID = ();
pub type WORD = core::ffi::c_int;
pub const MAX_32: WORD32 = 0x7fffffff as core::ffi::c_long as WORD32;
pub const MIN_32: WORD32 = 0x80000000 as core::ffi::c_long as WORD32;
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
unsafe extern "C" fn ixheaac_mult32_shl(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 32 as core::ffi::c_int) as WORD32;
    return result << 1 as core::ffi::c_int;
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
#[inline]
unsafe extern "C" fn ixheaacd_mult32_sat(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = ixheaac_sat64_32(temp_result >> 31 as core::ffi::c_int);
    return result;
}
#[inline]
unsafe extern "C" fn ixheaacd_mac32_sat(
    mut a: WORD32,
    mut b: WORD32,
    mut c: WORD32,
) -> WORD32 {
    let mut result: WORD32 = 0;
    result = ixheaac_add32_sat(a, ixheaacd_mult32_sat(b, c));
    return result;
}
#[inline]
unsafe extern "C" fn ixheaacd_mult32X32float(mut a: FLOAT32, mut b: FLOAT32) -> FLOAT32 {
    let mut result: FLOAT32 = 0.;
    result = a * b;
    return result;
}
#[inline]
unsafe extern "C" fn ixheaacd_mac32X32float(
    mut a: FLOAT32,
    mut b: FLOAT32,
    mut c: FLOAT32,
) -> FLOAT32 {
    let mut result: FLOAT32 = 0.;
    result = a + b * c;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_synth_calc_fft(
    mut ptr_xr: *mut FLOAT32,
    mut ptr_xi: *mut FLOAT32,
    mut npoints: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut y: [FLOAT32; 64] = [0.; 64];
    let mut z: [FLOAT32; 64] = [0.; 64];
    let mut ptr_y: *mut FLOAT32 = y.as_mut_ptr();
    let mut ptr_z: *mut FLOAT32 = z.as_mut_ptr();
    let mut ptr_w: *const FLOAT32 = ixheaacd_twiddle_table_fft_flt.as_ptr();
    i = 0 as core::ffi::c_int as WORD32;
    while i < npoints {
        let mut x0r: FLOAT32 = 0.;
        let mut x0i: FLOAT32 = 0.;
        let mut x1r: FLOAT32 = 0.;
        let mut x1i: FLOAT32 = 0.;
        let mut x2r: FLOAT32 = 0.;
        let mut x2i: FLOAT32 = 0.;
        let mut x3r: FLOAT32 = 0.;
        let mut x3i: FLOAT32 = 0.;
        let mut inp: *mut FLOAT32 = ptr_xr;
        let mut tmk: FLOAT32 = 0.;
        let mut h2: WORD32 = ixheaacd_mps_dig_rev[(i >> 2 as core::ffi::c_int) as usize]
            as WORD32;
        inp = inp.offset(h2 as isize);
        x0r = *inp;
        x0i = *inp.offset(1 as core::ffi::c_int as isize);
        inp = inp.offset(16 as core::ffi::c_int as isize);
        x1r = *inp;
        x1i = *inp.offset(1 as core::ffi::c_int as isize);
        inp = inp.offset(16 as core::ffi::c_int as isize);
        x2r = *inp;
        x2i = *inp.offset(1 as core::ffi::c_int as isize);
        inp = inp.offset(16 as core::ffi::c_int as isize);
        x3r = *inp;
        x3i = *inp.offset(1 as core::ffi::c_int as isize);
        x0r = x0r + x2r;
        x0i = x0i + x2i;
        tmk = x0r - x2r;
        x2r = tmk - x2r;
        tmk = x0i - x2i;
        x2i = tmk - x2i;
        x1r = x1r + x3r;
        x1i = x1i + x3i;
        tmk = x1r - x3r;
        x3r = tmk - x3r;
        tmk = x1i - x3i;
        x3i = tmk - x3i;
        x0r = x0r + x1r;
        x0i = x0i + x1i;
        tmk = x0r - x1r;
        x1r = tmk - x1r;
        tmk = x0i - x1i;
        x1i = tmk - x1i;
        x2r = x2r + x3i;
        x2i = x2i - x3r;
        tmk = x2r - x3i;
        x3i = tmk - x3i;
        tmk = x2i + x3r;
        x3r = tmk + x3r;
        let fresh22 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *fresh22 = x0r;
        let fresh23 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *fresh23 = x0i;
        let fresh24 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *fresh24 = x2r;
        let fresh25 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *fresh25 = x2i;
        let fresh26 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *fresh26 = x1r;
        let fresh27 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *fresh27 = x1i;
        let fresh28 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *fresh28 = x3i;
        let fresh29 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *fresh29 = x3r;
        inp = ptr_xi;
        inp = inp.offset(h2 as isize);
        x0r = *inp;
        x0i = *inp.offset(1 as core::ffi::c_int as isize);
        inp = inp.offset(16 as core::ffi::c_int as isize);
        x1r = *inp;
        x1i = *inp.offset(1 as core::ffi::c_int as isize);
        inp = inp.offset(16 as core::ffi::c_int as isize);
        x2r = *inp;
        x2i = *inp.offset(1 as core::ffi::c_int as isize);
        inp = inp.offset(16 as core::ffi::c_int as isize);
        x3r = *inp;
        x3i = *inp.offset(1 as core::ffi::c_int as isize);
        x0r = x0r + x2r;
        x0i = x0i + x2i;
        tmk = x0r - x2r;
        x2r = tmk - x2r;
        tmk = x0i - x2i;
        x2i = tmk - x2i;
        x1r = x1r + x3r;
        x1i = x1i + x3i;
        tmk = x1r - x3r;
        x3r = tmk - x3r;
        tmk = x1i - x3i;
        x3i = tmk - x3i;
        x0r = x0r + x1r;
        x0i = x0i + x1i;
        tmk = x0r - x1r;
        x1r = tmk - x1r;
        tmk = x0i - x1i;
        x1i = tmk - x1i;
        x2r = x2r + x3i;
        x2i = x2i - x3r;
        tmk = x2r - x3i;
        x3i = tmk - x3i;
        tmk = x2i + x3r;
        x3r = tmk + x3r;
        let fresh30 = ptr_z;
        ptr_z = ptr_z.offset(1);
        *fresh30 = x0r;
        let fresh31 = ptr_z;
        ptr_z = ptr_z.offset(1);
        *fresh31 = x0i;
        let fresh32 = ptr_z;
        ptr_z = ptr_z.offset(1);
        *fresh32 = x2r;
        let fresh33 = ptr_z;
        ptr_z = ptr_z.offset(1);
        *fresh33 = x2i;
        let fresh34 = ptr_z;
        ptr_z = ptr_z.offset(1);
        *fresh34 = x1r;
        let fresh35 = ptr_z;
        ptr_z = ptr_z.offset(1);
        *fresh35 = x1i;
        let fresh36 = ptr_z;
        ptr_z = ptr_z.offset(1);
        *fresh36 = x3i;
        let fresh37 = ptr_z;
        ptr_z = ptr_z.offset(1);
        *fresh37 = x3r;
        i += 4 as core::ffi::c_int;
    }
    ptr_y = ptr_y.offset(-(64 as core::ffi::c_int as isize));
    ptr_z = ptr_z.offset(-(64 as core::ffi::c_int as isize));
    let mut data_r: *mut FLOAT32 = ptr_y;
    let mut data_i: *mut FLOAT32 = ptr_z;
    k = 2 as core::ffi::c_int as WORD32;
    while k != 0 as core::ffi::c_int {
        let mut x0r_0: FLOAT32 = 0.;
        let mut x0i_0: FLOAT32 = 0.;
        let mut x1r_0: FLOAT32 = 0.;
        let mut x1i_0: FLOAT32 = 0.;
        let mut x2r_0: FLOAT32 = 0.;
        let mut x2i_0: FLOAT32 = 0.;
        let mut x3r_0: FLOAT32 = 0.;
        let mut x3i_0: FLOAT32 = 0.;
        x0r_0 = *data_r;
        x0i_0 = *data_r.offset(1 as core::ffi::c_int as isize);
        data_r = data_r.offset(8 as core::ffi::c_int as isize);
        x1r_0 = *data_r;
        x1i_0 = *data_r.offset(1 as core::ffi::c_int as isize);
        data_r = data_r.offset(8 as core::ffi::c_int as isize);
        x2r_0 = *data_r;
        x2i_0 = *data_r.offset(1 as core::ffi::c_int as isize);
        data_r = data_r.offset(8 as core::ffi::c_int as isize);
        x3r_0 = *data_r;
        x3i_0 = *data_r.offset(1 as core::ffi::c_int as isize);
        data_r = data_r.offset(-(24 as core::ffi::c_int as isize));
        x0r_0 = x0r_0 + x2r_0;
        x0i_0 = x0i_0 + x2i_0;
        x2r_0 = x0r_0 - x2r_0 * 2 as core::ffi::c_int as FLOAT32;
        x2i_0 = x0i_0 - x2i_0 * 2 as core::ffi::c_int as FLOAT32;
        x1r_0 = x1r_0 + x3r_0;
        x1i_0 = x1i_0 + x3i_0;
        x3r_0 = x1r_0 - x3r_0 * 2 as core::ffi::c_int as FLOAT32;
        x3i_0 = x1i_0 - x3i_0 * 2 as core::ffi::c_int as FLOAT32;
        x0r_0 = x0r_0 + x1r_0;
        x0i_0 = x0i_0 + x1i_0;
        x1r_0 = x0r_0 - x1r_0 * 2 as core::ffi::c_int as FLOAT32;
        x1i_0 = x0i_0 - x1i_0 * 2 as core::ffi::c_int as FLOAT32;
        x2r_0 = x2r_0 + x3i_0;
        x2i_0 = x2i_0 - x3r_0;
        x3i_0 = x2r_0 - x3i_0 * 2 as core::ffi::c_int as FLOAT32;
        x3r_0 = x2i_0 + x3r_0 * 2 as core::ffi::c_int as FLOAT32;
        *data_r = x0r_0;
        *data_r.offset(1 as core::ffi::c_int as isize) = x0i_0;
        data_r = data_r.offset(8 as core::ffi::c_int as isize);
        *data_r = x2r_0;
        *data_r.offset(1 as core::ffi::c_int as isize) = x2i_0;
        data_r = data_r.offset(8 as core::ffi::c_int as isize);
        *data_r = x1r_0;
        *data_r.offset(1 as core::ffi::c_int as isize) = x1i_0;
        data_r = data_r.offset(8 as core::ffi::c_int as isize);
        *data_r = x3i_0;
        *data_r.offset(1 as core::ffi::c_int as isize) = x3r_0;
        data_r = data_r.offset(8 as core::ffi::c_int as isize);
        x0r_0 = *data_i;
        x0i_0 = *data_i.offset(1 as core::ffi::c_int as isize);
        data_i = data_i.offset(8 as core::ffi::c_int as isize);
        x1r_0 = *data_i;
        x1i_0 = *data_i.offset(1 as core::ffi::c_int as isize);
        data_i = data_i.offset(8 as core::ffi::c_int as isize);
        x2r_0 = *data_i;
        x2i_0 = *data_i.offset(1 as core::ffi::c_int as isize);
        data_i = data_i.offset(8 as core::ffi::c_int as isize);
        x3r_0 = *data_i;
        x3i_0 = *data_i.offset(1 as core::ffi::c_int as isize);
        data_i = data_i.offset(-(24 as core::ffi::c_int as isize));
        x0r_0 = x0r_0 + x2r_0;
        x0i_0 = x0i_0 + x2i_0;
        x2r_0 = x0r_0 - x2r_0 * 2 as core::ffi::c_int as FLOAT32;
        x2i_0 = x0i_0 - x2i_0 * 2 as core::ffi::c_int as FLOAT32;
        x1r_0 = x1r_0 + x3r_0;
        x1i_0 = x1i_0 + x3i_0;
        x3r_0 = x1r_0 - x3r_0 * 2 as core::ffi::c_int as FLOAT32;
        x3i_0 = x1i_0 - x3i_0 * 2 as core::ffi::c_int as FLOAT32;
        x0r_0 = x0r_0 + x1r_0;
        x0i_0 = x0i_0 + x1i_0;
        x1r_0 = x0r_0 - x1r_0 * 2 as core::ffi::c_int as FLOAT32;
        x1i_0 = x0i_0 - x1i_0 * 2 as core::ffi::c_int as FLOAT32;
        x2r_0 = x2r_0 + x3i_0;
        x2i_0 = x2i_0 - x3r_0;
        x3i_0 = x2r_0 - x3i_0 * 2 as core::ffi::c_int as FLOAT32;
        x3r_0 = x2i_0 + x3r_0 * 2 as core::ffi::c_int as FLOAT32;
        *data_i = x0r_0;
        *data_i.offset(1 as core::ffi::c_int as isize) = x0i_0;
        data_i = data_i.offset(8 as core::ffi::c_int as isize);
        *data_i = x2r_0;
        *data_i.offset(1 as core::ffi::c_int as isize) = x2i_0;
        data_i = data_i.offset(8 as core::ffi::c_int as isize);
        *data_i = x1r_0;
        *data_i.offset(1 as core::ffi::c_int as isize) = x1i_0;
        data_i = data_i.offset(8 as core::ffi::c_int as isize);
        *data_i = x3i_0;
        *data_i.offset(1 as core::ffi::c_int as isize) = x3r_0;
        data_i = data_i.offset(8 as core::ffi::c_int as isize);
        k -= 1;
    }
    data_r = ptr_y.offset(2 as core::ffi::c_int as isize);
    data_i = ptr_z.offset(2 as core::ffi::c_int as isize);
    k = 2 as core::ffi::c_int as WORD32;
    while k != 0 as core::ffi::c_int {
        let mut tmp: FLOAT32 = 0.;
        let mut x0r_1: FLOAT32 = 0.;
        let mut x0i_1: FLOAT32 = 0.;
        let mut x1r_1: FLOAT32 = 0.;
        let mut x1i_1: FLOAT32 = 0.;
        let mut x2r_1: FLOAT32 = 0.;
        let mut x2i_1: FLOAT32 = 0.;
        let mut x3r_1: FLOAT32 = 0.;
        let mut x3i_1: FLOAT32 = 0.;
        data_r = data_r.offset(8 as core::ffi::c_int as isize);
        x1r_1 = *data_r;
        x1i_1 = *data_r.offset(1 as core::ffi::c_int as isize);
        data_r = data_r.offset(8 as core::ffi::c_int as isize);
        x2r_1 = *data_r;
        x2i_1 = *data_r.offset(1 as core::ffi::c_int as isize);
        data_r = data_r.offset(8 as core::ffi::c_int as isize);
        x3r_1 = *data_r;
        x3i_1 = *data_r.offset(1 as core::ffi::c_int as isize);
        data_r = data_r.offset(-(24 as core::ffi::c_int as isize));
        tmp = ixheaacd_mult32X32float(x1r_1, 0.923880f32)
            - ixheaacd_mult32X32float(x1i_1, -0.382683f32);
        x1i_1 = ixheaacd_mac32X32float(
            ixheaacd_mult32X32float(x1r_1, -0.382683f32),
            x1i_1,
            0.923880f32,
        );
        x1r_1 = tmp;
        tmp = ixheaacd_mult32X32float(x2r_1, 0.707107f32)
            - ixheaacd_mult32X32float(x2i_1, -0.707107f32);
        x2i_1 = ixheaacd_mac32X32float(
            ixheaacd_mult32X32float(x2r_1, -0.707107f32),
            x2i_1,
            0.707107f32,
        );
        x2r_1 = tmp;
        tmp = ixheaacd_mult32X32float(x3r_1, 0.382683f32)
            - ixheaacd_mult32X32float(x3i_1, -0.923880f32);
        x3i_1 = ixheaacd_mac32X32float(
            ixheaacd_mult32X32float(x3r_1, -0.923880f32),
            x3i_1,
            0.382683f32,
        );
        x3r_1 = tmp;
        x0r_1 = *data_r;
        x0i_1 = *data_r.offset(1 as core::ffi::c_int as isize);
        x0r_1 = x0r_1 + x2r_1;
        x0i_1 = x0i_1 + x2i_1;
        x2r_1 = x0r_1 - x2r_1 * 2 as core::ffi::c_int as FLOAT32;
        x2i_1 = x0i_1 - x2i_1 * 2 as core::ffi::c_int as FLOAT32;
        x1r_1 = x1r_1 + x3r_1;
        x1i_1 = x1i_1 + x3i_1;
        x3r_1 = x1r_1 - x3r_1 * 2 as core::ffi::c_int as FLOAT32;
        x3i_1 = x1i_1 - x3i_1 * 2 as core::ffi::c_int as FLOAT32;
        x0r_1 = x0r_1 + x1r_1;
        x0i_1 = x0i_1 + x1i_1;
        x1r_1 = x0r_1 - x1r_1 * 2 as core::ffi::c_int as FLOAT32;
        x1i_1 = x0i_1 - x1i_1 * 2 as core::ffi::c_int as FLOAT32;
        x2r_1 = x2r_1 + x3i_1;
        x2i_1 = x2i_1 - x3r_1;
        x3i_1 = x2r_1 - x3i_1 * 2 as core::ffi::c_int as FLOAT32;
        x3r_1 = x2i_1 + x3r_1 * 2 as core::ffi::c_int as FLOAT32;
        *data_r = x0r_1;
        *data_r.offset(1 as core::ffi::c_int as isize) = x0i_1;
        data_r = data_r.offset(8 as core::ffi::c_int as isize);
        *data_r = x2r_1;
        *data_r.offset(1 as core::ffi::c_int as isize) = x2i_1;
        data_r = data_r.offset(8 as core::ffi::c_int as isize);
        *data_r = x1r_1;
        *data_r.offset(1 as core::ffi::c_int as isize) = x1i_1;
        data_r = data_r.offset(8 as core::ffi::c_int as isize);
        *data_r = x3i_1;
        *data_r.offset(1 as core::ffi::c_int as isize) = x3r_1;
        data_r = data_r.offset(8 as core::ffi::c_int as isize);
        data_i = data_i.offset(8 as core::ffi::c_int as isize);
        x1r_1 = *data_i;
        x1i_1 = *data_i.offset(1 as core::ffi::c_int as isize);
        data_i = data_i.offset(8 as core::ffi::c_int as isize);
        x2r_1 = *data_i;
        x2i_1 = *data_i.offset(1 as core::ffi::c_int as isize);
        data_i = data_i.offset(8 as core::ffi::c_int as isize);
        x3r_1 = *data_i;
        x3i_1 = *data_i.offset(1 as core::ffi::c_int as isize);
        data_i = data_i.offset(-(24 as core::ffi::c_int as isize));
        tmp = ixheaacd_mult32X32float(x1r_1, 0.923880f32)
            - ixheaacd_mult32X32float(x1i_1, -0.382683f32);
        x1i_1 = ixheaacd_mac32X32float(
            ixheaacd_mult32X32float(x1r_1, -0.382683f32),
            x1i_1,
            0.923880f32,
        );
        x1r_1 = tmp;
        tmp = ixheaacd_mult32X32float(x2r_1, 0.707107f32)
            - ixheaacd_mult32X32float(x2i_1, -0.707107f32);
        x2i_1 = ixheaacd_mac32X32float(
            ixheaacd_mult32X32float(x2r_1, -0.707107f32),
            x2i_1,
            0.707107f32,
        );
        x2r_1 = tmp;
        tmp = ixheaacd_mult32X32float(x3r_1, 0.382683f32)
            - ixheaacd_mult32X32float(x3i_1, -0.923880f32);
        x3i_1 = ixheaacd_mac32X32float(
            ixheaacd_mult32X32float(x3r_1, -0.923880f32),
            x3i_1,
            0.382683f32,
        );
        x3r_1 = tmp;
        x0r_1 = *data_i;
        x0i_1 = *data_i.offset(1 as core::ffi::c_int as isize);
        x0r_1 = x0r_1 + x2r_1;
        x0i_1 = x0i_1 + x2i_1;
        x2r_1 = x0r_1 - x2r_1 * 2 as core::ffi::c_int as FLOAT32;
        x2i_1 = x0i_1 - x2i_1 * 2 as core::ffi::c_int as FLOAT32;
        x1r_1 = x1r_1 + x3r_1;
        x1i_1 = x1i_1 + x3i_1;
        x3r_1 = x1r_1 - x3r_1 * 2 as core::ffi::c_int as FLOAT32;
        x3i_1 = x1i_1 - x3i_1 * 2 as core::ffi::c_int as FLOAT32;
        x0r_1 = x0r_1 + x1r_1;
        x0i_1 = x0i_1 + x1i_1;
        x1r_1 = x0r_1 - x1r_1 * 2 as core::ffi::c_int as FLOAT32;
        x1i_1 = x0i_1 - x1i_1 * 2 as core::ffi::c_int as FLOAT32;
        x2r_1 = x2r_1 + x3i_1;
        x2i_1 = x2i_1 - x3r_1;
        x3i_1 = x2r_1 - x3i_1 * 2 as core::ffi::c_int as FLOAT32;
        x3r_1 = x2i_1 + x3r_1 * 2 as core::ffi::c_int as FLOAT32;
        *data_i = x0r_1;
        *data_i.offset(1 as core::ffi::c_int as isize) = x0i_1;
        data_i = data_i.offset(8 as core::ffi::c_int as isize);
        *data_i = x2r_1;
        *data_i.offset(1 as core::ffi::c_int as isize) = x2i_1;
        data_i = data_i.offset(8 as core::ffi::c_int as isize);
        *data_i = x1r_1;
        *data_i.offset(1 as core::ffi::c_int as isize) = x1i_1;
        data_i = data_i.offset(8 as core::ffi::c_int as isize);
        *data_i = x3i_1;
        *data_i.offset(1 as core::ffi::c_int as isize) = x3r_1;
        data_i = data_i.offset(8 as core::ffi::c_int as isize);
        k -= 1;
    }
    data_r = data_r.offset(-(62 as core::ffi::c_int as isize));
    data_i = data_i.offset(-(62 as core::ffi::c_int as isize));
    k = 2 as core::ffi::c_int as WORD32;
    while k != 0 as core::ffi::c_int {
        let mut tmp_0: FLOAT32 = 0.;
        let mut x0r_2: FLOAT32 = 0.;
        let mut x0i_2: FLOAT32 = 0.;
        let mut x1r_2: FLOAT32 = 0.;
        let mut x1i_2: FLOAT32 = 0.;
        let mut x2r_2: FLOAT32 = 0.;
        let mut x2i_2: FLOAT32 = 0.;
        let mut x3r_2: FLOAT32 = 0.;
        let mut x3i_2: FLOAT32 = 0.;
        data_r = data_r.offset(8 as core::ffi::c_int as isize);
        x1r_2 = *data_r;
        x1i_2 = *data_r.offset(1 as core::ffi::c_int as isize);
        data_r = data_r.offset(8 as core::ffi::c_int as isize);
        x2r_2 = *data_r;
        x2i_2 = *data_r.offset(1 as core::ffi::c_int as isize);
        data_r = data_r.offset(8 as core::ffi::c_int as isize);
        x3r_2 = *data_r;
        x3i_2 = *data_r.offset(1 as core::ffi::c_int as isize);
        data_r = data_r.offset(-(24 as core::ffi::c_int as isize));
        tmp_0 = ixheaacd_mult32X32float(x1r_2, 0.707107f32)
            - ixheaacd_mult32X32float(x1i_2, -0.707107f32);
        x1i_2 = ixheaacd_mac32X32float(
            ixheaacd_mult32X32float(x1r_2, -0.707107f32),
            x1i_2,
            0.707107f32,
        );
        x1r_2 = tmp_0;
        tmp_0 = x2i_2;
        x2i_2 = -x2r_2;
        x2r_2 = tmp_0;
        tmp_0 = ixheaacd_mult32X32float(x3r_2, -0.707107f32)
            + ixheaacd_mult32X32float(x3i_2, 0.707107f32);
        x3i_2 = -ixheaacd_mult32X32float(x3r_2, 0.707107f32)
            + ixheaacd_mult32X32float(x3i_2, -0.707107f32);
        x3r_2 = tmp_0;
        x0r_2 = *data_r;
        x0i_2 = *data_r.offset(1 as core::ffi::c_int as isize);
        x0r_2 = x0r_2 + x2r_2;
        x0i_2 = x0i_2 + x2i_2;
        x2r_2 = x0r_2 - x2r_2 * 2 as core::ffi::c_int as FLOAT32;
        x2i_2 = x0i_2 - x2i_2 * 2 as core::ffi::c_int as FLOAT32;
        x1r_2 = x1r_2 + x3r_2;
        x1i_2 = x1i_2 + x3i_2;
        x3r_2 = x1r_2 - x3r_2 * 2 as core::ffi::c_int as FLOAT32;
        x3i_2 = x1i_2 - x3i_2 * 2 as core::ffi::c_int as FLOAT32;
        x0r_2 = x0r_2 + x1r_2;
        x0i_2 = x0i_2 + x1i_2;
        x1r_2 = x0r_2 - x1r_2 * 2 as core::ffi::c_int as FLOAT32;
        x1i_2 = x0i_2 - x1i_2 * 2 as core::ffi::c_int as FLOAT32;
        x2r_2 = x2r_2 + x3i_2;
        x2i_2 = x2i_2 - x3r_2;
        x3i_2 = x2r_2 - x3i_2 * 2 as core::ffi::c_int as FLOAT32;
        x3r_2 = x2i_2 + x3r_2 * 2 as core::ffi::c_int as FLOAT32;
        *data_r = x0r_2;
        *data_r.offset(1 as core::ffi::c_int as isize) = x0i_2;
        data_r = data_r.offset(8 as core::ffi::c_int as isize);
        *data_r = x2r_2;
        *data_r.offset(1 as core::ffi::c_int as isize) = x2i_2;
        data_r = data_r.offset(8 as core::ffi::c_int as isize);
        *data_r = x1r_2;
        *data_r.offset(1 as core::ffi::c_int as isize) = x1i_2;
        data_r = data_r.offset(8 as core::ffi::c_int as isize);
        *data_r = x3i_2;
        *data_r.offset(1 as core::ffi::c_int as isize) = x3r_2;
        data_r = data_r.offset(8 as core::ffi::c_int as isize);
        data_i = data_i.offset(8 as core::ffi::c_int as isize);
        x1r_2 = *data_i;
        x1i_2 = *data_i.offset(1 as core::ffi::c_int as isize);
        data_i = data_i.offset(8 as core::ffi::c_int as isize);
        x2r_2 = *data_i;
        x2i_2 = *data_i.offset(1 as core::ffi::c_int as isize);
        data_i = data_i.offset(8 as core::ffi::c_int as isize);
        x3r_2 = *data_i;
        x3i_2 = *data_i.offset(1 as core::ffi::c_int as isize);
        data_i = data_i.offset(-(24 as core::ffi::c_int as isize));
        tmp_0 = ixheaacd_mult32X32float(x1r_2, 0.707107f32)
            - ixheaacd_mult32X32float(x1i_2, -0.707107f32);
        x1i_2 = ixheaacd_mac32X32float(
            ixheaacd_mult32X32float(x1r_2, -0.707107f32),
            x1i_2,
            0.707107f32,
        );
        x1r_2 = tmp_0;
        tmp_0 = x2i_2;
        x2i_2 = -x2r_2;
        x2r_2 = tmp_0;
        tmp_0 = ixheaacd_mult32X32float(x3r_2, -0.707107f32)
            + ixheaacd_mult32X32float(x3i_2, 0.707107f32);
        x3i_2 = -ixheaacd_mult32X32float(x3r_2, 0.707107f32)
            + ixheaacd_mult32X32float(x3i_2, -0.707107f32);
        x3r_2 = tmp_0;
        x0r_2 = *data_i;
        x0i_2 = *data_i.offset(1 as core::ffi::c_int as isize);
        x0r_2 = x0r_2 + x2r_2;
        x0i_2 = x0i_2 + x2i_2;
        x2r_2 = x0r_2 - x2r_2 * 2 as core::ffi::c_int as FLOAT32;
        x2i_2 = x0i_2 - x2i_2 * 2 as core::ffi::c_int as FLOAT32;
        x1r_2 = x1r_2 + x3r_2;
        x1i_2 = x1i_2 + x3i_2;
        x3r_2 = x1r_2 - x3r_2 * 2 as core::ffi::c_int as FLOAT32;
        x3i_2 = x1i_2 - x3i_2 * 2 as core::ffi::c_int as FLOAT32;
        x0r_2 = x0r_2 + x1r_2;
        x0i_2 = x0i_2 + x1i_2;
        x1r_2 = x0r_2 - x1r_2 * 2 as core::ffi::c_int as FLOAT32;
        x1i_2 = x0i_2 - x1i_2 * 2 as core::ffi::c_int as FLOAT32;
        x2r_2 = x2r_2 + x3i_2;
        x2i_2 = x2i_2 - x3r_2;
        x3i_2 = x2r_2 - x3i_2 * 2 as core::ffi::c_int as FLOAT32;
        x3r_2 = x2i_2 + x3r_2 * 2 as core::ffi::c_int as FLOAT32;
        *data_i = x0r_2;
        *data_i.offset(1 as core::ffi::c_int as isize) = x0i_2;
        data_i = data_i.offset(8 as core::ffi::c_int as isize);
        *data_i = x2r_2;
        *data_i.offset(1 as core::ffi::c_int as isize) = x2i_2;
        data_i = data_i.offset(8 as core::ffi::c_int as isize);
        *data_i = x1r_2;
        *data_i.offset(1 as core::ffi::c_int as isize) = x1i_2;
        data_i = data_i.offset(8 as core::ffi::c_int as isize);
        *data_i = x3i_2;
        *data_i.offset(1 as core::ffi::c_int as isize) = x3r_2;
        data_i = data_i.offset(8 as core::ffi::c_int as isize);
        k -= 1;
    }
    data_r = data_r.offset(-(62 as core::ffi::c_int as isize));
    data_i = data_i.offset(-(62 as core::ffi::c_int as isize));
    k = 2 as core::ffi::c_int as WORD32;
    while k != 0 as core::ffi::c_int {
        let mut tmp_1: FLOAT32 = 0.;
        let mut x0r_3: FLOAT32 = 0.;
        let mut x0i_3: FLOAT32 = 0.;
        let mut x1r_3: FLOAT32 = 0.;
        let mut x1i_3: FLOAT32 = 0.;
        let mut x2r_3: FLOAT32 = 0.;
        let mut x2i_3: FLOAT32 = 0.;
        let mut x3r_3: FLOAT32 = 0.;
        let mut x3i_3: FLOAT32 = 0.;
        data_r = data_r.offset(8 as core::ffi::c_int as isize);
        x1r_3 = *data_r;
        x1i_3 = *data_r.offset(1 as core::ffi::c_int as isize);
        data_r = data_r.offset(8 as core::ffi::c_int as isize);
        x2r_3 = *data_r;
        x2i_3 = *data_r.offset(1 as core::ffi::c_int as isize);
        data_r = data_r.offset(8 as core::ffi::c_int as isize);
        x3r_3 = *data_r;
        x3i_3 = *data_r.offset(1 as core::ffi::c_int as isize);
        data_r = data_r.offset(-(24 as core::ffi::c_int as isize));
        tmp_1 = ixheaacd_mult32X32float(x1r_3, 0.382683f32)
            - ixheaacd_mult32X32float(x1i_3, -0.923880f32);
        x1i_3 = ixheaacd_mac32X32float(
            ixheaacd_mult32X32float(x1r_3, -0.923880f32),
            x1i_3,
            0.382683f32,
        );
        x1r_3 = tmp_1;
        tmp_1 = ixheaacd_mult32X32float(x2r_3, -0.707107f32)
            + ixheaacd_mult32X32float(x2i_3, 0.707107f32);
        x2i_3 = -ixheaacd_mult32X32float(x2r_3, 0.707107f32)
            + ixheaacd_mult32X32float(x2i_3, -0.707107f32);
        x2r_3 = tmp_1;
        tmp_1 = -ixheaacd_mult32X32float(x3r_3, 0.923880f32)
            + ixheaacd_mult32X32float(x3i_3, -0.382683f32);
        x3i_3 = ixheaacd_mac32X32float(
            ixheaacd_mult32X32float(x3r_3, -0.382683f32),
            x3i_3,
            0.923880f32,
        );
        x3r_3 = tmp_1;
        x0r_3 = *data_r;
        x0i_3 = *data_r.offset(1 as core::ffi::c_int as isize);
        x0r_3 = x0r_3 + x2r_3;
        x0i_3 = x0i_3 + x2i_3;
        x2r_3 = x0r_3 - x2r_3 * 2 as core::ffi::c_int as FLOAT32;
        x2i_3 = x0i_3 - x2i_3 * 2 as core::ffi::c_int as FLOAT32;
        x1r_3 = x1r_3 + x3r_3;
        x1i_3 = x1i_3 - x3i_3;
        x3r_3 = x1r_3 - x3r_3 * 2 as core::ffi::c_int as FLOAT32;
        x3i_3 = x1i_3 + x3i_3 * 2 as core::ffi::c_int as FLOAT32;
        x0r_3 = x0r_3 + x1r_3;
        x0i_3 = x0i_3 + x1i_3;
        x1r_3 = x0r_3 - x1r_3 * 2 as core::ffi::c_int as FLOAT32;
        x1i_3 = x0i_3 - x1i_3 * 2 as core::ffi::c_int as FLOAT32;
        x2r_3 = x2r_3 + x3i_3;
        x2i_3 = x2i_3 - x3r_3;
        x3i_3 = x2r_3 - x3i_3 * 2 as core::ffi::c_int as FLOAT32;
        x3r_3 = x2i_3 + x3r_3 * 2 as core::ffi::c_int as FLOAT32;
        *data_r = x0r_3;
        *data_r.offset(1 as core::ffi::c_int as isize) = x0i_3;
        data_r = data_r.offset(8 as core::ffi::c_int as isize);
        *data_r = x2r_3;
        *data_r.offset(1 as core::ffi::c_int as isize) = x2i_3;
        data_r = data_r.offset(8 as core::ffi::c_int as isize);
        *data_r = x1r_3;
        *data_r.offset(1 as core::ffi::c_int as isize) = x1i_3;
        data_r = data_r.offset(8 as core::ffi::c_int as isize);
        *data_r = x3i_3;
        *data_r.offset(1 as core::ffi::c_int as isize) = x3r_3;
        data_r = data_r.offset(8 as core::ffi::c_int as isize);
        data_i = data_i.offset(8 as core::ffi::c_int as isize);
        x1r_3 = *data_i;
        x1i_3 = *data_i.offset(1 as core::ffi::c_int as isize);
        data_i = data_i.offset(8 as core::ffi::c_int as isize);
        x2r_3 = *data_i;
        x2i_3 = *data_i.offset(1 as core::ffi::c_int as isize);
        data_i = data_i.offset(8 as core::ffi::c_int as isize);
        x3r_3 = *data_i;
        x3i_3 = *data_i.offset(1 as core::ffi::c_int as isize);
        data_i = data_i.offset(-(24 as core::ffi::c_int as isize));
        tmp_1 = ixheaacd_mult32X32float(x1r_3, 0.382683f32)
            - ixheaacd_mult32X32float(x1i_3, -0.923880f32);
        x1i_3 = ixheaacd_mac32X32float(
            ixheaacd_mult32X32float(x1r_3, -0.923880f32),
            x1i_3,
            0.382683f32,
        );
        x1r_3 = tmp_1;
        tmp_1 = ixheaacd_mult32X32float(x2r_3, -0.707107f32)
            + ixheaacd_mult32X32float(x2i_3, 0.707107f32);
        x2i_3 = -ixheaacd_mult32X32float(x2r_3, 0.707107f32)
            + ixheaacd_mult32X32float(x2i_3, -0.707107f32);
        x2r_3 = tmp_1;
        tmp_1 = -ixheaacd_mult32X32float(x3r_3, 0.923880f32)
            + ixheaacd_mult32X32float(x3i_3, -0.382683f32);
        x3i_3 = ixheaacd_mac32X32float(
            ixheaacd_mult32X32float(x3r_3, -0.382683f32),
            x3i_3,
            0.923880f32,
        );
        x3r_3 = tmp_1;
        x0r_3 = *data_i;
        x0i_3 = *data_i.offset(1 as core::ffi::c_int as isize);
        x0r_3 = x0r_3 + x2r_3;
        x0i_3 = x0i_3 + x2i_3;
        x2r_3 = x0r_3 - x2r_3 * 2 as core::ffi::c_int as FLOAT32;
        x2i_3 = x0i_3 - x2i_3 * 2 as core::ffi::c_int as FLOAT32;
        x1r_3 = x1r_3 + x3r_3;
        x1i_3 = x1i_3 - x3i_3;
        x3r_3 = x1r_3 - x3r_3 * 2 as core::ffi::c_int as FLOAT32;
        x3i_3 = x1i_3 + x3i_3 * 2 as core::ffi::c_int as FLOAT32;
        x0r_3 = x0r_3 + x1r_3;
        x0i_3 = x0i_3 + x1i_3;
        x1r_3 = x0r_3 - x1r_3 * 2 as core::ffi::c_int as FLOAT32;
        x1i_3 = x0i_3 - x1i_3 * 2 as core::ffi::c_int as FLOAT32;
        x2r_3 = x2r_3 + x3i_3;
        x2i_3 = x2i_3 - x3r_3;
        x3i_3 = x2r_3 - x3i_3 * 2 as core::ffi::c_int as FLOAT32;
        x3r_3 = x2i_3 + x3r_3 * 2 as core::ffi::c_int as FLOAT32;
        *data_i = x0r_3;
        *data_i.offset(1 as core::ffi::c_int as isize) = x0i_3;
        data_i = data_i.offset(8 as core::ffi::c_int as isize);
        *data_i = x2r_3;
        *data_i.offset(1 as core::ffi::c_int as isize) = x2i_3;
        data_i = data_i.offset(8 as core::ffi::c_int as isize);
        *data_i = x1r_3;
        *data_i.offset(1 as core::ffi::c_int as isize) = x1i_3;
        data_i = data_i.offset(8 as core::ffi::c_int as isize);
        *data_i = x3i_3;
        *data_i.offset(1 as core::ffi::c_int as isize) = x3r_3;
        data_i = data_i.offset(8 as core::ffi::c_int as isize);
        k -= 1;
    }
    data_r = data_r.offset(-(62 as core::ffi::c_int as isize));
    data_i = data_i.offset(-(62 as core::ffi::c_int as isize));
    let mut twiddles: *const FLOAT32 = ptr_w;
    let mut x0r_4: FLOAT32 = 0.;
    let mut x0i_4: FLOAT32 = 0.;
    let mut x1r_4: FLOAT32 = 0.;
    let mut x1i_4: FLOAT32 = 0.;
    j = 8 as core::ffi::c_int as WORD32;
    while j != 0 as core::ffi::c_int {
        let mut W1: FLOAT32 = *twiddles;
        twiddles = twiddles.offset(1);
        let mut W4: FLOAT32 = *twiddles;
        twiddles = twiddles.offset(1);
        let mut tmp_2: FLOAT32 = 0.;
        x0r_4 = *ptr_y;
        x0i_4 = *ptr_y.offset(1 as core::ffi::c_int as isize);
        ptr_y = ptr_y.offset(32 as core::ffi::c_int as isize);
        ptr_xr = ptr_xr.offset(32 as core::ffi::c_int as isize);
        x1r_4 = *ptr_y;
        x1i_4 = *ptr_y.offset(1 as core::ffi::c_int as isize);
        tmp_2 = ixheaacd_mult32X32float(x1r_4, W1) - ixheaacd_mult32X32float(x1i_4, W4);
        x1i_4 = ixheaacd_mac32X32float(ixheaacd_mult32X32float(x1r_4, W4), x1i_4, W1);
        x1r_4 = tmp_2;
        *ptr_xr = x0r_4 - x1r_4;
        *ptr_xr.offset(1 as core::ffi::c_int as isize) = x0i_4 - x1i_4;
        ptr_y = ptr_y.offset(-(32 as core::ffi::c_int as isize));
        ptr_xr = ptr_xr.offset(-(32 as core::ffi::c_int as isize));
        *ptr_xr = x0r_4 + x1r_4;
        *ptr_xr.offset(1 as core::ffi::c_int as isize) = x0i_4 + x1i_4;
        ptr_y = ptr_y.offset(2 as core::ffi::c_int as isize);
        ptr_xr = ptr_xr.offset(2 as core::ffi::c_int as isize);
        x0r_4 = *ptr_z;
        x0i_4 = *ptr_z.offset(1 as core::ffi::c_int as isize);
        ptr_z = ptr_z.offset(32 as core::ffi::c_int as isize);
        ptr_xi = ptr_xi.offset(32 as core::ffi::c_int as isize);
        x1r_4 = *ptr_z;
        x1i_4 = *ptr_z.offset(1 as core::ffi::c_int as isize);
        tmp_2 = ixheaacd_mult32X32float(x1r_4, W1) - ixheaacd_mult32X32float(x1i_4, W4);
        x1i_4 = ixheaacd_mac32X32float(ixheaacd_mult32X32float(x1r_4, W4), x1i_4, W1);
        x1r_4 = tmp_2;
        *ptr_xi = x0r_4 - x1r_4;
        *ptr_xi.offset(1 as core::ffi::c_int as isize) = x0i_4 - x1i_4;
        ptr_z = ptr_z.offset(-(32 as core::ffi::c_int as isize));
        ptr_xi = ptr_xi.offset(-(32 as core::ffi::c_int as isize));
        *ptr_xi = x0r_4 + x1r_4;
        *ptr_xi.offset(1 as core::ffi::c_int as isize) = x0i_4 + x1i_4;
        ptr_z = ptr_z.offset(2 as core::ffi::c_int as isize);
        ptr_xi = ptr_xi.offset(2 as core::ffi::c_int as isize);
        j -= 1;
    }
    twiddles = ptr_w;
    j = 8 as core::ffi::c_int as WORD32;
    while j != 0 as core::ffi::c_int {
        let mut W1_0: FLOAT32 = *twiddles;
        twiddles = twiddles.offset(1);
        let mut W4_0: FLOAT32 = *twiddles;
        twiddles = twiddles.offset(1);
        let mut tmp_3: FLOAT32 = 0.;
        x0r_4 = *ptr_y;
        x0i_4 = *ptr_y.offset(1 as core::ffi::c_int as isize);
        ptr_y = ptr_y.offset(32 as core::ffi::c_int as isize);
        ptr_xr = ptr_xr.offset(32 as core::ffi::c_int as isize);
        x1r_4 = *ptr_y;
        x1i_4 = *ptr_y.offset(1 as core::ffi::c_int as isize);
        tmp_3 = ixheaacd_mult32X32float(x1r_4, W4_0)
            + ixheaacd_mult32X32float(x1i_4, W1_0);
        x1i_4 = -ixheaacd_mult32X32float(x1r_4, W1_0)
            + ixheaacd_mult32X32float(x1i_4, W4_0);
        x1r_4 = tmp_3;
        *ptr_xr = x0r_4 - x1r_4;
        *ptr_xr.offset(1 as core::ffi::c_int as isize) = x0i_4 - x1i_4;
        ptr_y = ptr_y.offset(-(32 as core::ffi::c_int as isize));
        ptr_xr = ptr_xr.offset(-(32 as core::ffi::c_int as isize));
        *ptr_xr = x0r_4 + x1r_4;
        *ptr_xr.offset(1 as core::ffi::c_int as isize) = x0i_4 + x1i_4;
        ptr_y = ptr_y.offset(2 as core::ffi::c_int as isize);
        ptr_xr = ptr_xr.offset(2 as core::ffi::c_int as isize);
        x0r_4 = *ptr_z;
        x0i_4 = *ptr_z.offset(1 as core::ffi::c_int as isize);
        ptr_z = ptr_z.offset(32 as core::ffi::c_int as isize);
        ptr_xi = ptr_xi.offset(32 as core::ffi::c_int as isize);
        x1r_4 = *ptr_z;
        x1i_4 = *ptr_z.offset(1 as core::ffi::c_int as isize);
        tmp_3 = ixheaacd_mult32X32float(x1r_4, W4_0)
            + ixheaacd_mult32X32float(x1i_4, W1_0);
        x1i_4 = -ixheaacd_mult32X32float(x1r_4, W1_0)
            + ixheaacd_mult32X32float(x1i_4, W4_0);
        x1r_4 = tmp_3;
        *ptr_xi = x0r_4 - x1r_4;
        *ptr_xi.offset(1 as core::ffi::c_int as isize) = x0i_4 - x1i_4;
        ptr_z = ptr_z.offset(-(32 as core::ffi::c_int as isize));
        ptr_xi = ptr_xi.offset(-(32 as core::ffi::c_int as isize));
        *ptr_xi = x0r_4 + x1r_4;
        *ptr_xi.offset(1 as core::ffi::c_int as isize) = x0i_4 + x1i_4;
        ptr_z = ptr_z.offset(2 as core::ffi::c_int as isize);
        ptr_xi = ptr_xi.offset(2 as core::ffi::c_int as isize);
        j -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_complex_fft(
    mut xr: *mut FLOAT32,
    mut xi: *mut FLOAT32,
    mut nlength: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut n_stages: WORD32 = 0;
    let mut h2: WORD32 = 0;
    let mut x0r: FLOAT32 = 0.;
    let mut x0i: FLOAT32 = 0.;
    let mut x1r: FLOAT32 = 0.;
    let mut x1i: FLOAT32 = 0.;
    let mut x2r: FLOAT32 = 0.;
    let mut x2i: FLOAT32 = 0.;
    let mut x3r: FLOAT32 = 0.;
    let mut x3i: FLOAT32 = 0.;
    let mut del: WORD32 = 0;
    let mut nodespacing: WORD32 = 0;
    let mut in_loop_cnt: WORD32 = 0;
    let mut dig_rev_shift: WORD32 = 0;
    let mut not_power_4: WORD32 = 0;
    let mut ptr_x: [FLOAT32; 256] = [0.; 256];
    let mut y: [FLOAT32; 256] = [0.; 256];
    let mut npoints: WORD32 = nlength;
    let mut ptr_y: *mut FLOAT32 = y.as_mut_ptr();
    let mut ptr_w: *const FLOAT32 = 0 as *const FLOAT32;
    dig_rev_shift = (ixheaac_norm32(npoints) as core::ffi::c_int + 1 as core::ffi::c_int
        - 16 as core::ffi::c_int) as WORD32;
    n_stages = (30 as WORD - ixheaac_norm32(npoints)) as WORD32;
    not_power_4 = (n_stages as core::ffi::c_int & 1 as core::ffi::c_int) as WORD32;
    n_stages = n_stages >> 1 as core::ffi::c_int;
    i = 0 as core::ffi::c_int as WORD32;
    while i < nlength {
        ptr_x[(2 as WORD32 * i) as usize] = *xr.offset(i as isize);
        ptr_x[(2 as core::ffi::c_int * i as core::ffi::c_int + 1 as core::ffi::c_int)
            as usize] = *xi.offset(i as isize);
        i += 1;
    }
    ptr_w = ixheaacd_twiddle_table_fft.as_ptr();
    i = 0 as core::ffi::c_int as WORD32;
    while i < npoints {
        let mut inp: *mut FLOAT32 = ptr_x.as_mut_ptr();
        let mut val: core::ffi::c_uint = i as core::ffi::c_uint;
        val = (val & 0x33333333 as core::ffi::c_uint) << 2 as core::ffi::c_int
            | (val & !(0x33333333 as core::ffi::c_int) as core::ffi::c_uint)
                >> 2 as core::ffi::c_int;
        val = (val & 0xf0f0f0f as core::ffi::c_uint) << 4 as core::ffi::c_int
            | (val & !(0xf0f0f0f as core::ffi::c_int) as core::ffi::c_uint)
                >> 4 as core::ffi::c_int;
        val = (val & 0xff00ff as core::ffi::c_uint) << 8 as core::ffi::c_int
            | (val & !(0xff00ff as core::ffi::c_int) as core::ffi::c_uint)
                >> 8 as core::ffi::c_int;
        h2 = (val >> dig_rev_shift) as WORD32;
        if not_power_4 != 0 {
            h2 += 1 as core::ffi::c_int;
            h2 &= !(1 as core::ffi::c_int);
        }
        inp = inp.offset(h2 as isize);
        x0r = *inp;
        x0i = *inp.offset(1 as core::ffi::c_int as isize);
        inp = inp.offset((npoints >> 1 as core::ffi::c_int) as isize);
        x1r = *inp;
        x1i = *inp.offset(1 as core::ffi::c_int as isize);
        inp = inp.offset((npoints >> 1 as core::ffi::c_int) as isize);
        x2r = *inp;
        x2i = *inp.offset(1 as core::ffi::c_int as isize);
        inp = inp.offset((npoints >> 1 as core::ffi::c_int) as isize);
        x3r = *inp;
        x3i = *inp.offset(1 as core::ffi::c_int as isize);
        x0r = x0r + x2r;
        x0i = x0i + x2i;
        x2r = x0r - x2r * 2 as core::ffi::c_int as FLOAT32;
        x2i = x0i - x2i * 2 as core::ffi::c_int as FLOAT32;
        x1r = x1r + x3r;
        x1i = x1i + x3i;
        x3r = x1r - x3r * 2 as core::ffi::c_int as FLOAT32;
        x3i = x1i - x3i * 2 as core::ffi::c_int as FLOAT32;
        x0r = x0r + x1r;
        x0i = x0i + x1i;
        x1r = x0r - x1r * 2 as core::ffi::c_int as FLOAT32;
        x1i = x0i - x1i * 2 as core::ffi::c_int as FLOAT32;
        x2r = x2r + x3i;
        x2i = x2i - x3r;
        x3i = x2r - x3i * 2 as core::ffi::c_int as FLOAT32;
        x3r = x2i + x3r * 2 as core::ffi::c_int as FLOAT32;
        let fresh38 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *fresh38 = x0r;
        let fresh39 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *fresh39 = x0i;
        let fresh40 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *fresh40 = x2r;
        let fresh41 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *fresh41 = x2i;
        let fresh42 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *fresh42 = x1r;
        let fresh43 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *fresh43 = x1i;
        let fresh44 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *fresh44 = x3i;
        let fresh45 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *fresh45 = x3r;
        i += 4 as core::ffi::c_int;
    }
    ptr_y = ptr_y.offset(-((2 as WORD32 * npoints) as isize));
    del = 4 as core::ffi::c_int as WORD32;
    nodespacing = 64 as core::ffi::c_int as WORD32;
    in_loop_cnt = npoints >> 4 as core::ffi::c_int;
    i = (n_stages as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while i > 0 as core::ffi::c_int {
        let mut twiddles: *const FLOAT32 = ptr_w;
        let mut data: *mut FLOAT32 = ptr_y;
        let mut w1h: FLOAT32 = 0.;
        let mut w2h: FLOAT32 = 0.;
        let mut w3h: FLOAT32 = 0.;
        let mut w1l: FLOAT32 = 0.;
        let mut w2l: FLOAT32 = 0.;
        let mut w3l: FLOAT32 = 0.;
        let mut sec_loop_cnt: WORD32 = 0;
        k = in_loop_cnt;
        while k != 0 as core::ffi::c_int {
            x0r = *data;
            x0i = *data.offset(1 as core::ffi::c_int as isize);
            data = data.offset((del << 1 as core::ffi::c_int) as isize);
            x1r = *data;
            x1i = *data.offset(1 as core::ffi::c_int as isize);
            data = data.offset((del << 1 as core::ffi::c_int) as isize);
            x2r = *data;
            x2i = *data.offset(1 as core::ffi::c_int as isize);
            data = data.offset((del << 1 as core::ffi::c_int) as isize);
            x3r = *data;
            x3i = *data.offset(1 as core::ffi::c_int as isize);
            data = data
                .offset(-((3 as WORD32 * (del << 1 as core::ffi::c_int)) as isize));
            x0r = x0r + x2r;
            x0i = x0i + x2i;
            x2r = x0r - x2r * 2 as core::ffi::c_int as FLOAT32;
            x2i = x0i - x2i * 2 as core::ffi::c_int as FLOAT32;
            x1r = x1r + x3r;
            x1i = x1i + x3i;
            x3r = x1r - x3r * 2 as core::ffi::c_int as FLOAT32;
            x3i = x1i - x3i * 2 as core::ffi::c_int as FLOAT32;
            x0r = x0r + x1r;
            x0i = x0i + x1i;
            x1r = x0r - x1r * 2 as core::ffi::c_int as FLOAT32;
            x1i = x0i - x1i * 2 as core::ffi::c_int as FLOAT32;
            x2r = x2r + x3i;
            x2i = x2i - x3r;
            x3i = x2r - x3i * 2 as core::ffi::c_int as FLOAT32;
            x3r = x2i + x3r * 2 as core::ffi::c_int as FLOAT32;
            *data = x0r;
            *data.offset(1 as core::ffi::c_int as isize) = x0i;
            data = data.offset((del << 1 as core::ffi::c_int) as isize);
            *data = x2r;
            *data.offset(1 as core::ffi::c_int as isize) = x2i;
            data = data.offset((del << 1 as core::ffi::c_int) as isize);
            *data = x1r;
            *data.offset(1 as core::ffi::c_int as isize) = x1i;
            data = data.offset((del << 1 as core::ffi::c_int) as isize);
            *data = x3i;
            *data.offset(1 as core::ffi::c_int as isize) = x3r;
            data = data.offset((del << 1 as core::ffi::c_int) as isize);
            k -= 1;
        }
        data = ptr_y.offset(2 as core::ffi::c_int as isize);
        sec_loop_cnt = nodespacing * del;
        sec_loop_cnt = (sec_loop_cnt as core::ffi::c_int / 4 as core::ffi::c_int
            + sec_loop_cnt as core::ffi::c_int / 8 as core::ffi::c_int
            - sec_loop_cnt as core::ffi::c_int / 16 as core::ffi::c_int
            + sec_loop_cnt as core::ffi::c_int / 32 as core::ffi::c_int
            - sec_loop_cnt as core::ffi::c_int / 64 as core::ffi::c_int
            + sec_loop_cnt as core::ffi::c_int / 128 as core::ffi::c_int
            - sec_loop_cnt as core::ffi::c_int / 256 as core::ffi::c_int) as WORD32;
        j = nodespacing;
        j = nodespacing;
        while j <= sec_loop_cnt {
            w1h = *twiddles.offset((2 as WORD32 * j) as isize);
            w1l = *twiddles
                .offset((2 as WORD32 * j) as isize)
                .offset(1 as core::ffi::c_int as isize);
            w2h = *twiddles
                .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize);
            w2l = *twiddles
                .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                .offset(1 as core::ffi::c_int as isize);
            w3h = *twiddles
                .offset((2 as WORD32 * j) as isize)
                .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize);
            w3l = *twiddles
                .offset((2 as WORD32 * j) as isize)
                .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                .offset(1 as core::ffi::c_int as isize);
            k = in_loop_cnt;
            while k != 0 as core::ffi::c_int {
                let mut tmp: FLOAT32 = 0.;
                let mut x0r_0: FLOAT32 = 0.;
                let mut x0i_0: FLOAT32 = 0.;
                let mut x1r_0: FLOAT32 = 0.;
                let mut x1i_0: FLOAT32 = 0.;
                let mut x2r_0: FLOAT32 = 0.;
                let mut x2i_0: FLOAT32 = 0.;
                let mut x3r_0: FLOAT32 = 0.;
                let mut x3i_0: FLOAT32 = 0.;
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                x1r_0 = *data;
                x1i_0 = *data.offset(1 as core::ffi::c_int as isize);
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                x2r_0 = *data;
                x2i_0 = *data.offset(1 as core::ffi::c_int as isize);
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                x3r_0 = *data;
                x3i_0 = *data.offset(1 as core::ffi::c_int as isize);
                data = data
                    .offset(-((3 as WORD32 * (del << 1 as core::ffi::c_int)) as isize));
                tmp = ixheaacd_mult32X32float(x1r_0, w1l)
                    - ixheaacd_mult32X32float(x1i_0, w1h);
                x1i_0 = ixheaacd_mac32X32float(
                    ixheaacd_mult32X32float(x1r_0, w1h),
                    x1i_0,
                    w1l,
                );
                x1r_0 = tmp;
                tmp = ixheaacd_mult32X32float(x2r_0, w2l)
                    - ixheaacd_mult32X32float(x2i_0, w2h);
                x2i_0 = ixheaacd_mac32X32float(
                    ixheaacd_mult32X32float(x2r_0, w2h),
                    x2i_0,
                    w2l,
                );
                x2r_0 = tmp;
                tmp = ixheaacd_mult32X32float(x3r_0, w3l)
                    - ixheaacd_mult32X32float(x3i_0, w3h);
                x3i_0 = ixheaacd_mac32X32float(
                    ixheaacd_mult32X32float(x3r_0, w3h),
                    x3i_0,
                    w3l,
                );
                x3r_0 = tmp;
                x0r_0 = *data;
                x0i_0 = *data.offset(1 as core::ffi::c_int as isize);
                x0r_0 = x0r_0 + x2r_0;
                x0i_0 = x0i_0 + x2i_0;
                x2r_0 = x0r_0 - x2r_0 * 2 as core::ffi::c_int as FLOAT32;
                x2i_0 = x0i_0 - x2i_0 * 2 as core::ffi::c_int as FLOAT32;
                x1r_0 = x1r_0 + x3r_0;
                x1i_0 = x1i_0 + x3i_0;
                x3r_0 = x1r_0 - x3r_0 * 2 as core::ffi::c_int as FLOAT32;
                x3i_0 = x1i_0 - x3i_0 * 2 as core::ffi::c_int as FLOAT32;
                x0r_0 = x0r_0 + x1r_0;
                x0i_0 = x0i_0 + x1i_0;
                x1r_0 = x0r_0 - x1r_0 * 2 as core::ffi::c_int as FLOAT32;
                x1i_0 = x0i_0 - x1i_0 * 2 as core::ffi::c_int as FLOAT32;
                x2r_0 = x2r_0 + x3i_0;
                x2i_0 = x2i_0 - x3r_0;
                x3i_0 = x2r_0 - x3i_0 * 2 as core::ffi::c_int as FLOAT32;
                x3r_0 = x2i_0 + x3r_0 * 2 as core::ffi::c_int as FLOAT32;
                *data = x0r_0;
                *data.offset(1 as core::ffi::c_int as isize) = x0i_0;
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                *data = x2r_0;
                *data.offset(1 as core::ffi::c_int as isize) = x2i_0;
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                *data = x1r_0;
                *data.offset(1 as core::ffi::c_int as isize) = x1i_0;
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                *data = x3i_0;
                *data.offset(1 as core::ffi::c_int as isize) = x3r_0;
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                k -= 1;
            }
            data = data.offset(-((2 as WORD32 * npoints) as isize));
            data = data.offset(2 as core::ffi::c_int as isize);
            j += nodespacing;
        }
        while j <= nodespacing * del >> 1 as core::ffi::c_int {
            w1h = *twiddles.offset((2 as WORD32 * j) as isize);
            w2h = *twiddles
                .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize);
            w3h = *twiddles
                .offset((2 as WORD32 * j) as isize)
                .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                .offset(-(512 as core::ffi::c_int as isize));
            w1l = *twiddles
                .offset((2 as WORD32 * j) as isize)
                .offset(1 as core::ffi::c_int as isize);
            w2l = *twiddles
                .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                .offset(1 as core::ffi::c_int as isize);
            w3l = *twiddles
                .offset((2 as WORD32 * j) as isize)
                .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                .offset(-(511 as core::ffi::c_int as isize));
            k = in_loop_cnt;
            while k != 0 as core::ffi::c_int {
                let mut tmp_0: FLOAT32 = 0.;
                let mut x0r_1: FLOAT32 = 0.;
                let mut x0i_1: FLOAT32 = 0.;
                let mut x1r_1: FLOAT32 = 0.;
                let mut x1i_1: FLOAT32 = 0.;
                let mut x2r_1: FLOAT32 = 0.;
                let mut x2i_1: FLOAT32 = 0.;
                let mut x3r_1: FLOAT32 = 0.;
                let mut x3i_1: FLOAT32 = 0.;
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                x1r_1 = *data;
                x1i_1 = *data.offset(1 as core::ffi::c_int as isize);
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                x2r_1 = *data;
                x2i_1 = *data.offset(1 as core::ffi::c_int as isize);
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                x3r_1 = *data;
                x3i_1 = *data.offset(1 as core::ffi::c_int as isize);
                data = data
                    .offset(-((3 as WORD32 * (del << 1 as core::ffi::c_int)) as isize));
                tmp_0 = ixheaacd_mult32X32float(x1r_1, w1l)
                    - ixheaacd_mult32X32float(x1i_1, w1h);
                x1i_1 = ixheaacd_mac32X32float(
                    ixheaacd_mult32X32float(x1r_1, w1h),
                    x1i_1,
                    w1l,
                );
                x1r_1 = tmp_0;
                tmp_0 = ixheaacd_mult32X32float(x2r_1, w2l)
                    - ixheaacd_mult32X32float(x2i_1, w2h);
                x2i_1 = ixheaacd_mac32X32float(
                    ixheaacd_mult32X32float(x2r_1, w2h),
                    x2i_1,
                    w2l,
                );
                x2r_1 = tmp_0;
                tmp_0 = ixheaacd_mult32X32float(x3r_1, w3h)
                    + ixheaacd_mult32X32float(x3i_1, w3l);
                x3i_1 = -ixheaacd_mult32X32float(x3r_1, w3l)
                    + ixheaacd_mult32X32float(x3i_1, w3h);
                x3r_1 = tmp_0;
                x0r_1 = *data;
                x0i_1 = *data.offset(1 as core::ffi::c_int as isize);
                x0r_1 = x0r_1 + x2r_1;
                x0i_1 = x0i_1 + x2i_1;
                x2r_1 = x0r_1 - x2r_1 * 2 as core::ffi::c_int as FLOAT32;
                x2i_1 = x0i_1 - x2i_1 * 2 as core::ffi::c_int as FLOAT32;
                x1r_1 = x1r_1 + x3r_1;
                x1i_1 = x1i_1 + x3i_1;
                x3r_1 = x1r_1 - x3r_1 * 2 as core::ffi::c_int as FLOAT32;
                x3i_1 = x1i_1 - x3i_1 * 2 as core::ffi::c_int as FLOAT32;
                x0r_1 = x0r_1 + x1r_1;
                x0i_1 = x0i_1 + x1i_1;
                x1r_1 = x0r_1 - x1r_1 * 2 as core::ffi::c_int as FLOAT32;
                x1i_1 = x0i_1 - x1i_1 * 2 as core::ffi::c_int as FLOAT32;
                x2r_1 = x2r_1 + x3i_1;
                x2i_1 = x2i_1 - x3r_1;
                x3i_1 = x2r_1 - x3i_1 * 2 as core::ffi::c_int as FLOAT32;
                x3r_1 = x2i_1 + x3r_1 * 2 as core::ffi::c_int as FLOAT32;
                *data = x0r_1;
                *data.offset(1 as core::ffi::c_int as isize) = x0i_1;
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                *data = x2r_1;
                *data.offset(1 as core::ffi::c_int as isize) = x2i_1;
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                *data = x1r_1;
                *data.offset(1 as core::ffi::c_int as isize) = x1i_1;
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                *data = x3i_1;
                *data.offset(1 as core::ffi::c_int as isize) = x3r_1;
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                k -= 1;
            }
            data = data.offset(-((2 as WORD32 * npoints) as isize));
            data = data.offset(2 as core::ffi::c_int as isize);
            j += nodespacing;
        }
        while j <= sec_loop_cnt as core::ffi::c_int * 2 as core::ffi::c_int {
            w1h = *twiddles.offset((2 as WORD32 * j) as isize);
            w2h = *twiddles
                .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                .offset(-(512 as core::ffi::c_int as isize));
            w3h = *twiddles
                .offset((2 as WORD32 * j) as isize)
                .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                .offset(-(512 as core::ffi::c_int as isize));
            w1l = *twiddles
                .offset((2 as WORD32 * j) as isize)
                .offset(1 as core::ffi::c_int as isize);
            w2l = *twiddles
                .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                .offset(-(511 as core::ffi::c_int as isize));
            w3l = *twiddles
                .offset((2 as WORD32 * j) as isize)
                .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                .offset(-(511 as core::ffi::c_int as isize));
            k = in_loop_cnt;
            while k != 0 as core::ffi::c_int {
                let mut tmp_1: FLOAT32 = 0.;
                let mut x0r_2: FLOAT32 = 0.;
                let mut x0i_2: FLOAT32 = 0.;
                let mut x1r_2: FLOAT32 = 0.;
                let mut x1i_2: FLOAT32 = 0.;
                let mut x2r_2: FLOAT32 = 0.;
                let mut x2i_2: FLOAT32 = 0.;
                let mut x3r_2: FLOAT32 = 0.;
                let mut x3i_2: FLOAT32 = 0.;
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                x1r_2 = *data;
                x1i_2 = *data.offset(1 as core::ffi::c_int as isize);
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                x2r_2 = *data;
                x2i_2 = *data.offset(1 as core::ffi::c_int as isize);
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                x3r_2 = *data;
                x3i_2 = *data.offset(1 as core::ffi::c_int as isize);
                data = data
                    .offset(-((3 as WORD32 * (del << 1 as core::ffi::c_int)) as isize));
                tmp_1 = ixheaacd_mult32X32float(x1r_2, w1l)
                    - ixheaacd_mult32X32float(x1i_2, w1h);
                x1i_2 = ixheaacd_mac32X32float(
                    ixheaacd_mult32X32float(x1r_2, w1h),
                    x1i_2,
                    w1l,
                );
                x1r_2 = tmp_1;
                tmp_1 = ixheaacd_mult32X32float(x2r_2, w2h)
                    + ixheaacd_mult32X32float(x2i_2, w2l);
                x2i_2 = -ixheaacd_mult32X32float(x2r_2, w2l)
                    + ixheaacd_mult32X32float(x2i_2, w2h);
                x2r_2 = tmp_1;
                tmp_1 = ixheaacd_mult32X32float(x3r_2, w3h)
                    + ixheaacd_mult32X32float(x3i_2, w3l);
                x3i_2 = -ixheaacd_mult32X32float(x3r_2, w3l)
                    + ixheaacd_mult32X32float(x3i_2, w3h);
                x3r_2 = tmp_1;
                x0r_2 = *data;
                x0i_2 = *data.offset(1 as core::ffi::c_int as isize);
                x0r_2 = x0r_2 + x2r_2;
                x0i_2 = x0i_2 + x2i_2;
                x2r_2 = x0r_2 - x2r_2 * 2 as core::ffi::c_int as FLOAT32;
                x2i_2 = x0i_2 - x2i_2 * 2 as core::ffi::c_int as FLOAT32;
                x1r_2 = x1r_2 + x3r_2;
                x1i_2 = x1i_2 + x3i_2;
                x3r_2 = x1r_2 - x3r_2 * 2 as core::ffi::c_int as FLOAT32;
                x3i_2 = x1i_2 - x3i_2 * 2 as core::ffi::c_int as FLOAT32;
                x0r_2 = x0r_2 + x1r_2;
                x0i_2 = x0i_2 + x1i_2;
                x1r_2 = x0r_2 - x1r_2 * 2 as core::ffi::c_int as FLOAT32;
                x1i_2 = x0i_2 - x1i_2 * 2 as core::ffi::c_int as FLOAT32;
                x2r_2 = x2r_2 + x3i_2;
                x2i_2 = x2i_2 - x3r_2;
                x3i_2 = x2r_2 - x3i_2 * 2 as core::ffi::c_int as FLOAT32;
                x3r_2 = x2i_2 + x3r_2 * 2 as core::ffi::c_int as FLOAT32;
                *data = x0r_2;
                *data.offset(1 as core::ffi::c_int as isize) = x0i_2;
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                *data = x2r_2;
                *data.offset(1 as core::ffi::c_int as isize) = x2i_2;
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                *data = x1r_2;
                *data.offset(1 as core::ffi::c_int as isize) = x1i_2;
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                *data = x3i_2;
                *data.offset(1 as core::ffi::c_int as isize) = x3r_2;
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                k -= 1;
            }
            data = data.offset(-((2 as WORD32 * npoints) as isize));
            data = data.offset(2 as core::ffi::c_int as isize);
            j += nodespacing;
        }
        while j < nodespacing * del {
            w1h = *twiddles.offset((2 as WORD32 * j) as isize);
            w2h = *twiddles
                .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                .offset(-(512 as core::ffi::c_int as isize));
            w3h = *twiddles
                .offset((2 as WORD32 * j) as isize)
                .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                .offset(-(1024 as core::ffi::c_int as isize));
            w1l = *twiddles
                .offset((2 as WORD32 * j) as isize)
                .offset(1 as core::ffi::c_int as isize);
            w2l = *twiddles
                .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                .offset(-(511 as core::ffi::c_int as isize));
            w3l = *twiddles
                .offset((2 as WORD32 * j) as isize)
                .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                .offset(-(1023 as core::ffi::c_int as isize));
            k = in_loop_cnt;
            while k != 0 as core::ffi::c_int {
                let mut tmp_2: FLOAT32 = 0.;
                let mut x0r_3: FLOAT32 = 0.;
                let mut x0i_3: FLOAT32 = 0.;
                let mut x1r_3: FLOAT32 = 0.;
                let mut x1i_3: FLOAT32 = 0.;
                let mut x2r_3: FLOAT32 = 0.;
                let mut x2i_3: FLOAT32 = 0.;
                let mut x3r_3: FLOAT32 = 0.;
                let mut x3i_3: FLOAT32 = 0.;
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                x1r_3 = *data;
                x1i_3 = *data.offset(1 as core::ffi::c_int as isize);
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                x2r_3 = *data;
                x2i_3 = *data.offset(1 as core::ffi::c_int as isize);
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                x3r_3 = *data;
                x3i_3 = *data.offset(1 as core::ffi::c_int as isize);
                data = data
                    .offset(-((3 as WORD32 * (del << 1 as core::ffi::c_int)) as isize));
                tmp_2 = ixheaacd_mult32X32float(x1r_3, w1l)
                    - ixheaacd_mult32X32float(x1i_3, w1h);
                x1i_3 = ixheaacd_mac32X32float(
                    ixheaacd_mult32X32float(x1r_3, w1h),
                    x1i_3,
                    w1l,
                );
                x1r_3 = tmp_2;
                tmp_2 = ixheaacd_mult32X32float(x2r_3, w2h)
                    + ixheaacd_mult32X32float(x2i_3, w2l);
                x2i_3 = -ixheaacd_mult32X32float(x2r_3, w2l)
                    + ixheaacd_mult32X32float(x2i_3, w2h);
                x2r_3 = tmp_2;
                tmp_2 = -ixheaacd_mult32X32float(x3r_3, w3l)
                    + ixheaacd_mult32X32float(x3i_3, w3h);
                x3i_3 = ixheaacd_mac32X32float(
                    ixheaacd_mult32X32float(x3r_3, w3h),
                    x3i_3,
                    w3l,
                );
                x3r_3 = tmp_2;
                x0r_3 = *data;
                x0i_3 = *data.offset(1 as core::ffi::c_int as isize);
                x0r_3 = x0r_3 + x2r_3;
                x0i_3 = x0i_3 + x2i_3;
                x2r_3 = x0r_3 - x2r_3 * 2 as core::ffi::c_int as FLOAT32;
                x2i_3 = x0i_3 - x2i_3 * 2 as core::ffi::c_int as FLOAT32;
                x1r_3 = x1r_3 + x3r_3;
                x1i_3 = x1i_3 - x3i_3;
                x3r_3 = x1r_3 - x3r_3 * 2 as core::ffi::c_int as FLOAT32;
                x3i_3 = x1i_3 + x3i_3 * 2 as core::ffi::c_int as FLOAT32;
                x0r_3 = x0r_3 + x1r_3;
                x0i_3 = x0i_3 + x1i_3;
                x1r_3 = x0r_3 - x1r_3 * 2 as core::ffi::c_int as FLOAT32;
                x1i_3 = x0i_3 - x1i_3 * 2 as core::ffi::c_int as FLOAT32;
                x2r_3 = x2r_3 + x3i_3;
                x2i_3 = x2i_3 - x3r_3;
                x3i_3 = x2r_3 - x3i_3 * 2 as core::ffi::c_int as FLOAT32;
                x3r_3 = x2i_3 + x3r_3 * 2 as core::ffi::c_int as FLOAT32;
                *data = x0r_3;
                *data.offset(1 as core::ffi::c_int as isize) = x0i_3;
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                *data = x2r_3;
                *data.offset(1 as core::ffi::c_int as isize) = x2i_3;
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                *data = x1r_3;
                *data.offset(1 as core::ffi::c_int as isize) = x1i_3;
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                *data = x3i_3;
                *data.offset(1 as core::ffi::c_int as isize) = x3r_3;
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                k -= 1;
            }
            data = data.offset(-((2 as WORD32 * npoints) as isize));
            data = data.offset(2 as core::ffi::c_int as isize);
            j += nodespacing;
        }
        nodespacing >>= 2 as core::ffi::c_int;
        del <<= 2 as core::ffi::c_int;
        in_loop_cnt >>= 2 as core::ffi::c_int;
        i -= 1;
    }
    if not_power_4 != 0 {
        let mut twiddles_0: *const FLOAT32 = ptr_w;
        nodespacing <<= 1 as core::ffi::c_int;
        j = (del as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
        while j != 0 as core::ffi::c_int {
            let mut w1h_0: FLOAT32 = *twiddles_0;
            let mut w1l_0: FLOAT32 = *twiddles_0.offset(1 as core::ffi::c_int as isize);
            let mut tmp_3: FLOAT32 = 0.;
            twiddles_0 = twiddles_0
                .offset(
                    (nodespacing as core::ffi::c_int * 2 as core::ffi::c_int) as isize,
                );
            x0r = *ptr_y;
            x0i = *ptr_y.offset(1 as core::ffi::c_int as isize);
            ptr_y = ptr_y.offset((del << 1 as core::ffi::c_int) as isize);
            x1r = *ptr_y;
            x1i = *ptr_y.offset(1 as core::ffi::c_int as isize);
            tmp_3 = ixheaacd_mult32X32float(x1r, w1l_0)
                - ixheaacd_mult32X32float(x1i, w1h_0);
            x1i = ixheaacd_mac32X32float(
                ixheaacd_mult32X32float(x1r, w1h_0),
                x1i,
                w1l_0,
            );
            x1r = tmp_3;
            *ptr_y = x0r - x1r;
            *ptr_y.offset(1 as core::ffi::c_int as isize) = x0i - x1i;
            ptr_y = ptr_y.offset(-((del << 1 as core::ffi::c_int) as isize));
            *ptr_y = x0r + x1r;
            *ptr_y.offset(1 as core::ffi::c_int as isize) = x0i + x1i;
            ptr_y = ptr_y.offset(2 as core::ffi::c_int as isize);
            j -= 1;
        }
        twiddles_0 = ptr_w;
        j = (del as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
        while j != 0 as core::ffi::c_int {
            let mut w1h_1: FLOAT32 = *twiddles_0;
            let mut w1l_1: FLOAT32 = *twiddles_0.offset(1 as core::ffi::c_int as isize);
            let mut tmp_4: FLOAT32 = 0.;
            twiddles_0 = twiddles_0
                .offset(
                    (nodespacing as core::ffi::c_int * 2 as core::ffi::c_int) as isize,
                );
            x0r = *ptr_y;
            x0i = *ptr_y.offset(1 as core::ffi::c_int as isize);
            ptr_y = ptr_y.offset((del << 1 as core::ffi::c_int) as isize);
            x1r = *ptr_y;
            x1i = *ptr_y.offset(1 as core::ffi::c_int as isize);
            tmp_4 = ixheaacd_mult32X32float(x1r, w1h_1)
                + ixheaacd_mult32X32float(x1i, w1l_1);
            x1i = -ixheaacd_mult32X32float(x1r, w1l_1)
                + ixheaacd_mult32X32float(x1i, w1h_1);
            x1r = tmp_4;
            *ptr_y = x0r - x1r;
            *ptr_y.offset(1 as core::ffi::c_int as isize) = x0i - x1i;
            ptr_y = ptr_y.offset(-((del << 1 as core::ffi::c_int) as isize));
            *ptr_y = x0r + x1r;
            *ptr_y.offset(1 as core::ffi::c_int as isize) = x0i + x1i;
            ptr_y = ptr_y.offset(2 as core::ffi::c_int as isize);
            j -= 1;
        }
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < nlength {
        *xr.offset(i as isize) = y[(2 as WORD32 * i) as usize];
        *xi.offset(i as isize) = y[(2 as core::ffi::c_int * i as core::ffi::c_int
            + 1 as core::ffi::c_int) as usize];
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_complex_fft_p2_dec(
    mut xr: *mut WORD32,
    mut xi: *mut WORD32,
    mut nlength: WORD32,
    mut fft_mode: WORD32,
    mut preshift: *mut WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut n_stages: WORD32 = 0;
    let mut h2: WORD32 = 0;
    let mut x0r: WORD32 = 0;
    let mut x0i: WORD32 = 0;
    let mut x1r: WORD32 = 0;
    let mut x1i: WORD32 = 0;
    let mut x2r: WORD32 = 0;
    let mut x2i: WORD32 = 0;
    let mut x3r: WORD32 = 0;
    let mut x3i: WORD32 = 0;
    let mut del: WORD32 = 0;
    let mut nodespacing: WORD32 = 0;
    let mut in_loop_cnt: WORD32 = 0;
    let mut not_power_4: WORD32 = 0;
    let mut npts: WORD32 = 0;
    let mut shift: WORD32 = 0;
    let mut dig_rev_shift: WORD32 = 0;
    let mut ptr_x: [WORD32; 1024] = [0; 1024];
    let mut y: [WORD32; 1024] = [0; 1024];
    let mut npoints: WORD32 = nlength;
    let mut n: WORD32 = 0 as WORD32;
    let mut ptr_y: *mut WORD32 = y.as_mut_ptr();
    let mut ptr_w: *const WORD32 = 0 as *const WORD32;
    dig_rev_shift = (ixheaac_norm32(npoints) as core::ffi::c_int + 1 as core::ffi::c_int
        - 16 as core::ffi::c_int) as WORD32;
    n_stages = (30 as WORD - ixheaac_norm32(npoints)) as WORD32;
    not_power_4 = (n_stages as core::ffi::c_int & 1 as core::ffi::c_int) as WORD32;
    n_stages = n_stages >> 1 as core::ffi::c_int;
    npts = npoints;
    while npts >> 1 as core::ffi::c_int != 0 {
        n += 1;
        npts = npts >> 1 as core::ffi::c_int;
    }
    if n as core::ffi::c_int % 2 as core::ffi::c_int == 0 as core::ffi::c_int {
        shift = ((n as core::ffi::c_int + 4 as core::ffi::c_int) / 2 as core::ffi::c_int)
            as WORD32;
    } else {
        shift = ((n as core::ffi::c_int + 3 as core::ffi::c_int) / 2 as core::ffi::c_int)
            as WORD32;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < nlength {
        ptr_x[(2 as WORD32 * i) as usize] = (*xr.offset(i as isize)
            / ((1 as core::ffi::c_int) << shift)) as WORD32;
        ptr_x[(2 as core::ffi::c_int * i as core::ffi::c_int + 1 as core::ffi::c_int)
            as usize] = (*xi.offset(i as isize) / ((1 as core::ffi::c_int) << shift))
            as WORD32;
        i += 1;
    }
    if fft_mode == -(1 as core::ffi::c_int) {
        ptr_w = ixheaacd_twiddle_table_fft_32x32.as_ptr();
        i = 0 as core::ffi::c_int as WORD32;
        while i < npoints {
            let mut inp: *mut WORD32 = ptr_x.as_mut_ptr();
            let mut val: core::ffi::c_uint = i as core::ffi::c_uint;
            val = (val & 0x33333333 as core::ffi::c_uint) << 2 as core::ffi::c_int
                | (val & !(0x33333333 as core::ffi::c_int) as core::ffi::c_uint)
                    >> 2 as core::ffi::c_int;
            val = (val & 0xf0f0f0f as core::ffi::c_uint) << 4 as core::ffi::c_int
                | (val & !(0xf0f0f0f as core::ffi::c_int) as core::ffi::c_uint)
                    >> 4 as core::ffi::c_int;
            val = (val & 0xff00ff as core::ffi::c_uint) << 8 as core::ffi::c_int
                | (val & !(0xff00ff as core::ffi::c_int) as core::ffi::c_uint)
                    >> 8 as core::ffi::c_int;
            h2 = (val >> dig_rev_shift) as WORD32;
            if not_power_4 != 0 {
                h2 += 1 as core::ffi::c_int;
                h2 &= !(1 as core::ffi::c_int);
            }
            inp = inp.offset(h2 as isize);
            x0r = *inp;
            x0i = *inp.offset(1 as core::ffi::c_int as isize);
            inp = inp.offset((npoints >> 1 as core::ffi::c_int) as isize);
            x1r = *inp;
            x1i = *inp.offset(1 as core::ffi::c_int as isize);
            inp = inp.offset((npoints >> 1 as core::ffi::c_int) as isize);
            x2r = *inp;
            x2i = *inp.offset(1 as core::ffi::c_int as isize);
            inp = inp.offset((npoints >> 1 as core::ffi::c_int) as isize);
            x3r = *inp;
            x3i = *inp.offset(1 as core::ffi::c_int as isize);
            x0r = ixheaac_add32_sat(x0r, x2r);
            x0i = ixheaac_add32_sat(x0i, x2i);
            x2r = ixheaac_sub32_sat(x0r, ixheaac_shl32_sat(x2r, 1 as WORD));
            x2i = ixheaac_sub32_sat(x0i, ixheaac_shl32_sat(x2i, 1 as WORD));
            x1r = ixheaac_add32_sat(x1r, x3r);
            x1i = ixheaac_add32_sat(x1i, x3i);
            x3r = ixheaac_sub32_sat(x1r, ixheaac_shl32_sat(x3r, 1 as WORD));
            x3i = ixheaac_sub32_sat(x1i, ixheaac_shl32_sat(x3i, 1 as WORD));
            x0r = ixheaac_add32_sat(x0r, x1r);
            x0i = ixheaac_add32_sat(x0i, x1i);
            x1r = ixheaac_sub32_sat(x0r, ixheaac_shl32_sat(x1r, 1 as WORD));
            x1i = ixheaac_sub32_sat(x0i, ixheaac_shl32_sat(x1i, 1 as WORD));
            x2r = ixheaac_add32_sat(x2r, x3i);
            x2i = ixheaac_sub32_sat(x2i, x3r);
            x3i = ixheaac_sub32_sat(x2r, ixheaac_shl32_sat(x3i, 1 as WORD));
            x3r = ixheaac_add32_sat(x2i, ixheaac_shl32_sat(x3r, 1 as WORD));
            let fresh6 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh6 = x0r;
            let fresh7 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh7 = x0i;
            let fresh8 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh8 = x2r;
            let fresh9 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh9 = x2i;
            let fresh10 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh10 = x1r;
            let fresh11 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh11 = x1i;
            let fresh12 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh12 = x3i;
            let fresh13 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh13 = x3r;
            i += 4 as core::ffi::c_int;
        }
        ptr_y = ptr_y.offset(-((2 as WORD32 * npoints) as isize));
        del = 4 as core::ffi::c_int as WORD32;
        nodespacing = 64 as core::ffi::c_int as WORD32;
        in_loop_cnt = npoints >> 4 as core::ffi::c_int;
        i = (n_stages as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        while i > 0 as core::ffi::c_int {
            let mut twiddles: *const WORD32 = ptr_w;
            let mut data: *mut WORD32 = ptr_y;
            let mut w1h: WORD32 = 0;
            let mut w2h: WORD32 = 0;
            let mut w3h: WORD32 = 0;
            let mut w1l: WORD32 = 0;
            let mut w2l: WORD32 = 0;
            let mut w3l: WORD32 = 0;
            let mut sec_loop_cnt: WORD32 = 0;
            k = in_loop_cnt;
            while k != 0 as core::ffi::c_int {
                x0r = *data;
                x0i = *data.offset(1 as core::ffi::c_int as isize);
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                x1r = *data;
                x1i = *data.offset(1 as core::ffi::c_int as isize);
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                x2r = *data;
                x2i = *data.offset(1 as core::ffi::c_int as isize);
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                x3r = *data;
                x3i = *data.offset(1 as core::ffi::c_int as isize);
                data = data
                    .offset(-((3 as WORD32 * (del << 1 as core::ffi::c_int)) as isize));
                x0r = ixheaac_add32_sat(x0r, x2r);
                x0i = ixheaac_add32_sat(x0i, x2i);
                x2r = ixheaac_sub32_sat(x0r, ixheaac_shl32_sat(x2r, 1 as WORD));
                x2i = ixheaac_sub32_sat(x0i, ixheaac_shl32_sat(x2i, 1 as WORD));
                x1r = ixheaac_add32_sat(x1r, x3r);
                x1i = ixheaac_add32_sat(x1i, x3i);
                x3r = ixheaac_sub32_sat(x1r, ixheaac_shl32_sat(x3r, 1 as WORD));
                x3i = ixheaac_sub32_sat(x1i, ixheaac_shl32_sat(x3i, 1 as WORD));
                x0r = ixheaac_add32_sat(x0r, x1r);
                x0i = ixheaac_add32_sat(x0i, x1i);
                x1r = ixheaac_sub32_sat(x0r, ixheaac_shl32_sat(x1r, 1 as WORD));
                x1i = ixheaac_sub32_sat(x0i, ixheaac_shl32_sat(x1i, 1 as WORD));
                x2r = ixheaac_add32_sat(x2r, x3i);
                x2i = ixheaac_sub32_sat(x2i, x3r);
                x3i = ixheaac_sub32_sat(x2r, ixheaac_shl32_sat(x3i, 1 as WORD));
                x3r = ixheaac_add32_sat(x2i, ixheaac_shl32_sat(x3r, 1 as WORD));
                *data = x0r;
                *data.offset(1 as core::ffi::c_int as isize) = x0i;
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                *data = x2r;
                *data.offset(1 as core::ffi::c_int as isize) = x2i;
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                *data = x1r;
                *data.offset(1 as core::ffi::c_int as isize) = x1i;
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                *data = x3i;
                *data.offset(1 as core::ffi::c_int as isize) = x3r;
                data = data.offset((del << 1 as core::ffi::c_int) as isize);
                k -= 1;
            }
            data = ptr_y.offset(2 as core::ffi::c_int as isize);
            sec_loop_cnt = nodespacing * del;
            sec_loop_cnt = (sec_loop_cnt as core::ffi::c_int / 4 as core::ffi::c_int
                + sec_loop_cnt as core::ffi::c_int / 8 as core::ffi::c_int
                - sec_loop_cnt as core::ffi::c_int / 16 as core::ffi::c_int
                + sec_loop_cnt as core::ffi::c_int / 32 as core::ffi::c_int
                - sec_loop_cnt as core::ffi::c_int / 64 as core::ffi::c_int
                + sec_loop_cnt as core::ffi::c_int / 128 as core::ffi::c_int
                - sec_loop_cnt as core::ffi::c_int / 256 as core::ffi::c_int) as WORD32;
            j = nodespacing;
            j = nodespacing;
            while j <= sec_loop_cnt {
                w1h = *twiddles.offset((2 as WORD32 * j) as isize);
                w1l = *twiddles
                    .offset((2 as WORD32 * j) as isize)
                    .offset(1 as core::ffi::c_int as isize);
                w2h = *twiddles
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize);
                w2l = *twiddles
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                    .offset(1 as core::ffi::c_int as isize);
                w3h = *twiddles
                    .offset((2 as WORD32 * j) as isize)
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize);
                w3l = *twiddles
                    .offset((2 as WORD32 * j) as isize)
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                    .offset(1 as core::ffi::c_int as isize);
                k = in_loop_cnt;
                while k != 0 as core::ffi::c_int {
                    let mut tmp: WORD32 = 0;
                    let mut x0r_0: WORD32 = 0;
                    let mut x0i_0: WORD32 = 0;
                    let mut x1r_0: WORD32 = 0;
                    let mut x1i_0: WORD32 = 0;
                    let mut x2r_0: WORD32 = 0;
                    let mut x2i_0: WORD32 = 0;
                    let mut x3r_0: WORD32 = 0;
                    let mut x3i_0: WORD32 = 0;
                    data = data.offset((del << 1 as core::ffi::c_int) as isize);
                    x1r_0 = *data;
                    x1i_0 = *data.offset(1 as core::ffi::c_int as isize);
                    data = data.offset((del << 1 as core::ffi::c_int) as isize);
                    x2r_0 = *data;
                    x2i_0 = *data.offset(1 as core::ffi::c_int as isize);
                    data = data.offset((del << 1 as core::ffi::c_int) as isize);
                    x3r_0 = *data;
                    x3i_0 = *data.offset(1 as core::ffi::c_int as isize);
                    data = data
                        .offset(
                            -((3 as WORD32 * (del << 1 as core::ffi::c_int)) as isize),
                        );
                    tmp = ixheaac_sub32_sat(
                        ixheaacd_mult32_sat(x1r_0, w1l),
                        ixheaacd_mult32_sat(x1i_0, w1h),
                    );
                    x1i_0 = ixheaacd_mac32_sat(
                        ixheaacd_mult32_sat(x1r_0, w1h),
                        x1i_0,
                        w1l,
                    );
                    x1r_0 = tmp;
                    tmp = ixheaac_sub32_sat(
                        ixheaacd_mult32_sat(x2r_0, w2l),
                        ixheaacd_mult32_sat(x2i_0, w2h),
                    );
                    x2i_0 = ixheaacd_mac32_sat(
                        ixheaacd_mult32_sat(x2r_0, w2h),
                        x2i_0,
                        w2l,
                    );
                    x2r_0 = tmp;
                    tmp = ixheaac_sub32_sat(
                        ixheaacd_mult32_sat(x3r_0, w3l),
                        ixheaacd_mult32_sat(x3i_0, w3h),
                    );
                    x3i_0 = ixheaacd_mac32_sat(
                        ixheaacd_mult32_sat(x3r_0, w3h),
                        x3i_0,
                        w3l,
                    );
                    x3r_0 = tmp;
                    x0r_0 = *data;
                    x0i_0 = *data.offset(1 as core::ffi::c_int as isize);
                    x0r_0 = ixheaac_add32_sat(x0r_0, x2r_0);
                    x0i_0 = ixheaac_add32_sat(x0i_0, x2i_0);
                    x2r_0 = ixheaac_sub32_sat(
                        x0r_0,
                        ixheaac_shl32_sat(x2r_0, 1 as WORD),
                    );
                    x2i_0 = ixheaac_sub32_sat(
                        x0i_0,
                        ixheaac_shl32_sat(x2i_0, 1 as WORD),
                    );
                    x1r_0 = ixheaac_add32_sat(x1r_0, x3r_0);
                    x1i_0 = ixheaac_add32_sat(x1i_0, x3i_0);
                    x3r_0 = ixheaac_sub32_sat(
                        x1r_0,
                        ixheaac_shl32_sat(x3r_0, 1 as WORD),
                    );
                    x3i_0 = ixheaac_sub32_sat(
                        x1i_0,
                        ixheaac_shl32_sat(x3i_0, 1 as WORD),
                    );
                    x0r_0 = ixheaac_add32_sat(x0r_0, x1r_0);
                    x0i_0 = ixheaac_add32_sat(x0i_0, x1i_0);
                    x1r_0 = ixheaac_sub32_sat(
                        x0r_0,
                        ixheaac_shl32_sat(x1r_0, 1 as WORD),
                    );
                    x1i_0 = ixheaac_sub32_sat(
                        x0i_0,
                        ixheaac_shl32_sat(x1i_0, 1 as WORD),
                    );
                    x2r_0 = ixheaac_add32_sat(x2r_0, x3i_0);
                    x2i_0 = ixheaac_sub32_sat(x2i_0, x3r_0);
                    x3i_0 = ixheaac_sub32_sat(
                        x2r_0,
                        ixheaac_shl32_sat(x3i_0, 1 as WORD),
                    );
                    x3r_0 = ixheaac_add32_sat(
                        x2i_0,
                        ixheaac_shl32_sat(x3r_0, 1 as WORD),
                    );
                    *data = x0r_0;
                    *data.offset(1 as core::ffi::c_int as isize) = x0i_0;
                    data = data.offset((del << 1 as core::ffi::c_int) as isize);
                    *data = x2r_0;
                    *data.offset(1 as core::ffi::c_int as isize) = x2i_0;
                    data = data.offset((del << 1 as core::ffi::c_int) as isize);
                    *data = x1r_0;
                    *data.offset(1 as core::ffi::c_int as isize) = x1i_0;
                    data = data.offset((del << 1 as core::ffi::c_int) as isize);
                    *data = x3i_0;
                    *data.offset(1 as core::ffi::c_int as isize) = x3r_0;
                    data = data.offset((del << 1 as core::ffi::c_int) as isize);
                    k -= 1;
                }
                data = data.offset(-((2 as WORD32 * npoints) as isize));
                data = data.offset(2 as core::ffi::c_int as isize);
                j += nodespacing;
            }
            while j <= nodespacing * del >> 1 as core::ffi::c_int {
                w1h = *twiddles.offset((2 as WORD32 * j) as isize);
                w2h = *twiddles
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize);
                w3h = *twiddles
                    .offset((2 as WORD32 * j) as isize)
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                    .offset(-(512 as core::ffi::c_int as isize));
                w1l = *twiddles
                    .offset((2 as WORD32 * j) as isize)
                    .offset(1 as core::ffi::c_int as isize);
                w2l = *twiddles
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                    .offset(1 as core::ffi::c_int as isize);
                w3l = *twiddles
                    .offset((2 as WORD32 * j) as isize)
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                    .offset(-(511 as core::ffi::c_int as isize));
                k = in_loop_cnt;
                while k != 0 as core::ffi::c_int {
                    let mut tmp_0: WORD32 = 0;
                    let mut x0r_1: WORD32 = 0;
                    let mut x0i_1: WORD32 = 0;
                    let mut x1r_1: WORD32 = 0;
                    let mut x1i_1: WORD32 = 0;
                    let mut x2r_1: WORD32 = 0;
                    let mut x2i_1: WORD32 = 0;
                    let mut x3r_1: WORD32 = 0;
                    let mut x3i_1: WORD32 = 0;
                    data = data.offset((del << 1 as core::ffi::c_int) as isize);
                    x1r_1 = *data;
                    x1i_1 = *data.offset(1 as core::ffi::c_int as isize);
                    data = data.offset((del << 1 as core::ffi::c_int) as isize);
                    x2r_1 = *data;
                    x2i_1 = *data.offset(1 as core::ffi::c_int as isize);
                    data = data.offset((del << 1 as core::ffi::c_int) as isize);
                    x3r_1 = *data;
                    x3i_1 = *data.offset(1 as core::ffi::c_int as isize);
                    data = data
                        .offset(
                            -((3 as WORD32 * (del << 1 as core::ffi::c_int)) as isize),
                        );
                    tmp_0 = ixheaac_sub32_sat(
                        ixheaacd_mult32_sat(x1r_1, w1l),
                        ixheaacd_mult32_sat(x1i_1, w1h),
                    );
                    x1i_1 = ixheaacd_mac32_sat(
                        ixheaacd_mult32_sat(x1r_1, w1h),
                        x1i_1,
                        w1l,
                    );
                    x1r_1 = tmp_0;
                    tmp_0 = ixheaac_sub32_sat(
                        ixheaacd_mult32_sat(x2r_1, w2l),
                        ixheaacd_mult32_sat(x2i_1, w2h),
                    );
                    x2i_1 = ixheaacd_mac32_sat(
                        ixheaacd_mult32_sat(x2r_1, w2h),
                        x2i_1,
                        w2l,
                    );
                    x2r_1 = tmp_0;
                    tmp_0 = ixheaac_add32_sat(
                        ixheaacd_mult32_sat(x3r_1, w3h),
                        ixheaacd_mult32_sat(x3i_1, w3l),
                    );
                    x3i_1 = ixheaac_sub32_sat(
                        ixheaacd_mult32_sat(x3i_1, w3h),
                        ixheaacd_mult32_sat(x3r_1, w3l),
                    );
                    x3r_1 = tmp_0;
                    x0r_1 = *data;
                    x0i_1 = *data.offset(1 as core::ffi::c_int as isize);
                    x0r_1 = ixheaac_add32_sat(x0r_1, x2r_1);
                    x0i_1 = ixheaac_add32_sat(x0i_1, x2i_1);
                    x2r_1 = ixheaac_sub32_sat(
                        x0r_1,
                        ixheaac_shl32_sat(x2r_1, 1 as WORD),
                    );
                    x2i_1 = ixheaac_sub32_sat(
                        x0i_1,
                        ixheaac_shl32_sat(x2i_1, 1 as WORD),
                    );
                    x1r_1 = ixheaac_add32_sat(x1r_1, x3r_1);
                    x1i_1 = ixheaac_add32_sat(x1i_1, x3i_1);
                    x3r_1 = ixheaac_sub32_sat(
                        x1r_1,
                        ixheaac_shl32_sat(x3r_1, 1 as WORD),
                    );
                    x3i_1 = ixheaac_sub32_sat(
                        x1i_1,
                        ixheaac_shl32_sat(x3i_1, 1 as WORD),
                    );
                    x0r_1 = ixheaac_add32_sat(x0r_1, x1r_1);
                    x0i_1 = ixheaac_add32_sat(x0i_1, x1i_1);
                    x1r_1 = ixheaac_sub32_sat(
                        x0r_1,
                        ixheaac_shl32_sat(x1r_1, 1 as WORD),
                    );
                    x1i_1 = ixheaac_sub32_sat(
                        x0i_1,
                        ixheaac_shl32_sat(x1i_1, 1 as WORD),
                    );
                    x2r_1 = ixheaac_add32_sat(x2r_1, x3i_1);
                    x2i_1 = ixheaac_sub32_sat(x2i_1, x3r_1);
                    x3i_1 = ixheaac_sub32_sat(
                        x2r_1,
                        ixheaac_shl32_sat(x3i_1, 1 as WORD),
                    );
                    x3r_1 = ixheaac_add32_sat(
                        x2i_1,
                        ixheaac_shl32_sat(x3r_1, 1 as WORD),
                    );
                    *data = x0r_1;
                    *data.offset(1 as core::ffi::c_int as isize) = x0i_1;
                    data = data.offset((del << 1 as core::ffi::c_int) as isize);
                    *data = x2r_1;
                    *data.offset(1 as core::ffi::c_int as isize) = x2i_1;
                    data = data.offset((del << 1 as core::ffi::c_int) as isize);
                    *data = x1r_1;
                    *data.offset(1 as core::ffi::c_int as isize) = x1i_1;
                    data = data.offset((del << 1 as core::ffi::c_int) as isize);
                    *data = x3i_1;
                    *data.offset(1 as core::ffi::c_int as isize) = x3r_1;
                    data = data.offset((del << 1 as core::ffi::c_int) as isize);
                    k -= 1;
                }
                data = data.offset(-((2 as WORD32 * npoints) as isize));
                data = data.offset(2 as core::ffi::c_int as isize);
                j += nodespacing;
            }
            while j <= sec_loop_cnt as core::ffi::c_int * 2 as core::ffi::c_int {
                w1h = *twiddles.offset((2 as WORD32 * j) as isize);
                w2h = *twiddles
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                    .offset(-(512 as core::ffi::c_int as isize));
                w3h = *twiddles
                    .offset((2 as WORD32 * j) as isize)
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                    .offset(-(512 as core::ffi::c_int as isize));
                w1l = *twiddles
                    .offset((2 as WORD32 * j) as isize)
                    .offset(1 as core::ffi::c_int as isize);
                w2l = *twiddles
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                    .offset(-(511 as core::ffi::c_int as isize));
                w3l = *twiddles
                    .offset((2 as WORD32 * j) as isize)
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                    .offset(-(511 as core::ffi::c_int as isize));
                k = in_loop_cnt;
                while k != 0 as core::ffi::c_int {
                    let mut tmp_1: WORD32 = 0;
                    let mut x0r_2: WORD32 = 0;
                    let mut x0i_2: WORD32 = 0;
                    let mut x1r_2: WORD32 = 0;
                    let mut x1i_2: WORD32 = 0;
                    let mut x2r_2: WORD32 = 0;
                    let mut x2i_2: WORD32 = 0;
                    let mut x3r_2: WORD32 = 0;
                    let mut x3i_2: WORD32 = 0;
                    data = data.offset((del << 1 as core::ffi::c_int) as isize);
                    x1r_2 = *data;
                    x1i_2 = *data.offset(1 as core::ffi::c_int as isize);
                    data = data.offset((del << 1 as core::ffi::c_int) as isize);
                    x2r_2 = *data;
                    x2i_2 = *data.offset(1 as core::ffi::c_int as isize);
                    data = data.offset((del << 1 as core::ffi::c_int) as isize);
                    x3r_2 = *data;
                    x3i_2 = *data.offset(1 as core::ffi::c_int as isize);
                    data = data
                        .offset(
                            -((3 as WORD32 * (del << 1 as core::ffi::c_int)) as isize),
                        );
                    tmp_1 = ixheaac_sub32_sat(
                        ixheaacd_mult32_sat(x1r_2, w1l),
                        ixheaacd_mult32_sat(x1i_2, w1h),
                    );
                    x1i_2 = ixheaacd_mac32_sat(
                        ixheaacd_mult32_sat(x1r_2, w1h),
                        x1i_2,
                        w1l,
                    );
                    x1r_2 = tmp_1;
                    tmp_1 = ixheaac_add32_sat(
                        ixheaacd_mult32_sat(x2r_2, w2h),
                        ixheaacd_mult32_sat(x2i_2, w2l),
                    );
                    x2i_2 = ixheaac_sub32_sat(
                        ixheaacd_mult32_sat(x2i_2, w2h),
                        ixheaacd_mult32_sat(x2r_2, w2l),
                    );
                    x2r_2 = tmp_1;
                    tmp_1 = ixheaac_add32_sat(
                        ixheaacd_mult32_sat(x3r_2, w3h),
                        ixheaacd_mult32_sat(x3i_2, w3l),
                    );
                    x3i_2 = ixheaac_sub32_sat(
                        ixheaacd_mult32_sat(x3i_2, w3h),
                        ixheaacd_mult32_sat(x3r_2, w3l),
                    );
                    x3r_2 = tmp_1;
                    x0r_2 = *data;
                    x0i_2 = *data.offset(1 as core::ffi::c_int as isize);
                    x0r_2 = ixheaac_add32_sat(x0r_2, x2r_2);
                    x0i_2 = ixheaac_add32_sat(x0i_2, x2i_2);
                    x2r_2 = ixheaac_sub32_sat(
                        x0r_2,
                        ixheaac_shl32_sat(x2r_2, 1 as WORD),
                    );
                    x2i_2 = ixheaac_sub32_sat(
                        x0i_2,
                        ixheaac_shl32_sat(x2i_2, 1 as WORD),
                    );
                    x1r_2 = ixheaac_add32_sat(x1r_2, x3r_2);
                    x1i_2 = ixheaac_add32_sat(x1i_2, x3i_2);
                    x3r_2 = ixheaac_sub32_sat(
                        x1r_2,
                        ixheaac_shl32_sat(x3r_2, 1 as WORD),
                    );
                    x3i_2 = ixheaac_sub32_sat(
                        x1i_2,
                        ixheaac_shl32_sat(x3i_2, 1 as WORD),
                    );
                    x0r_2 = ixheaac_add32_sat(x0r_2, x1r_2);
                    x0i_2 = ixheaac_add32_sat(x0i_2, x1i_2);
                    x1r_2 = ixheaac_sub32_sat(
                        x0r_2,
                        ixheaac_shl32_sat(x1r_2, 1 as WORD),
                    );
                    x1i_2 = ixheaac_sub32_sat(
                        x0i_2,
                        ixheaac_shl32_sat(x1i_2, 1 as WORD),
                    );
                    x2r_2 = ixheaac_add32_sat(x2r_2, x3i_2);
                    x2i_2 = ixheaac_sub32_sat(x2i_2, x3r_2);
                    x3i_2 = ixheaac_sub32_sat(
                        x2r_2,
                        ixheaac_shl32_sat(x3i_2, 1 as WORD),
                    );
                    x3r_2 = ixheaac_add32_sat(
                        x2i_2,
                        ixheaac_shl32_sat(x3r_2, 1 as WORD),
                    );
                    *data = x0r_2;
                    *data.offset(1 as core::ffi::c_int as isize) = x0i_2;
                    data = data.offset((del << 1 as core::ffi::c_int) as isize);
                    *data = x2r_2;
                    *data.offset(1 as core::ffi::c_int as isize) = x2i_2;
                    data = data.offset((del << 1 as core::ffi::c_int) as isize);
                    *data = x1r_2;
                    *data.offset(1 as core::ffi::c_int as isize) = x1i_2;
                    data = data.offset((del << 1 as core::ffi::c_int) as isize);
                    *data = x3i_2;
                    *data.offset(1 as core::ffi::c_int as isize) = x3r_2;
                    data = data.offset((del << 1 as core::ffi::c_int) as isize);
                    k -= 1;
                }
                data = data.offset(-((2 as WORD32 * npoints) as isize));
                data = data.offset(2 as core::ffi::c_int as isize);
                j += nodespacing;
            }
            while j < nodespacing * del {
                w1h = *twiddles.offset((2 as WORD32 * j) as isize);
                w2h = *twiddles
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                    .offset(-(512 as core::ffi::c_int as isize));
                w3h = *twiddles
                    .offset((2 as WORD32 * j) as isize)
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                    .offset(-(1024 as core::ffi::c_int as isize));
                w1l = *twiddles
                    .offset((2 as WORD32 * j) as isize)
                    .offset(1 as core::ffi::c_int as isize);
                w2l = *twiddles
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                    .offset(-(511 as core::ffi::c_int as isize));
                w3l = *twiddles
                    .offset((2 as WORD32 * j) as isize)
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                    .offset(-(1023 as core::ffi::c_int as isize));
                k = in_loop_cnt;
                while k != 0 as core::ffi::c_int {
                    let mut tmp_2: WORD32 = 0;
                    let mut x0r_3: WORD32 = 0;
                    let mut x0i_3: WORD32 = 0;
                    let mut x1r_3: WORD32 = 0;
                    let mut x1i_3: WORD32 = 0;
                    let mut x2r_3: WORD32 = 0;
                    let mut x2i_3: WORD32 = 0;
                    let mut x3r_3: WORD32 = 0;
                    let mut x3i_3: WORD32 = 0;
                    data = data.offset((del << 1 as core::ffi::c_int) as isize);
                    x1r_3 = *data;
                    x1i_3 = *data.offset(1 as core::ffi::c_int as isize);
                    data = data.offset((del << 1 as core::ffi::c_int) as isize);
                    x2r_3 = *data;
                    x2i_3 = *data.offset(1 as core::ffi::c_int as isize);
                    data = data.offset((del << 1 as core::ffi::c_int) as isize);
                    x3r_3 = *data;
                    x3i_3 = *data.offset(1 as core::ffi::c_int as isize);
                    data = data
                        .offset(
                            -((3 as WORD32 * (del << 1 as core::ffi::c_int)) as isize),
                        );
                    tmp_2 = ixheaac_sub32_sat(
                        ixheaacd_mult32_sat(x1r_3, w1l),
                        ixheaacd_mult32_sat(x1i_3, w1h),
                    );
                    x1i_3 = ixheaacd_mac32_sat(
                        ixheaacd_mult32_sat(x1r_3, w1h),
                        x1i_3,
                        w1l,
                    );
                    x1r_3 = tmp_2;
                    tmp_2 = ixheaac_add32_sat(
                        ixheaacd_mult32_sat(x2r_3, w2h),
                        ixheaacd_mult32_sat(x2i_3, w2l),
                    );
                    x2i_3 = ixheaac_sub32_sat(
                        ixheaacd_mult32_sat(x2i_3, w2h),
                        ixheaacd_mult32_sat(x2r_3, w2l),
                    );
                    x2r_3 = tmp_2;
                    tmp_2 = ixheaac_sub32_sat(
                        ixheaacd_mult32_sat(x3i_3, w3h),
                        ixheaacd_mult32_sat(x3r_3, w3l),
                    );
                    x3i_3 = ixheaacd_mac32_sat(
                        ixheaacd_mult32_sat(x3r_3, w3h),
                        x3i_3,
                        w3l,
                    );
                    x3r_3 = tmp_2;
                    x0r_3 = *data;
                    x0i_3 = *data.offset(1 as core::ffi::c_int as isize);
                    x0r_3 = ixheaac_add32_sat(x0r_3, x2r_3);
                    x0i_3 = ixheaac_add32_sat(x0i_3, x2i_3);
                    x2r_3 = ixheaac_sub32_sat(
                        x0r_3,
                        ixheaac_shl32_sat(x2r_3, 1 as WORD),
                    );
                    x2i_3 = ixheaac_sub32_sat(
                        x0i_3,
                        ixheaac_shl32_sat(x2i_3, 1 as WORD),
                    );
                    x1r_3 = ixheaac_add32_sat(x1r_3, x3r_3);
                    x1i_3 = ixheaac_sub32_sat(x1i_3, x3i_3);
                    x3r_3 = ixheaac_sub32_sat(
                        x1r_3,
                        ixheaac_shl32_sat(x3r_3, 1 as WORD),
                    );
                    x3i_3 = ixheaac_add32_sat(
                        x1i_3,
                        ixheaac_shl32_sat(x3i_3, 1 as WORD),
                    );
                    x0r_3 = ixheaac_add32_sat(x0r_3, x1r_3);
                    x0i_3 = ixheaac_add32_sat(x0i_3, x1i_3);
                    x1r_3 = ixheaac_sub32_sat(
                        x0r_3,
                        ixheaac_shl32_sat(x1r_3, 1 as WORD),
                    );
                    x1i_3 = ixheaac_sub32_sat(
                        x0i_3,
                        ixheaac_shl32_sat(x1i_3, 1 as WORD),
                    );
                    x2r_3 = ixheaac_add32_sat(x2r_3, x3i_3);
                    x2i_3 = ixheaac_sub32_sat(x2i_3, x3r_3);
                    x3i_3 = ixheaac_sub32_sat(
                        x2r_3,
                        ixheaac_shl32_sat(x3i_3, 1 as WORD),
                    );
                    x3r_3 = ixheaac_add32_sat(
                        x2i_3,
                        ixheaac_shl32_sat(x3r_3, 1 as WORD),
                    );
                    *data = x0r_3;
                    *data.offset(1 as core::ffi::c_int as isize) = x0i_3;
                    data = data.offset((del << 1 as core::ffi::c_int) as isize);
                    *data = x2r_3;
                    *data.offset(1 as core::ffi::c_int as isize) = x2i_3;
                    data = data.offset((del << 1 as core::ffi::c_int) as isize);
                    *data = x1r_3;
                    *data.offset(1 as core::ffi::c_int as isize) = x1i_3;
                    data = data.offset((del << 1 as core::ffi::c_int) as isize);
                    *data = x3i_3;
                    *data.offset(1 as core::ffi::c_int as isize) = x3r_3;
                    data = data.offset((del << 1 as core::ffi::c_int) as isize);
                    k -= 1;
                }
                data = data.offset(-((2 as WORD32 * npoints) as isize));
                data = data.offset(2 as core::ffi::c_int as isize);
                j += nodespacing;
            }
            nodespacing >>= 2 as core::ffi::c_int;
            del <<= 2 as core::ffi::c_int;
            in_loop_cnt >>= 2 as core::ffi::c_int;
            i -= 1;
        }
        if not_power_4 != 0 {
            let mut twiddles_0: *const WORD32 = ptr_w;
            nodespacing <<= 1 as core::ffi::c_int;
            shift += 1 as core::ffi::c_int;
            j = (del as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
            while j != 0 as core::ffi::c_int {
                let mut w1h_0: WORD32 = *twiddles_0;
                let mut w1l_0: WORD32 = *twiddles_0
                    .offset(1 as core::ffi::c_int as isize);
                let mut tmp_3: WORD32 = 0;
                twiddles_0 = twiddles_0
                    .offset(
                        (nodespacing as core::ffi::c_int * 2 as core::ffi::c_int)
                            as isize,
                    );
                x0r = *ptr_y;
                x0i = *ptr_y.offset(1 as core::ffi::c_int as isize);
                ptr_y = ptr_y.offset((del << 1 as core::ffi::c_int) as isize);
                x1r = *ptr_y;
                x1i = *ptr_y.offset(1 as core::ffi::c_int as isize);
                tmp_3 = ixheaac_sub32_sat(
                    ixheaacd_mult32_sat(x1r, w1l_0),
                    ixheaacd_mult32_sat(x1i, w1h_0),
                );
                x1i = ixheaacd_mac32_sat(ixheaacd_mult32_sat(x1r, w1h_0), x1i, w1l_0);
                x1r = tmp_3;
                *ptr_y = (x0r as core::ffi::c_int / 2 as core::ffi::c_int
                    - x1r as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
                *ptr_y.offset(1 as core::ffi::c_int as isize) = (x0i as core::ffi::c_int
                    / 2 as core::ffi::c_int
                    - x1i as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
                ptr_y = ptr_y.offset(-((del << 1 as core::ffi::c_int) as isize));
                *ptr_y = (x0r as core::ffi::c_int / 2 as core::ffi::c_int
                    + x1r as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
                *ptr_y.offset(1 as core::ffi::c_int as isize) = (x0i as core::ffi::c_int
                    / 2 as core::ffi::c_int
                    + x1i as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
                ptr_y = ptr_y.offset(2 as core::ffi::c_int as isize);
                j -= 1;
            }
            twiddles_0 = ptr_w;
            j = (del as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
            while j != 0 as core::ffi::c_int {
                let mut w1h_1: WORD32 = *twiddles_0;
                let mut w1l_1: WORD32 = *twiddles_0
                    .offset(1 as core::ffi::c_int as isize);
                let mut tmp_4: WORD32 = 0;
                twiddles_0 = twiddles_0
                    .offset(
                        (nodespacing as core::ffi::c_int * 2 as core::ffi::c_int)
                            as isize,
                    );
                x0r = *ptr_y;
                x0i = *ptr_y.offset(1 as core::ffi::c_int as isize);
                ptr_y = ptr_y.offset((del << 1 as core::ffi::c_int) as isize);
                x1r = *ptr_y;
                x1i = *ptr_y.offset(1 as core::ffi::c_int as isize);
                tmp_4 = ixheaac_add32_sat(
                    ixheaacd_mult32_sat(x1r, w1h_1),
                    ixheaacd_mult32_sat(x1i, w1l_1),
                );
                x1i = ixheaac_sub32_sat(
                    ixheaacd_mult32_sat(x1i, w1h_1),
                    ixheaacd_mult32_sat(x1r, w1l_1),
                );
                x1r = tmp_4;
                *ptr_y = (x0r as core::ffi::c_int / 2 as core::ffi::c_int
                    - x1r as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
                *ptr_y.offset(1 as core::ffi::c_int as isize) = (x0i as core::ffi::c_int
                    / 2 as core::ffi::c_int
                    - x1i as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
                ptr_y = ptr_y.offset(-((del << 1 as core::ffi::c_int) as isize));
                *ptr_y = (x0r as core::ffi::c_int / 2 as core::ffi::c_int
                    + x1r as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
                *ptr_y.offset(1 as core::ffi::c_int as isize) = (x0i as core::ffi::c_int
                    / 2 as core::ffi::c_int
                    + x1i as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
                ptr_y = ptr_y.offset(2 as core::ffi::c_int as isize);
                j -= 1;
            }
        }
    } else {
        ptr_w = ixheaacd_twiddle_table_fft_32x32.as_ptr();
        i = 0 as core::ffi::c_int as WORD32;
        while i < npoints {
            let mut inp_0: *mut WORD32 = ptr_x.as_mut_ptr();
            let mut val_0: core::ffi::c_uint = i as core::ffi::c_uint;
            val_0 = (val_0 & 0x33333333 as core::ffi::c_uint) << 2 as core::ffi::c_int
                | (val_0 & !(0x33333333 as core::ffi::c_int) as core::ffi::c_uint)
                    >> 2 as core::ffi::c_int;
            val_0 = (val_0 & 0xf0f0f0f as core::ffi::c_uint) << 4 as core::ffi::c_int
                | (val_0 & !(0xf0f0f0f as core::ffi::c_int) as core::ffi::c_uint)
                    >> 4 as core::ffi::c_int;
            val_0 = (val_0 & 0xff00ff as core::ffi::c_uint) << 8 as core::ffi::c_int
                | (val_0 & !(0xff00ff as core::ffi::c_int) as core::ffi::c_uint)
                    >> 8 as core::ffi::c_int;
            h2 = (val_0 >> dig_rev_shift) as WORD32;
            if not_power_4 != 0 {
                h2 += 1 as core::ffi::c_int;
                h2 &= !(1 as core::ffi::c_int);
            }
            inp_0 = inp_0.offset(h2 as isize);
            x0r = *inp_0;
            x0i = *inp_0.offset(1 as core::ffi::c_int as isize);
            inp_0 = inp_0.offset((npoints >> 1 as core::ffi::c_int) as isize);
            x1r = *inp_0;
            x1i = *inp_0.offset(1 as core::ffi::c_int as isize);
            inp_0 = inp_0.offset((npoints >> 1 as core::ffi::c_int) as isize);
            x2r = *inp_0;
            x2i = *inp_0.offset(1 as core::ffi::c_int as isize);
            inp_0 = inp_0.offset((npoints >> 1 as core::ffi::c_int) as isize);
            x3r = *inp_0;
            x3i = *inp_0.offset(1 as core::ffi::c_int as isize);
            x0r = ixheaac_add32_sat(x0r, x2r);
            x0i = ixheaac_add32_sat(x0i, x2i);
            x2r = ixheaac_sub32_sat(x0r, ixheaac_shl32_sat(x2r, 1 as WORD));
            x2i = ixheaac_sub32_sat(x0i, ixheaac_shl32_sat(x2i, 1 as WORD));
            x1r = ixheaac_add32_sat(x1r, x3r);
            x1i = ixheaac_add32_sat(x1i, x3i);
            x3r = ixheaac_sub32_sat(x1r, ixheaac_shl32_sat(x3r, 1 as WORD));
            x3i = ixheaac_sub32_sat(x1i, ixheaac_shl32_sat(x3i, 1 as WORD));
            x0r = ixheaac_add32_sat(x0r, x1r);
            x0i = ixheaac_add32_sat(x0i, x1i);
            x1r = ixheaac_sub32_sat(x0r, ixheaac_shl32_sat(x1r, 1 as WORD));
            x1i = ixheaac_sub32_sat(x0i, ixheaac_shl32_sat(x1i, 1 as WORD));
            x2r = ixheaac_sub32_sat(x2r, x3i);
            x2i = ixheaac_add32_sat(x2i, x3r);
            x3i = ixheaac_add32_sat(x2r, ixheaac_shl32_sat(x3i, 1 as WORD));
            x3r = ixheaac_sub32_sat(x2i, ixheaac_shl32_sat(x3r, 1 as WORD));
            let fresh14 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh14 = x0r;
            let fresh15 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh15 = x0i;
            let fresh16 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh16 = x2r;
            let fresh17 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh17 = x2i;
            let fresh18 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh18 = x1r;
            let fresh19 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh19 = x1i;
            let fresh20 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh20 = x3i;
            let fresh21 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh21 = x3r;
            i += 4 as core::ffi::c_int;
        }
        ptr_y = ptr_y.offset(-((2 as WORD32 * npoints) as isize));
        del = 4 as core::ffi::c_int as WORD32;
        nodespacing = 64 as core::ffi::c_int as WORD32;
        in_loop_cnt = npoints >> 4 as core::ffi::c_int;
        i = (n_stages as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        while i > 0 as core::ffi::c_int {
            let mut twiddles_1: *const WORD32 = ptr_w;
            let mut data_0: *mut WORD32 = ptr_y;
            let mut w1h_2: WORD32 = 0;
            let mut w2h_0: WORD32 = 0;
            let mut w3h_0: WORD32 = 0;
            let mut w1l_2: WORD32 = 0;
            let mut w2l_0: WORD32 = 0;
            let mut w3l_0: WORD32 = 0;
            let mut sec_loop_cnt_0: WORD32 = 0;
            k = in_loop_cnt;
            while k != 0 as core::ffi::c_int {
                x0r = *data_0;
                x0i = *data_0.offset(1 as core::ffi::c_int as isize);
                data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                x1r = *data_0;
                x1i = *data_0.offset(1 as core::ffi::c_int as isize);
                data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                x2r = *data_0;
                x2i = *data_0.offset(1 as core::ffi::c_int as isize);
                data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                x3r = *data_0;
                x3i = *data_0.offset(1 as core::ffi::c_int as isize);
                data_0 = data_0
                    .offset(-((3 as WORD32 * (del << 1 as core::ffi::c_int)) as isize));
                x0r = ixheaac_add32_sat(x0r, x2r);
                x0i = ixheaac_add32_sat(x0i, x2i);
                x2r = ixheaac_sub32_sat(x0r, ixheaac_shl32_sat(x2r, 1 as WORD));
                x2i = ixheaac_sub32_sat(x0i, ixheaac_shl32_sat(x2i, 1 as WORD));
                x1r = ixheaac_add32_sat(x1r, x3r);
                x1i = ixheaac_add32_sat(x1i, x3i);
                x3r = ixheaac_sub32_sat(x1r, ixheaac_shl32_sat(x3r, 1 as WORD));
                x3i = ixheaac_sub32_sat(x1i, ixheaac_shl32_sat(x3i, 1 as WORD));
                x0r = ixheaac_add32_sat(x0r, x1r);
                x0i = ixheaac_add32_sat(x0i, x1i);
                x1r = ixheaac_sub32_sat(x0r, ixheaac_shl32_sat(x1r, 1 as WORD));
                x1i = ixheaac_sub32_sat(x0i, ixheaac_shl32_sat(x1i, 1 as WORD));
                x2r = ixheaac_sub32_sat(x2r, x3i);
                x2i = ixheaac_add32_sat(x2i, x3r);
                x3i = ixheaac_add32_sat(x2r, ixheaac_shl32_sat(x3i, 1 as WORD));
                x3r = ixheaac_sub32_sat(x2i, ixheaac_shl32_sat(x3r, 1 as WORD));
                *data_0 = x0r;
                *data_0.offset(1 as core::ffi::c_int as isize) = x0i;
                data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                *data_0 = x2r;
                *data_0.offset(1 as core::ffi::c_int as isize) = x2i;
                data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                *data_0 = x1r;
                *data_0.offset(1 as core::ffi::c_int as isize) = x1i;
                data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                *data_0 = x3i;
                *data_0.offset(1 as core::ffi::c_int as isize) = x3r;
                data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                k -= 1;
            }
            data_0 = ptr_y.offset(2 as core::ffi::c_int as isize);
            sec_loop_cnt_0 = nodespacing * del;
            sec_loop_cnt_0 = (sec_loop_cnt_0 as core::ffi::c_int / 4 as core::ffi::c_int
                + sec_loop_cnt_0 as core::ffi::c_int / 8 as core::ffi::c_int
                - sec_loop_cnt_0 as core::ffi::c_int / 16 as core::ffi::c_int
                + sec_loop_cnt_0 as core::ffi::c_int / 32 as core::ffi::c_int
                - sec_loop_cnt_0 as core::ffi::c_int / 64 as core::ffi::c_int
                + sec_loop_cnt_0 as core::ffi::c_int / 128 as core::ffi::c_int
                - sec_loop_cnt_0 as core::ffi::c_int / 256 as core::ffi::c_int)
                as WORD32;
            j = nodespacing;
            j = nodespacing;
            while j <= sec_loop_cnt_0 {
                w1h_2 = *twiddles_1.offset((2 as WORD32 * j) as isize);
                w2h_0 = *twiddles_1
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize);
                w3h_0 = *twiddles_1
                    .offset((2 as WORD32 * j) as isize)
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize);
                w1l_2 = *twiddles_1
                    .offset((2 as WORD32 * j) as isize)
                    .offset(1 as core::ffi::c_int as isize);
                w2l_0 = *twiddles_1
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                    .offset(1 as core::ffi::c_int as isize);
                w3l_0 = *twiddles_1
                    .offset((2 as WORD32 * j) as isize)
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                    .offset(1 as core::ffi::c_int as isize);
                k = in_loop_cnt;
                while k != 0 as core::ffi::c_int {
                    let mut tmp_5: WORD32 = 0;
                    let mut x0r_4: WORD32 = 0;
                    let mut x0i_4: WORD32 = 0;
                    let mut x1r_4: WORD32 = 0;
                    let mut x1i_4: WORD32 = 0;
                    let mut x2r_4: WORD32 = 0;
                    let mut x2i_4: WORD32 = 0;
                    let mut x3r_4: WORD32 = 0;
                    let mut x3i_4: WORD32 = 0;
                    data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                    x1r_4 = *data_0;
                    x1i_4 = *data_0.offset(1 as core::ffi::c_int as isize);
                    data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                    x2r_4 = *data_0;
                    x2i_4 = *data_0.offset(1 as core::ffi::c_int as isize);
                    data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                    x3r_4 = *data_0;
                    x3i_4 = *data_0.offset(1 as core::ffi::c_int as isize);
                    data_0 = data_0
                        .offset(
                            -((3 as WORD32 * (del << 1 as core::ffi::c_int)) as isize),
                        );
                    tmp_5 = ixheaac_add32_sat(
                        ixheaacd_mult32_sat(x1r_4, w1l_2),
                        ixheaacd_mult32_sat(x1i_4, w1h_2),
                    );
                    x1i_4 = ixheaacd_mac32_sat(
                        -ixheaacd_mult32_sat(x1r_4, w1h_2),
                        x1i_4,
                        w1l_2,
                    );
                    x1r_4 = tmp_5;
                    tmp_5 = ixheaac_add32_sat(
                        ixheaacd_mult32_sat(x2r_4, w2l_0),
                        ixheaacd_mult32_sat(x2i_4, w2h_0),
                    );
                    x2i_4 = ixheaacd_mac32_sat(
                        -ixheaacd_mult32_sat(x2r_4, w2h_0),
                        x2i_4,
                        w2l_0,
                    );
                    x2r_4 = tmp_5;
                    tmp_5 = ixheaac_add32_sat(
                        ixheaacd_mult32_sat(x3r_4, w3l_0),
                        ixheaacd_mult32_sat(x3i_4, w3h_0),
                    );
                    x3i_4 = ixheaacd_mac32_sat(
                        -ixheaacd_mult32_sat(x3r_4, w3h_0),
                        x3i_4,
                        w3l_0,
                    );
                    x3r_4 = tmp_5;
                    x0r_4 = *data_0;
                    x0i_4 = *data_0.offset(1 as core::ffi::c_int as isize);
                    x0r_4 = ixheaac_add32_sat(x0r_4, x2r_4);
                    x0i_4 = ixheaac_add32_sat(x0i_4, x2i_4);
                    x2r_4 = ixheaac_sub32_sat(
                        x0r_4,
                        ixheaac_shl32_sat(x2r_4, 1 as WORD),
                    );
                    x2i_4 = ixheaac_sub32_sat(
                        x0i_4,
                        ixheaac_shl32_sat(x2i_4, 1 as WORD),
                    );
                    x1r_4 = ixheaac_add32_sat(x1r_4, x3r_4);
                    x1i_4 = ixheaac_add32_sat(x1i_4, x3i_4);
                    x3r_4 = ixheaac_sub32_sat(
                        x1r_4,
                        ixheaac_shl32_sat(x3r_4, 1 as WORD),
                    );
                    x3i_4 = ixheaac_sub32_sat(
                        x1i_4,
                        ixheaac_shl32_sat(x3i_4, 1 as WORD),
                    );
                    x0r_4 = ixheaac_add32_sat(x0r_4, x1r_4);
                    x0i_4 = ixheaac_add32_sat(x0i_4, x1i_4);
                    x1r_4 = ixheaac_sub32_sat(
                        x0r_4,
                        ixheaac_shl32_sat(x1r_4, 1 as WORD),
                    );
                    x1i_4 = ixheaac_sub32_sat(
                        x0i_4,
                        ixheaac_shl32_sat(x1i_4, 1 as WORD),
                    );
                    x2r_4 = ixheaac_sub32_sat(x2r_4, x3i_4);
                    x2i_4 = ixheaac_add32_sat(x2i_4, x3r_4);
                    x3i_4 = ixheaac_add32_sat(
                        x2r_4,
                        ixheaac_shl32_sat(x3i_4, 1 as WORD),
                    );
                    x3r_4 = ixheaac_sub32_sat(
                        x2i_4,
                        ixheaac_shl32_sat(x3r_4, 1 as WORD),
                    );
                    *data_0 = x0r_4;
                    *data_0.offset(1 as core::ffi::c_int as isize) = x0i_4;
                    data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                    *data_0 = x2r_4;
                    *data_0.offset(1 as core::ffi::c_int as isize) = x2i_4;
                    data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                    *data_0 = x1r_4;
                    *data_0.offset(1 as core::ffi::c_int as isize) = x1i_4;
                    data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                    *data_0 = x3i_4;
                    *data_0.offset(1 as core::ffi::c_int as isize) = x3r_4;
                    data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                    k -= 1;
                }
                data_0 = data_0.offset(-((2 as WORD32 * npoints) as isize));
                data_0 = data_0.offset(2 as core::ffi::c_int as isize);
                j += nodespacing;
            }
            while j <= nodespacing * del >> 1 as core::ffi::c_int {
                w1h_2 = *twiddles_1.offset((2 as WORD32 * j) as isize);
                w2h_0 = *twiddles_1
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize);
                w3h_0 = *twiddles_1
                    .offset((2 as WORD32 * j) as isize)
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                    .offset(-(512 as core::ffi::c_int as isize));
                w1l_2 = *twiddles_1
                    .offset((2 as WORD32 * j) as isize)
                    .offset(1 as core::ffi::c_int as isize);
                w2l_0 = *twiddles_1
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                    .offset(1 as core::ffi::c_int as isize);
                w3l_0 = *twiddles_1
                    .offset((2 as WORD32 * j) as isize)
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                    .offset(-(511 as core::ffi::c_int as isize));
                k = in_loop_cnt;
                while k != 0 as core::ffi::c_int {
                    let mut tmp_6: WORD32 = 0;
                    let mut x0r_5: WORD32 = 0;
                    let mut x0i_5: WORD32 = 0;
                    let mut x1r_5: WORD32 = 0;
                    let mut x1i_5: WORD32 = 0;
                    let mut x2r_5: WORD32 = 0;
                    let mut x2i_5: WORD32 = 0;
                    let mut x3r_5: WORD32 = 0;
                    let mut x3i_5: WORD32 = 0;
                    data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                    x1r_5 = *data_0;
                    x1i_5 = *data_0.offset(1 as core::ffi::c_int as isize);
                    data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                    x2r_5 = *data_0;
                    x2i_5 = *data_0.offset(1 as core::ffi::c_int as isize);
                    data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                    x3r_5 = *data_0;
                    x3i_5 = *data_0.offset(1 as core::ffi::c_int as isize);
                    data_0 = data_0
                        .offset(
                            -((3 as WORD32 * (del << 1 as core::ffi::c_int)) as isize),
                        );
                    tmp_6 = ixheaac_add32_sat(
                        ixheaacd_mult32_sat(x1r_5, w1l_2),
                        ixheaacd_mult32_sat(x1i_5, w1h_2),
                    );
                    x1i_5 = ixheaacd_mac32_sat(
                        -ixheaacd_mult32_sat(x1r_5, w1h_2),
                        x1i_5,
                        w1l_2,
                    );
                    x1r_5 = tmp_6;
                    tmp_6 = ixheaac_add32_sat(
                        ixheaacd_mult32_sat(x2r_5, w2l_0),
                        ixheaacd_mult32_sat(x2i_5, w2h_0),
                    );
                    x2i_5 = ixheaacd_mac32_sat(
                        -ixheaacd_mult32_sat(x2r_5, w2h_0),
                        x2i_5,
                        w2l_0,
                    );
                    x2r_5 = tmp_6;
                    tmp_6 = ixheaac_sub32_sat(
                        ixheaacd_mult32_sat(x3r_5, w3h_0),
                        ixheaacd_mult32_sat(x3i_5, w3l_0),
                    );
                    x3i_5 = ixheaac_add32_sat(
                        ixheaacd_mult32_sat(x3r_5, w3l_0),
                        ixheaacd_mult32_sat(x3i_5, w3h_0),
                    );
                    x3r_5 = tmp_6;
                    x0r_5 = *data_0;
                    x0i_5 = *data_0.offset(1 as core::ffi::c_int as isize);
                    x0r_5 = ixheaac_add32_sat(x0r_5, x2r_5);
                    x0i_5 = ixheaac_add32_sat(x0i_5, x2i_5);
                    x2r_5 = ixheaac_sub32_sat(
                        x0r_5,
                        ixheaac_shl32_sat(x2r_5, 1 as WORD),
                    );
                    x2i_5 = ixheaac_sub32_sat(
                        x0i_5,
                        ixheaac_shl32_sat(x2i_5, 1 as WORD),
                    );
                    x1r_5 = ixheaac_add32_sat(x1r_5, x3r_5);
                    x1i_5 = ixheaac_add32_sat(x1i_5, x3i_5);
                    x3r_5 = ixheaac_sub32_sat(
                        x1r_5,
                        ixheaac_shl32_sat(x3r_5, 1 as WORD),
                    );
                    x3i_5 = ixheaac_sub32_sat(
                        x1i_5,
                        ixheaac_shl32_sat(x3i_5, 1 as WORD),
                    );
                    x0r_5 = ixheaac_add32_sat(x0r_5, x1r_5);
                    x0i_5 = ixheaac_add32_sat(x0i_5, x1i_5);
                    x1r_5 = ixheaac_sub32_sat(
                        x0r_5,
                        ixheaac_shl32_sat(x1r_5, 1 as WORD),
                    );
                    x1i_5 = ixheaac_sub32_sat(
                        x0i_5,
                        ixheaac_shl32_sat(x1i_5, 1 as WORD),
                    );
                    x2r_5 = ixheaac_sub32_sat(x2r_5, x3i_5);
                    x2i_5 = ixheaac_add32_sat(x2i_5, x3r_5);
                    x3i_5 = ixheaac_add32_sat(
                        x2r_5,
                        ixheaac_shl32_sat(x3i_5, 1 as WORD),
                    );
                    x3r_5 = ixheaac_sub32_sat(
                        x2i_5,
                        ixheaac_shl32_sat(x3r_5, 1 as WORD),
                    );
                    *data_0 = x0r_5;
                    *data_0.offset(1 as core::ffi::c_int as isize) = x0i_5;
                    data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                    *data_0 = x2r_5;
                    *data_0.offset(1 as core::ffi::c_int as isize) = x2i_5;
                    data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                    *data_0 = x1r_5;
                    *data_0.offset(1 as core::ffi::c_int as isize) = x1i_5;
                    data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                    *data_0 = x3i_5;
                    *data_0.offset(1 as core::ffi::c_int as isize) = x3r_5;
                    data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                    k -= 1;
                }
                data_0 = data_0.offset(-((2 as WORD32 * npoints) as isize));
                data_0 = data_0.offset(2 as core::ffi::c_int as isize);
                j += nodespacing;
            }
            while j <= sec_loop_cnt_0 as core::ffi::c_int * 2 as core::ffi::c_int {
                w1h_2 = *twiddles_1.offset((2 as WORD32 * j) as isize);
                w2h_0 = *twiddles_1
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                    .offset(-(512 as core::ffi::c_int as isize));
                w3h_0 = *twiddles_1
                    .offset((2 as WORD32 * j) as isize)
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                    .offset(-(512 as core::ffi::c_int as isize));
                w1l_2 = *twiddles_1
                    .offset((2 as WORD32 * j) as isize)
                    .offset(1 as core::ffi::c_int as isize);
                w2l_0 = *twiddles_1
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                    .offset(-(511 as core::ffi::c_int as isize));
                w3l_0 = *twiddles_1
                    .offset((2 as WORD32 * j) as isize)
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                    .offset(-(511 as core::ffi::c_int as isize));
                k = in_loop_cnt;
                while k != 0 as core::ffi::c_int {
                    let mut tmp_7: WORD32 = 0;
                    let mut x0r_6: WORD32 = 0;
                    let mut x0i_6: WORD32 = 0;
                    let mut x1r_6: WORD32 = 0;
                    let mut x1i_6: WORD32 = 0;
                    let mut x2r_6: WORD32 = 0;
                    let mut x2i_6: WORD32 = 0;
                    let mut x3r_6: WORD32 = 0;
                    let mut x3i_6: WORD32 = 0;
                    data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                    x1r_6 = *data_0;
                    x1i_6 = *data_0.offset(1 as core::ffi::c_int as isize);
                    data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                    x2r_6 = *data_0;
                    x2i_6 = *data_0.offset(1 as core::ffi::c_int as isize);
                    data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                    x3r_6 = *data_0;
                    x3i_6 = *data_0.offset(1 as core::ffi::c_int as isize);
                    data_0 = data_0
                        .offset(
                            -((3 as WORD32 * (del << 1 as core::ffi::c_int)) as isize),
                        );
                    tmp_7 = ixheaac_add32_sat(
                        ixheaacd_mult32_sat(x1r_6, w1l_2),
                        ixheaacd_mult32_sat(x1i_6, w1h_2),
                    );
                    x1i_6 = ixheaacd_mac32_sat(
                        -ixheaacd_mult32_sat(x1r_6, w1h_2),
                        x1i_6,
                        w1l_2,
                    );
                    x1r_6 = tmp_7;
                    tmp_7 = ixheaac_sub32_sat(
                        ixheaacd_mult32_sat(x2r_6, w2h_0),
                        ixheaacd_mult32_sat(x2i_6, w2l_0),
                    );
                    x2i_6 = ixheaac_add32_sat(
                        ixheaacd_mult32_sat(x2r_6, w2l_0),
                        ixheaacd_mult32_sat(x2i_6, w2h_0),
                    );
                    x2r_6 = tmp_7;
                    tmp_7 = ixheaac_sub32_sat(
                        ixheaacd_mult32_sat(x3r_6, w3h_0),
                        ixheaacd_mult32_sat(x3i_6, w3l_0),
                    );
                    x3i_6 = ixheaac_add32_sat(
                        ixheaacd_mult32_sat(x3r_6, w3l_0),
                        ixheaacd_mult32_sat(x3i_6, w3h_0),
                    );
                    x3r_6 = tmp_7;
                    x0r_6 = *data_0;
                    x0i_6 = *data_0.offset(1 as core::ffi::c_int as isize);
                    x0r_6 = ixheaac_add32_sat(x0r_6, x2r_6);
                    x0i_6 = ixheaac_add32_sat(x0i_6, x2i_6);
                    x2r_6 = ixheaac_sub32_sat(
                        x0r_6,
                        ixheaac_shl32_sat(x2r_6, 1 as WORD),
                    );
                    x2i_6 = ixheaac_sub32_sat(
                        x0i_6,
                        ixheaac_shl32_sat(x2i_6, 1 as WORD),
                    );
                    x1r_6 = ixheaac_add32_sat(x1r_6, x3r_6);
                    x1i_6 = ixheaac_add32_sat(x1i_6, x3i_6);
                    x3r_6 = ixheaac_sub32_sat(
                        x1r_6,
                        ixheaac_shl32_sat(x3r_6, 1 as WORD),
                    );
                    x3i_6 = ixheaac_sub32_sat(
                        x1i_6,
                        ixheaac_shl32_sat(x3i_6, 1 as WORD),
                    );
                    x0r_6 = ixheaac_add32_sat(x0r_6, x1r_6);
                    x0i_6 = ixheaac_add32_sat(x0i_6, x1i_6);
                    x1r_6 = ixheaac_sub32_sat(
                        x0r_6,
                        ixheaac_shl32_sat(x1r_6, 1 as WORD),
                    );
                    x1i_6 = ixheaac_sub32_sat(
                        x0i_6,
                        ixheaac_shl32_sat(x1i_6, 1 as WORD),
                    );
                    x2r_6 = ixheaac_sub32_sat(x2r_6, x3i_6);
                    x2i_6 = ixheaac_add32_sat(x2i_6, x3r_6);
                    x3i_6 = ixheaac_add32_sat(
                        x2r_6,
                        ixheaac_shl32_sat(x3i_6, 1 as WORD),
                    );
                    x3r_6 = ixheaac_sub32_sat(
                        x2i_6,
                        ixheaac_shl32_sat(x3r_6, 1 as WORD),
                    );
                    *data_0 = x0r_6;
                    *data_0.offset(1 as core::ffi::c_int as isize) = x0i_6;
                    data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                    *data_0 = x2r_6;
                    *data_0.offset(1 as core::ffi::c_int as isize) = x2i_6;
                    data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                    *data_0 = x1r_6;
                    *data_0.offset(1 as core::ffi::c_int as isize) = x1i_6;
                    data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                    *data_0 = x3i_6;
                    *data_0.offset(1 as core::ffi::c_int as isize) = x3r_6;
                    data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                    k -= 1;
                }
                data_0 = data_0.offset(-((2 as WORD32 * npoints) as isize));
                data_0 = data_0.offset(2 as core::ffi::c_int as isize);
                j += nodespacing;
            }
            while j < nodespacing * del {
                w1h_2 = *twiddles_1.offset((2 as WORD32 * j) as isize);
                w2h_0 = *twiddles_1
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                    .offset(-(512 as core::ffi::c_int as isize));
                w3h_0 = *twiddles_1
                    .offset((2 as WORD32 * j) as isize)
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                    .offset(-(1024 as core::ffi::c_int as isize));
                w1l_2 = *twiddles_1
                    .offset((2 as WORD32 * j) as isize)
                    .offset(1 as core::ffi::c_int as isize);
                w2l_0 = *twiddles_1
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                    .offset(-(511 as core::ffi::c_int as isize));
                w3l_0 = *twiddles_1
                    .offset((2 as WORD32 * j) as isize)
                    .offset((2 as WORD32 * (j << 1 as core::ffi::c_int)) as isize)
                    .offset(-(1023 as core::ffi::c_int as isize));
                k = in_loop_cnt;
                while k != 0 as core::ffi::c_int {
                    let mut tmp_8: WORD32 = 0;
                    let mut x0r_7: WORD32 = 0;
                    let mut x0i_7: WORD32 = 0;
                    let mut x1r_7: WORD32 = 0;
                    let mut x1i_7: WORD32 = 0;
                    let mut x2r_7: WORD32 = 0;
                    let mut x2i_7: WORD32 = 0;
                    let mut x3r_7: WORD32 = 0;
                    let mut x3i_7: WORD32 = 0;
                    data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                    x1r_7 = *data_0;
                    x1i_7 = *data_0.offset(1 as core::ffi::c_int as isize);
                    data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                    x2r_7 = *data_0;
                    x2i_7 = *data_0.offset(1 as core::ffi::c_int as isize);
                    data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                    x3r_7 = *data_0;
                    x3i_7 = *data_0.offset(1 as core::ffi::c_int as isize);
                    data_0 = data_0
                        .offset(
                            -((3 as WORD32 * (del << 1 as core::ffi::c_int)) as isize),
                        );
                    tmp_8 = ixheaac_add32_sat(
                        ixheaacd_mult32_sat(x1r_7, w1l_2),
                        ixheaacd_mult32_sat(x1i_7, w1h_2),
                    );
                    x1i_7 = ixheaacd_mac32_sat(
                        -ixheaacd_mult32_sat(x1r_7, w1h_2),
                        x1i_7,
                        w1l_2,
                    );
                    x1r_7 = tmp_8;
                    tmp_8 = ixheaac_sub32_sat(
                        ixheaacd_mult32_sat(x2r_7, w2h_0),
                        ixheaacd_mult32_sat(x2i_7, w2l_0),
                    );
                    x2i_7 = ixheaac_add32_sat(
                        ixheaacd_mult32_sat(x2r_7, w2l_0),
                        ixheaacd_mult32_sat(x2i_7, w2h_0),
                    );
                    x2r_7 = tmp_8;
                    tmp_8 = -ixheaac_add32_sat(
                        ixheaacd_mult32_sat(x3r_7, w3l_0),
                        ixheaacd_mult32_sat(x3i_7, w3h_0),
                    );
                    x3i_7 = ixheaacd_mac32_sat(
                        -ixheaacd_mult32_sat(x3r_7, w3h_0),
                        x3i_7,
                        w3l_0,
                    );
                    x3r_7 = tmp_8;
                    x0r_7 = *data_0;
                    x0i_7 = *data_0.offset(1 as core::ffi::c_int as isize);
                    x0r_7 = ixheaac_add32_sat(x0r_7, x2r_7);
                    x0i_7 = ixheaac_add32_sat(x0i_7, x2i_7);
                    x2r_7 = ixheaac_sub32_sat(
                        x0r_7,
                        ixheaac_shl32_sat(x2r_7, 1 as WORD),
                    );
                    x2i_7 = ixheaac_sub32_sat(
                        x0i_7,
                        ixheaac_shl32_sat(x2i_7, 1 as WORD),
                    );
                    x1r_7 = ixheaac_add32_sat(x1r_7, x3r_7);
                    x1i_7 = ixheaac_sub32_sat(x1i_7, x3i_7);
                    x3r_7 = ixheaac_sub32_sat(
                        x1r_7,
                        ixheaac_shl32_sat(x3r_7, 1 as WORD),
                    );
                    x3i_7 = ixheaac_add32_sat(
                        x1i_7,
                        ixheaac_shl32_sat(x3i_7, 1 as WORD),
                    );
                    x0r_7 = ixheaac_add32_sat(x0r_7, x1r_7);
                    x0i_7 = ixheaac_add32_sat(x0i_7, x1i_7);
                    x1r_7 = ixheaac_sub32_sat(
                        x0r_7,
                        ixheaac_shl32_sat(x1r_7, 1 as WORD),
                    );
                    x1i_7 = ixheaac_sub32_sat(
                        x0i_7,
                        ixheaac_shl32_sat(x1i_7, 1 as WORD),
                    );
                    x2r_7 = ixheaac_sub32_sat(x2r_7, x3i_7);
                    x2i_7 = ixheaac_add32_sat(x2i_7, x3r_7);
                    x3i_7 = ixheaac_add32_sat(
                        x2r_7,
                        ixheaac_shl32_sat(x3i_7, 1 as WORD),
                    );
                    x3r_7 = ixheaac_sub32_sat(
                        x2i_7,
                        ixheaac_shl32_sat(x3r_7, 1 as WORD),
                    );
                    *data_0 = x0r_7;
                    *data_0.offset(1 as core::ffi::c_int as isize) = x0i_7;
                    data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                    *data_0 = x2r_7;
                    *data_0.offset(1 as core::ffi::c_int as isize) = x2i_7;
                    data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                    *data_0 = x1r_7;
                    *data_0.offset(1 as core::ffi::c_int as isize) = x1i_7;
                    data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                    *data_0 = x3i_7;
                    *data_0.offset(1 as core::ffi::c_int as isize) = x3r_7;
                    data_0 = data_0.offset((del << 1 as core::ffi::c_int) as isize);
                    k -= 1;
                }
                data_0 = data_0.offset(-((2 as WORD32 * npoints) as isize));
                data_0 = data_0.offset(2 as core::ffi::c_int as isize);
                j += nodespacing;
            }
            nodespacing >>= 2 as core::ffi::c_int;
            del <<= 2 as core::ffi::c_int;
            in_loop_cnt >>= 2 as core::ffi::c_int;
            i -= 1;
        }
        if not_power_4 != 0 {
            let mut twiddles_2: *const WORD32 = ptr_w;
            nodespacing <<= 1 as core::ffi::c_int;
            shift += 1 as core::ffi::c_int;
            j = (del as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
            while j != 0 as core::ffi::c_int {
                let mut w1h_3: WORD32 = *twiddles_2;
                let mut w1l_3: WORD32 = *twiddles_2
                    .offset(1 as core::ffi::c_int as isize);
                let mut tmp_9: WORD32 = 0;
                twiddles_2 = twiddles_2
                    .offset(
                        (nodespacing as core::ffi::c_int * 2 as core::ffi::c_int)
                            as isize,
                    );
                x0r = *ptr_y;
                x0i = *ptr_y.offset(1 as core::ffi::c_int as isize);
                ptr_y = ptr_y.offset((del << 1 as core::ffi::c_int) as isize);
                x1r = *ptr_y;
                x1i = *ptr_y.offset(1 as core::ffi::c_int as isize);
                tmp_9 = ixheaac_add32_sat(
                    ixheaacd_mult32_sat(x1r, w1l_3),
                    ixheaacd_mult32_sat(x1i, w1h_3),
                );
                x1i = ixheaacd_mac32_sat(-ixheaacd_mult32_sat(x1r, w1h_3), x1i, w1l_3);
                x1r = tmp_9;
                *ptr_y = (x0r as core::ffi::c_int / 2 as core::ffi::c_int
                    - x1r as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
                *ptr_y.offset(1 as core::ffi::c_int as isize) = (x0i as core::ffi::c_int
                    / 2 as core::ffi::c_int
                    - x1i as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
                ptr_y = ptr_y.offset(-((del << 1 as core::ffi::c_int) as isize));
                *ptr_y = (x0r as core::ffi::c_int / 2 as core::ffi::c_int
                    + x1r as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
                *ptr_y.offset(1 as core::ffi::c_int as isize) = (x0i as core::ffi::c_int
                    / 2 as core::ffi::c_int
                    + x1i as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
                ptr_y = ptr_y.offset(2 as core::ffi::c_int as isize);
                j -= 1;
            }
            twiddles_2 = ptr_w;
            j = (del as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
            while j != 0 as core::ffi::c_int {
                let mut w1h_4: WORD32 = *twiddles_2;
                let mut w1l_4: WORD32 = *twiddles_2
                    .offset(1 as core::ffi::c_int as isize);
                let mut tmp_10: WORD32 = 0;
                twiddles_2 = twiddles_2
                    .offset(
                        (nodespacing as core::ffi::c_int * 2 as core::ffi::c_int)
                            as isize,
                    );
                x0r = *ptr_y;
                x0i = *ptr_y.offset(1 as core::ffi::c_int as isize);
                ptr_y = ptr_y.offset((del << 1 as core::ffi::c_int) as isize);
                x1r = *ptr_y;
                x1i = *ptr_y.offset(1 as core::ffi::c_int as isize);
                tmp_10 = ixheaac_sub32_sat(
                    ixheaacd_mult32_sat(x1r, w1h_4),
                    ixheaacd_mult32_sat(x1i, w1l_4),
                );
                x1i = ixheaac_add32_sat(
                    ixheaacd_mult32_sat(x1r, w1l_4),
                    ixheaacd_mult32_sat(x1i, w1h_4),
                );
                x1r = tmp_10;
                *ptr_y = (x0r as core::ffi::c_int / 2 as core::ffi::c_int
                    - x1r as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
                *ptr_y.offset(1 as core::ffi::c_int as isize) = (x0i as core::ffi::c_int
                    / 2 as core::ffi::c_int
                    - x1i as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
                ptr_y = ptr_y.offset(-((del << 1 as core::ffi::c_int) as isize));
                *ptr_y = (x0r as core::ffi::c_int / 2 as core::ffi::c_int
                    + x1r as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
                *ptr_y.offset(1 as core::ffi::c_int as isize) = (x0i as core::ffi::c_int
                    / 2 as core::ffi::c_int
                    + x1i as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
                ptr_y = ptr_y.offset(2 as core::ffi::c_int as isize);
                j -= 1;
            }
        }
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < nlength {
        *xr.offset(i as isize) = y[(2 as WORD32 * i) as usize];
        *xi.offset(i as isize) = y[(2 as core::ffi::c_int * i as core::ffi::c_int
            + 1 as core::ffi::c_int) as usize];
        i += 1;
    }
    *preshift = shift - *preshift;
}
#[inline]
unsafe extern "C" fn ixheaacd_complex_3point_fft(
    mut inp: *mut WORD32,
    mut op: *mut WORD32,
    mut sign_dir: WORD32,
) {
    let mut add_r: WORD32 = 0;
    let mut sub_r: WORD32 = 0;
    let mut add_i: WORD32 = 0;
    let mut sub_i: WORD32 = 0;
    let mut temp_real: WORD32 = 0;
    let mut temp_imag: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut p1: WORD32 = 0;
    let mut p2: WORD32 = 0;
    let mut p3: WORD32 = 0;
    let mut p4: WORD32 = 0;
    let mut sinmu: WORD32 = 0;
    sinmu = -(1859775393 as WORD32) * sign_dir;
    temp_real = ixheaac_add32_sat(
        *inp.offset(0 as core::ffi::c_int as isize),
        *inp.offset(2 as core::ffi::c_int as isize),
    );
    temp_imag = ixheaac_add32_sat(
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
    p4 = add_i >> 1 as core::ffi::c_int;
    p2 = ixheaac_mult32_shl(sub_i, sinmu);
    p3 = ixheaac_mult32_shl(sub_r, sinmu);
    temp = ixheaac_sub32(*inp.offset(0 as core::ffi::c_int as isize), p1);
    *op.offset(0 as core::ffi::c_int as isize) = ixheaac_add32_sat(
        temp_real,
        *inp.offset(4 as core::ffi::c_int as isize),
    );
    *op.offset(1 as core::ffi::c_int as isize) = ixheaac_add32_sat(
        temp_imag,
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
pub unsafe extern "C" fn ixheaacd_complex_fft_p3(
    mut xr: *mut WORD32,
    mut xi: *mut WORD32,
    mut nlength: WORD32,
    mut fft_mode: WORD32,
    mut preshift: *mut WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut shift: WORD32 = 0 as WORD32;
    let mut xr_3: [WORD32; 384] = [0; 384];
    let mut xi_3: [WORD32; 384] = [0; 384];
    let mut x: [WORD32; 1024] = [0; 1024];
    let mut y: [WORD32; 1024] = [0; 1024];
    let mut cnfac: WORD32 = 0;
    let mut npts: WORD32 = 0;
    let mut mpass: WORD32 = nlength;
    let mut n: WORD32 = 0 as WORD32;
    let mut ptr_x: *mut WORD32 = x.as_mut_ptr();
    let mut ptr_y: *mut WORD32 = y.as_mut_ptr();
    cnfac = 0 as core::ffi::c_int as WORD32;
    while mpass as core::ffi::c_int % 3 as core::ffi::c_int == 0 as core::ffi::c_int {
        mpass /= 3 as core::ffi::c_int;
        cnfac += 1;
    }
    npts = mpass;
    i = 0 as core::ffi::c_int as WORD32;
    while i < 3 as WORD32 * cnfac {
        j = 0 as core::ffi::c_int as WORD32;
        while j < mpass {
            xr_3[j as usize] = *xr.offset((3 as WORD32 * j + i) as isize);
            xi_3[j as usize] = *xi.offset((3 as WORD32 * j + i) as isize);
            j += 1;
        }
        (Some(ixheaacd_complex_fft_p2.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(xr_3.as_mut_ptr(), xi_3.as_mut_ptr(), mpass, fft_mode, &mut shift);
        j = 0 as core::ffi::c_int as WORD32;
        while j < mpass {
            *xr.offset((3 as WORD32 * j + i) as isize) = xr_3[j as usize];
            *xi.offset((3 as WORD32 * j + i) as isize) = xi_3[j as usize];
            j += 1;
        }
        i += 1;
    }
    while npts >> 1 as core::ffi::c_int != 0 {
        n += 1;
        npts = npts >> 1 as core::ffi::c_int;
    }
    if n as core::ffi::c_int % 2 as core::ffi::c_int == 0 as core::ffi::c_int {
        shift = ((n as core::ffi::c_int + 4 as core::ffi::c_int) / 2 as core::ffi::c_int)
            as WORD32;
    } else {
        shift = ((n as core::ffi::c_int + 5 as core::ffi::c_int) / 2 as core::ffi::c_int)
            as WORD32;
    }
    *preshift = (shift as core::ffi::c_int - *preshift + 1 as core::ffi::c_int)
        as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < nlength {
        *ptr_x.offset((2 as WORD32 * i) as isize) = *xr.offset(i as isize)
            >> 1 as core::ffi::c_int;
        *ptr_x
            .offset(
                (2 as core::ffi::c_int * i as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            ) = *xi.offset(i as isize) >> 1 as core::ffi::c_int;
        i += 1;
    }
    let mut w1r: *const WORD32 = 0 as *const WORD32;
    let mut w1i: *const WORD32 = 0 as *const WORD32;
    let mut tmp: WORD32 = 0;
    w1r = ixheaacd_twiddle_table_3pr.as_ptr();
    w1i = ixheaacd_twiddle_table_3pi.as_ptr();
    if fft_mode < 0 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < nlength {
            w1r = w1r.offset(1);
            w1i = w1i.offset(1);
            tmp = ixheaac_sub32_sat(
                ixheaacd_mult32_sat(
                    *ptr_x
                        .offset(
                            (2 as core::ffi::c_int * i as core::ffi::c_int
                                + 2 as core::ffi::c_int) as isize,
                        ),
                    *w1r,
                ),
                ixheaacd_mult32_sat(
                    *ptr_x
                        .offset(
                            (2 as core::ffi::c_int * i as core::ffi::c_int
                                + 3 as core::ffi::c_int) as isize,
                        ),
                    *w1i,
                ),
            );
            *ptr_x
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 3 as core::ffi::c_int) as isize,
                ) = ixheaac_add32_sat(
                ixheaacd_mult32_sat(
                    *ptr_x
                        .offset(
                            (2 as core::ffi::c_int * i as core::ffi::c_int
                                + 2 as core::ffi::c_int) as isize,
                        ),
                    *w1i,
                ),
                ixheaacd_mult32_sat(
                    *ptr_x
                        .offset(
                            (2 as core::ffi::c_int * i as core::ffi::c_int
                                + 3 as core::ffi::c_int) as isize,
                        ),
                    *w1r,
                ),
            );
            *ptr_x
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 2 as core::ffi::c_int) as isize,
                ) = tmp;
            w1r = w1r.offset(1);
            w1i = w1i.offset(1);
            tmp = ixheaac_sub32_sat(
                ixheaacd_mult32_sat(
                    *ptr_x
                        .offset(
                            (2 as core::ffi::c_int * i as core::ffi::c_int
                                + 4 as core::ffi::c_int) as isize,
                        ),
                    *w1r,
                ),
                ixheaacd_mult32_sat(
                    *ptr_x
                        .offset(
                            (2 as core::ffi::c_int * i as core::ffi::c_int
                                + 5 as core::ffi::c_int) as isize,
                        ),
                    *w1i,
                ),
            );
            *ptr_x
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 5 as core::ffi::c_int) as isize,
                ) = ixheaac_add32_sat(
                ixheaacd_mult32_sat(
                    *ptr_x
                        .offset(
                            (2 as core::ffi::c_int * i as core::ffi::c_int
                                + 4 as core::ffi::c_int) as isize,
                        ),
                    *w1i,
                ),
                ixheaacd_mult32_sat(
                    *ptr_x
                        .offset(
                            (2 as core::ffi::c_int * i as core::ffi::c_int
                                + 5 as core::ffi::c_int) as isize,
                        ),
                    *w1r,
                ),
            );
            *ptr_x
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 4 as core::ffi::c_int) as isize,
                ) = tmp;
            w1r = w1r
                .offset(
                    (3 as core::ffi::c_int
                        * (128 as core::ffi::c_int / mpass as core::ffi::c_int
                            - 1 as core::ffi::c_int) + 1 as core::ffi::c_int) as isize,
                );
            w1i = w1i
                .offset(
                    (3 as core::ffi::c_int
                        * (128 as core::ffi::c_int / mpass as core::ffi::c_int
                            - 1 as core::ffi::c_int) + 1 as core::ffi::c_int) as isize,
                );
            i += 3 as core::ffi::c_int;
        }
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i < nlength {
            w1r = w1r.offset(1);
            w1i = w1i.offset(1);
            tmp = ixheaac_add32_sat(
                ixheaacd_mult32_sat(
                    *ptr_x
                        .offset(
                            (2 as core::ffi::c_int * i as core::ffi::c_int
                                + 2 as core::ffi::c_int) as isize,
                        ),
                    *w1r,
                ),
                ixheaacd_mult32_sat(
                    *ptr_x
                        .offset(
                            (2 as core::ffi::c_int * i as core::ffi::c_int
                                + 3 as core::ffi::c_int) as isize,
                        ),
                    *w1i,
                ),
            );
            *ptr_x
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 3 as core::ffi::c_int) as isize,
                ) = ixheaac_sub32_sat(
                ixheaacd_mult32_sat(
                    *ptr_x
                        .offset(
                            (2 as core::ffi::c_int * i as core::ffi::c_int
                                + 3 as core::ffi::c_int) as isize,
                        ),
                    *w1r,
                ),
                ixheaacd_mult32_sat(
                    *ptr_x
                        .offset(
                            (2 as core::ffi::c_int * i as core::ffi::c_int
                                + 2 as core::ffi::c_int) as isize,
                        ),
                    *w1i,
                ),
            );
            *ptr_x
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 2 as core::ffi::c_int) as isize,
                ) = tmp;
            w1r = w1r.offset(1);
            w1i = w1i.offset(1);
            tmp = ixheaac_add32_sat(
                ixheaacd_mult32_sat(
                    *ptr_x
                        .offset(
                            (2 as core::ffi::c_int * i as core::ffi::c_int
                                + 4 as core::ffi::c_int) as isize,
                        ),
                    *w1r,
                ),
                ixheaacd_mult32_sat(
                    *ptr_x
                        .offset(
                            (2 as core::ffi::c_int * i as core::ffi::c_int
                                + 5 as core::ffi::c_int) as isize,
                        ),
                    *w1i,
                ),
            );
            *ptr_x
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 5 as core::ffi::c_int) as isize,
                ) = ixheaac_sub32_sat(
                ixheaacd_mult32_sat(
                    *ptr_x
                        .offset(
                            (2 as core::ffi::c_int * i as core::ffi::c_int
                                + 5 as core::ffi::c_int) as isize,
                        ),
                    *w1r,
                ),
                ixheaacd_mult32_sat(
                    *ptr_x
                        .offset(
                            (2 as core::ffi::c_int * i as core::ffi::c_int
                                + 4 as core::ffi::c_int) as isize,
                        ),
                    *w1i,
                ),
            );
            *ptr_x
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 4 as core::ffi::c_int) as isize,
                ) = tmp;
            w1r = w1r
                .offset(
                    (3 as core::ffi::c_int
                        * (128 as core::ffi::c_int / mpass as core::ffi::c_int
                            - 1 as core::ffi::c_int) + 1 as core::ffi::c_int) as isize,
                );
            w1i = w1i
                .offset(
                    (3 as core::ffi::c_int
                        * (128 as core::ffi::c_int / mpass as core::ffi::c_int
                            - 1 as core::ffi::c_int) + 1 as core::ffi::c_int) as isize,
                );
            i += 3 as core::ffi::c_int;
        }
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < mpass {
        ixheaacd_complex_3point_fft(ptr_x, ptr_y, fft_mode);
        ptr_x = ptr_x.offset(6 as core::ffi::c_int as isize);
        ptr_y = ptr_y.offset(6 as core::ffi::c_int as isize);
        i += 1;
    }
    ptr_y = y.as_mut_ptr();
    i = 0 as core::ffi::c_int as WORD32;
    while i < mpass {
        let fresh0 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *xr.offset(i as isize) = *fresh0;
        let fresh1 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *xi.offset(i as isize) = *fresh1;
        let fresh2 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *xr.offset((mpass + i) as isize) = *fresh2;
        let fresh3 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *xi.offset((mpass + i) as isize) = *fresh3;
        let fresh4 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *xr.offset((2 as WORD32 * mpass + i) as isize) = *fresh4;
        let fresh5 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *xi.offset((2 as WORD32 * mpass + i) as isize) = *fresh5;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_complex_fft(
    mut data_r: *mut WORD32,
    mut data_i: *mut WORD32,
    mut nlength: WORD32,
    mut fft_mode: WORD32,
    mut preshift: *mut WORD32,
) -> VOID {
    if nlength as core::ffi::c_int & nlength as core::ffi::c_int - 1 as core::ffi::c_int
        != 0
    {
        ixheaacd_complex_fft_p3(data_r, data_i, nlength, fft_mode, preshift);
    } else {
        (Some(ixheaacd_complex_fft_p2.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(data_r, data_i, nlength, fft_mode, preshift);
    };
}
