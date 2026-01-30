extern "C" {
    fn ixheaacd_aac_showbits_32(
        ptr_read_next: *mut UWORD8,
        cnt_bits: WORD32,
        increment: *mut WORD32,
    ) -> UWORD32;
    fn ixheaacd_aac_read_byte_corr(
        ptr_read_next: *mut *mut UWORD8,
        ptr_bit_pos: *mut WORD32,
        readword: *mut WORD32,
        p_bit_buf_end: *mut UWORD8,
    ) -> VOID;
    fn ixheaacd_aac_read_byte_corr1(
        ptr_read_next: *mut *mut UWORD8,
        ptr_bit_pos: *mut WORD32,
        readword: *mut WORD32,
        p_bit_buf_end: *mut UWORD8,
    ) -> VOID;
    fn ixheaacd_inv_quant(
        x_quant: *mut WORD32,
        ixheaacd_pow_table_Q13: *mut WORD32,
    ) -> WORD32;
    fn ixheaacd_cnt_leading_ones(a: WORD32) -> WORD32;
    fn ixheaacd_huffman_decode(
        it_bit_buff: WORD32,
        huff_index: *mut WORD16,
        len: *mut WORD16,
        input_table: *const UWORD16,
        idx_table: *const UWORD32,
    ) -> VOID;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [core::ffi::c_ulong; 16],
}
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
pub type __jmp_buf = [core::ffi::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: core::ffi::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_crc_bit_buf_struct {
    pub ptr_bit_buf_base: *mut UWORD8,
    pub ptr_bit_buf_end: *mut UWORD8,
    pub ptr_read_next: *mut UWORD8,
    pub bit_pos: WORD16,
    pub cnt_bits: WORD32,
    pub size: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_crc_reg_data_struct {
    pub active: UWORD8,
    pub buf_size: WORD32,
    pub max_bits: WORD32,
    pub bit_cnt: UWORD32,
    pub bit_buf_cnt: WORD32,
    pub str_bit_buf: ia_crc_bit_buf_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_adts_crc_info_struct {
    pub crc_active: UWORD8,
    pub no_reg: UWORD16,
    pub file_value: UWORD16,
    pub crc_lookup: [UWORD16; 256],
    pub str_crc_reg_data: [ia_crc_reg_data_struct; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_bit_buf_struct {
    pub ptr_bit_buf_base: *mut UWORD8,
    pub ptr_bit_buf_end: *mut UWORD8,
    pub ptr_read_next: *mut UWORD8,
    pub bit_pos: WORD32,
    pub cnt_bits: WORD32,
    pub size: WORD32,
    pub adts_header_present: WORD32,
    pub crc_check: WORD32,
    pub protection_absent: WORD8,
    pub no_raw_data_blocks: WORD8,
    pub str_adts_crc_info: ia_adts_crc_info_struct,
    pub pstr_adts_crc_info: *mut ia_adts_crc_info_struct,
    pub initial_cnt_bits: WORD32,
    pub audio_mux_align: WORD32,
    pub bit_count: WORD32,
    pub valid_bits: WORD32,
    pub byte: UWORD8,
    pub byte_ptr: *mut UWORD8,
    pub ptr_start: *mut UWORD8,
    pub write_bit_count: WORD32,
    pub max_size: WORD32,
    pub xaac_jmp_buf: *mut jmp_buf,
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
unsafe extern "C" fn ixheaac_mult32x16in32_sat(mut a: WORD32, mut b: WORD16) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    if temp_result < MIN_32 as WORD64 {
        result = MIN_32;
    } else if temp_result > MAX_32 as WORD64 {
        result = MAX_32;
    } else {
        result = temp_result as WORD32;
    }
    return result;
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
unsafe extern "C" fn ixheaac_shr32_sat(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut out_val: WORD32 = 0;
    b = ((b << 24 as core::ffi::c_int) as UWORD32 >> 24 as core::ffi::c_int) as WORD32;
    if b >= 31 as core::ffi::c_int {
        if a < 0 as core::ffi::c_int {
            out_val = -(1 as core::ffi::c_int) as WORD32;
        } else {
            out_val = 0 as core::ffi::c_int as WORD32;
        }
    } else if b <= 0 as core::ffi::c_int {
        return a
    } else {
        a = ixheaac_add32_sat(
            a,
            (1 as WORD32) << b as core::ffi::c_int - 1 as core::ffi::c_int,
        );
        out_val = a >> b;
    }
    return out_val;
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
unsafe extern "C" fn ixheaac_shl16_sat(mut op1: WORD16, mut shift: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    let mut temp: WORD32 = 0;
    if shift as core::ffi::c_int > 15 as core::ffi::c_int {
        shift = 15 as WORD16;
    }
    temp = (op1 as core::ffi::c_int) << shift as core::ffi::c_int;
    var_out = ixheaac_sat16(temp);
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
unsafe extern "C" fn ixheaac_mult32x16in32(mut a: WORD32, mut b: WORD16) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 16 as core::ffi::c_int) as WORD32;
    return result;
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
unsafe extern "C" fn ixheaac_mac32x16in32_shl(
    mut a: WORD32,
    mut b: WORD32,
    mut c: WORD16,
) -> WORD32 {
    let mut result: WORD32 = 0;
    result = a + ixheaac_mult32x16in32_shl(b, c);
    return result;
}
#[inline]
unsafe extern "C" fn ixheaac_extu(
    mut a: UWORD32,
    mut shift_left: WORD32,
    mut shift_right: WORD32,
) -> UWORD32 {
    let mut x: UWORD32 = 0;
    x = a << shift_left;
    x = x >> shift_right;
    return x;
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
pub const MAX_BINS_SHORT: core::ffi::c_int = 128 as core::ffi::c_int;
pub const IQ_TABLE_SIZE_HALF: core::ffi::c_int = 128 as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES: core::ffi::c_int = 0x1804
    as core::ffi::c_int;
#[inline]
unsafe extern "C" fn ixheaacd_mac32x16in32_sat(
    mut a: WORD32,
    mut b: WORD32,
    mut c: WORD16,
) -> WORD32 {
    let mut acc: WORD32 = 0;
    acc = ixheaac_mult32x16in32_sat(b, c);
    acc = ixheaac_add32_sat(a, acc);
    return acc;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_huff_sfb_table(
    mut it_bit_buff: WORD32,
    mut huff_index: *mut WORD16,
    mut len: *mut WORD32,
    mut code_book_tbl: *const UWORD16,
    mut idx_table: *const UWORD32,
) -> VOID {
    let mut temp: UWORD32 = 0 as UWORD32;
    let mut temp1: UWORD32 = 0 as UWORD32;
    let mut found: WORD32 = 0 as WORD32;
    let mut mask: UWORD32 = 0x80000000 as UWORD32;
    let mut leading_ones: WORD32 = 0;
    let mut max_len: WORD32 = 0;
    let mut ixheaacd_drc_offset: WORD32 = 0 as WORD32;
    let mut length: WORD32 = 0;
    let mut code_word: UWORD32 = 0;
    let mut len_end: WORD32 = 0;
    max_len = *code_book_tbl.offset(0 as core::ffi::c_int as isize) as WORD32;
    mask = mask
        .wrapping_sub(((1 as core::ffi::c_int) << 31 as WORD32 - max_len) as UWORD32);
    mask = mask << 1 as core::ffi::c_int;
    temp = it_bit_buff as UWORD32 & mask;
    len_end = *code_book_tbl.offset(0 as core::ffi::c_int as isize) as WORD32;
    leading_ones = ixheaacd_cnt_leading_ones(temp as WORD32);
    loop {
        ixheaacd_drc_offset = (*idx_table.offset(leading_ones as isize)
            >> 20 as core::ffi::c_int & 0x1ff as UWORD32) as WORD32;
        length = (*code_book_tbl
            .offset(
                (ixheaacd_drc_offset as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            ) as core::ffi::c_int & 0x1f as core::ffi::c_int) as WORD32;
        code_word = *idx_table.offset(leading_ones as isize) & 0xfffff as UWORD32;
        temp1 = temp >> 32 as WORD32 - length;
        if temp1 <= code_word {
            ixheaacd_drc_offset = (ixheaacd_drc_offset as UWORD32)
                .wrapping_sub(code_word.wrapping_sub(temp1)) as WORD32;
            found = 1 as core::ffi::c_int as WORD32;
        } else {
            len_end = (len_end as UWORD32)
                .wrapping_add(
                    *idx_table.offset(leading_ones as isize) >> 29 as core::ffi::c_int
                        & 0x7 as UWORD32,
                ) as WORD32;
            leading_ones = len_end;
        }
        if !(found == 0) {
            break;
        }
    }
    *huff_index = (*code_book_tbl
        .offset(
            (ixheaacd_drc_offset as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
        ) as core::ffi::c_int >> 5 as core::ffi::c_int) as WORD16;
    *len = length;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_inverse_quantize(
    mut x_invquant: *mut WORD32,
    mut no_band: WORD,
    mut ixheaacd_pow_table_Q13: *mut WORD32,
    mut scratch_in: *mut WORD8,
) -> VOID {
    let mut j: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut q_abs: WORD32 = 0;
    j = (no_band as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while j >= 0 as core::ffi::c_int {
        let fresh0 = scratch_in;
        scratch_in = scratch_in.offset(1);
        q_abs = *fresh0 as WORD32;
        temp = *ixheaacd_pow_table_Q13.offset(q_abs as isize);
        let fresh1 = x_invquant;
        x_invquant = x_invquant.offset(1);
        *fresh1 = -temp;
        j -= 1;
    }
}
#[inline]
unsafe extern "C" fn ixheaacd_huffman_dec_word1(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut spec_coef: *mut WORD32,
    mut offsets: *mut WORD16,
    mut no_bands: WORD,
    mut group_len: WORD,
    mut code_book_tbl: *const UWORD16,
    mut ixheaacd_pow_table_Q13: *mut WORD32,
    mut idx_table: *const UWORD32,
    mut maximum_bins_short: WORD32,
) -> WORD {
    let mut sp1: WORD32 = 0;
    let mut sp2: WORD32 = 0;
    let mut flush_cw: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut value: WORD32 = 0;
    let mut norm_val: WORD32 = 0;
    let mut off: WORD32 = 0;
    let mut idx: WORD = 0;
    let mut grp_idx: WORD = 0;
    let mut out1: WORD32 = 0;
    let mut out2: WORD32 = 0;
    let mut err_code: WORD32 = 0 as WORD32;
    let mut len_idx: WORD = 0 as WORD;
    let mut ptr_read_next: *mut UWORD8 = (*it_bit_buff).ptr_read_next;
    let mut bit_pos: WORD32 = (*it_bit_buff).bit_pos;
    let mut index: WORD16 = 0;
    let mut length: WORD32 = 0;
    let mut read_word: WORD32 = 0;
    let mut increment: WORD32 = 0;
    read_word = ixheaacd_aac_showbits_32(
        ptr_read_next,
        (*it_bit_buff).cnt_bits,
        &mut increment,
    ) as WORD32;
    ptr_read_next = ptr_read_next.offset(increment as isize);
    loop {
        len_idx = (*offsets.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
            - *offsets.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
            as WORD;
        grp_idx = group_len;
        loop {
            spec_coef = spec_coef
                .offset(
                    *offsets.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                        as isize,
                );
            idx = len_idx;
            loop {
                let mut read_word1: UWORD32 = 0;
                read_word1 = (read_word << bit_pos) as UWORD32;
                ixheaacd_huff_sfb_table(
                    read_word1 as WORD32,
                    &mut index,
                    &mut length,
                    code_book_tbl,
                    idx_table,
                );
                bit_pos += length;
                ixheaacd_aac_read_byte_corr(
                    &mut ptr_read_next,
                    &mut bit_pos,
                    &mut read_word,
                    (*it_bit_buff).ptr_bit_buf_end,
                );
                out1 = (index as core::ffi::c_int / 17 as core::ffi::c_int) as WORD32;
                out2 = (index as core::ffi::c_int
                    - out1 as core::ffi::c_int * 17 as core::ffi::c_int) as WORD32;
                flush_cw = read_word << bit_pos;
                sp1 = out1;
                sp2 = out2;
                if out1 != 0 {
                    if flush_cw as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint
                        != 0
                    {
                        out1 = -out1;
                    }
                    bit_pos += 1;
                    flush_cw = flush_cw << 1 as core::ffi::c_int;
                }
                if out2 != 0 {
                    bit_pos += 1;
                    if flush_cw as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint
                        != 0
                    {
                        out2 = -out2;
                    }
                }
                ixheaacd_aac_read_byte_corr(
                    &mut ptr_read_next,
                    &mut bit_pos,
                    &mut read_word,
                    (*it_bit_buff).ptr_bit_buf_end,
                );
                if sp1 == 16 as core::ffi::c_int {
                    i = 4 as core::ffi::c_int as WORD32;
                    value = ixheaac_extu(read_word as UWORD32, bit_pos, 23 as WORD32)
                        as WORD32;
                    value = (value as core::ffi::c_uint
                        | 0xfffffe00 as core::ffi::c_uint) as WORD32;
                    norm_val = ixheaac_norm32(value) as WORD32;
                    i += norm_val as core::ffi::c_int - 22 as core::ffi::c_int;
                    bit_pos += norm_val as core::ffi::c_int - 21 as core::ffi::c_int;
                    ixheaacd_aac_read_byte_corr(
                        &mut ptr_read_next,
                        &mut bit_pos,
                        &mut read_word,
                        (*it_bit_buff).ptr_bit_buf_end,
                    );
                    off = ixheaac_extu(read_word as UWORD32, bit_pos, 32 as WORD32 - i)
                        as WORD32;
                    bit_pos += i;
                    ixheaacd_aac_read_byte_corr(
                        &mut ptr_read_next,
                        &mut bit_pos,
                        &mut read_word,
                        (*it_bit_buff).ptr_bit_buf_end,
                    );
                    ixheaacd_aac_read_byte_corr(
                        &mut ptr_read_next,
                        &mut bit_pos,
                        &mut read_word,
                        (*it_bit_buff).ptr_bit_buf_end,
                    );
                    i = off + ((1 as core::ffi::c_int) << i);
                    if i <= IQ_TABLE_SIZE_HALF {
                        i = *ixheaacd_pow_table_Q13.offset(i as isize);
                    } else {
                        err_code |= ixheaacd_inv_quant(&mut i, ixheaacd_pow_table_Q13);
                    }
                    if out1 < 0 as core::ffi::c_int {
                        out1 = -i;
                    } else {
                        out1 = i;
                    }
                    let fresh26 = spec_coef;
                    spec_coef = spec_coef.offset(1);
                    *fresh26 = out1;
                } else if out1 <= 0 as core::ffi::c_int {
                    out1 = -out1;
                    out1 = *ixheaacd_pow_table_Q13.offset(out1 as isize);
                    let fresh27 = spec_coef;
                    spec_coef = spec_coef.offset(1);
                    *fresh27 = -out1;
                } else {
                    out1 = *ixheaacd_pow_table_Q13.offset(out1 as isize);
                    let fresh28 = spec_coef;
                    spec_coef = spec_coef.offset(1);
                    *fresh28 = out1;
                }
                if sp2 == 16 as core::ffi::c_int {
                    i = 4 as core::ffi::c_int as WORD32;
                    value = ixheaac_extu(read_word as UWORD32, bit_pos, 23 as WORD32)
                        as WORD32;
                    value = (value as core::ffi::c_uint
                        | 0xfffffe00 as core::ffi::c_uint) as WORD32;
                    norm_val = ixheaac_norm32(value) as WORD32;
                    i += norm_val as core::ffi::c_int - 22 as core::ffi::c_int;
                    bit_pos += norm_val as core::ffi::c_int - 21 as core::ffi::c_int;
                    ixheaacd_aac_read_byte_corr(
                        &mut ptr_read_next,
                        &mut bit_pos,
                        &mut read_word,
                        (*it_bit_buff).ptr_bit_buf_end,
                    );
                    off = ixheaac_extu(read_word as UWORD32, bit_pos, 32 as WORD32 - i)
                        as WORD32;
                    bit_pos += i;
                    ixheaacd_aac_read_byte_corr(
                        &mut ptr_read_next,
                        &mut bit_pos,
                        &mut read_word,
                        (*it_bit_buff).ptr_bit_buf_end,
                    );
                    ixheaacd_aac_read_byte_corr(
                        &mut ptr_read_next,
                        &mut bit_pos,
                        &mut read_word,
                        (*it_bit_buff).ptr_bit_buf_end,
                    );
                    i = off + ((1 as core::ffi::c_int) << i);
                    if i <= IQ_TABLE_SIZE_HALF {
                        i = *ixheaacd_pow_table_Q13.offset(i as isize);
                    } else {
                        err_code |= ixheaacd_inv_quant(&mut i, ixheaacd_pow_table_Q13);
                    }
                    if out2 < 0 as core::ffi::c_int {
                        out2 = -i;
                    } else {
                        out2 = i;
                    }
                    let fresh29 = spec_coef;
                    spec_coef = spec_coef.offset(1);
                    *fresh29 = out2;
                } else if out2 <= 0 as core::ffi::c_int {
                    out2 = -out2;
                    out2 = *ixheaacd_pow_table_Q13.offset(out2 as isize);
                    let fresh30 = spec_coef;
                    spec_coef = spec_coef.offset(1);
                    *fresh30 = -out2;
                } else {
                    out2 = *ixheaacd_pow_table_Q13.offset(out2 as isize);
                    let fresh31 = spec_coef;
                    spec_coef = spec_coef.offset(1);
                    *fresh31 = out2;
                }
                idx -= 2 as core::ffi::c_int;
                if !(idx != 0 as core::ffi::c_int) {
                    break;
                }
            }
            if maximum_bins_short == 120 as core::ffi::c_int {
                spec_coef = spec_coef
                    .offset(
                        (maximum_bins_short as core::ffi::c_int
                            - *offsets.offset(1 as core::ffi::c_int as isize)
                                as core::ffi::c_int) as isize,
                    );
            } else {
                spec_coef = spec_coef
                    .offset(
                        (MAX_BINS_SHORT
                            - *offsets.offset(1 as core::ffi::c_int as isize)
                                as core::ffi::c_int) as isize,
                    );
            }
            grp_idx -= 1;
            if !(grp_idx != 0 as core::ffi::c_int) {
                break;
            }
        }
        offsets = offsets.offset(1);
        if maximum_bins_short == 120 as core::ffi::c_int {
            spec_coef = spec_coef
                .offset(-((maximum_bins_short as WORD * group_len) as isize));
        } else {
            spec_coef = spec_coef.offset(-((MAX_BINS_SHORT * group_len) as isize));
        }
        no_bands -= 1;
        if !(no_bands >= 0 as core::ffi::c_int) {
            break;
        }
    }
    ptr_read_next = ptr_read_next.offset(-(increment as isize));
    ixheaacd_aac_read_byte_corr1(
        &mut ptr_read_next,
        &mut bit_pos,
        &mut read_word,
        (*it_bit_buff).ptr_bit_buf_end,
    );
    (*it_bit_buff).bit_pos = bit_pos;
    (*it_bit_buff).ptr_read_next = ptr_read_next;
    return err_code as WORD;
}
#[inline]
unsafe extern "C" fn ixheaacd_huffman_dec_word2_11(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut width: WORD32,
    mut code_book_tbl: *const UWORD16,
    mut x_invquant: *mut WORD32,
    mut ixheaacd_pow_table_Q13: *mut WORD32,
    mut ptr_scratch: *mut WORD8,
    mut idx_table: *const UWORD32,
) -> WORD {
    let mut sp1: WORD32 = 0;
    let mut sp2: WORD32 = 0;
    let mut flush_cw: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut value: WORD32 = 0;
    let mut norm_val: WORD32 = 0;
    let mut off: WORD32 = 0;
    let mut idx: WORD = 0;
    let mut out1: WORD32 = 0;
    let mut out2: WORD32 = 0;
    let mut err_code: WORD32 = 0 as WORD32;
    let mut index: WORD16 = 0;
    let mut length: WORD32 = 0;
    let mut ptr_read_next: *mut UWORD8 = (*it_bit_buff).ptr_read_next;
    let mut bit_pos: WORD32 = (*it_bit_buff).bit_pos;
    let mut read_word: WORD32 = 0;
    let mut increment: WORD32 = 0;
    read_word = ixheaacd_aac_showbits_32(
        ptr_read_next,
        (*it_bit_buff).cnt_bits,
        &mut increment,
    ) as WORD32;
    ptr_read_next = ptr_read_next.offset(increment as isize);
    idx = width as WORD;
    while idx != 0 as core::ffi::c_int {
        let mut read_word1: UWORD32 = 0;
        read_word1 = (read_word << bit_pos) as UWORD32;
        ixheaacd_huff_sfb_table(
            read_word1 as WORD32,
            &mut index,
            &mut length,
            code_book_tbl,
            idx_table,
        );
        bit_pos += length;
        ixheaacd_aac_read_byte_corr(
            &mut ptr_read_next,
            &mut bit_pos,
            &mut read_word,
            (*it_bit_buff).ptr_bit_buf_end,
        );
        flush_cw = read_word << bit_pos;
        out1 = (index as core::ffi::c_int / 17 as core::ffi::c_int) as WORD32;
        out2 = (index as core::ffi::c_int
            - out1 as core::ffi::c_int * 17 as core::ffi::c_int) as WORD32;
        sp1 = out1;
        if out1 != 0 {
            if flush_cw as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0 {
                out1 = -out1;
            }
            bit_pos += 1;
            flush_cw = flush_cw << 1 as core::ffi::c_int;
        }
        sp2 = out2;
        if out2 != 0 {
            bit_pos += 1;
            if flush_cw as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0 {
                out2 = -out2;
            }
        }
        ixheaacd_aac_read_byte_corr(
            &mut ptr_read_next,
            &mut bit_pos,
            &mut read_word,
            (*it_bit_buff).ptr_bit_buf_end,
        );
        if sp1 == 16 as core::ffi::c_int {
            i = 4 as core::ffi::c_int as WORD32;
            value = ixheaac_extu(read_word as UWORD32, bit_pos, 23 as WORD32) as WORD32;
            value = (value as core::ffi::c_uint | 0xfffffe00 as core::ffi::c_uint)
                as WORD32;
            norm_val = ixheaac_norm32(value) as WORD32;
            i += norm_val as core::ffi::c_int - 22 as core::ffi::c_int;
            bit_pos += norm_val as core::ffi::c_int - 21 as core::ffi::c_int;
            ixheaacd_aac_read_byte_corr(
                &mut ptr_read_next,
                &mut bit_pos,
                &mut read_word,
                (*it_bit_buff).ptr_bit_buf_end,
            );
            off = ixheaac_extu(read_word as UWORD32, bit_pos, 32 as WORD32 - i)
                as WORD32;
            bit_pos += i;
            ixheaacd_aac_read_byte_corr(
                &mut ptr_read_next,
                &mut bit_pos,
                &mut read_word,
                (*it_bit_buff).ptr_bit_buf_end,
            );
            let fresh58 = ptr_scratch;
            ptr_scratch = ptr_scratch.offset(1);
            value = *fresh58 as WORD32;
            ixheaacd_aac_read_byte_corr(
                &mut ptr_read_next,
                &mut bit_pos,
                &mut read_word,
                (*it_bit_buff).ptr_bit_buf_end,
            );
            i = off + ((1 as core::ffi::c_int) << i);
            i += value;
            if i <= IQ_TABLE_SIZE_HALF {
                i = *ixheaacd_pow_table_Q13.offset(i as isize);
            } else {
                err_code |= ixheaacd_inv_quant(&mut i, ixheaacd_pow_table_Q13);
            }
            if out1 < 0 as core::ffi::c_int {
                i = -i;
            }
            let fresh59 = x_invquant;
            x_invquant = x_invquant.offset(1);
            *fresh59 = i;
        } else {
            let fresh60 = ptr_scratch;
            ptr_scratch = ptr_scratch.offset(1);
            let mut temp: WORD8 = *fresh60;
            if out1 <= 0 as core::ffi::c_int {
                out1 = temp as WORD32 - out1;
                out1 = *ixheaacd_pow_table_Q13.offset(out1 as isize);
                let fresh61 = x_invquant;
                x_invquant = x_invquant.offset(1);
                *fresh61 = -out1;
            } else {
                out1 += temp as core::ffi::c_int;
                out1 = *ixheaacd_pow_table_Q13.offset(out1 as isize);
                let fresh62 = x_invquant;
                x_invquant = x_invquant.offset(1);
                *fresh62 = out1;
            }
        }
        if sp2 == 16 as core::ffi::c_int {
            i = 4 as core::ffi::c_int as WORD32;
            value = ixheaac_extu(read_word as UWORD32, bit_pos, 23 as WORD32) as WORD32;
            value = (value as core::ffi::c_uint | 0xfffffe00 as core::ffi::c_uint)
                as WORD32;
            norm_val = ixheaac_norm32(value) as WORD32;
            i += norm_val as core::ffi::c_int - 22 as core::ffi::c_int;
            bit_pos += norm_val as core::ffi::c_int - 21 as core::ffi::c_int;
            ixheaacd_aac_read_byte_corr(
                &mut ptr_read_next,
                &mut bit_pos,
                &mut read_word,
                (*it_bit_buff).ptr_bit_buf_end,
            );
            off = ixheaac_extu(read_word as UWORD32, bit_pos, 32 as WORD32 - i)
                as WORD32;
            bit_pos += i;
            ixheaacd_aac_read_byte_corr(
                &mut ptr_read_next,
                &mut bit_pos,
                &mut read_word,
                (*it_bit_buff).ptr_bit_buf_end,
            );
            let fresh63 = ptr_scratch;
            ptr_scratch = ptr_scratch.offset(1);
            value = *fresh63 as WORD32;
            ixheaacd_aac_read_byte_corr(
                &mut ptr_read_next,
                &mut bit_pos,
                &mut read_word,
                (*it_bit_buff).ptr_bit_buf_end,
            );
            i = off + ((1 as core::ffi::c_int) << i);
            i += value;
            if i <= IQ_TABLE_SIZE_HALF {
                i = *ixheaacd_pow_table_Q13.offset(i as isize);
            } else {
                err_code |= ixheaacd_inv_quant(&mut i, ixheaacd_pow_table_Q13);
            }
            if out2 < 0 as core::ffi::c_int {
                i = -i;
            }
            let fresh64 = x_invquant;
            x_invquant = x_invquant.offset(1);
            *fresh64 = i;
        } else {
            let fresh65 = ptr_scratch;
            ptr_scratch = ptr_scratch.offset(1);
            let mut temp_0: WORD8 = *fresh65;
            if out2 <= 0 as core::ffi::c_int {
                out2 = temp_0 as WORD32 - out2;
                out2 = *ixheaacd_pow_table_Q13.offset(out2 as isize);
                let fresh66 = x_invquant;
                x_invquant = x_invquant.offset(1);
                *fresh66 = -out2;
            } else {
                out2 += temp_0 as core::ffi::c_int;
                out2 = *ixheaacd_pow_table_Q13.offset(out2 as isize);
                let fresh67 = x_invquant;
                x_invquant = x_invquant.offset(1);
                *fresh67 = out2;
            }
        }
        idx -= 2 as core::ffi::c_int;
    }
    ptr_read_next = ptr_read_next.offset(-(increment as isize));
    ixheaacd_aac_read_byte_corr1(
        &mut ptr_read_next,
        &mut bit_pos,
        &mut read_word,
        (*it_bit_buff).ptr_bit_buf_end,
    );
    (*it_bit_buff).ptr_read_next = ptr_read_next;
    (*it_bit_buff).bit_pos = bit_pos;
    return err_code as WORD;
}
#[inline]
unsafe extern "C" fn ixheaacd_huffman_dec_quad(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut spec_coef: *mut WORD32,
    mut offsets: *mut WORD16,
    mut no_bands: WORD,
    mut group_len: WORD,
    mut code_book_tbl: *const UWORD16,
    mut ixheaacd_pow_table_Q13: *mut WORD32,
    mut tbl_sign: WORD32,
    mut idx_table: *const UWORD32,
    mut maximum_bins_short: WORD32,
) -> WORD {
    let mut idx: WORD = 0;
    let mut grp_idx: WORD = 0;
    let mut idx_len: WORD = 0;
    let mut spec_orig: *mut WORD32 = 0 as *mut WORD32;
    let mut index: WORD16 = 0;
    let mut length: WORD16 = 0;
    let mut ptr_read_next: *mut UWORD8 = (*it_bit_buff).ptr_read_next;
    let mut bit_pos: WORD32 = (*it_bit_buff).bit_pos;
    let mut read_word: WORD32 = 0;
    let mut increment: WORD32 = 0;
    read_word = ixheaacd_aac_showbits_32(
        ptr_read_next,
        (*it_bit_buff).cnt_bits,
        &mut increment,
    ) as WORD32;
    ptr_read_next = ptr_read_next.offset(increment as isize);
    spec_orig = spec_coef;
    loop {
        idx_len = (*offsets.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
            - *offsets.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
            as WORD;
        grp_idx = group_len;
        loop {
            spec_coef = spec_coef
                .offset(
                    *offsets.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                        as isize,
                );
            idx = idx_len;
            loop {
                let mut read_word1: UWORD32 = 0;
                read_word1 = (read_word << bit_pos) as UWORD32;
                ixheaacd_huffman_decode(
                    read_word1 as WORD32,
                    &mut index,
                    &mut length,
                    code_book_tbl,
                    idx_table,
                );
                bit_pos += length as core::ffi::c_int;
                ixheaacd_aac_read_byte_corr(
                    &mut ptr_read_next,
                    &mut bit_pos,
                    &mut read_word,
                    (*it_bit_buff).ptr_bit_buf_end,
                );
                if tbl_sign != 0 {
                    let mut temp_word: WORD32 = 0;
                    let mut w: WORD32 = 0;
                    let mut x: WORD32 = 0;
                    let mut y: WORD32 = 0;
                    let mut z: WORD32 = 0;
                    temp_word = read_word << bit_pos;
                    w = (index as core::ffi::c_int / 27 as core::ffi::c_int) as WORD32;
                    index = (index as core::ffi::c_int
                        - w as core::ffi::c_int * 27 as core::ffi::c_int) as WORD16;
                    x = (index as core::ffi::c_int / 9 as core::ffi::c_int) as WORD32;
                    index = (index as core::ffi::c_int
                        - x as core::ffi::c_int * 9 as core::ffi::c_int) as WORD16;
                    y = (index as core::ffi::c_int / 3 as core::ffi::c_int) as WORD32;
                    z = (index as core::ffi::c_int
                        - y as core::ffi::c_int * 3 as core::ffi::c_int) as WORD32;
                    if w != 0 {
                        w = *ixheaacd_pow_table_Q13.offset(w as isize);
                        if temp_word as core::ffi::c_uint
                            & 0x80000000 as core::ffi::c_uint != 0
                        {
                            w = -w;
                        }
                        temp_word <<= 1 as core::ffi::c_int;
                        bit_pos += 1;
                    }
                    let fresh18 = spec_coef;
                    spec_coef = spec_coef.offset(1);
                    *fresh18 = w;
                    if x != 0 {
                        x = *ixheaacd_pow_table_Q13.offset(x as isize);
                        if temp_word as core::ffi::c_uint
                            & 0x80000000 as core::ffi::c_uint != 0
                        {
                            x = -x;
                        }
                        temp_word <<= 1 as core::ffi::c_int;
                        bit_pos += 1;
                    }
                    let fresh19 = spec_coef;
                    spec_coef = spec_coef.offset(1);
                    *fresh19 = x;
                    if y != 0 {
                        y = *ixheaacd_pow_table_Q13.offset(y as isize);
                        if temp_word as core::ffi::c_uint
                            & 0x80000000 as core::ffi::c_uint != 0
                        {
                            y = -y;
                        }
                        temp_word <<= 1 as core::ffi::c_int;
                        bit_pos += 1;
                    }
                    let fresh20 = spec_coef;
                    spec_coef = spec_coef.offset(1);
                    *fresh20 = y;
                    if z != 0 {
                        z = *ixheaacd_pow_table_Q13.offset(z as isize);
                        if temp_word as core::ffi::c_uint
                            & 0x80000000 as core::ffi::c_uint != 0
                        {
                            z = -z;
                        }
                        temp_word <<= 1 as core::ffi::c_int;
                        bit_pos += 1;
                    }
                    let fresh21 = spec_coef;
                    spec_coef = spec_coef.offset(1);
                    *fresh21 = z;
                } else {
                    let mut w_0: WORD32 = 0;
                    let mut x_0: WORD32 = 0;
                    let mut y_0: WORD32 = 0;
                    let mut z_0: WORD32 = 0;
                    w_0 = (index as core::ffi::c_int / 27 as core::ffi::c_int
                        - 1 as core::ffi::c_int) as WORD32;
                    index = (index as core::ffi::c_int
                        - (w_0 as core::ffi::c_int + 1 as core::ffi::c_int)
                            * 27 as core::ffi::c_int) as WORD16;
                    x_0 = (index as core::ffi::c_int / 9 as core::ffi::c_int
                        - 1 as core::ffi::c_int) as WORD32;
                    index = (index as core::ffi::c_int
                        - (x_0 as core::ffi::c_int + 1 as core::ffi::c_int)
                            * 9 as core::ffi::c_int) as WORD16;
                    y_0 = (index as core::ffi::c_int / 3 as core::ffi::c_int
                        - 1 as core::ffi::c_int) as WORD32;
                    z_0 = (index as core::ffi::c_int
                        - (y_0 as core::ffi::c_int + 1 as core::ffi::c_int)
                            * 3 as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
                    if w_0 < 0 as core::ffi::c_int {
                        w_0 = -w_0;
                        w_0 = *ixheaacd_pow_table_Q13.offset(w_0 as isize);
                        w_0 = -w_0;
                    } else {
                        w_0 = *ixheaacd_pow_table_Q13.offset(w_0 as isize);
                    }
                    let fresh22 = spec_coef;
                    spec_coef = spec_coef.offset(1);
                    *fresh22 = w_0;
                    if x_0 < 0 as core::ffi::c_int {
                        x_0 = -x_0;
                        x_0 = *ixheaacd_pow_table_Q13.offset(x_0 as isize);
                        x_0 = -x_0;
                    } else {
                        x_0 = *ixheaacd_pow_table_Q13.offset(x_0 as isize);
                    }
                    let fresh23 = spec_coef;
                    spec_coef = spec_coef.offset(1);
                    *fresh23 = x_0;
                    if y_0 < 0 as core::ffi::c_int {
                        y_0 = -y_0;
                        y_0 = *ixheaacd_pow_table_Q13.offset(y_0 as isize);
                        y_0 = -y_0;
                    } else {
                        y_0 = *ixheaacd_pow_table_Q13.offset(y_0 as isize);
                    }
                    let fresh24 = spec_coef;
                    spec_coef = spec_coef.offset(1);
                    *fresh24 = y_0;
                    if z_0 < 0 as core::ffi::c_int {
                        z_0 = -z_0;
                        z_0 = *ixheaacd_pow_table_Q13.offset(z_0 as isize);
                        z_0 = -z_0;
                    } else {
                        z_0 = *ixheaacd_pow_table_Q13.offset(z_0 as isize);
                    }
                    let fresh25 = spec_coef;
                    spec_coef = spec_coef.offset(1);
                    *fresh25 = z_0;
                }
                ixheaacd_aac_read_byte_corr(
                    &mut ptr_read_next,
                    &mut bit_pos,
                    &mut read_word,
                    (*it_bit_buff).ptr_bit_buf_end,
                );
                idx -= 4 as core::ffi::c_int;
                if !(idx != 0 as core::ffi::c_int) {
                    break;
                }
            }
            if maximum_bins_short == 120 as core::ffi::c_int {
                spec_coef = spec_coef
                    .offset(
                        (maximum_bins_short as core::ffi::c_int
                            - *offsets.offset(1 as core::ffi::c_int as isize)
                                as core::ffi::c_int) as isize,
                    );
            } else {
                spec_coef = spec_coef
                    .offset(
                        (MAX_BINS_SHORT
                            - *offsets.offset(1 as core::ffi::c_int as isize)
                                as core::ffi::c_int) as isize,
                    );
            }
            grp_idx -= 1;
            if !(grp_idx != 0 as core::ffi::c_int) {
                break;
            }
        }
        offsets = offsets.offset(1);
        spec_coef = spec_orig;
        no_bands -= 1;
        if !(no_bands >= 0 as core::ffi::c_int) {
            break;
        }
    }
    ptr_read_next = ptr_read_next.offset(-(increment as isize));
    ixheaacd_aac_read_byte_corr1(
        &mut ptr_read_next,
        &mut bit_pos,
        &mut read_word,
        (*it_bit_buff).ptr_bit_buf_end,
    );
    (*it_bit_buff).ptr_read_next = ptr_read_next;
    (*it_bit_buff).bit_pos = bit_pos;
    return 0 as WORD;
}
#[inline]
unsafe extern "C" fn ixheaacd_huffman_dec_word2_quad(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut width: WORD32,
    mut code_book_tbl: *const UWORD16,
    mut x_invquant: *mut WORD32,
    mut ixheaacd_pow_table_Q13: *mut WORD32,
    mut ptr_scratch: *mut WORD8,
    mut tbl_sign: WORD32,
    mut idx_table: *const UWORD32,
) -> WORD {
    let mut idx: WORD = 0;
    let mut index: WORD16 = 0;
    let mut length: WORD16 = 0;
    let mut ptr_read_next: *mut UWORD8 = (*it_bit_buff).ptr_read_next;
    let mut bit_pos: WORD32 = (*it_bit_buff).bit_pos;
    let mut read_word: WORD32 = 0;
    let mut increment: WORD32 = 0;
    read_word = ixheaacd_aac_showbits_32(
        ptr_read_next,
        (*it_bit_buff).cnt_bits,
        &mut increment,
    ) as WORD32;
    ptr_read_next = ptr_read_next.offset(increment as isize);
    idx = width as WORD;
    while idx != 0 as core::ffi::c_int {
        let mut ampres: WORD32 = 0;
        let mut ampres1: WORD32 = 0;
        let mut ampres2: WORD32 = 0;
        let mut ampres3: WORD32 = 0;
        let mut read_word1: UWORD32 = 0;
        read_word1 = (read_word << bit_pos) as UWORD32;
        ixheaacd_huffman_decode(
            read_word1 as WORD32,
            &mut index,
            &mut length,
            code_book_tbl,
            idx_table,
        );
        bit_pos += length as core::ffi::c_int;
        ixheaacd_aac_read_byte_corr(
            &mut ptr_read_next,
            &mut bit_pos,
            &mut read_word,
            (*it_bit_buff).ptr_bit_buf_end,
        );
        if tbl_sign != 0 {
            let mut w: WORD32 = 0;
            let mut x: WORD32 = 0;
            let mut y: WORD32 = 0;
            let mut z: WORD32 = 0;
            let mut ampout0: WORD32 = 0;
            let mut ampout1: WORD32 = 0;
            let mut ampout2: WORD32 = 0;
            let mut ampout3: WORD32 = 0;
            let mut temp_word: WORD32 = 0;
            temp_word = read_word << bit_pos;
            w = (index as core::ffi::c_int / 27 as core::ffi::c_int) as WORD32;
            index = (index as core::ffi::c_int
                - w as core::ffi::c_int * 27 as core::ffi::c_int) as WORD16;
            x = (index as core::ffi::c_int / 9 as core::ffi::c_int) as WORD32;
            index = (index as core::ffi::c_int
                - x as core::ffi::c_int * 9 as core::ffi::c_int) as WORD16;
            y = (index as core::ffi::c_int / 3 as core::ffi::c_int) as WORD32;
            z = (index as core::ffi::c_int
                - y as core::ffi::c_int * 3 as core::ffi::c_int) as WORD32;
            let fresh42 = ptr_scratch;
            ptr_scratch = ptr_scratch.offset(1);
            ampout0 = (w as core::ffi::c_int + *fresh42 as core::ffi::c_int) as WORD32;
            ampout0 = *ixheaacd_pow_table_Q13.offset(ampout0 as isize);
            if w != 0 {
                if temp_word as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0
                {
                    ampout0 = -ampout0;
                }
                temp_word = temp_word << 1 as core::ffi::c_int;
                bit_pos += 1;
            } else {
                ampout0 = -ampout0;
            }
            let fresh43 = ptr_scratch;
            ptr_scratch = ptr_scratch.offset(1);
            ampout1 = (x as core::ffi::c_int + *fresh43 as core::ffi::c_int) as WORD32;
            ampout1 = *ixheaacd_pow_table_Q13.offset(ampout1 as isize);
            if x != 0 {
                if temp_word as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0
                {
                    ampout1 = -ampout1;
                }
                temp_word = temp_word << 1 as core::ffi::c_int;
                bit_pos += 1;
            } else {
                ampout1 = -ampout1;
            }
            let fresh44 = ptr_scratch;
            ptr_scratch = ptr_scratch.offset(1);
            ampout2 = (y as core::ffi::c_int + *fresh44 as core::ffi::c_int) as WORD32;
            ampout2 = *ixheaacd_pow_table_Q13.offset(ampout2 as isize);
            if y != 0 {
                if temp_word as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0
                {
                    ampout2 = -ampout2;
                }
                temp_word = temp_word << 1 as core::ffi::c_int;
                bit_pos += 1;
            } else {
                ampout2 = -ampout2;
            }
            let fresh45 = ptr_scratch;
            ptr_scratch = ptr_scratch.offset(1);
            ampout3 = (z as core::ffi::c_int + *fresh45 as core::ffi::c_int) as WORD32;
            ampout3 = *ixheaacd_pow_table_Q13.offset(ampout3 as isize);
            if z != 0 {
                if temp_word as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0
                {
                    ampout3 = -ampout3;
                }
                temp_word = temp_word << 1 as core::ffi::c_int;
                bit_pos += 1;
            } else {
                ampout3 = -ampout3;
            }
            let fresh46 = x_invquant;
            x_invquant = x_invquant.offset(1);
            *fresh46 = ampout0;
            let fresh47 = x_invquant;
            x_invquant = x_invquant.offset(1);
            *fresh47 = ampout1;
            let fresh48 = x_invquant;
            x_invquant = x_invquant.offset(1);
            *fresh48 = ampout2;
            let fresh49 = x_invquant;
            x_invquant = x_invquant.offset(1);
            *fresh49 = ampout3;
        } else {
            let mut w_0: WORD32 = 0;
            let mut x_0: WORD32 = 0;
            let mut y_0: WORD32 = 0;
            let mut z_0: WORD32 = 0;
            let fresh50 = ptr_scratch;
            ptr_scratch = ptr_scratch.offset(1);
            ampres = *fresh50 as WORD32;
            let fresh51 = ptr_scratch;
            ptr_scratch = ptr_scratch.offset(1);
            ampres1 = *fresh51 as WORD32;
            let fresh52 = ptr_scratch;
            ptr_scratch = ptr_scratch.offset(1);
            ampres2 = *fresh52 as WORD32;
            let fresh53 = ptr_scratch;
            ptr_scratch = ptr_scratch.offset(1);
            ampres3 = *fresh53 as WORD32;
            w_0 = (index as core::ffi::c_int / 27 as core::ffi::c_int
                - 1 as core::ffi::c_int) as WORD32;
            index = (index as core::ffi::c_int
                - (w_0 as core::ffi::c_int + 1 as core::ffi::c_int)
                    * 27 as core::ffi::c_int) as WORD16;
            x_0 = (index as core::ffi::c_int / 9 as core::ffi::c_int
                - 1 as core::ffi::c_int) as WORD32;
            index = (index as core::ffi::c_int
                - (x_0 as core::ffi::c_int + 1 as core::ffi::c_int)
                    * 9 as core::ffi::c_int) as WORD16;
            y_0 = (index as core::ffi::c_int / 3 as core::ffi::c_int
                - 1 as core::ffi::c_int) as WORD32;
            z_0 = (index as core::ffi::c_int
                - (y_0 as core::ffi::c_int + 1 as core::ffi::c_int)
                    * 3 as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
            if w_0 <= 0 as core::ffi::c_int {
                ampres = ampres - w_0;
                ampres = *ixheaacd_pow_table_Q13.offset(ampres as isize);
                ampres = -ampres;
            } else {
                ampres += w_0;
                ampres = *ixheaacd_pow_table_Q13.offset(ampres as isize);
            }
            if x_0 <= 0 as core::ffi::c_int {
                ampres1 = ampres1 - x_0;
                ampres1 = *ixheaacd_pow_table_Q13.offset(ampres1 as isize);
                ampres1 = -ampres1;
            } else {
                ampres1 += x_0;
                ampres1 = *ixheaacd_pow_table_Q13.offset(ampres1 as isize);
            }
            if y_0 <= 0 as core::ffi::c_int {
                ampres2 = ampres2 - y_0;
                ampres2 = *ixheaacd_pow_table_Q13.offset(ampres2 as isize);
                ampres2 = -ampres2;
            } else {
                ampres2 += y_0;
                ampres2 = *ixheaacd_pow_table_Q13.offset(ampres2 as isize);
            }
            if z_0 <= 0 as core::ffi::c_int {
                ampres3 = ampres3 - z_0;
                ampres3 = *ixheaacd_pow_table_Q13.offset(ampres3 as isize);
                ampres3 = -ampres3;
            } else {
                ampres3 += z_0;
                ampres3 = *ixheaacd_pow_table_Q13.offset(ampres3 as isize);
            }
            let fresh54 = x_invquant;
            x_invquant = x_invquant.offset(1);
            *fresh54 = ampres;
            let fresh55 = x_invquant;
            x_invquant = x_invquant.offset(1);
            *fresh55 = ampres1;
            let fresh56 = x_invquant;
            x_invquant = x_invquant.offset(1);
            *fresh56 = ampres2;
            let fresh57 = x_invquant;
            x_invquant = x_invquant.offset(1);
            *fresh57 = ampres3;
        }
        ixheaacd_aac_read_byte_corr(
            &mut ptr_read_next,
            &mut bit_pos,
            &mut read_word,
            (*it_bit_buff).ptr_bit_buf_end,
        );
        idx -= 4 as core::ffi::c_int;
    }
    ptr_read_next = ptr_read_next.offset(-(increment as isize));
    ixheaacd_aac_read_byte_corr1(
        &mut ptr_read_next,
        &mut bit_pos,
        &mut read_word,
        (*it_bit_buff).ptr_bit_buf_end,
    );
    (*it_bit_buff).ptr_read_next = ptr_read_next;
    (*it_bit_buff).bit_pos = bit_pos;
    return 0 as WORD;
}
#[inline]
unsafe extern "C" fn ixheaacd_huffman_dec_pair(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut spec_coef: *mut WORD32,
    mut offsets: *mut WORD16,
    mut no_bands: WORD,
    mut group_len: WORD,
    mut code_book_tbl: *const UWORD16,
    mut ixheaacd_pow_table_Q13: *mut WORD32,
    mut tbl_sign: WORD32,
    mut idx_table: *const UWORD32,
    mut huff_mode: WORD32,
    mut maximum_bins_short: WORD32,
) -> WORD {
    let mut idx: WORD = 0;
    let mut grp_idx: WORD = 0;
    let mut len_idx: WORD = 0;
    let mut index: WORD16 = 0;
    let mut length: WORD16 = 0;
    let mut y: WORD32 = 0;
    let mut z: WORD32 = 0;
    let mut spec_orig: *mut WORD32 = spec_coef;
    let mut ptr_read_next: *mut UWORD8 = (*it_bit_buff).ptr_read_next;
    let mut bit_pos: WORD32 = (*it_bit_buff).bit_pos;
    let mut read_word: WORD32 = 0;
    let mut increment: WORD32 = 0;
    read_word = ixheaacd_aac_showbits_32(
        ptr_read_next,
        (*it_bit_buff).cnt_bits,
        &mut increment,
    ) as WORD32;
    ptr_read_next = ptr_read_next.offset(increment as isize);
    loop {
        len_idx = (*offsets.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
            - *offsets.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
            as WORD;
        grp_idx = group_len;
        loop {
            spec_coef = spec_coef
                .offset(
                    *offsets.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                        as isize,
                );
            idx = len_idx;
            loop {
                let mut read_word1: UWORD32 = 0;
                read_word1 = (read_word << bit_pos) as UWORD32;
                ixheaacd_huffman_decode(
                    read_word1 as WORD32,
                    &mut index,
                    &mut length,
                    code_book_tbl,
                    idx_table,
                );
                bit_pos += length as core::ffi::c_int;
                ixheaacd_aac_read_byte_corr(
                    &mut ptr_read_next,
                    &mut bit_pos,
                    &mut read_word,
                    (*it_bit_buff).ptr_bit_buf_end,
                );
                if tbl_sign != 0 {
                    let mut temp_word: WORD32 = 0;
                    temp_word = read_word << bit_pos;
                    y = index as WORD32 / huff_mode;
                    z = index as WORD32 - huff_mode * y;
                    if y != 0 {
                        y = *ixheaacd_pow_table_Q13.offset(y as isize);
                        if temp_word as core::ffi::c_uint
                            & 0x80000000 as core::ffi::c_uint != 0
                        {
                            y = -y;
                        }
                        temp_word = temp_word << 1 as core::ffi::c_int;
                        bit_pos += 1;
                    }
                    let fresh14 = spec_coef;
                    spec_coef = spec_coef.offset(1);
                    *fresh14 = y;
                    if z != 0 {
                        z = *ixheaacd_pow_table_Q13.offset(z as isize);
                        if temp_word as core::ffi::c_uint
                            & 0x80000000 as core::ffi::c_uint != 0
                        {
                            z = -z;
                        }
                        temp_word <<= 1 as core::ffi::c_int;
                        bit_pos += 1;
                    }
                    let fresh15 = spec_coef;
                    spec_coef = spec_coef.offset(1);
                    *fresh15 = z;
                } else {
                    y = (index as core::ffi::c_int / huff_mode as core::ffi::c_int
                        - 4 as core::ffi::c_int) as WORD32;
                    z = (index as core::ffi::c_int
                        - (y as core::ffi::c_int + 4 as core::ffi::c_int)
                            * huff_mode as core::ffi::c_int - 4 as core::ffi::c_int)
                        as WORD32;
                    if y < 0 as core::ffi::c_int {
                        y = -y;
                        y = *ixheaacd_pow_table_Q13.offset(y as isize);
                        y = -y;
                    } else {
                        y = *ixheaacd_pow_table_Q13.offset(y as isize);
                    }
                    if z < 0 as core::ffi::c_int {
                        z = -z;
                        z = *ixheaacd_pow_table_Q13.offset(z as isize);
                        z = -z;
                    } else {
                        z = *ixheaacd_pow_table_Q13.offset(z as isize);
                    }
                    let fresh16 = spec_coef;
                    spec_coef = spec_coef.offset(1);
                    *fresh16 = y;
                    let fresh17 = spec_coef;
                    spec_coef = spec_coef.offset(1);
                    *fresh17 = z;
                }
                ixheaacd_aac_read_byte_corr(
                    &mut ptr_read_next,
                    &mut bit_pos,
                    &mut read_word,
                    (*it_bit_buff).ptr_bit_buf_end,
                );
                idx -= 2 as core::ffi::c_int;
                if !(idx != 0 as core::ffi::c_int) {
                    break;
                }
            }
            if maximum_bins_short == 120 as core::ffi::c_int {
                spec_coef = spec_coef
                    .offset(
                        (maximum_bins_short as core::ffi::c_int
                            - *offsets.offset(1 as core::ffi::c_int as isize)
                                as core::ffi::c_int) as isize,
                    );
            } else {
                spec_coef = spec_coef
                    .offset(
                        (MAX_BINS_SHORT
                            - *offsets.offset(1 as core::ffi::c_int as isize)
                                as core::ffi::c_int) as isize,
                    );
            }
            grp_idx -= 1;
            if !(grp_idx != 0 as core::ffi::c_int) {
                break;
            }
        }
        offsets = offsets.offset(1);
        spec_coef = spec_orig;
        no_bands -= 1;
        if !(no_bands >= 0 as core::ffi::c_int) {
            break;
        }
    }
    ptr_read_next = ptr_read_next.offset(-(increment as isize));
    ixheaacd_aac_read_byte_corr1(
        &mut ptr_read_next,
        &mut bit_pos,
        &mut read_word,
        (*it_bit_buff).ptr_bit_buf_end,
    );
    (*it_bit_buff).ptr_read_next = ptr_read_next;
    (*it_bit_buff).bit_pos = bit_pos;
    return 0 as WORD;
}
#[inline]
unsafe extern "C" fn ixheaacd_huffman_dec_word2_pair(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut width: WORD32,
    mut code_book_tbl: *const UWORD16,
    mut x_invquant: *mut WORD32,
    mut ixheaacd_pow_table_Q13: *mut WORD32,
    mut ptr_scratch: *mut WORD8,
    mut tbl_sign: WORD32,
    mut idx_table: *const UWORD32,
    mut huff_mode: WORD32,
) -> WORD {
    let mut ampres: WORD32 = 0;
    let mut idx: WORD = 0;
    let mut index: WORD16 = 0;
    let mut length: WORD16 = 0;
    let mut ptr_read_next: *mut UWORD8 = (*it_bit_buff).ptr_read_next;
    let mut bit_pos: WORD32 = (*it_bit_buff).bit_pos;
    let mut read_word: WORD32 = 0;
    let mut increment: WORD32 = 0;
    read_word = ixheaacd_aac_showbits_32(
        ptr_read_next,
        (*it_bit_buff).cnt_bits,
        &mut increment,
    ) as WORD32;
    ptr_read_next = ptr_read_next.offset(increment as isize);
    idx = width as WORD;
    while idx != 0 as core::ffi::c_int {
        let mut read_word1: UWORD32 = 0;
        read_word1 = (read_word << bit_pos) as UWORD32;
        ixheaacd_huffman_decode(
            read_word1 as WORD32,
            &mut index,
            &mut length,
            code_book_tbl,
            idx_table,
        );
        bit_pos += length as core::ffi::c_int;
        ixheaacd_aac_read_byte_corr(
            &mut ptr_read_next,
            &mut bit_pos,
            &mut read_word,
            (*it_bit_buff).ptr_bit_buf_end,
        );
        if tbl_sign != 0 {
            let mut out0: WORD32 = 0;
            let mut out1: WORD32 = 0;
            let mut temp_word: WORD32 = 0;
            let mut ampout0: WORD32 = 0;
            let mut ampout1: WORD32 = 0;
            let fresh32 = ptr_scratch;
            ptr_scratch = ptr_scratch.offset(1);
            ampout0 = *fresh32 as WORD32;
            let fresh33 = ptr_scratch;
            ptr_scratch = ptr_scratch.offset(1);
            ampout1 = *fresh33 as WORD32;
            out0 = index as WORD32 / huff_mode;
            out1 = index as WORD32 - huff_mode * out0;
            ampout0 += out0;
            ampout0 = *ixheaacd_pow_table_Q13.offset(ampout0 as isize);
            ampout1 += out1;
            ampout1 = *ixheaacd_pow_table_Q13.offset(ampout1 as isize);
            temp_word = read_word << bit_pos;
            if out0 != 0 {
                if temp_word as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0
                {
                    ampout0 = -ampout0;
                }
                bit_pos += 1;
                temp_word = temp_word << 1 as core::ffi::c_int;
            } else {
                ampout0 = -ampout0;
            }
            if out1 != 0 {
                if temp_word as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0
                {
                    ampout1 = -ampout1;
                }
                bit_pos += 1;
            } else {
                ampout1 = -ampout1;
            }
            ixheaacd_aac_read_byte_corr(
                &mut ptr_read_next,
                &mut bit_pos,
                &mut read_word,
                (*it_bit_buff).ptr_bit_buf_end,
            );
            let fresh34 = x_invquant;
            x_invquant = x_invquant.offset(1);
            *fresh34 = ampout0;
            let fresh35 = x_invquant;
            x_invquant = x_invquant.offset(1);
            *fresh35 = ampout1;
        } else {
            let mut y: WORD32 = 0;
            let mut z: WORD32 = 0;
            y = (index as core::ffi::c_int / huff_mode as core::ffi::c_int
                - 4 as core::ffi::c_int) as WORD32;
            z = (index as core::ffi::c_int
                - (y as core::ffi::c_int + 4 as core::ffi::c_int)
                    * huff_mode as core::ffi::c_int - 4 as core::ffi::c_int) as WORD32;
            let fresh36 = ptr_scratch;
            ptr_scratch = ptr_scratch.offset(1);
            ampres = *fresh36 as WORD32;
            if y <= 0 as core::ffi::c_int {
                ampres = ampres - y;
                ampres = *ixheaacd_pow_table_Q13.offset(ampres as isize);
                let fresh37 = x_invquant;
                x_invquant = x_invquant.offset(1);
                *fresh37 = -ampres;
            } else {
                ampres += y;
                let fresh38 = x_invquant;
                x_invquant = x_invquant.offset(1);
                *fresh38 = *ixheaacd_pow_table_Q13.offset(ampres as isize);
            }
            let fresh39 = ptr_scratch;
            ptr_scratch = ptr_scratch.offset(1);
            ampres = *fresh39 as WORD32;
            if z <= 0 as core::ffi::c_int {
                ampres = ampres - z;
                ampres = *ixheaacd_pow_table_Q13.offset(ampres as isize);
                let fresh40 = x_invquant;
                x_invquant = x_invquant.offset(1);
                *fresh40 = -ampres;
            } else {
                ampres += z;
                let fresh41 = x_invquant;
                x_invquant = x_invquant.offset(1);
                *fresh41 = *ixheaacd_pow_table_Q13.offset(ampres as isize);
            }
        }
        ixheaacd_aac_read_byte_corr(
            &mut ptr_read_next,
            &mut bit_pos,
            &mut read_word,
            (*it_bit_buff).ptr_bit_buf_end,
        );
        idx -= 2 as core::ffi::c_int;
    }
    ptr_read_next = ptr_read_next.offset(-(increment as isize));
    ixheaacd_aac_read_byte_corr1(
        &mut ptr_read_next,
        &mut bit_pos,
        &mut read_word,
        (*it_bit_buff).ptr_bit_buf_end,
    );
    (*it_bit_buff).ptr_read_next = ptr_read_next;
    (*it_bit_buff).bit_pos = bit_pos;
    return 0 as WORD;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_decode_huffman(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut cb_no: WORD32,
    mut spec_coef: *mut WORD32,
    mut sfb_offset: *mut WORD16,
    mut start: WORD,
    mut sfb: WORD,
    mut group_len: WORD,
    mut ptr_aac_tables: *mut ia_aac_dec_tables_struct,
    mut maximum_bins_short: WORD32,
) -> WORD {
    let mut ret_val: WORD = 0 as WORD;
    let mut start_bit_pos: WORD = (*it_bit_buff).bit_pos as WORD;
    let mut start_read_pos: *mut UWORD8 = (*it_bit_buff).ptr_read_next;
    let mut cb_table: *const UWORD16 = (*ptr_aac_tables).code_book[cb_no as usize];
    let mut huff_mode: WORD32 = 0;
    let mut idx_table: *const UWORD32 = (*ptr_aac_tables).index_table[cb_no as usize];
    let mut pow_table: *mut WORD32 = ((*(*ptr_aac_tables).pstr_block_tables)
        .ixheaacd_pow_table_Q13)
        .as_mut_ptr();
    let mut no_bands: WORD32 = sfb as WORD32 - start as WORD32 - 1 as WORD32;
    let mut band_offset: *mut WORD16 = sfb_offset.offset(start as isize);
    if cb_no == 11 as core::ffi::c_int {
        let mut idx_table_0: *const UWORD32 = ((*(*ptr_aac_tables).pstr_huffmann_tables)
            .idx_table_hf11)
            .as_mut_ptr();
        let mut cb_table_0: *const UWORD16 = ((*(*ptr_aac_tables).pstr_huffmann_tables)
            .input_table_cb11)
            .as_mut_ptr();
        ret_val = ixheaacd_huffman_dec_word1(
            it_bit_buff,
            spec_coef,
            band_offset,
            no_bands as WORD,
            group_len,
            cb_table_0,
            pow_table,
            idx_table_0,
            maximum_bins_short,
        );
    } else if cb_no <= 4 as core::ffi::c_int {
        let mut tbl_sign: WORD32 = 0 as WORD32;
        if cb_no > 2 as core::ffi::c_int {
            tbl_sign = 1 as core::ffi::c_int as WORD32;
        }
        ret_val = ixheaacd_huffman_dec_quad(
            it_bit_buff,
            spec_coef,
            band_offset,
            no_bands as WORD,
            group_len,
            cb_table,
            pow_table,
            tbl_sign,
            idx_table,
            maximum_bins_short,
        );
    } else if cb_no <= 10 as core::ffi::c_int {
        let mut tbl_sign_0: WORD32 = 0 as WORD32;
        huff_mode = 9 as core::ffi::c_int as WORD32;
        if cb_no > 6 as core::ffi::c_int {
            if cb_no > 8 as core::ffi::c_int {
                huff_mode = 13 as core::ffi::c_int as WORD32;
            } else {
                huff_mode = 8 as core::ffi::c_int as WORD32;
            }
            tbl_sign_0 = 1 as core::ffi::c_int as WORD32;
        }
        ret_val = ixheaacd_huffman_dec_pair(
            it_bit_buff,
            spec_coef,
            band_offset,
            no_bands as WORD,
            group_len,
            cb_table,
            pow_table,
            tbl_sign_0,
            idx_table,
            huff_mode,
            maximum_bins_short,
        );
    }
    let mut bits_cons: WORD = 0;
    bits_cons = (((((*it_bit_buff).ptr_read_next).offset_from(start_read_pos)
        as core::ffi::c_long) << 3 as core::ffi::c_int)
        + ((*it_bit_buff).bit_pos as WORD - start_bit_pos) as core::ffi::c_long) as WORD;
    (*it_bit_buff).cnt_bits -= bits_cons as core::ffi::c_int;
    return ret_val;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_huffman_dec_word2(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut cb_no: WORD32,
    mut width: WORD32,
    mut ptr_aac_tables: *mut ia_aac_dec_tables_struct,
    mut x_invquant: *mut WORD32,
    mut scratch_ptr: *mut WORD8,
) -> WORD {
    let mut ret_val: WORD = 0 as WORD;
    let mut huff_mode: WORD32 = 0;
    let mut start_bit_pos: WORD = (*it_bit_buff).bit_pos as WORD;
    let mut cnt_bits: WORD32 = (*it_bit_buff).cnt_bits;
    let mut pow_table: *mut WORD32 = ((*(*ptr_aac_tables).pstr_block_tables)
        .ixheaacd_pow_table_Q13)
        .as_mut_ptr();
    let mut start_read_pos: *mut UWORD8 = (*it_bit_buff).ptr_read_next;
    let mut cb_table: *const UWORD16 = (*ptr_aac_tables).code_book[cb_no as usize];
    let mut idx_table: *const UWORD32 = (*ptr_aac_tables).index_table[cb_no as usize];
    if cb_no == 11 as core::ffi::c_int {
        let mut cb_table_0: *const UWORD16 = ((*(*ptr_aac_tables).pstr_huffmann_tables)
            .input_table_cb11)
            .as_mut_ptr();
        ret_val = ixheaacd_huffman_dec_word2_11(
            it_bit_buff,
            width,
            cb_table_0,
            x_invquant,
            pow_table,
            scratch_ptr,
            ((*(*ptr_aac_tables).pstr_huffmann_tables).idx_table_hf11).as_mut_ptr(),
        );
    } else if cb_no <= 4 as core::ffi::c_int {
        let mut tbl_sign: WORD32 = 0 as WORD32;
        if cb_no > 2 as core::ffi::c_int {
            tbl_sign = 1 as core::ffi::c_int as WORD32;
        }
        ret_val = ixheaacd_huffman_dec_word2_quad(
            it_bit_buff,
            width,
            cb_table,
            x_invquant,
            pow_table,
            scratch_ptr,
            tbl_sign,
            idx_table,
        );
    } else if cb_no <= 10 as core::ffi::c_int {
        let mut tbl_sign_0: WORD32 = 0 as WORD32;
        huff_mode = 9 as core::ffi::c_int as WORD32;
        if cb_no > 6 as core::ffi::c_int {
            if cb_no > 8 as core::ffi::c_int {
                huff_mode = 13 as core::ffi::c_int as WORD32;
            } else {
                huff_mode = 8 as core::ffi::c_int as WORD32;
            }
            tbl_sign_0 = 1 as core::ffi::c_int as WORD32;
        }
        ret_val = ixheaacd_huffman_dec_word2_pair(
            it_bit_buff,
            width,
            cb_table,
            x_invquant,
            pow_table,
            scratch_ptr,
            tbl_sign_0,
            idx_table,
            huff_mode,
        );
    }
    let mut bits_cons: WORD = 0;
    if (*it_bit_buff).bit_pos <= 7 as core::ffi::c_int {
        bits_cons = (((((*it_bit_buff).ptr_read_next).offset_from(start_read_pos)
            as core::ffi::c_long) << 3 as core::ffi::c_int)
            + ((*it_bit_buff).bit_pos as WORD - start_bit_pos) as core::ffi::c_long)
            as WORD;
        if bits_cons > cnt_bits {
            return IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES;
        }
        (*it_bit_buff).cnt_bits = (cnt_bits as WORD - bits_cons) as WORD32;
    } else {
        (*it_bit_buff).ptr_read_next = ((*it_bit_buff).ptr_read_next)
            .offset(((*it_bit_buff).bit_pos >> 3 as core::ffi::c_int) as isize);
        (*it_bit_buff).bit_pos = ((*it_bit_buff).bit_pos as core::ffi::c_int
            & 0x7 as core::ffi::c_int) as WORD32;
        if (*it_bit_buff).ptr_read_next as size_t
            > ((*it_bit_buff).ptr_bit_buf_end).offset(1 as core::ffi::c_int as isize)
                as size_t
        {
            (*it_bit_buff).ptr_read_next = ((*it_bit_buff).ptr_bit_buf_end)
                .offset(1 as core::ffi::c_int as isize);
            return IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES;
        }
        bits_cons = (((((*it_bit_buff).ptr_read_next).offset_from(start_read_pos)
            as core::ffi::c_long) << 3 as core::ffi::c_int)
            + ((*it_bit_buff).bit_pos as WORD - start_bit_pos) as core::ffi::c_long)
            as WORD;
        (*it_bit_buff).cnt_bits = (cnt_bits as WORD - bits_cons) as WORD32;
    }
    return ret_val;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_dec_copy_outsample(
    mut out_samples: *mut WORD32,
    mut p_overlap_buffer: *mut WORD32,
    mut size: WORD32,
    mut stride: WORD16,
) -> VOID {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < size {
        *out_samples.offset((stride as WORD32 * i) as isize) = ((ixheaac_shl16_sat(
            *p_overlap_buffer.offset(i as isize) as WORD16,
            1 as WORD16,
        ) as core::ffi::c_int) << 14 as core::ffi::c_int) as WORD32;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_lap1_512_480(
    mut coef: *mut WORD32,
    mut prev: *mut WORD32,
    mut out_tmp: *mut core::ffi::c_void,
    mut window: *const WORD16,
    mut q_shift: WORD16,
    mut size: WORD16,
    mut stride: WORD16,
    mut slot_element: WORD,
) -> VOID {
    let mut accu: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut rounding_fac: WORD16 = -(0x2000 as core::ffi::c_int) as WORD16;
    let mut window_i: *mut WORD32 = window as *mut WORD32;
    let mut ptr_out1: *mut WORD16 = 0 as *mut WORD16;
    let mut ptr_out2: *mut WORD16 = 0 as *mut WORD16;
    let mut pwin1: *mut WORD32 = 0 as *mut WORD32;
    let mut pwin2: *mut WORD32 = 0 as *mut WORD32;
    let mut pCoef: *mut WORD32 = &mut *coef
        .offset(
            (size as core::ffi::c_int * 2 as core::ffi::c_int - 1 as core::ffi::c_int
                - 0 as core::ffi::c_int) as isize,
        ) as *mut WORD32;
    let mut out: *mut WORD16 = (out_tmp as *mut WORD16).offset(-(slot_element as isize));
    pwin1 = &mut *window_i
        .offset(
            (size as core::ffi::c_int - 1 as core::ffi::c_int - 0 as core::ffi::c_int)
                as isize,
        ) as *mut WORD32;
    pwin2 = &mut *window_i
        .offset((size as core::ffi::c_int + 0 as core::ffi::c_int) as isize)
        as *mut WORD32;
    ptr_out1 = &mut *out
        .offset(
            (stride as core::ffi::c_int
                * (size as core::ffi::c_int - 1 as core::ffi::c_int
                    - 0 as core::ffi::c_int)) as isize,
        ) as *mut WORD16;
    ptr_out2 = &mut *out
        .offset(
            (stride as core::ffi::c_int
                * (size as core::ffi::c_int + 0 as core::ffi::c_int)) as isize,
        ) as *mut WORD16;
    i = 0 as core::ffi::c_int as WORD32;
    while i < size as core::ffi::c_int {
        let mut win1: WORD32 = 0;
        let mut win2: WORD32 = 0;
        let mut coeff: WORD32 = 0;
        let fresh68 = prev;
        prev = prev.offset(1);
        let mut prev_data: WORD32 = *fresh68;
        let fresh69 = pwin1;
        pwin1 = pwin1.offset(-1);
        win1 = *fresh69;
        let fresh70 = pCoef;
        pCoef = pCoef.offset(-1);
        coeff = *fresh70;
        let fresh71 = pwin2;
        pwin2 = pwin2.offset(1);
        win2 = *fresh71;
        accu = ixheaac_sub32_sat(
            ixheaac_shl32_dir_sat_limit(
                ixheaac_mult32_shl(coeff, win1),
                q_shift as WORD,
            ),
            ixheaac_mac32x16in32_shl(rounding_fac as WORD32, win2, prev_data as WORD16),
        );
        accu = ixheaac_add32_sat(accu, accu);
        accu = ixheaac_add32_sat(accu, accu);
        *ptr_out1 = ixheaac_shr32(accu, 16 as WORD) as WORD16;
        ptr_out1 = ptr_out1.offset(-(stride as core::ffi::c_int as isize));
        accu = ixheaac_sub32_sat(
            ixheaac_shl32_dir_sat_limit(
                ixheaac_mult32_shl(ixheaac_negate32_sat(coeff), win2),
                q_shift as WORD,
            ),
            ixheaac_mac32x16in32_shl(rounding_fac as WORD32, win1, prev_data as WORD16),
        );
        accu = ixheaac_add32_sat(accu, accu);
        accu = ixheaac_add32_sat(accu, accu);
        *ptr_out2 = ixheaac_shr32(accu, 16 as WORD) as WORD16;
        ptr_out2 = ptr_out2.offset(stride as core::ffi::c_int as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_over_lap_add1_dec(
    mut coef: *mut WORD32,
    mut prev: *mut WORD32,
    mut out: *mut WORD32,
    mut window: *const WORD16,
    mut q_shift: WORD16,
    mut size: WORD16,
    mut ch_fac: WORD16,
) -> VOID {
    let mut accu: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut rounding_fac: WORD16 = 0 as WORD16;
    i = 0 as core::ffi::c_int as WORD32;
    while i < size as core::ffi::c_int {
        let mut window1: WORD16 = 0;
        let mut window2: WORD16 = 0;
        window1 = *window
            .offset(
                (2 as core::ffi::c_int * size as core::ffi::c_int
                    - 2 as core::ffi::c_int * i as core::ffi::c_int
                    - 1 as core::ffi::c_int) as isize,
            );
        window2 = *window
            .offset(
                (2 as core::ffi::c_int * size as core::ffi::c_int
                    - 2 as core::ffi::c_int * i as core::ffi::c_int
                    - 2 as core::ffi::c_int) as isize,
            );
        accu = ixheaac_sub32_sat(
            ixheaac_shl32_dir_sat_limit(
                ixheaac_mult32x16in32(
                    *coef
                        .offset(
                            (size as WORD32 * 2 as WORD32 - 1 as WORD32 - i) as isize,
                        ),
                    window2,
                ),
                q_shift as WORD,
            ),
            ixheaacd_mac32x16in32_sat(
                rounding_fac as WORD32,
                *prev.offset(i as isize),
                window1,
            ),
        );
        *out
            .offset(
                (ch_fac as core::ffi::c_int
                    * (size as core::ffi::c_int - i as core::ffi::c_int
                        - 1 as core::ffi::c_int)) as isize,
            ) = accu;
        accu = ixheaac_sub32_sat(
            ixheaac_shl32_dir_sat_limit(
                ixheaac_mult32x16in32(
                    ixheaac_negate32_sat(
                        *coef
                            .offset(
                                (size as WORD32 * 2 as WORD32 - 1 as WORD32 - i) as isize,
                            ),
                    ),
                    window1,
                ),
                q_shift as WORD,
            ),
            ixheaacd_mac32x16in32_sat(
                rounding_fac as WORD32,
                *prev.offset(i as isize),
                window2,
            ),
        );
        *out.offset((ch_fac as WORD32 * (size as WORD32 + i)) as isize) = accu;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_over_lap_add2_dec(
    mut coef: *mut WORD32,
    mut prev: *mut WORD32,
    mut out: *mut WORD32,
    mut window: *const WORD16,
    mut q_shift: WORD16,
    mut size: WORD16,
    mut ch_fac: WORD16,
) -> VOID {
    let mut accu: WORD32 = 0;
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < size as core::ffi::c_int {
        accu = ixheaac_sub32_sat(
            ixheaac_mult32x16in32(
                *coef.offset((size as WORD32 + i) as isize),
                *window.offset((2 as WORD32 * i) as isize),
            ),
            ixheaac_mult32x16in32(
                *prev.offset((size as WORD32 - 1 as WORD32 - i) as isize),
                *window
                    .offset(
                        (2 as core::ffi::c_int * i as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    ),
            ),
        );
        *out.offset((ch_fac as WORD32 * i) as isize) = ixheaac_shr32_sat(
            accu,
            16 as WORD32 - (q_shift as WORD32 + 1 as WORD32),
        );
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < size as core::ffi::c_int {
        accu = ixheaac_sub32_sat(
            ixheaac_mult32x16in32(
                ixheaac_negate32_sat(
                    *coef
                        .offset(
                            (size as WORD32 * 2 as WORD32 - 1 as WORD32 - i) as isize,
                        ),
                ),
                *window
                    .offset(
                        (2 as core::ffi::c_int * size as core::ffi::c_int
                            - 2 as core::ffi::c_int * i as core::ffi::c_int
                            - 1 as core::ffi::c_int) as isize,
                    ),
            ),
            ixheaac_mult32x16in32(
                *prev.offset(i as isize),
                *window
                    .offset(
                        (2 as core::ffi::c_int * size as core::ffi::c_int
                            - 2 as core::ffi::c_int * i as core::ffi::c_int
                            - 2 as core::ffi::c_int) as isize,
                    ),
            ),
        );
        *out
            .offset(
                (ch_fac as core::ffi::c_int
                    * (i as core::ffi::c_int + size as core::ffi::c_int)) as isize,
            ) = ixheaac_shr32_sat(
            accu,
            16 as WORD32 - (q_shift as WORD32 + 1 as WORD32),
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_process_single_scf(
    mut scale_factor: WORD32,
    mut x_invquant: *mut WORD32,
    mut width: WORD32,
    mut ptr_scale_table: *mut WORD32,
    mut total_channels: WORD32,
    mut object_type: WORD32,
    mut aac_sf_data_resil_flag: WORD32,
) -> VOID {
    let mut j: WORD32 = 0;
    let mut temp1: WORD32 = 0;
    let mut q_factor: WORD32 = 0;
    let mut buffer1: WORD32 = 0;
    let mut scale_short: WORD16 = 0;
    object_type = 0 as core::ffi::c_int as WORD32;
    aac_sf_data_resil_flag = 0 as core::ffi::c_int as WORD32;
    if scale_factor < 24 as core::ffi::c_int {
        j = width;
        while j > 0 as core::ffi::c_int {
            let fresh3 = x_invquant;
            x_invquant = x_invquant.offset(1);
            *fresh3 = 0 as core::ffi::c_int as WORD32;
            j -= 1;
        }
    } else {
        let mut shift: WORD32 = 0;
        if total_channels > 2 as core::ffi::c_int {
            q_factor = 34 as WORD32 - (scale_factor >> 2 as core::ffi::c_int);
        } else {
            q_factor = 37 as WORD32 - (scale_factor >> 2 as core::ffi::c_int);
        }
        scale_short = *ptr_scale_table
            .offset(
                (scale_factor as core::ffi::c_int & 0x3 as core::ffi::c_int) as isize,
            ) as WORD16;
        shift = q_factor;
        if shift > 0 as core::ffi::c_int {
            if scale_short as core::ffi::c_int
                == 0x8000 as core::ffi::c_int as WORD16 as core::ffi::c_int
            {
                j = width;
                while j > 0 as core::ffi::c_int {
                    temp1 = *x_invquant;
                    buffer1 = ixheaac_mult32x16in32_shl_sat(temp1, scale_short);
                    buffer1 = ixheaac_shr32(buffer1, shift as WORD);
                    let fresh4 = x_invquant;
                    x_invquant = x_invquant.offset(1);
                    *fresh4 = buffer1;
                    j -= 1;
                }
            } else {
                j = width;
                while j > 0 as core::ffi::c_int {
                    temp1 = *x_invquant;
                    buffer1 = ixheaac_mult32x16in32_shl(temp1, scale_short);
                    buffer1 = ixheaac_shr32(buffer1, shift as WORD);
                    let fresh5 = x_invquant;
                    x_invquant = x_invquant.offset(1);
                    *fresh5 = buffer1;
                    j -= 1;
                }
            }
        } else {
            shift = -shift;
            if shift > 0 as core::ffi::c_int {
                if scale_short as core::ffi::c_int
                    == 0x8000 as core::ffi::c_int as WORD16 as core::ffi::c_int
                {
                    j = width;
                    while j > 0 as core::ffi::c_int {
                        temp1 = *x_invquant;
                        temp1 = ixheaac_shl32(temp1, shift as WORD - 1 as WORD);
                        buffer1 = ixheaac_mult32x16in32_shl_sat(temp1, scale_short);
                        buffer1 = ixheaac_shl32(buffer1, 1 as WORD);
                        let fresh6 = x_invquant;
                        x_invquant = x_invquant.offset(1);
                        *fresh6 = buffer1;
                        j -= 1;
                    }
                } else {
                    j = width;
                    while j > 0 as core::ffi::c_int {
                        temp1 = *x_invquant;
                        temp1 = ixheaac_shl32(temp1, shift as WORD - 1 as WORD);
                        buffer1 = ixheaac_mult32x16in32_shl(temp1, scale_short);
                        buffer1 = ixheaac_shl32(buffer1, 1 as WORD);
                        let fresh7 = x_invquant;
                        x_invquant = x_invquant.offset(1);
                        *fresh7 = buffer1;
                        j -= 1;
                    }
                }
            } else if scale_short as core::ffi::c_int
                == 0x8000 as core::ffi::c_int as WORD16 as core::ffi::c_int
            {
                j = width;
                while j > 0 as core::ffi::c_int {
                    temp1 = *x_invquant;
                    buffer1 = ixheaac_mult32x16in32_shl_sat(temp1, scale_short);
                    let fresh8 = x_invquant;
                    x_invquant = x_invquant.offset(1);
                    *fresh8 = buffer1;
                    j -= 1;
                }
            } else {
                j = width;
                while j > 0 as core::ffi::c_int {
                    temp1 = *x_invquant;
                    buffer1 = ixheaac_mult32x16in32_shl(temp1, scale_short);
                    let fresh9 = x_invquant;
                    x_invquant = x_invquant.offset(1);
                    *fresh9 = buffer1;
                    j -= 1;
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_scale_factor_process_dec(
    mut x_invquant: *mut WORD32,
    mut scale_fact: *mut WORD16,
    mut no_band: WORD,
    mut width: *mut WORD8,
    mut ptr_scale_table: *mut WORD32,
    mut total_channels: WORD32,
    mut object_type: WORD32,
    mut aac_sf_data_resil_flag: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut scale_factor: WORD16 = 0;
    i = (no_band as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while i >= 0 as core::ffi::c_int {
        let fresh2 = scale_fact;
        scale_fact = scale_fact.offset(1);
        scale_factor = *fresh2;
        ixheaacd_process_single_scf(
            scale_factor as WORD32,
            x_invquant,
            *width as WORD32,
            ptr_scale_table,
            total_channels,
            object_type,
            aac_sf_data_resil_flag,
        );
        x_invquant = x_invquant.offset(*width as core::ffi::c_int as isize);
        width = width.offset(1);
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_right_shift_block(
    mut p_spectrum: *mut WORD32,
    mut length: WORD32,
    mut shift_val: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut temp1: WORD32 = 0;
    let mut temp2: WORD32 = 0;
    let mut temp_ptr: *mut WORD32 = &mut *p_spectrum
        .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
    length = length >> 2 as core::ffi::c_int;
    i = (length as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while i >= 0 as core::ffi::c_int {
        temp1 = *temp_ptr;
        temp2 = *temp_ptr.offset(1 as core::ffi::c_int as isize);
        let fresh10 = temp_ptr;
        temp_ptr = temp_ptr.offset(1);
        *fresh10 = temp1 >> shift_val;
        temp1 = *temp_ptr.offset(1 as core::ffi::c_int as isize);
        let fresh11 = temp_ptr;
        temp_ptr = temp_ptr.offset(1);
        *fresh11 = temp2 >> shift_val;
        temp2 = *temp_ptr.offset(1 as core::ffi::c_int as isize);
        let fresh12 = temp_ptr;
        temp_ptr = temp_ptr.offset(1);
        *fresh12 = temp1 >> shift_val;
        let fresh13 = temp_ptr;
        temp_ptr = temp_ptr.offset(1);
        *fresh13 = temp2 >> shift_val;
        i -= 1;
    }
}
