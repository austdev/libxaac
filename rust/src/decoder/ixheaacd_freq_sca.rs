extern "C" {
    fn pow(__x: core::ffi::c_double, __y: core::ffi::c_double) -> core::ffi::c_double;
}
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD16 = core::ffi::c_short;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type WORD64 = core::ffi::c_longlong;
pub type FLOAT32 = core::ffi::c_float;
pub type VOID = ();
pub type FLAG = core::ffi::c_int;
pub type WORD = core::ffi::c_int;
pub type IA_ERRORCODE = WORD32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ixheaacd_misc_tables {
    pub trig_data: [WORD16; 513],
    pub sine_table8_16: [WORD16; 8],
    pub log_dual_is_table: [WORD16; 65],
    pub down_mix_martix: [[[WORD32; 8]; 2]; 4],
    pub cc_gain_scale: [WORD32; 4],
    pub inv_table: [WORD16; 256],
    pub sqrt_table: [WORD16; 257],
    pub dummy: WORD32,
    pub start_band: [[WORD32; 16]; 10],
    pub stop_band: [[WORD32; 16]; 10],
    pub stop_freq_table_fs40k_2: [WORD32; 14],
    pub stop_freq_table_fs40k_4: [WORD32; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_freq_band_data_struct {
    pub num_sf_bands: [WORD16; 2],
    pub num_nf_bands: WORD16,
    pub num_mf_bands: WORD16,
    pub sub_band_start: WORD16,
    pub sub_band_end: WORD16,
    pub freq_band_tbl_lim: [WORD16; 13],
    pub num_lf_bands: WORD16,
    pub num_if_bands: WORD16,
    pub freq_band_table: [*mut WORD16; 2],
    pub freq_band_tbl_lo: [WORD16; 29],
    pub freq_band_tbl_hi: [WORD16; 57],
    pub freq_band_tbl_noise: [WORD16; 6],
    pub f_master_tbl: [WORD16; 57],
    pub qmf_sb_prev: WORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_header_data_struct {
    pub sync_state: WORD32,
    pub err_flag: FLAG,
    pub err_flag_prev: FLAG,
    pub num_time_slots: WORD16,
    pub time_step: WORD16,
    pub core_frame_size: WORD16,
    pub out_sampling_freq: WORD32,
    pub channel_mode: WORD32,
    pub amp_res: WORD16,
    pub start_freq: WORD16,
    pub stop_freq: WORD16,
    pub xover_band: WORD16,
    pub freq_scale: WORD16,
    pub alter_scale: WORD16,
    pub noise_bands: WORD16,
    pub limiter_bands: WORD16,
    pub limiter_gains: WORD16,
    pub interpol_freq: WORD16,
    pub smoothing_mode: WORD16,
    pub pstr_freq_band_data: *mut ia_freq_band_data_struct,
    pub header_extra_1: WORD16,
    pub header_extra_2: WORD16,
    pub pre_proc_flag: WORD16,
    pub status: WORD32,
    pub sbr_ratio_idx: WORD32,
    pub upsamp_fac: WORD32,
    pub is_usf_4: WORD32,
    pub output_framesize: WORD32,
    pub usac_independency_flag: WORD32,
    pub pvc_flag: FLAG,
    pub hbe_flag: FLAG,
    pub esbr_start_up: WORD32,
    pub esbr_start_up_pvc: WORD32,
    pub usac_flag: WORD32,
    pub pvc_mode: UWORD8,
    pub enh_sbr: FLAG,
    pub esbr_hq: FLAG,
    pub enh_sbr_ps: FLAG,
    pub eld_sbr: FLAG,
}
pub const INT_BITS: core::ffi::c_int = 32 as core::ffi::c_int;
pub const SBR_UPSAMPLE_IDX_4_1: core::ffi::c_int = 3 as core::ffi::c_int;
pub const NO_SYNTHESIS_CHANNELS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const MAX_NOISE_COEFFS: core::ffi::c_int = 5 as core::ffi::c_int;
pub const MAX_FREQ_COEFFS: core::ffi::c_int = 56 as core::ffi::c_int;
pub const MAX_FREQ_COEFFS_FS44100: core::ffi::c_int = 35 as core::ffi::c_int;
pub const MAX_FREQ_COEFFS_FS48000: core::ffi::c_int = 32 as core::ffi::c_int;
pub const MAX_FREQ_COEFFS_SBR: core::ffi::c_int = 48 as core::ffi::c_int;
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
unsafe extern "C" fn ixheaac_mult16x16in32(mut a: WORD16, mut b: WORD16) -> WORD32 {
    let mut product: WORD32 = 0;
    product = a as WORD32 * b as WORD32;
    return product;
}
#[inline]
unsafe extern "C" fn ixheaac_mult16x16in32_shl(mut a: WORD16, mut b: WORD16) -> WORD32 {
    let mut product: WORD32 = 0;
    product = ixheaac_shl32(ixheaac_mult16x16in32(a, b), 1 as WORD);
    return product;
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
unsafe extern "C" fn ixheaac_shr16(mut op1: WORD16, mut shift: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = (op1 as core::ffi::c_int >> shift as core::ffi::c_int) as WORD16;
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_extract16h(mut var: WORD32) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = (var >> 16 as core::ffi::c_int) as WORD16;
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_extract16l(mut var: WORD32) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = var as WORD16;
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_deposit16l_in32(mut var: WORD16) -> WORD32 {
    let mut var_out: WORD32 = 0;
    var_out = var as WORD32;
    return var_out;
}
pub const IA_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const LOW: core::ffi::c_int = 0 as core::ffi::c_int;
pub const HIGH: core::ffi::c_int = 1 as core::ffi::c_int;
pub const MAX_OCTAVE: core::ffi::c_int = 29 as core::ffi::c_int;
#[no_mangle]
pub static mut ixheaacd_samp_rate_table: [WORD32; 12] = [
    92017 as core::ffi::c_int,
    75132 as core::ffi::c_int,
    55426 as core::ffi::c_int,
    46009 as core::ffi::c_int,
    37566 as core::ffi::c_int,
    27713 as core::ffi::c_int,
    23004 as core::ffi::c_int,
    18783 as core::ffi::c_int,
    13856 as core::ffi::c_int,
    11502 as core::ffi::c_int,
    9391 as core::ffi::c_int,
    16428320 as core::ffi::c_int,
];
#[no_mangle]
pub static mut ixheaacd_v_offset_40: [WORD32; 16] = [
    3 as core::ffi::c_int + 1 as core::ffi::c_int,
    2 as core::ffi::c_int + 1 as core::ffi::c_int,
    2 as core::ffi::c_int + 1 as core::ffi::c_int,
    2 as core::ffi::c_int + 1 as core::ffi::c_int,
    2 as core::ffi::c_int + 1 as core::ffi::c_int,
    2 as core::ffi::c_int + 1 as core::ffi::c_int,
    2 as core::ffi::c_int + 1 as core::ffi::c_int,
    2 as core::ffi::c_int + 1 as core::ffi::c_int,
    2 as core::ffi::c_int + 1 as core::ffi::c_int,
    2 as core::ffi::c_int + 1 as core::ffi::c_int,
    2 as core::ffi::c_int + 1 as core::ffi::c_int,
    2 as core::ffi::c_int + 1 as core::ffi::c_int,
    2 as core::ffi::c_int + 1 as core::ffi::c_int,
    2 as core::ffi::c_int + 1 as core::ffi::c_int,
    1 as core::ffi::c_int + 1 as core::ffi::c_int,
    0 as core::ffi::c_int,
];
unsafe extern "C" fn ixheaacd_int_div(mut num: WORD32, mut den: WORD32) -> WORD32 {
    if den != 0 as core::ffi::c_int {
        let mut result: WORD32 = 0 as WORD32;
        let mut temp: WORD32 = 0 as WORD32;
        while den <= num {
            temp = 0 as core::ffi::c_int as WORD32;
            while num >= den << temp as core::ffi::c_int + 1 as core::ffi::c_int {
                temp += 1;
            }
            result = (result as core::ffi::c_int + ((1 as core::ffi::c_int) << temp))
                as WORD32;
            num = (num as core::ffi::c_int
                - den as core::ffi::c_int * ((1 as core::ffi::c_int) << temp)) as WORD32;
        }
        return result;
    } else {
        return 0 as WORD32
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_aac_shellsort(
    mut in_0: *mut WORD16,
    mut n: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut inc: WORD32 = 0;
    let mut v: WORD32 = 0;
    let mut w: WORD32 = 0;
    inc = 1 as core::ffi::c_int as WORD32;
    loop {
        inc = (((inc as core::ffi::c_int) << 1 as core::ffi::c_int)
            + inc as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
        if !(inc <= n) {
            break;
        }
    }
    loop {
        inc = ixheaacd_int_div(inc, 3 as WORD32);
        i = inc;
        while i < n {
            v = *in_0.offset(i as isize) as WORD32;
            j = i;
            loop {
                w = *in_0.offset((j - inc) as isize) as WORD32;
                if !(w > v) {
                    break;
                }
                *in_0.offset(j as isize) = w as WORD16;
                j = j - inc;
                if j < inc {
                    break;
                }
            }
            *in_0.offset(j as isize) = v as WORD16;
            i += 1;
        }
        if !(inc > 1 as core::ffi::c_int) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_calc_start_band(
    mut fs_mapped: WORD32,
    start_freq: WORD32,
    mut upsamp_fac: FLOAT32,
) -> WORD32 {
    let mut k0_min: WORD32 = 0;
    if upsamp_fac == 4 as core::ffi::c_int as FLOAT32 {
        if fs_mapped < 32000 as core::ffi::c_int {
            k0_min = (((3000 as core::ffi::c_int * 2 as core::ffi::c_int
                * 32 as core::ffi::c_int) as FLOAT32 / fs_mapped as FLOAT32)
                as core::ffi::c_double + 0.5f64) as WORD32;
        } else if fs_mapped < 64000 as core::ffi::c_int {
            k0_min = (((4000 as core::ffi::c_int * 2 as core::ffi::c_int
                * 32 as core::ffi::c_int) as FLOAT32 / fs_mapped as FLOAT32)
                as core::ffi::c_double + 0.5f64) as WORD32;
        } else {
            k0_min = (((5000 as core::ffi::c_int * 2 as core::ffi::c_int
                * 32 as core::ffi::c_int) as FLOAT32 / fs_mapped as FLOAT32)
                as core::ffi::c_double + 0.5f64) as WORD32;
        }
    } else if fs_mapped < 32000 as core::ffi::c_int {
        k0_min = (((3000 as core::ffi::c_int * 2 as core::ffi::c_int
            * 64 as core::ffi::c_int) as FLOAT32 / fs_mapped as FLOAT32)
            as core::ffi::c_double + 0.5f64) as WORD32;
    } else if fs_mapped < 64000 as core::ffi::c_int {
        k0_min = (((4000 as core::ffi::c_int * 2 as core::ffi::c_int
            * 64 as core::ffi::c_int) as FLOAT32 / fs_mapped as FLOAT32)
            as core::ffi::c_double + 0.5f64) as WORD32;
    } else {
        k0_min = (((5000 as core::ffi::c_int * 2 as core::ffi::c_int
            * 64 as core::ffi::c_int) as FLOAT32 / fs_mapped as FLOAT32)
            as core::ffi::c_double + 0.5f64) as WORD32;
    }
    match fs_mapped {
        16000 => {
            let mut v_offset: [WORD32; 16] = [
                -(8 as core::ffi::c_int),
                -(7 as core::ffi::c_int),
                -(6 as core::ffi::c_int),
                -(5 as core::ffi::c_int),
                -(4 as core::ffi::c_int),
                -(3 as core::ffi::c_int),
                -(2 as core::ffi::c_int),
                -(1 as core::ffi::c_int),
                0 as core::ffi::c_int,
                1 as core::ffi::c_int,
                2 as core::ffi::c_int,
                3 as core::ffi::c_int,
                4 as core::ffi::c_int,
                5 as core::ffi::c_int,
                6 as core::ffi::c_int,
                7 as core::ffi::c_int,
            ];
            return k0_min + v_offset[start_freq as usize];
        }
        22050 => {
            let mut v_offset_0: [WORD32; 16] = [
                -(5 as core::ffi::c_int),
                -(4 as core::ffi::c_int),
                -(3 as core::ffi::c_int),
                -(2 as core::ffi::c_int),
                -(1 as core::ffi::c_int),
                0 as core::ffi::c_int,
                1 as core::ffi::c_int,
                2 as core::ffi::c_int,
                3 as core::ffi::c_int,
                4 as core::ffi::c_int,
                5 as core::ffi::c_int,
                6 as core::ffi::c_int,
                7 as core::ffi::c_int,
                9 as core::ffi::c_int,
                11 as core::ffi::c_int,
                13 as core::ffi::c_int,
            ];
            return k0_min + v_offset_0[start_freq as usize];
        }
        24000 => {
            let mut v_offset_1: [WORD32; 16] = [
                -(5 as core::ffi::c_int),
                -(3 as core::ffi::c_int),
                -(2 as core::ffi::c_int),
                -(1 as core::ffi::c_int),
                0 as core::ffi::c_int,
                1 as core::ffi::c_int,
                2 as core::ffi::c_int,
                3 as core::ffi::c_int,
                4 as core::ffi::c_int,
                5 as core::ffi::c_int,
                6 as core::ffi::c_int,
                7 as core::ffi::c_int,
                9 as core::ffi::c_int,
                11 as core::ffi::c_int,
                13 as core::ffi::c_int,
                16 as core::ffi::c_int,
            ];
            return k0_min + v_offset_1[start_freq as usize];
        }
        32000 => {
            let mut v_offset_2: [WORD32; 16] = [
                -(6 as core::ffi::c_int),
                -(4 as core::ffi::c_int),
                -(2 as core::ffi::c_int),
                -(1 as core::ffi::c_int),
                0 as core::ffi::c_int,
                1 as core::ffi::c_int,
                2 as core::ffi::c_int,
                3 as core::ffi::c_int,
                4 as core::ffi::c_int,
                5 as core::ffi::c_int,
                6 as core::ffi::c_int,
                7 as core::ffi::c_int,
                9 as core::ffi::c_int,
                11 as core::ffi::c_int,
                13 as core::ffi::c_int,
                16 as core::ffi::c_int,
            ];
            return k0_min + v_offset_2[start_freq as usize];
        }
        40000 => {
            let mut v_offset_3: [WORD32; 16] = [
                -(1 as core::ffi::c_int),
                0 as core::ffi::c_int,
                1 as core::ffi::c_int,
                2 as core::ffi::c_int,
                3 as core::ffi::c_int,
                4 as core::ffi::c_int,
                5 as core::ffi::c_int,
                6 as core::ffi::c_int,
                7 as core::ffi::c_int,
                8 as core::ffi::c_int,
                9 as core::ffi::c_int,
                11 as core::ffi::c_int,
                13 as core::ffi::c_int,
                15 as core::ffi::c_int,
                17 as core::ffi::c_int,
                19 as core::ffi::c_int,
            ];
            return k0_min + v_offset_3[start_freq as usize];
        }
        44100 | 48000 | 64000 => {
            let mut v_offset_4: [WORD32; 16] = [
                -(4 as core::ffi::c_int),
                -(2 as core::ffi::c_int),
                -(1 as core::ffi::c_int),
                0 as core::ffi::c_int,
                1 as core::ffi::c_int,
                2 as core::ffi::c_int,
                3 as core::ffi::c_int,
                4 as core::ffi::c_int,
                5 as core::ffi::c_int,
                6 as core::ffi::c_int,
                7 as core::ffi::c_int,
                9 as core::ffi::c_int,
                11 as core::ffi::c_int,
                13 as core::ffi::c_int,
                16 as core::ffi::c_int,
                20 as core::ffi::c_int,
            ];
            return k0_min + v_offset_4[start_freq as usize];
        }
        88200 | 96000 => {
            let mut v_offset_5: [WORD32; 16] = [
                -(2 as core::ffi::c_int),
                -(1 as core::ffi::c_int),
                0 as core::ffi::c_int,
                1 as core::ffi::c_int,
                2 as core::ffi::c_int,
                3 as core::ffi::c_int,
                4 as core::ffi::c_int,
                5 as core::ffi::c_int,
                6 as core::ffi::c_int,
                7 as core::ffi::c_int,
                9 as core::ffi::c_int,
                11 as core::ffi::c_int,
                13 as core::ffi::c_int,
                16 as core::ffi::c_int,
                20 as core::ffi::c_int,
                24 as core::ffi::c_int,
            ];
            return k0_min + v_offset_5[start_freq as usize];
        }
        _ => {
            let mut v_offset_6: [WORD32; 16] = [
                0 as core::ffi::c_int,
                1 as core::ffi::c_int,
                2 as core::ffi::c_int,
                3 as core::ffi::c_int,
                4 as core::ffi::c_int,
                5 as core::ffi::c_int,
                6 as core::ffi::c_int,
                7 as core::ffi::c_int,
                9 as core::ffi::c_int,
                11 as core::ffi::c_int,
                13 as core::ffi::c_int,
                16 as core::ffi::c_int,
                20 as core::ffi::c_int,
                24 as core::ffi::c_int,
                28 as core::ffi::c_int,
                33 as core::ffi::c_int,
            ];
            return k0_min + v_offset_6[start_freq as usize];
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_calc_stop_band(
    mut fs: WORD32,
    stop_freq: WORD32,
    mut upsamp_fac: FLOAT32,
) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut arr_stop_freq: [WORD16; 14] = [0; 14];
    let mut k1_min: WORD32 = 0;
    let mut arr_diff_stop_freq: [WORD16; 13] = [0; 13];
    if upsamp_fac == 4 as core::ffi::c_int as FLOAT32 {
        fs = (fs as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
        if fs < 32000 as core::ffi::c_int {
            k1_min = (((6000 as core::ffi::c_int * 2 as core::ffi::c_int
                * 32 as core::ffi::c_int) as FLOAT32 / fs as FLOAT32)
                as core::ffi::c_double + 0.5f64) as WORD32;
        } else if fs < 64000 as core::ffi::c_int {
            k1_min = (((8000 as core::ffi::c_int * 2 as core::ffi::c_int
                * 32 as core::ffi::c_int) as FLOAT32 / fs as FLOAT32)
                as core::ffi::c_double + 0.5f64) as WORD32;
        } else {
            k1_min = (((10000 as core::ffi::c_int * 2 as core::ffi::c_int
                * 32 as core::ffi::c_int) as FLOAT32 / fs as FLOAT32)
                as core::ffi::c_double + 0.5f64) as WORD32;
        }
    } else if fs < 32000 as core::ffi::c_int {
        k1_min = (((6000 as core::ffi::c_int * 2 as core::ffi::c_int
            * 64 as core::ffi::c_int) as FLOAT32 / fs as FLOAT32) as core::ffi::c_double
            + 0.5f64) as WORD32;
    } else if fs < 64000 as core::ffi::c_int {
        k1_min = (((8000 as core::ffi::c_int * 2 as core::ffi::c_int
            * 64 as core::ffi::c_int) as FLOAT32 / fs as FLOAT32) as core::ffi::c_double
            + 0.5f64) as WORD32;
    } else {
        k1_min = (((10000 as core::ffi::c_int * 2 as core::ffi::c_int
            * 64 as core::ffi::c_int) as FLOAT32 / fs as FLOAT32) as core::ffi::c_double
            + 0.5f64) as WORD32;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i <= 13 as core::ffi::c_int {
        arr_stop_freq[i as usize] = (k1_min as core::ffi::c_double
            * pow(
                64.0f64 / k1_min as core::ffi::c_double,
                i as core::ffi::c_double / 13.0f64,
            ) + 0.5f64) as WORD32 as WORD16;
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i <= 12 as core::ffi::c_int {
        arr_diff_stop_freq[i as usize] = (arr_stop_freq[(i as core::ffi::c_int
            + 1 as core::ffi::c_int) as usize] as core::ffi::c_int
            - arr_stop_freq[i as usize] as core::ffi::c_int) as WORD16;
        i += 1;
    }
    ixheaacd_aac_shellsort(
        &mut *arr_diff_stop_freq.as_mut_ptr().offset(0 as core::ffi::c_int as isize),
        13 as WORD32,
    );
    result = k1_min;
    i = 0 as core::ffi::c_int as WORD32;
    while i < stop_freq {
        result = ixheaac_add32_sat(result, arr_diff_stop_freq[i as usize] as WORD32);
        i += 1;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_calc_k0_k2_bands(
    samp_freq: WORD32,
    start_freq: WORD32,
    stop_freq: WORD32,
    mut upsamp_fac: FLOAT32,
    mut ptr_k0: *mut WORD16,
    mut ptr_k2: *mut WORD16,
) -> IA_ERRORCODE {
    let mut err_code: IA_ERRORCODE = IA_NO_ERROR;
    let mut fs_mapped: WORD32 = 0 as WORD32;
    let mut fs: WORD32 = samp_freq;
    if upsamp_fac == 4 as core::ffi::c_int as FLOAT32 {
        fs = (fs as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
    }
    if fs >= 0 as core::ffi::c_int && fs < 18783 as core::ffi::c_int {
        fs_mapped = 16000 as core::ffi::c_int as WORD32;
    } else if fs >= 18783 as core::ffi::c_int && fs < 23004 as core::ffi::c_int {
        fs_mapped = 22050 as core::ffi::c_int as WORD32;
    } else if fs >= 23004 as core::ffi::c_int && fs < 27713 as core::ffi::c_int {
        fs_mapped = 24000 as core::ffi::c_int as WORD32;
    } else if fs >= 27713 as core::ffi::c_int && fs < 35777 as core::ffi::c_int {
        fs_mapped = 32000 as core::ffi::c_int as WORD32;
    } else if fs >= 35777 as core::ffi::c_int && fs < 42000 as core::ffi::c_int {
        fs_mapped = 40000 as core::ffi::c_int as WORD32;
    } else if fs >= 42000 as core::ffi::c_int && fs < 46009 as core::ffi::c_int {
        fs_mapped = 44100 as core::ffi::c_int as WORD32;
    } else if fs >= 46009 as core::ffi::c_int && fs < 55426 as core::ffi::c_int {
        fs_mapped = 48000 as core::ffi::c_int as WORD32;
    } else if fs >= 55426 as core::ffi::c_int && fs < 75132 as core::ffi::c_int {
        fs_mapped = 64000 as core::ffi::c_int as WORD32;
    } else if fs >= 75132 as core::ffi::c_int && fs < 92017 as core::ffi::c_int {
        fs_mapped = 88200 as core::ffi::c_int as WORD32;
    } else if fs >= 92017 as core::ffi::c_int {
        fs_mapped = 96000 as core::ffi::c_int as WORD32;
    } else {
        return -(1 as IA_ERRORCODE)
    }
    *ptr_k0 = ixheaacd_calc_start_band(fs_mapped, start_freq, upsamp_fac) as WORD16;
    if stop_freq < 14 as core::ffi::c_int {
        *ptr_k2 = ixheaacd_calc_stop_band(samp_freq, stop_freq, upsamp_fac) as WORD16;
    } else if stop_freq == 14 as core::ffi::c_int {
        *ptr_k2 = (2 as core::ffi::c_int * *ptr_k0 as core::ffi::c_int) as WORD16;
    } else {
        *ptr_k2 = (3 as core::ffi::c_int * *ptr_k0 as core::ffi::c_int) as WORD16;
    }
    if *ptr_k2 as core::ffi::c_int > 64 as core::ffi::c_int {
        *ptr_k2 = 64 as WORD16;
    }
    return err_code;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_calc_master_frq_bnd_tbl(
    mut pstr_freq_band_data: *mut ia_freq_band_data_struct,
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut pstr_common_tables: *mut ixheaacd_misc_tables,
) -> IA_ERRORCODE {
    let mut k: WORD32 = 0;
    let mut fs: WORD32 = (*ptr_header_data).out_sampling_freq;
    let mut bands: WORD16 = 0;
    let mut k0: WORD16 = 0 as WORD16;
    let mut k2: WORD16 = 0 as WORD16;
    let mut k1: WORD16 = 0;
    let mut k2_achived: WORD32 = 0;
    let mut k2_diff: WORD32 = 0;
    let mut incr: WORD32 = 0;
    let mut dk: WORD32 = 0;
    let mut vec_dk: [WORD16; 79] = [0; 79];
    let mut vec_dk0: *mut WORD16 = &mut *vec_dk
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut WORD16;
    let mut vec_dk1: *mut WORD16 = &mut *vec_dk.as_mut_ptr().offset(MAX_OCTAVE as isize)
        as *mut WORD16;
    let mut upsamp_fac: WORD16 = (*ptr_header_data).upsamp_fac as WORD16;
    let mut f_master_tbl: *mut WORD16 = ((*pstr_freq_band_data).f_master_tbl)
        .as_mut_ptr();
    let mut num_mf_bands: WORD16 = 0;
    let mut err_code: IA_ERRORCODE = IA_NO_ERROR;
    k1 = 0 as WORD16;
    incr = 0 as core::ffi::c_int as WORD32;
    dk = 0 as core::ffi::c_int as WORD32;
    err_code = ixheaacd_calc_k0_k2_bands(
        fs,
        (*ptr_header_data).start_freq as WORD32,
        (*ptr_header_data).stop_freq as WORD32,
        upsamp_fac as FLOAT32,
        &mut k0,
        &mut k2,
    );
    if err_code != 0 {
        return err_code;
    }
    if k2 as core::ffi::c_int > NO_SYNTHESIS_CHANNELS {
        k2 = NO_SYNTHESIS_CHANNELS as WORD16;
    }
    if upsamp_fac as core::ffi::c_int == 4 as core::ffi::c_int {
        if k2 as core::ffi::c_int - k0 as core::ffi::c_int > MAX_FREQ_COEFFS
            || k2 as core::ffi::c_int <= k0 as core::ffi::c_int
        {
            return -(1 as IA_ERRORCODE);
        }
        if 2 as WORD32 * fs == 44100 as core::ffi::c_int
            && k2 as core::ffi::c_int - k0 as core::ffi::c_int > MAX_FREQ_COEFFS
        {
            return -(1 as IA_ERRORCODE);
        }
        if 2 as WORD32 * fs >= 48000 as core::ffi::c_int
            && k2 as core::ffi::c_int - k0 as core::ffi::c_int > MAX_FREQ_COEFFS
        {
            return -(1 as IA_ERRORCODE);
        }
    } else {
        if k2 as core::ffi::c_int - k0 as core::ffi::c_int > MAX_FREQ_COEFFS_SBR
            || k2 as core::ffi::c_int <= k0 as core::ffi::c_int
        {
            return -(1 as IA_ERRORCODE);
        }
        if fs == 44100 as core::ffi::c_int
            && k2 as core::ffi::c_int - k0 as core::ffi::c_int > MAX_FREQ_COEFFS_FS44100
        {
            return -(1 as IA_ERRORCODE);
        }
        if fs >= 48000 as core::ffi::c_int
            && k2 as core::ffi::c_int - k0 as core::ffi::c_int > MAX_FREQ_COEFFS_FS48000
        {
            return -(1 as IA_ERRORCODE);
        }
    }
    if (*ptr_header_data).freq_scale as core::ffi::c_int == 0 as core::ffi::c_int {
        let mut num_bands: WORD16 = 0;
        if (*ptr_header_data).alter_scale as core::ffi::c_int == 0 as core::ffi::c_int {
            dk = 1 as core::ffi::c_int as WORD32;
            num_bands = (k2 as core::ffi::c_int - k0 as core::ffi::c_int) as WORD16;
            num_bands = (num_bands as core::ffi::c_int
                - (num_bands as core::ffi::c_int & 0x1 as core::ffi::c_int)) as WORD16;
        } else {
            dk = 2 as core::ffi::c_int as WORD32;
            num_bands = ((k2 as core::ffi::c_int - k0 as core::ffi::c_int
                + 2 as core::ffi::c_int) as WORD16 as core::ffi::c_int
                >> 2 as core::ffi::c_int) as WORD16;
            num_bands = ((num_bands as core::ffi::c_int) << 1 as core::ffi::c_int)
                as WORD16;
        }
        if (num_bands as core::ffi::c_int) < 1 as core::ffi::c_int {
            return -(1 as IA_ERRORCODE);
        }
        k2_achived = (k0 as core::ffi::c_int
            + ((num_bands as core::ffi::c_int)
                << dk as core::ffi::c_int - 1 as core::ffi::c_int)) as WORD32;
        k2_diff = k2 as WORD32 - k2_achived;
        k = 0 as core::ffi::c_int as WORD32;
        while k < num_bands as core::ffi::c_int {
            vec_dk[k as usize] = dk as WORD16;
            k += 1;
        }
        if k2_diff < 0 as core::ffi::c_int {
            incr = 1 as core::ffi::c_int as WORD32;
            k = 0 as core::ffi::c_int as WORD32;
        }
        if k2_diff > 0 as core::ffi::c_int {
            incr = -(1 as core::ffi::c_int) as WORD32;
            k = (num_bands as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        }
        while k2_diff != 0 as core::ffi::c_int {
            vec_dk[k as usize] = (vec_dk[k as usize] as WORD32 - incr) as WORD16;
            k = (k + incr) as WORD16 as WORD32;
            k2_diff = k2_diff + incr;
        }
        *f_master_tbl.offset(0 as core::ffi::c_int as isize) = k0;
        k = 1 as core::ffi::c_int as WORD32;
        while k <= num_bands as core::ffi::c_int {
            *f_master_tbl.offset(k as isize) = (*f_master_tbl
                .offset((k as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
                as core::ffi::c_int
                + vec_dk[(k as core::ffi::c_int - 1 as core::ffi::c_int) as usize]
                    as core::ffi::c_int) as WORD16;
            k += 1;
        }
        num_mf_bands = num_bands;
    } else {
        let mut num_bands0: WORD32 = 0;
        let mut num_bands1: WORD32 = 0;
        match (*ptr_header_data).freq_scale as core::ffi::c_int {
            1 => {
                bands = 12 as WORD16;
            }
            2 => {
                bands = 10 as WORD16;
            }
            3 => {
                bands = 8 as WORD16;
            }
            _ => {
                bands = 8 as WORD16;
            }
        }
        if upsamp_fac as core::ffi::c_int == 4 as core::ffi::c_int
            && (k0 as core::ffi::c_int) < bands as core::ffi::c_int
        {
            bands = (k0 as core::ffi::c_int
                - (k0 as core::ffi::c_int & 1 as core::ffi::c_int)) as WORD16;
        }
        if 10000 as core::ffi::c_int * k2 as core::ffi::c_int
            > 22449 as core::ffi::c_int * k0 as core::ffi::c_int
        {
            k1 = ((k0 as core::ffi::c_int) << 1 as core::ffi::c_int) as WORD16;
            num_bands0 = bands as WORD32;
            num_bands1 = ((*pstr_common_tables).log_dual_is_table[k2 as usize]
                as core::ffi::c_int
                - (*pstr_common_tables).log_dual_is_table[k1 as usize]
                    as core::ffi::c_int) as WORD32;
            num_bands1 = bands as WORD32 * num_bands1;
            if (*ptr_header_data).alter_scale != 0 {
                num_bands1 = (num_bands1 as WORD64 * 0x6276 as WORD64
                    >> 15 as core::ffi::c_int) as WORD32;
            }
            num_bands1 = (num_bands1 as core::ffi::c_int + 0x1000 as core::ffi::c_int)
                as WORD32;
            num_bands1 = num_bands1 >> 13 as core::ffi::c_int;
            num_bands1 = num_bands1 << 1 as core::ffi::c_int;
            if num_bands0 < 1 as core::ffi::c_int {
                return -(1 as IA_ERRORCODE);
            }
            if num_bands1 < 1 as core::ffi::c_int {
                return -(1 as IA_ERRORCODE);
            }
            ixheaacd_calc_bands(vec_dk0, k0, k1, num_bands0 as WORD16);
            ixheaacd_aac_shellsort(vec_dk0, num_bands0);
            *f_master_tbl.offset(0 as core::ffi::c_int as isize) = k0;
            k = 1 as core::ffi::c_int as WORD32;
            while k <= num_bands0 {
                *f_master_tbl.offset(k as isize) = (*f_master_tbl
                    .offset((k as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_int
                    + *vec_dk0
                        .offset((k as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int) as WORD16;
                k += 1;
            }
            ixheaacd_calc_bands(vec_dk1, k1, k2, num_bands1 as WORD16);
            ixheaacd_aac_shellsort(vec_dk1, num_bands1);
            if (*vec_dk1.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
                < *vec_dk0
                    .offset(
                        (num_bands0 as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                    ) as core::ffi::c_int
            {
                let mut change: WORD16 = (*vec_dk0
                    .offset(
                        (num_bands0 as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                    ) as core::ffi::c_int
                    - *vec_dk1.offset(0 as core::ffi::c_int as isize)
                        as core::ffi::c_int) as WORD16;
                let mut temp: WORD16 = (*vec_dk1
                    .offset(
                        (num_bands1 as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                    ) as core::ffi::c_int
                    - *vec_dk1.offset(0 as core::ffi::c_int as isize)
                        as core::ffi::c_int) as WORD16;
                temp = (temp as core::ffi::c_int >> 1 as core::ffi::c_int) as WORD16;
                if change as core::ffi::c_int > temp as core::ffi::c_int {
                    change = temp;
                }
                *vec_dk1.offset(0 as core::ffi::c_int as isize) = (*vec_dk1
                    .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                    + change as core::ffi::c_int) as WORD16;
                *vec_dk1
                    .offset(
                        (num_bands1 as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                    ) = (*vec_dk1
                    .offset(
                        (num_bands1 as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                    ) as core::ffi::c_int - change as core::ffi::c_int) as WORD16;
                ixheaacd_aac_shellsort(vec_dk1, num_bands1);
            }
            *f_master_tbl.offset(num_bands0 as isize) = k1;
            k = 1 as core::ffi::c_int as WORD32;
            while k <= num_bands1 {
                *f_master_tbl.offset((num_bands0 + k) as isize) = (*f_master_tbl
                    .offset(
                        (num_bands0 as core::ffi::c_int + k as core::ffi::c_int
                            - 1 as core::ffi::c_int) as isize,
                    ) as core::ffi::c_int
                    + *vec_dk1
                        .offset((k as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int) as WORD16;
                k += 1;
            }
            num_mf_bands = (num_bands0 + num_bands1) as WORD16;
        } else {
            k1 = k2;
            num_bands0 = ((*pstr_common_tables).log_dual_is_table[k1 as usize]
                as core::ffi::c_int
                - (*pstr_common_tables).log_dual_is_table[k0 as usize]
                    as core::ffi::c_int) as WORD32;
            num_bands0 = bands as WORD32 * num_bands0;
            num_bands0 = (num_bands0 as core::ffi::c_int + 0x1000 as core::ffi::c_int)
                as WORD32;
            num_bands0 = num_bands0 >> 13 as core::ffi::c_int;
            num_bands0 = num_bands0 << 1 as core::ffi::c_int;
            if num_bands0 < 1 as core::ffi::c_int {
                return -(1 as IA_ERRORCODE);
            }
            ixheaacd_calc_bands(vec_dk0, k0, k1, num_bands0 as WORD16);
            ixheaacd_aac_shellsort(vec_dk0, num_bands0);
            if *vec_dk0.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                == 0 as core::ffi::c_int
            {
                return -(1 as IA_ERRORCODE);
            }
            *f_master_tbl.offset(0 as core::ffi::c_int as isize) = k0;
            k = 1 as core::ffi::c_int as WORD32;
            while k <= num_bands0 {
                *f_master_tbl.offset(k as isize) = (*f_master_tbl
                    .offset((k as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_int
                    + *vec_dk0
                        .offset((k as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int) as WORD16;
                k += 1;
            }
            num_mf_bands = num_bands0 as WORD16;
        }
    }
    if (num_mf_bands as core::ffi::c_int) < 1 as core::ffi::c_int {
        return -(1 as IA_ERRORCODE);
    }
    (*pstr_freq_band_data).num_mf_bands = num_mf_bands;
    if upsamp_fac as core::ffi::c_int == 4 as core::ffi::c_int {
        k = 1 as core::ffi::c_int as WORD32;
        while k < num_mf_bands as core::ffi::c_int {
            if !(*f_master_tbl.offset(k as isize) as core::ffi::c_int
                - *f_master_tbl
                    .offset((k as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_int
                <= k0 as core::ffi::c_int - 2 as core::ffi::c_int)
            {
                return -(1 as IA_ERRORCODE);
            }
            k += 1;
        }
    }
    return 0 as IA_ERRORCODE;
}
unsafe extern "C" fn ixheaacd_calc_freq_ratio(
    mut k_start: WORD16,
    mut k_stop: WORD16,
    mut num_bands: WORD16,
) -> WORD16 {
    let mut bandfactor: WORD32 = 0;
    let mut step: WORD32 = 0;
    let mut direction: WORD32 = 0;
    let mut start: WORD32 = 0;
    let mut stop: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut i: WORD32 = 0;
    bandfactor = 0x3f000000 as WORD32;
    step = 0x20000000 as WORD32;
    direction = 1 as core::ffi::c_int as WORD32;
    start = ixheaac_shl32(ixheaac_deposit16l_in32(k_start), INT_BITS - 8 as WORD);
    stop = ixheaac_shl32(ixheaac_deposit16l_in32(k_stop), INT_BITS - 8 as WORD);
    i = 0 as core::ffi::c_int as WORD32;
    loop {
        i = (i as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
        temp = stop;
        j = 0 as core::ffi::c_int as WORD32;
        while j < num_bands as core::ffi::c_int {
            temp = ixheaac_mult16x16in32_shl(
                ixheaac_extract16h(temp),
                ixheaac_extract16h(bandfactor),
            );
            j += 1;
        }
        if temp < start {
            if direction == 0 as core::ffi::c_int {
                step = ixheaac_shr32(step, 1 as WORD);
            }
            direction = 1 as core::ffi::c_int as WORD32;
            bandfactor = ixheaac_add32_sat(bandfactor, step);
        } else {
            if direction == 1 as core::ffi::c_int {
                step = ixheaac_shr32(step, 1 as WORD);
            }
            direction = 0 as core::ffi::c_int as WORD32;
            bandfactor = ixheaac_sub32_sat(bandfactor, step);
        }
        if i > 100 as core::ffi::c_int {
            step = 0 as core::ffi::c_int as WORD32;
        }
        if !(step > 0 as core::ffi::c_int) {
            break;
        }
    }
    return ixheaac_extract16h(bandfactor);
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_calc_bands(
    mut diff: *mut WORD16,
    mut start: WORD16,
    mut stop: WORD16,
    mut num_bands: WORD16,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut previous: WORD32 = 0;
    let mut current: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut exact: WORD32 = 0;
    let mut bandfactor: WORD16 = ixheaacd_calc_freq_ratio(start, stop, num_bands);
    previous = stop as WORD32;
    exact = ixheaac_shl32_sat(ixheaac_deposit16l_in32(stop), INT_BITS - 8 as WORD);
    i = (num_bands as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    while i >= 0 as core::ffi::c_int {
        exact = ixheaac_mult16x16in32(ixheaac_extract16h(exact), bandfactor);
        temp = ixheaac_add32_sat(exact, 0x400000 as WORD32);
        exact = exact << 1 as core::ffi::c_int;
        current = ixheaac_extract16l(ixheaac_shr32(temp, INT_BITS - 9 as WORD))
            as WORD32;
        *diff.offset(i as isize) = (previous - current) as WORD16;
        previous = current;
        i -= 1;
    }
}
unsafe extern "C" fn ixheaacd_derive_hi_lo_freq_bnd_tbls(
    mut pstr_freq_band_data: *mut ia_freq_band_data_struct,
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
) -> VOID {
    let mut k: WORD16 = 0;
    let mut xover_band: WORD16 = (*ptr_header_data).xover_band;
    let mut f_master_tbl: *mut WORD16 = ((*pstr_freq_band_data).f_master_tbl)
        .as_mut_ptr()
        .offset(xover_band as core::ffi::c_int as isize);
    let mut f_low_tbl: *mut WORD16 = (*pstr_freq_band_data)
        .freq_band_table[LOW as usize];
    let mut f_high_tbl: *mut WORD16 = (*pstr_freq_band_data)
        .freq_band_table[HIGH as usize];
    let mut num_mf_bands: WORD16 = (*pstr_freq_band_data).num_mf_bands;
    let mut num_lf_bands: WORD16 = 0;
    let mut num_hf_bands: WORD16 = 0;
    num_hf_bands = (num_mf_bands as core::ffi::c_int - xover_band as core::ffi::c_int)
        as WORD16;
    k = 0 as WORD16;
    *f_high_tbl = *f_master_tbl;
    *f_low_tbl = *f_high_tbl;
    f_low_tbl = f_low_tbl.offset(1);
    f_high_tbl = f_high_tbl.offset(1);
    f_master_tbl = f_master_tbl.offset(1);
    k += 1;
    if num_hf_bands as core::ffi::c_int & 1 as core::ffi::c_int != 0 {
        *f_high_tbl = *f_master_tbl;
        *f_low_tbl = *f_high_tbl;
        f_high_tbl = f_high_tbl.offset(1);
        f_master_tbl = f_master_tbl.offset(1);
        f_low_tbl = f_low_tbl.offset(1);
        k += 1;
    }
    while k as core::ffi::c_int <= num_hf_bands as core::ffi::c_int {
        *f_high_tbl = *f_master_tbl;
        f_high_tbl = f_high_tbl.offset(1);
        f_master_tbl = f_master_tbl.offset(1);
        k += 1;
        *f_high_tbl = *f_master_tbl;
        *f_low_tbl = *f_high_tbl;
        f_high_tbl = f_high_tbl.offset(1);
        f_master_tbl = f_master_tbl.offset(1);
        f_low_tbl = f_low_tbl.offset(1);
        k += 1;
    }
    num_lf_bands = (num_hf_bands as core::ffi::c_int + 1 as core::ffi::c_int
        >> 1 as core::ffi::c_int) as WORD16;
    (*pstr_freq_band_data).num_sf_bands[LOW as usize] = num_lf_bands;
    (*pstr_freq_band_data).num_sf_bands[HIGH as usize] = num_hf_bands;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_derive_noise_freq_bnd_tbl(
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut pstr_common_tables: *mut ixheaacd_misc_tables,
    mut pstr_freq_band_data: *mut ia_freq_band_data_struct,
) -> WORD32 {
    let mut k2: WORD16 = 0;
    let mut kx: WORD16 = 0;
    let mut temp: WORD32 = 0;
    let mut num_lf_bands: WORD32 = (*pstr_freq_band_data).num_sf_bands[LOW as usize]
        as WORD32;
    let mut num_hf_bands: WORD32 = (*pstr_freq_band_data).num_sf_bands[HIGH as usize]
        as WORD32;
    k2 = *((*pstr_freq_band_data).freq_band_table[HIGH as usize])
        .offset(num_hf_bands as isize);
    kx = *((*pstr_freq_band_data).freq_band_table[HIGH as usize])
        .offset(0 as core::ffi::c_int as isize);
    if (*ptr_header_data).noise_bands as core::ffi::c_int == 0 as core::ffi::c_int {
        temp = 1 as core::ffi::c_int as WORD32;
    } else {
        temp = ((*pstr_common_tables).log_dual_is_table[k2 as usize] as core::ffi::c_int
            - (*pstr_common_tables).log_dual_is_table[kx as usize] as core::ffi::c_int)
            as WORD32;
        temp = (temp as core::ffi::c_int
            * (*ptr_header_data).noise_bands as core::ffi::c_int) as WORD32;
        temp = (temp as core::ffi::c_int + 0x800 as core::ffi::c_int) as WORD32;
        temp = temp >> 12 as core::ffi::c_int;
        if temp == 0 as core::ffi::c_int {
            temp = 1 as core::ffi::c_int as WORD32;
        }
    }
    if temp > MAX_NOISE_COEFFS {
        return -(1 as WORD32);
    }
    (*pstr_freq_band_data).num_nf_bands = temp as WORD16;
    (*pstr_freq_band_data).num_if_bands = (*pstr_freq_band_data).num_nf_bands;
    let mut i_k: WORD16 = 0;
    let mut k: WORD16 = 0;
    let mut num: WORD16 = 0;
    let mut den: WORD16 = 0;
    let mut f_noise_tbl: *mut WORD16 = ((*pstr_freq_band_data).freq_band_tbl_noise)
        .as_mut_ptr();
    let mut f_low_tbl: *mut WORD16 = (*pstr_freq_band_data)
        .freq_band_table[LOW as usize];
    let mut num_nf_bands: WORD32 = (*pstr_freq_band_data).num_nf_bands as WORD32;
    num = num_lf_bands as WORD16;
    den = num_nf_bands as WORD16;
    k = 0 as WORD16;
    *f_noise_tbl = *f_low_tbl.offset(0 as core::ffi::c_int as isize);
    f_noise_tbl = f_noise_tbl.offset(1);
    k += 1;
    i_k = 0 as WORD16;
    while k as core::ffi::c_int <= num_nf_bands {
        i_k = (i_k as core::ffi::c_int
            + ixheaacd_int_div(num as WORD32, den as WORD32) as WORD16
                as core::ffi::c_int) as WORD16;
        *f_noise_tbl = *f_low_tbl.offset(i_k as isize);
        num = (num_lf_bands as core::ffi::c_int - i_k as core::ffi::c_int) as WORD16;
        den = (den as core::ffi::c_int - 1 as core::ffi::c_int) as WORD16;
        f_noise_tbl = f_noise_tbl.offset(1);
        k += 1;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_calc_frq_bnd_tbls(
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
    mut pstr_common_tables: *mut ixheaacd_misc_tables,
) -> WORD32 {
    let mut err: WORD32 = 0;
    let mut num_lf_bands: WORD16 = 0;
    let mut lsb: WORD16 = 0;
    let mut usb: WORD16 = 0;
    let mut pstr_freq_band_data: *mut ia_freq_band_data_struct = (*ptr_header_data)
        .pstr_freq_band_data;
    err = ixheaacd_calc_master_frq_bnd_tbl(
        pstr_freq_band_data,
        ptr_header_data,
        pstr_common_tables,
    ) as WORD32;
    if err != 0
        || (*ptr_header_data).xover_band as core::ffi::c_int
            > (*pstr_freq_band_data).num_mf_bands as core::ffi::c_int
    {
        return -(1 as WORD32);
    }
    ixheaacd_derive_hi_lo_freq_bnd_tbls(pstr_freq_band_data, ptr_header_data);
    num_lf_bands = (*pstr_freq_band_data).num_sf_bands[LOW as usize];
    if num_lf_bands as core::ffi::c_int <= 0 as core::ffi::c_int
        || num_lf_bands as core::ffi::c_int
            > ixheaac_shr16(MAX_FREQ_COEFFS as WORD16, 1 as WORD16) as core::ffi::c_int
    {
        return -(1 as WORD32);
    }
    lsb = *((*pstr_freq_band_data).freq_band_table[LOW as usize])
        .offset(0 as core::ffi::c_int as isize);
    usb = *((*pstr_freq_band_data).freq_band_table[LOW as usize])
        .offset(num_lf_bands as isize);
    (*pstr_freq_band_data).sub_band_start = lsb;
    (*ptr_header_data).status = 1 as core::ffi::c_int as WORD32;
    if lsb as core::ffi::c_int
        > (if (*ptr_header_data).sbr_ratio_idx == SBR_UPSAMPLE_IDX_4_1 {
            16 as core::ffi::c_int
        } else {
            32 as core::ffi::c_int
        }) || lsb as core::ffi::c_int >= usb as core::ffi::c_int
    {
        return -(1 as WORD32);
    }
    if ixheaacd_derive_noise_freq_bnd_tbl(
        ptr_header_data,
        pstr_common_tables,
        pstr_freq_band_data,
    ) != 0
    {
        return -(1 as WORD32);
    }
    (*pstr_freq_band_data).sub_band_start = lsb;
    (*pstr_freq_band_data).sub_band_end = usb;
    return 0 as WORD32;
}
pub const MAX_32: WORD32 = 0x7fffffff as core::ffi::c_long as WORD32;
pub const MIN_32: WORD32 = 0x80000000 as core::ffi::c_long as WORD32;
