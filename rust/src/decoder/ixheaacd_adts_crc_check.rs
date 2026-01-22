extern "C" {
    fn ixheaacd_read_bits_buf(
        it_bit_buff: *mut ia_bit_buf_struct,
        no_of_bits: WORD,
    ) -> WORD32;
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
pub type ia_crc_bit_buf_struct_handle = *mut ia_crc_bit_buf_struct;
pub const AAC_DEC_OK: core::ffi::c_int = IA_XHEAAC_DEC_API_NONFATAL_NO_ERROR;
pub const IA_XHEAAC_DEC_API_NONFATAL_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_ADTS_HDR_CRC_FAIL: core::ffi::c_int = 0x1803
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES: core::ffi::c_int = 0x1804
    as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_adts_crc_open(
    mut ptr_adts_crc_info: *mut ia_adts_crc_info_struct,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut val: UWORD16 = 0;
    (*ptr_adts_crc_info).no_reg = 0 as UWORD16;
    (*ptr_adts_crc_info).crc_active = 0 as UWORD8;
    i = 0 as core::ffi::c_int as WORD32;
    while i <= 255 as core::ffi::c_int {
        val = (i << 8 as core::ffi::c_int) as UWORD16;
        j = 8 as core::ffi::c_int as WORD32;
        loop {
            j -= 1;
            if !(j >= 0 as core::ffi::c_int) {
                break;
            }
            val = (if val as core::ffi::c_int & 0x8000 as core::ffi::c_int != 0 {
                (val as core::ffi::c_int) << 1 as core::ffi::c_int
                    ^ 0x8005 as core::ffi::c_int
            } else {
                (val as core::ffi::c_int) << 1 as core::ffi::c_int
            }) as UWORD16;
        }
        (*ptr_adts_crc_info).crc_lookup[i as usize] = val;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_copy_bit_buf_state(
    mut it_bit_buff_src: *mut ia_bit_buf_struct,
    mut it_crc_bit_buff_dst: ia_crc_bit_buf_struct_handle,
) -> VOID {
    (*it_crc_bit_buff_dst).ptr_bit_buf_base = (*it_bit_buff_src).ptr_bit_buf_base;
    (*it_crc_bit_buff_dst).ptr_bit_buf_end = (*it_bit_buff_src).ptr_bit_buf_end;
    (*it_crc_bit_buff_dst).ptr_read_next = (*it_bit_buff_src).ptr_read_next;
    (*it_crc_bit_buff_dst).bit_pos = (*it_bit_buff_src).bit_pos as WORD16;
    (*it_crc_bit_buff_dst).cnt_bits = (*it_bit_buff_src).cnt_bits;
    (*it_crc_bit_buff_dst).size = (*it_bit_buff_src).size;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_adts_crc_start_reg(
    mut ptr_adts_crc_info: *mut ia_adts_crc_info_struct,
    mut it_bit_buff_src: *mut ia_bit_buf_struct,
    mut no_bits: WORD32,
) -> WORD32 {
    let mut no_bytes: UWORD32 = 0;
    (*ptr_adts_crc_info)
        .str_crc_reg_data[(*ptr_adts_crc_info).no_reg as usize]
        .bit_cnt = 0 as UWORD32;
    (*ptr_adts_crc_info)
        .str_crc_reg_data[(*ptr_adts_crc_info).no_reg as usize]
        .max_bits = no_bits;
    if no_bits < 0 as core::ffi::c_int {
        no_bits = -no_bits;
    }
    if no_bits == 0 as core::ffi::c_int {
        no_bits = ((16 as core::ffi::c_int) << 3 as core::ffi::c_int) as WORD32;
    }
    no_bytes = (no_bits >> 3 as core::ffi::c_int) as UWORD32;
    if (no_bytes << 3 as core::ffi::c_int) < no_bits as UWORD32 {
        no_bytes = no_bytes.wrapping_add(1);
    }
    (*ptr_adts_crc_info)
        .str_crc_reg_data[(*ptr_adts_crc_info).no_reg as usize]
        .buf_size = no_bytes as WORD32;
    (*ptr_adts_crc_info).str_crc_reg_data[(*ptr_adts_crc_info).no_reg as usize].active = 1
        as UWORD8;
    ixheaacd_copy_bit_buf_state(
        it_bit_buff_src,
        &mut (*((*ptr_adts_crc_info).str_crc_reg_data)
            .as_mut_ptr()
            .offset((*ptr_adts_crc_info).no_reg as isize))
            .str_bit_buf,
    );
    (*ptr_adts_crc_info).no_reg = ((*ptr_adts_crc_info).no_reg as core::ffi::c_int
        + 1 as core::ffi::c_int) as UWORD16;
    return (*ptr_adts_crc_info).no_reg as WORD32 - 1 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_adts_crc_end_reg(
    mut ptr_adts_crc_info: *mut ia_adts_crc_info_struct,
    mut it_bit_buff_src: *mut ia_bit_buf_struct,
    mut reg: WORD32,
) -> VOID {
    (*ptr_adts_crc_info).str_crc_reg_data[reg as usize].active = 0 as UWORD8;
    (*ptr_adts_crc_info).str_crc_reg_data[reg as usize].bit_buf_cnt = (*ptr_adts_crc_info)
        .str_crc_reg_data[reg as usize]
        .str_bit_buf
        .cnt_bits - (*it_bit_buff_src).cnt_bits;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_adts_crc_fast_crc(
    mut ptr_adts_crc_info: *mut ia_adts_crc_info_struct,
    mut crc_reg: *mut UWORD16,
    mut feed: UWORD8,
) -> VOID {
    *crc_reg = ((*crc_reg as core::ffi::c_int) << 8 as core::ffi::c_int
        ^ (*ptr_adts_crc_info)
            .crc_lookup[(*crc_reg as core::ffi::c_int >> 8 as core::ffi::c_int
            ^ feed as core::ffi::c_int) as usize] as core::ffi::c_int) as UWORD16;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_adts_crc_slow_crc(
    mut crc_reg: *mut UWORD16,
    mut feed: UWORD8,
    mut no_bits: UWORD32,
) -> VOID {
    let mut i: UWORD32 = 0;
    let mut tmp: UWORD16 = 0;
    i = 0 as UWORD32;
    while i < no_bits {
        tmp = ((feed as core::ffi::c_int
            & (1 as core::ffi::c_int) << (7 as UWORD32).wrapping_sub(i))
            >> (7 as UWORD32).wrapping_sub(i)) as UWORD16;
        tmp = (tmp as core::ffi::c_int
            ^ (*crc_reg as core::ffi::c_int
                & (1 as core::ffi::c_int) << 15 as core::ffi::c_int)
                >> 15 as core::ffi::c_int) as UWORD16;
        tmp = (tmp as core::ffi::c_int * 32773 as core::ffi::c_int) as UWORD16;
        *crc_reg = ((*crc_reg as core::ffi::c_int) << 1 as core::ffi::c_int) as UWORD16;
        *crc_reg = (*crc_reg as core::ffi::c_int ^ tmp as core::ffi::c_int) as UWORD16;
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_adts_crc_check_crc(
    mut ptr_adts_crc_info: *mut ia_adts_crc_info_struct,
) -> WORD32 {
    let mut error_code: WORD32 = AAC_DEC_OK;
    let mut crc: UWORD16 = 65535 as UWORD16;
    let mut reg: WORD32 = 0;
    let mut ptr_reg_data: *mut ia_crc_reg_data_struct = 0 as *mut ia_crc_reg_data_struct;
    reg = 0 as core::ffi::c_int as WORD32;
    while reg < (*ptr_adts_crc_info).no_reg as core::ffi::c_int {
        let mut bits: UWORD8 = 0;
        let mut bits_remaining: WORD32 = 0;
        ptr_reg_data = &mut *((*ptr_adts_crc_info).str_crc_reg_data)
            .as_mut_ptr()
            .offset(reg as isize) as *mut ia_crc_reg_data_struct;
        if (*ptr_reg_data).max_bits > 0 as core::ffi::c_int {
            if (*ptr_adts_crc_info).str_crc_reg_data[reg as usize].bit_buf_cnt
                > (*ptr_reg_data).max_bits
            {
                bits_remaining = (*ptr_reg_data).max_bits;
            } else {
                bits_remaining = (*ptr_adts_crc_info)
                    .str_crc_reg_data[reg as usize]
                    .bit_buf_cnt;
            }
        } else {
            bits_remaining = (*ptr_adts_crc_info)
                .str_crc_reg_data[reg as usize]
                .bit_buf_cnt;
        }
        while bits_remaining >= 8 as core::ffi::c_int {
            if (*ptr_reg_data).str_bit_buf.cnt_bits < 8 as core::ffi::c_int {
                return IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES;
            }
            bits = ixheaacd_read_bits_buf(
                &mut (*((*ptr_adts_crc_info).str_crc_reg_data)
                    .as_mut_ptr()
                    .offset(reg as isize))
                    .str_bit_buf as *mut ia_crc_bit_buf_struct as *mut ia_bit_buf_struct,
                8 as WORD,
            ) as UWORD8;
            ixheaacd_adts_crc_fast_crc(ptr_adts_crc_info, &mut crc, bits);
            bits_remaining -= 8 as core::ffi::c_int;
        }
        if (*ptr_reg_data).str_bit_buf.cnt_bits < bits_remaining {
            return IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES;
        }
        bits = ixheaacd_read_bits_buf(
            &mut (*((*ptr_adts_crc_info).str_crc_reg_data)
                .as_mut_ptr()
                .offset(reg as isize))
                .str_bit_buf as *mut ia_crc_bit_buf_struct as *mut ia_bit_buf_struct,
            bits_remaining as WORD,
        ) as UWORD8;
        ixheaacd_adts_crc_slow_crc(
            &mut crc,
            ((bits as core::ffi::c_int) << 8 as WORD32 - bits_remaining) as UWORD8,
            bits_remaining as UWORD32,
        );
        if (*ptr_reg_data).max_bits
            > (*ptr_adts_crc_info).str_crc_reg_data[reg as usize].bit_buf_cnt
        {
            bits_remaining = (*ptr_reg_data).max_bits
                - (*ptr_adts_crc_info).str_crc_reg_data[reg as usize].bit_buf_cnt;
            while bits_remaining >= 8 as core::ffi::c_int {
                ixheaacd_adts_crc_fast_crc(ptr_adts_crc_info, &mut crc, 0 as UWORD8);
                bits_remaining -= 8 as core::ffi::c_int;
            }
            ixheaacd_adts_crc_slow_crc(&mut crc, 0 as UWORD8, bits_remaining as UWORD32);
        }
        reg += 1;
    }
    (*ptr_adts_crc_info).no_reg = 0 as UWORD16;
    if crc as core::ffi::c_int != (*ptr_adts_crc_info).file_value as core::ffi::c_int {
        return 0x1803 as WORD32;
    }
    return error_code;
}
