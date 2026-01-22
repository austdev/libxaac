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
pub struct ia_mps_dec_residual_ics_info_struct {
    pub window_sequence: WORD16,
    pub max_sf_bands: WORD16,
    pub total_sf_bands: WORD16,
    pub sampling_rate_index: WORD16,
    pub window_groups: WORD16,
    pub window_group_length: [WORD8; 8],
    pub frame_length: WORD16,
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
pub const EIGHT_SHORT_SEQUENCE: core::ffi::c_int = 2 as core::ffi::c_int;
pub const IA_XHEAAC_DEC_API_NONFATAL_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_EXCEEDS_SFB_TRANSMITTED: core::ffi::c_int = 0x1808
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_PREDICTION_DATA_PRESENT: core::ffi::c_int = 0x180b
    as core::ffi::c_int;
pub const AAC_DEC_OK: core::ffi::c_int = IA_XHEAAC_DEC_API_NONFATAL_NO_ERROR;
pub const AAC_DEC_PREDICTION_NOT_SUPPORTED_IN_LC_AAC: core::ffi::c_int = IA_XHEAAC_DEC_EXE_NONFATAL_PREDICTION_DATA_PRESENT;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_res_get_sfb_offsets(
    mut p_ics_info: *mut ia_mps_dec_residual_ics_info_struct,
    mut aac_tables_ptr: *mut ia_mps_dec_residual_aac_tables_struct,
) -> *const WORD16 {
    if (*p_ics_info).window_sequence as core::ffi::c_int != EIGHT_SHORT_SEQUENCE {
        return (*aac_tables_ptr).sfb_index_long
    } else {
        return (*aac_tables_ptr).sfb_index_short
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_res_get_sfb_width(
    mut p_ics_info: *mut ia_mps_dec_residual_ics_info_struct,
    mut aac_tables_ptr: *mut ia_mps_dec_residual_aac_tables_struct,
) -> *const WORD8 {
    if (*p_ics_info).window_sequence as core::ffi::c_int != EIGHT_SHORT_SEQUENCE {
        return (*aac_tables_ptr).sfb_index_long_width
    } else {
        return (*aac_tables_ptr).sfb_index_short_width
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_res_ics_read(
    mut it_bif_buf: *mut ia_bit_buf_struct,
    mut p_ics_info: *mut ia_mps_dec_residual_ics_info_struct,
    mut tot_sf_bands_ls: *mut WORD8,
) -> WORD16 {
    let mut i: WORD = 0;
    let mut mask: WORD = 0;
    let mut tmp: WORD = 0 as WORD;
    tmp = ixheaacd_read_bits_buf(it_bif_buf, 4 as WORD) as WORD;
    (*p_ics_info).window_sequence = ((tmp as core::ffi::c_int & 0x6 as core::ffi::c_int)
        >> 1 as core::ffi::c_int) as WORD16;
    if (*p_ics_info).window_sequence as core::ffi::c_int != EIGHT_SHORT_SEQUENCE {
        (*p_ics_info).total_sf_bands = *tot_sf_bands_ls
            .offset(0 as core::ffi::c_int as isize) as WORD16;
        (*p_ics_info).window_groups = 1 as WORD16;
        (*p_ics_info).window_group_length[0 as core::ffi::c_int as usize] = 1 as WORD8;
        tmp = ixheaacd_read_bits_buf(it_bif_buf, 7 as WORD) as WORD;
        (*p_ics_info).max_sf_bands = ((tmp as core::ffi::c_int
            & 0x7e as core::ffi::c_int) >> 1 as core::ffi::c_int) as WORD16;
        if tmp as core::ffi::c_int & 1 as core::ffi::c_int != 0 {
            return AAC_DEC_PREDICTION_NOT_SUPPORTED_IN_LC_AAC as WORD16;
        }
    } else {
        let mut win_grp: WORD32 = 0 as WORD32;
        let mut tmp2: WORD32 = 0;
        (*p_ics_info).total_sf_bands = *tot_sf_bands_ls
            .offset(1 as core::ffi::c_int as isize) as WORD16;
        tmp = ixheaacd_read_bits_buf(it_bif_buf, 11 as WORD) as WORD;
        (*p_ics_info).max_sf_bands = ((tmp as core::ffi::c_int
            & 0x780 as core::ffi::c_int) >> 7 as core::ffi::c_int) as WORD16;
        tmp2 = (tmp as core::ffi::c_int & 0x7f as core::ffi::c_int) as WORD32;
        i = 0 as core::ffi::c_int as WORD;
        while i < 7 as core::ffi::c_int {
            mask = ((1 as core::ffi::c_int) << 6 as WORD - i) as WORD;
            (*p_ics_info).window_group_length[i as usize] = 1 as WORD8;
            if tmp2 as WORD & mask != 0 {
                (*p_ics_info).window_group_length[win_grp as usize] = ((*p_ics_info)
                    .window_group_length[win_grp as usize] as core::ffi::c_int
                    + 1 as core::ffi::c_int) as WORD8;
            } else {
                win_grp = (win_grp as core::ffi::c_int + 1 as core::ffi::c_int)
                    as WORD32;
            }
            i += 1;
        }
        (*p_ics_info).window_group_length[7 as core::ffi::c_int as usize] = 1 as WORD8;
        (*p_ics_info).window_groups = (win_grp as core::ffi::c_int
            + 1 as core::ffi::c_int) as WORD16;
    }
    if (*p_ics_info).max_sf_bands as core::ffi::c_int
        > (*p_ics_info).total_sf_bands as core::ffi::c_int
    {
        return IA_XHEAAC_DEC_EXE_NONFATAL_EXCEEDS_SFB_TRANSMITTED as WORD16;
    }
    return AAC_DEC_OK as WORD16;
}
