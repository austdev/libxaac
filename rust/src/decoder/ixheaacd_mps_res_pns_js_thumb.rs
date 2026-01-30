extern "C" {
    fn ixheaacd_res_get_sfb_offsets(
        p_ics_info: *mut ia_mps_dec_residual_ics_info_struct,
        aac_tables_ptr: *mut ia_mps_dec_residual_aac_tables_struct,
    ) -> *const WORD16;
    fn ixheaacd_res_calc_max_spectral_line(p_tmp: *mut WORD32, size: WORD32) -> WORD32;
    fn ixheaacd_res_tns_parcor_2_lpc_32x16(
        parcor: *mut WORD16,
        lpc: *mut WORD16,
        scale: *mut WORD16,
        order: WORD,
    ) -> VOID;
    fn ixheaacd_res_tns_ar_filter_fixed_32x16(
        spectrum: *mut WORD32,
        size: WORD32,
        inc: WORD32,
        lpc: *mut WORD16,
        order: WORD32,
        shift_value: WORD32,
        scale_spec: WORD,
    ) -> VOID;
}
pub type WORD8 = core::ffi::c_schar;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD16 = core::ffi::c_short;
pub type UWORD16 = core::ffi::c_ushort;
pub type WORD32 = core::ffi::c_int;
pub type VOID = ();
pub type FLAG = core::ffi::c_int;
pub type WORD = core::ffi::c_int;
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
unsafe extern "C" fn ixheaac_min32(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut min_val: WORD32 = 0;
    min_val = if a < b { a } else { b };
    return min_val;
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
#[inline]
unsafe extern "C" fn ixheaacd_res_get_maximum_tns_bands(
    mut p_ics_info: *mut ia_mps_dec_residual_ics_info_struct,
    mut aac_tables_ptr: *mut ia_mps_dec_residual_aac_tables_struct,
    mut win_len: *mut WORD32,
) -> WORD16 {
    let mut i: WORD32 = 0 as WORD32;
    *win_len = 1 as core::ffi::c_int as WORD32;
    if (*p_ics_info).window_sequence as core::ffi::c_int == EIGHT_SHORT_SEQUENCE {
        *win_len = 8 as core::ffi::c_int as WORD32;
        i = 1 as core::ffi::c_int as WORD32;
    }
    return (*(*aac_tables_ptr).res_block_tables_ptr)
        .tns_max_bands_tbl[(*p_ics_info).sampling_rate_index as usize][i as usize]
        as WORD16;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_res_tns_decode_coeffs_32x16(
    mut filter: *const ia_mps_dec_residual_filter_struct,
    mut a: *mut WORD16,
    mut aac_tables_ptr: *mut ia_mps_dec_residual_aac_tables_struct,
) -> VOID {
    let mut tmp: WORD = 0;
    let mut aptr: *mut WORD16 = a;
    let mut tns_coeff_ptr: *mut WORD16 = 0 as *mut WORD16;
    let mut offset: WORD8 = 4 as WORD8;
    let mut p_coeff: *mut WORD8 = &*((*filter).coeff)
        .as_ptr()
        .offset(0 as core::ffi::c_int as isize) as *const WORD8 as *mut WORD8;
    let mut tmp1: WORD32 = 0;
    tmp = (*filter).resolution as WORD;
    tns_coeff_ptr = ((*(*aac_tables_ptr).res_block_tables_ptr).tns_coeff3_16)
        .as_mut_ptr();
    if tmp != 0 {
        tns_coeff_ptr = ((*(*aac_tables_ptr).res_block_tables_ptr).tns_coeff4_16)
            .as_mut_ptr();
        offset = ((offset as core::ffi::c_int) << 1 as core::ffi::c_int) as WORD8;
    }
    tmp1 = (*filter).order as WORD32;
    loop {
        let fresh0 = p_coeff;
        p_coeff = p_coeff.offset(1);
        let mut temp: WORD8 = *fresh0;
        let fresh1 = aptr;
        aptr = aptr.offset(1);
        *fresh1 = *tns_coeff_ptr
            .offset((temp as core::ffi::c_int + offset as core::ffi::c_int) as isize);
        tmp1 -= 1;
        if !(tmp1 != 0 as core::ffi::c_int) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_res_ctns_apply(
    mut p_aac_decoder_channel_info: *mut ia_mps_dec_residual_channel_info_struct,
    mut max_sfb: WORD16,
    mut aac_tables_ptr: *mut ia_mps_dec_residual_aac_tables_struct,
) -> VOID {
    let mut i: WORD = 0;
    let mut scale_lpc: WORD16 = 0;
    let mut p_tns_data: *mut ia_mps_dec_residual_tns_data = &mut (*p_aac_decoder_channel_info)
        .tns_data;
    let mut p_spectrum: *mut WORD32 = (*p_aac_decoder_channel_info)
        .p_spectral_coefficient;
    let mut window: WORD = 0;
    let mut index: WORD = 0;
    let mut start: WORD = 0;
    let mut stop: WORD = 0;
    let mut size: WORD = 0;
    let mut scale_spec: WORD = 0;
    let mut p_ics_info: *mut ia_mps_dec_residual_ics_info_struct = &mut (*p_aac_decoder_channel_info)
        .ics_info;
    let mut win_len: WORD = 0;
    let mut tns_max_bands: WORD = 0;
    let mut maximum_bins_short: WORD16 = ixheaac_shr16_dir_sat(
        (*p_ics_info).frame_length,
        3 as WORD16,
    );
    let mut coeff_parc: [WORD32; 32] = [0; 32];
    let mut lpc: [WORD32; 32] = [0; 32];
    let mut scale_factor_bands_tbl: *const WORD16 = 0 as *const WORD16;
    if (*p_tns_data).tns_data_present == 0 {
        return;
    }
    tns_max_bands = ixheaacd_res_get_maximum_tns_bands(
        p_ics_info,
        aac_tables_ptr,
        &mut win_len,
    ) as WORD;
    scale_factor_bands_tbl = ixheaacd_res_get_sfb_offsets(
        &mut (*p_aac_decoder_channel_info).ics_info,
        aac_tables_ptr,
    );
    window = 0 as core::ffi::c_int as WORD;
    while window < win_len {
        let mut ind_len: WORD = (*p_tns_data).number_of_filters[window as usize] as WORD;
        index = 0 as core::ffi::c_int as WORD;
        while index < ind_len {
            let mut filter: *mut ia_mps_dec_residual_filter_struct = &mut *(*((*p_tns_data)
                .filter)
                .as_mut_ptr()
                .offset(window as isize))
                .as_mut_ptr()
                .offset(index as isize) as *mut ia_mps_dec_residual_filter_struct;
            if !((*filter).order as core::ffi::c_int <= 0 as core::ffi::c_int
                || (*filter).order as core::ffi::c_int > MAX_ORDER_LONG)
            {
                ixheaacd_res_tns_decode_coeffs_32x16(
                    filter,
                    coeff_parc.as_mut_ptr() as *mut WORD16,
                    aac_tables_ptr,
                );
                start = ixheaac_min32(
                    ixheaac_min32(
                        (*filter).start_band as WORD32,
                        tns_max_bands as WORD32,
                    ),
                    max_sfb as WORD32,
                ) as WORD;
                start = *scale_factor_bands_tbl.offset(start as isize) as WORD;
                stop = ixheaac_min32(
                    ixheaac_min32(
                        (*filter).stop_band as WORD32,
                        tns_max_bands as WORD32,
                    ),
                    max_sfb as WORD32,
                ) as WORD;
                stop = *scale_factor_bands_tbl.offset(stop as isize) as WORD;
                size = stop - start;
                if !(size <= 0 as core::ffi::c_int) {
                    ixheaacd_res_tns_parcor_2_lpc_32x16(
                        coeff_parc.as_mut_ptr() as *mut WORD16,
                        lpc.as_mut_ptr() as *mut WORD16,
                        &mut scale_lpc,
                        (*filter).order as WORD,
                    );
                    let mut p_tmp: *mut WORD32 = p_spectrum
                        .offset(
                            (window as core::ffi::c_int
                                * maximum_bins_short as core::ffi::c_int) as isize,
                        )
                        .offset(start as isize);
                    scale_spec = ixheaacd_res_calc_max_spectral_line(
                        p_tmp,
                        size as WORD32,
                    ) as WORD;
                    scale_spec = (scale_spec as core::ffi::c_int - 4 as core::ffi::c_int
                        - scale_lpc as core::ffi::c_int) as WORD;
                    if scale_spec > 0 as core::ffi::c_int {
                        let mut shift: WORD = 0;
                        scale_spec = ixheaac_min32(scale_spec as WORD32, 31 as WORD32)
                            as WORD;
                        if (*filter).direction as core::ffi::c_int
                            == -(1 as core::ffi::c_int)
                        {
                            shift = (stop as core::ffi::c_int - 1 as core::ffi::c_int)
                                as WORD;
                        } else {
                            shift = start;
                        }
                        ixheaacd_res_tns_ar_filter_fixed_32x16(
                            &mut *p_spectrum
                                .offset(
                                    (window * maximum_bins_short as WORD + shift) as isize,
                                ),
                            size as WORD32,
                            (*filter).direction as WORD32,
                            lpc.as_mut_ptr() as *mut WORD16,
                            (*filter).order as WORD32,
                            scale_lpc as WORD32,
                            scale_spec,
                        );
                    } else {
                        let mut shift_0: WORD = 0;
                        let mut p_tmp_0: *mut WORD32 = p_spectrum
                            .offset(
                                (window as core::ffi::c_int
                                    * maximum_bins_short as core::ffi::c_int) as isize,
                            )
                            .offset(start as isize);
                        scale_spec = -scale_spec;
                        scale_spec = ixheaac_min32(scale_spec as WORD32, 31 as WORD32)
                            as WORD;
                        i = size;
                        while i != 0 as core::ffi::c_int {
                            *p_tmp_0 = *p_tmp_0 >> scale_spec;
                            p_tmp_0 = p_tmp_0.offset(1);
                            i -= 1;
                        }
                        if (*filter).direction as core::ffi::c_int
                            == -(1 as core::ffi::c_int)
                        {
                            shift_0 = (stop as core::ffi::c_int - 1 as core::ffi::c_int)
                                as WORD;
                        } else {
                            shift_0 = start;
                        }
                        let mut shift_val: WORD32 = scale_lpc as WORD32;
                        ixheaacd_res_tns_ar_filter_fixed_32x16(
                            &mut *p_spectrum
                                .offset(
                                    (window * maximum_bins_short as WORD + shift_0) as isize,
                                ),
                            size as WORD32,
                            (*filter).direction as WORD32,
                            lpc.as_mut_ptr() as *mut WORD16,
                            (*filter).order as WORD32,
                            shift_val,
                            0 as WORD,
                        );
                        p_tmp_0 = p_spectrum
                            .offset(
                                (window as core::ffi::c_int
                                    * maximum_bins_short as core::ffi::c_int) as isize,
                            )
                            .offset(start as isize);
                        i = size;
                        loop {
                            *p_tmp_0 = *p_tmp_0 << scale_spec;
                            p_tmp_0 = p_tmp_0.offset(1);
                            i -= 1;
                            if !(i != 0 as core::ffi::c_int) {
                                break;
                            }
                        }
                    }
                }
            }
            index += 1;
        }
        window += 1;
    }
}
