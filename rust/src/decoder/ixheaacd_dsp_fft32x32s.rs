pub type WORD16 = core::ffi::c_short;
pub type WORD32 = core::ffi::c_int;
pub type WORD64 = core::ffi::c_longlong;
pub type VOID = ();
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
unsafe extern "C" fn ixheaac_mult32x16in32_shl(mut a: WORD32, mut b: WORD16) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 16 as core::ffi::c_int) as WORD32;
    return result << 1 as core::ffi::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_inv_dit_fft_8pt_dec(
    mut y: *mut WORD32,
    mut real: *mut WORD32,
    mut imag: *mut WORD32,
) -> VOID {
    let mut a0: WORD32 = 0;
    let mut a1: WORD32 = 0;
    let mut a2: WORD32 = 0;
    let mut a3: WORD32 = 0;
    let mut a00: WORD32 = 0;
    let mut a10: WORD32 = 0;
    let mut a20: WORD32 = 0;
    let mut a30: WORD32 = 0;
    let mut vr: WORD32 = 0;
    let mut vi: WORD32 = 0;
    let mut x: [WORD32; 16] = [0; 16];
    a00 = ixheaac_add32_sat(
        *y.offset(0 as core::ffi::c_int as isize),
        *y.offset(8 as core::ffi::c_int as isize),
    );
    a0 = ixheaac_sub32_sat(
        *y.offset(0 as core::ffi::c_int as isize),
        *y.offset(8 as core::ffi::c_int as isize),
    );
    a20 = ixheaac_add32_sat(
        *y.offset(1 as core::ffi::c_int as isize),
        *y.offset(9 as core::ffi::c_int as isize),
    );
    a3 = ixheaac_sub32_sat(
        *y.offset(1 as core::ffi::c_int as isize),
        *y.offset(9 as core::ffi::c_int as isize),
    );
    a10 = ixheaac_add32_sat(
        *y.offset(4 as core::ffi::c_int as isize),
        *y.offset(12 as core::ffi::c_int as isize),
    );
    a2 = ixheaac_sub32_sat(
        *y.offset(4 as core::ffi::c_int as isize),
        *y.offset(12 as core::ffi::c_int as isize),
    );
    a30 = ixheaac_add32_sat(
        *y.offset(5 as core::ffi::c_int as isize),
        *y.offset(13 as core::ffi::c_int as isize),
    );
    a1 = ixheaac_sub32_sat(
        *y.offset(5 as core::ffi::c_int as isize),
        *y.offset(13 as core::ffi::c_int as isize),
    );
    x[0 as core::ffi::c_int as usize] = ixheaac_add32_sat(a00, a10);
    x[4 as core::ffi::c_int as usize] = ixheaac_sub32_sat(a00, a10);
    x[1 as core::ffi::c_int as usize] = ixheaac_add32_sat(a20, a30);
    x[5 as core::ffi::c_int as usize] = ixheaac_sub32_sat(a20, a30);
    x[2 as core::ffi::c_int as usize] = ixheaac_sub32_sat(a0, a1);
    x[6 as core::ffi::c_int as usize] = ixheaac_add32_sat(a0, a1);
    x[3 as core::ffi::c_int as usize] = ixheaac_add32_sat(a3, a2);
    x[7 as core::ffi::c_int as usize] = ixheaac_sub32_sat(a3, a2);
    a00 = ixheaac_add32_sat(
        *y.offset(2 as core::ffi::c_int as isize),
        *y.offset(10 as core::ffi::c_int as isize),
    );
    a0 = ixheaac_sub32_sat(
        *y.offset(2 as core::ffi::c_int as isize),
        *y.offset(10 as core::ffi::c_int as isize),
    );
    a20 = ixheaac_add32_sat(
        *y.offset(3 as core::ffi::c_int as isize),
        *y.offset(11 as core::ffi::c_int as isize),
    );
    a3 = ixheaac_sub32_sat(
        *y.offset(3 as core::ffi::c_int as isize),
        *y.offset(11 as core::ffi::c_int as isize),
    );
    a10 = ixheaac_add32_sat(
        *y.offset(6 as core::ffi::c_int as isize),
        *y.offset(14 as core::ffi::c_int as isize),
    );
    a2 = ixheaac_sub32_sat(
        *y.offset(6 as core::ffi::c_int as isize),
        *y.offset(14 as core::ffi::c_int as isize),
    );
    a30 = ixheaac_add32_sat(
        *y.offset(7 as core::ffi::c_int as isize),
        *y.offset(15 as core::ffi::c_int as isize),
    );
    a1 = ixheaac_sub32_sat(
        *y.offset(7 as core::ffi::c_int as isize),
        *y.offset(15 as core::ffi::c_int as isize),
    );
    x[8 as core::ffi::c_int as usize] = ixheaac_add32_sat(a00, a10);
    x[12 as core::ffi::c_int as usize] = ixheaac_sub32_sat(a00, a10);
    x[9 as core::ffi::c_int as usize] = ixheaac_add32_sat(a20, a30);
    x[13 as core::ffi::c_int as usize] = ixheaac_sub32_sat(a20, a30);
    x[10 as core::ffi::c_int as usize] = ixheaac_sub32_sat(a0, a1);
    x[14 as core::ffi::c_int as usize] = ixheaac_add32_sat(a0, a1);
    x[11 as core::ffi::c_int as usize] = ixheaac_add32_sat(a3, a2);
    x[15 as core::ffi::c_int as usize] = ixheaac_sub32_sat(a3, a2);
    *real.offset(0 as core::ffi::c_int as isize) = ixheaac_add32_sat(
        x[0 as core::ffi::c_int as usize],
        x[8 as core::ffi::c_int as usize],
    );
    *imag.offset(0 as core::ffi::c_int as isize) = ixheaac_add32_sat(
        x[1 as core::ffi::c_int as usize],
        x[9 as core::ffi::c_int as usize],
    );
    a00 = ixheaac_sub32_sat(
        x[0 as core::ffi::c_int as usize],
        x[8 as core::ffi::c_int as usize],
    );
    a10 = ixheaac_sub32_sat(
        x[1 as core::ffi::c_int as usize],
        x[9 as core::ffi::c_int as usize],
    );
    a0 = ixheaac_sub32_sat(
        x[4 as core::ffi::c_int as usize],
        x[13 as core::ffi::c_int as usize],
    );
    a1 = ixheaac_add32_sat(
        x[5 as core::ffi::c_int as usize],
        x[12 as core::ffi::c_int as usize],
    );
    *real.offset(4 as core::ffi::c_int as isize) = ixheaac_add32_sat(
        x[4 as core::ffi::c_int as usize],
        x[13 as core::ffi::c_int as usize],
    );
    *imag.offset(4 as core::ffi::c_int as isize) = ixheaac_sub32_sat(
        x[5 as core::ffi::c_int as usize],
        x[12 as core::ffi::c_int as usize],
    );
    vr = ixheaac_mult32x16in32_shl_sat(
        ixheaac_sub32_sat(
            x[10 as core::ffi::c_int as usize],
            x[11 as core::ffi::c_int as usize],
        ),
        0x5a82 as WORD16,
    );
    vi = ixheaac_mult32x16in32_shl_sat(
        ixheaac_add32_sat(
            x[10 as core::ffi::c_int as usize],
            x[11 as core::ffi::c_int as usize],
        ),
        0x5a82 as WORD16,
    );
    *real.offset(1 as core::ffi::c_int as isize) = ixheaac_add32_sat(
        x[2 as core::ffi::c_int as usize],
        vr,
    );
    *imag.offset(1 as core::ffi::c_int as isize) = ixheaac_add32_sat(
        x[3 as core::ffi::c_int as usize],
        vi,
    );
    a2 = ixheaac_sub32_sat(x[2 as core::ffi::c_int as usize], vr);
    a3 = ixheaac_sub32_sat(x[3 as core::ffi::c_int as usize], vi);
    *real.offset(2 as core::ffi::c_int as isize) = ixheaac_add32_sat(a0, a2);
    *imag.offset(2 as core::ffi::c_int as isize) = ixheaac_add32_sat(a1, a3);
    vr = ixheaac_mult32x16in32_shl_sat(
        ixheaac_add32_sat(
            x[14 as core::ffi::c_int as usize],
            x[15 as core::ffi::c_int as usize],
        ),
        0x5a82 as WORD16,
    );
    vi = ixheaac_mult32x16in32_shl_sat(
        ixheaac_sub32_sat(
            x[14 as core::ffi::c_int as usize],
            x[15 as core::ffi::c_int as usize],
        ),
        0x5a82 as WORD16,
    );
    a20 = ixheaac_sub32_sat(x[6 as core::ffi::c_int as usize], vr);
    a30 = ixheaac_add32_sat(x[7 as core::ffi::c_int as usize], vi);
    *real.offset(3 as core::ffi::c_int as isize) = ixheaac_add32_sat(a00, a20);
    *imag.offset(3 as core::ffi::c_int as isize) = ixheaac_add32_sat(a10, a30);
    *real.offset(5 as core::ffi::c_int as isize) = ixheaac_add32_sat(
        x[6 as core::ffi::c_int as usize],
        vr,
    );
    *imag.offset(5 as core::ffi::c_int as isize) = ixheaac_sub32_sat(
        x[7 as core::ffi::c_int as usize],
        vi,
    );
}
