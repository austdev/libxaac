extern "C" {
    static ixheaac_twid_tbl_fft_double: [FLOAT64; 514];
    static ixheaac_twid_tbl_fft_ntwt3r: [FLOAT64; 1155];
    static ixheaac_twid_tbl_fft_ntwt3i: [FLOAT64; 1155];
    static ixheaac_twid_tbl_fft_224: [FLOAT32; 372];
    static ixheaac_twid_tbl_fft_288: [FLOAT32; 380];
    static ixheaac_twid_tbl_fft_336: [FLOAT32; 564];
    static ixheaac_twid_tbl_fft_168: [FLOAT32; 276];
}
pub type WORD32 = core::ffi::c_int;
pub type FLOAT32 = core::ffi::c_float;
pub type FLOAT64 = core::ffi::c_double;
pub type VOID = ();
pub type WORD = core::ffi::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mult32X32float(
    mut a: FLOAT64,
    mut b: FLOAT64,
) -> FLOAT64 {
    let mut result: FLOAT64 = 0.;
    result = a * b;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mac32X32float(
    mut a: FLOAT64,
    mut b: FLOAT64,
    mut c: FLOAT64,
) -> FLOAT64 {
    let mut result: FLOAT64 = 0.;
    result = a + b * c;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hbe_apply_ifft_7(
    mut inp: *mut FLOAT32,
    mut op: *mut FLOAT32,
) -> VOID {
    let mut x0r: FLOAT32 = 0.;
    let mut x1r: FLOAT32 = 0.;
    let mut x2r: FLOAT32 = 0.;
    let mut x3r: FLOAT32 = 0.;
    let mut x4r: FLOAT32 = 0.;
    let mut x5r: FLOAT32 = 0.;
    let mut x6r: FLOAT32 = 0.;
    let mut x7r: FLOAT32 = 0.;
    let mut x8r: FLOAT32 = 0.;
    let mut x0i: FLOAT32 = 0.;
    let mut x1i: FLOAT32 = 0.;
    let mut x2i: FLOAT32 = 0.;
    let mut x3i: FLOAT32 = 0.;
    let mut x4i: FLOAT32 = 0.;
    let mut x5i: FLOAT32 = 0.;
    let mut x6i: FLOAT32 = 0.;
    let mut x7i: FLOAT32 = 0.;
    let mut x8i: FLOAT32 = 0.;
    let mut y0r: FLOAT32 = 0.;
    let mut y1r: FLOAT32 = 0.;
    let mut y2r: FLOAT32 = 0.;
    let mut y3r: FLOAT32 = 0.;
    let mut y4r: FLOAT32 = 0.;
    let mut y5r: FLOAT32 = 0.;
    let mut y6r: FLOAT32 = 0.;
    let mut y7r: FLOAT32 = 0.;
    let mut y8r: FLOAT32 = 0.;
    let mut y0i: FLOAT32 = 0.;
    let mut y1i: FLOAT32 = 0.;
    let mut y2i: FLOAT32 = 0.;
    let mut y3i: FLOAT32 = 0.;
    let mut y4i: FLOAT32 = 0.;
    let mut y5i: FLOAT32 = 0.;
    let mut y6i: FLOAT32 = 0.;
    let mut y7i: FLOAT32 = 0.;
    let mut y8i: FLOAT32 = 0.;
    x0r = *inp.offset(0 as core::ffi::c_int as isize);
    x0i = *inp.offset(1 as core::ffi::c_int as isize);
    x1r = *inp.offset(2 as core::ffi::c_int as isize)
        + *inp.offset(12 as core::ffi::c_int as isize);
    x1i = *inp.offset(3 as core::ffi::c_int as isize)
        + *inp.offset(13 as core::ffi::c_int as isize);
    x2r = *inp.offset(2 as core::ffi::c_int as isize)
        - *inp.offset(12 as core::ffi::c_int as isize);
    x2i = *inp.offset(3 as core::ffi::c_int as isize)
        - *inp.offset(13 as core::ffi::c_int as isize);
    x3r = *inp.offset(4 as core::ffi::c_int as isize)
        + *inp.offset(10 as core::ffi::c_int as isize);
    x3i = *inp.offset(5 as core::ffi::c_int as isize)
        + *inp.offset(11 as core::ffi::c_int as isize);
    x4r = *inp.offset(4 as core::ffi::c_int as isize)
        - *inp.offset(10 as core::ffi::c_int as isize);
    x4i = *inp.offset(5 as core::ffi::c_int as isize)
        - *inp.offset(11 as core::ffi::c_int as isize);
    x5r = *inp.offset(8 as core::ffi::c_int as isize)
        + *inp.offset(6 as core::ffi::c_int as isize);
    x5i = *inp.offset(9 as core::ffi::c_int as isize)
        + *inp.offset(7 as core::ffi::c_int as isize);
    x6r = *inp.offset(8 as core::ffi::c_int as isize)
        - *inp.offset(6 as core::ffi::c_int as isize);
    x6i = *inp.offset(9 as core::ffi::c_int as isize)
        - *inp.offset(7 as core::ffi::c_int as isize);
    y0r = x0r;
    y0i = x0i;
    y1r = x1r + x3r + x5r;
    y1i = x1i + x3i + x5i;
    y2r = x1r - x3r;
    y2i = x1i - x3i;
    y3r = x5r - x1r;
    y3i = x5i - x1i;
    y4r = x3r - x5r;
    y4i = x3i - x5i;
    y5r = x2r + x4r + x6r;
    y5i = x2i + x4i + x6i;
    y6r = x2r - x4r;
    y6i = x2i - x4i;
    y7r = x6r - x2r;
    y7i = x6i - x2i;
    y8r = x4r - x6r;
    y8i = x4i - x6i;
    x0r = y0r + y1r;
    x0i = y0i + y1i;
    x1r = y0r + C70 * y1r;
    x1i = y0i + C70 * y1i;
    x2r = C71 * y2r;
    x2i = C71 * y2i;
    x3r = C72 * y3r;
    x3i = C72 * y3i;
    x4r = C73 * y4r;
    x4i = C73 * y4i;
    x5r = C74 * y5i;
    x5i = -C74 * y5r;
    x6r = C75 * y6i;
    x6i = -C75 * y6r;
    x7r = C76 * y7i;
    x7i = -C76 * y7r;
    x8r = C77 * y8i;
    x8i = -C77 * y8r;
    y0r = x0r;
    y0i = x0i;
    y1r = x1r + x2r + x4r;
    y1i = x1i + x2i + x4i;
    y2r = x1r - x2r - x3r;
    y2i = x1i - x2i - x3i;
    y3r = x1r + x3r - x4r;
    y3i = x1i + x3i - x4i;
    y4r = x5r + x6r + x8r;
    y4i = x5i + x6i + x8i;
    y5r = x5r - x6r - x7r;
    y5i = x5i - x6i - x7i;
    y6r = x5r + x7r - x8r;
    y6i = x5i + x7i - x8i;
    x0r = y0r;
    x0i = y0i;
    x1r = y1r + y4r;
    x1i = y1i + y4i;
    x2r = y3r + y6r;
    x2i = y3i + y6i;
    x3r = y2r - y5r;
    x3i = y2i - y5i;
    x4r = y2r + y5r;
    x4i = y2i + y5i;
    x5r = y3r - y6r;
    x5i = y3i - y6i;
    x6r = y1r - y4r;
    x6i = y1i - y4i;
    *op.offset(0 as core::ffi::c_int as isize) = x0r;
    *op.offset(1 as core::ffi::c_int as isize) = x0i;
    *op.offset(2 as core::ffi::c_int as isize) = x1r;
    *op.offset(3 as core::ffi::c_int as isize) = x1i;
    *op.offset(4 as core::ffi::c_int as isize) = x2r;
    *op.offset(5 as core::ffi::c_int as isize) = x2i;
    *op.offset(6 as core::ffi::c_int as isize) = x3r;
    *op.offset(7 as core::ffi::c_int as isize) = x3i;
    *op.offset(8 as core::ffi::c_int as isize) = x4r;
    *op.offset(9 as core::ffi::c_int as isize) = x4i;
    *op.offset(10 as core::ffi::c_int as isize) = x5r;
    *op.offset(11 as core::ffi::c_int as isize) = x5i;
    *op.offset(12 as core::ffi::c_int as isize) = x6r;
    *op.offset(13 as core::ffi::c_int as isize) = x6i;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hbe_apply_fft_3(
    mut inp: *mut FLOAT32,
    mut op: *mut FLOAT32,
    mut i_sign: WORD32,
) -> VOID {
    let mut add_r: FLOAT32 = 0.;
    let mut sub_r: FLOAT32 = 0.;
    let mut add_i: FLOAT32 = 0.;
    let mut sub_i: FLOAT32 = 0.;
    let mut X01r: FLOAT32 = 0.;
    let mut X01i: FLOAT32 = 0.;
    let mut temp: FLOAT32 = 0.;
    let mut p1: FLOAT32 = 0.;
    let mut p2: FLOAT32 = 0.;
    let mut p3: FLOAT32 = 0.;
    let mut p4: FLOAT32 = 0.;
    let mut sinmu: FLOAT64 = 0.;
    sinmu = -0.866025403784439f64 * i_sign as FLOAT64;
    X01r = *inp.offset(0 as core::ffi::c_int as isize)
        + *inp.offset(2 as core::ffi::c_int as isize);
    X01i = *inp.offset(1 as core::ffi::c_int as isize)
        + *inp.offset(3 as core::ffi::c_int as isize);
    add_r = *inp.offset(2 as core::ffi::c_int as isize)
        + *inp.offset(4 as core::ffi::c_int as isize);
    add_i = *inp.offset(3 as core::ffi::c_int as isize)
        + *inp.offset(5 as core::ffi::c_int as isize);
    sub_r = *inp.offset(2 as core::ffi::c_int as isize)
        - *inp.offset(4 as core::ffi::c_int as isize);
    sub_i = *inp.offset(3 as core::ffi::c_int as isize)
        - *inp.offset(5 as core::ffi::c_int as isize);
    p1 = add_r / 2.0f64 as FLOAT32;
    p4 = add_i / 2.0f64 as FLOAT32;
    p2 = (sub_i as FLOAT64 * sinmu) as FLOAT32;
    p3 = (sub_r as FLOAT64 * sinmu) as FLOAT32;
    temp = *inp.offset(0 as core::ffi::c_int as isize) - p1;
    *op.offset(0 as core::ffi::c_int as isize) = X01r
        + *inp.offset(4 as core::ffi::c_int as isize);
    *op.offset(1 as core::ffi::c_int as isize) = X01i
        + *inp.offset(5 as core::ffi::c_int as isize);
    *op.offset(2 as core::ffi::c_int as isize) = temp + p2;
    *op.offset(3 as core::ffi::c_int as isize) = *inp
        .offset(1 as core::ffi::c_int as isize) - p3 - p4;
    *op.offset(4 as core::ffi::c_int as isize) = temp - p2;
    *op.offset(5 as core::ffi::c_int as isize) = *inp
        .offset(1 as core::ffi::c_int as isize) + p3 - p4;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hbe_apply_tw_mult_ifft(
    mut inp: *mut FLOAT32,
    mut op: *mut FLOAT32,
    mut dim1: WORD32,
    mut dim2: WORD32,
    mut tw: *const FLOAT32,
) -> VOID {
    let mut accu1: FLOAT32 = 0.;
    let mut accu2: FLOAT32 = 0.;
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut step_val: WORD32 = (dim2 - 1 as WORD32) << 1 as core::ffi::c_int;
    i = 0 as core::ffi::c_int as WORD32;
    while i < dim2 {
        *op.offset(0 as core::ffi::c_int as isize) = *inp
            .offset(0 as core::ffi::c_int as isize);
        *op.offset(1 as core::ffi::c_int as isize) = *inp
            .offset(1 as core::ffi::c_int as isize);
        op = op.offset(2 as core::ffi::c_int as isize);
        inp = inp.offset(2 as core::ffi::c_int as isize);
        i += 1;
    }
    j = 0 as core::ffi::c_int as WORD32;
    while j < dim1 as core::ffi::c_int - 1 as core::ffi::c_int {
        *op.offset(0 as core::ffi::c_int as isize) = *inp
            .offset(0 as core::ffi::c_int as isize);
        *op.offset(1 as core::ffi::c_int as isize) = *inp
            .offset(1 as core::ffi::c_int as isize);
        inp = inp.offset(2 as core::ffi::c_int as isize);
        op = op.offset(2 as core::ffi::c_int as isize);
        i = 0 as core::ffi::c_int as WORD32;
        while i < dim2 as core::ffi::c_int - 1 as core::ffi::c_int {
            accu1 = *inp
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                )
                * *tw
                    .offset(
                        (2 as core::ffi::c_int * i as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    )
                + *inp
                    .offset(
                        (2 as core::ffi::c_int * i as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    ) * *tw.offset((2 as WORD32 * i) as isize);
            accu2 = -(*inp
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ) * *tw.offset((2 as WORD32 * i) as isize))
                + *inp
                    .offset(
                        (2 as core::ffi::c_int * i as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    )
                    * *tw
                        .offset(
                            (2 as core::ffi::c_int * i as core::ffi::c_int
                                + 1 as core::ffi::c_int) as isize,
                        );
            *op
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ) = accu1;
            *op
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) = accu2;
            i += 1;
        }
        inp = inp.offset(step_val as isize);
        op = op.offset(step_val as isize);
        tw = tw
            .offset(
                ((dim2 as core::ffi::c_int - 1 as core::ffi::c_int)
                    * 2 as core::ffi::c_int) as isize,
            );
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hbe_apply_tw_mult_fft(
    mut inp: *mut FLOAT32,
    mut op: *mut FLOAT32,
    mut dim1: WORD32,
    mut dim2: WORD32,
    mut tw: *const FLOAT32,
) -> VOID {
    let mut accu1: FLOAT32 = 0.;
    let mut accu2: FLOAT32 = 0.;
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut step_val: WORD32 = (dim2 - 1 as WORD32) << 1 as core::ffi::c_int;
    i = 0 as core::ffi::c_int as WORD32;
    while i < dim2 {
        *op.offset(0 as core::ffi::c_int as isize) = *inp
            .offset(0 as core::ffi::c_int as isize);
        *op.offset(1 as core::ffi::c_int as isize) = *inp
            .offset(1 as core::ffi::c_int as isize);
        op = op.offset(2 as core::ffi::c_int as isize);
        inp = inp.offset(2 as core::ffi::c_int as isize);
        i += 1;
    }
    j = 0 as core::ffi::c_int as WORD32;
    while j < dim1 as core::ffi::c_int - 1 as core::ffi::c_int {
        *op.offset(0 as core::ffi::c_int as isize) = *inp
            .offset(0 as core::ffi::c_int as isize);
        *op.offset(1 as core::ffi::c_int as isize) = *inp
            .offset(1 as core::ffi::c_int as isize);
        inp = inp.offset(2 as core::ffi::c_int as isize);
        op = op.offset(2 as core::ffi::c_int as isize);
        i = 0 as core::ffi::c_int as WORD32;
        while i < dim2 as core::ffi::c_int - 1 as core::ffi::c_int {
            accu1 = *inp
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                )
                * *tw
                    .offset(
                        (2 as core::ffi::c_int * i as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    )
                - *inp
                    .offset(
                        (2 as core::ffi::c_int * i as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    ) * *tw.offset((2 as WORD32 * i) as isize);
            accu2 = *inp
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ) * *tw.offset((2 as WORD32 * i) as isize)
                + *inp
                    .offset(
                        (2 as core::ffi::c_int * i as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    )
                    * *tw
                        .offset(
                            (2 as core::ffi::c_int * i as core::ffi::c_int
                                + 1 as core::ffi::c_int) as isize,
                        );
            *op
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ) = accu1;
            *op
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) = accu2;
            i += 1;
        }
        inp = inp.offset(step_val as isize);
        op = op.offset(step_val as isize);
        tw = tw
            .offset(
                ((dim2 as core::ffi::c_int - 1 as core::ffi::c_int)
                    * 2 as core::ffi::c_int) as isize,
            );
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hbe_apply_cfftn(
    mut re: *mut FLOAT32,
    mut scratch: *mut FLOAT32,
    mut n_pass: WORD32,
    mut i_sign: WORD32,
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
    let mut not_power_4: WORD32 = 0;
    let mut dig_rev_shift: WORD32 = 0;
    let mut mpass: WORD32 = n_pass;
    let mut npoints: WORD32 = n_pass;
    let mut ptr_w: *const FLOAT64 = 0 as *const FLOAT64;
    let mut ptr_x: *mut FLOAT32 = scratch;
    let mut y: *mut FLOAT32 = scratch.offset((2 as WORD32 * n_pass) as isize);
    let mut ptr_y: *mut FLOAT32 = y;
    dig_rev_shift = (ixheaac_norm32(mpass) as core::ffi::c_int + 1 as core::ffi::c_int
        - 16 as core::ffi::c_int) as WORD32;
    n_stages = (30 as WORD - ixheaac_norm32(mpass)) as WORD32;
    not_power_4 = (n_stages as core::ffi::c_int & 1 as core::ffi::c_int) as WORD32;
    n_stages = n_stages >> 1 as core::ffi::c_int;
    ptr_w = ixheaac_twid_tbl_fft_double.as_ptr();
    ptr_x = re as *mut FLOAT32;
    if i_sign == -(1 as core::ffi::c_int) {
        i = 0 as core::ffi::c_int as WORD32;
        while i < npoints {
            let mut inp: *mut FLOAT32 = ptr_x;
            let mut tmk: FLOAT32 = 0.;
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
            let fresh0 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh0 = x0r;
            let fresh1 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh1 = x0i;
            let fresh2 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh2 = x2r;
            let fresh3 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh3 = x2i;
            let fresh4 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh4 = x1r;
            let fresh5 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh5 = x1i;
            let fresh6 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh6 = x3i;
            let fresh7 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh7 = x3r;
            i += 4 as core::ffi::c_int;
        }
        ptr_y = ptr_y.offset(-((2 as WORD32 * npoints) as isize));
        del = 4 as core::ffi::c_int as WORD32;
        nodespacing = 64 as core::ffi::c_int as WORD32;
        in_loop_cnt = npoints >> 4 as core::ffi::c_int;
        i = (n_stages as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        while i > 0 as core::ffi::c_int {
            let mut twiddles: *const FLOAT64 = ptr_w;
            let mut data: *mut FLOAT32 = ptr_y;
            let mut W1: FLOAT64 = 0.;
            let mut W2: FLOAT64 = 0.;
            let mut W3: FLOAT64 = 0.;
            let mut W4: FLOAT64 = 0.;
            let mut W5: FLOAT64 = 0.;
            let mut W6: FLOAT64 = 0.;
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
                W1 = *twiddles.offset(j as isize);
                W4 = *twiddles
                    .offset(j as isize)
                    .offset(257 as core::ffi::c_int as isize);
                W2 = *twiddles.offset((j << 1 as core::ffi::c_int) as isize);
                W5 = *twiddles
                    .offset((j << 1 as core::ffi::c_int) as isize)
                    .offset(257 as core::ffi::c_int as isize);
                W3 = *twiddles
                    .offset(j as isize)
                    .offset((j << 1 as core::ffi::c_int) as isize);
                W6 = *twiddles
                    .offset(j as isize)
                    .offset((j << 1 as core::ffi::c_int) as isize)
                    .offset(257 as core::ffi::c_int as isize);
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
                        .offset(
                            -((3 as WORD32 * (del << 1 as core::ffi::c_int)) as isize),
                        );
                    tmp = (ixheaacd_mult32X32float(x1r_0 as FLOAT64, W1)
                        - ixheaacd_mult32X32float(x1i_0 as FLOAT64, W4)) as FLOAT32;
                    x1i_0 = ixheaacd_mac32X32float(
                        ixheaacd_mult32X32float(x1r_0 as FLOAT64, W4),
                        x1i_0 as FLOAT64,
                        W1,
                    ) as FLOAT32;
                    x1r_0 = tmp;
                    tmp = (ixheaacd_mult32X32float(x2r_0 as FLOAT64, W2)
                        - ixheaacd_mult32X32float(x2i_0 as FLOAT64, W5)) as FLOAT32;
                    x2i_0 = ixheaacd_mac32X32float(
                        ixheaacd_mult32X32float(x2r_0 as FLOAT64, W5),
                        x2i_0 as FLOAT64,
                        W2,
                    ) as FLOAT32;
                    x2r_0 = tmp;
                    tmp = (ixheaacd_mult32X32float(x3r_0 as FLOAT64, W3)
                        - ixheaacd_mult32X32float(x3i_0 as FLOAT64, W6)) as FLOAT32;
                    x3i_0 = ixheaacd_mac32X32float(
                        ixheaacd_mult32X32float(x3r_0 as FLOAT64, W6),
                        x3i_0 as FLOAT64,
                        W3,
                    ) as FLOAT32;
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
                W1 = *twiddles.offset(j as isize);
                W4 = *twiddles
                    .offset(j as isize)
                    .offset(257 as core::ffi::c_int as isize);
                W2 = *twiddles.offset((j << 1 as core::ffi::c_int) as isize);
                W5 = *twiddles
                    .offset((j << 1 as core::ffi::c_int) as isize)
                    .offset(257 as core::ffi::c_int as isize);
                W3 = *twiddles
                    .offset(j as isize)
                    .offset((j << 1 as core::ffi::c_int) as isize)
                    .offset(-(256 as core::ffi::c_int as isize));
                W6 = *twiddles
                    .offset(j as isize)
                    .offset((j << 1 as core::ffi::c_int) as isize)
                    .offset(1 as core::ffi::c_int as isize);
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
                        .offset(
                            -((3 as WORD32 * (del << 1 as core::ffi::c_int)) as isize),
                        );
                    tmp_0 = (ixheaacd_mult32X32float(x1r_1 as FLOAT64, W1)
                        - ixheaacd_mult32X32float(x1i_1 as FLOAT64, W4)) as FLOAT32;
                    x1i_1 = ixheaacd_mac32X32float(
                        ixheaacd_mult32X32float(x1r_1 as FLOAT64, W4),
                        x1i_1 as FLOAT64,
                        W1,
                    ) as FLOAT32;
                    x1r_1 = tmp_0;
                    tmp_0 = (ixheaacd_mult32X32float(x2r_1 as FLOAT64, W2)
                        - ixheaacd_mult32X32float(x2i_1 as FLOAT64, W5)) as FLOAT32;
                    x2i_1 = ixheaacd_mac32X32float(
                        ixheaacd_mult32X32float(x2r_1 as FLOAT64, W5),
                        x2i_1 as FLOAT64,
                        W2,
                    ) as FLOAT32;
                    x2r_1 = tmp_0;
                    tmp_0 = (ixheaacd_mult32X32float(x3r_1 as FLOAT64, W6)
                        + ixheaacd_mult32X32float(x3i_1 as FLOAT64, W3)) as FLOAT32;
                    x3i_1 = (-ixheaacd_mult32X32float(x3r_1 as FLOAT64, W3)
                        + ixheaacd_mult32X32float(x3i_1 as FLOAT64, W6)) as FLOAT32;
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
                W1 = *twiddles.offset(j as isize);
                W4 = *twiddles
                    .offset(j as isize)
                    .offset(257 as core::ffi::c_int as isize);
                W2 = *twiddles
                    .offset((j << 1 as core::ffi::c_int) as isize)
                    .offset(-(256 as core::ffi::c_int as isize));
                W5 = *twiddles
                    .offset((j << 1 as core::ffi::c_int) as isize)
                    .offset(1 as core::ffi::c_int as isize);
                W3 = *twiddles
                    .offset(j as isize)
                    .offset((j << 1 as core::ffi::c_int) as isize)
                    .offset(-(256 as core::ffi::c_int as isize));
                W6 = *twiddles
                    .offset(j as isize)
                    .offset((j << 1 as core::ffi::c_int) as isize)
                    .offset(1 as core::ffi::c_int as isize);
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
                        .offset(
                            -((3 as WORD32 * (del << 1 as core::ffi::c_int)) as isize),
                        );
                    tmp_1 = (ixheaacd_mult32X32float(x1r_2 as FLOAT64, W1)
                        - ixheaacd_mult32X32float(x1i_2 as FLOAT64, W4)) as FLOAT32;
                    x1i_2 = ixheaacd_mac32X32float(
                        ixheaacd_mult32X32float(x1r_2 as FLOAT64, W4),
                        x1i_2 as FLOAT64,
                        W1,
                    ) as FLOAT32;
                    x1r_2 = tmp_1;
                    tmp_1 = (ixheaacd_mult32X32float(x2r_2 as FLOAT64, W5)
                        + ixheaacd_mult32X32float(x2i_2 as FLOAT64, W2)) as FLOAT32;
                    x2i_2 = (-ixheaacd_mult32X32float(x2r_2 as FLOAT64, W2)
                        + ixheaacd_mult32X32float(x2i_2 as FLOAT64, W5)) as FLOAT32;
                    x2r_2 = tmp_1;
                    tmp_1 = (ixheaacd_mult32X32float(x3r_2 as FLOAT64, W6)
                        + ixheaacd_mult32X32float(x3i_2 as FLOAT64, W3)) as FLOAT32;
                    x3i_2 = (-ixheaacd_mult32X32float(x3r_2 as FLOAT64, W3)
                        + ixheaacd_mult32X32float(x3i_2 as FLOAT64, W6)) as FLOAT32;
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
                W1 = *twiddles.offset(j as isize);
                W4 = *twiddles
                    .offset(j as isize)
                    .offset(257 as core::ffi::c_int as isize);
                W2 = *twiddles
                    .offset((j << 1 as core::ffi::c_int) as isize)
                    .offset(-(256 as core::ffi::c_int as isize));
                W5 = *twiddles
                    .offset((j << 1 as core::ffi::c_int) as isize)
                    .offset(1 as core::ffi::c_int as isize);
                W3 = *twiddles
                    .offset(j as isize)
                    .offset((j << 1 as core::ffi::c_int) as isize)
                    .offset(-(512 as core::ffi::c_int as isize));
                W6 = *twiddles
                    .offset(j as isize)
                    .offset((j << 1 as core::ffi::c_int) as isize)
                    .offset(-(512 as core::ffi::c_int as isize))
                    .offset(257 as core::ffi::c_int as isize);
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
                        .offset(
                            -((3 as WORD32 * (del << 1 as core::ffi::c_int)) as isize),
                        );
                    tmp_2 = (ixheaacd_mult32X32float(x1r_3 as FLOAT64, W1)
                        - ixheaacd_mult32X32float(x1i_3 as FLOAT64, W4)) as FLOAT32;
                    x1i_3 = ixheaacd_mac32X32float(
                        ixheaacd_mult32X32float(x1r_3 as FLOAT64, W4),
                        x1i_3 as FLOAT64,
                        W1,
                    ) as FLOAT32;
                    x1r_3 = tmp_2;
                    tmp_2 = (ixheaacd_mult32X32float(x2r_3 as FLOAT64, W5)
                        + ixheaacd_mult32X32float(x2i_3 as FLOAT64, W2)) as FLOAT32;
                    x2i_3 = (-ixheaacd_mult32X32float(x2r_3 as FLOAT64, W2)
                        + ixheaacd_mult32X32float(x2i_3 as FLOAT64, W5)) as FLOAT32;
                    x2r_3 = tmp_2;
                    tmp_2 = (-ixheaacd_mult32X32float(x3r_3 as FLOAT64, W3)
                        + ixheaacd_mult32X32float(x3i_3 as FLOAT64, W6)) as FLOAT32;
                    x3i_3 = ixheaacd_mac32X32float(
                        ixheaacd_mult32X32float(x3r_3 as FLOAT64, W6),
                        x3i_3 as FLOAT64,
                        W3,
                    ) as FLOAT32;
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
            let mut twiddles_0: *const FLOAT64 = ptr_w;
            nodespacing <<= 1 as core::ffi::c_int;
            j = (del as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
            while j != 0 as core::ffi::c_int {
                let mut W1_0: FLOAT64 = *twiddles_0;
                let mut W4_0: FLOAT64 = *twiddles_0
                    .offset(257 as core::ffi::c_int as isize);
                let mut tmp_3: FLOAT32 = 0.;
                twiddles_0 = twiddles_0.offset(nodespacing as isize);
                x0r = *ptr_y;
                x0i = *ptr_y.offset(1 as core::ffi::c_int as isize);
                ptr_y = ptr_y.offset((del << 1 as core::ffi::c_int) as isize);
                x1r = *ptr_y;
                x1i = *ptr_y.offset(1 as core::ffi::c_int as isize);
                tmp_3 = (ixheaacd_mult32X32float(x1r as FLOAT64, W1_0)
                    - ixheaacd_mult32X32float(x1i as FLOAT64, W4_0)) as FLOAT32;
                x1i = ixheaacd_mac32X32float(
                    ixheaacd_mult32X32float(x1r as FLOAT64, W4_0),
                    x1i as FLOAT64,
                    W1_0,
                ) as FLOAT32;
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
                let mut W1_1: FLOAT64 = *twiddles_0;
                let mut W4_1: FLOAT64 = *twiddles_0
                    .offset(257 as core::ffi::c_int as isize);
                let mut tmp_4: FLOAT32 = 0.;
                twiddles_0 = twiddles_0.offset(nodespacing as isize);
                x0r = *ptr_y;
                x0i = *ptr_y.offset(1 as core::ffi::c_int as isize);
                ptr_y = ptr_y.offset((del << 1 as core::ffi::c_int) as isize);
                x1r = *ptr_y;
                x1i = *ptr_y.offset(1 as core::ffi::c_int as isize);
                tmp_4 = (ixheaacd_mult32X32float(x1r as FLOAT64, W4_1)
                    + ixheaacd_mult32X32float(x1i as FLOAT64, W1_1)) as FLOAT32;
                x1i = (-ixheaacd_mult32X32float(x1r as FLOAT64, W1_1)
                    + ixheaacd_mult32X32float(x1i as FLOAT64, W4_1)) as FLOAT32;
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
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i < npoints {
            let mut inp_0: *mut FLOAT32 = ptr_x;
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
            x2r = x2r - x3i;
            x2i = x2i + x3r;
            x3i = x2r + x3i * 2 as core::ffi::c_int as FLOAT32;
            x3r = x2i - x3r * 2 as core::ffi::c_int as FLOAT32;
            let fresh8 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh8 = x0r;
            let fresh9 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh9 = x0i;
            let fresh10 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh10 = x2r;
            let fresh11 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh11 = x2i;
            let fresh12 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh12 = x1r;
            let fresh13 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh13 = x1i;
            let fresh14 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh14 = x3i;
            let fresh15 = ptr_y;
            ptr_y = ptr_y.offset(1);
            *fresh15 = x3r;
            i += 4 as core::ffi::c_int;
        }
        ptr_y = ptr_y.offset(-((2 as WORD32 * npoints) as isize));
        del = 4 as core::ffi::c_int as WORD32;
        nodespacing = 64 as core::ffi::c_int as WORD32;
        in_loop_cnt = npoints >> 4 as core::ffi::c_int;
        i = (n_stages as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        while i > 0 as core::ffi::c_int {
            let mut twiddles_1: *const FLOAT64 = ptr_w;
            let mut data_0: *mut FLOAT32 = ptr_y;
            let mut W1_2: FLOAT64 = 0.;
            let mut W2_0: FLOAT64 = 0.;
            let mut W3_0: FLOAT64 = 0.;
            let mut W4_2: FLOAT64 = 0.;
            let mut W5_0: FLOAT64 = 0.;
            let mut W6_0: FLOAT64 = 0.;
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
                x2r = x2r - x3i;
                x2i = x2i + x3r;
                x3i = x2r + x3i * 2 as core::ffi::c_int as FLOAT32;
                x3r = x2i - x3r * 2 as core::ffi::c_int as FLOAT32;
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
                W1_2 = *twiddles_1.offset(j as isize);
                W4_2 = *twiddles_1
                    .offset(j as isize)
                    .offset(257 as core::ffi::c_int as isize);
                W2_0 = *twiddles_1.offset((j << 1 as core::ffi::c_int) as isize);
                W5_0 = *twiddles_1
                    .offset((j << 1 as core::ffi::c_int) as isize)
                    .offset(257 as core::ffi::c_int as isize);
                W3_0 = *twiddles_1
                    .offset(j as isize)
                    .offset((j << 1 as core::ffi::c_int) as isize);
                W6_0 = *twiddles_1
                    .offset(j as isize)
                    .offset((j << 1 as core::ffi::c_int) as isize)
                    .offset(257 as core::ffi::c_int as isize);
                k = in_loop_cnt;
                while k != 0 as core::ffi::c_int {
                    let mut tmp_5: FLOAT32 = 0.;
                    let mut x0r_4: FLOAT32 = 0.;
                    let mut x0i_4: FLOAT32 = 0.;
                    let mut x1r_4: FLOAT32 = 0.;
                    let mut x1i_4: FLOAT32 = 0.;
                    let mut x2r_4: FLOAT32 = 0.;
                    let mut x2i_4: FLOAT32 = 0.;
                    let mut x3r_4: FLOAT32 = 0.;
                    let mut x3i_4: FLOAT32 = 0.;
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
                    tmp_5 = (x1r_4 as FLOAT64 * W1_2 + x1i_4 as FLOAT64 * W4_2)
                        as FLOAT32;
                    x1i_4 = (-(x1r_4 as FLOAT64 * W4_2) + x1i_4 as FLOAT64 * W1_2)
                        as FLOAT32;
                    x1r_4 = tmp_5;
                    tmp_5 = (x2r_4 as FLOAT64 * W2_0 + x2i_4 as FLOAT64 * W5_0)
                        as FLOAT32;
                    x2i_4 = (-(x2r_4 as FLOAT64 * W5_0) + x2i_4 as FLOAT64 * W2_0)
                        as FLOAT32;
                    x2r_4 = tmp_5;
                    tmp_5 = (x3r_4 as FLOAT64 * W3_0 + x3i_4 as FLOAT64 * W6_0)
                        as FLOAT32;
                    x3i_4 = (-(x3r_4 as FLOAT64 * W6_0) + x3i_4 as FLOAT64 * W3_0)
                        as FLOAT32;
                    x3r_4 = tmp_5;
                    x0r_4 = *data_0;
                    x0i_4 = *data_0.offset(1 as core::ffi::c_int as isize);
                    x0r_4 = x0r_4 + x2r_4;
                    x0i_4 = x0i_4 + x2i_4;
                    x2r_4 = x0r_4 - x2r_4 * 2 as core::ffi::c_int as FLOAT32;
                    x2i_4 = x0i_4 - x2i_4 * 2 as core::ffi::c_int as FLOAT32;
                    x1r_4 = x1r_4 + x3r_4;
                    x1i_4 = x1i_4 + x3i_4;
                    x3r_4 = x1r_4 - x3r_4 * 2 as core::ffi::c_int as FLOAT32;
                    x3i_4 = x1i_4 - x3i_4 * 2 as core::ffi::c_int as FLOAT32;
                    x0r_4 = x0r_4 + x1r_4;
                    x0i_4 = x0i_4 + x1i_4;
                    x1r_4 = x0r_4 - x1r_4 * 2 as core::ffi::c_int as FLOAT32;
                    x1i_4 = x0i_4 - x1i_4 * 2 as core::ffi::c_int as FLOAT32;
                    x2r_4 = x2r_4 - x3i_4;
                    x2i_4 = x2i_4 + x3r_4;
                    x3i_4 = x2r_4 + x3i_4 * 2 as core::ffi::c_int as FLOAT32;
                    x3r_4 = x2i_4 - x3r_4 * 2 as core::ffi::c_int as FLOAT32;
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
                W1_2 = *twiddles_1.offset(j as isize);
                W4_2 = *twiddles_1
                    .offset(j as isize)
                    .offset(257 as core::ffi::c_int as isize);
                W2_0 = *twiddles_1.offset((j << 1 as core::ffi::c_int) as isize);
                W5_0 = *twiddles_1
                    .offset((j << 1 as core::ffi::c_int) as isize)
                    .offset(257 as core::ffi::c_int as isize);
                W3_0 = *twiddles_1
                    .offset(j as isize)
                    .offset((j << 1 as core::ffi::c_int) as isize)
                    .offset(-(256 as core::ffi::c_int as isize));
                W6_0 = *twiddles_1
                    .offset(j as isize)
                    .offset((j << 1 as core::ffi::c_int) as isize)
                    .offset(1 as core::ffi::c_int as isize);
                k = in_loop_cnt;
                while k != 0 as core::ffi::c_int {
                    let mut tmp_6: FLOAT32 = 0.;
                    let mut x0r_5: FLOAT32 = 0.;
                    let mut x0i_5: FLOAT32 = 0.;
                    let mut x1r_5: FLOAT32 = 0.;
                    let mut x1i_5: FLOAT32 = 0.;
                    let mut x2r_5: FLOAT32 = 0.;
                    let mut x2i_5: FLOAT32 = 0.;
                    let mut x3r_5: FLOAT32 = 0.;
                    let mut x3i_5: FLOAT32 = 0.;
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
                    tmp_6 = (x1r_5 as FLOAT64 * W1_2 + x1i_5 as FLOAT64 * W4_2)
                        as FLOAT32;
                    x1i_5 = (-(x1r_5 as FLOAT64 * W4_2) + x1i_5 as FLOAT64 * W1_2)
                        as FLOAT32;
                    x1r_5 = tmp_6;
                    tmp_6 = (x2r_5 as FLOAT64 * W2_0 + x2i_5 as FLOAT64 * W5_0)
                        as FLOAT32;
                    x2i_5 = (-(x2r_5 as FLOAT64 * W5_0) + x2i_5 as FLOAT64 * W2_0)
                        as FLOAT32;
                    x2r_5 = tmp_6;
                    tmp_6 = (x3r_5 as FLOAT64 * W6_0 - x3i_5 as FLOAT64 * W3_0)
                        as FLOAT32;
                    x3i_5 = (x3r_5 as FLOAT64 * W3_0 + x3i_5 as FLOAT64 * W6_0)
                        as FLOAT32;
                    x3r_5 = tmp_6;
                    x0r_5 = *data_0;
                    x0i_5 = *data_0.offset(1 as core::ffi::c_int as isize);
                    x0r_5 = x0r_5 + x2r_5;
                    x0i_5 = x0i_5 + x2i_5;
                    x2r_5 = x0r_5 - x2r_5 * 2 as core::ffi::c_int as FLOAT32;
                    x2i_5 = x0i_5 - x2i_5 * 2 as core::ffi::c_int as FLOAT32;
                    x1r_5 = x1r_5 + x3r_5;
                    x1i_5 = x1i_5 + x3i_5;
                    x3r_5 = x1r_5 - x3r_5 * 2 as core::ffi::c_int as FLOAT32;
                    x3i_5 = x1i_5 - x3i_5 * 2 as core::ffi::c_int as FLOAT32;
                    x0r_5 = x0r_5 + x1r_5;
                    x0i_5 = x0i_5 + x1i_5;
                    x1r_5 = x0r_5 - x1r_5 * 2 as core::ffi::c_int as FLOAT32;
                    x1i_5 = x0i_5 - x1i_5 * 2 as core::ffi::c_int as FLOAT32;
                    x2r_5 = x2r_5 - x3i_5;
                    x2i_5 = x2i_5 + x3r_5;
                    x3i_5 = x2r_5 + x3i_5 * 2 as core::ffi::c_int as FLOAT32;
                    x3r_5 = x2i_5 - x3r_5 * 2 as core::ffi::c_int as FLOAT32;
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
                W1_2 = *twiddles_1.offset(j as isize);
                W4_2 = *twiddles_1
                    .offset(j as isize)
                    .offset(257 as core::ffi::c_int as isize);
                W2_0 = *twiddles_1
                    .offset((j << 1 as core::ffi::c_int) as isize)
                    .offset(-(256 as core::ffi::c_int as isize));
                W5_0 = *twiddles_1
                    .offset((j << 1 as core::ffi::c_int) as isize)
                    .offset(1 as core::ffi::c_int as isize);
                W3_0 = *twiddles_1
                    .offset(j as isize)
                    .offset((j << 1 as core::ffi::c_int) as isize)
                    .offset(-(256 as core::ffi::c_int as isize));
                W6_0 = *twiddles_1
                    .offset(j as isize)
                    .offset((j << 1 as core::ffi::c_int) as isize)
                    .offset(1 as core::ffi::c_int as isize);
                k = in_loop_cnt;
                while k != 0 as core::ffi::c_int {
                    let mut tmp_7: FLOAT32 = 0.;
                    let mut x0r_6: FLOAT32 = 0.;
                    let mut x0i_6: FLOAT32 = 0.;
                    let mut x1r_6: FLOAT32 = 0.;
                    let mut x1i_6: FLOAT32 = 0.;
                    let mut x2r_6: FLOAT32 = 0.;
                    let mut x2i_6: FLOAT32 = 0.;
                    let mut x3r_6: FLOAT32 = 0.;
                    let mut x3i_6: FLOAT32 = 0.;
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
                    tmp_7 = (x1r_6 as FLOAT64 * W1_2 + x1i_6 as FLOAT64 * W4_2)
                        as FLOAT32;
                    x1i_6 = (-(x1r_6 as FLOAT64 * W4_2) + x1i_6 as FLOAT64 * W1_2)
                        as FLOAT32;
                    x1r_6 = tmp_7;
                    tmp_7 = (x2r_6 as FLOAT64 * W5_0 - x2i_6 as FLOAT64 * W2_0)
                        as FLOAT32;
                    x2i_6 = (x2r_6 as FLOAT64 * W2_0 + x2i_6 as FLOAT64 * W5_0)
                        as FLOAT32;
                    x2r_6 = tmp_7;
                    tmp_7 = (x3r_6 as FLOAT64 * W6_0 - x3i_6 as FLOAT64 * W3_0)
                        as FLOAT32;
                    x3i_6 = (x3r_6 as FLOAT64 * W3_0 + x3i_6 as FLOAT64 * W6_0)
                        as FLOAT32;
                    x3r_6 = tmp_7;
                    x0r_6 = *data_0;
                    x0i_6 = *data_0.offset(1 as core::ffi::c_int as isize);
                    x0r_6 = x0r_6 + x2r_6;
                    x0i_6 = x0i_6 + x2i_6;
                    x2r_6 = x0r_6 - x2r_6 * 2 as core::ffi::c_int as FLOAT32;
                    x2i_6 = x0i_6 - x2i_6 * 2 as core::ffi::c_int as FLOAT32;
                    x1r_6 = x1r_6 + x3r_6;
                    x1i_6 = x1i_6 + x3i_6;
                    x3r_6 = x1r_6 - x3r_6 * 2 as core::ffi::c_int as FLOAT32;
                    x3i_6 = x1i_6 - x3i_6 * 2 as core::ffi::c_int as FLOAT32;
                    x0r_6 = x0r_6 + x1r_6;
                    x0i_6 = x0i_6 + x1i_6;
                    x1r_6 = x0r_6 - x1r_6 * 2 as core::ffi::c_int as FLOAT32;
                    x1i_6 = x0i_6 - x1i_6 * 2 as core::ffi::c_int as FLOAT32;
                    x2r_6 = x2r_6 - x3i_6;
                    x2i_6 = x2i_6 + x3r_6;
                    x3i_6 = x2r_6 + x3i_6 * 2 as core::ffi::c_int as FLOAT32;
                    x3r_6 = x2i_6 - x3r_6 * 2 as core::ffi::c_int as FLOAT32;
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
                W1_2 = *twiddles_1.offset(j as isize);
                W4_2 = *twiddles_1
                    .offset(j as isize)
                    .offset(257 as core::ffi::c_int as isize);
                W2_0 = *twiddles_1
                    .offset((j << 1 as core::ffi::c_int) as isize)
                    .offset(-(256 as core::ffi::c_int as isize));
                W5_0 = *twiddles_1
                    .offset((j << 1 as core::ffi::c_int) as isize)
                    .offset(1 as core::ffi::c_int as isize);
                W3_0 = *twiddles_1
                    .offset(j as isize)
                    .offset((j << 1 as core::ffi::c_int) as isize)
                    .offset(-(512 as core::ffi::c_int as isize));
                W6_0 = *twiddles_1
                    .offset(j as isize)
                    .offset((j << 1 as core::ffi::c_int) as isize)
                    .offset(-(512 as core::ffi::c_int as isize))
                    .offset(257 as core::ffi::c_int as isize);
                k = in_loop_cnt;
                while k != 0 as core::ffi::c_int {
                    let mut tmp_8: FLOAT32 = 0.;
                    let mut x0r_7: FLOAT32 = 0.;
                    let mut x0i_7: FLOAT32 = 0.;
                    let mut x1r_7: FLOAT32 = 0.;
                    let mut x1i_7: FLOAT32 = 0.;
                    let mut x2r_7: FLOAT32 = 0.;
                    let mut x2i_7: FLOAT32 = 0.;
                    let mut x3r_7: FLOAT32 = 0.;
                    let mut x3i_7: FLOAT32 = 0.;
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
                    tmp_8 = (x1r_7 as FLOAT64 * W1_2 + x1i_7 as FLOAT64 * W4_2)
                        as FLOAT32;
                    x1i_7 = (-(x1r_7 as FLOAT64 * W4_2) + x1i_7 as FLOAT64 * W1_2)
                        as FLOAT32;
                    x1r_7 = tmp_8;
                    tmp_8 = (x2r_7 as FLOAT64 * W5_0 - x2i_7 as FLOAT64 * W2_0)
                        as FLOAT32;
                    x2i_7 = (x2r_7 as FLOAT64 * W2_0 + x2i_7 as FLOAT64 * W5_0)
                        as FLOAT32;
                    x2r_7 = tmp_8;
                    tmp_8 = (-(x3r_7 as FLOAT64 * W3_0) - x3i_7 as FLOAT64 * W6_0)
                        as FLOAT32;
                    x3i_7 = (-(x3r_7 as FLOAT64 * W6_0) + x3i_7 as FLOAT64 * W3_0)
                        as FLOAT32;
                    x3r_7 = tmp_8;
                    x0r_7 = *data_0;
                    x0i_7 = *data_0.offset(1 as core::ffi::c_int as isize);
                    x0r_7 = x0r_7 + x2r_7;
                    x0i_7 = x0i_7 + x2i_7;
                    x2r_7 = x0r_7 - x2r_7 * 2 as core::ffi::c_int as FLOAT32;
                    x2i_7 = x0i_7 - x2i_7 * 2 as core::ffi::c_int as FLOAT32;
                    x1r_7 = x1r_7 + x3r_7;
                    x1i_7 = x1i_7 - x3i_7;
                    x3r_7 = x1r_7 - x3r_7 * 2 as core::ffi::c_int as FLOAT32;
                    x3i_7 = x1i_7 + x3i_7 * 2 as core::ffi::c_int as FLOAT32;
                    x0r_7 = x0r_7 + x1r_7;
                    x0i_7 = x0i_7 + x1i_7;
                    x1r_7 = x0r_7 - x1r_7 * 2 as core::ffi::c_int as FLOAT32;
                    x1i_7 = x0i_7 - x1i_7 * 2 as core::ffi::c_int as FLOAT32;
                    x2r_7 = x2r_7 - x3i_7;
                    x2i_7 = x2i_7 + x3r_7;
                    x3i_7 = x2r_7 + x3i_7 * 2 as core::ffi::c_int as FLOAT32;
                    x3r_7 = x2i_7 - x3r_7 * 2 as core::ffi::c_int as FLOAT32;
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
            let mut twiddles_2: *const FLOAT64 = ptr_w;
            nodespacing <<= 1 as core::ffi::c_int;
            j = (del as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
            while j != 0 as core::ffi::c_int {
                let mut W1_3: FLOAT64 = *twiddles_2;
                let mut W4_3: FLOAT64 = *twiddles_2
                    .offset(257 as core::ffi::c_int as isize);
                let mut tmp_9: FLOAT32 = 0.;
                twiddles_2 = twiddles_2.offset(nodespacing as isize);
                x0r = *ptr_y;
                x0i = *ptr_y.offset(1 as core::ffi::c_int as isize);
                ptr_y = ptr_y.offset((del << 1 as core::ffi::c_int) as isize);
                x1r = *ptr_y;
                x1i = *ptr_y.offset(1 as core::ffi::c_int as isize);
                tmp_9 = (x1r as FLOAT64 * W1_3 + x1i as FLOAT64 * W4_3) as FLOAT32;
                x1i = (-(x1r as FLOAT64 * W4_3) + x1i as FLOAT64 * W1_3) as FLOAT32;
                x1r = tmp_9;
                *ptr_y = x0r - x1r;
                *ptr_y.offset(1 as core::ffi::c_int as isize) = x0i - x1i;
                ptr_y = ptr_y.offset(-((del << 1 as core::ffi::c_int) as isize));
                *ptr_y = x0r + x1r;
                *ptr_y.offset(1 as core::ffi::c_int as isize) = x0i + x1i;
                ptr_y = ptr_y.offset(2 as core::ffi::c_int as isize);
                j -= 1;
            }
            twiddles_2 = ptr_w;
            j = (del as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
            while j != 0 as core::ffi::c_int {
                let mut W1_4: FLOAT64 = *twiddles_2;
                let mut W4_4: FLOAT64 = *twiddles_2
                    .offset(257 as core::ffi::c_int as isize);
                let mut tmp_10: FLOAT32 = 0.;
                twiddles_2 = twiddles_2.offset(nodespacing as isize);
                x0r = *ptr_y;
                x0i = *ptr_y.offset(1 as core::ffi::c_int as isize);
                ptr_y = ptr_y.offset((del << 1 as core::ffi::c_int) as isize);
                x1r = *ptr_y;
                x1i = *ptr_y.offset(1 as core::ffi::c_int as isize);
                tmp_10 = (x1r as FLOAT64 * W4_4 - x1i as FLOAT64 * W1_4) as FLOAT32;
                x1i = (x1r as FLOAT64 * W1_4 + x1i as FLOAT64 * W4_4) as FLOAT32;
                x1r = tmp_10;
                *ptr_y = x0r - x1r;
                *ptr_y.offset(1 as core::ffi::c_int as isize) = x0i - x1i;
                ptr_y = ptr_y.offset(-((del << 1 as core::ffi::c_int) as isize));
                *ptr_y = x0r + x1r;
                *ptr_y.offset(1 as core::ffi::c_int as isize) = x0i + x1i;
                ptr_y = ptr_y.offset(2 as core::ffi::c_int as isize);
                j -= 1;
            }
        }
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < n_pass {
        *re
            .offset(
                (2 as core::ffi::c_int * i as core::ffi::c_int + 0 as core::ffi::c_int)
                    as isize,
            ) = *y
            .offset(
                (2 as core::ffi::c_int * i as core::ffi::c_int + 0 as core::ffi::c_int)
                    as isize,
            );
        *re
            .offset(
                (2 as core::ffi::c_int * i as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            ) = *y
            .offset(
                (2 as core::ffi::c_int * i as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hbe_apply_cfftn_gen(
    mut re: *mut FLOAT32,
    mut scratch: *mut FLOAT32,
    mut n_pass: WORD32,
    mut i_sign: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut m_points: WORD32 = n_pass;
    let mut x: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut y: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut re3: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut ptr_x: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut ptr_y: *mut FLOAT32 = 0 as *mut FLOAT32;
    x = scratch;
    ptr_x = x;
    scratch = scratch.offset((2 as WORD32 * m_points) as isize);
    y = scratch;
    ptr_y = y;
    scratch = scratch.offset((4 as WORD32 * m_points) as isize);
    re3 = scratch;
    scratch = scratch.offset((2 as WORD32 * m_points) as isize);
    let mut cnfac: WORD32 = 0;
    let mut mpass: WORD32 = n_pass;
    cnfac = 0 as core::ffi::c_int as WORD32;
    while mpass as core::ffi::c_int % 3 as core::ffi::c_int == 0 as core::ffi::c_int {
        mpass /= 3 as core::ffi::c_int;
        cnfac += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < 3 as WORD32 * cnfac {
        j = 0 as core::ffi::c_int as WORD32;
        while j < mpass {
            *re3
                .offset(
                    (2 as core::ffi::c_int * j as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ) = *re
                .offset(
                    (6 as core::ffi::c_int * j as core::ffi::c_int
                        + 2 as core::ffi::c_int * i as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                );
            *re3
                .offset(
                    (2 as core::ffi::c_int * j as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) = *re
                .offset(
                    (6 as core::ffi::c_int * j as core::ffi::c_int
                        + 2 as core::ffi::c_int * i as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                );
            j += 1;
        }
        ixheaacd_hbe_apply_cfftn(re3 as *mut FLOAT32, scratch, mpass, i_sign);
        j = 0 as core::ffi::c_int as WORD32;
        while j < mpass {
            *re
                .offset(
                    (6 as core::ffi::c_int * j as core::ffi::c_int
                        + 2 as core::ffi::c_int * i as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ) = *re3
                .offset(
                    (2 as core::ffi::c_int * j as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                );
            *re
                .offset(
                    (6 as core::ffi::c_int * j as core::ffi::c_int
                        + 2 as core::ffi::c_int * i as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) = *re3
                .offset(
                    (2 as core::ffi::c_int * j as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                );
            j += 1;
        }
        i += 1;
    }
    let mut w1r: *mut FLOAT64 = 0 as *mut FLOAT64;
    let mut w1i: *mut FLOAT64 = 0 as *mut FLOAT64;
    let mut tmp: FLOAT32 = 0.;
    w1r = ixheaac_twid_tbl_fft_ntwt3r.as_ptr() as *mut FLOAT64;
    w1i = ixheaac_twid_tbl_fft_ntwt3i.as_ptr() as *mut FLOAT64;
    if i_sign < 0 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < n_pass {
            tmp = (*re
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ) as FLOAT64 * *w1r
                - *re
                    .offset(
                        (2 as core::ffi::c_int * i as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    ) as FLOAT64 * *w1i) as FLOAT32;
            *re
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) = (*re
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ) as FLOAT64 * *w1i
                + *re
                    .offset(
                        (2 as core::ffi::c_int * i as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    ) as FLOAT64 * *w1r) as FLOAT32;
            *re
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ) = tmp;
            w1r = w1r.offset(1);
            w1i = w1i.offset(1);
            tmp = (*re
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 2 as core::ffi::c_int) as isize,
                ) as FLOAT64 * *w1r
                - *re
                    .offset(
                        (2 as core::ffi::c_int * i as core::ffi::c_int
                            + 3 as core::ffi::c_int) as isize,
                    ) as FLOAT64 * *w1i) as FLOAT32;
            *re
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 3 as core::ffi::c_int) as isize,
                ) = (*re
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 2 as core::ffi::c_int) as isize,
                ) as FLOAT64 * *w1i
                + *re
                    .offset(
                        (2 as core::ffi::c_int * i as core::ffi::c_int
                            + 3 as core::ffi::c_int) as isize,
                    ) as FLOAT64 * *w1r) as FLOAT32;
            *re
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 2 as core::ffi::c_int) as isize,
                ) = tmp;
            w1r = w1r.offset(1);
            w1i = w1i.offset(1);
            tmp = (*re
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 4 as core::ffi::c_int) as isize,
                ) as FLOAT64 * *w1r
                - *re
                    .offset(
                        (2 as core::ffi::c_int * i as core::ffi::c_int
                            + 5 as core::ffi::c_int) as isize,
                    ) as FLOAT64 * *w1i) as FLOAT32;
            *re
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 5 as core::ffi::c_int) as isize,
                ) = (*re
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 4 as core::ffi::c_int) as isize,
                ) as FLOAT64 * *w1i
                + *re
                    .offset(
                        (2 as core::ffi::c_int * i as core::ffi::c_int
                            + 5 as core::ffi::c_int) as isize,
                    ) as FLOAT64 * *w1r) as FLOAT32;
            *re
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
        while i < n_pass {
            tmp = (*re
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ) as FLOAT64 * *w1r
                + *re
                    .offset(
                        (2 as core::ffi::c_int * i as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    ) as FLOAT64 * *w1i) as FLOAT32;
            *re
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) = (-(*re
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ) as FLOAT64) * *w1i
                + *re
                    .offset(
                        (2 as core::ffi::c_int * i as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    ) as FLOAT64 * *w1r) as FLOAT32;
            *re
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ) = tmp;
            w1r = w1r.offset(1);
            w1i = w1i.offset(1);
            tmp = (*re
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 2 as core::ffi::c_int) as isize,
                ) as FLOAT64 * *w1r
                + *re
                    .offset(
                        (2 as core::ffi::c_int * i as core::ffi::c_int
                            + 3 as core::ffi::c_int) as isize,
                    ) as FLOAT64 * *w1i) as FLOAT32;
            *re
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 3 as core::ffi::c_int) as isize,
                ) = (-(*re
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 2 as core::ffi::c_int) as isize,
                ) as FLOAT64) * *w1i
                + *re
                    .offset(
                        (2 as core::ffi::c_int * i as core::ffi::c_int
                            + 3 as core::ffi::c_int) as isize,
                    ) as FLOAT64 * *w1r) as FLOAT32;
            *re
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 2 as core::ffi::c_int) as isize,
                ) = tmp;
            w1r = w1r.offset(1);
            w1i = w1i.offset(1);
            tmp = (*re
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 4 as core::ffi::c_int) as isize,
                ) as FLOAT64 * *w1r
                + *re
                    .offset(
                        (2 as core::ffi::c_int * i as core::ffi::c_int
                            + 5 as core::ffi::c_int) as isize,
                    ) as FLOAT64 * *w1i) as FLOAT32;
            *re
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 5 as core::ffi::c_int) as isize,
                ) = (-(*re
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 4 as core::ffi::c_int) as isize,
                ) as FLOAT64) * *w1i
                + *re
                    .offset(
                        (2 as core::ffi::c_int * i as core::ffi::c_int
                            + 5 as core::ffi::c_int) as isize,
                    ) as FLOAT64 * *w1r) as FLOAT32;
            *re
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
    while i < n_pass {
        *ptr_x
            .offset(
                (2 as core::ffi::c_int * i as core::ffi::c_int + 0 as core::ffi::c_int)
                    as isize,
            ) = *re
            .offset(
                (2 as core::ffi::c_int * i as core::ffi::c_int + 0 as core::ffi::c_int)
                    as isize,
            );
        *ptr_x
            .offset(
                (2 as core::ffi::c_int * i as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            ) = *re
            .offset(
                (2 as core::ffi::c_int * i as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            );
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < mpass {
        ixheaacd_hbe_apply_fft_3(ptr_x, ptr_y, i_sign);
        ptr_x = ptr_x.offset(6 as core::ffi::c_int as isize);
        ptr_y = ptr_y.offset(6 as core::ffi::c_int as isize);
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < mpass {
        *re
            .offset(
                (2 as core::ffi::c_int * i as core::ffi::c_int + 0 as core::ffi::c_int)
                    as isize,
            ) = *y
            .offset(
                (6 as core::ffi::c_int * i as core::ffi::c_int + 0 as core::ffi::c_int)
                    as isize,
            );
        *re
            .offset(
                (2 as core::ffi::c_int * i as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            ) = *y
            .offset(
                (6 as core::ffi::c_int * i as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            );
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < mpass {
        *re
            .offset(
                (2 as core::ffi::c_int * mpass as core::ffi::c_int
                    + 2 as core::ffi::c_int * i as core::ffi::c_int
                    + 0 as core::ffi::c_int) as isize,
            ) = *y
            .offset(
                (6 as core::ffi::c_int * i as core::ffi::c_int + 2 as core::ffi::c_int)
                    as isize,
            );
        *re
            .offset(
                (2 as core::ffi::c_int * mpass as core::ffi::c_int
                    + 2 as core::ffi::c_int * i as core::ffi::c_int
                    + 1 as core::ffi::c_int) as isize,
            ) = *y
            .offset(
                (6 as core::ffi::c_int * i as core::ffi::c_int + 3 as core::ffi::c_int)
                    as isize,
            );
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < mpass {
        *re
            .offset(
                (4 as core::ffi::c_int * mpass as core::ffi::c_int
                    + 2 as core::ffi::c_int * i as core::ffi::c_int
                    + 0 as core::ffi::c_int) as isize,
            ) = *y
            .offset(
                (6 as core::ffi::c_int * i as core::ffi::c_int + 4 as core::ffi::c_int)
                    as isize,
            );
        *re
            .offset(
                (4 as core::ffi::c_int * mpass as core::ffi::c_int
                    + 2 as core::ffi::c_int * i as core::ffi::c_int
                    + 1 as core::ffi::c_int) as isize,
            ) = *y
            .offset(
                (6 as core::ffi::c_int * i as core::ffi::c_int + 5 as core::ffi::c_int)
                    as isize,
            );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hbe_apply_fft_288(
    mut inp: *mut FLOAT32,
    mut scratch: *mut FLOAT32,
    mut len: WORD32,
    mut i_sign: WORD32,
) -> VOID {
    let mut op: *mut FLOAT32 = scratch;
    let mut mpoints: WORD32 = len / 96 as WORD32;
    let mut fpoints: WORD32 = len / 3 as WORD32;
    let mut ii: WORD32 = 0;
    let mut jj: WORD32 = 0;
    scratch = scratch.offset((2 as WORD32 * len) as isize);
    ii = 0 as core::ffi::c_int as WORD32;
    while ii < mpoints {
        jj = 0 as core::ffi::c_int as WORD32;
        while jj < fpoints {
            *op
                .offset(
                    (2 as core::ffi::c_int * jj as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ) = *inp
                .offset((2 as WORD32 * mpoints * jj + 2 as WORD32 * ii) as isize);
            *op
                .offset(
                    (2 as core::ffi::c_int * jj as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) = *inp
                .offset(
                    (2 as core::ffi::c_int * mpoints as core::ffi::c_int
                        * jj as core::ffi::c_int
                        + 2 as core::ffi::c_int * ii as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                );
            jj += 1;
        }
        if fpoints as core::ffi::c_int
            & fpoints as core::ffi::c_int - 1 as core::ffi::c_int != 0
        {
            ixheaacd_hbe_apply_cfftn_gen(op as *mut FLOAT32, scratch, fpoints, i_sign);
        } else {
            ixheaacd_hbe_apply_cfftn(op as *mut FLOAT32, scratch, fpoints, i_sign);
        }
        jj = 0 as core::ffi::c_int as WORD32;
        while jj < fpoints {
            *inp
                .offset(
                    (mpoints as core::ffi::c_int * 2 as core::ffi::c_int
                        * jj as core::ffi::c_int
                        + 2 as core::ffi::c_int * ii as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ) = *op
                .offset(
                    (2 as core::ffi::c_int * jj as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                );
            *inp
                .offset(
                    (mpoints as core::ffi::c_int * 2 as core::ffi::c_int
                        * jj as core::ffi::c_int
                        + 2 as core::ffi::c_int * ii as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) = *op
                .offset(
                    (2 as core::ffi::c_int * jj as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                );
            jj += 1;
        }
        ii += 1;
    }
    ixheaacd_hbe_apply_tw_mult_fft(
        inp,
        op,
        fpoints,
        mpoints,
        ixheaac_twid_tbl_fft_288.as_ptr(),
    );
    ii = 0 as core::ffi::c_int as WORD32;
    while ii < fpoints {
        ixheaacd_hbe_apply_fft_3(op, scratch, i_sign);
        op = op.offset((mpoints as core::ffi::c_int * 2 as core::ffi::c_int) as isize);
        scratch = scratch
            .offset((mpoints as core::ffi::c_int * 2 as core::ffi::c_int) as isize);
        ii += 1;
    }
    scratch = scratch
        .offset(
            -((fpoints as core::ffi::c_int * mpoints as core::ffi::c_int
                * 2 as core::ffi::c_int) as isize),
        );
    jj = 0 as core::ffi::c_int as WORD32;
    while jj < fpoints {
        *inp
            .offset(
                (2 as core::ffi::c_int * jj as core::ffi::c_int + 0 as core::ffi::c_int)
                    as isize,
            ) = *scratch.offset((6 as WORD32 * jj) as isize);
        *inp
            .offset(
                (2 as core::ffi::c_int * jj as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            ) = *scratch
            .offset(
                (6 as core::ffi::c_int * jj as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            );
        jj += 1;
    }
    jj = 0 as core::ffi::c_int as WORD32;
    while jj < fpoints {
        *inp
            .offset(
                (2 as core::ffi::c_int * fpoints as core::ffi::c_int
                    + 2 as core::ffi::c_int * jj as core::ffi::c_int
                    + 0 as core::ffi::c_int) as isize,
            ) = *scratch
            .offset(
                (6 as core::ffi::c_int * jj as core::ffi::c_int + 2 as core::ffi::c_int)
                    as isize,
            );
        *inp
            .offset(
                (2 as core::ffi::c_int * fpoints as core::ffi::c_int
                    + 2 as core::ffi::c_int * jj as core::ffi::c_int
                    + 1 as core::ffi::c_int) as isize,
            ) = *scratch
            .offset(
                (6 as core::ffi::c_int * jj as core::ffi::c_int + 3 as core::ffi::c_int)
                    as isize,
            );
        jj += 1;
    }
    jj = 0 as core::ffi::c_int as WORD32;
    while jj < fpoints {
        *inp
            .offset(
                (4 as core::ffi::c_int * fpoints as core::ffi::c_int
                    + 2 as core::ffi::c_int * jj as core::ffi::c_int
                    + 0 as core::ffi::c_int) as isize,
            ) = *scratch
            .offset(
                (6 as core::ffi::c_int * jj as core::ffi::c_int + 4 as core::ffi::c_int)
                    as isize,
            );
        *inp
            .offset(
                (4 as core::ffi::c_int * fpoints as core::ffi::c_int
                    + 2 as core::ffi::c_int * jj as core::ffi::c_int
                    + 1 as core::ffi::c_int) as isize,
            ) = *scratch
            .offset(
                (6 as core::ffi::c_int * jj as core::ffi::c_int + 5 as core::ffi::c_int)
                    as isize,
            );
        jj += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hbe_apply_ifft_224(
    mut inp: *mut FLOAT32,
    mut scratch: *mut FLOAT32,
    mut len: WORD32,
    mut i_sign: WORD32,
) -> VOID {
    let mut mpoints: WORD32 = len / 32 as WORD32;
    let mut fpoints: WORD32 = len / 7 as WORD32;
    let mut ii: WORD32 = 0;
    let mut jj: WORD32 = 0;
    let mut op: *mut FLOAT32 = scratch;
    scratch = scratch.offset((2 as WORD32 * len) as isize);
    ii = 0 as core::ffi::c_int as WORD32;
    while ii < mpoints {
        jj = 0 as core::ffi::c_int as WORD32;
        while jj < fpoints {
            *op
                .offset(
                    (2 as core::ffi::c_int * jj as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ) = *inp
                .offset((2 as WORD32 * mpoints * jj + 2 as WORD32 * ii) as isize);
            *op
                .offset(
                    (2 as core::ffi::c_int * jj as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) = *inp
                .offset(
                    (2 as core::ffi::c_int * mpoints as core::ffi::c_int
                        * jj as core::ffi::c_int
                        + 2 as core::ffi::c_int * ii as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                );
            jj += 1;
        }
        if fpoints as core::ffi::c_int
            & fpoints as core::ffi::c_int - 1 as core::ffi::c_int != 0
        {
            ixheaacd_hbe_apply_cfftn_gen(op as *mut FLOAT32, scratch, fpoints, i_sign);
        } else {
            ixheaacd_hbe_apply_cfftn(op as *mut FLOAT32, scratch, fpoints, i_sign);
        }
        jj = 0 as core::ffi::c_int as WORD32;
        while jj < fpoints {
            *inp
                .offset(
                    (mpoints as core::ffi::c_int * 2 as core::ffi::c_int
                        * jj as core::ffi::c_int
                        + 2 as core::ffi::c_int * ii as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ) = *op
                .offset(
                    (2 as core::ffi::c_int * jj as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                );
            *inp
                .offset(
                    (mpoints as core::ffi::c_int * 2 as core::ffi::c_int
                        * jj as core::ffi::c_int
                        + 2 as core::ffi::c_int * ii as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) = *op
                .offset(
                    (2 as core::ffi::c_int * jj as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                );
            jj += 1;
        }
        ii += 1;
    }
    ixheaacd_hbe_apply_tw_mult_ifft(
        inp,
        op,
        fpoints,
        mpoints,
        ixheaac_twid_tbl_fft_224.as_ptr(),
    );
    ii = 0 as core::ffi::c_int as WORD32;
    while ii < fpoints {
        ixheaacd_hbe_apply_ifft_7(op, scratch);
        scratch = scratch
            .offset((mpoints as core::ffi::c_int * 2 as core::ffi::c_int) as isize);
        op = op.offset((mpoints as core::ffi::c_int * 2 as core::ffi::c_int) as isize);
        ii += 1;
    }
    scratch = scratch
        .offset(
            -((fpoints as core::ffi::c_int * mpoints as core::ffi::c_int
                * 2 as core::ffi::c_int) as isize),
        );
    jj = 0 as core::ffi::c_int as WORD32;
    while jj < fpoints {
        ii = 0 as core::ffi::c_int as WORD32;
        while ii < mpoints {
            *inp
                .offset(
                    (fpoints as core::ffi::c_int * ii as core::ffi::c_int
                        * 2 as core::ffi::c_int
                        + 2 as core::ffi::c_int * jj as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ) = *scratch
                .offset(
                    (mpoints as core::ffi::c_int * jj as core::ffi::c_int
                        * 2 as core::ffi::c_int
                        + 2 as core::ffi::c_int * ii as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                );
            *inp
                .offset(
                    (fpoints as core::ffi::c_int * ii as core::ffi::c_int
                        * 2 as core::ffi::c_int
                        + 2 as core::ffi::c_int * jj as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) = *scratch
                .offset(
                    (mpoints as core::ffi::c_int * jj as core::ffi::c_int
                        * 2 as core::ffi::c_int
                        + 2 as core::ffi::c_int * ii as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                );
            ii += 1;
        }
        jj += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hbe_apply_ifft_336(
    mut inp: *mut FLOAT32,
    mut ptr_scratch: *mut FLOAT32,
    mut len: WORD32,
    mut i_sign: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut m_points: WORD32 = len / 7 as WORD32;
    let mut n_points: WORD32 = len / 48 as WORD32;
    let mut ptr_real: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut ptr_imag: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut p_real_1: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut p_scratch: *mut FLOAT32 = 0 as *mut FLOAT32;
    ptr_real = ptr_scratch;
    ptr_scratch = ptr_scratch.offset((2 as WORD32 * len) as isize);
    ptr_imag = ptr_scratch;
    ptr_scratch = ptr_scratch.offset(len as isize);
    p_scratch = ptr_scratch;
    ptr_scratch = ptr_scratch.offset(len as isize);
    p_real_1 = ptr_scratch;
    ptr_scratch = ptr_scratch.offset(len as isize);
    i = 0 as core::ffi::c_int as WORD32;
    while i < len {
        *ptr_real.offset(i as isize) = *inp
            .offset(
                (2 as core::ffi::c_int * i as core::ffi::c_int + 0 as core::ffi::c_int)
                    as isize,
            );
        *ptr_imag.offset(i as isize) = *inp
            .offset(
                (2 as core::ffi::c_int * i as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            );
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < m_points {
        j = 0 as core::ffi::c_int as WORD32;
        while j < n_points {
            *p_real_1
                .offset(
                    (2 as core::ffi::c_int * j as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ) = *inp
                .offset(
                    (m_points as core::ffi::c_int * 2 as core::ffi::c_int
                        * j as core::ffi::c_int
                        + 2 as core::ffi::c_int * i as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                );
            *p_real_1
                .offset(
                    (2 as core::ffi::c_int * j as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) = *inp
                .offset(
                    (m_points as core::ffi::c_int * 2 as core::ffi::c_int
                        * j as core::ffi::c_int
                        + 2 as core::ffi::c_int * i as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                );
            j += 1;
        }
        ixheaacd_hbe_apply_ifft_7(p_real_1, ptr_scratch);
        j = 0 as core::ffi::c_int as WORD32;
        while j < n_points {
            *inp
                .offset(
                    (m_points as core::ffi::c_int * 2 as core::ffi::c_int
                        * j as core::ffi::c_int
                        + 2 as core::ffi::c_int * i as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ) = *ptr_scratch
                .offset(
                    (2 as core::ffi::c_int * j as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                );
            *inp
                .offset(
                    (m_points as core::ffi::c_int * 2 as core::ffi::c_int
                        * j as core::ffi::c_int
                        + 2 as core::ffi::c_int * i as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) = *ptr_scratch
                .offset(
                    (2 as core::ffi::c_int * j as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                );
            j += 1;
        }
        i += 1;
    }
    if m_points == 48 as core::ffi::c_int {
        ixheaacd_hbe_apply_tw_mult_ifft(
            inp,
            p_scratch,
            n_points,
            m_points,
            ixheaac_twid_tbl_fft_336.as_ptr(),
        );
    } else {
        ixheaacd_hbe_apply_tw_mult_ifft(
            inp,
            p_scratch,
            n_points,
            m_points,
            ixheaac_twid_tbl_fft_168.as_ptr(),
        );
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < len {
        *ptr_real
            .offset(
                (2 as core::ffi::c_int * i as core::ffi::c_int + 0 as core::ffi::c_int)
                    as isize,
            ) = *p_scratch
            .offset(
                (2 as core::ffi::c_int * i as core::ffi::c_int + 0 as core::ffi::c_int)
                    as isize,
            );
        *ptr_real
            .offset(
                (2 as core::ffi::c_int * i as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            ) = *p_scratch
            .offset(
                (2 as core::ffi::c_int * i as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            );
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < n_points {
        ixheaacd_hbe_apply_cfftn_gen(
            ptr_real as *mut FLOAT32,
            ptr_scratch,
            m_points,
            i_sign,
        );
        ptr_real = ptr_real.offset((2 as WORD32 * m_points) as isize);
        i += 1;
    }
    ptr_real = ptr_real.offset(-((n_points * 2 as WORD32 * m_points) as isize));
    j = 0 as core::ffi::c_int as WORD32;
    while j < n_points {
        i = 0 as core::ffi::c_int as WORD32;
        while i < m_points {
            *inp
                .offset(
                    (n_points as core::ffi::c_int * 2 as core::ffi::c_int
                        * i as core::ffi::c_int
                        + 2 as core::ffi::c_int * j as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ) = *ptr_real
                .offset(
                    (2 as core::ffi::c_int * m_points as core::ffi::c_int
                        * j as core::ffi::c_int
                        + 2 as core::ffi::c_int * i as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                );
            *inp
                .offset(
                    (n_points as core::ffi::c_int * 2 as core::ffi::c_int
                        * i as core::ffi::c_int
                        + 2 as core::ffi::c_int * j as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) = *ptr_real
                .offset(
                    (2 as core::ffi::c_int * m_points as core::ffi::c_int
                        * j as core::ffi::c_int
                        + 2 as core::ffi::c_int * i as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                );
            i += 1;
        }
        j += 1;
    }
}
pub const C70: core::ffi::c_float = -0.1666667014f32;
pub const C71: core::ffi::c_float = 0.7901564837f32;
pub const C72: core::ffi::c_float = 0.0558542535f32;
pub const C73: core::ffi::c_float = 0.7343022227f32;
pub const C74: core::ffi::c_float = -0.4409585893f32;
pub const C75: core::ffi::c_float = -0.3408728838f32;
pub const C76: core::ffi::c_float = 0.5339693427f32;
pub const C77: core::ffi::c_float = -0.8748422265f32;
