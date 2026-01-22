extern "C" {
    fn log10(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn pow(__x: core::ffi::c_double, __y: core::ffi::c_double) -> core::ffi::c_double;
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
    static ixheaacd_pvc_smoothing_wind_tab_ns4: [FLOAT32; 4];
    static ixheaacd_pvc_smoothing_wind_tab_ns16: [FLOAT32; 16];
    static ixheaacd_pvc_smoothing_wind_tab_ns3: [FLOAT32; 3];
    static ixheaacd_pvc_smoothing_wind_tab_ns12: [FLOAT32; 12];
    static ixheaacd_pred_coeff_pvc_id_boundaries_1: [UWORD8; 2];
    static ixheaacd_q_factor_table_mode_1: [FLOAT32; 4];
    static ixheaacd_pred_coeff_table_1_mode_1: [[[WORD8; 8]; 3]; 3];
    static ixheaacd_pred_coeff_table_2_mode_1: [[WORD8; 8]; 128];
    static ixheaacd_pred_coeff_pvc_id_boundaries_2: [UWORD8; 2];
    static ixheaacd_q_factor_table_mode_2: [FLOAT32; 4];
    static ixheaacd_pred_coeff_table_1_mode_2: [[[WORD8; 6]; 3]; 3];
    static ixheaacd_pred_coeff_table_2_mode_2: [[WORD8; 6]; 128];
}
pub type size_t = usize;
pub type WORD8 = core::ffi::c_schar;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD16 = core::ffi::c_short;
pub type UWORD16 = core::ffi::c_ushort;
pub type WORD32 = core::ffi::c_int;
pub type FLOAT32 = core::ffi::c_float;
pub type VOID = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_pvc_data_struct {
    pub pvc_mode: UWORD8,
    pub div_mode: UWORD8,
    pub ns_mode: UWORD8,
    pub num_time_slots: UWORD8,
    pub nb_high: UWORD8,
    pub nb_high_per_grp: UWORD8,
    pub prev_pvc_flg: UWORD8,
    pub prev_pvc_mode: UWORD8,
    pub pvc_rate: UWORD8,
    pub prev_pvc_rate: UWORD8,
    pub dummy: WORD16,
    pub pvc_id: [UWORD16; 16],
    pub prev_pvc_id: UWORD16,
    pub p_pred_coeff_tab_1: *mut WORD8,
    pub p_pred_coeff_tab_2: *mut WORD8,
    pub p_pvc_id_boundary: *mut UWORD8,
    pub prev_first_bnd_idx: WORD16,
    pub prev_sbr_mode: WORD32,
    pub p_smth_wind_coeff: *mut FLOAT32,
    pub p_q_fac: *mut FLOAT32,
    pub esg: [[FLOAT32; 3]; 31],
    pub smooth_esg_arr: [FLOAT32; 48],
    pub sbr_range_esg_arr: [FLOAT32; 128],
}
pub const SBR_NUM_QMF_BANDS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const SBR_NUM_QMF_BANDS_2: core::ffi::c_int = 32 as core::ffi::c_int;
pub const PVC_NUM_TIME_SLOTS: core::ffi::c_int = 16 as core::ffi::c_int;
pub const PVC_ESG_MIN_VAL: core::ffi::c_float = 0.1f32;
pub const PVC_10LOG10_ESG_MIN_VAL: core::ffi::c_float = -10.0f32;
pub const PVC_NB_HIGH_MODE1: core::ffi::c_int = 8 as core::ffi::c_int;
pub const PVC_NB_HIGH_MODE2: core::ffi::c_int = 6 as core::ffi::c_int;
pub const PVC_NB_LOW: core::ffi::c_int = 3 as core::ffi::c_int;
unsafe extern "C" fn ixheaacd_pvc_sb_parsing(
    mut ptr_pvc_data: *mut ia_pvc_data_struct,
    mut first_bnd_idx: WORD16,
    mut p_qmfh: *mut FLOAT32,
) -> VOID {
    let mut ksg: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut start_band: WORD32 = 0;
    let mut end_band: WORD32 = 0;
    let mut time_slot: WORD32 = 0;
    let mut p_sbr_range_esg: *mut FLOAT32 = &mut *((*ptr_pvc_data).sbr_range_esg_arr)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
    time_slot = 0 as core::ffi::c_int as WORD32;
    while time_slot < PVC_NUM_TIME_SLOTS {
        start_band = first_bnd_idx as WORD32;
        end_band = (start_band as core::ffi::c_int
            + (*ptr_pvc_data).nb_high_per_grp as core::ffi::c_int
            - 1 as core::ffi::c_int) as WORD32;
        ksg = 0 as core::ffi::c_int as WORD32;
        while ksg < (*ptr_pvc_data).nb_high as core::ffi::c_int {
            k = start_band;
            while k <= end_band {
                *p_qmfh.offset(k as isize) = pow(
                    10.0f64,
                    *p_sbr_range_esg.offset(ksg as isize) as core::ffi::c_double
                        / 10.0f64,
                ) as FLOAT32;
                k += 1;
            }
            start_band += (*ptr_pvc_data).nb_high_per_grp as core::ffi::c_int;
            if ksg >= (*ptr_pvc_data).nb_high as core::ffi::c_int - 2 as core::ffi::c_int
            {
                end_band = (SBR_NUM_QMF_BANDS - 1 as core::ffi::c_int) as WORD32;
            } else {
                end_band = (start_band as core::ffi::c_int
                    + (*ptr_pvc_data).nb_high_per_grp as core::ffi::c_int
                    - 1 as core::ffi::c_int) as WORD32;
                if end_band >= SBR_NUM_QMF_BANDS - 1 as core::ffi::c_int {
                    end_band = (SBR_NUM_QMF_BANDS - 1 as core::ffi::c_int) as WORD32;
                }
            }
            ksg += 1;
        }
        p_sbr_range_esg = p_sbr_range_esg.offset(8 as core::ffi::c_int as isize);
        p_qmfh = p_qmfh.offset(SBR_NUM_QMF_BANDS as isize);
        time_slot += 1;
    }
}
unsafe extern "C" fn ixheaacd_pvc_qmf_grouping(
    mut ptr_pvc_data: *mut ia_pvc_data_struct,
    mut first_bnd_idx: WORD16,
    mut p_qmf_ener: *mut FLOAT32,
    mut first_pvc_timeslot: WORD32,
) -> VOID {
    let mut ksg: WORD32 = 0;
    let mut time_slot: WORD32 = 0;
    let mut ib: WORD32 = 0;
    let mut lbw: WORD32 = 0;
    let mut start_band: WORD32 = 0;
    let mut end_band: WORD32 = 0;
    let mut esg: FLOAT32 = 0.;
    let mut p_esg: *mut FLOAT32 = ((*ptr_pvc_data).esg).as_mut_ptr() as *mut FLOAT32;
    lbw = (8 as core::ffi::c_int / (*ptr_pvc_data).pvc_rate as core::ffi::c_int)
        as WORD32;
    time_slot = 0 as core::ffi::c_int as WORD32;
    while time_slot < PVC_NUM_TIME_SLOTS {
        ksg = 0 as core::ffi::c_int as WORD32;
        while ksg < PVC_NB_LOW {
            start_band = first_bnd_idx as WORD32 - lbw * PVC_NB_LOW + lbw * ksg;
            end_band = (start_band as core::ffi::c_int + lbw as core::ffi::c_int
                - 1 as core::ffi::c_int) as WORD32;
            if start_band >= 0 as core::ffi::c_int {
                esg = 0.0f32 as FLOAT32;
                ib = start_band;
                while ib <= end_band {
                    esg += *p_qmf_ener.offset(ib as isize);
                    ib += 1;
                }
                esg = esg / lbw as FLOAT32;
            } else {
                esg = PVC_ESG_MIN_VAL as FLOAT32;
            }
            if esg > PVC_ESG_MIN_VAL {
                *p_esg
                    .offset(
                        ((time_slot + 16 as WORD32 - 1 as WORD32) * 3 as WORD32 + ksg)
                            as isize,
                    ) = 10 as core::ffi::c_int as FLOAT32
                    * log10(esg as core::ffi::c_double) as FLOAT32;
            } else {
                *p_esg
                    .offset(
                        ((time_slot + 16 as WORD32 - 1 as WORD32) * 3 as WORD32 + ksg)
                            as isize,
                    ) = PVC_10LOG10_ESG_MIN_VAL as FLOAT32;
            }
            ksg += 1;
        }
        p_qmf_ener = p_qmf_ener.offset(SBR_NUM_QMF_BANDS_2 as isize);
        time_slot += 1;
    }
    if (*ptr_pvc_data).prev_pvc_flg as core::ffi::c_int == 0 as core::ffi::c_int
        || first_bnd_idx as core::ffi::c_int
            * (*ptr_pvc_data).pvc_rate as core::ffi::c_int
            != (*ptr_pvc_data).prev_first_bnd_idx as core::ffi::c_int
                * (*ptr_pvc_data).prev_pvc_rate as core::ffi::c_int
    {
        time_slot = 0 as core::ffi::c_int as WORD32;
        while time_slot < 16 as WORD32 - 1 as WORD32 + first_pvc_timeslot {
            ksg = 0 as core::ffi::c_int as WORD32;
            while ksg < PVC_NB_LOW {
                *p_esg.offset((time_slot * 3 as WORD32 + ksg) as isize) = *p_esg
                    .offset(
                        ((16 as WORD32 - 1 as WORD32 + first_pvc_timeslot) * 3 as WORD32
                            + ksg) as isize,
                    );
                ksg += 1;
            }
            time_slot += 1;
        }
    }
}
unsafe extern "C" fn ixheaacd_pvc_time_smoothing(
    mut ptr_pvc_data: *mut ia_pvc_data_struct,
) -> VOID {
    let mut time_slot: WORD32 = 0;
    let mut p_smooth_esg: *mut FLOAT32 = &mut *((*ptr_pvc_data).smooth_esg_arr)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
    time_slot = 0 as core::ffi::c_int as WORD32;
    while time_slot < PVC_NUM_TIME_SLOTS {
        let mut ksg: WORD32 = 0;
        let mut time_slot_idx: WORD32 = 0;
        let mut p_esg: *mut FLOAT32 = &mut *(*((*ptr_pvc_data).esg)
            .as_mut_ptr()
            .offset(
                (time_slot as core::ffi::c_int + 16 as core::ffi::c_int
                    - 1 as core::ffi::c_int) as isize,
            ))
            .as_mut_ptr()
            .offset(2 as core::ffi::c_int as isize) as *mut FLOAT32;
        let mut p_smth_wind_coeff: *mut FLOAT32 = &mut *((*ptr_pvc_data)
            .p_smth_wind_coeff)
            .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
        memset(
            p_smooth_esg as *mut core::ffi::c_void,
            0.0f32 as core::ffi::c_int,
            (::core::mem::size_of::<FLOAT32>() as size_t)
                .wrapping_mul(PVC_NB_LOW as size_t),
        );
        time_slot_idx = 0 as core::ffi::c_int as WORD32;
        while time_slot_idx < (*ptr_pvc_data).num_time_slots as core::ffi::c_int {
            ksg = (PVC_NB_LOW - 1 as core::ffi::c_int) as WORD32;
            while ksg >= 0 as core::ffi::c_int {
                *p_smooth_esg.offset(ksg as isize) += *p_esg * *p_smth_wind_coeff;
                p_esg = p_esg.offset(-1);
                ksg -= 1;
            }
            p_smth_wind_coeff = p_smth_wind_coeff.offset(1);
            time_slot_idx += 1;
        }
        p_smooth_esg = p_smooth_esg.offset(3 as core::ffi::c_int as isize);
        time_slot += 1;
    }
}
unsafe extern "C" fn ixheaacd_pvc_pred_env_sf(
    mut ptr_pvc_data: *mut ia_pvc_data_struct,
) -> VOID {
    let mut ksg: WORD32 = 0;
    let mut kb: WORD32 = 0;
    let mut tab_1_index: WORD32 = 0;
    let mut tab_2_index: WORD32 = 0;
    let mut time_slot: WORD32 = 0;
    let mut pred_tab_1: *mut WORD8 = 0 as *mut WORD8;
    let mut pred_tab_2: *mut WORD8 = 0 as *mut WORD8;
    let mut temp: FLOAT32 = 0.;
    let mut p_smooth_esg: *mut FLOAT32 = &mut *((*ptr_pvc_data).smooth_esg_arr)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
    let mut p_sbr_range_esg: *mut FLOAT32 = &mut *((*ptr_pvc_data).sbr_range_esg_arr)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
    time_slot = 0 as core::ffi::c_int as WORD32;
    while time_slot < PVC_NUM_TIME_SLOTS {
        tab_2_index = (*ptr_pvc_data).pvc_id[time_slot as usize] as WORD32;
        if tab_2_index
            < *((*ptr_pvc_data).p_pvc_id_boundary).offset(0 as core::ffi::c_int as isize)
                as core::ffi::c_int
        {
            tab_1_index = 0 as core::ffi::c_int as WORD32;
        } else if tab_2_index
            < *((*ptr_pvc_data).p_pvc_id_boundary).offset(1 as core::ffi::c_int as isize)
                as core::ffi::c_int
        {
            tab_1_index = 1 as core::ffi::c_int as WORD32;
        } else {
            tab_1_index = 2 as core::ffi::c_int as WORD32;
        }
        pred_tab_1 = &mut *((*ptr_pvc_data).p_pred_coeff_tab_1)
            .offset(
                (tab_1_index as core::ffi::c_int * PVC_NB_LOW
                    * (*ptr_pvc_data).nb_high as core::ffi::c_int) as isize,
            ) as *mut WORD8;
        pred_tab_2 = &mut *((*ptr_pvc_data).p_pred_coeff_tab_2)
            .offset(
                (tab_2_index as core::ffi::c_int
                    * (*ptr_pvc_data).nb_high as core::ffi::c_int) as isize,
            ) as *mut WORD8;
        ksg = 0 as core::ffi::c_int as WORD32;
        while ksg < (*ptr_pvc_data).nb_high as core::ffi::c_int {
            let fresh0 = pred_tab_2;
            pred_tab_2 = pred_tab_2.offset(1);
            temp = *fresh0 as FLOAT32
                * *((*ptr_pvc_data).p_q_fac).offset(PVC_NB_LOW as isize);
            *p_sbr_range_esg.offset(ksg as isize) = temp;
            ksg += 1;
        }
        kb = 0 as core::ffi::c_int as WORD32;
        while kb < PVC_NB_LOW {
            ksg = 0 as core::ffi::c_int as WORD32;
            while ksg < (*ptr_pvc_data).nb_high as core::ffi::c_int {
                let fresh1 = pred_tab_1;
                pred_tab_1 = pred_tab_1.offset(1);
                temp = *fresh1 as FLOAT32
                    * *((*ptr_pvc_data).p_q_fac).offset(kb as isize);
                *p_sbr_range_esg.offset(ksg as isize)
                    += temp * *p_smooth_esg.offset(kb as isize);
                ksg += 1;
            }
            kb += 1;
        }
        p_smooth_esg = p_smooth_esg.offset(3 as core::ffi::c_int as isize);
        p_sbr_range_esg = p_sbr_range_esg.offset(8 as core::ffi::c_int as isize);
        time_slot += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_pvc_process(
    mut ptr_pvc_data: *mut ia_pvc_data_struct,
    mut first_bnd_idx: WORD16,
    mut first_pvc_timeslot: WORD32,
    mut p_qmf_ener: *mut FLOAT32,
    mut p_qmfh: *mut FLOAT32,
) -> WORD32 {
    match (*ptr_pvc_data).pvc_mode as core::ffi::c_int {
        1 => {
            (*ptr_pvc_data).nb_high = PVC_NB_HIGH_MODE1 as UWORD8;
            (*ptr_pvc_data).nb_high_per_grp = (8 as core::ffi::c_int
                / (*ptr_pvc_data).pvc_rate as core::ffi::c_int) as UWORD8;
            (*ptr_pvc_data).p_pred_coeff_tab_1 = ixheaacd_pred_coeff_table_1_mode_1
                .as_ptr() as *mut WORD8;
            (*ptr_pvc_data).p_pred_coeff_tab_2 = ixheaacd_pred_coeff_table_2_mode_1
                .as_ptr() as *mut WORD8;
            (*ptr_pvc_data).p_pvc_id_boundary = ixheaacd_pred_coeff_pvc_id_boundaries_1
                .as_ptr() as *mut UWORD8;
            (*ptr_pvc_data).p_q_fac = ixheaacd_q_factor_table_mode_1.as_ptr()
                as *mut FLOAT32;
            if (*ptr_pvc_data).ns_mode != 0 {
                (*ptr_pvc_data).num_time_slots = 4 as UWORD8;
                (*ptr_pvc_data).p_smth_wind_coeff = ixheaacd_pvc_smoothing_wind_tab_ns4
                    .as_ptr() as *mut FLOAT32;
            } else {
                (*ptr_pvc_data).num_time_slots = 16 as UWORD8;
                (*ptr_pvc_data).p_smth_wind_coeff = ixheaacd_pvc_smoothing_wind_tab_ns16
                    .as_ptr() as *mut FLOAT32;
            }
        }
        2 => {
            (*ptr_pvc_data).nb_high = PVC_NB_HIGH_MODE2 as UWORD8;
            (*ptr_pvc_data).nb_high_per_grp = (12 as core::ffi::c_int
                / (*ptr_pvc_data).pvc_rate as core::ffi::c_int) as UWORD8;
            (*ptr_pvc_data).p_pred_coeff_tab_1 = ixheaacd_pred_coeff_table_1_mode_2
                .as_ptr() as *mut WORD8;
            (*ptr_pvc_data).p_pred_coeff_tab_2 = ixheaacd_pred_coeff_table_2_mode_2
                .as_ptr() as *mut WORD8;
            (*ptr_pvc_data).p_pvc_id_boundary = ixheaacd_pred_coeff_pvc_id_boundaries_2
                .as_ptr() as *mut UWORD8;
            (*ptr_pvc_data).p_q_fac = ixheaacd_q_factor_table_mode_2.as_ptr()
                as *mut FLOAT32;
            if (*ptr_pvc_data).ns_mode != 0 {
                (*ptr_pvc_data).num_time_slots = 3 as UWORD8;
                (*ptr_pvc_data).p_smth_wind_coeff = ixheaacd_pvc_smoothing_wind_tab_ns3
                    .as_ptr() as *mut FLOAT32;
            } else {
                (*ptr_pvc_data).num_time_slots = 12 as UWORD8;
                (*ptr_pvc_data).p_smth_wind_coeff = ixheaacd_pvc_smoothing_wind_tab_ns12
                    .as_ptr() as *mut FLOAT32;
            }
        }
        _ => return -(1 as WORD32),
    }
    (*ptr_pvc_data).prev_pvc_id = (*ptr_pvc_data)
        .pvc_id[(PVC_NUM_TIME_SLOTS - 1 as core::ffi::c_int) as usize];
    ixheaacd_pvc_qmf_grouping(
        ptr_pvc_data,
        first_bnd_idx,
        p_qmf_ener,
        first_pvc_timeslot,
    );
    ixheaacd_pvc_time_smoothing(ptr_pvc_data);
    ixheaacd_pvc_pred_env_sf(ptr_pvc_data);
    ixheaacd_pvc_sb_parsing(ptr_pvc_data, first_bnd_idx, p_qmfh);
    memcpy(
        &mut *(*((*ptr_pvc_data).esg)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
            as *mut core::ffi::c_void,
        &mut *(*((*ptr_pvc_data).esg).as_mut_ptr().offset(PVC_NUM_TIME_SLOTS as isize))
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
            as *const core::ffi::c_void,
        (::core::mem::size_of::<FLOAT32>() as size_t)
            .wrapping_mul((PVC_NUM_TIME_SLOTS - 1 as core::ffi::c_int) as size_t)
            .wrapping_mul(3 as size_t),
    );
    return 0 as WORD32;
}
