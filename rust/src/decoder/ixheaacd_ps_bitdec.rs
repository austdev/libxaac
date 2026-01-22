extern "C" {
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn ixheaacd_map_34_params_to_20(a_idx: *mut WORD16) -> VOID;
}
pub type size_t = usize;
pub type WORD8 = core::ffi::c_schar;
pub type WORD16 = core::ffi::c_short;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type FLOAT32 = core::ffi::c_float;
pub type VOID = ();
pub type FLAG = core::ffi::c_int;
pub type WORD = core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_hybrid_flt_struct {
    pub num_qmf_bands: WORD32,
    pub frame_size: WORD32,
    pub ptr_resol: *mut WORD16,
    pub ptr_work_re: *mut FLOAT32,
    pub ptr_work_im: *mut FLOAT32,
    pub ptr_qmf_buf_re: *mut [FLOAT32; 12],
    pub ptr_qmf_buf_im: *mut [FLOAT32; 12],
    pub ptr_temp_re: *mut [FLOAT32; 64],
    pub ptr_temp_im: *mut [FLOAT32; 64],
}
pub type REVERB_BUFFERS_RI = *mut [[WORD16; 64]; 3];
pub type REVERB_BUFFERS_CH_RI = [[[WORD16; 32]; 3]; 5];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_ps_dec_struct {
    pub delay_buf_qmf_ap_re_im: *mut [WORD16; 64],
    pub delay_buf_qmf_ld_re_im: *mut [WORD16; 24],
    pub delay_buf_qmf_sd_re_im: *mut [WORD16; 64],
    pub delay_buf_idx_ser: [WORD16; 3],
    pub delay_sample_ser: [WORD16; 3],
    pub delay_buf_qmf_ser_re_im: REVERB_BUFFERS_RI,
    pub delay_buf_idx: WORD16,
    pub delay_buf_idx_long: WORD16,
    pub peak_decay_diff: *mut WORD32,
    pub energy_prev: *mut WORD32,
    pub peak_decay_diff_prev: *mut WORD32,
    pub ptr_hyb_left_re: *mut WORD32,
    pub ptr_hyb_left_im: *mut WORD32,
    pub ptr_hyb_right_re: *mut WORD32,
    pub ptr_hyb_right_im: *mut WORD32,
    pub delay_buf_qmf_sub_re_im: [[WORD16; 32]; 2],
    pub delay_buf_qmf_sub_ser_re_im: REVERB_BUFFERS_CH_RI,
    pub h11_h12_vec: [WORD16; 48],
    pub h21_h22_vec: [WORD16; 48],
    pub H11_H12: [WORD16; 48],
    pub H21_H22: [WORD16; 48],
    pub delta_h11_h12: [WORD16; 48],
    pub delta_h21_h22: [WORD16; 48],
    pub force_mono: FLAG,
    pub delay_buffer_scale: WORD16,
    pub usb: WORD16,
    pub iid_par_prev: [WORD16; 34],
    pub icc_par_prev: [WORD16; 34],
    pub ps_data_present: FLAG,
    pub enable_iid: FLAG,
    pub enable_icc: FLAG,
    pub enable_ext: FLAG,
    pub iid_mode: WORD16,
    pub icc_mode: WORD16,
    pub iid_quant: FLAG,
    pub frame_class: FLAG,
    pub num_env: WORD16,
    pub border_position: [WORD16; 7],
    pub iid_dt: [FLAG; 5],
    pub icc_dt: [FLAG; 5],
    pub iid_par_table: [[WORD16; 34]; 7],
    pub icc_par_table: [[WORD16; 34]; 7],
    pub str_hybrid: ia_hybrid_struct,
    pub hyb_left_re: [[FLOAT32; 32]; 32],
    pub hyb_left_im: [[FLOAT32; 32]; 32],
    pub hyb_right_re: [[FLOAT32; 32]; 32],
    pub hyb_right_im: [[FLOAT32; 32]; 32],
    pub h11_re_vec: [FLOAT32; 34],
    pub h11_im_vec: [FLOAT32; 34],
    pub h12_re_vec: [FLOAT32; 34],
    pub h12_im_vec: [FLOAT32; 34],
    pub h21_re_vec: [FLOAT32; 34],
    pub h21_im_vec: [FLOAT32; 34],
    pub h22_re_vec: [FLOAT32; 34],
    pub h22_im_vec: [FLOAT32; 34],
    pub h11_re_prev: [FLOAT32; 34],
    pub h11_im_prev: [FLOAT32; 34],
    pub h12_re_prev: [FLOAT32; 34],
    pub h12_im_prev: [FLOAT32; 34],
    pub h21_re_prev: [FLOAT32; 34],
    pub h21_im_prev: [FLOAT32; 34],
    pub h22_re_prev: [FLOAT32; 34],
    pub h22_im_prev: [FLOAT32; 34],
    pub qmf_delay_buf_re: [[FLOAT32; 64]; 14],
    pub qmf_delay_buf_im: [[FLOAT32; 64]; 14],
    pub sub_qmf_delay_buf_re: [[FLOAT32; 64]; 14],
    pub sub_qmf_delay_buf_im: [[FLOAT32; 64]; 14],
    pub ser_qmf_delay_buf_re: [[[FLOAT32; 64]; 5]; 3],
    pub ser_qmf_delay_buf_im: [[[FLOAT32; 64]; 5]; 3],
    pub ptr_hybrid: *mut ia_hybrid_flt_struct,
    pub str_flt_hybrid20: ia_hybrid_flt_struct,
    pub str_flt_hybrid34: ia_hybrid_flt_struct,
    pub use_34_st_bands: WORD32,
    pub use_34_st_bands_prev: WORD32,
    pub ps_mode: WORD32,
    pub ptr_group_borders: *mut WORD32,
    pub num_groups: WORD32,
    pub num_sub_qmf_groups: WORD32,
    pub num_bins: WORD32,
    pub first_delay_gr: WORD32,
    pub ptr_bins_group_map: *mut WORD32,
    pub num_sub_samples: WORD32,
    pub num_chans: WORD32,
    pub use_pca_rot_flg: WORD32,
    pub freq_res_ipd: WORD32,
    pub delay_qmf_delay_buf_idx: [WORD32; 64],
    pub delay_qmf_delay_num_samp: [WORD32; 64],
    pub peak_decay_fast_bin: [FLOAT32; 34],
    pub prev_nrg_bin: [FLOAT32; 34],
    pub prev_peak_diff_bin: [FLOAT32; 34],
    pub ipd_idx_map_1: [WORD32; 17],
    pub opd_idx_map_1: [WORD32; 17],
    pub ipd_idx_map_2: [WORD32; 17],
    pub opd_idx_map_2: [WORD32; 17],
    pub ipd_idx_map: [[WORD32; 17]; 5],
    pub opd_idx_map: [[WORD32; 17]; 5],
    pub ser_sub_qmf_dealy_buf_re: [[[FLOAT32; 64]; 5]; 3],
    pub ser_sub_qmf_dealy_buf_im: [[[FLOAT32; 64]; 5]; 3],
    pub hyb_work_re_20: [FLOAT32; 44],
    pub hyb_work_im_20: [FLOAT32; 44],
    pub hyb_qmf_buf_re_20: [[FLOAT32; 12]; 5],
    pub hyb_qmf_buf_im_20: [[FLOAT32; 12]; 5],
    pub hyb_temp_re_20: [[FLOAT32; 64]; 32],
    pub hyb_temp_im_20: [[FLOAT32; 64]; 32],
    pub hyb_work_re_34: [FLOAT32; 44],
    pub hyb_work_im_34: [FLOAT32; 44],
    pub hyb_qmf_buf_re_34: [[FLOAT32; 12]; 5],
    pub hyb_qmf_buf_im_34: [[FLOAT32; 12]; 5],
    pub hyb_temp_re_34: [[FLOAT32; 64]; 32],
    pub hyb_temp_im_34: [[FLOAT32; 64]; 32],
    pub pp_qmf_buf_real: [*mut *mut FLOAT32; 2],
    pub pp_qmf_buf_imag: [*mut *mut FLOAT32; 2],
    pub time_sample_buf: [*mut FLOAT32; 2],
}
pub const MAX_16: WORD16 = 0x7fff as core::ffi::c_int as WORD16;
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
unsafe extern "C" fn ixheaac_add16(mut op1: WORD16, mut op2: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = (op1 as core::ffi::c_int + op2 as core::ffi::c_int) as WORD16;
    return var_out;
}
#[inline]
unsafe extern "C" fn ixheaac_mult16_shl(mut op1: WORD16, mut op2: WORD16) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = (op1 as WORD32 * op2 as WORD32 >> 15 as core::ffi::c_int) as WORD16;
    return var_out;
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
pub const NUM_IID_LEVELS: core::ffi::c_int = 7 as core::ffi::c_int;
pub const NUM_IID_LEVELS_FINE: core::ffi::c_int = 15 as core::ffi::c_int;
pub const NUM_ICC_LEVELS: core::ffi::c_int = 8 as core::ffi::c_int;
pub const MAX_NUM_COLUMNS: core::ffi::c_int = 32 as core::ffi::c_int;
pub const MAX_NUM_COLUMNS_960: core::ffi::c_int = 30 as core::ffi::c_int;
pub const NUM_BANDS_FINE: core::ffi::c_int = 34 as core::ffi::c_int;
#[no_mangle]
pub static mut ixheaacd_num_bands: [WORD16; 3] = [
    10 as core::ffi::c_int as WORD16,
    20 as core::ffi::c_int as WORD16,
    34 as core::ffi::c_int as WORD16,
];
unsafe extern "C" fn ixheaacd_clamp(
    mut i: WORD32,
    mut min: WORD16,
    mut max: WORD16,
) -> WORD32 {
    let mut result: WORD32 = i;
    if i < min as core::ffi::c_int {
        result = min as WORD32;
    } else if i > max as core::ffi::c_int {
        result = max as WORD32;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_divideby2(mut op: WORD) -> WORD16 {
    let mut sign: FLAG = (op < 0 as core::ffi::c_int) as core::ffi::c_int;
    if sign != 0 {
        op = -op;
    }
    op = op >> 1 as core::ffi::c_int;
    if sign != 0 {
        op = -op;
    }
    return op as WORD16;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_divideby3(mut op: WORD) -> WORD16 {
    let mut temp: WORD16 = 0;
    let mut ret: WORD16 = 0;
    let mut sign: FLAG = (op < 0 as core::ffi::c_int) as core::ffi::c_int;
    if sign != 0 {
        op = -op;
    }
    temp = (op << 2 as core::ffi::c_int) as WORD16;
    temp = ixheaac_mult16_shl(temp, 0x2aab as WORD16);
    ret = (temp as core::ffi::c_int >> 2 as core::ffi::c_int) as WORD16;
    if sign != 0 {
        ret = -(ret as core::ffi::c_int) as WORD16;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_decode_ps_data(
    mut ptr_ps_dec: *mut ia_ps_dec_struct,
    mut frame_size: WORD32,
) -> VOID {
    let mut e: WORD = 0;
    let mut i: WORD = 0;
    let mut temp: WORD = 0;
    let mut iid_mode: WORD16 = (if (*ptr_ps_dec).iid_mode as core::ffi::c_int != 0 {
        1 as core::ffi::c_int
    } else {
        2 as core::ffi::c_int
    }) as WORD16;
    let mut icc_mode: WORD16 = (if (*ptr_ps_dec).icc_mode as core::ffi::c_int != 0 {
        1 as core::ffi::c_int
    } else {
        2 as core::ffi::c_int
    }) as WORD16;
    let mut num_iid_levels: WORD16 = (if (*ptr_ps_dec).iid_quant != 0 {
        NUM_IID_LEVELS_FINE
    } else {
        NUM_IID_LEVELS
    }) as WORD16;
    let mut max_num_columns: WORD32 = 0;
    if frame_size == 960 as core::ffi::c_int {
        max_num_columns = MAX_NUM_COLUMNS_960 as WORD32;
    } else {
        max_num_columns = MAX_NUM_COLUMNS as WORD32;
    }
    if (*ptr_ps_dec).ps_data_present == 0 {
        (*ptr_ps_dec).num_env = 0 as WORD16;
    }
    e = 0 as core::ffi::c_int as WORD;
    while e < (*ptr_ps_dec).num_env as core::ffi::c_int {
        let mut p_iid_par_prev: *mut WORD16 = 0 as *mut WORD16;
        let mut p_icc_par_prev: *mut WORD16 = 0 as *mut WORD16;
        if e == 0 as core::ffi::c_int {
            p_iid_par_prev = ((*ptr_ps_dec).iid_par_prev).as_mut_ptr();
            p_icc_par_prev = ((*ptr_ps_dec).icc_par_prev).as_mut_ptr();
        } else {
            p_iid_par_prev = ((*ptr_ps_dec)
                .iid_par_table[(e as core::ffi::c_int - 1 as core::ffi::c_int) as usize])
                .as_mut_ptr();
            p_icc_par_prev = ((*ptr_ps_dec)
                .icc_par_table[(e as core::ffi::c_int - 1 as core::ffi::c_int) as usize])
                .as_mut_ptr();
        }
        if (*ptr_ps_dec).enable_iid != 0 {
            if (*ptr_ps_dec).iid_dt[e as usize] != 0 {
                i = 0 as core::ffi::c_int as WORD;
                while i
                    < ixheaacd_num_bands[(*ptr_ps_dec).iid_mode as usize]
                        as core::ffi::c_int
                {
                    temp = ixheaac_add16(
                        *p_iid_par_prev,
                        (*ptr_ps_dec).iid_par_table[e as usize][i as usize],
                    ) as WORD;
                    (*ptr_ps_dec).iid_par_table[e as usize][i as usize] = ixheaacd_clamp(
                        temp as WORD32,
                        ixheaac_negate16(num_iid_levels),
                        num_iid_levels,
                    ) as WORD16;
                    p_iid_par_prev = p_iid_par_prev
                        .offset(iid_mode as core::ffi::c_int as isize);
                    i += 1;
                }
            } else {
                (*ptr_ps_dec)
                    .iid_par_table[e as usize][0 as core::ffi::c_int as usize] = ixheaacd_clamp(
                    (*ptr_ps_dec)
                        .iid_par_table[e as usize][0 as core::ffi::c_int as usize]
                        as WORD32,
                    ixheaac_negate16(num_iid_levels),
                    num_iid_levels,
                ) as WORD16;
                i = 1 as core::ffi::c_int as WORD;
                while i
                    < ixheaacd_num_bands[(*ptr_ps_dec).iid_mode as usize]
                        as core::ffi::c_int
                {
                    temp = ixheaac_add16(
                        (*ptr_ps_dec)
                            .iid_par_table[e
                            as usize][(i as core::ffi::c_int - 1 as core::ffi::c_int)
                            as usize],
                        (*ptr_ps_dec).iid_par_table[e as usize][i as usize],
                    ) as WORD;
                    (*ptr_ps_dec).iid_par_table[e as usize][i as usize] = ixheaacd_clamp(
                        temp as WORD32,
                        ixheaac_negate16(num_iid_levels),
                        num_iid_levels,
                    ) as WORD16;
                    i += 1;
                }
            }
        } else {
            memset(
                ((*ptr_ps_dec).iid_par_table[e as usize]).as_mut_ptr()
                    as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                (::core::mem::size_of::<WORD16>() as size_t)
                    .wrapping_mul(
                        ixheaacd_num_bands[(*ptr_ps_dec).iid_mode as usize] as size_t,
                    ),
            );
        }
        if iid_mode as core::ffi::c_int == 2 as core::ffi::c_int {
            i = (ixheaacd_num_bands[(*ptr_ps_dec).iid_mode as usize] as core::ffi::c_int
                * iid_mode as core::ffi::c_int - 1 as core::ffi::c_int) as WORD;
            while i != 0 as core::ffi::c_int {
                (*ptr_ps_dec).iid_par_table[e as usize][i as usize] = (*ptr_ps_dec)
                    .iid_par_table[e
                    as usize][ixheaac_shr32(i as WORD32, 1 as WORD) as usize];
                i -= 1;
            }
        }
        if (*ptr_ps_dec).enable_icc != 0 {
            if (*ptr_ps_dec).icc_dt[e as usize] != 0 {
                i = 0 as core::ffi::c_int as WORD;
                while i
                    < ixheaacd_num_bands[(*ptr_ps_dec).icc_mode as usize]
                        as core::ffi::c_int
                {
                    temp = ixheaac_add16(
                        *p_icc_par_prev,
                        (*ptr_ps_dec).icc_par_table[e as usize][i as usize],
                    ) as WORD;
                    (*ptr_ps_dec).icc_par_table[e as usize][i as usize] = ixheaacd_clamp(
                        temp as WORD32,
                        0 as WORD16,
                        (NUM_ICC_LEVELS - 1 as core::ffi::c_int) as WORD16,
                    ) as WORD16;
                    p_icc_par_prev = p_icc_par_prev
                        .offset(icc_mode as core::ffi::c_int as isize);
                    i += 1;
                }
            } else {
                (*ptr_ps_dec)
                    .icc_par_table[e as usize][0 as core::ffi::c_int as usize] = ixheaacd_clamp(
                    (*ptr_ps_dec)
                        .icc_par_table[e as usize][0 as core::ffi::c_int as usize]
                        as WORD32,
                    0 as WORD16,
                    (NUM_ICC_LEVELS - 1 as core::ffi::c_int) as WORD16,
                ) as WORD16;
                i = 1 as core::ffi::c_int as WORD;
                while i
                    < ixheaacd_num_bands[(*ptr_ps_dec).icc_mode as usize]
                        as core::ffi::c_int
                {
                    temp = ixheaac_add16(
                        (*ptr_ps_dec)
                            .icc_par_table[e
                            as usize][(i as core::ffi::c_int - 1 as core::ffi::c_int)
                            as usize],
                        (*ptr_ps_dec).icc_par_table[e as usize][i as usize],
                    ) as WORD;
                    (*ptr_ps_dec).icc_par_table[e as usize][i as usize] = ixheaacd_clamp(
                        temp as WORD32,
                        0 as WORD16,
                        (NUM_ICC_LEVELS - 1 as core::ffi::c_int) as WORD16,
                    ) as WORD16;
                    i += 1;
                }
            }
        } else {
            memset(
                ((*ptr_ps_dec).icc_par_table[e as usize]).as_mut_ptr()
                    as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                (::core::mem::size_of::<WORD16>() as size_t)
                    .wrapping_mul(
                        ixheaacd_num_bands[(*ptr_ps_dec).icc_mode as usize] as size_t,
                    ),
            );
        }
        if icc_mode as core::ffi::c_int == 2 as core::ffi::c_int {
            i = (ixheaacd_num_bands[(*ptr_ps_dec).icc_mode as usize] as core::ffi::c_int
                * icc_mode as core::ffi::c_int - 1 as core::ffi::c_int) as WORD;
            while i != 0 as core::ffi::c_int {
                (*ptr_ps_dec).icc_par_table[e as usize][i as usize] = (*ptr_ps_dec)
                    .icc_par_table[e
                    as usize][ixheaac_shr32(i as WORD32, 1 as WORD) as usize];
                i -= 1;
            }
        }
        e += 1;
    }
    if (*ptr_ps_dec).num_env as core::ffi::c_int == 0 as core::ffi::c_int {
        (*ptr_ps_dec).num_env = 1 as WORD16;
        if (*ptr_ps_dec).enable_iid != 0 {
            memcpy(
                ((*ptr_ps_dec).iid_par_table[0 as core::ffi::c_int as usize])
                    .as_mut_ptr() as *mut core::ffi::c_void,
                ((*ptr_ps_dec).iid_par_prev).as_mut_ptr() as *const core::ffi::c_void,
                (::core::mem::size_of::<WORD16>() as size_t)
                    .wrapping_mul(NUM_BANDS_FINE as size_t),
            );
        } else {
            memset(
                ((*ptr_ps_dec).iid_par_table[0 as core::ffi::c_int as usize])
                    .as_mut_ptr() as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                (::core::mem::size_of::<WORD16>() as size_t)
                    .wrapping_mul(NUM_BANDS_FINE as size_t),
            );
        }
        if (*ptr_ps_dec).enable_icc != 0 {
            memcpy(
                ((*ptr_ps_dec).icc_par_table[0 as core::ffi::c_int as usize])
                    .as_mut_ptr() as *mut core::ffi::c_void,
                ((*ptr_ps_dec).icc_par_prev).as_mut_ptr() as *const core::ffi::c_void,
                (::core::mem::size_of::<WORD16>() as size_t)
                    .wrapping_mul(NUM_BANDS_FINE as size_t),
            );
        } else {
            memset(
                ((*ptr_ps_dec).icc_par_table[0 as core::ffi::c_int as usize])
                    .as_mut_ptr() as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                (::core::mem::size_of::<WORD16>() as size_t)
                    .wrapping_mul(NUM_BANDS_FINE as size_t),
            );
        }
    }
    memcpy(
        ((*ptr_ps_dec).iid_par_prev).as_mut_ptr() as *mut core::ffi::c_void,
        ((*ptr_ps_dec)
            .iid_par_table[((*ptr_ps_dec).num_env as core::ffi::c_int
            - 1 as core::ffi::c_int) as usize])
            .as_mut_ptr() as *const core::ffi::c_void,
        (::core::mem::size_of::<WORD16>() as size_t)
            .wrapping_mul(NUM_BANDS_FINE as size_t),
    );
    memcpy(
        ((*ptr_ps_dec).icc_par_prev).as_mut_ptr() as *mut core::ffi::c_void,
        ((*ptr_ps_dec)
            .icc_par_table[((*ptr_ps_dec).num_env as core::ffi::c_int
            - 1 as core::ffi::c_int) as usize])
            .as_mut_ptr() as *const core::ffi::c_void,
        (::core::mem::size_of::<WORD16>() as size_t)
            .wrapping_mul(NUM_BANDS_FINE as size_t),
    );
    (*ptr_ps_dec).ps_data_present = 0 as core::ffi::c_int as FLAG;
    if (*ptr_ps_dec).frame_class == 0 as core::ffi::c_int {
        let mut env_count: WORD = 0;
        let mut shift: WORD16 = 0 as WORD16;
        match (*ptr_ps_dec).num_env as core::ffi::c_int {
            1 => {
                shift = 0 as WORD16;
            }
            2 => {
                shift = 1 as WORD16;
            }
            4 => {
                shift = 2 as WORD16;
            }
            _ => {}
        }
        (*ptr_ps_dec).border_position[0 as core::ffi::c_int as usize] = 0 as WORD16;
        env_count = 0 as core::ffi::c_int as WORD;
        e = 1 as core::ffi::c_int as WORD;
        while e < (*ptr_ps_dec).num_env as core::ffi::c_int {
            env_count = (env_count as WORD32 + max_num_columns) as WORD;
            (*ptr_ps_dec).border_position[e as usize] = (env_count
                >> shift as core::ffi::c_int) as WORD16;
            e += 1;
        }
        (*ptr_ps_dec).border_position[(*ptr_ps_dec).num_env as usize] = max_num_columns
            as WORD16;
    } else {
        (*ptr_ps_dec).border_position[0 as core::ffi::c_int as usize] = 0 as WORD16;
        if ((*ptr_ps_dec).border_position[(*ptr_ps_dec).num_env as usize]
            as core::ffi::c_int) < max_num_columns
        {
            (*ptr_ps_dec).num_env += 1;
            (*ptr_ps_dec).border_position[(*ptr_ps_dec).num_env as usize] = max_num_columns
                as WORD16;
            memcpy(
                ((*ptr_ps_dec)
                    .iid_par_table[((*ptr_ps_dec).num_env as core::ffi::c_int
                    - 1 as core::ffi::c_int) as usize])
                    .as_mut_ptr() as *mut core::ffi::c_void,
                ((*ptr_ps_dec)
                    .iid_par_table[((*ptr_ps_dec).num_env as core::ffi::c_int
                    - 2 as core::ffi::c_int) as usize])
                    .as_mut_ptr() as *const core::ffi::c_void,
                (::core::mem::size_of::<WORD16>() as size_t)
                    .wrapping_mul(NUM_BANDS_FINE as size_t),
            );
            memcpy(
                ((*ptr_ps_dec)
                    .icc_par_table[((*ptr_ps_dec).num_env as core::ffi::c_int
                    - 1 as core::ffi::c_int) as usize])
                    .as_mut_ptr() as *mut core::ffi::c_void,
                ((*ptr_ps_dec)
                    .icc_par_table[((*ptr_ps_dec).num_env as core::ffi::c_int
                    - 2 as core::ffi::c_int) as usize])
                    .as_mut_ptr() as *const core::ffi::c_void,
                (::core::mem::size_of::<WORD16>() as size_t)
                    .wrapping_mul(NUM_BANDS_FINE as size_t),
            );
        }
        e = 1 as core::ffi::c_int as WORD;
        while e < (*ptr_ps_dec).num_env as core::ffi::c_int {
            let mut threshold: WORD = 0;
            threshold = max_num_columns as WORD - ((*ptr_ps_dec).num_env as WORD - e);
            if (*ptr_ps_dec).border_position[e as usize] as core::ffi::c_int > threshold
            {
                (*ptr_ps_dec).border_position[e as usize] = threshold as WORD16;
            } else {
                threshold = ((*ptr_ps_dec)
                    .border_position[(e as core::ffi::c_int - 1 as core::ffi::c_int)
                    as usize] as core::ffi::c_int + 1 as core::ffi::c_int) as WORD;
                if ((*ptr_ps_dec).border_position[e as usize] as core::ffi::c_int)
                    < threshold
                {
                    (*ptr_ps_dec).border_position[e as usize] = threshold as WORD16;
                }
            }
            e += 1;
        }
    }
    e = 0 as core::ffi::c_int as WORD;
    while e < (*ptr_ps_dec).num_env as core::ffi::c_int {
        if (*ptr_ps_dec).iid_mode as core::ffi::c_int == 2 as core::ffi::c_int {
            ixheaacd_map_34_params_to_20(
                ((*ptr_ps_dec).iid_par_table[e as usize]).as_mut_ptr(),
            );
        }
        if (*ptr_ps_dec).icc_mode as core::ffi::c_int == 2 as core::ffi::c_int {
            ixheaacd_map_34_params_to_20(
                ((*ptr_ps_dec).icc_par_table[e as usize]).as_mut_ptr(),
            );
        }
        e += 1;
    }
}
