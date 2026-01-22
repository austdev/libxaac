extern "C" {
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memmove(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    static ixheaacd_ia_mps_hyb_filter_coeff_8: [FLOAT32; 13];
    static ixheaacd_mps_hyb_filter_coeff_2: [FLOAT32; 13];
    static ixheaacd_sine: [[FLOAT32; 8]; 8];
    static ixheaacd_cosine: [[FLOAT32; 8]; 8];
}
pub type size_t = usize;
pub type WORD16 = core::ffi::c_short;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type WORD64 = core::ffi::c_longlong;
pub type FLOAT32 = core::ffi::c_float;
pub type VOID = ();
pub type LOOPIDX = core::ffi::c_int;
pub type WORD = core::ffi::c_int;
pub type LOOPINDEX = LOOPIDX;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_hybrid_tables_struct {
    pub p8_13: [WORD16; 19],
    pub p2_6: [WORD16; 6],
    pub sine_array: [WORD32; 2048],
    pub cosine_array: [WORD32; 2048],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_mdct2qmf_table_struct {
    pub twi_post_cos: [WORD32; 64],
    pub twi_post_sin: [WORD32; 64],
    pub hybrid_2_qmf: [WORD32; 71],
    pub local_sin_4: [WORD32; 4],
    pub local_sin_15: [WORD32; 16],
    pub local_sin_16: [WORD32; 16],
    pub local_sin_18: [WORD32; 18],
    pub local_sin_24: [WORD32; 24],
    pub local_sin_30: [WORD32; 30],
    pub local_sin_32: [WORD32; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_cmplx_flt_struct {
    pub re: FLOAT32,
    pub im: FLOAT32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_cmplx_w32_struct {
    pub re: WORD32,
    pub im: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_hybrid_filt_struct {
    pub hf_buffer: [[ia_cmplx_flt_struct; 78]; 128],
    pub lf_buffer: [[ia_cmplx_flt_struct; 84]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_thyb_filter_state_struct {
    pub buffer_lf_real: [[WORD32; 84]; 3],
    pub buffer_lf_imag: [[WORD32; 84]; 3],
    pub qmf_lf_real: [[WORD32; 84]; 3],
    pub qmf_lf_imag: [[WORD32; 84]; 3],
    pub buffer_hf_real: [[WORD32; 78]; 64],
    pub buffer_hf_imag: [[WORD32; 78]; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct complex {
    pub re: WORD32,
    pub im: WORD32,
}
pub const HYBRID_FILTER_DELAY: core::ffi::c_int = 6 as core::ffi::c_int;
pub const QMF_BANDS_TO_HYBRID: core::ffi::c_int = 3 as core::ffi::c_int;
pub const MAX_NUM_QMF_BANDS_SAC: core::ffi::c_int = 128 as core::ffi::c_int;
pub const MAX_32: WORD32 = 0x7fffffff as core::ffi::c_long as WORD32;
pub const MIN_32: WORD32 = 0x80000000 as core::ffi::c_long as WORD32;
pub const MAX_NUM_QMF_BANDS_MPS: core::ffi::c_int = 128 as core::ffi::c_int;
pub const BUFFER_LEN_HF_MPS: core::ffi::c_int = (QMF_HYBRID_FILT_ORDER
    - 1 as core::ffi::c_int) / 2 as core::ffi::c_int + MAX_TIME_SLOTS;
pub const MAX_TIME_SLOTS: core::ffi::c_int = 72 as core::ffi::c_int;
pub const QMF_HYBRID_FILT_ORDER: core::ffi::c_int = 13 as core::ffi::c_int;
pub const BUFFER_LEN_LF_MPS: core::ffi::c_int = QMF_HYBRID_FILT_ORDER
    - 1 as core::ffi::c_int + MAX_TIME_SLOTS;
pub const MAX_NUM_QMF_BANDS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const MAX_HYBRID_BANDS: core::ffi::c_int = MAX_NUM_QMF_BANDS - 3 as core::ffi::c_int
    + 10 as core::ffi::c_int;
pub const PROTO_LEN: core::ffi::c_int = 13 as core::ffi::c_int;
pub const BUFFER_LEN_LF: core::ffi::c_int = PROTO_LEN - 1 as core::ffi::c_int
    + MAX_TIME_SLOTS;
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
unsafe extern "C" fn ixheaac_mult32x16in32(mut a: WORD32, mut b: WORD16) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 16 as core::ffi::c_int) as WORD32;
    return result;
}
pub const NR_QMF_BANDS_LFXTS: core::ffi::c_int = 216 as core::ffi::c_int;
pub const COS_PI_BY_8: core::ffi::c_int = 0x7642 as core::ffi::c_int;
pub const SIN_PI_BY_8: core::ffi::c_int = 0x30fc as core::ffi::c_int;
#[inline]
unsafe extern "C" fn ixheaacd_mps_mult32x16_shr_16(
    mut a: WORD32,
    mut b: WORD32,
) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 16 as core::ffi::c_int) as WORD32;
    return result;
}
unsafe extern "C" fn ixheaacd_mps_hyb_filt_type1(
    mut input: *mut ia_cmplx_flt_struct,
    mut output: *mut [ia_cmplx_flt_struct; 72],
    mut num_samples: WORD32,
    mut filt_coeff: *const FLOAT32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut q: WORD32 = 0;
    let mut in_re: FLOAT32 = 0.;
    let mut in_im: FLOAT32 = 0.;
    let mut coeff: FLOAT32 = 0.;
    let mut acc_re_l: FLOAT32 = 0.;
    let mut acc_re_h: FLOAT32 = 0.;
    let mut acc_im_l: FLOAT32 = 0.;
    let mut acc_im_h: FLOAT32 = 0.;
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_samples {
        let mut x0_re: [FLOAT32; 13] = [0.; 13];
        let mut x0_im: [FLOAT32; 13] = [0.; 13];
        let mut x0_1_re: [FLOAT32; 8] = [0.; 8];
        let mut x0_1_im: [FLOAT32; 8] = [0.; 8];
        let mut acc_re_val: [FLOAT32; 8] = [0.; 8];
        let mut acc_im_val: [FLOAT32; 8] = [0.; 8];
        n = 0 as core::ffi::c_int as WORD32;
        while n < QMF_HYBRID_FILT_ORDER {
            in_re = (*input.offset((n + i) as isize)).re;
            in_im = (*input.offset((n + i) as isize)).im;
            coeff = *filt_coeff
                .offset((QMF_HYBRID_FILT_ORDER - 1 as WORD32 - n) as isize);
            x0_re[n as usize] = coeff * in_re;
            x0_im[n as usize] = coeff * in_im;
            n += 1;
        }
        x0_1_re[0 as core::ffi::c_int as usize] = x0_re[6 as core::ffi::c_int as usize];
        x0_1_im[0 as core::ffi::c_int as usize] = x0_im[6 as core::ffi::c_int as usize];
        x0_1_re[1 as core::ffi::c_int as usize] = x0_re[7 as core::ffi::c_int as usize];
        x0_1_im[1 as core::ffi::c_int as usize] = x0_im[7 as core::ffi::c_int as usize];
        x0_1_re[2 as core::ffi::c_int as usize] = x0_re[8 as core::ffi::c_int as usize]
            - x0_re[0 as core::ffi::c_int as usize];
        x0_1_im[2 as core::ffi::c_int as usize] = x0_im[8 as core::ffi::c_int as usize]
            - x0_im[0 as core::ffi::c_int as usize];
        x0_1_re[3 as core::ffi::c_int as usize] = x0_re[9 as core::ffi::c_int as usize]
            - x0_re[1 as core::ffi::c_int as usize];
        x0_1_im[3 as core::ffi::c_int as usize] = x0_im[9 as core::ffi::c_int as usize]
            - x0_im[1 as core::ffi::c_int as usize];
        x0_1_re[4 as core::ffi::c_int as usize] = x0_re[10 as core::ffi::c_int as usize]
            - x0_re[2 as core::ffi::c_int as usize];
        x0_1_im[4 as core::ffi::c_int as usize] = x0_im[10 as core::ffi::c_int as usize]
            - x0_im[2 as core::ffi::c_int as usize];
        x0_1_re[5 as core::ffi::c_int as usize] = x0_re[11 as core::ffi::c_int as usize]
            - x0_re[3 as core::ffi::c_int as usize];
        x0_1_im[5 as core::ffi::c_int as usize] = x0_im[11 as core::ffi::c_int as usize]
            - x0_im[3 as core::ffi::c_int as usize];
        x0_1_re[6 as core::ffi::c_int as usize] = x0_re[12 as core::ffi::c_int as usize]
            - x0_re[4 as core::ffi::c_int as usize];
        x0_1_im[6 as core::ffi::c_int as usize] = x0_im[12 as core::ffi::c_int as usize]
            - x0_im[4 as core::ffi::c_int as usize];
        x0_1_re[7 as core::ffi::c_int as usize] = -x0_re[5 as core::ffi::c_int as usize];
        x0_1_im[7 as core::ffi::c_int as usize] = -x0_im[5 as core::ffi::c_int as usize];
        acc_re_val[0 as core::ffi::c_int as usize] = x0_1_re[0 as core::ffi::c_int
            as usize];
        acc_re_val[1 as core::ffi::c_int as usize] = x0_1_re[1 as core::ffi::c_int
            as usize] - x0_1_re[7 as core::ffi::c_int as usize];
        acc_re_val[2 as core::ffi::c_int as usize] = x0_1_re[2 as core::ffi::c_int
            as usize] - x0_1_re[6 as core::ffi::c_int as usize];
        acc_re_val[3 as core::ffi::c_int as usize] = x0_1_re[3 as core::ffi::c_int
            as usize] - x0_1_re[5 as core::ffi::c_int as usize];
        acc_re_val[4 as core::ffi::c_int as usize] = x0_1_im[1 as core::ffi::c_int
            as usize] + x0_1_im[7 as core::ffi::c_int as usize];
        acc_re_val[5 as core::ffi::c_int as usize] = x0_1_im[2 as core::ffi::c_int
            as usize] + x0_1_im[6 as core::ffi::c_int as usize];
        acc_re_val[6 as core::ffi::c_int as usize] = x0_1_im[3 as core::ffi::c_int
            as usize] + x0_1_im[5 as core::ffi::c_int as usize];
        acc_re_val[7 as core::ffi::c_int as usize] = x0_1_im[4 as core::ffi::c_int
            as usize];
        acc_im_val[0 as core::ffi::c_int as usize] = x0_1_im[0 as core::ffi::c_int
            as usize];
        acc_im_val[1 as core::ffi::c_int as usize] = x0_1_im[1 as core::ffi::c_int
            as usize] - x0_1_im[7 as core::ffi::c_int as usize];
        acc_im_val[2 as core::ffi::c_int as usize] = x0_1_im[2 as core::ffi::c_int
            as usize] - x0_1_im[6 as core::ffi::c_int as usize];
        acc_im_val[3 as core::ffi::c_int as usize] = x0_1_im[3 as core::ffi::c_int
            as usize] - x0_1_im[5 as core::ffi::c_int as usize];
        acc_im_val[4 as core::ffi::c_int as usize] = x0_1_re[1 as core::ffi::c_int
            as usize] + x0_1_re[7 as core::ffi::c_int as usize];
        acc_im_val[5 as core::ffi::c_int as usize] = x0_1_re[2 as core::ffi::c_int
            as usize] + x0_1_re[6 as core::ffi::c_int as usize];
        acc_im_val[6 as core::ffi::c_int as usize] = x0_1_re[3 as core::ffi::c_int
            as usize] + x0_1_re[5 as core::ffi::c_int as usize];
        acc_im_val[7 as core::ffi::c_int as usize] = x0_1_re[4 as core::ffi::c_int
            as usize];
        q = 0 as core::ffi::c_int as WORD32;
        while q < 4 as core::ffi::c_int {
            acc_re_l = 0 as core::ffi::c_int as FLOAT32;
            acc_im_l = 0 as core::ffi::c_int as FLOAT32;
            acc_re_h = 0 as core::ffi::c_int as FLOAT32;
            acc_im_h = 0 as core::ffi::c_int as FLOAT32;
            acc_re_l += acc_re_val[0 as core::ffi::c_int as usize];
            acc_re_l
                += acc_re_val[1 as core::ffi::c_int as usize]
                    * ixheaacd_cosine[q as usize][1 as core::ffi::c_int as usize];
            acc_re_l
                += acc_re_val[2 as core::ffi::c_int as usize]
                    * ixheaacd_cosine[q as usize][2 as core::ffi::c_int as usize];
            acc_re_l
                += acc_re_val[3 as core::ffi::c_int as usize]
                    * ixheaacd_cosine[q as usize][3 as core::ffi::c_int as usize];
            acc_re_h = acc_re_l;
            acc_re_l
                -= acc_re_val[4 as core::ffi::c_int as usize]
                    * ixheaacd_sine[q as usize][1 as core::ffi::c_int as usize];
            acc_re_l
                -= acc_re_val[5 as core::ffi::c_int as usize]
                    * ixheaacd_sine[q as usize][2 as core::ffi::c_int as usize];
            acc_re_l
                -= acc_re_val[6 as core::ffi::c_int as usize]
                    * ixheaacd_sine[q as usize][3 as core::ffi::c_int as usize];
            acc_re_l
                -= acc_re_val[7 as core::ffi::c_int as usize]
                    * ixheaacd_sine[q as usize][4 as core::ffi::c_int as usize];
            acc_re_h = acc_re_h - (acc_re_l - acc_re_h);
            acc_im_l += acc_im_val[0 as core::ffi::c_int as usize];
            acc_im_l
                += acc_im_val[1 as core::ffi::c_int as usize]
                    * ixheaacd_cosine[q as usize][1 as core::ffi::c_int as usize];
            acc_im_l
                += acc_im_val[2 as core::ffi::c_int as usize]
                    * ixheaacd_cosine[q as usize][2 as core::ffi::c_int as usize];
            acc_im_l
                += acc_im_val[3 as core::ffi::c_int as usize]
                    * ixheaacd_cosine[q as usize][3 as core::ffi::c_int as usize];
            acc_im_h = acc_im_l;
            acc_im_l
                += acc_im_val[4 as core::ffi::c_int as usize]
                    * ixheaacd_sine[q as usize][1 as core::ffi::c_int as usize];
            acc_im_l
                += acc_im_val[5 as core::ffi::c_int as usize]
                    * ixheaacd_sine[q as usize][2 as core::ffi::c_int as usize];
            acc_im_l
                += acc_im_val[6 as core::ffi::c_int as usize]
                    * ixheaacd_sine[q as usize][3 as core::ffi::c_int as usize];
            acc_im_l
                += acc_im_val[7 as core::ffi::c_int as usize]
                    * ixheaacd_sine[q as usize][4 as core::ffi::c_int as usize];
            acc_im_h = acc_im_h - (acc_im_l - acc_im_h);
            (*output.offset(q as isize))[i as usize].re = acc_re_l;
            (*output.offset(q as isize))[i as usize].im = acc_im_l;
            (*output.offset((7 as WORD32 - q) as isize))[i as usize].re = acc_re_h;
            (*output.offset((7 as WORD32 - q) as isize))[i as usize].im = acc_im_h;
            q += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_mps_hyb_filt_type2(
    mut input: *mut ia_cmplx_flt_struct,
    mut output: *mut [ia_cmplx_flt_struct; 72],
    mut num_samples: WORD32,
    mut filt_coeff: *const FLOAT32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut in_re: FLOAT32 = 0.;
    let mut in_im: FLOAT32 = 0.;
    let mut coeff: FLOAT32 = 0.;
    let mut acc_re: [FLOAT32; 2] = [0.; 2];
    let mut acc_im: [FLOAT32; 2] = [0.; 2];
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_samples {
        let mut x_0_re: [FLOAT32; 13] = [0.; 13];
        let mut x_0_im: [FLOAT32; 13] = [0.; 13];
        n = 1 as core::ffi::c_int as WORD32;
        while n < 6 as core::ffi::c_int {
            in_re = (*input.offset((n + i) as isize)).re;
            in_im = (*input.offset((n + i) as isize)).im;
            in_re += (*input.offset((12 as WORD32 - n + i) as isize)).re;
            in_im += (*input.offset((12 as WORD32 - n + i) as isize)).im;
            coeff = *filt_coeff
                .offset((QMF_HYBRID_FILT_ORDER - 1 as WORD32 - n) as isize);
            x_0_re[n as usize] = coeff * in_re;
            x_0_im[n as usize] = coeff * in_im;
            n = (n as core::ffi::c_int + 2 as core::ffi::c_int) as WORD32;
        }
        n = 6 as core::ffi::c_int as WORD32;
        in_re = (*input.offset((n + i) as isize)).re;
        in_im = (*input.offset((n + i) as isize)).im;
        coeff = *filt_coeff.offset((QMF_HYBRID_FILT_ORDER - 1 as WORD32 - n) as isize);
        x_0_re[n as usize] = coeff * in_re;
        x_0_im[n as usize] = coeff * in_im;
        x_0_re[1 as core::ffi::c_int as usize] = x_0_re[1 as core::ffi::c_int as usize]
            + x_0_re[3 as core::ffi::c_int as usize]
            + x_0_re[5 as core::ffi::c_int as usize];
        x_0_im[1 as core::ffi::c_int as usize] = x_0_im[1 as core::ffi::c_int as usize]
            + x_0_im[3 as core::ffi::c_int as usize]
            + x_0_im[5 as core::ffi::c_int as usize];
        acc_re[0 as core::ffi::c_int as usize] = x_0_re[6 as core::ffi::c_int as usize]
            + x_0_re[1 as core::ffi::c_int as usize];
        acc_im[0 as core::ffi::c_int as usize] = x_0_im[6 as core::ffi::c_int as usize]
            + x_0_im[1 as core::ffi::c_int as usize];
        acc_re[1 as core::ffi::c_int as usize] = x_0_re[6 as core::ffi::c_int as usize]
            - x_0_re[1 as core::ffi::c_int as usize];
        acc_im[1 as core::ffi::c_int as usize] = x_0_im[6 as core::ffi::c_int as usize]
            - x_0_im[1 as core::ffi::c_int as usize];
        (*output.offset(0 as core::ffi::c_int as isize))[i as usize].re = acc_re[0
            as core::ffi::c_int as usize];
        (*output.offset(0 as core::ffi::c_int as isize))[i as usize].im = acc_im[0
            as core::ffi::c_int as usize];
        (*output.offset(1 as core::ffi::c_int as isize))[i as usize].re = acc_re[1
            as core::ffi::c_int as usize];
        (*output.offset(1 as core::ffi::c_int as isize))[i as usize].im = acc_im[1
            as core::ffi::c_int as usize];
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_qmf_hybrid_analysis_init(
    mut handle: *mut ia_mps_hybrid_filt_struct,
) -> VOID {
    memset(
        ((*handle).lf_buffer).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ((QMF_BANDS_TO_HYBRID * BUFFER_LEN_LF_MPS) as size_t)
            .wrapping_mul(::core::mem::size_of::<ia_cmplx_w32_struct>() as size_t),
    );
    memset(
        ((*handle).hf_buffer).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ((MAX_NUM_QMF_BANDS_MPS * BUFFER_LEN_HF_MPS) as size_t)
            .wrapping_mul(::core::mem::size_of::<ia_cmplx_flt_struct>() as size_t),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_qmf_hybrid_analysis_no_pre_mix(
    mut handle: *mut ia_mps_hybrid_filt_struct,
    mut in_qmf: *mut [ia_cmplx_flt_struct; 72],
    mut num_bands: WORD32,
    mut num_samples: WORD32,
    mut v: *mut [ia_cmplx_flt_struct; 71],
) -> VOID {
    let mut lf_samples_shift: WORD32 = 0;
    let mut hf_samples_shift: WORD32 = 0;
    let mut lf_qmf_bands: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut scratch: [[ia_cmplx_flt_struct; 72]; 8] = [[ia_cmplx_flt_struct {
        re: 0.,
        im: 0.,
    }; 72]; 8];
    lf_samples_shift = BUFFER_LEN_LF_MPS - num_samples;
    hf_samples_shift = BUFFER_LEN_HF_MPS - num_samples;
    lf_qmf_bands = QMF_BANDS_TO_HYBRID as WORD32;
    k = 0 as core::ffi::c_int as WORD32;
    while k < lf_qmf_bands {
        memmove(
            &mut (*(*((*handle).lf_buffer).as_mut_ptr().offset(k as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .re as *mut FLOAT32 as *mut core::ffi::c_void,
            &mut (*(*((*handle).lf_buffer).as_mut_ptr().offset(k as isize))
                .as_mut_ptr()
                .offset(num_samples as isize))
                .re as *mut FLOAT32 as *const core::ffi::c_void,
            ((2 as WORD32 * lf_samples_shift) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        k += 1;
    }
    k = 0 as core::ffi::c_int as WORD32;
    while k < lf_qmf_bands {
        memcpy(
            &mut (*(*((*handle).lf_buffer).as_mut_ptr().offset(k as isize))
                .as_mut_ptr()
                .offset(lf_samples_shift as isize))
                .re as *mut FLOAT32 as *mut core::ffi::c_void,
            &mut (*(*in_qmf.offset(k as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .re as *mut FLOAT32 as *const core::ffi::c_void,
            ((2 as WORD32 * num_samples) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        k += 1;
    }
    k = 0 as core::ffi::c_int as WORD32;
    while k < MAX_NUM_QMF_BANDS_SAC / 2 as WORD32 - lf_qmf_bands {
        memmove(
            &mut (*(*((*handle).hf_buffer).as_mut_ptr().offset(k as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .re as *mut FLOAT32 as *mut core::ffi::c_void,
            &mut (*(*((*handle).hf_buffer).as_mut_ptr().offset(k as isize))
                .as_mut_ptr()
                .offset(num_samples as isize))
                .re as *mut FLOAT32 as *const core::ffi::c_void,
            ((2 as WORD32 * hf_samples_shift) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        k += 1;
    }
    k = 0 as core::ffi::c_int as WORD32;
    while k < num_bands - lf_qmf_bands {
        memcpy(
            &mut (*(*((*handle).hf_buffer).as_mut_ptr().offset(k as isize))
                .as_mut_ptr()
                .offset(hf_samples_shift as isize))
                .re as *mut FLOAT32 as *mut core::ffi::c_void,
            &mut (*(*in_qmf.offset((k + lf_qmf_bands) as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .re as *mut FLOAT32 as *const core::ffi::c_void,
            ((2 as WORD32 * num_samples) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        k += 1;
    }
    ixheaacd_mps_hyb_filt_type1(
        &mut *(*((*handle).lf_buffer)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(
                (lf_samples_shift as core::ffi::c_int + 1 as core::ffi::c_int
                    - QMF_HYBRID_FILT_ORDER) as isize,
            ),
        scratch.as_mut_ptr(),
        num_samples,
        ixheaacd_ia_mps_hyb_filter_coeff_8.as_ptr(),
    );
    k = 0 as core::ffi::c_int as WORD32;
    while k < 2 as core::ffi::c_int {
        n = 0 as core::ffi::c_int as WORD32;
        while n < num_samples {
            (*v.offset(n as isize))[k as usize].re = scratch[(k as core::ffi::c_int
                    + 6 as core::ffi::c_int) as usize][n as usize]
                .re;
            (*v
                .offset(
                    n as isize,
                ))[(k as core::ffi::c_int + 2 as core::ffi::c_int) as usize]
                .re = scratch[k as usize][n as usize].re;
            (*v
                .offset(
                    n as isize,
                ))[(k as core::ffi::c_int + 4 as core::ffi::c_int) as usize]
                .re = scratch[(k as core::ffi::c_int + 2 as core::ffi::c_int)
                    as usize][n as usize]
                .re;
            (*v
                .offset(
                    n as isize,
                ))[(k as core::ffi::c_int + 4 as core::ffi::c_int) as usize]
                .re += scratch[(5 as WORD32 - k) as usize][n as usize].re;
            (*v.offset(n as isize))[k as usize].im = scratch[(k as core::ffi::c_int
                    + 6 as core::ffi::c_int) as usize][n as usize]
                .im;
            (*v
                .offset(
                    n as isize,
                ))[(k as core::ffi::c_int + 2 as core::ffi::c_int) as usize]
                .im = scratch[k as usize][n as usize].im;
            (*v
                .offset(
                    n as isize,
                ))[(k as core::ffi::c_int + 4 as core::ffi::c_int) as usize]
                .im = scratch[(k as core::ffi::c_int + 2 as core::ffi::c_int)
                    as usize][n as usize]
                .im;
            (*v
                .offset(
                    n as isize,
                ))[(k as core::ffi::c_int + 4 as core::ffi::c_int) as usize]
                .im += scratch[(5 as WORD32 - k) as usize][n as usize].im;
            n += 1;
        }
        k += 1;
    }
    ixheaacd_mps_hyb_filt_type2(
        &mut *(*((*handle).lf_buffer)
            .as_mut_ptr()
            .offset(1 as core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(
                (lf_samples_shift as core::ffi::c_int + 1 as core::ffi::c_int
                    - QMF_HYBRID_FILT_ORDER) as isize,
            ),
        scratch.as_mut_ptr(),
        num_samples,
        ixheaacd_mps_hyb_filter_coeff_2.as_ptr(),
    );
    k = 0 as core::ffi::c_int as WORD32;
    while k < 2 as core::ffi::c_int {
        n = 0 as core::ffi::c_int as WORD32;
        while n < num_samples {
            (*v
                .offset(
                    n as isize,
                ))[(k as core::ffi::c_int + 6 as core::ffi::c_int) as usize]
                .re = scratch[(1 as WORD32 - k) as usize][n as usize].re;
            (*v
                .offset(
                    n as isize,
                ))[(k as core::ffi::c_int + 6 as core::ffi::c_int) as usize]
                .im = scratch[(1 as WORD32 - k) as usize][n as usize].im;
            n += 1;
        }
        k += 1;
    }
    ixheaacd_mps_hyb_filt_type2(
        &mut *(*((*handle).lf_buffer)
            .as_mut_ptr()
            .offset(2 as core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(
                (lf_samples_shift as core::ffi::c_int + 1 as core::ffi::c_int
                    - QMF_HYBRID_FILT_ORDER) as isize,
            ),
        scratch.as_mut_ptr(),
        num_samples,
        ixheaacd_mps_hyb_filter_coeff_2.as_ptr(),
    );
    k = 0 as core::ffi::c_int as WORD32;
    while k < 2 as core::ffi::c_int {
        n = 0 as core::ffi::c_int as WORD32;
        while n < num_samples {
            (*v
                .offset(
                    n as isize,
                ))[(k as core::ffi::c_int + 8 as core::ffi::c_int) as usize]
                .re = scratch[k as usize][n as usize].re;
            (*v
                .offset(
                    n as isize,
                ))[(k as core::ffi::c_int + 8 as core::ffi::c_int) as usize]
                .im = scratch[k as usize][n as usize].im;
            n += 1;
        }
        k += 1;
    }
    k = 0 as core::ffi::c_int as WORD32;
    while k < num_bands - lf_qmf_bands {
        n = 0 as core::ffi::c_int as WORD32;
        while n < num_samples {
            (*v
                .offset(
                    n as isize,
                ))[(k as core::ffi::c_int + 10 as core::ffi::c_int) as usize]
                .re = (*handle)
                .hf_buffer[k as usize][(n + hf_samples_shift) as usize]
                .re;
            (*v
                .offset(
                    n as isize,
                ))[(k as core::ffi::c_int + 10 as core::ffi::c_int) as usize]
                .im = (*handle)
                .hf_buffer[k as usize][(n + hf_samples_shift) as usize]
                .im;
            n += 1;
        }
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_qmf_hybrid_analysis(
    mut handle: *mut ia_mps_hybrid_filt_struct,
    mut in_qmf: *mut [ia_cmplx_flt_struct; 72],
    mut num_bands: WORD32,
    mut num_samples: WORD32,
    mut hyb: *mut [ia_cmplx_flt_struct; 72],
) -> VOID {
    let mut lf_samples_shift: WORD32 = 0;
    let mut hf_samples_shift: WORD32 = 0;
    let mut lf_qmf_bands: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut scratch: [[ia_cmplx_flt_struct; 72]; 8] = [[ia_cmplx_flt_struct {
        re: 0.,
        im: 0.,
    }; 72]; 8];
    lf_samples_shift = BUFFER_LEN_LF_MPS - num_samples;
    hf_samples_shift = BUFFER_LEN_HF_MPS - num_samples;
    lf_qmf_bands = QMF_BANDS_TO_HYBRID as WORD32;
    k = 0 as core::ffi::c_int as WORD32;
    while k < lf_qmf_bands {
        memmove(
            &mut (*(*((*handle).lf_buffer).as_mut_ptr().offset(k as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .re as *mut FLOAT32 as *mut core::ffi::c_void,
            &mut (*(*((*handle).lf_buffer).as_mut_ptr().offset(k as isize))
                .as_mut_ptr()
                .offset(num_samples as isize))
                .re as *mut FLOAT32 as *const core::ffi::c_void,
            ((2 as WORD32 * lf_samples_shift) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        k += 1;
    }
    k = 0 as core::ffi::c_int as WORD32;
    while k < lf_qmf_bands {
        memcpy(
            &mut (*(*((*handle).lf_buffer).as_mut_ptr().offset(k as isize))
                .as_mut_ptr()
                .offset(lf_samples_shift as isize))
                .re as *mut FLOAT32 as *mut core::ffi::c_void,
            &mut (*(*in_qmf.offset(k as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .re as *mut FLOAT32 as *const core::ffi::c_void,
            ((2 as WORD32 * num_samples) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        k += 1;
    }
    k = 0 as core::ffi::c_int as WORD32;
    while k < MAX_NUM_QMF_BANDS_SAC / 2 as WORD32 - lf_qmf_bands {
        memmove(
            &mut (*(*((*handle).hf_buffer).as_mut_ptr().offset(k as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .re as *mut FLOAT32 as *mut core::ffi::c_void,
            &mut (*(*((*handle).hf_buffer).as_mut_ptr().offset(k as isize))
                .as_mut_ptr()
                .offset(num_samples as isize))
                .re as *mut FLOAT32 as *const core::ffi::c_void,
            ((2 as WORD32 * hf_samples_shift) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        k += 1;
    }
    k = 0 as core::ffi::c_int as WORD32;
    while k < num_bands - lf_qmf_bands {
        memcpy(
            &mut (*(*((*handle).hf_buffer).as_mut_ptr().offset(k as isize))
                .as_mut_ptr()
                .offset(hf_samples_shift as isize))
                .re as *mut FLOAT32 as *mut core::ffi::c_void,
            &mut (*(*in_qmf.offset((k + lf_qmf_bands) as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .re as *mut FLOAT32 as *const core::ffi::c_void,
            ((2 as WORD32 * num_samples) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        k += 1;
    }
    ixheaacd_mps_hyb_filt_type1(
        &mut *(*((*handle).lf_buffer)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(
                (lf_samples_shift as core::ffi::c_int + 1 as core::ffi::c_int
                    - QMF_HYBRID_FILT_ORDER) as isize,
            ),
        scratch.as_mut_ptr(),
        num_samples,
        ixheaacd_ia_mps_hyb_filter_coeff_8.as_ptr(),
    );
    k = 0 as core::ffi::c_int as WORD32;
    while k < 2 as core::ffi::c_int {
        n = 0 as core::ffi::c_int as WORD32;
        while n < num_samples {
            (*hyb.offset(k as isize))[n as usize].re = scratch[(k as core::ffi::c_int
                    + 6 as core::ffi::c_int) as usize][n as usize]
                .re;
            (*hyb
                .offset(
                    (k as core::ffi::c_int + 2 as core::ffi::c_int) as isize,
                ))[n as usize]
                .re = scratch[k as usize][n as usize].re;
            (*hyb
                .offset(
                    (k as core::ffi::c_int + 4 as core::ffi::c_int) as isize,
                ))[n as usize]
                .re = scratch[(k as core::ffi::c_int + 2 as core::ffi::c_int)
                    as usize][n as usize]
                .re;
            (*hyb
                .offset(
                    (k as core::ffi::c_int + 4 as core::ffi::c_int) as isize,
                ))[n as usize]
                .re += scratch[(5 as WORD32 - k) as usize][n as usize].re;
            (*hyb.offset(k as isize))[n as usize].im = scratch[(k as core::ffi::c_int
                    + 6 as core::ffi::c_int) as usize][n as usize]
                .im;
            (*hyb
                .offset(
                    (k as core::ffi::c_int + 2 as core::ffi::c_int) as isize,
                ))[n as usize]
                .im = scratch[k as usize][n as usize].im;
            (*hyb
                .offset(
                    (k as core::ffi::c_int + 4 as core::ffi::c_int) as isize,
                ))[n as usize]
                .im = scratch[(k as core::ffi::c_int + 2 as core::ffi::c_int)
                    as usize][n as usize]
                .im;
            (*hyb
                .offset(
                    (k as core::ffi::c_int + 4 as core::ffi::c_int) as isize,
                ))[n as usize]
                .im += scratch[(5 as WORD32 - k) as usize][n as usize].im;
            n += 1;
        }
        k += 1;
    }
    ixheaacd_mps_hyb_filt_type2(
        &mut *(*((*handle).lf_buffer)
            .as_mut_ptr()
            .offset(1 as core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(
                (lf_samples_shift as core::ffi::c_int + 1 as core::ffi::c_int
                    - QMF_HYBRID_FILT_ORDER) as isize,
            ),
        scratch.as_mut_ptr(),
        num_samples,
        ixheaacd_mps_hyb_filter_coeff_2.as_ptr(),
    );
    k = 0 as core::ffi::c_int as WORD32;
    while k < 2 as core::ffi::c_int {
        n = 0 as core::ffi::c_int as WORD32;
        while n < num_samples {
            (*hyb
                .offset(
                    (k as core::ffi::c_int + 6 as core::ffi::c_int) as isize,
                ))[n as usize]
                .re = scratch[(1 as WORD32 - k) as usize][n as usize].re;
            (*hyb
                .offset(
                    (k as core::ffi::c_int + 6 as core::ffi::c_int) as isize,
                ))[n as usize]
                .im = scratch[(1 as WORD32 - k) as usize][n as usize].im;
            n += 1;
        }
        k += 1;
    }
    ixheaacd_mps_hyb_filt_type2(
        &mut *(*((*handle).lf_buffer)
            .as_mut_ptr()
            .offset(2 as core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(
                (lf_samples_shift as core::ffi::c_int + 1 as core::ffi::c_int
                    - QMF_HYBRID_FILT_ORDER) as isize,
            ),
        scratch.as_mut_ptr(),
        num_samples,
        ixheaacd_mps_hyb_filter_coeff_2.as_ptr(),
    );
    k = 0 as core::ffi::c_int as WORD32;
    while k < 2 as core::ffi::c_int {
        n = 0 as core::ffi::c_int as WORD32;
        while n < num_samples {
            (*hyb
                .offset(
                    (k as core::ffi::c_int + 8 as core::ffi::c_int) as isize,
                ))[n as usize]
                .re = scratch[k as usize][n as usize].re;
            (*hyb
                .offset(
                    (k as core::ffi::c_int + 8 as core::ffi::c_int) as isize,
                ))[n as usize]
                .im = scratch[k as usize][n as usize].im;
            n += 1;
        }
        k += 1;
    }
    k = 0 as core::ffi::c_int as WORD32;
    while k < num_bands - lf_qmf_bands {
        memcpy(
            &mut (*(*hyb
                .offset((k as core::ffi::c_int + 10 as core::ffi::c_int) as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .re as *mut FLOAT32 as *mut core::ffi::c_void,
            &mut (*(*((*handle).hf_buffer).as_mut_ptr().offset(k as isize))
                .as_mut_ptr()
                .offset(hf_samples_shift as isize))
                .re as *mut FLOAT32 as *const core::ffi::c_void,
            ((2 as WORD32 * num_samples) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_qmf_hybrid_synthesis(
    mut hyb: *mut [ia_cmplx_flt_struct; 71],
    mut num_bands: WORD32,
    mut num_samples: WORD32,
    mut in_qmf: *mut [ia_cmplx_flt_struct; 128],
) -> VOID {
    let mut k: WORD32 = 0;
    let mut n: WORD32 = 0;
    n = 0 as core::ffi::c_int as WORD32;
    while n < num_samples {
        (*in_qmf.offset(n as isize))[0 as core::ffi::c_int as usize].re = (*hyb
            .offset(n as isize))[0 as core::ffi::c_int as usize]
            .re;
        (*in_qmf.offset(n as isize))[0 as core::ffi::c_int as usize].im = (*hyb
            .offset(n as isize))[0 as core::ffi::c_int as usize]
            .im;
        k = 1 as core::ffi::c_int as WORD32;
        while k < 6 as core::ffi::c_int {
            (*in_qmf.offset(n as isize))[0 as core::ffi::c_int as usize].re
                += (*hyb.offset(n as isize))[k as usize].re;
            (*in_qmf.offset(n as isize))[0 as core::ffi::c_int as usize].im
                += (*hyb.offset(n as isize))[k as usize].im;
            k += 1;
        }
        (*in_qmf.offset(n as isize))[1 as core::ffi::c_int as usize].re = (*hyb
            .offset(n as isize))[6 as core::ffi::c_int as usize]
            .re + (*hyb.offset(n as isize))[7 as core::ffi::c_int as usize].re;
        (*in_qmf.offset(n as isize))[1 as core::ffi::c_int as usize].im = (*hyb
            .offset(n as isize))[6 as core::ffi::c_int as usize]
            .im + (*hyb.offset(n as isize))[7 as core::ffi::c_int as usize].im;
        (*in_qmf.offset(n as isize))[2 as core::ffi::c_int as usize].re = (*hyb
            .offset(n as isize))[8 as core::ffi::c_int as usize]
            .re + (*hyb.offset(n as isize))[9 as core::ffi::c_int as usize].re;
        (*in_qmf.offset(n as isize))[2 as core::ffi::c_int as usize].im = (*hyb
            .offset(n as isize))[8 as core::ffi::c_int as usize]
            .im + (*hyb.offset(n as isize))[9 as core::ffi::c_int as usize].im;
        memcpy(
            &mut (*(*in_qmf.offset(n as isize))
                .as_mut_ptr()
                .offset(3 as core::ffi::c_int as isize))
                .re as *mut FLOAT32 as *mut core::ffi::c_void,
            &mut (*(*hyb.offset(n as isize))
                .as_mut_ptr()
                .offset(10 as core::ffi::c_int as isize))
                .re as *mut FLOAT32 as *const core::ffi::c_void,
            ((2 as core::ffi::c_int
                * (num_bands as core::ffi::c_int - 3 as core::ffi::c_int)) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_fft(
    mut out: *mut complex,
    mut idx: LOOPINDEX,
    mut nob: WORD32,
    mut hyb_tab: *const ia_mps_dec_hybrid_tables_struct,
) -> VOID {
    let mut block_per_stage: LOOPINDEX = 0;
    let mut stage_num: LOOPINDEX = 0;
    let mut inner: LOOPINDEX = 0;
    let mut cosine_array: *const WORD32 = ((*hyb_tab).cosine_array).as_ptr();
    let mut sine_array: *const WORD32 = ((*hyb_tab).sine_array).as_ptr();
    let mut index_1: WORD32 = 0;
    let mut index_2: WORD32 = 0;
    let mut index: WORD32 = 0;
    let mut tab_modifier: WORD32 = 0;
    let mut len: WORD32 = 0;
    let mut increment: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut cos_val: WORD32 = 0;
    let mut sin_val: WORD32 = 0;
    let mut index1: WORD16 = 0;
    let mut re_temp: WORD32 = 0;
    let mut im_temp: WORD32 = 0;
    let mut out1_w32: *mut WORD32 = 0 as *mut WORD32;
    let mut out2_w32: *mut WORD32 = 0 as *mut WORD32;
    len = idx as WORD32;
    i = 1 as core::ffi::c_int as WORD32;
    increment = 0 as core::ffi::c_int as WORD32;
    len = len >> 1 as core::ffi::c_int;
    index_1 = 0 as core::ffi::c_int as WORD32;
    increment += 1 as core::ffi::c_int;
    index = 11 as WORD32 - increment;
    tab_modifier = ixheaac_shl32(1 as WORD32, index as WORD);
    out1_w32 = &mut *out.offset(index_1 as isize) as *mut complex as *mut WORD32;
    out2_w32 = &mut *out
        .offset((index_1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
        as *mut complex as *mut WORD32;
    block_per_stage = 0 as core::ffi::c_int as LOOPINDEX;
    while block_per_stage < len {
        re_temp = *out2_w32.offset(0 as core::ffi::c_int as isize);
        im_temp = *out2_w32.offset(1 as core::ffi::c_int as isize);
        *out2_w32.offset(0 as core::ffi::c_int as isize) = ixheaac_sub32_sat(
            *out1_w32.offset(0 as core::ffi::c_int as isize),
            re_temp,
        );
        *out2_w32.offset(1 as core::ffi::c_int as isize) = ixheaac_sub32_sat(
            *out1_w32.offset(1 as core::ffi::c_int as isize),
            im_temp,
        );
        *out1_w32.offset(0 as core::ffi::c_int as isize) = ixheaac_add32_sat(
            re_temp,
            *out1_w32.offset(0 as core::ffi::c_int as isize),
        );
        *out1_w32.offset(1 as core::ffi::c_int as isize) = ixheaac_add32_sat(
            im_temp,
            *out1_w32.offset(1 as core::ffi::c_int as isize),
        );
        out1_w32 = out1_w32.offset(4 as core::ffi::c_int as isize);
        out2_w32 = out2_w32.offset(4 as core::ffi::c_int as isize);
        block_per_stage += 1;
    }
    i <<= 1 as core::ffi::c_int;
    stage_num = 1 as core::ffi::c_int as LOOPINDEX;
    while stage_num < nob {
        len = len >> 1 as core::ffi::c_int;
        index_1 = 0 as core::ffi::c_int as WORD32;
        increment += 1 as core::ffi::c_int;
        index = 11 as WORD32 - increment;
        tab_modifier = ixheaac_shl32(1 as WORD32, index as WORD);
        block_per_stage = 0 as core::ffi::c_int as LOOPINDEX;
        while block_per_stage < len {
            index_2 = index_1 + i;
            out1_w32 = &mut *out.offset(index_1 as isize) as *mut complex as *mut WORD32;
            out2_w32 = &mut *out.offset(index_2 as isize) as *mut complex as *mut WORD32;
            re_temp = *out1_w32.offset(0 as core::ffi::c_int as isize);
            im_temp = *out1_w32.offset(1 as core::ffi::c_int as isize);
            *out1_w32.offset(0 as core::ffi::c_int as isize) = (re_temp as WORD64
                + *out2_w32.offset(0 as core::ffi::c_int as isize) as WORD64
                >> 1 as core::ffi::c_int) as WORD32;
            *out1_w32.offset(1 as core::ffi::c_int as isize) = (im_temp as WORD64
                + *out2_w32.offset(1 as core::ffi::c_int as isize) as WORD64
                >> 1 as core::ffi::c_int) as WORD32;
            *out2_w32.offset(0 as core::ffi::c_int as isize) = (re_temp as WORD64
                - *out2_w32.offset(0 as core::ffi::c_int as isize) as WORD64
                >> 1 as core::ffi::c_int) as WORD32;
            *out2_w32.offset(1 as core::ffi::c_int as isize) = (im_temp as WORD64
                - *out2_w32.offset(1 as core::ffi::c_int as isize) as WORD64
                >> 1 as core::ffi::c_int) as WORD32;
            index1 = tab_modifier as WORD16;
            out1_w32 = out1_w32.offset(2 as core::ffi::c_int as isize);
            out2_w32 = out2_w32.offset(2 as core::ffi::c_int as isize);
            inner = 0 as core::ffi::c_int as LOOPINDEX;
            while inner
                < (i as core::ffi::c_int - 1 as core::ffi::c_int)
                    << 1 as core::ffi::c_int
            {
                cos_val = *cosine_array.offset(index1 as isize);
                sin_val = *sine_array.offset(index1 as isize);
                re_temp = ixheaacd_mps_mult32x16_shr_16(
                    *out2_w32.offset(inner as isize),
                    cos_val,
                )
                    + ixheaacd_mps_mult32x16_shr_16(
                        *out2_w32
                            .offset(
                                (inner as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                            ),
                        sin_val,
                    );
                im_temp = ixheaacd_mps_mult32x16_shr_16(
                    *out2_w32
                        .offset(
                            (inner as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                        ),
                    cos_val,
                )
                    - ixheaacd_mps_mult32x16_shr_16(
                        *out2_w32.offset(inner as isize),
                        sin_val,
                    );
                *out1_w32.offset(inner as isize) >>= 1 as core::ffi::c_int;
                *out1_w32
                    .offset((inner as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                    >>= 1 as core::ffi::c_int;
                *out2_w32.offset(inner as isize) = ixheaac_sub32_sat(
                    *out1_w32.offset(inner as isize),
                    re_temp,
                );
                *out2_w32
                    .offset(
                        (inner as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                    ) = ixheaac_sub32_sat(
                    *out1_w32
                        .offset(
                            (inner as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                        ),
                    im_temp,
                );
                *out1_w32.offset(inner as isize) = ixheaac_add32_sat(
                    *out1_w32.offset(inner as isize),
                    re_temp,
                );
                *out1_w32
                    .offset(
                        (inner as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                    ) = ixheaac_add32_sat(
                    *out1_w32
                        .offset(
                            (inner as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                        ),
                    im_temp,
                );
                index1 = (index1 as core::ffi::c_int + tab_modifier as core::ffi::c_int)
                    as WORD16;
                inner += 2 as core::ffi::c_int;
            }
            index_1 += ixheaac_shl32(1 as WORD32, increment as WORD);
            block_per_stage += 1;
        }
        i <<= 1 as core::ffi::c_int;
        stage_num += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_8ch_filtering(
    mut p_qmf_real: *const WORD32,
    mut p_qmf_imag: *const WORD32,
    mut m_hybrid_real: *mut WORD32,
    mut m_hybrid_imag: *mut WORD32,
    mut hyb_tab: *const ia_mps_dec_hybrid_tables_struct,
) -> VOID {
    let mut n: WORD32 = 0;
    let mut real: WORD32 = 0;
    let mut imag: WORD32 = 0;
    let tcos: WORD16 = COS_PI_BY_8 as WORD16;
    let tsin: WORD16 = SIN_PI_BY_8 as WORD16;
    let mut cum: [WORD32; 16] = [0; 16];
    let mut p_complex: *mut WORD32 = 0 as *mut WORD32;
    let mut p8_13: *const WORD16 = ((*hyb_tab).p8_13).as_ptr();
    real = ixheaac_shl32(
        ixheaac_mult32x16in32(
            *p_qmf_real.offset(4 as core::ffi::c_int as isize),
            *p8_13.offset(4 as core::ffi::c_int as isize),
        )
            + ixheaac_mult32x16in32(
                *p_qmf_real.offset(12 as core::ffi::c_int as isize),
                *p8_13.offset(12 as core::ffi::c_int as isize),
            ),
        1 as WORD,
    );
    imag = ixheaac_shl32(
        ixheaac_mult32x16in32(
            *p_qmf_imag.offset(4 as core::ffi::c_int as isize),
            *p8_13.offset(4 as core::ffi::c_int as isize),
        )
            + ixheaac_mult32x16in32(
                *p_qmf_imag.offset(12 as core::ffi::c_int as isize),
                *p8_13.offset(12 as core::ffi::c_int as isize),
            ),
        1 as WORD,
    );
    cum[5 as core::ffi::c_int as usize] = ixheaac_sub32_sat(imag, real);
    cum[4 as core::ffi::c_int as usize] = -ixheaac_add32_sat(imag, real);
    real = ixheaac_shl32(
        ixheaac_mult32x16in32(
            *p_qmf_real.offset(3 as core::ffi::c_int as isize),
            *p8_13.offset(3 as core::ffi::c_int as isize),
        )
            + ixheaac_mult32x16in32(
                *p_qmf_real.offset(11 as core::ffi::c_int as isize),
                *p8_13.offset(11 as core::ffi::c_int as isize),
            ),
        1 as WORD,
    );
    imag = ixheaac_shl32(
        ixheaac_mult32x16in32(
            *p_qmf_imag.offset(3 as core::ffi::c_int as isize),
            *p8_13.offset(3 as core::ffi::c_int as isize),
        )
            + ixheaac_mult32x16in32(
                *p_qmf_imag.offset(11 as core::ffi::c_int as isize),
                *p8_13.offset(11 as core::ffi::c_int as isize),
            ),
        1 as WORD,
    );
    cum[13 as core::ffi::c_int as usize] = ixheaac_shl32(
        ixheaac_mult32x16in32(imag, tcos) - ixheaac_mult32x16in32(real, tsin),
        1 as WORD,
    );
    cum[12 as core::ffi::c_int as usize] = ixheaac_shl32(
        -(ixheaac_mult32x16in32(imag, tsin) + ixheaac_mult32x16in32(real, tcos)),
        1 as WORD,
    );
    cum[2 as core::ffi::c_int as usize] = ixheaac_shl32(
        ixheaac_mult32x16in32(
            *p_qmf_real.offset(2 as core::ffi::c_int as isize),
            *p8_13.offset(10 as core::ffi::c_int as isize),
        )
            - ixheaac_mult32x16in32(
                *p_qmf_real.offset(10 as core::ffi::c_int as isize),
                *p8_13.offset(10 as core::ffi::c_int as isize),
            ),
        1 as WORD,
    );
    cum[3 as core::ffi::c_int as usize] = ixheaac_shl32(
        ixheaac_mult32x16in32(
            *p_qmf_imag.offset(2 as core::ffi::c_int as isize),
            *p8_13.offset(2 as core::ffi::c_int as isize),
        )
            - ixheaac_mult32x16in32(
                *p_qmf_imag.offset(10 as core::ffi::c_int as isize),
                *p8_13.offset(2 as core::ffi::c_int as isize),
            ),
        1 as WORD,
    );
    real = ixheaac_shl32(
        ixheaac_mult32x16in32(
            *p_qmf_real.offset(1 as core::ffi::c_int as isize),
            *p8_13.offset(1 as core::ffi::c_int as isize),
        )
            + ixheaac_mult32x16in32(
                *p_qmf_real.offset(9 as core::ffi::c_int as isize),
                *p8_13.offset(9 as core::ffi::c_int as isize),
            ),
        1 as WORD,
    );
    imag = ixheaac_shl32(
        ixheaac_mult32x16in32(
            *p_qmf_imag.offset(1 as core::ffi::c_int as isize),
            *p8_13.offset(1 as core::ffi::c_int as isize),
        )
            + ixheaac_mult32x16in32(
                *p_qmf_imag.offset(9 as core::ffi::c_int as isize),
                *p8_13.offset(9 as core::ffi::c_int as isize),
            ),
        1 as WORD,
    );
    cum[11 as core::ffi::c_int as usize] = ixheaac_shl32(
        ixheaac_mult32x16in32(imag, tcos) + ixheaac_mult32x16in32(real, tsin),
        1 as WORD,
    );
    cum[10 as core::ffi::c_int as usize] = ixheaac_shl32(
        ixheaac_mult32x16in32(imag, tsin) - ixheaac_mult32x16in32(real, tcos),
        1 as WORD,
    );
    real = ixheaac_shl32(
        ixheaac_mult32x16in32(
            *p_qmf_real.offset(0 as core::ffi::c_int as isize),
            *p8_13.offset(0 as core::ffi::c_int as isize),
        )
            + ixheaac_mult32x16in32(
                *p_qmf_real.offset(8 as core::ffi::c_int as isize),
                *p8_13.offset(8 as core::ffi::c_int as isize),
            ),
        1 as WORD,
    );
    imag = ixheaac_shl32(
        ixheaac_mult32x16in32(
            *p_qmf_imag.offset(0 as core::ffi::c_int as isize),
            *p8_13.offset(0 as core::ffi::c_int as isize),
        )
            + ixheaac_mult32x16in32(
                *p_qmf_imag.offset(8 as core::ffi::c_int as isize),
                *p8_13.offset(8 as core::ffi::c_int as isize),
            ),
        1 as WORD,
    );
    cum[7 as core::ffi::c_int as usize] = ixheaac_add32_sat(imag, real);
    cum[6 as core::ffi::c_int as usize] = ixheaac_sub32_sat(imag, real);
    cum[15 as core::ffi::c_int as usize] = ixheaac_shl32(
        ixheaac_mult32x16in32(
            *p_qmf_imag.offset(7 as core::ffi::c_int as isize),
            *p8_13.offset(14 as core::ffi::c_int as isize),
        )
            + ixheaac_mult32x16in32(
                *p_qmf_real.offset(7 as core::ffi::c_int as isize),
                *p8_13.offset(13 as core::ffi::c_int as isize),
            ),
        1 as WORD,
    );
    cum[14 as core::ffi::c_int as usize] = ixheaac_shl32(
        ixheaac_mult32x16in32(
            *p_qmf_imag.offset(7 as core::ffi::c_int as isize),
            *p8_13.offset(13 as core::ffi::c_int as isize),
        )
            - ixheaac_mult32x16in32(
                *p_qmf_real.offset(7 as core::ffi::c_int as isize),
                *p8_13.offset(14 as core::ffi::c_int as isize),
            ),
        1 as WORD,
    );
    cum[1 as core::ffi::c_int as usize] = ixheaac_shl32(
        ixheaac_mult32x16in32(
            *p_qmf_real.offset(HYBRID_FILTER_DELAY as isize),
            *p8_13.offset(HYBRID_FILTER_DELAY as isize),
        ),
        1 as WORD,
    );
    cum[0 as core::ffi::c_int as usize] = ixheaac_shl32(
        ixheaac_mult32x16in32(
            *p_qmf_imag.offset(HYBRID_FILTER_DELAY as isize),
            *p8_13.offset(HYBRID_FILTER_DELAY as isize),
        ),
        1 as WORD,
    );
    cum[9 as core::ffi::c_int as usize] = ixheaac_shl32(
        ixheaac_mult32x16in32(
            *p_qmf_real.offset(5 as core::ffi::c_int as isize),
            *p8_13.offset(13 as core::ffi::c_int as isize),
        )
            - ixheaac_mult32x16in32(
                *p_qmf_imag.offset(5 as core::ffi::c_int as isize),
                *p8_13.offset(14 as core::ffi::c_int as isize),
            ),
        1 as WORD,
    );
    cum[8 as core::ffi::c_int as usize] = ixheaac_shl32(
        ixheaac_mult32x16in32(
            *p_qmf_real.offset(5 as core::ffi::c_int as isize),
            *p8_13.offset(14 as core::ffi::c_int as isize),
        )
            + ixheaac_mult32x16in32(
                *p_qmf_imag.offset(5 as core::ffi::c_int as isize),
                *p8_13.offset(13 as core::ffi::c_int as isize),
            ),
        1 as WORD,
    );
    ixheaacd_mps_fft(
        cum.as_mut_ptr() as *mut complex,
        8 as LOOPINDEX,
        3 as WORD32,
        hyb_tab,
    );
    p_complex = cum.as_mut_ptr();
    n = 0 as core::ffi::c_int as WORD32;
    while n < 8 as core::ffi::c_int {
        let fresh0 = p_complex;
        p_complex = p_complex.offset(1);
        *m_hybrid_imag.offset(n as isize) = *fresh0;
        let fresh1 = p_complex;
        p_complex = p_complex.offset(1);
        *m_hybrid_real.offset(n as isize) = *fresh1;
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_2ch_filtering(
    mut p_qmf: *mut WORD32,
    mut m_hybrid: *mut WORD32,
    mut hyb_tab_ptr: *const ia_mps_dec_hybrid_tables_struct,
) -> VOID {
    let mut cum0: WORD32 = 0;
    let mut cum1: WORD32 = 0;
    let mut temp: WORD64 = 0;
    let mut p2_6: *const WORD16 = ((*hyb_tab_ptr).p2_6).as_ptr();
    cum0 = *p_qmf.offset(HYBRID_FILTER_DELAY as isize) >> 1 as core::ffi::c_int;
    temp = *p2_6.offset(0 as core::ffi::c_int as isize) as WORD64
        * (*p_qmf.offset(1 as core::ffi::c_int as isize) as WORD64
            + *p_qmf.offset(11 as core::ffi::c_int as isize) as WORD64)
        + *p2_6.offset(1 as core::ffi::c_int as isize) as WORD64
            * (*p_qmf.offset(3 as core::ffi::c_int as isize) as WORD64
                + *p_qmf.offset(9 as core::ffi::c_int as isize) as WORD64);
    temp
        += *p2_6.offset(2 as core::ffi::c_int as isize) as WORD64
            * (*p_qmf.offset(5 as core::ffi::c_int as isize) as WORD64
                + *p_qmf.offset(7 as core::ffi::c_int as isize) as WORD64);
    cum1 = (temp >> 16 as core::ffi::c_int) as WORD32;
    *m_hybrid.offset(0 as core::ffi::c_int as isize) = ixheaac_add32_sat(cum0, cum1);
    *m_hybrid.offset(1 as core::ffi::c_int as isize) = ixheaac_sub32_sat(cum0, cum1);
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_get_qmf_sb(
    mut hybrid_subband: WORD32,
    mut ixheaacd_mps_dec_mdct2qmf_table: *const ia_mps_dec_mdct2qmf_table_struct,
) -> WORD32 {
    return (*ixheaacd_mps_dec_mdct2qmf_table).hybrid_2_qmf[hybrid_subband as usize];
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_init_ana_hyb_filt_bank(
    mut hyb_state: *mut ia_mps_dec_thyb_filter_state_struct,
) -> VOID {
    let mut k: WORD32 = 0;
    let mut n: WORD32 = 0;
    k = 0 as core::ffi::c_int as WORD32;
    while k < QMF_BANDS_TO_HYBRID {
        n = 0 as core::ffi::c_int as WORD32;
        while n < PROTO_LEN - 1 as core::ffi::c_int + MAX_TIME_SLOTS {
            (*hyb_state).buffer_lf_real[k as usize][n as usize] = 0 as core::ffi::c_int
                as WORD32;
            (*hyb_state).buffer_lf_imag[k as usize][n as usize] = 0 as core::ffi::c_int
                as WORD32;
            (*hyb_state).qmf_lf_real[k as usize][n as usize] = 0 as core::ffi::c_int
                as WORD32;
            (*hyb_state).qmf_lf_imag[k as usize][n as usize] = 0 as core::ffi::c_int
                as WORD32;
            n += 1;
        }
        k += 1;
    }
    k = 0 as core::ffi::c_int as WORD32;
    while k < MAX_NUM_QMF_BANDS {
        n = 0 as core::ffi::c_int as WORD32;
        while n
            < (PROTO_LEN - 1 as core::ffi::c_int >> 1 as core::ffi::c_int)
                + MAX_TIME_SLOTS
        {
            (*hyb_state).buffer_hf_real[k as usize][n as usize] = 0 as core::ffi::c_int
                as WORD32;
            (*hyb_state).buffer_hf_imag[k as usize][n as usize] = 0 as core::ffi::c_int
                as WORD32;
            n += 1;
        }
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_apply_ana_hyb_filt_bank_create_x(
    mut hyb_state: *mut ia_mps_dec_thyb_filter_state_struct,
    mut m_qmf_real: *mut WORD32,
    mut m_qmf_imag: *mut WORD32,
    mut nr_bands: WORD32,
    mut nr_samples: WORD32,
    mut m_hybrid_real: *mut WORD32,
    mut m_hybrid_imag: *mut WORD32,
    mut hyb_tab_ptr: *const ia_mps_dec_hybrid_tables_struct,
) -> VOID {
    let mut nr_samples_shift_lf: WORD32 = 0;
    let mut nr_qmf_bands_lf: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut time_slot: WORD32 = 0;
    let mut proto_len: WORD32 = PROTO_LEN - 1 as WORD32 >> 1 as core::ffi::c_int;
    let mut val: WORD32 = nr_samples - proto_len;
    let mut val_xhb: WORD32 = val * MAX_HYBRID_BANDS;
    let mut loop_cnt: WORD32 = 0;
    let mut loop_cnt_x4: WORD32 = 0;
    let mut p_qmf_real: *mut WORD32 = 0 as *mut WORD32;
    let mut p_qmf_re: *mut WORD32 = 0 as *mut WORD32;
    let mut p_qmf_imag: *mut WORD32 = 0 as *mut WORD32;
    let mut p_qmf_im: *mut WORD32 = 0 as *mut WORD32;
    let mut m_temp_output_real: [WORD32; 8] = [0; 8];
    let mut m_temp_output_imag: [WORD32; 8] = [0; 8];
    let mut p_hybrid_real: *mut WORD32 = m_hybrid_real
        .offset(10 as core::ffi::c_int as isize);
    let mut p_hybrid_imag: *mut WORD32 = m_hybrid_imag
        .offset(10 as core::ffi::c_int as isize);
    let mut p_hybrid_re: *mut WORD32 = 0 as *mut WORD32;
    let mut p_hybrid_im: *mut WORD32 = 0 as *mut WORD32;
    nr_samples_shift_lf = BUFFER_LEN_LF - nr_samples;
    nr_qmf_bands_lf = QMF_BANDS_TO_HYBRID as WORD32;
    loop_cnt = nr_bands - nr_qmf_bands_lf;
    loop_cnt_x4 = loop_cnt << 2 as core::ffi::c_int;
    k = 0 as core::ffi::c_int as WORD32;
    while k < nr_qmf_bands_lf {
        n = 0 as core::ffi::c_int as WORD32;
        while n < nr_samples_shift_lf {
            (*hyb_state).buffer_lf_real[k as usize][n as usize] = (*hyb_state)
                .buffer_lf_real[k as usize][(n + nr_samples) as usize];
            (*hyb_state).buffer_lf_imag[k as usize][n as usize] = (*hyb_state)
                .buffer_lf_imag[k as usize][(n + nr_samples) as usize];
            (*hyb_state).qmf_lf_real[k as usize][n as usize] = (*hyb_state)
                .qmf_lf_real[k as usize][(n + nr_samples) as usize];
            (*hyb_state).qmf_lf_imag[k as usize][n as usize] = (*hyb_state)
                .qmf_lf_imag[k as usize][(n + nr_samples) as usize];
            n += 1;
        }
        k += 1;
    }
    p_qmf_real = m_qmf_real;
    p_qmf_imag = m_qmf_imag;
    k = 0 as core::ffi::c_int as WORD32;
    while k < nr_qmf_bands_lf {
        p_qmf_re = p_qmf_real;
        p_qmf_im = p_qmf_imag;
        n = 0 as core::ffi::c_int as WORD32;
        while n < nr_samples {
            (*hyb_state)
                .buffer_lf_real[k as usize][(n + nr_samples_shift_lf) as usize] = *p_qmf_re;
            (*hyb_state)
                .buffer_lf_imag[k as usize][(n + nr_samples_shift_lf) as usize] = *p_qmf_im;
            (*hyb_state).qmf_lf_imag[k as usize][(n + nr_samples_shift_lf) as usize] = *p_qmf_im;
            (*hyb_state).qmf_lf_real[k as usize][(n + nr_samples_shift_lf) as usize] = *p_qmf_re;
            p_qmf_re = p_qmf_re.offset(MAX_HYBRID_BANDS as isize);
            p_qmf_im = p_qmf_im.offset(MAX_HYBRID_BANDS as isize);
            n += 1;
        }
        p_qmf_real = p_qmf_real.offset(1);
        p_qmf_imag = p_qmf_imag.offset(1);
        k += 1;
    }
    p_qmf_real = m_qmf_real.offset(nr_qmf_bands_lf as isize).offset(val_xhb as isize);
    p_qmf_imag = m_qmf_imag.offset(nr_qmf_bands_lf as isize).offset(val_xhb as isize);
    n = 0 as core::ffi::c_int as WORD32;
    while n < proto_len {
        p_qmf_re = p_qmf_real;
        p_qmf_im = p_qmf_imag;
        p_hybrid_re = p_hybrid_real;
        p_hybrid_im = p_hybrid_imag;
        k = 0 as core::ffi::c_int as WORD32;
        while k < loop_cnt {
            let fresh2 = p_hybrid_re;
            p_hybrid_re = p_hybrid_re.offset(1);
            *fresh2 = (*hyb_state).buffer_hf_real[k as usize][n as usize];
            let fresh3 = p_hybrid_im;
            p_hybrid_im = p_hybrid_im.offset(1);
            *fresh3 = (*hyb_state).buffer_hf_imag[k as usize][n as usize];
            let fresh4 = p_qmf_re;
            p_qmf_re = p_qmf_re.offset(1);
            (*hyb_state).buffer_hf_real[k as usize][n as usize] = *fresh4;
            let fresh5 = p_qmf_im;
            p_qmf_im = p_qmf_im.offset(1);
            (*hyb_state).buffer_hf_imag[k as usize][n as usize] = *fresh5;
            k += 1;
        }
        p_qmf_real = p_qmf_real.offset(MAX_HYBRID_BANDS as isize);
        p_qmf_imag = p_qmf_imag.offset(MAX_HYBRID_BANDS as isize);
        p_hybrid_real = p_hybrid_real.offset(MAX_HYBRID_BANDS as isize);
        p_hybrid_imag = p_hybrid_imag.offset(MAX_HYBRID_BANDS as isize);
        n += 1;
    }
    p_qmf_real = m_qmf_real;
    p_qmf_imag = m_qmf_imag;
    p_hybrid_real = m_hybrid_real.offset(10 as core::ffi::c_int as isize);
    p_hybrid_imag = m_hybrid_imag.offset(10 as core::ffi::c_int as isize);
    k = (proto_len as core::ffi::c_int * MAX_HYBRID_BANDS) as WORD32;
    p_hybrid_re = p_hybrid_real.offset(k as isize);
    p_hybrid_im = p_hybrid_imag.offset(k as isize);
    p_qmf_re = p_qmf_real.offset(nr_qmf_bands_lf as isize);
    p_qmf_im = p_qmf_imag.offset(nr_qmf_bands_lf as isize);
    n = 0 as core::ffi::c_int as WORD32;
    while n < val {
        memcpy(
            p_hybrid_re as *mut core::ffi::c_void,
            p_qmf_re as *const core::ffi::c_void,
            loop_cnt_x4 as size_t,
        );
        memcpy(
            p_hybrid_im as *mut core::ffi::c_void,
            p_qmf_im as *const core::ffi::c_void,
            loop_cnt_x4 as size_t,
        );
        p_qmf_re = p_qmf_re.offset(MAX_HYBRID_BANDS as isize);
        p_qmf_im = p_qmf_im.offset(MAX_HYBRID_BANDS as isize);
        p_hybrid_re = p_hybrid_re.offset(MAX_HYBRID_BANDS as isize);
        p_hybrid_im = p_hybrid_im.offset(MAX_HYBRID_BANDS as isize);
        n += 1;
    }
    p_hybrid_real = m_hybrid_real;
    p_hybrid_imag = m_hybrid_imag;
    time_slot = 0 as core::ffi::c_int as WORD32;
    while time_slot < nr_samples {
        p_hybrid_re = p_hybrid_real;
        p_hybrid_im = p_hybrid_imag;
        ixheaacd_8ch_filtering(
            &mut *(*((*hyb_state).buffer_lf_real)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(
                    (time_slot as core::ffi::c_int
                        + nr_samples_shift_lf as core::ffi::c_int + 1 as core::ffi::c_int
                        - PROTO_LEN) as isize,
                ),
            &mut *(*((*hyb_state).buffer_lf_imag)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(
                    (time_slot as core::ffi::c_int
                        + nr_samples_shift_lf as core::ffi::c_int + 1 as core::ffi::c_int
                        - PROTO_LEN) as isize,
                ),
            m_temp_output_real.as_mut_ptr(),
            m_temp_output_imag.as_mut_ptr(),
            hyb_tab_ptr,
        );
        let fresh6 = p_hybrid_re;
        p_hybrid_re = p_hybrid_re.offset(1);
        *fresh6 = m_temp_output_real[6 as core::ffi::c_int as usize];
        let fresh7 = p_hybrid_re;
        p_hybrid_re = p_hybrid_re.offset(1);
        *fresh7 = m_temp_output_real[7 as core::ffi::c_int as usize];
        let fresh8 = p_hybrid_re;
        p_hybrid_re = p_hybrid_re.offset(1);
        *fresh8 = m_temp_output_real[0 as core::ffi::c_int as usize];
        let fresh9 = p_hybrid_re;
        p_hybrid_re = p_hybrid_re.offset(1);
        *fresh9 = m_temp_output_real[1 as core::ffi::c_int as usize];
        let fresh10 = p_hybrid_re;
        p_hybrid_re = p_hybrid_re.offset(1);
        *fresh10 = m_temp_output_real[2 as core::ffi::c_int as usize]
            + m_temp_output_real[5 as core::ffi::c_int as usize];
        let fresh11 = p_hybrid_re;
        p_hybrid_re = p_hybrid_re.offset(1);
        *fresh11 = m_temp_output_real[3 as core::ffi::c_int as usize]
            + m_temp_output_real[4 as core::ffi::c_int as usize];
        let fresh12 = p_hybrid_im;
        p_hybrid_im = p_hybrid_im.offset(1);
        *fresh12 = m_temp_output_imag[6 as core::ffi::c_int as usize];
        let fresh13 = p_hybrid_im;
        p_hybrid_im = p_hybrid_im.offset(1);
        *fresh13 = m_temp_output_imag[7 as core::ffi::c_int as usize];
        let fresh14 = p_hybrid_im;
        p_hybrid_im = p_hybrid_im.offset(1);
        *fresh14 = m_temp_output_imag[0 as core::ffi::c_int as usize];
        let fresh15 = p_hybrid_im;
        p_hybrid_im = p_hybrid_im.offset(1);
        *fresh15 = m_temp_output_imag[1 as core::ffi::c_int as usize];
        let fresh16 = p_hybrid_im;
        p_hybrid_im = p_hybrid_im.offset(1);
        *fresh16 = m_temp_output_imag[2 as core::ffi::c_int as usize]
            + m_temp_output_imag[5 as core::ffi::c_int as usize];
        let fresh17 = p_hybrid_im;
        p_hybrid_im = p_hybrid_im.offset(1);
        *fresh17 = m_temp_output_imag[3 as core::ffi::c_int as usize]
            + m_temp_output_imag[4 as core::ffi::c_int as usize];
        ixheaacd_2ch_filtering(
            &mut *(*((*hyb_state).buffer_lf_real)
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(
                    (time_slot as core::ffi::c_int
                        + nr_samples_shift_lf as core::ffi::c_int + 1 as core::ffi::c_int
                        - PROTO_LEN) as isize,
                ),
            m_temp_output_real.as_mut_ptr(),
            hyb_tab_ptr,
        );
        ixheaacd_2ch_filtering(
            &mut *(*((*hyb_state).buffer_lf_imag)
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(
                    (time_slot as core::ffi::c_int
                        + nr_samples_shift_lf as core::ffi::c_int + 1 as core::ffi::c_int
                        - PROTO_LEN) as isize,
                ),
            m_temp_output_imag.as_mut_ptr(),
            hyb_tab_ptr,
        );
        let fresh18 = p_hybrid_re;
        p_hybrid_re = p_hybrid_re.offset(1);
        *fresh18 = m_temp_output_real[1 as core::ffi::c_int as usize];
        let fresh19 = p_hybrid_re;
        p_hybrid_re = p_hybrid_re.offset(1);
        *fresh19 = m_temp_output_real[0 as core::ffi::c_int as usize];
        let fresh20 = p_hybrid_im;
        p_hybrid_im = p_hybrid_im.offset(1);
        *fresh20 = m_temp_output_imag[1 as core::ffi::c_int as usize];
        let fresh21 = p_hybrid_im;
        p_hybrid_im = p_hybrid_im.offset(1);
        *fresh21 = m_temp_output_imag[0 as core::ffi::c_int as usize];
        ixheaacd_2ch_filtering(
            &mut *(*((*hyb_state).buffer_lf_real)
                .as_mut_ptr()
                .offset(2 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(
                    (time_slot as core::ffi::c_int
                        + nr_samples_shift_lf as core::ffi::c_int + 1 as core::ffi::c_int
                        - PROTO_LEN) as isize,
                ),
            m_temp_output_real.as_mut_ptr(),
            hyb_tab_ptr,
        );
        ixheaacd_2ch_filtering(
            &mut *(*((*hyb_state).buffer_lf_imag)
                .as_mut_ptr()
                .offset(2 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(
                    (time_slot as core::ffi::c_int
                        + nr_samples_shift_lf as core::ffi::c_int + 1 as core::ffi::c_int
                        - PROTO_LEN) as isize,
                ),
            m_temp_output_imag.as_mut_ptr(),
            hyb_tab_ptr,
        );
        let fresh22 = p_hybrid_re;
        p_hybrid_re = p_hybrid_re.offset(1);
        *fresh22 = m_temp_output_real[0 as core::ffi::c_int as usize];
        let fresh23 = p_hybrid_re;
        p_hybrid_re = p_hybrid_re.offset(1);
        *fresh23 = m_temp_output_real[1 as core::ffi::c_int as usize];
        let fresh24 = p_hybrid_im;
        p_hybrid_im = p_hybrid_im.offset(1);
        *fresh24 = m_temp_output_imag[0 as core::ffi::c_int as usize];
        let fresh25 = p_hybrid_im;
        p_hybrid_im = p_hybrid_im.offset(1);
        *fresh25 = m_temp_output_imag[1 as core::ffi::c_int as usize];
        p_hybrid_real = p_hybrid_real.offset(MAX_HYBRID_BANDS as isize);
        p_hybrid_imag = p_hybrid_imag.offset(MAX_HYBRID_BANDS as isize);
        time_slot += 1;
    }
    p_qmf_real = m_qmf_real;
    p_qmf_imag = m_qmf_imag;
    k = 0 as core::ffi::c_int as WORD32;
    while k < nr_qmf_bands_lf {
        p_qmf_re = p_qmf_real;
        p_qmf_im = p_qmf_imag;
        n = MAX_TIME_SLOTS as WORD32;
        while n < nr_samples_shift_lf {
            (*hyb_state).buffer_lf_real[k as usize][n as usize] = (*hyb_state)
                .qmf_lf_real[k as usize][n as usize];
            (*hyb_state).buffer_lf_imag[k as usize][n as usize] = (*hyb_state)
                .qmf_lf_imag[k as usize][n as usize];
            n += 1;
        }
        n = 0 as core::ffi::c_int as WORD32;
        while n < nr_samples {
            (*hyb_state)
                .buffer_lf_real[k as usize][(n + nr_samples_shift_lf) as usize] = *p_qmf_re;
            (*hyb_state)
                .buffer_lf_imag[k as usize][(n + nr_samples_shift_lf) as usize] = *p_qmf_im;
            p_qmf_re = p_qmf_re.offset(MAX_HYBRID_BANDS as isize);
            p_qmf_im = p_qmf_im.offset(MAX_HYBRID_BANDS as isize);
            n += 1;
        }
        p_qmf_real = p_qmf_real.offset(1);
        p_qmf_imag = p_qmf_imag.offset(1);
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_apply_ana_hyb_filt_bank_merge_res_decor(
    mut hyb_state: *mut ia_mps_dec_thyb_filter_state_struct,
    mut m_qmf_real: *mut WORD32,
    mut m_qmf_imag: *mut WORD32,
    mut nr_bands: WORD32,
    mut nr_samples: WORD32,
    mut m_hybrid_real: *mut WORD32,
    mut m_hybrid_imag: *mut WORD32,
    mut hyb_tab_ptr: *const ia_mps_dec_hybrid_tables_struct,
) -> VOID {
    let mut nr_samples_shift_lf: WORD32 = 0;
    let mut nr_qmf_bands_lf: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut time_slot: WORD32 = 0;
    let mut m_temp_output_real: [WORD32; 8] = [0; 8];
    let mut m_temp_output_imag: [WORD32; 8] = [0; 8];
    let mut proto_len: WORD32 = PROTO_LEN - 1 as WORD32 >> 1 as core::ffi::c_int;
    let mut val: WORD32 = nr_samples - proto_len;
    let mut loop_cnt: WORD32 = 0;
    let mut p_qmf_real: *mut WORD32 = m_qmf_real;
    let mut p_qmf_imag: *mut WORD32 = m_qmf_imag;
    let mut p_hybrid_real: *mut WORD32 = m_hybrid_real
        .offset(10 as core::ffi::c_int as isize);
    let mut p_hybrid_imag: *mut WORD32 = m_hybrid_imag
        .offset(10 as core::ffi::c_int as isize);
    let mut p_buffer_lf_real: *mut WORD32 = 0 as *mut WORD32;
    let mut p_buffer_lf_imag: *mut WORD32 = 0 as *mut WORD32;
    let mut nr_samples_x4: WORD32 = nr_samples << 2 as core::ffi::c_int;
    nr_samples_shift_lf = BUFFER_LEN_LF - nr_samples;
    nr_qmf_bands_lf = QMF_BANDS_TO_HYBRID as WORD32;
    loop_cnt = nr_bands - nr_qmf_bands_lf;
    k = 0 as core::ffi::c_int as WORD32;
    while k < nr_qmf_bands_lf {
        n = 0 as core::ffi::c_int as WORD32;
        while n < nr_samples_shift_lf {
            (*hyb_state).buffer_lf_real[k as usize][n as usize] = (*hyb_state)
                .buffer_lf_real[k as usize][(n + nr_samples) as usize];
            (*hyb_state).buffer_lf_imag[k as usize][n as usize] = (*hyb_state)
                .buffer_lf_imag[k as usize][(n + nr_samples) as usize];
            (*hyb_state).qmf_lf_real[k as usize][n as usize] = (*hyb_state)
                .qmf_lf_real[k as usize][(n + nr_samples) as usize];
            (*hyb_state).qmf_lf_imag[k as usize][n as usize] = (*hyb_state)
                .qmf_lf_imag[k as usize][(n + nr_samples) as usize];
            n += 1;
        }
        k += 1;
    }
    k = 0 as core::ffi::c_int as WORD32;
    while k < nr_qmf_bands_lf {
        let mut qmf_real: *mut WORD32 = p_qmf_real;
        let mut qmf_imag: *mut WORD32 = p_qmf_imag;
        n = 0 as core::ffi::c_int as WORD32;
        while n < nr_samples {
            (*hyb_state)
                .buffer_lf_real[k as usize][(n + nr_samples_shift_lf) as usize] = *qmf_real;
            (*hyb_state)
                .buffer_lf_imag[k as usize][(n + nr_samples_shift_lf) as usize] = *qmf_imag;
            let fresh26 = qmf_imag;
            qmf_imag = qmf_imag.offset(1);
            (*hyb_state).qmf_lf_imag[k as usize][(n + nr_samples_shift_lf) as usize] = *fresh26;
            let fresh27 = qmf_real;
            qmf_real = qmf_real.offset(1);
            (*hyb_state).qmf_lf_real[k as usize][(n + nr_samples_shift_lf) as usize] = *fresh27;
            n += 1;
        }
        p_qmf_real = p_qmf_real.offset(MAX_TIME_SLOTS as isize);
        p_qmf_imag = p_qmf_imag.offset(MAX_TIME_SLOTS as isize);
        k += 1;
    }
    p_qmf_real = m_qmf_real
        .offset((nr_qmf_bands_lf as core::ffi::c_int * MAX_TIME_SLOTS) as isize);
    p_qmf_imag = m_qmf_imag
        .offset((nr_qmf_bands_lf as core::ffi::c_int * MAX_TIME_SLOTS) as isize);
    k = 0 as core::ffi::c_int as WORD32;
    while k < loop_cnt {
        let mut qmf_real_0: *mut WORD32 = p_qmf_real.offset(val as isize);
        let mut qmf_imag_0: *mut WORD32 = p_qmf_imag.offset(val as isize);
        let mut hybrid_real: *mut WORD32 = p_hybrid_real;
        let mut hybrid_imag: *mut WORD32 = p_hybrid_imag;
        n = 0 as core::ffi::c_int as WORD32;
        while n < proto_len {
            *hybrid_real = (*hyb_state).buffer_hf_real[k as usize][n as usize];
            *hybrid_imag = (*hyb_state).buffer_hf_imag[k as usize][n as usize];
            let fresh28 = qmf_real_0;
            qmf_real_0 = qmf_real_0.offset(1);
            (*hyb_state).buffer_hf_real[k as usize][n as usize] = *fresh28;
            let fresh29 = qmf_imag_0;
            qmf_imag_0 = qmf_imag_0.offset(1);
            (*hyb_state).buffer_hf_imag[k as usize][n as usize] = *fresh29;
            hybrid_real = hybrid_real.offset(MAX_HYBRID_BANDS as isize);
            hybrid_imag = hybrid_imag.offset(MAX_HYBRID_BANDS as isize);
            n += 1;
        }
        p_qmf_real = p_qmf_real.offset(MAX_TIME_SLOTS as isize);
        p_qmf_imag = p_qmf_imag.offset(MAX_TIME_SLOTS as isize);
        p_hybrid_real = p_hybrid_real.offset(1);
        p_hybrid_imag = p_hybrid_imag.offset(1);
        k += 1;
    }
    p_qmf_real = m_qmf_real.offset(NR_QMF_BANDS_LFXTS as isize);
    p_qmf_imag = m_qmf_imag.offset(NR_QMF_BANDS_LFXTS as isize);
    p_hybrid_real = m_hybrid_real.offset(10 as core::ffi::c_int as isize);
    p_hybrid_imag = m_hybrid_imag.offset(10 as core::ffi::c_int as isize);
    k = 0 as core::ffi::c_int as WORD32;
    while k < loop_cnt {
        let mut qmf_real_1: *mut WORD32 = p_qmf_real;
        let mut qmf_imag_1: *mut WORD32 = p_qmf_imag;
        let mut hybrid_real_0: *mut WORD32 = p_hybrid_real
            .offset((proto_len as core::ffi::c_int * MAX_HYBRID_BANDS) as isize);
        let mut hybrid_imag_0: *mut WORD32 = p_hybrid_imag
            .offset((proto_len as core::ffi::c_int * MAX_HYBRID_BANDS) as isize);
        n = 0 as core::ffi::c_int as WORD32;
        while n < val {
            let fresh30 = qmf_real_1;
            qmf_real_1 = qmf_real_1.offset(1);
            *hybrid_real_0 = *fresh30;
            let fresh31 = qmf_imag_1;
            qmf_imag_1 = qmf_imag_1.offset(1);
            *hybrid_imag_0 = *fresh31;
            hybrid_real_0 = hybrid_real_0.offset(MAX_HYBRID_BANDS as isize);
            hybrid_imag_0 = hybrid_imag_0.offset(MAX_HYBRID_BANDS as isize);
            n += 1;
        }
        p_qmf_real = p_qmf_real.offset(MAX_TIME_SLOTS as isize);
        p_qmf_imag = p_qmf_imag.offset(MAX_TIME_SLOTS as isize);
        p_hybrid_real = p_hybrid_real.offset(1);
        p_hybrid_imag = p_hybrid_imag.offset(1);
        k += 1;
    }
    p_hybrid_real = m_hybrid_real;
    p_hybrid_imag = m_hybrid_imag;
    time_slot = 0 as core::ffi::c_int as WORD32;
    while time_slot < nr_samples {
        let mut hybrid_real_1: *mut WORD32 = p_hybrid_real;
        let mut hybrid_imag_1: *mut WORD32 = p_hybrid_imag;
        ixheaacd_8ch_filtering(
            &mut *(*((*hyb_state).buffer_lf_real)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(
                    (time_slot as core::ffi::c_int
                        + nr_samples_shift_lf as core::ffi::c_int + 1 as core::ffi::c_int
                        - PROTO_LEN) as isize,
                ),
            &mut *(*((*hyb_state).buffer_lf_imag)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(
                    (time_slot as core::ffi::c_int
                        + nr_samples_shift_lf as core::ffi::c_int + 1 as core::ffi::c_int
                        - PROTO_LEN) as isize,
                ),
            m_temp_output_real.as_mut_ptr(),
            m_temp_output_imag.as_mut_ptr(),
            hyb_tab_ptr,
        );
        let fresh32 = hybrid_real_1;
        hybrid_real_1 = hybrid_real_1.offset(1);
        *fresh32 = m_temp_output_real[6 as core::ffi::c_int as usize];
        let fresh33 = hybrid_real_1;
        hybrid_real_1 = hybrid_real_1.offset(1);
        *fresh33 = m_temp_output_real[7 as core::ffi::c_int as usize];
        let fresh34 = hybrid_real_1;
        hybrid_real_1 = hybrid_real_1.offset(1);
        *fresh34 = m_temp_output_real[0 as core::ffi::c_int as usize];
        let fresh35 = hybrid_real_1;
        hybrid_real_1 = hybrid_real_1.offset(1);
        *fresh35 = m_temp_output_real[1 as core::ffi::c_int as usize];
        let fresh36 = hybrid_imag_1;
        hybrid_imag_1 = hybrid_imag_1.offset(1);
        *fresh36 = m_temp_output_imag[2 as core::ffi::c_int as usize]
            + m_temp_output_imag[5 as core::ffi::c_int as usize];
        let fresh37 = hybrid_real_1;
        hybrid_real_1 = hybrid_real_1.offset(1);
        *fresh37 = m_temp_output_real[3 as core::ffi::c_int as usize]
            + m_temp_output_real[4 as core::ffi::c_int as usize];
        let fresh38 = hybrid_imag_1;
        hybrid_imag_1 = hybrid_imag_1.offset(1);
        *fresh38 = m_temp_output_imag[6 as core::ffi::c_int as usize];
        let fresh39 = hybrid_imag_1;
        hybrid_imag_1 = hybrid_imag_1.offset(1);
        *fresh39 = m_temp_output_imag[7 as core::ffi::c_int as usize];
        let fresh40 = hybrid_imag_1;
        hybrid_imag_1 = hybrid_imag_1.offset(1);
        *fresh40 = m_temp_output_imag[0 as core::ffi::c_int as usize];
        let fresh41 = hybrid_imag_1;
        hybrid_imag_1 = hybrid_imag_1.offset(1);
        *fresh41 = m_temp_output_imag[1 as core::ffi::c_int as usize];
        let fresh42 = hybrid_real_1;
        hybrid_real_1 = hybrid_real_1.offset(1);
        *fresh42 = m_temp_output_real[2 as core::ffi::c_int as usize]
            + m_temp_output_real[5 as core::ffi::c_int as usize];
        let fresh43 = hybrid_imag_1;
        hybrid_imag_1 = hybrid_imag_1.offset(1);
        *fresh43 = m_temp_output_imag[3 as core::ffi::c_int as usize]
            + m_temp_output_imag[4 as core::ffi::c_int as usize];
        ixheaacd_2ch_filtering(
            &mut *(*((*hyb_state).buffer_lf_real)
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(
                    (time_slot as core::ffi::c_int
                        + nr_samples_shift_lf as core::ffi::c_int + 1 as core::ffi::c_int
                        - PROTO_LEN) as isize,
                ),
            m_temp_output_real.as_mut_ptr(),
            hyb_tab_ptr,
        );
        ixheaacd_2ch_filtering(
            &mut *(*((*hyb_state).buffer_lf_imag)
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(
                    (time_slot as core::ffi::c_int
                        + nr_samples_shift_lf as core::ffi::c_int + 1 as core::ffi::c_int
                        - PROTO_LEN) as isize,
                ),
            m_temp_output_imag.as_mut_ptr(),
            hyb_tab_ptr,
        );
        let fresh44 = hybrid_real_1;
        hybrid_real_1 = hybrid_real_1.offset(1);
        *fresh44 = m_temp_output_real[1 as core::ffi::c_int as usize];
        let fresh45 = hybrid_real_1;
        hybrid_real_1 = hybrid_real_1.offset(1);
        *fresh45 = m_temp_output_real[0 as core::ffi::c_int as usize];
        let fresh46 = hybrid_imag_1;
        hybrid_imag_1 = hybrid_imag_1.offset(1);
        *fresh46 = m_temp_output_imag[0 as core::ffi::c_int as usize];
        let fresh47 = hybrid_imag_1;
        hybrid_imag_1 = hybrid_imag_1.offset(1);
        *fresh47 = m_temp_output_imag[1 as core::ffi::c_int as usize];
        ixheaacd_2ch_filtering(
            &mut *(*((*hyb_state).buffer_lf_real)
                .as_mut_ptr()
                .offset(2 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(
                    (time_slot as core::ffi::c_int
                        + nr_samples_shift_lf as core::ffi::c_int + 1 as core::ffi::c_int
                        - PROTO_LEN) as isize,
                ),
            m_temp_output_real.as_mut_ptr(),
            hyb_tab_ptr,
        );
        ixheaacd_2ch_filtering(
            &mut *(*((*hyb_state).buffer_lf_imag)
                .as_mut_ptr()
                .offset(2 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(
                    (time_slot as core::ffi::c_int
                        + nr_samples_shift_lf as core::ffi::c_int + 1 as core::ffi::c_int
                        - PROTO_LEN) as isize,
                ),
            m_temp_output_imag.as_mut_ptr(),
            hyb_tab_ptr,
        );
        let fresh48 = hybrid_real_1;
        hybrid_real_1 = hybrid_real_1.offset(1);
        *fresh48 = m_temp_output_real[0 as core::ffi::c_int as usize];
        let fresh49 = hybrid_real_1;
        hybrid_real_1 = hybrid_real_1.offset(1);
        *fresh49 = m_temp_output_real[1 as core::ffi::c_int as usize];
        let fresh50 = hybrid_imag_1;
        hybrid_imag_1 = hybrid_imag_1.offset(1);
        *fresh50 = m_temp_output_imag[0 as core::ffi::c_int as usize];
        let fresh51 = hybrid_imag_1;
        hybrid_imag_1 = hybrid_imag_1.offset(1);
        *fresh51 = m_temp_output_imag[1 as core::ffi::c_int as usize];
        p_hybrid_real = p_hybrid_real.offset(MAX_HYBRID_BANDS as isize);
        p_hybrid_imag = p_hybrid_imag.offset(MAX_HYBRID_BANDS as isize);
        time_slot += 1;
    }
    p_qmf_real = m_qmf_real;
    p_qmf_imag = m_qmf_imag;
    p_buffer_lf_real = &mut *(*((*hyb_state).buffer_lf_real)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize))
        .as_mut_ptr()
        .offset(nr_samples_shift_lf as isize) as *mut WORD32;
    p_buffer_lf_imag = &mut *(*((*hyb_state).buffer_lf_imag)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize))
        .as_mut_ptr()
        .offset(nr_samples_shift_lf as isize) as *mut WORD32;
    k = 0 as core::ffi::c_int as WORD32;
    while k < nr_qmf_bands_lf {
        n = MAX_TIME_SLOTS as WORD32;
        while n < nr_samples_shift_lf {
            (*hyb_state).buffer_lf_real[k as usize][n as usize] = (*hyb_state)
                .qmf_lf_real[k as usize][n as usize];
            (*hyb_state).buffer_lf_imag[k as usize][n as usize] = (*hyb_state)
                .qmf_lf_imag[k as usize][n as usize];
            n += 1;
        }
        memcpy(
            p_buffer_lf_real as *mut core::ffi::c_void,
            p_qmf_real as *const core::ffi::c_void,
            nr_samples_x4 as size_t,
        );
        memcpy(
            p_buffer_lf_imag as *mut core::ffi::c_void,
            p_qmf_imag as *const core::ffi::c_void,
            nr_samples_x4 as size_t,
        );
        p_qmf_real = p_qmf_real.offset(MAX_TIME_SLOTS as isize);
        p_qmf_imag = p_qmf_imag.offset(MAX_TIME_SLOTS as isize);
        p_buffer_lf_real = p_buffer_lf_real.offset(BUFFER_LEN_LF as isize);
        p_buffer_lf_imag = p_buffer_lf_imag.offset(BUFFER_LEN_LF as isize);
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_apply_ana_hyb_filt_bank_create_x_res(
    mut hyb_state: *mut ia_mps_dec_thyb_filter_state_struct,
    mut m_qmf_real: *mut WORD32,
    mut m_qmf_imag: *mut WORD32,
    mut nr_bands: WORD32,
    mut nr_samples: WORD32,
    mut m_hybrid_real: *mut WORD32,
    mut m_hybrid_imag: *mut WORD32,
    mut indx: *mut size_t,
    mut res: WORD32,
    mut hyb_bands: WORD32,
    mut num_parameter_bands: WORD32,
    mut counter: *mut WORD32,
    mut hyb_tab_ptr: *const ia_mps_dec_hybrid_tables_struct,
) -> VOID {
    let mut nr_samples_shift_lf: WORD32 = 0;
    let mut nr_qmf_bands_lf: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut qs: WORD32 = 0;
    let mut time_slot: WORD32 = 0;
    let mut ch_off_set: WORD32 = 0;
    let mut idx: *mut size_t = indx;
    let mut proto_len: WORD32 = PROTO_LEN - 1 as WORD32 >> 1 as core::ffi::c_int;
    let mut val: WORD32 = nr_samples - proto_len;
    let mut p_qmf_real: *mut WORD32 = m_qmf_real;
    let mut p_qmf_imag: *mut WORD32 = m_qmf_imag;
    let mut loop_cnt: WORD32 = 0;
    let mut m_temp_output_real: [WORD32; 8] = [0; 8];
    let mut m_temp_output_imag: [WORD32; 8] = [0; 8];
    let mut p_hybrid_real: *mut WORD32 = m_hybrid_real
        .offset(10 as core::ffi::c_int as isize);
    let mut p_hybrid_imag: *mut WORD32 = m_hybrid_imag
        .offset(10 as core::ffi::c_int as isize);
    let mut p_hybrid_re: *mut WORD32 = 0 as *mut WORD32;
    let mut p_hybrid_im: *mut WORD32 = 0 as *mut WORD32;
    let mut p_buffer_lf_real: *mut WORD32 = 0 as *mut WORD32;
    let mut p_buffer_lf_imag: *mut WORD32 = 0 as *mut WORD32;
    let mut nr_samples_x4: WORD32 = nr_samples << 2 as core::ffi::c_int;
    nr_samples_shift_lf = BUFFER_LEN_LF - nr_samples;
    nr_qmf_bands_lf = QMF_BANDS_TO_HYBRID as WORD32;
    loop_cnt = nr_bands - nr_qmf_bands_lf;
    ch_off_set = 0 as core::ffi::c_int as WORD32;
    k = 0 as core::ffi::c_int as WORD32;
    while k < nr_qmf_bands_lf {
        n = 0 as core::ffi::c_int as WORD32;
        while n < nr_samples_shift_lf {
            (*hyb_state).buffer_lf_real[k as usize][n as usize] = (*hyb_state)
                .buffer_lf_real[k as usize][(n + nr_samples) as usize];
            (*hyb_state).buffer_lf_imag[k as usize][n as usize] = (*hyb_state)
                .buffer_lf_imag[k as usize][(n + nr_samples) as usize];
            (*hyb_state).qmf_lf_real[k as usize][n as usize] = (*hyb_state)
                .qmf_lf_real[k as usize][(n + nr_samples) as usize];
            (*hyb_state).qmf_lf_imag[k as usize][n as usize] = (*hyb_state)
                .qmf_lf_imag[k as usize][(n + nr_samples) as usize];
            n += 1;
        }
        k += 1;
    }
    k = 0 as core::ffi::c_int as WORD32;
    while k < nr_qmf_bands_lf {
        let mut qmf_real: *mut WORD32 = p_qmf_real;
        let mut qmf_imag: *mut WORD32 = p_qmf_imag;
        n = 0 as core::ffi::c_int as WORD32;
        while n < nr_samples {
            (*hyb_state)
                .buffer_lf_real[k as usize][(n + nr_samples_shift_lf) as usize] = *qmf_real;
            (*hyb_state)
                .buffer_lf_imag[k as usize][(n + nr_samples_shift_lf) as usize] = *qmf_imag;
            let fresh52 = qmf_imag;
            qmf_imag = qmf_imag.offset(1);
            (*hyb_state).qmf_lf_imag[k as usize][(n + nr_samples_shift_lf) as usize] = *fresh52;
            let fresh53 = qmf_real;
            qmf_real = qmf_real.offset(1);
            (*hyb_state).qmf_lf_real[k as usize][(n + nr_samples_shift_lf) as usize] = *fresh53;
            n += 1;
        }
        p_qmf_real = p_qmf_real.offset(MAX_TIME_SLOTS as isize);
        p_qmf_imag = p_qmf_imag.offset(MAX_TIME_SLOTS as isize);
        k += 1;
    }
    p_qmf_real = m_qmf_real.offset(NR_QMF_BANDS_LFXTS as isize);
    p_qmf_imag = m_qmf_imag.offset(NR_QMF_BANDS_LFXTS as isize);
    k = 0 as core::ffi::c_int as WORD32;
    while k < loop_cnt {
        let mut qmf_real_0: *mut WORD32 = p_qmf_real.offset(val as isize);
        let mut qmf_imag_0: *mut WORD32 = p_qmf_imag.offset(val as isize);
        p_hybrid_re = p_hybrid_real;
        p_hybrid_im = p_hybrid_imag;
        n = 0 as core::ffi::c_int as WORD32;
        while n < proto_len {
            *p_hybrid_re = (*hyb_state).buffer_hf_real[k as usize][n as usize];
            *p_hybrid_im = (*hyb_state).buffer_hf_imag[k as usize][n as usize];
            let fresh54 = qmf_real_0;
            qmf_real_0 = qmf_real_0.offset(1);
            (*hyb_state).buffer_hf_real[k as usize][n as usize] = *fresh54;
            let fresh55 = qmf_imag_0;
            qmf_imag_0 = qmf_imag_0.offset(1);
            (*hyb_state).buffer_hf_imag[k as usize][n as usize] = *fresh55;
            p_hybrid_re = p_hybrid_re.offset(MAX_HYBRID_BANDS as isize);
            p_hybrid_im = p_hybrid_im.offset(MAX_HYBRID_BANDS as isize);
            n += 1;
        }
        p_qmf_real = p_qmf_real.offset(MAX_TIME_SLOTS as isize);
        p_qmf_imag = p_qmf_imag.offset(MAX_TIME_SLOTS as isize);
        p_hybrid_real = p_hybrid_real.offset(1);
        p_hybrid_imag = p_hybrid_imag.offset(1);
        k += 1;
    }
    p_qmf_real = m_qmf_real.offset(NR_QMF_BANDS_LFXTS as isize);
    p_qmf_imag = m_qmf_imag.offset(NR_QMF_BANDS_LFXTS as isize);
    p_hybrid_real = m_hybrid_real.offset(10 as core::ffi::c_int as isize);
    p_hybrid_imag = m_hybrid_imag.offset(10 as core::ffi::c_int as isize);
    k = 0 as core::ffi::c_int as WORD32;
    while k < loop_cnt {
        let mut qmf_real_1: *mut WORD32 = p_qmf_real;
        let mut qmf_imag_1: *mut WORD32 = p_qmf_imag;
        p_hybrid_re = p_hybrid_real
            .offset((proto_len as core::ffi::c_int * MAX_HYBRID_BANDS) as isize);
        p_hybrid_im = p_hybrid_imag
            .offset((proto_len as core::ffi::c_int * MAX_HYBRID_BANDS) as isize);
        n = 0 as core::ffi::c_int as WORD32;
        while n < val {
            let fresh56 = qmf_real_1;
            qmf_real_1 = qmf_real_1.offset(1);
            *p_hybrid_re = *fresh56;
            let fresh57 = qmf_imag_1;
            qmf_imag_1 = qmf_imag_1.offset(1);
            *p_hybrid_im = *fresh57;
            p_hybrid_re = p_hybrid_re.offset(MAX_HYBRID_BANDS as isize);
            p_hybrid_im = p_hybrid_im.offset(MAX_HYBRID_BANDS as isize);
            n += 1;
        }
        p_qmf_real = p_qmf_real.offset(MAX_TIME_SLOTS as isize);
        p_qmf_imag = p_qmf_imag.offset(MAX_TIME_SLOTS as isize);
        p_hybrid_real = p_hybrid_real.offset(1);
        p_hybrid_imag = p_hybrid_imag.offset(1);
        k += 1;
    }
    if res == 1 as core::ffi::c_int
        && (num_parameter_bands == 20 as core::ffi::c_int
            || num_parameter_bands == 28 as core::ffi::c_int)
    {
        *counter = 3 as core::ffi::c_int as WORD32;
    } else {
        idx = indx;
        qs = 0 as core::ffi::c_int as WORD32;
        while qs < hyb_bands {
            let fresh58 = idx;
            idx = idx.offset(1);
            if *fresh58 >= res as size_t {
                *counter = qs;
                qs = hyb_bands;
            }
            qs += 1;
        }
    }
    p_hybrid_real = m_hybrid_real;
    p_hybrid_imag = m_hybrid_imag;
    time_slot = 0 as core::ffi::c_int as WORD32;
    while time_slot < nr_samples {
        idx = indx;
        p_hybrid_re = p_hybrid_real;
        p_hybrid_im = p_hybrid_imag;
        ixheaacd_8ch_filtering(
            &mut *(*((*hyb_state).buffer_lf_real)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(
                    (time_slot as core::ffi::c_int
                        + nr_samples_shift_lf as core::ffi::c_int + 1 as core::ffi::c_int
                        - PROTO_LEN) as isize,
                ),
            &mut *(*((*hyb_state).buffer_lf_imag)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(
                    (time_slot as core::ffi::c_int
                        + nr_samples_shift_lf as core::ffi::c_int + 1 as core::ffi::c_int
                        - PROTO_LEN) as isize,
                ),
            m_temp_output_real.as_mut_ptr(),
            m_temp_output_imag.as_mut_ptr(),
            hyb_tab_ptr,
        );
        let fresh59 = p_hybrid_re;
        p_hybrid_re = p_hybrid_re.offset(1);
        *fresh59 = m_temp_output_real[6 as core::ffi::c_int as usize];
        let fresh60 = p_hybrid_re;
        p_hybrid_re = p_hybrid_re.offset(1);
        *fresh60 = m_temp_output_real[7 as core::ffi::c_int as usize];
        let fresh61 = p_hybrid_re;
        p_hybrid_re = p_hybrid_re.offset(1);
        *fresh61 = m_temp_output_real[0 as core::ffi::c_int as usize];
        let fresh62 = p_hybrid_re;
        p_hybrid_re = p_hybrid_re.offset(1);
        *fresh62 = m_temp_output_real[1 as core::ffi::c_int as usize];
        let fresh63 = p_hybrid_im;
        p_hybrid_im = p_hybrid_im.offset(1);
        *fresh63 = m_temp_output_imag[6 as core::ffi::c_int as usize];
        let fresh64 = p_hybrid_im;
        p_hybrid_im = p_hybrid_im.offset(1);
        *fresh64 = m_temp_output_imag[7 as core::ffi::c_int as usize];
        let fresh65 = p_hybrid_im;
        p_hybrid_im = p_hybrid_im.offset(1);
        *fresh65 = m_temp_output_imag[0 as core::ffi::c_int as usize];
        let fresh66 = p_hybrid_im;
        p_hybrid_im = p_hybrid_im.offset(1);
        *fresh66 = m_temp_output_imag[1 as core::ffi::c_int as usize];
        if *counter > 4 as core::ffi::c_int {
            let fresh67 = p_hybrid_re;
            p_hybrid_re = p_hybrid_re.offset(1);
            *fresh67 = m_temp_output_real[2 as core::ffi::c_int as usize]
                + m_temp_output_real[5 as core::ffi::c_int as usize];
            let fresh68 = p_hybrid_im;
            p_hybrid_im = p_hybrid_im.offset(1);
            *fresh68 = m_temp_output_imag[2 as core::ffi::c_int as usize]
                + m_temp_output_imag[5 as core::ffi::c_int as usize];
        }
        if *counter > 5 as core::ffi::c_int {
            let fresh69 = p_hybrid_re;
            p_hybrid_re = p_hybrid_re.offset(1);
            *fresh69 = m_temp_output_real[3 as core::ffi::c_int as usize]
                + m_temp_output_real[4 as core::ffi::c_int as usize];
            let fresh70 = p_hybrid_im;
            p_hybrid_im = p_hybrid_im.offset(1);
            *fresh70 = m_temp_output_imag[3 as core::ffi::c_int as usize]
                + m_temp_output_imag[4 as core::ffi::c_int as usize];
        }
        ch_off_set = 6 as core::ffi::c_int as WORD32;
        p_hybrid_re = p_hybrid_real.offset(ch_off_set as isize);
        p_hybrid_im = p_hybrid_imag.offset(ch_off_set as isize);
        ixheaacd_2ch_filtering(
            &mut *(*((*hyb_state).buffer_lf_real)
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(
                    (time_slot as core::ffi::c_int
                        + nr_samples_shift_lf as core::ffi::c_int + 1 as core::ffi::c_int
                        - PROTO_LEN) as isize,
                ),
            m_temp_output_real.as_mut_ptr(),
            hyb_tab_ptr,
        );
        ixheaacd_2ch_filtering(
            &mut *(*((*hyb_state).buffer_lf_imag)
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(
                    (time_slot as core::ffi::c_int
                        + nr_samples_shift_lf as core::ffi::c_int + 1 as core::ffi::c_int
                        - PROTO_LEN) as isize,
                ),
            m_temp_output_imag.as_mut_ptr(),
            hyb_tab_ptr,
        );
        let fresh71 = p_hybrid_re;
        p_hybrid_re = p_hybrid_re.offset(1);
        *fresh71 = m_temp_output_real[1 as core::ffi::c_int as usize];
        let fresh72 = p_hybrid_re;
        p_hybrid_re = p_hybrid_re.offset(1);
        *fresh72 = m_temp_output_real[0 as core::ffi::c_int as usize];
        let fresh73 = p_hybrid_im;
        p_hybrid_im = p_hybrid_im.offset(1);
        *fresh73 = m_temp_output_imag[1 as core::ffi::c_int as usize];
        let fresh74 = p_hybrid_im;
        p_hybrid_im = p_hybrid_im.offset(1);
        *fresh74 = m_temp_output_imag[0 as core::ffi::c_int as usize];
        ixheaacd_2ch_filtering(
            &mut *(*((*hyb_state).buffer_lf_real)
                .as_mut_ptr()
                .offset(2 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(
                    (time_slot as core::ffi::c_int
                        + nr_samples_shift_lf as core::ffi::c_int + 1 as core::ffi::c_int
                        - PROTO_LEN) as isize,
                ),
            m_temp_output_real.as_mut_ptr(),
            hyb_tab_ptr,
        );
        ixheaacd_2ch_filtering(
            &mut *(*((*hyb_state).buffer_lf_imag)
                .as_mut_ptr()
                .offset(2 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(
                    (time_slot as core::ffi::c_int
                        + nr_samples_shift_lf as core::ffi::c_int + 1 as core::ffi::c_int
                        - PROTO_LEN) as isize,
                ),
            m_temp_output_imag.as_mut_ptr(),
            hyb_tab_ptr,
        );
        let fresh75 = p_hybrid_re;
        p_hybrid_re = p_hybrid_re.offset(1);
        *fresh75 = m_temp_output_real[0 as core::ffi::c_int as usize];
        let fresh76 = p_hybrid_re;
        p_hybrid_re = p_hybrid_re.offset(1);
        *fresh76 = m_temp_output_real[1 as core::ffi::c_int as usize];
        let fresh77 = p_hybrid_im;
        p_hybrid_im = p_hybrid_im.offset(1);
        *fresh77 = m_temp_output_imag[0 as core::ffi::c_int as usize];
        let fresh78 = p_hybrid_im;
        p_hybrid_im = p_hybrid_im.offset(1);
        *fresh78 = m_temp_output_imag[1 as core::ffi::c_int as usize];
        p_hybrid_real = p_hybrid_real.offset(MAX_HYBRID_BANDS as isize);
        p_hybrid_imag = p_hybrid_imag.offset(MAX_HYBRID_BANDS as isize);
        time_slot += 1;
    }
    p_qmf_real = m_qmf_real;
    p_qmf_imag = m_qmf_imag;
    p_buffer_lf_real = &mut *(*((*hyb_state).buffer_lf_real)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize))
        .as_mut_ptr()
        .offset(nr_samples_shift_lf as isize) as *mut WORD32;
    p_buffer_lf_imag = &mut *(*((*hyb_state).buffer_lf_imag)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize))
        .as_mut_ptr()
        .offset(nr_samples_shift_lf as isize) as *mut WORD32;
    k = 0 as core::ffi::c_int as WORD32;
    while k < nr_qmf_bands_lf {
        n = MAX_TIME_SLOTS as WORD32;
        while n < nr_samples_shift_lf {
            (*hyb_state).buffer_lf_real[k as usize][n as usize] = (*hyb_state)
                .qmf_lf_real[k as usize][n as usize];
            (*hyb_state).buffer_lf_imag[k as usize][n as usize] = (*hyb_state)
                .qmf_lf_imag[k as usize][n as usize];
            n += 1;
        }
        memcpy(
            p_buffer_lf_real as *mut core::ffi::c_void,
            p_qmf_real as *const core::ffi::c_void,
            nr_samples_x4 as size_t,
        );
        memcpy(
            p_buffer_lf_imag as *mut core::ffi::c_void,
            p_qmf_imag as *const core::ffi::c_void,
            nr_samples_x4 as size_t,
        );
        p_qmf_real = p_qmf_real.offset(MAX_TIME_SLOTS as isize);
        p_qmf_imag = p_qmf_imag.offset(MAX_TIME_SLOTS as isize);
        p_buffer_lf_real = p_buffer_lf_real.offset(BUFFER_LEN_LF as isize);
        p_buffer_lf_imag = p_buffer_lf_imag.offset(BUFFER_LEN_LF as isize);
        k += 1;
    }
}
