extern "C" {
    fn cos(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn sin(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn sqrt(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn ixheaacd_complex_fft(
        data_r: *mut WORD32,
        data_i: *mut WORD32,
        len: WORD32,
        fft_mode: WORD32,
        preshift: *mut WORD32,
    ) -> VOID;
}
pub type size_t = usize;
pub type WORD8 = core::ffi::c_schar;
pub type WORD32 = core::ffi::c_int;
pub type WORD64 = core::ffi::c_longlong;
pub type FLOAT32 = core::ffi::c_float;
pub type FLOAT64 = core::ffi::c_double;
pub type VOID = ();
pub type WORD = core::ffi::c_int;
pub const ORDER: core::ffi::c_int = 16 as core::ffi::c_int;
pub const ORDER_BY_2: core::ffi::c_int = ORDER / 2 as core::ffi::c_int;
pub const PI: core::ffi::c_double = 3.14159265358979323846264338327950288f64;
unsafe extern "C" fn ixheaacd_compute_coeff_poly_f(
    mut lsp: *mut FLOAT32,
    mut f1: *mut FLOAT32,
    mut f2: *mut FLOAT32,
) -> VOID {
    let mut b1: FLOAT32 = 0.;
    let mut b2: FLOAT32 = 0.;
    let mut ptr_lsp: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    ptr_lsp = lsp as *mut FLOAT32;
    let ref mut fresh5 = *f2.offset(0 as core::ffi::c_int as isize);
    *fresh5 = 1.0f32 as FLOAT32;
    *f1.offset(0 as core::ffi::c_int as isize) = *fresh5;
    i = 1 as core::ffi::c_int as WORD32;
    while i <= ORDER_BY_2 {
        let fresh6 = ptr_lsp;
        ptr_lsp = ptr_lsp.offset(1);
        b1 = -2.0f32 * *fresh6;
        let fresh7 = ptr_lsp;
        ptr_lsp = ptr_lsp.offset(1);
        b2 = -2.0f32 * *fresh7;
        *f1.offset(i as isize) = b1
            * *f1.offset((i as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
            + 2.0f32
                * *f1.offset((i as core::ffi::c_int - 2 as core::ffi::c_int) as isize);
        *f2.offset(i as isize) = b2
            * *f2.offset((i as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
            + 2.0f32
                * *f2.offset((i as core::ffi::c_int - 2 as core::ffi::c_int) as isize);
        j = (i as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        while j > 0 as core::ffi::c_int {
            *f1.offset(j as isize)
                += b1
                    * *f1
                        .offset((j as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
                    + *f1
                        .offset(
                            (j as core::ffi::c_int - 2 as core::ffi::c_int) as isize,
                        );
            *f2.offset(j as isize)
                += b2
                    * *f2
                        .offset((j as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
                    + *f2
                        .offset(
                            (j as core::ffi::c_int - 2 as core::ffi::c_int) as isize,
                        );
            j -= 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_lsp_to_lp_conversion(
    mut lsp: *mut FLOAT32,
    mut lp_flt_coff_a: *mut FLOAT32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut ppoly_f1: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut ppoly_f2: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut plp_flt_coff_a_bott: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut plp_flt_coff_a_top: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut poly1: [FLOAT32; 10] = [0.; 10];
    let mut poly2: [FLOAT32; 10] = [0.; 10];
    poly1[0 as core::ffi::c_int as usize] = 0.0f32 as FLOAT32;
    poly2[0 as core::ffi::c_int as usize] = 0.0f32 as FLOAT32;
    ixheaacd_compute_coeff_poly_f(
        lsp as *mut FLOAT32,
        &mut *poly1.as_mut_ptr().offset(1 as core::ffi::c_int as isize),
        &mut *poly2.as_mut_ptr().offset(1 as core::ffi::c_int as isize),
    );
    ppoly_f1 = poly1
        .as_mut_ptr()
        .offset(ORDER_BY_2 as isize)
        .offset(1 as core::ffi::c_int as isize);
    ppoly_f2 = poly2
        .as_mut_ptr()
        .offset(ORDER_BY_2 as isize)
        .offset(1 as core::ffi::c_int as isize);
    i = 0 as core::ffi::c_int as WORD32;
    while i < ORDER_BY_2 {
        *ppoly_f1.offset(0 as core::ffi::c_int as isize)
            += *ppoly_f1.offset(-(1 as core::ffi::c_int) as isize);
        *ppoly_f2.offset(0 as core::ffi::c_int as isize)
            -= *ppoly_f2.offset(-(1 as core::ffi::c_int) as isize);
        ppoly_f1 = ppoly_f1.offset(-1);
        ppoly_f2 = ppoly_f2.offset(-1);
        i += 1;
    }
    plp_flt_coff_a_bott = lp_flt_coff_a;
    let fresh0 = plp_flt_coff_a_bott;
    plp_flt_coff_a_bott = plp_flt_coff_a_bott.offset(1);
    *fresh0 = 1.0f32 as FLOAT32;
    plp_flt_coff_a_top = lp_flt_coff_a.offset(ORDER as isize);
    ppoly_f1 = poly1.as_mut_ptr().offset(2 as core::ffi::c_int as isize);
    ppoly_f2 = poly2.as_mut_ptr().offset(2 as core::ffi::c_int as isize);
    i = 0 as core::ffi::c_int as WORD32;
    while i < ORDER_BY_2 {
        let fresh1 = plp_flt_coff_a_bott;
        plp_flt_coff_a_bott = plp_flt_coff_a_bott.offset(1);
        *fresh1 = 0.5f32 * (*ppoly_f1 + *ppoly_f2);
        let fresh2 = ppoly_f1;
        ppoly_f1 = ppoly_f1.offset(1);
        let fresh3 = ppoly_f2;
        ppoly_f2 = ppoly_f2.offset(1);
        let fresh4 = plp_flt_coff_a_top;
        plp_flt_coff_a_top = plp_flt_coff_a_top.offset(-1);
        *fresh4 = 0.5f32 * (*fresh2 - *fresh3);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_lpc_to_td(
    mut coeff: *mut FLOAT32,
    mut order: WORD32,
    mut gains: *mut FLOAT32,
    mut lg: WORD32,
) -> VOID {
    let mut data_r: [FLOAT32; 2048] = [0.; 2048];
    let mut data_i: [FLOAT32; 2048] = [0.; 2048];
    let mut avg_fac: FLOAT64 = 0.;
    let mut idata_r: [WORD32; 2048] = [0; 2048];
    let mut idata_i: [WORD32; 2048] = [0; 2048];
    let mut qshift: WORD8 = 0;
    let mut preshift: WORD32 = 0 as WORD32;
    let mut itemp: WORD32 = 0;
    let mut ftemp: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
    let mut tmp: FLOAT32 = 0.;
    let mut qfac: FLOAT32 = 0.;
    let mut i: WORD32 = 0;
    let mut size_n: WORD32 = 0;
    size_n = 2 as WORD32 * lg;
    avg_fac = (PI / size_n as FLOAT32 as core::ffi::c_double) as FLOAT64;
    i = 0 as core::ffi::c_int as WORD32;
    while i < order as core::ffi::c_int + 1 as core::ffi::c_int {
        tmp = (i as FLOAT32 as FLOAT64 * avg_fac) as FLOAT32;
        data_r[i as usize] = (*coeff.offset(i as isize) as core::ffi::c_double
            * cos(tmp as core::ffi::c_double)) as FLOAT32;
        data_i[i as usize] = (-*coeff.offset(i as isize) as core::ffi::c_double
            * sin(tmp as core::ffi::c_double)) as FLOAT32;
        i += 1;
    }
    while i < size_n {
        data_r[i as usize] = 0.0f32 as FLOAT32;
        data_i[i as usize] = 0.0f32 as FLOAT32;
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < size_n {
        if (if data_r[i as usize] < 0 as core::ffi::c_int as FLOAT32 {
            -data_r[i as usize]
        } else {
            data_r[i as usize]
        }) > ftemp
        {
            ftemp = if data_r[i as usize] < 0 as core::ffi::c_int as FLOAT32 {
                -data_r[i as usize]
            } else {
                data_r[i as usize]
            };
        }
        if (if data_i[i as usize] < 0 as core::ffi::c_int as FLOAT32 {
            -data_i[i as usize]
        } else {
            data_i[i as usize]
        }) > ftemp
        {
            ftemp = if data_i[i as usize] < 0 as core::ffi::c_int as FLOAT32 {
                -data_i[i as usize]
            } else {
                data_i[i as usize]
            };
        }
        i += 1;
    }
    itemp = ftemp as WORD32;
    qshift = ixheaac_norm32(itemp) as WORD8;
    i = 0 as core::ffi::c_int as WORD32;
    while i < size_n {
        idata_r[i as usize] = (data_r[i as usize]
            * ((1 as core::ffi::c_int as WORD64) << qshift as core::ffi::c_int)
                as FLOAT32) as WORD32;
        idata_i[i as usize] = (data_i[i as usize]
            * ((1 as core::ffi::c_int as WORD64) << qshift as core::ffi::c_int)
                as FLOAT32) as WORD32;
        i += 1;
    }
    ixheaacd_complex_fft(
        idata_r.as_mut_ptr(),
        idata_i.as_mut_ptr(),
        size_n,
        -(1 as WORD32),
        &mut preshift,
    );
    qfac = 1.0f32
        / ((1 as core::ffi::c_int as WORD64) << qshift as WORD32 - preshift) as FLOAT32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < size_n {
        data_r[i as usize] = idata_r[i as usize] as FLOAT32 * qfac;
        data_i[i as usize] = idata_i[i as usize] as FLOAT32 * qfac;
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < size_n as core::ffi::c_int / 2 as core::ffi::c_int {
        *gains.offset(i as isize) = (1.0f64
            / (sqrt(
                (data_r[i as usize] * data_r[i as usize]
                    + data_i[i as usize] * data_i[i as usize]) as core::ffi::c_double,
            ) + FLT_EPSILON as core::ffi::c_double)) as FLOAT32;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_noise_shaping(
    mut r: *mut FLOAT32,
    mut lg: WORD32,
    mut M: WORD32,
    mut g1: *mut FLOAT32,
    mut g2: *mut FLOAT32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut rr_prev: FLOAT32 = 0.;
    let mut a: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
    let mut b: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
    let mut rr: [FLOAT32; 1024] = [0.; 1024];
    k = lg / M;
    rr_prev = 0 as core::ffi::c_int as FLOAT32;
    memcpy(
        &mut rr as *mut [FLOAT32; 1024] as *mut core::ffi::c_void,
        r as *const core::ffi::c_void,
        (lg as size_t).wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
    );
    i = 0 as core::ffi::c_int as WORD32;
    while i < lg {
        if i % k == 0 as core::ffi::c_int {
            a = 2.0f32 * *g1.offset((i / k) as isize) * *g2.offset((i / k) as isize)
                / (*g1.offset((i / k) as isize) + *g2.offset((i / k) as isize));
            b = (*g2.offset((i / k) as isize) - *g1.offset((i / k) as isize))
                / (*g1.offset((i / k) as isize) + *g2.offset((i / k) as isize));
        }
        rr[i as usize] = a * rr[i as usize] + b * rr_prev;
        rr_prev = rr[i as usize];
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < lg as core::ffi::c_int / 2 as core::ffi::c_int {
        *r.offset(i as isize) = rr[(2 as WORD32 * i) as usize];
        *r.offset((lg / 2 as WORD32 + i) as isize) = rr[(lg as core::ffi::c_int
            - 2 as core::ffi::c_int * i as core::ffi::c_int - 1 as core::ffi::c_int)
            as usize];
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_lpc_coef_gen(
    mut lsf_old: *mut FLOAT32,
    mut lsf_new: *mut FLOAT32,
    mut a: *mut FLOAT32,
    mut nb_subfr: WORD32,
    mut m: WORD32,
) -> VOID {
    let mut lsf: [FLOAT32; 16] = [0.; 16];
    let mut ptr_a: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut inc: FLOAT32 = 0.;
    let mut fnew: FLOAT32 = 0.;
    let mut fold: FLOAT32 = 0.;
    let mut i: WORD32 = 0;
    ptr_a = a as *mut FLOAT32;
    inc = 1.0f32 / nb_subfr as FLOAT32;
    fnew = 0.5f32 - 0.5f32 * inc;
    fold = 1.0f32 - fnew;
    i = 0 as core::ffi::c_int as WORD32;
    while i < m {
        lsf[i as usize] = *lsf_old.offset(i as isize) * fold
            + *lsf_new.offset(i as isize) * fnew;
        i += 1;
    }
    ixheaacd_lsp_to_lp_conversion(lsf.as_mut_ptr(), ptr_a);
    ptr_a = ptr_a.offset((m as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
    ixheaacd_lsp_to_lp_conversion(lsf_old as *mut FLOAT32, ptr_a);
    ptr_a = ptr_a.offset((m as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
    ixheaacd_lsp_to_lp_conversion(lsf_new as *mut FLOAT32, ptr_a);
    ptr_a = ptr_a.offset((m as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_interpolation_lsp_params(
    mut lsp_old: *mut FLOAT32,
    mut lsp_new: *mut FLOAT32,
    mut lp_flt_coff_a: *mut FLOAT32,
    mut nb_subfr: WORD32,
) -> VOID {
    let mut lsp: [FLOAT32; 16] = [0.; 16];
    let mut factor: FLOAT32 = 0.;
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut x_plus_y: FLOAT32 = 0.;
    let mut x_minus_y: FLOAT32 = 0.;
    factor = 1.0f32 / nb_subfr as FLOAT32;
    x_plus_y = 0.5f32 * factor;
    k = 0 as core::ffi::c_int as WORD32;
    while k < nb_subfr {
        x_minus_y = 1.0f32 - x_plus_y;
        i = 0 as core::ffi::c_int as WORD32;
        while i < ORDER {
            lsp[i as usize] = *lsp_old.offset(i as isize) * x_minus_y
                + *lsp_new.offset(i as isize) * x_plus_y;
            i += 1;
        }
        x_plus_y += factor;
        ixheaacd_lsp_to_lp_conversion(lsp.as_mut_ptr(), lp_flt_coff_a as *mut FLOAT32);
        lp_flt_coff_a = lp_flt_coff_a.offset((ORDER + 1 as core::ffi::c_int) as isize);
        k += 1;
    }
    ixheaacd_lsp_to_lp_conversion(
        lsp_new as *mut FLOAT32,
        lp_flt_coff_a as *mut FLOAT32,
    );
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
pub const FLT_EPSILON: core::ffi::c_float = __FLT_EPSILON__;
pub const __FLT_EPSILON__: core::ffi::c_float = 1.19209290e-7f32;
