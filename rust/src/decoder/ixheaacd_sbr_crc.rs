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
pub type FLAG = core::ffi::c_int;
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
pub const SBR_CRC_POLY: core::ffi::c_int = 0x233 as core::ffi::c_int;
pub const SBR_CYC_REDCY_CHK_BITS: core::ffi::c_int = 10 as core::ffi::c_int;
unsafe extern "C" fn ixheaacd_calc_chk_sum(
    mut crc_state: *mut WORD16,
    mut stream_data: WORD32,
    mut num_bits: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut data_bit_mask: WORD32 = (1 as WORD32)
        << num_bits as core::ffi::c_int - 1 as core::ffi::c_int;
    let mut crc10: WORD16 = SBR_CRC_POLY as WORD16;
    let mut crc_mask: WORD16 = ((1 as core::ffi::c_int) << 9 as core::ffi::c_int)
        as WORD16;
    let mut crc_state_local: WORD16 = *crc_state;
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_bits {
        let mut bit0: WORD32 = 0;
        let mut bit1: WORD32 = 0;
        bit0 = (if crc_state_local as core::ffi::c_int & crc_mask as core::ffi::c_int
            != 0
        {
            1 as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        }) as WORD32;
        bit1 = (if data_bit_mask & stream_data != 0 {
            1 as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        }) as WORD32;
        bit0 ^= bit1;
        crc_state_local = ((crc_state_local as core::ffi::c_int
            & 0xffff as core::ffi::c_int) << 1 as core::ffi::c_int) as WORD16;
        if bit0 != 0 {
            crc_state_local = (crc_state_local as core::ffi::c_int
                ^ crc10 as core::ffi::c_int) as WORD16;
        }
        data_bit_mask = data_bit_mask >> 1 as core::ffi::c_int;
        i += 1;
    }
    *crc_state = crc_state_local;
}
#[inline]
unsafe extern "C" fn ixheaacd_sbr_crc(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut num_crc_bits: WORD32,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut num_full_bytes: WORD32 = 0;
    let mut rem_bits: WORD32 = 0;
    let mut byte_value: WORD32 = 0;
    let mut crc_state: WORD16 = 0 as WORD16;
    num_full_bytes = num_crc_bits >> 3 as core::ffi::c_int;
    rem_bits = (num_crc_bits as core::ffi::c_int & 0x7 as core::ffi::c_int) as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_full_bytes {
        byte_value = ixheaacd_read_bits_buf(it_bit_buff, 8 as WORD);
        ixheaacd_calc_chk_sum(&mut crc_state, byte_value, 8 as WORD32);
        i += 1;
    }
    byte_value = ixheaacd_read_bits_buf(it_bit_buff, rem_bits as WORD);
    ixheaacd_calc_chk_sum(&mut crc_state, byte_value, rem_bits);
    return crc_state as WORD32 & 0x3ff as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_sbr_crccheck(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut crc_bits_len: WORD32,
) -> FLAG {
    let mut it_bit_buff_local: ia_bit_buf_struct = {
        let mut init = ia_bit_buf_struct {
            ptr_bit_buf_base: 0 as *mut UWORD8,
            ptr_bit_buf_end: 0 as *mut UWORD8,
            ptr_read_next: 0 as *mut UWORD8,
            bit_pos: 0,
            cnt_bits: 0,
            size: 0,
            adts_header_present: 0,
            crc_check: 0,
            protection_absent: 0,
            no_raw_data_blocks: 0,
            str_adts_crc_info: ia_adts_crc_info_struct {
                crc_active: 0,
                no_reg: 0,
                file_value: 0,
                crc_lookup: [0; 256],
                str_crc_reg_data: [ia_crc_reg_data_struct {
                    active: 0,
                    buf_size: 0,
                    max_bits: 0,
                    bit_cnt: 0,
                    bit_buf_cnt: 0,
                    str_bit_buf: ia_crc_bit_buf_struct {
                        ptr_bit_buf_base: 0 as *mut UWORD8,
                        ptr_bit_buf_end: 0 as *mut UWORD8,
                        ptr_read_next: 0 as *mut UWORD8,
                        bit_pos: 0,
                        cnt_bits: 0,
                        size: 0,
                    },
                }; 7],
            },
            pstr_adts_crc_info: 0 as *mut ia_adts_crc_info_struct,
            initial_cnt_bits: 0,
            audio_mux_align: 0,
            bit_count: 0,
            valid_bits: 0,
            byte: 0,
            byte_ptr: 0 as *mut UWORD8,
            ptr_start: 0 as *mut UWORD8,
            write_bit_count: 0,
            max_size: 0,
            xaac_jmp_buf: 0 as *mut jmp_buf,
        };
        init
    };
    let mut num_crc_bits: WORD32 = 0;
    let mut calc_crc_sum: WORD32 = 0;
    let mut bits_available: WORD32 = 0;
    let mut crc_check_sum: WORD32 = 0;
    crc_check_sum = ixheaacd_read_bits_buf(it_bit_buff, SBR_CYC_REDCY_CHK_BITS);
    it_bit_buff_local = *it_bit_buff as ia_bit_buf_struct;
    bits_available = (*it_bit_buff).cnt_bits;
    if bits_available <= 0 as core::ffi::c_int {
        return 0 as FLAG;
    }
    num_crc_bits = if crc_bits_len > bits_available {
        bits_available
    } else {
        crc_bits_len
    };
    calc_crc_sum = ixheaacd_sbr_crc(&mut it_bit_buff_local, num_crc_bits);
    if calc_crc_sum != crc_check_sum {
        return 0 as FLAG;
    }
    return 1 as FLAG;
}
