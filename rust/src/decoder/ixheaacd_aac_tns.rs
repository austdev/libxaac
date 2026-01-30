pub type WORD8 = core::ffi::c_schar;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD16 = core::ffi::c_short;
pub type UWORD16 = core::ffi::c_ushort;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type WORD64 = core::ffi::c_longlong;
pub type VOID = ();
pub type WORD = core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sampling_rate_info_struct {
    pub sampling_frequency: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_block_tables_struct {
    pub ixheaacd_pow_table_Q13: [WORD32; 129],
    pub scale_table: [WORD32; 4],
    pub tns_max_bands_tbl: [[WORD8; 2]; 12],
    pub tns_coeff3_16: [WORD16; 8],
    pub tns_coeff4_16: [WORD16; 16],
    pub scale_mant_tab: [WORD32; 4],
    pub tns_coeff3: [WORD32; 8],
    pub tns_coeff4: [WORD32; 16],
    pub tns_coeff3_32: [WORD32; 8],
    pub tns_coeff4_32: [WORD32; 16],
    pub tns_max_bands_tbl_usac: [[WORD32; 2]; 16],
    pub tns_max_bands_tbl_ld: [WORD8; 12],
    pub tns_max_bands_tbl_480: [WORD8; 12],
    pub scale_table_960: [WORD32; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_huffman_tables_struct {
    pub ixheaacd_sfb_96_1024: [WORD8; 43],
    pub ixheaacd_sfb_96_128: [WORD8; 14],
    pub ixheaacd_sfb_64_1024: [WORD8; 49],
    pub ixheaacd_sfb_48_1024: [WORD8; 51],
    pub ixheaacd_sfb_48_128: [WORD8; 16],
    pub ixheaacd_sfb_32_1024: [WORD8; 53],
    pub ixheaacd_sfb_24_1024: [WORD8; 49],
    pub ixheaacd_sfb_24_128: [WORD8; 17],
    pub ixheaacd_sfb_16_1024: [WORD8; 45],
    pub ixheaacd_sfb_16_128: [WORD8; 17],
    pub ixheaacd_sfb_8_1024: [WORD8; 42],
    pub ixheaacd_sfb_8_128: [WORD8; 17],
    pub str_sample_rate_info: [ia_sampling_rate_info_struct; 13],
    pub idx_table_hf11: [UWORD32; 21],
    pub idx_table_hf10: [UWORD32; 20],
    pub idx_table_hf9: [UWORD32; 23],
    pub idx_table_hf8: [UWORD32; 17],
    pub idx_table_hf7: [UWORD32; 18],
    pub idx_table_hf6: [UWORD32; 17],
    pub idx_table_hf5: [UWORD32; 19],
    pub idx_table_hf4: [UWORD32; 19],
    pub idx_table_hf3: [UWORD32; 27],
    pub idx_table_hf2: [UWORD32; 16],
    pub idx_table_hf1: [UWORD32; 12],
    pub input_table_cb11: [UWORD16; 290],
    pub input_table_cb10: [UWORD16; 170],
    pub input_table_cb9: [UWORD16; 170],
    pub input_table_cb8: [UWORD16; 65],
    pub input_table_cb7: [UWORD16; 65],
    pub input_table_cb6: [UWORD16; 82],
    pub input_table_cb5: [UWORD16; 82],
    pub input_table_cb4: [UWORD16; 82],
    pub input_table_cb3: [UWORD16; 82],
    pub input_table_cb2: [UWORD16; 82],
    pub input_table_cb1: [UWORD16; 82],
    pub huffman_code_book_scl: [UWORD16; 122],
    pub huffman_code_book_scl_index: [UWORD32; 33],
    pub ixheaacd_sfb_48_512: [WORD8; 37],
    pub ixheaacd_sfb_32_512: [WORD8; 38],
    pub ixheaacd_sfb_24_512: [WORD8; 32],
    pub ixheaacd_sfb_48_480: [WORD8; 36],
    pub ixheaacd_sfb_32_480: [WORD8; 38],
    pub ixheaacd_sfb_24_480: [WORD8; 31],
    pub ixheaacd_sfb_96_960: [WORD8; 41],
    pub ixheaacd_sfb_96_120: [WORD8; 13],
    pub ixheaacd_sfb_64_960: [WORD8; 47],
    pub ixheaacd_sfb_48_960: [WORD8; 50],
    pub ixheaacd_sfb_48_120: [WORD8; 15],
    pub ixheaacd_sfb_24_960: [WORD8; 47],
    pub ixheaacd_sfb_24_120: [WORD8; 16],
    pub ixheaacd_sfb_16_960: [WORD8; 43],
    pub ixheaacd_sfb_16_120: [WORD8; 16],
    pub ixheaacd_sfb_8_960: [WORD8; 41],
    pub ixheaacd_sfb_8_120: [WORD8; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_imdct_tables_struct {
    pub cosine_array_2048_256: [WORD16; 514],
    pub dig_rev_table8_long: [WORD8; 64],
    pub dig_rev_table8_short: [WORD8; 8],
    pub fft_twiddle: [WORD32; 448],
    pub only_long_window_sine: [WORD16; 1024],
    pub only_long_window_kbd: [WORD16; 1024],
    pub only_short_window_sine: [WORD16; 128],
    pub only_short_window_kbd: [WORD16; 128],
    pub cosine_array_2048_256p: [WORD16; 514],
    pub w1024: [WORD32; 768],
    pub bit_rev_1024: [UWORD8; 256],
    pub bit_rev_512: [UWORD8; 64],
    pub bit_rev_128: [UWORD8; 16],
    pub bit_rev_32: [UWORD8; 4],
    pub w_256: [WORD32; 504],
    pub low_overlap_win: [WORD32; 512],
    pub window_sine_512: [WORD32; 512],
    pub cosine_array_1024: [WORD32; 512],
    pub low_overlap_win_480: [WORD32; 480],
    pub window_sine_480: [WORD32; 480],
    pub re_arr_tab_16: [UWORD8; 240],
    pub re_arr_tab_sml_240: [UWORD8; 240],
    pub cosine_array_960: [WORD32; 480],
    pub w_16: [WORD32; 24],
    pub window_sine_480_eld: [WORD16; 1920],
    pub window_sine_512_eld: [WORD16; 2048],
    pub only_long_window_sine_960: [WORD16; 960],
    pub only_long_window_kbd_960: [WORD16; 960],
    pub only_short_window_sine_120: [WORD16; 120],
    pub only_short_window_kbd_120: [WORD16; 120],
    pub re_arr_tab_32: [WORD16; 480],
    pub re_arr_tab_sml: [WORD16; 16],
    pub re_arr_tab_4: [WORD16; 60],
    pub re_arr_tab_15_4: [WORD16; 60],
    pub re_arr_tab_120: [WORD16; 60],
    pub re_arr_tab_5: [WORD16; 16],
    pub re_arr_tab_3: [WORD16; 16],
    pub re_arr_tab_sml_480: [WORD16; 480],
    pub cosine_array_1920: [WORD32; 960],
    pub w_512: [WORD16; 1020],
    pub w_32: [WORD16; 60],
    pub cosine_array_240: [WORD16; 120],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_sfb_info {
    pub sfb_index: *mut WORD16,
    pub sfb_width: *mut WORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_tables_struct {
    pub pstr_block_tables: *mut ia_aac_dec_block_tables_struct,
    pub pstr_huffmann_tables: *mut ia_aac_dec_huffman_tables_struct,
    pub pstr_imdct_tables: *mut ia_aac_dec_imdct_tables_struct,
    pub str_aac_sfb_info: [ia_aac_sfb_info; 4],
    pub scale_factor_bands_long: [*mut WORD8; 24],
    pub scale_factor_bands_short: [*mut WORD8; 24],
    pub sfb_long_table: [WORD16; 52],
    pub sfb_short_table: [WORD16; 16],
    pub code_book: [*mut UWORD16; 13],
    pub index_table: [*mut UWORD32; 13],
    pub scale_fac_bands_512: [*mut WORD8; 16],
    pub scale_fac_bands_480: [*mut WORD8; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_filter_info_struct {
    pub start_band: WORD16,
    pub stop_band: WORD16,
    pub direction: WORD8,
    pub resolution: WORD8,
    pub order: WORD8,
    pub coef: [WORD8; 31],
}
pub const MAX_64: WORD64 = 0x7fffffffffffffff as core::ffi::c_long as WORD64;
pub const MIN_64: WORD64 = 0x8000000000000000 as core::ffi::c_ulong as WORD64;
pub const MAX_32: WORD32 = 0x7fffffff as core::ffi::c_long as WORD32;
pub const MIN_32: WORD32 = 0x80000000 as core::ffi::c_long as WORD32;
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
unsafe extern "C" fn ixheaac_shl32_dir_sat(mut a: WORD32, mut b: WORD) -> WORD32 {
    let mut out_val: WORD32 = 0;
    if b < 0 as core::ffi::c_int {
        out_val = ixheaac_shr32(a, -b);
    } else {
        out_val = ixheaac_shl32_sat(a, b);
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
unsafe extern "C" fn ixheaac_mult32x16in32(mut a: WORD32, mut b: WORD16) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 16 as core::ffi::c_int) as WORD32;
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
unsafe extern "C" fn ixheaac_add64_sat(mut a: WORD64, mut b: WORD64) -> WORD64 {
    let mut result: WORD64 = 0;
    let mut comp: WORD64 = 0;
    result = if a < 0 as WORD64 { MIN_64 } else { MAX_64 };
    comp = result - a;
    if (a < 0 as WORD64) as core::ffi::c_int == (b > comp) as core::ffi::c_int {
        result = a + b;
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
unsafe extern "C" fn ixheaac_mac32x32in64_dual(
    mut a: WORD32,
    mut b: WORD32,
    mut c: WORD64,
) -> WORD64 {
    let mut result: WORD64 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = ixheaac_add64_sat(c, temp_result);
    return result;
}
pub const MAX_ORDER: core::ffi::c_int = 31 as core::ffi::c_int;
#[inline]
unsafe extern "C" fn ixheaacd_mac32_tns_sat(
    mut a: WORD32,
    mut b: WORD32,
    mut c: WORD32,
) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 32 as core::ffi::c_int) as WORD32;
    result = ixheaac_add32_sat(c, result);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_tns_decode_coefficients(
    mut filter: *const ia_filter_info_struct,
    mut a: *mut WORD32,
    mut ptr_aac_tables: *mut ia_aac_dec_tables_struct,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut tmp: WORD32 = 0;
    let mut aptr: *mut WORD32 = a;
    let mut tns_coeff_ptr: *mut WORD32 = 0 as *mut WORD32;
    let mut ixheaacd_drc_offset: WORD8 = 0;
    tmp = (*filter).resolution as WORD32;
    if tmp == 0 as core::ffi::c_int {
        tns_coeff_ptr = ((*(*ptr_aac_tables).pstr_block_tables).tns_coeff3).as_mut_ptr();
        ixheaacd_drc_offset = 4 as WORD8;
    } else {
        tns_coeff_ptr = ((*(*ptr_aac_tables).pstr_block_tables).tns_coeff4).as_mut_ptr();
        ixheaacd_drc_offset = 8 as WORD8;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*filter).order as core::ffi::c_int {
        let fresh11 = aptr;
        aptr = aptr.offset(1);
        *fresh11 = *tns_coeff_ptr
            .offset(
                ((*filter).coef[i as usize] as core::ffi::c_int
                    + ixheaacd_drc_offset as core::ffi::c_int) as isize,
            );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_tns_parcor_to_lpc(
    mut parcor: *mut WORD32,
    mut lpc: *mut WORD32,
    mut scale: *mut WORD16,
    mut order: WORD32,
) -> VOID {
    let mut i: WORD = 0;
    let mut j: WORD = 0;
    let mut status: WORD = 0;
    let mut z1: WORD32 = 0;
    let mut z: [WORD32; 32] = [0; 32];
    let mut w: [WORD32; 32] = [0; 32];
    let mut accu1: WORD32 = 0;
    let mut accu2: WORD32 = 0;
    status = 1 as core::ffi::c_int as WORD;
    *scale = 1 as WORD16;
    while status != 0 {
        status = 0 as core::ffi::c_int as WORD;
        i = MAX_ORDER as WORD;
        while i >= 0 as core::ffi::c_int {
            z[i as usize] = 0 as core::ffi::c_int as WORD32;
            w[i as usize] = 0 as core::ffi::c_int as WORD32;
            i -= 1;
        }
        accu1 = (0x40000000 as core::ffi::c_int
            >> *scale as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        i = 0 as core::ffi::c_int as WORD;
        while i <= order {
            z1 = accu1;
            j = 0 as core::ffi::c_int as WORD;
            while j < order {
                w[j as usize] = accu1;
                accu1 = ixheaac_add32_sat(
                    accu1,
                    ixheaac_mult32_shl_sat(*parcor.offset(j as isize), z[j as usize]),
                );
                if ixheaac_abs32_sat(accu1) == 0x7fffffff as core::ffi::c_int {
                    status = 1 as core::ffi::c_int as WORD;
                }
                j += 1;
            }
            j = (order as core::ffi::c_int - 1 as core::ffi::c_int) as WORD;
            while j >= 0 as core::ffi::c_int {
                accu2 = z[j as usize];
                accu2 = ixheaac_add32_sat(
                    accu2,
                    ixheaac_mult32_shl_sat(*parcor.offset(j as isize), w[j as usize]),
                );
                z[(j as core::ffi::c_int + 1 as core::ffi::c_int) as usize] = accu2;
                if ixheaac_abs32_sat(accu2) == 0x7fffffff as core::ffi::c_int {
                    status = 1 as core::ffi::c_int as WORD;
                }
                j -= 1;
            }
            z[0 as core::ffi::c_int as usize] = z1;
            *lpc.offset(i as isize) = accu1;
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
pub unsafe extern "C" fn ixheaacd_tns_parcor_lpc_convert_dec(
    mut parcor: *mut WORD16,
    mut lpc: *mut WORD16,
    mut scale: *mut WORD16,
    mut order: WORD,
) -> VOID {
    let mut i: WORD = 0;
    let mut j: WORD = 0;
    let mut status: WORD = 0;
    let mut accu: WORD32 = 0;
    let mut temp_buf1: [WORD16; 32] = [0; 32];
    let mut temp_buf2: [WORD16; 32] = [0; 32];
    let mut accu1: WORD32 = 0;
    let mut accu2: WORD32 = 0;
    status = 1 as core::ffi::c_int as WORD;
    *scale = 0 as WORD16;
    while status != 0 {
        status = 0 as core::ffi::c_int as WORD;
        i = MAX_ORDER as WORD;
        while i >= 0 as core::ffi::c_int {
            temp_buf1[i as usize] = 0 as WORD16;
            temp_buf2[i as usize] = 0 as WORD16;
            i -= 1;
        }
        accu1 = (0x7fffffff as core::ffi::c_int >> *scale as core::ffi::c_int) as WORD32;
        i = 0 as core::ffi::c_int as WORD;
        while i <= order {
            accu = accu1;
            j = 0 as core::ffi::c_int as WORD;
            while j < order {
                temp_buf2[j as usize] = ixheaac_round16(accu1);
                accu1 = ixheaac_mac16x16in32_shl_sat(
                    accu1,
                    *parcor.offset(j as isize),
                    temp_buf1[j as usize],
                );
                if ixheaac_abs32_sat(accu1) == 0x7fffffff as core::ffi::c_int {
                    status = 1 as core::ffi::c_int as WORD;
                }
                j += 1;
            }
            j = (order as core::ffi::c_int - 1 as core::ffi::c_int) as WORD;
            while j >= 0 as core::ffi::c_int {
                accu2 = ixheaac_deposit16h_in32(temp_buf1[j as usize]);
                accu2 = ixheaac_mac16x16in32_shl_sat(
                    accu2,
                    *parcor.offset(j as isize),
                    temp_buf2[j as usize],
                );
                temp_buf1[(j as core::ffi::c_int + 1 as core::ffi::c_int) as usize] = ixheaac_round16(
                    accu2,
                );
                if ixheaac_abs32_sat(accu2) == 0x7fffffff as core::ffi::c_int {
                    status = 1 as core::ffi::c_int as WORD;
                }
                j -= 1;
            }
            temp_buf1[0 as core::ffi::c_int as usize] = ixheaac_round16(accu);
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
pub unsafe extern "C" fn ixheaacd_tns_ar_filter_fixed_dec(
    mut spectrum: *mut WORD32,
    mut size: WORD32,
    mut inc: WORD32,
    mut lpc: *mut WORD32,
    mut order: WORD32,
    mut shift_value: WORD32,
    mut scale_spec: WORD,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut y: WORD32 = 0;
    let mut state: [WORD32; 32] = [0; 32];
    let mut acc: WORD32 = 0;
    if order as core::ffi::c_int & 3 as core::ffi::c_int != 0 as core::ffi::c_int {
        i = (order as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
        while i
            < (order as core::ffi::c_uint & 0xfffffffc as core::ffi::c_uint)
                as core::ffi::c_int + 4 as core::ffi::c_int
        {
            *lpc.offset(i as isize) = 0 as core::ffi::c_int as WORD32;
            i += 1;
        }
        *lpc.offset(i as isize) = 0 as core::ffi::c_int as WORD32;
        order = (order as core::ffi::c_uint & 0xfffffffc as core::ffi::c_uint)
            .wrapping_add(4 as core::ffi::c_uint) as WORD32;
        order = (order as core::ffi::c_int & 31 as core::ffi::c_int) as WORD32;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < order {
        y = ixheaac_shl32_sat(*spectrum, scale_spec);
        acc = 0 as core::ffi::c_int as WORD32;
        j = i;
        while j > 0 as core::ffi::c_int {
            acc = ixheaacd_mac32_tns_sat(
                state[(j as core::ffi::c_int - 1 as core::ffi::c_int) as usize],
                *lpc.offset(j as isize),
                acc,
            );
            state[j as usize] = state[(j as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize];
            j -= 1;
        }
        y = ixheaac_sub32_sat(y, ixheaac_shl32_sat(acc, 1 as WORD));
        state[0 as core::ffi::c_int as usize] = ixheaac_shl32_sat(
            y,
            shift_value as WORD,
        );
        *spectrum = y >> scale_spec;
        spectrum = spectrum.offset(inc as isize);
        i += 1;
    }
    i = order;
    while i < size {
        y = ixheaac_shl32_sat(*spectrum, scale_spec);
        acc = 0 as core::ffi::c_int as WORD32;
        j = order;
        while j > 0 as core::ffi::c_int {
            acc = ixheaacd_mac32_tns_sat(
                state[(j as core::ffi::c_int - 1 as core::ffi::c_int) as usize],
                *lpc.offset(j as isize),
                acc,
            );
            state[j as usize] = state[(j as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize];
            j -= 1;
        }
        y = ixheaac_sub32_sat(y, ixheaac_shl32_sat(acc, 1 as WORD));
        state[0 as core::ffi::c_int as usize] = ixheaac_shl32_sat(
            y,
            shift_value as WORD,
        );
        *spectrum = y >> scale_spec;
        spectrum = spectrum.offset(inc as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_tns_ar_filter_fixed_non_neon_armv7(
    mut spectrum: *mut WORD32,
    mut size: WORD32,
    mut inc: WORD32,
    mut lpc: *mut WORD32,
    mut order: WORD32,
    mut shift_value: WORD32,
    mut scale_spec: WORD,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut y: WORD32 = 0;
    let mut state: [WORD32; 32] = [0; 32];
    let mut acc: WORD32 = 0;
    if order as core::ffi::c_int & 3 as core::ffi::c_int != 0 as core::ffi::c_int {
        i = (order as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
        while i
            < (order as core::ffi::c_uint & 0xfffffffc as core::ffi::c_uint)
                as core::ffi::c_int + 4 as core::ffi::c_int
        {
            *lpc.offset(i as isize) = 0 as core::ffi::c_int as WORD32;
            i += 1;
        }
        *lpc.offset(i as isize) = 0 as core::ffi::c_int as WORD32;
        order = (order as core::ffi::c_uint & 0xfffffffc as core::ffi::c_uint)
            .wrapping_add(4 as core::ffi::c_uint) as WORD32;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < order {
        y = ixheaac_shl32_sat(*spectrum, scale_spec);
        acc = 0 as core::ffi::c_int as WORD32;
        j = i;
        while j > 0 as core::ffi::c_int {
            acc = ixheaacd_mac32_tns_sat(
                state[(j as core::ffi::c_int - 1 as core::ffi::c_int) as usize],
                *lpc.offset(j as isize),
                acc,
            );
            state[j as usize] = state[(j as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize];
            j -= 1;
        }
        y = ixheaac_sub32_sat(y, ixheaac_shl32_sat(acc, 1 as WORD));
        state[0 as core::ffi::c_int as usize] = ixheaac_shl32_sat(
            y,
            shift_value as WORD,
        );
        *spectrum = y >> scale_spec;
        spectrum = spectrum.offset(inc as isize);
        i += 1;
    }
    i = order;
    while i < size {
        let mut acc_0: WORD64 = 0 as WORD64;
        let mut acc1: WORD32 = 0;
        y = ixheaac_shl32_sat(*spectrum, scale_spec);
        j = order;
        while j > 0 as core::ffi::c_int {
            acc_0 = ixheaac_mac32x32in64_dual(
                state[(j as core::ffi::c_int - 1 as core::ffi::c_int) as usize],
                *lpc.offset(j as isize),
                acc_0,
            );
            state[j as usize] = state[(j as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize];
            j -= 1;
        }
        acc1 = (acc_0 >> 32 as core::ffi::c_int) as WORD32;
        y = ixheaac_sub32_sat(y, ixheaac_shl32_sat(acc1, 1 as WORD));
        state[0 as core::ffi::c_int as usize] = ixheaac_shl32_sat(
            y,
            shift_value as WORD,
        );
        *spectrum = y >> scale_spec;
        spectrum = spectrum.offset(inc as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_tns_ar_filter_fixed_armv8(
    mut spectrum: *mut WORD32,
    mut size: WORD32,
    mut inc: WORD32,
    mut lpc: *mut WORD32,
    mut order: WORD32,
    mut shift_value: WORD32,
    mut scale_spec: WORD,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut y: WORD32 = 0;
    let mut state: [WORD32; 32] = [0; 32];
    let mut acc: WORD32 = 0;
    if order as core::ffi::c_int & 3 as core::ffi::c_int != 0 as core::ffi::c_int {
        i = (order as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
        while i
            < (order as core::ffi::c_uint & 0xfffffffc as core::ffi::c_uint)
                as core::ffi::c_int + 4 as core::ffi::c_int
        {
            *lpc.offset(i as isize) = 0 as core::ffi::c_int as WORD32;
            i += 1;
        }
        *lpc.offset(i as isize) = 0 as core::ffi::c_int as WORD32;
        order = (order as core::ffi::c_uint & 0xfffffffc as core::ffi::c_uint)
            .wrapping_add(4 as core::ffi::c_uint) as WORD32;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < order {
        y = ixheaac_shl32_sat(*spectrum, scale_spec);
        acc = 0 as core::ffi::c_int as WORD32;
        j = i;
        while j > 0 as core::ffi::c_int {
            acc = ixheaacd_mac32_tns_sat(
                state[(j as core::ffi::c_int - 1 as core::ffi::c_int) as usize],
                *lpc.offset(j as isize),
                acc,
            );
            state[j as usize] = state[(j as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize];
            j -= 1;
        }
        y = ixheaac_sub32_sat(y, ixheaac_shl32_sat(acc, 1 as WORD));
        state[0 as core::ffi::c_int as usize] = ixheaac_shl32_sat(
            y,
            shift_value as WORD,
        );
        *spectrum = y >> scale_spec;
        spectrum = spectrum.offset(inc as isize);
        i += 1;
    }
    i = order;
    while i < size {
        let mut acc_0: WORD64 = 0 as WORD64;
        let mut acc1: WORD32 = 0;
        y = ixheaac_shl32_sat(*spectrum, scale_spec);
        j = order;
        while j > 0 as core::ffi::c_int {
            acc_0 = ixheaac_mac32x32in64_dual(
                state[(j as core::ffi::c_int - 1 as core::ffi::c_int) as usize],
                *lpc.offset(j as isize),
                acc_0,
            );
            state[j as usize] = state[(j as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize];
            j -= 1;
        }
        acc1 = (acc_0 >> 32 as core::ffi::c_int) as WORD32;
        y = ixheaac_sub32_sat(y, ixheaac_shl32_sat(acc1, 1 as WORD));
        state[0 as core::ffi::c_int as usize] = ixheaac_shl32_sat(
            y,
            shift_value as WORD,
        );
        *spectrum = y >> scale_spec;
        spectrum = spectrum.offset(inc as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_tns_ma_filter_fixed_ld(
    mut spectrum: *mut WORD32,
    mut size: WORD32,
    mut inc: WORD32,
    mut lpc: *mut WORD32,
    mut order: WORD32,
    mut shift_value: WORD16,
) {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut y: WORD32 = 0;
    let mut state: [WORD32; 31] = [0; 31];
    i = 0 as core::ffi::c_int as WORD32;
    while i < order {
        state[i as usize] = 0 as core::ffi::c_int as WORD32;
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < size {
        y = *spectrum;
        j = 0 as core::ffi::c_int as WORD32;
        while j < order {
            y
                += ixheaac_mult32_shl(
                    state[j as usize],
                    *lpc.offset((j as core::ffi::c_int + 1 as core::ffi::c_int) as isize),
                );
            j += 1;
        }
        j = (order as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        while j > 0 as core::ffi::c_int {
            state[j as usize] = state[(j as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize];
            j -= 1;
        }
        state[0 as core::ffi::c_int as usize] = ixheaac_shl32_dir_sat(
            *spectrum,
            shift_value as WORD,
        );
        *spectrum = y;
        spectrum = spectrum.offset(inc as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_tns_ar_filter_dec(
    mut spectrum: *mut WORD32,
    mut size: WORD32,
    mut inc: WORD32,
    mut lpc: *mut WORD16,
    mut order: WORD32,
    mut shift_value: WORD32,
    mut scale_spec: WORD,
    mut ptr_filter_state: *mut WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut y: WORD32 = 0;
    let mut acc: WORD32 = 0;
    if order as core::ffi::c_int & 3 as core::ffi::c_int != 0 as core::ffi::c_int {
        i = (order as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
        while i
            < (order as core::ffi::c_int & !(3 as core::ffi::c_int))
                + 4 as core::ffi::c_int
        {
            *lpc.offset(i as isize) = 0 as WORD16;
            i += 1;
        }
        if i < MAX_ORDER + 1 as core::ffi::c_int {
            *lpc.offset(i as isize) = 0 as WORD16;
            order = ((order as core::ffi::c_int & !(3 as core::ffi::c_int))
                + 4 as core::ffi::c_int) as WORD32;
        } else {
            order = MAX_ORDER as WORD32;
        }
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < order {
        y = ixheaac_shl32_sat(*spectrum, scale_spec);
        acc = 0 as core::ffi::c_int as WORD32;
        j = i;
        while j > 0 as core::ffi::c_int {
            acc = ixheaac_add32_sat(
                acc,
                ixheaac_mult32x16in32(
                    *ptr_filter_state
                        .offset(
                            (j as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                        ),
                    *lpc.offset(j as isize),
                ),
            );
            *ptr_filter_state.offset(j as isize) = *ptr_filter_state
                .offset((j as core::ffi::c_int - 1 as core::ffi::c_int) as isize);
            j -= 1;
        }
        y = ixheaac_sub32_sat(y, ixheaac_shl32_sat(acc, 1 as WORD));
        *ptr_filter_state.offset(0 as core::ffi::c_int as isize) = ixheaac_shl32_sat(
            y,
            shift_value as WORD,
        );
        *spectrum = y >> scale_spec;
        spectrum = spectrum.offset(inc as isize);
        i += 1;
    }
    i = order;
    while i < size {
        y = ixheaac_shl32_sat(*spectrum, scale_spec);
        acc = 0 as core::ffi::c_int as WORD32;
        j = order;
        while j > 0 as core::ffi::c_int {
            acc = ixheaac_add32_sat(
                acc,
                ixheaac_mult32x16in32(
                    *ptr_filter_state
                        .offset(
                            (j as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                        ),
                    *lpc.offset(j as isize),
                ),
            );
            *ptr_filter_state.offset(j as isize) = *ptr_filter_state
                .offset((j as core::ffi::c_int - 1 as core::ffi::c_int) as isize);
            j -= 1;
        }
        y = ixheaac_sub32_sat(y, ixheaac_shl32_sat(acc, 1 as WORD));
        *ptr_filter_state.offset(0 as core::ffi::c_int as isize) = ixheaac_shl32_sat(
            y,
            shift_value as WORD,
        );
        *spectrum = y >> scale_spec;
        spectrum = spectrum.offset(inc as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_calc_max_spectral_line_dec(
    mut ptr_tmp: *mut WORD32,
    mut size: WORD32,
) -> WORD32 {
    let mut max_spec_line: WORD32 = 0 as WORD32;
    let mut i: WORD32 = 0;
    let mut unroll_cnt: WORD = 0;
    let mut rem: WORD = 0;
    unroll_cnt = (size >> 3 as core::ffi::c_int) as WORD;
    i = unroll_cnt as WORD32;
    loop {
        let fresh0 = i;
        i = i - 1;
        if !(fresh0 != 0) {
            break;
        }
        let fresh1 = ptr_tmp;
        ptr_tmp = ptr_tmp.offset(1);
        max_spec_line = ixheaac_abs32_nrm(*fresh1) | max_spec_line;
        let fresh2 = ptr_tmp;
        ptr_tmp = ptr_tmp.offset(1);
        max_spec_line = ixheaac_abs32_nrm(*fresh2) | max_spec_line;
        let fresh3 = ptr_tmp;
        ptr_tmp = ptr_tmp.offset(1);
        max_spec_line = ixheaac_abs32_nrm(*fresh3) | max_spec_line;
        let fresh4 = ptr_tmp;
        ptr_tmp = ptr_tmp.offset(1);
        max_spec_line = ixheaac_abs32_nrm(*fresh4) | max_spec_line;
        let fresh5 = ptr_tmp;
        ptr_tmp = ptr_tmp.offset(1);
        max_spec_line = ixheaac_abs32_nrm(*fresh5) | max_spec_line;
        let fresh6 = ptr_tmp;
        ptr_tmp = ptr_tmp.offset(1);
        max_spec_line = ixheaac_abs32_nrm(*fresh6) | max_spec_line;
        let fresh7 = ptr_tmp;
        ptr_tmp = ptr_tmp.offset(1);
        max_spec_line = ixheaac_abs32_nrm(*fresh7) | max_spec_line;
        let fresh8 = ptr_tmp;
        ptr_tmp = ptr_tmp.offset(1);
        max_spec_line = ixheaac_abs32_nrm(*fresh8) | max_spec_line;
    }
    rem = size as WORD - (unroll_cnt << 3 as core::ffi::c_int);
    if rem != 0 {
        i = rem as WORD32;
        loop {
            let fresh9 = i;
            i = i - 1;
            if !(fresh9 != 0) {
                break;
            }
            let fresh10 = ptr_tmp;
            ptr_tmp = ptr_tmp.offset(1);
            max_spec_line = ixheaac_abs32_nrm(*fresh10) | max_spec_line;
        }
    }
    return ixheaac_norm32(max_spec_line) as WORD32;
}
