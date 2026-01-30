extern "C" {
    fn ixheaacd_read_bits_buf(
        it_bit_buff: *mut ia_bit_buf_struct,
        no_of_bits: WORD,
    ) -> WORD32;
}
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
pub type VOID = ();
pub type WORD = core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_huff_code_word_struct {
    pub index: WORD32,
    pub len: WORD32,
    pub code_word: UWORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_huff_code_book_struct {
    pub num: WORD32,
    pub dim: WORD32,
    pub lav: WORD32,
    pub lav_incr_esc: WORD32,
    pub huff_mode: WORD32,
    pub off: WORD32,
    pub sign_code_book: WORD32,
    pub max_code_word_len: UWORD16,
    pub pstr_huff_code_word: *const ia_huff_code_word_struct,
    pub code_book_tbl: *const WORD16,
    pub idx_tbl: *const WORD32,
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hufftab(
    mut ptr_huff_code_book: *mut ia_huff_code_book_struct,
    mut ptr_huff_code_word: *const ia_huff_code_word_struct,
    mut code_book_tbl: *const WORD16,
    mut index: *const WORD32,
    mut dim: WORD32,
    mut lav: WORD32,
    mut lav_incr_esc: WORD32,
    mut sign_code_book: WORD32,
    mut max_code_word_len: UWORD8,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut num: WORD32 = 0;
    if sign_code_book == 0 {
        (*ptr_huff_code_book).huff_mode = (lav as core::ffi::c_int
            + 1 as core::ffi::c_int) as WORD32;
        (*ptr_huff_code_book).off = 0 as core::ffi::c_int as WORD32;
    } else {
        (*ptr_huff_code_book).huff_mode = (2 as core::ffi::c_int
            * lav as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
        (*ptr_huff_code_book).off = lav;
    }
    num = 1 as core::ffi::c_int as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < dim {
        num *= (*ptr_huff_code_book).huff_mode;
        i += 1;
    }
    (*ptr_huff_code_book).num = num;
    (*ptr_huff_code_book).dim = dim;
    (*ptr_huff_code_book).lav = lav;
    (*ptr_huff_code_book).lav_incr_esc = lav_incr_esc;
    (*ptr_huff_code_book).sign_code_book = sign_code_book;
    (*ptr_huff_code_book).pstr_huff_code_word = ptr_huff_code_word;
    (*ptr_huff_code_book).code_book_tbl = code_book_tbl;
    (*ptr_huff_code_book).idx_tbl = index;
    (*ptr_huff_code_book).max_code_word_len = max_code_word_len as UWORD16;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_huff_codeword(
    mut ptr_huff_code_word: *const ia_huff_code_word_struct,
    mut data_present: UWORD16,
    mut it_bit_buff: *mut ia_bit_buf_struct,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut code_word: UWORD32 = 0 as UWORD32;
    let mut tmp: UWORD32 = 0 as UWORD32;
    i = (*ptr_huff_code_word).len;
    if data_present as core::ffi::c_int == 0 as core::ffi::c_int {
        code_word = ixheaacd_read_bits_buf(it_bit_buff, i as WORD) as UWORD32;
    }
    if data_present as core::ffi::c_int == 1 as core::ffi::c_int {
        code_word = ixheaacd_read_bits_buf(it_bit_buff, i as WORD) as UWORD32;
    }
    while code_word != (*ptr_huff_code_word).code_word {
        ptr_huff_code_word = ptr_huff_code_word.offset(1);
        j = (*ptr_huff_code_word).len - i;
        if j < 0 as core::ffi::c_int {
            return (*ptr_huff_code_word).index;
        }
        i += j;
        code_word <<= j;
        if data_present as core::ffi::c_int == 0 as core::ffi::c_int {
            tmp = ixheaacd_read_bits_buf(it_bit_buff, j as WORD) as UWORD32;
        }
        if data_present as core::ffi::c_int == 1 as core::ffi::c_int {
            tmp = ixheaacd_read_bits_buf(it_bit_buff, j as WORD) as UWORD32;
        }
        code_word |= tmp;
    }
    return (*ptr_huff_code_word).index;
}
