extern "C" {
    fn longjmp(__env: *mut __jmp_buf_tag, __val: core::ffi::c_int) -> !;
    fn ixheaacd_aac_showbits_32(
        ptr_read_next: *mut UWORD8,
        cnt_bits: WORD32,
        increment: *mut WORD32,
    ) -> UWORD32;
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
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES: core::ffi::c_int = 0x1804
    as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_byte_align(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut align_bits_cnt: *mut WORD32,
) -> VOID {
    let mut alignment: WORD = 0;
    alignment = *align_bits_cnt - (*it_bit_buff).cnt_bits as core::ffi::c_int
        & 0x7 as core::ffi::c_int;
    if alignment != 0 {
        ixheaacd_read_bits_buf(it_bit_buff, 8 as WORD - alignment);
    }
    *align_bits_cnt = (*it_bit_buff).cnt_bits;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_skip_bits_buf(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut no_of_bits: WORD,
) -> WORD32 {
    let mut ptr_read_next: *mut UWORD8 = (*it_bit_buff).ptr_read_next;
    let mut bit_pos: WORD = (*it_bit_buff).bit_pos as WORD;
    if (*it_bit_buff).cnt_bits < no_of_bits
        || (*it_bit_buff).cnt_bits < 0 as core::ffi::c_int
    {
        longjmp(
            (*(*it_bit_buff).xaac_jmp_buf).as_mut_ptr(),
            IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES,
        );
    }
    (*it_bit_buff).cnt_bits -= no_of_bits as core::ffi::c_int;
    ptr_read_next = ptr_read_next
        .offset((no_of_bits as core::ffi::c_int / 8 as core::ffi::c_int) as isize);
    bit_pos -= no_of_bits as core::ffi::c_int % 8 as core::ffi::c_int;
    if bit_pos < 0 as core::ffi::c_int {
        bit_pos += 8 as core::ffi::c_int;
        ptr_read_next = ptr_read_next.offset(1);
    }
    (*it_bit_buff).ptr_read_next = ptr_read_next;
    (*it_bit_buff).bit_pos = bit_pos as WORD16 as WORD32;
    return no_of_bits as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_show_bits_buf(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut no_of_bits: WORD,
) -> WORD32 {
    let mut ret_val: UWORD32 = 0;
    let mut ptr_read_next: *mut UWORD8 = (*it_bit_buff).ptr_read_next;
    let mut bit_pos: WORD = (*it_bit_buff).bit_pos as WORD;
    if no_of_bits == 0 as core::ffi::c_int {
        return 0 as WORD32;
    }
    if (*it_bit_buff).cnt_bits < no_of_bits
        || (*it_bit_buff).cnt_bits < 0 as core::ffi::c_int
        || no_of_bits > 25 as core::ffi::c_int
    {
        longjmp(
            (*(*it_bit_buff).xaac_jmp_buf).as_mut_ptr(),
            IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES,
        );
    }
    ret_val = *ptr_read_next as UWORD32;
    bit_pos -= no_of_bits;
    while bit_pos < -(1 as core::ffi::c_int) {
        bit_pos += 8 as core::ffi::c_int;
        ptr_read_next = ptr_read_next.offset(1);
        ret_val <<= 8 as core::ffi::c_int;
        ret_val |= *ptr_read_next as UWORD32;
    }
    if bit_pos == -(1 as core::ffi::c_int) {
        bit_pos += 8 as core::ffi::c_int;
        ret_val <<= 8 as core::ffi::c_int;
        ptr_read_next = ptr_read_next.offset(1);
    }
    ret_val = ret_val << 31 as WORD - no_of_bits - bit_pos >> 32 as WORD - no_of_bits;
    return ret_val as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_read_bits_buf(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut no_of_bits: WORD,
) -> WORD32 {
    let mut ret_val: UWORD32 = 0;
    let mut ptr_read_next: *mut UWORD8 = (*it_bit_buff).ptr_read_next;
    let mut bit_pos: WORD = (*it_bit_buff).bit_pos as WORD;
    if no_of_bits == 0 as core::ffi::c_int {
        return 0 as WORD32;
    }
    if (*it_bit_buff).cnt_bits < no_of_bits
        || (*it_bit_buff).cnt_bits < 0 as core::ffi::c_int
        || no_of_bits > 25 as core::ffi::c_int
    {
        longjmp(
            (*(*it_bit_buff).xaac_jmp_buf).as_mut_ptr(),
            IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES,
        );
    }
    (*it_bit_buff).cnt_bits -= no_of_bits as core::ffi::c_int;
    ret_val = *ptr_read_next as UWORD32;
    bit_pos -= no_of_bits;
    while bit_pos < -(1 as core::ffi::c_int) {
        bit_pos += 8 as core::ffi::c_int;
        ptr_read_next = ptr_read_next.offset(1);
        ret_val <<= 8 as core::ffi::c_int;
        ret_val |= *ptr_read_next as UWORD32;
    }
    if bit_pos == -(1 as core::ffi::c_int) {
        bit_pos += 8 as core::ffi::c_int;
        ret_val <<= 8 as core::ffi::c_int;
        ptr_read_next = ptr_read_next.offset(1);
    }
    ret_val = ret_val << 31 as WORD - no_of_bits - bit_pos >> 32 as WORD - no_of_bits;
    (*it_bit_buff).ptr_read_next = ptr_read_next;
    (*it_bit_buff).bit_pos = bit_pos as WORD16 as WORD32;
    return ret_val as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_aac_read_byte(
    mut ptr_read_next: *mut *mut UWORD8,
    mut bit_pos: *mut WORD32,
    mut readword: *mut WORD32,
) -> VOID {
    let mut v: *mut UWORD8 = *ptr_read_next;
    let mut bits_consumed: WORD32 = *bit_pos;
    bits_consumed -= 8 as core::ffi::c_int;
    if bits_consumed >= 0 as core::ffi::c_int {
        *readword = (*readword << 8 as core::ffi::c_int | *v as core::ffi::c_int)
            as WORD32;
        v = v.offset(1);
    } else {
        bits_consumed += 8 as core::ffi::c_int;
    }
    *bit_pos = bits_consumed;
    *ptr_read_next = v;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_aac_read_byte_corr1(
    mut ptr_read_next: *mut *mut UWORD8,
    mut ptr_bit_pos: *mut WORD32,
    mut readword: *mut WORD32,
    mut p_bit_buf_end: *mut UWORD8,
) -> VOID {
    let mut v: *mut UWORD8 = *ptr_read_next;
    let mut bits_consumed: WORD32 = *ptr_bit_pos;
    let mut temp_bit_count: WORD32 = 0 as WORD32;
    while bits_consumed >= 8 as core::ffi::c_int {
        bits_consumed -= 8 as core::ffi::c_int;
        if p_bit_buf_end < v && !p_bit_buf_end.is_null() {
            temp_bit_count += 8 as core::ffi::c_int;
        } else {
            *readword = (*readword << 8 as core::ffi::c_int | *v as core::ffi::c_int)
                as WORD32;
            v = v.offset(1);
        }
    }
    if bits_consumed > 31 as WORD32 - temp_bit_count {
        if !p_bit_buf_end.is_null() && p_bit_buf_end < v {
            bits_consumed = 31 as WORD32 - temp_bit_count;
        }
    }
    *ptr_bit_pos = bits_consumed + temp_bit_count;
    *ptr_read_next = v;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_aac_read_byte_corr(
    mut ptr_read_next: *mut *mut UWORD8,
    mut ptr_bit_pos: *mut WORD32,
    mut readword: *mut WORD32,
    mut p_bit_buf_end: *mut UWORD8,
) -> VOID {
    let mut v: *mut UWORD8 = *ptr_read_next;
    let mut bits_consumed: WORD32 = *ptr_bit_pos;
    bits_consumed -= 8 as core::ffi::c_int;
    if bits_consumed >= 0 as core::ffi::c_int {
        if p_bit_buf_end < v {
            bits_consumed += 8 as core::ffi::c_int;
        } else {
            *readword = (*readword << 8 as core::ffi::c_int | *v as core::ffi::c_int)
                as WORD32;
            v = v.offset(1);
        }
    } else {
        bits_consumed += 8 as core::ffi::c_int;
    }
    if bits_consumed > 31 as core::ffi::c_int {
        if p_bit_buf_end < v {
            bits_consumed = 31 as core::ffi::c_int as WORD32;
        }
    }
    *ptr_bit_pos = bits_consumed;
    *ptr_read_next = v;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_aac_read_bit(
    mut it_bit_buff: *mut ia_bit_buf_struct,
) -> WORD32 {
    let mut ret_val: UWORD8 = 0;
    let mut ptr_read_next: *mut UWORD8 = (*it_bit_buff).ptr_read_next;
    let mut bit_pos: WORD = (*it_bit_buff).bit_pos as WORD;
    let mut temp: UWORD32 = 0;
    let mut no_of_bits: WORD = 1 as WORD;
    if bit_pos < 0 as core::ffi::c_int {
        bit_pos = 7 as core::ffi::c_int as WORD;
        ptr_read_next = ptr_read_next.offset(-1);
    }
    if ptr_read_next < (*it_bit_buff).ptr_bit_buf_base {
        longjmp(
            (*(*it_bit_buff).xaac_jmp_buf).as_mut_ptr(),
            IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES,
        );
    }
    (*it_bit_buff).cnt_bits += no_of_bits as core::ffi::c_int;
    ret_val = *ptr_read_next;
    bit_pos -= no_of_bits;
    temp = (((ret_val as core::ffi::c_int) << 24 as core::ffi::c_int)
        << bit_pos + no_of_bits) as UWORD32;
    (*it_bit_buff).ptr_read_next = ptr_read_next;
    (*it_bit_buff).bit_pos = bit_pos as WORD16 as WORD32;
    return (temp >> 32 as WORD - no_of_bits) as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_aac_read_bit_rev(
    mut it_bit_buff: *mut ia_bit_buf_struct,
) -> WORD32 {
    let mut ret_val: UWORD8 = 0;
    let mut ptr_read_next: *mut UWORD8 = (*it_bit_buff).ptr_read_next;
    let mut bit_pos: WORD = (*it_bit_buff).bit_pos as WORD;
    let mut temp: UWORD32 = 0;
    let mut no_of_bits: WORD = 1 as WORD;
    if (*it_bit_buff).cnt_bits < no_of_bits
        || (*it_bit_buff).cnt_bits < 0 as core::ffi::c_int
    {
        longjmp(
            (*(*it_bit_buff).xaac_jmp_buf).as_mut_ptr(),
            IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES,
        );
    }
    if bit_pos >= 8 as core::ffi::c_int {
        bit_pos -= 8 as core::ffi::c_int;
        ptr_read_next = ptr_read_next.offset(1);
    }
    (*it_bit_buff).cnt_bits -= no_of_bits as core::ffi::c_int;
    ret_val = *ptr_read_next;
    bit_pos += no_of_bits;
    temp = (((ret_val as core::ffi::c_int) << 24 as core::ffi::c_int)
        << bit_pos - no_of_bits) as UWORD32;
    (*it_bit_buff).ptr_read_next = ptr_read_next;
    (*it_bit_buff).bit_pos = bit_pos as WORD16 as WORD32;
    return (temp >> 32 as WORD - no_of_bits) as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_write_bit(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut value: WORD32,
    mut no_of_bits: WORD32,
) -> VOID {
    let mut mask: WORD32 = 0;
    if no_of_bits == 0 as core::ffi::c_int {
        return;
    }
    mask = 0x1 as core::ffi::c_int as WORD32;
    mask <<= no_of_bits as core::ffi::c_int - 1 as core::ffi::c_int;
    (*it_bit_buff).bit_count += no_of_bits;
    while no_of_bits > 0 as core::ffi::c_int {
        while no_of_bits > 0 as core::ffi::c_int
            && (*it_bit_buff).valid_bits < 8 as core::ffi::c_int
        {
            (*it_bit_buff).byte = (((*it_bit_buff).byte as core::ffi::c_int)
                << 1 as core::ffi::c_int) as UWORD8;
            if value & mask != 0 {
                (*it_bit_buff).byte = ((*it_bit_buff).byte as core::ffi::c_int
                    | 0x1 as core::ffi::c_int) as UWORD8;
            }
            value <<= 1 as core::ffi::c_int;
            no_of_bits -= 1;
            (*it_bit_buff).valid_bits += 1;
        }
        if (*it_bit_buff).valid_bits == 8 as core::ffi::c_int {
            let fresh0 = (*it_bit_buff).byte_ptr;
            (*it_bit_buff).byte_ptr = ((*it_bit_buff).byte_ptr).offset(1);
            *fresh0 = (*it_bit_buff).byte;
            (*it_bit_buff).byte = 0 as UWORD8;
            (*it_bit_buff).valid_bits = 0 as core::ffi::c_int as WORD32;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_read_bit(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut no_of_bits: WORD32,
) -> WORD32 {
    let mut ret_val: UWORD32 = 0;
    let mut ptr_read_next: *mut UWORD8 = (*it_bit_buff).byte_ptr;
    if no_of_bits == 0 as core::ffi::c_int {
        return 0 as WORD32;
    }
    ret_val = ixheaacd_aac_showbits_32(
        ptr_read_next,
        (*it_bit_buff).bit_count,
        0 as *mut WORD32,
    );
    (*it_bit_buff).byte_ptr = ((*it_bit_buff).byte_ptr)
        .offset((no_of_bits >> 3 as core::ffi::c_int) as isize);
    if (*it_bit_buff).valid_bits != 8 as core::ffi::c_int {
        let mut v: *mut UWORD8 = (*it_bit_buff).byte_ptr;
        ret_val = ret_val << 8 as WORD32 - (*it_bit_buff).valid_bits
            | (*v as core::ffi::c_int >> (*it_bit_buff).valid_bits) as UWORD32;
    }
    (*it_bit_buff).valid_bits -= no_of_bits as core::ffi::c_int % 8 as core::ffi::c_int;
    ret_val = ret_val >> 32 as WORD32 - no_of_bits;
    return ret_val as WORD32;
}
