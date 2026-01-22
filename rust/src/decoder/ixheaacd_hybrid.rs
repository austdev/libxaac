extern "C" {
    static mut ixheaacd_inv_dit_fft_8pt: Option<
        unsafe extern "C" fn(*mut WORD32, *mut WORD32, *mut WORD32) -> VOID,
    >;
}
pub type WORD8 = core::ffi::c_schar;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD16 = core::ffi::c_short;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type WORD64 = core::ffi::c_longlong;
pub type FLOAT32 = core::ffi::c_float;
pub type VOID = ();
pub type FLAG = core::ffi::c_int;
pub type WORD = core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_frame_info_struct {
    pub frame_class: WORD16,
    pub num_env: WORD16,
    pub transient_env: WORD16,
    pub num_noise_env: WORD16,
    pub border_vec: [WORD16; 9],
    pub freq_res: [WORD16; 8],
    pub noise_border_vec: [WORD16; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_env_calc_tables_struct {
    pub sbr_lim_gains_m: [WORD16; 8],
    pub sbr_lim_bands_per_octave_q13: [WORD16; 4],
    pub sbr_smooth_filter: [WORD16; 4],
    pub sbr_inv_int_table: [WORD16; 49],
    pub sbr_rand_ph: [WORD32; 568],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_qmf_dec_tables_struct {
    pub w_32: [WORD16; 60],
    pub w_16: [WORD16; 24],
    pub dig_rev_table2_32: [WORD32; 4],
    pub dig_rev_table4_16: [WORD32; 2],
    pub sbr_sin_cos_twiddle_l64: [WORD16; 64],
    pub sbr_alt_sin_twiddle_l64: [WORD16; 32],
    pub sbr_cos_sin_twiddle_ds_l32: [WORD16; 64],
    pub sbr_sin_cos_twiddle_l32: [WORD16; 32],
    pub sbr_alt_sin_twiddle_l32: [WORD16; 16],
    pub sbr_t_cos_sin_l32: [WORD16; 64],
    pub post_fft_tbl: [WORD16; 18],
    pub dct23_tw: [WORD16; 66],
    pub qmf_c: [WORD16; 1280],
    pub dig_rev_table2_128: [UWORD8; 4],
    pub w1024: [WORD32; 1536],
    pub esbr_qmf_c: [WORD32; 1280],
    pub esbr_qmf_c_24: [WORD32; 480],
    pub esbr_w_32: [WORD32; 60],
    pub esbr_w_16: [WORD32; 24],
    pub esbr_sin_cos_twiddle_l64: [WORD32; 64],
    pub esbr_alt_sin_twiddle_l64: [WORD32; 32],
    pub esbr_sin_cos_twiddle_l32: [WORD32; 32],
    pub esbr_alt_sin_twiddle_l32: [WORD32; 16],
    pub esbr_t_cos_sin_l32: [WORD32; 64],
    pub esbr_sin_cos_twiddle_l24: [WORD32; 24],
    pub esbr_alt_sin_twiddle_l24: [WORD32; 12],
    pub esbr_t_cos_sin_l24: [WORD32; 48],
    pub esbr_sin_cos_twiddle_l16: [WORD32; 16],
    pub esbr_alt_sin_twiddle_l16: [WORD32; 8],
    pub esbr_t_cos_sin_l16: [WORD32; 32],
    pub ixheaacd_sbr_t_cos_sin_l32_eld: [WORD16; 64],
    pub qmf_c_eld: [WORD16; 640],
    pub qmf_c_eld2: [WORD16; 640],
    pub qmf_c_eld3: [WORD16; 640],
    pub qmf_c_ldsbr_mps: [WORD32; 640],
    pub ixheaacd_sbr_synth_cos_sin_l32: [WORD16; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_env_extr_tables_struct {
    pub sbr_frame_info1_2_4_16: [ia_frame_info_struct; 7],
    pub str_sbr_default_header: ia_sbr_header_data_struct,
    pub ixheaacd_t_huffman_env_bal_1_5db_inp_table: [WORD16; 50],
    pub ixheaacd_f_huffman_env_bal_1_5db_inp_table: [WORD16; 50],
    pub ixheaacd_t_huffman_env_bal_3_0db_inp_table: [WORD16; 26],
    pub ixheaacd_f_huffman_env_bal_3_0db_inp_table: [WORD16; 26],
    pub ixheaacd_t_huffman_noise_3_0db_inp_table: [WORD16; 64],
    pub ixheaacd_t_huffman_noise_bal_3_0db_inp_table: [WORD16; 26],
    pub ixheaacd_t_huffman_env_1_5db_inp_table: [WORD16; 122],
    pub ixheaacd_f_huffman_env_1_5db_inp_table: [WORD16; 122],
    pub ixheaacd_t_huffman_env_3_0db_inp_table: [WORD16; 64],
    pub ixheaacd_f_huffman_env_3_0db_inp_table: [WORD16; 64],
    pub ixheaacd_t_huffman_env_bal_1_5db_idx_table: [WORD32; 20],
    pub ixheaacd_f_huffman_env_bal_1_5db_idx_table: [WORD32; 23],
    pub ixheaacd_t_huffman_env_bal_3_0db_idx_table: [WORD32; 16],
    pub ixheaacd_f_huffman_env_bal_3_0db_idx_table: [WORD32; 17],
    pub ixheaacd_t_huffman_noise_3_0db_idx_table: [WORD32; 17],
    pub ixheaacd_t_huffman_noise_bal_3_0db_idx_table: [WORD32; 11],
    pub ixheaacd_t_huffman_env_1_5db_idx_table: [WORD32; 27],
    pub ixheaacd_f_huffman_env_1_5db_idx_table: [WORD32; 28],
    pub ixheaacd_t_huffman_env_3_0db_idx_table: [WORD32; 26],
    pub ixheaacd_f_huffman_env_3_0db_idx_table: [WORD32; 25],
    pub start_min: [WORD8; 12],
    pub offset_idx: [WORD8; 12],
    pub ixheaacd_drc_offset: [[WORD8; 16]; 7],
    pub stop_min: [WORD8; 12],
    pub stop_off: [[WORD8; 14]; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_ps_tables_struct {
    pub decay_scale_factor: [WORD16; 72],
    pub hyb_resol: [WORD16; 3],
    pub rev_link_decay_ser: [WORD16; 3],
    pub rev_link_delay_ser: [WORD16; 3],
    pub borders_group: [WORD16; 23],
    pub group_shift: [WORD16; 6],
    pub group_to_bin: [WORD16; 22],
    pub hybrid_to_bin: [WORD16; 10],
    pub delay_to_bin: [WORD16; 32],
    pub frac_delay_phase_fac_qmf_re_im: [WORD16; 48],
    pub frac_delay_phase_fac_qmf_sub_re_im: [WORD16; 32],
    pub frac_delay_phase_fac_qmf_ser_re_im: [[WORD16; 64]; 3],
    pub frac_delay_phase_fac_qmf_sub_ser_re_im: [[WORD16; 32]; 3],
    pub scale_factors: [WORD16; 15],
    pub scale_factors_fine: [WORD16; 31],
    pub alpha_values: [WORD16; 8],
    pub p2_6: [WORD16; 6],
    pub p8_13: [WORD16; 13],
    pub huff_iid_dt: [WORD16; 28],
    pub huff_iid_df: [WORD16; 28],
    pub huff_icc_dt: [WORD16; 14],
    pub huff_icc_df: [WORD16; 14],
    pub huff_iid_dt_fine: [WORD16; 60],
    pub huff_iid_df_fine: [WORD16; 60],
    pub dummy: WORD32,
    pub qmf_fract_delay_phase_factor_im: [FLOAT32; 64],
    pub qmf_fract_delay_phase_factor_re: [FLOAT32; 64],
    pub frac_delay_phase_fac_qmf_sub_im_20: [FLOAT32; 12],
    pub frac_delay_phase_fac_qmf_sub_re_20: [FLOAT32; 12],
    pub frac_delay_phase_fac_qmf_sub_im_34: [FLOAT32; 32],
    pub frac_delay_phase_fac_qmf_sub_re_34: [FLOAT32; 32],
    pub qmf_ser_fract_delay_phase_factor_im: [[FLOAT32; 3]; 64],
    pub qmf_ser_fract_delay_phase_factor_re: [[FLOAT32; 3]; 64],
    pub frac_delay_phase_fac_ser_qmf_sub_im_20: [[FLOAT32; 3]; 12],
    pub frac_delay_phase_fac_ser_qmf_sub_re_20: [[FLOAT32; 3]; 12],
    pub frac_delay_phase_fac_ser_qmf_sub_im_34: [[FLOAT32; 3]; 32],
    pub frac_delay_phase_fac_ser_qmf_sub_re_34: [[FLOAT32; 3]; 32],
    pub scale_factors_flt: [FLOAT32; 15],
    pub scale_factors_fine_flt: [FLOAT32; 31],
    pub alphas: [FLOAT32; 8],
    pub all_pass_link_decay_ser: [FLOAT32; 3],
    pub p8_13_20: [FLOAT32; 13],
    pub p2_13_20: [FLOAT32; 13],
    pub p12_13_34: [FLOAT32; 13],
    pub p8_13_34: [FLOAT32; 13],
    pub p4_13_34: [FLOAT32; 13],
    pub cos_mod_2channel: [[FLOAT32; 13]; 2],
    pub cos_sin_mod_4channel: [[FLOAT32; 26]; 4],
    pub cos_sin_mod_8channel: [[FLOAT32; 26]; 8],
    pub cos_sin_mod_12channel: [[FLOAT32; 26]; 12],
    pub qmf_delay_idx_tbl: [WORD32; 64],
    pub group_borders_20_tbl: [WORD32; 23],
    pub group_borders_34_tbl: [WORD32; 51],
    pub bin_group_map_20: [WORD32; 22],
    pub bin_group_map_34: [WORD32; 50],
    pub quantized_iids: [WORD32; 7],
    pub quantized_iids_fine: [WORD32; 15],
    pub quantized_rhos: [FLOAT32; 8],
    pub ipd_bins_tbl: [WORD32; 3],
    pub band_res_hyb20: [WORD16; 3],
    pub band_res_hyb34: [WORD16; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_tables_struct {
    pub env_calc_tables_ptr: *mut ia_env_calc_tables_struct,
    pub qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
    pub env_extr_tables_ptr: *mut ia_env_extr_tables_struct,
    pub ps_tables_ptr: *mut ia_ps_tables_struct,
    pub sbr_rand_ph: *mut WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_hybrid_struct {
    pub ptr_resol: *const WORD16,
    pub ptr_qmf_buf: WORD8,
    pub ptr_work_re: *mut WORD32,
    pub ptr_work_im: *mut WORD32,
    pub ptr_qmf_buf_re: [*mut WORD32; 3],
    pub ptr_qmf_buf_im: [*mut WORD32; 3],
    pub ptr_temp_re: *mut WORD32,
    pub ptr_temp_im: *mut WORD32,
}
pub const MAX_32: WORD32 = 0x7fffffff as core::ffi::c_long as WORD32;
pub const MIN_32: WORD32 = 0x80000000 as core::ffi::c_long as WORD32;
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
unsafe extern "C" fn ixheaac_mult32x16in32(mut a: WORD32, mut b: WORD16) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 16 as core::ffi::c_int) as WORD32;
    return result;
}
pub const HYBRID_FILTER_DELAY: core::ffi::c_int = 6 as core::ffi::c_int;
pub const NO_QMF_CHANNELS_IN_HYBRID: core::ffi::c_int = 3 as core::ffi::c_int;
pub const NO_HYBRID_CHANNELS_LOW: core::ffi::c_int = 2;
pub const NO_HYBRID_CHANNELS_HIGH: core::ffi::c_int = 8;
unsafe extern "C" fn ixheaacd_filt_2_ch(
    mut ptr_qmf: *const WORD32,
    mut ptr_hybrid: *mut WORD32,
    mut ptr_sbr_tables: *mut ia_sbr_tables_struct,
) -> VOID {
    let mut cum0: WORD32 = 0;
    let mut cum1: WORD32 = 0;
    let mut cum00: WORD32 = 0;
    let mut cum11: WORD32 = 0;
    let mut p2_6: *mut WORD16 = ((*(*ptr_sbr_tables).ps_tables_ptr).p2_6).as_mut_ptr();
    cum0 = *ptr_qmf.offset(HYBRID_FILTER_DELAY as isize) >> 1 as core::ffi::c_int;
    cum00 = *ptr_qmf.offset((HYBRID_FILTER_DELAY + 16 as core::ffi::c_int) as isize)
        >> 1 as core::ffi::c_int;
    cum1 = 0 as WORD32;
    cum11 = 0 as WORD32;
    cum1 = ixheaac_add32_sat(
        cum1,
        ixheaac_mult32x16in32(*ptr_qmf.offset(1 as core::ffi::c_int as isize), *p2_6),
    );
    let fresh15 = p2_6;
    p2_6 = p2_6.offset(1);
    cum11 = ixheaac_add32_sat(
        cum11,
        ixheaac_mult32x16in32(*ptr_qmf.offset(17 as core::ffi::c_int as isize), *fresh15),
    );
    cum1 = ixheaac_add32_sat(
        cum1,
        ixheaac_mult32x16in32(*ptr_qmf.offset(3 as core::ffi::c_int as isize), *p2_6),
    );
    let fresh16 = p2_6;
    p2_6 = p2_6.offset(1);
    cum11 = ixheaac_add32_sat(
        cum11,
        ixheaac_mult32x16in32(*ptr_qmf.offset(19 as core::ffi::c_int as isize), *fresh16),
    );
    cum1 = ixheaac_add32_sat(
        cum1,
        ixheaac_mult32x16in32(*ptr_qmf.offset(5 as core::ffi::c_int as isize), *p2_6),
    );
    let fresh17 = p2_6;
    p2_6 = p2_6.offset(1);
    cum11 = ixheaac_add32_sat(
        cum11,
        ixheaac_mult32x16in32(*ptr_qmf.offset(21 as core::ffi::c_int as isize), *fresh17),
    );
    cum1 = ixheaac_add32_sat(
        cum1,
        ixheaac_mult32x16in32(*ptr_qmf.offset(7 as core::ffi::c_int as isize), *p2_6),
    );
    let fresh18 = p2_6;
    p2_6 = p2_6.offset(1);
    cum11 = ixheaac_add32_sat(
        cum11,
        ixheaac_mult32x16in32(*ptr_qmf.offset(23 as core::ffi::c_int as isize), *fresh18),
    );
    cum1 = ixheaac_add32_sat(
        cum1,
        ixheaac_mult32x16in32(*ptr_qmf.offset(9 as core::ffi::c_int as isize), *p2_6),
    );
    let fresh19 = p2_6;
    p2_6 = p2_6.offset(1);
    cum11 = ixheaac_add32_sat(
        cum11,
        ixheaac_mult32x16in32(*ptr_qmf.offset(25 as core::ffi::c_int as isize), *fresh19),
    );
    cum1 = ixheaac_add32_sat(
        cum1,
        ixheaac_mult32x16in32(*ptr_qmf.offset(11 as core::ffi::c_int as isize), *p2_6),
    );
    let fresh20 = p2_6;
    p2_6 = p2_6.offset(1);
    cum11 = ixheaac_add32_sat(
        cum11,
        ixheaac_mult32x16in32(*ptr_qmf.offset(27 as core::ffi::c_int as isize), *fresh20),
    );
    cum1 = ixheaac_shl32(cum1, 1 as WORD);
    cum11 = ixheaac_shl32(cum11, 1 as WORD);
    *ptr_hybrid.offset(0 as core::ffi::c_int as isize) = ixheaac_add32_sat(cum0, cum1);
    *ptr_hybrid.offset(1 as core::ffi::c_int as isize) = ixheaac_sub32_sat(cum0, cum1);
    *ptr_hybrid.offset(16 as core::ffi::c_int as isize) = ixheaac_add32_sat(
        cum00,
        cum11,
    );
    *ptr_hybrid.offset(17 as core::ffi::c_int as isize) = ixheaac_sub32_sat(
        cum00,
        cum11,
    );
}
unsafe extern "C" fn ixheaacd_filt_8_ch(
    mut ptr_qmf_real: *const WORD32,
    mut ptr_qmf_imag: *const WORD32,
    mut ptr_hyb_real: *mut WORD32,
    mut ptr_hyb_imag: *mut WORD32,
    mut ptr_sbr_tables: *mut ia_sbr_tables_struct,
) -> VOID {
    let tcos: WORD16 = 0x7642 as WORD16;
    let tsin: WORD16 = 0x30fc as WORD16;
    let tcom: WORD16 = 0x5a82 as WORD16;
    let mut real: WORD32 = 0;
    let mut imag: WORD32 = 0;
    let mut cum: [WORD32; 16] = [0; 16];
    let mut p8_13: *const WORD16 = ((*(*ptr_sbr_tables).ps_tables_ptr).p8_13)
        .as_mut_ptr();
    let mut p8_13_8: *const WORD16 = ((*(*ptr_sbr_tables).ps_tables_ptr).p8_13)
        .as_mut_ptr()
        .offset(8 as core::ffi::c_int as isize);
    real = ixheaac_shl32(
        ixheaac_add32_sat(
            ixheaac_mult32x16in32(
                *ptr_qmf_real.offset(0 as core::ffi::c_int as isize),
                *p8_13,
            ),
            ixheaac_mult32x16in32(
                *ptr_qmf_real.offset(8 as core::ffi::c_int as isize),
                *p8_13_8,
            ),
        ),
        1 as WORD,
    );
    let fresh2 = p8_13;
    p8_13 = p8_13.offset(1);
    let fresh3 = p8_13_8;
    p8_13_8 = p8_13_8.offset(1);
    imag = ixheaac_shl32(
        ixheaac_add32_sat(
            ixheaac_mult32x16in32(
                *ptr_qmf_imag.offset(0 as core::ffi::c_int as isize),
                *fresh2,
            ),
            ixheaac_mult32x16in32(
                *ptr_qmf_imag.offset(8 as core::ffi::c_int as isize),
                *fresh3,
            ),
        ),
        1 as WORD,
    );
    cum[12 as core::ffi::c_int as usize] = ixheaac_shl32(
        ixheaac_mult32x16in32(ixheaac_add32_sat(imag, real), tcom),
        1 as WORD,
    );
    cum[13 as core::ffi::c_int as usize] = ixheaac_shl32(
        ixheaac_mult32x16in32(ixheaac_sub32_sat(imag, real), tcom),
        1 as WORD,
    );
    real = ixheaac_shl32(
        ixheaac_add32_sat(
            ixheaac_mult32x16in32(
                *ptr_qmf_real.offset(1 as core::ffi::c_int as isize),
                *p8_13,
            ),
            ixheaac_mult32x16in32(
                *ptr_qmf_real.offset(9 as core::ffi::c_int as isize),
                *p8_13_8,
            ),
        ),
        1 as WORD,
    );
    let fresh4 = p8_13;
    p8_13 = p8_13.offset(1);
    let fresh5 = p8_13_8;
    p8_13_8 = p8_13_8.offset(1);
    imag = ixheaac_shl32(
        ixheaac_add32_sat(
            ixheaac_mult32x16in32(
                *ptr_qmf_imag.offset(1 as core::ffi::c_int as isize),
                *fresh4,
            ),
            ixheaac_mult32x16in32(
                *ptr_qmf_imag.offset(9 as core::ffi::c_int as isize),
                *fresh5,
            ),
        ),
        1 as WORD,
    );
    cum[10 as core::ffi::c_int as usize] = ixheaac_shl32(
        ixheaac_add32_sat(
            ixheaac_mult32x16in32(imag, tcos),
            ixheaac_mult32x16in32(real, tsin),
        ),
        1 as WORD,
    );
    cum[11 as core::ffi::c_int as usize] = ixheaac_shl32(
        ixheaac_sub32_sat(
            ixheaac_mult32x16in32(imag, tsin),
            ixheaac_mult32x16in32(real, tcos),
        ),
        1 as WORD,
    );
    let fresh6 = p8_13_8;
    p8_13_8 = p8_13_8.offset(1);
    cum[9 as core::ffi::c_int as usize] = ixheaac_shl32(
        ixheaac_mult32x16in32(
            ixheaac_sub32_sat(
                *ptr_qmf_real.offset(2 as core::ffi::c_int as isize),
                *ptr_qmf_real.offset(10 as core::ffi::c_int as isize),
            ),
            *fresh6,
        ),
        1 as WORD,
    );
    let fresh7 = p8_13;
    p8_13 = p8_13.offset(1);
    cum[8 as core::ffi::c_int as usize] = ixheaac_shl32(
        ixheaac_mult32x16in32(
            ixheaac_sub32_sat(
                *ptr_qmf_imag.offset(2 as core::ffi::c_int as isize),
                *ptr_qmf_imag.offset(10 as core::ffi::c_int as isize),
            ),
            *fresh7,
        ),
        1 as WORD,
    );
    real = ixheaac_shl32(
        ixheaac_add32_sat(
            ixheaac_mult32x16in32(
                *ptr_qmf_real.offset(3 as core::ffi::c_int as isize),
                *p8_13,
            ),
            ixheaac_mult32x16in32(
                *ptr_qmf_real.offset(11 as core::ffi::c_int as isize),
                *p8_13_8,
            ),
        ),
        1 as WORD,
    );
    let fresh8 = p8_13;
    p8_13 = p8_13.offset(1);
    let fresh9 = p8_13_8;
    p8_13_8 = p8_13_8.offset(1);
    imag = ixheaac_shl32(
        ixheaac_add32_sat(
            ixheaac_mult32x16in32(
                *ptr_qmf_imag.offset(3 as core::ffi::c_int as isize),
                *fresh8,
            ),
            ixheaac_mult32x16in32(
                *ptr_qmf_imag.offset(11 as core::ffi::c_int as isize),
                *fresh9,
            ),
        ),
        1 as WORD,
    );
    cum[6 as core::ffi::c_int as usize] = ixheaac_shl32(
        ixheaac_sub32_sat(
            ixheaac_mult32x16in32(imag, tcos),
            ixheaac_mult32x16in32(real, tsin),
        ),
        1 as WORD,
    );
    cum[7 as core::ffi::c_int as usize] = ixheaac_shl32(
        ixheaac_negate32_sat(
            ixheaac_add32_sat(
                ixheaac_mult32x16in32(imag, tsin),
                ixheaac_mult32x16in32(real, tcos),
            ),
        ),
        1 as WORD,
    );
    real = ixheaac_shl32(
        ixheaac_add32_sat(
            ixheaac_mult32x16in32(
                *ptr_qmf_real.offset(4 as core::ffi::c_int as isize),
                *p8_13,
            ),
            ixheaac_mult32x16in32(
                *ptr_qmf_real.offset(12 as core::ffi::c_int as isize),
                *p8_13_8,
            ),
        ),
        1 as WORD,
    );
    let fresh10 = p8_13;
    p8_13 = p8_13.offset(1);
    let fresh11 = p8_13_8;
    p8_13_8 = p8_13_8.offset(1);
    imag = ixheaac_shl32(
        ixheaac_add32_sat(
            ixheaac_mult32x16in32(
                *ptr_qmf_imag.offset(4 as core::ffi::c_int as isize),
                *fresh10,
            ),
            ixheaac_mult32x16in32(
                *ptr_qmf_imag.offset(12 as core::ffi::c_int as isize),
                *fresh11,
            ),
        ),
        1 as WORD,
    );
    cum[4 as core::ffi::c_int as usize] = ixheaac_shl32(
        ixheaac_mult32x16in32(ixheaac_sub32_sat(imag, real), tcom),
        1 as WORD,
    );
    cum[5 as core::ffi::c_int as usize] = ixheaac_shl32(
        ixheaac_mult32x16in32(ixheaac_negate32_sat(ixheaac_add32_sat(imag, real)), tcom),
        1 as WORD,
    );
    real = ixheaac_shl32(
        ixheaac_mult32x16in32(
            *ptr_qmf_real.offset(5 as core::ffi::c_int as isize),
            *p8_13,
        ),
        1 as WORD,
    );
    let fresh12 = p8_13;
    p8_13 = p8_13.offset(1);
    imag = ixheaac_shl32(
        ixheaac_mult32x16in32(
            *ptr_qmf_imag.offset(5 as core::ffi::c_int as isize),
            *fresh12,
        ),
        1 as WORD,
    );
    cum[2 as core::ffi::c_int as usize] = ixheaac_shl32(
        ixheaac_sub32_sat(
            ixheaac_mult32x16in32(real, tcos),
            ixheaac_mult32x16in32(imag, tsin),
        ),
        1 as WORD,
    );
    cum[3 as core::ffi::c_int as usize] = ixheaac_shl32(
        ixheaac_add32_sat(
            ixheaac_mult32x16in32(real, tsin),
            ixheaac_mult32x16in32(imag, tcos),
        ),
        1 as WORD,
    );
    cum[0 as core::ffi::c_int as usize] = ixheaac_shl32(
        ixheaac_mult32x16in32(
            *ptr_qmf_real.offset(HYBRID_FILTER_DELAY as isize),
            *p8_13,
        ),
        1 as WORD,
    );
    let fresh13 = p8_13;
    p8_13 = p8_13.offset(1);
    cum[1 as core::ffi::c_int as usize] = ixheaac_shl32(
        ixheaac_mult32x16in32(
            *ptr_qmf_imag.offset(HYBRID_FILTER_DELAY as isize),
            *fresh13,
        ),
        1 as WORD,
    );
    real = ixheaac_shl32(
        ixheaac_mult32x16in32(
            *ptr_qmf_real.offset(7 as core::ffi::c_int as isize),
            *p8_13,
        ),
        1 as WORD,
    );
    let fresh14 = p8_13;
    p8_13 = p8_13.offset(1);
    imag = ixheaac_shl32(
        ixheaac_mult32x16in32(
            *ptr_qmf_imag.offset(7 as core::ffi::c_int as isize),
            *fresh14,
        ),
        1 as WORD,
    );
    cum[14 as core::ffi::c_int as usize] = ixheaac_shl32(
        ixheaac_add32_sat(
            ixheaac_mult32x16in32(imag, tsin),
            ixheaac_mult32x16in32(real, tcos),
        ),
        1 as WORD,
    );
    cum[15 as core::ffi::c_int as usize] = ixheaac_shl32(
        ixheaac_sub32_sat(
            ixheaac_mult32x16in32(imag, tcos),
            ixheaac_mult32x16in32(real, tsin),
        ),
        1 as WORD,
    );
    (Some(ixheaacd_inv_dit_fft_8pt.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(cum.as_mut_ptr(), ptr_hyb_real, ptr_hyb_imag);
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hybrid_analysis(
    mut ptr_qmf_real: *const WORD32,
    mut ptr_hyb_real: *mut WORD32,
    mut ptr_hyb_imag: *mut WORD32,
    mut ptr_hybrid: *mut ia_hybrid_struct,
    mut scale: WORD16,
    mut ptr_sbr_tables: *mut ia_sbr_tables_struct,
) -> VOID {
    let mut band: WORD = 0;
    let mut j: WORD = 0;
    let mut chn_offset: WORD = 0 as WORD;
    let mut ptr_re: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr_im: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr_temp_real: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr_temp_imag: *mut WORD32 = 0 as *mut WORD32;
    band = 0 as core::ffi::c_int as WORD;
    while band < NO_QMF_CHANNELS_IN_HYBRID {
        ptr_re = (*ptr_hybrid).ptr_qmf_buf_re[band as usize];
        ptr_im = (*ptr_hybrid).ptr_qmf_buf_im[band as usize];
        ptr_temp_real = &mut *((*ptr_hybrid).ptr_work_re)
            .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
        ptr_temp_imag = &mut *((*ptr_hybrid).ptr_work_im)
            .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
        *ptr_temp_real = *ptr_re;
        *ptr_temp_imag = *ptr_im;
        ptr_temp_real = ptr_temp_real.offset(1);
        ptr_re = ptr_re.offset(1);
        ptr_temp_imag = ptr_temp_imag.offset(1);
        ptr_im = ptr_im.offset(1);
        j = ((*ptr_hybrid).ptr_qmf_buf as core::ffi::c_int - 2 as core::ffi::c_int)
            as WORD;
        while j >= 0 as core::ffi::c_int {
            let fresh0 = ptr_temp_real;
            ptr_temp_real = ptr_temp_real.offset(1);
            *fresh0 = *ptr_re;
            *ptr_re.offset(-(1 as core::ffi::c_int as isize)) = *ptr_re;
            ptr_re = ptr_re.offset(1);
            let fresh1 = ptr_temp_imag;
            ptr_temp_imag = ptr_temp_imag.offset(1);
            *fresh1 = *ptr_im;
            *ptr_im.offset(-(1 as core::ffi::c_int as isize)) = *ptr_im;
            ptr_im = ptr_im.offset(1);
            j -= 1;
        }
        let mut temp_re: WORD32 = *ptr_qmf_real.offset(band as isize);
        let mut temp_im: WORD32 = *ptr_qmf_real
            .offset((band as core::ffi::c_int + 0x40 as core::ffi::c_int) as isize);
        if (scale as core::ffi::c_int) < 0 as core::ffi::c_int {
            temp_re = ixheaac_shl32(temp_re, -(scale as WORD));
            temp_im = ixheaac_shl32(temp_im, -(scale as WORD));
        } else {
            temp_re = ixheaac_shr32(temp_re, scale as WORD);
            temp_im = ixheaac_shr32(temp_im, scale as WORD);
        }
        *ptr_temp_real = temp_re;
        ptr_re = ptr_re.offset(-1);
        *ptr_re = temp_re;
        *ptr_temp_imag = temp_im;
        ptr_im = ptr_im.offset(-1);
        *ptr_im = temp_im;
        match *((*ptr_hybrid).ptr_resol).offset(band as isize) as core::ffi::c_int {
            NO_HYBRID_CHANNELS_LOW => {
                ixheaacd_filt_2_ch(
                    (*ptr_hybrid).ptr_work_re,
                    &mut *ptr_hyb_real.offset(chn_offset as isize),
                    ptr_sbr_tables,
                );
                chn_offset += 2 as core::ffi::c_int;
            }
            NO_HYBRID_CHANNELS_HIGH => {
                ixheaacd_filt_8_ch(
                    (*ptr_hybrid).ptr_work_re,
                    (*ptr_hybrid).ptr_work_im,
                    &mut *ptr_hyb_real.offset(chn_offset as isize),
                    &mut *ptr_hyb_imag.offset(chn_offset as isize),
                    ptr_sbr_tables,
                );
                chn_offset += 6 as core::ffi::c_int;
            }
            _ => {}
        }
        band += 1;
    }
}
