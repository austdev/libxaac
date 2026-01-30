pub type WORD16 = core::ffi::c_short;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type WORD64 = core::ffi::c_longlong;
pub type VOID = ();
pub type WORD = core::ffi::c_int;
pub const MAX_32: WORD32 = 0x7fffffff as core::ffi::c_long as WORD32;
pub const MIN_32: WORD32 = 0x80000000 as core::ffi::c_long as WORD32;
#[inline]
unsafe extern "C" fn ixheaac_min32(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut min_val: WORD32 = 0;
    min_val = if a < b { a } else { b };
    return min_val;
}
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
unsafe extern "C" fn ixheaac_mult16x16in32_shl_sat(
    mut a: WORD16,
    mut b: WORD16,
) -> WORD32 {
    let mut product: WORD32 = 0;
    product = a as WORD32 * b as WORD32;
    if product != 0x40000000 as core::ffi::c_long as WORD32 {
        product = ixheaac_shl32(product, 1 as WORD);
    } else {
        product = MAX_32;
    }
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
unsafe extern "C" fn ixheaac_abs32_nrm(mut a: WORD32) -> WORD32 {
    let mut abs_val: WORD32 = 0;
    abs_val = a;
    if a < 0 as core::ffi::c_int {
        abs_val = !a;
    }
    return abs_val;
}
#[inline]
unsafe extern "C" fn ixheaac_abs32_sat(mut a: WORD32) -> WORD32 {
    let mut abs_val: WORD32 = 0;
    abs_val = a;
    if a == MIN_32 {
        abs_val = MAX_32;
    } else if a < 0 as core::ffi::c_int {
        abs_val = -a;
    }
    return abs_val;
}
#[inline]
unsafe extern "C" fn ixheaac_mac16x16in32_shl_sat(
    mut a: WORD32,
    mut b: WORD16,
    mut c: WORD16,
) -> WORD32 {
    let mut acc: WORD32 = 0;
    acc = ixheaac_mult16x16in32_shl_sat(b, c);
    acc = ixheaac_add32_sat(a, acc);
    return acc;
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
#[inline]
unsafe extern "C" fn ixheaac_deposit16h_in32(mut var: WORD16) -> WORD32 {
    let mut var_out: WORD32 = 0;
    var_out = (var as WORD32) << 16 as core::ffi::c_int;
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_shl32_dir_sat_limit(mut a: WORD32, mut b: WORD) -> WORD32 {
    let mut out_val: WORD32 = 0;
    if b < 0 as core::ffi::c_int {
        b = -b;
        b = ixheaac_min32(b as WORD32, 31 as WORD32) as WORD;
        out_val = ixheaac_shr32(a, b);
    } else {
        out_val = ixheaac_shl32_sat(a, b);
    }
    return out_val;
}
pub const MAX_ORDER: core::ffi::c_int = 31 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_res_tns_parcor_2_lpc_32x16(
    mut parcor: *mut WORD16,
    mut lpc: *mut WORD16,
    mut scale: *mut WORD16,
    mut order: WORD,
) -> VOID {
    let mut i: WORD = 0;
    let mut j: WORD = 0;
    let mut status: WORD = 0;
    let mut z1: WORD32 = 0;
    let mut z: [WORD16; 32] = [0; 32];
    let mut w: [WORD16; 32] = [0; 32];
    let mut accu1: WORD32 = 0;
    let mut accu2: WORD32 = 0;
    status = 1 as core::ffi::c_int as WORD;
    *scale = 0 as WORD16;
    while status != 0 {
        status = 0 as core::ffi::c_int as WORD;
        i = MAX_ORDER as WORD;
        while i >= 0 as core::ffi::c_int {
            z[i as usize] = 0 as WORD16;
            w[i as usize] = 0 as WORD16;
            i -= 1;
        }
        accu1 = (0x7fffffff as core::ffi::c_int >> *scale as core::ffi::c_int) as WORD32;
        i = 0 as core::ffi::c_int as WORD;
        while i <= order {
            z1 = accu1;
            j = 0 as core::ffi::c_int as WORD;
            while j < order {
                w[j as usize] = ixheaac_round16(accu1);
                accu1 = ixheaac_mac16x16in32_shl_sat(
                    accu1,
                    *parcor.offset(j as isize),
                    z[j as usize],
                );
                if ixheaac_abs32_sat(accu1) == 0x7fffffff as core::ffi::c_int {
                    status = 1 as core::ffi::c_int as WORD;
                }
                j += 1;
            }
            j = (order as core::ffi::c_int - 1 as core::ffi::c_int) as WORD;
            while j >= 0 as core::ffi::c_int {
                accu2 = ixheaac_deposit16h_in32(z[j as usize]);
                accu2 = ixheaac_mac16x16in32_shl_sat(
                    accu2,
                    *parcor.offset(j as isize),
                    w[j as usize],
                );
                z[(j as core::ffi::c_int + 1 as core::ffi::c_int) as usize] = ixheaac_round16(
                    accu2,
                );
                if ixheaac_abs32_sat(accu2) == 0x7fffffff as core::ffi::c_int {
                    status = 1 as core::ffi::c_int as WORD;
                }
                j -= 1;
            }
            z[0 as core::ffi::c_int as usize] = ixheaac_round16(z1);
            *lpc.offset(i as isize) = ixheaac_round16(accu1);
            accu1 = 0 as core::ffi::c_int as WORD32;
            i += 1;
        }
        accu1 = (status as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        if accu1 == 0 as core::ffi::c_int {
            *scale = (*scale as core::ffi::c_int + 1 as core::ffi::c_int) as WORD16;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_res_tns_ar_filter_fixed_32x16(
    mut spectrum: *mut WORD32,
    mut size: WORD32,
    mut inc: WORD32,
    mut lpc: *mut WORD16,
    mut order: WORD32,
    mut shift_value: WORD32,
    mut scale_spec: WORD,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut y: WORD32 = 0;
    let mut state: [WORD32; 32] = [0; 32];
    if order as core::ffi::c_int & 3 as core::ffi::c_int != 0 as core::ffi::c_int {
        i = (order as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
        while i
            < (order as core::ffi::c_uint & 0xfffffffc as core::ffi::c_uint)
                as core::ffi::c_int + 4 as core::ffi::c_int
        {
            *lpc.offset(i as isize) = 0 as WORD16;
            i += 1;
        }
        *lpc.offset(i as isize) = 0 as WORD16;
        order = (order as core::ffi::c_uint & 0xfffffffc as core::ffi::c_uint)
            .wrapping_add(4 as core::ffi::c_uint) as WORD32;
        order = (order as core::ffi::c_int & 31 as core::ffi::c_int) as WORD32;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < order {
        y = *spectrum << scale_spec;
        j = i;
        while j > 0 as core::ffi::c_int {
            y = ixheaac_sub32_sat(
                y,
                ixheaac_mult32x16in32_shl_sat(
                    state[(j as core::ffi::c_int - 1 as core::ffi::c_int) as usize],
                    *lpc.offset(j as isize),
                ),
            );
            state[j as usize] = state[(j as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize];
            j -= 1;
        }
        state[0 as core::ffi::c_int as usize] = ixheaac_shl32_dir_sat_limit(
            y,
            shift_value as WORD,
        );
        *spectrum = y >> scale_spec;
        spectrum = spectrum.offset(inc as isize);
        i += 1;
    }
    i = order;
    while i < size {
        y = *spectrum << scale_spec;
        j = order;
        while j > 0 as core::ffi::c_int {
            y = ixheaac_sub32_sat(
                y,
                ixheaac_mult32x16in32_shl_sat(
                    state[(j as core::ffi::c_int - 1 as core::ffi::c_int) as usize],
                    *lpc.offset(j as isize),
                ),
            );
            state[j as usize] = state[(j as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize];
            j -= 1;
        }
        state[0 as core::ffi::c_int as usize] = ixheaac_shl32_dir_sat_limit(
            y,
            shift_value as WORD,
        );
        *spectrum = y >> scale_spec;
        spectrum = spectrum.offset(inc as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_res_calc_max_spectral_line(
    mut p_tmp: *mut WORD32,
    mut size: WORD32,
) -> WORD32 {
    let mut max_spectral_line: WORD32 = 0 as WORD32;
    let mut i: WORD32 = 0;
    let mut count: WORD = 0;
    let mut remaining: WORD = 0;
    let mut temp_1: WORD = 0;
    let mut temp_2: WORD = 0;
    let mut temp3: WORD = 0;
    let mut temp4: WORD = 0;
    count = (size >> 3 as core::ffi::c_int) as WORD;
    i = count as WORD32;
    loop {
        let fresh0 = i;
        i = i - 1;
        if !(fresh0 != 0) {
            break;
        }
        let fresh1 = p_tmp;
        p_tmp = p_tmp.offset(1);
        temp_1 = *fresh1 as WORD;
        let fresh2 = p_tmp;
        p_tmp = p_tmp.offset(1);
        temp_2 = *fresh2 as WORD;
        let fresh3 = p_tmp;
        p_tmp = p_tmp.offset(1);
        temp3 = *fresh3 as WORD;
        let fresh4 = p_tmp;
        p_tmp = p_tmp.offset(1);
        temp4 = *fresh4 as WORD;
        max_spectral_line = ixheaac_abs32_nrm(temp_1 as WORD32) | max_spectral_line;
        max_spectral_line = ixheaac_abs32_nrm(temp_2 as WORD32) | max_spectral_line;
        max_spectral_line = ixheaac_abs32_nrm(temp3 as WORD32) | max_spectral_line;
        max_spectral_line = ixheaac_abs32_nrm(temp4 as WORD32) | max_spectral_line;
        let fresh5 = p_tmp;
        p_tmp = p_tmp.offset(1);
        temp_1 = *fresh5 as WORD;
        let fresh6 = p_tmp;
        p_tmp = p_tmp.offset(1);
        temp_2 = *fresh6 as WORD;
        let fresh7 = p_tmp;
        p_tmp = p_tmp.offset(1);
        temp3 = *fresh7 as WORD;
        let fresh8 = p_tmp;
        p_tmp = p_tmp.offset(1);
        temp4 = *fresh8 as WORD;
        max_spectral_line = ixheaac_abs32_nrm(temp_1 as WORD32) | max_spectral_line;
        max_spectral_line = ixheaac_abs32_nrm(temp_2 as WORD32) | max_spectral_line;
        max_spectral_line = ixheaac_abs32_nrm(temp3 as WORD32) | max_spectral_line;
        max_spectral_line = ixheaac_abs32_nrm(temp4 as WORD32) | max_spectral_line;
    }
    remaining = size as WORD - (count << 3 as core::ffi::c_int);
    if remaining != 0 {
        i = remaining as WORD32;
        loop {
            let fresh9 = i;
            i = i - 1;
            if !(fresh9 != 0) {
                break;
            }
            max_spectral_line = ixheaac_abs32_nrm(*p_tmp) | max_spectral_line;
            p_tmp = p_tmp.offset(1);
        }
    }
    return ixheaac_norm32(max_spectral_line) as WORD32;
}
