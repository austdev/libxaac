extern "C" {
    fn sqrt(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn ixheaacd_rotated_gosset_mtx_dec(
        qn: WORD32,
        code_book_idx: WORD32,
        kv: *mut WORD32,
        y: *mut WORD32,
    ) -> VOID;
}
pub type WORD32 = core::ffi::c_int;
pub type FLOAT32 = core::ffi::c_float;
pub type VOID = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_td_frame_data_struct {
    pub acelp_core_mode: WORD32,
    pub mod_0: [WORD32; 4],
    pub fac: [WORD32; 512],
    pub fac_data: [WORD32; 129],
    pub mean_energy: [WORD32; 4],
    pub acb_index: [WORD32; 16],
    pub noise_factor: [WORD32; 4],
    pub global_gain: [WORD32; 4],
    pub arith_reset_flag: WORD32,
    pub x_tcx_invquant: [WORD32; 1024],
    pub tcx_lg: [WORD32; 16],
    pub ltp_filtering_flag: [WORD32; 16],
    pub icb_index: [[WORD32; 8]; 16],
    pub gains: [WORD32; 16],
    pub mode_lpc: [WORD32; 4],
    pub lpc_first_approx_idx: [WORD32; 110],
    pub lsp_coeff: [[FLOAT32; 16]; 5],
    pub lsf_adaptive_mean_cand: [FLOAT32; 16],
    pub lsf_adaptive_mean: [FLOAT32; 16],
    pub lpc4_lsf: [FLOAT32; 16],
}
pub const ORDER: core::ffi::c_int = 16 as core::ffi::c_int;
pub const LSF_GAP: core::ffi::c_float = 50.0f32;
pub const FREQ_MAX: core::ffi::c_float = 6400.0f32;
pub const FREQ_DIV: core::ffi::c_float = 400.0f32;
static mut factor_table: [FLOAT32; 4] = [60.0f32, 65.0f32, 64.0f32, 63.0f32];
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_lsf_weight_2st_flt(
    mut lsfq: *mut FLOAT32,
    mut w: *mut FLOAT32,
    mut mode: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut d: [FLOAT32; 17] = [0.; 17];
    d[0 as core::ffi::c_int as usize] = *lsfq.offset(0 as core::ffi::c_int as isize);
    d[ORDER as usize] = FREQ_MAX
        - *lsfq.offset((ORDER - 1 as core::ffi::c_int) as isize);
    i = 1 as core::ffi::c_int as WORD32;
    while i < ORDER {
        d[i as usize] = *lsfq.offset(i as isize)
            - *lsfq.offset((i as core::ffi::c_int - 1 as core::ffi::c_int) as isize);
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < ORDER {
        *w.offset(i as isize) = (factor_table[mode as usize] as core::ffi::c_double
            * sqrt(
                (d[i as usize]
                    * d[(i as core::ffi::c_int + 1 as core::ffi::c_int) as usize])
                    as core::ffi::c_double,
            ) / FREQ_DIV as core::ffi::c_double) as FLOAT32;
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_decoding_avq_tool(
    mut read_arr: *mut WORD32,
    mut nvecq: *mut WORD32,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut qn: WORD32 = 0;
    let mut kv: [WORD32; 8] = [0 as core::ffi::c_int; 8];
    let mut code_book_idx: WORD32 = 0;
    let mut ptr_kv: *mut WORD32 = &mut *kv
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
    let mut position: WORD32 = 2 as WORD32;
    k = 0 as core::ffi::c_int as WORD32;
    while k < 2 as core::ffi::c_int {
        qn = *read_arr.offset(k as isize);
        if qn > 0 as core::ffi::c_int {
            let fresh1 = position;
            position = position + 1;
            code_book_idx = *read_arr.offset(fresh1 as isize);
            ptr_kv = &mut *read_arr.offset(position as isize) as *mut WORD32;
            position += 8 as core::ffi::c_int;
        } else {
            code_book_idx = 0 as core::ffi::c_int as WORD32;
            i = 0 as core::ffi::c_int as WORD32;
            while i < 8 as core::ffi::c_int {
                ptr_kv = &mut *kv.as_mut_ptr().offset(0 as core::ffi::c_int as isize)
                    as *mut WORD32;
                i += 1;
            }
        }
        ixheaacd_rotated_gosset_mtx_dec(
            qn,
            code_book_idx,
            ptr_kv as *mut WORD32,
            &mut *nvecq.offset((k as core::ffi::c_int * 8 as core::ffi::c_int) as isize),
        );
        k += 1;
    }
    return position;
}
unsafe extern "C" fn ixheaacd_avq_first_approx_abs(
    mut lsf: *mut FLOAT32,
    mut indx: *mut WORD32,
) -> WORD32 {
    let mut i: WORD32 = 0;
    extern "C" {
        static ixheaacd_dico_lsf_abs_8b_flt: [FLOAT32; 0];
    }
    extern "C" {
        static ixheaacd_weight_table_avq: [FLOAT32; 0];
    }
    let mut position: WORD32 = 0 as WORD32;
    let mut avq: [WORD32; 16] = [0; 16];
    let mut lsf_min: FLOAT32 = 0.;
    let mut ptr_w: *const FLOAT32 = 0 as *const FLOAT32;
    ptr_w = &*ixheaacd_weight_table_avq
        .as_ptr()
        .offset((*indx.offset(0 as core::ffi::c_int as isize) * ORDER) as isize)
        as *const FLOAT32;
    position += 1;
    i = 0 as core::ffi::c_int as WORD32;
    while i < ORDER {
        *lsf.offset(i as isize) = *ixheaacd_dico_lsf_abs_8b_flt
            .as_ptr()
            .offset((*indx.offset(0 as core::ffi::c_int as isize) * ORDER + i) as isize);
        i += 1;
    }
    position
        += ixheaacd_decoding_avq_tool(
            &mut *indx.offset(position as isize),
            avq.as_mut_ptr(),
        );
    lsf_min = LSF_GAP as FLOAT32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < ORDER {
        *lsf.offset(i as isize)
            += *ptr_w.offset(i as isize) * avq[i as usize] as FLOAT32;
        if *lsf.offset(i as isize) < lsf_min {
            *lsf.offset(i as isize) = lsf_min;
        }
        lsf_min = (*lsf.offset(i as isize) + LSF_GAP) as FLOAT32;
        i += 1;
    }
    lsf_min = (FREQ_MAX - LSF_GAP) as FLOAT32;
    i = (ORDER - 1 as core::ffi::c_int) as WORD32;
    while i >= 0 as core::ffi::c_int {
        if *lsf.offset(i as isize) > lsf_min {
            *lsf.offset(i as isize) = lsf_min;
        }
        lsf_min = (*lsf.offset(i as isize) - LSF_GAP) as FLOAT32;
        i -= 1;
    }
    return position;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_avq_first_approx_rel(
    mut lsf: *mut FLOAT32,
    mut indx: *mut WORD32,
    mut mode: WORD32,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut w: [FLOAT32; 16] = [0.; 16];
    let mut avq: [WORD32; 16] = [0; 16];
    let mut position: WORD32 = 0 as WORD32;
    let mut lsf_min: FLOAT32 = 0.;
    ixheaacd_lsf_weight_2st_flt(lsf, w.as_mut_ptr(), mode);
    position = ixheaacd_decoding_avq_tool(indx, avq.as_mut_ptr());
    lsf_min = LSF_GAP as FLOAT32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < ORDER {
        *lsf.offset(i as isize) += w[i as usize] * avq[i as usize] as FLOAT32;
        if *lsf.offset(i as isize) < lsf_min {
            *lsf.offset(i as isize) = lsf_min;
        }
        lsf_min = (*lsf.offset(i as isize) + LSF_GAP) as FLOAT32;
        i += 1;
    }
    return position;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_alg_vec_dequant(
    mut pstr_td_frame_data: *mut ia_td_frame_data_struct,
    mut first_lpd_flag: WORD32,
    mut lsf: *mut FLOAT32,
    mut mod_0: *mut WORD32,
    mut ec_flag: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut lpc_index: *mut WORD32 = 0 as *mut WORD32;
    let mut mode_lpc: WORD32 = 0;
    let mut pos: WORD32 = 0 as WORD32;
    let mut lpc_present: [WORD32; 5] = [
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ];
    lpc_index = ((*pstr_td_frame_data).lpc_first_approx_idx).as_mut_ptr();
    lpc_present[4 as core::ffi::c_int as usize] = 1 as core::ffi::c_int as WORD32;
    pos = ixheaacd_avq_first_approx_abs(
        &mut *lsf.offset((4 as core::ffi::c_int * ORDER) as isize),
        &mut *lpc_index.offset(0 as core::ffi::c_int as isize),
    );
    lpc_index = lpc_index.offset(pos as isize);
    if first_lpd_flag != 0 {
        mode_lpc = *lpc_index.offset(0 as core::ffi::c_int as isize);
        lpc_index = lpc_index.offset(1);
        if mode_lpc == 0 as core::ffi::c_int {
            pos = ixheaacd_avq_first_approx_abs(
                &mut *lsf.offset(0 as core::ffi::c_int as isize),
                &mut *lpc_index.offset(0 as core::ffi::c_int as isize),
            );
        } else if mode_lpc == 1 as core::ffi::c_int {
            i = 0 as core::ffi::c_int as WORD32;
            while i < ORDER {
                *lsf.offset(i as isize) = *lsf
                    .offset((4 as WORD32 * ORDER + i) as isize);
                i += 1;
            }
            pos = ixheaacd_avq_first_approx_rel(
                &mut *lsf.offset(0 as core::ffi::c_int as isize),
                &mut *lpc_index.offset(0 as core::ffi::c_int as isize),
                3 as WORD32,
            );
        }
        lpc_index = lpc_index.offset(pos as isize);
    }
    lpc_present[0 as core::ffi::c_int as usize] = 1 as core::ffi::c_int as WORD32;
    if *mod_0.offset(0 as core::ffi::c_int as isize) < 3 as core::ffi::c_int {
        mode_lpc = *lpc_index.offset(0 as core::ffi::c_int as isize);
        lpc_index = lpc_index.offset(1);
        lpc_present[2 as core::ffi::c_int as usize] = 1 as core::ffi::c_int as WORD32;
        if mode_lpc == 0 as core::ffi::c_int {
            pos = ixheaacd_avq_first_approx_abs(
                &mut *lsf.offset((2 as core::ffi::c_int * ORDER) as isize),
                &mut *lpc_index.offset(0 as core::ffi::c_int as isize),
            );
        } else if mode_lpc == 1 as core::ffi::c_int {
            i = 0 as core::ffi::c_int as WORD32;
            while i < ORDER {
                *lsf.offset((2 as WORD32 * ORDER + i) as isize) = *lsf
                    .offset((4 as WORD32 * ORDER + i) as isize);
                i += 1;
            }
            pos = ixheaacd_avq_first_approx_rel(
                &mut *lsf.offset((2 as core::ffi::c_int * ORDER) as isize),
                &mut *lpc_index.offset(0 as core::ffi::c_int as isize),
                3 as WORD32,
            );
        }
        lpc_index = lpc_index.offset(pos as isize);
    }
    if *mod_0.offset(0 as core::ffi::c_int as isize) < 2 as core::ffi::c_int {
        mode_lpc = *lpc_index.offset(0 as core::ffi::c_int as isize);
        lpc_index = lpc_index.offset(1);
        lpc_present[1 as core::ffi::c_int as usize] = 1 as core::ffi::c_int as WORD32;
        if mode_lpc == 1 as core::ffi::c_int {
            i = 0 as core::ffi::c_int as WORD32;
            while i < ORDER {
                *lsf.offset((ORDER + i) as isize) = 0.5f32
                    * (*lsf.offset(i as isize)
                        + *lsf.offset((2 as WORD32 * ORDER + i) as isize));
                i += 1;
            }
        } else {
            if mode_lpc == 0 as core::ffi::c_int {
                pos = ixheaacd_avq_first_approx_abs(
                    &mut *lsf.offset(ORDER as isize),
                    &mut *lpc_index.offset(0 as core::ffi::c_int as isize),
                );
            } else if mode_lpc == 2 as core::ffi::c_int {
                i = 0 as core::ffi::c_int as WORD32;
                while i < ORDER {
                    *lsf.offset((ORDER + i) as isize) = *lsf
                        .offset((2 as WORD32 * ORDER + i) as isize);
                    i += 1;
                }
                pos = ixheaacd_avq_first_approx_rel(
                    &mut *lsf.offset(ORDER as isize),
                    &mut *lpc_index.offset(0 as core::ffi::c_int as isize),
                    2 as WORD32,
                );
            }
            lpc_index = lpc_index.offset(pos as isize);
        }
    }
    if *mod_0.offset(2 as core::ffi::c_int as isize) < 2 as core::ffi::c_int {
        mode_lpc = *lpc_index.offset(0 as core::ffi::c_int as isize);
        lpc_index = lpc_index.offset(1);
        lpc_present[3 as core::ffi::c_int as usize] = 1 as core::ffi::c_int as WORD32;
        if mode_lpc == 0 as core::ffi::c_int {
            pos = ixheaacd_avq_first_approx_abs(
                &mut *lsf.offset((3 as core::ffi::c_int * ORDER) as isize),
                &mut *lpc_index.offset(0 as core::ffi::c_int as isize),
            );
        } else if mode_lpc == 1 as core::ffi::c_int {
            i = 0 as core::ffi::c_int as WORD32;
            while i < ORDER {
                *lsf.offset((3 as WORD32 * ORDER + i) as isize) = 0.5f32
                    * (*lsf.offset((2 as WORD32 * ORDER + i) as isize)
                        + *lsf.offset((4 as WORD32 * ORDER + i) as isize));
                i += 1;
            }
            pos = ixheaacd_avq_first_approx_rel(
                &mut *lsf.offset((3 as core::ffi::c_int * ORDER) as isize),
                &mut *lpc_index.offset(0 as core::ffi::c_int as isize),
                1 as WORD32,
            );
        } else if mode_lpc == 2 as core::ffi::c_int {
            i = 0 as core::ffi::c_int as WORD32;
            while i < ORDER {
                *lsf.offset((3 as WORD32 * ORDER + i) as isize) = *lsf
                    .offset((2 as WORD32 * ORDER + i) as isize);
                i += 1;
            }
            pos = ixheaacd_avq_first_approx_rel(
                &mut *lsf.offset((3 as core::ffi::c_int * ORDER) as isize),
                &mut *lpc_index.offset(0 as core::ffi::c_int as isize),
                2 as WORD32,
            );
        } else if mode_lpc == 3 as core::ffi::c_int {
            i = 0 as core::ffi::c_int as WORD32;
            while i < ORDER {
                *lsf.offset((3 as WORD32 * ORDER + i) as isize) = *lsf
                    .offset((4 as WORD32 * ORDER + i) as isize);
                i += 1;
            }
            pos = ixheaacd_avq_first_approx_rel(
                &mut *lsf.offset((3 as core::ffi::c_int * ORDER) as isize),
                &mut *lpc_index.offset(0 as core::ffi::c_int as isize),
                2 as WORD32,
            );
        }
        lpc_index = lpc_index.offset(pos as isize);
    }
    if ec_flag != 0 {
        let mut last: WORD32 = 0;
        let mut k: WORD32 = 0;
        let mut num_lpc: WORD32 = 0 as WORD32;
        let mut num_div: WORD32 = 4 as WORD32;
        let mut div_fac: FLOAT32 = 0.;
        let mut lsf4: *mut FLOAT32 = &mut *lsf
            .offset((4 as core::ffi::c_int * ORDER) as isize) as *mut FLOAT32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < ORDER {
            (*pstr_td_frame_data).lpc4_lsf[i as usize] = *lsf4.offset(i as isize);
            i += 1;
        }
        i = num_div;
        loop {
            let fresh0 = i;
            i = i - 1;
            num_lpc += lpc_present[fresh0 as usize];
            if !(i >= 0 as core::ffi::c_int && num_lpc < 3 as core::ffi::c_int) {
                break;
            }
        }
        last = i;
        match num_lpc {
            3 => {
                div_fac = (1.0f32 / 3.0f32) as FLOAT32;
            }
            2 => {
                div_fac = (1.0f32 / 2.0f32) as FLOAT32;
            }
            _ => {
                div_fac = 1.0f32 as FLOAT32;
            }
        }
        k = 0 as core::ffi::c_int as WORD32;
        while k < ORDER {
            let mut temp: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
            i = 4 as core::ffi::c_int as WORD32;
            while i > last {
                if lpc_present[i as usize] != 0 {
                    temp = temp + *lsf.offset((i * ORDER + k) as isize) * div_fac;
                }
                i -= 1;
            }
            (*pstr_td_frame_data).lsf_adaptive_mean_cand[k as usize] = temp;
            k += 1;
        }
    }
}
