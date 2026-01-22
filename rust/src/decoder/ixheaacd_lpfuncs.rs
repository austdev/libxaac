extern "C" {
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn ixheaacd_over_lap_add1_dec(
        coef: *mut WORD32,
        prev: *mut WORD32,
        out: *mut WORD32,
        window: *const WORD16,
        q_shift: WORD16,
        size: WORD16,
        ch_fac: WORD16,
    ) -> VOID;
    fn ixheaacd_over_lap_add2_dec(
        coef: *mut WORD32,
        prev: *mut WORD32,
        out: *mut WORD32,
        window: *const WORD16,
        q_shift: WORD16,
        size: WORD16,
        ch_fac: WORD16,
    ) -> VOID;
    fn ixheaacd_lap1_512_480(
        coef: *mut WORD32,
        prev: *mut WORD32,
        out_tmp: *mut core::ffi::c_void,
        window: *const WORD16,
        q_shift: WORD16,
        size: WORD16,
        stride: WORD16,
        slot_element: WORD,
    ) -> VOID;
    fn ixheaacd_dec_copy_outsample(
        out_samples: *mut WORD32,
        p_overlap_buffer: *mut WORD32,
        size: WORD32,
        stride: WORD16,
    ) -> VOID;
    fn ixheaacd_inverse_transform(
        spec_data: *mut WORD32,
        scratch: *mut WORD32,
        ptr_imdct_tables: *mut ia_aac_dec_imdct_tables_struct,
        expo: WORD32,
        npoints: WORD32,
    ) -> WORD32;
    fn ixheaacd_inverse_transform_960(
        spec_data: *mut WORD32,
        scratch: *mut WORD32,
        ptr_imdct_tables: *mut ia_aac_dec_imdct_tables_struct,
        expo: WORD32,
        imdct_scale: *mut WORD32,
    ) -> VOID;
    fn ixheaacd_inverse_transform_512(
        data: *mut WORD32,
        temp: *mut WORD32,
        imdct_scale: *mut WORD32,
        cos_sin_ptr: *mut WORD32,
        imdct_tables_ptr: *mut ia_aac_dec_imdct_tables_struct,
        object_type: WORD32,
    ) -> VOID;
    fn ixheaacd_mdct_960(
        inp: *mut WORD32,
        scratch: *mut WORD32,
        mdct_scale: *mut WORD32,
        mdct_flag: WORD32,
        imdct_tables_ptr: *mut ia_aac_dec_imdct_tables_struct,
    ) -> VOID;
    fn ixheaacd_mdct_480_ld(
        inp: *mut WORD32,
        scratch: *mut WORD32,
        mdct_scale: *mut WORD32,
        mdct_flag: WORD32,
        imdct_tables_ptr: *mut ia_aac_dec_imdct_tables_struct,
        object_type: WORD32,
    ) -> VOID;
    static mut ixheaacd_over_lap_add1: Option<
        unsafe extern "C" fn(
            *mut WORD32,
            *mut WORD32,
            *mut WORD32,
            *const WORD16,
            WORD16,
            WORD16,
            WORD16,
        ) -> VOID,
    >;
    static mut ixheaacd_over_lap_add2: Option<
        unsafe extern "C" fn(
            *mut WORD32,
            *mut WORD32,
            *mut WORD32,
            *const WORD16,
            WORD16,
            WORD16,
            WORD16,
        ) -> VOID,
    >;
    static mut ixheaacd_calc_max_spectral_line: Option<
        unsafe extern "C" fn(*mut WORD32, WORD32) -> WORD32,
    >;
    static mut ixheaacd_post_twiddle: Option<
        unsafe extern "C" fn(
            *mut WORD32,
            *mut WORD32,
            *mut ia_aac_dec_imdct_tables_struct,
            WORD,
        ) -> VOID,
    >;
    static mut ixheaacd_post_twid_overlap_add: Option<
        unsafe extern "C" fn(
            *mut WORD32,
            *mut WORD32,
            *mut ia_aac_dec_imdct_tables_struct,
            WORD,
            *mut WORD32,
            WORD16,
            *const WORD16,
            WORD16,
        ) -> VOID,
    >;
    static mut ixheaacd_neg_shift_spec: Option<
        unsafe extern "C" fn(*mut WORD32, *mut WORD32, WORD16, WORD16) -> VOID,
    >;
    static mut ixheaacd_spec_to_overlapbuf: Option<
        unsafe extern "C" fn(*mut WORD32, *mut WORD32, WORD32, WORD32) -> VOID,
    >;
    static mut ixheaacd_overlap_buf_out: Option<
        unsafe extern "C" fn(*mut WORD32, *mut WORD32, WORD32, WORD16) -> VOID,
    >;
    static mut ixheaacd_overlap_out_copy: Option<
        unsafe extern "C" fn(
            *mut WORD32,
            *mut WORD32,
            *mut WORD32,
            WORD16,
            WORD16,
        ) -> VOID,
    >;
}
pub type size_t = usize;
pub type WORD8 = core::ffi::c_schar;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD16 = core::ffi::c_short;
pub type UWORD16 = core::ffi::c_ushort;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type WORD64 = core::ffi::c_longlong;
pub type VOID = ();
pub type WORD = core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sampling_rate_info_struct {
    pub sampling_frequency: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_block_tables_struct {
    pub ixheaacd_pow_table_Q13: [WORD32; 129],
    pub scale_table: [WORD32; 4],
    pub tns_max_bands_tbl: [[WORD8; 2]; 12],
    pub tns_coeff3_16: [WORD16; 8],
    pub tns_coeff4_16: [WORD16; 16],
    pub scale_mant_tab: [WORD32; 4],
    pub tns_coeff3: [WORD32; 8],
    pub tns_coeff4: [WORD32; 16],
    pub tns_coeff3_32: [WORD32; 8],
    pub tns_coeff4_32: [WORD32; 16],
    pub tns_max_bands_tbl_usac: [[WORD32; 2]; 16],
    pub tns_max_bands_tbl_ld: [WORD8; 12],
    pub tns_max_bands_tbl_480: [WORD8; 12],
    pub scale_table_960: [WORD32; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_huffman_tables_struct {
    pub ixheaacd_sfb_96_1024: [WORD8; 43],
    pub ixheaacd_sfb_96_128: [WORD8; 14],
    pub ixheaacd_sfb_64_1024: [WORD8; 49],
    pub ixheaacd_sfb_48_1024: [WORD8; 51],
    pub ixheaacd_sfb_48_128: [WORD8; 16],
    pub ixheaacd_sfb_32_1024: [WORD8; 53],
    pub ixheaacd_sfb_24_1024: [WORD8; 49],
    pub ixheaacd_sfb_24_128: [WORD8; 17],
    pub ixheaacd_sfb_16_1024: [WORD8; 45],
    pub ixheaacd_sfb_16_128: [WORD8; 17],
    pub ixheaacd_sfb_8_1024: [WORD8; 42],
    pub ixheaacd_sfb_8_128: [WORD8; 17],
    pub str_sample_rate_info: [ia_sampling_rate_info_struct; 13],
    pub idx_table_hf11: [UWORD32; 21],
    pub idx_table_hf10: [UWORD32; 20],
    pub idx_table_hf9: [UWORD32; 23],
    pub idx_table_hf8: [UWORD32; 17],
    pub idx_table_hf7: [UWORD32; 18],
    pub idx_table_hf6: [UWORD32; 17],
    pub idx_table_hf5: [UWORD32; 19],
    pub idx_table_hf4: [UWORD32; 19],
    pub idx_table_hf3: [UWORD32; 27],
    pub idx_table_hf2: [UWORD32; 16],
    pub idx_table_hf1: [UWORD32; 12],
    pub input_table_cb11: [UWORD16; 290],
    pub input_table_cb10: [UWORD16; 170],
    pub input_table_cb9: [UWORD16; 170],
    pub input_table_cb8: [UWORD16; 65],
    pub input_table_cb7: [UWORD16; 65],
    pub input_table_cb6: [UWORD16; 82],
    pub input_table_cb5: [UWORD16; 82],
    pub input_table_cb4: [UWORD16; 82],
    pub input_table_cb3: [UWORD16; 82],
    pub input_table_cb2: [UWORD16; 82],
    pub input_table_cb1: [UWORD16; 82],
    pub huffman_code_book_scl: [UWORD16; 122],
    pub huffman_code_book_scl_index: [UWORD32; 33],
    pub ixheaacd_sfb_48_512: [WORD8; 37],
    pub ixheaacd_sfb_32_512: [WORD8; 38],
    pub ixheaacd_sfb_24_512: [WORD8; 32],
    pub ixheaacd_sfb_48_480: [WORD8; 36],
    pub ixheaacd_sfb_32_480: [WORD8; 38],
    pub ixheaacd_sfb_24_480: [WORD8; 31],
    pub ixheaacd_sfb_96_960: [WORD8; 41],
    pub ixheaacd_sfb_96_120: [WORD8; 13],
    pub ixheaacd_sfb_64_960: [WORD8; 47],
    pub ixheaacd_sfb_48_960: [WORD8; 50],
    pub ixheaacd_sfb_48_120: [WORD8; 15],
    pub ixheaacd_sfb_24_960: [WORD8; 47],
    pub ixheaacd_sfb_24_120: [WORD8; 16],
    pub ixheaacd_sfb_16_960: [WORD8; 43],
    pub ixheaacd_sfb_16_120: [WORD8; 16],
    pub ixheaacd_sfb_8_960: [WORD8; 41],
    pub ixheaacd_sfb_8_120: [WORD8; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_imdct_tables_struct {
    pub cosine_array_2048_256: [WORD16; 514],
    pub dig_rev_table8_long: [WORD8; 64],
    pub dig_rev_table8_short: [WORD8; 8],
    pub fft_twiddle: [WORD32; 448],
    pub only_long_window_sine: [WORD16; 1024],
    pub only_long_window_kbd: [WORD16; 1024],
    pub only_short_window_sine: [WORD16; 128],
    pub only_short_window_kbd: [WORD16; 128],
    pub cosine_array_2048_256p: [WORD16; 514],
    pub w1024: [WORD32; 768],
    pub bit_rev_1024: [UWORD8; 256],
    pub bit_rev_512: [UWORD8; 64],
    pub bit_rev_128: [UWORD8; 16],
    pub bit_rev_32: [UWORD8; 4],
    pub w_256: [WORD32; 504],
    pub low_overlap_win: [WORD32; 512],
    pub window_sine_512: [WORD32; 512],
    pub cosine_array_1024: [WORD32; 512],
    pub low_overlap_win_480: [WORD32; 480],
    pub window_sine_480: [WORD32; 480],
    pub re_arr_tab_16: [UWORD8; 240],
    pub re_arr_tab_sml_240: [UWORD8; 240],
    pub cosine_array_960: [WORD32; 480],
    pub w_16: [WORD32; 24],
    pub window_sine_480_eld: [WORD16; 1920],
    pub window_sine_512_eld: [WORD16; 2048],
    pub only_long_window_sine_960: [WORD16; 960],
    pub only_long_window_kbd_960: [WORD16; 960],
    pub only_short_window_sine_120: [WORD16; 120],
    pub only_short_window_kbd_120: [WORD16; 120],
    pub re_arr_tab_32: [WORD16; 480],
    pub re_arr_tab_sml: [WORD16; 16],
    pub re_arr_tab_4: [WORD16; 60],
    pub re_arr_tab_15_4: [WORD16; 60],
    pub re_arr_tab_120: [WORD16; 60],
    pub re_arr_tab_5: [WORD16; 16],
    pub re_arr_tab_3: [WORD16; 16],
    pub re_arr_tab_sml_480: [WORD16; 480],
    pub cosine_array_1920: [WORD32; 960],
    pub w_512: [WORD16; 1020],
    pub w_32: [WORD16; 60],
    pub cosine_array_240: [WORD16; 120],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_sfb_info {
    pub sfb_index: *mut WORD16,
    pub sfb_width: *mut WORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_tables_struct {
    pub pstr_block_tables: *mut ia_aac_dec_block_tables_struct,
    pub pstr_huffmann_tables: *mut ia_aac_dec_huffman_tables_struct,
    pub pstr_imdct_tables: *mut ia_aac_dec_imdct_tables_struct,
    pub str_aac_sfb_info: [ia_aac_sfb_info; 4],
    pub scale_factor_bands_long: [*mut WORD8; 24],
    pub scale_factor_bands_short: [*mut WORD8; 24],
    pub sfb_long_table: [WORD16; 52],
    pub sfb_short_table: [WORD16; 16],
    pub code_book: [*mut UWORD16; 13],
    pub index_table: [*mut UWORD32; 13],
    pub scale_fac_bands_512: [*mut WORD8; 16],
    pub scale_fac_bands_480: [*mut WORD8; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ltp_info {
    pub last_band: UWORD8,
    pub data_present: UWORD8,
    pub lag: UWORD16,
    pub lag_update: UWORD8,
    pub coef: UWORD8,
    pub long_used: [UWORD8; 51],
    pub short_used: [UWORD8; 8],
    pub short_lag_present: [UWORD8; 8],
    pub short_lag: [UWORD8; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_ics_info_struct {
    pub window_shape: WORD16,
    pub window_sequence: WORD16,
    pub max_sfb: WORD16,
    pub num_swb_window: WORD16,
    pub sampling_rate_index: WORD16,
    pub num_window_groups: WORD16,
    pub window_group_length: [WORD8; 8],
    pub frame_length: WORD16,
    pub frame_size: WORD32,
    pub predictor_data_present: WORD16,
    pub ltp: ltp_info,
    pub ltp2: ltp_info,
    pub qshift_adj: WORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_overlap_info {
    pub ptr_long_window: [*const WORD16; 2],
    pub ptr_short_window: [*const WORD16; 2],
    pub window_shape: WORD16,
    pub window_sequence: WORD16,
    pub ptr_overlap_buf: *mut WORD32,
    pub rvlc_prev_sf: [WORD16; 128],
    pub rvlc_prev_cb: [WORD16; 128],
    pub rvlc_prev_blk_type: WORD8,
    pub rvlc_prev_sf_ok: WORD8,
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
pub const MAX_32: WORD32 = 0x7fffffff as core::ffi::c_long as WORD32;
pub const MIN_32: WORD32 = 0x80000000 as core::ffi::c_long as WORD32;
pub const MAX_16: WORD16 = 0x7fff as core::ffi::c_int as WORD16;
#[inline]
unsafe extern "C" fn ixheaac_min32(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut min_val: WORD32 = 0;
    min_val = if a < b { a } else { b };
    return min_val;
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
unsafe extern "C" fn ixheaac_shl32_sat(mut a: WORD32, mut b: WORD) -> WORD32 {
    let mut out_val: WORD32 = 0;
    if a > MAX_32 >> b {
        out_val = MAX_32;
    } else if a < MIN_32 >> b {
        out_val = MIN_32;
    } else {
        out_val = a << b;
    }
    return out_val;
}
#[inline]
unsafe extern "C" fn ixheaac_mult32x16in32_sat(mut a: WORD32, mut b: WORD16) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    if temp_result < MIN_32 as WORD64 {
        result = MIN_32;
    } else if temp_result > MAX_32 as WORD64 {
        result = MAX_32;
    } else {
        result = temp_result as WORD32;
    }
    return result;
}
#[inline]
unsafe extern "C" fn ixheaac_add32_sat(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut sum: WORD64 = 0;
    sum = a as WORD64 + b as WORD64;
    if sum >= MAX_32 as WORD64 {
        return MAX_32;
    }
    if sum <= MIN_32 as WORD64 {
        return MIN_32;
    }
    return sum as WORD32;
}
#[inline]
unsafe extern "C" fn ixheaac_sub32_sat(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut diff: WORD64 = 0;
    diff = a as WORD64 - b as WORD64;
    if diff >= MAX_32 as WORD64 {
        return MAX_32;
    }
    if diff <= MIN_32 as WORD64 {
        return MIN_32;
    }
    return diff as WORD32;
}
#[inline]
unsafe extern "C" fn ixheaac_negate32_sat(mut a: WORD32) -> WORD32 {
    let mut neg_val: WORD32 = 0;
    if a == MIN_32 {
        neg_val = MAX_32;
    } else {
        neg_val = -a;
    }
    return neg_val;
}
#[inline]
unsafe extern "C" fn ixheaac_shr32_sat(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut out_val: WORD32 = 0;
    b = ((b << 24 as core::ffi::c_int) as UWORD32 >> 24 as core::ffi::c_int) as WORD32;
    if b >= 31 as core::ffi::c_int {
        if a < 0 as core::ffi::c_int {
            out_val = -(1 as core::ffi::c_int) as WORD32;
        } else {
            out_val = 0 as core::ffi::c_int as WORD32;
        }
    } else if b <= 0 as core::ffi::c_int {
        return a
    } else {
        a = ixheaac_add32_sat(
            a,
            (1 as WORD32) << b as core::ffi::c_int - 1 as core::ffi::c_int,
        );
        out_val = a >> b;
    }
    return out_val;
}
#[inline]
unsafe extern "C" fn ixheaac_negate16(mut op1: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    if -(32768 as core::ffi::c_int) == op1 as core::ffi::c_int {
        var_out = MAX_16;
    } else {
        var_out = -(op1 as core::ffi::c_int) as WORD16;
    }
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_round16(mut op1: WORD32) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = (ixheaac_add32_sat(op1, 0x8000 as WORD32) >> 16 as core::ffi::c_int)
        as WORD16;
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_mult32x16in32(mut a: WORD32, mut b: WORD16) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 16 as core::ffi::c_int) as WORD32;
    return result;
}
#[inline]
unsafe extern "C" fn ixheaac_shl32_dir_sat_limit(mut a: WORD32, mut b: WORD) -> WORD32 {
    let mut out_val: WORD32 = 0;
    if b < 0 as core::ffi::c_int {
        b = -b;
        b = ixheaac_min32(b as WORD32, 31 as WORD32) as WORD;
        out_val = ixheaac_shr32(a, b);
    } else {
        out_val = ixheaac_shl32_sat(a, b);
    }
    return out_val;
}
pub const MAX_BINS_LONG: core::ffi::c_int = 1024 as core::ffi::c_int;
pub const SIZE01: core::ffi::c_int = MAX_BINS_LONG / 16 as core::ffi::c_int;
pub const ONLY_LONG_SEQUENCE: core::ffi::c_int = 0;
pub const LONG_START_SEQUENCE: core::ffi::c_int = 1;
pub const EIGHT_SHORT_SEQUENCE: core::ffi::c_int = 2;
pub const LONG_STOP_SEQUENCE: core::ffi::c_int = 3;
pub const MAX_WINDOWS: core::ffi::c_int = 8 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_process_win_seq(
    mut coef: *mut WORD32,
    mut prev: *mut WORD32,
    mut out: *mut WORD32,
    mut window_long: *const WORD16,
    mut window_short: *const WORD16,
    mut q_shift: WORD16,
    mut ch_fac: WORD16,
    mut flag: WORD16,
    mut size_01: WORD16,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut accu: WORD32 = 0;
    let mut coef_1: *mut WORD32 = 0 as *mut WORD32;
    let mut temp_win_sh: *const WORD16 = 0 as *const WORD16;
    let mut temp_win_long: *const WORD16 = 0 as *const WORD16;
    let mut out1: *mut WORD32 = 0 as *mut WORD32;
    let mut out2: *mut WORD32 = 0 as *mut WORD32;
    let mut temp_prev: *mut WORD32 = 0 as *mut WORD32;
    let mut size_07: WORD16 = (7 as core::ffi::c_int * size_01 as core::ffi::c_int)
        as WORD16;
    let mut size_08: WORD16 = (8 as core::ffi::c_int * size_01 as core::ffi::c_int)
        as WORD16;
    let mut size_09: WORD16 = (9 as core::ffi::c_int * size_01 as core::ffi::c_int)
        as WORD16;
    let mut size_14: WORD16 = (14 as core::ffi::c_int * size_01 as core::ffi::c_int)
        as WORD16;
    let mut size_15: WORD16 = (15 as core::ffi::c_int * size_01 as core::ffi::c_int)
        as WORD16;
    if flag as core::ffi::c_int == 1 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < size_07 as core::ffi::c_int {
            let mut temp1: WORD32 = ixheaac_shl32_dir_sat_limit(
                ixheaac_mult32x16in32(
                    *coef.offset((size_08 as WORD32 + i) as isize),
                    *window_long.offset((2 as WORD32 * i) as isize),
                ),
                q_shift as WORD + 1 as WORD,
            );
            accu = ixheaac_add32_sat(
                temp1,
                *prev.offset(i as isize) << 16 as core::ffi::c_int,
            );
            *out.offset((ch_fac as WORD32 * i) as isize) = accu;
            accu = ixheaac_shl32_dir_sat_limit(
                ixheaac_mult32x16in32(
                    -*coef.offset((size_15 as WORD32 - 1 as WORD32 - i) as isize),
                    *window_long
                        .offset(
                            (2 as core::ffi::c_int
                                * (size_07 as core::ffi::c_int - i as core::ffi::c_int)
                                - 1 as core::ffi::c_int) as isize,
                        ),
                ),
                q_shift as WORD,
            );
            *out
                .offset(
                    (ch_fac as core::ffi::c_int
                        * (i as core::ffi::c_int + size_09 as core::ffi::c_int)) as isize,
                ) = accu << 1 as core::ffi::c_int;
            i += 1;
        }
        temp_win_sh = &*window_short.offset(0 as core::ffi::c_int as isize)
            as *const WORD16;
        coef_1 = &mut *coef.offset(size_15 as isize) as *mut WORD32;
        temp_win_long = &*window_long.offset(size_14 as isize) as *const WORD16;
        temp_prev = &mut *prev
            .offset((size_08 as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
            as *mut WORD32;
        out1 = &mut *out
            .offset((ch_fac as core::ffi::c_int * size_07 as core::ffi::c_int) as isize)
            as *mut WORD32;
        out2 = &mut *out
            .offset(
                (ch_fac as core::ffi::c_int
                    * (size_09 as core::ffi::c_int - 1 as core::ffi::c_int)) as isize,
            ) as *mut WORD32;
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i < size_07 as core::ffi::c_int {
            accu = ixheaac_mult32x16in32_sat(
                *prev.offset((size_08 as WORD32 - 1 as WORD32 - i) as isize),
                ixheaac_negate16(
                    *window_long
                        .offset(
                            (2 as core::ffi::c_int * i as core::ffi::c_int
                                + 1 as core::ffi::c_int) as isize,
                        ),
                ),
            );
            *out.offset((ch_fac as WORD32 * i) as isize) = accu;
            accu = ixheaac_sub32_sat(
                ixheaac_shl32_dir_sat_limit(
                    -*coef.offset((size_15 as WORD32 - 1 as WORD32 - i) as isize),
                    q_shift as WORD - 1 as WORD,
                ),
                ixheaac_mult32x16in32_sat(
                    *prev
                        .offset(
                            (i as core::ffi::c_int + size_01 as core::ffi::c_int)
                                as isize,
                        ),
                    *window_long
                        .offset(
                            (2 as WORD32 * size_07 as WORD32 - 2 as WORD32
                                - 2 as WORD32 * i) as isize,
                        ),
                ),
            );
            *out.offset((ch_fac as WORD32 * (size_09 as WORD32 + i)) as isize) = accu;
            i += 1;
        }
        temp_win_sh = &*window_long.offset(size_14 as isize) as *const WORD16;
        coef_1 = &mut *coef.offset(size_15 as isize) as *mut WORD32;
        temp_win_long = &*window_short.offset(0 as core::ffi::c_int as isize)
            as *const WORD16;
        temp_prev = &mut *prev
            .offset((size_01 as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
            as *mut WORD32;
        out1 = &mut *out
            .offset((ch_fac as core::ffi::c_int * size_07 as core::ffi::c_int) as isize)
            as *mut WORD32;
        out2 = &mut *out
            .offset(
                (ch_fac as core::ffi::c_int
                    * (size_09 as core::ffi::c_int - 1 as core::ffi::c_int)) as isize,
            ) as *mut WORD32;
    }
    i = (size_01 as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while i >= 0 as core::ffi::c_int {
        let fresh4 = coef_1;
        coef_1 = coef_1.offset(1);
        let mut temp_coef: WORD32 = *fresh4;
        let fresh5 = temp_win_long;
        temp_win_long = temp_win_long.offset(1);
        let mut win1: WORD16 = *fresh5;
        let fresh6 = temp_win_long;
        temp_win_long = temp_win_long.offset(1);
        let mut win2: WORD16 = *fresh6;
        let fresh7 = temp_prev;
        temp_prev = temp_prev.offset(-1);
        let mut prev1: WORD32 = *fresh7;
        let fresh8 = temp_win_sh;
        temp_win_sh = temp_win_sh.offset(1);
        let mut win4: WORD16 = *fresh8;
        let fresh9 = temp_win_sh;
        temp_win_sh = temp_win_sh.offset(1);
        let mut win3: WORD16 = *fresh9;
        accu = ixheaac_sub32_sat(
            ixheaac_shl32_dir_sat_limit(
                ixheaac_mult32x16in32(temp_coef, win1),
                q_shift as WORD,
            ),
            ixheaac_mult32x16in32_sat(prev1, win3),
        );
        *out1 = accu << flag as core::ffi::c_int;
        out1 = out1.offset(ch_fac as core::ffi::c_int as isize);
        accu = ixheaac_sub32_sat(
            ixheaac_shl32_dir_sat_limit(
                ixheaac_mult32x16in32(ixheaac_negate32_sat(temp_coef), win2),
                q_shift as WORD,
            ),
            ixheaac_mult32x16in32_sat(prev1, win4),
        );
        *out2 = accu << flag as core::ffi::c_int;
        out2 = out2.offset(-(ch_fac as core::ffi::c_int as isize));
        i -= 1;
    }
}
#[inline]
unsafe extern "C" fn ixheaacd_long_short_win_process(
    mut current: *mut WORD32,
    mut prev: *mut WORD32,
    mut out: *mut WORD32,
    mut short_window: *const WORD16,
    mut long_window_prev: *const WORD16,
    mut q_shift: WORD16,
    mut ch_fac: WORD16,
    mut flag: WORD32,
    mut size_01: WORD16,
) -> VOID {
    let mut size_02: WORD16 = (2 as core::ffi::c_int * size_01 as core::ffi::c_int)
        as WORD16;
    let mut size_03: WORD16 = (3 as core::ffi::c_int * size_01 as core::ffi::c_int)
        as WORD16;
    let mut i: WORD = 0;
    let mut accu: WORD32 = 0;
    let mut current_tmp1: *mut WORD32 = &mut *current
        .offset((size_03 as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
        as *mut WORD32;
    let mut current_tmp2: *mut WORD32 = &mut *current
        .offset(-(size_01 as core::ffi::c_int) as isize) as *mut WORD32;
    let mut short_ptr: *const WORD16 = &*short_window
        .offset((size_02 as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
        as *const WORD16;
    i = (size_01 as core::ffi::c_int - 1 as core::ffi::c_int) as WORD;
    while i >= 0 as core::ffi::c_int {
        let fresh0 = current_tmp1;
        current_tmp1 = current_tmp1.offset(-1);
        let mut tmp1_cur: WORD32 = *fresh0;
        let fresh1 = current_tmp2;
        current_tmp2 = current_tmp2.offset(1);
        let mut tmp2_cur: WORD32 = *fresh1;
        let fresh2 = short_ptr;
        short_ptr = short_ptr.offset(-1);
        let mut short1: WORD16 = *fresh2;
        let fresh3 = short_ptr;
        short_ptr = short_ptr.offset(-1);
        let mut short2: WORD16 = *fresh3;
        accu = ixheaac_sub32_sat(
            ixheaac_shl32_dir_sat_limit(
                ixheaac_mult32x16in32(tmp1_cur, short2)
                    - ixheaac_mult32x16in32(tmp2_cur, short1),
                q_shift as WORD,
            ),
            ixheaac_mult32x16in32_sat(
                *prev.offset(i as isize),
                *long_window_prev
                    .offset((0 as WORD - 2 as WORD - 2 as WORD * i) as isize),
            ),
        );
        *out.offset((ch_fac as WORD * (0 as WORD + i)) as isize) = accu;
        if flag != 0 {
            accu = ixheaac_sub32_sat(
                ixheaac_shl32_dir_sat_limit(
                    ixheaac_mult32x16in32(ixheaac_negate32_sat(tmp1_cur), short1)
                        - ixheaac_mult32x16in32(tmp2_cur, short2),
                    q_shift as WORD,
                ),
                ixheaac_mult32x16in32_sat(
                    *prev.offset((size_02 as WORD - 1 as WORD - i) as isize),
                    *long_window_prev
                        .offset(
                            (-(2 as WORD) * size_02 as WORD + 2 as WORD * i) as isize,
                        ),
                ),
            );
            *out.offset((ch_fac as WORD * (size_02 as WORD - 1 as WORD - i)) as isize) = accu;
        }
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_long_short_win_seq(
    mut current: *mut WORD32,
    mut prev: *mut WORD32,
    mut out: *mut WORD32,
    mut short_window: *const WORD16,
    mut short_window_prev: *const WORD16,
    mut long_window_prev: *const WORD16,
    mut q_shift: WORD16,
    mut ch_fac: WORD16,
    mut size_01: WORD16,
) -> VOID {
    let mut size_02: WORD16 = (2 as core::ffi::c_int * size_01 as core::ffi::c_int)
        as WORD16;
    let mut size_06: WORD16 = (6 as core::ffi::c_int * size_01 as core::ffi::c_int)
        as WORD16;
    let mut size_07: WORD16 = (7 as core::ffi::c_int * size_01 as core::ffi::c_int)
        as WORD16;
    let mut size_08: WORD16 = (8 as core::ffi::c_int * size_01 as core::ffi::c_int)
        as WORD16;
    let mut size_09: WORD16 = (9 as core::ffi::c_int * size_01 as core::ffi::c_int)
        as WORD16;
    let mut size_10: WORD16 = (10 as core::ffi::c_int * size_01 as core::ffi::c_int)
        as WORD16;
    let mut size_16: WORD16 = (16 as core::ffi::c_int * size_01 as core::ffi::c_int)
        as WORD16;
    let mut i: WORD32 = 0;
    let mut flag: WORD32 = 0;
    let mut accu: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < size_07 as core::ffi::c_int {
        accu = ixheaac_mult32x16in32_sat(
            *prev.offset((size_08 as WORD32 - 1 as WORD32 - i) as isize),
            ixheaac_negate16(
                *long_window_prev
                    .offset(
                        (2 as core::ffi::c_int * i as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    ),
            ),
        );
        *out.offset((ch_fac as WORD32 * i) as isize) = accu;
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < size_01 as core::ffi::c_int {
        accu = ixheaac_sub32_sat(
            ixheaac_shl32_dir_sat_limit(
                ixheaac_mult32x16in32(
                    *current.offset((size_01 as WORD32 + i) as isize),
                    *short_window_prev.offset((2 as WORD32 * i) as isize),
                ),
                q_shift as WORD,
            ),
            ixheaac_mult32x16in32_sat(
                *prev.offset((size_01 as WORD32 - 1 as WORD32 - i) as isize),
                *long_window_prev
                    .offset(
                        (2 as WORD32 * size_07 as WORD32 + 1 as WORD32 + 2 as WORD32 * i)
                            as isize,
                    ),
            ),
        );
        *out.offset((ch_fac as WORD32 * (size_07 as WORD32 + i)) as isize) = accu;
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < size_01 as core::ffi::c_int {
        accu = ixheaac_sub32_sat(
            ixheaac_shl32_dir_sat_limit(
                ixheaac_mult32x16in32(
                    ixheaac_negate32_sat(
                        *current.offset((size_02 as WORD32 - 1 as WORD32 - i) as isize),
                    ),
                    *short_window_prev
                        .offset(
                            (size_02 as core::ffi::c_int
                                - 2 as core::ffi::c_int * i as core::ffi::c_int
                                - 1 as core::ffi::c_int) as isize,
                        ),
                ),
                q_shift as WORD,
            ),
            ixheaac_mult32x16in32_sat(
                *prev.offset(i as isize),
                *long_window_prev
                    .offset((size_16 as WORD32 - 2 as WORD32 - 2 as WORD32 * i) as isize),
            ),
        );
        *out.offset((ch_fac as WORD32 * (size_08 as WORD32 + i)) as isize) = accu;
        i += 1;
    }
    flag = 1 as core::ffi::c_int as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < 4 as core::ffi::c_int {
        let mut inc: WORD32 = i * size_02 as WORD32;
        if i == 3 as core::ffi::c_int {
            flag = 0 as core::ffi::c_int as WORD32;
        }
        ixheaacd_long_short_win_process(
            &mut *current.offset((size_01 as WORD32 + inc) as isize),
            &mut *prev.offset((size_01 as WORD32 + inc) as isize),
            &mut *out.offset((ch_fac as WORD32 * (size_09 as WORD32 + inc)) as isize),
            short_window,
            &*long_window_prev
                .offset((2 as WORD32 * (size_07 as WORD32 - inc)) as isize),
            q_shift,
            ch_fac,
            flag,
            size_01,
        );
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < size_01 as core::ffi::c_int {
        accu = ixheaac_mult32x16in32(
            -*current.offset((size_10 as WORD32 - 1 as WORD32 - i) as isize),
            *short_window
                .offset(
                    (size_02 as core::ffi::c_int
                        - 2 as core::ffi::c_int * i as core::ffi::c_int
                        - 1 as core::ffi::c_int) as isize,
                ),
        )
            - ixheaac_mult32x16in32(
                *current.offset((size_06 as WORD32 + i) as isize),
                *short_window
                    .offset(
                        (size_02 as core::ffi::c_int
                            - 2 as core::ffi::c_int * i as core::ffi::c_int
                            - 2 as core::ffi::c_int) as isize,
                    ),
            );
        *prev.offset(i as isize) = ixheaac_round16(
            ixheaac_shl32_dir_sat_limit(accu, q_shift as WORD + 1 as WORD),
        ) as WORD32;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_nolap1_32(
    mut coef: *mut WORD32,
    mut out: *mut WORD32,
    mut q_shift: WORD16,
    mut ch_fac: WORD16,
    mut size_01: WORD16,
) -> VOID {
    let mut size_07: WORD16 = (7 as core::ffi::c_int * size_01 as core::ffi::c_int)
        as WORD16;
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < size_07 as core::ffi::c_int {
        *out.offset((ch_fac as WORD32 * i) as isize) = ixheaac_shr32_sat(
            ixheaac_negate32_sat(
                *coef.offset((size_07 as WORD32 - 1 as WORD32 - i) as isize),
            ),
            16 as WORD32 - q_shift as WORD32,
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_neg_shift_spec_dec(
    mut coef: *mut WORD32,
    mut out: *mut WORD32,
    mut q_shift: WORD16,
    mut ch_fac: WORD16,
) -> VOID {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < 7 as core::ffi::c_int * SIZE01 {
        *out.offset((ch_fac as WORD32 * i) as isize) = ixheaac_shl32_dir_sat_limit(
            ixheaac_negate32_sat(
                *coef.offset((7 as WORD32 * SIZE01 - 1 as WORD32 - i) as isize),
            ),
            q_shift as WORD,
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_Nolap_dec(
    mut coef: *mut WORD32,
    mut out: *mut WORD32,
    mut q_shift: WORD16,
    mut ch_fac: WORD16,
    mut size_01: WORD16,
) -> VOID {
    let mut size_07: WORD16 = (7 as core::ffi::c_int * size_01 as core::ffi::c_int)
        as WORD16;
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < size_07 as core::ffi::c_int {
        *out.offset((ch_fac as WORD32 * i) as isize) = ixheaac_shl32_dir_sat_limit(
            ixheaac_negate32_sat(
                *coef.offset((size_07 as WORD32 - 1 as WORD32 - i) as isize),
            ),
            q_shift as WORD,
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_spec_to_overlapbuf_dec(
    mut ptr_overlap_buf: *mut WORD32,
    mut ptr_spec_coeff: *mut WORD32,
    mut q_shift: WORD32,
    mut size: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < size {
        *ptr_overlap_buf.offset(i as isize) = ixheaac_shr32_sat(
            *ptr_spec_coeff.offset(i as isize),
            16 as WORD32 - q_shift,
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_overlap_buf_out_dec(
    mut out_samples: *mut WORD32,
    mut ptr_overlap_buf: *mut WORD32,
    mut size: WORD32,
    ch_fac: WORD16,
) -> VOID {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < size {
        *out_samples.offset((ch_fac as WORD32 * i) as isize) = ixheaac_shl32_sat(
            *ptr_overlap_buf.offset(i as isize) as WORD16 as WORD32,
            15 as WORD,
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_overlap_out_copy_dec(
    mut out_samples: *mut WORD32,
    mut ptr_overlap_buf: *mut WORD32,
    mut ptr_overlap_buf1: *mut WORD32,
    ch_fac: WORD16,
    mut size_01: WORD16,
) -> VOID {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < size_01 as core::ffi::c_int {
        *out_samples.offset((ch_fac as WORD32 * i) as isize) = ixheaac_shl32_sat(
            *ptr_overlap_buf.offset(i as isize) as WORD16 as WORD32,
            15 as WORD,
        );
        *ptr_overlap_buf.offset(i as isize) = *ptr_overlap_buf1.offset(i as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_imdct_process(
    mut ptr_aac_dec_overlap_info: *mut ia_aac_dec_overlap_info,
    mut ptr_spec_coeff: *mut WORD32,
    mut ptr_ics_info: *mut ia_ics_info_struct,
    mut out_samples: *mut core::ffi::c_void,
    ch_fac: WORD16,
    mut scratch: *mut WORD32,
    mut ptr_aac_tables: *mut ia_aac_dec_tables_struct,
    mut object_type: WORD32,
    mut ld_mps_present: WORD32,
    mut slot_element: WORD,
) -> VOID {
    let mut ptr_overlap_buf: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr_long_window: *const WORD16 = 0 as *const WORD16;
    let mut ptr_short_window: *const WORD16 = 0 as *const WORD16;
    let mut max_bin_long: WORD16 = (*ptr_ics_info).frame_length;
    let mut size_01: WORD16 = 0;
    if max_bin_long as core::ffi::c_int == 960 as core::ffi::c_int {
        size_01 = (max_bin_long as core::ffi::c_int / 16 as core::ffi::c_int) as WORD16;
    } else {
        size_01 = (MAX_BINS_LONG / 16 as core::ffi::c_int) as WORD16;
    }
    let mut size_02: WORD16 = (2 as core::ffi::c_int * size_01 as core::ffi::c_int)
        as WORD16;
    let mut size_04: WORD16 = (4 as core::ffi::c_int * size_01 as core::ffi::c_int)
        as WORD16;
    let mut size_06: WORD16 = (6 as core::ffi::c_int * size_01 as core::ffi::c_int)
        as WORD16;
    let mut size_07: WORD16 = (7 as core::ffi::c_int * size_01 as core::ffi::c_int)
        as WORD16;
    let mut size_08: WORD16 = (8 as core::ffi::c_int * size_01 as core::ffi::c_int)
        as WORD16;
    let mut size_09: WORD16 = (9 as core::ffi::c_int * size_01 as core::ffi::c_int)
        as WORD16;
    let mut size_10: WORD16 = (10 as core::ffi::c_int * size_01 as core::ffi::c_int)
        as WORD16;
    let mut size_14: WORD16 = (14 as core::ffi::c_int * size_01 as core::ffi::c_int)
        as WORD16;
    let mut size_15: WORD16 = (15 as core::ffi::c_int * size_01 as core::ffi::c_int)
        as WORD16;
    ptr_overlap_buf = (*ptr_aac_dec_overlap_info).ptr_overlap_buf;
    ptr_long_window = (*ptr_aac_dec_overlap_info)
        .ptr_long_window[(*ptr_aac_dec_overlap_info).window_shape as WORD32 as usize];
    ptr_short_window = (*ptr_aac_dec_overlap_info)
        .ptr_short_window[(*ptr_aac_dec_overlap_info).window_shape as WORD32 as usize];
    if (*ptr_ics_info).window_sequence as core::ffi::c_int != EIGHT_SHORT_SEQUENCE {
        let mut q_shift: WORD16 = 0;
        let mut expo: WORD32 = 0;
        let mut imdct_scale: WORD32 = 0;
        if 512 as core::ffi::c_int == (*ptr_ics_info).frame_length as core::ffi::c_int
            || 480 as core::ffi::c_int
                == (*ptr_ics_info).frame_length as core::ffi::c_int
        {
            (*ptr_ics_info).qshift_adj = -(2 as core::ffi::c_int) as WORD8;
            if 512 as core::ffi::c_int
                == (*ptr_ics_info).frame_length as core::ffi::c_int
            {
                let mut ld_cos_sin_ptr: *mut WORD32 = ((*(*ptr_aac_tables)
                    .pstr_imdct_tables)
                    .cosine_array_1024)
                    .as_mut_ptr();
                ixheaacd_inverse_transform_512(
                    ptr_spec_coeff as *mut WORD32,
                    scratch as *mut WORD32,
                    &mut imdct_scale,
                    ld_cos_sin_ptr,
                    (*ptr_aac_tables).pstr_imdct_tables,
                    object_type,
                );
            } else {
                ixheaacd_mdct_480_ld(
                    ptr_spec_coeff,
                    scratch,
                    &mut imdct_scale,
                    0 as WORD32,
                    (*ptr_aac_tables).pstr_imdct_tables,
                    object_type,
                );
            }
            if object_type == AOT_ER_AAC_ELD as core::ffi::c_int {
                let mut i: core::ffi::c_int = 0;
                let mut N: core::ffi::c_int = ((*ptr_ics_info).frame_length
                    as core::ffi::c_int) << 1 as core::ffi::c_int;
                i = 0 as core::ffi::c_int;
                while i < N / 2 as core::ffi::c_int {
                    *ptr_spec_coeff.offset(i as isize) = -*ptr_spec_coeff
                        .offset((i + N) as isize);
                    *ptr_spec_coeff
                        .offset((i + N + N / 2 as core::ffi::c_int) as isize) = -*ptr_spec_coeff
                        .offset((i + N / 2 as core::ffi::c_int) as isize);
                    i += 1;
                }
            }
        } else if 960 as core::ffi::c_int
            == (*ptr_ics_info).frame_length as core::ffi::c_int
        {
            ixheaacd_mdct_960(
                ptr_spec_coeff,
                scratch,
                &mut imdct_scale,
                0 as WORD32,
                (*ptr_aac_tables).pstr_imdct_tables,
            );
        } else {
            expo = ((Some(
                ixheaacd_calc_max_spectral_line.expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")(ptr_spec_coeff, 1024 as WORD32)
                as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
            expo = 8 as WORD32 - expo;
            imdct_scale = ixheaacd_inverse_transform(
                ptr_spec_coeff as *mut WORD32,
                scratch as *mut WORD32,
                (*ptr_aac_tables).pstr_imdct_tables,
                expo,
                1024 as WORD32,
            );
        }
        q_shift = (31 as core::ffi::c_int + imdct_scale as core::ffi::c_int
            + (-(1 as core::ffi::c_int) - 16 as core::ffi::c_int
                - 9 as core::ffi::c_int)) as WORD16;
        match (*ptr_ics_info).window_sequence as core::ffi::c_int {
            ONLY_LONG_SEQUENCE => {
                match (*ptr_aac_dec_overlap_info).window_sequence as core::ffi::c_int {
                    ONLY_LONG_SEQUENCE | LONG_STOP_SEQUENCE => {
                        if 1024 as core::ffi::c_int
                            == (*ptr_ics_info).frame_length as core::ffi::c_int
                        {
                            let mut tmp_ptr_ics_info: *mut ia_ics_info_struct = ptr_ics_info;
                            (Some(
                                ixheaacd_post_twid_overlap_add
                                    .expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                out_samples as *mut WORD32,
                                ptr_spec_coeff as *mut WORD32,
                                (*ptr_aac_tables).pstr_imdct_tables,
                                1024 as WORD,
                                ptr_overlap_buf,
                                q_shift,
                                ptr_long_window,
                                ch_fac,
                            );
                            (*ptr_ics_info).qshift_adj = 2 as WORD8;
                            ptr_ics_info = tmp_ptr_ics_info;
                        }
                        if 960 as core::ffi::c_int
                            == (*ptr_ics_info).frame_length as core::ffi::c_int
                        {
                            ixheaacd_over_lap_add1_dec(
                                ptr_spec_coeff,
                                ptr_overlap_buf,
                                out_samples as *mut WORD32,
                                ptr_long_window,
                                q_shift,
                                480 as WORD16,
                                ch_fac,
                            );
                            ixheaacd_spec_to_overlapbuf
                                .expect(
                                    "non-null function pointer",
                                )(
                                ptr_overlap_buf,
                                ptr_spec_coeff,
                                q_shift as WORD32,
                                480 as WORD32,
                            );
                            (*ptr_ics_info).qshift_adj = 2 as WORD8;
                        }
                        if 512 as core::ffi::c_int
                            == (*ptr_ics_info).frame_length as core::ffi::c_int
                            || 480 as core::ffi::c_int
                                == (*ptr_ics_info).frame_length as core::ffi::c_int
                        {
                            if object_type != AOT_ER_AAC_ELD as core::ffi::c_int {
                                if 512 as core::ffi::c_int
                                    == (*ptr_ics_info).frame_length as core::ffi::c_int
                                {
                                    ixheaacd_lap1_512_480(
                                        ptr_spec_coeff,
                                        ptr_overlap_buf,
                                        out_samples as *mut WORD16 as *mut core::ffi::c_void,
                                        ptr_long_window,
                                        q_shift,
                                        size_04,
                                        ch_fac,
                                        slot_element,
                                    );
                                    ixheaacd_spec_to_overlapbuf
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        ptr_overlap_buf,
                                        ptr_spec_coeff,
                                        q_shift as WORD32,
                                        size_04 as WORD32,
                                    );
                                } else if 480 as core::ffi::c_int
                                    == (*ptr_ics_info).frame_length as core::ffi::c_int
                                {
                                    ixheaacd_lap1_512_480(
                                        ptr_spec_coeff,
                                        ptr_overlap_buf,
                                        out_samples as *mut WORD16 as *mut core::ffi::c_void,
                                        ptr_long_window,
                                        q_shift,
                                        240 as WORD16,
                                        ch_fac,
                                        slot_element,
                                    );
                                    ixheaacd_spec_to_overlapbuf
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        ptr_overlap_buf,
                                        ptr_spec_coeff,
                                        q_shift as WORD32,
                                        240 as WORD32,
                                    );
                                }
                            } else {
                                if ld_mps_present == 1 as core::ffi::c_int {
                                    ixheaacd_eld_dec_windowing_32bit(
                                        ptr_spec_coeff,
                                        ptr_long_window,
                                        (*ptr_ics_info).frame_length as WORD32,
                                        q_shift,
                                        ptr_overlap_buf,
                                        ch_fac,
                                        out_samples as *mut WORD32,
                                    );
                                } else {
                                    ixheaacd_eld_dec_windowing(
                                        ptr_spec_coeff,
                                        ptr_long_window,
                                        (*ptr_ics_info).frame_length as WORD32,
                                        q_shift,
                                        ptr_overlap_buf,
                                        ch_fac,
                                        out_samples as *mut WORD16 as *mut core::ffi::c_void,
                                        slot_element,
                                    );
                                }
                                (*ptr_ics_info).qshift_adj = -(2 as core::ffi::c_int)
                                    as WORD8;
                            }
                        }
                    }
                    LONG_START_SEQUENCE | EIGHT_SHORT_SEQUENCE => {
                        if 1024 as core::ffi::c_int
                            == (*ptr_ics_info).frame_length as core::ffi::c_int
                        {
                            (Some(
                                ixheaacd_post_twiddle.expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                scratch as *mut WORD32,
                                ptr_spec_coeff as *mut WORD32,
                                (*ptr_aac_tables).pstr_imdct_tables,
                                1024 as WORD,
                            );
                            ixheaacd_process_win_seq(
                                scratch,
                                ptr_overlap_buf,
                                out_samples as *mut WORD32,
                                ptr_long_window,
                                ptr_short_window,
                                q_shift,
                                ch_fac,
                                1 as WORD16,
                                size_01,
                            );
                            (Some(
                                ixheaacd_spec_to_overlapbuf
                                    .expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                ptr_overlap_buf,
                                scratch,
                                q_shift as WORD32,
                                size_08 as WORD32,
                            );
                        }
                        if 960 as core::ffi::c_int
                            == (*ptr_ics_info).frame_length as core::ffi::c_int
                        {
                            ixheaacd_process_win_seq(
                                ptr_spec_coeff,
                                ptr_overlap_buf,
                                out_samples as *mut WORD32,
                                ptr_long_window,
                                ptr_short_window,
                                q_shift,
                                ch_fac,
                                1 as WORD16,
                                size_01,
                            );
                            ixheaacd_spec_to_overlapbuf
                                .expect(
                                    "non-null function pointer",
                                )(
                                ptr_overlap_buf,
                                ptr_spec_coeff,
                                q_shift as WORD32,
                                480 as WORD32,
                            );
                        }
                        (*ptr_ics_info).qshift_adj = 1 as WORD8;
                        if 512 as core::ffi::c_int
                            == (*ptr_ics_info).frame_length as core::ffi::c_int
                        {
                            ixheaacd_spec_to_overlapbuf
                                .expect(
                                    "non-null function pointer",
                                )(
                                ptr_overlap_buf,
                                ptr_spec_coeff,
                                q_shift as WORD32,
                                size_04 as WORD32,
                            );
                        }
                        if 480 as core::ffi::c_int
                            == (*ptr_ics_info).frame_length as core::ffi::c_int
                        {
                            ixheaacd_spec_to_overlapbuf
                                .expect(
                                    "non-null function pointer",
                                )(
                                ptr_overlap_buf,
                                ptr_spec_coeff,
                                q_shift as WORD32,
                                240 as WORD32,
                            );
                        }
                    }
                    _ => {}
                }
            }
            LONG_START_SEQUENCE => {
                if 1024 as core::ffi::c_int
                    == (*ptr_ics_info).frame_length as core::ffi::c_int
                {
                    (Some(ixheaacd_post_twiddle.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(
                        scratch as *mut WORD32,
                        ptr_spec_coeff as *mut WORD32,
                        (*ptr_aac_tables).pstr_imdct_tables,
                        1024 as WORD,
                    );
                }
                match (*ptr_aac_dec_overlap_info).window_sequence as core::ffi::c_int {
                    ONLY_LONG_SEQUENCE | LONG_STOP_SEQUENCE => {
                        if 1024 as core::ffi::c_int
                            == (*ptr_ics_info).frame_length as core::ffi::c_int
                        {
                            (Some(
                                ixheaacd_over_lap_add1.expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                scratch,
                                ptr_overlap_buf,
                                out_samples as *mut WORD32,
                                ptr_long_window,
                                q_shift,
                                size_08,
                                ch_fac,
                            );
                        } else {
                            ixheaacd_over_lap_add1_dec(
                                ptr_spec_coeff,
                                ptr_overlap_buf,
                                out_samples as *mut WORD32,
                                ptr_long_window,
                                q_shift,
                                size_08,
                                ch_fac,
                            );
                        }
                        (*ptr_ics_info).qshift_adj = 2 as WORD8;
                    }
                    LONG_START_SEQUENCE | EIGHT_SHORT_SEQUENCE => {
                        if 1024 as core::ffi::c_int
                            == (*ptr_ics_info).frame_length as core::ffi::c_int
                        {
                            ixheaacd_process_win_seq(
                                scratch,
                                ptr_overlap_buf,
                                out_samples as *mut WORD32,
                                ptr_long_window,
                                ptr_short_window,
                                q_shift,
                                ch_fac,
                                1 as WORD16,
                                size_01,
                            );
                        } else {
                            ixheaacd_process_win_seq(
                                ptr_spec_coeff,
                                ptr_overlap_buf,
                                out_samples as *mut WORD32,
                                ptr_long_window,
                                ptr_short_window,
                                q_shift,
                                ch_fac,
                                1 as WORD16,
                                size_01,
                            );
                        }
                        (*ptr_ics_info).qshift_adj = 1 as WORD8;
                    }
                    _ => {}
                }
                if 960 as core::ffi::c_int
                    != (*ptr_ics_info).frame_length as core::ffi::c_int
                {
                    ixheaacd_nolap1_32(
                        &mut *scratch.offset(size_01 as isize),
                        ptr_overlap_buf,
                        q_shift,
                        1 as WORD16,
                        size_01,
                    );
                    (Some(
                        ixheaacd_spec_to_overlapbuf.expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        &mut *ptr_overlap_buf.offset(size_07 as isize),
                        scratch,
                        q_shift as WORD32,
                        size_01 as WORD32,
                    );
                } else {
                    ixheaacd_nolap1_32(
                        &mut *ptr_spec_coeff.offset(size_01 as isize),
                        ptr_overlap_buf,
                        q_shift,
                        1 as WORD16,
                        size_01,
                    );
                    (Some(
                        ixheaacd_spec_to_overlapbuf.expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        &mut *ptr_overlap_buf.offset(size_07 as isize),
                        ptr_spec_coeff,
                        q_shift as WORD32,
                        size_01 as WORD32,
                    );
                }
            }
            LONG_STOP_SEQUENCE => {
                if 1024 as core::ffi::c_int
                    == (*ptr_ics_info).frame_length as core::ffi::c_int
                {
                    (Some(ixheaacd_post_twiddle.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(
                        scratch as *mut WORD32,
                        ptr_spec_coeff as *mut WORD32,
                        (*ptr_aac_tables).pstr_imdct_tables,
                        1024 as WORD,
                    );
                }
                match (*ptr_aac_dec_overlap_info).window_sequence as core::ffi::c_int {
                    EIGHT_SHORT_SEQUENCE | LONG_START_SEQUENCE => {
                        if 960 as core::ffi::c_int
                            != (*ptr_ics_info).frame_length as core::ffi::c_int
                        {
                            (Some(
                                ixheaacd_overlap_buf_out.expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                out_samples as *mut WORD32,
                                ptr_overlap_buf,
                                size_07 as WORD32,
                                ch_fac,
                            );
                            (Some(
                                ixheaacd_over_lap_add1.expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                &mut *scratch.offset(size_14 as isize),
                                &mut *ptr_overlap_buf.offset(size_07 as isize),
                                (out_samples as *mut WORD32)
                                    .offset(
                                        (ch_fac as core::ffi::c_int * size_07 as core::ffi::c_int)
                                            as isize,
                                    ),
                                ptr_short_window,
                                q_shift,
                                size_01,
                                ch_fac,
                            );
                        } else {
                            ixheaacd_dec_copy_outsample(
                                out_samples as *mut WORD32,
                                ptr_overlap_buf,
                                size_07 as WORD32,
                                ch_fac,
                            );
                            ixheaacd_over_lap_add1_dec(
                                &mut *ptr_spec_coeff.offset(size_14 as isize),
                                &mut *ptr_overlap_buf.offset(size_07 as isize),
                                (out_samples as *mut WORD32)
                                    .offset(
                                        (ch_fac as core::ffi::c_int * size_07 as core::ffi::c_int)
                                            as isize,
                                    ),
                                ptr_short_window,
                                q_shift,
                                size_01,
                                ch_fac,
                            );
                        }
                        if 960 as core::ffi::c_int
                            != (*ptr_ics_info).frame_length as core::ffi::c_int
                        {
                            let mut q_shift1: WORD16 = (q_shift as core::ffi::c_int
                                - 1 as core::ffi::c_int) as WORD16;
                            (Some(
                                ixheaacd_neg_shift_spec.expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                &mut *scratch.offset(size_08 as isize),
                                (out_samples as *mut WORD32)
                                    .offset(
                                        (ch_fac as core::ffi::c_int * size_09 as core::ffi::c_int)
                                            as isize,
                                    ),
                                q_shift1,
                                ch_fac,
                            );
                        } else {
                            let mut q_shift1_0: WORD16 = (q_shift as core::ffi::c_int
                                - 1 as core::ffi::c_int) as WORD16;
                            ixheaacd_Nolap_dec(
                                &mut *ptr_spec_coeff.offset(size_08 as isize),
                                (out_samples as *mut WORD32)
                                    .offset(
                                        (ch_fac as core::ffi::c_int * size_09 as core::ffi::c_int)
                                            as isize,
                                    ),
                                q_shift1_0,
                                ch_fac,
                                size_01,
                            );
                        }
                        (*ptr_ics_info).qshift_adj = 2 as WORD8;
                    }
                    ONLY_LONG_SEQUENCE | LONG_STOP_SEQUENCE => {
                        if 1024 as core::ffi::c_int
                            == (*ptr_ics_info).frame_length as core::ffi::c_int
                        {
                            ixheaacd_process_win_seq(
                                scratch,
                                ptr_overlap_buf,
                                out_samples as *mut WORD32,
                                ptr_long_window,
                                ptr_short_window,
                                q_shift,
                                ch_fac,
                                0 as WORD16,
                                size_01,
                            );
                        } else {
                            ixheaacd_process_win_seq(
                                ptr_spec_coeff,
                                ptr_overlap_buf,
                                out_samples as *mut WORD32,
                                ptr_long_window,
                                ptr_short_window,
                                q_shift,
                                ch_fac,
                                0 as WORD16,
                                size_01,
                            );
                        }
                        (*ptr_ics_info).qshift_adj = 2 as WORD8;
                    }
                    _ => {}
                }
                if 1024 as core::ffi::c_int
                    == (*ptr_ics_info).frame_length as core::ffi::c_int
                {
                    (Some(
                        ixheaacd_spec_to_overlapbuf.expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        ptr_overlap_buf,
                        scratch,
                        q_shift as WORD32,
                        size_08 as WORD32,
                    );
                } else {
                    (Some(
                        ixheaacd_spec_to_overlapbuf.expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        ptr_overlap_buf,
                        ptr_spec_coeff,
                        q_shift as WORD32,
                        size_08 as WORD32,
                    );
                }
            }
            _ => {}
        }
    } else {
        let mut q_shift_0: WORD16 = 0;
        let mut max_scale: WORD16 = 0;
        let mut imdct_scale_0: [WORD32; 8] = [0; 8];
        let mut i_0: WORD32 = 0;
        let mut short_window: *const WORD16 = 0 as *const WORD16;
        short_window = (*ptr_aac_dec_overlap_info)
            .ptr_short_window[(*ptr_ics_info).window_shape as WORD32 as usize];
        let mut expo_0: WORD32 = 0;
        if 1024 as core::ffi::c_int == (*ptr_ics_info).frame_length as core::ffi::c_int {
            expo_0 = ((Some(
                ixheaacd_calc_max_spectral_line.expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")(ptr_spec_coeff, 1024 as WORD32)
                as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
            expo_0 = 5 as WORD32 - expo_0;
            i_0 = 0 as core::ffi::c_int as WORD32;
            while i_0 < MAX_WINDOWS {
                imdct_scale_0[i_0 as usize] = ixheaacd_inverse_transform(
                    &mut *ptr_spec_coeff
                        .offset(
                            (i_0 as core::ffi::c_int * size_02 as core::ffi::c_int)
                                as isize,
                        ),
                    &mut *scratch
                        .offset(
                            (i_0 as core::ffi::c_int * size_02 as core::ffi::c_int)
                                as isize,
                        ),
                    (*ptr_aac_tables).pstr_imdct_tables,
                    expo_0,
                    128 as WORD32,
                );
                (Some(ixheaacd_post_twiddle.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    &mut *scratch
                        .offset(
                            (i_0 as core::ffi::c_int * size_02 as core::ffi::c_int)
                                as isize,
                        ),
                    &mut *ptr_spec_coeff
                        .offset(
                            (i_0 as core::ffi::c_int * size_02 as core::ffi::c_int)
                                as isize,
                        ),
                    (*ptr_aac_tables).pstr_imdct_tables,
                    128 as WORD,
                );
                i_0 += 1;
            }
            max_scale = (31 as WORD32 + imdct_scale_0[0 as core::ffi::c_int as usize])
                as WORD16;
            q_shift_0 = (max_scale as core::ffi::c_int
                + (-(16 as core::ffi::c_int) - 6 as core::ffi::c_int
                    - 1 as core::ffi::c_int)) as WORD16;
        } else {
            expo_0 = ((Some(
                ixheaacd_calc_max_spectral_line.expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")(ptr_spec_coeff, 960 as WORD32)
                as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
            memcpy(
                scratch as *mut core::ffi::c_void,
                ptr_spec_coeff as *const core::ffi::c_void,
                (::core::mem::size_of::<WORD32>() as size_t).wrapping_mul(960 as size_t),
            );
            i_0 = 0 as core::ffi::c_int as WORD32;
            while i_0 < MAX_WINDOWS {
                ixheaacd_inverse_transform_960(
                    &mut *ptr_spec_coeff
                        .offset(
                            (i_0 as core::ffi::c_int * size_02 as core::ffi::c_int)
                                as isize,
                        ),
                    &mut *scratch
                        .offset(
                            (i_0 as core::ffi::c_int * size_02 as core::ffi::c_int)
                                as isize,
                        ),
                    (*ptr_aac_tables).pstr_imdct_tables,
                    expo_0,
                    &mut *imdct_scale_0.as_mut_ptr().offset(i_0 as isize),
                );
                imdct_scale_0[i_0 as usize] -= expo_0;
                i_0 += 1;
            }
            max_scale = (31 as WORD32 + imdct_scale_0[0 as core::ffi::c_int as usize])
                as WORD16;
            q_shift_0 = (max_scale as core::ffi::c_int
                + (-(16 as core::ffi::c_int) - 6 as core::ffi::c_int
                    - 1 as core::ffi::c_int)) as WORD16;
        }
        let mut overlap_buf_loc: [WORD32; 64] = [0; 64];
        match (*ptr_aac_dec_overlap_info).window_sequence as core::ffi::c_int {
            EIGHT_SHORT_SEQUENCE | LONG_START_SEQUENCE => {
                (Some(ixheaacd_overlap_buf_out.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    out_samples as *mut WORD32,
                    ptr_overlap_buf,
                    size_07 as WORD32,
                    ch_fac,
                );
                if 1024 as core::ffi::c_int
                    == (*ptr_ics_info).frame_length as core::ffi::c_int
                {
                    (Some(ixheaacd_over_lap_add1.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(
                        &mut *scratch.offset(0 as core::ffi::c_int as isize),
                        &mut *ptr_overlap_buf.offset(size_07 as isize),
                        (out_samples as *mut WORD32)
                            .offset(
                                (ch_fac as core::ffi::c_int * size_07 as core::ffi::c_int)
                                    as isize,
                            ),
                        ptr_short_window,
                        q_shift_0,
                        size_01,
                        ch_fac,
                    );
                } else {
                    ixheaacd_over_lap_add1_dec(
                        &mut *ptr_spec_coeff.offset(0 as core::ffi::c_int as isize),
                        &mut *ptr_overlap_buf.offset(size_07 as isize),
                        (out_samples as *mut WORD32)
                            .offset(
                                (ch_fac as core::ffi::c_int * size_07 as core::ffi::c_int)
                                    as isize,
                            ),
                        ptr_short_window,
                        q_shift_0,
                        size_01,
                        ch_fac,
                    );
                }
                i_0 = 0 as core::ffi::c_int as WORD32;
                while i_0 < 3 as core::ffi::c_int {
                    let mut inc: WORD32 = i_0 * size_02 as WORD32;
                    if 1024 as core::ffi::c_int
                        == (*ptr_ics_info).frame_length as core::ffi::c_int
                    {
                        (Some(
                            ixheaacd_spec_to_overlapbuf
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            overlap_buf_loc.as_mut_ptr(),
                            &mut *scratch.offset(inc as isize),
                            q_shift_0 as WORD32,
                            size_01 as WORD32,
                        );
                        (Some(
                            ixheaacd_over_lap_add1.expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            &mut *scratch.offset((size_02 as WORD32 + inc) as isize),
                            overlap_buf_loc.as_mut_ptr(),
                            (out_samples as *mut WORD32)
                                .offset(
                                    (ch_fac as WORD32 * (size_09 as WORD32 + inc)) as isize,
                                ),
                            short_window,
                            q_shift_0,
                            size_01,
                            ch_fac,
                        );
                    } else {
                        (Some(
                            ixheaacd_spec_to_overlapbuf
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            overlap_buf_loc.as_mut_ptr(),
                            &mut *ptr_spec_coeff.offset(inc as isize),
                            q_shift_0 as WORD32,
                            size_01 as WORD32,
                        );
                        ixheaacd_over_lap_add1_dec(
                            &mut *ptr_spec_coeff
                                .offset((size_02 as WORD32 + inc) as isize),
                            overlap_buf_loc.as_mut_ptr(),
                            (out_samples as *mut WORD32)
                                .offset(
                                    (ch_fac as WORD32 * (size_09 as WORD32 + inc)) as isize,
                                ),
                            short_window,
                            q_shift_0,
                            size_01,
                            ch_fac,
                        );
                    }
                    i_0 += 1;
                }
                if 1024 as core::ffi::c_int
                    == (*ptr_ics_info).frame_length as core::ffi::c_int
                {
                    (Some(ixheaacd_over_lap_add2.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(
                        &mut *scratch.offset(size_08 as isize),
                        &mut *scratch.offset(size_06 as isize),
                        ptr_overlap_buf,
                        short_window,
                        q_shift_0,
                        size_01,
                        1 as WORD16,
                    );
                } else {
                    ixheaacd_over_lap_add2_dec(
                        &mut *ptr_spec_coeff.offset(size_08 as isize),
                        &mut *ptr_spec_coeff.offset(size_06 as isize),
                        ptr_overlap_buf,
                        short_window,
                        q_shift_0,
                        size_01,
                        1 as WORD16,
                    );
                }
                (Some(ixheaacd_overlap_out_copy.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    (out_samples as *mut WORD32)
                        .offset(
                            (ch_fac as core::ffi::c_int * size_15 as core::ffi::c_int)
                                as isize,
                        ),
                    ptr_overlap_buf,
                    &mut *ptr_overlap_buf.offset(size_01 as isize),
                    ch_fac,
                    size_01,
                );
                (*ptr_ics_info).qshift_adj = 2 as WORD8;
            }
            ONLY_LONG_SEQUENCE | LONG_STOP_SEQUENCE => {
                if 1024 as core::ffi::c_int
                    == (*ptr_ics_info).frame_length as core::ffi::c_int
                {
                    ixheaacd_long_short_win_seq(
                        scratch,
                        ptr_overlap_buf,
                        out_samples as *mut WORD32,
                        short_window,
                        ptr_short_window,
                        ptr_long_window,
                        q_shift_0,
                        ch_fac,
                        size_01,
                    );
                } else {
                    ixheaacd_long_short_win_seq(
                        ptr_spec_coeff,
                        ptr_overlap_buf,
                        out_samples as *mut WORD32,
                        short_window,
                        ptr_short_window,
                        ptr_long_window,
                        q_shift_0,
                        ch_fac,
                        size_01,
                    );
                }
                (*ptr_ics_info).qshift_adj = 2 as WORD8;
            }
            _ => {}
        }
        i_0 = 0 as core::ffi::c_int as WORD32;
        while i_0 < 3 as core::ffi::c_int {
            let mut inc_0: WORD32 = i_0 * size_02 as WORD32;
            if 1024 as core::ffi::c_int
                == (*ptr_ics_info).frame_length as core::ffi::c_int
            {
                (Some(ixheaacd_over_lap_add2.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    &mut *scratch.offset((size_10 as WORD32 + inc_0) as isize),
                    &mut *scratch.offset((size_08 as WORD32 + inc_0) as isize),
                    &mut *ptr_overlap_buf.offset((size_01 as WORD32 + inc_0) as isize),
                    short_window,
                    q_shift_0,
                    size_01,
                    1 as WORD16,
                );
            } else {
                ixheaacd_over_lap_add2_dec(
                    &mut *ptr_spec_coeff.offset((size_10 as WORD32 + inc_0) as isize),
                    &mut *ptr_spec_coeff.offset((size_08 as WORD32 + inc_0) as isize),
                    &mut *ptr_overlap_buf.offset((size_01 as WORD32 + inc_0) as isize),
                    short_window,
                    q_shift_0,
                    size_01,
                    1 as WORD16,
                );
            }
            i_0 += 1;
        }
        if 1024 as core::ffi::c_int == (*ptr_ics_info).frame_length as core::ffi::c_int {
            (Some(ixheaacd_spec_to_overlapbuf.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                &mut *ptr_overlap_buf.offset(size_07 as isize),
                &mut *scratch.offset(size_14 as isize),
                q_shift_0 as WORD32,
                size_01 as WORD32,
            );
        } else {
            (Some(ixheaacd_spec_to_overlapbuf.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                &mut *ptr_overlap_buf.offset(size_07 as isize),
                &mut *ptr_spec_coeff.offset(size_14 as isize),
                q_shift_0 as WORD32,
                size_01 as WORD32,
            );
        }
    }
    (*ptr_aac_dec_overlap_info).window_shape = (*ptr_ics_info).window_shape;
    (*ptr_aac_dec_overlap_info).window_sequence = (*ptr_ics_info).window_sequence;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_eld_dec_windowing(
    mut ptr_spect_coeff: *mut WORD32,
    mut p_win: *const WORD16,
    mut framesize: WORD32,
    mut q_shift: WORD16,
    mut p_overlap_buffer: *mut WORD32,
    stride: WORD16,
    mut out_samples_t: *mut core::ffi::c_void,
    mut slot_element: WORD,
) -> VOID {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut loop_size: core::ffi::c_int = 0;
    let mut ptr_z: *mut WORD32 = ptr_spect_coeff;
    let mut ptr_out: *mut WORD32 = 0 as *mut WORD32;
    let mut p_out2: *mut WORD32 = 0 as *mut WORD32;
    let mut p_overlap_buffer32: *mut WORD32 = p_overlap_buffer;
    let mut delay: WORD32 = framesize >> 2 as core::ffi::c_int;
    let mut out_samples: *mut WORD16 = (out_samples_t as *mut WORD16)
        .offset(-(slot_element as isize));
    ptr_z = ptr_spect_coeff.offset(delay as isize);
    p_win = p_win.offset(delay as isize);
    ptr_out = p_overlap_buffer32;
    q_shift = (q_shift as core::ffi::c_int + 2 as core::ffi::c_int) as WORD16;
    if q_shift as core::ffi::c_int >= 0 as core::ffi::c_int {
        i = delay as core::ffi::c_int - 1 as core::ffi::c_int;
        while i >= 0 as core::ffi::c_int {
            let mut win_op: WORD32 = 0;
            let mut win_ovadd_op: WORD32 = 0;
            let mut win_val: WORD16 = 0;
            let fresh10 = p_win;
            p_win = p_win.offset(1);
            win_val = *fresh10;
            let fresh11 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op = ixheaac_mult32x16in32(*fresh11, win_val);
            let fresh12 = ptr_out;
            ptr_out = ptr_out.offset(1);
            win_ovadd_op = ixheaac_add32_sat(
                ixheaac_shl32(win_op, q_shift as WORD),
                *fresh12,
            );
            *out_samples = ixheaac_round16(ixheaac_shl32_sat(win_ovadd_op, 1 as WORD));
            out_samples = out_samples.offset(stride as core::ffi::c_int as isize);
            let fresh13 = p_win;
            p_win = p_win.offset(1);
            win_val = *fresh13;
            let fresh14 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op = ixheaac_mult32x16in32(*fresh14, win_val);
            let fresh15 = ptr_out;
            ptr_out = ptr_out.offset(1);
            win_ovadd_op = ixheaac_add32_sat(
                ixheaac_shl32(win_op, q_shift as WORD),
                *fresh15,
            );
            *out_samples = ixheaac_round16(ixheaac_shl32_sat(win_ovadd_op, 1 as WORD));
            out_samples = out_samples.offset(stride as core::ffi::c_int as isize);
            let fresh16 = p_win;
            p_win = p_win.offset(1);
            win_val = *fresh16;
            let fresh17 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op = ixheaac_mult32x16in32(*fresh17, win_val);
            let fresh18 = ptr_out;
            ptr_out = ptr_out.offset(1);
            win_ovadd_op = ixheaac_add32_sat(
                ixheaac_shl32(win_op, q_shift as WORD),
                *fresh18,
            );
            *out_samples = ixheaac_round16(ixheaac_shl32_sat(win_ovadd_op, 1 as WORD));
            out_samples = out_samples.offset(stride as core::ffi::c_int as isize);
            let fresh19 = p_win;
            p_win = p_win.offset(1);
            win_val = *fresh19;
            let fresh20 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op = ixheaac_mult32x16in32(*fresh20, win_val);
            let fresh21 = ptr_out;
            ptr_out = ptr_out.offset(1);
            win_ovadd_op = ixheaac_add32_sat(
                ixheaac_shl32(win_op, q_shift as WORD),
                *fresh21,
            );
            *out_samples = ixheaac_round16(ixheaac_shl32_sat(win_ovadd_op, 1 as WORD));
            out_samples = out_samples.offset(stride as core::ffi::c_int as isize);
            i -= 1;
        }
        p_out2 = p_overlap_buffer32;
        loop_size = (framesize as core::ffi::c_int * 3 as core::ffi::c_int
            - framesize as core::ffi::c_int >> 2 as core::ffi::c_int)
            - 1 as core::ffi::c_int;
        i = loop_size;
        while i >= 0 as core::ffi::c_int {
            let mut win_op_0: WORD32 = 0;
            let mut win_val_0: WORD16 = 0;
            let fresh22 = p_win;
            p_win = p_win.offset(1);
            win_val_0 = *fresh22;
            let fresh23 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_0 = ixheaac_mult32x16in32(*fresh23, win_val_0);
            let fresh24 = ptr_out;
            ptr_out = ptr_out.offset(1);
            let fresh25 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh25 = ixheaac_add32_sat(
                ixheaac_shl32(win_op_0, q_shift as WORD),
                *fresh24,
            );
            let fresh26 = p_win;
            p_win = p_win.offset(1);
            win_val_0 = *fresh26;
            let fresh27 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_0 = ixheaac_mult32x16in32(*fresh27, win_val_0);
            let fresh28 = ptr_out;
            ptr_out = ptr_out.offset(1);
            let fresh29 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh29 = ixheaac_add32_sat(
                ixheaac_shl32(win_op_0, q_shift as WORD),
                *fresh28,
            );
            let fresh30 = p_win;
            p_win = p_win.offset(1);
            win_val_0 = *fresh30;
            let fresh31 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_0 = ixheaac_mult32x16in32(*fresh31, win_val_0);
            let fresh32 = ptr_out;
            ptr_out = ptr_out.offset(1);
            let fresh33 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh33 = ixheaac_add32_sat(
                ixheaac_shl32(win_op_0, q_shift as WORD),
                *fresh32,
            );
            let fresh34 = p_win;
            p_win = p_win.offset(1);
            win_val_0 = *fresh34;
            let fresh35 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_0 = ixheaac_mult32x16in32(*fresh35, win_val_0);
            let fresh36 = ptr_out;
            ptr_out = ptr_out.offset(1);
            let fresh37 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh37 = ixheaac_add32_sat(
                ixheaac_shl32(win_op_0, q_shift as WORD),
                *fresh36,
            );
            i -= 1;
        }
        loop_size = (((framesize as core::ffi::c_int) << 2 as core::ffi::c_int)
            - delay as core::ffi::c_int
            - framesize as core::ffi::c_int * 3 as core::ffi::c_int
            >> 2 as core::ffi::c_int) - 1 as core::ffi::c_int;
        i = loop_size;
        while i >= 0 as core::ffi::c_int {
            let mut win_op_1: WORD32 = 0;
            let mut win_val_1: WORD16 = 0;
            let fresh38 = p_win;
            p_win = p_win.offset(1);
            win_val_1 = *fresh38;
            let fresh39 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_1 = ixheaac_mult32x16in32(*fresh39, win_val_1);
            let fresh40 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh40 = ixheaac_shl32(win_op_1, q_shift as WORD);
            let fresh41 = p_win;
            p_win = p_win.offset(1);
            win_val_1 = *fresh41;
            let fresh42 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_1 = ixheaac_mult32x16in32(*fresh42, win_val_1);
            let fresh43 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh43 = ixheaac_shl32(win_op_1, q_shift as WORD);
            let fresh44 = p_win;
            p_win = p_win.offset(1);
            win_val_1 = *fresh44;
            let fresh45 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_1 = ixheaac_mult32x16in32(*fresh45, win_val_1);
            let fresh46 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh46 = ixheaac_shl32(win_op_1, q_shift as WORD);
            let fresh47 = p_win;
            p_win = p_win.offset(1);
            win_val_1 = *fresh47;
            let fresh48 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_1 = ixheaac_mult32x16in32(*fresh48, win_val_1);
            let fresh49 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh49 = ixheaac_shl32(win_op_1, q_shift as WORD);
            i -= 1;
        }
    } else {
        q_shift = -(q_shift as core::ffi::c_int) as WORD16;
        i = delay as core::ffi::c_int - 1 as core::ffi::c_int;
        while i >= 0 as core::ffi::c_int {
            let mut win_op_2: WORD32 = 0;
            let mut win_ovadd_op_0: WORD32 = 0;
            let mut win_val_2: WORD16 = 0;
            let fresh50 = p_win;
            p_win = p_win.offset(1);
            win_val_2 = *fresh50;
            let fresh51 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_2 = ixheaac_mult32x16in32(*fresh51, win_val_2);
            let fresh52 = ptr_out;
            ptr_out = ptr_out.offset(1);
            win_ovadd_op_0 = ixheaac_add32_sat(
                ixheaac_shr32(win_op_2, q_shift as WORD),
                *fresh52,
            );
            *out_samples = ixheaac_round16(ixheaac_shl32(win_ovadd_op_0, 1 as WORD));
            out_samples = out_samples.offset(stride as core::ffi::c_int as isize);
            let fresh53 = p_win;
            p_win = p_win.offset(1);
            win_val_2 = *fresh53;
            let fresh54 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_2 = ixheaac_mult32x16in32(*fresh54, win_val_2);
            let fresh55 = ptr_out;
            ptr_out = ptr_out.offset(1);
            win_ovadd_op_0 = ixheaac_add32_sat(
                ixheaac_shr32(win_op_2, q_shift as WORD),
                *fresh55,
            );
            *out_samples = ixheaac_round16(ixheaac_shl32(win_ovadd_op_0, 1 as WORD));
            out_samples = out_samples.offset(stride as core::ffi::c_int as isize);
            let fresh56 = p_win;
            p_win = p_win.offset(1);
            win_val_2 = *fresh56;
            let fresh57 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_2 = ixheaac_mult32x16in32(*fresh57, win_val_2);
            let fresh58 = ptr_out;
            ptr_out = ptr_out.offset(1);
            win_ovadd_op_0 = ixheaac_add32_sat(
                ixheaac_shr32(win_op_2, q_shift as WORD),
                *fresh58,
            );
            *out_samples = ixheaac_round16(ixheaac_shl32(win_ovadd_op_0, 1 as WORD));
            out_samples = out_samples.offset(stride as core::ffi::c_int as isize);
            let fresh59 = p_win;
            p_win = p_win.offset(1);
            win_val_2 = *fresh59;
            let fresh60 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_2 = ixheaac_mult32x16in32(*fresh60, win_val_2);
            let fresh61 = ptr_out;
            ptr_out = ptr_out.offset(1);
            win_ovadd_op_0 = ixheaac_add32_sat(
                ixheaac_shr32(win_op_2, q_shift as WORD),
                *fresh61,
            );
            *out_samples = ixheaac_round16(ixheaac_shl32(win_ovadd_op_0, 1 as WORD));
            out_samples = out_samples.offset(stride as core::ffi::c_int as isize);
            i -= 1;
        }
        p_out2 = p_overlap_buffer32;
        loop_size = (framesize as core::ffi::c_int * 3 as core::ffi::c_int
            - framesize as core::ffi::c_int >> 2 as core::ffi::c_int)
            - 1 as core::ffi::c_int;
        i = loop_size;
        while i >= 0 as core::ffi::c_int {
            let mut win_op_3: WORD32 = 0;
            let mut win_val_3: WORD16 = 0;
            let fresh62 = p_win;
            p_win = p_win.offset(1);
            win_val_3 = *fresh62;
            let fresh63 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_3 = ixheaac_mult32x16in32(*fresh63, win_val_3);
            let fresh64 = ptr_out;
            ptr_out = ptr_out.offset(1);
            let fresh65 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh65 = ixheaac_add32_sat(
                ixheaac_shr32(win_op_3, q_shift as WORD),
                *fresh64,
            );
            let fresh66 = p_win;
            p_win = p_win.offset(1);
            win_val_3 = *fresh66;
            let fresh67 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_3 = ixheaac_mult32x16in32(*fresh67, win_val_3);
            let fresh68 = ptr_out;
            ptr_out = ptr_out.offset(1);
            let fresh69 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh69 = ixheaac_add32_sat(
                ixheaac_shr32(win_op_3, q_shift as WORD),
                *fresh68,
            );
            let fresh70 = p_win;
            p_win = p_win.offset(1);
            win_val_3 = *fresh70;
            let fresh71 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_3 = ixheaac_mult32x16in32(*fresh71, win_val_3);
            let fresh72 = ptr_out;
            ptr_out = ptr_out.offset(1);
            let fresh73 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh73 = ixheaac_add32_sat(
                ixheaac_shr32(win_op_3, q_shift as WORD),
                *fresh72,
            );
            let fresh74 = p_win;
            p_win = p_win.offset(1);
            win_val_3 = *fresh74;
            let fresh75 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_3 = ixheaac_mult32x16in32(*fresh75, win_val_3);
            let fresh76 = ptr_out;
            ptr_out = ptr_out.offset(1);
            let fresh77 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh77 = ixheaac_add32_sat(
                ixheaac_shr32(win_op_3, q_shift as WORD),
                *fresh76,
            );
            i -= 1;
        }
        loop_size = (((framesize as core::ffi::c_int) << 2 as core::ffi::c_int)
            - delay as core::ffi::c_int
            - framesize as core::ffi::c_int * 3 as core::ffi::c_int
            >> 2 as core::ffi::c_int) - 1 as core::ffi::c_int;
        i = loop_size;
        while i >= 0 as core::ffi::c_int {
            let mut win_op_4: WORD32 = 0;
            let mut win_val_4: WORD16 = 0;
            let fresh78 = p_win;
            p_win = p_win.offset(1);
            win_val_4 = *fresh78;
            let fresh79 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_4 = ixheaac_mult32x16in32(*fresh79, win_val_4);
            let fresh80 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh80 = ixheaac_shr32(win_op_4, q_shift as WORD);
            let fresh81 = p_win;
            p_win = p_win.offset(1);
            win_val_4 = *fresh81;
            let fresh82 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_4 = ixheaac_mult32x16in32(*fresh82, win_val_4);
            let fresh83 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh83 = ixheaac_shr32(win_op_4, q_shift as WORD);
            let fresh84 = p_win;
            p_win = p_win.offset(1);
            win_val_4 = *fresh84;
            let fresh85 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_4 = ixheaac_mult32x16in32(*fresh85, win_val_4);
            let fresh86 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh86 = ixheaac_shr32(win_op_4, q_shift as WORD);
            let fresh87 = p_win;
            p_win = p_win.offset(1);
            win_val_4 = *fresh87;
            let fresh88 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_4 = ixheaac_mult32x16in32(*fresh88, win_val_4);
            let fresh89 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh89 = ixheaac_shr32(win_op_4, q_shift as WORD);
            i -= 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_eld_dec_windowing_32bit(
    mut ptr_spect_coeff: *mut WORD32,
    mut p_win: *const WORD16,
    mut framesize: WORD32,
    mut q_shift: WORD16,
    mut p_overlap_buffer: *mut WORD32,
    stride: WORD16,
    mut out_samples: *mut WORD32,
) -> VOID {
    let mut i: WORD32 = 0 as WORD32;
    let mut loop_size: WORD32 = 0;
    let mut ptr_z: *mut WORD32 = ptr_spect_coeff;
    let mut ptr_out: *mut WORD32 = 0 as *mut WORD32;
    let mut p_out2: *mut WORD32 = 0 as *mut WORD32;
    let mut p_overlap_buffer32: *mut WORD32 = p_overlap_buffer;
    let mut delay: WORD32 = framesize >> 2 as core::ffi::c_int;
    ptr_z = ptr_spect_coeff.offset(delay as isize);
    p_win = p_win.offset(delay as isize);
    ptr_out = p_overlap_buffer32;
    q_shift = (q_shift as core::ffi::c_int + 2 as core::ffi::c_int) as WORD16;
    if q_shift as core::ffi::c_int >= 0 as core::ffi::c_int {
        i = (delay as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        while i >= 0 as core::ffi::c_int {
            let mut win_op: WORD32 = 0;
            let mut win_val: WORD16 = 0;
            let fresh90 = p_win;
            p_win = p_win.offset(1);
            win_val = *fresh90;
            let fresh91 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op = ixheaac_mult32x16in32(*fresh91, win_val);
            let fresh92 = ptr_out;
            ptr_out = ptr_out.offset(1);
            *out_samples = ixheaac_add32_sat(
                ixheaac_shl32(win_op, q_shift as WORD),
                *fresh92,
            );
            out_samples = out_samples.offset(stride as core::ffi::c_int as isize);
            let fresh93 = p_win;
            p_win = p_win.offset(1);
            win_val = *fresh93;
            let fresh94 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op = ixheaac_mult32x16in32(*fresh94, win_val);
            let fresh95 = ptr_out;
            ptr_out = ptr_out.offset(1);
            *out_samples = ixheaac_add32_sat(
                ixheaac_shl32(win_op, q_shift as WORD),
                *fresh95,
            );
            out_samples = out_samples.offset(stride as core::ffi::c_int as isize);
            let fresh96 = p_win;
            p_win = p_win.offset(1);
            win_val = *fresh96;
            let fresh97 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op = ixheaac_mult32x16in32(*fresh97, win_val);
            let fresh98 = ptr_out;
            ptr_out = ptr_out.offset(1);
            *out_samples = ixheaac_add32_sat(
                ixheaac_shl32(win_op, q_shift as WORD),
                *fresh98,
            );
            out_samples = out_samples.offset(stride as core::ffi::c_int as isize);
            let fresh99 = p_win;
            p_win = p_win.offset(1);
            win_val = *fresh99;
            let fresh100 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op = ixheaac_mult32x16in32(*fresh100, win_val);
            let fresh101 = ptr_out;
            ptr_out = ptr_out.offset(1);
            *out_samples = ixheaac_add32_sat(
                ixheaac_shl32(win_op, q_shift as WORD),
                *fresh101,
            );
            out_samples = out_samples.offset(stride as core::ffi::c_int as isize);
            i -= 1;
        }
        p_out2 = p_overlap_buffer32;
        loop_size = ((framesize as core::ffi::c_int * 3 as core::ffi::c_int
            - framesize as core::ffi::c_int >> 2 as core::ffi::c_int)
            - 1 as core::ffi::c_int) as WORD32;
        i = loop_size;
        while i >= 0 as core::ffi::c_int {
            let mut win_op_0: WORD32 = 0;
            let mut win_val_0: WORD16 = 0;
            let fresh102 = p_win;
            p_win = p_win.offset(1);
            win_val_0 = *fresh102;
            let fresh103 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_0 = ixheaac_mult32x16in32(*fresh103, win_val_0);
            let fresh104 = ptr_out;
            ptr_out = ptr_out.offset(1);
            let fresh105 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh105 = ixheaac_add32_sat(
                ixheaac_shl32(win_op_0, q_shift as WORD),
                *fresh104,
            );
            let fresh106 = p_win;
            p_win = p_win.offset(1);
            win_val_0 = *fresh106;
            let fresh107 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_0 = ixheaac_mult32x16in32(*fresh107, win_val_0);
            let fresh108 = ptr_out;
            ptr_out = ptr_out.offset(1);
            let fresh109 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh109 = ixheaac_add32_sat(
                ixheaac_shl32(win_op_0, q_shift as WORD),
                *fresh108,
            );
            let fresh110 = p_win;
            p_win = p_win.offset(1);
            win_val_0 = *fresh110;
            let fresh111 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_0 = ixheaac_mult32x16in32(*fresh111, win_val_0);
            let fresh112 = ptr_out;
            ptr_out = ptr_out.offset(1);
            let fresh113 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh113 = ixheaac_add32_sat(
                ixheaac_shl32(win_op_0, q_shift as WORD),
                *fresh112,
            );
            let fresh114 = p_win;
            p_win = p_win.offset(1);
            win_val_0 = *fresh114;
            let fresh115 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_0 = ixheaac_mult32x16in32(*fresh115, win_val_0);
            let fresh116 = ptr_out;
            ptr_out = ptr_out.offset(1);
            let fresh117 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh117 = ixheaac_add32_sat(
                ixheaac_shl32(win_op_0, q_shift as WORD),
                *fresh116,
            );
            i -= 1;
        }
        loop_size = ((((framesize as core::ffi::c_int) << 2 as core::ffi::c_int)
            - delay as core::ffi::c_int
            - framesize as core::ffi::c_int * 3 as core::ffi::c_int
            >> 2 as core::ffi::c_int) - 1 as core::ffi::c_int) as WORD32;
        i = loop_size;
        while i >= 0 as core::ffi::c_int {
            let mut win_op_1: WORD32 = 0;
            let mut win_val_1: WORD16 = 0;
            let fresh118 = p_win;
            p_win = p_win.offset(1);
            win_val_1 = *fresh118;
            let fresh119 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_1 = ixheaac_mult32x16in32(*fresh119, win_val_1);
            let fresh120 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh120 = ixheaac_shl32(win_op_1, q_shift as WORD);
            let fresh121 = p_win;
            p_win = p_win.offset(1);
            win_val_1 = *fresh121;
            let fresh122 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_1 = ixheaac_mult32x16in32(*fresh122, win_val_1);
            let fresh123 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh123 = ixheaac_shl32(win_op_1, q_shift as WORD);
            let fresh124 = p_win;
            p_win = p_win.offset(1);
            win_val_1 = *fresh124;
            let fresh125 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_1 = ixheaac_mult32x16in32(*fresh125, win_val_1);
            let fresh126 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh126 = ixheaac_shl32(win_op_1, q_shift as WORD);
            let fresh127 = p_win;
            p_win = p_win.offset(1);
            win_val_1 = *fresh127;
            let fresh128 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_1 = ixheaac_mult32x16in32(*fresh128, win_val_1);
            let fresh129 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh129 = ixheaac_shl32(win_op_1, q_shift as WORD);
            i -= 1;
        }
    } else {
        q_shift = -(q_shift as core::ffi::c_int) as WORD16;
        i = (delay as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        while i >= 0 as core::ffi::c_int {
            let mut win_op_2: WORD32 = 0;
            let mut win_val_2: WORD16 = 0;
            let fresh130 = p_win;
            p_win = p_win.offset(1);
            win_val_2 = *fresh130;
            let fresh131 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_2 = ixheaac_mult32x16in32(*fresh131, win_val_2);
            let fresh132 = ptr_out;
            ptr_out = ptr_out.offset(1);
            *out_samples = ixheaac_add32_sat(
                ixheaac_shr32(win_op_2, q_shift as WORD),
                *fresh132,
            );
            out_samples = out_samples.offset(stride as core::ffi::c_int as isize);
            let fresh133 = p_win;
            p_win = p_win.offset(1);
            win_val_2 = *fresh133;
            let fresh134 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_2 = ixheaac_mult32x16in32(*fresh134, win_val_2);
            let fresh135 = ptr_out;
            ptr_out = ptr_out.offset(1);
            *out_samples = ixheaac_add32_sat(
                ixheaac_shr32(win_op_2, q_shift as WORD),
                *fresh135,
            );
            out_samples = out_samples.offset(stride as core::ffi::c_int as isize);
            let fresh136 = p_win;
            p_win = p_win.offset(1);
            win_val_2 = *fresh136;
            let fresh137 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_2 = ixheaac_mult32x16in32(*fresh137, win_val_2);
            let fresh138 = ptr_out;
            ptr_out = ptr_out.offset(1);
            *out_samples = ixheaac_add32_sat(
                ixheaac_shr32(win_op_2, q_shift as WORD),
                *fresh138,
            );
            out_samples = out_samples.offset(stride as core::ffi::c_int as isize);
            let fresh139 = p_win;
            p_win = p_win.offset(1);
            win_val_2 = *fresh139;
            let fresh140 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_2 = ixheaac_mult32x16in32(*fresh140, win_val_2);
            let fresh141 = ptr_out;
            ptr_out = ptr_out.offset(1);
            *out_samples = ixheaac_add32_sat(
                ixheaac_shr32(win_op_2, q_shift as WORD),
                *fresh141,
            );
            out_samples = out_samples.offset(stride as core::ffi::c_int as isize);
            i -= 1;
        }
        p_out2 = p_overlap_buffer32;
        loop_size = ((framesize as core::ffi::c_int * 3 as core::ffi::c_int
            - framesize as core::ffi::c_int >> 2 as core::ffi::c_int)
            - 1 as core::ffi::c_int) as WORD32;
        i = loop_size;
        while i >= 0 as core::ffi::c_int {
            let mut win_op_3: WORD32 = 0;
            let mut win_val_3: WORD16 = 0;
            let fresh142 = p_win;
            p_win = p_win.offset(1);
            win_val_3 = *fresh142;
            let fresh143 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_3 = ixheaac_mult32x16in32(*fresh143, win_val_3);
            let fresh144 = ptr_out;
            ptr_out = ptr_out.offset(1);
            let fresh145 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh145 = ixheaac_add32_sat(
                ixheaac_shr32(win_op_3, q_shift as WORD),
                *fresh144,
            );
            let fresh146 = p_win;
            p_win = p_win.offset(1);
            win_val_3 = *fresh146;
            let fresh147 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_3 = ixheaac_mult32x16in32(*fresh147, win_val_3);
            let fresh148 = ptr_out;
            ptr_out = ptr_out.offset(1);
            let fresh149 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh149 = ixheaac_add32_sat(
                ixheaac_shr32(win_op_3, q_shift as WORD),
                *fresh148,
            );
            let fresh150 = p_win;
            p_win = p_win.offset(1);
            win_val_3 = *fresh150;
            let fresh151 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_3 = ixheaac_mult32x16in32(*fresh151, win_val_3);
            let fresh152 = ptr_out;
            ptr_out = ptr_out.offset(1);
            let fresh153 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh153 = ixheaac_add32_sat(
                ixheaac_shr32(win_op_3, q_shift as WORD),
                *fresh152,
            );
            let fresh154 = p_win;
            p_win = p_win.offset(1);
            win_val_3 = *fresh154;
            let fresh155 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_3 = ixheaac_mult32x16in32(*fresh155, win_val_3);
            let fresh156 = ptr_out;
            ptr_out = ptr_out.offset(1);
            let fresh157 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh157 = ixheaac_add32_sat(
                ixheaac_shr32(win_op_3, q_shift as WORD),
                *fresh156,
            );
            i -= 1;
        }
        loop_size = ((((framesize as core::ffi::c_int) << 2 as core::ffi::c_int)
            - delay as core::ffi::c_int
            - framesize as core::ffi::c_int * 3 as core::ffi::c_int
            >> 2 as core::ffi::c_int) - 1 as core::ffi::c_int) as WORD32;
        i = loop_size;
        while i >= 0 as core::ffi::c_int {
            let mut win_op_4: WORD32 = 0;
            let mut win_val_4: WORD16 = 0;
            let fresh158 = p_win;
            p_win = p_win.offset(1);
            win_val_4 = *fresh158;
            let fresh159 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_4 = ixheaac_mult32x16in32(*fresh159, win_val_4);
            let fresh160 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh160 = ixheaac_shr32(win_op_4, q_shift as WORD);
            let fresh161 = p_win;
            p_win = p_win.offset(1);
            win_val_4 = *fresh161;
            let fresh162 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_4 = ixheaac_mult32x16in32(*fresh162, win_val_4);
            let fresh163 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh163 = ixheaac_shr32(win_op_4, q_shift as WORD);
            let fresh164 = p_win;
            p_win = p_win.offset(1);
            win_val_4 = *fresh164;
            let fresh165 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_4 = ixheaac_mult32x16in32(*fresh165, win_val_4);
            let fresh166 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh166 = ixheaac_shr32(win_op_4, q_shift as WORD);
            let fresh167 = p_win;
            p_win = p_win.offset(1);
            win_val_4 = *fresh167;
            let fresh168 = ptr_z;
            ptr_z = ptr_z.offset(1);
            win_op_4 = ixheaac_mult32x16in32(*fresh168, win_val_4);
            let fresh169 = p_out2;
            p_out2 = p_out2.offset(1);
            *fresh169 = ixheaac_shr32(win_op_4, q_shift as WORD);
            i -= 1;
        }
    };
}
