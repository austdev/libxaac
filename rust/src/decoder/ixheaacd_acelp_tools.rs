extern "C" {
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
}
pub type size_t = usize;
pub type WORD8 = core::ffi::c_schar;
pub type WORD16 = core::ffi::c_short;
pub type WORD32 = core::ffi::c_int;
pub type WORD64 = core::ffi::c_longlong;
pub type FLOAT32 = core::ffi::c_float;
pub type VOID = ();
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
#[no_mangle]
pub static mut ixheaacd_gamma_table: [FLOAT32; 17] = [
    1.0f32,
    0.92f32,
    0.8464f32,
    0.778688f32,
    0.716393f32,
    0.659082f32,
    0.606355f32,
    0.557847f32,
    0.513219f32,
    0.472161f32,
    0.434389f32,
    0.399637f32,
    0.367666f32,
    0.338253f32,
    0.311193f32,
    0.286298f32,
    0.263394f32,
];
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_rand_gen(mut seed: *mut WORD16) -> WORD16 {
    *seed = (*seed as core::ffi::c_long * 31821 as core::ffi::c_long
        + 13849 as core::ffi::c_long) as WORD16;
    return *seed;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_preemphsis_tool(
    mut signal: *mut WORD32,
    mut mu: WORD32,
    mut len: WORD32,
    mut mem: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    i = (len as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while i > 0 as core::ffi::c_int {
        *signal.offset(i as isize)
            -= ixheaac_mul32_sh(
                mu,
                *signal.offset((i as core::ffi::c_int - 1 as core::ffi::c_int) as isize),
                16 as WORD8,
            );
        i -= 1;
    }
    *signal.offset(0 as core::ffi::c_int as isize)
        -= ixheaac_mul32_sh(mu, mem, 16 as WORD8);
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_preemphsis_tool_float(
    mut signal: *mut FLOAT32,
    mut mu: FLOAT32,
    mut len: WORD32,
    mut mem: FLOAT32,
) -> VOID {
    let mut i: WORD32 = 0;
    i = (len as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while i > 0 as core::ffi::c_int {
        *signal.offset(i as isize) = *signal.offset(i as isize)
            - mu
                * *signal
                    .offset((i as core::ffi::c_int - 1 as core::ffi::c_int) as isize);
        i -= 1;
    }
    *signal.offset(0 as core::ffi::c_int as isize) -= mu * mem;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_deemphsis_tool(
    mut signal: *mut FLOAT32,
    mut len: WORD32,
    mut mem: FLOAT32,
) -> VOID {
    let mut i: WORD32 = 0;
    *signal.offset(0 as core::ffi::c_int as isize) = *signal
        .offset(0 as core::ffi::c_int as isize) + PREEMPH_FILT_FAC * mem;
    i = 1 as core::ffi::c_int as WORD32;
    while i < len {
        *signal.offset(i as isize) = *signal.offset(i as isize)
            + PREEMPH_FILT_FAC
                * *signal
                    .offset((i as core::ffi::c_int - 1 as core::ffi::c_int) as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_lpc_wt_synthesis_tool(
    mut a: *mut FLOAT32,
    mut x: *mut FLOAT32,
    mut l: WORD32,
) -> VOID {
    let mut s: FLOAT32 = 0.;
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < l {
        s = *x.offset(i as isize);
        j = 1 as core::ffi::c_int as WORD32;
        while j <= ORDER {
            s
                -= *a.offset(j as isize) * ixheaacd_gamma_table[j as usize]
                    * *x.offset((i - j) as isize);
            s
                -= *a.offset((j as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                    * ixheaacd_gamma_table[(j as core::ffi::c_int
                        + 1 as core::ffi::c_int) as usize]
                    * *x
                        .offset(
                            (i as core::ffi::c_int
                                - (j as core::ffi::c_int + 1 as core::ffi::c_int)) as isize,
                        );
            s
                -= *a.offset((j as core::ffi::c_int + 2 as core::ffi::c_int) as isize)
                    * ixheaacd_gamma_table[(j as core::ffi::c_int
                        + 2 as core::ffi::c_int) as usize]
                    * *x
                        .offset(
                            (i as core::ffi::c_int
                                - (j as core::ffi::c_int + 2 as core::ffi::c_int)) as isize,
                        );
            s
                -= *a.offset((j as core::ffi::c_int + 3 as core::ffi::c_int) as isize)
                    * ixheaacd_gamma_table[(j as core::ffi::c_int
                        + 3 as core::ffi::c_int) as usize]
                    * *x
                        .offset(
                            (i as core::ffi::c_int
                                - (j as core::ffi::c_int + 3 as core::ffi::c_int)) as isize,
                        );
            j += 4 as core::ffi::c_int;
        }
        *x.offset(i as isize) = s;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_synthesis_tool_float(
    mut a: *mut FLOAT32,
    mut x: *mut FLOAT32,
    mut y: *mut FLOAT32,
    mut l: WORD32,
    mut mem: *mut FLOAT32,
) -> VOID {
    let mut buf: [FLOAT32; 512] = [0.; 512];
    let mut s: FLOAT32 = 0.;
    let mut yy: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    memcpy(
        buf.as_mut_ptr() as *mut core::ffi::c_void,
        mem as *const core::ffi::c_void,
        (ORDER as size_t).wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
    );
    yy = &mut *buf.as_mut_ptr().offset(ORDER as isize) as *mut FLOAT32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < l {
        s = *x.offset(i as isize);
        j = 1 as core::ffi::c_int as WORD32;
        while j <= ORDER {
            s -= *a.offset(j as isize) * *yy.offset((i - j) as isize);
            s
                -= *a.offset((j as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                    * *yy
                        .offset(
                            (i as core::ffi::c_int
                                - (j as core::ffi::c_int + 1 as core::ffi::c_int)) as isize,
                        );
            s
                -= *a.offset((j as core::ffi::c_int + 2 as core::ffi::c_int) as isize)
                    * *yy
                        .offset(
                            (i as core::ffi::c_int
                                - (j as core::ffi::c_int + 2 as core::ffi::c_int)) as isize,
                        );
            s
                -= *a.offset((j as core::ffi::c_int + 3 as core::ffi::c_int) as isize)
                    * *yy
                        .offset(
                            (i as core::ffi::c_int
                                - (j as core::ffi::c_int + 3 as core::ffi::c_int)) as isize,
                        );
            j += 4 as core::ffi::c_int;
        }
        *yy.offset(i as isize) = s;
        *y.offset(i as isize) = s;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_synthesis_tool_float1(
    mut a: *mut FLOAT32,
    mut x: *mut FLOAT32,
    mut l: WORD32,
) -> VOID {
    let mut s: FLOAT32 = 0.;
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < l {
        s = *x.offset(i as isize);
        j = 1 as core::ffi::c_int as WORD32;
        while j <= ORDER {
            s -= *a.offset(j as isize) * *x.offset((i - j) as isize);
            s
                -= *a.offset((j as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int
                                - (j as core::ffi::c_int + 1 as core::ffi::c_int)) as isize,
                        );
            s
                -= *a.offset((j as core::ffi::c_int + 2 as core::ffi::c_int) as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int
                                - (j as core::ffi::c_int + 2 as core::ffi::c_int)) as isize,
                        );
            s
                -= *a.offset((j as core::ffi::c_int + 3 as core::ffi::c_int) as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int
                                - (j as core::ffi::c_int + 3 as core::ffi::c_int)) as isize,
                        );
            j += 4 as core::ffi::c_int;
        }
        *x.offset(i as isize) = s;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_residual_tool(
    mut a: *mut WORD32,
    mut x: *mut WORD32,
    mut y: *mut WORD32,
    mut l: WORD32,
    mut count: WORD32,
) -> VOID {
    let mut s: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut n: WORD32 = l * count;
    i = 0 as core::ffi::c_int as WORD32;
    while i < n {
        s = *x.offset(i as isize);
        j = 1 as core::ffi::c_int as WORD32;
        while j <= 16 as core::ffi::c_int {
            s
                += ixheaac_mul32_sh(
                    *a.offset(j as isize),
                    *x.offset((i - j) as isize),
                    24 as WORD8,
                );
            j += 1;
        }
        *y.offset(i as isize) = s;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_residual_tool_float(
    mut a: *mut FLOAT32,
    mut x: *mut FLOAT32,
    mut y: *mut FLOAT32,
    mut l: WORD32,
    mut loop_count: WORD32,
) -> VOID {
    let mut s: FLOAT32 = 0.;
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    j = 0 as core::ffi::c_int as WORD32;
    while j < loop_count {
        i = 0 as core::ffi::c_int as WORD32;
        while i < l {
            s = *x.offset(i as isize);
            s
                += *a.offset(1 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(2 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 2 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(3 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 3 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(4 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 4 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(5 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 5 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(6 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 6 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(7 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 7 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(8 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 8 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(9 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 9 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(10 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 10 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(11 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 11 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(12 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 12 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(13 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 13 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(14 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 14 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(15 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 15 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(16 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 16 as core::ffi::c_int) as isize,
                        );
            *y.offset(i as isize) = s;
            i += 1;
        }
        a = a.offset(17 as core::ffi::c_int as isize);
        x = x.offset(l as isize);
        y = y.offset(l as isize);
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_residual_tool_float1(
    mut a: *mut FLOAT32,
    mut x: *mut FLOAT32,
    mut y: *mut FLOAT32,
    mut l: WORD32,
    mut loop_count: WORD32,
) -> VOID {
    let mut s: FLOAT32 = 0.;
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    j = 0 as core::ffi::c_int as WORD32;
    while j < loop_count {
        i = 0 as core::ffi::c_int as WORD32;
        while i < l {
            s = *x.offset(i as isize);
            s
                += *a.offset(1 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(2 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 2 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(3 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 3 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(4 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 4 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(5 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 5 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(6 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 6 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(7 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 7 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(8 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 8 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(9 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 9 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(10 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 10 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(11 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 11 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(12 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 12 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(13 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 13 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(14 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 14 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(15 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 15 as core::ffi::c_int) as isize,
                        );
            s
                += *a.offset(16 as core::ffi::c_int as isize)
                    * *x
                        .offset(
                            (i as core::ffi::c_int - 16 as core::ffi::c_int) as isize,
                        );
            *y.offset(i as isize) = s;
            i += 1;
        }
        x = x.offset(l as isize);
        y = y.offset(l as isize);
        j += 1;
    }
}
pub const ORDER: core::ffi::c_int = 16 as core::ffi::c_int;
pub const PREEMPH_FILT_FAC: core::ffi::c_float = 0.68f32;
