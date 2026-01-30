pub type WORD16 = core::ffi::c_short;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type WORD64 = core::ffi::c_longlong;
pub type VOID = ();
pub type WORD = core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ixheaacd_misc_tables {
    pub trig_data: [WORD16; 513],
    pub sine_table8_16: [WORD16; 8],
    pub log_dual_is_table: [WORD16; 65],
    pub down_mix_martix: [[[WORD32; 8]; 2]; 4],
    pub cc_gain_scale: [WORD32; 4],
    pub inv_table: [WORD16; 256],
    pub sqrt_table: [WORD16; 257],
    pub dummy: WORD32,
    pub start_band: [[WORD32; 16]; 10],
    pub stop_band: [[WORD32; 16]; 10],
    pub stop_freq_table_fs40k_2: [WORD32; 14],
    pub stop_freq_table_fs40k_4: [WORD32; 14],
}
pub const MAX_32: WORD32 = 0x7fffffff as core::ffi::c_long as WORD32;
pub const MIN_32: WORD32 = 0x80000000 as core::ffi::c_long as WORD32;
pub const MAX_16: WORD16 = 0x7fff as core::ffi::c_int as WORD16;
#[inline]
unsafe extern "C" fn ixheaac_min32(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut min_val: WORD32 = 0;
    min_val = if a < b { a } else { b };
    return min_val;
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
unsafe extern "C" fn ixheaac_mult16x16in32(mut a: WORD16, mut b: WORD16) -> WORD32 {
    let mut product: WORD32 = 0;
    product = a as WORD32 * b as WORD32;
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
unsafe extern "C" fn ixheaac_abs32(mut a: WORD32) -> WORD32 {
    let mut abs_val: WORD32 = 0;
    abs_val = a;
    if a < 0 as core::ffi::c_int {
        abs_val = -a;
    }
    return abs_val;
}
#[inline]
unsafe extern "C" fn ixheaac_sat16(mut op1: WORD32) -> WORD16 {
    let mut var_out: WORD16 = 0;
    if op1 as core::ffi::c_long > 0x7fff as core::ffi::c_long {
        var_out = MAX_16;
    } else if op1 < 0xffff8000 as core::ffi::c_long as WORD32 {
        var_out = -(32768 as core::ffi::c_int) as WORD16;
    } else {
        var_out = op1 as WORD16;
    }
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
unsafe extern "C" fn ixheaac_mult32_shl(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 32 as core::ffi::c_int) as WORD32;
    return result << 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn ixheaac_mult32_shl_sat(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut result: WORD32 = 0;
    if a == 0x80000000 as core::ffi::c_uint as WORD32
        && b == 0x80000000 as core::ffi::c_uint as WORD32
    {
        result = 0x7fffffff as core::ffi::c_int as WORD32;
    } else {
        result = ixheaac_mult32_shl(a, b);
    }
    return result;
}
#[inline]
unsafe extern "C" fn ixheaac_extract16h(mut var: WORD32) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = (var >> 16 as core::ffi::c_int) as WORD16;
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_mult32x16h_in32_shl_sat(
    mut a: WORD32,
    mut b: WORD32,
) -> WORD32 {
    let mut result: WORD32 = 0;
    if a == 0x80000000 as core::ffi::c_uint as WORD32
        && b == 0x8000 as core::ffi::c_int as WORD16 as core::ffi::c_int
    {
        result = 0x7fffffff as core::ffi::c_int;
    } else {
        result = ixheaac_mult32x16in32_shl(a, ixheaac_extract16h(b));
    }
    return result;
}
#[inline]
unsafe extern "C" fn ixheaac_shr32_dir_sat_limit(mut a: WORD32, mut b: WORD) -> WORD32 {
    let mut out_val: WORD32 = 0;
    if b < 0 as core::ffi::c_int {
        out_val = ixheaac_shl32_sat(a, -b);
    } else {
        b = ixheaac_min32(b as WORD32, 31 as WORD32) as WORD;
        out_val = ixheaac_shr32(a, b);
    }
    return out_val;
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
pub const SHORT_BITS: core::ffi::c_int = 16 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_fix_mant_exp_add(
    mut op1_mant: WORD16,
    mut op1_exp: WORD16,
    mut op2_mant: WORD16,
    mut op2_exp: WORD16,
    mut ptr_result_mant: *mut WORD16,
    mut ptr_result_exp: *mut WORD16,
) -> VOID {
    let mut new_mant: WORD32 = 0;
    let mut new_exp: WORD32 = 0;
    new_exp = (op1_exp as core::ffi::c_int - op2_exp as core::ffi::c_int) as WORD32;
    if new_exp < 0 as core::ffi::c_int {
        if new_exp < -(31 as core::ffi::c_int) {
            new_exp = -(31 as core::ffi::c_int) as WORD32;
        }
        op1_mant = (op1_mant as core::ffi::c_int >> -new_exp) as WORD16;
        new_exp = op2_exp as WORD32;
    } else {
        if new_exp > 31 as core::ffi::c_int {
            new_exp = 31 as core::ffi::c_int as WORD32;
        }
        op2_mant = (op2_mant as core::ffi::c_int >> new_exp) as WORD16;
        new_exp = op1_exp as WORD32;
    }
    new_mant = (op1_mant as core::ffi::c_int + op2_mant as core::ffi::c_int) as WORD32;
    if ixheaac_abs32(new_mant) >= 0x8000 as core::ffi::c_int {
        new_mant = new_mant >> 1 as core::ffi::c_int;
        new_exp += 1;
    }
    *ptr_result_mant = new_mant as WORD16;
    *ptr_result_exp = new_exp as WORD16;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_fix_mant_div(
    mut op1_mant: WORD16,
    mut op2_mant: WORD16,
    mut ptr_result_mant: *mut WORD16,
    mut pstr_common_tables: *mut ixheaacd_misc_tables,
) -> WORD32 {
    let mut pre_shift_val: WORD32 = 0;
    let mut post_shift_val: WORD32 = 0;
    let mut index: WORD32 = 0;
    let mut one_by_op2_mant: WORD16 = 0;
    pre_shift_val = (ixheaac_norm32(op2_mant as WORD32) as core::ffi::c_int
        - 16 as core::ffi::c_int) as WORD32;
    index = ((op2_mant as core::ffi::c_int) << pre_shift_val
        >> SHORT_BITS - 3 as core::ffi::c_int - 8 as core::ffi::c_int) as WORD32;
    index
        &= ((1 as core::ffi::c_int) << 8 as core::ffi::c_int + 1 as core::ffi::c_int)
            - 1 as core::ffi::c_int;
    if index == 0 as core::ffi::c_int {
        post_shift_val = (ixheaac_norm32(op1_mant as WORD32) as core::ffi::c_int
            - 16 as core::ffi::c_int) as WORD32;
        *ptr_result_mant = ((op1_mant as core::ffi::c_int) << post_shift_val) as WORD16;
    } else {
        let mut ratio_m: WORD32 = 0;
        index = (index as core::ffi::c_int - 1 as core::ffi::c_int
            >> 1 as core::ffi::c_int) as WORD32;
        one_by_op2_mant = (*pstr_common_tables).inv_table[index as usize];
        ratio_m = ixheaac_mult16x16in32(one_by_op2_mant, op1_mant);
        post_shift_val = (ixheaac_norm32(ratio_m) as core::ffi::c_int
            - 1 as core::ffi::c_int) as WORD32;
        *ptr_result_mant = (ratio_m << post_shift_val >> 15 as core::ffi::c_int)
            as WORD16;
    }
    return pre_shift_val - post_shift_val;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_fix_mant_exp_sqrt(
    mut ptr_in_out: *mut WORD16,
    mut sqrt_table: *mut WORD16,
) -> VOID {
    let mut index: WORD32 = 0;
    let mut pre_shift_val: WORD32 = 0;
    let mut op_mant: WORD32 = *ptr_in_out as WORD32;
    let mut op_exp: WORD32 = *ptr_in_out.offset(1 as core::ffi::c_int as isize)
        as WORD32;
    let mut result_m: WORD32 = 0;
    let mut result_e: WORD32 = 0;
    if op_mant > 0 as core::ffi::c_int {
        pre_shift_val = (ixheaac_norm32(op_mant as WORD16 as WORD32) as core::ffi::c_int
            - 16 as core::ffi::c_int) as WORD32;
        op_exp = op_exp - pre_shift_val;
        index = op_mant << pre_shift_val
            >> SHORT_BITS - 3 as core::ffi::c_int - 8 as core::ffi::c_int;
        index
            &= ((1 as core::ffi::c_int) << 8 as core::ffi::c_int + 1 as core::ffi::c_int)
                - 1 as core::ffi::c_int;
        result_m = *sqrt_table.offset((index >> 1 as core::ffi::c_int) as isize)
            as WORD32;
        if op_exp as core::ffi::c_int & 1 as core::ffi::c_int != 0 as core::ffi::c_int {
            result_m = (result_m as core::ffi::c_int * 0x5a82 as core::ffi::c_int
                >> 16 as core::ffi::c_int) as WORD32;
            op_exp += 3 as core::ffi::c_int;
        }
        result_e = op_exp >> 1 as core::ffi::c_int;
    } else {
        result_m = 0 as core::ffi::c_int as WORD32;
        result_e = -SHORT_BITS as WORD32;
    }
    let fresh0 = ptr_in_out;
    ptr_in_out = ptr_in_out.offset(1);
    *fresh0 = result_m as WORD16;
    *ptr_in_out = result_e as WORD16;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_fix_div_dec(
    mut op1: WORD32,
    mut op2: WORD32,
) -> WORD32 {
    let mut quotient: WORD32 = 0 as WORD32;
    let mut abs_num: UWORD32 = 0;
    let mut abs_den: UWORD32 = 0;
    let mut k: WORD32 = 0;
    let mut sign: WORD32 = 0;
    abs_num = ixheaac_abs32(op1 >> 1 as core::ffi::c_int) as UWORD32;
    abs_den = ixheaac_abs32(op2 >> 1 as core::ffi::c_int) as UWORD32;
    sign = op1 ^ op2;
    if abs_num != 0 as UWORD32 {
        k = 15 as core::ffi::c_int as WORD32;
        while k > 0 as core::ffi::c_int {
            quotient = quotient << 1 as core::ffi::c_int;
            abs_num = abs_num << 1 as core::ffi::c_int;
            if abs_num >= abs_den {
                abs_num = abs_num.wrapping_sub(abs_den);
                quotient += 1;
            }
            k -= 1;
        }
    }
    if sign < 0 as core::ffi::c_int {
        quotient = -quotient;
    }
    return quotient;
}
pub const ONE_IN_Q30: core::ffi::c_int = 0x40000000 as core::ffi::c_int;
#[inline]
unsafe extern "C" fn ixheaacd_one_by_sqrt_calc(mut op: WORD32) -> WORD32 {
    let mut a: WORD32 = ixheaac_add32_sat(
        0x900ebee0u32 as i32 as WORD32,
        ixheaac_mult32x16in32_shl_sat(op, 0x39d9 as WORD16),
    );
    let mut iy: WORD32 = ixheaac_add32_sat(
        0x573b645a as WORD32,
        ixheaac_mult32x16h_in32_shl_sat(op, a),
    );
    iy = ixheaac_shl32_dir_sat_limit(iy, 1 as WORD);
    a = ixheaac_mult32_shl_sat(op, iy);
    a = ixheaac_sub32_sat(
        ONE_IN_Q30,
        ixheaac_shl32_dir_sat_limit(ixheaac_mult32_shl_sat(a, iy), 1 as WORD),
    );
    iy = ixheaac_add32_sat(iy, ixheaac_mult32_shl_sat(a, iy));
    a = ixheaac_mult32_shl_sat(op, iy);
    a = ixheaac_sub32_sat(
        ONE_IN_Q30,
        ixheaac_shl32_dir_sat_limit(ixheaac_mult32_shl_sat(a, iy), 1 as WORD),
    );
    iy = ixheaac_add32_sat(iy, ixheaac_mult32_shl_sat(a, iy));
    a = ixheaac_mult32_shl_sat(op, iy);
    a = ixheaac_sub32_sat(
        ONE_IN_Q30,
        ixheaac_shl32_dir_sat_limit(ixheaac_mult32_shl_sat(a, iy), 1 as WORD),
    );
    iy = ixheaac_add32_sat(iy, ixheaac_mult32_shl_sat(a, iy));
    return iy;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_sqrt(mut op: WORD32) -> WORD32 {
    let mut result: WORD32 = 0 as WORD32;
    let mut shift: WORD16 = 0;
    if op != 0 as core::ffi::c_int {
        shift = (ixheaac_norm32(op) as core::ffi::c_int & !(1 as core::ffi::c_int))
            as WORD16;
        op = ixheaac_shl32_dir_sat_limit(op, shift as WORD);
        shift = ixheaac_shr32_dir_sat_limit(shift as WORD32, 1 as WORD) as WORD16;
        op = ixheaac_mult32_shl_sat(ixheaacd_one_by_sqrt_calc(op), op);
        result = ixheaac_shr32_dir_sat_limit(
            op,
            ixheaac_sat16(shift as WORD32 - 1 as WORD32) as WORD,
        );
    }
    return result;
}
