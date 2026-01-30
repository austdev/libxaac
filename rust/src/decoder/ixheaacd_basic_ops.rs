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
}
pub type size_t = usize;
pub type WORD8 = core::ffi::c_schar;
pub type WORD32 = core::ffi::c_int;
pub type WORD64 = core::ffi::c_longlong;
pub type FLOAT32 = core::ffi::c_float;
pub type VOID = ();
pub type WORD = core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct offset_lengths {
    pub lfac: WORD32,
    pub n_flat_ls: WORD32,
    pub n_trans_ls: WORD32,
    pub n_long: WORD32,
    pub n_short: WORD32,
}
pub const MAX_32: WORD32 = 0x7fffffff as core::ffi::c_long as WORD32;
pub const MIN_32: WORD32 = 0x80000000 as core::ffi::c_long as WORD32;
pub const ADJ_SCALE: core::ffi::c_int = 11 as core::ffi::c_int;
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
unsafe extern "C" fn ixheaacd_mult32_sh1(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 31 as core::ffi::c_int) as WORD32;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_memset(mut x: *mut FLOAT32, mut n: WORD32) -> VOID {
    memset(
        x as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (n as size_t).wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mem_cpy(
    mut x: *const FLOAT32,
    mut y: *mut FLOAT32,
    mut n: WORD32,
) -> VOID {
    memcpy(
        y as *mut core::ffi::c_void,
        x as *const core::ffi::c_void,
        (n as size_t).wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_vec_cnst_mul(
    mut a: FLOAT32,
    mut x: *mut FLOAT32,
    mut z: *mut FLOAT32,
    mut n: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < n {
        *z.offset(i as isize) = a * *x.offset(i as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_combine_fac(
    mut src1: *mut WORD32,
    mut src2: *mut WORD32,
    mut dest: *mut WORD32,
    mut len: WORD32,
    mut output_q: WORD8,
    mut fac_q: WORD8,
) -> VOID {
    let mut i: WORD32 = 0;
    if fac_q as core::ffi::c_int > output_q as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < len {
            *dest = ixheaac_add32_sat(
                *src1,
                *src2 >> fac_q as core::ffi::c_int - output_q as core::ffi::c_int,
            );
            dest = dest.offset(1);
            src1 = src1.offset(1);
            src2 = src2.offset(1);
            i += 1;
        }
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i < len {
            *dest = ixheaac_add32_sat(
                *src1,
                ixheaac_shl32_sat(*src2, output_q as WORD - fac_q as WORD),
            );
            dest = dest.offset(1);
            src1 = src1.offset(1);
            src2 = src2.offset(1);
            i += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_windowing_long1(
    mut src1: *mut WORD32,
    mut src2: *mut WORD32,
    mut win_fwd: *const WORD32,
    mut win_rev: *const WORD32,
    mut dest: *mut WORD32,
    mut vlen: WORD32,
    mut shift1: WORD8,
    mut shift2: WORD8,
) -> WORD8 {
    let mut i: WORD32 = 0;
    let mut rsrc2: *mut WORD32 = src2
        .offset(vlen as isize)
        .offset(-(1 as core::ffi::c_int as isize));
    if shift1 as core::ffi::c_int > shift2 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < vlen as core::ffi::c_int / 2 as core::ffi::c_int {
            *dest = ixheaac_add32_sat(
                ixheaacd_mult32_sh1(*src1, *win_fwd)
                    >> shift1 as core::ffi::c_int - shift2 as core::ffi::c_int,
                ixheaacd_mult32_sh1(*src2, *win_rev),
            );
            *dest
                .offset((vlen - 2 as WORD32 * i) as isize)
                .offset(-(1 as core::ffi::c_int as isize)) = ixheaac_add32_sat(
                ixheaacd_mult32_sh1(ixheaac_negate32_sat(*src1), *win_rev)
                    >> shift1 as core::ffi::c_int - shift2 as core::ffi::c_int,
                ixheaacd_mult32_sh1(*rsrc2, *win_fwd),
            );
            src1 = src1.offset(1);
            src2 = src2.offset(1);
            win_fwd = win_fwd.offset(1);
            win_rev = win_rev.offset(-1);
            rsrc2 = rsrc2.offset(-1);
            dest = dest.offset(1);
            i += 1;
        }
        return shift2;
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i < vlen as core::ffi::c_int / 2 as core::ffi::c_int {
            *dest = ixheaac_add32_sat(
                ixheaacd_mult32_sh1(*src1, *win_fwd),
                ixheaacd_mult32_sh1(*src2, *win_rev)
                    >> shift2 as core::ffi::c_int - shift1 as core::ffi::c_int,
            );
            *dest
                .offset((vlen - 2 as WORD32 * i) as isize)
                .offset(-(1 as core::ffi::c_int as isize)) = ixheaac_add32_sat(
                ixheaacd_mult32_sh1(ixheaac_negate32_sat(*src1), *win_rev),
                ixheaacd_mult32_sh1(*rsrc2, *win_fwd)
                    >> shift2 as core::ffi::c_int - shift1 as core::ffi::c_int,
            );
            src1 = src1.offset(1);
            src2 = src2.offset(1);
            win_fwd = win_fwd.offset(1);
            win_rev = win_rev.offset(-1);
            rsrc2 = rsrc2.offset(-1);
            dest = dest.offset(1);
            i += 1;
        }
        return shift1;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_windowing_long2(
    mut src1: *mut WORD32,
    mut win_fwd: *const WORD32,
    mut fac_data_out: *mut WORD32,
    mut over_lap: *mut WORD32,
    mut p_out_buffer: *mut WORD32,
    mut ixheaacd_drc_offset: *mut offset_lengths,
    mut shiftp: WORD8,
    mut shift_olap: WORD8,
    mut fac_q: WORD8,
) -> WORD8 {
    let mut i: WORD32 = 0;
    let mut dest: *mut WORD32 = p_out_buffer;
    win_fwd = win_fwd.offset((*ixheaacd_drc_offset).lfac as isize);
    if shiftp as core::ffi::c_int > fac_q as core::ffi::c_int {
        if shift_olap as core::ffi::c_int > fac_q as core::ffi::c_int {
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*ixheaacd_drc_offset).n_flat_ls + (*ixheaacd_drc_offset).lfac {
                *dest.offset(i as isize) = *over_lap.offset(i as isize)
                    >> shift_olap as core::ffi::c_int - fac_q as core::ffi::c_int;
                i += 1;
            }
            i = (*ixheaacd_drc_offset).n_flat_ls + (*ixheaacd_drc_offset).lfac;
            while i
                < (*ixheaacd_drc_offset).n_flat_ls + (*ixheaacd_drc_offset).n_trans_ls
            {
                *dest.offset(i as isize) = ixheaac_add32_sat(
                    ixheaacd_mult32_sh1(
                        ixheaac_negate32_sat(
                            *src1
                                .offset(
                                    ((*ixheaacd_drc_offset).n_long as core::ffi::c_int
                                        / 2 as core::ffi::c_int
                                        + (*ixheaacd_drc_offset).n_flat_ls as core::ffi::c_int
                                        + (*ixheaacd_drc_offset).lfac as core::ffi::c_int
                                        - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                                ),
                        ),
                        *win_fwd,
                    ) >> shiftp as core::ffi::c_int - fac_q as core::ffi::c_int,
                    *fac_data_out,
                );
                win_fwd = win_fwd.offset(1);
                fac_data_out = fac_data_out.offset(1);
                i += 1;
            }
            while i
                < (*ixheaacd_drc_offset).n_flat_ls as core::ffi::c_int
                    + (*ixheaacd_drc_offset).lfac as core::ffi::c_int
                        * 3 as core::ffi::c_int
            {
                *dest.offset(i as isize) = ixheaac_add32_sat(
                    ixheaac_negate32_sat(
                        *src1
                            .offset(
                                ((*ixheaacd_drc_offset).n_long as core::ffi::c_int
                                    / 2 as core::ffi::c_int
                                    + (*ixheaacd_drc_offset).n_flat_ls as core::ffi::c_int
                                    + (*ixheaacd_drc_offset).lfac as core::ffi::c_int
                                    - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                            ),
                    ) >> shiftp as core::ffi::c_int - fac_q as core::ffi::c_int,
                    *fac_data_out,
                );
                fac_data_out = fac_data_out.offset(1);
                i += 1;
            }
            while i < (*ixheaacd_drc_offset).n_long {
                *dest.offset(i as isize) = ixheaac_negate32_sat(
                    *src1
                        .offset(
                            ((*ixheaacd_drc_offset).n_long as core::ffi::c_int
                                / 2 as core::ffi::c_int
                                + (*ixheaacd_drc_offset).n_flat_ls as core::ffi::c_int
                                + (*ixheaacd_drc_offset).lfac as core::ffi::c_int
                                - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                        ),
                ) >> shiftp as core::ffi::c_int - fac_q as core::ffi::c_int;
                i += 1;
            }
            return fac_q;
        } else {
            memcpy(
                dest as *mut core::ffi::c_void,
                over_lap as *const core::ffi::c_void,
                (::core::mem::size_of::<WORD32>() as size_t)
                    .wrapping_mul(
                        ((*ixheaacd_drc_offset).n_flat_ls + (*ixheaacd_drc_offset).lfac)
                            as size_t,
                    ),
            );
            i = (*ixheaacd_drc_offset).n_flat_ls + (*ixheaacd_drc_offset).lfac;
            while i
                < (*ixheaacd_drc_offset).n_flat_ls + (*ixheaacd_drc_offset).n_trans_ls
            {
                *dest.offset(i as isize) = ixheaac_add32_sat(
                    ixheaacd_mult32_sh1(
                        ixheaac_negate32_sat(
                            *src1
                                .offset(
                                    ((*ixheaacd_drc_offset).n_long as core::ffi::c_int
                                        / 2 as core::ffi::c_int
                                        + (*ixheaacd_drc_offset).n_flat_ls as core::ffi::c_int
                                        + (*ixheaacd_drc_offset).lfac as core::ffi::c_int
                                        - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                                ),
                        ),
                        *win_fwd,
                    ) >> shiftp as core::ffi::c_int - shift_olap as core::ffi::c_int,
                    *fac_data_out
                        >> fac_q as core::ffi::c_int - shift_olap as core::ffi::c_int,
                );
                win_fwd = win_fwd.offset(1);
                fac_data_out = fac_data_out.offset(1);
                i += 1;
            }
            while i
                < (*ixheaacd_drc_offset).n_flat_ls as core::ffi::c_int
                    + (*ixheaacd_drc_offset).lfac as core::ffi::c_int
                        * 3 as core::ffi::c_int
            {
                *dest.offset(i as isize) = ixheaac_add32_sat(
                    ixheaac_negate32_sat(
                        *src1
                            .offset(
                                ((*ixheaacd_drc_offset).n_long as core::ffi::c_int
                                    / 2 as core::ffi::c_int
                                    + (*ixheaacd_drc_offset).n_flat_ls as core::ffi::c_int
                                    + (*ixheaacd_drc_offset).lfac as core::ffi::c_int
                                    - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                            ),
                    ) >> shiftp as core::ffi::c_int - shift_olap as core::ffi::c_int,
                    *fac_data_out
                        >> fac_q as core::ffi::c_int - shift_olap as core::ffi::c_int,
                );
                fac_data_out = fac_data_out.offset(1);
                i += 1;
            }
            while i < (*ixheaacd_drc_offset).n_long {
                *dest.offset(i as isize) = ixheaac_negate32_sat(
                    *src1
                        .offset(
                            ((*ixheaacd_drc_offset).n_long as core::ffi::c_int
                                / 2 as core::ffi::c_int
                                + (*ixheaacd_drc_offset).n_flat_ls as core::ffi::c_int
                                + (*ixheaacd_drc_offset).lfac as core::ffi::c_int
                                - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                        ),
                ) >> shiftp as core::ffi::c_int - shift_olap as core::ffi::c_int;
                i += 1;
            }
            return shift_olap;
        }
    } else if shift_olap as core::ffi::c_int > shiftp as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*ixheaacd_drc_offset).n_flat_ls + (*ixheaacd_drc_offset).lfac {
            *dest.offset(i as isize) = *over_lap.offset(i as isize)
                >> shift_olap as core::ffi::c_int - shiftp as core::ffi::c_int;
            i += 1;
        }
        i = (*ixheaacd_drc_offset).n_flat_ls + (*ixheaacd_drc_offset).lfac;
        while i < (*ixheaacd_drc_offset).n_flat_ls + (*ixheaacd_drc_offset).n_trans_ls {
            *dest.offset(i as isize) = ixheaac_add32_sat(
                ixheaacd_mult32_sh1(
                    ixheaac_negate32_sat(
                        *src1
                            .offset(
                                ((*ixheaacd_drc_offset).n_long as core::ffi::c_int
                                    / 2 as core::ffi::c_int
                                    + (*ixheaacd_drc_offset).n_flat_ls as core::ffi::c_int
                                    + (*ixheaacd_drc_offset).lfac as core::ffi::c_int
                                    - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                            ),
                    ),
                    *win_fwd,
                ),
                *fac_data_out >> fac_q as core::ffi::c_int - shiftp as core::ffi::c_int,
            );
            win_fwd = win_fwd.offset(1);
            fac_data_out = fac_data_out.offset(1);
            i += 1;
        }
        while i
            < (*ixheaacd_drc_offset).n_flat_ls as core::ffi::c_int
                + (*ixheaacd_drc_offset).lfac as core::ffi::c_int * 3 as core::ffi::c_int
        {
            *dest.offset(i as isize) = ixheaac_add32_sat(
                ixheaac_negate32_sat(
                    *src1
                        .offset(
                            ((*ixheaacd_drc_offset).n_long as core::ffi::c_int
                                / 2 as core::ffi::c_int
                                + (*ixheaacd_drc_offset).n_flat_ls as core::ffi::c_int
                                + (*ixheaacd_drc_offset).lfac as core::ffi::c_int
                                - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                        ),
                ),
                *fac_data_out >> fac_q as core::ffi::c_int - shiftp as core::ffi::c_int,
            );
            fac_data_out = fac_data_out.offset(1);
            i += 1;
        }
        while i < (*ixheaacd_drc_offset).n_long {
            *dest.offset(i as isize) = ixheaac_negate32_sat(
                *src1
                    .offset(
                        ((*ixheaacd_drc_offset).n_long as core::ffi::c_int
                            / 2 as core::ffi::c_int
                            + (*ixheaacd_drc_offset).n_flat_ls as core::ffi::c_int
                            + (*ixheaacd_drc_offset).lfac as core::ffi::c_int
                            - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                    ),
            );
            i += 1;
        }
        return shiftp;
    } else {
        memcpy(
            dest as *mut core::ffi::c_void,
            over_lap as *const core::ffi::c_void,
            (::core::mem::size_of::<WORD32>() as size_t)
                .wrapping_mul(
                    ((*ixheaacd_drc_offset).n_flat_ls + (*ixheaacd_drc_offset).lfac)
                        as size_t,
                ),
        );
        i = (*ixheaacd_drc_offset).n_flat_ls + (*ixheaacd_drc_offset).lfac;
        while i < (*ixheaacd_drc_offset).n_flat_ls + (*ixheaacd_drc_offset).n_trans_ls {
            *dest.offset(i as isize) = ixheaac_add32_sat(
                ixheaacd_mult32_sh1(
                    ixheaac_negate32_sat(
                        *src1
                            .offset(
                                ((*ixheaacd_drc_offset).n_long as core::ffi::c_int
                                    / 2 as core::ffi::c_int
                                    + (*ixheaacd_drc_offset).n_flat_ls as core::ffi::c_int
                                    + (*ixheaacd_drc_offset).lfac as core::ffi::c_int
                                    - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                            ),
                    ),
                    *win_fwd,
                ) >> shiftp as core::ffi::c_int - shift_olap as core::ffi::c_int,
                *fac_data_out
                    >> fac_q as core::ffi::c_int - shift_olap as core::ffi::c_int,
            );
            win_fwd = win_fwd.offset(1);
            fac_data_out = fac_data_out.offset(1);
            i += 1;
        }
        while i
            < (*ixheaacd_drc_offset).n_flat_ls as core::ffi::c_int
                + (*ixheaacd_drc_offset).lfac as core::ffi::c_int * 3 as core::ffi::c_int
        {
            *dest.offset(i as isize) = ixheaac_add32_sat(
                ixheaac_negate32_sat(
                    *src1
                        .offset(
                            ((*ixheaacd_drc_offset).n_long as core::ffi::c_int
                                / 2 as core::ffi::c_int
                                + (*ixheaacd_drc_offset).n_flat_ls as core::ffi::c_int
                                + (*ixheaacd_drc_offset).lfac as core::ffi::c_int
                                - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                        ),
                ) >> shiftp as core::ffi::c_int - shift_olap as core::ffi::c_int,
                *fac_data_out
                    >> fac_q as core::ffi::c_int - shift_olap as core::ffi::c_int,
            );
            fac_data_out = fac_data_out.offset(1);
            i += 1;
        }
        while i < (*ixheaacd_drc_offset).n_long {
            *dest.offset(i as isize) = ixheaac_negate32_sat(
                *src1
                    .offset(
                        ((*ixheaacd_drc_offset).n_long as core::ffi::c_int
                            / 2 as core::ffi::c_int
                            + (*ixheaacd_drc_offset).n_flat_ls as core::ffi::c_int
                            + (*ixheaacd_drc_offset).lfac as core::ffi::c_int
                            - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                    ),
            ) >> shiftp as core::ffi::c_int - shift_olap as core::ffi::c_int;
            i += 1;
        }
        return shift_olap;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_windowing_long3(
    mut src1: *mut WORD32,
    mut win_fwd: *const WORD32,
    mut over_lap: *mut WORD32,
    mut p_out_buffer: *mut WORD32,
    mut win_rev: *const WORD32,
    mut ixheaacd_drc_offset: *mut offset_lengths,
    mut shiftp: WORD8,
    mut shift_olap: WORD8,
) -> WORD8 {
    let mut i: WORD32 = 0;
    let mut dest: *mut WORD32 = p_out_buffer;
    if shiftp as core::ffi::c_int > shift_olap as core::ffi::c_int {
        memcpy(
            dest as *mut core::ffi::c_void,
            over_lap as *const core::ffi::c_void,
            (::core::mem::size_of::<FLOAT32>() as size_t)
                .wrapping_mul((*ixheaacd_drc_offset).n_flat_ls as size_t),
        );
        i = (*ixheaacd_drc_offset).n_flat_ls;
        while i
            < (*ixheaacd_drc_offset).n_long as core::ffi::c_int / 2 as core::ffi::c_int
        {
            *dest.offset(i as isize) = ixheaac_add32_sat(
                ixheaacd_mult32_sh1(*src1.offset(i as isize), *win_fwd)
                    >> shiftp as core::ffi::c_int - shift_olap as core::ffi::c_int,
                ixheaacd_mult32_sh1(*over_lap.offset(i as isize), *win_rev),
            );
            win_fwd = win_fwd.offset(1);
            win_rev = win_rev.offset(-1);
            i += 1;
        }
        i = ((*ixheaacd_drc_offset).n_long as core::ffi::c_int / 2 as core::ffi::c_int)
            as WORD32;
        while i < (*ixheaacd_drc_offset).n_flat_ls + (*ixheaacd_drc_offset).n_trans_ls {
            *dest.offset(i as isize) = ixheaac_add32_sat(
                ixheaacd_mult32_sh1(
                    ixheaac_negate32_sat(
                        *src1
                            .offset(
                                ((*ixheaacd_drc_offset).n_long as core::ffi::c_int
                                    - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                            ),
                    ),
                    *win_fwd,
                ) >> shiftp as core::ffi::c_int - shift_olap as core::ffi::c_int,
                ixheaacd_mult32_sh1(*over_lap.offset(i as isize), *win_rev),
            );
            win_fwd = win_fwd.offset(1);
            win_rev = win_rev.offset(-1);
            i += 1;
        }
        while i < (*ixheaacd_drc_offset).n_long {
            *dest.offset(i as isize) = ixheaac_negate32_sat(
                *src1
                    .offset(
                        ((*ixheaacd_drc_offset).n_long as core::ffi::c_int
                            - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                    ),
            ) >> shiftp as core::ffi::c_int - shift_olap as core::ffi::c_int;
            i += 1;
        }
        return shift_olap;
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*ixheaacd_drc_offset).n_flat_ls {
            *dest.offset(i as isize) = *over_lap.offset(i as isize)
                >> shift_olap as core::ffi::c_int - shiftp as core::ffi::c_int;
            i += 1;
        }
        i = (*ixheaacd_drc_offset).n_flat_ls;
        while i
            < (*ixheaacd_drc_offset).n_long as core::ffi::c_int / 2 as core::ffi::c_int
        {
            *dest.offset(i as isize) = ixheaac_add32_sat(
                ixheaacd_mult32_sh1(*src1.offset(i as isize), *win_fwd),
                ixheaacd_mult32_sh1(*over_lap.offset(i as isize), *win_rev)
                    >> shift_olap as core::ffi::c_int - shiftp as core::ffi::c_int,
            );
            win_fwd = win_fwd.offset(1);
            win_rev = win_rev.offset(-1);
            i += 1;
        }
        i = ((*ixheaacd_drc_offset).n_long as core::ffi::c_int / 2 as core::ffi::c_int)
            as WORD32;
        while i < (*ixheaacd_drc_offset).n_flat_ls + (*ixheaacd_drc_offset).n_trans_ls {
            *dest.offset(i as isize) = ixheaac_add32_sat(
                ixheaacd_mult32_sh1(
                    ixheaac_negate32_sat(
                        *src1
                            .offset(
                                ((*ixheaacd_drc_offset).n_long as core::ffi::c_int
                                    - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                            ),
                    ),
                    *win_fwd,
                ),
                ixheaacd_mult32_sh1(*over_lap.offset(i as isize), *win_rev)
                    >> shift_olap as core::ffi::c_int - shiftp as core::ffi::c_int,
            );
            win_fwd = win_fwd.offset(1);
            win_rev = win_rev.offset(-1);
            i += 1;
        }
        while i < (*ixheaacd_drc_offset).n_long {
            *dest.offset(i as isize) = ixheaac_negate32_sat(
                *src1
                    .offset(
                        ((*ixheaacd_drc_offset).n_long as core::ffi::c_int
                            - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                    ),
            );
            i += 1;
        }
        return shiftp;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_windowing_short1(
    mut src1: *mut WORD32,
    mut src2: *mut WORD32,
    mut fp: *mut WORD32,
    mut ixheaacd_drc_offset: *mut offset_lengths,
    mut shiftp: WORD8,
    mut shift_olap: WORD8,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut dest: *mut WORD32 = fp;
    if shift_olap as core::ffi::c_int > shiftp as core::ffi::c_int {
        if (*ixheaacd_drc_offset).n_short > (*ixheaacd_drc_offset).lfac {
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*ixheaacd_drc_offset).lfac {
                *dest.offset(i as isize) = *dest.offset(i as isize)
                    >> shift_olap as core::ffi::c_int - shiftp as core::ffi::c_int;
                i += 1;
            }
            i = (*ixheaacd_drc_offset).lfac;
            while i < (*ixheaacd_drc_offset).n_short {
                *dest.offset(i as isize) = ixheaacd_mult32_sh1(
                    ixheaac_negate32_sat(
                        *src1
                            .offset(
                                ((*ixheaacd_drc_offset).n_short as core::ffi::c_int
                                    - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                            ),
                    ),
                    *src2.offset(i as isize),
                );
                i += 1;
            }
            while i < (*ixheaacd_drc_offset).n_flat_ls + (*ixheaacd_drc_offset).lfac {
                *dest.offset(i as isize) = 0 as core::ffi::c_int as WORD32;
                i += 1;
            }
        } else {
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*ixheaacd_drc_offset).lfac {
                *dest.offset(i as isize) = *dest.offset(i as isize)
                    >> shift_olap as core::ffi::c_int - shiftp as core::ffi::c_int;
                i += 1;
            }
            i = (*ixheaacd_drc_offset).lfac;
            while i < (*ixheaacd_drc_offset).n_flat_ls + (*ixheaacd_drc_offset).lfac {
                *dest.offset(i as isize) = 0 as core::ffi::c_int as WORD32;
                i += 1;
            }
        }
    } else if (*ixheaacd_drc_offset).n_short > (*ixheaacd_drc_offset).lfac {
        i = (*ixheaacd_drc_offset).lfac;
        while i < (*ixheaacd_drc_offset).n_short {
            *dest.offset(i as isize) = ixheaacd_mult32_sh1(
                ixheaac_negate32_sat(
                    *src1
                        .offset(
                            ((*ixheaacd_drc_offset).n_short as core::ffi::c_int
                                - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                        ),
                ),
                *src2.offset(i as isize),
            ) >> shiftp as core::ffi::c_int - shift_olap as core::ffi::c_int;
            i += 1;
        }
        while i < (*ixheaacd_drc_offset).n_flat_ls + (*ixheaacd_drc_offset).lfac {
            *dest.offset(i as isize) = 0 as core::ffi::c_int as WORD32;
            i += 1;
        }
    } else {
        i = (*ixheaacd_drc_offset).lfac;
        while i < (*ixheaacd_drc_offset).n_flat_ls + (*ixheaacd_drc_offset).lfac {
            *dest.offset(i as isize) = 0 as core::ffi::c_int as WORD32;
            i += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_windowing_short2(
    mut src1: *mut WORD32,
    mut win_fwd: *mut WORD32,
    mut fp: *mut WORD32,
    mut ixheaacd_drc_offset: *mut offset_lengths,
    mut shiftp: WORD8,
    mut shift_olap: WORD8,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut win_rev: *mut WORD32 = win_fwd
        .offset((*ixheaacd_drc_offset).n_short as isize)
        .offset(-(1 as core::ffi::c_int as isize));
    if shift_olap as core::ffi::c_int > shiftp as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i
            < (*ixheaacd_drc_offset).n_short as core::ffi::c_int / 2 as core::ffi::c_int
        {
            *fp.offset(i as isize) = ixheaac_add32_sat(
                ixheaacd_mult32_sh1(*src1.offset(i as isize), *win_fwd),
                ixheaacd_mult32_sh1(*fp.offset(i as isize), *win_rev)
                    >> shift_olap as core::ffi::c_int - shiftp as core::ffi::c_int,
            );
            *fp
                .offset(
                    ((*ixheaacd_drc_offset).n_short as core::ffi::c_int
                        - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                ) = ixheaac_add32_sat(
                ixheaacd_mult32_sh1(
                    ixheaac_negate32_sat(*src1.offset(i as isize)),
                    *win_rev,
                ),
                ixheaacd_mult32_sh1(
                    *fp
                        .offset(
                            ((*ixheaacd_drc_offset).n_short as core::ffi::c_int
                                - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                        ),
                    *win_fwd,
                ) >> shift_olap as core::ffi::c_int - shiftp as core::ffi::c_int,
            );
            win_fwd = win_fwd.offset(1);
            win_rev = win_rev.offset(-1);
            i += 1;
        }
        i = (*ixheaacd_drc_offset).n_short;
        while i < (*ixheaacd_drc_offset).n_flat_ls + (*ixheaacd_drc_offset).n_short {
            *fp.offset(i as isize) = 0 as core::ffi::c_int as WORD32;
            i += 1;
        }
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i
            < (*ixheaacd_drc_offset).n_short as core::ffi::c_int / 2 as core::ffi::c_int
        {
            *fp.offset(i as isize) = ixheaac_add32_sat(
                ixheaacd_mult32_sh1(*src1.offset(i as isize), *win_fwd)
                    >> shiftp as core::ffi::c_int - shift_olap as core::ffi::c_int,
                ixheaacd_mult32_sh1(*fp.offset(i as isize), *win_rev),
            );
            *fp
                .offset(
                    ((*ixheaacd_drc_offset).n_short as core::ffi::c_int
                        - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                ) = ixheaac_add32_sat(
                ixheaacd_mult32_sh1(
                    ixheaac_negate32_sat(*src1.offset(i as isize)),
                    *win_rev,
                ) >> shiftp as core::ffi::c_int - shift_olap as core::ffi::c_int,
                ixheaacd_mult32_sh1(
                    *fp
                        .offset(
                            ((*ixheaacd_drc_offset).n_short as core::ffi::c_int
                                - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                        ),
                    *win_fwd,
                ),
            );
            win_fwd = win_fwd.offset(1);
            win_rev = win_rev.offset(-1);
            i += 1;
        }
        i = (*ixheaacd_drc_offset).n_short;
        while i < (*ixheaacd_drc_offset).n_flat_ls + (*ixheaacd_drc_offset).n_short {
            *fp.offset(i as isize) = 0 as core::ffi::c_int as WORD32;
            i += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_windowing_short3(
    mut src1: *mut WORD32,
    mut win_rev: *mut WORD32,
    mut fp: *mut WORD32,
    mut n_short: WORD32,
    mut shiftp: WORD8,
    mut shift_olap: WORD8,
) -> WORD8 {
    let mut i: WORD32 = 0;
    let mut win_fwd: *const WORD32 = win_rev
        .offset(-(n_short as isize))
        .offset(1 as core::ffi::c_int as isize);
    if shift_olap as core::ffi::c_int > shiftp as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < n_short as core::ffi::c_int / 2 as core::ffi::c_int {
            *fp.offset(i as isize) = ixheaac_add32_sat(
                ixheaacd_mult32_sh1(
                    ixheaac_negate32_sat(
                        *src1
                            .offset(
                                (n_short as core::ffi::c_int / 2 as core::ffi::c_int
                                    - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                            ),
                    ),
                    *win_rev,
                ),
                *fp.offset(i as isize)
                    >> shift_olap as core::ffi::c_int - shiftp as core::ffi::c_int,
            );
            *fp
                .offset(
                    (n_short as core::ffi::c_int - i as core::ffi::c_int
                        - 1 as core::ffi::c_int) as isize,
                ) = ixheaac_add32_sat(
                ixheaacd_mult32_sh1(
                    ixheaac_negate32_sat(
                        *src1
                            .offset(
                                (n_short as core::ffi::c_int / 2 as core::ffi::c_int
                                    - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                            ),
                    ),
                    *win_fwd,
                ),
                *fp
                    .offset(
                        (n_short as core::ffi::c_int - i as core::ffi::c_int
                            - 1 as core::ffi::c_int) as isize,
                    ) >> shift_olap as core::ffi::c_int - shiftp as core::ffi::c_int,
            );
            win_rev = win_rev.offset(-1);
            win_fwd = win_fwd.offset(1);
            i += 1;
        }
        return shiftp;
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i < n_short as core::ffi::c_int / 2 as core::ffi::c_int {
            *fp.offset(i as isize) = ixheaac_add32_sat(
                ixheaacd_mult32_sh1(
                    ixheaac_negate32_sat(
                        *src1
                            .offset(
                                (n_short as core::ffi::c_int / 2 as core::ffi::c_int
                                    - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                            ),
                    ),
                    *win_rev,
                ) >> shiftp as core::ffi::c_int - shift_olap as core::ffi::c_int,
                *fp.offset(i as isize),
            );
            *fp
                .offset(
                    (n_short as core::ffi::c_int - i as core::ffi::c_int
                        - 1 as core::ffi::c_int) as isize,
                ) = ixheaac_add32_sat(
                ixheaacd_mult32_sh1(
                    ixheaac_negate32_sat(
                        *src1
                            .offset(
                                (n_short as core::ffi::c_int / 2 as core::ffi::c_int
                                    - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                            ),
                    ),
                    *win_fwd,
                ) >> shiftp as core::ffi::c_int - shift_olap as core::ffi::c_int,
                *fp
                    .offset(
                        (n_short as core::ffi::c_int - i as core::ffi::c_int
                            - 1 as core::ffi::c_int) as isize,
                    ),
            );
            win_rev = win_rev.offset(-1);
            win_fwd = win_fwd.offset(1);
            i += 1;
        }
        return shift_olap;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_windowing_short4(
    mut src1: *mut WORD32,
    mut win_fwd: *mut WORD32,
    mut fp: *mut WORD32,
    mut win_fwd1: *mut WORD32,
    mut n_short: WORD32,
    mut flag: WORD32,
    mut shiftp: WORD8,
    mut shift_olap: WORD8,
    mut output_q: WORD8,
) -> WORD8 {
    let mut i: WORD32 = 0;
    let mut win_rev: *const WORD32 = win_fwd
        .offset(n_short as isize)
        .offset(-(1 as core::ffi::c_int as isize));
    let mut win_rev1: *const WORD32 = win_fwd1
        .offset(-(n_short as isize))
        .offset(1 as core::ffi::c_int as isize);
    if shift_olap as core::ffi::c_int > output_q as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < n_short as core::ffi::c_int / 2 as core::ffi::c_int {
            *fp.offset(i as isize) = ixheaac_add32_sat(
                ixheaacd_mult32_sh1(
                    *src1.offset((n_short / 2 as WORD32 + i) as isize),
                    *win_fwd,
                ) >> shiftp as core::ffi::c_int - output_q as core::ffi::c_int,
                *fp.offset(i as isize),
            );
            *fp
                .offset(
                    (n_short as core::ffi::c_int - i as core::ffi::c_int
                        - 1 as core::ffi::c_int) as isize,
                ) = ixheaac_add32_sat(
                ixheaacd_mult32_sh1(
                    ixheaac_negate32_sat(
                        *src1.offset((n_short / 2 as WORD32 + i) as isize),
                    ),
                    *win_rev,
                ) >> shiftp as core::ffi::c_int - output_q as core::ffi::c_int,
                *fp
                    .offset(
                        (n_short as core::ffi::c_int - i as core::ffi::c_int
                            - 1 as core::ffi::c_int) as isize,
                    ),
            );
            win_fwd = win_fwd.offset(1);
            win_rev = win_rev.offset(-1);
            i += 1;
        }
        if flag == 1 as core::ffi::c_int {
            while i < n_short {
                *fp
                    .offset(
                        (i as core::ffi::c_int
                            + n_short as core::ffi::c_int / 2 as core::ffi::c_int)
                            as isize,
                    ) = ixheaac_add32_sat(
                    ixheaacd_mult32_sh1(
                        ixheaac_negate32_sat(
                            *src1
                                .offset(
                                    (n_short as core::ffi::c_int - i as core::ffi::c_int
                                        - 1 as core::ffi::c_int) as isize,
                                ),
                        ),
                        *win_fwd1,
                    ) >> shiftp as core::ffi::c_int - output_q as core::ffi::c_int,
                    *fp
                        .offset(
                            (i as core::ffi::c_int
                                + n_short as core::ffi::c_int / 2 as core::ffi::c_int)
                                as isize,
                        )
                        >> shift_olap as core::ffi::c_int - output_q as core::ffi::c_int,
                );
                *fp
                    .offset(
                        (3 as core::ffi::c_int * n_short as core::ffi::c_int
                            - n_short as core::ffi::c_int / 2 as core::ffi::c_int
                            - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                    ) = ixheaac_add32_sat(
                    ixheaacd_mult32_sh1(
                        ixheaac_negate32_sat(
                            *src1
                                .offset(
                                    (n_short as core::ffi::c_int - i as core::ffi::c_int
                                        - 1 as core::ffi::c_int) as isize,
                                ),
                        ),
                        *win_rev1,
                    ) >> shiftp as core::ffi::c_int - output_q as core::ffi::c_int,
                    *fp
                        .offset(
                            (3 as core::ffi::c_int * n_short as core::ffi::c_int
                                - n_short as core::ffi::c_int / 2 as core::ffi::c_int
                                - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                        )
                        >> shift_olap as core::ffi::c_int - output_q as core::ffi::c_int,
                );
                win_fwd1 = win_fwd1.offset(-1);
                win_rev1 = win_rev1.offset(1);
                i += 1;
            }
        } else {
            while i < n_short {
                *fp
                    .offset(
                        (i as core::ffi::c_int
                            + n_short as core::ffi::c_int / 2 as core::ffi::c_int)
                            as isize,
                    ) = ixheaac_add32_sat(
                    ixheaac_negate32_sat(
                        *src1
                            .offset(
                                (n_short as core::ffi::c_int - i as core::ffi::c_int
                                    - 1 as core::ffi::c_int) as isize,
                            ),
                    ) >> shiftp as core::ffi::c_int - output_q as core::ffi::c_int,
                    *fp
                        .offset(
                            (i as core::ffi::c_int
                                + n_short as core::ffi::c_int / 2 as core::ffi::c_int)
                                as isize,
                        )
                        >> shift_olap as core::ffi::c_int - output_q as core::ffi::c_int,
                );
                *fp
                    .offset(
                        (3 as core::ffi::c_int * n_short as core::ffi::c_int
                            - n_short as core::ffi::c_int / 2 as core::ffi::c_int
                            - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                    ) = ixheaac_add32_sat(
                    ixheaac_negate32_sat(
                        *src1
                            .offset(
                                (n_short as core::ffi::c_int - i as core::ffi::c_int
                                    - 1 as core::ffi::c_int) as isize,
                            ),
                    ) >> shiftp as core::ffi::c_int - output_q as core::ffi::c_int,
                    *fp
                        .offset(
                            (3 as core::ffi::c_int * n_short as core::ffi::c_int
                                - n_short as core::ffi::c_int / 2 as core::ffi::c_int
                                - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                        )
                        >> shift_olap as core::ffi::c_int - output_q as core::ffi::c_int,
                );
                i += 1;
            }
        }
        return output_q;
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i < n_short as core::ffi::c_int / 2 as core::ffi::c_int {
            *fp.offset(i as isize) = ixheaac_add32_sat(
                ixheaacd_mult32_sh1(
                    *src1.offset((n_short / 2 as WORD32 + i) as isize),
                    *win_fwd,
                ) >> shiftp as core::ffi::c_int - shift_olap as core::ffi::c_int,
                *fp.offset(i as isize)
                    >> output_q as core::ffi::c_int - shift_olap as core::ffi::c_int,
            );
            *fp
                .offset(
                    (n_short as core::ffi::c_int - i as core::ffi::c_int
                        - 1 as core::ffi::c_int) as isize,
                ) = ixheaac_add32_sat(
                ixheaacd_mult32_sh1(
                    ixheaac_negate32_sat(
                        *src1.offset((n_short / 2 as WORD32 + i) as isize),
                    ),
                    *win_rev,
                ) >> shiftp as core::ffi::c_int - shift_olap as core::ffi::c_int,
                *fp
                    .offset(
                        (n_short as core::ffi::c_int - i as core::ffi::c_int
                            - 1 as core::ffi::c_int) as isize,
                    ),
            );
            win_fwd = win_fwd.offset(1);
            win_rev = win_rev.offset(-1);
            i += 1;
        }
        if flag == 1 as core::ffi::c_int {
            while i < n_short {
                *fp
                    .offset(
                        (i as core::ffi::c_int
                            + n_short as core::ffi::c_int / 2 as core::ffi::c_int)
                            as isize,
                    ) = ixheaac_add32_sat(
                    ixheaacd_mult32_sh1(
                        ixheaac_negate32_sat(
                            *src1
                                .offset(
                                    (n_short as core::ffi::c_int - i as core::ffi::c_int
                                        - 1 as core::ffi::c_int) as isize,
                                ),
                        ),
                        *win_fwd1,
                    ) >> shiftp as core::ffi::c_int - shift_olap as core::ffi::c_int,
                    *fp
                        .offset(
                            (i as core::ffi::c_int
                                + n_short as core::ffi::c_int / 2 as core::ffi::c_int)
                                as isize,
                        ),
                );
                *fp
                    .offset(
                        (3 as core::ffi::c_int * n_short as core::ffi::c_int
                            - n_short as core::ffi::c_int / 2 as core::ffi::c_int
                            - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                    ) = ixheaac_add32_sat(
                    ixheaacd_mult32_sh1(
                        ixheaac_negate32_sat(
                            *src1
                                .offset(
                                    (n_short as core::ffi::c_int - i as core::ffi::c_int
                                        - 1 as core::ffi::c_int) as isize,
                                ),
                        ),
                        *win_rev1,
                    ) >> shiftp as core::ffi::c_int - shift_olap as core::ffi::c_int,
                    *fp
                        .offset(
                            (3 as core::ffi::c_int * n_short as core::ffi::c_int
                                - n_short as core::ffi::c_int / 2 as core::ffi::c_int
                                - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                        ),
                );
                win_fwd1 = win_fwd1.offset(-1);
                win_rev1 = win_rev1.offset(1);
                i += 1;
            }
        } else {
            while i < n_short {
                *fp
                    .offset(
                        (i as core::ffi::c_int
                            + n_short as core::ffi::c_int / 2 as core::ffi::c_int)
                            as isize,
                    ) = ixheaac_add32_sat(
                    ixheaac_negate32_sat(
                        *src1
                            .offset(
                                (n_short as core::ffi::c_int - i as core::ffi::c_int
                                    - 1 as core::ffi::c_int) as isize,
                            ),
                    ) >> shiftp as core::ffi::c_int - shift_olap as core::ffi::c_int,
                    *fp
                        .offset(
                            (i as core::ffi::c_int
                                + n_short as core::ffi::c_int / 2 as core::ffi::c_int)
                                as isize,
                        ),
                );
                *fp
                    .offset(
                        (3 as core::ffi::c_int * n_short as core::ffi::c_int
                            - n_short as core::ffi::c_int / 2 as core::ffi::c_int
                            - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                    ) = ixheaac_add32_sat(
                    ixheaac_negate32_sat(
                        *src1
                            .offset(
                                (n_short as core::ffi::c_int - i as core::ffi::c_int
                                    - 1 as core::ffi::c_int) as isize,
                            ),
                    ) >> shiftp as core::ffi::c_int - shift_olap as core::ffi::c_int,
                    *fp
                        .offset(
                            (3 as core::ffi::c_int * n_short as core::ffi::c_int
                                - n_short as core::ffi::c_int / 2 as core::ffi::c_int
                                - i as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                        ),
                );
                i += 1;
            }
        }
        return shift_olap;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_scale_down(
    mut dest: *mut WORD32,
    mut src: *mut WORD32,
    mut len: WORD32,
    mut shift1: WORD8,
    mut shift2: WORD8,
) -> VOID {
    let mut i: WORD32 = 0;
    if shift1 as core::ffi::c_int > shift2 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < len {
            *dest = *src >> shift1 as core::ffi::c_int - shift2 as core::ffi::c_int;
            src = src.offset(1);
            dest = dest.offset(1);
            i += 1;
        }
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i < len {
            *dest = ixheaac_shl32_sat(*src, shift2 as WORD - shift1 as WORD);
            src = src.offset(1);
            dest = dest.offset(1);
            i += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_scale_down_adj(
    mut dest: *mut WORD32,
    mut src: *mut WORD32,
    mut len: WORD32,
    mut shift1: WORD8,
    mut shift2: WORD8,
) -> VOID {
    let mut i: WORD32 = 0;
    if shift1 as core::ffi::c_int > shift2 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < len {
            *dest = ixheaac_add32_sat(
                *src >> shift1 as core::ffi::c_int - shift2 as core::ffi::c_int,
                ADJ_SCALE,
            );
            src = src.offset(1);
            dest = dest.offset(1);
            i += 1;
        }
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i < len {
            *dest = ixheaac_add32_sat(
                ixheaac_shl32_sat(*src, shift2 as WORD - shift1 as WORD),
                ADJ_SCALE,
            );
            src = src.offset(1);
            dest = dest.offset(1);
            i += 1;
        }
    };
}
