extern "C" {
    fn ixheaacd_aac_read_byte_corr(
        ptr_read_next: *mut *mut UWORD8,
        ptr_bit_pos: *mut WORD32,
        readword: *mut WORD32,
        p_bit_buf_end: *mut UWORD8,
    ) -> VOID;
    fn ixheaacd_res_inv_quant(
        x_quant: *mut WORD32,
        pow_table_q17: *mut WORD32,
    ) -> WORD32;
    fn ixheaacd_res_aac_showbits_32(p_read_next: *mut UWORD8) -> UWORD32;
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
pub type __jmp_buf = [core::ffi::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [core::ffi::c_ulong; 16],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_res_block_tables_struct {
    pub pow_table_q17: [WORD32; 129],
    pub scale_table: [WORD32; 4],
    pub scale_table_960: [WORD32; 4],
    pub tns_max_bands_tbl: [[WORD8; 2]; 12],
    pub tns_coeff3_16: [WORD16; 8],
    pub tns_coeff4_16: [WORD16; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_res_huffmann_tables_struct {
    pub sfb_96_1024: [WORD8; 43],
    pub sfb_96_128: [WORD8; 14],
    pub sfb_64_1024: [WORD8; 49],
    pub sfb_48_1024: [WORD8; 51],
    pub sfb_48_128: [WORD8; 16],
    pub sfb_32_1024: [WORD8; 53],
    pub sfb_24_1024: [WORD8; 49],
    pub sfb_24_128: [WORD8; 17],
    pub sfb_16_1024: [WORD8; 45],
    pub sfb_16_128: [WORD8; 17],
    pub sfb_8_1024: [WORD8; 42],
    pub sfb_8_128: [WORD8; 17],
    pub sfb_96_960: [WORD8; 41],
    pub sfb_96_120: [WORD8; 13],
    pub sfb_64_960: [WORD8; 47],
    pub sfb_48_960: [WORD8; 50],
    pub sfb_48_120: [WORD8; 15],
    pub sfb_24_960: [WORD8; 47],
    pub sfb_24_120: [WORD8; 16],
    pub sfb_16_960: [WORD8; 43],
    pub sfb_16_120: [WORD8; 16],
    pub sfb_8_960: [WORD8; 41],
    pub sfb_8_120: [WORD8; 16],
    pub huffman_code_book_1: [UWORD16; 108],
    pub huffman_code_book_2: [UWORD16; 110],
    pub huffman_code_book_3: [UWORD16; 136],
    pub huffman_code_book_4: [UWORD16; 116],
    pub huffman_code_book_5: [UWORD16; 126],
    pub huffman_code_book_6: [UWORD16; 120],
    pub huffman_code_book_7: [UWORD16; 112],
    pub huffman_code_book_8: [UWORD16; 92],
    pub huffman_code_book_9: [UWORD16; 236],
    pub huffman_code_book_10: [UWORD16; 218],
    pub huffman_codebook_11: [UWORD16; 344],
    pub huffman_code_book_scl: [UWORD16; 273],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_residual_aac_tables_struct {
    pub res_block_tables_ptr: *mut ia_mps_dec_res_block_tables_struct,
    pub res_huffmann_tables_ptr: *mut ia_mps_dec_res_huffmann_tables_struct,
    pub scale_factor_bands_long: [*mut WORD8; 24],
    pub scale_factor_bands_short: [*mut WORD8; 24],
    pub sfb_index_long: *mut WORD16,
    pub sfb_index_short: *mut WORD16,
    pub sfb_index_long_width: *mut WORD8,
    pub sfb_index_short_width: *mut WORD8,
    pub code_book: [*mut UWORD16; 13],
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
pub const IQ_TABLE_SIZE_HALF: core::ffi::c_int = 128 as core::ffi::c_int;
#[inline]
unsafe extern "C" fn ixheaacd_res_exts(
    mut a: UWORD32,
    mut shift_left: WORD32,
    mut shift_right: WORD32,
) -> WORD32 {
    let mut x: WORD32 = 0;
    x = (a << shift_left) as WORD32;
    x = x >> shift_right;
    return x;
}
#[inline]
unsafe extern "C" fn ixheaacd_res_extract_symbol(
    mut value: WORD32,
    mut l_shift: WORD32,
    mut r_shift: WORD32,
    mut pow_table_q17: *mut WORD32,
) -> WORD32 {
    let mut out: WORD32 = 0;
    out = (value << l_shift >> r_shift) as WORD16 as WORD32;
    if out < 0 as core::ffi::c_int {
        out = -out;
        out = *pow_table_q17.offset(out as isize);
        out = -out;
    } else {
        out = *pow_table_q17.offset(out as isize);
    }
    return out;
}
#[inline]
unsafe extern "C" fn ixheaacd_res_extract_signed_symbol(
    mut value: WORD32,
    mut l_shift: WORD32,
    mut r_shift: WORD32,
    mut pow_table_q17: *mut WORD32,
    mut temp_word: *mut WORD32,
    mut pr_bit_pos: *mut WORD32,
) -> WORD32 {
    let mut out: WORD32 = 0;
    out = ixheaac_extu(value as UWORD32, l_shift, r_shift) as WORD32;
    if out != 0 {
        let mut bit_pos: WORD32 = *pr_bit_pos;
        out = *pow_table_q17.offset(out as isize);
        if *temp_word as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0 {
            out = -out;
        }
        *temp_word = *temp_word << 1 as core::ffi::c_int;
        bit_pos += 1;
        *pr_bit_pos = bit_pos;
    }
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_res_inverse_quant_lb(
    mut x_invquant: *mut WORD32,
    mut t_bands: WORD,
    mut pow_table_q17: *mut WORD32,
    mut pulse_data: *mut WORD8,
) -> VOID {
    let mut j: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut q_abs: WORD32 = 0;
    j = (t_bands as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while j >= 0 as core::ffi::c_int {
        let fresh0 = pulse_data;
        pulse_data = pulse_data.offset(1);
        q_abs = *fresh0 as WORD32;
        temp = *pow_table_q17.offset(q_abs as isize);
        let fresh1 = x_invquant;
        x_invquant = x_invquant.offset(1);
        *fresh1 = -temp;
        j -= 1;
    }
}
#[inline]
unsafe extern "C" fn ixheaacd_res_c_block_decode_huff_word1(
    mut it_bit_buf: *mut ia_bit_buf_struct,
    mut qp: *mut WORD32,
    mut offsets: *mut WORD16,
    mut no_bands: WORD,
    mut group_no: WORD,
    mut h_ori: *const UWORD16,
    mut pow_table_q17: *mut WORD32,
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
    let mut ptr_read_next: *mut UWORD8 = (*it_bit_buf).ptr_read_next;
    let mut bit_pos: WORD32 = (*it_bit_buf).bit_pos;
    let mut read_word: WORD32 = ixheaacd_res_aac_showbits_32(ptr_read_next) as WORD32;
    ptr_read_next = ptr_read_next.offset(4 as core::ffi::c_int as isize);
    loop {
        len_idx = (*offsets.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
            - *offsets.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
            as WORD;
        grp_idx = group_no;
        loop {
            qp = qp
                .offset(
                    *offsets.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                        as isize,
                );
            idx = len_idx;
            loop {
                let mut first_offset: UWORD16 = 0;
                let mut sign_ret_val: WORD16 = 0;
                let mut read_word1: UWORD32 = 0;
                let mut h: *mut UWORD16 = 0 as *mut UWORD16;
                read_word1 = (read_word << bit_pos) as UWORD32;
                h = h_ori as *mut UWORD16;
                h = h.offset((read_word1 >> 27 as core::ffi::c_int) as isize);
                sign_ret_val = *h as WORD16;
                first_offset = 5 as UWORD16;
                while sign_ret_val as core::ffi::c_int > 0 as core::ffi::c_int {
                    bit_pos += first_offset as core::ffi::c_int;
                    ixheaacd_aac_read_byte_corr(
                        &mut ptr_read_next,
                        &mut bit_pos,
                        &mut read_word,
                        (*it_bit_buf).ptr_bit_buf_end,
                    );
                    read_word1 = read_word1 << first_offset as core::ffi::c_int;
                    first_offset = (sign_ret_val as core::ffi::c_int
                        >> 11 as core::ffi::c_int) as UWORD16;
                    h = h
                        .offset(
                            (sign_ret_val as core::ffi::c_int
                                & 0x7ff as core::ffi::c_int) as isize,
                        );
                    h = h
                        .offset(
                            (read_word1
                                >> 32 as core::ffi::c_int
                                    - first_offset as core::ffi::c_int) as isize,
                        );
                    sign_ret_val = *h as WORD16;
                }
                bit_pos
                    += (sign_ret_val as core::ffi::c_int & 0x7fff as core::ffi::c_int)
                        >> 11 as core::ffi::c_int;
                bit_pos = (if bit_pos < 31 as core::ffi::c_int {
                    bit_pos as core::ffi::c_int
                } else {
                    31 as core::ffi::c_int
                }) as WORD32;
                value = (sign_ret_val as core::ffi::c_int & 0x7ff as core::ffi::c_int)
                    as WORD32;
                out1 = ((value as core::ffi::c_int & 0x3e0 as core::ffi::c_int)
                    >> 5 as core::ffi::c_int) as WORD32;
                out2 = (value as core::ffi::c_int & 0x1f as core::ffi::c_int) as WORD32;
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
                    (*it_bit_buf).ptr_bit_buf_end,
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
                        (*it_bit_buf).ptr_bit_buf_end,
                    );
                    off = ixheaac_extu(read_word as UWORD32, bit_pos, 32 as WORD32 - i)
                        as WORD32;
                    bit_pos += i;
                    ixheaacd_aac_read_byte_corr(
                        &mut ptr_read_next,
                        &mut bit_pos,
                        &mut read_word,
                        (*it_bit_buf).ptr_bit_buf_end,
                    );
                    ixheaacd_aac_read_byte_corr(
                        &mut ptr_read_next,
                        &mut bit_pos,
                        &mut read_word,
                        (*it_bit_buf).ptr_bit_buf_end,
                    );
                    i = off + ((1 as core::ffi::c_int) << i);
                    if i <= IQ_TABLE_SIZE_HALF {
                        i = *pow_table_q17.offset(i as isize);
                    } else {
                        err_code |= ixheaacd_res_inv_quant(&mut i, pow_table_q17);
                    }
                    if out1 < 0 as core::ffi::c_int {
                        out1 = -i;
                    } else {
                        out1 = i;
                    }
                    let fresh22 = qp;
                    qp = qp.offset(1);
                    *fresh22 = out1;
                } else if out1 <= 0 as core::ffi::c_int {
                    out1 = -out1;
                    out1 = *pow_table_q17.offset(out1 as isize);
                    let fresh23 = qp;
                    qp = qp.offset(1);
                    *fresh23 = -out1;
                } else {
                    out1 = *pow_table_q17.offset(out1 as isize);
                    let fresh24 = qp;
                    qp = qp.offset(1);
                    *fresh24 = out1;
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
                        (*it_bit_buf).ptr_bit_buf_end,
                    );
                    off = ixheaac_extu(read_word as UWORD32, bit_pos, 32 as WORD32 - i)
                        as WORD32;
                    bit_pos += i;
                    ixheaacd_aac_read_byte_corr(
                        &mut ptr_read_next,
                        &mut bit_pos,
                        &mut read_word,
                        (*it_bit_buf).ptr_bit_buf_end,
                    );
                    ixheaacd_aac_read_byte_corr(
                        &mut ptr_read_next,
                        &mut bit_pos,
                        &mut read_word,
                        (*it_bit_buf).ptr_bit_buf_end,
                    );
                    i = off + ((1 as core::ffi::c_int) << i);
                    if i <= IQ_TABLE_SIZE_HALF {
                        i = *pow_table_q17.offset(i as isize);
                    } else {
                        err_code |= ixheaacd_res_inv_quant(&mut i, pow_table_q17);
                    }
                    if out2 < 0 as core::ffi::c_int {
                        out2 = -i;
                    } else {
                        out2 = i;
                    }
                    let fresh25 = qp;
                    qp = qp.offset(1);
                    *fresh25 = out2;
                } else if out2 <= 0 as core::ffi::c_int {
                    out2 = -out2;
                    out2 = *pow_table_q17.offset(out2 as isize);
                    let fresh26 = qp;
                    qp = qp.offset(1);
                    *fresh26 = -out2;
                } else {
                    out2 = *pow_table_q17.offset(out2 as isize);
                    let fresh27 = qp;
                    qp = qp.offset(1);
                    *fresh27 = out2;
                }
                idx -= 2 as core::ffi::c_int;
                if !(idx != 0 as core::ffi::c_int) {
                    break;
                }
            }
            qp = qp
                .offset(
                    (maximum_bins_short as core::ffi::c_int
                        - *offsets.offset(1 as core::ffi::c_int as isize)
                            as core::ffi::c_int) as isize,
                );
            grp_idx -= 1;
            if !(grp_idx != 0 as core::ffi::c_int) {
                break;
            }
        }
        offsets = offsets.offset(1);
        qp = qp.offset(-((maximum_bins_short as WORD * group_no) as isize));
        no_bands -= 1;
        if !(no_bands >= 0 as core::ffi::c_int) {
            break;
        }
    }
    (*it_bit_buf).bit_pos = bit_pos;
    (*it_bit_buf).ptr_read_next = ptr_read_next
        .offset(-(4 as core::ffi::c_int as isize));
    return err_code as WORD;
}
#[inline]
unsafe extern "C" fn ixheaacd_res_c_block_decode_huff_word1_lb(
    mut it_bif_buf: *mut ia_bit_buf_struct,
    mut len: WORD32,
    mut h_ori: *const UWORD16,
    mut x_invquant: *mut WORD32,
    mut pow_table_q17: *mut WORD32,
    mut p_pul_arr: *mut WORD8,
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
    let mut ptr_read_next: *mut UWORD8 = (*it_bif_buf).ptr_read_next;
    let mut bit_pos: WORD32 = (*it_bif_buf).bit_pos;
    let mut read_word: WORD32 = ixheaacd_res_aac_showbits_32(ptr_read_next) as WORD32;
    ptr_read_next = ptr_read_next.offset(4 as core::ffi::c_int as isize);
    idx = len as WORD;
    while idx != 0 as core::ffi::c_int {
        let mut first_offset: UWORD16 = 0;
        let mut sign_ret_val: WORD16 = 0;
        let mut read_word1: UWORD32 = 0;
        let mut h: *mut UWORD16 = 0 as *mut UWORD16;
        read_word1 = (read_word << bit_pos) as UWORD32;
        h = h_ori as *mut UWORD16;
        h = h.offset((read_word1 >> 27 as core::ffi::c_int) as isize);
        sign_ret_val = *h as WORD16;
        first_offset = 5 as UWORD16;
        while sign_ret_val as core::ffi::c_int > 0 as core::ffi::c_int {
            bit_pos += first_offset as core::ffi::c_int;
            ixheaacd_aac_read_byte_corr(
                &mut ptr_read_next,
                &mut bit_pos,
                &mut read_word,
                (*it_bif_buf).ptr_bit_buf_end,
            );
            read_word1 = read_word1 << first_offset as core::ffi::c_int;
            first_offset = (sign_ret_val as core::ffi::c_int >> 11 as core::ffi::c_int)
                as UWORD16;
            h = h
                .offset(
                    (sign_ret_val as core::ffi::c_int & 0x7ff as core::ffi::c_int)
                        as isize,
                );
            h = h
                .offset(
                    (read_word1
                        >> 32 as core::ffi::c_int - first_offset as core::ffi::c_int)
                        as isize,
                );
            sign_ret_val = *h as WORD16;
        }
        bit_pos
            += (sign_ret_val as core::ffi::c_int & 0x7fff as core::ffi::c_int)
                >> 11 as core::ffi::c_int;
        bit_pos = (if bit_pos < 31 as core::ffi::c_int {
            bit_pos as core::ffi::c_int
        } else {
            31 as core::ffi::c_int
        }) as WORD32;
        value = (sign_ret_val as core::ffi::c_int & 0x7ff as core::ffi::c_int) as WORD32;
        flush_cw = read_word << bit_pos;
        out1 = ((value as core::ffi::c_int & 0x3e0 as core::ffi::c_int)
            >> 5 as core::ffi::c_int) as WORD32;
        out2 = (value as core::ffi::c_int & 0x1f as core::ffi::c_int) as WORD32;
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
            (*it_bif_buf).ptr_bit_buf_end,
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
                (*it_bif_buf).ptr_bit_buf_end,
            );
            off = ixheaac_extu(read_word as UWORD32, bit_pos, 32 as WORD32 - i)
                as WORD32;
            bit_pos += i;
            ixheaacd_aac_read_byte_corr(
                &mut ptr_read_next,
                &mut bit_pos,
                &mut read_word,
                (*it_bif_buf).ptr_bit_buf_end,
            );
            let fresh54 = p_pul_arr;
            p_pul_arr = p_pul_arr.offset(1);
            value = *fresh54 as WORD32;
            ixheaacd_aac_read_byte_corr(
                &mut ptr_read_next,
                &mut bit_pos,
                &mut read_word,
                (*it_bif_buf).ptr_bit_buf_end,
            );
            i = off + ((1 as core::ffi::c_int) << i);
            i = i + value;
            if i <= IQ_TABLE_SIZE_HALF {
                i = *pow_table_q17.offset(i as isize);
            } else {
                err_code |= ixheaacd_res_inv_quant(&mut i, pow_table_q17);
            }
            if out1 < 0 as core::ffi::c_int {
                i = -i;
            }
            let fresh55 = x_invquant;
            x_invquant = x_invquant.offset(1);
            *fresh55 = i;
        } else {
            let fresh56 = p_pul_arr;
            p_pul_arr = p_pul_arr.offset(1);
            let mut temp: WORD8 = *fresh56;
            if out1 <= 0 as core::ffi::c_int {
                out1 = temp as WORD32 - out1;
                out1 = *pow_table_q17.offset(out1 as isize);
                let fresh57 = x_invquant;
                x_invquant = x_invquant.offset(1);
                *fresh57 = -out1;
            } else {
                out1 = (out1 as core::ffi::c_int + temp as core::ffi::c_int) as WORD32;
                out1 = *pow_table_q17.offset(out1 as isize);
                let fresh58 = x_invquant;
                x_invquant = x_invquant.offset(1);
                *fresh58 = out1;
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
                (*it_bif_buf).ptr_bit_buf_end,
            );
            off = ixheaac_extu(read_word as UWORD32, bit_pos, 32 as WORD32 - i)
                as WORD32;
            bit_pos += i;
            ixheaacd_aac_read_byte_corr(
                &mut ptr_read_next,
                &mut bit_pos,
                &mut read_word,
                (*it_bif_buf).ptr_bit_buf_end,
            );
            let fresh59 = p_pul_arr;
            p_pul_arr = p_pul_arr.offset(1);
            value = *fresh59 as WORD32;
            ixheaacd_aac_read_byte_corr(
                &mut ptr_read_next,
                &mut bit_pos,
                &mut read_word,
                (*it_bif_buf).ptr_bit_buf_end,
            );
            i = off + ((1 as core::ffi::c_int) << i);
            i = i + value;
            if i <= IQ_TABLE_SIZE_HALF {
                i = *pow_table_q17.offset(i as isize);
            } else {
                err_code |= ixheaacd_res_inv_quant(&mut i, pow_table_q17);
            }
            if out2 < 0 as core::ffi::c_int {
                i = -i;
            }
            let fresh60 = x_invquant;
            x_invquant = x_invquant.offset(1);
            *fresh60 = i;
        } else {
            let fresh61 = p_pul_arr;
            p_pul_arr = p_pul_arr.offset(1);
            let mut temp_0: WORD8 = *fresh61;
            if out2 <= 0 as core::ffi::c_int {
                out2 = temp_0 as WORD32 - out2;
                out2 = *pow_table_q17.offset(out2 as isize);
                let fresh62 = x_invquant;
                x_invquant = x_invquant.offset(1);
                *fresh62 = -out2;
            } else {
                out2 = (out2 as core::ffi::c_int + temp_0 as core::ffi::c_int) as WORD32;
                out2 = *pow_table_q17.offset(out2 as isize);
                let fresh63 = x_invquant;
                x_invquant = x_invquant.offset(1);
                *fresh63 = out2;
            }
        }
        idx -= 2 as core::ffi::c_int;
    }
    (*it_bif_buf).ptr_read_next = ptr_read_next
        .offset(-(4 as core::ffi::c_int as isize));
    (*it_bif_buf).bit_pos = bit_pos;
    return err_code as WORD;
}
#[inline]
unsafe extern "C" fn ixheaacd_res_c_block_decode_huff_word2_4(
    mut it_bit_buf: *mut ia_bit_buf_struct,
    mut qp: *mut WORD32,
    mut offsets: *mut WORD16,
    mut no_bands: WORD,
    mut group_no: WORD,
    mut h_ori: *const UWORD16,
    mut pow_table_q17: *mut WORD32,
    mut sign: WORD32,
    mut maximum_bins_short: WORD32,
) -> WORD {
    let mut value: WORD32 = 0;
    let mut idx: WORD = 0;
    let mut grp_idx: WORD = 0;
    let mut idx_len: WORD = 0;
    let mut qp_org: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr_read_next: *mut UWORD8 = (*it_bit_buf).ptr_read_next;
    let mut bit_pos: WORD32 = (*it_bit_buf).bit_pos;
    let mut read_word: WORD32 = ixheaacd_res_aac_showbits_32(ptr_read_next) as WORD32;
    ptr_read_next = ptr_read_next.offset(4 as core::ffi::c_int as isize);
    qp_org = qp;
    loop {
        idx_len = (*offsets.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
            - *offsets.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
            as WORD;
        grp_idx = group_no;
        loop {
            qp = qp
                .offset(
                    *offsets.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                        as isize,
                );
            idx = idx_len;
            loop {
                let mut first_offset: UWORD16 = 0;
                let mut sign_ret_val: WORD16 = 0;
                let mut read_word1: UWORD32 = 0;
                let mut h: *mut UWORD16 = 0 as *mut UWORD16;
                read_word1 = (read_word << bit_pos) as UWORD32;
                h = h_ori as *mut UWORD16;
                h = h.offset((read_word1 >> 27 as core::ffi::c_int) as isize);
                sign_ret_val = *h as WORD16;
                first_offset = 5 as UWORD16;
                while sign_ret_val as core::ffi::c_int > 0 as core::ffi::c_int {
                    bit_pos += first_offset as core::ffi::c_int;
                    ixheaacd_aac_read_byte_corr(
                        &mut ptr_read_next,
                        &mut bit_pos,
                        &mut read_word,
                        (*it_bit_buf).ptr_bit_buf_end,
                    );
                    read_word1 = read_word1 << first_offset as core::ffi::c_int;
                    first_offset = (sign_ret_val as core::ffi::c_int
                        >> 11 as core::ffi::c_int) as UWORD16;
                    h = h
                        .offset(
                            (sign_ret_val as core::ffi::c_int
                                & 0x7ff as core::ffi::c_int) as isize,
                        );
                    h = h
                        .offset(
                            (read_word1
                                >> 32 as core::ffi::c_int
                                    - first_offset as core::ffi::c_int) as isize,
                        );
                    sign_ret_val = *h as WORD16;
                }
                bit_pos
                    += (sign_ret_val as core::ffi::c_int & 0x7fff as core::ffi::c_int)
                        >> 11 as core::ffi::c_int;
                bit_pos = (if bit_pos < 31 as core::ffi::c_int {
                    bit_pos as core::ffi::c_int
                } else {
                    31 as core::ffi::c_int
                }) as WORD32;
                value = (sign_ret_val as core::ffi::c_int & 0x7ff as core::ffi::c_int)
                    as WORD32;
                if sign != 0 {
                    let mut temp_word: WORD32 = 0;
                    temp_word = read_word << bit_pos;
                    let fresh14 = qp;
                    qp = qp.offset(1);
                    *fresh14 = ixheaacd_res_extract_signed_symbol(
                        value,
                        24 as WORD32,
                        30 as WORD32,
                        pow_table_q17,
                        &mut temp_word,
                        &mut bit_pos,
                    );
                    let fresh15 = qp;
                    qp = qp.offset(1);
                    *fresh15 = ixheaacd_res_extract_signed_symbol(
                        value,
                        26 as WORD32,
                        30 as WORD32,
                        pow_table_q17,
                        &mut temp_word,
                        &mut bit_pos,
                    );
                    let fresh16 = qp;
                    qp = qp.offset(1);
                    *fresh16 = ixheaacd_res_extract_signed_symbol(
                        value,
                        28 as WORD32,
                        30 as WORD32,
                        pow_table_q17,
                        &mut temp_word,
                        &mut bit_pos,
                    );
                    let fresh17 = qp;
                    qp = qp.offset(1);
                    *fresh17 = ixheaacd_res_extract_signed_symbol(
                        value,
                        30 as WORD32,
                        30 as WORD32,
                        pow_table_q17,
                        &mut temp_word,
                        &mut bit_pos,
                    );
                } else {
                    let fresh18 = qp;
                    qp = qp.offset(1);
                    *fresh18 = ixheaacd_res_extract_symbol(
                        value,
                        24 as WORD32,
                        30 as WORD32,
                        pow_table_q17,
                    );
                    let fresh19 = qp;
                    qp = qp.offset(1);
                    *fresh19 = ixheaacd_res_extract_symbol(
                        value,
                        26 as WORD32,
                        30 as WORD32,
                        pow_table_q17,
                    );
                    let fresh20 = qp;
                    qp = qp.offset(1);
                    *fresh20 = ixheaacd_res_extract_symbol(
                        value,
                        28 as WORD32,
                        30 as WORD32,
                        pow_table_q17,
                    );
                    let fresh21 = qp;
                    qp = qp.offset(1);
                    *fresh21 = ixheaacd_res_extract_symbol(
                        value,
                        30 as WORD32,
                        30 as WORD32,
                        pow_table_q17,
                    );
                }
                ixheaacd_aac_read_byte_corr(
                    &mut ptr_read_next,
                    &mut bit_pos,
                    &mut read_word,
                    (*it_bit_buf).ptr_bit_buf_end,
                );
                idx -= 4 as core::ffi::c_int;
                if !(idx != 0 as core::ffi::c_int) {
                    break;
                }
            }
            qp = qp
                .offset(
                    (maximum_bins_short as core::ffi::c_int
                        - *offsets.offset(1 as core::ffi::c_int as isize)
                            as core::ffi::c_int) as isize,
                );
            grp_idx -= 1;
            if !(grp_idx != 0 as core::ffi::c_int) {
                break;
            }
        }
        offsets = offsets.offset(1);
        qp = qp_org;
        no_bands -= 1;
        if !(no_bands >= 0 as core::ffi::c_int) {
            break;
        }
    }
    (*it_bit_buf).ptr_read_next = ptr_read_next
        .offset(-(4 as core::ffi::c_int as isize));
    (*it_bit_buf).bit_pos = bit_pos;
    return 0 as WORD;
}
#[inline]
unsafe extern "C" fn ixheaacd_res_c_block_decode_huff_word2_4_lb(
    mut it_bit_buf: *mut ia_bit_buf_struct,
    mut len: WORD32,
    mut h_ori: *const UWORD16,
    mut x_invquant: *mut WORD32,
    mut pow_table_q17: *mut WORD32,
    mut p_pul_arr: *mut WORD8,
    mut sign: WORD32,
) -> WORD {
    let mut value: WORD32 = 0;
    let mut idx: WORD = 0;
    let mut ptr_read_next: *mut UWORD8 = (*it_bit_buf).ptr_read_next;
    let mut bit_pos: WORD32 = (*it_bit_buf).bit_pos;
    let mut read_word: WORD32 = ixheaacd_res_aac_showbits_32(ptr_read_next) as WORD32;
    ptr_read_next = ptr_read_next.offset(4 as core::ffi::c_int as isize);
    idx = len as WORD;
    while idx != 0 as core::ffi::c_int {
        let mut res: WORD32 = 0;
        let mut ampres: WORD32 = 0;
        let mut ampres1: WORD32 = 0;
        let mut ampres2: WORD32 = 0;
        let mut ampres3: WORD32 = 0;
        let mut first_offset: UWORD16 = 0;
        let mut sign_ret_val: WORD16 = 0;
        let mut read_word1: UWORD32 = 0;
        let mut h: *mut UWORD16 = 0 as *mut UWORD16;
        read_word1 = (read_word << bit_pos) as UWORD32;
        h = h_ori as *mut UWORD16;
        h = h.offset((read_word1 >> 27 as core::ffi::c_int) as isize);
        sign_ret_val = *h as WORD16;
        first_offset = 5 as UWORD16;
        while sign_ret_val as core::ffi::c_int > 0 as core::ffi::c_int {
            bit_pos += first_offset as core::ffi::c_int;
            ixheaacd_aac_read_byte_corr(
                &mut ptr_read_next,
                &mut bit_pos,
                &mut read_word,
                (*it_bit_buf).ptr_bit_buf_end,
            );
            read_word1 = read_word1 << first_offset as core::ffi::c_int;
            first_offset = (sign_ret_val as core::ffi::c_int >> 11 as core::ffi::c_int)
                as UWORD16;
            h = h
                .offset(
                    (sign_ret_val as core::ffi::c_int & 0x7ff as core::ffi::c_int)
                        as isize,
                );
            h = h
                .offset(
                    (read_word1
                        >> 32 as core::ffi::c_int - first_offset as core::ffi::c_int)
                        as isize,
                );
            sign_ret_val = *h as WORD16;
        }
        bit_pos
            += (sign_ret_val as core::ffi::c_int & 0x7fff as core::ffi::c_int)
                >> 11 as core::ffi::c_int;
        bit_pos = (if bit_pos < 31 as core::ffi::c_int {
            bit_pos as core::ffi::c_int
        } else {
            31 as core::ffi::c_int
        }) as WORD32;
        value = (sign_ret_val as core::ffi::c_int & 0x7ff as core::ffi::c_int) as WORD32;
        if sign != 0 {
            let mut out0: WORD32 = 0;
            let mut out1: WORD32 = 0;
            let mut out2: WORD32 = 0;
            let mut out3: WORD32 = 0;
            let mut ampout0: WORD32 = 0;
            let mut ampout1: WORD32 = 0;
            let mut ampout2: WORD32 = 0;
            let mut ampout3: WORD32 = 0;
            let mut temp_word: WORD32 = 0;
            temp_word = read_word << bit_pos;
            out0 = ixheaac_extu(value as UWORD32, 24 as WORD32, 30 as WORD32) as WORD32;
            let fresh38 = p_pul_arr;
            p_pul_arr = p_pul_arr.offset(1);
            ampout0 = (out0 as core::ffi::c_int + *fresh38 as core::ffi::c_int)
                as WORD32;
            ampout0 = *pow_table_q17.offset(ampout0 as isize);
            if out0 != 0 {
                if temp_word as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0
                {
                    ampout0 = -ampout0;
                }
                temp_word = temp_word << 1 as core::ffi::c_int;
                bit_pos += 1;
            } else {
                ampout0 = -ampout0;
            }
            out1 = ixheaac_extu(value as UWORD32, 26 as WORD32, 30 as WORD32) as WORD32;
            let fresh39 = p_pul_arr;
            p_pul_arr = p_pul_arr.offset(1);
            ampout1 = (out1 as core::ffi::c_int + *fresh39 as core::ffi::c_int)
                as WORD32;
            ampout1 = *pow_table_q17.offset(ampout1 as isize);
            if out1 != 0 {
                if temp_word as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0
                {
                    ampout1 = -ampout1;
                }
                temp_word = temp_word << 1 as core::ffi::c_int;
                bit_pos += 1;
            } else {
                ampout1 = -ampout1;
            }
            out2 = ixheaac_extu(value as UWORD32, 28 as WORD32, 30 as WORD32) as WORD32;
            let fresh40 = p_pul_arr;
            p_pul_arr = p_pul_arr.offset(1);
            ampout2 = (out2 as core::ffi::c_int + *fresh40 as core::ffi::c_int)
                as WORD32;
            ampout2 = *pow_table_q17.offset(ampout2 as isize);
            if out2 != 0 {
                if temp_word as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0
                {
                    ampout2 = -ampout2;
                }
                temp_word = temp_word << 1 as core::ffi::c_int;
                bit_pos += 1;
            } else {
                ampout2 = -ampout2;
            }
            let fresh41 = x_invquant;
            x_invquant = x_invquant.offset(1);
            *fresh41 = ampout0;
            let fresh42 = x_invquant;
            x_invquant = x_invquant.offset(1);
            *fresh42 = ampout1;
            let fresh43 = x_invquant;
            x_invquant = x_invquant.offset(1);
            *fresh43 = ampout2;
            out3 = ixheaac_extu(value as UWORD32, 30 as WORD32, 30 as WORD32) as WORD32;
            let fresh44 = p_pul_arr;
            p_pul_arr = p_pul_arr.offset(1);
            ampout3 = (out3 as core::ffi::c_int + *fresh44 as core::ffi::c_int)
                as WORD32;
            ampout3 = *pow_table_q17.offset(ampout3 as isize);
            if out3 != 0 {
                if temp_word as core::ffi::c_uint & 0x80000000 as core::ffi::c_uint != 0
                {
                    ampout3 = -ampout3;
                }
                temp_word = temp_word << 1 as core::ffi::c_int;
                bit_pos += 1;
            } else {
                ampout3 = -ampout3;
            }
            let fresh45 = x_invquant;
            x_invquant = x_invquant.offset(1);
            *fresh45 = ampout3;
        } else {
            let fresh46 = p_pul_arr;
            p_pul_arr = p_pul_arr.offset(1);
            ampres = *fresh46 as WORD32;
            res = ixheaacd_res_exts(value as UWORD32, 24 as WORD32, 30 as WORD32);
            if res > 0 as core::ffi::c_int {
                ampres = res + ampres;
                ampres = *pow_table_q17.offset(ampres as isize);
            } else {
                ampres = ampres - res;
                ampres = *pow_table_q17.offset(ampres as isize);
                ampres = -ampres;
            }
            res = ixheaacd_res_exts(value as UWORD32, 26 as WORD32, 30 as WORD32);
            let fresh47 = p_pul_arr;
            p_pul_arr = p_pul_arr.offset(1);
            ampres1 = *fresh47 as WORD32;
            if res > 0 as core::ffi::c_int {
                ampres1 = res + ampres1;
                ampres1 = *pow_table_q17.offset(ampres1 as isize);
            } else {
                ampres1 = ampres1 - res;
                ampres1 = *pow_table_q17.offset(ampres1 as isize);
                ampres1 = -ampres1;
            }
            res = ixheaacd_res_exts(value as UWORD32, 28 as WORD32, 30 as WORD32);
            let fresh48 = p_pul_arr;
            p_pul_arr = p_pul_arr.offset(1);
            ampres2 = *fresh48 as WORD32;
            if res > 0 as core::ffi::c_int {
                ampres2 = res + ampres2;
                ampres2 = *pow_table_q17.offset(ampres2 as isize);
            } else {
                ampres2 = ampres2 - res;
                ampres2 = *pow_table_q17.offset(ampres2 as isize);
                ampres2 = -ampres2;
            }
            res = ixheaacd_res_exts(value as UWORD32, 30 as WORD32, 30 as WORD32);
            let fresh49 = p_pul_arr;
            p_pul_arr = p_pul_arr.offset(1);
            ampres3 = *fresh49 as WORD32;
            if res > 0 as core::ffi::c_int {
                ampres3 = res + ampres3;
                ampres3 = *pow_table_q17.offset(ampres3 as isize);
            } else {
                ampres3 = ampres3 - res;
                ampres3 = *pow_table_q17.offset(ampres3 as isize);
                ampres3 = -ampres3;
            }
            let fresh50 = x_invquant;
            x_invquant = x_invquant.offset(1);
            *fresh50 = ampres;
            let fresh51 = x_invquant;
            x_invquant = x_invquant.offset(1);
            *fresh51 = ampres1;
            let fresh52 = x_invquant;
            x_invquant = x_invquant.offset(1);
            *fresh52 = ampres2;
            let fresh53 = x_invquant;
            x_invquant = x_invquant.offset(1);
            *fresh53 = ampres3;
        }
        ixheaacd_aac_read_byte_corr(
            &mut ptr_read_next,
            &mut bit_pos,
            &mut read_word,
            (*it_bit_buf).ptr_bit_buf_end,
        );
        idx -= 4 as core::ffi::c_int;
    }
    (*it_bit_buf).ptr_read_next = ptr_read_next
        .offset(-(4 as core::ffi::c_int as isize));
    (*it_bit_buf).bit_pos = bit_pos;
    return 0 as WORD;
}
#[inline]
unsafe extern "C" fn ixheaacd_res_c_block_decode_huff_word2_2(
    mut it_bif_buf: *mut ia_bit_buf_struct,
    mut qp: *mut WORD32,
    mut offsets: *mut WORD16,
    mut no_bands: WORD,
    mut group_no: WORD,
    mut h_ori: *const UWORD16,
    mut pow_table_q17: *mut WORD32,
    mut sign: WORD32,
    mut maximum_bins_short: WORD32,
) -> WORD {
    let mut value: WORD32 = 0;
    let mut idx: WORD = 0;
    let mut grp_idx: WORD = 0;
    let mut len_idx: WORD = 0;
    let mut qp_org: *mut WORD32 = qp;
    let mut ptr_read_next: *mut UWORD8 = (*it_bif_buf).ptr_read_next;
    let mut bit_pos: WORD32 = (*it_bif_buf).bit_pos;
    let mut read_word: WORD32 = ixheaacd_res_aac_showbits_32(ptr_read_next) as WORD32;
    ptr_read_next = ptr_read_next.offset(4 as core::ffi::c_int as isize);
    loop {
        len_idx = (*offsets.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
            - *offsets.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
            as WORD;
        grp_idx = group_no;
        loop {
            qp = qp
                .offset(
                    *offsets.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                        as isize,
                );
            idx = len_idx;
            loop {
                let mut first_offset: UWORD16 = 0;
                let mut sign_ret_val: WORD16 = 0;
                let mut read_word1: UWORD32 = 0;
                let mut h: *mut UWORD16 = 0 as *mut UWORD16;
                read_word1 = (read_word << bit_pos) as UWORD32;
                h = h_ori as *mut UWORD16;
                h = h.offset((read_word1 >> 27 as core::ffi::c_int) as isize);
                sign_ret_val = *h as WORD16;
                first_offset = 5 as UWORD16;
                while sign_ret_val as core::ffi::c_int > 0 as core::ffi::c_int {
                    bit_pos += first_offset as core::ffi::c_int;
                    ixheaacd_aac_read_byte_corr(
                        &mut ptr_read_next,
                        &mut bit_pos,
                        &mut read_word,
                        (*it_bif_buf).ptr_bit_buf_end,
                    );
                    read_word1 = read_word1 << first_offset as core::ffi::c_int;
                    first_offset = (sign_ret_val as core::ffi::c_int
                        >> 11 as core::ffi::c_int) as UWORD16;
                    h = h
                        .offset(
                            (sign_ret_val as core::ffi::c_int
                                & 0x7ff as core::ffi::c_int) as isize,
                        );
                    h = h
                        .offset(
                            (read_word1
                                >> 32 as core::ffi::c_int
                                    - first_offset as core::ffi::c_int) as isize,
                        );
                    sign_ret_val = *h as WORD16;
                }
                bit_pos
                    += (sign_ret_val as core::ffi::c_int & 0x7fff as core::ffi::c_int)
                        >> 11 as core::ffi::c_int;
                bit_pos = (if bit_pos < 31 as core::ffi::c_int {
                    bit_pos as core::ffi::c_int
                } else {
                    31 as core::ffi::c_int
                }) as WORD32;
                value = (sign_ret_val as core::ffi::c_int & 0x7ff as core::ffi::c_int)
                    as WORD32;
                if sign != 0 {
                    let mut temp_word: WORD32 = 0;
                    temp_word = read_word << bit_pos;
                    let fresh10 = qp;
                    qp = qp.offset(1);
                    *fresh10 = ixheaacd_res_extract_signed_symbol(
                        value,
                        24 as WORD32,
                        28 as WORD32,
                        pow_table_q17,
                        &mut temp_word,
                        &mut bit_pos,
                    );
                    let fresh11 = qp;
                    qp = qp.offset(1);
                    *fresh11 = ixheaacd_res_extract_signed_symbol(
                        value,
                        28 as WORD32,
                        28 as WORD32,
                        pow_table_q17,
                        &mut temp_word,
                        &mut bit_pos,
                    );
                } else {
                    let fresh12 = qp;
                    qp = qp.offset(1);
                    *fresh12 = ixheaacd_res_extract_symbol(
                        value,
                        24 as WORD32,
                        28 as WORD32,
                        pow_table_q17,
                    );
                    let fresh13 = qp;
                    qp = qp.offset(1);
                    *fresh13 = ixheaacd_res_extract_symbol(
                        value,
                        28 as WORD32,
                        28 as WORD32,
                        pow_table_q17,
                    );
                }
                ixheaacd_aac_read_byte_corr(
                    &mut ptr_read_next,
                    &mut bit_pos,
                    &mut read_word,
                    (*it_bif_buf).ptr_bit_buf_end,
                );
                idx -= 2 as core::ffi::c_int;
                if !(idx != 0 as core::ffi::c_int) {
                    break;
                }
            }
            qp = qp
                .offset(
                    (maximum_bins_short as core::ffi::c_int
                        - *offsets.offset(1 as core::ffi::c_int as isize)
                            as core::ffi::c_int) as isize,
                );
            grp_idx -= 1;
            if !(grp_idx != 0 as core::ffi::c_int) {
                break;
            }
        }
        offsets = offsets.offset(1);
        qp = qp_org;
        no_bands -= 1;
        if !(no_bands >= 0 as core::ffi::c_int) {
            break;
        }
    }
    (*it_bif_buf).ptr_read_next = ptr_read_next
        .offset(-(4 as core::ffi::c_int as isize));
    (*it_bif_buf).bit_pos = bit_pos;
    return 0 as WORD;
}
#[inline]
unsafe extern "C" fn ixheaacd_res_c_block_decode_huff_word2_2_lb(
    mut it_bit_buf: *mut ia_bit_buf_struct,
    mut len: WORD32,
    mut h_ori: *const UWORD16,
    mut x_invquant: *mut WORD32,
    mut pow_table_q17: *mut WORD32,
    mut p_pul_arr: *mut WORD8,
    mut sign: WORD32,
) -> WORD {
    let mut value: WORD32 = 0;
    let mut res: WORD32 = 0;
    let mut ampres: WORD32 = 0;
    let mut idx: WORD = 0;
    let mut ptr_read_next: *mut UWORD8 = (*it_bit_buf).ptr_read_next;
    let mut bit_pos: WORD32 = (*it_bit_buf).bit_pos;
    let mut read_word: WORD32 = ixheaacd_res_aac_showbits_32(ptr_read_next) as WORD32;
    ptr_read_next = ptr_read_next.offset(4 as core::ffi::c_int as isize);
    idx = len as WORD;
    while idx != 0 as core::ffi::c_int {
        let mut first_offset: UWORD16 = 0;
        let mut sign_ret_val: WORD16 = 0;
        let mut read_word1: UWORD32 = 0;
        let mut h: *mut UWORD16 = 0 as *mut UWORD16;
        read_word1 = (read_word << bit_pos) as UWORD32;
        h = h_ori as *mut UWORD16;
        h = h.offset((read_word1 >> 27 as core::ffi::c_int) as isize);
        sign_ret_val = *h as WORD16;
        first_offset = 5 as UWORD16;
        while sign_ret_val as core::ffi::c_int > 0 as core::ffi::c_int {
            bit_pos += first_offset as core::ffi::c_int;
            ixheaacd_aac_read_byte_corr(
                &mut ptr_read_next,
                &mut bit_pos,
                &mut read_word,
                (*it_bit_buf).ptr_bit_buf_end,
            );
            read_word1 = read_word1 << first_offset as core::ffi::c_int;
            first_offset = (sign_ret_val as core::ffi::c_int >> 11 as core::ffi::c_int)
                as UWORD16;
            h = h
                .offset(
                    (sign_ret_val as core::ffi::c_int & 0x7ff as core::ffi::c_int)
                        as isize,
                );
            h = h
                .offset(
                    (read_word1
                        >> 32 as core::ffi::c_int - first_offset as core::ffi::c_int)
                        as isize,
                );
            sign_ret_val = *h as WORD16;
        }
        bit_pos
            += (sign_ret_val as core::ffi::c_int & 0x7fff as core::ffi::c_int)
                >> 11 as core::ffi::c_int;
        bit_pos = (if bit_pos < 31 as core::ffi::c_int {
            bit_pos as core::ffi::c_int
        } else {
            31 as core::ffi::c_int
        }) as WORD32;
        value = (sign_ret_val as core::ffi::c_int & 0x7ff as core::ffi::c_int) as WORD32;
        if sign != 0 {
            let mut out0: WORD32 = 0;
            let mut out1: WORD32 = 0;
            let mut temp_word: WORD32 = 0;
            let mut ampout0: WORD32 = 0;
            let mut ampout1: WORD32 = 0;
            let fresh28 = p_pul_arr;
            p_pul_arr = p_pul_arr.offset(1);
            ampout0 = *fresh28 as WORD32;
            let fresh29 = p_pul_arr;
            p_pul_arr = p_pul_arr.offset(1);
            ampout1 = *fresh29 as WORD32;
            out0 = (value as core::ffi::c_int & 0xf0 as core::ffi::c_int) as WORD32;
            ampout0 = (ampout0 as UWORD32)
                .wrapping_add(out0 as UWORD32 >> 4 as core::ffi::c_int) as WORD32;
            ampout0 = *pow_table_q17.offset(ampout0 as isize);
            out1 = (value as core::ffi::c_int & 0xf as core::ffi::c_int) as WORD32;
            ampout1 = out1 + ampout1;
            ampout1 = *pow_table_q17.offset(ampout1 as isize);
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
                (*it_bit_buf).ptr_bit_buf_end,
            );
            let fresh30 = x_invquant;
            x_invquant = x_invquant.offset(1);
            *fresh30 = ampout0;
            let fresh31 = x_invquant;
            x_invquant = x_invquant.offset(1);
            *fresh31 = ampout1;
        } else {
            res = value << 24 as core::ffi::c_int >> 28 as core::ffi::c_int;
            let fresh32 = p_pul_arr;
            p_pul_arr = p_pul_arr.offset(1);
            ampres = *fresh32 as WORD32;
            if res > 0 as core::ffi::c_int {
                ampres = res + ampres;
                let fresh33 = x_invquant;
                x_invquant = x_invquant.offset(1);
                *fresh33 = *pow_table_q17.offset(ampres as isize);
            } else {
                ampres = ampres - res;
                ampres = *pow_table_q17.offset(ampres as isize);
                let fresh34 = x_invquant;
                x_invquant = x_invquant.offset(1);
                *fresh34 = -ampres;
            }
            res = value << 28 as core::ffi::c_int >> 28 as core::ffi::c_int;
            let fresh35 = p_pul_arr;
            p_pul_arr = p_pul_arr.offset(1);
            value = *fresh35 as WORD32;
            if res > 0 as core::ffi::c_int {
                ampres = res + value;
                let fresh36 = x_invquant;
                x_invquant = x_invquant.offset(1);
                *fresh36 = *pow_table_q17.offset(ampres as isize);
            } else {
                ampres = value - res;
                ampres = *pow_table_q17.offset(ampres as isize);
                let fresh37 = x_invquant;
                x_invquant = x_invquant.offset(1);
                *fresh37 = -ampres;
            }
        }
        ixheaacd_aac_read_byte_corr(
            &mut ptr_read_next,
            &mut bit_pos,
            &mut read_word,
            (*it_bit_buf).ptr_bit_buf_end,
        );
        idx -= 2 as core::ffi::c_int;
    }
    (*it_bit_buf).ptr_read_next = ptr_read_next
        .offset(-(4 as core::ffi::c_int as isize));
    (*it_bit_buf).bit_pos = bit_pos;
    return 0 as WORD;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_res_c_block_decode_huff_word_all(
    mut it_bit_buf: *mut ia_bit_buf_struct,
    mut code_no: WORD32,
    mut quantized_coef: *mut WORD32,
    mut band_offsets: *mut WORD16,
    mut start: WORD,
    mut band: WORD,
    mut group_no: WORD,
    mut aac_tables_ptr: *mut ia_mps_dec_residual_aac_tables_struct,
    mut maximum_bins_short: WORD32,
) -> WORD {
    let mut ret_val: WORD = 0 as WORD;
    let mut start_bit_pos: WORD = (*it_bit_buf).bit_pos as WORD;
    let mut start_read_pos: *mut UWORD8 = (*it_bit_buf).ptr_read_next;
    let mut h_ori: *const UWORD16 = (*aac_tables_ptr).code_book[code_no as usize];
    let mut pow_table: *mut WORD32 = ((*(*aac_tables_ptr).res_block_tables_ptr)
        .pow_table_q17)
        .as_mut_ptr();
    let mut no_bands: WORD32 = band as WORD32 - start as WORD32 - 1 as WORD32;
    let mut p_band_off: *mut WORD16 = band_offsets.offset(start as isize);
    if code_no == 11 as core::ffi::c_int {
        let mut h_ori_0: *const UWORD16 = ((*(*aac_tables_ptr).res_huffmann_tables_ptr)
            .huffman_codebook_11)
            .as_mut_ptr();
        ret_val = ixheaacd_res_c_block_decode_huff_word1(
            it_bit_buf,
            quantized_coef,
            p_band_off,
            no_bands as WORD,
            group_no,
            h_ori_0,
            pow_table,
            maximum_bins_short,
        );
    } else if code_no <= 4 as core::ffi::c_int {
        let mut sign: WORD32 = 0 as WORD32;
        if code_no > 2 as core::ffi::c_int {
            sign = 1 as core::ffi::c_int as WORD32;
        }
        ret_val = ixheaacd_res_c_block_decode_huff_word2_4(
            it_bit_buf,
            quantized_coef,
            p_band_off,
            no_bands as WORD,
            group_no,
            h_ori,
            pow_table,
            sign,
            maximum_bins_short,
        );
    } else if code_no <= 10 as core::ffi::c_int {
        let mut sign_0: WORD32 = 0 as WORD32;
        if code_no > 6 as core::ffi::c_int {
            sign_0 = 1 as core::ffi::c_int as WORD32;
        }
        ret_val = ixheaacd_res_c_block_decode_huff_word2_2(
            it_bit_buf,
            quantized_coef,
            p_band_off,
            no_bands as WORD,
            group_no,
            h_ori,
            pow_table,
            sign_0,
            maximum_bins_short,
        );
    }
    let mut bits_cons: WORD = 0;
    bits_cons = (((((*it_bit_buf).ptr_read_next).offset_from(start_read_pos)
        as core::ffi::c_long) << 3 as core::ffi::c_int)
        + ((*it_bit_buf).bit_pos as WORD - start_bit_pos) as core::ffi::c_long) as WORD;
    (*it_bit_buf).cnt_bits -= bits_cons as core::ffi::c_int;
    return ret_val;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_res_c_block_decode_huff_word_all_lb(
    mut it_bit_buf: *mut ia_bit_buf_struct,
    mut code_no: WORD32,
    mut len: WORD32,
    mut aac_tables_ptr: *mut ia_mps_dec_residual_aac_tables_struct,
    mut x_invquant: *mut WORD32,
    mut p_pul_arr: *mut WORD8,
) -> WORD {
    let mut ret_val: WORD = 0 as WORD;
    let mut start_bit_pos: WORD = (*it_bit_buf).bit_pos as WORD;
    let mut pow_table: *mut WORD32 = ((*(*aac_tables_ptr).res_block_tables_ptr)
        .pow_table_q17)
        .as_mut_ptr();
    let mut start_read_pos: *mut UWORD8 = (*it_bit_buf).ptr_read_next;
    let mut h_ori: *const UWORD16 = (*aac_tables_ptr).code_book[code_no as usize];
    if code_no == 11 as core::ffi::c_int {
        let mut h_ori_0: *const UWORD16 = ((*(*aac_tables_ptr).res_huffmann_tables_ptr)
            .huffman_codebook_11)
            .as_mut_ptr();
        ret_val = ixheaacd_res_c_block_decode_huff_word1_lb(
            it_bit_buf,
            len,
            h_ori_0,
            x_invquant,
            pow_table,
            p_pul_arr,
        );
    } else if code_no <= 4 as core::ffi::c_int {
        let mut sign: WORD32 = 0 as WORD32;
        if code_no > 2 as core::ffi::c_int {
            sign = 1 as core::ffi::c_int as WORD32;
        }
        ret_val = ixheaacd_res_c_block_decode_huff_word2_4_lb(
            it_bit_buf,
            len,
            h_ori,
            x_invquant,
            pow_table,
            p_pul_arr,
            sign,
        );
    } else if code_no <= 10 as core::ffi::c_int {
        let mut sign_0: WORD32 = 0 as WORD32;
        if code_no > 6 as core::ffi::c_int {
            sign_0 = 1 as core::ffi::c_int as WORD32;
        }
        ret_val = ixheaacd_res_c_block_decode_huff_word2_2_lb(
            it_bit_buf,
            len,
            h_ori,
            x_invquant,
            pow_table,
            p_pul_arr,
            sign_0,
        );
    }
    let mut bits_cons: WORD = 0;
    if (*it_bit_buf).bit_pos <= 7 as core::ffi::c_int {
        bits_cons = (((((*it_bit_buf).ptr_read_next).offset_from(start_read_pos)
            as core::ffi::c_long) << 3 as core::ffi::c_int)
            + ((*it_bit_buf).bit_pos as WORD - start_bit_pos) as core::ffi::c_long)
            as WORD;
        (*it_bit_buf).cnt_bits -= bits_cons as core::ffi::c_int;
    } else {
        (*it_bit_buf).ptr_read_next = ((*it_bit_buf).ptr_read_next)
            .offset(((*it_bit_buf).bit_pos >> 3 as core::ffi::c_int) as isize);
        (*it_bit_buf).bit_pos = ((*it_bit_buf).bit_pos as core::ffi::c_int
            & 0x7 as core::ffi::c_int) as WORD32;
        bits_cons = (((((*it_bit_buf).ptr_read_next).offset_from(start_read_pos)
            as core::ffi::c_long) << 3 as core::ffi::c_int)
            + ((*it_bit_buf).bit_pos as WORD - start_bit_pos) as core::ffi::c_long)
            as WORD;
        (*it_bit_buf).cnt_bits -= bits_cons as core::ffi::c_int;
    }
    return ret_val;
}
unsafe extern "C" fn ixheaacd_res_apply_one_scf(
    mut scale_factor: WORD32,
    mut x_invquant: *mut WORD32,
    mut end: WORD32,
    mut scale_table_ptr: *mut WORD32,
) -> VOID {
    let mut j: WORD32 = 0;
    let mut temp_1: WORD32 = 0;
    let mut q_factor: WORD32 = 0;
    let mut buffer1: WORD32 = 0;
    let mut scale_short: WORD16 = 0;
    if scale_factor < 24 as core::ffi::c_int {
        j = end;
        while j > 0 as core::ffi::c_int {
            let fresh3 = x_invquant;
            x_invquant = x_invquant.offset(1);
            *fresh3 = 0 as core::ffi::c_int as WORD32;
            j -= 1;
        }
    } else {
        let mut shift: WORD32 = 0;
        q_factor = 37 as WORD32 - (scale_factor >> 2 as core::ffi::c_int);
        scale_short = *scale_table_ptr
            .offset(
                (scale_factor as core::ffi::c_int & 0x3 as core::ffi::c_int) as isize,
            ) as WORD16;
        shift = q_factor;
        if shift > 0 as core::ffi::c_int {
            if scale_short as core::ffi::c_int
                == 0x8000 as core::ffi::c_int as WORD16 as core::ffi::c_int
            {
                j = end;
                while j > 0 as core::ffi::c_int {
                    temp_1 = *x_invquant;
                    buffer1 = ixheaac_mult32x16in32_shl_sat(temp_1, scale_short);
                    buffer1 = ixheaac_shr32(buffer1, shift as WORD);
                    let fresh4 = x_invquant;
                    x_invquant = x_invquant.offset(1);
                    *fresh4 = buffer1;
                    j -= 1;
                }
            } else {
                j = end;
                while j > 0 as core::ffi::c_int {
                    temp_1 = *x_invquant;
                    buffer1 = ixheaac_mult32x16in32_shl(temp_1, scale_short);
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
                    j = end;
                    while j > 0 as core::ffi::c_int {
                        temp_1 = *x_invquant;
                        temp_1 = ixheaac_shl32(temp_1, shift as WORD - 1 as WORD);
                        buffer1 = ixheaac_mult32x16in32_shl_sat(temp_1, scale_short);
                        buffer1 = ixheaac_shl32(buffer1, 1 as WORD);
                        let fresh6 = x_invquant;
                        x_invquant = x_invquant.offset(1);
                        *fresh6 = buffer1;
                        j -= 1;
                    }
                } else {
                    j = end;
                    while j > 0 as core::ffi::c_int {
                        temp_1 = *x_invquant;
                        temp_1 = ixheaac_shl32(temp_1, shift as WORD - 1 as WORD);
                        buffer1 = ixheaac_mult32x16in32_shl(temp_1, scale_short);
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
                j = end;
                while j > 0 as core::ffi::c_int {
                    temp_1 = *x_invquant;
                    buffer1 = ixheaac_mult32x16in32_shl_sat(temp_1, scale_short);
                    let fresh8 = x_invquant;
                    x_invquant = x_invquant.offset(1);
                    *fresh8 = buffer1;
                    j -= 1;
                }
            } else {
                j = end;
                while j > 0 as core::ffi::c_int {
                    temp_1 = *x_invquant;
                    buffer1 = ixheaac_mult32x16in32_shl(temp_1, scale_short);
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
pub unsafe extern "C" fn ixheaacd_res_apply_scfs(
    mut x_invquant: *mut WORD32,
    mut sc_factor: *mut WORD16,
    mut t_bands: WORD,
    mut offset: *mut WORD8,
    mut scale_table_ptr: *mut WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut scale_factor: WORD16 = 0;
    i = (t_bands as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while i >= 0 as core::ffi::c_int {
        let fresh2 = sc_factor;
        sc_factor = sc_factor.offset(1);
        scale_factor = *fresh2;
        ixheaacd_res_apply_one_scf(
            scale_factor as WORD32,
            x_invquant,
            *offset as WORD32,
            scale_table_ptr,
        );
        x_invquant = x_invquant.offset(*offset as core::ffi::c_int as isize);
        offset = offset.offset(1);
        i -= 1;
    }
}
