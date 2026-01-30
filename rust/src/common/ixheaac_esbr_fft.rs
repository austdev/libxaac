extern "C" {
    static ixheaac_twiddle_table_fft_float: [FLOAT32; 514];
    static ixheaac_twidle_tbl_48: [FLOAT32; 64];
    static ixheaac_twidle_tbl_24: [FLOAT32; 32];
}
pub type size_t = usize;
pub type WORD32 = core::ffi::c_int;
pub type FLOAT32 = core::ffi::c_float;
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
pub unsafe extern "C" fn ixheaac_real_synth_fft_p2(
    mut ptr_x: *mut FLOAT32,
    mut ptr_y: *mut FLOAT32,
    mut npoints: WORD32,
) {
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
    let mut ptr_w: *const FLOAT32 = 0 as *const FLOAT32;
    dig_rev_shift = (ixheaac_norm32(npoints) as core::ffi::c_int + 1 as core::ffi::c_int
        - 16 as core::ffi::c_int) as WORD32;
    n_stages = (30 as WORD - ixheaac_norm32(npoints)) as WORD32;
    not_power_4 = (n_stages as core::ffi::c_int & 1 as core::ffi::c_int) as WORD32;
    n_stages = n_stages >> 1 as core::ffi::c_int;
    ptr_w = ixheaac_twiddle_table_fft_float.as_ptr();
    i = 0 as core::ffi::c_int as WORD32;
    while i < npoints {
        let mut inp: *mut FLOAT32 = ptr_x;
        let mut _: core::ffi::c_uint = i as core::ffi::c_uint;
        _ = (_ & 0x33333333 as core::ffi::c_uint) << 2 as core::ffi::c_int
            | (_ & !(0x33333333 as core::ffi::c_int) as core::ffi::c_uint)
                >> 2 as core::ffi::c_int;
        _ = (_ & 0xf0f0f0f as core::ffi::c_uint) << 4 as core::ffi::c_int
            | (_ & !(0xf0f0f0f as core::ffi::c_int) as core::ffi::c_uint)
                >> 4 as core::ffi::c_int;
        _ = (_ & 0xff00ff as core::ffi::c_uint) << 8 as core::ffi::c_int
            | (_ & !(0xff00ff as core::ffi::c_int) as core::ffi::c_uint)
                >> 8 as core::ffi::c_int;
        h2 = (_ >> dig_rev_shift) as WORD32;
        if not_power_4 != 0 {
            h2 += 1 as core::ffi::c_int;
            h2 &= !(1 as core::ffi::c_int);
        }
        inp = inp.offset((h2 >> 1 as core::ffi::c_int) as isize);
        x0r = *inp;
        inp = inp.offset((npoints >> 2 as core::ffi::c_int) as isize);
        x1r = *inp;
        inp = inp.offset((npoints >> 2 as core::ffi::c_int) as isize);
        x2r = *inp;
        inp = inp.offset((npoints >> 2 as core::ffi::c_int) as isize);
        x3r = *inp;
        x0r = x0r + x2r;
        x2r = x0r - x2r * 2 as core::ffi::c_int as FLOAT32;
        x1r = x1r + x3r;
        x3r = x1r - x3r * 2 as core::ffi::c_int as FLOAT32;
        x0r = x0r + x1r;
        x1r = x0r - x1r * 2 as core::ffi::c_int as FLOAT32;
        let fresh0 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *fresh0 = x0r;
        let fresh1 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *fresh1 = 0 as core::ffi::c_int as FLOAT32;
        let fresh2 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *fresh2 = x2r;
        let fresh3 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *fresh3 = x3r;
        let fresh4 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *fresh4 = x1r;
        let fresh5 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *fresh5 = 0 as core::ffi::c_int as FLOAT32;
        let fresh6 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *fresh6 = x2r;
        let fresh7 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *fresh7 = -x3r;
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
        let mut W1: FLOAT32 = 0.;
        let mut W2: FLOAT32 = 0.;
        let mut W3: FLOAT32 = 0.;
        let mut W4: FLOAT32 = 0.;
        let mut W5: FLOAT32 = 0.;
        let mut W6: FLOAT32 = 0.;
        let mut sec_loop_cnt: WORD32 = 0;
        k = in_loop_cnt;
        while k != 0 as core::ffi::c_int {
            x0r = *data;
            x0i = *data.offset(1 as core::ffi::c_int as isize);
            data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
            x1r = *data;
            x1i = *data.offset(1 as core::ffi::c_int as isize);
            data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
            x2r = *data;
            x2i = *data.offset(1 as core::ffi::c_int as isize);
            data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
            x3r = *data;
            x3i = *data.offset(1 as core::ffi::c_int as isize);
            data = data
                .offset(
                    -((3 as size_t)
                        .wrapping_mul((del as size_t) << 1 as core::ffi::c_int) as isize),
                );
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
            *data = x0r;
            *data.offset(1 as core::ffi::c_int as isize) = x0i;
            data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
            *data = x2r;
            *data.offset(1 as core::ffi::c_int as isize) = x2i;
            data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
            *data = x1r;
            *data.offset(1 as core::ffi::c_int as isize) = x1i;
            data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
            *data = x3i;
            *data.offset(1 as core::ffi::c_int as isize) = x3r;
            data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
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
        while j <= sec_loop_cnt {
            W1 = *twiddles.offset(j as isize);
            W4 = *twiddles.offset(j as isize).offset(257 as core::ffi::c_int as isize);
            W2 = *twiddles.offset(((j as size_t) << 1 as core::ffi::c_int) as isize);
            W5 = *twiddles
                .offset(((j as size_t) << 1 as core::ffi::c_int) as isize)
                .offset(257 as core::ffi::c_int as isize);
            W3 = *twiddles
                .offset(j as isize)
                .offset(((j as size_t) << 1 as core::ffi::c_int) as isize);
            W6 = *twiddles
                .offset(j as isize)
                .offset(((j as size_t) << 1 as core::ffi::c_int) as isize)
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
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                x1r_0 = *data;
                x1i_0 = *data.offset(1 as core::ffi::c_int as isize);
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                x2r_0 = *data;
                x2i_0 = *data.offset(1 as core::ffi::c_int as isize);
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                x3r_0 = *data;
                x3i_0 = *data.offset(1 as core::ffi::c_int as isize);
                data = data
                    .offset(
                        -((3 as size_t)
                            .wrapping_mul((del as size_t) << 1 as core::ffi::c_int)
                            as isize),
                    );
                tmp = x1r_0 * W1 + x1i_0 * W4;
                x1i_0 = -(x1r_0 * W4) + x1i_0 * W1;
                x1r_0 = tmp;
                tmp = x2r_0 * W2 + x2i_0 * W5;
                x2i_0 = -(x2r_0 * W5) + x2i_0 * W2;
                x2r_0 = tmp;
                tmp = x3r_0 * W3 + x3i_0 * W6;
                x3i_0 = -(x3r_0 * W6) + x3i_0 * W3;
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
                x2r_0 = x2r_0 - x3i_0;
                x2i_0 = x2i_0 + x3r_0;
                x3i_0 = x2r_0 + x3i_0 * 2 as core::ffi::c_int as FLOAT32;
                x3r_0 = x2i_0 - x3r_0 * 2 as core::ffi::c_int as FLOAT32;
                *data = x0r_0;
                *data.offset(1 as core::ffi::c_int as isize) = x0i_0;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                *data = x2r_0;
                *data.offset(1 as core::ffi::c_int as isize) = x2i_0;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                *data = x1r_0;
                *data.offset(1 as core::ffi::c_int as isize) = x1i_0;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                *data = x3i_0;
                *data.offset(1 as core::ffi::c_int as isize) = x3r_0;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                k -= 1;
            }
            data = data.offset(-((2 as WORD32 * npoints) as isize));
            data = data.offset(2 as core::ffi::c_int as isize);
            j += nodespacing;
        }
        while j <= nodespacing * del >> 1 as core::ffi::c_int {
            W1 = *twiddles.offset(j as isize);
            W4 = *twiddles.offset(j as isize).offset(257 as core::ffi::c_int as isize);
            W2 = *twiddles.offset(((j as size_t) << 1 as core::ffi::c_int) as isize);
            W5 = *twiddles
                .offset(((j as size_t) << 1 as core::ffi::c_int) as isize)
                .offset(257 as core::ffi::c_int as isize);
            W3 = *twiddles
                .offset(j as isize)
                .offset(((j as size_t) << 1 as core::ffi::c_int) as isize)
                .offset(-(256 as core::ffi::c_int as isize));
            W6 = *twiddles
                .offset(j as isize)
                .offset(((j as size_t) << 1 as core::ffi::c_int) as isize)
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
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                x1r_1 = *data;
                x1i_1 = *data.offset(1 as core::ffi::c_int as isize);
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                x2r_1 = *data;
                x2i_1 = *data.offset(1 as core::ffi::c_int as isize);
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                x3r_1 = *data;
                x3i_1 = *data.offset(1 as core::ffi::c_int as isize);
                data = data
                    .offset(
                        -((3 as size_t)
                            .wrapping_mul((del as size_t) << 1 as core::ffi::c_int)
                            as isize),
                    );
                tmp_0 = x1r_1 * W1 + x1i_1 * W4;
                x1i_1 = -(x1r_1 * W4) + x1i_1 * W1;
                x1r_1 = tmp_0;
                tmp_0 = x2r_1 * W2 + x2i_1 * W5;
                x2i_1 = -(x2r_1 * W5) + x2i_1 * W2;
                x2r_1 = tmp_0;
                tmp_0 = x3r_1 * W6 - x3i_1 * W3;
                x3i_1 = x3r_1 * W3 + x3i_1 * W6;
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
                x2r_1 = x2r_1 - x3i_1;
                x2i_1 = x2i_1 + x3r_1;
                x3i_1 = x2r_1 + x3i_1 * 2 as core::ffi::c_int as FLOAT32;
                x3r_1 = x2i_1 - x3r_1 * 2 as core::ffi::c_int as FLOAT32;
                *data = x0r_1;
                *data.offset(1 as core::ffi::c_int as isize) = x0i_1;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                *data = x2r_1;
                *data.offset(1 as core::ffi::c_int as isize) = x2i_1;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                *data = x1r_1;
                *data.offset(1 as core::ffi::c_int as isize) = x1i_1;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                *data = x3i_1;
                *data.offset(1 as core::ffi::c_int as isize) = x3r_1;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                k -= 1;
            }
            data = data.offset(-((2 as WORD32 * npoints) as isize));
            data = data.offset(2 as core::ffi::c_int as isize);
            j += nodespacing;
        }
        while j <= sec_loop_cnt as core::ffi::c_int * 2 as core::ffi::c_int {
            W1 = *twiddles.offset(j as isize);
            W4 = *twiddles.offset(j as isize).offset(257 as core::ffi::c_int as isize);
            W2 = *twiddles
                .offset(((j as size_t) << 1 as core::ffi::c_int) as isize)
                .offset(-(256 as core::ffi::c_int as isize));
            W5 = *twiddles
                .offset(((j as size_t) << 1 as core::ffi::c_int) as isize)
                .offset(1 as core::ffi::c_int as isize);
            W3 = *twiddles
                .offset(j as isize)
                .offset(((j as size_t) << 1 as core::ffi::c_int) as isize)
                .offset(-(256 as core::ffi::c_int as isize));
            W6 = *twiddles
                .offset(j as isize)
                .offset(((j as size_t) << 1 as core::ffi::c_int) as isize)
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
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                x1r_2 = *data;
                x1i_2 = *data.offset(1 as core::ffi::c_int as isize);
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                x2r_2 = *data;
                x2i_2 = *data.offset(1 as core::ffi::c_int as isize);
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                x3r_2 = *data;
                x3i_2 = *data.offset(1 as core::ffi::c_int as isize);
                data = data
                    .offset(
                        -((3 as size_t)
                            .wrapping_mul((del as size_t) << 1 as core::ffi::c_int)
                            as isize),
                    );
                tmp_1 = x1r_2 * W1 + x1i_2 * W4;
                x1i_2 = -(x1r_2 * W4) + x1i_2 * W1;
                x1r_2 = tmp_1;
                tmp_1 = x2r_2 * W5 - x2i_2 * W2;
                x2i_2 = x2r_2 * W2 + x2i_2 * W5;
                x2r_2 = tmp_1;
                tmp_1 = x3r_2 * W6 - x3i_2 * W3;
                x3i_2 = x3r_2 * W3 + x3i_2 * W6;
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
                x2r_2 = x2r_2 - x3i_2;
                x2i_2 = x2i_2 + x3r_2;
                x3i_2 = x2r_2 + x3i_2 * 2 as core::ffi::c_int as FLOAT32;
                x3r_2 = x2i_2 - x3r_2 * 2 as core::ffi::c_int as FLOAT32;
                *data = x0r_2;
                *data.offset(1 as core::ffi::c_int as isize) = x0i_2;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                *data = x2r_2;
                *data.offset(1 as core::ffi::c_int as isize) = x2i_2;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                *data = x1r_2;
                *data.offset(1 as core::ffi::c_int as isize) = x1i_2;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                *data = x3i_2;
                *data.offset(1 as core::ffi::c_int as isize) = x3r_2;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                k -= 1;
            }
            data = data.offset(-((2 as WORD32 * npoints) as isize));
            data = data.offset(2 as core::ffi::c_int as isize);
            j += nodespacing;
        }
        while j < nodespacing * del {
            W1 = *twiddles.offset(j as isize);
            W4 = *twiddles.offset(j as isize).offset(257 as core::ffi::c_int as isize);
            W2 = *twiddles
                .offset(((j as size_t) << 1 as core::ffi::c_int) as isize)
                .offset(-(256 as core::ffi::c_int as isize));
            W5 = *twiddles
                .offset(((j as size_t) << 1 as core::ffi::c_int) as isize)
                .offset(1 as core::ffi::c_int as isize);
            W3 = *twiddles
                .offset(j as isize)
                .offset(((j as size_t) << 1 as core::ffi::c_int) as isize)
                .offset(-(512 as core::ffi::c_int as isize));
            W6 = *twiddles
                .offset(j as isize)
                .offset(((j as size_t) << 1 as core::ffi::c_int) as isize)
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
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                x1r_3 = *data;
                x1i_3 = *data.offset(1 as core::ffi::c_int as isize);
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                x2r_3 = *data;
                x2i_3 = *data.offset(1 as core::ffi::c_int as isize);
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                x3r_3 = *data;
                x3i_3 = *data.offset(1 as core::ffi::c_int as isize);
                data = data
                    .offset(
                        -((3 as size_t)
                            .wrapping_mul((del as size_t) << 1 as core::ffi::c_int)
                            as isize),
                    );
                tmp_2 = x1r_3 * W1 + x1i_3 * W4;
                x1i_3 = -(x1r_3 * W4) + x1i_3 * W1;
                x1r_3 = tmp_2;
                tmp_2 = x2r_3 * W5 - x2i_3 * W2;
                x2i_3 = x2r_3 * W2 + x2i_3 * W5;
                x2r_3 = tmp_2;
                tmp_2 = -(x3r_3 * W3) - x3i_3 * W6;
                x3i_3 = -(x3r_3 * W6) + x3i_3 * W3;
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
                x2r_3 = x2r_3 - x3i_3;
                x2i_3 = x2i_3 + x3r_3;
                x3i_3 = x2r_3 + x3i_3 * 2 as core::ffi::c_int as FLOAT32;
                x3r_3 = x2i_3 - x3r_3 * 2 as core::ffi::c_int as FLOAT32;
                *data = x0r_3;
                *data.offset(1 as core::ffi::c_int as isize) = x0i_3;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                *data = x2r_3;
                *data.offset(1 as core::ffi::c_int as isize) = x2i_3;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                *data = x1r_3;
                *data.offset(1 as core::ffi::c_int as isize) = x1i_3;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                *data = x3i_3;
                *data.offset(1 as core::ffi::c_int as isize) = x3r_3;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
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
            let mut W1_0: FLOAT32 = *twiddles_0;
            let mut W4_0: FLOAT32 = *twiddles_0.offset(257 as core::ffi::c_int as isize);
            let mut tmp_3: FLOAT32 = 0.;
            twiddles_0 = twiddles_0.offset(nodespacing as isize);
            x0r = *ptr_y;
            x0i = *ptr_y.offset(1 as core::ffi::c_int as isize);
            ptr_y = ptr_y.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
            x1r = *ptr_y;
            x1i = *ptr_y.offset(1 as core::ffi::c_int as isize);
            tmp_3 = x1r * W1_0 + x1i * W4_0;
            x1i = -(x1r * W4_0) + x1i * W1_0;
            x1r = tmp_3;
            *ptr_y = x0r - x1r;
            *ptr_y.offset(1 as core::ffi::c_int as isize) = x0i - x1i;
            ptr_y = ptr_y.offset(-(((del as size_t) << 1 as core::ffi::c_int) as isize));
            *ptr_y = x0r + x1r;
            *ptr_y.offset(1 as core::ffi::c_int as isize) = x0i + x1i;
            ptr_y = ptr_y.offset(2 as core::ffi::c_int as isize);
            j -= 1;
        }
        twiddles_0 = ptr_w;
        j = (del as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
        while j != 0 as core::ffi::c_int {
            let mut W1_1: FLOAT32 = *twiddles_0;
            let mut W4_1: FLOAT32 = *twiddles_0.offset(257 as core::ffi::c_int as isize);
            let mut tmp_4: FLOAT32 = 0.;
            twiddles_0 = twiddles_0.offset(nodespacing as isize);
            x0r = *ptr_y;
            x0i = *ptr_y.offset(1 as core::ffi::c_int as isize);
            ptr_y = ptr_y.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
            x1r = *ptr_y;
            x1i = *ptr_y.offset(1 as core::ffi::c_int as isize);
            tmp_4 = x1r * W4_1 - x1i * W1_1;
            x1i = x1r * W1_1 + x1i * W4_1;
            x1r = tmp_4;
            *ptr_y = x0r - x1r;
            *ptr_y.offset(1 as core::ffi::c_int as isize) = x0i - x1i;
            ptr_y = ptr_y.offset(-(((del as size_t) << 1 as core::ffi::c_int) as isize));
            *ptr_y = x0r + x1r;
            *ptr_y.offset(1 as core::ffi::c_int as isize) = x0i + x1i;
            ptr_y = ptr_y.offset(2 as core::ffi::c_int as isize);
            j -= 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaac_cmplx_anal_fft_p2(
    mut ptr_x: *mut FLOAT32,
    mut ptr_y: *mut FLOAT32,
    mut npoints: WORD32,
) {
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
    let mut ptr_w: *const FLOAT32 = 0 as *const FLOAT32;
    dig_rev_shift = (ixheaac_norm32(npoints) as core::ffi::c_int + 1 as core::ffi::c_int
        - 16 as core::ffi::c_int) as WORD32;
    n_stages = (30 as WORD - ixheaac_norm32(npoints)) as WORD32;
    not_power_4 = (n_stages as core::ffi::c_int & 1 as core::ffi::c_int) as WORD32;
    n_stages = n_stages >> 1 as core::ffi::c_int;
    ptr_w = ixheaac_twiddle_table_fft_float.as_ptr();
    i = 0 as core::ffi::c_int as WORD32;
    while i < npoints {
        let mut inp: *mut FLOAT32 = ptr_x;
        let mut _: core::ffi::c_uint = i as core::ffi::c_uint;
        _ = (_ & 0x33333333 as core::ffi::c_uint) << 2 as core::ffi::c_int
            | (_ & !(0x33333333 as core::ffi::c_int) as core::ffi::c_uint)
                >> 2 as core::ffi::c_int;
        _ = (_ & 0xf0f0f0f as core::ffi::c_uint) << 4 as core::ffi::c_int
            | (_ & !(0xf0f0f0f as core::ffi::c_int) as core::ffi::c_uint)
                >> 4 as core::ffi::c_int;
        _ = (_ & 0xff00ff as core::ffi::c_uint) << 8 as core::ffi::c_int
            | (_ & !(0xff00ff as core::ffi::c_int) as core::ffi::c_uint)
                >> 8 as core::ffi::c_int;
        h2 = (_ >> dig_rev_shift) as WORD32;
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
        let mut twiddles: *const FLOAT32 = ptr_w;
        let mut data: *mut FLOAT32 = ptr_y;
        let mut W1: FLOAT32 = 0.;
        let mut W2: FLOAT32 = 0.;
        let mut W3: FLOAT32 = 0.;
        let mut W4: FLOAT32 = 0.;
        let mut W5: FLOAT32 = 0.;
        let mut W6: FLOAT32 = 0.;
        let mut sec_loop_cnt: WORD32 = 0;
        k = in_loop_cnt;
        while k != 0 as core::ffi::c_int {
            x0r = *data;
            x0i = *data.offset(1 as core::ffi::c_int as isize);
            data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
            x1r = *data;
            x1i = *data.offset(1 as core::ffi::c_int as isize);
            data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
            x2r = *data;
            x2i = *data.offset(1 as core::ffi::c_int as isize);
            data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
            x3r = *data;
            x3i = *data.offset(1 as core::ffi::c_int as isize);
            data = data
                .offset(
                    -((3 as size_t)
                        .wrapping_mul((del as size_t) << 1 as core::ffi::c_int) as isize),
                );
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
            *data = x0r;
            *data.offset(1 as core::ffi::c_int as isize) = x0i;
            data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
            *data = x2r;
            *data.offset(1 as core::ffi::c_int as isize) = x2i;
            data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
            *data = x1r;
            *data.offset(1 as core::ffi::c_int as isize) = x1i;
            data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
            *data = x3i;
            *data.offset(1 as core::ffi::c_int as isize) = x3r;
            data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
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
        while j <= sec_loop_cnt {
            W1 = *twiddles.offset(j as isize);
            W4 = *twiddles.offset(j as isize).offset(257 as core::ffi::c_int as isize);
            W2 = *twiddles.offset(((j as size_t) << 1 as core::ffi::c_int) as isize);
            W5 = *twiddles
                .offset(((j as size_t) << 1 as core::ffi::c_int) as isize)
                .offset(257 as core::ffi::c_int as isize);
            W3 = *twiddles
                .offset(j as isize)
                .offset(((j as size_t) << 1 as core::ffi::c_int) as isize);
            W6 = *twiddles
                .offset(j as isize)
                .offset(((j as size_t) << 1 as core::ffi::c_int) as isize)
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
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                x1r_0 = *data;
                x1i_0 = *data.offset(1 as core::ffi::c_int as isize);
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                x2r_0 = *data;
                x2i_0 = *data.offset(1 as core::ffi::c_int as isize);
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                x3r_0 = *data;
                x3i_0 = *data.offset(1 as core::ffi::c_int as isize);
                data = data
                    .offset(
                        -((3 as size_t)
                            .wrapping_mul((del as size_t) << 1 as core::ffi::c_int)
                            as isize),
                    );
                tmp = x1r_0 * W1 + x1i_0 * W4;
                x1i_0 = -(x1r_0 * W4) + x1i_0 * W1;
                x1r_0 = tmp;
                tmp = x2r_0 * W2 + x2i_0 * W5;
                x2i_0 = -(x2r_0 * W5) + x2i_0 * W2;
                x2r_0 = tmp;
                tmp = x3r_0 * W3 + x3i_0 * W6;
                x3i_0 = -(x3r_0 * W6) + x3i_0 * W3;
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
                x2r_0 = x2r_0 - x3i_0;
                x2i_0 = x2i_0 + x3r_0;
                x3i_0 = x2r_0 + x3i_0 * 2 as core::ffi::c_int as FLOAT32;
                x3r_0 = x2i_0 - x3r_0 * 2 as core::ffi::c_int as FLOAT32;
                *data = x0r_0;
                *data.offset(1 as core::ffi::c_int as isize) = x0i_0;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                *data = x2r_0;
                *data.offset(1 as core::ffi::c_int as isize) = x2i_0;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                *data = x1r_0;
                *data.offset(1 as core::ffi::c_int as isize) = x1i_0;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                *data = x3i_0;
                *data.offset(1 as core::ffi::c_int as isize) = x3r_0;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                k -= 1;
            }
            data = data.offset(-((2 as WORD32 * npoints) as isize));
            data = data.offset(2 as core::ffi::c_int as isize);
            j += nodespacing;
        }
        while j <= nodespacing * del >> 1 as core::ffi::c_int {
            W1 = *twiddles.offset(j as isize);
            W4 = *twiddles.offset(j as isize).offset(257 as core::ffi::c_int as isize);
            W2 = *twiddles.offset(((j as size_t) << 1 as core::ffi::c_int) as isize);
            W5 = *twiddles
                .offset(((j as size_t) << 1 as core::ffi::c_int) as isize)
                .offset(257 as core::ffi::c_int as isize);
            W3 = *twiddles
                .offset(j as isize)
                .offset(((j as size_t) << 1 as core::ffi::c_int) as isize)
                .offset(-(256 as core::ffi::c_int as isize));
            W6 = *twiddles
                .offset(j as isize)
                .offset(((j as size_t) << 1 as core::ffi::c_int) as isize)
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
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                x1r_1 = *data;
                x1i_1 = *data.offset(1 as core::ffi::c_int as isize);
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                x2r_1 = *data;
                x2i_1 = *data.offset(1 as core::ffi::c_int as isize);
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                x3r_1 = *data;
                x3i_1 = *data.offset(1 as core::ffi::c_int as isize);
                data = data
                    .offset(
                        -((3 as size_t)
                            .wrapping_mul((del as size_t) << 1 as core::ffi::c_int)
                            as isize),
                    );
                tmp_0 = x1r_1 * W1 + x1i_1 * W4;
                x1i_1 = -(x1r_1 * W4) + x1i_1 * W1;
                x1r_1 = tmp_0;
                tmp_0 = x2r_1 * W2 + x2i_1 * W5;
                x2i_1 = -(x2r_1 * W5) + x2i_1 * W2;
                x2r_1 = tmp_0;
                tmp_0 = x3r_1 * W6 - x3i_1 * W3;
                x3i_1 = x3r_1 * W3 + x3i_1 * W6;
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
                x2r_1 = x2r_1 - x3i_1;
                x2i_1 = x2i_1 + x3r_1;
                x3i_1 = x2r_1 + x3i_1 * 2 as core::ffi::c_int as FLOAT32;
                x3r_1 = x2i_1 - x3r_1 * 2 as core::ffi::c_int as FLOAT32;
                *data = x0r_1;
                *data.offset(1 as core::ffi::c_int as isize) = x0i_1;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                *data = x2r_1;
                *data.offset(1 as core::ffi::c_int as isize) = x2i_1;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                *data = x1r_1;
                *data.offset(1 as core::ffi::c_int as isize) = x1i_1;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                *data = x3i_1;
                *data.offset(1 as core::ffi::c_int as isize) = x3r_1;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                k -= 1;
            }
            data = data.offset(-((2 as WORD32 * npoints) as isize));
            data = data.offset(2 as core::ffi::c_int as isize);
            j += nodespacing;
        }
        while j <= sec_loop_cnt as core::ffi::c_int * 2 as core::ffi::c_int {
            W1 = *twiddles.offset(j as isize);
            W4 = *twiddles.offset(j as isize).offset(257 as core::ffi::c_int as isize);
            W2 = *twiddles
                .offset(((j as size_t) << 1 as core::ffi::c_int) as isize)
                .offset(-(256 as core::ffi::c_int as isize));
            W5 = *twiddles
                .offset(((j as size_t) << 1 as core::ffi::c_int) as isize)
                .offset(1 as core::ffi::c_int as isize);
            W3 = *twiddles
                .offset(j as isize)
                .offset(((j as size_t) << 1 as core::ffi::c_int) as isize)
                .offset(-(256 as core::ffi::c_int as isize));
            W6 = *twiddles
                .offset(j as isize)
                .offset(((j as size_t) << 1 as core::ffi::c_int) as isize)
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
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                x1r_2 = *data;
                x1i_2 = *data.offset(1 as core::ffi::c_int as isize);
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                x2r_2 = *data;
                x2i_2 = *data.offset(1 as core::ffi::c_int as isize);
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                x3r_2 = *data;
                x3i_2 = *data.offset(1 as core::ffi::c_int as isize);
                data = data
                    .offset(
                        -((3 as size_t)
                            .wrapping_mul((del as size_t) << 1 as core::ffi::c_int)
                            as isize),
                    );
                tmp_1 = x1r_2 * W1 + x1i_2 * W4;
                x1i_2 = -(x1r_2 * W4) + x1i_2 * W1;
                x1r_2 = tmp_1;
                tmp_1 = x2r_2 * W5 - x2i_2 * W2;
                x2i_2 = x2r_2 * W2 + x2i_2 * W5;
                x2r_2 = tmp_1;
                tmp_1 = x3r_2 * W6 - x3i_2 * W3;
                x3i_2 = x3r_2 * W3 + x3i_2 * W6;
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
                x2r_2 = x2r_2 - x3i_2;
                x2i_2 = x2i_2 + x3r_2;
                x3i_2 = x2r_2 + x3i_2 * 2 as core::ffi::c_int as FLOAT32;
                x3r_2 = x2i_2 - x3r_2 * 2 as core::ffi::c_int as FLOAT32;
                *data = x0r_2;
                *data.offset(1 as core::ffi::c_int as isize) = x0i_2;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                *data = x2r_2;
                *data.offset(1 as core::ffi::c_int as isize) = x2i_2;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                *data = x1r_2;
                *data.offset(1 as core::ffi::c_int as isize) = x1i_2;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                *data = x3i_2;
                *data.offset(1 as core::ffi::c_int as isize) = x3r_2;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                k -= 1;
            }
            data = data.offset(-((2 as WORD32 * npoints) as isize));
            data = data.offset(2 as core::ffi::c_int as isize);
            j += nodespacing;
        }
        while j < nodespacing * del {
            W1 = *twiddles.offset(j as isize);
            W4 = *twiddles.offset(j as isize).offset(257 as core::ffi::c_int as isize);
            W2 = *twiddles
                .offset(((j as size_t) << 1 as core::ffi::c_int) as isize)
                .offset(-(256 as core::ffi::c_int as isize));
            W5 = *twiddles
                .offset(((j as size_t) << 1 as core::ffi::c_int) as isize)
                .offset(1 as core::ffi::c_int as isize);
            W3 = *twiddles
                .offset(j as isize)
                .offset(((j as size_t) << 1 as core::ffi::c_int) as isize)
                .offset(-(512 as core::ffi::c_int as isize));
            W6 = *twiddles
                .offset(j as isize)
                .offset(((j as size_t) << 1 as core::ffi::c_int) as isize)
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
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                x1r_3 = *data;
                x1i_3 = *data.offset(1 as core::ffi::c_int as isize);
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                x2r_3 = *data;
                x2i_3 = *data.offset(1 as core::ffi::c_int as isize);
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                x3r_3 = *data;
                x3i_3 = *data.offset(1 as core::ffi::c_int as isize);
                data = data
                    .offset(
                        -((3 as size_t)
                            .wrapping_mul((del as size_t) << 1 as core::ffi::c_int)
                            as isize),
                    );
                tmp_2 = x1r_3 * W1 + x1i_3 * W4;
                x1i_3 = -(x1r_3 * W4) + x1i_3 * W1;
                x1r_3 = tmp_2;
                tmp_2 = x2r_3 * W5 - x2i_3 * W2;
                x2i_3 = x2r_3 * W2 + x2i_3 * W5;
                x2r_3 = tmp_2;
                tmp_2 = -(x3r_3 * W3) - x3i_3 * W6;
                x3i_3 = -(x3r_3 * W6) + x3i_3 * W3;
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
                x2r_3 = x2r_3 - x3i_3;
                x2i_3 = x2i_3 + x3r_3;
                x3i_3 = x2r_3 + x3i_3 * 2 as core::ffi::c_int as FLOAT32;
                x3r_3 = x2i_3 - x3r_3 * 2 as core::ffi::c_int as FLOAT32;
                *data = x0r_3;
                *data.offset(1 as core::ffi::c_int as isize) = x0i_3;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                *data = x2r_3;
                *data.offset(1 as core::ffi::c_int as isize) = x2i_3;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                *data = x1r_3;
                *data.offset(1 as core::ffi::c_int as isize) = x1i_3;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
                *data = x3i_3;
                *data.offset(1 as core::ffi::c_int as isize) = x3r_3;
                data = data.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
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
            let mut W1_0: FLOAT32 = *twiddles_0;
            let mut W4_0: FLOAT32 = *twiddles_0.offset(257 as core::ffi::c_int as isize);
            let mut tmp_3: FLOAT32 = 0.;
            twiddles_0 = twiddles_0.offset(nodespacing as isize);
            x0r = *ptr_y;
            x0i = *ptr_y.offset(1 as core::ffi::c_int as isize);
            ptr_y = ptr_y.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
            x1r = *ptr_y;
            x1i = *ptr_y.offset(1 as core::ffi::c_int as isize);
            tmp_3 = x1r * W1_0 + x1i * W4_0;
            x1i = -(x1r * W4_0) + x1i * W1_0;
            x1r = tmp_3;
            *ptr_y = x0r - x1r;
            *ptr_y.offset(1 as core::ffi::c_int as isize) = x0i - x1i;
            ptr_y = ptr_y.offset(-(((del as size_t) << 1 as core::ffi::c_int) as isize));
            *ptr_y = x0r + x1r;
            *ptr_y.offset(1 as core::ffi::c_int as isize) = x0i + x1i;
            ptr_y = ptr_y.offset(2 as core::ffi::c_int as isize);
            j -= 1;
        }
        twiddles_0 = ptr_w;
        j = (del as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
        while j != 0 as core::ffi::c_int {
            let mut W1_1: FLOAT32 = *twiddles_0;
            let mut W4_1: FLOAT32 = *twiddles_0.offset(257 as core::ffi::c_int as isize);
            let mut tmp_4: FLOAT32 = 0.;
            twiddles_0 = twiddles_0.offset(nodespacing as isize);
            x0r = *ptr_y;
            x0i = *ptr_y.offset(1 as core::ffi::c_int as isize);
            ptr_y = ptr_y.offset(((del as size_t) << 1 as core::ffi::c_int) as isize);
            x1r = *ptr_y;
            x1i = *ptr_y.offset(1 as core::ffi::c_int as isize);
            tmp_4 = x1r * W4_1 - x1i * W1_1;
            x1i = x1r * W1_1 + x1i * W4_1;
            x1r = tmp_4;
            *ptr_y = x0r - x1r;
            *ptr_y.offset(1 as core::ffi::c_int as isize) = x0i - x1i;
            ptr_y = ptr_y.offset(-(((del as size_t) << 1 as core::ffi::c_int) as isize));
            *ptr_y = x0r + x1r;
            *ptr_y.offset(1 as core::ffi::c_int as isize) = x0i + x1i;
            ptr_y = ptr_y.offset(2 as core::ffi::c_int as isize);
            j -= 1;
        }
    }
}
#[inline]
unsafe extern "C" fn ixheaac_aac_ld_dec_fft_3_float(
    mut inp: *mut FLOAT32,
    mut op: *mut FLOAT32,
) {
    let mut add_r: FLOAT32 = 0.;
    let mut sub_r: FLOAT32 = 0.;
    let mut add_i: FLOAT32 = 0.;
    let mut sub_i: FLOAT32 = 0.;
    let mut temp_real: FLOAT32 = 0.;
    let mut temp_imag: FLOAT32 = 0.;
    let mut temp: FLOAT32 = 0.;
    let mut p1: FLOAT32 = 0.;
    let mut p2: FLOAT32 = 0.;
    let mut p3: FLOAT32 = 0.;
    let mut p4: FLOAT32 = 0.;
    let mut sinmu: FLOAT32 = 0.;
    sinmu = -0.866025403784439f32 as FLOAT32;
    temp_real = *inp.offset(0 as core::ffi::c_int as isize)
        + *inp.offset(2 as core::ffi::c_int as isize);
    temp_imag = *inp.offset(1 as core::ffi::c_int as isize)
        + *inp.offset(3 as core::ffi::c_int as isize);
    add_r = *inp.offset(2 as core::ffi::c_int as isize)
        + *inp.offset(4 as core::ffi::c_int as isize);
    add_i = *inp.offset(3 as core::ffi::c_int as isize)
        + *inp.offset(5 as core::ffi::c_int as isize);
    sub_r = *inp.offset(2 as core::ffi::c_int as isize)
        - *inp.offset(4 as core::ffi::c_int as isize);
    sub_i = *inp.offset(3 as core::ffi::c_int as isize)
        - *inp.offset(5 as core::ffi::c_int as isize);
    p1 = (add_r as core::ffi::c_float / 2.0f32) as FLOAT32;
    p4 = (add_i as core::ffi::c_float / 2.0f32) as FLOAT32;
    p2 = sub_i * sinmu;
    p3 = sub_r * sinmu;
    temp = *inp.offset(0 as core::ffi::c_int as isize) - p1;
    *op.offset(0 as core::ffi::c_int as isize) = temp_real
        + *inp.offset(4 as core::ffi::c_int as isize);
    *op.offset(1 as core::ffi::c_int as isize) = temp_imag
        + *inp.offset(5 as core::ffi::c_int as isize);
    *op.offset(2 as core::ffi::c_int as isize) = temp + p2;
    *op.offset(3 as core::ffi::c_int as isize) = *inp
        .offset(1 as core::ffi::c_int as isize) - p3 - p4;
    *op.offset(4 as core::ffi::c_int as isize) = temp - p2;
    *op.offset(5 as core::ffi::c_int as isize) = *inp
        .offset(1 as core::ffi::c_int as isize) + p3 - p4;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaac_real_synth_fft_p3(
    mut x_in: *mut FLOAT32,
    mut x_out: *mut FLOAT32,
    mut npoints: WORD32,
) {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut x_3: [FLOAT32; 8] = [0.; 8];
    let mut y_3: [FLOAT32; 16] = [0.; 16];
    let mut y: [FLOAT32; 48] = [0.; 48];
    let mut x: [FLOAT32; 48] = [0.; 48];
    let mut ptr_y: *mut FLOAT32 = y.as_mut_ptr();
    let mut y_p3: *mut FLOAT32 = y.as_mut_ptr();
    let mut x_p3: *mut FLOAT32 = x.as_mut_ptr();
    i = 0 as core::ffi::c_int as WORD32;
    while i < 3 as core::ffi::c_int {
        j = 0 as core::ffi::c_int as WORD32;
        while j < npoints as core::ffi::c_int / 3 as core::ffi::c_int {
            x_3[j as usize] = *x_in.offset((3 as WORD32 * j + i) as isize);
            j += 1;
        }
        ixheaac_real_synth_fft_p2(x_3.as_mut_ptr(), y_3.as_mut_ptr(), 8 as WORD32);
        j = 0 as core::ffi::c_int as WORD32;
        while j < 16 as core::ffi::c_int {
            x[(3 as WORD32 * j + 2 as WORD32 * i) as usize] = y_3[j as usize];
            x[(3 as core::ffi::c_int * j as core::ffi::c_int
                + 2 as core::ffi::c_int * i as core::ffi::c_int + 1 as core::ffi::c_int)
                as usize] = y_3[(j as core::ffi::c_int + 1 as core::ffi::c_int)
                as usize];
            j += 2 as core::ffi::c_int;
        }
        i += 1 as core::ffi::c_int;
    }
    let mut wr: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut tmp: FLOAT32 = 0.;
    let mut x_tw: *mut FLOAT32 = x.as_mut_ptr();
    wr = ixheaac_twidle_tbl_24.as_ptr() as *mut FLOAT32;
    x_tw = x_tw.offset(2 as core::ffi::c_int as isize);
    i = 0 as core::ffi::c_int as WORD32;
    while i < npoints as core::ffi::c_int / 3 as core::ffi::c_int {
        tmp = *x_tw * *wr
            + *x_tw.offset(1 as core::ffi::c_int as isize)
                * *wr.offset(1 as core::ffi::c_int as isize);
        *x_tw.offset(1 as core::ffi::c_int as isize) = -*x_tw
            * *wr.offset(1 as core::ffi::c_int as isize)
            + *x_tw.offset(1 as core::ffi::c_int as isize) * *wr;
        *x_tw = tmp;
        wr = wr.offset(2 as core::ffi::c_int as isize);
        x_tw = x_tw.offset(2 as core::ffi::c_int as isize);
        tmp = *x_tw * *wr
            + *x_tw.offset(1 as core::ffi::c_int as isize)
                * *wr.offset(1 as core::ffi::c_int as isize);
        *x_tw.offset(1 as core::ffi::c_int as isize) = -*x_tw
            * *wr.offset(1 as core::ffi::c_int as isize)
            + *x_tw.offset(1 as core::ffi::c_int as isize) * *wr;
        *x_tw = tmp;
        wr = wr.offset(2 as core::ffi::c_int as isize);
        x_tw = x_tw.offset(4 as core::ffi::c_int as isize);
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < npoints as core::ffi::c_int / 3 as core::ffi::c_int {
        ixheaac_aac_ld_dec_fft_3_float(x_p3, y_p3);
        x_p3 = x_p3.offset(6 as core::ffi::c_int as isize);
        y_p3 = y_p3.offset(6 as core::ffi::c_int as isize);
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < 16 as core::ffi::c_int {
        let fresh16 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *x_out.offset(i as isize) = *fresh16;
        let fresh17 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *x_out.offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = *fresh17;
        let fresh18 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *x_out.offset((16 as WORD32 + i) as isize) = *fresh18;
        let fresh19 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *x_out
            .offset(
                (16 as core::ffi::c_int + i as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            ) = *fresh19;
        let fresh20 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *x_out.offset((32 as WORD32 + i) as isize) = *fresh20;
        let fresh21 = ptr_y;
        ptr_y = ptr_y.offset(1);
        *x_out
            .offset(
                (32 as core::ffi::c_int + i as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            ) = *fresh21;
        i += 2 as core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaac_cmplx_anal_fft_p3(
    mut x_in: *mut FLOAT32,
    mut x_out: *mut FLOAT32,
    mut npoints: WORD32,
) {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut x_3: [FLOAT32; 32] = [0.; 32];
    let mut y_3: [FLOAT32; 32] = [0.; 32];
    let mut y: [FLOAT32; 96] = [0.; 96];
    let mut ptr_x: *mut FLOAT32 = x_in;
    let mut ptr_y: *mut FLOAT32 = y.as_mut_ptr();
    let mut y_p3: *mut FLOAT32 = y.as_mut_ptr();
    i = 0 as core::ffi::c_int as WORD32;
    while i < 6 as core::ffi::c_int {
        j = 0 as core::ffi::c_int as WORD32;
        while j < 32 as core::ffi::c_int {
            x_3[j as usize] = *x_in.offset((3 as WORD32 * j + i) as isize);
            x_3[(j as core::ffi::c_int + 1 as core::ffi::c_int) as usize] = *x_in
                .offset(
                    (3 as core::ffi::c_int * j as core::ffi::c_int
                        + i as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                );
            j += 2 as core::ffi::c_int;
        }
        ixheaac_cmplx_anal_fft_p2(x_3.as_mut_ptr(), y_3.as_mut_ptr(), 16 as WORD32);
        j = 0 as core::ffi::c_int as WORD32;
        while j < 32 as core::ffi::c_int {
            *x_in.offset((3 as WORD32 * j + i) as isize) = y_3[j as usize];
            *x_in
                .offset(
                    (3 as core::ffi::c_int * j as core::ffi::c_int
                        + i as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                ) = y_3[(j as core::ffi::c_int + 1 as core::ffi::c_int) as usize];
            j += 2 as core::ffi::c_int;
        }
        i += 2 as core::ffi::c_int;
    }
    let mut wr: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut tmp: FLOAT32 = 0.;
    wr = ixheaac_twidle_tbl_48.as_ptr() as *mut FLOAT32;
    x_in = x_in.offset(2 as core::ffi::c_int as isize);
    i = 0 as core::ffi::c_int as WORD32;
    while i < npoints as core::ffi::c_int / 3 as core::ffi::c_int {
        tmp = *x_in * *wr
            + *x_in.offset(1 as core::ffi::c_int as isize)
                * *wr.offset(1 as core::ffi::c_int as isize);
        *x_in.offset(1 as core::ffi::c_int as isize) = -*x_in
            * *wr.offset(1 as core::ffi::c_int as isize)
            + *x_in.offset(1 as core::ffi::c_int as isize) * *wr;
        *x_in = tmp;
        wr = wr.offset(2 as core::ffi::c_int as isize);
        x_in = x_in.offset(2 as core::ffi::c_int as isize);
        tmp = *x_in * *wr
            + *x_in.offset(1 as core::ffi::c_int as isize)
                * *wr.offset(1 as core::ffi::c_int as isize);
        *x_in.offset(1 as core::ffi::c_int as isize) = -*x_in
            * *wr.offset(1 as core::ffi::c_int as isize)
            + *x_in.offset(1 as core::ffi::c_int as isize) * *wr;
        *x_in = tmp;
        wr = wr.offset(2 as core::ffi::c_int as isize);
        x_in = x_in.offset(4 as core::ffi::c_int as isize);
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < npoints as core::ffi::c_int / 3 as core::ffi::c_int {
        ixheaac_aac_ld_dec_fft_3_float(ptr_x, ptr_y);
        ptr_x = ptr_x.offset(6 as core::ffi::c_int as isize);
        ptr_y = ptr_y.offset(6 as core::ffi::c_int as isize);
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < 32 as core::ffi::c_int {
        let fresh22 = y_p3;
        y_p3 = y_p3.offset(1);
        *x_out.offset(i as isize) = *fresh22;
        let fresh23 = y_p3;
        y_p3 = y_p3.offset(1);
        *x_out.offset((i as core::ffi::c_int + 1 as core::ffi::c_int) as isize) = *fresh23;
        let fresh24 = y_p3;
        y_p3 = y_p3.offset(1);
        *x_out.offset((32 as WORD32 + i) as isize) = *fresh24;
        let fresh25 = y_p3;
        y_p3 = y_p3.offset(1);
        *x_out
            .offset(
                (32 as core::ffi::c_int + i as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            ) = *fresh25;
        let fresh26 = y_p3;
        y_p3 = y_p3.offset(1);
        *x_out.offset((64 as WORD32 + i) as isize) = *fresh26;
        let fresh27 = y_p3;
        y_p3 = y_p3.offset(1);
        *x_out
            .offset(
                (64 as core::ffi::c_int + i as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            ) = *fresh27;
        i += 2 as core::ffi::c_int;
    }
}
