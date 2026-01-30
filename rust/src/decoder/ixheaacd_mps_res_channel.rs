extern "C" {
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn ixheaacd_read_bits_buf(
        it_bit_buff: *mut ia_bit_buf_struct,
        no_of_bits: WORD,
    ) -> WORD32;
    fn ixheaacd_res_c_pulse_data_read(
        it_bit_buf: *mut ia_bit_buf_struct,
        pulse_data: *mut ia_mps_dec_residual_pulse_data_struct,
        aac_tables_ptr: *mut ia_mps_dec_residual_aac_tables_struct,
    ) -> WORD32;
    fn ixheaacd_res_ics_read(
        it_bit_buf: *mut ia_bit_buf_struct,
        p_ics_info: *mut ia_mps_dec_residual_ics_info_struct,
        tot_sf_bands_ls: *mut WORD8,
    ) -> WORD16;
    fn ixheaacd_res_get_sfb_offsets(
        p_ics_info: *mut ia_mps_dec_residual_ics_info_struct,
        aac_tables_ptr: *mut ia_mps_dec_residual_aac_tables_struct,
    ) -> *const WORD16;
    fn ixheaacd_res_get_sfb_width(
        p_ics_info: *mut ia_mps_dec_residual_ics_info_struct,
        aac_tables_ptr: *mut ia_mps_dec_residual_aac_tables_struct,
    ) -> *const WORD8;
    fn ixheaacd_res_inverse_quant_lb(
        x_invquant: *mut WORD32,
        t_bands: WORD,
        pow_table_q17: *mut WORD32,
        pulse_data: *mut WORD8,
    ) -> VOID;
    fn ixheaacd_res_apply_scfs(
        x_invquant: *mut WORD32,
        sc_factor: *mut WORD16,
        t_bands: WORD,
        offset: *mut WORD8,
        scale_tables_ptr: *mut WORD32,
    ) -> VOID;
    fn ixheaacd_res_c_block_decode_huff_word_all(
        it_bit_buf: *mut ia_bit_buf_struct,
        code_no: WORD32,
        qp: *mut WORD32,
        offsets: *mut WORD16,
        start_band: WORD,
        endBand: WORD,
        group_no: WORD,
        aac_tables_ptr: *mut ia_mps_dec_residual_aac_tables_struct,
        maximum_bins_short: WORD32,
    ) -> WORD;
    fn ixheaacd_res_c_block_decode_huff_word_all_lb(
        it_bit_buf: *mut ia_bit_buf_struct,
        code_no: WORD32,
        len: WORD32,
        aac_tables_ptr: *mut ia_mps_dec_residual_aac_tables_struct,
        x_invquant: *mut WORD32,
        p_pul_arr: *mut WORD8,
    ) -> WORD;
    fn ixheaacd_res_c_block_read_scf_data(
        it_bit_buf: *mut ia_bit_buf_struct,
        p_aac_decoder_channel_info: *mut ia_mps_dec_residual_channel_info_struct,
        global_gain: WORD16,
        aac_tables_ptr: *mut ia_mps_dec_residual_aac_tables_struct,
    ) -> VOID;
    fn ixheaacd_c_block_read_section_data(
        it_bit_buf: *mut ia_bit_buf_struct,
        p_aac_decoder_channel_info: *mut ia_mps_dec_residual_channel_info_struct,
    ) -> WORD16;
}
pub type size_t = usize;
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
unsafe extern "C" fn ixheaac_shr16(mut op1: WORD16, mut shift: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = (op1 as core::ffi::c_int >> shift as core::ffi::c_int) as WORD16;
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_shr16_dir_sat(
    mut op1: WORD16,
    mut shift: WORD16,
) -> WORD16 {
    let mut var_out: WORD16 = 0;
    if (shift as core::ffi::c_int) < 0 as core::ffi::c_int {
        var_out = ixheaac_shl16_sat(op1, -(shift as core::ffi::c_int) as WORD16);
    } else {
        var_out = ixheaac_shr16(op1, shift);
    }
    return var_out;
}
pub const MAX_ORDER_LONG: core::ffi::c_int = 12 as core::ffi::c_int;
pub const ZERO_HCB: core::ffi::c_int = 0 as core::ffi::c_int;
pub const NOISE_HCB: core::ffi::c_int = 13 as core::ffi::c_int;
pub const MAX_WINDOWS: core::ffi::c_int = 8 as core::ffi::c_int;
pub const MAX_SFB_SHORT: core::ffi::c_int = 16 as core::ffi::c_int;
pub const LEFT: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_XHEAAC_DEC_API_NONFATAL_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES: core::ffi::c_int = 0x1804
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_EXCEEDS_MAX_HUFFDEC_VAL: core::ffi::c_int = 0x1806
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_GAIN_CONTROL_DATA_PRESENT: core::ffi::c_int = 0x1809
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_TNS_ORDER_ERROR: core::ffi::c_int = 0x180a
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_FATAL_TNS_RANGE_ERROR: core::ffi::c_uint = 0xffff9801
    as core::ffi::c_uint;
pub const AAC_DEC_OK: core::ffi::c_int = IA_XHEAAC_DEC_API_NONFATAL_NO_ERROR;
pub const AAC_DEC_UNIMPLEMENTED_GAIN_CONTROL_DATA: core::ffi::c_int = IA_XHEAAC_DEC_EXE_NONFATAL_GAIN_CONTROL_DATA_PRESENT;
pub const AAC_DEC_TNS_RANGE_ERROR: core::ffi::c_uint = IA_XHEAAC_DEC_EXE_FATAL_TNS_RANGE_ERROR;
pub const AAC_DEC_TNS_ORDER_ERROR: core::ffi::c_int = IA_XHEAAC_DEC_EXE_NONFATAL_TNS_ORDER_ERROR;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_res_aac_showbits_32(
    mut p_read_next: *mut UWORD8,
) -> UWORD32 {
    let mut v: *mut UWORD8 = p_read_next;
    let mut b: UWORD32 = 0 as UWORD32;
    b = ((*v.offset(0 as core::ffi::c_int as isize) as WORD32) << 24 as core::ffi::c_int
        | (*v.offset(1 as core::ffi::c_int as isize) as WORD32) << 16 as core::ffi::c_int
        | (*v.offset(2 as core::ffi::c_int as isize) as WORD32) << 8 as core::ffi::c_int
        | *v.offset(3 as core::ffi::c_int as isize) as WORD32) as UWORD32;
    return b;
}
unsafe extern "C" fn ixheaacd_res_c_block_read(
    mut it_bit_buf: *mut ia_bit_buf_struct,
    mut p_aac_decoder_channel_info: *mut ia_mps_dec_residual_channel_info_struct,
    mut global_gain: WORD16,
    mut aac_tables_ptr: *mut ia_mps_dec_residual_aac_tables_struct,
) -> WORD16 {
    let mut gain_control_data_present: FLAG = 0;
    let mut error_status: WORD16 = AAC_DEC_OK as WORD16;
    if (*p_aac_decoder_channel_info).ics_info.window_sequence as core::ffi::c_int
        == EIGHT_SHORT_SEQUENCE
    {
        memset(
            (*p_aac_decoder_channel_info).p_scale_factor as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (MAX_WINDOWS * MAX_SFB_SHORT * 3 as core::ffi::c_int) as size_t,
        );
    }
    error_status = ixheaacd_c_block_read_section_data(
        it_bit_buf,
        p_aac_decoder_channel_info,
    );
    if error_status != 0 {
        return error_status;
    }
    ixheaacd_res_c_block_read_scf_data(
        it_bit_buf,
        p_aac_decoder_channel_info,
        global_gain,
        aac_tables_ptr,
    );
    error_status = ixheaacd_res_c_pulse_data_read(
        it_bit_buf,
        &mut (*p_aac_decoder_channel_info).pulse_data,
        aac_tables_ptr,
    ) as WORD16;
    if error_status != 0 {
        return error_status;
    }
    (*p_aac_decoder_channel_info).tns_data.tns_data_present = ixheaacd_read_bits_buf(
        it_bit_buf,
        1 as WORD,
    );
    error_status = ixheaacd_res_c_tns_read(it_bit_buf, p_aac_decoder_channel_info);
    if error_status != 0 {
        return error_status;
    }
    gain_control_data_present = ixheaacd_read_bits_buf(it_bit_buf, 1 as WORD) as FLAG;
    if gain_control_data_present != 0 {
        return AAC_DEC_UNIMPLEMENTED_GAIN_CONTROL_DATA as WORD16;
    }
    (*it_bit_buf).bit_pos = 7 as WORD32 - (*it_bit_buf).bit_pos;
    error_status = ixheaacd_res_c_block_read_spec_data(
        it_bit_buf,
        p_aac_decoder_channel_info,
        aac_tables_ptr,
    );
    (*it_bit_buf).bit_pos = 7 as WORD32 - (*it_bit_buf).bit_pos;
    return error_status;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_res_read_ics(
    mut it_bit_buf: *mut ia_bit_buf_struct,
    mut p_aac_decoder_channel_info: *mut *mut ia_mps_dec_residual_channel_info_struct,
    mut num_ch: WORD32,
    mut aac_tables_ptr: *mut ia_mps_dec_residual_aac_tables_struct,
    mut tot_sf_bands_ls: *mut WORD8,
) -> WORD16 {
    let mut error_status: WORD16 = AAC_DEC_OK as WORD16;
    let mut ch: WORD32 = 0;
    ch = 0 as core::ffi::c_int as WORD32;
    while ch < num_ch {
        let mut p_aac_dec_ch_info: *mut ia_mps_dec_residual_channel_info_struct = *p_aac_decoder_channel_info
            .offset(ch as isize);
        let mut p_ics_info: *mut ia_mps_dec_residual_ics_info_struct = &mut (*p_aac_dec_ch_info)
            .ics_info;
        (*p_aac_dec_ch_info).global_gain = ixheaacd_read_bits_buf(it_bit_buf, 8 as WORD)
            as WORD16;
        if (**p_aac_decoder_channel_info.offset(LEFT as isize)).common_window == 0 {
            error_status = ixheaacd_res_ics_read(
                it_bit_buf,
                p_ics_info,
                tot_sf_bands_ls,
            );
            if error_status != 0 {
                if (*it_bit_buf).cnt_bits < 0 as core::ffi::c_int {
                    error_status = IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES
                        as WORD16;
                }
                return error_status;
            }
        }
        error_status = ixheaacd_res_c_block_read(
            it_bit_buf,
            p_aac_dec_ch_info,
            (*p_aac_dec_ch_info).global_gain,
            aac_tables_ptr,
        );
        if error_status != 0 {
            if (*it_bit_buf).cnt_bits < 0 as core::ffi::c_int {
                error_status = IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES
                    as WORD16;
            }
            return error_status;
        }
        ch += 1;
    }
    return error_status;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_res_c_pulse_data_apply(
    mut pulse_data: *mut ia_mps_dec_residual_pulse_data_struct,
    mut p_pulse_arr: *mut WORD8,
    mut p_scale_factor_band_offsets: *const WORD16,
) -> VOID {
    let mut i: WORD = 0;
    let mut number_pulse: WORD = 0;
    let mut k: WORD32 = 0;
    memset(
        p_pulse_arr as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<WORD32>() as size_t).wrapping_mul(256 as size_t),
    );
    if (*pulse_data).pulse_data_present != 0 {
        k = *p_scale_factor_band_offsets.offset((*pulse_data).pulse_start_band as isize)
            as WORD32;
        number_pulse = (*pulse_data).number_pulse as WORD;
        i = 0 as core::ffi::c_int as WORD;
        while i <= number_pulse {
            k = (k as core::ffi::c_int
                + (*pulse_data).pulse_offset[i as usize] as core::ffi::c_int) as WORD32;
            *p_pulse_arr.offset(k as isize) = (*pulse_data).pulse_amp[i as usize];
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_res_c_block_read_spec_data(
    mut it_bit_buf: *mut ia_bit_buf_struct,
    mut p_aac_decoder_channel_info: *mut ia_mps_dec_residual_channel_info_struct,
    mut aac_tables_ptr: *mut ia_mps_dec_residual_aac_tables_struct,
) -> WORD16 {
    let mut band: WORD = 0;
    let mut tot_bands: WORD = 0;
    let mut tot_groups: WORD = 0 as WORD;
    let mut group: WORD = 0;
    let mut groupwin: WORD = 0;
    let mut groupoffset: WORD = 0;
    let mut index: WORD = 0;
    let mut p_code_book: *mut WORD8 = 0 as *mut WORD8;
    let mut p_codebook_tmp: *mut WORD8 = 0 as *mut WORD8;
    let mut p_scale_factor: *mut WORD16 = 0 as *mut WORD16;
    let mut p_spectral_coefficient: *mut WORD32 = 0 as *mut WORD32;
    let mut p_ics_info: *mut ia_mps_dec_residual_ics_info_struct = &mut (*p_aac_decoder_channel_info)
        .ics_info;
    let mut band_offsets: *mut WORD16 = 0 as *mut WORD16;
    let mut maximum_bins_short: WORD32 = ixheaac_shr16_dir_sat(
        (*p_ics_info).frame_length,
        3 as WORD16,
    ) as WORD32;
    let mut p_spec_coeff_out: *mut WORD32 = 0 as *mut WORD32;
    p_code_book = (*p_aac_decoder_channel_info).p_code_book;
    p_scale_factor = (*p_aac_decoder_channel_info).p_scale_factor;
    p_spectral_coefficient = (*p_aac_decoder_channel_info).p_spectral_coefficient;
    tot_groups = (*p_ics_info).window_groups as WORD;
    tot_bands = (*p_ics_info).max_sf_bands as WORD;
    band_offsets = ixheaacd_res_get_sfb_offsets(p_ics_info, aac_tables_ptr)
        as *mut WORD16;
    if (*p_aac_decoder_channel_info).ics_info.window_sequence as core::ffi::c_int
        != EIGHT_SHORT_SEQUENCE
    {
        let mut p_pul_arr: *mut WORD8 = (*p_aac_decoder_channel_info).p_tns_scratch
            as *mut WORD8;
        ixheaacd_res_c_pulse_data_apply(
            &mut (*p_aac_decoder_channel_info).pulse_data,
            p_pul_arr,
            band_offsets,
        );
        p_spec_coeff_out = &mut *p_spectral_coefficient
            .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
        band = 0 as core::ffi::c_int as WORD;
        while band < tot_bands {
            let mut ret_val: WORD = 0;
            let mut len: WORD32 = 0;
            let mut code_no: WORD32 = *p_code_book.offset(band as isize) as WORD32;
            let mut start: WORD = band;
            while band < tot_bands
                && *p_code_book.offset(band as isize) as core::ffi::c_int == code_no
            {
                band += 1;
            }
            len = (*band_offsets.offset(band as isize) as core::ffi::c_int
                - *band_offsets.offset(start as isize) as core::ffi::c_int) as WORD32;
            if code_no > ZERO_HCB && code_no < NOISE_HCB {
                ret_val = ixheaacd_res_c_block_decode_huff_word_all_lb(
                    it_bit_buf,
                    code_no,
                    len,
                    aac_tables_ptr,
                    p_spec_coeff_out,
                    p_pul_arr,
                );
                if ret_val != 0 as core::ffi::c_int {
                    return IA_XHEAAC_DEC_EXE_NONFATAL_EXCEEDS_MAX_HUFFDEC_VAL as WORD16;
                }
            } else if (*p_aac_decoder_channel_info).pulse_data.pulse_data_present != 0 {
                ixheaacd_res_inverse_quant_lb(
                    p_spec_coeff_out,
                    len as WORD,
                    ((*(*aac_tables_ptr).res_block_tables_ptr).pow_table_q17)
                        .as_mut_ptr(),
                    p_pul_arr,
                );
            } else {
                memset(
                    p_spec_coeff_out as *mut core::ffi::c_void,
                    0 as core::ffi::c_int,
                    (::core::mem::size_of::<WORD32>() as size_t)
                        .wrapping_mul(len as size_t),
                );
            }
            p_pul_arr = p_pul_arr.offset(len as isize);
            p_spec_coeff_out = p_spec_coeff_out.offset(len as isize);
        }
        index = (1024 as core::ffi::c_int
            - *band_offsets.offset(tot_bands as isize) as core::ffi::c_int) as WORD;
        memset(
            p_spec_coeff_out as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<WORD32>() as size_t).wrapping_mul(index as size_t),
        );
    } else {
        memset(
            p_spectral_coefficient as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<WORD32>() as size_t).wrapping_mul(1024 as size_t),
        );
        groupoffset = 0 as core::ffi::c_int as WORD;
        group = 0 as core::ffi::c_int as WORD;
        while group < tot_groups {
            let mut grp_win: WORD = (*p_ics_info).window_group_length[group as usize]
                as WORD;
            p_codebook_tmp = &mut *p_code_book
                .offset((group as core::ffi::c_int * MAX_SFB_SHORT) as isize)
                as *mut WORD8;
            p_spec_coeff_out = &mut *p_spectral_coefficient
                .offset((groupoffset as WORD32 * maximum_bins_short) as isize)
                as *mut WORD32;
            band = 0 as core::ffi::c_int as WORD;
            while band < tot_bands {
                let mut code_no_0: WORD = *p_codebook_tmp as WORD;
                let mut start_0: WORD = band;
                let mut ret_val_0: WORD = 0;
                while band < tot_bands
                    && *p_codebook_tmp as core::ffi::c_int == code_no_0
                {
                    band += 1;
                    p_codebook_tmp = p_codebook_tmp.offset(1);
                }
                if code_no_0 > ZERO_HCB && code_no_0 < NOISE_HCB {
                    ret_val_0 = ixheaacd_res_c_block_decode_huff_word_all(
                        it_bit_buf,
                        code_no_0 as WORD32,
                        p_spec_coeff_out,
                        band_offsets,
                        start_0,
                        band,
                        grp_win,
                        aac_tables_ptr,
                        maximum_bins_short,
                    );
                    if ret_val_0 != 0 as core::ffi::c_int {
                        return IA_XHEAAC_DEC_EXE_NONFATAL_EXCEEDS_MAX_HUFFDEC_VAL
                            as WORD16;
                    }
                }
            }
            groupoffset = groupoffset + grp_win;
            group += 1;
        }
    }
    let mut p_win_grp_len: *mut WORD8 = &mut *((*p_ics_info).window_group_length)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut WORD8;
    let mut psfb_width: *mut WORD8 = ixheaacd_res_get_sfb_width(
        p_ics_info,
        aac_tables_ptr,
    ) as *mut WORD8;
    let mut scale_table_ptr: *mut WORD32 = 0 as *mut WORD32;
    if 120 as core::ffi::c_int == maximum_bins_short {
        scale_table_ptr = ((*(*aac_tables_ptr).res_block_tables_ptr).scale_table_960)
            .as_mut_ptr();
    } else {
        scale_table_ptr = ((*(*aac_tables_ptr).res_block_tables_ptr).scale_table)
            .as_mut_ptr();
    }
    loop {
        let fresh0 = p_win_grp_len;
        p_win_grp_len = p_win_grp_len.offset(1);
        groupwin = *fresh0 as WORD;
        loop {
            ixheaacd_res_apply_scfs(
                &mut *p_spectral_coefficient.offset(0 as core::ffi::c_int as isize),
                &mut *p_scale_factor.offset(0 as core::ffi::c_int as isize),
                tot_bands,
                psfb_width,
                scale_table_ptr,
            );
            p_spectral_coefficient = p_spectral_coefficient
                .offset(maximum_bins_short as isize);
            groupwin -= 1;
            if !(groupwin != 0 as core::ffi::c_int) {
                break;
            }
        }
        p_scale_factor = p_scale_factor.offset(MAX_SFB_SHORT as isize);
        tot_groups -= 1;
        if !(tot_groups != 0 as core::ffi::c_int) {
            break;
        }
    }
    return AAC_DEC_OK as WORD16;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_res_c_tns_read(
    mut it_bit_buf: *mut ia_bit_buf_struct,
    mut p_aac_decoder_channel_info: *mut ia_mps_dec_residual_channel_info_struct,
) -> WORD16 {
    let mut window: WORD = 0;
    let mut window_per_frame: WORD = 0;
    let mut n_filt_bits: WORD = 0;
    let mut len_bits: WORD = 0;
    let mut order_bits: WORD = 0;
    let mut next_stop_band_tmp: WORD32 = 0;
    let mut p_ics_info: *mut ia_mps_dec_residual_ics_info_struct = &mut (*p_aac_decoder_channel_info)
        .ics_info;
    let mut p_tns_data: *mut ia_mps_dec_residual_tns_data = &mut (*p_aac_decoder_channel_info)
        .tns_data;
    if (*p_tns_data).tns_data_present == 0 {
        return AAC_DEC_OK as WORD16;
    }
    if (*p_ics_info).window_sequence as core::ffi::c_int != EIGHT_SHORT_SEQUENCE {
        n_filt_bits = 2 as core::ffi::c_int as WORD;
        len_bits = 6 as core::ffi::c_int as WORD;
        order_bits = 5 as core::ffi::c_int as WORD;
        window_per_frame = 1 as core::ffi::c_int as WORD;
    } else {
        n_filt_bits = 1 as core::ffi::c_int as WORD;
        len_bits = 4 as core::ffi::c_int as WORD;
        order_bits = 3 as core::ffi::c_int as WORD;
        window_per_frame = 8 as core::ffi::c_int as WORD;
    }
    next_stop_band_tmp = (*p_ics_info).total_sf_bands as WORD32;
    window = 0 as core::ffi::c_int as WORD;
    while window < window_per_frame {
        let mut n_filt: WORD = 0;
        let mut length: WORD = 0;
        let mut coef_res: WORD = 0;
        n_filt = ixheaacd_read_bits_buf(it_bit_buf, n_filt_bits) as WORD16 as WORD;
        (*p_tns_data).number_of_filters[window as usize] = n_filt as WORD8;
        if n_filt != 0 {
            let mut accu: WORD32 = 0;
            let mut index: WORD = 0;
            let mut nextstopband: WORD = 0;
            coef_res = ixheaacd_read_bits_buf(it_bit_buf, 1 as WORD) as WORD;
            nextstopband = next_stop_band_tmp as WORD;
            index = 0 as core::ffi::c_int as WORD;
            while index < n_filt {
                let mut order: WORD = 0;
                let mut filter: *mut ia_mps_dec_residual_filter_struct = &mut *(*((*p_tns_data)
                    .filter)
                    .as_mut_ptr()
                    .offset(window as isize))
                    .as_mut_ptr()
                    .offset(index as isize) as *mut ia_mps_dec_residual_filter_struct;
                length = ixheaacd_read_bits_buf(it_bit_buf, len_bits) as WORD;
                (*filter).start_band = (nextstopband - length) as WORD16;
                (*filter).stop_band = nextstopband as WORD16;
                nextstopband = (*filter).start_band as WORD;
                if ((*filter).start_band as core::ffi::c_int) < 0 as core::ffi::c_int {
                    (*filter).order = -(1 as core::ffi::c_int) as WORD8;
                    return AAC_DEC_TNS_RANGE_ERROR as WORD32 as WORD16;
                }
                order = ixheaacd_read_bits_buf(it_bit_buf, order_bits) as WORD;
                (*filter).order = order as WORD8;
                accu = (order as core::ffi::c_int - MAX_ORDER_LONG) as WORD32;
                if accu > 0 as core::ffi::c_int {
                    return AAC_DEC_TNS_ORDER_ERROR as WORD16;
                }
                if order != 0 {
                    let mut i: WORD = 0;
                    let mut coef: WORD32 = 0;
                    let mut coef_compress: WORD32 = 0;
                    let mut resolution: WORD = 0;
                    let mut shift: WORD = 0;
                    (*filter).direction = (if ixheaacd_read_bits_buf(
                        it_bit_buf,
                        1 as WORD,
                    ) != 0
                    {
                        -(1 as core::ffi::c_int)
                    } else {
                        1 as core::ffi::c_int
                    }) as WORD8;
                    coef_compress = ixheaacd_read_bits_buf(it_bit_buf, 1 as WORD);
                    (*filter).resolution = coef_res as WORD8;
                    resolution = (coef_res as WORD32 + 3 as WORD32 - coef_compress)
                        as WORD;
                    shift = 32 as WORD - resolution;
                    i = 0 as core::ffi::c_int as WORD;
                    while i < order {
                        coef = ixheaacd_read_bits_buf(it_bit_buf, resolution);
                        coef = coef << shift;
                        (*filter).coeff[i as usize] = (coef >> shift) as WORD8;
                        i += 1;
                    }
                }
                index += 1;
            }
        }
        window += 1;
    }
    return AAC_DEC_OK as WORD16;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_res_inv_quant(
    mut px_quant: *mut WORD32,
    mut pow_table_q17: *mut WORD32,
) -> WORD32 {
    let mut q1: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut q_abs: WORD32 = 0;
    let mut interp: WORD16 = 0;
    let mut shift: WORD32 = 0;
    q_abs = *px_quant;
    if q_abs > 8191 as core::ffi::c_int + 32 as core::ffi::c_int {
        return IA_XHEAAC_DEC_EXE_NONFATAL_EXCEEDS_MAX_HUFFDEC_VAL;
    }
    if q_abs < 1024 as core::ffi::c_int {
        shift = 3 as core::ffi::c_int as WORD32;
    } else {
        shift = 6 as core::ffi::c_int as WORD32;
    }
    q1 = q_abs >> shift;
    interp = (q_abs - (q1 << shift)) as WORD16;
    temp = *pow_table_q17
        .offset((q1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
        - *pow_table_q17.offset(q1 as isize);
    temp = temp * interp as WORD32;
    temp = temp + (*pow_table_q17.offset(q1 as isize) << shift);
    if shift == 3 as core::ffi::c_int {
        temp = temp << 1 as core::ffi::c_int;
    } else {
        temp = temp << 2 as core::ffi::c_int;
    }
    *px_quant = temp;
    return 0 as WORD32;
}
