extern "C" {
    fn ixheaacd_read_bits_buf(
        it_bit_buff: *mut ia_bit_buf_struct,
        no_of_bits: WORD,
    ) -> WORD32;
    fn ixheaacd_aac_read_byte_corr(
        ptr_read_next: *mut *mut UWORD8,
        ptr_bit_pos: *mut WORD32,
        readword: *mut WORD32,
        p_bit_buf_end: *mut UWORD8,
    ) -> VOID;
    fn ixheaacd_res_aac_showbits_32(p_read_next: *mut UWORD8) -> UWORD32;
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
pub struct ia_mps_dec_residual_filter_struct {
    pub start_band: WORD16,
    pub stop_band: WORD16,
    pub direction: WORD8,
    pub resolution: WORD8,
    pub order: WORD8,
    pub coeff: [WORD8; 31],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_residual_tns_data {
    pub tns_data_present: FLAG,
    pub number_of_filters: [WORD8; 8],
    pub filter: [[ia_mps_dec_residual_filter_struct; 3]; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_residual_pulse_data_struct {
    pub pulse_data_present: FLAG,
    pub number_pulse: WORD16,
    pub pulse_start_band: WORD16,
    pub pulse_offset: [WORD8; 4],
    pub pulse_amp: [WORD8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_residual_pns_data_struct {
    pub pns_used: [UWORD8; 128],
    pub current_energy: WORD16,
    pub pns_active: UWORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_residual_channel_info_struct {
    pub p_scale_factor: *mut WORD16,
    pub p_code_book: *mut WORD8,
    pub p_spectral_coefficient: *mut WORD32,
    pub ics_info: ia_mps_dec_residual_ics_info_struct,
    pub tns_data: ia_mps_dec_residual_tns_data,
    pub pulse_data: ia_mps_dec_residual_pulse_data_struct,
    pub pns_data: ia_mps_dec_residual_pns_data_struct,
    pub common_window: WORD16,
    pub global_gain: WORD16,
    pub p_tns_scratch: *mut WORD32,
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
pub const MAX_16: WORD16 = 0x7fff as core::ffi::c_int as WORD16;
pub const EIGHT_SHORT_SEQUENCE: core::ffi::c_int = 2 as core::ffi::c_int;
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
unsafe extern "C" fn ixheaac_add16_sat(mut op1: WORD16, mut op2: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    let mut sum: WORD32 = 0;
    sum = op1 as WORD32 + op2 as WORD32;
    var_out = ixheaac_sat16(sum);
    return var_out;
}
pub const ZERO_HCB: core::ffi::c_int = 0 as core::ffi::c_int;
pub const NOISE_OFFSET: core::ffi::c_int = 90 as core::ffi::c_int;
pub const NOISE_HCB: core::ffi::c_int = 13 as core::ffi::c_int;
pub const BOOKSCL: core::ffi::c_int = 12 as core::ffi::c_int;
pub const MAX_SFB_SHORT: core::ffi::c_int = 16 as core::ffi::c_int;
pub const IA_XHEAAC_DEC_API_NONFATAL_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_EXCEEDS_SFB_TRANSMITTED: core::ffi::c_int = 0x1808
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_INVALID_CODE_BOOK: core::ffi::c_int = 0x180e
    as core::ffi::c_int;
pub const AAC_DEC_OK: core::ffi::c_int = IA_XHEAAC_DEC_API_NONFATAL_NO_ERROR;
pub const AAC_DEC_INVALID_CODE_BOOK: core::ffi::c_int = IA_XHEAAC_DEC_EXE_NONFATAL_INVALID_CODE_BOOK;
#[inline]
unsafe extern "C" fn ixheaacd_aac_read_2bytes(
    mut p_read_next: *mut *mut UWORD8,
    mut r_bit_pos: *mut WORD32,
    mut readword: *mut WORD32,
) -> UWORD32 {
    let mut v: *mut UWORD8 = *p_read_next;
    let mut bits_consumed: WORD32 = *r_bit_pos;
    if bits_consumed as core::ffi::c_int - 16 as core::ffi::c_int
        >= 0 as core::ffi::c_int
    {
        *readword = (*readword << 8 as core::ffi::c_int | *v as core::ffi::c_int)
            as WORD32;
        v = v.offset(1);
        *readword = (*readword << 8 as core::ffi::c_int | *v as core::ffi::c_int)
            as WORD32;
        v = v.offset(1);
        bits_consumed -= 16 as core::ffi::c_int;
    } else if bits_consumed as core::ffi::c_int - 8 as core::ffi::c_int
        >= 0 as core::ffi::c_int
    {
        *readword = (*readword << 8 as core::ffi::c_int | *v as core::ffi::c_int)
            as WORD32;
        v = v.offset(1);
        bits_consumed -= 8 as core::ffi::c_int;
    }
    *r_bit_pos = bits_consumed;
    *p_read_next = v;
    return 1 as UWORD32;
}
pub const LONG_BLOCK_SECT_LEN: core::ffi::c_int = 5 as core::ffi::c_int;
pub const SHORT_BLOCK_SECT_LEN: core::ffi::c_int = 3 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_c_block_read_section_data(
    mut it_bit_buf: *mut ia_bit_buf_struct,
    mut p_aac_decoder_channel_info: *mut ia_mps_dec_residual_channel_info_struct,
) -> WORD16 {
    let mut band: WORD = 0;
    let mut sect_cb: WORD = 0;
    let mut sect_len: WORD = 0;
    let mut sect_len_incr: WORD = 0;
    let mut sect_esc_val: WORD = 0;
    let mut p_ics_info: *mut ia_mps_dec_residual_ics_info_struct = &mut (*p_aac_decoder_channel_info)
        .ics_info;
    let mut sfb_transmitted: WORD = (*p_ics_info).max_sf_bands as WORD;
    let mut win_group: WORD = (*p_ics_info).window_groups as WORD;
    let mut p_code_book: *mut WORD8 = (*p_aac_decoder_channel_info).p_code_book;
    let mut p_code_book_temp: *mut WORD8 = p_code_book;
    let mut sect_bitlen: WORD32 = LONG_BLOCK_SECT_LEN;
    if (*p_aac_decoder_channel_info).ics_info.window_sequence as core::ffi::c_int
        == EIGHT_SHORT_SEQUENCE
    {
        sect_bitlen = SHORT_BLOCK_SECT_LEN as WORD32;
    }
    sect_esc_val = (((1 as core::ffi::c_int) << sect_bitlen) - 1 as core::ffi::c_int)
        as WORD;
    loop {
        band = 0 as core::ffi::c_int as WORD;
        while band < sfb_transmitted {
            let mut temp_word: WORD32 = 0;
            sect_len = 0 as core::ffi::c_int as WORD;
            temp_word = ixheaacd_read_bits_buf(
                it_bit_buf,
                4 as WORD + sect_bitlen as WORD,
            );
            sect_cb = (temp_word >> sect_bitlen) as WORD;
            sect_len_incr = temp_word as WORD & sect_esc_val;
            while sect_len_incr == sect_esc_val {
                sect_len = sect_len + sect_esc_val;
                sect_len_incr = ixheaacd_read_bits_buf(it_bit_buf, sect_bitlen as WORD)
                    as WORD;
            }
            sect_len = sect_len + sect_len_incr;
            band = band + sect_len;
            if band > sfb_transmitted {
                return IA_XHEAAC_DEC_EXE_NONFATAL_EXCEEDS_SFB_TRANSMITTED as WORD16;
            }
            if sect_cb == BOOKSCL {
                return AAC_DEC_INVALID_CODE_BOOK as WORD16;
            }
            sect_len = (sect_len as core::ffi::c_int - 1 as core::ffi::c_int) as WORD;
            while sect_len >= 0 as core::ffi::c_int {
                let fresh0 = p_code_book_temp;
                p_code_book_temp = p_code_book_temp.offset(1);
                *fresh0 = sect_cb as WORD8;
                sect_len -= 1;
            }
        }
        p_code_book = p_code_book.offset(MAX_SFB_SHORT as isize);
        p_code_book_temp = p_code_book;
        win_group -= 1;
        if !(win_group != 0 as core::ffi::c_int) {
            break;
        }
    }
    return AAC_DEC_OK as WORD16;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_res_c_block_read_scf_data(
    mut it_bit_buf: *mut ia_bit_buf_struct,
    mut p_aac_decoder_channel_info: *mut ia_mps_dec_residual_channel_info_struct,
    mut global_gain: WORD16,
    mut aac_tables_ptr: *mut ia_mps_dec_residual_aac_tables_struct,
) -> VOID {
    let mut band: WORD = 0;
    let mut position: WORD16 = 0 as WORD16;
    let mut group: WORD = 0;
    let mut factor: WORD16 = global_gain;
    let mut p_code_book: *mut WORD8 = 0 as *mut WORD8;
    let mut p_codebook_tmp: *mut WORD8 = 0 as *mut WORD8;
    let mut p_scale_factor: *mut WORD16 = 0 as *mut WORD16;
    let mut p_scale_factor_tmp: *mut WORD16 = 0 as *mut WORD16;
    let mut norm_value: WORD16 = 0;
    let mut p_ics_info: *mut ia_mps_dec_residual_ics_info_struct = 0
        as *mut ia_mps_dec_residual_ics_info_struct;
    let mut window_groups: WORD = 0;
    let mut sfb_transmitted: WORD = 0;
    let mut h: *mut UWORD16 = 0 as *mut UWORD16;
    let mut hscf: *const UWORD16 = &mut *((*(*aac_tables_ptr).res_huffmann_tables_ptr)
        .huffman_code_book_scl)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as isize) as *mut UWORD16;
    let mut start_bit_pos: WORD = (*it_bit_buf).bit_pos as WORD;
    let mut start_read_pos: *mut UWORD8 = (*it_bit_buf).ptr_read_next;
    let mut ptr_read_next: *mut UWORD8 = (*it_bit_buf).ptr_read_next;
    let mut bit_pos: WORD32 = 7 as WORD32 - (*it_bit_buf).bit_pos;
    let mut read_word: WORD32 = 0;
    let mut diffbytes: WORD32 = 0;
    diffbytes = ((*it_bit_buf).ptr_bit_buf_end).offset_from(ptr_read_next)
        as core::ffi::c_long as WORD32;
    diffbytes += 1;
    if diffbytes >= 4 as core::ffi::c_int {
        read_word = ixheaacd_res_aac_showbits_32(ptr_read_next) as WORD32;
        diffbytes = 4 as core::ffi::c_int as WORD32;
        ptr_read_next = ((*it_bit_buf).ptr_read_next)
            .offset(4 as core::ffi::c_int as isize);
    } else {
        let mut ii: WORD32 = 0;
        read_word = 0 as core::ffi::c_int as WORD32;
        ii = 0 as core::ffi::c_int as WORD32;
        while ii < diffbytes {
            read_word = ((read_word as core::ffi::c_int) << 8 as core::ffi::c_int
                | *ptr_read_next as core::ffi::c_int) as WORD32;
            ptr_read_next = ptr_read_next.offset(1);
            ii += 1;
        }
        read_word <<= 4 as WORD32 - diffbytes << 3 as core::ffi::c_int;
    }
    p_code_book = (*p_aac_decoder_channel_info).p_code_book;
    p_ics_info = &mut (*p_aac_decoder_channel_info).ics_info;
    sfb_transmitted = (*p_ics_info).max_sf_bands as WORD;
    p_scale_factor = (*p_aac_decoder_channel_info).p_scale_factor;
    window_groups = (*p_ics_info).window_groups as WORD;
    band = (sfb_transmitted as core::ffi::c_int - 1 as core::ffi::c_int) as WORD;
    group = 0 as core::ffi::c_int as WORD;
    while group < window_groups {
        p_codebook_tmp = &mut *p_code_book
            .offset((group as core::ffi::c_int * MAX_SFB_SHORT) as isize) as *mut WORD8;
        p_scale_factor_tmp = &mut *p_scale_factor
            .offset((group as core::ffi::c_int * MAX_SFB_SHORT) as isize) as *mut WORD16;
        band = (sfb_transmitted as core::ffi::c_int - 1 as core::ffi::c_int) as WORD;
        while band >= 0 as core::ffi::c_int {
            let fresh1 = p_codebook_tmp;
            p_codebook_tmp = p_codebook_tmp.offset(1);
            let mut cb_num: WORD32 = *fresh1 as WORD32;
            if cb_num == ZERO_HCB {
                let fresh2 = p_scale_factor_tmp;
                p_scale_factor_tmp = p_scale_factor_tmp.offset(1);
                *fresh2 = 0 as WORD16;
            } else {
                let mut flag: WORD32 = 1 as WORD32;
                let mut pns_band: WORD = 0;
                let mut p_pns_data: *mut ia_mps_dec_residual_pns_data_struct = &mut (*p_aac_decoder_channel_info)
                    .pns_data;
                if cb_num == NOISE_HCB
                    && (*p_pns_data).pns_active as core::ffi::c_int
                        != 1 as core::ffi::c_int
                {
                    flag = 0 as core::ffi::c_int as WORD32;
                }
                if flag != 0 {
                    let mut first_offset: UWORD16 = 0;
                    let mut sign_ret_val: WORD16 = 0;
                    let mut read_word1: UWORD32 = 0;
                    read_word1 = (read_word << bit_pos) as UWORD32;
                    h = hscf as *mut UWORD16;
                    first_offset = 7 as UWORD16;
                    h = h
                        .offset(
                            (read_word1
                                >> 32 as core::ffi::c_int
                                    - first_offset as core::ffi::c_int) as isize,
                        );
                    sign_ret_val = *h as WORD16;
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
                        += (sign_ret_val as core::ffi::c_int
                            & 0x7fff as core::ffi::c_int) >> 11 as core::ffi::c_int;
                    ixheaacd_aac_read_byte_corr(
                        &mut ptr_read_next,
                        &mut bit_pos,
                        &mut read_word,
                        (*it_bit_buf).ptr_bit_buf_end,
                    );
                    norm_value = ((sign_ret_val as core::ffi::c_int
                        & 0x7ff as core::ffi::c_int) - 60 as core::ffi::c_int) as WORD16;
                } else {
                    let mut noise_start_value: WORD32 = 0;
                    let mut temp: UWORD32 = 0;
                    temp = (read_word << bit_pos) as UWORD32;
                    temp = temp >> 32 as core::ffi::c_int - 9 as core::ffi::c_int;
                    noise_start_value = temp as WORD32;
                    bit_pos += 9 as core::ffi::c_int;
                    ixheaacd_aac_read_2bytes(
                        &mut ptr_read_next,
                        &mut bit_pos,
                        &mut read_word,
                    );
                    norm_value = (noise_start_value as core::ffi::c_int
                        - 256 as core::ffi::c_int) as WORD16;
                    (*p_pns_data).pns_active = 1 as UWORD16;
                    (*p_pns_data).current_energy = (global_gain as core::ffi::c_int
                        - NOISE_OFFSET) as WORD16;
                }
                if cb_num > NOISE_HCB {
                    position = (position as core::ffi::c_int
                        + norm_value as core::ffi::c_int) as WORD16;
                    let fresh3 = p_scale_factor_tmp;
                    p_scale_factor_tmp = p_scale_factor_tmp.offset(1);
                    *fresh3 = -(position as core::ffi::c_int) as WORD16;
                } else if cb_num < NOISE_HCB {
                    factor = (factor as core::ffi::c_int
                        + norm_value as core::ffi::c_int) as WORD16;
                    let fresh4 = p_scale_factor_tmp;
                    p_scale_factor_tmp = p_scale_factor_tmp.offset(1);
                    *fresh4 = factor;
                } else {
                    (*p_pns_data).current_energy = ixheaac_add16_sat(
                        (*p_pns_data).current_energy,
                        norm_value,
                    );
                    pns_band = (((group as core::ffi::c_int) << 4 as core::ffi::c_int)
                        + sfb_transmitted as core::ffi::c_int - band as core::ffi::c_int
                        - 1 as core::ffi::c_int) as WORD;
                    *((*p_aac_decoder_channel_info).p_scale_factor)
                        .offset(pns_band as isize) = (*p_pns_data).current_energy;
                    (*p_pns_data).pns_used[pns_band as usize] = 1 as UWORD8;
                    p_scale_factor_tmp = p_scale_factor_tmp.offset(1);
                }
            }
            band -= 1;
        }
        group += 1;
    }
    (*it_bit_buf).ptr_read_next = ptr_read_next.offset(-(diffbytes as isize));
    (*it_bit_buf).bit_pos = 7 as WORD32 - bit_pos;
    let mut bits_cons: WORD = 0;
    bits_cons = (((((*it_bit_buf).ptr_read_next).offset_from(start_read_pos)
        as core::ffi::c_long) << 3 as core::ffi::c_int)
        + (start_bit_pos as WORD32 - (*it_bit_buf).bit_pos) as core::ffi::c_long)
        as WORD;
    (*it_bit_buf).cnt_bits -= bits_cons as core::ffi::c_int;
}
