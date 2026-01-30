extern "C" {
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn longjmp(__env: *mut __jmp_buf_tag, __val: core::ffi::c_int) -> !;
    fn ixheaacd_read_bits_buf(
        it_bit_buff: *mut ia_bit_buf_struct,
        no_of_bits: WORD,
    ) -> WORD32;
    fn ixheaacd_read_bidirection(
        it_bit_buff: *mut ia_bit_buf_struct,
        ixheaacd_drc_offset: WORD32,
    ) -> VOID;
    fn ixheaacd_dec_drc_read_element(
        pstr_drc_dec: *mut ia_drc_dec_struct,
        drc_dummy: *mut ia_drc_dec_struct,
        it_bit_buf: *mut ia_bit_buf_struct,
    ) -> WORD32;
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
pub struct ixheaac_drc_data_struct {
    pub prog_ref_level: WORD32,
    pub n_mdct_bands: [WORD16; 16],
    pub drc_fac: [WORD16; 16],
    pub drc_fac_dvb: [WORD16; 16],
    pub drc_exp: WORD8,
    pub short_block: UWORD8,
    pub drc_interp_scheme: UWORD8,
    pub n_drc_bands: UWORD8,
    pub new_prog_ref_level: UWORD8,
    pub new_drc_fac: UWORD8,
    pub prev_interp_scheme: UWORD8,
    pub drc_factors_sbr: [[WORD32; 64]; 64],
    pub drc_factors_sbr_lat: [[WORD32; 64]; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ixheaac_drc_bs_data_struct {
    pub b_channel_on: [UWORD8; 8],
    pub prog_ref_level_present: UWORD8,
    pub prog_ref_level: UWORD8,
    pub drc_num_bands: UWORD8,
    pub drc_band_top: [UWORD8; 16],
    pub dyn_rng_dlbl: [WORD8; 16],
    pub dyn_rng_dlbl_dvb: [WORD8; 16],
    pub max_dyn_rng_dlbl: WORD8,
    pub drc_interpolation_scheme: UWORD8,
    pub drc_data_type: WORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_dec_struct {
    pub str_drc_bs_data: [ixheaac_drc_bs_data_struct; 10],
    pub str_drc_channel_data: [ixheaac_drc_data_struct; 10],
    pub drc_ref_level: WORD16,
    pub drc_def_level: WORD16,
    pub drc_channel_next_index: [UWORD8; 10],
    pub sbr_allowed: UWORD8,
    pub sbr_found: UWORD8,
    pub drc_element_found: UWORD8,
    pub max_audio_channels: UWORD8,
    pub length_history: UWORD8,
    pub num_drc_elements: UWORD8,
    pub state: WORD32,
    pub target_ref_level: WORD32,
    pub prog_ref_level: WORD32,
    pub cut_factor: WORD32,
    pub boost_factor: WORD32,
    pub drc_dig_norm: FLAG,
    pub drc_on: FLAG,
    pub dvb_anc_data_present: FLAG,
    pub dvb_anc_data_pos: WORD32,
    pub pres_mode: WORD32,
    pub heavy_mode: WORD32,
}
pub type C2RustUnnamed = core::ffi::c_uint;
pub const SBR_ID_END: C2RustUnnamed = 8;
pub const SBR_ID_FIL: C2RustUnnamed = 7;
pub const SBR_ID_PCE: C2RustUnnamed = 6;
pub const SBR_ID_DSE: C2RustUnnamed = 5;
pub const SBR_ID_LFE: C2RustUnnamed = 4;
pub const SBR_ID_LCS: C2RustUnnamed = 3;
pub const SBR_ID_CCE: C2RustUnnamed = 2;
pub const SBR_ID_CPE: C2RustUnnamed = 1;
pub const SBR_ID_SCE: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_element_stream_struct {
    pub sbr_ele_id: WORD32,
    pub extension_type: WORD32,
    pub size_payload: WORD32,
    pub ptr_sbr_data: *mut WORD8,
    pub ptr_prev_sbr_data: *mut WORD8,
    pub prev_size_payload: WORD32,
    pub frame_error_flag: [WORD32; 2],
    pub use_frame_slot: WORD32,
    pub prev_sbr_ele_id: WORD32,
    pub prev_extension_type: WORD32,
    pub size_payload_old: WORD32,
    pub sbr_prev_data: [WORD8; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_sbr_bitstream_struct {
    pub no_elements: WORD16,
    pub str_sbr_ele: [ia_sbr_element_stream_struct; 1],
}
pub const AOT_ER_AAC_ELD: AUDIO_OBJECT_TYPE = 39;
pub type AUDIO_OBJECT_TYPE = core::ffi::c_uint;
pub const AOT_USAC: AUDIO_OBJECT_TYPE = 42;
pub const AOT_ESC: AUDIO_OBJECT_TYPE = 31;
pub const AOT_RSVD_31: AUDIO_OBJECT_TYPE = 31;
pub const AOT_RSVD_30: AUDIO_OBJECT_TYPE = 30;
pub const AOT_PS: AUDIO_OBJECT_TYPE = 29;
pub const AOT_RSVD_28: AUDIO_OBJECT_TYPE = 28;
pub const AOT_ER_PARA: AUDIO_OBJECT_TYPE = 27;
pub const AOT_ER_HILN: AUDIO_OBJECT_TYPE = 26;
pub const AOT_ER_HVXC: AUDIO_OBJECT_TYPE = 25;
pub const AOT_ER_CELP: AUDIO_OBJECT_TYPE = 24;
pub const AOT_ER_AAC_LD: AUDIO_OBJECT_TYPE = 23;
pub const AOT_ER_BSAC: AUDIO_OBJECT_TYPE = 22;
pub const AOT_ER_TWIN_VQ: AUDIO_OBJECT_TYPE = 21;
pub const AOT_ER_AAC_SCAL: AUDIO_OBJECT_TYPE = 20;
pub const AOT_ER_AAC_LTP: AUDIO_OBJECT_TYPE = 19;
pub const AOT_RSVD_18: AUDIO_OBJECT_TYPE = 18;
pub const AOT_ER_AAC_LC: AUDIO_OBJECT_TYPE = 17;
pub const AOT_ALG_SYNTH_AUD_FX: AUDIO_OBJECT_TYPE = 16;
pub const AOT_GEN_MIDI: AUDIO_OBJECT_TYPE = 15;
pub const AOT_WAV_TAB_SYNTH: AUDIO_OBJECT_TYPE = 14;
pub const AOT_MAIN_SYNTH: AUDIO_OBJECT_TYPE = 13;
pub const AOT_TTSI: AUDIO_OBJECT_TYPE = 12;
pub const AOT_RSVD_11: AUDIO_OBJECT_TYPE = 11;
pub const AOT_RSVD_10: AUDIO_OBJECT_TYPE = 10;
pub const AOT_HVXC: AUDIO_OBJECT_TYPE = 9;
pub const AOT_CELP: AUDIO_OBJECT_TYPE = 8;
pub const AOT_TWIN_VQ: AUDIO_OBJECT_TYPE = 7;
pub const AOT_AAC_SCAL: AUDIO_OBJECT_TYPE = 6;
pub const AOT_SBR: AUDIO_OBJECT_TYPE = 5;
pub const AOT_AAC_LTP: AUDIO_OBJECT_TYPE = 4;
pub const AOT_AAC_SSR: AUDIO_OBJECT_TYPE = 3;
pub const AOT_AAC_LC: AUDIO_OBJECT_TYPE = 2;
pub const AOT_AAC_MAIN: AUDIO_OBJECT_TYPE = 1;
pub const AOT_NULL_OBJECT: AUDIO_OBJECT_TYPE = 0;
pub const EXT_DYNAMIC_RANGE: core::ffi::c_int = 11 as core::ffi::c_int;
pub const EXT_SAC_DATA: core::ffi::c_int = 12 as core::ffi::c_int;
pub const SBR_EXTENSION: core::ffi::c_int = 13 as core::ffi::c_int;
pub const SBR_EXTENSION_CRC: core::ffi::c_int = 14 as core::ffi::c_int;
pub const MAXSBRBYTES: core::ffi::c_int = 1024 as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES: core::ffi::c_int = 0x1804
    as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_check_for_sbr_payload(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut pstr_stream_sbr: *mut ia_aac_dec_sbr_bitstream_struct,
    mut prev_element: WORD16,
    mut pstr_drc_dec: *mut ia_drc_dec_struct,
    mut object_type: WORD32,
    mut adtsheader: WORD32,
    mut cnt_bits: WORD32,
    mut ld_sbr_crc_flag: WORD32,
    mut drc_dummy: *mut ia_drc_dec_struct,
    mut mps_buffer: *mut UWORD8,
    mut mps_header: *mut WORD32,
    mut mps_bytes: *mut WORD32,
    mut is_init: WORD32,
    mut is_first: *mut WORD32,
    mut ec_flag: WORD32,
) -> FLAG {
    let mut ret: FLAG = 0 as FLAG;
    let mut count: WORD32 = 0;
    if object_type == AOT_ER_AAC_ELD as core::ffi::c_int {
        count = (*it_bit_buff).cnt_bits >> 3 as core::ffi::c_int;
        if adtsheader == 1 as core::ffi::c_int {
            count = cnt_bits >> 3 as core::ffi::c_int;
        }
    } else {
        count = ixheaacd_read_bits_buf(it_bit_buff, 4 as WORD);
        if count as core::ffi::c_int - 15 as core::ffi::c_int == 0 as core::ffi::c_int {
            let mut esc_count: WORD32 = 0;
            esc_count = ixheaacd_read_bits_buf(it_bit_buff, 8 as WORD);
            count = (esc_count as core::ffi::c_int + 14 as core::ffi::c_int) as WORD32;
        }
    }
    if count > 0 as core::ffi::c_int {
        let mut extension_type: WORD32 = 0;
        if object_type == AOT_ER_AAC_ELD as core::ffi::c_int {
            extension_type = (if ld_sbr_crc_flag != 0 {
                SBR_EXTENSION_CRC
            } else {
                SBR_EXTENSION
            }) as WORD32;
        } else {
            extension_type = ixheaacd_read_bits_buf(it_bit_buff, 4 as WORD);
        }
        if count < MAXSBRBYTES
            && (extension_type == SBR_EXTENSION || extension_type == SBR_EXTENSION_CRC)
            && (prev_element as core::ffi::c_int == SBR_ID_SCE as core::ffi::c_int
                || prev_element as core::ffi::c_int == SBR_ID_CPE as core::ffi::c_int
                || prev_element as core::ffi::c_int - SBR_ID_CCE as core::ffi::c_int
                    == 0 as core::ffi::c_int)
        {
            let mut no_elements: WORD32 = (*pstr_stream_sbr).no_elements as WORD32;
            let mut byte_count: WORD32 = 0;
            let mut ptr_stream_sbr: *mut ia_sbr_element_stream_struct = 0
                as *mut ia_sbr_element_stream_struct;
            ret = 1 as core::ffi::c_int as FLAG;
            ptr_stream_sbr = &mut *((*pstr_stream_sbr).str_sbr_ele)
                .as_mut_ptr()
                .offset(no_elements as isize) as *mut ia_sbr_element_stream_struct;
            if ec_flag != 0 {
                (*ptr_stream_sbr).size_payload = (*ptr_stream_sbr).size_payload_old;
                byte_count = (*ptr_stream_sbr).size_payload;
                (*ptr_stream_sbr).extension_type = (*ptr_stream_sbr).prev_extension_type;
                (*ptr_stream_sbr).sbr_ele_id = (*ptr_stream_sbr).prev_sbr_ele_id;
            }
            if ec_flag != 0 {
                (*ptr_stream_sbr).size_payload_old = count;
                byte_count = (*ptr_stream_sbr).size_payload_old;
                (*ptr_stream_sbr).prev_extension_type = extension_type;
                (*ptr_stream_sbr).prev_sbr_ele_id = prev_element as WORD32;
            } else {
                (*ptr_stream_sbr).size_payload = count;
                byte_count = (*ptr_stream_sbr).size_payload;
                (*ptr_stream_sbr).extension_type = extension_type;
                (*ptr_stream_sbr).sbr_ele_id = prev_element as WORD32;
            }
            (*pstr_stream_sbr).no_elements = (no_elements as core::ffi::c_int
                + 1 as core::ffi::c_int) as WORD16;
            if !pstr_drc_dec.is_null() {
                (*pstr_drc_dec).sbr_found = 1 as UWORD8;
            }
            if ec_flag != 0 {
                memcpy(
                    (*ptr_stream_sbr).ptr_sbr_data as *mut core::ffi::c_void,
                    ((*ptr_stream_sbr).sbr_prev_data).as_mut_ptr()
                        as *const core::ffi::c_void,
                    ::core::mem::size_of::<[WORD8; 1024]>() as size_t,
                );
            }
            if byte_count > 0 as core::ffi::c_int
                && byte_count as core::ffi::c_int - 1024 as core::ffi::c_int
                    <= 0 as core::ffi::c_int
            {
                let mut i: WORD32 = 0;
                let mut ptr_sbr_data: *mut WORD8 = 0 as *mut WORD8;
                if object_type != AOT_ER_AAC_ELD as core::ffi::c_int {
                    if ec_flag != 0 {
                        ptr_sbr_data = &mut *((*ptr_stream_sbr).sbr_prev_data)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as isize) as *mut WORD8;
                        (*ptr_stream_sbr)
                            .sbr_prev_data[0 as core::ffi::c_int as usize] = ixheaacd_read_bits_buf(
                            it_bit_buff,
                            4 as WORD,
                        ) as WORD8;
                    } else {
                        ptr_sbr_data = &mut *((*ptr_stream_sbr).ptr_sbr_data)
                            .offset(1 as core::ffi::c_int as isize) as *mut WORD8;
                        *((*ptr_stream_sbr).ptr_sbr_data)
                            .offset(0 as core::ffi::c_int as isize) = ixheaacd_read_bits_buf(
                            it_bit_buff,
                            4 as WORD,
                        ) as WORD8;
                    }
                } else if ec_flag != 0 {
                    ptr_sbr_data = ((*ptr_stream_sbr).sbr_prev_data).as_mut_ptr();
                } else {
                    ptr_sbr_data = (*ptr_stream_sbr).ptr_sbr_data;
                }
                i = (byte_count as core::ffi::c_int - 2 as core::ffi::c_int) as WORD32;
                while i >= 0 as core::ffi::c_int {
                    let fresh0 = ptr_sbr_data;
                    ptr_sbr_data = ptr_sbr_data.offset(1);
                    *fresh0 = ixheaacd_read_bits_buf(it_bit_buff, 8 as WORD) as WORD8;
                    if object_type == AOT_ER_AAC_ELD as core::ffi::c_int {
                        if adtsheader == 1 as core::ffi::c_int {
                            cnt_bits = (cnt_bits as core::ffi::c_int
                                - 8 as core::ffi::c_int) as WORD32;
                        }
                    }
                    i -= 1;
                }
                if object_type == AOT_ER_AAC_ELD as core::ffi::c_int {
                    let fresh1 = ptr_sbr_data;
                    ptr_sbr_data = ptr_sbr_data.offset(1);
                    *fresh1 = ixheaacd_read_bits_buf(it_bit_buff, 8 as WORD) as WORD8;
                    if adtsheader == 1 as core::ffi::c_int {
                        cnt_bits = (cnt_bits as core::ffi::c_int - 8 as core::ffi::c_int)
                            as WORD32;
                        if cnt_bits > 0 as core::ffi::c_int {
                            let mut unaligned_bits: WORD32 = 8 as WORD32
                                - (*it_bit_buff).cnt_bits;
                            *ptr_sbr_data = ixheaacd_read_bits_buf(
                                it_bit_buff,
                                cnt_bits as WORD,
                            ) as WORD8;
                            *ptr_sbr_data = ((*ptr_sbr_data as core::ffi::c_int)
                                << unaligned_bits) as WORD8;
                            if ec_flag == 0 {
                                (*ptr_stream_sbr).size_payload += 1;
                            } else {
                                (*ptr_stream_sbr).size_payload_old += 1;
                            }
                        }
                    } else if (*it_bit_buff).cnt_bits > 0 as core::ffi::c_int {
                        let mut unaligned_bits_0: WORD32 = 8 as WORD32
                            - (*it_bit_buff).cnt_bits;
                        *ptr_sbr_data = ixheaacd_read_bits_buf(
                            it_bit_buff,
                            (*it_bit_buff).cnt_bits as WORD,
                        ) as WORD8;
                        *ptr_sbr_data = ((*ptr_sbr_data as core::ffi::c_int)
                            << unaligned_bits_0) as WORD8;
                        if ec_flag == 0 {
                            (*ptr_stream_sbr).size_payload += 1;
                        } else {
                            (*ptr_stream_sbr).size_payload_old += 1;
                        }
                    }
                }
            }
        } else if extension_type == EXT_DYNAMIC_RANGE {
            (*pstr_drc_dec).drc_element_found = 1 as UWORD8;
            count -= ixheaacd_dec_drc_read_element(pstr_drc_dec, drc_dummy, it_bit_buff);
        } else if extension_type == EXT_SAC_DATA {
            let mut anc_type: WORD32 = 0;
            let mut anc_start: WORD32 = 0;
            let mut i_0: WORD32 = 0;
            let mut len: WORD32 = 0 as WORD32;
            anc_type = ixheaacd_read_bits_buf(it_bit_buff, 2 as WORD);
            *mps_header = anc_type;
            anc_start = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
            if anc_start == 1 as core::ffi::c_int {
                *mps_bytes = 0 as core::ffi::c_int as WORD32;
            }
            ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
            if anc_type == 1 as core::ffi::c_int && is_init == 0 as core::ffi::c_int
                && *is_first == 1 as core::ffi::c_int
            {
                len = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD);
                len = (ixheaacd_read_bits_buf(it_bit_buff, 7 as WORD) as core::ffi::c_int
                    + 1 as core::ffi::c_int) as WORD32;
                ixheaacd_read_bidirection(it_bit_buff, -(8 as WORD32));
            }
            i_0 = 0 as core::ffi::c_int as WORD32;
            while i_0 < count as core::ffi::c_int - 1 as core::ffi::c_int {
                *mps_buffer.offset((i_0 + *mps_bytes) as isize) = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    8 as WORD,
                ) as UWORD8;
                i_0 += 1;
            }
            *mps_bytes += count as core::ffi::c_int - 1 as core::ffi::c_int;
            if anc_type == 1 as core::ffi::c_int && is_init == 0 as core::ffi::c_int
                && *is_first == 1 as core::ffi::c_int
            {
                if *mps_bytes < len {
                    if ec_flag != 0 {
                        *mps_bytes = 0 as core::ffi::c_int as WORD32;
                    }
                    longjmp(
                        (*(*it_bit_buff).xaac_jmp_buf).as_mut_ptr(),
                        IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES,
                    );
                }
                i_0 = 0 as core::ffi::c_int as WORD32;
                while i_0 < count as core::ffi::c_int - 1 as core::ffi::c_int {
                    *mps_buffer.offset(i_0 as isize) = *mps_buffer
                        .offset((i_0 + len) as isize);
                    i_0 += 1;
                }
                *mps_bytes = *mps_bytes - len;
            }
            *is_first = 1 as core::ffi::c_int as WORD32;
        } else {
            ixheaacd_read_bits_buf(it_bit_buff, 4 as WORD);
            if (*it_bit_buff).cnt_bits
                < (count as core::ffi::c_int - 1 as core::ffi::c_int)
                    << 3 as core::ffi::c_int
            {
                longjmp(
                    (*(*it_bit_buff).xaac_jmp_buf).as_mut_ptr(),
                    IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES,
                );
            }
            (*it_bit_buff).ptr_read_next = ((*it_bit_buff).ptr_read_next)
                .offset((count as core::ffi::c_int - 1 as core::ffi::c_int) as isize);
            (*it_bit_buff).cnt_bits
                -= (count as core::ffi::c_int - 1 as core::ffi::c_int)
                    << 3 as core::ffi::c_int;
        }
    }
    return ret;
}
